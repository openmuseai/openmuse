// Java: import cn.wps.io.file.FileFormatEnum
// use util::cn::wps::io::file::FileFormatEnum;

// Java: public interface IFileParser
pub trait IFileParser {
    // Java: abstract FileFormatEnum parse(String var1)
    fn parse(&self, var1: &str) -> Box<dyn std::any::Any>;

}
