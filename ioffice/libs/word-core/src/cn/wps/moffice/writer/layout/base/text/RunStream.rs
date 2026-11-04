#![allow(non_snake_case)]

// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{empty, PropertySet};
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;

use super::ChpxRuns::{ChpxRuns, ChpxSpan};
use super::CpText::CpText;
use super::PapxParas::{PapxParas, PapxSpan};

/// Feature wr-09 output run. Style ids travel as PropertySet refs; wr-10 resolves them.
#[derive(Clone)]
pub struct LayoutSourceRun {
    pub start_cp: i32,
    pub end_cp: i32,
    pub text: String,
    pub chpx: PropertySet,
    pub papx: PropertySet,
}

pub struct RunStream {}

impl RunStream {
    // java public Vec collectMainStoryRuns(Document doc)
    pub fn collectMainStoryRuns(doc: &dyn Document) -> Vec<LayoutSourceRun> {
        let maxCP = doc.getLength();
        let _revisionFinal = false;
        Self::collectRuns(doc, 0, maxCP)
    }

    // java public Vec collectRuns(Document doc, int start, int end)
    pub fn collectRuns(doc: &dyn Document, start: i32, end: i32) -> Vec<LayoutSourceRun> {
        if start >= end {
            return Vec::new();
        }
        let chpx_table = match doc.getChpxTable() {
            Some(p) if !p.is_null() => p,
            _ => return Vec::new(),
        };
        let papx_table = match doc.getPapxTable() {
            Some(p) if !p.is_null() => p,
            _ => return Vec::new(),
        };
        let chpxs: Vec<ChpxSpan> = unsafe { ChpxRuns::walkChpx(&*chpx_table, start, end) };
        let papxs: Vec<PapxSpan> = unsafe { PapxParas::walkPapx(&*papx_table, start, end) };
        let chpxs = if chpxs.is_empty() {
            vec![ChpxSpan {
                start_cp: start,
                end_cp: end,
                property: empty.clone(),
            }]
        } else {
            chpxs
        };
        let papxs = if papxs.is_empty() {
            vec![PapxSpan {
                start_cp: start,
                end_cp: end,
                property: empty.clone(),
            }]
        } else {
            papxs
        };
        let mut out: Vec<LayoutSourceRun> = Vec::new();
        for p in &papxs {
            for c in &chpxs {
                let a = p.start_cp.max(c.start_cp);
                let b = p.end_cp.min(c.end_cp);
                if b > a {
                    out.push(LayoutSourceRun {
                        start_cp: a,
                        end_cp: b,
                        text: CpText::getText(doc, a, b),
                        chpx: c.property.clone(),
                        papx: p.property.clone(),
                    });
                }
            }
        }
        out
    }
}
