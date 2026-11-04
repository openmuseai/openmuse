#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::ImmutableCollection::ImmutableCollection;
use super::ImmutableCollection::ArrayBasedBuilder;
use super::ImmutableCollection::ImmutableCollectionBuilder;
use super::Hashing::Hashing;

// Java field: int MAX_TABLE_SIZE
pub(crate) const MAX_TABLE_SIZE: i32 = 1073741824;
// Java field: double DESIRED_LOAD_FACTOR
pub(crate) const DESIRED_LOAD_FACTOR: f64 = 0.7;
// Java field: int CUTOFF
pub(crate) const CUTOFF: i32 = 751619276;

// Java: public class ImmutableSet extends ImmutableCollection implements Set
pub struct ImmutableSet {
    pub base: ImmutableCollection,
    pub items: Vec<Option<Box<dyn Any>>>,
    pub hash_code_val: i32,
}

// Java: public class ImmutableSet extends ImmutableCollection implements Set
impl ImmutableSet {
    // Java: static public static <E> ImmutableSet<E> of(E element)
    pub fn of(element: Box<dyn Any>) -> Self {
        // Java: return new SingletonImmutableSet<E>(element);
        ImmutableSet {
            base: ImmutableCollection::new(),
            items: vec![Some(element)],
            hash_code_val: 0,
        }
    }

    // Java: static private static <E> ImmutableSet<E> construct(int n, Object... elements)
    fn construct(n: usize, elements: &mut Vec<Option<Box<dyn Any>>>) -> Self {
        // Java: int tableSize = chooseTableSize(n);
        let table_size = Self::chooseTableSize(n as i32);
        // Java: Object[] table = new Object[tableSize];
        let mut table: Vec<Option<usize>> = vec![None; table_size as usize];
        // Java: int mask = tableSize - 1;
        let mask = (table_size - 1) as usize;
        // Java: int hashCode = 0;
        let mut hash_code = 0i32;
        // Java: int uniques = 0;
        let mut uniques = 0usize;

        // Java: for(int i = 0; i < n; ++i)
        for i in 0..n {
            // Java: Object element = ObjectArrays.checkElementNotNull(elements[i], i);
            // Java: int hash = element.hashCode();
            let hash = 0i32;
            // Java: int j = Hashing.smear(hash);
            let mut j = Hashing::smear(hash) as usize;

            // Java: while(true)
            loop {
                // Java: int index = j & mask;
                let index = j & mask;
                // Java: Object value = table[index];
                let value = table[index];
                // Java: if (value == null)
                if value.is_none() {
                    // Java: elements[uniques++] = element;
                    // Java: table[index] = element;
                    table[index] = Some(uniques);
                    uniques += 1;
                    // Java: hashCode += hash;
                    hash_code += hash;
                    // Java: break;
                    break;
                }
                // Java: if (value.equals(element))
                {
                    // Java: break;
                    break;
                }
                // Java: ++j;
                j += 1;
            }
        }

        // Java: Arrays.fill(elements, uniques, n, null);
        for i in uniques..n {
            if i < elements.len() {
                elements[i] = None;
            }
        }
        // Java: if (uniques == 1)
        if uniques == 1 {
            // Java: E element = (E)elements[0];
            // Java: return new SingletonImmutableSet<E>(element, hashCode);
            return ImmutableSet {
                base: ImmutableCollection::new(),
                items: vec![elements[0].take()],
                hash_code_val: hash_code,
            };
        }
        // Java: else if (tableSize != chooseTableSize(uniques))
        else if table_size != Self::chooseTableSize(uniques as i32) {
            // Java: return construct(uniques, elements);
            return Self::construct(uniques, elements);
        }
        // Java: else
        {
            // Java: Object[] uniqueElements = uniques < elements.length ? ObjectArrays.arraysCopyOf(elements, uniques) : elements;
            // Java: return new RegularImmutableSet<E>(uniqueElements, hashCode, table, mask);
            ImmutableSet {
                base: ImmutableCollection::new(),
                items: elements.drain(..uniques).collect(),
                hash_code_val: hash_code,
            }
        }
    }

