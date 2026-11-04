#![allow(non_snake_case)]
#![allow(dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.graphics.PointF
use core_libs_util::graphics::PointF::PointF;

use std::io;

pub trait Externalizable {}

// Java field: int Pin
pub(crate) const Pin: i32 = 1;
// Java field: int Polar
pub(crate) const Polar: i32 = 2;
// Java field: int PolarPin
pub(crate) const PolarPin: i32 = 3;

// Java: public class ADJH implements Externalizable
pub struct ADJH {
    // Java field: int f
    pub(crate) f: i32,
    // Java field: PointF pos
    pub(crate) pos: PointF,
    // Java field: int xRange
    pub(crate) xRange: i32,
    // Java field: int yRange
    pub(crate) yRange: i32,
    // Java field: int xMin
    pub(crate) xMin: i32,
    // Java field: int xMax
    pub(crate) xMax: i32,
    // Java field: int yMin
    pub(crate) yMin: i32,
    // Java field: int yMax
    pub(crate) yMax: i32,
}

impl Externalizable for ADJH {}

// Java: public class ADJH implements Externalizable
impl ADJH {
    // Java: ADJH()
    pub fn new() -> Self {
        // Java: this.pos = new PointF();
        // LLM JavaLine Aggregation: merged into struct initializer below
        // Java: super();
        ADJH { f: 1, pos: PointF::new(), xRange: 0, yRange: 0, xMin: 0, xMax: 0, yMin: 0, yMax: 0 }
    }

    // Java: ADJH(PointF pos)
    pub fn new_from(pos: PointF) -> Self {
        // Java: this.pos = pos;
        // LLM JavaLine Aggregation: merged into struct initializer below
        // Java: super();
        ADJH { f: 1, pos, xRange: 0, yRange: 0, xMin: 0, xMax: 0, yMin: 0, yMax: 0 }
    }

    // Java: void setType(int f)
    pub fn setType(&mut self, f: i32) {
      // Java: this.f = f;
      self.f = f;
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
      // Java: return this.f;
      self.f
    }

    // Java: PointF getPos()
    pub fn getPos(&self) -> &PointF {
      // Java: return this.pos;
      &self.pos
    }

    // Java: void setXRange(int range)
    pub fn setXRange(&mut self, range: i32) {
      // Java: this.xRange = range;
      self.xRange = range;
    }

    // Java: int getXRange()
    pub fn getXRange(&self) -> i32 {
      // Java: return this.xRange;
      self.xRange
    }

    // Java: void setYRange(int range)
    pub fn setYRange(&mut self, range: i32) {
      // Java: this.yRange = range;
      self.yRange = range;
    }

    // Java: int getYRange()
    pub fn getYRange(&self) -> i32 {
      // Java: return this.yRange;
      self.yRange
    }

    // Java: void setXMin(int min)
    pub fn setXMin(&mut self, min: i32) {
      // Java: this.xMin = min;
      self.xMin = min;
    }

    // Java: int getXMin()
    pub fn getXMin(&self) -> i32 {
      // Java: return this.xMin;
      self.xMin
    }

    // Java: void setXMax(int max)
    pub fn setXMax(&mut self, max: i32) {
      // Java: this.xMax = max;
      self.xMax = max;
    }

    // Java: int getXMax()
    pub fn getXMax(&self) -> i32 {
      // Java: return this.xMax;
      self.xMax
    }

    // Java: void setYMin(int min)
    pub fn setYMin(&mut self, min: i32) {
      // Java: this.yMin = min;
      self.yMin = min;
    }

    // Java: int getYMin()
    pub fn getYMin(&self) -> i32 {
      // Java: return this.yMin;
      self.yMin
    }

    // Java: void setYMax(int max)
    pub fn setYMax(&mut self, max: i32) {
      // Java: this.yMax = max;
      self.yMax = max;
    }

    // Java: int getYMax()
    pub fn getYMax(&self) -> i32 {
      // Java: return this.yMax;
      self.yMax
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &mut dyn io::Read) -> io::Result<()> {
      let mut buf4 = [0u8; 4];
      // Java: this.f = input.readInt();
      input.read_exact(&mut buf4)?;
      self.f = i32::from_be_bytes(buf4);
      // Java: this.pos.readExternal(input);
      self.pos.readExternal(input)?;
      // Java: this.xRange = input.readInt();
      input.read_exact(&mut buf4)?;
      self.xRange = i32::from_be_bytes(buf4);
      // Java: this.yRange = input.readInt();
      input.read_exact(&mut buf4)?;
      self.yRange = i32::from_be_bytes(buf4);
      // Java: this.xMin = input.readInt();
      input.read_exact(&mut buf4)?;
      self.xMin = i32::from_be_bytes(buf4);
      // Java: this.xMax = input.readInt();
      input.read_exact(&mut buf4)?;
      self.xMax = i32::from_be_bytes(buf4);
      // Java: this.yMin = input.readInt();
      input.read_exact(&mut buf4)?;
      self.yMin = i32::from_be_bytes(buf4);
      // Java: this.yMax = input.readInt();
      input.read_exact(&mut buf4)?;
      self.yMax = i32::from_be_bytes(buf4);
      Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut dyn io::Write) -> io::Result<()> {
      // Java: output.writeInt(this.f);
      output.write_all(&self.f.to_be_bytes())?;
      // Java: this.pos.writeExternal(output);
      self.pos.writeExternal(output)?;
      // Java: output.writeInt(this.xRange);
      output.write_all(&self.xRange.to_be_bytes())?;
      // Java: output.writeInt(this.yRange);
      output.write_all(&self.yRange.to_be_bytes())?;
      // Java: output.writeInt(this.xMin);
      output.write_all(&self.xMin.to_be_bytes())?;
      // Java: output.writeInt(this.xMax);
      output.write_all(&self.xMax.to_be_bytes())?;
      // Java: output.writeInt(this.yMin);
      output.write_all(&self.yMin.to_be_bytes())?;
      // Java: output.writeInt(this.yMax);
      output.write_all(&self.yMax.to_be_bytes())?;
      Ok(())
    }

}
