use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData,KmoAtomDataSub};
// java package cn.wps.moss.core.sheet;
// java
// java import cn.wps.memor4j.KmoAtomData;
// java
// java class PaneInfoData extends KmoAtomData {
// java public int x = 0;
// java public int y = 0;
// java public int topRow = 0;
// java public int leftCol = 0;
// java public int activePane = 0;
// java
// java public KmoAtomData copy() {
// java PaneInfoData data = new PaneInfoData();
// java data.x = this.x;
// java data.y = this.y;
// java data.topRow = this.topRow;
// java data.leftCol = this.leftCol;
// java data.activePane = this.activePane;
// java return data;
// java }
// java }
#[derive(Clone)]pub struct PaneInfoData{pub base:KmoAtomData,pub x:i32,pub y:i32,pub topRow:i32,pub leftCol:i32,pub activePane:i32}
impl PaneInfoData{pub fn new()->Self{Self{base:KmoAtomData::new(),x:0,y:0,topRow:0,leftCol:0,activePane:0}}}
impl KmoAtomDataSub for PaneInfoData{fn as_base(&self)->&KmoAtomData{&self.base}fn as_base_mut(&mut self)->&mut KmoAtomData{&mut self.base}fn copy(&self)->Self{Self{base:KmoAtomData::new(),x:self.x,y:self.y,topRow:self.topRow,leftCol:self.leftCol,activePane:self.activePane}}}
