// Java: cn.wps.moffice.writer.layout.base.languages.WestLayout
// rust: ★wr-29 04 —— 真实移植（Java 158 行）：addLangGraph 合并 ASCII 字母/数字 run
//       （capsHandler 恒 None —— 首字下沉非 hello）；回滚（压缩 05 阶段）停车。
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

// Java: import cn.wps.moffice.writer.cache.TypoPara
use crate::cn::wps::moffice::writer::layout::base::languages::ILangLayout::{GraphOperationEnv, RollBackResult};
use crate::cn::wps::moffice::writer::layout::base::languages::LangLayout::{LangLayout, *};

// java public class WestLayout extends LangLayout
pub struct WestLayout {
    // java LangLayout super_
    pub(crate) super_: LangLayout,
    // java EnglishBreaker mBreaker = new EnglishBreaker();
    // rust: 断词器（压缩/连字符 05）未接线 → 停车
}

impl WestLayout {
    // java public WestLayout(int language)
    pub fn WestLayout(language: i32) -> WestLayout {
        // java super(language);
        WestLayout { super_: LangLayout { language } }
    }

    // java public int addLangGraph(int startGraphIndex, int endGraphIndex, int bufOffset, int bufLimit, int[] charBuff, ILangLayout.GraphOperationEnv graphOperationEnv)
    pub fn addLangGraph(&mut self, startGraphIndex: i32, endGraphIndex: i32, mut bufOffset: i32, bufLimit: i32, charBuff: &[i32], graphOperationEnv: &mut GraphOperationEnv) -> i32 {
      // java int autoSpaceType = graphOperationEnv.autoSpaceType;
      let autoSpaceType: i32 = graphOperationEnv.autoSpaceType;

      // java ICapsHandler capsHandler = graphOperationEnv.capsHandler;
      let capsHandler: Option<()> = graphOperationEnv.capsHandler;
      // java TypoPara typoPara = graphOperationEnv.layoutState.typoPara;
      // java int[] blockResult = graphOperationEnv.blockResult;
      let layoutState = unsafe { &mut *graphOperationEnv.layoutState };
      // java int firstChar = charBuff[bufOffset];
      let firstChar: i32 = charBuff[bufOffset as usize];
      // java int graphLen = 1;
      let mut graphLen: i32 = 1;
      // java if (1 == autoSpaceType)
      if 1 == autoSpaceType {
        // java if (null == capsHandler)
        if capsHandler.is_none() {
          // java int nextGraphIndex = startGraphIndex + 1; int nextBlockOffset = bufOffset + 1; int nextBuffLimit = bufLimit;
          let mut nextGraphIndex: i32 = startGraphIndex + 1;
          let mut nextBlockOffset: i32 = bufOffset + 1;
          let mut nextBuffLimit: i32 = bufLimit;
          // java for(int[] nextCharsBuff = charBuff; nextGraphIndex < endGraphIndex; ++nextBlockOffset)
          let mut nextCharsBuff: Vec<i32> = charBuff.to_vec();
          while nextGraphIndex < endGraphIndex {
            // java if (nextBlockOffset >= nextBuffLimit) { nextCharsBuff = typoPara.getCharBlock(nextGraphIndex, blockResult); ... }
            if nextBlockOffset >= nextBuffLimit {
              let mut blockResult = [0; 2];
              let blk: &[char] = layoutState.typoPara.getCharBlock(nextGraphIndex, &mut blockResult);
              nextCharsBuff = blk.iter().map(|c| *c as u32 as i32).collect();
              nextBlockOffset = blockResult[0];
              nextBuffLimit = blockResult[1];
            }
            // java int curCh = nextCharsBuff[nextBlockOffset];
            let curCh: i32 = nextCharsBuff[nextBlockOffset as usize];
            // java if ((curCh < 97 || curCh > 122) && (curCh < 65 || curCh > 90)) { break; }
            if (curCh < 97 || curCh > 122) && (curCh < 65 || curCh > 90) { break; }
            // java ++graphLen; ++nextGraphIndex; ++nextBlockOffset;
            graphLen += 1;
            nextGraphIndex += 1;
            nextBlockOffset += 1;
          }
        } else {
          // rust: capsHandler 恒 None（首字下沉非 hello，GR-D3）—— Java else 分支停车
          /* DEFERRED_NON_FUNCTION （caps onAddAlphabet 分支） */
        }
      }
      // java else if (4 == autoSpaceType)
      else if 4 == autoSpaceType {
        // java 数字合并 —— 同字母模式
        let mut nextGraphIndex: i32 = startGraphIndex + 1;
        let mut nextBlockOffset: i32 = bufOffset + 1;
        let mut nextBuffLimit: i32 = bufLimit;
        let mut nextCharsBuff: Vec<i32> = charBuff.to_vec();
        while nextGraphIndex < endGraphIndex {
          if nextBlockOffset >= nextBuffLimit {
            let mut blockResult = [0; 2];
            let blk: &[char] = layoutState.typoPara.getCharBlock(nextGraphIndex, &mut blockResult);
            nextCharsBuff = blk.iter().map(|c| *c as u32 as i32).collect();
            nextBlockOffset = blockResult[0];
            nextBuffLimit = blockResult[1];
          }
          let curCh: i32 = nextCharsBuff[nextBlockOffset as usize];
          if curCh < 48 || curCh > 57 { break; }
          graphLen += 1;
          nextGraphIndex += 1;
          nextBlockOffset += 1;
        }
      }
      // java else if (null != capsHandler) { ... }
      // rust: capsHandler None → 跳过
      let _ = firstChar;
      // java bufOffset /* 未用（original 亦未用） */ ;
      let _ = &mut bufOffset;
      // java this.addGraphAfterParse(startGraphIndex, startGraphIndex + graphLen, bufOffset, bufLimit, charBuff, graphOperationEnv);
      self.super_.addGraphAfterParse(startGraphIndex, startGraphIndex + graphLen, bufOffset, bufLimit, charBuff, graphOperationEnv);
      // java graphOperationEnv.isScralForTmpGrpahNode = graphOperationEnv.isScralForSmallCaps;
      graphOperationEnv.isScralForTmpGrpahNode = graphOperationEnv.isScralForSmallCaps;
      // java return graphLen;
      graphLen
    }
}

