use std::sync::Arc;

// Java: public interface Entry
pub trait Entry {
    // Java: abstract String getName()
    fn getName(&self) -> String;

    // Java: abstract boolean isDirectoryEntry()
    fn isDirectoryEntry(&self) -> bool;

    // Java: abstract boolean isDocumentEntry()
    fn isDocumentEntry(&self) -> bool;

    // Java: abstract DirectoryEntry getParent()
    fn getParent(&self) -> Option<Arc<dyn DirectoryEntry>>;

    // Java: abstract boolean delete()
    fn delete(&mut self) -> bool;

    // Java: abstract boolean renameTo(String var1)
    fn renameTo(&mut self, var1: &str) -> bool;

    // Java: abstract void dispose()
    fn dispose(&mut self);
}

pub trait DirectoryEntry: Entry {}
