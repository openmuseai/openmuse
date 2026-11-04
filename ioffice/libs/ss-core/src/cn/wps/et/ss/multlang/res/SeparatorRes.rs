// java public final class SeparatorRes {
#[derive(Clone,serde::Serialize,serde::Deserialize)]
pub struct SeparatorRes{
    // java public final char GROUP_SEPARATOR;
    pub GROUP_SEPARATOR:u16,
    // java public final char DECIMAL_SEPARATOR;
    pub DECIMAL_SEPARATOR:u16,
    // java public final char PARAMETER_SEPARATOR;
    pub PARAMETER_SEPARATOR:u16,
    // java public final char UNION_SEPARATOR;
    pub UNION_SEPARATOR:u16,
    // java public final char CONST_ARRAY_SEPARATOR;
    pub CONST_ARRAY_SEPARATOR:u16,
    // java public final char CONST_ARRAY_ROW_SEPARATOR;
    pub CONST_ARRAY_ROW_SEPARATOR:u16,
}
// java public static final SeparatorRes COMMON_SEPERATORS = new SeparatorRes(',', '.', ',', ',', ',', ';');
pub const COMMON_SEPERATORS:SeparatorRes=SeparatorRes::SeparatorRes(44,46,44,44,44,59);
// java public static final SeparatorRes AR_SEPERATORS = new SeparatorRes(',', '.', ';', ';', ',', ';');
pub const AR_SEPERATORS:SeparatorRes=SeparatorRes::SeparatorRes(44,46,59,59,44,59);
// java public static final SeparatorRes SPACE_GROUPER_SEPERATORS = new SeparatorRes(' ', ',', ';', ';', '\\', ';');
pub const SPACE_GROUPER_SEPERATORS:SeparatorRes=SeparatorRes::SeparatorRes(32,44,59,59,92,59);
// java public static final SeparatorRes DOT_GROUPER_SEPERATORS = new SeparatorRes('.', ',', ';', ';', '\\', ';');
pub const DOT_GROUPER_SEPERATORS:SeparatorRes=SeparatorRes::SeparatorRes(46,44,59,59,92,59);
// java public static final SeparatorRes QUOT_GROUPER_SEPERATORS = new SeparatorRes('\'', '.', ';', ';', ',', ';');
pub const QUOT_GROUPER_SEPERATORS:SeparatorRes=SeparatorRes::SeparatorRes(39,46,59,59,44,59);
impl SeparatorRes{
    // java private SeparatorRes(char groupSeparator, char decimalSeparator, char parameterSeparator, char unionSeparator, char constArraySeparator, char constArrayRowSeparator) {
    pub const fn SeparatorRes(groupSeparator:u16,decimalSeparator:u16,parameterSeparator:u16,unionSeparator:u16,constArraySeparator:u16,constArrayRowSeparator:u16)->Self{
        // java this.GROUP_SEPARATOR = groupSeparator;
        // java this.DECIMAL_SEPARATOR = decimalSeparator;
        // java this.PARAMETER_SEPARATOR = parameterSeparator;
        // java this.UNION_SEPARATOR = unionSeparator;
        // java this.CONST_ARRAY_SEPARATOR = constArraySeparator;
        // java this.CONST_ARRAY_ROW_SEPARATOR = constArrayRowSeparator;
        Self{GROUP_SEPARATOR:groupSeparator,DECIMAL_SEPARATOR:decimalSeparator,PARAMETER_SEPARATOR:parameterSeparator,UNION_SEPARATOR:unionSeparator,CONST_ARRAY_SEPARATOR:constArraySeparator,CONST_ARRAY_ROW_SEPARATOR:constArrayRowSeparator}
    }
}
