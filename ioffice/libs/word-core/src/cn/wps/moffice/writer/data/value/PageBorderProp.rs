#![allow(dead_code, non_snake_case)]

// java public class PageBorderProp
#[derive(Default, Clone)]
pub struct PageBorderProp {
    // java private int mPgbApplyTo = 0;
    mPgbApplyTo: i32,
    // java private int mPgbPageDepth = 0;
    mPgbPageDepth: i32,
    // java private int mPgbOffsetFrom = 0;
    mPgbOffsetFrom: i32,
}

impl PageBorderProp {
    pub fn setApplyto(&mut self, applyto: i32) {
        self.mPgbApplyTo = applyto;
    }

    pub fn setPageDepth(&mut self, depth: i32) {
        self.mPgbPageDepth = depth;
    }

    pub fn setPgbOffsetFrom(&mut self, offsetFrom: i32) {
        self.mPgbOffsetFrom = offsetFrom;
    }

    pub fn getApplyto(&self) -> i32 {
        self.mPgbApplyTo
    }

    pub fn getPageDepth(&self) -> i32 {
        self.mPgbPageDepth
    }

    pub fn getPgbOffsetFrom(&self) -> i32 {
        self.mPgbOffsetFrom
    }
}
