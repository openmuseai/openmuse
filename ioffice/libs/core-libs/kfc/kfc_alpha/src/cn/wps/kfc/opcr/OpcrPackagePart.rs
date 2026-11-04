// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.kfc.ooxfr.OxfrPackage
use super::OpcrPackage::OpcrPackage;
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import cn.wps.moffice.kfs.zip.ZipFile
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use super::OpcrRelationships::OpcrRelationships;
use super::ZipFile::{ZipEntry, ZipFile};
use std::io::{Cursor, Read};

// java private static final String TAG = null;
const TAG: &str = "";

// java public class OpcrPackagePart
#[derive(Default)]
pub struct OpcrPackagePart {
    // java private ZipFile mZipFile = null;
    mZipFile: ZipFile,
    // java private ZipEntry mZipEntry;
    mZipEntry: ZipEntry,
    // java private OpcrRelationships mRelses = null;
    mRelses: Option<OpcrRelationships>,
}

// java public class OpcrPackagePart
impl OpcrPackagePart {
    // java public  OpcrPackagePart(ZipFile zipFile, ZipEntry entry)
    pub fn OpcrPackagePart(zipFile: &ZipFile, entry: &ZipEntry) -> Self {
      // java super();
      // java this.mZipFile = zipFile;
      // java this.mZipEntry = entry;
      Self {
        mZipFile: zipFile.clone(),
        mZipEntry: entry.clone(),
        mRelses: None,
      }
    }

    // java public InputStream getInputStream(boolean isOriginInput)
    pub fn getInputStream(&self, isOriginInput: bool) -> Result<Box<dyn Read>, std::io::Error> {
      // java return OxfrPackage.getInputStream(this.mZipFile, this.mZipEntry, isOriginInput);
      let bytes = OpcrPackage::getInputStream__entry_flag(&self.mZipFile, &self.mZipEntry, isOriginInput)?
        .unwrap_or_default();
      Ok(Box::new(Cursor::new(bytes)))
    }

    // Native nullable return transport of the same Java getInputStream overload.
    // Original property parse skips null; it must not parse an invented empty stream.
    pub fn getInputStream_nullable(&self,isOriginInput:bool)->std::io::Result<Option<Box<dyn Read>>>{
      // java return OxfrPackage.getInputStream(this.mZipFile, this.mZipEntry, isOriginInput);
      Ok(OpcrPackage::getInputStream__entry_flag(&self.mZipFile,&self.mZipEntry,isOriginInput)?.map(|bytes|Box::new(Cursor::new(bytes))as Box<dyn Read>))
    }
    // Native InputStream.available view of the same nullable buffered stream.
    // No second ZIP read or part-size approximation.
    pub fn getInputStream_available(&self,isOriginInput:bool)->std::io::Result<Option<Cursor<Vec<u8>>>>{
      Ok(OpcrPackage::getInputStream__entry_flag(&self.mZipFile,&self.mZipEntry,isOriginInput)?.map(Cursor::new))
    }

    // java public InputStream getInputStream()
    pub fn getInputStream_default(&self) -> Result<Box<dyn Read>, std::io::Error> {
      // java return this.getInputStream(false);
      return self.getInputStream(false);
    }

    // java public ZipEntry getZipEntry()
    pub fn getZipEntry(&self) -> Result<ZipEntry, std::io::Error> {
      // java return this.mZipEntry;
      Ok(self.mZipEntry.clone())
    }

    // java public ZipFile getZipFile()
    pub fn getZipFile(&self) -> ZipFile {
      // java return this.mZipFile;
      return self.mZipFile.clone();
    }

    fn loadRelationships(&mut self) -> Result<(), std::io::Error> {
      if self.mRelses.is_none() {
        let mut entryName = self.mZipEntry.getName();
        OpcrPackage::checkEntryNameLegal(&entryName);
        if entryName.contains('\\') {
          entryName = entryName.replace('\\', "/");
        }
        // java try {this.mRelses=new OpcrRelationships(...);}catch(Throwable e){IOLog.d(TAG,"Throwable",e);}
        if let Ok(Ok(rels))=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||OpcrRelationships::OpcrRelationships(&self.mZipFile,&entryName))){self.mRelses=Some(rels);}
      }
      Ok(())
    }

    // java public OpcrRelationships getRelationships()
    pub fn getRelationships(&mut self) -> Result<Option<&OpcrRelationships>, std::io::Error> {
      // java if (this.mRelses == null)
      // java String entryName = this.mZipEntry.getName();
      // java OxfrPackage.checkEntryNameLegal(entryName);
      // java if (entryName.contains("\\"))
      // java entryName = entryName.replace('\\', '/');
      // java try {
      // java this.mRelses = new OpcrRelationships(this.mZipFile, entryName);
      // java } catch (Throwable e) {
      // java IOLog.d(TAG, "Throwable", e);
      self.loadRelationships()?;
      // java return this.mRelses;
      Ok(self.mRelses.as_ref())
    }

    pub fn getRelationships_mut(&mut self) -> Result<Option<&mut OpcrRelationships>, std::io::Error> {
      self.loadRelationships()?;
      Ok(self.mRelses.as_mut())
    }

    // java public String getPartName()
    pub fn getPartName(&self) -> String {
      // java String entryName = this.mZipEntry.getName();
      let entryName = self.mZipEntry.getName();
      // java OxfrPackage.checkEntryNameLegal(entryName);
      OpcrPackage::checkEntryNameLegal(&entryName);
      // java return entryName;
      return entryName;
    }

    // java public int getPartSize()
    pub fn getPartSize(&self) -> Result<i32, std::io::Error> {
      // java int ret = (int)this.mZipEntry.getSize();
      let ret = self.mZipEntry.getSize() as i32;
      // java if (ret > 0)
      if ret > 0 {
        // java return ret;
        return Ok(ret);
      }
      // java else {
      // java InputStream stream = null;
      let stream: Option<Vec<u8>> = None;
      // java try {
      // java stream = this.getInputStream();
      // java if (stream != null) {
      // java ret = stream.available();
      // java } finally {
      // java if (stream != null) {
      // java stream.close();
      let _ = stream;
      Ok(ret)
    }

}
