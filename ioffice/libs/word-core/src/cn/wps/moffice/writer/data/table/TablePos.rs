// Java: import cn.wps.graphics.RectF
use util::graphics::RectF::*;
// Java: import cn.wps.moffice.drawing.IPos
use drawing::cn::wps::moffice::drawing::IPos::*;
// Java: import cn.wps.moffice.util.BoolBitSet
use crate::cn::wps::moffice::util::BoolBitSet::*;
// Java: import cn.wps.moffice.util.IntBitSet
use crate::cn::wps::moffice::util::IntBitSet::*;
// Java: import cn.wps.moffice.util.UnitUtil
use util::moffice::util::UnitUtil::UnitUtil;

// java static final IntBitSet FLAG_ALIGNMENTH = new IntBitSet(0, 2);
pub(crate) static FLAG_ALIGNMENTH: std::sync::LazyLock<std::sync::Arc<IntBitSet>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(IntBitSet::new(0, 2)));
// java static final IntBitSet FLAG_ALIGNMENTV = new IntBitSet(3, 5);
pub(crate) static FLAG_ALIGNMENTV: std::sync::LazyLock<std::sync::Arc<IntBitSet>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(IntBitSet::new(3, 5)));
// java static final IntBitSet FLAG_RELATIVEH = new IntBitSet(6, 8);
pub(crate) static FLAG_RELATIVEH: std::sync::LazyLock<std::sync::Arc<IntBitSet>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(IntBitSet::new(6, 8)));
// java static final IntBitSet FLAG_RELATIVEV = new IntBitSet(9, 11);
pub(crate) static FLAG_RELATIVEV: std::sync::LazyLock<std::sync::Arc<IntBitSet>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(IntBitSet::new(9, 11)));
// java static final IntBitSet FLAG_TEXTWRAP = new IntBitSet(12, 14);
pub(crate) static FLAG_TEXTWRAP: std::sync::LazyLock<std::sync::Arc<IntBitSet>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(IntBitSet::new(12, 14)));
// java static final BoolBitSet FLAG_OVERLAP = new BoolBitSet(15);
pub(crate) static FLAG_OVERLAP: std::sync::LazyLock<std::sync::Arc<BoolBitSet>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(BoolBitSet::new(15)));
// java static final BoolBitSet FLAG_LAYOUTINCELL = new BoolBitSet(16);
pub(crate) static FLAG_LAYOUTINCELL: std::sync::LazyLock<std::sync::Arc<BoolBitSet>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(BoolBitSet::new(16)));

// java public class TablePos implements IPos
#[derive(Clone)]
pub struct TablePos {
    // java RectF rect = new RectF();
    pub(crate) rect: Option<RectF>,
    // java float leftFromText = UnitUtil.TWIPS2PT(0);
    pub(crate) leftFromText: f32,
    // java float topFromText = UnitUtil.TWIPS2PT(0);
    pub(crate) topFromText: f32,
    // java float rightFromText = UnitUtil.TWIPS2PT(0);
    pub(crate) rightFromText: f32,
    // java float bottomFromText = UnitUtil.TWIPS2PT(0);
    pub(crate) bottomFromText: f32,
    // java int flag = 0;
    pub(crate) flag: i32,
}

impl Default for TablePos {
    fn default() -> Self {
        Self {
            rect: Some(RectF::new()),
            leftFromText: UnitUtil::TWIPS2PT(0),
            topFromText: UnitUtil::TWIPS2PT(0),
            rightFromText: UnitUtil::TWIPS2PT(0),
            bottomFromText: UnitUtil::TWIPS2PT(0),
            flag: 0,
        }
    }
}

// java public interface IPos
impl IPos for TablePos {
    // java public boolean contains(int nProp)
    fn contains(&mut self, nProp: i32) -> bool {
      // java return true;
      return true;
    }

    // java public void setShapeRect(RectF rectF)
    fn setShapeRect(&mut self, rectF: RectF) {
      // java this.rect.set(rectF);
      if let Some(r) = self.rect.as_mut() {
        r.set_from(&rectF);
      } else {
        self.rect = Some(rectF);
      }
    }

    // java public RectF getShapeRect()
    fn getShapeRect(&self) -> RectF {
      // java return this.rect;
      return self.rect.clone().unwrap_or_else(RectF::new);
    }

    // java public void setRelativeH(int rel)
    fn setRelativeH(&mut self, rel: i32) {
      // java this.flag = FLAG_RELATIVEH.set(this.flag, rel);
      self.flag = FLAG_RELATIVEH.set(self.flag, rel);
    }

    // java public int getRelativeH()
    fn getRelativeH(&self) -> i32 {
      // java return FLAG_RELATIVEH.get(this.flag);
      return FLAG_RELATIVEH.get(self.flag);
    }

