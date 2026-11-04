#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.language.impl.Ligature
use super::lang_impl::Ligature::Ligature;

// Java field: int[][] arabicTable
pub static ARABIC_TABLE: &[[i32; 2]] = &[
    [1536, 1791],
    [1872, 1919],
    [64336, 65023],
    [65136, 65279],
];

// Java field: boolean[] ZEN_WIDTH_WHEN_NO_ISOLATE_60
static ZEN_WIDTH_WHEN_NO_ISOLATE_60: &[bool] = &[
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, true, true, true, true, true, true, true, false, false, true,
    true, true, true, true, true, false, false, true, true, false, true, true, true, true, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

// Java: public class Arabic
pub struct Arabic;

// Java: public class Args
pub struct Args {
    // Java field: _desBuff: &[char]
    pub desBuff: Option<Vec<char>>,
    // Java field: int desPos
    pub desPos: usize,
    // Java field: int len
    pub len: usize,
    // Java field: _srcBuff: &[char]
    pub srcBuff: Option<Vec<char>>,
    // Java field: int srcPos
    pub srcPos: usize,
}

// Java: public class Args
impl Args {
    // Java: Args()
    pub fn new() -> Self {
        // Java: super();
        Self {
            desBuff: None,
            desPos: 0,
            len: 0,
            srcBuff: None,
            srcPos: 0,
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.srcBuff = null;
        self.srcBuff = None;
        // Java: this.desBuff = null;
        self.desBuff = None;
    }
}

// Java: public class Arabic
impl Arabic {
    // Java: Arabic()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    // Java: static boolean charIsArabic(char ch)
    pub fn charIsArabic(ch: char) -> bool {
        // Java: int nCh = ch;
        let n_ch = ch as i32;
        // Java: int i = 0;
        // Java: for(int size = arabicTable.length; i < size; ++i)
        for row in ARABIC_TABLE.iter()
        {
            // Java: _row: &[i32] = arabicTable[i];
            // Java: if (nCh >= row[0] && nCh <= row[1])
            if n_ch >= row[0] && n_ch <= row[1]
            {
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: static int getRenderGraphs(Args args)
    pub fn getRenderGraphs(args: &mut Args) -> i32 {
        // Java: return null != args && null != args.srcBuff && null != args.desBuff ? Ligature.getLigatureGraphs(args) : 1;
        if args.srcBuff.is_some() && args.desBuff.is_some() {
            Ligature::getLigatureGraphs(args)
        } else {
            1
        }
    }

    // Java: static boolean isZenWidthWhenNoIsolateChar(char ch)
    pub fn isZenWidthWhenNoIsolateChar(ch: char) -> bool {
        // Java: int height = '\uff00' & ch;
        let height = 0xFF00u32 & (ch as u32);
        // Java: if (height == 1536)
        if height == 1536
        {
            // Java: return ZEN_WIDTH_WHEN_NO_ISOLATE_60[ch & 255];
            let idx = (ch as u32 & 255) as usize;
            if idx < ZEN_WIDTH_WHEN_NO_ISOLATE_60.len() {
                return ZEN_WIDTH_WHEN_NO_ISOLATE_60[idx];
            }
            return false;
        }
        // Java: else
        else
        {
            // Java: return height == 1792 && (ch == 1809 || ch >= 1840 && ch < 1867);
            let ch_val = ch as u32;
            height == 1792 && (ch_val == 1809 || (ch_val >= 1840 && ch_val < 1867))
        }
    }
}
