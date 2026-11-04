// Rust stub for KmoSelection (auto-generated)
// Original: cn/wps/show/app/selection/KmoSelection.java
use crate::*;

pub struct KmoSelection {
    // TODO: fields
}

impl KmoSelection {
    pub fn new() -> Self { todo!() }
    pub fn init(&self) { todo!() }
    pub fn state(&self) -> i32 { todo!() }
    pub fn setState(&self, state: i32) { todo!() }
    pub fn slideRange(&self) -> KmoSlideRange { todo!() }
    pub fn selectedShapeRange(&self) -> KmoShapeRange { todo!() }
    pub fn shapeRange(&self) -> KmoShapeRange { todo!() }
    pub fn textRange(&self) -> KmoTextRange { todo!() }
    pub fn childShapeRange(&self) -> KmoShapeRange { todo!() }
    pub fn hasChildRange(&self) -> bool { todo!() }
    pub fn hasShapeRange(&self) -> bool { todo!() }
    pub fn activedSlideIndex(&self) -> i32 { todo!() }
    pub fn activedSlide(&self) -> KmoSlide { todo!() }
    pub fn selectNextSlide(&self) { todo!() }
    pub fn selectPrevSlide(&self) { todo!() }
    pub fn setThumbnailSlideIndex(&self, index: i32) { todo!() }
    pub fn thumbnailSlideIndex(&self) -> i32 { todo!() }
    pub fn setHistorySlideIndex(&self, index: i32) { todo!() }
    pub fn getHistorySlideIndex(&self) -> i32 { todo!() }
    pub fn setPriorityList(&self, list: ArrayList) { todo!() }
    pub fn getPriorityList(&self) -> ArrayList { todo!() }
    pub fn selectSlide(&self, slideIndex: i32) { todo!() }
    pub fn selectSlide_1(&self, slideIndex: i32, isNeedSendEvent: bool) { todo!() }
    pub fn hasSelectShape(&self, shape: KmoShowShape) -> bool { todo!() }
    pub fn selectedShape(&self) -> KmoShowShape { todo!() }
    pub fn selectShape(&self, sp: KmoShowShape, isNeedSendEvent: bool, canMultiSelect: bool) -> bool { todo!() }
    pub fn selectShape_1(&self, sp: KmoShowShape) { todo!() }
    pub fn selectShape_2(&self, sp: KmoShowShape, isNeedSendEvent: bool) { todo!() }
    pub fn canSelectChild(&self, childSp: KmoShowShape) -> bool { todo!() }
    pub fn selectChildShape(&self, childSp: KmoShowShape, isNeedSendEvent: bool, canMultiSelect: bool) -> bool { todo!() }
    pub fn selectChildShape_1(&self, childSp: KmoShowShape) { todo!() }
    pub fn selectChildShape_2(&self, childSp: KmoShowShape, isNeedSendEvent: bool) { todo!() }
    pub fn deleteClearProp(&self, kmoShowShape: KmoShowShape) -> bool { todo!() }
    pub fn restoreClearProp(&self, kmoShowShape: KmoShowShape) { todo!() }
    pub fn restoreClearPropAllExceptSelectedShape(&self, selectedShape: KmoShowShape) { todo!() }
    pub fn selectText(&self, start: i32, end: i32, isNeedSendEvent: bool) { todo!() }
    pub fn selectText_1(&self, start: i32, end: i32) { todo!() }
    pub fn selectText_2(&self, charPos: i32, caretFlag: i8, charXWidthToBegin: i32, isNeedSendEvent: bool) { todo!() }
    pub fn selectText_3(&self, charPos: i32, caretFlag: i8, charXWidthToBegin: i32) { todo!() }
    pub fn selectText_4(&self, expandmode: i8) { todo!() }
    pub fn unselect(&self) { todo!() }
    pub fn unselectText(&self) { todo!() }
    pub fn unselectShape(&self, sp: KmoShowShape) { todo!() }
    pub fn unselectShape_1(&self, sp: KmoShowShape, isNeedSendEvent: bool) { todo!() }
    pub fn canGroup(&self) -> bool { todo!() }
    pub fn canUngroup(&self) -> bool { todo!() }
    pub fn group(&self) { todo!() }
    pub fn ungroup(&self) { todo!() }
    pub fn selectShapes(&self, sps: ArrayList) { todo!() }
    pub fn cut(&self) { todo!() }
    pub fn copy(&self) { todo!() }
    pub fn paste(&self) -> i32 { todo!() }
    pub fn canCutCopy(&self) -> bool { todo!() }
    pub fn canDeleteShapes(&self) -> bool { todo!() }
    pub fn canPaste(&self) -> bool { todo!() }
    pub fn isClipContextSlide(&self) -> bool { todo!() }
    pub fn isMultiShapesSelected(&self) -> bool { todo!() }
    pub fn delete(&self, isCut: bool) { todo!() }
    pub fn delete_1(&self) { todo!() }
    pub fn canCopyFormat(&self) -> bool { todo!() }
    pub fn canPasteFormat(&self) -> bool { todo!() }
    pub fn copyForamt(&self) { todo!() }
    pub fn pasteForamt(&self) { todo!() }
    pub fn deleteSelectedShape(&self, isCut: bool) { todo!() }
    pub fn deleteCells(&self, table: KmoTable) { todo!() }
    pub fn existMergeCells(&self, table: KmoTable, cellrange: KmoRange) -> bool { todo!() }
    pub fn getTable(&self, shape: KmoShowShape) -> KmoTable { todo!() }
    pub fn isTextEditingState(&self) -> bool { todo!() }
    pub fn isShapeSelectingState(&self) -> bool { todo!() }
    pub fn isSlideSelectingState(&self) -> bool { todo!() }
    pub fn font(&self) -> KmoFont { todo!() }
    pub fn paragraphFormat(&self) -> KmoParagraphFormat { todo!() }
    pub fn mode(&self) -> i32 { todo!() }
    pub fn setMode(&self, mode: i32) { todo!() }
    pub fn unsetMode(&self, mode: i32) { todo!() }
    pub fn attach(&self, observer: IKmoSelectionObserver) { todo!() }
    pub fn notifyObservers(&self) { todo!() }
    pub fn notifyNewSlide(&self) { todo!() }
    pub fn notifyObserversBeginChange(&self) { todo!() }
    pub fn notifyIni(&self) { todo!() }
    pub fn hasHlinkClik(&self) -> bool { todo!() }
    pub fn isValidFormat(&self) -> bool { todo!() }
    pub fn getValidShapes(&self) -> List { todo!() }
    pub fn getValidShapes_1(&self, shapeList: List, shape: KmoShowShape) { todo!() }
    pub fn isValidFormat_1(&self, shapeSelected: KmoShowShape) -> bool { todo!() }
    pub fn hasSelectedVirtualGroup(&self) -> bool { todo!() }
    pub fn virtualGroupRange(&self) -> KmoShapeRange { todo!() }
    pub fn selectVirtualGroup(&self, shapes: Vec<KmoShowShape>, isNeedSendEvent: bool) -> bool { todo!() }
    pub fn unselectVirtualGroup(&self) { todo!() }
}

pub trait IKmoSelectionObserver {}
