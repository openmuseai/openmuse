// java public final class KmoFormat
#[derive(Clone, Debug, PartialEq)]
pub struct KmoFormat {
    field_1_index_code: i32,
    field_4_formatstring: Option<String>,
}
impl KmoFormat {
    // java KmoFormat(int indexCode, String fs) {this.field_1_index_code=indexCode; this.field_4_formatstring=fs;}
    pub fn KmoFormat(indexCode: i32, fs: impl Into<Option<String>>) -> Self {
        Self {
            field_1_index_code: indexCode,
            field_4_formatstring: fs.into(),
        }
    }
    // java public int getIndexCode() {return this.field_1_index_code;}

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoFormat.java:21
    // java public int getIndexCode() {
    // java return this.field_1_index_code;
    // java }
    pub fn getIndexCode(&self) -> i32 {
        self.field_1_index_code
    }
    // java public String getFormatString() {return this.field_4_formatstring;}

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoFormat.java:25
    // java public String getFormatString() {
    // java return this.field_4_formatstring;
    // java }
    pub fn getFormatString(&self) -> Option<String> {
        self.field_4_formatstring.clone()
    }
    // java KmoFormat(FormatRecord fmtRec) { this.field_1_index_code = fmtRec.getIndexCode(); this.field_4_formatstring = fmtRec.getFormatString(); }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoFormat.java:16
    // java KmoFormat(FormatRecord fmtRec) {
    // java this.field_1_index_code = fmtRec.getIndexCode();
    // java this.field_4_formatstring = fmtRec.getFormatString();
    // java }
    pub fn fromRecord(
        rec: &crate::cn::wps::moss::filefmt::biff8::record::FormatRecord::FormatRecord,
    ) -> Self {
        Self::KmoFormat(rec.getIndexCode(), rec.getFormatString())
    }
    // java public void copyToRecord(FormatRecord fmtRec) { fmtRec.setIndexCode(this.field_1_index_code); fmtRec.setFormatString(this.field_4_formatstring); }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoFormat.java:38
    // java public void copyToRecord(FormatRecord fmtRec) {
    // java fmtRec.setIndexCode(this.field_1_index_code);
    // java fmtRec.setFormatString(this.field_4_formatstring);
    // java }
    pub fn copyToRecord(
        &self,
        rec: &mut crate::cn::wps::moss::filefmt::biff8::record::FormatRecord::FormatRecord,
    ) {
        rec.setIndexCode(self.field_1_index_code);
        rec.setFormatString(self.field_4_formatstring.clone());
    }
}
