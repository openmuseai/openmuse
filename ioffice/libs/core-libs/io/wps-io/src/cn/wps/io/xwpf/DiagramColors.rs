// Java: public interface DiagramColors
pub trait DiagramColors {
    // Java: abstract String getUniqueFileName()
    fn getUniqueFileName(&self) -> String;

    // Java: abstract boolean dump(String var1)
    fn dump(&self, var1: &str) -> bool;

}
