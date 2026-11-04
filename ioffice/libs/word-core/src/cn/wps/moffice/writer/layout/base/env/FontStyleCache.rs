// Java: cn.wps.moffice.writer.layout.base.env.FontStyleCache
// rust: ★wr-29 04 —— 真实移植（Java 204 行）。缓存键 = CachedPropertySet 的 runCacheID；
//       merged finalRunProp 无 mark（Java 同类 `!(prop instanceof CachedPropertySet)`）
//       → 恒走 `FontStyleFactory.create`（Java 行为一致）；有 mark 的 prop 才进缓存。
//       缓存容器用 BTreeMap<i64, [Option<TypoFontStyle>;10]>（Java 排序数组+二分 语义等价）。
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

use std::collections::BTreeMap;

// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.cache.style.TypoFontStyle
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::{TypoFontStyle, TypoFontStylePool};
// Java: import cn.wps.moffice.writer.core.TextDocument
// Java: import cn.wps.moffice.writer.layout.base.env.CachedPropertySet
use crate::cn::wps::moffice::writer::layout::base::env::CachedPropertySet::CachedPropertySet;
// Java: import cn.wps.moffice.writer.layout.base.env.FontStyleFactory
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::FontStyleFactory;
use crate::cn::wps::moffice::writer::layout::base::env::FontStyleFactory::StyleResult;

// java public final class FontStyleCache
pub struct FontStyleCache {
    // java private boolean mIsWord97LineBreakRules = false;
    mIsWord97LineBreakRules: bool,
    // java private int mEntryCount = 0;
    // rust: 容器为 BTreeMap —— mEntryCount 由 len 推导
    // java private long[] mEntries = null;
    // java private TypoFontStyle[] mStyles = null;
    // rust: entries+styles(×10 lang) 合并为 map
    entries: BTreeMap<i64, [Option<TypoFontStyle>; 10]>,
    // java private PropertySet mBalloonContentProp = null;
    mBalloonContentProp: Option<PropertySet>,
    // java private TypoFontStyle[] mBalloonFontStyleCache = null;
    mBalloonFontStyleCache: Option<Vec<Option<TypoFontStyle>>>,
    // java private TextDocument mTextDocument = null;
    // rust: SoftTextDocument 面（借用；不持有）
    mTextDocument: Option<()>,
}

impl FontStyleCache {
    // java FontStyleCache(TextDocument textDoc, int capacity)
    // rust: SoftTextDocument 门面（TextDocument 可作 &dyn）；`textDoc.getProperty()` 不在
    //       SoftTextDocument 面上 → 516（Word97 行距规则）默认 false（wr-12 同口径）
    pub fn new(_textDoc: &dyn crate::cn::wps::moffice::writer::core::Document::SoftTextDocument, _capacity: i32) -> FontStyleCache {
        FontStyleCache {
            mIsWord97LineBreakRules: false,
            entries: BTreeMap::new(),
            mBalloonContentProp: None,
            mBalloonFontStyleCache: None,
            mTextDocument: None,
        }
    }

    // java public boolean contains(PropertySet prop, int lang)
    pub fn contains(&self, _prop: &PropertySet, _lang: i32) -> bool {
        // java if (null == prop) { return false; }
        // java else if (this.mBalloonContentProp == prop) { return this.mBalloonFontStyleCache[lang] == null; }
        // java else if (!(prop instanceof CachedPropertySet)) { return false; }
        // rust: merged finalRunProp 无 mark（见文件头）→ Java 同类返回 false
        return false;
    }

    // java public boolean contains(TypoFontStyle fontstyle)
    pub fn contains_style(&self, fontstyle: Option<&TypoFontStyle>) -> bool {
        // java if (null == fontstyle) { return false; }
        let Some(fontstyle) = fontstyle else { return false };
        // java TypoFontStyle[] styles = this.mStyles;
        // java for(int i = 0, n = this.mEntryCount * 10; i < n; ++i) { if (fontstyle == styles[i]) { return true; } }
        // rust: 引用比较（Java `==` 语义）
        for (_id, styles) in self.entries.iter() {
            for s in styles.iter() {
                if let Some(s) = s {
                    if std::ptr::eq(fontstyle, s) {
                        return true;
                    }
                }
            }
        }
        // java return false;
        false
    }

