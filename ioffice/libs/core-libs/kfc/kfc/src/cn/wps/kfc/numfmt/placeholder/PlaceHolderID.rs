#![allow(
    non_snake_case,
    non_camel_case_types,
    unused_variables,
    unused_imports,
    dead_code,
    unreachable_code
)]

use std::sync::{Arc, Mutex, OnceLock};

use super::PlaceHolder::PlaceHolder;

// Java field: short PHOLDER_INVALID
pub const PHOLDER_INVALID: i16 = -1;
// Java field: short PHOLDER_PLAIN
pub const PHOLDER_PLAIN: i16 = 0;
// Java field: short PHOLDER_GENERAL
pub const PHOLDER_GENERAL: i16 = 1;
// Java field: short PHOLDER_SHARP
pub const PHOLDER_SHARP: i16 = 2;
// Java field: short PHOLDER_ZERO
pub const PHOLDER_ZERO: i16 = 3;
// Java field: short PHOLDER_ASK
pub const PHOLDER_ASK: i16 = 4;
// Java field: short PHOLDER_FRACTION
pub const PHOLDER_FRACTION: i16 = 5;
// Java field: short PHOLDER_PERCENT
pub const PHOLDER_PERCENT: i16 = 6;
// Java field: short PHOLDER_SCIENCE
pub const PHOLDER_SCIENCE: i16 = 7;
// Java field: short PHOLDER_THOUS_SEP
pub const PHOLDER_THOUS_SEP: i16 = 8;
// Java field: short PHOLDER_DECI_POINT
pub const PHOLDER_DECI_POINT: i16 = 9;
// Java field: short PHOLDER_FILL_CHAR
pub const PHOLDER_FILL_CHAR: i16 = 10;
// Java field: short PHOLDER_UNDERLINE
pub const PHOLDER_UNDERLINE: i16 = 11;
// Java field: short PHOLDER_TEXT
pub const PHOLDER_TEXT: i16 = 12;
// Java field: short PHOLDER_DOUBLE_QUATATION
pub const PHOLDER_DOUBLE_QUATATION: i16 = 13;
// Java field: short PHOLDER_SINGLE_QUATATION
pub const PHOLDER_SINGLE_QUATATION: i16 = 14;
// Java field: short PHOLDER_PLAIN_DIGIT
pub const PHOLDER_PLAIN_DIGIT: i16 = 15;
// Java field: short PHOLDER_COLOR
pub const PHOLDER_COLOR: i16 = 16;
// Java field: short PHOLDER_CONDITION
pub const PHOLDER_CONDITION: i16 = 17;
// Java field: short PHOLDER_NCL
pub const PHOLDER_NCL: i16 = 18;
// Java field: short PHOLDER_DBNUM
pub const PHOLDER_DBNUM: i16 = 19;
// Java field: short PHOLDER_g_YEAR
pub const PHOLDER_g_YEAR: i16 = 20;
// Java field: short PHOLDER_e_YEAR
pub const PHOLDER_e_YEAR: i16 = 21;
// Java field: short PHOLDER_b_YEAR
pub const PHOLDER_b_YEAR: i16 = 22;
// Java field: short PHOLDER_y_YEAR
pub const PHOLDER_y_YEAR: i16 = 23;
// Java field: short PHOLDER_m_MONTH
pub const PHOLDER_m_MONTH: i16 = 24;
// Java field: short PHOLDER_d_DAY
pub const PHOLDER_d_DAY: i16 = 25;
// Java field: short PHOLDER_h_HOUR
pub const PHOLDER_h_HOUR: i16 = 26;
// Java field: short PHOLDER_m_MIN
pub const PHOLDER_m_MIN: i16 = 27;
// Java field: short PHOLDER_s_SEC
pub const PHOLDER_s_SEC: i16 = 28;
// Java field: short PHOLDER_h_ELAPSE
pub const PHOLDER_h_ELAPSE: i16 = 29;
// Java field: short PHOLDER_m_ELAPSE
pub const PHOLDER_m_ELAPSE: i16 = 30;
// Java field: short PHOLDER_s_ELAPSE
pub const PHOLDER_s_ELAPSE: i16 = 31;
// Java field: short PHOLDER_d_WEEK
pub const PHOLDER_d_WEEK: i16 = 32;
// Java field: short PHOLDER_a_WEEK
pub const PHOLDER_a_WEEK: i16 = 33;
// Java field: short PHOLDER_AM_PM
pub const PHOLDER_AM_PM: i16 = 34;
// Java field: short PHOLDER_COLON
pub const PHOLDER_COLON: i16 = 35;
// Java field: short PHOLDER_TH_DAY
pub const PHOLDER_TH_DAY: i16 = 36;
// Java field: short PHOLDER_TH_MONTH
pub const PHOLDER_TH_MONTH: i16 = 37;
// Java field: short PHOLDER_TH_YEAR
pub const PHOLDER_TH_YEAR: i16 = 38;
// Java field: short PHOLDER_TH_HOUR
pub const PHOLDER_TH_HOUR: i16 = 39;
// Java field: short PHOLDER_TH_MINUTE
pub const PHOLDER_TH_MINUTE: i16 = 40;
// Java field: short PHOLDER_TH_SECOND
pub const PHOLDER_TH_SECOND: i16 = 41;
// Java field: short PHOLDER_W_X
pub const PHOLDER_W_X: i16 = 42;
// Java field: short PHOLDER_W_SIGN
pub const PHOLDER_W_SIGN: i16 = 43;
// Java field: short PHOLDER_W_APOSTROPHE
pub const PHOLDER_W_APOSTROPHE: i16 = 44;
// Java field: short PHOLDER_W_O_MONTH
pub const PHOLDER_W_O_MONTH: i16 = 45;
// Java field: short PHOLDER_W_A_DAY
pub const PHOLDER_W_A_DAY: i16 = 46;
// Java field: short PHOLDER_W_D_DAY
pub const PHOLDER_W_D_DAY: i16 = 47;
// Java field: short PHOLDER_W_G_ERA
pub const PHOLDER_W_G_ERA: i16 = 48;
// Java field: short PHOLDER_W_R
pub const PHOLDER_W_R: i16 = 49;
// Java field: short PHOLDER_W_M
pub const PHOLDER_W_M: i16 = 50;
// Java field: short PHOLDER_W_Y
pub const PHOLDER_W_Y: i16 = 51;
// Java field: short PHOLDER_W_E_YEAR
pub const PHOLDER_W_E_YEAR: i16 = 52;
// Java field: short PHOLDER_W_W_WEEK
pub const PHOLDER_W_W_WEEK: i16 = 53;
// Java field: short PHOLDER_W_AMPM
pub const PHOLDER_W_AMPM: i16 = 54;
// Java field: short PHOLDER_W_AP
pub const PHOLDER_W_AP: i16 = 55;
// Java field: short PHOLDER_W_B_YEAR
pub const PHOLDER_W_B_YEAR: i16 = 56;
// Java field: short PHOLDER_COUNT
pub const PHOLDER_COUNT: i16 = 57;

