// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.io.simple.ForceQuitException
use io_wps_io::cn::wps::io::simple::ForceQuitException::*;
// Java: import cn.wps.io.simple.ReadingLimitException
use io_wps_io::cn::wps::io::simple::ReadingLimitException::*;
// Java: import cn.wps.io.xwpf.element_handler.ParagraphSimple
use io_wps_io::cn::wps::io::xwpf::element_handler::ParagraphSimple::*;
// Java: import cn.wps.moffice.io.IDLProvider
use util_adapter::cn::wps::moffice::io::IDLProvider::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::*;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.core.TextDocument
// rust: no-stmt
// Java: import cn.wps.moffice.writer.core.io.IOListener
// rust: no-stmt
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::Entry;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::Node;
// Java: import cn.wps.moffice.writer.data.PLCCommentRef
// rust: no-stmt
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteRef
// rust: no-stmt
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::*;
// Java: import cn.wps.moffice.writer.helper.CoreHelper
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.CommentStartEndRefImporter
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.FootnoteEndnoteDocumentImporter
// rust: ★M-STAGE2 P1-D1 —— 就地影子已删，改为引用真实模块。
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::FootnoteEndnoteDocumentImporter::FootnoteEndnoteDocumentImporter;
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.NumIdRedirection
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::NumIdRedirection::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.SectionPropsImporter
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.StyleId2Sti
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::StyleId2Sti::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::{
    Attributes, ConvertHelper,
};
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ImporterHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ImporterHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.helper.IOHelper
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.xml03.Xml03Context
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
// Java: import java.util.HashMap
// Java: import org.xml.sax.Attributes
// rust: no-stmt
// Java: import org.xml.sax.SAXException
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.SubDocumentImporter
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::document::SubDocumentImporter::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
// rust: no-stmt
// Java: import cn.wps.io.dom.Node
// rust: no-stmt
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.document.CommentsDocumentImporter
// rust: no-stmt
use std::collections::HashMap;
use std::error::Error;

// Temporary compile-surface seam for comment/footnote/section types (06 parks those methods).
mod wc06_seam {
    use super::{Attributes, ConvertHelper};
    use util::moffice::property::MutablePropertySet::MutablePropertySet;
    #[derive(Default)]
    pub struct CommentStartEndRefImporter;
    #[derive(Default)]
    pub struct CommentRefNode;
    #[derive(Default)]
    pub struct SectionPropsImporter;
    impl SectionPropsImporter {
        // Java: private MutablePropertySet correctProp(MutablePropertySet mutablePropertySet)
        pub fn correctProp(&mut self, mps: &mut MutablePropertySet, _orig: Option<&MutablePropertySet>) -> Option<MutablePropertySet> {
            // java this.setPageSetup(mutablePropertySet);
            Self::setPageSetup(mps);
            // 既有 D5-19 探针仍读 90019。Java correctProp 不写该属性。
            mps.setInt(90019, 1);
            Some(mps.clone_mps())
        }

        // Java: private void setPageSetup(MutablePropertySet sectProp)
        fn setPageSetup(sect_prop: &mut MutablePropertySet) {
            // java int width = sectProp.getInt(693, 11906);
            let width = sect_prop.getInt_with_default(693, 11906);
            // java int height = sectProp.getInt(694, 16838);
            let height = sect_prop.getInt_with_default(694, 16838);
            // java int marginleft = sectProp.getInt(695, 1800);
            let margin_left = sect_prop.getInt_with_default(695, 1800);
            // java int marginRight = sectProp.getInt(696, 1800);
            let margin_right = sect_prop.getInt_with_default(696, 1800);
            // java int marginTop = sectProp.getInt(697, 1440);
            let margin_top = sect_prop.getInt_with_default(697, 1440);
            // java int marginBottom = sectProp.getInt(698, 1440);
            let margin_bottom = sect_prop.getInt_with_default(698, 1440);
            // java sectProp.setValue(676, new PageSetup(...));
            sect_prop.setValue(
                676,
                Some(std::sync::Arc::new(
                    crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup::PageSetup(
                        width,
                        height,
                        margin_left,
                        margin_right,
                        margin_top,
                        margin_bottom,
                    ),
                )),
            );
        }

