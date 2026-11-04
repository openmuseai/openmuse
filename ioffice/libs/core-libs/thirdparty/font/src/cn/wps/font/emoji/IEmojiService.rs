// Java: public interface IEmojiService
pub trait IEmojiService {
    // Java: abstract void setEmoji(char var1, float var2)
    fn setEmoji_char(&self, var1: u16, var2: f32);

    // Java: abstract void setEmoji(char var1, char var2, float var3)
    fn setEmoji_char_char(&self, var1: u16, var2: u16, var3: f32);

    // Java: abstract void setEmoji(int var1, float var2)
    fn setEmoji_int(&self, var1: i32, var2: f32);

    // Java: abstract void setEmoji(char[] var1, int var2, int var3, float var4)
    fn setEmoji_chars(&self, var1: &[u16], var2: i32, var3: i32, var4: f32);

    // Java: abstract void draw(Object var1)
    fn draw(&self, var1: i64);

    // Java: abstract float getAscent()
    fn getAscent(&self) -> f32;

    // Java: abstract float getDescent()
    fn getDescent(&self) -> f32;

    // Java: abstract float getHeight()
    fn getHeight(&self) -> f32;

    // Java: abstract float getWidth()
    fn getWidth(&self) -> f32;

    // Java: abstract void dispose()
    fn dispose(&self);
}
