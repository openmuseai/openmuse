// Java: public class KsoWMInfo extends HashMap<String, String>
use crate::cn::wps::show::data::custdatalst::CustDataLst::CustDataLst;

pub struct KsoWMInfo {
    // Java: extends HashMap<String, String>
    entries: Vec<(String, String)>,
}

impl KsoWMInfo {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    // Java: public static KsoWMInfo create(CustDataLst custDataLst)
    pub fn create(mut custDataLst: CustDataLst) -> Self {
        let mut this = Self::new();
        this.load(&mut custDataLst);
        this
    }

    // Java: public void load(CustDataLst custDataLst)
    pub fn load(&mut self, custDataLst: &mut CustDataLst) {
        // Java: this.clear();
        self.entries.clear();
        // Java: if (custDataLst.hasTags())
        if custDataLst.hasTags() {
            // Java: tagLst.read(tags); this.put(tag.name(), tag.val());
            let mut tags = Vec::new();
            custDataLst.tags().read(&mut tags);
            for tag in tags {
                self.put(tag.name(), tag.val());
            }
        }
    }

    // Java: HashMap.put
    pub fn put(&mut self, key: String, val: String) {
        if let Some(slot) = self.entries.iter_mut().find(|e| e.0 == key) {
            slot.1 = val;
        } else {
            self.entries.push((key, val));
        }
    }

    // Java: HashMap.get
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.iter().find(|e| e.0 == key).map(|e| e.1.as_str())
    }

    pub fn entry_pairs(&self) -> &[(String, String)] {
        &self.entries
    }
}
