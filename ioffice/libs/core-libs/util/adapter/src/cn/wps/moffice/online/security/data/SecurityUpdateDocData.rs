// Java: public class SecurityUpdateDocData
pub struct SecurityUpdateDocData {
    // Java field: String docGuid
    pub(crate) docGuid: String,
    // Java field: String encData
    pub(crate) encData: String,
}

// Java: public class SecurityUpdateDocData
impl SecurityUpdateDocData {
    // Java: SecurityUpdateDocData(String docGuid, String encData)
    pub fn new(docGuid: String, encData: String) -> Self {
        // Java: super();
        // Java: this.docGuid = docGuid;
        // Java: this.encData = encData;
        SecurityUpdateDocData { docGuid, encData }
    }

}
