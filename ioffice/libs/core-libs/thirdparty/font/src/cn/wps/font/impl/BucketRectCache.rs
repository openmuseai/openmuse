// Java: import cn.wps.graphics.RectF
use crate::cn::wps::font::FreeTypeJNI::RectF;
// Java: import java.util.Arrays

// Java field: int BUCKETS
const BUCKETS: usize = 256;
// Java field: int MAX_NEXT
const MAX_NEXT: usize = 8;

// Java: public class BucketRectCache
pub struct BucketRectCache {
    // Java field: char[] chars
    chars: Vec<u16>,
    // Java field: RectF[] rects
    pub(crate) rects: Vec<Option<RectF>>,
}

// Java: public class BucketRectCache
impl BucketRectCache {
    // Java: BucketRectCache()
    pub fn new() -> Self {
        // Java: super();
        // Java: Arrays.fill(this.chars, '\u0000');
        BucketRectCache {
            chars: vec![0u16; BUCKETS],
            rects: (0..BUCKETS).map(|_| None).collect(),
        }
    }

    // Java: RectF findRect(char c)
    pub fn findRect(&self, c: u16) -> Option<&RectF> {
        // Java: int index = c % 256;
        let mut index = (c as usize) % BUCKETS;
        // Java: for(int tryTime = 0; tryTime < 8; ++tryTime)
        for _try_time in 0..MAX_NEXT {
            // Java: char curChar = this.chars[index];
            let cur_char = self.chars[index];
            // Java: if (0 == curChar)
            if cur_char == 0 {
                // Java: break;
                break;
            }
            // Java: if (curChar == c)
            if cur_char == c {
                // Java: return this.rects[index];
                return self.rects[index].as_ref();
            }
            // Java: ++index;
            index += 1;
            // Java: if (index >= 256)
            if index >= BUCKETS {
                // Java: index = 0;
                index = 0;
            }
        }
        // Java: return null;
        None
    }

    // Java: void addRect(char c, RectF rectInCache)
    pub fn addRect(&mut self, c: u16, rect_in_cache: &RectF) {
        // Java: RectF rect = new RectF(rectInCache);
        let rect = rect_in_cache.clone();
        // Java: int index = c % 256;
        let mut index = (c as usize) % BUCKETS;
        // Java: for(int tryTime = 0; tryTime < 8; ++tryTime)
        for _try_time in 0..MAX_NEXT {
            // Java: char curChar = this.chars[index];
            let cur_char = self.chars[index];
            // Java: if (0 == curChar)
            if cur_char == 0 {
                // Java: this.chars[index] = c;
                self.chars[index] = c;
                // Java: this.rects[index] = rect;
                self.rects[index] = Some(rect);
                // Java: break;
                return;
            }
            // Java: ++index;
            index += 1;
            // Java: if (index >= 256)
            if index >= BUCKETS {
                // Java: index = 0;
                index = 0;
            }
        }
        // Java: index = c % 256;
        index = (c as usize) % BUCKETS;
        // Java: this.chars[index] = c;
        self.chars[index] = c;
        // Java: this.rects[index] = rect;
        self.rects[index] = Some(rect);
    }
}
