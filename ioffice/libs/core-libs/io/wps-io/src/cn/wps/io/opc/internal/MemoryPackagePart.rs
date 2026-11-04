#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.opc.OPCPackage
// Java: import cn.wps.io.opc.PackagePart
// Java: import cn.wps.io.opc.PackagePartName
// Java: import cn.wps.io.opc.internal.marshallers.ZipPartMarshaller
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Java: import cn.wps.io.xwpf.usermodel.exceptions.OpenXML4JException
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import cn.wps.moffice.kfs.zip.ZipOutputStream
// Java: import java.io.ByteArrayInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream

use std::io::Cursor;
use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackagePartName::PackagePartName;
use crate::io::opc::OPCPackage::OPCPackage;
use crate::io::opc::internal::ContentType::ContentType;
use crate::io::opc::internal::marshallers::ZipPartMarshaller::ZipPartMarshaller;

// Java field: String TAG
pub(crate) const TAG: Option<&str> = None;

// Java: public class MemoryPackagePart extends PackagePart
pub struct MemoryPackagePart {
    pub(crate) base: PackagePart,
    // Java field: byte[] data
    pub(crate) data: Option<Vec<u8>>,
    // Java field: int length
    pub(crate) length: i32,
    // Java field: boolean mEntryCreated
    mEntryCreated: bool,
}

// Java: public class MemoryPackagePart extends PackagePart
impl MemoryPackagePart {
    // Java: MemoryPackagePart(OPCPackage pack, PackagePartName partName, String contentType)
    pub fn new(pack: Option<*const OPCPackage>, part_name: PackagePartName, content_type: &str) -> Self {
        // Java: super(pack, partName, contentType);
        MemoryPackagePart {
            base: PackagePart::new(pack, part_name, ContentType::new(content_type).unwrap()),
            data: None,
            length: 0,
            mEntryCreated: false,
        }
    }

    // Java: MemoryPackagePart(OPCPackage pack, PackagePartName partName, String contentType, boolean loadRelationships)
    pub fn new_with_load(pack: Option<*const OPCPackage>, part_name: PackagePartName, content_type: &str, _load_relationships: bool) -> Self {
        // Java: super(pack, partName, new ContentType(contentType), loadRelationships);
        MemoryPackagePart {
            base: PackagePart::new(pack, part_name, ContentType::new(content_type).unwrap()),
            data: None,
            length: 0,
            mEntryCreated: false,
        }
    }

    // Java: InputStream getInputStreamImpl(boolean isOriginInput)
    pub fn getInputStreamImpl(&mut self, _is_origin_input: bool) -> Cursor<Vec<u8>> {
        // Java: if (this.data == null)
        if self.data.is_none() {
            // Java: this.data = new byte[0];
            self.data = Some(Vec::new());
        }
        // Java: return new ByteArrayInputStream(this.data);
        Cursor::new(self.data.clone().unwrap_or_default())
    }

    // Java: OutputStream getOutputStreamImpl()
    pub fn getOutputStreamImpl(&mut self) -> Box<dyn std::io::Write> {
        // Java: ZipOutputStream zos = this._container.getZipOutputStream();
        // Java: IOAssert.assertNotNull("zos should not be null.", zos);
        // Java: ZipEntry partEntry = new ZipEntry(ZipHelper.getZipItemNameFromOPCName(this.getPartName().getURI().getPath()));
        // Java: this.mEntryCreated = true;
        self.mEntryCreated = true;
        // Java: try
        // Java: zos.putNextEntry(partEntry);
        // Java: catch (IOException e)
        // Java: IOLog.e(TAG, "IOException", e);
        // Java: IOAssert.fail("It should not reach to here.");
        // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // Java: throw new RuntimeException(e);
        // Java: return zos;
        todo!()
    }

    // Java: OutputStream getOutputStreamImpl(ZipEntry entry, ZipOutputStream.IListener listener, ZipOutputStream.DeflateDataCallBack deflateDataCallBack)
    pub fn getOutputStreamImpl_with_entry(&mut self, _entry: Option<Box<dyn std::any::Any + Send + Sync>>, _listener: Option<Box<dyn std::any::Any + Send + Sync>>, _deflate_data_callback: Option<Box<dyn std::any::Any + Send + Sync>>) -> Box<dyn std::io::Write> {
        // Java: ZipOutputStream zos = this._container.getZipOutputStream();
        // Java: IOAssert.assertNotNull("zos should not be null.", zos);
        // Java: ZipEntry tmpEntry = null != entry ? (ZipEntry)entry.clone() : null;
        // Java: if (null == tmpEntry)
        // Java: tmpEntry = new ZipEntry();
        // Java: this.mEntryCreated = true;
        self.mEntryCreated = true;
        // Java: tmpEntry.setName(ZipHelper.getZipItemNameFromOPCName(this.getPartName().getURI().getPath()));
        // Java: try
        // Java: zos.putNextEntry(tmpEntry, listener, deflateDataCallBack);
        // Java: catch (IOException e)
        // Java: IOLog.e(TAG, "IOException", e);
        // Java: IOAssert.fail("It should not reach to here.");
        // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // Java: throw new RuntimeException(e);
        // Java: return zos;
        todo!()
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.data = null;
        self.data = None;
        // Java: this.length = 0;
        self.length = 0;
    }

    // Java: boolean save(OutputStream os)
    pub fn save(&self, _os: &mut dyn std::io::Write) -> bool {
        // Java: return (new ZipPartMarshaller()).marshall(this, os);
        todo!()
    }

    // Java: boolean load(InputStream ios)
    pub fn load(&self, _ios: &mut dyn std::io::Read) -> bool {
        // Java: throw new InvalidFormatException("Method not implemented");
        panic!("Method not implemented")
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: if (this.mEntryCreated)
        if self.mEntryCreated {
            // Java: ZipOutputStream zos = this._container.getZipOutputStream();
            // Java: IOAssert.assertNotNull("zos should not be null.", zos);
            // Java: try
            // Java: zos.closeEntry();
            // Java: catch (IOException e)
            // Java: IOLog.e(TAG, "IOException", e);
            // Java: IOAssert.fail("It should not reach to here.");
            // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
            // Java: throw new RuntimeException(e);
            // Java: this.mEntryCreated = false;
            self.mEntryCreated = false;
            todo!()
        }
    }

    // Java: void flush()
    pub fn flush(&self) {
    }
}
