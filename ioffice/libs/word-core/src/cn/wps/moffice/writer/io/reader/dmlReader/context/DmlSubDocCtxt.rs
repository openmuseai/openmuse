#![allow(non_snake_case)]
use std::{cell::RefCell, rc::Rc};
use super::TxbxChains::TxbxChains;
use crate::cn::wps::moffice::writer::io::reader::dmlReader::PositioningContext::PositioningContext;

// Java: public class DmlSubDocCtxt
// The shape-importer type is supplied when the Java DmlShapeImporter class is
// compiled. This preserves the concrete Java reference and shared identity
// without introducing an erased pointer or a fake importer implementation.
pub struct DmlSubDocCtxt<ShapeImporter> {
    // Java: public PositioningContext mPositioningCtxt;
    pub mPositioningCtxt: Option<Rc<RefCell<PositioningContext>>>,
    // Java: public TxbxChains mTxbxChains;
    pub mTxbxChains: Option<Rc<RefCell<TxbxChains>>>,
    // Java: public DmlShapeImporter mLastDmlShapeImporter;
    pub mLastDmlShapeImporter: Option<Rc<RefCell<ShapeImporter>>>,
    // Java: public String mGVmlThemePath = null;
    pub mGVmlThemePath: Option<String>,
}

impl<ShapeImporter> Default for DmlSubDocCtxt<ShapeImporter> {
    fn default() -> Self {
        Self {
            mPositioningCtxt: None,
            mTxbxChains: None,
            mLastDmlShapeImporter: None,
            mGVmlThemePath: None,
        }
    }
}

impl<ShapeImporter> DmlSubDocCtxt<ShapeImporter> {
    // Java: implicit DmlSubDocCtxt()
    pub fn new() -> Self { Self::default() }
}
