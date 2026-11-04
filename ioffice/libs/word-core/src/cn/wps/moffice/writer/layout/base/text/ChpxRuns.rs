#![allow(non_snake_case)]

// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{empty, PropertySet};
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{Entry as IChpxEntry, IChpxTable};

/// One CHPX-constant half-open cp range.
#[derive(Clone)]
pub struct ChpxSpan {
    pub start_cp: i32,
    pub end_cp: i32,
    pub property: PropertySet,
}

pub struct ChpxRuns {}

impl ChpxRuns {
    // java public Vec walkChpx(IChpxTable table, int start, int end)
    pub fn walkChpx(table: &dyn IChpxTable, start: i32, end: i32) -> Vec<ChpxSpan> {
        let mut out: Vec<ChpxSpan> = Vec::new();
        if start >= end {
            return out;
        }
        let mut run = IChpxTable::seek(table, start);
        while let Some(e) = run {
            if e.isEnd() {
                break;
            }
            if e.position() >= end {
                break;
            }
            if !e.isHead() {
                let a = e.position().max(start);
                let b = e.endCp().min(end);
                if b > a {
                    out.push(ChpxSpan {
                        start_cp: a,
                        end_cp: b,
                        property: IChpxEntry::getProperty(e.as_ref())
                            .cloned()
                            .unwrap_or_else(|| empty.clone()),
                    });
                }
            }
            run = IChpxEntry::getNext(e.as_ref());
        }
        out
    }
}
