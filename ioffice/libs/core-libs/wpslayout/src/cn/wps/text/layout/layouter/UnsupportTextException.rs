// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class UnsupportTextException extends Exception
pub struct UnsupportTextException {
    pub(crate) message: Option<String>,
}

// Java: public class UnsupportTextException extends Exception
impl UnsupportTextException {
    // Java: UnsupportTextException()
    pub fn new() -> Self {
      // Java: super();
      UnsupportTextException { message: None }
    }

    // Java: UnsupportTextException(String detailMsg)
    pub fn new_with_message(detailMsg: String) -> Self {
      // Java: super(detailMsg);
      UnsupportTextException { message: Some(detailMsg) }
    }

}
