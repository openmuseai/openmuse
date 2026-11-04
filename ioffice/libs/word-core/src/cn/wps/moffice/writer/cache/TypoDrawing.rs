#![allow(dead_code, non_snake_case)]

use std::sync::{Arc, Mutex};

use drawing::cn::wps::moffice::drawing::Shape::Shape;

use crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet;
use crate::cn::wps::moffice::util::IntBitSet::IntBitSet;
use crate::cn::wps::moffice::writer::cache::TypoList::TypoList;
use crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl;
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
use crate::cn::wps::moffice::writer::core::Document::Document;

const OFFSET_FLAGS: i32 = 7;
const OFFSET_POS: i32 = 8;
const OFFSET_SHAPE: i32 = 9;
const OFFSET_CPOFFSET: i32 = 11;
const MEM_SIZE: i32 = 31;
const TYPE_DRAWING: i32 = 7;

fn flag_drawing_type() -> IntBitSet { IntBitSet::new(0, 4) }
fn flag_embed() -> BoolBitSet { BoolBitSet::new(5) }
fn flag_wrap() -> BoolBitSet { BoolBitSet::new(6) }
fn flag_shadow() -> BoolBitSet { BoolBitSet::new(10) }
fn flag_math_zoom() -> IntBitSet { IntBitSet::new(12, 13) }
fn flag_math_align() -> IntBitSet { IntBitSet::new(14, 15) }
fn flag_rotated_with_page() -> BoolBitSet { BoolBitSet::new(18) }
fn flag_nokeep() -> BoolBitSet { BoolBitSet::new(24) }
fn flag_revision_del() -> BoolBitSet { BoolBitSet::new(26) }

// java public class TypoDrawing extends TypoObjBase
#[derive(Clone)]
pub struct TypoDrawing {
    mPointer: i32,
    mSnapshot: *mut TypoSnapshot,
}

impl Default for TypoDrawing {
    fn default() -> Self {
        Self { mPointer: 0, mSnapshot: std::ptr::null_mut() }
    }
}

impl TypoDrawing {
    pub fn new() -> Self { Self::default() }

    // java public int getType()
    pub fn getType(&self) -> i32 { TYPE_DRAWING }

    pub fn getPointer(&self) -> i32 { self.mPointer }

    // java public void assignment(int pointer, TypoSnapshot snapshot, boolean verify)
    pub fn assignment(&mut self, pointer: i32, snapshot: *mut TypoSnapshot) {
        self.mPointer = pointer;
        self.mSnapshot = snapshot;
    }

    pub fn clear(&mut self) {
        self.mPointer = 0;
        self.mSnapshot = std::ptr::null_mut();
    }

    // java public int alloc(TypoSnapshot snapshot)
    pub fn alloc(&mut self, snapshot: &mut TypoSnapshot) -> i32 {
        self.mPointer = snapshot.alloc(MEM_SIZE);
        snapshot.set__int_int(self.mPointer, TYPE_DRAWING);
        snapshot.set__int_int(self.mPointer + 1, MEM_SIZE);
        self.mSnapshot = snapshot as *mut TypoSnapshot;
        // java this.initData(); this.setZoom(1.0F);
        self.setZoom(1.0);
        self.mPointer
    }

    pub fn setParent(&mut self, parent: i32) {
        // java snapshot.set(pointer + 2, parent);
        let pointer = self.mPointer + 2;
        let snap = self.mSnapshot;
        unsafe { (*snap).set__int_int(pointer, parent); }
    }

    // java public void setTextbox(int textbox)
    pub fn setTextbox(&mut self, textbox: i32) {
        let snap = self.mSnapshot;
        if snap.is_null() { return; }
        unsafe {
            (*snap).set__int_int(self.mPointer + 22, textbox);
            if textbox != 0 {
                // java TypoObjBase.setParent(pointer, textbox, snapshot);
                (*snap).set__int_int(textbox + 2, self.mPointer);
            }
        }
    }

    // java public void setAlignLine(int baseLine)
    pub fn setAlignLine(&mut self, base_line: i32) {
        let snap = self.mSnapshot;
        if snap.is_null() { return; }
        unsafe { (*snap).set__int_int(self.mPointer + 21, base_line); }
    }

    // java public void setMathZoomType(int type)
    pub fn setMathZoomType(&mut self, zoom_type: i32) {
        let snap = self.mSnapshot;
        if snap.is_null() { return; }
        let bitset = flag_math_zoom();
        unsafe { (*snap).setIntBitSet(self.mPointer + OFFSET_FLAGS, &bitset, zoom_type); }
    }

