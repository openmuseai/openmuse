// Java: import java.util.ArrayList

// Java: public interface IHunSpell
pub trait IHunSpell {
    // Java: abstract int spell(String var1)
    fn spell(&self, _var1: &str) -> i32;

    // Java: abstract ArrayList<String> suggest(String var1)
    fn suggest(&self, _var1: &str) -> Vec<String>;

    // Java: abstract void addUsrWord(String var1)
    fn addUsrWord(&mut self, _var1: &str);

}
