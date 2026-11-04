#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.kfc.api.stack.IDoubleStack
// Java: import cn.wps.kfc.impl.arrayList.WDoubleArrayList
use crate::kfc::r#impl::arrayList::WDoubleArrayList::WDoubleArrayList;
// Java: import java.io.Serializable

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 1;
// Java field: int DEFAULT_CAPACITY
pub const DEFAULT_CAPACITY: i32 = 10;

// Java: public class WDoubleArrayStack implements IDoubleStack, Serializable
pub struct WDoubleArrayStack {
    // Java field: WDoubleArrayList mDoubleArrayList
    pub(crate) mDoubleArrayList: WDoubleArrayList,
    // Java field: int DEFAULT_CAPACITY
}

// Java: public class WDoubleArrayStack implements IDoubleStack, Serializable
impl WDoubleArrayStack {
    // Java: WDoubleArrayStack()
    pub fn new() -> Self {
        // Java: this(10);
        Self::new_with_capacity(10)
    }

    // Java: WDoubleArrayStack(int capacity)
    pub fn new_with_capacity(capacity: i32) -> Self {
        // Java: super();
        // Java: this.mDoubleArrayList = new WDoubleArrayList(capacity);
        WDoubleArrayStack {
            mDoubleArrayList: WDoubleArrayList::new_with_capacity(capacity),
        }
    }

    // Java: WDoubleArrayStack(int capacity, double no_entry_value)
    pub fn new_with_capacity_and_entry(capacity: i32, no_entry_value: f64) -> Self {
        // Java: super();
        // Java: this.mDoubleArrayList = new WDoubleArrayList(capacity, no_entry_value);
        WDoubleArrayStack {
            mDoubleArrayList: WDoubleArrayList::new_with_capacity_and_entry(capacity, no_entry_value),
        }
    }

    // Java: WDoubleArrayStack(IDoubleStack stack)
    pub fn new_from_stack(stack: &WDoubleArrayStack) -> Self {
        // Java: super();
        // Java: if (stack instanceof WDoubleArrayStack)
        {
            // Java: WDoubleArrayStack array_stack = (WDoubleArrayStack)stack;
            // Java: this.mDoubleArrayList = new WDoubleArrayList(array_stack.mDoubleArrayList);
            WDoubleArrayStack {
                mDoubleArrayList: WDoubleArrayList::new_from_array(&stack.mDoubleArrayList.toArray()),
            }
        }
        // Java: else
        // Java: throw new UnsupportedOperationException("Only support WDoubleArrayStack");
    }

    // Java: double getNoEntryValue()
    pub fn getNoEntryValue(&self) -> f64 {
        // Java: return this.mDoubleArrayList.getNoEntryValue();
        self.mDoubleArrayList.getNoEntryValue()
    }

    // Java: void push(double val)
    pub fn push(&mut self, val: f64) {
        // Java: this.mDoubleArrayList.add(val);
        self.mDoubleArrayList.add(val);
    }

    // Java: double pop()
    pub fn pop(&mut self) -> f64 {
        // Java: return this.mDoubleArrayList.removeAt(this.mDoubleArrayList.size() - 1);
        self.mDoubleArrayList.removeAt(self.mDoubleArrayList.size() - 1)
    }

    // Java: double peek()
    pub fn peek(&self) -> f64 {
        // Java: return this.mDoubleArrayList.get(this.mDoubleArrayList.size() - 1);
        self.mDoubleArrayList.get(self.mDoubleArrayList.size() - 1)
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.mDoubleArrayList.size();
        self.mDoubleArrayList.size()
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mDoubleArrayList.clear();
        self.mDoubleArrayList.clear();
    }

    // Java: double[] toArray()
    pub fn toArray(&self) -> Vec<f64> {
        // Java: double[] retval = this.mDoubleArrayList.toArray();
        let mut retval = self.mDoubleArrayList.toArray();
        // Java: this.reverse(retval, 0, this.size());
        Self::reverse(&mut retval, 0, self.size());
        // Java: return retval;
        retval
    }

    // Java: void toArray(double[] dest)
    pub fn toArray_into(&mut self, dest: &mut [f64]) {
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
        // Java: this.mDoubleArrayList.toArray(dest, start, length);
        self.mDoubleArrayList.toArray_dest_range(dest, start, length);
        // Java: this.reverse(dest, 0, length);
        Self::reverse(dest, 0, length);
        // Java: if (dest.length > size)
        if dest.len() as i32 > size
        {
            // Java: dest[size] = this.mDoubleArrayList.getNoEntryValue();
            dest[size as usize] = self.mDoubleArrayList.getNoEntryValue();
        }
    }

    // Java: void reverse(double[] dest, int from, int to)
    fn reverse(dest: &mut [f64], from: i32, to: i32) {
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

    // Java: void swap(double[] dest, int i, int j)
    fn swap(dest: &mut [f64], i: i32, j: i32) {
        // Java: double tmp = dest[i];
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
        // Java: for(int i = this.mDoubleArrayList.size() - 1; i > 0; --i)
        let mut i = self.mDoubleArrayList.size() - 1;
        while i > 0
        {
            // Java: buf.append(this.mDoubleArrayList.get(i));
            buf.push_str(&self.mDoubleArrayList.get(i).to_string());
            // Java: buf.append(", ");
            buf.push_str(", ");
            i -= 1;
        }
        // Java: if (this.size() > 0)
        if self.size() > 0
        {
            // Java: buf.append(this.mDoubleArrayList.get(0));
            buf.push_str(&self.mDoubleArrayList.get(0).to_string());
        }
        // Java: buf.append("}");
        buf.push('}');
        // Java: return buf.toString();
        buf
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &WDoubleArrayStack) -> bool {
        // Java: if (this == o)
        // Java: return true;
        // Java: else if (o != null && this.getClass() == o.getClass())
        {
            // Java: WDoubleArrayStack that = (WDoubleArrayStack)o;
            // Java: return this.mDoubleArrayList.equals(that.mDoubleArrayList);
            self.mDoubleArrayList.equals(&o.mDoubleArrayList)
        }
        // Java: else
        // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.mDoubleArrayList.hashCode();
        self.mDoubleArrayList.hashCode()
    }
}
