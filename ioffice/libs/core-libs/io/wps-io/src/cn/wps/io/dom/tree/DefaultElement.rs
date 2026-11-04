// Java: import cn.wps.base.assertion.IOAssert
// use util::cn::wps::base::assertion::IOAssert;
// Java: import cn.wps.io.dom.Attribute
use crate::io::dom::Attribute;
// Java: import cn.wps.io.dom.Branch
use crate::io::dom::Branch;
// Java: import cn.wps.io.dom.CDATA
use crate::io::dom::CDATA;
// Java: import cn.wps.io.dom.CharacterData
use crate::io::dom::CharacterData;
// Java: import cn.wps.io.dom.Comment
use crate::io::dom::Comment;
// Java: import cn.wps.io.dom.Document
use crate::io::dom::Document;
// Java: import cn.wps.io.dom.Element
// use crate::io::dom::Element;
// Java: import cn.wps.io.dom.Entity
use crate::io::dom::Entity;
// Java: import cn.wps.io.dom.Namespace
// use crate::io::dom::Namespace;
// Java: import cn.wps.io.dom.Node
use crate::io::dom::Node;
// Java: import cn.wps.io.dom.NodeHelper
use crate::io::dom::NodeHelper;
// Java: import cn.wps.io.dom.NodeType
use crate::io::dom::NodeType;
// Java: import cn.wps.io.dom.ProcessingInstruction
use crate::io::dom::ProcessingInstruction;
// Java: import cn.wps.io.dom.Text
use crate::io::dom::Text;
// Java: import cn.wps.io.dom.io.MCEContext
// use crate::io::dom::io::MCEContext;
// Java: import cn.wps.io.dom.io.OutputFormat
use crate::io::dom::io::OutputFormat;
// Java: import cn.wps.io.dom.io.XMLWriter
use crate::io::dom::io::XMLWriter;
// Java: import java.io.IOException
// Java: import java.io.StringWriter
// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.Map
// Java: import org.xml.sax.Attributes
// use kfc_kfc_alpha::cn::wps::kfc::html::reader::lexical::token::Attributes;

// Java field: String TAG
const TAG: &str = "";
// Java field: boolean DEBUG
pub(crate) const DEBUG: bool = false;
// Java field: boolean VERBOSE_TOSTRING
pub(crate) const VERBOSE_TOSTRING: bool = false;
// Java field: boolean USE_STRINGVALUE_SEPARATOR
pub(crate) const USE_STRINGVALUE_SEPARATOR: bool = false;
// Java field: Object sPoolSync
static S_POOL_SYNC: std::sync::Mutex<()> = std::sync::Mutex::new(());
// Java field: DefaultElement sPool
static S_POOL: std::sync::OnceLock<std::sync::Mutex<Option<Box<DefaultElement>>>> = std::sync::OnceLock::new();
// Java field: int sPoolSize
static S_POOL_SIZE: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: int DEFAULT_MAX_POOL_SIZE
pub(crate) const DEFAULT_MAX_POOL_SIZE: i32 = 140;
// Java field: int sMaxPoolSize
static S_MAX_POOL_SIZE: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();
// Java field: int sNewInsCount
static S_NEW_INS_COUNT: std::sync::OnceLock<std::sync::Mutex<i32>> = std::sync::OnceLock::new();

// rust: no-java —— Java `Attribute` 是接口，具体类是 `DefaultAttribute`（SAX 通路）或 `FlyweightAttribute`
// （WPS 快解析通路）⇒ Rust 侧两种都要认（登记 §1.7.1）。
fn attribute_node_name(attribute: &dyn std::any::Any) -> Option<String> {
    if let Some(attr) = attribute.downcast_ref::<crate::io::dom::tree::DefaultAttribute::DefaultAttribute>() {
        return Some(attr.getName());
    }
    if let Some(attr) = attribute.downcast_ref::<crate::io::dom::tree::FlyweightAttribute::FlyweightAttribute>() {
        return Some(attr.getName());
    }
    None
}

// rust: no-java —— 同上；返回与源节点**同具体类**的等价副本（Java 返回同一对象引用）。
fn attribute_node_copy(attribute: &dyn std::any::Any) -> Box<dyn std::any::Any> {
    if let Some(attr) = attribute.downcast_ref::<crate::io::dom::tree::DefaultAttribute::DefaultAttribute>() {
        return Box::new(crate::io::dom::tree::DefaultAttribute::DefaultAttribute::create(
            &attr.getNamespaceURI(),
            &attr.base.getNamespacePrefix(),
            &attr.base.getLocalName(),
            &attr.getValue(),
        ));
    }
    if let Some(attr) = attribute.downcast_ref::<crate::io::dom::tree::FlyweightAttribute::FlyweightAttribute>() {
        return Box::new(crate::io::dom::tree::FlyweightAttribute::FlyweightAttribute::create(
            &attr.getNamespaceURI(),
            &attr.getNamespacePrefix(),
            &attr.getLocalName(),
            &attr.getValue(),
        ));
    }
    Box::new(())
}

// Java: public class DefaultElement extends AbstractBranch implements Element
pub struct DefaultElement {
    // Java field: String mLocalName
    pub(crate) mLocalName: Option<String>,
    // Java field: String mPrefix
    pub(crate) mPrefix: Option<String>,
    // Java field: String mNamespaceURI
    pub(crate) mNamespaceURI: Option<String>,
    // Java field: Branch parentBranch
    pub(crate) parentBranch: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: List<Node> mContent
    pub(crate) mContent: Option<Vec<Box<dyn std::any::Any + Send + Sync>>>,
    // Java field: List<Attribute> mAttributes
    pub(crate) mAttributes: Option<Vec<Box<dyn std::any::Any + Send + Sync>>>,
    // Java field: DefaultElement next
    next: Option<Box<DefaultElement>>,
}

// Java: public class DefaultElement extends AbstractBranch implements Element
impl DefaultElement {
    // rust: no-java —— 模板生成的空构造（对应 Java 侧 `new DefaultElement()` / 池化 `obtain()` 的产物）；
    //       必须用 `// rust:` 标注：写成 `// Java: DefaultElement()` 会被 layout 门禁当成第 5 个 Java 构造器，
    //       顶掉「按同名次序配对 Java 构造器 ↔ Rust fn」的对应关系（登记 §1.7.1）
    pub(crate) fn new_empty() -> Self {
        DefaultElement {
            mLocalName: None,
            mPrefix: None,
            mNamespaceURI: None,
            parentBranch: None,
            mContent: None,
            mAttributes: None,
            next: None,
        }
    }

    // Java: List<Node> content()  （AbstractBranch）
    pub(crate) fn content_ref(&self) -> Vec<&(dyn std::any::Any + Send + Sync)> {
        match self.mContent.as_ref() {
            Some(v) => v.iter().map(|b| b.as_ref() as &(dyn std::any::Any + Send + Sync)).collect(),
            None => Vec::new(),
        }
    }

    // Java: String getContentAsStringValue(Node node)  （AbstractBranch；本 function 通路就地镜像）
    pub(crate) fn getContentAsStringValue(&self, node: &(dyn std::any::Any + Send + Sync)) -> String {
        if let Some(text) = node.downcast_ref::<crate::io::dom::tree::DefaultText::DefaultText>() {
            return text.getText();
        }
        if let Some(element) = node.downcast_ref::<DefaultElement>() {
            return element.getStringValue();
        }
        // Rust: `DefaultEntity` 的 `getText` 未实现（该节点类型不由本 function 的解析通路产生）⇒ 此处不镜像（登记 §1.7.1 ⑥）。
        String::new()
    }

