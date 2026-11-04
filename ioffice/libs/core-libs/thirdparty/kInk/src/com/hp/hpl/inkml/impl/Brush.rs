use super::EraseMode::EraseMode;

// Java field: float HIGH_LIHGT_TRANSPARENCY
pub(crate) const HIGH_LIHGT_TRANSPARENCY: f32 = 0.666667;
// Java field: int DEFAULT_F_MAX
pub(crate) const DEFAULT_F_MAX: i32 = 1023;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -6134459251745274014;
// Java field: Shape DEFAULT_SHAPE
static DEFAULT_SHAPE: std::sync::OnceLock<Shape> = std::sync::OnceLock::new();
// Java field: int DEFAULT_COLOR
const DEFAULT_COLOR: i32 = -16777216;
// Java field: float DEFAULT_WIDTH
const DEFAULT_WIDTH: f32 = 3.0;
// Java field: int DEFAULT_TRANSPARENCY
const DEFAULT_TRANSPARENCY: i32 = 255;

// Java: public enum Shape
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Shape {
    ellipse,
    rectangle,
    drop,
}

// Java: public enum RasterOp
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum RasterOp {
    noOperation,
    copyPen,
    maskPen,
}

// Java: public class Brush implements Cloneable, Serializable
pub struct Brush {
    // Java field: int _color
    pub(crate) _color: i32,
    // Java field: float _width
    pub(crate) _width: f32,
    // Java field: float _height
    pub(crate) _height: f32,
    // Java field: Shape _tipShape
    pub(crate) _tipShape: Shape,
    // Java field: RasterOp _rasterOp
    pub(crate) _rasterOp: RasterOp,
    // Java field: boolean _antiAliased
    pub(crate) _antiAliased: bool,
    // Java field: boolean _fitToCurve
    pub(crate) _fitToCurve: bool,
    // Java field: boolean _ignorePressure
    _ignorePressure: bool,
    // Java field: int transparency
    pub(crate) transparency: i32,
    // Java field: boolean _eraser
    pub(crate) _eraser: bool,
    // Java field: EraseMode _eraseMode
    pub(crate) _eraseMode: EraseMode,
    // Java field: LinkedList<BrushListener> _listeners
    _listeners: Option<Vec<i32>>,
    // Java field: float _maxFChannel
    pub(crate) _maxFChannel: f32,
}

// Java: public class Brush implements Cloneable, Serializable
impl Brush {
    // Java: static boolean isValidStrokeScale(float scale)
    pub fn isValidStrokeScale(scale: f32) -> bool {
        // Java: return scale > 0.0F;
        scale > 0.0
    }

    // Java: Brush()
    pub fn Brush() -> Self {
        // Java: super();
        // Java: this.setShape(DEFAULT_SHAPE);
        // Java: this.setColor(-16777216);
        // Java: this.setWidth(3.0F);
        // Java: this.setHeight(3.0F);
        // Java: this.setEraser(false);
        // Java: this.setAntiAliased(true);
        // Java: this.setRasterOp(Brush.RasterOp.copyPen);
        // Java: this.setTransparency(255);
        // Java: this.setMaxFChannel(false);
        // Java: this._listeners = null;
        Self {
            _color: DEFAULT_COLOR,
            _width: DEFAULT_WIDTH,
            _height: DEFAULT_WIDTH,
            _tipShape: Shape::ellipse,
            _rasterOp: RasterOp::copyPen,
            _antiAliased: true,
            _fitToCurve: false,
            _ignorePressure: false,
            transparency: DEFAULT_TRANSPARENCY,
            _eraser: false,
            _eraseMode: EraseMode::ERASE_BY_STROKE,
            _listeners: None,
            _maxFChannel: 0.0,
        }
    }

    // Java: Brush(Shape paramShape, float paramFloat, int paramInt, int paramColor, boolean paramBoolean, EraseMode paramEraseMode)
    pub fn Brush_with_params(paramShape: Shape, paramFloat: f32, paramInt: i32, paramColor: i32, paramBoolean: bool, paramEraseMode: EraseMode) -> Self {
        // Java: super();
        // Java: this.setShape(paramShape);
        // Java: this.setColor(paramColor);
        // Java: this.setWidth(paramFloat);
        // Java: this.setEraser(paramBoolean);
        // Java: this.setAntiAliased(true);
        // Java: this.setRasterOp(Brush.RasterOp.copyPen);
        // Java: this.setTransparency(paramInt);
        // Java: this.setEraserMode(paramEraseMode);
        // Java: this._listeners = null;
        Self {
            _color: paramColor,
            _width: paramFloat,
            _height: paramFloat,
            _tipShape: paramShape,
            _rasterOp: RasterOp::copyPen,
            _antiAliased: true,
            _fitToCurve: false,
            _ignorePressure: false,
            transparency: paramInt,
            _eraser: paramBoolean,
            _eraseMode: paramEraseMode,
            _listeners: None,
            _maxFChannel: 0.0,
        }
    }

