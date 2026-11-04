// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.settings.StylePaneFormatFilterImporter
//
// ★M-STAGE1 P2 —— 新建类型。全仓此前缺失（0 hit），是 `SettingsImporter` 依赖闭包
//   中的一环（`setStylePaneFormatFilter` / `crrectStylePaneFormatFilter` 的转发目标）。
//
// Java 原文 75 行 / `final class`。依赖闭包已就位。
//
// ★`convertStylePaneFormatFilter` 的 **15 个位掩码**（1/2/4/8/32/64/128/256/512/1024/
//   2048/4096/8192/16384/32768）★注意**跳过 16** —— Java 原文如此，由测试锁定。
// ★`crrectStylePaneFormatFilter` 的默认值 **20516** 亦由测试锁定。

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ShareConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::*;
// Java: import org.xml.sax.Attributes
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

// java final class StylePaneFormatFilterImporter
pub struct StylePaneFormatFilterImporter;

impl StylePaneFormatFilterImporter {
    // java static void setStylePaneFormatFilter(Attributes attributes, MutablePropertySet dst)
    pub fn setStylePaneFormatFilter(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("attributes should not be null", attributes);
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Integer hexVal = ShareConvertHelper.getInteger4STShortHexNumber(attributes);
        let mut hexVal: Option<i32> = ShareConvertHelper::getInteger4STShortHexNumber(attributes);
        // java if (null == hexVal)
        if hexVal.is_none() {
            // java hexVal = convertStylePaneFormatFilter(attributes);
            hexVal = Some(Self::convertStylePaneFormatFilter(attributes));
        }
        // java dst.setInt(437, hexVal);
        dst.setInt(437, hexVal.unwrap());
    }

