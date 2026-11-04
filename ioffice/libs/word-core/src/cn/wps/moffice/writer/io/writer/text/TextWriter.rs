// Rust stub for TextWriter (auto-generated)
// Original: cn/wps/moffice/writer/io/writer/text/TextWriter.java
use crate::*;

pub struct TextWriter {
    // TODO: fields
}

impl TextWriter {
    pub fn new() -> Self { todo!() }
    pub fn write(&self, bAutoWrite: bool) { todo!() }
    pub fn writeSingleParagraph(&self, bAutoWrite: bool, lock: WriterReadLock, out: Writer, body: Box<dyn Document>, chpxIt: CPIterator, paraEnd: i32) { todo!() }
    pub fn isFirstLineIndent(&self, parProp: PropertySet) -> bool { todo!() }
    pub fn checkEncoding(&self) -> String { todo!() }
    pub fn addBom2ToFile(&self, encoding: String, out: FileOutputStream) { todo!() }
    pub fn writeSingleRun(&self, out: Writer, body: Box<dyn Document>, run: IChpxTable::Entry, start: i32, end: i32) { todo!() }
    pub fn filterText(&self, start: i32, end: i32, bSpecial: bool) -> KCharArrayBuffer { todo!() }
    pub fn getSpecialChar(&self, c: u16) -> u16 { todo!() }
    pub fn isVisibleChar(&self, c: u16) -> bool { todo!() }
    pub fn expand(&self, newlen: i32) { todo!() }
    pub fn append(&self, b: Vec<u16>, off: i32, len: i32) { todo!() }
    pub fn append_1(&self, str: String) { todo!() }
    pub fn append_2(&self, b: KCharArrayBuffer, off: i32, len: i32) { todo!() }
    pub fn append_3(&self, b: KCharArrayBuffer) { todo!() }
    pub fn append_4(&self, ch: u16) { todo!() }
    pub fn append_5(&self, b: Vec<i8>, off: i32, len: i32) { todo!() }
    pub fn clear(&self) { todo!() }
    pub fn toCharArray(&self) -> Vec<u16> { todo!() }
    pub fn charAt(&self, i: i32) -> u16 { todo!() }
    pub fn buffer(&self) -> Vec<u16> { todo!() }
    pub fn capacity(&self) -> i32 { todo!() }
    pub fn length(&self) -> i32 { todo!() }
    pub fn ensureCapacity(&self, required: i32) { todo!() }
    pub fn setLength(&self, len: i32) { todo!() }
    pub fn isEmpty(&self) -> bool { todo!() }
    pub fn isFull(&self) -> bool { todo!() }
    pub fn indexOf(&self, ch: i32, beginIndex: i32, endIndex: i32) -> i32 { todo!() }
    pub fn indexOf_1(&self, ch: i32) -> i32 { todo!() }
    pub fn substring(&self, beginIndex: i32, endIndex: i32) -> String { todo!() }
    pub fn toString(&self) -> String { todo!() }
}
