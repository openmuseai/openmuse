// Java: import cn.wps.moffice.drawing.GRF
use drawing::cn::wps::moffice::drawing::GRF::*;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::*;
// Java: import cn.wps.moffice.drawing.chart.IChart
use drawing::cn::wps::moffice::drawing::chart::IChart::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.helper.CoreHelper
use crate::cn::wps::moffice::writer::helper::CoreHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.helper.IOHelper
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.helper.Shapes2Core
use crate::cn::wps::moffice::writer::io::reader::helper::Shapes2Core::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import org.apache.poi.hwpf.ole.DocxOleInfo
use io_poi_compact::org::apache::poi::hwpf::ole::DocxOleInfo::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use std::collections::{HashMap};

// java private static String TAG = null;
static TAG: std::sync::LazyLock<Option<String>> =
                    std::sync::LazyLock::new(|| None);

// java public final class Shapes2Core4Docx extends Shapes2Core
#[derive(Default)]
pub struct Shapes2Core4Docx {
    // java Shapes2Core super_
    pub(crate) super_: Shapes2Core,
    // java private HashMap<String, Shapes2Core.OLEObjectInfo> mShapeID2OLEObjectInfo;
    mShapeID2OLEObjectInfo: Option<HashMap<String, Shapes2Core::OLEObjectInfo>>,
    // java private HashMap<Shape, String> mShape2ShapeID;
    mShape2ShapeID: Option<HashMap<Shape, String>>,
    // java private Xml07Context mXml07Cxt;
    mXml07Cxt: Option<Xml07Context>,
}

// java public final class Shapes2Core4Docx extends Shapes2Core
impl Shapes2Core4Docx {
    // java public  Shapes2Core4Docx(SubDocument document, boolean isVml, Xml07Context xml07Context)
    pub fn Shapes2Core4Docx(document: &SubDocument, isVml: bool, xml07Context: &Xml07Context) -> Self {
      // java super(document, isVml);
      // java this.mXml07Cxt = xml07Context;
      Self { super_: Shapes2Core::new(&document, isVml), mShapeID2OLEObjectInfo: Default::default(), mShape2ShapeID: Default::default(), mXml07Cxt: xml07Context }
    }

    // java public void setShape2ShapeID(HashMap<Shape, String> shape2ShapeID)
    pub fn setShape2ShapeID(&mut self, shape2ShapeID: &HashMap<Shape, String>) {
      // java this.mShape2ShapeID = shape2ShapeID;
      self.mShape2ShapeID = shape2ShapeID;
    }

    // java public void setShapeID2OLEObjectInfo(HashMap<String, Shapes2Core.OLEObjectInfo> shapeID2OLEObjectInfo)
    pub fn setShapeID2OLEObjectInfo(&mut self, shapeID2OLEObjectInfo: Option<&HashMap<String, Shapes2Core::OLEObjectInfo>>) {
      // java this.mShapeID2OLEObjectInfo = shapeID2OLEObjectInfo;
      self.mShapeID2OLEObjectInfo = shapeID2OLEObjectInfo;
      // java if (shapeID2OLEObjectInfo != null && shapeID2OLEObjectInfo.size() >= 1)
      if let Some(shapeID2OLEObjectInfo) = shapeID2OLEObjectInfo {
        if (shapeID2OLEObjectInfo.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) >= 1 {
          // java for(String key : shapeID2OLEObjectInfo.keySet())
          for key in shapeID2OLEObjectInfo.keySet().iter() {
            // java Shapes2Core.OLEObjectInfo oleObjectInfo = (Shapes2Core.OLEObjectInfo)shapeID2OLEObjectInfo.get(key);
            let mut oleObjectInfo: Shapes2Core::OLEObjectInfo = (shapeID2OLEObjectInfo.get(key)).downcast_ref::<Shapes2Core::OLEObjectInfo>().unwrap();
            // java DocxOleInfo oleInfo = new DocxOleInfo(oleObjectInfo.mPartName);
            let mut oleInfo: DocxOleInfo = DocxOleInfo::new(oleObjectInfo.mPartName);
            // java this.mDocument.getTextDocument().addOle(oleObjectInfo.mMediaId, oleInfo);
            self.mDocument.getTextDocument().addOle(oleObjectInfo.mMediaId, oleInfo);
          }
        }
      }
    }

