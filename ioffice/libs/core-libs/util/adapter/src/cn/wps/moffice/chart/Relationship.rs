// Java: public class Relationship
pub struct Relationship {
    // Java field: String id
    pub(crate) id: String,
    // Java field: String relationshipType
    pub(crate) relationshipType: String,
    // Java field: TargetMode targetMode
    pub(crate) targetMode: TargetMode,
    // Java field: String targetUri
    pub(crate) targetUri: String,
}

// Java: public enum TargetMode
pub enum TargetMode {
    Internal,
    External,
}

// Java: public class Relationship
impl Relationship {
    // Java: Relationship(String targetUri, TargetMode targetMode, String relationshipType, String id)
    pub fn new(_targetUri: &str, _targetMode: TargetMode, _relationshipType: &str, _id: &str) -> Self {
      // Java: super();
      // Java: if (targetUri == null)
      {
         // Java: throw new IllegalArgumentException("targetUri");
      }
      // Java: else if (relationshipType == null)
      {
         // Java: throw new IllegalArgumentException("relationshipType");
      }
      // Java: else if (id == null)
      {
         // Java: throw new IllegalArgumentException("id");
      }
      // Java: else
      {
         // Java: this.targetUri = targetUri;
         // Java: this.targetMode = targetMode;
         // Java: this.relationshipType = relationshipType;
         // Java: this.id = id;
      }
      Relationship {
          id: _id.to_string(),
          relationshipType: _relationshipType.to_string(),
          targetMode: _targetMode,
          targetUri: _targetUri.to_string(),
      }
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
    pub fn getTargetMode(&self) -> &TargetMode {
      // Java: return this.targetMode;
      &self.targetMode
    }

    // Java: String getTargetURI()
    pub fn getTargetURI(&self) -> &str {
      // Java: return this.targetUri;
      &self.targetUri
    }

}

// Java: public enum TargetMode
impl TargetMode {
    // Java: TargetMode()
    pub fn new() -> Self {
      TargetMode::Internal
    }

}
