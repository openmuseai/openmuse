// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.CUpdateCommand
use crate::cn::wps::moffice::writer::core::CUpdateCommand::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.KSectionRevision
use crate::cn::wps::moffice::writer::core::KSectionRevision::*;
// Java: import cn.wps.moffice.writer.core.SelectionType
use crate::cn::wps::moffice::writer::core::SelectionType::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.data.DefaultValue
use crate::cn::wps::moffice::writer::data::DefaultValue::*;
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.value.DxaColumn
use crate::cn::wps::moffice::writer::data::value::DxaColumn::*;
// Java: import cn.wps.moffice.writer.data.value.PageSetup
use crate::cn::wps::moffice::writer::data::value::PageSetup::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.moffice.writer.core.sectionProp.ISectionProperty
use crate::cn::wps::moffice::writer::core::sectionProp::ISectionProperty::*;
// Java: import cn.wps.moffice.writer.core.sectionProp.KDxaCol
use crate::cn::wps::moffice::writer::core::sectionProp::KDxaCol::*;
// Java: import cn.wps.moffice.writer.core.sectionProp.KPageSetup
use crate::cn::wps::moffice::writer::core::sectionProp::KPageSetup::*;

// java public class KSectionProperty implements ISectionProperty
#[derive(Default)]
pub struct KSectionProperty {
    // java TextDocument textDocument;
    pub(crate) textDocument: Option<TextDocument>,
    // java KDxaCol dxaCol;
    pub(crate) dxaCol: Option<KDxaCol>,
    // java KPageSetup pageSetup;
    pub(crate) pageSetup: Option<KPageSetup>,
}

// java public interface ISectionProperty
impl ISectionProperty for KSectionProperty {
    // java public PageSetup getPageSetup(int cp)
    fn getPageSetup__int(&self, cp: i32) -> PageSetup {
      // java return this.getPageSetup().getPageSetup(cp);
      return self.getPageSetup().getPageSetup(cp);
    }

    // java public void setPageSetup(int cp, PageSetup pageSetup)
    fn setPageSetup(&mut self, cp: i32, pageSetup: &PageSetup) {
      // java this.getPageSetup().setPageSetup(cp, pageSetup);
      // PageSetup stub has no Clone yet; pass a placeholder until value type is rewritten.
      let _ = pageSetup;
      self.getPageSetup().setPageSetup(cp, PageSetup {});
    }

    // java public Integer getOrientation(int cp)
    fn getOrientation(&self, cp: i32) -> i32 {
      // java return this.getPageSetup().getOrientation(cp);
      return self.getPageSetup().getOrientation(cp);
    }

    // java public DxaColumn[] getDxaColumns(int cp)
    fn getDxaColumns(&self, cp: i32) -> Vec<DxaColumn> {
      // java return this.getDxaCol().getDxaColumns(cp);
      return self.getDxaCol().getDxaColumns(cp);
    }

    // java public float getMinFixedColumnWdith(int cp)
    fn getMinFixedColumnWdith(&self, cp: i32) -> f32 {
      // java return this.getDxaCol().getMinFixedColumnWdith(cp);
      return self.getDxaCol().getMinFixedColumnWdith(cp);
    }

    // java public int getTextFlow(int cp)
    fn getTextFlow(&self, cp: i32) -> i32 {
      // java return this.getMainDocument().getPLCSection().getSection(cp).getPropertySet().getInt(714, DefaultValue.section_TextFlow);
      return self.getMainDocument().getPLCSection().getSection(cp).getPropertySet().getInt(714, DefaultValue::section_TextFlow);
    }

    // java public boolean setFixedDxaColumns(SelectionType selType, KRange originRange, int column)
    fn setFixedDxaColumns__selectiontype_krange_int(&self, selType: &SelectionType, originRange: &KRange, column: i32) -> bool {
      // java return this.getDxaCol().setFixedDxaColumns(selType, originRange, column);
      return self.getDxaCol().setFixedDxaColumns(selType, originRange, column);
    }

    // java public boolean setFixedDxaColumns(SelectionType selType, KRange originRange, boolean fToLeft)
    fn setFixedDxaColumns__selectiontype_krange_boolean(&self, selType: &SelectionType, originRange: &KRange, fToLeft: bool) -> bool {
      // java return this.getDxaCol().setFixedDxaColumns(selType, originRange, fToLeft);
      return self.getDxaCol().setFixedDxaColumns(selType, originRange, fToLeft);
    }

