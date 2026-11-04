// Java: import android.graphics.Bitmap
use crate::cn::wps::text::layout::data::FontFill::Bitmap;
// Java: import android.graphics.BitmapFactory
// Java: import android.graphics.BitmapShader
// Java: import android.graphics.Matrix
// Java: import android.graphics.Bitmap.Config
// Java: import android.graphics.Shader.TileMode
// Java: import android.text.TextPaint
// Java: import cn.wps.moffice.drawing.graphics.PatternFillUtil
use drawing::cn::wps::moffice::drawing::graphics::PatternFillUtil::PatternFillUtil;
// Java: import cn.wps.text.layout.data.PatternAttr
use crate::cn::wps::text::layout::data::PatternAttr::PatternAttr;

pub struct BitmapShader;
impl BitmapShader {
    pub fn new(_bmp: &Bitmap, _t1: TileMode, _t2: TileMode) -> Self { BitmapShader }
    pub fn setLocalMatrix(&mut self, _m: &Matrix) {}
}

#[derive(Clone, Copy)]
pub enum TileMode { REPEAT }

pub struct Matrix {
    values: Vec<f32>,
}
impl Matrix {
    pub fn new() -> Self { Matrix { values: vec![0.0; 9] } }
    pub fn from(other: &Matrix) -> Self { Matrix { values: other.values.clone() } }
    pub fn setScale(&mut self, _sx: f32, _sy: f32) {}
    pub fn preRotate(&mut self, _degrees: f32) {}
    pub fn getValues(&self, out: &mut [f32]) {
        for (i, v) in self.values.iter().enumerate() {
            if i < out.len() { out[i] = *v; }
        }
    }
}

pub struct TextPaint {
    pub color: i32,
    pub shader: Option<BitmapShader>,
}
impl TextPaint {
    pub fn setColor(&mut self, c: i32) { self.color = c; }
    pub fn setShader(&mut self, s: BitmapShader) { self.shader = Some(s); }
}

pub struct BitmapFactory;
impl BitmapFactory {
    pub fn decodeByteArray(data: &[u8], _offset: usize, _length: usize) -> Option<Bitmap> {
        if data.is_empty() { None } else { Some(Bitmap) }
    }
}

#[derive(Clone, Copy)]
pub enum Config { ARGB_4444 }

// Java: public class PatternPaintSetter
pub struct PatternPaintSetter {
    // Java field: PatternAttr mPatternAttr
    mPatternAttr: Option<PatternAttr>,
}

// Java: public class CanvasUtil
pub struct CanvasUtil {
    // Java field: float[] values
    pub(crate) values: Vec<f32>,
}

// Java: public class PatternPaintSetter
impl PatternPaintSetter {
    // Java: PatternPaintSetter(PatternAttr patternAttr)
    pub fn new(patternAttr: PatternAttr) -> Self {
      // Java: super();
      // Java: this.mPatternAttr = patternAttr;
      PatternPaintSetter { mPatternAttr: Some(patternAttr) }
    }

