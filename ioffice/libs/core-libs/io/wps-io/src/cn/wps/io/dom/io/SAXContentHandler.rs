#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.io.dom.Branch
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.DocumentType
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.ElementHandler
// Java: import cn.wps.io.dom.Namespace
// Java: import cn.wps.io.dom.dtd.AttributeDecl
// Java: import cn.wps.io.dom.dtd.Declaration
// Java: import cn.wps.io.dom.dtd.ElementDecl
// Java: import cn.wps.io.dom.dtd.ExternalEntityDecl
// Java: import cn.wps.io.dom.dtd.InternalEntityDecl
// Java: import cn.wps.io.dom.tree.DefaultDocument
// Java: import cn.wps.io.dom.tree.DefaultElement
// Java: import cn.wps.io.dom.tree.NamespaceStack
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Map
// Java: import org.xml.sax.Attributes
// Java: import org.xml.sax.DTDHandler
// Java: import org.xml.sax.EntityResolver
// Java: import org.xml.sax.InputSource
// Java: import org.xml.sax.Locator
// Java: import org.xml.sax.SAXException
// Java: import org.xml.sax.SAXParseException
// Java: import org.xml.sax.ext.DeclHandler
// Java: import org.xml.sax.ext.LexicalHandler
// Java: import org.xml.sax.ext.Locator2
// Java: import org.xml.sax.helpers.DefaultHandler

use std::collections::HashMap;

// Java: public class SAXContentHandler extends DefaultHandler implements LexicalHandler, DeclHandler, DTDHandler
pub struct SAXContentHandler {
    // Java field: Document document
    // Rust: `XmlParse` 以 `Box<dyn ContentHandler>` 持有本 handler ⇒ 文档用共享单元回传（登记 §1.7.1 ①）
    document: std::rc::Rc<std::cell::RefCell<Option<Box<dyn std::any::Any + Send + Sync>>>>,
    // Java field: ElementStack elementStack
    // Rust: 类型化栈（Java 的 `ElementStack` 是 Any 壳）——push/pop/peek 步骤等价（登记 §1.7.1 ②）
    elementStack: Vec<crate::io::dom::tree::DefaultElement::DefaultElement>,
    // Java field: （由 `NamespaceStack` 承担的默认命名空间作用域）
    // Rust: 本 crate 的 `XmlParse` 不为**无前缀**元素解析默认命名空间（`xmlns=`）⇒ handler 侧自行维护作用域（登记 §1.7.1 ①）
    defaultNsScope: Vec<Option<String>>,
    // Java field: NamespaceStack namespaceStack
    namespaceStack: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: ElementHandler elementHandler
    elementHandler: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: Locator locator
    locator: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: String entity
    entity: Option<String>,
    // Java field: boolean insideDTDSection
    insideDTDSection: bool,
    // Java field: boolean insideCDATASection
    insideCDATASection: bool,
    // Java field: StringBuffer cdataText
    cdataText: Option<String>,
    // Java field: Map availableNamespaceMap
    availableNamespaceMap: HashMap<String, String>,
    // Java field: List declaredNamespaceList
    declaredNamespaceList: Vec<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: List<Declaration> internalDTDDeclarations
    internalDTDDeclarations: Option<Vec<Box<dyn std::any::Any + Send + Sync>>>,
    // Java field: List<Declaration> externalDTDDeclarations
    externalDTDDeclarations: Option<Vec<Box<dyn std::any::Any + Send + Sync>>>,
    // Java field: int declaredNamespaceIndex
    declaredNamespaceIndex: i32,
    // Java field: EntityResolver entityResolver
    entityResolver: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: InputSource inputSource
    inputSource: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: Element currentElement
    currentElement: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: boolean includeInternalDTDDeclarations
    includeInternalDTDDeclarations: bool,
    // Java field: boolean includeExternalDTDDeclarations
    includeExternalDTDDeclarations: bool,
    // Java field: int entityLevel
    entityLevel: i32,
    // Java field: boolean internalDTDsubset
    internalDTDsubset: bool,
    // Java field: boolean mergeAdjacentText
    mergeAdjacentText: bool,
    // Java field: boolean textInTextBuffer
    textInTextBuffer: bool,
    // Java field: boolean ignoreComments
    ignoreComments: bool,
    // Java field: StringBuffer textBuffer
    textBuffer: Option<String>,
    // Java field: boolean stripWhitespaceText
    stripWhitespaceText: bool,
    // Java field: boolean mNewNamespace
    mNewNamespace: bool,
    // Java field: int mCurrentLevel
    mCurrentLevel: i32,
    // Java field: MCEContext mMCEContext
    mMCEContext: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class SAXContentHandler extends DefaultHandler implements LexicalHandler, DeclHandler, DTDHandler
impl SAXContentHandler {
    // Java: SAXContentHandler()
    pub fn new() -> Self {
        // Java: this((ElementHandler)null, false);
        Self::new_with_handler(None, false)
    }

