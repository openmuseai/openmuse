#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.collections.KArrayDeque
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.List
use std::collections::VecDeque;
use super::Action::{Action, ActionType, CallBack};

// Java: public interface ActionChangeListener
pub trait ActionChangeListener {
    // Java: abstract void beforeMerge(Action var1, Action var2)
    fn beforeMerge(&mut self, last: &mut Action, current: &Action);
    // Java: abstract void onAbandonUndoAction(Action var1)
    fn onAbandonUndoAction(&mut self, action: Action);
}

// Java field: int MAX_UNDO_STEPS
const MAX_UNDO_STEPS: i32 = 50;
// Java field: int MAX_CHAR_COUNT
const MAX_CHAR_COUNT: i32 = 200;
// Java field: long MAX_DELAY
const MAX_DELAY: i64 = 10000;

// Java: public class ActionList
pub struct ActionList {
    // Java field: ArrayList<Action> redo_actions
    pub(crate) redo_actions: Vec<Action>,
    // Java field: KArrayDeque<Action> undo_actions
    pub(crate) undo_actions: VecDeque<Action>,
    // Java field: ActionChangeListener mActionChangeListener
    pub(crate) mActionChangeListener: Option<Box<dyn ActionChangeListener>>,
    // Java field: boolean mInterruptNext
    pub(crate) mInterruptNext: bool,
    // Java field: long mMaxMergeTime
    mMaxMergeTime: i64,
    // Java field: int mMaxMergeCharCount
    mMaxMergeCharCount: i32,
    // Java field: boolean mMergeCommitTime
    mMergeCommitTime: bool,
}

