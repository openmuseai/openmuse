// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::*;
// Java: import cn.wps.base.log.IOLog
use util::base::log::IOLog::*;
// Java: import cn.wps.base.log.Log
use util::base::log::Log::*;
// Java: import cn.wps.base.utils.StringUtils
use util::base::utils::StringUtils::*;
// Java: import cn.wps.graphics.RectF
use util::graphics::RectF::*;
// Java: import cn.wps.moffice.drawing.OLE
use drawing::cn::wps::moffice::drawing::OLE::*;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::*;
// Java: import cn.wps.moffice.drawing.graphics.Picture
use drawing::cn::wps::moffice::drawing::graphics::Picture::*;
// Java: import cn.wps.moffice.drawing.media.MediaLib
use drawing::cn::wps::moffice::drawing::media::MediaLib::*;
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.VersionState
use crate::cn::wps::moffice::writer::VersionState::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.WriterModelException
use crate::cn::wps::moffice::writer::core::WriterModelException::*;
// Java: import cn.wps.moffice.writer.core.shape.KInkCommentShape
use crate::cn::wps::moffice::writer::core::shape::KInkCommentShape::*;
// Java: import cn.wps.moffice.writer.core.shape.KInlineShapes
use crate::cn::wps::moffice::writer::core::shape::KInlineShapes::*;
// Java: import cn.wps.moffice.writer.core.shape.KShape
use crate::cn::wps::moffice::writer::core::shape::KShape::*;
// Java: import cn.wps.moffice.writer.core.shape.KShapeRange
use crate::cn::wps::moffice::writer::core::shape::KShapeRange::*;
// Java: import cn.wps.moffice.writer.core.shape.KShapes
use crate::cn::wps::moffice::writer::core::shape::KShapes::*;
// Java: import cn.wps.moffice.writer.core.shape.ShapeTool
use crate::cn::wps::moffice::writer::core::shape::ShapeTool::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCBookmarkEnd
use crate::cn::wps::moffice::writer::data::PLCBookmarkEnd::*;
// Java: import cn.wps.moffice.writer.data.PLCBookmarkStart
use crate::cn::wps::moffice::writer::data::PLCBookmarkStart::*;
// Java: import cn.wps.moffice.writer.data.PLCCommentRef
use crate::cn::wps::moffice::writer::data::PLCCommentRef::*;
// Java: import cn.wps.moffice.writer.data.PLCCommentTxt
use crate::cn::wps::moffice::writer::data::PLCCommentTxt::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.field.FieldCodeLex
use crate::cn::wps::moffice::writer::data::field::FieldCodeLex::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.field.exception.UnKnownSwitchAgrument
use crate::cn::wps::moffice::writer::data::field::exception::UnKnownSwitchAgrument::*;
// Java: import cn.wps.moffice.writer.data.value.DateTime
use crate::cn::wps::moffice::writer::data::value::DateTime::*;
// Java: import com.hp.hpl.inkml.Ink
use thirdparty_kInk::com::hp::hpl::inkml::Ink::*;
// Java: import java.security.SecureRandom
use stubs::java::security::SecureRandom::*;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.poi.hwpf.ole.stream.AudioCommentStreamParser
use io_poi_compact::org::apache::poi::hwpf::ole::stream::AudioCommentStreamParser::*;
// Java: import org.apache.poi.hwpf.util.OleParseInterruptException
use io_poi_compact::org::apache::poi::hwpf::util::OleParseInterruptException::*;
// Java: import cn.wps.moffice.writer.core.comment.CommentInfo
use crate::cn::wps::moffice::writer::core::comment::CommentInfo::*;
// Java: import cn.wps.moffice.writer.core.comment.CommentItem
use crate::cn::wps::moffice::writer::core::comment::CommentItem::*;
// Java: import PLCCommentRef.CommentRefNode  (via PLCCommentRef::* above)


// java public static SecureRandom r = new SecureRandom();
pub(crate) static r: std::sync::LazyLock<std::sync::Arc<Option<SecureRandom>>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(SecureRandom::new()));

// java public class KComment
#[derive(Default)]
pub struct KComment {
    // java Document document;
    pub(crate) document: Option<Document>,
    // java PLCBookmarkStart.BookmarkStartNode commentStart;
    pub(crate) commentStart: Option<PLCBookmarkStart::BookmarkStartNode>,
    // java PLCBookmarkEnd.BookmarkEndNode commentEnd;
    pub(crate) commentEnd: Option<PLCBookmarkEnd::BookmarkEndNode>,
    // java PLCCommentRef.CommentRefNode commentRef;
    pub(crate) commentRef: Option<PLCCommentRef::CommentRefNode>,
    // java PLCCommentTxt.CommentTxtNode commentTxt;
    pub(crate) commentTxt: Option<PLCCommentTxt::CommentTxtNode>,
    // java CommentInfo commentInfo;
    pub(crate) commentInfo: Option<CommentInfo>,
}

