#![allow(unreachable_code)]
// Java: import cn.wps.base.io.XmlFeatureSecUtil
// use util::cn::wps::base::io::XmlFeatureSecUtil;
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.DocumentException
// Java: import cn.wps.io.dom.ElementHandler
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import java.io.FileNotFoundException
// Java: import java.io.InputStream
// Java: import java.io.Reader
// Java: import java.io.Serializable
// Java: import java.net.URL
// Java: import org.xml.sax.EntityResolver
// Java: import org.xml.sax.ErrorHandler
// Java: import org.xml.sax.InputSource
// Java: import org.xml.sax.SAXException
// Java: import org.xml.sax.SAXNotRecognizedException
// Java: import org.xml.sax.SAXNotSupportedException
// Java: import org.xml.sax.SAXParseException
// Java: import org.xml.sax.XMLFilter
// Java: import org.xml.sax.XMLReader
// Java: import org.xml.sax.helpers.DefaultHandler
// Java: import org.xml.sax.helpers.XMLReaderFactory

use super::DispatchHandler::DispatchHandler;
use super::MCEContext::MCEContext;

// Java field: String SAX_STRING_INTERNING
pub(crate) const SAX_STRING_INTERNING: &str = "http://xml.org/sax/features/string-interning";
// Java field: String SAX_NAMESPACE_PREFIXES
pub(crate) const SAX_NAMESPACE_PREFIXES: &str = "http://xml.org/sax/features/namespace-prefixes";
// Java field: String SAX_NAMESPACES
pub(crate) const SAX_NAMESPACES: &str = "http://xml.org/sax/features/namespaces";
// Java field: String SAX_DECL_HANDLER
pub(crate) const SAX_DECL_HANDLER: &str = "http://xml.org/sax/properties/declaration-handler";
// Java field: String SAX_LEXICAL_HANDLER
pub(crate) const SAX_LEXICAL_HANDLER: &str = "http://xml.org/sax/properties/lexical-handler";

// Java: public class SAXReader
pub struct SAXReader {
    // Java field: XMLReader xmlReader
    pub(crate) xmlReader: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: boolean validating
    pub(crate) validating: bool,
    // Java field: DispatchHandler dispatchHandler
    dispatchHandler: Option<DispatchHandler>,
    // Java field: ErrorHandler errorHandler
    pub(crate) errorHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: EntityResolver entityResolver
    pub(crate) entityResolver: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: boolean stringInternEnabled
    pub(crate) stringInternEnabled: bool,
    // Java field: boolean includeInternalDTDDeclarations
    pub(crate) includeInternalDTDDeclarations: bool,
    // Java field: boolean includeExternalDTDDeclarations
    pub(crate) includeExternalDTDDeclarations: bool,
    // Java field: boolean mergeAdjacentText
    pub(crate) mergeAdjacentText: bool,
    // Java field: boolean stripWhitespaceText
    pub(crate) stripWhitespaceText: bool,
    // Java field: boolean ignoreComments
    pub(crate) ignoreComments: bool,
    // Java field: String encoding
    pub(crate) encoding: Option<String>,
    // Java field: XMLFilter xmlFilter
    pub(crate) xmlFilter: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: boolean mOnlyUseName
    mOnlyUseName: bool,
    // Java field: MCEContext mMCEContext
    pub(crate) mMCEContext: MCEContext,
}

// Java: public class SAXEntityResolver
pub struct SAXEntityResolver {
    // Java field: String uriPrefix
    pub(crate) uriPrefix: Option<String>,
}

// Java: public class SAXReader
impl SAXReader {
    fn _default() -> Self {
        Self {
            xmlReader: None,
            validating: false,
            dispatchHandler: None,
            errorHandler: None,
            entityResolver: None,
            stringInternEnabled: true,
            includeInternalDTDDeclarations: false,
            includeExternalDTDDeclarations: false,
            mergeAdjacentText: false,
            stripWhitespaceText: false,
            ignoreComments: false,
            encoding: None,
            xmlFilter: None,
            mOnlyUseName: false,
            mMCEContext: MCEContext::new(),
        }
    }

