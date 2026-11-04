// Java: public interface HLInnerAccepter
pub trait HLInnerAccepter {
    // Java: void label(char[] chars, int start, int end)
    fn label(&mut self, chars: &[char], start: usize, end: usize);

    // Java: void characters(char[] chars, int start, int end, boolean bEscape)
    fn characters(&mut self, chars: &[char], start: usize, end: usize, b_escape: bool);
}
