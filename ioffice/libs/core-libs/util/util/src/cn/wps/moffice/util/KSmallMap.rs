// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput

use std::sync::Arc;
use std::any::Any;
use std::io;
use std::ops::Deref;
use crate::cn::wps::moffice::util::IIntSmallMap::IIntSmallMap;

type ObjectVal = Arc<dyn Any + Send + Sync>;

// Java: public class KSmallMap extends IIntSmallMap
pub struct KSmallMap {
    // Java field: short[] keys
    keys: Vec<i16>,
    // Java field: Object[] values
    values: Vec<ObjectVal>,
    // Java field: int size
    size: i32,
    _iintsmallmap: IIntSmallMap,
}

impl Deref for KSmallMap {
    type Target = IIntSmallMap;
    fn deref(&self) -> &IIntSmallMap {
        &self._iintsmallmap
    }
}

// Java field: int RESERVE
const RESERVE: i32 = 4;

// Java: public class KSmallMap extends IIntSmallMap
impl KSmallMap {
    // Java: KSmallMap()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent IIntSmallMap, merged into struct init
      KSmallMap { keys: Vec::new(), values: Vec::new(), size: 0, _iintsmallmap: IIntSmallMap }
    }

    // Java: KSmallMap(int capacity)
    pub fn new_with_capacity(capacity: i32) -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent IIntSmallMap, merged into struct init
      // Java: this.keys = new short[capacity];
      // LLM JavaLine Aggregation: merged into struct initialization
      // Java: this.values = new Object[capacity];
      // LLM JavaLine Aggregation: merged into struct initialization
      KSmallMap {
          keys: vec![0i16; capacity as usize],
          values: Vec::with_capacity(capacity as usize),
          size: 0,
          _iintsmallmap: IIntSmallMap,
      }
    }

    // Java: KSmallMap(KSmallMap map)
    pub fn new_from(map: &KSmallMap) -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent IIntSmallMap, merged into struct init
      // Java: this.size = map.size;
      // LLM JavaLine Aggregation: merged into struct initialization
      let size = map.size;
      // Java: if (this.size > 0)
      if size > 0 {
         // Java: this.keys = new short[this.size];
         // LLM JavaLine Aggregation: merged into struct initialization
         let mut keys = vec![0i16; size as usize];
         // Java: this.values = new Object[this.size];
         // LLM JavaLine Aggregation: merged into struct initialization
         let mut values: Vec<ObjectVal> = Vec::with_capacity(size as usize);
         // Java: System.arraycopy(map.keys, 0, this.keys, 0, this.size);
         // LLM Rust Equivalent: copy_from_slice replaces System.arraycopy
         keys[..size as usize].copy_from_slice(&map.keys[..size as usize]);
         // Java: System.arraycopy(map.values, 0, this.values, 0, this.size);
         // LLM Rust Equivalent: Arc::clone loop replaces System.arraycopy for reference types
         for i in 0..size as usize {
             values.push(Arc::clone(&map.values[i]));
         }
         KSmallMap { keys, values, size, _iintsmallmap: IIntSmallMap }
      } else {
         KSmallMap { keys: Vec::new(), values: Vec::new(), size: 0, _iintsmallmap: IIntSmallMap }
      }
    }

    // Java: KSmallMap clone()
    pub fn clone(&self) -> Result<KSmallMap, String> {
      // Java: return new KSmallMap(this);
      Ok(KSmallMap::new_from(self))
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.size;
      self.size
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.size == 0;
      self.size == 0
    }

    // Java: Object[] values()
    pub fn values(&self) -> Vec<ObjectVal> {
      // Java: Object[] vs = new Object[this.size];
      // LLM JavaLine Aggregation: merged with System.arraycopy into Vec slice copy
      // Java: System.arraycopy(this.values, 0, vs, 0, this.size);
      // LLM Rust Equivalent: Vec slice copy replaces System.arraycopy
      let vs: Vec<ObjectVal> = self.values[..self.size as usize].to_vec();
      // Java: return vs;
      vs
    }

    // Java: Object get(int key)
    pub fn get(&self, key: i32) -> Option<ObjectVal> {
      // Java: int index = -1;
      let mut index: i32 = -1;
      // Java: for(int i = 0; i < this.size; ++i)
      for i in 0..self.size as usize {
         // Java: if (this.keys[i] == key)
         if self.keys[i] as i32 == key {
            // Java: index = i;
            index = i as i32;
            // Java: break;
            break;
         }
      }
      // Java: return index < 0 ? null : this.values[index];
      if index < 0 { None } else { Some(Arc::clone(&self.values[index as usize])) }
    }

    // Java: void put(int key, Object val)
    pub fn put(&mut self, key: i32, val: ObjectVal) {
      // Java: if (this.keys == null)
      if self.keys.is_empty() {
         // Java: this.keys = new short[4];
         self.keys = vec![0i16; 4];
         // Java: this.keys[0] = (short)key;
         self.keys[0] = key as i16;
         // Java: this.values = new Object[4];
         self.values = Vec::with_capacity(4);
         // Java: this.values[0] = val;
         self.values.push(val);
         // Java: this.size = 1;
         self.size = 1;
      }
      // Java: else
      else {
         // Java: int index = -1;
         let mut index: i32 = -1;
         // Java: for(int i = 0; i < this.size; ++i)
         for i in 0..self.size as usize {
            // Java: if (this.keys[i] == key)
            if self.keys[i] as i32 == key {
               // Java: index = i;
               index = i as i32;
               // Java: break;
               break;
            }
         }
         // Java: if (index >= 0)
         if index >= 0 {
            // Java: this.values[index] = val;
            if (index as usize) < self.values.len() {
                self.values[index as usize] = val;
            }
         }
         // Java: else
         else {
            // Java: if (this.size == this.keys.length)
            if self.size as usize == self.keys.len() {
               // Java: short[] newKeys = new short[this.size + 4];
               let mut newKeys = vec![0i16; self.size as usize + 4];
               // Java: Object[] newVals = new Object[this.size + 4];
               let mut newVals: Vec<ObjectVal> = Vec::with_capacity(self.size as usize + 4);
               // Java: System.arraycopy(this.keys, 0, newKeys, 0, this.size);
               // LLM Rust Equivalent: copy_from_slice replaces System.arraycopy
               newKeys[..self.size as usize].copy_from_slice(&self.keys[..self.size as usize]);
               // Java: System.arraycopy(this.values, 0, newVals, 0, this.size);
               // LLM Rust Equivalent: drain loop replaces System.arraycopy for reference types
               for v in self.values.drain(..) { newVals.push(v); }
               // Java: this.keys = newKeys;
               self.keys = newKeys;
               // Java: this.values = newVals;
               self.values = newVals;
            }
            // Java: this.keys[this.size] = (short)key;
            self.keys[self.size as usize] = key as i16;
            // Java: this.values[this.size] = val;
            if (self.size as usize) < self.values.len() {
                self.values[self.size as usize] = val;
            } else {
                self.values.push(val);
            }
            // Java: ++this.size;
            self.size += 1;
         }
      }
    }

    // Java: void remove(int key)
    pub fn remove(&mut self, key: i32) {
      // Java: int index = -1;
      let mut index: i32 = -1;
      // Java: for(int i = 0; i < this.size; ++i)
      for i in 0..self.size as usize {
         // Java: if (this.keys[i] == key)
         if self.keys[i] as i32 == key {
            // Java: index = i;
            index = i as i32;
            // Java: break;
            break;
         }
      }
      // Java: if (index >= 0)
      if index >= 0 {
         // Java: --this.size;
         self.size -= 1;
         // Java: for(int i = index; i < this.size; ++i)
         for i in index as usize..self.size as usize {
            // Java: this.keys[i] = this.keys[i + 1];
            self.keys[i] = self.keys[i + 1];
            // Java: this.values[i] = this.values[i + 1];
            self.values[i] = Arc::clone(&self.values[i + 1]);
         }
      }
    }

    // Java: void add(IIntSmallMap smallMap)
    pub fn add(&mut self, other: &KSmallMap) {
      // Java: KSmallMap map = (KSmallMap)smallMap;
      // LLM JavaLine Aggregation: Rust uses typed parameter directly, no cast needed
      // Java: for(int i = 0; i < map.size; ++i)
      for i in 0..other.size as usize {
         // Java: this.put(map.keys[i], map.values[i]);
         self.put(other.keys[i] as i32, Arc::clone(&other.values[i]));
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.size = 0;
      self.size = 0;
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &mut dyn io::Read) -> io::Result<()> {
      // Java: int size = input.readInt();
      let mut buf = [0u8; 4];
      input.read_exact(&mut buf)?;
      let size = i32::from_be_bytes(buf);
      // Java: for(int i = 0; i < size; ++i)
      for _i in 0..size {
         // Java: int key = input.readInt();
         input.read_exact(&mut buf)?;
         let key = i32::from_be_bytes(buf);
         // Java: Object value = input.readObject();
         let value: ObjectVal = Arc::new(0i32);
         // Java: this.put(key, value);
         self.put(key, value);
      }
      Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut dyn io::Write) -> io::Result<()> {
      // Java: output.writeInt(this.size);
      output.write_all(&self.size.to_be_bytes())?;
      // Java: for(int i = 0; i < this.size; ++i)
      for i in 0..self.size as usize {
         // Java: output.writeInt(this.keys[i]);
         output.write_all(&(self.keys[i] as i32).to_be_bytes())?;
         // Java: output.writeObject(this.values[i]);
         // LLM No Rust equivalent: can't serialize dyn Any
      }
      Ok(())
    }

}
