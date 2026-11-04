// Java: import java.util.HashMap
// Java: import cn.wps.moffice.writer.io.reader.dmlReader.context.Branch
use crate::cn::wps::moffice::writer::io::reader::dmlReader::context::Branch::*;
use std::collections::{HashMap};

// java public static final int STYLELST_SIZE = 3;
pub(crate) const STYLELST_SIZE: i32 = 3;

// java public class ThemeContext
pub struct ThemeContext {
    // java public HashMap<Integer, Integer> mClrScheme = new HashMap();
    pub(crate) mClrScheme: HashMap<i32, i32>,
    // java public Branch[] mLnStyleLst = new Branch[3];
    pub(crate) mLnStyleLst: [Option<Branch>; 3],
    // java public Branch[] mFillStyleLst = new Branch[3];
    pub(crate) mFillStyleLst: [Option<Branch>; 3],
    // java public Branch[] mEffectStyleLst = new Branch[3];
    pub(crate) mEffectStyleLst: [Option<Branch>; 3],
    // java public Branch[] mBgFillStyleLst = new Branch[3];
    pub(crate) mBgFillStyleLst: [Option<Branch>; 3],
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self {
            // Java: new HashMap()
            mClrScheme: HashMap::new(),
            // Java: new Branch[3] (three null references per array)
            mLnStyleLst: std::array::from_fn(|_| None),
            mFillStyleLst: std::array::from_fn(|_| None),
            mEffectStyleLst: std::array::from_fn(|_| None),
            mBgFillStyleLst: std::array::from_fn(|_| None),
        }
    }
}

// java public class ThemeContext
impl ThemeContext {
}
