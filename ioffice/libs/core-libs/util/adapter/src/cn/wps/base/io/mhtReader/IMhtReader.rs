// Java: public interface IMhtReader
pub trait IMhtReader {
    // Java: abstract String getHtmlPath()
    fn getHtmlPath(&self) -> String;

    // Java: abstract String[] getAllPartPath()
    fn getAllPartPath(&self) -> Vec<String>;

}
