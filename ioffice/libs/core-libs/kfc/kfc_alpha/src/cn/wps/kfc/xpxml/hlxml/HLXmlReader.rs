use std::io::Read;
use crate::kfc::xpxml::hlxml::HLElementReader::HLElementReader;
use crate::kfc::xpxml::hlxml::HLHandlerBuilder::HLHandlerBuilder;
use crate::kfc::xpxml::hlxml::str::HLStrHandlerBuilder::HLStrHandlerBuilder;
use crate::kfc::xpxml::reader::ElementHandler::ElementHandler;
use crate::kfc::xpxml::reader::ElementStrHandler::ElementStrHandler;
use crate::kfc::xpxml::reader::XmlName2Id::XmlName2Id;
use crate::kfc::xpxml::util::FilePartInputStream::FilePartInputStream;
use crate::kfc::xpxml::hlxml::Mode::Mode;

// Java: public class HLXmlReader
pub struct HLXmlReader {
    // Java field: XmlName2Id mXmlIds
    mXmlIds: Box<dyn XmlName2Id>,
    // Java field: ElementHandler mHandler
    mHandler: Option<Box<dyn ElementHandler>>,
    // Java field: Mode mMode
    mMode: Mode,
}

// Java: public class HLXmlReader
impl HLXmlReader {
    // Java: HLXmlReader(XmlName2Id xmlIds, ElementHandler handler)
    pub fn new(xml_ids: Box<dyn XmlName2Id>, handler: Box<dyn ElementHandler>) -> Self {
        // Java: this(xmlIds, handler, Mode.normal);
        Self::new_with_mode(xml_ids, handler, Mode::Normal)
    }

    // Java: HLXmlReader(XmlName2Id xmlIds, ElementHandler handler, Mode mode)
    pub fn new_with_mode(xml_ids: Box<dyn XmlName2Id>, handler: Box<dyn ElementHandler>, mode: Mode) -> Self {
        // Java: super();
        // Java: this.mXmlIds = xmlIds;
        // Java: this.mHandler = handler;
        // Java: this.mMode = mode;
        HLXmlReader {
            mXmlIds: xml_ids,
            mHandler: Some(handler),
            mMode: mode,
        }
    }

    // Java: void parser(String path)
    pub fn parser_path(&mut self, path: &str) -> std::io::Result<()> {
        // Java: FileInputStream stream = null;
        // Java: try
        // Java: stream = new FileInputStream(path);
        let mut stream = std::fs::File::open(path)?;
        // Java: this.parser(stream, true);
        self.parser_stream_begin(&mut stream, true)?;
        // Java: finally
        // Java: if (stream != null)
        // Java: stream.close();
        // Java: FileInputStream var6 = null;
        Ok(())
    }

    // Java: void parser(String path, int beginOffset, int endOffset)
    pub fn parser_path_offsets(&mut self, path: &str, begin_offset: i32, end_offset: i32) -> std::io::Result<()> {
        // Java: this.parser(path, beginOffset, endOffset, beginOffset == 0);
        self.parser_path_offsets_root(path, begin_offset, end_offset, begin_offset == 0)
    }

    // Java: void parser(String path, int beginOffset, int endOffset, boolean bBeginAtRoot)
    pub fn parser_path_offsets_root(&mut self, path: &str, begin_offset: i32, end_offset: i32, b_begin_at_root: bool) -> std::io::Result<()> {
        // Java: if (beginOffset >= 0 && endOffset > beginOffset)
        if begin_offset >= 0 && end_offset > begin_offset {
            // Java: FilePartInputStream stream = null;
            // Java: try
            // Java: stream = new FilePartInputStream(path, beginOffset, endOffset);
            let mut stream = FilePartInputStream::new(path, begin_offset, end_offset)?;
            // Java: this.parser(stream, bBeginAtRoot);
            self.parser_stream_begin(&mut stream, b_begin_at_root)?;
            // Java: finally
            // Java: if (stream != null)
            // Java: stream.close();
            // Java: FilePartInputStream var9 = null;
        }
        // Java: else if (beginOffset == -1 && endOffset == -1)
        else if begin_offset == -1 && end_offset == -1 {
            // Java: this.parser(path);
            self.parser_path(path)?;
        }
        Ok(())
    }

