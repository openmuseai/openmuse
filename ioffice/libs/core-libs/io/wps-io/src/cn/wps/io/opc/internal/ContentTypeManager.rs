#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.dom.Attribute
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.DocumentException
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.io.SAXReader
// Java: import cn.wps.io.dom.tree.DefaultDocument
// Java: import cn.wps.io.opc.OPCPackage
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackagePartName
// Java: import cn.wps.io.opc.PackagingURIHelper
// Java: import cn.wps.io.util.URI
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidOperationException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JRuntimeException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.net.URISyntaxException
// Java: import java.util.HashMap
// Java: import java.util.Map

use std::collections::HashMap;
use std::io::Read;
use crate::io::opc::PackagePartName::PackagePartName;
use crate::io::opc::OPCPackage::OPCPackage;
use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackagingURIHelper::PackagingURIHelper;
use crate::io::util::URI::URI;

// Java field: String CONTENT_TYPES_PART_NAME
pub const CONTENT_TYPES_PART_NAME: &str = "[Content_Types].xml";
// Java field: String TYPES_NAMESPACE_URI
pub const TYPES_NAMESPACE_URI: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
// Java field: String TYPES_TAG_NAME
const TYPES_TAG_NAME: &str = "Types";
// Java field: String DEFAULT_TAG_NAME
const DEFAULT_TAG_NAME: &str = "Default";
// Java field: String EXTENSION_ATTRIBUTE_NAME
const EXTENSION_ATTRIBUTE_NAME: &str = "Extension";
// Java field: String CONTENT_TYPE_ATTRIBUTE_NAME
const CONTENT_TYPE_ATTRIBUTE_NAME: &str = "ContentType";
// Java field: String OVERRIDE_TAG_NAME
const OVERRIDE_TAG_NAME: &str = "Override";
// Java field: String PART_NAME_ATTRIBUTE_NAME
const PART_NAME_ATTRIBUTE_NAME: &str = "PartName";

// Java: public class ContentTypeManager
pub struct ContentTypeManager {
    // Java field: OPCPackage container
    // NOTE(unsafe-send): `container` 是指向**所属 `OPCPackage`** 的共享引用（Java 引用语义：包持有本对象、
    //                   本对象回指包 ⇒ 真环，Rust 用裸指针表达；生命周期由包持有者保证；文档模型单线程）。
    //                   与 `PackagePart._container` 同型同理由（`PackagePart.rs:31-50`）。
    //                   ⚠️ **不得**退回 `Box<dyn std::any::Any + Send + Sync>`：那会抹掉 Java 的静态类型
    //                   `OPCPackage`，使 `this.container.getParts()` / `getPart(partName)` 无法按 Java 调用
    //                   （曾因此把两处容器守卫改成 DEFERRED，见 §1.7.1 ②）。
    pub(crate) container: Option<*const OPCPackage>,
    // Java field: HashMap<String, String> defaultContentType
    pub(crate) defaultContentType: HashMap<String, String>,
    // Java field: HashMap<PackagePartName, String> overrideContentType
    pub(crate) overrideContentType: Option<HashMap<String, String>>,
}

