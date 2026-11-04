// Java: cn.wps.moffice.writer.io.reader.docxReader.importer.settings.SmartTagTypeImporter
//
// ★M-STAGE1 P2 —— 新建类型。全仓此前缺失（0 hit），是 `SettingsImporter` 依赖闭包
//   中的一环（`setSmartTagType` / `correctSmartTagType` 的转发目标）。
//
// Java 原文 49 行 / `final class`。
// ★本轮连带新建其依赖的值类型 `SmartTag`（Java 仅 3 个 public 字段）。
//
// ★`setSmartTagType` 的属性位是 **574**，且为**累积**语义（Java `List.add`）——
//   与 `CapationsImporter.setCaption` / `setDocVar` 同类陷阱，由测试锁定。

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.writer.data.value.SmartTag
use crate::cn::wps::moffice::writer::data::value::SmartTag::SmartTag;
// Java: import cn.wps.moffice.writer.io.reader.docxReader.importer.helpers.ConvertHelper
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::*;
// Java: import java.util.ArrayList
// Java: import java.util.List
// rust: Java `ArrayList`/`List` ⇒ Rust `Vec`（值语义下需显式回写，见下）。
// Java: import org.xml.sax.Attributes
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;

// java final class SmartTagTypeImporter
pub struct SmartTagTypeImporter;

impl SmartTagTypeImporter {
    // java static void setSmartTagType(Attributes attributes, MutablePropertySet dst)
    pub fn setSmartTagType(attributes: &dyn Attributes, dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("attributes should not be null", attributes);
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Object smartTag = dst.getValue(574);
        let smartTag = dst.getValue(574);
        // java if (null == smartTag)
        if smartTag.is_none() {
            // java List<SmartTag> smartTagList = new ArrayList();
            // java dst.setValue(574, smartTagList);
            // java smartTagList.add(convertSmartTageType(attributes));
            // rust: Java 先存空表再 add（引用语义，终点是"表含新元素"）；
            //   Rust 值语义无法先存后改 ⇒ 等价做法为**构建完成后一次写回**。
            let mut smartTagList: Vec<SmartTag> = Vec::new();
            smartTagList.push(Self::convertSmartTageType(attributes));
            dst.setValue(574, Some(std::sync::Arc::new(smartTagList)));
        // java } else if (smartTag instanceof List)
        } else {
            // java List<SmartTag> smartTagList = (List)smartTag;
            // rust: Java 是**引用语义** —— 取出的是同一个 List，add 直接改到 `dst` 里。
            //   Rust 值语义必须先取回已有内容再追加，否则丢失历史项。
            let mut smartTagList: Vec<SmartTag> = dst
                .getValue(574)
                .and_then(|v| v.as_ref().as_any().downcast_ref::<Vec<SmartTag>>().cloned())
                .unwrap_or_default();
            // java smartTagList.add(convertSmartTageType(attributes));
            smartTagList.push(Self::convertSmartTageType(attributes));
            dst.setValue(574, Some(std::sync::Arc::new(smartTagList)));
        }
    }

    // java private static SmartTag convertSmartTageType(Attributes attributes)
    pub fn convertSmartTageType(attributes: &dyn Attributes) -> SmartTag {
        // java SmartTag smartTage = new SmartTag();
        let mut smartTage = SmartTag::default();
        // java String url = "url";
        let url = "url";
        let _ = url;
        // java String name = "name";
        let name = "name";
        let _ = name;
        // java String namespaceuri = "namespaceuri";
        let namespaceuri = "namespaceuri";
        let _ = namespaceuri;
        // java smartTage.mUrl = ConvertHelper.getString(attributes, "url");
        smartTage.mUrl = ConvertHelper::getString__String(attributes, "url");
        // java smartTage.mName = ConvertHelper.getString(attributes, "name");
        smartTage.mName = ConvertHelper::getString__String(attributes, "name");
        // java smartTage.mNamespaceuri = ConvertHelper.getString(attributes, "namespaceuri");
        smartTage.mNamespaceuri = ConvertHelper::getString__String(attributes, "namespaceuri");
        // java return smartTage;
        return smartTage;
    }

    // java static void correctSmartTagType(MutablePropertySet dst)
    pub fn correctSmartTagType(dst: &mut MutablePropertySet) {
        // java IOAssert.assertNotNull("dst should not be null", dst);
        // java Object smartTag = dst.getValue(574);
        let smartTag = dst.getValue(574);
        // java if (null != smartTag && smartTag instanceof List)
        let is_list = smartTag
            .as_ref()
            .map(|v| v.as_ref().as_any().downcast_ref::<Vec<SmartTag>>().is_some())
            .unwrap_or(false);
        if is_list {
            // java List<SmartTag> smartTagList = (List)smartTag;
            // java SmartTag[] smartTages = new SmartTag[smartTagList.size()];
            // java smartTagList.toArray(smartTages);
            // java dst.setValue(574, smartTages);
            // rust: List→数组在 Rust 中为恒等（`Vec` 即数组）⇒ 不重写。
        }
    }
}
