#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::ImmutableEntry::ImmutableEntry;
use super::CollectPreconditions::CollectPreconditions;

// Java: public class ImmutableMapEntry extends ImmutableEntry
pub struct ImmutableMapEntry {
    pub base: ImmutableEntry,
}

// Java: public class ImmutableMapEntry extends ImmutableEntry
impl ImmutableMapEntry {
    // Java: ImmutableMapEntry(K key, V value)
    pub fn new(key: Option<Box<dyn Any>>, value: Option<Box<dyn Any>>) -> Self {
        // Java: super(key, value);
        // Java: CollectPreconditions.checkEntryNotNull(key, value);
        ImmutableMapEntry {
            base: ImmutableEntry::new(key, value),
        }
    }

    // Java: ImmutableMapEntry(ImmutableMapEntry contents)
    pub fn from_entry(contents: &ImmutableEntry) -> Self {
        // Java: super(contents.getKey(), contents.getValue());
        ImmutableMapEntry {
            base: ImmutableEntry::new(None, None),
        }
    }

    // Java: abstract abstract ImmutableMapEntry<K, V> getNextInKeyBucket()

    // Java: abstract abstract ImmutableMapEntry<K, V> getNextInValueBucket()
}

// Java: public class TerminalEntry
pub struct TerminalEntry {
    pub base: ImmutableMapEntry,
}

// Java: public class TerminalEntry
impl TerminalEntry {
    // Java: TerminalEntry(ImmutableMapEntry contents)
    pub fn from_entry(contents: &ImmutableEntry) -> Self {
        // Java: super(contents);
        TerminalEntry {
            base: ImmutableMapEntry::from_entry(contents),
        }
    }

    // Java: TerminalEntry(K key, V value)
    pub fn new(key: Option<Box<dyn Any>>, value: Option<Box<dyn Any>>) -> Self {
        // Java: super(key, value);
        TerminalEntry {
            base: ImmutableMapEntry::new(key, value),
        }
    }

    // Java: ImmutableMapEntry<K, V> getNextInKeyBucket()
    pub fn getNextInKeyBucket(&self) -> Option<Box<ImmutableMapEntry>> {
        // Java: return null;
        None
    }

    // Java: ImmutableMapEntry<K, V> getNextInValueBucket()
    pub fn getNextInValueBucket(&self) -> Option<Box<ImmutableMapEntry>> {
        // Java: return null;
        None
    }
}
