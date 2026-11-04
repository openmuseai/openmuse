// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.Comparator
// Java: import java.util.HashSet
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.Set

use std::collections::HashSet;
use std::sync::Arc;
use super::Property::Property;
use super::Child::Child;
use super::Parent::Parent;

// Java: public class PropertyComparator
pub struct PropertyComparator {
}

// Java: public class PropertyComparator
impl PropertyComparator {
    // Java: PropertyComparator()
    pub fn new() -> Self {
        // Java: super();
        PropertyComparator {}
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, _o: &PropertyComparator) -> bool {
        // Java: return this == o;
        std::ptr::eq(self, _o)
    }

    // Java: int compare(Property o1, Property o2)
    pub fn compare(&self, o1: &Property, o2: &Property) -> i32 {
        // Java: String VBA_PROJECT = "_VBA_PROJECT";
        let VBA_PROJECT = "_VBA_PROJECT";
        // Java: String name1 = o1.getName();
        let name1 = o1.getName().to_string();
        // Java: String name2 = o2.getName();
        let name2 = o2.getName().to_string();
        // Java: int result = name1.length() - name2.length();
        let mut result = name1.len() as i32 - name2.len() as i32;
        // Java: if (result == 0)
        if result == 0 {
            // Java: if (name1.compareTo(VBA_PROJECT) == 0)
            if name1 == VBA_PROJECT {
                // Java: result = 1;
                result = 1;
            }
            // Java: else if (name2.compareTo(VBA_PROJECT) == 0)
            else if name2 == VBA_PROJECT {
                // Java: result = -1;
                result = -1;
            }
            // Java: else if (name1.startsWith("__") && name2.startsWith("__"))
            else if name1.starts_with("__") && name2.starts_with("__") {
                // Java: result = name1.compareToIgnoreCase(name2);
                result = name1.to_lowercase().cmp(&name2.to_lowercase()) as i32;
            }
            // Java: else if (name1.startsWith("__"))
            else if name1.starts_with("__") {
                // Java: result = 1;
                result = 1;
            }
            // Java: else if (name2.startsWith("__"))
            else if name2.starts_with("__") {
                // Java: result = -1;
                result = -1;
            }
            // Java: else
            else {
                // Java: result = name1.compareToIgnoreCase(name2);
                result = name1.to_lowercase().cmp(&name2.to_lowercase()) as i32;
            }
        }
        // Java: return result;
        result
    }
}

// Java: public class DirectoryProperty extends Property implements Parent
pub struct DirectoryProperty {
    pub(crate) base: Property,
    // Java field: List<Property> _children
    pub(crate) _children: Vec<Property>,
    // Java field: Set<String> _children_names
    pub(crate) _children_names: HashSet<String>,
}

// Java: public class DirectoryProperty extends Property implements Parent
impl DirectoryProperty {
    // Java: DirectoryProperty(String name)
    pub fn new(name: String) -> Self {
        // Java: super(name, (byte)1, (byte)1, 0, 0);
        let mut base = Property::new_with_params(name.clone(), 1, 1, 0, 0);
        // Java: this.setName(name);
        base.setName(name);
        // Java: this.setSize(0);
        base.setSize(0);
        // Java: this.setPropertyType((byte)1);
        base.setPropertyType(1);
        // Java: this.setStartBlock(0);
        base.setStartBlock(0);
        // Java: this.setNodeColor((byte)1);
        base.setNodeColor(1);
        DirectoryProperty {
            base,
            _children: Vec::new(),
            _children_names: HashSet::new(),
        }
    }

    // Java: DirectoryProperty(int index, byte[] array, int offset)
    pub fn new_from_array(index: i32, array: &[u8], offset: i32) -> Self {
        // Java: super(index, array, offset);
        let base = Property::new_from_array(index, array, offset);
        DirectoryProperty {
            base,
            _children: Vec::new(),
            _children_names: HashSet::new(),
        }
    }

    // Java: boolean changeName(Property property, String newName)
    pub fn changeName(&mut self, property: &mut Property, newName: String) -> bool {
        // Java: String oldName = property.getName();
        let oldName = property.getName().to_string();
        // Java: property.setName(newName);
        property.setName(newName);
        // Java: String cleanNewName = property.getName();
        let cleanNewName = property.getName().to_string();
        // Java: boolean result;
        let result;
        // Java: if (this._children_names.contains(cleanNewName))
        if self._children_names.contains(&cleanNewName) {
            // Java: property.setName(oldName);
            property.setName(oldName);
            // Java: result = false;
            result = false;
        }
        // Java: else
        else {
            // Java: this._children_names.add(cleanNewName);
            self._children_names.insert(cleanNewName);
            // Java: this._children_names.remove(oldName);
            self._children_names.remove(&oldName);
            // Java: result = true;
            result = true;
        }
        // Java: return result;
        result
    }

    // Java: boolean deleteChild(Property property)
    pub fn deleteChild(&mut self, property: &Property) -> bool {
        // Java: boolean result = this._children.remove(property);
        let pos = self._children.iter().position(|c| c.getName() == property.getName());
        let result = pos.is_some();
        if let Some(idx) = pos {
            self._children.remove(idx);
        }
        // Java: if (result)
        if result {
            // Java: this._children_names.remove(property.getName());
            self._children_names.remove(property.getName());
        }
        // Java: return result;
        result
    }

