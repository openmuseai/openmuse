use super::{
    PieceContext::PieceContext,
    PlaceHolderID::PlaceHolderID,
    PlaceHolderToken::{PlaceHolderToken, TokenRef},
};
// java public abstract class PlaceHolderPiece
pub struct PlaceHolderPiece {
    pub mIdentifer: u16,
    pub mMatchBuffer: Vec<u16>,
    pub mMatchCount: i32,
    pub mHolderToken: Option<TokenRef>,
    pub mDefaultHolder: i16,
    pub mCurPlaceHolder: i16,
    pub mIgnoreCase: bool,
}
impl PlaceHolderPiece {
    // java this.mIdentifer=identifer;this.mDefaultHolder=holderID;this.mMatchBuffer="";
    pub fn new(identifer: char, holderID: i16) -> Self {
        Self {
            mIdentifer: identifer as u16,
            mDefaultHolder: holderID,
            mMatchBuffer: Vec::new(),
            mMatchCount: 0,
            mHolderToken: None,
            mCurPlaceHolder: 0,
            mIgnoreCase: false,
        }
    }
    // java this.mMatchBuffer="";this.mMatchCount=0;this.mCurPlaceHolder=-1;this.mHolderToken=null;
    pub fn clear(&mut self) {
        self.mMatchBuffer.clear();
        self.mMatchCount = 0;
        self.mCurPlaceHolder = -1;
        self.mHolderToken = None;
    }
    // java protected void match(String formatCode,int start,int end)
    pub fn match_base(&mut self, code: &[u16], start: usize, end: usize) {
        for i in start..end {
            let c = code[i];
            let flipped=if crate::native_runtime::Character::isUpperCase(c){crate::native_runtime::Character::toLowerCase(c)}else{crate::native_runtime::Character::toUpperCase(c)};
            // java if(c!=this.mIdentifer&&(!this.mIgnoreCase||this.mIdentifer!=(Character.isUpperCase(c)?Character.toLowerCase(c):Character.toUpperCase(c)))) break;
            if c != self.mIdentifer && (!self.mIgnoreCase || self.mIdentifer != flipped) {
                break;
            }
            self.mMatchCount += 1;
        }
        self.mMatchBuffer = code[start..start + self.mMatchCount as usize].to_vec();
    }
}
// Rust dispatch for original protected overrides; every concrete Java piece
// retains its own struct, superclass and method implementations.
pub trait Piece {
    // Native vtable for the original PieceVariousChar downcast in VariantPieceDic.
    fn find_various(&mut self, _code:&[u16], _start:usize, _end:usize, _ignore:bool)->bool {panic!("ClassCastException")}
    fn base(&self) -> &PlaceHolderPiece;
    fn base_mut(&mut self) -> &mut PlaceHolderPiece;
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        self.base_mut().match_base(code, start, end);
    }
    fn matchToHolderID(&self) -> i16 {
        -1
    }
    fn check(&mut self) {}
    fn setPieceContext(&self, p: &mut PieceContext);
    // java identify: clear; mIgnoreCase=ignoreCase;match;setPlaceHolder;setPieceContext;return mMatchCount.
    fn identify(
        &mut self,
        code: &[u16],
        start: usize,
        end: usize,
        ignoreCase: bool,
        p: &mut PieceContext,
    ) -> i32 {
        self.base_mut().clear();
        self.base_mut().mIgnoreCase = ignoreCase;
        self.match_piece(code, start, end);
        self.setPlaceHolder();
        self.setPieceContext(p);
        self.base().mMatchCount
    }
    // java analyze additionally calls check() between match() and setPlaceHolder().
    fn analyze(&mut self, code: &[u16], start: usize, end: usize, p: &mut PieceContext) -> i32 {
        self.base_mut().clear();
        self.base_mut().mIgnoreCase = true;
        self.match_piece(code, start, end);
        self.check();
        self.setPlaceHolder();
        self.setPieceContext(p);
        self.base().mMatchCount
    }
    fn setPlaceHolder(&mut self) {
        // java short holderID=this.matchToHolderID();this.mCurPlaceHolder=holderID<0?this.mDefaultHolder:holderID;
        let id = self.matchToHolderID();
        let b = self.base_mut();
        b.mCurPlaceHolder = if id < 0 { b.mDefaultHolder } else { id };
        // java if(this.mMatchCount>0&&this.mCurPlaceHolder> -1){this.mHolderToken=PlaceHolderToken.obtain();...}
        if b.mMatchCount > 0 && b.mCurPlaceHolder > -1 {
            let token = PlaceHolderToken::obtain();
            {
                let mut data = token
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                data.mToken =
                    String::from_utf16(&b.mMatchBuffer).expect("unpaired UTF16 format token");
                data.mPlaceHolder = Some(PlaceHolderID::getPlaceHolder(b.mCurPlaceHolder));
            }
            b.mHolderToken = Some(token);
        }
    }
}

