// Java: import java.util.Collections
// Java: import java.util.HashMap
// Java: import java.util.Map
use std::collections::HashMap;
use std::sync::OnceLock;

// Java field: int PID_TITLE
pub const PID_TITLE: i32 = 2;
// Java field: int PID_SUBJECT
pub const PID_SUBJECT: i32 = 3;
// Java field: int PID_AUTHOR
pub const PID_AUTHOR: i32 = 4;
// Java field: int PID_KEYWORDS
pub const PID_KEYWORDS: i32 = 5;
// Java field: int PID_COMMENTS
pub const PID_COMMENTS: i32 = 6;
// Java field: int PID_TEMPLATE
pub const PID_TEMPLATE: i32 = 7;
// Java field: int PID_LASTAUTHOR
pub const PID_LASTAUTHOR: i32 = 8;
// Java field: int PID_REVNUMBER
pub const PID_REVNUMBER: i32 = 9;
// Java field: int PID_EDITTIME
pub const PID_EDITTIME: i32 = 10;
// Java field: int PID_LASTPRINTED
pub const PID_LASTPRINTED: i32 = 11;
// Java field: int PID_CREATE_DTM
pub const PID_CREATE_DTM: i32 = 12;
// Java field: int PID_LASTSAVE_DTM
pub const PID_LASTSAVE_DTM: i32 = 13;
// Java field: int PID_PAGECOUNT
pub const PID_PAGECOUNT: i32 = 14;
// Java field: int PID_WORDCOUNT
pub const PID_WORDCOUNT: i32 = 15;
// Java field: int PID_CHARCOUNT
pub const PID_CHARCOUNT: i32 = 16;
// Java field: int PID_THUMBNAIL
pub const PID_THUMBNAIL: i32 = 17;
// Java field: int PID_APPNAME
pub const PID_APPNAME: i32 = 18;
// Java field: int PID_SECURITY
pub const PID_SECURITY: i32 = 19;
// Java field: int PID_DICTIONARY
pub const PID_DICTIONARY: i32 = 0;
// Java field: int PID_CODEPAGE
pub const PID_CODEPAGE: i32 = 1;
// Java field: int PID_CATEGORY
pub const PID_CATEGORY: i32 = 2;
// Java field: int PID_PRESFORMAT
pub const PID_PRESFORMAT: i32 = 3;
// Java field: int PID_BYTECOUNT
pub const PID_BYTECOUNT: i32 = 4;
// Java field: int PID_LINECOUNT
pub const PID_LINECOUNT: i32 = 5;
// Java field: int PID_PARACOUNT
pub const PID_PARACOUNT: i32 = 6;
// Java field: int PID_SLIDECOUNT
pub const PID_SLIDECOUNT: i32 = 7;
// Java field: int PID_NOTECOUNT
pub const PID_NOTECOUNT: i32 = 8;
// Java field: int PID_HIDDENCOUNT
pub const PID_HIDDENCOUNT: i32 = 9;
// Java field: int PID_MMCLIPCOUNT
pub const PID_MMCLIPCOUNT: i32 = 10;
// Java field: int PID_SCALE
pub const PID_SCALE: i32 = 11;
// Java field: int PID_HEADINGPAIR
pub const PID_HEADINGPAIR: i32 = 12;
// Java field: int PID_DOCPARTS
pub const PID_DOCPARTS: i32 = 13;
// Java field: int PID_MANAGER
pub const PID_MANAGER: i32 = 14;
// Java field: int PID_COMPANY
pub const PID_COMPANY: i32 = 15;
// Java field: int PID_LINKSDIRTY
pub const PID_LINKSDIRTY: i32 = 16;
// Java field: int PID_CCHWITHSPACES
pub const PID_CCHWITHSPACES: i32 = 17;
// Java field: int PID_SHAREDDOC
pub const PID_SHAREDDOC: i32 = 19;
// Java field: int PID_LINKBASE
pub const PID_LINKBASE: i32 = 20;
// Java field: int PID_HLINKS
pub const PID_HLINKS: i32 = 21;
// Java field: int PID_HYPERLINKSCHANGED
pub const PID_HYPERLINKSCHANGED: i32 = 22;
// Java field: int PID_VERSION
pub const PID_VERSION: i32 = 23;
// Java field: int PID_DIGSIG
pub const PID_DIGSIG: i32 = 24;
// Java field: int PID_CONTENTTYPE
pub const PID_CONTENTTYPE: i32 = 26;
// Java field: int PID_CONTENTSTATUS
pub const PID_CONTENTSTATUS: i32 = 27;
// Java field: int PID_LANGUAGE
pub const PID_LANGUAGE: i32 = 28;
// Java field: int PID_DOCVERSION
pub const PID_DOCVERSION: i32 = 29;
// Java field: int PID_MAX
pub const PID_MAX: i32 = 29;

