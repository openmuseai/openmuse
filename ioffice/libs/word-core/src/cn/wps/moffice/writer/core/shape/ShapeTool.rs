// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::*;
// Java: import cn.wps.graphics.PointF
use util::graphics::PointF::*;
// Java: import cn.wps.graphics.RectF
use util::graphics::RectF::*;
// Java: import cn.wps.moffice.drawing.GroupShape
use drawing::cn::wps::moffice::drawing::GroupShape::*;
// Java: import cn.wps.moffice.drawing.IDrawingContainer
use drawing::cn::wps::moffice::drawing::IDrawingContainer::*;
// Java: import cn.wps.moffice.drawing.OLE
use drawing::cn::wps::moffice::drawing::OLE::*;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.ShapeFactory
use drawing::cn::wps::moffice::drawing::ShapeFactory::*;
// Java: import cn.wps.moffice.drawing.ShapePos
use drawing::cn::wps::moffice::drawing::ShapePos::*;
// Java: import cn.wps.moffice.drawing.graphics.Picture
use drawing::cn::wps::moffice::drawing::graphics::Picture::*;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
use drawing::cn::wps::moffice::drawing::media::IMediaLib::*;
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::*;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.util.FileUtil
use util::moffice::util::FileUtil::*;
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog::*;
// Java: import cn.wps.moffice.util.MetricsUtil
use util::moffice::util::MetricsUtil::*;
// Java: import cn.wps.moffice.util.StringUtil
use util::moffice::util::StringUtil::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KParaRunRevision
use crate::cn::wps::moffice::writer::core::KParaRunRevision::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.comment.CommentInfo
use crate::cn::wps::moffice::writer::core::comment::CommentInfo;
// Java: import cn.wps.moffice.writer.core.shape.command.KDeleteCommand
use crate::cn::wps::moffice::writer::core::shape::command::KDeleteCommand::*;
// CARGO_BLOCKER: KGrpAddChildCommand/KGrpDeleteChildCommand not generated
// Java: import cn.wps.moffice.writer.core.shape.command.KGrpAddChildCommand
// use crate::cn::wps::moffice::writer::core::shape::command::KGrpAddChildCommand::*;
// Java: import cn.wps.moffice.writer.core.shape.command.KGrpDeleteChildCommand
// use crate::cn::wps::moffice::writer::core::shape::command::KGrpDeleteChildCommand::*;
// Java: import cn.wps.moffice.writer.data.CPieceTable
use crate::cn::wps::moffice::writer::data::CPieceTable::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Disambiguate nested Entry traits (IChpxTable/IPapxTable/ITable)
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::IPapxTable::Entry as PapxEntry;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::*;
// Java: import cn.wps.moffice.writer.data.PLCShapeAnchor
use crate::cn::wps::moffice::writer::data::PLCShapeAnchor::*;
// Java: import cn.wps.moffice.writer.data.PLCTextboxText
use crate::cn::wps::moffice::writer::data::PLCTextboxText::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.value.PageSetup
use crate::cn::wps::moffice::writer::data::value::PageSetup::*;
// Java: import cn.wps.moffice.writer.drawing.DrawingContainer
use crate::cn::wps::moffice::writer::drawing::DrawingContainer::*;
// Java: import cn.wps.moffice.writer.drawing.TextFrame
use crate::cn::wps::moffice::writer::drawing::TextFrame::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.Map
// Java: import org.apache.poi.hwpf.ole.OleClsTypeMatcher
use io_poi_compact::org::apache::poi::hwpf::ole::OleClsTypeMatcher::*;
// Java: import cn.wps.moffice.writer.core.shape.KShape
use crate::cn::wps::moffice::writer::core::shape::KShape::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use std::collections::{HashMap};

// java public static final String BRAND_HUAWEI = "huawei";
pub(crate) const BRAND_HUAWEI: &str = "huawei";
// java public static final String BRAND_HONOR = "honor";
pub(crate) const BRAND_HONOR: &str = "honor";
// java public static final String STR_PENKIT = "penkit";
pub(crate) const STR_PENKIT: &str = "penkit";
// java public static final String PENKIT_SUFFIX_HUAWEI = "hwPenkit";
pub(crate) const PENKIT_SUFFIX_HUAWEI: &str = "hwPenkit";
// java public static final String PENKIT_SUFFIX_HONOR = "hnPenkit";
pub(crate) const PENKIT_SUFFIX_HONOR: &str = "hnPenkit";
// java private static final Map<String, String> brandNameMap = new HashMap<String, String>() { { this.put("huawei", "hwPenkit"); this.put("honor", "hnPenkit"); } };
static brandNameMap: std::sync::LazyLock<Option<HashMap<String, String>>> =
                    std::sync::LazyLock::new(|| HashMap::new());

// java public class ShapeTool
#[derive(Default)]
pub struct ShapeTool {
}

