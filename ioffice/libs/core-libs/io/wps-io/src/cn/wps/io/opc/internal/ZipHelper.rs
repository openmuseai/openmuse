#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.io.opc.PackageRelationship
// Java: import cn.wps.io.util.URI
// Java: import cn.wps.io.xwpf.XWPFPackage
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import cn.wps.moffice.kfs.zip.ZipFile
// Java: import java.io.IOException
// Java: import java.net.URISyntaxException
// Java: import java.util.Enumeration

use crate::io::xwpf::util::ZipEntrySource::ZipEntryRef;
use std::path::PathBuf;

use crate::io::util::URI::URI;
use crate::io::opc::OPCPackage::OPCPackage;
use crate::io::xwpf::util::ZipFileZipEntrySource::ZipFileZipEntrySource;
use crate::io::xwpf::util::ZipInputStreamZipEntrySource::ZipInputStreamZipEntrySource;
use core_libs_kfs::kfs::zip::ZipFile::ZipFile;

// Java field: String TAG
const TAG: Option<&str> = None;
// Java field: char FORWARD_SLASH
const FORWARD_SLASH: char = '/';
// Java field: int READ_WRITE_FILE_BUFFER_SIZE
pub const READ_WRITE_FILE_BUFFER_SIZE: i32 = 8192;

// Java: public class ZipHelper
pub struct ZipHelper;

// Java: public class ZipHelper
impl ZipHelper {
    // Java: ZipHelper()
    fn new() -> Self {
        // Java: super();
        ZipHelper
    }

    // Java: static URI getCorePropertiesURI(XWPFPackage pkg)
    pub fn getCorePropertiesURI(pkg: &mut OPCPackage) -> Option<URI> {
    // ★2026-09-20（"接入真实类型，包括整个类的其他地方"）：形参由 `&mut dyn std::any::Any` 改为
    //   Java 体内的**真实所需类型** `&mut OPCPackage` —— Java 签名写的是 `XWPFPackage pkg`，
    //   但方法体只调 `pkg.getRelationshipsByType(...)`，而该方法声明在**基类** `OPCPackage` 上
    //   ⇒ 声明类型收窄成 `XWPFPackage` 并非必要；改为真实类型后，旧实现里的
    //   `pkg.downcast_mut::<XWPFPackage>()`（`dyn Any` 试探，失败即静默 `None`）随之删除。
        // Java: PackageRelationship corePropsRel = pkg.getRelationshipsByType("http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties").getRelationship(0);
        let coll = pkg.getRelationshipsByType(
            "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
        );
        let core_props_rel = coll.getRelationship_by_index(0);
        // Java: return corePropsRel == null ? null : corePropsRel.getTargetURI();
        core_props_rel.map(|r| r.getTargetURI())
    }

    // Java: static URI getExtendedPropertiesURI(XWPFPackage pkg)
    pub fn getExtendedPropertiesURI(pkg: &mut OPCPackage) -> Option<URI> {
    // ★2026-09-20（"接入真实类型，包括整个类的其他地方"）：形参由 `&mut dyn std::any::Any` 改为
    //   Java 体内的**真实所需类型** `&mut OPCPackage` —— Java 签名写的是 `XWPFPackage pkg`，
    //   但方法体只调 `pkg.getRelationshipsByType(...)`，而该方法声明在**基类** `OPCPackage` 上
    //   ⇒ 声明类型收窄成 `XWPFPackage` 并非必要；改为真实类型后，旧实现里的
    //   `pkg.downcast_mut::<XWPFPackage>()`（`dyn Any` 试探，失败即静默 `None`）随之删除。
        // Java: PackageRelationship extendedPropsRel = pkg.getRelationshipsByType("http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties").getRelationship(0);
        let coll = pkg.getRelationshipsByType(
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
        );
        let mut extended_props_rel = coll.getRelationship_by_index(0).map(|r| r.getTargetURI());
        // Java: if (extendedPropsRel == null)
        if extended_props_rel.is_none() {
            // Java: extendedPropsRel = pkg.getRelationshipsByType("http://purl.oclc.org/ooxml/officeDocument/relationships/extendedProperties").getRelationship(0);
            let coll2 = pkg.getRelationshipsByType(
                "http://purl.oclc.org/ooxml/officeDocument/relationships/extendedProperties",
            );
            extended_props_rel = coll2.getRelationship_by_index(0).map(|r| r.getTargetURI());
        }
        // Java: return extendedPropsRel == null ? null : extendedPropsRel.getTargetURI();
        extended_props_rel
    }

