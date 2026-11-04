#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_assignments, unused_mut, unused_imports)]

// Java: import java.util.List

use std::cell::RefCell;
use std::any::Any;

// Java field: ThreadLocal<ReColorMap> sThreadMap
thread_local! {
    static sThreadMap: RefCell<Option<ReColorMap>> = RefCell::new(None);
}

// Java: public interface RecolorEntry
// Java field: int SOURCE_COLOR
pub const SOURCE_COLOR: i32 = 0;
// Java field: int SOURCE_BLUSH
pub const SOURCE_BLUSH: i32 = 1;
pub trait RecolorEntry: Any {
    // Java: abstract int getSourceType()
    fn getSourceType(&self) -> i32;
    // Java: abstract int getToColor()
    fn getToColor(&self) -> i32;
    fn as_any(&self) -> &dyn Any;
    fn as_recolor_entry_color(&self) -> Option<&dyn RecolorEntryColor> { None }
    fn as_recolor_entry_blush(&self) -> Option<&dyn RecolorEntryBlush> { None }
}

// Java: public interface RecolorEntryBlush extends RecolorEntry
pub trait RecolorEntryBlush: RecolorEntry {
    // Java: abstract int getLbStyle()
    fn getLbStyle(&self) -> i32;
    // Java: abstract int getLbColor()
    fn getLbColor(&self) -> i32;
    // Java: abstract int getLbHatch()
    fn getLbHatch(&self) -> i32;
    // Java: abstract int getFgColor()
    fn getFgColor(&self) -> i32;
    // Java: abstract int getBgColor()
    fn getBgColor(&self) -> i32;
    // Java: abstract int getBitmapType()
    fn getBitmapType(&self) -> i32;
    // Java: abstract byte[] getPattern()
    fn getPattern(&self) -> Vec<u8>;
}

// Java: public interface RecolorEntryColor extends RecolorEntry
pub trait RecolorEntryColor: RecolorEntry {
    // Java: abstract int getFromColor()
    fn getFromColor(&self) -> i32;
}

// Java: public class ReColorMap
pub struct ReColorMap {
    // Java field: List<RecolorEntry> recolorEntryMap
    pub(crate) recolorEntryMap: Option<Vec<Box<dyn RecolorEntry>>>,
}

// Java: public class ReColorMap
impl ReColorMap {
    // Java: ReColorMap()
    pub fn new() -> Self {
        // Java: super();
        ReColorMap { recolorEntryMap: None }
    }

    // Java: static ReColorMap getReColorMap(boolean create)
    fn getReColorMap(create: bool) -> Option<ReColorMap> {
        // Java: ReColorMap value = (ReColorMap)sThreadMap.get();
        sThreadMap.with(|cell| {
            let mut opt = cell.borrow_mut();
            // Java: if (value == null && create)
            if opt.is_none() && create {
                // Java: value = new ReColorMap();
                // Java: sThreadMap.set(value);
                *opt = Some(ReColorMap::new());
            }
            // Java: return value;
            opt.take()
        })
    }

    fn put_back(value: ReColorMap) {
        sThreadMap.with(|cell| {
            *cell.borrow_mut() = Some(value);
        });
    }

    // Java: static void setReColorMap(List<RecolorEntry> map)
    pub fn setReColorMap(map: Option<Vec<Box<dyn RecolorEntry>>>) {
        // Java: ReColorMap value = getReColorMap(map != null);
        let value = Self::getReColorMap(map.is_some());
        // Java: if (value != null)
        if let Some(mut value) = value {
            // Java: value.recolorEntryMap = map;
            value.recolorEntryMap = map;
            Self::put_back(value);
        }
    }

    // Java: static void clearReColorMap()
    pub fn clearReColorMap() {
        // Java: ReColorMap value = getReColorMap(false);
        let value = Self::getReColorMap(false);
        // Java: if (value != null)
        if let Some(mut value) = value {
            // Java: value.recolorEntryMap = null;
            value.recolorEntryMap = None;
            Self::put_back(value);
        }
    }

    // Java: static int getReColor(int fromColor)
    pub fn getReColor(fromColor: i32) -> i32 {
        // Java: int toColor = fromColor;
        let mut toColor = fromColor;
        // Java: ReColorMap value = getReColorMap(false);
        let value = Self::getReColorMap(false);
        // Java: if (value != null && value.recolorEntryMap != null)
        if let Some(ref value) = value {
            if let Some(ref entries) = value.recolorEntryMap {
                // Java: for(int i = 0; i < value.recolorEntryMap.size(); ++i)
                for i in 0..entries.len() {
                    // Java: RecolorEntry entry = (RecolorEntry)value.recolorEntryMap.get(i);
                    let entry = &entries[i];
                    // Java: if (entry.getSourceType() == 0 && entry instanceof RecolorEntryColor && ((RecolorEntryColor)entry).getFromColor() == fromColor)
                    if entry.getSourceType() == 0 {
                        if let Some(color_entry) = entry.as_recolor_entry_color() {
                            if color_entry.getFromColor() == fromColor {
                                // Java: toColor = entry.getToColor();
                                toColor = entry.getToColor();
                                // Java: break;
                                break;
                            }
                        }
                    }
                }
            }
        }
        if let Some(v) = value { Self::put_back(v); }
        // Java: return toColor;
        toColor
    }

    // Java: static int getReColorFill(int fromColor, int style)
    pub fn getReColorFill(fromColor: i32, style: i32) -> i32 {
        // Java: int toColor = fromColor;
        let mut toColor = fromColor;
        // Java: if (style != 0)
        if style != 0 {
            // Java: return fromColor;
            return fromColor;
        }
        // Java: else
        else {
            // Java: ReColorMap value = getReColorMap(false);
            let value = Self::getReColorMap(false);
            // Java: if (value != null && value.recolorEntryMap != null)
            if let Some(ref value) = value {
                if let Some(ref entries) = value.recolorEntryMap {
                    // Java: boolean findBlush = false;
                    let mut findBlush = false;
                    // Java: for(int i = 0; i < value.recolorEntryMap.size(); ++i)
                    for i in 0..entries.len() {
                        // Java: RecolorEntry entry = (RecolorEntry)value.recolorEntryMap.get(i);
                        let entry = &entries[i];
                        // Java: if (entry.getSourceType() == 1 && entry instanceof RecolorEntryBlush)
                        if entry.getSourceType() == 1 {
                            // Java: RecolorEntryBlush blush = (RecolorEntryBlush)entry;
                            if let Some(blush) = entry.as_recolor_entry_blush() {
                                // Java: if (blush.getLbStyle() == style && blush.getLbColor() == fromColor)
                                if blush.getLbStyle() == style && blush.getLbColor() == fromColor {
                                    // Java: toColor = entry.getToColor();
                                    toColor = entry.getToColor();
                                    // Java: findBlush = true;
                                    findBlush = true;
                                    // Java: break;
                                    break;
                                }
                            }
                        }
                    }
                    // Java: if (!findBlush)
                    if !findBlush {
                        // Java: toColor = getReColor(fromColor);
                        toColor = fromColor;
                    }
                }
            }
            if let Some(v) = value { Self::put_back(v); }
            // Java: return toColor;
            return toColor;
        }
    }
}
