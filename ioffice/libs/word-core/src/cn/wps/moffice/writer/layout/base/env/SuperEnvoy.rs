// Java: import cn.wps.collections.KList
// CARGO_OFF_SURFACE use util::collections::KList::KList;
// Java: import cn.wps.moffice.define.FontName
use util::moffice::define::FontName::FontName;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.cache.TypoPage
use crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage;
// Java: import cn.wps.moffice.writer.cache.TypoRectCoreImpl
use crate::cn::wps::moffice::writer::cache::TypoRectCoreImpl::TypoRectCoreImpl;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::CellInfo::CellInfo;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::RowInfo::RowInfo;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::table::TableInfo::TableInfo;
// Java: import cn.wps.moffice.writer.layout.base.ParaLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::ParaLayouter::ParaLayouter;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::DrawingLayouter;
// Java: import cn.wps.moffice.writer.layout.base.drawing.TextFrameLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::drawing::TextFrameLayouter::TextFrameLayouter;
// Java: import cn.wps.moffice.writer.layout.base.graph.SrcNode
use crate::cn::wps::moffice::writer::layout::base::graph::SrcNode::SrcNode;
// Java: import cn.wps.moffice.writer.layout.base.page.PageLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::page::PageLayouter::PageLayouter;
// Java: import cn.wps.moffice.writer.layout.base.table.TableLayouter
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::table::TableLayouter::TableLayouter;
// Java: import cn.wps.moffice.writer.layout.webview.WebViewLayoutState
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::webview::WebViewLayoutState::WebViewLayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos;
use crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::CellInfoNode;
use crate::cn::wps::moffice::writer::layout::base::env::TableCellStateEnvoy::TableCellStateEnvoy;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaEnv
use crate::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.PageEnv
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.TableCellLayoutState
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import java.lang.Object
// CARGO_OFF_SURFACE use stubs::java::lang::Object::Object;
// Java: import cn.wps.moffice.writer.layout.base.env.StateSnapshot
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::StateSnapshot::StateSnapshot;
// Java: import cn.wps.moffice.writer.core.Range
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::core::Range::Range;
// Java: import cn.wps.moffice.writer.layout.base.env.SignableRange
use core::marker::PhantomData;

// CARGO_OFF_SURFACE_TYPES
#[derive(Default)]
pub struct CellInfo;
#[derive(Default)]
pub struct RowInfo;
#[derive(Default)]
pub struct TableInfo;
#[derive(Default)]
pub struct DrawingLayouter;
#[derive(Default)]
pub struct TextFrameLayouter;
#[derive(Default)]
pub struct TableLayouter;
#[derive(Default)]
pub struct WebViewLayoutState;
#[derive(Default)]
pub struct TableCellLayoutState;

/// Java `StateSnapshot` keys written by `SuperEnvoy.enumToSave` / `TableCellStateEnvoy`.
/// Key 0 is the start CP. Key 4 is the layout range. Keys 11–15 are cell flags.
/// `first_merge` is Java key 10 (`setFirstMergeCell`).
#[derive(Clone)]
pub struct StateSnapshot {
    present: u32,
    values: [i32; 16],
    range: i64,
    pub cells: Vec<StateSnapshot>,
    pub first_merge: Option<Box<StateSnapshot>>,
}

impl Default for StateSnapshot {
    fn default() -> Self {
        Self {
            present: 0,
            values: [0; 16],
            range: 0,
            cells: Vec::new(),
            first_merge: None,
        }
    }
}

impl StateSnapshot {
    // java public void setStartCp(int cp)
    pub fn setStartCp(&mut self, cp: i32) {
        self.putInt(0, cp);
    }

    // java public int getStartCp()
    pub fn getStartCp(&self) -> i32 {
        self.getInt(0).unwrap_or(-1)
    }

    // java public void putInt(int key, int value)
    pub fn putInt(&mut self, key: i32, value: i32) {
        if (0..16).contains(&key) {
            self.values[key as usize] = value;
            self.present |= 1 << key;
        }
    }

