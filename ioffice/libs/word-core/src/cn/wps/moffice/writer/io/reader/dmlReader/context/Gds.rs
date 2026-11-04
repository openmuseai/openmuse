#![allow(non_snake_case, non_upper_case_globals)]
use std::{cell::RefCell, collections::HashMap, rc::{Rc, Weak}};
use drawing::cn::wps::moffice::drawing::smartshape::GeoAdjust::GeoAdjust;
use super::{Fmla::Fmla, SchemeClr::java_hash};

// Java: public class Gds
pub type GdsRef = Rc<RefCell<Gds>>;
pub struct Gds { mGds: HashMap<i32, Rc<RefCell<Fmla>>>, mSelf: Weak<RefCell<Gds>> }
impl Gds {
    // Java: public static final int HF = 3326;
    pub const HF: i32 = 3326;
    // Java: public static final int VF = 3760;
    pub const VF: i32 = 3760;
    // Java: public static final int ADJ = 96423;
    pub const ADJ: i32 = 96423;
    // Java: public static final int ADJ1 = 2989162;
    pub const ADJ1: i32 = 2989162;
    // Java: public static final int ADJ2 = 2989163;
    pub const ADJ2: i32 = 2989163;
    // Java: public static final int ADJ3 = 2989164;
    pub const ADJ3: i32 = 2989164;
    // Java: public static final int ADJ4 = 2989165;
    pub const ADJ4: i32 = 2989165;
    // Java: public static final int ADJ5 = 2989166;
    pub const ADJ5: i32 = 2989166;
    // Java: public static final int ADJ6 = 2989167;
    pub const ADJ6: i32 = 2989167;
    // Java: public static final int ADJ7 = 2989168;
    pub const ADJ7: i32 = 2989168;
    // Java: public static final int ADJ8 = 2989169;
    pub const ADJ8: i32 = 2989169;
    // Java: public static final int ADJ9 = 2989170;
    pub const ADJ9: i32 = 2989170;
    // Java: public static final int ADJ10 = 92664070;
    pub const ADJ10: i32 = 92664070;
    // Java: public static final int _3CD4 = 1617632;
    pub const _3CD4: i32 = 1617632;
    // Java: public static final int _3cd8 = 1617636;
    pub const _3cd8: i32 = 1617636;
    // Java: public static final int _5cd8 = 1677218;
    pub const _5cd8: i32 = 1677218;
    // Java: public static final int _7cd8 = 1736800;
    pub const _7cd8: i32 = 1736800;
    // Java: public static final int W = 119;
    pub const W: i32 = 119;
    // Java: public static final int H = 104;
    pub const H: i32 = 104;
    // Java: public static final int B = 98;
    pub const B: i32 = 98;
    // Java: public static final int L = 108;
    pub const L: i32 = 108;
    // Java: public static final int R = 114;
    pub const R: i32 = 114;
    // Java: public static final int T = 116;
    pub const T: i32 = 116;
    // Java: public static final int CD2 = 98289;
    pub const CD2: i32 = 98289;
    // Java: public static final int CD4 = 98291;
    pub const CD4: i32 = 98291;
    // Java: public static final int CD8 = 98295;
    pub const CD8: i32 = 98295;
    // Java: public static final String HC = "hc";
    pub const HC: &'static str = "hc";
    // Java: public static final String HD2 = "hd2";
    pub const HD2: &'static str = "hd2";
    // Java: public static final String HD4 = "hd4";
    pub const HD4: &'static str = "hd4";
    // Java: public static final String HD5 = "hd5";
    pub const HD5: &'static str = "hd5";
    // Java: public static final String HD6 = "hd6";
    pub const HD6: &'static str = "hd6";
    // Java: public static final String HD8 = "hd8";
    pub const HD8: &'static str = "hd8";
    // Java: public static final String LS = "ls";
    pub const LS: &'static str = "ls";
    // Java: public static final String SS = "ss";
    pub const SS: &'static str = "ss";
    // Java: public static final String SS2 = "ss2";
    pub const SS2: &'static str = "ss2";
    // Java: public static final String SS4 = "ss4";
    pub const SS4: &'static str = "ss4";
    // Java: public static final String SS6 = "ss6";
    pub const SS6: &'static str = "ss6";
    // Java: public static final String SS8 = "ss8";
    pub const SS8: &'static str = "ss8";
    // Java: public static final String VC = "vc";
    pub const VC: &'static str = "vc";
    // Java: public static final String WD2 = "wd2";
    pub const WD2: &'static str = "wd2";
    // Java: public static final String WD4 = "wd4";
    pub const WD4: &'static str = "wd4";
    // Java: public static final String WD5 = "wd5";
    pub const WD5: &'static str = "wd5";
    // Java: public static final String WD6 = "wd6";
    pub const WD6: &'static str = "wd6";
    // Java: public static final String WD8 = "wd8";
    pub const WD8: &'static str = "wd8";
    // Java: public static final String WD10 = "wd10";
    pub const WD10: &'static str = "wd10";
    // Java: public static final String HC_FMLA = "*/w 1.0 2.0";
    pub const HC_FMLA: &'static str = "*/w 1.0 2.0";
    // Java: public static final String HD2_FMLA = "*/ h 1.0 2.0";
    pub const HD2_FMLA: &'static str = "*/ h 1.0 2.0";
    // Java: public static final String HD4_FMLA = "*/ h 1.0 4.0";
    pub const HD4_FMLA: &'static str = "*/ h 1.0 4.0";
    // Java: public static final String HD5_FMLA = "*/ h 1.0 5.0";
    pub const HD5_FMLA: &'static str = "*/ h 1.0 5.0";
    // Java: public static final String HD6_FMLA = "*/ h 1.0 6.0";
    pub const HD6_FMLA: &'static str = "*/ h 1.0 6.0";
    // Java: public static final String HD8_FMLA = "*/ h 1.0 8.0";
    pub const HD8_FMLA: &'static str = "*/ h 1.0 8.0";
    // Java: public static final String LS_FMLA = "max w h";
    pub const LS_FMLA: &'static str = "max w h";
    // Java: public static final String SS_FMLA = "min w h";
    pub const SS_FMLA: &'static str = "min w h";
    // Java: public static final String SS2_FMLA = "*/ ss 1.0 2.0";
    pub const SS2_FMLA: &'static str = "*/ ss 1.0 2.0";
    // Java: public static final String SS4_FMLA = "*/ ss 1.0 4.0";
    pub const SS4_FMLA: &'static str = "*/ ss 1.0 4.0";
    // Java: public static final String SS6_FMLA = "*/ ss 1.0 6.0";
    pub const SS6_FMLA: &'static str = "*/ ss 1.0 6.0";
    // Java: public static final String SS8_FMLA = "*/ ss 1.0 8.0";
    pub const SS8_FMLA: &'static str = "*/ ss 1.0 8.0";
    // Java: public static final String VC_FMLA = "*/ h 1.0 2.0";
    pub const VC_FMLA: &'static str = "*/ h 1.0 2.0";
    // Java: public static final String WD2_FMLA = "*/ w 1.0 2.0";
    pub const WD2_FMLA: &'static str = "*/ w 1.0 2.0";
    // Java: public static final String WD4_FMLA = "*/ w 1.0 4.0";
    pub const WD4_FMLA: &'static str = "*/ w 1.0 4.0";
    // Java: public static final String WD5_FMLA = "*/ w 1.0 5.0";
    pub const WD5_FMLA: &'static str = "*/ w 1.0 5.0";
    // Java: public static final String WD6_FMLA = "*/ w 1.0 6.0";
    pub const WD6_FMLA: &'static str = "*/ w 1.0 6.0";
    // Java: public static final String WD8_FMLA = "*/ w 1.0 8.0";
    pub const WD8_FMLA: &'static str = "*/ w 1.0 8.0";
    // Java: public static final String WD10_FMLA = "*/ w 1.0 10.0";
    pub const WD10_FMLA: &'static str = "*/ w 1.0 10.0";
    // Java: public static Long getPredefinedConstant(int k, int w, int h)
    pub fn getPredefinedConstant(k: i32, w: i32, h: i32) -> Option<i64> {
        match k {
            98 | 104 => Some(h as i64),
            108 | 116 => Some(0),
            114 | 119 => Some(w as i64),
            98289 => Some(10800000), 98291 => Some(5400000), 98295 => Some(2700000),
            1617632 => Some(16200000), 1617636 => Some(8100000),
            1677218 => Some(13500000), 1736800 => Some(18900000),
            _ => None,
        }
    }

