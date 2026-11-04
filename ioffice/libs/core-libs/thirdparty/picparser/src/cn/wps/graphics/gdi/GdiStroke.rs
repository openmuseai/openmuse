#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import android.graphics.DashPathEffect
#[derive(Clone)]
pub struct DashPathEffect {
    intervals: Vec<f32>,
    phase: f32,
}

impl DashPathEffect {
    pub fn new(intervals: &[f32], phase: f32) -> Self {
        DashPathEffect { intervals: intervals.to_vec(), phase }
    }
}

// Java: import android.graphics.Paint
pub struct Paint {
    stroke_width: f32,
    stroke_miter: f32,
    path_effect: Option<DashPathEffect>,
}

impl Paint {
    pub fn setStrokeWidth(&mut self, width: f32) {
        self.stroke_width = width;
    }
    pub fn setStrokeMiter(&mut self, miter: f32) {
        self.stroke_miter = miter;
    }
    pub fn setPathEffect(&mut self, effect: Option<DashPathEffect>) {
        self.path_effect = effect;
    }
}

// Java: public class GdiStroke
#[derive(Clone)]
pub struct GdiStroke {
    // Java field: float width
    pub(crate) width: f32,
    // Java field: int cap
    cap: i32,
    // Java field: int join
    join: i32,
    // Java field: float miterlimit
    miterlimit: f32,
    // Java field: float[] dash
    pub(crate) dash: Option<Vec<f32>>,
    // Java field: float dash_phase
    pub(crate) dash_phase: f32,
    // Java field: DashPathEffect pathEffect
    pathEffect: Option<DashPathEffect>,
}

// Java: public class GdiStroke
impl GdiStroke {
    // Java: GdiStroke(float width, int cap, int join, float miterlimit, float[] dash, float dash_phase)
    pub fn new(width: f32, cap: i32, join: i32, miterlimit: f32, dash: Option<Vec<f32>>, dash_phase: f32) -> Self {
        // Java: super();
        // Java: this.width = width;
        // Java: this.cap = cap;
        // Java: this.join = join;
        // Java: this.miterlimit = miterlimit;
        // Java: this.dash = dash;
        // Java: this.dash_phase = dash_phase;
        let path_effect;
        // Java: if (dash != null)
        if let Some(ref d) = dash {
            // Java: this.pathEffect = new DashPathEffect(dash, dash_phase);
            path_effect = Some(DashPathEffect::new(d, dash_phase));
        } else {
            path_effect = None;
        }
        GdiStroke {
            width,
            cap,
            join,
            miterlimit,
            dash,
            dash_phase,
            pathEffect: path_effect,
        }
    }

    // Java: GdiStroke()
    pub fn new_default() -> Self {
        // Java: super();
        GdiStroke {
            width: 1.0,
            cap: 0,
            join: 0,
            miterlimit: 0.0,
            dash: None,
            dash_phase: 0.0,
            pathEffect: None,
        }
    }

    // Java: void initPaint(Paint paint, float minStroke, float scaleDc)
    pub fn initPaint(&self, paint: &mut Paint, minStroke: f32, scaleDc: f32) {
        // Java: float strokeWidth = Math.max(minStroke, this.width * scaleDc);
        let strokeWidth = minStroke.max(self.width * scaleDc);
        // Java: paint.setStrokeWidth(strokeWidth);
        paint.setStrokeWidth(strokeWidth);
        // Java: paint.setStrokeMiter(this.miterlimit);
        paint.setStrokeMiter(self.miterlimit);
        // Java: paint.setPathEffect(this.pathEffect);
        paint.setPathEffect(None);
    }

    // Java: float getWidth()
    pub fn getWidth(&self) -> f32 {
        // Java: return this.width;
        self.width
    }

    // Java: void setWidth(float value)
    pub fn setWidth(&mut self, value: f32) {
        // Java: this.width = value;
        self.width = value;
    }
}
