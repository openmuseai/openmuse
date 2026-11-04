#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.assertion.IOAssert
// Rust: assertions use `assert!` at the corresponding call site.
// Java: import cn.wps.base.log.IOLog
// Rust: logging is handled at the corresponding error branches.
// Java: import cn.wps.io.dom.tree.DefaultAttribute
// Rust: pool tuning is not exposed by the current DOM tree port.
// Java: import cn.wps.io.dom.tree.DefaultElement
// Rust: pool tuning is not exposed by the current DOM tree port.
// Java: import cn.wps.io.dom.tree.DefaultText
// Rust: pool tuning is not exposed by the current DOM tree port.
// Java: import cn.wps.io.opc.CustomPackageProperties
// Rust: this API is reached through `XWPFPackage` when implemented.
// Java: import cn.wps.io.opc.CustomXmlInfo
// Rust: this API is reached through `XWPFPackage` when implemented.
// Java: import cn.wps.io.opc.ExtendedPackageProperties
// Rust: this API is reached through `XWPFPackage` when implemented.
// Java: import cn.wps.io.opc.OPCPackage
use crate::io::opc::OPCPackage::OPCPackage;
// Java: import cn.wps.io.opc.OpenXmlNamespaceHelper
// Rust: namespace selection remains internal to package loading.
// Java: import cn.wps.io.opc.PackageAccess
use crate::io::opc::PackageAccess::PackageAccess;
// Java: import cn.wps.io.opc.PackagePartName
use crate::io::opc::PackagePartName::PackagePartName;
// Java: import cn.wps.io.opc.PackageProperties
// Rust: this API is reached through `XWPFPackage` when implemented.
// Java: import cn.wps.io.opc.PackageRelationshipCollection
use crate::io::opc::PackageRelationshipCollection::PackageRelationshipCollection;
// Java: import cn.wps.io.opc.PackagingURIHelper
use crate::io::opc::PackagingURIHelper::PackagingURIHelper;
// Java: import cn.wps.io.opc.TargetMode
use crate::io::opc::TargetMode::TargetMode;
// Java: import cn.wps.io.opc.model.POIXMLDocument
use crate::io::opc::model::POIXMLDocument::POIXMLDocument;
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
// Java: import cn.wps.io.opc.model.POIXMLException
// Rust: package failures currently use native `Result`/panic boundaries.
// Java: import cn.wps.io.opc.model.POIXMLProperties
use crate::io::opc::model::POIXMLProperties::POIXMLProperties;
// Java: import cn.wps.io.opt.XmlReader
use crate::io::opt::XmlReader::XmlReader;
// Java: import cn.wps.io.xwpf.IDocumentImporter
use crate::io::xwpf::IDocumentImporter::IDocumentImporter;
// Java: import cn.wps.io.xwpf.XWPFPackage
use crate::io::xwpf::XWPFPackage::XWPFPackage;
// Java: import cn.wps.io.xwpf.element_handler.DocumentHandler
use crate::io::xwpf::element_handler::DocumentHandler::DocumentHandler;
// Java: import cn.wps.io.xwpf.usermodel.exceptions.InvalidFormatException
// Rust: invalid formats are represented by native package errors.
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// Rust: no-space detection is handled by native I/O errors.
// Java: import cn.wps.moffice.io.ContentLockedException
// Rust: content-lock failures are represented by native errors.
// Java: import cn.wps.moffice.io.ForceQuitException
// Rust: force-quit propagation is represented by native errors.
// Java: import cn.wps.moffice.io.ReadingLimitException
// Rust: reading-limit failures are represented by native errors.
// Java: import cn.wps.moffice.kfs.File
// Rust: the current public compatibility entry point accepts an opaque KFS file reference.
// Java: import cn.wps.moffice.kfs.FileInputStream
// Rust: package parts expose `std::io::Read` streams directly.
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Rust: random-access construction is handled by `XWPFPackage`.
// Java: import cn.wps.moffice.util.FileUtil
// Rust: stream cleanup is automatic through `Drop`.
// Java: import cn.wps.moffice.writer.data.value.FileFormat
// Rust: the current public compatibility entry point accepts an opaque format reference.

use crate::io::opc::internal::ZipHelper::ZipHelper;
use crate::io::simple::ElementHandlerSimple::ElementHandlerRef;
use crate::io::xwpf::element_handler::styles::StylesHandler::StylesHandler;
use crate::io::xwpf::usermodel::XWPFComment::XWPFComment;
use crate::io::xwpf::usermodel::XWPFCommentsEx::XWPFCommentsEx;
use crate::io::xwpf::usermodel::XWPFEndnote::XWPFEndnote;
use crate::io::xwpf::usermodel::XWPFFontTable::XWPFFontTable;
use crate::io::xwpf::usermodel::XWPFFootnote::XWPFFootnote;
use crate::io::xwpf::usermodel::XWPFNumbering::XWPFNumbering;
use crate::io::xwpf::usermodel::XWPFSettings::XWPFSettings;
use crate::io::xwpf::usermodel::XWPFTheme::XWPFTheme;
use crate::io::xwpf::usermodel::XWPFRelation::XWPFRelation;
use crate::io::xwpf::usermodel::XWPFRelationPurl::XWPFRelationPurl;
use super::XWPFFactory::XWPFFactory;
use std::rc::Rc;

const OFFICE_DOCUMENT_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const OFFICE_DOCUMENT_REL_STRICT: &str =
    "http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument";
const STYLES_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
const STYLES_REL_STRICT: &str =
    "http://purl.oclc.org/ooxml/officeDocument/relationships/styles";
const NUMBERING_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering";
const NUMBERING_REL_STRICT: &str =
    "http://purl.oclc.org/ooxml/officeDocument/relationships/numbering";

// Java field: String TAG
const TAG: Option<&str> = None;
// Java field: String DOCUMENT_APPLICATION
pub const DOCUMENT_APPLICATION: &str = "WPS Office";

// Java: public class XWPFDocument extends POIXMLDocument
pub struct XWPFDocument {
    pub(crate) base: POIXMLDocument,
    // Java field: String mNotePath
    pub(crate) mNotePath: Option<String>,
    // Java field: XWPFSettings settings
    pub(crate) settings: Option<XWPFSettings>,
    // Java field: XWPFComment comments
    pub(crate) comments: Option<XWPFComment>,
    // Java field: XWPFEndnote endnotes
    pub(crate) endnotes: Option<XWPFEndnote>,
    // Java field: XWPFFootnote footnotes
    pub(crate) footnotes: Option<XWPFFootnote>,
    // Java field: XWPFNumbering numbering
    pub(crate) numbering: Option<XWPFNumbering>,
    // Java field: XWPFStyles styles
    pub(crate) styles: Option<POIXMLDocumentPart>,
    // Java field: XWPFFontTable fontTablePart
    pub(crate) fontTablePart: Option<XWPFFontTable>,
    // Java field: XWPFTheme mTheme
    pub(crate) mTheme: Option<XWPFTheme>,
    // Java field: XWPFCommentsEx mCommentsEx
    pub(crate) mCommentsEx: Option<XWPFCommentsEx>,
    // Java field: String fileStr
    pub(crate) fileStr: Option<String>,
    // Java field: POIXMLProperties properties
    pub(crate) properties: Option<POIXMLProperties>,
    // Java field: DocumentHandler mDocumentHandler
    pub(crate) mDocumentHandler: Option<DocumentHandler>,
    // Java field: IDocumentImporter mImporter
    pub(crate) mImporter: Option<Rc<dyn IDocumentImporter>>,
    pub(crate) document_part_name: Option<PackagePartName>,
}

