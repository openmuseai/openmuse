// Java: import cn.wps.moffice.drawing.IPos
use drawing::cn::wps::moffice::drawing::IPos::*;
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
// Java: import cn.wps.moffice.util.StringUtil
use util::moffice::util::StringUtil::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.OleManager
use crate::cn::wps::moffice::writer::core::OleManager::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.TextTool
use crate::cn::wps::moffice::writer::core::TextTool::*;
// Java: import cn.wps.moffice.writer.core.env.text.TextFilterTool
use crate::cn::wps::moffice::writer::core::env::text::TextFilterTool::*;
// Java: import cn.wps.moffice.writer.core.shape.KShape
use crate::cn::wps::moffice::writer::core::shape::KShape::*;
// Java: import cn.wps.moffice.writer.core.shape.ShapeTool
use crate::cn::wps::moffice::writer::core::shape::ShapeTool::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCCommentRef
use crate::cn::wps::moffice::writer::data::PLCCommentRef::*;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.util.concurrent.locks.RangeReadLock
use crate::cn::wps::util::concurrent::locks::RangeReadLock::*;
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.ole.stream.AudioCommentStreamParser
use io_poi_compact::org::apache::poi::hwpf::ole::stream::AudioCommentStreamParser::*;
// Java: import org.apache.poi.hwpf.ole.stream.PackageStreamParser
use io_poi_compact::org::apache::poi::hwpf::ole::stream::PackageStreamParser::*;
// Java: import org.apache.poi.hwpf.util.OleParseInterruptException
use io_poi_compact::org::apache::poi::hwpf::util::OleParseInterruptException::*;
// Java: import cn.wps.moffice.writer.core.comment.KComment
use crate::cn::wps::moffice::writer::core::comment::KComment::*;
// Java: import cn.wps.moffice.writer.core.comment.CommentItem
use crate::cn::wps::moffice::writer::core::comment::CommentItem::*;
// Java: import cn.wps.moffice.writer.core.comment.TextItem
use crate::cn::wps::moffice::writer::core::comment::TextItem::*;
// Java: import cn.wps.moffice.writer.core.comment.KCommentRange
use crate::cn::wps::moffice::writer::core::comment::KCommentRange::*;
// Java: import cn.wps.moffice.writer.core.comment.InkItem
use crate::cn::wps::moffice::writer::core::comment::InkItem::*;
// Java: import cn.wps.moffice.writer.core.comment.OleItem
use crate::cn::wps::moffice::writer::core::comment::OleItem::*;
// Java: import cn.wps.moffice.writer.core.comment.KCmtCustData
use crate::cn::wps::moffice::writer::core::comment::KCmtCustData::*;
// Java: import PLCCommentRef.CommentRefNode  (via PLCCommentRef::* above)

// Java: import cn.wps.moffice.writer.core.comment.AudioItem
use crate::cn::wps::moffice::writer::core::comment::AudioItem::*;
// Java: import cn.wps.moffice.writer.core.comment.AudioTextItem
use crate::cn::wps::moffice::writer::core::comment::AudioTextItem::*;

// java public class CommentInfo
#[derive(Default)]
pub struct CommentInfo {
    // java private KComment curComment;
    curComment: Option<KComment>,
    // java private ArrayList<Shape> shapes;
    shapes: Option<Vec<Shape>>,
    // java private CommentItem curItem;
    curItem: Option<CommentItem>,
    // java private ArrayList<CommentInfo> childs;
    childs: Option<Vec<CommentInfo>>,
}

// java public class CommentInfo
impl CommentInfo {
    // java public  CommentInfo(KComment comment)
    pub fn new__kcomment(comment: &KComment) -> Self {
      // java this(comment, (KComment)null);
      let mut this = Self::new__kcomment_kcomment(&mut comment, Default::default());
      this
    }

    // java private  CommentInfo(KComment comment, KComment parent)
    pub fn new__kcomment_kcomment(comment: &mut KComment, parent: &KComment) -> Self {
      // java comment.commentInfo = this;
      comment.commentInfo = self;
      // java this.curComment = comment;
      let mut this = Self { curComment: comment, shapes: self.getCommentShapes(), curItem: self.initCurItem(Some(&mut parent)), childs: self.getChildList() };
      // java this.shapes = this.getCommentShapes();
      // java this.curItem = this.initCurItem(parent);
      // java this.childs = this.getChildList();
      this
    }

