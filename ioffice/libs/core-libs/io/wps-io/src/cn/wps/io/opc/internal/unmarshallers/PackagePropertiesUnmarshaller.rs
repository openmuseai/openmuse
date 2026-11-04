#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.dom.Attribute
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.DocumentException
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.Namespace
// Java: import cn.wps.io.dom.io.SAXReader
// Java: import cn.wps.io.opc.OPCPackage
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackagePartName
// Java: import cn.wps.io.opc.PackagingURIHelper
// Java: import cn.wps.io.opc.ZipPackagePart
// Java: import cn.wps.io.opc.internal.PackagePropertiesPart
// Java: import cn.wps.io.opc.internal.PartUnmarshaller
// Java: import cn.wps.io.opc.internal.ZipHelper
// Java: import cn.wps.io.xwpf.XWPFPackage
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.Iterator

use crate::io::dom::Namespace::Namespace;
use crate::io::dom::tree::DefaultElement::DefaultElement;
use crate::io::opc::internal::PackagePropertiesPart::PackagePropertiesPart;
use crate::io::xwpf::XWPFPackage::XWPFPackage;
use crate::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
use super::UnmarshallContext::UnmarshallContext;

// Java field: String TAG
const TAG: Option<&str> = None;
// Java field: Namespace namespaceDC
const NAMESPACE_DC_URI: &str = "http://purl.org/dc/elements/1.1/";
// Java field: Namespace namespaceCP
const NAMESPACE_CP_URI: &str = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
// Java field: Namespace namespaceDcTerms
const NAMESPACE_DC_TERMS_URI: &str = "http://purl.org/dc/terms/";
// Java field: Namespace namespaceXML
const NAMESPACE_XML_URI: &str = "http://www.w3.org/XML/1998/namespace";
// Java field: Namespace namespaceXSI
const NAMESPACE_XSI_URI: &str = "http://www.w3.org/2001/XMLSchema-instance";
const NAMESPACE_XSI_PREFIX: &str = "xsi";
const MARKUP_COMPAT_NS: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";

// Java field: String KEYWORD_CATEGORY
pub(crate) const KEYWORD_CATEGORY: &str = "category";
// Java field: String KEYWORD_CONTENT_STATUS
pub(crate) const KEYWORD_CONTENT_STATUS: &str = "contentStatus";
// Java field: String KEYWORD_CONTENT_TYPE
pub(crate) const KEYWORD_CONTENT_TYPE: &str = "contentType";
// Java field: String KEYWORD_CREATED
pub(crate) const KEYWORD_CREATED: &str = "created";
// Java field: String KEYWORD_CREATOR
pub(crate) const KEYWORD_CREATOR: &str = "creator";
// Java field: String KEYWORD_DESCRIPTION
pub(crate) const KEYWORD_DESCRIPTION: &str = "description";
// Java field: String KEYWORD_IDENTIFIER
pub(crate) const KEYWORD_IDENTIFIER: &str = "identifier";
// Java field: String KEYWORD_KEYWORDS
pub(crate) const KEYWORD_KEYWORDS: &str = "keywords";
// Java field: String KEYWORD_LANGUAGE
pub(crate) const KEYWORD_LANGUAGE: &str = "language";
// Java field: String KEYWORD_LAST_MODIFIED_BY
pub(crate) const KEYWORD_LAST_MODIFIED_BY: &str = "lastModifiedBy";
// Java field: String KEYWORD_LAST_PRINTED
pub(crate) const KEYWORD_LAST_PRINTED: &str = "lastPrinted";
// Java field: String KEYWORD_MODIFIED
pub(crate) const KEYWORD_MODIFIED: &str = "modified";
// Java field: String KEYWORD_REVISION
pub(crate) const KEYWORD_REVISION: &str = "revision";
// Java field: String KEYWORD_SUBJECT
pub(crate) const KEYWORD_SUBJECT: &str = "subject";
// Java field: String KEYWORD_TITLE
pub(crate) const KEYWORD_TITLE: &str = "title";
// Java field: String KEYWORD_VERSION
pub(crate) const KEYWORD_VERSION: &str = "version";

// Java: public class PackagePropertiesUnmarshaller implements PartUnmarshaller
pub struct PackagePropertiesUnmarshaller;

