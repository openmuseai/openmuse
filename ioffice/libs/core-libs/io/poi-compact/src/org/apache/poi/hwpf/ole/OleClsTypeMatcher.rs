// Java: import cn.wps.moffice.util.StringUtil
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

struct Maps {
    suffix2progid: HashMap<String, String>,
    progid2suffix: HashMap<String, String>,
    progid2cls: HashMap<String, String>,
    cn_progid: HashMap<String, String>,
    support_ole: Vec<String>,
}

fn get_maps() -> &'static Mutex<Maps> {
    static MAPS: OnceLock<Mutex<Maps>> = OnceLock::new();
    MAPS.get_or_init(|| {
        let mut s2p = HashMap::new();
        let mut p2s = HashMap::new();
        let mut p2c = HashMap::new();
        let mut cn = HashMap::new();
        let mut sol = Vec::new();

        p2s.insert("Word.Document.8".into(), "doc".into());
        p2s.insert("PowerPoint.Show.8".into(), "ppt".into());
        p2s.insert("WPP.PPT.6".into(), "ppt".into());
        p2s.insert("Excel.Sheet.8".into(), "xls".into());
        p2s.insert("Word.Document.12".into(), "docx".into());
        p2s.insert("PowerPoint.Show.12".into(), "pptx".into());
        p2s.insert("WPP.PPTX.6".into(), "pptx".into());
        p2s.insert("Excel.Chart.8".into(), "xls".into());
        p2s.insert("Excel.Sheet.12".into(), "xlsx".into());
        p2s.insert("ET.Xlsx.6".into(), "xlsx".into());
        p2s.insert("Excel.SheetBinaryMacroEnabled.12".into(), "xlsb".into());
        p2s.insert("Word.DocumentMacroEnabled.12".into(), "docm".into());
        p2s.insert("PowerPoint.ShowMacroEnabled.12".into(), "pptm".into());
        p2s.insert("WPP.PPTM.6".into(), "pptm".into());
        p2s.insert("Excel.SheetMacroEnabled.12".into(), "xlsm".into());
        p2s.insert("ET.Xlsm.6".into(), "xlsm".into());
        p2s.insert("PowerPoint.Slide.12".into(), "sldx".into());
        p2s.insert("WPP.SLDX.6".into(), "sldx".into());
        p2s.insert("PowerPoint.SlideMacroEnabled.12".into(), "sldm".into());
        p2s.insert("WPP.SLDM.6".into(), "sldm".into());
        p2s.insert("PowerPoint.OpenDocumentPresentation.12".into(), "odp".into());
        p2s.insert("Excel.OpenDocumentSpreadsheet.12".into(), "odf".into());
        p2s.insert("Word.OpenDocumentText.12".into(), "odt".into());
        p2s.insert("WPS.Doc.6".into(), "doc".into());
        p2s.insert("WPS.Docx.6".into(), "docx".into());
        p2s.insert("WPS.Docm.6".into(), "docm".into());
        p2s.insert("ET.Xls.6".into(), "xls".into());
        p2s.insert("Worksheet".into(), "xlsx".into());
        p2s.insert("Document".into(), "doc".into());
        p2s.insert("Presentation".into(), "pptx".into());
        p2s.insert("\u{6587}\u{6863}".into(), "doc".into());
        p2s.insert("\u{542F}\u{7528}\u{4E86}\u{5B8F}\u{7684}\u{6A21}\u{677F}".into(), "docm".into());
        p2s.insert("\u{5DE5}\u{4F5C}\u{8868}".into(), "xlsx".into());
        p2s.insert("\u{542F}\u{7528}\u{4E86}\u{5B8F}\u{7684}\u{5DE5}\u{4F5C}\u{8868}".into(), "xlsm".into());
        p2s.insert("\u{6F14}\u{793A}\u{6587}\u{7A3F}".into(), "pptx".into());
        p2s.insert("KPDF.Document.9".into(), "pdf".into());

        s2p.insert("doc".into(), "Word.Document.8".into());
        s2p.insert("ppt".into(), "PowerPoint.Show.8".into());
        s2p.insert("xls".into(), "Excel.Sheet.8".into());
        s2p.insert("docx".into(), "Word.Document.12".into());
        s2p.insert("pptx".into(), "PowerPoint.Show.12".into());
        s2p.insert("xlsx".into(), "Excel.Sheet.12".into());
        s2p.insert("xlsb".into(), "Excel.SheetBinaryMacroEnabled.12".into());
        s2p.insert("docm".into(), "Word.DocumentMacroEnabled.12".into());
        s2p.insert("pptm".into(), "PowerPoint.ShowMacroEnabled.12".into());
        s2p.insert("xlsm".into(), "Excel.SheetMacroEnabled.12".into());
        s2p.insert("sldx".into(), "PowerPoint.Slide.12".into());
        s2p.insert("sldm".into(), "PowerPoint.SlideMacroEnabled.12".into());
        s2p.insert("odp".into(), "PowerPoint.OpenDocumentPresentation.12".into());
        s2p.insert("odf".into(), "Excel.OpenDocumentSpreadsheet.12".into());
        s2p.insert("odt".into(), "Word.OpenDocumentText.12".into());
        s2p.insert("et".into(), "Excel.Sheet.8".into());
        s2p.insert("wps".into(), "Word.Document.8".into());
        s2p.insert("dps".into(), "PowerPoint.Show.8".into());
        s2p.insert("pdf".into(), "KPDF.Document.9".into());

        p2c.insert("Excel.Sheet.12".into(), "{00020830-0000-0000-C000-000000000046}".into());
        p2c.insert("ET.Xlsx.6".into(), "{00020830-0000-0000-C000-000000000046}".into());
        p2c.insert("Word.Document.12".into(), "{F4754C9B-64F5-4B40-8AF4-679732AC0607}".into());
        p2c.insert("Excel.SheetMacroEnabled.12".into(), "{00020832-0000-0000-C000-000000000046}".into());
        p2c.insert("ET.Xlsm.6".into(), "{00020832-0000-0000-C000-000000000046}".into());
        p2c.insert("PowerPoint.ShowMacroEnabled.12".into(), "{DC020317-E6E2-4A62-B9FA-B3EFE16626F4}".into());
        p2c.insert("WPP.PPTM.6".into(), "{DC020317-E6E2-4A62-B9FA-B3EFE16626F4}".into());
        p2c.insert("PowerPoint.Show.8".into(), "{64818D10-4F9B-11CF-86EA-00AA00B929E8}".into());
        p2c.insert("WPP.PPT.6".into(), "{64818D10-4F9B-11CF-86EA-00AA00B929E8}".into());
        p2c.insert("PowerPoint.Show.12".into(), "{CF4F55F4-8F87-4D47-80BB-5808164BB3F8}".into());
        p2c.insert("WPP.PPTX.6".into(), "{CF4F55F4-8F87-4D47-80BB-5808164BB3F8}".into());
        p2c.insert("Word.DocumentMacroEnabled.12".into(), "{18A06B6B-2F3F-4E2B-A611-52BE631B2D22}".into());
        p2c.insert("Excel.Sheet.8".into(), "{00020820-0000-0000-C000-000000000046}".into());
        p2c.insert("Excel.SheetBinaryMacroEnabled.12".into(), "{00020833-0000-0000-C000-000000000046}".into());
        p2c.insert("Excel.Chart.8".into(), "{00020821-0000-0000-C000-000000000046}".into());
        p2c.insert("PowerPoint.Slide.12".into(), "{048EB43E-2059-422F-95E0-557DA96038AF}".into());
        p2c.insert("WPP.SLDX.6".into(), "{048EB43E-2059-422F-95E0-557DA96038AF}".into());
        p2c.insert("Word.Document.8".into(), "{00020906-0000-0000-C000-000000000046}".into());
        p2c.insert("Package".into(), "{0003000C-0000-0000-C000-000000000046}".into());
        p2c.insert("PowerPoint.SlideMacroEnabled.12".into(), "{3C18EAE4-BC25-4134-B7DF-1ECA1337DDDC}".into());
        p2c.insert("WPP.SLDM.6".into(), "{3C18EAE4-BC25-4134-B7DF-1ECA1337DDDC}".into());
        p2c.insert("WPS.Doc.6".into(), "{00020906-0000-0000-C000-000000000046}".into());
        p2c.insert("WPS.Docx.6".into(), "{F4754C9B-64F5-4B40-8AF4-679732AC0607}".into());
        p2c.insert("MSGraph.Chart.8".into(), "{00020803-0000-0000-C000-000000000046}".into());
        p2c.insert("WPS.Docm.6".into(), "{18A06B6B-2F3F-4E2B-A611-52BE631B2D22}".into());
        p2c.insert("PowerPoint.OpenDocumentPresentation.12".into(), "{C282417B-2662-44B8-8A94-3BFF61C50900}".into());
        p2c.insert("Word.OpenDocumentText.12".into(), "{1B261B22-AC6A-4E68-A870-AB5080E8687B}".into());
        p2c.insert("ET.Xls.6".into(), "{00020820-0000-0000-C000-000000000046}".into());
        p2c.insert("Worksheet".into(), "{00020830-0000-0000-C000-000000000046}".into());
        p2c.insert("Document".into(), "{00020906-0000-0000-C000-000000000046}".into());
        p2c.insert("Presentation".into(), "{CF4F55F4-8F87-4D47-80BB-5808164BB3F8}".into());
        p2c.insert("KPDF.Document.9".into(), "{CE015F62-05ED-4CFE-9B40-D5345BDC7B1F}".into());

        sol.push("Word.Document.8".into());
        sol.push("Word.Document.12".into());
        sol.push("PowerPoint.Show.8".into());
        sol.push("PowerPoint.Show.12".into());
        sol.push("Excel.Sheet.8".into());
        sol.push("Excel.Chart.8".into());
        sol.push("Excel.Sheet.12".into());
        sol.push("Word.DocumentMacroEnabled.12".into());
        sol.push("PowerPoint.ShowMacroEnabled.12".into());
        sol.push("Excel.SheetMacroEnabled.12".into());
        sol.push("WPS.Doc.6".into());
        sol.push("WPS.Docx.6".into());
        sol.push("WPS.Docm.6".into());
        sol.push("ET.Xls.6".into());
        sol.push("Document".into());
        sol.push("Presentation".into());
        sol.push("Worksheet".into());
        sol.push("ET.Xlsx.6".into());
        sol.push("ET.Xlsm.6".into());
        sol.push("WPP.PPTM.6".into());
        sol.push("WPP.PPT.6".into());
        sol.push("WPP.PPTX.6".into());
        sol.push("\u{6587}\u{6863}".into());
        sol.push("\u{542F}\u{7528}\u{4E86}\u{5B8F}\u{7684}\u{6A21}\u{677F}".into());
        sol.push("\u{5DE5}\u{4F5C}\u{8868}".into());
        sol.push("\u{542F}\u{7528}\u{4E86}\u{5B8F}\u{7684}\u{5DE5}\u{4F5C}\u{8868}".into());
        sol.push("\u{6F14}\u{793A}\u{6587}\u{7A3F}".into());
        sol.push("\\*MERGEFORMAT".into());

        cn.insert("\u{6587}\u{6863}".into(), "Word.Document.12".into());
        cn.insert("\u{542F}\u{7528}\u{4E86}\u{5B8F}\u{7684}\u{6A21}\u{677F}".into(), "Word.DocumentMacroEnabled.12".into());
        cn.insert("\u{5DE5}\u{4F5C}\u{8868}".into(), "Excel.Sheet.12".into());
        cn.insert("\u{542F}\u{7528}\u{4E86}\u{5B8F}\u{7684}\u{5DE5}\u{4F5C}\u{8868}".into(), "Excel.SheetMacroEnabled.12".into());

        Mutex::new(Maps {
            suffix2progid: s2p,
            progid2suffix: p2s,
            progid2cls: p2c,
            cn_progid: cn,
            support_ole: sol,
        })
    })
}