// java public static class PieceSharp extends PlaceHolderPiece
pub struct PieceSharp {
    pub base: PlaceHolderPiece,
}
impl PieceSharp {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 2),
        }
    }
}
impl Piece for PieceSharp {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|512);
        p.mPieceType |= 512;
    }
}

// java public static class PieceAsk extends PlaceHolderPiece
pub struct PieceAsk {
    pub base: PlaceHolderPiece,
}
impl PieceAsk {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 4),
        }
    }
}
impl Piece for PieceAsk {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|512);
        p.mPieceType |= 512;
    }
    // java if(this.mMatchBuffer.charAt(0)!='?')this.mMatchBuffer=new String(fillChar('?',...));
    fn check(&mut self) {
        if self.base.mMatchBuffer[0] != 63 {
            self.base.mMatchBuffer.fill(63);
        }
    }
}

// java public static class PieceThousSep extends PlaceHolderPiece
pub struct PieceThousSep {
    pub base: PlaceHolderPiece,
}
impl PieceThousSep {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 8),
        }
    }
}
impl Piece for PieceThousSep {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|128);
        p.mPieceType |= 128;
    }
}

// java public static class PieceFraction extends PlaceHolderPiece
pub struct PieceFraction {
    pub base: PlaceHolderPiece,
}
impl PieceFraction {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 5),
        }
    }
}
impl Piece for PieceFraction {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|8);
        p.mPieceType |= 8;
    }
}

// java public static class PiecePercent extends PlaceHolderPiece
pub struct PiecePercent {
    pub base: PlaceHolderPiece,
}
impl PiecePercent {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 6),
        }
    }
}
impl Piece for PiecePercent {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|4);
        p.mPieceType |= 4;
    }
}

// java public static class PieceText extends PlaceHolderPiece
pub struct PieceText {
    pub base: PlaceHolderPiece,
}
impl PieceText {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 12),
        }
    }
}
impl Piece for PieceText {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|64);
        p.mPieceType |= 64;
        p.mHasTextPlaceHolder = true;
    }
}

// java public static class Piece_y extends PlaceHolderPiece
pub struct Piece_y {
    pub base: PlaceHolderPiece,
}
impl Piece_y {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 23),
        }
    }
}
impl Piece for Piece_y {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|8192);
        p.mPieceType |= 8192;
    }
}

// java public static class Piece_m extends PlaceHolderPiece
pub struct Piece_m {
    pub base: PlaceHolderPiece,
}
impl Piece_m {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 27),
        }
    }
}
impl Piece for Piece_m {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn matchToHolderID(&self) -> i16 {
        if self.base.mMatchCount > 2 {
            24
        } else {
            -1
        }
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|8192);
        p.mPieceType |= 8192;
    }
}

// java public static class Piece_d extends PlaceHolderPiece
pub struct Piece_d {
    pub base: PlaceHolderPiece,
}
impl Piece_d {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 25),
        }
    }
}
impl Piece for Piece_d {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn matchToHolderID(&self) -> i16 {
        if self.base.mMatchCount > 2 {
            32
        } else {
            -1
        }
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|8192);
        p.mPieceType |= 8192;
    }
}

// java public static class Piece_h extends PlaceHolderPiece
pub struct Piece_h {
    pub base: PlaceHolderPiece,
}
impl Piece_h {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 26),
        }
    }
}
impl Piece for Piece_h {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|8192);
        p.mPieceType |= 8192;
    }
}

// java public static class Piece_s extends PlaceHolderPiece
pub struct Piece_s {
    pub base: PlaceHolderPiece,
}
impl Piece_s {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 28),
        }
    }
}
impl Piece for Piece_s {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|8192);
        p.mPieceType |= 8192;
    }
}

// java public static class PieceThaiYear extends PlaceHolderPiece
pub struct PieceThaiYear {
    pub base: PlaceHolderPiece,
}
impl PieceThaiYear {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 38),
        }
    }
}
impl Piece for PieceThaiYear {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|8192);
        p.mPieceType |= 8192;
    }
}

// java public static class PieceThaiMonth extends PlaceHolderPiece
pub struct PieceThaiMonth {
    pub base: PlaceHolderPiece,
}
impl PieceThaiMonth {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 37),
        }
    }
}
impl Piece for PieceThaiMonth {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|8192);
        p.mPieceType |= 8192;
    }
}

// java public static class PieceThaiDay extends PlaceHolderPiece
pub struct PieceThaiDay {
    pub base: PlaceHolderPiece,
}
impl PieceThaiDay {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 36),
        }
    }
}
impl Piece for PieceThaiDay {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|8192);
        p.mPieceType |= 8192;
    }
}

// java public static class PieceThaiHour extends PlaceHolderPiece
pub struct PieceThaiHour {
    pub base: PlaceHolderPiece,
}
impl PieceThaiHour {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 39),
        }
    }
}
impl Piece for PieceThaiHour {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|8192);
        p.mPieceType |= 8192;
    }
}

