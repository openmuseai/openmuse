// `s0_01_function_zip_open` —— 测试镜像（⇄ `src/cn/wps/kfc/opcr/ZipFile.rs`）。
// 覆盖矩阵与 PASS/FAIL 判据见 `features/s0-01-zip-source/functions/s0_01_function_zip_open/FUNCTION.md` §3。
// 运行：cargo test --manifest-path libs/core-libs/kfc/kfc_alpha/Cargo.toml --test s0_01_function_zip_open

use core_libs_kfc::kfc::opcr::ZipFile::{ZipEntry, ZipFile};
use std::io::Cursor;
use std::path::PathBuf;

const C1: &str = "samples/ppt-render/flutter/assets/Sample_ppt.pptx";
const C2: &str = "samples/ppt-render/flutter/assets/hello.pptx";

fn repoRoot() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

fn corpus(rel: &str) -> String {
    repoRoot().join(rel).to_string_lossy().to_string()
}

fn tempDir() -> PathBuf {
    let d = std::env::temp_dir().join(format!("s0_01_zip_open_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&d);
    d
}

fn writeTemp(name: &str, bytes: &[u8]) -> String {
    let p = tempDir().join(name);
    std::fs::write(&p, bytes).expect("write temp fixture");
    p.to_string_lossy().to_string()
}

fn keys(entries: &[ZipEntry]) -> Vec<(String, i64, i64)> {
    entries
        .iter()
        .map(|e| (e.getName(), e.getSize(), e.getCompressedSize()))
        .collect()
}

/// 只有 EOCD 的「空 zip」：`PK\x05\x06` + 18 字节（共 22 字节）。
/// Java `ZipFile.java:204-205` 显式拒绝；`zip` crate 会接受 ⇒ 本条是行为对齐点。
fn eocdOnlyZip() -> Vec<u8> {
    let mut v = vec![b'P', b'K', 0x05, 0x06];
    v.extend_from_slice(&[0u8; 18]);
    v
}

/// 两个同名 entry 的 zip（Java `:241-243` 在构造期抛 `Duplicate entry name`）。
/// `zip` crate 的 `ZipWriter` 拒绝写重名，故此处手工拼字节：2 个 store 型空 entry，名字相同。
fn zipWithDuplicateNames() -> Vec<u8> {
    let name = b"dup.txt";
    let mut out: Vec<u8> = Vec::new();
    let mut offsets: Vec<u32> = Vec::new();
    for _ in 0..2 {
        offsets.push(out.len() as u32);
        out.extend_from_slice(&0x04034b50u32.to_le_bytes()); // local file header
        out.extend_from_slice(&20u16.to_le_bytes()); // version needed
        out.extend_from_slice(&0u16.to_le_bytes()); // flags
        out.extend_from_slice(&0u16.to_le_bytes()); // method = store
        out.extend_from_slice(&0u16.to_le_bytes()); // time
        out.extend_from_slice(&0u16.to_le_bytes()); // date
        out.extend_from_slice(&0u32.to_le_bytes()); // crc32（空数据）
        out.extend_from_slice(&0u32.to_le_bytes()); // compressed size
        out.extend_from_slice(&0u32.to_le_bytes()); // uncompressed size
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // extra len
        out.extend_from_slice(name);
    }
    let cd_offset = out.len() as u32;
    for off in &offsets {
        out.extend_from_slice(&0x02014b50u32.to_le_bytes()); // central directory header
        out.extend_from_slice(&20u16.to_le_bytes()); // version made by
        out.extend_from_slice(&20u16.to_le_bytes()); // version needed
        out.extend_from_slice(&0u16.to_le_bytes()); // flags
        out.extend_from_slice(&0u16.to_le_bytes()); // method
        out.extend_from_slice(&0u16.to_le_bytes()); // time
        out.extend_from_slice(&0u16.to_le_bytes()); // date
        out.extend_from_slice(&0u32.to_le_bytes()); // crc32
        out.extend_from_slice(&0u32.to_le_bytes()); // compressed size
        out.extend_from_slice(&0u32.to_le_bytes()); // uncompressed size
        out.extend_from_slice(&(name.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // extra len
        out.extend_from_slice(&0u16.to_le_bytes()); // comment len
        out.extend_from_slice(&0u16.to_le_bytes()); // disk number start
        out.extend_from_slice(&0u16.to_le_bytes()); // internal attrs
        out.extend_from_slice(&0u32.to_le_bytes()); // external attrs
        out.extend_from_slice(&off.to_le_bytes()); // local header offset
        out.extend_from_slice(name);
    }
    let cd_size = out.len() as u32 - cd_offset;
    out.extend_from_slice(&0x06054b50u32.to_le_bytes()); // EOCD
    out.extend_from_slice(&0u16.to_le_bytes()); // disk number
    out.extend_from_slice(&0u16.to_le_bytes()); // disk with cd
    out.extend_from_slice(&2u16.to_le_bytes()); // entries on this disk
    out.extend_from_slice(&2u16.to_le_bytes()); // total entries
    out.extend_from_slice(&cd_size.to_le_bytes());
    out.extend_from_slice(&cd_offset.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes()); // comment len
    out
}

// ── T01/T02 正向：真实语料两个规模档 ───────────────────────────────────────────
#[test]
fn t01_open_path_c2_minimal_package() {
    let zf = ZipFile::from_path(&corpus(C2)).expect("C2 hello.pptx 应打开成功");
    let entries = zf.entries();
    assert!(!entries.is_empty(), "C2 应有 entry");
    assert!(entries.iter().all(|e| !e.getName().is_empty()));
}

#[test]
fn t02_open_path_c1_real_package() {
    let zf = ZipFile::from_path(&corpus(C1)).expect("C1 Sample_ppt.pptx 应打开成功");
    let entries = zf.entries();
    assert!(entries.len() > 1, "C1 应有多个 entry，实测 {}", entries.len());
    let first = entries[0].getName();
    let again = zf.getEntry(&first).expect("枚举出的 entry 必须可再取到");
    assert_eq!(again.getName(), first);
    assert_eq!(again.getSize(), entries[0].getSize());
}

// ── T03 §1.6 不变量 5：三种后端产出等价句柄 ───────────────────────────────────
#[test]
fn t03_backends_produce_equivalent_handles() {
    let path = corpus(C2);
    let viaPath = ZipFile::from_path(&path).expect("path 后端");
    let bytes = std::fs::read(&path).expect("读 C2 字节");
    let mut cur = Cursor::new(bytes.clone());
    let viaRaf = ZipFile::from_reader(&mut cur).expect("raf 后端");
    let mut cur2 = Cursor::new(bytes);
    let viaNew = ZipFile::ZipFile__RandomAccessFile(&mut cur2).expect("Java `ZipFile(RandomAccessFile)` 门面");
    assert_eq!(keys(&viaPath.entries()), keys(&viaRaf.entries()), "path ⇄ raf 必须逐字段相同");
    assert_eq!(keys(&viaPath.entries()), keys(&viaNew.entries()), "path ⇄ 门面 ctor 必须逐字段相同");
}

// ── T04–T08 失败面：必须 Err、可分类、不 panic ────────────────────────────────
#[test]
fn t04_zero_byte_file_rejected() {
    let p = writeTemp("zero.pptx", b"");
    let e = ZipFile::from_path(&p).err().expect("0 字节必须失败");
    assert!(
        e.to_string().contains("File too short to be a zip file: 0"),
        "实测消息：{}",
        e
    );
}

#[test]
fn t05_plain_text_rejected() {
    let p = writeTemp("text.pptx", b"this is not a zip archive, just plain ASCII text padding.");
    let e = ZipFile::from_path(&p).err().expect("非 zip 必须失败");
    assert!(e.to_string().contains("Not a zip archive"), "实测消息：{}", e);
}

#[test]
fn t06_eocd_only_empty_zip_rejected_like_java() {
    let p = writeTemp("empty.pptx", &eocdOnlyZip());
    let e = ZipFile::from_path(&p).err().expect("空 zip 必须失败（Java `:204-205` 口径）");
    assert!(
        e.to_string().contains("Empty zip archive not supported"),
        "实测消息：{}",
        e
    );
}

#[test]
fn t07_missing_path_is_io_error() {
    let p = tempDir().join("no_such_file_9f3c.pptx").to_string_lossy().to_string();
    let e = ZipFile::from_path(&p).err().expect("不存在路径必须失败");
    assert_eq!(e.kind(), std::io::ErrorKind::NotFound, "实测：{:?}", e.kind());
}

#[test]
fn t08_directory_path_rejected() {
    let d = tempDir().to_string_lossy().to_string();
    assert!(ZipFile::from_path(&d).is_err(), "目录路径必须失败");
}

#[test]
fn t09_truncated_c2_rejected() {
    let bytes = std::fs::read(corpus(C2)).expect("读 C2 字节");
    let cut = bytes.len() * 2 / 3;
    let p = writeTemp("truncated.pptx", &bytes[..cut]);
    let e = ZipFile::from_path(&p).err().expect("截断包必须失败");
    assert!(!e.to_string().is_empty());
}

#[test]
fn t15_duplicate_entry_name_collapsed_by_zip_crate() {
    // ★实测：Java `:241-243` 在构造期抛 `Duplicate entry name`；`zip` crate 2.4.2 以名字为键
    // 把重名折叠为 1 条（`ZipArchive::len()` = 1）⇒ S0 轨无法复现 Java 的拒绝语义。
    // 本条断言的是 **S0 轨实测行为**，差异已登记（docs/s0/s0_facade_map.csv S0M58）。
    let bytes = zipWithDuplicateNames();
    let mut cur = Cursor::new(bytes);
    let zf = ZipFile::from_reader(&mut cur).expect("S0 轨实测：重名包打开成功");
    assert_eq!(zf.entries().len(), 1, "重名被折叠为 1 条（实测）");
}

// ── T10 关闭语义：所有克隆同时失效 ────────────────────────────────────────────
#[test]
fn t10_close_invalidates_all_clones() {
    let mut zf = ZipFile::from_path(&corpus(C2)).expect("C2 应打开成功");
    let mut clone = zf.clone();
    assert!(!zf.entries().is_empty());
    zf.close();
    assert!(zf.entries().is_empty(), "本体在 close 后必须失效");
    assert!(clone.entries().is_empty(), "克隆必须与本体同时失效（共享缓冲被清空）");
    assert!(clone.getEntry("任何名字").is_none());
    clone.close();
}

// ── T11 G2：同输入重复打开必须逐字段相同 ─────────────────────────────────────
#[test]
fn t11_repeat_open_is_deterministic() {
    let a = ZipFile::from_path(&corpus(C1)).expect("第一次打开");
    let b = ZipFile::from_path(&corpus(C1)).expect("第二次打开");
    assert_eq!(keys(&a.entries()), keys(&b.entries()));
}

// ── T12 门面 ctor 等价（Java 4 个构造出口 ⇄ S0 轨出口） ───────────────────────
#[test]
fn t12_facade_ctors_equivalent() {
    let path = corpus(C2);
    let baseline = keys(&ZipFile::from_path(&path).expect("path").entries());
    let viaNew = keys(&ZipFile::ZipFile__File(PathBuf::from(&path).as_path()).expect("new(File)").entries());
    let viaString = keys(&ZipFile::ZipFile__String(&path).expect("new(String)").entries());
    let viaMode = keys(
        &ZipFile::ZipFile__File_int(PathBuf::from(&path).as_path(), 1)
            .expect("new(File,int)")
            .entries(),
    );
    assert_eq!(baseline, viaNew);
    assert_eq!(baseline, viaString);
    assert_eq!(baseline, viaMode);
}

// ── T13/T14 OPEN_DELETE 与非法 mode（Java `:55-70`） ─────────────────────────
#[test]
fn t13_open_delete_mode_removes_file_on_close() {
    let src = std::fs::read(corpus(C2)).expect("读 C2 字节");
    let keep = writeTemp("keep.pptx", &src);
    let del = writeTemp("del.pptx", &src);

    let mut zf = ZipFile::ZipFile__File_int(PathBuf::from(&keep).as_path(), 1).expect("OPEN_READ");
    zf.close();
    assert!(std::path::Path::new(&keep).exists(), "OPEN_READ 关闭后不得删除源文件");

    let mut zf = ZipFile::ZipFile__File_int(PathBuf::from(&del).as_path(), 1 | 4).expect("OPEN_READ|OPEN_DELETE");
    zf.close();
    assert!(!std::path::Path::new(&del).exists(), "OPEN_DELETE 关闭后必须删除");
}

#[test]
fn t14_bad_mode_rejected() {
    let e = ZipFile::ZipFile__File_int(PathBuf::from(corpus(C2)).as_path(), 2).err().expect("mode=2 非法");
    assert_eq!(e.kind(), std::io::ErrorKind::InvalidInput);
    assert!(e.to_string().contains("Bad mode: 2"), "实测消息：{}", e);
}

// ── T16 §1.6 不变量 1：打开是只读的，不改写源文件 ─────────────────────────────
#[test]
fn t16_open_does_not_mutate_source_file() {
    let path = corpus(C2);
    let before = std::fs::read(&path).expect("读前字节");
    let _ = ZipFile::from_path(&path).expect("打开");
    let after = std::fs::read(&path).expect("读后字节");
    assert_eq!(before, after, "打开/析构不得改写源文件");
}

// ── T17 C18 类输入：OLE/CFB 复合文档头 ───────────────────────────────────────
#[test]
fn t17_ole_cfb_header_rejected() {
    // Java `:207-208` 把 OLE 头判为非 zip ⇒ ZipException，上层转加密分支；
    // `encrypted` 的 kind 归一归 s0-12（W5）。本 function 只要求**不静默成功**。
    let mut bytes = vec![0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
    bytes.extend_from_slice(&[0u8; 512]);
    let p = writeTemp("ole.pptx", &bytes);
    let e = ZipFile::from_path(&p).err().expect("OLE 头必须失败（不得静默成功）");
    assert!(e.to_string().contains("Not a zip archive"), "实测消息：{}", e);
}
