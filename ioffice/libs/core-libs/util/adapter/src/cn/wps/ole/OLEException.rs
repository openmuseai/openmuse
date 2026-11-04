// Java: import java.io.IOException

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 1;

// Java: public class OLEException extends IOException
pub struct OLEException {
    pub(crate) message: String,
}

// Java: public class OLEException extends IOException
impl OLEException {
    // Java: OLEException(String what)
    pub fn new(_what: &str) -> Self {
      // Java: super(what);
      OLEException {
          message: _what.to_string(),
      }
    }

}
