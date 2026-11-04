#![allow(non_snake_case, dead_code, unused_variables)]

use super::atom_reflect::{atom_field_layout, AtomBackupValue};
use super::AtomException::AtomException;
use super::IUndoable::IUndoable;

// Java field: String CLONE
pub(crate) const CLONE: &str = "clone";
// Java field: Class<?>[] dummyClassArray
// Java field: HashMap<Class<?>, Field[]> mAtomFieldMap
// Java field: Method mShadowClone
// (replaced by atom_reflect::register_atom_layout + TypeId registry)

// Java: final class AtomImpl
pub struct AtomImpl;

// Java: final class AtomImpl
impl AtomImpl {
    // Java: static Object[] backupAtomData(IUndoable pAtom)
    pub fn backupAtomData(p_atom: &dyn IUndoable) -> Option<Vec<AtomBackupValue>> {
        // Java: try
        match p_atom.backup_atom_fields() {
            Some(data) => Some(data),
            None => {
                // Java: catch (Exception e)
                // Java: e.printStackTrace();
                // Java: return null;
                None
            }
        }
    }

    // Java: static Field[] getAtomFields(Class<?> pClass)
    pub fn get_atom_fields(owner: std::any::TypeId) -> Vec<super::atom_reflect::AtomFieldMeta> {
        // Java: Field[] pAtomFields = (Field[])mAtomFieldMap.get(pClass);
        // Java: if (pAtomFields != null) return pAtomFields;
        // Java: else { scan hierarchy for @AtomMember ... mAtomFieldMap.put ... }
        atom_field_layout(owner)
    }

    // Java: static void swapAtom(IUndoable pAtom, Object[] pData)
    pub fn swapAtom(
        p_atom: &mut dyn IUndoable,
        p_data: &mut Vec<AtomBackupValue>,
    ) -> Result<(), AtomException> {
        // Java: Field[] pFields = getAtomFields(pAtom.getClass());
        // Java: try
        p_atom.swap_atom_fields(p_data)
        // Java: catch (Exception var5)
        // Java: throw new AtomException();
    }
}
