use crate::java_runtime::Cell;
use std::sync::LazyLock;
use util::org::apache::poi::util::{BitField::BitField, BitFieldFactory::BitFieldFactory};
use crate::cn::wps::moss::filefmt::biff8::record::WindowOneRecord::WindowOneRecord;
// java private static final BitField showTabs = BitFieldFactory.getInstance(32);
static showTabs:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(32));
// java public class Window1
pub struct Window1 {
    // java short xWn;
    xWn:Cell<i16>,
    // java short yWn;
    yWn:Cell<i16>,
    // java short dxWn;
    dxWn:Cell<i16>,
    // java short dyWn;
    dyWn:Cell<i16>,
    // java short grbit;
    grbit:Cell<i16>,
    // java short wTabRatio;
    wTabRatio:Cell<i16>,
    // java int activeSheet;
    activeSheet:Cell<i32>,
    // java int firstVisibleTab;
    firstVisibleTab:Cell<i32>,
}
#[cfg(test)]
#[path="Window1.review.rs"]
mod review;
impl Window1 {
    // java public Window1() {
    pub fn new()->Self{
        // java this.xWn = 360; this.yWn = 270; this.dxWn = 14940; this.dyWn = 9150;
        // java this.grbit = 56; this.wTabRatio = 600; this.activeSheet = 0; this.firstVisibleTab = 0;
        Self{xWn:Cell::new(360),yWn:Cell::new(270),dxWn:Cell::new(14940),dyWn:Cell::new(9150),grbit:Cell::new(56),wTabRatio:Cell::new(600),activeSheet:Cell::new(0),firstVisibleTab:Cell::new(0)}
    }
    // java public Window1(WindowOneRecord win1) {
    pub fn new__record(win1:Option<&WindowOneRecord>)->Self{
        let win1=win1.expect("NullPointerException");
        // java this.xWn = win1.getHorizontalHold();
        let xWn=Cell::new(win1.getHorizontalHold());
        // java this.yWn = win1.getVerticalHold();
        let yWn=Cell::new(win1.getVerticalHold());
        // java this.dxWn = win1.getWidth();
        let dxWn=Cell::new(win1.getWidth());
        // java this.dyWn = win1.getHeight();
        let dyWn=Cell::new(win1.getHeight());
        // java this.grbit = win1.getOptions();
        let grbit=Cell::new(win1.getOptions());
        // java this.wTabRatio = win1.getTabWidthRatio();
        let wTabRatio=Cell::new(win1.getTabWidthRatio());
        // java this.activeSheet = win1.getActiveSheetIndex();
        let activeSheet=Cell::new(win1.getActiveSheetIndex());
        // java this.firstVisibleTab = win1.getFirstVisibleTab();
        let firstVisibleTab=Cell::new(win1.getFirstVisibleTab());
        Self{xWn,yWn,dxWn,dyWn,grbit,wTabRatio,activeSheet,firstVisibleTab}
    }
    // java public void toRecord(WindowOneRecord win1) {
    pub fn toRecord(&self,win1:Option<&WindowOneRecord>){
        let win1=win1.expect("NullPointerException");
        // java win1.setHorizontalHold(this.xWn);
        win1.setHorizontalHold(self.xWn.get());
        // java win1.setVerticalHold(this.yWn);
        win1.setVerticalHold(self.yWn.get());
        // java win1.setWidth(this.dxWn);
        win1.setWidth(self.dxWn.get());
        // java win1.setHeight(this.dyWn);
        win1.setHeight(self.dyWn.get());
        // java win1.setOptions(this.grbit);
        win1.setOptions(self.grbit.get());
        // java win1.setTabWidthRatio(this.wTabRatio);
        win1.setTabWidthRatio(self.wTabRatio.get());
        // java win1.setActiveSheetIndex(this.activeSheet);
        win1.setActiveSheetIndex(self.activeSheet.get());
        // java win1.setFirstVisibleTab(this.firstVisibleTab);
        win1.setFirstVisibleTab(self.firstVisibleTab.get());
    }
    // java public boolean showTabs() { return showTabs.isSet(this.grbit); }
    pub fn showTabs(&self)->bool{showTabs.isSet(self.grbit.get() as i32)}
    // java public void setShowTabs(boolean s) { this.grbit = showTabs.setShortBoolean(this.grbit, s); }
    pub fn setShowTabs(&self,s:bool){self.grbit.set(showTabs.setShortBoolean(self.grbit.get(),s));}
    // java public void setActiveSheet(int index) { this.activeSheet = index; }
    pub fn setActiveSheet(&self,index:i32){self.activeSheet.set(index);}
    // java public int getActiveSheet() { return this.activeSheet; }
    pub fn getActiveSheet(&self)->i32{self.activeSheet.get()}
    // java public void setFirstVisibleSheet(int index) { this.firstVisibleTab = index; }
    pub fn setFirstVisibleSheet(&self,index:i32){self.firstVisibleTab.set(index);}
    // java public int getFirstVisibleSheet() { return this.firstVisibleTab; }
    pub fn getFirstVisibleSheet(&self)->i32{self.firstVisibleTab.get()}
}