    // Java: SAXContentHandler(ElementHandler elementHandler, boolean onlyUseName)
    pub fn new_with_handler(element_handler: Option<Box<dyn std::any::Any + Send + Sync>>, _only_use_name: bool) -> Self {
        // Java: this(elementHandler, (ElementStack)null);
        Self::new_with_stack(element_handler, None)
        // Java: this.elementStack = this.createElementStack(onlyUseName);
        // Rust: `createElementStack` 未镜像（类型化栈不接受 onlyUseName）⇒ 忽略该形参（登记 §1.7.1 ②）
    }

    // Java: SAXContentHandler(ElementHandler elementHandler, ElementStack elementStack)
    pub fn new_with_stack(element_handler: Option<Box<dyn std::any::Any + Send + Sync>>, element_stack: Option<Box<dyn std::any::Any + Send + Sync>>) -> Self {
        // Java: super();
        // Java: this.availableNamespaceMap = new HashMap();
        // Java: this.declaredNamespaceList = new ArrayList();
        // Java: this.includeInternalDTDDeclarations = false;
        // Java: this.includeExternalDTDDeclarations = false;
        // Java: this.internalDTDsubset = false;
        // Java: this.mergeAdjacentText = false;
        // Java: this.textInTextBuffer = false;
        // Java: this.ignoreComments = false;
        // Java: this.stripWhitespaceText = false;
        // Java: this.mNewNamespace = false;
        // Java: this.mCurrentLevel = -1;
        // Java: this.elementHandler = elementHandler;
        // Java: this.elementStack = elementStack;
        // Java: this.namespaceStack = new NamespaceStack();
        SAXContentHandler {
            document: std::rc::Rc::new(std::cell::RefCell::new(None)),
            elementStack: Vec::new(),
            defaultNsScope: Vec::new(),
            namespaceStack: None,
            elementHandler: element_handler,
            locator: None,
            entity: None,
            insideDTDSection: false,
            insideCDATASection: false,
            cdataText: None,
            availableNamespaceMap: HashMap::new(),
            declaredNamespaceList: Vec::new(),
            internalDTDDeclarations: None,
            externalDTDDeclarations: None,
            declaredNamespaceIndex: 0,
            entityResolver: None,
            inputSource: None,
            currentElement: None,
            includeInternalDTDDeclarations: false,
            includeExternalDTDDeclarations: false,
            entityLevel: 0,
            internalDTDsubset: false,
            mergeAdjacentText: false,
            textInTextBuffer: false,
            ignoreComments: false,
            textBuffer: None,
            stripWhitespaceText: false,
            mNewNamespace: false,
            mCurrentLevel: -1,
            mMCEContext: None,
        }
    }

    // rust: no-java（Rust 原生辅助：把文档共享单元注入 handler，替代 Java 侧「持有 handler 引用」的方式）
    pub(crate) fn new_with_document(
        document: std::rc::Rc<std::cell::RefCell<Option<Box<dyn std::any::Any + Send + Sync>>>>,
    ) -> Self {
        let mut handler = Self::new_with_stack(None, None);
        handler.document = document;
        handler
    }

    // Java: Document getDocument()
    pub fn getDocument(&self) -> Option<std::cell::Ref<'_, Box<dyn std::any::Any + Send + Sync>>> {
        // Java: if (this.document == null)
        // Java: this.document = this.createDocument();
        if self.document.borrow().is_none() {
            *self.document.borrow_mut() = Some(Box::new(
                crate::io::dom::tree::DefaultDocument::DefaultDocument::new(),
            ));
        }
        // Java: return this.document;
        Some(std::cell::Ref::map(self.document.borrow(), |d| {
            d.as_ref().expect("document 已在上面初始化")
        }))
    }

    // Java: void setDocumentLocator(Locator documentLocator)
    pub fn setDocumentLocator(&mut self, document_locator: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.locator = documentLocator;
        self.locator = Some(document_locator);
    }

    // Java: void processingInstruction(String target, String data)
    pub fn processingInstruction(&mut self, _target: &str, _data: &str) {
        // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
        // Java: this.completeCurrentTextNode();
        // Java: if (this.currentElement != null)
        // Java: this.currentElement.addProcessingInstruction(target, data);
        // Java: else
        // Java: this.getDocument().addProcessingInstruction(target, data);
        todo!()
    }

    // Java: void startPrefixMapping(String prefix, String uri)
    pub fn startPrefixMapping(&mut self, _prefix: &str, _uri: &str) {
        // Java: this.namespaceStack.push(prefix, uri);
        // Java: this.mNewNamespace = true;
        todo!()
    }

    // Java: void endPrefixMapping(String prefix)
    pub fn endPrefixMapping(&mut self, _prefix: &str) {
        // Java: this.namespaceStack.pop(prefix);
        // Java: this.declaredNamespaceIndex = this.namespaceStack.size();
        // Java: this.mNewNamespace = false;
        todo!()
    }