    // Java: void applyPatternFill(TextPaint paint, Matrix canvasMatrix)
    pub fn applyPatternFill(&self, paint: &mut TextPaint, canvasMatrix: Option<&Matrix>) {
      // Java: boolean isDoc = false;
      let mut isDoc = false;
      // Java: Bitmap bmp;
      let bmp: Option<Bitmap>;
      // Java: if (this.mPatternAttr.getPatternBmp() != null)
      if self.mPatternAttr.as_ref().unwrap().getPatternBmp().is_some()
      {
         // Java: bmp = this.mPatternAttr.getPatternBmp();
         bmp = self.mPatternAttr.as_ref().unwrap().getPatternBmp().cloned();
         // Java: isDoc = true;
         isDoc = true;
      }
      // Java: else
      else
      {
         // Java: int pattType = this.mPatternAttr.getPattType();
         let pattType = self.mPatternAttr.as_ref().unwrap().getPattType();
         // Java: String prst = this.getPatternPrst(pattType);
         let prst = self.getPatternPrst(pattType);
         // Java: byte[] data = PatternFillUtil.prstStrMatcheToBitByte(prst);
         let data = PatternFillUtil::prstStrMatcheToBitByte(&prst);
         // Java: bmp = BitmapFactory.decodeByteArray(data, 0, data.length);
         bmp = BitmapFactory::decodeByteArray(&data, 0, data.len());
      }
      // Java: if (bmp == null)
      if bmp.is_none()
      {
         // Java: paint.setColor(-16777216);
         paint.setColor(-16777216);
      }
      // Java: else
      else
      {
         // Java: int foreColor = this.mPatternAttr.getForeColor();
         let foreColor = self.mPatternAttr.as_ref().unwrap().getForeColor();
         // Java: int bgColor = this.mPatternAttr.getBgColor();
         let bgColor = self.mPatternAttr.as_ref().unwrap().getBgColor();
         // Java: bmp = this.createColorBitmap(bmp, foreColor, bgColor);
         let bmp = self.createColorBitmap(bmp.unwrap(), foreColor, bgColor);
         // Java: BitmapShader bmpShader = new BitmapShader(bmp, TileMode.REPEAT, TileMode.REPEAT);
         let mut bmpShader = BitmapShader::new(&bmp, TileMode::REPEAT, TileMode::REPEAT);
         // Java: if (canvasMatrix != null)
         if canvasMatrix.is_some()
         {
            // Java: float[] values = new float[2];
            let mut values = vec![0.0f32; 2];
            // Java: (new CanvasUtil()).calScaleXy(canvasMatrix, values);
            CanvasUtil::new().calScaleXy(canvasMatrix.unwrap(), &mut values);
            // Java: Matrix matrix = new Matrix();
            let mut matrix = Matrix::new();
            // Java: if (isDoc)
            if isDoc
            {
               // Java: matrix.setScale(1.0F / values[0], 1.0F / values[1]);
               matrix.setScale(1.0f32 / values[0], 1.0f32 / values[1]);
            }
            // Java: else
            else
            {
               // Java: matrix.setScale(1.0F / values[0] * 2.0F, 1.0F / values[1] * 2.0F);
               matrix.setScale(1.0f32 / values[0] * 2.0f32, 1.0f32 / values[1] * 2.0f32);
            }
            // Java: bmpShader.setLocalMatrix(matrix);
            bmpShader.setLocalMatrix(&matrix);
         }
         // Java: paint.setShader(bmpShader);
         paint.setShader(bmpShader);
      }
    }

    // Java: Bitmap createColorBitmap(Bitmap bitmap, int patternColor, int backColor)
    pub fn createColorBitmap(&self, bitmap: Bitmap, patternColor: i32, backColor: i32) -> Bitmap {
      // Java: int width = bitmap.getWidth();
      let width = 8;
      // Java: int height = bitmap.getHeight();
      let height = 8;
      // Java: int[] colors = new int[width * height];
      let mut colors: Vec<i32> = vec![0; width * height];
      // Java: bitmap.getPixels(colors, 0, width, 0, 0, width, height);
      // Java: int size = width * height;
      let size = width * height;
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size
      {
         // Java: if ((colors[i] & 16777215) == 0)
         if (colors[i] & 16777215) == 0
         {
            // Java: colors[i] = backColor;
            colors[i] = backColor;
         }
         // Java: else
         else
         {
            // Java: colors[i] = patternColor;
            colors[i] = patternColor;
         }
      }
      // Java: return Bitmap.createBitmap(colors, width, height, Config.ARGB_4444);
      Bitmap
    }

