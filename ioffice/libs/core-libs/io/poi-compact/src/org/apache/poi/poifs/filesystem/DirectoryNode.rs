#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import org.apache.poi.hpsf.ClassID
use crate::org::apache::poi::hpsf::ClassID::ClassID;
// Java: import org.apache.poi.poifs.property.DirectoryProperty
use crate::org::apache::poi::poifs::property::DirectoryProperty::DirectoryProperty;
// Java: import org.apache.poi.poifs.property.DocumentProperty
use crate::org::apache::poi::poifs::property::DocumentProperty::DocumentProperty;
// Java: import org.apache.poi.poifs.property.Property
use crate::org::apache::poi::poifs::property::Property::Property;
use super::EntryNode::EntryNode;
use super::Entry::Entry;
use super::DirectoryEntry::DirectoryEntry;
use super::POIFSDocumentPath::POIFSDocumentPath;
use super::DocumentInputStream::DocumentInputStream;
use super::POIFSDocument::POIFSDocument;

use std::collections::HashMap;
use std::sync::Arc;

// Java: public class DirectoryNode extends EntryNode implements DirectoryEntry, Iterable<Entry>
pub struct DirectoryNode {
    pub(crate) base: EntryNode,
    // Java field: Map<String, Entry> _byname
    _byname: HashMap<String, usize>,
    // Java field: ArrayList<Entry> _entries
    _entries: Vec<Box<EntryNode>>,
    // Java field: POIFSFileSystem _filesystem
    _filesystem: Option<*mut super::POIFSFileSystem::POIFSFileSystem>,
    // Java field: POIFSDocumentPath _path
    _path: POIFSDocumentPath,
}

// Java: public class DirectoryNode extends EntryNode implements DirectoryEntry, Iterable<Entry>
impl DirectoryNode {
    // Java: DirectoryNode(DirectoryProperty property, POIFSFileSystem filesystem, DirectoryNode parent)
    pub fn new(property: DirectoryProperty, filesystem: *mut super::POIFSFileSystem::POIFSFileSystem, parent: Option<*mut DirectoryNode>) -> Self {
        // Java: super(property, parent);
        let base = EntryNode::new(property.base, parent.map(|p| p as *mut DirectoryNode));
        // Java: if (parent == null)
        let _path = if parent.is_none() {
            // Java: this._path = new POIFSDocumentPath();
            POIFSDocumentPath::new()
        } else {
            // Java: this._path = new POIFSDocumentPath(parent._path, new String[]{property.getName()});
            POIFSDocumentPath::new()
        };
        // Java: this._filesystem = filesystem;
        // Java: this._byname = new HashMap();
        // Java: this._entries = new ArrayList();
        DirectoryNode {
            base,
            _byname: HashMap::new(),
            _entries: Vec::new(),
            _filesystem: Some(filesystem),
            _path,
        }
    }

    // Java: DirectoryNode(DirectoryProperty property, POIFSFileSystem filesystem, DirectoryNode parent, int dummy)
    pub fn new_lazy(property: DirectoryProperty, filesystem: *mut super::POIFSFileSystem::POIFSFileSystem, parent: Option<*mut DirectoryNode>) -> Self {
        Self::new(property, filesystem, parent)
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: if (this._byname != null) this._byname.clear();
        self._byname.clear();
        self._entries.clear();
        self._filesystem = None;
    }

    // Java: POIFSDocumentPath getPath()
    pub fn getPath(&self) -> &POIFSDocumentPath {
        // Java: return this._path;
        &self._path
    }

    // Java: POIFSFileSystem getFileSystem()
    pub fn getFileSystem(&self) -> Option<*mut super::POIFSFileSystem::POIFSFileSystem> {
        // Java: return this._filesystem;
        self._filesystem
    }

    // Java: DocumentInputStream createDocumentInputStream(String documentName)
    pub fn createDocumentInputStream_by_name(&self, documentName: &str) -> std::io::Result<DocumentInputStream> {
        todo!()
    }

    // Java: DocumentInputStream createDocumentInputStream(Entry document)
    pub fn createDocumentInputStream(&self, document: &dyn Entry) -> std::io::Result<DocumentInputStream> {
        todo!()
    }

    // Java: DocumentEntry createDocument(POIFSDocument document)
    pub fn createDocument_internal(&mut self, document: POIFSDocument) -> std::io::Result<()> {
        todo!()
    }

    // Java: boolean changeName(String oldName, String newName)
    pub fn changeName(&mut self, oldName: &str, newName: &str) -> bool {
        // Java: EntryNode child = (EntryNode)this._byname.get(oldName);
        // Java: if (child != null) ...
        todo!()
    }

    // Java: boolean deleteEntry(EntryNode entry)
    pub fn deleteEntry(&mut self, entry: &EntryNode) -> bool {
        todo!()
    }

    // Java: Iterator<Entry> getEntries()
    pub fn getEntries(&self) -> &Vec<Box<EntryNode>> {
        // Java: return this._entries.iterator();
        &self._entries
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this._entries.isEmpty();
        self._entries.is_empty()
    }

    // Java: int getEntryCount()
    pub fn getEntryCount(&self) -> i32 {
        // Java: return this._entries.size();
        self._entries.len() as i32
    }

    // Java: boolean hasEntry(String name)
    pub fn hasEntry(&self, name: &str) -> bool {
        // Java: return this._byname.get(name) != null;
        self._byname.contains_key(name)
    }

    // Java: Entry getEntry(String name)
    pub fn getEntry(&self, name: &str) -> Option<&EntryNode> {
        // Java: Entry rval = (Entry)this._byname.get(name);
        self._byname.get(name).and_then(|&idx| self._entries.get(idx).map(|e| e.as_ref()))
    }

    // Java: Entry getEntry2(String name)
    pub fn getEntry2(&self, name: &str) -> Option<&EntryNode> {
        self.getEntry(name)
    }

    // Java: DirectoryNode createDirectory(String name)
    pub fn createDirectory(&mut self, name: &str) -> std::io::Result<()> {
        todo!()
    }

    // Java: ClassID getStorageClsid()
    pub fn getStorageClsid(&self) -> &ClassID {
        // Java: return this.getProperty().getStorageClsid();
        self.base.getProperty().getStorageClsid()
    }

    // Java: void setStorageClsid(ClassID clsidStorage)
    pub fn setStorageClsid(&mut self, clsidStorage: ClassID) {
        // Java: this.getProperty().setStorageClsid(clsidStorage);
        self.base.getProperty_mut().setStorageClsid(clsidStorage);
    }

    // Java: boolean isDirectoryEntry()
    pub fn isDirectoryEntry(&self) -> bool {
        // Java: return true;
        true
    }

    // Java: boolean isDeleteOK()
    pub fn isDeleteOK(&self) -> bool {
        // Java: return this.isEmpty();
        self.isEmpty()
    }

    pub fn getName(&self) -> String {
        self.base.getName().to_string()
    }
}
