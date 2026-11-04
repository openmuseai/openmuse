// Java: import cn.wps.collections.KPList
use util::collections::KPList::{KPList, KPNode};
// Java: import cn.wps.moffice.writer.cache.TypoRun
use crate::cn::wps::moffice::writer::cache::TypoRun::TypoRun;
// Java: import cn.wps.moffice.writer.cache.style.TypoFontStyle
// rust: ★wr-29 04 —— 真实 cache::style::TypoFontStyle（Java 同包名）
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle;
// Java: import cn.wps.moffice.writer.layout.base.languages.ILangLayout
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::languages::ILangLayout::ILangLayout;
// Java: import cn.wps.moffice.writer.layout.base.line.charborder.BdrInfo
use crate::cn::wps::moffice::writer::layout::base::line::charborder::BdrInfo::BdrInfo;
// Java: import cn.wps.moffice.writer.layout.base.line.docfield.DfdFlagsInGraphs
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::line::docfield::DfdFlagsInGraphs::DfdFlagsInGraphs;
// Java: import cn.wps.io.dom.Node
// CARGO_OFF_SURFACE use io_wps_io::cn::wps::io::dom::Node::Node;
// Java: import cn.wps.moffice.writer.layout.base.graph.CharsNode
use crate::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode;
// Java: import cn.wps.moffice.writer.layout.base.graph.SrcNode
use crate::cn::wps::moffice::writer::layout::base::graph::SrcNode::SrcNode;
use core::marker::PhantomData as _;

// rust: ★wr-29 04 —— ILangLayout 去影子：真实 trait 在 languages::ILangLayout；
//       Graphs.mLangLayout 以语言 id（Option<i32>）承载（Java 对象引用 → 语言标识）
#[derive(Default, Clone)]
pub struct DfdFlagsIterator {
    flag: i32,
}
impl DfdFlagsIterator {
    pub fn getDfdFlag(&self) -> i32 { self.flag }
    pub fn recycle(&mut self) {}
}

// java public final class Graphs extends KPList.Node
pub struct Graphs {
    // java KPList.Node super_
    pub super_: KPNode,
    // java public int runType;
    pub runType: i32,
    // java public int language;
    pub language: i32,
    // java public CharsNode charsRef;
    pub charsRef: Option<CharsNode>,
    // java public int graphIndex;
    pub graphIndex: i32,
    // java public int graphLength;
    pub graphLength: i32,
    // java public TypoFontStyle fontStyle;
    pub fontStyle: Option<TypoFontStyle>,
    // java public int baseLine;
    pub baseLine: i32,
    // java public int mXPos;
    pub mXPos: i32,
    // java public int mWidth;
    pub mWidth: i32,
    // java public int autoSpaceType;
    pub autoSpaceType: i32,
    // java public ILangLayout mLangLayout;
    // rust: ★wr-29 04 —— Option<i32>（语言 id；Java 对象引用语义见 languages/README）
    pub mLangLayout: Option<i32>,
    // java public DfdFlagsInGraphs.Iterator mDfdFlagsIt;
    pub mDfdFlagsIt: Option<DfdFlagsIterator>,
    // java public BdrInfo mBdrInfo;
    pub mBdrInfo: BdrInfo,
    // java public int mHyphenFlag;
    pub mHyphenFlag: i32,
}

// java public static class Alloc implements KPList.INodeAlloc<Graphs>
#[derive(Default)]
pub struct Alloc {
}

// java public final class Graphs extends KPList.Node
impl Graphs {
    // java public boolean hasBdr()
    pub fn hasBdr(&self) -> bool {
      // java return this.mBdrInfo.hasBdr();
      return self.mBdrInfo.hasBdr();
    }

    // java public void clearBdr()
    pub fn clearBdr(&mut self) {
      // java this.mBdrInfo.clearWidth();
      self.mBdrInfo.clearWidth();
    }

    // java public int getBdrWidth()
    pub fn getBdrWidth(&self) -> i32 {
      // java return this.mBdrInfo.getWidth();
      return self.mBdrInfo.getWidth();
    }

    // java public boolean hasBeginBdr()
    pub fn hasBeginBdr(&self) -> bool {
      // java return this.mBdrInfo.hasBegin();
      return self.mBdrInfo.hasBegin();
    }

    // java public boolean hasEndBdr()
    pub fn hasEndBdr(&self) -> bool {
      // java return this.mBdrInfo.hasEnd();
      return self.mBdrInfo.hasEnd();
    }

    // java public boolean hasBeginOrEndBdr()
    pub fn hasBeginOrEndBdr(&self) -> bool {
      // java return this.hasBeginBdr() || this.hasEndBdr();
      return self.hasBeginBdr() || self.hasEndBdr();
    }

