#![allow(dead_code, non_snake_case, unused_variables, unused_mut)]

// Java: import android.graphics.Paint
// (android.graphics.Paint not available in Rust)
// Java: import android.graphics.PorterDuff
// (android.graphics.PorterDuff not available in Rust)
// Java: import android.graphics.Xfermode
// (android.graphics.Xfermode not available in Rust)
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Map
// Java: import java.util.Vector
use std::sync::{Mutex, OnceLock};

use super::KPdfObject::KPdfObject;
use super::KStream::KStream;

pub type Paint = ();
pub type Xfermode = ();

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PorterDuffMode {
    SrcOver,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    Other,
}

// Java field: Map<Paint, KPdfGraphicsState> gStateMap
// Java field: Vector<KPdfGraphicsState> gStates
static G_STATES: OnceLock<Mutex<Vec<KPdfGraphicsState>>> = OnceLock::new();

fn get_states() -> &'static Mutex<Vec<KPdfGraphicsState>> {
    G_STATES.get_or_init(|| Mutex::new(Vec::new()))
}

// Java: public class KPdfGraphicsState extends KPdfDict
pub struct KPdfGraphicsState {
    // Java field: Paint mPaint
    mPaint: Paint,
    // Java field: boolean mPopulated
    mPopulated: bool,
    // Java field: int alpha
    pub(crate) alpha: i32,
    // Java field: int cap
    pub(crate) cap: i32,
    // Java field: int join
    pub(crate) join: i32,
    // Java field: float width
    pub(crate) width: f32,
    // Java field: float miter
    pub(crate) miter: f32,
    // Java field: Xfermode mode
    pub(crate) mode: Option<Xfermode>,
}

// Java: public class KPdfGraphicsState extends KPdfDict
#[allow(non_snake_case)]
impl KPdfGraphicsState {
    // Java: static KPdfGraphicsState GetGraphicStateForPaint(Paint paint)
    pub fn GetGraphicStateForPaint(_paint: &Paint) -> KPdfGraphicsState {
      // Java: int size = gStates.size();
      let states = get_states().lock().unwrap();
      let size = states.len();
      // Java: for(int i = 0; i < size; ++i)
      for _i in 0..size {
         // Java: KPdfGraphicsState state = (KPdfGraphicsState)gStates.get(i);
         // Java: if (state.alpha == paint.getAlpha() && state.cap == paint.getStrokeCap().ordinal() && state.join == paint.getStrokeJoin().ordinal() && state.width == paint.getStrokeWidth() && state.miter == paint.getStrokeMiter() && isSameMode(state.mode, paint.getXfermode()))
         {
            // Java: return state;
         }
      }
      drop(states);
      // Java: KPdfGraphicsState graphicsState = new KPdfGraphicsState(paint);
      let graphicsState = KPdfGraphicsState::new();
      // Java: gStateMap.put(paint, graphicsState);
      // Java: gStates.add(graphicsState);
      get_states().lock().unwrap().push(KPdfGraphicsState::new());
      // Java: return graphicsState;
      graphicsState
    }

    // Java: static boolean isSameMode(Xfermode mode1, Xfermode mode2)
    pub fn isSameMode(mode1: &Option<Xfermode>, mode2: &Option<Xfermode>) -> bool {
      // Java: if (mode1 == null && mode2 == null)
      if mode1.is_none() && mode2.is_none() {
         // Java: return true;
         return true;
      }
      // Java: else
      {
         // Java: return mode1 != null && mode2 != null ? mode1.equals(mode2) : false;
         mode1.is_some() && mode2.is_some()
      }
    }

    // Java: static void clearGraphicsState()
    pub fn clearGraphicsState() {
      // Java: gStateMap.clear();
      // Java: gStates.clear();
      get_states().lock().unwrap().clear();
    }