    // java private static int convertStylePaneFormatFilter(Attributes attributes)
    pub fn convertStylePaneFormatFilter(attributes: &dyn Attributes) -> i32 {
        // java String allStyles = "allStyles";
        let allStyles = "allStyles";
        let _ = allStyles;
        // java String customStyles = "customStyles";
        let customStyles = "customStyles";
        let _ = customStyles;
        // java String latentStyles = "latentStyles";
        let latentStyles = "latentStyles";
        let _ = latentStyles;
        // java String stylesInUse = "stylesInUse";
        let stylesInUse = "stylesInUse";
        let _ = stylesInUse;
        // java String headingStyles = "headingStyles";
        let headingStyles = "headingStyles";
        let _ = headingStyles;
        // java String numberingStyles = "numberingStyles";
        let numberingStyles = "numberingStyles";
        let _ = numberingStyles;
        // java String tableStyles = "tableStyles";
        let tableStyles = "tableStyles";
        let _ = tableStyles;
        // java String directFormattingOnRuns = "directFormattingOnRuns";
        let directFormattingOnRuns = "directFormattingOnRuns";
        let _ = directFormattingOnRuns;
        // java String directFormattingOnParagraphs = "directFormattingOnParagraphs";
        let directFormattingOnParagraphs = "directFormattingOnParagraphs";
        let _ = directFormattingOnParagraphs;
        // java String directFormattingOnNumbering = "directFormattingOnNumbering";
        let directFormattingOnNumbering = "directFormattingOnNumbering";
        let _ = directFormattingOnNumbering;
        // java String directFormattingOnTables = "directFormattingOnTables";
        let directFormattingOnTables = "directFormattingOnTables";
        let _ = directFormattingOnTables;
        // java String clearFormatting = "clearFormatting";
        let clearFormatting = "clearFormatting";
        let _ = clearFormatting;
        // java String top3HeadingStyles = "top3HeadingStyles";
        let top3HeadingStyles = "top3HeadingStyles";
        let _ = top3HeadingStyles;
        // java String visibleStyles = "visibleStyles";
        let visibleStyles = "visibleStyles";
        let _ = visibleStyles;
        // java String alternateStyleNames = "alternateStyleNames";
        let alternateStyleNames = "alternateStyleNames";
        let _ = alternateStyleNames;
        // java int val = 0;
        let mut val: i32 = 0;
        // java val = setBool2Int(val, 1, attributes, "allStyles");
        val = Self::setBool2Int(val, 1, attributes, "allStyles");
        // java val = setBool2Int(val, 2, attributes, "customStyles");
        val = Self::setBool2Int(val, 2, attributes, "customStyles");
        // java val = setBool2Int(val, 4, attributes, "latentStyles");
        val = Self::setBool2Int(val, 4, attributes, "latentStyles");
        // java val = setBool2Int(val, 8, attributes, "stylesInUse");
        val = Self::setBool2Int(val, 8, attributes, "stylesInUse");
        // java val = setBool2Int(val, 32, attributes, "headingStyles");
        val = Self::setBool2Int(val, 32, attributes, "headingStyles");
        // java val = setBool2Int(val, 64, attributes, "numberingStyles");
        val = Self::setBool2Int(val, 64, attributes, "numberingStyles");
        // java val = setBool2Int(val, 128, attributes, "tableStyles");
        val = Self::setBool2Int(val, 128, attributes, "tableStyles");
        // java val = setBool2Int(val, 256, attributes, "directFormattingOnRuns");
        val = Self::setBool2Int(val, 256, attributes, "directFormattingOnRuns");
        // java val = setBool2Int(val, 512, attributes, "directFormattingOnParagraphs");
        val = Self::setBool2Int(val, 512, attributes, "directFormattingOnParagraphs");
        // java val = setBool2Int(val, 1024, attributes, "directFormattingOnNumbering");
        val = Self::setBool2Int(val, 1024, attributes, "directFormattingOnNumbering");
        // java val = setBool2Int(val, 2048, attributes, "directFormattingOnTables");
        val = Self::setBool2Int(val, 2048, attributes, "directFormattingOnTables");
        // java val = setBool2Int(val, 4096, attributes, "clearFormatting");
        val = Self::setBool2Int(val, 4096, attributes, "clearFormatting");
        // java val = setBool2Int(val, 8192, attributes, "top3HeadingStyles");
        val = Self::setBool2Int(val, 8192, attributes, "top3HeadingStyles");
        // java val = setBool2Int(val, 16384, attributes, "visibleStyles");
        val = Self::setBool2Int(val, 16384, attributes, "visibleStyles");
        // java val = setBool2Int(val, 32768, attributes, "alternateStyleNames");
        val = Self::setBool2Int(val, 32768, attributes, "alternateStyleNames");
        // java return val;
        return val;
    }

    // java private static int setBool2Int(int val, int mask, Attributes attributes, String attrName)
    pub fn setBool2Int(val: i32, mask: i32, attributes: &dyn Attributes, attrName: &str) -> i32 {
        // java Boolean bVal = ConvertHelper.getBoolean(attributes, attrName);
        let mut bVal: Option<bool> = ConvertHelper::getBoolean(attributes, attrName);
        // java if (null == bVal)
        if bVal.is_none() {
            // java bVal = false;
            bVal = Some(false);
        }
        // java return bVal ? val | mask : val & ~mask;
        return if bVal.unwrap() { val | mask } else { val & !mask };
    }

    // java static void crrectStylePaneFormatFilter(MutablePropertySet dst)
    pub fn crrectStylePaneFormatFilter(dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Object stylePaneFormatFilter = dst.getValue(437);
        let stylePaneFormatFilter = dst.getValue(437);
        // java if (null == stylePaneFormatFilter)
        if stylePaneFormatFilter.is_none() {
            // java dst.setInt(437, 20516);
            dst.setInt(437, 20516);
        // java } else if (0 == (Integer)stylePaneFormatFilter)
        } else if dst.getInt(437) == 0 {
            // java dst.remove(437);
            dst.remove(437);
        }
    }
}