    // java public void setBeginBdr(int width)
    pub fn setBeginBdr(&mut self, width: i32) {
      // java if (width > 0)
      if width > 0 {
        // java if (!this.hasBeginBdr())
        if !self.hasBeginBdr() {
          // java this.mBdrInfo.setBegin(width);
          self.mBdrInfo.setBegin(width);
          // java this.extendsWidth(width);
          self.extendsWidth(width);
        }
      }
      // java else if (this.hasBeginBdr())
      else if self.hasBeginBdr() {
        // java this.extendsWidth(-this.getBdrWidth());
        self.extendsWidth(-self.getBdrWidth());
      }
    }

    // java public void setBdrWidth(int width)
    pub fn setBdrWidth(&mut self, width: i32) {
      // java this.mBdrInfo.setWidth(width);
      self.mBdrInfo.setWidth(width);
    }

    // java public void setEndBdr(int width)
    pub fn setEndBdr(&mut self, width: i32) {
      // java if (width > 0)
      if width > 0 {
        // java if (!this.hasEndBdr())
        if !self.hasEndBdr() {
          // java this.mBdrInfo.setEnd(width);
          self.mBdrInfo.setEnd(width);
          // java this.extendsWidth(width);
          self.extendsWidth(width);
        }
      }
      // java else if (this.hasEndBdr())
      else if self.hasEndBdr() {
        // java this.extendsWidth(-this.getBdrWidth());
        self.extendsWidth(-self.getBdrWidth());
      }
    }

    // java public int getDfdFlag()
    pub fn getDfdFlag(&self) -> i32 {
      // java return this.mDfdFlagsIt != null ? this.mDfdFlagsIt.getDfdFlag() : 0;
      return (if let Some(it) = self.mDfdFlagsIt.as_ref() { it.getDfdFlag() } else { 0 });
    }

    // java private  Graphs()
    pub fn Graphs() -> Self {
      // java this.runType = 0;
      Self { super_: KPNode::new(), runType: 0, language: 0, charsRef: Default::default(), graphIndex: -1, graphLength: Default::default(), fontStyle: Default::default(), baseLine: Default::default(), mXPos: Default::default(), mWidth: Default::default(), autoSpaceType: 0, mLangLayout: Default::default(), mDfdFlagsIt: Default::default(), mBdrInfo: BdrInfo::default(), mHyphenFlag: 0 }
      // java this.language = 0;
      // java this.graphIndex = -1;
      // java this.autoSpaceType = 0;
      // java this.mBdrInfo = new BdrInfo();
      // java this.mHyphenFlag = 0;
    }

    // java public void clone(Graphs src)
    pub fn clone(&mut self, src: &Graphs) {
      // java this.runType = src.runType;
      self.runType = src.runType;
      // java this.language = src.language;
      self.language = src.language;
      // java this.charsRef = src.charsRef;
      self.charsRef = src.charsRef.clone();
      // java this.graphIndex = src.graphIndex;
      self.graphIndex = src.graphIndex;
      // java this.graphLength = src.graphLength;
      self.graphLength = src.graphLength;
      // java this.fontStyle = src.fontStyle;
      self.fontStyle = src.fontStyle.clone();
      // java this.baseLine = src.baseLine;
      self.baseLine = src.baseLine;
      // java this.mXPos = src.mXPos;
      self.mXPos = src.mXPos;
      // java this.mWidth = src.mWidth;
      self.mWidth = src.mWidth;
      // java this.autoSpaceType = src.autoSpaceType;
      self.autoSpaceType = src.autoSpaceType;
      // java this.mLangLayout = src.mLangLayout;
      self.mLangLayout = src.mLangLayout.clone();
      // java src.mBdrInfo.copyTo(this.mBdrInfo);
      src.mBdrInfo.copyTo(Some(&mut self.mBdrInfo));
      // java this.mHyphenFlag = src.mHyphenFlag;
      self.mHyphenFlag = src.mHyphenFlag;
    }

