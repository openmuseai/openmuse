#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

use std::cell::RefCell;
use std::rc::Rc;

// Java: import cn.wps.drawing.data.media.Media
// rust: no-stmt —— `cn.wps.drawing.data.media.Media` 在本仓库**任何**构建下都不参与编译
//   （`drawing/data/mod.rs`：`#[cfg(feature = "full")] pub mod media;`；
//    `drawing/data/media/mod.rs`：全文 `#[cfg(any())] pub mod Media;`）
//   ⇒ `Media` 及其嵌套类 `Media.MediaExtAttr` / `Media.Bookmark` / `Media.BookmarkLst`
//   都无法在类型里直接命名。本类的处理：
//     ① `Media` 本体借 `HandlerCallback` 的关联类型 `Media` 承载（Java 侧由
//        `HandlerCallbackImpl` 绑定到 `cn.wps.drawing.data.media.Media`）；
//     ② `mediaExtAttr()` / `bookmarkLst()` / `write(...)` / `Bookmark.create()` 等成员面
//        由下面 4 个**本地模型契约 trait** 承载（方法名与 Java 一致）。不是替身结构体，
//        只是把 Java 的成员面写成 trait bound；真实模型接线时补 4 个 `impl` 即可，
//        本类的分支、属性号与调用顺序均无需改动。
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
// Java: import java.util.ArrayList
// Java: import java.util.List
// rust: no-stmt —— `List<Media.Bookmark> bookmarkLst = new ArrayList()` 用
//   `RefCell<Vec<Rc<RefCell<_>>>>` 承载（见 `BookmarkLstHandler.bookmarkLst` 字段注）。

// ---------------------------------------------------------------------------
// rust: 本地模型契约（见文件头 no-stmt 注）——逐层对应 Java 的模型成员面
// ---------------------------------------------------------------------------

// rust: 对应 `Media.Bookmark` 的成员面
pub trait BookmarkModel {
    // Java: static Media.Bookmark create()
    fn create() -> Self;
    // Java: void putName(String value)
    fn putName(&mut self, value: &str);
    // Java: void putTime(double value)
    fn putTime(&mut self, value: f64);
    // Java: void save()
    fn save(&mut self);
}

// rust: 对应 `Media.BookmarkLst` 的成员面
pub trait BookmarkLstModel {
    // Java: void write(List<Media.Bookmark> valueLst)
    fn write<B: BookmarkModel>(&mut self, valueLst: &[Rc<RefCell<B>>]);
}

// rust: 对应 `Media.MediaExtAttr` 的成员面
pub trait MediaExtAttrModel {
    // rust: Java `Media.BookmarkLst` 的类型槽（自带 bound，避免调用处重复声明投影约束）
    type BookmarkLst: BookmarkLstModel;
    // Java: void putTrimStart(int value)
    fn putTrimStart(&mut self, value: i32);
    // Java: void putTrimEnd(int value)
    fn putTrimEnd(&mut self, value: i32);
    // Java: void putFadeIn(int value)
    fn putFadeIn(&mut self, value: i32);
    // Java: void putFadeOut(int value)
    fn putFadeOut(&mut self, value: i32);
    // Java: Media.BookmarkLst bookmarkLst()
    fn bookmarkLst(&mut self) -> &mut Self::BookmarkLst;
}

// rust: 对应 `Media` 的成员面（仅本文件用到的部分）
pub trait MediaModel {
    // rust: Java `Media.MediaExtAttr` / `Media.Bookmark` 的类型槽（自带 bound）
    type MediaExtAttr: MediaExtAttrModel;
    type Bookmark: BookmarkModel;
    // Java: Media.MediaExtAttr mediaExtAttr()
    fn mediaExtAttr(&mut self) -> &mut Self::MediaExtAttr;
}

// rust: 关联类型投影别名（同 `ShapeStyleHandler` 的 `type Callback<C>` 手法）——
//   只用于压缩下面字段与调用处的投影书写。
type MediaOf<C> = <C as HandlerCallback>::Media;
type BookmarkOf<C> = <MediaOf<C> as MediaModel>::Bookmark;