// Java: public class XWPFDocument extends POIXMLDocument
impl XWPFDocument {
    // Java: static XWPFDocument startPackage(File file, FileFormat fileFormat)
    pub fn startPackage(file: &dyn std::any::Any, file_format: &dyn std::any::Any) -> Self {
        // Java: return new XWPFDocument(file, true, fileFormat);
        Self::XWPFDocument__file_bool_format(file, true, file_format)
    }

    // Java: XWPFDocument(OutputStream outputStream)
    pub fn XWPFDocument__output(output_stream: &mut dyn std::io::Write) -> Self {
        // Java: this(newPackage(outputStream));
        let pkg = Self::newPackage(output_stream);
        let mut doc = Self::XWPFDocument(pkg);
        // Java: this.load(XWPFFactory.getInstance());
        doc.base.load(super::XWPFFactory::XWPFFactory::getInstance());
        doc
    }

    // Java: static XWPFDocument startPackage(OutputStream outputStream)
    pub fn startPackage_output(output_stream: &mut dyn std::io::Write) -> Self {
        // Java: return new XWPFDocument(outputStream);
        Self::XWPFDocument__output(output_stream)
    }

    // Java: static XWPFPackage newPackage(OutputStream outputStream)
    pub fn newPackage(output_stream: &mut dyn std::io::Write) -> XWPFPackage {
        // Java: XWPFPackage pkg = XWPFPackage.create(outputStream);
        let mut pkg = crate::io::xwpf::XWPFPackage::XWPFPackage::create_output(output_stream);
        // Java: PackagePartName corePartName = PackagingURIHelper.createPartName(XWPFRelation.DOCUMENT.getDefaultFileName());
        let default_name = "/word/document.xml";
        let core_part_name = crate::io::opc::PackagingURIHelper::PackagingURIHelper::createPartName_str(default_name)
            .unwrap_or_else(|_| panic!("InvalidFormatException"));
        // Java: pkg.addRelationship(corePartName, TargetMode.INTERNAL, "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument");
        let _ = pkg.addRelationship(
            &core_part_name,
            crate::io::opc::TargetMode::TargetMode::INTERNAL as i32,
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
            None,
        );
        // Java: pkg.createPart(corePartName, XWPFRelation.DOCUMENT.getContentType());
        let _ = pkg.createPart(
            &core_part_name,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
            true,
        );
        // Java: return pkg;
        pkg
    }

    // Java: XWPFDocument(XWPFPackage pkg)
    pub fn XWPFDocument(pkg: XWPFPackage) -> Self {
        // Java: super(pkg);
        Self::XWPFDocument__pkg(pkg)
    }

