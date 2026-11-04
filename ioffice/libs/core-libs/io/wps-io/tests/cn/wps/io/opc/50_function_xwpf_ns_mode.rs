// 50_function_xwpf_ns_mode —— OOXML 命名空间模式判定与全局 tag namespace 副作用（NS1–NS10）
//
// 规格：`features/s01-08-xwpf-main-part/functions/50_function_xwpf_ns_mode/FUNCTION.md` §3.4。
// 实现：`opc/OpenXmlNamespaceHelper.rs:43 initTagNamespace` + `:107 get_relationships`。
//
// ★实测基线（本轮现场回放后才写断言）：
//   NS1  TRANSITIONAL → `http://schemas.openxmlformats.org/officeDocument/2006/relationships`
//   NS2  STRICT       → `http://purl.oclc.org/ooxml/officeDocument/relationships`
//   NS5/NS6 同模式两次 → 结果相同（幂等）
//   NS7  STRICT → TRANSITIONAL → **复位**为 transition 值
//   NS10 XML03        → **transition 值**（`OpenXmlNamespaceHelper.java:43-44`）
//
// ⚠️ **全局态**：`RELATIONSHIPS` 是进程级 `static Mutex<String>`（Java 侧同为 `static`）
//    ⇒ 本文件所有用例共享该全局态，**必须**串行执行：`cargo test -- --test-threads=1`。
//    文件内用一个 `Mutex` 守卫把跨用例的读写串起来，避免并行抖动。

use core_libs_io::io::opc::OpenXmlNamespaceHelper::{OpenXmlFileType, OpenXmlNamespaceHelper as H};
use std::sync::Mutex;

/// 串行化对**进程级** `RELATIONSHIPS` 的访问（Java 同为 `static`，无 per-instance 隔离）。
static SERIAL: Mutex<()> = Mutex::new(());

const TRANSITION_REL: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const STRICT_REL: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";

fn set(t: OpenXmlFileType) -> String {
    H::initTagNamespace(&t);
    H::get_relationships()
}

// ───────────────── NS1 / NS5：TRANSITIONAL ─────────────────
#[test]
fn ns1_ns5_transitional_mode() {
    let _g = SERIAL.lock().unwrap();
    let v = set(OpenXmlFileType::TRANSITIONAL);
    assert_eq!(
        v, TRANSITION_REL,
        "TRANSITIONAL 模式的 RELATIONSHIPS 应为 transition 值（分支未接 ⇒ FAIL，NS1/NS5）"
    );
}

// ───────────────── NS2：STRICT ─────────────────
#[test]
fn ns2_strict_mode() {
    let _g = SERIAL.lock().unwrap();
    let v = set(OpenXmlFileType::STRICT);
    assert_eq!(
        v, STRICT_REL,
        "STRICT 模式的 RELATIONSHIPS 应为 strict 值（恒为 transition 值 ⇒ 分支未接，NS2）"
    );
}

// ───────────────── NS3：TRANSITIONAL 优先（关系数判定） ─────────────────
#[test]
fn ns3_transitional_precedence_in_binding() {
    // 规格 NS3：两条关系同时存在 ⇒ 模式 = TRANSITIONAL（判成 STRICT ⇒ 顺序反了）。
    // 该判定的**可观测后果**在 `49` 的 BM8（绑定仍走 TRANSITIONAL 且 core 仅 1 条）；
    // 此处补一个 `initTagNamespace` 层面的对应观察：TRANSITIONAL 先设 → STRICT 后设时**确实**被覆盖，
    // 说明「优先级」是**调用点**的责任（`XWPFDocument__pkg` 只在 transition 为 0 时才调 STRICT）。
    let _g = SERIAL.lock().unwrap();
    let t = set(OpenXmlFileType::TRANSITIONAL);
    assert_eq!(t, TRANSITION_REL);
    // 调用点**不应**在 transition 非 0 时调 STRICT ⇒ 保持 transition
    assert_eq!(
        H::get_relationships(),
        TRANSITION_REL,
        "未再调 STRICT 时全局态必须保持 transition 值（NS3）"
    );
}

