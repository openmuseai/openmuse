#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// java public interface DocumentEvent
#[derive(Default, Clone, Debug)]
pub struct DocumentEvent {
    // java fields live on DefaultModelEvent; product events carry the same payload
    pub(crate) mEventStart: i32,
    pub(crate) mEventLength: i32,
    pub(crate) mEventType: i32,
    pub(crate) mSubEventType: i32,
}

// java int EVENT_EMPTY = 0;
pub const EVENT_EMPTY: i32 = 0;
// java int EVENT_INSERT = 1;
pub const EVENT_INSERT: i32 = 1;
// java int EVENT_REMOVE = 2;
pub const EVENT_REMOVE: i32 = 2;
// java int EVENT_CHANGED = 3;
pub const EVENT_CHANGED: i32 = 3;
// java int EVENT_STRUCTURE = 4;
pub const EVENT_STRUCTURE: i32 = 4;
// java int EVENT_TABLE_CHANGE = 5;
pub const EVENT_TABLE_CHANGE: i32 = 5;
// java int EVENT_SHAPE_EDIT = 6;
pub const EVENT_SHAPE_EDIT: i32 = 6;
// java int EVENT_REFRESH = 7;
pub const EVENT_REFRESH: i32 = 7;
// java int EVENT_REVISION_UNDO_REMOVE = 8;
pub const EVENT_REVISION_UNDO_REMOVE: i32 = 8;
// java int EVENT_REVISION_REMOVE = 9;
pub const EVENT_REVISION_REMOVE: i32 = 9;
// java int EVENT_MAX = 10;
pub const EVENT_MAX: i32 = 10;
// java int EVENT_SUBTYPE_PAPX_CHANGED = 11;
pub const EVENT_SUBTYPE_PAPX_CHANGED: i32 = 11;
// java int EVENT_SUBTYPE_CHPX_CHANGED = 12;
pub const EVENT_SUBTYPE_CHPX_CHANGED: i32 = 12;
// java int EVENT_SUBTYPE_SHAPE_MOVE = 14;
pub const EVENT_SUBTYPE_SHAPE_MOVE: i32 = 14;
// java int EVENT_SUBTYPE_SHAPE_ROTATE = 15;
pub const EVENT_SUBTYPE_SHAPE_ROTATE: i32 = 15;
// java int EVENT_SUBTYPE_SHAPE_CLIP = 16;
pub const EVENT_SUBTYPE_SHAPE_CLIP: i32 = 16;
// java int EVENT_SUBTYPE_SHAPE_CHANGED = 17;
pub const EVENT_SUBTYPE_SHAPE_CHANGED: i32 = 17;
// java int EVENT_SUBTYPE_SECTION_REMOVE = 18;
pub const EVENT_SUBTYPE_SECTION_REMOVE: i32 = 18;
// java int EVENT_SUBTYPE_MERGE_SPILIT = 19;
pub const EVENT_SUBTYPE_MERGE_SPILIT: i32 = 19;
// java int EVENT_SUBTYPE_DOC_FIELD_CHANGE = 20;
pub const EVENT_SUBTYPE_DOC_FIELD_CHANGE: i32 = 20;

impl DocumentEvent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_at(offset: i32, length: i32) -> Self {
        Self {
            mEventStart: offset,
            mEventLength: length,
            mEventType: EVENT_INSERT,
            mSubEventType: 0,
        }
    }

    pub fn remove_at(offset: i32, length: i32) -> Self {
        Self {
            mEventStart: offset,
            mEventLength: length,
            mEventType: EVENT_REMOVE,
            mSubEventType: 0,
        }
    }

    // java void increaseRef();
    pub fn increaseRef(&mut self) {
        // rust: no-stmt
    }

    // java void decreaseRef();
    pub fn decreaseRef(&mut self) {
        // rust: no-stmt
    }

    // java int getOffset();
    pub fn getOffset(&self) -> i32 {
        self.mEventStart
    }

    // java int getLength();
    pub fn getLength(&self) -> i32 {
        self.mEventLength
    }

    // java void setLength(int var1);
    pub fn setLength(&mut self, var1: i32) {
        self.mEventLength = var1;
    }

    // java int getType();
    pub fn getType(&self) -> i32 {
        self.mEventType
    }

    // java int getSubType();
    pub fn getSubType(&self) -> i32 {
        self.mSubEventType
    }

    // java Document getDocument();
    pub fn getDocument(&self) -> Option<*mut u8> {
        // Product events do not retain a Document* (heap session owns the doc).
        None
    }

    // java String toString();
    pub fn toString(&self) -> String {
        format!(
            "DocumentEvent{{type={},offset={},length={}}}",
            self.mEventType, self.mEventStart, self.mEventLength
        )
    }
}
