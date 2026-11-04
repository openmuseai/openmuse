// java package cn.wps.et.ss.formula.parser;

// java public class ParserOption
#[derive(Clone, Copy)]
pub struct ParserOption {
    // java public final char DECIMAL_SEPARATOR;
    pub DECIMAL_SEPARATOR: char,
    // java public final char PARAMETER_SEPARATOR;
    pub PARAMETER_SEPARATOR: char,
    // java public final char UNION_SEPARATOR;
    pub UNION_SEPARATOR: char,
    // java public final char CONST_ARRAY_SEPARATOR;
    pub CONST_ARRAY_SEPARATOR: char,
    // java public final char CONST_ARRAY_ROW_SEPARATOR;
    pub CONST_ARRAY_ROW_SEPARATOR: char,
}

impl ParserOption {
    fn new(
        decimal: char,
        parameter: char,
        union: char,
        array: char,
        array_row: char,
    ) -> Self {
        Self {
            DECIMAL_SEPARATOR: decimal,
            PARAMETER_SEPARATOR: parameter,
            UNION_SEPARATOR: union,
            CONST_ARRAY_SEPARATOR: array,
            CONST_ARRAY_ROW_SEPARATOR: array_row,
        }
    }

    // java public static ParserOption createOOXMLParserOption()
    pub fn createOOXMLParserOption() -> Self {
        // java new ParserOption('.', ',', ',', ',', ';')
        Self::new('.', ',', ',', ',', ';')
    }
}
