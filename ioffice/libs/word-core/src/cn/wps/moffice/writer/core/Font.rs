// java public interface Font
pub trait Font {
    // java  void setStyle(int var1)
    fn setStyle(&mut self, var1: i32);

    // java  int getStyle()
    fn getStyle(&self) -> i32;

    // java  void setBold(boolean var1)
    fn setBold(&mut self, var1: bool);

    // java  Boolean getBold()
    fn getBold(&self) -> Option<bool>;

    // java  void setItalic(boolean var1)
    fn setItalic(&mut self, var1: bool);

    // java  Boolean getItalic()
    fn getItalic(&self) -> Option<bool>;

    // java  void setUnderline(int var1)
    fn setUnderline(&mut self, var1: i32);

    // java  Integer getUnderline()
    fn getUnderline(&self) -> Option<i32>;

    // java  Boolean getStrikeThrough()
    fn getStrikeThrough(&self) -> Option<bool>;

    // java  Boolean getSubscript()
    fn getSubscript(&self) -> Option<bool>;

    // java  Boolean getSuperscript()
    fn getSuperscript(&self) -> Option<bool>;

    // java  Boolean getSmallCapital()
    fn getSmallCapital(&self) -> Option<bool>;

    // java  void setSmallCapital(boolean var1)
    fn setSmallCapital(&mut self, var1: bool);

    // java  Boolean getAllCapital()
    fn getAllCapital(&self) -> Option<bool>;

    // java  void setAllCapital(boolean var1)
    fn setAllCapital(&mut self, var1: bool);

}
