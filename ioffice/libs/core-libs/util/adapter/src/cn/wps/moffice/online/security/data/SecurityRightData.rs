// Java: import java.util.ArrayList

// Java: public class SecurityRightData
pub struct SecurityRightData {
    // Java field: String principalId
    pub(crate) principalId: String,
    // Java field: String principalTitle
    pub(crate) principalTitle: String,
    // Java field: ArrayList<String> operationIds
    pub(crate) operationIds: Vec<String>,
}

// Java: public class SecurityRightData
impl SecurityRightData {
    // Java: SecurityRightData(String principalId, String principalTitle, ArrayList<String> operationIds)
    pub fn new(principalId: String, principalTitle: String, operationIds: Vec<String>) -> Self {
        // Java: super();
        // Java: this.principalId = principalId;
        // Java: this.principalTitle = principalTitle;
        // Java: this.operationIds = operationIds;
        SecurityRightData { principalId, principalTitle, operationIds }
    }

}