    // Java: public Gds() creates a table whose Fmla entries refer to the same table.
    pub fn new() -> GdsRef {
        let table = Rc::new_cyclic(|weak| RefCell::new(Self { mGds: HashMap::new(), mSelf: weak.clone() }));
        for (name, fmla) in [
            ("hc", "*/w 1.0 2.0"), ("hd2", "*/ h 1.0 2.0"),
            ("hd4", "*/ h 1.0 4.0"), ("hd6", "*/ h 1.0 6.0"),
            ("hd8", "*/ h 1.0 8.0"), ("ls", "max w h"),
            ("ss", "min w h"), ("ss2", "*/ ss 1.0 2.0"),
            ("ss4", "*/ ss 1.0 4.0"), ("ss6", "*/ ss 1.0 6.0"),
            ("ss8", "*/ ss 1.0 8.0"), ("vc", "*/ h 1.0 2.0"),
            ("wd2", "*/ w 1.0 2.0"), ("wd4", "*/ w 1.0 4.0"),
            ("wd5", "*/ w 1.0 5.0"), ("wd6", "*/ w 1.0 6.0"),
            ("wd8", "*/ w 1.0 8.0"), ("wd10", "*/ w 1.0 10.0"),
        ] { table.borrow_mut().add(name, fmla); }
        table
    }
    // Java: public void add(String name, String fmla)
    pub fn add(&mut self, name: &str, fmla: &str) {
        let key = java_hash(name);
        if !self.mGds.contains_key(&key) {
            let this = self.mSelf.upgrade().expect("Gds table owner");
            self.mGds.insert(key, Rc::new(RefCell::new(Fmla::new(fmla, &this))));
        }
    }
    // Java: public boolean contains(String key)
    pub fn contains(&self, key: Option<&str>) -> bool {
        key.is_some_and(|k| self.mGds.contains_key(&java_hash(k)))
    }
    // Java: public Long get(Integer k, Integer w, Integer h)
    pub fn get(&self, k: Option<i32>, w: Option<i32>, h: Option<i32>) -> Option<i64> {
        let ret = self.mGds.get(&k?)?;
        let result = ret.borrow_mut().result(w,h)?;
        // Java Math.round(double) is floor(value + 0.5), including negative half values.
        Some((result + 0.5).floor() as i64)
    }
    // Java: public Double getDouble(Integer k, Integer w, Integer h)
    pub fn getDouble(&self, k: Option<i32>, w: Option<i32>, h: Option<i32>) -> Option<f64> {
        self.mGds.get(&k?)?.borrow_mut().result(w,h)
    }
    // Java: public void gds2GeaoAdjust(GeoAdjust adjust)
    pub fn gds2GeaoAdjust(&self, adjust: &mut GeoAdjust) {
        // Java's nested null checks stop after the first absent/unevaluable adjustment.
        for (index, key) in [2989162, 2989163, 2989164, 2989165, 2989166,
            2989167, 2989168, 2989169, 2989170, 92664070].into_iter().enumerate() {
            let Some(fmla) = self.mGds.get(&key) else { break; };
            let Some(value) = fmla.borrow_mut().result(None,None) else { break; };
            adjust.setAdjust(index as i32, value as i32);
        }
    }
    // Java: public boolean isEmpty()
    pub fn isEmpty(&self) -> bool { self.mGds.is_empty() }
}