// java public class ShapeTool
impl ShapeTool {
    // java public static String getPenKitName(String brand)
    pub fn getPenKitName(brand: Option<&mut str>) -> Option<String> {
      // java return brand != null && brand.length() > 0 ? (String)brandNameMap.get(brand.toLowerCase()) : null;
      return (if brand.is_some() && (brand.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 { Some(brandNameMap.get(brand.toLowerCase()).to_string()) } else { None });
    }

    // java public static boolean IsOleShape(Document document, Shape shape)
    pub fn IsOleShape(document: &dyn Document, shape: &Shape) -> bool {
      // java if (shape.isOleShape())
      if shape.isOleShape() {
        // java return true;
        return true;
      }
      // java else
      else {
        // java int cp = position(document, shape);
        let mut cp: i32 = Self::position(&document, &shape);
        // java PLCField plcField = document.getPLCField();
        let mut plcField: PLCField = document.getPLCField();
        // java PLCField.FieldTriple triple = plcField.getContainingField(cp);
        let mut triple: PLCField::FieldTriple = plcField.getContainingField(cp);
        // java if (isOleTriple(triple))
        if Self::isOleTriple(Some(&triple)) {
          // java IChpxTable.Entry chpx = document.getChpxTable().seek(triple.separator.getCP());
          let mut chpx: Box<dyn ChpxEntry> = document.getChpxTable().seek(triple.separator.getCP());
          // java if (chpx.getProperty().getBoolean(30, false))
          if chpx.getProperty().getBoolean(30, false) {
            // java return true;
            return true;
          }
        }
        // java return false;
        return false;
      }
    }

    // java public static final void updateAllOlePicID(Document document)
    pub fn updateAllOlePicID(document: Option<&dyn Document>) {
      // java if (document != null)
      if let Some(document) = document {
        // java DrawingContainer drawingContainer = (DrawingContainer)document.getDrawingContainer();
        let mut drawingContainer: DrawingContainer = (document.getDrawingContainer()).downcast_ref::<DrawingContainer>().unwrap();
        // java Document textboxDocument = document.getTextboxDocument();
        let mut textboxDocument: Box<dyn Document> = document.getTextboxDocument();
        // java PLCTextboxText textboxText = textboxDocument.getTextboxText();
        let mut textboxText: PLCTextboxText = textboxDocument.getTextboxText();
        // java if (textboxText != null && !textboxText.isEmpty())
        if textboxText.is_some() && !textboxText.is_empty() {
          // java PLCShapeAnchor anchors = document.getShapeAnchors();
          let mut anchors: PLCShapeAnchor = document.getShapeAnchors();
          // java if (!anchors.isEmpty())
          if !anchors.is_empty() {
            // java PLC.Iterator iter = anchors.createIterator();
            let mut iter: Vec<()> = anchors.createIterator();
            // java while (iter.hasNext())
            while iter.hasNext() {
              // java PLCShapeAnchor.ShapeAnchor shapeAnchor = (PLCShapeAnchor.ShapeAnchor)iter.next();
              let mut shapeAnchor: PLCShapeAnchor::ShapeAnchor = (iter.next()).downcast_ref::<PLCShapeAnchor::ShapeAnchor>().unwrap();
              // java int shapeID = shapeAnchor.getShapeId();
              let mut shapeID: i32 = shapeAnchor.getShapeId();
              // java Shape shape = drawingContainer.findShapeByID(shapeID);
              let mut shape: Shape = drawingContainer.findShapeByID(shapeID);
              // java updateShapePicID(shape, textboxText);
              Self::updateShapePicID(Some(&shape), &mut textboxText);
            }
          }
        }
      }
    }

    // java protected static void updateShapePicID(Shape shape, PLCTextboxText textboxText)
    pub fn updateShapePicID(shape: Option<&Shape>, textboxText: &mut PLCTextboxText) {
      // java if (shape != null)
      if let Some(shape) = shape {
        // java if (shape.isGroupShape())
        if shape.isGroupShape() {
          // java GroupShape groupShape = (GroupShape)shape;
          let mut groupShape: GroupShape = (shape).downcast_ref::<GroupShape>().unwrap();
          // java int size = groupShape.count();
          let mut size: i32 = groupShape.count();
          let mut i: i32 = 0;
          // java for(int i = 0; i < size; ++i)
          while i < size {
            // java updateShapePicID(groupShape.getChildShape(i), textboxText);
            Self::updateShapePicID(Some(&groupShape.getChildShape(i)), &mut textboxText);
          }
        }
        // java else if (shape.isOleShape())
        else if shape.isOleShape() {
          // java Picture picture = shape.getPicture();
          let mut picture: Picture = shape.getPicture();
          // java if (picture != null)
          if picture.is_some() {
            // java PLCTextboxText.TextboxTextNode node = textboxText.findTextboxByShapeId(shape.getShapeID());
            let mut node: PLCTextboxText::TextboxTextNode = textboxText.findTextboxByShapeId(shape.getShapeID());
            // java int picID = textboxText.getNodePos(node);
            let mut picID: i32 = textboxText.getNodePos(Some(&mut node));
            // java if (picID < 0)
            if picID < 0 {
              // java picID = 0;
              picID = 0;
            }
            // java picture.setPicID(picID << 16);
            picture.setPicID(picID << 16);
          }
        }
      }
    }

    // java public static final boolean isOleTriple(PLCField.FieldTriple triple)
    pub fn isOleTriple(triple: Option<&PLCField::FieldTriple>) -> Option<bool> {
      // java return triple != null && KFieldType.FieldEmbed.getFlt() == triple.getFieldType();
      return triple.is_some() && KFieldType::FieldEmbed.getFlt() == triple.getFieldType();
    }

    // java public static final boolean isShapeTriple(PLCField.FieldTriple triple)
    pub fn isShapeTriple(triple: Option<&PLCField::FieldTriple>) -> Option<bool> {
      // java return triple != null && (KFieldType.FieldShape.getFlt() == triple.getFieldType() || KFieldType.FieldEmbed.getFlt() == triple.getFieldType());
      return triple.is_some() && KFieldType::FieldShape.getFlt() == triple.getFieldType() || KFieldType::FieldEmbed.getFlt() == triple.getFieldType();
    }

    // java public static int position(Document document, Shape shape)
    pub fn position(document: &dyn Document, shape: &Shape) -> i32 {
      // java return cn.wps.moffice.writer.data.ShapeTool.position(document, shape);
      return cn.wps.moffice.writer.data.ShapeTool.position(document, shape);
    }

    // java public static Shape getShape01(Shape shape)
    pub fn getShape01(shape: &Shape) -> Option<Shape> {
      // java Document document = (Document)shape.getDrawingContainer().getSubDocument();
      let mut document: Box<dyn Document> = todo!("cast → Box<dyn Document>");
      // java int cp = position(document, shape);
      let mut cp: i32 = Self::position(&document, &shape);
      // java PLCField plcField = document.getPLCField();
      let mut plcField: PLCField = document.getPLCField();
      // java PLCField.FieldTriple triple = plcField.getContainingField(cp);
      let mut triple: PLCField::FieldTriple = plcField.getContainingField(cp);
      // java if (isShapeTriple(triple))
      if Self::isShapeTriple(Some(&triple)) {
        // java PLCShapeAnchor.ShapeAnchor anchor = document.getShapeAnchors().findByCP(cp + 1);
        let mut anchor: PLCShapeAnchor::ShapeAnchor = document.getShapeAnchors().findByCP(cp + 1);
        // java if (anchor != null)
        if anchor.is_some() {
          // java return document.getDrawingContainer().findShapeByID(anchor.getShapeId());
          return document.getDrawingContainer().findShapeByID(anchor.getShapeId());
        }
      }
      // java return null;
      return None;
    }

    // java public static int getShapeStartCP(Document document, Shape shape)
    pub fn getShapeStartCP(document: &dyn Document, shape: &Shape) -> i32 {
      // java int cp = position(document, shape);
      let mut cp: i32 = Self::position(&document, &shape);
      // java if (shape.getInlineShape())
      if shape.getInlineShape() {
        // java PLCField plcField = document.getPLCField();
        let mut plcField: PLCField = document.getPLCField();
        // java if (plcField != null)
        if plcField.is_some() {
          // java PLCField.FieldTriple triple = plcField.getContainingField(cp);
          let mut triple: PLCField::FieldTriple = plcField.getContainingField(cp);
          // java if (isShapeTriple(triple))
          if Self::isShapeTriple(Some(&triple)) {
            // java cp = triple.start.getCP();
            cp = triple.start.getCP();
          }
        }
      }
      // java return cp;
      return cp;
    }

    // java public static void deleteSubText(Document document, GroupShape groupShape)
    pub fn deleteSubText(document: &dyn Document, groupShape: &GroupShape) {
      // java int size = groupShape.count();
      let mut size: i32 = groupShape.count();
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java Shape shape = groupShape.getChildShape(i);
        let mut shape: Shape = groupShape.getChildShape(i);
        // java if (shape != null)
        if shape.is_some() {
          // java if (shape.isGroupShape())
          if shape.isGroupShape() {
            // java deleteSubText(document, (GroupShape)shape);
            Self::deleteSubText(&document, &(shape).downcast_ref::<GroupShape>().unwrap());
          }
          // java else
          else {
            // java deleteText(document, shape.getShapeID());
            Self::deleteText(&document, shape.getShapeID());
          }
        }
      }
    }

