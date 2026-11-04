#![allow(non_snake_case, dead_code, unused_imports)]

// java public enum BookmarkNodeType
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BookmarkNodeType {
    // java CommentNodeStart
    CommentNodeStart,
    // java CommentNodeEnd
    CommentNodeEnd,
    // java BookmarkNodeStart
    BookmarkNodeStart,
    // java BookmarkNodeEnd
    BookmarkNodeEnd,
}
