#![allow(non_snake_case)]

// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Condvar,
};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use super::DumpInfo::{DumpInfo, FillHandle};
use super::PictureDumpMap::PictureDumpMap;

// Java: public interface IDumpFinishHandler
pub trait IDumpFinishHandler: Send + Sync {
    // Java: void onSuccess(int var1);
    fn onSuccess(&self, mediaID: i32);
    // Java: boolean useThread();
    fn useThread(&self) -> bool;
}

// Java: public interface IDumpBlipHandler
pub trait IDumpBlipHandler: Send + Sync {
    // Java: boolean onHandle(BlipFill var1, IDumpFinishHandler var2);
    fn onHandle(&self, fill: FillHandle, handler: Option<FinishHandlerRef>) -> bool;

    // Java: DefaultDumpBlipHandler.notifyAndWaitDumpThreadFinish()
    fn notifyAndWaitDumpThreadFinish(&self) -> bool { false }

    // Java: DefaultDumpBlipHandler.hasNoSpaceLeftException()
    fn hasNoSpaceLeftException(&self) -> bool { false }
}

pub type FinishHandlerRef = Arc<dyn IDumpFinishHandler>;
pub type BlipHandlerRef = Arc<dyn IDumpBlipHandler>;

// Java: public abstract static class DefaultDumpBlipHandler.BlipBaseInfo
// The node is shared by the queued worker and every destination of one image.
pub trait BlipBaseInfo: Send + Sync + 'static {
    // Java: protected Vector<IDumpFinishHandler> mDumpFinishHandlers;
    fn addDumpFinishHandler(&self, handler: FinishHandlerRef);
    fn dumpFinishHandlers(&self) -> Vec<FinishHandlerRef>;
    // Java: public abstract boolean isEndNode();
    fn isEndNode(&self) -> bool;
    // Java: info instanceof PictureImporter.BlipDocInfo
    fn isBlipDocInfo(&self) -> bool {
        false
    }
}

pub enum DumpBlipError {
    // Java: catch (ForceQuitException e) { mHasForceQuitException = true; }
    ForceQuit,
    // Java: catch (IOException e) { if (NoSpaceLeftException.isNoSpaceLeftException(e)) ... }
    Io(std::io::Error),
}

impl From<std::io::Error> for DumpBlipError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

// Java: DefaultDumpBlipHandler.DumpBlipThread
// The worker owns only a queue of Java-equivalent shared nodes. The importer
// supplies dumpBlip, so media registration remains in the corresponding class.
pub struct DumpBlipThread<N: BlipBaseInfo> {
    state: Arc<(Mutex<DumpThreadState<N>>, Condvar)>,
    worker: Mutex<Option<JoinHandle<()>>>,
    dumpBlip: Arc<dyn Fn(&N) -> Result<i32, DumpBlipError> + Send + Sync>,
}

struct DumpThreadState<N: BlipBaseInfo> {
    queue: VecDeque<Arc<N>>,
    pause: bool,
    end: bool,
    disposed: bool,
    has_no_space_left_exception: bool,
    has_force_quit_exception: bool,
}

impl<N: BlipBaseInfo> DumpBlipThread<N> {
    // Java: public DumpBlipThread() { super("DThread"); }
    pub fn new(dumpBlip: Arc<dyn Fn(&N) -> Result<i32, DumpBlipError> + Send + Sync>) -> Self {
        Self {
            state: Arc::new((
                Mutex::new(DumpThreadState {
                    queue: VecDeque::new(),
                    pause: false,
                    end: false,
                    disposed: false,
                    has_no_space_left_exception: false,
                    has_force_quit_exception: false,
                }),
                Condvar::new(),
            )),
            worker: Mutex::new(None),
            dumpBlip,
        }
    }