    // java public static void deleteText(Document document, int shapeID)
    pub fn deleteText(document: &dyn Document, shapeID: i32) {
      // java Document textboxDocument = document.getTextboxDocument();
      let mut textboxDocument: Box<dyn Document> = document.getTextboxDocument();
      // java if (null != textboxDocument)
      if textboxDocument.is_some() {
        // java PLCTextboxText textboxText = textboxDocument.getTextboxText();
        let mut textboxText: PLCTextboxText = textboxDocument.getTextboxText();
        // java if (null != textboxText)
        if textboxText.is_some() {
          // java PLCTextboxText.TextboxTextNode node = textboxText.findTextboxByShapeId(shapeID);
          let mut node: PLCTextboxText::TextboxTextNode = textboxText.findTextboxByShapeId(shapeID);
          // java if (null != node)
          if node.is_some() {
            // java long rg = textboxText.getRange(node);
            let mut rg: i64 = textboxText.getRange(node);
            // java boolean trackChange = textboxDocument.getTextDocument().getTrackChange();
            let mut trackChange: bool = textboxDocument.getTextDocument().getTrackChange();
            // java if (trackChange)
            if trackChange {
              // java textboxDocument.getTextStream().disableRevisionFileter();
              textboxDocument.getTextStream().disableRevisionFileter();
            }
                // java textboxDocument.getTextStream().removeText(RANGEFactory.getStart(rg), RANGEFactory.getEnd(rg));
                textboxDocument.getTextStream().removeText(RANGEFactory::getStart(rg), RANGEFactory::getEnd(rg));
                    // java (trackChange)
                    trackChange;
                      // java textboxDocument.getTextStream().enableRevisionFileter();
                      textboxDocument.getTextStream().enableRevisionFileter();
          }
        }
      }
    }

    // java public static Long deleteShape(Document document, KShape coreShape)
    pub fn deleteShape__document_kshape(document: &dyn Document, coreShape: &KShape) -> i64 {
      // java return deleteShape(document, coreShape.getShape(), coreShape.getInlineShape());
      return Self::deleteShape__document_shape_shape(&document, &coreShape.getShape(), Some(&coreShape.getInlineShape()));
    }