    // java public int getRelativeX()
    fn getRelativeX(&self) -> i32 {
      // java return this.getRelativeH();
      return self.getRelativeH();
    }

    // java public void setPosRelH(int rel)
    fn setPosRelH(&mut self, rel: i32) {
    }

    // java public int getPosRelH()
    fn getPosRelH(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java public void setRelativeV(int rel)
    fn setRelativeV(&mut self, rel: i32) {
      // java this.flag = FLAG_RELATIVEV.set(this.flag, rel);
      self.flag = FLAG_RELATIVEV.set(self.flag, rel);
    }

    // java public int getRelativeV()
    fn getRelativeV(&self) -> i32 {
      // java return FLAG_RELATIVEV.get(this.flag);
      return FLAG_RELATIVEV.get(self.flag);
    }

    // java public int getRelativeY()
    fn getRelativeY(&self) -> i32 {
      // java return this.getRelativeV();
      return self.getRelativeV();
    }

    // java public void setPosRelV(int rel)
    fn setPosRelV(&mut self, rel: i32) {
    }

    // java public int getPosRelV()
    fn getPosRelV(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java public void setPosHPercent(float val)
    fn setPosHPercent(&mut self, val: f32) {
    }

    // java public float getPosHPercent()
    fn getPosHPercent(&self) -> f32 {
      // java return 0.0F;
      return 0.0;
    }

    // java public Float getPosHPercentObj()
    fn getPosHPercentObj(&self) -> Option<f32> {
      // java return null;
      return None;
    }

    // java public void setPosVPercent(float val)
    fn setPosVPercent(&mut self, val: f32) {
    }

    // java public float getPosVPercent()
    fn getPosVPercent(&self) -> f32 {
      // java return 0.0F;
      return 0.0;
    }

    // java public Float getPosVPercentObj()
    fn getPosVPercentObj(&self) -> Option<f32> {
      // java return null;
      return None;
    }

    // java public final void setAlignmentH(int h)
    fn setAlignmentH(&mut self, h: i32) {
      // java this.flag = FLAG_ALIGNMENTH.set(this.flag, h);
      self.flag = FLAG_ALIGNMENTH.set(self.flag, h);
    }

    // java public final int getAlignmentH()
    fn getAlignmentH(&self) -> i32 {
      // java return FLAG_ALIGNMENTH.get(this.flag);
      return FLAG_ALIGNMENTH.get(self.flag);
    }

    // java public final void setAlignmentV(int v)
    fn setAlignmentV(&mut self, v: i32) {
      // java this.flag = FLAG_ALIGNMENTV.set(this.flag, v);
      self.flag = FLAG_ALIGNMENTV.set(self.flag, v);
    }

    // java public final int getAlignmentV()
    fn getAlignmentV(&self) -> i32 {
      // java return FLAG_ALIGNMENTV.get(this.flag);
      return FLAG_ALIGNMENTV.get(self.flag);
    }

    // java public void setSizeRelH(int v)
    fn setSizeRelH(&mut self, v: i32) {
    }

    // java public int getSizeRelH()
    fn getSizeRelH(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java public Integer getSizeRelHObj()
    fn getSizeRelHObj(&self) -> Option<i32> {
      // java return null;
      return None;
    }

    // java public void setSizeRelV(int v)
    fn setSizeRelV(&mut self, v: i32) {
    }

    // java public int getSizeRelV()
    fn getSizeRelV(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java public Integer getSizeRelVObj()
    fn getSizeRelVObj(&self) -> Option<i32> {
      // java return null;
      return None;
    }

    // java public void setSizeHPercent(float val)
    fn setSizeHPercent(&mut self, val: f32) {
    }

    // java public float getSizeHPercent()
    fn getSizeHPercent(&self) -> f32 {
      // java return 0.0F;
      return 0.0;
    }

    // java public Float getSizeHPercentObj()
    fn getSizeHPercentObj(&self) -> Option<f32> {
      // java return null;
      return None;
    }

    // java public void setSizeVPercent(float val)
    fn setSizeVPercent(&mut self, val: f32) {
    }

    // java public float getSizeVPercent()
    fn getSizeVPercent(&self) -> f32 {
      // java return 0.0F;
      return 0.0;
    }

    // java public Float getSizeVPercentObj()
    fn getSizeVPercentObj(&self) -> Option<f32> {
      // java return null;
      return None;
    }

    // java public void setScaleH(float scale)
    fn setScaleH(&mut self, scale: f32) {
    }

    // java public float getScaleH()
    fn getScaleH(&self) -> f32 {
      // java return 1.0F;
      return 1.0;
    }

    // java public void setScaleV(float scale)
    fn setScaleV(&mut self, scale: f32) {
    }

    // java public float getScaleV()
    fn getScaleV(&self) -> f32 {
      // java return 1.0F;
      return 1.0;
    }

    // java public void setWrapDistLeft(float dist)
    fn setWrapDistLeft(&mut self, dist: f32) {
      // java this.leftFromText = dist;
      self.leftFromText = dist;
    }

    // java public float getWrapDistLeft()
    fn getWrapDistLeft(&self) -> f32 {
      // java return this.leftFromText;
      return self.leftFromText;
    }

    // java public Float getWrapDistLeftObj()
    fn getWrapDistLeftObj(&self) -> Option<f32> {
      // java return this.leftFromText;
      return Some(self.leftFromText);
    }

    // java public void setWrapDistTop(float dist)
    fn setWrapDistTop(&mut self, dist: f32) {
      // java this.topFromText = dist;
      self.topFromText = dist;
    }

    // java public float getWrapDistTop()
    fn getWrapDistTop(&self) -> f32 {
      // java return this.topFromText;
      return self.topFromText;
    }

    // java public Float getWrapDistTopObj()
    fn getWrapDistTopObj(&self) -> Option<f32> {
      // java return this.topFromText;
      return Some(self.topFromText);
    }

    // java public void setWrapDistRight(float dist)
    fn setWrapDistRight(&mut self, dist: f32) {
      // java this.rightFromText = dist;
      self.rightFromText = dist;
    }

    // java public float getWrapDistRight()
    fn getWrapDistRight(&self) -> f32 {
      // java return this.rightFromText;
      return self.rightFromText;
    }

    // java public Float getWrapDistRightObj()
    fn getWrapDistRightObj(&self) -> Option<f32> {
      // java return this.rightFromText;
      return Some(self.rightFromText);
    }

    // java public void setWrapDistBottom(float dist)
    fn setWrapDistBottom(&mut self, dist: f32) {
      // java this.bottomFromText = dist;
      self.bottomFromText = dist;
    }

    // java public float getWrapDistBottom()
    fn getWrapDistBottom(&self) -> f32 {
      // java return this.bottomFromText;
      return self.bottomFromText;
    }

    // java public Float getWrapDistBottomObj()
    fn getWrapDistBottomObj(&self) -> Option<f32> {
      // java return this.bottomFromText;
      return Some(self.bottomFromText);
    }

    // java public final void setTextWrap(int wrap)
    fn setTextWrap(&mut self, wrap: i32) {
      // java this.flag = FLAG_TEXTWRAP.set(this.flag, wrap);
      self.flag = FLAG_TEXTWRAP.set(self.flag, wrap);
    }

    // java public final int getTextWrap()
    fn getTextWrap(&self) -> i32 {
      // java return FLAG_TEXTWRAP.get(this.flag);
      return FLAG_TEXTWRAP.get(self.flag);
    }

    // java public void setTextWrapType(int wrap)
    fn setTextWrapType(&mut self, wrap: i32) {
    }

    // java public int getTextWrapType()
    fn getTextWrapType(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java public void setRotation(float degree)
    fn setRotation(&mut self, degree: f32) {
    }

    // java public float getRotation()
    fn getRotation(&self) -> f32 {
      // java return 0.0F;
      return 0.0;
    }

    // java public final void setAllowOverlap(boolean bAllow)
    fn setAllowOverlap(&mut self, bAllow: bool) {
      // java this.flag = FLAG_OVERLAP.set(this.flag, bAllow);
      self.flag = FLAG_OVERLAP.set(self.flag, bAllow);
    }

    // java public final boolean getAllowOverlap()
    fn getAllowOverlap(&self) -> bool {
      // java return FLAG_OVERLAP.get(this.flag);
      return FLAG_OVERLAP.get(self.flag);
    }

    // java public final boolean getLayoutInCell()
    fn getLayoutInCell(&self) -> bool {
      // java return FLAG_LAYOUTINCELL.get(this.flag);
      return FLAG_LAYOUTINCELL.get(self.flag);
    }

    // java public final void setLayoutInCell(boolean bInCell)
    fn setLayoutInCell(&mut self, bInCell: bool) {
      // java this.flag = FLAG_LAYOUTINCELL.set(this.flag, bInCell);
      self.flag = FLAG_LAYOUTINCELL.set(self.flag, bInCell);
    }

    // java public boolean isEmpty()
    fn isEmpty(&mut self) -> bool {
      // java return false;
      return false;
    }

    // java public void dispose()
    fn dispose(&mut self) {
      // java this.rect = null;
      self.rect = None;
    }

}

// java public class TablePos implements IPos
impl TablePos {
    // java public int getFlag()
    pub fn getFlag(&self) -> i32 {
      // java return this.flag;
      return self.flag;
    }

    // java public void setFlag(int value)
    pub fn setFlag(&mut self, value: i32) {
      // java this.flag = value;
      self.flag = value;
    }

}
