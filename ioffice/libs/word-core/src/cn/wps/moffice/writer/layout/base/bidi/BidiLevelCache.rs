// Java: cn.wps.moffice.writer.layout.base.bidi.BidiLevelCache
#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::value::Toggle::Toggle;
use crate::cn::wps::moffice::writer::layout::base::bidi::BidiEmbeddingsCache::BidiEmbeddingsCache;

const L: u8 = 0;
const R: u8 = 1;
const AL: u8 = 2;
const EN: u8 = 3;
const ES: u8 = 4;
const ET: u8 = 5;
const AN: u8 = 6;
const CS: u8 = 7;
const NSM: u8 = 8;
const BN: u8 = 9;
const B: u8 = 10;
const S: u8 = 11;
const WS: u8 = 12;
const ON: u8 = 13;
const LRE: u8 = 14;
const LRO: u8 = 15;
const RLE: u8 = 16;
const RLO: u8 = 17;
const PDF: u8 = 18;

// java public class BidiLevelCache
pub struct BidiLevelCache {
    mParaIsRTL: bool,
    mToggle: Option<Toggle>,
    mLevel: Vec<u8>,
    mMirrored: Vec<bool>,
    mLevelOfCharBuff: Vec<i32>,
    mEmbeddings: Vec<i8>,
    mGraphIndex: i32,
    mBaseLevel: i32,
    mStartAt: i32,
    mStr: String,
    mEmbeddingscache: BidiEmbeddingsCache,
}

impl Default for BidiLevelCache {
    fn default() -> Self { Self::new() }
}

impl BidiLevelCache {
    pub fn new() -> Self {
        Self {
            mParaIsRTL: false,
            mToggle: None,
            mLevel: vec![0; 128],
            mMirrored: vec![false; 128],
            mLevelOfCharBuff: Vec::new(),
            mEmbeddings: vec![0; 128],
            mGraphIndex: 0,
            mBaseLevel: 0,
            mStartAt: 0,
            mStr: String::new(),
            mEmbeddingscache: BidiEmbeddingsCache::new(),
        }
    }

    pub fn clean(&mut self) {
        self.mLevelOfCharBuff.clear();
    }

    // java public void start(TypoPara, int graphIndex, int graphLength, boolean paraIsRTL, Toggle toggle, boolean needLROWhileToggleIsEmpty, Object langBidi)
    pub fn start(&mut self, graphsBuff: &TypoPara, graphIndex: i32, graphLength: i32, paraIsRTL: bool, toggle: Option<Toggle>, needLROWhileToggleIsEmpty: bool, langBidiIsNull: bool) {
        let mut mAddMark = false;
        let mut text = String::new();
        self.mParaIsRTL = paraIsRTL;
        self.mToggle = toggle;
        self.mGraphIndex = graphIndex;
        if toggle != Some(Toggle::ON) && toggle != Some(Toggle::OPPOSITE) {
            if toggle == Some(Toggle::OFF) {
                mAddMark = true;
                text.push('\u{200e}');
            } else if toggle.is_none() {
                mAddMark = true;
                text.push('\u{202d}');
            }
        } else {
            mAddMark = true;
            text.push('\u{200f}');
        }
        if graphLength > 0 {
            let mut buf = vec!['\0'; graphLength as usize];
            graphsBuff.getChars(graphIndex, graphLength, &mut buf, 0);
            for ch in buf {
                text.push(ch);
            }
        }
        self.mStr = text;
        let strLen = self.mStr.chars().count();
        if self.mEmbeddings.len() < strLen {
            self.mEmbeddings.resize(strLen, 0);
        }
        let chars: Vec<char> = self.mStr.chars().collect();
        self.calcEmbeddings(strLen, toggle);
        let embeddings = self.mEmbeddings[..strLen].to_vec();
        self.mLevelOfCharBuff = resolve_levels(&chars, &embeddings, paraIsRTL);
        self.fixLevels(toggle, paraIsRTL, langBidiIsNull);
        let _ = needLROWhileToggleIsEmpty;
        self.mBaseLevel = if self.mParaIsRTL { 1 } else { 0 };
        self.mStartAt = if mAddMark { 1 } else { 0 };
        if graphLength >= self.mLevel.len() as i32 {
            self.extendBuff(graphLength);
        }
        let mut bufPos = self.mStartAt as usize;
        let bufSize = self.mLevelOfCharBuff.len();
        let mut graphPos = 0usize;
        let graphLen = graphLength.max(0) as usize;
        while graphPos < graphLen && bufPos < bufSize {
            let mut level = self.mLevelOfCharBuff[bufPos] as i32;
            if level < self.mBaseLevel {
                level += 2;
            }
            self.mLevel[graphPos] = level as u8;
            if level % 2 == 0 {
                self.mMirrored[graphPos] = false;
            } else {
                let ch = chars.get(bufPos).copied().unwrap_or('\0');
                self.mMirrored[graphPos] = is_mirrored(ch);
            }
            bufPos += 1;
            graphPos += 1;
        }
    }

