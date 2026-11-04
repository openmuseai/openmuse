use crate::org::jaxen::saxpath::SAXPathException::SAXPathException;
use crate::org::jaxen::saxpath::XPathReader::XPathReader;

// Java field: String DRIVER_PROPERTY
pub const DRIVER_PROPERTY: &str = "org.saxpath.driver";
// Java field: String DEFAULT_DRIVER
pub(crate) const DEFAULT_DRIVER: &str = "org.jaxen.saxpath.base.XPathReader";

// Java: public class XPathReaderFactory
pub struct XPathReaderFactory;

// Java: public class XPathReaderFactory
impl XPathReaderFactory {
    // Java: XPathReaderFactory()
    fn new() -> Self {
        // Java: super();
        XPathReaderFactory
    }

    // Java: static XPathReader createReader()
    pub fn createReader() -> Result<Box<dyn XPathReader>, SAXPathException> {
        // Java: String className = null;
        let mut class_name: Option<String> = None;
        // Java: try
        // Java: className = System.getProperty("org.saxpath.driver");
        // Java: catch (SecurityException var2)
        // (no system properties in Rust, skip)
        // Java: if (className == null || className.length() == 0)
        if class_name.as_ref().map_or(true, |s| s.is_empty()) {
            // Java: className = "org.jaxen.saxpath.base.XPathReader";
            class_name = Some(DEFAULT_DRIVER.to_string());
        }
        // Java: return createReader(className);
        Self::createReader_with_class(&class_name.unwrap())
    }

    // Java: static XPathReader createReader(String className)
    pub fn createReader_with_class(className: &str) -> Result<Box<dyn XPathReader>, SAXPathException> {
        // Java: Class readerClass = null;
        // Java: XPathReader reader = null;
        // Java: try
        // Java: readerClass = Class.forName(className, true, XPathReaderFactory.class.getClassLoader());
        // Java: if (!XPathReader.class.isAssignableFrom(readerClass))
        if className != DEFAULT_DRIVER {
            // Java: throw new SAXPathException("Class [" + className + "] does not implement the org.jaxen.saxpath.XPathReader interface.");
            return Err(SAXPathException::new(
                format!("Class [{}] does not implement the org.jaxen.saxpath.XPathReader interface.", className)
            ));
        }
        // Java: catch (ClassNotFoundException e)
        // Java: throw new SAXPathException(e);

        // Java: try
        // Java: reader = (XPathReader)readerClass.newInstance();
        // Java: return reader;
        Err(SAXPathException::new(
            format!("Cannot instantiate XPathReader class: {}", className)
        ))
        // Java: catch (IllegalAccessException e)
        // Java: throw new SAXPathException(e);
        // Java: catch (InstantiationException e)
        // Java: throw new SAXPathException(e);
    }
}
