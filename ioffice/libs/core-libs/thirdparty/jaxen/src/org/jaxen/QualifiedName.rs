// Java: import java.io.Serializable
use std::hash::{Hash, Hasher};

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 2734958615642751535;

// Java: public class QualifiedName implements Serializable
#[derive(Clone, Debug)]
pub struct QualifiedName {
    // Java field: String namespaceURI
    pub(crate) namespaceURI: String,
    // Java field: String localName
    pub(crate) localName: String,
}

// Java: public class QualifiedName implements Serializable
impl QualifiedName {
    pub(crate) fn new(namespaceURI: Option<&str>, localName: &str) -> Self {
        let ns = namespaceURI.unwrap_or("");
        QualifiedName {
            namespaceURI: ns.to_string(),
            localName: localName.to_string(),
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return this.localName.hashCode() ^ this.namespaceURI.hashCode();
      let mut h1 = std::collections::hash_map::DefaultHasher::new();
      self.localName.hash(&mut h1);
      let hash1 = h1.finish() as i32;
      let mut h2 = std::collections::hash_map::DefaultHasher::new();
      self.namespaceURI.hash(&mut h2);
      let hash2 = h2.finish() as i32;
      hash1 ^ hash2
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, other: &QualifiedName) -> bool {
      // Java: if (!(o instanceof QualifiedName))
      // Java: return false;
      // Java: else
      // Java: QualifiedName other = (QualifiedName)o;
      // Java: return this.namespaceURI.equals(other.namespaceURI) && other.localName.equals(this.localName);
      self.namespaceURI == other.namespaceURI && self.localName == other.localName
    }

    // Java: String getClarkForm()
    pub fn getClarkForm(&self) -> String {
      // Java: return "".equals(this.namespaceURI) ? this.localName : "{" + this.namespaceURI + "}:" + this.localName;
      if self.namespaceURI.is_empty() {
          self.localName.clone()
      } else {
          format!("{{{}}}:{}", self.namespaceURI, self.localName)
      }
    }
}

impl PartialEq for QualifiedName {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Eq for QualifiedName {}

impl Hash for QualifiedName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.namespaceURI.hash(state);
        self.localName.hash(state);
    }
}
