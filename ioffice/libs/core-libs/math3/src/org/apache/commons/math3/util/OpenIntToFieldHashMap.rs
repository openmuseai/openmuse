// Java: import java.io.IOException
// Java: import java.io.ObjectInputStream
// Java: import java.io.Serializable
// Java: import java.lang.reflect.Array
// Java: import java.util.ConcurrentModificationException
// Java: import java.util.NoSuchElementException
// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field;
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;

// Java field: byte FREE
pub(crate) const FREE: i8 = 0;
// Java field: byte FULL
pub(crate) const FULL: i8 = 0;
// Java field: byte REMOVED
pub(crate) const REMOVED: i8 = 0;
// Java field: long serialVersionUID
// Java field: float LOAD_FACTOR
pub(crate) const LOAD_FACTOR: f32 = 0.0;
// Java field: int DEFAULT_EXPECTED_SIZE
pub(crate) const DEFAULT_EXPECTED_SIZE: i32 = 0;
// Java field: int RESIZE_MULTIPLIER
pub(crate) const RESIZE_MULTIPLIER: i32 = 0;
// Java field: int PERTURB_SHIFT
pub(crate) const PERTURB_SHIFT: i32 = 0;

// Java: public class provides
pub struct provides {
}

// Java: public class OpenIntToFieldHashMap extends FieldElement implements Serializable
pub struct OpenIntToFieldHashMap {
    // Java field: Field<T> field
    field: Box<dyn std::any::Any>,
    // Java field: int[] keys
    keys: Option<Vec<i32>>,
    // Java field: T[] values
    pub(crate) values: Box<dyn std::any::Any>,
    // Java field: byte[] states
    states: Option<Vec<u8>>,
    // Java field: T missingEntries
    pub(crate) missingEntries: Box<dyn std::any::Any>,
    // Java field: int size
    pub(crate) size: i32,
    // Java field: int mask
    mask: i32,
    // Java field: int count
    pub(crate) count: i32,
}

// Java: public class Iterator
pub struct Iterator {
    // Java field: int referenceCount
    pub(crate) referenceCount: i32,
    // Java field: int current
    pub(crate) current: i32,
    // Java field: int next
    pub(crate) next: i32,
}

// Java: public class provides
impl provides {
}

// Java: public class OpenIntToFieldHashMap extends FieldElement implements Serializable
impl OpenIntToFieldHashMap {
    // Java: OpenIntToFieldHashMap(final Field<T>field)
    pub fn OpenIntToFieldHashMap(&self) {
        // Java: this(field, DEFAULT_EXPECTED_SIZE, field.getZero());
    }

    // Java: OpenIntToFieldHashMap(final Field<T>field, final T missingEntries)
    pub fn OpenIntToFieldHashMap_2(&self) {
        // Java: this(field,DEFAULT_EXPECTED_SIZE, missingEntries);
    }

    // Java: OpenIntToFieldHashMap(final Field<T> field, final int expectedSize)
    pub fn OpenIntToFieldHashMap_3(&self) {
        // Java: this(field,expectedSize, field.getZero());
    }

    // Java: OpenIntToFieldHashMap()
    pub fn OpenIntToFieldHashMap_4(&self) {
    }

    // Java: OpenIntToFieldHashMap(final OpenIntToFieldHashMap<T> source)
    pub fn OpenIntToFieldHashMap_5(&self) {
        // Java: field = source.field;
        // Java: final int length = source.keys.length;
        // Java: keys = new int[length];
        // Java: System.arraycopy(source.keys, 0, keys, 0, length);
        // Java: values = buildArray(length);
        // Java: System.arraycopy(source.values, 0, values, 0, length);
        // Java: states = new byte[length];
        // Java: System.arraycopy(source.states, 0, states, 0, length);
        // Java: missingEntries = source.missingEntries;
        // Java: size  = source.size;
        // Java: mask  = source.mask;
        // Java: count = source.count;
    }