    // java public void putBool(int key, boolean value)
    pub fn putBool(&mut self, key: i32, value: bool) {
        self.putInt(key, if value { 1 } else { 0 });
    }

    pub fn getInt(&self, key: i32) -> Option<i32> {
        if (0..16).contains(&key) && self.present & (1 << key) != 0 {
            Some(self.values[key as usize])
        } else {
            None
        }
    }

    // java stateSnapshot.putObject(4, range)
    pub fn putRange(&mut self, range: i64) {
        self.range = range;
        self.present |= 1 << 4;
    }

    pub fn getRange(&self) -> Option<i64> {
        if self.present & (1 << 4) != 0 { Some(self.range) } else { None }
    }

    // java public KList<StateSnapshot> getCellStates()
    pub fn getCellStates(&mut self) -> &mut Vec<StateSnapshot> {
        &mut self.cells
    }

    // java public void setFirstMergeCell(StateSnapshot firstCellState)
    pub fn setFirstMergeCell(&mut self, first: StateSnapshot) {
        self.first_merge = Some(Box::new(first));
    }

    pub fn getFirstMergeCell(&self) -> Option<&StateSnapshot> {
        self.first_merge.as_deref()
    }
}
#[derive(Default)]
pub struct Range;
#[derive(Default)]
pub struct SignableRange;
#[derive(Default)]
pub struct KList;
#[derive(Default)]
pub struct Object;

// java public class SuperEnvoy implements LayoutState.HostEnvoy<T>, ParaLayouter.IStateEnvoy, PageLayouter.IStateEnvoy, TableLayouter.IStateEnvoy, DrawingLayouter.IStateEnvoy, TextFrameLayouter.IStateEnvoy
#[derive(Default)]
pub struct SuperEnvoy {
    // java protected T mLayoutState;
    pub(crate) mLayoutState: *mut (),
    // java protected SuperEnvoy<T>.StateEnumProcedure mRestoreProcesure = new StateEnumProcedure();
    pub(crate) mRestoreProcesure: StateEnumProcedure,
    // Slot for DrawingLayouter.IStateEnvoy.getAffectRectMut. setAffectRect writes LayoutState.
    drawing_affect: crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::TypoRectCoreImpl,
}

// java protected class StateEnumProcedure implements StateSnapshot.StateEnumProcedure
#[derive(Default)]
pub struct StateEnumProcedure {
    // java DocumentEnv docEnv;
    pub(crate) docEnv: Option<DocumentEnv>,
}

// java public class SuperEnvoy implements LayoutState.HostEnvoy<T>, ParaLayouter.IStateEnvoy, PageLayouter.IStateEnvoy, TableLayouter.IStateEnvoy, DrawingLayouter.IStateEnvoy, TextFrameLayouter.IStateEnvoy
impl SuperEnvoy {
    // java public T getLayoutState()
    pub fn getLayoutState(&self) -> *mut () {
      // java return this.mLayoutState;
      return self.mLayoutState;
    }

    // java public void setLayoutState(T layoutState)
    pub fn setLayoutState(&mut self, layoutState: *mut ()) {
      // java this.mLayoutState = layoutState;
      self.mLayoutState = layoutState;
    }

    fn ls(&self) -> *mut LayoutState {
      self.mLayoutState as *mut LayoutState
    }

    // java public final Document getDocument()
    pub fn getDocument(&self) -> &dyn Document {
      // java return this.mLayoutState.document;
      unsafe {
        let st = &*self.ls();
        &*st.document.expect("LayoutState.document")
      }
    }

    // java public final ParaEnv getParaEnv()
    pub fn getParaEnv(&mut self) -> &mut ParaEnv {
      // java return this.mLayoutState.paraEnv;
      unsafe { &mut (*self.ls()).paraEnv }
    }

