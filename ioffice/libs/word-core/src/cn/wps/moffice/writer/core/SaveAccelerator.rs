// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import java.lang.ThreadLocal
use stubs::java::lang::ThreadLocal::*;

// java private static final String TAG = "SaveAccelerator";
const TAG: &str = "SaveAccelerator";

// java public class SaveAccelerator
#[derive(Default)]
pub struct SaveAccelerator {
    // java TextDocument mTextDocument;
    // rust: Java 对象引用**别名/回指**宿主文档 ⇒ 裸指针（先例 `InnerIOListener.mDoc: Option<*mut TextDocument>`）。
    //   按值持有 `Option<TextDocument>` 会与 `TextDocument.mSaveAccelerator` 构成 `E0072` 无限大小类型。
    pub(crate) mTextDocument: Option<*mut TextDocument>,
    // java private ThreadLocal<TagsToThread> mBindToThread = new ThreadLocal();
    // rust: Java `ThreadLocal<T>` 语义是“每线程一个**可共享可变**实例”，且 `getFromThread()`
    //   返回**同一实例**的引用（故 Java 的 `this.getFromThread().isQuickSave = quick;` 能持久生效）。
    //   Rust 正解是共享可变句柄 `Rc<RefCell<TagsToThread>>`；外层 `Mutex` 仅提供 `&self` 下的内部可变性。
    mBindToThread: std::sync::Mutex<Option<std::rc::Rc<std::cell::RefCell<TagsToThread>>>>,
    // java boolean mForbiddenAutoSave = false;
    pub(crate) mForbiddenAutoSave: bool,
    // java Boolean mForbiddenAutoSaveTemp = false;
    pub(crate) mForbiddenAutoSaveTemp: Option<bool>,
}

// java static class TagsToThread
#[derive(Default)]
pub struct TagsToThread {
    // java boolean isQuickSave = false;
    pub(crate) isQuickSave: bool,
}

// java public class SaveAccelerator
impl SaveAccelerator {
    // java   SaveAccelerator(TextDocument textDocument)
    pub fn SaveAccelerator(textDocument: &TextDocument) -> Self {
      // java this.mTextDocument = textDocument;
      Self { mTextDocument: Some(textDocument as *const TextDocument as *mut TextDocument), mBindToThread: Default::default(), mForbiddenAutoSave: Default::default(), mForbiddenAutoSaveTemp: Default::default() }
    }

    // java private TagsToThread getFromThread()
    pub fn getFromThread(&self) -> std::rc::Rc<std::cell::RefCell<TagsToThread>> {
      // java TagsToThread tag = (TagsToThread)this.mBindToThread.get();
      let mut slot = self.mBindToThread.lock().unwrap();
      let mut tag = slot.clone();
      // java if (tag == null)
      if tag.is_none() {
        // java tag = new TagsToThread();
        tag = Some(std::rc::Rc::new(std::cell::RefCell::new(TagsToThread::default())));
        // java this.mBindToThread.set(tag);
        *slot = tag.clone();
      }
      // java return tag;
      return tag.unwrap();
    }

    // java public boolean isQuickSave()
    pub fn isQuickSave(&self) -> bool {
      // java return this.getFromThread().isQuickSave;
      return self.getFromThread().borrow().isQuickSave;
    }

    // java public void setQuickSave(boolean quick)
    pub fn setQuickSave(&self, quick: bool) {
      // java this.getFromThread().isQuickSave = quick;
      self.getFromThread().borrow_mut().isQuickSave = quick;
    }

    // java public synchronized boolean isForbiddenSave()
    pub fn isForbiddenSave(&self) -> bool {
      // java return this.mForbiddenAutoSave;
      return self.mForbiddenAutoSave;
    }

    // java public synchronized void setForbiddenAutoSave(boolean bForbiddenAutoSave)
    pub fn setForbiddenAutoSave(&mut self, bForbiddenAutoSave: bool) {
      // java this.mForbiddenAutoSave = bForbiddenAutoSave;
      self.mForbiddenAutoSave = bForbiddenAutoSave;
    }

    // java public synchronized void backupAndForbiddenAutoSave()
    pub fn backupAndForbiddenAutoSave(&mut self) {
      // java if (this.mForbiddenAutoSaveTemp == null)
      if self.mForbiddenAutoSaveTemp.is_none() {
        // java this.mForbiddenAutoSaveTemp = this.isForbiddenSave();
        self.mForbiddenAutoSaveTemp = Some(self.isForbiddenSave());
      }
      // java this.setForbiddenAutoSave(true);
      self.setForbiddenAutoSave(true);
    }

    // java public synchronized void restoreForbiddenAutoSave()
    pub fn restoreForbiddenAutoSave(&mut self) {
      // java boolean backedValue = this.mForbiddenAutoSaveTemp == null ? false : this.mForbiddenAutoSaveTemp;
      let mut backedValue: bool = self.mForbiddenAutoSaveTemp.unwrap_or(false);
      // java this.setForbiddenAutoSave(backedValue);
      self.setForbiddenAutoSave(backedValue);
      // java this.mForbiddenAutoSaveTemp = null;
      self.mForbiddenAutoSaveTemp = None;
    }

}

// java static class TagsToThread
impl TagsToThread {
}
