// Java: public class MapEntry
pub struct MapEntry {
    // Java field: String[] set
    pub(crate) set: Option<Vec<String>>,
    // Java field: int len
    pub(crate) len: i32,
}

// Java: public class MapEntry
impl MapEntry {
    // Java: MapEntry()
    pub fn new() -> Self {
        // Java: super();
        Self {
            set: None,
            len: 0,
        }
    }
}