// Java field: Class<? extends PlaceHolder>[] mHolderClasses
// Java field: PlaceHolder DEFAULT_HOLDER
// Java field: LocalPHolderContainer mHolderInstances

use super::PlaceHolder::*;
use super::PlaceHolder::{Holder, HolderRef};
// java public class PlaceHolderID
pub struct PlaceHolderID;
// java private static final LocalPHolderContainer mHolderInstances=new LocalPHolderContainer();
thread_local! {static INSTANCES:std::cell::RefCell<PlaceHolderContainer>=std::cell::RefCell::new(PlaceHolderContainer::new());}
static DEFAULT: std::sync::LazyLock<HolderRef> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(Box::new(PlaceHolderPlainText::new()))));
impl PlaceHolderID {
    pub fn new() -> Self {
        Self
    }
    // java return ((PlaceHolderContainer)mHolderInstances.get()).getPlaceHolder(id);
    pub fn getPlaceHolder(id: i16) -> HolderRef {
        INSTANCES.with(|instances| instances.borrow_mut().getPlaceHolder(id))
    }
}
// java private static class PlaceHolderContainer
pub struct PlaceHolderContainer {
    mHolderInstances: Vec<Option<HolderRef>>,
}
impl PlaceHolderContainer {
    pub fn new() -> Self {
        Self {
            mHolderInstances: vec![None; 57],
        }
    }
    pub fn getPlaceHolder(&mut self, id: i16) -> HolderRef {
        if id < 0 || id >= 57 {
            return DEFAULT.clone();
        }
        if let Some(holder) = &self.mHolderInstances[id as usize] {
            return holder.clone();
        }
        // java newInstance of the original concrete holder class, cached in this thread.
        let holder: Box<dyn Holder> = match id {
            0 => Box::new(PlaceHolderPlainText::new()),
            1 => Box::new(PlaceHolderGeneral::new()),
            2 => Box::new(PlaceHolderSharp::new()),
            3 => Box::new(PlaceHolderZero::new()),
            4 => Box::new(PlaceHolderAsk::new()),
            5 => Box::new(PlaceHolderFraction::new()),
            6 => Box::new(PlaceHolderPercent::new()),
            7 => Box::new(PlaceHolderScience::new()),
            8 => Box::new(PlaceHolderThousSep::new()),
            9 => Box::new(PlaceHolderDeciPoint::new()),
            10 => Box::new(PlaceHolderFillChar::new()),
            11 => Box::new(PlaceHolderUnderline::new()),
            12 => Box::new(PlaceHolderText::new()),
            13 => Box::new(PlaceHolderDoubleQuatation::new()),
            14 => Box::new(PlaceHolderSingleQuatation::new()),
            15 => Box::new(PlaceHolderPlainDigit::new()),
            16 => Box::new(PlaceHolderColor::new()),
            17 => Box::new(PlaceHolderCondition::new()),
            18 => Box::new(PlaceHolderNCL::new()),
            19 => Box::new(PlaceHolderDBNum::new()),
            35 => Box::new(PlaceHolderColon::new()),
            20 => Box::new(super::PlaceHolderDate::PlaceHolderGYear::new()),
            21 => Box::new(super::PlaceHolderDate::PlaceHolderEYear::new()),
            22 => Box::new(super::PlaceHolderDate::PlaceHolderBYear::new()),
            23 => Box::new(super::PlaceHolderDate::PlaceHolderYYear::new()),
            24 => Box::new(super::PlaceHolderDate::PlaceHolderMonth::new()),
            25 => Box::new(super::PlaceHolderDate::PlaceHolderDay::new()),
            26 => Box::new(super::PlaceHolderTime::PlaceHolderHour::new()),
            27 => Box::new(super::PlaceHolderTime::PlaceHolderMin::new()),
            28 => Box::new(super::PlaceHolderTime::PlaceHolderSec::new()),
            29 => Box::new(super::PlaceHolderTime::PlaceHolderElapseHour::new()),
            30 => Box::new(super::PlaceHolderTime::PlaceHolderElapseMin::new()),
            31 => Box::new(super::PlaceHolderTime::PlaceHolderElapseSec::new()),
            32 => Box::new(super::PlaceHolderDate::PlaceHolderDWeek::new()),
            33 => Box::new(super::PlaceHolderDate::PlaceHolderAWeek::new()),
            34 => Box::new(super::PlaceHolderTime::PlaceHolderAmPm::new()),
            36 => Box::new(super::PlaceHolderDate::PlaceHolderThaiDay::new()),
            37 => Box::new(super::PlaceHolderDate::PlaceHolderThaiMonth::new()),
            38 => Box::new(super::PlaceHolderDate::PlaceHolderThaiYear::new()),
            39 => Box::new(super::PlaceHolderTime::PlaceHolderThaiHour::new()),
            40 => Box::new(super::PlaceHolderTime::PlaceHolderThaiMinute::new()),
            41 => Box::new(super::PlaceHolderTime::PlaceHolderThaiSecond::new()),
            _ => panic!("DEFERRED_NON_FUNCTION: PlaceHolderID Word {id}"),
        };
        let holder = Arc::new(Mutex::new(holder));
        self.mHolderInstances[id as usize] = Some(holder.clone());
        holder
    }
}
