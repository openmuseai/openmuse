use super::EnumMap::EnumMap;

// Java: public class StringEnumMap extends EnumMap
pub struct StringEnumMap<T> {
    pub(crate) inner: EnumMap<String, T>,
}

// Java: public class StringEnumMap extends EnumMap
impl<T> StringEnumMap<T> {
    // Java: StringEnumMap()
    pub fn new() -> Self {
      // Java: super();
      StringEnumMap { inner: EnumMap::new() }
    }

    pub fn put(&mut self, k: String, v: T) {
        self.inner.put(k, v);
    }

    pub fn get(&self, k: &String) -> Option<&T> {
        self.inner.get(k)
    }
}