    // java public void init()
    pub fn init(&mut self) {
      // java this.language = 0;
      self.language = 0;
      // java this.charsRef = null;
      self.charsRef = None;
      // java this.graphIndex = -1;
      self.graphIndex = -1;
      // java this.graphLength = 0;
      self.graphLength = 0;
      // java this.fontStyle = null;
      self.fontStyle = None;
      // java this.runType = -1;
      self.runType = -1;
      // java this.baseLine = 0;
      self.baseLine = 0;
      // java this.mXPos = 0;
      self.mXPos = 0;
      // java this.mWidth = 0;
      self.mWidth = 0;
      // java this.autoSpaceType = 0;
      self.autoSpaceType = 0;
      // java if (this.mDfdFlagsIt != null)
      if let Some(mDfdFlagsIt) = self.mDfdFlagsIt.as_mut() {
        // java this.mDfdFlagsIt.recycle();
        mDfdFlagsIt.recycle();
        // java this.mDfdFlagsIt = null;
        self.mDfdFlagsIt = None;
      }
      // java this.mBdrInfo.clear();
      self.mBdrInfo.clear();
      // java this.mHyphenFlag = 0;
      self.mHyphenFlag = 0;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.charsRef = null;
      self.charsRef = None;
      // java this.fontStyle = null;
      self.fontStyle = None;
      // java super.dispose();
      self.super_.dispose();
    }

    // java public int getRight()
    pub fn getRight(&self) -> i32 {
      // java return this.mXPos + this.mWidth;
      return self.mXPos + self.mWidth;
    }

    // java public int getRightWithBdr()
    pub fn getRightWithBdr(&self) -> i32 {
      // java return this.getRight() + (this.hasBdr() ? this.getBdrWidth() : 0);
      return self.getRight() + (if self.hasBdr() { self.getBdrWidth() } else { 0 });
    }

    // java public int getLeft()
    pub fn getLeft(&self) -> i32 {
      // java return this.mXPos;
      return self.mXPos;
    }

    // java public void setRight(int mCurTabPos)
    pub fn setRight(&mut self, mCurTabPos: i32) {
      // java this.mWidth = mCurTabPos - this.mXPos;
      self.mWidth = mCurTabPos - self.mXPos;
    }

    // java public void extendsWidth(int f)
    pub fn extendsWidth(&mut self, f: i32) {
      // java this.mWidth += f;
      self.mWidth = self.mWidth + f;
    }

    // java public void changeWidthSum(int charWidths)
    pub fn changeWidthSum(&mut self, charWidths: i32) {
      // java this.mWidth = charWidths;
      self.mWidth = charWidths;
      // java if (this.mBdrInfo.hasBegin())
      if self.mBdrInfo.hasBegin() {
        // java this.mWidth += this.mBdrInfo.getWidth();
        self.mWidth = self.mWidth + self.mBdrInfo.getWidth();
      }
      // java if (this.mBdrInfo.hasEnd())
      if self.mBdrInfo.hasEnd() {
        // java this.mWidth += this.mBdrInfo.getWidth();
        self.mWidth = self.mWidth + self.mBdrInfo.getWidth();
      }
    }

    // java public boolean isFootEndNote()
    pub fn isFootEndNote(&self) -> bool {
      // java return this.runType == 8;
      return self.runType == 8;
    }

    // java public int getEndGraphIndex()
    pub fn getEndGraphIndex(&self) -> i32 {
      // java return this.graphIndex + this.graphLength - 1;
      return self.graphIndex + self.graphLength - 1;
    }

    // java public void offsetXTo(int mCurRunPos)
    pub fn offsetXTo(&mut self, mCurRunPos: i32) {
      // java this.mXPos = mCurRunPos;
      self.mXPos = mCurRunPos;
    }