impl super::ILangLayout::ILangLayout for WestLayout {
    fn addGraphNode(&mut self, var1: i32, var2: i32, var3: &mut GraphOperationEnv) -> *mut crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs {
        // java super.addGraphNode(...)
        self.super_.addGraphNode(var1, var2, var3)
    }
    fn addLangGraph(&mut self, var1: i32, var2: i32, var3: i32, var4: i32, var5: &[i32], var6: &mut GraphOperationEnv) -> i32 {
        // java 覆盖（合并）
        self.addLangGraph(var1, var2, var3, var4, var5, var6)
    }
    fn commitLangGraphNode(&mut self, var1: &mut GraphOperationEnv) {
        self.super_.commitLangGraphNode(var1)
    }
    fn addCommonGraph(&mut self, var1: i32, var2: &mut GraphOperationEnv) -> i32 {
        self.super_.addCommonGraph(var1, var2)
    }
    fn commitCommonGraphNode(&mut self, var1: &mut GraphOperationEnv) {
        self.super_.commitCommonGraphNode(var1)
    }
    fn processMathUnderline(&mut self, var1: crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle, var2: &mut GraphOperationEnv) -> crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle {
        self.super_.processMathUnderline(var1, var2)
    }
    fn rollBackForPartialWord(&mut self, var1: i32, var2: i32, var3: i32, var4: i32, var5: i32, var6: i32, var7: &mut RollBackResult) {
        // rust: DEFERRED（05 压缩）；Java 原文见 WestLayout.java:73-151
        var7.adjusted = false;
        var7.rollbackedAll = false;
    }
    fn hasMergeChar(&self) -> bool {
        self.super_.hasMergeChar()
    }
}