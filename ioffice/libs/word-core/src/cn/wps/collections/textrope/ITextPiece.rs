// java public interface ITextPiece
pub trait ITextPiece {
    // java  int getCp()
    fn getCp(&self) -> i32;

    // java  int getFc()
    fn getFc(&self) -> i32;

    // java  int getLength()
    fn getLength(&self) -> i32;

    // java  ITextPiece getNext()
    fn getNext(&self) -> Option<Box<dyn ITextPiece>>; /* soft: owned trait obj */

    // java  ITextPiece getPrev()
    fn getPrev(&self) -> Option<Box<dyn ITextPiece>>; /* soft: owned trait obj */

    // java  boolean isEnd()
    fn isEnd(&self) -> bool;

}
