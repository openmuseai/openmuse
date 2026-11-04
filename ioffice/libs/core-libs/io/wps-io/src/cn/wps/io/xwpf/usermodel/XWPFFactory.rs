#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.gvml.usermodel.GVmlFactory
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackageRelationship
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.opc.model.POIXMLException
// Java: import cn.wps.io.opc.model.POIXMLFactory
// Java: import cn.wps.io.opc.model.POIXMLRelation
// Java: import cn.wps.io.xwpf.DmlRelation
// Java: import java.lang.reflect.Constructor

use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackageRelationship::PackageRelationship;
use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use crate::io::opc::model::POIXMLRelation::POIXMLRelation;
use crate::io::opc::model::POIXMLFactory::POIXMLFactory;
use super::XWPFRelation::XWPFRelation;
use super::XWPFRelationPurl::XWPFRelationPurl;

// Java field: String TAG
// Java field: POIXMLFactory inst
static mut INST: Option<XWPFFactory> = None;

// Java: public class XWPFFactory extends POIXMLFactory
pub struct XWPFFactory {
    _private: (),
}

// Java: public class XWPFFactory extends POIXMLFactory
impl XWPFFactory {
    // Java: XWPFFactory()
    fn new() -> Self {
        // Java: super();
        XWPFFactory { _private: () }
    }

