// Rust port of cn/wps/moffice/writer/io/reader/docxReader/importer/FontTableImporter.java
//
// ★M-STAGE1 P2-2 —— 本文件原为 auto-generated 桩（`use crate::*;` + 全 `todo!()`，
//   且**门面与 Java 不一致**：无参 `new()`，而 Java 是
//   `FontTableImporter(FontTable, int, Xml07Context, Xml03Context)`）。
//   为纳入编译树（`lib.rs` 内联登记）并按门面纪律对齐，此处：
//     ① 补显式 `use`（`Attributes` / `FontInfo` 不在 crate 根）；
//     ② `Attributes` 由裸 trait 改为 `&dyn Attributes`；
//     ③ 构造函数改为 Java 的 4 参形态，字段按 Java 原文声明。
//
// ★★M-STAGE1 P2-2（本轮）—— **由桩做实**。
//   ★重要发现：原桩里的 `// java` 语句注释**是伪造的**，与 Java 原文完全不符
//   （例如桩写 `fontInfo.setName(attributes.getValue("", "name"))`，
//     而 Java 原文是 `ConvertHelper.getString(attributes, "name")` + null 判断）。
//   ⇒ 本轮**以 Java 原文为准逐语句重写**，不再沿用旧桩注释。

use crate::cn::wps::moffice::writer::core::FontTable::FontTable;
use crate::cn::wps::moffice::writer::data::value::FontInfo::FontInfo;
use crate::cn::wps::moffice::writer::io::reader::ImporterBase::ImporterBase;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::Attributes;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ConvertHelper::ConvertHelper;
use crate::cn::wps::moffice::writer::io::reader::docxReader::importer::helpers::ShareConvertHelper::ShareConvertHelper;
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Context::Xml03Context;
use crate::cn::wps::moffice::writer::io::reader::xml03::Xml03Utils::Xml03Utils;
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;

// java public final class FontTableImporter extends ImporterBase
pub struct FontTableImporter {
    // java private FontTable mFontTable;
    mFontTable: Option<FontTable>,
    // java private FontInfo mFontInfo;
    mFontInfo: Option<FontInfo>,
    // rust: Java `extends ImporterBase` ⇒ 组合基类（与 SettingsImporter 一致的做法）
    base: ImporterBase,
}

impl FontTableImporter {
    // java public FontTableImporter(FontTable fontTable, int mode, Xml07Context xml07Context, Xml03Context xml03Context)
    pub fn FontTableImporter(
        fontTable: FontTable,
        mode: i32,
        xml07Context: Option<Xml07Context>,
        xml03Context: Option<Xml03Context>,
    ) -> Self {
        // java super(mode, xml07Context, xml03Context);
        let base = ImporterBase::ImporterBase(mode, xml07Context, xml03Context);
        // java this.mFontTable = fontTable;
        FontTableImporter {
            mFontTable: Some(fontTable),
            mFontInfo: None,
            base,
        }
    }

    // java public void importFontStart(Attributes attributes)
    pub fn importFontStart(&mut self, attributes: &dyn Attributes) {
        // java IOAssert.assertNotNull("attributes should not be null", attributes);
        // rust: `&dyn` 天然非空 ⇒ 断言语义由类型系统保证。
        // java this.mFontInfo = toFontInfo(attributes);
        self.mFontInfo = Self::toFontInfo(attributes);
    }

    // java public void importFontEnd()
    pub fn importFontEnd(&mut self) {
        // java FontInfo fontInfo = this.mFontInfo;
        let fontInfo = self.mFontInfo.clone();
        // java if (null != fontInfo && null != fontInfo.getFontSignature())
        if fontInfo.is_some() && fontInfo.as_ref().unwrap().getFontSignature().is_some() {
            // java this.correctAlternativeName(fontInfo);
            self.correctAlternativeName(fontInfo.clone().unwrap());
            // java this.mFontTable.add(fontInfo);
            self.mFontTable
                .as_mut()
                .expect("mFontTable")
                .add__fontinfo(fontInfo.as_ref().unwrap());
            // java this.mFontInfo = null;
            self.mFontInfo = None;
        }
    }

