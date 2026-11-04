// Java: public class LineInfo
pub struct LineInfo {
}

// Java: public class COMPOUND_LINE
pub struct COMPOUND_LINE {
}

// Java: public class PEN_ALIGN
pub struct PEN_ALIGN {
}

// Java: public class LINE_CAP
pub struct LINE_CAP {
}

// Java: public class LINE_END_LENGTH
pub struct LINE_END_LENGTH {
}

// Java: public class LINE_END
pub struct LINE_END {
}

// Java: public class LINE_END_HEAD_SIZE_TYPE
pub struct LINE_END_HEAD_SIZE_TYPE {
}

// Java: public class LINE_END_TAIL_SIZE_TYPE
pub struct LINE_END_TAIL_SIZE_TYPE {
}

// Java: public class LINE_END_WIDTH
pub struct LINE_END_WIDTH {
}

// Java: public class PRESET_LINE_DASH
pub struct PRESET_LINE_DASH {
}

// Java: public class LineInfo
impl LineInfo {
    // Java: LineInfo()
    pub fn new() -> Self {
      // Java: super();
      LineInfo {}
    }

}

// Java: public class COMPOUND_LINE
impl COMPOUND_LINE {
    // Java field: int SINGLE_LINE
    pub const SINGLE_LINE: i32 = 0;
    // Java field: int DOUBLE_LINES
    pub const DOUBLE_LINES: i32 = 1;
    // Java field: int THICK_THIN
    pub const THICK_THIN: i32 = 2;
    // Java field: int THIN_THICK
    pub const THIN_THICK: i32 = 3;
    // Java field: int THIN_THICK_THIN
    pub const THIN_THICK_THIN: i32 = 4;

    // Java: COMPOUND_LINE()
    pub fn new() -> Self {
         // Java: super();
         COMPOUND_LINE {}
    }

}

// Java: public class PEN_ALIGN
impl PEN_ALIGN {
    // Java field: int CENTER_ALIGN
    pub const CENTER_ALIGN: i32 = 0;
    // Java field: int INSET_ALIGN
    pub const INSET_ALIGN: i32 = 1;

    // Java: PEN_ALIGN()
    pub fn new() -> Self {
         // Java: super();
         PEN_ALIGN {}
    }

}

// Java: public class LINE_CAP
impl LINE_CAP {
    // Java field: int ROUND
    pub const ROUND: i32 = 0;
    // Java field: int SQUARE
    pub const SQUARE: i32 = 1;
    // Java field: int FLAT
    pub const FLAT: i32 = 2;

    // Java: LINE_CAP()
    pub fn new() -> Self {
         // Java: super();
         LINE_CAP {}
    }

}

// Java: public class LINE_END_LENGTH
impl LINE_END_LENGTH {
    // Java field: int SMALL
    pub const SMALL: i32 = 0;
    // Java field: int MEDIUM
    pub const MEDIUM: i32 = 1;
    // Java field: int LARGE
    pub const LARGE: i32 = 2;

    // Java: LINE_END_LENGTH()
    pub fn new() -> Self {
         // Java: super();
         LINE_END_LENGTH {}
    }

}

// Java: public class LINE_END
impl LINE_END {
    // Java field: int NONE
    pub const NONE: i32 = 0;
    // Java field: int TRIANGLE
    pub const TRIANGLE: i32 = 1;
    // Java field: int STEALTH
    pub const STEALTH: i32 = 2;
    // Java field: int DIAMOND
    pub const DIAMOND: i32 = 3;
    // Java field: int OVAL
    pub const OVAL: i32 = 4;
    // Java field: int ARROW
    pub const ARROW: i32 = 5;

    // Java: LINE_END()
    pub fn new() -> Self {
         // Java: super();
         LINE_END {}
    }

}

