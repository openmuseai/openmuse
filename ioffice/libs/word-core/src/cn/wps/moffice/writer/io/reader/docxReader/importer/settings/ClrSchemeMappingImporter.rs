// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.settings.ClrSchemeMappingImporter
//
// ★M-STAGE1 P2 —— 新建类型。全仓此前缺失（0 hit），是 `SettingsImporter`
//   依赖闭包中的一环（`setClrSchemeMapping` 的唯一转发目标）。
//
// Java 原文 78 行 / `final class`（包级可见，无实例成员）。
// 依赖闭包已全部就位：`IOAssert` / `MutablePropertySet` / `ConvertHelper`。
//
// ★`convertClrSchemeMapping` 的 12 分支映射表**非顺序**
//   （`accent1→4` … `accent6→9`，但 `dark1→0` / `light1→1` / `dark2→2` / `light2→3`）
//   —— 由测试锁定。

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::*;
// Java: import org.xml.sax.Attributes
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

// java final class ClrSchemeMappingImporter
pub struct ClrSchemeMappingImporter;

impl ClrSchemeMappingImporter {
    // java static void setClrSchemeMapping(Attributes attributes, MutablePropertySet dst)
    pub fn setClrSchemeMapping(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("attributes should not be null", attributes);
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // rust: `&dyn`/`&mut` 天然非空 ⇒ 由类型系统保证。
        // java String bg1 = "bg1";
        let bg1 = "bg1";
        let _ = bg1;
        // java String t1 = "t1";
        let t1 = "t1";
        let _ = t1;
        // java String bg2 = "bg2";
        let bg2 = "bg2";
        let _ = bg2;
        // java String t2 = "t2";
        let t2 = "t2";
        let _ = t2;
        // java String accent1 = "accent1";
        let accent1 = "accent1";
        let _ = accent1;
        // java String accent2 = "accent2";
        let accent2 = "accent2";
        let _ = accent2;
        // java String accent3 = "accent3";
        let accent3 = "accent3";
        let _ = accent3;
        // java String accent4 = "accent4";
        let accent4 = "accent4";
        let _ = accent4;
        // java String accent5 = "accent5";
        let accent5 = "accent5";
        let _ = accent5;
        // java String accent6 = "accent6";
        let accent6 = "accent6";
        let _ = accent6;
        // java String hyperlink = "hyperlink";
        let hyperlink = "hyperlink";
        let _ = hyperlink;
        // java String followedHyperlink = "followedHyperlink";
        let followedHyperlink = "followedHyperlink";
        let _ = followedHyperlink;
        // java convertSchemeColor(attributes, dst, "bg1", 549);
        Self::convertSchemeColor(attributes, dst, "bg1", 549);
        // java convertSchemeColor(attributes, dst, "t1", 550);
        Self::convertSchemeColor(attributes, dst, "t1", 550);
        // java convertSchemeColor(attributes, dst, "bg2", 551);
        Self::convertSchemeColor(attributes, dst, "bg2", 551);
        // java convertSchemeColor(attributes, dst, "t2", 552);
        Self::convertSchemeColor(attributes, dst, "t2", 552);
        // java convertSchemeColor(attributes, dst, "accent1", 553);
        Self::convertSchemeColor(attributes, dst, "accent1", 553);
        // java convertSchemeColor(attributes, dst, "accent2", 554);
        Self::convertSchemeColor(attributes, dst, "accent2", 554);
        // java convertSchemeColor(attributes, dst, "accent3", 555);
        Self::convertSchemeColor(attributes, dst, "accent3", 555);
        // java convertSchemeColor(attributes, dst, "accent4", 556);
        Self::convertSchemeColor(attributes, dst, "accent4", 556);
        // java convertSchemeColor(attributes, dst, "accent5", 557);
        Self::convertSchemeColor(attributes, dst, "accent5", 557);
        // java convertSchemeColor(attributes, dst, "accent6", 558);
        Self::convertSchemeColor(attributes, dst, "accent6", 558);
        // java convertSchemeColor(attributes, dst, "hyperlink", 559);
        Self::convertSchemeColor(attributes, dst, "hyperlink", 559);
        // java convertSchemeColor(attributes, dst, "followedHyperlink", 560);
        Self::convertSchemeColor(attributes, dst, "followedHyperlink", 560);
    }

    // java private static void convertSchemeColor(Attributes attr, MutablePropertySet dst, String attrKey, int propId)
    pub fn convertSchemeColor(attr: &dyn Attributes, dst: &mut MutablePropertySet, attrKey: &str, propId: i32) {
        // java String bg1 = ConvertHelper.getString(attr, attrKey);
        let bg1: Option<String> = ConvertHelper::getString__String(attr, attrKey);
        // java if (null != bg1)
        if bg1.is_some() {
            // java dst.setInt(propId, convertClrSchemeMapping(bg1));
            dst.setInt(propId, Self::convertClrSchemeMapping(&bg1.unwrap()));
        }
    }

    // java private static int convertClrSchemeMapping(String color)
    pub fn convertClrSchemeMapping(color: &str) -> i32 {
        // java int coreColor = 4;
        let mut coreColor: i32 = 4;
        // java if ("accent1".equals(color))
        if "accent1" == color {
            // java coreColor = 4;
            coreColor = 4;
        // java } else if ("accent2".equals(color))
        } else if "accent2" == color {
            // java coreColor = 5;
            coreColor = 5;
        // java } else if ("accent3".equals(color))
        } else if "accent3" == color {
            // java coreColor = 6;
            coreColor = 6;
        // java } else if ("accent4".equals(color))
        } else if "accent4" == color {
            // java coreColor = 7;
            coreColor = 7;
        // java } else if ("accent5".equals(color))
        } else if "accent5" == color {
            // java coreColor = 8;
            coreColor = 8;
        // java } else if ("accent6".equals(color))
        } else if "accent6" == color {
            // java coreColor = 9;
            coreColor = 9;
        // java } else if ("dark1".equals(color))
        } else if "dark1" == color {
            // java coreColor = 0;
            coreColor = 0;
        // java } else if ("dark2".equals(color))
        } else if "dark2" == color {
            // java coreColor = 2;
            coreColor = 2;
        // java } else if ("light1".equals(color))
        } else if "light1" == color {
            // java coreColor = 1;
            coreColor = 1;
        // java } else if ("light2".equals(color))
        } else if "light2" == color {
            // java coreColor = 3;
            coreColor = 3;
        // java } else if ("hyperlink".equals(color))
        } else if "hyperlink" == color {
            // java coreColor = 10;
            coreColor = 10;
        // java } else if ("followedHyperlink".equals(color))
        } else if "followedHyperlink" == color {
            // java coreColor = 11;
            coreColor = 11;
        } else {
            // java IOAssert.fail("it should not reach here!");
            IOAssert::fail_msg("it should not reach here!");
        }
        // java return coreColor;
        return coreColor;
    }
}