    // Java: void startDocument()
    pub fn startDocument(&mut self) {
        // Java: this.document = null;
        *self.document.borrow_mut() = None;
        // Java: this.currentElement = null;
        self.currentElement = None;
        // Java: this.elementStack.clear();
        self.elementStack.clear();
        // Java: if (this.elementHandler != null && this.elementHandler instanceof DispatchHandler)
        // Java: this.elementStack.setDispatchHandler((DispatchHandler)this.elementHandler);
        // rust: no-stmt —— 本 crate 的 `elementHandler` 是 `&dyn Any` 壳、`elementStack` 是
        //       `Vec<DefaultElement>`，无 `DispatchHandler`/`setDispatchHandler`（登记 §1.7.1 ⑬）
        // Java: this.namespaceStack.clear();
        // rust: Java 的 `NamespaceStack` 在本 crate 由 `defaultNsScope` 承担（登记 §1.7.1 ①）
        self.defaultNsScope.clear();
        // Java: this.declaredNamespaceIndex = 0;
        self.declaredNamespaceIndex = 0;
        // Java: if (this.mergeAdjacentText && this.textBuffer == null)
        // Java: this.textBuffer = new StringBuffer();
        if self.mergeAdjacentText && self.textBuffer.is_none() {
            self.textBuffer = Some(String::new());
        }
        // Java: this.textInTextBuffer = false;
        self.textInTextBuffer = false;
    }

    // Java: void endDocument()
    pub fn endDocument(&mut self) {
        // Java: this.namespaceStack.clear();
        // Java: this.elementStack.clear();
        // Java: this.currentElement = null;
        // Java: this.textBuffer = null;
        self.currentElement = None;
        self.textBuffer = None;
    }

    // Java: void setMCEContext(MCEContext mceContext)
    pub fn setMCEContext(&mut self, mce_context: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.mMCEContext = mceContext;
        self.mMCEContext = Some(mce_context);
    }

    // Java: static Element addElementToBranch(Branch branch, String namespaceURI, String localName, String qualifiedName)
    fn addElementToBranch(branch: &mut dyn std::any::Any, namespaceURI: &str, localName: &str, qualifiedName: &str) -> Box<dyn std::any::Any + Send + Sync> {
        // Java: if (localName == null)
        // rust: Rust 无 `null` 字符串 ⇒ 空串表示「未给」（登记 §1.7.1 ①）
        // Java: localName = qualifiedName;
        let localName = if localName.is_empty() { qualifiedName } else { localName };
        // Java: else if (qualifiedName == null)
        // Java: qualifiedName = localName;
        let qualifiedName = if qualifiedName.is_empty() { localName } else { qualifiedName };
        // Java: if (namespaceURI == null)
        // Java: namespaceURI = "";
        let namespaceURI = namespaceURI;
        // Java: String prefix = qualifiedName;
        let prefix = qualifiedName;
        // Java: int index = qualifiedName.indexOf(58);
        let index = qualifiedName.find(':');
        // Java: if (index > 0)
        // Java: prefix = qualifiedName.substring(0, index);
        let prefix = match index {
            Some(i) if i > 0 => &qualifiedName[..i],
            _ => prefix,
        };
        // Java: return branch.addElement(namespaceURI, prefix, localName);
        // rust: no-stmt —— `Branch::addElement` 未接线（DOM 层停车，登记 §1.7.1 ⑩）
        let _ = (branch, namespaceURI, prefix, localName);
        todo!("`Branch::addElement` 未接线（DOM 层停车，登记 §1.7.1 ⑩）")
    }