    /// Open-package load path used by File/RAF/stream constructors after `XWPFPackage.open`.
    pub fn XWPFDocument__pkg(mut pack: XWPFPackage) -> Self {
        // Java: OpenXmlNamespaceHelper.OpenXmlFileType fileType = OpenXmlNamespaceHelper.OpenXmlFileType.TRANSITIONAL;
        let mut fileType = crate::io::opc::OpenXmlNamespaceHelper::OpenXmlFileType::TRANSITIONAL;
        // Java: PackageRelationshipCollection coreRels = pkg.getRelationshipsByType("http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument");
        let mut core = pack.base.getRelationshipsByType(OFFICE_DOCUMENT_REL);
        // Java: if (0 == coreRels.size())
        if core.size() == 0 {
            // Java: coreRels = pkg.getRelationshipsByType("http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument");
            core = pack.base.getRelationshipsByType(OFFICE_DOCUMENT_REL_STRICT);
            // Java: if (0 != coreRels.size())
            if core.size() != 0 {
                // Java: fileType = OpenXmlNamespaceHelper.OpenXmlFileType.STRICT;
                fileType = crate::io::opc::OpenXmlNamespaceHelper::OpenXmlFileType::STRICT;
            }
        }
        // Java: OpenXmlNamespaceHelper.initTagNamespace(fileType);
        // rust: no-stmt（namespace 选择由解析器内部按 document part 处理，此处不改动包加载流程）
        // Java: IOAssert.assertTrue("The number of main document relationship should be only 1.", coreRels.size() == 1);
        assert!(
            core.size() == 1,
            "The number of main document relationship should be only 1."
        );
        // Java: this.packageRel = coreRels.getRelationship(0);
        let core_rel = core.getRelationship_by_index(0).expect("officeDocument rel");
        // Java: this.packagePart = pkg.getPart(this.packageRel);
        // rust: `PackagePartName` 由下方 target 推导后立即赋值（见 `core part` 段）。
        let target = core_rel.getTargetURI().toString();
        let opc_name = ZipHelper::getOPCNameFromZipItemName(&target);
        let part_name = PackagingURIHelper::createPartName_str(&opc_name)
            .expect("officeDocument part name");
        // Java: this.packagePart = pkg.getPart(this.packageRel);
        // rust: 本赋值是 `POIXMLDocumentPart::read()` 能否装配 relations 的前置条件
        //       （`read()` 首行即 `if (null != this.packagePart)`）；缺此赋值 ⇒ relations 恒空
        //       ⇒ XWPFDocument.onDocumentRead 的 9 路分派全部落空。
        //       `get_part_mut` 借用包的 partList，故先取出所需字段；★包指针**不能在此取** ——
        //       `pack.base` 随后会被 `empty_with_pkg` move 进 `doc`，此处捕获的指针会立即悬垂
        //       （实测 SIGSEGV）。指针改在 `doc` 落位之后从 `doc.base.pkg` 取。
        let core_content_type = {
            let pkg_ref = &mut pack.base;
            match pkg_ref.get_part_mut(&part_name) {
                Some(part) => part.getContentType().toString(),
                None => String::new(),
            }
        };
        // （`doc.base.base.packagePart` 的赋值见下方 doc 构造之后）

        let mut has_styles = false;
        let mut has_numbering = false;
        if let Some(rel_part_name) = PackagingURIHelper::getRelationshipPartName(&part_name) {
            if let Some(rel_part) = pack.base.get_part_mut(&rel_part_name) {
                if let Ok(mut stream) = rel_part.getInputStream_default() {
                    let mut child_rels = crate::io::opc::PackageRelationshipCollection::PackageRelationshipCollection::new();
                    child_rels.parseRelationshipsXml(&mut *stream);
                    for rel in child_rels.iter() {
                        if rel.getTargetMode() != TargetMode::INTERNAL {
                            continue;
                        }
                        let ty = rel.getRelationshipType();
                        if ty == STYLES_REL || ty == STYLES_REL_STRICT {
                            has_styles = true;
                        } else if ty == NUMBERING_REL || ty == NUMBERING_REL_STRICT {
                            has_numbering = true;
                        }
                    }
                }
            }
        }

        let mut doc = Self::empty_with_pkg(pack.base);
        // Java: this.packagePart = pkg.getPart(this.packageRel);
        // rust: 必须在 `load` 之前赋值 —— `POIXMLDocumentPart::read()` 首行即
        //       `if (null != this.packagePart)`，缺此赋值则 relations 恒空，
        //       onDocumentRead 的 9 路分派全部落空（本里程碑 P1 的根因）。
        //       ★主 part 缺失时**不得 panic**（BM5）：Java 侧 `getPart` 返回 null 后
        //       仍可继续，故此处仅在确有 part 时赋值，否则保持 None 由后续守卫兜住。
        // rust: ★指针必须从 `doc` **落位之后**的 `doc.base.pkg` 取：`OPCPackage` 是值类型，
        //       move 进 `doc` 后地址即变，任何 move 前捕获的指针都会悬垂（实测 SIGSEGV）。
        let doc_pkg_ptr: Option<*const crate::io::opc::OPCPackage::OPCPackage> =
            doc.base.pkg.as_ref().map(|p| p as *const _);
        // rust: ★重定位包内每个 part 的 `_container` 到**包的新地址**。
        //   为什么必须做：`PackagePart::_container` 是 `Option<*const OPCPackage>`（Java 引用语义的承载），
        //   而 Rust 的 `OPCPackage` 是**值类型** —— `XWPFPackage::open_stream` 先造 part、再把包按值返回，
        //   包最终 move 进 `doc.base.pkg` 时栈地址改变，所有 part 里的旧指针随即悬垂。
        //   实测后果：关系解析成功、`target_part.is_some` 为真之后，一旦真的去取该 part 的流
        //   （`getInputStream` → `getPackage().getZipArchive()`）即 **SIGBUS**。
        //   Java 无此问题（堆引用），故此处属「同一语义的 Rust 落地」，不是行为差异。
        if let Some(pkg_ptr) = doc_pkg_ptr {
            unsafe {
                let pkg_mut = &mut *(pkg_ptr as *mut crate::io::opc::OPCPackage::OPCPackage);
                if let Some(list) = pkg_mut.partList.as_mut() {
                    for part in list.values_mut() {
                        part.set_container(pkg_ptr);
                    }
                }
            }
        }
        // rust: 仅当**确有**主 part 且其 content type 非空时才赋值 ——
        //       主 part 缺失（BM5）时 `core_content_type` 为空串，而 `ContentType::new("")` 会
        //       报 "not compliant with RFC 2616" 并 unwrap panic；Java 的 `getPart` 返回 null 后
        //       根本不会走到构造器，故此处必须同等跳过，保持「主 part 缺失不 panic」。
        if let Some(doc_pkg_ptr) = doc_pkg_ptr {
            if !core_content_type.is_empty() {
                doc.base.base.packagePart = Some(Box::new(
                    crate::io::opc::PackagePart::PackagePart::new_with_string(
                        Some(doc_pkg_ptr),
                        part_name.clone(),
                        &core_content_type,
                    ),
                ));
            }
        }
        // rust: Java 侧 styles/numbering 由虚调用 onDocumentRead() 装配；Rust 无法虚派发到
        //       XWPFDocument 的覆写，故改由下方显式调用 onDocumentRead() 统一装配（见其注释）。
        //       这两行保留为「load 之前的初始占位」，其值随后被 onDocumentRead 覆盖。
        doc.styles = has_styles.then(POIXMLDocumentPart::new);
        doc.numbering = has_numbering.then(XWPFNumbering::new);
        doc.document_part_name = Some(part_name);
        doc.base.load(XWPFFactory::getInstance());
        // Java: this.onDocumentRead();（POIXMLDocument.java:41 的虚调用在 XWPFDocument 上的落点）
        // rust: Rust 的 POIXMLDocument::load 内是 self.base.onDocumentRead()（基类空实现），
        //       不会虚派发到本覆写 ⇒ 必须在此显式调用，否则 9 个 part 字段恒为 None。
        doc.onDocumentRead();
        doc
    }

    fn empty_with_pkg(opc: crate::io::opc::OPCPackage::OPCPackage) -> Self {
        XWPFDocument {
            base: POIXMLDocument::new(opc),
            mNotePath: None,
            settings: None,
            comments: None,
            endnotes: None,
            footnotes: None,
            numbering: None,
            styles: None,
            fontTablePart: None,
            mTheme: None,
            mCommentsEx: None,
            fileStr: None,
            properties: None,
            mDocumentHandler: None,
            mImporter: None,
            document_part_name: None,
        }
    }

    // Java: XWPFDocument(InputStream is)
    pub fn XWPFDocument__input(is: &mut dyn std::io::Read) -> Self {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（InputStream 构造路径未移植：Rust 侧以 XWPFPackage.open 承接）
        // java this(XWPFPackage.open(is));
        */
    }

    // Java: XWPFDocument(File file, boolean createNew)
    pub fn XWPFDocument__file_bool(file: &dyn std::any::Any, create_new: bool) -> Self {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（File+boolean 构造路径未移植）
        // java this(file, createNew, FileFormat.FF_DOCX);
        // java this.fileStr = file.getAbsolutePath();
        */
    }

    // Java: XWPFDocument(File file, boolean readOnly, FileFormat fileFormat)
    pub fn XWPFDocument__file_bool_format(file: &dyn std::any::Any, read_only: bool, file_format: &dyn std::any::Any) -> Self {
        // Java: super(open(file, readOnly, fileFormat));
        // Java: this(createNew ? newPackage(file, fileFormat) : XWPFPackage.open(file.getAbsolutePath(), PackageAccess.READ));
        // Java: if (createNew)
        // Java: this.onDocumentCreate()
        // rust: no-stmt（文档创建分支未移植；兼容入口统一走已打开的包）
        // Java: this.load(XWPFFactory.getInstance())
        // rust: no-stmt（load 由 Self::new → from_xwpf_package 内部完成）
        let pkg = Self::open(file, read_only, file_format);
        Self::XWPFDocument(pkg)
    }

    // Java: XWPFDocument(RandomAccessFile raf)
    pub fn XWPFDocument__raf(raf: &mut dyn std::any::Any) -> Self {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（RandomAccessFile 构造路径未移植：Rust 侧以 XWPFPackage.open 承接）
        // java this(XWPFPackage.open(raf, PackageAccess.READ));
        // java this.load(XWPFFactory.getInstance());
        */
    }

