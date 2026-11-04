// Java: import cn.wps.io.dom.Element
use crate::io::dom::Element::Element;
// Java: import cn.wps.io.dom.ElementHandler
use crate::io::dom::ElementHandler::ElementHandler;
// Java: import cn.wps.io.dom.ElementPath
use crate::io::dom::ElementPath::ElementPath;

// Java: public class ElementStack implements ElementPath
pub struct ElementStack {
    // Java field: Element[] stack
    pub(crate) stack: Vec<Option<Box<dyn std::any::Any + Send + Sync>>>,
    // Java field: int lastElementIndex
    pub(crate) lastElementIndex: i32,
    // Java field: DispatchHandler handler
    pub(crate) handler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: boolean mOnlyUseName
    pub(crate) mOnlyUseName: bool,
    // Java field: StringBuilder mStrBuilder
    mStrBuilder: String,
}

// Java: public class ElementStack implements ElementPath
impl ElementStack {
    // Java: ElementStack(boolean onlyUseName)
    pub fn new(onlyUseName: bool) -> Self {
      // Java: this(50, onlyUseName);
      Self::new_with_capacity(50, onlyUseName)
    }

    // Java: ElementStack(int defaultCapacity, boolean onlyUseName)
    pub fn new_with_capacity(defaultCapacity: i32, onlyUseName: bool) -> Self {
      // Java: super();
      // Java: this.lastElementIndex = -1;
      // Java: this.handler = null;
      // Java: this.stack = new Element[defaultCapacity];
      // Java: this.mOnlyUseName = onlyUseName;
      // Java: this.mStrBuilder = new StringBuilder(128);
      let mut stack = Vec::with_capacity(defaultCapacity as usize);
      for _ in 0..defaultCapacity {
          stack.push(None);
      }
      Self {
          stack,
          lastElementIndex: -1,
          handler: None,
          mOnlyUseName: onlyUseName,
          mStrBuilder: String::with_capacity(128),
      }
    }

    // Java: void setDispatchHandler(DispatchHandler dispatchHandler)
    pub fn setDispatchHandler(&mut self, dispatchHandler: Box<dyn std::any::Any + Send + Sync>) {
      // Java: this.handler = dispatchHandler;
      self.handler = Some(dispatchHandler);
    }

    // Java: DispatchHandler getDispatchHandler()
    pub fn getDispatchHandler(&self) -> &Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return this.handler;
      &self.handler
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.lastElementIndex = -1;
      self.lastElementIndex = -1;
    }

    // Java: Element peekElement()
    pub fn peekElement(&self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
      // Java: return this.lastElementIndex < 0 ? null : this.stack[this.lastElementIndex];
      if self.lastElementIndex < 0 {
          None
      } else {
          self.stack[self.lastElementIndex as usize].as_ref()
      }
    }

    // Java: Element popElement()
    pub fn popElement(&mut self) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: if (this.lastElementIndex < 0)
      if self.lastElementIndex < 0 {
         // Java: return null;
         None
      }
      // Java: else
      else {
         // Java: int last = this.lastElementIndex--;
         let last = self.lastElementIndex as usize;
         self.lastElementIndex -= 1;
         // Java: Element ret = this.stack[last];
         // Java: this.stack[last] = null;
         // Java: return ret;
         self.stack[last].take()
      }
    }

    // Java: void pushElement(Element element)
    pub fn pushElement(&mut self, element: Box<dyn std::any::Any + Send + Sync>) {
      // Java: int length = this.stack.length;
      let length = self.stack.len();
      // Java: if (++this.lastElementIndex >= length)
      self.lastElementIndex += 1;
      if self.lastElementIndex as usize >= length {
         // Java: this.reallocate(length * 2);
         self.reallocate((length * 2) as i32);
      }
      // Java: this.stack[this.lastElementIndex] = element;
      self.stack[self.lastElementIndex as usize] = Some(element);
    }

    // Java: void reallocate(int size)
    pub fn reallocate(&mut self, size: i32) {
      // Java: Element[] oldStack = this.stack;
      // Java: this.stack = new Element[size];
      // Java: System.arraycopy(oldStack, 0, this.stack, 0, oldStack.length);
      self.stack.resize_with(size as usize, || None);
    }

    // Java: String getHandlerPath(String path)
    pub fn getHandlerPath(&self, path: &str) -> String {
      // Java: String handlerPath;
      // Java: if (path.charAt(0) == '/')
      if path.starts_with('/') {
         // Java: handlerPath = path;
         path.to_string()
      }
      // Java: else
      else {
         // Java: String curPath = this.getPath();
         let cur_path = self.getPath();
         // Java: handlerPath = this.mStrBuilder.append(curPath).append('/').append(path).toString();
         format!("{}/{}", cur_path, path)
      }
    }

    // Java: ElementHandler getElementHandler()
    pub fn getElementHandler(&self) -> &Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: return this.handler;
      &self.handler
    }

}

impl ElementPath for ElementStack {
    // Java: abstract boolean getOnlyUseName()
    fn getOnlyUseName(&self) -> bool {
      // Java: return this.mOnlyUseName;
      self.mOnlyUseName
    }

    // Java: abstract int size()
    fn size(&self) -> i32 {
      // Java: return this.lastElementIndex + 1;
      self.lastElementIndex + 1
    }

    // Java: abstract Element getElement(int var1)
    fn getElement(&self, var1: i32) -> Option<&dyn std::any::Any> {
      // Java: element = this.stack[depth];
      if var1 >= 0 && (var1 as usize) < self.stack.len() {
          self.stack[var1 as usize].as_ref().map(|b| b.as_ref() as &dyn std::any::Any)
      } else {
          None
      }
    }

    // Java: abstract String getPath()
    fn getPath(&self) -> String {
      // Java: return this.handler.getPath();
      String::new()
    }

    // Java: abstract Element getCurrent()
    fn getCurrent(&self) -> Option<&dyn std::any::Any> {
      // Java: return this.lastElementIndex < 0 ? null : this.stack[this.lastElementIndex];
      if self.lastElementIndex < 0 {
          None
      } else {
          self.stack[self.lastElementIndex as usize].as_ref().map(|b| b.as_ref() as &dyn std::any::Any)
      }
    }

    // Java: abstract String getCurrentName()
    fn getCurrentName(&self) -> String {
      // Java: return this.lastElementIndex < 0 ? null : this.stack[this.lastElementIndex].getName();
      String::new()
    }

    // Java: abstract void addHandler(String var1, ElementHandler var2)
    fn addHandler(&mut self, var1: &str, var2: Box<dyn ElementHandler>) {
      // Java: this.handler.addHandler(this.mOnlyUseName ? path : this.getHandlerPath(path), elementHandler);
      // TODO_RUST_CODE
    }

    // Java: abstract void removeHandler(String var1)
    fn removeHandler(&mut self, var1: &str) {
      // Java: this.handler.removeHandler(this.mOnlyUseName ? path : this.getHandlerPath(path));
      // TODO_RUST_CODE
    }
}