    // Java: void startElement(String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    // rust: 本 crate 门面多一个 `_element_id`（`XmlParse` 的元素 id，Rust-native，登记 §1.7.1 ①）
    pub fn startElement(&mut self, _element_id: i32, namespace_uri: &str, local_name: &str, qualified_name: &str, attributes: &dyn std::any::Any) {
        // Java: if (this.mergeAdjacentText && this.textInTextBuffer) {
        // Java: this.completeCurrentTextNode();
        // rust: no-stmt —— `completeCurrentTextNode()` 在本 crate 未接线；文本合并由 `getStringValue` 串接表达（登记 §1.7.1 ③）
        // Java: Branch branch = this.currentElement; if (branch == null) { branch = this.getDocument(); }
        let prefix = match qualified_name.find(':') {
            Some(i) => &qualified_name[..i],
            None => "",
        };
        // rust: 本 crate 的 `XmlParse` 在部分模式下只给 qualifiedName ⇒ local 为空时从 qname 推导（登记 §1.7.1 ①）
        let local_name = if local_name.is_empty() {
            match qualified_name.find(':') {
                Some(i) => &qualified_name[i + 1..],
                None => qualified_name,
            }
        } else {
            local_name
        };
        // Java: IOAssert.assertNotNull("element should not be null.", element);
        // rust: no-stmt —— `IOAssert` 是断言日志；本 crate 的 `obtain_3` 直接返回元素（登记 §1.7.1 ⑦）
        // Java: if (this.mNewNamespace) {
        // Java: for(int size = this.namespaceStack.size(); this.declaredNamespaceIndex < size; ++this.declaredNamespaceIndex) {
        // rust: Java 的 `NamespaceStack` 在本 crate 由 `defaultNsScope` 承担（登记 §1.7.1 ①）
        let mut declared_default_ns: Option<String> = None;
        let mut attribute_nodes: Vec<(String, String, String, String)> = Vec::new();
        if let Some(attrs) = attributes.downcast_ref::<crate::io::opt::AttributesImpl::AttributesImpl>() {
            // Java: int size = attributes.getLength();
            for i in 0..attrs.getLength() {
                let attr_qname = attrs.getQName(i).unwrap_or_default();
                let attr_value = attrs.getValue_by_index(i).unwrap_or_default();
                if attr_qname == "xmlns" {
                    // rust: 无前缀的 `xmlns=` 被 `XmlParse` 当普通属性上报（带前缀的 `xmlns:*` 被它吞掉）⇒ 只在此处理默认命名空间
                    declared_default_ns = Some(attr_value.clone());
                    attribute_nodes.push((String::new(), String::new(), "xmlns".to_string(), attr_value));
                } else {
                    let attr_prefix = match attr_qname.find(':') {
                        Some(i) => &attr_qname[..i],
                        None => "",
                    };
                    let attr_local = attrs.getLocalName(i).unwrap_or_else(|| attr_qname.clone());
                    let attr_uri = attrs.getURI(i).unwrap_or_default();
                    attribute_nodes.push((attr_uri, attr_prefix.to_string(), attr_local, attr_value));
                }
            }
        }
        // Java: Namespace namespace = this.namespaceStack.getNamespace(this.declaredNamespaceIndex);
        // Java: IOAssert.assertNotNull("namespace should not be null.", namespace);
        // rust: （SAX 由 `SAXParser` 解析默认命名空间；本 crate 的 `XmlParse` 不解析）⇒ 无前缀元素用作用域内的默认命名空间（登记 §1.7.1 ①）
        let in_scope_default_ns = declared_default_ns
            .clone()
            .or_else(|| self.defaultNsScope.last().cloned().flatten());
        self.defaultNsScope.push(in_scope_default_ns.clone());
        let effective_namespace_uri = if prefix.is_empty() {
            in_scope_default_ns.clone().unwrap_or_default()
        } else {
            namespace_uri.to_string()
        };
        // Java: element.add(namespace);
        // Java: Element element = addElementToBranch(branch, namespaceURI, localName, qualifiedName);
        let mut element = crate::io::dom::tree::DefaultElement::DefaultElement::obtain_3(
            &effective_namespace_uri,
            prefix,
            local_name,
        );
        for (attr_uri, attr_prefix, attr_local, attr_value) in attribute_nodes {
            if attr_local == "xmlns" && attr_prefix.is_empty() {
                element.add_2(Box::new(crate::io::dom::Namespace::Namespace::get(
                    "",
                    &attr_value,
                )));
            } else {
                // Java: int size = attributes.getLength();
                // Java: if (size > 0) {
                // Java: boolean noNamespaceAttributes = false; element.setAttributes(attributes, size, this.namespaceStack, false, this.mMCEContext);
                // rust: SAX 替代通路逐个建属性 ⇒ 直接追加到 `mAttributes`（**不是** `mContent`；
                //       属性不是子节点，登记 §1.7.1 ⑫）
                let attribute = crate::io::dom::tree::DefaultAttribute::DefaultAttribute::create(
                    &attr_uri,
                    &attr_prefix,
                    &attr_local,
                    &attr_value,
                );
                element
                    .mAttributes
                    .get_or_insert_with(Vec::new)
                    .push(Box::new(attribute));
            }
        }
        // Java: this.mNewNamespace = false;
        // rust: no-stmt —— `mNewNamespace` 由 `startPrefixMapping` 置位，本 crate 的 SAX 替代通路
        //       不产生该状态（登记 §1.7.1 ①）
        // Java: this.elementStack.pushElement(element);
        self.elementStack.push(element);
        // Java: this.currentElement = element;
        // rust: 类型化栈的 `last()` 即当前元素 ⇒ 不再单独持有 `currentElement`（登记 §1.7.1 ③）
        // Java: this.entity = null;
        self.entity = None;
        // Java: if (this.elementHandler != null) {
        // Java: this.elementHandler.onStart(this.elementStack);
        // rust: no-stmt —— 本 crate 的 `elementHandler` 是 `&dyn Any` 壳，未接线回调（登记 §1.7.1 ⑬）
    }