    // java public final void addAutoNumString(String str, PropertySet finalRunProp, ViewEnv viewEnv)
    pub fn addAutoNumString(&mut self, str: &str, finalRunProp: &PropertySet, viewEnv: &ViewEnv) {
      // java SrcNode srcNode = (SrcNode)this.mLayoutState.srcList.obtainAndAddFirst();
      let layout_state = unsafe { &mut *self.ls() };
      let srcNode = unsafe { &mut *layout_state.srcList.obtainAndAddFirst() };
      // java srcNode.type = 1;
      srcNode.r#type = 1;
      // java srcNode.length = str.length();
      srcNode.length = str.encode_utf16().count() as i32;
      // java srcNode.data = str;
      srcNode.data = Some(str.to_string());
      let mut run_prop = finalRunProp.clone();
      // java if (viewEnv.getForceUseShareViewRunProperty())
      if viewEnv.getForceUseShareViewRunProperty() {
        // java PropertySet shareViewPropert = viewEnv.getShareViewProperty();
        let shareViewPropert = viewEnv.getShareViewProperty();
        // java if (null != shareViewPropert)
        if let Some(shareViewPropert) = shareViewPropert {
          // java MutablePropertySet mutableProp = new MutablePropertySet(finalRunProp);
          let mut mutableProp = MutablePropertySet::new_from_ps(finalRunProp);
          // java String oldFont = finalRunProp.getString(3, "Times New Roman");
          let oldFont = finalRunProp.getString__int_string(3, "Times New Roman");
          // java mutableProp.add(shareViewPropert);
          mutableProp.add(&shareViewPropert);
          // java if (2 == this.mLayoutState.curPage.getType())
          if layout_state.curPage.as_ref().is_some_and(|page| page.documentType == 2) {
            // java float fontSize = viewEnv.getShareViewFontsize();
            let fontSize = viewEnv.getShareViewFontsize() as f32;
            // java mutableProp.setFloat(10, fontSize);
            mutableProp.setFloat(10, fontSize);
            // java mutableProp.setFloat(37, fontSize);
            mutableProp.setFloat(37, fontSize);
          }
          // java if (FontName.isSpecialFontName(oldFont))
          if FontName::isSpecialFontName(&oldFont) {
            // java mutableProp.setString(3, oldFont);
            mutableProp.setString(3, oldFont);
          }
          // java finalRunProp = mutableProp.freeze();
          run_prop = mutableProp.freeze();
        }
      }
      // java srcNode.finalRunProp = finalRunProp;
      srcNode.finalRunProp = Some(run_prop);
    }

    // java public final int getCP()
    pub fn getCP(&self) -> i32 {
      // java return this.mLayoutState.cp;
      unsafe { (*self.ls()).cp }
    }

    // java public void setCP(int cp)
    pub fn setCP(&mut self, cp: i32) {
      // java this.mLayoutState.cp = cp;
      unsafe { (*self.ls()).cp = cp; }
    }

    // java public void incrementCP()
    pub fn incrementCP(&mut self) {
      // java ++this.mLayoutState.cp;
      unsafe { (*self.ls()).cp += 1; }
    }

