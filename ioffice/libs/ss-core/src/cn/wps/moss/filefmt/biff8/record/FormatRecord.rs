// java public final class FormatRecord extends StandardRecord implements ReusableRecord
// rust: XLSX style value facade; BIFF stream reuse/serialization remains outside this read closure.
#[derive(Clone, Debug)]
pub struct FormatRecord {
    field_1_index_code: i32,
    field_3_hasMultibyte: bool,
    field_4_formatstring: Option<String>,
}
impl FormatRecord {
    // java public FormatRecord(int indexCode, String fs) { this.field_1_index_code = indexCode; this.field_4_formatstring = fs; this.field_3_hasMultibyte = StringUtil.hasMultibyte(fs); }
    pub fn FormatRecord(indexCode: i32, fs: Option<String>) -> Self {
        let mut rec = Self {
            field_1_index_code: indexCode,
            field_3_hasMultibyte: false,
            field_4_formatstring: None,
        };
        rec.setFormatString(fs);
        rec
    }
    // java public int getIndexCode() { return this.field_1_index_code; }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FormatRecord.java:29
    // java public int getIndexCode() {
    // java return this.field_1_index_code;
    // java }
    pub fn getIndexCode(&self) -> i32 {
        self.field_1_index_code
    }
    // java public void setIndexCode(int indexCode) { this.field_1_index_code = indexCode; }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FormatRecord.java:33
    // java public void setIndexCode(int indexCode) {
    // java this.field_1_index_code = indexCode;
    // java }
    pub fn setIndexCode(&mut self, indexCode: i32) {
        self.field_1_index_code = indexCode;
    }
    // java public String getFormatString() { return this.field_4_formatstring; }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FormatRecord.java:37
    // java public String getFormatString() {
    // java return this.field_4_formatstring;
    // java }
    pub fn getFormatString(&self) -> Option<String> {
        self.field_4_formatstring.clone()
    }
    // java public void setFormatString(String fs) { this.field_4_formatstring = fs; this.field_3_hasMultibyte = StringUtil.hasMultibyte(fs); }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FormatRecord.java:41
    // java public void setFormatString(String fs) {
    // java this.field_4_formatstring = fs;
    // java this.field_3_hasMultibyte = StringUtil.hasMultibyte(fs);
    // java }
    pub fn setFormatString(&mut self, fs: Option<String>) {
        self.field_3_hasMultibyte = fs
            .as_ref()
            .is_some_and(|s| s.encode_utf16().any(|c| c > 255));
        self.field_4_formatstring = fs;
    }
    // java protected int getDataSize() { String formatString = this.getFormatString(); return formatString == null ? 5 : 5 + formatString.length() * (this.field_3_hasMultibyte ? 2 : 1); }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FormatRecord.java:69
    // java protected int getDataSize() {
    // java String formatString = this.getFormatString();
    // java return formatString == null ? 5 : 5 + formatString.length() * (this.field_3_hasMultibyte ? 2 : 1);
    // java }
    pub fn getDataSize(&self) -> i32 {
        5 + self.field_4_formatstring.as_ref().map_or(0, |s| {
            s.encode_utf16().count() as i32 * if self.field_3_hasMultibyte { 2 } else { 1 }
        })
    }
    // java public short getSid() { return 1054; }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FormatRecord.java:74
    // java public short getSid() {
    // java return 1054;
    // java }
    pub fn getSid(&self) -> i16 {
        1054
    }
}
