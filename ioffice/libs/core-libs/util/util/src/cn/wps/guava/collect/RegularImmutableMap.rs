#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::ImmutableMap::ImmutableMap;
use super::ImmutableMapEntry::ImmutableMapEntry;
use super::ImmutableMapEntrySet::ImmutableMapEntrySet;
use super::ImmutableSet::ImmutableSet;
use super::ImmutableList::ImmutableList;
use super::ImmutableCollection::ImmutableCollection;

// Java: import java.util.Map

// Java: public class RegularImmutableMap extends ImmutableMap
pub struct RegularImmutableMap {
    pub base: ImmutableMap,
    // Java field: ImmutableMapEntry<K, V>[] entries
    pub entries: Vec<Option<Box<ImmutableMapEntry>>>,
    // Java field: ImmutableMapEntry<K, V>[] table
    pub table: Vec<Option<Box<ImmutableMapEntry>>>,
    // Java field: int mask
    pub mask: i32,
    // Java field: double MAX_LOAD_FACTOR
}

pub(crate) const MAX_LOAD_FACTOR: f64 = 1.2;

// Java: public class NonTerminalMapEntry
pub struct NonTerminalMapEntry {
    pub base: ImmutableMapEntry,
    // Java field: ImmutableMapEntry<K, V> nextInKeyBucket
    pub nextInKeyBucket: Option<Box<ImmutableMapEntry>>,
}

// Java: public class EntrySet
pub struct EntrySet {
    pub base: ImmutableMapEntrySet,
}

// Java: public class RegularImmutableMap extends ImmutableMap
impl RegularImmutableMap {
    // Java: RegularImmutableMap(ImmutableMapEntry.TerminalEntry<K, V>... theEntries)
    pub fn from_entries(theEntries: Vec<Option<Box<ImmutableMapEntry>>>) -> Self {
        // Java: this(theEntries.length, theEntries);
        let size = theEntries.len() as i32;
        Self::new(size, theEntries)
    }

    // Java: RegularImmutableMap(int size, ImmutableMapEntry.TerminalEntry<K, V>[] theEntries)
    pub fn new(size: i32, theEntries: Vec<Option<Box<ImmutableMapEntry>>>) -> Self {
        // Java: super();
        // Java: this.entries = this.createEntryArray(size);
        let entries: Vec<Option<Box<ImmutableMapEntry>>> = (0..size as usize).map(|_| None).collect();
        // Java: int tableSize = closedTableSize(size, 1.2);
        let tableSize = Self::closedTableSize(size, 1.2);
        // Java: this.table = this.createEntryArray(tableSize);
        let table: Vec<Option<Box<ImmutableMapEntry>>> = (0..tableSize as usize).map(|_| None).collect();
        // Java: this.mask = tableSize - 1;
        let mask = tableSize - 1;

        // Java: for(int entryIndex = 0; entryIndex < size; ++entryIndex)
        // Java: ImmutableMapEntry.TerminalEntry<K, V> entry = theEntries[entryIndex];
        // Java: K key = (K)entry.getKey();
        // Java: int tableIndex = Hashing.smear(key.hashCode()) & this.mask;
        // Java: ImmutableMapEntry<K, V> existing = this.table[tableIndex];
        // Java: ImmutableMapEntry<K, V> newEntry = (ImmutableMapEntry<K, V>)(existing == null ? entry : new NonTerminalMapEntry(entry, existing));
        // Java: this.table[tableIndex] = newEntry;
        // Java: this.entries[entryIndex] = newEntry;
        // Java: this.checkNoConflictInBucket(key, newEntry, existing);

        RegularImmutableMap {
            base: ImmutableMap::new(),
            entries,
            table,
            mask,
        }
    }

    // Java: static int closedTableSize(int expectedEntries, double loadFactor)
    pub fn closedTableSize(expectedEntries: i32, loadFactor: f64) -> i32 {
        // Java: expectedEntries = Math.max(expectedEntries, 2);
        let expectedEntries = expectedEntries.max(2);
        // Java: int tableSize = Integer.highestOneBit(expectedEntries);
        let mut tableSize = if expectedEntries <= 0 { 0 } else { 1 << (31 - (expectedEntries as u32).leading_zeros()) };
        // Java: if (expectedEntries > (int)(loadFactor * (double)tableSize))
        if expectedEntries > (loadFactor * tableSize as f64) as i32 {
            // Java: tableSize <<= 1;
            tableSize <<= 1;
            // Java: return tableSize > 0 ? tableSize : 1073741824;
            if tableSize > 0 { tableSize } else { 1073741824 }
        }
        // Java: else
        else {
            // Java: return tableSize;
            tableSize
        }
    }

