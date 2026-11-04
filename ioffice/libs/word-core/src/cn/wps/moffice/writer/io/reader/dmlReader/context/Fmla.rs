#![allow(non_snake_case, dead_code)]
use std::{cell::RefCell, rc::{Rc, Weak}};
use util::base::io::TypeConversion::TypeConversion;
use super::{FormulaType::FormulaType, Gds::{Gds, GdsRef}, SchemeClr::java_hash};

// Java: public class Fmla
pub struct Fmla {
    // Java: private Gds mGds; each formula refers back to its owning table.
    mGds: Weak<RefCell<Gds>>,
    mFormula: i32,
    mArgX: Option<String>, mArgY: Option<String>, mArgZ: Option<String>,
    mTmpArgX: Option<f64>, mTmpArgY: Option<f64>, mTmpArgZ: Option<f64>,
    mResult: Option<f64>,
}
impl Fmla {
    // Java: public Fmla(String value, Gds gds)
    pub fn new(value: &str, gds: &GdsRef) -> Self {
        let mut this = Self { mGds: Rc::downgrade(gds), mFormula: 0,
            mArgX: None, mArgY: None, mArgZ: None,
            mTmpArgX: None, mTmpArgY: None, mTmpArgZ: None, mResult: None };
        this.parse(value);
        this
    }
    // Java: public Double result(Integer w, Integer h)
    pub fn result(&mut self, w: Option<i32>, h: Option<i32>) -> Option<f64> {
        if self.mResult.is_none() {
            self.initArgs(w,h);
            self.mResult = self.calc();
        }
        self.mResult
    }
    fn initArgs(&mut self, w: Option<i32>, h: Option<i32>) {
        self.mTmpArgX = self.arg(self.mArgX.as_deref(), w,h);
        self.mTmpArgY = self.arg(self.mArgY.as_deref(), w,h);
        self.mTmpArgZ = self.arg(self.mArgZ.as_deref(), w,h);
    }
    fn arg(&self, val: Option<&str>, w: Option<i32>, h: Option<i32>) -> Option<f64> {
        let val = val?;
        let w = w.unwrap_or(0);
        let h = h.unwrap_or(0);
        if let Some(ret) = Gds::getPredefinedConstant(java_hash(val),w,h) { return Some(ret as f64); }
        if let Some(tmp) = TypeConversion::string2Float(val) { return Some(tmp as f64); }
        self.mGds.upgrade()?.borrow().getDouble(Some(java_hash(val)), Some(w), Some(h))
    }
    fn calc(&self) -> Option<f64> {
        match self.mFormula {
            1 => self.mul_div(), 2 => self.add_subract(), 3 => self.add_div(),
            4 => self.if_else(), 5 => self.abs(), 6 => self.at2(), 7 => self.cat2(),
            8 => self.cos(), 9 => self.max(), 10 => self.min(), 11 => self.r#mod(),
            12 => self.pin(), 13 => self.sat2(), 14 => self.sin(), 15 => self.sqrt(),
            16 => self.tan(), 17 => self.val(), _ => None,
        }
    }
    fn tan(&self) -> Option<f64> { Some(self.mTmpArgX? * self.mTmpArgY?.tan()) }
    fn sqrt(&self) -> Option<f64> { Some(self.mTmpArgX?.sqrt()) }
    fn sin(&self) -> Option<f64> { Some(self.mTmpArgX? * (self.mTmpArgY? / 60000.0_f32 as f64).to_radians().sin()) }
    fn sat2(&self) -> Option<f64> { Some(self.mTmpArgX? * (self.mTmpArgZ? / self.mTmpArgY?).tan().sin()) }
    fn r#mod(&self) -> Option<f64> {
        let x=self.mTmpArgX?; let y=self.mTmpArgY?; let z=self.mTmpArgZ?;
        Some((x*x+y*y+z*z).sqrt())
    }
    fn min(&self) -> Option<f64> { let x=self.mTmpArgX?; let y=self.mTmpArgY?; Some(if x<y{x}else{y}) }
    fn max(&self) -> Option<f64> { let x=self.mTmpArgX?; let y=self.mTmpArgY?; Some(if x>y{x}else{y}) }
    fn cos(&self) -> Option<f64> { Some(self.mTmpArgX? * (self.mTmpArgY? / 60000.0_f32 as f64).to_radians().cos()) }
    fn cat2(&self) -> Option<f64> { let x=self.mTmpArgX?; let y=self.mTmpArgY?; self.mTmpArgZ?; Some(x*(y/x).tan().cos()) }
    fn at2(&self) -> Option<f64> { Some((self.mTmpArgY? / self.mTmpArgX?).tan()) }
    fn abs(&self) -> Option<f64> { Some(self.mTmpArgX?.abs()) }
    fn if_else(&self) -> Option<f64> { let x=self.mTmpArgX?; let y=self.mTmpArgY?; let z=self.mTmpArgZ?; Some(if x>0.0 {y}else{z}) }
    fn add_div(&self) -> Option<f64> { Some((self.mTmpArgX?+self.mTmpArgY?)/self.mTmpArgZ?) }
    fn add_subract(&self) -> Option<f64> { Some(self.mTmpArgX?+self.mTmpArgY?-self.mTmpArgZ?) }
    fn pin(&self) -> Option<f64> {
        let x=self.mTmpArgX?; let y=self.mTmpArgY?; let z=self.mTmpArgZ?;
        Some(if y<x {x} else if y>z {z} else {y})
    }
    fn val(&self) -> Option<f64> { self.mTmpArgX }
    fn mul_div(&self) -> Option<f64> {
        let x=self.mTmpArgX?; let y=self.mTmpArgY?; let z=self.mTmpArgZ?;
        Some(if z==0.0 {0.0}else{x*y/z})
    }
    // Java: private void parse(String value)
    fn parse(&mut self, value: &str) {
        // Java String.split(" ") retains interior empty fields and removes trailing empty fields.
        let mut strs: Vec<&str> = value.split(' ').collect();
        while strs.last() == Some(&"") { strs.pop(); }
        if strs.is_empty() { return; }
        self.mFormula = FormulaType::toType(strs[0]);
        match self.mFormula {
            1|2|3|4|7|11|12|13 if strs.len()==4 => {
                self.mArgX=Some(strs[1].into()); self.mArgY=Some(strs[2].into()); self.mArgZ=Some(strs[3].into());
            }
            5|15|17 if strs.len()==2 => { self.mArgX=Some(strs[1].into()); }
            6|8|9|10|14|16 if strs.len()==3 => {
                self.mArgX=Some(strs[1].into()); self.mArgY=Some(strs[2].into());
            }
            _ => {}
        }
    }
}
