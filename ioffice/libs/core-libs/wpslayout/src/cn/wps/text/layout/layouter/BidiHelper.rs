// Java: import java.util.Arrays
use std::sync::OnceLock;

// Java field: char[] BIDI_MIRRORING
static BIDI_MIRRORING: OnceLock<Vec<char>> = OnceLock::new();
// Java field: char[] BIDI_MIRRORING_PAIR
static BIDI_MIRRORING_PAIR: OnceLock<Vec<char>> = OnceLock::new();

fn get_bidi_mirroring() -> &'static Vec<char> {
    BIDI_MIRRORING.get_or_init(|| {
        vec!['(', ')', '<', '>', '[', ']', '{', '}', '\u{00ab}', '\u{00bb}',
             '\u{0f3a}', '\u{0f3b}', '\u{0f3c}', '\u{0f3d}', '\u{169b}', '\u{169c}',
             '\u{2039}', '\u{203a}', '\u{2045}', '\u{2046}', '\u{207d}', '\u{207e}',
             '\u{208d}', '\u{208e}', '\u{2208}', '\u{2209}', '\u{220a}', '\u{220b}',
             '\u{220c}', '\u{220d}', '\u{2215}', '\u{221f}', '\u{2220}', '\u{2221}',
             '\u{2222}', '\u{2224}', '\u{223c}', '\u{223d}', '\u{2243}', '\u{2245}',
             '\u{224c}', '\u{2252}', '\u{2253}', '\u{2254}', '\u{2255}', '\u{2264}',
             '\u{2265}', '\u{2266}', '\u{2267}', '\u{2268}', '\u{2269}', '\u{226a}',
             '\u{226b}', '\u{226e}', '\u{226f}', '\u{2270}', '\u{2271}', '\u{2272}',
             '\u{2273}', '\u{2274}', '\u{2275}', '\u{2276}', '\u{2277}', '\u{2278}',
             '\u{2279}', '\u{227a}', '\u{227b}', '\u{227c}', '\u{227d}', '\u{227e}',
             '\u{227f}', '\u{2280}', '\u{2281}', '\u{2282}', '\u{2283}', '\u{2284}',
             '\u{2285}', '\u{2286}', '\u{2287}', '\u{2288}', '\u{2289}', '\u{228a}',
             '\u{228b}', '\u{228f}', '\u{2290}', '\u{2291}', '\u{2292}', '\u{2298}',
             '\u{22a2}', '\u{22a3}', '\u{22a6}', '\u{22a8}', '\u{22a9}', '\u{22ab}',
             '\u{22b0}', '\u{22b1}', '\u{22b2}', '\u{22b3}', '\u{22b4}', '\u{22b5}',
             '\u{22b6}', '\u{22b7}', '\u{22b8}', '\u{22c9}', '\u{22ca}', '\u{22cb}',
             '\u{22cc}', '\u{22cd}', '\u{22d0}', '\u{22d1}', '\u{22d6}', '\u{22d7}',
             '\u{22d8}', '\u{22d9}', '\u{22da}', '\u{22db}', '\u{22dc}', '\u{22dd}',
             '\u{22de}', '\u{22df}', '\u{22e0}', '\u{22e1}', '\u{22e2}', '\u{22e3}',
             '\u{22e4}', '\u{22e5}', '\u{22e6}', '\u{22e7}', '\u{22e8}', '\u{22e9}',
             '\u{22ea}', '\u{22eb}', '\u{22ec}', '\u{22ed}', '\u{22f0}', '\u{22f1}',
             '\u{22f2}', '\u{22f3}', '\u{22f4}', '\u{22f6}', '\u{22f7}', '\u{22fa}',
             '\u{22fb}', '\u{22fc}', '\u{22fd}', '\u{22fe}', '\u{2308}', '\u{2309}',
             '\u{230a}', '\u{230b}', '\u{2329}', '\u{232a}', '\u{2768}', '\u{2769}',
             '\u{276a}', '\u{276b}', '\u{276c}', '\u{276d}', '\u{276e}', '\u{276f}',
             '\u{2770}', '\u{2771}', '\u{2772}', '\u{2773}', '\u{2774}', '\u{2775}',
             '\u{27c3}', '\u{27c4}', '\u{27c5}', '\u{27c6}', '\u{27c8}', '\u{27c9}',
             '\u{27cb}', '\u{27cd}', '\u{27d5}', '\u{27d6}', '\u{27dc}', '\u{27dd}',
             '\u{27de}', '\u{27e2}', '\u{27e3}', '\u{27e4}', '\u{27e5}', '\u{27e6}',
             '\u{27e7}', '\u{27e8}', '\u{27e9}', '\u{27ea}', '\u{27eb}', '\u{27ec}',
             '\u{27ed}', '\u{27ee}', '\u{27ef}', '\u{2983}', '\u{2984}', '\u{2985}',
             '\u{2986}', '\u{2987}', '\u{2988}', '\u{2989}', '\u{298a}', '\u{298b}',
             '\u{298c}', '\u{298d}', '\u{298e}', '\u{298f}', '\u{2990}', '\u{2991}',
             '\u{2992}', '\u{2993}', '\u{2994}', '\u{2995}', '\u{2996}', '\u{2997}',
             '\u{2998}']
    })
}