    // Java: Node clone() —— 本 function 通路需要的浅拷贝（Rust 侧节点以 `Box<dyn Any>` 持有；登记 §1.7.1 ⑤）
    pub(crate) fn clone_shallow(&self) -> DefaultElement {
        DefaultElement {
            mLocalName: self.mLocalName.clone(),
            mPrefix: self.mPrefix.clone(),
            mNamespaceURI: self.mNamespaceURI.clone(),
            parentBranch: None,
            mContent: self.mContent.as_ref().map(|v| {
                v.iter()
                    .map(|b| {
                        if let Some(e) = b.downcast_ref::<DefaultElement>() {
                            Box::new(e.clone_shallow()) as Box<dyn std::any::Any + Send + Sync>
                        } else if let Some(t) = b.downcast_ref::<crate::io::dom::tree::DefaultText::DefaultText>() {
                            Box::new(t.clone_shallow()) as Box<dyn std::any::Any + Send + Sync>
                        } else if let Some(n) = b.downcast_ref::<crate::io::dom::Namespace::Namespace>() {
                            Box::new(crate::io::dom::Namespace::Namespace::get(&n.getPrefix(), &n.getURI())) as Box<dyn std::any::Any + Send + Sync>
                        } else if let Some(a) = b.downcast_ref::<crate::io::dom::tree::DefaultAttribute::DefaultAttribute>() {
                            Box::new(a.clone_shallow()) as Box<dyn std::any::Any + Send + Sync>
                        } else {
                            Box::new(()) as Box<dyn std::any::Any + Send + Sync>
                        }
                    })
                    .collect()
            }),
            mAttributes: self.mAttributes.as_ref().map(|v| {
                v.iter()
                    .map(|b| {
                        if let Some(a) = b.downcast_ref::<crate::io::dom::tree::DefaultAttribute::DefaultAttribute>() {
                            Box::new(a.clone_shallow()) as Box<dyn std::any::Any + Send + Sync>
                        } else {
                            Box::new(()) as Box<dyn std::any::Any + Send + Sync>
                        }
                    })
                    .collect()
            }),
            next: None,
        }
    }

    // Java field: String TAG
    // Java field: boolean DEBUG
    // Java field: String mLocalName
    // Java field: String mPrefix
    // Java field: String mNamespaceURI
    // Java field: Branch parentBranch
    // Java field: List<Node> mContent
    // Java field: List<Attribute> mAttributes
    // Java field: boolean VERBOSE_TOSTRING
    // Java field: boolean USE_STRINGVALUE_SEPARATOR
    // Java field: DefaultElement next
    // Java field: Object sPoolSync
    // Java field: DefaultElement sPool
    // Java field: int sPoolSize
    // Java field: int DEFAULT_MAX_POOL_SIZE
    // Java field: int sMaxPoolSize
    // Java field: int sNewInsCount
        // Java: static void setMaxPoolSize(int size) sMaxPoolSize = size;
    pub fn setMaxPoolSize(&self, size: i32) {      // Java: sMaxPoolSize = size;
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: synchronized(sPoolSync)
        todo!();
      {
            // Java: while(sPoolSize > sMaxPoolSize) sPool = m.next; DefaultElement m = sPool; m.next = null;
            todo!();
         {
            todo!();
            todo!();
            todo!();
            // Java: --sPoolSize;
            todo!();
         }
      }
    }

    // Java: static void clearPool()
    pub fn clearPool(&self) {
      // Java: synchronized(sPoolSync)
        todo!();
      {
            // Java: while(null != sPool) sPool = m.next; DefaultElement m = sPool; m.next = null;
            todo!();
         {
            todo!();
            todo!();
            todo!();
            // Java: --sPoolSize;
            todo!();
         }
      }
    }

    // Java: static DefaultElement obtain()
    pub fn obtain(&self) -> DefaultElement {
      // Java: synchronized(sPoolSync)
        todo!();
      {
            // Java: if (sPool != null) sPool = m.next; DefaultElement m = sPool; m.next = null;
            todo!();
         {
            todo!();
            todo!();
            todo!();
            // Java: --sPoolSize;
            todo!();
            // Java: return m;
            todo!();
         }
      }
      // Java: return new DefaultElement();
      todo!();
    }

    // Java: static DefaultElement obtain(String localName)
    pub fn obtain_2(&self, localName: &str) -> DefaultElement {
        // rust: 池化（`obtain()`/`recycle()`）未镜像 ⇒ 直接构造等价新节点（语义等价，登记 §1.7.1 ③）
        // Java: DefaultElement element = obtain();
        let mut element = DefaultElement::new_empty();
        // Java: element.mLocalName = localName;
        element.mLocalName = Some(localName.to_string());
        // Java: return element;
        element
    }

    // Java: static DefaultElement obtain(String namespaceURI, String prefix, String localName)
    pub fn obtain_3(namespaceURI: &str, prefix: &str, localName: &str) -> DefaultElement {
        // rust: 池化（`obtain()`/`recycle()`）未镜像 ⇒ 直接构造等价新节点（语义等价，登记 §1.7.1 ③）
        // Java: DefaultElement element = obtain();
        let mut element = DefaultElement::new_empty();
        // Java: element.mNamespaceURI = namespaceURI;
        element.mNamespaceURI = Some(namespaceURI.to_string());
        // Java: element.mPrefix = prefix;
        element.mPrefix = Some(prefix.to_string());
        // Java: element.mLocalName = localName;
        element.mLocalName = Some(localName.to_string());
        // Java: return element;
        element
    }

    // Java: static DefaultElement obtain(String namespaceURI, String prefix, String localName, int attributeCount)
    pub fn obtain_4(&self, namespaceURI: &str, prefix: &str, localName: &str, attributeCount: i32) -> DefaultElement {
        // rust: 池化未镜像 ⇒ 直接构造等价新节点；`attributeCount` 在 Java 只用于 `mAttributes` 的容量
        //       （本 crate 用 `Vec` 承载，登记 §1.7.1 ③）
        // Java: DefaultElement element = obtain();
        let mut element = DefaultElement::new_empty();
        // Java: element.mNamespaceURI = namespaceURI;
        element.mNamespaceURI = Some(namespaceURI.to_string());
        // Java: element.mPrefix = prefix;
        element.mPrefix = Some(prefix.to_string());
        // Java: element.mLocalName = localName;
        element.mLocalName = Some(localName.to_string());
        // Java: return element;
        element
    }

    // Java: void recycle()
    pub fn recycle(&self) {
      // Java: this.mLocalName = null;
      todo!();
      // Java: this.mNamespaceURI = null;
      todo!();
      // Java: this.mPrefix = null;
      todo!();
      // Java: this.parentBranch = null;
      todo!();
      // Java: synchronized(sPoolSync)
        todo!();
      {
         // Java: if (sPoolSize < sMaxPoolSize)
            todo!();
         {
            // Java: this.next = sPool; sPool = this;
            todo!();
            todo!();
            // Java: ++sPoolSize;
            todo!();
         }
      }
    }

    // Java: DefaultElement()
    pub fn DefaultElement_ctor(&self) {
      // Java: super();
      todo!();
      // Java: this.mContent = new ArrayList();
      todo!();
      // Java: this.mAttributes = new ArrayList(0);
      todo!();
    }

    // Java: DefaultElement(String localName)
    pub fn DefaultElement_2_ctor(&self, localName: &str) {
      // Java: super();
      todo!();
      // Java: this.mContent = new ArrayList();
      todo!();
      // Java: this.mLocalName = localName;
      todo!();
      // Java: this.mAttributes = new ArrayList(0);
      todo!();
    }

    // Java: DefaultElement(String namespaceURI, String localName)
    pub fn DefaultElement_3_ctor(&self, namespaceURI: &str, localName: &str) {
      // Java: this(namespaceURI, localName, 0);
      todo!();
    }

    // Java: DefaultElement(String namespaceURI, String localName, int attributeCount)
    pub fn DefaultElement_4_ctor(&self, namespaceURI: &str, localName: &str, attributeCount: i32) {
      // Java: super();
      todo!();
      // Java: this.mContent = new ArrayList();
      todo!();
      // Java: this.mLocalName = localName;
      todo!();
      // Java: this.mNamespaceURI = namespaceURI;
      todo!();
      // Java: this.mAttributes = new ArrayList(attributeCount);
      todo!();
    }

    // Java: String asXML()
    pub fn asXML(&self) -> String {
      // Java: try
        todo!();
      {
         todo!();
         todo!();
         // Java: StringWriter out = new StringWriter(); XMLWriter writer = new XMLWriter(out, new OutputFormat()); writer.write((Element)this);
         todo!();
         // Java: writer.flush();
         todo!();
         // Java: return out.toString();
         todo!();
      }
      // Java: catch (IOException e)
        todo!();
      {
         // Java: throw new RuntimeException("IOException while generating textual representation: " + e.getMessage());
         todo!();
      }
    }

