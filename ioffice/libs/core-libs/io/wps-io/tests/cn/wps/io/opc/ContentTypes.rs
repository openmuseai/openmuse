// function_id=21_function_content_types_guess
// status=loop — executable tests live beside src, not in features/
// cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 21_function_content_types_guess
// src:  libs/core-libs/io/wps-io/src/cn/wps/io/opc/ContentTypes.rs
//        libs/core-libs/io/wps-io/src/cn/wps/io/xwpf/usermodel/PictureType.rs（被查表）
// test: libs/core-libs/io/wps-io/tests/cn/wps/io/opc/ContentTypes.rs

use core_libs_io::io::opc::ContentTypes::ContentTypes;
use core_libs_io::io::xwpf::usermodel::PictureType::PictureType;

/// Java `PictureType` 的 18 项（`PictureType.java:7-24`：MIME / 扩展名 / ooxmlId）。
/// 表驱动，G4 用它做「枚举常量集合漂移」检测。
const ALL_18: [(&str, &str); 18] = [
    ("emf", "image/x-emf"),
    ("wmf", "image/x-wmf"),
    ("pict", "image/x-pict"),
    ("jpeg", "image/jpeg"),
    ("jpg", "image/jpeg"),
    ("png", "image/png"),
    ("dib", "image/dib"),
    ("gif", "image/gif"),
    ("tiff", "image/tiff"),
    ("tif", "image/tiff"),
    ("eps", "image/x-eps"),
    ("bmp", "image/x-bmp"),
    ("wpg", "image/x-wpg"),
    ("wdp", "image/vnd.ms-photo"),
    ("svg", "image/svg+xml"),
    ("pcx", "image/pcx"),
    ("psd", "image/psd"),
    ("tag", "image/tag"),
];

/// G1: 主路径 —— `png`。
#[test]
fn g1_png_hits_image_png() {
    assert_eq!(
        ContentTypes::guessContentTypeByExtension("png").as_deref(),
        Some("image/png")
    );
}

/// G2: 主路径 —— `jpeg` / `jpg` 是两个不同枚举常量、同一个 MIME
/// （Java `:56` 先判 JPEG、`:58` 再判 JPG ⇒ 身份必须分别是 JPEG / JPG）。
#[test]
fn g2_jpeg_and_jpg_share_jpeg_mime_but_are_distinct_constants() {
    for ext in ["jpeg", "jpg"] {
        assert_eq!(
            ContentTypes::guessContentTypeByExtension(ext).as_deref(),
            Some("image/jpeg"),
            "扩展名 {ext}"
        );
    }
    assert!(
        PictureType::getPictureType("jpeg") == Some(PictureType::JPEG),
        "`jpeg` 必须解析为 JPEG 常量（Java `:56` 先判 JPEG）"
    );
    assert!(
        PictureType::getPictureType("jpg") == Some(PictureType::JPG),
        "`jpg` 必须解析为 JPG 常量（Java `:58` 再判 JPG）"
    );
}

/// G3: 主路径 —— `tiff` / `tif`（Java `:74` / `:76`，同为 `image/tiff`）。
#[test]
fn g3_tiff_and_tif_share_tiff_mime_but_are_distinct_constants() {
    for ext in ["tiff", "tif"] {
        assert_eq!(
            ContentTypes::guessContentTypeByExtension(ext).as_deref(),
            Some("image/tiff"),
            "扩展名 {ext}"
        );
    }
    assert!(
        PictureType::getPictureType("tiff") == Some(PictureType::TIFF),
        "`tiff` 必须解析为 TIFF 常量"
    );
    assert!(
        PictureType::getPictureType("tif") == Some(PictureType::TIF),
        "`tif` 必须解析为 TIF 常量"
    );
}

/// G4: 18 项全表命中 + 枚举常量集合与表一一对应（防漂移）。
#[test]
fn g4_all_18_extensions_hit_their_mime() {
    let mut hits = 0usize;
    for (ext, ct) in ALL_18 {
        assert_eq!(
            ContentTypes::guessContentTypeByExtension(ext).as_deref(),
            Some(ct),
            "扩展名 {ext} 应命中 {ct}"
        );
        assert_eq!(
            PictureType::getPictureType(ext).map(|pt| pt.getContentType()),
            Some(ct.to_string()),
            "扩展名 {ext} 经枚举取 MIME 应一致"
        );
        hits += 1;
    }
    assert_eq!(hits, 18, "18 项图片扩展名必须全部命中");

    // 反漂移：枚举的 18 个常量，其 `getExtension()` 必须**逐个**出现在上表里且只出现一次
    let constants = [
        PictureType::EMF,
        PictureType::WMF,
        PictureType::PICT,
        PictureType::JPEG,
        PictureType::JPG,
        PictureType::PNG,
        PictureType::DIB,
        PictureType::GIF,
        PictureType::TIFF,
        PictureType::TIF,
        PictureType::EPS,
        PictureType::BMP,
        PictureType::WPG,
        PictureType::WDP,
        PictureType::SVG,
        PictureType::PCX,
        PictureType::PSD,
        PictureType::TGA,
    ];
    assert_eq!(constants.len(), 18);
    for constant in constants {
        let ext = constant.getExtension();
        let seen: Vec<&str> = ALL_18
            .iter()
            .filter(|(e, _)| *e == ext)
            .map(|(e, _)| *e)
            .collect();
        assert_eq!(seen.len(), 1, "扩展名 {ext} 在表中出现 {} 次", seen.len());
        assert!(
            PictureType::getPictureType(&ext) == Some(constant),
            "扩展名 {ext} 必须解析回同一常量（Java 顺序链的身份）"
        );
    }
}

