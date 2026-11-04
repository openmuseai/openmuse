use super::super::control::grid::render::graphics::{Canvas, Paint, Path, Rect};
use crate::cn::wps::moss::filefmt::biff8::record::common::XFGradient::XFGradient;
use serde_json::json;
pub struct CellGradientUtil;
impl CellGradientUtil {
    // java public static void drawGradient(Canvas canvas, Paint paint, Rect rect, XFGradient xfGradient) {
    // java paint.setStyle(Style.FILL);
    // java if (xfGradient.isLinear) {
    // java int[] colors = new int[xfGradient.colorStops.size()];
    // java float[] stops = new float[xfGradient.colorStops.size()];
    // java
    // java for(int i = 0; i < xfGradient.colorStops.size(); ++i) {
    // java colors[i] = ((XFGradient.ColorStop)xfGradient.colorStops.get(i)).color;
    // java stops[i] = ((XFGradient.ColorStop)xfGradient.colorStops.get(i)).position;
    // java }
    // java
    // java LinearGradient gradient = null;
    // java switch (xfGradient.degree) {
    // java case 0:
    // java gradient = new LinearGradient(0.0F, 0.0F, 1.0F, 0.0F, colors, stops, TileMode.CLAMP);
    // java break;
    // java case 45:
    // java gradient = new LinearGradient(0.0F, 0.0F, 1.0F, 1.0F, colors, stops, TileMode.CLAMP);
    // java break;
    // java case 90:
    // java gradient = new LinearGradient(0.0F, 0.0F, 0.0F, 1.0F, colors, stops, TileMode.CLAMP);
    // java break;
    // java case 135:
    // java gradient = new LinearGradient(1.0F, 0.0F, 0.0F, 1.0F, colors, stops, TileMode.CLAMP);
    // java break;
    // java case 180:
    // java gradient = new LinearGradient(1.0F, 0.0F, 0.0F, 0.0F, colors, stops, TileMode.CLAMP);
    // java break;
    // java case 225:
    // java gradient = new LinearGradient(1.0F, 1.0F, 0.0F, 0.0F, colors, stops, TileMode.CLAMP);
    // java break;
    // java case 270:
    // java gradient = new LinearGradient(0.0F, 1.0F, 0.0F, 0.0F, colors, stops, TileMode.CLAMP);
    // java break;
    // java case 315:
    // java gradient = new LinearGradient(0.0F, 1.0F, 1.0F, 0.0F, colors, stops, TileMode.CLAMP);
    // java }
    // java
    // java paint.setShader(gradient);
    // java canvas.save();
    // java canvas.translate((float)rect.left, (float)rect.top);
    // java canvas.scale((float)rect.width(), (float)rect.height());
    // java canvas.drawRect(RECT, paint);
    // java canvas.restore();
    // java } else {
    // java XFGradient.ColorStop start = null;
    // java XFGradient.ColorStop end = null;
    // java
    // java for(int i = 0; i < xfGradient.colorStops.size(); ++i) {
    // java XFGradient.ColorStop colorStop = (XFGradient.ColorStop)xfGradient.colorStops.get(i);
    // java if (Float.compare(colorStop.position, 0.0F) == 0) {
    // java start = colorStop;
    // java } else if (Float.compare(colorStop.position, 1.0F) == 0) {
    // java end = colorStop;
    // java }
    // java }
    // java
    // java if (start != null && end != null) {
    // java int[] colors = new int[]{end.color, start.color, start.color, end.color};
    // java float[] stops = new float[]{0.0F, xfGradient.top, xfGradient.bottom, 1.0F};
    // java LinearGradient gradient = new LinearGradient(0.0F, 0.0F, 0.0F, 1.0F, colors, stops, TileMode.CLAMP);
    // java paint.setShader(gradient);
    // java canvas.save();
    // java canvas.translate((float)rect.left, (float)rect.top);
    // java canvas.scale((float)rect.width(), (float)rect.height());
    // java canvas.drawRect(RECT, paint);
    // java Path left = new Path();
    // java left.moveTo(xfGradient.left, xfGradient.top);
    // java left.lineTo(0.0F, 0.0F);
    // java left.lineTo(0.0F, 1.0F);
    // java left.lineTo(xfGradient.left, xfGradient.bottom);
    // java left.close();
    // java Path right = new Path();
    // java right.moveTo(xfGradient.right, xfGradient.bottom);
    // java right.lineTo(1.0F, 1.0F);
    // java right.lineTo(1.0F, 0.0F);
    // java right.lineTo(xfGradient.right, xfGradient.top);
    // java right.close();
    // java left.addPath(right);
    // java canvas.save();
    // java canvas.clipPath(left);
    // java stops = new float[]{0.0F, xfGradient.left, xfGradient.right, 1.0F};
    // java gradient = new LinearGradient(0.0F, 0.0F, 1.0F, 0.0F, colors, stops, TileMode.CLAMP);
    // java paint.setShader(gradient);
    // java canvas.drawRect(RECT, paint);
    // java canvas.restore();
    // java canvas.restore();
    // java }
    // java }
    // java
    // java }
    pub fn drawGradient(c: &mut Canvas, p: &mut Paint, r: &Rect, g: &XFGradient) {
        p.style = 0;
        if g.isLinear {
            let colors = g
                .colorStops
                .iter()
                .map(|s| s.borrow().color)
                .collect::<Vec<_>>();
            let stops = g
                .colorStops
                .iter()
                .map(|s| s.borrow().position)
                .collect::<Vec<_>>();
            let points = match g.degree {
                0 => Some([0.0, 0.0, 1.0, 0.0]),
                45 => Some([0.0, 0.0, 1.0, 1.0]),
                90 => Some([0.0, 0.0, 0.0, 1.0]),
                135 => Some([1.0, 0.0, 0.0, 1.0]),
                180 => Some([1.0, 0.0, 0.0, 0.0]),
                225 => Some([1.0, 1.0, 0.0, 0.0]),
                270 => Some([0.0, 1.0, 0.0, 0.0]),
                315 => Some([0.0, 1.0, 1.0, 0.0]),
                _ => None,
            };
            p.shader=points.map(|q|json!({"kind":"linear","points":q,"colors":colors,"positions":stops,"tileMode":"clamp"}));
            c.save();
            c.translate(r.left as f32, r.top as f32);
            c.scale(r.width() as f32, r.height() as f32, 0.0, 0.0);
            c.drawRect__float(0.0, 0.0, 1.0, 1.0, p);
            c.restore();
        } else {
            let (mut start, mut end) = (None, None);
            for stop in &g.colorStops {
                let stop = stop.borrow();
                if stop.position.to_bits() == 0.0f32.to_bits() {
                    start = Some(stop.color);
                } else if stop.position == 1.0 {
                    end = Some(stop.color);
                }
            }
            if let (Some(start), Some(end)) = (start, end) {
                let colors = [end, start, start, end];
                p.shader = Some(
                    json!({"kind":"linear","points":[0.0,0.0,0.0,1.0],"colors":colors,"positions":[0.0,g.top,g.bottom,1.0],"tileMode":"clamp"}),
                );
                c.save();
                c.translate(r.left as f32, r.top as f32);
                c.scale(r.width() as f32, r.height() as f32, 0.0, 0.0);
                c.drawRect__float(0.0, 0.0, 1.0, 1.0, p);
                let mut path = Path::default();
                path.moveTo(g.left, g.top);
                path.lineTo(0.0, 0.0);
                path.lineTo(0.0, 1.0);
                path.lineTo(g.left, g.bottom);
                path.close();
                path.moveTo(g.right, g.bottom);
                path.lineTo(1.0, 1.0);
                path.lineTo(1.0, 0.0);
                path.lineTo(g.right, g.top);
                path.close();
                c.save();
                c.clipPath(&path);
                p.shader = Some(
                    json!({"kind":"linear","points":[0.0,0.0,1.0,0.0],"colors":colors,"positions":[0.0,g.left,g.right,1.0],"tileMode":"clamp"}),
                );
                c.drawRect__float(0.0, 0.0, 1.0, 1.0, p);
                c.restore();
                c.restore();
            }
        }
    }
}
