#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import android.graphics.Canvas
// Java: import android.graphics.Matrix
// Java: import cn.wps.graphics.Size
use core_libs_util::graphics::Size::Size;

use std::sync::Arc;
use std::sync::Mutex;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

static LOCK: Mutex<()> = Mutex::new(());

// Java: public class CanvasUtil
pub struct CanvasUtil {
    // Java field: Matrix matrix
    pub(crate) matrix: Vec<f32>,
    // Java field: float[] values
    pub(crate) values: Vec<f32>,
    // Java field: float[] scales
    pub(crate) scales: Vec<f32>,
}

// Java: public class CanvasUtil
impl CanvasUtil {
    // Java: CanvasUtil()
    pub fn new() -> Self {
      // Java: super();
      CanvasUtil { matrix: vec![0.0; 9], values: vec![0.0; 9], scales: vec![0.0; 2] }
    }

    // Java: synchronized static void calScaleXy(Canvas canvas, float[] outScales)
    pub fn calScaleXy(_canvas: &Element, outScales: &mut [f32]) {
      let _lock = LOCK.lock().unwrap();
      // Java: canvas.getMatrix(matrix);
      let mut matrix = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0_f32];
      // Java: float xScale = values[0];
      let mut xScale = matrix[0];
      // Java: matrix.getValues(values);
      let mut values = matrix.clone();
      // Java: float xSkew = values[1];
      let xSkew = values[1];
      // Java: float yScale = values[4];
      let mut yScale = values[4];
      // Java: if (xSkew != 0.0F)
      if xSkew != 0.0_f32 {
         // Java: float preRotationValue = xScale == 0.0F ? 90.0F : (float)(Math.atan((double)(-xSkew / xScale)) * (double)180.0F / Math.PI);
         let preRotationValue = if xScale == 0.0_f32 { 90.0_f32 } else { ((-xSkew / xScale) as f64).atan() as f32 * 180.0_f32 / std::f32::consts::PI };
         // Java: matrix.preRotate(-preRotationValue);
         let angle_rad = (-preRotationValue as f64).to_radians();
         let cos = angle_rad.cos() as f32;
         let sin = angle_rad.sin() as f32;
         let m0 = matrix[0] * cos + matrix[3] * sin;
         let m3 = matrix[0] * (-sin) + matrix[3] * cos;
         matrix[0] = m0;
         matrix[3] = m3;
         // Java: matrix.getValues(values);
         values = matrix.clone();
         // Java: xScale = values[0];
         xScale = values[0];
         // Java: yScale = values[4];
         yScale = values[4];
      }
      // Java: outScales[0] = xScale;
      if outScales.len() > 0 { outScales[0] = xScale; }
      // Java: outScales[1] = yScale;
      if outScales.len() > 1 { outScales[1] = yScale; }
    }

    // Java: synchronized static Size unitDensitySize(Canvas canvas, float width, float height, Size outSize, float xScale, float yScale)
    pub fn unitDensitySize(_canvas: &Element, width: f32, height: f32, outSize: Option<Size>, xScale: f32, yScale: f32) -> Size {
      let _lock = LOCK.lock().unwrap();
      let mut outSize = outSize;
      // Java: if (null == outSize)
      if outSize.is_none() {
         // Java: outSize = new Size();
         outSize = Some(Size::new());
      }
      let mut outSize = outSize.unwrap();
      // Java: float sx = xScale;
      let mut sx = xScale;
      // Java: float sy = yScale;
      let mut sy = yScale;
      // Java: if (xScale == 0.0F || yScale == 0.0F)
      if xScale == 0.0_f32 || yScale == 0.0_f32 {
         // Java: calScaleXy(canvas, scales);
         let mut scales = [0.0_f32; 2];
         CanvasUtil::calScaleXy(_canvas, &mut scales);
         // Java: sx = scales[0];
         sx = scales[0];
         // Java: sy = scales[1];
         sy = scales[1];
      }
      // Java: outSize.width = (int)(width * Math.abs(sx) + 0.5F);
      outSize.width = (width * sx.abs() + 0.5_f32) as i32;
      // Java: outSize.height = (int)(height * Math.abs(sy) + 0.5F);
      outSize.height = (height * sy.abs() + 0.5_f32) as i32;
      // Java: return outSize;
      outSize
    }

}