    // java public void beginSectionChange()
    fn beginSectionChange(&mut self) {
      // java Document document = this.getMainDocument();
      let mut document: Box<dyn Document> = self.getMainDocument();
      // java long range = RANGEFactory.getRANGE(0, document.getLength());
      let mut range: i64 = RANGEFactory::getRANGE(0, document.getLength());
      // java CUpdateCommand.Callback callback = new CUpdateCommand.Callback() { public void beforeUpdate() { KSectionProperty.this.textDocument.beforeSectionChanged(); } public void afterUpdate() { KSectionProperty.this.textDocument.afterSectionChanged(); } };
      let mut callback: CUpdateCommand::Callback = Callback::new();
      // java CUpdateCommand.UpdateAfterUndo cmd = new CUpdateCommand.UpdateAfterUndo(document, range, callback);
      let mut cmd: CUpdateCommand::UpdateAfterUndo = UpdateAfterUndo::new(document, range, callback);
      // java this.textDocument.submit(cmd);
      self.textDocument.submit(&cmd);
    }

    // java public void endSectionChange()
    fn endSectionChange(&mut self) {
      // java Document document = this.getMainDocument();
      let mut document: Box<dyn Document> = self.getMainDocument();
      // java long range = RANGEFactory.getRANGE(0, document.getLength());
      let mut range: i64 = RANGEFactory::getRANGE(0, document.getLength());
      // java CUpdateCommand.Callback callback = new CUpdateCommand.Callback() { public void beforeUpdate() { KSectionProperty.this.textDocument.beforeSectionChanged(); } public void afterUpdate() { KSectionProperty.this.textDocument.afterSectionChanged(); } };
      let mut callback: CUpdateCommand::Callback = Callback::new();
      // java CUpdateCommand.UpdateAfterRedo cmd = new CUpdateCommand.UpdateAfterRedo(document, range, callback);
      let mut cmd: CUpdateCommand::UpdateAfterRedo = UpdateAfterRedo::new(document, range, callback);
      // java cmd.execute();
      cmd.execute();
      // java this.textDocument.submit(cmd);
      self.textDocument.submit(&cmd);
    }

}

// java public class KSectionProperty implements ISectionProperty
impl KSectionProperty {
    // java public  KSectionProperty(TextDocument textDocument)
    pub fn KSectionProperty(textDocument: &TextDocument) -> Self {
      // java this.textDocument = textDocument;
      Self { textDocument: textDocument, dxaCol: Default::default(), pageSetup: Default::default() }
    }

    // java public KDxaCol getDxaCol()
    pub fn getDxaCol(&mut self) -> KDxaCol {
      // java if (null == this.dxaCol)
      if self.dxaCol.is_none() {
        // java this.dxaCol = new KDxaCol(this);
        self.dxaCol = KDxaCol::new(self);
      }
      // java return this.dxaCol;
      return self.dxaCol;
    }

    // java public KPageSetup getPageSetup()
    pub fn getPageSetup(&mut self) -> KPageSetup {
      // java if (null == this.pageSetup)
      if self.pageSetup.is_none() {
        // java this.pageSetup = new KPageSetup(this);
        self.pageSetup = KPageSetup::new(self);
      }
      // java return this.pageSetup;
      return self.pageSetup;
    }

    // java public PropertySet getMergedSepx(PropertySet sepx)
    pub fn getMergedSepx(&self, sepx: &PropertySet) -> PropertySet {
      // java Revision revision = (Revision)sepx.getValue(727);
      let mut revision: Revision = (sepx.getValue(727)).downcast_ref::<Revision>().unwrap();
      // java if (revision != null && revision.getProperty() != null)
      if revision.is_some() && revision.getProperty().is_some() {
        // java MutablePropertySet m = new MutablePropertySet(revision.getProperty());
        let mut m: MutablePropertySet = MutablePropertySet::new(revision.getProperty());
        // java m.add(sepx);
        m.add(sepx);
        // java m.add(revision.getProperty());
        m.add(revision.getProperty());
        // java return m.freeze();
        return m.freeze();
      }
      // java else
      else {
        // java return sepx;
        return sepx;
      }
    }

