#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use super::atom_reflect::AtomBackupValue;
use super::AtomImpl::AtomImpl;
use super::CommandBase::CommandBase;
use super::IUndoable::IUndoable;

// Java: public class AtomCommand extends CommandBase
pub struct AtomCommand {
    pub base: CommandBase,
    // Java field: Object mArg
    pub(crate) mArg: Option<Box<dyn std::any::Any>>,
    // Java field: IUndoable mAtom
    pub(crate) mAtom: Box<dyn IUndoable>,
    // Java field: Object[] mAtomData
    pub(crate) mAtomData: Vec<AtomBackupValue>,
}

// Java: public class AtomCommand extends CommandBase
impl AtomCommand {
    // Java: AtomCommand(IUndoable atom, Object[] data, Object arg)
    pub fn new(atom: Box<dyn IUndoable>, data: Vec<AtomBackupValue>, arg: Option<Box<dyn std::any::Any>>) -> Self {
        // Java: super();
        // Java: this.mAtom = atom;
        // Java: this.mAtomData = data;
        // Java: this.mArg = arg;
        Self {
            base: CommandBase::new(),
            mAtom: atom,
            mAtomData: data,
            mArg: arg,
        }
    }

    // Java: void execute()
    pub fn execute(&mut self) {
        // Java: try
        {
            // Java: AtomImpl.swapAtom(this.mAtom, this.mAtomData);
            if AtomImpl::swapAtom(&mut *self.mAtom, &mut self.mAtomData).is_ok() {
                // Java: this.mAtom.onSwapped(this.mAtomData, this.mArg);
                let arg = self
                    .mArg
                    .as_deref()
                    .map(|b| b as &dyn std::any::Any)
                    .unwrap_or(&() as &dyn std::any::Any);
                self.mAtom.onSwapped(&self.mAtomData, arg);
            }
        }
        // Java: catch (Exception e)
        {
            // Java: e.printStackTrace();
        }
    }

    // Java: Object getArg()
    pub fn getArg(&self) -> &Option<Box<dyn std::any::Any>> {
        // Java: return this.mArg;
        &self.mArg
    }

    // Java: IUndoable getAtom()
    pub fn getAtom(&self) -> &dyn IUndoable {
        // Java: return this.mAtom;
        &*self.mAtom
    }

    // Java: Object[] getAtomData()
    pub fn getAtomData(&self) -> &Vec<AtomBackupValue> {
        // Java: return this.mAtomData;
        &self.mAtomData
    }

    // Java: void unexecute()
    pub fn unexecute(&mut self) {
        // Java: this.execute();
        self.execute();
    }
}