// java public static class PieceThaiMinute extends PlaceHolderPiece
pub struct PieceThaiMinute {
    pub base: PlaceHolderPiece,
}
impl PieceThaiMinute {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 40),
        }
    }
}
impl Piece for PieceThaiMinute {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|8192);
        p.mPieceType |= 8192;
    }
}

// java public static class PieceThaiSecond extends PlaceHolderPiece
pub struct PieceThaiSecond {
    pub base: PlaceHolderPiece,
}
impl PieceThaiSecond {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 41),
        }
    }
}
impl Piece for PieceThaiSecond {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|8192);
        p.mPieceType |= 8192;
    }
}

// java public static class PieceColon extends PlaceHolderPiece
pub struct PieceColon {
    pub base: PlaceHolderPiece,
}
impl PieceColon {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 35),
        }
    }
}
impl Piece for PieceColon {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType=(short)(pieceContext.mPieceType|128);
        p.mPieceType |= 128;
    }
}

// java public static class PieceZero extends PlaceHolderPiece
pub struct PieceZero {
    pub base: PlaceHolderPiece,
}
impl PieceZero {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 3),
        }
    }
}
impl Piece for PieceZero {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java date piece keeps date type; otherwise numeric.
        p.mPieceType |= if p.mPieceType & 8192 > 0 { 8192 } else { 512 };
    }
}

// java public static class PieceDecPoint extends PlaceHolderPiece
pub struct PieceDecPoint {
    pub base: PlaceHolderPiece,
}
impl PieceDecPoint {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 9),
        }
    }
}
impl Piece for PieceDecPoint {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java date decimal sets text type; otherwise numeric.
        p.mPieceType |= if p.mPieceType & 8192 > 0 { 64 } else { 512 };
    }
}

// java public static class PieceEscape extends PlaceHolderPiece
pub struct PieceEscape {
    pub base: PlaceHolderPiece,
}
impl PieceEscape {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 0),
        }
    }
}
impl Piece for PieceEscape {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        // java if(start+1<end){mMatchCount=2;mMatchBuffer=substring(start+1,start+2);}else mMatchCount=1;
        if start + 1 < end {
            self.base.mMatchCount = 2;
            self.base.mMatchBuffer = code[start + 1..start + 2].to_vec();
        } else {
            self.base.mMatchCount = 1;
        }
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= 128;
    }
    // java if(this.mMatchCount<2)throw new AnalyzeException();
    fn check(&mut self) {
        assert!(self.base.mMatchCount >= 2, "AnalyzeException");
    }
}

// java public static class PieceAsterisk extends PlaceHolderPiece
pub struct PieceAsterisk {
    pub base: PlaceHolderPiece,
}
impl PieceAsterisk {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 10),
        }
    }
}
impl Piece for PieceAsterisk {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        // java if(start+1<end){mMatchCount=2;mMatchBuffer=substring(start+1,start+2);}else mMatchCount=1;
        if start + 1 < end {
            self.base.mMatchCount = 2;
            self.base.mMatchBuffer = code[start + 1..start + 2].to_vec();
        } else {
            self.base.mMatchCount = 1;
        }
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= 128;
        p.mHasFill = true;
    }
    // java if(this.mMatchCount<2)throw new AnalyzeException();
    fn check(&mut self) {
        assert!(self.base.mMatchCount >= 2, "AnalyzeException");
    }
}

// java public static class PieceUnderline extends PlaceHolderPiece
pub struct PieceUnderline {
    pub base: PlaceHolderPiece,
}
impl PieceUnderline {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 11),
        }
    }
}
impl Piece for PieceUnderline {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        // java if(start+1<end){mMatchCount=2;mMatchBuffer=substring(start+1,start+2);}else mMatchCount=1;
        if start + 1 < end {
            self.base.mMatchCount = 2;
            self.base.mMatchBuffer = code[start + 1..start + 2].to_vec();
        } else {
            self.base.mMatchCount = 1;
        }
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= 128;
    }
    // java if(this.mMatchCount<2)throw new AnalyzeException();
    fn check(&mut self) {
        assert!(self.base.mMatchCount >= 2, "AnalyzeException");
    }
}

// java public static class PieceDoubleQuatation extends PlaceHolderPiece
pub struct PieceDoubleQuatation {
    pub base: PlaceHolderPiece,
    matched: bool,
}
impl PieceDoubleQuatation {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 13),
            matched: false,
        }
    }
}
impl Piece for PieceDoubleQuatation {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        // java scan to the matching quote; unmatched literal still advances two beyond content for identify().
        self.matched = false;
        for i in start + 1..code.len() {
            if code[i] == self.base.mIdentifer {
                self.matched = true;
                break;
            }
            self.base.mMatchCount += 1;
        }
        self.base.mMatchBuffer =
            code[start + 1..start + 1 + self.base.mMatchCount as usize].to_vec();
        self.base.mMatchCount += 2;
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= 128;
    }
    fn check(&mut self) {
        assert!(self.matched, "AnalyzeException");
    }
}

