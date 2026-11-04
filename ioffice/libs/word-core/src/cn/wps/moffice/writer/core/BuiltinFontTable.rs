// Java: cn.wps.moffice.writer.core.BuiltinFontTable
//
// ★M-STAGE1 P2 —— 本类型此前**完全缺失**（全仓 0 hit），是 `di03_seam` 影子存在的
//   又一根因：`FontTable.genFontInfo` 需要 `BuiltinFontTable.font[index]` 构造 `Ffn`
//   ⇒ 类型不存在时整条链路只能被影子挡成 `{}`。见 `M_STAGE1_BASELINE.md` §16.3。
//
// 按 Java 原文对齐：18 行 `font` 内置字体数据表（**各行长度不等** 46–72，
// 故 Rust 侧用 `[&[i8]; 18]` 而非定长二维数组）+ `getBuiltinFontTable()` 惰性装配。
//
// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.moffice.writer.data.value.FontInfo
use crate::cn::wps::moffice::writer::data::value::FontInfo::FontInfo;
// Java: import cn.wps.moffice.writer.io.reader.docReader.importer.FontTableImporter
use crate::cn::wps::moffice::writer::io::reader::docReader::importer::FontTableImporter::FontTableImporter;
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import org.apache.poi.hwpf.model.Ffn
use io_poi_compact::org::apache::poi::hwpf::model::Ffn::Ffn;

// Java: public class BuiltinFontTable
pub struct BuiltinFontTable;

// java private static HashMap<String, FontInfo> sBuiltInFontTable = new HashMap();
static sBuiltInFontTable: std::sync::Mutex<Option<HashMap<String, FontInfo>>> =
    std::sync::Mutex::new(None);