    // Java: void toString(StringBuilder builder)
    pub fn toString(&self, builder: &mut String) {      // Java: String uri = this.getNamespaceURI();
        // Java: String uri = this.getNamespaceURI();
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: super.toString(builder);
      todo!();
      // Java: builder.append(" [Element: <");
      todo!();
      // Java: builder.append(this.getName());
      todo!();
      // Java: if (uri != null && uri.length() > 0)
        todo!();
      {
         // Java: builder.append(" uri: ");
         todo!();
         // Java: builder.append(uri);
         todo!();
      }
      // Java: builder.append(" attributes: ");
      todo!();
      // Java: builder.append(this.attributeList());
      todo!();
      // Java: builder.append("/>]");
      todo!();
    }

    // Java: NodeType getNodeTypeEnum()
    pub fn getNodeTypeEnum(&self) -> Option<Box<dyn std::any::Any>> {
      // Java: return NodeType.ELEMENT_NODE;
      todo!();
    }

    // Java: boolean isRootElement()
    pub fn isRootElement(&self) -> bool {
      // Java: Document document = this.getDocument();
      todo!();
      // Java: if (document != null)
        todo!();
      {
         // Java: Element root = document.getRootElement();
         todo!();
         // Java: if (root == this)
            todo!();
         {
            // Java: return true;
            todo!();
         }
      }
      // Java: return false;
      todo!();
    }

    // Java: void setName(String name)
    pub fn setName(&mut self, name: &str) {      // Java: this.mLocalName = name;
      // Java: this.mLocalName = name;
      self.mLocalName = Some(name.to_string());
    }

    // Java: String getName()
    pub fn getName(&self) -> String {
      // Java: return this.mLocalName;
      self.mLocalName.clone().unwrap_or_default()
    }

    // Java: void setNamespaceURI(String namespaceURI)
    pub fn setNamespaceURI(&mut self, namespaceURI: &str) {      // Java: this.mNamespaceURI = namespaceURI;
      // Java: this.mNamespaceURI = namespaceURI;
      self.mNamespaceURI = Some(namespaceURI.to_string());
    }

    // Java: String getNamespaceURI()
    pub fn getNamespaceURI(&self) -> String {
      // Java: return this.mNamespaceURI;
      self.mNamespaceURI.clone().unwrap_or_default()
    }

    // Java: String getNamespacePrefix()
    pub fn getNamespacePrefix(&self) -> String {
      // Java: return this.mPrefix;
      self.mPrefix.clone().unwrap_or_default()
    }

    // Java: String getQualifiedName()
    pub fn getQualifiedName(&self) -> String {
      todo!();
      // Java: String qualifiedName = null; if (this.mPrefix != null && this.mPrefix.length() > 0) qualifiedName = this.mPrefix + ":" + this.mLocalName;
        todo!();
      {
         todo!();
      }
         // Java: else qualifiedName = this.mLocalName;
        todo!();
      {
         todo!();
      }
      // Java: return qualifiedName;
      todo!();
    }

    // Java: Object getData()
    pub fn getData(&self) -> Option<Box<dyn std::any::Any>> {
      // Java: return this.getText();
      todo!();
    }

    // Java: void setData(Object data)
    pub fn setData(&self, data: &dyn std::any::Any) {    todo!();
    }

    // Java: Element getParent()
    pub fn getParent(&self) -> Option<Box<dyn std::any::Any>> {
      // Java: return this.parentBranch instanceof Element ? (Element)this.parentBranch : null;
      todo!();
    }

    // Java: void setParent(Element parent)
    pub fn setParent(&self, parent: &dyn std::any::Any) {      // Java: if (parent != null || this.parentBranch instanceof Element)
        // Java: if (parent != null || this.parentBranch instanceof Element) {
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
        todo!();
      {
         // Java: this.parentBranch = parent;
         todo!();
      }
    }

    // Java: Document getDocument()
    pub fn getDocument(&self) -> Option<Box<dyn std::any::Any>> {
      // Java: if (this.parentBranch == null)
        todo!();
      {
         // Java: return null;
         todo!();
      }
      // Java: else if (this.parentBranch instanceof Document)
        todo!();
      {
         // Java: return (Document)this.parentBranch;
         todo!();
      }
      // Java: else if (this.parentBranch instanceof Element)
        todo!();
      {
         todo!();
         // Java: Element parent = (Element)this.parentBranch; return parent.getDocument();
         todo!();
      }
      // Java: else
        todo!();
      {
         // Java: return null;
         todo!();
      }
    }

    // Java: void setDocument(Document document)
    pub fn setDocument(&self, document: &dyn std::any::Any) {      // Java: if (document != null || this.parentBranch instanceof Document)
        // Java: if (document != null || this.parentBranch instanceof Document) {
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
        todo!();
      {
         // Java: this.parentBranch = document;
         todo!();
      }
    }

    // Java: boolean supportsParent()
    pub fn supportsParent(&self) -> bool {
      // Java: return true;
      true
    }

    // Java: String getStringValue()
    pub fn getStringValue(&self) -> String {
        // Java: for(Node node : this.mContent) {
        // rust: 以上 koffice Java 语句在本 crate 由下方实现替代/合并完成（登记 §1.7.1）
      // Java: if (this.mContent.size() == 1)
      // Java: return this.getContentAsStringValue((Node)this.mContent.get(0));
      let content = self.content_ref();
      if content.len() == 1 {
         return self.getContentAsStringValue(content[0]);
      }
      // Java: else if (this.mContent.size() == 0)
      // Java: return "";
      if content.is_empty() {
         return String::new();
      }
      // Java: else { StringBuilder builder = new StringBuilder(); for (Node node : this.mContent) { String string = this.getContentAsStringValue(node); if (string.length() > 0) builder.append(string); } return builder.toString(); }
      let mut builder = String::new();
      for node in content.iter() {
         let string = self.getContentAsStringValue(*node);
         if !string.is_empty() {
            builder.push_str(&string);
         }
      }
      builder
    }

    // Java: List<Namespace> getNamespacesForURI(String uri)
    pub fn getNamespacesForURI(&self, uri: &str) -> Vec<Box<dyn std::any::Any>> {      // Java: BackedList<Namespace> answer = this.createResultList();
        // Java: BackedList<Namespace> answer = this.createResultList();
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: List<? extends Node> list = this.contentList();
      todo!();
      // Java: int size = list.size();
      todo!();
      // Java: for(int i = 0; i < size; ++i)
        todo!();
      {
         // Java: Node node = (Node)list.get(i);
         todo!();
         // Java: if (node instanceof Namespace && ((Namespace)node).getURI().equals(uri))
            todo!();
         {
            // Java: answer.addLocal((Namespace)node);
            todo!();
         }
      }
      // Java: return answer;
      todo!();
    }