    // java public final int getCpExResult()
    pub fn getCpExResult(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java public final int getRangeStart()
    pub fn getRangeStart(&self) -> i32 {
      // java return this.mLayoutState.getRangeStart();
      unsafe { (*self.ls()).getRangeStart() }
    }

    // java public final int getRangeEnd()
    pub fn getRangeEnd(&self) -> i32 {
      // java return this.mLayoutState.getRangeEnd();
      unsafe { (*self.ls()).getRangeEnd() }
    }

    // java public final PageEnv getPageEnv()
    pub fn getPageEnv(&self) -> &PageEnv {
      // java return this.mLayoutState.pageEnv;
      unsafe { &(*self.ls()).pageEnv }
    }

    // java public final TypoPage getCurPage()
    pub fn getCurPage(&self) -> &TypoPage {
      // java return this.mLayoutState.curPage;
      unsafe { (*self.ls()).curPage.as_ref().expect("curPage") }
    }

    // java public final void skipSpaceBeforePara(int firstNormalCharCP)
    pub fn skipSpaceBeforePara(&mut self, firstNormalCharCP: i32) {
      let _ = firstNormalCharCP;
      // java this.mLayoutState.skipSpaceBeforePara(firstNormalCharCP);
    }

    // java public int getCurLayoutHeight()
    pub fn getCurLayoutHeight(&self) -> i32 {
      // java return this.mLayoutState.curLayoutHeight;
      unsafe { (*self.ls()).curLayoutHeight }
    }

    // java public final void setCurLayoutHeight(int height)
    pub fn setCurLayoutHeight(&mut self, height: i32) {
      // java this.mLayoutState.curLayoutHeight = height;
      unsafe { (*self.ls()).curLayoutHeight = height; }
    }

    // java public boolean isValidLine()
    pub fn isValidLine(&self) -> bool {
      // java return this.mLayoutState.validLine;
      unsafe { (*self.ls()).validLine }
    }

    // java public void setValidLine(boolean b)
    pub fn setValidLine(&mut self, b: bool) {
      // java this.mLayoutState.validLine = b;
      unsafe { (*self.ls()).validLine = b; }
    }

    // java public void setFinishPage(boolean b)
    pub fn setFinishPage(&mut self, b: bool) {
      // java this.mLayoutState.setFinishPage(b);
      unsafe { (*self.ls()).setFinishPage(b); }
    }

    // java public KList<TableCellLayoutState> getCellStateList()
    pub fn getCellStateList(&mut self) -> &mut crate::cn::wps::moffice::writer::layout::base::env::LayoutState::CellStateList {
      // java return this.mLayoutState.cellStateList;
      unsafe { &mut (*self.ls()).cellStateList }
    }

    // java public boolean isAtLeastOnLine()
    pub fn isAtLeastOnLine(&self) -> bool {
      // java return this.mLayoutState.atLeastOneLine;
      unsafe { (*self.ls()).atLeastOneLine }
    }

    // java public int getTablePeriLayer()
    pub fn getTablePeriLayer(&self) -> i32 {
      // java return this.mLayoutState.tablePeriLayer;
      unsafe { (*self.ls()).tablePeriLayer }
    }

    // java public void setHasFlowTableHeader(boolean b)
    pub fn setHasFlowTableHeader(&mut self, b: bool) {
      // java this.mLayoutState.hasFlowTableHeader = b;
      unsafe { (*self.ls()).hasFlowTableHeader = b; }
    }

    // java public void setTableHaderRange(long range)
    pub fn setTableHaderRange(&mut self, range: i64) {
      // java this.mLayoutState.tableHeaderRange = range;
      unsafe { (*self.ls()).tableHeaderRange = range; }
    }

    // java public int getMaxWrapDrawingCP()
    pub fn getMaxWrapDrawingCP(&self) -> i32 {
      // java return this.mLayoutState.maxWrapDrawingCP;
      unsafe { (*self.ls()).maxWrapDrawingCP }
    }

    // java public void setMaxWrapDrawingCP(int cp)
    pub fn setMaxWrapDrawingCP(&mut self, cp: i32) {
      // java this.mLayoutState.maxWrapDrawingCP = cp;
      unsafe { (*self.ls()).maxWrapDrawingCP = cp; }
    }

    // java public int getLinePos()
    pub fn getLinePos(&self) -> i32 {
      // java return this.mLayoutState.linePos;
      unsafe { (*self.ls()).linePos }
    }

    // java public void setLinePos(int newPos)
    pub fn setLinePos(&mut self, newPos: i32) {
      // java this.mLayoutState.linePos = newPos;
      unsafe { (*self.ls()).linePos = newPos; }
    }

    // java public void updateLinesOfPage(int newPos)
    pub fn updateLinesOfPage(&mut self, newPos: i32) {
      // java this.mLayoutState.linePos = newPos;
      unsafe { (*self.ls()).linePos = newPos; }
    }

    // java protected boolean enumIntToRestore(int key, int value, DocumentEnv docEnv)
    pub fn enumIntToRestore(&mut self, key: i32, value: i32, docEnv: &mut DocumentEnv) -> bool {
      // java switch (key)
      let ls = self.ls();
      match key {
        // java case 0:
        0 => {
          // java this.mLayoutState.cp = value;
          unsafe { (*ls).cp = value; }
        }
        // java case 1:
        1 => {
          // java this.mLayoutState.tablePeriLayer = value;
          unsafe { (*ls).tablePeriLayer = value; }
        }
        // java case 2:
        2 => {
          // java this.mLayoutState.textFrameLayer = value;
          unsafe { (*ls).textFrameLayer = value; }
        }
        // java case 3:
        3 => {
          // java this.mLayoutState.document = docEnv.textDocument.getSubdocument(value);
          if let Some(sub) = docEnv.textDocument.as_ref().and_then(|td| td.getSubdocument(value)) {
            let p: *const dyn Document = sub;
            unsafe { (*ls).document = Some(p as *mut dyn Document); }
          }
        }
        // java case 4: range is restored as an object, not an int
        4 => {}
        // java case 5:
        5 => {
          // java this.mLayoutState.pageEnv.allowOverCoreHeight = value == 1;
          unsafe { (*ls).pageEnv.allowOverCoreHeight = value == 1; }
        }
        // java case 6:
        6 => {
          // java this.mLayoutState.pageEnv.gridHeight = value;
          unsafe { (*ls).pageEnv.gridHeight = value; }
        }
        // java case 7:
        7 => {
          // java this.mLayoutState.pageEnv.gridWidth = value;
          unsafe { (*ls).pageEnv.gridWidth = value; }
        }
        // java case 8:
        8 => {
          // java this.mLayoutState.pageEnv.gridModel = value;
          unsafe { (*ls).pageEnv.gridModel = value; }
        }
        // java case 9:
        9 => {
          // java this.mLayoutState.pageEnv.charSpaceForGrid = Float.intBitsToFloat(value);
          unsafe { (*ls).pageEnv.charSpaceForGrid = f32::from_bits(value as u32); }
        }
        _ => {}
      }
      // java return true;
      true
    }

    // java protected boolean enumObjectToRestore(int key, Object obj, DocumentEnv docEnv)
    pub fn enumObjectToRestore(&mut self, key: i32, obj: Option<&dyn core::any::Any>, _docEnv: &mut DocumentEnv) -> bool {
      // java if (key == 4) this.mLayoutState.setRange(((StateSnapshot.Range)obj).range);
      if key == 4 {
        if let Some(range) = obj.and_then(|o| o.downcast_ref::<i64>()) {
          unsafe { (*self.ls()).setRange__long(*range); }
        }
      }
      // java return true;
      true
    }

    // java protected void enumToSave(StateSnapshot stateSnapshot, DocumentEnv docEnv)
    pub fn enumToSave(&self, stateSnapshot: &mut StateSnapshot, _docEnv: &DocumentEnv) {
      let ls = unsafe { &*self.ls() };
      // java stateSnapshot.setStartCp(this.mLayoutState.cp);
      stateSnapshot.setStartCp(ls.cp);
      // java stateSnapshot.putInt(1, this.mLayoutState.tablePeriLayer);
      stateSnapshot.putInt(1, ls.tablePeriLayer);
      // java stateSnapshot.putInt(2, this.mLayoutState.textFrameLayer);
      stateSnapshot.putInt(2, ls.textFrameLayer);
      // java stateSnapshot.putInt(3, this.mLayoutState.document.getType());
      let doc_type = ls.document.filter(|p| !p.is_null()).map(|p| unsafe { (*p).getType__on_IDocument() }).unwrap_or(0);
      stateSnapshot.putInt(3, doc_type);
      // java stateSnapshot.putObject(4, stateSnapshot.reuseRangeInstance(this.mLayoutState.getRange()));
      stateSnapshot.putRange(ls.getRange());
      // java stateSnapshot.putBool(5, this.mLayoutState.pageEnv.allowOverCoreHeight);
      stateSnapshot.putBool(5, ls.pageEnv.allowOverCoreHeight);
      // java stateSnapshot.putInt(6, this.mLayoutState.pageEnv.gridHeight);
      stateSnapshot.putInt(6, ls.pageEnv.gridHeight);
      // java stateSnapshot.putInt(7, this.mLayoutState.pageEnv.gridWidth);
      stateSnapshot.putInt(7, ls.pageEnv.gridWidth);
      // java stateSnapshot.putInt(8, this.mLayoutState.pageEnv.gridModel);
      stateSnapshot.putInt(8, ls.pageEnv.gridModel);
      // java stateSnapshot.putInt(9, Float.floatToIntBits(this.mLayoutState.pageEnv.charSpaceForGrid));
      stateSnapshot.putInt(9, ls.pageEnv.charSpaceForGrid.to_bits() as i32);
    }

    // java public void saveToSnapShot(StateSnapshot stateSnapshot, DocumentEnv docEnv)
    pub fn saveToSnapShot(&mut self, stateSnapshot: &mut StateSnapshot, docEnv: &mut DocumentEnv) {
      // java this.enumToSave(stateSnapshot, docEnv);
      self.enumToSave(stateSnapshot, docEnv);
      // java docEnv.pools.recycleStatesSnapshot(stateSnapshot.getCellStates());
      stateSnapshot.cells.clear();
      // java while (null != (cellEnv = it.next2())) cellEnv.getEnvoy().saveToSnapShot(...)
      let n = unsafe { (*self.ls()).cellStateList.len() };
      for i in 0..n {
        let cell = unsafe { (*self.ls()).cellStateList.ptr_at(i) };
        if cell.is_null() {
          continue;
        }
        let mut dest = StateSnapshot::default();
        TableCellStateEnvoy::bind(cell).saveToSnapShot(&mut dest, docEnv);
        stateSnapshot.cells.push(dest);
      }
    }

    // java protected void forEachValueRestore(StateSnapshot stateSnapshot, DocumentEnv docEnv)
    pub fn forEachValueRestore(&mut self, stateSnapshot: &mut StateSnapshot, docEnv: &mut DocumentEnv) {
      // java stateSnapshot.forEachValue(this.mRestoreProcesure);
      let present = stateSnapshot.present;
      let values = stateSnapshot.values;
      let range = stateSnapshot.range;
      for key in 0..16 {
        if key == 4 {
          continue;
        }
        if present & (1 << key) != 0 {
          self.enumIntToRestore(key, values[key as usize], docEnv);
        }
      }
      if present & (1 << 4) != 0 {
        // java case 4: this.mLayoutState.setRange(range);
        unsafe { (*self.ls()).setRange__long(range); }
      }
    }

    // java public void restoreFromSnapShot(StateSnapshot stateSnapshot, DocumentEnv docEnv)
    pub fn restoreFromSnapShot(&mut self, stateSnapshot: &mut StateSnapshot, docEnv: &mut DocumentEnv) {
      // java this.forEachValueRestore(stateSnapshot, docEnv);
      self.forEachValueRestore(stateSnapshot, docEnv);
      // java docEnv.pools.recycleTableCellLayoutStates(this.mLayoutState.cellStateList);
      let cell_snaps = stateSnapshot.cells.clone();
      unsafe { (*self.ls()).cellStateList.clear(); }
      // java if (!stateSnapshot.getCellStates().isEmpty()) rebuild when row.size() matches
      if cell_snaps.is_empty() {
        return;
      }
      let (cp, layer, doc) = unsafe {
        let ls = &*self.ls();
        (ls.cp, ls.tablePeriLayer, ls.document)
      };
      let Some(prepared) = Self::cells_for_row(doc, cp, layer, cell_snaps.len()) else {
        return;
      };
      for (mut snap, info) in cell_snaps.into_iter().zip(prepared) {
        // java TableCellLayoutState desCell = docEnv.pools.obtainTableCellLayoutState();
        let mut des_cell = docEnv.pools.obtainTableCellLayoutState();
        // java desCell.setCellnfo(cellInfo);
        if let Some(info) = info {
          des_cell.setCellnfo(info);
        }
        // java desCell.getEnvoy().restoreFromSnapShot(cellSnapShot, docEnv);
        TableCellStateEnvoy::bind(&mut des_cell).restoreFromSnapShot(&mut snap, docEnv);
        // java this.mLayoutState.cellStateList.add(desCell);
        unsafe { (*self.ls()).cellStateList.add(des_cell); }
      }
    }

    fn cells_for_row(doc: Option<*mut dyn Document>, cp: i32, layer: i32, snap_len: usize) -> Option<Vec<Option<CellInfoNode>>> {
      let doc = doc.filter(|p| !p.is_null())?;
      let infos = unsafe { (*doc).getTableInfos() }?;
      if infos.is_null() {
        return None;
      }
      // java TableInfo tableInfo = document.getTableInfos().getTableInfoByLevel(cp, tablePeriLayer);
      let mut table = unsafe { (&*infos).getTableInfoByLevel(cp, layer) }?;
      // java RowInfo row = tableInfo.getRowByCP(cp);
      let row = table.getRowByCP(cp)?;
      // java if (stateSnapshot.getCellStates().size() == row.size())
      if row.size() != snap_len as i32 {
        return None;
      }
      Some((0..row.size()).map(|index| row.getCellByIndex(index).cloned()).collect())
    }

    // java public TypoRectCoreImpl getAffectRect()
    pub fn getAffectRect(&mut self) -> &mut TypoRectCoreImpl {
      // java return this.mLayoutState.affectRect;
      unsafe { &mut (*self.ls()).affectRect }
    }

    // java public int getTextFrameLayer()
    pub fn getTextFrameLayer(&self) -> i32 {
      // java return this.mLayoutState.textFrameLayer;
      unsafe { (*self.ls()).textFrameLayer }
    }

    // java public SignableRange getSignableRange()
    pub fn getSignableRange(&self) -> Option<&SignableRange> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mLayoutState.mSignableRangeToReflow;
      return Some(&self.mLayoutState.mSignableRangeToReflow);
      */
    }

