// Java: import cn.wps.base.assertion.IOAssert
// use util::cn::wps::base::assertion::IOAssert;
// Java: import cn.wps.moffice.io.IDocument
// use util_adapter::cn::wps::moffice::io::IDocument;
// Java: import java.io.PrintWriter
// Java: import java.io.StringWriter
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

// Java field: ISlimToolManager sNull
static sNull: OnceLock<ISlimToolManager> = OnceLock::new();
// Java field: ISlimListener sNullListener
static sNullListener: OnceLock<Mutex<Option<Box<dyn std::any::Any + Send>>>> = OnceLock::new();
// Java field: ISlimToolManager sInstance
static sInstance: OnceLock<Mutex<Option<ISlimToolManager>>> = OnceLock::new();

// Java: public class ISlimToolManager
pub struct ISlimToolManager {
    // Java field: SlimThread mSlimThread
    mSlimThread: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: IDocument mSource
    pub(crate) mSource: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: InnerSlimListener mListener
    pub(crate) mListener: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: HashMap<Integer, ISlimTool> mSlimTools
    pub(crate) mSlimTools: HashMap<i32, Box<dyn std::any::Any + Send + Sync>>,
    // Java field: SlimContext mSlimContext
    pub(crate) mSlimContext: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: boolean mIsIOFinish
    mIsIOFinish: bool,
}

// Java: public class ISlimToolManager
impl ISlimToolManager {
    // Java: ISlimToolManager(IDocument source, ISlimListener listener)
    pub fn new(_source: Option<Box<dyn std::any::Any + Send + Sync>>, _listener: Option<Box<dyn std::any::Any + Send + Sync>>) -> Self {
      // Java: super();
      // Java: this.mSource = source;
      // Java: this.mSlimTools = new HashMap();
      // Java: ISlimListener tmpListener = sNullListener;
      // Java: if (null != listener)
      {
         // Java: tmpListener = listener;
      }
      // Java: this.mListener = new InnerSlimListener(tmpListener);
      // Java: this.mSlimContext = new SlimContext();
      // Java: this.mIsIOFinish = false;
      ISlimToolManager {
          mSlimThread: None,
          mSource: _source,
          mListener: _listener,
          mSlimTools: HashMap::new(),
          mSlimContext: None,
          mIsIOFinish: false,
      }
    }

    // Java: static void build(IDocument source, ISlimListener listener)
    pub fn build(_source: Option<Box<dyn std::any::Any + Send + Sync>>, _listener: Option<Box<dyn std::any::Any + Send + Sync>>) {
      // Java: if (null != source)
      if _source.is_some() {
         // Java: switch (source.type())
         {
            // Java: case 1:
               // Java: sInstance = ReflectionHelper.getWriterSlimToolMgr(source, source, listener);
               // Java: break;
            // Java: case 2:
               // Java: sInstance = ReflectionHelper.getETSlimToolMgr(source, source, listener);
               // Java: break;
            // Java: case 3:
               // Java: sInstance = ReflectionHelper.getWPPSlimToolMgr(source, source, listener);
               // Java: break;
            // Java: case 4:
               // Java: sInstance = ReflectionHelper.getPDFSlimToolMgr(source, source, listener);
               // Java: break;
            // Java: default:
               // Java: IOAssert.assertTrue("需要补充对应的瘦身工具实例", false);
         }
         // Java: if (null == sInstance)
         {
            // Java: sInstance = sNull;
         }
      }
      todo!()
    }

