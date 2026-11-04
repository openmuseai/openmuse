// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable::Localizable;
// Java: import org.apache.commons.math3.exception.util.ExceptionContext
use crate::org::apache::commons::math3::exception::util::ExceptionContext::ExceptionContext;
// Java: import org.apache.commons.math3.exception.util.ExceptionContextProvider
use crate::org::apache::commons::math3::exception::util::ExceptionContextProvider::ExceptionContextProvider;

// Java: public class for
#[allow(non_camel_case_types)]
pub struct r#for {
}

// Java: public class MathIllegalArgumentException extends IllegalArgumentException
pub struct MathIllegalArgumentException {
    context: ExceptionContext,
}

// Java: public class for
impl r#for {
}

// Java: public class MathIllegalArgumentException extends IllegalArgumentException
#[allow(non_snake_case)]
impl MathIllegalArgumentException {
    // Java: MathIllegalArgumentException(Localizable pattern, Object ... args)
    pub fn new(pattern: Box<dyn Localizable>, args: Vec<String>) -> Self {
        let mut context = ExceptionContext::new("MathIllegalArgumentException".to_string());
        context.addMessage(pattern, args);
        MathIllegalArgumentException { context }
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

impl ExceptionContextProvider for MathIllegalArgumentException {
    fn getContext(&self) -> &ExceptionContext {
        &self.context
    }
}

impl std::fmt::Display for MathIllegalArgumentException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.context.getMessage())
    }
}

impl std::fmt::Debug for MathIllegalArgumentException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MathIllegalArgumentException: {}", self.context.getMessage())
    }
}

impl std::error::Error for MathIllegalArgumentException {}
