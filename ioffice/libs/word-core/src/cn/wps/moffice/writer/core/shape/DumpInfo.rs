#![allow(non_snake_case)]

// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.graphics.BlipFill
use drawing::cn::wps::moffice::drawing::graphics::BlipFill::BlipFill;
// Java: import cn.wps.moffice.drawing.graphics.Picture
use drawing::cn::wps::moffice::drawing::graphics::Picture::Picture;
// Java: import cn.wps.moffice.drawing.line.LineProperty
use drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty;
use std::any::Any;
use std::sync::{Arc, Mutex};

use super::PictureLoader::BlipHandlerRef;

/// The shared Rust view of Java's BlipFill reference, including its Picture subclass.
#[derive(Clone)]
pub enum FillHandle {
    Picture(Arc<Mutex<Picture>>),
    BlipFill(Arc<Mutex<BlipFill>>),
}

impl FillHandle {
    fn from_element(value: Arc<dyn Any + Send + Sync>) -> Option<Self> {
        if let Ok(picture) = value.clone().downcast::<Mutex<Picture>>() {
            return Some(Self::Picture(picture));
        }
        value.downcast::<Mutex<BlipFill>>().ok().map(Self::BlipFill)
    }

    pub fn getMediaID(&self) -> i32 {
        match self {
            Self::Picture(value) => value.lock().expect("picture lock").getMediaID(),
            Self::BlipFill(value) => value.lock().expect("blip fill lock").getMediaID(),
        }
    }

    pub fn setMediaID(&self, id: i32) {
        match self {
            Self::Picture(value) => value.lock().expect("picture lock").setMediaID(id),
            Self::BlipFill(value) => value.lock().expect("blip fill lock").setMediaID(id),
        }
    }
}

// Java: public class DumpInfo
#[derive(Default)]
pub struct DumpInfo {
    // Java: PictureLoader.IDumpBlipHandler picHandler = null;
    pub picHandler: Option<BlipHandlerRef>,
    // Java: PictureLoader.IDumpBlipHandler fillHandler = null;
    pub fillHandler: Option<BlipHandlerRef>,
    // Java: PictureLoader.IDumpBlipHandler lineHandler = null;
    pub lineHandler: Option<BlipHandlerRef>,
}

impl DumpInfo {
    pub const NONE: i32 = 0;
    pub const PIC: i32 = 1;
    pub const FILL: i32 = 2;
    pub const LINE: i32 = 3;

    pub fn new() -> Self {
        Self::default()
    }

    // Java: public static final BlipFill getFill(Shape shape, int type)
    pub fn getFill(shape: &mut Shape, r#type: i32) -> Option<FillHandle> {
        // Java: BlipFill fill = null; switch (type)
        match r#type {
            // Java: case 1: fill = shape.getPicture(); break;
            Self::PIC => shape.getPicture().and_then(FillHandle::from_element),
            // Java: case 2: fill = shape.getBlipFill(); break;
            Self::FILL => shape.getBlipFill().and_then(FillHandle::from_element),
            // Java: case 3: LineProperty line = shape.getLineProperty();
            Self::LINE => {
                let line = shape.getLineProperty()?;
                // Java: if (line != null) { FillBase fillBase = line.getFill(); }
                let fill = if let Some(line) = line.downcast_ref::<Mutex<LineProperty>>() {
                    line.lock().expect("line property lock").getFill()
                } else if let Some(line) = line.downcast_ref::<LineProperty>() {
                    line.getFill()
                } else {
                    None
                };
                // Java: if (fillBase != null && fillBase instanceof BlipFill)
                fill.and_then(FillHandle::from_element)
            }
            _ => None,
        }
    }

    // Java: public static final int getType(Shape shape, BlipFill fill)
    pub fn getType(shape: Option<&Shape>, fill: Option<&FillHandle>) -> i32 {
        // Java: if (shape != null && fill != null)
        if shape.is_none() {
            return Self::NONE;
        }
        match fill {
            // Java: return fill instanceof Picture ? 1 : 2;
            Some(FillHandle::Picture(_)) => Self::PIC,
            Some(FillHandle::BlipFill(_)) => Self::FILL,
            // Java: else return 0;
            None => Self::NONE,
        }
    }

    // Java: public static final PictureLoader.IDumpBlipHandler getHandler(DumpInfo info, int type)
    pub fn getHandler(info: Option<&Self>, r#type: i32) -> Option<BlipHandlerRef> {
        // Java: if (info == null) return null;
        let info = info?;
        // Java: switch (type)
        match r#type {
            Self::PIC => info.picHandler.clone(),
            Self::FILL => info.fillHandler.clone(),
            Self::LINE => info.lineHandler.clone(),
            _ => None,
        }
    }
}