    fn calcEmbeddings(&mut self, strLen: usize, toggle: Option<Toggle>) {
        let mut fixEmbeddings: i8 = 0;
        if toggle == Some(Toggle::OPPOSITE) || toggle == Some(Toggle::ON) {
            fixEmbeddings = if self.mParaIsRTL { 3 } else { 2 };
        }
        let embedding = self.mEmbeddingscache.getEmbedding().wrapping_add(fixEmbeddings);
        for i in 0..strLen {
            self.mEmbeddings[i] = embedding;
        }
    }

    pub fn addEmbed(&mut self, graphIndex: i32, paraIsRTL: bool) {
        self.mGraphIndex = graphIndex;
        if self.mLevel.is_empty() || 1 >= self.mLevel.len() {
            self.extendBuff(1);
        }
        self.mLevel[0] = if paraIsRTL { 2 } else { 0 };
    }

    pub fn addMath(&mut self, graphIndex: i32, graphLength: i32, paraIsRTL: bool) {
        self.mGraphIndex = graphIndex;
        if self.mLevel.is_empty() || graphLength >= self.mLevel.len() as i32 {
            self.extendBuff(graphLength);
        }
        let level: u8 = if paraIsRTL { 6 } else { 0 };
        let n = graphLength.max(0) as usize;
        for i in 0..n.min(self.mLevel.len()) {
            self.mLevel[i] = level;
        }
    }

    fn fixLevels(&mut self, toggle: Option<Toggle>, paraIsRTL: bool, needFixPunch: bool) {
        let chars: Vec<char> = self.mStr.chars().collect();
        let strLength = chars.len();
        if self.mLevelOfCharBuff.len() < strLength {
            self.mLevelOfCharBuff.resize(strLength, 0);
        }
        if toggle != Some(Toggle::ON) && toggle != Some(Toggle::OPPOSITE) {
            for i in 0..strLength {
                let ch = chars[i];
                if self.mLevelOfCharBuff[i] % 2 != 0 && needFixToLTRCharWhileToggleEmpty(ch) {
                    self.mLevelOfCharBuff[i] += 1;
                }
            }
        } else {
            let mut numAndSlashOnly = true;
            let startAt = if toggle.is_none() { 0 } else { 1 };
            for i in startAt..strLength {
                let ch = chars[i];
                let orgLevel = self.mLevelOfCharBuff[i];
                let emb = self.mEmbeddings.get(i).copied().unwrap_or(0) as i32;
                self.mLevelOfCharBuff[i] = if orgLevel < emb {
                    if emb % 2 == orgLevel % 2 { emb } else { emb + 1 }
                } else {
                    orgLevel
                };
                if ch == '/' && numAndSlashOnly && self.mLevelOfCharBuff[i] % 2 == 0 {
                    self.mLevelOfCharBuff[i] -= 1;
                } else if self.mLevelOfCharBuff[i] % 2 != 0 {
                    numAndSlashOnly = true;
                } else if needFixPunch && ch == '-' && self.mLevelOfCharBuff[i] % 2 == 0 {
                    self.mLevelOfCharBuff[i] -= 1;
                } else if !is_java_digit(ch) {
                    numAndSlashOnly = false;
                }
            }
        }
        if strLength > 0 {
            let lastChar = chars[strLength - 1];
            if lastChar == '\r' || lastChar == '\u{000b}' || lastChar == '\u{0007}' || lastChar == '\u{000e}' {
                self.mLevelOfCharBuff[strLength - 1] = if paraIsRTL { 1 } else { 0 };
            }
        }
    }