    // Java: void checkNoConflictInBucket(K key, ImmutableMapEntry<K, V> entry, ImmutableMapEntry<K, V> bucketHead)
    pub fn checkNoConflictInBucket(&self, _key: &dyn Any, _entry: &ImmutableMapEntry, _bucketHead: Option<&ImmutableMapEntry>) {
        // Java: while(bucketHead != null)
        {
            // Java: checkNoConflict(!key.equals(bucketHead.getKey()), "key", entry, bucketHead);
            // Java: bucketHead = bucketHead.getNextInKeyBucket();
        }
    }

    // Java: private ImmutableMapEntry<K, V>[] createEntryArray(int size)
    pub fn createEntryArray(size: i32) -> Vec<Option<Box<ImmutableMapEntry>>> {
        // Java: return new ImmutableMapEntry[size];
        (0..size as usize).map(|_| None).collect()
    }

    // Java: V get(Object key)
    pub fn get(&self, _key: &dyn Any) -> Option<&Box<dyn Any>> {
        // Java: if (key == null)
        {
            // Java: return null;
        }
        // Java: else
        {
            // Java: int index = Hashing.smear(key.hashCode()) & this.mask;
            // Java: for(ImmutableMapEntry<K, V> entry = this.table[index]; entry != null; entry = entry.getNextInKeyBucket())
            {
                // Java: K candidateKey = (K)entry.getKey();
                // Java: if (key.equals(candidateKey))
                {
                    // Java: return (V)entry.getValue();
                }
            }
            // Java: return null;
            None
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.entries.length;
        self.entries.len() as i32
    }

    // Java: ImmutableSet<Map.Entry<K, V>> createEntrySet()
    pub fn createEntrySet(&self) -> EntrySet {
        // Java: return new EntrySet();
        EntrySet { base: ImmutableMapEntrySet::new(ImmutableSet::new()) }
    }
}

// Java: public class NonTerminalMapEntry
impl NonTerminalMapEntry {
    // Java: NonTerminalMapEntry(K key, V value, ImmutableMapEntry<K, V> nextInKeyBucket)
    pub fn new(key: Option<Box<dyn Any>>, value: Option<Box<dyn Any>>, nextInKeyBucket: Option<Box<ImmutableMapEntry>>) -> Self {
        // Java: super(key, value);
        // Java: this.nextInKeyBucket = nextInKeyBucket;
        NonTerminalMapEntry {
            base: ImmutableMapEntry::new(key, value),
            nextInKeyBucket,
        }
    }

    // Java: NonTerminalMapEntry(ImmutableMapEntry<K, V> contents, ImmutableMapEntry<K, V> nextInKeyBucket)
    pub fn from_entry(contents: &ImmutableMapEntry, nextInKeyBucket: Option<Box<ImmutableMapEntry>>) -> Self {
        // Java: super(contents);
        // Java: this.nextInKeyBucket = nextInKeyBucket;
        NonTerminalMapEntry {
            base: ImmutableMapEntry::from_entry(contents),
            nextInKeyBucket,
        }
    }

    // Java: ImmutableMapEntry<K, V> getNextInKeyBucket()
    pub fn getNextInKeyBucket(&self) -> &Option<Box<ImmutableMapEntry>> {
        // Java: return this.nextInKeyBucket;
        &self.nextInKeyBucket
    }

    // Java: ImmutableMapEntry<K, V> getNextInValueBucket()
    pub fn getNextInValueBucket(&self) -> Option<&ImmutableMapEntry> {
        // Java: return null;
        None
    }
}

// Java: public class EntrySet
impl EntrySet {
    // Java: EntrySet()
    pub fn new() -> Self {
        // Java: super();
        EntrySet { base: ImmutableMapEntrySet::new(ImmutableSet::new()) }
    }

    // Java: ImmutableMap<K, V> map()
    pub fn map(&self) {
        // Java: return RegularImmutableMap.this;
    }

    // Java: UnmodifiableIterator<Map.Entry<K, V>> iterator()
    pub fn iterator(&self) {
        // Java: return this.asList().iterator();
    }

    // Java: ImmutableList<Map.Entry<K, V>> createAsList()
    pub fn createAsList(&self) {
        // Java: return new RegularImmutableAsList<Map.Entry<K, V>>(this, RegularImmutableMap.this.entries);
    }
}
