#![allow(non_snake_case)]
// Java: public class CalcPr
pub struct CalcPr {
    calcCompleted: bool,
    calcId: i32,
    calcMode: String,
    calcOnSave: bool,
    concurrentCalc: bool,
    concurrentManualcCount: i32,
    forceFullCalc: bool,
    fullCalcOnLoad: bool,
    fullPrecision: bool,
    iterate: bool,
    iterateCount: i32,
    iterateDelta: f64,
    refMode: String,
}
impl Default for CalcPr {
    fn default() -> Self {
        Self { calcCompleted: true, calcId: 0, calcMode: "auto".into(), calcOnSave: true,
            concurrentCalc: true, concurrentManualcCount: 0, forceFullCalc: false,
            fullCalcOnLoad: false, fullPrecision: true, iterate: false, iterateCount: 100,
            iterateDelta: 0.001, refMode: "A1".into() }
    }
}
impl CalcPr {
    // Java: public boolean isCalcCompleted()
    pub fn isCalcCompleted(&self) -> bool { self.calcCompleted }
    // Java: public void setCalcCompleted(boolean calcCompleted)
    pub fn setCalcCompleted(&mut self, value: bool) { self.calcCompleted = value; }
    // Java: public int getCalcId()
    pub fn getCalcId(&self) -> i32 { self.calcId }
    // Java: public void setCalcId(int calcId)
    pub fn setCalcId(&mut self, value: i32) { self.calcId = value; }
    // Java: public String getCalcMode()
    pub fn getCalcMode(&self) -> &str { &self.calcMode }
    // Java: public void setCalcMode(String calcMode)
    pub fn setCalcMode(&mut self, value: String) { self.calcMode = value; }
    // Java: public boolean isCalcOnSave()
    pub fn isCalcOnSave(&self) -> bool { self.calcOnSave }
    // Java: public void setCalcOnSave(boolean calcOnSave)
    pub fn setCalcOnSave(&mut self, value: bool) { self.calcOnSave = value; }
    // Java: public boolean isConcurrentCalc()
    pub fn isConcurrentCalc(&self) -> bool { self.concurrentCalc }
    // Java: public void setConcurrentCalc(boolean concurrentCalc)
    pub fn setConcurrentCalc(&mut self, value: bool) { self.concurrentCalc = value; }
    // Java: public int getConcurrentManualcCount()
    pub fn getConcurrentManualcCount(&self) -> i32 { self.concurrentManualcCount }
    // Java: public void setConcurrentManualcCount(int concurrentManualcCount)
    pub fn setConcurrentManualcCount(&mut self, value: i32) { self.concurrentManualcCount = value; }
    // Java: public boolean isForceFullCalc()
    pub fn isForceFullCalc(&self) -> bool { self.forceFullCalc }
    // Java: public void setForceFullCalc(boolean forceFullCalc)
    pub fn setForceFullCalc(&mut self, value: bool) { self.forceFullCalc = value; }
    // Java: public boolean isFullCalcOnLoad()
    pub fn isFullCalcOnLoad(&self) -> bool { self.fullCalcOnLoad }
    // Java: public void setFullCalcOnLoad(boolean fullCalcOnLoad)
    pub fn setFullCalcOnLoad(&mut self, value: bool) { self.fullCalcOnLoad = value; }
    // Java: public boolean isFullPrecision()
    pub fn isFullPrecision(&self) -> bool { self.fullPrecision }
    // Java: public void setFullPrecision(boolean fullPrecision)
    pub fn setFullPrecision(&mut self, value: bool) { self.fullPrecision = value; }
    // Java: public boolean isIterate()
    pub fn isIterate(&self) -> bool { self.iterate }
    // Java: public void setIterate(boolean iterate)
    pub fn setIterate(&mut self, value: bool) { self.iterate = value; }
    // Java: public int getIterateCount()
    pub fn getIterateCount(&self) -> i32 { self.iterateCount }
    // Java: public void setIterateCount(int iterateCount)
    pub fn setIterateCount(&mut self, value: i32) { self.iterateCount = value; }
    // Java: public double getIterateDelta()
    pub fn getIterateDelta(&self) -> f64 { self.iterateDelta }
    // Java: public void setIterateDelta(double iterateDelta)
    pub fn setIterateDelta(&mut self, value: f64) { self.iterateDelta = value; }
    // Java: public String getRefMode()
    pub fn getRefMode(&self) -> &str { &self.refMode }
    // Java: public void setRefMode(String refMode)
    pub fn setRefMode(&mut self, value: String) { self.refMode = value; }
}