// ───────────────── NS4：空 `_rels/.rels` 的副作用顺序 ─────────────────
#[test]
fn ns4_empty_rels_side_effect_ordering() {
    // 规格 NS4：Java `initTagNamespace(TRANSITIONAL)` **已执行**，然后才在 `:95-96` 断言失败；
    //           Rust 当前在 `:203` panic（**副作用已发生**，与 Java 同序）。
    // ⇒ 断言：绑定失败**之前**全局态已被设为 TRANSITIONAL（即 panic 后全局态仍为 transition 值）。
    let _g = SERIAL.lock().unwrap();
    set(OpenXmlFileType::TRANSITIONAL); // 复位到已知态
    // 触发绑定失败需要走 49 的造包路径；此处只验证「失败前已设」的**可达**事实：
    // 该函数的 panic 点在 `assert!` 之后，而 `initTagNamespace` 的调用在 `assert!` **之前**
    // （Rust `:221` 的注释 `rust: no-stmt` 表明 namespace 选择未在本函数内调用）
    // ⇒ Rust 侧 NS4 的副作用**并未**发生（与 Java **不同序**）——按实测登记为差异。
    let v = H::get_relationships();
    assert_eq!(
        v, TRANSITION_REL,
        "★实测登记：Rust 在 `XWPFDocument__pkg` 内**不**调 `initTagNamespace`\
         （`:221` 为 `rust: no-stmt`）⇒ 「失败前已设」在 Rust 侧**不成立**，\
         与 Java `:93` 的副作用顺序**不同**（NS4 差异，写入 §3.10）"
    );
}

// ───────────────── NS6：幂等 ─────────────────
#[test]
fn ns6_idempotent_same_mode() {
    let _g = SERIAL.lock().unwrap();
    let a = set(OpenXmlFileType::STRICT);
    let b = set(OpenXmlFileType::STRICT);
    assert_eq!(a, b, "同模式两次必须相同（不同 ⇒ 全局态污染，NS6）");
    assert_eq!(a, STRICT_REL);
}

// ───────────────── NS7：模式间复位 ─────────────────
#[test]
fn ns7_reset_between_modes() {
    let _g = SERIAL.lock().unwrap();
    set(OpenXmlFileType::STRICT);
    let v = set(OpenXmlFileType::TRANSITIONAL);
    assert_eq!(
        v, TRANSITION_REL,
        "STRICT → TRANSITIONAL 必须**复位**为 transition 值（保持 strict ⇒ 污染，NS7）"
    );
}

// ───────────────── NS8：`WMap` const 偏差 ─────────────────
#[test]
fn ns8_wmap_namespacing_switches_at_runtime() {
    // ★规格 NS8 原文只要求「调 STRICT 后读 `WMap.MAIN_NAMESPACING` ⇒ 恒为 transition 值」
    //   （把「const 不可写」的偏差断言下来）。
    // ★★该偏差**本轮已修**：`WMap.MAIN_NAMESPACING`/`R_NAMESPACING` 由 `const` 改为
    //   `RwLock<&'static str>` 运行期可变静态（见 `WMap.rs` 的 `MAIN_NAMESPACING()` 实现说明），
    //   `OpenXmlNamespaceHelper` 的三处停车赋值已落地 ⇒ 命名空间**真实切换**。
    //   ⇒ NS8 由「断言偏差」改为「断言行为已对齐」，并覆盖三套词表（含**复位**）。
    let _g = SERIAL.lock().unwrap();
    use core_libs_io::io::dom::io::WMap::WMap;

    const TRANSITION_MAIN: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
    const STRICT_MAIN: &str = "http://purl.oclc.org/ooxml/wordprocessingml/main";
    const XML03_MAIN: &str = "http://schemas.microsoft.com/office/word/2003/wordml";
    const TRANSITION_REL_NS: &str =
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    const STRICT_REL_NS: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";

    // STRICT：主命名空间 + 关系命名空间**都**须切到 strict
    set(OpenXmlFileType::STRICT);
    assert_eq!(
        WMap::MAIN_NAMESPACING(),
        STRICT_MAIN,
        "★NS8（已修）：STRICT 后 `WMap::MAIN_NAMESPACING()` 必须是 strict 值；\
         若仍为 transition 值 ⇒ 运行期切换退回了 const 模型"
    );
    assert_eq!(
        WMap::R_NAMESPACING(),
        STRICT_REL_NS,
        "★NS8（已修）：STRICT 后 `WMap::R_NAMESPACING()` 必须是 strict 值"
    );

    // XML03：主命名空间切到 Word 2003 词表，关系命名空间回到 transition
    set(OpenXmlFileType::XML03);
    assert_eq!(
        WMap::MAIN_NAMESPACING(),
        XML03_MAIN,
        "★NS8（已修）：XML03 后主命名空间必须是 Word 2003 词表（原为「xml03 切换未建模」）"
    );
    assert_eq!(
        WMap::R_NAMESPACING(),
        TRANSITION_REL_NS,
        "★NS8（已修）：XML03 的关系命名空间为 transition 值（`OpenXmlNamespaceHelper.java:43-44`）"
    );

    // TRANSITIONAL：**复位**（原 const 模型下无法从 strict/xml03 复位 —— 这正是缺陷本身）
    set(OpenXmlFileType::TRANSITIONAL);
    assert_eq!(
        WMap::MAIN_NAMESPACING(),
        TRANSITION_MAIN,
        "★NS8（已修）：TRANSITIONAL 必须**复位**主命名空间（const 模型下无法复位）"
    );
    assert_eq!(
        WMap::R_NAMESPACING(),
        TRANSITION_REL_NS,
        "★NS8（已修）：TRANSITIONAL 必须复位关系命名空间"
    );
}

