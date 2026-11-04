#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use crate::org::apache::poi::poifs::common::POIFSBigBlockSize::POIFSBigBlockSize;
use crate::org::apache::poi::poifs::common::POIFSConstants;
use crate::org::apache::poi::poifs::property::DirectoryProperty::DirectoryProperty;
use crate::org::apache::poi::poifs::property::PropertyTable::PropertyTable;
use super::DirectoryNode::DirectoryNode;
use super::POIFSDocument::POIFSDocument;

use std::io::{self, Read, Write};

// Java: public class POIFSFileSystem implements IPOIFSFileSystem
pub struct POIFSFileSystem {
    pub(crate) _property_table: PropertyTable,
    pub(crate) _documents: Vec<POIFSDocument>,
    pub(crate) _root: Option<Box<DirectoryNode>>,
    pub(crate) bigBlockSize: POIFSBigBlockSize,
}

// Java: public class POIFSFileSystem implements IPOIFSFileSystem
impl POIFSFileSystem {
    pub fn new() -> Self {
        let bigBlockSize = POIFSBigBlockSize::new(512, 3);
        let _property_table = PropertyTable::new(POIFSBigBlockSize::new(512, 3));
        POIFSFileSystem {
            _property_table,
            _documents: Vec::new(),
            _root: None,
            bigBlockSize,
        }
    }

    pub fn from_stream(stream: &mut dyn Read) -> io::Result<Self> {
        todo!()
    }

    pub fn getRoot(&mut self) -> &mut DirectoryNode {
        if self._root.is_none() {
            let root_property = DirectoryProperty::new("Root Entry".to_string());
            let self_ptr = self as *mut POIFSFileSystem;
            self._root = Some(Box::new(DirectoryNode::new(root_property, self_ptr, None)));
        }
        self._root.as_mut().unwrap()
    }

    pub fn addDocument(&mut self, document: POIFSDocument) {
        self._documents.push(document);
    }

    pub fn addDirectory(&mut self, property: DirectoryProperty) {}

    pub fn remove(&mut self, _entry: &super::EntryNode::EntryNode) {}

    pub fn writeFilesystem(&mut self, stream: &mut dyn Write) -> io::Result<()> {
        todo!()
    }

    pub fn close(self) { drop(self); }

    pub fn dispose(&mut self) {
        if let Some(ref mut root) = self._root {
            root.dispose();
        }
        self._root = None;
        self._documents.clear();
    }
}