// java public static class PieceSingleQuatation extends PlaceHolderPiece
pub struct PieceSingleQuatation {
    pub base: PlaceHolderPiece,
    matched: bool,
}
impl PieceSingleQuatation {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 14),
            matched: false,
        }
    }
}
impl Piece for PieceSingleQuatation {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        // java scan to the matching quote; unmatched literal still advances two beyond content for identify().
        self.matched = false;
        for i in start + 1..code.len() {
            if code[i] == self.base.mIdentifer {
                self.matched = true;
                break;
            }
            self.base.mMatchCount += 1;
        }
        self.base.mMatchBuffer =
            code[start + 1..start + 1 + self.base.mMatchCount as usize].to_vec();
        self.base.mMatchCount += 2;
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= 128;
    }
    fn check(&mut self) {
        assert!(self.matched, "AnalyzeException");
    }
}

// java public static class PiecePlainDigit extends PlaceHolderPiece
pub struct PiecePlainDigit {
    pub base: PlaceHolderPiece,
}
impl PiecePlainDigit {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 15),
        }
    }
}
impl Piece for PiecePlainDigit {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        self.base.mMatchCount += 1;
        self.base.mMatchBuffer = code[start..start + 1].to_vec();
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= 128;
    }
}

// java public static class PiecePlainText extends PlaceHolderPiece
pub struct PiecePlainText {
    pub base: PlaceHolderPiece,
}
impl PiecePlainText {
    pub fn new(identifer: char) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, 0),
        }
    }
}
impl Piece for PiecePlainText {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        self.base.mMatchCount += 1;
        self.base.mMatchBuffer = code[start..start + 1].to_vec();
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= 128;
    }
    fn check(&mut self) {
        assert!(
            !self
                .base
                .mMatchBuffer
                .first()
                .is_some_and(|c| *c == 78 || *c == 110),
            "AnalyzeException"
        );
    }
}
// java public abstract static class PieceVariousChar extends PlaceHolderPiece
pub struct PieceVariousChar {
    pub base: PlaceHolderPiece,
    pub mVariousIdentifer: Vec<String>,
    pub mVarIdenMap: super::ConstToken::TokenChildMap,
    pub mMathWordOnly: bool,
    pub mCurVarIdentifer: Option<String>,
}
impl PieceVariousChar {
    pub fn new(
        identifer: char,
        holderID: i16,
        children: Vec<String>,
        childMap: super::ConstToken::TokenChildMap,
        wordOnly: bool,
    ) -> Self {
        Self {
            base: PlaceHolderPiece::new(identifer, holderID),
            mVariousIdentifer: children,
            mVarIdenMap: childMap,
            mMathWordOnly: wordOnly,
            mCurVarIdentifer: None,
        }
    }
    // java public boolean find(...): clear; mIgnoreCase=ignoreCase;match;return mMatchCount>0;
    pub fn find(&mut self, code: &[u16], start: usize, end: usize, ignore: bool) -> bool {
        self.base.clear();
        self.base.mIgnoreCase = ignore;
        self.match_various(code, start, end);
        self.base.mMatchCount > 0
    }
    pub fn match_various(&mut self, code: &[u16], start: usize, end: usize) {
        self.mCurVarIdentifer = None;
        // java for(String item:this.mVariousIdentifer) if(CompareUtil.isStringEqual(...)){mMatchCount=item.length();...;break;}
        for item in &self.mVariousIdentifer {
            let units: Vec<_> = item.encode_utf16().collect();
            if equal(code, start, &units, self.base.mIgnoreCase) {
                self.base.mMatchCount = units.len() as i32;
                self.base.mMatchBuffer = code[start..start + units.len()].to_vec();
                self.mCurVarIdentifer = Some(item.clone());
                break;
            }
        }
        // java if(this.mMatchCount==0&&!this.mMathWordOnly) super.match(...);
        if self.base.mMatchCount == 0 && !self.mMathWordOnly {
            self.base.match_base(code, start, end);
        }
    }
    pub fn holder_id(&self) -> i16 {
        self.mVarIdenMap
            .tokenChildToHolder(self.mCurVarIdentifer.as_deref())
    }
}
fn equal(code: &[u16], start: usize, item: &[u16], ignore: bool) -> bool {
    // java CompareUtil.isStringEqual compares Java UTF16 code units, ignoring case when requested.
    if start + item.len() > code.len() {
        return false;
    }
    code[start..start + item.len()]
        .iter()
        .zip(item)
        .all(|(a, b)| {
            a == b
                || (ignore && crate::native_runtime::Character::toUpperCase(*a)==crate::native_runtime::Character::toUpperCase(*b))
        })
}

