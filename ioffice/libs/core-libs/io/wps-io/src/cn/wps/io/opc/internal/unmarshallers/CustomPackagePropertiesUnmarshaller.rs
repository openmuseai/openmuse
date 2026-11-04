#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.io.TypeConversion
// Java: import cn.wps.io.dom.Attribute
// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.DocumentException
// Java: import cn.wps.io.dom.Element
// Java: import cn.wps.io.dom.io.SAXReader
// Java: import cn.wps.io.opc.CustomPackageProperties
// Java: import cn.wps.io.opc.OPCPackage
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackagePartName
// Java: import cn.wps.io.opc.PackagingURIHelper
// Java: import cn.wps.io.opc.ZipPackagePart
// Java: import cn.wps.io.opc.internal.CustomPackagePropertiesPart
// Java: import cn.wps.io.opc.internal.PartUnmarshaller
// Java: import cn.wps.io.opc.internal.ZipHelper
// Java: import cn.wps.io.xwpf.XWPFPackage
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.text.ParsePosition
// Java: import java.text.SimpleDateFormat
// Java: import java.util.ArrayList
// Java: import java.util.Date
// Java: import java.util.List
// Java: import java.util.TimeZone

use crate::io::dom::tree::DefaultElement::DefaultElement;
use crate::io::opc::CustomPackageProperties::Property;
use crate::io::opc::internal::CustomPackagePropertiesPart::CustomPackagePropertiesPart;
use crate::io::xwpf::XWPFPackage::XWPFPackage;
use crate::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
use super::UnmarshallContext::UnmarshallContext;