    // java public boolean isLayoutOnePara()
    pub fn isLayoutOnePara(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mLayoutState instanceof WebViewLayoutState && ((WebViewLayoutState)this.mLayoutState).layoutOnePara;
      return self.mLayoutState.downcast_ref::<WebViewLayoutState>().is_some() && (self.mLayoutState).downcast_ref::<WebViewLayoutState>().unwrap().layoutOnePara;
      */
    }

}

// java protected class StateEnumProcedure implements StateSnapshot.StateEnumProcedure
impl StateEnumProcedure {
    // java public boolean enumInt(int key, int value)
    pub fn enumInt(&mut self, key: i32, value: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return SuperEnvoy.this.enumIntToRestore(key, value, this.docEnv);
      return self.enumIntToRestore(key, value, self.docEnv);
      */
    }

}

impl crate::cn::wps::moffice::writer::layout::base::LayouterBase::IStateEnvoy for SuperEnvoy {}

impl crate::cn::wps::moffice::writer::layout::base::ParaLayouter::IStateEnvoy for SuperEnvoy {
    fn getDocument(&self) -> &dyn Document { SuperEnvoy::getDocument(self) }
    fn getParaEnv(&mut self) -> &mut ParaEnv { SuperEnvoy::getParaEnv(self) }
    fn addAutoNumString(&mut self, var1: &str, var2: &PropertySet, var3: &ViewEnv) { SuperEnvoy::addAutoNumString(self, var1, var2, var3) }
    fn getCP(&self) -> i32 { SuperEnvoy::getCP(self) }
    fn getCpExResult(&self) -> i32 { SuperEnvoy::getCpExResult(self) }
    fn getRangeStart(&self) -> i32 { SuperEnvoy::getRangeStart(self) }
    fn getRangeEnd(&self) -> i32 { SuperEnvoy::getRangeEnd(self) }
    fn getPageEnv(&self) -> &PageEnv { SuperEnvoy::getPageEnv(self) }
    fn getCurPage(&self) -> &TypoPage { SuperEnvoy::getCurPage(self) }
    fn skipSpaceBeforePara(&mut self, var1: i32) { SuperEnvoy::skipSpaceBeforePara(self, var1) }
    fn getLinePos(&self) -> i32 { SuperEnvoy::getLinePos(self) }
}

