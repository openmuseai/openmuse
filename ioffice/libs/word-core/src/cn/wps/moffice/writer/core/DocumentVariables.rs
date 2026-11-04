// Java: import cn.wps.moffice.util.StringUtil
// Java: import java.lang.IllegalArgumentException
use stubs::java::lang::IllegalArgumentException::*;
use util::moffice::util::StringUtil::*;
// Java: import java.util.Map
// Java: import java.util.TreeMap
use stubs::java::util::TreeMap::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import java.util
use stubs::java::util::*;
use std::collections::{HashMap};

// java public class DocumentVariables
#[derive(Default)]
pub struct DocumentVariables {
    // java TextDocument document;
    // rust: Java 对象引用**别名/回指**宿主文档 ⇒ 裸指针（先例 `InnerIOListener.mDoc: Option<*mut TextDocument>`）。
    pub(crate) document: *mut TextDocument,
    // java Map<String, Variable> map = new TreeMap();
    // rust: Java `TreeMap` 恒非 null（构造即 `new`）⇒ 直接持有 `HashMap`，不用 `Option` 包裹。
    pub(crate) map: HashMap<String, Variable>,
}

impl DocumentVariables {
    fn document(&self) -> &TextDocument {
        unsafe { &*self.document }
    }

    fn document_mut(&mut self) -> &mut TextDocument {
        unsafe { &mut *self.document }
    }
}

// java public static class Variable
#[derive(Default, Clone)]
pub struct Variable {
    // java String name;
    pub(crate) name: Option<String>,
    // java String value;
    pub(crate) value: Option<String>,
}

// java public class DocumentVariables
impl DocumentVariables {
    // java public  DocumentVariables(TextDocument document)
    pub fn DocumentVariables(document: &TextDocument) -> Self {
      // java this.document = document;
      Self { document: document as *const TextDocument as *mut TextDocument, map: Default::default() }
    }

    // java public boolean add(String name, String value)
    pub fn add(&mut self, name: &str, value: Option<&str>) -> bool {
      // java if (!StringUtil.isEmpty(name) && value != null)
      if !StringUtil::isEmpty(Some(name)) && value.is_some() {
        // java String nameKey = name.toLowerCase();
        let mut nameKey: String = name.to_lowercase();
        // java if (this.map.containsKey(nameKey))
        if self.map.contains_key(&nameKey) {
          // java return false;
          return false;
        }
        // java else
        else {
          // java this.map.put(nameKey, new Variable(name, value));
          self.map.insert(nameKey, Variable::Variable(name, value.unwrap()));
          // java if (!this.document.isInIOState())
          if !self.document().isInIOState() {
            // java this.document.setModified(true, false);
            self.document_mut().setModified__boolean_boolean(true, false);
          }
          // java return true;
          return true;
        }
      }
      // java else
      else {
        // java throw new IllegalArgumentException();
        std::panic::panic_any(IllegalArgumentException::new());
      }
    }

    // java public String get(String name)
    pub fn get(&self, name: &str) -> Option<String> {
      // java if (StringUtil.isEmpty(name))
      if StringUtil::isEmpty(Some(name)) {
        // java throw new IllegalArgumentException();
        std::panic::panic_any(IllegalArgumentException::new());
      }
      // java else
      else {
        // java Variable v = (Variable)this.map.get(name.toLowerCase());
        let v: Option<&Variable> = self.map.get(&name.to_lowercase());
        // java return v == null ? null : v.getValue();
        return v.map(|v| v.getValue()).flatten();
      }
    }

    // java public boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // java return this.map.isEmpty();
      return self.map.is_empty();
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      // java return this.map.size();
      return (self.map.len() as i32);
    }

    // java public java.util.Collection<Variable> getAll()
    pub fn getAll(&self) -> Vec<Variable> {
      // java return this.map.values();
      return self.map.values().cloned().collect();
    }

}

// java public static class Variable
impl Variable {
    // java public  Variable(String name, String value)
    pub fn Variable(name: &str, value: &str) -> Self {
      // java this.name = name;
      Self { name: Some(name.to_string()), value: Some(value.to_string()) }
      // java this.value = value;
    }

    // java public String getName()
    pub fn getName(&self) -> Option<String> {
      // java return this.name;
      return self.name.clone();
    }

    // java public String getValue()
    pub fn getValue(&self) -> Option<String> {
      // java return this.value;
      return self.value.clone();
    }

}