// Java: public class MediaExtLstHandler extends DefaultElementHandler
pub struct MediaExtLstHandler<C: HandlerCallback> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java field: Media media
    media: Option<Rc<RefCell<C::Media>>>,
    // Java field: HandlerCallback handlerCallback
    handlerCallback: Option<Rc<RefCell<C>>>,
}

// Java: public class MediaExtLstHandler extends DefaultElementHandler
impl<C: HandlerCallback> MediaExtLstHandler<C> {
    // Java: MediaExtLstHandler(Media media, HandlerCallback handlerCallback)
    pub fn new(
        media: Option<Rc<RefCell<C::Media>>>,
        handlerCallback: Option<Rc<RefCell<C>>>,
    ) -> Self {
        let this = MediaExtLstHandler {
            parent: DefaultElementHandler::new(),
            media,
            handlerCallback,
        };
        // Java: this.media = media;
        // LLM JavaLine Aggregation: merged into struct init above
        // Java: this.handlerCallback = handlerCallback;
        // LLM JavaLine Aggregation: merged into struct init above
        this
    }
}

// rust: `enterSubElement` 要装箱 `MediaExtHandler<C>`，故带上其 impl 的同一约束。
impl<C: HandlerCallback> ElementHandler for MediaExtLstHandler<C>
where
    C: 'static,
    C::Media: MediaModel,
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
            // Java: case 3211447:
            3211447 => {
                // Java: handler = new MediaExtHandler();
                // rust: 内层类在 Java 里经 `MediaExtLstHandler.this` 反向取 `media`/
                //   `handlerCallback`；Rust 侧改为把同一组共享句柄传下去（无 `Weak` 回环）。
                handler = Some(Box::new(MediaExtHandler::<C>::new(
                    self.media.clone(),
                    self.handlerCallback.clone(),
                )));
                // Java: default:
                // Java: return handler;   —— `case` 无 `break`，赋值后直接落到 `default` 的 return
            }
            // Java: default:
            // Java: return handler;   —— handler 仍为 null（Java 的 `default` 已提前返回）
            _ => {
                handler = None;
            }
        }
        // Java: return handler;   —— Java 在 `default:` 里提前返回，此处为 switch 外的统一返回点
        handler
    }
}

// Java: (inner class) class MediaExtHandler extends DefaultElementHandler
pub struct MediaExtHandler<C: HandlerCallback> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java: MediaExtLstHandler.this.media   —— 外层实例引用改为直接持有共享句柄
    media: Option<Rc<RefCell<C::Media>>>,
    // Java: MediaExtLstHandler.this.handlerCallback
    handlerCallback: Option<Rc<RefCell<C>>>,
}

// Java: (inner class) class MediaExtHandler extends DefaultElementHandler
impl<C: HandlerCallback> MediaExtHandler<C> {
    // Java: MediaExtHandler()
    pub fn new(
        media: Option<Rc<RefCell<C::Media>>>,
        handlerCallback: Option<Rc<RefCell<C>>>,
    ) -> Self {
        MediaExtHandler {
            parent: DefaultElementHandler::new(),
            media,
            handlerCallback,
        }
    }
}

// rust: `enterSubElement` 要装箱 `MediaHandler<C>`，故带上其 impl 的同一约束。
impl<C: HandlerCallback> ElementHandler for MediaExtHandler<C>
where
    C: 'static,
    C::Media: MediaModel,
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
            // Java: case 3276849:
            3276849 => {
                // Java: handler = new MediaHandler();
                handler = Some(Box::new(MediaHandler::<C>::new(
                    self.media.clone(),
                    self.handlerCallback.clone(),
                )));
                // Java: default:
                // Java: return handler;   —— `case` 无 `break`，赋值后直接落到 `default` 的 return
            }
            // Java: default:
            // Java: return handler;   —— handler 仍为 null
            _ => {
                handler = None;
            }
        }
        // Java: return handler;
        handler
    }
}

