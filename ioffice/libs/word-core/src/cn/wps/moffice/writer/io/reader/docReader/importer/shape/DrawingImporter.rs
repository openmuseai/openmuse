// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.moffice.drawing.GroupShape
use drawing::cn::wps::moffice::drawing::GroupShape::*;
// Java: import cn.wps.moffice.drawing.IDrawingContainer
use drawing::cn::wps::moffice::drawing::IDrawingContainer::*;
// Java: import cn.wps.moffice.drawing.IPos
use drawing::cn::wps::moffice::drawing::IPos::*;
// Java: import cn.wps.moffice.drawing.OLE
use drawing::cn::wps::moffice::drawing::OLE::*;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::*;
// Java: import cn.wps.moffice.drawing.ShapeFactory
use drawing::cn::wps::moffice::drawing::ShapeFactory::*;
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::*;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.util.OleFileUtil
use crate::cn::wps::moffice::util::OleFileUtil::*;
// Java: import cn.wps.moffice.util.StringUtil
use util::moffice::util::StringUtil::*;
// Java: import cn.wps.moffice.writer.VersionState
use crate::cn::wps::moffice::writer::VersionState::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.io.reader.docReader.importer.SubDocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docReader::importer::SubDocumentImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docReader.tools.ShapeReaderTool
use crate::cn::wps::moffice::writer::io::reader::docReader::tools::ShapeReaderTool::*;
// Java: import cn.wps.moffice.writer.io.tools.doc.TextBoxPropTransfrom
use crate::cn::wps::moffice::writer::io::tools::doc::TextBoxPropTransfrom::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.util.ArrayList
// Java: import org.apache.poi.ddf.EscherContainerRecord
use io_poi_compact::org::apache::poi::ddf::EscherContainerRecord::*;
// Java: import org.apache.poi.ddf.EscherDggContainerRecord
use io_poi_compact::org::apache::poi::ddf::EscherDggContainerRecord::*;
// Java: import org.apache.poi.hwpf.HWPFDocument
use io_poi_compact::org::apache::poi::hwpf::HWPFDocument::*;
// Java: import org.apache.poi.hwpf.model.EscherRecordHolderWriter
use io_poi_compact::org::apache::poi::hwpf::model::EscherRecordHolderWriter::*;
// Java: import org.apache.poi.hwpf.model.GenericPropertyNode
use io_poi_compact::org::apache::poi::hwpf::model::GenericPropertyNode::*;
// Java: import org.apache.poi.hwpf.ole.DocOLEInfo
use io_poi_compact::org::apache::poi::hwpf::ole::DocOLEInfo::*;
// Java: import org.apache.poi.hwpf.usermodel.Picture
use io_poi_compact::org::apache::poi::hwpf::usermodel::Picture::*;
// Java: import org.apache.poi.util.BitField
use util::poi_util::BitField::*;
// Java: import org.apache.poi.util.BitFieldFactory
use util::poi_util::BitFieldFactory::*;
// Java: import org.apache.poi.util.LittleEndian
use util::poi_util::LittleEndian::*;
// Java: import cn.wps.moffice.writer.io.reader.docReader.importer.shape.PictureImporter
use crate::cn::wps::moffice::writer::io::reader::docReader::importer::shape::PictureImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docReader.importer.shape.InlineDrawingImporter
use crate::cn::wps::moffice::writer::io::reader::docReader::importer::shape::InlineDrawingImporter::*;
// Java: import cn.wps.moffice.writer.io.reader.docReader.importer.shape.AnchorDrawingImporter
use crate::cn::wps::moffice::writer::io::reader::docReader::importer::shape::AnchorDrawingImporter::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::*;

// java private static final String TAG = null;
const TAG: &str = "";

// java public final class DrawingImporter
#[derive(Default)]
pub struct DrawingImporter {
    // java private SubDocumentImporter mParent = null;
    mParent: Option<SubDocumentImporter>,
    // java private HWPFDocument mDiskDoc = null;
    mDiskDoc: Option<HWPFDocument>,
    // java private EscherContainerRecord mBStoreContainer = null;
    mBStoreContainer: Option<EscherContainerRecord>,
    // java private EscherContainerRecord mSpgrContainer = null;
    mSpgrContainer: Option<EscherContainerRecord>,
    // java private Document mSubDocument = null;
    mSubDocument: Option<Document>,
    // java private ArrayList<PictureImporter.BlipDocInfo> mBlipDocInfoList = null;
    mBlipDocInfoList: Option<Vec<PictureImporter::BlipDocInfo>>,
    // java private InlineDrawingImporter mInlineDrawingImporter = null;
    mInlineDrawingImporter: Option<InlineDrawingImporter>,
    // java private AnchorDrawingImporter mAnchorDrawingImporter = null;
    mAnchorDrawingImporter: Option<AnchorDrawingImporter>,
}

