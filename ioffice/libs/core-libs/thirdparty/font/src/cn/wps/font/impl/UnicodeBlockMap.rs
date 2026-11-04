// Java: import cn.wps.moffice.define.FontName
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.List
// Java: import java.util.Map
use std::sync::OnceLock;

struct FontName;
impl FontName {
    fn isSymbolCodePoint(_c: i32) -> bool { false }
    fn isWingdingsCodePoint(_c: i32) -> bool { false }
    fn isWebdingsCodePoint(_c: i32) -> bool { false }
    fn isExtraCodePoint(_c: i32) -> bool { false }
    fn isKingsoftSignCodePoint(_c: i32) -> bool { false }
    fn isKingsoftExtraCodePoint(_c: i32) -> bool { false }
    fn isKingsoftConfettiCodePoint(_c: i32) -> bool { false }
    fn isKingsoftMarkCodePoint(_c: i32) -> bool { false }
}

// Java: public interface BlockItem
pub trait BlockItem: Send + Sync {
    // Java: boolean contains(int var1)
    fn contains(&self, c: i32) -> bool;
}

#[derive(Clone)]
// Java: public class RangeBlockItem
pub struct RangeBlockItem {
    // Java field: int bitIndex
    pub(crate) bitIndex: i32,
    // Java field: int[] ranges
    pub(crate) ranges: Vec<i32>,
}

// Java: public class RangeBlockItem
impl RangeBlockItem {
    // Java: RangeBlockItem(int bitIndex, int... rs)
    pub fn new(bit_index: i32, rs: &[i32]) -> Self {
        // Java: super();
        // Java: this.bitIndex = bitIndex;
        // Java: this.ranges = new int[rs.length];
        let mut ranges = vec![0i32; rs.len()];
        // Java: for(int i = 0; i < rs.length; ++i)
        for i in 0..rs.len() {
            // Java: this.ranges[i] = rs[i];
            ranges[i] = rs[i];
        }
        RangeBlockItem {
            bitIndex: bit_index,
            ranges,
        }
    }