    // java private void correctAlternativeName(FontInfo fontInfo)
    pub fn correctAlternativeName(&mut self, mut fontInfo: FontInfo) {
        // java String altName = fontInfo.getAlternativeName();
        let altName = fontInfo.getAlternativeName();
        // java if (null == altName || 0 == altName.length())
        if altName.is_empty() {
            // java fontInfo.setAlternativeName(fontInfo.getFontName());
            let fontName = fontInfo.getFontName();
            fontInfo.setAlternativeName(fontName);
        }
    }

    // java public void importFontChild(int elementId, Attributes attributes)
    pub fn importFontChild(&mut self, elementId: i32, attributes: &dyn Attributes) {
        // java IOAssert.assertNotNull("attributes should not be null", attributes);
        // rust: `&dyn` 天然非空 ⇒ 由类型系统保证。
        // java FontInfo fontInfo = this.mFontInfo;
        let fontInfo = self.mFontInfo.clone();
        // java if (null != fontInfo)
        if fontInfo.is_some() {
            // java this.convertFontAttribute(fontInfo, elementId, attributes);
            self.convertFontAttribute(fontInfo.unwrap(), elementId, attributes);
        }
    }

    // java private static FontInfo toFontInfo(Attributes attributes)
    pub fn toFontInfo(attributes: &dyn Attributes) -> Option<FontInfo> {
        // java String attrName = "name";
        let _attrName = "name";
        // java String fontName = ConvertHelper.getString(attributes, "name");
        let fontName: Option<String> = ConvertHelper::getString__String(attributes, "name");
        // java if (null == fontName)
        if fontName.is_none() {
            // java return null;
            return None;
        // java else
        } else {
            // java FontInfo fontInfo = new FontInfo();
            let mut fontInfo = FontInfo::default();
            // java fontInfo.setFontName(fontName);
            fontInfo.setFontName(fontName.unwrap());
            // java return fontInfo;
            return Some(fontInfo);
        }
    }

    // java private void convertFontAttribute(FontInfo fontInfo, int elementId, Attributes attributes)
    pub fn convertFontAttribute(&mut self, mut fontInfo: FontInfo, elementId: i32, attributes: &dyn Attributes) {
        // java IOAssert.assertNotNull("fontInfo should not be null", fontInfo);
        // java IOAssert.assertNotNull("attributes should not be null", attributes);
        // rust: 二者均为值/引用 ⇒ 天然非空。
        // java switch (elementId)
        match elementId {
            // java case -2031710757:
            -2031710757 => {
                // java setTrueType(attributes, fontInfo);
                Self::setTrueType(attributes, &mut fontInfo);
                // java （★Java 原文此处**无 break** ⇒ 落到下一组 case 的 break）
            }
            // java case -1450986877:
            -1450986877 => {
                // java break;
            }
            // java case -1409488642:
            -1409488642 => {
                // java break;
            }
            // java case -1399181719:
            -1399181719 => {
                // java break;
            }
            // java case 1681905870:
            1681905870 => {
                // java break;
            }
            // java case -1281860764:
            -1281860764 => {
                // java this.setFamily(attributes, fontInfo);
                self.setFamily(attributes, &mut fontInfo);
                // java break;
            }
            // java case -904959020:
            -904959020 => {
                // java setAlternativeName(attributes, fontInfo);
                Self::setAlternativeName(attributes, &mut fontInfo);
                // java break;
            }
            // java case -796767379:
            -796767379 => {
                // java setPanose1(attributes, fontInfo);
                Self::setPanose1(attributes, &mut fontInfo);
                // java break;
            }
            // java case 113873:
            113873 => {
                // java setSignature(attributes, fontInfo);
                Self::setSignature(attributes, &mut fontInfo);
                // java break;
            }
            // java case 106677056:
            106677056 => {
                // java setCharacterPitch(attributes, fontInfo);
                Self::setCharacterPitch(attributes, &mut fontInfo);
                // java break;
            }
            // java case 739074380:
            739074380 => {
                // java setCharacterSet(attributes, fontInfo);
                Self::setCharacterSet(attributes, &mut fontInfo);
                // java break;
            }
            // java case 1070014952:
            1070014952 => {
                // java if (this.isXml03())
                if self.base.isXml03() {
                    // java setPanose1(attributes, fontInfo);
                    Self::setPanose1(attributes, &mut fontInfo);
                }
                // java break;
            }
            // java default:
            _ => {
                // java IOAssert.fail("it should not reach here");
                IOAssert::fail_msg("it should not reach here");
            }
        }
        // rust: Java 以**对象引用**改 `fontInfo`（调用方持有的同一对象）；
        //   Rust 值语义 ⇒ 方法内改完后写回 `self.mFontInfo`，
        //   使 `importFontEnd` 看到与 Java 相同的状态。
        self.mFontInfo = Some(fontInfo);
    }

