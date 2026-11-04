// Java: public interface NameStep extends Step
pub trait NameStep {
    // Java: abstract String getPrefix()
    fn getPrefix(&self) -> String;

    // Java: abstract String getLocalName()
    fn getLocalName(&self) -> String;

}
