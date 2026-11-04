// java private static final long serialVersionUID = -965459879744468707L;
const serialVersionUID: i64 = -965459879744468700;

// java public class InvalidPositionException extends RuntimeException
#[derive(Default)]
pub struct InvalidPositionException {
    // java RuntimeException super_
    pub(crate) super_: RuntimeException,
}

// java public class InvalidPositionException extends RuntimeException
impl InvalidPositionException {
    // java public  InvalidPositionException()
    pub fn new() -> Self {
      Self { super_: RuntimeException::new() }
    }

    // java public  InvalidPositionException(String message)
    pub fn new__string(message: &str) -> Self {
      // java super(message);
      Self { super_: RuntimeException::new__string(message) }
    }

}

impl std::ops::Deref for InvalidPositionException {
    type Target = RuntimeException;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for InvalidPositionException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