    // Java: void parser(InputStream stream)
    pub fn parser_stream(&mut self, stream: &mut dyn Read) -> std::io::Result<()> {
        // Java: this.parser(stream, true);
        self.parser_stream_begin(stream, true)
    }

    // Java: void parser(InputStream stream, boolean bBeginAtRoot)
    pub fn parser_stream_begin(&mut self, stream: &mut dyn Read, b_begin_at_root: bool) -> std::io::Result<()> {
        // Java: this.parser(stream, bBeginAtRoot, -1);
        self.parser_stream_begin_prefix(stream, b_begin_at_root, -1)
    }

    // Java: void parser(InputStream stream, boolean bBeginAtRoot, int labelPrefix)
    pub fn parser_stream_begin_prefix(&mut self, stream: &mut dyn Read, b_begin_at_root: bool, label_prefix: i32) -> std::io::Result<()> {
        // Java: HLHandlerBuilder handlerBuilder = new HLHandlerBuilder(this.mXmlIds, this.mHandler, bBeginAtRoot, this.mMode);
        let handler = self.mHandler.take().expect("HLXmlReader.mHandler");
        let ids = &*self.mXmlIds as *const dyn XmlName2Id;
        let mut handler_builder = HLHandlerBuilder::new(ids, handler, b_begin_at_root, self.mMode);
        // Java: handlerBuilder.setLabelParserPrefix(labelPrefix);
        handler_builder.set_label_parser_prefix(label_prefix);
        // Java: HLElementReader reader = new HLElementReader(handlerBuilder);
        let mut reader = HLElementReader::new(handler_builder);
        // Java: reader.parser(stream);
        reader.parser(stream)?;
        self.mHandler = Some(reader.into_accept().into_root());
        Ok(())
    }

    // Java: static void parser(InputStream stream, ElementStrHandler handler)
    pub fn parser_str_handler(stream: &mut dyn Read, handler: Box<dyn ElementStrHandler>) -> std::io::Result<()> {
        // Java: HLStrHandlerBuilder handlerBuilder = new HLStrHandlerBuilder(handler);
        let handler_builder = HLStrHandlerBuilder::new(handler);
        // Java: HLElementReader reader = new HLElementReader(handlerBuilder);
        // Java: reader.parser(stream);
        let mut reader=HLElementReader::new(handler_builder);reader.parser(stream)?;
        Ok(())
    }

    // Java: static void parser(InputStream stream, ElementStrHandler handler, String encoding)
    pub fn parser_str_handler_encoding(stream: &mut dyn Read, handler: Box<dyn ElementStrHandler>, encoding: &str) -> std::io::Result<()> {
        // Java: HLStrHandlerBuilder handlerBuilder = new HLStrHandlerBuilder(handler);
        let handler_builder = HLStrHandlerBuilder::new(handler);
        // Java: HLElementReader reader = new HLElementReader(handlerBuilder);
        // Java: reader.parser(stream, encoding);
        let mut reader=HLElementReader::new(handler_builder);reader.parser_encoding(stream,encoding)?;
        Ok(())
    }

    // Java: void parser(InputStream stream, String encoding)
    pub fn parser_stream_encoding(&mut self, stream: &mut dyn Read, encoding: &str) -> std::io::Result<()> {
        // Java: HLHandlerBuilder handlerBuilder = new HLHandlerBuilder(this.mXmlIds, this.mHandler, true, this.mMode);
        let handler = self.mHandler.take().expect("HLXmlReader.mHandler");
        let ids = &*self.mXmlIds as *const dyn XmlName2Id;
        let handler_builder = HLHandlerBuilder::new(ids, handler, true, self.mMode);
        // Java: HLElementReader reader = new HLElementReader(handlerBuilder);
        let mut reader = HLElementReader::new(handler_builder);
        // Java: reader.parser(stream, encoding);
        reader.parser_encoding(stream, encoding)?;
        self.mHandler = Some(reader.into_accept().into_root());
        Ok(())
    }
}
