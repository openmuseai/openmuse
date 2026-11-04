use super::UopContext::UopContext;

// Java: public interface IUopImporter extends IUofImporter
pub trait IUopImporter {
    // Java: abstract UopContext getContext()
    fn getContext(&self) -> UopContext;

}