    // java public CommentItem getCurItem()
    pub fn getCurItem(&self) -> CommentItem {
      // java return this.curItem;
      return self.curItem;
    }

    // java public KComment getComment()
    pub fn getComment(&self) -> KComment {
      // java return this.curComment;
      return self.curComment;
    }

    // java public ArrayList<Shape> getShapes()
    pub fn getShapes(&self) -> Vec<Shape> {
      // java return this.shapes;
      return self.shapes;
    }

    // java public void setShapes(ArrayList<Shape> shapes)
    pub fn setShapes(&mut self, shapes: &Vec<Shape>) {
      // java this.shapes = shapes;
      self.shapes = shapes;
    }

    // java public String getAuthor()
    pub fn getAuthor(&self) -> String {
      // java return this.curItem.author;
      return self.curItem.author;
    }

    // java public void setAuthor(String author)
    pub fn setAuthor(&mut self, author: &str) {
      // java this.curItem.author = author;
      self.curItem.author = author;
    }

    // java public String getText()
    pub fn getText(&self) -> String {
      // java return this.curItem instanceof TextItem ? ((TextItem)this.curItem).text : "";
      return (if self.curItem.downcast_ref::<TextItem>().is_some() { (self.curItem).downcast_ref::<TextItem>().unwrap().text } else { "".to_string() });
    }

    // java public void setText(String text)
    pub fn setText(&self, text: &str) {
      // java if (this.curItem instanceof TextItem)
      if self.curItem.downcast_ref::<TextItem>().is_some() {
        // java ((TextItem)this.curItem).text = text;
        (self.curItem).downcast_ref::<TextItem>().unwrap().text = text;
      }
    }

    // java public ArrayList<CommentInfo> getChilds()
    pub fn getChilds(&self) -> Vec<CommentInfo> {
      // java return this.childs;
      return self.childs;
    }

    // java public int getCommentType()
    pub fn getCommentType(&self) -> i32 {
      // java return this.curItem.type;
      return self.curItem.r#type;
    }

    // java public KComment getParent()
    pub fn getParent(&self) -> KComment {
      // java return this.curItem.getParent();
      return self.curItem.getParent();
    }

    // java public boolean isEmpty()
    pub fn isEmpty(&self) -> Option<bool> {
      // java return StringUtil.isEmpty(this.getText()) && (null == this.shapes || this.shapes.isEmpty());
      return StringUtil::isEmpty(Some(&mut self.getText())) && self.shapes.is_none() || self.shapes.is_empty();
    }

    // java private ArrayList<CommentInfo> getChildList()
    pub fn getChildList(&self) -> Vec<CommentInfo> {
      // java ArrayList<CommentInfo> childs = new ArrayList();
      let mut childs: Vec<CommentInfo> = Vec::new();
      // java Document document = this.curComment.getMainDocument();
      let mut document: Box<dyn Document> = self.curComment.getMainDocument();
      // java PLCCommentRef plcCommentRef = document.getCommentRef();
      let mut plcCommentRef: PLCCommentRef = document.getCommentRef();
      // java PLCCommentRef.CommentRefNode cmtRef = this.curComment.commentRef;
      let mut cmtRef: PLCCommentRef::CommentRefNode = self.curComment.commentRef;
      // java if (null != cmtRef.getParentRefNode())
      if cmtRef.getParentRefNode().is_some() {
        // java return childs;
        return childs;
      }
      // java else
      else {
        // java PLC.Iterator iter = plcCommentRef.createIterator(cmtRef);
        let mut iter: Vec<()> = plcCommentRef.createIterator(cmtRef);
        // java TextDocument textDocument = this.curComment.getMainDocument().getTextDocument();
        let mut textDocument: TextDocument = self.curComment.getMainDocument().getTextDocument();
        // java KCommentRange kCommentRange = new KCommentRange(textDocument);
        let mut kCommentRange: KCommentRange = KCommentRange::new(textDocument);
        // java while (iter.hasNext())
        while iter.hasNext() {
          // java PLCCommentRef.CommentRefNode refNode = (PLCCommentRef.CommentRefNode)iter.next();
          let mut refNode: PLCCommentRef::CommentRefNode = (iter.next()).downcast_ref::<PLCCommentRef::CommentRefNode>().unwrap();
          // java PLCCommentRef.CommentRefNode parent = refNode.getParentRefNode();
          let mut parent: PLCCommentRef::CommentRefNode = refNode.getParentRefNode();
          // java if (parent != cmtRef)
          if parent != cmtRef {
            // java break;
            break;
          }
          // java KComment subComment = kCommentRange.findComment(document, refNode.getCP());
          let mut subComment: KComment = kCommentRange.findComment(document, refNode.getCP());
          // java if (null != subComment)
          if subComment.is_some() {
            // java childs.add(new CommentInfo(subComment, this.curComment));
            childs.add(CommentInfo::new(subComment, self.curComment));
          }
        }
        // java return childs;
        return childs;
      }
    }

