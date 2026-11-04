// Java: public interface ISpellCheck
pub trait ISpellCheck {
    // Java: abstract void setDictPath(String var1)
    fn setDictPath(&mut self, _var1: &str);

    // Java: abstract IHunSpell getSpell(int var1)
    fn getSpell(&self, _var1: i32) -> Option<Box<dyn std::any::Any>>;

}
