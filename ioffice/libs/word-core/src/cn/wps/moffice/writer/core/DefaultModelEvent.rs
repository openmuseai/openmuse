#![allow(non_snake_case, dead_code)]
#[derive(Default, Clone, Debug)]
pub struct DefaultModelEvent {}
impl DefaultModelEvent {
    pub fn new() -> Self { Self::default() }
    pub fn obtain__document_int_int_int(
        _doc: Option<&dyn crate::cn::wps::moffice::writer::core::Document::Document>,
        _start: i32,
        _len: i32,
        _event_type: i32,
    ) -> crate::cn::wps::moffice::writer::core::DocumentEvent::DocumentEvent {
        crate::cn::wps::moffice::writer::core::DocumentEvent::DocumentEvent {
            mEventStart: _start,
            mEventLength: _len,
            mEventType: _event_type,
            mSubEventType: 0,
        }
    }

    pub fn obtain__document_int_int_int_int(
        _doc: Option<&dyn crate::cn::wps::moffice::writer::core::Document::Document>,
        _start: i32,
        _end: i32,
        _a: i32,
        _b: i32,
    ) -> crate::cn::wps::moffice::writer::core::DocumentEvent::DocumentEvent {
        crate::cn::wps::moffice::writer::core::DocumentEvent::DocumentEvent {
            mEventStart: _start,
            mEventLength: _end,
            mEventType: _a,
            mSubEventType: _b,
        }
    }
}