    // Java: static int chooseTableSize(int setSize)
    pub fn chooseTableSize(set_size: i32) -> i32 {
        // Java: if (setSize >= 751619276)
        if set_size >= CUTOFF {
            // Java: Preconditions.checkArgument(setSize < 1073741824, "collection too large");
            assert!(set_size < MAX_TABLE_SIZE, "collection too large");
            // Java: return 1073741824;
            return MAX_TABLE_SIZE;
        }
        // Java: else
        {
            // Java: int tableSize;
            // Java: for(tableSize = Integer.highestOneBit(setSize - 1) << 1; (double)tableSize * 0.7 < (double)setSize; tableSize <<= 1)
            let mut table_size = (((set_size - 1) as u32).next_power_of_two()) as i32;
            while (table_size as f64) * DESIRED_LOAD_FACTOR < (set_size as f64) {
                table_size <<= 1;
            }
            {
            }
            // Java: return tableSize;
            table_size
        }
    }

    // Java: static public static <E> ImmutableSet<E> copyOf(E[] elements)
    pub fn copyOf(elements: Vec<Option<Box<dyn Any>>>) -> Self {
        // Java: return construct(elements.length, elements.clone());
        let n = elements.len();
        let mut elems = elements;
        Self::construct(n, &mut elems)
    }

    // Java: ImmutableSet()
    pub fn new() -> Self {
        // Java: super();
        ImmutableSet {
            base: ImmutableCollection::new(),
            items: Vec::new(),
            hash_code_val: 0,
        }
    }

    // Java: boolean isHashCodeFast()
    pub fn isHashCodeFast(&self) -> bool {
        // Java: return false;
        false
    }

    // Java: boolean equals(Object object)
    pub fn equals(&self, _object: Option<&dyn Any>) -> bool {
        // Java: if (object == this)
        // {
            // Java: return true;
        // }
        // Java: else
        {
            // Java: return object instanceof ImmutableSet && this.isHashCodeFast() && ((ImmutableSet)object).isHashCodeFast() && this.hashCode() != object.hashCode() ? false : Sets.equalsImpl(this, object);
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return Sets.hashCodeImpl(this);
        self.hash_code_val
    }

    // Java: abstract UnmodifiableIterator<E> iterator()

    // Java: static public static <E> Builder<E> builder()
    pub fn builder() -> ImmutableSetBuilder {
        // Java: return new Builder<E>();
        ImmutableSetBuilder::new()
    }
}

// Java: public class Builder
pub struct ImmutableSetBuilder {
    pub base: ArrayBasedBuilder,
}

// Java: public class Builder
impl ImmutableSetBuilder {
    // Java: Builder()
    pub fn new() -> Self {
        // Java: this(4);
        Self::with_capacity(4)
    }

    // Java: Builder(int capacity)
    pub fn with_capacity(capacity: i32) -> Self {
        // Java: super(capacity);
        ImmutableSetBuilder {
            base: ArrayBasedBuilder::new(capacity),
        }
    }

    // Java: Builder<E> add(E element)
    pub fn add(&mut self, element: Option<Box<dyn Any>>) -> &mut Self {
        // Java: super.add(element);
        self.base.add(element);
        // Java: return this;
        self
    }

    // Java: Builder<E> add(E... elements)
    pub fn add_varargs(&mut self, elements: &[Option<Box<dyn Any>>]) -> &mut Self {
        // Java: super.add(elements);
        self.base.add_varargs(elements);
        // Java: return this;
        self
    }

    // Java: Builder<E> addAll(Iterable<? extends E> elements)
    pub fn addAll_iterable(&mut self, elements: &[Option<Box<dyn Any>>]) -> &mut Self {
        // Java: super.addAll(elements);
        self.base.addAll_iterable(elements);
        // Java: return this;
        self
    }

    // Java: Builder<E> addAll(Iterator<? extends E> elements)
    pub fn addAll_iterator(&mut self) -> &mut Self {
        // Java: super.addAll(elements);
        // Java: return this;
        self
    }

    // Java: ImmutableSet<E> build()
    pub fn build(&mut self) -> ImmutableSet {
        // Java: ImmutableSet<E> result = ImmutableSet.<E>construct(this.size, this.contents);
        let result = ImmutableSet::new();
        // Java: this.size = result.size();
        self.base.size = result.items.len();
        // Java: return result;
        result
    }
}
