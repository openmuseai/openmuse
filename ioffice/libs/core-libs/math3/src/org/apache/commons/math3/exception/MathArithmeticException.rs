// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats::LocalizedFormats;
// Java: import org.apache.commons.math3.exception.util.ExceptionContext
use crate::org::apache::commons::math3::exception::util::ExceptionContext::ExceptionContext;
// Java: import org.apache.commons.math3.exception.util.ExceptionContextProvider
use crate::org::apache::commons::math3::exception::util::ExceptionContextProvider::ExceptionContextProvider;

// Java: public class for
#[allow(non_camel_case_types)]
pub struct r#for {
}

// Java: public class MathArithmeticException extends ArithmeticException
pub struct MathArithmeticException {
    context: ExceptionContext,
}

// Java: public class for
impl r#for {
}

// Java: public class MathArithmeticException extends ArithmeticException
#[allow(non_snake_case)]
impl MathArithmeticException {
    // Java: MathArithmeticException()
    pub fn new() -> Self {
        let mut context = ExceptionContext::new("MathArithmeticException".to_string());
        context.addMessage(
            Box::new(LocalizedFormats::ARITHMETIC_EXCEPTION()),
            vec![],
        );
        MathArithmeticException { context }
    }

    // Java: MathArithmeticException(Localizable pattern, Object ... args)
    pub fn new_with_pattern(pattern: Box<dyn Localizable>, args: Vec<String>) -> Self {
        let mut context = ExceptionContext::new("MathArithmeticException".to_string());
        context.addMessage(pattern, args);
        MathArithmeticException { context }
    }

    // Java: ExceptionContext getContext()
    pub fn getContext(&self) -> &ExceptionContext {
        &self.context
    }

    // Java: String getMessage()
    pub fn getMessage(&self) -> String {
        self.context.getMessage()
    }

    // Java: String getLocalizedMessage()
    pub fn getLocalizedMessage(&self) -> String {
        self.context.getLocalizedMessage()
    }
}

impl ExceptionContextProvider for MathArithmeticException {
    fn getContext(&self) -> &ExceptionContext {
        &self.context
    }
}

impl std::fmt::Display for MathArithmeticException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.context.getMessage())
    }
}

impl std::fmt::Debug for MathArithmeticException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MathArithmeticException: {}", self.context.getMessage())
    }
}

impl std::error::Error for MathArithmeticException {}
