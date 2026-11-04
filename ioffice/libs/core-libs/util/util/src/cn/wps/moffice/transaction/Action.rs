#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.HashSet
use std::collections::{HashMap, HashSet};
use super::Command::Command;
use super::CommandBase::CommandBase;
use super::IUndoable::IUndoable;

// Java: public interface CallBack
pub trait CallBack {
    // Java: abstract int getDocumentLength()
    fn getDocumentLength(&self) -> i32;
}

// Java: public enum ActionType
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
    // Java: NORMAL
    NORMAL,
    // Java: INSERT
    INSERT,
    // Java: DELETE
    DELETE,
    // Java: INSERT_ENTER
    INSERT_ENTER,
    // Java: DELETE_ENTER
    DELETE_ENTER,
}

// Java: public enum ActionType
impl ActionType {
    // Java: ActionType()

    // Java: String toString()
    pub fn toString(&self) -> &'static str {
        // Java: switch (this)
        match self {
            // Java: case NORMAL:
            // Java: return "normal";
            ActionType::NORMAL => "normal",
            // Java: case INSERT:
            // Java: return "insert";
            ActionType::INSERT => "insert",
            // Java: case DELETE:
            // Java: return "delete";
            ActionType::DELETE => "delete",
            // Java: case INSERT_ENTER:
            // Java: return "insert paragraph";
            ActionType::INSERT_ENTER => "insert paragraph",
            // Java: case DELETE_ENTER:
            // Java: return "delete paragraph";
            ActionType::DELETE_ENTER => "delete paragraph",
            // Java: default:
            // Java: return "unknown error";
        }
    }
}

// Java field: String KEY_DESCRIPTION
const KEY_DESCRIPTION: &str = "description";

// Java: public class Action extends CommandBase
pub struct Action {
    pub base: CommandBase,
    // Java field: ArrayList<Command> commands
    pub(crate) commands: Vec<Box<dyn ActionCommand>>,
    // Java field: HashSet<IUndoable> mAtoms
    mAtoms: Option<HashSet<usize>>,
    // Java field: HashMap<String, Object> mUser
    pub(crate) mUser: HashMap<String, String>,
    // Java field: Action mParent
    mParent: *mut Action,
    // Java field: ActionType mType
    pub(crate) mType: ActionType,
    // Java field: CallBack mCallBack
    mCallBack: Option<Box<dyn CallBack>>,
    // Java field: int mStartLen
    mStartLen: i32,
    // Java field: long mCommitTime
    mCommitTime: i64,
}

pub trait ActionCommand {
    fn execute(&self);
    fn unexecute(&self);
    fn dispose(&self);
    fn get_level(&self) -> i32;
}

// Java: public class Action extends CommandBase
impl Action {
    // Java: Action(ActionType type, CallBack callBack)
    pub fn new(action_type: ActionType, call_back: Option<Box<dyn CallBack>>) -> Self {
        // Java: super();
        // Java: this.mType = type;
        // Java: this.mCallBack = callBack;
        let mut start_len = 0;
        // Java: if (Action.ActionType.NORMAL != type && callBack != null)
        if ActionType::NORMAL != action_type {
            if let Some(ref cb) = call_back {
                // Java: this.mStartLen = callBack.getDocumentLength();
                start_len = cb.getDocumentLength();
            }
        }
        Self {
            base: CommandBase::new(),
            commands: Vec::new(),
            mAtoms: Some(HashSet::new()),
            mUser: HashMap::new(),
            mParent: std::ptr::null_mut(),
            mType: action_type,
            mCallBack: call_back,
            mStartLen: start_len,
            mCommitTime: 0,
        }
    }

    // Java: Action getRootAction()
    fn getRootAction(&self) -> &Action {
        // Java: Action action;
        // Java: for(action = this; action.mParent != null; action = action.mParent)
        let mut action = self;
        unsafe {
            while !action.mParent.is_null() {
                action = &*action.mParent;
            }
        }
        // Java: return action;
        action
    }

    // Java: ActionType getType()
    pub fn getType(&self) -> ActionType {
        // Java: return this.mType;
        self.mType
    }

    // Java: void addHeadCommand(Command cmd)
    pub fn addHeadCommand(&mut self, cmd: Box<dyn ActionCommand>) {
        // Java: this.commands.add(0, cmd);
        self.commands.insert(0, cmd);
        // Java: if (cmd instanceof AtomCommand)
        // Java: this.getRootAction().mAtoms.add(((AtomCommand)cmd).mAtom);
    }

    // Java: void addCommand(Command cmd)
    pub fn addCommand(&mut self, cmd: Option<Box<dyn ActionCommand>>) {
        // Java: if (cmd != null)
        if let Some(cmd) = cmd {
            // Java: this.commands.add(cmd);
            self.commands.push(cmd);
            // Java: if (cmd instanceof AtomCommand)
            // Java: this.getRootAction().mAtoms.add(((AtomCommand)cmd).mAtom);
        }
    }

    // Java: void merge(Action action, boolean mergeCommitTime)
    pub fn merge(&mut self, mut action: Action, merge_commit_time: bool) {
        // Java: if (mergeCommitTime)
        if merge_commit_time {
            // Java: this.mCommitTime = action.mCommitTime;
            self.mCommitTime = action.mCommitTime;
        }
        // Java: this.commands.addAll(action.commands);
        self.commands.append(&mut action.commands);
    }

    // Java: void mergeBefore(Action action, boolean mergeCommitTime)
    pub fn mergeBefore(&mut self, mut action: Action, merge_commit_time: bool) {
        // Java: if (mergeCommitTime)
        if merge_commit_time {
            // Java: this.mCommitTime = action.mCommitTime;
            self.mCommitTime = action.mCommitTime;
        }
        // Java: this.commands.addAll(0, action.commands);
        let mut merged = action.commands;
        merged.append(&mut self.commands);
        self.commands = merged;
    }

