// Java: import cn.wps.moffice.util.BoolBitSet
use crate::cn::wps::moffice::util::BoolBitSet::*;
// Java: import cn.wps.moffice.util.IntBitSet
use crate::cn::wps::moffice::util::IntBitSet::*;
// Java: import cn.wps.moffice.writer.cache.style.TypoSHDs
use crate::cn::wps::moffice::writer::cache::style::TypoSHDs::*;
// Java: import cn.wps.moffice.writer.cache.TypoObjBase
use crate::cn::wps::moffice::writer::cache::TypoObjBase::*;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::*;
// Java: import cn.wps.moffice.writer.cache.TypoPara
use crate::cn::wps::moffice::writer::cache::TypoPara::*;
// Java: import cn.wps.moffice.writer.cache.TypoMemoryObj
use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::*;
// Java: import cn.wps.moffice.writer.cache.ITypoRect
use crate::cn::wps::moffice::writer::cache::ITypoRect::*;
// Java: import java.lang.Class
use stubs::java::lang::Class::*;

// java static final int OFFSET_START = 7;
pub(crate) const OFFSET_START: i32 = 7;
// java public static final int OFFSET_PARA = 7;
pub(crate) const OFFSET_PARA: i32 = 7;
// java static final int OFFSET_TEXT_RECT_TOP = 8;
pub(crate) const OFFSET_TEXT_RECT_TOP: i32 = 8;
// java static final int OFFSET_TEXT_RECT_HEIGHT = 9;
pub(crate) const OFFSET_TEXT_RECT_HEIGHT: i32 = 9;
// java static final int OFFSET_CHANEL_ASC_POS = 10;
pub(crate) const OFFSET_CHANEL_ASC_POS: i32 = 10;
// java static final int OFFSET_CHANEL_DES_POS = 11;
pub(crate) const OFFSET_CHANEL_DES_POS: i32 = 11;
// java static final int OFFSET_FLAG = 12;
pub(crate) const OFFSET_FLAG: i32 = 12;
// java public static final int OFFSET_GRAPH_INDEX_START = 13;
pub(crate) const OFFSET_GRAPH_INDEX_START: i32 = 13;
// java public static final int OFFSET_GRAPH_INDEX_END = 14;
pub(crate) const OFFSET_GRAPH_INDEX_END: i32 = 14;
// java public static final int OFFSET_SHDS = 15;
pub(crate) const OFFSET_SHDS: i32 = 15;
// java public static final int OFFSET_BORDERS = 16;
pub(crate) const OFFSET_BORDERS: i32 = 16;
// java public static final int OFFSET_HIGHLIGHTS = 17;
pub(crate) const OFFSET_HIGHLIGHTS: i32 = 17;
// java public static final int OFFSET_DOCFIELDS = 18;
pub(crate) const OFFSET_DOCFIELDS: i32 = 18;
// java public static final int OFFSET_LINES = 19;
pub(crate) const OFFSET_LINES: i32 = 19;
// java public static final int OFFSET_PARA_SHD_AND_BRCS = 20;
pub(crate) const OFFSET_PARA_SHD_AND_BRCS: i32 = 20;
// java public static final int OFFSET_MEM_END = 21;
pub(crate) const OFFSET_MEM_END: i32 = 21;
// java public static final int OFFSET_Y_POS = 22;
pub(crate) const OFFSET_Y_POS: i32 = 22;
// java static final int OFFSET_LINE_BEFORE = 23;
pub(crate) const OFFSET_LINE_BEFORE: i32 = 23;
// java static final int OFFSET_LINE_AFTER = 24;
pub(crate) const OFFSET_LINE_AFTER: i32 = 24;
// java static final int OFFSET_LINE_BALLOON_PAGE_HEIGHT = 25;
pub(crate) const OFFSET_LINE_BALLOON_PAGE_HEIGHT: i32 = 25;
// java public static final int OFFSET_MAX = 26;
pub(crate) const OFFSET_MAX: i32 = 26;
// java static final BoolBitSet FLAG_FIRSTLINEOFPARA;
pub(crate) const FLAG_FIRSTLINEOFPARA: i32 = 0;
// java static final BoolBitSet FLAG_LASTLINEOFPARA;
pub(crate) const FLAG_LASTLINEOFPARA: i32 = 1;
// java static final BoolBitSet FLAG_LASTLINEOFSECTION;
pub(crate) const FLAG_LASTLINEOFSECTION: i32 = 2;
// java static final BoolBitSet FLAG_REVISION;
pub(crate) const FLAG_REVISION: i32 = 3;
// java static final BoolBitSet FLAG_COMMENT;
pub(crate) const FLAG_COMMENT: i32 = 4;
// java static final BoolBitSet FLAG_RTL;
pub(crate) const FLAG_RTL: i32 = 5;
// java static final BoolBitSet FLAG_MODIFY;
pub(crate) const FLAG_MODIFY: i32 = 6;
// java static final BoolBitSet FLAG_LASTLINEOFPAGE;
pub(crate) const FLAG_LASTLINEOFPAGE: i32 = 7;
// java static final BoolBitSet FLAG_LASTLINEOFCOLUMN;
pub(crate) const FLAG_LASTLINEOFCOLUMN: i32 = 8;
// java static final BoolBitSet FLAG_HASBIDI;
pub(crate) const FLAG_HASBIDI: i32 = 9;
// java static final BoolBitSet FLAG_HAS_SUBLINE;
pub(crate) const FLAG_HAS_SUBLINE: i32 = 10;
// java static final BoolBitSet FLAG_HAS_IMBEDS_DRAWING;
pub(crate) const FLAG_HAS_IMBEDS_DRAWING: i32 = 12;
// java static final BoolBitSet FLAG_INMATHPARA;
pub(crate) const FLAG_INMATHPARA: i32 = 13;
// java static final BoolBitSet FLAG_FIRSTLINEOFMATHPARA;
pub(crate) const FLAG_FIRSTLINEOFMATHPARA: i32 = 14;
// java static final BoolBitSet FLAG_LASTLINEOFMATHPARA;
pub(crate) const FLAG_LASTLINEOFMATHPARA: i32 = 15;
// java static final BoolBitSet FLAG_FIXEDHEIGHT;
pub(crate) const FLAG_FIXEDHEIGHT: i32 = 16;
// java static final BoolBitSet FLAG_EXTEND_LASTSHDANDBRC;
pub(crate) const FLAG_EXTEND_LASTSHDANDBRC: i32 = 17;
// java static final BoolBitSet FLAG_HAS_NOT_CLIP_EMBEDS;
pub(crate) const FLAG_HAS_NOT_CLIP_EMBEDS: i32 = 18;
// java static final BoolBitSet FLAG_ISTEMPLATECONTENT;
pub(crate) const FLAG_ISTEMPLATECONTENT: i32 = 19;
// java static final BoolBitSet FLAG_HAS_AUTO_NUMBER;
pub(crate) const FLAG_HAS_AUTO_NUMBER: i32 = 20;
// java static final BoolBitSet FLAG_CARD_BEGIN;
pub(crate) const FLAG_CARD_BEGIN: i32 = 21;
// java static final BoolBitSet FLAG_CARD_END;
pub(crate) const FLAG_CARD_END: i32 = 22;
// java static final IntBitSet FLAG_CARD_STYLE;
pub(crate) const FLAG_CARD_STYLE: i32 = 0;

