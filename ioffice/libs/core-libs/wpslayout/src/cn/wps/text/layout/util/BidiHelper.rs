// Java: import android.text.SpannableStringBuilder
// Java: import android.text.TextUtils

pub struct SpannableStringBuilder {
    chars: Vec<char>,
}

impl SpannableStringBuilder {
    pub fn new() -> Self {
        SpannableStringBuilder { chars: Vec::new() }
    }
    pub fn getChars(&self, start: i32, end: i32, dest: &mut Vec<char>, _destOff: i32) {
        for i in start..end {
            dest[(i - start) as usize] = self.chars[i as usize];
        }
    }
    pub fn subSequence(&self, _start: i32, _end: i32) -> SpannableStringBuilder {
        SpannableStringBuilder::new()
    }
    pub fn getSpans(&self, _start: i32, _end: i32) -> Vec<i32> {
        Vec::new()
    }
    pub fn clearSpans(&mut self) {}
    pub fn setSpan(&mut self, _span: i32, _start: i32, _end: i32, _flags: i32) {}
    pub fn replace(&mut self, _start: i32, _end: i32, _source: &SpannableStringBuilder) {}
}

// Java: public class BidiHelper
pub struct BidiHelper {
}

// Java: public class BidiHelper
impl BidiHelper {
    // Java: BidiHelper()
    pub fn new() -> Self {
      // Java: super();
      BidiHelper {}
    }

    // Java: static void updateText(SpannableStringBuilder source, int start, int end, boolean[] embed)
    pub fn updateText(source: &mut SpannableStringBuilder, start: i32, end: i32, embed: &[bool]) {
        // Java: int direction = -2;
        let _direction: i32 = -2;
        // Java: int length = end - start;
        let length = (end - start) as usize;
        // Java: char[] chars = new char[length];
        let mut chars: Vec<char> = vec!['\0'; length];
        // Java: source.getChars(start, end, chars, 0);
        source.getChars(start, end, &mut chars, 0);
        // Java: byte[] en = new byte[length];
        let mut en: Vec<i8> = vec![0; length];

        // Java: for(int i = 0; i < length; ++i)
        for i in 0..length
        {
            // Java: if (embed[i])
            if embed[i]
            {
                // Java: boolean rt = (new Bidi(String.valueOf(chars[i]), -1)).isRightToLeft();
                let rt = is_right_to_left(chars[i]);
                // Java: if (rt && chars[i] != '.')
                if rt && chars[i] != '.'
                {
                    // Java: en[i] = -1;
                    en[i] = -1;
                }
                // Java: else
                else
                {
                    // Java: en[i] = 0;
                    en[i] = 0;
                }
            }
            // Java: else
            else
            {
                // Java: en[i] = -2;
                en[i] = -2;
            }
        }

        // Java: Bidi bidi = new Bidi(chars, 0, en, 0, chars.length, direction);
        // Java: byte[] bytes = new byte[length];
        let mut bytes: Vec<u8> = vec![0; length];
        // Java: CharSequence[] charSequences = new CharSequence[length];
        let mut charSequences: Vec<SpannableStringBuilder> = Vec::with_capacity(length);
        for _ in 0..length {
            charSequences.push(SpannableStringBuilder::new());
        }
        // Java: int i = 0;
        let mut i: usize = 0;

        // Java: for(int j = start + i; i < length; ++j)
        let mut j = start as usize + i;
        while i < length
        {
            // Java: bytes[i] = (byte)bidi.getLevelAt(i);
            bytes[i] = 0;
            // Java: charSequences[i] = source.subSequence(j, j + 1);
            charSequences[i] = source.subSequence(j as i32, (j + 1) as i32);
            // Java: ++i;
            i += 1;
            j += 1;
        }

        // Java: reorderVisually(bytes, 0, charSequences, 0, length);
        Self::reorderVisually(&bytes, 0, &mut charSequences, 0, length as i32);

        // Java: for(i = 0; i < length; ++i)
        i = 0;
        while i < length
        {
            // Java: CharacterStyle[] characterStyles = (CharacterStyle[])((SpannableStringBuilder)charSequences[i]).getSpans(0, 1, CharacterStyle.class);
            let characterStyles = charSequences[i].getSpans(0, 1);
            // Java: ((SpannableStringBuilder)charSequences[i]).clearSpans();
            charSequences[i].clearSpans();

            // Java: for(CharacterStyle characterStyle : characterStyles)
            for characterStyle in characterStyles.iter()
            {
                // Java: ((SpannableStringBuilder)charSequences[i]).setSpan(CharacterStyle.wrap(characterStyle), 0, 1, 17);
                charSequences[i].setSpan(*characterStyle, 0, 1, 17);
            }

            // Java: source.replace(start + i, start + i + 1, charSequences[i]);
            source.replace(start + i as i32, start + i as i32 + 1, &charSequences[i]);
            i += 1;
        }

    }