    // java private CommentItem initCurItem(KComment parent)
    pub fn initCurItem(&mut self, parent: Option<&mut KComment>) -> CommentItem {
      // java String author = this.curComment.getAuthor();
      let mut author: String = self.curComment.getAuthor();
      // java TextDocument textDocument = this.curComment.getMainDocument().getTextDocument();
      let mut textDocument: TextDocument = self.curComment.getMainDocument().getTextDocument();
      // java KCommentRange kCommentRange = new KCommentRange(textDocument);
      let mut kCommentRange: KCommentRange = KCommentRange::new(textDocument);
      // java String text = getText(this.curComment.getCommentRange());
      let mut text: String = Self::getText__krange(&self.curComment.getCommentRange());
      // java int commentType = StringUtil.isEmpty(text) ? this.caclCommentType(this.curComment, this.shapes) : 0;
      let mut commentType: i32 = (if StringUtil::isEmpty(Some(&mut text)) { self.caclCommentType(&self.curComment, &self.shapes) } else { 0 });
      // java if (null == parent)
      if parent.is_none() {
        // java PLCCommentRef.CommentRefNode pRefNode = this.curComment.commentRef.getParentRefNode();
        let mut pRefNode: PLCCommentRef::CommentRefNode = self.curComment.commentRef.getParentRefNode();
        // java if (null != pRefNode)
        if pRefNode.is_some() {
          // java parent = kCommentRange.findComment(this.curComment.getMainDocument(), pRefNode.getCP());
          parent = kCommentRange.findComment(self.curComment.getMainDocument(), pRefNode.getCP());
        }
      }
      // java CommentItem item = null;
      let mut item: CommentItem = CommentItem::default();
      // java switch (commentType)
      match commentType {
        // java case 0:
        0 => {
          // java item = new TextItem(author, text, parent);
          item = TextItem::new(author, text, parent);
        }
        // java case 1:
        1 => {
          // java Shape shape = (Shape)this.shapes.get(0);
          let mut shape: Shape = (self.shapes.get(0)).downcast_ref::<Shape>().unwrap();
          // java MediaLib mediaLib = textDocument.getMediaLib();
          let mut mediaLib: MediaLib = textDocument.getMediaLib();
          // java Picture picture = shape.getPicture();
          let mut picture: Picture = shape.getPicture();
          // java if (null != picture)
          if picture.is_some() {
            // java String tmpImg = mediaLib.getMediaByID(picture.getMediaID(), MediaTypeEnum.PICTURE);
            let mut tmpImg: String = mediaLib.getMediaByID(picture.getMediaID(), MediaTypeEnum::PICTURE);
            // java item = new InkItem(author, shape.getInk(), shape.getShapePos().getShapeRect(), shape.getLineProperty(), tmpImg, parent);
            item = InkItem::new(author, shape.getInk(), shape.getShapePos().getShapeRect(), shape.getLineProperty(), tmpImg, parent);
          }
        }
        // java case 2:
        2 => {
          // java shape = (Shape)this.shapes.get(0);
          shape = (self.shapes.get(0)).downcast_ref::<Shape>().unwrap();
          // java OLE ole = ShapeTool.getShapeOle(shape);
          let mut ole: OLE = ShapeTool::getShapeOle(Some(&shape));
          // java mediaLib = textDocument.getMediaLib();
          mediaLib = textDocument.getMediaLib();
          // java picture = shape.getPicture();
          picture = shape.getPicture();
          // java if (null != picture)
          if picture.is_some() {
            // java String tmpImg = mediaLib.getMediaByID(picture.getMediaID(), MediaTypeEnum.PICTURE);
            let mut tmpImg: String = mediaLib.getMediaByID(picture.getMediaID(), MediaTypeEnum::PICTURE);
            // java String icolImg = null;
            let mut icolImg: String = String::new();
                // java OleManager oleManager = textDocument.getOleManager();
                let mut oleManager: OleManager = textDocument.getOleManager();
                // java int id = ole.getMediaID();
                let mut id: i32 = ole.getMediaID();
                // java if (id != -1)
                if id != -1 {
                  // java String binPath = oleManager.getOriginFile(ole);
                  let mut binPath: String = oleManager.getOriginFile(ole);
                  // java if (binPath != null)
                  if binPath.is_some() {
                    // java icolImg = (new PackageStreamParser(oleManager.getOleCache())).getRawFilePath(binPath);
                    icolImg = PackageStreamParser::new(oleManager.getOleCache()).getRawFilePath(binPath);
                    // java item = new OleItem(author, shape.getShapePos().getShapeRect(), ole.getProgId(), tmpImg, icolImg, parent);
                    item = OleItem::new(author, shape.getShapePos().getShapeRect(), ole.getProgId(), tmpImg, icolImg, parent);
                  }
                }
                // java OleParseInterruptException e
                todo!();
                  // java e.printStackTrace();
                  e.printStackTrace();
          }
        }
        // java case 3:
        3 => {
          // java shape = (Shape)this.shapes.get(0);
          shape = (self.shapes.get(0)).downcast_ref::<Shape>().unwrap();
          // java KCmtCustData custData = new KCmtCustData();
          let mut custData: KCmtCustData = KCmtCustData::new();
          // java PLCCommentRef.CommentRefNode.CustomData coreCustData = this.curComment.commentRef.getCustomData();
          let mut coreCustData: CommentRefNode::CustomData = self.curComment.commentRef.getCustomData();
          // java mediaLib = textDocument.getMediaLib();
          mediaLib = textDocument.getMediaLib();
          // java if (null != coreCustData)
          if coreCustData.is_some() {
            // java custData.mAnchorSpId = coreCustData.anchorSpId;
            custData.mAnchorSpId = coreCustData.anchorSpId;
            // java custData.mX = coreCustData.x;
            custData.mX = coreCustData.x;
            // java custData.mY = coreCustData.y;
            custData.mY = coreCustData.y;
            // java custData.mOffsetX = coreCustData.offsetX;
            custData.mOffsetX = coreCustData.offsetX;
            // java custData.mOffsetY = coreCustData.offsetY;
            custData.mOffsetY = coreCustData.offsetY;
            // java custData.mAudioDuration = coreCustData.audioDuration;
            custData.mAudioDuration = coreCustData.audioDuration;
            // java if (null != coreCustData.anchorSpId)
            if coreCustData.anchorSpId.is_some() {
              // java custData.mAuthorIcon = mediaLib.getMediaByID(coreCustData.anchorSpId, MediaTypeEnum.PICTURE);
              custData.mAuthorIcon = mediaLib.getMediaByID(coreCustData.anchorSpId, MediaTypeEnum::PICTURE);
            }
            // java if (null != parent)
            if let Some(parent) = parent {
              // java PLCCommentRef.CommentRefNode.CustomData parentCustData = parent.commentRef.getCustomData();
              let mut parentCustData: CommentRefNode::CustomData = parent.commentRef.getCustomData();
              // java if (null != parentCustData)
              if parentCustData.is_some() {
                // java custData.mAnchorSpId = parentCustData.anchorSpId;
                custData.mAnchorSpId = parentCustData.anchorSpId;
                // java custData.mX = parentCustData.x;
                custData.mX = parentCustData.x;
                // java custData.mY = parentCustData.y;
                custData.mY = parentCustData.y;
                // java custData.mOffsetX = parentCustData.offsetX;
                custData.mOffsetX = parentCustData.offsetX;
                // java custData.mOffsetY = parentCustData.offsetY;
                custData.mOffsetY = parentCustData.offsetY;
              }
            }
          }
          // java ole = ShapeTool.getShapeOle(shape);
          ole = ShapeTool::getShapeOle(Some(&shape));
          // java int mediaId = ole.getMediaID();
          let mut mediaId: i32 = ole.getMediaID();
          // java String binPath = null;
          let mut binPath: String = String::new();
          // java String audioPath = null;
          let mut audioPath: String = String::new();
              // java binPath = textDocument.getOleManager().getOleBinPath(mediaId);
              binPath = textDocument.getOleManager().getOleBinPath(mediaId);
              // java if (null != binPath)
              if binPath.is_some() {
                // java audioPath = (new AudioCommentStreamParser(textDocument.getOleManager().getOleCache())).getRawFilePath(binPath);
                audioPath = AudioCommentStreamParser::new(textDocument.getOleManager().getOleCache()).getRawFilePath(binPath);
                // java item = new AudioItem(author, audioPath, (byte[])null, custData, parent);
                item = AudioItem::new(author, audioPath, None, custData, parent);
              }
              // java OleParseInterruptException e
              todo!();
                // java e.printStackTrace();
                e.printStackTrace();
        }
        // java case 4:
        4 => {
          // java custData = new KCmtCustData();
          custData = KCmtCustData::new();
          // java coreCustData = this.curComment.commentRef.getCustomData();
          coreCustData = self.curComment.commentRef.getCustomData();
          // java if (null != coreCustData)
          if coreCustData.is_some() {
            // java custData.mAnchorSpId = coreCustData.anchorSpId;
            custData.mAnchorSpId = coreCustData.anchorSpId;
            // java custData.mX = coreCustData.x;
            custData.mX = coreCustData.x;
            // java custData.mY = coreCustData.y;
            custData.mY = coreCustData.y;
            // java custData.mOffsetX = coreCustData.offsetX;
            custData.mOffsetX = coreCustData.offsetX;
            // java custData.mOffsetY = coreCustData.offsetY;
            custData.mOffsetY = coreCustData.offsetY;
            // java custData.mAudioDuration = coreCustData.audioDuration;
            custData.mAudioDuration = coreCustData.audioDuration;
            // java if (null != parent)
            if let Some(parent) = parent {
              // java PLCCommentRef.CommentRefNode.CustomData parentCustData = parent.commentRef.getCustomData();
              let mut parentCustData: CommentRefNode::CustomData = parent.commentRef.getCustomData();
              // java if (null != parentCustData)
              if parentCustData.is_some() {
                // java custData.mAnchorSpId = parentCustData.anchorSpId;
                custData.mAnchorSpId = parentCustData.anchorSpId;
                // java custData.mX = parentCustData.x;
                custData.mX = parentCustData.x;
                // java custData.mY = parentCustData.y;
                custData.mY = parentCustData.y;
                // java custData.mOffsetX = parentCustData.offsetX;
                custData.mOffsetX = parentCustData.offsetX;
                // java custData.mOffsetY = parentCustData.offsetY;
                custData.mOffsetY = parentCustData.offsetY;
              }
            }
          }
          // java item = new AudioTextItem(author, text, custData, parent);
          item = AudioTextItem::new(author, text, custData, parent);
        }
        // java default:
        _ => {
        }
      }
      // java if (null == item)
      if item.is_none() {
        // java item = new TextItem(author, text, parent);
        item = TextItem::new(author, text, parent);
      }
      // java item.setDateTime(this.curComment.getDateTime());
      item.setDateTime(self.curComment.getDateTime());
      // java return item;
      return item;
    }

