#![allow(non_snake_case)]

use std::cell::{Cell, Ref, RefCell};
use std::io::{self, Read};
use std::rc::Rc;

use crate::io::opc::PackagePart::PackagePart;
use crate::io::opc::PackageRelationship::PackageRelationship;
use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use crate::io::opc::model::POIXMLException::POIXMLException;
use crate::io::xwpf::PictureData::PictureData;
use crate::io::xwpf::util::ZipEntrySource::{ZipEntryRef, ZipEntrySource};
use core_libs_util::cn::wps::util::IOUtils::IOUtils;

// Java: public class XWPFPictureData extends POIXMLDocumentPart implements PictureData
pub struct XWPFPictureData {
    // Java: inherited POIXMLDocumentPart. Rc is the ownership adapter for the
    // factory's existing relation entry; it keeps one shared PackagePart.
    base: Rc<RefCell<POIXMLDocumentPart>>,
    // Java: private Long checksum = null;
    checksum: Cell<Option<i64>>,
    // Java: private PictureInfo mPicInfo; and InputStream mPictureStream;
    // PictureParser is not yet on the Rust compile surface. No width/height
    // result is synthesized here.
}

// Java: public static class XWPFPictureData.PictureDataSource
// The Ref keeps the same POIXMLDocumentPart/PackagePart borrowed while its
// Java entry and source references are used by the delayed picture dumper.
pub struct PictureDataSource<'a> {
    filename: String,
    part: Ref<'a, POIXMLDocumentPart>,
}

impl PictureDataSource<'_> {
    // Java: public String getFilename()
    pub fn getFilename(&self) -> &str { &self.filename }

    // Java: public ZipEntry getEntry()
    pub fn getEntry(&self) -> &ZipEntryRef {
        self.part.getPackagePart().as_ref().expect("picture package part")
            .zip_entry().expect("picture zip entry")
    }

    // Java: public ZipEntrySource getSource()
    pub fn getSource(&self) -> &dyn ZipEntrySource {
        self.part.getPackagePart().as_ref().expect("picture package part")
            .zip_entry_source().expect("picture zip source")
    }
}

impl PictureData for XWPFPictureData {
    // Java: public byte[] getData()
    fn getData(&self) -> Vec<u8> { XWPFPictureData::getData(self) }

    // Java: public String suggestFileExtension()
    fn suggestFileExtension(&self) -> String { XWPFPictureData::suggestFileExtension(self) }

    // Java: public String getUniqueFileName()
    fn getUniqueFileName(&self) -> String { XWPFPictureData::getUniqueFileName(self) }

    // Java: public PictureDataSource getDataSource()
    fn getDataSource(&self) -> Option<PictureDataSource<'_>> { XWPFPictureData::getDataSource(self) }
}

impl XWPFPictureData {
    // Java: public XWPFPictureData() { super(); }
    pub fn new() -> Self {
        Self { base: Rc::new(RefCell::new(POIXMLDocumentPart::new())), checksum: Cell::new(None) }
    }

    // Java: public XWPFPictureData(POIXMLDocumentPart parent, PackagePart part, PackageRelationship rel)
    pub fn new_with_parts(parent: Option<&POIXMLDocumentPart>, part: Option<Box<PackagePart>>, rel: Option<Box<PackageRelationship>>) -> Self {
        // Java: super(parent, part, rel); this.mPictureStream = null;
        Self { base: Rc::new(RefCell::new(POIXMLDocumentPart::POIXMLDocumentPart__parent(parent, part, rel))), checksum: Cell::new(None) }
    }

    // Rust ownership adapter: XWPFFactory currently stores relation subclasses
    // as POIXMLDocumentPart entries. Preserve that same entry when downcasting
    // the image relation instead of cloning or discarding its PackagePart.
    pub fn from_relation(part: Rc<RefCell<POIXMLDocumentPart>>) -> Self {
        Self { base: part, checksum: Cell::new(None) }
    }

    pub fn document_part(&self) -> Rc<RefCell<POIXMLDocumentPart>> { self.base.clone() }

    fn read_data(&self) -> io::Result<Vec<u8>> {
        let base = self.base.borrow();
        let part = base.getPackagePart().as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "picture package part is missing"))?;
        let mut stream = part.getInputStream_default()?;
        let mut data = Vec::new();
        stream.read_to_end(&mut data)?;
        Ok(data)
    }

    // Java: public byte[] getData()
    pub fn getData(&self) -> Vec<u8> {
        // Java: return IOUtils.toByteArray(this.getPackagePart().getInputStream());
        // Java: catch (IOException e) { throw new POIXMLException(e); }
        self.read_data().unwrap_or_else(|error| std::panic::panic_any(POIXMLException::new_with_cause(Box::new(error))))
    }

    // Java: public String getFileName()
    pub fn getFileName(&self) -> Option<String> {
        // Java: String name = this.getPackagePart().getPartName().getName();
        let base = self.base.borrow();
        let name = base.getPackagePart().as_ref()?.getPartName().getName();
        // Java: return name == null ? null : name.substring(name.lastIndexOf(47) + 1);
        Some(name.rsplit('/').next().unwrap_or("").to_string())
    }

    // Java: public String suggestFileExtension()
    pub fn suggestFileExtension(&self) -> String {
        // Java: return this.getPackagePart().getPartName().getExtension();
        self.base.borrow().getPackagePart().as_ref().expect("picture package part").getPartName().getExtension()
    }

    // Java: public Long getChecksum()
    pub fn getChecksum(&self) -> i64 {
        // Java: if (this.checksum == null) read the part stream and calculateChecksum(data).
        if let Some(checksum) = self.checksum.get() { return checksum; }
        let checksum = IOUtils::calculateChecksum(&self.getData());
        self.checksum.set(Some(checksum));
        checksum
    }

    // Java: public String getUniqueFileName()
    pub fn getUniqueFileName(&self) -> String {
        // Java: return UUID.randomUUID().toString() + "." + this.suggestFileExtension();
        format!("{}.{}", uuid::Uuid::new_v4(), self.suggestFileExtension())
    }

    // Java: public PictureDataSource getDataSource()
    pub fn getDataSource(&self) -> Option<PictureDataSource<'_>> {
        // Java: PackagePart part = this.getPackagePart();
        let part = self.base.borrow();
        let package_part = part.getPackagePart().as_ref().expect("part should not be null!");
        // Java: if (!(part instanceof ZipPackagePart)) return null;
        // OPCPackage's Rust partList stores the ZipPackagePart base value;
        // its zipEntry is present only for that Java subclass.
        package_part.zip_entry()?;
        // Java: return new PictureDataSource(this.getUniqueFileName(),
        // Java:     zipPart.getZipArchive(), zipPackage.getZipArchive());
        package_part.zip_entry_source().expect("source should not be null!");
        Some(PictureDataSource { filename: self.getUniqueFileName(), part })
    }
}
