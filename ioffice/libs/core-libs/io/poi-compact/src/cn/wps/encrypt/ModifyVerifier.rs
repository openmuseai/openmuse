// Java: public class ModifyVerifier
#[derive(Clone)]
pub struct ModifyVerifier {
    // Java field: String hashData
    pub(crate) hashData: Option<String>,
    // Java field: String saltData
    pub(crate) saltData: Option<String>,
    // Java field: int spinCount
    pub(crate) spinCount: i32,
    // Java field: int cryptAlgorithmSid
    pub(crate) cryptAlgorithmSid: i32,
}

// Java: public class ModifyVerifier
impl ModifyVerifier {
    // Java: ModifyVerifier(String hash, String saltData, int spinCount)
    pub fn new(hash: &str, saltData: &str, spinCount: i32) -> Self {
        // rust: existing non-null string transport.
        Self::new__nullable(Some(hash),Some(saltData),spinCount)
    }
    // java public ModifyVerifier(String hash, String saltData, int spinCount) {
    pub fn new__nullable(hash:Option<&str>,saltData:Option<&str>,spinCount:i32)->Self{
        // Java: super();
        ModifyVerifier {
            // Java: this.hashData = hash;
            hashData: hash.map(str::to_owned),
            // Java: this.saltData = saltData;
            saltData: saltData.map(str::to_owned),
            // Java: this.spinCount = spinCount;
            spinCount,
            cryptAlgorithmSid: 4,
        }
    }

    // Java: ModifyVerifier(String hash, String saltData, int spinCount, int algorithmSid)
    pub fn new_with_algorithm(hash: &str, saltData: &str, spinCount: i32, algorithmSid: i32) -> Self {
        // rust: existing non-null string transport.
        Self::new_with_algorithm__nullable(Some(hash),Some(saltData),spinCount,algorithmSid)
    }
    // java public ModifyVerifier(String hash, String saltData, int spinCount, int algorithmSid) {
    pub fn new_with_algorithm__nullable(hash:Option<&str>,saltData:Option<&str>,spinCount:i32,algorithmSid:i32)->Self{
        // Java: super();
        ModifyVerifier {
            // Java: this.hashData = hash;
            hashData: hash.map(str::to_owned),
            // Java: this.saltData = saltData;
            saltData: saltData.map(str::to_owned),
            // Java: this.spinCount = spinCount;
            spinCount,
            // Java: this.cryptAlgorithmSid = algorithmSid;
            cryptAlgorithmSid: algorithmSid,
        }
    }

    // Java: String getHashData()
    pub fn getHashData(&self) -> Option<&str> {
        // Java: return this.hashData;
        self.hashData.as_deref()
    }

    // Java: String getSaltData()
    pub fn getSaltData(&self) -> Option<&str> {
        // Java: return this.saltData;
        self.saltData.as_deref()
    }

    // Java: int getSpinCount()
    pub fn getSpinCount(&self) -> i32 {
        // Java: return this.spinCount;
        self.spinCount
    }

    // Java: void setHashData(String hash)
    pub fn setHashData(&mut self, hash: &str) {
        // rust: existing non-null string transport.
        self.setHashData__nullable(Some(hash));
    }
    // java public void setHashData(String hash) { this.hashData = hash; }
    pub fn setHashData__nullable(&mut self,hash:Option<&str>){
        // Java: this.hashData = hash;
        self.hashData = hash.map(str::to_owned);
    }

    // Java: void setSaltData(String salt)
    pub fn setSaltData(&mut self, salt: &str) {
        // rust: existing non-null string transport.
        self.setSaltData__nullable(Some(salt));
    }
    // java public void setSaltData(String salt) { this.saltData = salt; }
    pub fn setSaltData__nullable(&mut self,salt:Option<&str>){
        // Java: this.saltData = salt;
        self.saltData = salt.map(str::to_owned);
    }

    // Java: void setSpinCount(int spin)
    pub fn setSpinCount(&mut self, spin: i32) {
        // Java: this.spinCount = spin;
        self.spinCount = spin;
    }

    // Java: void setAlogrithmSid(int algorithmSid)
    pub fn setAlogrithmSid(&mut self, algorithmSid: i32) {
        // Java: this.cryptAlgorithmSid = algorithmSid;
        self.cryptAlgorithmSid = algorithmSid;
    }

    // Java: int getAlogrithmSid()
    pub fn getAlogrithmSid(&self) -> i32 {
        // Java: return this.cryptAlgorithmSid;
        self.cryptAlgorithmSid
    }
}