    pub fn getBidiLevel(&self, graphIndex: i32) -> u8 {
        let offset = graphIndex - self.mGraphIndex;
        if offset >= 0 && (offset as usize) < self.mLevel.len() {
            self.mLevel[offset as usize]
        } else {
            0
        }
    }

    pub fn getSameLevelLength(&self, graphIndex: i32) -> i32 {
        let offset = graphIndex - self.mGraphIndex;
        if offset >= 0 && (offset as usize) < self.mLevel.len() {
            let offset = offset as usize;
            let mut count = 1i32;
            let curLevel = self.mLevel[offset];
            let curRunIsMirrored = self.mMirrored[offset];
            let mut i = offset + 1;
            while i < self.mLevel.len() && curLevel == self.mLevel[i] && curRunIsMirrored == self.mMirrored[i] {
                count += 1;
                i += 1;
            }
            count
        } else {
            1
        }
    }

    pub fn isMirrored(&self, graphIndex: i32) -> bool {
        let offset = graphIndex - self.mGraphIndex;
        if offset >= 0 && (offset as usize) < self.mMirrored.len() {
            self.mMirrored[offset as usize]
        } else {
            false
        }
    }

    fn extendBuff(&mut self, newFlagsSize: i32) {
        let oldLength = self.mLevel.len();
        let newLength = ((newFlagsSize as usize + 128 - 1) / 128) * 128;
        self.mLevel.resize(newLength.max(oldLength), 0);
        self.mMirrored.resize(newLength.max(oldLength), false);
    }

    pub fn initEmbeddings(&mut self, doc: &dyn Document, paraStart: i32, lineStart: i32) {
        self.mEmbeddingscache.init(doc, paraStart, lineStart);
    }

    pub fn setNextLineStart(&mut self, lineStart: i32) {
        self.mEmbeddingscache.setLineStart(lineStart);
    }

    pub fn processBidiChar(&mut self, c: char) {
        self.mEmbeddingscache.processChar(c);
    }
}

fn needFixToLTRCharWhileToggleEmpty(c: char) -> bool {
    matches!(c, ' ' | '*' | '+' | '.' | '=' | '\u{2193}') || is_mirrored(c)
}

fn is_java_digit(c: char) -> bool {
    c.is_ascii_digit() || bidi_class(c) == EN || bidi_class(c) == AN
}

fn is_mirrored(c: char) -> bool {
    matches!(c,
        '(' | ')' | '<' | '>' | '[' | ']' | '{' | '}' |
        '\u{00AB}' | '\u{00BB}' | '\u{2039}' | '\u{203A}' |
        '\u{2045}' | '\u{2046}' | '\u{207D}' | '\u{207E}' |
        '\u{208D}' | '\u{208E}' | '\u{2264}' | '\u{2265}' |
        '\u{2329}' | '\u{232A}' | '\u{3008}' | '\u{3009}' |
        '\u{300A}' | '\u{300B}' | '\u{300C}' | '\u{300D}' |
        '\u{300E}' | '\u{300F}' | '\u{3010}' | '\u{3011}' |
        '\u{FF08}' | '\u{FF09}' | '\u{FF1C}' | '\u{FF1E}' |
        '\u{FF3B}' | '\u{FF3D}' | '\u{FF5B}' | '\u{FF5D}' |
        '\u{FF62}' | '\u{FF63}'
    ) || (0xFE59..=0xFE5E).contains(&(c as u32))
}

