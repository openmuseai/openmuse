#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
// Java: import cn.wps.kfc.xpxml.reader.DefaultElementHandler
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
// Java: import cn.wps.show.data.Presentation.CommentAuthor
use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
// Java: import cn.wps.show.data.Presentation.CommentAuthorLst
use crate::cn::wps::show::pptxr::ppt::CommentIdContext::CommentIdContext;
// Java: import cn.wps.show.pptxr.ppt.CommentIdContext

/// `p:cmAuthor` 的元素 id（`kfc/xsd/pml/P.java`）
pub const CM_AUTHOR: i32 = 3211453;
/// `p:cmAuthor/@id`
pub const CM_AUTHOR_ID: i32 = 3211342;
/// `p:cmAuthor/@name`
pub const CM_AUTHOR_NAME: i32 = 3211449;
/// `p:cmAuthor/@initials`
pub const CM_AUTHOR_INITIALS: i32 = 3211450;
/// `p:cmAuthor/@lastIdx`
pub const CM_AUTHOR_LAST_IDX: i32 = 3211451;
/// `p:cmAuthor/@clrIdx`（Java 里叫 `colorIndex`）
pub const CM_AUTHOR_CLR_IDX: i32 = 3211452;

/// `java cn.wps.show.data.Presentation.CommentAuthor` 的 Rust 端口
/// （`cn.wps.show.data.Presentation` 在 default feature 下不可达，见台账）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommentAuthor {
    // java private int id;
    pub id: i32,
    // java private String name;
    pub name: String,
    // java private String initials;
    pub initials: String,
    // java private int colorIndex;
    pub colorIndex: i32,
    // java private int lastCommentIndex;
    pub lastCommentIndex: i32,
}

impl CommentAuthor {
    // java static CommentAuthor create()
    pub fn create() -> Self {
        Self::default()
    }
    // java int id()
    pub fn id(&self) -> i32 {
        self.id
    }
    // java String initials()
    pub fn initials(&self) -> String {
        self.initials.clone()
    }
    // java void putId(int id)
    pub fn putId(&mut self, id: i32) {
        self.id = id;
    }
    // java void putName(String name)
    pub fn putName(&mut self, name: String) {
        self.name = name;
    }
    // java void putInitials(String initials)
    pub fn putInitials(&mut self, initials: String) {
        self.initials = initials;
    }
    // java void putColorIndex(int colorIndex)
    pub fn putColorIndex(&mut self, colorIndex: i32) {
        self.colorIndex = colorIndex;
    }
    // java void putLastCommentIndex(int lastCommentIndex)
    pub fn putLastCommentIndex(&mut self, lastCommentIndex: i32) {
        self.lastCommentIndex = lastCommentIndex;
    }
}

/// `java cn.wps.show.data.Presentation.CommentAuthorLst` 的 Rust 端口。
#[derive(Clone, Debug, Default)]
pub struct CommentAuthorLst {
    // java private List<CommentAuthor> commentAuthors = new ArrayList();
    pub commentAuthors: Vec<CommentAuthor>,
}

impl CommentAuthorLst {
    pub fn size(&self) -> i32 {
        self.commentAuthors.len() as i32
    }
}

// java public class CommentAuthorListHandler extends DefaultElementHandler
pub struct CommentAuthorListHandler {
    // java private CommentAuthorLst mCommentAuthorLst;
    mCommentAuthorLst: CommentAuthorLst,
    // java private HandlerCallback mHandlerCallback;
    // rust: 见 DEFERRED 说明
    // java private CommentIdContext mCommentIdCxt;
    mCommentIdCxt: CommentIdContext,
    // java private List<CommentAuthor> commentAuthors;
    commentAuthors: Vec<CommentAuthor>,
    // java private CommentAuthor commentAuthor;
    commentAuthor: CommentAuthor,
}

impl CommentAuthorListHandler {
    // java public CommentAuthorListHandler(CommentAuthorLst commentAuthorLst, HandlerCallback handlerCallback, CommentIdContext commentIdCxt)
    pub fn CommentAuthorListHandler(
        commentAuthorLst: CommentAuthorLst,
        commentIdCxt: CommentIdContext,
    ) -> Self {
        // java this.mCommentAuthorLst = commentAuthorLst;
        // java this.mHandlerCallback = handlerCallback;
        // java this.mCommentIdCxt = commentIdCxt;
        let commentAuthors = commentAuthorLst.commentAuthors.clone();
        Self {
            mCommentAuthorLst: commentAuthorLst,
            mCommentIdCxt: commentIdCxt,
            commentAuthors,
            commentAuthor: CommentAuthor::default(),
        }
    }