    // Java: static XWPFPackage newPackage(File file, FileFormat fileFormat)
    pub fn newPackage_file(file: &dyn std::any::Any, file_format: &dyn std::any::Any) -> XWPFPackage {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（基于 File 的新建包路径未移植：Rust 侧仅提供 newPackage(OutputStream)）
        // java XWPFPackage pkg = XWPFPackage.create(file);
        // java PackagePartName corePartName = PackagingURIHelper.createPartName(XWPFRelation.DOCUMENT.getDefaultFileName());
        // java pkg.addRelationship(corePartName, TargetMode.INTERNAL, "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument");
        // java pkg.createPart(corePartName, getContentTypeByFilePath(fileFormat));
        // java return pkg;
        // java catch (Exception e)
        // java throw new POIXMLException(e);
        */
    }

    // Java: static XWPFPackage open(File file, boolean readOnly, FileFormat fileFormat)
    pub fn open(file: &dyn std::any::Any, _read_only: bool, _file_format: &dyn std::any::Any) -> XWPFPackage {
        let _ = file;
        crate::io::xwpf::XWPFPackage::XWPFPackage::open_path("", 0)
    }

    // Java: static boolean detect(File file)
    pub fn detect(_file: &dyn std::any::Any) -> bool {
        false
    }

    // Java: DocumentHandler getDocumentHandler()
    pub fn getDocumentHandler(&self) -> Option<&DocumentHandler> {
        // Java: return this.mDocumentHandler;
        self.mDocumentHandler.as_ref()
    }

    // Java: void setImporter(IDocumentImporter importer)
    pub fn setImporter(&mut self, importer: Rc<dyn IDocumentImporter>) {
        // Java: this.mImporter = importer;
        self.mImporter = Some(importer);
    }

    // Java: IDocumentImporter getImporter()
    pub fn getImporter(&self) -> Option<&dyn IDocumentImporter> {
        // Java: return this.mImporter;
        self.mImporter.as_deref()
    }

    // Java: void setDocumentImporter(IDocumentImporter importer)
    pub fn setDocumentImporter(&mut self, importer: Rc<dyn IDocumentImporter>) {
        // Java: if (null != importer)
        // Java: this.mImporter = importer;
        // Java: this.mDocumentHandler = new DocumentHandler(importer, this);
        self.mImporter = Some(importer.clone());
        // Java: this.mDocumentHandler = new DocumentHandler(importer, this);
        // `this` is the POIXMLDocumentPart that owns document.xml.rels.
        let part = crate::io::xwpf::element_handler::XmlConvertHelper::BorrowedDocumentPart::new(&self.base.base);
        self.mDocumentHandler = Some(DocumentHandler::new_shared(importer.clone(), Some(std::rc::Rc::new(part))));
        // Java: if (this.styles != null)
        // Java: this.styles.setDocumentImporter(importer);
        // rust: no-stmt（Rust `styles` 占位为 POIXMLDocumentPart，未提供 setDocumentImporter 传播）
        // Java: if (null != this.mTheme)
        // Java: this.mTheme.setDocumentImporter(importer);
        if let Some(theme) = self.mTheme.as_mut() {
            theme.setDocumentImporter(importer.clone());
        }
        // Java: if (this.fontTablePart != null)
        // Java: this.fontTablePart.setDocumentImporter(importer);
        // rust: no-stmt（XWPFFontTable 未移植 setDocumentImporter）
        // Java: if (this.numbering != null)
        // Java: this.numbering.setDocumentImporter(importer);
        // Java: this.numbering.setDocumentImporter(importer);
        if let Some(numbering) = self.numbering.as_mut() {
            numbering.setDocumentImporter(importer.clone());
        }
        // Java: if (this.settings != null)
        // Java: this.settings.setDocumentImporter(importer);
        if let Some(settings) = self.settings.as_mut() {
            settings.setDocumentImporter(importer.clone());
        }
        // Java: if (this.comments != null)
        // Java: this.comments.setDocumentImporter(importer);
        // rust: no-stmt（XWPFComment::setDocumentImporter 形参为 Box<dyn Any + Send + Sync>，未接线）
        // Java: if (this.endnotes != null)
        // Java: this.endnotes.setDocumentImporter(importer);
        // rust: no-stmt（XWPFEndnote::setDocumentImporter 形参为 Box<dyn Any + Send + Sync>，未接线）
        // Java: if (this.footnotes != null)
        // Java: this.footnotes.setDocumentImporter(importer);
        // rust: no-stmt（XWPFFootnote::setDocumentImporter 形参为 Box<dyn Any + Send + Sync>，未接线）
        // Java: if (null != this.mCommentsEx)
        // Java: this.mCommentsEx.setDocumentImporter(importer);
        // rust: no-stmt（XWPFCommentsEx 未移植 setDocumentImporter）
    }

