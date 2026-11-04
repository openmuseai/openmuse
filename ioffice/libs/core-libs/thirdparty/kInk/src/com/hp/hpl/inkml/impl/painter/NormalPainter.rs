use super::IPainter::IPainter;

// Java: public class NormalPainter extends IPainter
pub struct NormalPainter {
    // Java field: Path mPath
    mPath: Vec<(f32, f32)>,
}

// Java: public class NormalPainter extends IPainter
impl NormalPainter {
    // Java: NormalPainter()
    pub fn NormalPainter() -> Self {
        // Java: super();
        Self { mPath: Vec::new() }
    }
}

impl IPainter for NormalPainter {
    // Java: void begin(Path path, float width, float height, float x, float y, float f)
    fn begin(&mut self, _width: f32, _height: f32, x: f32, y: f32, _f: f32) {
        // Java: this.mPath = path;
        self.mPath.clear();
        // Java: this.mPath.moveTo(x, y);
        self.mPath.push((x, y));
    }

    // Java: void move(float x, float y, float f)
    fn move_xyf(&mut self, x: f32, y: f32, _f: f32) {
        // Java: this.mPath.lineTo(x, y);
        self.mPath.push((x, y));
    }

    // Java: void end()
    fn end(&mut self) {
    }

    // Java: void draw(Canvas canvas, Paint paint)
    fn draw(&self, _canvas: i64, _paint: i64) {
        // Java: canvas.drawPath(this.mPath, paint);
    }
}