// Java: public class OleClsTypeMatcher
pub struct OleClsTypeMatcher {
    // Java field: String DEFAULT_OLE_SUFFIX
    // Java field: HashMap<String, String> mSuffix2ProgIdMap
    // Java field: HashMap<String, String> mProgId2SuffixMap
    // Java field: HashMap<String, String> mProgid2ClsMap
    // Java field: HashMap<String, String> mCNProgIdMap
    // Java field: List<String> mSupportOleList
    // Java field: String PROGID_MERGE_PREFIX
}

// Java: public class OleClsTypeMatcher
impl OleClsTypeMatcher {
    // Java: OleClsTypeMatcher()
    pub fn new() -> Self {
        // Java: super();
        OleClsTypeMatcher {}
    }

    // Java: static void extendSupportPdf()
    pub fn extendSupportPdf() {
        let mut maps = get_maps().lock().unwrap();
        // Java: if (!mSupportOleList.contains("Package"))
        if !maps.support_ole.contains(&"Package".to_string()) {
            // Java: mSupportOleList.add("Package");
            maps.support_ole.push("Package".into());
        }
        // Java: if (!mSupportOleList.contains("KPDF.Document.9"))
        if !maps.support_ole.contains(&"KPDF.Document.9".to_string()) {
            // Java: mSupportOleList.add("KPDF.Document.9");
            maps.support_ole.push("KPDF.Document.9".into());
        }
        // Java: if (!mSupportOleList.contains("包装程序外壳对象"))
        if !maps.support_ole.contains(&"\u{5305}\u{88C5}\u{7A0B}\u{5E8F}\u{5916}\u{58F3}\u{5BF9}\u{8C61}".to_string()) {
            // Java: mSupportOleList.add("包装程序外壳对象");
            maps.support_ole.push("\u{5305}\u{88C5}\u{7A0B}\u{5E8F}\u{5916}\u{58F3}\u{5BF9}\u{8C61}".into());
        }
        // Java: if (!mSupportOleList.contains("AcroExch.Document.11"))
        if !maps.support_ole.contains(&"AcroExch.Document.11".to_string()) {
            // Java: mSupportOleList.add("AcroExch.Document.11");
            maps.support_ole.push("AcroExch.Document.11".into());
        }
        // Java: if (!mSupportOleList.contains("AcroExch.Document.15"))
        if !maps.support_ole.contains(&"AcroExch.Document.15".to_string()) {
            // Java: mSupportOleList.add("AcroExch.Document.15");
            maps.support_ole.push("AcroExch.Document.15".into());
        }
        // Java: if (!mSupportOleList.contains("AcroExch.Document.DC"))
        if !maps.support_ole.contains(&"AcroExch.Document.DC".to_string()) {
            // Java: mSupportOleList.add("AcroExch.Document.DC");
            maps.support_ole.push("AcroExch.Document.DC".into());
        }
    }

