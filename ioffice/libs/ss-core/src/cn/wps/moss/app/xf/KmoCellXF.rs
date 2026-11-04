use super::{KmoXF::KmoXF, KmoXFData::KmoXFData};
use crate::cn::wps::moss::filefmt::biff8::record::ExtendedFormatRecord::ExtendedFormatRecord;
// java public final class KmoCellXF extends KmoXF
pub struct KmoCellXF {
    pub base: KmoXF,
}
impl KmoCellXF {
    // java KmoCellXF() {}

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoCellXF.java:6
    // java KmoCellXF() {
    // java }
    pub fn KmoCellXF() -> Self {
        Self {
            base: KmoXF::fromData(KmoXFData::new(), false),
        }
    }
    // java KmoCellXF(ExtendedFormatRecord xfRec) {super(xfRec);}

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoCellXF.java:9
    // java KmoCellXF(ExtendedFormatRecord xfRec) {
    // java super(xfRec);
    // java }
    pub fn fromRecord(rec: &ExtendedFormatRecord) -> Self {
        Self {
            base: KmoXF::fromRecord(rec),
        }
    }
    // java public static KmoCellXF copyOf(KmoCellXF xf)

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoCellXF.java:13
    // java public static KmoCellXF copyOf(KmoCellXF xf) {
    // java KmoCellXF ret = new KmoCellXF();
    // java ((KmoXFData)ret.rawData()).completeCopy((KmoXFData)xf.rawData());
    // java return ret;
    // java }
    pub fn copyOf(xf: &Self) -> Self {
        let ret = Self::KmoCellXF();
        ret.base.completeCopy(&xf.base);
        ret
    }
    // java public static KmoCellXF copyFromParentStyle(KmoStyleXF style)

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoCellXF.java:19
    // java public static KmoCellXF copyFromParentStyle(KmoStyleXF style) {
    // java KmoCellXF ret = new KmoCellXF();
    // java ((KmoXFData)ret.rawData()).copyFromParentXF(style.getXFData());
    // java return ret;
    // java }
    pub fn copyFromParentStyle(style: &super::KmoStyleXF::KmoStyleXF) -> Self {
        let ret = Self::KmoCellXF();
        ret.base.copyFromParent(&style.base);
        ret
    }
}
impl std::ops::Deref for KmoCellXF {
    type Target = KmoXF;
    fn deref(&self) -> &KmoXF {
        &self.base
    }
}