// Java field: PropertyIDMap summaryInformationProperties
static SUMMARY_INFO_PROPS: OnceLock<PropertyIDMap> = OnceLock::new();
// Java field: PropertyIDMap documentSummaryInformationProperties
static DOC_SUMMARY_INFO_PROPS: OnceLock<PropertyIDMap> = OnceLock::new();

// Java: public class PropertyIDMap extends HashMap
pub struct PropertyIDMap {
    map: HashMap<i64, String>,
}

// Java: public class PropertyIDMap extends HashMap
impl PropertyIDMap {
    // Java: PropertyIDMap(int initialCapacity, float loadFactor)
    pub fn new(initialCapacity: usize, _loadFactor: f32) -> Self {
        // Java: super(initialCapacity, loadFactor);
        PropertyIDMap {
            map: HashMap::with_capacity(initialCapacity),
        }
    }

    // Java: PropertyIDMap(Map map)
    pub fn new_from_map(map: HashMap<i64, String>) -> Self {
        // Java: super(map);
        PropertyIDMap { map }
    }

    // Java: Object put(long id, String idString)
    pub fn put(&mut self, id: i64, idString: &str) -> Option<String> {
        // Java: return this.put(id, idString);
        self.map.insert(id, idString.to_string())
    }

    // Java: Object get(long id)
    pub fn get(&self, id: i64) -> Option<&String> {
        // Java: return this.get(id);
        self.map.get(&id)
    }