    // Java: static int[] reorderVisually(byte[] levels, int levelStart, Object[] objects1, int objectStart, int count)
    pub fn reorderVisually(levels: &[u8], levelStart: i32, objects1: &mut Vec<SpannableStringBuilder>, objectStart: i32, count: i32) -> Vec<i32> {
        // Java: byte[] reorderLevels = new byte[count];
        let mut reorderLevels: Vec<u8> = vec![0; count as usize];
        // Java: System.arraycopy(levels, levelStart, reorderLevels, 0, count);
        for idx in 0..count as usize {
            reorderLevels[idx] = levels[levelStart as usize + idx];
        }
        // Java: int[] indexMap = reorderVisual(reorderLevels);
        let indexMap = Self::reorderVisual(&reorderLevels);
        // Java: Object[] temp1 = new Object[count];
        let mut temp1: Vec<SpannableStringBuilder> = Vec::with_capacity(count as usize);
        // Java: System.arraycopy(objects1, objectStart, temp1, 0, count);
        for idx in 0..count as usize {
            temp1.push(SpannableStringBuilder::new());
            std::mem::swap(&mut temp1[idx], &mut objects1[objectStart as usize + idx]);
        }

        // Java: for(int i = 0; i < count; ++i)
        for i in 0..count as usize
        {
            // Java: objects1[objectStart + i] = temp1[indexMap[i]];
            std::mem::swap(&mut objects1[objectStart as usize + i], &mut temp1[indexMap[i] as usize]);
        }

        // Java: return indexMap;
        indexMap
    }

    // Java: static void invertMap1(int[] indexMap, int[] desMap)
    pub fn invertMap1(indexMap: &[i32], desMap: &mut [i32]) {
        // Java: int i = 0;
        let mut i: usize = 0;

        // Java: for(int length = indexMap.length; i < length; desMap[indexMap[i]] = i++)
        let length = indexMap.len();
        while i < length
        {
            // Java: desMap[indexMap[i]] = i++;
            desMap[indexMap[i] as usize] = i as i32;
            i += 1;
        }

    }

    // Java: static int[] prepareReorder(byte[] levels, byte[] pMinLevel, byte[] pMaxLevel)
    pub fn prepareReorder(levels: &[u8], pMinLevel: &mut [u8], pMaxLevel: &mut [u8]) -> Vec<i32> {
        // Java: if (levels != null && levels.length > 0)
        if !levels.is_empty()
        {
            // Java: byte minLevel = 126;
            let mut minLevel: u8 = 126;
            // Java: byte maxLevel = 0;
            let mut maxLevel: u8 = 0;
            // Java: int start = levels.length;
            let mut start = levels.len() as i32;

            // Java: while(start > 0)
            while start > 0
            {
                // Java: --start;
                start -= 1;
                // Java: byte level = levels[start];
                let level = levels[start as usize];
                // Java: if (level < 0)
                // Note: level is u8 so never < 0 in Rust
                // Java: return null;

                // Java: if (level > 126)
                if level > 126
                {
                    // Java: return null;
                    return Vec::new();
                }

                // Java: if (level < minLevel)
                if level < minLevel
                {
                    // Java: minLevel = level;
                    minLevel = level;
                }

                // Java: if (level > maxLevel)
                if level > maxLevel
                {
                    // Java: maxLevel = level;
                    maxLevel = level;
                }
            }

            // Java: pMinLevel[0] = minLevel;
            pMinLevel[0] = minLevel;
            // Java: pMaxLevel[0] = maxLevel;
            pMaxLevel[0] = maxLevel;
            // Java: int[] indexMap = new int[levels.length];
            let mut indexMap: Vec<i32> = vec![0; levels.len()];

            // Java: for(int var7 = levels.length; var7 > 0; indexMap[var7] = var7)
            let mut var7 = levels.len() as i32;
            while var7 > 0
            {
                // Java: --var7;
                var7 -= 1;
                // Java: indexMap[var7] = var7;
                indexMap[var7 as usize] = var7;
            }

            // Java: return indexMap;
            return indexMap;
        }
        // Java: else
        else
        {
            // Java: return null;
            return Vec::new();
        }
    }

