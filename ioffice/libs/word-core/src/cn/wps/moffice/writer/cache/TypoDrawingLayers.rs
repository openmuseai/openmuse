#![allow(dead_code, non_snake_case)]

use std::sync::{Arc, Mutex};

use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::ShapePos::ShapePos;

use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase;
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use crate::cn::wps::moffice::writer::core::Document::Document;

const OFFSET_DRAWINGS: i32 = 2;
const OFFSET_TOP: i32 = 3;
const OFFSET_MIDDLE: i32 = 4;
const OFFSET_BOTTOM: i32 = 5;
const OFFSET_WRAP: i32 = 6;
const OFFSET_DOCSET: i32 = 7;
const MEM_SIZE: i32 = 8;
const TYPE_LAYERS: i32 = 16;

// java public class TypoDrawingLayers extends TypoMemoryObj
pub struct TypoDrawingLayers {
    mPointer: i32,
    mSnapshot: *mut TypoSnapshot,
    /// Java reads this from snapshot.getDocument().getCompatibilityMode().
    pub before_2013: bool,
}

impl Default for TypoDrawingLayers {
    fn default() -> Self {
        Self { mPointer: 0, mSnapshot: std::ptr::null_mut(), before_2013: true }
    }
}

impl TypoDrawingLayers {
    // java public static TypoDrawingLayers newInstance()
    pub fn newInstance() -> Self { Self::default() }

    pub fn getPointer(&self) -> i32 { self.mPointer }

    // Java keeps the same MainState and snapshot object through nested cell
    // layout. The Rust subclass adapter moves the snapshot value, so retain
    // the compact layer pointer while rebinding its heap reference.
    pub fn rebind_snapshot(&mut self, snapshot: &mut TypoSnapshot) {
        if self.mPointer != 0 {
            self.mSnapshot = snapshot as *mut TypoSnapshot;
        }
    }

    // java TypoMemoryObj.clear
    pub fn clear(&mut self) {
        self.mSnapshot = std::ptr::null_mut();
        self.mPointer = 0;
    }

    // java public final boolean isValid()
    pub fn isValid(&self) -> bool { self.mPointer != 0 }

    // java public int alloc(TypoSnapshot snapshot)
    pub fn alloc(&mut self, snapshot: &mut TypoSnapshot) -> i32 {
        self.mPointer = snapshot.alloc(MEM_SIZE);
        snapshot.set__int_int(self.mPointer, TYPE_LAYERS);
        snapshot.set__int_int(self.mPointer + 1, MEM_SIZE);
        self.mSnapshot = snapshot as *mut TypoSnapshot;
        self.mPointer
    }

    fn snap(&self) -> Option<&TypoSnapshot> {
        if self.mSnapshot.is_null() { None } else { Some(unsafe { &*self.mSnapshot }) }
    }

    fn snap_mut(&mut self) -> Option<&mut TypoSnapshot> {
        if self.mSnapshot.is_null() { None } else { Some(unsafe { &mut *self.mSnapshot }) }
    }

    // java public int getDrawins()
    pub fn getDrawins(&self) -> i32 {
        self.snap().map(|s| s.get__int(self.mPointer + OFFSET_DRAWINGS)).unwrap_or(0)
    }

    pub fn getTops(&self) -> i32 {
        self.snap().map(|s| s.get__int(self.mPointer + OFFSET_TOP)).unwrap_or(0)
    }

    pub fn getMiddles(&self) -> i32 {
        self.snap().map(|s| s.get__int(self.mPointer + OFFSET_MIDDLE)).unwrap_or(0)
    }

    pub fn getBottoms(&self) -> i32 {
        self.snap().map(|s| s.get__int(self.mPointer + OFFSET_BOTTOM)).unwrap_or(0)
    }

    pub fn getWraps(&self) -> i32 {
        self.snap().map(|s| s.get__int(self.mPointer + OFFSET_WRAP)).unwrap_or(0)
    }

