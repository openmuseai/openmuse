// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 4951916710448210624;

// Java: public class TracePoint extends PointF implements Serializable
pub struct TracePoint {
    pub x: f32,
    pub y: f32,
    // Java field: float f
    pub f: f32,
    // Java field: long ts
    pub ts: i64,
}

// Java: public class TracePoint extends PointF implements Serializable
impl TracePoint {
    // Java: TracePoint(float paramFloat1, float paramFloat2)
    pub fn TracePoint_xy(param_float1: f32, param_float2: f32) -> Self {
      // Java: super(paramFloat1, paramFloat2);
      Self { x: param_float1, y: param_float2, f: 0.0, ts: 0 }
    }

    // Java: TracePoint(float paramFloat1, float paramFloat2, float paramFloat3)
    pub fn TracePoint_xyf(param_float1: f32, param_float2: f32, param_float3: f32) -> Self {
      // Java: super(paramFloat1, paramFloat2);
      // Java: this.f = paramFloat3;
      Self { x: param_float1, y: param_float2, f: param_float3, ts: 0 }
    }

    // Java: TracePoint(float paramFloat1, float paramFloat2, float paramFloat3, long paramLong)
    pub fn TracePoint_xyft(param_float1: f32, param_float2: f32, param_float3: f32, param_long: i64) -> Self {
      // Java: this(paramFloat1, paramFloat2, paramFloat3);
      // Java: this.ts = paramLong;
      Self { x: param_float1, y: param_float2, f: param_float3, ts: param_long }
    }

    // Java: TracePoint()
    pub fn TracePoint() -> Self {
      // Java: super();
      Self { x: 0.0, y: 0.0, f: 0.0, ts: 0 }
    }

    // Java: float getX()
    pub fn getX(&self) -> f32 {
      // Java: return this.x;
      self.x
    }

    // Java: float getY()
    pub fn getY(&self) -> f32 {
      // Java: return this.y;
      self.y
    }

    // Java: Object clone()
    pub fn clone(&self) -> TracePoint {
      // Java: return new TracePoint(this.x, this.y, this.f, this.ts);
      TracePoint { x: self.x, y: self.y, f: self.f, ts: self.ts }
    }

    // Java: boolean equals(Object paramObject)
    pub fn equals(&self, param_object: &TracePoint) -> bool {
      // Java: if (paramObject instanceof TracePoint)
      {
         // Java: TracePoint localTracePoint = (TracePoint)paramObject;
         let local_trace_point = param_object;
         // Java: if (localTracePoint.x != this.x)
         if local_trace_point.x != self.x
         {
            // Java: return false;
            return false;
         }
         // Java: else if (localTracePoint.y != this.y)
         else if local_trace_point.y != self.y
         {
            // Java: return false;
            return false;
         }
         // Java: else if (localTracePoint.f != this.f)
         else if local_trace_point.f != self.f
         {
            // Java: return false;
            return false;
         }
         // Java: else
         else
         {
            // Java: return localTracePoint.ts == this.ts;
            return local_trace_point.ts == self.ts;
         }
      }
      // Java: else
      // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int hash = 0;
      let mut hash: i32 = 0;
      // Java: hash = hash * 37 + Float.floatToIntBits(this.x);
      hash = hash.wrapping_mul(37).wrapping_add(self.x.to_bits() as i32);
      // Java: hash = hash * 37 + Float.floatToIntBits(this.y);
      hash = hash.wrapping_mul(37).wrapping_add(self.y.to_bits() as i32);
      // Java: hash = hash * 37 + Float.floatToIntBits(this.f);
      hash = hash.wrapping_mul(37).wrapping_add(self.f.to_bits() as i32);
      // Java: hash = hash * 37 + (int)(this.ts ^ this.ts >>> 32);
      hash = hash.wrapping_mul(37).wrapping_add((self.ts ^ ((self.ts as u64) >> 32) as i64) as i32);
      // Java: return hash;
      hash
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return String.format("TracePoint: %f, %f, %f, %d", this.x, this.y, this.f, this.ts);
      format!("TracePoint: {}, {}, {}, {}", self.x, self.y, self.f, self.ts)
    }

}