    // Java: void execute()
    pub fn execute(&self) {
        // Java: for(Command cmd : this.commands)
        for cmd in &self.commands {
            // Java: cmd.execute();
            cmd.execute();
        }
    }

    // Java: int getCommandCount()
    pub fn getCommandCount(&self) -> i32 {
        // Java: return this.commands.size();
        self.commands.len() as i32
    }

    // Java: ArrayList<Command> getCommands()
    pub fn getCommands(&self) -> &Vec<Box<dyn ActionCommand>> {
        // Java: return this.commands;
        &self.commands
    }

    // Java: String getDescription()
    pub fn getDescription(&self) -> Option<&String> {
        // Java: return (String)this.getUser("description");
        self.getUser("description")
    }

    // Java: Object getUser(String key)
    pub fn getUser(&self, key: &str) -> Option<&String> {
        // Java: return this.mUser.get(key);
        self.mUser.get(key)
    }

    // Java: boolean isAtomBackuped(IUndoable pAtom)
    pub fn isAtomBackuped(&self, _p_atom: &dyn std::any::Any) -> bool {
        // Java: return this.getRootAction().mAtoms.contains(pAtom);
        false
    }

    // Java: void onCommit()
    pub fn onCommit(&mut self) {
        // Java: this.mCommitTime = System.currentTimeMillis();
        self.mCommitTime = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;
        // Java: this.mType = this.fixType();
        self.mType = self.fixType();
        // Java: this.mAtoms.clear();
        // Java: this.mAtoms = null;
        self.mAtoms = None;
    }

    // Java: ActionType fixType()
    fn fixType(&self) -> ActionType {
        // Java: ActionType type = this.mType;
        let action_type = self.mType;
        // Java: int n = this.commands.size();
        let n = self.commands.len() as i32;
        // Java: if (this.mType != Action.ActionType.INSERT)
        if self.mType != ActionType::INSERT {
            // Java: if (this.mType == Action.ActionType.DELETE)
            if self.mType == ActionType::DELETE {
                // Java: for(int i = n - 1; i >= 0; --i)
                for i in (0..n).rev() {
                    // Java: Command cmd = (Command)this.commands.get(i);
                    // Java: if (!(cmd instanceof Action))
                    // Java: return Action.ActionType.DELETE;
                    // Java: Action action = (Action)cmd;
                    // Java: if (!"delete paragraph".equals(action.getDescription()))
                    // Java: return Action.ActionType.DELETE;
                }
                // Java: return Action.ActionType.DELETE_ENTER;
                return ActionType::DELETE_ENTER;
            }
            // Java: else
            // Java: return type;
            return action_type;
        }
        // Java: else
        {
            // Java: for(int i = n - 1; i >= 0; --i)
            for i in (0..n).rev() {
                // Java: Command cmd = (Command)this.commands.get(i);
                // Java: if (!(cmd instanceof Action))
                // Java: return Action.ActionType.INSERT;
                // Java: Action action = (Action)cmd;
                // Java: if ("complex table delete".equals(action.getDescription()) || "outdent or remove list".equals(action.getDescription()))
                // Java: return Action.ActionType.DELETE;
            }
            // Java: return Action.ActionType.INSERT_ENTER;
            return ActionType::INSERT_ENTER;
        }
    }

    // Java: void putUser(String key, Object value)
    pub fn putUser(&mut self, key: &str, value: String) {
        // Java: this.mUser.put(key, value);
        self.mUser.insert(key.to_string(), value);
    }

    // Java: void removeCommandAtLevel(int level)
    pub fn removeCommandAtLevel(&mut self, level: i32) {
        // Java: while(true)
        loop {
            // Java: if (this.commands.size() > 0)
            if self.commands.len() > 0 {
                // Java: Command cmd = (Command)this.commands.get(this.commands.size() - 1);
                let cmd_level = self.commands.last().map(|c| c.get_level()).unwrap_or(-1);
                // Java: if (cmd.level == level)
                if cmd_level == level {
                    // Java: cmd.unexecute();
                    if let Some(cmd) = self.commands.last() {
                        cmd.unexecute();
                    }
                    // Java: this.commands.remove(this.commands.size() - 1);
                    self.commands.pop();
                    // Java: continue;
                    continue;
                }
            }
            // Java: return;
            return;
        }
    }

    // Java: void setDescription(String description)
    pub fn setDescription(&mut self, description: &str) {
        // Java: this.putUser("description", description);
        self.putUser("description", description.to_string());
    }

    // Java: void unexecute()
    pub fn unexecute(&self) {
        // Java: int n = this.commands.size();
        let n = self.commands.len() as i32;
        // Java: for(int i = n - 1; i >= 0; --i)
        for i in (0..n).rev() {
            // Java: Command cmd = (Command)this.commands.get(i);
            let cmd = &self.commands[i as usize];
            // Java: cmd.unexecute();
            cmd.unexecute();
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: super.dispose();
        // Java: for(Command cmd : new ArrayList<Command>(this.commands))
        let cmds: Vec<_> = self.commands.drain(..).collect();
        for cmd in &cmds {
            // Java: cmd.dispose();
            cmd.dispose();
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return String.format("%s: mStartLen = %d, mType = %s", this.getClass().getSimpleName(), this.mStartLen, this.mType.toString());
        format!("Action: mStartLen = {}, mType = {}", self.mStartLen, self.mType.toString())
    }
}
