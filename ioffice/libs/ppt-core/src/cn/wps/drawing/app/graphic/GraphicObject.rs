// Java: public interface GraphicObject
pub trait GraphicObject {
    // Java: abstract int type()
    // LLM Rust Renamed: type -> r#type (Rust keyword)
    fn r#type(&self) -> i32;

    // Java: abstract GraphicObject copy()
    fn copy(&self) -> Box<dyn GraphicObject>;

}