    // java private static void setAlternativeName(Attributes attributes, FontInfo fontInfo)
    pub fn setAlternativeName(attributes: &dyn Attributes, fontInfo: &mut FontInfo) {
        // java String altName = ConvertHelper.getString(attributes);
        let altName: Option<String> = ConvertHelper::getString(attributes);
        // java altName = null == altName ? fontInfo.getFontName() : altName;
        let altName = match altName {
            None => fontInfo.getFontName(),
            Some(v) => v,
        };
        // java fontInfo.setAlternativeName(altName);
        fontInfo.setAlternativeName(altName);
    }

    // java private static void setPanose1(Attributes attributes, FontInfo fontInfo)
    pub fn setPanose1(attributes: &dyn Attributes, fontInfo: &mut FontInfo) {
        // java String panose1 = ConvertHelper.getString(attributes);
        let panose1: Option<String> = ConvertHelper::getString(attributes);
        // java if (null != panose1)
        if panose1.is_some() {
            // java ConvertHelper.setPanose(panose1, fontInfo);
            ConvertHelper::setPanose(panose1.as_ref().unwrap(), fontInfo);
        }
    }

    // java private static void setCharacterSet(Attributes attributes, FontInfo fontInfo)
    pub fn setCharacterSet(attributes: &dyn Attributes, fontInfo: &mut FontInfo) {
        // java Short charset = ShareConvertHelper.getShort4STUcharHexNumber(attributes);
        let charset: Option<i16> = ShareConvertHelper::getShort4STUcharHexNumber(attributes);
        // java charset = null == charset ? 1 : charset;
        let charset: i16 = match charset {
            None => 1,
            Some(v) => v,
        };
        // java fontInfo.setCharacterSet(charset);
        fontInfo.setCharacterSet(charset);
    }

    // java private static void setTrueType(Attributes attributes, FontInfo fontInfo)
    pub fn setTrueType(attributes: &dyn Attributes, fontInfo: &mut FontInfo) {
        // java Boolean value = ConvertHelper.toBoolean4NotNull(attributes);
        let value: Option<bool> = ConvertHelper::toBoolean4NotNull(attributes);
        // java fontInfo.setTrueType(value);
        fontInfo.setTrueType(value.unwrap_or(false));
    }

    // java private static void setCharacterPitch(Attributes attributes, FontInfo fontInfo)
    pub fn setCharacterPitch(attributes: &dyn Attributes, fontInfo: &mut FontInfo) {
        // java String pitch = ConvertHelper.getString(attributes);
        let pitch: Option<String> = ConvertHelper::getString(attributes);
        // java byte value = 0;
        let mut value: i8 = 0;
        // java if ("fixed".equals(pitch))
        if Some("fixed".to_string()) == pitch {
            // java value = 1;
            value = 1;
        // java } else if ("variable".equals(pitch))
        } else if Some("variable".to_string()) == pitch {
            // java value = 2;
            value = 2;
        // java } else if ("default".equals(pitch))
        } else if Some("default".to_string()) == pitch {
            // java value = 0;
            value = 0;
        } else {
            // java IOAssert.fail("It should not reach here!");
            IOAssert::fail_msg("It should not reach here!");
        }
        // java fontInfo.setCharacterPitch(value);
        fontInfo.setCharacterPitch(value);
    }

