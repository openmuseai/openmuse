// java public final class ColorToolKit {
// Method slice only: palette search/stateful conversion is not part of this slice.
pub struct ColorToolKit;
impl ColorToolKit{
    // java public static boolean isPaletteIndex(int value) { return (value >> 24 & 255) == 0; }
    pub fn isPaletteIndex(value:i32)->bool{(value>>24&255)==0}
    // java public static boolean isValidColor(int value) { return value != 32767; }
    pub fn isValidColor(value:i32)->bool{value!=32767}
}
