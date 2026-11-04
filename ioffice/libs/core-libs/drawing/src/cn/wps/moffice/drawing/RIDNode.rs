#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import cn.wps.moffice.util.FileUtil
// Java: import java.net.URI

use super::RelationshipCollection::RelationshipCollection;

// Java: public class RIDNode
pub struct RIDNode {
    // Java field: String path
    pub(crate) path: Option<String>,
    // Java field: RelationshipCollection relationshipCollection
    pub(crate) relationshipCollection: Option<RelationshipCollection>,
}

// Java: public class RIDNode
impl RIDNode {
    // Java: RIDNode()
    pub fn new() -> Self {
      // Java: super();
      RIDNode { path: None, relationshipCollection: None }
    }

    // Java: RIDNode(String drawingPath, RelationshipCollection relationshipCollection)
    pub fn new_with(drawingPath: String, relationshipCollection: RelationshipCollection) -> Self {
      // Java: super();
      // Java: this.path = drawingPath;
      // Java: this.relationshipCollection = relationshipCollection;
      RIDNode { path: Some(drawingPath), relationshipCollection: Some(relationshipCollection) }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: if (null != this.path)
      if self.path.is_some()
      {
         // Java: FileUtil.delFile(this.path);
         if let Some(ref path) = self.path {
             let _ = std::fs::remove_file(path);
         }
      }
      // Java: if (null != this.relationshipCollection)
      if self.relationshipCollection.is_some()
      {
         // Java: int i = 0;
         let mut i = 0;
         // Java: for(int size = this.relationshipCollection.size(); i < size; ++i)
         let size = self.relationshipCollection.as_ref().unwrap().size();
         while i < size
         {
            // Java: Relationship relation = this.relationshipCollection.getRelationship(i);
            let relation = self.relationshipCollection.as_ref().unwrap().getRelationship(i);
            // Java: if (null != relation)
            if relation.is_some()
            {
               // Java: URI targetURI = relation.getTargetURI();
               // TODO: Relationship type not yet available
               // Java: if (null != targetURI)
               {
                  // Java: String mediaPath = targetURI.getPath();
                  // Java: FileUtil.delFile(mediaPath);
                  // TODO: Relationship.getTargetURI not yet available
               }
            }
            i += 1;
         }
         // Java: this.path = null;
         self.path = None;
         // Java: this.relationshipCollection = null;
         self.relationshipCollection = None;
      }
    }

}
