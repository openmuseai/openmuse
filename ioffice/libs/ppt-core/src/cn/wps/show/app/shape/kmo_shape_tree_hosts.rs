#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]
//! Compile-surface hosts for parked KmoShapeTree signatures (pp-19 function 02).
//! Not under libs/core-libs/stubs.

use crate::cn::wps::kso::kproto::KoBin::KoBin;

#[derive(Default, Clone, Debug)]
pub struct DgCanvas;
#[derive(Default, Clone, Debug)]
pub struct KmoOleChart;
#[derive(Default, Clone, Debug)]
pub struct GroupTransform2D;
#[derive(Default, Clone, Debug)]
pub struct Transform2D;
#[derive(Default, Clone, Debug)]
pub struct TableStyle;
#[derive(Default, Clone, Debug)]
pub struct PointF;
#[derive(Default, Clone, Debug)]
pub struct Rect;
#[derive(Default, Clone, Debug)]
pub struct File;
#[derive(Default, Clone, Debug)]
pub struct FileDataStorage;
#[derive(Default, Clone, Debug)]
pub struct KmoMediaUtil;
#[derive(Default, Clone, Debug)]
pub struct KmoTheme;
#[derive(Default, Clone, Debug)]
pub struct KmoVmlDrawing;
#[derive(Default, Clone, Debug)]
pub struct ShowShape;
#[derive(Default, Clone, Debug)]
pub struct Ink;

pub trait KmoProtocol {
    fn protocol(&self) -> Box<dyn KoBin>;
    fn updateProtocol(&mut self, data: &dyn KoBin);
}