    // Java: void setRasterOp(RasterOp paramRasterOp)
    pub fn setRasterOp(&mut self, paramRasterOp: RasterOp) {
        // Java: this._rasterOp = paramRasterOp;
        self._rasterOp = paramRasterOp;
    }

    // Java: RasterOp getRasterOp()
    pub fn getRasterOp(&self) -> RasterOp {
        // Java: return this._rasterOp;
        self._rasterOp
    }

    // Java: int getColor()
    pub fn getColor(&self) -> i32 {
        // Java: return this._color;
        self._color
    }

    // Java: void addBrushListener(BrushListener paramBrushListener)
    pub fn addBrushListener(&mut self) {
        // Java: if (this._listeners == null)
        // Java: this._listeners = new LinkedList();
        // Java: this._listeners.add(paramBrushListener);
    }

    // Java: float getWidth()
    pub fn getWidth(&self) -> f32 {
        // Java: return this._width;
        self._width
    }

    // Java: float getHeight()
    pub fn getHeight(&self) -> f32 {
        // Java: return this._height;
        self._height
    }

    // Java: Shape getShape()
    pub fn getShape(&self) -> Shape {
        // Java: return this._tipShape;
        self._tipShape
    }

    // Java: boolean isEraser()
    pub fn isEraser(&self) -> bool {
        // Java: return this._eraser;
        self._eraser
    }

    // Java: void setEraser(boolean paramBoolean)
    pub fn setEraser(&mut self, paramBoolean: bool) {
        // Java: this._eraser = paramBoolean;
        self._eraser = paramBoolean;
    }

    // Java: void setShape(String paramString)
    pub fn setShape_str(&mut self, paramString: &str) {
        // Java: Shape localShape = Brush.Shape.valueOf(paramString);
        let localShape = match paramString {
            "ellipse" => Shape::ellipse,
            "rectangle" => Shape::rectangle,
            "drop" => Shape::drop,
            _ => Shape::ellipse,
        };
        // Java: this._tipShape = localShape;
        self._tipShape = localShape;
        // Java: if (this._listeners != null)
        // Java: for(BrushListener localBrushListener : this._listeners)
        // Java: localBrushListener.brushShapeChanged(this);
    }

    // Java: void setShape(Shape paramShape)
    pub fn setShape(&mut self, paramShape: Shape) {
        // Java: this._tipShape = paramShape;
        self._tipShape = paramShape;
        // Java: if (this._listeners != null)
        // Java: for(BrushListener localBrushListener : this._listeners)
        // Java: localBrushListener.brushShapeChanged(this);
    }

    // Java: Brush clone()
    pub fn clone(&self) -> Brush {
        // Java: Brush localBrush = new Brush();
        let mut localBrush = Brush::Brush();
        // Java: localBrush._color = this._color;
        localBrush._color = self._color;
        // Java: localBrush._width = this._width;
        localBrush._width = self._width;
        // Java: localBrush._height = this._height;
        localBrush._height = self._height;
        // Java: localBrush._tipShape = this._tipShape;
        localBrush._tipShape = self._tipShape;
        // Java: localBrush._rasterOp = this._rasterOp;
        localBrush._rasterOp = self._rasterOp;
        // Java: localBrush._antiAliased = this._antiAliased;
        localBrush._antiAliased = self._antiAliased;
        // Java: localBrush._fitToCurve = this._fitToCurve;
        localBrush._fitToCurve = self._fitToCurve;
        // Java: localBrush._ignorePressure = this._ignorePressure;
        localBrush._ignorePressure = self._ignorePressure;
        // Java: localBrush._eraser = this._eraser;
        localBrush._eraser = self._eraser;
        // Java: localBrush._eraseMode = this._eraseMode;
        localBrush._eraseMode = self._eraseMode;
        // Java: localBrush.transparency = this.transparency;
        localBrush.transparency = self.transparency;
        // Java: return localBrush;
        localBrush
    }