    // java private static void setSignature(Attributes attributes, FontInfo fontInfo)
    pub fn setSignature(attributes: &dyn Attributes, fontInfo: &mut FontInfo) {
        // java String USB0 = "usb0";
        let _USB0 = "usb0";
        // java Integer usb0 = ConvertHelper.hex2Int(attributes, "usb0");
        let mut usb0: Option<i32> = ConvertHelper::hex2Int(attributes, "usb0");
        // java if (null == usb0)
        if usb0.is_none() {
            // java usb0 = ConvertHelper.hex2Int(attributes, "usb-0");
            usb0 = ConvertHelper::hex2Int(attributes, "usb-0");
        }
        // java if (null != usb0)
        if usb0.is_some() {
            // java String USB1 = "usb1";
            let _USB1 = "usb1";
            // java Integer usb1 = ConvertHelper.hex2Int(attributes, "usb1");
            let mut usb1: Option<i32> = ConvertHelper::hex2Int(attributes, "usb1");
            // java if (usb1 == null)
            if usb1.is_none() {
                // java usb1 = ConvertHelper.hex2Int(attributes, "usb-1");
                usb1 = ConvertHelper::hex2Int(attributes, "usb-1");
            }
            // java if (null != usb1)
            if usb1.is_some() {
                // java String USB2 = "usb2";
                let _USB2 = "usb2";
                // java Integer usb2 = ConvertHelper.hex2Int(attributes, "usb2");
                let mut usb2: Option<i32> = ConvertHelper::hex2Int(attributes, "usb2");
                // java if (usb2 == null)
                if usb2.is_none() {
                    // java usb2 = ConvertHelper.hex2Int(attributes, "usb-2");
                    usb2 = ConvertHelper::hex2Int(attributes, "usb-2");
                }
                // java if (null != usb2)
                if usb2.is_some() {
                    // java String USB3 = "usb3";
                    let _USB3 = "usb3";
                    // java Integer usb3 = ConvertHelper.hex2Int(attributes, "usb3");
                    let mut usb3: Option<i32> = ConvertHelper::hex2Int(attributes, "usb3");
                    // java if (usb3 == null)
                    if usb3.is_none() {
                        // java usb3 = ConvertHelper.hex2Int(attributes, "usb-3");
                        usb3 = ConvertHelper::hex2Int(attributes, "usb-3");
                    }
                    // java if (null != usb3)
                    if usb3.is_some() {
                        // java String CSB0 = "csb0";
                        let _CSB0 = "csb0";
                        // java Integer csb0 = ConvertHelper.hex2Int(attributes, "csb0");
                        let mut csb0: Option<i32> = ConvertHelper::hex2Int(attributes, "csb0");
                        // java if (csb0 == null)
                        if csb0.is_none() {
                            // java csb0 = ConvertHelper.hex2Int(attributes, "csb-0");
                            csb0 = ConvertHelper::hex2Int(attributes, "csb-0");
                        }
                        // java if (null != csb0)
                        if csb0.is_some() {
                            // java String CSB1 = "csb1";
                            let _CSB1 = "csb1";
                            // java Integer csb1 = ConvertHelper.hex2Int(attributes, "csb1");
                            let mut csb1: Option<i32> = ConvertHelper::hex2Int(attributes, "csb1");
                            // java if (csb1 == null)
                            if csb1.is_none() {
                                // java csb1 = ConvertHelper.hex2Int(attributes, "csb-1");
                                csb1 = ConvertHelper::hex2Int(attributes, "csb-1");
                            }
                            // java if (null != csb1)
                            if csb1.is_some() {
                                // java byte[] sig = new byte[24];
                                let mut sig: Vec<i8> = vec![0i8; 24];
                                // java System.arraycopy(int2Bytes(usb0), 0, sig, 0, 4);
                                sig[0..4].clone_from_slice(&Self::int2Bytes(usb0.unwrap())[0..4]);
                                // java System.arraycopy(int2Bytes(usb1), 0, sig, 4, 4);
                                sig[4..8].clone_from_slice(&Self::int2Bytes(usb1.unwrap())[0..4]);
                                // java System.arraycopy(int2Bytes(usb2), 0, sig, 8, 4);
                                sig[8..12].clone_from_slice(&Self::int2Bytes(usb2.unwrap())[0..4]);
                                // java System.arraycopy(int2Bytes(usb3), 0, sig, 12, 4);
                                sig[12..16].clone_from_slice(&Self::int2Bytes(usb3.unwrap())[0..4]);
                                // java System.arraycopy(int2Bytes(csb0), 0, sig, 16, 4);
                                sig[16..20].clone_from_slice(&Self::int2Bytes(csb0.unwrap())[0..4]);
                                // java System.arraycopy(int2Bytes(csb1), 0, sig, 20, 4);
                                sig[20..24].clone_from_slice(&Self::int2Bytes(csb1.unwrap())[0..4]);
                                // java fontInfo.setFontSignature(sig);
                                fontInfo.setFontSignature(sig);
                            }
                        }
                    }
                }
            }
        }
    }

