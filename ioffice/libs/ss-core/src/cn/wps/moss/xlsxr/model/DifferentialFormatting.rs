use crate::{java_runtime::{Rc,RefCell},cn::wps::moss::filefmt::biff8::record::cf::{FontFormatting::FontFormatting,BorderFormatting::BorderFormatting,PatternFormatting::PatternFormatting,NumberFormatFormatting::NumberFormatFormatting}};
// java public final class DifferentialFormatting {
pub struct DifferentialFormatting{
    // java public FontFormatting fontFormatting;
    pub fontFormatting:Option<Rc<FontFormatting>>,
    // java public BorderFormatting borderFormatting;
    pub borderFormatting:Option<Rc<RefCell<BorderFormatting>>>,
    // java public PatternFormatting patternFormatting;
    pub patternFormatting:Option<Rc<RefCell<PatternFormatting>>>,
    // java private NumberFormatFormatting numFmtFormatting;
    numFmtFormatting:Option<Rc<NumberFormatFormatting>>,
}
impl DifferentialFormatting{
    // java public DifferentialFormatting() { this((FontFormatting)null, (BorderFormatting)null, (PatternFormatting)null); }
    pub fn DifferentialFormatting()->Self{Self::DifferentialFormatting__3(None,None,None)}
    // java public DifferentialFormatting(FontFormatting font, BorderFormatting border, PatternFormatting pattern) {
    pub fn DifferentialFormatting__3(font:Option<Rc<FontFormatting>>,border:Option<Rc<RefCell<BorderFormatting>>>,pattern:Option<Rc<RefCell<PatternFormatting>>>)->Self{
        // java this.fontFormatting = font; this.borderFormatting = border; this.patternFormatting = pattern;
        Self{fontFormatting:font,borderFormatting:border,patternFormatting:pattern,numFmtFormatting:None}
    }
    // java public void setNumberFormatFormatting(NumberFormatFormatting numFmt) { this.numFmtFormatting = numFmt; }
    pub fn setNumberFormatFormatting(&mut self,numFmt:Option<Rc<NumberFormatFormatting>>){self.numFmtFormatting=numFmt;}
    // java public NumberFormatFormatting numberFormatFormatting() { return this.numFmtFormatting; }
    pub fn numberFormatFormatting(&self)->Option<Rc<NumberFormatFormatting>>{self.numFmtFormatting.clone()}
}
