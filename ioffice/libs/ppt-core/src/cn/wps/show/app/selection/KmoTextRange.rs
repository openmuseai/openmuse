// Rust stub for KmoTextRange (auto-generated)
// Original: cn/wps/show/app/selection/KmoTextRange.java
use crate::*;

pub struct KmoTextRange {
    // TODO: fields
}

impl KmoTextRange {
    pub fn new() -> Self { todo!() }
    pub fn activedShape(&self) -> KmoShowShape { todo!() }
    pub fn caret(&self) -> KmoCaret { todo!() }
    pub fn putCaret(&self, caret: KmoCaret) { todo!() }
    pub fn hasCaret(&self) -> bool { todo!() }
    pub fn start(&self) -> i32 { todo!() }
    pub fn putStart(&self, start: i32) { todo!() }
    pub fn end(&self) -> i32 { todo!() }
    pub fn putEnd(&self, end: i32) { todo!() }
    pub fn count(&self) -> i32 { todo!() }
    pub fn row(&self) -> i32 { todo!() }
    pub fn col(&self) -> i32 { todo!() }
    pub fn select(&self, start: i32, end: i32) { todo!() }
    pub fn selectText(&self, expandmode: i8) -> IndexRange { todo!() }
    pub fn cut(&self) -> KmoCopiedText { todo!() }
    pub fn copy(&self) -> KmoCopiedText { todo!() }
    pub fn paste(&self, copiedText: KmoCopiedText) -> bool { todo!() }
    pub fn insert(&self, textInsert: String) -> bool { todo!() }
    pub fn insert_1(&self, textInsert: String, inputLocal: String) -> bool { todo!() }
    pub fn insertBefore(&self, textInsert: String) -> bool { todo!() }
    pub fn insertAfter(&self, textInsert: String) -> bool { todo!() }
    pub fn insertDateTime(&self, r#type: i32) -> bool { todo!() }
    pub fn insertSlideNumber(&self) -> bool { todo!() }
    pub fn insertSymbol(&self, fontName: String, charNumber: i64) -> bool { todo!() }
    pub fn replace(&self, start: i32, end: i32, text: String) -> bool { todo!() }
    pub fn replace_1(&self, start: i32, end: i32, text: String, inputLocal: String) -> bool { todo!() }
    pub fn delete(&self) -> bool { todo!() }
    pub fn typeBackspace(&self) -> bool { todo!() }
    pub fn typeForwardDel(&self) -> bool { todo!() }
    pub fn font(&self) -> KmoFont { todo!() }
    pub fn paragraphFormat(&self) -> KmoParagraphFormat { todo!() }
    pub fn textOperator(&self) -> KmoTextOperator { todo!() }
    pub fn setTemporaryProperties(&self, runprop: TextCharacterProperties) { todo!() }
    pub fn getTemporaryProperties(&self) -> TextCharacterProperties { todo!() }
    pub fn getPresentation(&self) -> KmoPresentation { todo!() }
    pub fn insert_2(&self, cp: i32, textInsert: String) -> bool { todo!() }
    pub fn insert_3(&self, cp: i32, textInsert: String, inputLocal: String) -> bool { todo!() }
    pub fn hasArabicChar(&self, text: String) -> bool { todo!() }
    pub fn applyMatchTextCharacterProperties(&self, textOp: KmoTextOperator, properties: TextCharacterProperties, textInsert: String) -> TextCharacterProperties { todo!() }
    pub fn getFontNameInScheme(&self, theme: KmoTheme, phType: i32) -> String { todo!() }
    pub fn getSchemeFontName(textFont: TextFont) -> String { todo!() }
    pub fn createTypeface(&self, fontName: String) -> Box<dyn ITypeface> { todo!() }
    pub fn delete_1(&self, start: i32, end: i32) -> bool { todo!() }
    pub fn changerange(&self, start: i32, end: i32, caretFlag: i8) { todo!() }
    pub fn notifyTextSelectionChanged(&self) { todo!() }
    pub fn isTable(&self, shape: KmoShowShape) -> bool { todo!() }
    pub fn isTable_1(&self, textOp: KmoTextOperator) -> bool { todo!() }
    pub fn hasHlinkClik(&self) -> bool { todo!() }
    pub fn hyperlink(&self) -> Hyperlink { todo!() }
    pub fn hyperlinkMouseOver(&self) -> Hyperlink { todo!() }
    pub fn hasHlinkMouseOver(&self) -> bool { todo!() }
    pub fn findSelectedHyperlinkRun(&self) -> TextCharacterProperties { todo!() }
    pub fn findSelectedHyperlinkMouseOverRun(&self) -> TextCharacterProperties { todo!() }
    pub fn setHlinkClicked(&self) { todo!() }
    pub fn getSelectedHyperLinkRange(&self) -> IndexRange { todo!() }
    pub fn parsePrevHlink(&self, textOp: KmoTextOperator, start: i32, action: String, target: String, hlinkRange: IndexRange) { todo!() }
    pub fn parseAfterHlink(&self, textOp: KmoTextOperator, start: i32, action: String, target: String, hlinkRange: IndexRange) { todo!() }
    pub fn setPrevCloseHlinkClick(&self, textOp: KmoTextOperator, start: i32, action: String, target: String) { todo!() }
    pub fn setAfterCloseHlinkClick(&self, textOp: KmoTextOperator, start: i32, action: String, target: String) { todo!() }
    pub fn deleteHyperlink(&self, start: i32, end: i32) { todo!() }
    pub fn setHlink(&self, text: String, hyperlink: Hyperlink) -> bool { todo!() }
    pub fn setHlink_1(&self, start: i32, end: i32, text: String, hyperlink: Hyperlink) -> bool { todo!() }
    pub fn checkHlink(&self) -> KmoHyperlink { todo!() }
    pub fn makeUrl(url: String, prefixes: Vec<String>) -> String { todo!() }
    pub fn getSlide(&self) -> KmoSlide { todo!() }
    pub fn textOperators(&self) -> List { todo!() }
    pub fn updateTableSelectedPosition(&self) { todo!() }
    pub fn isEqualTableSelectedPosition(&self) -> bool { todo!() }
    pub fn clearBulletNumber(&self) -> bool { todo!() }
    pub fn deleteHlinkByBackspace(&self) -> bool { todo!() }
    pub fn tabBullets(&self, isShiftPressed: bool) -> bool { todo!() }
    pub fn canIncreaseIndent(&self, paragraphFormat: KmoParagraphFormat) -> bool { todo!() }
    pub fn canDecreaseIndent(&self, paragraphFormat: KmoParagraphFormat) -> bool { todo!() }
    pub fn isSplitCharacter(ch: u16) -> bool { todo!() }
    pub fn getSpanRange(&self, start: i32, startSpan: i32, text: String) -> IndexRange { todo!() }
    pub fn getUrlOrEmailRange(&self, start: i32, startPara: i32, text: String) -> IndexRange { todo!() }
    pub fn isSpaceChar(&self, ch: u16) -> bool { todo!() }
}