    // Java: String getPatternPrst(int type)
    pub fn getPatternPrst(&self, r#type: i32) -> String {
      // Java: switch (type)
      match r#type
      {
         // Java: case 0:
         0 =>
            // Java: return "pct5";
            { return "pct5".to_string(); }
         // Java: case 1:
         1 =>
            // Java: return "pct10";
            { return "pct10".to_string(); }
         // Java: case 2:
         2 =>
            // Java: return "pct20";
            { return "pct20".to_string(); }
         // Java: case 3:
         3 =>
            // Java: return "pct25";
            { return "pct25".to_string(); }
         // Java: case 4:
         4 =>
            // Java: return "pct30";
            { return "pct30".to_string(); }
         // Java: case 5:
         5 =>
            // Java: return "pct40";
            { return "pct40".to_string(); }
         // Java: case 6:
         6 =>
            // Java: return "pct50";
            { return "pct50".to_string(); }
         // Java: case 7:
         7 =>
            // Java: return "pct60";
            { return "pct60".to_string(); }
         // Java: case 8:
         8 =>
            // Java: return "pct70";
            { return "pct70".to_string(); }
         // Java: case 9:
         9 =>
            // Java: return "pct75";
            { return "pct75".to_string(); }
         // Java: case 10:
         10 =>
            // Java: return "pct80";
            { return "pct80".to_string(); }
         // Java: case 11:
         11 =>
            // Java: return "pct90";
            { return "pct90".to_string(); }
         // Java: case 12:
         // Java: case 13:
         // Java: case 22:
         // Java: case 23:
         // Java: case 24:
         // Java: case 33:
         // Java: default:
         12 | 13 | 22 | 23 | 24 | 33 | _ =>
            // Java: return "pct5";
            { return "pct5".to_string(); }
         // Java: case 14:
         14 =>
            // Java: return "ltHorz";
            { return "ltHorz".to_string(); }
         // Java: case 15:
         15 =>
            // Java: return "ltVert";
            { return "ltVert".to_string(); }
         // Java: case 16:
         16 =>
            // Java: return "dkHorz";
            { return "dkHorz".to_string(); }
         // Java: case 17:
         17 =>
            // Java: return "dkVert";
            { return "dkVert".to_string(); }
         // Java: case 18:
         18 =>
            // Java: return "narHorz";
            { return "narHorz".to_string(); }
         // Java: case 19:
         19 =>
            // Java: return "narVert";
            { return "narVert".to_string(); }
         // Java: case 20:
         20 =>
            // Java: return "dashHorz";
            { return "dashHorz".to_string(); }
         // Java: case 21:
         21 =>
            // Java: return "dashVert";
            { return "dashVert".to_string(); }
         // Java: case 25:
         25 =>
            // Java: return "ltDnDiag";
            { return "ltDnDiag".to_string(); }
         // Java: case 26:
         26 =>
            // Java: return "ltUpDiag";
            { return "ltUpDiag".to_string(); }
         // Java: case 27:
         27 =>
            // Java: return "dkDnDiag";
            { return "dkDnDiag".to_string(); }
         // Java: case 28:
         28 =>
            // Java: return "dkUpDiag";
            { return "dkUpDiag".to_string(); }
         // Java: case 29:
         29 =>
            // Java: return "wdDnDiag";
            { return "wdDnDiag".to_string(); }
         // Java: case 30:
         30 =>
            // Java: return "wdUpDiag";
            { return "wdUpDiag".to_string(); }
         // Java: case 31:
         31 =>
            // Java: return "dashDnDiag";
            { return "dashDnDiag".to_string(); }
         // Java: case 32:
         32 =>
            // Java: return "dashUpDiag";
            { return "dashUpDiag".to_string(); }
         // Java: case 34:
         34 =>
            // Java: return "smCheck";
            { return "smCheck".to_string(); }
         // Java: case 35:
         35 =>
            // Java: return "lgCheck";
            { return "lgCheck".to_string(); }
         // Java: case 36:
         36 =>
            // Java: return "smGrid";
            { return "smGrid".to_string(); }
         // Java: case 37:
         37 =>
            // Java: return "lgGrid";
            { return "lgGrid".to_string(); }
         // Java: case 38:
         38 =>
            // Java: return "dotGrid";
            { return "dotGrid".to_string(); }
         // Java: case 39:
         39 =>
            // Java: return "smConfetti";
            { return "smConfetti".to_string(); }
         // Java: case 40:
         40 =>
            // Java: return "lgConfetti";
            { return "lgConfetti".to_string(); }
         // Java: case 41:
         41 =>
            // Java: return "horzBrick";
            { return "horzBrick".to_string(); }
         // Java: case 42:
         42 =>
            // Java: return "diagBrick";
            { return "diagBrick".to_string(); }
         // Java: case 43:
         43 =>
            // Java: return "solidDmnd";
            { return "solidDmnd".to_string(); }
         // Java: case 44:
         44 =>
            // Java: return "openDmnd";
            { return "openDmnd".to_string(); }
         // Java: case 45:
         45 =>
            // Java: return "dotDmnd";
            { return "dotDmnd".to_string(); }
         // Java: case 46:
         46 =>
            // Java: return "plaid";
            { return "plaid".to_string(); }
         // Java: case 47:
         47 =>
            // Java: return "sphere";
            { return "sphere".to_string(); }
         // Java: case 48:
         48 =>
            // Java: return "weave";
            { return "weave".to_string(); }
         // Java: case 49:
         49 =>
            // Java: return "divot";
            { return "divot".to_string(); }
         // Java: case 50:
         50 =>
            // Java: return "shingle";
            { return "shingle".to_string(); }
         // Java: case 51:
         51 =>
            // Java: return "wave";
            { return "wave".to_string(); }
         // Java: case 52:
         52 =>
            // Java: return "trellis";
            { return "trellis".to_string(); }
         // Java: case 53:
         53 =>
            // Java: return "zigZag";
            { return "zigZag".to_string(); }
      }
    }

}