// NOTE(unsafe-send): 字段里的 `*const OPCPackage` 是 Java 引用语义的**共享指针**（生命周期由包持有者保证，
// 文档模型单线程 parse）。★**未**加 `unsafe impl Send/Sync`（与 `PackagePart.rs:47-50` 不同）：
// 本类型当前没有跨线程通道要求 `Send`（`cargo check` 实测：不加也全绿）⇒ 保持 Rust 更严格的默认
// `!Send`，待真的需要跨线程时再按 `PackagePart` 的先例补 unsafe impl 并写明理由。
// Java: public class ContentTypeManager
impl ContentTypeManager {
    // Java: ContentTypeManager(InputStream in, OPCPackage pkg)
    pub fn new(input: Option<&mut dyn std::io::Read>, pkg: Option<*const OPCPackage>) -> Self {
        // Java: super();
        // Java: this.container = pkg;
        // Java: this.defaultContentType = new HashMap();
        let mut mgr = ContentTypeManager {
            container: pkg,
            defaultContentType: HashMap::new(),
            overrideContentType: None,
        };
        // Java: if (in != null)
        if input.is_some() {
            // Java: try
            // Java: this.parseContentTypesFile(in);
            let parsed = mgr.try_parseContentTypesFile(input.unwrap());
            // Java: catch (InvalidFormatException var4)
            // Java: throw new InvalidFormatException("Can't read content types part !");
            // Rust: Java 的 catch **丢弃**原消息、换成字面量 ⇒ 此处同样归一（否则 `PackagingURIHelper`
            //       的原始消息会漏到调用方，与 Java 的 `"Can't read content types part !"` 分叉）。
            //       ⚠️ Java 只捕获 `InvalidFormatException`；RuntimeException 家族（缺属性 ⇒ NPE）
            //       在 Java 侧**不被捕获** ⇒ 本归一**不**覆盖它（由 `try_parseContentTypesFile` 直接 panic 表达）。
            if let Err(_error) = parsed {
                // Java: throw new InvalidFormatException("Can't read content types part !");
                panic!("Can't read content types part !");
            }
        }
        mgr
    }

    // Rust-native: Java 构造器 `ContentTypeManager(InputStream, OPCPackage)` 的**同名入口**
    // （仓内约定：与 Java 同名的静态构造，如 `ZipFileZipEntrySource::ZipFileZipEntrySource`；
    //  `new` 与它同一实现，既有调用方继续用 `new`）。
    pub fn ContentTypeManager(input: Option<&mut dyn std::io::Read>, pkg: Option<*const OPCPackage>) -> Self {
        Self::new(input, pkg)
    }

    // Java: void addContentType(PackagePartName partName, String contentType)
    pub fn addContentType(&mut self, part_name: &PackagePartName, content_type: &str) {
        // Java: boolean defaultCTExists = false;
        let mut defaultCTExists = false;
        // Java: String extension = partName.getExtension().toLowerCase();
        let extension = part_name.getExtension().to_lowercase();
        // Java: if (extension.length() != 0 && (!this.defaultContentType.containsKey(extension) || (defaultCTExists = this.defaultContentType.containsValue(contentType))))
        if !extension.is_empty() && (!self.defaultContentType.contains_key(&extension) || {
            defaultCTExists = self.defaultContentType.values().any(|v| v == content_type);
            defaultCTExists
        }) {
            // Java: if (!defaultCTExists)
            if !defaultCTExists {
                // Java: this.addDefaultContentType(extension, contentType);
                self.addDefaultContentType(&extension, content_type);
            }
        // Java: else
        } else {
            // Java: this.addOverrideContentType(partName, contentType);
            self.addOverrideContentType(part_name, content_type);
        }
    }

    // Java: void addOverrideContentType(PackagePartName partName, String contentType)
    pub fn addOverrideContentType(&mut self, part_name: &PackagePartName, content_type: &str) {
        // Java: if (this.overrideContentType == null)
        if self.overrideContentType.is_none() {
            // Java: this.overrideContentType = new HashMap();
            self.overrideContentType = Some(HashMap::new());
        }
        // Java: this.overrideContentType.put(partName, contentType);
        if let Some(ref mut map) = self.overrideContentType {
            map.insert(part_name.getName(), content_type.to_string());
        }
    }

    // Java: void addDefaultContentType(String extension, String contentType)
    pub fn addDefaultContentType(&mut self, extension: &str, content_type: &str) {
        // Java: this.defaultContentType.put(extension.toLowerCase(), contentType);
        self.defaultContentType.insert(extension.to_lowercase(), content_type.to_string());
    }

