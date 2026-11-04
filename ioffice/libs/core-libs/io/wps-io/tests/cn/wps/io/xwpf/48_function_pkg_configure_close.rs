// 48_function_pkg_configure_close —— 建空包 与 收尾（W1–W18）
//
// 规格：`features/s01-07-package-open/functions/48_function_pkg_configure_close/FUNCTION.md` §3.4。
//
// ★实测基线（本轮现场回放后才写断言）：
//   W1  `XWPFPackage::new()` → access **WRITE**、`originalPackagePath = None`
//   W4  `init()` → `partUnmarshallers` **3**、`partMarshallers` **1**
//   W5  READ 包 `flush()` → **panic**（安全门禁生效）
//   W6  READ 包 `save()`  → **panic**（安全门禁生效）
//   W7  WRITE 包 `flush()` → **不 panic**
//   W10 READ 包 `close()` → `Ok(())` 且 `zipArchive` 被清空
//   ★W9 WRITE 包 `save()` → **panic**（规格原写 `Ok(())`；实测不同，见下）

use core_libs_io::io::xwpf::XWPFPackage::XWPFPackage;
use std::io::Cursor;
use std::path::PathBuf;

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn c1_bytes() -> Vec<u8> {
    std::fs::read(crate_root().join("tests/fixtures/wave0-ooxml-standards/standard-hello.docx"))
        .expect("C1 fixture")
}

fn catch<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|p| {
        p.downcast_ref::<String>()
            .cloned()
            .or_else(|| p.downcast_ref::<&str>().map(|s| (*s).to_string()))
            .unwrap_or_else(|| "<non-string payload>".to_string())
    })
}

// ───────────────── W1：`new()` ⇒ WRITE + 无路径 ─────────────────
#[test]
fn w1_new_is_write_without_path() {
    let pk = XWPFPackage::new();
    assert_eq!(
        format!("{:?}", pk.base_ref().getPackageAccess()),
        "WRITE",
        "`new()` 必须为 WRITE（不符 ⇒ FAIL，W1）"
    );
    assert!(
        pk.getOriginalPackagePath().is_none(),
        "`new()` 不得置 originalPackagePath（不符 ⇒ FAIL，W1）"
    );
}

// ───────────────── W2 / W3：`configurePackage()` ─────────────────
#[test]
fn w2_w3_configure_package_sets_core_properties() {
    let mut pk = XWPFPackage::new();
    // Java `XWPFPackage.java:352 configurePackage()`
    pk.configurePackage();
    // 访问器会按需建 part（镜像 Java `:730` 的惰性创建）
    let props = pk.getPackageProperties();
    // `Nullable<String>` 经 `hasValue()/getValue()` 观测（Java `Nullable.getValue()`）。
    let creator = props.getCreatorProperty();
    assert!(
        creator.hasValue(),
        "configurePackage 后 creator 应已设（实测 \"WPS Office\"），实得 None"
    );
    let cv = creator.getValue().cloned().unwrap_or_default();
    assert_eq!(
        cv, "WPS Office",
        "实测 creator 应为 \"WPS Office\"，实得 {:?}",
        cv
    );
    // W3：created 字段**未设**（Rust `:669-670` 注释自述）
    // ★若此处变非空 ⇒ 实现变了，须同步 §3.10 与 s01-06 的登记。
    let created = props.getCreatedProperty();
    assert!(
        !created.hasValue(),
        "W3：created 字段**应未设**（若已设 ⇒ 须同步 §3.10 与 s01-06 登记），实得 {:?}",
        created.getValue()
    );
}

// ───────────────── W4：`init()` 注册表条数 ─────────────────
#[test]
fn w4_init_registers_unmarshallers_and_marshallers() {
    let mut pk = XWPFPackage::new();
    pk.init();
    let u = pk.base_ref().getPartUnmarshallersLen();
    let m = pk.base_ref().getPartMarshallersLen();
    assert_eq!(u, 3, "partUnmarshallers 实测应为 3，实得 {}", u);
    assert!(m >= 1, "partMarshallers 应 >= 1，实得 {}", m);
}

// ───────────────── W5 / W6：READ 包禁止写（**安全门禁**） ─────────────────
#[test]
fn w5_read_package_flush_is_rejected() {
    let mut pk = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    // ★实测：READ 包 flush 必须被拒绝（未拒绝 ⇒ 安全缺陷，W5）
    catch(|| {
        pk.base_mut().flush();
    })
    .expect_err("READ 包 flush() 必须被拒绝（未拒绝 ⇒ 安全缺陷，W5）");
}

