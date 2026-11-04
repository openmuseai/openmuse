// Java: import cn.wps.base.io.XmlFeatureSecUtil
// Java: import cn.wps.base.log.Log
// Java: import java.io.ByteArrayInputStream
// Java: import java.io.IOException
// Java: import javax.xml.parsers.DocumentBuilderFactory
// Java: import org.xml.sax.EntityResolver
// Java: import org.xml.sax.InputSource
// Java: import org.xml.sax.SAXException
// Java: import org.xml.sax.SAXNotRecognizedException
// Java: import org.xml.sax.SAXNotSupportedException
// Java: import org.xml.sax.XMLReader
// Java: import org.xml.sax.helpers.XMLReaderFactory
use std::sync::atomic::{AtomicBool, Ordering};

// Java field: String TAG
pub(crate) const TAG: &str = "SAXHelper";
// Java field: boolean loggedWarning
static LOGGED_WARNING: AtomicBool = AtomicBool::new(true);

// Java: public class SAXHelper
pub struct SAXHelper {
}

// Java: public class SAXHelper
impl SAXHelper {
    // Java: SAXHelper()
    pub fn new() -> Self {
        // Java: super();
        SAXHelper {}
    }

    // Java: static boolean setParserProperty(XMLReader reader, String propertyName, Object value)
    pub fn setParserProperty(_property_name: &str) -> bool {
        // Java: try
        {
            // Java: reader.setProperty(propertyName, value);
            // Java: return true;
        }
        // Java: catch (SAXNotSupportedException e)
        {
            // Java: Log.e(TAG, "SAXNotSupportedException", e);
        }
        // Java: catch (SAXNotRecognizedException e)
        {
            // Java: Log.e(TAG, "SAXNotRecognizedException", e);
        }
        // Java: return false;
        false
    }

    // Java: static boolean setParserFeature(XMLReader reader, String featureName, boolean value)
    pub fn setParserFeature(_feature_name: &str, _value: bool) -> bool {
        // Java: try
        {
            // Java: reader.setFeature(featureName, value);
            // Java: return true;
        }
        // Java: catch (SAXNotSupportedException var4)
        {
        }
        // Java: catch (SAXNotRecognizedException var5)
        {
        }
        // Java: return false;
        false
    }

    // Java: static XMLReader createXMLReader(boolean validating)
    pub fn createXMLReader(_validating: bool) {
        // Java: XMLReader reader = null;
        // Java: if (reader == null)
        {
            // Java: reader = createXMLReaderViaJAXP(validating, true);
        }
        // Java: if (reader == null)
        {
            // Java: try
            {
                // Java: reader = XMLReaderFactory.createXMLReader();
            }
            // Java: catch (Exception var3)
            {
                // Java: if (isVerboseErrorReporting())
                {
                    // Java: System.out.println("Warning: Caught exception attempting to use SAX to load a SAX XMLReader ");
                    // Java: System.out.println("Warning: Exception was: " + var3);
                    // Java: System.out.println("Warning: I will print the stack trace then carry on using the default SAX parser");
                    // Java: var3.printStackTrace();
                }
                // Java: throw new SAXException(var3);
            }
        }
        // Java: if (reader == null)
        {
            // Java: throw new SAXException("Couldn't create SAX reader");
        }
        // Java: else
        {
            // Java: setFeatureXml(reader);
            // Java: return reader;
        }
    }

    // Java: static void setFeatureXml(XMLReader xmlReader)
    pub fn setFeatureXml_reader() {
        // Java: XmlFeatureSecUtil.setFeatureXml(xmlReader);
    }

    // Java: static EntityResolver createEntityResolver()
    pub fn createEntityResolver() {
        // Java: EntityResolver entityResolver = null;
        // Java: try
        {
            // Java: entityResolver = new EntityResolver()
            {
                // Java: ByteArrayInputStream byteIs = new ByteArrayInputStream("".getBytes("UTF-8"));
                // Java: public InputSource resolveEntity(String publicId, String systemId) throws SAXException, IOException
                {
                    // Java: return new InputSource(this.byteIs);
                }
            }
        }
        // Java: catch (Exception var2)
        {
        }
        // Java: return entityResolver;
    }

    // Java: static void setFeatureXml(DocumentBuilderFactory documentBuilderFactory)
    pub fn setFeatureXml_factory() {
        // Java: XmlFeatureSecUtil.setFeatureXml(documentBuilderFactory);
    }

    // Java: static XMLReader createXMLReaderViaJAXP(boolean validating, boolean namespaceAware)
    pub fn createXMLReaderViaJAXP(_validating: bool, _namespace_aware: bool) {
        // Java: try
        {
            // Java: return JAXPHelper.createXMLReader(validating, namespaceAware);
        }
        // Java: catch (Throwable e)
        {
            // Java: if (!loggedWarning)
            if !LOGGED_WARNING.load(Ordering::Relaxed)
            {
                // Java: loggedWarning = true;
                LOGGED_WARNING.store(true, Ordering::Relaxed);
                // Java: if (isVerboseErrorReporting())
                {
                    // Java: System.out.println("Warning: Caught exception attempting to use JAXP to load a SAX XMLReader");
                    // Java: System.out.println("Warning: Exception was: " + e);
                    // Java: e.printStackTrace();
                }
            }
            // Java: return null;
        }
    }

    // Java: static boolean isVerboseErrorReporting()
    pub fn isVerboseErrorReporting() -> bool {
        // Java: try
        {
            // Java: String flag = System.getProperty("org.dom4j.verbose");
            // Java: if (flag != null && flag.equalsIgnoreCase("true"))
            {
                // Java: return true;
            }
        }
        // Java: catch (Exception var1)
        {
        }
        // Java: return true;
        true
    }
}