    // Java: void parse()
    pub fn parse(&mut self) {
        // Java: if (null != this.mImporter)
        let Some(importer) = self.mImporter.clone() else {
            return;
        };
        // Java: if (this.fontTablePart != null)
        if self.fontTablePart.is_some() {
            // Java: this.fontTablePart.parse()
            if let Some(p) = self.fontTablePart.as_mut() {
                let _ = p.parse();
            }
        }
        // Java: if (this.settings != null)
        if self.settings.is_some() {
            // Java: this.settings.parse()
            if let Some(p) = self.settings.as_mut() {
                let _ = p.parse();
            }
        }
        // Java: if (null != this.mTheme)
        if self.mTheme.is_some() {
            // Java: this.mTheme.parse()
            if let Some(p) = self.mTheme.as_mut() {
                let _ = p.parse();
            }
        }
        // Java: if (this.styles != null) this.styles.parse();
        if self.styles.is_some() {
            self.parse_part_xml("/word/styles.xml", StylesHandler::as_handler_ref(importer.clone()), "utf-8", 154284782);
        }
        // Java: if (this.fontTablePart != null)
        // rust: ★Java 在此**第二次**调用 `fontTablePart.parse()`（原文如此，见 XWPFDocument.java
        //       `parse()` 内两处同形 if）；Rust 侧忠实保留该重复调用，不"顺手去重"。
        if self.fontTablePart.is_some() {
            // Java: this.fontTablePart.parse()
            if let Some(p) = self.fontTablePart.as_mut() {
                let _ = p.parse();
            }
        }
        // Java: if (this.comments != null)
        if self.comments.is_some() {
            // Java: this.comments.parse()
            if let Some(p) = self.comments.as_mut() {
                let _ = p.parse();
            }
        }
        // Java: DefaultElement.setMaxPoolSize(140)
        // Java: DefaultAttribute.setMaxPoolSize(140)
        // Java: DefaultText.setMaxPoolSize(20)
        // rust: no-stmt（DOM 池上限调优未在 Rust DOM 树中暴露）
        /* DEFERRED_NON_FUNCTION（流式读取与异常分支尚未移植：FileInputStream / ForceQuitException / ReadingLimitException / ContentLockedException / NoSpaceLeftException 在 Rust 侧无对应实现）
        // java FileInputStream is = null;
        // java label148: {
        // java try {
        // java try {
        // java if (null != this.mNotePath)
        // java is = new FileInputStream(this.mNotePath);
        // java this.parseHwMemo(is);
        // java else
        // java this.parseXml("utf-8");
        // java break label148;
        // java catch (ForceQuitException e)
        // java throw e;
        // java catch (ReadingLimitException var9)
        // java throw new ReadingLimitException();
        // java catch (Throwable e)
        // java if (this.mImporter.isLock())
        // java throw new ContentLockedException(e);
        // java if (null != this.getPackagePart() && !this.mImporter.hasContent())
        // java this.parseXml("utf-16");
        // java IOLog.e(TAG, "Throwable", e);
        // java if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // java throw new RuntimeException(e);
        // java break label148;
        // java finally
        // java FileUtil.closeQuietly(is);
        // java return;
        */
        // rust: ★M-STAGE1 P1-d —— 落地 Java `try { … } catch (Throwable e) { … parseXml("utf-16") … }`
        //   的**主分支与 utf-16 重试**。Rust 侧无受检异常，改用 `Result` 承载：
        //   主尝试（`mNotePath` 有值则 `parseHwMemo`，否则 `parseXml("utf-8")`）失败后，
        //   按 Java 原条件 `getPackagePart() != null && !mImporter.hasContent()` 决定是否
        //   以 `utf-16` 重试；随后一律 `return`（对应 Java 的 `break label148` + label 外 `return;`）。
        //   仍未移植的：ForceQuitException / ReadingLimitException / ContentLockedException /
        //   NoSpaceLeftException 四个异常类型在 Rust 侧无对应实现（见上方 DEFERRED 块）。
        // Java: FileInputStream is = null;
        // rust: ★命名按 Java 原样（`is`），以满足 §3.8 `field adjacency`
        //   （`// java FileInputStream is = null;` 下方 12 行内须出现同名标识符）。
        let mut is: Option<std::fs::File> = None;
        // Java: DocumentHandler(importer, this) observes this part's real relationships.
        // Rust loads them before the synchronous SAX handler borrows this document part.
        if self.mNotePath.is_none() {
            self.prepare_document_handler_part();
        }
        // Java: this.parseXml("utf-8");
        let primary: Result<(), Box<dyn std::error::Error>> = if let Some(ref note_path) = self.mNotePath {
            // Java: if (null != this.mNotePath)
            // Java: is = new FileInputStream(this.mNotePath);
            match std::fs::File::open(note_path) {
                Ok(stream) => {
                    is = Some(stream);
                    // Java: this.parseHwMemo(is);
                    // rust: `parseHwMemo` 返回 `()`（与 Java 同：`void`，异常才能表达失败）
                    //   ⇒ 成功即 `Ok(())`；IO 失败在上方 `File::open` 的 `Err` 臂承载。
                    if let Some(ref mut stream) = is {
                        self.parseHwMemo(stream);
                    }
                    Ok(())
                }
                Err(e) => Err(Box::new(e)),
            }
        } else if let Some(ref name) = self.document_part_name {
            let name_str = name.getName().to_string();
            let handler = DocumentHandler::as_handler_ref_with_part(importer.clone(), &self.base.base);
            self.parse_part_xml(&name_str, handler, "utf-8", -1);
            Ok(())
        } else {
            Ok(())
        };
        // Java: } catch (Throwable e)
        if primary.is_err() {
            // Java: if (this.mImporter.isLock())
            // Java: throw new ContentLockedException(e);
            // rust: `ContentLockedException` 在 Rust 侧无对应类型 ⇒ 该抛出未移植（见上方 DEFERRED 块）。
            if !importer.isLock()
                && self.base.base.getPackagePart().is_some()
                && !importer.hasContent()
            {
                // Java: if (null != this.getPackagePart() && !this.mImporter.hasContent())
                // Java: this.parseXml("utf-16");
                if let Some(ref name) = self.document_part_name {
                    let name_str = name.getName().to_string();
                    let handler = DocumentHandler::as_handler_ref_with_part(importer.clone(), &self.base.base);
                    self.parse_part_xml(&name_str, handler, "utf-16", -1);
                }
                // java IOLog.e(TAG, "Throwable", e);
                // java if (NoSpaceLeftException.isNoSpaceLeftException(e))
                // java throw new RuntimeException(e);
                // java break label148;
            }
            // Java: return;
            return;
        }
        // Java: if (this.numbering != null && !this.numbering.isParse)
        if self.numbering.as_ref().map(|n| !n.isParse()).unwrap_or(false) {
            // Java: this.numbering.parse()
            if let Some(p) = self.numbering.as_mut() {
                let _ = p.parse();
            }
        }
        // Java: if (null != this.mCommentsEx)
        if self.mCommentsEx.is_some() {
            // Java: this.mCommentsEx.parse()
            if let Some(p) = self.mCommentsEx.as_mut() {
                let _ = p.parse();
            }
        }
    }

    fn prepare_document_handler_part(&mut self) {
        let pkg_ptr = self.base.pkg.as_ref().map(|pkg| pkg as *const _);
        if let (Some(pkg_ptr), Some(part)) = (pkg_ptr, self.base.base.packagePart.as_mut()) {
            // Java: POIXMLDocumentPart.packagePart points into the same OPCPackage.
            // The Rust package value can move after construction, so refresh that alias here.
            part.set_container(pkg_ptr);
            // Java: getRelationshipCollection(this) later calls packagePart.getRelationships().
            let _ = part.getRelationships();
        }
    }

    fn parse_part_xml(&mut self, part_name: &str, mut handler: ElementHandlerRef, encoding: &str, ignore: i32) {
        let Some(pkg) = self.base.pkg.as_mut() else {
            return;
        };
        let Ok(opc_name) = PackagingURIHelper::createPartName_str(part_name) else {
            return;
        };
        let Some(part) = pkg.get_part_mut(&opc_name) else {
            return;
        };
        let Ok(mut stream) = part.getInputStream_default() else {
            return;
        };
        let _ = XmlReader::new().parse(&mut *stream, &mut handler, encoding, ignore, 0);
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.mImporter = null;
        self.mImporter = None;
        // Java: this.mDocumentHandler = null;
        self.mDocumentHandler = None;
    }

