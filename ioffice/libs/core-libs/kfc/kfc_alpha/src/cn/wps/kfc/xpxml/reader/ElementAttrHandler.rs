use super::XmlRoAttr::XmlRoAttr;
// java public interface ElementAttrHandler
pub trait ElementAttrHandler {
    // java void addElementAttr(int element, XmlRoAttr attrsNotCopy);
    fn addElementAttr(&self,element:i32,attrsNotCopy:&dyn XmlRoAttr);
}
