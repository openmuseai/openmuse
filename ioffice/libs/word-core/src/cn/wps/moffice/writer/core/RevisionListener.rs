#![allow(non_snake_case, dead_code, unused_imports)]

// java public static enum RevisionChange  (KRevisionChange.RevisionChange)
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum RevisionChange {
    #[default]
    ACCEPT,
    REJECT,
}

// java public static enum RevisionType  (KRevisionChange.RevisionType)
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum RevisionType {
    #[default]
    INSERT,
    DELETE,
    RUN_FORMAT,
    PARAGRAPH_FORMAT,
    TABLE_FORMAT,
    SECTION_FORMAT,
}

// java public interface RevisionListener
pub trait RevisionListener {
    // java  void beforeChangeAll(int var1, int var2, KRevisionChange.RevisionChange var3)
    fn beforeChangeAll(&mut self, var1: i32, var2: i32, var3: &RevisionChange);

    // java  void afterChangeAll(int var1, int var2, KRevisionChange.RevisionChange var3)
    fn afterChangeAll(&mut self, var1: i32, var2: i32, var3: &RevisionChange);

    // java  void beforeChange(int var1, int var2, KRevisionChange.RevisionType var3, KRevisionChange.RevisionChange var4)
    fn beforeChange(&mut self, var1: i32, var2: i32, var3: &RevisionType, var4: &RevisionChange);

    // java  void afterChange(int var1, int var2, KRevisionChange.RevisionType var3, KRevisionChange.RevisionChange var4)
    fn afterChange(&mut self, var1: i32, var2: i32, var3: &RevisionType, var4: &RevisionChange);
}