// Java: public class ActionList
impl ActionList {
    // Java: ActionList()
    pub fn new() -> Self {
        // Java: super();
        Self {
            redo_actions: Vec::new(),
            undo_actions: VecDeque::with_capacity(50),
            mActionChangeListener: None,
            mInterruptNext: false,
            mMaxMergeTime: 10000,
            mMaxMergeCharCount: 200,
            mMergeCommitTime: true,
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.undo_actions = new KArrayDeque<Action>(50);
        self.undo_actions = VecDeque::with_capacity(50);
        // Java: this.redo_actions = new ArrayList();
        self.redo_actions = Vec::new();
    }

    // Java: void setMaxMergeTime(long ms)
    pub fn setMaxMergeTime(&mut self, ms: i64) {
        // Java: this.mMaxMergeTime = ms;
        self.mMaxMergeTime = ms;
    }

    // Java: void setMaxMergeCharCount(int count)
    pub fn setMaxMergeCharCount(&mut self, count: i32) {
        // Java: this.mMaxMergeCharCount = count;
        self.mMaxMergeCharCount = count;
    }

    // Java: void setMergeCommitTime(boolean merge)
    pub fn setMergeCommitTime(&mut self, merge: bool) {
        // Java: this.mMergeCommitTime = merge;
        self.mMergeCommitTime = merge;
    }

    // Java: boolean getInterruptNext()
    pub fn getInterruptNext(&self) -> bool {
        // Java: return this.mInterruptNext;
        self.mInterruptNext
    }

    // Java: void setInterruptNext(boolean interruptNext)
    pub fn setInterruptNext(&mut self, interrupt_next: bool) {
        // Java: this.mInterruptNext = interruptNext;
        self.mInterruptNext = interrupt_next;
    }

    // Java: ActionChangeListener getActionChangeListener()
    pub fn getActionChangeListener(&self) -> &Option<Box<dyn ActionChangeListener>> {
        // Java: return this.mActionChangeListener;
        &self.mActionChangeListener
    }

    // Java: void setActionChangeListener(ActionChangeListener listener)
    pub fn setActionChangeListener(&mut self, listener: Option<Box<dyn ActionChangeListener>>) {
        // Java: this.mActionChangeListener = listener;
        self.mActionChangeListener = listener;
    }

    // Java: void commitAction(Action action)
    pub fn commitAction(&mut self, action: Option<Action>) {
        // Java: if (action != null)
        if let Some(mut action) = action
        {
            // Java: action.onCommit();
            action.onCommit();
            // Java: if (this.canMerge(action))
            if false
            {
                // Java: Action lastAction = this.getLastAction();
                // Java: if (this.mActionChangeListener != null)
                {
                    // Java: this.mActionChangeListener.beforeMerge(lastAction, action);
                }
                // Java: lastAction.merge(action, this.mMergeCommitTime);
            }
            // Java: else
            else
            {
                // Java: if (this.undo_actions.size() >= 50)
                if self.undo_actions.len() >= 50
                {
                    // Java: Action firstAction = this.undo_actions.pollFirst();
                    let first_action = self.undo_actions.pop_front();
                    // Java: if (this.mActionChangeListener != null && firstAction != null)
                    if let (Some(listener), Some(first)) = (&mut self.mActionChangeListener, first_action)
                    {
                        // Java: this.mActionChangeListener.onAbandonUndoAction(firstAction);
                        listener.onAbandonUndoAction(first);
                    }
                }
                // Java: this.undo_actions.add(action);
                self.undo_actions.push_back(action);
            }
        }
        // Java: this.mInterruptNext = false;
        self.mInterruptNext = false;
        // Java: this.redo_actions.clear();
        self.redo_actions.clear();
    }

    // Java: boolean canMerge(Action action)
    fn canMerge(&self, action: &Action) -> bool {
        // Java: if (!this.mInterruptNext && this.undo_actions.size() != 0 && action.mType != Action.ActionType.NORMAL)
        if !self.mInterruptNext && !self.undo_actions.is_empty() && action.mType != ActionType::NORMAL
        {
            // Java: Action lastAction = this.getLastAction();
            // Java: if (lastAction.mType != action.mType)
            {
                // Java: return false;
                return false;
            }
            // Java: else if (action.mCommitTime - lastAction.mCommitTime > this.mMaxMergeTime)
            {
                // Java: return false;
                return false;
            }
            // Java: else
            {
                // Java: int length = action.mCallBack.getDocumentLength();
                // Java: return Math.abs(length - lastAction.mStartLen) <= 200;
                return false;
            }
        }
        // Java: else
        {
            // Java: return false;
            return false;
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: if (this.undo_actions != null)
        {
            // Java: Action[] undoActions = (Action[])this.undo_actions.toArray(new Action[this.undo_actions.size()]);
            // Java: this.disposeAction(Arrays.asList(undoActions));
        }
        // Java: if (this.redo_actions != null)
        {
            // Java: this.disposeAction(this.redo_actions);
        }
        // Java: this.undo_actions = null;
        // Java: this.redo_actions = null;
    }

    // Java: void disposeAction(List<Action> actionList)
    fn disposeAction(&mut self, action_list: &mut Vec<Action>) {
        // Java: if (actionList != null && !actionList.isEmpty())
        if !action_list.is_empty()
        {
            // Java: for(Action action : new ArrayList<Action>(actionList))
            let actions: Vec<Action> = action_list.drain(..).collect();
            for mut action in actions
            {
                // Java: action.dispose();
                action.dispose();
            }
        }
    }

    // Java: Action getLastAction()
    pub fn getLastAction(&self) -> Option<&Action> {
        // Java: return this.undo_actions.peekLast();
        self.undo_actions.back()
    }

    // Java: Action getRedoAction()
    fn getRedoAction(&self) -> Option<&Action> {
        // Java: int nRedo = this.redo_actions.size();
        let n_redo = self.redo_actions.len();
        // Java: return nRedo == 0 ? null : (Action)this.redo_actions.get(nRedo - 1);
        if n_redo == 0 { None } else { self.redo_actions.get(n_redo - 1) }
    }

    // Java: int getRedoSteps()
    pub fn getRedoSteps(&self) -> i32 {
        // Java: return this.redo_actions.size();
        self.redo_actions.len() as i32
    }

    // Java: Action getUndoAction()
    fn getUndoAction(&self) -> Option<&Action> {
        // Java: return this.undo_actions.peekLast();
        self.undo_actions.back()
    }

    // Java: int getUndoSteps()
    pub fn getUndoSteps(&self) -> i32 {
        // Java: return this.undo_actions.size();
        self.undo_actions.len() as i32
    }

    // Java: Action redo()
    pub fn redo(&mut self) -> Option<&Action> {
        // Java: int nRedo = this.redo_actions.size();
        let n_redo = self.redo_actions.len();
        // Java: if (nRedo == 0)
        if n_redo == 0
        {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            // Java: Action action = (Action)this.redo_actions.get(nRedo - 1);
            let action = self.redo_actions.remove(n_redo - 1);
            // Java: action.execute();
            action.execute();
            // Java: this.undo_actions.add(action);
            self.undo_actions.push_back(action);
            // Java: this.redo_actions.remove(nRedo - 1);
            // Java: return action;
            self.undo_actions.back()
        }
    }

    // Java: Action undo()
    pub fn undo(&mut self) {
        // Java: int nUndo = this.undo_actions.size();
        let n_undo = self.undo_actions.len();
        // Java: if (nUndo == 0)
        if n_undo == 0
        {
            // Java: return null;
            return;
        }
        // Java: else
        {
            // Java: Action action = this.undo_actions.peekLast();
            // Java: action.unexecute();
            // Java: this.redo_actions.add(action);
            // Java: this.undo_actions.pollLast();
            if let Some(action) = self.undo_actions.pop_back() {
                action.unexecute();
                self.redo_actions.push(action);
            }
            // Java: return action;
        }
    }

    // Java: Action removeFirstUndo()
    pub fn removeFirstUndo(&mut self) -> Option<Action> {
        // Java: return this.undo_actions.pollLast();
        self.undo_actions.pop_back()
    }

    // Java: void addUndo(Action action)
    pub fn addUndo(&mut self, action: Action) {
        // Java: this.undo_actions.addLast(action);
        self.undo_actions.push_back(action);
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append(this.getClass().getSimpleName());
        sb.push_str("ActionList");
        // Java: sb.append(" undo_actions :\n");
        sb.push_str(" undo_actions :\n");
        // Java: for(Action action : this.undo_actions)
        for action in &self.undo_actions
        {
            // Java: sb.append(action.toString());
            sb.push_str(&action.toString());
            // Java: sb.append("\n");
            sb.push('\n');
        }
        // Java: return sb.toString();
        sb
    }

    // Java: KArrayDeque<Action> getUndoActions()
    pub fn getUndoActions(&self) -> &VecDeque<Action> {
        // Java: return this.undo_actions;
        &self.undo_actions
    }

    // Java: void discardUndoAction(Action action)
    fn discardUndoAction(&mut self, action: &Action) {
        // Java: int nUndo = this.undo_actions.size();
        let n_undo = self.undo_actions.len() as i32;
        // Java: if (nUndo != 0 && action != null)
        if n_undo != 0
        {
            // Java: for(int i = nUndo - 1; i >= 0; --i)
            for _i in (0..n_undo).rev()
            {
                // Java: if (action.equals(this.undo_actions.get(i)))
                {
                    // Java: action.unexecute();
                    // Java: this.undo_actions.remove(action);
                    // Java: return;
                }
            }
        }
    }
}
