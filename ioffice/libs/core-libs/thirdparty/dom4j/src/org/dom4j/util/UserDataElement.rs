// Java: import org.dom4j.Element
// use crate::org::dom4j::Element;
// Java: import org.dom4j.QName
// use crate::org::dom4j::QName;
// Java: import org.dom4j.tree.DefaultElement
// use crate::org::dom4j::tree::DefaultElement;

// Java: public class UserDataElement extends DefaultElement
pub struct UserDataElement {
    // Java field: Object data
    pub(crate) data: i32,
}

// Java: public class UserDataElement extends DefaultElement
impl UserDataElement {
    // Java: UserDataElement(String name)
    pub fn new(_name: &str) -> Self {
      // Java: super(name);
      Self { data: 0 }
    }

    // Java: UserDataElement(QName qname)
    pub fn new_with_qname(_qname: i32) -> Self {
      // Java: super(qname);
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

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return super.toString() + " userData: " + this.data;
      String::new()
    }

    // Java: UserDataElement clone()
    pub fn clone_element(&self) -> Self {
      // Java: UserDataElement answer = (UserDataElement)super.clone();
      // Java: if (answer != this)
      {
         // Java: answer.data = this.getCopyOfUserData();
      }
      // Java: return answer;
      Self { data: 0 }
    }

    // Java: Object getCopyOfUserData()
    pub fn getCopyOfUserData(&self) -> i32 {
      // Java: return this.data;
      0
    }

    // Java: Element createElement(String name)
    pub fn createElement_name(&self, _name: &str) -> i32 {
      // Java: Element answer = this.getDocumentFactory().createElement(name);
      // Java: answer.setData(this.getCopyOfUserData());
      // Java: return answer;
      0
    }

    // Java: Element createElement(QName qName)
    pub fn createElement_qname(&self, _qName: i32) -> i32 {
      // Java: Element answer = this.getDocumentFactory().createElement(qName);
      // Java: answer.setData(this.getCopyOfUserData());
      // Java: return answer;
      0
    }

}
