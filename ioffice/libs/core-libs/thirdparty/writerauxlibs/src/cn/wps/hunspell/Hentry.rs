// Java: public class Hentry
pub struct Hentry {
    // Java field: String word
    pub(crate) word: String,
    // Java field: char[] aff
    pub(crate) aff: Vec<char>,
    // Java field: Hentry next
    pub(crate) next: Option<Box<Hentry>>,
}

// Java: public class Hentry
impl Hentry {
    // Java: Hentry(String word, char[] aff)
    pub fn new(word: String, aff: Vec<char>) -> Self {
        // Java: super();
        // Java: this.word = word;
        // Java: this.aff = aff;
        // Java: this.next = null;
        Self {
            word,
            aff,
            next: None,
        }
    }
}
