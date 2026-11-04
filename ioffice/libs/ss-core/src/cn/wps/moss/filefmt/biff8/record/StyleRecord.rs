// Original static naming method slice; binary record storage/IO is not claimed here.
// java public final class StyleRecord extends StandardRecord implements ReusableRecord {
pub struct StyleRecord;
impl StyleRecord{
    // java public static String getName(int buildStyleIndex) {
    pub fn getName(buildStyleIndex:i32)->&'static str{
        // java switch (buildStyleIndex) { case 0: return "常规"; case 1: case 2: default: return "";
        // java case 3: return "千位分隔"; case 4: return "货币"; case 5: return "百分比";
        // java case 6: return "千位分隔[0]"; case 7: return "货币[0]"; }
        match buildStyleIndex{0=>"常规",3=>"千位分隔",4=>"货币",5=>"百分比",6=>"千位分隔[0]",7=>"货币[0]",_=>""}
    }
}