    // Java: static int[] reorderVisual(byte[] levels)
    pub fn reorderVisual(levels: &[u8]) -> Vec<i32> {
        // Java: byte[] aMinLevel = new byte[1];
        let mut aMinLevel: Vec<u8> = vec![0; 1];
        // Java: byte[] aMaxLevel = new byte[1];
        let mut aMaxLevel: Vec<u8> = vec![0; 1];
        // Java: int[] indexMap = prepareReorder(levels, aMinLevel, aMaxLevel);
        let indexMap = Self::prepareReorder(levels, &mut aMinLevel, &mut aMaxLevel);
        // Java: if (indexMap == null)
        if indexMap.is_empty()
        {
            // Java: return null;
            return Vec::new();
        }
        // Java: else
        else
        {
            // Java: byte minLevel = aMinLevel[0];
            let minLevel = aMinLevel[0];
            // Java: byte maxLevel = aMaxLevel[0];
            let mut maxLevel = aMaxLevel[0];
            // Java: if (minLevel == maxLevel && (minLevel & 1) == 0)
            if minLevel == maxLevel && (minLevel & 1) == 0
            {
                // Java: return indexMap;
                return indexMap;
            }
            // Java: else
            else
            {
                // Java: minLevel = (byte)(minLevel | 1);
                let minLevel = minLevel | 1;
                let mut indexMap = indexMap;

                // Java: do
                loop
                {
                    // Java: int start = 0;
                    let mut start: usize = 0;

                    // Java: label57:
                    // Java: while(true)
                    'label57: loop
                    {
                        // Java: while(start >= levels.length || levels[start] >= maxLevel)
                        while start >= levels.len() || levels[start] >= maxLevel
                        {
                            // Java: if (start >= levels.length)
                            if start >= levels.len()
                            {
                                // Java: break label57;
                                break 'label57;
                            }

                            // Java: int limit = start;
                            let mut limit = start;

                            // Java: do
                            loop
                            {
                                // Java: ++limit;
                                limit += 1;
                                // Java: while(limit < levels.length && levels[limit] >= maxLevel);
                                if !(limit < levels.len() && levels[limit] >= maxLevel) {
                                    break;
                                }
                            }

                            // Java: for(int end = limit - 1; start < end; --end)
                            let mut end = limit - 1;
                            while start < end
                            {
                                // Java: int temp = indexMap[start];
                                let temp = indexMap[start];
                                // Java: indexMap[start] = indexMap[end];
                                indexMap[start] = indexMap[end];
                                // Java: indexMap[end] = temp;
                                indexMap[end] = temp;
                                // Java: ++start;
                                start += 1;
                                end -= 1;
                            }

                            // Java: if (limit == levels.length)
                            if limit == levels.len()
                            {
                                // Java: break label57;
                                break 'label57;
                            }

                            // Java: start = limit + 1;
                            start = limit + 1;
                        }

                        // Java: ++start;
                        start += 1;
                    }

                    // Java: --maxLevel;
                    maxLevel -= 1;
                    // Java: while(maxLevel >= minLevel);
                    if maxLevel < minLevel {
                        break;
                    }
                }

                // Java: return indexMap;
                return indexMap;
            }
        }
    }

}

fn is_right_to_left(c: char) -> bool {
    let cp = c as u32;
    (0x0590..=0x05FF).contains(&cp) ||
    (0x0600..=0x06FF).contains(&cp) ||
    (0xFB50..=0xFDFF).contains(&cp) ||
    (0xFE70..=0xFEFF).contains(&cp)
}
