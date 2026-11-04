// Java: import android.graphics.Bitmap
use std::any::Any;
use super::RIDNode::RIDNode;
pub struct IHostApp;

// Java: public interface IDiagram
pub trait IDiagram {
    // Java: abstract String getDiagramBitmapPath(int var1, int var2)
    fn getDiagramBitmapPath(&self, var1: i32, var2: i32) -> String;

    // Java: abstract Bitmap getDiagramBitmap(float var1, float var2, float var3)
    fn getDiagramBitmap(&self, var1: f32, var2: f32, var3: f32) -> Option<Box<dyn Any>>;

    // Java: abstract void setDrawingNode(RIDNode var1)
    fn setDrawingNode(&mut self, var1: RIDNode);

    // Java: abstract RIDNode getDrawingNode()
    fn getDrawingNode(&self) -> &RIDNode;

    // Java: abstract void setDataNode(RIDNode var1)
    fn setDataNode(&mut self, var1: RIDNode);

    // Java: abstract RIDNode getDataNode()
    fn getDataNode(&self) -> &RIDNode;

    // Java: abstract String getBitMapPath()
    fn getBitMapPath(&self) -> String;

    // Java: abstract void setColorsPath(String var1)
    fn setColorsPath(&mut self, var1: String);

    // Java: abstract String getColorsPath()
    fn getColorsPath(&self) -> String;

    // Java: abstract void setDataPath(String var1)
    fn setDataPath(&mut self, var1: String);

    // Java: abstract String getDataPath()
    fn getDataPath(&self) -> String;

    // Java: abstract void setLayoutPath(String var1)
    fn setLayoutPath(&mut self, var1: String);

    // Java: abstract String getLayoutPath()
    fn getLayoutPath(&self) -> String;

    // Java: abstract void setQuickStylePath(String var1)
    fn setQuickStylePath(&mut self, var1: String);

    // Java: abstract String getQuickStylePath()
    fn getQuickStylePath(&self) -> String;

    // Java: abstract void setDrawingPath(String var1)
    fn setDrawingPath(&mut self, var1: String);

    // Java: abstract String getDrawingPath()
    fn getDrawingPath(&self) -> String;

    // Java: abstract Object openDrawing(int var1)
    fn openDrawing(&mut self, var1: i32) -> Option<Box<dyn Any>>;

    // Java: abstract void openData(Object var1)
    fn openData(&mut self, var1: Box<dyn Any>);

    // Java: abstract void setShapeIdx(int var1)
    fn setShapeIdx(&mut self, var1: i32);

    // Java: abstract void setHostApp(IHostApp var1)
    fn setHostApp(&mut self, var1: IHostApp);

    // Java: abstract IHostApp getHostApp()
    fn getHostApp(&self) -> &IHostApp;
}