// Java: public class PackagePropertiesUnmarshaller implements PartUnmarshaller
impl PackagePropertiesUnmarshaller {
    // Java: PackagePropertiesUnmarshaller()
    pub fn PackagePropertiesUnmarshaller() -> Self {
        // Java: super();
        PackagePropertiesUnmarshaller
    }

    // rust: no-java（Rust 惯用别名，crate 内沿用 `new`）
    pub fn new() -> Self {
        Self::PackagePropertiesUnmarshaller()
    }

    // Java: PackagePart unmarshall(UnmarshallContext context, InputStream in)
    pub fn unmarshall(&self, context: &UnmarshallContext, input: Option<&mut dyn std::io::Read>) -> Result<PackagePropertiesPart, String> {
        // Java: PackagePropertiesPart coreProps = new PackagePropertiesPart(context.getPackage(), context.getPartName());
        // rust: ★2026-09-21 修正（M-STAGE1 P1-c1）：此前 `pack` 传 `None`（登记 §1.7.1 ④）；
        //       但 `PackagePart::loadRelationships()` 现在**真解析** `_rels`，并沿用 Java
        //       `PackageRelationshipCollection` 构造器的 `if (container == null) throw` 判据
        //       ⇒ 必须照 Java 传 `context.getPackage()`，否则构造即 panic。
        let partName = context
            .getPartName()
            .as_ref()
            .ok_or_else(|| "partName".to_string())?
            .clone();
        let mut coreProps = PackagePropertiesPart::new(*context.getPackage(), partName);
        let mut ownedStream: Option<Box<dyn std::io::Read>> = None;
        // Java: if (in == null) {
        // rust: Java 的 `in` ↔ 本 crate 的 `input`（「调用方未给流」同为 `None` 语义，登记 §1.7.1 ④）
        if input.is_none() {
            // Java: ZipEntry zipEntry = context.getZipEntry();
            let zipEntry = context.getZipEntry();
            // Java: if (zipEntry != null) {
            // Java: String zipEntryName = zipEntry.getName();
            // Java: if (zipEntryName != null && zipEntryName.contains("../")) {
            // Java: throw new IOException("Unsafe zip file.");
            // ★2026-09-20 落地（原为 §1.7.1 ⑧ 登记的未实现项）：`UnmarshallContext::getZipEntry()`
            //   现在返回**真实类型** `&Option<ZipEntryRef>`（不再是 `&dyn Any` 壳）⇒ `zipEntry.getName()`
            //   可用，Java 的 `../` 越界（zip-slip）校验得以在**此处**按原文实现，
            //   不再依赖 `ZipFileZipEntrySource::getInputStream` 内部兜底。
            let zipEntryName: Option<String> = zipEntry.as_ref().map(|ze| ze.getName().to_string());
            if zipEntryName.as_deref().map_or(false, |n| n.contains("../")) {
                return Err("Unsafe zip file.".to_string());
            }
            // Java: OPCPackage opcPackage = context.getPackage();
            let opcPackage = context.getPackage();
            // Java: if (zipEntry != null) {
            if zipEntry.is_some() {
                // ★2026-09-20 **分支归位**（对齐 `PackagePropertiesUnmarshaller.java:66-67`）：
                //   Java 的 `if (zipEntry != null)` 分支体**只有一句**
                //   `in = ((XWPFPackage)opcPackage).getZipArchive().getInputStream(zipEntry);`；
                //   而 `getPart` / `zipPackagePart` 那几行属于 **`else` 分支**。旧 Rust 把它们
                //   错放进了 `if` 分支 ⇒ 本次按 Java 原文归位（这也是 §3.8 报
                //   "Java field 'zipPackagePart' has no adjacent Rust identifier" 的根因）。
                if let Some(pkg_ptr) = opcPackage {
                    // ★`UnmarshallContext._package` 已是真实类型 `*const OPCPackage`（不再是 `dyn Any`）
                    //   ⇒ Java 的 `(XWPFPackage)opcPackage` 向上强转不再需要：`getZipArchive()`
                    //   声明在**基类** `OPCPackage`（`OPCPackage.java:77`）上，直接在真实基类引用上调用。
                    let opc_pkg: &crate::io::opc::OPCPackage::OPCPackage = unsafe { &**pkg_ptr };
                    // Java: in = ((XWPFPackage)opcPackage).getZipArchive().getInputStream(zipEntry);
                    // ★去掉 `archive.as_any().downcast_ref::<ZipFileZipEntrySource>()`：Java 是在
                    //   `ZipEntrySource` **接口**上直接调 `getInputStream` ⇒ Rust 直接在 trait 对象上调同名方法。
                    if let Some(ze) = zipEntry.as_ref() {
                        if let Some(archive) = opc_pkg.getZipArchive().as_ref() {
                            // ★`ze` 就是 `&ZipEntryRef` ⇒ 原样透传
                            if let Ok(stream) = archive.getInputStream_with_flag(ze, false) {
                                ownedStream = Some(stream);
                            }
                        }
                    }
                }
            // Java: else
            } else {
                // Java: if (opcPackage == null) {
                if opcPackage.is_none() {
                    // Java: throw new IOException("Error while trying to get the part input stream.");
                    return Err("Error while trying to get the part input stream.".to_string());
                }
                // ★2026-09-20：以下 `else` 分支体本次**按 Java 原文实现**（旧实现是注释 + no-stmt）。
                let opc_pkg: &mut crate::io::opc::OPCPackage::OPCPackage =
                    unsafe { &mut *(*opcPackage.as_ref().unwrap() as *mut crate::io::opc::OPCPackage::OPCPackage) };
                // Java: PackagePartName partName = PackagingURIHelper.createPartName(ZipHelper.getCorePropertiesURI((XWPFPackage)context.getPackage()));
                let partName = crate::io::opc::internal::ZipHelper::ZipHelper::getCorePropertiesURI(opc_pkg)
                    .and_then(|uri| crate::io::opc::PackagingURIHelper::PackagingURIHelper::createPartName_uri(&uri).ok());
                // Java: PackagePart part = opcPackage.getPart(partName);
                let part = partName.as_ref().and_then(|pn| opc_pkg.getPart(pn));
                // Java: if (part != null && part instanceof ZipPackagePart) {
                //   ★Rust 组合式建模：Java 的 `instanceof ZipPackagePart` ⇔ 该 part 带 zip entry
                if part.map_or(false, |p| p.zipEntry.is_some()) {
                    // Java: ZipPackagePart zipPackagePart = (ZipPackagePart)part;
                    let zipPackagePart = part.expect("part != null");
                    // Java: zipEntry = zipPackagePart.getZipArchive();
                    let zipEntry = zipPackagePart.zipEntry.clone();
                    // Java: in = ((XWPFPackage)opcPackage).getZipArchive().getInputStream(zipEntry);
                    if let (Some(ze), Some(archive)) =
                        (zipEntry.as_ref(), opc_pkg.getZipArchive().as_ref())
                    {
                        if let Ok(stream) = archive.getInputStream_with_flag(ze, false) {
                            ownedStream = Some(stream);
                        }
                    }
                }
            }
        }
        // Java: SAXReader xmlReader = new SAXReader();
        let mut xmlReader = crate::io::dom::io::SAXReader::SAXReader::new();
        // Java: try {
        // Java: xmlDoc = xmlReader.read(in);
        // Java: } catch (DocumentException e) {
        // Java: IOLog.e(TAG, "DocumentException :", e);
        // Java: throw new IOException(e.getMessage());
        // rust: Java 的 `in` 在本 crate 由 `stream` 承载（`input` 或包内 zip 流，登记 §1.7.1 ④）
        let stream: &mut dyn std::io::Read = if let Some(ref mut s) = ownedStream {
            s.as_mut()
        } else if let Some(s) = input {
            s
        } else {
            return Err("Error while trying to get the part input stream.".to_string());
        };
        // Java: xmlDoc = xmlReader.read(in);
        // Java: Document xmlDoc = null;
        let xmlDoc = match xmlReader.read_stream(stream) {
            Ok(doc) => doc,
            Err(e) => {
                let _ = TAG;
                return Err(e.to_string());
            }
        };
        // Java: root = xmlDoc.getRootElement();
        // Java: Element root = null;
        let root_any: &dyn std::any::Any = xmlDoc.as_ref();
        let root = if let Some(dd) =
            root_any.downcast_ref::<crate::io::dom::tree::DefaultDocument::DefaultDocument>()
        {
            dd.getRootElement()
                .map(|r| r.as_ref() as &dyn std::any::Any)
        } else {
            None
        };
        let root = root.unwrap_or(root_any);
        // Java: coreProps.setCategoryProperty(this.loadCategory(root));
        coreProps.setCategoryProperty(self.loadCategory(root).as_deref());
        // Java: coreProps.setContentStatusProperty(this.loadContentStatus(root));
        coreProps.setContentStatusProperty(self.loadContentStatus(root).as_deref());
        // Java: coreProps.setContentTypeProperty(this.loadContentType(root));
        coreProps.setContentTypeProperty(self.loadContentType(root).as_deref());
        // Java: coreProps.setCreatedProperty(this.loadCreated(root));
        coreProps.setCreatedProperty_str(self.loadCreated(root).as_deref());
        // Java: coreProps.setCreatorProperty(this.loadCreator(root));
        coreProps.setCreatorProperty(self.loadCreator(root).as_deref());
        // Java: coreProps.setDescriptionProperty(this.loadDescription(root));
        coreProps.setDescriptionProperty(self.loadDescription(root).as_deref());
        // Java: coreProps.setIdentifierProperty(this.loadIdentifier(root));
        coreProps.setIdentifierProperty(self.loadIdentifier(root).as_deref());
        // Java: coreProps.setKeywordsProperty(this.loadKeywords(root));
        coreProps.setKeywordsProperty(self.loadKeywords(root).as_deref());
        // Java: coreProps.setLanguageProperty(this.loadLanguage(root));
        coreProps.setLanguageProperty(self.loadLanguage(root).as_deref());
        // Java: coreProps.setLastModifiedByProperty(this.loadLastModifiedBy(root));
        coreProps.setLastModifiedByProperty(self.loadLastModifiedBy(root).as_deref());
        // Java: coreProps.setLastPrintedProperty(this.loadLastPrinted(root));
        coreProps.setLastPrintedProperty_str(self.loadLastPrinted(root).as_deref());
        // Java: coreProps.setModifiedProperty(this.loadModified(root));
        coreProps.setModifiedProperty_str(self.loadModified(root).as_deref());
        // Java: coreProps.setRevisionProperty(this.loadRevision(root));
        coreProps.setRevisionProperty(self.loadRevision(root).as_deref());
        // Java: coreProps.setSubjectProperty(this.loadSubject(root));
        coreProps.setSubjectProperty(self.loadSubject(root).as_deref());
        // Java: coreProps.setTitleProperty(this.loadTitle(root));
        coreProps.setTitleProperty(self.loadTitle(root).as_deref());
        // Java: coreProps.setVersionProperty(this.loadVersion(root));
        coreProps.setVersionProperty(self.loadVersion(root).as_deref());
        // Java: root.detachRecursively();
        // rust: no-stmt —— `Element.detachRecursively()` 在本 crate 未接线（DOM 层停车，登记 §1.7.1 ⑩）
        // Java: return coreProps;
        Ok(coreProps)
    }