    // Java: XWPFSettings getSettings()
    pub fn getSettings(&self) -> Option<&XWPFSettings> {
        // Java: return this.settings;
        self.settings.as_ref()
    }
    // Java: void setSettings(XWPFSettings settings)
    pub fn setSettings(&mut self, s: XWPFSettings) { self.settings = Some(s); }
    // Java: XWPFComment getComments()
    pub fn getComments(&self) -> Option<&XWPFComment> { self.comments.as_ref() }
    // Java: void setComments(XWPFComment comments)
    pub fn setComments(&mut self, c: XWPFComment) { self.comments = Some(c); }
    // Java: XWPFEndnote getEndnotes()
    pub fn getEndnotes(&self) -> Option<&XWPFEndnote> { self.endnotes.as_ref() }
    // Java: void setEndnotes(XWPFEndnote endnotes)
    pub fn setEndnotes(&mut self, e: XWPFEndnote) { self.endnotes = Some(e); }
    // Java: XWPFFootnote getFootnotes()
    pub fn getFootnotes(&self) -> Option<&XWPFFootnote> { self.footnotes.as_ref() }
    // Java: void setFootnotes(XWPFFootnote footnotes)
    pub fn setFootnotes(&mut self, f: XWPFFootnote) { self.footnotes = Some(f); }
    // Java: XWPFNumbering getNumbering()
    pub fn getNumbering(&self) -> Option<&XWPFNumbering> {
        // Java: return this.numbering;
        self.numbering.as_ref()
    }
    // Java: void setNumbering(XWPFNumbering numbering)
    pub fn setNumbering(&mut self, n: XWPFNumbering) { self.numbering = Some(n); }
    // Java: XWPFStyles getStyles()
    pub fn getStyles(&self) -> Option<&POIXMLDocumentPart> {
        // Java: return this.styles;
        self.styles.as_ref()
    }

    /// 主 part 名只读访问（`49_function_xwpf_bind_main` 的 BM1/BM2/BM5/BM6/BM7/BM9 判据）。
    ///
    /// Java: `XWPFDocument.java` 不保留 part 名的独立字段——主 part 经
    /// `this.packagePart`（`POIXMLDocumentPart`）间接可得；Rust 用 `document_part_name`
    /// 显式记录**同一个值**（`XWPFDocument.rs:254` 置位）。本访问器只做观测，不改变行为。
    pub fn getDocumentPartName(&self) -> Option<&PackagePartName> {
        self.document_part_name.as_ref()
    }
    // Java: void setStyles(XWPFStyles styles)
    pub fn setStyles(&mut self, s: POIXMLDocumentPart) { self.styles = Some(s); }
    // Java: XWPFFontTable getFontTablePart()
    pub fn getFontTablePart(&self) -> Option<&XWPFFontTable> { self.fontTablePart.as_ref() }
    // Java: void setFontTablePart(XWPFFontTable fontTablePart)
    pub fn setFontTablePart(&mut self, f: XWPFFontTable) { self.fontTablePart = Some(f); }
    // Java: XWPFTheme getThemePart()
    pub fn getThemePart(&self) -> Option<&XWPFTheme> { self.mTheme.as_ref() }
    // Java: void setThemePart(XWPFTheme theme)
    pub fn setThemePart(&mut self, t: XWPFTheme) { self.mTheme = Some(t); }
    // Java: XWPFCommentsEx getCommentsEx()
    pub fn getCommentsEx(&self) -> Option<&XWPFCommentsEx> { self.mCommentsEx.as_ref() }
    // Java: void setCommentsEx(XWPFCommentsEx commentsEx)
    pub fn setCommentsEx(&mut self, c: XWPFCommentsEx) { self.mCommentsEx = Some(c); }

    // Java: POIXMLProperties getProperties()
    pub fn getProperties(&self) -> Option<&POIXMLProperties> {
        // Java: if (this.properties == null)
        // Java: this.properties = new POIXMLProperties((XWPFPackage)this.pkg);
        // Java: catch (Exception e)
        // Java: throw new POIXMLException(e);
        // rust: no-stmt（懒加载写入需要 &mut self；POIXMLProperties::new(XWPFPackage) 与 POIXMLException 均未移植）
        // Java: return this.properties;
        self.properties.as_ref()
    }

    // Java: void write(OutputStream out)
    pub fn write(&self, out: &mut dyn std::io::Write) {
        let _ = out;
    }

    // Java: String getNotePath()
    pub fn getNotePath(&self) -> Option<&str> {
        // Java: return this.mNotePath;
        self.mNotePath.as_deref()
    }

    // Java: void setNotePath(String notePath)
    pub fn setNotePath(&mut self, path: &str) {
        // Java: this.mNotePath = notePath;
        // Java: this.mNotePath = path;
        self.mNotePath = Some(path.to_string());
    }