// java public final class DrawingImporter
impl DrawingImporter {
    // java public  DrawingImporter(SubDocumentImporter parent)
    pub fn DrawingImporter(parent: &SubDocumentImporter) -> Self {
      // java IOAssert.assertNotNull("parent should not be null!", parent);
      IOAssert::assertNotNull__string_object(&"parent should not be null!", Some(&parent));
      // java this.mParent = parent;
      let mut this = Self { mParent: parent, mDiskDoc: self.mParent.getHWPFDocument(), mBStoreContainer: Default::default(), mSpgrContainer: Default::default(), mSubDocument: self.mParent.getSubDocument(), mBlipDocInfoList: self.mParent.getBlipInfoList(), mInlineDrawingImporter: InlineDrawingImporter::new(self.mParent), mAnchorDrawingImporter: AnchorDrawingImporter::new(self.mParent) };
      // java this.mDiskDoc = this.mParent.getHWPFDocument();
      // java this.mSubDocument = this.mParent.getSubDocument();
      // java this.mBlipDocInfoList = this.mParent.getBlipInfoList();
      // java this.mInlineDrawingImporter = new InlineDrawingImporter(this.mParent);
      // java this.mAnchorDrawingImporter = new AnchorDrawingImporter(this.mParent);
      // java this.initDggInfo();
      this.initDggInfo();
      this
    }

    // java private void initDggInfo()
    pub fn initDggInfo(&mut self) {
      // java IOAssert.assertNotNull("mDiskDoc should not be null!", this.mDiskDoc);
      IOAssert::assertNotNull__string_object(&"mDiskDoc should not be null!", Some(&self.mDiskDoc));
      // java EscherRecordHolderWriter dggHolder = this.mDiskDoc.getEscherRecordHolderWriter();
      let mut dggHolder: EscherRecordHolderWriter = self.mDiskDoc.getEscherRecordHolderWriter();
      // java if (null != dggHolder)
      if dggHolder.is_some() {
        // java EscherContainerRecord dgContainer = null;
        let mut dgContainer: EscherContainerRecord = EscherContainerRecord::default();
        // java int documentType = this.mSubDocument.getType();
        let mut documentType: i32 = self.mSubDocument.getType();
        // java if (0 == documentType)
        if 0 == documentType {
          // java dgContainer = dggHolder.getMainDgContainerRecord();
          dgContainer = dggHolder.getMainDgContainerRecord();
        }
        // java else if (2 == documentType)
        else if 2 == documentType {
          // java dgContainer = dggHolder.getHeaderContainerRecord();
          dgContainer = dggHolder.getHeaderContainerRecord();
        }
        // java if (null != dgContainer)
        if dgContainer.is_some() {
          // java this.mSpgrContainer = (EscherContainerRecord)dgContainer.getChildById((short)-4093);
          self.mSpgrContainer = (dgContainer.getChildById(((-4093) as i16))).downcast_ref::<EscherContainerRecord>().unwrap();
        }
      }
      // java if (null != this.mSpgrContainer)
      if let Some(mSpgrContainer) = self.mSpgrContainer.as_mut() {
        // java EscherDggContainerRecord dggContainer = (EscherDggContainerRecord)dggHolder.findFirstWithId((short)-4096);
        let mut dggContainer: EscherDggContainerRecord = (dggHolder.findFirstWithId(((-4096) as i16))).downcast_ref::<EscherDggContainerRecord>().unwrap();
        // java this.mBStoreContainer = (EscherContainerRecord)dggContainer.getChildById((short)-4095);
        self.mBStoreContainer = (dggContainer.getChildById(((-4095) as i16))).downcast_ref::<EscherContainerRecord>().unwrap();
      }
    }