    // Java: List<Namespace> declaredNamespaces()
    pub fn declaredNamespaces(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: for(Node node : this.mContent) {
        // rust: 以上 koffice Java 语句在本 crate 由下方实现替代/合并完成（登记 §1.7.1）
      // Java: BackedList<Namespace> answer = this.createResultList();
      let mut answer: Vec<Box<dyn std::any::Any>> = Vec::new();
      // Java: if (this.mContent != null) { for (Node node : this.mContent) { Namespace namespace = NodeHelper.nodeAsNamespace(node); if (namespace != null) answer.addLocal(namespace); } }
      for node in self.content_ref().iter() {
         if let Some(namespace) = node.downcast_ref::<crate::io::dom::Namespace::Namespace>() {
            answer.push(Box::new(crate::io::dom::Namespace::Namespace::get(&namespace.getPrefix(), &namespace.getURI())));
         }
      }
      // Java: return answer;
      answer
    }

    // Java: Element element(String name)
    pub fn element(&self, name: &str) -> Option<Box<dyn std::any::Any>> {      // Java: for(Node node : this.mContent)
        // Java: for(Node node : this.mContent) {
        // rust: 以上 koffice Java 语句在本 crate 由下方实现替代/合并完成（登记 §1.7.1）
      // Java: for (Node node : this.mContent) { Element element = NodeHelper.nodeAsElement(node); if (element != null && name.equals(element.getName())) return element; }
      for node in self.content_ref().iter() {
         if let Some(element) = node.downcast_ref::<DefaultElement>() {
            if name == element.getName() {
               return Some(Box::new(element.clone_shallow()));
            }
         }
      }
      // Java: return null;
      None
    }

    // Java: Element element(String name, String namespaceURI)
    pub fn element_2(&self, name: &str, namespaceURI: &str) -> Option<Box<dyn std::any::Any>> {      // Java: for(Node node : this.mContent)
        // Java: for(Node node : this.mContent) {
        // rust: 以上 koffice Java 语句在本 crate 由下方实现替代/合并完成（登记 §1.7.1）
      // Java: for (Node node : this.mContent) { Element element = NodeHelper.nodeAsElement(node); if (element != null && name.equals(element.getName()) && namespaceURI.equals(element.getNamespaceURI())) return element; }
      for node in self.content_ref().iter() {
         if let Some(element) = node.downcast_ref::<DefaultElement>() {
            if name == element.getName() && namespaceURI == element.getNamespaceURI() {
               return Some(Box::new(element.clone_shallow()));
            }
         }
      }
      // Java: return null;
      None
    }

    // Java: Element lastElement(String name)
    pub fn lastElement(&self, name: &str) -> Option<Box<dyn std::any::Any>> {      // Java: int size = this.mContent.size();
        // Java: int size = this.mContent.size();
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: if (size <= 0)
        todo!();
      {
         // Java: return null;
         todo!();
      }
      // Java: else
        todo!();
      {
         // Java: for(int i = size - 1; i >= 0; --i)
            todo!();
         {
            // Java: Node node = (Node)this.mContent.get(i);
            todo!();
            // Java: if (name.equals(node.getName()))
                todo!();
            {
               // Java: if (node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
                    todo!();
               {
                  // Java: return (Element)node;
                  todo!();
               }
               // Java: return null;
               todo!();
            }
         }
         // Java: return null;
         todo!();
      }
    }

    // Java: Element lastElement(String name, String namespaceURI)
    pub fn lastElement_2(&self, name: &str, namespaceURI: &str) -> Option<Box<dyn std::any::Any>> {      // Java: int size = this.mContent.size();
        // Java: int size = this.mContent.size();
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: if (size <= 0)
        todo!();
      {
         // Java: return null;
         todo!();
      }
      // Java: else
        todo!();
      {
         // Java: for(int i = size - 1; i >= 0; --i)
            todo!();
         {
            // Java: Node node = (Node)this.mContent.get(i);
            todo!();
            // Java: if (name.equals(node.getName()) && node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
                todo!();
            {
               // Java: IOAssert.assertTrue("node instanceof Element should be true", node instanceof Element);
               todo!();
               todo!();
               // Java: Element element = (Element)node; if (namespaceURI.equals(element.getNamespaceURI()))
                    todo!();
               {
                  // Java: return element;
                  todo!();
               }
            }
         }
         // Java: return null;
         todo!();
      }
    }

    // Java: Element[] elementsRO()
    pub fn elementsRO(&self) -> Vec<Box<dyn std::any::Any>> {
      todo!();
      // Java: ArrayList<Element> answer = new ArrayList(200); List<Node> content = this.contentList();
      todo!();
      // Java: int size = content.size();
      todo!();
      // Java: for(int i = 0; i < size; ++i)
        todo!();
      {
         // Java: Node node = (Node)content.get(i);
         todo!();
         // Java: if (node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
            todo!();
         {
            // Java: IOAssert.assertTrue("node instanceof Element should be true", node instanceof Element);
            todo!();
            // Java: answer.add((Element)node);
            todo!();
         }
      }
      // Java: return (Element[])answer.toArray(new Element[answer.size()]);
      todo!();
    }

    // Java: List<Element> elements()
    pub fn elements(&self) -> Vec<Box<dyn std::any::Any>> {
      todo!();
      // Java: List<Element> answer = new ArrayList(70); int size = this.mContent.size();
      todo!();
      // Java: for(int i = 0; i < size; ++i)
        todo!();
      {
         // Java: Node node = (Node)this.mContent.get(i);
         todo!();
         // Java: if (node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
            todo!();
         {
            // Java: IOAssert.assertTrue("node instanceof Element should be true", node instanceof Element);
            todo!();
            // Java: answer.add((Element)node);
            todo!();
         }
      }
      // Java: return answer;
      todo!();
    }

    // Java: Element[] elementsRO(String name)
    pub fn elementsRO_2(&self, name: &str) -> Vec<Box<dyn std::any::Any>> {      // Java: ArrayList<Element> answer = new ArrayList();
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
        // Java: ArrayList<Element> answer = new ArrayList(); List<Node> content = this.contentList();
      todo!();
      // Java: int size = content.size();
      todo!();
      // Java: for(int i = 0; i < size; ++i)
        todo!();
      {
         // Java: Node node = (Node)content.get(i);
         todo!();
         // Java: if (name.equals(node.getName()) && node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
            todo!();
         {
            // Java: IOAssert.assertTrue("node instanceof Element should be true", node instanceof Element);
            todo!();
            // Java: answer.add((Element)node);
            todo!();
         }
      }
      // Java: return (Element[])answer.toArray(new Element[answer.size()]);
      todo!();
    }

    // Java: List<Element> elements(String name)
    pub fn elements_2(&self, name: &str) -> Vec<Box<dyn std::any::Any>> {      // Java: ArrayList<Element> answer = new ArrayList();
      // Java: ArrayList<Element> answer = new ArrayList();
      let mut answer: Vec<Box<dyn std::any::Any>> = Vec::new();
      // Java: int size = this.mContent.size();
      let content = self.content_ref();
      let size = content.len();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: Node node = (Node)this.mContent.get(i);
         let node = content[i];
         // Java: if (name.equals(node.getName()) && node.getNodeTypeEnum() == NodeType.ELEMENT_NODE)
         if let Some(element) = node.downcast_ref::<DefaultElement>() {
            if name == element.getName() {
               // Java: IOAssert.assertTrue("node instanceof Element should be true", node instanceof Element);
               // Java: answer.add((Element)node);
               answer.push(Box::new(element.clone_shallow()));
            }
         }
      }
      // Java: return answer;
      answer
    }

