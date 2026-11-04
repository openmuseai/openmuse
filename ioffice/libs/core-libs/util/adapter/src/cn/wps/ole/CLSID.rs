// Java: public class CLSID
pub struct CLSID {
    // Java field: byte[] data
    data: Option<Vec<u8>>,
}

// Java: public class CLSID
impl CLSID {
    // Java: CLSID()
    pub fn new() -> Self {
        // Java: super();
        CLSID { data: None }
    }

}
