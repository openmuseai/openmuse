#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::cell::RefCell;
use std::io;
use std::rc::Rc;

// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrPackagePartRef;
// Java: import cn.wps.show.pptxr.handler.CommentAuthorListHandler
use crate::cn::wps::show::pptxr::handler::CommentAuthorListHandler::{
    CommentAuthorLst, CommentAuthorListHandler,
};
// Java: import cn.wps.show.pptxr.util.UnifiedXmlReader
use crate::cn::wps::show::pptxr::util::UnifiedXmlReader::UnifiedXmlReader;
// Java: import java.io.IOException
// Java: import java.io.InputStream
use super::CommentIdContext::CommentIdContext;

// java public class PptxrCommentAuthors
pub struct PptxrCommentAuthors {
    // java private CommentAuthorLst mCommentAuthorLst;
    mCommentAuthorLst: CommentAuthorLst,
    // java private OpcrPackagePart mPart;
    mPart: Option<OpcrPackagePartRef>,
    // java private CommentIdContext mCommentIdCxt;
    mCommentIdCxt: CommentIdContext,
    // rust: 真实解析出的作者列表（可观测面）
    mParsed: CommentAuthorLst,
}

// java public class PptxrCommentAuthors
impl PptxrCommentAuthors {
    // java public PptxrCommentAuthors(CommentAuthorLst commentAuthorLst, OpcrPackagePart part, CommentIdContext commentIdCxt)
    pub fn PptxrCommentAuthors(
        commentAuthorLst: CommentAuthorLst,
        part: Option<OpcrPackagePartRef>,
        commentIdCxt: CommentIdContext,
    ) -> Self {
        // java this.mCommentAuthorLst = commentAuthorLst;
        // java this.mPart = part;
        // java this.mCommentIdCxt = commentIdCxt;
        Self {
            mCommentAuthorLst: commentAuthorLst,
            mPart: part,
            mCommentIdCxt: commentIdCxt,
            mParsed: CommentAuthorLst::default(),
        }
    }

    // java public void open() throws IOException
    pub fn open(&mut self) -> io::Result<()> {
        // java if (this.mPart != null)
        if let Some(part) = self.mPart.clone() {
            // java InputStream stream = this.mPart.getInputStream();
            let mut stream = part.getInputStream_default()?;
            // java CommentAuthorListHandler handler = new CommentAuthorListHandler(this.mCommentAuthorLst, null, this.mCommentIdCxt);
            let mut handler = CommentAuthorListHandler::CommentAuthorListHandler(
                self.mCommentAuthorLst.clone(),
                self.mCommentIdCxt.clone(),
            );
            // java UnifiedXmlReader.read((InputStream)stream, handler);
            UnifiedXmlReader::read__stream_elementhandler(&mut stream, &mut handler)?;
            // java this.mCommentAuthorLst 由 handler 落盘
            self.mParsed = handler.commentAuthorLst();
            self.mCommentAuthorLst = self.mParsed.clone();
            self.mCommentIdCxt = handler.into_commentIdContext();
        }
        Ok(())
    }

    // rust: 可观测面（Java 无此方法）
    pub fn commentAuthorLst(&self) -> CommentAuthorLst {
        self.mParsed.clone()
    }
    pub fn commentIdContext(&self) -> CommentIdContext {
        self.mCommentIdCxt.clone()
    }
}