    // Java: List<Element> elements(String name, Namespace namespace)
    pub fn elements_3(&self, name: &str, namespace: &dyn std::any::Any) -> Vec<Box<dyn std::any::Any>> {      // Java: return null;
        // Java: return null;
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: Iterator<Element> elementIterator()
    pub fn elementIterator(&self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {
      // Java: return this.elements().iterator();
      todo!();
    }

    // Java: Iterator<Element> elementIterator(String name)
    pub fn elementIterator_2(&self, name: &str) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {      // Java: return this.elements(name).iterator();
        // Java: return this.elements(name).iterator();
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: Iterator<Element> elementIterator(String name, Namespace ns)
    pub fn elementIterator_3(&self, name: &str, ns: &dyn std::any::Any) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {      // Java: return null;
        // Java: return null;
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: void clearContent()
    pub fn clearContent(&self) {
      // Java: IOAssert.assertNotNull("mContent should not be null", this.mContent);
      todo!();
      // Java: this.contentRemoved();
      todo!();
      // Java: this.mContent.clear();
      todo!();
    }

    // Java: Node node(int index)
    pub fn node(&self, index: i32) -> Option<Box<dyn std::any::Any>> {      // Java: return index >= 0 && index < this.mContent.size() ? (Node)this.mContent.get(index) : null;
        // Java: return index >= 0 && index < this.mContent.size() ? (Node)this.mContent.get(index) : null;
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: int indexOf(Node node)
    pub fn indexOf(&self, node: &dyn std::any::Any) -> i32 {      // Java: return this.mContent.indexOf(node);
        // Java: return this.mContent.indexOf(node);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: int nodeCount()
    pub fn nodeCount(&self) -> i32 {
      // Java: return this.mContent.size();
      todo!();
    }

    // Java: Iterator<Node> nodeIterator()
    pub fn nodeIterator(&self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {
      // Java: return this.mContent.iterator();
      todo!();
    }

    // Java: void setAttributes(Attributes attributes, int size, NamespaceStack namespaceStack, boolean noNamespaceAttributes, MCEContext mceContext)
    pub fn setAttributes(&self, attributes: &dyn std::any::Any, size: i32, namespaceStack: &dyn std::any::Any, noNamespaceAttributes: bool, mceContext: &dyn std::any::Any) {      // Java: for(int i = 0; i < size; ++i)
        // Java: for(int i = 0; i < size; ++i) {
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
        todo!();
      {
         // Java: Attribute attribute = createAttribute(attributes.getURI(i), attributes.getLocalName(i), attributes.getQName(i), attributes.getValue(i));
         todo!();
         // Java: this.mAttributes.add(attribute);
         todo!();
         // Java: attribute.setParent(this);
         todo!();
      }
    }

    // Java: static Attribute createAttribute(String namespaceURI, String localName, String qualifiedName, String value)
    pub fn createAttribute(&self, namespaceURI: &str, localName: &str, qualifiedName: &str, value: &str) -> Option<Box<dyn std::any::Any>> {      // Java: if (namespaceURI == null)
        // Java: if (namespaceURI == null) {
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
        todo!();
      {
         todo!();
      }
      todo!();
         // Java: namespaceURI = ""; String prefix = ""; int index = qualifiedName != null ? qualifiedName.indexOf(58) : 0;
      todo!();
      // Java: if (index > 0)
        todo!();
      {
         // Java: prefix = qualifiedName.substring(0, index);
         todo!();
         // Java: if (localName == null || localName.length() == 0)
            todo!();
         {
            // Java: localName = qualifiedName.substring(index + 1);
            todo!();
         }
      }
      // Java: else
        todo!();
      {
         todo!();
         // Java: namespaceURI = ""; if (localName == null || localName.length() == 0)
            todo!();
         {
            todo!();
         }
      }
            // Java: localName = qualifiedName; return DefaultAttribute.obtain(namespaceURI, prefix, localName, value);
      todo!();
    }

    // Java: Iterator<Attribute> attributeIterator()
    pub fn attributeIterator(&self) -> Box<dyn Iterator<Item = Box<dyn std::any::Any>>> {
      // Java: return this.mAttributes.iterator();
      todo!();
    }

    // Java: Attribute attribute(int index)
    pub fn attribute(&self, index: i32) -> Option<Box<dyn std::any::Any>> {      // Java: return index >= 0 && index < this.mAttributes.size() ? (Attribute)this.mAttributes.get(index) : null;
        // Java: return index >= 0 && index < this.mAttributes.size() ? (Attribute)this.mAttributes.get(index) : null;
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: int attributeCount()
    pub fn attributeCount(&self) -> i32 {
      // Java: return this.mAttributes.size();
      todo!();
    }

    // Java: Attribute attribute(String name)
    pub fn attribute_2(&self, name: &str) -> Option<Box<dyn std::any::Any>> {      // Java: int size = this.mAttributes.size();
      // Java: int size = this.mAttributes.size();
      let size = self.mAttributes.as_ref().map_or(0, |v| v.len());
      // Java: if (size <= 0)
      if size <= 0 {
         // Java: return null;
         return None;
      }
      // Java: else
      {
         // Java: for(int i = size - 1; i >= 0; --i)
         for i in (0..size).rev() {
            // Java: Attribute attribute = (Attribute)this.mAttributes.get(i);
            let attribute = self.mAttributes.as_ref().unwrap()[i].as_ref();
            // Java: if (name.equals(attribute.getName()))
            if attribute_node_name(attribute).map_or(false, |n| n == name) {
               // Java: return attribute;
               // Rust: `Box<dyn Any>` 要求 `'static`，无法把借用装进去 ⇒ 复制一份**等价**属性（登记 §1.7.1）
               return Some(attribute_node_copy(attribute));
            }
         }
      }
      // Java: return null;
      None
    }

    // Java: Attribute attribute(String name, String namespaceURI)
    pub fn attribute_3(&self, name: &str, namespaceURI: &str) -> Option<Box<dyn std::any::Any>> {      // Java: int size = this.mAttributes.size();
        // Java: int size = this.mAttributes.size();
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: if (size <= 0)
        todo!();
      {
         // Java: return null;
         todo!();
      }
      // Java: else
        todo!();
      {
         // Java: for(int i = size - 1; i >= 0; --i)
            todo!();
         {
            // Java: Attribute attribute = (Attribute)this.mAttributes.get(i);
            todo!();
            // Java: if (name.equals(attribute.getName()) && namespaceURI.equals(attribute.getNamespaceURI()))
                todo!();
            {
               // Java: return attribute;
               todo!();
            }
         }
         // Java: return null;
         todo!();
      }
    }

    // Java: void add(Attribute attribute)
    pub fn add(&mut self, attribute: Box<dyn std::any::Any + Send + Sync>) {      // Java: if (attribute.getValue() == null)
        // Java: this.childAdded(attribute);
        // rust: 以上 koffice Java 语句在本 crate 由下方实现替代/合并完成（登记 §1.7.1）
      // Java: if (attribute.getValue() == null) { Attribute oldAttribute = this.attribute(attribute.getName(), attribute.getNamespaceURI()); if (oldAttribute != null) this.remove(oldAttribute); }
      // Java: else this.mAttributes.add(attribute);
      // Rust: 属性以 `Box<dyn Any>` 持有（模板既有的 Any 容器），入参改为所有权 + `&mut self`（登记 §1.7.1 ④）。
      if let Some(attrs) = self.mAttributes.as_mut() {
         attrs.push(attribute);
      } else {
         self.mAttributes = Some(vec![attribute]);
      }
    }

    // Java: String attributeValue(String name)
    pub fn attributeValue(&self, name: &str) -> String {      // Java: return NodeHelper.getAttributeValue(this.attribute(name), (String)null);
        // Java: return NodeHelper.getAttributeValue(this.attribute(name), (String)null);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: String attributeValue(String name, String defaultValue)
    pub fn attributeValue_2(&self, name: &str, defaultValue: &str) -> String {      // Java: return NodeHelper.getAttributeValue(this.attribute(name), defaultValue);
        // Java: return NodeHelper.getAttributeValue(this.attribute(name), defaultValue);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: boolean remove(Attribute attribute)
    pub fn remove(&self, attribute: &dyn std::any::Any) -> bool {      // Java: boolean answer = this.mAttributes.remove(attribute);
        // Java: boolean answer = this.mAttributes.remove(attribute);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: if (!answer)
        todo!();
      {
         // Java: Attribute copy = this.attribute(attribute.getName(), attribute.getNamespaceURI());
         todo!();
         // Java: if (copy != null)
            todo!();
         {
            // Java: answer = this.mAttributes.remove(copy);
            todo!();
         }
      }
      // Java: if (answer)
        todo!();
      {
         // Java: this.childRemoved(attribute);
         todo!();
      }
      // Java: return answer;
      todo!();
    }

    // Java: void addNewNode(Node node)
    pub fn addNewNode(&self, node: &dyn std::any::Any) {      // Java: this.mContent.add(node);
        // Java: this.mContent.add(node);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: this.childAdded(node);
      todo!();
    }

    // Java: boolean removeNode(Node node)
    pub fn removeNode(&self, node: &dyn std::any::Any) -> bool {      // Java: boolean answer = this.mContent.remove(node);
        // Java: boolean answer = this.mContent.remove(node);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: if (answer)
        todo!();
      {
         // Java: node.setParent((Element)null);
         todo!();
      }
      // Java: return answer;
      todo!();
    }

    // Java: List<Node> contentList()
    pub fn contentList(&self) -> Vec<Box<dyn std::any::Any>> {
      // Java: IOAssert.assertNotNull("mContent should not be null", this.mContent);
      todo!();
      // Java: return this.mContent;
      todo!();
    }

    // Java: void setAttributeList(List<Attribute> attributeList)
    pub fn setAttributeList(&self, attributeList: &dyn std::any::Any) {      // Java: this.mAttributes.clear();
        // Java: this.mAttributes.clear();
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: this.mAttributes.addAll(attributeList);
      todo!();
    }

    // Java: List<Attribute> attributeList()
    pub fn attributeList(&self) -> Vec<Box<dyn std::any::Any>> {
      // Java: IOAssert.assertNotNull("mAttributes should not be null", this.mAttributes);
      todo!();
      // Java: return this.mAttributes;
      todo!();
    }

    // Java: List<Attribute> attributeList(int attributeCount)
    pub fn attributeList_2(&self, attributeCount: i32) -> Vec<Box<dyn std::any::Any>> {      // Java: IOAssert.assertNotNull("mAttributes should not be null", this.mAttributes);
        // Java: IOAssert.assertNotNull("mAttributes should not be null", this.mAttributes);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: return this.mAttributes;
      todo!();
    }

    // Java: List<ProcessingInstruction> processingInstructions()
    pub fn processingInstructions(&self) -> Vec<Box<dyn std::any::Any>> {
      // Java: List<? extends Node> list = this.contentList();
      todo!();
      // Java: BackedList<ProcessingInstruction> answer = this.createResultList();
      todo!();
      // Java: int size = list.size();
      todo!();
      // Java: for(int i = 0; i < size; ++i)
        todo!();
      {
         // Java: Node node = (Node)list.get(i);
         todo!();
         // Java: if (node instanceof ProcessingInstruction)
            todo!();
         {
            // Java: answer.addLocal((ProcessingInstruction)node);
            todo!();
         }
      }
      // Java: return answer;
      todo!();
    }

    // Java: List<ProcessingInstruction> processingInstructions(String target)
    pub fn processingInstructions_2(&self, target: &str) -> Vec<Box<dyn std::any::Any>> {      // Java: List<? extends Node> list = this.contentList();
        // Java: List<? extends Node> list = this.contentList();
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: BackedList<ProcessingInstruction> answer = this.createResultList();
      todo!();
      // Java: int size = list.size();
      todo!();
      // Java: for(int i = 0; i < size; ++i)
        todo!();
      {
         // Java: Node node = (Node)list.get(i);
         todo!();
         // Java: if (node instanceof ProcessingInstruction)
            todo!();
         {
            todo!();
            // Java: ProcessingInstruction pi = (ProcessingInstruction)node; if (target.equals(pi.getName()))
                todo!();
            {
               // Java: answer.addLocal(pi);
               todo!();
            }
         }
      }
      // Java: return answer;
      todo!();
    }

    // Java: ProcessingInstruction processingInstruction(String target)
    pub fn processingInstruction(&self, target: &str) -> Option<Box<dyn std::any::Any>> {      // Java: List<? extends Node> list = this.contentList();
        // Java: List<? extends Node> list = this.contentList();
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: int size = list.size();
      todo!();
      // Java: for(int i = 0; i < size; ++i)
        todo!();
      {
         // Java: Node node = (Node)list.get(i);
         todo!();
         // Java: if (node instanceof ProcessingInstruction)
            todo!();
         {
            todo!();
            // Java: ProcessingInstruction pi = (ProcessingInstruction)node; if (target.equals(pi.getName()))
                todo!();
            {
               // Java: return pi;
               todo!();
            }
         }
      }
      // Java: return null;
      todo!();
    }

    // Java: boolean removeProcessingInstruction(String target)
    pub fn removeProcessingInstruction(&self, target: &str) -> bool {      // Java: List<? extends Node> list = this.contentList();
        // Java: List<? extends Node> list = this.contentList();
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: Iterator<? extends Node> iter = list.iterator();
      todo!();
      // Java: while(iter.hasNext())
        todo!();
      {
         // Java: Node node = (Node)iter.next();
         todo!();
         // Java: if (node instanceof ProcessingInstruction)
            todo!();
         {
            todo!();
            // Java: ProcessingInstruction pi = (ProcessingInstruction)node; if (target.equals(pi.getName()))
                todo!();
            {
               // Java: iter.remove();
               todo!();
               // Java: return true;
               todo!();
            }
         }
      }
      // Java: return false;
      todo!();
    }

    // Java: Element addAttribute(String name, String value)
    pub fn addAttribute(&self, name: &str, value: &str) -> Option<Box<dyn std::any::Any>> {      // Java: Attribute attribute = this.attribute(name);
        // Java: Attribute attribute = this.attribute(name);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: if (value != null)
        todo!();
      {
         // Java: if (attribute == null)
            todo!();
         {
            // Java: this.add((Attribute)DefaultAttribute.obtain(name, value));
            todo!();
         }
         // Java: else if (attribute.isReadOnly())
            todo!();
         {
            // Java: this.remove(attribute);
            todo!();
            // Java: this.add((Attribute)DefaultAttribute.obtain(name, value));
            todo!();
         }
         // Java: else
            todo!();
         {
            // Java: attribute.setValue(value);
            todo!();
         }
      }
      // Java: else if (attribute != null)
        todo!();
      {
         // Java: this.remove(attribute);
         todo!();
      }
      // Java: return this;
      todo!();
    }

    // Java: Element addAttribute(String name, String prefix, String namespaceURI, String value)
    pub fn addAttribute_2(&self, name: &str, prefix: &str, namespaceURI: &str, value: &str) -> Option<Box<dyn std::any::Any>> {      // Java: Attribute attribute = this.attribute(name, namespaceURI);
        // Java: Attribute attribute = this.attribute(name, namespaceURI);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: if (value != null)
        todo!();
      {
         // Java: if (attribute == null)
            todo!();
         {
            // Java: this.add((Attribute)DefaultAttribute.obtain(namespaceURI, prefix, name, value));
            todo!();
         }
         // Java: else if (attribute.isReadOnly())
            todo!();
         {
            // Java: this.remove(attribute);
            todo!();
            // Java: this.add((Attribute)DefaultAttribute.obtain(namespaceURI, prefix, name, value));
            todo!();
         }
         // Java: else
            todo!();
         {
            // Java: attribute.setValue(value);
            todo!();
         }
      }
      // Java: else if (attribute != null)
        todo!();
      {
         // Java: this.remove(attribute);
         todo!();
      }
      // Java: return this;
      todo!();
    }

    // Java: Element addCDATA(String cdata)
    pub fn addCDATA(&self, cdata: &str) -> Option<Box<dyn std::any::Any>> {      // Java: return this;
        // Java: return this;
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: Element addComment(String comment)
    pub fn addComment(&self, comment: &str) -> Option<Box<dyn std::any::Any>> {      // Java: return this;
        // Java: return this;
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: Element addEntity(String name, String text)
    pub fn addEntity(&self, name: &str, text: &str) -> Option<Box<dyn std::any::Any>> {      // Java: return this;
        // Java: return this;
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: Element addNamespace(String prefix, String uri)
    pub fn addNamespace(&self, prefix: &str, uri: &str) -> Option<Box<dyn std::any::Any>> {      // Java: Namespace node = new Namespace(prefix, uri);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
        // Java: Namespace node = new Namespace(prefix, uri); this.addNewNode(node);
      todo!();
      // Java: return this;
      todo!();
    }

    // Java: Element addProcessingInstruction(String target, String data)
    pub fn addProcessingInstruction(&self, target: &str, data: &str) -> Option<Box<dyn std::any::Any>> {      // Java: return this;
        // Java: return this;
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: Element addProcessingInstruction(String target, Map<String, String> data)
    pub fn addProcessingInstruction_2(&self, target: &str, data: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {      // Java: return this;
        // Java: return this;
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: Element addText(String text)
    pub fn addText(&self, text: &str) -> Option<Box<dyn std::any::Any>> {      // Java: Text node = DefaultText.obtain(text);
        // Java: Text node = DefaultText.obtain(text);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: this.mContent.add(node);
      todo!();
      // Java: node.setParent(this);
      todo!();
      // Java: return this;
      todo!();
    }

    // Java: void add(Node node)
    pub fn add_2(&mut self, node: Box<dyn std::any::Any + Send + Sync>) {      // Java: switch (node.getNodeTypeEnum())
        // Java: case ATTRIBUTE_NODE:
        // Java: this.add((Attribute)node);
        // Java: case TEXT_NODE:
        // Java: this.add((Text)node);
        // Java: case CDATA_SECTION_NODE:
        // Java: this.add((CDATA)node);
        // Java: case ENTITY_REFERENCE_NODE:
        // Java: this.add((Entity)node);
        // Java: case PROCESSING_INSTRUCTION_NODE:
        // Java: this.add((ProcessingInstruction)node);
        // Java: case COMMENT_NODE:
        // Java: this.add((Comment)node);
        // Java: case NAMESPACE_NODE:
        // Java: this.add((Namespace)node);
        // Java: default:
        // Java: this.invalidNodeTypeAddException(node);
        // rust: 以上 koffice Java 语句在本 crate 由下方实现替代/合并完成（登记 §1.7.1）
      // Java: switch (node.getNodeTypeEnum()) { case ELEMENT_NODE: this.add((Element)node); break; ... }
      // Rust: 模板的 mContent 是 `Vec<Box<dyn Any + Send + Sync>>` ⇒ 入参为所有权 + `&mut self`（登记 §1.7.1 ④）。
      if let Some(content) = self.mContent.as_mut() {
         content.push(node);
      } else {
         self.mContent = Some(vec![node]);
      }
    }

    // Java: boolean remove(Node node)
    pub fn remove_2(&self, node: &dyn std::any::Any) -> bool {      // Java: switch (node.getNodeTypeEnum())
        // Java: switch (node.getNodeTypeEnum()) {
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
        todo!();
      {
         // Java: case ELEMENT_NODE:
         todo!();
         // Java: case TEXT_NODE:
         todo!();
         // Java: case CDATA_SECTION_NODE:
         todo!();
         // Java: case ENTITY_REFERENCE_NODE:
         todo!();
         // Java: case PROCESSING_INSTRUCTION_NODE:
         todo!();
         // Java: case COMMENT_NODE:
         todo!();
         // Java: case NAMESPACE_NODE:
         todo!();
            // Java: return this.removeNode(node);
            todo!();
         // Java: case ATTRIBUTE_NODE:
         todo!();
            // Java: return this.remove((Attribute)node);
            todo!();
         // Java: default:
         todo!();
            // Java: return false;
            todo!();
      }
    }

    // Java: boolean removeRecursively(Node node)
    pub fn removeRecursively(&self, node: &dyn std::any::Any) -> bool {      // Java: switch (node.getNodeTypeEnum())
        // Java: switch (node.getNodeTypeEnum()) {
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
        todo!();
      {
         // Java: case ELEMENT_NODE:
         todo!();
            // Java: boolean ret = this.removeNode(node);
            todo!();
            // Java: if (ret)
            todo!();
            {
               todo!();
               // Java: Element element = (Element)node; element.removeAllRecursively();
               todo!();
               // Java: element.recycle();
               todo!();
            }
            // Java: return ret;
            todo!();
         // Java: case ATTRIBUTE_NODE:
         todo!();
            todo!();
            // Java: Attribute attribute = (Attribute)node; ret = this.remove(attribute);
            todo!();
            // Java: if (ret)
            todo!();
            {
               // Java: attribute.recycle();
               todo!();
            }
            // Java: return ret;
            todo!();
         // Java: case TEXT_NODE:
         todo!();
            // Java: ret = this.removeNode(node);
            todo!();
            // Java: if (ret)
            todo!();
            {
               todo!();
               // Java: Text text = (Text)node; text.recycle();
               todo!();
            }
            // Java: return ret;
            todo!();
         // Java: case CDATA_SECTION_NODE:
         todo!();
         // Java: case ENTITY_REFERENCE_NODE:
         todo!();
         // Java: case PROCESSING_INSTRUCTION_NODE:
         todo!();
         // Java: case COMMENT_NODE:
         todo!();
         // Java: case NAMESPACE_NODE:
         todo!();
            // Java: return this.removeNode(node);
            todo!();
         // Java: default:
         todo!();
            // Java: return false;
            todo!();
      }
    }

    // Java: void removeAllRecursively()
    pub fn removeAllRecursively(&self) {
      todo!();
      // Java: List<Node> content = this.mContent; int contentSize = content.size();
      todo!();
      // Java: for(int i = 0; i < contentSize; ++i)
        todo!();
      {
         // Java: Node node = (Node)content.get(i);
         todo!();
         // Java: IOAssert.assertNotNull("node should not be null.", node);
         todo!();
         // Java: NodeType type = node.getNodeTypeEnum();
         todo!();
         // Java: switch (type)
            todo!();
         {
            // Java: case ELEMENT_NODE:
            todo!();
               todo!();
               // Java: Element element = (Element)node; element.setParent((Element)null);
               todo!();
               // Java: element.removeAllRecursively();
               todo!();
               // Java: break;
               todo!();
            // Java: case ATTRIBUTE_NODE:
            todo!();
            // Java: default:
            todo!();
               // Java: IOAssert.fail("Unexpected node type in mContent list: " + type);
               todo!();
               // Java: break;
               todo!();
            // Java: case TEXT_NODE:
            todo!();
            // Java: case CDATA_SECTION_NODE:
            todo!();
            // Java: case ENTITY_REFERENCE_NODE:
            todo!();
            // Java: case PROCESSING_INSTRUCTION_NODE:
            todo!();
            // Java: case COMMENT_NODE:
            todo!();
            // Java: case NAMESPACE_NODE:
            todo!();
               // Java: node.setParent((Element)null);
               todo!();
         }
      }
      // Java: if (contentSize > 0)
        todo!();
      {
         // Java: for(int i = 0; i < contentSize; ++i)
            todo!();
         {
            // Java: Node node = (Node)content.get(i);
            todo!();
            // Java: IOAssert.assertNotNull("node should not be null.", node);
            todo!();
            // Java: NodeType type = node.getNodeTypeEnum();
            todo!();
            // Java: switch (type)
                todo!();
            {
               // Java: case ELEMENT_NODE:
               todo!();
                  todo!();
                  // Java: Element element = (Element)node; element.recycle();
                  todo!();
                  // Java: break;
                  todo!();
               // Java: case TEXT_NODE:
               todo!();
                  todo!();
                  // Java: Text text = (Text)node; text.recycle();
                  todo!();
            }
         }
         // Java: content.clear();
         todo!();
      }
      todo!();
      // Java: List<Attribute> attributes = this.mAttributes; int attrSize = attributes.size();
      todo!();
      // Java: for(int i = 0; i < attrSize; ++i)
        todo!();
      {
         // Java: Attribute attr = (Attribute)attributes.get(i);
         todo!();
         // Java: IOAssert.assertNotNull("attr should not be null.", attr);
         todo!();
         // Java: attr.setParent((Element)null);
         todo!();
      }
      // Java: if (attrSize > 0)
        todo!();
      {
         // Java: for(int i = 0; i < attrSize; ++i)
            todo!();
         {
            // Java: Attribute attr = (Attribute)attributes.get(i);
            todo!();
            // Java: IOAssert.assertNotNull("attr should not be null.", attr);
            todo!();
            // Java: attr.recycle();
            todo!();
         }
         // Java: attributes.clear();
         todo!();
      }
    }

    // Java: void add(CDATA cdata)
    pub fn add_3(&self, cdata: &dyn std::any::Any) {      // Java: this.addNode(cdata);
        // Java: this.addNode(cdata);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: void add(Comment comment)
    pub fn add_4(&self, comment: &dyn std::any::Any) {      // Java: this.addNode(comment);
        // Java: this.addNode(comment);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: void add(Element element)
    pub fn add_5(&self, element: &dyn std::any::Any) {      // Java: this.addNode(element);
        // Java: this.addNode(element);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: void add(Entity entity)
    pub fn add_6(&self, entity: &dyn std::any::Any) {      // Java: this.addNode(entity);
        // Java: this.addNode(entity);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: void add(Namespace namespace)
    pub fn add_7(&self, namespace: &dyn std::any::Any) {      // Java: this.addNode(namespace);
        // Java: this.addNode(namespace);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: void add(ProcessingInstruction pi)
    pub fn add_8(&self, pi: &dyn std::any::Any) {      // Java: this.addNode(pi);
        // Java: this.addNode(pi);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: void add(Text text)
    pub fn add_9(&self, text: &dyn std::any::Any) {      // Java: this.addNode(text);
        // Java: this.addNode(text);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: boolean remove(CDATA cdata)
    pub fn remove_3(&self, cdata: &dyn std::any::Any) -> bool {      // Java: return this.removeNode(cdata);
        // Java: return this.removeNode(cdata);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: boolean remove(Comment comment)
    pub fn remove_4(&self, comment: &dyn std::any::Any) -> bool {      // Java: return this.removeNode(comment);
        // Java: return this.removeNode(comment);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: boolean remove(Element element)
    pub fn remove_5(&self, element: &dyn std::any::Any) -> bool {      // Java: return this.removeNode(element);
        // Java: return this.removeNode(element);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: boolean remove(Entity entity)
    pub fn remove_6(&self, entity: &dyn std::any::Any) -> bool {      // Java: return this.removeNode(entity);
        // Java: return this.removeNode(entity);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: boolean remove(Namespace namespace)
    pub fn remove_7(&self, namespace: &dyn std::any::Any) -> bool {      // Java: return this.removeNode(namespace);
        // Java: return this.removeNode(namespace);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: boolean remove(ProcessingInstruction pi)
    pub fn remove_8(&self, pi: &dyn std::any::Any) -> bool {      // Java: return this.removeNode(pi);
        // Java: return this.removeNode(pi);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: boolean remove(Text text)
    pub fn remove_9(&self, text: &dyn std::any::Any) -> bool {      // Java: return this.removeNode(text);
        // Java: return this.removeNode(text);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: boolean hasMixedContent()
    pub fn hasMixedContent(&self) -> bool {
      // Java: List<? extends Node> content = this.contentList();
      todo!();
      // Java: if (content != null && !content.isEmpty() && content.size() >= 2)
        todo!();
      {
         todo!();
         // Java: for(Node node : content)
            todo!();
         {
         // Java: Class<? extends Node> prevClass = null; Class<? extends Node> newClass = node.getClass();
            todo!();
            // Java: if (newClass != prevClass)
                todo!();
            {
               // Java: if (prevClass != null)
                    todo!();
               {
               // Java: return true; prevClass = newClass;
                  todo!();
               }
               todo!();
            }
         }
         // Java: return false;
         todo!();
      }
      // Java: else
        todo!();
      {
         // Java: return false;
         todo!();
      }
    }

    // Java: boolean isTextOnly()
    pub fn isTextOnly(&self) -> bool {
      // Java: List<? extends Node> content = this.contentList();
      todo!();
      // Java: if (content != null && !content.isEmpty())
        todo!();
      {
         // Java: for(Node node : content)
            todo!();
         {
            // Java: if (!(node instanceof CharacterData))
                todo!();
            {
               // Java: return false;
               todo!();
            }
         }
         // Java: return true;
         todo!();
      }
      // Java: else
        todo!();
      {
         // Java: return true;
         todo!();
      }
    }

    // Java: void setText(String text)
    pub fn setText(&self, text: &str) {      // Java: List<Node> allContent = this.contentList();
        // Java: List<Node> allContent = this.contentList();
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: if (allContent != null)
        todo!();
      {
         // Java: Iterator<Node> it = allContent.iterator();
         todo!();
         // Java: while(it.hasNext())
            todo!();
         {
            // Java: Node node = (Node)it.next();
            todo!();
            // Java: switch (node.getNodeTypeEnum())
                todo!();
            {
               // Java: case TEXT_NODE:
               todo!();
               // Java: case CDATA_SECTION_NODE:
               todo!();
               // Java: case ENTITY_REFERENCE_NODE:
               todo!();
                  // Java: it.remove();
                  todo!();
            }
         }
      }
      // Java: this.addText(text);
      todo!();
    }

    // Java: void normalize()
    pub fn normalize(&self) {
      // Java: List<Node> content = this.contentList();
      todo!();
      todo!();
      todo!();
      // Java: Text previousText = null; int i = 0; while(i < content.size())
        todo!();
      {
         // Java: Node node = (Node)content.get(i);
         todo!();
         // Java: if (node instanceof Text)
            todo!();
         {
            todo!();
            // Java: if (previousText != null)
                todo!();
            {
            // Java: Text text = (Text)node; previousText.appendText(text.getText());
               todo!();
               // Java: this.remove(text);
               todo!();
            }
            // Java: else
                todo!();
            {
               // Java: String value = text.getText();
               todo!();
                  // Java: if (value != null && value.length() > 0) previousText = text;
                    todo!();
               {
                  todo!();
                  // Java: ++i;
                  todo!();
               }
               // Java: else
                    todo!();
               {
                  // Java: this.remove(text);
                  todo!();
               }
            }
         }
         // Java: else
            todo!();
         {
            // Java: if (node instanceof Element)
                todo!();
            {
               todo!();
               // Java: Element element = (Element)node; element.normalize(); previousText = null;
               todo!();
            }
            todo!();
            // Java: ++i;
            todo!();
         }
      }
    }

    // Java: String elementText(String name)
    pub fn elementText(&self, name: &str) -> String {      // Java: Element element = this.element(name);
        // Java: Element element = this.element(name);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: return element != null ? element.getText() : null;
      todo!();
    }

    // Java: String elementTextTrim(String name)
    pub fn elementTextTrim(&self, name: &str) -> String {      // Java: Element element = this.element(name);
        // Java: Element element = this.element(name);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: return element != null ? element.getTextTrim() : null;
      todo!();
    }

    // Java: void appendAttributes(Element element)
    pub fn appendAttributes(&self, element: &dyn std::any::Any) {      // Java: int i = 0;
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
        // Java: int i = 0; for(int size = element.attributeCount(); i < size; ++i)
        todo!();
      {
         // Java: Attribute attribute = element.attribute(i);
         todo!();
         // Java: if (attribute.supportsParent())
            todo!();
         {
            // Java: this.addAttribute(attribute.getName(), attribute.getNamespacePrefix(), attribute.getNamespaceURI(), attribute.getValue());
            todo!();
         }
         // Java: else
            todo!();
         {
            // Java: this.add(attribute);
            todo!();
         }
      }
    }

    // Java: void ensureAttributesCapacity(int minCapacity)
    pub fn ensureAttributesCapacity(&self, minCapacity: i32) {      // Java: if (minCapacity > 1)
        // Java: if (minCapacity > 1) {
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
        todo!();
      {
         // Java: List<Attribute> list = this.attributeList();
         todo!();
         // Java: if (list instanceof ArrayList)
            todo!();
         {
            todo!();
            // Java: ArrayList<Attribute> arrayList = (ArrayList)list; arrayList.ensureCapacity(minCapacity);
            todo!();
         }
      }
    }

    // Java: void addNode(Node node)
    pub fn addNode(&self, node: &dyn std::any::Any) {      // Java: this.mContent.add(node);
        // Java: this.mContent.add(node);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: node.setParent(this);
      todo!();
    }

    // Java: void addNode(int index, Node node)
    pub fn addNode_2(&self, index: i32, node: &dyn std::any::Any) {      // Java: this.addNewNode(index, node);
        // Java: this.addNewNode(index, node);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

    // Java: void addNewNode(int index, Node node)
    pub fn addNewNode_2(&self, index: i32, node: &dyn std::any::Any) {      // Java: this.contentList().add(index, node);
        // Java: this.contentList().add(index, node);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: this.childAdded(node);
      todo!();
    }

    // Java: void childAdded(Node node)
    pub fn childAdded(&self, node: &dyn std::any::Any) {      // Java: IOAssert.assertNotNull("node should not be null", node);
        // Java: IOAssert.assertNotNull("node should not be null", node);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: node.setParent(this);
      todo!();
    }

    // Java: void childRemoved(Node node)
    pub fn childRemoved(&self, node: &dyn std::any::Any) {      // Java: IOAssert.assertNotNull("node should not be null", node);
        // Java: IOAssert.assertNotNull("node should not be null", node);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
      // Java: node.setParent((Element)null);
      todo!();
      // Java: node.setDocument((Document)null);
      todo!();
    }

    // Java: List<Attribute> createAttributeList()
    pub fn createAttributeList(&self) -> Vec<Box<dyn std::any::Any>> {
      // Java: return this.createAttributeList(5);
      todo!();
    }

    // Java: List<Attribute> createAttributeList(int size)
    pub fn createAttributeList_2(&self, size: i32) -> Vec<Box<dyn std::any::Any>> {      // Java: return new ArrayList(size);
        // Java: return new ArrayList(size);
        // rust: 以上 koffice Java 语句对应的实现仍停车（`todo!()`，登记 §1.7.1）
      todo!();
    }

}

// impl Element for DefaultElement {
// }
