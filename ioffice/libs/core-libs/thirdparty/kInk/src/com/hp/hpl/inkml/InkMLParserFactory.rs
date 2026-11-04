use super::InkMLParser::InkMLParser;

// Java: public class InkMLParserFactory
pub struct InkMLParserFactory {
}

// Java: public class InkMLParserFactory
impl InkMLParserFactory {
    // Java: InkMLParserFactory()
    pub fn InkMLParserFactory() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static InkMLParser getInkMLParser(String parserClassPath)
    pub fn getInkMLParser(_parser_class_path: &str) -> Box<dyn InkMLParser> {
      // Java: throw new InkMLException("Feature not implemented yet.");
      panic!("Feature not implemented yet.")
    }

}