    // Java: static URI getCustomPropertiesURI(XWPFPackage pkg)
    pub fn getCustomPropertiesURI(pkg: &mut OPCPackage) -> Option<URI> {
    // ★2026-09-20（"接入真实类型，包括整个类的其他地方"）：形参由 `&mut dyn std::any::Any` 改为
    //   Java 体内的**真实所需类型** `&mut OPCPackage` —— Java 签名写的是 `XWPFPackage pkg`，
    //   但方法体只调 `pkg.getRelationshipsByType(...)`，而该方法声明在**基类** `OPCPackage` 上
    //   ⇒ 声明类型收窄成 `XWPFPackage` 并非必要；改为真实类型后，旧实现里的
    //   `pkg.downcast_mut::<XWPFPackage>()`（`dyn Any` 试探，失败即静默 `None`）随之删除。
        // Java: PackageRelationship customPropsRel = pkg.getRelationshipsByType("http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties").getRelationship(0);
        let coll = pkg.getRelationshipsByType(
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties",
        );
        let mut custom_props_rel = coll.getRelationship_by_index(0).map(|r| r.getTargetURI());
        // Java: if (customPropsRel == null)
        if custom_props_rel.is_none() {
            // Java: customPropsRel = pkg.getRelationshipsByType("http://purl.oclc.org/ooxml/officeDocument/customProperties").getRelationship(0);
            let coll2 = pkg.getRelationshipsByType(
                "http://purl.oclc.org/ooxml/officeDocument/customProperties",
            );
            custom_props_rel = coll2.getRelationship_by_index(0).map(|r| r.getTargetURI());
        }
        // Java: return customPropsRel == null ? null : customPropsRel.getTargetURI();
        custom_props_rel
    }

    // Java: static ZipEntry getContentTypeZipEntry(XWPFPackage pkg)
    // ★2026-09-20（"接入真实类型，包括整个类的其他地方"）：形参 `&dyn std::any::Any` → `&OPCPackage`
    //   （Java 体只调 `pkg.getZipArchive()`，该方法声明在**基类** `OPCPackage` 上 ⇒ `XWPFPackage`
    //   这一收窄并非必要）；返回类型 `Option<Box<dyn Any + Send + Sync>>`（旧实现**从未真正返回值**，
    //   只做了名字比对后 `let _ = entry;`）→ 真实类型 `Option<ZipEntryRef>`（对应 Java 的 `ZipEntry`）。
    pub fn getContentTypeZipEntry(pkg: &OPCPackage) -> Option<ZipEntryRef> {
        // Java: Enumeration<? extends ZipEntry> entries = pkg.getZipArchive().getEntries();
        let archive = pkg.getZipArchive().as_ref()?;
        let entries = archive.getEntries()?;
        // Java: while(entries.hasMoreElements())
        // Java: ZipEntry entry = (ZipEntry)entries.nextElement();
        // Java: if (entry.getName().equals("[Content_Types].xml"))
        // Java: return entry;
        entries
            .iter()
            .find(|e| e.getName() == "[Content_Types].xml")
            .map(|e| ZipEntryRef::Fake(e.clone()))
        // Java: return null;
        // （`find` 未命中即返回 `None`，与 Java 的 `return null` 一致）
    }

