#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
//! Soft DrawingContainer / SoftShape for CPieceTable + CoreHelper.

use util::cn::wps::graphics::RectF::RectF;
use util::moffice::property::PropertySet::PropertySet;
use crate::cn::wps::moffice::writer::helper::soft_drawing::{
    OLE, Picture, SoftIChart, SoftIPos,
};

#[derive(Default, Clone)]
pub struct SoftShape {
    pub(crate) shape_id: i32,
    pub(crate) inline_shape: bool,
    pub(crate) child_shape: bool,
    pub(crate) parent: Option<Box<SoftShape>>,
    pub(crate) shape_pos: Option<SoftIPos>,
    pub(crate) coord_bounds: Option<RectF>,
    pub(crate) props: std::collections::HashMap<i32, i32>,
    pub(crate) picture: Option<Picture>,
    pub(crate) ole: Option<OLE>,
    pub(crate) diagram_dml: bool,
    pub(crate) shape_name: String,
    pub(crate) shape_type: i32,
    pub(crate) chart: Option<SoftIChart>,
    pub(crate) chart_shape: Option<Box<SoftShape>>,
}

impl SoftShape {
    pub fn SoftShape(shape_id: i32) -> Self {
        Self {
            shape_id,
            ..Default::default()
        }
    }
    pub fn getShapeID(&self) -> i32 {
        self.shape_id
    }
    pub fn getDrawingContainer(&self) -> DrawingContainer {
        DrawingContainer::default()
    }
    pub fn getInlineShape(&self) -> bool {
        self.inline_shape
    }
    pub fn getProperty(&self, nProp: i32) -> Option<i32> {
        self.props.get(&nProp).copied()
    }
    pub fn getShapePos(&self) -> Option<&SoftIPos> {
        self.shape_pos.as_ref()
    }
    pub fn getParent(&self) -> Option<&SoftShape> {
        self.parent.as_deref()
    }
    pub fn getCoordBounds(&self) -> Option<RectF> {
        self.coord_bounds.clone()
    }
    pub fn setPicture(&mut self, pic: Picture) {
        self.picture = Some(pic);
    }
    pub fn isChildShape(&self) -> bool {
        self.child_shape
    }
    pub fn getOLE(&self) -> Option<&OLE> {
        self.ole.as_ref()
    }
    pub fn getDiagramDrawingML(&self) -> Option<()> {
        if self.diagram_dml {
            Some(())
        } else {
            None
        }
    }
    pub fn getShapeName(&self) -> String {
        self.shape_name.clone()
    }
    pub fn getShapeType(&self) -> i32 {
        self.shape_type
    }
}

#[derive(Default, Clone)]
pub struct DrawingContainer {}

impl DrawingContainer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn addShape(&mut self, _shape: &SoftShape) {}
    pub fn delShapeByID(&mut self, _shapeId: i32) {}
    pub fn getChartShape(&self, shape: &SoftShape) -> Option<SoftShape> {
        shape.chart_shape.as_ref().map(|s| (**s).clone())
    }
    pub fn getChart(&self, shape: &SoftShape) -> Option<SoftIChart> {
        shape.chart.clone()
    }
    pub fn findShapeByID(&self, _id: i32) -> Option<SoftShape> {
        None
    }
}
