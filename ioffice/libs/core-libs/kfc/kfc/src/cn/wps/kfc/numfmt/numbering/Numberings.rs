// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.HashMap
use std::collections::HashMap;
#[allow(unused_imports)] use std::sync::Mutex;

use super::INumbering::INumbering;
use super::NFCDefine;

type NumberingFactory = fn() -> Box<dyn INumbering>;

// Java field: HashMap<Integer, Class<? extends INumbering>> NumberingClasses
static NumberingClasses: std::sync::OnceLock<HashMap<i32, NumberingFactory>> = std::sync::OnceLock::new();
fn get_NumberingClasses() -> &'static HashMap<i32, NumberingFactory> {
    NumberingClasses.get_or_init(|| {
        let mut m: HashMap<i32, NumberingFactory> = HashMap::new();
        m.insert(5, || Box::new(super::Ordinal::Ordinal::new()));
        m.insert(6, || Box::new(super::CardinalText::CardinalText::new()));
        m.insert(7, || Box::new(super::OrdinalText::OrdinalText::new()));
        m.insert(8, || Box::new(super::Radix::Radix::new()));
        m.insert(22, || Box::new(super::DecimalZero::DecimalZero::new()));
        m.insert(32, || Box::new(super::IdeoZodiacTrad::IdeoZodiacTrad::new()));
        m.insert(33, || Box::new(super::TaiwaneseCounting::TaiwaneseCounting::new()));
        m.insert(42, || Box::new(super::KoreanCounting::KoreanCounting::new()));
        m.insert(45, || Box::new(super::Hebrew1::Hebrew1::new()));
        m.insert(47, || Box::new(super::Hebrew2::Hebrew2::new()));
        m.insert(56, || Box::new(super::Vietnamese::Vietnamese::new()));
        m.insert(57, || Box::new(super::Template::Template::new()));
        m
    })
}

// Java: public class Numberings
pub struct Numberings {
    // Java field: ArrayList<NumberingCache> mMultiNumberings
    mMultiNumberings: Option<Vec<NumberingCache>>,
    // Java field: HashMap<Integer, INumbering> mSoleNumberings
    mSoleNumberings: Option<HashMap<i32, Box<dyn INumbering>>>,
}

// Java: public class NumberingCache
pub struct NumberingCache {
    // Java field: int[] mNFCDefines
    pub(crate) mNFCDefines: Vec<i32>,
    // Java field: Class<? extends INumbering> mClass
    pub(crate) mClass: NumberingFactory,
    // Java field: INumbering mNumbering
    pub(crate) mNumbering: Option<Box<dyn INumbering>>,
}

// Java: public class Numberings
impl Numberings {
    // Java field: HashMap<Integer, Class<? extends INumbering>> NumberingClasses

    // Java: Numberings()
    pub fn new() -> Self {
      // Java: super();
      let mut multi = Vec::new();
      // Java: NumberingCache cache = new NumberingCache(Decimal.class, new int[]{0, 15, 23, 40, 43, 52, 55});
      let cache = NumberingCache::new(|| Box::new(super::Decimal::Decimal::new()), &[0, 15, 23, 40, 43, 52, 55]);
      // Java: this.mMultiNumberings.add(cache);
      multi.push(cache);
      // Java: cache = new NumberingCache(Alphabet.class, new int[]{4, 3, 9, 46, 48, 49, 50, 53, 58, 59});
      let cache = NumberingCache::new(|| Box::new(super::Alphabet::Alphabet::new()), &[4, 3, 9, 46, 48, 49, 50, 53, 58, 59]);
      // Java: this.mMultiNumberings.add(cache);
      multi.push(cache);
      // Java: cache = new NumberingCache(NumberShape.class, new int[]{10, 14, 17, 19, 36, 37, 41, 44, 51, 54});
      let cache = NumberingCache::new(|| Box::new(super::NumberShape::NumberShape::new()), &[10, 14, 17, 19, 36, 37, 41, 44, 51, 54]);
      // Java: this.mMultiNumberings.add(cache);
      multi.push(cache);
      // Java: cache = new NumberingCache(Series.class, new int[]{12, 13, 18, 20, 21, 26, 27, 28, 29, 30, 31});
      let cache = NumberingCache::new(|| Box::new(super::Series::Series::new()), &[12, 13, 18, 20, 21, 26, 27, 28, 29, 30, 31]);
      // Java: this.mMultiNumberings.add(cache);
      multi.push(cache);
      // Java: cache = new NumberingCache(Tongue.class, new int[]{34, 35, 38, 39});
      let cache = NumberingCache::new(|| Box::new(super::Tongue::Tongue::new()), &[34, 35, 38, 39]);
      // Java: this.mMultiNumberings.add(cache);
      multi.push(cache);
      // Java: cache = new NumberingCache(Roma.class, new int[]{1, 2});
      let cache = NumberingCache::new(|| Box::new(super::Roma::Roma::new()), &[1, 2]);
      // Java: this.mMultiNumberings.add(cache);
      multi.push(cache);
      // Java: cache = new NumberingCache(TongueJapanese.class, new int[]{11, 16});
      let cache = NumberingCache::new(|| Box::new(super::TongueJapanese::TongueJapanese::new()), &[11, 16]);
      // Java: this.mMultiNumberings.add(cache);
      multi.push(cache);
      // Java: cache = new NumberingCache(Period.class, new int[]{24, 25});
      let cache = NumberingCache::new(|| Box::new(super::Period::Period::new()), &[24, 25]);
      // Java: this.mMultiNumberings.add(cache);
      multi.push(cache);
      Numberings {
          mMultiNumberings: Some(multi),
          mSoleNumberings: Some(HashMap::new()),
      }
    }