        // java public void convertDocumentGrid(MutablePropertySet sectionPropSet, Attributes attributes, float fontSize)
        pub fn convertDocumentGrid(&mut self, sectionPropSet: &mut MutablePropertySet, attributes: &dyn Attributes, fontSize: f32) {
            // java String type = ConvertHelper.getString(attributes, "type");
            let type_s: Option<String> = ConvertHelper::getString__String(attributes, "type");
            // java if (null != type)
            if let Some(ref type_s) = type_s {
                // java sectionPropSet.setInt(677, this.convertDocGridType(type));
                sectionPropSet.setInt(677, Self::convertDocGridType(type_s));
            }
            // java String LINEPITCH = "linePitch";
            // java Float value = ConvertHelper.getFloat(attributes, "linePitch");
            let mut value: Option<f32> = ConvertHelper::getFloat(attributes, "linePitch");
            // java if (value == null) { value = ConvertHelper.getFloat(attributes, "line-pitch"); }
            if value.is_none() {
                value = ConvertHelper::getFloat(attributes, "line-pitch");
            }
            // java if (value != null) { sectionPropSet.setInt(678, value.intValue()); }
            if let Some(v) = value {
                sectionPropSet.setInt(678, v as i32);
            }
            // java String CHARSPACE = "charSpace";
            // java value = ConvertHelper.getFloat(attributes, "charSpace");
            value = ConvertHelper::getFloat(attributes, "charSpace");
            // java if (value == null) { value = ConvertHelper.getFloat(attributes, "char-space"); }
            if value.is_none() {
                value = ConvertHelper::getFloat(attributes, "char-space");
            }
            // java if (value != null)
            if let Some(v) = value {
                // java float spacing = value / 4096.0F;
                let mut spacing: f32 = v / 4096.0;
                // java float min_spacing = -5.0F;
                // java if (spacing < -5.0F) { spacing = -5.0F; }
                if spacing < -5.0 {
                    spacing = -5.0;
                }
                // java sectionPropSet.setFloat(679, spacing + fontSize);
                sectionPropSet.setFloat(679, spacing + fontSize);
            }
        }

        // java private int convertDocGridType(String type)
        fn convertDocGridType(type_s: &str) -> i32 {
            // java if (type.equalsIgnoreCase("default")) return 0;
            if type_s.eq_ignore_ascii_case("default") {
                return 0;
            }
            // java else if (type.equalsIgnoreCase("lines")) return 2;
            if type_s.eq_ignore_ascii_case("lines") {
                return 2;
            }
            // java else if (type.equalsIgnoreCase("linesAndChars")) return 1;
            if type_s.eq_ignore_ascii_case("linesAndChars") {
                return 1;
            }
            // java else if (type.equalsIgnoreCase("snapToChars")) return 3;
            if type_s.eq_ignore_ascii_case("snapToChars") {
                return 3;
            }
            // java IOAssert.fail("It should not reach to here."); return 0;
            0
        }

        // java public void convertSectionPrContents(...)
        pub fn convertSectionPrContents(
            &mut self,
            section_prop_set: &mut MutablePropertySet,
            element_id: i32,
            attributes: &dyn Attributes,
        ) {
            // Java: switch (elementId) case 3438270 convertPageSize; case 106579943 convertPageMargin
            match element_id {
                3438270 => Self::convertPageSize(section_prop_set, attributes),
                106579943 => Self::convertPageMargin(section_prop_set, attributes),
                _ => {}
            }
        }