    // Java: void removeContentType(PackagePartName partName)
    pub fn removeContentType(&mut self, part_name: &PackagePartName) {
        // Java: if (partName == null)
        // Java: throw new IllegalArgumentException("partName");
        // Java: else if (this.overrideContentType != null && this.overrideContentType.get(partName) != null)
        if let Some(ref mut map) = self.overrideContentType {
            if map.get(&part_name.getName()).is_some() {
                // Java: this.overrideContentType.remove(partName);
                map.remove(&part_name.getName());
                return;
            }
        }
        // Java: else
        // Java: String extensionToDelete = partName.getExtension();
        let extension_to_delete = part_name.getExtension();
        // Java: boolean deleteDefaultContentTypeFlag = true;
        let mut deleteDefaultContentTypeFlag = true;
        // Java: if (this.container != null)
        if let Some(container) = self.container {
            // Java: try
            // Java: for(PackagePart part : this.container.getParts())
            // NOTE: Java 源为 this.container.getParts()（container 为共享引用 ⇒ unsafe 解引用）；
            //       `getParts()` 与 Java 同样会**惰性建表**（`partList == null` 时补建），故按 &mut 访问。
            unsafe { (*(container as *mut OPCPackage)).getParts() };
            // Java: catch (InvalidFormatException e)
            // Java: throw new InvalidOperationException(e.getMessage());
            // Rust: 两侧都是 panic（`getPartsImpl` 内部按 Java `throw` 处 panic），转换后消息不变 ⇒ 仅留注释。
            if let Some(parts) = unsafe { (*container).getPartList() } {
                for part in parts.values() {
                    // Java: if (!part.getPartName().equals(partName) && part.getPartName().getExtension().equalsIgnoreCase(extensionToDelete))
                    // Rust: 相等判断走 `PartialEq`（`getName()` 比较，`PackagePartName.rs:548`）。Java 逐语句版
                    //       `equals()`（`PackagePartName.rs:472`）在 s01-03 尚未填 `partName` 字段时会 `unwrap()`
                    //       panic ⇒ 见 §1.7.1 ⑧；`PartialEq` 对同一 part 名与 Java `equals` 同判据。
                    if part.getPartName() != part_name
                        && part
                            .getPartName()
                            .getExtension()
                            .eq_ignore_ascii_case(&extension_to_delete)
                    {
                        // Java: deleteDefaultContentTypeFlag = false;
                        deleteDefaultContentTypeFlag = false;
                        // Java: break;
                        break;
                    }
                }
            }
        }
        // Java: if (deleteDefaultContentTypeFlag)
        if deleteDefaultContentTypeFlag {
            // Java: this.defaultContentType.remove(extensionToDelete);
            self.defaultContentType.remove(&extension_to_delete);
        }
        // Java: if (this.container != null)
        if let Some(container) = self.container {
            // Java: try
            // Java: for(PackagePart part : this.container.getParts())
            // NOTE: 同上一处守卫（Java `this.container.getParts()`；共享引用 ⇒ unsafe 解引用）。
            unsafe { (*(container as *mut OPCPackage)).getParts() };
            // Java: catch (InvalidFormatException e)
            // Java: throw new InvalidOperationException(e.getMessage());
            // Rust: 两侧同为 panic 且消息不变（见上一处守卫的同名注释）。
            // Rust: 先快照「part 名」——遍历 `partList` 期间还要 `self.getContentType(..)`（&self），
            //       快照可避免同时持有两处借用，语义与 Java 的 `for(PackagePart part : ...)` 一致。
            let partNames: Vec<PackagePartName> = unsafe { (*container).getPartList() }
                .map(|parts| parts.values().map(|p| p.getPartName().clone()).collect())
                .unwrap_or_default();
            for partName in partNames {
                // Java: if (!part.getPartName().equals(partName) && this.getContentType(part.getPartName()) == null)
                // Rust: 同守卫 1 —— 用 `PartialEq`（`getName()` 比较）代替会 panic 的 `equals()`（§1.7.1 ⑧）。
                if partName != *part_name && self.getContentType(&partName).is_none() {
                    // Java: throw new InvalidOperationException("Rule M2.4 is not respected: Nor a default element or override element is associated with the part: " + part.getPartName().getName());
                    panic!(
                        "Rule M2.4 is not respected: Nor a default element or override element is associated with the part: {}",
                        partName.getName()
                    );
                }
            }
        }
    }

