// Java: import java.io.ByteArrayInputStream
// Java: import java.io.IOException
// Java: import javax.xml.parsers.DocumentBuilderFactory
// Java: import javax.xml.parsers.ParserConfigurationException
// Java: import javax.xml.parsers.SAXParserFactory
// Java: import org.xml.sax.EntityResolver
// Java: import org.xml.sax.InputSource
// Java: import org.xml.sax.SAXException
// Java: import org.xml.sax.XMLFilter
// Java: import org.xml.sax.XMLReader

// Java: public class XmlFeatureSecUtil
pub struct XmlFeatureSecUtil;

// Java field: String XML_FEATURE_EXTERNAL_G_D
pub(crate) const XML_FEATURE_EXTERNAL_G_D: &str = "http://xml.org/sax/features/external-general-entities";
// Java field: String XML_FEATURE_EXTERNAL_P_E
pub(crate) const XML_FEATURE_EXTERNAL_P_E: &str = "http://xml.org/sax/features/external-parameter-entities";

impl XmlFeatureSecUtil {
    // Java: XmlFeatureSecUtil()
    fn new() -> Self {
        // Java: super();
        XmlFeatureSecUtil
    }

    // Java: void setFeatureXml(XMLFilter xmlReader)
    pub fn setFeatureXml_filter(_xml_reader: &mut dyn std::any::Any) {
        // Java: xmlReader.setFeature("http://xml.org/sax/features/external-general-entities", false);
        // Java: xmlReader.setFeature("http://xml.org/sax/features/external-parameter-entities", false);
        // Java: EntityResolver entityResolver = createEntityResolver();
        // Java: if (entityResolver != null)
        // Java: xmlReader.setEntityResolver(entityResolver);
    }

    // Java: void setFeatureXml(XMLReader xmlFilter)
    pub fn setFeatureXml_reader(_xml_filter: &mut dyn std::any::Any) {
        // Java: xmlFilter.setFeature("http://xml.org/sax/features/external-general-entities", false);
        // Java: xmlFilter.setFeature("http://xml.org/sax/features/external-parameter-entities", false);
        // Java: EntityResolver entityResolver = createEntityResolver();
        // Java: if (entityResolver != null)
        // Java: xmlFilter.setEntityResolver(entityResolver);
    }

    // Java: void setFeatureXml(DocumentBuilderFactory documentBuilderFactory)
    pub fn setFeatureXml_doc_builder(_document_builder_factory: &mut dyn std::any::Any) {
        // Java: try
        // Java: documentBuilderFactory.setFeature("http://xml.org/sax/features/external-general-entities", false);
        // Java: documentBuilderFactory.setFeature("http://xml.org/sax/features/external-parameter-entities", false);
        // Java: documentBuilderFactory.setFeature("http://javax.xml.XMLConstants/feature/secure-processing", true);
        // Java: catch (ParserConfigurationException var2)
    }

    // Java: void setFeatureXml(SAXParserFactory saxParserFactory)
    pub fn setFeatureXml_sax(_sax_parser_factory: &mut dyn std::any::Any) {
        // Java: saxParserFactory.setFeature("http://xml.org/sax/features/external-general-entities", false);
        // Java: saxParserFactory.setFeature("http://xml.org/sax/features/external-parameter-entities", false);
    }

    // Java: EntityResolver createEntityResolver()
    fn createEntityResolver() -> Option<Box<dyn std::any::Any>> {
        // Java: EntityResolver entityResolver = null;
        // Java: try
        // Java: entityResolver = new EntityResolver()
        // Java: ByteArrayInputStream byteIs = new ByteArrayInputStream("".getBytes("UTF-8"));
        // Java: public InputSource resolveEntity(String publicId, String systemId) throws SAXException, IOException
        // Java: return new InputSource(this.byteIs);
        // Java: catch (Exception var2)
        // Java: return entityResolver;
        None
    }
}