    // Java: SAXReader()
    pub fn new() -> Self {
        // Java: super();
        Self::_default()
    }

    // Java: SAXReader(boolean validating)
    pub fn new_with_validating(validating: bool) -> Self {
        // Java: super();
        let mut reader = Self::_default();
        // Java: this.validating = validating;
        reader.validating = validating;
        reader
    }

    // Java: SAXReader(XMLReader xmlReader)
    pub fn new_with_xml_reader(xml_reader: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super();
        let mut reader = Self::_default();
        // Java: this.xmlReader = xmlReader;
        reader.xmlReader = Some(xml_reader);
        reader
    }

    // Java: SAXReader(XMLReader xmlReader, boolean validating)
    pub fn new_with_xml_reader_and_validating(xml_reader: Box<dyn std::any::Any + Send + Sync>, validating: bool) -> Self {
        // Java: super();
        let mut reader = Self::_default();
        // Java: this.xmlReader = xmlReader;
        reader.xmlReader = Some(xml_reader);
        // Java: this.validating = validating;
        reader.validating = validating;
        reader
    }

    // Java: SAXReader(String xmlReaderClassName)
    pub fn new_with_class_name(_xml_reader_class_name: Option<&str>) -> Self {
        // Java: super();
        let reader = Self::_default();
        // Java: if (xmlReaderClassName != null)
        {
            // Java: this.xmlReader = XMLReaderFactory.createXMLReader(xmlReaderClassName);
        }
        reader
    }

    // Java: SAXReader(String xmlReaderClassName, boolean validating)
    pub fn new_with_class_name_and_validating(_xml_reader_class_name: Option<&str>, validating: bool) -> Self {
        // Java: super();
        let mut reader = Self::_default();
        // Java: if (xmlReaderClassName != null)
        {
            // Java: this.xmlReader = XMLReaderFactory.createXMLReader(xmlReaderClassName);
            // Java: XmlFeatureSecUtil.setFeatureXml(this.xmlReader);
        }
        // Java: this.validating = validating;
        reader.validating = validating;
        reader
    }

    // Java: void setOnlyUseName(boolean val)
    pub fn setOnlyUseName(&mut self, val: bool) {
        // Java: this.mOnlyUseName = val;
        self.mOnlyUseName = val;
    }

    // Java: void setProperty(String name, Object value)
    pub fn setProperty(&mut self, _name: &str, _value: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.getXMLReader().setProperty(name, value);
        todo!();
    }

    // Java: void setFeature(String name, boolean value)
    pub fn setFeature(&mut self, _name: &str, _value: bool) {
        // Java: this.getXMLReader().setFeature(name, value);
        todo!();
    }

    // Java: Document read(File file)
    pub fn read_file(&mut self, _file: &dyn std::any::Any) -> Result<Box<dyn std::any::Any + Send + Sync>, Box<dyn std::error::Error>> {
        // Java: try
        {
            // Java: if (this.encoding != null)
            {
            // Java: InputSource source = new InputSource(new FileInputStream(file)); source.setEncoding(this.encoding);
            }
            // Java: String path = file.getAbsolutePath();
            // Java: if (path != null)
            {
                // Java: StringBuffer sb = new StringBuffer("file://"); if (!path.startsWith(File.separator))
                {
                    // Java: sb.append("/");
                }
                // Java: path = path.replace('\\', '/');
                // Java: sb.append(path);
                // Java: source.setSystemId(sb.toString());
            }
            // Java: return this.read(source);
        }
        // Java: catch (FileNotFoundException e)
        {
            // Java: throw new DocumentException(e.getMessage(), e);
        }
        todo!()
    }