    // java public void setAlignType(int alignType)
    pub fn setAlignType(&mut self, align_type: i32) {
        let snap = self.mSnapshot;
        if snap.is_null() { return; }
        let bitset = flag_math_align();
        unsafe { (*snap).setIntBitSet(self.mPointer + OFFSET_FLAGS, &bitset, align_type); }
    }

    // java TypoRectMemoryImpl.setLeft / setTop. Rect starts at pointer + 3.
    pub fn setLeft(&mut self, left: i32) { self.rect_set(0, left); }
    pub fn setTop(&mut self, top: i32) { self.rect_set(1, top); }

    pub fn getParent__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        snapshot.get__int(pointer + 2)
    }

    pub fn setDrawingType(&mut self, drawing_type: i32) {
        let pointer = self.mPointer + OFFSET_FLAGS;
        let bitset = flag_drawing_type();
        let snap = self.mSnapshot;
        unsafe { (*snap).setIntBitSet(pointer, &bitset, drawing_type); }
    }

    pub fn getDrawingType__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        flag_drawing_type().get(snapshot.get__int(pointer + OFFSET_FLAGS))
    }

    pub fn setEmbed(&mut self, is_embed: bool) {
        let pointer = self.mPointer + OFFSET_FLAGS;
        let bitset = flag_embed();
        let snap = self.mSnapshot;
        unsafe { (*snap).setBoolBitSet(pointer, &bitset, is_embed); }
    }

    pub fn isEmbed__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> bool {
        flag_embed().get(snapshot.get__int(pointer + OFFSET_FLAGS))
    }

    pub fn setWrap(&mut self, wrap: bool) {
        let pointer = self.mPointer + OFFSET_FLAGS;
        let bitset = flag_wrap();
        let snap = self.mSnapshot;
        if !snap.is_null() { unsafe { (*snap).setBoolBitSet(pointer, &bitset, wrap); } }
    }

    // java public static boolean isMath(int embedDrawing, TypoSnapshot snapshot)
    pub fn isMath__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> bool {
        matches!(Self::getDrawingType__int_typosnapshot(pointer, snapshot), 13 | 14 | 15)
    }

    // java public static int getAlignType(int pDrawing, TypoSnapshot snapshot)
    pub fn getAlignType__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        flag_math_align().get(snapshot.get__int(pointer + OFFSET_FLAGS))
    }

    // java public static int getAlignLine(int pDrawing, TypoSnapshot snapshot)
    pub fn getAlignLine__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        snapshot.get__int(pointer + 21)
    }

    // java public static float getZoom(int pointer, TypoSnapshot snapshot) — stored as zoom * 1000
    pub fn getZoom__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> f32 {
        snapshot.get__int(pointer + 12) as f32 / 1000.0
    }

    pub fn isRotatedWithPage(&self) -> bool {
        if self.mSnapshot.is_null() { return false; }
        flag_rotated_with_page().get(unsafe { &*self.mSnapshot }.get__int(self.mPointer + OFFSET_FLAGS))
    }

    // java public void setIsRotateWithPage(boolean val)
    pub fn setIsRotateWithPage(&mut self, value: bool) {
        let pointer = self.mPointer + OFFSET_FLAGS;
        let bitset = flag_rotated_with_page();
        let snap = self.mSnapshot;
        if !snap.is_null() {
            unsafe { (*snap).setBoolBitSet(pointer, &bitset, value); }
        }
    }

    // java public static void setIsRevisionDelEmbed(boolean value, int pointer, TypoSnapshot snapshot)
    pub fn setIsRevisionDelEmbed__boolean_int_typosnapshot(value: bool, pointer: i32, snapshot: &mut TypoSnapshot) {
        snapshot.setBoolBitSet(pointer + OFFSET_FLAGS, &flag_revision_del(), value);
    }

    // java public static void setRevisionDelId(int id, int pointer, TypoSnapshot snapshot)
    pub fn setRevisionDelId(id: i32, pointer: i32, snapshot: &mut TypoSnapshot) {
        snapshot.set__int_int(pointer + 28, id);
    }

    // java DrawingResizeTool.rotatePaddings on a padding rect stored at `base`
    pub fn rotatePadding(&mut self, base: i32, textflow: i32) {
        if textflow == 0 || self.mSnapshot.is_null() { return; }
        unsafe {
            let snapshot = &mut *self.mSnapshot;
            let left = TypoRectMemoryImpl::getLeft__int_typosnapshot(base, snapshot);
            let top = TypoRectMemoryImpl::getTop__int_typosnapshot(base, snapshot);
            let right = TypoRectMemoryImpl::getRight__int_typosnapshot(base, snapshot);
            let bottom = TypoRectMemoryImpl::getBottom__int_typosnapshot(base, snapshot);
            let (l, t, r, b) = match textflow {
                1 => (bottom, left, top, right),
                3 => (top, right, bottom, left),
                _ => return,
            };
            TypoRectMemoryImpl::set__int_int_int_int_int_typosnapshot(l, t, r, b, base, snapshot);
        }
    }

    pub fn isWrap__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> bool {
        flag_wrap().get(snapshot.get__int(pointer + OFFSET_FLAGS))
    }

    pub fn isWrap(&self) -> bool {
        if self.mSnapshot.is_null() { return false; }
        Self::isWrap__int_typosnapshot(self.mPointer, unsafe { &*self.mSnapshot })
    }

    // java public long getZIndex(); negative ints are unsigned above Integer.MAX_VALUE
    pub fn getZIndex__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i64 {
        let value = snapshot.get__int(pointer + 10);
        if value < 0 { 2147483648i64 + (value & i32::MAX) as i64 } else { value as i64 }
    }

    pub fn getZIndex(&self) -> i64 {
        if self.mSnapshot.is_null() { 0 } else { Self::getZIndex__int_typosnapshot(self.mPointer, unsafe { &*self.mSnapshot }) }
    }

    pub fn setZIndex(&mut self, z_index: i32) {
        let snap = self.mSnapshot;
        if !snap.is_null() { unsafe { (*snap).set__int_int(self.mPointer + 10, z_index); } }
    }

    pub fn setZIndex__int_int_typosnapshot(z_index: i32, pointer: i32, snapshot: &mut TypoSnapshot) {
        snapshot.set__int_int(pointer + 10, z_index);
    }

    pub fn isLayoutNotInCell__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> bool {
        BoolBitSet::new(9).get(snapshot.get__int(pointer + OFFSET_FLAGS))
    }

    pub fn isLayoutNotInCell(&self) -> bool {
        if self.mSnapshot.is_null() { false } else { Self::isLayoutNotInCell__int_typosnapshot(self.mPointer, unsafe { &*self.mSnapshot }) }
    }

    pub fn with_snapshot<R>(&self, f: impl FnOnce(&TypoSnapshot) -> R) -> Option<R> {
        if self.mSnapshot.is_null() { None } else { Some(f(unsafe { &*self.mSnapshot })) }
    }

    pub fn isTable__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> bool {
        Self::getDrawingType__int_typosnapshot(pointer, snapshot) == 7
    }

    pub fn isNoKeepCacheWhenRollBack(pointer: i32, snapshot: &TypoSnapshot) -> bool {
        flag_nokeep().get(snapshot.get__int(pointer + OFFSET_FLAGS))
    }

    pub fn setZoom(&mut self, zoom: f32) {
        let pointer = self.mPointer + 12;
        let snap = self.mSnapshot;
        unsafe { (*snap).set__int_int(pointer, (zoom * 1000.0) as i32); }
    }

    // java public int getLayoutPadding()
    pub fn getLayoutPadding(&self) -> i32 { self.mPointer + 13 }

    pub fn getRenderPadding(&self) -> i32 { self.mPointer + 17 }

    pub fn getLayoutRectPadding(p_drawing: i32) -> i32 { p_drawing + 13 }

    pub fn setCloneShape(&mut self, shape: Arc<Mutex<Shape>>) {
        let pointer = self.mPointer + OFFSET_SHAPE;
        let snap = self.mSnapshot;
        let index = unsafe { (*snap).prepareAddShape(shape) };
        unsafe { (*snap).set__int_int(pointer, index); }
    }

    pub fn getShape__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> Option<Arc<Mutex<Shape>>> {
        let index = snapshot.get__int(pointer + OFFSET_SHAPE);
        snapshot.getShapeSlot(index)
    }

    // java public static void setIntersectedWithText(boolean val, int drawing, TypoSnapshot snapshot)
    pub fn setIntersectedWithText__int_typosnapshot(val: bool, drawing: i32, snapshot: &mut TypoSnapshot) {
        let bit = BoolBitSet::new(11);
        snapshot.setBoolBitSet(drawing + OFFSET_FLAGS, &bit, val);
    }

    // java public static void removeShape(int pDrawing, TypoSnapshot snapshot)
    pub fn removeShape__int_typosnapshot(p_drawing: i32, snapshot: &mut TypoSnapshot) {
        if p_drawing == 0 { return; }
        let shape_index = snapshot.get__int(p_drawing + OFFSET_SHAPE);
        if shape_index != 0 {
            snapshot.set__int_int(p_drawing + OFFSET_SHAPE, 0);
            snapshot.clear_shape_slot(shape_index);
        }
        let children = Self::getChildDrawings__int_typosnapshot(p_drawing, snapshot);
        if children == 0 { return; }
        let size = TypoList::size__int_typosnapshot(children, snapshot);
        for index in 0..size {
            let child = TypoList::quickGet__int_int_typosnapshot(index, children, snapshot);
            if child != 0 {
                Self::removeShape__int_typosnapshot(child, snapshot);
            }
        }
    }

    // java public IPos getPos(): pos slot is 0 until TypoObjsIndexdMap.addPos; otherwise shape.getShapePos().
    pub fn getShapePosElement(&self) -> Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        if self.mSnapshot.is_null() { return None; }
        let snapshot = unsafe { &*self.mSnapshot };
        if snapshot.get__int(self.mPointer + 8) != 0 { return None; }
        let shape = Self::getShape__int_typosnapshot(self.mPointer, snapshot)?;
        let pos = {
            let mut guard = shape.lock().ok()?;
            guard.getShapePos()
        };
        pos
    }

    // Java: setPos(IPos) stores a TypoObjsIndexdMap index in slot +8.
    // The compiled table path carries the concrete TablePos returned by
    // TableInfo.getPos; other drawing IPos subclasses remain separate gates.
    pub fn setPos(&mut self, pos: crate::cn::wps::moffice::writer::data::table::TablePos::TablePos) {
        if self.mSnapshot.is_null() || self.mPointer == 0 { return; }
        let snapshot = unsafe { &mut *self.mSnapshot };
        let index = snapshot.addTablePos(pos);
        snapshot.set__int_int(self.mPointer + OFFSET_POS, index);
    }

    // Java: getPos() resolves the non-zero slot before trying Shape.getShapePos.
    pub fn getTablePos(&self) -> Option<crate::cn::wps::moffice::writer::data::table::TablePos::TablePos> {
        if self.mSnapshot.is_null() || self.mPointer == 0 { return None; }
        let snapshot = unsafe { &*self.mSnapshot };
        snapshot.getTablePos(snapshot.get__int(self.mPointer + OFFSET_POS))
    }

    pub fn setCP(&mut self, cp: i32, doc: &dyn Document) {
        let pointer = self.mPointer + OFFSET_CPOFFSET;
        let snap = self.mSnapshot;
        let r = unsafe { (*snap).addCPRef(Some(doc), cp) };
        unsafe { (*snap).set__int_int(pointer, r); }
    }

    pub fn setCP__int(p_drawing: i32, doc: &dyn Document, snapshot: &mut TypoSnapshot, cp: i32) {
        let r = snapshot.addCPRef(Some(doc), cp);
        snapshot.set__int_int(p_drawing + OFFSET_CPOFFSET, r);
    }

    pub fn getCP__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        snapshot.getCPRef(snapshot.get__int(pointer + OFFSET_CPOFFSET))
    }

    // java public int getCP()
    pub fn getCP(&self) -> i32 {
        self.with_snapshot(|snapshot| Self::getCP__int_typosnapshot(self.mPointer, snapshot)).unwrap_or(0)
    }

    // java public int getTextbox()
    pub fn getTextbox(&self) -> i32 {
        self.with_snapshot(|snapshot| Self::getTextbox__int_typosnapshot(self.mPointer, snapshot)).unwrap_or(0)
    }

    // java public boolean isShadow()
    pub fn isShadow(&self) -> bool {
        self.with_snapshot(|snapshot| flag_shadow().get(snapshot.get__int(self.mPointer + OFFSET_FLAGS))).unwrap_or(false)
    }

    // java public void setAnchorCP(int cp, Document doc)
    pub fn setAnchorCP(&mut self, cp: i32, doc: &dyn Document) {
        if self.mSnapshot.is_null() { return; }
        let snapshot = unsafe { &mut *self.mSnapshot };
        let cp_ref = snapshot.addCPRef(Some(doc), cp);
        snapshot.set__int_int(self.mPointer + 24, cp_ref);
    }

    // java public static int getAnchorCP(int drawing, TypoSnapshot snapshot)
    pub fn getAnchorCP__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        snapshot.getCPRef(snapshot.get__int(pointer + 24))
    }

    pub fn getTextbox__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        snapshot.get__int(pointer + 22)
    }

    // java public static void setTextbox(int textbox, int pointer, TypoSnapshot snapshot)
    pub fn setTextbox__int_int_typosnapshot(textbox: i32, pointer: i32, snapshot: &mut TypoSnapshot) {
        snapshot.set__int_int(pointer + 22, textbox);
        if textbox != 0 {
            snapshot.set__int_int(textbox + 2, pointer);
        }
    }

    pub fn getChildDrawings__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        snapshot.get__int(pointer + 23)
    }

    // java public int getMaxHeight()
    pub fn getMaxHeight(&self) -> i32 {
        if self.mSnapshot.is_null() {
            return 0;
        }
        let snapshot = unsafe { &mut *self.mSnapshot };
        let p_childs = TypoDrawing::getChildDrawings__int_typosnapshot(self.mPointer, snapshot);
        if p_childs == 0 {
            return 0;
        }
        let count = TypoList::size__int_typosnapshot(p_childs, snapshot);
        let mut max_height = 0;
        for i in 0..count {
            let child = TypoList::quickGet__int_int_typosnapshot(i, p_childs, snapshot);
            let h = TypoDrawing::layoutHeight__int_typosnapshot(child, snapshot);
            if h > max_height {
                max_height = h;
            }
        }
        max_height
    }

    // java public int getMaxWidth()
    pub fn getMaxWidth(&self) -> i32 {
        if self.mSnapshot.is_null() {
            return 0;
        }
        let snapshot = unsafe { &mut *self.mSnapshot };
        let p_childs = TypoDrawing::getChildDrawings__int_typosnapshot(self.mPointer, snapshot);
        if p_childs == 0 {
            return 0;
        }
        let count = TypoList::size__int_typosnapshot(p_childs, snapshot);
        let mut max_width = 0;
        for i in 0..count {
            let child = TypoList::quickGet__int_int_typosnapshot(i, p_childs, snapshot);
            let w = TypoDrawing::layoutWidth__int_typosnapshot(child, snapshot);
            if w > max_width {
                max_width = w;
            }
        }
        max_width
    }

    pub fn getChildDrawings(&self) -> i32 {
        if self.mSnapshot.is_null() {
            0
        } else {
            unsafe { (*self.mSnapshot).get__int(self.mPointer + 23) }
        }
    }

    // java public void addChildDrawing(int subDrawing)
    pub fn addChildDrawing(&mut self, sub_drawing: i32) {
        let snap = self.mSnapshot;
        if snap.is_null() {
            return;
        }
        unsafe {
            let snapshot = &mut *snap;
            let p_old = snapshot.get__int(self.mPointer + 23);
            let p_list = TypoList::add__int_int_typosnapshot(sub_drawing, p_old, snapshot);
            // java TypoObjBase.setParent(parent, subDrawing, snapshot)
            snapshot.set__int_int(sub_drawing + 2, self.mPointer);
            if p_list != p_old {
                snapshot.set__int_int(self.mPointer + 23, p_list);
            }
        }
    }

    // java public static void setAlignLine(int baseLine, int pDrawing, TypoSnapshot snapshot)
    pub fn setAlignLine__int_int_typosnapshot(base_line: i32, pointer: i32, snapshot: &mut TypoSnapshot) {
        snapshot.set__int_int(pointer + 21, base_line);
    }

    // java TypoObjBase.set via TypoRectMemoryImpl at pointer + 3
    pub fn set(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        let pointer = self.mPointer + 3;
        let snap = self.mSnapshot;
        unsafe {
            TypoRectMemoryImpl::set__int_int_int_int_int_typosnapshot(left, top, right, bottom, pointer, &mut *snap);
        }
    }

    pub fn width(&self) -> i32 {
        if self.mSnapshot.is_null() { 0 } else { Self::width__int(self.mPointer, unsafe { &*self.mSnapshot }) }
    }

    pub fn height(&self) -> i32 {
        if self.mSnapshot.is_null() { 0 } else { Self::height__int(self.mPointer, unsafe { &*self.mSnapshot }) }
    }

    pub fn width__int(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        TypoRectMemoryImpl::width__int_typosnapshot(pointer + 3, snapshot)
    }

    pub fn height__int(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        TypoRectMemoryImpl::height__int_typosnapshot(pointer + 3, snapshot)
    }

    // java public static int layoutWidth(int pDrawing, TypoSnapshot snapshot)
    // DrawingResizeTool.getExpandedWidth(drawing, layoutPadding)
    pub fn layoutWidth__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        let width = Self::width__int(pointer, snapshot);
        let pad = pointer + 13;
        width
            + TypoRectMemoryImpl::getLeft__int_typosnapshot(pad, snapshot)
            + TypoRectMemoryImpl::getRight__int_typosnapshot(pad, snapshot)
    }

    // java public static int layoutHeight(int pDrawing, TypoSnapshot snapshot)
    // DrawingResizeTool.getExpandedHeight(drawing, layoutPadding)
    pub fn layoutHeight__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
        let height = Self::height__int(pointer, snapshot);
        let pad = pointer + 13;
        height
            + TypoRectMemoryImpl::getTop__int_typosnapshot(pad, snapshot)
            + TypoRectMemoryImpl::getBottom__int_typosnapshot(pad, snapshot)
    }

    pub fn isEmbed(&self) -> bool {
        if self.mSnapshot.is_null() { false } else { Self::isEmbed__int_typosnapshot(self.mPointer, unsafe { &*self.mSnapshot }) }
    }

    pub fn getLeft(&self) -> i32 { self.rect_get(0) }
    pub fn getTop(&self) -> i32 { self.rect_get(1) }
    pub fn getRight(&self) -> i32 { self.rect_get(2) }
    pub fn getBottom(&self) -> i32 { self.rect_get(3) }

    fn rect_get(&self, offset: i32) -> i32 {
        if self.mSnapshot.is_null() { 0 } else { unsafe { &*self.mSnapshot }.get__int(self.mPointer + 3 + offset) }
    }

    // java TypoObjBase.setWidth → TypoRectMemoryImpl.setRight(left + width)
    pub fn setWidth(&mut self, width: i32) {
        let right = self.getLeft() + width;
        self.rect_set(2, right);
    }

    pub fn setHeight(&mut self, height: i32) {
        let bottom = self.getTop() + height;
        self.rect_set(3, bottom);
    }

    pub fn setEmpty(&mut self) { self.set(0, 0, 0, 0); }

    // java TypoObjBase.offset
    pub fn offset(&mut self, dx: i32, dy: i32) {
        self.offsetTo(self.getLeft().wrapping_add(dx), self.getTop().wrapping_add(dy));
    }

    // java public void setWrapTableOverFlow(boolean value) — FLAG_WrapTableOverFlow bit 23
    pub fn setWrapTableOverFlow(&mut self, value: bool) {
        let snap = self.mSnapshot;
        if snap.is_null() { return; }
        let bit = BoolBitSet::new(23);
        unsafe { (*snap).setBoolBitSet(self.mPointer + OFFSET_FLAGS, &bit, value); }
    }

    // java TypoRectMemoryImpl.offsetTo
    pub fn offsetTo(&mut self, x: i32, y: i32) {
        let dx = x - self.getLeft();
        let dy = y - self.getTop();
        let base = self.mPointer + 3;
        let snap = self.mSnapshot;
        if snap.is_null() { return; }
        unsafe {
            (*snap).extend__int_int(base, dx);
            (*snap).extend__int_int(base + 2, dx);
            (*snap).extend__int_int(base + 1, dy);
            (*snap).extend__int_int(base + 3, dy);
        }
    }

    fn rect_set(&mut self, offset: i32, value: i32) {
        let pointer = self.mPointer + 3 + offset;
        let snap = self.mSnapshot;
        if !snap.is_null() { unsafe { (*snap).set__int_int(pointer, value); } }
    }

    // java DrawingResizeTool.calcPaddings into render padding, then layout padding copies it for embeds.
    pub fn setRenderPadding(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.write_rect(self.getRenderPadding(), left, top, right, bottom);
    }

    pub fn setLayoutPadding(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.write_rect(self.getLayoutPadding(), left, top, right, bottom);
    }

    pub fn render_padding_tuple(&self) -> (i32, i32, i32, i32) {
        if self.mSnapshot.is_null() { return (0, 0, 0, 0); }
        let snapshot = unsafe { &*self.mSnapshot };
        let pad = self.getRenderPadding();
        (
            TypoRectMemoryImpl::getLeft__int_typosnapshot(pad, snapshot),
            TypoRectMemoryImpl::getTop__int_typosnapshot(pad, snapshot),
            TypoRectMemoryImpl::getRight__int_typosnapshot(pad, snapshot),
            TypoRectMemoryImpl::getBottom__int_typosnapshot(pad, snapshot),
        )
    }

    pub fn shape_arc(&self) -> Option<Arc<Mutex<Shape>>> {
        if self.mSnapshot.is_null() { return None; }
        Self::getShape__int_typosnapshot(self.mPointer, unsafe { &*self.mSnapshot })
    }

    pub fn copyRenderPaddingToLayout(&mut self) {
        let src = self.getRenderPadding();
        let dst = self.getLayoutPadding();
        let snap = self.mSnapshot;
        if snap.is_null() { return; }
        unsafe {
            let snapshot = &mut *snap;
            let left = TypoRectMemoryImpl::getLeft__int_typosnapshot(src, snapshot);
            let top = TypoRectMemoryImpl::getTop__int_typosnapshot(src, snapshot);
            let right = TypoRectMemoryImpl::getRight__int_typosnapshot(src, snapshot);
            let bottom = TypoRectMemoryImpl::getBottom__int_typosnapshot(src, snapshot);
            TypoRectMemoryImpl::set__int_int_int_int_int_typosnapshot(left, top, right, bottom, dst, snapshot);
        }
    }

    pub fn setLayoutPaddingEdge(&mut self, which: i32, value: i32) {
        let pointer = self.getLayoutPadding() + which;
        let snap = self.mSnapshot;
        if !snap.is_null() { unsafe { (*snap).set__int_int(pointer, value); } }
    }

    // java public void layoutRectForPageCore(ITypoRect rect)
    pub fn layoutRectForPageCore(&self, out: &mut crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl) {
        if self.mSnapshot.is_null() || self.mPointer == 0 {
            out.set__int_int_int_int(0, 0, 0, 0);
            return;
        }
        let snapshot = unsafe { &*self.mSnapshot };
        // java doGetRectForPageCore(layoutPaddings, rect)
        if self.is_embed_with_text_line_anchor(snapshot) {
            self.do_get_embed_rect_for_page_core(snapshot, true, out);
            return;
        }
        let page = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getPage__int_typosnapshot(self.mPointer, snapshot);
        // java doGetRectForPage(paddings, pPage, false, outRect)
        self.do_get_rect_for_page(snapshot, page, false, true, out);
        // java if (0 != pPage) page.rectForPageToRectForPageCore(outRect);
        if page != 0 {
            let mut lines_page = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::default();
            lines_page.snapshot = self.mSnapshot;
            lines_page.snapshot_pointer = page;
            lines_page.rectForPageToRectForPageCore(out);
        }
    }

    // java return this.isEmbed() && TypoMemoryObj.getType(this.getParent(), snapshot) == 3;
    fn is_embed_with_text_line_anchor(&self, snapshot: &TypoSnapshot) -> bool {
        let parent = Self::getParent__int_typosnapshot(self.mPointer, snapshot);
        self.isEmbed() && crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj::getType__int_typosnapshot(parent, snapshot) == 3
    }

    // java doGetEmbedRectForPageCore
    fn do_get_embed_rect_for_page_core(
        &self,
        _snapshot: &TypoSnapshot,
        use_padding: bool,
        out: &mut crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl,
    ) {
        let snap = self.mSnapshot;
        let parent = unsafe { Self::getParent__int_typosnapshot(self.mPointer, &*snap) };
        let p_para = unsafe {
            crate::cn::wps::moffice::writer::cache::TypoTextLine::TypoTextLine::getPara__int_typosnapshot(parent, &*snap)
        };
        // java anchorLine.getRelTextRectTop() == getTextRectTop() + getTop()
        let line_rel_top = unsafe {
            (*snap).get__int(parent + crate::cn::wps::moffice::writer::cache::TypoTextLine::OFFSET_TEXT_RECT_TOP)
                + crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getTop__int_typosnapshot(parent, &*snap)
        };
        let mut drawing_left = 0;
        let mut drawing_top = 0;
        let mut drawing_right = 0;
        let mut drawing_bottom = 0;
        let mut texts = crate::cn::wps::moffice::writer::cache::TypoTextsOpIterator::TypoTextsOpIterator::newInstance();
        texts.init(parent, snap);
        while let Some(typo_text) = texts.next() {
            let embed = unsafe {
                crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara::getEmbed__int_int_typosnapshot(
                    typo_text.embedIndex,
                    p_para,
                    &mut *snap,
                )
            };
            // java if (typoText.runType == 6 && typoPara.getEmbed(typoText.embedIndex) == this.mPointer)
            if typo_text.runType == 6 && embed == self.mPointer {
                let p_padding = self.getLayoutPadding();
                let begin = if typo_text.hasBeginBdr() { typo_text.getBdrWidth() } else { 0 };
                let left = typo_text.textLeft
                    + unsafe { TypoRectMemoryImpl::getLeft__int_typosnapshot(p_padding, &*snap) }
                    + begin;
                let max_ascent = typo_text.typoFontStyle.as_ref().map(|style| style.maxAscent).unwrap_or(0);
                let top = typo_text.baseline
                    - max_ascent
                    + unsafe { TypoRectMemoryImpl::getTop__int_typosnapshot(p_padding, &*snap) }
                    + typo_text.getBdrWidth();
                drawing_left = left;
                drawing_top = top;
                drawing_right = left + self.width();
                drawing_bottom = top + self.height();
                break;
            }
        }
        // java DrawingResizeTool.expandRect(drawingRect, paddings, outRect)
        if use_padding {
            let pad = self.getLayoutPadding();
            unsafe {
                let snapshot = &*snap;
                out.set__int_int_int_int(
                    drawing_left - TypoRectMemoryImpl::getLeft__int_typosnapshot(pad, snapshot),
                    drawing_top - TypoRectMemoryImpl::getTop__int_typosnapshot(pad, snapshot),
                    drawing_right + TypoRectMemoryImpl::getRight__int_typosnapshot(pad, snapshot),
                    drawing_bottom + TypoRectMemoryImpl::getBottom__int_typosnapshot(pad, snapshot),
                );
            }
        } else {
            out.set__int_int_int_int(drawing_left, drawing_top, drawing_right, drawing_bottom);
        }
        // java outRect.offset(0, lineRelTop);
        out.offset(0, line_rel_top);
    }

    // java doGetRectForPage(paddings, pDrawingPage, needCheckIsEmbed, outRect)
    fn do_get_rect_for_page(
        &self,
        snapshot: &TypoSnapshot,
        p_drawing_page: i32,
        need_check_embed: bool,
        use_padding: bool,
        out: &mut crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl,
    ) {
        if need_check_embed && self.is_embed_with_text_line_anchor(snapshot) {
            self.do_get_embed_rect_for_page_core(snapshot, use_padding, out);
            let page = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getPage__int_typosnapshot(self.mPointer, snapshot);
            if page != 0 {
                let mut coord = crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool::CoordRect {
                    left: out.getLeft(),
                    top: out.getTop(),
                    right: out.getRight(),
                    bottom: out.getBottom(),
                };
                crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool::calcRectForPage(page, page, &mut coord, snapshot);
                out.set__int_int_int_int(coord.left, coord.top, coord.right, coord.bottom);
            }
            return;
        }
        // java outRect.setEmpty(); if (pParent == 0) return;
        out.set__int_int_int_int(0, 0, 0, 0);
        let parent = Self::getParent__int_typosnapshot(self.mPointer, snapshot);
        if parent == 0 {
            return;
        }
        // java DrawingResizeTool.expandRect(this, paddings, outRect); null paddings copies the drawing rect.
        if use_padding {
            let pad = self.getLayoutPadding();
            out.set__int_int_int_int(
                self.getLeft() - TypoRectMemoryImpl::getLeft__int_typosnapshot(pad, snapshot),
                self.getTop() - TypoRectMemoryImpl::getTop__int_typosnapshot(pad, snapshot),
                self.getRight() + TypoRectMemoryImpl::getRight__int_typosnapshot(pad, snapshot),
                self.getBottom() + TypoRectMemoryImpl::getBottom__int_typosnapshot(pad, snapshot),
            );
        } else {
            out.set__int_int_int_int(self.getLeft(), self.getTop(), self.getRight(), self.getBottom());
        }
        // java if (getType(pParent) == 7) offset by the parent drawing's page rect (null paddings, check embed)
        if crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj::getType__int_typosnapshot(parent, snapshot) == 7 {
            let mut parent_drawing = TypoDrawing::new();
            parent_drawing.assignment(parent, self.mSnapshot);
            let mut parent_rect = crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl::default();
            parent_drawing.do_get_rect_for_page(snapshot, p_drawing_page, true, false, &mut parent_rect);
            out.offset(parent_rect.getLeft(), parent_rect.getTop());
        } else {
            // java DrawingCoordTool.offsetMarginLeftAndTop(drawingPage, outRect);
            let mut page = p_drawing_page;
            if page == 0 {
                page = crate::cn::wps::moffice::writer::cache::TypoObjBase::TypoObjBase::getPage__int_typosnapshot(self.mPointer, snapshot);
            }
            if page != 0 {
                let textflow = crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage::getVisualTextflow__int_typosnapshot(page, snapshot);
                let ml = crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool::getMarginLeft(page, textflow, snapshot);
                let mt = crate::cn::wps::moffice::writer::layout::base::drawing::DrawingCoordTool::getMarginTop(page, textflow, snapshot);
                out.offset(ml, mt);
            }
        }
    }

    fn write_rect(&mut self, pointer: i32, left: i32, top: i32, right: i32, bottom: i32) {
        let snap = self.mSnapshot;
        if snap.is_null() { return; }
        unsafe { TypoRectMemoryImpl::set__int_int_int_int_int_typosnapshot(left, top, right, bottom, pointer, &mut *snap); }
    }
}
