// Java: import java.util.Arrays
// Java: import java.util.HashMap
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use std::hash::{Hash, Hasher};

// java public class ReadOnlyArray
#[derive(Clone)]
pub struct ReadOnlyArray<T> {
    // java T[] data;
    pub(crate) data: Vec<T>,
}

// java public static class Builder
#[derive()]
pub struct Builder<E> {
    // java private HashMap<ReadOnlyArray<E>, ReadOnlyArray<E>> mPool = new HashMap();
    mPool: Vec<ReadOnlyArray<E>>,
    // java private ReadOnlyArray<E> mTemp = new ReadOnlyArray<E>();
    mTemp: ReadOnlyArray<E>,
}

// java public class ReadOnlyArray
impl<T> ReadOnlyArray<T> {
    // java public int size()
    pub fn size(&self) -> i32 {
      // java return this.data.length;
      return self.data.len() as i32;
    }

    // java public T get(int index)
    pub fn get__int(&self, index: i32) -> &T {
      // java return (T)this.data[index];
      return &self.data[index as usize];
    }

    // java public T[] get()
    pub fn get(&self) -> &[T] {
      // java return this.data;
      return &self.data;
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32
    where
      T: Hash,
    {
      // java return Arrays.hashCode(this.data);
      let mut hasher = std::collections::hash_map::DefaultHasher::new();
      self.data.hash(&mut hasher);
      return hasher.finish() as i32;
    }

    // java public boolean equals(Object obj)
    pub fn equals(&self, obj: &dyn Object) -> bool
    where
      T: PartialEq + 'static,
    {
      // java if (obj instanceof ReadOnlyArray)
      if obj.as_any().downcast_ref::<ReadOnlyArray<T>>().is_some() {
        // java ReadOnlyArray<?> array2 = (ReadOnlyArray)obj;
        let array2: &ReadOnlyArray<T> = obj.as_any().downcast_ref::<ReadOnlyArray<T>>().unwrap();
        // java return Arrays.equals(this.data, array2.data);
        return self.data == array2.data;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

}

// java public static class Builder
impl<E> Builder<E> {
    pub fn new() -> Self {
        Self { mPool: Vec::new(), mTemp: ReadOnlyArray { data: Vec::new() } }
    }

    // java public synchronized void clear()
    pub fn clear(&mut self) {
      // java this.mPool.clear();
      self.mPool.clear();
    }

    // java public synchronized ReadOnlyArray<E> obtain(E[] origin)
    pub fn obtain(&mut self, origin: &Vec<E>) -> ReadOnlyArray<E>
    where
      E: Clone + PartialEq,
    {
      // java this.mTemp.data = (E[])origin;
      self.mTemp.data = origin.clone();
      // java ReadOnlyArray<E> cached = (ReadOnlyArray)this.mPool.get(this.mTemp);
      let mut cached: Option<ReadOnlyArray<E>> = self.mPool.iter().find(|item| item.data == self.mTemp.data).cloned();
      // java if (cached == null)
      if cached.is_none() {
        // java cached = new ReadOnlyArray<E>();
        cached = Some(ReadOnlyArray { data: Vec::new() });
        // java cached.data = (E[])Arrays.copyOf(origin, origin.length);
        cached.as_mut().unwrap().data = origin.clone();
        // java this.mPool.put(cached, cached);
        self.mPool.push(cached.as_ref().unwrap().clone());
      }
      // java return cached;
      return cached.unwrap();
    }

}