    // Java: boolean contains(int c)
    pub fn contains(&self, c: i32) -> bool {
        // Java: int count = this.ranges.length / 2;
        let count = self.ranges.len() / 2;
        // Java: for(int i = 0; i < count; ++i)
        for i in 0..count {
            // Java: if (c >= this.ranges[i * 2] && c <= this.ranges[i * 2 + 1])
            if c >= self.ranges[i * 2] && c <= self.ranges[i * 2 + 1] {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }
}

impl BlockItem for RangeBlockItem {
    fn contains(&self, c: i32) -> bool {
        RangeBlockItem::contains(self, c)
    }
}

struct SpecialBlockItem {
    check_fn: fn(i32) -> bool,
}

impl BlockItem for SpecialBlockItem {
    fn contains(&self, c: i32) -> bool {
        let high = (c & 0xff00) >> 8;
        let mut c = c;
        if high == 0xf0 {
            c &= 255;
        } else if high != 0 {
            return false;
        }
        (self.check_fn)(c)
    }
}

// Java field: BlockItem[] s_range
static S_RANGE: OnceLock<Vec<Option<RangeBlockItem>>> = OnceLock::new();
// Java field: Map<String, BlockItem> s_specialRange
static S_SPECIAL_RANGE: OnceLock<HashMap<String, SpecialBlockItem>> = OnceLock::new();

fn get_s_range() -> &'static Vec<Option<RangeBlockItem>> {
    S_RANGE.get_or_init(|| {
        let mut range: Vec<Option<RangeBlockItem>> = vec![None; 128];
        // Java: static
        // Java: addToMap(0, 0, 127);
        add_to_map(&mut range, 0, &[0,127]);
        // Java: addToMap(1, 128, 255);
        add_to_map(&mut range, 1, &[128,255]);
        // Java: addToMap(2, 256, 383);
        add_to_map(&mut range, 2, &[256,383]);
        // Java: addToMap(3, 384, 591);
        add_to_map(&mut range, 3, &[384,591]);
        // Java: addToMap(4, 592, 687, 7424, 7551, 7552, 7615);
        add_to_map(&mut range, 4, &[592,687,7424,7551,7552,7615]);
        // Java: addToMap(5, 688, 767, 42752, 42783);
        add_to_map(&mut range, 5, &[688,767,42752,42783]);
        // Java: addToMap(6, 768, 879, 7616, 7679);
        add_to_map(&mut range, 6, &[768,879,7616,7679]);
        // Java: addToMap(7, 880, 1023);
        add_to_map(&mut range, 7, &[880,1023]);
        // Java: addToMap(8, 11392, 11519);
        add_to_map(&mut range, 8, &[11392,11519]);
        // Java: addToMap(9, 1024, 1279, 1280, 1327, 11744, 11775, 42560, 42655);
        add_to_map(&mut range, 9, &[1024,1279,1280,1327,11744,11775,42560,42655]);
        // Java: addToMap(10, 1328, 1423);
        add_to_map(&mut range, 10, &[1328,1423]);
        // Java: addToMap(11, 1424, 1535);
        add_to_map(&mut range, 11, &[1424,1535]);
        // Java: addToMap(12, 42240, 42559);
        add_to_map(&mut range, 12, &[42240,42559]);
        // Java: addToMap(13, 1536, 1791, 1872, 1919);
        add_to_map(&mut range, 13, &[1536,1791,1872,1919]);
        // Java: addToMap(14, 1984, 2047);
        add_to_map(&mut range, 14, &[1984,2047]);
        // Java: addToMap(15, 2304, 2431);
        add_to_map(&mut range, 15, &[2304,2431]);
        // Java: addToMap(16, 2432, 2559);
        add_to_map(&mut range, 16, &[2432,2559]);
        // Java: addToMap(17, 2560, 2687);
        add_to_map(&mut range, 17, &[2560,2687]);
        // Java: addToMap(18, 2688, 2815);
        add_to_map(&mut range, 18, &[2688,2815]);
        // Java: addToMap(19, 2816, 2943);
        add_to_map(&mut range, 19, &[2816,2943]);
        // Java: addToMap(20, 2944, 3071);
        add_to_map(&mut range, 20, &[2944,3071]);
        // Java: addToMap(21, 3072, 3199);
        add_to_map(&mut range, 21, &[3072,3199]);
        // Java: addToMap(22, 3200, 3327);
        add_to_map(&mut range, 22, &[3200,3327]);
        // Java: addToMap(23, 3328, 3455);
        add_to_map(&mut range, 23, &[3328,3455]);
        // Java: addToMap(24, 3584, 3711);
        add_to_map(&mut range, 24, &[3584,3711]);
        // Java: addToMap(25, 3712, 3839);
        add_to_map(&mut range, 25, &[3712,3839]);
        // Java: addToMap(26, 4256, 4351, 11520, 11567);
        add_to_map(&mut range, 26, &[4256,4351,11520,11567]);
        // Java: addToMap(27, 6912, 7039);
        add_to_map(&mut range, 27, &[6912,7039]);
        // Java: addToMap(28, 4352, 4607);
        add_to_map(&mut range, 28, &[4352,4607]);
        // Java: addToMap(29, 7680, 7935, 11360, 11391, 42784, 43007);
        add_to_map(&mut range, 29, &[7680,7935,11360,11391,42784,43007]);
        // Java: addToMap(30, 7936, 8191);
        add_to_map(&mut range, 30, &[7936,8191]);
        // Java: addToMap(31, 8192, 8303, 11776, 11903);
        add_to_map(&mut range, 31, &[8192,8303,11776,11903]);
        // Java: addToMap(32, 8304, 8351);
        add_to_map(&mut range, 32, &[8304,8351]);
        // Java: addToMap(33, 8352, 8399);
        add_to_map(&mut range, 33, &[8352,8399]);
        // Java: addToMap(34, 8400, 8447);
        add_to_map(&mut range, 34, &[8400,8447]);
        // Java: addToMap(35, 8448, 8527);
        add_to_map(&mut range, 35, &[8448,8527]);
        // Java: addToMap(36, 8528, 8591);
        add_to_map(&mut range, 36, &[8528,8591]);
        // Java: addToMap(37, 8592, 8703, 10224, 10239, 10496, 10623, 11008, 11263);
        add_to_map(&mut range, 37, &[8592,8703,10224,10239,10496,10623,11008,11263]);
        // Java: addToMap(38, 8704, 8959, 10752, 11007, 10176, 10223, 10624, 10751);
        add_to_map(&mut range, 38, &[8704,8959,10752,11007,10176,10223,10624,10751]);
        // Java: addToMap(39, 8960, 9215);
        add_to_map(&mut range, 39, &[8960,9215]);
        // Java: addToMap(40, 9216, 9279);
        add_to_map(&mut range, 40, &[9216,9279]);
        // Java: addToMap(41, 9280, 9311);
        add_to_map(&mut range, 41, &[9280,9311]);
        // Java: addToMap(42, 9312, 9471);
        add_to_map(&mut range, 42, &[9312,9471]);
        // Java: addToMap(43, 9472, 9599);
        add_to_map(&mut range, 43, &[9472,9599]);
        // Java: addToMap(44, 9600, 9631);
        add_to_map(&mut range, 44, &[9600,9631]);
        // Java: addToMap(45, 9632, 9727);
        add_to_map(&mut range, 45, &[9632,9727]);
        // Java: addToMap(46, 9728, 9983);
        add_to_map(&mut range, 46, &[9728,9983]);
        // Java: addToMap(47, 9984, 10175);
        add_to_map(&mut range, 47, &[9984,10175]);
        // Java: addToMap(48, 12288, 12351);
        add_to_map(&mut range, 48, &[12288,12351]);
        // Java: addToMap(49, 12352, 12447);
        add_to_map(&mut range, 49, &[12352,12447]);
        // Java: addToMap(50, 12448, 12543, 12784, 12799);
        add_to_map(&mut range, 50, &[12448,12543,12784,12799]);
        // Java: addToMap(51, 12544, 12591, 12704, 12735);
        add_to_map(&mut range, 51, &[12544,12591,12704,12735]);
        // Java: addToMap(52, 12592, 12687);
        add_to_map(&mut range, 52, &[12592,12687]);
        // Java: addToMap(53, 43072, 43135);
        add_to_map(&mut range, 53, &[43072,43135]);
        // Java: addToMap(54, 12800, 13055);
        add_to_map(&mut range, 54, &[12800,13055]);
        // Java: addToMap(55, 13056, 13311);
        add_to_map(&mut range, 55, &[13056,13311]);
        // Java: addToMap(56, 44032, 55215);
        add_to_map(&mut range, 56, &[44032,55215]);
        // Java: addToMap(57, 55296, 57343);
        add_to_map(&mut range, 57, &[55296,57343]);
        // Java: addToMap(58, 67840, 67871);
        add_to_map(&mut range, 58, &[67840,67871]);
        // Java: addToMap(59, 19968, 40959, 11904, 12031, 12032, 12255, 12272, 12287, 13312, 19903, 131072, 173791, 12688, 12703);
        add_to_map(&mut range, 59, &[19968,40959,11904,12031,12032,12255,12272,12287,13312,19903,131072,173791,12688,12703]);
        // Java: addToMap(60, 57344, 63743);
        add_to_map(&mut range, 60, &[57344,63743]);
        // Java: addToMap(61, 12736, 12783, 63744, 64255, 194560, 195103);
        add_to_map(&mut range, 61, &[12736,12783,63744,64255,194560,195103]);
        // Java: addToMap(62, 64256, 64335);
        add_to_map(&mut range, 62, &[64256,64335]);
        // Java: addToMap(63, 64336, 65023);
        add_to_map(&mut range, 63, &[64336,65023]);
        // Java: addToMap(64, 65056, 65071);
        add_to_map(&mut range, 64, &[65056,65071]);
        // Java: addToMap(65, 65040, 65055, 65072, 65103);
        add_to_map(&mut range, 65, &[65040,65055,65072,65103]);
        // Java: addToMap(66, 65104, 65135);
        add_to_map(&mut range, 66, &[65104,65135]);
        // Java: addToMap(67, 65136, 65279);
        add_to_map(&mut range, 67, &[65136,65279]);
        // Java: addToMap(68, 65280, 65519);
        add_to_map(&mut range, 68, &[65280,65519]);
        // Java: addToMap(69, 65520, 65535);
        add_to_map(&mut range, 69, &[65520,65535]);
        // Java: addToMap(70, 3840, 4095);
        add_to_map(&mut range, 70, &[3840,4095]);
        // Java: addToMap(71, 1792, 1871);
        add_to_map(&mut range, 71, &[1792,1871]);
        // Java: addToMap(72, 1920, 1983);
        add_to_map(&mut range, 72, &[1920,1983]);
        // Java: addToMap(73, 3456, 3583);
        add_to_map(&mut range, 73, &[3456,3583]);
        // Java: addToMap(74, 4096, 4255);
        add_to_map(&mut range, 74, &[4096,4255]);
        // Java: addToMap(75, 4608, 4991, 4992, 5023, 11648, 11743);
        add_to_map(&mut range, 75, &[4608,4991,4992,5023,11648,11743]);
        // Java: addToMap(76, 5024, 5119);
        add_to_map(&mut range, 76, &[5024,5119]);
        // Java: addToMap(77, 5120, 5759);
        add_to_map(&mut range, 77, &[5120,5759]);
        // Java: addToMap(78, 5760, 5791);
        add_to_map(&mut range, 78, &[5760,5791]);
        // Java: addToMap(79, 5792, 5887);
        add_to_map(&mut range, 79, &[5792,5887]);
        // Java: addToMap(80, 6016, 6143, 6624, 6655);
        add_to_map(&mut range, 80, &[6016,6143,6624,6655]);
        // Java: addToMap(81, 6144, 6319);
        add_to_map(&mut range, 81, &[6144,6319]);
        // Java: addToMap(82, 10240, 10495);
        add_to_map(&mut range, 82, &[10240,10495]);
        // Java: addToMap(83, 40960, 42127, 42128, 42191);
        add_to_map(&mut range, 83, &[40960,42127,42128,42191]);
        // Java: addToMap(84, 5888, 5919, 5920, 5951, 5952, 5983, 5984, 6015);
        add_to_map(&mut range, 84, &[5888,5919,5920,5951,5952,5983,5984,6015]);
        // Java: addToMap(85, 66304, 66351);
        add_to_map(&mut range, 85, &[66304,66351]);
        // Java: addToMap(86, 66352, 66383);
        add_to_map(&mut range, 86, &[66352,66383]);
        // Java: addToMap(87, 66560, 66639);
        add_to_map(&mut range, 87, &[66560,66639]);
        // Java: addToMap(88, 118784, 119039, 119040, 119295, 119296, 119375);
        add_to_map(&mut range, 88, &[118784,119039,119040,119295,119296,119375]);
        // Java: addToMap(89, 119808, 120831);
        add_to_map(&mut range, 89, &[119808,120831]);
        // Java: addToMap(90, 983040, 1048573, 1048576, 1114109);
        add_to_map(&mut range, 90, &[983040,1048573,1048576,1114109]);
        // Java: addToMap(91, 65024, 65039, 917760, 917999);
        add_to_map(&mut range, 91, &[65024,65039,917760,917999]);
        // Java: addToMap(92, 917504, 917631);
        add_to_map(&mut range, 92, &[917504,917631]);
        // Java: addToMap(93, 6400, 6479);
        add_to_map(&mut range, 93, &[6400,6479]);
        // Java: addToMap(94, 6480, 6527);
        add_to_map(&mut range, 94, &[6480,6527]);
        // Java: addToMap(95, 6528, 6623);
        add_to_map(&mut range, 95, &[6528,6623]);
        // Java: addToMap(96, 6656, 6687);
        add_to_map(&mut range, 96, &[6656,6687]);
        // Java: addToMap(97, 11264, 11359);
        add_to_map(&mut range, 97, &[11264,11359]);
        // Java: addToMap(98, 11568, 11647);
        add_to_map(&mut range, 98, &[11568,11647]);
        // Java: addToMap(99, 19904, 19967);
        add_to_map(&mut range, 99, &[19904,19967]);
        // Java: addToMap(100, 43008, 43055);
        add_to_map(&mut range, 100, &[43008,43055]);
        // Java: addToMap(101, 65536, 65663, 65664, 65791, 65792, 65855);
        add_to_map(&mut range, 101, &[65536,65663,65664,65791,65792,65855]);
        // Java: addToMap(102, 65856, 65935);
        add_to_map(&mut range, 102, &[65856,65935]);
        // Java: addToMap(103, 66432, 66463);
        add_to_map(&mut range, 103, &[66432,66463]);
        // Java: addToMap(104, 66464, 66527);
        add_to_map(&mut range, 104, &[66464,66527]);
        // Java: addToMap(105, 66640, 66687);
        add_to_map(&mut range, 105, &[66640,66687]);
        // Java: addToMap(106, 66688, 66735);
        add_to_map(&mut range, 106, &[66688,66735]);
        // Java: addToMap(107, 67584, 67647);
        add_to_map(&mut range, 107, &[67584,67647]);
        // Java: addToMap(108, 68096, 68191);
        add_to_map(&mut range, 108, &[68096,68191]);
        // Java: addToMap(109, 119552, 119647);
        add_to_map(&mut range, 109, &[119552,119647]);
        // Java: addToMap(110, 73728, 74751, 74752, 74879);
        add_to_map(&mut range, 110, &[73728,74751,74752,74879]);
        // Java: addToMap(111, 119648, 119679);
        add_to_map(&mut range, 111, &[119648,119679]);
        // Java: addToMap(112, 7040, 7103);
        add_to_map(&mut range, 112, &[7040,7103]);
        // Java: addToMap(113, 7168, 7247);
        add_to_map(&mut range, 113, &[7168,7247]);
        // Java: addToMap(114, 7248, 7295);
        add_to_map(&mut range, 114, &[7248,7295]);
        // Java: addToMap(115, 43136, 43231);
        add_to_map(&mut range, 115, &[43136,43231]);
        // Java: addToMap(116, 43264, 43311);
        add_to_map(&mut range, 116, &[43264,43311]);
        // Java: addToMap(117, 43312, 43359);
        add_to_map(&mut range, 117, &[43312,43359]);
        // Java: addToMap(118, 43520, 43615);
        add_to_map(&mut range, 118, &[43520,43615]);
        // Java: addToMap(119, 65936, 65999);
        add_to_map(&mut range, 119, &[65936,65999]);
        // Java: addToMap(120, 66000, 66047);
        add_to_map(&mut range, 120, &[66000,66047]);
        // Java: addToMap(121, 66208, 66271, 66176, 66207, 67872, 67903);
        add_to_map(&mut range, 121, &[66208,66271,66176,66207,67872,67903]);
        // Java: addToMap(122, 127024, 127135, 126976, 127023);
        add_to_map(&mut range, 122, &[127024,127135,126976,127023]);
        // Java: addToMap(123);
        add_to_map(&mut range, 123, &[]);
        // Java: addToMap(124);
        add_to_map(&mut range, 124, &[]);
        // Java: addToMap(125);
        add_to_map(&mut range, 125, &[]);
        // Java: addToMap(126);
        add_to_map(&mut range, 126, &[]);
        // Java: addToMap(127);
        add_to_map(&mut range, 127, &[]);
        range
    })
}

fn get_s_special_range() -> &'static HashMap<String, SpecialBlockItem> {
    S_SPECIAL_RANGE.get_or_init(|| {
        let mut map = HashMap::new();
        // Java: s_specialRange.put("Symbol", new BlockItem() {...});
        map.insert("Symbol".to_string(), SpecialBlockItem { check_fn: FontName::isSymbolCodePoint });
        // Java: s_specialRange.put("Wingdings", new BlockItem() {...});
        map.insert("Wingdings".to_string(), SpecialBlockItem { check_fn: FontName::isWingdingsCodePoint });
        // Java: s_specialRange.put("Webdings", new BlockItem() {...});
        map.insert("Webdings".to_string(), SpecialBlockItem { check_fn: FontName::isWebdingsCodePoint });
        // Java: s_specialRange.put("MT Extra", new BlockItem() {...});
        map.insert("MT Extra".to_string(), SpecialBlockItem { check_fn: FontName::isExtraCodePoint });
        // Java: s_specialRange.put("Kingsoft Sign", new BlockItem() {...});
        map.insert("Kingsoft Sign".to_string(), SpecialBlockItem { check_fn: FontName::isKingsoftSignCodePoint });
        // Java: s_specialRange.put("Kingsoft Extra", new BlockItem() {...});
        map.insert("Kingsoft Extra".to_string(), SpecialBlockItem { check_fn: FontName::isKingsoftExtraCodePoint });
        // Java: s_specialRange.put("Kingsoft Confetti", new BlockItem() {...});
        map.insert("Kingsoft Confetti".to_string(), SpecialBlockItem { check_fn: FontName::isKingsoftConfettiCodePoint });
        // Java: s_specialRange.put("Kingsoft Mark", new BlockItem() {...});
        map.insert("Kingsoft Mark".to_string(), SpecialBlockItem { check_fn: FontName::isKingsoftMarkCodePoint });
        map
    })
}

