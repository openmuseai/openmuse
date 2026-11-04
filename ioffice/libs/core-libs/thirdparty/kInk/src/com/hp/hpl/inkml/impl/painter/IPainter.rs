use crate::com::hp::hpl::inkml::r#impl::Brush::Brush;

// Java: public class IPainter
pub trait IPainter {
    // Java: IPainter()
    // Java: super();

    // Java: void begin(Path path, Brush brush, float x, float y)
    fn begin_brush(&mut self, brush: &Brush, x: f32, y: f32) {
        // Java: this.begin(path, brush, x, y, 1.0F);
        self.begin_brush_f(brush, x, y, 1.0);
    }

    // Java: void begin(Path path, Brush brush, float x, float y, float f)
    fn begin_brush_f(&mut self, brush: &Brush, x: f32, y: f32, f: f32) {
        // Java: float width = brush.getWidth();
        let width = brush.getWidth();
        // Java: float height = brush.getHeight();
        let height = brush.getHeight();
        // Java: this.begin(path, width, height, x, y, f);
        self.begin(width, height, x, y, f);
    }

    // Java: void move(float x, float y)
    fn move_xy(&mut self, x: f32, y: f32) {
        // Java: this.move(x, y, 1.0F);
        self.move_xyf(x, y, 1.0);
    }

    // Java: void end()
    fn end(&mut self) {
        // Java: this.end(true);
        self.end_completed(true);
    }

    // Java: void end(boolean completed)
    fn end_completed(&mut self, _completed: bool) {
    }

    // Java: abstract void begin(Path var1, float var2, float var3, float var4, float var5, float var6)
    fn begin(&mut self, var2: f32, var3: f32, var4: f32, var5: f32, var6: f32);

    // Java: abstract void move(float var1, float var2, float var3)
    fn move_xyf(&mut self, var1: f32, var2: f32, var3: f32);

    // Java: abstract void draw(Canvas var1, Paint var2)
    fn draw(&self, var1: i64, var2: i64);
}