    // Java: boolean isContentTypeRegister(String contentType)
    pub fn isContentTypeRegister(&self, content_type: &str) -> bool {
        // Java: if (contentType == null)
        // Java: throw new IllegalArgumentException("contentType");
        // Java: else
        // Java: return this.defaultContentType.values().contains(contentType) || this.overrideContentType != null && this.overrideContentType.values().contains(contentType);
        self.defaultContentType.values().any(|v| v == content_type) ||
            self.overrideContentType.as_ref().map_or(false, |m| m.values().any(|v| v == content_type))
    }

    // Java: String toLowercase(String partName)
    pub fn toLowercase(&self, part_name: &str) -> String {
        // Java: int i = 0;
        let mut i = 0usize;
        // Java: for(int n = partName.length(); i < n; ++i)
        let n = part_name.len();
        while i < n {
            // Java: char c = partName.charAt(i);
            // Rust: `charAt(i)` 只用于与 'A'..='Z' 比较；Rust 按字节取同样只在 ASCII 区间为真
            //       （UTF-8 的非 ASCII 字节 ≥ 0x80）⇒ 判据与 Java 等价。
            let c = part_name.as_bytes()[i] as char;
            // Java: if ('A' <= c && c <= 'Z')
            if ('A'..='Z').contains(&c) {
                // Java: return partName.toLowerCase();
                return part_name.to_lowercase();
            }
            // Java: ++i （for 的增量）
            i += 1;
        }
        // Java: return partName;
        part_name.to_string()
    }

    // Java: String getContentType(PackagePartName partName)
    pub fn getContentType(&self, part_name: &PackagePartName) -> Option<String> {
        // Java: if (partName == null)
        // Java: throw new IllegalArgumentException("partName");
        // Java: else
        // Java: if (this.overrideContentType != null)
        if let Some(ref map) = self.overrideContentType {
            // Java: String contentType = (String)this.overrideContentType.get(partName);
            let content_type = map.get(&part_name.getName());
            // Java: if (contentType != null)
            if let Some(ct) = content_type {
                // Java: return contentType;
                return Some(ct.clone());
            }
        }
        // Java: String extension = this.toLowercase(partName.getExtension());
        let extension = self.toLowercase(&part_name.getExtension());
        // Java: String contentType = (String)this.defaultContentType.get(extension);
        let content_type = self.defaultContentType.get(&extension);
        // Java: if (contentType != null)
        if let Some(ct) = content_type {
            // Java: return contentType;
            return Some(ct.clone());
        }
        // Java: else if (this.container != null && this.container.getPart(partName) != null)
        if let Some(container) = self.container {
            // NOTE: Java 源为 this.container.getPart(partName)（共享引用 ⇒ unsafe 解引用；`getPart` 同样惰性建表）
            let part_exists = unsafe { (*(container as *mut OPCPackage)).getPart(part_name).is_some() };
            if part_exists {
                // Java: throw new OpenXML4JRuntimeException("Rule M2.4 exception : this error should NEVER happen, if so please send a mail to the developers team, thanks !");
                panic!("Rule M2.4 exception : this error should NEVER happen, if so please send a mail to the developers team, thanks !");
            }
        }
        // Java: else
        // Java: return null;
        None
    }

    // Rust-native: 两张表的只读视图（Java 侧 `save()` 在同类内直接迭代，用例与 `s01-10` 的 dump 需要读）。
    // 只读、不改语义；写入仍只能走 addContentType/addOverrideContentType/addDefaultContentType/removeContentType。
    pub fn getDefaultContentTypes(&self) -> &HashMap<String, String> {
        &self.defaultContentType
    }

