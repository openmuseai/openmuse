#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.media.Media
// rust: no-stmt —— `cn.wps.drawing.data.media.Media` 在本仓库**任何**构建下都不参与编译
//   （`drawing/data/mod.rs`：`#[cfg(feature = "full")] pub mod media;`；
//    `drawing/data/media/mod.rs`：全文 `#[cfg(any())] pub mod Media;`）
//   ⇒ 嵌套类 `Media.AudioFile` 无法在类型里直接命名。本类的模型槽借 `HandlerCallback`
//   的关联类型 `AudioFile` 承载（Java 侧由 `HandlerCallbackImpl` 绑定到 `Media.AudioFile`），
//   其 setter 面 `putContentType` 由本地契约 trait 承载（见下），不是替身结构体。
//   owner: ppt.drawing.dmlr.handler.media    wave: M-PPT-KMOSLIDE-W8a
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::HandlerCallback;
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::DefaultElementHandler::DefaultElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// rust: Java 的 `XmlRoAttr extends XmlRoAttrValue`，`temp.toString()` / `temp.toInt()` /
//   `temp.toDouble()` 由父接口提供。Rust 侧 `dyn XmlRoAttr` 的方法解析同样覆盖父 trait
//   `XmlRoAttrValue`（实测：额外 `use` 该 trait 会触发 `unused_imports`），故无需额外导入。

// rust: 本地模型契约 —— `Media.AudioFile` 的 setter 面（见文件头 no-stmt 注）。
//   真实模型接线时 `impl AudioFileModel for <真实 Media.AudioFile>` 即可。
pub trait AudioFileModel {
    // Java: void putContentType(String value)
    fn putContentType(&mut self, value: &str);
}

// Java: public class AudioFileHandler extends DefaultElementHandler
pub struct AudioFileHandler<C: HandlerCallback> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java field: Media.AudioFile audioFile
    audioFile: Option<Rc<RefCell<C::AudioFile>>>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Option<Rc<RefCell<C>>>,
}

// Java: public class AudioFileHandler extends DefaultElementHandler
impl<C: HandlerCallback> AudioFileHandler<C> {
    // Java: AudioFileHandler(Media.AudioFile audioFile, HandlerCallback handlerCallback)
    pub fn new(
        audioFile: Option<Rc<RefCell<C::AudioFile>>>,
        handlerCallback: Option<Rc<RefCell<C>>>,
    ) -> Self {
        let this = AudioFileHandler {
            parent: DefaultElementHandler::new(),
            audioFile,
            handlerCallback,
        };
        // Java: this.audioFile = audioFile;
        // LLM JavaLine Aggregation: merged into struct init above
        // Java: this.handlerCallback = handlerCallback;
        // LLM JavaLine Aggregation: merged into struct init above
        this
    }
}

// rust: `putContentType` 落在 `AudioFileModel` 契约上，故此处带该约束。
impl<C: HandlerCallback> ElementHandler for AudioFileHandler<C>
where
    C::AudioFile: AudioFileModel,
{
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr temp = null;
        // LLM JavaLine Aggregation: merged into if-let pattern
        // Java: if ((temp = attrsNotCopy.getByName(1114114)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114114) {
            // Java: this.audioFile.putContentType(temp.toString());
            let value = temp.to_string();
            // rust: Java 此处无判空 ⇒ `expect` 对齐 NPE 语义（同 `HyperLinkHandler` 的取舍）。
            self.audioFile
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .putContentType(&value);
        }
        // Java: if ((temp = attrsNotCopy.getByName(4259843)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(4259843) {
            // Java: this.handlerCallback.onAudioFileLink(temp.toString(), this.audioFile);
            let link = temp.to_string();
            // rust: 本契约的 `startElement` 取 `&self`，而 `onAudioFileLink` 取 `&mut self`；
            //   回调句柄与模型句柄都先 clone 成局部量，再 `borrow_mut()` 落地 `RefMut`，
            //   使借用期覆盖整条回调语句（同 `HyperLinkHandler` 的写法）。
            let audioFile = self.audioFile.clone();
            let mut audioFile = audioFile.as_ref().map(|m| m.borrow_mut());
            self.handlerCallback
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .onAudioFileLink(Some(link.as_str()), audioFile.as_deref_mut());
        }
    }

    // Java: public void endElement(int element)   —— 继承空实现
    fn endElement(&self, element: i32) {
        // Java: super.endElement(element);
        self.parent.endElement(element);
    }

    // Java: public void characters(String chars)   —— 继承空实现
    fn characters(&self, chars: &str) {
        // Java: super.characters(chars);
        self.parent.characters(chars);
    }

    // Java: public ElementHandler enterSubElement(int subElement)   —— 继承空实现
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: return super.enterSubElement(subElement);
        self.parent.enterSubElement(subElement)
    }
}