    fn load_elem(&self, root: &dyn std::any::Any, name: &str, ns_uri: &str) -> Option<String> {
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element_2(name, ns_uri)?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        Some(child_el.getStringValue())
    }

    // Java: String loadCategory(Element root)
    fn loadCategory(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("category", namespaceCP.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "category", NAMESPACE_CP_URI)
    }
    // Java: String loadContentStatus(Element root)
    fn loadContentStatus(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("contentStatus", namespaceCP.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "contentStatus", NAMESPACE_CP_URI)
    }
    // Java: String loadContentType(Element root)
    fn loadContentType(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("contentType", namespaceCP.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "contentType", NAMESPACE_CP_URI)
    }
    // Java: String loadCreated(Element root)
    fn loadCreated(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("created", namespaceDcTerms.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "created", NAMESPACE_DC_TERMS_URI)
    }
    // Java: String loadCreator(Element root)
    fn loadCreator(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("creator", namespaceDC.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "creator", NAMESPACE_DC_URI)
    }
    // Java: String loadDescription(Element root)
    fn loadDescription(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("description", namespaceDC.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "description", NAMESPACE_DC_URI)
    }
    // Java: String loadIdentifier(Element root)
    fn loadIdentifier(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("identifier", namespaceDC.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "identifier", NAMESPACE_DC_URI)
    }
    // Java: String loadKeywords(Element root)
    fn loadKeywords(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("keywords", namespaceCP.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "keywords", NAMESPACE_CP_URI)
    }
    // Java: String loadLanguage(Element root)
    fn loadLanguage(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("language", namespaceDC.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "language", NAMESPACE_DC_URI)
    }
    // Java: String loadLastModifiedBy(Element root)
    fn loadLastModifiedBy(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("lastModifiedBy", namespaceCP.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "lastModifiedBy", NAMESPACE_CP_URI)
    }
    // Java: String loadLastPrinted(Element root)
    fn loadLastPrinted(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("lastPrinted", namespaceCP.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "lastPrinted", NAMESPACE_CP_URI)
    }
    // Java: String loadModified(Element root)
    fn loadModified(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("modified", namespaceDcTerms.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "modified", NAMESPACE_DC_TERMS_URI)
    }
    // Java: String loadRevision(Element root)
    fn loadRevision(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("revision", namespaceCP.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "revision", NAMESPACE_CP_URI)
    }
    // Java: String loadSubject(Element root)
    fn loadSubject(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("subject", namespaceDC.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "subject", NAMESPACE_DC_URI)
    }
    // Java: String loadTitle(Element root)
    fn loadTitle(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("title", namespaceDC.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "title", NAMESPACE_DC_URI)
    }
    // Java: String loadVersion(Element root)
    fn loadVersion(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("version", namespaceCP.getURI());
        // Java: return el == null ? null : el.getStringValue();
        // rust: 上述两条由本 crate 的 `load_elem` 一步完成（同名等价实现，登记 §1.7.1 ⑤）
        self.load_elem(root, "version", NAMESPACE_CP_URI)
    }

