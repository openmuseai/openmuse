#![allow(non_snake_case, dead_code)]

// Java: import java.util.HashMap

use std::collections::HashMap;
use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class RelationshipCollection
pub struct RelationshipCollection {
    // Java field: HashMap<String, Relationship> relationshipsByID
    pub(crate) relationshipsByID: HashMap<String, Element>,
}

// Java: public class RelationshipCollection
impl RelationshipCollection {
    // Java: RelationshipCollection()
    pub fn new() -> Self {
      // Java: super();
      RelationshipCollection { relationshipsByID: HashMap::new() }
    }

    // Java: Relationship getRelationshipByID(String id)
    pub fn getRelationshipByID(&self, id: &str) -> Element {
      // Java: return (Relationship)this.relationshipsByID.get(id);
      self.relationshipsByID.get(id).cloned().flatten()
    }

    // Java: Relationship getRelationship(int index)
    pub fn getRelationship(&self, index: i32) -> Element {
      // Java: if (index >= 0 && index <= this.relationshipsByID.values().size())
      if index >= 0 && (index as usize) <= self.relationshipsByID.values().len() {
         // Java: Relationship retRel = null;
         let mut retRel: Element = None;
         // Java: int i = 0;
         let mut i = 0;
         // Java: for(Relationship rel : this.relationshipsByID.values())
         for rel in self.relationshipsByID.values() {
            // Java: if (index == i++)
            if index == i {
               // Java: return rel;
               return rel.clone();
            }
            i += 1;
         }
         // Java: return retRel;
         retRel
      }
      // Java: else
      else {
         // Java: throw new IllegalArgumentException("index");
         panic!("index");
      }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.relationshipsByID.size();
      self.relationshipsByID.len() as i32
    }

    // Java: void add(String id, Relationship relationship)
    pub fn add(&mut self, id: String, relationship: Element) {
      // Java: this.relationshipsByID.put(id, relationship);
      self.relationshipsByID.insert(id, relationship);
    }

}
