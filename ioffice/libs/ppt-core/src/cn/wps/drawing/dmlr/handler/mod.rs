use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

pub trait ElementHandler {
    fn kind(&self) -> &'static str;
    fn startElement(&mut self, element: i32, attrs: &XmlRoAttrList);
    fn enterSubElement(&mut self, subElement: i32) -> Option<Box<dyn ElementHandler>>;
    fn endElement(&mut self, element: i32);
    fn characters(&mut self, _chars: &str) {}
}

pub mod shape;
pub mod media;
pub mod font;
pub mod color;
pub mod style;
