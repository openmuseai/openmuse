#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::cell::RefCell;
use std::io::{Cursor, Read};
use std::rc::Rc;

// Java: import cn.wps.kfc.ooxfr.OxfPptRelations
use kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfPptRelations::OxfPptRelations;
// Java: import cn.wps.kfc.ooxfr.OxfrPackage
use crate::cn::wps::kfc::ooxfr::OxfrPackage::OxfrPackage as RealOxfrPackage;
use kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfrPackage::OxfrPackage;
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use kfc_kfc_alpha::cn::wps::kfc::opcr::OpcrPackage::OpcrPackage;
use kfc_kfc_alpha::cn::wps::kfc::opcr::OpcrPackagePart::OpcrPackagePart;
// Java: import cn.wps.kso.filter.KsoFilterPasswdProvider
// rust: `cn.wps.kso.filter` 在 default feature 下不可达，端口省略（与 `PptxrPresentation` 同一登记）。
// Java: import cn.wps.moffice.crash.FileDamagedException
// Java: import cn.wps.moffice.crash.SuffixErrorException
// Java: import cn.wps.moffice.io.IDLProvider
use crate::cn::wps::show::pptxr::ppt::slides::PptxrSlides::IDLProvider;
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.show.PptxReadWriteLock
// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::KmoPresentation as RealKmoPresentation;
// Java: import cn.wps.show.app.event.KmoPresOpenEventListener
use crate::cn::wps::show::app::event::KmoPresOpenEventListener::KmoPresOpenEventListener;
// Java: import cn.wps.show.app.io.PresentationReader
// Java: import cn.wps.show.data.Presentation
// Java: import cn.wps.show.data.docproperty.AppProperty
// Java: import cn.wps.show.data.docproperty.CoreProperty
// Java: import cn.wps.show.pptxr.docprops.PptxrCustom
// Java: import cn.wps.show.pptxr.docprops.PptxrDocProps
// Java: import cn.wps.show.pptxr.ppt.PptxrPresentation
use crate::cn::wps::show::pptxr::ppt::PptxrPresentation::PptxrPresentation;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedia
use crate::cn::wps::show::pptxr::ppt::media::PptxrMedia::PptxrMedia;
// Java: import cn.wps.show.pptxr.util.PptxrFileDecrypt
// Java: import cn.wps.show.pptxw.ppt.ole.PptxOleManager
// Java: import java.io.IOException
use crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrPackagePartRef;

// java public interface PresentationReader

