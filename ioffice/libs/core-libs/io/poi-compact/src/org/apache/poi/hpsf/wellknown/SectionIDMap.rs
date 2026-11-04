// Java: import java.nio.charset.StandardCharsets
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;
use super::PropertyIDMap::PropertyIDMap;

// Java field: byte[] SUMMARY_INFORMATION_ID
pub static SUMMARY_INFORMATION_ID: &[u8] = &[
    0xF2u8 as u8, 0x9Fu8 as u8, 0x85u8 as u8, 0xE0u8 as u8,
    0x4Fu8, 0xF9u8 as u8, 0x10u8, 0x68u8,
    0xABu8 as u8, 0x91u8 as u8, 0x08u8, 0x00u8,
    0x2Bu8, 0x27u8, 0xB3u8 as u8, 0xD9u8 as u8,
];

// Java field: byte[][] DOCUMENT_SUMMARY_INFORMATION_ID
pub static DOCUMENT_SUMMARY_INFORMATION_ID: &[&[u8]] = &[
    &[
        0xD5u8 as u8, 0xCDu8 as u8, 0xD5u8 as u8, 0x02u8,
        0x2Eu8, 0x9Cu8 as u8, 0x10u8, 0x1Bu8,
        0x93u8 as u8, 0x97u8 as u8, 0x08u8, 0x00u8,
        0x2Bu8, 0x2Cu8, 0xF9u8 as u8, 0xAEu8 as u8,
    ],
    &[
        0xD5u8 as u8, 0xCDu8 as u8, 0xD5u8 as u8, 0x05u8,
        0x2Eu8, 0x9Cu8 as u8, 0x10u8, 0x1Bu8,
        0x93u8 as u8, 0x97u8 as u8, 0x08u8, 0x00u8,
        0x2Bu8, 0x2Cu8, 0xF9u8 as u8, 0xAEu8 as u8,
    ],
];

// Java field: String UNDEFINED
pub const UNDEFINED: &str = "[undefined]";

// Java field: SectionIDMap defaultMap
static DEFAULT_MAP: OnceLock<SectionIDMap> = OnceLock::new();

// Java: public class SectionIDMap extends HashMap
pub struct SectionIDMap {
    map: HashMap<String, &'static PropertyIDMap>,
}

// Java: public class SectionIDMap extends HashMap
impl SectionIDMap {
    // Java: SectionIDMap()
    pub fn new() -> Self {
        // Java: super();
        SectionIDMap {
            map: HashMap::new(),
        }
    }

    // Java: static SectionIDMap getInstance()
    pub fn getInstance() -> &'static SectionIDMap {
        // Java: if (defaultMap == null)
        if DEFAULT_MAP.get().is_none()
        {
            // Java: SectionIDMap m = new SectionIDMap();
            let mut m = SectionIDMap::new();
            // Java: m.put(SUMMARY_INFORMATION_ID, PropertyIDMap.getSummaryInformationProperties());
            m.put_entry(SUMMARY_INFORMATION_ID, PropertyIDMap::getSummaryInformationProperties());
            // Java: m.put(DOCUMENT_SUMMARY_INFORMATION_ID[0], PropertyIDMap.getDocumentSummaryInformationProperties());
            m.put_entry(DOCUMENT_SUMMARY_INFORMATION_ID[0], PropertyIDMap::getDocumentSummaryInformationProperties());
            // Java: defaultMap = m;
            let _ = DEFAULT_MAP.set(m);
        }
        // Java: return defaultMap;
        DEFAULT_MAP.get().unwrap()
    }

    // Java: static String getPIDString(byte[] sectionFormatID, long pid)
    pub fn getPIDString(sectionFormatID: &[u8], pid: i64) -> String {
        // Java: PropertyIDMap m = getInstance().get(sectionFormatID);
        let m = SectionIDMap::getInstance().get_entry(sectionFormatID);
        // Java: if (m == null)
        if m.is_none() {
            // Java: return "[undefined]";
            return "[undefined]".to_string();
        }
        // Java: else
        {
            // Java: String s = (String)m.get(pid);
            let s = m.unwrap().get(pid);
            // Java: return s == null ? "[undefined]" : s;
            match s {
                Some(val) => val.clone(),
                None => "[undefined]".to_string(),
            }
        }
    }

    // Java: PropertyIDMap get(byte[] sectionFormatID)
    pub fn get_entry(&self, sectionFormatID: &[u8]) -> Option<&&'static PropertyIDMap> {
        // Java: return (PropertyIDMap)super.get(new String(sectionFormatID, StandardCharsets.UTF_8));
        let key = String::from_utf8_lossy(sectionFormatID).to_string();
        self.map.get(&key)
    }

    // Java: Object get(Object sectionFormatID)
    pub fn get_object(&self, section_format_id: &dyn std::any::Any) -> Option<&&'static PropertyIDMap> {
        // Java: return this.get((byte[])sectionFormatID);
        None
    }

    // Java: Object put(byte[] sectionFormatID, PropertyIDMap propertyIDMap)
    pub fn put_entry(&mut self, sectionFormatID: &[u8], propertyIDMap: &'static PropertyIDMap) {
        // Java: return super.put(new String(sectionFormatID, StandardCharsets.UTF_8), propertyIDMap);
        let key = String::from_utf8_lossy(sectionFormatID).to_string();
        self.map.insert(key, propertyIDMap);
    }

    // Java: Object put(Object key, Object value)
    pub fn put_object(&mut self, _key: &dyn std::any::Any, _value: &dyn std::any::Any) {
        // Java: return this.put((byte[])key, (PropertyIDMap)value);
    }
}