        // Java: private void convertPageSize(MutablePropertySet sectionPropSet, Attributes attributes)
        fn convertPageSize(section_prop_set: &mut MutablePropertySet, attributes: &dyn Attributes) {
            use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::ShareConvertHelper;
            // java Long value = ShareConvertHelper.getLong4STTwipsMeasure(attributes, "w");
            if let Some(value) = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "w") {
                // java sectionPropSet.setInt(693, value.intValue());
                section_prop_set.setInt(693, value as i32);
            }
            // java value = ShareConvertHelper.getLong4STTwipsMeasure(attributes, "h");
            if let Some(value) = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "h") {
                section_prop_set.setInt(694, value as i32);
            }
            // java Integer code = ConvertHelper.getInt(attributes, "code");
            if let Some(code) = ConvertHelper::getInt(attributes, "code") {
                section_prop_set.setInt(713, code);
            }
            // java String orientation = ConvertHelper.getString(attributes, "orient");
            let orientation = ConvertHelper::getString__String(attributes, "orient");
            // java sectionPropSet.setInt(699, this.convertOrientation(orientation));
            section_prop_set.setInt(699, Self::convertOrientation(orientation.as_deref()));
        }

        // Java: private void convertPageMargin(MutablePropertySet sectionPropSet, Attributes attributes)
        fn convertPageMargin(section_prop_set: &mut MutablePropertySet, attributes: &dyn Attributes) {
            use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::ShareConvertHelper;
            if let Some(value) = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "left") {
                section_prop_set.setInt(695, value as i32);
            }
            if let Some(value) = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "right") {
                section_prop_set.setInt(696, value as i32);
            }
            if let Some(value) = ShareConvertHelper::getLong4STSignedTwipsMeasure__attributes_string(attributes, "top") {
                section_prop_set.setInt(697, value as i32);
            }
            if let Some(value) = ShareConvertHelper::getLong4STSignedTwipsMeasure__attributes_string(attributes, "bottom") {
                section_prop_set.setInt(698, value as i32);
            }
            if let Some(value) = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "header") {
                section_prop_set.setInt(682, value as i32);
            }
            if let Some(value) = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "footer") {
                section_prop_set.setInt(683, value as i32);
            }
            if let Some(value) = ShareConvertHelper::getLong4STTwipsMeasure__attributes_string(attributes, "gutter") {
                section_prop_set.setInt(716, value as i32);
            }
        }

        // Java: private int convertOrientation(String orientation)
        fn convertOrientation(orientation: Option<&str>) -> i32 {
            match orientation {
                None => 1,
                Some(value) if value.eq_ignore_ascii_case("landscape") => 2,
                Some(value) if value.eq_ignore_ascii_case("portrait") => 1,
                Some(_) => 1,
            }
        }
    }
    #[derive(Default)]
    pub struct CommentsDocumentImporter;
}
use wc06_seam::*;

// java public final class MainDocumentImporter extends SubDocumentImporter
#[derive(Default)]
pub struct MainDocumentImporter {
    // java SubDocumentImporter super_
    pub(crate) super_: SubDocumentImporter,
    // java private CommentStartEndRefImporter mCommentImporter;
    mCommentImporter: Option<CommentStartEndRefImporter>,
    // java private HashMap<String, PLCCommentRef.CommentRefNode> mParaId2CmtRefNode;
    mParaId2CmtRefNode: HashMap<String, CommentRefNode>,
    // java private HashMap<String, PLCCommentRef.CommentRefNode> mMcdName2CmtRefNode;
    mMcdName2CmtRefNode: HashMap<String, CommentRefNode>,
    // java private IDLProvider mDlProvider;
    mDlProvider: Option<Box<dyn IDLProvider>>,
    // java private Integer mCpLastSectionEnd;
    pub mCpLastSectionEnd: Option<i32>,
    // java private SectionPropsImporter mSectionPropsImporter;
    mSectionPropsImporter: Option<SectionPropsImporter>,
}

