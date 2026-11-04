#![allow(non_snake_case)]

/// wr-12 I/O: wr-02 content_width + PAPX 208 first-line indent → this line's mMaxWidth.
///
/// Java: `SubLineLayouter.flow` indent block. wr-02 already subtracted left/right
/// indent (210/212). This function must not subtract them again.
pub struct ContentWidth {}

impl ContentWidth {
    // java propEnv.leftIndent = prop.getInt(210, 0);
    // java propEnv.rightIndent = prop.getInt(212, 0);
    pub fn paraMaxWidth(page_content: i32, left_indent: i32, right_indent: i32) -> i32 {
        let w = page_content - left_indent - right_indent;
        if w < 0 {
            0
        } else {
            w
        }
    }

    // java if (firstSubLine) { subLineEnv.layoutLeft += paraPropEnv.firstLineIndent; }
    // java this.mMaxWidth = subLineEnv.layoutRight - subLineEnv.layoutLeft;
    pub fn lineMaxWidth(content_width: i32, first_line_indent: i32, first_sub_line: bool) -> i32 {
        let w = if first_sub_line {
            content_width - first_line_indent
        } else {
            content_width
        };
        if w < 0 {
            0
        } else {
            w
        }
    }
}