/// G5: 非图片扩展名 ⇒ 「无」（`ContentTypes` 自身没有映射表，Java `:23` 只委托 PictureType）。
#[test]
fn g5_non_image_extensions_return_none() {
    for ext in ["xml", "rels", "bin", "docx", "txt", ""] {
        if ext.is_empty() {
            continue; // 空串由 G6 单独钉
        }
        assert_eq!(
            ContentTypes::guessContentTypeByExtension(ext),
            None,
            "扩展名 {ext} 不得命中"
        );
    }
}

/// G6: 空扩展名 ⇒ 「无」，且**不 panic**（Java `:54 StringUtil.isEmpty` / Rust `PictureType.rs:111 is_empty`）。
#[test]
fn g6_empty_extension_returns_none_without_panic() {
    assert_eq!(ContentTypes::guessContentTypeByExtension(""), None);
    assert!(PictureType::getPictureType("").is_none(), "空串 ⇒ None");
}

/// G7: 全空白扩展名 ⇒ 两侧**结果相同**（都是「无」），但判定路径不同：
/// Java `StringUtil.isEmpty` 带 `trim()` ⇒ `" ".isEmpty` 为真；Rust `is_empty()` 为假 ⇒ 落到 18 项比较。
#[test]
fn g7_whitespace_extension_returns_none() {
    assert_eq!(ContentTypes::guessContentTypeByExtension(" "), None);
    assert_eq!(ContentTypes::guessContentTypeByExtension("\t"), None);
    assert_eq!(ContentTypes::guessContentTypeByExtension("  png  "), None, "不得 trim 后再匹配");
}

/// G8: 带前导点 ⇒ **不命中**（本 function 不去点；去点是调用方的责任）。
#[test]
fn g8_leading_dot_does_not_hit() {
    assert_eq!(ContentTypes::guessContentTypeByExtension(".png"), None);
    assert_eq!(ContentTypes::guessContentTypeByExtension(".jpg"), None);
}

/// G9: 大小写不敏感（`compareExtensition` = `equalsIgnoreCase` / `eq_ignore_ascii_case`）。
#[test]
fn g9_extension_comparison_ignores_ascii_case() {
    for ext in ["PNG", "Png", "pNg"] {
        assert_eq!(
            ContentTypes::guessContentTypeByExtension(ext).as_deref(),
            Some("image/png"),
            "扩展名 {ext}"
        );
    }
    assert!(
        PictureType::getPictureType("JPEG") == Some(PictureType::JPEG),
        "大写扩展名同样命中（`equalsIgnoreCase`）"
    );
}

/// G10: 近邻拼写 ⇒ 「无」（**不得**前缀/包含匹配）。
#[test]
fn g10_near_miss_spellings_return_none() {
    for ext in ["pn", "pngg", "image/png", "png ", " jpg"] {
        assert_eq!(
            ContentTypes::guessContentTypeByExtension(ext),
            None,
            "扩展名 {ext} 不得命中"
        );
    }
}

/// G11: 纯函数 —— 同输入连跑两次结果相同（禁止缓存/全局可变表）。
#[test]
fn g11_pure_function_is_deterministic() {
    for (ext, _) in ALL_18 {
        let first = ContentTypes::guessContentTypeByExtension(ext);
        let second = ContentTypes::guessContentTypeByExtension(ext);
        assert_eq!(first, second, "扩展名 {ext} 两次结果必须相同");
    }
    // 全表跑过一遍后，单点结果不得改变（无跨调用状态）
    assert_eq!(
        ContentTypes::guessContentTypeByExtension("png").as_deref(),
        Some("image/png")
    );
}

/// 门面结构：Java `public ContentTypes()` ⇒ Rust 同名入口（与 `new` 同实现）。
#[test]
fn facade_same_named_constructor_matches_new() {
    let by_java_name = ContentTypes::ContentTypes();
    let by_rust_name = ContentTypes::new();
    // 两侧都是无状态值对象：行为一致（Java 没有 equals 覆写，故只比可观测行为）
    assert_eq!(
        ContentTypes::guessContentTypeByExtension("gif").as_deref(),
        Some("image/gif")
    );
    drop(by_java_name);
    drop(by_rust_name);
}