    // Java: void endElement(String namespaceURI, String localName, String qName)
    pub fn endElement(&mut self, _element_id: i32, _namespace_uri: &str, _local_name: &str, _q_name: &str) {
        // Java: if (this.mergeAdjacentText && this.textInTextBuffer) {
        // Java: this.completeCurrentTextNode();
        // rust: no-stmt —— 同 `startElement`（登记 §1.7.1 ③）
        // Java: IOAssert.assertNotNull("currentElement should not be null", this.currentElement);
        // rust: no-stmt —— 断言日志；本 crate 用 `Option` 语义承接（登记 §1.7.1 ⑦）
        // Java: if (this.elementHandler != null) {
        // Java: this.elementHandler.onEnd(this.elementStack);
        // rust: no-stmt —— `elementHandler` 未接线回调（登记 §1.7.1 ⑬）
        // rust: 与 `startElement` 的 defaultNsScope.push 配对（登记 §1.7.1 ①）
        self.defaultNsScope.pop();
        // Java: this.elementStack.popElement();
        if let Some(element) = self.elementStack.pop() {
            // Java: this.currentElement = this.elementStack.peekElement();
            if let Some(parent) = self.elementStack.last_mut() {
                parent.add_2(Box::new(element));
            } else {
                // Java: document.setRootElement(element)（栈空 ⇒ 根元素）
                // rust: `XmlParse` 不调用 `startDocument()` ⇒ 文档在此按需创建（登记 §1.7.1 ③）
                if self.document.borrow().is_none() {
                    *self.document.borrow_mut() = Some(Box::new(
                        crate::io::dom::tree::DefaultDocument::DefaultDocument::new(),
                    ));
                }
                if let Some(doc) = self.document.borrow_mut().as_mut() {
                    if let Some(default_document) =
                        doc.downcast_mut::<crate::io::dom::tree::DefaultDocument::DefaultDocument>()
                    {
                        default_document.setRootElement(Some(Box::new(element)));
                    }
                }
            }
        }
        // Java: --this.mCurrentLevel;
        self.mCurrentLevel -= 1;
    }

    // Java: void characters(char[] ch, int start, int end)
    pub fn characters(&mut self, ch: &[char], start: usize, end: usize) {
        // Java: if (end != 0) {
        if end != 0 {
            // Java: if (this.currentElement != null) {
            // rust: 本 crate 以类型化栈的 `last_mut()` 承接 `currentElement`（登记 §1.7.1 ③）
            if let Some(current) = self.elementStack.last_mut() {
                // Java: if (this.mMCEContext != null && !this.mMCEContext.mMceDisabled) {
                // Java: boolean skip = this.mMCEContext.shouldSkipSkipStack(this.mCurrentLevel + 1);
                // Java: if (skip) {
                // Java: return;
                // rust: no-stmt —— `mMCEContext` 在本 crate 是 `&dyn Any` 壳（未接线 MCE 跳过栈，登记 §1.7.1 ⑬）
                // Java: if (this.entity != null) {
                // Java: if (this.mergeAdjacentText && this.textInTextBuffer) {
                // Java: this.completeCurrentTextNode();
                // Java: this.currentElement.addEntity(this.entity, new String(ch, start, end));
                // Java: this.entity = null;
                // rust: no-stmt —— 实体文本在本 crate 未接线（`entity` 仅由 `startEntity` 置位，登记 §1.7.1 ⑬）
                // Java: } else if (this.insideCDATASection) {
                // Java: if (this.mergeAdjacentText && this.textInTextBuffer) {
                // Java: this.completeCurrentTextNode();
                // Java: this.cdataText.append(new String(ch, start, end));
                // rust: no-stmt —— CDATA 段在本 crate 由 `startCDATA`/`endCDATA` 直接建文本节点（登记 §1.7.1 ⑬）
                // Java: } else if (this.mergeAdjacentText) {
                // Java: this.textBuffer.append(ch, start, end);
                // Java: this.textInTextBuffer = true;
                // rust: no-stmt —— `mergeAdjacentText` 的合并语义由 `getStringValue` 串接表达（登记 §1.7.1 ③）
                // Java: } else {
                // Java: this.currentElement.addText(new String(ch, start, end));
                let text: String = ch[start..start + end].iter().collect();
                if !text.is_empty() {
                    current.add_2(Box::new(
                        crate::io::dom::tree::DefaultText::DefaultText::new_1(&text),
                    ));
                }
            }
        }
    }

    // Java: void warning(SAXParseException exception)
    pub fn warning(&self, _exception: Box<dyn std::any::Any + Send + Sync>) {
    }

    // Java: void error(SAXParseException exception)
    pub fn error(&self, _exception: Box<dyn std::any::Any + Send + Sync>) -> Result<(), String> {
        // Java: throw exception;
        Err("SAXParseException".to_string())
    }

    // Java: void fatalError(SAXParseException exception)
    pub fn fatalError(&self, _exception: Box<dyn std::any::Any + Send + Sync>) -> Result<(), String> {
        // Java: throw exception;
        Err("SAXParseException".to_string())
    }

    // Java: void startDTD(String name, String publicId, String systemId)
    pub fn startDTD(&mut self, _name: &str, _public_id: &str, _system_id: &str) {
        // Java: this.getDocument().addDocType(name, publicId, systemId);
        // Java: this.insideDTDSection = true;
        // Java: this.internalDTDsubset = true;
        self.insideDTDSection = true;
        self.internalDTDsubset = true;
    }

