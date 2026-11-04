// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::{TextDocument};
// Java: import cn.wps.moffice.writer.data.autonum.AutoNumTable
use crate::cn::wps::moffice::writer::data::autonum::AutoNumTable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoData
use crate::cn::wps::moffice::writer::data::autonum::LfoData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstData
use crate::cn::wps::moffice::writer::data::autonum::LstData::*;
// Java: import cn.wps.moffice.writer.core.io.ICoreDataAcceptor
use crate::cn::wps::moffice::writer::core::io::ICoreDataAcceptor::*;

// java public class TextCoreDataAcceptor implements ICoreDataAcceptor
#[derive(Default)]
pub struct TextCoreDataAcceptor {
    // java TextDocument mDocument;
    pub(crate) mDocument: Option<TextDocument>,
    // java Transaction mTransaction;
    pub(crate) mTransaction: Option<Transaction>,
    // java KStyles mStyles;
    pub(crate) mStyles: Option<KStyles>,
    // java AutoNumTable mNumberings;
    pub(crate) mNumberings: Option<AutoNumTable>,
}

// java public interface ICoreDataAcceptor
impl ICoreDataAcceptor for TextCoreDataAcceptor {
    // java public void begin()
    fn begin(&mut self) {
    }

    // java public void end()
    fn end(&mut self) {
    }

    // java public TextDocument getMainDocument()
    fn getMainDocument(&self) -> TextDocument {
      // java return this.mDocument;
      return self.mDocument;
    }

    // java public void addSettings(PropertySet settings)
    fn addSettings(&mut self, settings: &PropertySet) {
      // java this.mDocument.setProperty(PropertySet.merge(this.mDocument.getProperty(), settings));
      self.mDocument.setProperty__propertyset(&PropertySet::merge(&self.mDocument.getProperty(), &settings));
    }

    // java public void addSectionProperty(PropertySet property)
    fn addSectionProperty(&mut self, property: &PropertySet) {
      // java this.mDocument.setProperty(PropertySet.merge(this.mDocument.getProperty(), property));
      self.mDocument.setProperty__propertyset(&PropertySet::merge(&self.mDocument.getProperty(), &property));
    }

    // java public KStyle addStyle(KStyle style)
    fn addStyle(&self, style: &mut KStyle) -> KStyle {
      // java style.setTransaction(this.mTransaction);
      style.setTransaction(self.mTransaction);
      // java this.mStyles.getStyles().add(style);
      self.mStyles.getStyles().add(style);
      // java return style;
      return style;
    }

    // java public void setRPrDefault(PropertySet rPr)
    fn setRPrDefault(&mut self, rPr: &PropertySet) {
      // java this.mStyles.setRPrDefault(rPr);
      self.mStyles.setRPrDefault(&rPr);
    }

    // java public void setPPrDefault(PropertySet pPr)
    fn setPPrDefault(&mut self, pPr: &PropertySet) {
      // java this.mStyles.setPPrDefault(pPr);
      self.mStyles.setPPrDefault(&pPr);
    }

    // java public LstData addList(LstData list)
    fn addList(&self, list: &LstData) -> LstData {
      // java this.mNumberings.getLstDataTable().add(list);
      self.mNumberings.getLstDataTable().add(list);
      // java return list;
      return list;
    }

    // java public LfoData addListOverride(int lfoId, LfoData listOverride)
    fn addListOverride(&self, lfoId: i32, listOverride: &LfoData) -> LfoData {
      // java this.mNumberings.getLfoDataTable().add(lfoId, listOverride);
      self.mNumberings.getLfoDataTable().add(lfoId, listOverride);
      // java return listOverride;
      return listOverride;
    }

}

// java public class TextCoreDataAcceptor implements ICoreDataAcceptor
impl TextCoreDataAcceptor {
    // java private  TextCoreDataAcceptor(TextDocument document)
    pub fn TextCoreDataAcceptor(document: &TextDocument) -> Self {
      // java this.mDocument = document;
      Self { mDocument: document, mTransaction: document.getTransaction(), mStyles: document.getStyles(), mNumberings: document.getAutoNumTable() }
      // java this.mTransaction = document.getTransaction();
      // java this.mStyles = document.getStyles();
      // java this.mNumberings = document.getAutoNumTable();
    }

    // java public static TextCoreDataAcceptor create(TextDocument document)
    pub fn create(document: Option<&TextDocument>) -> Option<TextCoreDataAcceptor> {
      // java return document == null ? null : new TextCoreDataAcceptor(document);
      return (if document.is_none() { None } else { Some(TextCoreDataAcceptor::new(document)) });
    }

}
