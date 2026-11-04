// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.base.io.MathArgmentType
// rust: no-stmt
// Java: import cn.wps.base.log.IOLog
// rust: no-stmt
// Java: import cn.wps.collections.textrope.ITextPool
use crate::cn::wps::collections::textrope::ITextPool::*;
// Java: import cn.wps.io.xwpf.IDmlImporter
// rust: no-stmt
// Java: import cn.wps.io.xwpf.IDocumentImporter
// rust: no-stmt
// Java: import cn.wps.io.xwpf.ISubDocumentImporter
// rust: no-stmt
// Java: import cn.wps.io.xwpf.IVmlImporter
// rust: no-stmt
// Java: import cn.wps.io.xwpf.MathIOType
// rust: no-stmt
// Java: import cn.wps.io.xwpf.element_handler.ParagraphSimple
use io_wps_io::cn::wps::io::xwpf::element_handler::ParagraphSimple::*;
// Java: import cn.wps.io.xwpf.element_handler.RubyContent
// rust: no-stmt
// Java: import cn.wps.io.xwpf.element_handler.RubyPr
// rust: no-stmt
// Java: import cn.wps.moffice.drawing.Shape
// rust: no-stmt
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.utils.SubDocumentImporterUtils
// rust: no-stmt
// Java: import cn.wps.moffice.writer.core.KStyle
// rust: no-stmt
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
// Java: import cn.wps.moffice.writer.core.helper.BitsOperation
// rust: no-stmt
// Java: import cn.wps.moffice.writer.core.io.IOListener
// rust: no-stmt
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{Entry, IPapxTable};
// Java: import cn.wps.moffice.writer.data.PLCField
// rust: no-stmt
// Java: import cn.wps.moffice.writer.data.PLCSection
// rust: no-stmt
// Java: import cn.wps.moffice.writer.data.field.KFieldType
// rust: no-stmt
// Java: import cn.wps.moffice.writer.helper.CoreHelper
use crate::cn::wps::moffice::writer::helper::CoreHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.ImporterBase
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::*;
// Java: import cn.wps.moffice.writer.io.reader.dmlReader.DmlImporter
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.dmlReader.context.ThemeContext
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.docReader.importer.shape.DrawingImporter
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.BookmarkImporter
// rust: ★M-STAGE2 P1-D1 —— `wc04_seam` 的空 struct 影子已删；改为真实模块。
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::BookmarkImporter::BookmarkImporter;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.DocFieldImporter
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.MathImporter
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ImporterHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ImporterHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ParaPropsConverterSimple
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ParaPropsConverterSimple::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.RunPropsConverterSimple
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::RunPropsConverterSimple::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.Shapes2Core4Docx
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.helper.ParagraphContext
use crate::cn::wps::moffice::writer::io::reader::helper::ParagraphContext::*;
// Java: import cn.wps.moffice.writer.io.reader.helper.Shapes2Core
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.vmlReader.importer.vml.VmlImporter
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.PartInfo
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
// Java: import cn.wps.util.concurrent.locks.RangeWriteLock
use crate::cn::wps::util::concurrent::locks::RangeWriteLock::*;
// Java: import java.io.IOException
// rust: no-stmt
// Java: import org.xml.sax.Attributes
// rust: no-stmt
// Java: import org.xml.sax.SAXException
// rust: no-stmt
// Java: import org.apache.poi.poifs.filesystem.Entry
// rust: no-stmt
// Java: import java.lang.CloneNotSupportedException
// rust: no-stmt
use crate::cn::wps::moffice::writer::core::Document::Document;
use std::sync::Arc;

// Temporary compile-surface seam for off-surface Java templates (skill policy #3).
mod wc04_seam {
    #[derive(Default)] pub struct DrawingImporter;
    #[derive(Default)] pub struct MathImporter;
    #[derive(Default)] pub struct DocFieldImporter;
    #[derive(Default)] pub struct MathArgmentType;
    #[derive(Default)] pub struct Shape;
    #[derive(Default)] pub struct RubyPr;
    #[derive(Default)] pub struct RubyContent;
    #[derive(Default)] pub struct MathIOType;
    // rust: ★M-STAGE2 P1-D1 —— 影子 `IOListener`（只有 3 个空方法）已删除；
    //   真实模块 `core/io/IOListener.rs`（9 个方法，已在 lib.rs 登记）改为直接引用。
    pub trait ISubDocumentImporter {}
    pub trait IDocumentImporter {}
    pub trait IVmlImporter {}
    pub trait IDmlImporter {}
}
use wc04_seam::*;
pub use wc04_seam::Shape;
pub use crate::cn::wps::moffice::writer::data::PLCField::FieldNode;
// rust: ★M-STAGE2 P1-D1 —— `IOListener` 由 `wc04_seam` 影子改为真实模块再导出。
pub use crate::cn::wps::moffice::writer::core::io::IOListener::IOListener;

// java public abstract class SubDocumentImporter extends ImporterBase implements ISubDocumentImporter
#[derive(Default)]
pub struct SubDocumentImporter {
    // java ImporterBase super_
    pub(crate) super_: ImporterBase,
    // java protected SubDocument mDocument;
    pub(crate) mDocument: Option<*mut SubDocument>,
    // java protected ITextPool mCharBuffer;
    pub(crate) mCharBuffer: Option<Box<dyn ITextPool>>,
    // java protected StyleId2Sti mStyleId2Sti;
    pub(crate) mStyleId2Sti: Option<StyleId2Sti>,
    // java protected NumIdRedirection mNumIdRedirection;
    pub(crate) mNumIdRedirection: Option<NumIdRedirection>,
    // java protected DrawingImporter mDrawingImporter;
    pub(crate) mDrawingImporter: Option<DrawingImporter>,
    // java protected MathImporter mMathImporter;
    pub(crate) mMathImporter: Option<MathImporter>,
    // java private PLCField.FieldNode mComplexFieldNodeBegin;
    mComplexFieldNodeBegin: Option<FieldNode>,
    // java protected int mFieldsNotEnded;
    pub mFieldsNotEnded: i32,
    // java private boolean mShouldBeRecalculated;
    mShouldBeRecalculated: bool,
    // java @Deprecated private ParagraphContext mCurrentParagraph;
    pub mCurrentParagraph: Option<ParagraphContext>,
    // java private BookmarkImporter mBookmarkImporter;
    mBookmarkImporter: Option<BookmarkImporter>,
    // java private DocFieldImporter mDocFieldImporter;
    mDocFieldImporter: Option<DocFieldImporter>,
    // java private RangeWriteLock mWriteLock;
    pub mWriteLock: Option<RangeWriteLock>,
    // java private IOListener mIoListener;
    mIoListener: Option<Box<dyn IOListener>>,
    // java private DmlImporter mDmlImporter;
    mDmlImporter: Option<std::rc::Rc<std::cell::RefCell<crate::cn::wps::moffice::writer::io::reader::dmlReader::DmlImporter::DmlImporter>>>,
    // Java: VmlImporter cached the same way as mDmlImporter.
    mVmlImporter: Option<std::rc::Rc<std::cell::RefCell<crate::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::VmlImporter::VmlImporter>>>,
    // java private final int MINLAYERTABLE = 1;
    MINLAYERTABLE: i32,
    // java private boolean mIsFirstUnlock = true;
    mIsFirstUnlock: bool,
}

// java public interface ISubDocumentImporter
impl ISubDocumentImporter for SubDocumentImporter {
}