// Java: public class LINE_END_HEAD_SIZE_TYPE
impl LINE_END_HEAD_SIZE_TYPE {
    // Java field: int LEFT_ARROW_1
    pub const LEFT_ARROW_1: i32 = 0;
    // Java field: int LEFT_ARROW_2
    pub const LEFT_ARROW_2: i32 = 1;
    // Java field: int LEFT_ARROW_3
    pub const LEFT_ARROW_3: i32 = 2;
    // Java field: int LEFT_ARROW_4
    pub const LEFT_ARROW_4: i32 = 3;
    // Java field: int LEFT_ARROW_5
    pub const LEFT_ARROW_5: i32 = 4;
    // Java field: int LEFT_ARROW_6
    pub const LEFT_ARROW_6: i32 = 5;
    // Java field: int LEFT_ARROW_7
    pub const LEFT_ARROW_7: i32 = 6;
    // Java field: int LEFT_ARROW_8
    pub const LEFT_ARROW_8: i32 = 7;
    // Java field: int LEFT_ARROW_9
    pub const LEFT_ARROW_9: i32 = 8;

    // Java: LINE_END_HEAD_SIZE_TYPE()
    pub fn new() -> Self {
         // Java: super();
         LINE_END_HEAD_SIZE_TYPE {}
    }

}

// Java: public class LINE_END_TAIL_SIZE_TYPE
impl LINE_END_TAIL_SIZE_TYPE {
    // Java field: int RIGHT_ARROW_1
    pub const RIGHT_ARROW_1: i32 = 0;
    // Java field: int RIGHT_ARROW_2
    pub const RIGHT_ARROW_2: i32 = 1;
    // Java field: int RIGHT_ARROW_3
    pub const RIGHT_ARROW_3: i32 = 2;
    // Java field: int RIGHT_ARROW_4
    pub const RIGHT_ARROW_4: i32 = 3;
    // Java field: int RIGHT_ARROW_5
    pub const RIGHT_ARROW_5: i32 = 4;
    // Java field: int RIGHT_ARROW_6
    pub const RIGHT_ARROW_6: i32 = 5;
    // Java field: int RIGHT_ARROW_7
    pub const RIGHT_ARROW_7: i32 = 6;
    // Java field: int RIGHT_ARROW_8
    pub const RIGHT_ARROW_8: i32 = 7;
    // Java field: int RIGHT_ARROW_9
    pub const RIGHT_ARROW_9: i32 = 8;

    // Java: LINE_END_TAIL_SIZE_TYPE()
    pub fn new() -> Self {
         // Java: super();
         LINE_END_TAIL_SIZE_TYPE {}
    }

}

// Java: public class LINE_END_WIDTH
impl LINE_END_WIDTH {
    // Java field: int SMALL
    pub const SMALL: i32 = 0;
    // Java field: int MEDIUM
    pub const MEDIUM: i32 = 1;
    // Java field: int LARGE
    pub const LARGE: i32 = 2;

    // Java: LINE_END_WIDTH()
    pub fn new() -> Self {
         // Java: super();
         LINE_END_WIDTH {}
    }

}

// Java: public class PRESET_LINE_DASH
impl PRESET_LINE_DASH {
    // Java field: int SOLID
    pub const SOLID: i32 = 0;
    // Java field: int SYSTEM_DASH
    pub const SYSTEM_DASH: i32 = 1;
    // Java field: int SYSTEM_DOT
    pub const SYSTEM_DOT: i32 = 2;
    // Java field: int SYSTEM_DASH_DOT
    pub const SYSTEM_DASH_DOT: i32 = 3;
    // Java field: int SYSTEM_DASH_DOT_DOT
    pub const SYSTEM_DASH_DOT_DOT: i32 = 4;
    // Java field: int DOT
    pub const DOT: i32 = 5;
    // Java field: int DASH
    pub const DASH: i32 = 6;
    // Java field: int LARGE_DASH
    pub const LARGE_DASH: i32 = 7;
    // Java field: int DASH_DOT
    pub const DASH_DOT: i32 = 8;
    // Java field: int LARGE_DASH_DOT
    pub const LARGE_DASH_DOT: i32 = 9;
    // Java field: int LARGE_DASH_DOT_DOT
    pub const LARGE_DASH_DOT_DOT: i32 = 10;

    // Java: PRESET_LINE_DASH()
    pub fn new() -> Self {
         // Java: super();
         PRESET_LINE_DASH {}
    }

}