    // java public static Long deleteShape(Document document, Shape shape, Shape inlineShape)
    pub fn deleteShape__document_shape_shape(document: &dyn Document, shape: &Shape, inlineShape: Option<&Shape>) -> Option<i64> {
      // java Long range = getShapeRange(document, shape);
      let mut range: i64 = Self::getShapeRange(&document, &shape);
      // java if (range == null)
      if range.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java boolean isDeleteShape = checkDeleteShape(document, range);
        let mut isDeleteShape: bool = Self::checkDeleteShape(&document, range);
        // java int start = RANGEFactory.getStart(range);
        let mut start: i32 = RANGEFactory::getStart(range);
        // java int end = RANGEFactory.getEnd(range);
        let mut end: i32 = RANGEFactory::getEnd(range);
        // java if (isDeleteShape)
        if isDeleteShape {
          // java if (inlineShape != null)
          if let Some(inlineShape) = inlineShape {
            // java document.getDrawingContainer().delShape(inlineShape);
            document.getDrawingContainer().delShape(inlineShape);
            // java if (inlineShape.getParent() == null)
            if inlineShape.getParent().is_none() {
              // java document.getTextDocument().submit(new KDeleteCommand((Shape)null, inlineShape));
              document.getTextDocument().submit(KDeleteCommand::new(None, inlineShape));
            }
          }
          // java document.getPieceTable().atomicRemoveRangeDisableRevision(start, end);
          document.getPieceTable().atomicRemoveRangeDisableRevision(start, end);
        }
        // java else
        else {
          // java KRange shapeRange = new KRange(document, start, end);
          let mut shapeRange: KRange = KRange::new(document, start, end);
          // java shapeRange.removeText();
          shapeRange.removeText();
          // java start = shapeRange.getStart();
          start = shapeRange.getStart();
          // java end = shapeRange.getEnd();
          end = shapeRange.getEnd();
          // java range = RANGEFactory.getRANGEObject(start, end);
          range = RANGEFactory::getRANGEObject(start, end);
          // java shapeRange.dispose();
          shapeRange.dispose();
        }
        // java return range;
        return range;
      }
    }

    // java public static Long removeShapeAnchor(Document document, KShape coreShape)
    pub fn removeShapeAnchor(document: &dyn Document, coreShape: &KShape) -> Option<i64> {
      // java Shape shape = coreShape.getShape();
      let mut shape: Shape = coreShape.getShape();
      // java Shape inlineShape = coreShape.getInlineShape();
      let mut inlineShape: Shape = coreShape.getInlineShape();
      // java Long range = getShapeRange(document, shape);
      let mut range: i64 = Self::getShapeRange(&document, &shape);
      // java if (range == null)
      if range.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java boolean isDeleteShape = checkDeleteShape(document, range);
        let mut isDeleteShape: bool = Self::checkDeleteShape(&document, range);
        // java int shapeId = shape.getShapeID();
        let mut shapeId: i32 = shape.getShapeID();
        // java PLCShapeAnchor shapes = null;
        let mut shapes: PLCShapeAnchor = PLCShapeAnchor::default();
        // java PLCShapeAnchor.ShapeAnchor anchor = null;
        let mut anchor: PLCShapeAnchor::ShapeAnchor = PLCShapeAnchor::ShapeAnchor::default();
        // java boolean isTextbox = coreShape.isTextBox() || shape.isOleShape();
        let mut isTextbox: bool = coreShape.isTextBox() || shape.isOleShape();
        // java if (isTextbox)
        if isTextbox {
          // java shapes = document.getShapeAnchors();
          shapes = document.getShapeAnchors();
          // java anchor = shapes.findById(shapeId);
          anchor = shapes.findById(shapeId);
          // java anchor.setInvalidShapeId();
          anchor.setInvalidShapeId();
        }
        // java if (isDeleteShape)
        if isDeleteShape {
          // java if (inlineShape != null)
          if inlineShape.is_some() {
            // java document.getDrawingContainer().delShape(inlineShape);
            document.getDrawingContainer().delShape(inlineShape);
          }
          // java document.getDrawingContainer().delShape(shape);
          document.getDrawingContainer().delShape(shape);
          // java document.getPieceTable().atomicRemoveRangeDisableRevision(RANGEFactory.getStart(range), RANGEFactory.getEnd(range));
          document.getPieceTable().atomicRemoveRangeDisableRevision(RANGEFactory::getStart(range), RANGEFactory::getEnd(range));
        }
        // java else
        else {
          // java document.getTextStream().removeText(RANGEFactory.getStart(range), RANGEFactory.getEnd(range));
          document.getTextStream().removeText(RANGEFactory::getStart(range), RANGEFactory::getEnd(range));
        }
        // java if (isTextbox)
        if isTextbox {
          // java anchor.setShapeId(shapeId);
          anchor.setShapeId(shapeId);
        }
        // java return range;
        return range;
      }
    }

    // java private static boolean checkDeleteShape(Document document, Long range)
    pub fn checkDeleteShape(document: &dyn Document, range: i64) -> bool {
      // java KSAssert.assertNotNull("document should not be null!", document);
      KSAssert::assertNotNull("document should not be null!", document);
      // java KSAssert.assertNotNull("range should not be null!", range);
      KSAssert::assertNotNull("range should not be null!", range);
      // java if (document.getTextDocument().getTrackChange() && document.getTextStream().getRevisionFileter())
      if document.getTextDocument().getTrackChange() && document.getTextStream().getRevisionFileter() {
        // java KParaRunRevision mKRevision = new KParaRunRevision(document);
        let mut mKRevision: KParaRunRevision = KParaRunRevision::new(document);
        // java IChpxTable iChpxTable = document.getChpxTable();
        let mut iChpxTable: Box<dyn IChpxTable> = document.getChpxTable();
        // java IChpxTable.Entry chpxStart = iChpxTable.seek(RANGEFactory.getStart(range));
        let mut chpxStart: Box<dyn ChpxEntry> = iChpxTable.seek(RANGEFactory::getStart(range));
        // java IChpxTable.Entry chpxEnd = iChpxTable.seek(RANGEFactory.getEnd(range));
        let mut chpxEnd: Box<dyn ChpxEntry> = iChpxTable.seek(RANGEFactory::getEnd(range));
        let mut e: Box<dyn ChpxEntry> = chpxStart;
        // java for(IChpxTable.Entry e = chpxStart; e != chpxEnd e = e.getNext())
        while e != chpxEnd {
          // java if (mKRevision.checkDeleteRun(e.getProperty()))
          if mKRevision.checkDeleteRun(e.getProperty()) {
            // java return true;
            return true;
          }
        }
        // java return false;
        return false;
      }
      // java else
      else {
        // java return true;
        return true;
      }
    }

    // java public static Long getShapeRange(Document document, Shape shape)
    pub fn getShapeRange(document: &dyn Document, shape: &Shape) -> Option<i64> {
      // java KSAssert.assertNotNull("document should not be null!", document);
      KSAssert::assertNotNull("document should not be null!", document);
      // java KSAssert.assertNotNull("shape should not be null!", shape);
      KSAssert::assertNotNull("shape should not be null!", shape);
      // java int cp = position(document, shape);
      let mut cp: i32 = Self::position(&document, &shape);
      // java if (cp == -1)
      if cp == -1 {
        // java KSLog.w("ShapeTool", "Invalid shape");
        KSLog::w__string_string(&"ShapeTool", &"Invalid shape");
        // java return null;
        return None;
      }
      // java else
      else {
        // java int start = cp;
        let mut start: i32 = cp;
        // java int end = cp + 1;
        let mut end: i32 = cp + 1;
        // java if (shape.getInlineShape())
        if shape.getInlineShape() {
          // java PLCField plcField = document.getPLCField();
          let mut plcField: PLCField = document.getPLCField();
          // java if (plcField != null)
          if plcField.is_some() {
            // java PLCField.FieldTriple triple = plcField.getContainingField(cp);
            let mut triple: PLCField::FieldTriple = plcField.getContainingField(cp);
            // java if (isShapeTriple(triple))
            if Self::isShapeTriple(Some(&triple)) {
              // java start = triple.start.getCP();
              start = triple.start.getCP();
              // java end = triple.end.getCP() + 1;
              end = triple.end.getCP() + 1;
            }
          }
        }
        // java return RANGEFactory.getRANGEObject(start, end);
        return RANGEFactory::getRANGEObject(start, end);
      }
    }

    // java public static void insertShape(Document document, int cp, char anchor, Shape shape, PropertySet prop)
    pub fn insertShape__document_int_char_shape_propertyset(document: &dyn Document, cp: i32, anchor: char, shape: &Shape, prop: &PropertySet) {
      // java document.getPieceTable().atomicInsertShape(cp, anchor, shape, PropertySet.merge(prop, PropertySet.valueOf(28, true)));
      document.getPieceTable().atomicInsertShape(cp, anchor, shape, PropertySet::merge(&prop, &PropertySet::valueOf__int_object(28, &true)));
    }

    // java public static void insertShape(Document document, int cp, char anchor, Shape shape)
    pub fn insertShape__document_int_char_shape(document: &dyn Document, cp: i32, anchor: char, shape: &Shape) {
      // java document.getPieceTable().atomicInsertShape(cp, anchor, shape, PropertySet.valueOf(28, true));
      document.getPieceTable().atomicInsertShape(cp, anchor, shape, PropertySet::valueOf__int_object(28, &true));
    }

    // java public static RectF getScaleRect(Shape shape)
    pub fn getScaleRect(shape: &Shape) -> RectF {
      // java ShapePos shapePos = (ShapePos)shape.getShapePos();
      let mut shapePos: ShapePos = (shape.getShapePos()).downcast_ref::<ShapePos>().unwrap();
      // java RectF rectF = shapePos.getShapeRect();
      let mut rectF: RectF = shapePos.getShapeRect();
      // java RectF newRectF = new RectF(rectF);
      let mut newRectF: RectF = RectF::new(rectF);
      // java newRectF.right = rectF.left + rectF.width() * shapePos.getScaleH();
      newRectF.right = rectF.left + rectF.width() * shapePos.getScaleH();
      // java newRectF.bottom = rectF.top + rectF.height() * shapePos.getScaleV();
      newRectF.bottom = rectF.top + rectF.height() * shapePos.getScaleV();
      // java return newRectF;
      return newRectF;
    }

    // java public static void adjustShapePos(Shape shape, float left, float top)
    pub fn adjustShapePos__shape_float_float(shape: &Shape, left: f32, top: f32) {
      // java ShapePos shapePos = (ShapePos)shape.getShapePos();
      let mut shapePos: ShapePos = (shape.getShapePos()).downcast_ref::<ShapePos>().unwrap();
      // java shapePos.setPosRelH(2);
      shapePos.setPosRelH(2);
      // java shapePos.setPosRelV(2);
      shapePos.setPosRelV(2);
      // java RectF shapeRectF = shapePos.getShapeRect();
      let mut shapeRectF: RectF = shapePos.getShapeRect();
      // java RectF newRectF = new RectF(shapeRectF);
      let mut newRectF: RectF = RectF::new(shapeRectF);
      // java newRectF.offsetTo(left, top);
      newRectF.offsetTo(left, top);
      // java shapePos.setShapeRect(newRectF);
      shapePos.setShapeRect(newRectF);
    }

    // java public static void adjustShapePos(Shape shape, int left, int top, int relH, int relV)
    pub fn adjustShapePos__shape_int_int_int_int(shape: &Shape, left: i32, top: i32, relH: i32, relV: i32) {
      // java ShapePos shapePos = (ShapePos)shape.getShapePos();
      let mut shapePos: ShapePos = (shape.getShapePos()).downcast_ref::<ShapePos>().unwrap();
      // java shapePos.setPosRelH(relH);
      shapePos.setPosRelH(relH);
      // java shapePos.setPosRelV(relV);
      shapePos.setPosRelV(relV);
      // java shapePos.setAlignmentH(0);
      shapePos.setAlignmentH(0);
      // java shapePos.setAlignmentV(0);
      shapePos.setAlignmentV(0);
      // java RectF shapeRectF = shapePos.getShapeRect();
      let mut shapeRectF: RectF = shapePos.getShapeRect();
      // java RectF newRectF = new RectF(shapeRectF);
      let mut newRectF: RectF = RectF::new(shapeRectF);
      // java newRectF.offsetTo(ShapeMetrics.transFromLayout((float)left), ShapeMetrics.transFromLayout((float)top));
      newRectF.offsetTo(ShapeMetrics::transFromLayout__float(((left) as f32)), ShapeMetrics::transFromLayout__float(((top) as f32)));
      // java shapePos.setShapeRect(newRectF);
      shapePos.setShapeRect(newRectF);
    }

    // java public static boolean isInTable(Document document, int cp)
    pub fn isInTable(document: &dyn Document, cp: i32) -> bool {
      // java IPapxTable.Entry entry = document.getPapxTable().seek(cp);
      let mut entry: Box<dyn PapxEntry> = document.getPapxTable().seek(cp);
      // java return entry.getProperty().getBoolean(223, false);
      return entry.getProperty().getBoolean(223, false);
    }

    // java public static PointF getSuitablePicSize(PageSetup pageSetup, float width, float height)
    pub fn getSuitablePicSize__pagesetup_float_float(pageSetup: &PageSetup, width: f32, height: f32) -> PointF {
      // java float maxWidth = (float)(pageSetup.getWidth() - pageSetup.getMarginLeft() - pageSetup.getMarginRight());
      let mut maxWidth: f32 = ((pageSetup.getWidth() - pageSetup.getMarginLeft() - pageSetup.getMarginRight()) as f32);
      // java float maxHeight = (float)(pageSetup.getHeight() - pageSetup.getMarginTop() - pageSetup.getMarginBottom());
      let mut maxHeight: f32 = ((pageSetup.getHeight() - pageSetup.getMarginTop() - pageSetup.getMarginBottom()) as f32);
      // java float picWidth = MetricsUtil.pixel2point_x(width);
      let mut picWidth: f32 = MetricsUtil::pixel2point_x(width);
      // java float picHeight = MetricsUtil.pixel2point_y(height);
      let mut picHeight: f32 = MetricsUtil::pixel2point_y(height);
      // java float xScale = ShapeMetrics.transFromLayout(maxWidth) / picWidth;
      let mut xScale: f32 = ShapeMetrics::transFromLayout__float(maxWidth) / picWidth;
      // java float yScale = ShapeMetrics.transFromLayout(maxHeight) / picHeight;
      let mut yScale: f32 = ShapeMetrics::transFromLayout__float(maxHeight) / picHeight;
      // java if (xScale < 1.0F || yScale < 1.0F)
      if xScale < 1 || yScale < 1 {
        // java float scale = xScale > yScale ? yScale : xScale;
        let mut scale: f32 = (if xScale > yScale { yScale } else { xScale });
        // java picWidth *= scale;
        picWidth = picWidth * scale;
        // java picHeight *= scale;
        picHeight = picHeight * scale;
      }
      // java return new PointF(picWidth, picHeight);
      return PointF::new(picWidth, picHeight);
    }

    // java public static PointF getSuitablePicSize(float picWidth, float picHeight, float maxWidth, float maxHeight)
    pub fn getSuitablePicSize__float_float_float_float(picWidth: f32, picHeight: f32, maxWidth: f32, maxHeight: f32) -> PointF {
      // java picWidth = MetricsUtil.pixel2twips_x(picWidth);
      picWidth = MetricsUtil::pixel2twips_x(picWidth);
      // java picHeight = MetricsUtil.pixel2twips_y(picHeight);
      picHeight = MetricsUtil::pixel2twips_y(picHeight);
      // java float xScale = maxWidth / picWidth;
      let mut xScale: f32 = maxWidth / picWidth;
      // java float yScale = maxHeight / picHeight;
      let mut yScale: f32 = maxHeight / picHeight;
      // java if (xScale < 1.0F || yScale < 1.0F)
      if xScale < 1 || yScale < 1 {
        // java float scale = xScale > yScale ? yScale : xScale;
        let mut scale: f32 = (if xScale > yScale { yScale } else { xScale });
        // java picWidth *= scale;
        picWidth = picWidth * scale;
        // java picHeight *= scale;
        picHeight = picHeight * scale;
      }
      // java return new PointF(picWidth, picHeight);
      return PointF::new(picWidth, picHeight);
    }

    // java public static void deleteChildShape(Document document, KShape shape)
    pub fn deleteChildShape(document: &dyn Document, shape: &KShape) {
      // java Shape coreShape = shape.getShape().getBaseShape();
      let mut coreShape: Shape = shape.getShape().getBaseShape();
      // java GroupShape parent = (GroupShape)coreShape.getParent();
      let mut parent: GroupShape = (coreShape.getParent()).downcast_ref::<GroupShape>().unwrap();
      // java if (parent != null)
      if parent.is_some() {
        // java if (coreShape.isGroupShape())
        if coreShape.isGroupShape() {
          // java deleteSubText(document, (GroupShape)coreShape);
          Self::deleteSubText(&document, &(coreShape).downcast_ref::<GroupShape>().unwrap());
        }
        // java else
        else {
          // java deleteText(document, coreShape.getShapeID());
          Self::deleteText(&document, coreShape.getShapeID());
        }
        // java IDrawingContainer drawingContainer = document.getDrawingContainer();
        let mut drawingContainer: Box<dyn IDrawingContainer> = document.getDrawingContainer();
        // java drawingContainer.delShape(coreShape);
        drawingContainer.delShape(coreShape);
        // java document.getTextDocument().submit(new KGrpDeleteChildCommand(parent, coreShape));
        // CARGO_BLOCKER: KGrpDeleteChildCommand not generated
        let _ = (parent, coreShape);
      }
    }

    // java public static void addChildShape(Document document, Shape shape, GroupShape newParent)
    pub fn addChildShape(document: &dyn Document, shape: &Shape, newParent: Option<&GroupShape>) {
      // java Shape coreShape = shape.getBaseShape();
      let mut coreShape: Shape = shape.getBaseShape();
      // java if (newParent != null)
      if let Some(newParent) = newParent {
        // java KGrpAddChildCommand grpAddCmd = new KGrpAddChildCommand(newParent, coreShape);
        // CARGO_BLOCKER: KGrpAddChildCommand not generated
        let _ = (newParent, coreShape);
      }
    }

    // java public static Shape getAncestor(Shape shape)
    pub fn getAncestor(shape: Option<&mut Shape>) -> Option<Shape> {
      // java if (shape == null)
      if shape.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java while (shape.getParent() != null)
        while shape.getParent().is_some() {
          // java shape = shape.getParent();
          shape = shape.getParent();
        }
        // java return shape;
        return shape;
      }
    }

    // java public static boolean getFlipV(Shape shape)
    pub fn getFlipV(shape: Option<&Shape>) -> Option<bool> {
      // java if (shape == null)
      if shape.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java boolean isFlipV = shape.isFlipV();
        let mut isFlipV: bool = shape.isFlipV();
        // java Shape parent = shape.getParent();
        let mut parent: Shape = shape.getParent();
        // java return parent != null ? isFlipV ^ getFlipV(parent) : isFlipV;
        return (if parent.is_some() { Some(isFlipV ^ Self::getFlipV(Some(&parent))) } else { Some(isFlipV) });
      }
    }

    // java public static boolean getFlipH(Shape shape)
    pub fn getFlipH(shape: Option<&Shape>) -> Option<bool> {
      // java if (shape == null)
      if shape.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java boolean isFlipH = shape.isFlipH();
        let mut isFlipH: bool = shape.isFlipH();
        // java Shape parent = shape.getParent();
        let mut parent: Shape = shape.getParent();
        // java return parent != null ? isFlipH ^ getFlipH(parent) : isFlipH;
        return (if parent.is_some() { Some(isFlipH ^ Self::getFlipH(Some(&parent))) } else { Some(isFlipH) });
      }
    }

    // java public static float getShapeScale(GroupShape shape)
    pub fn getShapeScale(shape: Option<&GroupShape>) -> Option<f32> {
      // java if (shape == null)
      if shape.is_none() {
        // java return 1.0F;
        return 1;
      }
      // java else
      else {
        // java RectF rectCoord = shape.getCoordBounds();
        let mut rectCoord: RectF = shape.getCoordBounds();
        // java ShapePos shapePos = (ShapePos)shape.getShapePos();
        let mut shapePos: ShapePos = (shape.getShapePos()).downcast_ref::<ShapePos>().unwrap();
        // java if (shapePos == null)
        if shapePos.is_none() {
          // java return 1.0F;
          return 1;
        }
        // java else
        else {
          // java RectF rectLayout = shape.getShapePos().getShapeRect();
          let mut rectLayout: RectF = shape.getShapePos().getShapeRect();
          // java float scaleX = rectCoord.width() / rectLayout.width();
          let mut scaleX: f32 = rectCoord.width() / rectLayout.width();
          // java return shape.getParent() != null ? scaleX * getShapeScale((GroupShape)shape.getParent()) : scaleX;
          return (if shape.getParent().is_some() { Some(scaleX * Self::getShapeScale(Some(&(shape.getParent()).downcast_ref::<GroupShape>().unwrap()))) } else { Some(scaleX) });
        }
      }
    }

    // java public static boolean canOpenOle(KShape kShape)
    pub fn canOpenOle__kshape(kShape: Option<&KShape>) -> Option<bool> {
      // java return kShape == null ? false : canOpenOle(kShape.getShape());
      return (if kShape.is_none() { Some(false) } else { Some(Self::canOpenOle__kshape(Some(&kShape.getShape()))) });
    }

    // java public static boolean canOpenOle(Shape shape)
    pub fn canOpenOle__shape(shape: Option<&Shape>) -> bool {
      // java if (shape == null)
      if shape.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java OLE ole = getShapeOle(shape);
        let mut ole: OLE = Self::getShapeOle(Some(&shape));
        // java if (null != ole)
        if ole.is_some() {
          // java String progId = ole.getProgId();
          let mut progId: String = ole.getProgId();
          // java if (!StringUtil.isEmpty(progId))
          if !StringUtil::isEmpty(Some(&mut progId)) {
            // java return OleClsTypeMatcher.canOpenOle(progId);
            return OleClsTypeMatcher::canOpenOle(Some(&mut progId));
          }
        }
        // java return false;
        return false;
      }
    }

    // java public static OLE getShapeOle(Shape shape)
    pub fn getShapeOle(shape: Option<&Shape>) -> Option<OLE> {
      // java if (shape != null && !shape.isTextBox())
      if let Some(shape) = shape {
        if !shape.isTextBox() {
          // java Document document = (Document)shape.getSubDocument();
          let mut document: Box<dyn Document> = todo!("cast → Box<dyn Document>");
          // java boolean isInlineShape = shape.getInlineShape();
          let mut isInlineShape: bool = shape.getInlineShape();
          // java if (isInlineShape)
          if isInlineShape {
            // java PLCShapeAnchor plcShapeAnchor = document.getShapeAnchors();
            let mut plcShapeAnchor: PLCShapeAnchor = document.getShapeAnchors();
            // java PLCShapeAnchor.ShapeAnchor anchor = plcShapeAnchor.findById(shape.getShapeID());
            let mut anchor: PLCShapeAnchor::ShapeAnchor = plcShapeAnchor.findById(shape.getShapeID());
            // java if (anchor != null)
            if anchor.is_some() {
              // java return getOleByCp(document, anchor.getCP());
              return Self::getOleByCp(&document, anchor.getCP());
            }
          }
          // java else
          else {
            // java Document textBoxDocument = document.getTextboxDocument();
            let mut textBoxDocument: Box<dyn Document> = document.getTextboxDocument();
            // java if (textBoxDocument == null)
            if textBoxDocument.is_none() {
              // java return null;
              return None;
            }
            // java PLCTextboxText plcTextboxText = textBoxDocument.getTextboxText();
            let mut plcTextboxText: PLCTextboxText = textBoxDocument.getTextboxText();
            // java PLCTextboxText.TextboxTextNode textNode = plcTextboxText.findTextboxByShapeId(shape.getShapeID());
            let mut textNode: PLCTextboxText::TextboxTextNode = plcTextboxText.findTextboxByShapeId(shape.getShapeID());
            // java if (null != textNode)
            if textNode.is_some() {
              // java return getOleByCp(textBoxDocument, textNode.getCP());
              return Self::getOleByCp(&textBoxDocument, textNode.getCP());
            }
          }
          // java return null;
          return None;
        }
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public static String getOleFilePath(Shape shape)
    pub fn getOleFilePath(shape: Option<&Shape>) -> Option<String> {
      // java if (shape == null)
      if shape.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java Document document = (Document)shape.getSubDocument();
        let mut document: Box<dyn Document> = todo!("cast → Box<dyn Document>");
        // java TextDocument textDocument = document.getTextDocument();
        let mut textDocument: TextDocument = document.getTextDocument();
        // java IMediaLib mediaLib = textDocument.getMediaLib();
        let mut mediaLib: Box<dyn IMediaLib> = textDocument.getMediaLib();
        // java String oleFilePath = null;
        let mut oleFilePath: String = String::new();
        // java OLE ole = getShapeOle(shape);
        let mut ole: OLE = Self::getShapeOle(Some(&shape));
        // java if (null != ole)
        if ole.is_some() {
          // java oleFilePath = mediaLib.getMediaByID(ole.getMediaID(), MediaTypeEnum.OLE);
          oleFilePath = mediaLib.getMediaByID(ole.getMediaID(), MediaTypeEnum::OLE);
        }
        // java return oleFilePath;
        return oleFilePath;
      }
    }

    // java public static OLE getOleByCp(Document document, int cp)
    pub fn getOleByCp(document: &dyn Document, cp: i32) -> Option<OLE> {
      // java PLCField plcField = document.getPLCField();
      let mut plcField: PLCField = document.getPLCField();
      // java PLCField.FieldTriple fieldTriple = plcField.getMatchCpField(cp);
      let mut fieldTriple: PLCField::FieldTriple = plcField.getMatchCpField(cp);
      // java if (null == fieldTriple)
      if fieldTriple.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java IChpxTable.Entry run = document.getChpxTable().seek(fieldTriple.getSepCP());
        let mut run: Box<dyn ChpxEntry> = document.getChpxTable().seek(fieldTriple.getSepCP());
        // java if (run.isEnd())
        if run.isEnd() {
          // java return null;
          return None;
        }
        // java else
        else {
          // java PropertySet propertySet = run.getProperty();
          let mut propertySet: PropertySet = run.getProperty();
          // java Object object = propertySet.getObject(56, (Object)null);
          let mut object: Box<dyn Object> = propertySet.getObject(56, None);
          // java if (null != object && object instanceof Shape)
          if object.is_some() && object.downcast_ref::<Shape>().is_some() {
            // java Shape oleShape = (Shape)object;
            let mut oleShape: Shape = (object).downcast_ref::<Shape>().unwrap();
            // java return oleShape.getOLE();
            return oleShape.getOLE();
          }
          // java else
          else {
            // java return null;
            return None;
          }
        }
      }
    }

    // java public static boolean exists(Shape shape)
    pub fn exists(shape: Option<&Shape>) -> Option<bool> {
      // java if (null == shape)
      if shape.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java IDrawingContainer drawingContainer = shape.getDrawingContainer();
        let mut drawingContainer: Box<dyn IDrawingContainer> = shape.getDrawingContainer();
        // java if (null != drawingContainer)
        if drawingContainer.is_some() {
          // java return null != drawingContainer.findShapeByID(shape.getShapeID());
          return drawingContainer.findShapeByID(shape.getShapeID()).is_some();
        }
        // java else
        else {
          // java return false;
          return false;
        }
      }
    }

    // java public static int insertOle(Document document, Shape shape, int cp, String classType, String fileName)
    pub fn insertOle(document: &dyn Document, shape: &Shape, cp: i32, classType: &str, fileName: &str) -> i32 {
      // java int originCP = cp;
      let mut originCP: i32 = cp;
      // java CPieceTable pieceTbl = document.getPieceTable();
      let mut pieceTbl: CPieceTable = document.getPieceTable();
      // java pieceTbl.atmoicInsertField('\u0013', cp++, PropertySet.valueOf(28, true), KFieldType.FieldEmbed);
      pieceTbl.atmoicInsertField('\u{0013}', { let _tmp = cp; cp += 1; _tmp }, PropertySet::valueOf__int_object(28, &true), KFieldType::FieldEmbed);
      // java String fldCode = " " + KFieldType.FieldEmbed + " " + classType + " ";
      let mut fldCode: String = " " + KFieldType::FieldEmbed + " " + classType + " ";
      // java pieceTbl.atomicInsertText(fldCode, cp, PropertySet.empty);
      pieceTbl.atomicInsertText(fldCode, cp, PropertySet::empty());
      // java cp += fldCode.length();
      cp = cp + (fldCode.len() as i32);
      // java PropertySet prop = prepareOleSepProp(document, fileName, classType);
      let mut prop: PropertySet = Self::prepareOleSepProp(&document, &fileName, &classType);
      // java pieceTbl.atmoicInsertField('\u0014', cp++, prop, (KFieldType)null);
      pieceTbl.atmoicInsertField('\u{0014}', { let _tmp = cp; cp += 1; _tmp }, prop, None);
      // java pieceTbl.atomicInsertShape(cp++, '\u0001', shape, PropertySet.valueOf(28, true));
      pieceTbl.atomicInsertShape({ let _tmp = cp; cp += 1; _tmp }, '\u{0001}', shape, PropertySet::valueOf__int_object(28, &true));
      // java pieceTbl.atmoicInsertField('\u0015', cp++, PropertySet.valueOf(28, true), (KFieldType)null);
      pieceTbl.atmoicInsertField('\u{0015}', { let _tmp = cp; cp += 1; _tmp }, PropertySet::valueOf__int_object(28, &true), None);
      // java return cp - originCP;
      return cp - originCP;
    }

    // java public static PropertySet prepareOleSepProp(Document document, String fileName, String progId)
    pub fn prepareOleSepProp(document: &dyn Document, fileName: &str, progId: &str) -> PropertySet {
      // java MutablePropertySet prop = new MutablePropertySet();
      let mut prop: MutablePropertySet = MutablePropertySet::new();
      // java prop.setBoolean(28, true);
      prop.setBoolean(28, true);
      // java prop.setBoolean(30, true);
      prop.setBoolean(30, true);
      // java prop.setBoolean(29, true);
      prop.setBoolean(29, true);
      // java OLE ole = createOLE(fileName, document, progId);
      let mut ole: OLE = Self::createOLE(&mut fileName, &document, &progId);
      // java IDrawingContainer drawingContainer = document.getDrawingContainer();
      let mut drawingContainer: Box<dyn IDrawingContainer> = document.getDrawingContainer();
      // java Shape shape = ShapeFactory.getShapeFactory().createShape(drawingContainer);
      let mut shape: Shape = ShapeFactory::getShapeFactory().createShape(drawingContainer);
      // java shape.setShapeID(drawingContainer.newShapeID());
      shape.setShapeID(drawingContainer.newShapeID());
      // java shape.setOLE(ole);
      shape.setOLE(ole);
      // java prop.setValue(56, shape);
      prop.setValue(56, shape);
      // java return prop.freeze();
      return prop.freeze();
    }

    // java public static OLE createOLE(String fileName, Document document, String progId)
    pub fn createOLE(fileName: &mut str, document: &dyn Document, progId: &str) -> OLE {
      // java IDrawingContainer container = document.getDrawingContainer();
      let mut container: Box<dyn IDrawingContainer> = document.getDrawingContainer();
      // java IMediaLib mediaLib = container.getMediaLib();
      let mut mediaLib: Box<dyn IMediaLib> = container.getMediaLib();
      // java String binPath = document.getTextDocument().getTmpRootDir() + System.currentTimeMillis() + fileName.substring(fileName.lastIndexOf("."));
      let mut binPath: String = document.getTextDocument().getTmpRootDir() + System::currentTimeMillis() + fileName.substring(fileName.lastIndexOf("."));
      // java FileUtil.copyFileOrg(fileName, binPath);
      FileUtil::copyFileOrg__string_string(Some(&fileName), Some(&binPath));
      // java int mediaId = mediaLib.addMedia(binPath, binPath, MediaTypeEnum.OLE);
      let mut mediaId: i32 = mediaLib.addMedia(binPath, binPath, MediaTypeEnum::OLE);
      // java OLE ole = new OLE(mediaId);
      let mut ole: OLE = OLE::new(mediaId);
      // java ole.setProgId(progId);
      ole.setProgId(progId);
      // java return ole;
      return ole;
    }

    // java public static int addOleToMediaLib(String filePath, IDrawingContainer drawingContainer, boolean bCopyFile)
    pub fn addOleToMediaLib(filePath: Option<&str>, drawingContainer: &dyn IDrawingContainer, bCopyFile: bool) -> i32 {
      // java int mediaID = -1;
      let mut mediaID: i32 = -1;
      // java if (filePath == null)
      if filePath.is_none() {
        // java return mediaID;
        return mediaID;
      }
      // java else
      else {
        // java File file = new File(filePath);
        let mut file: std::path::PathBuf = File::new(filePath);
        // java if (file.exists() && file.length() >= 1L)
        if file.exists() && (file.len() as i32) >= 1 {
          // java String picName = file.getName();
          let mut picName: String = file.getName();
          // java if (bCopyFile)
          if bCopyFile {
            // java String picPath = drawingContainer.getTmpDir() + file.getName();
            let mut picPath: String = drawingContainer.getTmpDir() + file.getName();
            // java FileUtil.copyFileOrg(filePath, picPath);
            FileUtil::copyFileOrg__string_string(Some(&filePath), Some(&picPath));
            // java mediaID = drawingContainer.getMediaLib().addMedia(picPath, picPath, MediaTypeEnum.OLE);
            mediaID = drawingContainer.getMediaLib().addMedia(picPath, picPath, MediaTypeEnum::OLE);
          }
          // java else
          else {
            // java mediaID = drawingContainer.getMediaLib().addMedia(picName, filePath, MediaTypeEnum.OLE);
            mediaID = drawingContainer.getMediaLib().addMedia(picName, filePath, MediaTypeEnum::OLE);
          }
          // java return mediaID;
          return mediaID;
        }
        // java else
        else {
          // java return mediaID;
          return mediaID;
        }
      }
    }

    // java public static OLE getOLEWithProgId(Document document, Shape shape)
    pub fn getOLEWithProgId(document: &dyn Document, shape: &Shape) -> Option<OLE> {
      // java Document oleDoc = null;
      let mut oleDoc: Box<dyn Document> = Box::new(()) as Box<dyn Document>;
      // java Integer cp = null;
      let mut cp: i32 = 0;
      // java if (shape.isInDggInfo())
      if shape.isInDggInfo() {
        // java oleDoc = document.getTextboxDocument();
        oleDoc = document.getTextboxDocument();
        // java cp = getCpContainingOle(shape);
        cp = Self::getCpContainingOle(&shape);
      }
      // java else
      else {
        // java oleDoc = document;
        oleDoc = document;
        // java cp = position(document, shape);
        cp = Self::position(&document, &shape);
      }
      // java if (null != oleDoc && null != cp)
      if oleDoc.is_some() && cp.is_some() {
        // java PLCField plcField = oleDoc.getPLCField();
        let mut plcField: PLCField = oleDoc.getPLCField();
        // java PLCField.FieldTriple triple = plcField.getContainingField(cp);
        let mut triple: PLCField::FieldTriple = plcField.getContainingField(cp);
        // java if (!isOleTriple(triple))
        if !Self::isOleTriple(Some(&triple)) {
          // java return null;
          return None;
        }
        // java else
        else {
          // java IChpxTable.Entry chpx = oleDoc.getChpxTable().seek(triple.separator.getCP());
          let mut chpx: Box<dyn ChpxEntry> = oleDoc.getChpxTable().seek(triple.separator.getCP());
          // java PropertySet prop = chpx.getProperty();
          let mut prop: PropertySet = chpx.getProperty();
          // java Shape oleShape = (Shape)prop.getValue(56);
          let mut oleShape: Shape = (prop.getValue(56)).downcast_ref::<Shape>().unwrap();
          // java OLE ole = oleShape.getOLE();
          let mut ole: OLE = oleShape.getOLE();
          // java return ole;
          return ole;
        }
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java private static Integer getCpContainingOle(Shape shape)
    pub fn getCpContainingOle(shape: &Shape) -> Option<i32> {
      // java TextFrame textFrame = (TextFrame)shape.getTextObj();
      let mut textFrame: TextFrame = (shape.getTextObj()).downcast_ref::<TextFrame>().unwrap();
      // java PLCTextboxText.TextboxTextNode node = textFrame.getNode();
      let mut node: PLCTextboxText::TextboxTextNode = textFrame.getNode();
      // java return null == node ? null : node.getCP();
      return (if node.is_none() { None } else { Some(node.getCP()) });
    }

    // java public static ArrayList<Shape> getShapes(KRange range)
    pub fn getShapes(range: Option<&KRange>) -> Vec<Shape> {
      // java ArrayList<Shape> shapes = new ArrayList();
      let mut shapes: Vec<Shape> = Vec::new();
      // java Document subDoc = range.getSubDocument();
      let mut subDoc: Box<dyn Document> = range.getSubDocument();
      // java if (null != range)
      if let Some(range) = range {
        // java int start = range.getStart();
        let mut start: i32 = range.getStart();
        // java int end = Math.min(range.getEnd(), subDoc.getLength());
        let mut end: i32 = Math::min(range.getEnd(), subDoc.getLength());
        // java IDrawingContainer drawingContainer = subDoc.getDrawingContainer();
        let mut drawingContainer: Box<dyn IDrawingContainer> = subDoc.getDrawingContainer();
        // java PLCShapeAnchor plcShapeAnchor = subDoc.getShapeAnchors();
        let mut plcShapeAnchor: PLCShapeAnchor = subDoc.getShapeAnchors();
        let mut s: Box<dyn Node> = plcShapeAnchor.lowerBound(start);
        // java for(PLC.Node s = plcShapeAnchor.lowerBound(start); !s.isEnd() && s.position() <= end s = s.getNext())
        while !s.isEnd() && s.position() <= end {
          // java PLCShapeAnchor.ShapeAnchor anchor = (PLCShapeAnchor.ShapeAnchor)s;
          let mut anchor: PLCShapeAnchor::ShapeAnchor = (s).downcast_ref::<PLCShapeAnchor::ShapeAnchor>().unwrap();
          // java Shape shape = drawingContainer.findShapeByID(anchor.getShapeId());
          let mut shape: Shape = drawingContainer.findShapeByID(anchor.getShapeId());
          // java if (null != shape && !CommentInfo.isInvisibaleCP(subDoc, s.position()))
          if shape.is_some() && !CommentInfo::isInvisibaleCP(&subDoc, s.position()) {
            // java shapes.add(shape);
            shapes.add(shape);
          }
        }
      }
      // java return shapes;
      return shapes;
    }

}