    // Java: String getNumbering(long value, String switchArg)
    pub fn getNumbering_with_str(&mut self, value: i64, switch_arg: &str) -> String {
      // Java: int nfc = NFCDefine.getNfc(switchArg);
      let nfc = NFCDefine::NFCDefine::getNfc(switch_arg);
      // Java: return this.getNumbering(value, nfc);
      self.getNumbering_with_nfc(value, nfc)
    }

    // Java: String getNumbering(long value, int nfc)
    pub fn getNumbering_with_nfc(&mut self, value: i64, nfc: i32) -> String {
      // Java: INumbering numbering = this.getNumbering(nfc);
      let numbering = self.getNumbering(nfc);
      // Java: return numbering.getNumbering(value, nfc);
      numbering.map(|n| n.getNumbering(value, nfc)).unwrap_or_default()
    }

    // Java: INumbering getNumbering(int nfc)
    fn getNumbering(&mut self, nfc: i32) -> Option<&dyn INumbering> {
        // Java: if (nfc >= 60)
      if nfc >= 60 {
         // Java: assert false;
         debug_assert!(false);
         // Java: return null;
         return None;
      }
        // Java: else
      {
         // Java: INumbering numbering = null;
         // Java: if (NumberingClasses.containsKey(nfc))
         if get_NumberingClasses().contains_key(&nfc) {
            // Java: numbering = (INumbering)this.mSoleNumberings.get(nfc);
            let sole = self.mSoleNumberings.as_mut().unwrap();
                // Java: if (numbering == null)
            if !sole.contains_key(&nfc) {
               // Java: numbering = Numberings.NumberingCache.createInstance((Class)NumberingClasses.get(nfc));
               let factory = get_NumberingClasses().get(&nfc).unwrap();
               let numbering = factory();
               // Java: this.mSoleNumberings.put(nfc, numbering);
               sole.insert(nfc, numbering);
            }
            return sole.get(&nfc).map(|n| n.as_ref());
         }
            // Java: else
         else {
                // Java: for(NumberingCache cache : this.mMultiNumberings)
            let multi = self.mMultiNumberings.as_mut().unwrap();
            for i in 0..multi.len()
            {
               // Java: numbering = cache.getNumbering(nfc);
               if multi[i].contains(nfc) {
                    // Java: if (numbering != null)
                  // Java: break;
                  return multi[i].getNumbering(nfc);
               }
            }
         }
         // Java: return numbering;
         None
      }
    }

    // Java: INumbering getNumbering(String switchArg)
    pub fn getNumbering_by_str(&mut self, switch_arg: &str) -> Option<&dyn INumbering> {
      // Java: int nfc = NFCDefine.getNfc(switchArg);
      let nfc = NFCDefine::NFCDefine::getNfc(switch_arg);
      // Java: return this.getNumbering(nfc);
      self.getNumbering(nfc)
    }

}

// Java: public class NumberingCache
impl NumberingCache {
    // Java: NumberingCache(Class<? extends INumbering> classArg, int... values)
    pub fn new(class_arg: NumberingFactory, values: &[i32]) -> Self {
         // Java: super();
         // Java: assert values.length > 0;
         debug_assert!(!values.is_empty());
         // Java: this.mClass = classArg;
         // Java: this.mNFCDefines = new int[values.length];
         let mut nfc_defines = Vec::with_capacity(values.len());
        // Java: for(int i = 0; i < values.length; ++i)
        for i in 0..values.len()
         {
            // Java: this.mNFCDefines[i] = values[i];
            nfc_defines.push(values[i]);
         }
         // Java: Arrays.sort(this.mNFCDefines, 0, values.length);
         nfc_defines.sort();
         NumberingCache {
             mNFCDefines: nfc_defines,
             mClass: class_arg,
             mNumbering: None,
         }
    }

    // Java: INumbering getNumbering(int nfc)
    pub fn getNumbering(&mut self, nfc: i32) -> Option<&dyn INumbering> {
        // Java: if (!this.contains(nfc))
      if !self.contains(nfc) {
            // Java: return null;
            return None;
         }
        // Java: else
      else {
            // Java: if (this.mNumbering == null)
            if self.mNumbering.is_none() {
                // Java: this.mNumbering = createInstance(this.mClass);
                self.mNumbering = Self::createInstance(self.mClass);
            }
            // Java: return this.mNumbering;
            self.mNumbering.as_ref().map(|n| n.as_ref())
         }
    }

    // Java: static INumbering createInstance(Class<? extends INumbering> classbody)
    pub fn createInstance(classbody: NumberingFactory) -> Option<Box<dyn INumbering>> {
        // Java: try
      {
            // Java: return (INumbering)classbody.newInstance();
            return Some(classbody());
         }
        // Java: catch (InstantiationException e)
        // Java: e.printStackTrace();
        // Java: catch (IllegalAccessException e)
        // Java: e.printStackTrace();
         // Java: return null;
    }

    // Java: boolean contains(int nfc)
    pub fn contains(&self, nfc: i32) -> bool {
         // Java: return Arrays.binarySearch(this.mNFCDefines, nfc) >= 0;
         self.mNFCDefines.binary_search(&nfc).is_ok()
    }

}
