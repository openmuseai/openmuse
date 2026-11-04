#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
//! Soft drawing / IO facades for CoreHelper (real drawing::Shape uses Element).

use util::cn::wps::graphics::RectF::RectF;

#[derive(Default, Clone, Debug)]
pub struct SoftFile {
    pub path: String,
    pub name: String,
    pub exists: bool,
}

impl SoftFile {
    pub fn SoftFile(path: &str) -> Self {
        Self {
            path: path.to_string(),
            name: path.rsplit('/').next().unwrap_or(path).to_string(),
            exists: true,
        }
    }
    pub fn getAbsolutePath(&self) -> String {
        self.path.clone()
    }
    pub fn getName(&self) -> String {
        self.name.clone()
    }
    pub fn exists(&self) -> bool {
        self.exists
    }
}

#[derive(Default, Clone)]
pub struct SoftIPos {
    pub rect: Option<RectF>,
}

impl SoftIPos {
    pub fn getShapeRect(&self) -> Option<RectF> {
        self.rect.clone()
    }
}

#[derive(Default, Clone, Debug)]
pub struct SoftIChart {}

impl SoftIChart {
    pub fn generatePicture(
        &self,
        _filePath: &str,
        _w: i32,
        _h: i32,
        _dpiScale: f32,
        _flag: bool,
    ) {
    }
}

#[derive(Default, Clone, Debug)]
pub struct Picture {
    pub mediaId: i32,
}

impl Picture {
    pub fn Picture(mediaId: i32) -> Self {
        Self { mediaId }
    }
}

#[derive(Default, Clone, Debug)]
pub struct OLE {}

impl OLE {
    pub fn getProgId(&self) -> Option<String> {
        None
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaTypeEnum {
    #[default]
    PICTURE,
    OLE,
}

#[derive(Default, Clone, Debug)]
pub struct MediaLib {}

impl MediaLib {
    pub fn addMedia(&mut self, _name: &str, _path: &str, _ty: MediaTypeEnum) -> i32 {
        0
    }
}

#[derive(Default, Clone, Debug)]
pub struct Geometry {}

#[derive(Default, Clone, Debug)]
pub struct Segment {}

pub struct PathTool;

impl PathTool {
    pub fn convertToCubic(
        _shapeType: i32,
        _rectF: Option<&RectF>,
        _oldVertices: Option<&[i32]>,
        _oldSegments: Option<&[Segment]>,
    ) -> Option<Geometry> {
        Some(Geometry::default())
    }
}

pub struct ClsidHelper;

impl ClsidHelper {
    pub fn getClsid(path: String) -> String {
        path
    }
}

pub struct SpaHandler;

impl SpaHandler {
    pub fn getDiagramPath(_shape: &crate::cn::wps::moffice::writer::data::DrawingContainer::SoftShape) -> Option<String> {
        None
    }
}

pub trait ICmtCustDatasWriter {
    fn writeItem(&mut self, _path: &str);
    fn writeProperties(&mut self, _path: &str);
    fn getHiddenBkmks(&self) -> Option<std::collections::HashMap<i32, Vec<String>>>;
}

pub struct SoftCmtCustDatasWriter {
    pub hidden: Option<std::collections::HashMap<i32, Vec<String>>>,
}

impl SoftCmtCustDatasWriter {
    pub fn new() -> Self {
        Self { hidden: Some(std::collections::HashMap::new()) }
    }
}

impl ICmtCustDatasWriter for SoftCmtCustDatasWriter {
    fn writeItem(&mut self, _path: &str) {}
    fn writeProperties(&mut self, _path: &str) {}
    fn getHiddenBkmks(&self) -> Option<std::collections::HashMap<i32, Vec<String>>> {
        self.hidden.clone()
    }
}

pub struct ReflectionHelper;

impl ReflectionHelper {
    pub fn getCmtCustDatasWriter(
        _owner: Option<&()>,
        _mainDoc: Option<&mut dyn crate::cn::wps::moffice::writer::core::Document::Document>,
    ) -> SoftCmtCustDatasWriter {
        SoftCmtCustDatasWriter::new()
    }
}