    // java public void changeSepx(PLCSection.SectionNode node, PropertySet newProp)
    pub fn changeSepx(&mut self, node: &mut PLCSection::SectionNode, newProp: &PropertySet) {
      // java Revision revision = (Revision)node.getPropertySet().getValue(727);
      let mut revision: Revision = (node.getPropertySet().getValue(727)).downcast_ref::<Revision>().unwrap();
      // java KSectionRevision sectionRevision = new KSectionRevision(this.textDocument);
      let mut sectionRevision: KSectionRevision = KSectionRevision::new(self.textDocument);
      // java if (this.textDocument.getTrackChange())
      if self.textDocument.getTrackChange() {
        // java PropertySet revisionProp = sectionRevision.setSectionFormatRevision(node.sepx);
        let mut revisionProp: PropertySet = sectionRevision.setSectionFormatRevision(node.sepx);
        // java this.apply_section_revision(node, newProp, revisionProp);
        self.apply_section_revision(&mut node, &newProp, &revisionProp);
      }
      // java else if (revision != null)
      else if revision.is_some() {
        // java PropertySet revisionProp = sectionRevision.setModifySectionFormatRevision(revision, this.getTextDocument().getTrackChange());
        let mut revisionProp: PropertySet = sectionRevision.setModifySectionFormatRevision(revision, self.getTextDocument().getTrackChange());
        // java this.apply_section_revision(node, newProp, revisionProp);
        self.apply_section_revision(&mut node, &newProp, &revisionProp);
      }
      // java else
      else {
        // java MutablePropertySet newSepx = new MutablePropertySet(node.sepx);
        let mut newSepx: MutablePropertySet = MutablePropertySet::new(node.sepx);
        // java newSepx.add(newProp);
        newSepx.add(newProp);
        // java node.setPropertySet(newSepx.freeze());
        node.setPropertySet(newSepx.freeze());
      }
    }

    // java private void apply_section_revision(PLCSection.SectionNode node, PropertySet newProp, PropertySet revisionProp)
    pub fn apply_section_revision(&mut self, node: &mut PLCSection::SectionNode, newProp: &PropertySet, revisionProp: &PropertySet) {
      // java Revision revision = (Revision)revisionProp.getValue(727);
      let mut revision: Revision = (revisionProp.getValue(727)).downcast_ref::<Revision>().unwrap();
      // java MutablePropertySet sectionProp = new MutablePropertySet(node.getPropertySet());
      let mut sectionProp: MutablePropertySet = MutablePropertySet::new(node.getPropertySet());
      // java PropertySet changeFormat = PropertySet.merge(revision.getProperty(), newProp);
      let mut changeFormat: PropertySet = PropertySet::merge(&revision.getProperty(), &newProp);
      // java if (this.checkSectionChangeFormatValid(node, changeFormat))
      if self.checkSectionChangeFormatValid(&node, &changeFormat) {
        // java changeFormat = this.sectionFormatChange(node, changeFormat);
        changeFormat = self.sectionFormatChange(&node, &changeFormat);
        // java if (!changeFormat.isEmpty())
        if !changeFormat.is_empty() {
          // java revision.setProperty(changeFormat);
          revision.setProperty(changeFormat);
          // java sectionProp.add(revisionProp);
          sectionProp.add(revisionProp);
          // java node.setPropertySet(sectionProp.freeze());
          node.setPropertySet(sectionProp.freeze());
        }
        // java else
        else {
          // java KSectionRevision.clearProperty(sectionProp, revisionProp);
          KSectionRevision::clearProperty(&mut sectionProp, &revisionProp);
          // java node.setPropertySet(sectionProp.freeze());
          node.setPropertySet(sectionProp.freeze());
        }
      }
    }

    // java private boolean checkSectionChangeFormatValid(PLCSection.SectionNode node, PropertySet changeFormat)
    pub fn checkSectionChangeFormatValid(&self, node: &PLCSection::SectionNode, changeFormat: &PropertySet) -> bool {
      // java PropertySet previousFinalProp = this.getMergedSepx(node.getPropertySet());
      let mut previousFinalProp: PropertySet = self.getMergedSepx(&node.getPropertySet());
      // java PropertySet changeFormatFinalProp = this.getMergedSepx(changeFormat);
      let mut changeFormatFinalProp: PropertySet = self.getMergedSepx(&changeFormat);
      // java return !changeFormatFinalProp.equals(previousFinalProp);
      return !changeFormatFinalProp.equals(previousFinalProp);
    }

    // java private PropertySet sectionFormatChange(PLCSection.SectionNode node, PropertySet changeFormat)
    pub fn sectionFormatChange(&mut self, node: &PLCSection::SectionNode, changeFormat: &PropertySet) -> PropertySet {
      // java MutablePropertySet mutaChange = new MutablePropertySet(changeFormat);
      let mut mutaChange: MutablePropertySet = MutablePropertySet::new(changeFormat);
      // java mutaChange.differenceSet(PropertySet.merge(DefaultSectionProperties.getProperties(), node.getPropertySet()));
      mutaChange.differenceSet(PropertySet::merge(&DefaultSectionProperties::getProperties(), &node.getPropertySet()));
      // java return mutaChange.freeze();
      return mutaChange.freeze();
    }

    // java public Document getMainDocument()
    pub fn getMainDocument(&self) -> Box<dyn Document> {
      // java return this.textDocument.getMainDocument();
      return self.textDocument.getMainDocument();
    }

    // java public TextDocument getTextDocument()
    pub fn getTextDocument(&self) -> TextDocument {
      // java return this.textDocument;
      return self.textDocument;
    }

}