    // java public boolean isAutoNum()
    pub fn isAutoNum(&self) -> bool {
      // java if (null != this.charsRef)
      if let Some(charsRef) = self.charsRef.as_ref() {
        // java SrcNode srcNode = this.charsRef.srcRef;
        let srcNode: Option<*mut SrcNode> = charsRef.srcRef;
        // java if (null != srcNode && srcNode.type == 1)
        if srcNode.map(|p| unsafe { (*p).r#type == 1 }).unwrap_or(false) {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
    }

    // java public boolean isSpace()
    pub fn isSpace(&self) -> bool {
      // java return this.runType == 2;
      return self.runType == 2;
    }

    // java public int width()
    pub fn width(&self) -> i32 {
      // java return this.mWidth;
      return self.mWidth;
    }

    // java public int getWithExcludeBdr()
    pub fn getWithExcludeBdr(&self) -> i32 {
      // java int w = this.mWidth;
      let mut w: i32 = self.mWidth;
      // java if (this.hasBeginBdr())
      if self.hasBeginBdr() {
        // java w -= this.getBdrWidth();
        w = w - self.getBdrWidth();
      }
      // java if (this.hasEndBdr())
      if self.hasEndBdr() {
        // java w -= this.getBdrWidth();
        w = w - self.getBdrWidth();
      }
      // java return w;
      return w;
    }

    // java public void offsetX(int f)
    pub fn offsetX(&mut self, f: i32) {
      // java this.mXPos += f;
      self.mXPos = self.mXPos + f;
    }

    // java public boolean isEmbed()
    pub fn isEmbed(&self) -> bool {
      // java return this.runType == 6;
      return self.runType == 6;
    }

    // java public boolean isTab()
    pub fn isTab(&self) -> bool {
      // java return this.runType >= 20 && this.runType <= 25;
      return self.runType >= 20 && self.runType <= 25;
    }

    // java public int getRevisionInsAuthor()
    pub fn getRevisionInsAuthor(&self) -> i32 {
      // java return null == this.charsRef ? -1 : this.charsRef.revisionInsAuthor;
      return (if self.charsRef.is_none() { -1 } else { self.charsRef.as_ref().unwrap().revisionInsAuthor });
    }

    // java public int getRevisionDelAuthor()
    pub fn getRevisionDelAuthor(&self) -> i32 {
      // java return null == this.charsRef ? -1 : this.charsRef.revisionDelAuthor;
      return (if self.charsRef.is_none() { -1 } else { self.charsRef.as_ref().unwrap().revisionDelAuthor });
    }

    // java public int getHyperlinkId()
    pub fn getHyperlinkId(&self) -> i32 {
      // java return null == this.charsRef ? -1 : this.charsRef.hyperlinkIndex;
      return (if self.charsRef.is_none() { -1 } else { self.charsRef.as_ref().unwrap().hyperlinkIndex });
    }

    // java public boolean isLineBreak()
    pub fn isLineBreak(&self) -> bool {
      // java return TypoRun.isLineBreak(this.runType);
      return TypoRun::isLineBreak(self.runType);
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuffer stringBuff = new StringBuffer();
      let mut stringBuff: String = String::new();
      // java stringBuff.append("lang=");
      stringBuff.push_str("lang=");
      // java switch (this.language)
      match self.language {
        // java case 0: case 1:
        0 | 1 => {
          // java stringBuff.append("WEST ");
          stringBuff.push_str("WEST ");
        }
        // java case 2: case 3:
        2 | 3 => {
          // java stringBuff.append("FAREAST ");
          stringBuff.push_str("FAREAST ");
        }
        // java case 4:
        4 => {
          // java stringBuff.append("COMPLEX ");
          stringBuff.push_str("COMPLEX ");
        }
        // java case 5:
        5 => {
          // java stringBuff.append("ARABIC ");
          stringBuff.push_str("ARABIC ");
        }
        // java default:
        _ => {
        }
      }
      // java stringBuff.append(" graphIndex=" + this.graphIndex);
      stringBuff.push_str(&format!(" graphIndex={}", self.graphIndex));
      // java stringBuff.append(" graphLength=" + this.graphLength);
      stringBuff.push_str(&format!(" graphLength={}", self.graphLength));
      // java stringBuff.append("[");
      stringBuff.push_str("[");
      // java stringBuff.append(this.charsRef.toString());
      stringBuff.push_str(&format!("{:?}", self.charsRef.as_ref().map(|c| c.hyperlinkIndex)));
      // java stringBuff.append("]");
      stringBuff.push_str("]");
      // java stringBuff.append("," + this.mBdrInfo);
      stringBuff.push_str(",");
      stringBuff.push_str(&self.mBdrInfo.toString());
      // java stringBuff.append("\nRect[");
      stringBuff.push_str("\nRect[");
      // java stringBuff.append(this.mXPos);
      stringBuff.push_str(&self.mXPos.to_string());
      // java stringBuff.append(", ");
      stringBuff.push_str(", ");
      // java stringBuff.append(this.mXPos + this.mWidth);
      stringBuff.push_str(&(self.mXPos + self.mWidth).to_string());
      // java stringBuff.append("]");
      stringBuff.push_str("]");
      // java stringBuff.append(", RunType:");
      stringBuff.push_str(", RunType:");
      // java stringBuff.append(TypoRun.getTypeStr(this.runType));
      stringBuff.push_str(&TypoRun::getTypeStr(self.runType));
      // java return stringBuff.toString();
      return stringBuff;
    }

}

impl std::ops::Deref for Graphs {
    type Target = KPNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for Graphs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl Default for Graphs {
    fn default() -> Self {
        Self::Graphs()
    }
}

impl Clone for Graphs {
    fn clone(&self) -> Self {
        let mut c = Self::Graphs();
        Graphs::clone(&mut c, self);
        c.super_.next = self.super_.next;
        c.super_.prev = self.super_.prev;
        c
    }
}

// java public static class Alloc implements KPList.INodeAlloc<Graphs>
impl Alloc {
    // java public Graphs alloc()
    pub fn alloc(&mut self) -> Graphs {
      // java return new Graphs();
      return Graphs::Graphs();
    }

}
