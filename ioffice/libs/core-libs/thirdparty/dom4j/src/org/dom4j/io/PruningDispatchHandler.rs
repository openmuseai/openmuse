// Java: import org.dom4j.ElementPath
use crate::org::dom4j::io::DispatchHandler::DispatchHandler;

// Java: public class PruningDispatchHandler extends DispatchHandler
pub struct PruningDispatchHandler {
    pub(crate) base: DispatchHandler,
}

// Java: public class PruningDispatchHandler extends DispatchHandler
impl PruningDispatchHandler {
    pub fn new() -> Self {
        PruningDispatchHandler {
            base: DispatchHandler::new(),
        }
    }

    // Java: void onEnd(ElementPath elementPath)
    pub fn onEnd(&self) {
        // Java: super.onEnd(elementPath);
        // Java: if (this.getActiveHandlerCount() == 0)
        {
            // Java: elementPath.getCurrent().detach();
        }
    }
}
