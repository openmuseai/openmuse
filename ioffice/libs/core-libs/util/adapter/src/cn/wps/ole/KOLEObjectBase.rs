// Java: public class KOLEObjectBase
pub struct KOLEObjectBase {
    // Java field: KOLEDocument mDocument
    pub(crate) mDocument: Option<Box<dyn std::any::Any>>,
    // Java field: int SECTOR_SIZE
    pub(crate) SECTOR_SIZE: i32,
    // Java field: int SECTOR_SHIFT
    pub(crate) SECTOR_SHIFT: i32,
    // Java field: int SHORT_SECTOR_SIZE
    pub(crate) SHORT_SECTOR_SIZE: i32,
    // Java field: int SHORT_SECTOR_SHIFT
    pub(crate) SHORT_SECTOR_SHIFT: i32,
    // Java field: int SECTID_COUNT_SHIFT
    pub(crate) SECTID_COUNT_SHIFT: i32,
    // Java field: int SECTID_COUNT
    pub(crate) SECTID_COUNT: i32,
}

// Java: public class KOLEObjectBase
impl KOLEObjectBase {
    // Java: KOLEObjectBase()
    pub fn new() -> Self {
      // Java: super();
      KOLEObjectBase {
          mDocument: None,
          SECTOR_SIZE: 0,
          SECTOR_SHIFT: 0,
          SHORT_SECTOR_SIZE: 0,
          SHORT_SECTOR_SHIFT: 0,
          SECTID_COUNT_SHIFT: 0,
          SECTID_COUNT: 0,
      }
    }

    // Java: void initBase(KOLEDocument document)
    pub fn initBase(&mut self, _document: Box<dyn std::any::Any>) {
      // Java: assert document != null;

      // Java: this.mDocument = document;

      // Java: this.SECTOR_SHIFT = document.getSectorShift();

      // Java: this.SECTOR_SIZE = 1 << this.SECTOR_SHIFT;

      // Java: this.SHORT_SECTOR_SHIFT = document.getShortSectorShift();

      // Java: this.SHORT_SECTOR_SIZE = 1 << this.SHORT_SECTOR_SHIFT;

      // Java: this.SECTID_COUNT_SHIFT = this.SECTOR_SHIFT - 2;

      // Java: this.SECTID_COUNT = 1 << this.SECTID_COUNT_SHIFT;

    }

    // Java: KOLEDocument getDocument()
    pub fn getDocument(&self) -> Option<&Box<dyn std::any::Any>> {
      // Java: return this.mDocument;

      self.mDocument.as_ref()
    }

    // Java: int SAT_INDEX(int sector)
    pub fn SAT_INDEX(&self, _sector: i32) -> i32 {
      // Java: return sector >> this.SECTID_COUNT_SHIFT;

      _sector >> self.SECTID_COUNT_SHIFT
    }

    // Java: int SAT_OFFSET(int sector)
    pub fn SAT_OFFSET(&self, _sector: i32) -> i32 {
      // Java: return sector & this.SECTID_COUNT - 1;

      _sector & (self.SECTID_COUNT - 1)
    }

    // Java: static void setEntry(OLEEntry entry, KDirEntry pEntry)
    pub fn setEntry(_entry: &mut dyn std::any::Any, _pEntry: &dyn std::any::Any) {
      // Java: entry.szName = pEntry.getName();

      // Java: entry.size = pEntry.getSize();

      // Java: entry.type = pEntry.getType();

    }

}
