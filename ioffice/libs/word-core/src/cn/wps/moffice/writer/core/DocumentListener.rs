// Java: import java.util.EventListener
use stubs::java::util::EventListener::*;
// Java: import cn.wps.moffice.writer.core.DocumentEvent
use crate::cn::wps::moffice::writer::core::DocumentEvent::*;

// java public interface DocumentListener
pub trait DocumentListener {
    // java  void update(DocumentEvent var1)
    // rust: ★ Java `DocumentEvent` 是**接口**，本移植把事件载荷建模为**具体结构体**
    //   `core/DocumentEvent.rs`（唯一载荷形状；既有 `DocumentEventDispatcher::update`、
    //   `FlowLayout::update` 均用 `&DocumentEvent`）⇒ 此处按同一约定用具体类型，
    //   而非 `&dyn`（`&dyn` 会因 `DocumentEvent` 非 trait 而 E0404）。
    fn update(&mut self, var1: &DocumentEvent);

}
