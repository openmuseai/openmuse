#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]
// Java: import cn.wps.moffice.kfs.raw.KRawData
use crate::kfs::raw::KRawData::*;
// Java: import cn.wps.moffice.kfs.raw.EntryBase
use crate::kfs::raw::EntryBase::*;

// Java: public abstract class KRawHostBase implements ZipOutputStream.IListener 
#[derive(Default)]
pub struct KRawHostBase {
    // Java field:    private KRawData mRawData = null;
    mRawData: Option<KRawData>,
    // Java field:    private KRawData mNextRawData = null;
    mNextRawData: Option<KRawData>,
}

// Java: public abstract class KRawHostBase implements ZipOutputStream.IListener 
impl KRawHostBase {
    // Java: protected KRawHostBase(KRawData rawData) 
    pub fn new(rawData: KRawData) -> KRawHostBase {
        Self { mRawData: Some(rawData), mNextRawData: None }
    }

    // Java: private void update(KRawData rawData) 
    fn update(&mut self, rawData: Option<KRawData>) {
        // Java: if (null != rawData && null != rawData.mEntry && -1 != rawData.mEntrySourceId)
        if let Some(mut rd) = rawData {
            if rd.mEntry.is_some() && rd.mEntrySourceId != -1 {
                // Java: this.mRawData = rawData;
                // Java: this.mRawData.mEntry = (EntryBase)this.mRawData.mEntry.clone();
                if let Some(ref entry) = rd.mEntry {
                    rd.mEntry = Some(entry.clone());
                }
                // Java: this.mRawData.mEntry.setMode(1);
                if let Some(ref mut entry) = rd.mEntry {
                    entry.setMode(1);
                }
                self.mRawData = Some(rd);
            }
        }
    }

    // Java: public KRawData getRawData() 
    pub fn getRawData(&self) -> &Option<KRawData> {
        // Java: return this.mRawData;
        &self.mRawData
    }

    // Java: public void clearRawData() 
    pub fn clearRawData(&mut self) {
        // Java: this.mRawData = null;
        self.mRawData = None;
    }

    // Java: public void resetRawData(int entrySourceId) 
    pub fn resetRawData(&mut self, entrySourceId: i32) {
        // Java: if (null == this.mNextRawData)
        if self.mNextRawData.is_none() {
            // Java: this.mNextRawData = new KRawData();
            self.mNextRawData = Some(KRawData::new());
        }
        // Java: this.mNextRawData.mEntrySourceId = entrySourceId;
        if let Some(ref mut next) = self.mNextRawData {
            next.mEntrySourceId = entrySourceId;
        }
        // Java: this.update(this.mNextRawData);
        let next = self.mNextRawData.take();
        self.update(next);
        // Java: this.mNextRawData = null;
        self.mNextRawData = None;
    }

    // Java: public void onUpdateEntry(EntryBase entry) 
    pub fn onUpdateEntry(&mut self, entry: EntryBase) {
        // Java: if (null == this.mNextRawData)
        if self.mNextRawData.is_none() {
            // Java: this.mNextRawData = new KRawData();
            self.mNextRawData = Some(KRawData::new());
        }
        // Java: this.mNextRawData.mEntry = entry;
        if let Some(ref mut next) = self.mNextRawData {
            next.mEntry = Some(entry);
        }
    }
}