// java static final byte[][] font = new byte[][]{...};
pub static font: [&[i8]; 18] = [
    &[71, 22, -112, 1, 0, 0, 2, 2, 6, 3, 5, 4, 5, 2, 3, 4, -121, 122, 0, 32, 0, 0, 0, -128, 8, 0, 0, 0, 0, 0, 0, 0, -1, 1, 0, 0, 0, 0, 0, 0, 84, 0, 105, 0, 109, 0, 101, 0, 115, 0, 32, 0, 78, 0, 101, 0, 119, 0, 32, 0, 82, 0, 111, 0, 109, 0, 97, 0, 110, 0, 0, 0],
    &[55, 36, -112, 1, -122, 0, 2, 15, 5, 2, 2, 2, 4, 3, 2, 4, -1, 2, 0, -31, -1, -84, 0, 64, 9, 0, 0, 0, 0, 0, 0, 0, -97, 1, 0, 32, 0, 0, 0, 0, 67, 0, 97, 0, 108, 0, 105, 0, 98, 0, 114, 0, 105, 0, 0, 0],
    &[65, 20, -112, 1, 0, 0, 2, 4, 5, 3, 5, 4, 6, 3, 2, 4, -1, 2, 0, -32, -1, 36, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, -97, 1, 0, 32, 0, 0, 0, 0, 67, 0, 97, 0, 109, 0, 98, 0, 114, 0, 105, 0, 97, 0, 32, 0, 77, 0, 97, 0, 116, 0, 104, 0, 0, 0],
    &[55, 20, -112, 1, 0, 0, 2, 4, 5, 3, 5, 4, 6, 3, 2, 4, -1, 2, 0, -32, -1, 4, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, -97, 1, 0, 32, 0, 0, 0, 0, 67, 0, 97, 0, 109, 0, 98, 0, 114, 0, 105, 0, 97, 0, 0, 0],
    &[51, 36, -112, 1, 1, 0, 2, 11, 6, 4, 2, 2, 2, 2, 2, 4, -121, 122, 0, 0, 0, 0, 0, -128, 8, 0, 0, 0, 0, 0, 0, 0, -1, 1, 0, 64, 0, 0, -1, -1, 65, 0, 114, 0, 105, 0, 97, 0, 108, 0, 0, 0],
    &[63, 52, -112, 1, 1, 0, 2, 7, 3, 9, 2, 2, 5, 2, 4, 4, -121, 122, 0, 0, 0, 0, 0, -128, 8, 0, 0, 0, 0, 0, 0, 0, -1, 1, 0, 64, 0, 0, -1, -1, 67, 0, 111, 0, 117, 0, 114, 0, 105, 0, 101, 0, 114, 0, 32, 0, 78, 0, 101, 0, 119, 0, 0, 0],
    &[53, 20, -112, 1, 2, 0, 5, 5, 1, 2, 1, 7, 6, 2, 5, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 83, 0, 121, 0, 109, 0, 98, 0, 111, 0, 108, 0, 0, 0],
    &[59, 4, -112, 1, 2, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 87, 0, 105, 0, 110, 0, 103, 0, 100, 0, 105, 0, 110, 0, 103, 0, 115, 0, 0, 0],
    &[63, -112, 30, 1, 2, 0, 5, 2, 1, 2, 1, 5, 7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 87, 0, 105, 0, 110, 0, 103, 0, 100, 0, 105, 0, 110, 0, 103, 0, 115, 0, 32, 0, 50, 0, 0, 0],
    &[45, 4, -112, 1, -122, 0, 2, 1, 6, 0, 3, 1, 1, 1, 1, 1, 3, 0, 0, 0, 0, 0, 14, 8, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 4, 0, 0, 0, 0, 0, -117, 91, 83, 79, 0, 0],
    &[45, 4, -112, 1, -122, 0, 2, 1, 6, 0, 3, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 14, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, -47, -98, 83, 79, 0, 0],
    &[49, 36, -112, 1, -122, 0, 2, 11, 5, 3, 2, 2, 4, 2, 2, 4, -121, 2, 0, -128, 82, 60, 15, 40, 22, 0, 0, 0, 0, 0, 0, 0, 31, 0, 4, 0, 0, 0, 0, 0, -82, 95, 111, -113, -59, -106, -47, -98, 0, 0],
    &[55, 4, -112, 1, -122, 5, 2, 1, 8, 0, 4, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 15, 8, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 78, 83, -121, 101, 76, -120, 119, 105, 0, 0, -117, 91, 83, 79, 0, 0],
    &[59, 4, -112, 1, 2, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 87, 0, 105, 0, 110, 0, 103, 0, 100, 0, 105, 0, 110, 0, 103, 0, 115, 0, 0, 0],
    &[53, 20, -112, 1, 2, 0, 5, 5, 1, 2, 1, 7, 6, 2, 5, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 83, 0, 121, 0, 109, 0, 98, 0, 111, 0, 108, 0, 0, 0],
    &[57, 22, -112, 1, 2, 0, 5, 5, 1, 2, 1, 2, 5, 2, 2, 2, 0, 0, 0, -128, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -128, 0, 0, 0, 0, 77, 0, 84, 0, 32, 0, 69, 0, 120, 0, 116, 0, 114, 0, 97, 0, 0, 0],
    &[53, 30, -112, 1, 0, 0, 2, 4, 5, 3, 5, 2, 3, 3, 2, 2, 3, -128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 77, 0, 97, 0, 110, 0, 103, 0, 97, 0, 108, 0, 0, 0],
    &[63, 30, -112, 1, 0, 0, 2, 2, 6, 3, 5, 4, 5, 2, 3, 4, 3, 0, 0, -127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 65, 0, 110, 0, 103, 0, 115, 0, 97, 0, 110, 0, 97, 0, 32, 0, 78, 0, 101, 0, 119, 0, 0, 0],
];

impl BuiltinFontTable {
    // java public static HashMap<String, FontInfo> getBuiltinFontTable()
    pub fn getBuiltinFontTable() -> HashMap<String, FontInfo> {
        // java Assert.assertNotNull("sBuiltInFontTable should not be null!", sBuiltInFontTable);
        // rust: assertNotNull 对 Mutex 不适用（真实 Assert 仅收 &Option<T>）；
        //   改为断言锁可取（与原意一致：静态表必须可用）。
        let mut guard = sBuiltInFontTable.lock().expect("sBuiltInFontTable lock");
        let table = guard.get_or_insert_with(HashMap::new);

        // java for(int i = 0; i < font.length; ++i)
        let mut i: i32 = 0;
        let len: i32 = font.len() as i32;
        while i < len {
            // java Ffn ffn = new Ffn(font[i], 0);
            let mut ffn: Ffn = Ffn::new_from_bytes(&font[i as usize].iter().map(|b| *b as u8).collect::<Vec<u8>>(), 0);
            // java String fontName = ffn.getMainFontName();
            let fontName: String = ffn.getMainFontName();
            // java sBuiltInFontTable.put(fontName, FontTableImporter.toFontInfo(ffn));
            table.insert(fontName, FontTableImporter::toFontInfo(&ffn));
            i += 1;
        }

        // java return sBuiltInFontTable;
        table.clone()
    }



}
