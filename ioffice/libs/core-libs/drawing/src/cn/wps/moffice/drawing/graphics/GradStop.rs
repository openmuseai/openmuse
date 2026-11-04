#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.color.ColorWithTransform
// use super::super::color::ColorWithTransform::ColorWithTransform; // color module disabled
#[derive(Clone)]
pub struct ColorWithTransform;
// Java: import java.io.Externalizable
use std::io;
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput

// Java field: float FORECOLOR_POSTION
pub static FORECOLOR_POSTION: f32 = 0.0_f32;
// Java field: float BACKCOLOR_POSTION
pub static BACKCOLOR_POSTION: f32 = 1.0_f32;

// Java: public class GradStop implements Externalizable, Cloneable
pub struct GradStop {
    // Java field: ColorWithTransform mColorWithTransform
    pub(crate) mColorWithTransform: Option<ColorWithTransform>,
    // Java field: float postion
    pub(crate) postion: f32,
    // Java field: int color
    pub(crate) color: i32,
}

// Java: public class GradStop implements Externalizable, Cloneable
impl GradStop {
    // Java: GradStop()
    pub fn new() -> Self {
      // Java: super();
      GradStop {
          mColorWithTransform: None,
          postion: 0.0_f32,
          color: 0,
      }
    }

    // Java: GradStop(float pos, int clr)
    pub fn new_with(pos: f32, clr: i32) -> Self {
      // Java: super();
      GradStop {
          mColorWithTransform: None,
          // Java: this.postion = pos;
          postion: pos,
          // Java: this.color = clr;
          color: clr,
      }
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &GradStop) -> bool {
      // Java: if (o != null && o instanceof GradStop)
      {
         // Java: if (o == this)
         if std::ptr::eq(o, self) {
            // Java: return true;
            return true;
         }
         // Java: else
         {
            // Java: GradStop gs = (GradStop)o;
            let gs = o;
            // Java: return Math.abs(gs.postion - this.postion) < 1.0E-4F && this.color == gs.color;
            return (gs.postion - self.postion).abs() < 1.0e-4_f32 && self.color == gs.color;
         }
      }
      // Java: else
      {
         // Java: return false;
         false
      }
    }

    // Java: void setPostion(float pos)
    pub fn setPostion(&mut self, pos: f32) {
      // Java: this.postion = pos;
      self.postion = pos;
    }

    // Java: float getPostion()
    pub fn getPostion(&self) -> f32 {
      // Java: return this.postion;
      self.postion
    }

    // Java: void setColr(int clr)
    pub fn setColr(&mut self, clr: i32) {
      // Java: this.color = clr;
      self.color = clr;
    }

    // Java: int getColor()
    pub fn getColor(&self) -> i32 {
      // Java: return this.color;
      self.color
    }

    // Java: float getAlpha()
    pub fn getAlpha(&self) -> f32 {
      // Java: float alpha = (float)(this.color >> 24 & 255);
      let alpha = (self.color >> 24 & 255) as f32;
      // Java: return alpha / 255.0F;
      alpha / 255.0_f32
    }

    // Java: void setAlpha(float alpha)
    pub fn setAlpha(&mut self, alpha: f32) {
      // Java: int a = (int)(alpha * 255.0F);
      let a = (alpha * 255.0_f32) as i32;
      // Java: this.color = this.color & 16777215 | a << 24 & -16777216;
      self.color = self.color & 0x00FF_FFFF | (a << 24 & (0xFF00_0000u32 as i32));
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &mut dyn io::Read) -> io::Result<()> {
      // Java: this.postion = input.readFloat();
      let mut buf4 = [0u8; 4];
      input.read_exact(&mut buf4)?;
      self.postion = f32::from_be_bytes(buf4);
      // Java: this.color = input.readInt();
      input.read_exact(&mut buf4)?;
      self.color = i32::from_be_bytes(buf4);
      Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut dyn io::Write) -> io::Result<()> {
      // Java: output.writeFloat(this.postion);
      output.write_all(&self.postion.to_be_bytes())?;
      // Java: output.writeInt(this.color);
      output.write_all(&self.color.to_be_bytes())?;
      Ok(())
    }

    // Java: ColorWithTransform getColorWithTransform()
    pub fn getColorWithTransform(&self) -> &Option<ColorWithTransform> {
      // Java: return this.mColorWithTransform;
      &self.mColorWithTransform
    }

    // Java: void setColorWithTransform(ColorWithTransform colorWithTransform)
    pub fn setColorWithTransform(&mut self, colorWithTransform: Option<ColorWithTransform>) {
      // Java: this.mColorWithTransform = colorWithTransform;
      self.mColorWithTransform = colorWithTransform;
    }

    // Java: GradStop clone()
    pub fn clone(&self) -> GradStop {
      // Java: return new GradStop(this.postion, this.color);
      GradStop::new_with(self.postion, self.color)
    }

}

impl Clone for GradStop {
    fn clone(&self) -> Self {
        Self {
            mColorWithTransform: self.mColorWithTransform.clone(),
            postion: self.postion,
            color: self.color,
        }
    }
}
