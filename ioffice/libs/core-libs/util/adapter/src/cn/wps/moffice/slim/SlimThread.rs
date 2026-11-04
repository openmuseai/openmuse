// Java: import cn.wps.base.log.IOLog
// use util::cn::wps::base::log::IOLog;
// Java: import cn.wps.moffice.io.IDocument
// use util_adapter::cn::wps::moffice::io::IDocument;
// Java: import java.util.concurrent.LinkedBlockingQueue
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

// Java field: String TAG
pub(crate) const TAG: &str = "";
// Java field: int TASK_TYPE_CHECK
pub(crate) const TASK_TYPE_CHECK: i32 = 1;
// Java field: int TASK_TYPE_SLIM
pub(crate) const TASK_TYPE_SLIM: i32 = 2;
// Java field: int TASK_TYPE_CHECK_DONE
pub(crate) const TASK_TYPE_CHECK_DONE: i32 = 3;
// Java field: int TASK_TYPE_SLIM_DONE
pub(crate) const TASK_TYPE_SLIM_DONE: i32 = 4;
// Java field: int TASK_TYPE_QUIT
pub(crate) const TASK_TYPE_QUIT: i32 = 5;
// Java field: Task sCheckDone
static sCheckDone: std::sync::OnceLock<Task> = std::sync::OnceLock::new();
// Java field: Task sSlimDone
static sSlimDone: std::sync::OnceLock<Task> = std::sync::OnceLock::new();
// Java field: Task sSlimQuit
static sSlimQuit: std::sync::OnceLock<Task> = std::sync::OnceLock::new();

// Java: public class SlimThread implements Runnable
pub struct SlimThread {
    // Java field: LinkedBlockingQueue<Task> mTaskQueue
    mTaskQueue: VecDeque<Task>,
    // Java field: ISlimListener mListener
    mListener: Option<Box<dyn std::any::Any>>,
    // Java field: boolean mQuiting
    mQuiting: bool,
    // Java field: boolean mIsAlive
    pub(crate) mIsAlive: bool,
    // Java field: boolean mIsSleeping
    mIsSleeping: bool,
    // Java field: Object mLock
    mLock: Arc<Mutex<()>>,
}

// Java: public class Task
pub struct Task {
    // Java field: int mType
    pub(crate) mType: i32,
    // Java field: ISlimTool mTool
    pub(crate) mTool: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class SlimThread implements Runnable
impl SlimThread {
    // Java: SlimThread(ISlimListener listener)
    pub fn new(listener: Option<Box<dyn std::any::Any>>) -> Self {
      // Java: super();
      // Java: this.mListener = listener;
      // Java: this.mQuiting = false;
      // Java: this.mIsSleeping = false;
      // Java: this.mLock = new Object();
      SlimThread {
          mTaskQueue: VecDeque::new(),
          mListener: listener,
          mQuiting: false,
          mIsAlive: false,
          mIsSleeping: false,
          mLock: Arc::new(Mutex::new(())),
      }
    }

    // Java: void run()
    pub fn run(&mut self) {
      // Java: this.mIsAlive = true;
      self.mIsAlive = true;
      // Java: synchronized(this.mLock)
      {
         // Java: try
         {
            // Java: while(!this.mQuiting)
            while !self.mQuiting {
               // Java: Task task = (Task)this.mTaskQueue.take();
               // Java: if (this.mIsSleeping)
               {
                  // Java: this.mLock.wait();
               }
               // Java: if (null != task && this.handleTask(task))
               {
                  // Java: break;
                  break;
               }
            }
         }
         // Java: catch (InterruptedException e)
         {
            // Java: IOLog.e(TAG, "InterruptedException", e);
         }
         // Java: if (null != this.mListener && this.mQuiting)
         {
            // Java: this.mListener.onStopFinish();
         }
         // Java: this.mIsAlive = false;
         self.mIsAlive = false;
      }
    }

    // Java: boolean isAlive()
    pub fn isAlive(&self) -> bool {
      // Java: return this.mIsAlive;
      self.mIsAlive
    }

    // Java: boolean handleTask(Task task)
    pub fn handleTask(&mut self, _task: &Task) -> bool {
      // Java: boolean ret = false;
      // Java: switch (task.mType)
      {
         // Java: case 1:
            // Java: if (null != task.mTool)
            {
               // Java: task.mTool.checkSlim();
            }
            // Java: break;
         // Java: case 2:
            // Java: if (null != task.mTool)
            {
               // Java: long slimSize = task.mTool.slim();
               // Java: if (!this.mQuiting)
               {
                  // Java: this.mListener.onSlimItemFinish(task.mTool.type(), slimSize);
               }
               // Java: if (slimSize > 0L)
               {
                  // Java: this.setSlimDirty();
               }
            }
            // Java: break;
         // Java: case 3:
            // Java: this.mListener.onSlimCheckFinish(ISlimToolManager.instance().getSlimResult());
            // Java: ret = true;
            // Java: break;
         // Java: case 4:
            // Java: this.mListener.onSlimFinish();
            // Java: ret = true;
      }
      // Java: return ret;
      todo!()
    }

    // Java: void setSlimDirty()
    pub fn setSlimDirty(&self) {
      // Java: IDocument src = ISlimToolManager.instance().getSlimSource();
      // Java: if (null != src)
      {
         // Java: src.setSlimDirty();
      }
      todo!()
    }

    // Java: void quit()
    pub fn quit(&mut self) {
      // Java: this.mQuiting = true;
      self.mQuiting = true;
      // Java: this.mTaskQueue.add(sSlimQuit);
      todo!()
    }

    // Java: void addCheckTask(ISlimTool tool)
    pub fn addCheckTask(&mut self, _tool: Box<dyn std::any::Any>) {
      // Java: this.mTaskQueue.add(new Task(1, tool));
      todo!()
    }

    // Java: void addCheckDone()
    pub fn addCheckDone(&mut self) {
      // Java: this.mTaskQueue.add(sCheckDone);
      todo!()
    }

    // Java: void addSlimTask(ISlimTool tool)
    pub fn addSlimTask(&mut self, _tool: Box<dyn std::any::Any>) {
      // Java: this.mTaskQueue.add(new Task(2, tool));
      todo!()
    }

    // Java: void addSlimDone()
    pub fn addSlimDone(&mut self) {
      // Java: this.mTaskQueue.add(sSlimDone);
      todo!()
    }

    // Java: void wakeUp()
    pub fn wakeUp(&mut self) {
      // Java: this.mIsSleeping = false;
      self.mIsSleeping = false;
      // Java: synchronized(this.mLock)
      {
         // Java: this.mLock.notify();
      }
    }

    // Java: void sleep()
    pub fn sleep(&mut self) {
      // Java: this.mIsSleeping = true;
      self.mIsSleeping = true;
    }

}

// Java: public class Task
impl Task {
    // Java: Task(int type, ISlimTool tool)
    pub fn new(r#type: i32, tool: Option<Box<dyn std::any::Any + Send + Sync>>) -> Self {
         // Java: super();
         // Java: this.mType = type;
         // Java: this.mTool = tool;
         Task { mType: r#type, mTool: tool }
    }

}