    // Java: static ISlimToolManager instance()
    pub fn instance() -> &'static ISlimToolManager {
      // Java: IOAssert.assertTrue("快去build一下再调用我", false);
      // Java: return sInstance;
      todo!()
    }

    // Java: void start()
    pub fn start(&mut self) {
      // Java: this.startSlimThread();
      // Java: for(int i = 1; i < 124; ++i)
      {
         // Java: ISlimTool tool = this.getSlimTool(i);
         // Java: if (0 != tool.type())
         {
            // Java: this.mSlimThread.addSlimTask(tool);
         }
      }
      // Java: this.mSlimThread.addSlimDone();
      todo!()
    }

    // Java: void stop()
    pub fn stop(&mut self) {
      // Java: if (null != this.mSlimThread)
      {
         // Java: this.mSlimThread.quit();
      }
      todo!()
    }

    // Java: void check()
    pub fn check(&mut self) {
      // Java: System.out.println("slim: check");
      // Java: this.startSlimThread();
      // Java: for(int i = 1; i < 124; ++i)
      {
         // Java: ISlimTool tool = this.getSlimTool(i);
         // Java: if (0 != tool.type())
         {
            // Java: this.mSlimThread.addCheckTask(tool);
         }
      }
      // Java: this.mSlimThread.addCheckDone();
      todo!()
    }

    // Java: ArrayList<SlimResult> getSlimResult()
    pub fn getSlimResult(&self) -> Vec<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return this.mSlimContext.getSlimResults();
      todo!()
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: sInstance = sNull;
      todo!()
    }

    // Java: void clearCache()
    pub fn clearCache(&mut self) {
      // Java: for(int i = 1; i < 124; ++i)
      {
         // Java: ISlimTool tool = this.getSlimTool(i);
         // Java: if (0 != tool.type())
         {
            // Java: tool.dispose();
         }
      }
      // Java: this.mSlimContext.clearSlimResult();
      todo!()
    }

    // Java: IDocument getSlimSource()
    pub fn getSlimSource(&self) -> Option<&(dyn std::any::Any + Send + Sync)> {
      // Java: return this.mSource;
      self.mSource.as_deref()
    }

    // Java: ISlimTool getSlimTool(int type)
    pub fn getSlimTool(&self, _type_val: i32) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
      // Java: ISlimTool tool = (ISlimTool)this.mSlimTools.get(type);
      // Java: if (null != tool && this.mSource.isDirty())
      {
         // Java: tool.dispose();
      }
      // Java: if (null != tool)
      {
         // Java: return tool;
      }
      // Java: else
      {
         // Java: tool = (ISlimTool)this.mSlimTools.get(0);
         // Java: if (null == tool)
         {
            // Java: tool = ISlimTool.sNULL;
            // Java: this.mSlimTools.put(0, tool);
         }
         // Java: return tool;
      }
      todo!()
    }

    // Java: void onIOFinish(IDocument source)
    pub fn onIOFinish(&mut self, _source: &dyn std::any::Any) {
      // Java: if (this.mSource == source)
      {
         // Java: System.out.println("slim: current thread name: " + Thread.currentThread().getName());
         // Java: System.out.println("slim: onIOFinish");
         // Java: this.mIsIOFinish = true;
         self.mIsIOFinish = true;
         // Java: this.mListener.exclude(0);
         // Java: if (null != this.mSlimThread)
         {
            // Java: System.out.println("slim: wakeUp");
            // Java: this.mSlimThread.wakeUp();
         }
      }
      todo!()
    }

    // Java: InnerSlimListener getListener()
    pub fn getListener(&self) -> Option<&(dyn std::any::Any + Send + Sync)> {
      // Java: return this.mListener;
      self.mListener.as_deref()
    }

    // Java: static String getStackTraceString(Throwable tr)
    pub fn getStackTraceString(_tr: &dyn std::any::Any) -> String {
      // Java: StringWriter sw = new StringWriter();
      // Java: PrintWriter pw = new PrintWriter(sw);
      // Java: tr.printStackTrace(pw);
      // Java: pw.flush();
      // Java: return sw.toString();
      todo!()
    }

    // Java: void startSlimThread()
    pub fn startSlimThread(&mut self) {
      // Java: if (null != this.mSlimThread && this.mSlimThread.isAlive())
      {
         // Java: this.sleep();
      }
      // Java: else
      {
         // Java: this.mSlimThread = new SlimThread(this.mListener);
         // Java: (new Thread(this.mSlimThread, SlimThread.TAG)).start();
         // Java: this.sleep();
      }
      todo!()
    }

    // Java: void sleep()
    pub fn sleep(&self) {
      // Java: if (!this.mIsIOFinish)
      if !self.mIsIOFinish {
         // Java: System.out.println("slim: sleep");
         // Java: this.mSlimThread.sleep();
         todo!()
      }
    }

}
