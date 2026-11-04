// Java: public class Command
#[derive(Default)]
pub struct Command {
    // Java field: int level
    pub level: i32,
}

// Java: public class Command
impl Command {
    // Java: Command()
    pub fn new() -> Self {
      // Java: super();
      Self { level: 0 }
    }

    // Java: void dispose()
    pub fn dispose(&self) {
    }

    // Java: void execute()
    pub fn execute(&self) {
    }

    // Java: void unexecute()
    pub fn unexecute(&self) {
    }

}
