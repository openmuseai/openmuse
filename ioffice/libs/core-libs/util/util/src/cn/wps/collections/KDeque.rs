// Java: import java.util.Iterator
// Java: import java.util.Queue

// Java: public interface KDeque extends Queue
pub trait KDeque<E> {
    // Java: abstract void addFirst(E var1)
    fn addFirst(&mut self, var1: E);

    // Java: abstract void addLast(E var1)
    fn addLast(&mut self, var1: E);

    // Java: abstract boolean offerFirst(E var1)
    fn offerFirst(&mut self, var1: E) -> bool;

    // Java: abstract boolean offerLast(E var1)
    fn offerLast(&mut self, var1: E) -> bool;

    // Java: abstract E removeFirst()
    fn removeFirst(&mut self) -> Option<E>;

    // Java: abstract E removeLast()
    fn removeLast(&mut self) -> Option<E>;

    // Java: abstract E pollFirst()
    fn pollFirst(&mut self) -> Option<E>;

    // Java: abstract E pollLast()
    fn pollLast(&mut self) -> Option<E>;

    // Java: abstract E getFirst()
    fn getFirst(&self) -> Option<&E>;

    // Java: abstract E getLast()
    fn getLast(&self) -> Option<&E>;

    // Java: abstract E peekFirst()
    fn peekFirst(&self) -> Option<&E>;

    // Java: abstract E peekLast()
    fn peekLast(&self) -> Option<&E>;

    // Java: abstract boolean removeFirstOccurrence(Object var1)
    fn removeFirstOccurrence(&mut self, var1: &E) -> bool where E: PartialEq;

    // Java: abstract boolean removeLastOccurrence(Object var1)
    fn removeLastOccurrence(&mut self, var1: &E) -> bool where E: PartialEq;

    // Java: abstract boolean add(E var1)
    fn add(&mut self, var1: E) -> bool;

    // Java: abstract boolean offer(E var1)
    fn offer(&mut self, var1: E) -> bool;

    // Java: abstract E remove()
    fn remove(&mut self) -> Option<E>;

    // Java: abstract E poll()
    fn poll(&mut self) -> Option<E>;

    // Java: abstract E element()
    fn element(&self) -> Option<&E>;

    // Java: abstract E peek()
    fn peek(&self) -> Option<&E>;

    // Java: abstract void push(E var1)
    fn push(&mut self, var1: E);

    // Java: abstract E pop()
    fn pop(&mut self) -> Option<E>;

    // Java: abstract boolean remove(Object var1)
    fn remove_obj(&mut self, var1: &E) -> bool where E: PartialEq;

    // Java: abstract boolean contains(Object var1)
    fn contains(&self, var1: &E) -> bool where E: PartialEq;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract Iterator<E> iterator()
    // Java: abstract Iterator<E> descendingIterator()
}