    // Java: static int computeCapacity(final int expectedSize)
    pub fn computeCapacity(&self) -> i32 {
        // Java: if (expectedSize == 0)
        {
            // Java: return 1;
        }
        // Java: final int capacity   = (int) FastMath.ceil(expectedSize / LOAD_FACTOR);
        // Java: final int powerOfTwo = Integer.highestOneBit(capacity);
        // Java: if (powerOfTwo == capacity)
        {
            // Java: return capacity;
        }
        // Java: return nextPowerOfTwo(capacity);
        todo!()
    }

    // Java: static int nextPowerOfTwo(final int i)
    pub fn nextPowerOfTwo(&self) -> i32 {
        // Java: return Integer.highestOneBit(i) << 1;
        todo!()
    }

    // Java: T get(final int key)
    pub fn get(&self) -> Box<dyn std::any::Any> {
        // Java: final int hash  = hashOf(key);
        // Java: int index = hash & mask;
        // Java: if (containsKey(key, index))
        {
            // Java: return values[index];
        }
        // Java: if (states[index] == FREE)
        {
            // Java: return missingEntries;
        }
        // Java: int j = index;
        // Java: for (int perturb = perturb(hash); states[index] != FREE; perturb >>= PERTURB_SHIFT)
        {
            // Java: j = probe(perturb, j);
            // Java: index = j & mask;
            // Java: if (containsKey(key, index))
            {
                // Java: return values[index];
            }
        }
        // Java: return missingEntries;
        todo!()
    }

    // Java: boolean containsKey(final int key)
    pub fn containsKey(&self) -> bool {
        // Java: final int hash  = hashOf(key);
        // Java: int index = hash & mask;
        // Java: if (containsKey(key, index))
        {
            // Java: return true;
        }
        // Java: if (states[index] == FREE)
        {
            // Java: return false;
        }
        // Java: int j = index;
        // Java: for (int perturb = perturb(hash); states[index] != FREE; perturb >>= PERTURB_SHIFT)
        {
            // Java: j = probe(perturb, j);
            // Java: index = j & mask;
            // Java: if (containsKey(key, index))
            {
                // Java: return true;
            }
        }
        // Java: return false;
        todo!()
    }

    // Java: Iterator iterator()
    pub fn iterator(&self) -> Box<dyn std::any::Any> {
        // Java: return new Iterator();
        todo!()
    }

    // Java: static int perturb(final int hash)
    pub fn perturb(&self) -> i32 {
        // Java: return hash & 0x7fffffff;
        todo!()
    }

    // Java: int findInsertionIndex(final int key)
    pub fn findInsertionIndex(&self) -> i32 {
        // Java: return findInsertionIndex(keys, states, key, mask);
        todo!()
    }

    // Java: static int findInsertionIndex()
    pub fn findInsertionIndex_2(&self) -> i32 {
        todo!()
    }

    // Java: static int probe(final int perturb, final int j)
    pub fn probe(&self) -> i32 {
        // Java: return (j << 2) + j + perturb + 1;
        todo!()
    }

    // Java: static int changeIndexSign(final int index)
    pub fn changeIndexSign(&self) -> i32 {
        // Java: return -index - 1;
        todo!()
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return size;
        todo!()
    }

    // Java: T remove(final int key)
    pub fn remove(&self) -> Box<dyn std::any::Any> {
        // Java: final int hash  = hashOf(key);
        // Java: int index = hash & mask;
        // Java: if (containsKey(key, index))
        {
            // Java: return doRemove(index);
        }
        // Java: if (states[index] == FREE)
        {
            // Java: return missingEntries;
        }
        // Java: int j = index;
        // Java: for (int perturb = perturb(hash); states[index] != FREE; perturb >>= PERTURB_SHIFT)
        {
            // Java: j = probe(perturb, j);
            // Java: index = j & mask;
            // Java: if (containsKey(key, index))
            {
                // Java: return doRemove(index);
            }
        }
        // Java: return missingEntries;
        todo!()
    }

