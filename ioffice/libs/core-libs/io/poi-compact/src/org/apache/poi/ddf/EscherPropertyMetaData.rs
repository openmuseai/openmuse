// Java field: byte TYPE_ILLEGAL
pub const TYPE_ILLEGAL: i8 = -1;
// Java field: byte TYPE_UNKNOWN
pub const TYPE_UNKNOWN: i8 = 0;
// Java field: byte TYPE_BOOLEAN
pub const TYPE_BOOLEAN: i8 = 1;
// Java field: byte TYPE_RGB
pub const TYPE_RGB: i8 = 2;
// Java field: byte TYPE_SHAPEPATH
pub const TYPE_SHAPEPATH: i8 = 3;
// Java field: byte TYPE_SIMPLE
pub const TYPE_SIMPLE: i8 = 4;
// Java field: byte TYPE_ARRAY
pub const TYPE_ARRAY: i8 = 5;

// Java: public class EscherPropertyMetaData
pub struct EscherPropertyMetaData {
    // Java field: String description
    pub(crate) description: String,
    // Java field: byte type
    pub(crate) type_: i8,
}

// Java: public class EscherPropertyMetaData
impl EscherPropertyMetaData {
    // Java: EscherPropertyMetaData(String description)
    pub fn new(description: &str) -> Self {
        // Java: super();
        // Java: this.description = description;
        EscherPropertyMetaData {
            description: description.to_string(),
            type_: 0,
        }
    }

    // Java: EscherPropertyMetaData(String description, byte type)
    pub fn new_with_type(description: &str, type_val: i8) -> Self {
        // Java: super();
        // Java: this.description = description;
        // Java: this.type = type;
        EscherPropertyMetaData {
            description: description.to_string(),
            type_: type_val,
        }
    }

    // Java: String getDescription()
    pub fn getDescription(&self) -> &str {
        // Java: return this.description;
        &self.description
    }

    // Java: byte getType()
    pub fn getType(&self) -> i8 {
        // Java: return this.type;
        self.type_
    }
}