    // java public Shape onImportInlineDrawing(Picture srcPic)
    pub fn onImportInlineDrawing(&mut self, srcPic: &Picture) -> Shape {
      // java IOAssert.assertNotNull("srcPic should not be null!", srcPic);
      IOAssert::assertNotNull__string_object(&"srcPic should not be null!", Some(&srcPic));
      // java IOAssert.assertNotNull("mInlineDrawingImporter should not be null!", this.mInlineDrawingImporter);
      IOAssert::assertNotNull__string_object(&"mInlineDrawingImporter should not be null!", Some(&self.mInlineDrawingImporter));
      // java Shape shape = ShapeFactory.getShapeFactory().createShape(this.mSubDocument.getDrawingContainer());
      let mut shape: Shape = ShapeFactory::getShapeFactory().createShape(self.mSubDocument.getDrawingContainer());
      // java IOAssert.assertNotNull("shape should not be null!", shape);
      IOAssert::assertNotNull__string_object(&"shape should not be null!", Some(&shape));
      // java shape.setIsFromDml(false);
      shape.setIsFromDml(false);
      // java shape.setInDggInfo(false);
      shape.setInDggInfo(false);
      // java shape.setInlineShape(true);
      shape.setInlineShape(true);
      // java this.mSubDocument.getDrawingContainer().addShape(shape);
      self.mSubDocument.getDrawingContainer().addShape(shape);
      // java return this.mInlineDrawingImporter.onImport(srcPic, shape);
      return self.mInlineDrawingImporter.onImport(&srcPic, &mut shape);
    }

