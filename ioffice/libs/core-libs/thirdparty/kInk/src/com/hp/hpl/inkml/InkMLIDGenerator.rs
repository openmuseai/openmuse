use std::sync::atomic::{AtomicI64, Ordering};

// Java field: long idForContext
static idForContext: AtomicI64 = AtomicI64::new(0);
// Java field: long idForBrush
static idForBrush: AtomicI64 = AtomicI64::new(0);
// Java field: long idForTF
static idForTF: AtomicI64 = AtomicI64::new(0);
// Java field: long idForInkSrc
static idForInkSrc: AtomicI64 = AtomicI64::new(0);
// Java field: long idForCanvas
static idForCanvas: AtomicI64 = AtomicI64::new(0);
// Java field: long idForCT
static idForCT: AtomicI64 = AtomicI64::new(0);
// Java field: long idForTS
static idForTS: AtomicI64 = AtomicI64::new(0);

// Java: public class InkMLIDGenerator
pub struct InkMLIDGenerator {
}

// Java: public class InkMLIDGenerator
impl InkMLIDGenerator {
    // Java: InkMLIDGenerator()
    pub fn InkMLIDGenerator() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static String getNextIDForContext()
    pub fn getNextIDForContext() -> String {
      // Java: String id = "Ctx" + idForContext;
      let id = format!("Ctx{}", idForContext.load(Ordering::SeqCst));
      // Java: ++idForContext;
      idForContext.fetch_add(1, Ordering::SeqCst);
      // Java: return id;
      id
    }

    // Java: static String getNextIDForBrush()
    pub fn getNextIDForBrush() -> String {
      // Java: String id = "Brush" + idForBrush;
      let id = format!("Brush{}", idForBrush.load(Ordering::SeqCst));
      // Java: ++idForBrush;
      idForBrush.fetch_add(1, Ordering::SeqCst);
      // Java: return id;
      id
    }

    // Java: static String getNextIDForTF()
    pub fn getNextIDForTF() -> String {
      // Java: String id = "TF" + idForTF;
      let id = format!("TF{}", idForTF.load(Ordering::SeqCst));
      // Java: ++idForTF;
      idForTF.fetch_add(1, Ordering::SeqCst);
      // Java: return id;
      id
    }

    // Java: static String getNextIDForInkSource()
    pub fn getNextIDForInkSource() -> String {
      // Java: String id = "IS" + idForInkSrc;
      let id = format!("IS{}", idForInkSrc.load(Ordering::SeqCst));
      // Java: ++idForInkSrc;
      idForInkSrc.fetch_add(1, Ordering::SeqCst);
      // Java: return id;
      id
    }

    // Java: static String getNextIDForCanvas()
    pub fn getNextIDForCanvas() -> String {
      // Java: String id = "CAnvas" + idForCanvas;
      let id = format!("CAnvas{}", idForCanvas.load(Ordering::SeqCst));
      // Java: ++idForCanvas;
      idForCanvas.fetch_add(1, Ordering::SeqCst);
      // Java: return id;
      id
    }

    // Java: static String getNextIDForCanvasTransform()
    pub fn getNextIDForCanvasTransform() -> String {
      // Java: String id = "CT" + idForCT;
      let id = format!("CT{}", idForCT.load(Ordering::SeqCst));
      // Java: ++idForCT;
      idForCT.fetch_add(1, Ordering::SeqCst);
      // Java: return id;
      id
    }

    // Java: static String getNextIDForTimeStamp()
    pub fn getNextIDForTimeStamp() -> String {
      // Java: String id = "TS" + idForTS;
      let id = format!("TS{}", idForTS.load(Ordering::SeqCst));
      // Java: ++idForTS;
      idForTS.fetch_add(1, Ordering::SeqCst);
      // Java: return id;
      id
    }

}
