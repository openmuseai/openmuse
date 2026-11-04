// Java: import cn.wps.graphics.RectF
// Java: import cn.wps.moffice.print.IProgress
// Java: import java.io.IOException

// Java: public interface IPdfExporter
pub trait IPdfExporter {
    // Java: abstract Object startPage(float var1, float var2, RectF var3)
    fn startPage(&mut self, _var1: f32, _var2: f32, _var3: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract void finishPage()
    fn finishPage(&mut self);

    // Java: abstract boolean writeTo(String var1)
    fn writeTo(&self, _var1: &str) -> bool;

    // Java: abstract boolean writeTo(String var1, IProgress var2, int var3)
    fn writeTo_with_progress(&self, _var1: &str, _var2: &dyn std::any::Any, _var3: i32) -> bool;

    // Java: abstract void close()
    fn close(&mut self);

    // Java: abstract Object newPDFPath()
    fn newPDFPath(&self) -> Box<dyn std::any::Any>;

    // Java: abstract void setFont(Object var1)
    fn setFont(&mut self, _var1: &dyn std::any::Any);

    // Java: abstract Object getDashPathEffect(float[] var1, float var2)
    fn getDashPathEffect(&self, _var1: &[f32], _var2: f32) -> Box<dyn std::any::Any>;

    // Java: abstract Object getBitmapShader(Object var1, Object var2, Object var3)
    fn getBitmapShader(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any, _var3: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract Object getDrawingRender()
    fn getDrawingRender(&self) -> Box<dyn std::any::Any>;

}
