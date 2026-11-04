#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.io.XmlFeatureSecUtil
// Java: import cn.wps.base.log.IOLog
// Java: import org.xml.sax.SAXException
// Java: import org.xml.sax.SAXNotRecognizedException
// Java: import org.xml.sax.SAXNotSupportedException
// Java: import org.xml.sax.XMLReader
// Java: import org.xml.sax.helpers.XMLReaderFactory

// Java: public class SAXHelper
pub struct SAXHelper {
    _private: (),
}

// Java field: String TAG
static TAG: Option<&str> = None;
// Java field: boolean loggedWarning
static mut LOGGED_WARNING: bool = true;

// Java: public class SAXHelper
impl SAXHelper {
    // Java: SAXHelper()
    pub fn new() -> Self {
        // Java: super();
        SAXHelper { _private: () }
    }

    // Java: static boolean setParserProperty(XMLReader reader, String propertyName, Object value)
    pub fn setParserProperty(_reader: &mut dyn std::any::Any, _property_name: &str, _value: Box<dyn std::any::Any>) -> bool {
        // Java: try
        // Java: reader.setProperty(propertyName, value);
        // Java: return true;
        // Java: catch (SAXNotSupportedException e)
        // Java: IOLog.e(TAG, "SAXNotSupportedException", e);
        // Java: catch (SAXNotRecognizedException e)
        // Java: IOLog.e(TAG, "SAXNotRecognizedException", e);
        // Java: return false;
        todo!()
    }

    // Java: static boolean setParserFeature(XMLReader reader, String featureName, boolean value)
    pub fn setParserFeature(_reader: &mut dyn std::any::Any, _feature_name: &str, _value: bool) -> bool {
        // Java: try
        // Java: reader.setFeature(featureName, value);
        // Java: return true;
        // Java: catch (SAXNotSupportedException var4)
        // Java: catch (SAXNotRecognizedException var5)
        // Java: return false;
        todo!()
    }

    // Java: static XMLReader createXMLReader(boolean validating)
    pub fn createXMLReader(_validating: bool) -> Result<Box<dyn std::any::Any + Send + Sync>, String> {
        // Java: XMLReader reader = null;
        // Java: if (reader == null)
        // Java: reader = createXMLReaderViaJAXP(validating, true);
        // Java: if (reader == null)
        // Java: try
        // Java: reader = XMLReaderFactory.createXMLReader();
        // Java: catch (Exception var3)
        // Java: if (isVerboseErrorReporting())
        // Java: System.out.println("Warning: Caught exception attempting to use SAX to load a SAX XMLReader ");
        // Java: System.out.println("Warning: Exception was: " + var3);
        // Java: System.out.println("Warning: I will print the stack trace then carry on using the default SAX parser");
        // Java: var3.printStackTrace();
        // Java: throw new SAXException(var3);
        // Java: if (reader == null)
        // Java: throw new SAXException("Couldn't create SAX reader");
        // Java: else
        // Java: XmlFeatureSecUtil.setFeatureXml(reader);
        // Java: return reader;
        Ok(Box::new(()))
    }

    // Java: static XMLReader createXMLReaderViaJAXP(boolean validating, boolean namespaceAware)
    fn createXMLReaderViaJAXP(_validating: bool, _namespace_aware: bool) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: try
        // Java: return JAXPHelper.createXMLReader(validating, namespaceAware);
        // Java: catch (Throwable e)
        // Java: if (!loggedWarning)
        // Java: loggedWarning = true;
        // Java: if (isVerboseErrorReporting())
        // Java: System.out.println("Warning: Caught exception attempting to use JAXP to load a SAX XMLReader");
        // Java: System.out.println("Warning: Exception was: " + e);
        // Java: e.printStackTrace();
        // Java: return null;
        todo!()
    }

    // Java: static boolean isVerboseErrorReporting()
    fn isVerboseErrorReporting() -> bool {
        // Java: try
        // Java: String flag = System.getProperty("cn.wps.io.dom.verbose");
        // Java: if (flag != null && flag.equalsIgnoreCase("true"))
        // Java: return true;
        // Java: catch (Exception var1)
        // Java: return true;
        true
    }
}
