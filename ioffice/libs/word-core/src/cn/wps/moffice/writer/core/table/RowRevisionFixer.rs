// Java: import cn.wps.kfc.api.procedure.IIntObjectProcedure
use util::kfc::api::procedure::IIntObjectProcedure::*;
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
use util::kfc::r#impl::map::WIntObjectHashMap::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.value.DefTable
use crate::cn::wps::moffice::writer::data::value::DefTable::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.moffice.writer.core.table.DefTableFixer
/* DEFERRED_NON_FUNCTION —— `DefTableFixer` 随 `data::table::TableTool` 延后。
 *   本文件对它的引用**仅出现在注释中**（`implements DefTableFixer.DefTableTransport`
 *   是 Java 的接口声明，Rust 侧未生成对应 `trait`/`impl`），故无真实调用点。 */
// Java: import java.lang.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;

// java public class RowRevisionFixer implements DefTableFixer.DefTableTransport
#[derive(Default)]
pub struct RowRevisionFixer {
    // java private Document mDocument;
    mDocument: Option<Box<dyn Document>>,
    // java private WIntObjectHashMap<PropertySet> mChpxes = new WIntObjectHashMap();
    mChpxes: Option<WIntObjectHashMap>,
    // java private WIntObjectHashMap<MutablePropertySet> mPapxes = new WIntObjectHashMap();
    mPapxes: Option<WIntObjectHashMap>,
}

// java public class RowRevisionFixer implements DefTableFixer.DefTableTransport
impl RowRevisionFixer {
    // java public  RowRevisionFixer(Document document)
    pub fn RowRevisionFixer(document: &dyn Document) -> Self {
      // java this.mDocument = document;
      Self { mDocument: document, mChpxes: Default::default(), mPapxes: Default::default() }
    }

    // java public void addChpx(int cp, PropertySet prop)
    pub fn addChpx(&mut self, cp: i32, prop: &PropertySet) {
      // java this.mChpxes.put(cp, prop);
      self.mChpxes.put(cp, prop);
    }

    // java public void addPapx(int cp, MutablePropertySet prop)
    pub fn addPapx(&mut self, cp: i32, prop: &MutablePropertySet) {
      // java this.mPapxes.put(cp, prop);
      self.mPapxes.put(cp, prop);
    }

    // java public void fixAll()
    pub fn fixAll(&mut self) {
      // java this.mChpxes.forEachEntry(new IIntObjectProcedure<PropertySet>() { public boolean execute(int cp, PropertySet propertySet) { RowRevisionFixer.this.mDocument.getChpxTable().seek(cp).setProperty(propertySet); return true; } });
      self.mChpxes.forEachEntry(IIntObjectProcedure::new());
      // java this.mPapxes.forEachEntry(new IIntObjectProcedure<MutablePropertySet>() { public boolean execute(int cp, MutablePropertySet propertySet) { RowRevisionFixer.this.mDocument.getPapxTable().seek(cp).setProperty(propertySet.freeze()); return true; } });
      self.mPapxes.forEachEntry(IIntObjectProcedure::new());
    }

    // java public DefTable getDefTable(Document document, int rowEndCp)
    pub fn getDefTable(&self, document: &dyn Document, rowEndCp: i32) -> Option<DefTable> {
      // java MutablePropertySet prop = (MutablePropertySet)this.mPapxes.get(rowEndCp);
      let mut prop: MutablePropertySet = (self.mPapxes.get(rowEndCp)).downcast_ref::<MutablePropertySet>().unwrap();
      // java return prop != null ? (DefTable)Tools.getPropFromRowProp(prop.get(), 306) : (DefTable)Tools.getPropFromRowProp(document.getPapxTable().seek(rowEndCp).getProperty(), 306);
      return (if prop.is_some() { Some((Tools::getPropFromRowProp(prop.get(), 306)).downcast_ref::<DefTable>().unwrap()) } else { Some((Tools::getPropFromRowProp(document.getPapxTable().seek(rowEndCp).getProperty(), 306)).downcast_ref::<DefTable>().unwrap()) });
    }

    // java public void setDefTable(Document document, int rowEndCp, DefTable defTable)
    pub fn setDefTable(&mut self, document: &dyn Document, rowEndCp: i32, defTable: &DefTable) {
      // java MutablePropertySet newProp = (MutablePropertySet)this.mPapxes.get(rowEndCp);
      let mut newProp: MutablePropertySet = (self.mPapxes.get(rowEndCp)).downcast_ref::<MutablePropertySet>().unwrap();
      // java if (newProp == null)
      if newProp.is_none() {
        // java newProp = new MutablePropertySet(document.getPapxTable().seek(rowEndCp).getProperty());
        newProp = MutablePropertySet::new(document.getPapxTable().seek(rowEndCp).getProperty());
        // java this.mPapxes.put(rowEndCp, newProp);
        self.mPapxes.put(rowEndCp, newProp);
      }
      // java Revision revision = (Revision)newProp.getValue(363);
      let mut revision: Revision = (newProp.getValue(363)).downcast_ref::<Revision>().unwrap();
      // java if (revision != null)
      if revision.is_some() {
            // java Revision newRevision = revision.clone();
            let mut newRevision: Revision = revision.clone();
            // java MutablePropertySet newRevisionProp = new MutablePropertySet(revision.getProperty());
            let mut newRevisionProp: MutablePropertySet = MutablePropertySet::new(revision.getProperty());
            // java newRevisionProp.setValue(306, defTable);
            newRevisionProp.setValue(306, defTable);
            // java newRevision.setProperty(newRevisionProp.freeze());
            newRevision.setProperty(newRevisionProp.freeze());
            // java newProp.setValue(363, newRevision);
            newProp.setValue(363, newRevision);
            // java CloneNotSupportedException var8
            todo!();
      }
      // java else
      else {
        // java newProp.setValue(306, defTable);
        newProp.setValue(306, defTable);
      }
    }

}
