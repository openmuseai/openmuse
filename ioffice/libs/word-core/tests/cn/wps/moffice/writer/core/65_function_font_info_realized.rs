// function_id=65_function_font_info_realized
// cargo test --manifest-path libs/word-core/Cargo.toml --test 65_function_font_info_realized
//
// ★M-STAGE1 P2 —— `FontInfo` 由 **compile-surface shell**（3 行 `pub struct FontInfo;`）
//   实化为 Java 304 行实现的逐字移植。本用例锁定实化后的**语义**，
//   而不是只证明"能编译"。
//
// 为什么需要这组用例（`M_STAGE1_BASELINE.md` §16.3）：
//   空壳 `pub struct FontInfo;` 让 `FontTable` 等调用方"能编译"，
//   但**所有字段读写都被静默丢弃** —— 与 `di03_seam` 影子的 `{}` 同一类假绿。
//   因此必须用**可观测的取值**证明字段真的存在且被正确初始化/夹取。
#![allow(non_snake_case)]


use word_core::cn::wps::moffice::writer::core::BuiltinFontTable::BuiltinFontTable;
use word_core::cn::wps::moffice::writer::data::value::FontInfo::FontInfo;

// ───────────────── FI1：Java 字段初值逐项对齐 ─────────────────

/// FI1：`new FontInfo()` 的初值须与 Java 字段声明一致 ——
/// 尤其 `mWeight = 400`（**不是 0**）与两个空串。空壳时代这些全不可观测。
#[test]
fn FI1_default_values_match_java_field_initializers() {
    let f = FontInfo::FontInfo();
    assert_eq!(f.getCharacterPitch(), 0, "mCharacterPitch = 0");
    assert!(!f.isTrueType(), "mTrueType = false");
    assert_eq!(f.getFontFamily(), 0, "mFontFamily = 0");
    assert_eq!(f.getWeight(), 400, "★mWeight = 400（Java 初值，非 0）");
    assert_eq!(f.getCharacterSet(), 0, "mCharacterSet = 0");
    assert_eq!(f.getFontName(), "", "mFontName = \"\"");
    assert_eq!(f.getAlternativeName(), "", "mAlternativeName = \"\"");
    assert_eq!(f.getPanoseFamilyType(), 0);
    assert_eq!(f.getPanoseSerifStyle(), 0);
    assert_eq!(f.getPanoseWeight(), 0);
    assert!(f.getFontSignature().is_none(), "mFontSignature = null");
}

// ───────────────── FI2：setter/getter 往返（字段真的存住了） ─────────────────

/// FI2：写入后必须读回**同一个值**。空壳时代读写都被丢弃，此断言必红。
#[test]
fn FI2_setters_round_trip() {
    let mut f = FontInfo::FontInfo();
    f.setFontName("Times New Roman".to_string());
    f.setAlternativeName("Times".to_string());
    f.setCharacterPitch(FontInfo::VariablePitch);
    f.setTrueType(true);
    f.setFontFamily(FontInfo::Decorative_Fantasy);
    f.setWeight(700);
    f.setCharacterSet(FontInfo::GB2312_CHARSET);

    assert_eq!(f.getFontName(), "Times New Roman");
    assert_eq!(f.getAlternativeName(), "Times");
    assert_eq!(f.getCharacterPitch(), FontInfo::VariablePitch);
    assert!(f.isTrueType());
    assert_eq!(f.getFontFamily(), FontInfo::Decorative_Fantasy);
    assert_eq!(f.getWeight(), 700);
    assert_eq!(f.getCharacterSet(), FontInfo::GB2312_CHARSET);
}

// ───────────────── FI3：Java 的**夹取**逻辑逐字保留 ─────────────────

/// FI3：以下 4 个 setter 在 Java 中**没有 `logTrue`，而是三元夹取**。
/// 阈值必须与 Java 逐字一致（越界回落到固定默认值）。
#[test]
fn FI3_clamping_setters_match_java_ternaries() {
    // java this.mPanoseWeight = 0 <= value && value <= 11 ? value : 3;
    let mut f = FontInfo::FontInfo();
    f.setPanoseWeight(5);
    assert_eq!(f.getPanoseWeight(), 5, "界内保留");
    f.setPanoseWeight(12);
    assert_eq!(f.getPanoseWeight(), 3, "越界回落 3");

    // java this.mPanoseProportion = 0 <= value && value <= 9 ? value : 9;
    f.setPanoseProportion(9);
    assert_eq!(f.getPanoseProportion(), 9, "界内保留（边界值 9）");
    f.setPanoseProportion(10);
    assert_eq!(f.getPanoseProportion(), 9, "越界回落 9");

    // java this.mPanoseConstrast = 0 <= value && value <= 9 ? value : 2;
    f.setPanoseConstrast(0);
    assert_eq!(f.getPanoseConstrast(), 0, "界内保留（边界值 0）");
    f.setPanoseConstrast(10);
    assert_eq!(f.getPanoseConstrast(), 2, "越界回落 2");
}