    // Java: static void removeSupportPdf()
    pub fn removeSupportPdf() {
        let mut maps = get_maps().lock().unwrap();
        // Java: mSupportOleList.remove("Package");
        maps.support_ole.retain(|s| s != "Package");
        // Java: mSupportOleList.remove("KPDF.Document.9");
        maps.support_ole.retain(|s| s != "KPDF.Document.9");
        // Java: mSupportOleList.remove("包装程序外壳对象");
        maps.support_ole.retain(|s| s != "\u{5305}\u{88C5}\u{7A0B}\u{5E8F}\u{5916}\u{58F3}\u{5BF9}\u{8C61}");
        // Java: mSupportOleList.remove("AcroExch.Document.11");
        maps.support_ole.retain(|s| s != "AcroExch.Document.11");
        // Java: mSupportOleList.remove("AcroExch.Document.15");
        maps.support_ole.retain(|s| s != "AcroExch.Document.15");
        // Java: mSupportOleList.remove("AcroExch.Document.DC");
        maps.support_ole.retain(|s| s != "AcroExch.Document.DC");
    }

    // Java: static String getSuffixByProgId(String progId)
    pub fn getSuffixByProgId(progId: &str) -> Option<String> {
        let maps = get_maps().lock().unwrap();
        // Java: if (null == mProgId2SuffixMap)
        // Java: return null;
        // Java: else
        // Java: progId = trimProgId(progId);
        let trimmed = Self::trimProgId(progId);
        // Java: return (String)mProgId2SuffixMap.get(progId);
        maps.progid2suffix.get(&trimmed).cloned()
    }