    // java private int caclCommentType(KComment comment, ArrayList<Shape> commentShapes)
    pub fn caclCommentType(&self, comment: &KComment, commentShapes: &Vec<Shape>) -> i32 {
      // java int commentType = 0;
      let mut commentType: i32 = 0;
      // java int size = commentShapes.size();
      let mut size: i32 = (commentShapes.len() as i32);
      // java if (size <= 1 && size >= 1)
      if size <= 1 && size >= 1 {
        // java Shape shape = (Shape)commentShapes.get(0);
        let mut shape: Shape = (commentShapes.get(0)).downcast_ref::<Shape>().unwrap();
        // java OLE ole = ShapeTool.getShapeOle(shape);
        let mut ole: OLE = ShapeTool::getShapeOle(Some(&shape));
        // java if (null != ole)
        if ole.is_some() {
          // java KShape kShape = new KShape(shape);
          let mut kShape: KShape = KShape::new(shape);
          // java if (this.isValidPenKit(comment, shape) && kShape.isPenkit())
          if self.isValidPenKit(&comment, &shape) && kShape.isPenkit() {
            // java commentType = 2;
            commentType = 2;
          }
          // java PLCCommentRef.CommentRefNode.CustomData customData = comment.commentRef.getCustomData();
          let mut customData: CommentRefNode::CustomData = comment.commentRef.getCustomData();
          // java if (null != customData)
          if customData.is_some() {
            // java commentType = 3;
            commentType = 3;
            // java if (!customData.isAudioComment())
            if !customData.isAudioComment() {
              // java commentType = 4;
              commentType = 4;
            }
          }
        }
        // java else if (shape.hasInk())
        else if shape.hasInk() {
          // java commentType = 1;
          commentType = 1;
        }
      }
      // java else
      else {
        // java commentType = 0;
        commentType = 0;
      }
      // java return commentType;
      return commentType;
    }