// ───────────────── FI4：★忠实保留 Java 原文的疑似笔误 ─────────────────

/// FI4：Java `setPanoseStrokeVariation` 体内写的是 **`this.mPanoseArmStyle`**
/// （疑为原代码笔误）。移植纪律要求**忠实保留**，不得"顺手纠正"。
///
/// 本用例把这一行为**固定下来**：若日后有人"修好"它，此断言会失败并提示
/// 该改动需要与 Java 侧对齐确认，而不是悄悄改变语义。
#[test]
fn FI4_setPanoseStrokeVariation_writes_ArmStyle_as_in_java() {
    let mut f = FontInfo::FontInfo();
    assert_eq!(f.getPanoseArmStyle(), 0, "前置：ArmStyle 初值 0");

    f.setPanoseStrokeVariation(7);
    assert_eq!(
        f.getPanoseArmStyle(),
        7,
        "★Java 原文即写 mPanoseArmStyle（笔误，忠实保留）"
    );
    assert_eq!(
        f.getPanoseStrokeVariation(),
        0,
        "★mPanoseStrokeVariation 本身**不会**被写入（Java 行为如此）"
    );

    f.setPanoseStrokeVariation(11);
    assert_eq!(
        f.getPanoseArmStyle(),
        2,
        "越界（>10）回落 2，同样写入 ArmStyle"
    );
}

// ───────────────── FI5：常量表逐项核对（抽样覆盖各段） ─────────────────

/// FI5：Java ~115 个 `public static final` 常量须逐字对应。
/// 抽样覆盖每个语义段（pitch / charset / panose family / serif / weight / …）。
#[test]
fn FI5_constants_match_java() {
    assert_eq!(FontInfo::DefaultPitch, 0);
    assert_eq!(FontInfo::FixedPitch, 1);
    assert_eq!(FontInfo::VariablePitch, 2);

    assert_eq!(FontInfo::ANSI_CHARSET, 0);
    assert_eq!(FontInfo::GB2312_CHARSET, 134);
    assert_eq!(FontInfo::CHINESEBIG5_CHARSET, 136);
    assert_eq!(FontInfo::OEM_CHARSET, 255);
    // java HANGEUL_CHARSET = 129; HANGUL_CHARSET = 129;（同值异名，两者都在）
    assert_eq!(FontInfo::HANGEUL_CHARSET, 129);
    assert_eq!(FontInfo::HANGUL_CHARSET, 129);

    assert_eq!(FontInfo::NONE, 0);
    assert_eq!(FontInfo::Decorative_Fantasy, 5);

    assert_eq!(FontInfo::PAN_ANY, 0);
    assert_eq!(FontInfo::PAN_FAMILY_PICTORIAL, 5);
    assert_eq!(FontInfo::PAN_SERIF_ROUNDED, 15);
    assert_eq!(FontInfo::PAN_WEIGHT_NORD, 11);
    assert_eq!(FontInfo::PAN_CONTRAST_VERY_HIGH, 9);
    assert_eq!(FontInfo::PAN_MIDLINE_LOW_SERIFED, 13);
    assert_eq!(FontInfo::PAN_XHEIGHT_DUCKING_LARGE, 7);
}

// ───────────────── FI6：`BuiltinFontTable` 真实装配（此前完全缺失） ─────────────────

/// FI6：`BuiltinFontTable.font` 有 **18** 行（Java 原文行数），且各行长度**不等**。
///
/// ★诚实边界：`getBuiltinFontTable()` 会调
/// `docReader::FontTableImporter::toFontInfo(ffn)`，而该函数**仍是 `todo!()` 桩**
/// （尚未按 Java 304 行同族实现）⇒ 完整装配目前会 panic。
/// 本用例因此**只断言已就位的部分**（数据表本身），并把"下游未实现"显式记录，
/// 而不是用 `catch_unwind` 吞掉 panic 假装通过。
#[test]
fn FI6_builtin_font_table_data_is_present_and_ragged() {
    assert_eq!(
        word_core::cn::wps::moffice::writer::core::BuiltinFontTable::font.len(),
        18,
        "Java font[][] 共 18 行"
    );

    // 各行长度**不等**（46–72）—— 这正是 Rust 侧必须用 `[&[i8]; 18]` 的原因。
    let lens: std::collections::BTreeSet<usize> =
        word_core::cn::wps::moffice::writer::core::BuiltinFontTable::font
            .iter()
            .map(|r| r.len())
            .collect();
    assert!(
        lens.len() > 1,
        "各行长度应不等（实测 {{46,50,52,54,56,58,60,64,66,72}}），实得 {lens:?}"
    );

    // 第一行首字节锚定（取自 Java 原文），防止数据被误改。
    assert_eq!(
        word_core::cn::wps::moffice::writer::core::BuiltinFontTable::font[0][0],
        71,
        "第 0 行首字节 = 71（Java 原文）"
    );
    assert_eq!(
        word_core::cn::wps::moffice::writer::core::BuiltinFontTable::font[0].len(),
        72,
        "第 0 行长度 = 72（Java 原文）"
    );

    eprintln!(
        "FI6: font rows = 18, distinct lengths = {} -> {lens:?}",
        lens.len()
    );
}

