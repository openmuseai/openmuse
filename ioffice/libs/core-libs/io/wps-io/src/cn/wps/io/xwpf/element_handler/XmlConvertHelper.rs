#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.dom.io.WMap
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackageRelationshipCollection
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import org.xml.sax.Attributes

// Java: public class XmlConvertHelper
/// Rust ownership adapter for Java's shared `POIXMLDocumentPart` reference.
/// Created only for one synchronous `XWPFDocument::parse_part_xml` call; the
/// handler tree is dropped before that call returns and the document can move.
pub struct BorrowedDocumentPart {
    part: *const crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart,
}

impl BorrowedDocumentPart {
    pub(crate) fn new(part: &crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart) -> Self {
        Self { part: part as *const _ }
    }

    /// # Safety
    /// The originating `XWPFDocument::parse_part_xml` call must still be active.
    pub unsafe fn get(&self) -> &crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart {
        unsafe { &*self.part }
    }
}

// Java: public class XmlConvertHelper
pub struct XmlConvertHelper {
    // Java field: String TAG
    _private: (),
}

// Java: public class XmlConvertHelper
impl XmlConvertHelper {
    // Java: XmlConvertHelper()
    pub fn new() -> Self {
        // Java: super();
        XmlConvertHelper { _private: () }
    }

    // Java: static Boolean getBoolean(Attributes attributes, String name)
    pub fn getBoolean(attributes: &dyn std::any::Any, name: &str) -> Option<bool> {
        // Java: IOAssert.assertTrue(!name.contains(":"));
        debug_assert!(!name.contains(':'));
        // Java: String str = attributes.getValue(WMap.MAIN_NAMESPACING, name);
        let str_opt = if let Some(attrs) = attributes.downcast_ref::<crate::io::opt::AttributesImpl::AttributesImpl>() {
            attrs.getValue_by_uri(crate::io::dom::io::WMap::WMap::MAIN_NAMESPACING(), name)
        } else {
            None
        };
        // Java: return null == str ? null : toBoolean(str);
        match str_opt {
            None => None,
            Some(s) => Self::toBoolean(&s),
        }
    }

    // Java: static Boolean toBoolean(String str)
    fn toBoolean(s: &str) -> Option<bool> {
        // Java: if (str != null && str.length() > 0)
        // Java: return !"off".equalsIgnoreCase(str) && !"0".equals(str) && !"false".equalsIgnoreCase(str) && !"f".equalsIgnoreCase(str) ? Boolean.TRUE : Boolean.FALSE;
        // Java: else
        // Java: return null;
        if !s.is_empty() {
            let lower = s.to_lowercase();
            if lower == "off" || s == "0" || lower == "false" || lower == "f" {
                Some(false)
            } else {
                Some(true)
            }
        } else {
            None
        }
    }

    // Java: static PackageRelationshipCollection getRelationshipCollection(POIXMLDocumentPart part)
    pub fn getRelationshipCollection(part: &dyn std::any::Any) -> Option<crate::io::opc::PackageRelationshipCollection::PackageRelationshipCollection> {
        // Java: IOAssert.assertNotNull("part should not be null!", part);
        // Java: if (null == part)
        {
            // Java: return null;
        }
        // Java: else
        {
            // Java: PackagePart packagePart = part.getPackagePart();
            let doc_part = if let Some(borrowed) = part.downcast_ref::<BorrowedDocumentPart>() {
                // SAFETY: the handler is consumed by the synchronous SAX parse that created it.
                Some(unsafe { borrowed.get() })
            } else {
                part.downcast_ref::<crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart>()
            }?;
            // Java: IOAssert.assertNotNull("packagePart should not be null!", packagePart);
            // Java: if (null == packagePart)
            let Some(ref part_box) = *doc_part.getPackagePart() else {
                // Java: return null;
                return None;
            };
            // Java: else
            {
                // Java: try
                {
                    // Java: return packagePart.getRelationships();
                    // XWPFDocument loaded the collection before synchronous SAX parse.
                    part_box.getRelationshipsCached()
                }
                // Java: catch (InvalidFormatException e)
                // unreachable with empty collection glue
                // Java: IOLog.e(TAG, "InvalidFormatException: ", e);
                // Java: IOAssert.fail("It should not reach here!");
                // Java: return null;
            }
        }
    }
}
