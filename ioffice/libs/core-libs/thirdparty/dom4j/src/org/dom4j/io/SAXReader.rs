// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import java.io.FileNotFoundException
// Java: import java.io.InputStream
// Java: import java.io.Reader
// Java: import java.io.Serializable
// Java: import java.net.URL
// Java: import org.dom4j.Document
// Java: import org.dom4j.DocumentException
// Java: import org.dom4j.DocumentFactory
// Java: import org.dom4j.ElementHandler
// Java: import org.xml.sax.EntityResolver
// Java: import org.xml.sax.ErrorHandler
// Java: import org.xml.sax.InputSource
// Java: import org.xml.sax.SAXException
// Java: import org.xml.sax.SAXParseException
// Java: import org.xml.sax.XMLFilter
// Java: import org.xml.sax.XMLReader
// Java: import org.xml.sax.helpers.DefaultHandler
// Java: import org.xml.sax.helpers.XMLReaderFactory

use super::MCEContext::MCEContext;

// Java field: String SAX_STRING_INTERNING
const SAX_STRING_INTERNING: &str = "http://xml.org/sax/features/string-interning";
// Java field: String SAX_NAMESPACE_PREFIXES
const SAX_NAMESPACE_PREFIXES: &str = "http://xml.org/sax/features/namespace-prefixes";
// Java field: String SAX_NAMESPACES
const SAX_NAMESPACES: &str = "http://xml.org/sax/features/namespaces";
// Java field: String SAX_DECL_HANDLER
const SAX_DECL_HANDLER: &str = "http://xml.org/sax/properties/declaration-handler";
// Java field: String SAX_LEXICAL_HANDLER
const SAX_LEXICAL_HANDLER: &str = "http://xml.org/sax/properties/lexical-handler";

// Java: public class SAXReader
pub struct SAXReader {
    // Java field: DocumentFactory factory
    factory: i32,
    // Java field: XMLReader xmlReader
    xmlReader: i32,
    // Java field: boolean validating
    validating: bool,
    // Java field: DispatchHandler dispatchHandler
    dispatchHandler: i32,
    // Java field: ErrorHandler errorHandler
    errorHandler: i32,
    // Java field: EntityResolver entityResolver
    entityResolver: i32,
    // Java field: boolean stringInternEnabled
    stringInternEnabled: bool,
    // Java field: boolean includeInternalDTDDeclarations
    includeInternalDTDDeclarations: bool,
    // Java field: boolean includeExternalDTDDeclarations
    includeExternalDTDDeclarations: bool,
    // Java field: boolean mergeAdjacentText
    mergeAdjacentText: bool,
    // Java field: boolean stripWhitespaceText
    stripWhitespaceText: bool,
    // Java field: boolean ignoreComments
    ignoreComments: bool,
    // Java field: String encoding
    encoding: Option<String>,
    // Java field: XMLFilter xmlFilter
    xmlFilter: i32,
    // Java field: boolean mOnlyUseName
    mOnlyUseName: bool,
    // Java field: MCEContext mMCEContext
    mMCEContext: MCEContext,
}

// Java: public class SAXEntityResolver
pub struct SAXEntityResolver {
    // Java field: String uriPrefix
    pub(crate) uriPrefix: Option<String>,
}

// Java: public class SAXReader
impl SAXReader {
    // Java: SAXReader()
    pub fn new() -> Self {
      // Java: super();
      SAXReader {
          factory: 0,
          xmlReader: 0,
          validating: false,
          dispatchHandler: 0,
          errorHandler: 0,
          entityResolver: 0,
          stringInternEnabled: true,
          includeInternalDTDDeclarations: false,
          includeExternalDTDDeclarations: false,
          mergeAdjacentText: false,
          stripWhitespaceText: false,
          ignoreComments: false,
          encoding: None,
          xmlFilter: 0,
          mOnlyUseName: false,
          mMCEContext: MCEContext::new(),
      }
    }

