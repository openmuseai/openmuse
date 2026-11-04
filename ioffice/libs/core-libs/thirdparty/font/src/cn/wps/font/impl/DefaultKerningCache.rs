// Java: import java.util.Arrays
use super::KerningPairCache::KerningPairCache;

// Java field: byte LEFTMAXCOUNT
const LEFTMAXCOUNT: i32 = 52;
// Java field: byte[] LeftAnsi128Index
const LEFT_ANSI_128_INDEX: [i8; 128] = [
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,
    15,16,17,18,19,20,21,22,23,24,25,-1,-1,-1,-1,-1,
    -1,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,
    41,42,43,44,45,46,47,48,49,50,51,-1,-1,-1,-1,-1,
];
// Java field: byte RIGHTMAXCOUNT
const RIGHTMAXCOUNT: i32 = 54;
// Java field: byte[] RightAnsi128Index
const RIGHT_ANSI_128_INDEX: [i8; 128] = [
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,52,-1,53,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,
    15,16,17,18,19,20,21,22,23,24,25,-1,-1,-1,-1,-1,
    -1,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,
    41,42,43,44,45,46,47,48,49,50,51,-1,-1,-1,-1,-1,
];

// Java: public class DefaultKerningCache extends KerningPairCache
pub struct DefaultKerningCache {
    pub base: KerningPairCache,
}

// Java: public class DefaultKerningCache extends KerningPairCache
impl DefaultKerningCache {
    // Java: DefaultKerningCache()
    pub fn new() -> Self {
        // Java: super();
        DefaultKerningCache {
            base: KerningPairCache::new(),
        }
    }

    // Java: boolean allocateCache()
    pub fn allocateCache(&mut self) -> bool {
        // Java: if (this.kerningTable == null)
        if self.base.kerningTable.is_none() {
            // Java: try
            {
                // Java: this.kerningTable = new int[2808];
                // Java: Arrays.fill(this.kerningTable, Integer.MIN_VALUE);
                self.base.kerningTable = Some(vec![i32::MIN; 2808]);
            }
            // Java: catch (OutOfMemoryError var2)
            // In Rust, allocation failure panics by default
            {
                // Java: return false;
            }
        }
        // Java: return this.kerningTable != null;
        self.base.kerningTable.is_some()
    }

    // Java: int toIndex(char left, char right)
    pub fn toIndex(&self, left: u16, right: u16) -> i32 {
        // Java: if (right >= 0 && right < RightAnsi128Index.length && left >= 0 && left < LeftAnsi128Index.length)
        if (right as usize) < RIGHT_ANSI_128_INDEX.len() && (left as usize) < LEFT_ANSI_128_INDEX.len() {
            // Java: byte rightIndex = RightAnsi128Index[right];
            let right_index = RIGHT_ANSI_128_INDEX[right as usize];
            // Java: if (rightIndex != -1)
            if right_index != -1 {
                // Java: byte leftIndex = LeftAnsi128Index[left];
                let left_index = LEFT_ANSI_128_INDEX[left as usize];
                // Java: if (leftIndex != -1)
                if left_index != -1 {
                    // Java: return leftIndex * 54 + rightIndex;
                    return (left_index as i32) * RIGHTMAXCOUNT + (right_index as i32);
                }
            }
            // Java: return -1;
            -1
        }
        // Java: else
        else {
            // Java: return -1;
            -1
        }
    }
}