// Java field: String TAG
const TAG: Option<&str> = None;
// Java field: String CHILD_NODE_TYPE_PROPERTY
const CHILD_NODE_TYPE_PROPERTY: &str = "property";
// Java field: String CHILD_NODE_TYPE_VECTOR
const CHILD_NODE_TYPE_VECTOR: &str = "vector";
// Java field: String CHILD_NODE_TYPE_ARRAY
const CHILD_NODE_TYPE_ARRAY: &str = "array";
// Java field: String CHILD_NODE_TYPE_BLOB
const CHILD_NODE_TYPE_BLOB: &str = "blob";
// Java field: String CHILD_NODE_TYPE_OBLOB
const CHILD_NODE_TYPE_OBLOB: &str = "oblob";
// Java field: String CHILD_NODE_TYPE_EMPTY
const CHILD_NODE_TYPE_EMPTY: &str = "empty";
// Java field: String CHILD_NODE_TYPE_NULL
const CHILD_NODE_TYPE_NULL: &str = "null";
// Java field: String CHILD_NODE_TYPE_I1
const CHILD_NODE_TYPE_I1: &str = "i1";
// Java field: String CHILD_NODE_TYPE_I2
const CHILD_NODE_TYPE_I2: &str = "i2";
// Java field: String CHILD_NODE_TYPE_I4
const CHILD_NODE_TYPE_I4: &str = "i4";
// Java field: String CHILD_NODE_TYPE_INT
const CHILD_NODE_TYPE_INT: &str = "int";
// Java field: String CHILD_NODE_TYPE_UI1
const CHILD_NODE_TYPE_UI1: &str = "ui1";
// Java field: String CHILD_NODE_TYPE_UI2
const CHILD_NODE_TYPE_UI2: &str = "ui2";
// Java field: String CHILD_NODE_TYPE_UI4
const CHILD_NODE_TYPE_UI4: &str = "ui4";
// Java field: String CHILD_NODE_TYPE_UI8
const CHILD_NODE_TYPE_UI8: &str = "ui8";
// Java field: String CHILD_NODE_TYPE_UINT
const CHILD_NODE_TYPE_UINT: &str = "uint";
// Java field: String CHILD_NODE_TYPE_R4
const CHILD_NODE_TYPE_R4: &str = "r4";
// Java field: String CHILD_NODE_TYPE_R8
const CHILD_NODE_TYPE_R8: &str = "r8";
// Java field: String CHILD_NODE_TYPE_DECIMAL
const CHILD_NODE_TYPE_DECIMAL: &str = "decimal";
// Java field: String CHILD_NODE_TYPE_LPSTR
const CHILD_NODE_TYPE_LPSTR: &str = "lpstr";
// Java field: String CHILD_NODE_TYPE_LPWSTR
const CHILD_NODE_TYPE_LPWSTR: &str = "lpwstr";
// Java field: String CHILD_NODE_TYPE_BSTR
const CHILD_NODE_TYPE_BSTR: &str = "bstr";
// Java field: String CHILD_NODE_TYPE_DATE
const CHILD_NODE_TYPE_DATE: &str = "date";
// Java field: String CHILD_NODE_TYPE_FILETIME
const CHILD_NODE_TYPE_FILETIME: &str = "filetime";
// Java field: String CHILD_NODE_TYPE_BOOL
const CHILD_NODE_TYPE_BOOL: &str = "bool";
// Java field: String CHILD_NODE_TYPE_CY
const CHILD_NODE_TYPE_CY: &str = "cy";
// Java field: String CHILD_NODE_TYPE_ERROR
const CHILD_NODE_TYPE_ERROR: &str = "error";
// Java field: String CHILD_NODE_TYPE_STREAM
const CHILD_NODE_TYPE_STREAM: &str = "stream";
// Java field: String CHILD_NODE_TYPE_OSTREAM
const CHILD_NODE_TYPE_OSTREAM: &str = "ostream";
// Java field: String CHILD_NODE_TYPE_STORAGE
const CHILD_NODE_TYPE_STORAGE: &str = "storage";
// Java field: String CHILD_NODE_TYPE_OSTORAGE
const CHILD_NODE_TYPE_OSTORAGE: &str = "ostorage";
// Java field: String CHILD_NODE_TYPE_VSTREAM
const CHILD_NODE_TYPE_VSTREAM: &str = "vstream";
// Java field: String CHILD_NODE_TYPE_CLSID
const CHILD_NODE_TYPE_CLSID: &str = "clsid";
// Java field: String ATTRIBUTE_FMTID
const ATTRIBUTE_FMTID: &str = "fmtid";
// Java field: String ATTRIBUTE_PID
const ATTRIBUTE_PID: &str = "pid";
// Java field: String ATTRIBUTE_NAME
const ATTRIBUTE_NAME: &str = "name";
// Java field: String ATTRIBUTE_LINK_TARGET
const ATTRIBUTE_LINK_TARGET: &str = "linkTarget";

// rust: no-java —— Java `Element.attribute(String)` 返回接口 `Attribute`，具体类由解析器决定：
// SAX 通路（`SAXReader` → `SAXContentHandler`）产出 `DefaultAttribute`，WPS 快解析通路产出 `FlyweightAttribute`
// ⇒ Rust 侧两种都接受（登记 §1.7.1）。
fn attribute_value(attribute: &dyn std::any::Any) -> Option<String> {
    if let Some(attr) = attribute.downcast_ref::<crate::io::dom::tree::DefaultAttribute::DefaultAttribute>() {
        return Some(attr.getValue());
    }
    if let Some(attr) = attribute.downcast_ref::<crate::io::dom::tree::FlyweightAttribute::FlyweightAttribute>() {
        return Some(attr.getValue());
    }
    None
}

// Java: public class CustomPackagePropertiesUnmarshaller implements PartUnmarshaller
pub struct CustomPackagePropertiesUnmarshaller;

// Java: public class CustomPackagePropertiesUnmarshaller implements PartUnmarshaller
impl CustomPackagePropertiesUnmarshaller {
    // Java: CustomPackagePropertiesUnmarshaller()
    pub fn CustomPackagePropertiesUnmarshaller() -> Self {
        // Java: super();
        CustomPackagePropertiesUnmarshaller
    }

