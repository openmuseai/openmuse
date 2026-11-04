#![allow(dead_code, non_snake_case)]

// java public final class DrawingResizeTool
pub struct DrawingResizeTool;

impl DrawingResizeTool {
    // java public static void expandRect(ITypoRect relative, int pl, int pt, int pr, int pb, ITypoRect outRect)
    pub fn expanded_width(relative_width: i32, pl: i32, pr: i32) -> i32 {
        relative_width + pl + pr
    }

    pub fn expanded_height(relative_height: i32, pt: i32, pb: i32) -> i32 {
        relative_height + pt + pb
    }

    // java public static void reverseRotateEmbedDrawing(TypoPage embedPage, TypoDrawing drawing)
    // visual textflow is read from the embed page before the call.
    // java public static void scaleDrawing(int pDrawing, float scale, boolean needSetZoom, boolean scaleAlignline, TypoSnapshot snapshot)
    pub fn scaleDrawing(
        p_drawing: i32,
        scale: f32,
        need_set_zoom: bool,
        scale_alignline: bool,
        snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot,
    ) {
        let _ = scale_alignline;
        use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
        let zoom = TypoDrawing::getZoom__int_typosnapshot(p_drawing, snapshot);
        let mut drawing = snapshot.getTypoTools().obtainTypoDrawing__int(p_drawing);
        drawing.setWidth((drawing.width() as f32 * scale) as i32);
        drawing.setHeight((drawing.height() as f32 * scale) as i32);
        let layout_pad = drawing.getLayoutPadding();
        let render_pad = drawing.getRenderPadding();
        Self::scale_paddings(layout_pad, scale, snapshot);
        Self::scale_paddings(render_pad, scale, snapshot);
        if need_set_zoom {
            drawing.setZoom(zoom * scale);
        }
        snapshot.getTypoTools().recycle_drawing(&mut drawing);
    }

    fn scale_paddings(pointer: i32, scale: f32, snapshot: &mut crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot) {
        use crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl;
        let left = (TypoRectMemoryImpl::getLeft__int_typosnapshot(pointer, snapshot) as f32 * scale) as i32;
        let top = (TypoRectMemoryImpl::getTop__int_typosnapshot(pointer, snapshot) as f32 * scale) as i32;
        let right = (TypoRectMemoryImpl::getRight__int_typosnapshot(pointer, snapshot) as f32 * scale) as i32;
        let bottom = (TypoRectMemoryImpl::getBottom__int_typosnapshot(pointer, snapshot) as f32 * scale) as i32;
        TypoRectMemoryImpl::setLeft__int_int_typosnapshot(left, pointer, snapshot);
        TypoRectMemoryImpl::setTop__int_int_typosnapshot(top, pointer, snapshot);
        TypoRectMemoryImpl::setRight__int_int_typosnapshot(right, pointer, snapshot);
        TypoRectMemoryImpl::setBottom__int_int_typosnapshot(bottom, pointer, snapshot);
    }

    pub fn reverseRotateEmbedDrawing(drawing: &mut crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing, textflow: i32) {
        // java int reverseTextflow = DrawingCoordTool.getReverseTextflow(textflow); swapWidthAndHeight(drawing);
        let reverse = match textflow {
            0 => 0,
            1 => 3,
            3 => 1,
            _ => 0,
        };
        let width = drawing.width();
        let height = drawing.height();
        drawing.setWidth(height);
        drawing.setHeight(width);
        drawing.rotatePadding(drawing.getLayoutPadding(), reverse);
        drawing.rotatePadding(drawing.getRenderPadding(), reverse);
    }
}
