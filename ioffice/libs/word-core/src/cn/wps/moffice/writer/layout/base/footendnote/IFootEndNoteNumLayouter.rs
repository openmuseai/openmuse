// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteRef
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode;

// java public interface IFootEndNoteNumLayouter
pub trait IFootEndNoteNumLayouter {
    // java  String getFootNoteNum(PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode var1)
    fn getFootNoteNum(&mut self, var1: &FootnoteEndnoteRefNode) -> String;

    // java  String getEndNoteNum(PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode var1)
    fn getEndNoteNum(&mut self, var1: &FootnoteEndnoteRefNode) -> String;

    // java  String getNumByCP(Document var1, int var2)
    fn getNumByCP(&mut self, var1: &dyn Document, var2: i32) -> Option<String>;

    // java  void dispose()
    fn dispose(&mut self);

    // rust: Java 字段是同一 MainState / DocumentEnv / ViewEnv / StateSnapshot 对象。
    // 宿主移动到最终地址后重绑指针，方法本身不在 Java 接口上。
    fn rebind_ptrs(&mut self, _main: *mut (), _doc: *mut (), _view: *mut (), _snap: *mut ()) {}

    // java PageViewFootEndNoteNumLayouter.reset(StateSnapshot) — 快照已由 rebind_ptrs 指向同一对象
    fn reset(&mut self) {}
}
