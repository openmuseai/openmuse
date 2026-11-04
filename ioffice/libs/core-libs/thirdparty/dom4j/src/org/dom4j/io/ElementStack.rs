// Java: import org.dom4j.Element
// Java: import org.dom4j.ElementHandler
// Java: import org.dom4j.ElementPath
use crate::org::dom4j::io::DispatchHandler::DispatchHandler;

// Java: public class ElementStack implements ElementPath
pub struct ElementStack {
    // Java field: Element[] stack
    pub(crate) stack: Vec<Option<String>>,
    // Java field: int lastElementIndex
    pub(crate) lastElementIndex: i32,
    // Java field: DispatchHandler handler
    pub(crate) handler: Option<DispatchHandler>,
    // Java field: boolean mOnlyUseName
    pub(crate) mOnlyUseName: bool,
}

// Java: public class ElementStack implements ElementPath
impl ElementStack {
    // Java: ElementStack(boolean onlyUseName)
    pub fn new(only_use_name: bool) -> Self {
        // Java: this(50, onlyUseName);
        Self::new_with_capacity(50, only_use_name)
    }

    // Java: ElementStack(int defaultCapacity, boolean onlyUseName)
    pub fn new_with_capacity(default_capacity: usize, only_use_name: bool) -> Self {
        // Java: super();
        // Java: this.lastElementIndex = -1;
        // Java: this.handler = null;
        // Java: this.stack = new Element[defaultCapacity];
        // Java: this.mOnlyUseName = onlyUseName;
        ElementStack {
            stack: vec![None; default_capacity],
            lastElementIndex: -1,
            handler: None,
            mOnlyUseName: only_use_name,
        }
    }

    // Java: void setDispatchHandler(DispatchHandler dispatchHandler)
    pub fn setDispatchHandler(&mut self, dispatch_handler: DispatchHandler) {
        // Java: this.handler = dispatchHandler;
        self.handler = Some(dispatch_handler);
    }

    // Java: DispatchHandler getDispatchHandler()
    pub fn getDispatchHandler(&self) -> &Option<DispatchHandler> {
        // Java: return this.handler;
        &self.handler
    }

    // Java: boolean getOnlyUseName()
    pub fn getOnlyUseName(&self) -> bool {
        // Java: return this.mOnlyUseName;
        self.mOnlyUseName
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.lastElementIndex = -1;
        self.lastElementIndex = -1;
    }

    // Java: Element peekElement()
    pub fn peekElement(&self) -> Option<&str> {
        // Java: return this.lastElementIndex < 0 ? null : this.stack[this.lastElementIndex];
        if self.lastElementIndex < 0 {
            None
        } else {
            self.stack.get(self.lastElementIndex as usize).and_then(|e| e.as_deref())
        }
    }

    // Java: Element popElement()
    pub fn popElement(&mut self) -> Option<String> {
        // Java: if (this.lastElementIndex < 0)
        if self.lastElementIndex < 0
        {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            // Java: int last = this.lastElementIndex--;
            let last = self.lastElementIndex as usize;
            self.lastElementIndex -= 1;
            // Java: Element ret = this.stack[last];
            // Java: this.stack[last] = null;
            // Java: return ret;
            self.stack.get_mut(last).and_then(|e| e.take())
        }
    }

    // Java: void pushElement(Element element)
    pub fn pushElement(&mut self, element: String) {
        // Java: int length = this.stack.length;
        let length = self.stack.len();
        // Java: if (++this.lastElementIndex >= length)
        self.lastElementIndex += 1;
        if self.lastElementIndex as usize >= length
        {
            // Java: this.reallocate(length * 2);
            self.reallocate(length * 2);
        }
        // Java: this.stack[this.lastElementIndex] = element;
        self.stack[self.lastElementIndex as usize] = Some(element);
    }

    // Java: void reallocate(int size)
    pub fn reallocate(&mut self, size: usize) {
        // Java: Element[] oldStack = this.stack;
        // Java: this.stack = new Element[size];
        // Java: System.arraycopy(oldStack, 0, this.stack, 0, oldStack.length);
        self.stack.resize(size, None);
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.lastElementIndex + 1;
        self.lastElementIndex + 1
    }

    // Java: Element getElement(int depth)
    pub fn getElement(&self, depth: usize) -> Option<&str> {
        // Java: Element element;
        // Java: try
        // Java: element = this.stack[depth];
        // Java: catch (ArrayIndexOutOfBoundsException var4)
        // Java: element = null;
        // Java: return element;
        self.stack.get(depth).and_then(|e| e.as_deref())
    }

    // Java: String getPath()
    pub fn getPath(&mut self) -> String {
        // Java: if (this.handler == null)
        if self.handler.is_none()
        {
            // Java: this.setDispatchHandler(new DispatchHandler());
            self.handler = Some(DispatchHandler::new());
        }
        // Java: return this.handler.getPath();
        self.handler.as_ref().map(|h| h.getPath()).unwrap_or_default()
    }

    // Java: Element getCurrent()
    pub fn getCurrent(&self) -> Option<&str> {
        // Java: return this.lastElementIndex < 0 ? null : this.stack[this.lastElementIndex];
        if self.lastElementIndex < 0 {
            None
        } else {
            self.stack.get(self.lastElementIndex as usize).and_then(|e| e.as_deref())
        }
    }

    // Java: String getCurrentName()
    pub fn getCurrentName(&self) -> Option<&str> {
        // Java: return this.lastElementIndex < 0 ? null : this.stack[this.lastElementIndex].getName();
        if self.lastElementIndex < 0 {
            None
        } else {
            self.stack.get(self.lastElementIndex as usize).and_then(|e| e.as_deref())
        }
    }

    // Java: void addHandler(String path, ElementHandler elementHandler)
    pub fn addHandler(&mut self, path: &str) {
        // Java: this.handler.addHandler(this.mOnlyUseName ? path : this.getHandlerPath(path), elementHandler);
        let _ = path;
    }

    // Java: void removeHandler(String path)
    pub fn removeHandler(&mut self, path: &str) {
        // Java: this.handler.removeHandler(this.mOnlyUseName ? path : this.getHandlerPath(path));
        let _ = path;
    }

    // Java: boolean containsHandler(String path)
    pub fn containsHandler(&self, path: &str) -> bool {
        // Java: return this.handler.containsHandler(path);
        self.handler.as_ref().map(|h| h.containsHandler(path)).unwrap_or(false)
    }

    // Java: String getHandlerPath(String path)
    fn getHandlerPath(&mut self, path: &str) -> String {
        // Java: String handlerPath;
        // Java: if (path.charAt(0) == '/')
        if path.starts_with('/')
        {
            // Java: handlerPath = path;
            path.to_string()
        }
        // Java: else
        else
        {
            // Java: String curPath = this.getPath();
            let cur_path = self.getPath();
            // Java: handlerPath = curPath + "/" + path;
            format!("{}/{}", cur_path, path)
        }
        // Java: return handlerPath;
    }

    // Java: ElementHandler getElementHandler()
    pub fn getElementHandler(&self) -> &Option<DispatchHandler> {
        // Java: return this.handler;
        &self.handler
    }
}