    // Java: void checkElementForOPCCompliance(Element el)
    pub fn checkElementForOPCCompliance(&self, el: &dyn std::any::Any) -> Result<(), String> {
        // rust: Java 参数是 `Element`；本 crate 的门面沿用 `&dyn Any` 壳（登记 §1.7.1 ⑪）
        let el = el
            .downcast_ref::<DefaultElement>()
            .ok_or_else(|| "el".to_string())?;
        // Java: for(Namespace ns : el.declaredNamespaces()) {
        for ns in el.declaredNamespaces() {
            if let Some(ns_el) = ns.downcast_ref::<Namespace>() {
                // Java: if (ns.getURI().equals("http://schemas.openxmlformats.org/markup-compatibility/2006")) {
                if ns_el.getURI() == MARKUP_COMPAT_NS {
                    // Java: throw new InvalidFormatException("OPC Compliance error [M4.2]: A format consumer shall consider the use of the Markup Compatibility namespace to be an error.");
                    return Err("OPC Compliance error [M4.2]: A format consumer shall consider the use of the Markup Compatibility namespace to be an error.".to_string());
                }
            }
        }
        // Java: if (el.getNamespaceURI().equals("http://purl.org/dc/terms/") && !el.getName().equals("created") && !el.getName().equals("modified")) {
        if el.getNamespaceURI() == NAMESPACE_DC_TERMS_URI
            && el.getName() != "created"
            && el.getName() != "modified"
        {
            // Java: throw new InvalidFormatException("OPC Compliance error [M4.3]: Producers shall not create a document element that contains refinements to the Dublin Core elements, except for the two specified in the schema: <dcterms:created> and <dcterms:modified> Consumers shall consider a document element that violates this constraint to be an error.");
            return Err("OPC Compliance error [M4.3]: Producers shall not create a document element that contains refinements to the Dublin Core elements, except for the two specified in the schema: <dcterms:created> and <dcterms:modified> Consumers shall consider a document element that violates this constraint to be an error.".to_string());
        // Java: } else if (el.attribute("lang", namespaceXML.getURI()) != null) {
        } else if el.attribute_3("lang", NAMESPACE_XML_URI).is_some() {
            // Java: throw new InvalidFormatException("OPC Compliance error [M4.4]: Producers shall not create a document element that contains the xml:lang attribute. Consumers shall consider a document element that violates this constraint to be an error.");
            return Err("OPC Compliance error [M4.4]: Producers shall not create a document element that contains the xml:lang attribute. Consumers shall consider a document element that violates this constraint to be an error.".to_string());
        // Java: } else {
        } else {
            // Java: if (el.getNamespaceURI().equals("http://purl.org/dc/terms/")) {
            if el.getNamespaceURI() == NAMESPACE_DC_TERMS_URI {
                // Java: String elName = el.getName();
                let el_name = el.getName();
                // Java: if (!elName.equals("created") && !elName.equals("modified")) {
                if el_name != "created" && el_name != "modified" {
                    // Java: throw new InvalidFormatException("Namespace error : " + elName + " shouldn't have the following naemspace -> " + "http://purl.org/dc/terms/");
                    return Err(format!(
                        "Namespace error : {} shouldn't have the following naemspace -> {}",
                        el_name, NAMESPACE_DC_TERMS_URI
                    ));
                }
                // Java: Attribute typeAtt = el.attribute("type", namespaceXSI.getURI());
                // rust: Java 的 `Element.attribute(name, ns)` ↔ 本 crate 的 `attribute_3`（登记 §1.7.1 ⑪）
                let mut type_att = el.attribute_3("type", NAMESPACE_XSI_URI);
                // Java: if (null == typeAtt) {
                if type_att.is_none() {
                    // Java: typeAtt = el.attribute("type");
                    // rust: Java 的单参重载 `attribute(name)` ↔ 本 crate 的 `attribute_2`（登记 §1.7.1 ⑪）
                    type_att = el.attribute_2("type");
                }
                // Java: if (typeAtt == null) {
                if type_att.is_none() {
                    // Java: throw new InvalidFormatException("The element '" + elName + "' must have the '" + namespaceXSI.getPrefix() + ":type' attribute present !");
                    return Err(format!(
                        "The element '{}' must have the '{}:type' attribute present !",
                        el_name, NAMESPACE_XSI_PREFIX
                    ));
                }
                // Java: if (!typeAtt.getValue().equals("dcterms:W3CDTF")) {
                // Java: throw new InvalidFormatException("The element '" + elName + "' must have the '" + namespaceXSI.getPrefix() + ":type' attribute with the value 'dcterms:W3CDTF' !");
                // rust: no-stmt —— `DefaultElement::attribute_3` 仍是 `todo!()` 停车体、属性值的 `Nullable`
                //       取值在本 crate 未接线 ⇒ 该校验暂缺（登记 §1.7.1 ⑩，owner: DOM 层 function）
            }
            // Java: Iterator<Element> itChildren = el.elementIterator();
            // Java: while(itChildren.hasNext()) {
            // Java: this.checkElementForOPCCompliance((Element)itChildren.next());
            // rust: Java 的 `Iterator<Element>` 在本 crate 由 `elementIterator()` 直接迭代（登记 §1.7.1 ⑪）；
            //       注：`DefaultElement::elementIterator` 目前是 `todo!()` 停车体（登记 §1.7.1 ⑩）
            for child in el.elementIterator() {
                self.checkElementForOPCCompliance(child.as_ref())?;
            }
        }
        Ok(())
    }
}


// NOTE(impl-trait): public class PackagePropertiesUnmarshaller implements PartUnmarshaller
impl crate::io::opc::internal::PartUnmarshaller::PartUnmarshaller for PackagePropertiesUnmarshaller {
    // NOTE(impl-trait): PackagePart unmarshall(UnmarshallContext context, InputStream in)
    // NOTE(impl-trait) 多态返回子类；组合模式降级为基类值
    fn unmarshall(&self, context: &crate::io::opc::internal::unmarshallers::UnmarshallContext::UnmarshallContext, input: Option<&mut dyn std::io::Read>) -> Result<Option<crate::io::opc::PackagePart::PackagePart>, String> {
        let part = PackagePropertiesUnmarshaller::unmarshall(self, context, input)?;
        Ok(Some(part.base))
    }
}