    // java public int getWraps(WIntArrayList floats, int parentPage)
    pub fn getWraps__list_int(&self, floats: &mut Vec<i32>, parent_page: i32) -> i32 {
        // java floats.resetQuick();
        floats.clear();
        let wraps = self.getWraps();
        if std::env::var_os("WORD_PARITY_WRAP_TRACE").is_some() {
            eprintln!("wrap_layers layers={} wraps={} parent={}", self.mPointer, wraps, parent_page);
        }
        if wraps != 0 {
            let Some(snapshot) = self.snap() else { return 0; };
            // java for each wrap whose parent page is parentPage
            for i in 0..TypoList::size__int_typosnapshot(wraps, snapshot) {
                let drawing = snapshot.get__int(wraps + 3 + i);
                if std::env::var_os("WORD_PARITY_WRAP_TRACE").is_some() {
                    eprintln!("wrap_layers item={} parent={} expected={}", drawing, TypoDrawing::getParent__int_typosnapshot(drawing, snapshot), parent_page);
                }
                if TypoDrawing::getParent__int_typosnapshot(drawing, snapshot) == parent_page {
                    floats.push(drawing);
                }
            }
        }
        floats.len() as i32
    }

    // java public int getFloats(WIntArrayList floats, int parentPage)
    pub fn getFloats__list_int(&self, floats: &mut Vec<i32>, parent_page: i32) -> i32 {
        // java floats.resetQuick(); int wraps = this.getDrawins();
        floats.clear();
        let drawings = self.getDrawins();
        if drawings != 0 {
            if let Some(snapshot) = self.snap() {
                // java include only drawings whose parent is this cell page.
                for i in 0..TypoList::size__int_typosnapshot(drawings, snapshot) {
                    let drawing = snapshot.get__int(drawings + 3 + i);
                    if TypoDrawing::getParent__int_typosnapshot(drawing, snapshot) == parent_page {
                        floats.push(drawing);
                    }
                }
            }
        }
        floats.len() as i32
    }

    // java public int getWrapTableByCP(int startCP, int parentPage)
    pub fn getWrapTableByCP(&self, startCP: i32, parentPage: i32) -> i32 {
        let Some(snapshot) = self.snap() else { return 0; };
        Self::getWrapTableByCP__int_int_int_typosnapshot(startCP, parentPage, self.mPointer, snapshot)
    }

    // java public static int getWrapTableByCP(int startCP, int parentPage, int pDrawingLayers, TypoSnapshot snapshot)
    pub fn getWrapTableByCP__int_int_int_typosnapshot(startCP: i32, parentPage: i32, pDrawingLayers: i32, snapshot: &TypoSnapshot) -> i32 {
        // java int pWrapList = getWraps(pDrawingLayers, snapshot);
        let p_wrap_list = snapshot.get__int(pDrawingLayers + OFFSET_WRAP);
        if p_wrap_list == 0 { return 0; }
        // java for (int i = 0, size = TypoList.size(pWrapList, snapshot); i < size; ++i)
        for i in 0..TypoList::size__int_typosnapshot(p_wrap_list, snapshot) {
            // java int pItem = TypoList.quickGet(i, pWrapList, snapshot);
            let p_item = snapshot.get__int(p_wrap_list + 3 + i);
            // java TypoDrawing.getCP(pItem, snapshot) == startCP
            if TypoDrawing::getCP__int_typosnapshot(p_item, snapshot) == startCP
                // java parentPage == 0 || parentPage == TypoObjBase.getParent(pItem, snapshot)
                && (parentPage == 0 || parentPage == TypoDrawing::getParent__int_typosnapshot(p_item, snapshot))
            {
                return p_item;
            }
        }
        0
    }

    // java public int getFloat(int startCP, int parentPage)
    pub fn getFloat(&self, start_cp: i32, parent_page: i32) -> i32 {
        let Some(snapshot) = self.snap() else { return 0; };
        let wraps = self.getDrawins();
        if wraps == 0 { return 0; }
        let size = TypoList::size__int_typosnapshot(wraps, snapshot);
        for i in 0..size {
            let drawing = snapshot.get__int(wraps + 3 + i);
            if TypoDrawing::getParent__int_typosnapshot(drawing, snapshot) == parent_page
                && TypoDrawing::getCP__int_typosnapshot(drawing, snapshot) == start_cp
            {
                return drawing;
            }
        }
        0
    }

    // java public boolean hasDocTag(int docType)
    pub fn hasDocTag(&self, doc_type: i32) -> bool {
        let Some(snapshot) = self.snap() else { return false; };
        if self.mPointer == 0 { return false; }
        let doc_set = snapshot.get__int(self.mPointer + OFFSET_DOCSET);
        let mask = 1i32 << doc_type;
        (doc_set & mask) != 0
    }

