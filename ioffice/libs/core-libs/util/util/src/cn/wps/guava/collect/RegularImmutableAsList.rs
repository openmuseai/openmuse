#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::ImmutableAsList::ImmutableAsList;
use super::ImmutableCollection::ImmutableCollection;
use super::ImmutableList::ImmutableList;

// Java: public class RegularImmutableAsList extends ImmutableAsList
pub struct RegularImmutableAsList {
    pub base: ImmutableAsList,
    // Java field: ImmutableCollection<E> delegate
    pub delegate: ImmutableCollection,
    // Java field: ImmutableList<? extends E> delegateList
    pub delegateList: ImmutableList,
}

// Java: public class RegularImmutableAsList extends ImmutableAsList
impl RegularImmutableAsList {
    // Java: RegularImmutableAsList(ImmutableCollection<E> delegate, ImmutableList<? extends E> delegateList)
    pub fn new(delegate: ImmutableCollection, delegateList: ImmutableList) -> Self {
        // Java: super();
        // Java: this.delegate = delegate;
        // Java: this.delegateList = delegateList;
        RegularImmutableAsList {
            base: ImmutableAsList::new(ImmutableList::new()),
            delegate,
            delegateList,
        }
    }

    // Java: RegularImmutableAsList(ImmutableCollection<E> delegate, Object[] array)
    pub fn from_array(delegate: ImmutableCollection, array: Vec<Option<Box<dyn Any>>>) -> Self {
        // Java: this(delegate, ImmutableList.asImmutableList(array));
        let list = ImmutableList::asImmutableList(array);
        Self::new(delegate, list)
    }

    // Java: ImmutableCollection<E> delegateCollection()
    pub fn delegateCollection(&self) -> &ImmutableCollection {
        // Java: return this.delegate;
        &self.delegate
    }

    // Java: ImmutableList<? extends E> delegateList()
    pub fn delegateList(&self) -> &ImmutableList {
        // Java: return this.delegateList;
        &self.delegateList
    }

    // Java: UnmodifiableListIterator<E> listIterator(int index)
    pub fn listIterator(&self, index: i32) {
        // Java: return (UnmodifiableListIterator<E>)this.delegateList.listIterator(index);
        self.delegateList.listIterator(index);
    }

    // Java: int copyIntoArray(Object[] dst, int offset)
    pub fn copyIntoArray(&self, dst: &mut Vec<Option<Box<dyn Any>>>, offset: i32) -> i32 {
        // Java: return this.delegateList.copyIntoArray(dst, offset);
        self.delegateList.copyIntoArray(dst, offset)
    }

    // Java: E get(int index)
    pub fn get(&self, index: i32) -> &Option<Box<dyn Any>> {
        // Java: return (E)this.delegateList.get(index);
        &self.delegateList.items[index as usize]
    }
}
