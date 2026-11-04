#![allow(non_snake_case)]

// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{empty, PropertySet};
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{Entry as IPapxEntry, IPapxTable};

/// One PAPX-constant half-open paragraph range.
#[derive(Clone)]
pub struct PapxSpan {
    pub start_cp: i32,
    pub end_cp: i32,
    pub property: PropertySet,
}

pub struct PapxParas {}

impl PapxParas {
    // java public Vec walkPapx(IPapxTable table, int start, int end)
    pub fn walkPapx(table: &dyn IPapxTable, start: i32, end: i32) -> Vec<PapxSpan> {
        let mut out: Vec<PapxSpan> = Vec::new();
        if start >= end {
            return out;
        }
        let mut para = IPapxTable::seek(table, start);
        while let Some(e) = para {
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
                    out.push(PapxSpan {
                        start_cp: a,
                        end_cp: b,
                        property: IPapxEntry::getProperty(e.as_ref())
                            .cloned()
                            .unwrap_or_else(|| empty.clone()),
                    });
                }
            }
            para = IPapxEntry::getNext(e.as_ref());
        }
        out
    }
}
