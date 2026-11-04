// Java: import org.dom4j.DocumentHelper
// use crate::org::dom4j::DocumentHelper;
// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.QName
// use crate::org::dom4j::QName;
// Java: import org.xml.sax.ErrorHandler
// use crate::org::xml::sax::ErrorHandler;
// Java: import org.xml.sax.SAXParseException
// use crate::org::xml::sax::SAXParseException;

// Java field: QName ERROR_QNAME
// Java field: QName FATALERROR_QNAME
// Java field: QName WARNING_QNAME

// Java: public class XMLErrorHandler implements ErrorHandler
pub struct XMLErrorHandler {
    // Java field: Element errors
    pub(crate) errors: i32,
    // Java field: QName errorQName
    pub(crate) errorQName: i32,
    // Java field: QName fatalErrorQName
    pub(crate) fatalErrorQName: i32,
    // Java field: QName warningQName
    pub(crate) warningQName: i32,
}

// Java: public class XMLErrorHandler implements ErrorHandler
impl XMLErrorHandler {
    // Java: XMLErrorHandler()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.errorQName = ERROR_QNAME;
      // Java: this.fatalErrorQName = FATALERROR_QNAME;
      // Java: this.warningQName = WARNING_QNAME;
      // Java: this.errors = DocumentHelper.createElement("errors");
      Self { errors: 0, errorQName: 0, fatalErrorQName: 0, warningQName: 0 }
    }

    // Java: XMLErrorHandler(Element errors)
    pub fn new_with_errors(_errors: i32) -> Self {
      // Java: super();
      // Java: this.errorQName = ERROR_QNAME;
      // Java: this.fatalErrorQName = FATALERROR_QNAME;
      // Java: this.warningQName = WARNING_QNAME;
      // Java: this.errors = errors;
      Self { errors: _errors, errorQName: 0, fatalErrorQName: 0, warningQName: 0 }
    }

    // Java: void error(SAXParseException e)
    pub fn error(&self, _e: &str) {
      // Java: Element element = this.errors.addElement(this.errorQName);
      // Java: this.addException(element, e);
    }

    // Java: void fatalError(SAXParseException e)
    pub fn fatalError(&self, _e: &str) {
      // Java: Element element = this.errors.addElement(this.fatalErrorQName);
      // Java: this.addException(element, e);
    }

    // Java: void warning(SAXParseException e)
    pub fn warning(&self, _e: &str) {
      // Java: Element element = this.errors.addElement(this.warningQName);
      // Java: this.addException(element, e);
    }

    // Java: Element getErrors()
    pub fn getErrors(&self) -> i32 {
      // Java: return this.errors;
      self.errors
    }

    // Java: void setErrors(Element errors)
    pub fn setErrors(&mut self, _errors: i32) {
      // Java: this.errors = errors;
      self.errors = _errors;
    }

    // Java: QName getErrorQName()
    pub fn getErrorQName(&self) -> i32 {
      // Java: return this.errorQName;
      self.errorQName
    }

    // Java: void setErrorQName(QName errorQName)
    pub fn setErrorQName(&mut self, _errorQName: i32) {
      // Java: this.errorQName = errorQName;
      self.errorQName = _errorQName;
    }

    // Java: QName getFatalErrorQName()
    pub fn getFatalErrorQName(&self) -> i32 {
      // Java: return this.fatalErrorQName;
      self.fatalErrorQName
    }

    // Java: void setFatalErrorQName(QName fatalErrorQName)
    pub fn setFatalErrorQName(&mut self, _fatalErrorQName: i32) {
      // Java: this.fatalErrorQName = fatalErrorQName;
      self.fatalErrorQName = _fatalErrorQName;
    }

    // Java: QName getWarningQName()
    pub fn getWarningQName(&self) -> i32 {
      // Java: return this.warningQName;
      self.warningQName
    }

    // Java: void setWarningQName(QName warningQName)
    pub fn setWarningQName(&mut self, _warningQName: i32) {
      // Java: this.warningQName = warningQName;
      self.warningQName = _warningQName;
    }

    // Java: void addException(Element element, SAXParseException e)
    pub fn addException(&self, _element: i32, _e: &str) {
      // Java: element.addAttribute("column", Integer.toString(e.getColumnNumber()));
      // Java: element.addAttribute("line", Integer.toString(e.getLineNumber()));
      // Java: String publicID = e.getPublicId();
      // Java: if (publicID != null && publicID.length() > 0)
      {
         // Java: element.addAttribute("publicID", publicID);
      }
      // Java: String systemID = e.getSystemId();
      // Java: if (systemID != null && systemID.length() > 0)
      {
         // Java: element.addAttribute("systemID", systemID);
      }
      // Java: element.addText(e.getMessage());
    }

}

// impl ErrorHandler for XMLErrorHandler {
// }
