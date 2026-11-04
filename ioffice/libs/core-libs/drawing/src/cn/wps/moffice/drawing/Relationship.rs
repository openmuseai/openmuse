#![allow(non_snake_case, dead_code)]

// Java: import java.net.URI

// Java field: String ID_ATTRIBUTE_NAME
pub(crate) const ID_ATTRIBUTE_NAME: &str = "Id";
// Java field: String RELATIONSHIPS_TAG_NAME
pub(crate) const RELATIONSHIPS_TAG_NAME: &str = "Relationships";
// Java field: String RELATIONSHIP_TAG_NAME
pub(crate) const RELATIONSHIP_TAG_NAME: &str = "Relationship";
// Java field: String TARGET_ATTRIBUTE_NAME
pub(crate) const TARGET_ATTRIBUTE_NAME: &str = "Target";
// Java field: String TARGET_MODE_ATTRIBUTE_NAME
pub(crate) const TARGET_MODE_ATTRIBUTE_NAME: &str = "TargetMode";
// Java field: String TYPE_ATTRIBUTE_NAME
pub(crate) const TYPE_ATTRIBUTE_NAME: &str = "Type";

// Java: public enum TargetMode
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TargetMode { // struct TargetMode
    INTERNAL,
    EXTERNAL,
}

// Java: public class Relationship
pub struct Relationship {
    // Java field: String id
    pub(crate) id: String,
    // Java field: String relationshipType
    pub(crate) relationshipType: String,
    // Java field: TargetMode targetMode
    pub(crate) targetMode: TargetMode,
    // Java field: URI targetUri
    pub(crate) targetUri: String,
    // Java field: int mediaId
    pub(crate) mediaId: i32,
}

// Java: public class Relationship
impl Relationship {
    // Java: Relationship(URI targetUri, TargetMode targetMode, String relationshipType, String id, int mediaId)
    pub fn new(targetUri: String, targetMode: TargetMode, relationshipType: String, id: String, mediaId: i32) -> Self {
      // Java: super();
      let mut instance = Relationship { targetUri: String::new(), targetMode: TargetMode::INTERNAL, relationshipType: String::new(), id: String::new(), mediaId: 0 };
      // Java: if (targetUri == null)
      if targetUri.is_empty() {
         // Java: throw new IllegalArgumentException("targetUri");
         panic!("targetUri");
      }
      // Java: else if (relationshipType == null)
      else if relationshipType.is_empty() {
         // Java: throw new IllegalArgumentException("relationshipType");
         panic!("relationshipType");
      }
      // Java: else if (id == null)
      else if id.is_empty() {
         // Java: throw new IllegalArgumentException("id");
         panic!("id");
      }
      // Java: else
      else {
         instance.init_fields(targetUri, targetMode, relationshipType, id, mediaId);
      }
      instance
    }

    fn init_fields(&mut self, targetUri: String, targetMode: TargetMode, relationshipType: String, id: String, mediaId: i32) {
      // Java: this.targetUri = targetUri;
      self.targetUri = targetUri;
      // Java: this.targetMode = targetMode;
      self.targetMode = targetMode;
      // Java: this.relationshipType = relationshipType;
      self.relationshipType = relationshipType;
      // Java: this.id = id;
      self.id = id;
      // Java: this.mediaId = mediaId;
      self.mediaId = mediaId;
    }

    // Java: String getId()
    pub fn getId(&self) -> &str {
      // Java: return this.id;
      &self.id
    }

    // Java: String getRelationshipType()
    pub fn getRelationshipType(&self) -> &str {
      // Java: return this.relationshipType;
      &self.relationshipType
    }

    // Java: TargetMode getTargetMode()
    pub fn getTargetMode(&self) -> TargetMode {
      // Java: return this.targetMode;
      self.targetMode
    }

    // Java: URI getTargetURI()
    pub fn getTargetURI(&self) -> &str {
      // Java: return this.targetUri;
      &self.targetUri
    }

    // Java: int getMediaId()
    pub fn getMediaId(&self) -> i32 {
      // Java: return this.mediaId;
      self.mediaId
    }

}

// Java: public enum TargetMode
impl TargetMode {
    // Java: TargetMode()
    pub fn new() -> Self {
        TargetMode::INTERNAL
    }
}
