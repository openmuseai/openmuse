// Java: import cn.wps.kfc.opcr.OpcRelation
use crate::kfc::opcr::OpcRelation::OpcRelation;

// java public static final OpcRelation APP = new OpcRelation("application/vnd.openxmlformats-officedocument.extended-properties+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties", "/docProps/app.xml");
pub static APP: std::sync::LazyLock<OpcRelation> =
    std::sync::LazyLock::new(|| OpcRelation::OpcRelation("application/vnd.openxmlformats-officedocument.extended-properties+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties", "/docProps/app.xml"));
// java public static final OpcRelation CORE = new OpcRelation("application/vnd.openxmlformats-package.core-properties+xml", "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties", "/docProps/core.xml");
pub static CORE: std::sync::LazyLock<OpcRelation> =
    std::sync::LazyLock::new(|| OpcRelation::OpcRelation("application/vnd.openxmlformats-package.core-properties+xml", "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties", "/docProps/core.xml"));
// java public static final OpcRelation CUSTOM = new OpcRelation("application/vnd.openxmlformats-officedocument.custom-properties+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties", "/docProps/custom.xml");
pub static CUSTOM: std::sync::LazyLock<OpcRelation> =
    std::sync::LazyLock::new(|| OpcRelation::OpcRelation("application/vnd.openxmlformats-officedocument.custom-properties+xml", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties", "/docProps/custom.xml"));
// java public static final OpcRelation RELS = new OpcRelation("application/vnd.openxmlformats-package.relationships+xml", "http://schemas.openxmlformats.org/package/2006/relationships", "#_rels/.rels");
pub static RELS: std::sync::LazyLock<OpcRelation> =
    std::sync::LazyLock::new(|| OpcRelation::OpcRelation("application/vnd.openxmlformats-package.relationships+xml", "http://schemas.openxmlformats.org/package/2006/relationships", "#_rels/.rels"));

// java public final class OxfRelations
#[derive(Default)]
pub struct OxfRelations {
}

// java public final class OxfRelations
impl OxfRelations {
    // java public  OxfRelations()
    pub fn OxfRelations() -> Self {
      // java super();
      Self {}
    }

}
