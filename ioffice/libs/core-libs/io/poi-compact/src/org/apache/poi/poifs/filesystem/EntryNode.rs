#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import org.apache.poi.poifs.property.Property
use crate::org::apache::poi::poifs::property::Property::Property;
use super::Entry::Entry;
use super::DirectoryEntry::DirectoryEntry;

use std::sync::Arc;

// Java: public abstract class EntryNode implements Entry
pub struct EntryNode {
    // Java field: Property _property
    pub(crate) _property: Property,
    // Java field: DirectoryNode _parent
    pub(crate) _parent: Option<*mut super::DirectoryNode::DirectoryNode>,
}

// Java: public abstract class EntryNode implements Entry
impl EntryNode {
    // Java: EntryNode(Property property, DirectoryNode parent)
    pub fn new(property: Property, parent: Option<*mut super::DirectoryNode::DirectoryNode>) -> Self {
        // Java: super();
        // Java: this._property = property;
        // Java: this._parent = parent;
        EntryNode { _property: property, _parent: parent }
    }

    // Java: Property getProperty()
    pub fn getProperty(&self) -> &Property {
        // Java: return this._property;
        &self._property
    }

    pub fn getProperty_mut(&mut self) -> &mut Property {
        &mut self._property
    }

    // Java: boolean isRoot()
    pub fn isRoot(&self) -> bool {
        // Java: return this._parent == null;
        self._parent.is_none()
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
        // Java: return this._property.getName();
        self._property.getName().to_string()
    }

    // Java: boolean isDirectoryEntry()
    pub fn isDirectoryEntry(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: boolean isDocumentEntry()
    pub fn isDocumentEntry(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: DirectoryEntry getParent()
    pub fn getParent(&self) -> Option<Arc<dyn DirectoryEntry>> {
        // Java: return this._parent;
        None
    }

    // Java: boolean delete()
    pub fn delete(&mut self) -> bool {
        // Java: boolean rval = false;
        let mut rval = false;
        // Java: if (!this.isRoot() && this.isDeleteOK())
        // Java: rval = this._parent.deleteEntry(this);
        // Java: return rval;
        rval
    }

    // Java: boolean renameTo(String newName)
    pub fn renameTo(&mut self, newName: &str) -> bool {
        // Java: boolean rval = false;
        let mut rval = false;
        // Java: if (!this.isRoot())
        // Java: rval = this._parent.changeName(this.getName(), newName);
        // Java: return rval;
        rval
    }

    // Java: abstract boolean isDeleteOK()
    pub fn dispose(&mut self) {}
}