// java public class TypoTextLine extends TypoObjBase
#[derive(Default, Clone)]
pub struct TypoTextLine {
    // java TypoObjBase super_
    pub(crate) super_: TypoObjBase,
    // rust: ★wr-29 05 —— Java 行指针（snapshot.alloc）
    pub(crate) mPointer: i32,
    // java this.mSnapshot = snapshot
    pub(crate) mSnapshot: *mut TypoSnapshot,
    pub(crate) para: i32,
    pub(crate) textRectTop: i32,
    pub(crate) textRectHeight: i32,
    pub(crate) ascentPos: i32,
    pub(crate) descentPos: i32,
    pub(crate) flags: i32,
    pub(crate) graphIndexStart: i32,
    pub(crate) graphIndexEnd: i32,
    pub(crate) yPos: i32,
    pub(crate) start_cp: i32,
    pub(crate) end_cp: i32,
    pub(crate) left: i32,
    pub(crate) width: i32,
    // rust: ★wr-29 07 —— 行盒 top/height（Java snapshot 字段对等）
    pub(crate) top: i32,
    pub(crate) height: i32,
    pub(crate) lineBefore: i32,
    pub(crate) lineAfter: i32,
}

// java public class TypoTextLine extends TypoObjBase
impl TypoTextLine {
    pub fn new() -> Self {
      Self::default()
    }

    // java this.mSnapshot.set(this.mPointer + offset, value)
    fn write_slot(&mut self, offset: i32, value: i32) {
      if self.mPointer != 0 && !self.mSnapshot.is_null() {
        let pointer = self.mPointer + offset;
        let snapshot = self.mSnapshot;
        unsafe { (*snapshot).set__int_int(pointer, value); }
      }
    }

    fn sync_flags(&mut self) {
      let flags = self.flags;
      self.write_slot(OFFSET_FLAG, flags);
    }

    // java private static void checkClassType(int pointer, TypoSnapshot snapshot)
    pub fn checkClassType() {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java checkClassTypeCompatible(TypoMemoryObj.getType(pointer, snapshot), 3);
      TypoType::checkClassTypeCompatible(TypoMemoryObj::getType__int_typosnapshot(pointer, snapshot), 3);
      **/
    }

    // java public int getType()
    pub fn getType(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return 3;
      return 3;
      **/
    }

    // java public int alloc(TypoSnapshot snapshot)
    // rust: ★wr-29 05 —— 真实：snapshot.alloc(26)（Java 数据面）+ 类型头写 3/26
    pub fn alloc(&mut self, snapshot: &mut TypoSnapshot) -> i32 {
      // java this.mPointer = snapshot.alloc(26);
      self.mPointer = snapshot.alloc(26);
      // java snapshot.set(this.mPointer, 3, 26);
      snapshot.set__int_int_int(self.mPointer, 3, 26);
      // java this.mSnapshot = snapshot;
      self.mSnapshot = snapshot as *mut TypoSnapshot;
      // java return this.mPointer;
      self.mPointer
    }

    // java public void setParent(int parent)
    pub fn setParent(&mut self, parent: i32) {
      // java this.mSnapshot.set(this.mPointer + 2, parent);
      self.write_slot(2, parent);
    }

    // java public boolean isPointer(int index)
    pub fn isPointer(&self, index: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return index == 7 || index == 12 || index == 15 || index == 17 || index == 19 || index == 20 || index == 16 || super.isPointer(index);
      return index == 7 || index == 12 || index == 15 || index == 17 || index == 19 || index == 20 || index == 16 || self.super_.isPointer(index);
      **/
    }

    // java public void setPara(int para)
    pub fn setPara__int(&mut self, para: i32) {
      // java this.mSnapshot.set(this.mPointer + 7, para);
      self.para = para;
      self.write_slot(OFFSET_PARA, para);
    }

