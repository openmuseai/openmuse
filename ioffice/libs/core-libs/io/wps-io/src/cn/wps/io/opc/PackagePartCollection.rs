// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.HashSet
use std::collections::HashSet;

use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackagePartName::PackagePartName;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 2515031135957635515i64;

// Java: public class PackagePartCollection extends HashMap
pub struct PackagePartCollection {
    // Java field: HashSet<String> registerPartNameStr
    registerPartNameStr: HashSet<String>,
    map: HashMap<PackagePartName, PackagePart>,
}

// Java: public class PackagePartCollection
impl PackagePartCollection {
    // Java: public PackagePartCollection()
    pub fn PackagePartCollection() -> Self {
        // Java: super();
        Self::new()
    }

    // Java: PackagePartCollection()
    pub fn new() -> Self {
      // Java: super();
      PackagePartCollection { registerPartNameStr: HashSet::new(), map: HashMap::new() }
    }

    // Java: Object clone()
    pub fn clone_collection(&mut self) -> PackagePartCollection {
      // Java: return super.clone();
      // HashMap/PackagePart not Clone; shallow empty clone of register set only is insufficient —
      // reconstruct with cloned registerPartNameStr; map entries require PackagePart Clone (ownership glue: empty map)
      PackagePartCollection {
          registerPartNameStr: self.registerPartNameStr.clone(),
          map: HashMap::new(),
      }
    }

    // Java: PackagePart put(PackagePartName partName, PackagePart part)
    pub fn put(&mut self, partName: PackagePartName, part: PackagePart) -> Option<PackagePart> {
      // Java: return (PackagePart)super.put(partName, part);
      self.map.insert(partName, part)
    }

    pub fn size(&self) -> usize {
      self.map.len()
    }

    pub fn get(&self, key: &PackagePartName) -> Option<&PackagePart> {
      self.map.get(key)
    }

    pub fn get_mut(&mut self, key: &PackagePartName) -> Option<&mut PackagePart> {
      self.map.get_mut(key)
    }

    pub fn values(&self) -> impl Iterator<Item = &PackagePart> {
      self.map.values()
    }

    /// rust: ★补充门面（无 Java 对应 —— Java 的 `_container` 是堆引用，包被 move 也不受影响）。
    /// 用于包**落位之后**把每个 part 的 `_container` 重新指向新地址：Rust 的 `OPCPackage` 是值类型，
    /// 被 move 后栈地址改变，旧指针一旦解引用即 SIGBUS（见 `PackagePart::_container` 字段注释）。
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut PackagePart> {
      self.map.values_mut()
    }

    // Java: PackagePart remove(Object key)
    pub fn remove(&mut self, key: &PackagePartName) -> Option<PackagePart> {
      // Java: return (PackagePart)super.remove(key);
      self.map.remove(key)
    }

}
