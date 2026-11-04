// Java: import java.io.IOException

// Java: public interface IWpsSecurityFileWrite
pub trait IWpsSecurityFileWrite {
    // Java: abstract void dispose()
    fn dispose(&mut self);

    // Java: abstract void save(String var1, String var2, String var3)
    fn save(&self, _var1: &str, _var2: &str, _var3: &str);

    // Java: abstract String getDestPath()
    fn getDestPath(&self) -> String;

    // Java: abstract String getSrcPath()
    fn getSrcPath(&self) -> String;

    // Java: abstract String generateDataSign(String var1)
    fn generateDataSign(&self, _var1: &str) -> String;

}