    // java private static byte[] int2Bytes(int src)
    pub fn int2Bytes(src: i32) -> Vec<i8> {
        // java byte[] bytes = new byte[4];
        let mut bytes: Vec<i8> = vec![0i8; 4];
        // java bytes[0] = (byte)(src & 255);
        bytes[0] = (src & 255) as i8;
        // java bytes[1] = (byte)(src >> 8 & 255);
        bytes[1] = ((src >> 8) & 255) as i8;
        // java bytes[2] = (byte)(src >> 16 & 255);
        bytes[2] = ((src >> 16) & 255) as i8;
        // java bytes[3] = (byte)(src >>> 24);
        bytes[3] = ((src as u32) >> 24) as i8;
        // java return bytes;
        return bytes;
    }

    // java private void setFamily(Attributes attributes, FontInfo fontInfo)
    pub fn setFamily(&mut self, attributes: &dyn Attributes, fontInfo: &mut FontInfo) {
        // java String family = ConvertHelper.getString(attributes);
        let family: Option<String> = ConvertHelper::getString(attributes);
        // java if (null != family)
        if family.is_some() {
            // java if (this.isXml03())
            let family: Option<String> = if self.base.isXml03() {
                // java family = Xml03Utils.firstCharToLowerCase(family);
                Xml03Utils::firstCharToLowerCase(family.as_deref())
            } else {
                family
            };
            // java byte value = 0;
            let mut value: i8 = 0;
            // java if ("auto".equals(family))
            if Some("auto".to_string()) == family {
                // java value = 0;
                value = 0;
            // java } else if ("decorative".equals(family))
            } else if Some("decorative".to_string()) == family {
                // java value = 5;
                value = 5;
            // java } else if ("modern".equals(family))
            } else if Some("modern".to_string()) == family {
                // java value = 3;
                value = 3;
            // java } else if ("roman".equals(family))
            } else if Some("roman".to_string()) == family {
                // java value = 1;
                value = 1;
            // java } else if ("script".equals(family))
            } else if Some("script".to_string()) == family {
                // java value = 4;
                value = 4;
            // java } else if ("swiss".equals(family))
            } else if Some("swiss".to_string()) == family {
                // java value = 2;
                value = 2;
            } else {
                // java IOAssert.fail("It should not reach here!");
                IOAssert::fail_msg("It should not reach here!");
            }
            // java fontInfo.setFontFamily(value);
            fontInfo.setFontFamily(value);
        }
    }
}