    // java private boolean isValidPenKit(KComment comment, Shape shape)
    pub fn isValidPenKit(&self, comment: &KComment, shape: &Shape) -> Option<bool> {
      // java Picture picture = shape.getPicture();
      let mut picture: Picture = shape.getPicture();
      // java if (picture == null)
      if picture.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java TextDocument textDocument = comment.getMainDocument().getTextDocument();
        let mut textDocument: TextDocument = comment.getMainDocument().getTextDocument();
        // java MediaLib mediaLib = textDocument.getMediaLib();
        let mut mediaLib: MediaLib = textDocument.getMediaLib();
        // java String tmpImg = mediaLib.getMediaByID(picture.getMediaID(), MediaTypeEnum.PICTURE);
        let mut tmpImg: String = mediaLib.getMediaByID(picture.getMediaID(), MediaTypeEnum::PICTURE);
        // java return tmpImg != null && tmpImg.toLowerCase().endsWith(".png");
        return tmpImg.is_some() && tmpImg.toLowerCase().endsWith(".png");
      }
    }

    // java private ArrayList getCommentShapes()
    pub fn getCommentShapes(&self) -> Vec<()> {
      // java ArrayList<Shape> shapes = ShapeTool.getShapes(this.curComment.getCommentRange());
      let mut shapes: Vec<Shape> = ShapeTool::getShapes(Some(&self.curComment.getCommentRange()));
      // java ArrayList<Shape> commentShapes = new ArrayList();
      let mut commentShapes: Vec<Shape> = Vec::new();
      // java for(Shape shape : shapes)
      for shape in shapes.iter() {
        // java if (shape.isPicture() || shape.hasInk())
        if shape.isPicture() || shape.hasInk() {
          // java IPos pos = shape.getShapePos();
          let mut pos: Box<dyn IPos> = shape.getShapePos();
          // java if (null == pos || !pos.getShapeRect().isEmpty())
          if pos.is_none() || !pos.getShapeRect().is_empty() {
            // java commentShapes.add(shape);
            commentShapes.add(shape);
          }
        }
      }
      // java return commentShapes;
      return commentShapes;
    }

    // java public static String getText(KRange commentRange)
    pub fn getText__krange(commentRange: &KRange) -> String {
      // java String commentText = TextTool.filterMath(commentRange);
      let mut commentText: String = TextTool::filterMath__krange(&commentRange);
      // java return TextTool.filterTextForComment(commentText).trim();
      return TextTool::filterTextForComment(Some(&mut commentText)).trim();
    }

    // java public static boolean isInvisibaleCP(Document document, int cp)
    pub fn isInvisibaleCP(document: &dyn Document, cp: i32) -> bool {
      // java IChpxTable.Entry run = document.getChpxTable().seek(cp);
      let mut run: Box<dyn Entry> = document.getChpxTable().seek(cp);
      // java if (!run.isHead())
      if !run.isHead() {
        // java PropertySet finalRunProp = PropertyUtil.getFinalRunFormat(run.getProperty());
        let mut finalRunProp: PropertySet = PropertyUtil::getFinalRunFormat(&run.getProperty());
        // java return finalRunProp.getBoolean(18, false);
        return finalRunProp.getBoolean(18, false);
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public Shape getFirstShape()
    pub fn getFirstShape(&self) -> Option<Shape> {
      // java return this.shapes.isEmpty() ? null : (Shape)this.shapes.get(0);
      return (if self.shapes.is_empty() { None } else { Some((self.shapes.get(0)).downcast_ref::<Shape>().unwrap()) });
    }

    // java private void deleteChilds()
    pub fn deleteChilds(&mut self) {
      // java this.childs.clear();
      self.childs.clear();
    }

    // java private void deleteComment(KComment comment)
    pub fn deleteComment(&mut self, comment: &KComment) {
      // java if (comment == this.curComment)
      if comment == self.curComment {
        // java this.deleteChilds();
        self.deleteChilds();
      }
      // java else
      else {
        // java for(CommentInfo child : this.childs)
        for child in self.childs.iter() {
          // java if (child.getComment() == comment)
          if child.getComment() == comment {
            // java this.childs.remove(child);
            self.childs.remove(child);
            // java break;
            break;
          }
        }
      }
    }

    // java private void addReplayComment(KComment comment)
    pub fn addReplayComment(&mut self, comment: &KComment) {
      // java if (comment.commentRef.getParentRefNode() == this.curComment.commentRef)
      if comment.commentRef.getParentRefNode() == self.curComment.commentRef {
        // java CommentInfo child = new CommentInfo(comment, this.curComment);
        let mut child: CommentInfo = CommentInfo::new(comment, self.curComment);
        // java this.childs.add(child);
        self.childs.add(child);
      }
    }

    // java private void editComment()
    pub fn editComment(&mut self) {
      // java this.curItem = this.initCurItem(this.curItem.getParent());
      self.curItem = self.initCurItem(Some(&mut self.curItem.getParent()));
      // java this.shapes = this.getCommentShapes();
      self.shapes = self.getCommentShapes();
    }

    // java public void onEditComment()
    pub fn onEditComment(&mut self) {
      // java this.editComment();
      self.editComment();
    }

    // java public void onAddReplayComment(KComment comment)
    pub fn onAddReplayComment(&mut self, comment: &KComment) {
      // java this.addReplayComment(comment);
      self.addReplayComment(&comment);
    }

    // java public void onDeleteComment(KComment comment)
    pub fn onDeleteComment(&mut self, comment: &KComment) {
      // java this.deleteComment(comment);
      self.deleteComment(&comment);
    }

    // java public String getCommentMainText(int maxLen)
    pub fn getCommentMainText(&self, maxLen: i32) -> String {
      // java if (null == this.curComment)
      if self.curComment.is_none() {
        // java return "<>";
        return "<>";
      }
      // java else
      else {
        // java Document document = this.curComment.getMainDocument();
        let mut document: Box<dyn Document> = self.curComment.getMainDocument();
        // java RangeReadLock lock = document.readLock();
        let mut lock: RangeReadLock = document.readLock();
        // java String var7;
        let mut var7: String = String::new();
            // java KRange rg = document.getRange(this.curComment.getCommentStartCp(), this.curComment.getCommentEndCp());
            let mut rg: KRange = document.getRange(self.curComment.getCommentStartCp(), self.curComment.getCommentEndCp());
            // java TextFilterTool textFilterTool = new TextFilterTool(rg);
            let mut textFilterTool: TextFilterTool = TextFilterTool::new(rg);
            // java String text = textFilterTool.getCommentMainText(maxLen);
            let mut text: String = textFilterTool.getCommentMainText(maxLen);
            // java var7 = StringUtil.isEmpty(text) ? "<>" : text;
            var7 = (if StringUtil::isEmpty(Some(&mut text)) { "<>".to_string() } else { text });
              // java lock.unlock();
              lock.unlock();
        // java return var7;
        return var7;
      }
    }

}