    // Rust-native: 见 getDefaultContentTypes（Java field: overrideContentType，未写入时仍为 null）。
    pub fn getOverrideContentTypes(&self) -> Option<&HashMap<String, String>> {
        self.overrideContentType.as_ref()
    }

    // Java: void clearAll()
    pub fn clearAll(&mut self) {
        // Java: this.defaultContentType.clear();
        self.defaultContentType.clear();
        // Java: if (this.overrideContentType != null)
        if let Some(ref mut map) = self.overrideContentType {
            // Java: this.overrideContentType.clear();
            map.clear();
        }
    }

    // Java: void clearOverrideContentTypes()
    pub fn clearOverrideContentTypes(&mut self) {
        // Java: if (this.overrideContentType != null)
        if let Some(ref mut map) = self.overrideContentType {
            // Java: this.overrideContentType.clear();
            map.clear();
        }
    }

    // Java: void parseContentTypesFile(InputStream in)
    pub fn parseContentTypesFile(&mut self, input: &mut dyn std::io::Read) {
        // Java: throws InvalidFormatException
        if let Err(error) = self.try_parseContentTypesFile(input) {
            // Java: throw new InvalidFormatException(urie.getMessage());
            // Java: throw new InvalidFormatException(e.getMessage());
            panic!("{}", error);
        }
    }

    // Rust-native: parseContentTypesFile 的可恢复版本（两层约定：try_ 核心 + Java 签名外壳）。
    // 判据与 Java 逐语句一致；`Err` 只承载 Java 的 `InvalidFormatException` 家族
    // （URISyntaxException / DocumentException）；RuntimeException 家族（缺属性 ⇒ NPE）按仓内约定
    // 直接 panic!，**不经** Err 通道 —— 因为 Java 的 ctor catch 也捕获不到它。
    pub fn try_parseContentTypesFile(&mut self, input: &mut dyn std::io::Read) -> Result<(), String> {
        // Java: try
        let parsed = (|| -> Result<(), String> {
            let mut bytes = Vec::new();
            input.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
            let xml = String::from_utf8_lossy(&bytes);
            if !xml.contains('<') {
                // Java: SAXReader.read(in) ⇒ DocumentException（非 XML）⇒ ctor 归一为 "Can't read content types part !"
                return Err("Can't read content types part !".to_string());
            }
            // Java: SAXReader xmlReader = new SAXReader();
            // Rust: `SAXReader::read_stream` 仍是 todo!()（DOM/SAX 层未实现，见 §1.7 偏差 ①）⇒ 用本文件内的
            //       具名替身读同一份字节流；**绑定名沿用 Java 的 `xmlReader`**，以标示「此处本应是读 XML 的 reader」。
            let xmlReader = XmlTypesReader::new(&xml);
            // Java: Document xmlContentTypetDoc = xmlReader.read(in);
            // Java: Element root = xmlContentTypetDoc.getRootElement();
            // Java: for(Element element : root.elementsRO("Default"))
            for tag in xmlReader.elementsRO(DEFAULT_TAG_NAME) {
                // Java: Attribute attrExtension = element.attribute("Extension");
                // Java: String extension = attrExtension.getValue();
                let extension = xmlReader.attribute(&tag, EXTENSION_ATTRIBUTE_NAME).unwrap_or_else(|| {
                    // Java: Attribute attrExtension = element.attribute("Extension");
                    // Java: String extension = attrExtension.getValue();   ⇒ 属性缺失时 attrExtension == null
                    // Java: ⇒ NullPointerException（RuntimeException），**不被 ctor 的 catch(InvalidFormatException) 捕获**
                    panic!("NullPointerException");
                });
                // Java: Attribute attrContentType = element.attribute("ContentType");
                // Java: String contentType = attrContentType.getValue();
                let content_type = xmlReader.attribute(&tag, CONTENT_TYPE_ATTRIBUTE_NAME).unwrap_or_else(|| {
                    // Java: Attribute attrContentType = element.attribute("ContentType");
                    // Java: String contentType = attrContentType.getValue();  ⇒ 同上 NPE
                    panic!("NullPointerException");
                });
                // Java: this.addDefaultContentType(extension, contentType);
                self.addDefaultContentType(&extension, &content_type);
            }
            // Java: for(Element element : root.elementsRO("Override"))
            for tag in xmlReader.elementsRO(OVERRIDE_TAG_NAME) {
                // Java: Attribute attrPartName = element.attribute("PartName");
                let part_name_str = xmlReader.attribute(&tag, PART_NAME_ATTRIBUTE_NAME).unwrap_or_else(|| {
                    // Java: Attribute attrPartName = element.attribute("PartName");
                    // Java: PackagePartName partName = ...(new URI(attrPartName.getValue()));  ⇒ 属性缺失 ⇒ NPE
                    panic!("NullPointerException");
                });
                // Java: PackagePartName partName = PackagingURIHelper.createPartName(new URI(attrPartName.getValue()));
                let part_name = PackagingURIHelper::createPartName_str(&part_name_str)?;
                // Java: Attribute attrContentType = element.attribute("ContentType");
                // Java: String contentType = attrContentType.getValue();
                let content_type = xmlReader.attribute(&tag, CONTENT_TYPE_ATTRIBUTE_NAME).unwrap_or_else(|| {
                    // Java: Attribute attrContentType = element.attribute("ContentType");
                    // Java: String contentType = attrContentType.getValue();  ⇒ 同上 NPE
                    panic!("NullPointerException");
                });
                // Java: this.addOverrideContentType(partName, contentType);
                self.addOverrideContentType(&part_name, &content_type);
            }
            // Java: root.detachRecursively();
            Ok(())
        })();
        // Java: catch (URISyntaxException urie)
        // Java: throw new InvalidFormatException(urie.getMessage());
        // Java: catch (DocumentException e)
        // Java: throw new InvalidFormatException(e.getMessage());
        parsed
    }

