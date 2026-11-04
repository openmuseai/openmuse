//! Stage1 PAPX/CHPX dump. No Java type `PropDump`.
//!
//! Reads `IPapxTable.seek` / `IChpxTable.seek` via existing walkers.
//! Missing keys are `None` (`contains` == false), never default 0.

use serde::{Deserialize, Serialize};
use util::moffice::property::PropertySet::PropertySet;

use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::layout::base::text::ChpxRuns::ChpxRuns;
use crate::cn::wps::moffice::writer::layout::base::text::PapxParas::PapxParas;

const PAPX_IDS: [i32; 9] = [191, 193, 197, 208, 210, 212, 214, 217, 220];

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Stage1Papx {
    pub start_cp: i32,
    pub end_cp: i32,
    #[serde(rename = "191")]
    pub p191: Option<i32>,
    #[serde(rename = "193")]
    pub p193: Option<i32>,
    #[serde(rename = "197")]
    pub p197: Option<bool>,
    #[serde(rename = "208")]
    pub p208: Option<i32>,
    #[serde(rename = "210")]
    pub p210: Option<i32>,
    #[serde(rename = "212")]
    pub p212: Option<i32>,
    #[serde(rename = "214")]
    pub p214: Option<i32>,
    #[serde(rename = "217")]
    pub p217: Option<i32>,
    #[serde(rename = "220")]
    pub p220: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Stage1Chpx {
    pub start_cp: i32,
    pub end_cp: i32,
    /// Property 10 (`run_fontSize`) as twip when present (pt × 20).
    pub sz_twip: Option<i32>,
    /// Property 5 (`run_bold`).
    pub bold: Option<bool>,
    /// Property 4 (`run_fontAsian`).
    pub east_asia: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Stage1Props {
    pub papx: Vec<Stage1Papx>,
    pub chpx: Vec<Stage1Chpx>,
}

pub struct PropDump {}

fn opt_int(ps: &PropertySet, id: i32) -> Option<i32> {
    if ps.contains(id) {
        Some(ps.getInt(id))
    } else {
        None
    }
}

fn opt_bool(ps: &PropertySet, id: i32) -> Option<bool> {
    if ps.contains(id) {
        Some(ps.getBoolean(id))
    } else {
        None
    }
}

fn dump_papx(ps: &PropertySet, start_cp: i32, end_cp: i32) -> Stage1Papx {
    let _ = PAPX_IDS;
    Stage1Papx {
        start_cp,
        end_cp,
        p191: opt_int(ps, 191),
        p193: opt_int(ps, 193),
        p197: opt_bool(ps, 197),
        p208: opt_int(ps, 208),
        p210: opt_int(ps, 210),
        p212: opt_int(ps, 212),
        p214: opt_int(ps, 214),
        p217: opt_int(ps, 217),
        p220: if ps.contains(220) { Some(true) } else { None },
    }
}

fn dump_chpx(ps: &PropertySet, start_cp: i32, end_cp: i32) -> Stage1Chpx {
    let sz_twip = if ps.contains(10) {
        let pt = ps.getFloat__int_float(10, 0.0);
        Some((pt * 20.0 + 0.5) as i32)
    } else {
        None
    };
    let east_asia = if ps.contains(4) {
        Some(ps.getString(4))
    } else {
        None
    };
    Stage1Chpx {
        start_cp,
        end_cp,
        sz_twip,
        bold: opt_bool(ps, 5),
        east_asia,
    }
}

impl PropDump {
    /// Java: seek PAPX/CHPX over `[0, length)` and export listed sprms with `contains`.
    pub fn dump(doc: &TextDocument) -> Stage1Props {
        let Some(main) = doc.getMainDocument() else {
            return Stage1Props::default();
        };
        let len = main.getLength();
        let papx = main
            .getPapxTable()
            .map(|t| {
                PapxParas::walkPapx(t, 0, len)
                    .into_iter()
                    .map(|s| dump_papx(&s.property, s.start_cp, s.end_cp))
                    .collect()
            })
            .unwrap_or_default();
        let chpx = main
            .getChpxTable()
            .map(|t| {
                ChpxRuns::walkChpx(t, 0, len)
                    .into_iter()
                    .map(|s| dump_chpx(&s.property, s.start_cp, s.end_cp))
                    .collect()
            })
            .unwrap_or_default();
        Stage1Props { papx, chpx }
    }
}