    // Java: static POIXMLFactory getInstance()
    pub fn getInstance() -> &'static XWPFFactory {
        // Java: return inst;
        unsafe {
            if INST.is_none() {
                INST = Some(XWPFFactory::new());
            }
            INST.as_ref().unwrap()
        }
    }

    // Java: static void setGVmlFactory()
    pub fn setGVmlFactory() {
        // Java: DmlRelation.setGVmlRelation();
        // Java: inst = GVmlFactory.getInstance();
    }

    // Java: static void setXWPFFactory()
    pub fn setXWPFFactory() {
        // Java: DmlRelation.setXWPFRelation();
        // Java: inst = new XWPFFactory();
        let inst = XWPFFactory::new();
        unsafe {
            INST = Some(inst);
        }
    }

    // Java: POIXMLDocumentPart createDocumentPart(POIXMLDocumentPart parent, PackageRelationship rel, PackagePart part)
    pub fn createDocumentPart(&self, parent: &POIXMLDocumentPart, rel: &PackageRelationship, part: &PackagePart) -> Option<POIXMLDocumentPart> {
        // Java: try
        // Java: Class<? extends POIXMLDocumentPart> cls = getPOIXMLDocumentPartClass(rel.getRelationshipType());
        let cls = Self::getPOIXMLDocumentPartClass(rel.getRelationshipType());
        // Java: if (null == cls)
        if cls.is_none() {
            // Java: return null;
            return None;
        // Java: else
        } else {
            // Java: try
            // Java: Constructor<? extends POIXMLDocumentPart> constructor = cls.getDeclaredConstructor(POIXMLDocumentPart.class, PackagePart.class, PackageRelationship.class);
            // Java: return (POIXMLDocumentPart)constructor.newInstance(parent, part, rel);
            // Java: catch (NoSuchMethodException e)
            // Java: IOLog.d(TAG, "NoSuchMethodException: ", e);
            // Java: Constructor<? extends POIXMLDocumentPart> constructor = cls.getDeclaredConstructor(PackagePart.class, PackageRelationship.class);
            // Java: return (POIXMLDocumentPart)constructor.newInstance(part, rel);
            // rust: ★M-STAGE1 P1-e1 —— 此前这里 `let _ = (parent, part, rel, cls);` 把 `part` **丢弃**、
            //   并给 `new_with_part` 传 `None`。后果是一条**静默假绿链**：
            //     子 part 的 `packagePart` 为 None ⇒ 其 `parse()` 首行
            //     `if let Some(ref part_box) = self.base.getPackagePart()` 不成立
            //     ⇒ 静默 `return Ok(())`，从不真正调 `XmlReader::parse`。
            //   Java 的 `newInstance(parent, part, rel)` 会把 `read()` 循环里那个**真实 part**
            //   交给子 part 构造器（`POIXMLDocumentPart(parent, part, rel)` ⇒ `this.packagePart = part`）。
            //   ⇒ 现按同一语义：带上源 part 的 zip 载体，构造一个 owned 承载交给 `new_with_part`。
            //     ★**不能**用 `OPCPackage::createPart` —— 那是**写路径**，会撞
            //     `throwExceptionIfReadOnly`（实测：解析 READ 模式的包时 panic
            //     "Operation not allowed, document open in read only mode!"，l1/l2 用例即刻红）。
            //     读路径的正确构造器是 `ZipPackagePart::new_with_entry`（与
            //     `OPCPackage::part_from_entry` 同一承载约定，无写守卫）。
            let part_name = part.getPartName().clone();
            let content_type = part.getContentType().toString();
            let container = part.getPackage().as_ref().map(|p| *p);
            // rust: 同 `OPCPackage::part_from_entry` —— 构造 `ZipPackagePart` 后把 `zipEntry`
            //       落入基类承载位并直接取其 `base`（`PackagePart`），与 Java 的多态值等价。
            let mut zip_part = crate::io::opc::ZipPackagePart::ZipPackagePart::new_with_entry(
                container,
                part.zipEntry.clone(),
                part_name.clone(),
                &content_type,
            );
            zip_part.base.zipEntry = zip_part.zipEntry.take();
            zip_part.base.zipArchive = part.zipArchive;
            let owned_part = Some(zip_part.base);
            let _ = (parent, cls);
            Some(POIXMLDocumentPart::new_with_part(
                owned_part.map(Box::new),
                Some(Box::new(PackageRelationship {
                    id: rel.getId().to_string(),
                    container: None,
                    relationshipType: rel.getRelationshipType().to_string(),
                    source: None,
                    sourceUri: Some(rel.getTargetURI().clone()),
                    targetMode: rel.getTargetMode(),
                    targetUri: rel.getTargetURI(),
                })),
            ))
        }
        // Java: catch (Exception e)
        // Java: throw new POIXMLException(e);
    }

    // Java: static Class<? extends POIXMLDocumentPart> getPOIXMLDocumentPartClass(String relationType)
    fn getPOIXMLDocumentPartClass(relation_type: &str) -> Option<String> {
        // Java: POIXMLRelation descriptor = XWPFRelation.getInstance(relationType);
        let mut descriptor_cls = {
            let descriptor = XWPFRelation::getInstance(relation_type);
            // Java: if (isDescriptorClsNull(descriptor) || descriptor._cls == XWPFNotImplemented.class)
            if Self::isDescriptorClsNull_poi(&descriptor.base)
                || descriptor.base.getRelationClass() == Some("XWPFNotImplemented")
            {
                None
            } else {
                descriptor.base.getRelationClass().map(|s| s.to_string())
            }
        };
        if descriptor_cls.is_none() {
            // Java: descriptor = XWPFRelationPurl.getInstance(relationType);
            let descriptor = XWPFRelationPurl::getInstance(relation_type);
            // Java: if (isDescriptorClsNull(descriptor))
            if Self::isDescriptorClsNull_poi(&descriptor.base) {
                // Java: return null;
                return None;
            }
            descriptor_cls = descriptor.base.getRelationClass().map(|s| s.to_string());
        }
        // Java: return descriptor._cls;
        descriptor_cls
    }

    // Java: static boolean isDescriptorClsNull(POIXMLRelation descriptor)
    fn isDescriptorClsNull(_descriptor: &Option<Box<dyn std::any::Any + Send + Sync>>) -> bool {
        // Java: return descriptor == null || descriptor._cls == null;
        true
    }

    fn isDescriptorClsNull_poi(descriptor: &POIXMLRelation) -> bool {
        descriptor.getRelationClass().is_none()
    }

    // Java: POIXMLDocumentPart newDocumentPart(POIXMLRelation descriptor)
    pub fn newDocumentPart(&self, descriptor: &POIXMLRelation) -> Option<POIXMLDocumentPart> {
        // Java: try
        // Java: Class<? extends POIXMLDocumentPart> cls = descriptor.getRelationClass();
        let cls = descriptor.getRelationClass();
        // Java: Constructor<? extends POIXMLDocumentPart> constructor = cls.getDeclaredConstructor();
        // Java: return (POIXMLDocumentPart)constructor.newInstance();
        // Java: catch (Exception e)
        // Java: throw new POIXMLException(e);
        if cls.is_none() {
            panic!("POIXMLException");
        }
        Some(POIXMLDocumentPart::new())
    }
}

impl POIXMLFactory for XWPFFactory {
    fn createDocumentPart(
        &self,
        var1: &POIXMLDocumentPart,
        var2: &PackageRelationship,
        var3: &PackagePart,
    ) -> Option<POIXMLDocumentPart> {
        XWPFFactory::createDocumentPart(self, var1, var2, var3)
    }

    fn newDocumentPart(&self, var1: &dyn std::any::Any) -> Option<POIXMLDocumentPart> {
        if let Some(d) = var1.downcast_ref::<POIXMLRelation>() {
            return XWPFFactory::newDocumentPart(self, d);
        }
        if let Some(d) = var1.downcast_ref::<XWPFRelation>() {
            return XWPFFactory::newDocumentPart(self, &d.base);
        }
        None
    }
}