    // Java: boolean isDirectory()
    pub fn isDirectory(&self) -> bool {
        // Java: return true;
        true
    }

    // Java: void preWrite()
    pub fn preWrite(&mut self) {
        // Java: if (this._children.size() > 0)
        if self._children.len() > 0 {
            // Java: Property[] children = (Property[])this._children.toArray(new Property[0]);
            let comparator = PropertyComparator::new();
            // Java: Arrays.sort(children, new PropertyComparator());
            self._children.sort_by(|a, b| {
                let r = comparator.compare(a, b);
                if r < 0 { std::cmp::Ordering::Less } else if r > 0 { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal }
            });
            // Java: int midpoint = children.length / 2;
            let midpoint = self._children.len() / 2;
            // Java: this.setChildProperty(children[midpoint].getIndex());
            self.base.setChildProperty(self._children[midpoint].getIndex());
            // Java: children[0].setPreviousChild((Child)null);
            self._children[0]._previous_property = -1;
            self._children[0]._previous_child = None;
            // Java: children[0].setNextChild((Child)null);
            self._children[0]._next_property = -1;
            self._children[0]._next_child = None;
            // Java: for(int j = 1; j < midpoint; ++j)
            for j in 1..midpoint {
                // Java: children[j].setPreviousChild(children[j - 1]);
                let prev_index = self._children[j - 1].getIndex();
                self._children[j]._previous_property = prev_index;
                // Java: children[j].setNextChild((Child)null);
                self._children[j]._next_property = -1;
                self._children[j]._next_child = None;
            }
            // Java: if (midpoint != 0)
            if midpoint != 0 {
                // Java: children[midpoint].setPreviousChild(children[midpoint - 1]);
                let prev_index = self._children[midpoint - 1].getIndex();
                self._children[midpoint]._previous_property = prev_index;
            }
            // Java: if (midpoint != children.length - 1)
            if midpoint != self._children.len() - 1 {
                // Java: children[midpoint].setNextChild(children[midpoint + 1]);
                let next_index = self._children[midpoint + 1].getIndex();
                self._children[midpoint]._next_property = next_index;
                // Java: for(int j = midpoint + 1; j < children.length - 1; ++j)
                for j in (midpoint + 1)..(self._children.len() - 1) {
                    // Java: children[j].setPreviousChild((Child)null);
                    self._children[j]._previous_property = -1;
                    self._children[j]._previous_child = None;
                    // Java: children[j].setNextChild(children[j + 1]);
                    let next_idx = self._children[j + 1].getIndex();
                    self._children[j]._next_property = next_idx;
                }
                let last = self._children.len() - 1;
                // Java: children[children.length - 1].setPreviousChild((Child)null);
                self._children[last]._previous_property = -1;
                self._children[last]._previous_child = None;
                // Java: children[children.length - 1].setNextChild((Child)null);
                self._children[last]._next_property = -1;
                self._children[last]._next_child = None;
            }
            // Java: else
            else {
                // Java: children[midpoint].setNextChild((Child)null);
                self._children[midpoint]._next_property = -1;
                self._children[midpoint]._next_child = None;
            }
        }
    }

    // Java: Iterator<Property> getChildren()
    pub fn getChildren(&self) -> &Vec<Property> {
        // Java: return this._children.iterator();
        &self._children
    }

    // Java: void addChild(Property property)
    pub fn addChild(&mut self, property: Property) -> std::io::Result<()> {
        // Java: if (!this._children_names.add(property.getName()))
        if !self._children_names.insert(property.getName().to_string()) {
            // Java: throw new IOException("Duplicate name \"" + property.getName() + "\"");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Duplicate name \"{}\"", property.getName())));
        }
        // Java: else
        else {
            // Java: this._children.add(property);
            self._children.push(property);
        }
        Ok(())
    }

    // Java: boolean isPropertyExist(Property property)
    pub fn isPropertyExist(&self, property: &Property) -> bool {
        // Java: return this._children_names.contains(property.getName());
        self._children_names.contains(property.getName())
    }
}

impl Child for DirectoryProperty {
    fn getNextChild(&self) -> Option<Arc<dyn Child>> {
        self.base.getNextChild_impl()
    }
    fn getPreviousChild(&self) -> Option<Arc<dyn Child>> {
        self.base.getPreviousChild_impl()
    }
    fn setNextChild(&mut self, var1: Option<Arc<dyn Child>>) {
        self.base.setNextChild_impl(var1);
    }
    fn setPreviousChild(&mut self, var1: Option<Arc<dyn Child>>) {
        self.base.setPreviousChild_impl(var1);
    }
}

impl Parent for DirectoryProperty {
    fn getChildren(&self) -> Vec<Arc<dyn Child>> {
        Vec::new()
    }
    fn addChild(&mut self, _var1: Arc<dyn Child>) {
    }
    fn setPreviousChild_parent(&mut self, var1: Option<Arc<dyn Child>>) {
        self.base.setPreviousChild_impl(var1);
    }
    fn setNextChild_parent(&mut self, var1: Option<Arc<dyn Child>>) {
        self.base.setNextChild_impl(var1);
    }
}
