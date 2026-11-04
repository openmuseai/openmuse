#![allow(non_snake_case, dead_code, unused_variables)]

use super::CommandBase::CommandBase;

// Java: public class CommandUpdate extends CommandBase
pub struct CommandUpdate {
    pub base: CommandBase,
}

// Java: public class CommandUpdate extends CommandBase
impl CommandUpdate {
    // Java: CommandUpdate()
    pub fn new() -> Self {
        // Java: super();
        Self {
            base: CommandBase::new(),
        }
    }

    // Java: abstract void update(Object var1)
}

pub trait CommandUpdateTrait {
    fn update(&mut self, var1: &dyn std::any::Any);
}
