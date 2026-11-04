// Java: import cn.wps.moffice.transaction.Command
use util::moffice::transaction::Command::Command;
// Java: import java.util.Stack

// Java: public class Transaction
pub struct Transaction {
    // Java field: Stack<Command> undoCommands
    pub(crate) undoCommands: Vec<Command>,
    // Java field: Stack<Command> redoCommands
    pub(crate) redoCommands: Vec<Command>,
}

// Java: public class Transaction
impl Transaction {
    // Java: Transaction()
    pub fn Transaction() -> Self {
      // Java: super();
      Self {
          undoCommands: Vec::new(),
          redoCommands: Vec::new(),
      }
    }

    // Java: boolean canUndo()
    pub fn canUndo(&self) -> bool {
      // Java: return this.undoCommands.size() > 0;
      !self.undoCommands.is_empty()
    }

    // Java: void undo()
    pub fn undo(&mut self) {
      // Java: if (this.canUndo())
      if !self.undoCommands.is_empty()
      {
         // Java: Command command = (Command)this.undoCommands.pop();
         let command = self.undoCommands.pop().unwrap();
         // Java: command.unexecute();
         command.unexecute();
         // Java: this.redoCommands.push(command);
         self.redoCommands.push(command);
      }
    }

    // Java: boolean canRedo()
    pub fn canRedo(&self) -> bool {
      // Java: return this.redoCommands.size() > 0;
      !self.redoCommands.is_empty()
    }

    // Java: void redo()
    pub fn redo(&mut self) {
      // Java: if (this.canRedo())
      if !self.redoCommands.is_empty()
      {
         // Java: Command command = (Command)this.redoCommands.pop();
         let command = self.redoCommands.pop().unwrap();
         // Java: command.execute();
         command.execute();
         // Java: this.undoCommands.push(command);
         self.undoCommands.push(command);
      }
    }

    // Java: void addCommand(Command command)
    pub fn addCommand(&mut self, command: Command) {
      // Java: this.undoCommands.push(command);
      self.undoCommands.push(command);
      // Java: this.redoCommands.clear();
      self.redoCommands.clear();
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.undoCommands.clear();
      self.undoCommands.clear();
      // Java: this.redoCommands.clear();
      self.redoCommands.clear();
    }

}
