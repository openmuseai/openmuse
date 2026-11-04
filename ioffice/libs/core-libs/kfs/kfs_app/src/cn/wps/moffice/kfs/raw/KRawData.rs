#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]
// Java: import cn.wps.moffice.kfs.raw.EntryBase
use crate::kfs::raw::EntryBase::*;

// Java: public class KRawData 
#[derive(Default, Clone, Debug)]
pub struct KRawData {
    // Java field:    public EntryBase mEntry;
    pub mEntry: Option<EntryBase>,
    // Java field:    public int mEntrySourceId;
    pub mEntrySourceId: i32,
}

// Java: public class KRawData 
impl KRawData {
    // Java: public KRawData() 
    pub fn new() -> KRawData {
        Self { mEntry: None, mEntrySourceId: -1 }
    }

    // Java: public KRawData(EntryBase entry, int entrySourceId) 
    pub fn new__entrybase_int(entry: EntryBase, entrySourceId: i32) -> KRawData {
        Self { mEntry: Some(entry), mEntrySourceId: entrySourceId }
    }

    // Java: public String toString() 
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("[KRawData] mEntry: ");
        sb.push_str("[KRawData] mEntry: ");
        // Java: sb.append(this.mEntry);
        sb.push_str(&format!("{:?}", self.mEntry));
        // Java: sb.append(". mEntrySourceId: ");
        sb.push_str(". mEntrySourceId: ");
        // Java: sb.append(this.mEntrySourceId);
        sb.push_str(&self.mEntrySourceId.to_string());
        // Java: return sb.toString();
        sb
    }
}