    // Java: SAXReader(boolean validating)
    pub fn new_validating(validating: bool) -> Self {
      // Java: super();
      // Java: this.validating = validating;
      let mut reader = Self::new();
      reader.validating = validating;
      reader
    }

    // Java: SAXReader(DocumentFactory factory)
    pub fn new_factory(_factory: i32) -> Self {
      // Java: super();
      // Java: this.factory = factory;
      let mut reader = Self::new();
      reader.factory = _factory;
      reader
    }

    // Java: SAXReader(DocumentFactory factory, boolean validating)
    pub fn new_factory_validating(_factory: i32, validating: bool) -> Self {
      // Java: super();
      // Java: this.factory = factory;
      // Java: this.validating = validating;
      let mut reader = Self::new();
      reader.factory = _factory;
      reader.validating = validating;
      reader
    }

    // Java: SAXReader(XMLReader xmlReader)
    pub fn new_xmlreader(_xml_reader: i32) -> Self {
      // Java: super();
      // Java: this.xmlReader = xmlReader;
      let mut reader = Self::new();
      reader.xmlReader = _xml_reader;
      reader
    }

    // Java: SAXReader(XMLReader xmlReader, boolean validating)
    pub fn new_xmlreader_validating(_xml_reader: i32, validating: bool) -> Self {
      // Java: super();
      // Java: this.xmlReader = xmlReader;
      // Java: this.validating = validating;
      let mut reader = Self::new();
      reader.xmlReader = _xml_reader;
      reader.validating = validating;
      reader
    }

    // Java: SAXReader(String xmlReaderClassName)
    pub fn new_classname(_xml_reader_class_name: &str) -> Self {
      // Java: super();
      // Java: if (xmlReaderClassName != null)
      // Java: this.xmlReader = XMLReaderFactory.createXMLReader(xmlReaderClassName);
      // Java: SAXHelper.setFeatureXml(this.xmlReader);
      Self::new()
    }

    // Java: SAXReader(String xmlReaderClassName, boolean validating)
    pub fn new_classname_validating(_xml_reader_class_name: &str, validating: bool) -> Self {
      // Java: super();
      // Java: if (xmlReaderClassName != null)
      // Java: this.xmlReader = XMLReaderFactory.createXMLReader(xmlReaderClassName);
      // Java: SAXHelper.setFeatureXml(this.xmlReader);
      // Java: this.validating = validating;
      let mut reader = Self::new();
      reader.validating = validating;
      reader
    }

    // Java: void setOnlyUseName(boolean val)
    pub fn setOnlyUseName(&mut self, val: bool) {
      // Java: this.mOnlyUseName = val;
      self.mOnlyUseName = val;
    }

    // Java: void setProperty(String name, Object value)
    pub fn setProperty(&mut self, _name: &str, _value: i32) {
      // Java: this.getXMLReader().setProperty(name, value);
      todo!()
    }

    // Java: void setFeature(String name, boolean value)
    pub fn setFeature(&mut self, _name: &str, _value: bool) {
      // Java: this.getXMLReader().setFeature(name, value);
      todo!()
    }

    // Java: Document read(File file)
    pub fn read_file(&mut self, _file: &str) -> i32 {
      // Java: try
      // Java: InputSource source = new InputSource(new FileInputStream(file));
      // Java: if (this.encoding != null)
      // Java: source.setEncoding(this.encoding);
      // Java: String path = file.getAbsolutePath();
      // Java: if (path != null)
      // Java: StringBuffer sb = new StringBuffer("file://");
      // Java: if (!path.startsWith(File.separator))
      // Java: sb.append("/");
      // Java: path = path.replace('\\', '/');
      // Java: sb.append(path);
      // Java: source.setSystemId(sb.toString());
      // Java: return this.read(source);
      // Java: catch (FileNotFoundException e)
      // Java: throw new DocumentException(e.getMessage(), e);
      todo!()
    }