pub use super::pptx_mvp::{
    attr_id, element_id, open_pptx_bytes, open_pptx_document_bytes, open_pptx_document_path,
    open_pptx_path, parse_placeholder_from_shape_xml, parse_theme_scheme_table,
    pic_blip_embed, resolve_relationship_target, slide_image_part_from_rels,
    slide_layout_part_from_rels, walk_xml, PptxDocument,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresPartError {
    FileDamaged(&'static str),
    SuffixError(&'static str),
}

// java public final class PptxReader implements PresentationReader
pub struct PptxReader {
    // java private KmoPresentation mPres;
    mPres: Option<Rc<RefCell<RealKmoPresentation>>>,
    // java private String mFilePath;
    mFilePath: Option<String>,
    // java private RandomAccessFile mRaf;
    // rust: Java 的 `RandomAccessFile` 在这里是包字节源。端口保存这份字节，交给真实 `OxfrPackage(raf)`。
    mRaf: Option<Vec<u8>>,
    // java private boolean bEncrypted;
    bEncrypted: bool,
    // java private KmoPresOpenEventListener mOpenListener;
    mOpenListener: Option<Rc<RefCell<dyn KmoPresOpenEventListener>>>,
    // java private KsoFilterPasswdProvider mPassProvider;
    // rust: 见上方 import 说明。
    // java private IDLProvider mDlProvider;
    mDlProvider: Option<IDLProvider>,
    // java private OxfrPackage mPackage = null;
    pub mPackage: Option<OxfrPackage>,
    // java private PptxrPresentation mPresReader = null;
    mPresReader: Option<PptxrPresentation>,
}

// java public final class PptxReader implements PresentationReader
impl PptxReader {
    // java public void open()
    pub fn open(&mut self) -> Result<(), PresPartError> {
        // java (PptxReadWriteLock.sLock)
        // rust: 进程内无 Java 监视器；装配本身不依赖这把锁的互斥。
        // java this.mPres.setFileFormat(32);
        let pres = self
            .mPres
            .clone()
            .unwrap_or_else(|| Rc::new(RefCell::new(RealKmoPresentation::default())));
        self.mPres = Some(pres.clone());
        pres.borrow_mut().file_format = 32;
        // java String oriPath = this.mFilePath;
        let oriPath: String = self.mFilePath.clone().unwrap_or_default();
        // java if (this.bEncrypted)
        if self.bEncrypted {
            // java this.mFilePath = PptxrFileDecrypt.decryptFile(this.mFilePath, this.mRaf, this.mPres, this.mPassProvider);
            // java this.mRaf = null;
            /* DEFERRED_NON_FUNCTION: `PptxrFileDecrypt` 不在未加密产品路径上。
             * C1/C2 与管线测试都不走加密分支。 */
            let _ = oriPath;
            todo!("DEFERRED_NON_FUNCTION");
        }
        // java if (null != this.mRaf) this.mPackage = new OxfrPackage(this.mRaf); else this.mPackage = new OxfrPackage(this.mFilePath);
        let mut pack = if let Some(bytes) = self.mRaf.clone() {
            let mut cursor = Cursor::new(bytes);
            match OxfrPackage::OxfrPackage__raf(&mut cursor) {
                Ok(p) => p,
                Err(_) => {
                    // java throw new FileDamagedException(t);
                    return Err(PresPartError::FileDamaged("Not OOXML file"));
                }
            }
        } else {
            let path = self.mFilePath.clone().unwrap_or_default();
            match OxfrPackage::OxfrPackage(&path) {
                Ok(p) => p,
                Err(_) => {
                    // java throw new FileDamagedException(t);
                    return Err(PresPartError::FileDamaged("Not OOXML file"));
                }
            }
        };
        let body = self.read_package(&mut pack, pres);
        self.mPackage = Some(pack);
        // java this.close();
        self.close();
        body
    }

    fn read_package(
        &mut self,
        pack: &mut OxfrPackage,
        pres: Rc<RefCell<RealKmoPresentation>>,
    ) -> Result<(), PresPartError> {
        // java String pwd = this.bEncrypted ? this.mPres.cryptInfo().openPassword() : "";
        let pwd: String = String::new();
        // java this.mPres.getOleManager().install(new PptxOleManager(oriPath, this.mRaf, pwd, this.mPres.mediaLib()));
        /* DEFERRED_NON_FUNCTION: `PptxOleManager` 仍是会 panic 的自动桩。
         * OLE 安装不参与 presentation / slide 装配，不能停在这条产品路径上。 */
        let _ = pwd;
        // java PptxrMedia media = new PptxrMedia(this.mPackage, 0);
        let media = self.real_media();
        // java OxfPptRelations.setPptRelation();
        OxfPptRelations::setPptRelation();
        // java Presentation presentationData = Presentation.open(this.mPres.protocol());
        // java AppProperty appProperty = presentationData.docProperty().appProperty();
        // java PptxrDocProps.readApp(appProperty, this.mPackage.getAppPart());
        // java this.mPres.updateProtocol(presentationData.save());
        // rust: kfc `OxfrAppPart.parse` 仍是 `todo!()`。这里读真实 part 字节，写入 Java `readApp` 拷贝的那四个字段。
        if let Some(part) = pack.getAppPart() {
            if let Some(xml) = part_xml(part) {
                let mut model = pres.borrow_mut();
                model.app_application = xml_text(&xml, "Application");
                model.app_version = xml_text(&xml, "AppVersion");
                model.app_company = xml_text(&xml, "Company");
                model.app_manager = xml_text(&xml, "Manager");
            }
        }
        // java OpcrPackagePart presPart = this.getPresPart(this.mPackage);
        let pres_part = {
            let part = PptxReader::getPresPart(pack)?;
            OpcrPackagePartRef::of(part).map_err(|_| PresPartError::FileDamaged("Not OOXML file"))?
        };
        // java this.mPresReader = new PptxrPresentation(this.mPres, presPart, this.mOpenListener, this.mPassProvider, this.mDlProvider, media, 0);
        self.mPresReader = Some(PptxrPresentation::PptxrPresentation(
            pres.clone(),
            Some(pres_part),
            self.mOpenListener.clone(),
            self.mDlProvider.clone(),
            media,
            0,
        ));
        // java this.mPresReader.open();
        if let Some(r) = self.mPresReader.as_mut() {
            r.open()
                .map_err(|_| PresPartError::FileDamaged("Not OOXML file"))?;
        }
        // java presentationData = Presentation.open(this.mPres.protocol());
        // java CoreProperty coreProperty = presentationData.docProperty().coreProperty();
        // java PptxrDocProps.readCore(coreProperty, this.mPackage.getCorePart());
        // rust: kfc `OxfrCorePart.parse` 仍是 `todo!()`。这里读真实 part 字节，写入 Java `readCore` 拷贝的字段。
        if let Some(part) = pack.getCorePart() {
            if let Some(xml) = part_xml(part) {
                let mut model = pres.borrow_mut();
                model.core_title = xml_text(&xml, "title");
                model.core_creator = xml_text(&xml, "creator");
                model.core_last_modified_by = xml_text(&xml, "lastModifiedBy");
                model.core_created = xml_text(&xml, "created");
                model.core_modified = xml_text(&xml, "modified");
            }
        }
        // java PptxrCustom customReader = new PptxrCustom(presentationData.docProperty().customProperties(), this.mPackage.getCustomPart());
        // java customReader.open();
        // rust: `PptxrCustom` 仍是会 panic 的自动桩。有真实 custom part 时直接读 `property name`。
        if let Some(part) = pack.getCustomPart() {
            if let Some(xml) = part_xml(part) {
                pres.borrow_mut().custom_property_names = custom_names(&xml);
            }
        }
        // java this.mPres.updateProtocol(presentationData.save());
        // java this.mOpenListener.docDidFinishReadingEverything();
        if let Some(listener) = self.mOpenListener.clone() {
            listener.borrow_mut().docDidFinishReadingEverything();
        }
        // java this.mPres.setReadOnly(false);
        pres.borrow_mut().read_only = false;
        Ok(())
    }

    fn real_media(&self) -> PptxrMedia {
        if let Some(bytes) = self.mRaf.clone() {
            let mut cursor = Cursor::new(bytes);
            let pack = OpcrPackage::OpcrPackage__raf(&mut cursor).ok();
            return PptxrMedia::PptxrMedia(pack, 0);
        }
        let path = self.mFilePath.clone().unwrap_or_default();
        let pack = OpcrPackage::OpcrPackage(&path).ok();
        PptxrMedia::PptxrMedia(pack, 0)
    }

    /// 产品链读取 `open()` 写完的真实模型。调用方传入的就是这一份。
    pub fn assembled_pres(&self) -> Option<Rc<RefCell<RealKmoPresentation>>> {
        self.mPres.clone()
    }

    // java public boolean isLargestFile()
    pub fn isLargestFile(&self) -> bool {
      // java return false;
      return false;
    }

    // java public void currentSlide(int slideIndex)
    pub fn currentSlide(&mut self, slideIndex: i32) {
    let _ = slideIndex;
    }

    // java public void close()
    pub fn close(&mut self) {
      // java if (this.mPresReader != null)
      if self.mPresReader.is_some() {
        // java this.mPresReader = null;
        self.mPresReader = None;
      }
      // java if (this.mPackage != null)
      if let Some(mPackage) = self.mPackage.as_mut() {
            // java this.mPackage.close();
            if let Err(_e) = mPackage.close() {
            // java IOException e
              // java e.printStackTrace();
              let _ = _e;
            }
        // java this.mPackage = null;
      }
      self.mPackage = None;
      // java if (this.bEncrypted)
      if self.bEncrypted {
        // java File tmpFile = new File(this.mFilePath);
        if let Some(p) = self.mFilePath.as_ref() {
          let tmpFile = std::path::Path::new(p);
        // java if (tmpFile.exists())
        if tmpFile.exists() {
          // java tmpFile.delete();
          let _ = std::fs::remove_file(tmpFile);
        }
        }
      }
    }

    // java public  PptxReader(KmoPresentation pres, String path, KmoPresOpenEventListener openListener, KsoFilterPasswdProvider passProvider, boolean encrypted, IDLProvider dlProvider)
    pub fn PptxReader(
        pres: Option<Rc<RefCell<RealKmoPresentation>>>,
        path: &str,
        openListener: Option<Rc<RefCell<dyn KmoPresOpenEventListener>>>,
        encrypted: bool,
        dlProvider: Option<IDLProvider>,
    ) -> Self {
      // java this.mFilePath = path;
      let mut this = Self {
          mPres: pres.clone(),
          mFilePath: Some(path.to_string()),
          mRaf: None,
          bEncrypted: false,
          mOpenListener: openListener.clone(),
          mDlProvider: dlProvider.clone(),
          mPackage: None,
          mPresReader: None,
      };
      // java this.init(pres, openListener, passProvider, encrypted, dlProvider);
      this.init(pres, openListener, encrypted, dlProvider);
      this
    }

    // java public  PptxReader(KmoPresentation pres, RandomAccessFile randomAccessFile, KmoPresOpenEventListener openListener, KsoFilterPasswdProvider passProvider, boolean encrypted, IDLProvider dlProvider)
    pub fn PptxReader__raf(
        pres: Option<Rc<RefCell<RealKmoPresentation>>>,
        randomAccessFile: Option<Vec<u8>>,
        openListener: Option<Rc<RefCell<dyn KmoPresOpenEventListener>>>,
        encrypted: bool,
        dlProvider: Option<IDLProvider>,
    ) -> Self {
      // java this.mRaf = randomAccessFile;
      let mut this = Self {
          mPres: pres.clone(),
          mFilePath: None,
          mRaf: randomAccessFile,
          bEncrypted: false,
          mOpenListener: openListener.clone(),
          mDlProvider: dlProvider.clone(),
          mPackage: None,
          mPresReader: None,
      };
      // java this.init(pres, openListener, passProvider, encrypted, dlProvider);
      this.init(pres, openListener, encrypted, dlProvider);
      this
    }

    // java private void init(KmoPresentation pres, KmoPresOpenEventListener openListener, KsoFilterPasswdProvider passProvider, boolean encrypted, IDLProvider dlProvider)
    pub fn init(
        &mut self,
        pres: Option<Rc<RefCell<RealKmoPresentation>>>,
        openListener: Option<Rc<RefCell<dyn KmoPresOpenEventListener>>>,
        encrypted: bool,
        dlProvider: Option<IDLProvider>,
    ) {
      // java this.mPres = pres;
      self.mPres = pres;
      // java this.bEncrypted = encrypted;
      self.bEncrypted = encrypted;
      // java this.mOpenListener = openListener;
      self.mOpenListener = openListener;
      // java this.mPassProvider = passProvider;
      // java this.mDlProvider = dlProvider;
      self.mDlProvider = dlProvider;
    }

    // java private OpcrPackagePart getPresPart(OxfrPackage pack)
    pub fn getPresPart(pack: &mut RealOxfrPackage) -> Result<&OpcrPackagePart, PresPartError> {
      // java OpcrPackagePart part = pack.getOfficeDocument();
      let part = pack.getOfficeDocument();
      // java if (part == null)
      if part.is_none() {
        // java throw new FileDamagedException("Not OOXML file");
        return Err(PresPartError::FileDamaged("Not OOXML file"));
      }
      let part = part.unwrap();
      // java else if (!part.getPartName().contains("ppt"))
      if !part.getPartName().contains("ppt") {
        // java throw new SuffixErrorException("Not pptx file");
        return Err(PresPartError::SuffixError("Not pptx file"));
      }
      // java else
      else {
        // java return part;
        return Ok(part);
      }
    }

}

pub fn getPresPart(
    pack: &mut RealOxfrPackage,
) -> Result<&OpcrPackagePart, PresPartError> {
    PptxReader::getPresPart(pack)
}

fn part_xml(part: &OpcrPackagePart) -> Option<String> {
    let mut stream = part.getInputStream_default().ok()?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).ok()?;
    String::from_utf8(buf).ok()
}

/// 取元素本地名的文本。空元素返回 `None`，对应 Java getter 的 null。
fn xml_text(xml: &str, local: &str) -> Option<String> {
    let patterns = [format!("<{local}"), format!(":{local}")];
    let mut name_at = None;
    for pattern in &patterns {
        if let Some(i) = xml.find(pattern) {
            let after = i + pattern.len();
            match xml.as_bytes().get(after).copied() {
                Some(b'>') | Some(b' ') | Some(b'/') | Some(b'\t') | Some(b'\n') => {
                    name_at = Some(after);
                    break;
                }
                _ => {}
            }
        }
    }
    let after_name = name_at?;
    let rest = &xml[after_name..];
    let gt = rest.find('>')?;
    if rest.as_bytes().get(gt.wrapping_sub(1)) == Some(&b'/') {
        return None;
    }
    let rest = &rest[gt + 1..];
    let end = rest.find('<')?;
    let text = rest[..end].trim();
    if text.is_empty() {
        None
    } else {
        Some(text.to_string())
    }
}

fn custom_names(xml: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = xml;
    while let Some(i) = rest.find("<property ") {
        rest = &rest[i + 10..];
        let Some(n) = rest.find("name=\"") else { break };
        rest = &rest[n + 6..];
        let Some(end) = rest.find('"') else { break };
        names.push(rest[..end].to_string());
        rest = &rest[end + 1..];
    }
    names
}