    // Java: sDumpPicThread.start();
    pub fn start(&self) {
        let mut worker = self.worker.lock().expect("dump worker lock");
        if worker.is_some() {
            return;
        }
        let state = self.state.clone();
        let dump = self.dumpBlip.clone();
        *worker = Some(
            thread::Builder::new()
                .name("DThread".into())
                .spawn(move || {
                    loop {
                        // Java: pauseIfNeed(); mQueue.take();
                        let node = {
                            let (lock, ready) = &*state;
                            let mut current = lock.lock().expect("dump queue lock");
                            while current.pause || (current.queue.is_empty() && !current.end) {
                                current = ready.wait(current).expect("dump queue wait");
                            }
                            // Java: if (mEnd && mQueue.size() == 0) break;
                            if current.end && current.queue.is_empty() {
                                break;
                            }
                            let node = current.queue.pop_front().expect("queued node");
                            // Java: if (info.isEndNode()) this.mEnd = true;
                            if node.isEndNode() {
                                current.end = true;
                            }
                            // Java: if (this.mEnd && this.mQueue.size() == 0) break;
                            if current.end && current.queue.is_empty() {
                                break;
                            }
                            node
                        };
                        // Java: int mediaId = dumpBlip(info); then notify each handler.
                        match dump(&node) {
                            Ok(media_id) => {
                                for handler in node.dumpFinishHandlers() {
                                    handler.onSuccess(media_id);
                                }
                            }
                            Err(DumpBlipError::ForceQuit) => {
                                state
                                    .0
                                    .lock()
                                    .expect("dump queue lock")
                                    .has_force_quit_exception = true;
                            }
                            Err(DumpBlipError::Io(error))
                                if error.kind() == std::io::ErrorKind::StorageFull =>
                            {
                                state
                                    .0
                                    .lock()
                                    .expect("dump queue lock")
                                    .has_no_space_left_exception = true;
                            }
                            Err(_) => {}
                        }
                    }
                    // Java: dispose();
                    let mut current = state.0.lock().expect("dump queue lock");
                    current.queue.clear();
                    current.disposed = true;
                })
                .expect("start DThread"),
        );
    }

    // Java: public boolean add(BlipBaseInfo info)
    pub fn add(&self, info: Arc<N>) -> bool {
        let (lock, ready) = &*self.state;
        let mut current = lock.lock().expect("dump queue lock");
        if current.end || current.disposed {
            return false;
        }
        current.queue.push_back(info);
        ready.notify_one();
        true
    }

    // Java: public void finish() { mQueue.add(getEndNode()); }
    pub fn finish(&self, end_node: Arc<N>) {
        let (lock, ready) = &*self.state;
        let mut current = lock.lock().expect("dump queue lock");
        if !current.end && !current.disposed {
            current.queue.push_back(end_node);
        }
        ready.notify_all();
    }

    // Java: public synchronized void pause()/wakeUp()
    pub fn pause(&self) {
        self.state.0.lock().expect("dump queue lock").pause = true;
    }
    pub fn wakeUp(&self) {
        self.state.0.lock().expect("dump queue lock").pause = false;
        self.state.1.notify_all();
    }
    pub fn isAlive(&self) -> bool {
        self.worker
            .lock()
            .expect("dump worker lock")
            .as_ref()
            .is_some_and(|w| !w.is_finished())
    }
    pub fn join(&self) {
        if let Some(worker) = self.worker.lock().expect("dump worker lock").take() {
            let _ = worker.join();
        }
    }
    pub fn hasNoSpaceLeftException(&self) -> bool {
        self.state
            .0
            .lock()
            .expect("dump queue lock")
            .has_no_space_left_exception
    }
    pub fn hasForceQuitException(&self) -> bool {
        self.state
            .0
            .lock()
            .expect("dump queue lock")
            .has_force_quit_exception
    }
}

// Java: public abstract static class DefaultDumpBlipHandler implements IDumpBlipHandler
pub struct DefaultDumpBlipHandler<N: BlipBaseInfo> {
    thread: Arc<DumpBlipThread<N>>,
    append_node: Arc<dyn Fn(&FillHandle) -> Arc<N> + Send + Sync>,
    end_node: Arc<dyn Fn() -> Arc<N> + Send + Sync>,
    handled: AtomicBool,
}

impl<N: BlipBaseInfo> DefaultDumpBlipHandler<N> {
    pub fn new(
        thread: Arc<DumpBlipThread<N>>,
        append_node: Arc<dyn Fn(&FillHandle) -> Arc<N> + Send + Sync>,
        end_node: Arc<dyn Fn() -> Arc<N> + Send + Sync>,
    ) -> Self {
        Self {
            thread,
            append_node,
            end_node,
            handled: AtomicBool::new(false),
        }
    }

    // Java: public boolean notifyAndWaitDumpThreadFinish()
    pub fn notifyAndWaitDumpThreadFinish(&self) -> bool {
        if !self.thread.isAlive() {
            return false;
        }
        self.thread.finish((self.end_node)());
        self.thread.join();
        true
    }

    // Java: public boolean hasNoSpaceLeftException()
    pub fn hasNoSpaceLeftException(&self) -> bool {
        self.thread.hasNoSpaceLeftException()
    }
    // Java: public boolean hasForceQuitException()
    pub fn hasForceQuitException(&self) -> bool {
        self.thread.hasForceQuitException()
    }
}

impl<N: BlipBaseInfo> IDumpBlipHandler for DefaultDumpBlipHandler<N> {
    fn notifyAndWaitDumpThreadFinish(&self) -> bool {
        DefaultDumpBlipHandler::notifyAndWaitDumpThreadFinish(self)
    }
    fn hasNoSpaceLeftException(&self) -> bool {
        DefaultDumpBlipHandler::hasNoSpaceLeftException(self)
    }

