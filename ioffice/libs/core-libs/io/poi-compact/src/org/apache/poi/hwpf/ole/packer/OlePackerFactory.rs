// Java: import org.apache.poi.hwpf.util.StorageUtil
use crate::org::apache::poi::hwpf::util::StorageUtil::StorageUtil;
use super::IOlePacker::IOlePacker;

pub enum OlePackerVariant {
    Compound(String),
    Common(String),
    AudioComment(String),
    Package(String),
    VBA(String, String),
}

// Java: public class OlePackerFactory
pub struct OlePackerFactory {
}

// Java: public class OlePackerFactory
impl OlePackerFactory {
    // Java: OlePackerFactory()
    pub fn new() -> Self {
      // Java: super();
      OlePackerFactory {}
    }

    // Java: static IOlePacker getPacker(String path)
    pub fn getPacker(path: &str) -> OlePackerVariant {
      // Java: return (IOlePacker)(StorageUtil.isCoumpondFile(path) ? new CompoundOlePacker(path) : new CommonOlePacker(path));
      if StorageUtil::isCoumpondFile(path) {
          OlePackerVariant::Compound(path.to_string())
      } else {
          OlePackerVariant::Common(path.to_string())
      }
    }

    // Java: static IOlePacker getPacker(String progId, String path, boolean isVba)
    pub fn getPacker_with_prog_id(prog_id: Option<&str>, path: Option<&str>, is_vba: bool) -> Option<OlePackerVariant> {
      // Java: if (null == path)
      if path.is_none() {
         // Java: return null;
         return None;
      }
      let path = path.unwrap();
      // Java: else if (null == progId)
      if prog_id.is_none() {
         // Java: return new CompoundOlePacker(path);
         return Some(OlePackerVariant::Compound(path.to_string()));
      }
      let prog_id = prog_id.unwrap();
      // Java: else if ("Package".equals(progId) && !path.toLowerCase().endsWith("bin"))
      if prog_id == "Package" && !path.to_lowercase().ends_with("bin") {
         // Java: return new AudioCommentOlePacker(path);
         return Some(OlePackerVariant::AudioComment(path.to_string()));
      }
      // Java: else if (progId.toLowerCase().endsWith("penkit") && !path.toLowerCase().endsWith("bin"))
      else if prog_id.to_lowercase().ends_with("penkit") && !path.to_lowercase().ends_with("bin") {
         // Java: return new PackageOlePacker(path);
         return Some(OlePackerVariant::Package(path.to_string()));
      }
      // Java: else if (isVba)
      else if is_vba {
         // Java: return new VBAOlePacker(path, progId);
         return Some(OlePackerVariant::VBA(path.to_string(), prog_id.to_string()));
      }
      // Java: else if (StorageUtil.isCompoundFile(path))
      else if StorageUtil::isCompoundFile(path) {
         // Java: return new CompoundOlePacker(path);
         return Some(OlePackerVariant::Compound(path.to_string()));
      }
      // Java: else
      else {
         // Java: return (IOlePacker)(!"Excel.Sheet.12".equalsIgnoreCase(progId) && !"Word.Document.12".equalsIgnoreCase(progId) && !"Excel.SheetMacroEnabled.12".equalsIgnoreCase(progId) && !"PowerPoint.ShowMacroEnabled.12".equalsIgnoreCase(progId) && !"PowerPoint.Slide.12".equalsIgnoreCase(progId) && !"WPP.SLDX.6".equalsIgnoreCase(progId) && !"PowerPoint.Show.12".equalsIgnoreCase(progId) && !"Word.DocumentMacroEnabled.12".equalsIgnoreCase(progId) && !"Excel.SheetBinaryMacroEnabled.12".equalsIgnoreCase(progId) && !"PowerPoint.SlideMacroEnabled.12".equals(progId) && !"WPP.SLDM.6".equals(progId) && !"WPS.Docx.6".equals(progId) && !"WPP.PPTX.6".equalsIgnoreCase(progId) && !"ET.Xlsx.6".equals(progId) && !"WPS.Docm.6".equals(progId) && !"WPP.PPTM.6".equals(progId) && !"ET.Xlsm.6".equals(progId) && !"Presentation".equals(progId) && !"Worksheet".equals(progId) ? new CompoundOlePacker(path) : new CommonOlePacker(path));
         let prog_lower = prog_id.to_lowercase();
         let is_ooxml = prog_lower == "excel.sheet.12"
             || prog_lower == "word.document.12"
             || prog_lower == "excel.sheetmacroenabled.12"
             || prog_lower == "powerpoint.showmacroenabled.12"
             || prog_lower == "powerpoint.slide.12"
             || prog_lower == "wpp.sldx.6"
             || prog_lower == "powerpoint.show.12"
             || prog_lower == "word.documentmacroenabled.12"
             || prog_lower == "excel.sheetbinarymacroenabled.12"
             || prog_id == "PowerPoint.SlideMacroEnabled.12"
             || prog_id == "WPP.SLDM.6"
             || prog_id == "WPS.Docx.6"
             || prog_lower == "wpp.pptx.6"
             || prog_id == "ET.Xlsx.6"
             || prog_id == "WPS.Docm.6"
             || prog_id == "WPP.PPTM.6"
             || prog_id == "ET.Xlsm.6"
             || prog_id == "Presentation"
             || prog_id == "Worksheet";
         if is_ooxml {
             return Some(OlePackerVariant::Common(path.to_string()));
         } else {
             return Some(OlePackerVariant::Compound(path.to_string()));
         }
      }
    }

}
