#![allow(non_snake_case, dead_code)]

use super::Command::Command;

// Java: public class CommandBase extends Command
pub struct CommandBase {
    pub base: Command,
}

// Java: public class CommandBase extends Command
impl CommandBase {
    // Java: CommandBase()
    pub fn new() -> Self {
        // Java: super();
        Self {
            base: Command::new(),
        }
    }
}
