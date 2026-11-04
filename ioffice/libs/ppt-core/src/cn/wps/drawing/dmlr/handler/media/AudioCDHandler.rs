#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.media.Media
// rust: no-stmt —— `cn.wps.drawing.data.media.Media` 在本仓库**任何**构建下都不参与编译
//   （`drawing/data/mod.rs`：`#[cfg(feature = "full")] pub mod media;`；
//    `drawing/data/media/mod.rs`：全文 `#[cfg(any())] pub mod Media;`）
//   ⇒ 其嵌套类 `Media.AudioCD` / `Media.AudioCD.AudioCDTime` 无法在类型里直接命名。
//   本类在 Java 侧没有 `HandlerCallback` 形参可借关联类型槽，故用「泛型槽 + 本地模型契约」
//   承载它：`AudioCDHandler<M: AudioCDModel>`。真实模型接线时
//   `impl AudioCDModel for <真实 AudioCD>`、`impl AudioCDTimeModel for <真实 AudioCDTime>` 即可，
//   本类的分支/属性号/调用顺序无需改动。这不是替身结构体，只是把 Java 的成员面写成 trait bound。
//   owner: ppt.drawing.dmlr.handler.media    wave: M-PPT-KMOSLIDE-W8a
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::DefaultElementHandler::DefaultElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;
// rust: Java 的 `XmlRoAttr extends XmlRoAttrValue`，`temp.toString()` / `temp.toInt()` /
//   `temp.toDouble()` 由父接口提供。Rust 侧 `dyn XmlRoAttr` 的方法解析同样覆盖父 trait
//   `XmlRoAttrValue`（实测：额外 `use` 该 trait 会触发 `unused_imports`），故无需额外导入。

// rust: 本地模型契约 —— `Media.AudioCD.AudioCDTime` 的成员面（见文件头 no-stmt 注）。
pub trait AudioCDTimeModel {
    // Java: void putTrack(int value)
    fn putTrack(&mut self, value: i32);
    // Java: void putTime(int value)
    fn putTime(&mut self, value: i32);
}

// rust: 本地模型契约 —— `Media.AudioCD` 的成员面（见文件头 no-stmt 注）。
pub trait AudioCDModel {
    // rust: Java `Media.AudioCD.AudioCDTime` 的类型槽
    type AudioCDTime: AudioCDTimeModel;
    // Java: Media.AudioCD.AudioCDTime st()
    fn st(&mut self) -> Rc<RefCell<Self::AudioCDTime>>;
    // Java: Media.AudioCD.AudioCDTime end()
    fn end(&mut self) -> Rc<RefCell<Self::AudioCDTime>>;
}

// Java: public class AudioCDHandler extends DefaultElementHandler
pub struct AudioCDHandler<M: AudioCDModel> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java field: Media.AudioCD audioCD
    audioCD: Option<Rc<RefCell<M>>>,
}

// Java: public class AudioCDHandler extends DefaultElementHandler
impl<M: AudioCDModel> AudioCDHandler<M> {
    // Java: AudioCDHandler(Media.AudioCD audioCD)
    pub fn new(audioCD: Option<Rc<RefCell<M>>>) -> Self {
        let this = AudioCDHandler {
            parent: DefaultElementHandler::new(),
            audioCD,
        };
        // Java: this.audioCD = audioCD;
        // LLM JavaLine Aggregation: merged into struct init above
        this
    }
}

// rust: `enterSubElement` 返回 `Box<dyn ElementHandler>`（默认 `+ 'static`），被装箱的
//   `AudioCDTimeHandler<M::AudioCDTime>` 需要该关联类型 `'static`。
impl<M: AudioCDModel> ElementHandler for AudioCDHandler<M>
where
    M::AudioCDTime: 'static,
{
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)   —— 继承空实现
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: super.startElement(element, attrsNotCopy);
        self.parent.startElement(element, attrsNotCopy);
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

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 1114117:
            1114117 => {
                // Java: handler = new AudioCDTimeHandler(this.audioCD.st());
                // rust: Java 的 `this.audioCD` 在此**没有** null 判空，为对齐「null 即 NPE」
                //   的 Java 语义用 `expect`，而非静默返回 null。
                let audioCD = self.audioCD.as_ref().expect("NullPointerException");
                // rust: Java 的 `st()` 是懒取（会写回模型缓存），故契约取 `&mut self`，
                //   并以共享句柄 `Rc<RefCell<_>>` 交出子对象。
                let audioCDTime = audioCD.borrow_mut().st();
                handler = Some(Box::new(AudioCDTimeHandler::new(Some(audioCDTime))));
                // Java: break;
                // rust: no-stmt —— 赋值后跳出 switch 即到达末尾 `return handler;`
            }
            // Java: case 1114118:
            1114118 => {
                // Java: handler = new AudioCDTimeHandler(this.audioCD.end());
                let audioCD = self.audioCD.as_ref().expect("NullPointerException");
                let audioCDTime = audioCD.borrow_mut().end();
                handler = Some(Box::new(AudioCDTimeHandler::new(Some(audioCDTime))));
                // Java: break;
                // rust: no-stmt
            }
            // Java: (switch 无 default 分支 —— 其它 subElement 下 handler 保持 null)
            _ => {
                handler = None;
            }
        }
        // Java: return handler;
        handler
    }
}

// Java: (inner class) private class AudioCDTimeHandler extends DefaultElementHandler
pub struct AudioCDTimeHandler<T: AudioCDTimeModel> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java field: Media.AudioCD.AudioCDTime audioCDTime
    audioCDTime: Option<Rc<RefCell<T>>>,
}

// Java: (inner class) private class AudioCDTimeHandler extends DefaultElementHandler
impl<T: AudioCDTimeModel> AudioCDTimeHandler<T> {
    // Java: AudioCDTimeHandler(Media.AudioCD.AudioCDTime audioCDTime)
    pub fn new(audioCDTime: Option<Rc<RefCell<T>>>) -> Self {
        let this = AudioCDTimeHandler {
            parent: DefaultElementHandler::new(),
            audioCDTime,
        };
        // Java: this.audioCDTime = audioCDTime;
        // LLM JavaLine Aggregation: merged into struct init above
        this
    }
}

impl<T: AudioCDTimeModel> ElementHandler for AudioCDTimeHandler<T> {
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr temp = null;
        // LLM JavaLine Aggregation: merged into if-let pattern
        // Java: if ((temp = attrsNotCopy.getByName(1114115)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114115) {
            // Java: this.audioCDTime.putTrack(temp.toInt());
            let value = temp.to_int();
            // rust: Java 此处无判空（构造器已赋非空），故用 `expect` 对齐 NPE 语义。
            self.audioCDTime
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .putTrack(value);
        }
        // Java: if ((temp = attrsNotCopy.getByName(1114116)) != null)
        if let Some(temp) = attrsNotCopy.get_by_name(1114116) {
            // Java: this.audioCDTime.putTime(temp.toInt());
            let value = temp.to_int();
            self.audioCDTime
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .putTime(value);
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
