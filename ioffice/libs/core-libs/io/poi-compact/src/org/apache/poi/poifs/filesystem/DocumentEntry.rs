use super::Entry::Entry;

// Java: public interface DocumentEntry extends Entry
pub trait DocumentEntry: Entry {
    // Java: abstract int getSize()
    fn getSize(&self) -> i32;
}
