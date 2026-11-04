// Java: import java.util.HashMap
// Java: import java.util.Map
use std::collections::HashMap;
use std::sync::{OnceLock, Mutex};
use super::XWPFRelationPurl::XWPFRelationPurl;
use super::XWPFRelation::XWPFRelation;

// Java field: Map<String, String> _table
static TABLE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn ensure_table() -> &'static Mutex<HashMap<String, String>> {
    TABLE.get_or_init(|| {
        let mut map = HashMap::new();
        let _ = XWPFRelation::chart();
        let purl_chart = XWPFRelationPurl::getInstance("http://purl.oclc.org/ooxml/officeDocument/relationships/chart");
        if let (Some(old), Some(new_rel)) = (purl_chart.getRelation(), XWPFRelation::chart().base.getRelation()) {
            map.insert(old.to_string(), new_rel.to_string());
        }
        let purl_ext = XWPFRelationPurl::getInstance("http://purl.oclc.org/ooxml/officeDocument/relationships/oleObject");
        if let Some(old) = purl_ext.getRelation() {
            map.insert(old.to_string(), "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject".to_string());
        }
        let purl_src = XWPFRelationPurl::getInstance("http://purl.oclc.org/ooxml/officeDocument/relationships/package");
        if let Some(old) = purl_src.getRelation() {
            map.insert(old.to_string(), "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package".to_string());
        }
        Mutex::new(map)
    })
}

// Java: public class XWPFRelationTool
pub struct XWPFRelationTool {
}

// Java: public class XWPFRelationTool
impl XWPFRelationTool {
    // Java: XWPFRelationTool()
    pub fn new() -> Self {
      // Java: super();
      XWPFRelationTool {}
    }

    // Java: static String getTransitionalRelationType(String oldType)
    pub fn getTransitionalRelationType(old_type: &str) -> Option<String> {
      // Java: return (String)_table.get(oldType);
      let table = ensure_table();
      let guard = table.lock().unwrap();
      guard.get(old_type).cloned()
    }

}