    // Java: Document read(URL url)
    pub fn read_url(&mut self, _url: &str) -> Result<Box<dyn std::any::Any + Send + Sync>, Box<dyn std::error::Error>> {
        // Java: String systemID = url.toExternalForm(); InputSource source = new InputSource(systemID);
        // Java: if (this.encoding != null)
        {
            // Java: source.setEncoding(this.encoding);
        }
        // Java: return this.read(source);
        todo!()
    }

    // Java: Document read(String systemId)
    pub fn read_system_id(&mut self, _system_id: &str) -> Result<Box<dyn std::any::Any + Send + Sync>, Box<dyn std::error::Error>> {
        // Java: if (this.encoding != null)
        {
        // Java: InputSource source = new InputSource(systemId); source.setEncoding(this.encoding);
        }
        // Java: return this.read(source);
        todo!()
    }

    // Java: Document read(InputStream in)
    pub fn read_stream(&mut self, input: &mut dyn std::io::Read) -> Result<Box<dyn std::any::Any + Send + Sync>, Box<dyn std::error::Error>> {
        // Java: InputSource source = new InputSource(in); if (this.encoding != null) source.setEncoding(this.encoding);
        // Java: return this.read(source);
        // Rust: 底层 SAX 解析器由 JDK 的 `javax.xml.parsers.SAXParser` 换为本 crate 的 `XmlParse`（同层替代，登记 §1.7.1 ①）
        let mut bytes = Vec::new();
        input.read_to_end(&mut bytes)?;
        let text = decode_xml_bytes(&bytes, self.encoding.as_deref())?;
        let document: std::rc::Rc<std::cell::RefCell<Option<Box<dyn std::any::Any + Send + Sync>>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));
        let handler = crate::io::dom::io::SAXContentHandler::SAXContentHandler::new_with_document(
            std::rc::Rc::clone(&document),
        );
        let mut xml_parse = crate::io::opt::XmlParse::XmlParse::new(Box::new(handler), 0);
        // rust: Java 的 JDK `SAXParser` 原样上报字符（含空白）⇒ 打开保真开关（登记 §1.7.1 ⑩）
        xml_parse.setPreserveWhitespace(true);
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        xml_parse.parse(&chars, 0, len);
        let doc = document.borrow_mut().take().ok_or_else(|| {
            "DocumentException: 文档没有根元素（畸形或无内容的 XML）".to_string()
        })?;
        Ok(doc)
    }

    // Java: Document read(InputStream in, DefaultHandler handler)
    pub fn read_stream_with_handler(&mut self, _input: &mut dyn std::io::Read, _handler: &mut dyn std::any::Any) -> Result<Box<dyn std::any::Any + Send + Sync>, Box<dyn std::error::Error>> {
        // Java: if (this.encoding != null)
        {
        // Java: InputSource source = new InputSource(in); source.setEncoding(this.encoding);
        }
        // Java: return this.read(source, handler);
        todo!()
    }

    // Java: void mceUnderstandsNamespace(String ns)
    pub fn mceUnderstandsNamespace(&mut self, _ns: &str) {
        // Java: this.mMCEContext.addIntoUnderstandsSet(ns);
        todo!();
    }

    // Java: void setDisableMCE(boolean disable)
    pub fn setDisableMCE(&mut self, disable: bool) {
        // Java: this.mMCEContext.setDisableMCE(disable);
        self.mMCEContext.mMceDisabled = disable;
    }

    // Java: boolean getDisableMCE()
    pub fn getDisableMCE(&self) -> bool {
        // Java: return this.mMCEContext.getDisableMCE();
        self.mMCEContext.mMceDisabled
    }

    // Java: Document read(Reader reader)
    pub fn read_reader(&mut self, _reader: &mut dyn std::io::Read) -> Result<Box<dyn std::any::Any + Send + Sync>, Box<dyn std::error::Error>> {
        // Java: if (this.encoding != null)
        {
        // Java: InputSource source = new InputSource(reader); source.setEncoding(this.encoding);
        }
        // Java: return this.read(source);
        todo!()
    }

    // Java: Document read(InputStream in, String systemId)
    pub fn read_stream_with_system_id(&mut self, _input: &mut dyn std::io::Read, _system_id: &str) -> Result<Box<dyn std::any::Any + Send + Sync>, Box<dyn std::error::Error>> {
        // Java: InputSource source = new InputSource(in); source.setSystemId(systemId);
        // Java: if (this.encoding != null)
        {
            // Java: source.setEncoding(this.encoding);
        }
        // Java: return this.read(source);
        todo!()
    }

    // Java: Document read(Reader reader, String systemId)
    pub fn read_reader_with_system_id(&mut self, _reader: &mut dyn std::io::Read, _system_id: &str) -> Result<Box<dyn std::any::Any + Send + Sync>, Box<dyn std::error::Error>> {
        // Java: InputSource source = new InputSource(reader); source.setSystemId(systemId);
        // Java: if (this.encoding != null)
        {
            // Java: source.setEncoding(this.encoding);
        }
        // Java: return this.read(source);
        todo!()
    }

    // Java: Document read(InputSource in)
    pub fn read_input_source(&mut self, _input_source: &dyn std::any::Any) -> Result<Box<dyn std::any::Any + Send + Sync>, Box<dyn std::error::Error>> {
        // Java: try
        {
            // Java: XMLReader reader = this.getXMLReader();
            // Java: reader = this.installXMLFilter(reader); EntityResolver thatEntityResolver = this.entityResolver;
            // Java: if (thatEntityResolver == null)
            {
                // Java: thatEntityResolver = this.createDefaultEntityResolver(in.getSystemId());
                // Java: this.entityResolver = thatEntityResolver;
            }
            // Java: reader.setEntityResolver(thatEntityResolver);
            // Java: SAXContentHandler contentHandler = this.createContentHandler(reader);
            // Java: contentHandler.setEntityResolver(thatEntityResolver);
            // Java: contentHandler.setInputSource(in);
            // Java: contentHandler.setMCEContext(this.mMCEContext);
            // Java: boolean internal = this.isIncludeInternalDTDDeclarations();
            // Java: boolean external = this.isIncludeExternalDTDDeclarations();
            // Java: contentHandler.setIncludeInternalDTDDeclarations(internal);
            // Java: contentHandler.setIncludeExternalDTDDeclarations(external);
            // Java: contentHandler.setMergeAdjacentText(this.isMergeAdjacentText());
            // Java: contentHandler.setStripWhitespaceText(this.isStripWhitespaceText());
            // Java: contentHandler.setIgnoreComments(this.isIgnoreComments());
            // Java: reader.setContentHandler(contentHandler);
            // Java: this.configureReader(reader, contentHandler);
            // Java: reader.parse(in);
            // Java: return contentHandler.getDocument();
        }
        // Java: catch (Exception e)
        {
            // Java: if (e instanceof SAXParseException)
            {
                // Java: SAXParseException parseException = (SAXParseException)e; String systemId = parseException.getSystemId();
                // Java: if (systemId == null)
                {
                }
                    // Java: systemId = ""; String message = "Error on line " + parseException.getLineNumber() + " of document " + systemId + " : " + parseException.getMessage();
                // Java: throw new DocumentException(message, e);
            }
            // Java: else if (e instanceof DocumentEndOfParseException)
            {
                // Java: return null;
            }
            // Java: else
            {
                // Java: throw new DocumentException(e.getMessage(), e);
            }
        }
        todo!()
    }

    // Java: Document read(InputSource in, DefaultHandler handler)
    pub fn read_input_source_with_handler(&mut self, _input_source: &dyn std::any::Any, _handler: &mut dyn std::any::Any) -> Result<Box<dyn std::any::Any + Send + Sync>, Box<dyn std::error::Error>> {
        // Java: try
        {
            // Java: XMLReader reader = this.getXMLReader();
            // Java: reader = this.installXMLFilter(reader); EntityResolver thatEntityResolver = this.entityResolver;
            // Java: if (thatEntityResolver == null)
            {
                // Java: thatEntityResolver = this.createDefaultEntityResolver(in.getSystemId());
                // Java: this.entityResolver = thatEntityResolver;
            }
            // Java: reader.setEntityResolver(thatEntityResolver);
            // Java: SAXContentHandler contentHandler = this.createContentHandler(reader);
            // Java: contentHandler.setEntityResolver(thatEntityResolver);
            // Java: contentHandler.setInputSource(in);
            // Java: contentHandler.setMCEContext(this.mMCEContext);
            // Java: boolean internal = this.isIncludeInternalDTDDeclarations();
            // Java: boolean external = this.isIncludeExternalDTDDeclarations();
            // Java: contentHandler.setIncludeInternalDTDDeclarations(internal);
            // Java: contentHandler.setIncludeExternalDTDDeclarations(external);
            // Java: contentHandler.setMergeAdjacentText(this.isMergeAdjacentText());
            // Java: contentHandler.setStripWhitespaceText(this.isStripWhitespaceText());
            // Java: contentHandler.setIgnoreComments(this.isIgnoreComments());
            // Java: reader.setContentHandler(contentHandler);
            // Java: this.configureReader(reader, handler);
            // Java: reader.parse(in);
            // Java: return null;
        }
        // Java: catch (Exception e)
        {
            // Java: if (e instanceof SAXParseException)
            {
                // Java: SAXParseException parseException = (SAXParseException)e; String systemId = parseException.getSystemId();
                // Java: if (systemId == null)
                {
                }
                    // Java: systemId = ""; String message = "Error on line " + parseException.getLineNumber() + " of document " + systemId + " : " + parseException.getMessage();
                // Java: throw new DocumentException(message, e);
            }
            // Java: else if (e instanceof DocumentEndOfParseException)
            {
                // Java: return null;
            }
            // Java: else
            {
                // Java: throw new DocumentException(e.getMessage(), e);
            }
        }
        todo!()
    }

    // Java: boolean isValidating()
    pub fn isValidating(&self) -> bool {
        // Java: return this.validating;
        self.validating
    }

    // Java: void setValidation(boolean validation)
    pub fn setValidation(&mut self, validation: bool) {
        // Java: this.validating = validation;
        self.validating = validation;
    }

    // Java: boolean isIncludeInternalDTDDeclarations()
    pub fn isIncludeInternalDTDDeclarations(&self) -> bool {
        // Java: return this.includeInternalDTDDeclarations;
        self.includeInternalDTDDeclarations
    }

    // Java: void setIncludeInternalDTDDeclarations(boolean include)
    pub fn setIncludeInternalDTDDeclarations(&mut self, include: bool) {
        // Java: this.includeInternalDTDDeclarations = include;
        self.includeInternalDTDDeclarations = include;
    }

    // Java: boolean isIncludeExternalDTDDeclarations()
    pub fn isIncludeExternalDTDDeclarations(&self) -> bool {
        // Java: return this.includeExternalDTDDeclarations;
        self.includeExternalDTDDeclarations
    }

    // Java: void setIncludeExternalDTDDeclarations(boolean include)
    pub fn setIncludeExternalDTDDeclarations(&mut self, include: bool) {
        // Java: this.includeExternalDTDDeclarations = include;
        self.includeExternalDTDDeclarations = include;
    }

    // Java: boolean isStringInternEnabled()
    pub fn isStringInternEnabled(&self) -> bool {
        // Java: return this.stringInternEnabled;
        self.stringInternEnabled
    }

    // Java: void setStringInternEnabled(boolean stringInternEnabled)
    pub fn setStringInternEnabled(&mut self, string_intern_enabled: bool) {
        // Java: this.stringInternEnabled = stringInternEnabled;
        self.stringInternEnabled = string_intern_enabled;
    }

    // Java: boolean isMergeAdjacentText()
    pub fn isMergeAdjacentText(&self) -> bool {
        // Java: return this.mergeAdjacentText;
        self.mergeAdjacentText
    }

    // Java: void setMergeAdjacentText(boolean mergeAdjacentText)
    pub fn setMergeAdjacentText(&mut self, merge_adjacent_text: bool) {
        // Java: this.mergeAdjacentText = mergeAdjacentText;
        self.mergeAdjacentText = merge_adjacent_text;
    }

    // Java: boolean isStripWhitespaceText()
    pub fn isStripWhitespaceText(&self) -> bool {
        // Java: return this.stripWhitespaceText;
        self.stripWhitespaceText
    }

    // Java: void setStripWhitespaceText(boolean stripWhitespaceText)
    pub fn setStripWhitespaceText(&mut self, strip_whitespace_text: bool) {
        // Java: this.stripWhitespaceText = stripWhitespaceText;
        self.stripWhitespaceText = strip_whitespace_text;
    }

    // Java: boolean isIgnoreComments()
    pub fn isIgnoreComments(&self) -> bool {
        // Java: return this.ignoreComments;
        self.ignoreComments
    }

    // Java: void setIgnoreComments(boolean ignoreComments)
    pub fn setIgnoreComments(&mut self, ignore_comments: bool) {
        // Java: this.ignoreComments = ignoreComments;
        self.ignoreComments = ignore_comments;
    }

    // Java: ErrorHandler getErrorHandler()
    pub fn getErrorHandler(&self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: return this.errorHandler;
        self.errorHandler.as_ref()
    }

    // Java: void setErrorHandler(ErrorHandler errorHandler)
    pub fn setErrorHandler(&mut self, error_handler: Option<Box<dyn std::any::Any + Send + Sync>>) {
        // Java: this.errorHandler = errorHandler;
        self.errorHandler = error_handler;
    }

    // Java: EntityResolver getEntityResolver()
    pub fn getEntityResolver(&self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: return this.entityResolver;
        self.entityResolver.as_ref()
    }

    // Java: void setEntityResolver(EntityResolver entityResolver)
    pub fn setEntityResolver(&mut self, entity_resolver: Option<Box<dyn std::any::Any + Send + Sync>>) {
        // Java: this.entityResolver = entityResolver;
        self.entityResolver = entity_resolver;
    }

    // Java: XMLReader getXMLReader()
    pub fn getXMLReader(&mut self) -> &Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: if (this.xmlReader == null)
        if self.xmlReader.is_none() {
            // Java: this.xmlReader = this.createXMLReader();
        }
        // Java: return this.xmlReader;
        &self.xmlReader
    }

    // Java: void setXMLReader(XMLReader reader)
    pub fn setXMLReader(&mut self, reader: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.xmlReader = reader;
        self.xmlReader = Some(reader);
    }

    // Java: String getEncoding()
    pub fn getEncoding(&self) -> Option<&String> {
        // Java: return this.encoding;
        self.encoding.as_ref()
    }

    // Java: void setEncoding(String encoding)
    pub fn setEncoding(&mut self, encoding: Option<String>) {
        // Java: this.encoding = encoding;
        self.encoding = encoding;
    }

    // Java: void setXMLReaderClassName(String xmlReaderClassName)
    pub fn setXMLReaderClassName(&mut self, _xml_reader_class_name: &str) {
        // Java: XMLReader xmlReader = XMLReaderFactory.createXMLReader(xmlReaderClassName);
        // Java: XmlFeatureSecUtil.setFeatureXml(xmlReader);
        // Java: this.setXMLReader(xmlReader);
        todo!();
    }

    // Java: void addHandler(String path, ElementHandler handler)
    pub fn addHandler(&mut self, _path: &str, _handler: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.getDispatchHandler().addHandler(path, handler);
        todo!();
    }

    // Java: void removeHandler(String path)
    pub fn removeHandler(&mut self, _path: &str) {
        // Java: if (this.dispatchHandler != null)
        if let Some(ref mut _dispatch_handler) = self.dispatchHandler {
            // Java: this.dispatchHandler.removeHandler(path);
            todo!();
        }
    }

    // Java: void setDefaultHandler(ElementHandler handler)
    pub fn setDefaultHandler(&mut self, _handler: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.getDispatchHandler().setDefaultHandler(handler);
        todo!();
    }

    // Java: void resetHandlers()
    pub fn resetHandlers(&mut self) {
        // Java: if (this.dispatchHandler != null)
        if let Some(ref mut _dispatch_handler) = self.dispatchHandler {
            // Java: this.dispatchHandler.resetHandlers();
            todo!();
        }
    }

    // Java: XMLFilter getXMLFilter()
    pub fn getXMLFilter(&self) -> Option<&Box<dyn std::any::Any + Send + Sync>> {
        // Java: return this.xmlFilter;
        self.xmlFilter.as_ref()
    }

    // Java: void setXMLFilter(XMLFilter filter)
    pub fn setXMLFilter(&mut self, filter: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.xmlFilter = filter;
        self.xmlFilter = Some(filter);
    }

    // Java: XMLReader installXMLFilter(XMLReader reader)
    pub fn installXMLFilter(&mut self, reader: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
        // Java: XMLFilter filter = this.getXMLFilter();
        // Java: if (filter == null)
        if self.xmlFilter.is_none() {
            // Java: return reader;
            return reader;
        }
        // Java: else
        {
            // Java: while(true)
            {
            // Java: XMLFilter root = filter; XMLReader parent = root.getParent();
                // Java: if (!(parent instanceof XMLFilter))
                {
                    // Java: root.setParent(reader);
                    // Java: try
                    {
                        // Java: XmlFeatureSecUtil.setFeatureXml(filter);
                    }
                    // Java: catch (SAXNotRecognizedException e)
                    {
                        // Java: e.printStackTrace();
                    }
                    // Java: catch (SAXNotSupportedException e)
                    {
                        // Java: e.printStackTrace();
                    }
                    // Java: catch (Exception var7)
                    {
                    }
                // Java: return filter; root = (XMLFilter)parent;
                }
            }
        }
        todo!()
    }

    // Java: DispatchHandler getDispatchHandler()
    pub fn getDispatchHandler(&mut self) -> &mut DispatchHandler {
        // Java: if (this.dispatchHandler == null)
        if self.dispatchHandler.is_none() {
            // Java: this.dispatchHandler = new DispatchHandler();
            self.dispatchHandler = Some(DispatchHandler::new());
        }
        // Java: return this.dispatchHandler;
        self.dispatchHandler.as_mut().unwrap()
    }

    // Java: void setDispatchHandler(DispatchHandler dispatchHandler)
    pub fn setDispatchHandler(&mut self, dispatch_handler: DispatchHandler) {
        // Java: this.dispatchHandler = dispatchHandler;
        self.dispatchHandler = Some(dispatch_handler);
    }

    // Java: void resetDispatchHandler()
    pub fn resetDispatchHandler(&mut self) {
        // Java: this.dispatchHandler = null;
        self.dispatchHandler = None;
    }

    // Java: XMLReader createXMLReader()
    pub fn createXMLReader(&self) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: return SAXHelper.createXMLReader(this.isValidating());
        todo!()
    }

    // Java: void configureReader(XMLReader reader, DefaultHandler handler)
    pub fn configureReader(&self, _reader: &mut dyn std::any::Any, _handler: &mut dyn std::any::Any) {
        // Java: SAXHelper.setParserProperty(reader, "http://xml.org/sax/properties/lexical-handler", handler);
        // Java: if (this.includeInternalDTDDeclarations || this.includeExternalDTDDeclarations)
        {
            // Java: SAXHelper.setParserProperty(reader, "http://xml.org/sax/properties/declaration-handler", handler);
        }
        // Java: SAXHelper.setParserFeature(reader, "http://xml.org/sax/features/namespaces", true);
        // Java: SAXHelper.setParserFeature(reader, "http://xml.org/sax/features/namespace-prefixes", false);
        // Java: SAXHelper.setParserFeature(reader, "http://xml.org/sax/features/string-interning", this.isStringInternEnabled());
        // Java: SAXHelper.setParserFeature(reader, "http://xml.org/sax/features/use-locator2", true);
        // Java: try
        {
            // Java: reader.setFeature("http://xml.org/sax/features/validation", this.isValidating());
            // Java: if (this.errorHandler != null)
            {
                // Java: reader.setErrorHandler(this.errorHandler);
            }
            // Java: else
            {
                // Java: reader.setErrorHandler(handler);
            }
        }
        // Java: catch (Exception e)
        {
            // Java: if (this.isValidating())
            {
                // Java: throw new DocumentException("Validation not supported for XMLReader: " + reader, e);
            }
        }
    }

    // Java: SAXContentHandler createContentHandler(XMLReader reader)
    pub fn createContentHandler(&self, _reader: &dyn std::any::Any) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: return new SAXContentHandler(this.dispatchHandler, this.mOnlyUseName);
        todo!()
    }

    // Java: EntityResolver createDefaultEntityResolver(String systemId)
    pub fn createDefaultEntityResolver(&self, system_id: Option<&str>) -> SAXEntityResolver {
        // Java: String prefix = null;
        let mut prefix: Option<String> = None;
        // Java: if (systemId != null && systemId.length() > 0)
        if let Some(sid) = system_id {
            if !sid.is_empty() {
                // Java: int idx = systemId.lastIndexOf(47);
                let idx = sid.rfind('/');
                // Java: if (idx > 0)
                if let Some(i) = idx {
                    if i > 0 {
                        // Java: prefix = systemId.substring(0, idx + 1);
                        prefix = Some(sid[..i + 1].to_string());
                    }
                }
            }
        }
        // Java: return new SAXEntityResolver(prefix);
        SAXEntityResolver::new(prefix)
    }

}

