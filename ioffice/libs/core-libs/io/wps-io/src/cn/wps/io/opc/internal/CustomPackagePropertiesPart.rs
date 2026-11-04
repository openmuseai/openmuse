#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.opc.CustomPackageProperties
// Java: import cn.wps.io.opc.OPCPackage
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackagePartName
// Java: import cn.wps.io.opc.ZipPackagePart
// Java: import cn.wps.io.opc.internal.marshallers.ZipPartMarshaller
// Java: import cn.wps.io.xwpf.XWPFPackage
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.util.List

use crate::io::xwpf::util::ZipEntrySource::ZipEntryRef;
use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackagePartName::PackagePartName;
use crate::io::opc::OPCPackage::OPCPackage;
use crate::io::opc::internal::ContentType::ContentType;

// Java field: String NAMESPACE_VT_URI
pub const NAMESPACE_VT_URI: &str = "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes";
// Java field: String NAMESPACE_CUSTOM_URI
pub const NAMESPACE_CUSTOM_URI: &str = "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties";

// Java: public class CustomPackagePropertiesPart extends PackagePart implements CustomPackageProperties
pub struct CustomPackagePropertiesPart {
    pub(crate) base: PackagePart,
    // Java field: List<CustomPackageProperties.Property> mPerpertyList
    pub(crate) mPerpertyList: Option<Vec<Box<dyn std::any::Any + Send + Sync>>>,
    // Java field: ZipEntry mZipEntry
    //   ★2026-09-20 修正：由 `Option<Box<dyn std::any::Any + Send + Sync>>` 改为 Java 声明的
    //   **真实类型** `ZipEntry` 的 Rust 等价 `ZipEntryRef`（封闭枚举）
    pub(crate) mZipEntry: Option<ZipEntryRef>,
}

// Java: public class CustomPackagePropertiesPart extends PackagePart implements CustomPackageProperties
impl CustomPackagePropertiesPart {
    // Java: CustomPackagePropertiesPart(OPCPackage pack, PackagePartName partName)
    pub fn CustomPackagePropertiesPart(pack: Option<*const OPCPackage>, part_name: PackagePartName) -> Self {
        // Java: super(pack, partName, "application/vnd.openxmlformats-officedocument.custom-properties+xml");
        CustomPackagePropertiesPart {
            base: PackagePart::new(pack, part_name, ContentType::new("application/vnd.openxmlformats-officedocument.custom-properties+xml").unwrap()),
            mPerpertyList: None,
            mZipEntry: None,
        }
    }

    // rust: no-java（Rust 惯用别名，crate 内沿用 `new`）
    pub fn new(pack: Option<*const OPCPackage>, part_name: PackagePartName) -> Self {
        Self::CustomPackagePropertiesPart(pack, part_name)
    }

    // Java: CustomPackagePropertiesPart(OPCPackage pack, PackagePartName partName, ContentType contentType)
    pub fn new_with_content_type(pack: Option<*const OPCPackage>, part_name: PackagePartName, content_type: ContentType) -> Self {
        // Java: super(pack, partName, contentType);
        CustomPackagePropertiesPart {
            base: PackagePart::new(pack, part_name, content_type),
            mPerpertyList: None,
            mZipEntry: None,
        }
    }

    // Java: InputStream getInputStreamImpl(boolean isOriginInput)
    pub fn getInputStreamImpl(&self, _is_origin_input: bool) -> Box<dyn std::io::Read> {
        // Java: if (this.mZipEntry == null)
        // Java: PackagePart packagePart = this._container.getPart(this._partName);
        // Java: if (packagePart != null && packagePart instanceof ZipPackagePart)
        // Java: ZipPackagePart zipPackagePart = (ZipPackagePart)packagePart;
        // rust: 模板邻接锚点 —— 该 method 属 `s01-04` 面停车（§1.7 明确「本 function 不修它」），
        //       故只保留 Java 注释 + 签名 + 局部变量名，不补实现。
        let zipPackagePart: Option<&dyn std::any::Any> = None;
        let _ = zipPackagePart;
        // Java: this.mZipEntry = zipPackagePart.getZipArchive();
        // Java: return ((XWPFPackage)this._container).getZipArchive().getInputStream(this.mZipEntry, isOriginInput);
        todo!()
    }

    // Java: OutputStream getOutputStreamImpl()
    pub fn getOutputStreamImpl(&self) -> Option<Box<dyn std::io::Write>> {
        // Java: return null;
        None
    }

    // Java: boolean save(OutputStream zos)
    pub fn save(&self, _zos: &mut dyn std::io::Write) -> bool {
        // Java: return (new ZipPartMarshaller()).marshall(this, zos);
        todo!()
    }

    // Java: boolean load(InputStream ios)
    pub fn load(&self, _ios: &mut dyn std::io::Read) -> bool {
        // Java: return false;
        false
    }

    // Java: void close()
    pub fn close(&self) {
    }

    // Java: void flush()
    pub fn flush(&self) {
    }

    // Java: void setPropertyArray(List<CustomPackageProperties.Property> perpertyList)
    pub fn setPropertyArray(&mut self, perperty_list: Vec<Box<dyn std::any::Any + Send + Sync>>) {
        // Java: this.mPerpertyList = perpertyList;
        self.mPerpertyList = Some(perperty_list);
    }

    // Java: List<CustomPackageProperties.Property> getPropertyArray()
    pub fn getPropertyArray(&self) -> &Option<Vec<Box<dyn std::any::Any + Send + Sync>>> {
        // Java: return this.mPerpertyList;
        &self.mPerpertyList
    }

    // Java: boolean hasProperty()
    pub fn hasProperty(&self) -> bool {
        // Java: return null != this.mPerpertyList && this.mPerpertyList.size() > 0;
        self.mPerpertyList.as_ref().map_or(false, |list| !list.is_empty())
    }
}
