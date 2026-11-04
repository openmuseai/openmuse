// java public class FitText
#[derive(Default)]
pub struct FitText {
    // java private int fitTextID = 0;
    fitTextID: i32,
    // java private float dxaFitText = 0.0F;
    dxaFitText: f32,
}

// java public class FitText
impl FitText {
    // java public  FitText()
    pub fn new() -> Self {
      // java this.fitTextID = 0;
      Self { fitTextID: 0, dxaFitText: 0.0 }
      // java this.dxaFitText = 0.0F;
    }

    // java public  FitText(int fitTextID, float dxaFitText)
    pub fn new__int_float(fitTextID: i32, dxaFitText: f32) -> Self {
      // java this.fitTextID = fitTextID;
      Self { fitTextID: fitTextID, dxaFitText: dxaFitText }
      // java this.dxaFitText = dxaFitText;
    }

    // java public int getID()
    pub fn getID(&self) -> i32 {
      // java return this.fitTextID;
      return self.fitTextID;
    }

    // java public void setID(int fitTextID)
    pub fn setID(&mut self, fitTextID: i32) {
      // java this.fitTextID = fitTextID;
      self.fitTextID = fitTextID;
    }

    // java public float getValue()
    pub fn getValue(&self) -> f32 {
      // java return this.dxaFitText;
      return self.dxaFitText;
    }

    // java public void setValue(float dxaFitText)
    pub fn setValue(&mut self, dxaFitText: f32) {
      // java this.dxaFitText = dxaFitText;
      self.dxaFitText = dxaFitText;
    }

}
