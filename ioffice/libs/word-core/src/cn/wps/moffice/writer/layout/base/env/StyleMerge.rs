#![allow(non_snake_case)]

// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{empty, PropertySet};
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::KStyles;

/// Layout adapter: rPrDefault ∪ pStyle basedOn ∪ rStyle ∪ direct CHPX.
pub struct StyleMerge {}

impl StyleMerge {
    // java public static PropertySet merge(PropertySet chpx, PropertySet papx, KStyles styles)
    pub fn merge(chpx: &PropertySet, papx: &PropertySet, styles: Option<&KStyles>) -> PropertySet {
        let Some(styles) = styles else {
            return chpx.clone();
        };
        let mut acc = styles.getRPrDefault();
        let pStyleId = papx.getInt__int_int(191, -1);
        if pStyleId >= 0 {
            acc = PropertySet::merge(&acc, &Self::styleFinal(styles, pStyleId));
        }
        let rStyleId = chpx.getInt__int_int(2, -1);
        if rStyleId >= 0 {
            acc = PropertySet::merge(&acc, &Self::styleFinal(styles, rStyleId));
        }
        PropertySet::merge(&acc, chpx)
    }

    fn styleFinal(styles: &KStyles, startId: i32) -> PropertySet {
        let mut ids: Vec<i32> = Vec::new();
        let mut cur = startId;
        for _ in 0..32 {
            if ids.contains(&cur) {
                break;
            }
            let Some(st) = styles.getStyleById__int_boolean(cur, false) else {
                break;
            };
            ids.push(cur);
            let base = st.getBase();
            if base == 4095 || base == cur {
                break;
            }
            cur = base;
        }
        ids.reverse();
        let mut acc = empty.clone();
        for id in ids {
            if let Some(st) = styles.getStyleById__int_boolean(id, false) {
                if let Some(prop) = st.getProperty() {
                    acc = PropertySet::merge(&acc, &prop);
                }
            }
        }
        acc
    }
}
