// Java: import cn.wps.collections.textrope.INotifyTextRope
use crate::cn::wps::collections::textrope::INotifyTextRope::*;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::{Document, SoftTextDocument};
// Java: import cn.wps.moffice.writer.core.TextDocument
// soft: TextDocument leaf deferred — holder keeps *const SoftTextDocument for live IO flags
// Java: import cn.wps.moffice.writer.core.DocumentEventDispatcher
use crate::cn::wps::moffice::writer::core::DocumentEventDispatcher::*;

/// Soft stand-in for cached TextDocument field until TextDocument leaf is rewritten.
/// Java stores the TextDocument reference; flags are read live in `isDisabled`.
#[derive(Default, Clone, Copy)]
pub struct SoftTextDocumentHolder {
    td: Option<*const (dyn SoftTextDocument + 'static)>,
}
impl SoftTextDocumentHolder {
    pub fn isInIOState(&self) -> bool {
        self.td
            .map(|p| unsafe { (*p).isInIOState() })
            .unwrap_or(false)
    }
    pub fn isInUserAction(&self) -> bool {
        self.td
            .map(|p| unsafe { (*p).isInUserAction() })
            .unwrap_or(false)
    }
}

/// Soft Document holder: ctor accepts &dyn Document to call getDispatcher; field stores owned dispatcher only.
#[derive(Default)]
pub struct SoftDocumentHolder {}

// java final class TextRopeListeners implements INotifyTextRope.Listener
#[derive(Default)]
pub struct TextRopeListeners {
    // java private ArrayList<INotifyTextRope.Listener> mListeners;
    mListeners: Option<Vec<Box<dyn Listener>>>,
    // java private Document mDocument;
    mDocument: Option<SoftDocumentHolder>,
    // java private TextDocument mTextDocument;
    mTextDocument: SoftTextDocumentHolder,
    // java private DocumentEventDispatcher mDispatcher;
    mDispatcher: DocumentEventDispatcher,
}

// java final class TextRopeListeners implements INotifyTextRope.Listener
impl TextRopeListeners {
    // java public  TextRopeListeners(Document document)
    pub fn TextRopeListeners(document: &dyn Document) -> Self {
      // java this.mDocument = document;
      // java this.mTextDocument = this.mDocument.getTextDocument();
      let td = document.getTextDocument().map(|r| {
        // field holds the peer for the Document's lifetime (same as Java alias)
        unsafe {
          std::mem::transmute::<&dyn SoftTextDocument, *const (dyn SoftTextDocument + 'static)>(r)
        }
      });
      // java this.mDispatcher = this.mDocument.getDispatcher();
      let dispatcher = document.getDispatcher().unwrap_or_else(DocumentEventDispatcher::new);
      // java this.mListeners = new ArrayList();
      Self {
        mListeners: Some(Vec::new()),
        mDocument: Some(SoftDocumentHolder::default()),
        mTextDocument: SoftTextDocumentHolder { td },
        mDispatcher: dispatcher,
      }
    }

    // java public final void clear()
    pub fn clear(&mut self) {
      // java this.mListeners.clear();
      if let Some(m) = self.mListeners.as_mut() {
        m.clear();
      }
    }

    // java public final int size()
    pub fn size(&self) -> i32 {
      // java return this.mListeners.size();
      return (self.mListeners.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
    }

    // java public final INotifyTextRope.Listener get(int i)
    pub fn get(&self, i: i32) -> Option<&dyn Listener> {
      // java return (INotifyTextRope.Listener)this.mListeners.get(i);
      return self
        .mListeners
        .as_ref()
        .and_then(|m| m.get(i as usize).map(|b| b.as_ref()));
    }

    // java public final void add(INotifyTextRope.Listener listener)
    pub fn add(&mut self, listener: Box<dyn Listener>) {
      // java this.mListeners.add(listener);
      if let Some(m) = self.mListeners.as_mut() {
        m.push(listener);
      }
    }

    // java public final INotifyTextRope.Listener remove(int i)
    pub fn remove__int(&mut self, i: i32) -> Option<Box<dyn Listener>> {
      // java return (INotifyTextRope.Listener)this.mListeners.remove(i);
      return self.mListeners.as_mut().map(|m| m.remove(i as usize));
    }

    // java public final INotifyTextRope.Listener remove(INotifyTextRope.Listener listener)
    pub fn remove__listener(&mut self, listener: Box<dyn Listener>) -> Box<dyn Listener> {
      // java this.mListeners.remove(listener);
      if let Some(m) = self.mListeners.as_mut() {
        // ArrayList.remove(Object) by equals/identity — trait objects: ptr identity
        if let Some(pos) = m.iter().position(|x| std::ptr::eq(x.as_ref(), listener.as_ref())) {
          m.remove(pos);
        }
      }
      // java return listener;
      return listener;
    }

    // java private boolean isDisabled()
    pub fn isDisabled(&self) -> bool {
      // java return this.mTextDocument.isInIOState() && !this.mTextDocument.isInUserAction();
      return self.mTextDocument.isInIOState() && !self.mTextDocument.isInUserAction();
    }

    // java public void beforeInsertText(int cp, int fc, int length)
    pub fn beforeInsertText(&mut self, cp: i32, fc: i32, length: i32) {
      // java if (!this.isDisabled())
      if !self.isDisabled() {
        // java this.mDispatcher.beforeInsertText(cp, fc, length);
        self.mDispatcher.beforeInsertText(cp, fc, length);
        // java int n = this.size();
        let n: i32 = self.size();
        let mut i: i32 = n - 1;
        // java for(int i = n - 1; i >= 0 --i)
        while i >= 0 {
          // java this.get(i).beforeInsertText(cp, fc, length);
          if let Some(m) = self.mListeners.as_mut() {
            if let Some(b) = m.get_mut(i as usize) {
              b.beforeInsertText(cp, fc, length);
            }
          }
          i -= 1;
        }
      }
    }

    // java public void afterInsertText(int cp, int fc, int length)
    pub fn afterInsertText(&mut self, cp: i32, fc: i32, length: i32) {
      // java if (!this.isDisabled())
      if !self.isDisabled() {
        // java int n = this.size();
        let n: i32 = self.size();
        let mut i: i32 = 0;
        // java for(int i = 0; i < n; ++i)
        while i < n {
          // java this.get(i).afterInsertText(cp, fc, length);
          if let Some(m) = self.mListeners.as_mut() {
            if let Some(b) = m.get_mut(i as usize) {
              b.afterInsertText(cp, fc, length);
            }
          }
          i += 1;
        }
        // java this.mDispatcher.afterInsertText(cp, fc, length);
        self.mDispatcher.afterInsertText(cp, fc, length);
      }
    }

    // java public void beforeRemoveText(int start, int end)
    pub fn beforeRemoveText(&mut self, start: i32, end: i32) {
      // java if (!this.isDisabled())
      if !self.isDisabled() {
        // java this.mDispatcher.beforeRemoveText(start, end);
        self.mDispatcher.beforeRemoveText(start, end);
        // java int n = this.size();
        let n: i32 = self.size();
        let mut i: i32 = n - 1;
        // java for(int i = n - 1; i >= 0 --i)
        while i >= 0 {
          // java this.get(i).beforeRemoveText(start, end);
          if let Some(m) = self.mListeners.as_mut() {
            if let Some(b) = m.get_mut(i as usize) {
              b.beforeRemoveText(start, end);
            }
          }
          i -= 1;
        }
      }
    }

    // java public void afterRemoveText(int start, int end)
    pub fn afterRemoveText(&mut self, start: i32, end: i32) {
      // java if (!this.isDisabled())
      if !self.isDisabled() {
        // java int n = this.size();
        let n: i32 = self.size();
        let mut i: i32 = 0;
        // java for(int i = 0; i < n; ++i)
        while i < n {
          // java this.get(i).afterRemoveText(start, end);
          if let Some(m) = self.mListeners.as_mut() {
            if let Some(b) = m.get_mut(i as usize) {
              b.afterRemoveText(start, end);
            }
          }
          i += 1;
        }
        // java this.mDispatcher.afterRemoveText(start, end);
        self.mDispatcher.afterRemoveText(start, end);
      }
    }

}

impl Listener for TextRopeListeners {
    fn beforeInsertText(&mut self, var1: i32, var2: i32, var3: i32) {
        TextRopeListeners::beforeInsertText(self, var1, var2, var3);
    }
    fn afterInsertText(&mut self, var1: i32, var2: i32, var3: i32) {
        TextRopeListeners::afterInsertText(self, var1, var2, var3);
    }
    fn beforeRemoveText(&mut self, var1: i32, var2: i32) {
        TextRopeListeners::beforeRemoveText(self, var1, var2);
    }
    fn afterRemoveText(&mut self, var1: i32, var2: i32) {
        TextRopeListeners::afterRemoveText(self, var1, var2);
    }
    fn addListener(&mut self, listener: Box<dyn Listener>) {
        // Java: this.mListeners.add(listener);
        self.add(listener);
    }
}