fn bidi_class(c: char) -> u8 {
    match c {
        '\u{202A}' => LRE,
        '\u{202B}' => RLE,
        '\u{202C}' => PDF,
        '\u{202D}' => LRO,
        '\u{202E}' => RLO,
        '\u{200E}' => L,
        '\u{200F}' => R,
        '\u{0009}' | '\u{000B}' => S,
        '\u{000A}' | '\u{000D}' | '\u{001C}' | '\u{001D}' | '\u{001E}' | '\u{0085}' => B,
        '\u{000C}' | ' ' | '\u{1680}' => WS,
        '\u{00A0}' | '\u{202F}' => CS,
        '\u{2000}'..='\u{200A}' | '\u{2028}' | '\u{205F}' | '\u{3000}' => WS,
        '\u{00AD}' => BN,
        '\u{0000}'..='\u{0008}' | '\u{000E}'..='\u{001B}' | '\u{007F}'..='\u{0084}' | '\u{0086}'..='\u{009F}' => BN,
        '#' | '$' | '%' | '\u{00A2}'..='\u{00A5}' | '\u{00B0}' | '\u{00B1}' => ET,
        '+' | '-' | '/' => ES,
        ',' | '.' | ':' => CS,
        '0'..='9' | '\u{00B2}' | '\u{00B3}' | '\u{00B9}' => EN,
        '\u{0590}'..='\u{05FF}' => hebrew_class(c),
        '\u{0600}'..='\u{06FF}' | '\u{0750}'..='\u{077F}' | '\u{08A0}'..='\u{08FF}' | '\u{FB50}'..='\u{FDFF}' | '\u{FE70}'..='\u{FEFF}' => arabic_class(c),
        '\u{0700}'..='\u{074F}' => AL,
        _ => {
            let u = c as u32;
            if u <= 0x007E && !c.is_ascii_alphanumeric() && c != ' ' { ON }
            else if (0x0041..=0x005A).contains(&u) || (0x0061..=0x007A).contains(&u) { L }
            else if u >= 0x0080 { L }
            else { ON }
        }
    }
}

fn hebrew_class(c: char) -> u8 {
    match c as u32 {
        0x0591..=0x05BD | 0x05BF | 0x05C1 | 0x05C2 | 0x05C4 | 0x05C5 | 0x05C7 => NSM,
        0x05BE | 0x05C6 | 0x05F3 | 0x05F4 => ON,
        _ => R,
    }
}

fn arabic_class(c: char) -> u8 {
    match c as u32 {
        0x0660..=0x0669 => AN,
        0x06F0..=0x06F9 => EN,
        0x060C | 0x066C => CS,
        0x066A => ET,
        0x061B | 0x061F => ON,
        0x064B..=0x065F | 0x0670 | 0x06D6..=0x06ED => NSM,
        _ => AL,
    }
}

