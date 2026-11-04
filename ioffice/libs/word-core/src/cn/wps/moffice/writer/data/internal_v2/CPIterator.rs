#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]
// Java: import cn.wps.moffice.writer.data.internal_v2.CPIteratorImpl
// java public class CPIterator
    // java private CPIteratorImpl mImpl;
// java public class CPIterator
    // java public  CPIterator(CPIteratorImpl impl)
      // java this.mImpl = impl;
    // java public boolean hasNext()
      // java return this.mImpl.hasNext();
    // java public T next()
      // java return (T)this.mImpl.next();
    // java public int getStart()
      // java return this.mImpl.getStart();
    // java public int getEnd()
      // java return this.mImpl.getEnd();

#[derive(Default, Clone, Debug)]
pub struct CPIterator {
    pub(crate) index: i32,
}
impl CPIterator {
    pub fn new() -> Self { Self::default() }
    pub fn hasNext(&self) -> bool { false }
    pub fn next(&mut self) -> Option<()> { None }
}