fn get_bidi_mirroring_pair() -> &'static Vec<char> {
    BIDI_MIRRORING_PAIR.get_or_init(|| {
        vec![')', '(', '>', '<', ']', '[', '}', '{', '\u{00bb}', '\u{00ab}',
             '\u{0f3b}', '\u{0f3a}', '\u{0f3d}', '\u{0f3c}', '\u{169c}', '\u{169b}',
             '\u{203a}', '\u{2039}', '\u{2046}', '\u{2045}', '\u{207e}', '\u{207d}',
             '\u{208e}', '\u{208d}', '\u{220b}', '\u{220c}', '\u{220d}', '\u{2208}',
             '\u{2209}', '\u{220a}', '\u{29f5}', '\u{2bfe}', '\u{2223}', '\u{29db}',
             '\u{29a0}', '\u{2aee}', '\u{223d}', '\u{223c}', '\u{22cd}', '\u{224c}',
             '\u{2245}', '\u{2253}', '\u{2252}', '\u{2255}', '\u{2254}', '\u{2265}',
             '\u{2264}', '\u{2267}', '\u{2266}', '\u{2269}', '\u{2268}', '\u{226b}',
             '\u{226a}', '\u{226f}', '\u{226e}', '\u{2271}', '\u{2270}', '\u{2273}',
             '\u{2272}', '\u{2275}', '\u{2274}', '\u{2277}', '\u{2276}', '\u{2279}',
             '\u{2278}', '\u{227b}', '\u{227a}', '\u{227d}', '\u{227c}', '\u{227f}',
             '\u{227e}', '\u{2281}', '\u{2280}', '\u{2283}', '\u{2282}', '\u{2285}',
             '\u{2284}', '\u{2287}', '\u{2286}', '\u{2289}', '\u{2288}', '\u{228b}',
             '\u{228a}', '\u{2290}', '\u{228f}', '\u{2292}', '\u{2291}', '\u{29b8}',
             '\u{22a3}', '\u{22a2}', '\u{2ade}', '\u{2ae4}', '\u{2ae3}', '\u{2ae5}',
             '\u{22b1}', '\u{22b0}', '\u{22b3}', '\u{22b2}', '\u{22b5}', '\u{22b4}',
             '\u{22b7}', '\u{22b6}', '\u{27dc}', '\u{22ca}', '\u{22c9}', '\u{22cc}',
             '\u{22cb}', '\u{2243}', '\u{22d1}', '\u{22d0}', '\u{22d7}', '\u{22d6}',
             '\u{22d9}', '\u{22d8}', '\u{22db}', '\u{22da}', '\u{22dd}', '\u{22dc}',
             '\u{22df}', '\u{22de}', '\u{22e1}', '\u{22e0}', '\u{22e3}', '\u{22e2}',
             '\u{22e5}', '\u{22e4}', '\u{22e7}', '\u{22e6}', '\u{22e9}', '\u{22e8}',
             '\u{22eb}', '\u{22ea}', '\u{22ed}', '\u{22ec}', '\u{22f1}', '\u{22f0}',
             '\u{22fa}', '\u{22fb}', '\u{22fc}', '\u{22fd}', '\u{22fe}', '\u{22f2}',
             '\u{22f3}', '\u{22f4}', '\u{22f6}', '\u{22f7}', '\u{2309}', '\u{2308}',
             '\u{230b}', '\u{230a}', '\u{232a}', '\u{2329}', '\u{2769}', '\u{2768}',
             '\u{276b}', '\u{276a}', '\u{276d}', '\u{276c}', '\u{276f}', '\u{276e}',
             '\u{2771}', '\u{2770}', '\u{2773}', '\u{2772}', '\u{2775}', '\u{2774}',
             '\u{27c4}', '\u{27c3}', '\u{27c6}', '\u{27c5}', '\u{27c9}', '\u{27c8}',
             '\u{27cd}', '\u{27cb}', '\u{27d6}', '\u{27d5}', '\u{22b8}', '\u{27de}',
             '\u{27dd}', '\u{27e3}', '\u{27e2}', '\u{27e5}', '\u{27e4}', '\u{27e7}',
             '\u{27e6}', '\u{27e9}', '\u{27e8}', '\u{27eb}', '\u{27ea}', '\u{27ed}',
             '\u{27ec}', '\u{27ef}', '\u{27ee}', '\u{2984}', '\u{2983}', '\u{2986}',
             '\u{2985}', '\u{2988}', '\u{2987}', '\u{298a}', '\u{2989}', '\u{298c}',
             '\u{298b}', '\u{2990}', '\u{298f}', '\u{298e}', '\u{298d}', '\u{2992}',
             '\u{2991}', '\u{2994}', '\u{2993}', '\u{2996}', '\u{2995}', '\u{2998}',
             '\u{2997}']
    })
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

    // Java: static char findBidiMirrorChar(char c)
    pub fn findBidiMirrorChar(c: char) -> char {
      // Java: int index = Arrays.binarySearch(BIDI_MIRRORING, 0, BIDI_MIRRORING.length, c);
      let mirroring = get_bidi_mirroring();
      let index = mirroring.binary_search(&c);
      // Java: return index >= 0 ? BIDI_MIRRORING_PAIR[index] : c;
      match index {
          Ok(i) => get_bidi_mirroring_pair()[i],
          Err(_) => c,
      }
    }

    // Java: static int[] prepareReorder(byte[] levels, byte[] pMinLevel, byte[] pMaxLevel)
    pub fn prepareReorder(levels: &[i8], pMinLevel: &mut i8, pMaxLevel: &mut i8) -> Option<Vec<i32>> {
      // Java: if (levels != null && levels.length > 0)
      if !levels.is_empty()
      {
         // Java: byte minLevel = 126;
         let mut minLevel: i8 = 126;
         // Java: byte maxLevel = 0;
         let mut maxLevel: i8 = 0;
         // Java: int start = levels.length;
         let mut start = levels.len();
         // Java: while(start > 0)
         while start > 0
         {
            // Java: --start;
            start -= 1;
            // Java: byte level = levels[start];
            let level = levels[start];
            // Java: if (level < 0)
            if level < 0
            {
               // Java: return null;
               return None;
            }
            // Java: if (level > 126)
            if level > 126
            {
               // Java: return null;
               return None;
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
         *pMinLevel = minLevel;
         // Java: pMaxLevel[0] = maxLevel;
         *pMaxLevel = maxLevel;
         // Java: int[] indexMap = new int[levels.length];
         let mut indexMap = vec![0i32; levels.len()];
         // Java: for(int var7 = levels.length; var7 > 0; indexMap[var7] = var7)
         let mut var7 = levels.len();
         while var7 > 0 {
            // Java: --var7;
            var7 -= 1;
            indexMap[var7] = var7 as i32;
         }
         // Java: return indexMap;
         Some(indexMap)
      }
      // Java: else
      else
      {
         // Java: return null;
         None
      }
    }

    // Java: static int[] reorderVisual(byte[] levels)
    pub fn reorderVisual(levels: &[i8]) -> Option<Vec<i32>> {
      // Java: byte[] aMinLevel = new byte[1];
      let mut aMinLevel: i8 = 0;
      // Java: byte[] aMaxLevel = new byte[1];
      let mut aMaxLevel: i8 = 0;
      // Java: int[] indexMap = prepareReorder(levels, aMinLevel, aMaxLevel);
      let mut indexMap = Self::prepareReorder(levels, &mut aMinLevel, &mut aMaxLevel);
      // Java: if (indexMap == null)
      if indexMap.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: byte minLevel = aMinLevel[0];
         let minLevel = aMinLevel;
         // Java: byte maxLevel = aMaxLevel[0];
         let mut maxLevel = aMaxLevel;
         // Java: if (minLevel == maxLevel && (minLevel & 1) == 0)
         if minLevel == maxLevel && (minLevel & 1) == 0
         {
            // Java: return indexMap;
            return indexMap;
         }
         // Java: else
         {
            // Java: minLevel = (byte)(minLevel | 1);
            let minLevel = minLevel | 1;
            // Java: do
            let map = indexMap.as_mut().unwrap();
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
                        if limit >= levels.len() || levels[limit] < maxLevel { break; }
                     }
                     // Java: for(int end = limit - 1; start < end; --end)
                     let mut end = limit - 1;
                     while start < end
                     {
                        // Java: int temp = indexMap[start];
                        let temp = map[start];
                        // Java: indexMap[start] = indexMap[end];
                        map[start] = map[end];
                        // Java: indexMap[end] = temp;
                        map[end] = temp;
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
               if maxLevel < minLevel { break; }
            }
            // Java: return indexMap;
            return indexMap;
         }
      }
    }

}
