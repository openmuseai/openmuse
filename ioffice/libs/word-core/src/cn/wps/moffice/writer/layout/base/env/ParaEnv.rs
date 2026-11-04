// Java: import cn.wps.collections.KList
use util::collections::KList::KList;
// Java: import cn.wps.moffice.writer.data.DefaultValue
use crate::cn::wps::moffice::writer::data::DefaultValue::DefaultValue;
// Java: import cn.wps.moffice.writer.data.value.LineSpacing
use crate::cn::wps::moffice::writer::data::value::LineSpacing::LineSpacing;
// Java: import cn.wps.moffice.writer.layout.base.env.TBD
use crate::cn::wps::moffice::writer::layout::base::env::TBD::TBD;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaPropEnv
use crate::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;

// java public class ParaEnv
pub struct ParaEnv {
    // java public boolean wrapText = true;
    pub wrapText: bool,
    // java public int alignment = 3;
    pub alignment: i32,
    // java public int wordAlignFont = 4;
    pub wordAlignFont: i32,
    // java public LineSpacing lineSpacing;
    pub lineSpacing: LineSpacing,
    // java public boolean hasInitTBDs;
    pub hasInitTBDs: bool,
    // java public KList<TBD> tabStops;
    pub tabStops: Option<Vec<TBD>>,
    // java public boolean wordWrap;
    pub wordWrap: bool,
    // java public boolean kinsoku;
    pub kinsoku: bool,
    // java public boolean overflowPunct;
    pub overflowPunct: bool,
    // java public boolean topLinePunct;
    pub topLinePunct: bool,
    // java public boolean compressLine;
    pub compressLine: bool,
    // java public ParaPropEnv prevParaPropEnv;
    pub prevParaPropEnv: Option<ParaPropEnv>,
    // java public ParaPropEnv curParaPropEnv;
    pub curParaPropEnv: Option<ParaPropEnv>,
    // java public ParaPropEnv nextParaPropEnv;
    pub nextParaPropEnv: Option<ParaPropEnv>,
    // java public boolean snapToGrid;
    pub snapToGrid: bool,
    // java public boolean autoSpaceDE;
    pub autoSpaceDE: bool,
    // java public boolean autoSpaceDN;
    pub autoSpaceDN: bool,
    // java public boolean isRTL;
    pub isRTL: bool,
    // java public boolean isFirstLineOfPara;
    pub isFirstLineOfPara: bool,
    // java public boolean firstLineIsSecBreak;
    pub firstLineIsSecBreak: bool,
}

// java public class ParaEnv
impl ParaEnv {
    // java public  ParaEnv()
    pub fn ParaEnv() -> Self {
      // java this.lineSpacing = DefaultValue.paragraph_lineSpacing;
      Self { wrapText: true, alignment: 3, wordAlignFont: 4, lineSpacing: LineSpacing::LineSpacing(0, 1.0), hasInitTBDs: false, tabStops: Some(Vec::new()), wordWrap: true, kinsoku: true, overflowPunct: true, topLinePunct: false, compressLine: false, prevParaPropEnv: None, curParaPropEnv: None, nextParaPropEnv: None, snapToGrid: true, autoSpaceDE: false, autoSpaceDN: false, isRTL: false, isFirstLineOfPara: false, firstLineIsSecBreak: false }
      // java this.hasInitTBDs = false;
      // java this.tabStops = new KList();
      // java this.wordWrap = true;
      // java this.kinsoku = true;
      // java this.overflowPunct = true;
      // java this.topLinePunct = false;
      // java this.compressLine = false;
      // java this.prevParaPropEnv = null;
      // java this.curParaPropEnv = null;
      // java this.nextParaPropEnv = null;
      // java this.snapToGrid = true;
      // java this.autoSpaceDE = false;
      // java this.autoSpaceDN = false;
      // java this.isRTL = false;
      // java this.isFirstLineOfPara = false;
      // java this.firstLineIsSecBreak = false;
    }

    // java public void saveBrcAndShdAjustInfo(Pools pools)
    pub fn saveBrcAndShdAjustInfo(&mut self, pools: &mut Pools) {
      // java if (null != this.prevParaPropEnv)
      if let Some(prevParaPropEnv) = self.prevParaPropEnv.take() {
        // java pools.recycleParaPropEnv(this.prevParaPropEnv);
        pools.recycleParaPropEnv(prevParaPropEnv);
      }
      // java this.prevParaPropEnv = this.curParaPropEnv;
      self.prevParaPropEnv = self.curParaPropEnv.take();
      // java this.curParaPropEnv = null;
    }

    // java public void clearBrcAndShdAdjustInfo(Pools pools)
    pub fn clearBrcAndShdAdjustInfo(&mut self, pools: &mut Pools) {
      // java if (null != this.prevParaPropEnv)
      if let Some(prevParaPropEnv) = self.prevParaPropEnv.take() {
        // java pools.recycleParaPropEnv(this.prevParaPropEnv);
        pools.recycleParaPropEnv(prevParaPropEnv);
        // java this.prevParaPropEnv = null;
      }
      // java if (null != this.curParaPropEnv)
      if let Some(curParaPropEnv) = self.curParaPropEnv.take() {
        // java pools.recycleParaPropEnv(this.curParaPropEnv);
        pools.recycleParaPropEnv(curParaPropEnv);
        // java this.curParaPropEnv = null;
      }
      // java if (null != this.nextParaPropEnv)
      if let Some(nextParaPropEnv) = self.nextParaPropEnv.take() {
        // java pools.recycleParaPropEnv(this.nextParaPropEnv);
        pools.recycleParaPropEnv(nextParaPropEnv);
        // java this.nextParaPropEnv = null;
      }
    }

    // java public void init(Pools pools)
    pub fn init(&mut self, pools: &mut Pools) {
      // java this.wrapText = true;
      self.wrapText = true;
      // java this.alignment = 3;
      self.alignment = 3;
      // java this.wordAlignFont = 4;
      self.wordAlignFont = 4;
      // java this.lineSpacing = DefaultValue.paragraph_lineSpacing;
      self.lineSpacing = LineSpacing::LineSpacing(0, 1.0);
      // java this.kinsoku = true;
      self.kinsoku = true;
      // java this.wordWrap = true;
      self.wordWrap = true;
      // java this.overflowPunct = true;
      self.overflowPunct = true;
      // java this.topLinePunct = false;
      self.topLinePunct = false;
      // java this.snapToGrid = true;
      self.snapToGrid = true;
      // java this.autoSpaceDE = false;
      self.autoSpaceDE = false;
      // java this.autoSpaceDN = false;
      self.autoSpaceDN = false;
      // java this.isRTL = false;
      self.isRTL = false;
      // java this.isFirstLineOfPara = false;
      self.isFirstLineOfPara = false;
      // java this.firstLineIsSecBreak = false;
      self.firstLineIsSecBreak = false;
      // java this.clearBrcAndShdAdjustInfo(pools);
      self.clearBrcAndShdAdjustInfo(pools);
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java if (null != this.tabStops)
      if self.tabStops.is_some() {
        // java this.tabStops.dispose();
        // rust: no-stmt
        // java this.tabStops = null;
        self.tabStops = None;
      }
      // java this.prevParaPropEnv = null;
      self.prevParaPropEnv = None;
      // java this.curParaPropEnv = null;
      self.curParaPropEnv = None;
      // java this.nextParaPropEnv = null;
      self.nextParaPropEnv = None;
    }

}

impl Default for ParaEnv {
    fn default() -> Self {
        Self::ParaEnv()
    }
}