// Java: public class CanvasUtil
impl CanvasUtil {
    // Java: CanvasUtil()
    pub fn new() -> Self {
         // Java: super();
         // Java: this.values = new float[9];
         CanvasUtil { values: vec![0.0f32; 9] }
    }

    // Java: void calScaleXy(Matrix canvasMatrix, float[] outScales)
    pub fn calScaleXy(&mut self, canvasMatrix: &Matrix, outScales: &mut [f32]) {
         // Java: Matrix matrix = new Matrix(canvasMatrix);
         let mut matrix = Matrix::from(canvasMatrix);
         // Java: matrix.getValues(this.values);
         matrix.getValues(&mut self.values);
         // Java: float xSkew = this.values[1];
         let xSkew = self.values[1];
         // Java: float xScale = this.values[0];
         let mut xScale = self.values[0];
         // Java: float yScale = this.values[4];
         let mut yScale = self.values[4];
         // Java: if (xSkew != 0.0F)
         if xSkew != 0.0f32
         {
            // Java: float preRotationValue = xScale == 0.0F ? 90.0F : (float)(Math.atan((double)(-xSkew / xScale)) * (double)180.0F / Math.PI);
            let preRotationValue: f32 = if xScale == 0.0f32 { 90.0f32 } else { ((-xSkew as f64 / xScale as f64).atan() * 180.0f64 / std::f64::consts::PI) as f32 };
            // Java: matrix.preRotate(-preRotationValue);
            matrix.preRotate(-preRotationValue);
            // Java: matrix.getValues(this.values);
            matrix.getValues(&mut self.values);
            // Java: xScale = this.values[0];
            xScale = self.values[0];
            // Java: yScale = this.values[4];
            yScale = self.values[4];
         }
         // Java: outScales[0] = xScale;
         outScales[0] = xScale;
         // Java: outScales[1] = yScale;
         outScales[1] = yScale;
    }

}