    // Java: static String getOPCNameFromZipItemName(String zipItemName)
    pub fn getOPCNameFromZipItemName(zip_item_name: &str) -> String {
        // Java: if (zipItemName == null)
        // Java: throw new IllegalArgumentException("zipItemName");
        // Java: else
        // Java: int len = zipItemName.length();
        let len = zip_item_name.len();
        // Java: if (len > 0 && zipItemName.charAt(0) == '/')
        if len > 0 && zip_item_name.starts_with('/') {
            // Java: return zipItemName;
            return zip_item_name.to_string();
        // Java: else
        } else {
            // Java: char[] chars = new char[len + 1];
            let mut chars = String::with_capacity(len + 1);
            // Java: chars[0] = '/';
            chars.push(FORWARD_SLASH);
            // Java: zipItemName.getChars(0, len, chars, 1);
            chars.push_str(zip_item_name);
            // Java: return new String(chars);
            return chars;
        }
    }

    // Java: static String getZipItemNameFromOPCName(String opcItemName)
    pub fn getZipItemNameFromOPCName(opc_item_name: &str) -> String {
        // Java: if (opcItemName == null)
        // Java: throw new IllegalArgumentException("opcItemName");
        // Java: else
        // Java: String retVal;
        // Java: for(retVal = opcItemName; retVal.charAt(0) == '/'; retVal = retVal.substring(1))
        // Java: return retVal;
        let mut ret_val = opc_item_name;
        while ret_val.starts_with('/') {
            ret_val = &ret_val[1..];
        }
        ret_val.to_string()
    }

    // Java: static URI getZipURIFromOPCName(String opcItemName)
    pub fn getZipURIFromOPCName(opc_item_name: &str) -> Option<URI> {
        // Java: if (opcItemName == null)
        // Java: throw new IllegalArgumentException("opcItemName");
        // Java: else
        // Java: String retVal;
        // Java: for(retVal = opcItemName; retVal.charAt(0) == '/'; retVal = retVal.substring(1))
        let mut ret_val = opc_item_name;
        while ret_val.starts_with('/') {
            ret_val = &ret_val[1..];
        }
        // Java: try
        // Java: return new URI(retVal);
        // Java: catch (URISyntaxException e)
        // Java: IOLog.e(TAG, "URISyntaxException: " + e);
        // Java: return null;
        match URI::new(ret_val) {
            Ok(uri) => Some(uri),
            Err(_) => {
                let _ = TAG;
                None
            }
        }
    }

    // Java: static ZipFile openZipFile(String path)
    pub fn openZipFile(path: &str) -> Option<ZipFile> {
        // Java: File f = new File(path);
        let f = path;
        // Java: try
        // Java: return new ZipFile(f);
        // Java: catch (IOException ioe)
        // Java: IOLog.e(TAG, "IOException: " + ioe);
        // Java: return null;
        // Rust: kfs `ZipFile::new__string` 在坏 zip / 缺文件时会 panic（移植时把 Java 的
        //       `throws IOException` 写成了 panic）⇒ 走可恢复入口 `try_new__string`
        match ZipFile::try_new__string(f.to_string()) {
            Ok(zipFile) => Some(zipFile),
            Err(_) => {
                let _ = TAG;
                None
            }
        }
    }

    // Java: static ZipFile openZipFile(RandomAccessFile raf)
    pub fn openZipFile_raf(raf: &dyn std::any::Any) -> Option<ZipFile> {
        // Java: try
        // Java: return new ZipFile(raf);
        // Java: catch (IOException e)
        // Java: IOLog.e(TAG, "IOException: " + e);
        // Java: return null;
        // Rust-only: Java 的入参是强类型 `RandomAccessFile`；Rust 侧没有同名可搬运句柄
        //            （kfs `RandomAccessFile` 内部持有 `Box<dyn IMfsFile>`，不可跨调用方传递），
        //            故本入口按「路径形态」接收（`String` / `PathBuf`），其余类型返回 `None`。
        if let Some(path) = raf.downcast_ref::<String>() {
            return Self::openZipFile(path);
        }
        if let Some(path) = raf.downcast_ref::<PathBuf>() {
            return Self::openZipFile(path.to_string_lossy().as_ref());
        }
        let _ = TAG;
        None
    }
}

