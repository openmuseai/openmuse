// Java: import org.dom4j.QName
// use crate::org::dom4j::QName;
// Java: import org.dom4j.tree.DefaultAttribute
// use crate::org::dom4j::tree::DefaultAttribute;

// Java: public class UserDataAttribute extends DefaultAttribute
pub struct UserDataAttribute {
    // Java field: Object data
    pub(crate) data: i32,
}

// Java: public class UserDataAttribute extends DefaultAttribute
impl UserDataAttribute {
    // Java: UserDataAttribute(QName qname)
    pub fn new(_qname: i32) -> Self {
      // Java: super(qname);
      Self { data: 0 }
    }

    // Java: UserDataAttribute(QName qname, String text)
    pub fn new_with_text(_qname: i32, _text: &str) -> Self {
      // Java: super(qname, text);
      Self { data: 0 }
    }

    // Java: Object getData()
    pub fn getData(&self) -> i32 {
      // Java: return this.data;
      0
    }

    // Java: void setData(Object data)
    pub fn setData(&self, _data: i32) {
      // Java: this.data = data;
    }

}