// Java: public class SAXEntityResolver
impl SAXEntityResolver {
    // Java: SAXEntityResolver(String uriPrefix)
    pub fn new(uri_prefix: Option<String>) -> Self {
        // Java: super();
        // Java: this.uriPrefix = uriPrefix;
        Self { uriPrefix: uri_prefix }
    }

    // Java: InputSource resolveEntity(String publicId, String systemId)
    pub fn resolveEntity(&self, _public_id: Option<&str>, system_id: Option<&str>) -> Option<String> {
                // Java: if (systemId != null && systemId.length() > 0 && this.uriPrefix != null && systemId.indexOf(58) <= 0) systemId = this.uriPrefix + systemId;
        let mut resolved_id = system_id.map(|s| s.to_string());
        if let (Some(sid), Some(prefix)) = (system_id, &self.uriPrefix) {
            if !sid.is_empty() && !sid.contains(':') {
                resolved_id = Some(format!("{}{}", prefix, sid));
            }
        }
        // Java: return new InputSource(systemId);
        resolved_id
    }

}

// Java: `InputSource`/`SAXParser` 负责的字节 → 字符解码（Rust 侧就地实现；登记 §1.7.1 ④）
fn decode_xml_bytes(bytes: &[u8], _encoding: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    // Java: 支持 UTF-8（含 BOM）与 UTF-16（BOM 探测）
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Ok(String::from_utf8(bytes[3..].to_vec())?);
    }
    if bytes.starts_with(&[0xFF, 0xFE]) || bytes.starts_with(&[0xFE, 0xFF]) {
        let big_endian = bytes.starts_with(&[0xFE, 0xFF]);
        let mut units: Vec<u16> = Vec::new();
        let body = &bytes[2..];
        let mut i = 0;
        while i + 1 < body.len() {
            let u = if big_endian {
                u16::from_be_bytes([body[i], body[i + 1]])
            } else {
                u16::from_le_bytes([body[i], body[i + 1]])
            };
            units.push(u);
            i += 2;
        }
        return String::from_utf16(&units).map_err(|e| e.to_string().into());
    }
    Ok(String::from_utf8(bytes.to_vec())?)
}