// ───────────────── FI7：★完整装配链跑通（`toFontInfo` 已实现） ─────────────────

/// FI7：`getBuiltinFontTable()` **完整装配**必须跑通并产出非空表。
///
/// 上一轮此断言**无法成立**：下游 `FontTableImporter::toFontInfo(ffn)` 是 `todo!()`，
/// 一调用即 panic（§18.6 记录的诚实边界）。本轮该 function 已按 Java 原文实现，
/// 故把"只验证数据表"升级为"验证**端到端装配**"。
///
/// 这是"接线 ≠ 生效"纪律的应用：数据在（FI6 已证）+ 转换在（FI7 证）⇒ 装配才真生效。
#[test]
fn FI7_builtin_font_table_fully_assembles() {
    let table = BuiltinFontTable::getBuiltinFontTable();
    assert!(
        !table.is_empty(),
        "★内置字体表须非空 —— 为空即 `toFontInfo`/装配链未生效"
    );

    // Java 为 18 行 font 各建一个 FontInfo，按 fontName 去重后入表。
    assert!(
        table.len() <= 18,
        "去重后条目数不可能超过 18 行，实得 {}",
        table.len()
    );

    // ★装配自洽性：每个 FontInfo 的 fontName 必须与它的 map key 一致。
    for (k, v) in table.iter() {
        assert_eq!(
            v.getFontName(),
            *k,
            "装配自洽性：FontInfo.fontName 必须等于其 map key"
        );
    }

    let mut names: Vec<&String> = table.keys().collect();
    names.sort();
    eprintln!(
        "FI7: ★完整装配跑通 —— 内置字体表 {} 项：{:?}",
        table.len(),
        names
    );
}

// ───────────────── FI8：`toFontInfo` 字段映射逐项对齐 Java ─────────────────

/// FI8：`toFontInfo(ffn)` 的 7 个字段赋值 + `setPanose` 的 10 个字节
/// 必须与 Java 原文逐项一致。
///
/// 用 `BuiltinFontTable.font[0]`（已知 72 字节）构造**真实** `Ffn`，
/// 而不是伪造对象 —— 保证走的是真实 `Ffn` 解码路径。
#[test]
fn FI8_toFontInfo_maps_fields_per_java() {
    use word_core::cn::wps::moffice::writer::io::reader::docReader::importer::FontTableImporter::FontTableImporter;

    let bytes: Vec<u8> = word_core::cn::wps::moffice::writer::core::BuiltinFontTable::font[0]
        .iter()
        .map(|b| *b as u8)
        .collect();
    let ffn = io_poi_compact::org::apache::poi::hwpf::model::Ffn::Ffn::new_from_bytes(&bytes, 0);

    let info = FontTableImporter::toFontInfo(&ffn);
    assert_eq!(
        info.getFontName(),
        ffn.getMainFontName(),
        "setFontName(ffn.getMainFontName())"
    );
    assert_eq!(
        info.getAlternativeName(),
        ffn.getAltFontName(),
        "setAlternativeName(ffn.getAltFontName())"
    );
    assert_eq!(info.getCharacterPitch(), ffn.get_prq(), "setCharacterPitch");
    assert_eq!(info.isTrueType(), ffn.isTrueType(), "setTrueType");
    assert_eq!(info.getFontFamily(), ffn.get_ff(), "setFontFamily");
    assert_eq!(info.getWeight(), ffn.getWeight(), "setWeight");
    assert_eq!(info.getCharacterSet(), ffn.getChs(), "setCharacterSet");

    // Panose：Java 按 bytes[0..10] 顺序映射。
    // ★bytes[5] 走 setPanoseStrokeVariation，而该 setter 按 Java 原文写入
    //   **mPanoseArmStyle**（FI4 已固定），随后 bytes[6] 由 setPanoseArmStyle
    //   再写一次 ⇒ 最终 ArmStyle 应为 bytes[6]。
    let panose = ffn.getPanose();
    assert_eq!(panose.len(), 10, "Ffn.getPanose() 须为 10 字节");
    assert_eq!(info.getPanoseFamilyType(), panose[0] as i8, "bytes[0]");
    assert_eq!(info.getPanoseSerifStyle(), panose[1] as i8, "bytes[1]");
    assert_eq!(
        info.getPanoseArmStyle(),
        panose[6] as i8,
        "★bytes[6] 最后写入 ArmStyle"
    );

    eprintln!(
        "FI8: fontName={:?} weight={} charset={} panose={:?}",
        info.getFontName(),
        info.getWeight(),
        info.getCharacterSet(),
        panose
    );
}