    // java public void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, element: i32, attrsNotCopy: &XmlRoAttrList) {
        // java XmlRoAttr temp = null;
        // java if ((temp = attrsNotCopy.getByName(3211342)) != null) { ... }
        let ioId = attrsNotCopy.get_by_name(CM_AUTHOR_ID).map(|t| t.to_int());
        let initials = attrsNotCopy
            .get_by_name(CM_AUTHOR_INITIALS)
            .map(|t| t.to_string());
        // java CommentAuthor commentAuthor = this.createCommentAuthor(ioId, initials);
        let created = self.createCommentAuthor(ioId, initials);
        // java if (commentAuthor != null)
        if let Some(author) = created {
            self.commentAuthor = author;
        }
        // java if ((tmp = attrsNotCopy.getByName(3211452)) != null) { this.commentAuthor.putColorIndex(tmp.toInt()); }
        if let Some(tmp) = attrsNotCopy.get_by_name(CM_AUTHOR_CLR_IDX) {
            let v = tmp.to_int();
            self.commentAuthor.putColorIndex(v);
        }
        // java if ((tmp = attrsNotCopy.getByName(3211451)) != null) { this.commentAuthor.putLastCommentIndex(tmp.toInt()); }
        if let Some(tmp) = attrsNotCopy.get_by_name(CM_AUTHOR_LAST_IDX) {
            let v = tmp.to_int();
            self.commentAuthor.putLastCommentIndex(v);
        }
        // java if ((tmp = attrsNotCopy.getByName(3211449)) != null) { this.commentAuthor.putName(tmp.toString()); }
        if let Some(tmp) = attrsNotCopy.get_by_name(CM_AUTHOR_NAME) {
            let v = tmp.to_string();
            self.commentAuthor.putName(v);
        }
    }

    // java private CommentAuthor createCommentAuthor(Integer ioId, String initials)
    fn createCommentAuthor(&mut self, ioId: Option<i32>, initials: Option<String>) -> Option<CommentAuthor> {
        // java CommentAuthor ret = null;
        // java if (null != initials)
        if let Some(initials) = initials {
            // java for (CommentAuthor author : CommentAuthorListHandler.this.commentAuthors)
            for author in self.commentAuthors.iter() {
                // java if (author.initials().equals(initials))
                if author.initials() == initials {
                    // java CommentAuthorListHandler.this.mCommentIdCxt.put(ioId, author.id());
                    if let Some(ioId) = ioId {
                        self.mCommentIdCxt.put(ioId, author.id());
                    }
                    // java return null;
                    return None;
                }
            }
        }
        // java if (null == ret) { ret = CommentAuthor.create(); CommentAuthorListHandler.this.commentAuthors.add(ret); }
        let ret = CommentAuthor::create();
        self.commentAuthors.push(ret.clone());
        // java return ret;
        Some(ret)
    }

    // java public void endElement(int element) —— 把新建的 author 落进 CommentAuthorLst
    pub fn endElement(&mut self, _element: i32) {
        // Java: `commentAuthors` 与 `mCommentAuthorLst.commentAuthors` 是同一个列表引用，
        // 端口里用 `CommentAuthorLst` 的 `Vec` 直接承载，故此处只做落盘同步。
        self.mCommentAuthorLst.commentAuthors = self.commentAuthors.clone();
    }

    // rust: 可观测面（Java 无此方法）
    pub fn commentAuthorLst(&self) -> CommentAuthorLst {
        let mut lst = self.mCommentAuthorLst.clone();
        lst.commentAuthors = self.commentAuthors.clone();
        lst
    }
    pub fn into_commentIdContext(self) -> CommentIdContext {
        self.mCommentIdCxt
    }
}

impl ElementHandler for CommentAuthorListHandler {
    fn kind(&self) -> &'static str {
        "CommentAuthorListHandler"
    }
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList) {
        CommentAuthorListHandler::startElement(self, element, attrs)
    }
    fn enterSubElement(&mut self, _subElement: i32) -> Option<Box<dyn ElementHandler>> {
        None
    }
    fn endElement(&mut self, element: i32) {
        CommentAuthorListHandler::endElement(self, element)
    }
}