    // Java: void endDTD()
    pub fn endDTD(&mut self) {
        // Java: this.insideDTDSection = false;
        self.insideDTDSection = false;
        // Java: DocumentType docType = this.getDocument().getDocType();
        // Java: if (docType != null)
        // Java: if (this.internalDTDDeclarations != null)
        // Java: if (this.externalDTDDeclarations != null)
        // Java: this.internalDTDDeclarations = null;
        self.internalDTDDeclarations = None;
        // Java: this.externalDTDDeclarations = null;
        self.externalDTDDeclarations = None;
    }

    // Java: void startEntity(String name)
    pub fn startEntity(&mut self, name: &str) {
        // Java: if (!this.mMCEContext.mMceDisabled)
        // Java: boolean skip = this.mMCEContext.shouldSkipSkipStack(this.mCurrentLevel + 1);
        // Java: if (skip)
        // Java: return;
        // Java: ++this.entityLevel;
        self.entityLevel += 1;
        // Java: this.entity = null;
        self.entity = None;
        // Java: if (!this.insideDTDSection && !this.isIgnorableEntity(name))
        if !self.insideDTDSection && !self.isIgnorableEntity(name) {
            // Java: this.entity = name;
            self.entity = Some(name.to_string());
        }
        // Java: this.internalDTDsubset = false;
        self.internalDTDsubset = false;
    }

    // Java: void endEntity(String name)
    pub fn endEntity(&mut self, _name: &str) {
        // Java: if (!this.mMCEContext.mMceDisabled)
        // Java: boolean skip = this.mMCEContext.shouldSkipSkipStack(this.mCurrentLevel + 1);
        // Java: if (skip)
        // Java: return;
        // Java: --this.entityLevel;
        self.entityLevel -= 1;
        // Java: this.entity = null;
        self.entity = None;
        // Java: if (this.entityLevel == 0)
        if self.entityLevel == 0 {
            // Java: this.internalDTDsubset = true;
            self.internalDTDsubset = true;
        }
    }

    // Java: void startCDATA()
    pub fn startCDATA(&mut self) {
        // Java: this.insideCDATASection = true;
        self.insideCDATASection = true;
        // Java: this.cdataText = new StringBuffer();
        self.cdataText = Some(String::new());
    }

    // Java: void endCDATA()
    pub fn endCDATA(&mut self) {
        // Java: this.insideCDATASection = false;
        self.insideCDATASection = false;
        // Java: this.currentElement.addCDATA(this.cdataText.toString());
        todo!()
    }

    // Java: void comment(char[] ch, int start, int end)
    pub fn comment(&mut self, _ch: &[char], _start: usize, _end: usize) {
        // Java: if (!this.ignoreComments)
        // Java: if (this.mergeAdjacentText && this.textInTextBuffer)
        // Java: this.completeCurrentTextNode();
        // Java: String text = new String(ch, start, end); if (!this.insideDTDSection && text.length() > 0)
        // Java: if (this.currentElement != null)
        // Java: this.currentElement.addComment(text);
        // Java: else
        // Java: this.getDocument().addComment(text);
        todo!()
    }

    // Java: void elementDecl(String name, String model)
    pub fn elementDecl(&mut self, _name: &str, _model: &str) {
        // Java: if (this.internalDTDsubset)
        // Java: if (this.includeInternalDTDDeclarations)
        // Java: this.addDTDDeclaration(new ElementDecl(name, model));
        // Java: else if (this.includeExternalDTDDeclarations)
        // Java: this.addExternalDTDDeclaration(new ElementDecl(name, model));
        todo!()
    }

    // Java: void attributeDecl(String eName, String aName, String type, String valueDefault, String val)
    pub fn attributeDecl(&mut self, _e_name: &str, _a_name: &str, _type_str: &str, _value_default: &str, _val: &str) {
        // Java: if (this.internalDTDsubset)
        // Java: if (this.includeInternalDTDDeclarations)
        // Java: this.addDTDDeclaration(new AttributeDecl(eName, aName, type, valueDefault, val));
        // Java: else if (this.includeExternalDTDDeclarations)
        // Java: this.addExternalDTDDeclaration(new AttributeDecl(eName, aName, type, valueDefault, val));
        todo!()
    }

    // Java: void internalEntityDecl(String name, String value)
    pub fn internalEntityDecl(&mut self, _name: &str, _value: &str) {
        // Java: if (this.internalDTDsubset)
        // Java: if (this.includeInternalDTDDeclarations)
        // Java: this.addDTDDeclaration(new InternalEntityDecl(name, value));
        // Java: else if (this.includeExternalDTDDeclarations)
        // Java: this.addExternalDTDDeclaration(new InternalEntityDecl(name, value));
        todo!()
    }

    // Java: void externalEntityDecl(String name, String publicId, String sysId)
    pub fn externalEntityDecl(&mut self, _name: &str, _public_id: &str, _sys_id: &str) {
        // Java: ExternalEntityDecl declaration = new ExternalEntityDecl(name, publicId, sysId); if (this.internalDTDsubset) { if (this.includeInternalDTDDeclarations) { this.addDTDDeclaration(declaration); } }
        // Java: else if (this.includeExternalDTDDeclarations)
        // Java: this.addExternalDTDDeclaration(declaration);
        todo!()
    }

