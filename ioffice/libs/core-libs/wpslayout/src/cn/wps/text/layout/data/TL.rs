// Java: public class TL
pub struct TL {
}

// Java: public class LINE_LAYOUT
pub struct LINE_LAYOUT {
}

// Java: public class PLACEHOLDER
pub struct PLACEHOLDER {
}

// Java: public class TEXT_ANCHOR
pub struct TEXT_ANCHOR {
}

// Java: public class TL
impl TL {
    // Java: TL()
    pub fn new() -> Self {
      // Java: super();
      TL {}
    }

}

// Java: public class LINE_LAYOUT
impl LINE_LAYOUT {
    // Java field: float LINE_SPC_REDUCTION
    pub const LINE_SPC_REDUCTION: f32 = 0.2;
    // Java field: float ONCE_SPC_REDUCTION
    pub const ONCE_SPC_REDUCTION: f32 = 0.1;

    // Java: LINE_LAYOUT()
    pub fn new() -> Self {
         // Java: super();
         LINE_LAYOUT {}
    }

}

// Java: public class PLACEHOLDER
impl PLACEHOLDER {
    // Java field: int TITLE
    pub const TITLE: i32 = 0;
    // Java field: int BODY
    pub const BODY: i32 = 1;
    // Java field: int CTR_TITLE
    pub const CTR_TITLE: i32 = 2;
    // Java field: int SUB_TITLE
    pub const SUB_TITLE: i32 = 3;
    // Java field: int DT
    pub const DT: i32 = 4;
    // Java field: int SLDNUM
    pub const SLDNUM: i32 = 5;
    // Java field: int FTR
    pub const FTR: i32 = 6;
    // Java field: int HDR
    pub const HDR: i32 = 7;
    // Java field: int OBJ
    pub const OBJ: i32 = 8;
    // Java field: int CHART
    pub const CHART: i32 = 9;
    // Java field: int TBL
    pub const TBL: i32 = 10;
    // Java field: int CLIPART
    pub const CLIPART: i32 = 11;
    // Java field: int DGM
    pub const DGM: i32 = 12;
    // Java field: int MEDIA
    pub const MEDIA: i32 = 13;
    // Java field: int SLDIMG
    pub const SLDIMG: i32 = 14;
    // Java field: int PIC
    pub const PIC: i32 = 15;

    // Java: PLACEHOLDER()
    pub fn new() -> Self {
         // Java: super();
         PLACEHOLDER {}
    }

}

// Java: public class TEXT_ANCHOR
impl TEXT_ANCHOR {
    // Java field: int TOP
    pub const TOP: i32 = 0;
    // Java field: int CTR
    pub const CTR: i32 = 1;
    // Java field: int BOTTOM
    pub const BOTTOM: i32 = 2;
    // Java field: int JUST
    pub const JUST: i32 = 3;
    // Java field: int DIST
    pub const DIST: i32 = 4;

    // Java: TEXT_ANCHOR()
    pub fn new() -> Self {
         // Java: super();
         TEXT_ANCHOR {}
    }

}
