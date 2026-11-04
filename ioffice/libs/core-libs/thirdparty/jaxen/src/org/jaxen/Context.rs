use super::{List, Object};
use super::ContextSupport::ContextSupport;
use super::Navigator::Navigator;
use super::function::Function;
use super::JaxenException::JaxenException;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 2315979994685591055;

// Java: public class Context implements Serializable
pub struct Context {
    // Java field: ContextSupport contextSupport
    pub(crate) contextSupport: ContextSupport,
    // Java field: List nodeSet
    pub(crate) nodeSet: List,
    // Java field: int size
    pub(crate) size: i32,
    // Java field: int position
    pub(crate) position: i32,
}

// Java: public class Context implements Serializable
impl Context {
    // Java: Context(ContextSupport contextSupport)
    pub fn new(contextSupport: ContextSupport) -> Self {
        // Java: super();
        // Java: this.contextSupport = contextSupport;
        // Java: this.nodeSet = Collections.EMPTY_LIST;
        // Java: this.size = 0;
        // Java: this.position = 0;
        Context {
            contextSupport,
            nodeSet: Vec::new(),
            size: 0,
            position: 0,
        }
    }

    // Java: void setNodeSet(List nodeSet)
    pub fn setNodeSet(&mut self, nodeSet: List) {
        // Java: this.nodeSet = nodeSet;
        // Java: this.size = nodeSet.size();
        self.size = nodeSet.len() as i32;
        self.nodeSet = nodeSet;
        // Java: if (this.position >= this.size)
        if self.position >= self.size {
            // Java: this.position = 0;
            self.position = 0;
        }
    }

    // Java: List getNodeSet()
    pub fn getNodeSet(&self) -> &List {
        // Java: return this.nodeSet;
        &self.nodeSet
    }

    // Java: void setContextSupport(ContextSupport contextSupport)
    pub fn setContextSupport(&mut self, contextSupport: ContextSupport) {
        // Java: this.contextSupport = contextSupport;
        self.contextSupport = contextSupport;
    }

    // Java: ContextSupport getContextSupport()
    pub fn getContextSupport(&self) -> &ContextSupport {
        // Java: return this.contextSupport;
        &self.contextSupport
    }

    // Java: Navigator getNavigator()
    pub fn getNavigator(&self) -> Option<&dyn Navigator> {
        // Java: return this.getContextSupport().getNavigator();
        self.getContextSupport().getNavigator()
    }

    // Java: String translateNamespacePrefixToUri(String prefix)
    pub fn translateNamespacePrefixToUri(&self, prefix: &str) -> Option<String> {
        // Java: return this.getContextSupport().translateNamespacePrefixToUri(prefix);
        self.getContextSupport().translateNamespacePrefixToUri(prefix)
    }

    // Java: Object getVariableValue(String namespaceURI, String prefix, String localName)
    pub fn getVariableValue(&self, namespaceURI: &str, prefix: &str, localName: &str) -> Result<Object, JaxenException> {
        // Java: return this.getContextSupport().getVariableValue(namespaceURI, prefix, localName);
        self.getContextSupport().getVariableValue(namespaceURI, prefix, localName)
    }

    // Java: Function getFunction(String namespaceURI, String prefix, String localName)
    pub fn getFunction(&self, namespaceURI: &str, prefix: &str, localName: &str) -> Result<Box<dyn Function>, JaxenException> {
        // Java: return this.getContextSupport().getFunction(namespaceURI, prefix, localName);
        self.getContextSupport().getFunction(namespaceURI, prefix, localName)
    }

    // Java: void setSize(int size)
    pub fn setSize(&mut self, size: i32) {
        // Java: this.size = size;
        self.size = size;
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return this.size;
        self.size
    }

    // Java: void setPosition(int position)
    pub fn setPosition(&mut self, position: i32) {
        // Java: this.position = position;
        self.position = position;
    }

    // Java: int getPosition()
    pub fn getPosition(&self) -> i32 {
        // Java: return this.position;
        self.position
    }

    // Java: Context duplicate()
    pub fn duplicate(&self) -> Context {
        // Java: Context dupe = new Context(this.getContextSupport());
        let mut dupe = Context::new(ContextSupport::new());
        // Java: List thisNodeSet = this.getNodeSet();
        let this_node_set = self.getNodeSet();
        // Java: if (thisNodeSet != null)
        if !this_node_set.is_empty() {
            // Java: List dupeNodeSet = new ArrayList(thisNodeSet.size());
            // Java: dupeNodeSet.addAll(thisNodeSet);
            let dupe_node_set: List = this_node_set.clone();
            // Java: dupe.setNodeSet(dupeNodeSet);
            dupe.setNodeSet(dupe_node_set);
            // Java: dupe.setPosition(this.position);
            dupe.setPosition(self.position);
        }
        // Java: return dupe;
        dupe
    }
}
