#![allow(non_snake_case)]

use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;

// Java: ShapeCollision.ClipRect.RectFWrapImp extends TypoRect.
// Rust keeps the nested Java type beside ClipRect because Rust has no nested classes.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RectFWrapImp {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub leftWrap: i32,
    pub rightWrap: i32,
}

impl RectFWrapImp {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self { left, top, right, bottom, leftWrap: 0, rightWrap: 0 }
    }

    // Java: RectFWrapImp.getWrapType()
    pub fn getWrapType(&self) -> i32 {
        if self.leftWrap != 0 { self.leftWrap } else { self.rightWrap }
    }

    pub fn width(&self) -> i32 { self.right.wrapping_sub(self.left) }
}

// Java: ShapeCollision.ClipRect. Its list order is the order consumed by
// LineLayouter.prepareSubLineEnvs; it must not be sorted after subtraction.
#[derive(Clone, Debug, Default)]
pub struct ClipRect {
    rts: Vec<RectFWrapImp>,
}

impl ClipRect {
    pub const NONEWRAP: i32 = 0;
    pub const SQUARE: i32 = 1;
    pub const TIGHT: i32 = 2;
    pub const TABLESQUARE: i32 = 3;
    pub const TEXTFRAMESQUARE: i32 = 4;

    // Java: ClipRect.clear() recycles every existing strip.
    pub fn clear(&mut self) { self.rts.clear(); }

    pub fn isEmpty(&self) -> bool { self.rts.is_empty() }

    // Java: ClipRect.init(left, top, right, bottom)
    pub fn init(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        self.clear();
        self.rts.push(RectFWrapImp::new(left, top, right, bottom));
    }

    // Java: ClipRect.getRects(KList<RectFWrapImp>) copies each strip.
    pub fn getRects(&self, out: &mut Vec<RectFWrapImp>) -> bool {
        if self.rts.is_empty() { return false; }
        out.extend(self.rts.iter().cloned());
        true
    }

    pub fn rects(&self) -> &[RectFWrapImp] { &self.rts }

    // Java: ClipRect.subtract(ITypoRect reduce, int wrapType). The compiled
    // cache rectangle is TypoRectCoreImpl; the current ITypoRect Rust trait has
    // no getters yet, so this facade accepts that concrete Java implementor.
    pub fn subtract(&mut self, reduce: &TypoRectCoreImpl, wrapType: i32) {
        self.subtract_span(reduce.getLeft(), reduce.getRight(), wrapType);
    }

    // Y is deliberately not tested here: TextWrapLayouter already selects
    // drawings whose vertical interval intersects the current line.
    fn subtract_span(&mut self, left: i32, right: i32, wrapType: i32) {
        let mut i = 0;
        while i < self.rts.len() {
            let rt = &self.rts[i];
            // Java: isIntersectX(rt, reduce)
            if !(rt.left < right && rt.right > left) {
                i += 1;
                continue;
            }
            let rt = self.rts.remove(i);
            let pieces = Self::subtract_one(rt, left, right, wrapType);
            for piece in pieces {
                // Java: TEXTFRAMESQUARE 1440, TABLESQUARE 380, SQUARE 360,
                // TIGHT 180, NONEWRAP 0. Either edge can impose the minimum.
                let min_width = if piece.leftWrap == Self::TEXTFRAMESQUARE || piece.rightWrap == Self::TEXTFRAMESQUARE {
                    1440
                } else if piece.leftWrap == Self::TABLESQUARE || piece.rightWrap == Self::TABLESQUARE {
                    380
                } else if piece.leftWrap == Self::SQUARE || piece.rightWrap == Self::SQUARE {
                    360
                } else if piece.leftWrap == Self::TIGHT || piece.rightWrap == Self::TIGHT {
                    180
                } else {
                    0
                };
                if piece.width() >= min_width {
                    self.rts.insert(i, piece);
                    i += 1;
                }
            }
        }
    }

    // Java: ClipRect.subtract(RectFWrapImp rt, ITypoRect sub, int wrapType).
    fn subtract_one(mut rt: RectFWrapImp, left: i32, right: i32, wrapType: i32) -> Vec<RectFWrapImp> {
        if left <= rt.left {
            if right < rt.right {
                rt.left = right;
                rt.leftWrap = wrapType;
                vec![rt]
            } else {
                Vec::new()
            }
        } else if right < rt.right {
            let mut left_piece = RectFWrapImp::new(rt.left, rt.top, left, rt.bottom);
            left_piece.rightWrap = wrapType;
            rt.left = right;
            rt.leftWrap = wrapType;
            vec![left_piece, rt]
        } else {
            rt.right = left;
            rt.rightWrap = wrapType;
            vec![rt]
        }
    }
}