impl crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::IStateEnvoy for SuperEnvoy {
    fn setMaxWrapDrawingCP(&mut self, var1: i32) {
        SuperEnvoy::setMaxWrapDrawingCP(self, var1);
    }
    fn getMaxWrapDrawingCP(&self) -> i32 {
        unsafe { (*self.ls()).maxWrapDrawingCP }
    }
    fn getCurPage(&self) -> crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::TypoPage {
        let page = SuperEnvoy::getCurPage(self);
        let env = SuperEnvoy::getPageEnv(self);
        crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::TypoPage {
            type_: page.getDocumentType(),
            pointer: page.getPointer(),
            marginLeft: page.getMarginLeft(),
            marginTop: page.getMarginTop(),
            width: env.width,
            height: env.height,
            coreWidth: env.getCoreWidth(),
            coreHeight: env.getCoreHeight(),
            visualTextflow: page.getVisualTextflow(),
            lines: page.getLines(),
            parent: 0,
            documentType: page.getDocumentType(),
        }
    }
    fn getAffectRect(&self) -> crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::TypoRectCoreImpl {
        unsafe {
            let rect = &(*self.ls()).affectRect.super_;
            crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::TypoRectCoreImpl {
                left: rect.left,
                top: rect.top,
                right: rect.right,
                bottom: rect.bottom,
            }
        }
    }
    fn getAffectRectMut(&mut self) -> &mut crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::TypoRectCoreImpl {
        let copied = <Self as crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::IStateEnvoy>::getAffectRect(self);
        self.drawing_affect = copied;
        &mut self.drawing_affect
    }
    fn setAffectRect(&mut self, rect: crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::TypoRectCoreImpl) {
        self.drawing_affect = rect.clone();
        unsafe {
            let target = &mut (*self.ls()).affectRect.super_;
            target.left = rect.left;
            target.top = rect.top;
            target.right = rect.right;
            target.bottom = rect.bottom;
        }
    }
    fn getPageEnv(&self) -> crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::PageEnv {
        let env = SuperEnvoy::getPageEnv(self);
        crate::cn::wps::moffice::writer::layout::base::drawing::DrawingLayouter::PageEnv {
            marginLeft: env.marginLeft,
            marginTop: env.marginTop,
            coreWidth: env.getCoreWidth(),
            height: env.height,
        }
    }
    fn getTablePeriLayer(&self) -> i32 {
        unsafe { (*self.ls()).tablePeriLayer }
    }
    fn getTextFrameLayer(&self) -> i32 {
        unsafe { (*self.ls()).textFrameLayer }
    }
    fn getCurLayoutHeight(&self) -> i32 {
        SuperEnvoy::getCurLayoutHeight(self)
    }
    fn getRangeStart(&self) -> i32 {
        unsafe { (*self.ls()).getRangeStart() }
    }
    fn getRangeEnd(&self) -> i32 {
        unsafe { (*self.ls()).getRangeEnd() }
    }
    fn setFinishPage(&mut self, finish: bool) {
        unsafe { (*self.ls()).setFinishPage(finish); }
    }
}
