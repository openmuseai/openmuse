// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::KStyle::TextDocument as WriterTextDocument;
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::ShapeFactory::ShapeFactory;
use drawing::cn::wps::moffice::drawing::IDrawingContainer::IDrawingContainer;
use drawing::cn::wps::moffice::drawing::graphics::BlipFill::BlipFill;
use drawing::cn::wps::moffice::drawing::graphics::PatternFill::PatternFill;
use drawing::cn::wps::moffice::drawing::graphics::Picture::Picture;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::BlipFillInfos::BlipFillInfos;
use std::sync::{Arc, Mutex};
use crate::cn::wps::moffice::writer::helper::CoreHelper::CoreHelper;
use io_wps_io::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use io_wps_io::io::xwpf::usermodel::XWPFPictureData::XWPFPictureData;
use io_wps_io::io::xwpf::usermodel::XWPFRelation::XWPFRelation;
use io_wps_io::io::xwpf::usermodel::XWPFRelationPurl::XWPFRelationPurl;
use util::util::CreatefileInfolder::CreatefileInfolder;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLCTextboxText::TextboxTextNode;
use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use util::moffice::property::PropertySet::PropertySet;
use util::base::io::{MetricUtility::MetricUtility, TypeConversion::TypeConversion};

// java public final class IOHelper
#[derive(Default)]
pub struct IOHelper {}

// java public final class IOHelper
impl IOHelper {
    // java public static void addNormalStyle(KStyles styles)
    pub fn addNormalStyle(styles: &mut KStyles) -> () {
      // java IOAssert.assertNotNull("styles should not be null", styles);
      let styles_opt = Some(&*styles);
      IOAssert::assertNotNull_msg("styles should not be null", &styles_opt);
      // java if (null == styles.getStyleById(0))
      if styles.getStyleById__int(0).is_none() {
        // java TextDocument textDocument = styles.getTextDocument();
        let mut textDocument: WriterTextDocument = styles.getTextDocument();
        // java IOAssert.assertNotNull("textDocument should not be null.", textDocument);
        let td_opt = Some(&textDocument);
        IOAssert::assertNotNull_msg("textDocument should not be null.", &td_opt);
        // java styles.add(normalStyle(textDocument));
        let mut style = Self::normalStyle(&mut textDocument);
        styles.add(&mut style);
      }
    }