    // Java: void setColor(int paramColor)
    pub fn setColor(&mut self, paramColor: i32) {
        // Java: this._color = paramColor;
        self._color = paramColor;
    }

    // Java: EraseMode getEraseMode()
    pub fn getEraseMode(&self) -> EraseMode {
        // Java: return this._eraseMode;
        self._eraseMode
    }

    // Java: void setEraserMode(EraseMode paramEraseMode)
    pub fn setEraserMode(&mut self, paramEraseMode: EraseMode) {
        // Java: this._eraseMode = paramEraseMode;
        self._eraseMode = paramEraseMode;
    }

    // Java: void setWidth(float paramFloat)
    pub fn setWidth(&mut self, paramFloat: f32) {
        // Java: this._width = paramFloat;
        self._width = paramFloat;
    }

    // Java: void setHeight(float height)
    pub fn setHeight(&mut self, height: f32) {
        // Java: this._height = height;
        self._height = height;
    }

    // Java: static Brush fromInkML(IBrush paramBrush)
    pub fn fromInkML(paramBrush: &dyn crate::com::hp::hpl::inkml::IBrush::IBrush) -> Brush {
        // Java: Brush localBrush = new Brush();
        let mut localBrush = Brush::Brush();
        // Java: try
        {
            // Java: String transparencyStr = paramBrush.getBrushProperty("transparency");
            let transparencyStr = paramBrush.getBrushProperty("transparency");
            // Java: if (transparencyStr != null)
            if !transparencyStr.is_empty() {
                // Java: localBrush.setTransparency(255 - Integer.parseInt(transparencyStr));
                if let Ok(v) = transparencyStr.parse::<i32>() {
                    localBrush.setTransparency(255 - v);
                }
            }
            // Java: String color = paramBrush.getBrushProperty("color");
            let color = paramBrush.getBrushProperty("color");
            // Java: localBrush.setColor(toRGBA(color, localBrush.getTransparency()));
            localBrush.setColor(Brush::toRGBA(&color, localBrush.getTransparency()));
            // Java: String shape = paramBrush.getBrushProperty("tip");
            let shape = paramBrush.getBrushProperty("tip");
            // Java: if (shape != null)
            if !shape.is_empty() {
                // Java: localBrush.setShape(Brush.Shape.valueOf(shape));
                localBrush.setShape_str(&shape);
            }
            // Java: String width = paramBrush.getBrushProperty("width");
            let mut width = paramBrush.getBrushProperty("width");
            // Java: String height = paramBrush.getBrushProperty("height");
            let mut height = paramBrush.getBrushProperty("height");
            // Java: if (width == null)
            if width.is_empty() {
                // Java: width = height;
                width = height.clone();
            }
            // Java: if (height == null)
            if height.is_empty() {
                // Java: height = width;
                height = width.clone();
            }
            // Java: if (width != null)
            if !width.is_empty() {
                // Java: localBrush.setWidth(Float.valueOf(width));
                if let Ok(v) = width.parse::<f32>() {
                    localBrush.setWidth(v);
                }
            }
            // Java: if (height != null)
            if !height.is_empty() {
                // Java: localBrush.setHeight(Float.valueOf(height));
                if let Ok(v) = height.parse::<f32>() {
                    localBrush.setHeight(v);
                }
            }
            // Java: String rasterOp = paramBrush.getBrushProperty("rasterOp");
            let rasterOp = paramBrush.getBrushProperty("rasterOp");
            // Java: if (rasterOp != null)
            if !rasterOp.is_empty() {
                // Java: localBrush.setRasterOp(Brush.RasterOp.valueOf(rasterOp));
                let op = match rasterOp.as_str() {
                    "copyPen" => RasterOp::copyPen,
                    "maskPen" => RasterOp::maskPen,
                    _ => RasterOp::noOperation,
                };
                localBrush.setRasterOp(op);
            }
            // Java: String fitToCurve = paramBrush.getBrushProperty("fitToCurve");
            let fitToCurve = paramBrush.getBrushProperty("fitToCurve");
            // Java: if (fitToCurve != null)
            if !fitToCurve.is_empty() {
                // Java: localBrush.setFitToCurve();
                localBrush.setFitToCurve();
            }
        }
        // Java: catch (NumberFormatException var9)
        // Java: catch (InkMLException var10)
        // Java: catch (Exception var11)
        // Java: return localBrush;
        localBrush
    }