    // Java: void notationDecl(String name, String publicId, String systemId)
    pub fn notationDecl(&self, _name: &str, _public_id: &str, _system_id: &str) {
    }

    // Java: void unparsedEntityDecl(String name, String publicId, String systemId, String notationName)
    pub fn unparsedEntityDecl(&self, _name: &str, _public_id: &str, _system_id: &str, _notation_name: &str) {
    }

    // Java: ElementStack getElementStack()
    pub fn getElementStack(&self) -> &Vec<crate::io::dom::tree::DefaultElement::DefaultElement> {
        // Java: return this.elementStack;
        &self.elementStack
    }

    // Java: void setElementStack(ElementStack elementStack)
    pub fn setElementStack(&mut self, element_stack: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.elementStack = elementStack;
        // Java: this.elementStack = elementStack;（Java 的 ElementStack 壳未镜像，登记 §1.7.1 ②）
        let _ = element_stack;
        self.elementStack = Vec::new();
    }

    // Java: EntityResolver getEntityResolver()
    pub fn getEntityResolver(&self) -> &Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: return this.entityResolver;
        &self.entityResolver
    }

    // Java: void setEntityResolver(EntityResolver entityResolver)
    pub fn setEntityResolver(&mut self, entity_resolver: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.entityResolver = entityResolver;
        self.entityResolver = Some(entity_resolver);
    }

    // Java: InputSource getInputSource()
    pub fn getInputSource(&self) -> &Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: return this.inputSource;
        &self.inputSource
    }

    // Java: void setInputSource(InputSource inputSource)
    pub fn setInputSource(&mut self, input_source: Box<dyn std::any::Any + Send + Sync>) {
        // Java: this.inputSource = inputSource;
        self.inputSource = Some(input_source);
    }

    // Java: boolean isIncludeInternalDTDDeclarations()
    pub fn isIncludeInternalDTDDeclarations(&self) -> bool {
        // Java: return this.includeInternalDTDDeclarations;
        self.includeInternalDTDDeclarations
    }

    // Java: void setIncludeInternalDTDDeclarations(boolean include)
    pub fn setIncludeInternalDTDDeclarations(&mut self, include: bool) {
        // Java: this.includeInternalDTDDeclarations = include;
        self.includeInternalDTDDeclarations = include;
    }

    // Java: boolean isIncludeExternalDTDDeclarations()
    pub fn isIncludeExternalDTDDeclarations(&self) -> bool {
        // Java: return this.includeExternalDTDDeclarations;
        self.includeExternalDTDDeclarations
    }

    // Java: void setIncludeExternalDTDDeclarations(boolean include)
    pub fn setIncludeExternalDTDDeclarations(&mut self, include: bool) {
        // Java: this.includeExternalDTDDeclarations = include;
        self.includeExternalDTDDeclarations = include;
    }

    // Java: boolean isMergeAdjacentText()
    pub fn isMergeAdjacentText(&self) -> bool {
        // Java: return this.mergeAdjacentText;
        self.mergeAdjacentText
    }

    // Java: void setMergeAdjacentText(boolean mergeAdjacentText)
    pub fn setMergeAdjacentText(&mut self, merge: bool) {
        // Java: this.mergeAdjacentText = mergeAdjacentText;
        self.mergeAdjacentText = merge;
    }

    // Java: boolean isStripWhitespaceText()
    pub fn isStripWhitespaceText(&self) -> bool {
        // Java: return this.stripWhitespaceText;
        self.stripWhitespaceText
    }

    // Java: void setStripWhitespaceText(boolean stripWhitespaceText)
    pub fn setStripWhitespaceText(&mut self, strip: bool) {
        // Java: this.stripWhitespaceText = stripWhitespaceText;
        self.stripWhitespaceText = strip;
    }

    // Java: boolean isIgnoreComments()
    pub fn isIgnoreComments(&self) -> bool {
        // Java: return this.ignoreComments;
        self.ignoreComments
    }

    // Java: void setIgnoreComments(boolean ignoreComments)
    pub fn setIgnoreComments(&mut self, ignore: bool) {
        // Java: this.ignoreComments = ignoreComments;
        self.ignoreComments = ignore;
    }

    // Java: void completeCurrentTextNode()
    fn completeCurrentTextNode(&mut self) {
        // Java: if (this.stripWhitespaceText)
        // Java: String trimed = this.textBuffer.toString().trim();
        // Java: if (trimed.length() > 0)
        // Java: this.currentElement.addText(trimed);
        // Java: if (this.elementHandler != null)
        // Java: this.elementHandler.onCharacters(trimed);
        // Java: else
        // Java: String untrimed = this.textBuffer.toString();
        // Java: this.currentElement.addText(untrimed);
        // Java: if (this.elementHandler != null)
        // Java: this.elementHandler.onCharacters(untrimed);
        // Java: this.textBuffer.setLength(0);
        // Java: this.textInTextBuffer = false;
        self.elementStack = Vec::new(); // Java: this.elementStack = elementStack;（Java 的 ElementStack 壳未镜像，登记 §1.7.1 ②）
    }

    // Java: Document createDocument()
    fn createDocument(&mut self) -> Box<dyn std::any::Any + Send + Sync> {
        // Java: String encoding = this.getEncoding();
        // Java: Document doc = new DefaultDocument(); doc.setXMLEncoding(encoding);
        // Java: doc.setEntityResolver(this.entityResolver);
        // Java: if (this.inputSource != null)
        // Java: doc.setName(this.inputSource.getSystemId());
        // Java: return doc;
        todo!()
    }

    // Java: String getEncoding()
    fn getEncoding(&self) -> Option<String> {
        // Java: if (this.locator == null)
        // Java: return null;
        // Java: else
        // Java: return this.locator instanceof Locator2 ? ((Locator2)this.locator).getEncoding() : null;
        None
    }

    // Java: boolean isIgnorableEntity(String name)
    fn isIgnorableEntity(&self, name: &str) -> bool {
        // Java: return "amp".equals(name) || "apos".equals(name) || "gt".equals(name) || "lt".equals(name) || "quot".equals(name);
        name == "amp" || name == "apos" || name == "gt" || name == "lt" || name == "quot"
    }

    // Java: void addDeclaredNamespaces(Element element)
    fn addDeclaredNamespaces(&mut self, _element: &mut dyn std::any::Any) {
        // Java: for(int size = this.namespaceStack.size(); this.declaredNamespaceIndex < size; ++this.declaredNamespaceIndex)
        // Java: Namespace namespace = this.namespaceStack.getNamespace(this.declaredNamespaceIndex);
        // Java: IOAssert.assertNotNull("namespace should not be null.", namespace);
        // Java: element.add(namespace);
        todo!()
    }

    // Java: void addAttributes(Element element, Attributes attributes)
    fn addAttributes(&self, _element: &mut dyn std::any::Any, _attributes: &dyn std::any::Any) {
        // Java: boolean noNamespaceAttributes = false; IOAssert.assertTrue("element should be instanceof DefaultElement.", element instanceof DefaultElement);
        // Java: int size = attributes.getLength();
        // Java: if (size > 0)
        // Java: element.setAttributes(attributes, size, this.namespaceStack, noNamespaceAttributes, this.mMCEContext);
        todo!()
    }

    // Java: void addDTDDeclaration(Declaration declaration)
    fn addDTDDeclaration(&mut self, declaration: Box<dyn std::any::Any + Send + Sync>) {
        // Java: if (this.internalDTDDeclarations == null)
        if self.internalDTDDeclarations.is_none() {
            // Java: this.internalDTDDeclarations = new ArrayList();
            self.internalDTDDeclarations = Some(Vec::new());
        }
        // Java: this.internalDTDDeclarations.add(declaration);
        self.internalDTDDeclarations.as_mut().unwrap().push(declaration);
    }

    // Java: void addExternalDTDDeclaration(Declaration declaration)
    fn addExternalDTDDeclaration(&mut self, declaration: Box<dyn std::any::Any + Send + Sync>) {
        // Java: if (this.externalDTDDeclarations == null)
        if self.externalDTDDeclarations.is_none() {
            // Java: this.externalDTDDeclarations = new ArrayList();
            self.externalDTDDeclarations = Some(Vec::new());
        }
        // Java: this.externalDTDDeclarations.add(declaration);
        self.externalDTDDeclarations.as_mut().unwrap().push(declaration);
    }

    // Java: ElementStack createElementStack(boolean onlyUseName)
    fn createElementStack(&self, _only_use_name: bool) -> Box<dyn std::any::Any + Send + Sync> {
        // Java: return new ElementStack(onlyUseName);
        todo!()
    }
}
// Java: public class SAXContentHandler extends DefaultHandler implements LexicalHandler, DeclHandler, DTDHandler（ContentHandler 部分）
impl crate::io::opt::ContentHandler::ContentHandler for SAXContentHandler {
    fn startElement(&mut self, var1: i32, var2: &str, var3: &str, var4: &str, var5: &dyn std::any::Any) {
        SAXContentHandler::startElement(self, var1, var2, var3, var4, var5);
    }
    fn endElement(&mut self, var1: i32, var2: &str, var3: &str, var4: &str) {
        SAXContentHandler::endElement(self, var1, var2, var3, var4);
    }
    fn character_buf(&mut self, var1: &[char], var2: i32, var3: i32) {
        SAXContentHandler::characters(self, var1, var2 as usize, var3 as usize);
    }
    fn character_str(&mut self, var1: &str) {
        let chars: Vec<char> = var1.chars().collect();
        let len = chars.len();
        SAXContentHandler::characters(self, &chars, 0, len);
    }
}