// java public static class Piece_a extends PieceVariousChar
pub struct Piece_a {
    pub base: PieceVariousChar,
}
impl Piece_a {
    pub fn new(c: char, children: Vec<String>, map: super::ConstToken::TokenChildMap) -> Self {
        Self {
            base: PieceVariousChar::new(c, 0, children, map, false),
        }
    }
}
impl Piece for Piece_a {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        // java this.mIgnoreCase=true;super.match(...);if(no child&&count>1&&count+start<end) trim last repeated 'a' if AM/PM begins there.
        self.base.base.mIgnoreCase = true;
        self.base.match_various(code, start, end);
        let count = self.base.base.mMatchCount as usize;
        if self.base.mCurVarIdentifer.is_none() && count > 1 && count + start < end {
            for item in &self.base.mVariousIdentifer {
                if equal(
                    code,
                    start + count - 1,
                    &item.encode_utf16().collect::<Vec<_>>(),
                    true,
                ) {
                    self.base.base.mMatchBuffer.truncate(count - 1);
                    self.base.base.mMatchCount -= 1;
                    break;
                }
            }
        }
    }
    fn matchToHolderID(&self) -> i16 {
        let id = self.base.holder_id();
        if id == -1 && self.base.base.mMatchCount > 2 {
            33
        } else {
            id
        }
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= if matches!(self.base.base.mCurPlaceHolder, 33 | 34) {
            8192
        } else {
            128
        };
    }
}

// java public static class Piece_AMPM extends PieceVariousChar
pub struct Piece_AMPM {
    pub base: PieceVariousChar,
}
impl Piece_AMPM {
    pub fn new(c: char, children: Vec<String>, map: super::ConstToken::TokenChildMap) -> Self {
        Self {
            base: PieceVariousChar::new(c, 0, children, map, false),
        }
    }
    // java public Piece_AMPM(char identifer, String[] variousIdentifer, ConstToken.TokenChildMap varIdenMap, boolean mathWordOnly) { super(identifer, (short)0, variousIdentifer, varIdenMap, mathWordOnly); }
    pub fn new__4(c:char,children:Vec<String>,map:super::ConstToken::TokenChildMap,mathWordOnly:bool)->Self{Self{base:PieceVariousChar::new(c,0,children,map,mathWordOnly)}}
}
impl Piece for Piece_AMPM {
    fn find_various(&mut self,code:&[u16],start:usize,end:usize,ignore:bool)->bool{self.base.find(code,start,end,ignore)}
    fn base(&self) -> &PlaceHolderPiece {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        self.base.match_various(code, start, end);
    }
    fn matchToHolderID(&self) -> i16 {
        self.base.holder_id()
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= if self.base.base.mCurPlaceHolder == 34 {
            8192
        } else {
            128
        };
    }
}

// java public static class Piece_g extends PieceVariousChar
pub struct Piece_g {
    pub base: PieceVariousChar,
}
impl Piece_g {
    pub fn new(c: char, children: Vec<String>, map: super::ConstToken::TokenChildMap) -> Self {
        Self {
            base: PieceVariousChar::new(c, 20, children, map, false),
        }
    }
}
impl Piece for Piece_g {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        self.base.match_various(code, start, end);
    }
    fn matchToHolderID(&self) -> i16 {
        self.base.holder_id()
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= match self.base.base.mCurPlaceHolder {
            1 => 1,
            20 => 8192,
            _ => 0,
        };
    }
}

// java public static class Piece_e extends PieceVariousChar
pub struct Piece_e {
    pub base: PieceVariousChar,
}
impl Piece_e {
    pub fn new(c: char, children: Vec<String>, map: super::ConstToken::TokenChildMap) -> Self {
        Self {
            base: PieceVariousChar::new(c, 21, children, map, false),
        }
    }
}
impl Piece for Piece_e {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        // java this.mIgnoreCase=false;super.match(...);
        self.base.base.mIgnoreCase = false;
        self.base.match_various(code, start, end);
    }
    fn matchToHolderID(&self) -> i16 {
        self.base.holder_id()
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= if self.base.base.mCurPlaceHolder == 7 {
            16
        } else {
            8192
        };
    }
    fn check(&mut self) {
        assert!(self.base.base.mMatchCount != 0, "AnalyzeException");
    }
}

// java public static class Piece_b extends PieceVariousChar
pub struct Piece_b {
    pub base: PieceVariousChar,
}
impl Piece_b {
    pub fn new(c: char, children: Vec<String>, map: super::ConstToken::TokenChildMap) -> Self {
        Self {
            base: PieceVariousChar::new(c, 22, children, map, false),
        }
    }
}
impl Piece for Piece_b {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        self.base.match_various(code, start, end);
    }
    fn matchToHolderID(&self) -> i16 {
        self.base.holder_id()
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= 8192;
    }
}

