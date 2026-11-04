use crate::{java_runtime::{Rc,Weak,RefCell},cn::wps::moss::{app::KmoBook::KmoBook,xlsxw::label::Label::Label}};
use super::{BordersLabel::BordersLabel,CellStyleXfsLabel::CellStyleXfsLabel,CellXfsLabel::CellXfsLabel,FillsLabel::FillsLabel,FontsLabel::FontsLabel,NumFmtsLabel::NumFmtsLabel,ColorsLabel::ColorsLabel,DifferentialFormattingLabel::DifferentialFormattingLabel,CellStylesLabel::CellStylesLabel,TableStylesLabel::TableStylesLabel};
use kfc_kfc_alpha::kfc::xpxml::writer::{XmlWriter::XmlWriter,r#impl::MemoryXmlWriter::MemoryXmlWriter};
// java public final class StyleSheetLabel implements Label {
pub struct StyleSheetLabel{
    // java private BordersLabel bordersLabel;
    bordersLabel:Rc<RefCell<BordersLabel>>,
    // java private CellStyleXfsLabel styleXfsLabel;
    styleXfsLabel:Rc<RefCell<CellStyleXfsLabel>>,
    // java private CellXfsLabel cellXfsLabel;
    cellXfsLabel:Rc<RefCell<CellXfsLabel>>,
    // java private FillsLabel fillsLabel;
    fillsLabel:Rc<RefCell<FillsLabel>>,
    // java private FontsLabel fontsLabel;
    fontsLabel:Rc<RefCell<FontsLabel>>,
    // java private NumFmtsLabel numFmtsLabel;
    numFmtsLabel:Rc<RefCell<NumFmtsLabel>>,
    // java private ColorsLabel colorsLabel;
    colorsLabel:Rc<RefCell<ColorsLabel>>,
    // java private DifferentialFormattingLabel dxfLabel;
    dxfLabel:Rc<RefCell<DifferentialFormattingLabel>>,
    // java private KmoBook book;
    book:Rc<KmoBook>,
    // Native reference adapter: child getter handles retain the Java parent graph;
    // weak back references avoid leaking an Arc cycle without copying Java objects.
    selfRef:Weak<StyleSheetLabel>,
}
pub struct StyleLabelRef<T>{value:Rc<RefCell<T>>,_owner:Rc<StyleSheetLabel>}
impl<T> std::ops::Deref for StyleLabelRef<T>{type Target=RefCell<T>;fn deref(&self)->&Self::Target{&self.value}}
impl<T> Clone for StyleLabelRef<T>{fn clone(&self)->Self{Self{value:self.value.clone(),_owner:self._owner.clone()}}}
impl StyleSheetLabel{
    // java public StyleSheetLabel(KmoBook book) {
    pub fn StyleSheetLabel(book:Rc<KmoBook>)->Rc<Self>{Rc::new_cyclic(|this|{
        // java this.book = book;
        let book=book.clone();
        // java this.bordersLabel = new BordersLabel();
        let bordersLabel=Rc::new(RefCell::new(BordersLabel::BordersLabel()));
        // java this.styleXfsLabel = new CellStyleXfsLabel(this, book);
        let styleXfsLabel=Rc::new(RefCell::new(CellStyleXfsLabel::CellStyleXfsLabel(this.clone(),book.clone())));
        // java this.cellXfsLabel = new CellXfsLabel(this, book);
        let cellXfsLabel=Rc::new(RefCell::new(CellXfsLabel::CellXfsLabel(this.clone(),book.clone())));
        // java this.fillsLabel = new FillsLabel();
        let fillsLabel=Rc::new(RefCell::new(FillsLabel::FillsLabel()));
        // java this.fontsLabel = new FontsLabel();
        let fontsLabel=Rc::new(RefCell::new(FontsLabel::FontsLabel()));
        // java this.numFmtsLabel = new NumFmtsLabel(book);
        let numFmtsLabel=Rc::new(RefCell::new(NumFmtsLabel::NumFmtsLabel(book.clone())));
        // java this.dxfLabel = new DifferentialFormattingLabel(book);
        let dxfLabel=Rc::new(RefCell::new(DifferentialFormattingLabel::DifferentialFormattingLabel(book.clone())));
        // java this.colorsLabel = new ColorsLabel(book);
        let colorsLabel=Rc::new(RefCell::new(ColorsLabel::ColorsLabel(&book)));
        Self{bordersLabel,styleXfsLabel,cellXfsLabel,fillsLabel,fontsLabel,numFmtsLabel,colorsLabel,dxfLabel,book,selfRef:this.clone()}
    })}
    // java BordersLabel getBordersLabel() { return this.bordersLabel; }
    pub(crate) fn getBordersLabel(&self)->StyleLabelRef<BordersLabel>{StyleLabelRef{value:self.bordersLabel.clone(),_owner:self.selfRef.upgrade().unwrap()}}
    // java CellStyleXfsLabel getCellStyleXfsLabel() { return this.styleXfsLabel; }
    pub(crate) fn getCellStyleXfsLabel(&self)->StyleLabelRef<CellStyleXfsLabel>{StyleLabelRef{value:self.styleXfsLabel.clone(),_owner:self.selfRef.upgrade().unwrap()}}
    // java public CellXfsLabel getCellXfsLabel() { return this.cellXfsLabel; }
    pub fn getCellXfsLabel(&self)->StyleLabelRef<CellXfsLabel>{StyleLabelRef{value:self.cellXfsLabel.clone(),_owner:self.selfRef.upgrade().unwrap()}}
    // java FillsLabel getFillsLabel() { return this.fillsLabel; }
    pub(crate) fn getFillsLabel(&self)->StyleLabelRef<FillsLabel>{StyleLabelRef{value:self.fillsLabel.clone(),_owner:self.selfRef.upgrade().unwrap()}}
    // java FontsLabel getFontsLabel() { return this.fontsLabel; }
    pub(crate) fn getFontsLabel(&self)->StyleLabelRef<FontsLabel>{StyleLabelRef{value:self.fontsLabel.clone(),_owner:self.selfRef.upgrade().unwrap()}}
    // java NumFmtsLabel getNumFmtsLabel() { return this.numFmtsLabel; }
    pub(crate) fn getNumFmtsLabel(&self)->StyleLabelRef<NumFmtsLabel>{StyleLabelRef{value:self.numFmtsLabel.clone(),_owner:self.selfRef.upgrade().unwrap()}}
    // java public DifferentialFormattingLabel getDifferentialFormattingLabel() { return this.dxfLabel; }
    pub fn getDifferentialFormattingLabel(&self)->StyleLabelRef<DifferentialFormattingLabel>{StyleLabelRef{value:self.dxfLabel.clone(),_owner:self.selfRef.upgrade().unwrap()}}
    // java public void write(XmlWriter writer) {
    pub fn write(&self,writer:&mut dyn XmlWriter){
        // java MemoryXmlWriter cellXfsWriter = new MemoryXmlWriter(); this.cellXfsLabel.write(cellXfsWriter);
        let mut cellXfsWriter=MemoryXmlWriter::new();self.cellXfsLabel.borrow().write(&mut cellXfsWriter);
        // java MemoryXmlWriter styleXfsWriter = new MemoryXmlWriter(); this.styleXfsLabel.write(styleXfsWriter);
        let mut styleXfsWriter=MemoryXmlWriter::new();self.styleXfsLabel.borrow().write(&mut styleXfsWriter);
        // java this.numFmtsLabel.write(writer); this.fontsLabel.write(writer); this.fillsLabel.write(writer); this.bordersLabel.write(writer);
        self.numFmtsLabel.borrow().write(writer);self.fontsLabel.borrow().write(writer);self.fillsLabel.borrow().write(writer);self.bordersLabel.borrow().write(writer);
        // java writer.pushWriter(styleXfsWriter); writer.pushWriter(cellXfsWriter);
        writer.pushWriter(Box::new(styleXfsWriter));writer.pushWriter(Box::new(cellXfsWriter));
        // java CellStylesLabel.write(writer, this.book.getXFManager(), this.styleXfsLabel);
        CellStylesLabel::write(writer,&self.book.getXFManager().borrow(),&self.styleXfsLabel.borrow());
        // java TableStylesLabel.addWriteForDxfs(this.book.getXFManager(), this);
        TableStylesLabel::addWriteForDxfs(&self.book.getXFManager().borrow(),self);
        // java this.dxfLabel.write(writer);
        self.dxfLabel.borrow().write(writer);
        // java TableStylesLabel.write(writer, this.book.getXFManager(), this);
        TableStylesLabel::write(writer,&self.book.getXFManager().borrow(),self);
        // java this.colorsLabel.write(writer);
        self.colorsLabel.borrow().write(writer);
    }
}
impl Label for StyleSheetLabel{fn write(&self,w:&mut dyn XmlWriter){StyleSheetLabel::write(self,w)}}
#[cfg(test)]
#[path="StyleSheetLabel.review.rs"]
mod review;