    // Java: static int toRGBA(String paramString, int paramInt)
    pub fn toRGBA(paramString: &str, paramInt: i32) -> i32 {
        // Java: int i = 0;
        let mut i: i32 = 0;
        // Java: if (paramString != null)
        if !paramString.is_empty() {
            // Java: i = Integer.decode(paramString);
            if let Some(hex) = paramString.strip_prefix("#") {
                i = i32::from_str_radix(hex, 16).unwrap_or(0);
            } else if let Some(hex) = paramString.strip_prefix("0x") {
                i = i32::from_str_radix(hex, 16).unwrap_or(0);
            } else {
                i = paramString.parse::<i32>().unwrap_or(0);
            }
        }
        // Java: paramInt = paramInt << 24 & -16777216;
        let alpha = (paramInt << 24) & (-16777216i32);
        // Java: return paramInt | i;
        alpha | i
    }

    // Java: String toInkML()
    pub fn toInkML(&self) -> String {
        // Java: return "<brush><brushProperty name='color' value='#" + toRGB(this.getColor()) + "' />" + "<brushProperty name='shape' value='" + this.getShape() + "' />" + "<brushProperty name='type' value='" + (this._eraser ? "eraser" : "regular") + "' />" + "<brushProperty name='eraseMethod' value='" + this._eraseMode + "' />" + "<brushProperty name='transparency' value='" + (255 - this.getTransparency()) + "' />" + "</brush>";
        format!(
            "<brush><brushProperty name='color' value='#{}' /><brushProperty name='shape' value='{:?}' /><brushProperty name='type' value='{}' /><brushProperty name='eraseMethod' value='{:?}' /><brushProperty name='transparency' value='{}' /></brush>",
            Brush::toRGB(self.getColor()),
            self.getShape(),
            if self._eraser { "eraser" } else { "regular" },
            self._eraseMode,
            255 - self.getTransparency()
        )
    }

    // Java: static String toRGB(int color)
    pub fn toRGB(color: i32) -> String {
        // Java: int red = Color.red(color);
        let red = (color >> 16) & 0xFF;
        // Java: int green = Color.green(color);
        let green = (color >> 8) & 0xFF;
        // Java: int blue = Color.blue(color);
        let blue = color & 0xFF;
        // Java: return String.format("%02x%02x%02x", red, green, blue).toUpperCase();
        format!("{:02X}{:02X}{:02X}", red, green, blue)
    }

    // Java: void setAntiAliased(boolean paramBoolean)
    pub fn setAntiAliased(&mut self, paramBoolean: bool) {
        // Java: this._antiAliased = paramBoolean;
        self._antiAliased = paramBoolean;
    }

    // Java: boolean isAntiAliased()
    pub fn isAntiAliased(&self) -> bool {
        // Java: return this._antiAliased;
        self._antiAliased
    }

    // Java: void setTransparency(int paramInt)
    pub fn setTransparency(&mut self, paramInt: i32) {
        // Java: this.transparency = paramInt;
        self.transparency = paramInt;
    }

    // Java: int getTransparency()
    pub fn getTransparency(&self) -> i32 {
        // Java: return this.transparency;
        self.transparency
    }

    // Java: void setFitToCurve()
    pub fn setFitToCurve(&mut self) {
        // Java: this._fitToCurve = true;
        self._fitToCurve = true;
    }

    // Java: boolean isFitToCurve()
    pub fn isFitToCurve(&self) -> bool {
        // Java: return this._fitToCurve;
        self._fitToCurve
    }

    // Java: void setMaxFChannel(boolean hasFChannel)
    pub fn setMaxFChannel_bool(&mut self, hasFChannel: bool) {
        // Java: this._maxFChannel = hasFChannel ? 1023.0F : 0.0F;
        self._maxFChannel = if hasFChannel { 1023.0 } else { 0.0 };
    }

    // Java: void setMaxFChannel(float maxFChannel)
    pub fn setMaxFChannel(&mut self, maxFChannel: f32) {
        // Java: this._maxFChannel = maxFChannel;
        self._maxFChannel = maxFChannel;
    }

    // Java: float getMaxFChannel()
    pub fn getMaxFChannel(&self) -> f32 {
        // Java: return this._maxFChannel;
        self._maxFChannel
    }

    // Java: boolean hasMaxFChannel()
    pub fn hasMaxFChannel(&self) -> bool {
        // Java: return this._maxFChannel != 0.0F;
        self._maxFChannel != 0.0
    }
}

// Java: public enum Shape
impl Shape {
    // Java: Shape()
}

// Java: public enum RasterOp
impl RasterOp {
    // Java: RasterOp()
}
