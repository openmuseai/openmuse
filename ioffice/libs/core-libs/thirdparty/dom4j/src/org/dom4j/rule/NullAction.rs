// Java: import org.dom4j.Node
use crate::org::dom4j::rule::Action::Action;
use std::sync::LazyLock;

// Java field: NullAction SINGLETON
pub static SINGLETON: LazyLock<NullAction> = LazyLock::new(|| NullAction::new());

// Java: public class NullAction implements Action
pub struct NullAction {
}

// Java: public class NullAction implements Action
impl NullAction {
    // Java: NullAction()
    pub fn new() -> Self {
        // Java: super();
        NullAction {}
    }

    // Java: void run(Node node)
    pub fn run_node(&self) {
    }
}

impl Action for NullAction {
    fn run(&self) {
    }
}