    fn addDocTag(&mut self, doc: &dyn Document) {
        let pointer = self.mPointer;
        let Some(snapshot) = self.snap_mut() else { return; };
        let doc_set = snapshot.get__int(pointer + OFFSET_DOCSET);
        let mask = 1i32 << doc.getType__on_IDocument();
        if (doc_set & mask) == 0 {
            snapshot.set__int_int(pointer + OFFSET_DOCSET, doc_set | mask);
        }
    }

    // java public void addToList(int listOfffset, int item)
    pub fn addToList(&mut self, list_offset: i32, item: i32) {
        let pointer = self.mPointer;
        let Some(snapshot) = self.snap_mut() else { return; };
        let mut p_old = snapshot.get__int(pointer + list_offset);
        if p_old == 0 {
            p_old = TypoList::alloc__typosnapshot_int(snapshot, 4);
            snapshot.set__int_int(pointer + list_offset, p_old);
        }
        let line_pos = TypoList::size__int_typosnapshot(p_old, snapshot);
        let p_new = TypoList::insert__int_int_int_typosnapshot(item, line_pos, p_old, snapshot);
        if p_old != p_new {
            snapshot.set__int_int(pointer + list_offset, p_new);
        }
    }

    // java public void insertItem(int listOfffset, int pointer, int linePos)
    pub fn insertItem(&mut self, list_offset: i32, item: i32, line_pos: i32) {
        let base = self.mPointer;
        let Some(snapshot) = self.snap_mut() else { return; };
        let mut p_old = snapshot.get__int(base + list_offset);
        if p_old == 0 {
            p_old = TypoList::alloc__typosnapshot_int(snapshot, 4);
            snapshot.set__int_int(base + list_offset, p_old);
        }
        let p_new = TypoList::insert__int_int_int_typosnapshot(item, line_pos, p_old, snapshot);
        if p_old != p_new {
            snapshot.set__int_int(base + list_offset, p_new);
        }
    }

    fn offset_for_layer(layer: i32) -> i32 {
        match layer {
            1 => OFFSET_MIDDLE,
            2 => OFFSET_BOTTOM,
            _ => OFFSET_TOP,
        }
    }

    fn shape_z_index(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        let Some(shape) = TypoDrawing::getShape__int_typosnapshot(pointer, snapshot) else { return 0; };
        shape.lock().map(|guard| guard.getZIndex()).unwrap_or(0)
    }

    fn is_top_drawing(drawing: &TypoDrawing) -> bool {
        if drawing.isWrap() { return true; }
        if drawing.isLayoutNotInCell() {
            if let Some(pos) = drawing.getShapePosElement() {
                if let Some(shape_pos) = pos.downcast_ref::<ShapePos>() {
                    if matches!(shape_pos.getTextWrap(), 0 | 1 | 2 | 4 | 5) { return true; }
                }
            }
        }
        let Some(shape) = drawing.shape_arc() else { return true; };
        let Ok(mut guard) = shape.lock() else { return true; };
        let Some(pos) = guard.getShapePos() else { return true; };
        let Some(shape_pos) = pos.downcast_ref::<ShapePos>() else { return true; };
        let text_wrap = shape_pos.getTextWrap();
        if text_wrap == 1 { return true; }
        text_wrap == 3 && !guard.getBehindDocument()
    }

    fn drawing_layer(drawing: &TypoDrawing) -> i32 {
        let is_table_or_frame = drawing.with_snapshot(|snapshot| {
            let ty = TypoDrawing::getDrawingType__int_typosnapshot(drawing.getPointer(), snapshot);
            ty == 7 || ty == 8
        }).unwrap_or(false);
        if is_table_or_frame { 1 } else if Self::is_top_drawing(drawing) { 0 } else { 2 }
    }