    // Java: static PropertyIDMap getSummaryInformationProperties()
    pub fn getSummaryInformationProperties() -> &'static PropertyIDMap {
        SUMMARY_INFO_PROPS.get_or_init(|| {
            // Java: PropertyIDMap m = new PropertyIDMap(18, 1.0F);
            let mut m = PropertyIDMap::new(18, 1.0);
            // Java: m.put(2L, "PID_TITLE");
            m.put(2, "PID_TITLE");
            // Java: m.put(3L, "PID_SUBJECT");
            m.put(3, "PID_SUBJECT");
            // Java: m.put(4L, "PID_AUTHOR");
            m.put(4, "PID_AUTHOR");
            // Java: m.put(5L, "PID_KEYWORDS");
            m.put(5, "PID_KEYWORDS");
            // Java: m.put(6L, "PID_COMMENTS");
            m.put(6, "PID_COMMENTS");
            // Java: m.put(7L, "PID_TEMPLATE");
            m.put(7, "PID_TEMPLATE");
            // Java: m.put(8L, "PID_LASTAUTHOR");
            m.put(8, "PID_LASTAUTHOR");
            // Java: m.put(9L, "PID_REVNUMBER");
            m.put(9, "PID_REVNUMBER");
            // Java: m.put(10L, "PID_EDITTIME");
            m.put(10, "PID_EDITTIME");
            // Java: m.put(11L, "PID_LASTPRINTED");
            m.put(11, "PID_LASTPRINTED");
            // Java: m.put(12L, "PID_CREATE_DTM");
            m.put(12, "PID_CREATE_DTM");
            // Java: m.put(13L, "PID_LASTSAVE_DTM");
            m.put(13, "PID_LASTSAVE_DTM");
            // Java: m.put(14L, "PID_PAGECOUNT");
            m.put(14, "PID_PAGECOUNT");
            // Java: m.put(15L, "PID_WORDCOUNT");
            m.put(15, "PID_WORDCOUNT");
            // Java: m.put(16L, "PID_CHARCOUNT");
            m.put(16, "PID_CHARCOUNT");
            // Java: m.put(17L, "PID_THUMBNAIL");
            m.put(17, "PID_THUMBNAIL");
            // Java: m.put(18L, "PID_APPNAME");
            m.put(18, "PID_APPNAME");
            // Java: m.put(19L, "PID_SECURITY");
            m.put(19, "PID_SECURITY");
            // Java: summaryInformationProperties = new PropertyIDMap(Collections.unmodifiableMap(m));
            m
        })
    }

    // Java: static PropertyIDMap getDocumentSummaryInformationProperties()
    pub fn getDocumentSummaryInformationProperties() -> &'static PropertyIDMap {
        DOC_SUMMARY_INFO_PROPS.get_or_init(|| {
            // Java: PropertyIDMap m = new PropertyIDMap(28, 1.0F);
            let mut m = PropertyIDMap::new(28, 1.0);
            // Java: m.put(0L, "PID_DICTIONARY");
            m.put(0, "PID_DICTIONARY");
            // Java: m.put(1L, "PID_CODEPAGE");
            m.put(1, "PID_CODEPAGE");
            // Java: m.put(2L, "PID_CATEGORY");
            m.put(2, "PID_CATEGORY");
            // Java: m.put(3L, "PID_PRESFORMAT");
            m.put(3, "PID_PRESFORMAT");
            // Java: m.put(4L, "PID_BYTECOUNT");
            m.put(4, "PID_BYTECOUNT");
            // Java: m.put(5L, "PID_LINECOUNT");
            m.put(5, "PID_LINECOUNT");
            // Java: m.put(6L, "PID_PARACOUNT");
            m.put(6, "PID_PARACOUNT");
            // Java: m.put(7L, "PID_SLIDECOUNT");
            m.put(7, "PID_SLIDECOUNT");
            // Java: m.put(8L, "PID_NOTECOUNT");
            m.put(8, "PID_NOTECOUNT");
            // Java: m.put(9L, "PID_HIDDENCOUNT");
            m.put(9, "PID_HIDDENCOUNT");
            // Java: m.put(10L, "PID_MMCLIPCOUNT");
            m.put(10, "PID_MMCLIPCOUNT");
            // Java: m.put(11L, "PID_SCALE");
            m.put(11, "PID_SCALE");
            // Java: m.put(12L, "PID_HEADINGPAIR");
            m.put(12, "PID_HEADINGPAIR");
            // Java: m.put(13L, "PID_DOCPARTS");
            m.put(13, "PID_DOCPARTS");
            // Java: m.put(14L, "PID_MANAGER");
            m.put(14, "PID_MANAGER");
            // Java: m.put(15L, "PID_COMPANY");
            m.put(15, "PID_COMPANY");
            // Java: m.put(16L, "PID_LINKSDIRTY");
            m.put(16, "PID_LINKSDIRTY");
            // Java: m.put(17L, "PID_CCHWITHSPACES");
            m.put(17, "PID_CCHWITHSPACES");
            // Java: m.put(19L, "PID_SHAREDDOC");
            m.put(19, "PID_SHAREDDOC");
            // Java: m.put(20L, "PID_LINKBASE");
            m.put(20, "PID_LINKBASE");
            // Java: m.put(21L, "PID_HLINKS");
            m.put(21, "PID_HLINKS");
            // Java: m.put(22L, "PID_HYPERLINKSCHANGED");
            m.put(22, "PID_HYPERLINKSCHANGED");
            // Java: m.put(23L, "PID_VERSION");
            m.put(23, "PID_VERSION");
            // Java: m.put(24L, "PID_DIGSIG");
            m.put(24, "PID_DIGSIG");
            // Java: m.put(26L, "PID_CONTENTTYPE");
            m.put(26, "PID_CONTENTTYPE");
            // Java: m.put(27L, "PID_CONTENTSTATUS");
            m.put(27, "PID_CONTENTSTATUS");
            // Java: m.put(28L, "PID_LANGUAGE");
            m.put(28, "PID_LANGUAGE");
            // Java: m.put(29L, "PID_DOCVERSION");
            m.put(29, "PID_DOCVERSION");
            // Java: documentSummaryInformationProperties = new PropertyIDMap(Collections.unmodifiableMap(m));
            m
        })
    }
}
