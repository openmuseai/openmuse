// Java: import cn.wps.moffice.transaction.Command
use util::moffice::transaction::Command::Command;
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog::KSLog;
// Java: import com.hp.hpl.inkml.impl.InkCreator
use crate::com::hp::hpl::inkml::r#impl::InkCreator::InkCreator;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import java.util.Stack

use super::Transaction::Transaction;

// Java field: String TAG
pub(crate) const TAG: &str = "TraceCommands";

// Java: public class TraceCommands
pub struct TraceCommands {
    // Java field: Transaction transaction
    transaction: Transaction,
    // Java field: List<InkCreator> inkCreators
    pub(crate) inkCreators: Vec<InkCreator>,
    // Java field: InkEraserCommand eraserCommand
    eraserCommand: Option<InkEraserCommand>,
}

// Java: public class InkAddCommand
pub struct InkAddCommand {
    // Java field: InkCreator inkCreator
    pub(crate) inkCreator: Option<InkCreator>,
    // Java field: int index
    pub(crate) index: i32,
}

// Java: public class InkEraserCommand
pub struct InkEraserCommand {
    // Java field: Stack<InkCreator> eraserCreators
    pub(crate) eraserCreators: Vec<InkCreator>,
    // Java field: List<Integer> indexs
    pub(crate) indexs: Vec<i32>,
}

// Java: public class TraceCommands
impl TraceCommands {
    // Java: TraceCommands(Transaction transaction, List<InkCreator> inkCreators)
    pub fn TraceCommands(transaction: Transaction, ink_creators: Vec<InkCreator>) -> Self {
      // Java: super();
      // Java: this.transaction = transaction;
      // Java: this.inkCreators = inkCreators;
      Self {
          transaction,
          inkCreators: ink_creators,
          eraserCommand: None,
      }
    }

    // Java: void onInkAdd(int index)
    pub fn onInkAdd(&mut self, index: i32) {
      // Java: Command command = new InkAddCommand(index);
      // Java: this.transaction.addCommand(command);
      let command = Command::new();
      self.transaction.addCommand(command);
    }

    // Java: void onInkEraser(int index, InkCreator inkCreator)
    pub fn onInkEraser(&mut self, _index: i32, _ink_creator: InkCreator) {
      // Java: if (this.eraserCommand == null)
      if self.eraserCommand.is_none()
      {
         // Java: this.eraserCommand = new InkEraserCommand();
         self.eraserCommand = Some(InkEraserCommand::InkEraserCommand());
         // Java: this.transaction.addCommand(this.eraserCommand);
      }
      // Java: this.eraserCommand.addItem(index, inkCreator);
    }

    // Java: void onInkEraserEnd()
    pub fn onInkEraserEnd(&mut self) {
      // Java: this.eraserCommand = null;
      self.eraserCommand = None;
    }

}

// Java: public class InkAddCommand
impl InkAddCommand {
    // Java: InkAddCommand(int index)
    pub fn InkAddCommand(index: i32) -> Self {
         // Java: super();
         // Java: this.index = index;
         Self {
             inkCreator: None,
             index,
         }
    }

    // Java: void execute()
    pub fn execute(&mut self) {
         // Java: if (TraceCommands.this.inkCreators.size() == this.index && this.inkCreator != null)
         {
            // Java: TraceCommands.this.inkCreators.add(this.inkCreator);
            // Java: this.inkCreator = null;
         }
    }

    // Java: void unexecute()
    pub fn unexecute(&mut self) {
         // Java: if (TraceCommands.this.inkCreators.size() > this.index && this.inkCreator == null)
         {
            // Java: this.inkCreator = (InkCreator)TraceCommands.this.inkCreators.remove(this.index);
         }
    }

}

// Java: public class InkEraserCommand
impl InkEraserCommand {
    // Java: InkEraserCommand()
    pub fn InkEraserCommand() -> Self {
         // Java: super();
         // Java: this.eraserCreators = new Stack();
         // Java: this.indexs = new ArrayList();
         Self {
             eraserCreators: Vec::new(),
             indexs: Vec::new(),
         }
    }

    // Java: void addItem(int index, InkCreator inkCreator)
    pub fn addItem(&mut self, index: i32, ink_creator: InkCreator) {
         // Java: this.indexs.add(index);
         self.indexs.push(index);
         // Java: this.eraserCreators.push(inkCreator);
         self.eraserCreators.push(ink_creator);
    }

    // Java: void execute()
    pub fn execute(&mut self) {
         // Java: int size = this.indexs.size();
         let _size = self.indexs.len();
         // Java: if (size > 0 && this.eraserCreators.size() == 0)
         {
            // Java: for(int i = 0; i < size; ++i)
            {
               // Java: int index = (Integer)this.indexs.get(i);
               // Java: if (TraceCommands.this.inkCreators.size() > index)
               {
                  // Java: this.eraserCreators.push((InkCreator)TraceCommands.this.inkCreators.remove(index));
               }
               // Java: else
               {
                  // Java: KSLog.w(TraceCommands.TAG, "InkEraserCommand:execute: index out of bounds!");
               }
            }
         }
    }

    // Java: void unexecute()
    pub fn unexecute(&mut self) {
         // Java: int size = this.indexs.size();
         let _size = self.indexs.len();
         // Java: if (size > 0 && size == this.eraserCreators.size())
         {
            // Java: for(int i = size - 1; i >= 0; --i)
            {
               // Java: int index = (Integer)this.indexs.get(i);
               // Java: InkCreator inkCreator = (InkCreator)this.eraserCreators.pop();
               // Java: if (TraceCommands.this.inkCreators.size() == index)
               {
                  // Java: TraceCommands.this.inkCreators.add(inkCreator);
               }
               // Java: else if (TraceCommands.this.inkCreators.size() > index)
               {
                  // Java: TraceCommands.this.inkCreators.add(index, inkCreator);
               }
               // Java: else
               {
                  // Java: KSLog.w(TraceCommands.TAG, "InkEraserCommand:unexecute: index out of bounds!");
               }
            }
         }
    }

}
