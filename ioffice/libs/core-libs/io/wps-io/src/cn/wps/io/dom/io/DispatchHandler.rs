// Java: import cn.wps.io.dom.ElementHandler
// Java: import cn.wps.io.dom.ElementPath
// Java: import java.util.HashMap
use std::collections::HashMap;
use super::ArrayListStack::ArrayListStack;

type Element = Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>;

// Java: public class DispatchHandler implements ElementHandler
pub struct DispatchHandler {
    // Java field: String path
    pub(crate) path: String,
    // Java field: ArrayListStack<String> pathStack
    pathStack: Vec<String>,
    // Java field: HashMap<String, ElementHandler> handlers
    pub(crate) handlers: HashMap<String, i32>,
    // Java field: StringBuilder mStrBuilder
}

// Java: public class DispatchHandler implements ElementHandler
impl DispatchHandler {
    // Java: DispatchHandler()
    pub fn new() -> Self {
        // Java: super();
        // Java: mStrBuilder = new StringBuilder(128);
        DispatchHandler {
            path: String::new(),
            pathStack: Vec::with_capacity(32),
            handlers: HashMap::new(),
        }
    }

    // Java: void addHandler(String handlerPath, ElementHandler handler)
    pub fn addHandler(&mut self, handlerPath: &str, handler: i32) {        // Java: this.handlers.put(handlerPath, handler);
        self.handlers.insert(handlerPath.to_string(), handler);
    }

    // Java: ElementHandler removeHandler(String handlerPath)
    pub fn removeHandler(&mut self, handlerPath: &str) -> Option<i32> {        // Java: return (ElementHandler)this.handlers.remove(handlerPath);
        self.handlers.remove(handlerPath)
    }

    // Java: boolean containsHandler(String handlerPath)
    pub fn containsHandler(&self, handlerPath: &str) -> bool {        // Java: return this.handlers.containsKey(handlerPath);
        self.handlers.contains_key(handlerPath)
    }

    // Java: ElementHandler getHandler(String handlerPath)
    pub fn getHandler(&self, handlerPath: &str) -> Option<&i32> {        // Java: return (ElementHandler)this.handlers.get(handlerPath);
        self.handlers.get(handlerPath)
    }

    // Java: int getActiveHandlerCount()
    pub fn getActiveHandlerCount(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: void setDefaultHandler(ElementHandler handler)
    pub fn setDefaultHandler(&mut self, _handler: i32) {    }

    // Java: void resetHandlers()
    pub fn resetHandlers(&mut self) {
        // Java: this.path = "";
        self.path = String::new();
        // Java: this.handlers.clear();
        self.handlers.clear();
    }

    // Java: String getPath()
    pub fn getPath(&self) -> &str {
        // Java: return this.path;
        &self.path
    }

    // Java: void onStart(ElementPath elementPath)
    pub fn onStart_dispatch(&mut self, current_name: &str, only_use_name: bool) {        // Java: String name = elementPath.getCurrentName();
        let name = current_name;
        // Java: this.pathStack.add(this.path);
        self.pathStack.push(self.path.clone());
        // Java: if (elementPath.getOnlyUseName())
        if only_use_name
        {
            // Java: this.path = name;
            self.path = name.to_string();
        }
        // Java: else
        else
        {
            // Java: this.path = mStrBuilder.append(this.path).append('/').append(name).toString();
            self.path = format!("{}/{}", self.path, name);
            // Java: mStrBuilder.setLength(0);
        }
        // Java: ElementHandler handler = (ElementHandler)this.handlers.get(this.path);
        let _handler = self.handlers.get(&self.path);
        // Java: if (handler != null)
        if _handler.is_some()
        {
            // Java: handler.onStart(elementPath);
        }
    }

    // Java: void onCharacters(String chars)
    pub fn onCharacters_dispatch(&mut self, _chars: &str) {        // Java: ElementHandler handler = (ElementHandler)this.handlers.get(this.path);
        let _handler = self.handlers.get(&self.path);
        // Java: if (handler != null)
        if _handler.is_some()
        {
            // Java: handler.onCharacters(chars);
        }
    }

    // Java: void onEnd(ElementPath elementPath)
    pub fn onEnd_dispatch(&mut self) {
        // Java: ElementHandler handler = (ElementHandler)this.handlers.get(this.path);
        let _handler = self.handlers.get(&self.path);
        // Java: if (handler != null)
        if _handler.is_some()
        {
            // Java: handler.onEnd(elementPath);
        }
        // Java: this.path = this.pathStack.pop();
        self.path = self.pathStack.pop().unwrap_or_default();
    }

    // Java: boolean shouldNotifyChild()
    pub fn shouldNotifyChild(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: void onStartChild(ElementPath elementPath)
    pub fn onStartChild(&self) {
    }

    // Java: void onEndChild(ElementPath elementPath)
    pub fn onEndChild(&self) {
    }
}
