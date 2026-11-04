#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.media.Media
// rust: no-stmt —— `cn.wps.drawing.data.media.Media` 在本仓库**任何**构建下都不参与编译
//   （`drawing/data/mod.rs`：`#[cfg(feature = "full")] pub mod media;`；
//    `drawing/data/media/mod.rs`：全文 `#[cfg(any())] pub mod Media;`）
//   ⇒ 嵌套类 `Media.EmbeddedWAVAudioFile` 无法在类型里直接命名。本类的模型槽借
//   `HandlerCallback` 的关联类型 `EmbeddedWAVAudioFile` 承载（Java 侧由 `HandlerCallbackImpl`
//   绑定到 `Media.EmbeddedWAVAudioFile`），其 setter 面 `putName` 由本地契约 trait 承载（见下），
//   不是替身结构体。
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

// rust: 本地模型契约 —— `Media.EmbeddedWAVAudioFile` 的 setter 面（见文件头 no-stmt 注）。
//   真实模型接线时 `impl EmbeddedWAVAudioFileModel for <真实 Media.EmbeddedWAVAudioFile>` 即可。
pub trait EmbeddedWAVAudioFileModel {
    // Java: void putName(String value)
    fn putName(&mut self, value: &str);
}

// Java: public class EmbeddedWAVAudioFileHandler extends DefaultElementHandler
pub struct EmbeddedWAVAudioFileHandler<C: HandlerCallback> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java field: Media.EmbeddedWAVAudioFile waveAudioFile
    waveAudioFile: Option<Rc<RefCell<C::EmbeddedWAVAudioFile>>>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Option<Rc<RefCell<C>>>,
}

// Java: public class EmbeddedWAVAudioFileHandler extends DefaultElementHandler
impl<C: HandlerCallback> EmbeddedWAVAudioFileHandler<C> {
    // Java: EmbeddedWAVAudioFileHandler(Media.EmbeddedWAVAudioFile waveAudioFile, HandlerCallback handlerCallback)
    pub fn new(
        waveAudioFile: Option<Rc<RefCell<C::EmbeddedWAVAudioFile>>>,
        handlerCallback: Option<Rc<RefCell<C>>>,
    ) -> Self {
        let this = EmbeddedWAVAudioFileHandler {
            parent: DefaultElementHandler::new(),
            waveAudioFile,
            handlerCallback,
        };
        // Java: this.waveAudioFile = waveAudioFile;
        // LLM JavaLine Aggregation: merged into struct init above
        // Java: this.handlerCallback = handlerCallback;
        // LLM JavaLine Aggregation: merged into struct init above
        this
    }
}

// rust: `putName` 落在 `EmbeddedWAVAudioFileModel` 契约上，故此处带该约束。
impl<C: HandlerCallback> ElementHandler for EmbeddedWAVAudioFileHandler<C>
where
    C::EmbeddedWAVAudioFile: EmbeddedWAVAudioFileModel,
{
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr temp = null;
        // LLM JavaLine Aggregation: merged into if-let pattern
        // Java: if ((temp = attrsNotCopy.getByName(4259842)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(4259842) {
            // Java: this.handlerCallback.onEmbeddedWAVAudioFileEmbed(temp.toString(), this.waveAudioFile);
            let embed = temp.to_string();
            // rust: `startElement` 取 `&self`，回调取 `&mut self`；句柄先 clone 再 `borrow_mut()`，
            //   借用期覆盖整条回调语句（同 `HyperLinkHandler`）。
            let waveAudioFile = self.waveAudioFile.clone();
            let mut waveAudioFile = waveAudioFile.as_ref().map(|m| m.borrow_mut());
            self.handlerCallback
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .onEmbeddedWAVAudioFileEmbed(Some(embed.as_str()), waveAudioFile.as_deref_mut());
        }
        // Java: if ((temp = attrsNotCopy.getByName(3211449)) != null || (temp = attrsNotCopy.getByName(1114136)) != null)
        // rust: Java 用同一个 `temp` 串接 `||`（短路：第一个非空即取第一个，否则取第二个）。
        //   逐段 `Option` 选择与之一一对应（同 `ParagraphHandler` 的同名写法）。
        let temp = match attrsNotCopy.get_by_name(3211449) {
            // Java: (temp = attrsNotCopy.getByName(3211449)) != null
            Some(temp) => Some(temp),
            // Java: (temp = attrsNotCopy.getByName(1114136)) != null
            None => attrsNotCopy.get_by_name(1114136),
        };
        // Java: {
        if let Some(temp) = temp {
            // Java: this.waveAudioFile.putName(temp.toString());
            let value = temp.to_string();
            // rust: Java 此处无判空 ⇒ `expect` 对齐 NPE 语义。
            self.waveAudioFile
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .putName(&value);
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
