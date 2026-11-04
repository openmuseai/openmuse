// Java: import java.util.ArrayList

// Java: public class SecurityVersionsData
pub struct SecurityVersionsData {
    // Java field: ArrayList<String> perm
    pub(crate) perm: Vec<String>,
    // Java field: ArrayList<String> trans
    pub(crate) trans: Vec<String>,
}

// Java: public class SecurityVersionsData
impl SecurityVersionsData {
    // Java: SecurityVersionsData(ArrayList<String> perm, ArrayList<String> trans)
    pub fn new(perm: Vec<String>, trans: Vec<String>) -> Self {
        // Java: super();
        // Java: this.perm = perm;
        // Java: this.trans = trans;
        SecurityVersionsData { perm, trans }
    }

}