    // rust: no-java（Rust 惯用别名，crate 内沿用 `new`）
    pub fn new() -> Self {
        Self::CustomPackagePropertiesUnmarshaller()
    }

    // Java: PackagePart unmarshall(UnmarshallContext context, InputStream in)
    pub fn unmarshall(
        &self,
        context: &UnmarshallContext,
        input: Option<&mut dyn std::io::Read>,
    ) -> Result<Option<CustomPackagePropertiesPart>, String> {
        // Java: CustomPackagePropertiesPart customProp = new CustomPackagePropertiesPart(context.getPackage(), context.getPartName());
        // rust: 同名构造器 `CustomPackagePropertiesPart(OPCPackage pack, PackagePartName partName)`；本 crate 的包参数
        //       由 `UnmarshallContext` 承载，构造器 `pack` 传 `None`（登记 §1.7.1 ④）
        let partName = context
            .getPartName()
            .as_ref()
            .ok_or_else(|| "partName".to_string())?
            .clone();
                // rust: ★2026-09-21 修正（M-STAGE1 P1-c1）：照 Java 传 `context.getPackage()`；
        //       `PackagePart::loadRelationships()` 已改为真解析 `_rels`，container 为 null 会 panic。
let mut customProp = CustomPackagePropertiesPart::new(*context.getPackage(), partName);
        let mut ownedStream: Option<Box<dyn std::io::Read>> = None;
        // Java: if (in == null) {
        // rust: Java 的 `in` ↔ 本 crate 的 `input`（「调用方未给流」同为 `None` 语义，登记 §1.7.1 ④）
        if input.is_none() {
            // Java: ZipEntry zipEntry = context.getZipEntry();
            let zipEntry = context.getZipEntry();
            // Java: if (zipEntry == null || zipEntry.getName().contains("../")) {
            // Java: return null;
            // ★2026-09-20 落地（原为 §1.7.1 ⑧ 登记的未实现项）：`UnmarshallContext::getZipEntry()`
            //   现在返回**真实类型** `&Option<ZipEntryRef>`（不再是 `&dyn Any` 壳）⇒ `zipEntry.getName()`
            //   可用，Java 的 `../` 越界（zip-slip）校验得以在**此处**按原文实现，
            //   不再依赖 `ZipFileZipEntrySource::getInputStream` 内部兜底。
            let zipEntryName: Option<String> = zipEntry.as_ref().map(|ze| ze.getName().to_string());
            if zipEntry.is_none() || zipEntryName.as_deref().map_or(false, |n| n.contains("../")) {
                return Ok(None);
            }
            // Java: OPCPackage opcPackage = context.getPackage();
            let opcPackage = context.getPackage();
            // Java: if (zipEntry != null) {
            if zipEntry.is_some() {
                // ★2026-09-20 **分支归位**（对齐 `CustomPackagePropertiesUnmarshaller.java`）：
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
                    // Java: in = ((XWPFPackage)opcPackage).getZipArchive().getInputStream(context.getZipEntry());
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
                // Java: PackagePartName partName = PackagingURIHelper.createPartName(ZipHelper.getCustomPropertiesURI((XWPFPackage)context.getPackage()));
                let partName = crate::io::opc::internal::ZipHelper::ZipHelper::getCustomPropertiesURI(opc_pkg)
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
        // Java: root = xmlDoc.getRootElement();
        // Java: } catch (DocumentException e) {
        // Java: throw new IOException(e.getMessage());
        // rust: Java 的 `in` 在本 crate 由 `stream` 承载（`input` 或包内 zip 流，登记 §1.7.1 ④）
        let stream: &mut dyn std::io::Read = if let Some(ref mut s) = ownedStream {
            s.as_mut()
        } else if let Some(s) = input {
            s
        } else {
            return Err("Error while trying to get the part input stream.".to_string());
        };
        // Java: Document xmlDoc = xmlReader.read(in);
        let xmlDoc = match xmlReader.read_stream(stream) {
            Ok(doc) => doc,
            Err(e) => return Err(e.to_string()),
        };
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
        let root_any = match root {
            Some(r) => r,
            None => return Ok(None),
        };
        // Java: List<Element> elements = this.loadProperty(root);
        let elements = self.loadProperty(root_any);
        // Java: if (null == elements) {
        // Java: return null;
        let Some(elements) = elements else {
            return Ok(None);
        };
        // Java: List<CustomPackageProperties.Property> perpertyList = new ArrayList();
        let mut perpertyList: Vec<Box<dyn std::any::Any + Send + Sync>> = Vec::new();
        // Java: int len = elements.size();
        let len = elements.len();
        // Java: for(int i = 0; i < len; ++i) {
        let mut i = 0;
        while i < len {
            // Java: Element ele = (Element)elements.get(i);
            let ele = elements[i].as_ref();
            // Java: if (null != ele) {
            // Java: CustomPackageProperties.Property property = this.convertElement2Property(ele);
            // Java: IOAssert.logNotNull("property should not be null!", property);
            // rust: `IOAssert.logNotNull` 只记日志不抛异常；本 crate 由 `Option` 语义承接（`None` 即不入表，登记 §1.7.1 ⑦）
            if let Some(property) = self.convertElement2Property(ele) {
                // Java: if (null != property) {
                // Java: perpertyList.add(property);
                perpertyList.push(Box::new(property));
            }
            i += 1;
        }
        // Java: root.detachRecursively();
        // rust: no-stmt —— `Element.detachRecursively()` 在本 crate 未接线（DOM 层停车，登记 §1.7.1 ⑩）
        // Java: if (perpertyList.size() > 0) {
        // Java: customProp.setPropertyArray(perpertyList);
        if perpertyList.len() > 0 {
            customProp.setPropertyArray(perpertyList);
        }
        // Java: return customProp;
        Ok(Some(customProp))
    }

    // Java: CustomPackageProperties.Property convertElement2Property(Element element)
    fn convertElement2Property(&self, element: &dyn std::any::Any) -> Option<Property> {
        // Java: IOAssert.assertNotNull("element should not be null!", element);
        // Java: String name = this.loadName(element);
        // Java: IOAssert.logNotNull("name should not be null!", name);
        // Java: if (null == name) {
        // Java: return null;
        // rust: 上述断言/判空在本 crate 由 `?` 承接（`None` ⇒ 直接返回 `None`，登记 §1.7.1 ⑦）
        let name = self.loadName(element)?;
        // Java: CustomPackageProperties.Property property = null;
        let mut property: Option<Property> = None;
        // Java: String strVal = this.loadLpwstr(element);
        // Java: if (null != strVal) {
        // Java: property = new CustomPackageProperties.Property(name, strVal);
        if let Some(str_val) = self.loadLpwstr(element) {
            property = Some(Property::new_str(name.clone(), str_val));
        }
        // Java: Boolean booVal = this.loadBool(element);
        // Java: if (null != booVal) {
        // Java: property = new CustomPackageProperties.Property(name, booVal);
        if let Some(boo_val) = self.loadBool(element) {
            property = Some(Property::new_bool(name.clone(), boo_val));
        }
        // Java: Integer intVal = this.loadI4(element);
        // Java: if (null != intVal) {
        // Java: property = new CustomPackageProperties.Property(name, intVal);
        if let Some(int_val) = self.loadI4(element) {
            property = Some(Property::new_int(name.clone(), int_val));
        }
        // Java: Double doubleVal = this.loadR8(element);
        // Java: if (null != doubleVal) {
        // Java: property = new CustomPackageProperties.Property(name, doubleVal);
        if let Some(double_val) = self.loadR8(element) {
            property = Some(Property::new_double(name.clone(), double_val));
        }
        // Java: Date date = this.loadFileTime(element);
        // Java: if (null != date) {
        // Java: property = new CustomPackageProperties.Property(name, date);
        if let Some(date) = self.loadFileTime(element) {
            property = Some(Property::new_date(name.clone(), date));
        }
        // Java: if (null == property) {
        // Java: IOAssert.fail("the Type of property not support!");
        // Java: return null;
        // rust: `IOAssert.fail` 在本 crate 由 `?` 承接（类型不受支持 ⇒ 返回 `None`，登记 §1.7.1 ⑦）
        let mut property = property?;
        // Java: property.mLinkTarge = this.loadLinkTarget(element);
        property.mLinkTarge = self.loadLinkTarget(element);
        // Java: return property;
        Some(property)
    }
    // Java: List<Element> loadProperty(Element root)
    fn loadProperty(&self, root: &dyn std::any::Any) -> Option<Vec<Box<dyn std::any::Any>>> {
        // Java: List<Element> el = root.elements("property");
        // Java: return null == el ? null : el;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let list = el.elements_2("property");
        Some(list)
    }
    // Java: Integer loadFmtid(Element root)
    fn loadFmtid(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("fmtid");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element("fmtid")?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        let value = child_el.getStringValue();
        if value.len() > 0 {
            // Java: TypeConversion.string2Integer(value)
            core_libs_util::cn::wps::base::io::TypeConversion::TypeConversion::string2Integer(&value)
        } else {
            None
        }
    }
    // Java: String loadName(Element root)
    fn loadName(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Attribute el = root.attribute("name");
        // Java: return null == el ? null : el.getValue();
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let attr = el.attribute_2("name")?;
        // Java: element.attribute("name").getValue()（`Attribute` 是接口，具体类由解析器决定）
        attribute_value(attr.as_ref())
    }
    // Java: String loadLinkTarget(Element root)
    fn loadLinkTarget(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Attribute el = root.attribute("linkTarget");
        // Java: return null == el ? null : el.getValue();
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let attr = el.attribute_2("linkTarget")?;
        // Java: element.attribute("linkTarget").getValue()
        attribute_value(attr.as_ref())
    }
    // Java: Boolean loadBool(Element root)
    fn loadBool(&self, root: &dyn std::any::Any) -> Option<bool> {
        // Java: Element el = root.element("bool");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? new Boolean(el.getStringValue()) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element("bool")?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        let value = child_el.getStringValue();
        if value.len() > 0 {
            Some(value.eq_ignore_ascii_case("true"))
        } else {
            None
        }
    }
    // Java: String loadLpwstr(Element root)
    fn loadLpwstr(&self, root: &dyn std::any::Any) -> Option<String> {
        // Java: Element el = root.element("lpwstr");
        // Java: return null == el ? null : el.getStringValue();
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element("lpwstr")?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        Some(child_el.getStringValue())
    }
    // Java: Integer loadI4(Element root)
    fn loadI4(&self, root: &dyn std::any::Any) -> Option<i32> {
        // Java: Element el = root.element("i4");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Integer(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element("i4")?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        let value = child_el.getStringValue();
        if value.len() > 0 {
            // Java: TypeConversion.string2Integer(value)
            core_libs_util::cn::wps::base::io::TypeConversion::TypeConversion::string2Integer(&value)
        } else {
            None
        }
    }
    // Java: Double loadR8(Element root)
    fn loadR8(&self, root: &dyn std::any::Any) -> Option<f64> {
        // Java: Element el = root.element("r8");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return null != value && value.length() > 0 ? TypeConversion.string2Double(value) : null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element("r8")?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        let value = child_el.getStringValue();
        if value.len() > 0 {
            // Java: TypeConversion.string2Double(value)
            core_libs_util::cn::wps::base::io::TypeConversion::TypeConversion::string2Double(&value)
        } else {
            None
        }
    }
    // Java: Date loadFileTime(Element root)
    fn loadFileTime(&self, root: &dyn std::any::Any) -> Option<i64> {
        // Java: Element el = root.element("filetime");
        // Java: if (null == el) {
        // Java: return null;
        // Java: String value = el.getStringValue();
        // Java: return this.setDateValue(value);
        // Java: } catch (InvalidFormatException var5) {
        // Java: return null;
        // rust: 上述 koffice 语句由本 crate 的 `load_*` 辅助/下方实现一步完成（登记 §1.7.1 ⑤）
        let el = root.downcast_ref::<DefaultElement>()?;
        let child = el.element("filetime")?;
        let child_el = child.downcast_ref::<DefaultElement>()?;
        let value = child_el.getStringValue();
        match self.setDateValue(&value) {
            Ok(v) => v,
            Err(_) => None,
        }
    }
    // Java: Date setDateValue(String s)
    fn setDateValue(&self, s: &str) -> Result<Option<i64>, String> {
        // Java: if (s != null && !s.equals("")) {
        if !s.is_empty() {
            // Java: df.setTimeZone(TimeZone.getTimeZone("UTC"));
            // Java: SimpleDateFormat df = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss'Z'");
            // rust: 本 crate 无 `SimpleDateFormat`/`TimeZone` ⇒ 以闭包 `df` 承接「格式串 + UTC」两件事；
            //       解析按 `SimpleDateFormat` 的**前缀**语义（`find('Z')` 截断，尾随字符忽略，登记 §1.7.1 ⑨）
            let df = |text: &str| -> Option<(i64, i64, i64, i64, i64, i64)> {
                let head = match text.find('Z') {
                    Some(idx) => &text[..idx],
                    None => text,
                };
                let parts: Vec<&str> = head.split('T').collect();
                if parts.len() != 2 {
                    return None;
                }
                let date_parts: Vec<&str> = parts[0].split('-').collect();
                let time_parts: Vec<&str> = parts[1].split(':').collect();
                if date_parts.len() != 3 || time_parts.len() != 3 {
                    return None;
                }
                Some((
                    date_parts[0].parse().ok()?,
                    date_parts[1].parse().ok()?,
                    date_parts[2].parse().ok()?,
                    time_parts[0].parse().ok()?,
                    time_parts[1].parse().ok()?,
                    time_parts[2].parse().ok()?,
                ))
            };
            // Java: Date d = df.parse(s, new ParsePosition(0));
            let d = df(s);
            // Java: if (d == null) {
            let Some((year, month, day, hour, min, sec)) = d else {
                // Java: throw new InvalidFormatException("Date not well formated");
                return Err("Date not well formated".to_string());
            };
            // Java: return d;
            // rust: 与 Java `Date` 一致，返回 epoch 毫秒（登记 §1.7.1 ⑨）
            let y = if month <= 2 { year - 1 } else { year };
            let era = y.div_euclid(400);
            let yoe = y.rem_euclid(400);
            let mp = if month > 2 { month - 3 } else { month + 9 };
            let doy = (153 * mp + 2) / 5 + day - 1;
            let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
            let days = era * 146097 + doe - 719468;
            Ok(Some((days * 86400 + hour * 3600 + min * 60 + sec) * 1000))
        } else {
            // Java: return null;
            Ok(None)
        }
    }
}


// NOTE(impl-trait): public class CustomPackagePropertiesUnmarshaller implements PartUnmarshaller
impl crate::io::opc::internal::PartUnmarshaller::PartUnmarshaller for CustomPackagePropertiesUnmarshaller {
    // NOTE(impl-trait): PackagePart unmarshall(UnmarshallContext context, InputStream in)
    fn unmarshall(&self, context: &crate::io::opc::internal::unmarshallers::UnmarshallContext::UnmarshallContext, input: Option<&mut dyn std::io::Read>) -> Result<Option<crate::io::opc::PackagePart::PackagePart>, String> {
        Ok(CustomPackagePropertiesUnmarshaller::unmarshall(self, context, input)?.map(|p| p.base))
    }
}