    // Java: Document read(URL url)
    pub fn read_url(&mut self, _url: &str) -> i32 {
      // Java: String systemID = url.toExternalForm();
      // Java: InputSource source = new InputSource(systemID);
      // Java: if (this.encoding != null)
      {
         // Java: source.setEncoding(this.encoding);
      }
      // Java: return this.read(source);
      todo!()
    }

    // Java: Document read(String systemId)
    pub fn read_systemId(&mut self, _system_id: &str) -> i32 {
      // Java: InputSource source = new InputSource(systemId);
      // Java: if (this.encoding != null)
      {
         // Java: source.setEncoding(this.encoding);
      }
      // Java: return this.read(source);
      todo!()
    }

    // Java: Document read(InputStream in)
    pub fn read_inputStream(&mut self, _input: &mut dyn std::io::Read) -> i32 {
      // Java: InputSource source = new InputSource(in);
      // Java: if (this.encoding != null)
      {
         // Java: source.setEncoding(this.encoding);
      }
      // Java: return this.read(source);
      todo!()
    }

    // Java: void mceUnderstandsNamespace(String ns)
    pub fn mceUnderstandsNamespace(&mut self, ns: &str) {
      // Java: this.mMCEContext.addIntoUnderstandsSet(ns);
      self.mMCEContext.addIntoUnderstandsSet(ns);
    }

    // Java: void setDisableMCE(boolean disable)
    pub fn setDisableMCE(&mut self, disable: bool) {
      // Java: this.mMCEContext.setDisableMCE(disable);
      self.mMCEContext.setDisableMCE(disable);
    }

    // Java: boolean getDisableMCE()
    pub fn getDisableMCE(&self) -> bool {
      // Java: return this.mMCEContext.getDisableMCE();
      self.mMCEContext.getDisableMCE()
    }

    // Java: Document read(Reader reader)
    pub fn read_reader(&mut self, _reader: &mut dyn std::io::Read) -> i32 {
      // Java: InputSource source = new InputSource(reader);
      // Java: if (this.encoding != null)
      {
         // Java: source.setEncoding(this.encoding);
      }
      // Java: return this.read(source);
      todo!()
    }

    // Java: Document read(InputStream in, String systemId)
    pub fn read_inputStream_systemId(&mut self, _input: &mut dyn std::io::Read, _system_id: &str) -> i32 {
      // Java: InputSource source = new InputSource(in);
      // Java: source.setSystemId(systemId);
      // Java: if (this.encoding != null)
      {
         // Java: source.setEncoding(this.encoding);
      }
      // Java: return this.read(source);
      todo!()
    }

    // Java: Document read(Reader reader, String systemId)
    pub fn read_reader_systemId(&mut self, _reader: &mut dyn std::io::Read, _system_id: &str) -> i32 {
      // Java: InputSource source = new InputSource(reader);
      // Java: source.setSystemId(systemId);
      // Java: if (this.encoding != null)
      {
         // Java: source.setEncoding(this.encoding);
      }
      // Java: return this.read(source);
      todo!()
    }

