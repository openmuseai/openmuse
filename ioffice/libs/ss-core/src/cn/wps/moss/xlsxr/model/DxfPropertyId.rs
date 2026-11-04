use crate::java_runtime::Cell;
// java public class DxfPropertyId
pub struct DxfPropertyId {
    bordid: Cell<i32>,
    fontid: Cell<i32>,
    patid: Cell<i32>,
    numFmtId: Cell<i32>,
}
impl DxfPropertyId {
    pub fn new() -> Self {
        Self {
            bordid: Cell::new(-1),
            fontid: Cell::new(-1),
            patid: Cell::new(-1),
            numFmtId: Cell::new(-1),
        }
    }
    // java public int getFontid() { return this.fontid; }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/DxfPropertyId.java:9
    // java public int getFontid() {
    // java return this.fontid;
    // java }
    pub fn getFontid(&self) -> i32 {
        self.fontid.get()
    }
    // java public void setFontid(int fontid) { this.fontid = fontid; }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/DxfPropertyId.java:13
    // java public void setFontid(int fontid) {
    // java this.fontid = fontid;
    // java }
    pub fn setFontid(&self, value: i32) {
        self.fontid.set(value);
    }
    // java public int getBordid() { return this.bordid; }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/DxfPropertyId.java:17
    // java public int getBordid() {
    // java return this.bordid;
    // java }
    pub fn getBordid(&self) -> i32 {
        self.bordid.get()
    }
    // java public void setBordid(int bordid) { this.bordid = bordid; }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/DxfPropertyId.java:21
    // java public void setBordid(int bordid) {
    // java this.bordid = bordid;
    // java }
    pub fn setBordid(&self, value: i32) {
        self.bordid.set(value);
    }
    // java public int getPatid() { return this.patid; }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/DxfPropertyId.java:25
    // java public int getPatid() {
    // java return this.patid;
    // java }
    pub fn getPatid(&self) -> i32 {
        self.patid.get()
    }
    // java public void setPatid(int patid) { this.patid = patid; }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/DxfPropertyId.java:29
    // java public void setPatid(int patid) {
    // java this.patid = patid;
    // java }
    pub fn setPatid(&self, value: i32) {
        self.patid.set(value);
    }
    // java public int getNumFmtId() { return this.numFmtId; }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/DxfPropertyId.java:33
    // java public int getNumFmtId() {
    // java return this.numFmtId;
    // java }
    pub fn getNumFmtId(&self) -> i32 {
        self.numFmtId.get()
    }
    // java public void setNumFmtId(int numFmtId) { this.numFmtId = numFmtId; }

    // rust: source koffice/et/core/cn/wps/moss/xlsxr/model/DxfPropertyId.java:37
    // java public void setNumFmtId(int numFmtId) {
    // java this.numFmtId = numFmtId;
    // java }
    pub fn setNumFmtId(&self, value: i32) {
        self.numFmtId.set(value);
    }
}
