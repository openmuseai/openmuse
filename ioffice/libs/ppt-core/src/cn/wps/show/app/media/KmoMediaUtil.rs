#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// M-PPT-KMOSLIDE SP2c：`show/app/media/` 原先只有 3 个 form-E 自动桩（且无 `mod.rs` ⇒ 整目录不可达）。
// 本波按 Java 真身重写 `KmoMediaData` / `KmoMediaLib` / `KmoMediaUtil` 三件并登记。
// owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c

// Java: package cn.wps.show.app.media
// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;

// Java: public class KmoMediaUtil
pub struct KmoMediaUtil;

// Java field: private static HashMap<String, Byte> mediaTypeMap = new HashMap()
//
// rust: 偏差登记 —— Java 的静态字段 + `static { }` 初始化块（45 条 `put`）在 Rust 侧由
//   `OnceLock<HashMap<&'static str, i8>>` 一次性构建承载；键为字面量 ⇒ `&'static str` 足够
//   （Java 的 `String` 键不参与身份比较，只作查表）。可见性同 Java：模块私有。
static MEDIA_TYPE_MAP: OnceLock<HashMap<&'static str, i8>> = OnceLock::new();

// Java: static { mediaTypeMap.put(...); ... }   （Java 静态初始化块，45 条 put）
fn mediaTypeMap() -> &'static HashMap<&'static str, i8> {
    MEDIA_TYPE_MAP.get_or_init(|| {
        // Java field: private static HashMap<String, Byte> mediaTypeMap = new HashMap()
        let mut mediaTypeMap: HashMap<&'static str, i8> = HashMap::new();
        // Java: mediaTypeMap.put("jpg", (byte)2);
        mediaTypeMap.insert("jpg", 2);
        // Java: mediaTypeMap.put("jpeg", (byte)2);
        mediaTypeMap.insert("jpeg", 2);
        // Java: mediaTypeMap.put("jpe", (byte)2);
        mediaTypeMap.insert("jpe", 2);
        // Java: mediaTypeMap.put("png", (byte)3);
        mediaTypeMap.insert("png", 3);
        // Java: mediaTypeMap.put("bmp", (byte)4);
        mediaTypeMap.insert("bmp", 4);
        // Java: mediaTypeMap.put("wmf", (byte)5);
        mediaTypeMap.insert("wmf", 5);
        // Java: mediaTypeMap.put("emf", (byte)6);
        mediaTypeMap.insert("emf", 6);
        // Java: mediaTypeMap.put("dib", (byte)7);
        mediaTypeMap.insert("dib", 7);
        // Java: mediaTypeMap.put("pict", (byte)9);
        mediaTypeMap.insert("pict", 9);
        // Java: mediaTypeMap.put("gif", (byte)8);
        mediaTypeMap.insert("gif", 8);
        // Java: mediaTypeMap.put("tiff", (byte)10);
        mediaTypeMap.insert("tiff", 10);
        // Java: mediaTypeMap.put("tif", (byte)10);
        mediaTypeMap.insert("tif", 10);
        // Java: mediaTypeMap.put("webp", (byte)11);
        mediaTypeMap.insert("webp", 11);
        // Java: mediaTypeMap.put("wdp", (byte)12);
        mediaTypeMap.insert("wdp", 12);
        // Java: mediaTypeMap.put("svg", (byte)13);
        mediaTypeMap.insert("svg", 13);
        // Java: mediaTypeMap.put("mp3", (byte)16);
        mediaTypeMap.insert("mp3", 16);
        // Java: mediaTypeMap.put("wma", (byte)17);
        mediaTypeMap.insert("wma", 17);
        // Java: mediaTypeMap.put("wav", (byte)18);
        mediaTypeMap.insert("wav", 18);
        // Java: mediaTypeMap.put("mid", (byte)20);
        mediaTypeMap.insert("mid", 20);
        // Java: mediaTypeMap.put("m4a", (byte)19);
        mediaTypeMap.insert("m4a", 19);
        // Java: mediaTypeMap.put("aac", (byte)21);
        mediaTypeMap.insert("aac", 21);
        // Java: mediaTypeMap.put("ogg", (byte)22);
        mediaTypeMap.insert("ogg", 22);
        // Java: mediaTypeMap.put("au", (byte)23);
        mediaTypeMap.insert("au", 23);
        // Java: mediaTypeMap.put("amr", (byte)24);
        mediaTypeMap.insert("amr", 24);
        // Java: mediaTypeMap.put("ape", (byte)25);
        mediaTypeMap.insert("ape", 25);
        // Java: mediaTypeMap.put("m4r", (byte)26);
        mediaTypeMap.insert("m4r", 26);
        // Java: mediaTypeMap.put("mmf", (byte)27);
        mediaTypeMap.insert("mmf", 27);
        // Java: mediaTypeMap.put("flac", (byte)28);
        mediaTypeMap.insert("flac", 28);
        // Java: mediaTypeMap.put("aiff", (byte)29);
        mediaTypeMap.insert("aiff", 29);
        // Java: mediaTypeMap.put("3gpp", (byte)30);
        mediaTypeMap.insert("3gpp", 30);
        // Java: mediaTypeMap.put("mp4", (byte)34);
        mediaTypeMap.insert("mp4", 34);
        // Java: mediaTypeMap.put("mov", (byte)36);
        mediaTypeMap.insert("mov", 36);
        // Java: mediaTypeMap.put("avi", (byte)35);
        mediaTypeMap.insert("avi", 35);
        // Java: mediaTypeMap.put("swf", (byte)39);
        mediaTypeMap.insert("swf", 39);
        // Java: mediaTypeMap.put("3gp", (byte)37);
        mediaTypeMap.insert("3gp", 37);
        // Java: mediaTypeMap.put("wmv", (byte)38);
        mediaTypeMap.insert("wmv", 38);
        // Java: mediaTypeMap.put("m4v", (byte)34);
        mediaTypeMap.insert("m4v", 34);
        // Java: mediaTypeMap.put("3g2", (byte)40);
        mediaTypeMap.insert("3g2", 40);
        // Java: mediaTypeMap.put("asf", (byte)41);
        mediaTypeMap.insert("asf", 41);
        // Java: mediaTypeMap.put("mpg", (byte)42);
        mediaTypeMap.insert("mpg", 42);
        // Java: mediaTypeMap.put("m2ts", (byte)43);
        mediaTypeMap.insert("m2ts", 43);
        // Java: mediaTypeMap.put("flv", (byte)44);
        mediaTypeMap.insert("flv", 44);
        // Java: mediaTypeMap.put("mkv", (byte)45);
        mediaTypeMap.insert("mkv", 45);
        // Java: mediaTypeMap.put("glb", (byte)48);
        mediaTypeMap.insert("glb", 48);
        // Java: mediaTypeMap.put("obj", (byte)49);
        mediaTypeMap.insert("obj", 49);
        // Java: （静态初始化块结束）
        mediaTypeMap
    })
}

