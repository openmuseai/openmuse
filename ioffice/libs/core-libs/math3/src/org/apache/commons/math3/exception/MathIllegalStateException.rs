// Java: import org.apache.commons.math3.exception.util.Localizable
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::Localizable::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::LocalizedFormats::LocalizedFormats;
// Java: import org.apache.commons.math3.exception.util.ExceptionContext
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::ExceptionContext::ExceptionContext;
// Java: import org.apache.commons.math3.exception.util.ExceptionContextProvider
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::ExceptionContextProvider::ExceptionContextProvider;

// Java: public class for
#[allow(non_camel_case_types)]
pub struct r#for {}

// Java: public class MathIllegalStateException
pub struct MathIllegalStateException {
    context: ExceptionContext,
}

// Java: public class for
impl r#for {}

// Java: public class MathIllegalStateException
#[allow(non_snake_case)]
impl MathIllegalStateException {
    pub fn new(pattern: Box<dyn Localizable>, args: Vec<String>) -> Self {
        let mut context = ExceptionContext::new("MathIllegalStateException".to_string());
        context.addMessage(pattern, args);
        MathIllegalStateException { context }
    }

    pub fn getMessage(&self) -> String { self.context.getMessage() }
}

impl std::fmt::Display for MathIllegalStateException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.context.getMessage())
    }
}
impl std::fmt::Debug for MathIllegalStateException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MathIllegalStateException: {}", self.context.getMessage())
    }
}
impl std::error::Error for MathIllegalStateException {}
