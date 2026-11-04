use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

use drawing::cn::wps::moffice::drawing::graphics::BlipFill::BlipFill;
use word_core::cn::wps::moffice::writer::core::shape::DumpInfo::FillHandle;
use word_core::cn::wps::moffice::writer::core::shape::PictureLoader::{
    BlipBaseInfo, DefaultDumpBlipHandler, DumpBlipThread, FinishHandlerRef, IDumpBlipHandler,
    IDumpFinishHandler,
};

struct Node {
    end: bool,
    legacy_blip: bool,
    handlers: Mutex<Vec<FinishHandlerRef>>,
}

impl Node {
    fn new(end: bool) -> Self {
        Self {
            end,
            legacy_blip: false,
            handlers: Mutex::new(Vec::new()),
        }
    }
}

impl BlipBaseInfo for Node {
    fn addDumpFinishHandler(&self, handler: FinishHandlerRef) {
        self.handlers.lock().unwrap().push(handler);
    }
    fn dumpFinishHandlers(&self) -> Vec<FinishHandlerRef> {
        self.handlers.lock().unwrap().clone()
    }
    fn isEndNode(&self) -> bool {
        self.end
    }
    fn isBlipDocInfo(&self) -> bool {
        self.legacy_blip
    }
}

struct Finish {
    id: AtomicI32,
    threaded: bool,
}

impl IDumpFinishHandler for Finish {
    fn onSuccess(&self, media_id: i32) {
        self.id.store(media_id, Ordering::SeqCst);
    }
    fn useThread(&self) -> bool {
        self.threaded
    }
}

fn fill() -> FillHandle {
    FillHandle::BlipFill(Arc::new(Mutex::new(BlipFill::new_with_media_id(-1))))
}

#[test]
fn default_handler_sync_and_queued_callbacks_follow_java_branches() {
    let calls = Arc::new(AtomicI32::new(0));
    let calls_for_dump = calls.clone();
    let thread = Arc::new(DumpBlipThread::new(Arc::new(move |_node: &Node| {
        calls_for_dump.fetch_add(1, Ordering::SeqCst);
        Ok(82)
    })));
    let node = Arc::new(Node::new(false));
    let node_for_append = node.clone();
    let handler = DefaultDumpBlipHandler::new(
        thread.clone(),
        Arc::new(move |_fill| node_for_append.clone()),
        Arc::new(|| Arc::new(Node::new(true))),
    );

    let sync = Arc::new(Finish {
        id: AtomicI32::new(-1),
        threaded: false,
    });
    assert!(handler.onHandle(fill(), Some(sync.clone())));
    assert_eq!(sync.id.load(Ordering::SeqCst), 82);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(!handler.notifyAndWaitDumpThreadFinish());

    thread.pause();
    let first = Arc::new(Finish {
        id: AtomicI32::new(-1),
        threaded: true,
    });
    let second = Arc::new(Finish {
        id: AtomicI32::new(-1),
        threaded: true,
    });
    assert!(handler.onHandle(fill(), Some(first.clone())));
    assert!(handler.onHandle(fill(), Some(second.clone())));
    thread.wakeUp();
    assert!(handler.notifyAndWaitDumpThreadFinish());
    // Java only short-circuits PictureImporter.BlipDocInfo. The DOCX node is
    // BlipFillInfos.BlipDocxInfo and is queued on both calls.
    assert_eq!(calls.load(Ordering::SeqCst), 3);
    assert_eq!(first.id.load(Ordering::SeqCst), 82);
    assert_eq!(second.id.load(Ordering::SeqCst), 82);
}
