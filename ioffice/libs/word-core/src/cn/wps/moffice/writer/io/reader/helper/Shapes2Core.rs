// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.drawing.GRF
use drawing::cn::wps::moffice::drawing::GRF::*;
// Java: import cn.wps.moffice.drawing.IDiagram
use drawing::cn::wps::moffice::drawing::IDiagram::*;
// Java: import cn.wps.moffice.drawing.IPos
use drawing::cn::wps::moffice::drawing::IPos::*;
// Java: import cn.wps.moffice.drawing.Protection
use drawing::cn::wps::moffice::drawing::Protection::*;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::*;
// Java: import cn.wps.moffice.drawing.ShapePos
use drawing::cn::wps::moffice::drawing::ShapePos::*;
// Java: import cn.wps.moffice.drawing.chart.IChart
use drawing::cn::wps::moffice::drawing::chart::IChart::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.helper.BitsOperation
use crate::cn::wps::moffice::writer::core::helper::BitsOperation::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.helper.ShapeIDTool
use crate::cn::wps::moffice::writer::io::reader::docxReader::helper::ShapeIDTool::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
use std::collections::{HashMap};

// java public abstract class Shapes2Core
#[derive(Default)]
pub struct Shapes2Core {
    // java protected SubDocument mDocument;
    pub(crate) mDocument: Option<SubDocument>,
    // java protected Shape mShape;
    pub(crate) mShape: Option<Shape>,
    // java protected MutablePropertySet mRunProps;
    pub(crate) mRunProps: Option<MutablePropertySet>,
    // java private boolean mIsVml;
    mIsVml: bool,
    // java private boolean mHasOle = false;
    mHasOle: bool,
}

// java public static final class OLEObjectInfo
#[derive(Default)]
pub struct OLEObjectInfo {
    // java public String mType;
    pub(crate) mType: Option<String>,
    // java public String mProgID;
    pub(crate) mProgID: Option<String>,
    // java public String mRid;
    pub(crate) mRid: Option<String>,
    // java public int mMediaId;
    pub(crate) mMediaId: i32,
    // java public String mVbaClsid;
    pub(crate) mVbaClsid: Option<String>,
    // java public boolean mIsVba;
    pub(crate) mIsVba: bool,
    // java public String mPartName;
    pub(crate) mPartName: Option<String>,
    // java public String mPersistence;
    pub(crate) mPersistence: Option<String>,
    // java public boolean mIsIcon;
    pub(crate) mIsIcon: bool,
}

// java public abstract class Shapes2Core
impl Shapes2Core {
    // java public  Shapes2Core(SubDocument document, boolean isVml)
    pub fn Shapes2Core(document: &SubDocument, isVml: bool) -> Self {
      // java IOAssert.assertNotNull("document should not be null", document);
      IOAssert::assertNotNull__string_object(&"document should not be null", Some(&document));
      // java this.mDocument = document;
      let mut this = Self { mDocument: document, mShape: Default::default(), mRunProps: Default::default(), mIsVml: isVml, mHasOle: Default::default() };
      // java this.mIsVml = isVml;
      this
    }

