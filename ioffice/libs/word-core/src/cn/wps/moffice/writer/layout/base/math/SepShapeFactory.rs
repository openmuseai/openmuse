#![allow(dead_code, non_snake_case)]
// Java: cn.wps.moffice.writer.layout.base.math.SepShapeFactory
use std::sync::Arc;
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::ShapePos::ShapePos;
use drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty;
use drawing::cn::wps::moffice::drawing::smartshape::Geometry::Geometry;
use drawing::cn::wps::moffice::drawing::smartshape::Segment::Segment;
use util::graphics::RectF::RectF;

// java public class SepShapeFactory
pub struct SepShapeFactory;

impl SepShapeFactory {
    // java public static final Shape getShape(int type, RectF rect, float lineWidth, int lineClr, float size)
    pub fn getShape(shape_type: i32, rect: &RectF, line_width: f32, line_color: i32, size: f32) -> Arc<std::sync::Mutex<Shape>> {
        let mut shape = Shape::Shape_new(None);
        let mut shape_pos = ShapePos::new();
        shape_pos.setShapeRect(Some(Arc::new(rect.clone())));
        shape_pos.setPosRelV(1);
        shape_pos.setPosRelH(1);
        shape_pos.setTextWrap(3);
        shape.setShapePos(Some(Arc::new(shape_pos)));
        let line = LineProperty::new_with_color_width(line_color, line_width);
        shape.setLineProperty(Some(Arc::new(line)));
        if let Some(geometry) = Self::getGeomery(shape_type, rect, size) {
            shape.setGeometry(Some(Arc::new(geometry)));
        }
        shape.into_shared()
    }

    // java static final Geometry getGeomery(int type, RectF orgRect, float size)
    fn getGeomery(shape_type: i32, org_rect: &RectF, size: f32) -> Option<Geometry> {
        let mut rect = org_rect.clone();
        rect.top *= 20.0;
        rect.left *= 20.0;
        rect.right *= 20.0;
        rect.bottom *= 20.0;
        let mut vertices = get_vertices(shape_type, &rect, size * 20.0)?;
        adjust_vertices(&mut vertices, rect.left, rect.top);
        let mut geometry = Geometry::new();
        geometry.setVertices(Some(Arc::new(vertices.clone())));
        geometry.setSegment(Some(Arc::new(get_segments(vertices.len() as i16, 0))));
        geometry.setGeoCordSize(Some(Arc::new(RectF::new_with(0.0, 0.0, rect.width(), rect.height()))));
        Some(geometry)
    }
}

// java KShapes.adjustVertics
fn adjust_vertices(vertices: &mut [i32], adjust_x: f32, adjust_y: f32) {
    let size = vertices.len() / 2;
    for i in 0..size {
        vertices[2 * i] = (vertices[2 * i] as f32 + adjust_x) as i32;
        vertices[2 * i + 1] = (vertices[2 * i + 1] as f32 + adjust_y) as i32;
    }
}

// java KShapes.getSegments
fn get_segments(length: i16, cmd: i16) -> Vec<Segment> {
    let mut first = Segment::new();
    first.cmd = 64;
    first.start = 0;
    let mut second = Segment::new();
    second.cmd = cmd;
    second.start = 2;
    let mut third = Segment::new();
    third.cmd = 128;
    third.start = length as i32;
    vec![first, second, third]
}

// java static final int[] getVertices(int type, RectF rect, float size)
fn get_vertices(shape_type: i32, rect: &RectF, size: f32) -> Option<Vec<i32>> {
    match shape_type {
        1 | 5 => Some(vec![rect.left as i32, rect.centerY() as i32, rect.right as i32, rect.centerY() as i32]),
        2 => {
            let dx = 0.22 * size;
            let dy = 0.3 * size;
            let dy2 = 0.06 * size;
            let mut vertices = vec![0; 10];
            vertices[0] = rect.right as i32;
            vertices[1] = rect.top as i32;
            vertices[2] = (rect.left + 0.54 * size) as i32;
            vertices[3] = vertices[1];
            vertices[4] = (vertices[2] as f32 - dx) as i32;
            vertices[5] = rect.bottom as i32;
            vertices[6] = (vertices[4] as f32 - dx) as i32;
            vertices[7] = (vertices[5] as f32 - dy) as i32;
            vertices[8] = rect.left as i32;
            vertices[9] = (vertices[7] as f32 + dy2) as i32;
            Some(vertices)
        }
        3 => Some(vec![
            rect.left as i32, rect.top as i32,
            rect.right as i32, rect.top as i32,
            rect.right as i32, rect.bottom as i32,
            rect.left as i32, rect.bottom as i32,
            rect.left as i32, rect.top as i32,
        ]),
        4 => Some(vec![rect.right as i32, rect.top as i32, rect.left as i32, rect.bottom as i32]),
        6 => {
            let height = rect.height();
            let dx = 0.22 * height;
            let dy = 0.3 * height;
            let dy2 = 0.06 * height;
            let mut vertices = vec![0; 10];
            vertices[0] = rect.right as i32;
            vertices[1] = rect.top as i32;
            vertices[2] = (rect.left + 0.54 * height) as i32;
            vertices[3] = vertices[1];
            vertices[4] = (vertices[2] as f32 - dx) as i32;
            vertices[5] = rect.bottom as i32;
            vertices[6] = (vertices[4] as f32 - dx) as i32;
            vertices[7] = (vertices[5] as f32 - dy) as i32;
            vertices[8] = rect.left as i32;
            vertices[9] = (vertices[7] as f32 + dy2) as i32;
            Some(vertices)
        }
        _ => None,
    }
}