    // java public static void setPara(int para, int pointer, TypoSnapshot snapshot)
    pub fn setPara__int_int_typosnapshot(_para: i32, _pointer: i32, _snapshot: &mut TypoSnapshot) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.set(pointer + 7, para);
      **/
    }

    // java public int getPara()
    pub fn getPara(&self) -> i32 {
      // java return this.mSnapshot.get(this.mPointer + 7);
      return self.para;
    }

    // java public static int getPara(int pointer, TypoSnapshot snapshot)
    pub fn getPara__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      snapshot.get__int(pointer + 7)
    }

    // java public void setTextRectTop(int textRectTop)
    pub fn setTextRectTop(&mut self, textRectTop: i32) {
      // java this.mSnapshot.set(this.mPointer + 8, textRectTop);
      self.textRectTop = textRectTop;
      self.write_slot(OFFSET_TEXT_RECT_TOP, textRectTop);
    }

    // java public int getTextRectTop()
    pub fn getTextRectTop(&self) -> i32 {
      // java return this.mSnapshot.get(this.mPointer + 8);
      return self.textRectTop;
    }

    // java public int getRelTextRectTop()
    pub fn getRelTextRectTop(&self) -> i32 {
      // java return this.getTextRectTop() + this.getTop();
      if self.mPointer != 0 && !self.mSnapshot.is_null() {
        let snapshot = unsafe { &*self.mSnapshot };
        return snapshot.get__int(self.mPointer + OFFSET_TEXT_RECT_TOP)
            + TypoObjBase::getTop__int_typosnapshot(self.mPointer, snapshot);
      }
      self.textRectTop + self.top
    }

    // java public void setTextRectHeight(int textRectHeight)
    pub fn setTextRectHeight(&mut self, textRectHeight: i32) {
      // java this.mSnapshot.set(this.mPointer + 9, textRectHeight);
      self.textRectHeight = textRectHeight;
      self.write_slot(OFFSET_TEXT_RECT_HEIGHT, textRectHeight);
    }

    // java public void setAscDesPos(int ascentPos, int descentPos)
    // java public void setTop(int top)
    pub fn setTop(&mut self, top: i32) {
      // java TypoRectMemoryImpl.setTop: snapshot.set(rect + 1, top); rect = pointer + 3
      self.top = top;
      self.write_slot(4, top);
    }
    // java public void setHeight(int height)
    pub fn setHeight(&mut self, height: i32) {
      // java setBottom(getTop() + height)
      self.height = height;
      self.write_slot(6, self.top.wrapping_add(height));
    }
    // java public void setLineBefore(int lineBefore)
    pub fn setLineBeforeVal(&mut self, lineBefore: i32) {
      // java this.mSnapshot.set(this.mPointer + 23, value);
      self.lineBefore = lineBefore;
      self.write_slot(OFFSET_LINE_BEFORE, lineBefore);
    }
    // java public void setLineAfter(int lineAfter)
    pub fn setLineAfterVal(&mut self, lineAfter: i32) {
      // java this.mSnapshot.set(this.mPointer + 24, value);
      self.lineAfter = lineAfter;
      self.write_slot(OFFSET_LINE_AFTER, lineAfter);
    }
    pub fn setAscDesPos(&mut self, ascentPos: i32, descentPos: i32) {
      // java this.mSnapshot.set(this.mPointer + 10, ascentPos);
      self.ascentPos = ascentPos;
      self.write_slot(OFFSET_CHANEL_ASC_POS, ascentPos);
      // java this.mSnapshot.set(this.mPointer + 11, descentPos);
      self.descentPos = descentPos;
      self.write_slot(OFFSET_CHANEL_DES_POS, descentPos);
    }

    // java public int getTextRectHeight()
    pub fn getTextRectHeight(&self) -> i32 {
      // java return this.mSnapshot.get(this.mPointer + 9);
      return self.textRectHeight;
    }

    // java public static int getTextRectHeight(int pointer, TypoSnapshot snapshot)
    pub fn getTextRectHeight__int_typosnapshot(_pointer: i32, _snapshot: &TypoSnapshot) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.get(pointer + 9);
      **/
    }

    // java public int getDescentPos()
    pub fn getDescentPos(&self) -> i32 {
      // java return this.mSnapshot.get(this.mPointer + 11);
      return self.descentPos;
    }

    // java public int getAscentPos()
    pub fn getAscentPos(&self) -> i32 {
      // java return this.mSnapshot.get(this.mPointer + 10);
      return self.ascentPos;
    }

    // java public static int getDescentPos(int pointer, TypoSnapshot snapshot)
    pub fn getDescentPos__int_typosnapshot(_pointer: i32, _snapshot: &TypoSnapshot) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.get(pointer + 11);
      **/
    }

    // java public int getTextRectBottom()
    pub fn getTextRectBottom(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.mSnapshot;
      let mut snapshot: TypoSnapshot = self.mSnapshot;
      // java return TypoRectMemoryImpl.getTop(this.mPointer + 3, snapshot) + snapshot.get(this.mPointer + 8) + snapshot.get(this.mPointer + 9);
      return TypoRectMemoryImpl::getTop__int_typosnapshot(self.mPointer + 3, snapshot) + snapshot.get(self.mPointer + 8) + snapshot.get(self.mPointer + 9);
      **/
    }

    // java public static int getTextRectBottom(int line, TypoSnapshot snapshot)
    pub fn getTextRectBottom__int_typosnapshot(line: i32, snapshot: &TypoSnapshot) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return TypoRectMemoryImpl.getTop(line + 3, snapshot) + snapshot.get(line + 8) + snapshot.get(line + 9);
      return TypoRectMemoryImpl::getTop__int_typosnapshot(line + 3, snapshot) + snapshot.get__int(line + 8) + snapshot.get__int(line + 9);
      **/
    }

    // java public void offsetTextRectTop(int offset)
    pub fn offsetTextRectTop(&mut self, offset: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSnapshot.extend(this.mPointer + 8, offset);
      self.mSnapshot.extend(self.mPointer + 8, offset);
      **/
    }

    // java public void setFlags(int flags)
    pub fn setFlags(&mut self, flags: i32) {
      // java this.mSnapshot.set(this.mPointer + 12, flags);
      self.flags = flags;
      self.write_slot(OFFSET_FLAG, flags);
    }

    // java public int getFlags()
    pub fn getFlags(&self) -> i32 {
      // java return this.mSnapshot.get(this.mPointer + 12);
      return self.flags;
    }

    // java public static int getFlags(int pointer, TypoSnapshot snapshot)
    pub fn getFlags__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      snapshot.get__int(pointer + 12)
    }

    // java public int getGraphIndexStart()
    pub fn getGraphIndexStart(&self) -> i32 {
      // java return this.mSnapshot.get(this.mPointer + 13);
      return self.graphIndexStart;
    }

    // java public static int getGraphIndexStart(int pointer, TypoSnapshot snapshot)
    pub fn getGraphIndexStart__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      snapshot.get__int(pointer + 13)
    }

    // java public void setGraphIndexStart(int graphIndexStart)
    pub fn setGraphIndexStart(&mut self, graphIndexStart: i32) {
      // java this.mSnapshot.set(this.mPointer + 13, graphIndexStart);
      self.graphIndexStart = graphIndexStart;
      self.write_slot(OFFSET_GRAPH_INDEX_START, graphIndexStart);
    }

    // java public int getGraphIndexEnd()
    pub fn getGraphIndexEnd(&self) -> i32 {
      // java return this.mSnapshot.get(this.mPointer + 14);
      return self.graphIndexEnd;
    }

    // java public static int getGraphIndexEnd(int pointer, TypoSnapshot snapshot)
    pub fn getGraphIndexEnd__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      // java return snapshot.get(pointer + 14);
      snapshot.get__int(pointer + 14)
    }

    // java public void setGraphIndexEnd(int graphIndexEnd)
    pub fn setGraphIndexEnd(&mut self, graphIndexEnd: i32) {
      // java this.mSnapshot.set(this.mPointer + 14, graphIndexEnd);
      self.graphIndexEnd = graphIndexEnd;
      self.write_slot(OFFSET_GRAPH_INDEX_END, graphIndexEnd);
    }

    // java public static int getGraphLength(int pointer, TypoSnapshot snapshot)
    pub fn getGraphLength(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return getGraphIndexEnd(pointer, snapshot) - getGraphIndexStart(pointer, snapshot) + 1;
      return Self::getGraphIndexEnd__int_typosnapshot(pointer, snapshot) - Self::getGraphIndexStart__int_typosnapshot(pointer, snapshot) + 1;
      **/
    }

    // java public void setYPos(int yPos)
    pub fn setYPos(&mut self, yPos: i32) {
      // java this.mSnapshot.set(this.mPointer + 22, yPos);
      self.yPos = yPos;
    }

    // java public int getYPos()
    pub fn getYPos(&self) -> i32 {
      // java return this.mSnapshot.get(this.mPointer + 22);
      return self.yPos;
    }

    // java public static int getYPos(int pointer, TypoSnapshot snapshot)
    pub fn getYPos__int_typosnapshot(_pointer: i32, _snapshot: &TypoSnapshot) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.get(pointer + 22);
      **/
    }

    // java public char getEndChar()
    pub fn getEndChar(&self) -> char {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int pPara = this.getPara();
      let mut pPara: i32 = self.getPara();
      // java if (pPara != 0)
      if pPara != 0 {
        // java TypoPara para = this.mSnapshot.getTypoTools().obtainTypoPara(pPara);
        let mut para: TypoPara = self.mSnapshot.getTypoTools().obtainTypoPara(pPara);
        // java char c = para.getChar(this.getGraphIndexEnd());
        let mut c: char = para.getChar(self.getGraphIndexEnd());
        // java this.mSnapshot.getTypoTools().recycle((TypoMemoryObj)para);
        self.mSnapshot.getTypoTools().recycle((para).downcast_ref::<TypoMemoryObj>().unwrap());
        // java return c;
        return c;
      }
      // java else
      else {
        // java return '\u0000';
        return '\u{0000}';
      }
      **/
    }

    // java public boolean hasSHDs()
    pub fn hasSHDs(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int pSHDs = this.getSHDs();
      let mut pSHDs: i32 = self.getSHDs();
      // java return pSHDs != 0 && TypoSHDs.size(pSHDs, this.mSnapshot) > 0;
      return pSHDs != 0 && TypoSHDs::size(pSHDs, self.mSnapshot) > 0;
      **/
    }

    // java public int getSHDs()
    pub fn getSHDs(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSnapshot.get(this.mPointer + 15);
      return self.mSnapshot.get(self.mPointer + 15);
      **/
    }

    // java public static int getSHDs(int pointer, TypoSnapshot snapshot)
    pub fn getSHDs__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.get(pointer + 15);
      return snapshot.get__int(pointer + 15);
      **/
    }

    // java public void setSHDs(int pointer)
    pub fn setSHDs(&mut self, pointer: i32) {
      // java this.mSnapshot.set(this.mPointer + 15, pointer);
      self.write_slot(OFFSET_SHDS, pointer);
    }

    // java public int getBorders()
    pub fn getBorders(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSnapshot.get(this.mPointer + 16);
      return self.mSnapshot.get(self.mPointer + 16);
      **/
    }

    // java public static int getBorders(int pointer, TypoSnapshot snapshot)
    pub fn getBorders__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.get(pointer + 16);
      return snapshot.get__int(pointer + 16);
      **/
    }

    // java public void setBorders(int pointer)
    pub fn setBorders__int(&mut self, pointer: i32) {
      // java this.mSnapshot.set(this.mPointer + 16, pointer);
      self.write_slot(OFFSET_BORDERS, pointer);
    }

    // java public static void setBorders(int pBorder, int pointer, TypoSnapshot snapshot)
    pub fn setBorders__int_int_typosnapshot(pBorder: i32, pointer: i32, snapshot: &mut TypoSnapshot) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.set(pointer + 16, pBorder);
      snapshot.set__int_int(pointer + 16, pBorder);
      **/
    }

    // java public int getHighlights()
    pub fn getHighlights(&self) -> i32 {
      // java return this.mSnapshot.get(this.mPointer + 17);
      if self.mSnapshot.is_null() { 0 } else { unsafe { (*self.mSnapshot).get__int(self.mPointer + OFFSET_HIGHLIGHTS) } }
    }

    // java public static int getHighlights(int pointer, TypoSnapshot snapshot)
    pub fn getHighlights__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.get(pointer + 17);
      return snapshot.get__int(pointer + 17);
      **/
    }

    // java public void setHighlights(int point)
    pub fn setHighlights(&mut self, point: i32) {
      // java this.mSnapshot.set(this.mPointer + 17, point);
      self.write_slot(OFFSET_HIGHLIGHTS, point);
    }

    // java public int getDocFiedls()
    pub fn getDocFiedls(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSnapshot.get(this.mPointer + 18);
      return self.mSnapshot.get(self.mPointer + 18);
      **/
    }

    // java public static int getDocFields(int pointer, TypoSnapshot snapshot)
    pub fn getDocFields(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.get(pointer + 18);
      return snapshot.get__int(pointer + 18);
      **/
    }

    // java public void setDocFields(int pointer)
    pub fn setDocFields(&mut self, pointer: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSnapshot.set(this.mPointer + 18, pointer);
      self.mSnapshot.set(self.mPointer + 18, pointer);
      **/
    }

    // java public int getLines()
    pub fn getLines(&self) -> i32 {
      // java return this.mSnapshot.get(this.mPointer + 19);
      if self.mSnapshot.is_null() {
        0
      } else {
        unsafe { (*self.mSnapshot).get__int(self.mPointer + OFFSET_LINES) }
      }
    }

    // java public static int getLines(int pointer, TypoSnapshot snapshot)
    pub fn getLines__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      // java return snapshot.get(pointer + 19);
      snapshot.get__int(pointer + 19)
    }

    // java public void setLines(int lines)
    pub fn setLines__int(&mut self, lines: i32) {
      // java this.mSnapshot.set(this.mPointer + 19, lines);
      self.write_slot(OFFSET_LINES, lines);
    }

    // java public static void setLines(int lines, int pointer, TypoSnapshot snapshot)
    pub fn setLines__int_int_typosnapshot(lines: i32, pointer: i32, snapshot: &mut TypoSnapshot) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.set(pointer + 19, lines);
      snapshot.set__int_int(pointer + 19, lines);
      **/
    }

    // java public static boolean isFirstLineOfMathPara(int pointer, TypoSnapshot snapshot)
    pub fn isFirstLineOfMathPara__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.getBoolBitSet(pointer + 12, FLAG_FIRSTLINEOFMATHPARA);
      return snapshot.getBoolBitSet(pointer + 12, FLAG_FIRSTLINEOFMATHPARA);
      **/
    }

    // java public boolean isFirstLineOfMathPara()
    pub fn isFirstLineOfMathPara(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSnapshot.getBoolBitSet(this.mPointer + 12, FLAG_FIRSTLINEOFMATHPARA);
      return self.mSnapshot.getBoolBitSet(self.mPointer + 12, FLAG_FIRSTLINEOFMATHPARA);
      **/
    }

    // java public static void setIsInMathPara(boolean value, int pointer, TypoSnapshot snapshot)
    pub fn setIsInMathPara(value: bool, pointer: i32, snapshot: &mut TypoSnapshot) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.setBoolBitSet(pointer + 12, FLAG_INMATHPARA, value);
      snapshot.setBoolBitSet(pointer + 12, FLAG_INMATHPARA, value);
      **/
    }

    // java public static void setIsFirstLineOfMathPara(boolean value, int pointer, TypoSnapshot snapshot)
    pub fn setIsFirstLineOfMathPara(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.setBoolBitSet(pointer + 12, FLAG_FIRSTLINEOFMATHPARA, value);
      snapshot.setBoolBitSet(pointer + 12, FLAG_FIRSTLINEOFMATHPARA, value);
      **/
    }

    // java public static void setIsLastLineOfMathPara(boolean value, int pointer, TypoSnapshot snapshot)
    pub fn setIsLastLineOfMathPara(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.setBoolBitSet(pointer + 12, FLAG_LASTLINEOFMATHPARA, value);
      snapshot.setBoolBitSet(pointer + 12, FLAG_LASTLINEOFMATHPARA, value);
      **/
    }

    // java public void setFixedHeight(boolean value)
    pub fn setFixedHeight(&mut self) {
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_FIXEDHEIGHT, value);
      self.flags = BoolBitSet::new(FLAG_FIXEDHEIGHT).set(self.flags, true);
      self.sync_flags();
    }

    // java public boolean isFixedHeight()
    pub fn isFixedHeight(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return FLAG_FIXEDHEIGHT.get(this.mSnapshot.get(this.mPointer + 12));
      return FLAG_FIXEDHEIGHT::get(self.mSnapshot.get(self.mPointer + 12));
      **/
    }

    // java public static void setExtendLastSHDAndBrc(boolean value, int pLine, TypoSnapshot snapshot)
    pub fn setExtendLastSHDAndBrc(value: bool, pLine: i32, snapshot: &mut TypoSnapshot) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.setBoolBitSet(pLine + 12, FLAG_EXTEND_LASTSHDANDBRC, value);
      snapshot.setBoolBitSet(pLine + 12, FLAG_EXTEND_LASTSHDANDBRC, value);
      **/
    }

    // java public static boolean isExtendLastSHDAndBrc(int pLine, TypoSnapshot snapshot)
    pub fn isExtendLastSHDAndBrc(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return FLAG_EXTEND_LASTSHDANDBRC.get(snapshot.get(pLine + 12));
      return FLAG_EXTEND_LASTSHDANDBRC::get(snapshot.get__int(pLine + 12));
      **/
    }

    // java public boolean isLastLineOfPara()
    pub fn isLastLineOfPara(&self) -> bool {
      // java return FLAG_LASTLINEOFPARA.get(this.getFlags());
      return BoolBitSet::new(FLAG_LASTLINEOFPARA).get(self.getFlags());
    }

    // java public static boolean isLastLineOfPara(int line, TypoSnapshot snapshot)
    pub fn isLastLineOfPara__int_typosnapshot(line: i32, snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return FLAG_LASTLINEOFPARA.get(snapshot.get(line + 12));
      return FLAG_LASTLINEOFPARA::get(snapshot.get__int(line + 12));
      **/
    }

    // java public boolean isLastLineOfSection()
    pub fn isLastLineOfSection(&self) -> bool {
      // java return FLAG_LASTLINEOFSECTION.get(this.getFlags());
      BoolBitSet::new(FLAG_LASTLINEOFSECTION).get(self.getFlags())
    }

    // java public static boolean isLastLineOfSection(int line, TypoSnapshot snapshot)
    pub fn isLastLineOfSection__int_typosnapshot(line: i32, snapshot: &TypoSnapshot) -> bool {
      // java return FLAG_LASTLINEOFSECTION.get(snapshot.get(line + 12));
      BoolBitSet::new(FLAG_LASTLINEOFSECTION).get(snapshot.get__int(line + 12))
    }

    // java public void setFirstLineOfPara(boolean value)
    pub fn setFirstLineOfPara(&mut self, value: bool) {
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_FIRSTLINEOFPARA, value);
      self.flags = BoolBitSet::new(FLAG_FIRSTLINEOFPARA).set(self.flags, value);
      self.sync_flags();
    }

    // java public void setLastLineOfPara(boolean value)
    pub fn setLastLineOfPara(&mut self, value: bool) {
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_LASTLINEOFPARA, value);
      self.flags = BoolBitSet::new(FLAG_LASTLINEOFPARA).set(self.flags, value);
      self.sync_flags();
    }

    // java public void setLastLineOfSection(boolean value)
    pub fn setLastLineOfSection(&mut self) {
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_LASTLINEOFSECTION, value);
      self.flags = BoolBitSet::new(FLAG_LASTLINEOFSECTION).set(self.flags, true);
      self.sync_flags();
    }

    // java public void setLastLineOfPage(boolean value)
    pub fn setLastLineOfPage(&mut self, value: bool) {
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_LASTLINEOFPAGE, value);
      self.flags = BoolBitSet::new(FLAG_LASTLINEOFPAGE).set(self.flags, value);
      self.sync_flags();
    }

    // java public void setLastLineOfColumn(boolean value)
    pub fn setLastLineOfColumn(&mut self) {
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_LASTLINEOFCOLUMN, value);
      self.flags = BoolBitSet::new(FLAG_LASTLINEOFCOLUMN).set(self.flags, true);
      self.sync_flags();
    }

    // java public void setRTL(boolean value)
    pub fn setRTL(&mut self) {
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_RTL, value);
      self.flags = BoolBitSet::new(FLAG_RTL).set(self.flags, true);
      self.sync_flags();
    }

    // java public boolean hasComment()
    pub fn hasComment(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSnapshot.getBoolBitSet(this.mPointer + 12, FLAG_COMMENT);
      return self.mSnapshot.getBoolBitSet(self.mPointer + 12, FLAG_COMMENT);
      **/
    }

    // java public static boolean hasComment(int pointer, TypoSnapshot snapshot)
    pub fn hasComment__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.getBoolBitSet(pointer + 12, FLAG_COMMENT);
      return snapshot.getBoolBitSet(pointer + 12, FLAG_COMMENT);
      **/
    }

    // java public static void setHasComment(boolean value, int pointer, TypoSnapshot snapshot)
    pub fn setHasComment(value: bool, pointer: i32, snapshot: &mut TypoSnapshot) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.setBoolBitSet(pointer + 12, FLAG_COMMENT, value);
      snapshot.setBoolBitSet(pointer + 12, FLAG_COMMENT, value);
      **/
    }

    // java public boolean hasEmbedsDrawing()
    pub fn hasEmbedsDrawing(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSnapshot.getBoolBitSet(this.mPointer + 12, FLAG_HAS_IMBEDS_DRAWING);
      return self.mSnapshot.getBoolBitSet(self.mPointer + 12, FLAG_HAS_IMBEDS_DRAWING);
      **/
    }

    // java public static boolean hasEmbedsDrawing(int pointer, TypoSnapshot snapshot)
    pub fn hasEmbedsDrawing__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.getBoolBitSet(pointer + 12, FLAG_HAS_IMBEDS_DRAWING);
      return snapshot.getBoolBitSet(pointer + 12, FLAG_HAS_IMBEDS_DRAWING);
      **/
    }

    // java public void setHasEmbedsDrawing(boolean value)
    pub fn setHasEmbedsDrawing(&mut self, value: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_HAS_IMBEDS_DRAWING, value);
      self.mSnapshot.setBoolBitSet(self.mPointer + 12, FLAG_HAS_IMBEDS_DRAWING, value);
      **/
    }

    // java public boolean hasNotClipEmbeds()
    pub fn hasNotClipEmbeds(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSnapshot.getBoolBitSet(this.mPointer + 12, FLAG_HAS_NOT_CLIP_EMBEDS);
      return self.mSnapshot.getBoolBitSet(self.mPointer + 12, FLAG_HAS_NOT_CLIP_EMBEDS);
      **/
    }

    // java public static boolean hasNotClipEmbeds(int pointer, TypoSnapshot snapshot)
    pub fn hasNotClipEmbeds__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.getBoolBitSet(pointer + 12, FLAG_HAS_NOT_CLIP_EMBEDS);
      return snapshot.getBoolBitSet(pointer + 12, FLAG_HAS_NOT_CLIP_EMBEDS);
      **/
    }

    // java public void setHasNotClipEmbeds(boolean value)
    pub fn setHasNotClipEmbeds(&mut self, value: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_HAS_NOT_CLIP_EMBEDS, value);
      self.mSnapshot.setBoolBitSet(self.mPointer + 12, FLAG_HAS_NOT_CLIP_EMBEDS, value);
      **/
    }

    // java public static boolean isTemplateContent(int pointer, TypoSnapshot snapshot)
    pub fn isTemplateContent(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.getBoolBitSet(pointer + 12, FLAG_ISTEMPLATECONTENT);
      return snapshot.getBoolBitSet(pointer + 12, FLAG_ISTEMPLATECONTENT);
      **/
    }

    // java public static void setIsTemplateContent(boolean value, int pointer, TypoSnapshot snapshot)
    pub fn setIsTemplateContent(value: bool, pointer: i32, snapshot: &mut TypoSnapshot) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.setBoolBitSet(pointer + 12, FLAG_ISTEMPLATECONTENT, value);
      snapshot.setBoolBitSet(pointer + 12, FLAG_ISTEMPLATECONTENT, value);
      **/
    }

    // java public static boolean hasAutoNumber(int pointer, TypoSnapshot snapshot)
    pub fn hasAutoNumber__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.getBoolBitSet(pointer + 12, FLAG_HAS_AUTO_NUMBER);
      return snapshot.getBoolBitSet(pointer + 12, FLAG_HAS_AUTO_NUMBER);
      **/
    }

    // java public boolean hasAutoNumber()
    pub fn hasAutoNumber(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return hasAutoNumber(this.mPointer, this.mSnapshot);
      return Self::hasAutoNumber__int_typosnapshot(self.mPointer, self.mSnapshot);
      **/
    }

    // java public void setHasAutoNumber(boolean value)
    pub fn setHasAutoNumber(&mut self, value: bool) {
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_HAS_AUTO_NUMBER, value);
      self.flags = BoolBitSet::new(FLAG_HAS_AUTO_NUMBER).set(self.flags, value);
      self.sync_flags();
    }

    // java public void setHasSubLine(boolean value)
    pub fn setHasSubLine(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_HAS_SUBLINE, value);
      self.mSnapshot.setBoolBitSet(self.mPointer + 12, FLAG_HAS_SUBLINE, value);
      **/
    }

    // java public boolean hasSubLine()
    pub fn hasSubLine(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSnapshot.getBoolBitSet(this.mPointer + 12, FLAG_HAS_SUBLINE);
      return self.mSnapshot.getBoolBitSet(self.mPointer + 12, FLAG_HAS_SUBLINE);
      **/
    }

    // java public void setHasBidi(boolean value)
    pub fn setHasBidi(&mut self, value: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_HASBIDI, value);
      self.mSnapshot.setBoolBitSet(self.mPointer + 12, FLAG_HASBIDI, value);
      **/
    }

    // java public boolean hasBidi()
    pub fn hasBidi(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSnapshot.getBoolBitSet(this.mPointer + 12, FLAG_HASBIDI);
      return self.mSnapshot.getBoolBitSet(self.mPointer + 12, FLAG_HASBIDI);
      **/
    }

    // java public boolean isFirstLineOfPara()
    pub fn isFirstLineOfPara(&self) -> bool {
      // java return FLAG_FIRSTLINEOFPARA.get(this.getFlags());
      return BoolBitSet::new(FLAG_FIRSTLINEOFPARA).get(self.getFlags());
    }

    // java public static boolean isInMathPara(int pointer, TypoSnapshot snapshot)
    pub fn isInMathPara(pointer: i32, snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return FLAG_INMATHPARA.get(getFlags(pointer, snapshot));
      return FLAG_INMATHPARA::get(Self::getFlags__int_typosnapshot(pointer, snapshot));
      **/
    }

    // java public static boolean isLastLineOfMathPara(int pointer, TypoSnapshot snapshot)
    pub fn isLastLineOfMathPara(pointer: i32, snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return FLAG_LASTLINEOFMATHPARA.get(getFlags(pointer, snapshot));
      return FLAG_LASTLINEOFMATHPARA::get(Self::getFlags__int_typosnapshot(pointer, snapshot));
      **/
    }

    // java public static boolean isLastLineOfPage(int pointer, TypoSnapshot snapshot)
    pub fn isLastLineOfPage(pointer: i32, snapshot: &TypoSnapshot) -> bool {
      // java return FLAG_LASTLINEOFPAGE.get(getFlags(pointer, snapshot));
      BoolBitSet::new(FLAG_LASTLINEOFPAGE).get(snapshot.get__int(pointer + 12))
    }

    pub fn isLastLineOfPage__self(&self) -> bool {
      BoolBitSet::new(FLAG_LASTLINEOFPAGE).get(self.getFlags())
    }

    // java public static boolean isLastLineOfColumn(int pointer, TypoSnapshot snapshot)
    pub fn isLastLineOfColumn(pointer: i32, snapshot: &TypoSnapshot) -> bool {
      // java return FLAG_LASTLINEOFCOLUMN.get(getFlags(pointer, snapshot));
      BoolBitSet::new(FLAG_LASTLINEOFCOLUMN).get(snapshot.get__int(pointer + 12))
    }

    // java public static boolean isFirstLineOfPara(int pointer, TypoSnapshot snapshot)
    pub fn isFirstLineOfPara__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> bool {
      BoolBitSet::new(FLAG_FIRSTLINEOFPARA).get(Self::getFlags__int_typosnapshot(pointer, snapshot))
    }

    // java public int getStartCP()
    pub fn getStartCP(&self) -> i32 {
      // java return getStartCP(this.mPointer, this.mSnapshot);
      return self.start_cp;
    }

    // java public int getEndCP()
    pub fn getEndCP(&self) -> i32 {
      // java return getEndCP(this.mPointer, this.mSnapshot);
      return self.end_cp;
    }

    pub fn setStartCP(&mut self, start: i32) {
      self.start_cp = start;
    }

    pub fn setEndCP(&mut self, end: i32) {
      self.end_cp = end;
    }

    pub fn setLeft(&mut self, left: i32) {
      // java TypoRectMemoryImpl.setLeft: snapshot.set(rect + 0, left); rect = pointer + 3
      self.left = left;
      self.write_slot(3, left);
    }

    pub fn getLeft(&self) -> i32 {
      self.left
    }

    pub fn setWidth(&mut self, width: i32) {
      // java setRight(getLeft() + width)
      self.width = width;
      self.write_slot(5, self.left.wrapping_add(width));
    }

    pub fn getWidth(&self) -> i32 {
      self.width
    }

    pub fn contains__int(&self, cp: i32) -> bool {
      cp >= self.start_cp && cp < self.end_cp
    }

    // java public static int getStartCP(int pointer, TypoSnapshot snapshot)
    pub fn getStartCP__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      let p_para = snapshot.get__int(pointer + 7);
      if p_para != 0 {
        TypoPara::getStartCP__int_int_typosnapshot(snapshot.get__int(pointer + 13), p_para, snapshot)
      } else {
        -1
      }
    }

    // java public static int getFirstValidCp(int pointer, TypoSnapshot snapshot)
    pub fn getFirstValidCp(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int pPara = snapshot.get(pointer + 7);
      let mut pPara: i32 = snapshot.get__int(pointer + 7);
      // java return 0 != pPara ? TypoPara.getFirstValidCP(snapshot.get(pointer + 13), snapshot.get(pointer + 14), pPara, snapshot) : -1;
      return (if 0 != pPara { TypoPara::getFirstValidCP(snapshot.get__int(pointer + 13), snapshot.get__int(pointer + 14), pPara, snapshot) } else { -1 });
      **/
    }

    // java public static int getEndCP(int pointer, TypoSnapshot snapshot)
    pub fn getEndCP__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      let p_para = snapshot.get__int(pointer + 7);
      if p_para != 0 {
        TypoPara::getEndCP__int_int_typosnapshot(snapshot.get__int(pointer + 14), p_para, snapshot)
      } else {
        -1
      }
    }

    // java public static boolean contains(int cp, int pointer, TypoSnapshot snapshot)
    pub fn contains(_cp: i32, _pointer: i32, _snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      // java int startCp = getStartCP(pointer, snapshot);
      let startCp: i32 = 0;
      let _ = startCp;
      // java return cp >= startCp && cp < getEndCP(pointer, snapshot);
      /* DEFERRED_NON_FUNCTION
      **/
    }

    // java public static int getCPRef(int pointer, TypoSnapshot snapshot)
    pub fn getCPRef(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int pPara = snapshot.get(pointer + 7);
      let mut pPara: i32 = snapshot.get__int(pointer + 7);
      // java return 0 != pPara ? TypoPara.getCPRef(pPara, snapshot) : 0;
      return (if 0 != pPara { TypoPara::getCPRef(pPara, snapshot) } else { 0 });
      **/
    }

    // java public void setSHDAndBRCs(int shdAndBRCs)
    pub fn setSHDAndBRCs__int(&mut self, shdAndBRCs: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSnapshot.set(this.mPointer + 20, shdAndBRCs);
      self.mSnapshot.set(self.mPointer + 20, shdAndBRCs);
      **/
    }

    // java public static void setSHDAndBRCs(int shdAndBRCs, int pointer, TypoSnapshot snapshot)
    pub fn setSHDAndBRCs__int_int_typosnapshot(shdAndBRCs: i32, pointer: i32, snapshot: &mut TypoSnapshot) {
      // java snapshot.set(pointer + 20, shdAndBRCs);
      snapshot.set__int_int(pointer + 20, shdAndBRCs);
    }

    // java public int getSHDAndBRCs()
    pub fn getSHDAndBRCs(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSnapshot.get(this.mPointer + 20);
      return self.mSnapshot.get(self.mPointer + 20);
      **/
    }

    // java public static int getSHDAndBRCs(int pointer, TypoSnapshot snapshot)
    pub fn getSHDAndBRCs__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      // java return snapshot.get(pointer + 20);
      snapshot.get__int(pointer + 20)
    }

    // java public final boolean isRTL()
    pub fn isRTL(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return FLAG_RTL.get(this.getFlags());
      return FLAG_RTL::get(self.getFlags());
      **/
    }

    // java public void setMemEnd(int memEnd)
    pub fn setMemEnd(&mut self, memEnd: i32) {
      // java this.mSnapshot.set(this.mPointer + 21, memEnd);
      self.write_slot(OFFSET_MEM_END, memEnd);
    }

    // java public static final int getMemEnd(int pointer, TypoSnapshot snapshot)
    pub fn getMemEnd__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      // java return snapshot.get(pointer + 21);
      snapshot.get__int(pointer + OFFSET_MEM_END)
    }

    pub fn getMemEnd(&self) -> i32 {
      if self.mPointer == 0 || self.mSnapshot.is_null() { return 0; }
      let snapshot = unsafe { &*self.mSnapshot };
      Self::getMemEnd__int_typosnapshot(self.mPointer, snapshot)
    }

    // java public void rectForPageCore(ITypoRect rect)
    pub fn rectForPageCore(&mut self, rect: &mut dyn ITypoRect) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java rect.set(this);
      rect.set__ityporect(self);
      // java if (this.isRTL())
      if self.isRTL() {
        // java int width = this.width();
        let mut width: i32 = self.super_.width();
        // java rect.setRight(TypoPage.getCoreWidth(this.getPage(), this.mSnapshot) - this.getLeft());
        rect.setRight(TypoPage::getCoreWidth__int_typosnapshot(self.super_.getPage(), self.mSnapshot) - self.super_.getLeft());
        // java rect.setLeft(rect.getRight() - width);
        rect.setLeft(rect.getRight() - width);
      }
      **/
    }

    // java public static TypoTextLine newInstance()
    pub fn newInstance() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return new TypoTextLine();
      return { let __x: Self = todo!("no align strategy matched"); __x };
      **/
    }

    // java public static TypoMemoryObj.TypoObjectFactory<TypoTextLine> newObjecctFactory()
    pub fn newObjecctFactory() -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoMemoryObj.TypoObjectFactory<TypoTextLine> factory = new TypoMemoryObj.TypoObjectFactory<TypoTextLine>() { public final TypoTextLine newObject() { return TypoTextLine.newInstance(); } public Class<TypoTextLine> getTClass() { return TypoTextLine.class; } };
      let mut factory: TypoObjectFactory = TypoObjectFactory::new();
      // java return factory;
      return factory;
      **/
    }

    // java public final TypoTextLine newObject()
    pub fn newObject() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return TypoTextLine.newInstance();
      return TypoTextLine::newInstance();
      */
    }

    // java public Class<TypoTextLine> getTClass()
    pub fn getTClass() {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return TypoTextLine.class;
      // rust: no-stmt
      */
    }

    // java public void commitOp(int[] ops, int opSize)
    pub fn commitOp(&mut self, ops: &[i32], opSize: i32) {
      // java TypoSnapshot snapshort = this.mSnapshot;
      if self.mSnapshot.is_null() {
        return;
      }
      let snapshort = unsafe { &mut *self.mSnapshot };
      // java int oldAllocSize = snapshort.get(this.mPointer + 1);
      let oldAllocSize = snapshort.get__int(self.mPointer + 1);
      let opSize = opSize.max(0);
      // java this.mPointer = snapshort.extendAlloc(this.mPointer, oldAllocSize, opSize);
      self.mPointer = snapshort.extendAlloc(self.mPointer, oldAllocSize, opSize);
      // java snapshort.extend(this.mPointer + 1, opSize);
      snapshort.extend(self.mPointer + 1, opSize);
      // java snapshort.set(this.mPointer + oldAllocSize, (int[])ops, 0, opSize);
      let base = self.mPointer + oldAllocSize;
      let n = (opSize as usize).min(ops.len());
      for i in 0..n {
        snapshort.set__int_int(base + i as i32, ops[i]);
      }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java StringBuffer sb = new StringBuffer();
      let mut sb: String = { let __x: String = todo!("no align strategy matched"); __x };
      // java int para = this.getPara();
      let mut para: i32 = self.getPara();
      // java if (0 != para)
      if 0 != para {
        // java sb.append("Ranges[" + this.getStartCP() + ", " + this.getEndCP() + "]   ");
        sb.append("Ranges[" + self.getStartCP() + ", " + self.getEndCP() + "]   ");
        // java TypoPara typoPara = this.mSnapshot.getTypoTools().obtainTypoPara(para);
        let mut typoPara: TypoPara = self.mSnapshot.getTypoTools().obtainTypoPara(para);
        // java int startGraphIndex = this.getGraphIndexStart();
        let mut startGraphIndex: i32 = self.getGraphIndexStart();
        // java int endGraphIndex = this.getGraphIndexEnd();
        let mut endGraphIndex: i32 = self.getGraphIndexEnd();
        // java typoPara.getChars(startGraphIndex, endGraphIndex - startGraphIndex + 1, sb);
        typoPara.getChars(startGraphIndex, endGraphIndex - startGraphIndex + 1, sb);
        // java this.mSnapshot.getTypoTools().recycle((TypoMemoryObj)typoPara);
        self.mSnapshot.getTypoTools().recycle((typoPara).downcast_ref::<TypoMemoryObj>().unwrap());
      }
      // java return super.toString() + " " + sb.toString();
      return self.super_.toString() + " " + sb.toString();
      **/
    }

    // java public String getLineChars()
    pub fn getLineChars(&self) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java StringBuffer sb = new StringBuffer();
      let mut sb: String = { let __x: String = todo!("no align strategy matched"); __x };
      // java int para = this.getPara();
      let mut para: i32 = self.getPara();
      // java if (0 != para)
      if 0 != para {
        // java TypoPara typoPara = this.mSnapshot.getTypoTools().obtainTypoPara(para);
        let mut typoPara: TypoPara = self.mSnapshot.getTypoTools().obtainTypoPara(para);
        // java int startGraphIndex = this.getGraphIndexStart();
        let mut startGraphIndex: i32 = self.getGraphIndexStart();
        // java int endGraphIndex = this.getGraphIndexEnd();
        let mut endGraphIndex: i32 = self.getGraphIndexEnd();
        // java typoPara.getChars(startGraphIndex, endGraphIndex - startGraphIndex + 1, sb);
        typoPara.getChars(startGraphIndex, endGraphIndex - startGraphIndex + 1, sb);
        // java this.mSnapshot.getTypoTools().recycle((TypoMemoryObj)typoPara);
        self.mSnapshot.getTypoTools().recycle((typoPara).downcast_ref::<TypoMemoryObj>().unwrap());
      }
      // java return sb.toString();
      return sb.toString();
      **/
    }

    // java public static boolean isCardBegin(int pLine, TypoSnapshot snapshot)
    pub fn isCardBegin(pLine: i32, snapshot: &TypoSnapshot) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.getBoolBitSet(pLine + 12, FLAG_CARD_BEGIN);
      return snapshot.getBoolBitSet(pLine + 12, FLAG_CARD_BEGIN);
      **/
    }

    // java public static void setCardBegin(int pLine, boolean value, TypoSnapshot snapshot)
    pub fn setCardBegin__int_boolean_typosnapshot(pLine: i32, value: bool, snapshot: &mut TypoSnapshot) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.setBoolBitSet(pLine + 12, FLAG_CARD_BEGIN, value);
      snapshot.setBoolBitSet(pLine + 12, FLAG_CARD_BEGIN, value);
      **/
    }

    // java public static boolean isCardEnd(int pLine, TypoSnapshot snapshot)
    pub fn isCardEnd(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.getBoolBitSet(pLine + 12, FLAG_CARD_END);
      return snapshot.getBoolBitSet(pLine + 12, FLAG_CARD_END);
      **/
    }

    // java public static void setCardEnd(int pLine, boolean value, TypoSnapshot snapshot)
    pub fn setCardEnd__int_boolean_typosnapshot(pLine: i32, value: bool, snapshot: &mut TypoSnapshot) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.setBoolBitSet(pLine + 12, FLAG_CARD_END, value);
      snapshot.setBoolBitSet(pLine + 12, FLAG_CARD_END, value);
      **/
    }

    // java public void setLineBefore(int value)
    pub fn setLineBefore(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSnapshot.set(this.mPointer + 23, value);
      self.mSnapshot.set(self.mPointer + 23, value);
      **/
    }

    // java public int getLineBefore()
    pub fn getLineBefore(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSnapshot.get(this.mPointer + 23);
      return self.mSnapshot.get(self.mPointer + 23);
      **/
    }

    // java public static int getLineBefore(int pointer, TypoSnapshot snapshot)
    pub fn getLineBefore__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.get(pointer + 23);
      return snapshot.get__int(pointer + 23);
      **/
    }

    // java public void setLineAfter(int value)
    pub fn setLineAfter(&mut self, value: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mSnapshot.set(this.mPointer + 24, value);
      self.mSnapshot.set(self.mPointer + 24, value);
      **/
    }

    // java public int getLineAfter()
    pub fn getLineAfter(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSnapshot.get(this.mPointer + 24);
      return self.mSnapshot.get(self.mPointer + 24);
      **/
    }

    // java public static int getLineAfter(int pointer, TypoSnapshot snapshot)
    pub fn getLineAfter__int_typosnapshot(pointer: i32, snapshot: &TypoSnapshot) -> i32 {
      // java return snapshot.get(pointer + 24);
      snapshot.get__int(pointer + 24)
    }

    // java public void setCardBegin(boolean value)
    // rust: ★wr-29 07 —— mirror 写（Java snapshot flag）
    pub fn setCardBegin__boolean(&mut self, value: bool) {
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_CARD_BEGIN, value);
      self.flags = BoolBitSet::new(FLAG_CARD_BEGIN).set(self.flags, value);
      self.sync_flags();
    }

    // java public void setCardEnd(boolean value)
    pub fn setCardEnd__boolean(&mut self, value: bool) {
      // java this.mSnapshot.setBoolBitSet(this.mPointer + 12, FLAG_CARD_END, value);
      self.flags = BoolBitSet::new(FLAG_CARD_END).set(self.flags, value);
      self.sync_flags();
    }

    // java public static void setLineBalloonPageHeight(int pointer, TypoSnapshot snapshot, int value)
    pub fn setLineBalloonPageHeight(pointer: i32, snapshot: &mut TypoSnapshot, value: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.set(pointer + 25, value);
      snapshot.set__int_int(pointer + 25, value);
      **/
    }

    // java public static int getLineBalloonPageHeight(int pointer, TypoSnapshot snapshot)
    pub fn getLineBalloonPageHeight(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.get(pointer + 25);
      return snapshot.get__int(pointer + 25);
      **/
    }

}

impl std::ops::Deref for TypoTextLine {
    type Target = TypoObjBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TypoTextLine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