    // java public Shape onImportAnchorDrawing(int spid, GenericPropertyNode fspaNode)
    pub fn onImportAnchorDrawing(&mut self, spid: i32, fspaNode: &GenericPropertyNode) -> Option<Shape> {
      // java if (null == this.mSpgrContainer)
      if self.mSpgrContainer.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java Shape shape = null;
        let mut shape: Shape = Shape::default();
        // java EscherContainerRecord shapeRecord = (EscherContainerRecord)this.mSpgrContainer.findChildByShapeId(spid);
        let mut shapeRecord: EscherContainerRecord = (self.mSpgrContainer.findChildByShapeId(spid)).downcast_ref::<EscherContainerRecord>().unwrap();
        // java if (null != shapeRecord)
        if shapeRecord.is_some() {
          // java switch (shapeRecord.getRecordId())
          match shapeRecord.getRecordId() {
            // java case -4093:
            -4093 => {
              // java shape = this.mAnchorDrawingImporter.onImportGroupShape(shapeRecord, this.mBStoreContainer, (GroupShape)null);
              shape = self.mAnchorDrawingImporter.onImportGroupShape(&shapeRecord, &self.mBStoreContainer, Default::default());
            }
            // java case -4092:
            -4092 => {
              // java shape = this.mAnchorDrawingImporter.onImportOneShape(shapeRecord, this.mBStoreContainer, (GroupShape)null);
              shape = self.mAnchorDrawingImporter.onImportOneShape(&shapeRecord, &self.mBStoreContainer, Default::default());
            }
            // java default:
            _ => {
            }
          }
          // java this.checkAnchorPosition(fspaNode, shape);
          self.checkAnchorPosition(&fspaNode, &mut shape);
        }
        // java return shape;
        return shape;
      }
    }

    // java private void checkAnchorPosition(GenericPropertyNode fspaNode, Shape shape)
    pub fn checkAnchorPosition(&self, fspaNode: &GenericPropertyNode, shape: &mut Shape) {
      // java IPos pos = ShapeReaderTool.getShapePos(this.mDiskDoc, fspaNode, shape.getShapePos());
      let mut pos: Box<dyn IPos> = ShapeReaderTool::getShapePos__hwpfdocument_genericpropertynode_ipos(&self.mDiskDoc, &fspaNode, Some(&mut shape.getShapePos()));
      // java shape.setShapePos(pos);
      shape.setShapePos(Some(&pos));
      // java int offset = fspaNode.getOffset() + 20;
      let mut offset: i32 = fspaNode.getOffset() + 20;
      // java int option = LittleEndian.getInt(fspaNode.getBytes(), offset);
      let mut option: i32 = LittleEndian::getInt__bytearr_int(&fspaNode.getBytes(), offset);
      // java BitField wr = BitFieldFactory.getInstance(480);
      let mut wr: BitField = BitFieldFactory::getInstance(480);
      // java BitField wrk = BitFieldFactory.getInstance(7680);
      let mut wrk: BitField = BitFieldFactory::getInstance(7680);
      // java BitField fBelowText = BitFieldFactory.getInstance(16384);
      let mut fBelowText: BitField = BitFieldFactory::getInstance(16384);
      // java BitField fAnchorLock = BitFieldFactory.getInstance(32768);
      let mut fAnchorLock: BitField = BitFieldFactory::getInstance(32768);
      // java pos.setTextWrap(TextBoxPropTransfrom.MsoTextWrap2CoreTextWrap(wr.getValue(option)));
      pos.setTextWrap(TextBoxPropTransfrom::MsoTextWrap2CoreTextWrap(wr.getValue(option)));
      // java pos.setTextWrapType(wrk.getValue(option));
      pos.setTextWrapType(wrk.getValue(option));
      // java shape.setBolowText(fBelowText.isSet(option));
      shape.setBolowText(fBelowText.isSet(option));
      // java shape.setAnchorLock(fAnchorLock.isSet(option));
      shape.setAnchorLock(fAnchorLock.isSet(option));
      // java if (!shape.isChildShape())
      if !shape.isChildShape() {
        // java ShapeReaderTool.convertGeoAdjust(shape);
        ShapeReaderTool::convertGeoAdjust(&mut shape);
      }
    }

    // java public PropertySet onImportOLE(PropertySet runProp)
    pub fn onImportOLE(&mut self, runProp: &PropertySet) -> Option<PropertySet> {
      // java String fieldStr = this.getFiledString();
      let mut fieldStr: String = self.getFiledString();
      // java IOAssert.assertNotNull("fieldStr should not be null!", fieldStr);
      IOAssert::assertNotNull__string_object(&"fieldStr should not be null!", Some(&fieldStr));
      // java String progId = this.getOleProgId(fieldStr);
      let mut progId: String = self.getOleProgId(&mut fieldStr);
      // java boolean isVba = this.isVba(fieldStr);
      let mut isVba: bool = self.isVba(Some(&fieldStr));
      // java IOAssert.assertNotNull("progId should not be null!", progId);
      IOAssert::assertNotNull__string_object(&"progId should not be null!", Some(&progId));
      // java IOAssert.assertNotNull("runProp should not be null!", runProp);
      IOAssert::assertNotNull__string_object(&"runProp should not be null!", Some(&runProp));
      // java if (null == progId)
      if progId.is_none() {
        // java return runProp;
        return runProp;
      }
      // java else
      else {
        // java DocOLEInfo docOleInfo = this.createOLEDocInfo(runProp);
        let mut docOleInfo: DocOLEInfo = self.createOLEDocInfo(&runProp);
        // java if (null == docOleInfo)
        if docOleInfo.is_none() {
          // java return runProp;
          return runProp;
        }
        // java else
        else {
          // java IDrawingContainer drawingContainer = this.mSubDocument.getDrawingContainer();
          let mut drawingContainer: Box<dyn IDrawingContainer> = self.mSubDocument.getDrawingContainer();
          // java int mediaId = drawingContainer.getMediaLib().addMedia(docOleInfo.getBinPath(), docOleInfo.getBinPath(), MediaTypeEnum.OLE);
          let mut mediaId: i32 = drawingContainer.getMediaLib().addMedia(docOleInfo.getBinPath(), docOleInfo.getBinPath(), MediaTypeEnum::OLE);
          // java if (mediaId < 0)
          if mediaId < 0 {
            // java return runProp;
            return runProp;
          }
          // java else
          else {
            // java Shape shape = ShapeFactory.getShapeFactory().createShape(drawingContainer);
            let mut shape: Shape = ShapeFactory::getShapeFactory().createShape(drawingContainer);
            // java shape.setShapeID(drawingContainer.newShapeID());
            shape.setShapeID(drawingContainer.newShapeID());
            // java shape.setIsFromDml(false);
            shape.setIsFromDml(false);
            // java OLE ole = new OLE(mediaId, VersionState.isProVersion() ? drawingContainer.getMediaLib() : null);
            let mut ole: OLE = OLE::new(mediaId, (if VersionState::isProVersion() { Some(drawingContainer.getMediaLib()) } else { None }));
            // java ole.setProgId(progId);
            ole.setProgId(progId);
            // java ole.setIsVba(isVba);
            ole.setIsVba(isVba);
            // java shape.setOLE(ole);
            shape.setOLE(ole);
            // java this.mSubDocument.getTextDocument().addOle(mediaId, docOleInfo);
            self.mSubDocument.getTextDocument().addOle(mediaId, docOleInfo);
            // java MutablePropertySet newMutableRunPropertySet = new MutablePropertySet(runProp);
            let mut newMutableRunPropertySet: MutablePropertySet = MutablePropertySet::new(runProp);
            // java newMutableRunPropertySet.setValue(56, shape);
            newMutableRunPropertySet.setValue(56, shape);
            // java return newMutableRunPropertySet.freeze();
            return newMutableRunPropertySet.freeze();
          }
        }
      }
    }

    // java private String getFiledString()
    pub fn getFiledString(&self) -> Option<String> {
      // java IChpxTable.Entry entry = this.mSubDocument.getChpxTable().end();
      let mut entry: Box<dyn Entry> = self.mSubDocument.getChpxTable().end();
      // java while (!entry.getPrev().isHead())
      while !entry.getPrev().isHead() {
        // java entry = entry.getPrev();
        entry = entry.getPrev();
        // java int start = entry.position();
        let mut start: i32 = entry.position();
        // java int end = entry.endCp();
        let mut end: i32 = entry.endCp();
        // java int len = end - start;
        let mut len: i32 = end - start;
        // java if (len == 1)
        if len == 1 {
          // java char[] tmp = new char[1];
          let mut tmp: Vec<char> = vec![Default::default(); 1usize];
          // java this.mSubDocument.getText(start, start + 1, tmp, 0);
          self.mSubDocument.getText(start, start + 1, &tmp, 0);
          // java char c = tmp[0];
          let mut c: char = tmp[0usize];
          // java if (c == 19)
          if c == 19 {
            // java break;
            break;
          }
        }
      }
      // java if (entry.isHead())
      if entry.isHead() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java int start = entry.endCp();
        let mut start: i32 = entry.endCp();
        // java int len = this.mSubDocument.getLength() - entry.endCp();
        let mut len: i32 = self.mSubDocument.getLength() - entry.endCp();
        // java if (len < 1)
        if len < 1 {
          // java return null;
          return None;
        }
        // java else
        else {
          // java char[] buffer = new char[len];
          let mut buffer: Vec<char> = vec![Default::default(); (len) as usize];
          // java this.mSubDocument.getText(start, start + len, buffer, 0);
          self.mSubDocument.getText(start, start + len, &buffer, 0);
          // java String text = String.valueOf(buffer, 0, len);
          let mut text: String = String::valueOf(buffer, 0, len);
          // java return text;
          return text;
        }
      }
    }

    // java private String getOleProgId(String fieldStr)
    pub fn getOleProgId(&self, fieldStr: &mut str) -> Option<String> {
      // java if (StringUtil.isEmpty(fieldStr))
      if StringUtil::isEmpty(Some(&mut fieldStr)) {
        // java return null;
        return None;
      }
      // java else
      else {
        // java String progId = null;
        let mut progId: String = String::new();
        // java String embed = KFieldType.FieldEmbed.toString();
        let mut embed: String = KFieldType::FieldEmbed.toString();
        // java String control = KFieldType.FieldControl.toString();
        let mut control: String = KFieldType::FieldControl.toString();
        // java if (fieldStr.contains(embed))
        if fieldStr.contains(embed) {
          // java progId = fieldStr.substring(fieldStr.lastIndexOf(embed) + embed.length());
          progId = fieldStr.substring(fieldStr.lastIndexOf(embed) + (embed.len() as i32));
        }
        // java else if (fieldStr.contains(control))
        else if fieldStr.contains(control) {
          // java progId = fieldStr.substring(fieldStr.lastIndexOf(control) + control.length());
          progId = fieldStr.substring(fieldStr.lastIndexOf(control) + (control.len() as i32));
          // java progId = progId.replace("Forms.", "").replace(".", "");
          progId = progId.replace("Forms.", "").replace(".", "");
        }
        // java if (null != progId && progId.length() > 0)
        if progId.is_some() && (progId.len() as i32) > 0 {
          // java progId = progId.replace(" ", "").replace("\\s", "");
          progId = progId.replace(" ", "").replace("\\s", "");
        }
        // java return progId;
        return progId;
      }
    }

    // java private boolean isVba(String fieldStr)
    pub fn isVba(&self, fieldStr: Option<&str>) -> Option<bool> {
      // java return fieldStr != null && fieldStr.contains(KFieldType.FieldControl.toString());
      return fieldStr.is_some() && fieldStr.contains(KFieldType::FieldControl.toString());
    }

    // java private DocOLEInfo createOLEDocInfo(PropertySet runProp)
    pub fn createOLEDocInfo(&self, runProp: &PropertySet) -> Option<DocOLEInfo> {
      // java int picLocation = runProp.getInt(32);
      let mut picLocation: i32 = runProp.getInt__int(32);
      // java File olePath = OleFileUtil.createTmpOleFile();
      let mut olePath: std::path::PathBuf = OleFileUtil::createTmpOleFile();
      // java if (null == olePath)
      if olePath.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java DocOLEInfo docOleInfo = new DocOLEInfo(picLocation, olePath.getAbsolutePath(), this.mDiskDoc.isEncrypted());
        let mut docOleInfo: DocOLEInfo = DocOLEInfo::new(picLocation, olePath.getAbsolutePath(), self.mDiskDoc.isEncrypted());
        // java return docOleInfo;
        return docOleInfo;
      }
    }

}
