#![allow(non_snake_case)]
use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};
use drawing::cn::wps::moffice::drawing::{
    Shape::Shape, graphics::GradStop::GradStop, line::Arrow::Arrow,
    smartshape::Segment::Segment,
};
use util::cn::wps::graphics::RectF::RectF;
use crate::cn::wps::moffice::writer::core::Document::Document;
use super::{BgClr::BgClr, ClrFrom::ClrFrom, ColorTransform::ColorTransformValue,
    Cxn::Cxn, Cxnlst::Cxnlst, Duotone::Duotone, EffectRef::EffectRef,
    FgClr::FgClr, FillRef::FillRef, Gds::GdsRef, Glow::Glow, Gs::Gs,
    InnerShdw::InnerShdw, LnRef::LnRef, OuterShdw::OuterShdw,
    PrstShdw::PrstShdw, Shadow::Shadow, SolidFill::SolidFill, Tile::Tile};

// Java: public class DmlShapeCtxt
pub struct DmlShapeCtxt {
    // Java: Shape mShape. The shared handle is the same object stored by Shape's fills.
    pub mShape: Option<Arc<Mutex<Shape>>>,
    pub mDoc: Option<Rc<RefCell<Box<dyn Document>>>>,
    pub mOffX: Option<i64>, pub mOffY: Option<i64>,
    pub mExtX: Option<i64>, pub mExtY: Option<i64>,
    pub mChOffX: Option<i64>, pub mChOffY: Option<i64>,
    pub mChExtX: Option<i64>, pub mChExtY: Option<i64>,
    pub mAng: Option<String>, pub mScaled: Option<bool>, pub mPath: Option<String>,
    pub mGsLst: Option<Vec<GradStop>>, pub mGs: Option<Gs>,
    pub mFgClr: Option<FgClr>, pub mBgClr: Option<BgClr>,
    pub mSolidFill: Option<SolidFill>,
    pub mPrstShdw: Option<PrstShdw>, pub mOuterShdw: Option<OuterShdw>,
    pub mShadow: Option<Shadow>, pub mInnerShdw: Option<InnerShdw>,
    pub mClrTransform: Option<ColorTransformValue>,
    pub mLnRef: Option<LnRef>, pub mFillRef: Option<FillRef>, pub mEffectRef: Option<EffectRef>,
    pub mFillOn: bool,
    pub mPathFillType: Option<String>, pub mPathStrokeOn: Option<bool>,
    pub mGlow: Option<Glow>, pub mGdlst: Option<GdsRef>,
    pub mCxnlst: Option<Cxnlst>, pub mCxn: Option<Cxn>,
    pub mPathW: Option<i32>, pub mPathH: Option<i32>,
    pub mFlipH: Option<bool>, pub mFlipV: Option<bool>,
    pub mSizeRelH: Option<i32>, pub mSizeRelV: Option<i32>,
    pub mIsGroup: Option<bool>, pub mIsChild: Option<bool>,
    pub mDashing: Option<i32>, pub mJoinStyle: Option<i32>,
    pub mStartArrow: Option<Arrow>, pub mEndArrow: Option<Arrow>,
    pub mTailType: Option<i32>, pub mTailW: Option<i32>, pub mTailLen: Option<i32>,
    pub mLineWidth: Option<f32>, pub mLineStyle: Option<i32>, pub mCap: Option<i32>,
    pub mPhClr: Option<i32>, pub mFillRect: Option<RectF>, pub mTileRect: Option<RectF>,
    pub mVertices: Option<Vec<i32>>, pub mSegments: Option<Vec<Segment>>,
    pub mFillType: Option<i32>, pub mShapeType: Option<i32>,
    pub mTile: Option<Tile>, pub mDuotone: Option<Duotone>,
    pub mLineFillOn: bool, pub mNormalEastAsianFlow: Option<bool>,
    pub mClrFrom: Option<ClrFrom>, pub mBlipFillRotWithShape: bool,
}
impl Default for DmlShapeCtxt {
    fn default() -> Self {
        Self {
            mShape: None, mDoc: None, mOffX: None, mOffY: None,
            mExtX: None, mExtY: None, mChOffX: None, mChOffY: None,
            mChExtX: None, mChExtY: None, mAng: None, mScaled: None, mPath: None,
            mGsLst: None, mGs: None, mFgClr: None, mBgClr: None, mSolidFill: None,
            mPrstShdw: None, mOuterShdw: None, mShadow: None, mInnerShdw: None,
            mClrTransform: None, mLnRef: None, mFillRef: None, mEffectRef: None,
            mFillOn: true, mPathFillType: None, mPathStrokeOn: None,
            mGlow: None, mGdlst: None, mCxnlst: None, mCxn: None,
            mPathW: None, mPathH: None, mFlipH: None, mFlipV: None,
            mSizeRelH: None, mSizeRelV: None, mIsGroup: None, mIsChild: None,
            mDashing: None, mJoinStyle: None, mStartArrow: None, mEndArrow: None,
            mTailType: None, mTailW: None, mTailLen: None,
            mLineWidth: None, mLineStyle: None, mCap: None, mPhClr: None,
            mFillRect: None, mTileRect: None, mVertices: None, mSegments: None,
            mFillType: None, mShapeType: None, mTile: None, mDuotone: None,
            mLineFillOn: true, mNormalEastAsianFlow: None, mClrFrom: None,
            mBlipFillRotWithShape: true,
        }
    }
}
impl DmlShapeCtxt {
    // Java: implicit DmlShapeCtxt()
    pub fn new() -> Self { Self::default() }
}