// Java: public class KmoMediaUtil
impl KmoMediaUtil {
    // Java: KmoMediaUtil()   （Java 未显式声明构造器 ⇒ 隐式 public 无参构造器）
    pub fn new() -> Self {
        // Java: super();
        KmoMediaUtil
    }

    // Java: public static byte getMediaType(String fileExtension)
    pub fn getMediaType(fileExtension: String) -> i8 {
        // Java: fileExtension = fileExtension.toLowerCase();
        // rust: Java 重新赋值形参 ⇒ Rust 以遮蔽（shadowing）承载，语义一致。
        // rust: 偏差登记 —— Java `String.toLowerCase()` 用**默认 Locale**（如 tr_TR 下 "I"→"ı"），
        //   Rust `str::to_lowercase()` 是 Locale 无关的 Unicode 全量小写；本表的键全为 ASCII
        //   扩展名，且 Java 侧 Locale 默认即 en_US，两者结果一致。
        let fileExtension = fileExtension.to_lowercase();
        // Java: Byte mediaType = (Byte)mediaTypeMap.get(fileExtension);
        let mediaType = mediaTypeMap().get(fileExtension.as_str());
        // Java: return mediaType != null ? mediaType : 0;
        // rust: Java 条件表达式对 `Byte` 拆箱；`copied()` + `unwrap_or(0)` 等价。
        mediaType.copied().unwrap_or(0)
    }

    // Java: public static boolean isAudio(byte mediaType)
    pub fn isAudio(mediaType: i8) -> bool {
        // Java: return mediaType > 15 && mediaType < 32;
        mediaType > 15 && mediaType < 32
    }

    // Java: public static boolean isImage(byte mediaType)
    pub fn isImage(mediaType: i8) -> bool {
        // Java: return mediaType > 1 && mediaType < 14;
        mediaType > 1 && mediaType < 14
    }

    // Java: public static boolean isVideo(byte mediaType)
    pub fn isVideo(mediaType: i8) -> bool {
        // Java: return mediaType > 33 && mediaType < 46;
        mediaType > 33 && mediaType < 46
    }
}

// rust: 扩展（Java 无）—— `KmoMediaUtil` 在 Java 只有静态成员；`Default` 使
//   `..Default::default()` 装配风格可用，语义等同 `new()`。
impl Default for KmoMediaUtil {
    fn default() -> Self {
        Self::new()
    }
}
