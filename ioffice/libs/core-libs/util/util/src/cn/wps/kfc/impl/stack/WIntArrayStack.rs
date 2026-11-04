#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.kfc.api.stack.IIntStack
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
use crate::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;
// Java: import java.io.Serializable

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 1;

// Java: public class WIntArrayStack implements IIntStack, Serializable
pub struct WIntArrayStack {
    // Java field: WIntArrayList mIntArrayList
    pub(crate) mIntArrayList: WIntArrayList,
    // Java field: int DEFAULT_CAPACITY
}

pub const DEFAULT_CAPACITY: i32 = 10;

// Java: public class WIntArrayStack implements IIntStack, Serializable
impl WIntArrayStack {
    // Java: WIntArrayStack()
    pub fn new() -> Self {
        // Java: this(10);
        Self::new_with_capacity(10)
    }

    // Java: WIntArrayStack(int capacity)
    pub fn new_with_capacity(capacity: i32) -> Self {
        // Java: super();
        // Java: this.mIntArrayList = new WIntArrayList(capacity);
        WIntArrayStack {
            mIntArrayList: WIntArrayList::new_with_capacity(capacity),
        }
    }

    // Java: WIntArrayStack(int capacity, int no_entry_value)
    pub fn new_with_capacity_and_entry(capacity: i32, no_entry_value: i32) -> Self {
        // Java: super();
        // Java: this.mIntArrayList = new WIntArrayList(capacity, no_entry_value);
        WIntArrayStack {
            mIntArrayList: WIntArrayList::new_with_capacity_and_entry(capacity, no_entry_value),
        }
    }

    // Java: WIntArrayStack(IIntStack stack)
    pub fn new_from_stack(stack: &WIntArrayStack) -> Self {
        // Java: super();
        // Java: if (stack instanceof WIntArrayStack)
        {
            // Java: WIntArrayStack array_stack = (WIntArrayStack)stack;
            // Java: this.mIntArrayList = new WIntArrayList(array_stack.mIntArrayList);
            WIntArrayStack {
                mIntArrayList: WIntArrayList::new_from_array(&stack.mIntArrayList.toArray()),
            }
        }
        // Java: else
        // Java: throw new UnsupportedOperationException("Only support WIntArrayStack");
    }

    // Java: int getNoEntryValue()
    pub fn getNoEntryValue(&self) -> i32 {
        // Java: return this.mIntArrayList.getNoEntryValue();
        self.mIntArrayList.getNoEntryValue()
    }

    // Java: void push(int val)
    pub fn push(&mut self, val: i32) {
        // Java: this.mIntArrayList.add(val);
        self.mIntArrayList.add(val);
    }

    // Java: int pop()
    pub fn pop(&mut self) -> i32 {
        // Java: return this.mIntArrayList.removeAt(this.mIntArrayList.size() - 1);
        self.mIntArrayList.removeAt(self.mIntArrayList.size() - 1)
    }

    // Java: int peek()
    pub fn peek(&self) -> i32 {
        // Java: return this.mIntArrayList.get(this.mIntArrayList.size() - 1);
        self.mIntArrayList.get(self.mIntArrayList.size() - 1)
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.mIntArrayList.size();
        self.mIntArrayList.size()
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mIntArrayList.clear();
        self.mIntArrayList.clear();
    }

    // Java: int[] toArray()
    pub fn toArray(&self) -> Vec<i32> {
        // Java: int[] retval = this.mIntArrayList.toArray();
        let mut retval = self.mIntArrayList.toArray();
        // Java: this.reverse(retval, 0, this.size());
        Self::reverse(&mut retval, 0, self.size());
        // Java: return retval;
        retval
    }

    // Java: void toArray(int[] dest)
    pub fn toArray_into(&mut self, dest: &mut [i32]) {
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
        // Java: this.mIntArrayList.toArray(dest, start, length);
        self.mIntArrayList.toArray_dest_range(dest, start, length);
        // Java: this.reverse(dest, 0, length);
        Self::reverse(dest, 0, length);
        // Java: if (dest.length > size)
        if dest.len() as i32 > size
        {
            // Java: dest[size] = this.mIntArrayList.getNoEntryValue();
            dest[size as usize] = self.mIntArrayList.getNoEntryValue();
        }
    }

    // Java: void reverse(int[] dest, int from, int to)
    fn reverse(dest: &mut [i32], from: i32, to: i32) {
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

    // Java: void swap(int[] dest, int i, int j)
    fn swap(dest: &mut [i32], i: i32, j: i32) {
        // Java: int tmp = dest[i];
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
        // Java: for(int i = this.mIntArrayList.size() - 1; i > 0; --i)
        let mut i = self.mIntArrayList.size() - 1;
        while i > 0
        {
            // Java: buf.append(this.mIntArrayList.get(i));
            buf.push_str(&self.mIntArrayList.get(i).to_string());
            // Java: buf.append(", ");
            buf.push_str(", ");
            i -= 1;
        }
        // Java: if (this.size() > 0)
        if self.size() > 0
        {
            // Java: buf.append(this.mIntArrayList.get(0));
            buf.push_str(&self.mIntArrayList.get(0).to_string());
        }
        // Java: buf.append("}");
        buf.push('}');
        // Java: return buf.toString();
        buf
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &WIntArrayStack) -> bool {
        // Java: if (this == o)
        // Java: return true;
        // Java: else if (o != null && this.getClass() == o.getClass())
        {
            // Java: WIntArrayStack that = (WIntArrayStack)o;
            // Java: return this.mIntArrayList.equals(that.mIntArrayList);
            self.mIntArrayList.equals(&o.mIntArrayList)
        }
        // Java: else
        // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.mIntArrayList.hashCode();
        self.mIntArrayList.hashCode()
    }
}
