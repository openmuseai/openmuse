#![allow(non_snake_case, dead_code, unused_variables)]

// Java: import java.lang.reflect.Array
// Java: import java.util.AbstractSet
// Java: import java.util.Collection
// Java: import java.util.Iterator
// Java: import java.util.Set

use std::any::Any;

// Java: public class MapValueView extends AbstractSet implements Set, Iterable
pub trait MapValueView {
    // Java: MapValueView()
    // Java: super();

    // Java: abstract Iterator<E> iterator()
    fn iterator(&self) -> Box<dyn Iterator<Item = Box<dyn Any>>>;

    // Java: abstract boolean removeElement(E var1)
    fn removeElement(&mut self, var1: &dyn Any) -> bool;

    // Java: abstract boolean containsElement(E var1)
    fn containsElement(&self, var1: &dyn Any) -> bool;

    // Java: abstract void clear()
    fn clear(&mut self);

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract boolean isEmpty()
    fn isEmpty(&self) -> bool;

    // Java: boolean contains(Object key)
    fn contains(&self, key: &dyn Any) -> bool {
        // Java: return this.containsElement((E)key);
        self.containsElement(key)
    }

    // Java: boolean remove(Object o)
    fn remove(&mut self, o: &dyn Any) -> bool {
        // Java: return this.removeElement((E)o);
        self.removeElement(o)
    }

    // Java: boolean add(E obj)
    fn add(&mut self, _obj: Box<dyn Any>) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: Object[] toArray()
    fn toArray(&self) -> Vec<Box<dyn Any>> {
        // Java: Object[] result = new Object[this.size()];
        let mut result: Vec<Box<dyn Any>> = Vec::with_capacity(self.size() as usize);
        // Java: Iterator<E> e = this.iterator();
        let e = self.iterator();
        // Java: for(int i = 0; e.hasNext(); ++i)
        for item in e
        {
            // Java: result[i] = e.next();
            result.push(item);
        }
        // Java: return result;
        result
    }

    // Java: public <T> T[] toArray(T[] a)
    fn toArray_into(&self, a: &mut Vec<Box<dyn Any>>) -> Vec<Box<dyn Any>> {
        // Java: int size = this.size();
        let size = self.size();
        // Java: if (a.length < size)
        if (a.len() as i32) < size
        {
            // Java: a = (T[])((Object[])Array.newInstance(a.getClass().getComponentType(), size));
            a.resize_with(size as usize, || Box::new(()));
        }
        // Java: Iterator<E> it = this.iterator();
        let it = self.iterator();
        // Java: Object[] result = a;
        let mut i = 0;
        // Java: for(int i = 0; i < size; ++i)
        for item in it
        {
            // Java: result[i] = it.next();
            if i < size as usize {
                a[i] = item;
            }
            i += 1;
        }
        // Java: if (a.length > size)
        if a.len() as i32 > size
        {
            // Java: a[size] = null;
            a[size as usize] = Box::new(());
        }
        // Java: return a;
        std::mem::take(a)
    }

    // Java: boolean addAll(Collection<? extends E> collection)
    fn addAll(&mut self, _collection: &[Box<dyn Any>]) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: boolean retainAll(Collection<?> collection)
    fn retainAll(&mut self, collection: &[Box<dyn Any>]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: Iterator<E> i = this.iterator();
        let items: Vec<Box<dyn Any>> = self.iterator().collect();
        // Java: while(i.hasNext())
        // Java: if (!collection.contains(i.next()))
        // Java: i.remove();
        // Java: changed = true;
        // Java: return changed;
        changed
    }
}