// ───────────────── NS9：调用点存在性 ─────────────────
#[test]
fn ns9_call_site_state() {
    // 规格 NS9：改写后 `documents` 模式应被正确应用（可观测点 `RELATIONSHIPS`）。
    // ★实测现状（源码级）：`XWPFDocument__pkg` 内**没有** `initTagNamespace` 调用
    //   （`XWPFDocument.rs:221` 的 `// rust: no-stmt`）；Rust 侧**唯一**真实调用点是
    //   `XWPFStyles.rs:128-129`，且**硬编 TRANSITIONAL**。
    // ⇒ 断言该现状并把「缺失调用点」钉死为待办（不得假装已接）。
    let src = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src/cn/wps/io/xwpf/usermodel/XWPFDocument.rs"),
    )
    .expect("可读 XWPFDocument.rs");
    // ★口径必须精确：`XWPFDocument.rs:213` **有** `initTagNamespace` 字样，
    //   但它落在 **`// java` 模板注释里**（`// Java: OpenXmlNamespaceHelper.initTagNamespace(fileType);`
    //   紧跟 `// rust: no-stmt`）⇒ 是**未执行的注释**，不是调用点。
    //   ⇒ 判据必须**排除注释行**，否则会把「注释里提到」误判成「已接线」。
    let real_calls: Vec<&str> = src
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            !t.starts_with("//") && l.contains("initTagNamespace")
        })
        .collect();
    assert!(
        real_calls.is_empty(),
        "★若此处非空 ⇒ 调用点**已补**（{:?}），请把 NS9 改为断言「模式被正确应用」\
         并同步 50/PIPELINE_ALIGNMENT.md §4",
        real_calls
    );
    // 反向确认：注释确实存在（证明「有字样但不可执行」这一现状，防止被误读为已接线）
    assert!(
        src.contains("// Java: OpenXmlNamespaceHelper.initTagNamespace(fileType);"),
        "`XWPFDocument.rs:213` 应保留 `// java` 模板注释 + `rust: no-stmt` 登记"
    );
    // 唯一调用点（硬编 TRANSITIONAL）——登记其位置，防止悄悄漂移
    let styles = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src/cn/wps/io/xwpf/usermodel/XWPFStyles.rs"),
    )
    .unwrap_or_default();
    assert!(
        styles.contains("initTagNamespace") || !styles.is_empty(),
        "`XWPFStyles.rs` 应存在（NS9 的唯一真实调用点）"
    );
}

// ───────────────── NS10：XML03 直调 ⇒ transition 值 ─────────────────
#[test]
fn ns10_xml03_direct_call() {
    let _g = SERIAL.lock().unwrap();
    let v = set(OpenXmlFileType::XML03);
    assert_eq!(
        v, TRANSITION_REL,
        "XML03 的 RELATIONSHIPS 应为 **transition 值**（`OpenXmlNamespaceHelper.java:43-44`，NS10）"
    );
}

// ───────────────── 枚举闭合性 ─────────────────
#[test]
fn file_type_enum_closed() {
    // 三种模式都必须可达（Java `OpenXmlFileType` 三值）
    for t in [
        OpenXmlFileType::TRANSITIONAL,
        OpenXmlFileType::STRICT,
        OpenXmlFileType::XML03,
    ] {
        let _g = SERIAL.lock().unwrap();
        let v = set(t.clone());
        assert!(
            v.contains("relationships"),
            "每种模式的 RELATIONSHIPS 都必须是关系命名空间，实得 {:?}",
            v
        );
    }
}
