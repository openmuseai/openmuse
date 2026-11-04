// Java: public class CustomXmlPair
pub struct CustomXmlPair {
    // Java field: String mItemPath
    pub mItemPath: String,
    // Java field: String mPropsPath
    pub mPropsPath: String,
}

// Java: public class CustomXmlPair
impl CustomXmlPair {
    pub fn new() -> Self {
        CustomXmlPair {
            mItemPath: String::new(),
            mPropsPath: String::new(),
        }
    }
}
