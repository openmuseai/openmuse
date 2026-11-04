// Java: import java.util.ArrayList

// Java: public class SecurityReadDocData
pub struct SecurityReadDocData {
    // Java field: String docGuid
    pub(crate) docGuid: String,
    // Java field: String docSecretKey
    pub(crate) docSecretKey: String,
    // Java field: ArrayList<SecurityRightData> rights
    pub(crate) rights: Vec<Box<dyn std::any::Any>>,
}

// Java: public class SecurityReadDocData
impl SecurityReadDocData {
    // Java: SecurityReadDocData(String docGuid, String docSecretKey, ArrayList<SecurityRightData> rights)
    pub fn new(docGuid: String, docSecretKey: String, rights: Vec<Box<dyn std::any::Any>>) -> Self {
        // Java: super();
        // Java: this.docGuid = docGuid;
        // Java: this.docSecretKey = docSecretKey;
        // Java: this.rights = rights;
        SecurityReadDocData { docGuid, docSecretKey, rights }
    }

}