    // java public void append(ArrayList<Shape> mShapes, MutablePropertySet rPrSet)
    pub fn append__arraylistshape_mutablepropertyset(&mut self, mShapes: &Vec<Shape>, rPrSet: &MutablePropertySet) {
      // java IOAssert.assertNotNull("mShapes should not be null", mShapes);
      IOAssert::assertNotNull__string_object(&"mShapes should not be null", Some(&mShapes));
      // java this.mRunProps = new MutablePropertySet(rPrSet);
      self.mRunProps = MutablePropertySet::new(rPrSet);
      // java this.adjustShapes(mShapes);
      self.adjustShapes(&mShapes);
      // java int size = mShapes.size();
      let mut size: i32 = (mShapes.len() as i32);
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java this.mShape = (Shape)mShapes.get(i);
        self.mShape = (mShapes.get(i)).downcast_ref::<Shape>().unwrap();
        // java this.mShape.setIsVml(this.mIsVml);
        self.mShape.setIsVml(self.mIsVml);
        // java this.appendSingleShape();
        self.appendSingleShape();
      }
    }

    // java public void append(Shape shape, MutablePropertySet rPrSet)
    pub fn append__shape_mutablepropertyset(&mut self, shape: &Shape, rPrSet: &MutablePropertySet) {
      // java this.mRunProps = new MutablePropertySet(rPrSet);
      self.mRunProps = MutablePropertySet::new(rPrSet);
      // java this.mShape = shape;
      self.mShape = shape;
      // java this.appendSingleShape();
      self.appendSingleShape();
    }

    // java protected abstract void adjustShapes(ArrayList<Shape> var1)
    pub fn adjustShapes(&mut self, var1: &Vec<Shape>) {
      panic!("Shapes2Core::adjustShapes not implemented");
    }

    // java protected abstract int appendControlCharShape(char var1, SubDocument var2)
    pub fn appendControlCharShape__char_subdocument(&mut self, var1: char, var2: &SubDocument) -> i32 {
      panic!("Shapes2Core::appendControlCharShape not implemented");
    }

    // java protected abstract int appendText(SubDocument var1, String var2, MutablePropertySet var3)
    pub fn appendText(&mut self, var1: &SubDocument, var2: &str, var3: &MutablePropertySet) -> i32 {
      panic!("Shapes2Core::appendText not implemented");
    }

    // java protected void appendShape2CoreWithoutField()
    pub fn appendShape2CoreWithoutField(&mut self) {
      // java IOAssert.assertNotNull("mDocument should not be null", this.mDocument);
      IOAssert::assertNotNull__string_object(&"mDocument should not be null", Some(&self.mDocument));
      // java IOAssert.assertNotNull("mShape should not be null", this.mShape);
      IOAssert::assertNotNull__string_object(&"mShape should not be null", Some(&self.mShape));
      // java IOHelper.addTopLevelShape(this.mDocument, this.appendControlCharShape(), this.mShape);
      IOHelper::addTopLevelShape(self.mDocument, self.appendControlCharShape(), self.mShape);
    }

    // java protected void appendSingleShape()
    pub fn appendSingleShape(&mut self) {
      // java IOAssert.assertNotNull("mDocument should not be null", this.mDocument);
      IOAssert::assertNotNull__string_object(&"mDocument should not be null", Some(&self.mDocument));
      // java IOAssert.assertNotNull("mShape should not be null", this.mShape);
      IOAssert::assertNotNull__string_object(&"mShape should not be null", Some(&self.mShape));
      // java if (!this.isNotAllowAddShape())
      if !self.isNotAllowAddShape() {
        // java IDiagram diagram = this.mShape.getDiagramDrawingML();
        let mut diagram: Box<dyn IDiagram> = self.mShape.getDiagramDrawingML();
        // java boolean hasNoObj = !this.hasOLE();
        let mut hasNoObj: bool = !self.hasOLE();
        // java if (null == diagram && null == this.mShape.getInkModel() && (!this.mShape.isInDggInfo() || !hasNoObj) && !this.mShape.isArtText() && (null == this.mShape.getPicture() || this.isPictureInField(this.mShape) || !hasNoObj) && !this.mShape.getHorzRule() && (this.mShape.getShapeType() != 75 || null != this.mShape.getTextObj()))
        if diagram.is_none() && self.mShape.getInkModel().is_none() && !self.mShape.isInDggInfo() || !hasNoObj && !self.mShape.isArtText() && self.mShape.getPicture().is_none() || self.isPictureInField(&self.mShape) || !hasNoObj && !self.mShape.getHorzRule() && self.mShape.getShapeType() != 75 || self.mShape.getTextObj().is_some() {
          // java this.appendShape2CoreWithField();
          self.appendShape2CoreWithField();
        }
        // java else
        else {
          // java this.appendShape2CoreWithoutField();
          self.appendShape2CoreWithoutField();
        }
      }
    }

    // java protected boolean hasOLE()
    pub fn hasOLE(&self) -> bool {
      // java return this.mHasOle;
      return self.mHasOle;
    }

    // java public void setHasOle(boolean hasOle)
    pub fn setHasOle(&mut self, hasOle: bool) {
      // java this.mHasOle = hasOle;
      self.mHasOle = hasOle;
    }

    // java private boolean isPictureInField(Shape shape)
    pub fn isPictureInField(&self, shape: &Shape) -> bool {
      // java IPos pos = shape.getShapePos();
      let mut pos: Box<dyn IPos> = shape.getShapePos();
      // java if (null == pos)
      if pos.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java float rot = pos.getRotation();
        let mut rot: f32 = pos.getRotation();
        // java return 0.0F != rot % 90.0F;
        return 0 != rot % 90;
      }
    }

    // java protected void appendInlineShapeBegin()
    pub fn appendInlineShapeBegin(&mut self) {
      // java this.appendFieldBegin(KFieldType.FieldShape, this.mDocument);
      self.appendFieldBegin(&KFieldType::FieldShape, &self.mDocument);
      // java this.appendText(this.mDocument, " " + KFieldType.FieldShape.toString() + " \\* MERGEFORMAT ", this.mRunProps);
      self.appendText(&self.mDocument, &" " + KFieldType::FieldShape.toString() + " \\* MERGEFORMAT ", &self.mRunProps);
      // java this.appendFieldSeparator(false, this.mDocument);
      self.appendFieldSeparator__boolean_subdocument(false, &self.mDocument);
    }

    // java protected void appendInlineShapeEnd(SubDocument doc)
    pub fn appendInlineShapeEnd(&mut self, doc: &SubDocument) {
      // java this.appendFieldEnd(doc);
      self.appendFieldEnd(&doc);
    }

    // java protected void appendShape2CoreWithField()
    pub fn appendShape2CoreWithField(&mut self) {
      // java this.appendInlineShapeBegin();
      self.appendInlineShapeBegin();
      // java IPos pos = this.mShape.getShapePos();
      let mut pos: Box<dyn IPos> = self.mShape.getShapePos();
      // java IOAssert.assertNotNull("pos should not be null!", pos);
      IOAssert::assertNotNull__string_object(&"pos should not be null!", Some(&pos));
      // java pos.setRelativeH(2);
      pos.setRelativeH(2);
      // java pos.setPosRelH(2);
      pos.setPosRelH(2);
      // java pos.setPosRelV(2);
      pos.setPosRelV(2);
      // java pos.setRelativeV(2);
      pos.setRelativeV(2);
      // java pos.setLayoutInCell(true);
      pos.setLayoutInCell(true);
      // java this.mShape.setUseShapeAnchor(true);
      self.mShape.setUseShapeAnchor(true);
      // java this.mShape.setInDggInfo(true);
      self.mShape.setInDggInfo(true);
      // java Protection protection = this.mShape.getProtection();
      let mut protection: Protection = self.mShape.getProtection();
      // java if (protection == null)
      if protection.is_none() {
        // java protection = new Protection();
        protection = Protection::new();
        // java this.mShape.setProtection(protection);
        self.mShape.setProtection(Some(&mut protection));
      }
      // java protection.setLockAgainstGrouping(false);
      protection.setLockAgainstGrouping(false);
      // java protection.setLockText(false);
      protection.setLockText(false);
      // java protection.setLockVertices(false);
      protection.setLockVertices(false);
      // java protection.setLockAspectRatio(false);
      protection.setLockAspectRatio(false);
      // java protection.setLockRotation(false);
      protection.setLockRotation(false);
      // java protection.setLockPosition(false);
      protection.setLockPosition(false);
      // java IOHelper.addTopLevelShape(this.mDocument, this.appendControlCharShape('\b', this.mDocument), this.mShape);
      IOHelper::addTopLevelShape(self.mDocument, self.appendControlCharShape__char_subdocument('\u{0008}', &self.mDocument), self.mShape);
      // java int shapeType = 75;
      let mut shapeType: i32 = 75;
      // java IOHelper.addTopLevelShape(this.mDocument, this.appendControlCharShape('\u0001', this.mDocument), this.createPictureFrame(this.mShape, 75));
      IOHelper::addTopLevelShape(self.mDocument, self.appendControlCharShape__char_subdocument('\u{0001}', &self.mDocument), self.createPictureFrame(&mut self.mShape, 75));
      // java this.appendInlineShapeEnd(this.mDocument);
      self.appendInlineShapeEnd(&self.mDocument);
    }

    // java protected void appendFieldBegin(KFieldType fieldType, SubDocument doc)
    pub fn appendFieldBegin(&mut self, fieldType: &KFieldType, doc: &SubDocument) {
      // java IOAssert.assertNotNull("fieldType should not be null!", fieldType);
      IOAssert::assertNotNull__string_object(&"fieldType should not be null!", Some(&fieldType));
      // java char fieldBegin = 19;
      let mut fieldBegin: char = 19;
      // java PLCField.FieldNode fieldNodeBegin = this.appendToPLCField('\u0013', this.appendControlCharField('\u0013', true, doc), doc);
      let mut fieldNodeBegin: PLCField::FieldNode = self.appendToPLCField('\u{0013}', self.appendControlCharField__char_boolean_subdocument('\u{0013}', true, &doc), &doc);
      // java IOAssert.assertNotNull("fieldNodeBegin should not be null!", fieldNodeBegin);
      IOAssert::assertNotNull__string_object(&"fieldNodeBegin should not be null!", Some(&fieldNodeBegin));
      // java fieldNodeBegin.mData = fieldType.getFlt();
      fieldNodeBegin.mData = fieldType.getFlt();
    }

    // java protected PLCField.FieldNode appendFieldSeparator(boolean isIgnoredField, MutablePropertySet prop, SubDocument doc)
    pub fn appendFieldSeparator__boolean_mutablepropertyset_subdocument(&mut self, isIgnoredField: bool, prop: &MutablePropertySet, doc: &SubDocument) -> PLCField::FieldNode {
      // java char fieldSeparator = 20;
      let mut fieldSeparator: char = 20;
      // java PLCField.FieldNode fieldNodeSeparator = this.appendToPLCField('\u0014', this.appendControlCharField('\u0014', isIgnoredField, prop, doc), doc);
      let mut fieldNodeSeparator: PLCField::FieldNode = self.appendToPLCField('\u{0014}', self.appendControlCharField__char_boolean_mutablepropertyset_subdocument('\u{0014}', isIgnoredField, &mut prop, &doc), &doc);
      // java IOAssert.assertNotNull("fieldNodeSeparator should not be null!", fieldNodeSeparator);
      IOAssert::assertNotNull__string_object(&"fieldNodeSeparator should not be null!", Some(&fieldNodeSeparator));
      // java return fieldNodeSeparator;
      return fieldNodeSeparator;
    }

    // java protected void appendFieldSeparator(boolean isIgnoredField, SubDocument doc)
    pub fn appendFieldSeparator__boolean_subdocument(&mut self, isIgnoredField: bool, doc: &SubDocument) {
      // java MutablePropertySet dstRunProp = new MutablePropertySet(this.mRunProps);
      let mut dstRunProp: MutablePropertySet = MutablePropertySet::new(self.mRunProps);
      // java this.appendFieldSeparator(isIgnoredField, dstRunProp, doc);
      self.appendFieldSeparator__boolean_mutablepropertyset_subdocument(isIgnoredField, &dstRunProp, &doc);
    }

    // java private void appendFieldEnd(SubDocument doc)
    pub fn appendFieldEnd(&mut self, doc: &SubDocument) {
      // java char fieldEnd = 21;
      let mut fieldEnd: char = 21;
      // java PLCField.FieldNode fieldNodeEnd = this.appendToPLCField('\u0015', this.appendControlCharField('\u0015', false, doc), doc);
      let mut fieldNodeEnd: PLCField::FieldNode = self.appendToPLCField('\u{0015}', self.appendControlCharField__char_boolean_subdocument('\u{0015}', false, &doc), &doc);
      // java IOAssert.assertNotNull("fieldNodeEnd should not be null!", fieldNodeEnd);
      IOAssert::assertNotNull__string_object(&"fieldNodeEnd should not be null!", Some(&fieldNodeEnd));
    }

    // java protected int appendControlCharShape()
    pub fn appendControlCharShape(&mut self) -> i32 {
      // java return this.appendControlCharShape(this.getControlChar(), this.mDocument);
      return self.appendControlCharShape__char_subdocument(self.getControlChar(), &self.mDocument);
    }

    // java private char getControlChar()
    pub fn getControlChar(&self) -> char {
      // java IOAssert.assertNotNull("mShape should not be null!", this.mShape);
      IOAssert::assertNotNull__string_object(&"mShape should not be null!", Some(&self.mShape));
      // java return (char)(this.mShape.isInDggInfo() ? '\b' : '\u0001');
      return todo!("cast → char");
    }

    // java public Shape createPictureFrame(Shape sp, int shapeType)
    pub fn createPictureFrame(&mut self, sp: &mut Shape, shapeType: i32) -> Shape {
      // java Shape shape = IOHelper.createShape(this.mDocument.getDrawingContainer(), false);
      let mut shape: Shape = IOHelper::createShape(self.mDocument.getDrawingContainer(), false);
      // java IChart chart = sp.getDrawingContainer().getChart(sp);
      let mut chart: Box<dyn IChart> = sp.getDrawingContainer().getChart(sp);
      // java if (null != chart)
      if chart.is_some() {
        // java shape.getDrawingContainer().addChartShape(shape, sp);
        shape.getDrawingContainer().addChartShape(shape, sp);
        // java sp.setProperty(887, shape);
        sp.setProperty(887, &shape);
        // java this.addShape4Chart(shape);
        self.addShape4Chart(&shape);
      }
      // java shape.setShapeType(shapeType);
      shape.setShapeType(shapeType);
      // java shape.setShapeID(ShapeIDTool.getNewID(this.mDocument.getDrawingContainer()));
      shape.setShapeID(ShapeIDTool::getNewID__idrawingcontainer(Some(&mut self.mDocument.getDrawingContainer())));
      // java shape.setInDggInfo(false);
      shape.setInDggInfo(false);
      // java shape.setInlineShape(true);
      shape.setInlineShape(true);
      // java shape.setUseShapeAnchor(true);
      shape.setUseShapeAnchor(true);
      // java shape.setBehindDocument(sp.getBehindDocument());
      shape.setBehindDocument(sp.getBehindDocument());
      // java IPos shapePos = sp.getShapePos();
      let mut shapePos: Box<dyn IPos> = sp.getShapePos();
      // java IPos newShapePos = shape.getShapePos();
      let mut newShapePos: Box<dyn IPos> = shape.getShapePos();
      // java if (newShapePos == null)
      if newShapePos.is_none() {
        // java newShapePos = new ShapePos();
        newShapePos = ShapePos::new();
        // java shape.setShapePos(newShapePos);
        shape.setShapePos(newShapePos);
      }
      // java if (null != shapePos)
      if shapePos.is_some() {
        // java newShapePos.setShapeRect(shapePos.getShapeRect());
        newShapePos.setShapeRect(shapePos.getShapeRect());
        // java newShapePos.setRotation(shapePos.getRotation());
        newShapePos.setRotation(shapePos.getRotation());
      }
      // java newShapePos.setLayoutInCell(true);
      newShapePos.setLayoutInCell(true);
      // java GRF grf = sp.getGRF();
      let mut grf: GRF = sp.getGRF();
      // java GRF newGRF = shape.getGRF();
      let mut newGRF: GRF = shape.getGRF();
      // java if (newGRF == null)
      if newGRF.is_none() {
        // java newGRF = new GRF();
        newGRF = GRF::new();
        // java shape.setGRF(newGRF);
        shape.setGRF(newGRF);
      }
      // java newGRF.setBackground(grf.isBackground());
      newGRF.setBackground(grf.isBackground());
      // java newGRF.setChild(false);
      newGRF.setChild(false);
      // java newGRF.setHaveAnchor(grf.isHaveAnchor());
      newGRF.setHaveAnchor(grf.isHaveAnchor());
      // java newGRF.setHaveSpt(true);
      newGRF.setHaveSpt(true);
      // java newGRF.setGroup(false);
      newGRF.setGroup(false);
      // java Protection protection = shape.getProtection();
      let mut protection: Protection = shape.getProtection();
      // java if (protection == null)
      if protection.is_none() {
        // java protection = new Protection();
        protection = Protection::new();
        // java shape.setProtection(protection);
        shape.setProtection(protection);
      }
      // java protection.setLockAgainstGrouping(false);
      protection.setLockAgainstGrouping(false);
      // java protection.setLockText(false);
      protection.setLockText(false);
      // java protection.setLockVertices(false);
      protection.setLockVertices(false);
      // java protection.setLockAspectRatio(false);
      protection.setLockAspectRatio(false);
      // java protection.setLockRotation(false);
      protection.setLockRotation(false);
      // java protection.setLockPosition(true);
      protection.setLockPosition(true);
      // java return shape;
      return shape;
    }

    // java private int appendControlCharField(char fieldChar, boolean isIgnoredField, SubDocument doc)
    pub fn appendControlCharField__char_boolean_subdocument(&mut self, fieldChar: char, isIgnoredField: bool, doc: &SubDocument) -> i32 {
      // java IOAssert.assertNotNull("mRunProps should not be null!", this.mRunProps);
      IOAssert::assertNotNull__string_object(&"mRunProps should not be null!", Some(&self.mRunProps));
      // java MutablePropertySet dstRunProp = new MutablePropertySet(this.mRunProps);
      let mut dstRunProp: MutablePropertySet = MutablePropertySet::new(self.mRunProps);
      // java return this.appendControlCharField(fieldChar, isIgnoredField, dstRunProp, doc);
      return self.appendControlCharField__char_boolean_mutablepropertyset_subdocument(fieldChar, isIgnoredField, &mut dstRunProp, &doc);
    }

    // java private int appendControlCharField(char fieldChar, boolean isIgnoredField, MutablePropertySet prop, SubDocument doc)
    pub fn appendControlCharField__char_boolean_mutablepropertyset_subdocument(&mut self, fieldChar: char, isIgnoredField: bool, prop: &mut MutablePropertySet, doc: &SubDocument) -> i32 {
      // java prop.setBoolean(28, true);
      prop.setBoolean(28, true);
      // java if (isIgnoredField)
      if isIgnoredField {
        // java prop.setBoolean(758, isIgnoredField);
        prop.setBoolean(758, isIgnoredField);
      }
      // java return this.appendText(doc, "" + fieldChar, prop);
      return self.appendText(&doc, &"" + fieldChar, &prop);
    }

    // java private PLCField.FieldNode appendToPLCField(char fieldChar, int cpFieldChar, SubDocument doc)
    pub fn appendToPLCField(&mut self, fieldChar: char, cpFieldChar: i32, doc: &SubDocument) -> PLCField::FieldNode {
      // java IOAssert.assertTrue("(ControlChar.FIELD_BEGIN == fieldChar ||ControlChar.FIELD_SEPARATOR == fieldChar ||ControlChar.FIELD_END == fieldChar) should not be false!", 19 == fieldChar || 20 == fieldChar || 21 == fieldChar);
      IOAssert::assertTrue__string_boolean(&"(ControlChar.FIELD_BEGIN == fieldChar ||ControlChar.FIELD_SEPARATOR == fieldChar ||ControlChar.FIELD_END == fieldChar) should not be false!", 19 == fieldChar || 20 == fieldChar || 21 == fieldChar);
      // java IOAssert.assertTrue("cpFieldChar >= 0 should not be false!", cpFieldChar >= 0);
      IOAssert::assertTrue__string_boolean(&"cpFieldChar >= 0 should not be false!", cpFieldChar >= 0);
      // java PLCField plcField = doc.getPLCField();
      let mut plcField: PLCField = doc.getPLCField();
      // java IOAssert.assertNotNull("plcField should not be null!", plcField);
      IOAssert::assertNotNull__string_object(&"plcField should not be null!", Some(&plcField));
      // java PLCField.FieldNode fieldNode = plcField.addFieldChar(cpFieldChar);
      let mut fieldNode: PLCField::FieldNode = plcField.addFieldChar(cpFieldChar);
      // java IOAssert.assertNotNull("fieldNode should not be null!", fieldNode);
      IOAssert::assertNotNull__string_object(&"fieldNode should not be null!", Some(&fieldNode));
      // java fieldNode.setFieldChar(fieldChar);
      fieldNode.setFieldChar(fieldChar);
      // java if (19 != fieldChar && 21 == fieldChar)
      if 19 != fieldChar && 21 == fieldChar {
        // java fieldNode.setFieldFlags(this.fieldFlags(fieldNode));
        fieldNode.setFieldFlags(self.fieldFlags(&fieldNode));
      }
      // java return fieldNode;
      return fieldNode;
    }

    // java private int fieldFlags(PLCField.FieldNode fieldEnd)
    pub fn fieldFlags(&mut self, fieldEnd: &PLCField::FieldNode) -> i32 {
      // java IOAssert.assertNotNull("fieldEnd should not be nul!", fieldEnd);
      IOAssert::assertNotNull__string_object(&"fieldEnd should not be nul!", Some(&fieldEnd));
      // java int flags = 0;
      let mut flags: i32 = 0;
      // java flags = BitsOperation.resetBits(flags, 2);
      flags = BitsOperation::resetBits(flags, 2);
      // java flags = BitsOperation.resetBits(flags, 16);
      flags = BitsOperation::resetBits(flags, 16);
      // java PLCField.FieldNode fieldSeparator = (PLCField.FieldNode)fieldEnd.getPrev();
      let mut fieldSeparator: PLCField::FieldNode = (fieldEnd.getPrev()).downcast_ref::<PLCField::FieldNode>().unwrap();
      // java if (fieldSeparator.isFieldSeperator())
      if fieldSeparator.isFieldSeperator() {
        // java flags = BitsOperation.setBits(flags, 128);
        flags = BitsOperation::setBits(flags, 128);
      }
      // java else
      else {
        // java flags = BitsOperation.resetBits(flags, 128);
        flags = BitsOperation::resetBits(flags, 128);
      }
      // java return flags;
      return flags;
    }

    // java private boolean isNotAllowAddShape()
    pub fn isNotAllowAddShape(&self) -> bool {
      // java int docType = this.mDocument.getType();
      let mut docType: i32 = self.mDocument.getType();
      // java if (5 != docType && 6 != docType)
      if 5 != docType && 6 != docType {
        // java return false;
        return false;
      }
      // java else
      else {
        // java int shapeType = this.mShape.getShapeType();
        let mut shapeType: i32 = self.mShape.getShapeType();
        // java switch (shapeType)
        match shapeType {
          // java case 204:
          204 => {
            // java return false;
            return false;
          }
          // java default:
          _ => {
            // java return !this.mShape.isArtText();
            return !self.mShape.isArtText();
          }
        }
      }
    }

    // java private void addShape4Chart(Shape shape)
    pub fn addShape4Chart(&self, shape: &Shape) {
      // java TextDocument textDocument = this.mDocument.getTextDocument();
      let mut textDocument: TextDocument = self.mDocument.getTextDocument();
      // java List<Shape> chartShapes = textDocument.getChartShapes();
      let mut chartShapes: Vec<Shape> = textDocument.getChartShapes();
      // java if (null == chartShapes)
      if chartShapes.is_none() {
        // java chartShapes = new ArrayList();
        chartShapes = Vec::new();
        // java textDocument.setChartShapes(chartShapes);
        textDocument.setChartShapes(chartShapes);
      }
      // java chartShapes.add(shape);
      chartShapes.add(shape);
    }

    // java public void setShape2ShapeID(HashMap<Shape, String> mShape2ShapeID)
    pub fn setShape2ShapeID(&mut self, mShape2ShapeID: &HashMap<Shape, String>) {
    }

    // java public void setShapeID2OLEObjectInfo(HashMap<String, OLEObjectInfo> mShapeID2OLEObjectInfo)
    pub fn setShapeID2OLEObjectInfo(&mut self, mShapeID2OLEObjectInfo: &HashMap<String, OLEObjectInfo>) {
    }

}

// java public static final class OLEObjectInfo
impl OLEObjectInfo {
}
