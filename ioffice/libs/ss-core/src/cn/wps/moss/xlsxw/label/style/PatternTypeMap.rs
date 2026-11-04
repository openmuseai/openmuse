// java class PatternTypeMap {
pub(crate) struct PatternTypeMap;
impl PatternTypeMap{
    // java private static String[] typeList = new String[19];
    const typeList:[&'static str;19]=["none","solid","mediumGray","darkGray","lightGray","darkHorizontal","darkVertical","darkDown","darkUp","darkGrid","darkTrellis","lightHorizontal","lightVertical","lightDown","lightUp","lightGrid","lightTrellis","gray125","gray0625"];
    // java public static String getType(Short value) { return value >= 0 && value <= 18 ? typeList[value] : null; }
    pub fn getType(value:Option<i16>)->Option<&'static str>{let value=value.expect("NullPointerException");if value>=0&&value<=18{Some(Self::typeList[value as usize])}else{None}}
    // java static { typeList[0] = "none"; typeList[1] = "solid"; typeList[2] = "mediumGray";
    // java typeList[3] = "darkGray"; typeList[4] = "lightGray"; typeList[5] = "darkHorizontal";
    // java typeList[6] = "darkVertical"; typeList[7] = "darkDown"; typeList[8] = "darkUp";
    // java typeList[9] = "darkGrid"; typeList[10] = "darkTrellis"; typeList[11] = "lightHorizontal";
    // java typeList[12] = "lightVertical"; typeList[13] = "lightDown"; typeList[14] = "lightUp";
    // java typeList[15] = "lightGrid"; typeList[16] = "lightTrellis"; typeList[17] = "gray125"; typeList[18] = "gray0625"; }
}