// Java: public class UnicodeBlockMap
pub struct UnicodeBlockMap {
}

// Java: public class UnicodeBlockMap
impl UnicodeBlockMap {
    // Java: UnicodeBlockMap()
    pub fn new() -> Self {
        // Java: super();
        UnicodeBlockMap {}
    }

    // Java: static void parseRangeToList(int range, int startIndex, int endIndex, List<BlockItem> list)
    pub fn parseRangeToList(range: i32, start_index: i32, mut end_index: i32, list: &mut Vec<RangeBlockItem>) {
        let s_range = get_s_range();
        // Java: if (endIndex >= s_range.length)
        if end_index as usize >= s_range.len() {
            // Java: endIndex = s_range.length - 1;
            end_index = s_range.len() as i32 - 1;
        }
        // Java: if (startIndex < endIndex)
        if start_index < end_index {
            // Java: int rangeLength = endIndex - startIndex + 1;
            let range_length = end_index - start_index + 1;
            // Java: if (rangeLength <= 32)
            if range_length <= 32 {
                // Java: for(int i = 0; i < rangeLength; ++i)
                for i in 0..range_length {
                    // Java: int mask = 1 << i;
                    let mask = 1 << i;
                    // Java: if ((range & mask) == mask)
                    if (range & mask) == mask {
                        // Java: BlockItem ub = s_range[startIndex + i];
                        let idx = (start_index + i) as usize;
                        // Java: if (ub != null)
                        if let Some(ref ub) = s_range[idx] {
                            // Java: list.add(ub);
                            list.push(ub.clone());
                        }
                    }
                }
            }
        }
    }

    // Java: static void parseRangeToList(List<BlockItem> list, String name)
    pub fn parseRangeToList_name(list: &mut Vec<RangeBlockItem>, name: &str) {
        let special_range = get_s_special_range();
        // Java: BlockItem item = (BlockItem)s_specialRange.get(name);
        let item = special_range.get(name);
        // Java: if (item != null)
        if let Some(_item) = item {
            // Java: list.add(item);
        }
    }

    // Java: static void addToMap(int bitIndex, int... rs)
    fn addToMap(range: &mut Vec<Option<RangeBlockItem>>, bit_index: usize, rs: &[i32]) {
        // Java: if (rs.length == 0)
        if rs.is_empty() {
            // Java: s_range[bitIndex] = null;
            range[bit_index] = None;
        }
        // Java: else
        else {
            // Java: s_range[bitIndex] = new RangeBlockItem(bitIndex, rs);
            range[bit_index] = Some(RangeBlockItem::new(bit_index as i32, rs));
        }
    }
}

fn add_to_map(range: &mut Vec<Option<RangeBlockItem>>, bit_index: usize, rs: &[i32]) {
    UnicodeBlockMap::addToMap(range, bit_index, rs);
}
