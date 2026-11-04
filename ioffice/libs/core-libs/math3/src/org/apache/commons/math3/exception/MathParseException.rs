// Java: import org.apache.commons.math3.exception.util.ExceptionContextProvider
use crate::org::apache::commons::math3::exception::util::ExceptionContextProvider;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -6024911025449780478;

// Java: public class MathParseException extends MathIllegalStateException
#[derive(Debug, Clone)]
pub struct MathParseException {
    pub(crate) wrong: String,
    pub(crate) position: i32,
    pub(crate) type_name: Option<String>,
}

// Java: public class MathParseException extends MathIllegalStateException
impl MathParseException {
    // Java: MathParseException(String wrong, int position, Class<?> type)
    pub fn new_with_type(wrong: String, position: i32, type_name: String) -> Self {
        MathParseException { wrong, position, type_name: Some(type_name) }
    }

    // Java: MathParseException(String wrong, int position)
    pub fn new(wrong: String, position: i32) -> Self {
        MathParseException { wrong, position, type_name: None }
    }
}

impl std::fmt::Display for MathParseException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.type_name {
            Some(t) => write!(f, "Cannot parse '{}' at position {} as type {}", self.wrong, self.position, t),
            None => write!(f, "Cannot parse '{}' at position {}", self.wrong, self.position),
        }
    }
}

impl std::error::Error for MathParseException {}
