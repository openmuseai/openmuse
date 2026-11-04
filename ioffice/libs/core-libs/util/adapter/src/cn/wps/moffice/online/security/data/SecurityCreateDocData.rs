// Java: public class SecurityCreateDocData
pub struct SecurityCreateDocData {
    // Java field: String docGuid
    pub(crate) docGuid: String,
    // Java field: String encData
    pub(crate) encData: String,
}

// Java: public class SecurityCreateDocData
impl SecurityCreateDocData {
    // Java: SecurityCreateDocData(String docGuid, String encData)
    pub fn new(docGuid: String, encData: String) -> Self {
        // Java: super();
        // Java: this.docGuid = docGuid;
        // Java: this.encData = encData;
        SecurityCreateDocData { docGuid, encData }
    }

}
