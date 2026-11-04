#![allow(non_snake_case)]

// Java: public class PreDefData
#[derive(Clone)]
pub struct PreDefData {
    // Java field: int[] avcLst
    pub avcLst: Vec<i32>,
    // Java field: int[] gdLst
    pub gdLst: Vec<i32>,
    // Java field: int[] ahLst
    pub ahLst: Vec<i32>,
    // Java field: int[] cxnLst
    pub cxnLst: Vec<i32>,
    // Java field: int[] rect
    pub rect: Vec<i32>,
    // Java field: int[][] pathLst
    pub pathLst: Vec<Vec<i32>>,
}

// Java: public class PreDefData
impl PreDefData {
    // Java: PreDefData()
    pub fn new() -> Self {
        // Java: super();
        PreDefData {
            avcLst: Vec::new(),
            gdLst: Vec::new(),
            ahLst: Vec::new(),
            cxnLst: Vec::new(),
            rect: Vec::new(),
            pathLst: Vec::new(),
        }
    }
}
