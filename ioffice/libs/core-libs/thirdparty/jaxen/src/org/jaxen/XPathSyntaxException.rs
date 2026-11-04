use super::JaxenException::JaxenException;

// Java field: long serialVersionUID
// Java field: String xpath
// Java field: int position

// Java: public class XPathSyntaxException extends JaxenException
pub struct XPathSyntaxException {
    pub(crate) base: JaxenException,
    xpath: String,
    position: i32,
}

// Java: public class XPathSyntaxException extends JaxenException
impl XPathSyntaxException {
    // Java: XPathSyntaxException(org.jaxen.saxpath.XPathSyntaxException e)
    pub fn from_saxpath(xpath: String, position: i32, message: String) -> Self {
      // Java: super((Throwable)e);
      // Java: this.xpath = e.getXPath();
      // Java: this.position = e.getPosition();
      XPathSyntaxException {
         base: JaxenException::new(message),
         xpath,
         position,
      }
    }

    // Java: XPathSyntaxException(String xpath, int position, String message)
    pub fn new(xpath: String, position: i32, message: String) -> Self {
      // Java: super(message);
      // Java: this.xpath = xpath;
      // Java: this.position = position;
      XPathSyntaxException {
         base: JaxenException::new(message),
         xpath,
         position,
      }
    }

    // Java: int getPosition()
    pub fn getPosition(&self) -> i32 {
      // Java: return this.position;
      self.position
    }

    // Java: String getXPath()
    pub fn getXPath(&self) -> &str {
      // Java: return this.xpath;
      &self.xpath
    }

    // Java: String getPositionMarker()
    pub fn getPositionMarker(&self) -> String {
      // Java: StringBuffer buf = new StringBuffer();
      let mut buf = String::new();
      // Java: int pos = this.getPosition();
      let pos = self.getPosition();
      // Java: for(int i = 0; i < pos; ++i)
      for _i in 0..pos {
         // Java: buf.append(" ");
         buf.push(' ');
      }
      // Java: buf.append("^");
      buf.push('^');
      // Java: return buf.toString();
      buf
    }

    // Java: String getMultilineMessage()
    pub fn getMultilineMessage(&self) -> String {
      // Java: StringBuffer buf = new StringBuffer(this.getMessage());
      let mut buf = String::from(&self.base.message);
      // Java: buf.append("\n");
      buf.push('\n');
      // Java: buf.append(this.getXPath());
      buf.push_str(self.getXPath());
      // Java: buf.append("\n");
      buf.push('\n');
      // Java: buf.append(this.getPositionMarker());
      buf.push_str(&self.getPositionMarker());
      // Java: return buf.toString();
      buf
    }
}

impl std::fmt::Display for XPathSyntaxException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.getMultilineMessage())
    }
}

impl std::fmt::Debug for XPathSyntaxException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XPathSyntaxException: {}", self.getMultilineMessage())
    }
}

impl std::error::Error for XPathSyntaxException {}
