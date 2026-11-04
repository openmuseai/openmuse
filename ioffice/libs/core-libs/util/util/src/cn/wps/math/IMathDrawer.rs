use crate::graphics::RectF::RectF;

// Java: public interface IMathDrawer
pub trait IMathDrawer: Send + Sync {
    // Java: abstract boolean draw(String var1, RectF var2, int var3, int var4, int var5)
    fn draw(&self, var1: &str, var2: &RectF, var3: i32, var4: i32, var5: i32) -> bool;
}