    // Java: boolean containsKey(final int key, final int index)
    pub fn containsKey_2(&self) -> bool {
        // Java: return (key != 0 || states[index] == FULL) && keys[index] == key;
        todo!()
    }

    // Java: T doRemove(int index)
    pub fn doRemove(&self) -> Box<dyn std::any::Any> {
        // Java: keys[index]   = 0;
        // Java: states[index] = REMOVED;
        // Java: final T previous = values[index];
        // Java: values[index] = missingEntries;
        // Java: --size;
        // Java: ++count;
        // Java: return previous;
        todo!()
    }

    // Java: T put(final int key, final T value)
    pub fn put(&self) -> Box<dyn std::any::Any> {
        // Java: int index = findInsertionIndex(key);
        // Java: T previous = missingEntries;
        // Java: boolean newMapping = true;
        // Java: if (index < 0)
        {
            // Java: index = changeIndexSign(index);
            // Java: previous = values[index];
            // Java: newMapping = false;
        }
        // Java: keys[index]   = key;
        // Java: states[index] = FULL;
        // Java: values[index] = value;
        // Java: if (newMapping)
        {
            // Java: ++size;
            // Java: if (shouldGrowTable())
            {
                // Java: growTable();
            }
            // Java: ++count;
        }
        // Java: return previous;
        todo!()
    }

    // Java: void growTable()
    pub fn growTable(&self) {
        // Java: final int oldLength      = states.length;
        // Java: final int[] oldKeys      = keys;
        // Java: final T[] oldValues = values;
        // Java: final byte[] oldStates   = states;
        // Java: final int newLength = RESIZE_MULTIPLIER * oldLength;
        // Java: final int[] newKeys = new int[newLength];
        // Java: final T[] newValues = buildArray(newLength);
        // Java: final byte[] newStates = new byte[newLength];
        // Java: final int newMask = newLength - 1;
        // Java: for (int i = 0; i < oldLength; ++i)
        {
            // Java: if (oldStates[i] == FULL)
            {
                // Java: final int key = oldKeys[i];
                // Java: final int index = findInsertionIndex(newKeys, newStates, key, newMask);
                // Java: newKeys[index]   = key;
                // Java: newValues[index] = oldValues[i];
                // Java: newStates[index] = FULL;
            }
        }
        // Java: mask   = newMask;
        // Java: keys   = newKeys;
        // Java: values = newValues;
        // Java: states = newStates;
    }

    // Java: boolean shouldGrowTable()
    pub fn shouldGrowTable(&self) -> bool {
        // Java: return size > (mask + 1) * LOAD_FACTOR;
        todo!()
    }

    // Java: static int hashOf(final int key)
    pub fn hashOf(&self) -> i32 {
        // Java: final int h = key ^ ((key >>> 20) ^ (key >>> 12));
        // Java: return h ^ (h >>> 7) ^ (h >>> 4);
        todo!()
    }

    // Java: void readObject(final ObjectInputStream stream)
    pub fn readObject(&self) {
    }

    // Java: T[] buildArray(final int length)
    pub fn buildArray(&self) -> Box<dyn std::any::Any> {
        // Java: return (T[]) Array.newInstance(field.getRuntimeClass(), length);
        todo!()
    }

}

// Java: public class Iterator
impl Iterator {
    // Java: Iterator()
    pub fn Iterator(&self) {
            // Java: // preserve the modification count of the map to detect concurrent modifications later
            // Java: referenceCount = count;
            // Java: // initialize current index
            // Java: next = -1;
            // Java: try
            {
                // Java: advance();
            }
                // Java: // ignored
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
            // Java: return next >= 0;
        todo!()
    }

    // Java: int key()
    pub fn key(&self) -> i32 {
        todo!()
    }

    // Java: T value()
    pub fn value(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void advance()
    pub fn advance(&self) {
    }

}

