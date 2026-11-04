// Java: cn.wps.moffice.writer.io.reader.docReader.importer.FontTableImporter
//
// ★M-STAGE1 P2 —— 本文件原为 auto-generated 桩（全部 `todo!()`），
//   本轮按 Java 原文实现 `toFontInfo` / `setPanose`，解除 `BuiltinFontTable`、
//   `FontTable` 装配链上的 panic（`M_STAGE1_BASELINE.md` §18.6 / §18.8 步骤 1）。
//
// ★注意与 **docxReader** 侧同名类区分：Java 中这是**两个不同的类**。
//   - `docReader/importer/FontTableImporter.java` ← 本文件，持有 `toFontInfo` 静态方法
//   - `docxReader/importer/FontTableImporter.java` ← 另一个类（实例 importer）

// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::IOAssert;
// Java: import cn.wps.moffice.writer.data.value.FontInfo
use crate::cn::wps::moffice::writer::data::value::FontInfo::FontInfo;
// Java: import org.apache.poi.hwpf.model.Ffn
use io_poi_compact::org::apache::poi::hwpf::model::Ffn::Ffn;
// Java: import org.apache.poi.hwpf.model.FontTable
use io_poi_compact::org::apache::poi::hwpf::model::FontTable::FontTable;
// Java: import cn.wps.moffice.writer.core.FontTable  ← 与上一行**同名不同包**，
//   在 Rust 侧对应 `writer::core::FontTable`，故需别名区分（Java 靠包名区分）。
use crate::cn::wps::moffice::writer::core::FontTable::FontTable as WpsFontTable;

// Java: public final class FontTableImporter
pub struct FontTableImporter {
    // java private static final String TAG = null;
    // rust: Java 的 TAG 为 null 且仅用于 IOLog.e 日志，移植未启用日志故不置字段。
}

impl FontTableImporter {
    // java public void onImportFontTable(FontTable src, cn.wps.moffice.writer.core.FontTable dst)
    pub fn onImportFontTable(&self, src: Option<FontTable>, dst: &mut WpsFontTable) {
        // java IOAssert.assertNotNull("dst should not be null!", dst);
        // rust: Rust 的 `&mut` 天然非空 ⇒ 该断言由类型系统保证，无需运行期检查。
        // java if (null == src)
        if src.is_none() {
            // java dst.add("Times New Roman");
            dst.add__string("Times New Roman");
        } else {
            // java Ffn[] fontList = src.getFontNames();
            let src_table = src.unwrap();
            let fontList: &Vec<Ffn> = src_table.getFontNames();

            // java for(int i = 0; i < fontList.length; ++i)
            let mut i: i32 = 0;
            let n: i32 = fontList.len() as i32;
            while i < n {
                // java try {
                // java    dst.add(toFontInfo(fontList[i]));
                // java } catch (Throwable e) {
                // java    IOLog.e(TAG, "Throwable", e);
                // java }
                // rust: Java 用 catch(Throwable) 吞掉单个字体项的异常并继续。
                //   Rust 无异常，对应语义为"单点失败不影响整体" ⇒ 用 catch_unwind 等价还原，
                //   并以 `let _ =` 显式丢弃（与 Java 只记日志、不重抛一致）。
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    Self::toFontInfo(&fontList[i as usize])
                }));
                if let Ok(info) = r {
                    dst.add__fontinfo(&info);
                }
                i += 1;
            }
        }
    }

    // java public static FontInfo toFontInfo(Ffn ffn)
    pub fn toFontInfo(ffn: &Ffn) -> FontInfo {
        // java IOAssert.assertNotNull("ffn should not be null!", ffn);
        // rust: `ffn` 按值传入 ⇒ 非空由类型系统保证。
        // java FontInfo info = new FontInfo();
        let mut info: FontInfo = FontInfo::FontInfo();
        // java info.setCharacterPitch(ffn.get_prq());
        info.setCharacterPitch(ffn.get_prq());
        // java info.setTrueType(ffn.isTrueType());
        info.setTrueType(ffn.isTrueType());
        // java info.setFontFamily(ffn.get_ff());
        info.setFontFamily(ffn.get_ff());
        // java info.setWeight(ffn.getWeight());
        info.setWeight(ffn.getWeight());
        // java info.setCharacterSet(ffn.getChs());
        info.setCharacterSet(ffn.getChs());
        // java info.setFontName(ffn.getMainFontName());
        info.setFontName(ffn.getMainFontName());
        // java info.setAlternativeName(ffn.getAltFontName());
        info.setAlternativeName(ffn.getAltFontName());
        // java setPanose(info, ffn.getPanose());
        Self::setPanose(&mut info, ffn.getPanose());
        // java info.setFontSignature(ffn.getFontSig());
        info.setFontSignature(ffn.getFontSig().iter().map(|b| *b as i8).collect());
        // java return info;
        return info;
    }

    // java private static void setPanose(FontInfo info, byte[] bytes)
    pub fn setPanose(info: &mut FontInfo, bytes: &[u8]) {
        // java IOAssert.assertNotNull("info should not be null!", info);
        // rust: `&mut` 天然非空 ⇒ 类型系统保证。
        // java IOAssert.assertNotNull("bytes should not be null!", bytes);
        // rust: `&[u8]` 天然非空 ⇒ 类型系统保证。
        // java IOAssert.assertTrue("10 == bytes.length should be true!", 10 == bytes.length);
        IOAssert::assertTrue_msg("10 == bytes.length should be true!", 10 == bytes.len() as i32);
        // java info.setPanoseFamilyType(bytes[0]);
        info.setPanoseFamilyType(bytes[0] as i8);
        // java info.setPanoseSerifStyle(bytes[1]);
        info.setPanoseSerifStyle(bytes[1] as i8);
        // java info.setPanoseWeight(bytes[2]);
        info.setPanoseWeight(bytes[2] as i8);
        // java info.setPanoseProportion(bytes[3]);
        info.setPanoseProportion(bytes[3] as i8);
        // java info.setPanoseConstrast(bytes[4]);
        info.setPanoseConstrast(bytes[4] as i8);
        // java info.setPanoseStrokeVariation(bytes[5]);
        info.setPanoseStrokeVariation(bytes[5] as i8);
        // java info.setPanoseArmStyle(bytes[6]);
        info.setPanoseArmStyle(bytes[6] as i8);
        // java info.setPanoseLetterform(bytes[7]);
        info.setPanoseLetterform(bytes[7] as i8);
        // java info.setPanoseMidline(bytes[8]);
        info.setPanoseMidline(bytes[8] as i8);
        // java info.setPanoseHeight(bytes[9]);
        info.setPanoseHeight(bytes[9] as i8);
    }
}
