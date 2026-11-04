#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.base.assertion.Assert
use crate::base::assertion::Assert::Assert;
// Java: import java.util.ArrayList
use super::Action::{Action, ActionType, CallBack};
use super::ActionList::ActionList;
use super::Command::Command;

// Java: public interface Listener
pub trait Listener {
    // Java: abstract void beforeStart(String var1)
    fn beforeStart(&mut self, description: &str);
    // Java: abstract void afterCommit(String var1)
    fn afterCommit(&mut self, description: &str);
    // Java: abstract void afterRollback()
    fn afterRollback(&mut self);
    // Java: abstract void beforeUndo(String var1)
    fn beforeUndo(&mut self, description: &str);
    // Java: abstract void afterUndo(String var1)
    fn afterUndo(&mut self, description: &str);
    // Java: abstract void beforeRedo(String var1)
    fn beforeRedo(&mut self, description: &str);
    // Java: abstract void afterRedo(String var1)
    fn afterRedo(&mut self, description: &str);

    // Java: TransactionListeners.add(Listener). 默认空，供 `((TransactionListeners) getListener()).add` 向下调用。
    fn addListener(&mut self, _listener: Box<dyn Listener>) {}
}

// Java field: int TRANSACTION_STATE_NONE
pub const TRANSACTION_STATE_NONE: i32 = 0;
// Java field: int TRANSACTION_STATE_NORMAL
pub const TRANSACTION_STATE_NORMAL: i32 = 1;
// Java field: int TRANSACTION_STATE_UNDO
pub const TRANSACTION_STATE_UNDO: i32 = 2;
// Java field: int TRANSACTION_STATE_REDO
pub const TRANSACTION_STATE_REDO: i32 = 3;

// Java: public class Transaction
pub struct Transaction {
    // Java field: ActionList action_list
    pub(crate) action_list: Option<ActionList>,
    // Java field: Action current_action
    pub(crate) current_action: Option<Action>,
    // Java field: int level
    pub(crate) level: i32,
    // Java field: boolean mEnabled
    pub(crate) mEnabled: bool,
    // Java field: int mState
    pub(crate) mState: i32,
    // Java field: Listener mListener
    pub(crate) mListener: Option<Box<dyn Listener>>,
}

