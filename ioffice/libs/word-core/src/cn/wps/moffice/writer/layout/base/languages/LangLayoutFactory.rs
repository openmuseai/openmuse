// Java: cn.wps.moffice.writer.layout.base.languages.LangLayoutFactory
// rust: ★wr-29 04 —— 真实移植（Java 57 行）：getInstance(lang) → LangLayout（单实现类）。
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

// Java: import cn.wps.moffice.writer.layout.base.languages.LangLayout
use crate::cn::wps::moffice::writer::layout::base::languages::ILangLayout::ILangLayout;
use crate::cn::wps::moffice::writer::layout::base::languages::LangLayout::LangLayout;
use crate::cn::wps::moffice::writer::layout::base::languages::WestLayout::WestLayout;
use crate::cn::wps::moffice::writer::layout::base::languages::FarEastLayout::FarEastLayout;
use crate::cn::wps::moffice::writer::layout::base::languages::SharedFarEastLayout::SharedFarEastLayout;

// java public class LangLayoutFactory
// rust: ★wr-29 04 —— lang 0/1/9 = WestLayout（hello ASCII 合并热路径）；其它用 LangLayout 基类
pub struct LangLayoutFactory {}

impl LangLayoutFactory {
    // java public static ILangLayout getInstance(int language)
    pub fn getInstance(language: i32) -> Box<dyn ILangLayout> {
        // java switch (language) { case 0: case 1: case 9: return new WestLayout(lang); case 2: FarEast; ... }
        // java case 2: new FarEastLayout(lang); case 3: new SharedFarEastLayout(lang);
        match language {
            0 | 1 | 9 => Box::new(WestLayout::WestLayout(language)),
            2 => Box::new(FarEastLayout::FarEastLayout(language)),
            3 => Box::new(SharedFarEastLayout::SharedFarEastLayout(language)),
            _ => Box::new(LangLayout { language }),
        }
    }
}