    // java protected boolean hasOLE()
    pub fn hasOLE(&self) -> Option<bool> {
      // java return null != this.mShapeID2OLEObjectInfo && this.mShapeID2OLEObjectInfo.size() > 0;
      return self.mShapeID2OLEObjectInfo.is_some() && (self.mShapeID2OLEObjectInfo.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0;
    }

    // java protected int appendControlCharShape(char controlChar, SubDocument doc)
    pub fn appendControlCharShape(&self, controlChar: char, doc: &SubDocument) -> i32 {
      // java return IOHelper.appendControlChar(doc, controlChar, new MutablePropertySet(this.mRunProps));
      return IOHelper::appendControlChar(&doc, controlChar, &mut MutablePropertySet::new(self.mRunProps));
    }

    // java protected int appendText(SubDocument document, String text, MutablePropertySet props)
    pub fn appendText(&mut self, document: &SubDocument, text: &str, props: &mut MutablePropertySet) -> i32 {
      // java return CoreHelper.appendText(document, text, props.freeze());
      return CoreHelper::appendText__subdocument_char_propertyset(&document, text, &props.freeze());
    }

    // java protected void adjustShapes(ArrayList<Shape> shapes)
    pub fn adjustShapes(&mut self, shapes: &Vec<Shape>) {
    }

    // java protected void appendShape2CoreWithField()
    pub fn appendShape2CoreWithField(&mut self) {
      // java Object chart = this.mShape.getDrawingContainer().getChart(this.mShape);
      let mut chart: Box<dyn Object> = self.mShape.getDrawingContainer().getChart(self.mShape);
      // java Shapes2Core.OLEObjectInfo info = null;
      let mut info: Shapes2Core::OLEObjectInfo = Shapes2Core::OLEObjectInfo::default();
      // java if (null != this.mShapeID2OLEObjectInfo)
      if let Some(mShapeID2OLEObjectInfo) = self.mShapeID2OLEObjectInfo.as_mut() {
        // java String shapeID = (String)this.mShape2ShapeID.get(this.mShape);
        let mut shapeID: String = self.mShape2ShapeID.get(self.mShape).to_string();
        // java if (null != shapeID)
        if shapeID.is_some() {
          // java info = (Shapes2Core.OLEObjectInfo)this.mShapeID2OLEObjectInfo.get(shapeID);
          info = (mShapeID2OLEObjectInfo.get(shapeID)).downcast_ref::<Shapes2Core::OLEObjectInfo>().unwrap();
        }
      }
      // java if (null != chart)
      if chart.is_some() {
        // java this.appendFieldBegin(KFieldType.FieldEmbed, this.mDocument);
        self.super_.appendFieldBegin(&KFieldType::FieldEmbed, &self.mDocument);
        // java String progId = "Excel.Chart.8";
        let mut progId: String = "Excel.Chart.8".to_string();
        // java this.appendText(this.mDocument, " " + KFieldType.FieldEmbed.toString() + " " + progId + " \\s ", this.mRunProps);
        self.appendText(&self.mDocument, &" " + KFieldType::FieldEmbed.toString() + " " + progId + " \\s ", &mut self.mRunProps);
        // java IOHelper.createOLeShape(this.mShape, (IChart)chart, this.mDocument, progId);
        IOHelper::createOLeShape__shape_ichart_subdocument_string(&mut self.mShape, &mut todo!("cast → Box<dyn IChart>"), &self.mDocument, &progId);
        // java MutablePropertySet sepProp = this.prepareSepProp(this.mShape, this.mRunProps);
        let mut sepProp: MutablePropertySet = self.prepareSepProp(&self.mShape, &self.mRunProps);
        // java PLCField.FieldNode fieldNode = this.appendFieldSeparator(false, sepProp, this.mDocument);
        let mut fieldNode: PLCField::FieldNode = self.super_.appendFieldSeparator__boolean_mutablepropertyset_subdocument(false, &sepProp, &self.mDocument);
        // java fieldNode.mData = KFieldType.FieldRefOmitted.getFlt();
        fieldNode.mData = KFieldType::FieldRefOmitted.getFlt();
        // java this.mShape.setInlineShape(true);
        self.mShape.setInlineShape(true);
        // java this.mShape.getShapePos().setTextWrap(3);
        self.mShape.getShapePos().setTextWrap(3);
        // java int shapeType = 204;
        let mut shapeType: i32 = 204;
        // java IOHelper.addTopLevelShape(this.mDocument, this.appendControlCharShape('\u0001', this.mDocument), this.createPictureFrame(this.mShape, 204));
        IOHelper::addTopLevelShape(&self.mDocument, self.appendControlCharShape('\u{0001}', &self.mDocument), &self.super_.createPictureFrame(&mut self.mShape, 204));
        // java this.appendInlineShapeEnd(this.mDocument);
        self.super_.appendInlineShapeEnd(&self.mDocument);
      }
      // java else if (null != info && null != info.mProgID)
      else if info.is_some() && info.mProgID.is_some() {
        // java this.appendOLE2CoreWithField(info);
        self.appendOLE2CoreWithField(&info);
      }
      // java else
      else {
        // java super.appendShape2CoreWithField();
        self.super_.appendShape2CoreWithField();
      }
    }

    // java private void appendOLE2CoreWithField(Shapes2Core.OLEObjectInfo info)
    pub fn appendOLE2CoreWithField(&mut self, info: &Shapes2Core::OLEObjectInfo) {
      // java if (this.mShape.isInDggInfo())
      if self.mShape.isInDggInfo() {
        // java this.appendAnchorOLE2CoreWithField(info);
        self.appendAnchorOLE2CoreWithField(&info);
      }
      // java else
      else {
        // java this.appendInlineOLE2CoreWithField(info, this.mDocument, this.mRunProps, this.mShape, true);
        self.appendInlineOLE2CoreWithField(&info, &self.mDocument, &self.mRunProps, &self.mShape, true);
      }
    }

    // java private void appendAnchorOLE2CoreWithField(Shapes2Core.OLEObjectInfo info)
    pub fn appendAnchorOLE2CoreWithField(&mut self, info: &Shapes2Core::OLEObjectInfo) {
      // java GRF grf = this.mShape.getGRF();
      let mut grf: GRF = self.mShape.getGRF();
      // java grf.setOleShape(true);
      grf.setOleShape(true);
      // java IOHelper.addTopLevelShape(this.mDocument, this.appendControlCharShape(), this.mShape);
      IOHelper::addTopLevelShape(&self.mDocument, self.appendControlCharShape(), &self.mShape);
      // java SubDocument textboxDocument = (SubDocument)IOHelper.getTextboxDocument(this.mDocument);
      let mut textboxDocument: SubDocument = (IOHelper::getTextboxDocument(&self.mDocument)).downcast_ref::<SubDocument>().unwrap();
      // java IOHelper.addTextboxTextNode(textboxDocument, this.mShape);
      IOHelper::addTextboxTextNode(&textboxDocument, &self.mShape);
      // java Shape picShape = IOHelper.createPicShape(this.mShape, textboxDocument);
      let mut picShape: Shape = IOHelper::createPicShape(&self.mShape, &textboxDocument);
      // java int cp = textboxDocument.getLength();
      let mut cp: i32 = textboxDocument.getLength();
      // java this.appendInlineOLE2CoreWithField(info, textboxDocument, this.mRunProps, picShape, false);
      self.appendInlineOLE2CoreWithField(&info, &textboxDocument, &self.mRunProps, &picShape, false);
      // java CoreHelper.appendParagraph(textboxDocument, '\r', cp, PropertySet.empty, PropertySet.empty);
      CoreHelper::appendParagraph__subdocument_char_int_propertyset_propertyset(&textboxDocument, '\r', cp, &PropertySet::empty(), &PropertySet::empty());
      // java IOHelper.addTextboxSeparator(textboxDocument);
      IOHelper::addTextboxSeparator(&textboxDocument);
    }

    // java private void appendInlineOLE2CoreWithField(Shapes2Core.OLEObjectInfo info, SubDocument doc, MutablePropertySet prop, Shape picShape, boolean addText)
    pub fn appendInlineOLE2CoreWithField(&mut self, info: &Shapes2Core::OLEObjectInfo, doc: &SubDocument, prop: &MutablePropertySet, picShape: &Shape, addText: bool) {
      // java this.appendFieldBegin(KFieldType.FieldEmbed, doc);
      self.super_.appendFieldBegin(&KFieldType::FieldEmbed, &doc);
      // java if (info.mIsVba)
      if info.mIsVba {
        // java if (addText)
        if addText {
          // java this.appendText(this.mDocument, " " + KFieldType.FieldControl.toString() + " " + info.mProgID + " \\s ", this.mRunProps);
          self.appendText(&self.mDocument, &" " + KFieldType::FieldControl.toString() + " " + info.mProgID + " \\s ", &mut self.mRunProps);
        }
      }
      // java else
      else {
        // java this.appendText(doc, " " + KFieldType.FieldEmbed.toString() + " " + info.mProgID + " ", prop);
        self.appendText(&doc, &" " + KFieldType::FieldEmbed.toString() + " " + info.mProgID + " ", &mut prop);
      }
      // java Shape shape = IOHelper.createOLeShape(info, doc, this.mXml07Cxt);
      let mut shape: Shape = IOHelper::createOLeShape__oleobjectinfo_subdocument_xml07context(&info, &doc, Some(&mut self.mXml07Cxt));
      // java this.appendInlineObj2CoreWithField(doc, shape, prop, picShape, info.mIsIcon);
      self.appendInlineObj2CoreWithField(&doc, &shape, &prop, &picShape, info.mIsIcon);
    }

    // java private void appendInlineObj2CoreWithField(SubDocument doc, Shape shape, MutablePropertySet prop, Shape picShape, boolean isIcon)
    pub fn appendInlineObj2CoreWithField(&mut self, doc: &SubDocument, shape: &Shape, prop: &MutablePropertySet, picShape: &Shape, isIcon: bool) {
      // java MutablePropertySet sepProp = this.prepareSepProp(shape, prop);
      let mut sepProp: MutablePropertySet = self.prepareSepProp(&shape, &prop);
      // java PLCField.FieldNode fieldNode = this.appendFieldSeparator(false, sepProp, doc);
      let mut fieldNode: PLCField::FieldNode = self.super_.appendFieldSeparator__boolean_mutablepropertyset_subdocument(false, &sepProp, &doc);
      // java if (!isIcon)
      if !isIcon {
        // java fieldNode.mData = KFieldType.FieldRefOmitted.getFlt();
        fieldNode.mData = KFieldType::FieldRefOmitted.getFlt();
      }
      // java if (picShape.getInlineShape())
      if picShape.getInlineShape() {
        // java this.mShape.setInlineShape(true);
        self.mShape.setInlineShape(true);
      }
      // java IOHelper.addTopLevelShape(doc, this.appendControlCharShape('\u0001', doc), picShape);
      IOHelper::addTopLevelShape(&doc, self.appendControlCharShape('\u{0001}', &doc), &picShape);
      // java this.appendInlineShapeEnd(doc);
      self.super_.appendInlineShapeEnd(&doc);
    }

    // java private MutablePropertySet prepareSepProp(Shape shape, MutablePropertySet srcRunProps)
    pub fn prepareSepProp(&mut self, shape: &Shape, srcRunProps: &MutablePropertySet) -> MutablePropertySet {
      // java MutablePropertySet prop = new MutablePropertySet(srcRunProps);
      let mut prop: MutablePropertySet = MutablePropertySet::new(srcRunProps);
      // java prop.setBoolean(28, true);
      prop.setBoolean(28, true);
      // java prop.setBoolean(30, true);
      prop.setBoolean(30, true);
      // java prop.setBoolean(29, true);
      prop.setBoolean(29, true);
      // java prop.setValue(56, shape);
      prop.setValue(56, shape);
      // java return prop;
      return prop;
    }

}

impl std::ops::Deref for Shapes2Core4Docx {
    type Target = Shapes2Core;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for Shapes2Core4Docx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