    // Java: Document read(InputSource in)
    pub fn read(&mut self, _input: i32) -> i32 {
      // Java: try
      // Java: XMLReader reader = this.getXMLReader();
      // Java: reader = this.installXMLFilter(reader);
      // Java: EntityResolver thatEntityResolver = this.entityResolver;
      // Java: if (thatEntityResolver == null)
      // Java: thatEntityResolver = this.createDefaultEntityResolver(in.getSystemId());
      // Java: this.entityResolver = thatEntityResolver;
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
      // Java: catch (Exception e)
      // Java: if (e instanceof SAXParseException)
      // Java: SAXParseException parseException = (SAXParseException)e;
      // Java: String systemId = parseException.getSystemId();
      // Java: if (systemId == null)
      // Java: systemId = "";
      // Java: String message = "Error on line " + parseException.getLineNumber() + " of document " + systemId + " : " + parseException.getMessage();
      // Java: throw new DocumentException(message, e);
      // Java: else if (e instanceof DocumentEndOfParseException)
      // Java: return null;
      // Java: else
      // Java: throw new DocumentException(e.getMessage(), e);
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

    // Java: DocumentFactory getDocumentFactory()
    pub fn getDocumentFactory(&mut self) -> i32 {
      // Java: if (this.factory == null)
      // Java: this.factory = DocumentFactory.getInstance();
      // Java: return this.factory;
      self.factory
    }

    // Java: void setDocumentFactory(DocumentFactory documentFactory)
    pub fn setDocumentFactory(&mut self, document_factory: i32) {
      // Java: this.factory = documentFactory;
      self.factory = document_factory;
    }

    // Java: ErrorHandler getErrorHandler()
    pub fn getErrorHandler(&self) -> i32 {
      // Java: return this.errorHandler;
      self.errorHandler
    }

    // Java: void setErrorHandler(ErrorHandler errorHandler)
    pub fn setErrorHandler(&mut self, error_handler: i32) {
      // Java: this.errorHandler = errorHandler;
      self.errorHandler = error_handler;
    }

    // Java: EntityResolver getEntityResolver()
    pub fn getEntityResolver(&self) -> i32 {
      // Java: return this.entityResolver;
      self.entityResolver
    }

    // Java: void setEntityResolver(EntityResolver entityResolver)
    pub fn setEntityResolver(&mut self, entity_resolver: i32) {
      // Java: this.entityResolver = entityResolver;
      self.entityResolver = entity_resolver;
    }

    // Java: XMLReader getXMLReader()
    pub fn getXMLReader(&mut self) -> i32 {
      // Java: if (this.xmlReader == null)
      // Java: this.xmlReader = this.createXMLReader();
      // Java: return this.xmlReader;
      self.xmlReader
    }

    // Java: void setXMLReader(XMLReader reader)
    pub fn setXMLReader(&mut self, reader: i32) {
      // Java: this.xmlReader = reader;
      self.xmlReader = reader;
    }

    // Java: String getEncoding()
    pub fn getEncoding(&self) -> Option<&str> {
      // Java: return this.encoding;
      self.encoding.as_deref()
    }

    // Java: void setEncoding(String encoding)
    pub fn setEncoding(&mut self, encoding: &str) {
      // Java: this.encoding = encoding;
      self.encoding = Some(encoding.to_string());
    }

    // Java: void setXMLReaderClassName(String xmlReaderClassName)
    pub fn setXMLReaderClassName(&mut self, _xml_reader_class_name: &str) {
      // Java: XMLReader xmlReader = XMLReaderFactory.createXMLReader(xmlReaderClassName);
      // Java: SAXHelper.setFeatureXml(xmlReader);
      // Java: this.setXMLReader(xmlReader);
      todo!()
    }

    // Java: void addHandler(String path, ElementHandler handler)
    pub fn addHandler(&mut self, _path: &str, _handler: i32) {
      // Java: this.getDispatchHandler().addHandler(path, handler);
      todo!()
    }

    // Java: void removeHandler(String path)
    pub fn removeHandler(&mut self, _path: &str) {
      // Java: if (this.dispatchHandler != null)
      // Java: this.dispatchHandler.removeHandler(path);
    }

    // Java: void setDefaultHandler(ElementHandler handler)
    pub fn setDefaultHandler(&mut self, _handler: i32) {
      // Java: this.getDispatchHandler().setDefaultHandler(handler);
      todo!()
    }

    // Java: void resetHandlers()
    pub fn resetHandlers(&mut self) {
      // Java: if (this.dispatchHandler != null)
      // Java: this.dispatchHandler.resetHandlers();
    }

    // Java: XMLFilter getXMLFilter()
    pub fn getXMLFilter(&self) -> i32 {
      // Java: return this.xmlFilter;
      self.xmlFilter
    }

    // Java: void setXMLFilter(XMLFilter filter)
    pub fn setXMLFilter(&mut self, filter: i32) {
      // Java: this.xmlFilter = filter;
      self.xmlFilter = filter;
    }

    // Java: XMLReader installXMLFilter(XMLReader reader)
    pub fn installXMLFilter(&self, _reader: i32) -> i32 {
      // Java: XMLFilter filter = this.getXMLFilter();
      // Java: if (filter == null)
      // Java: return reader;
      // Java: else
      // Java: XMLFilter root = filter;
      // Java: while(true)
      // Java: XMLReader parent = root.getParent();
      // Java: if (!(parent instanceof XMLFilter))
      // Java: root.setParent(reader);
      // Java: try
      // Java: SAXHelper.setFeatureXml((XMLReader)filter);
      // Java: catch (Exception var5)
      // Java: return filter;
      // Java: root = (XMLFilter)parent;
      _reader
    }

    // Java: DispatchHandler getDispatchHandler()
    pub fn getDispatchHandler(&mut self) -> i32 {
      // Java: if (this.dispatchHandler == null)
      // Java: this.dispatchHandler = new DispatchHandler();
      // Java: return this.dispatchHandler;
      self.dispatchHandler
    }

    // Java: void setDispatchHandler(DispatchHandler dispatchHandler)
    pub fn setDispatchHandler(&mut self, dispatch_handler: i32) {
      // Java: this.dispatchHandler = dispatchHandler;
      self.dispatchHandler = dispatch_handler;
    }

    // Java: void resetDispatchHandler()
    pub fn resetDispatchHandler(&mut self) {
      // Java: this.dispatchHandler = null;
      self.dispatchHandler = 0;
    }

    // Java: XMLReader createXMLReader()
    pub fn createXMLReader(&self) -> i32 {
      // Java: return SAXHelper.createXMLReader(this.isValidating());
      todo!()
    }

    // Java: void configureReader(XMLReader reader, DefaultHandler handler)
    pub fn configureReader(&self, _reader: i32, _handler: i32) {
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
    pub fn createContentHandler(&self, _reader: i32) -> i32 {
      // Java: return new SAXContentHandler(this.getDocumentFactory(), this.dispatchHandler, this.mOnlyUseName);
      todo!()
    }

    // Java: EntityResolver createDefaultEntityResolver(String systemId)
    pub fn createDefaultEntityResolver(&self, system_id: Option<&str>) -> SAXEntityResolver {
      // Java: String prefix = null;
      let mut prefix: Option<String> = None;
      // Java: if (systemId != null && systemId.length() > 0)
      if let Some(sys_id) = system_id {
          if !sys_id.is_empty() {
              // Java: int idx = systemId.lastIndexOf(47);
              let idx = sys_id.rfind('/');
              // Java: if (idx > 0)
              if let Some(idx) = idx {
                  if idx > 0 {
                      // Java: prefix = systemId.substring(0, idx + 1);
                      prefix = Some(sys_id[..idx + 1].to_string());
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
      SAXEntityResolver {
          uriPrefix: uri_prefix,
      }
    }

    // Java: InputSource resolveEntity(String publicId, String systemId)
    pub fn resolveEntity(&self, _public_id: Option<&str>, system_id: Option<&str>) -> Option<String> {
      // Java: if (systemId != null && systemId.length() > 0 && this.uriPrefix != null && systemId.indexOf(58) <= 0)
      if let Some(sys_id) = system_id {
          if !sys_id.is_empty() {
              if let Some(uri_prefix) = &self.uriPrefix {
                  if !sys_id.contains(':') {
                      // Java: systemId = this.uriPrefix + systemId;
                      return Some(format!("{}{}", uri_prefix, sys_id));
                  }
              }
              // Java: return new InputSource(systemId);
              return Some(sys_id.to_string());
          }
      }
      // Java: return new InputSource(systemId);
      system_id.map(|s| s.to_string())
    }
}