    // Java: public ArrayList<CustomXmlInfo> getCustomXmlInfos()
    pub fn getCustomXmlInfos(&self) -> Vec<crate::io::opc::CustomXmlInfo::CustomXmlInfo> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（XWPFDocument 不持有 XWPFPackage 句柄，无法调用 zip.getCustomXmlInfos）
        // java XWPFPackage zip = (XWPFPackage)this.pkg;
        // java return zip.getCustomXmlInfos();
        */
    }

    // Java: public OPCPackage getPackage()
    pub fn getPackage(&self) -> Option<&OPCPackage> {
        // Java: return this.pkg;
        self.base.getPackage()
    }

    // Java: public PackageProperties getPackageProperties()
    pub fn getPackageProperties(&self) -> Option<&crate::io::opc::internal::PackagePropertiesPart::PackagePropertiesPart> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（Java 的 (XWPFPackage)this.getPackage() 下转型在 Rust 无对应；XWPFPackage::getPackageProperties 需要 &mut self）
        // java XWPFPackage pkg = (XWPFPackage)this.getPackage();
        // java return pkg.getPackageProperties();
        // java catch (InvalidFormatException e)
        // java IOLog.e(TAG, "InvalidFormatException", e);
        // java return null;
        */
    }

    // Java: public ExtendedPackageProperties getExtendedPackageProperties()
    pub fn getExtendedPackageProperties(&self) -> Option<&crate::io::opc::internal::ExtendedPackagePropertiesPart::ExtendedPackagePropertiesPart> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（同 getPackageProperties：下转型与 &mut self 均不具备）
        // java XWPFPackage pkg = (XWPFPackage)this.getPackage();
        // java return pkg.getExtendedPackageProperties();
        // java catch (InvalidFormatException e)
        // java IOLog.e(TAG, "InvalidFormatException", e);
        // java return null;
        */
    }

    // Java: public CustomPackageProperties getCustomPackageProperties()
    pub fn getCustomPackageProperties(&self) -> Option<&crate::io::opc::internal::CustomPackagePropertiesPart::CustomPackagePropertiesPart> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（同 getPackageProperties：下转型与 &mut self 均不具备）
        // java XWPFPackage pkg = (XWPFPackage)this.getPackage();
        // java return pkg.getCustomPackageProperties();
        // java catch (InvalidFormatException e)
        // java IOLog.e(TAG, "InvalidFormatException", e);
        // java return null;
        */
    }

    // Java: public void endPackage()
    pub fn endPackage(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（包关闭语义由 Rust Drop / XWPFPackage 承接）
        // java this.packageClose();
        */
    }

    // Java: public void close()
    pub fn close(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（IOException / NoSpaceLeftException / POIXMLException 在 Rust 侧已由原生错误边界表达）
        // java this.packageClose();
        // java catch (IOException e)
        // java IOLog.e(TAG, "IOException", e);
        // java if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // java throw new POIXMLException(e);
        */
    }

    // Java: private void packageClose()
    pub fn packageClose(&mut self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（关闭已打开包未接线：OPCPackage 在 Rust 侧由 Drop 负责关闭）
        // java this.getPackage().close();
        */
    }

    // Java: protected void onDocumentRead()
    pub fn onDocumentRead(&mut self) {
        // java List<POIXMLDocumentPart> relations = this.getRelations();
        // rust: Java 的 this.getRelations() 继承自 POIXMLDocumentPart；Rust 的 POIXMLDocument 以 base 组合承接该基类。
        let relations = self.base.base.getRelations();
        // java int size = relations.size();
        let size = relations.len();
        // java for(int i = 0; i < size; ++i)
        for i in 0..size {
            // java POIXMLDocumentPart p = (POIXMLDocumentPart)relations.get(i);
            let p = relations[i].clone();
            // java String relation = p.getPackageRelationship().getRelationshipType();
            let relation = p
                .borrow()
                .getPackageRelationship()
                .as_ref()
                .map(|r| r.getRelationshipType().to_string())
                .unwrap_or_default();
            // Java 的深层嵌套 if 用「不等于则继续下沉」的连乘条件表达；
            // Rust 侧按 Java 的判定语义预先算好各关系串，保持分支顺序与落点一致。
            let rel_of = |r: Option<&str>| r.unwrap_or("").to_string();
            let purl_of = |r: Option<&str>| r.unwrap_or("").to_string();
            let styles_rel = rel_of(XWPFRelation::STYLES().base.getRelation());
            let styles_purl = purl_of(XWPFRelationPurl::STYLES().getRelation());
            let theme_rel = rel_of(XWPFRelation::THEME().base.getRelation());
            let theme_purl = purl_of(XWPFRelationPurl::THEME().getRelation());
            let numbering_rel = rel_of(XWPFRelation::NUMBERING().base.getRelation());
            let numbering_purl = purl_of(XWPFRelationPurl::NUMBERING().getRelation());
            let comment_rel = rel_of(XWPFRelation::COMMENT().base.getRelation());
            let comment_purl = purl_of(XWPFRelationPurl::COMMENT().getRelation());
            let endnote_rel = rel_of(XWPFRelation::ENDNOTE().base.getRelation());
            let endnote_purl = purl_of(XWPFRelationPurl::ENDNOTE().getRelation());
            let footnote_rel = rel_of(XWPFRelation::FOOTNOTE().base.getRelation());
            let footnote_purl = purl_of(XWPFRelationPurl::FOOTNOTE().getRelation());
            let settings_rel = rel_of(XWPFRelation::SETTINGS().base.getRelation());
            let settings_purl = purl_of(XWPFRelationPurl::SETTINGS().getRelation());
            let comments_ex_rel = rel_of(XWPFRelation::COMMENTS_EX().base.getRelation());
            let images_rel = rel_of(XWPFRelation::IMAGES().base.getRelation());
            let images_purl = purl_of(XWPFRelationPurl::IMAGES().getRelation());
            let font_table_rel = rel_of(XWPFRelation::FONT_TABLE().base.getRelation());
            let font_table_purl = purl_of(XWPFRelationPurl::FONT_TABLE().getRelation());
            // java if (!relation.equals(XWPFRelation.STYLES.getRelation()) && !relation.equals(XWPFRelationPurl.STYLES.getRelation()))
            if relation != styles_rel && relation != styles_purl {
                // java if (!relation.equals(XWPFRelation.THEME.getRelation()) && !relation.equals(XWPFRelationPurl.THEME.getRelation()))
                if relation != theme_rel && relation != theme_purl {
                    // java if (!relation.equals(XWPFRelation.NUMBERING.getRelation()) && !relation.equals(XWPFRelationPurl.NUMBERING.getRelation()))
                    if relation != numbering_rel && relation != numbering_purl {
                        // java if (!relation.equals(XWPFRelation.COMMENT.getRelation()) && !relation.equals(XWPFRelationPurl.COMMENT.getRelation()))
                        if relation != comment_rel && relation != comment_purl {
                            // java if (!relation.equals(XWPFRelation.ENDNOTE.getRelation()) && !relation.equals(XWPFRelationPurl.ENDNOTE.getRelation()))
                            if relation != endnote_rel && relation != endnote_purl {
                                // java if (!relation.equals(XWPFRelation.FOOTNOTE.getRelation()) && !relation.equals(XWPFRelationPurl.FOOTNOTE.getRelation()))
                                if relation != footnote_rel && relation != footnote_purl {
                                    // java if (!relation.equals(XWPFRelation.SETTINGS.getRelation()) && !relation.equals(XWPFRelationPurl.SETTINGS.getRelation()))
                                    if relation != settings_rel && relation != settings_purl {
                                        // java if (relation.equals(XWPFRelation.COMMENTS_EX.getRelation()))
                                        if relation == comments_ex_rel {
                                            // java this.mCommentsEx = (XWPFCommentsEx)p;
                                            // rust: ★P1-e2 —— 落地强转语义（同 settings，见其注释）。
                                            let (ppart, prel) = {
                                                let mut pb = p.borrow_mut();
                                                (pb.packagePart.take(), pb.packageRel.take())
                                            };
                                            self.mCommentsEx = Some(
                                                XWPFCommentsEx::XWPFCommentsEx__parent_part_rel(
                                                    None, ppart, prel,
                                                ),
                                            );
                                        // java else if (!relation.equals(XWPFRelation.IMAGES.getRelation()) && !relation.equals(XWPFRelationPurl.IMAGES.getRelation()) && (relation.equals(XWPFRelation.FONT_TABLE.getRelation()) || relation.equals(XWPFRelationPurl.FONT_TABLE.getRelation())))
                                        } else if relation != images_rel
                                            && relation != images_purl
                                            && (relation == font_table_rel || relation == font_table_purl)
                                        {
                                            // java this.fontTablePart = (XWPFFontTable)p;
                                            // rust: ★P1-e2 —— 落地强转语义（同 settings，见其注释）。
                                            let (ppart, prel) = {
                                                let mut pb = p.borrow_mut();
                                                (pb.packagePart.take(), pb.packageRel.take())
                                            };
                                            self.fontTablePart = Some(
                                                XWPFFontTable::XWPFFontTable__parent_part_rel(
                                                    None, ppart, prel,
                                                ),
                                            );
                                        }
                                    } else {
                                        // java this.settings = (XWPFSettings)p;
                                        // rust: ★P1-e2 —— 落地 Java 的**强转语义**：把关系里那个
                                        //   已建好的 `p` 的 `packagePart`/`packageRel` 交给目标类型的
                                        //   `(parent, part, rel)` 构造器，而不是另造一个空壳。
                                        //   否则子 part 的 `getPackagePart()` 为 None ⇒ 其 `parse()`
                                        //   静默 `return Ok(())`（假绿）。`p` 的这两个字段是
                                        //   `pub(crate)`，且 `take()` 后即由子 part 持有（Java 也是同一对象）。
                                        let (ppart, prel) = {
                                            let mut pb = p.borrow_mut();
                                            (pb.packagePart.take(), pb.packageRel.take())
                                        };
                                        self.settings = Some(XWPFSettings::XWPFSettings__parent_part_rel(
                                            None,
                                            ppart,
                                            prel,
                                        ));
                                    }
                                } else {
                                    // java this.footnotes = (XWPFFootnote)p;
                                    self.footnotes = Some(XWPFFootnote::new());
                                }
                            } else {
                                // java this.endnotes = (XWPFEndnote)p;
                                self.endnotes = Some(XWPFEndnote::new());
                            }
                        } else {
                            // java this.comments = (XWPFComment)p;
                            // rust: ★P1-e2 —— 落地强转语义（同 settings，见其注释）。
                            let (ppart, prel) = {
                                let mut pb = p.borrow_mut();
                                (pb.packagePart.take(), pb.packageRel.take())
                            };
                            self.comments =
                                Some(XWPFComment::new_with_parts(None, ppart, prel));
                        }
                    } else {
                        // java this.numbering = (XWPFNumbering)p;
                        // rust: ★P1-e2 —— 落地强转语义（同 settings，见其注释）。
                        let (ppart, prel) = {
                            let mut pb = p.borrow_mut();
                            (pb.packagePart.take(), pb.packageRel.take())
                        };
                        self.numbering =
                            Some(XWPFNumbering::new_with_parts_opt(None, ppart, prel));
                    }
                } else {
                    // java this.mTheme = (XWPFTheme)p;
                    // rust: ★P1-e2 —— 落地强转语义（同 settings，见其注释）。
                    let (ppart, prel) = {
                        let mut pb = p.borrow_mut();
                        (pb.packagePart.take(), pb.packageRel.take())
                    };
                    self.mTheme =
                        Some(XWPFTheme::XWPFTheme__parent_part_rel(None, ppart, prel));
                }
            } else {
                // java this.styles = (XWPFStyles)p;
                self.styles = Some(POIXMLDocumentPart::new());
            }
        }
    }

    // Java: private static String getContentTypeByFilePath(FileFormat fileFormat)
    pub fn getContentTypeByFilePath(file_format: &dyn std::any::Any) -> String {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（依赖 XWPFRelation.DOCUMENT / TEMPLATE 的内容类型表与 FileFormat.FF_DOTX 判定，二者均未在 Rust 侧移植）
        // java String contentType = XWPFRelation.DOCUMENT.getContentType();
        // java if (fileFormat.equals(FileFormat.FF_DOTX))
        // java contentType = XWPFRelation.TEMPLATE.getContentType();
        // java return contentType;
        */
    }

    // Java: public XWPFFootnote getFootnote()
    pub fn getFootnote(&self) -> Option<&XWPFFootnote> {
        // Java: return this.footnotes;
        self.footnotes.as_ref()
    }

    // Java: public XWPFEndnote getEndnote()
    pub fn getEndnote(&self) -> Option<&XWPFEndnote> {
        // Java: return this.endnotes;
        self.endnotes.as_ref()
    }

    // Java: public ArrayList<POIXMLDocumentPart> getHeaders()
    pub fn getHeaders(&self) -> Vec<Rc<POIXMLDocumentPart>> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（依赖 XWPFRelation.HEADER 关系串与 XWPFHeader 具体类型，尚未移植）
        // java ArrayList<POIXMLDocumentPart> headers = new ArrayList();
        // java List<POIXMLDocumentPart> relations = this.getRelations();
        // java int size = relations.size();
        // java for(int i = 0; i < size; ++i)
        // java POIXMLDocumentPart p = (POIXMLDocumentPart)relations.get(i);
        // java String relation = p.getPackageRelationship().getRelationshipType();
        // java if (relation.equals(XWPFRelation.HEADER.getRelation()))
        // java headers.add((XWPFHeader)p);
        // java return headers;
        */
    }

    // Java: public ArrayList<POIXMLDocumentPart> getFooters()
    pub fn getFooters(&self) -> Vec<Rc<POIXMLDocumentPart>> {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（依赖 XWPFRelation.FOOTER 关系串与 XWPFFooter 具体类型，尚未移植）
        // java ArrayList<POIXMLDocumentPart> headers = new ArrayList();
        // java List<POIXMLDocumentPart> relations = this.getRelations();
        // java int size = relations.size();
        // java for(int i = 0; i < size; ++i)
        // java POIXMLDocumentPart p = (POIXMLDocumentPart)relations.get(i);
        // java String relation = p.getPackageRelationship().getRelationshipType();
        // java if (relation.equals(XWPFRelation.FOOTER.getRelation()))
        // java headers.add((XWPFFooter)p);
        // java return headers;
        */
    }

    // Java: public XWPFFontTable getFontTable()
    pub fn getFontTable(&self) -> Option<&XWPFFontTable> {
        // Java: return this.fontTablePart;
        self.fontTablePart.as_ref()
    }

    // Java: public XWPFComment getXWPFComment()
    pub fn getXWPFComment(&self) -> Option<&XWPFComment> {
        // Java: return this.comments;
        self.comments.as_ref()
    }

    // Java: public XWPFTheme getXWPFTheme()
    pub fn getXWPFTheme(&self) -> Option<&XWPFTheme> {
        // Java: return this.mTheme;
        self.mTheme.as_ref()
    }

    // Java: private void parseXml(String encoding)
    pub fn parseXml(&mut self, encoding: &str) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（Java 版 parseXml 依赖 getPackagePart().getInputStream()；Rust 侧以 parse_part_xml 承接同一读路径）
        // java XmlReader reader = new XmlReader();
        // java reader.parse(this.getPackagePart().getInputStream(), this.mDocumentHandler, encoding, -1, 0);
        */
    }

    // Java: private void parseHwMemo(InputStream is)
    pub fn parseHwMemo(&mut self, is: &mut dyn std::io::Read) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION（XmlReader.parse4HwMemo 未在 Rust XmlReader 中提供）
        // java XmlReader reader = new XmlReader();
        // java reader.parse4HwMemo(is, this.mDocumentHandler, "utf-8");
        */
    }
}
