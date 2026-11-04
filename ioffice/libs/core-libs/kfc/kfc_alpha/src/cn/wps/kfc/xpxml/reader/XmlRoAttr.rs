use super::XmlRoAttrValue::XmlRoAttrValue;

// Java: public interface XmlRoAttr extends XmlRoAttrValue
pub trait XmlRoAttr: XmlRoAttrValue {
    // Java: abstract XmlRoAttr getByName(int var1)
    fn get_by_name(&self, var1: i32) -> Option<&dyn XmlRoAttr>;

    // Java: abstract XmlRoAttr getAttrAt(int var1)
    fn get_attr_at(&self, var1: i32) -> Option<&dyn XmlRoAttr>;

    // Java: abstract int getNameAt(int var1)
    fn get_name_at(&self, var1: i32) -> i32;

    // Java: abstract int count()
    fn count(&self) -> i32;

    // Java: abstract String getCharacters()
    fn get_characters(&self) -> String;

    // Java: abstract int getName()
    fn get_name(&self) -> i32;

    // Java: abstract XmlRoAttr getByName(int var1, boolean var2)
    fn get_by_name_with_flag(&self, var1: i32, var2: bool) -> Option<&dyn XmlRoAttr>;

    // Java: abstract boolean isLeaf()
    fn is_leaf(&self) -> bool;
}
