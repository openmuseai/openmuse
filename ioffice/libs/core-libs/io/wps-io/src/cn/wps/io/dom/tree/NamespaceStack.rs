// Java: import cn.wps.io.dom.Namespace
// Java: import java.util.ArrayList

// Java: public class NamespaceStack
pub struct NamespaceStack {
    // Java field: ArrayList<Namespace> namespaceStack
    namespaceStack: Vec<NamespaceEntry>,
    // Java field: Namespace defaultNamespace
    defaultNamespace: Option<NamespaceEntry>,
}

pub struct NamespaceEntry {
    pub prefix: String,
    pub uri: String,
}

impl NamespaceEntry {
    pub fn getPrefix(&self) -> &str {
        &self.prefix
    }

    pub fn getURI(&self) -> &str {
        &self.uri
    }
}

// Java: public class NamespaceStack
impl NamespaceStack {
    // Java: NamespaceStack()
    pub fn new() -> Self {
        NamespaceStack {
            namespaceStack: Vec::new(),
            defaultNamespace: None,
        }
    }

    // Java: void push(Namespace namespace)
    pub fn push(&mut self, namespace: NamespaceEntry) {
        let prefix = namespace.getPrefix().to_string();
        self.namespaceStack.push(namespace);
        if prefix.is_empty() {
            self.defaultNamespace = self.namespaceStack.last().cloned();
        }
    }

    // Java: Namespace pop()
    pub fn pop(&mut self) -> Option<NamespaceEntry> {
        let len = self.namespaceStack.len();
        if len > 0 {
            self.remove(len - 1)
        } else {
            None
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        self.namespaceStack.len() as i32
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        self.namespaceStack.clear();
    }

    // Java: Namespace getNamespace(int index)
    pub fn getNamespace(&self, index: i32) -> Option<&NamespaceEntry> {
        self.namespaceStack.get(index as usize)
    }

    // Java: Namespace getNamespaceForPrefix(String prefix)
    pub fn getNamespaceForPrefix(&self, prefix: &str) -> Option<&NamespaceEntry> {
        let prefix = if prefix.is_empty() { "" } else { prefix };
        for i in (0..self.namespaceStack.len()).rev() {
            let namespace = &self.namespaceStack[i];
            if prefix == namespace.getPrefix() {
                return Some(namespace);
            }
        }
        None
    }

    // Java: String getURI(String prefix)
    pub fn getURI(&self, prefix: &str) -> Option<&str> {
        self.getNamespaceForPrefix(prefix).map(|ns| ns.getURI())
    }

    // Java: boolean contains(Namespace namespace)
    pub fn contains(&self, namespace: &NamespaceEntry) -> bool {
        let prefix = namespace.getPrefix();
        let current = if !prefix.is_empty() {
            self.getNamespaceForPrefix(prefix)
        } else {
            self.getDefaultNamespace()
        };
        match current {
            None => false,
            Some(current) => namespace.getURI() == current.getURI(),
        }
    }

    // Java: void push(String prefix, String uri)
    pub fn push_prefix_uri(&mut self, prefix: &str, uri: &str) {
        let uri = if uri.is_empty() { "" } else { uri };
        let ns = self.createNamespace(prefix, uri);
        self.push(ns);
    }

    // Java: Namespace addNamespace(String prefix, String uri)
    pub fn addNamespace(&mut self, prefix: &str, uri: &str) -> usize {
        let namespace = self.createNamespace(prefix, uri);
        self.push(namespace);
        self.namespaceStack.len() - 1
    }

    // Java: Namespace pop(String prefix)
    pub fn pop_by_prefix(&mut self, prefix: &str) -> Option<NamespaceEntry> {
        let prefix = if prefix.is_empty() { "" } else { prefix };
        let mut found_idx = None;
        for i in (0..self.namespaceStack.len()).rev() {
            if prefix == self.namespaceStack[i].getPrefix() {
                found_idx = Some(i);
                break;
            }
        }
        match found_idx {
            Some(idx) => self.remove(idx),
            None => {
                eprintln!("Warning: missing namespace prefix ignored: {}", prefix);
                None
            }
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        format!("NamespaceStack Stack: [{}]",
            self.namespaceStack.iter()
                .map(|ns| format!("{}={}", ns.getPrefix(), ns.getURI()))
                .collect::<Vec<_>>()
                .join(", "))
    }

    // Java: Namespace getDefaultNamespace()
    pub fn getDefaultNamespace(&self) -> Option<&NamespaceEntry> {
        if self.defaultNamespace.is_some() {
            self.defaultNamespace.as_ref()
        } else {
            self.findDefaultNamespace()
        }
    }

    // Java: Namespace createNamespace(String prefix, String namespaceURI)
    fn createNamespace(&self, prefix: &str, namespaceURI: &str) -> NamespaceEntry {
        NamespaceEntry {
            prefix: prefix.to_string(),
            uri: namespaceURI.to_string(),
        }
    }

    // Java: Namespace findDefaultNamespace()
    fn findDefaultNamespace(&self) -> Option<&NamespaceEntry> {
        for i in (0..self.namespaceStack.len()).rev() {
            let namespace = &self.namespaceStack[i];
            let prefix = namespace.getPrefix();
            if prefix.is_empty() {
                return Some(namespace);
            }
        }
        None
    }

    // Java: Namespace remove(int index)
    fn remove(&mut self, index: usize) -> Option<NamespaceEntry> {
        if index < self.namespaceStack.len() {
            let namespace = self.namespaceStack.remove(index);
            self.defaultNamespace = None;
            Some(namespace)
        } else {
            None
        }
    }
}

impl Clone for NamespaceEntry {
    fn clone(&self) -> Self {
        NamespaceEntry {
            prefix: self.prefix.clone(),
            uri: self.uri.clone(),
        }
    }
}
