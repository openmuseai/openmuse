// Java: import cn.wps.moffice.online.security.data.SecurityCreateDocData
// Java: import cn.wps.moffice.online.security.data.SecurityReadDocData
// Java: import cn.wps.moffice.online.security.data.SecurityRightData
// Java: import cn.wps.moffice.online.security.data.SecurityUpdateDocData
// Java: import cn.wps.moffice.online.security.data.SecurityVersionsData
// Java: import cn.wps.moffice.online.security.exception.OnlineSecurityException
// Java: import java.util.ArrayList

// Java: public interface IOnlineSecurityAPI
pub trait IOnlineSecurityAPI {
    // Java: abstract SecurityVersionsData versions()
    fn versions(&self) -> Box<dyn std::any::Any>;

    // Java: abstract SecurityReadDocData readDoc(String var1, String var2, String var3, boolean var4)
    fn readDoc(&self, _var1: &str, _var2: &str, _var3: &str, _var4: bool) -> Box<dyn std::any::Any>;

    // Java: abstract SecurityCreateDocData createDoc(String var1, String var2, String var3, ArrayList<SecurityRightData> var4, boolean var5)
    fn createDoc(&self, _var1: &str, _var2: &str, _var3: &str, _var4: Vec<Box<dyn std::any::Any>>, _var5: bool) -> Box<dyn std::any::Any>;

    // Java: abstract SecurityUpdateDocData updateDoc(String var1, String var2, String var3, String var4, String var5, String var6, ArrayList<SecurityRightData> var7, boolean var8)
    fn updateDoc(&self, _var1: &str, _var2: &str, _var3: &str, _var4: &str, _var5: &str, _var6: &str, _var7: Vec<Box<dyn std::any::Any>>, _var8: bool) -> Box<dyn std::any::Any>;

    // Java: abstract void checkOperation(String var1, String var2)
    fn checkOperation(&self, _var1: &str, _var2: &str);

    // Java: abstract void checkErrorCode(int var1)
    fn checkErrorCode(&self, _var1: i32);

    // Java: abstract void onOnlineSecurityDocOpened()
    fn onOnlineSecurityDocOpened(&self);

    // Java: abstract boolean inquireOperation(String var1, String var2, String var3)
    fn inquireOperation(&self, _var1: &str, _var2: &str, _var3: &str) -> bool;

}