// java public static class PieceGeneral extends PieceVariousChar
pub struct PieceGeneral {
    pub base: PieceVariousChar,
}
impl PieceGeneral {
    pub fn new(c: char, children: Vec<String>, map: super::ConstToken::TokenChildMap) -> Self {
        Self {
            base: PieceVariousChar::new(c, -1, children, map, true),
        }
    }
}
impl Piece for PieceGeneral {
    fn find_various(&mut self,code:&[u16],start:usize,end:usize,ignore:bool)->bool{self.base.find(code,start,end,ignore)}
    fn base(&self) -> &PlaceHolderPiece {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, end: usize) {
        self.base.match_various(code, start, end);
    }
    fn matchToHolderID(&self) -> i16 {
        self.base.holder_id()
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        p.mPieceType |= 1;
    }
}
// java public static class PieceBracket extends PieceVariousChar
pub struct PieceBracket {
    pub base: PieceVariousChar,
    mPattern: Vec<regex::Regex>,
    mMatchedPtnIdx: i32,
    mCaptures: Option<Vec<Option<String>>>,
    mEnclosed: bool,
    mIsFormat: bool,
}
impl PieceBracket {
    pub fn new(
        c: char,
        children: Vec<String>,
        map: super::ConstToken::TokenChildMap,
        isFormat: bool,
    ) -> Self {
        // java this.mPattern[i]=Pattern.compile(this.mVariousIdentifer[i],2);
        let patterns = children
            .iter()
            .map(|p| {
                regex::RegexBuilder::new(p)
                    .case_insensitive(true)
                    .build()
                    .expect("bracket regex")
            })
            .collect();
        Self {
            base: PieceVariousChar::new(c, -1, children, map, false),
            mPattern: patterns,
            mMatchedPtnIdx: -1,
            mCaptures: None,
            mEnclosed: false,
            mIsFormat: isFormat,
        }
    }
    // java quickMate: $, comparisons, D, H/h, m, s, otherwise color.
    fn quickMate(c: u16) -> i32 {
        match c {
            36 => 0,
            60 | 61 | 62 => 3,
            68 => 2,
            72 | 104 => 4,
            109 => 5,
            115 => 6,
            _ => 1,
        }
    }
}
impl Piece for PieceBracket {
    fn base(&self) -> &PlaceHolderPiece {
        &self.base.base
    }
    fn base_mut(&mut self) -> &mut PlaceHolderPiece {
        &mut self.base.base
    }
    fn match_piece(&mut self, code: &[u16], start: usize, _end: usize) {
        // java clear override resets match index and matcher; match scans to ] without validation during identify.
        self.base.mCurVarIdentifer = None;
        self.mMatchedPtnIdx = -1;
        self.mCaptures = None;
        self.mEnclosed = false;
        for i in start + 1..code.len() {
            if code[i] == 93 {
                self.mEnclosed = true;
                break;
            }
            self.base.base.mMatchCount += 1;
        }
        self.base.base.mMatchBuffer =
            code[start + 1..start + 1 + self.base.base.mMatchCount as usize].to_vec();
        self.base.base.mMatchCount += 2;
        // java if(this.mIsFormat){this.mMatchedPtnIdx=this.quickMate(this.mMatchBuffer.charAt(0));this.mCurVarIdentifer=this.mVariousIdentifer[this.mMatchedPtnIdx];this.mMatcher=null;}
        if self.mIsFormat {
            self.mMatchedPtnIdx = Self::quickMate(self.base.base.mMatchBuffer[0]);
            self.base.mCurVarIdentifer =
                Some(self.base.mVariousIdentifer[self.mMatchedPtnIdx as usize].clone());
        } else {
            // java for(Pattern item:this.mPattern){++this.mMatchedPtnIdx;this.mMatcher=item.matcher(...);if(this.mMatcher.lookingAt()){this.mCurVarIdentifer=...;break;}}
            let text = String::from_utf16(&self.base.base.mMatchBuffer).expect("bracket UTF16");
            for pattern in &self.mPattern {
                self.mMatchedPtnIdx += 1;
                if let Some(captures) = pattern
                    .captures(&text)
                    .filter(|c| c.get(0).unwrap().start() == 0)
                {
                    self.mCaptures = Some(
                        captures
                            .iter()
                            .map(|m| m.map(|m| m.as_str().into()))
                            .collect(),
                    );
                    self.base.mCurVarIdentifer =
                        Some(self.base.mVariousIdentifer[self.mMatchedPtnIdx as usize].clone());
                    break;
                }
            }
        }
    }
    fn matchToHolderID(&self) -> i16 {
        self.base.holder_id()
    }
    fn setPieceContext(&self, p: &mut PieceContext) {
        // java pieceContext.mPieceType|=128;switch(mMatchedPtnIdx) ...
        p.mPieceType |= 128;
        let b = &self.base.base.mMatchBuffer;
        match self.mMatchedPtnIdx {
            0 => {
                if self.mIsFormat {
                    if let Some(index) = b.iter().position(|c| *c == 45) {
                        p.mNumAreaCode = Some(String::from_utf16(&b[index + 1..]).unwrap());
                    }
                } else {
                    p.mNumAreaCode = self
                        .mCaptures
                        .as_ref()
                        .and_then(|c| c.get(1))
                        .cloned()
                        .flatten();
                }
            }
            2 => {
                if self.mIsFormat {
                    let index = b.iter().position(|c| *c == 109).map_or(0, |i| i + 1);
                    p.mDBNumCode = Some(String::from_utf16(&b[index..]).unwrap());
                } else {
                    p.mDBNumCode = self
                        .mCaptures
                        .as_ref()
                        .and_then(|c| c.get(1))
                        .cloned()
                        .flatten();
                }
            }
            4 | 5 | 6 => p.mPieceType |= 8192,
            _ => {}
        }
    }
    fn check(&mut self) {
        // java if(!mEnclosed)throw AnalyzeException; if(color capture invalid) throw; if(mCurVarIdentifer==null) throw.
        assert!(self.mEnclosed, "AnalyzeException: bracket not encolsed");
        if self.mMatchedPtnIdx == 1 {
            if let Some(Some(index)) = self.mCaptures.as_ref().and_then(|c| c.get(1)) {
                assert!(crate::cn::wps::kfc::numfmt::builtin::BuiltinColors::BuiltinColors::isValidColorIdx(index),"AnalyzeException: outer color to inner color fail!");
            }
        }
        assert!(
            self.base.mCurVarIdentifer.is_some(),
            "AnalyzeException: not recognised format!"
        );
    }
}