// java public abstract class SubDocumentImporter extends ImporterBase implements ISubDocumentImporter
impl SubDocumentImporter {
    // java protected  SubDocumentImporter(SubDocument document, StyleId2Sti styleId2Sti, NumIdRedirection numIdRedirection, IOListener ioListener, boolean initDrawingContainer, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn SubDocumentImporter(document: &mut SubDocument, styleId2Sti: StyleId2Sti, numIdRedirection: NumIdRedirection, ioListener: Option<Box<dyn IOListener>>, initDrawingContainer: bool, mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(mode, xml07Context, xml03Context);
      // java IOAssert.assertNotNull("document should not be null!", document);
      // rust: no-stmt
      // java IOAssert.assertNotNull("styleId2Sti should not be null!", styleId2Sti);
      // rust: no-stmt
      // java this.mDocument = document;
      let this = Self { super_: ImporterBase::ImporterBase(mode, xml07Context, xml03Context), mDocument: Some(document as *mut SubDocument), mCharBuffer: Default::default(), mStyleId2Sti: Some(styleId2Sti), mNumIdRedirection: Some(numIdRedirection), mDrawingImporter: Default::default(), mMathImporter: Default::default(), mComplexFieldNodeBegin: None, mFieldsNotEnded: 0, mShouldBeRecalculated: false, mCurrentParagraph: Default::default(), mBookmarkImporter: Default::default(), mDocFieldImporter: Default::default(), mWriteLock: None, mIoListener: ioListener, mDmlImporter: Default::default(), mVmlImporter: Default::default(), MINLAYERTABLE: 1, mIsFirstUnlock: true };
      // java document.getDrawingContainer().beginRead();
      // rust: no-stmt
      // java this.mStyleId2Sti = styleId2Sti;
      // java this.mNumIdRedirection = numIdRedirection;
      // java this.mComplexFieldNodeBegin = null;
      // java this.mFieldsNotEnded = 0;
      // java this.mShouldBeRecalculated = false;
      // java this.mWriteLock = null;
      // java this.mIoListener = ioListener;
      this
    }

    // java public void importParagraphStart(String paraId)
    pub fn importParagraphStart(&mut self, paraId: Option<&str>) {
      // java this.mCurrentParagraph = new ParagraphContext(this.mDocument.getLength());
      let len = match self.mDocument {
        Some(p) => unsafe { (*p).getLength() },
        None => 0,
      };
      self.mCurrentParagraph = Some(ParagraphContext::ParagraphContext__int(len));
      // java if (null != paraId)
      if let Some(paraId) = paraId {
        // java this.mCurrentParagraph.mParaId = paraId;
        if let Some(ctx) = self.mCurrentParagraph.as_mut() {
          ctx.mParaId = Some(paraId.to_string());
        }
      }
    }

    // java private boolean isContainNumbering(IPapxTable.Entry paraBreak)
    pub fn isContainNumbering(&self, paraBreak: &dyn Entry) -> bool {
      // java IOAssert.assertNotNull("paraBreak should not be null!", paraBreak);
      // rust: no-stmt
      // java PropertySet prop = paraBreak.getProperty();
      let Some(prop) = Entry::getProperty(paraBreak) else { return false; };
      // java if (!prop.contains(234) && !prop.contains(235))
      if !prop.contains(234) && !prop.contains(235) {
        // java if (prop.contains(191))
        if prop.contains(191) {
          // java Integer style = (Integer)prop.getValue(191);
          let style = prop.getInt(191);
          // java KStyle kStyle = this.mDocument.getStyles().getStyleById(style);
          let kstyle = match self.mDocument {
            Some(d) => unsafe { (*d).getStyles() }
              .and_then(|s| unsafe { (*s).getStyleById__int(style) }),
            None => None,
          };
          // java if (null == kStyle)
          let Some(kstyle) = kstyle else { return false; };
          // java prop = kStyle.getProperty();
          let Some(prop) = kstyle.getProperty() else { return false; };
          // java if (prop.contains(234) || prop.contains(235))
          if prop.contains(234) || prop.contains(235) {
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

    // java public void importTableStart(int layerTable)
    pub fn importTableStart(&mut self, layerTable: i32) {
      // java if (null != this.mCurrentParagraph)
      if self.mCurrentParagraph.is_some() {
        // java IOAssert.assertTrue("layerTable >= 1", layerTable >= 1);
        IOAssert::assertTrue_msg("layerTable >= 1", layerTable >= 1);
        // java IOAssert.assertNotNull("mCurrentParagraph should not be null", this.mCurrentParagraph);
        // rust: no-stmt
        let doc_ptr = self.mDocument.expect("mDocument");
        // java PropertySet props = this.mDocument.getPapxTable().last().getProperty();
        let last = unsafe { (*doc_ptr).getPapxTable().and_then(|t| IPapxTable::last(t)) };
        let props_layer = last
          .as_ref()
          .and_then(|e| Entry::getProperty(e.as_ref()))
          .map(|p| p.getInt__int_int(224, 0))
          .unwrap_or(0);
        // java IOAssert.assertNotNull("props should not be null", props);
        // rust: no-stmt
        // java if (layerTable == props.getInt(224, 0))
        if layerTable == props_layer {
          // java PropertySet prop = PropertySet.empty;
          let mut prop: PropertySet = (*empty).clone();
          // java if (layerTable > 1)
          if layerTable > 1 {
            // java MutablePropertySet dst = new MutablePropertySet();
            let mut dst: MutablePropertySet = MutablePropertySet::new();
            // java dst.setValue(224, layerTable - 1);
            dst.setInt(224, layerTable - 1);
            // java dst.setBoolean(223, true);
            dst.setBoolean(223, true);
            // java prop = dst.freeze();
            prop = dst.freeze();
          }
          // java CoreHelper.appendParagraph(this.mDocument, '\r', PropertySet.empty, prop);
          let empty_ps = (*empty).clone();
          CoreHelper::appendParagraph__char_ps_ps(
            Some(unsafe { &mut *doc_ptr as &mut dyn Document }),
            '\r',
            Some(&empty_ps),
            Some(&prop),
          );
        }
      }
    }

    // java public void importRunText4InlineChart(String runText, MutablePropertySet runProps)
    pub fn importRunText4InlineChart(&mut self, runText: &str, runProps: &MutablePropertySet) {
      // java IOAssert.assertNotNull("runText should not be null", runProps);
      // rust: no-stmt
      // java IOAssert.assertNotNull("runText should not be null", runText);
      // rust: no-stmt
      // java MutablePropertySet dst = new MutablePropertySet(runProps);
      let mut dst = MutablePropertySet::new_from_mutable(runProps);
      // java this.importRunText(runText, dst.freeze());
      let frozen = dst.freeze();
      self.importRunText(runText, &frozen);
    }

    // java public void importRunText(String runText, PropertySet runProps)
    pub fn importRunText(&mut self, runText: &str, runProps: &PropertySet) {
      // java IOAssert.assertNotNull("runText should not be null", runText);
      // rust: no-stmt
      // java IOAssert.assertNotNull("runProps should not be null", runProps);
      // rust: no-stmt
      // java if (runText.length() > 0)
      if (runText.len() as i32) > 0 {
        // java CoreHelper.appendText(this.mDocument, runText, runProps);
        let doc = self.mDocument.map(|p| unsafe { &mut *p as &mut dyn Document });
        CoreHelper::appendText__string_ps(doc, Some(runText), Some(runProps));
      }
      // java if (null != this.mComplexFieldNodeBegin && KFieldType.FieldEmpty.getFlt() == this.mComplexFieldNodeBegin.mData)
      // rust: no-stmt
        // java this.mComplexFieldNodeBegin.mData = SubDocumentImporterUtils.toFieldType(runText);
      // rust: no-stmt
    }

    // java public void importHyperlinkBegin(String hyperlink)
    pub fn importHyperlinkBegin(&mut self, hyperlink: &str) {
      /** DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("hyperlink should not be null", hyperlink);
      IOAssert::assertNotNull__string_object(&"hyperlink should not be null", Some(&hyperlink));
      // java this.importFldSimpleBeginOrHyperLinkBegin(hyperlink);
      self.importFldSimpleBeginOrHyperLinkBegin(&hyperlink);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importHyperlinkEnd()
    pub fn importHyperlinkEnd(&mut self) {
      /** DEFERRED_NON_FUNCTION
      // java this.importFldSimpleBeginOrHyperLinkEnd();
      self.importFldSimpleBeginOrHyperLinkEnd();
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importFldSimpleEnd()
    pub fn importFldSimpleEnd(&mut self) {
      /** DEFERRED_NON_FUNCTION
      // java this.importFldSimpleBeginOrHyperLinkEnd();
      self.importFldSimpleBeginOrHyperLinkEnd();
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public int appendControlCharField(char fieldChar)
    pub fn appendControlCharField__char(&mut self, fieldChar: char) -> i32 {
      // java return this.appendControlCharField(fieldChar, false);
      self.appendControlCharField__char_boolean(fieldChar, false)
    }

    // java public int appendControlCharField(char fieldChar, boolean isIgnoredField)
    pub fn appendControlCharField__char_boolean(&mut self, fieldChar: char, isIgnoredField: bool) -> i32 {
      // java return this.appendControlCharField(fieldChar, isIgnoredField, (Shape)null);
      self.appendControlCharField__char_boolean_shape(fieldChar, isIgnoredField, None)
    }

    // java public int appendControlCharField(char fieldChar, boolean isIgnoredField, Shape shape)
    pub fn appendControlCharField__char_boolean_shape(&mut self, fieldChar: char, isIgnoredField: bool, shape: Option<&Shape>) -> i32 {
      // java MutablePropertySet dstRunProp = new MutablePropertySet();
      let mut dstRunProp: MutablePropertySet = MutablePropertySet::new();
      // java dstRunProp.setBoolean(28, true);
      dstRunProp.setBoolean(28, true);
      // java if (isIgnoredField)
      if isIgnoredField {
        // java dstRunProp.setBoolean(758, isIgnoredField);
        dstRunProp.setBoolean(758, isIgnoredField);
      }
      // java if (null != shape)
      if shape.is_some() {
        // java dstRunProp.setValue(56, shape);
        dstRunProp.setValue(56, Some(Arc::new(true)));
        // java dstRunProp.setBoolean(30, true);
        dstRunProp.setBoolean(30, true);
        // java dstRunProp.setBoolean(29, true);
        dstRunProp.setBoolean(29, true);
      }
      // java return CoreHelper.appendText(this.mDocument, fieldChar, dstRunProp.freeze());
      let frozen = dstRunProp.freeze();
      let doc = self.mDocument.map(|p| unsafe { &mut *p as &mut dyn Document });
      CoreHelper::appendText__char_ps(doc, fieldChar, Some(&frozen))
    }

    // java public PLCField.FieldNode appendToPLCField(char fieldChar, int cpFieldChar)
    pub fn appendToPLCField(&mut self, fieldChar: char, cpFieldChar: i32) -> FieldNode {
      // java IOAssert.assertTrue("(ControlChar.FIELD_BEGIN == fieldChar ||ControlChar.FIELD_SEPARATOR == fieldChar ||ControlChar.FIELD_END == fieldChar) should not be false!", 19 == fieldChar || 20 == fieldChar || 21 == fieldChar);
      IOAssert::assertTrue_msg(
        "(ControlChar.FIELD_BEGIN == fieldChar ||ControlChar.FIELD_SEPARATOR == fieldChar ||ControlChar.FIELD_END == fieldChar) should not be false!",
        fieldChar == '\u{0013}' || fieldChar == '\u{0014}' || fieldChar == '\u{0015}',
      );
      // java IOAssert.assertTrue("cpFieldChar >= 0 should not be false!", cpFieldChar >= 0);
      IOAssert::assertTrue_msg("cpFieldChar >= 0 should not be false!", cpFieldChar >= 0);
      // java PLCField plcField = this.mDocument.getPLCField();
      let document = self.mDocument.expect("SubDocumentImporter document");
      // java IOAssert.assertNotNull("plcField should not be null!", plcField);
      let plcField = unsafe { &mut *document }.getPLCFieldMut().expect("PLCField");
      // java PLCField.FieldNode fieldNode = plcField.addFieldChar(cpFieldChar);
      let mut fieldNode: FieldNode = plcField.addFieldChar__int(cpFieldChar);
      // java IOAssert.assertNotNull("fieldNode should not be null!", fieldNode);
      // rust: no-stmt
      // java fieldNode.setFieldChar(fieldChar);
      fieldNode.setFieldChar(fieldChar as i32);
      // java if (19 == fieldChar)
      if fieldChar == '\u{0013}' {
        // java ++this.mFieldsNotEnded;
        self.mFieldsNotEnded += 1;
      }
      // java else if (21 == fieldChar)
      else if fieldChar == '\u{0015}' {
        // java --this.mFieldsNotEnded;
        self.mFieldsNotEnded -= 1;
        // java fieldNode.setFieldFlags(this.fieldFlags(fieldNode));
        let flags = self.fieldFlags(&fieldNode);
        fieldNode.setFieldFlags(flags);
      }
      // java return fieldNode;
      fieldNode
    }

    // java private int fieldFlags(PLCField.FieldNode fieldEnd)
    pub fn fieldFlags(&mut self, fieldEnd: &FieldNode) -> i32 {
      // java IOAssert.assertNotNull("fieldEnd should not be nul!", fieldEnd);
      // Rust's non-optional reference carries Java's non-null precondition.
      // java int flags = 0;
      let mut flags = 0;
      // java if (this.mFieldsNotEnded >= 1)
      if self.mFieldsNotEnded >= 1 {
        // java flags = BitsOperation.setBits(flags, 2);
        flags = crate::cn::wps::moffice::writer::core::helper::BitsOperation::BitsOperation::setBits(flags, 2);
      }
      // java else
      else {
        // java flags = BitsOperation.resetBits(flags, 2);
        flags = crate::cn::wps::moffice::writer::core::helper::BitsOperation::BitsOperation::resetBits(flags, 2);
      }
      // java if (this.mShouldBeRecalculated)
      if self.mShouldBeRecalculated {
        // java flags = BitsOperation.setBits(flags, 16);
        flags = crate::cn::wps::moffice::writer::core::helper::BitsOperation::BitsOperation::setBits(flags, 16);
      }
      // java else
      else {
        // java flags = BitsOperation.resetBits(flags, 16);
        flags = crate::cn::wps::moffice::writer::core::helper::BitsOperation::BitsOperation::resetBits(flags, 16);
      }
      // java this.mShouldBeRecalculated = false;
      self.mShouldBeRecalculated = false;
      // java boolean hasSeparator = this.fieldEndHasSeparator(fieldEnd);
      let hasSeparator = self.fieldEndHasSeparator(fieldEnd);
      // java if (hasSeparator)
      if hasSeparator {
        // java flags = BitsOperation.setBits(flags, 128);
        flags = crate::cn::wps::moffice::writer::core::helper::BitsOperation::BitsOperation::setBits(flags, 128);
      }
      // java else
      else {
        // java flags = BitsOperation.resetBits(flags, 128);
        flags = crate::cn::wps::moffice::writer::core::helper::BitsOperation::BitsOperation::resetBits(flags, 128);
      }
      // java return flags;
      flags
    }

    // java private boolean fieldEndHasSeparator(PLCField.FieldNode fieldEnd)
    pub fn fieldEndHasSeparator(&mut self, fieldEnd: &FieldNode) -> bool {
      // java PLCField.FieldNode fieldSeparator = (PLCField.FieldNode)fieldEnd.getPrev();
      let fieldSeparator = fieldEnd.getPrev().expect("field end must follow a field begin");
      // java if (fieldSeparator.isFieldSeperator())
      if fieldSeparator.isFieldSeperator() {
        // java return true;
        true
      }
      // java else if (fieldSeparator.isFieldBegin())
      else if fieldSeparator.isFieldBegin() {
        // java return false;
        false
      }
      // java else
      else {
        // java return null != fieldEnd.getSeparatorNode();
        fieldEnd.getSeparatorNode().is_some()
      }
    }

    // java private void importFldSimpleBeginOrHyperLinkBegin(String fieldCode)
    pub fn importFldSimpleBeginOrHyperLinkBegin(&mut self, fieldCode: &str) {
      /** DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("fieldCode should not be null", fieldCode);
      IOAssert::assertNotNull__string_object(&"fieldCode should not be null", Some(&fieldCode));
      // java char fieldChar = '\u0013';
      let mut fieldChar: char = '\u{0013}';
      // java int cpFieldChar = this.appendControlCharField(fieldChar);
      let mut cpFieldChar: i32 = self.appendControlCharField__char(fieldChar);
      // java PLCField.FieldNode fieldBeginNode = this.appendToPLCField(fieldChar, cpFieldChar);
      let mut fieldBeginNode: FieldNode = self.appendToPLCField(fieldChar, cpFieldChar);
      // java fieldBeginNode.mData = SubDocumentImporterUtils.toFieldType(fieldCode);
      fieldBeginNode.mData = SubDocumentImporterUtils::toFieldType(&mut fieldCode);
      // java if (fieldCode.length() > 0)
      if (fieldCode.len() as i32) > 0 {
        // java CoreHelper.appendText(this.mDocument, fieldCode, PropertySet.empty);
        CoreHelper::appendText__subdocument_char_propertyset(&self.mDocument, fieldCode, &PropertySet::empty());
      }
      // java fieldChar = '\u0014';
      fieldChar = '\u{0014}';
      // java cpFieldChar = this.appendControlCharField(fieldChar);
      cpFieldChar = self.appendControlCharField__char(fieldChar);
      // java this.appendToPLCField(fieldChar, cpFieldChar);
      self.appendToPLCField(fieldChar, cpFieldChar);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java private void importFldSimpleBeginOrHyperLinkEnd()
    pub fn importFldSimpleBeginOrHyperLinkEnd(&mut self) {
      /** DEFERRED_NON_FUNCTION
      // java char fieldChar = 21;
      let mut fieldChar: char = 21;
      // java int cpFieldChar = this.appendControlCharField('\u0015');
      let mut cpFieldChar: i32 = self.appendControlCharField__char('\u{0015}');
      // java this.appendToPLCField('\u0015', cpFieldChar);
      self.appendToPLCField('\u{0015}', cpFieldChar);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    pub fn getRoutedType(&self) -> i32 {
      match self.mDocument {
        Some(p) if !p.is_null() => unsafe { (*p).getType() },
        _ => -1,
      }
    }

    pub fn getSubDocumentPtr(&self) -> Option<*mut SubDocument> {
      self.mDocument
    }

    pub fn clear(&mut self) {}

    // java public SubDocument getSubDocument()
    pub fn getSubDocument(&self) -> SubDocument {
      /** DEFERRED_NON_FUNCTION
      // java return this.mDocument;
      return self.mDocument;
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public StyleId2Sti getStyleId2Sti()
    pub fn getStyleId2Sti(&self) -> StyleId2Sti {
      /** DEFERRED_NON_FUNCTION
      // java return this.mStyleId2Sti;
      return self.mStyleId2Sti;
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public NumIdRedirection getNumIdRedirection()
    pub fn getNumIdRedirection(&self) -> NumIdRedirection {
      /** DEFERRED_NON_FUNCTION
      // java return this.mNumIdRedirection;
      return self.mNumIdRedirection;
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java private void updateShouldBeRecalculated(Boolean shouldBeRecalculated)
    pub fn updateShouldBeRecalculated(&mut self, shouldBeRecalculated: Option<bool>) {
      // java boolean value = null == shouldBeRecalculated ? false : shouldBeRecalculated;
      let value = shouldBeRecalculated.unwrap_or(false);
      // java this.mShouldBeRecalculated = this.mShouldBeRecalculated || value;
      self.mShouldBeRecalculated |= value;
    }

    // java public void importSpecialChar(MutablePropertySet dst, char speChar)
    pub fn importSpecialChar(&mut self, dst: Option<&mut MutablePropertySet>, speChar: char) {
      // java if (null == dst)
      let mut local = MutablePropertySet::new();
      let mut dst = dst.unwrap_or(&mut local);
      // java dst.setBoolean(28, true);
      dst.setBoolean(28, true);
      // java CoreHelper.appendText(this.mDocument, speChar, dst.freeze());
      let frozen = dst.freeze();
      let doc = self.mDocument.map(|p| unsafe { &mut *p as &mut dyn Document });
      CoreHelper::appendText__char_ps(doc, speChar, Some(&frozen));
    }

    // java  void lock()
    pub fn lock(&mut self) {
      // java if (null == this.mWriteLock)
      if self.mWriteLock.is_none() {
        // java IOAssert.assertNotNull("mDocument should not be null", this.mDocument);
        // rust: no-stmt
        let doc_ptr = self.mDocument.expect("mDocument");
        // java int len = this.mDocument.getLength();
        let len: i32 = unsafe { (*doc_ptr).getLength() };
        // java this.mWriteLock = this.mDocument.writeLock(len);
        self.mWriteLock = Some(unsafe { (*doc_ptr).writeLock__int(len) });
        // java if (0 == len && null != this.mIoListener)
        if len == 0 {
          if let Some(mIoListener) = self.mIoListener.as_mut() {
            // java this.mIoListener.onFirstLock();
            mIoListener.onFirstLock();
          }
        }
      }
    }

    // java public void unlock()
    pub fn unlock(&mut self) {
      // java if (null != this.mWriteLock)
      if self.mWriteLock.is_some() {
        // java this.mWriteLock.unlock();
        if let Some(mut mWriteLock) = self.mWriteLock.take() {
          mWriteLock.unlock();
        }
        // java this.mWriteLock = null;
        // rust: no-stmt
        // java if (this.mIsFirstUnlock && null != this.mIoListener)
        if self.mIsFirstUnlock {
          if let Some(mIoListener) = self.mIoListener.as_mut() {
            // java this.mIoListener.onFirstUnLock();
            mIoListener.onFirstUnLock();
            // java this.mIsFirstUnlock = false;
            self.mIsFirstUnlock = false;
          }
        }
      }
    }

    // java public void importOMathStart(boolean isInOMathPara, MutablePropertySet oMathParaPr)
    pub fn importOMathStart(&mut self, isInOMathPara: bool, oMathParaPr: &MutablePropertySet) {
      /** DEFERRED_NON_FUNCTION
      // java this.initMathImporter();
      self.initMathImporter();
      // java this.mMathImporter.importOMathStart(isInOMathPara, oMathParaPr);
      self.mMathImporter.importOMathStart(isInOMathPara, &oMathParaPr);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java protected void initMathImporter()
    pub fn initMathImporter(&mut self) {
      /** DEFERRED_NON_FUNCTION
      // java if (null == this.mMathImporter)
      if self.mMathImporter.is_none() {
        // java this.mMathImporter = new MathImporter(this.mDocument);
        self.mMathImporter = MathImporter::new(self.mDocument);
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importOMathEnd()
    pub fn importOMathEnd(&mut self) {
      /** DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("mMathImporter should not be null!", this.mMathImporter);
      IOAssert::assertNotNull__string_object(&"mMathImporter should not be null!", Some(&self.mMathImporter));
      // java this.mMathImporter.importOMathEnd();
      self.mMathImporter.importOMathEnd();
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importMathObjectBegin(MathIOType mathType)
    pub fn importMathObjectBegin(&mut self, mathType: &MathIOType) {
      /** DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("mMathImporter should not be null", this.mMathImporter);
      IOAssert::assertNotNull__string_object(&"mMathImporter should not be null", Some(&self.mMathImporter));
      // java this.mMathImporter.importerMathObjectBegin(mathType);
      self.mMathImporter.importerMathObjectBegin(&mathType);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importMathObjectEndSimple(MutablePropertySet runPropSet, MathIOType mathType)
    pub fn importMathObjectEndSimple(&mut self, runPropSet: &MutablePropertySet, mathType: &MathIOType) {
      /** DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("mMathImporter should not be null", this.mMathImporter);
      IOAssert::assertNotNull__string_object(&"mMathImporter should not be null", Some(&self.mMathImporter));
      // java this.mMathImporter.importMathObjectEndSimple(runPropSet, mathType);
      self.mMathImporter.importMathObjectEndSimple(&runPropSet, &mathType);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importMathArgmentStart(MathIOType mathType, MathArgmentType argmentType)
    pub fn importMathArgmentStart(&mut self, mathType: &MathIOType, argmentType: &MathArgmentType) {
      /** DEFERRED_NON_FUNCTION
      // java this.mMathImporter.importMathArgmentStart(mathType, argmentType);
      self.mMathImporter.importMathArgmentStart(&mathType, &argmentType);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importMathArgmentEnd(MutablePropertySet propSet, MathArgmentType argType, MathIOType mathType)
    pub fn importMathArgmentEnd(&mut self, propSet: &MutablePropertySet, argType: &MathArgmentType, mathType: &MathIOType) {
      /** DEFERRED_NON_FUNCTION
      // java this.mMathImporter.importMathArgmentEnd(propSet, argType, mathType);
      self.mMathImporter.importMathArgmentEnd(&propSet, &argType, &mathType);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importMathElementsProp(MathIOType mathType, int elementId, Attributes attributes)
    pub fn importMathElementsProp(&mut self, mathType: &MathIOType, elementId: i32, attributes: &dyn Attributes) {
      /** DEFERRED_NON_FUNCTION
      // java this.mMathImporter.importMathElementsProp(mathType, elementId, attributes);
      self.mMathImporter.importMathElementsProp(&mathType, elementId, &attributes);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importMathMatrixRowEnd()
    pub fn importMathMatrixRowEnd(&mut self) {
      /** DEFERRED_NON_FUNCTION
      // java this.mMathImporter.importMatrixRowEnd();
      self.mMathImporter.importMatrixRowEnd();
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public IPapxTable.Entry importParagraphEnd(ParagraphSimple paragraph, int layerTable)
    pub fn importParagraphEnd(&mut self, paragraph: Option<&ParagraphSimple>, layerTable: i32) -> Option<Box<dyn Entry>> {
      // java IOAssert.assertNotNull("paragraph should not be null", paragraph);
      let paragraph = paragraph.expect("paragraph should not be null");
      // java char endMark = paragraph.getEndMark();
      let endMark = paragraph.getEndMark();
      // java MutablePropertySet dstParaProps = paragraph.getParaProp();
      let mut dstParaProps = match paragraph.getParaProp() {
        Some(p) => MutablePropertySet::new_from_mutable(p),
        None => MutablePropertySet::new(),
      };
      // java MutablePropertySet dstRunProps = paragraph.getRunProp();
      let dstRunProps = paragraph.getRunProp();
      // java if (null == dstParaProps)
      // rust: no-stmt
      let doc_ptr = self.mDocument.expect("mDocument");
      let inserted_cp = {
      let ctx = self.mCurrentParagraph.as_mut().expect("mCurrentParagraph");
      // java if (null != this.mCurrentParagraph.mParaId)
      if let Some(para_id) = ctx.mParaId.clone() {
        // java dstParaProps.setValue(282, this.mCurrentParagraph.mParaId);
        dstParaProps.setString(282, para_id);
        // java this.mDocument.getTextDocument().getKParagraphsID().addParaphId(this.mCurrentParagraph.mParaId);
        // rust: no-stmt
      }
      // java ParaPropsConverterSimple.correctProp(dstParaProps);
      ParaPropsConverterSimple::correctProp(Some(&mut dstParaProps));
      // java if (layerTable >= 1)
      if layerTable >= 1 {
        // java ImporterHelper.amendTableProps(dstParaProps, layerTable);
        ImporterHelper::amendTableProps(&mut dstParaProps, layerTable);
      }
      // java this.mCurrentParagraph.mEntry = CoreHelper.appendParagraph(this.mDocument, endMark, this.mCurrentParagraph.mPositionInserted, null == dstRunProps ? PropertySet.empty : dstRunProps.freeze(), null == dstParaProps ? PropertySet.empty : dstParaProps.freeze());
      let run_ps = match dstRunProps {
        Some(mut p) => p.freeze(),
        None => MutablePropertySet::new().freeze(),
      };
      let para_ps = dstParaProps.freeze();
      ctx.mEntry = CoreHelper::appendParagraph__char_int_ps_ps(
        Some(unsafe { &mut *doc_ptr as &mut dyn Document }),
        endMark,
        ctx.mPositionInserted.unwrap_or(0),
        Some(&run_ps),
        Some(&para_ps),
      );
      // java android.util.Log.d("WPS_PIPE", "{\"phase\":\"parse\",\"para_end_mark\":\"" + (int)endMark + "\",\"doc_len\":" + this.mDocument.getLength() + "}");
      // rust: no-stmt
      // java if (null != this.mCurrentParagraph.mParaId)
      if ctx.mParaId.is_some() {
        // java this.mDocument.getTextDocument().getKParagraphsID().putId(this.mCurrentParagraph.mParaId, this.mDocument.getType(), this.mCurrentParagraph.mEntry);
        // rust: no-stmt
      }
      ctx.mPositionInserted.unwrap_or(0)
      };
      // java if ('\f' == endMark)
      if endMark == '\u{000C}' {
        // java PLCSection plcSection = this.mDocument.getPLCSection();
        // java plcSection.setLastSectionEndCP(this.mDocument.getLength());
        unsafe {
          let d = &mut *doc_ptr;
          let len = SubDocument::getLength(d);
          if let Some(sec) = d.mSections.as_mut() {
            sec.setLastSectionEndCP(len);
          }
        }
      }
      // java if (this.isContainNumbering(this.mCurrentParagraph.mEntry))
      let has_numbering = self
        .mCurrentParagraph
        .as_ref()
        .and_then(|c| c.mEntry.as_ref())
        .map(|e| self.isContainNumbering(e.as_ref()))
        .unwrap_or(false);
      if has_numbering {
        // java if (!paragraph.isParseNumbering() && !this.isXml07())
        if !paragraph.isParseNumbering() && !self.super_.isXml07() {
          // java paragraph.parseNumbering();
          // rust: no-stmt
        }
        // java else if (this.isXml07())
        else if self.super_.isXml07() {
          // java PartInfo info = this.xml07Context().getPartInfoByContentType("application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml");
          // rust: no-stmt
          // java if (null != info && !this.xml07Context().haveParsedNumbering())
          // rust: no-stmt
          // java try
          // rust: no-stmt
          // java this.xml07Context().getPartReader(5).read(this.xml07Context().getRawData(info.getPartName()));
          // rust: no-stmt
          // java catch (IOException var8)
          // rust: no-stmt
          // java IOLog.e(info.getPartName(), "xml 2007 part reader error");
          // rust: no-stmt
          // java this.xml07Context().setHaveParsedNumbering(true);
          // rust: no-stmt
        }
      }
      // java return this.mCurrentParagraph.mEntry;
      unsafe {
        (*doc_ptr).getPapxTable().and_then(|t| IPapxTable::seek(t, inserted_cp))
      }
    }

    // java public void importRunTextSimple(char[] buffer, int start, int len, PropertySet runProps)
    pub fn importRunTextSimple__chararr_int_int_propertyset(&mut self, buffer: &Vec<char>, start: i32, len: i32, runProps: &PropertySet) {
      // java if (len > 0)
      if len > 0 {
        // java CoreHelper.appendText(this.mDocument, buffer, start, start + len, runProps);
        let doc = self.mDocument.map(|p| unsafe { &mut *p as &mut dyn Document });
        CoreHelper::appendText__chararr_int_int_ps(doc, Some(buffer.as_slice()), start, start + len, Some(runProps));
      }
      // java if (null != this.mComplexFieldNodeBegin && KFieldType.FieldEmpty.getFlt() == this.mComplexFieldNodeBegin.mData)
      // rust: no-stmt
        // java this.mComplexFieldNodeBegin.mData = SubDocumentImporterUtils.toFieldType(new String(buffer, start, len));
      // rust: no-stmt
    }

    // java public void importRunTextSimple(String runText, PropertySet runProps)
    pub fn importRunTextSimple__string_propertyset(&mut self, runText: &str, runProps: &PropertySet) {
      // java IOAssert.assertNotNull("runText should not be null", runText);
      // rust: no-stmt
      // java IOAssert.assertNotNull("runProps should not be null", runProps);
      // rust: no-stmt
      // java if (runText.length() > 0)
      if (runText.len() as i32) > 0 {
        // java CoreHelper.appendText(this.mDocument, runText, runProps);
        let doc = self.mDocument.map(|p| unsafe { &mut *p as &mut dyn Document });
        CoreHelper::appendText__string_ps(doc, Some(runText), Some(runProps));
      }
      // java if (null != this.mComplexFieldNodeBegin && KFieldType.FieldEmpty.getFlt() == this.mComplexFieldNodeBegin.mData)
      // rust: no-stmt
        // java this.mComplexFieldNodeBegin.mData = SubDocumentImporterUtils.toFieldType(runText);
      // rust: no-stmt
    }

    // java public IPapxTable.Entry doImportTableRowEndSimple(MutablePropertySet tableProps, PropertySet runProp, char rowMark)
    pub fn doImportTableRowEndSimple(&mut self, tableProps: &mut MutablePropertySet, runProp: &PropertySet, rowMark: char) -> Option<Box<dyn Entry>> {
      // java return CoreHelper.appendParagraph(this.mDocument, rowMark, runProp, tableProps.freeze());
      let papx = tableProps.freeze();
      let doc = self.mDocument.map(|p| unsafe { &mut *p as &mut dyn Document });
      CoreHelper::appendParagraph__char_ps_ps(doc, rowMark, Some(runProp), Some(&papx))
    }

    // java public void importTableCellStartSimple()
    pub fn importTableCellStartSimple(&mut self) {
      // java this.resetParagraph();
      self.resetParagraph();
    }

    // java private void resetParagraph()
    pub fn resetParagraph(&mut self) {
      // java if (this.mCurrentParagraph != null && this.mCurrentParagraph.mEntry != null)
      if let Some(ctx) = self.mCurrentParagraph.as_mut() {
        // java this.mCurrentParagraph.mEntry = null;
        ctx.mEntry = None;
      }
    }

    // java public void importTableCellEndSimple(int layerTable, char cellMark)
    pub fn importTableCellEndSimple(&mut self, layerTable: i32, cellMark: char) {
      // java if (this.mCurrentParagraph != null)
      if self.mCurrentParagraph.is_some() {
        let doc_ptr = self.mDocument.expect("mDocument");
        {
          let ctx = self.mCurrentParagraph.as_mut().expect("mCurrentParagraph");
          // java if (this.mCurrentParagraph.mEntry == null)
          if ctx.mEntry.is_none() {
            // java this.mCurrentParagraph.mEntry = CoreHelper.appendParagraph(this.mDocument, (char)(layerTable > 1 ? '\r' : '\u0007'), PropertySet.empty, PropertySet.empty);
            let ch = if layerTable > 1 { '\r' } else { '\u{0007}' };
            let empty_ps = (*empty).clone();
            ctx.mEntry = CoreHelper::appendParagraph__char_ps_ps(
              Some(unsafe { &mut *doc_ptr as &mut dyn Document }),
              ch,
              Some(&empty_ps),
              Some(&empty_ps),
            );
          }
          // java MutablePropertySet paraProps = new MutablePropertySet(this.mCurrentParagraph.mEntry.getProperty());
          let mut paraProps: MutablePropertySet = match ctx.mEntry.as_ref().and_then(|e| Entry::getProperty(e.as_ref())) {
            Some(ps) => MutablePropertySet::new_from_ps(ps),
            None => MutablePropertySet::new(),
          };
          // java paraProps.setBoolean(223, true);
          paraProps.setBoolean(223, true);
          // java paraProps.setInt(224, layerTable);
          paraProps.setInt(224, layerTable);
          // java paraProps.setBoolean(226, true);
          paraProps.setBoolean(226, true);
          // java CoreHelper.setCellMark(cellMark, this.mDocument, this.mCurrentParagraph.mEntry);
          CoreHelper::setCellMark(
            cellMark,
            Some(unsafe { &mut *doc_ptr as &mut dyn Document }),
            ctx.mEntry.as_deref(),
          );
          // java this.mCurrentParagraph.mEntry.setProperty(paraProps.freeze());
          let frozen = paraProps.freeze();
          if let Some(entry) = ctx.mEntry.as_mut() {
            Entry::setProperty(entry.as_mut(), Some(&frozen));
          }
        }
        // java this.resetParagraph();
        self.resetParagraph();
      }
    }

    // java public void importFldCharSimple(Attributes fldChar, MutablePropertySet srcRunProps)
    pub fn importFldCharSimple(&mut self, fldChar: &dyn Attributes, srcRunProps: Option<&mut MutablePropertySet>) {
      // java IOAssert.assertNotNull("fldChar should not be null", fldChar);
      // Rust's non-optional reference carries Java's non-null precondition.
      // java if (null == srcRunProps)
      let empty_props = MutablePropertySet::new();
      let props = srcRunProps.as_deref().unwrap_or(&empty_props);
      // java Character fieldChar = ImporterHelper.getFldChar(fldChar);
      let fieldChar = crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ImporterHelper::ImporterHelper::getFldChar(fldChar);
      // java if (null != fieldChar)
      if let Some(fieldChar) = fieldChar {
        // java if (21 != fieldChar || 0 != this.mFieldsNotEnded)
        if fieldChar != '\u{0015}' || self.mFieldsNotEnded != 0 {
          // java int cpFieldChar = this.appendControlCharFieldSimple(fieldChar, srcRunProps, false);
          let cpFieldChar = self.appendControlCharFieldSimple__char_mutablepropertyset_boolean(fieldChar, props, false);
          // java PLCField.FieldNode node = this.appendToPLCField(fieldChar, cpFieldChar);
          let node = self.appendToPLCField(fieldChar, cpFieldChar);
          // java if (19 == fieldChar)
          if fieldChar == '\u{0013}' {
            // java this.mComplexFieldNodeBegin = node;
            self.mComplexFieldNodeBegin = Some(node);
          }
          // java else
          else {
            // java this.mComplexFieldNodeBegin = null;
            self.mComplexFieldNodeBegin = None;
          }
          // java this.updateShouldBeRecalculated(ConvertHelper.getBoolean(fldChar, "fldLock"));
          self.updateShouldBeRecalculated(crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::ConvertHelper::getBoolean(fldChar, "fldLock"));
        }
      }
    }

    // java public int appendControlCharFieldSimple(char fieldChar, MutablePropertySet dstRunProp)
    pub fn appendControlCharFieldSimple__char_mutablepropertyset(&mut self, fieldChar: char, dstRunProp: &MutablePropertySet) -> i32 {
      // java return this.appendControlCharFieldSimple(fieldChar, dstRunProp, false);
      self.appendControlCharFieldSimple__char_mutablepropertyset_boolean(fieldChar, dstRunProp, false)
    }

    // java public int appendControlCharFieldSimple(char fieldChar, MutablePropertySet dstRunProp, boolean isIgnoredField)
    pub fn appendControlCharFieldSimple__char_mutablepropertyset_boolean(&mut self, fieldChar: char, dstRunProp: &MutablePropertySet, isIgnoredField: bool) -> i32 {
      // java return this.appendControlCharFieldSimple(fieldChar, dstRunProp, isIgnoredField, (Shape)null);
      self.appendControlCharFieldSimple__char_mutablepropertyset_boolean_shape(fieldChar, Some(dstRunProp), isIgnoredField, None)
    }

    // java public int appendControlCharFieldSimple(char fieldChar, MutablePropertySet srcRunProps, boolean isIgnoredField, Shape shape)
    pub fn appendControlCharFieldSimple__char_mutablepropertyset_boolean_shape(&mut self, fieldChar: char, srcRunProps: Option<&MutablePropertySet>, isIgnoredField: bool, shape: Option<&Shape>) -> i32 {
      // java MutablePropertySet dstRunProp = null != srcRunProps ? new MutablePropertySet(srcRunProps) : new MutablePropertySet();
      let mut dstRunProp: MutablePropertySet = match srcRunProps {
        Some(p) => MutablePropertySet::new_from_mutable(p),
        None => MutablePropertySet::new(),
      };
      // java dstRunProp.setBoolean(28, true);
      dstRunProp.setBoolean(28, true);
      // java if (isIgnoredField)
      if isIgnoredField {
        // java dstRunProp.setBoolean(758, isIgnoredField);
        dstRunProp.setBoolean(758, isIgnoredField);
      }
      // java if (shape != null)
      if shape.is_some() {
        // java dstRunProp.setValue(56, shape);
        dstRunProp.setValue(56, Some(Arc::new(true)));
        // java dstRunProp.setBoolean(30, true);
        dstRunProp.setBoolean(30, true);
        // java dstRunProp.setBoolean(29, true);
        dstRunProp.setBoolean(29, true);
      }
      // java return CoreHelper.appendText(this.mDocument, fieldChar, dstRunProp.freeze());
      let frozen = dstRunProp.freeze();
      let doc = self.mDocument.map(|p| unsafe { &mut *p as &mut dyn Document });
      CoreHelper::appendText__char_ps(doc, fieldChar, Some(&frozen))
    }

    // java public void importBookmarkStart1(Attributes bookmark)
    pub fn importBookmarkStart1(&self, bookmark: &dyn Attributes) {
      /** DEFERRED_NON_FUNCTION
      // java BookmarkImporter bookmarkImporter = this.getBookmarkImporter();
      let mut bookmarkImporter: BookmarkImporter = self.getBookmarkImporter();
      // java bookmarkImporter.importBookmarkStart(bookmark);
      bookmarkImporter.importBookmarkStart(bookmark);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public BookmarkImporter getBookmarkImporter()
    pub fn getBookmarkImporter(&mut self) -> BookmarkImporter {
      /** DEFERRED_NON_FUNCTION
      // java if (null == this.mBookmarkImporter)
      if self.mBookmarkImporter.is_none() {
        // java this.mBookmarkImporter = new BookmarkImporter(this.mDocument);
        self.mBookmarkImporter = BookmarkImporter::new(self.mDocument);
      }
      // java return this.mBookmarkImporter;
      return self.mBookmarkImporter;
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public DocFieldImporter getDocumentFieldImporter()
    pub fn getDocumentFieldImporter(&mut self) -> DocFieldImporter {
      /** DEFERRED_NON_FUNCTION
      // java if (null == this.mDocFieldImporter)
      if self.mDocFieldImporter.is_none() {
        // java this.mDocFieldImporter = new DocFieldImporter(this.mDocument);
        self.mDocFieldImporter = DocFieldImporter::new(self.mDocument);
      }
      // java return this.mDocFieldImporter;
      return self.mDocFieldImporter;
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importDocFieldStart(Attributes attributes)
    pub fn importDocFieldStart(&self, attributes: &dyn Attributes) {
      /** DEFERRED_NON_FUNCTION
      // java DocFieldImporter docFieldImporter = this.getDocumentFieldImporter();
      let mut docFieldImporter: DocFieldImporter = self.getDocumentFieldImporter();
      // java docFieldImporter.importDocFieldStart(attributes);
      docFieldImporter.importDocFieldStart(attributes);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importDocumentFieldEnd(Attributes attributes)
    pub fn importDocumentFieldEnd(&self, attributes: &dyn Attributes) {
      /** DEFERRED_NON_FUNCTION
      // java DocFieldImporter docFieldImporter = this.getDocumentFieldImporter();
      let mut docFieldImporter: DocFieldImporter = self.getDocumentFieldImporter();
      // java docFieldImporter.importDocFieldEnd(attributes);
      docFieldImporter.importDocFieldEnd(attributes);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importBookmarkEndSimple(Attributes markupRange)
    pub fn importBookmarkEndSimple(&self, markupRange: &dyn Attributes) {
      /** DEFERRED_NON_FUNCTION
      // java BookmarkImporter bookmarkImporter = this.getBookmarkImporter();
      let mut bookmarkImporter: BookmarkImporter = self.getBookmarkImporter();
      // java bookmarkImporter.importBookmarkEnd(markupRange);
      bookmarkImporter.importBookmarkEnd(markupRange);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importCharacterRunSymbolSimple(Attributes attributes, MutablePropertySet runProps, RunPropsConverterSimple runPropsConverter)
    pub fn importCharacterRunSymbolSimple(&mut self, attributes: &dyn Attributes, runProps: Option<&MutablePropertySet>, runPropsConverter: &mut RunPropsConverterSimple) {
      // java IOAssert.assertNotNull("attributes should not be null", attributes);
      // rust: no-stmt
      // java MutablePropertySet dst = null;
      let mut dst = MutablePropertySet::new();
      // java if (null != runProps)
      if let Some(runProps) = runProps {
        // java dst = new MutablePropertySet(runProps);
        dst = MutablePropertySet::new_from_mutable(runProps);
      }
      // java else
      else {
        // java dst = new MutablePropertySet();
        dst = MutablePropertySet::new();
      }
      // java runPropsConverter.convertSymbolProp(attributes, dst);
      runPropsConverter.convertSymbolProp(attributes, &mut dst);
      // java this.importRunText("(", dst.freeze());
      let frozen = dst.freeze();
      self.importRunText("(", &frozen);
    }

    // java public void importFldSimpleBeginSimple(Attributes fldSimple)
    pub fn importFldSimpleBeginSimple(&mut self, fldSimple: &dyn Attributes) {
      /** DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("fldSimple should not be null", fldSimple);
      IOAssert::assertNotNull__string_object(&"fldSimple should not be null", Some(&fldSimple));
      // java String fieldCode = ConvertHelper.getString(fldSimple, "instr");
      let mut fieldCode: String = ConvertHelper::getString__attributes_string(&fldSimple, &"instr");
      // java if (null != fieldCode)
      if fieldCode.is_some() {
        // java this.importFldSimpleBeginOrHyperLinkBegin(fieldCode);
        self.importFldSimpleBeginOrHyperLinkBegin(&fieldCode);
        // java Boolean value = ConvertHelper.getBoolean(fldSimple, "fldLock");
        let mut value: bool = ConvertHelper::getBoolean(&fldSimple, &"fldLock");
        // java this.updateShouldBeRecalculated(value);
        self.updateShouldBeRecalculated(value);
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importRuby(RubyPr rubyPr, RubyContent rubyBase, RubyContent rt)
    pub fn importRuby(&mut self, rubyPr: &RubyPr, rubyBase: &RubyContent, rt: &RubyContent) {
      /** DEFERRED_NON_FUNCTION
      // java char fieldChar = '\u0013';
      let mut fieldChar: char = '\u{0013}';
      // java MutablePropertySet baseMutProp = null;
      let mut baseMutProp: MutablePropertySet = MutablePropertySet::default();
          // java baseMutProp = rubyBase.mProp.clone();
          baseMutProp = rubyBase.mProp.clone();
          // java CloneNotSupportedException var8
          todo!();
      // java int cpFieldChar = this.appendControlCharFieldSimple(fieldChar, baseMutProp);
      let mut cpFieldChar: i32 = self.appendControlCharFieldSimple__char_mutablepropertyset(fieldChar, &baseMutProp);
      // java PLCField.FieldNode node = this.appendToPLCField('\u0013', cpFieldChar);
      let mut node: FieldNode = self.appendToPLCField('\u{0013}', cpFieldChar);
      // java if (19 == fieldChar)
      if 19 == fieldChar {
        // java this.mComplexFieldNodeBegin = node;
        self.mComplexFieldNodeBegin = node;
      }
      // java else
      else {
        // java this.mComplexFieldNodeBegin = null;
        self.mComplexFieldNodeBegin = None;
      }
      // java this.appendEQField_Code(rubyPr, rubyBase, rt);
      self.appendEQField_Code(&rubyPr, &mut rubyBase, &rt);
      // java fieldChar = '\u0015';
      fieldChar = '\u{0015}';
      // java cpFieldChar = this.appendControlCharFieldSimple(fieldChar, baseMutProp);
      cpFieldChar = self.appendControlCharFieldSimple__char_mutablepropertyset(fieldChar, &baseMutProp);
      // java this.appendToPLCField('\u0015', cpFieldChar);
      self.appendToPLCField('\u{0015}', cpFieldChar);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java private void appendEQField_Code(RubyPr rubyPr, RubyContent rubyBase, RubyContent rt)
    pub fn appendEQField_Code(&mut self, rubyPr: &RubyPr, rubyBase: &mut RubyContent, rt: &RubyContent) {
      /** DEFERRED_NON_FUNCTION
      // java PropertySet baseProp = null != rubyBase.mProp ? rubyBase.mProp.freeze() : PropertySet.empty;
      let mut baseProp: PropertySet = (if rubyBase.mProp.is_some() { rubyBase.mProp.freeze() } else { PropertySet::empty() });
      // java StringBuilder sb = new StringBuilder();
      let mut sb: String = StringBuilder::new();
      // java sb.append("EQ ");
      sb.append("EQ ");
      // java this.appendRubyAlign(sb, rubyPr);
      self.appendRubyAlign(&mut sb, &rubyPr);
      // java this.appendRubyFont(sb, rt);
      self.appendRubyFont(&mut sb, &rt);
      // java this.appendRubyHps(sb, rubyPr);
      self.appendRubyHps(&mut sb, &rubyPr);
      // java this.appendRubyAlign2(sb, rubyPr);
      self.appendRubyAlign2(&mut sb, &rubyPr);
      // java this.appendRubyHpsRaise(sb, rubyPr);
      self.appendRubyHpsRaise(&mut sb, &rubyPr);
      // java this.importRunText(sb.toString(), baseProp);
      self.importRunText(&sb.toString(), &baseProp);
      // java this.appendRubyRt(rt, baseProp);
      self.appendRubyRt(&mut rt, &baseProp);
      // java this.appendRubyBase(rubyBase, baseProp);
      self.appendRubyBase(&rubyBase, &baseProp);
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java private void appendRubyBase(RubyContent rubyBase, PropertySet baseProp)
    pub fn appendRubyBase(&mut self, rubyBase: &RubyContent, baseProp: &PropertySet) {
      /** DEFERRED_NON_FUNCTION
      // java if (null != rubyBase.mTmpText)
      if rubyBase.mTmpText.is_some() {
        // java this.importRunText(rubyBase.mTmpText, baseProp);
        self.importRunText(&rubyBase.mTmpText, &baseProp);
        // java this.importRunText(")", baseProp);
        self.importRunText(&")", &baseProp);
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java private void appendRubyRt(RubyContent rt, PropertySet baseProp)
    pub fn appendRubyRt(&mut self, rt: &mut RubyContent, baseProp: &PropertySet) {
      /** DEFERRED_NON_FUNCTION
      // java if (null != rt.mTmpText)
      if rt.mTmpText.is_some() {
        // java PropertySet prop = null != rt.mProp ? rt.mProp.freeze() : PropertySet.empty;
        let mut prop: PropertySet = (if rt.mProp.is_some() { rt.mProp.freeze() } else { PropertySet::empty() });
        // java this.importRunText(rt.mTmpText, prop);
        self.importRunText(&rt.mTmpText, &prop);
        // java this.importRunText("),", baseProp);
        self.importRunText(&"),", &baseProp);
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java private void appendRubyAlign2(StringBuilder sb, RubyPr rubyPr)
    pub fn appendRubyAlign2(&mut self, sb: &mut str, rubyPr: &RubyPr) {
      /** DEFERRED_NON_FUNCTION
      // java if (null != rubyPr.mTmpRubyAlign)
      if rubyPr.mTmpRubyAlign.is_some() {
        // java sb.append("\\o");
        sb.append("\\o");
        // java switch (rubyPr.mTmpRubyAlign)
        match rubyPr.mTmpRubyAlign {
          // java case -1364013995:
          -1364013995 => {
            // java sb.append("\\ac");
            sb.append("\\ac");
          }
          // java case -1063614105: case -997367899:
          -1063614105 | -997367899 => {
            // java sb.append("\\ad");
            sb.append("\\ad");
          }
          // java case 3317767:
          3317767 => {
            // java sb.append("\\al");
            sb.append("\\al");
          }
          // java case 108511772:
          108511772 => {
            // java sb.append("\\ar");
            sb.append("\\ar");
          }
          // java default:
          _ => {
          }
        }
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java private void appendRubyHpsRaise(StringBuilder sb, RubyPr rubyPr)
    pub fn appendRubyHpsRaise(&mut self, sb: &mut str, rubyPr: &RubyPr) {
      /** DEFERRED_NON_FUNCTION
      // java if (null != rubyPr.mTmpHpsRaise)
      if rubyPr.mTmpHpsRaise.is_some() {
        // java sb.append("(\\s\\up ");
        sb.append("(\\s\\up ");
        // java int hpsRaise = rubyPr.mTmpHpsRaise / 2;
        let mut hpsRaise: i32 = rubyPr.mTmpHpsRaise / 2;
        // java sb.append(hpsRaise);
        sb.append(hpsRaise);
        // java sb.append("(");
        sb.append("(");
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java private void appendRubyHps(StringBuilder sb, RubyPr rubyPr)
    pub fn appendRubyHps(&mut self, sb: &mut str, rubyPr: &RubyPr) {
      /** DEFERRED_NON_FUNCTION
      // java if (null != rubyPr.mTmpHps)
      if rubyPr.mTmpHps.is_some() {
        // java sb.append("\\* hps");
        sb.append("\\* hps");
        // java sb.append(rubyPr.mTmpHps);
        sb.append(rubyPr.mTmpHps);
        // java sb.append(" ");
        sb.append(" ");
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java private void appendRubyFont(StringBuilder sb, RubyContent rt)
    pub fn appendRubyFont(&mut self, sb: &mut str, rt: &RubyContent) {
      /** DEFERRED_NON_FUNCTION
      // java if (null != rt.mProp)
      if rt.mProp.is_some() {
        // java String fontAsian = (String)rt.mProp.getValue(3);
        let mut fontAsian: String = rt.mProp.getValue(3).to_string();
        // java if (null != fontAsian)
        if fontAsian.is_some() {
          // java sb.append("\\* \"Font:");
          sb.append("\\* \"Font:");
          // java sb.append(fontAsian);
          sb.append(fontAsian);
          // java sb.append("\" ");
          sb.append("\" ");
        }
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java private void appendRubyAlign(StringBuilder sb, RubyPr rubyPr)
    pub fn appendRubyAlign(&mut self, sb: &mut str, rubyPr: &RubyPr) {
      /** DEFERRED_NON_FUNCTION
      // java if (null != rubyPr.mTmpRubyAlign)
      if rubyPr.mTmpRubyAlign.is_some() {
        // java switch (rubyPr.mTmpRubyAlign)
        match rubyPr.mTmpRubyAlign {
          // java case -1364013995:
          -1364013995 => {
            // java sb.append("\\* jc0 ");
            sb.append("\\* jc0 ");
          }
          // java case -1063614105:
          -1063614105 => {
            // java sb.append("\\* jc1 ");
            sb.append("\\* jc1 ");
          }
          // java case -997367899:
          -997367899 => {
            // java sb.append("\\* jc2 ");
            sb.append("\\* jc2 ");
          }
          // java case 3317767:
          3317767 => {
            // java sb.append("\\* jc3 ");
            sb.append("\\* jc3 ");
          }
          // java case 108511772:
          108511772 => {
            // java sb.append("\\* jc4 ");
            sb.append("\\* jc4 ");
          }
          // java default:
          _ => {
          }
        }
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public IVmlImporter getVmlImporter(IDocumentImporter documentImporter)
    pub fn getVmlImporter(&mut self, documentImporter: &dyn IDocumentImporter) -> Box<dyn IVmlImporter> {
      /** DEFERRED_NON_FUNCTION
      // java Shapes2Core4Docx shapes2Core = new Shapes2Core4Docx(this.mDocument, true, this.xml07Context());
      let mut shapes2Core: Shapes2Core4Docx = Shapes2Core4Docx::new(self.mDocument, true, self.super_.xml07Context());
      // java return new VmlImporter(documentImporter, shapes2Core, this.mDocument, this.mode(), this.xml07Context(), this.xml03Context());
      return VmlImporter::new(documentImporter, shapes2Core, self.mDocument, self.super_.mode(), self.super_.xml07Context(), self.super_.xml03Context());
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public IDmlImporter getDmlImporter(IDocumentImporter documentImporter, ThemeContext themeCtxt, Xml07Context xml07Cxt)
    pub fn cache_dml_importer(
      &mut self,
      document_importer: *const dyn io_wps_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter,
      document_importer_ref: Option<std::rc::Rc<dyn io_wps_io::cn::wps::io::xwpf::IDocumentImporter::IDocumentImporter>>,
      theme: std::rc::Rc<std::cell::RefCell<crate::cn::wps::moffice::writer::io::reader::dmlReader::context::ThemeContext::ThemeContext>>,
      xml07: Option<Xml07Context>,
      mode: i32,
    ) -> io_wps_io::cn::wps::io::xwpf::IDmlImporter::DmlImporterRef {
      // java if (null != this.mDmlImporter) return this.mDmlImporter;
      if let Some(existing) = self.mDmlImporter.clone() {
        return existing as io_wps_io::cn::wps::io::xwpf::IDmlImporter::DmlImporterRef;
      }
      // java this.mDmlImporter = new DmlImporter(documentImporter, shapes2Core, this.mDocument, themeCtxt, xml07Cxt, this.mode());
      let document = self.mDocument.expect("mDocument");
      let created = crate::cn::wps::moffice::writer::io::reader::dmlReader::DmlImporter::DmlImporter::create(
        document, document_importer, document_importer_ref, theme, xml07, mode,
      );
      self.mDmlImporter = Some(created.clone());
      created as io_wps_io::cn::wps::io::xwpf::IDmlImporter::DmlImporterRef
    }

    // Java: VmlImporter getVmlImporter(IDocumentImporter). One importer per sub-document.
    pub fn cache_vml_importer(
      &mut self,
      mode: i32,
    ) -> io_wps_io::cn::wps::io::xwpf::IVmlImporter::VmlImporterRef {
      if let Some(existing) = self.mVmlImporter.clone() {
        return existing as io_wps_io::cn::wps::io::xwpf::IVmlImporter::VmlImporterRef;
      }
      let document = self.mDocument.expect("mDocument");
      let created = crate::cn::wps::moffice::writer::io::reader::vmlReader::importer::vml::VmlImporter::VmlImporter::create(document, mode);
      self.mVmlImporter = Some(created.clone());
      created as io_wps_io::cn::wps::io::xwpf::IVmlImporter::VmlImporterRef
    }

}

impl std::ops::Deref for SubDocumentImporter {
    type Target = ImporterBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for SubDocumentImporter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
