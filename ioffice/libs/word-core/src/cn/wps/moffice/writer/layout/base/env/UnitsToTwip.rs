#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::layout::text::FontStack::{
    units_to_twip, units_to_twip_from_size_twip, FontError,
};

/// wr-11 I/O: font units + size + upem → twip (same formula as FontStack hmtx).
pub struct UnitsToTwip {}

impl UnitsToTwip {
    // java round(units * sizePt * 20 / upem)
    pub fn unitsToTwip(units: f64, sizePt: f64, upem: f64) -> Result<i32, FontError> {
        units_to_twip(units, sizePt, upem)
    }

    // java round(units * sizeTwip / upem)  // sizeTwip = sizePt * 20
    pub fn unitsToTwipFromSizeTwip(units: f64, sizeTwip: i32, upem: f64) -> Result<i32, FontError> {
        units_to_twip_from_size_twip(units, sizeTwip, upem)
    }
}