// rust: Existing Word facade, outside the Excel read closure.
pub struct WPieceX {}
pub struct WPieceApostrophe {}
pub struct WPieceSign {}
pub struct WPiece_a {
    // Java field: String mKeyIdentifer
    pub(crate) mKeyIdentifer: String,
    // Java field: String mKeyGroupIdentifer
    pub(crate) mKeyGroupIdentifer: String,
}
pub struct WPiece_d {}
pub struct WPiece_e {}
pub struct WPiece_g {}
pub struct WPiece_o {}
pub struct WPiece_y {}
pub struct WPiece_m {}
pub struct WPiece_n {}
pub struct WPiece_r {}
pub struct WPiece_w {}
pub struct WPiece_b {}
pub struct WPiecePlainText {
    // Java field: char mIdentifer
    pub(crate) mIdentifer: char,
}
impl WPieceX {
    // Java: WPieceX(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)42);
        WPieceX {}
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 512);
        todo!("WPieceX::setPieceContext")
    }
}
impl WPieceApostrophe {
    // Java: WPieceApostrophe(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)44);
        WPieceApostrophe {}
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 128);
        todo!("WPieceApostrophe::setPieceContext")
    }
}
impl WPieceSign {
    // Java: WPieceSign(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)43);
        WPieceSign {}
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mAddMinus = false;
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 128);
        todo!("WPieceSign::setPieceContext")
    }
}
impl WPiece_a {
    // Java: WPiece_a(char identifer, String[] children, ConstToken.TokenChildMap childMap)
    pub fn new(_identifer: char, _children: Vec<String>, _child_map: i32) -> Self {
        // Java: super(identifer, children, childMap);
        // Java: this.mKeyIdentifer = Character.toString(identifer);
        // Java: char c = Character.toLowerCase(identifer);
        // Java: this.mKeyGroupIdentifer = new String(new char[]{c, c, c});
        let c = _identifer.to_lowercase().next().unwrap_or(_identifer);
        WPiece_a {
            mKeyIdentifer: _identifer.to_string(),
            mKeyGroupIdentifer: format!("{}{}{}", c, c, c),
        }
    }