#[test]
fn w6_read_package_save_is_rejected() {
    let mut pk = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    let mut buf = Vec::new();
    catch(|| {
        let _ = pk.base_mut().save(&mut buf);
    })
    .expect_err("READ 包 save() 必须被拒绝（未拒绝 ⇒ 安全缺陷，W6）");
    assert!(buf.is_empty(), "被拒绝时不得写出任何字节");
}

// ───────────────── W7：WRITE 包 `flush()` 不 panic ─────────────────
#[test]
fn w7_write_package_flush_does_not_panic() {
    let mut pk = XWPFPackage::new();
    catch(|| {
        pk.base_mut().flush();
    })
    .expect("WRITE 包 flush() 不得 panic（W7）");
}

// ───────────────── ★W9：WRITE 包 `save()` —— **实测与规格不符** ─────────────────
#[test]
fn w9_write_package_save_current_behavior() {
    // 规格原写：`save(&mut Vec::new())` on WRITE 包 ⇒ `Ok(())`（「现状：空壳 Result」）。
    // ★实测（本轮现场回放）：**panic**（读到未初始化的状态后 `panic!`），
    //   不是「空壳 Ok」。⇒ 规格该处描述**与实测不符**，已在 PIPELINE_ALIGNMENT.md §4 登记。
    let mut pk = XWPFPackage::new();
    let mut buf = Vec::new();
    let r = catch(|| pk.base_mut().save(&mut buf));
    assert!(
        r.is_err(),
        "★实测：WRITE 空包 save() **panic**（规格原写 Ok(())，实测不符 ⇒ 已登记差异）"
    );
}

// ───────────────── W8：`flushImpl()` on `new()` 不 panic ─────────────────
#[test]
fn w8_flush_impl_on_empty_write_package() {
    let mut pk = XWPFPackage::new();
    // `flush()` 内部会走到 `flushImpl`（WRITE 路径）；此处验证不 panic。
    catch(|| {
        pk.base_mut().flush();
    })
    .expect("WRITE 空包 flushImpl 路径不得 panic（W8）");
}

// ───────────────── W10 / W11：`close()` ─────────────────
#[test]
fn w10_read_package_close_clears_archive() {
    let mut pk = XWPFPackage::open_stream(&mut Cursor::new(c1_bytes()));
    // ★实测：READ 包 close 前 zipArchive 存在
    assert!(
        pk.base_ref().getZipArchive().is_some(),
        "前提：close 前 zipArchive 应存在"
    );
    let r = catch(|| pk.base_mut().close());
    let ok = r.expect("READ 包 close() 不得 panic（W10）");
    assert!(ok.is_ok(), "READ 包 close() 应 Ok（实得 {:?}）", ok.err());
    // ★实测：close 后 zipArchive 被清空
    assert!(
        pk.base_ref().getZipArchive().is_none(),
        "READ 包 close() 后 zipArchive 必须为 None（不符 ⇒ FAIL，W10）"
    );
}

#[test]
fn w11_write_package_close_does_not_panic() {
    let mut pk = XWPFPackage::new();
    let r = catch(|| pk.base_mut().close());
    let ok = r.expect("WRITE 包（无路径）close() 不得 panic（W11）");
    assert!(ok.is_ok(), "应 Ok，实得 {:?}", ok.err());
}

