use crate::java_runtime::Cell;
use std::sync::LazyLock;
use util::org::apache::poi::util::{BitField::BitField, BitFieldFactory::BitFieldFactory, LittleEndianOutput::LittleEndianOutput};
use super::StandardRecord::StandardRecord;

// java public static final short sid = 61;
pub const sid:i16=61;
// java private static final BitField hidden = BitFieldFactory.getInstance(1);
static hidden:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(1));
// java private static final BitField iconic = BitFieldFactory.getInstance(2);
static iconic:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(2));
// java private static final BitField hscroll = BitFieldFactory.getInstance(8);
static hscroll:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(8));
// java private static final BitField vscroll = BitFieldFactory.getInstance(16);
static vscroll:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(16));
// java private static final BitField tabs = BitFieldFactory.getInstance(32);
static tabs:LazyLock<BitField>=LazyLock::new(||BitFieldFactory::getInstance(32));
// java public final class WindowOneRecord extends StandardRecord
pub struct WindowOneRecord {
    // java private short field_1_h_hold;
    field_1_h_hold:Cell<i16>,
    // java private short field_2_v_hold;
    field_2_v_hold:Cell<i16>,
    // java private short field_3_width;
    field_3_width:Cell<i16>,
    // java private short field_4_height;
    field_4_height:Cell<i16>,
    // java private short field_5_options;
    field_5_options:Cell<i16>,
    // java private int field_6_active_sheet;
    field_6_active_sheet:Cell<i32>,
    // java private int field_7_first_visible_tab;
    field_7_first_visible_tab:Cell<i32>,
    // java private short field_8_num_selected_tabs;
    field_8_num_selected_tabs:Cell<i16>,
    // java private short field_9_tab_width_ratio;
    field_9_tab_width_ratio:Cell<i16>,
}
impl WindowOneRecord {
    // java public WindowOneRecord() {
    pub fn new()->Self {
        // rust: Java zero-initialized instance fields, before constructor statements.
        let this=Self{field_1_h_hold:Cell::new(0),field_2_v_hold:Cell::new(0),field_3_width:Cell::new(0),field_4_height:Cell::new(0),field_5_options:Cell::new(0),field_6_active_sheet:Cell::new(0),field_7_first_visible_tab:Cell::new(0),field_8_num_selected_tabs:Cell::new(0),field_9_tab_width_ratio:Cell::new(0)};
        // java this.setDisplayHorizonalScrollbar(true);
        this.setDisplayHorizonalScrollbar(true);
        // java this.setDisplayVerticalScrollbar(true);
        this.setDisplayVerticalScrollbar(true);
        this
    }
    // rust: The two original RecordInputStream constructors await the real BIFF
    // reader/continue-record graph; no substitute reader or empty overload is exposed.
    // java public void setHorizontalHold(short h) { this.field_1_h_hold = h; }
    pub fn setHorizontalHold(&self,h:i16){self.field_1_h_hold.set(h);}
    // java public void setVerticalHold(short v) { this.field_2_v_hold = v; }
    pub fn setVerticalHold(&self,v:i16){self.field_2_v_hold.set(v);}
    // java public void setWidth(short w) { this.field_3_width = w; }
    pub fn setWidth(&self,w:i16){self.field_3_width.set(w);}
    // java public void setHeight(short h) { this.field_4_height = h; }
    pub fn setHeight(&self,h:i16){self.field_4_height.set(h);}
    // java public void setOptions(short o) { this.field_5_options = o; }
    pub fn setOptions(&self,o:i16){self.field_5_options.set(o);}
    // java public void setHidden(boolean ishidden) { this.field_5_options = hidden.setShortBoolean(this.field_5_options, ishidden); }
    pub fn setHidden(&self,ishidden:bool){self.field_5_options.set(hidden.setShortBoolean(self.field_5_options.get(),ishidden));}
    // java public void setIconic(boolean isiconic) { this.field_5_options = iconic.setShortBoolean(this.field_5_options, isiconic); }
    pub fn setIconic(&self,isiconic:bool){self.field_5_options.set(iconic.setShortBoolean(self.field_5_options.get(),isiconic));}
    // java public void setDisplayHorizonalScrollbar(boolean scroll) { this.field_5_options = hscroll.setShortBoolean(this.field_5_options, scroll); }
    pub fn setDisplayHorizonalScrollbar(&self,scroll:bool){self.field_5_options.set(hscroll.setShortBoolean(self.field_5_options.get(),scroll));}
    // java public void setDisplayVerticalScrollbar(boolean scroll) { this.field_5_options = vscroll.setShortBoolean(this.field_5_options, scroll); }
    pub fn setDisplayVerticalScrollbar(&self,scroll:bool){self.field_5_options.set(vscroll.setShortBoolean(self.field_5_options.get(),scroll));}
    // java public void setDisplayTabs(boolean disptabs) { this.field_5_options = tabs.setShortBoolean(this.field_5_options, disptabs); }
    pub fn setDisplayTabs(&self,disptabs:bool){self.field_5_options.set(tabs.setShortBoolean(self.field_5_options.get(),disptabs));}
    // java public void setActiveSheetIndex(int index) { this.field_6_active_sheet = index; }
    pub fn setActiveSheetIndex(&self,index:i32){self.field_6_active_sheet.set(index);}
    // java public void setSelectedTab(short s) { this.setActiveSheetIndex(s); }
    pub fn setSelectedTab(&self,s:i16){self.setActiveSheetIndex(s as i32);}
    // java public void setFirstVisibleTab(int t) { this.field_7_first_visible_tab = t; }
    pub fn setFirstVisibleTab(&self,t:i32){self.field_7_first_visible_tab.set(t);}
    // java public void setDisplayedTab(short t) { this.setFirstVisibleTab(t); }
    pub fn setDisplayedTab(&self,t:i16){self.setFirstVisibleTab(t as i32);}
    // java public void setNumSelectedTabs(short n) { this.field_8_num_selected_tabs = n; }
    pub fn setNumSelectedTabs(&self,n:i16){self.field_8_num_selected_tabs.set(n);}
    // java public void setTabWidthRatio(short r) { this.field_9_tab_width_ratio = r; }
    pub fn setTabWidthRatio(&self,r:i16){self.field_9_tab_width_ratio.set(r);}
    // java public short getHorizontalHold() { return this.field_1_h_hold; }
    pub fn getHorizontalHold(&self)->i16{self.field_1_h_hold.get()}
    // java public short getVerticalHold() { return this.field_2_v_hold; }
    pub fn getVerticalHold(&self)->i16{self.field_2_v_hold.get()}
    // java public short getWidth() { return this.field_3_width; }
    pub fn getWidth(&self)->i16{self.field_3_width.get()}
    // java public short getHeight() { return this.field_4_height; }
    pub fn getHeight(&self)->i16{self.field_4_height.get()}
    // java public short getOptions() { return this.field_5_options; }
    pub fn getOptions(&self)->i16{self.field_5_options.get()}
    // java public boolean getHidden() { return hidden.isSet(this.field_5_options); }
    pub fn getHidden(&self)->bool{hidden.isSet(self.field_5_options.get() as i32)}
    // java public boolean getIconic() { return iconic.isSet(this.field_5_options); }
    pub fn getIconic(&self)->bool{iconic.isSet(self.field_5_options.get() as i32)}
    // java public boolean getDisplayHorizontalScrollbar() { return hscroll.isSet(this.field_5_options); }
    pub fn getDisplayHorizontalScrollbar(&self)->bool{hscroll.isSet(self.field_5_options.get() as i32)}
    // java public boolean getDisplayVerticalScrollbar() { return vscroll.isSet(this.field_5_options); }
    pub fn getDisplayVerticalScrollbar(&self)->bool{vscroll.isSet(self.field_5_options.get() as i32)}
    // java public boolean getDisplayTabs() { return tabs.isSet(this.field_5_options); }
    pub fn getDisplayTabs(&self)->bool{tabs.isSet(self.field_5_options.get() as i32)}
    // java public int getActiveSheetIndex() { return this.field_6_active_sheet; }
    pub fn getActiveSheetIndex(&self)->i32{self.field_6_active_sheet.get()}
    // java public short getSelectedTab() { return (short)this.getActiveSheetIndex(); }
    pub fn getSelectedTab(&self)->i16{self.getActiveSheetIndex() as i16}
    // java public int getFirstVisibleTab() { return this.field_7_first_visible_tab; }
    pub fn getFirstVisibleTab(&self)->i32{self.field_7_first_visible_tab.get()}
    // java public short getDisplayedTab() { return (short)this.getFirstVisibleTab(); }
    pub fn getDisplayedTab(&self)->i16{self.getFirstVisibleTab() as i16}
    // java public short getNumSelectedTabs() { return this.field_8_num_selected_tabs; }
    pub fn getNumSelectedTabs(&self)->i16{self.field_8_num_selected_tabs.get()}
    // java public short getTabWidthRatio() { return this.field_9_tab_width_ratio; }
    pub fn getTabWidthRatio(&self)->i16{self.field_9_tab_width_ratio.get()}
    // java public String toString() {
    pub fn toString(&self)->String {
        // java StringBuffer buffer = new StringBuffer(); buffer.append("[WINDOW1]\n");
        let mut buffer=String::from("[WINDOW1]\n");
        // java buffer.append("    .h_hold          = ").append(Integer.toHexString(this.getHorizontalHold())).append("\n");
        buffer.push_str(&format!("    .h_hold          = {:x}\n",self.getHorizontalHold() as i32));
        // java buffer.append("    .v_hold          = ").append(Integer.toHexString(this.getVerticalHold())).append("\n");
        buffer.push_str(&format!("    .v_hold          = {:x}\n",self.getVerticalHold() as i32));
        // java buffer.append("    .width           = ").append(Integer.toHexString(this.getWidth())).append("\n");
        buffer.push_str(&format!("    .width           = {:x}\n",self.getWidth() as i32));
        // java buffer.append("    .height          = ").append(Integer.toHexString(this.getHeight())).append("\n");
        buffer.push_str(&format!("    .height          = {:x}\n",self.getHeight() as i32));
        // java buffer.append("    .options         = ").append(Integer.toHexString(this.getOptions())).append("\n");
        buffer.push_str(&format!("    .options         = {:x}\n",self.getOptions() as i32));
        // java buffer.append("        .hidden      = ").append(this.getHidden()).append("\n");
        buffer.push_str(&format!("        .hidden      = {}\n",self.getHidden()));
        // java buffer.append("        .iconic      = ").append(this.getIconic()).append("\n");
        buffer.push_str(&format!("        .iconic      = {}\n",self.getIconic()));
        // java buffer.append("        .hscroll     = ").append(this.getDisplayHorizontalScrollbar()).append("\n");
        buffer.push_str(&format!("        .hscroll     = {}\n",self.getDisplayHorizontalScrollbar()));
        // java buffer.append("        .vscroll     = ").append(this.getDisplayVerticalScrollbar()).append("\n");
        buffer.push_str(&format!("        .vscroll     = {}\n",self.getDisplayVerticalScrollbar()));
        // java buffer.append("        .tabs        = ").append(this.getDisplayTabs()).append("\n");
        buffer.push_str(&format!("        .tabs        = {}\n",self.getDisplayTabs()));
        // java buffer.append("    .activeSheet     = ").append(Integer.toHexString(this.getActiveSheetIndex())).append("\n");
        buffer.push_str(&format!("    .activeSheet     = {:x}\n",self.getActiveSheetIndex()));
        // java buffer.append("    .firstVisibleTab    = ").append(Integer.toHexString(this.getFirstVisibleTab())).append("\n");
        buffer.push_str(&format!("    .firstVisibleTab    = {:x}\n",self.getFirstVisibleTab()));
        // java buffer.append("    .numselectedtabs = ").append(Integer.toHexString(this.getNumSelectedTabs())).append("\n");
        buffer.push_str(&format!("    .numselectedtabs = {:x}\n",self.getNumSelectedTabs() as i32));
        // java buffer.append("    .tabwidthratio   = ").append(Integer.toHexString(this.getTabWidthRatio())).append("\n");
        buffer.push_str(&format!("    .tabwidthratio   = {:x}\n",self.getTabWidthRatio() as i32));
        // java buffer.append("[/WINDOW1]\n"); return buffer.toString();
        buffer.push_str("[/WINDOW1]\n");buffer
    }
    // java public void serializeData(LittleEndianOutput out) {
    pub fn serializeData(&self,out:&mut dyn LittleEndianOutput){
        // java out.writeShort(this.getHorizontalHold());
        out.writeShort(self.getHorizontalHold() as i32);
        // java out.writeShort(this.getVerticalHold());
        out.writeShort(self.getVerticalHold() as i32);
        // java out.writeShort(this.getWidth());
        out.writeShort(self.getWidth() as i32);
        // java out.writeShort(this.getHeight());
        out.writeShort(self.getHeight() as i32);
        // java out.writeShort(this.getOptions());
        out.writeShort(self.getOptions() as i32);
        // java out.writeShort(this.getActiveSheetIndex());
        out.writeShort(self.getActiveSheetIndex());
        // java out.writeShort(this.getFirstVisibleTab());
        out.writeShort(self.getFirstVisibleTab());
        // java out.writeShort(this.getNumSelectedTabs());
        out.writeShort(self.getNumSelectedTabs() as i32);
        // java out.writeShort(this.getTabWidthRatio());
        out.writeShort(self.getTabWidthRatio() as i32);
    }
    // java protected int getDataSize() { return 18; }
    pub fn getDataSize(&self)->i32{18}
    // java public short getSid() { return 61; }
    pub fn getSid(&self)->i16{61}
}
// rust: StandardRecord's original dynamic dispatch, no duplicated serializer.
impl StandardRecord for WindowOneRecord {
    fn getDataSize(&self)->i32{WindowOneRecord::getDataSize(self)}
    fn getSid(&self)->i16{WindowOneRecord::getSid(self)}
    fn serializeData(&self,out:&mut dyn LittleEndianOutput){WindowOneRecord::serializeData(self,out)}
}
