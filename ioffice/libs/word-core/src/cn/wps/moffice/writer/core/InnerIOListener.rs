// Java: import cn.wps.moffice.writer.core.io.IOListener
use crate::cn::wps::moffice::writer::core::io::IOListener::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
// rust: ★M-STAGE2 P1-D1 —— 由 glob 改为具名引用：`TextDocument` 里曾有一个
//   空 trait `IOListener` 影子，glob 会把它带进来与真实 trait 冲突（E0659）。
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public class InnerIOListener implements IOListener
#[derive(Default)]
pub struct InnerIOListener {
    // java private int mErrorCode = 0;
    mErrorCode: i32,
    // java private TextDocument mDoc;
    // rust: Java 的 `TextDocument mDoc` 是**引用**（构造时 `this.mDoc = doc;`），
    //   且 `onFinishDumpObjects` 会回写 `mDoc`。按 Java 别名语义用裸指针表达
    //   （同 `SubDocument.mParent` 的既有做法），避免按值克隆出第二个文档。
    mDoc: Option<*mut TextDocument>,
}

// java public interface IOListener
impl IOListener for InnerIOListener {
    // java public void onFirstLock()
    fn onFirstLock(&mut self) {
    }

    // java public void onFirstUnLock()
    fn onFirstUnLock(&mut self) {
    }

    // java public void onLoadParas(int cp)
    fn onLoadParas(&mut self, cp: i32) {
    }

    // java public void onError(int resultCode, Object msg)
    fn onError(&mut self, resultCode: i32, msg: &dyn Object) {
      // java this.mErrorCode = resultCode;
      self.mErrorCode = resultCode;
    }

    // java public void beginLoadOnlineSecurityDoc()
    fn beginLoadOnlineSecurityDoc(&mut self) {
    }

    // java public void onFinish()
    fn onFinish(&mut self) {
    }

    // java public void onFinishDumpObjects()
    fn onFinishDumpObjects(&mut self) {
      // java this.mDoc.onFinishDumpObjects();
      if let Some(p) = self.mDoc {
        unsafe { (*p).onFinishDumpObjects(); }
      }
    }

    // java public void onHtmlOpenError()
    fn onHtmlOpenError(&mut self) {
    }

    // java public void onFinishVerifyWritePassword()
    fn onFinishVerifyWritePassword(&mut self) {
    }

}

// java public class InnerIOListener implements IOListener
impl InnerIOListener {
    // java public  InnerIOListener(TextDocument doc)
    pub fn InnerIOListener(doc: *mut TextDocument) -> Self {
      // java this.mDoc = doc;
      Self { mErrorCode: Default::default(), mDoc: Some(doc) }
    }

    // java public int getErrorCode()
    pub fn getErrorCode(&self) -> i32 {
      // java return this.mErrorCode;
      return self.mErrorCode;
    }

}