    // Java: KPdfGraphicsState(Paint paint)
    pub fn new() -> Self {
      // Java: super();
      // Java: this.mPaint = paint;
      let mut this = Self {
          mPaint: (),
          mPopulated: false,
          alpha: 0,
          cap: 0,
          join: 0,
          width: 0.0,
          miter: 0.0,
          mode: None,
      };
      // Java: this.populateDict();
      this.populateDict();
      this
    }

    // Java: void getResources(Vector<KPdfObject> resources)
    pub fn getResources(&self, _resources: &mut Vec<KPdfObject>) {
    }

    // Java: long getOutputSize(KPdfCatalog catalog, boolean indirect)
    pub fn getOutputSize(&mut self, _catalog: &KPdfObject, _indirect: bool) -> i64 {
      // Java: this.populateDict();
      self.populateDict();
      // Java: return super.getOutputSize(catalog, indirect);
      0
    }

    // Java: void emit(KStream stream, KPdfCatalog catalog, boolean indirect)
    pub fn emit(&mut self, _stream: &mut KStream, _catalog: &KPdfObject, _indirect: bool) {
      // Java: this.populateDict();
      self.populateDict();
      // Java: super.emit(stream, catalog, indirect);
    }

    // Java: static String blendModeFromXfermode(PorterDuff.Mode mode)
    pub fn blendModeFromXfermode(mode: PorterDuffMode) -> Option<&'static str> {
      // Java: switch (mode)
      match mode {
         // Java: case SRC_OVER:
         PorterDuffMode::SrcOver => {
            // Java: return "Normal";
            Some("Normal")
         }
         // Java: case MULTIPLY:
         PorterDuffMode::Multiply => {
            // Java: return "Multiply";
            Some("Multiply")
         }
         // Java: case SCREEN:
         PorterDuffMode::Screen => {
            // Java: return "Screen";
            Some("Screen")
         }
         // Java: case OVERLAY:
         PorterDuffMode::Overlay => {
            // Java: return "Overlay";
            Some("Overlay")
         }
         // Java: case DARKEN:
         PorterDuffMode::Darken => {
            // Java: return "Darken";
            Some("Darken")
         }
         // Java: case LIGHTEN:
         PorterDuffMode::Lighten => {
            // Java: return "Lighten";
            Some("Lighten")
         }
         // Java: default:
         _ => {
            // Java: return null;
            None
         }
      }
    }

    // Java: void populateDict()
    pub fn populateDict(&mut self) {
      // Java: if (!this.mPopulated)
      if !self.mPopulated {
         // Java: this.insertName("Type", "ExtGState");
         // Java: this.alpha = this.mPaint.getAlpha();
         // Java: KPdfScalar alphaf = new KPdfScalar((float)this.alpha / 255.0F);
         // Java: this.insert("CA", alphaf);
         // Java: this.insert("ca", alphaf);
         // Java: this.cap = this.mPaint.getStrokeCap().ordinal();
         // Java: this.insert("LC", this.cap);
         // Java: this.join = this.mPaint.getStrokeJoin().ordinal();
         // Java: this.insert("LJ", this.join);
         // Java: this.width = this.mPaint.getStrokeWidth();
         // Java: this.insert("LW", Math.max(0.5F, this.width));
         // Java: this.miter = this.mPaint.getStrokeMiter();
         // Java: this.insert("ML", this.miter);
         // Java: this.insert("SA", true);
         // Java: Xfermode xMode = this.mPaint.getXfermode();
         // Java: if (xMode != null && xMode instanceof KPorterDuffXfermode)
         {
            // Java: KPorterDuffXfermode pdMode = (KPorterDuffXfermode)xMode;
            // Java: PorterDuff.Mode mode = pdMode.getMode();
            // Java: String bm = blendModeFromXfermode(mode);
            // Java: if (bm != null)
            {
               // Java: this.insertName("BM", bm);
            }
         }
         // Java: this.mPopulated = true;
         self.mPopulated = true;
      }
    }

}
