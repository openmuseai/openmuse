// java public interface ICmtCustDatasReader
pub trait ICmtCustDatasReader {
    // java  boolean read(String var1)
    fn read(&mut self, var1: &str) -> bool;

}
