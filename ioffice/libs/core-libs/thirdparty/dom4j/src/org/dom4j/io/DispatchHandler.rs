// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import org.dom4j.ElementHandler
use crate::org::dom4j::ElementHandler::ElementHandler;
// Java: import org.dom4j.ElementPath
use crate::org::dom4j::ElementPath::ElementPath;
use super::ArrayListStack::ArrayListStack;

// Java: public class DispatchHandler implements ElementHandler
pub struct DispatchHandler {
    // Java field: String path
    pub(crate) path: String,
    // Java field: ArrayListStack<String> pathStack
    pathStack: ArrayListStack<String>,
    // Java field: HashMap<String, ElementHandler> handlers
    pub(crate) handlers: HashMap<String, Box<dyn ElementHandler>>,
}

// Java: public class DispatchHandler implements ElementHandler
impl DispatchHandler {
    // Java: DispatchHandler()
    pub fn new() -> Self {
        // Java: super();
        DispatchHandler {
            path: String::new(),
            pathStack: ArrayListStack::new_with_capacity(32),
            handlers: HashMap::new(),
        }
    }

    // Java: void addHandler(String handlerPath, ElementHandler handler)
    pub fn addHandler(&mut self, handlerPath: &str, handler: Box<dyn ElementHandler>) {
        // Java: this.handlers.put(handlerPath, handler);
        self.handlers.insert(handlerPath.to_string(), handler);
    }

    // Java: ElementHandler removeHandler(String handlerPath)
    pub fn removeHandler(&mut self, handlerPath: &str) -> Option<Box<dyn ElementHandler>> {
        // Java: return (ElementHandler)this.handlers.remove(handlerPath);
        self.handlers.remove(handlerPath)
    }

    // Java: boolean containsHandler(String handlerPath)
    pub fn containsHandler(&self, handlerPath: &str) -> bool {
        // Java: return this.handlers.containsKey(handlerPath);
        self.handlers.contains_key(handlerPath)
    }

    // Java: ElementHandler getHandler(String handlerPath)
    pub fn getHandler(&self, handlerPath: &str) -> Option<&dyn ElementHandler> {
        // Java: return (ElementHandler)this.handlers.get(handlerPath);
        self.handlers.get(handlerPath).map(|h| h.as_ref())
    }

    // Java: int getActiveHandlerCount()
    pub fn getActiveHandlerCount(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: void setDefaultHandler(ElementHandler handler)
    pub fn setDefaultHandler(&mut self, _handler: Box<dyn ElementHandler>) {
    }

    // Java: void resetHandlers()
    pub fn resetHandlers(&mut self) {
        // Java: this.path = "";
        self.path = String::new();
        // Java: this.handlers.clear();
        self.handlers.clear();
    }

    // Java: String getPath()
    pub fn getPath(&self) -> String {
        // Java: return this.path;
        self.path.clone()
    }
}

impl ElementHandler for DispatchHandler {
    // Java: void onStart(ElementPath elementPath)
    fn onStart(&self, elementPath: &dyn ElementPath) {
        // Java: String name = elementPath.getCurrentName();
        let name = elementPath.getCurrentName();
        // Java: this.pathStack.add(this.path);
        // Java: this.path = elementPath.getOnlyUseName() ? name : this.path + "/" + name;
        // Java: ElementHandler handler = (ElementHandler)this.handlers.get(this.path);
        // Java: if (handler != null)
        {
            // Java: handler.onStart(elementPath);
        }
    }

    // Java: void onCharacters(String chars)
    fn onCharacters(&self, chars: &str) {
        // Java: ElementHandler handler = (ElementHandler)this.handlers.get(this.path);
        // Java: if (handler != null)
        {
            // Java: handler.onCharacters(chars);
        }
    }

    // Java: void onEnd(ElementPath elementPath)
    fn onEnd(&self, elementPath: &dyn ElementPath) {
        // Java: ElementHandler handler = (ElementHandler)this.handlers.get(this.path);
        // Java: if (handler != null)
        {
            // Java: handler.onEnd(elementPath);
        }
        // Java: this.path = this.pathStack.pop();
    }

    // Java: boolean shouldNotifyChild()
    fn shouldNotifyChild(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: void onStartChild(ElementPath elementPath)
    fn onStartChild(&self, _elementPath: &dyn ElementPath) {
    }

    // Java: void onEndChild(ElementPath elementPath)
    fn onEndChild(&self, _elementPath: &dyn ElementPath) {
    }
}