// Java: public class Transaction
impl Transaction {
    // Java: Transaction()
    pub fn new() -> Self {
        // Java: super();
        Self {
            action_list: Some(ActionList::new()),
            current_action: None,
            level: 0,
            mEnabled: true,
            mState: 0,
            mListener: None,
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: if (this.action_list != null)
        if let Some(ref mut al) = self.action_list
        {
            // Java: this.action_list.dispose();
            al.dispose();
            // Java: this.action_list.clear();
            al.clear();
            // Java: this.action_list = null;
        }
        self.action_list = None;
        // Java: if (this.current_action != null)
        if let Some(ref mut ca) = self.current_action
        {
            // Java: this.current_action.dispose();
            ca.dispose();
            // Java: this.current_action = null;
        }
        self.current_action = None;
    }

    // Java: int getState()
    pub fn getState(&self) -> i32 {
        // Java: return this.mState;
        self.mState
    }

    // Java: Action getCurrentAction()
    pub fn getCurrentAction(&self) -> &Option<Action> {
        // Java: return this.current_action;
        &self.current_action
    }

    // Java: int getLevel()
    pub fn getLevel(&self) -> i32 {
        // Java: return this.level;
        self.level
    }

    // Java: void setEnabled(boolean bEnabled)
    pub fn setEnabled(&mut self, b_enabled: bool) {
        // Java: this.mEnabled = bEnabled;
        self.mEnabled = b_enabled;
    }

    // Java: boolean getEnabled()
    pub fn getEnabled(&self) -> bool {
        // Java: return this.mEnabled;
        self.mEnabled
    }

    // Java: void start()
    pub fn start(&mut self) {
        // Java: this.start("");
        self.start_with_description("");
    }

    // Java: void start(String description)
    pub fn start_with_description(&mut self, description: &str) {
        // Java: this.start(description, Action.ActionType.NORMAL, (Action.CallBack)null);
        self.start_full(description, ActionType::NORMAL, None);
    }

    // Java: void start(String description, Action.ActionType type, Action.CallBack callBack)
    pub fn start_full(&mut self, description: &str, action_type: ActionType, call_back: Option<Box<dyn CallBack>>) {
        // Java: if (this.mEnabled)
        if self.mEnabled
        {
            // Java: if (this.level == 0)
            if self.level == 0
            {
                // Java: this.fireBeforeStart(description);
                self.fireBeforeStart(description);
                // Java: this.current_action = new Action(type, callBack);
                self.current_action = Some(Action::new(action_type, call_back));
                // Java: this.mState = 1;
                self.mState = 1;
            }
            // Java: ++this.level;
            self.level += 1;
        }
    }

    // Java: void startInner(Action.ActionType type)
    pub fn startInner(&mut self, action_type: ActionType) {
        // Java: if (this.mEnabled)
        if self.mEnabled
        {
            // Java: Action innerAction = new Action(type, (Action.CallBack)null);
            let mut inner_action = Action::new(action_type, None);
            // Java: innerAction.mParent = this.current_action;
            // Java: innerAction.level = this.level;
            inner_action.base.base.level = self.level;
            // Java: this.current_action = innerAction;
            self.current_action = Some(inner_action);
            // Java: ++this.level;
            self.level += 1;
        }
    }

    // Java: void update(Object object)
    pub fn update(&self, _object: &dyn std::any::Any) {
        // Java: ArrayList<Command> commands = this.current_action.getCommands();
        // Java: int count = commands.size();
        // Java: for(int i = 0; i < count; ++i)
        {
            // Java: Command command = (Command)commands.get(i);
            // Java: if (command instanceof CommandUpdate)
            {
                // Java: CommandUpdate update = (CommandUpdate)command;
                // Java: update.update(object);
            }
        }
    }

    // Java: void commit(String description)
    pub fn commit(&mut self, description: &str) {
        // Java: this.commit(description, false);
        self.commit_force(description, false);
    }

    // Java: void commit(String description, boolean forceCommit)
    pub fn commit_force(&mut self, description: &str, force_commit: bool) {
        // Java: if (this.mEnabled)
        if self.mEnabled
        {
            // Java: --this.level;
            self.level -= 1;
            // Java: if (this.level == 0)
            if self.level == 0
            {
                // Java: if (this.current_action.getCommandCount() > 0 || forceCommit)
                if let Some(ref mut ca) = self.current_action {
                    if ca.getCommandCount() > 0 || force_commit
                    {
                        // Java: this.current_action.setDescription(description);
                        ca.setDescription(description);
                        // Java: this.action_list.commitAction(this.current_action);
                    }
                }
                // Java: this.mState = 0;
                self.mState = 0;
                // Java: this.current_action = null;
                self.current_action = None;
                // Java: this.fireAfterCommit(description);
                self.fireAfterCommit(description);
            }
            // Java: else if (this.level == this.current_action.level)
            {
                // Java: Action innerAction = this.current_action;
                // Java: this.current_action = innerAction.mParent;
                // Java: if (innerAction.getCommandCount() > 0)
                {
                    // Java: innerAction.setDescription(description);
                    // Java: this.current_action.addCommand(innerAction);
                }
            }
        }
    }

    // Java: void submitHeadCommand(Command cmd)
    pub fn submitHeadCommand(&mut self, cmd: Box<dyn super::Action::ActionCommand>) {
        // Java: if (this.mEnabled)
        if self.mEnabled
        {
            // Java: cmd.level = this.level;
            // Java: this.current_action.addHeadCommand(cmd);
            if let Some(ref mut ca) = self.current_action {
                ca.addHeadCommand(cmd);
            }
        }
    }

    // Java: void submitCommand(Command cmd)
    pub fn submitCommand(&mut self, cmd: Option<Box<dyn super::Action::ActionCommand>>) {
        // Java: if (this.mEnabled)
        if self.mEnabled
        {
            // Java: cmd.level = this.level;
            // Java: this.current_action.addCommand(cmd);
            if let Some(ref mut ca) = self.current_action {
                ca.addCommand(cmd);
            }
        }
    }

    // Java: void writeLockAtom(IUndoable pAtom, Object arg)
    pub fn writeLockAtom(&mut self, _p_atom: &dyn std::any::Any, _arg: &dyn std::any::Any) {
        // Java: if (this.mEnabled && this.current_action != null && !this.current_action.isAtomBackuped(pAtom))
        if self.mEnabled && self.current_action.is_some()
        {
            // Java: this.submitCommand(new AtomCommand(pAtom, AtomImpl.backupAtomData(pAtom), arg));
        }
    }

    // Java: void writeLockAtom(IUndoable pAtom)
    pub fn writeLockAtom_no_arg(&mut self, _p_atom: &dyn std::any::Any) {
        // Java: this.writeLockAtom(pAtom, null);
    }

    // Java: void rollback()
    pub fn rollback(&mut self) {
        // Java: if (this.mEnabled)
        if self.mEnabled
        {
            // Java: int nStateBackup = this.mState;
            let n_state_backup = self.mState;
            // Java: this.mState = 2;
            self.mState = 2;
            // Java: if (this.current_action != null)
            if let Some(ref mut ca) = self.current_action
            {
                // Java: this.current_action.removeCommandAtLevel(this.level);
                ca.removeCommandAtLevel(self.level);
            }
            // Java: this.mState = nStateBackup;
            self.mState = n_state_backup;
            // Java: --this.level;
            self.level -= 1;
            // Java: if (this.level == 0)
            if self.level == 0
            {
                // Java: this.mState = 0;
                self.mState = 0;
                // Java: this.current_action = null;
                self.current_action = None;
                // Java: this.fireAfterRollback();
                self.fireAfterRollback();
            }
        }
    }

    // Java: void clearCurrentAction()
    pub fn clearCurrentAction(&mut self) {
        // Java: if (this.mEnabled)
        if self.mEnabled
        {
            // Java: if (this.level > 0)
            if self.level > 0
            {
                // Java: --this.level;
                self.level -= 1;
                // Java: if (this.level == 0)
                if self.level == 0
                {
                    // Java: this.mState = 0;
                    self.mState = 0;
                    // Java: this.current_action = null;
                    self.current_action = None;
                }
            }
            // Java: else
            else
            {
                // Java: Assert.fail("It should not reach here!");
                Assert::fail();
            }
        }
    }

    // Java: Action undo()
    pub fn undo(&mut self) {
        // Java: if (this.level != 0)
        if self.level != 0
        {
            // Java: throw new TransactionException("Can't perform undo within transaction");
            panic!("TransactionException: Can't perform undo within transaction");
        }
        // Java: else
        {
            // Java: this.mState = 2;
            self.mState = 2;
            // Java: Action var2;
            // Java: try
            {
                // Java: if (!this.mEnabled || !this.canUndo())
                if !self.mEnabled || !self.canUndo()
                {
                    // Java: Object var6 = null;
                    // Java: return (Action)var6;
                }
                // Java: this.fireBeforeUndo(this.action_list.getUndoAction().getDescription());
                // Java: Action rs = this.action_list.undo();
                if let Some(ref mut al) = self.action_list {
                    al.undo();
                }
                // Java: this.fireAfterUndo(rs.getDescription());
                // Java: var2 = rs;
            }
            // Java: finally
            {
                // Java: this.mState = 0;
                self.mState = 0;
            }
            // Java: return var2;
        }
    }

    // Java: Action redo()
    pub fn redo(&mut self) {
        // Java: if (this.level != 0)
        if self.level != 0
        {
            // Java: throw new TransactionException("Can't perform redo within transaction");
            panic!("TransactionException: Can't perform redo within transaction");
        }
        // Java: else
        {
            // Java: this.mState = 3;
            self.mState = 3;
            // Java: Action var2;
            // Java: try
            {
                // Java: if (!this.mEnabled || !this.canRedo())
                if !self.mEnabled || !self.canRedo()
                {
                    // Java: Object var6 = null;
                    // Java: return (Action)var6;
                }
                // Java: this.fireBeforeRedo(this.action_list.getRedoAction().getDescription());
                // Java: Action rs = this.action_list.redo();
                // Java: this.fireAfterRedo(rs.getDescription());
                // Java: var2 = rs;
            }
            // Java: finally
            {
                // Java: this.mState = 0;
                self.mState = 0;
            }
            // Java: return var2;
        }
    }

    // Java: boolean isInUndoRedo()
    pub fn isInUndoRedo(&self) -> bool {
        // Java: return this.current_action == null;
        self.current_action.is_none()
    }

    // Java: boolean canUndo()
    pub fn canUndo(&self) -> bool {
        // Java: return this.mEnabled && this.action_list.getUndoSteps() != 0;
        self.mEnabled && self.action_list.as_ref().map_or(false, |al| al.getUndoSteps() != 0)
    }

    // Java: boolean canRedo()
    pub fn canRedo(&self) -> bool {
        // Java: return this.mEnabled && this.action_list.getRedoSteps() != 0;
        self.mEnabled && self.action_list.as_ref().map_or(false, |al| al.getRedoSteps() != 0)
    }

    // Java: void clearHistory()
    pub fn clearHistory(&mut self) {
        // Java: if (this.action_list != null)
        if let Some(ref mut al) = self.action_list
        {
            // Java: this.action_list.dispose();
            al.dispose();
            // Java: this.action_list.clear();
            al.clear();
        }
        // Java: this.current_action = null;
        self.current_action = None;
        // Java: this.level = 0;
        self.level = 0;
    }

    // Java: ActionList getActionList()
    pub fn getActionList(&self) -> &Option<ActionList> {
        // Java: return this.action_list;
        &self.action_list
    }

    // Java: Action getLastAction()
    pub fn getLastAction(&self) -> Option<&Action> {
        // Java: return this.action_list.getLastAction();
        self.action_list.as_ref().and_then(|al| al.getLastAction())
    }

    // Java: Listener getListener()
    pub fn getListener(&self) -> &Option<Box<dyn Listener>> {
        // Java: return this.mListener;
        &self.mListener
    }

    pub fn listener_mut(&mut self) -> Option<&mut (dyn Listener + 'static)> {
        self.mListener.as_deref_mut()
    }

    // Java: void setListener(Listener listener)
    pub fn setListener(&mut self, listener: Option<Box<dyn Listener>>) {
        // Java: this.mListener = listener;
        self.mListener = listener;
    }

    // Java: void removeListener()
    pub fn removeListener(&mut self) {
        // Java: this.mListener = null;
        self.mListener = None;
    }

    // Java: void fireBeforeStart(String description)
    fn fireBeforeStart(&mut self, description: &str) {
        // Java: if (this.mListener != null)
        if let Some(ref mut listener) = self.mListener
        {
            // Java: this.mListener.beforeStart(description);
            listener.beforeStart(description);
        }
    }

    // Java: void fireAfterCommit(String description)
    fn fireAfterCommit(&mut self, description: &str) {
        // Java: if (this.mListener != null)
        if let Some(ref mut listener) = self.mListener
        {
            // Java: this.mListener.afterCommit(description);
            listener.afterCommit(description);
        }
    }

    // Java: void fireAfterRollback()
    fn fireAfterRollback(&mut self) {
        // Java: if (this.mListener != null)
        if let Some(ref mut listener) = self.mListener
        {
            // Java: this.mListener.afterRollback();
            listener.afterRollback();
        }
    }

    // Java: void fireAfterUndo(String description)
    fn fireAfterUndo(&mut self, description: &str) {
        // Java: if (this.mListener != null)
        if let Some(ref mut listener) = self.mListener
        {
            // Java: this.mListener.afterUndo(description);
            listener.afterUndo(description);
        }
    }

    // Java: void fireBeforeUndo(String description)
    fn fireBeforeUndo(&mut self, description: &str) {
        // Java: if (this.mListener != null)
        if let Some(ref mut listener) = self.mListener
        {
            // Java: this.mListener.beforeUndo(description);
            listener.beforeUndo(description);
        }
    }

    // Java: void fireAfterRedo(String description)
    fn fireAfterRedo(&mut self, description: &str) {
        // Java: if (this.mListener != null)
        if let Some(ref mut listener) = self.mListener
        {
            // Java: this.mListener.afterRedo(description);
            listener.afterRedo(description);
        }
    }

    // Java: void fireBeforeRedo(String description)
    fn fireBeforeRedo(&mut self, description: &str) {
        // Java: if (this.mListener != null)
        if let Some(ref mut listener) = self.mListener
        {
            // Java: this.mListener.beforeRedo(description);
            listener.beforeRedo(description);
        }
    }

    // Java: void clearRedo()
    pub fn clearRedo(&mut self) {
        // Java: this.action_list.redo_actions.clear();
        if let Some(ref mut al) = self.action_list {
            al.redo_actions.clear();
        }
    }

    // Java: Action removeFirstUndo()
    pub fn removeFirstUndo(&mut self) -> Option<Action> {
        // Java: return this.action_list.removeFirstUndo();
        self.action_list.as_mut().and_then(|al| al.removeFirstUndo())
    }

    // Java: void addUndo(Action action)
    pub fn addUndo(&mut self, action: Action) {
        // Java: this.action_list.addUndo(action);
        if let Some(ref mut al) = self.action_list {
            al.addUndo(action);
        }
    }

    // Java: void discardUndoAction(Action action)
    pub fn discardUndoAction(&mut self, action: &Action) {
        // Java: if (this.level != 0)
        if self.level != 0
        {
            // Java: throw new TransactionException("Can't perform undo within transaction");
            panic!("TransactionException: Can't perform undo within transaction");
        }
        // Java: else
        {
            // Java: this.mState = 2;
            self.mState = 2;
            // Java: try
            {
                // Java: if (this.mEnabled && this.canUndo() && action != null)
                if self.mEnabled && self.canUndo()
                {
                    // Java: this.fireBeforeUndo(this.action_list.getUndoAction().getDescription());
                    // Java: this.action_list.discardUndoAction(action);
                    // Java: this.fireAfterUndo(action.getDescription());
                }
            }
            // Java: finally
            {
                // Java: this.mState = 0;
                self.mState = 0;
            }
        }
    }
}