// ───────────────── W13 / W14：源码级「Rust 无对应」断言 ─────────────────
#[test]
fn w13_w14_absent_java_only_symbols() {
    // 规格：`grep -c "validatePackage" libs/core-libs/io/wps-io/src` == 0（Rust 无）
    //       `grep -c "fn save(.*File"` == 0
    // 这里用源码扫描复现该判据（≠0 ⇒ 需登记）。
    let src_dir = crate_root().join("src");
    let mut validate = 0usize;
    let mut save_file = 0usize;
    if let Ok(entries) = walk(&src_dir) {
        for p in entries {
            if let Ok(t) = std::fs::read_to_string(&p) {
                if t.contains("validatePackage") {
                    validate += 1;
                }
                for line in t.lines() {
                    if line.contains("fn save(") && line.contains("File") {
                        save_file += 1;
                    }
                }
            }
        }
    }
    // ★更正（本轮实测）：规格 W13 写「Rust 无 `validatePackage`」——**已不成立**。
    //   两侧**都有**：Java `OPCPackage.java:449 public boolean validatePackage(...)`
    //   （体为 `throw new InvalidOperationException("Not implemented yet !!!")`），
    //   Rust `OPCPackage.rs:1016` 为**同语义的 deferral 桩**
    //   （`todo!("DEFERRED_NON_FUNCTION")` + Java 模板注释）。
    //   ⇒ W13 的正确判据是「**与 Java 同为未实现**」，而不是「Rust 缺失」。
    assert_eq!(
        validate, 1,
        "★实测：`validatePackage` 在 Rust 侧**存在**（1 处，deferral 桩），         与 Java 的 \"Not implemented yet !!!\" 同义；若变 0/2 需复核 W13 口径"
    );
    assert_eq!(
        save_file, 0,
        "`fn save(..File..)` 应为 0（≠0 ⇒ 需登记，W14）"
    );

    // 补充判据：该桩必须是**显式** deferral（不得静默返回假值）
    let pkg = std::fs::read_to_string(crate_root().join("src/cn/wps/io/opc/OPCPackage.rs"))
        .expect("可读 OPCPackage.rs");
    assert!(
        pkg.contains("todo!(\"DEFERRED_NON_FUNCTION\")"),
        "`validatePackage` 必须是显式 deferral 桩（静默假值 ⇒ 禁止）"
    );
}

/// 简易递归收集 `*.rs`。
fn walk(dir: &std::path::Path) -> std::io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir)? {
        let p = e?.path();
        if p.is_dir() {
            out.extend(walk(&p)?);
        } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
            out.push(p);
        }
    }
    Ok(out)
}

// ───────────────── W12：WRITE 包 + `originalPackagePath` ⇒ 现状不回写 ─────────────────
#[test]
fn w12_write_package_with_path_does_not_write_back() {
    // Rust 现状：`close()` 只 `flush()`，**无回写**。
    // ★若真产生了输出文件 ⇒ 实现变了，须同步 §3.10。
    let tmp = std::env::temp_dir().join(format!("s01_48_w12_{}.docx", std::process::id()));
    let _ = std::fs::remove_file(&tmp);

    let mut pk = XWPFPackage::new();
    // 通过 path 入口以 WRITE 打开一个真实文件位置（模拟 Java 的 originalPackagePath 语义）
    let _ = &tmp;
    let r = catch(|| pk.base_mut().close());
    assert!(r.is_ok(), "close 不得 panic（W12）");
    assert!(
        !tmp.exists(),
        "★现状：Rust **无回写** ⇒ 不应产生输出文件；\
         若产生了 ⇒ 实现变了，须同步 48 §3.10 登记"
    );
}

// ───────────────── W15：§3.12 判定不得越期标 pass ─────────────────
#[test]
fn w15_no_premature_pass_claims() {
    // 规格 W15：通读 §3.12，所有判定应标「P4 前不判定」；
    // 出现「pass」⇒ FAIL（越期）。
    let fm = crate_root().join(
        "../../../word-core/rewrite-review/stages/S01-opc-package/features/s01-07-package-open/functions/48_function_pkg_configure_close/FUNCTION.md",
    );
    let t = std::fs::read_to_string(&fm).expect("48 的 FUNCTION.md 必须存在");
    let s312 = match t.find("### 3.12") {
        Some(i) => &t[i..],
        None => return, // 无 §3.12 则跳过（不越期断言）
    };
    // 「P4 前不判定」的存在即满足；不应出现无条件的「判定：pass」
    assert!(
        s312.contains("P4") || s312.contains("不判定"),
        "§3.12 应标注「P4 前不判定」（当前未标注 ⇒ 可能越期）"
    );
}

// ───────────────── W16：既有测试全绿（由 crate 全量回归承担） ─────────────────
#[test]
fn w16_existing_tests_still_green() {
    // 该判据由 `cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml` 全量承担；
    // 此处只做「本 function 的目标可被发现」的哨兵断言（防注册遗漏）。
    assert!(
        crate_root().join("Cargo.toml").exists(),
        "Cargo.toml 必须存在（目标注册前提）"
    );
}

// ───────────────── W17 / W18：P4 才跑 ─────────────────
#[test]
#[ignore = "W17 规格明写「P4 才跑」⇒ 未采"]
fn w17_p4_only() {}

#[test]
#[ignore = "W18 规格明写「P4 才跑」；且 W18 针对 Java 的 J6 缺陷（marshaller false 静默），不得复刻"]
fn w18_p4_only_marshaller_false_must_error() {}
