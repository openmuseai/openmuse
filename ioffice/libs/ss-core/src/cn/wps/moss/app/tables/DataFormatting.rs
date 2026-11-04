use crate::java_runtime::{Rc,RefCell,Cell};
use crate::cn::wps::moss::filefmt::biff8::record::cf::{FontFormatting::FontFormatting,BorderFormatting::BorderFormatting,PatternFormatting::PatternFormatting,NumberFormatFormatting::NumberFormatFormatting};
// java public class DataFormatting
pub struct DataFormatting{font:RefCell<Option<Rc<FontFormatting>>>,border:RefCell<Option<Rc<RefCell<BorderFormatting>>>>,pattern:RefCell<Option<Rc<RefCell<PatternFormatting>>>>,numFmt:RefCell<Option<Rc<NumberFormatFormatting>>>}impl DataFormatting{// java public DataFormatting() {
// java this((FontFormatting)null, (BorderFormatting)null, (PatternFormatting)null);
// java }
pub fn new()->Self {
Self{font:RefCell::new(None),border:RefCell::new(None),pattern:RefCell::new(None),numFmt:RefCell::new(None)}
}
// java public FontFormatting getFontFormatting() {
// java return this.fontFormatting;
// java }
pub fn getFontFormatting(&self)->Option<Rc<FontFormatting>> {
self.font.borrow().clone()
}
// java public void setFontFormatting(FontFormatting fontFormatting) {
// java this.fontFormatting = fontFormatting;
// java }
pub fn setFontFormatting(&self,v:Option<Rc<FontFormatting>>) {
*self.font.borrow_mut()=v;
}
// java public BorderFormatting getBorderFormatting() {
// java return this.borderFormatting;
// java }
pub fn getBorderFormatting(&self)->Option<Rc<RefCell<BorderFormatting>>> {
self.border.borrow().clone()
}
// java public void setBorderFormatting(BorderFormatting borderFormatting) {
// java this.borderFormatting = borderFormatting;
// java }
pub fn setBorderFormatting(&self,v:Option<Rc<RefCell<BorderFormatting>>>) {
*self.border.borrow_mut()=v;
}
// java public PatternFormatting getPatternFormatting() {
// java return this.patternFormatting;
// java }
pub fn getPatternFormatting(&self)->Option<Rc<RefCell<PatternFormatting>>> {
self.pattern.borrow().clone()
}
// java public void setPatternFormatting(PatternFormatting patternFormatting) {
// java this.patternFormatting = patternFormatting;
// java }
pub fn setPatternFormatting(&self,v:Option<Rc<RefCell<PatternFormatting>>>) {
*self.pattern.borrow_mut()=v;
}
// java public NumberFormatFormatting getNumberFormatFormatting() {
// java return this.numFmtFormatting;
// java }
pub fn getNumberFormatFormatting(&self)->Option<Rc<NumberFormatFormatting>> {
self.numFmt.borrow().clone()
}
// java public void setNumberFormatFormatting(NumberFormatFormatting numFmt) {
// java this.numFmtFormatting = numFmt;
// java }
pub fn setNumberFormatFormatting(&self,v:Option<Rc<NumberFormatFormatting>>) {
*self.numFmt.borrow_mut()=v;
}
// java public boolean isEmpty() {
// java return this.getBorderFormatting() == null && this.getFontFormatting() == null && this.getNumberFormatFormatting() == null && this.getPatternFormatting() == null;
// java }
pub fn isEmpty(&self)->bool {
self.font.borrow().is_none()&&self.border.borrow().is_none()&&self.pattern.borrow().is_none()&&self.numFmt.borrow().is_none()
}
}