// java public class KComment
impl KComment {
    // java   KComment(Document document, int ref)
    pub fn new__document_int(document: &dyn Document, r#ref: i32) -> Self {
      // java this.document = document.getMainDocument();
      let mut this = Self { document: document.getMainDocument(), commentStart: self.commentRef.getCommentStart(), commentEnd: self.commentRef.getCommentEnd(), commentRef: document.getCommentRef().findCommentRefByCP(r#ref), commentTxt: self.commentRef.getCommentTxt(), commentInfo: Default::default() };
      // java this.commentRef = document.getCommentRef().findCommentRefByCP(ref);
      // java Assert.assertNotNull("commentRef should not be nul!", this.commentRef);
      Assert::assertNotNull__string_object(&"commentRef should not be nul!", Some(&this.commentRef));
      // java this.commentTxt = this.commentRef.getCommentTxt();
      // java this.commentStart = this.commentRef.getCommentStart();
      // java this.commentEnd = this.commentRef.getCommentEnd();
      // java Assert.assertNotNull("commentStart should not be nul!", this.commentStart);
      Assert::assertNotNull__string_object(&"commentStart should not be nul!", Some(&this.commentStart));
      // java Assert.assertNotNull("commentEnd should not be nul!", this.commentEnd);
      Assert::assertNotNull__string_object(&"commentEnd should not be nul!", Some(&this.commentEnd));
      // java Assert.assertNotNull("commentTxt should not be nul!", this.commentTxt);
      Assert::assertNotNull__string_object(&"commentTxt should not be nul!", Some(&this.commentTxt));
      this
    }

    // java   KComment(KRange range, CommentItem item)
    pub fn new__krange_commentitem(range: &KRange, item: &CommentItem) -> Self {
      // java this.document = range.getMainDocument();
      let mut this = Self { document: range.getMainDocument(), commentStart: self.addCommentStartNode(range.getStart()), commentEnd: self.addCommentEndNode(end), commentRef: self.addCommentRefNode(end), commentTxt: self.addEmptyCommentContent(&commentDocument, end), commentInfo: Default::default() };
      // java int end = this.expandCommentRange(range.getEnd(), item);
      let mut end: i32 = this.expandCommentRange(range.getEnd(), &item);
      // java Document commentDocument = this.getCommentDocument();
      let mut commentDocument: Box<dyn Document> = this.getCommentDocument();
      // java this.commentTxt = this.addEmptyCommentContent(commentDocument, end);
      // java this.insertRefChar(end);
      this.insertRefChar(end);
      // java this.commentRef = this.addCommentRefNode(end);
      // java this.commentStart = this.addCommentStartNode(range.getStart());
      // java this.commentEnd = this.addCommentEndNode(end);
      // java this.initCommentData(item);
      this.initCommentData(&item);
      // java this.resetRelationships();
      this.resetRelationships();
      // java this.initCommentInfo(item);
      this.initCommentInfo(&item);
      // java this.addReplayInfo(item);
      this.addReplayInfo(&item);
      this
    }

    // java private void addReplayInfo(CommentItem item)
    pub fn addReplayInfo(&mut self, item: &CommentItem) {
      // java KComment parent = item.parent;
      let mut parent: KComment = item.parent;
      // java if (null != parent)
      if parent.is_some() {
        // java if (null == parent.commentRef.getParentRefNode())
        if parent.commentRef.getParentRefNode().is_none() {
          // java this.commentRef.setParentRefNode(parent.commentRef);
          self.commentRef.setParentRefNode__object(&parent.commentRef);
        }
        // java else
        else {
          // java this.commentRef.setParentRefNode(parent.commentRef.getParentRefNode());
          self.commentRef.setParentRefNode(&parent.commentRef.getParentRefNode());
        }
      }
    }

    // java  void initCommentData(CommentItem item)
    pub fn initCommentData(&mut self, item: &CommentItem) {
    }

    // java private void initCommentInfo(CommentItem item)
    pub fn initCommentInfo(&mut self, item: &CommentItem) {
      // java this.commentRef.setAuthor(item.author);
      self.commentRef.setAuthor__object(&item.author);
      // java this.commentRef.setAbbreviation(item.abbreviation);
      self.commentRef.setAbbreviation__object(&item.abbreviation);
      // java this.commentRef.setDateTime(item.dateTime);
      self.commentRef.setDateTime__object(&item.dateTime);
      // java int tag = r.nextInt(10000000);
      let mut tag: i32 = r.nextInt(10000000);
      // java this.commentRef.setTag(tag);
      self.commentRef.setTag__object(&tag);
    }

    // java private void resetRelationships()
    pub fn resetRelationships(&mut self) {
      // java this.commentStart.setEnd(this.commentEnd);
      self.commentStart.setEnd__object(&self.commentEnd);
      // java this.commentEnd.setStart(this.commentStart);
      self.commentEnd.setStart__object(&self.commentStart);
      // java this.commentTxt.setCommentRef(this.commentRef);
      self.commentTxt.setCommentRef(self.commentRef);
      // java this.commentRef.setCommentTxt(this.commentTxt);
      self.commentRef.setCommentTxt__object(&self.commentTxt);
      // java this.commentRef.setCommentStart(this.commentStart);
      self.commentRef.setCommentStart__object(&self.commentStart);
      // java this.commentStart.setCommentRef(this.commentRef);
      self.commentStart.setCommentRef__object(&self.commentRef);
      // java this.commentRef.setCommentEnd(this.commentEnd);
      self.commentRef.setCommentEnd__object(&self.commentEnd);
      // java this.commentEnd.setCommentRef(this.commentRef);
      self.commentEnd.setCommentRef__object(&self.commentRef);
    }

    // java public void insertRefChar(int cp)
    pub fn insertRefChar(&self, cp: i32) {
      // java this.document.getTextStream().insertPlaceHolder(cp, '\u0005', PropertySet.valueOf(28, true));
      self.document.getTextStream().insertPlaceHolder(cp, '\u{0005}', PropertySet::valueOf__int_object(28, &true));
    }

    // java private PLCCommentRef.CommentRefNode addCommentRefNode(int cp)
    pub fn addCommentRefNode(&self, cp: i32) -> PLCCommentRef::CommentRefNode {
      // java return this.document.getCommentRef().addCommentRef(cp);
      return self.document.getCommentRef().addCommentRef(cp);
    }

    // java private PLCBookmarkStart.BookmarkStartNode addCommentStartNode(int cp)
    pub fn addCommentStartNode(&self, cp: i32) -> PLCBookmarkStart::BookmarkStartNode {
      // java return this.document.getCommentStarts().addBookmarkStart(cp);
      return self.document.getCommentStarts().addBookmarkStart(cp);
    }

    // java private PLCBookmarkEnd.BookmarkEndNode addCommentEndNode(int cp)
    pub fn addCommentEndNode(&self, cp: i32) -> PLCBookmarkEnd::BookmarkEndNode {
      // java return this.document.getCommentEnds().addBookmarkEnd(cp);
      return self.document.getCommentEnds().addBookmarkEnd(cp);
    }

    // java  void start()
    pub fn start(&self) {
      // java this.document.getTextDocument().start();
      self.document.getTextDocument().start();
    }

    // java  void commit(String description)
    pub fn commit(&self, description: &str) {
      // java this.document.getTextDocument().commit(description);
      self.document.getTextDocument().commit(description);
    }

    // java private int expandCommentRange(int nCommentEnd, CommentItem item)
    pub fn expandCommentRange(&self, nCommentEnd: i32, item: &CommentItem) -> Option<i32> {
      // java IChpxTable pChpxTable = this.document.getChpxTable();
      let mut pChpxTable: Box<dyn IChpxTable> = self.document.getChpxTable();
      // java KComment parent = item.getParent();
      let mut parent: KComment = item.getParent();
      // java PLCCommentRef.CommentRefNode parentRefNode = null == parent ? null : parent.commentRef;
      let mut parentRefNode: PLCCommentRef::CommentRefNode = (if parent.is_none() { PLCCommentRef::CommentRefNode::default() } else { parent.commentRef });
      // java while (this.isCommentRef(nCommentEnd, pChpxTable))
      while self.isCommentRef(nCommentEnd, &mut pChpxTable) {
        // java if (parentRefNode == null)
        if parentRefNode.is_none() {
          // java ++nCommentEnd;
          { let _tmp = nCommentEnd; nCommentEnd += 1; _tmp };
        }
        // java else
        else {
          // java PLCCommentRef.CommentRefNode refNode = this.document.getCommentRef().findCommentRefByCP(nCommentEnd);
          let mut refNode: PLCCommentRef::CommentRefNode = self.document.getCommentRef().findCommentRefByCP(nCommentEnd);
          // java if (refNode != parentRefNode && refNode.getParentRefNode() != parentRefNode)
          if refNode != parentRefNode && refNode.getParentRefNode() != parentRefNode {
            // java break;
            break;
          }
          // java ++nCommentEnd;
          { let _tmp = nCommentEnd; nCommentEnd += 1; _tmp };
        }
      }
      // java return nCommentEnd;
      return nCommentEnd;
    }

    // java private boolean isCommentRef(int cp, IChpxTable pChpxTable)
    pub fn isCommentRef(&mut self, cp: i32, pChpxTable: &mut dyn IChpxTable) -> bool {
      // java return this.document.charAt(cp) == 5 && pChpxTable.seek(cp).getProperty().getBoolean(28, false);
      return self.document.charAt(cp) == 5 && pChpxTable.seek(cp).getProperty().getBoolean(28, false);
    }

    // java private PLCCommentTxt.CommentTxtNode addEmptyCommentContent(Document pCommentDocument, int nRefPos)
    pub fn addEmptyCommentContent(&self, pCommentDocument: &dyn Document, nRefPos: i32) -> PLCCommentTxt::CommentTxtNode {
      // java PLCCommentTxt.CommentTxtNode nextCommentTxt = null;
      let mut nextCommentTxt: PLCCommentTxt::CommentTxtNode = PLCCommentTxt::CommentTxtNode::default();
      // java PLCBookmarkEnd.BookmarkEndNode nextCommentEnd = this.document.getCommentEnds().findLowerBookmarkEndByCP(nRefPos);
      let mut nextCommentEnd: PLCBookmarkEnd::BookmarkEndNode = self.document.getCommentEnds().findLowerBookmarkEndByCP(nRefPos);
      // java if (nextCommentEnd != null)
      if nextCommentEnd.is_some() {
        // java nextCommentTxt = nextCommentEnd.getCommentRef().getCommentTxt();
        nextCommentTxt = nextCommentEnd.getCommentRef().getCommentTxt();
      }
      // java int nCommentContent = pCommentDocument.getLength();
      let mut nCommentContent: i32 = pCommentDocument.getLength();
      // java if (nextCommentTxt != null)
      if nextCommentTxt.is_some() {
        // java nCommentContent = nextCommentTxt.position();
        nCommentContent = nextCommentTxt.position();
      }
      // java pCommentDocument.getTextStream().insertParagraph(nCommentContent, '\r', PropertySet.empty, PropertySet.empty);
      pCommentDocument.getTextStream().insertParagraph(nCommentContent, '\r', PropertySet::empty(), PropertySet::empty());
      // java if (nextCommentTxt != null)
      if nextCommentTxt.is_some() {
        // java KSAssert.assertEquals(nCommentContent, nextCommentTxt.position());
        KSAssert::assertEquals(nCommentContent, nextCommentTxt.position());
        // java pCommentDocument.getCommentTxt().moveRight(nextCommentTxt, 1);
        pCommentDocument.getCommentTxt().moveRight(nextCommentTxt, 1);
      }
      // java return pCommentDocument.getCommentTxt().addCommentTxt(nCommentContent);
      return pCommentDocument.getCommentTxt().addCommentTxt(nCommentContent);
    }

    // java public int getCommentRefCp()
    pub fn getCommentRefCp(&self) -> i32 {
      // java return this.commentRef.getCP();
      return self.commentRef.getCP();
    }

    // java public int getCommentEndCp()
    pub fn getCommentEndCp(&self) -> i32 {
      // java return this.commentRef.getCommentEnd().getCP();
      return self.commentRef.getCommentEnd().getCP();
    }

    // java public int getCommentStartCp()
    pub fn getCommentStartCp(&self) -> i32 {
      // java return this.commentRef.getCommentStart().getCP();
      return self.commentRef.getCommentStart().getCP();
    }

    // java public Document getMainDocument()
    pub fn getMainDocument(&self) -> Box<dyn Document> {
      // java return this.document;
      return self.document;
    }

    // java public Document getCommentDocument()
    pub fn getCommentDocument(&self) -> Box<dyn Document> {
      // java return this.document.getTextDocument().createSubDocument(3);
      return self.document.getTextDocument().createSubDocument(3);
    }

    // java private String getAbbreviation(String author)
    pub fn getAbbreviation(&self, author: &mut str) -> String {
      // java int abbrLen = Math.min(author.length(), CommentItem.Abbreviation_MAX_LENGTH);
      let mut abbrLen: i32 = Math::min((author.len() as i32), CommentItem::Abbreviation_MAX_LENGTH);
      // java if (abbrLen > 0 && Character.isHighSurrogate(author.charAt(abbrLen - 1)))
      if abbrLen > 0 && Character::isHighSurrogate(author.charAt(abbrLen - 1)) {
        // java --abbrLen;
        { let _tmp = abbrLen; abbrLen -= 1; _tmp };
      }
      // java return author.substring(0, abbrLen);
      return author.substring(0, abbrLen);
    }

    // java public boolean editComment(String author, String text)
    pub fn editComment__string_string(&mut self, author: &str, text: Option<&mut str>) -> bool {
      // java boolean result = false;
      let mut result: bool = false;
      // java this.start();
      self.start();
          // java KRange range = this.getCommentRange();
          let mut range: KRange = self.getCommentRange();
          // java if (null != range)
          if range.is_some() {
            // java this.setAuthor(author);
            self.setAuthor(&author);
            // java if (range.getLength() > 0 && null != text && text.length() > 0)
            if let Some(text) = text {
              if range.getLength() > 0 && (text.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 {
                // java char c = text.charAt(0);
                let mut c: char = text.charAt(0);
                // java if (c != 5 && this.document.charAt(range.getStart()) == 5)
                if c != 5 && self.document.charAt(range.getStart()) == 5 {
                  // java range.setRange(range.getStart() + 1, range.getEnd());
                  range.setRange(range.getStart() + 1, range.getEnd());
                }
              }
            }
            // java int length = range.getLength();
            let mut length: i32 = range.getLength();
            // java range.setRange(range.getStart(), range.getStart());
            range.setRange(range.getStart(), range.getStart());
            // java range.insertText(text);
            range.insertText(text);
            // java range.setRange(range.getEnd(), range.getEnd() + length);
            range.setRange(range.getEnd(), range.getEnd() + length);
            // java range.deleteRange();
            range.deleteRange();
            // java result = true;
            result = true;
          }
            // java this.commit("edit comment");
            self.commit(&"edit comment");
            // java this.onEditComment();
            self.onEditComment();
      // java return result;
      return result;
    }

    // java public boolean editComment(CommentInfo commentInfo)
    pub fn editComment__commentinfo(&mut self, commentInfo: &CommentInfo) -> bool {
      // java String author = commentInfo.getAuthor();
      let mut author: String = commentInfo.getAuthor();
      // java String text = commentInfo.getText();
      let mut text: String = commentInfo.getText();
      // java List<Shape> shapes = commentInfo.getShapes();
      let mut shapes: Vec<Shape> = commentInfo.getShapes();
      // java return this.editComment(author, text, shapes);
      return self.editComment__string_string_listshape(Some(&author), &mut text, Some(&shapes));
    }

    // java public boolean editComment(String author, String text, List<Shape> shapes)
    pub fn editComment__string_string_listshape(&mut self, author: Option<&str>, text: &mut str, shapes: Option<&Vec<Shape>>) -> bool {
      // java boolean result = false;
      let mut result: bool = false;
      // java this.start();
      self.start();
          // java KRange range = this.getCommentRange();
          let mut range: KRange = self.getCommentRange();
          // java if (null != range)
          if range.is_some() {
            // java if (null != author)
            if let Some(author) = author {
              // java this.setAuthor(author);
              self.setAuthor(&author);
            }
            // java Document subDocument = range.getSubDocument();
            let mut subDocument: Box<dyn Document> = range.getSubDocument();
            // java if (range.getLength() > 0 && !StringUtils.isEmpty(text))
            if range.getLength() > 0 && !StringUtils::isEmpty(Some(&text)) {
              // java char c = text.charAt(0);
              let mut c: char = text.charAt(0);
              // java if (c != 5 && subDocument.charAt(range.getStart()) == 5)
              if c != 5 && subDocument.charAt(range.getStart()) == 5 {
                // java range.setRange(range.getStart() + 1, range.getEnd());
                range.setRange(range.getStart() + 1, range.getEnd());
              }
            }
            // java int length = range.getLength();
            let mut length: i32 = range.getLength();
            // java range.setRange(range.getStart(), range.getStart());
            range.setRange(range.getStart(), range.getStart());
            // java range.insertText(text);
            range.insertText(text);
            // java TextDocument textDocument = subDocument.getTextDocument();
            let mut textDocument: TextDocument = subDocument.getTextDocument();
            // java if (null != shapes && !shapes.isEmpty())
            if let Some(shapes) = shapes {
              if !shapes.is_empty() {
                // java if (!StringUtils.isEmpty(text))
                if !StringUtils::isEmpty(Some(&text)) {
                  // java subDocument.getTextStream().insertParagraph(range.getStart(), '\r', PropertySet.empty, PropertySet.empty);
                  subDocument.getTextStream().insertParagraph(range.getStart(), '\r', PropertySet::empty(), PropertySet::empty());
                  // java range.setRange(range.getStart() + 1, range.getEnd() + 1);
                  range.setRange(range.getStart() + 1, range.getEnd() + 1);
                }
                // java MediaLib mediaLib = textDocument.getMediaLib();
                let mut mediaLib: MediaLib = textDocument.getMediaLib();
                // java for(Shape shape : shapes)
                for shape in shapes.iter() {
                  // java if (shape.hasInk())
                  if shape.hasInk() {
                    // java KShapeRange shapeRange = new KShapeRange(subDocument, range);
                    let mut shapeRange: KShapeRange = KShapeRange::new(subDocument, range);
                    // java KShapes kShapes = new KShapes(shapeRange);
                    let mut kShapes: KShapes = KShapes::new(shapeRange);
                    // java kShapes.addInkComment(range, shape);
                    kShapes.addInkComment(range, shape);
                  }
                  // java else if (ShapeTool.IsOleShape(subDocument, shape))
                  else if ShapeTool::IsOleShape(&subDocument, &shape) {
                    // java KShapeRange shapeRange = new KShapeRange(subDocument, range);
                    let mut shapeRange: KShapeRange = KShapeRange::new(subDocument, range);
                    // java KShapes kShapes = new KShapes(shapeRange);
                    let mut kShapes: KShapes = KShapes::new(shapeRange);
                    // java kShapes.addOleComment(range, shape);
                    kShapes.addOleComment(range, shape);
                  }
                  // java else if (shape.isPicture())
                  else if shape.isPicture() {
                    // java KInlineShapes inlineShapes = new KInlineShapes();
                    let mut inlineShapes: KInlineShapes = KInlineShapes::new();
                    // java Picture picture = shape.getPicture();
                    let mut picture: Picture = shape.getPicture();
                    // java String tmpImg = mediaLib.getMediaByID(picture.getMediaID(), MediaTypeEnum.PICTURE);
                    let mut tmpImg: String = mediaLib.getMediaByID(picture.getMediaID(), MediaTypeEnum::PICTURE);
                    // java inlineShapes.addPicture(tmpImg, false, true, range);
                    inlineShapes.addPicture(tmpImg, false, true, range);
                  }
                  // java else
                  else {
                    // java Assert.fail("It's unreachable!");
                    Assert::fail__string(&"It's unreachable!");
                  }
                }
              }
            }
            // java range.setRange(range.getEnd(), range.getEnd() + length);
            range.setRange(range.getEnd(), range.getEnd() + length);
            // java range.deleteRangeDirectly();
            range.deleteRangeDirectly();
            // java result = true;
            result = true;
          }
            // java this.commit("edit comment");
            self.commit(&"edit comment");
            // java this.onEditComment();
            self.onEditComment();
      // java return result;
      return result;
    }

    // java public boolean editInkComment(String author, Ink ink, RectF cropRect, String tmpImage)
    pub fn editInkComment(&mut self, author: &str, ink: &Ink, cropRect: &RectF, tmpImage: &str) -> bool {
      // java boolean result = false;
      let mut result: bool = false;
      // java this.start();
      self.start();
          // java Shape shape = this.getInkCommentShape();
          let mut shape: Shape = self.getInkCommentShape();
          // java if (null != shape)
          if shape.is_some() {
            // java KRange commentRange = this.getCommentRange();
            let mut commentRange: KRange = self.getCommentRange();
            // java if (null != commentRange)
            if commentRange.is_some() {
              // java this.setAuthor(author);
              self.setAuthor(&author);
              // java KShapeRange shapeRange = new KShapeRange(commentRange.getSubDocument(), commentRange);
              let mut shapeRange: KShapeRange = KShapeRange::new(commentRange.getSubDocument(), commentRange);
              // java shapeRange.setcurEditShape(shape);
              shapeRange.setcurEditShape(shape);
              // java KInkCommentShape commentShape = new KInkCommentShape(shapeRange, commentRange);
              let mut commentShape: KInkCommentShape = KInkCommentShape::new(shapeRange, commentRange);
              // java commentShape.commitEditInkComment(ink, cropRect, tmpImage);
              commentShape.commitEditInkComment(ink, cropRect, tmpImage);
              // java result = true;
              result = true;
            }
          }
            // java this.commit("edit comment");
            self.commit(&"edit comment");
            // java this.onEditComment();
            self.onEditComment();
      // java return result;
      return result;
    }

    // java public boolean editComment(RectF rectF, String iconPath, String olePath)
    pub fn editComment__rectf_string_string(&mut self, rectF: &RectF, iconPath: &str, olePath: &str) -> bool {
      // java boolean result = false;
      let mut result: bool = false;
      // java this.start();
      self.start();
          // java Shape shape = this.getCommentShape();
          let mut shape: Shape = self.getCommentShape();
          // java if (null != shape)
          if shape.is_some() {
            // java KShape kShape = new KShape(shape);
            let mut kShape: KShape = KShape::new(shape);
            // java kShape.updateOle(rectF, olePath, iconPath);
            kShape.updateOle(rectF, olePath, iconPath);
            // java String expectProgId = ShapeTool.getPenKitName(VersionState.getDeviceBrand());
            let mut expectProgId: String = ShapeTool::getPenKitName(Some(&mut VersionState::getDeviceBrand()));
            // java KRange commentRange = this.getCommentRange();
            let mut commentRange: KRange = self.getCommentRange();
            // java int start = commentRange.getStart();
            let mut start: i32 = commentRange.getStart();
            // java int end = commentRange.getEnd();
            let mut end: i32 = commentRange.getEnd();
            // java this.updateCommentField(expectProgId, (SubDocument)this.getCommentDocument(), start, end);
            self.updateCommentField(Some(&expectProgId), &(self.getCommentDocument()).downcast_ref::<SubDocument>().unwrap(), start, end);
            // java result = true;
            result = true;
          }
            // java this.commit("edit comment");
            self.commit(&"edit comment");
            // java this.onEditComment();
            self.onEditComment();
      // java return result;
      return result;
    }

    // java private void updateCommentField(String progId, SubDocument document, int start, int end)
    pub fn updateCommentField(&mut self, progId: Option<&str>, document: &SubDocument, start: i32, end: i32) {
      // java if (progId != null && progId.length() > 0)
      if let Some(progId) = progId {
        if (progId.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 {
          // java PLCField plcField = document.getPLCField();
          let mut plcField: PLCField = document.getPLCField();
          // java ArrayList<PLCField.FieldTriple> fieldTriples = plcField.getFieldTriples(start, end);
          let mut fieldTriples: Vec<PLCField::FieldTriple> = plcField.getFieldTriples(start, end);
          // java if (fieldTriples.size() > 0)
          if (fieldTriples.len() as i32) > 0 {
            // java PLCField.FieldTriple fieldTriple = (PLCField.FieldTriple)fieldTriples.get(0);
            let mut fieldTriple: PLCField::FieldTriple = (fieldTriples.get(0)).downcast_ref::<PLCField::FieldTriple>().unwrap();
            // java Long codeRange = fieldTriple.getCodeRange();
            let mut codeRange: i64 = fieldTriple.getCodeRange();
            // java if (null != codeRange)
            if codeRange.is_some() {
              // java int rangeStart = RANGEFactory.getStart(codeRange);
              let mut rangeStart: i32 = RANGEFactory::getStart(codeRange);
              // java int rangeEnd = RANGEFactory.getEnd(codeRange);
              let mut rangeEnd: i32 = RANGEFactory::getEnd(codeRange);
              // java int size = rangeEnd - rangeStart;
              let mut size: i32 = rangeEnd - rangeStart;
              // java char[] buff = new char[size];
              let mut buff: Vec<char> = vec![Default::default(); (size) as usize];
              // java document.getText(rangeStart, rangeEnd, buff, 0);
              document.getText(rangeStart, rangeEnd, &buff, 0);
              // java FieldCodeLex codeLex = new FieldCodeLex();
              let mut codeLex: FieldCodeLex = FieldCodeLex::new();
                  // java codeLex.parse(buff, size);
                  codeLex.parse(buff, size);
                  // java UnKnownSwitchAgrument e
                  todo!();
                    // java IOLog.d("KComment", "UnKnownSwitchAgrument", e);
                    IOLog::d__string_string_throwable(&"KComment", &"UnKnownSwitchAgrument", &e);
                    // java return;
                    return;
              // java if (codeLex.getParamCount() > 0)
              if codeLex.getParamCount() > 0 {
                // java String oldProgId = codeLex.getParam(0).mStr;
                let mut oldProgId: String = codeLex.getParam(0).mStr;
                // java if (!progId.equals(oldProgId))
                if !progId.equals(oldProgId) {
                  // java String newFieldCode = " " + KFieldType.FieldEmbed.toString() + " " + progId + " ";
                  let mut newFieldCode: String = " " + KFieldType::FieldEmbed.toString() + " " + progId + " ";
                  // java int length = newFieldCode.length();
                  let mut length: i32 = (newFieldCode.len() as i32);
                  // java document.getTextRope().insert(rangeStart, (String)newFieldCode, 0, length);
                  document.getTextRope().insert(rangeStart, newFieldCode.to_string(), 0, length);
                  // java document.getTextRope().erase(rangeStart + length, rangeEnd + length);
                  document.getTextRope().erase(rangeStart + length, rangeEnd + length);
                }
              }
            }
          }
        }
      }
    }

    // java protected boolean deleteComment()
    pub fn deleteComment(&mut self) -> bool {
      // java int commentRef = this.getCommentRefCp();
      let mut commentRef: i32 = self.getCommentRefCp();
      // java KRange range = KRange.newInstance(this.document, commentRef, commentRef + 1);
      let mut range: KRange = KRange::newInstance__document_int_int(&self.document, commentRef, commentRef + 1);
      // java range.deleteRange();
      range.deleteRange();
      // java range.dispose();
      range.dispose();
      // java if (null != this.commentInfo)
      if let Some(commentInfo) = self.commentInfo.as_mut() {
        // java for(CommentInfo child : this.commentInfo.getChilds())
        for child in commentInfo.getChilds().iter() {
          // java child.getComment().deleteComment();
          child.getComment().deleteComment();
        }
      }
      // java else
      else {
        // java this.removeReplyRelation();
        self.removeReplyRelation();
      }
      // java return true;
      return true;
    }

    // java private void removeReplyRelation()
    pub fn removeReplyRelation(&self) {
      // java PLCCommentRef plcCommentRef = this.document.getCommentRef();
      let mut plcCommentRef: PLCCommentRef = self.document.getCommentRef();
      // java PLCCommentRef.CommentRefNode cmtRef = this.commentRef;
      let mut cmtRef: PLCCommentRef::CommentRefNode = self.commentRef;
      // java if (null == cmtRef.getParentRefNode())
      if cmtRef.getParentRefNode().is_none() {
        // java PLC.Iterator iter = plcCommentRef.createIterator();
        let mut iter: Vec<()> = plcCommentRef.createIterator();
        // java while (iter.hasNext())
        while iter.hasNext() {
          // java PLCCommentRef.CommentRefNode refNode = (PLCCommentRef.CommentRefNode)iter.next();
          let mut refNode: PLCCommentRef::CommentRefNode = (iter.next()).downcast_ref::<PLCCommentRef::CommentRefNode>().unwrap();
          // java if (refNode.getParentRefNode() == cmtRef)
          if refNode.getParentRefNode() == cmtRef {
            // java plcCommentRef.setParentRefNode(refNode, (PLCCommentRef.CommentRefNode)null);
            plcCommentRef.setParentRefNode(refNode, None);
          }
        }
      }
    }

    // java public KRange getCommentRange()
    pub fn getCommentRange(&self) -> Option<KRange> {
      // java Assert.assertNotNull("commentText should not be null !", this.commentTxt);
      Assert::assertNotNull__string_object(&"commentText should not be null !", Some(&self.commentTxt));
      // java Document document = this.getCommentDocument();
      let mut document: Box<dyn Document> = self.getCommentDocument();
          // java long rg = document.getCommentTxt().getRange(this.commentTxt);
          let mut rg: i64 = document.getCommentTxt().getRange(self.commentTxt);
          // java return KRange.newInstance(document, RANGEFactory.getStart(rg), RANGEFactory.getEnd(rg) - 1);
          return KRange::newInstance__document_int_int(&document, RANGEFactory::getStart(rg), RANGEFactory::getEnd(rg) - 1);
          // java WriterModelException e
          todo!();
            // java Log.e("", "WriterModelException", e);
            Log::e__string_string_throwable(&"", &"WriterModelException", &e);
            // java return null;
            return None;
    }

    // java public Shape getInkCommentShape()
    pub fn getInkCommentShape(&self) -> Option<Shape> {
      // java CommentInfo commentInfo = this.getCommentInfo();
      let mut commentInfo: CommentInfo = self.getCommentInfo();
      // java int type = commentInfo.getCommentType();
      let mut r#type: i32 = commentInfo.getCommentType();
      // java return type == 1 ? commentInfo.getFirstShape() : null;
      return (if r#type == 1 { Some(commentInfo.getFirstShape()) } else { None });
    }

    // java public Shape getCommentShape()
    pub fn getCommentShape(&self) -> Shape {
      // java return this.getCommentInfo().getFirstShape();
      return self.getCommentInfo().getFirstShape();
    }

    // java public String getAudio()
    pub fn getAudio(&self) -> Option<String> {
      // java if (null == this.commentRef.getCustomData())
      if self.commentRef.getCustomData().is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java KRange rg = this.getCommentRange();
        let mut rg: KRange = self.getCommentRange();
        // java if (null == rg)
        if rg.is_none() {
          // java return null;
          return None;
        }
        // java else if (rg.getLength() <= 0)
        else if rg.getLength() <= 0 {
          // java return null;
          return None;
        }
        // java else
        else {
          // java Document document = rg.getSubDocument();
          let mut document: Box<dyn Document> = rg.getSubDocument();
          // java PLCField plcField = document.getPLCField();
          let mut plcField: PLCField = document.getPLCField();
          // java PLCField.FieldTriple fldTriple = plcField.getContainingField(rg.getStart());
          let mut fldTriple: PLCField::FieldTriple = plcField.getContainingField(rg.getStart());
          // java if (null == fldTriple)
          if fldTriple.is_none() {
            // java return null;
            return None;
          }
          // java else if (KFieldType.FieldEmbed.getFlt() != fldTriple.start.mData)
          else if KFieldType::FieldEmbed.getFlt() != fldTriple.start.mData {
            // java return null;
            return None;
          }
          // java else
          else {
            // java long codeRange = fldTriple.getCodeRange();
            let mut codeRange: i64 = fldTriple.getCodeRange();
            // java KRange codeRg = new KRange(document, codeRange);
            let mut codeRg: KRange = KRange::new(document, codeRange);
            // java String code = codeRg.getText();
            let mut code: String = codeRg.getText();
            // java if (!code.contains("Package"))
            if !code.contains("Package") {
              // java return null;
              return None;
            }
            // java else
            else {
              // java IChpxTable chpxtbl = document.getChpxTable();
              let mut chpxtbl: Box<dyn IChpxTable> = document.getChpxTable();
              // java IChpxTable.Entry entry = chpxtbl.seek(fldTriple.separator.getCP());
              let mut entry: Box<dyn Entry> = chpxtbl.seek(fldTriple.separator.getCP());
              // java if (null == entry)
              if entry.is_none() {
                // java return null;
                return None;
              }
              // java else
              else {
                // java PropertySet prop = entry.getProperty();
                let mut prop: PropertySet = entry.getProperty();
                // java Shape oleShape = (Shape)prop.getValue(56);
                let mut oleShape: Shape = (prop.getValue(56)).downcast_ref::<Shape>().unwrap();
                // java if (null == oleShape)
                if oleShape.is_none() {
                  // java return null;
                  return None;
                }
                // java else
                else {
                  // java OLE ole = oleShape.getOLE();
                  let mut ole: OLE = oleShape.getOLE();
                  // java int mediaId = ole.getMediaID();
                  let mut mediaId: i32 = ole.getMediaID();
                  // java String binPath = null;
                  let mut binPath: String = String::new();
                      // java binPath = document.getTextDocument().getOleManager().getOleBinPath(mediaId);
                      binPath = document.getTextDocument().getOleManager().getOleBinPath(mediaId);
                      // java OleParseInterruptException var17
                      todo!();
                        // java IOLog.d("KComment", "create ole bin interrupted");
                        IOLog::d__string_string(&"KComment", &"create ole bin interrupted");
                  // java return null == binPath ? null : (new AudioCommentStreamParser(document.getTextDocument().getOleManager().getOleCache())).getRawFilePath(binPath);
                  return (if binPath.is_none() { None } else { Some(AudioCommentStreamParser::new(document.getTextDocument().getOleManager().getOleCache()).getRawFilePath(binPath)) });
                }
              }
            }
          }
        }
      }
    }

    // java public String getText()
    pub fn getText(&self) -> String {
      // java return this.getCommentRange().getText();
      return self.getCommentRange().getText();
    }

    // java public long getAudioDuration()
    pub fn getAudioDuration(&self) -> i64 {
      // java return this.commentRef.getCustomData().audioDuration;
      return self.commentRef.getCustomData().audioDuration;
    }

    // java public String getAuthorIcon()
    pub fn getAuthorIcon(&self) -> Option<String> {
      // java if (null == this.commentRef.getCustomData())
      if self.commentRef.getCustomData().is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java Integer authorIconId = this.commentRef.getCustomData().authorIconId;
        let mut authorIconId: i32 = self.commentRef.getCustomData().authorIconId;
        // java return null == authorIconId ? null : this.document.getTextDocument().getMediaLib().getMediaByID(authorIconId, MediaTypeEnum.PICTURE);
        return (if authorIconId.is_none() { None } else { Some(self.document.getTextDocument().getMediaLib().getMediaByID(authorIconId, MediaTypeEnum::PICTURE)) });
      }
    }

    // java public DateTime getDateTime()
    pub fn getDateTime(&self) -> DateTime {
      // java Assert.assertNotNull("commentRef should not be nul!", this.commentRef);
      Assert::assertNotNull__string_object(&"commentRef should not be nul!", Some(&self.commentRef));
      // java return this.commentRef.getDateTime();
      return self.commentRef.getDateTime();
    }

    // java public void detach()
    pub fn detach(&self) {
      // java PLCBookmarkStart starts = this.document.getCommentStarts();
      let mut starts: PLCBookmarkStart = self.document.getCommentStarts();
      // java PLCBookmarkEnd ends = this.document.getCommentEnds();
      let mut ends: PLCBookmarkEnd = self.document.getCommentEnds();
      // java starts.removeBookmarkStart(this.commentStart);
      starts.removeBookmarkStart(self.commentStart);
      // java ends.removeBookmarkEnd(this.commentEnd);
      ends.removeBookmarkEnd(self.commentEnd);
      // java PLCCommentRef refs = this.document.getCommentRef();
      let mut refs: PLCCommentRef = self.document.getCommentRef();
      // java refs.detachCommentRef(this.commentRef);
      refs.detachCommentRef(self.commentRef);
    }

    // java public void attach(KRange range, int ref)
    pub fn attach(&mut self, range: &KRange, r#ref: i32) {
      // java PLCBookmarkStart.BookmarkStartNode oldStart = this.commentStart;
      let mut oldStart: PLCBookmarkStart::BookmarkStartNode = self.commentStart;
      // java PLCCommentRef.CommentRefNode oldRef = this.commentRef;
      let mut oldRef: PLCCommentRef::CommentRefNode = self.commentRef;
      // java PLCBookmarkStart starts = this.document.getCommentStarts();
      let mut starts: PLCBookmarkStart = self.document.getCommentStarts();
      // java PLCBookmarkEnd ends = this.document.getCommentEnds();
      let mut ends: PLCBookmarkEnd = self.document.getCommentEnds();
      // java this.commentStart = starts.addBookmarkStart(range.getStart());
      self.commentStart = starts.addBookmarkStart(range.getStart());
      // java this.commentEnd = ends.addBookmarkEnd(range.getEnd());
      self.commentEnd = ends.addBookmarkEnd(range.getEnd());
      // java PLCCommentRef refs = this.document.getCommentRef();
      let mut refs: PLCCommentRef = self.document.getCommentRef();
      // java this.commentRef = refs.addCommentRef(ref);
      self.commentRef = refs.addCommentRef(r#ref);
      // java this.commentStart.setName(oldStart.getName());
      self.commentStart.setName__object(&oldStart.getName());
      // java this.commentStart.setCommentRef(this.commentRef);
      self.commentStart.setCommentRef(&self.commentRef);
      // java this.commentStart.setEnd(this.commentEnd);
      self.commentStart.setEnd(&self.commentEnd);
      // java this.commentEnd.setCommentRef(this.commentRef);
      self.commentEnd.setCommentRef(&self.commentRef);
      // java this.commentEnd.setStart(this.commentStart);
      self.commentEnd.setStart(&self.commentStart);
      // java this.commentRef.setAuthor(oldRef.getAuthor());
      self.commentRef.setAuthor(&oldRef.getAuthor());
      // java this.commentRef.setAbbreviation(oldRef.getAbbreviation());
      self.commentRef.setAbbreviation(&oldRef.getAbbreviation());
      // java this.commentRef.setDateTime(oldRef.getDateTime());
      self.commentRef.setDateTime(&oldRef.getDateTime());
      // java this.commentRef.setTag(oldRef.getTag());
      self.commentRef.setTag(&oldRef.getTag());
      // java this.commentRef.setParentRefNode(oldRef.getParentRefNode());
      self.commentRef.setParentRefNode(&oldRef.getParentRefNode());
      // java this.commentRef.setInk(oldRef.isInk());
      self.commentRef.setInk__object(&oldRef.isInk());
      // java this.commentRef.setCustomData(oldRef.getCustomData());
      self.commentRef.setCustomData__object(&oldRef.getCustomData());
      // java this.commentRef.setCommentStart(this.commentStart);
      self.commentRef.setCommentStart(&self.commentStart);
      // java this.commentRef.setCommentEnd(this.commentEnd);
      self.commentRef.setCommentEnd(&self.commentEnd);
      // java this.commentRef.setCommentTxt(this.commentTxt);
      self.commentRef.setCommentTxt(&self.commentTxt);
      // java this.commentTxt.setCommentRef(this.commentRef);
      self.commentTxt.setCommentRef(self.commentRef);
    }

    // java public int removeRefChar()
    pub fn removeRefChar(&mut self) -> i32 {
      // java this.start();
      self.start();
      // java int var2;
      let mut var2: i32;
          // java int cp = this.commentRef.getCP();
          let mut cp: i32 = self.commentRef.getCP();
          // java var2 = this.document.getTextStream().removeText(cp, cp + 1);
          var2 = self.document.getTextStream().removeText(cp, cp + 1);
            // java this.commit("remove commentRef char");
            self.commit(&"remove commentRef char");
      // java return var2;
      return var2;
    }

    // java public String getAuthor()
    pub fn getAuthor(&self) -> String {
      // java Assert.assertNotNull("commentRef should not be nul!", this.commentRef);
      Assert::assertNotNull__string_object(&"commentRef should not be nul!", Some(&self.commentRef));
      // java return this.commentRef.getAuthor();
      return self.commentRef.getAuthor();
    }

    // java public void setAuthor(String author)
    pub fn setAuthor(&mut self, author: &str) {
      // java this.start();
      self.start();
          // java PLCCommentRef plcCommentRef = this.document.getCommentRef();
          let mut plcCommentRef: PLCCommentRef = self.document.getCommentRef();
          // java plcCommentRef.setAuthor(this.commentRef, author);
          plcCommentRef.setAuthor(self.commentRef, author);
          // java plcCommentRef.setAbbreviation(this.commentRef, this.getAbbreviation(author));
          plcCommentRef.setAbbreviation(self.commentRef, self.getAbbreviation(&mut author));
            // java this.commit("set author");
            self.commit(&"set author");
    }

    // java public KRange getRange()
    pub fn getRange(&self) -> KRange {
      // java return new KRange(this.document, this.commentStart.getCP(), this.commentEnd.getCP());
      return KRange::new(self.document, self.commentStart.getCP(), self.commentEnd.getCP());
    }

    // java public boolean isEmptyComment()
    pub fn isEmptyComment(&self) -> bool {
          // java PLCCommentRef.CommentRefNode.CustomData customData = this.commentRef.getCustomData();
          let mut customData: CommentRefNode::CustomData = self.commentRef.getCustomData();
          // java if (customData != null)
          if customData.is_some() {
            // java return false;
            return false;
          }
          // java Document document = this.getCommentDocument();
          let mut document: Box<dyn Document> = self.getCommentDocument();
          // java KRange rg = this.getCommentRange();
          let mut rg: KRange = self.getCommentRange();
          // java if (null != rg)
          if rg.is_some() {
            // java int startCP = rg.getStart();
            let mut startCP: i32 = rg.getStart();
            // java int endCP = rg.getEnd();
            let mut endCP: i32 = rg.getEnd();
            // java if (endCP - startCP <= 1)
            if endCP - startCP <= 1 {
              // java char c = document.charAt(startCP);
              let mut c: char = document.charAt(startCP);
              // java if (c == 5)
              if c == 5 {
                // java IChpxTable.Entry run = document.getChpxTable().seek(startCP);
                let mut run: Box<dyn Entry> = document.getChpxTable().seek(startCP);
                // java if (!run.isHead() && run.getProperty().getBoolean(18, false))
                if !run.isHead() && run.getProperty().getBoolean(18, false) {
                  // java return true;
                  return true;
                }
              }
            }
            // java rg.dispose();
            rg.dispose();
          }
          // java Exception e
          todo!();
            // java e.printStackTrace();
            e.printStackTrace();
      // java return false;
      return false;
    }

    // java public CommentInfo getCommentInfo()
    pub fn getCommentInfo(&mut self) -> CommentInfo {
      // java if (null == this.commentInfo)
      if self.commentInfo.is_none() {
        // java this.commentInfo = new CommentInfo(this);
        self.commentInfo = CommentInfo::new(self);
      }
      // java return this.commentInfo;
      return self.commentInfo;
    }

    // java public void onEditComment()
    pub fn onEditComment(&mut self) {
      // java if (null != this.commentInfo)
      if let Some(commentInfo) = self.commentInfo.as_mut() {
        // java this.commentInfo.onEditComment();
        commentInfo.onEditComment();
      }
    }

    // java public void onAddReplayComment(KComment subComment)
    pub fn onAddReplayComment(&mut self, subComment: &KComment) {
      // java if (null != this.commentInfo)
      if let Some(commentInfo) = self.commentInfo.as_mut() {
        // java this.commentInfo.onAddReplayComment(subComment);
        commentInfo.onAddReplayComment(&subComment);
      }
    }

    // java public void onDeleteComment()
    pub fn onDeleteComment(&mut self) {
      // java if (null != this.commentInfo)
      if let Some(commentInfo) = self.commentInfo.as_mut() {
        // java KComment parent = this.commentInfo.getParent();
        let mut parent: KComment = commentInfo.getParent();
        // java if (null != parent && null != parent.commentInfo)
        if let Some(parent_commentInfo) = parent.commentInfo.as_mut() {
          if parent.is_some() {
            // java parent.commentInfo.onDeleteComment(this);
            parent_commentInfo.onDeleteComment(self);
          }
        }
        // java else
        else {
          // java this.commentInfo.onDeleteComment(this);
          commentInfo.onDeleteComment(&self);
        }
      }
    }

}
