use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData,KmoAtomDataSub};use crate::cn::wps::moss::{util::KmoRect::KmoRect,filefmt::biff8::util::CellRangeAddress::CellRangeAddress};
// java package cn.wps.moss.core.sheet;
// java
// java import cn.wps.memor4j.KmoAtomData;
// java import cn.wps.moss.filefmt.biff8.util.CellRangeAddress;
// java import cn.wps.moss.util.KmoRect;
// java import java.util.ArrayList;
// java import java.util.List;
// java
// java class KmoSelectionData extends KmoAtomData {
// java public byte pane = 3;
// java public int activeRow;
// java public int activeCol;
// java public KmoRect selection = new KmoRect();
// java public int activeRefIndex;
// java public List<CellRangeAddress> refs = new ArrayList();
// java
// java public KmoAtomData copy() {
// java KmoSelectionData data = new KmoSelectionData();
// java data.pane = this.pane;
// java data.activeRow = this.activeRow;
// java data.activeCol = this.activeCol;
// java data.selection.copy(this.selection);
// java data.activeRefIndex = this.activeRefIndex;
// java data.refs = new ArrayList(this.refs);
// java return data;
// java }
// java }
#[derive(Clone)]pub struct KmoSelectionData{pub base:KmoAtomData,pub pane:i8,pub activeRow:i32,pub activeCol:i32,pub selection:KmoRect,pub activeRefIndex:i32,pub refs:Vec<CellRangeAddress>}
impl KmoSelectionData{pub fn new()->Self{Self{base:KmoAtomData::new(),pane:3,activeRow:0,activeCol:0,selection:KmoRect::KmoRect(),activeRefIndex:0,refs:vec![]}}}
impl KmoAtomDataSub for KmoSelectionData{fn as_base(&self)->&KmoAtomData{&self.base}fn as_base_mut(&mut self)->&mut KmoAtomData{&mut self.base}fn copy(&self)->Self{Self{base:KmoAtomData::new(),pane:self.pane,activeRow:self.activeRow,activeCol:self.activeCol,selection:self.selection.clone(),activeRefIndex:self.activeRefIndex,refs:self.refs.clone()}}}
