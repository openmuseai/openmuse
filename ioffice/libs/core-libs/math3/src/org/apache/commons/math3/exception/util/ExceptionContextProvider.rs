use super::ExceptionContext::ExceptionContext;

// Java: public interface ExceptionContextProvider
pub trait ExceptionContextProvider {
    // Java: abstract ExceptionContext getContext()
    fn getContext(&self) -> &ExceptionContext;
}