    // java private static KStyle normalStyle(TextDocument textDocument)
    pub fn normalStyle(textDocument: &mut WriterTextDocument) -> KStyle {
      // java String displayName = "Normal";
      let displayName = "Normal";
      // java int type = 1;
      let r#type: i32 = 1;
      // java MutablePropertySet property = new MutablePropertySet();
      let mut property: MutablePropertySet = MutablePropertySet::new();
      // java property.setFloat(10, 10.5F);
      property.setFloat(10, 10.5);
      // java property.setInt(193, 3);
      property.setInt(193, 3);
      // java KStyle pStyle = new KStyle(textDocument, 0, displayName, 4095, 4095, type, property.freeze());
      let frozen = property.freeze();
      // java return pStyle;
      return KStyle::new__textdocument_int_string_int_int_int_propertyset(textDocument, 0, displayName, 4095, 4095, r#type, &frozen);
    }

    // --- layout coverage stubs (parked, not rewritten this function) ---
    // java public static boolean stringHasValue(String value)
    pub fn stringHasValue(value: Option<&str>) -> bool {
      // Java: return null != value && value.length() > 0;
      value.is_some_and(|value| !value.is_empty())
    }

    // java public static PictureData getPictureData(String rId, POIXMLDocumentPart parent)
    pub fn getPictureData(rId: Option<&str>, parent: &POIXMLDocumentPart) -> Option<XWPFPictureData> {
      // Java: IOAssert.assertNotNull("parent should not be null", parent);
      // Java: if (!stringHasValue(rId)) return null;
      if !Self::stringHasValue(rId) { return None; }
      // Java: POIXMLDocumentPart part = parent.getRelationById(rId);
      let part = parent.getRelationById(rId?)?;
      // Java: PictureData pictureData = (XWPFPictureData)part;
      // The Rust factory retains the subclass in its shared base relation entry.
      let is_picture = {
        let borrowed = part.borrow();
        let relation_type = borrowed.getPackageRelationship().as_ref()?.getRelationshipType();
        XWPFRelation::getInstance(relation_type).getRelationClass() == Some("XWPFPictureData")
          || XWPFRelationPurl::getInstance(relation_type).getRelationClass() == Some("XWPFPictureData")
      };
      assert!(is_picture, "picture relation is not XWPFPictureData");
      // Java: return pictureData;
      Some(XWPFPictureData::from_relation(part))
    }

    // java public static int appendControlChar(SubDocument document, char controlChar, MutablePropertySet props)
    pub fn appendControlChar(document: &mut SubDocument, controlChar: char, props: &mut MutablePropertySet) -> i32 {
      // Java: IOAssert.assertNotNull("document should not be null", document);
      // Java: IOAssert.assertNotNull("props should not be null", props);
      // Java: props.setBoolean(28, true);
      props.setBoolean(28, true);
      // Java: return CoreHelper.appendText(document, controlChar, props.freeze());
      let frozen = props.freeze();
      CoreHelper::appendText__char_ps(Some(document), controlChar, Some(&frozen))
    }

    // Java: public static void addTopLevelShape(SubDocument document, int cp, Shape shape)
    pub fn addTopLevelShape(document: &mut SubDocument, cp: i32, shape: Arc<Mutex<Shape>>) {
      // Java: IOAssert.assertNotNull("document should not be null", document);
      // Java: IOAssert.assertNotNull("shape should not be null", shape);
      // Java: PLCShapeAnchor pShapeAnchors = document.getShapeAnchors();
      let shape_id = shape.lock().expect("shape lock").getShapeID();
      // Java: IOAssert.assertNotNull("pShapeAnchors should not be null!", pShapeAnchors);
      let pShapeAnchors = document.mShapeAnchors.as_mut().expect("pShapeAnchors should not be null!");
      // Java: if (null != pShapeAnchors) pShapeAnchors.addShapeAnchor(cp, shape.getShapeID());
      pShapeAnchors.addShapeAnchor(cp, shape_id);
      // Java: IDrawingContainer drawingContainer = document.getDrawingContainer();
      let drawingContainer = document.getDrawingContainer();
      // Java: IOAssert.assertNotNull("drawingContainer should not be null", drawingContainer);
      let drawingContainer = drawingContainer.expect("drawingContainer should not be null");
      // Java: drawingContainer.addShape(shape);
      drawingContainer.addShape(Some(shape as Arc<dyn std::any::Any + Send + Sync>));
    }

    // java public static void endAllSubdocumentsDrawingContainer(TextDocument doc)
    pub fn endAllSubdocumentsDrawingContainer(doc: &mut TextDocument) {
      // java IOAssert.assertNotNull("doc should not be null.", doc);
      let doc_opt = Some(&*doc);
      IOAssert::assertNotNull_msg("doc should not be null.", &doc_opt);
      let mut i: i32 = 0;
      // java for(int i = 0; i < 7; ++i)
      while i < 7 {
        // java Document document = doc.createSubDocument(i);
        let _ = doc.createSubDocument(i);
        // java if (null != document)
        if let Some(document) = doc.getSubdocumentMut(i) {
          // java document.getDrawingContainer().endRead();
          if let Some(dc) = document.getDrawingContainer() {
            dc.endRead();
          }
        }
        i += 1;
      }
    }

    // java public static final String createPicturePath(String rootDir, String fileName)
    pub fn createPicturePath(rootDir: &str, fileName: &str) -> Option<String> {
      // Java: IOAssert.assertNotNull("fileName should not be null", fileName);
      // Java: Pair<File, Boolean> ret = CreatefileInfolder.createFile(rootDir, fileName);
      let ret = CreatefileInfolder::createFile(rootDir, fileName);
      // Java: if (null != ret) return ((File)ret.first).getAbsolutePath(); else return null;
      ret.map(|(file, _)| {
        // Java File.getAbsolutePath does not resolve symlinks (for example
        // macOS /var -> /private/var); preserve the path spelling.
        let absolute = if file.is_absolute() { file } else {
          std::env::current_dir().expect("current directory").join(file)
        };
        absolute.to_string_lossy().into_owned()
      })
    }

    // java public static Picture dumpPicture(Shape shape, PictureData pictureData, TextDocument doc)
    pub fn dumpPicture(shape: &Arc<Mutex<Shape>>, pictureData: &XWPFPictureData, doc: &mut TextDocument) -> Picture {
      // java Picture picture = new Picture();
      let picture = Picture::new();
      // java addBlipDocxInfo(shape, pictureData, doc, 1);
      Self::addBlipDocxInfo(shape, pictureData, doc, 1);
      // java return picture;
      picture
    }

    // java private static void addBlipDocxInfo(Shape shape, PictureData pictureData, TextDocument doc, int type)
    pub fn addBlipDocxInfo(shape: &Arc<Mutex<Shape>>, pictureData: &XWPFPictureData, doc: &mut TextDocument, r#type: i32) {
      // java BlipFillInfos.getInstance(); getBlipDocxInfo; add or addBlipFill; PictureDumpMap.add
      BlipFillInfos::register(shape, pictureData, doc, r#type);
    }

    // java public static BlipFill dumpPicture4BlipFill(Shape shape, PictureData pictureData, TextDocument doc, int type)
    pub fn dumpPicture4BlipFill(shape: &Arc<Mutex<Shape>>, pictureData: &XWPFPictureData, doc: &mut TextDocument, r#type: i32) -> BlipFill {
      // java BlipFill fill = new BlipFill(-1);
      let fill = BlipFill::new_with_media_id(-1);
      // java addBlipDocxInfo(shape, pictureData, doc, type);
      Self::addBlipDocxInfo(shape, pictureData, doc, r#type);
      // java return fill;
      fill
    }

    // java public static PatternFill dumpPicture4PatternFill(Shape shape, PictureData pictureData, TextDocument doc, int type)
    pub fn dumpPicture4PatternFill(shape: &Arc<Mutex<Shape>>, pictureData: &XWPFPictureData, doc: &mut TextDocument, r#type: i32) -> PatternFill {
      // java PatternFill fill = new PatternFill(-1);
      let fill = PatternFill::new_with_media_id(-1);
      // java addBlipDocxInfo(shape, pictureData, doc, type);
      Self::addBlipDocxInfo(shape, pictureData, doc, r#type);
      // java return fill;
      fill
    }

    // java public static Shape createShape(IDrawingContainer drawingContainer, boolean isGroupShape)
    pub fn createShape(drawingContainer: &mut dyn IDrawingContainer, isGroupShape: bool) -> Shape {
      // java return (Shape)(isGroupShape ? new GroupShape(drawingContainer) : ShapeFactory.getShapeFactory().createShape(drawingContainer));
      let handle = drawingContainer.containerElement();
      if isGroupShape {
        // Java: GroupShape(drawingContainer) calls super then setShapeType(0).
        let mut shape = Shape::Shape_new(handle);
        shape.setShapeType(0);
        shape
      } else {
        ShapeFactory::getShapeFactory().createShape(handle)
      }
    }

    pub fn createShape__idrawingcontainer_boolean(drawingContainer: &mut dyn IDrawingContainer, isGroupShape: bool) -> Shape {
      Self::createShape(drawingContainer, isGroupShape)
    }

    // java public static Float toFloat(PercentOrFractionUnit value)
    pub fn toFloat() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static float toOpacity(float transparency)
    pub fn toOpacity(transparency: f32) -> f32 {
      // java return 1.0F - transparency;
      1.0 - transparency
    }

    // java public static float toTransparency(float opacity)
    pub fn toTransparency(opacity: f32) -> f32 {
      // java return 1.0F - opacity;
      1.0 - opacity
    }

    // java public static int toCoreColor()
    pub fn toCoreColor() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static void setFlip(GRF grf, VmlCssStyle mCss)
    pub fn setFlip() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static String getPicturePath(Picture pic, SubDocument subDocument)
    pub fn getPicturePath() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static BRC toBRC(Border border)
    pub fn toBRC() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static boolean getBorderFrame(Boolean isFrame)
    pub fn getBorderFrame() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static boolean getBorderShadow(Boolean isShadow)
    pub fn getBorderShadow() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static float getSpace(Long space)
    pub fn getSpace() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static float getLineWidthBRC(Long lineWidth)
    pub fn getLineWidthBRC() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static int getBorderType(Border.BorderStyle borderStyle)
    pub fn getBorderType() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static int toCoreBorderType(Border.BorderStyle borderStyle)
    pub fn toCoreBorderType() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static float cssUnit2Pt(String strCssUnit)
    pub fn cssUnit2Pt() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static void appendHeaderFooterNodeSections(Document mainDocument)
    pub fn appendHeaderFooterNodeSections() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static PLCSection.SectionNode getSectionNodeByIndex(int index, PLCSection plcSection)
    pub fn getSectionNodeByIndex() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static void appendHeaderFooterNodeSection(int len, PLCSection.SectionNode sectionNode, PLCHeader plcHeader)
    pub fn appendHeaderFooterNodeSection() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static int cssUnitGd2Hundred(CssStyle.CssUnit cssUnit)
    pub fn cssUnitGd2Hundred() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static float cssUnitPercentage2Float(CssStyle.CssUnit cssUnit)
    pub fn cssUnitPercentage2Float() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static int getLengthDocument(Document document)
    pub fn getLengthDocument() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static int hexColor2Core(String hexColor)
    pub fn hexColor2Core() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static SubDocType getCurrentSubDocType(int docType)
    pub fn getCurrentSubDocType() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static String dumpPatternFill(byte[] bytes)
    pub fn dumpPatternFill(bytes: &[u8]) -> Option<String> {
        // java File target = Platform.createTempFile("patt", ".dib");
        let target = util_adapter::cn::wps::core::runtime::Platform::Platform::createTempFile("patt", ".dib")?;
        // java out.write(bytes); return target.getAbsolutePath(); IOException -> null
        match std::fs::write(&target, bytes) {
            Ok(()) => Some(target),
            Err(_) => None,
        }
    }

    // java public static int addAlphaToCoreColor(int rgb, float alpha)
    pub fn addAlphaToCoreColor(mut rgb: i32, alpha: f32) -> i32 {
      // java short _alpha = (short)((int)(255.0F * alpha));
      let _alpha = (255.0_f32 * alpha) as i32 as i16;
      // java rgb &= 16777215;
      rgb &= 0x00ff_ffff;
      // java return rgb | _alpha << 24;
      rgb | ((_alpha as i32) << 24)
    }

    // java public static Float PositiveFixedPercentage2Float(String value)
    pub fn PositiveFixedPercentage2Float(value: Option<&str>) -> Option<f32> {
      // java if (null == value) return null;
      let value = value?;
      // java else if (!checkPercentSymbol(value))
      if !Self::checkPercentSymbol(Some(value)) {
        // java Long longVal = TypeConversion.string2Long(value);
        let longVal = TypeConversion::string2Long(value)?;
        // java return null == longVal ? null : MetricUtility.STPercentageDecimal2Float(longVal);
        Some(MetricUtility::STPercentageDecimal2Float(longVal))
      } else {
        // java Float floatVal = TypeConversion.string2Float(value);
        let floatVal = TypeConversion::string2Float(value)?;
        // java return MetricUtility.Percent2Float(floatVal);
        Some(MetricUtility::Percent2Float(floatVal))
      }
    }

    // java public static Float PositivePercentage2Float(String value)
    pub fn PositivePercentage2Float(value: Option<&str>) -> Option<f32> {
      // java if (null == value) return null;
      let value = value?;
      // java if (!checkPercentSymbol(value)) ... else ...; return null;
      if !Self::checkPercentSymbol(Some(value)) {
        TypeConversion::string2Long(value).map(MetricUtility::STPercentageDecimal2Float)
      } else {
        TypeConversion::string2Float(value).map(MetricUtility::Percent2Float)
      }
    }

    // java public static Float FixedPercentage2Float(String value)
    pub fn FixedPercentage2Float(value: Option<&str>) -> Option<f32> {
      // java if (null == value) return null;
      let value = value?;
      // java if (!checkPercentSymbol(value)) ... else ...; return null;
      if !Self::checkPercentSymbol(Some(value)) {
        TypeConversion::string2Long(value).map(MetricUtility::STPercentageDecimal2Float)
      } else {
        TypeConversion::string2Float(value).map(MetricUtility::Percent2Float)
      }
    }

    // java public static Float Percentage2Float(String value)
    pub fn Percentage2Float(value: Option<&str>) -> Option<f32> {
      // java if (null == value) return null;
      let value = value?;
      // java if (!checkPercentSymbol(value)) ... else ...; return null;
      if !Self::checkPercentSymbol(Some(value)) {
        TypeConversion::string2Long(value).map(MetricUtility::STPercentageDecimal2Float)
      } else {
        TypeConversion::string2Float(value).map(MetricUtility::Percent2Float)
      }
    }

    // java public static Integer Angle2Integer(String value)
    pub fn Angle2Integer(value: Option<&str>) -> Option<i32> {
      // java return null == value ? null : TypeConversion.string2Integer(value);
      TypeConversion::string2Integer(value?)
    }

    // java public static Integer PositiveFixedAngle2Integer(String value)
    pub fn PositiveFixedAngle2Integer(value: Option<&str>) -> Option<i32> {
      // java Integer intValue = null == value ? null : TypeConversion.string2Integer(value);
      let intValue = TypeConversion::string2Integer(value?);
      // java return intValue;
      intValue
    }

    // java public static Integer fixedAngle2Integer(String value)
    pub fn fixedAngle2Integer(value: Option<&str>) -> Option<i32> {
      // java Integer intValue = null == value ? null : TypeConversion.string2Integer(value);
      let intValue = TypeConversion::string2Integer(value?);
      // java return intValue;
      intValue
    }

    // java private static boolean checkPercentSymbol(String value)
    pub fn checkPercentSymbol(value: Option<&str>) -> bool {
      // java if (null != value) { value = value.trim(); return value.endsWith("%"); } else return false;
      value.is_some_and(|value| value.trim().ends_with('%'))
    }

    // java public static Float fovAngle2Float(String value)
    pub fn fovAngle2Float(value: Option<&str>) -> Option<f32> {
      // java Integer intValue = null == value ? null : TypeConversion.string2Integer(value);
      let intValue = TypeConversion::string2Integer(value?)?;
      // java return null != intValue ? MetricUtility.ST_ANGLE_Degrees2Rad(intValue) : null;
      Some(MetricUtility::ST_ANGLE_Degrees2Rad(intValue))
    }

    // java public static Float positiveFixedAngle2Float(String value)
    pub fn positiveFixedAngle2Float(value: Option<&str>) -> Option<f32> {
      // java Integer intValue = null == value ? null : TypeConversion.string2Integer(value);
      let intValue = TypeConversion::string2Integer(value?)?;
      // java return null != intValue ? MetricUtility.ST_ANGLE_Degrees2Degree(intValue) : null;
      Some(MetricUtility::ST_ANGLE_Degrees2Degree(intValue))
    }

    // java public static Float colorTemperature2Float(String value)
    pub fn colorTemperature2Float(value: Option<&str>) -> Option<f32> {
      // java Integer intValue = null == value ? null : TypeConversion.string2Integer(value);
      let intValue = TypeConversion::string2Integer(value?)?;
      // java return null == intValue ? null : intValue.floatValue();
      Some(intValue as f32)
    }

    // java private static boolean dumpByteArrayToFile(byte[] data, String filePath)
    pub fn dumpByteArrayToFile() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static String dump(PackagePart part, String rootDir)
    pub fn dump() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static IDiagram dumpDiagram(String colorsRid, String dataRid, String layoutRid, String styleRid, Document doc, POIXMLDocumentPart parent, int shapeId, String gvmlThemePath)
    pub fn dumpDiagram() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static IDiagram buildDiagram(TextDocument textDocument, IMediaLib iMediaLib, Xml07Context cxt, String colorFilePath, String dataFilePath, String layoutFilePath, String styleFilePath, String drawingFilePath, RIDNode dataNode, RIDNode drawingNode, int shapeId, String gvmlThemePath)
    pub fn buildDiagram() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static DiagramColors getDiagramColorsPart(String colorsRid, POIXMLDocumentPart part)
    pub fn getDiagramColorsPart() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static DiagramData getDiagramDataPart(String dataRid, POIXMLDocumentPart part)
    pub fn getDiagramDataPart() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static String initDiagramDrawing(XWPFDiagramData dataRelatedPart)
    pub fn initDiagramDrawing() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static DiagramDrawing getDiagramDrawingPart(String drawingRid, POIXMLDocumentPart parent, POIXMLDocumentPart dataPart)
    pub fn getDiagramDrawingPart() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static POIXMLDocumentPart getSameNumDrawingPart(POIXMLDocumentPart parent, POIXMLDocumentPart dataPart, POIXMLDocumentPart drawingPart)
    pub fn getSameNumDrawingPart() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static String getPackagePartName(POIXMLDocumentPart part)
    pub fn getPackagePartName() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static POIXMLDocumentPart getSpecialDrawingPart(POIXMLDocumentPart dataPart, POIXMLDocumentPart parent)
    pub fn getSpecialDrawingPart() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static POIXMLDocumentPart getDrawingPart(POIXMLDocumentPart parent, String dataNumStr)
    pub fn getDrawingPart() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static String getNumInXmlName(String partPackageName)
    pub fn getNumInXmlName() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static DiagramLayout getDiagramLayoutPart(String layoutRid, POIXMLDocumentPart part)
    pub fn getDiagramLayoutPart() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static DiagramStyle getDiagramStylePart(String styleRid, POIXMLDocumentPart part)
    pub fn getDiagramStylePart() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static RIDNode getRIDNode(IMediaLib mediaLib, POIXMLDocumentPart part, int shapeId, String filePath, String rootDir, HashMap<String, String> part2MediaPath)
    pub fn getRIDNode() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static RelationshipCollection getRelationshipCollection(PackageRelationshipCollection relationships, POIXMLDocumentPart part, IMediaLib mediaLib, String rootDir, int shapeId, HashMap<String, String> part2MediaPath)
    pub fn getRelationshipCollection() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static Relationship.TargetMode getTargetMode(TargetMode mode)
    pub fn getTargetMode() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static boolean collectExternalRelation(PackageRelationship relation, RelationshipCollection relationshipCollection)
    pub fn collectExternalRelation() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static IDiagram newDiagramInstance(ClassLoader cl, String colorFilePath, String dataFilePath, String layoutFilePath, String styleFilePath, String drawingFilePath)
    pub fn newDiagramInstance() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static InkModel dumkInk(String rid, POIXMLDocumentPart parent, TextDocument doc)
    pub fn dumkInk() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static InputStream getInputStream(String rid, POIXMLDocumentPart part) throws IOException
    pub fn getInputStream() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static InkModel getInkModel(String rid, POIXMLDocumentPart part, TextDocument doc) throws IOException
    pub fn getInkModel() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static String dumpInkFile(InputStream is)
    pub fn dumpInkFile() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static LineProperty inkLineProperty(Ink ink)
    pub fn inkLineProperty() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static ChartData getChartData(String rId, POIXMLDocumentPart part)
    pub fn getChartData() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static String dumpChartData(ChartData chartData, String rootDir)
    pub fn dumpChartData() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static String dumpChartSrc(ChartSrc chartSrc, String rootDir)
    pub fn dumpChartSrc() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static ChartObject getChartObject(POIXMLDocumentPart part, String rootDir)
    pub fn getChartObject() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static PLCTextboxText.TextboxTextNode addTextboxTextNode(Document txtboxDocument, Shape shape)
    pub fn addTextboxTextNode(txtboxDocument: *mut SubDocument, shape: &Shape) -> Option<TextboxTextNode> {
        if txtboxDocument.is_null() { return None; }
        let sub = unsafe { &mut *txtboxDocument };
        let cp_start = sub.getLength();
        let reuse = sub.mTextboxText.as_ref().and_then(|plc| plc.last_textbox())
            .filter(|node| node.getCP() == cp_start);
        let plc = sub.mTextboxText.as_mut().expect("coreTextboxText");
        let mut node = if let Some(node) = reuse { node } else { plc.addTextbox(cp_start) };
        node.setShapeId(shape.getShapeID());
        node.setCi(1);
        node.setCReusable(0);
        node.setReusable(0);
        plc.update_textbox(&node);
        Some(node)
    }

    // java public static void addTextboxSeparator(Document txtboxDocument)
    pub fn addTextboxSeparator(txtboxDocument: *mut SubDocument) {
        IOAssert::assertNotNull_msg("txtboxDocument should not be null", &Some(()).filter(|_| !txtboxDocument.is_null()));
        if txtboxDocument.is_null() { return; }
        let sub = unsafe { &mut *txtboxDocument };
        let length = sub.getLength();
        let last = sub.mTextboxText.as_ref().and_then(|plc| plc.last_textbox());
        let Some(last) = last else { return; };
        if last.getCP() == length {
            sub.mTextboxText.as_mut().expect("coreTextboxText").removeTextbox(&last);
        } else {
            let empty = PropertySet::empty();
            CoreHelper::appendParagraph__char_ps_ps(Some(sub), '\r', Some(&empty), Some(&empty));
        }
    }

    // java private static PLCTextboxText.TextboxTextNode getLastTxtbxTextNode(PLCTextboxText coreTextboxText)
    pub fn getLastTxtbxTextNode(coreTextboxText: &crate::cn::wps::moffice::writer::data::PLCTextboxText::PLCTextboxText) -> Option<TextboxTextNode> {
        coreTextboxText.last_textbox()
    }

    fn text_document_ptr(doc: &dyn Document) -> *mut TextDocument {
        if let Some(sub_document) = doc.asAny().downcast_ref::<SubDocument>() {
            return SubDocument::getTextDocument(sub_document);
        }
        let text_document = doc.asAny().downcast_ref::<TextDocument>().expect("TextDocument");
        text_document as *const TextDocument as *mut TextDocument
    }

    // java public static Document getTextboxDocument(Document doc)
    pub fn getTextboxDocument(doc: &dyn Document) -> *mut SubDocument {
        let txtbox_document_type = match IDocument::getType(doc) {
            0 => 5,
            2 => 6,
            _ => {
                IOAssert::fail_msg("VML objects occured in unexpected sub-document.");
                5
            }
        };
        let text_document = Self::text_document_ptr(doc);
        if text_document.is_null() { return std::ptr::null_mut(); }
        unsafe {
            (*text_document).createSubDocument(txtbox_document_type)
                .map(|sub| sub as *const SubDocument as *mut SubDocument)
                .unwrap_or(std::ptr::null_mut())
        }
    }

    // java public static final float opacity2Transparency(float floatVal)
    pub fn opacity2Transparency() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static final Shape createOLeShape(Shapes2Core.OLEObjectInfo objectInfo, SubDocument subDocument, Xml07Context xml07Context)
    pub fn createOLeShape() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static Random getRandom()
    pub fn getRandom() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static String creatFile(String binName)
    pub fn creatFile() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static IChart dumpChart(ChartOOXmlData ooxmlData, Document document, Xml07Context cxt)
    pub fn dumpChart() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static cn.wps.moffice.drawing.chart.IHostApp getHostApp(String themePath, Map<String, Integer> rid2MediaId, final IMediaLib mediaLib, final String docAppVersion, final String docAppName)
    pub fn getHostApp() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static String getThemePath(Theme theme, IMediaLib mediaLib, String rootDir, Xml07Context cxt)
    pub fn getThemePath() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static Date convertDate(String dateTimeStr)
    pub fn convertDate() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static MutablePropertySet getDifferenceRunProp(MutablePropertySet originalProp, MutablePropertySet afterProp)
    pub fn getDifferenceRunProp() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static byte[] getRgbxchNums(String lvlTextVal)
    pub fn getRgbxchNums() { todo!("DEFERRED_NON_FUNCTION"); }

    // java private static boolean isValidLevelNum(char codePoint)
    pub fn isValidLevelNum() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static String convertLvlTextVal2Xst(String lvlTextVal, int lvlIlvl)
    pub fn convertLvlTextVal2Xst() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public static Shape createPicShape(Shape shape, SubDocument textboxDocument)
    pub fn createPicShape() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public IMediaLib mediaLib()
    pub fn mediaLib() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public IColorMapping colorMap()
    pub fn colorMap() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public HostTheme hostTheme()
    pub fn hostTheme() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public int appType()
    pub fn appType() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public String docAppVersion()
    pub fn docAppVersion() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public String docAppName()
    pub fn docAppName() { todo!("DEFERRED_NON_FUNCTION"); }

    // java public void destroy()
    pub fn destroy() { todo!("DEFERRED_NON_FUNCTION"); }

}
