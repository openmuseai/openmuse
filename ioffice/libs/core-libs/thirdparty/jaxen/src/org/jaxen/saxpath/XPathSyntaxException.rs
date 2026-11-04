use super::SAXPathException::SAXPathException;

// Java field: long serialVersionUID
// Java field: String xpath
// Java field: int position
// Java field: String lineSeparator

// Java: public class XPathSyntaxException extends SAXPathException
#[derive(Debug)]
pub struct XPathSyntaxException {
    pub(crate) base: SAXPathException,
    pub(crate) xpath: String,
    pub(crate) position: i32,
}

// Java: public class XPathSyntaxException extends SAXPathException
impl XPathSyntaxException {
    // Java: XPathSyntaxException(String xpath, int position, String message)
    pub fn new(xpath: String, position: i32, message: String) -> Self {
      // Java: super(message);
      // Java: this.position = position;
      // Java: this.xpath = xpath;
      XPathSyntaxException {
         base: SAXPathException::new(message),
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

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return this.getClass() + ": " + this.getXPath() + ": " + this.getPosition() + ": " + this.getMessage();
      format!("XPathSyntaxException: {}: {}: {}", self.getXPath(), self.getPosition(), self.base.message)
    }

    // Java: String getPositionMarker()
    fn getPositionMarker(&self) -> String {
      // Java: int pos = this.getPosition();
      let pos = self.getPosition();
      // Java: StringBuffer buf = new StringBuffer(pos + 1);
      let mut buf = String::with_capacity((pos + 1) as usize);
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
      // Java: StringBuffer buf = new StringBuffer();
      let mut buf = String::new();
      // Java: buf.append(this.getMessage());
      buf.push_str(&self.base.message);
      // Java: buf.append(lineSeparator);
      buf.push('\n');
      // Java: buf.append(this.getXPath());
      buf.push_str(self.getXPath());
      // Java: buf.append(lineSeparator);
      buf.push('\n');
      // Java: buf.append(this.getPositionMarker());
      buf.push_str(&self.getPositionMarker());
      // Java: return buf.toString();
      buf
    }
}

impl std::fmt::Display for XPathSyntaxException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.toString())
    }
}

impl std::error::Error for XPathSyntaxException {}