    // Java: boolean save(OutputStream outStream)
    pub fn save(&mut self, out_stream: &mut dyn std::io::Write) -> bool {
        // Java: Document xmlOutDoc = new DefaultDocument();
        let mut xmlOutDoc = crate::io::dom::tree::DefaultDocument::DefaultDocument::new();
        // Java: Element typesElem = xmlOutDoc.addElement("http://schemas.openxmlformats.org/package/2006/content-types", "", "Types");
        // Java: for(Map.Entry<String, String> entry : this.defaultContentType.entrySet())
        // Java: this.appendDefaultType(typesElem, entry);
        // Java: if (this.overrideContentType != null)
        // Java: for(Map.Entry<PackagePartName, String> entry : this.overrideContentType.entrySet())
        // Java: this.appendSpecificTypes(typesElem, entry);
        // Java: xmlOutDoc.normalize();
        // Java: boolean ret = this.saveImpl(xmlOutDoc, outStream);
        // Java: typesElem.detachRecursively();
        // Java: return ret;
        let _ = &mut xmlOutDoc;
        todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: void appendSpecificTypes(Element root, Map.Entry<PackagePartName, String> entry)
    // Rust: 参数用 Java 的真实类型 `Element`（`cn/wps/io/dom/Element.rs`）承接，**不用** `dyn Any` 兜底。
    //       `Map.Entry` 拆成 `key`/`value`（Rust 无 Entry 借用语义，见 §1.7.1 ⑩）。
    pub fn appendSpecificTypes(&self, _root: &mut dyn crate::io::dom::Element::Element, _key: &str, _value: &str) {
        // Java: root.addElement("Override").addAttribute("PartName", ((PackagePartName)entry.getKey()).getName()).addAttribute("ContentType", (String)entry.getValue());
        // Java: root.addElement / addAttribute（DOM 写侧）
        todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: void appendDefaultType(Element root, Map.Entry<String, String> entry)
    // Rust: 同 `appendSpecificTypes` —— 参数用 Java 的真实类型 `Element`。
    pub fn appendDefaultType(&self, _root: &mut dyn crate::io::dom::Element::Element, _key: &str, _value: &str) {
        // Java: root.addElement("Default").addAttribute("Extension", (String)entry.getKey()).addAttribute("ContentType", (String)entry.getValue());
        // Java: root.addElement / addAttribute（DOM 写侧）
        todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: abstract boolean saveImpl(Document var1, OutputStream var2)
    // Rust: 参数本应为 `&dyn crate::io::dom::Document::Document`（Java 的静态类型），但 `cn/wps/io/dom` 目前
    //       **只有 trait、没有任何 impl**（`DefaultDocument` 未 impl `Document`，`Document::getRootElement`
    //       自身还返回 `dyn Any`）⇒ 用具体 trait 对象会连调用方都构造不出。撤回条件见 §1.7.1 ⑩。
    pub fn saveImpl(&self, _doc: &dyn std::any::Any, _out: &mut dyn std::io::Write) -> bool {
        // abstract in Java — overridden by ZipContentTypeManager
        false
    }
}

// Rust-native: Java `SAXReader` + `Element`/`Attribute` 的**只读替身**（DOM/SAX 层未实现，见 §1.7 偏差 ①）。
// 只提供 `parseContentTypesFile` 实际用到的两件事：按标签名取子元素、取属性值；
// 行为（含缺属性 ⇒ Java NPE、非法 PartName ⇒ 归一消息）由 §3.4 的 CT15–CT20 钉住。
pub struct XmlTypesReader<'a> {
    raw: &'a str,
}

impl<'a> XmlTypesReader<'a> {
    // Rust-native: 对应 Java `new SAXReader()` + `read(in)`（本替身只存字节，解析在 `elementsRO` 里按名取）
    pub fn new(raw: &'a str) -> Self {
        XmlTypesReader { raw }
    }

    // Rust-native: 对应 Java `root.elementsRO(name)`（Java 取**直接**子元素；此处按标签名线性取，
    // 与 Java 在 CT 文件这种浅层结构上的结果一致）。
    pub fn elementsRO(&self, local_name: &str) -> Vec<String> {
        named_start_tags(self.raw, local_name)
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    // Rust-native: 对应 Java `element.attribute(name)`（缺失时 Java 返回 null ⇒ 调用处 `getValue()` NPE）。
    pub fn attribute(&self, tag_inner: &str, name: &str) -> Option<String> {
        attr_value(tag_inner, name)
    }
}

fn named_start_tags<'a>(xml: &'a str, local_name: &str) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(rel) = xml[from..].find('<') {
        let start = from + rel + 1;
        if start >= xml.len() {
            break;
        }
        let after = &xml[start..];
        if after.starts_with('/') {
            from = start;
            continue;
        }
        let name_end = after
            .find(|c: char| c == '>' || c == '/' || c.is_whitespace())
            .unwrap_or(after.len());
        let raw_name = &after[..name_end];
        let local = raw_name.rsplit(':').next().unwrap_or(raw_name);
        if local == local_name {
            let rest = &after[name_end..];
            if let Some(gt) = rest.find('>') {
                let inner = rest[..gt].trim_end_matches('/').trim();
                out.push(inner);
            }
        }
        from = start;
    }
    out
}

fn attr_value(tag_inner: &str, name: &str) -> Option<String> {
    let key = format!("{}=", name);
    let idx = tag_inner.find(&key)?;
    let rest = tag_inner[idx + key.len()..].trim_start();
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let rest = &rest[quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}