    // Java: short matchToHolderID()
    pub fn matchToHolderID(&self) -> i16 {
        // Java: short id = this.mVarIdenMap.tokenChildToHolder(this.mCurVarIdentifer);
        // Java: return id != -1 || this.mMatchBuffer.indexOf(this.mKeyIdentifer) <= -1 && this.mMatchBuffer.indexOf(this.mKeyGroupIdentifer) <= -1 ? id : 46;
        todo!("WPiece_a::matchToHolderID")
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: switch (this.mCurPlaceHolder)
        {
            // Java: case 46:
            // Java: case 54:
            // Java: case 55:
            // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 8192);
            // Java: break;
            // Java: default:
            // Java: super.setPieceContext(pieceContext);
        }
        todo!("WPiece_a::setPieceContext")
    }
}
impl WPiece_d {
    // Java: WPiece_d(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)47);
        WPiece_d {}
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 8192);
        todo!("WPiece_d::setPieceContext")
    }
}
impl WPiece_e {
    // Java: WPiece_e(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)52);
        WPiece_e {}
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 8192);
        todo!("WPiece_e::setPieceContext")
    }

    // Java: void match(String formatCode, int start, int end)
    pub fn match_piece(&mut self, _format_code: &str, _start: i32, _end: i32) {
        // Java: char prevMatchedChar = formatCode.charAt(start);
        // Java: for(int i = start; i < end && formatCode.charAt(i) == prevMatchedChar; ++i)
        {
            // Java: ++this.mMatchCount;
        }
        // Java: this.mMatchBuffer = formatCode.substring(start, start + this.mMatchCount);
        todo!("WPiece_e::match_piece")
    }
}
impl WPiece_g {
    // Java: WPiece_g(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)48);
        WPiece_g {}
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 8192);
        todo!("WPiece_g::setPieceContext")
    }
}
impl WPiece_o {
    // Java: WPiece_o(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)45);
        WPiece_o {}
    }

    // Java: void match(String formatCode, int start, int end)
    pub fn match_piece(&mut self, _format_code: &str, _start: i32, _end: i32) {
        // Java: this.mIgnoreCase = false;
        // Java: super.match(formatCode, start, end);
        todo!("WPiece_o::match_piece")
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 8192);
        todo!("WPiece_o::setPieceContext")
    }
}
impl WPiece_y {
    // Java: WPiece_y(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)51);
        WPiece_y {}
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 8192);
        todo!("WPiece_y::setPieceContext")
    }
}
impl WPiece_m {
    // Java: WPiece_m(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)50);
        WPiece_m {}
    }

    // Java: void match(String formatCode, int start, int end)
    pub fn match_piece(&mut self, _format_code: &str, _start: i32, _end: i32) {
        // Java: this.mIgnoreCase = false;
        // Java: super.match(formatCode, start, end);
        todo!("WPiece_m::match_piece")
    }

    // Java: short matchToHolderID()
    pub fn matchToHolderID(&self) -> i16 {
        // Java: return this.mMatchBuffer.charAt(0) == 'm' ? 27 : super.matchToHolderID();
        todo!("WPiece_m::matchToHolderID")
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 8192);
        todo!("WPiece_m::setPieceContext")
    }
}
impl WPiece_n {
    // Java: WPiece_n(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)52);
        WPiece_n {}
    }

    // Java: void match(String formatCode, int start, int end)
    pub fn match_piece(&mut self, _format_code: &str, _start: i32, _end: i32) {
        // Java: this.mIgnoreCase = false;
        // Java: super.match(formatCode, start, end);
        todo!("WPiece_n::match_piece")
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 8192);
        todo!("WPiece_n::setPieceContext")
    }
}
impl WPiece_r {
    // Java: WPiece_r(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)49);
        WPiece_r {}
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 8192);
        todo!("WPiece_r::setPieceContext")
    }
}
impl WPiece_w {
    // Java: WPiece_w(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)53);
        WPiece_w {}
    }

    // Java: void match(String formatCode, int start, int end)
    pub fn match_piece(&mut self, _format_code: &str, _start: i32, _end: i32) {
        // Java: this.mIgnoreCase = false;
        // Java: super.match(formatCode, start, end);
        todo!("WPiece_w::match_piece")
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 8192);
        todo!("WPiece_w::setPieceContext")
    }
}
impl WPiece_b {
    // Java: WPiece_b(char identifer)
    pub fn new(_identifer: char) -> Self {
        // Java: super(identifer, (short)56);
        WPiece_b {}
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 8192);
        todo!("WPiece_b::setPieceContext")
    }
}
impl WPiecePlainText {
    // Java: WPiecePlainText()
    pub fn new() -> Self {
        // Java: super('p', (short)0);
        WPiecePlainText { mIdentifer: 'p' }
    }

    // Java: void setIdentifer(char identifer)
    pub fn setIdentifer(&mut self, _identifer: char) {
        // Java: this.mIdentifer = identifer;
        self.mIdentifer = _identifer;
    }

    // Java: void match(String formatCode, int start, int end)
    pub fn match_piece(&mut self, _format_code: &str, _start: i32, _end: i32) {
        // Java: for(int i = start; i < end; ++i)
        {
            // Java: char c = formatCode.charAt(i);
            // Java: if (c != this.mIdentifer)
            {
                // Java: break;
            }
            // Java: ++this.mMatchCount;
        }
        // Java: this.mMatchBuffer = formatCode.substring(start, start + this.mMatchCount);
        todo!("WPiecePlainText::match_piece")
    }

    // Java: void setPieceContext(PieceContext pieceContext)
    pub fn setPieceContext(&self, _piece_context: &mut i32) {
        // Java: pieceContext.mPieceType = (short)(pieceContext.mPieceType | 128);
        todo!("WPiecePlainText::setPieceContext")
    }
}