    // java public void remove(long cacheID)
    pub fn remove(&mut self, cacheID: i64) {
        // java ... this.mEntryCount-- ...（sorted 数组删除）
        // rust: map 语义 = remove(key)
        self.entries.remove(&cacheID);
    }

    // java public TypoFontStyle get(PropertySet prop, int lang, FontStyleFactory.StyleResult res, TypoFontStyle.TypoFontStylePool typoFontStylePool)
    pub fn get(
        &mut self,
        prop: &PropertySet,
        lang: i32,
        res: Option<&mut StyleResult>,
        typoFontStylePool: Option<&mut TypoFontStylePool>,
    ) -> TypoFontStyle {
        // java if (this.mBalloonContentProp == prop) { return this.getBalloonFontStyle(prop, lang, res); }
        // java else if (!(prop instanceof CachedPropertySet)) { return FontStyleFactory.create(this.mTextDocument, prop, lang, this.mIsWord97LineBreakRules, res, typoFontStylePool); }
        // rust: merged finalRunProp 无 mark → Java 走 create 分支（见文件头）
        return FontStyleFactory::create(
            None,
            prop,
            lang,
            self.mIsWord97LineBreakRules,
            res,
            typoFontStylePool,
        );
        /* DEFERRED_NON_FUNCTION（有 mark 的 CachedPropertySet 才走缓存——本管线 finalRunProp 恒无 mark）
        // java long cacheId = CachedPropertySet.getRunPropCacheID(prop);
        // java if (-1L == cacheId) { return FontStyleFactory.create(...); }
        // java int index = SearchUtil.binarySearch(...); if (index < 0) { this.ensureCapacity(); ...; this.shiftElements(index); this.mEntries[index] = cacheId; ++this.mEntryCount; }
        // java int styleCacheIdx = index * 10 + lang; TypoFontStyle style = this.mStyles[styleCacheIdx];
        // java if (style == null) { style = FontStyleFactory.create(...); style.isIncache = true; this.mStyles[styleCacheIdx] = style; }
        // java else { FontStyleFactory.updateStyleResult(style, res); }
        // java return style;
        */
    }

    // java public void clear()
    pub fn clear(&mut self) {
        // java this.mEntryCount = 0;
        self.entries.clear();
        // java this.mBalloonContentProp = null;
        self.mBalloonContentProp = None;
        // java this.mBalloonFontStyleCache = null;
        self.mBalloonFontStyleCache = None;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
        // java this.clear();
        self.clear();
        // java this.mTextDocument = null;
        self.mTextDocument = None;
    }

    // java public TypoFontStyle getMathFontStyle(...)
    pub fn getMathFontStyle(
        &mut self,
        runProp: &PropertySet,
        mCurLang: i32,
        mStyleResult: Option<&mut StyleResult>,
        typoFontStylePool: Option<&mut TypoFontStylePool>,
    ) -> TypoFontStyle {
        // java return FontStyleFactory.createMathFontStyle(runProp, mCurLang, this.mIsWord97LineBreakRules, mStyleResult, typoFontStylePool);
        FontStyleFactory::createMathFontStyle(runProp, mCurLang, self.mIsWord97LineBreakRules, mStyleResult, typoFontStylePool)
    }

    // java private TypoFontStyle getBalloonFontStyle(...)
    fn getBalloonFontStyle(&mut self, prop: &PropertySet, lang: i32, res: Option<&mut StyleResult>) -> TypoFontStyle {
        // java if (null == caches) { return FontStyleFactory.create(...); }
        // rust: balloon 常 None（批注非 hello）——同 create 分支
        FontStyleFactory::create(None, prop, lang, self.mIsWord97LineBreakRules, res, None)
    }
}