    // Java: public boolean onHandle(BlipFill blipFill, IDumpFinishHandler handler)
    fn onHandle(&self, fill: FillHandle, handler: Option<FinishHandlerRef>) -> bool {
        // Java: BlipBaseInfo info = this.getAppendNode(blipFill);
        let info = (self.append_node)(&fill);
        // Java: if (handler != null && !handler.useThread())
        if handler.as_ref().is_some_and(|h| !h.useThread()) {
            self.thread.pause();
            let result = (self.thread.dumpBlip)(&info);
            match result {
                Ok(media_id) => {
                    if let Some(handler) = handler {
                        handler.onSuccess(media_id);
                    }
                }
                Err(DumpBlipError::Io(error))
                    if error.kind() == std::io::ErrorKind::StorageFull =>
                {
                    self.thread.wakeUp();
                    panic!("{error}");
                }
                Err(_) => {}
            }
            self.thread.wakeUp();
            return true;
        }
        // Java: if (null != handler) info.mDumpFinishHandlers.add(handler);
        if let Some(handler) = handler {
            info.addDumpFinishHandler(handler);
        }
        // Java: DumpBlipThread thread = this.getDumpBlipThread();
        // The DOCX override constructs and starts the shared thread lazily.
        self.thread.start();
        // Java: if (this.isHandled() && info instanceof BlipDocInfo) return true;
        if self.handled.load(Ordering::SeqCst) && info.isBlipDocInfo() {
            return true;
        }
        // Java: boolean bAdd = thread.add(info); if (bAdd) this.setHandled();
        let added = self.thread.add(info);
        if added {
            self.handled.store(true, Ordering::SeqCst);
        }
        added
    }
}

// Java: public class PictureLoader
#[derive(Default)]
pub struct PictureLoader;

impl PictureLoader {
    pub fn new() -> Self {
        Self
    }

    // Java: public int getMediaID(IDumpFinishHandler handler, Shape shape, int type)
    pub fn getMediaID(
        &mut self,
        handler: Option<FinishHandlerRef>,
        shape: &Arc<Mutex<Shape>>,
        r#type: i32,
    ) -> i32 {
        // Java: return this.dump(handler, shape, type);
        self.dump_1(handler, shape, r#type)
    }

    // Java: public synchronized int dump(Shape shape, int type)
    pub fn dump(&mut self, shape: &Arc<Mutex<Shape>>, r#type: i32) -> i32 {
        // Java: return this.dump((IDumpFinishHandler)null, shape, type);
        self.dump_1(None, shape, r#type)
    }

    // Java: public synchronized int dump(IDumpFinishHandler handler, Shape shape, int type)
    pub fn dump_1(
        &mut self,
        handler: Option<FinishHandlerRef>,
        shape: &Arc<Mutex<Shape>>,
        r#type: i32,
    ) -> i32 {
        // Java: BlipFill fill = DumpInfo.getFill(shape, type);
        let (fill, shapeID) = {
            let mut shape = shape.lock().expect("shape lock");
            (DumpInfo::getFill(&mut shape, r#type), shape.getShapeID())
        };
        // Java: if (null == fill) return -1;
        let Some(fill) = fill else { return -1 };

        // Java: IDumpBlipHandler dumpBlipHandler = PictureDumpMap.get(shape.getShapeID(), type);
        let dumpBlipHandler = PictureDumpMap::get(shapeID, r#type);
        // Java: int ret = fill.getMediaID();
        let mut ret = fill.getMediaID();
        // Java: if (-1 == ret && dumpBlipHandler != null)
        if ret == -1 {
            if let Some(dumpBlipHandler) = dumpBlipHandler {
                // Java: if (!dumpBlipHandler.onHandle(fill, handler))
                if !dumpBlipHandler.onHandle(fill.clone(), handler.clone()) {
                    // Java: ret = fill.getMediaID();
                    ret = fill.getMediaID();
                    // Java: if (-1 != ret && handler != null) handler.onSuccess(ret);
                    if ret != -1 {
                        if let Some(handler) = handler.as_ref() {
                            handler.onSuccess(ret);
                        }
                    }
                } else {
                    // Java: ret = fill.getMediaID();
                    ret = fill.getMediaID();
                }
            } else if let Some(handler) = handler.as_ref() {
                // Java: else if (handler != null) handler.onSuccess(ret);
                handler.onSuccess(ret);
            }
        } else if let Some(handler) = handler.as_ref() {
            // Java: else if (handler != null) handler.onSuccess(ret);
            handler.onSuccess(ret);
        }
        // Java: return ret;
        ret
    }
}