/// java.text.Bidi levels. Embeddings are the stack base (negative = override).
/// Explicit marks in the text still push and pop, so a leading LRO/LRM affects the run.
fn resolve_levels(text: &[char], embeddings: &[i8], default_rtl: bool) -> Vec<i32> {
    let n = text.len();
    if n == 0 {
        return Vec::new();
    }
    let mut cls = vec![0u8; n];
    let mut first_strong: Option<u8> = None;
    for i in 0..n {
        cls[i] = bidi_class(text[i]);
        if first_strong.is_none() && (cls[i] == L || cls[i] == R || cls[i] == AL) {
            first_strong = Some(cls[i]);
        }
    }
    let base: u8 = match first_strong {
        Some(L) => 0,
        Some(R) | Some(AL) => 1,
        _ => if default_rtl { 1 } else { 0 },
    };
    let emb0 = embeddings.first().copied().unwrap_or(0);
    let start_level = if emb0 == 0 { base } else { emb0.unsigned_abs() };
    let start_override = if emb0 < 0 { Some(if start_level % 2 == 0 { L } else { R }) } else { None };
    let mut stack: Vec<(u8, Option<u8>)> = vec![(start_level, start_override)];
    let mut levels = vec![start_level as i32; n];
    for i in 0..n {
        let (level, over) = *stack.last().unwrap();
        match cls[i] {
            LRE | RLE | LRO | RLO => {
                let rtl = cls[i] == RLE || cls[i] == RLO;
                let next = if rtl { (level + 1) | 1 } else { (level + 2) & !1 };
                if next <= 125 && stack.len() < 62 {
                    let ov = if cls[i] == LRO { Some(L) } else if cls[i] == RLO { Some(R) } else { None };
                    stack.push((next, ov));
                }
                cls[i] = BN;
                levels[i] = level as i32;
            }
            PDF => {
                if stack.len() > 1 {
                    stack.pop();
                }
                cls[i] = BN;
                levels[i] = stack.last().unwrap().0 as i32;
            }
            _ => {
                levels[i] = level as i32;
                if let Some(dir) = over {
                    cls[i] = dir;
                }
            }
        }
    }
    let sor = if base % 2 == 0 { L } else { R };
    let mut prev: u8 = sor;
    let mut prev_strong: u8 = sor;
    let mut last_strong_al = false;
    for i in 0..n {
        if cls[i] == BN {
            continue;
        }
        if cls[i] == NSM {
            cls[i] = prev;
        }
        prev = cls[i];
        if cls[i] == EN && last_strong_al {
            cls[i] = AN;
        }
        if cls[i] == AL {
            cls[i] = R;
            last_strong_al = true;
            prev_strong = R;
        } else if cls[i] == L || cls[i] == R {
            last_strong_al = false;
            prev_strong = cls[i];
        }
    }
    for i in 0..n {
        if cls[i] == ES {
            let left = prev_non_bn(&cls, i);
            let right = next_non_bn(&cls, i);
            cls[i] = if left == EN && right == EN { EN } else { ON };
        } else if cls[i] == CS {
            let left = prev_non_bn(&cls, i);
            let right = next_non_bn(&cls, i);
            cls[i] = if left == right && (left == EN || left == AN) { left } else { ON };
        }
    }
    for i in 0..n {
        if cls[i] == ET {
            let mut j = i;
            while j > 0 && (cls[j - 1] == ET || cls[j - 1] == BN) { j -= 1; }
            let left = if j > 0 { cls[j - 1] } else { ON };
            let mut k = i;
            while k + 1 < n && (cls[k + 1] == ET || cls[k + 1] == BN) { k += 1; }
            let right = if k + 1 < n { cls[k + 1] } else { ON };
            if left == EN || right == EN {
                cls[i] = EN;
            } else {
                cls[i] = ON;
            }
        }
    }
    for i in 0..n {
        if cls[i] == ES || cls[i] == ET || cls[i] == CS {
            cls[i] = ON;
        }
    }
    let mut run_strong = sor;
    for i in 0..n {
        if cls[i] == L || cls[i] == R {
            run_strong = cls[i];
        } else if cls[i] == EN && run_strong == L {
            cls[i] = L;
        }
    }
    let mut i = 0;
    while i < n {
        if !is_neutral(cls[i]) {
            i += 1;
            continue;
        }
        let start = i;
        while i < n && is_neutral(cls[i]) { i += 1; }
        let lead = strong_before(&cls, start, sor);
        let trail = strong_after(&cls, i, if levels[start] % 2 == 0 { L } else { R });
        let repl = if lead == trail { lead } else if levels[start] % 2 == 0 { L } else { R };
        for c in &mut cls[start..i] {
            if *c != BN { *c = repl; }
        }
    }
    let _ = prev_strong;
    for i in 0..n {
        if cls[i] == BN { continue; }
        let even = levels[i] % 2 == 0;
        match cls[i] {
            R => if even { levels[i] += 1; },
            AN | EN => levels[i] += if even { 2 } else { 1 },
            L => if !even { levels[i] += 1; },
            _ => {}
        }
    }
    levels
}

fn is_neutral(c: u8) -> bool {
    matches!(c, B | S | WS | ON | BN)
}

fn prev_non_bn(cls: &[u8], i: usize) -> u8 {
    let mut j = i;
    while j > 0 {
        j -= 1;
        if cls[j] != BN { return cls[j]; }
    }
    ON
}

fn next_non_bn(cls: &[u8], i: usize) -> u8 {
    let mut j = i + 1;
    while j < cls.len() {
        if cls[j] != BN { return cls[j]; }
        j += 1;
    }
    ON
}

fn strong_before(cls: &[u8], i: usize, sor: u8) -> u8 {
    let mut j = i;
    while j > 0 {
        j -= 1;
        if cls[j] == L || cls[j] == R { return cls[j]; }
    }
    sor
}

fn strong_after(cls: &[u8], i: usize, eor: u8) -> u8 {
    let mut j = i;
    while j < cls.len() {
        if cls[j] == L || cls[j] == R { return cls[j]; }
        j += 1;
    }
    eor
}