    fn add_drawing_by_z_index(&mut self, pointer: i32, is_wrap: bool, z_index: i64, list_offset: i32) {
        let p_list = self.snap().map(|s| s.get__int(self.mPointer + list_offset)).unwrap_or(0);
        if p_list == 0 {
            self.addToList(list_offset, pointer);
            return;
        }
        let is_compat = self.before_2013;
        let mut insert_pos = -1i32;
        let size = self.snap().map(|s| TypoList::size__int_typosnapshot(p_list, s)).unwrap_or(0);
        for i in 0..size {
            let p_drawing = self.snap().map(|s| s.get__int(p_list + 3 + i)).unwrap_or(0);
            let cur_z = self.snap().map(|s| TypoDrawing::getZIndex__int_typosnapshot(p_drawing, s)).unwrap_or(0);
            let cur_wrap = self.snap().map(|s| TypoDrawing::isWrap__int_typosnapshot(p_drawing, s)).unwrap_or(false);
            if is_compat {
                if is_wrap && (!cur_wrap || z_index < cur_z) {
                    insert_pos = i;
                    break;
                }
                if !is_wrap && !cur_wrap && z_index < cur_z {
                    insert_pos = i;
                    break;
                }
            } else if z_index < cur_z {
                insert_pos = i;
                break;
            }
        }
        if insert_pos == -1 { insert_pos = size; }
        self.insertItem(list_offset, pointer, insert_pos);
    }

    // java private void removeDrawingFromList(int pDrawing, int listOfffset)
    fn remove_from_list(&mut self, p_drawing: i32, list_offset: i32) {
        let base = self.mPointer;
        let Some(snapshot) = self.snap_mut() else { return; };
        let p_list = snapshot.get__int(base + list_offset);
        if p_list == 0 { return; }
        let index = TypoList::indexOf__int_int_typosnapshot(p_drawing, p_list, snapshot);
        if index >= 0 {
            TypoList::remove__int_int_typosnapshot(index, p_list, snapshot);
        }
    }

    // java public void removeDrawing(int pDrawing)
    pub fn removeDrawing(&mut self, p_drawing: i32) {
        self.remove_from_list(p_drawing, OFFSET_DRAWINGS);
        let mut drawing = TypoDrawing::new();
        if let Some(snapshot) = self.snap_mut() {
            drawing.assignment(p_drawing, snapshot);
        }
        let layer = Self::drawing_layer(&drawing);
        self.remove_from_list(p_drawing, Self::offset_for_layer(layer));
        if drawing.isWrap() {
            self.remove_from_list(p_drawing, OFFSET_WRAP);
        }
    }

    // java public void removeDrawinghsForParent(int parentPage)
    pub fn removeDrawinghsForParent(&mut self, parent_page: i32) {
        if parent_page == 0 { return; }
        let mut found: Vec<i32> = Vec::new();
        if let Some(snapshot) = self.snap() {
            let drawings = snapshot.get__int(self.mPointer + OFFSET_DRAWINGS);
            let size = TypoList::size__int_typosnapshot(drawings, snapshot);
            for i in 0..size {
                let drawing = snapshot.get__int(drawings + 3 + i);
                if TypoObjBase::seekParent__int_int_typosnapshot(drawing, parent_page, snapshot) {
                    found.push(drawing);
                }
            }
        }
        for drawing in found {
            self.removeDrawing(drawing);
        }
    }

    // java public void appendDrawingObject(int pDrawing, Document doc)
    pub fn appendDrawingObject(&mut self, p_drawing: i32, doc: Option<&dyn Document>) {
        if let Some(document) = doc {
            self.addDocTag(document);
        }
        self.addToList(OFFSET_DRAWINGS, p_drawing);
        let z = self.snap().map(|s| Self::shape_z_index(p_drawing, s)).unwrap_or(0);
        let (is_wrap, layer, z_index) = {
            let mut drawing = TypoDrawing::new();
            if let Some(snapshot) = self.snap_mut() {
                drawing = snapshot.getTypoTools().obtainTypoDrawing__int(p_drawing);
            }
        drawing.setZIndex(z);
        let z_index = drawing.getZIndex();
        let layer = Self::drawing_layer(&drawing);
        let is_wrap = drawing.isWrap();
        if let Some(snapshot) = self.snap_mut() {
            snapshot.getTypoTools().recycle_drawing(&mut drawing);
        }
        (is_wrap, layer, z_index)
    };
    self.add_drawing_by_z_index(p_drawing, is_wrap, z_index, Self::offset_for_layer(layer));
    if is_wrap {
        self.add_drawing_by_z_index(p_drawing, true, z_index, OFFSET_WRAP);
    }
    }
}
