// Java: public class TokenTypes
pub struct TokenTypes;

impl TokenTypes {
    // Java field: int EOF
    pub const EOF: i32 = -1;
    // Java field: int SKIP
    pub const SKIP: i32 = -2;
    // Java field: int ERROR
    pub const ERROR: i32 = -3;
    // Java field: int EQUALS
    pub const EQUALS: i32 = 1;
    // Java field: int NOT_EQUALS
    pub const NOT_EQUALS: i32 = 2;
    // Java field: int LESS_THAN_SIGN
    pub const LESS_THAN_SIGN: i32 = 3;
    // Java field: int LESS_THAN_OR_EQUALS_SIGN
    pub const LESS_THAN_OR_EQUALS_SIGN: i32 = 4;
    // Java field: int GREATER_THAN_SIGN
    pub const GREATER_THAN_SIGN: i32 = 5;
    // Java field: int GREATER_THAN_OR_EQUALS_SIGN
    pub const GREATER_THAN_OR_EQUALS_SIGN: i32 = 6;
    // Java field: int PLUS
    pub const PLUS: i32 = 7;
    // Java field: int MINUS
    pub const MINUS: i32 = 8;
    // Java field: int STAR
    pub const STAR: i32 = 9;
    // Java field: int MOD
    pub const MOD: i32 = 10;
    // Java field: int DIV
    pub const DIV: i32 = 11;
    // Java field: int SLASH
    pub const SLASH: i32 = 12;
    // Java field: int DOUBLE_SLASH
    pub const DOUBLE_SLASH: i32 = 13;
    // Java field: int DOT
    pub const DOT: i32 = 14;
    // Java field: int DOT_DOT
    pub const DOT_DOT: i32 = 15;
    // Java field: int IDENTIFIER
    pub const IDENTIFIER: i32 = 16;
    // Java field: int AT
    pub const AT: i32 = 17;
    // Java field: int PIPE
    pub const PIPE: i32 = 18;
    // Java field: int COLON
    pub const COLON: i32 = 19;
    // Java field: int DOUBLE_COLON
    pub const DOUBLE_COLON: i32 = 20;
    // Java field: int LEFT_BRACKET
    pub const LEFT_BRACKET: i32 = 21;
    // Java field: int RIGHT_BRACKET
    pub const RIGHT_BRACKET: i32 = 22;
    // Java field: int LEFT_PAREN
    pub const LEFT_PAREN: i32 = 23;
    // Java field: int RIGHT_PAREN
    pub const RIGHT_PAREN: i32 = 24;
    // Java field: int DOLLAR
    pub const DOLLAR: i32 = 25;
    // Java field: int LITERAL
    pub const LITERAL: i32 = 26;
    // Java field: int AND
    pub const AND: i32 = 27;
    // Java field: int OR
    pub const OR: i32 = 28;
    // Java field: int DOUBLE
    pub const DOUBLE: i32 = 29;
    // Java field: int COMMA
    pub const COMMA: i32 = 30;
    // Java field: int STAR_OPERATOR
    pub const STAR_OPERATOR: i32 = 31;

    // Java: TokenTypes()
    pub fn new() -> Self {
      // Java: super();
      TokenTypes
    }

    // Java: static String getTokenText(int tokenType)
    pub fn getTokenText(tokenType: i32) -> String {
      // Java: switch (tokenType)
      match tokenType {
         // Java: case -3:
         // Java: return "(error)";
         -3 => "(error)".to_string(),
         // Java: case -2:
         // Java: return "(skip)";
         -2 => "(skip)".to_string(),
         // Java: case -1:
         // Java: return "(eof)";
         -1 => "(eof)".to_string(),
         // Java: case 0:
         // Java: return "Unrecognized token type: 0";
         0 => "Unrecognized token type: 0".to_string(),
         // Java: case 1:
         // Java: return "=";
         1 => "=".to_string(),
         // Java: case 2:
         // Java: return "!=";
         2 => "!=".to_string(),
         // Java: case 3:
         // Java: return "<";
         3 => "<".to_string(),
         // Java: case 4:
         // Java: return "<=";
         4 => "<=".to_string(),
         // Java: case 5:
         // Java: return ">";
         5 => ">".to_string(),
         // Java: case 6:
         // Java: return ">=";
         6 => ">=".to_string(),
         // Java: case 7:
         // Java: return "+";
         7 => "+".to_string(),
         // Java: case 8:
         // Java: return "-";
         8 => "-".to_string(),
         // Java: case 9:
         // Java: return "*";
         9 => "*".to_string(),
         // Java: case 10:
         // Java: return "mod";
         10 => "mod".to_string(),
         // Java: case 11:
         // Java: return "div";
         11 => "div".to_string(),
         // Java: case 12:
         // Java: return "/";
         12 => "/".to_string(),
         // Java: case 13:
         // Java: return "//";
         13 => "//".to_string(),
         // Java: case 14:
         // Java: return ".";
         14 => ".".to_string(),
         // Java: case 15:
         // Java: return "..";
         15 => "..".to_string(),
         // Java: case 16:
         // Java: return "(identifier)";
         16 => "(identifier)".to_string(),
         // Java: case 17:
         // Java: return "@";
         17 => "@".to_string(),
         // Java: case 18:
         // Java: return "|";
         18 => "|".to_string(),
         // Java: case 19:
         // Java: return ":";
         19 => ":".to_string(),
         // Java: case 20:
         // Java: return "::";
         20 => "::".to_string(),
         // Java: case 21:
         // Java: return "[";
         21 => "[".to_string(),
         // Java: case 22:
         // Java: return "]";
         22 => "]".to_string(),
         // Java: case 23:
         // Java: return "(";
         23 => "(".to_string(),
         // Java: case 24:
         // Java: return ")";
         24 => ")".to_string(),
         // Java: case 25:
         // Java: return "$";
         25 => "$".to_string(),
         // Java: case 26:
         // Java: return "(literal)";
         26 => "(literal)".to_string(),
         // Java: case 27:
         // Java: return "and";
         27 => "and".to_string(),
         // Java: case 28:
         // Java: return "or";
         28 => "or".to_string(),
         // Java: case 29:
         // Java: return "(double)";
         29 => "(double)".to_string(),
         // Java: case 30:
         // Java: return ",";
         30 => ",".to_string(),
         // Java: case 31:
         // Java: return "*";
         31 => "*".to_string(),
         // Java: default:
         // Java: return "Unrecognized token type: " + tokenType;
         _ => format!("Unrecognized token type: {}", tokenType),
      }
    }
}
