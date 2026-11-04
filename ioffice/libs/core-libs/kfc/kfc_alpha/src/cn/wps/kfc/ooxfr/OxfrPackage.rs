#![allow(non_snake_case, unused_imports)]

// Java: import cn.wps.kfc.opcr.OpcrPackage
use crate::kfc::opcr::OpcrPackage::OpcrPackage;
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use crate::kfc::opcr::OpcrPackagePart::OpcrPackagePart;
// Java: import cn.wps.kfc.opcr.OpcrRelationship
use crate::kfc::opcr::OpcrRelationship::OpcrRelationship;
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use crate::kfc::opcr::OpcrRelationships::OpcrRelationships;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use std::io::Read;
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use crate::kfc::opcr::ZipFile::{ZipEntry, ZipFile};
// Java: import java.io.IOException
// rust: no-stmt
// Java: import java.util.Enumeration
// rust: no-stmt
use crate::kfc::ooxfr::OxfRelations::{APP, CORE, CUSTOM};
use crate::kfc::{opcr::OpcrRelationship::PackagePartObject,ooxfr::{OxfrAppPart::OxfrAppPart,OxfrCorePart::OxfrCorePart,OxfrCustomPart::OxfrCustomPart}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxfrPackageError {
    message: String,
}

impl std::fmt::Display for OxfrPackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for OxfrPackageError {}

impl From<std::io::Error> for OxfrPackageError {
    fn from(e: std::io::Error) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}

// java public final class OxfrPackage extends OpcrPackage
pub struct OxfrPackage {
    // java OpcrPackage super_
    pub(crate) super_: OpcrPackage,
}

// java public final class OxfrPackage extends OpcrPackage
impl OxfrPackage {
    // java public  OxfrPackage(String filePath)
    pub fn OxfrPackage(filePath: &str) -> Result<Self, OxfrPackageError> {
      // java super(filePath);
      Ok(Self { super_: OpcrPackage::OpcrPackage(filePath)? })
    }

    // java public  OxfrPackage(RandomAccessFile raf)
    pub fn OxfrPackage__raf(raf: &mut dyn Read) -> Result<Self, OxfrPackageError> {
      // java super(raf);
      Ok(Self { super_: OpcrPackage::OpcrPackage__raf(raf)? })
    }

    // java public int type()
    pub fn type_(&self) -> i32 {
      // java return 1;
      return 1;
    }

    pub fn r#type(&self) -> i32 {
        self.type_()
    }

    // java public OpcrPackagePart getOfficeDocument()
    pub fn getOfficeDocument(&mut self) -> Option<&OpcrPackagePart> {
      // java for(OpcrRelationship rels : this.mRelses)
      let n = self.super_.mRelses.size();
      let mut found = None;
      for i in 0..n {
        let rels = self.super_.mRelses.getRelationship(i);
        // java if (null != rels && null != rels.getType() && rels.getType().equals("http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"))
        if rels.getType().as_deref() == Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument") {
          // java return rels.getPackagePart();
          found = Some(i);
          break;
        }
      }
      // java return null;
      return found.and_then(|i| self.super_.mRelses.getRelationship_mut(i).getPackagePart());
    }

    pub fn getOfficeDocument_mut(&mut self) -> Option<&mut OpcrPackagePart> {
      let n = self.super_.mRelses.size();
      let mut found = None;
      for i in 0..n {
        let rels = self.super_.mRelses.getRelationship(i);
        if rels.getType().as_deref()
            == Some("http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument")
        {
          found = Some(i);
          break;
        }
      }
      found.and_then(|i| self.super_.mRelses.getRelationship_mut(i).getPackagePart_mut())
    }

    // java public OpcrRelationships getRelationships()
    pub fn getRelationships(&mut self) -> &OpcrRelationships {
      // java return this.mRelses;
      return &self.super_.mRelses;
    }

    // java public OxfrAppPart getAppPart()
    pub fn getAppPart(&mut self) -> Option<&mut OxfrAppPart> {
      // java for(OpcrRelationship rels : this.mRelses)
      let want = APP.getRelsType().expect("NullPointerException");
      let n = self.super_.mRelses.size();
      let mut found = None;
      for i in 0..n {
        let rels = self.super_.mRelses.getRelationship(i);
        // java if (OxfRelations.APP.getRelsType().equals(rels.getType()))
        if rels.getType().as_deref() == Some(want.as_str()) {
          // java return (OxfrAppPart)rels.getPackagePart();
          found = Some(i);
          break;
        }
      }
      // java return null;
      return found.and_then(|i| self.super_.mRelses.getRelationship_mut(i).getPackagePart_object()).map(|p|match p{PackagePartObject::App(p)=>p,_=>panic!("ClassCastException: OxfrAppPart")});
    }

    // java public OxfrCorePart getCorePart()
    pub fn getCorePart(&mut self) -> Option<&mut OxfrCorePart> {
      // java for(OpcrRelationship rels : this.mRelses)
      let want = CORE.getRelsType().expect("NullPointerException");
      let n = self.super_.mRelses.size();
      let mut found = None;
      for i in 0..n {
        let rels = self.super_.mRelses.getRelationship(i);
        // java if (rels.getType().equals(OxfRelations.CORE.getRelsType()))
        if rels.getType().expect("NullPointerException") == want {
          // java return (OxfrCorePart)rels.getPackagePart();
          found = Some(i);
          break;
        }
      }
      // java return null;
      return found.and_then(|i| self.super_.mRelses.getRelationship_mut(i).getPackagePart_object()).map(|p|match p{PackagePartObject::Core(p)=>p,_=>panic!("ClassCastException: OxfrCorePart")});
    }

    // java public OxfrCustomPart getCustomPart()
    pub fn getCustomPart(&mut self) -> Option<&mut OxfrCustomPart> {
      // java for(OpcrRelationship rels : this.mRelses)
      let want = CUSTOM.getRelsType().expect("NullPointerException");
      let n = self.super_.mRelses.size();
      let mut found = None;
      for i in 0..n {
        let rels = self.super_.mRelses.getRelationship(i);
        // java if (rels.getType().equals(OxfRelations.CUSTOM.getRelsType()))
        if rels.getType().expect("NullPointerException") == want {
          // java return (OxfrCustomPart)rels.getPackagePart();
          found = Some(i);
          break;
        }
      }
      // java return null;
      return found.and_then(|i| self.super_.mRelses.getRelationship_mut(i).getPackagePart_object()).map(|p|match p{PackagePartObject::Custom(p)=>p,_=>panic!("ClassCastException: OxfrCustomPart")});
    }

    // java public int getMediasSize()
    pub fn getMediasSize(&self) -> i32 {
      // java int ret = 0;
      let mut ret: i32 = 0;
      // java Enumeration<? extends ZipEntry> entrys = this.mZipFile.entries();
      // ★Java 的 `this.mZipFile` 由构造器保证非 null；Rust 侧该字段经基类为 `Option<ZipFile>` ⇒
      //   以 `match` 显式承接「None」，行为差异（Java NPE ⇄ Rust 空表 ⇒ ret=0）已登记。
      let entrys: Vec<ZipEntry> = match self.mZipFile.as_ref() {
        Some(zf) => zf.entries(),
        None => Vec::new(),
      };
      // java while(entrys.hasMoreElements())
      // rust: no-stmt（`Enumeration` 循环由 `for` 承担）
      for entry in entrys.iter() {
        // java ZipEntry entry = (ZipEntry)entrys.nextElement();
        // rust: no-stmt（Enumeration 游标由 `for` 承担）
        // java String entryName = entry.getName();
        let entryName: String = entry.getName();
        // java checkEntryNameLegal(entryName);
        OpcrPackage::checkEntryNameLegal(&entryName);
        // java if (entryName.contains("/media/"))
        if entryName.contains("/media/") {
          // java ret = (int)((long)ret + entry.getCompressedSize());
          ret = ((ret as i64) + entry.getCompressedSize()) as i32;
        }
      }
      // java return ret;
      ret
    }

    pub fn close(&mut self) -> Result<(), std::io::Error> {
        self.super_.close()
    }

    pub fn getInputStream_part(
        part: &OpcrPackagePart,
        isOriginInput: bool,
    ) -> Result<Box<dyn Read>, std::io::Error> {
        part.getInputStream(isOriginInput)
    }

    pub fn checkEntryNameLegal(entryName: &str) {
        OpcrPackage::checkEntryNameLegal(entryName);
    }

    pub fn getInputStream(zipFile: &ZipFile, entry: &ZipEntry, isOriginInput: bool) -> Result<Option<Vec<u8>>, std::io::Error> {
        OpcrPackage::getInputStream__entry_flag(zipFile, entry, isOriginInput)
    }

    pub fn getInputStream__name(zipFile: &ZipFile, entryName: &str) -> Result<Option<Vec<u8>>, std::io::Error> {
        OpcrPackage::getInputStream__name(zipFile, entryName)
    }

    pub fn getEntry(zipFile: &ZipFile, entryName: Option<&str>) -> Option<ZipEntry> {
        OpcrPackage::getEntry(zipFile, entryName)
    }
}

impl std::ops::Deref for OxfrPackage {
    type Target = OpcrPackage;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for OxfrPackage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
