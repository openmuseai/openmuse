#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.kfc.api.stack.ILongStack
// Java: import cn.wps.kfc.impl.arrayList.WLongArrayList
use crate::kfc::r#impl::arrayList::WLongArrayList::WLongArrayList;
// Java: import java.io.Serializable

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 1;

// Java: public class WLongArrayStack implements ILongStack, Serializable
pub struct WLongArrayStack {
    // Java field: WLongArrayList mLongArrayList
    pub(crate) mLongArrayList: WLongArrayList,
    // Java field: int DEFAULT_CAPACITY
}

pub const DEFAULT_CAPACITY: i32 = 10;

// Java: public class WLongArrayStack implements ILongStack, Serializable
impl WLongArrayStack {
    // Java: WLongArrayStack()
    pub fn new() -> Self {
        // Java: this(10);
        Self::new_with_capacity(10)
    }

    // Java: WLongArrayStack(int capacity)
    pub fn new_with_capacity(capacity: i32) -> Self {
        // Java: super();
        // Java: this.mLongArrayList = new WLongArrayList(capacity);
        WLongArrayStack {
            mLongArrayList: WLongArrayList::new_with_capacity(capacity),
        }
    }

    // Java: WLongArrayStack(int capacity, long no_entry_value)
    pub fn new_with_capacity_and_entry(capacity: i32, no_entry_value: i64) -> Self {
        // Java: super();
        // Java: this.mLongArrayList = new WLongArrayList(capacity, no_entry_value);
        WLongArrayStack {
            mLongArrayList: WLongArrayList::new_with_capacity_and_entry(capacity, no_entry_value),
        }
    }

    // Java: WLongArrayStack(ILongStack stack)
    pub fn new_from_stack(stack: &WLongArrayStack) -> Self {
        // Java: super();
        // Java: if (stack instanceof WLongArrayStack)
        {
            // Java: WLongArrayStack array_stack = (WLongArrayStack)stack;
            // Java: this.mLongArrayList = new WLongArrayList(array_stack.mLongArrayList);
            WLongArrayStack {
                mLongArrayList: WLongArrayList::new_from_array(&stack.mLongArrayList.toArray()),
            }
        }
        // Java: else
        // Java: throw new UnsupportedOperationException("Only support WLongArrayStack");
    }

    // Java: long getNoEntryValue()
    pub fn getNoEntryValue(&self) -> i64 {
        // Java: return this.mLongArrayList.getNoEntryValue();
        self.mLongArrayList.getNoEntryValue()
    }

    // Java: void push(long val)
    pub fn push(&mut self, val: i64) {
        // Java: this.mLongArrayList.add(val);
        self.mLongArrayList.add(val);
    }

    // Java: long pop()
    pub fn pop(&mut self) -> i64 {
        // Java: return this.mLongArrayList.removeAt(this.mLongArrayList.size() - 1);
        self.mLongArrayList.removeAt(self.mLongArrayList.size() - 1)
    }

    // Java: long peek()
    pub fn peek(&self) -> i64 {
        // Java: return this.mLongArrayList.get(this.mLongArrayList.size() - 1);
        self.mLongArrayList.get(self.mLongArrayList.size() - 1)
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.mLongArrayList.size();
        self.mLongArrayList.size()
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mLongArrayList.clear();
        self.mLongArrayList.clear();
    }

    // Java: long[] toArray()
    pub fn toArray(&self) -> Vec<i64> {
        // Java: long[] retval = this.mLongArrayList.toArray();
        let mut retval = self.mLongArrayList.toArray();
        // Java: this.reverse(retval, 0, this.size());
        Self::reverse(&mut retval, 0, self.size());
        // Java: return retval;
        retval
    }

    // Java: void toArray(long[] dest)
    pub fn toArray_into(&mut self, dest: &mut [i64]) {
        // Java: int size = this.size();
        let size = self.size();
        // Java: int start = size - dest.length;
        let mut start = size - dest.len() as i32;
        // Java: if (start < 0)
        if start < 0
        {
            // Java: start = 0;
            start = 0;
        }
        // Java: int length = Math.min(size, dest.length);
        let length = std::cmp::min(size, dest.len() as i32);
        // Java: this.mLongArrayList.toArray(dest, start, length);
        self.mLongArrayList.toArray_dest_range(dest, start, length);
        // Java: this.reverse(dest, 0, length);
        Self::reverse(dest, 0, length);
        // Java: if (dest.length > size)
        if dest.len() as i32 > size
        {
            // Java: dest[size] = this.mLongArrayList.getNoEntryValue();
            dest[size as usize] = self.mLongArrayList.getNoEntryValue();
        }
    }

    // Java: void reverse(long[] dest, int from, int to)
    fn reverse(dest: &mut [i64], from: i32, to: i32) {
        // Java: if (from != to)
        if from != to
        {
            // Java: if (from > to)
            if from > to
            {
                // Java: throw new IllegalArgumentException("from cannot be greater than to");
                panic!("from cannot be greater than to");
            }
            // Java: else
            {
                // Java: int i = from;
                let mut i = from;
                // Java: for(int j = to - 1; i < j; --j)
                let mut j = to - 1;
                while i < j
                {
                    // Java: this.swap(dest, i, j);
                    Self::swap(dest, i, j);
                    // Java: ++i;
                    i += 1;
                    j -= 1;
                }
            }
        }
    }

    // Java: void swap(long[] dest, int i, int j)
    fn swap(dest: &mut [i64], i: i32, j: i32) {
        // Java: long tmp = dest[i];
        let tmp = dest[i as usize];
        // Java: dest[i] = dest[j];
        dest[i as usize] = dest[j as usize];
        // Java: dest[j] = tmp;
        dest[j as usize] = tmp;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder buf = new StringBuilder("{");
        let mut buf = String::from("{");
        // Java: for(int i = this.mLongArrayList.size() - 1; i > 0; --i)
        let mut i = self.mLongArrayList.size() - 1;
        while i > 0
        {
            // Java: buf.append(this.mLongArrayList.get(i));
            buf.push_str(&self.mLongArrayList.get(i).to_string());
            // Java: buf.append(", ");
            buf.push_str(", ");
            i -= 1;
        }
        // Java: if (this.size() > 0)
        if self.size() > 0
        {
            // Java: buf.append(this.mLongArrayList.get(0));
            buf.push_str(&self.mLongArrayList.get(0).to_string());
        }
        // Java: buf.append("}");
        buf.push('}');
        // Java: return buf.toString();
        buf
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &WLongArrayStack) -> bool {
        // Java: if (this == o)
        // Java: return true;
        // Java: else if (o != null && this.getClass() == o.getClass())
        {
            // Java: WLongArrayStack that = (WLongArrayStack)o;
            // Java: return this.mLongArrayList.equals(that.mLongArrayList);
            self.mLongArrayList.equals(&o.mLongArrayList)
        }
        // Java: else
        // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.mLongArrayList.hashCode();
        self.mLongArrayList.hashCode()
    }
}
