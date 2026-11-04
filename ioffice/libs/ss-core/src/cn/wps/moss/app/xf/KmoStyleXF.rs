use super::{KmoXF::KmoXF, KmoXFData::KmoXFData};
use crate::cn::wps::moss::filefmt::biff8::record::ExtendedFormatRecord::ExtendedFormatRecord;
// java public final class KmoStyleXF extends KmoXF
pub struct KmoStyleXF {
    pub base: KmoXF,
}
impl KmoStyleXF {
    // java KmoStyleXF() {}

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoStyleXF.java:6
    // java KmoStyleXF() {
    // java }
    pub fn KmoStyleXF() -> Self {
        Self {
            base: KmoXF::fromData(KmoXFData::new(), true),
        }
    }
    // java KmoStyleXF(ExtendedFormatRecord xfRec) {super(xfRec);}

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoStyleXF.java:9
    // java KmoStyleXF(ExtendedFormatRecord xfRec) {
    // java super(xfRec);
    // java }
    pub fn fromRecord(rec: &ExtendedFormatRecord) -> Self {
        Self {
            base: KmoXF::fromRecord(rec),
        }
    }
    // java public static KmoStyleXF copyOf(KmoStyleXF xf)

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoStyleXF.java:17
    // java public static KmoStyleXF copyOf(KmoStyleXF xf) {
    // java KmoStyleXF ret = new KmoStyleXF();
    // java ret.completeCopy(xf);
    // java return ret;
    // java }
    pub fn copyOf(xf: &Self) -> Self {
        let ret = Self::KmoStyleXF();
        ret.base.completeCopy(&xf.base);
        ret
    }
}
impl std::ops::Deref for KmoStyleXF {
    type Target = KmoXF;
    fn deref(&self) -> &KmoXF {
        &self.base
    }
}
