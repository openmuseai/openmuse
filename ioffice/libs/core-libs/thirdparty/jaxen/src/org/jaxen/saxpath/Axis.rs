// Java: import org.jaxen.JaxenRuntimeException

// Java field: int INVALID_AXIS
pub const INVALID_AXIS: i32 = 0;
// Java field: int CHILD
pub const CHILD: i32 = 1;
// Java field: int DESCENDANT
pub const DESCENDANT: i32 = 2;
// Java field: int PARENT
pub const PARENT: i32 = 3;
// Java field: int ANCESTOR
pub const ANCESTOR: i32 = 4;
// Java field: int FOLLOWING_SIBLING
pub const FOLLOWING_SIBLING: i32 = 5;
// Java field: int PRECEDING_SIBLING
pub const PRECEDING_SIBLING: i32 = 6;
// Java field: int FOLLOWING
pub const FOLLOWING: i32 = 7;
// Java field: int PRECEDING
pub const PRECEDING: i32 = 8;
// Java field: int ATTRIBUTE
pub const ATTRIBUTE: i32 = 9;
// Java field: int NAMESPACE
pub const NAMESPACE: i32 = 10;
// Java field: int SELF
pub const SELF: i32 = 11;
// Java field: int DESCENDANT_OR_SELF
pub const DESCENDANT_OR_SELF: i32 = 12;
// Java field: int ANCESTOR_OR_SELF
pub const ANCESTOR_OR_SELF: i32 = 13;

// Java: public class Axis
pub struct Axis {
}

// Java: public class Axis
impl Axis {
    // Java: Axis()
    // Java: super();

    // Java: static String lookup(int axisNum)
    pub fn lookup_by_num(axisNum: i32) -> String {
      // Java: switch (axisNum)
      match axisNum {
         // Java: case 1:
         1 => {
            // Java: return "child";
            "child".to_string()
         }
         // Java: case 2:
         2 => {
            // Java: return "descendant";
            "descendant".to_string()
         }
         // Java: case 3:
         3 => {
            // Java: return "parent";
            "parent".to_string()
         }
         // Java: case 4:
         4 => {
            // Java: return "ancestor";
            "ancestor".to_string()
         }
         // Java: case 5:
         5 => {
            // Java: return "following-sibling";
            "following-sibling".to_string()
         }
         // Java: case 6:
         6 => {
            // Java: return "preceding-sibling";
            "preceding-sibling".to_string()
         }
         // Java: case 7:
         7 => {
            // Java: return "following";
            "following".to_string()
         }
         // Java: case 8:
         8 => {
            // Java: return "preceding";
            "preceding".to_string()
         }
         // Java: case 9:
         9 => {
            // Java: return "attribute";
            "attribute".to_string()
         }
         // Java: case 10:
         10 => {
            // Java: return "namespace";
            "namespace".to_string()
         }
         // Java: case 11:
         11 => {
            // Java: return "self";
            "self".to_string()
         }
         // Java: case 12:
         12 => {
            // Java: return "descendant-or-self";
            "descendant-or-self".to_string()
         }
         // Java: case 13:
         13 => {
            // Java: return "ancestor-or-self";
            "ancestor-or-self".to_string()
         }
         // Java: default:
         _ => {
            // Java: throw new JaxenRuntimeException("Illegal Axis Number");
            panic!("Illegal Axis Number")
         }
      }
    }

    // Java: static int lookup(String axisName)
    pub fn lookup_by_name(axisName: &str) -> i32 {
      // Java: if ("child".equals(axisName))
      if axisName == "child" {
         // Java: return 1;
         1
      }
      // Java: else if ("descendant".equals(axisName))
      else if axisName == "descendant" {
         // Java: return 2;
         2
      }
      // Java: else if ("parent".equals(axisName))
      else if axisName == "parent" {
         // Java: return 3;
         3
      }
      // Java: else if ("ancestor".equals(axisName))
      else if axisName == "ancestor" {
         // Java: return 4;
         4
      }
      // Java: else if ("following-sibling".equals(axisName))
      else if axisName == "following-sibling" {
         // Java: return 5;
         5
      }
      // Java: else if ("preceding-sibling".equals(axisName))
      else if axisName == "preceding-sibling" {
         // Java: return 6;
         6
      }
      // Java: else if ("following".equals(axisName))
      else if axisName == "following" {
         // Java: return 7;
         7
      }
      // Java: else if ("preceding".equals(axisName))
      else if axisName == "preceding" {
         // Java: return 8;
         8
      }
      // Java: else if ("attribute".equals(axisName))
      else if axisName == "attribute" {
         // Java: return 9;
         9
      }
      // Java: else if ("namespace".equals(axisName))
      else if axisName == "namespace" {
         // Java: return 10;
         10
      }
      // Java: else if ("self".equals(axisName))
      else if axisName == "self" {
         // Java: return 11;
         11
      }
      // Java: else if ("descendant-or-self".equals(axisName))
      else if axisName == "descendant-or-self" {
         // Java: return 12;
         12
      }
      // Java: else
      else {
         // Java: return "ancestor-or-self".equals(axisName) ? 13 : 0;
         if axisName == "ancestor-or-self" { 13 } else { 0 }
      }
    }
}
