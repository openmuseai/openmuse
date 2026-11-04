use crate::{java_runtime::{Rc,Cell},cn::wps::moss::{app::KmoBook::KmoBook,core::book::KmoPalette::KmoPalette,xlsxr::util::ColorHelper::ColorHelper}};
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
// java package cn.wps.moss.xlsxr.handler.style;
// java
// java import cn.wps.kfc.xpxml.reader.DefaultElementHandler;
// java import cn.wps.kfc.xpxml.reader.ElementHandler;
// java import cn.wps.kfc.xpxml.reader.XmlRoAttr;
// java import cn.wps.moss.app.KmoBook;
// java import cn.wps.moss.core.book.KmoPalette;
// java import cn.wps.moss.xlsxr.util.ColorHelper;
// java
// java class IndexedColorsHandler extends DefaultElementHandler {
// java private KmoPalette palette;
// java private RGBColorHandler rgbHandler;
// java private int rgbIndex = -1;
// java private KmoBook book;
// java
// java public IndexedColorsHandler(KmoBook book) {
// java this.book = book;
// java this.palette = book.getPalette();
// java this.rgbHandler = new RGBColorHandler();
// java }
// java
// java public ElementHandler enterSubElement(int subElement) {
// java ++this.rgbIndex;
// java if (subElement == 5180) {
// java this.rgbHandler.init(this.rgbIndex);
// java return this.rgbHandler;
// java } else {
// java return null;
// java }
// java }
// java
// java public void endElement(int element) {
// java this.palette.setIsPaletteMod(true);
// java this.book.setPalette(this.palette);
// java }
// java
// java private final class RGBColorHandler extends DefaultElementHandler {
// java private int index;
// java
// java private RGBColorHandler() {
// java this.index = -1;
// java }
// java
// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java XmlRoAttr tmp;
// java if ((tmp = attrsNotCopy.getByName(5155)) != null) {
// java ColorHelper.setPaletteColor(IndexedColorsHandler.this.palette, this.index, tmp.toString());
// java }
// java
// java }
// java
// java public void init(int index) {
// java this.index = index;
// java }
// java }
// java }

pub struct IndexedColorsHandler {book:Rc<KmoBook>,palette:Rc<KmoPalette>,rgbIndex:Cell<i32>,rgbHandler:Rc<RGBColorHandler>}
pub struct RGBColorHandler {palette:Rc<KmoPalette>,index:Cell<i32>}
impl IndexedColorsHandler {pub fn new(book:Rc<KmoBook>)->Self{let palette=book.getPalette();Self{rgbHandler:Rc::new(RGBColorHandler{palette:palette.clone(),index:Cell::new(-1)}),book,palette,rgbIndex:Cell::new(-1)}}}
struct RGBRef(Rc<RGBColorHandler>);
impl ElementHandler for RGBRef {
fn startElement(&self,_:i32,a:&dyn XmlRoAttr){if let Some(rgb)=a.get_by_name(5155){ColorHelper::setPaletteColor(&self.0.palette,self.0.index.get(),&rgb.to_string());}}
fn endElement(&self,_:i32){}fn characters(&self,_:&str){}fn enterSubElement(&self,_:i32)->Option<Box<dyn ElementHandler>>{None}}
impl ElementHandler for IndexedColorsHandler {
fn startElement(&self,_:i32,_:&dyn XmlRoAttr){}fn characters(&self,_:&str){}
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{self.rgbIndex.set(self.rgbIndex.get().wrapping_add(1));if e==5180{self.rgbHandler.index.set(self.rgbIndex.get());Some(Box::new(RGBRef(self.rgbHandler.clone())))}else{None}}
fn endElement(&self,_:i32){self.palette.setIsPaletteMod(true);self.book.setPalette(self.palette.clone());}
}