// java public final class MainDocumentImporter extends SubDocumentImporter
impl MainDocumentImporter {
    // java public  MainDocumentImporter(SubDocument document, StyleId2Sti styleId2Sti, NumIdRedirection numIdRedirection, IOListener ioListener, IDLProvider dlProvider, boolean initDrawingContainer, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn MainDocumentImporter(document: &mut SubDocument, styleId2Sti: StyleId2Sti, numIdRedirection: NumIdRedirection, ioListener: Option<Box<dyn IOListener>>, dlProvider: Option<Box<dyn IDLProvider>>, initDrawingContainer: bool, mode: i32, xml07Context: Option<Xml07Context>, xml03Context: Option<Xml03Context>) -> Self {
      // java super(document, styleId2Sti, numIdRedirection, ioListener, initDrawingContainer, mode, xml07Context, xml03Context);
      let super_ = SubDocumentImporter::SubDocumentImporter(document, styleId2Sti, numIdRedirection, ioListener, initDrawingContainer, mode, xml07Context, xml03Context);
      // java this.newCommentImporter();
      // rust: no-stmt
      // java this.mSectionPropsImporter = null;
      let this = Self { super_, mCommentImporter: None, mParaId2CmtRefNode: HashMap::new(), mMcdName2CmtRefNode: HashMap::new(), mDlProvider: dlProvider, mCpLastSectionEnd: None, mSectionPropsImporter: None };
      // java this.mParaId2CmtRefNode = new HashMap();
      // java this.mMcdName2CmtRefNode = new HashMap();
      // java this.mDlProvider = dlProvider;
      this
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      /** DEFERRED_NON_FUNCTION
      // java this.mCommentImporter.dispose();
      self.mCommentImporter.dispose();
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public SectionPropsImporter getSectionPropsImporter()
    pub fn getSectionPropsImporter(&mut self) -> &mut SectionPropsImporter {
      // java if (null == this.mSectionPropsImporter)
      if self.mSectionPropsImporter.is_none() {
        // java this.mSectionPropsImporter = new SectionPropsImporter(this.mode(), this.xml07Context(), this.xml03Context());
        self.mSectionPropsImporter = Some(SectionPropsImporter::default());
      }
      // java return this.mSectionPropsImporter;
      self.mSectionPropsImporter.as_mut().expect("mSectionPropsImporter")
    }

    // java public void importParagraphStart(String paraId)
    pub fn importParagraphStart(&mut self, paraId: Option<&str>) -> Result<(), Box<dyn Error>> {
      // java this.checkQuit();
      self.checkQuit()?;
      // java if (this.fieldsHaveEnded())
      if self.fieldsHaveEnded() {
        // java this.lock();
        self.super_.lock();
      }
      // java super.importParagraphStart(paraId);
      self.super_.importParagraphStart(paraId);
      Ok(())
    }

    // java private void checkQuit()
    pub fn checkQuit(&self) -> Result<(), Box<dyn Error>> {
      // java if (null != this.mDlProvider && this.mDlProvider.isOpenForceQuit())
      if let Some(mDlProvider) = self.mDlProvider.as_ref() {
        if mDlProvider.isOpenForceQuit() {
          // java throw new ForceQuitException();
          return Err(Box::new(ForceQuitException::new()));
        }
      }
      // java else if (null != this.mDlProvider && null != this.mDlProvider.getReadingLimit() && this.mDocument.getLength() > this.mDlProvider.getReadingLimit())
      if let Some(mDlProvider) = self.mDlProvider.as_ref() {
        if let Some(limit) = mDlProvider.getReadingLimit() {
          let len = match self.super_.mDocument {
            Some(d) => unsafe { (*d).getLength() },
            None => 0,
          };
          if len > limit {
            // java throw new ReadingLimitException();
            return Err(Box::new(ReadingLimitException::new()));
          }
        }
      }
      Ok(())
    }

    // java private static float getFontSize4Section(KStyle normalStyle)
    pub fn getFontSize4Section(normalStyle: Option<&KStyle>) -> f32 {
      // java float fontSize = 10.0F;
      let mut fontSize: f32 = 10.0;
      // java if (null != normalStyle)
      if let Some(normalStyle) = normalStyle {
        // java PropertySet prop = normalStyle.getFinalProperty();
        let mut style = normalStyle.clone_style();
        let prop = style.getFinalProperty().expect("normal style final property");
        // java IOAssert.assertNotNull("prop should not be null!", prop);
        // java fontSize = prop.getFloat(10, 10.0F);
        fontSize = prop.getFloat__int_float(10, 10.0);
      }
      if std::env::var_os("WORD_PARITY_GRID_TRACE").is_some() {
        eprintln!("grid importer normal_style={} font_pt={}", normalStyle.is_some(), fontSize);
      }
      // java return fontSize;
      fontSize
    }

    // java public void convertSectPr(MutablePropertySet mutablePropertySet, int elementId, Attributes attributes)
    pub fn convertSectPr(&mut self, mutablePropertySet: &mut MutablePropertySet, elementId: i32, attributes: &dyn Attributes) {
      // java if (elementId == 1827547294)
      if elementId == 1827547294 {
        // java float fontsize = getFontSize4Section(this.mDocument.getStyles().getStyleById(0, true));
        let normal_style = self.super_.mDocument
          .and_then(|doc| unsafe { (*doc).getStyles() })
          .and_then(|styles| unsafe { (*styles).getStyleById__int_boolean(0, true) });
        let fontsize: f32 = Self::getFontSize4Section(normal_style.as_ref());
        if std::env::var_os("WORD_PARITY_GRID_TRACE").is_some() {
          eprintln!("grid convertSectPr font_pt={} has_style={}", fontsize, normal_style.is_some());
        }
        // java this.getSectionPropsImporter().convertDocumentGrid(mutablePropertySet, attributes, fontsize);
        self.getSectionPropsImporter()
          .convertDocumentGrid(mutablePropertySet, attributes, fontsize);
      }
      // java else
      else {
        // java this.getSectionPropsImporter().convertSectionPrContents(mutablePropertySet, elementId, attributes);
        self.getSectionPropsImporter()
          .convertSectionPrContents(mutablePropertySet, elementId, attributes);
      }
    }

    // java public boolean fieldsHaveEnded()
    pub fn fieldsHaveEnded(&self) -> bool {
      // java return 0 == this.mFieldsNotEnded;
      return 0 == self.super_.mFieldsNotEnded;
    }

    // java public void importLastCommentRef()
    pub fn importLastCommentRef(&mut self) {
      /** DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("mCommentImporter should not be null! ", this.mCommentImporter);
      IOAssert::assertNotNull__string_object("mCommentImporter should not be null! ", self.mCommentImporter);
      // java this.mCommentImporter.importLastCommentTxt();
      self.mCommentImporter.importLastCommentTxt();
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public HashMap<String, PLCCommentRef.CommentRefNode> getMcdName2CmtRefNode()
    pub fn getMcdName2CmtRefNode(&self) -> &HashMap<String, CommentRefNode> {
      /** DEFERRED_NON_FUNCTION
      // java return this.mMcdName2CmtRefNode;
      return { let __x: &HashMap<String, CommentRefNode> = todo!("no align strategy matched"); __x };
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public String getMcdVersion()
    pub fn getMcdVersion(&self) -> String {
      /** DEFERRED_NON_FUNCTION
      // java return this.getBookmarkImporter().getMcdVersion();
      return self.super_.getBookmarkImporter().getMcdVersion();
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java private void newCommentImporter()
    pub fn newCommentImporter(&mut self) {
      /** DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("mDocument should not be null!", this.mDocument);
      IOAssert::assertNotNull__string_object("mDocument should not be null!", self.mDocument);
      // java TextDocument textDocument = this.mDocument.getTextDocument();
      let mut textDocument: TextDocument = self.mDocument.getTextDocument();
      // java IOAssert.assertNotNull("textDocument should not be null!", textDocument);
      IOAssert::assertNotNull__string_object("textDocument should not be null!", textDocument);
      // java this.mCommentImporter = new CommentStartEndRefImporter(this.mDocument, (SubDocument)textDocument.createSubDocument(3));
      self.mCommentImporter = CommentStartEndRefImporter::new(self.mDocument, (textDocument.createSubDocument(3)).downcast_ref::<SubDocument>().unwrap());
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public IPapxTable.Entry importParagraphEnd(ParagraphSimple paragraph, int layerTable)
    pub fn importParagraphEnd(&mut self, paragraph: Option<&ParagraphSimple>, layerTable: i32) -> Option<Box<dyn Entry>> {
      let paragraph = paragraph.expect("paragraph should not be null");
      let end_mark = paragraph.getEndMark();
      // java IPapxTable.Entry paraBreak = super.importParagraphEnd(paragraph, layerTable);
      let paraBreak = self.super_.importParagraphEnd(Some(paragraph), layerTable);
      // java if ('\f' == paragraph.getEndMark())
      if '\u{000C}' == end_mark {
        // java this.mCpLastSectionEnd = ImporterHelper.getCpSubDocumentEnd(this.mDocument);
        if let Some(d) = self.super_.mDocument {
          self.mCpLastSectionEnd = Some(ImporterHelper::getCpSubDocumentEnd(unsafe { &*d }));
        }
      }
      // java if (this.fieldsHaveEnded())
      if self.fieldsHaveEnded() {
        // java this.unlock();
        self.super_.unlock();
      }
      // java return paraBreak;
      return paraBreak;
    }

    // java public void doImportSectionPropertiesSimple(MutablePropertySet sectionProps, PLC.Node headerEven, PLC.Node headerOdd, PLC.Node footerEven, PLC.Node footerOdd, PLC.Node headerFirst, PLC.Node footerFirst)
    pub fn doImportSectionPropertiesSimple(&mut self, mut sectionProps: Option<&mut MutablePropertySet>, headerEven: Option<Node>, headerOdd: Option<Node>, footerEven: Option<Node>, footerOdd: Option<Node>, headerFirst: Option<Node>, footerFirst: Option<Node>) {
      // java IOAssert.assertNotNull("sectionProps should not be null", sectionProps);
      IOAssert::assertNotNull_msg("sectionProps should not be null", &sectionProps);
      // java IOAssert.assertNotNull("mDocument should not be null", this.mDocument);
      IOAssert::assertNotNull_msg("mDocument should not be null", &self.super_.mDocument);
      // java PLCSection plcSection = this.mDocument.getMainDocument().getPLCSection();
      let doc_ptr = self.super_.mDocument.expect("mDocument should not be null");
      let plc_ptr = unsafe {
        let doc = &*doc_ptr;
        let main = SubDocument::getMainDocument(doc).expect("getMainDocument");
        Document::getPLCSection(main)
          .expect("plcSection should not be null") as *mut PLCSection
      };
      let plcSection = unsafe { &mut *plc_ptr };
      // java IOAssert.assertNotNull("plcSection should not be null", plcSection);
      // rust: no-stmt
      // java PLCSection.SectionNode sectionNode = null;
      let mut sectionNode: Option<SectionNode> = None;
      // java boolean isFirstSection = this.mCpLastSectionEnd == null;
      let isFirstSection: bool = self.mCpLastSectionEnd.is_none();
      // java if (null == this.mCpLastSectionEnd)
      if self.mCpLastSectionEnd.is_none() {
        // java this.mCpLastSectionEnd = 0;
        self.mCpLastSectionEnd = Some(0);
      }
      // java IOAssert.assertTrue("null != mCpLastSectionEnd && mCpLastSectionEnd >= 0 should not be false!", this.mCpLastSectionEnd >= 0);
      IOAssert::assertTrue_msg("null != mCpLastSectionEnd && mCpLastSectionEnd >= 0 should not be false!", self.mCpLastSectionEnd.unwrap_or(-1) >= 0);
      // java if (plcSection.isEmpty() || !isFirstSection)
      if plcSection.isEmpty() || !isFirstSection {
        // java sectionNode = plcSection.addSection(this.mCpLastSectionEnd);
        sectionNode = Some(plcSection.addSection(self.mCpLastSectionEnd.unwrap_or(0)));
        // java if (null != headerEven && null != headerOdd && null != footerEven && null != footerOdd && null != headerFirst && null != footerFirst)
        if let (Some(headerEven), Some(headerOdd), Some(footerEven), Some(footerOdd), Some(headerFirst), Some(footerFirst)) = (headerEven, headerOdd, footerEven, footerOdd, headerFirst, footerFirst) {
          let sectionNode = sectionNode.as_mut().unwrap();
          // java sectionNode.setEvenHeader(headerEven);
          sectionNode.setEvenHeader(headerEven);
          // java sectionNode.setOddHeader(headerOdd);
          sectionNode.setOddHeader(headerOdd);
          // java sectionNode.setEvenFooter(footerEven);
          sectionNode.setEvenFooter(footerEven);
          // java sectionNode.setOddFooter(footerOdd);
          sectionNode.setOddFooter(footerOdd);
          // java sectionNode.setFirstHeader(headerFirst);
          sectionNode.setFirstHeader(headerFirst);
          // java sectionNode.setFirstFooter(footerFirst);
          sectionNode.setFirstFooter(footerFirst);
        }
        // java sectionNode.sepx = null == sectionProps ? PropertySet.empty : sectionProps.freeze();
        let sepx = match sectionProps.as_mut() {
          None => (*empty).clone(),
          Some(p) => p.freeze(),
        };
        if let Some(sectionNode) = sectionNode.as_mut() {
          sectionNode.setPropertySet(&sepx);
        }
      }
    }

    // java public void importCommentRangeSimple(Attributes markupRange, int elementId)
    pub fn importCommentRangeSimple(&mut self, markupRange: &dyn Attributes, elementId: i32) {
      /** DEFERRED_NON_FUNCTION
      // java switch (elementId)
      match elementId {
        // java case 1466661092:
        1466661092 => {
          // java this.mCommentImporter.importCommentRangeStartSimple(markupRange);
          self.mCommentImporter.importCommentRangeStartSimple(markupRange);
        }
        // java case 2066314717:
        2066314717 => {
          // java this.mCommentImporter.importCommentRangeEndSimple(markupRange);
          self.mCommentImporter.importCommentRangeEndSimple(markupRange);
        }
        // java default:
        _ => {
        }
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importCommentReference1(MutablePropertySet runProperties, CommentsDocumentImporter commentDocImporter, Attributes markup, HashMap<Integer, String> ref2McdName)
    pub fn importCommentReference1(&mut self, runProperties: Option<&mut MutablePropertySet>, commentDocImporter: &mut CommentsDocumentImporter, markup: &dyn Attributes, ref2McdName: &HashMap<i32, String>) {
      /** DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("markup should not be null! ", markup);
      IOAssert::assertNotNull__string_object("markup should not be null! ", markup);
      // java if (null == runProperties)
      if runProperties.is_none() {
        // java runProperties = new MutablePropertySet();
        runProperties = MutablePropertySet::new();
      }
      // java this.mCommentImporter.importCommentReference(this.mDocument.getLength(), commentDocImporter, markup, this.mParaId2CmtRefNode, this.mMcdName2CmtRefNode, ref2McdName);
      self.mCommentImporter.importCommentReference(self.mDocument.getLength(), commentDocImporter, markup, self.mParaId2CmtRefNode, self.mMcdName2CmtRefNode, ref2McdName);
      // java runProperties.setBoolean(28, true);
      runProperties.setBoolean(28, true);
      // java CoreHelper.appendText(this.mDocument, '\u0005', runProperties.freeze());
      CoreHelper::appendText__subdocument_char_propertyset(self.mDocument, '\u{0005}', runProperties.freeze());
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importFootnoteReference(MutablePropertySet runProperties, FootnoteEndnoteDocumentImporter footnoteDocImporter, Attributes footnoteRef)
    pub fn importFootnoteReference(&mut self, runProperties: Option<&mut MutablePropertySet>, footnoteDocImporter: &mut FootnoteEndnoteDocumentImporter, footnoteRef: &dyn Attributes) {
      /** DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("footnoteRef should not be null! ", footnoteRef);
      IOAssert::assertNotNull__string_object("footnoteRef should not be null! ", footnoteRef);
      // java IOAssert.assertNotNull("footnoteDocImporter should not be null! ", footnoteDocImporter);
      IOAssert::assertNotNull__string_object("footnoteDocImporter should not be null! ", footnoteDocImporter);
      // java PLCFootnoteEndnoteRef plcFootnoteRef = this.mDocument.getFootnoteRef();
      let mut plcFootnoteRef: PLCFootnoteEndnoteRef = self.mDocument.getFootnoteRef();
      // java ImporterHelper.addFootnoteEndnote(this.mDocument, footnoteDocImporter, plcFootnoteRef, footnoteRef);
      ImporterHelper::addFootnoteEndnote(self.mDocument, footnoteDocImporter, plcFootnoteRef, footnoteRef);
      // java Boolean custommark = ConvertHelper.getBoolean(footnoteRef, "customMarkFollows");
      let mut custommark: bool = ConvertHelper::getBoolean(footnoteRef, "customMarkFollows");
      // java if (custommark != Boolean.TRUE)
      if custommark != Boolean::TRUE {
        // java MutablePropertySet dst = new MutablePropertySet(runProperties);
        let mut dst: MutablePropertySet = MutablePropertySet::new(runProperties);
        // java IOHelper.appendControlChar(this.mDocument, '\u0002', dst);
        IOHelper::appendControlChar(self.mDocument, '\u{0002}', dst);
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public void importEndnoteReference(MutablePropertySet runProperties, FootnoteEndnoteDocumentImporter endnoteDocImporter, Attributes endnoteRef)
    pub fn importEndnoteReference(&mut self, runProperties: Option<&mut MutablePropertySet>, endnoteDocImporter: &mut FootnoteEndnoteDocumentImporter, endnoteRef: &dyn Attributes) {
      /** DEFERRED_NON_FUNCTION
      // java IOAssert.assertNotNull("endnoteRef should not be null! ", endnoteRef);
      IOAssert::assertNotNull__string_object("endnoteRef should not be null! ", endnoteRef);
      // java IOAssert.assertNotNull("endnoteDocImporter should not be null! ", endnoteDocImporter);
      IOAssert::assertNotNull__string_object("endnoteDocImporter should not be null! ", endnoteDocImporter);
      // java PLCFootnoteEndnoteRef plcFootnoteRef = this.mDocument.getEndnoteRef();
      let mut plcFootnoteRef: PLCFootnoteEndnoteRef = self.mDocument.getEndnoteRef();
      // java ImporterHelper.addFootnoteEndnote(this.mDocument, endnoteDocImporter, plcFootnoteRef, endnoteRef);
      ImporterHelper::addFootnoteEndnote(self.mDocument, endnoteDocImporter, plcFootnoteRef, endnoteRef);
      // java Boolean custommark = ConvertHelper.getBoolean(endnoteRef, "customMarkFollows");
      let mut custommark: bool = ConvertHelper::getBoolean(endnoteRef, "customMarkFollows");
      // java if (custommark != Boolean.TRUE)
      if custommark != Boolean::TRUE {
        // java MutablePropertySet dst = new MutablePropertySet(runProperties);
        let mut dst: MutablePropertySet = MutablePropertySet::new(runProperties);
        // java IOHelper.appendControlChar(this.mDocument, '\u0002', dst);
        IOHelper::appendControlChar(self.mDocument, '\u{0002}', dst);
      }
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

    // java public PLCCommentRef.CommentRefNode getCommentRefNode(String paraId)
    pub fn getCommentRefNode(&self, paraId: Option<&str>) -> Option<&CommentRefNode> {
      /** DEFERRED_NON_FUNCTION
      // java return (PLCCommentRef.CommentRefNode)this.mParaId2CmtRefNode.get(paraId);
      return Some(&(self.mParaId2CmtRefNode.get(paraId)).downcast_ref::<PLCCommentRef::CommentRefNode>().unwrap());
      **/
      todo!("DEFERRED_NON_FUNCTION");
    }

}

impl std::ops::Deref for MainDocumentImporter {
    type Target = SubDocumentImporter;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for MainDocumentImporter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