    // Java: static String getProgIdBySuffix(String suffix)
    pub fn getProgIdBySuffix(suffix: &str) -> Option<String> {
        let maps = get_maps().lock().unwrap();
        // Java: return null == mSuffix2ProgIdMap ? null : (String)mSuffix2ProgIdMap.get(suffix.toLowerCase());
        maps.suffix2progid.get(&suffix.to_lowercase()).cloned()
    }

    // Java: static String getClsidByProgid(String progId)
    pub fn getClsidByProgid(progId: &str) -> Option<String> {
        let maps = get_maps().lock().unwrap();
        // Java: if (null != mProgid2ClsMap && progId != null)
        // Java: progId = trimProgId(progId);
        let trimmed = Self::trimProgId(progId);
        // Java: return (String)mProgid2ClsMap.get(progId);
        maps.progid2cls.get(&trimmed).cloned()
        // Java: else
        // Java: return null;
    }

    // Java: static boolean canOpenOle(String progId)
    pub fn canOpenOle(progId: &str) -> bool {
        let maps = get_maps().lock().unwrap();
        // Java: if (null != progId && null != mSupportOleList && mSupportOleList.size() > 0)
        if !maps.support_ole.is_empty() {
            // Java: progId = trimProgId(progId);
            let trimmed = Self::trimProgId(progId);
            // Java: return mSupportOleList.contains(progId);
            maps.support_ole.contains(&trimmed)
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: static String trimProgId(String progId)
    fn trimProgId(progId: &str) -> String {
        // Java: return !StringUtil.isEmpty(progId) && progId.contains("\\*") && !progId.equals("\\*MERGEFORMAT") ? progId.substring(0, progId.indexOf("\\*")) : progId;
        if !progId.is_empty() && progId.contains("\\*") && progId != "\\*MERGEFORMAT" {
            if let Some(idx) = progId.find("\\*") {
                return progId[..idx].to_string();
            }
        }
        progId.to_string()
    }

    // Java: static String convertProgId(String oriProgId, String suffix)
    pub fn convertProgId(oriProgId: &str, suffix: Option<&str>) -> String {
        // Java: String result = oriProgId;
        let maps = get_maps().lock().unwrap();
        // Java: if (mCNProgIdMap != null && mCNProgIdMap.get(oriProgId) != null)
        if let Some(mapped) = maps.cn_progid.get(oriProgId) {
            // Java: result = (String)mCNProgIdMap.get(oriProgId);
            return mapped.clone();
        }
        // Java: else if (oriProgId.equals("演示文稿") && null != suffix)
        else if oriProgId == "\u{6F14}\u{793A}\u{6587}\u{7A3F}" && suffix.is_some() {
            let s = suffix.unwrap();
            // Java: if (suffix.toLowerCase().endsWith("ppt"))
            if s.to_lowercase().ends_with("ppt") {
                // Java: return "PowerPoint.Show.8";
                return "PowerPoint.Show.8".to_string();
            }
            // Java: if (suffix.toLowerCase().endsWith("pptx"))
            if s.to_lowercase().ends_with("pptx") {
                // Java: return "PowerPoint.Show.12";
                return "PowerPoint.Show.12".to_string();
            }
            // Java: if (suffix.toLowerCase().endsWith("pptm"))
            if s.to_lowercase().ends_with("pptm") {
                // Java: return "PowerPoint.ShowMacroEnabled.12";
                return "PowerPoint.ShowMacroEnabled.12".to_string();
            }
        }
        // Java: return result;
        oriProgId.to_string()
    }
}