// Java: (inner class) class MediaHandler extends DefaultElementHandler
pub struct MediaHandler<C: HandlerCallback> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java: MediaExtLstHandler.this.media
    media: Option<Rc<RefCell<C::Media>>>,
    // Java: MediaExtLstHandler.this.handlerCallback
    handlerCallback: Option<Rc<RefCell<C>>>,
}

// Java: (inner class) class MediaHandler extends DefaultElementHandler
impl<C: HandlerCallback> MediaHandler<C> {
    // Java: MediaHandler()
    pub fn new(
        media: Option<Rc<RefCell<C::Media>>>,
        handlerCallback: Option<Rc<RefCell<C>>>,
    ) -> Self {
        MediaHandler {
            parent: DefaultElementHandler::new(),
            media,
            handlerCallback,
        }
    }
}

// rust: `enterSubElement` 要装箱 `TrimHandler<C>` / `FadeHandler<C>` / `BookmarkLstHandler<C>`，
//   故带上它们的 impl 的同一约束。
impl<C: HandlerCallback> ElementHandler for MediaHandler<C>
where
    C: 'static,
    C::Media: MediaModel,
{
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr tmp;
        // LLM JavaLine Aggregation: merged into if-let pattern
        // Java: if ((tmp = attrsNotCopy.getByName(4259842)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(4259842) {
            // Java: MediaExtLstHandler.this.handlerCallback.onExtMediaFileLink(tmp.toString(), MediaExtLstHandler.this.media);
            let link = tmp.to_string();
            // rust: `startElement` 取 `&self`，回调取 `&mut self`；句柄先 clone 再 `borrow_mut()`，
            //   借用期覆盖整条回调语句（同 `HyperLinkHandler`）。
            let media = self.media.clone();
            let mut media = media.as_ref().map(|m| m.borrow_mut());
            self.handlerCallback
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .onExtMediaFileLink(Some(link.as_str()), media.as_deref_mut());
        }
        // Java: else if ((tmp = attrsNotCopy.getByName(4259843)) != null)
        else if let Some(tmp) = attrsNotCopy.get_by_name(4259843) {
            // Java: MediaExtLstHandler.this.handlerCallback.onExtMediaFileLink(tmp.toString(), MediaExtLstHandler.this.media);
            let link = tmp.to_string();
            let media = self.media.clone();
            let mut media = media.as_ref().map(|m| m.borrow_mut());
            self.handlerCallback
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .onExtMediaFileLink(Some(link.as_str()), media.as_deref_mut());
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

    // Java: public ElementHandler enterSubElement(int subElement)
    fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
        // Java: ElementHandler handler = null;
        let handler: Option<Box<dyn ElementHandler>>;
        // Java: switch (subElement)
        match subElement {
            // Java: case 3276846:
            3276846 => {
                // Java: handler = MediaExtHandler.this.new TrimHandler();
                handler = Some(Box::new(TrimHandler::<C>::new(self.media.clone())));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3276847:
            3276847 => {
                // Java: handler = MediaExtHandler.this.new FadeHandler();
                handler = Some(Box::new(FadeHandler::<C>::new(self.media.clone())));
                // Java: break;
                // rust: no-stmt
            }
            // Java: case 3276848:
            3276848 => {
                // Java: handler = MediaExtHandler.this.new BookmarkLstHandler();
                // rust: 无 `break` —— 赋值后落到 switch 外统一 `return handler;`
                handler = Some(Box::new(BookmarkLstHandler::<C>::new(self.media.clone())));
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

// Java: (inner class) class TrimHandler extends DefaultElementHandler
pub struct TrimHandler<C: HandlerCallback> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java: MediaExtLstHandler.this.media
    media: Option<Rc<RefCell<C::Media>>>,
}

// Java: (inner class) class TrimHandler extends DefaultElementHandler
impl<C: HandlerCallback> TrimHandler<C> {
    // Java: TrimHandler()
    pub fn new(media: Option<Rc<RefCell<C::Media>>>) -> Self {
        TrimHandler {
            parent: DefaultElementHandler::new(),
            media,
        }
    }
}

impl<C: HandlerCallback> ElementHandler for TrimHandler<C>
where
    C: 'static,
    C::Media: MediaModel,
{
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr tmp;
        // LLM JavaLine Aggregation: merged into if-let pattern
        // Java: if ((tmp = attrsNotCopy.getByName(3276839)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(3276839) {
            // Java: MediaExtLstHandler.this.media.mediaExtAttr().putTrimStart((int)tmp.toDouble());
            let value = tmp.to_double() as i32;
            // rust: Java 此处无判空 ⇒ `expect` 对齐 NPE 语义。
            self.media
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .mediaExtAttr()
                .putTrimStart(value);
        }
        // Java: if ((tmp = attrsNotCopy.getByName(3276840)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(3276840) {
            // Java: MediaExtLstHandler.this.media.mediaExtAttr().putTrimEnd((int)tmp.toDouble());
            let value = tmp.to_double() as i32;
            self.media
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .mediaExtAttr()
                .putTrimEnd(value);
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

// Java: (inner class) class FadeHandler extends DefaultElementHandler
pub struct FadeHandler<C: HandlerCallback> {
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java: MediaExtLstHandler.this.media
    media: Option<Rc<RefCell<C::Media>>>,
}

// Java: (inner class) class FadeHandler extends DefaultElementHandler
impl<C: HandlerCallback> FadeHandler<C> {
    // Java: FadeHandler()
    pub fn new(media: Option<Rc<RefCell<C::Media>>>) -> Self {
        FadeHandler {
            parent: DefaultElementHandler::new(),
            media,
        }
    }
}

impl<C: HandlerCallback> ElementHandler for FadeHandler<C>
where
    C: 'static,
    C::Media: MediaModel,
{
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr tmp;
        // LLM JavaLine Aggregation: merged into if-let pattern
        // Java: if ((tmp = attrsNotCopy.getByName(3276841)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(3276841) {
            // Java: MediaExtLstHandler.this.media.mediaExtAttr().putFadeIn(tmp.toInt());
            let value = tmp.to_int();
            // rust: Java 此处无判空 ⇒ `expect` 对齐 NPE 语义。
            self.media
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .mediaExtAttr()
                .putFadeIn(value);
        }
        // Java: if ((tmp = attrsNotCopy.getByName(3276842)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(3276842) {
            // Java: MediaExtLstHandler.this.media.mediaExtAttr().putFadeOut(tmp.toInt());
            let value = tmp.to_int();
            self.media
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .mediaExtAttr()
                .putFadeOut(value);
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

// Java: (inner class) class BookmarkLstHandler extends DefaultElementHandler
pub struct BookmarkLstHandler<C>
where
    C: HandlerCallback,
    C: 'static,
    C::Media: MediaModel,
{
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java: MediaExtLstHandler.this.media
    media: Option<Rc<RefCell<C::Media>>>,
    // Java field: List<Media.Bookmark> bookmarkLst = new ArrayList()
    // rust: 本契约的两个方法都取 `&self`，而 Java 的 `bookmarkLst.add(...)` 是写操作
    //   ⇒ 列表落在 `RefCell` 里；元素与 `BookmarkHandler` 共享同一 `Rc<RefCell<_>>` 句柄
    //   （Java 里是同一个 `Media.Bookmark` 对象）。
    bookmarkLst: RefCell<Vec<Rc<RefCell<BookmarkOf<C>>>>>,
}

// Java: (inner class) class BookmarkLstHandler extends DefaultElementHandler
impl<C> BookmarkLstHandler<C>
where
    C: HandlerCallback,
    C: 'static,
    C::Media: MediaModel,
{
    // Java: BookmarkLstHandler()
    pub fn new(media: Option<Rc<RefCell<C::Media>>>) -> Self {
        // Java: List<Media.Bookmark> bookmarkLst = new ArrayList();
        // Java: this.bookmarkLst = new ArrayList();
        BookmarkLstHandler {
            parent: DefaultElementHandler::new(),
            media,
            bookmarkLst: RefCell::new(Vec::new()),
        }
    }
}

// rust: `enterSubElement` 要装箱 `BookmarkHandler<C>`，故带上其 impl 的同一约束。
impl<C> ElementHandler for BookmarkLstHandler<C>
where
    C: HandlerCallback,
    C: 'static,
    C::Media: MediaModel,
{
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)   —— 继承空实现
    fn startElement(&self, element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: super.startElement(element, attrsNotCopy);
        self.parent.startElement(element, attrsNotCopy);
    }

    // Java: public void endElement(int element)
    fn endElement(&self, _element: i32) {
        // Java: for(Media.Bookmark bookmark : this.bookmarkLst)
        for bookmark in self.bookmarkLst.borrow().iter() {
            // Java: bookmark.save();
            bookmark.borrow_mut().save();
        }
        // Java: MediaExtLstHandler.this.media.mediaExtAttr().bookmarkLst().write(this.bookmarkLst);
        // rust: Java 此处无判空 ⇒ `expect` 对齐 NPE 语义；借用期用局部量覆盖整条调用。
        let media = self.media.as_ref().expect("NullPointerException");
        let mut media = media.borrow_mut();
        let bookmarkLst = self.bookmarkLst.borrow();
        media
            .mediaExtAttr()
            .bookmarkLst()
            .write(bookmarkLst.as_slice());
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
            // Java: case 3276845:
            3276845 => {
                // Java: Media.Bookmark bookmark = Media.Bookmark.create();
                let bookmark = Rc::new(RefCell::new(<BookmarkOf<C> as BookmarkModel>::create()));
                // Java: this.bookmarkLst.add(bookmark);
                self.bookmarkLst.borrow_mut().push(bookmark.clone());
                // Java: handler = new BookmarkHandler(bookmark);
                handler = Some(Box::new(BookmarkHandler::<C>::new(Some(bookmark))));
                // Java: default:
                // Java: return handler;   —— `case` 无 `break`，赋值后直接落到 `default` 的 return
            }
            // Java: default:
            // Java: return handler;   —— handler 仍为 null
            _ => {
                handler = None;
            }
        }
        // Java: return handler;
        handler
    }
}

// Java: (inner class) class BookmarkHandler extends DefaultElementHandler
pub struct BookmarkHandler<C>
where
    C: HandlerCallback,
    C: 'static,
    C::Media: MediaModel,
{
    // parent: DefaultElementHandler composition for extends
    pub(crate) parent: DefaultElementHandler,
    // Java field: Media.Bookmark bookmark
    bookmark: Option<Rc<RefCell<BookmarkOf<C>>>>,
}

// Java: (inner class) class BookmarkHandler extends DefaultElementHandler
impl<C> BookmarkHandler<C>
where
    C: HandlerCallback,
    C: 'static,
    C::Media: MediaModel,
{
    // Java: BookmarkHandler(Media.Bookmark bookmark)
    pub fn new(bookmark: Option<Rc<RefCell<BookmarkOf<C>>>>) -> Self {
        let this = BookmarkHandler {
            parent: DefaultElementHandler::new(),
            bookmark,
        };
        // Java: this.bookmark = bookmark;
        // LLM JavaLine Aggregation: merged into struct init above
        this
    }
}

impl<C> ElementHandler for BookmarkHandler<C>
where
    C: HandlerCallback,
    C: 'static,
    C::Media: MediaModel,
{
    // Java: public void startElement(int element, XmlRoAttr attrsNotCopy)
    fn startElement(&self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
        // Java: XmlRoAttr tmp;
        // LLM JavaLine Aggregation: merged into if-let pattern
        // Java: if ((tmp = attrsNotCopy.getByName(3276843)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(3276843) {
            // Java: this.bookmark.putName(tmp.toString());
            let value = tmp.to_string();
            // rust: Java 此处无判空 ⇒ `expect` 对齐 NPE 语义。
            self.bookmark
                .as_ref()
                .expect("NullPointerException")
                .borrow_mut()
                .putName(&value);
        }
        // Java: if ((tmp = attrsNotCopy.getByName(3276844)) != null)
        if let Some(tmp) = attrsNotCopy.get_by_name(3276844) {
            // Java: this.bookmark.putTime(tmp.toDouble());
            let value = tmp.to_double();
            self.bookmark
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
