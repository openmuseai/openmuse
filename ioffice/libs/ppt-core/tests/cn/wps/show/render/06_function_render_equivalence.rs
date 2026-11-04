//! R15 等价性门禁（PLAN §23.9 / §26 / §27）：
//! **对 C1 的 23 页，冻结产物 B（MVP 渲染命令面）的逐页输出**。
//!
//! 目的：R15 的收口方式是"**换数据源**"（`SlideRender` 由 `slide.spTree()` 改读
//! `slide.kmo_sp_root`，真树），而唯一能保证"换源不弄坏已正确的渲染"的客观手段，就是
//! 先把**换源前**的 `RenderCommand` 序列按页冻结成常数，换源后逐页逐命令必须字节相等。
//! 本 target 即该判据的**基线**；它不依赖任何新的人工判据（PLAN §23.9 判据 2）。
//!
//! 同时它是 §27 第 2B 段（`sppr/**` 属性落点去影子）的安全网：2B 会改动
//! `sppr/fill|geometry|outline|transform` 的数据目的类型与 `pptx_mvp.rs` 的消费点，
//! 一旦影响了渲染命令，本 target 立刻变红。
//!
//! 冻结方式：每页取
//!   · `len`  —— 该页命令条数；
//!   · `digest` —— 该页命令序列 `serde_json` 字节的 FNV-1a 64 位哈希。
//! 二者都是**纯函数**于命令序列，故"换源不改行为"等价于本表逐行相等。

use ppt_core::cn::wps::show::render::mvp_pipeline::render_pptx_pages;

/// 语料 C1：23 页，真实演示文稿。
fn c1_bytes() -> Vec<u8> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../samples/ppt-render/flutter/assets/Sample_ppt.pptx"
    );
    std::fs::read(path).expect("Sample_ppt.pptx")
}

/// FNV-1a 64。选它而非 `DefaultHasher`：跨进程/跨次运行稳定，且不引入新依赖。
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// 一页的冻结指纹：命令条数 + 命令序列 JSON 字节的 FNV-1a 64。
fn page_fingerprint(cmds: &[ppt_core::cn::wps::show::render::rendercommand::RenderCommand]) -> (usize, u64) {
    let json = serde_json::to_vec(cmds).expect("serialize RenderCommand[]");
    (cmds.len(), fnv1a64(&json))
}

/// C1 的 23 页指纹基线。`len` 与 `digest` 都必须逐页相等。
///
/// 该表由 `R15_EQ_PRINT=1 cargo test --test 06_function_render_equivalence -- --nocapture` 生成。
const EXPECTED: [(usize, u64); 23] = [
    (16, 0x1d8d555c12d8e9ae), // slide1
    (141, 0x26d0b4d691e85703), // slide2
    (62, 0xf3e26bbb5bc47236), // slide3
    (86, 0x9f9e166932eada97), // slide4
    (58, 0x73d28e61616a0c85), // slide5
    (95, 0xcdca8362c1c103de), // slide6
    (50, 0x6cf9e43ef837730a), // slide7
    (134, 0xe961f0452ae29c8d), // slide8
    (62, 0x057be4329edb2726), // slide9
    (353, 0xe45174965b0ab2d8), // slide10
    (388, 0x1608e8b59abd153d), // slide11
    (419, 0x5bdf9591a54ba983), // slide12
    (15, 0xa7548c9d020c696e), // slide13
    (40, 0xed551c04e6f8487e), // slide14
    (62, 0x163a6bbc8d66a19d), // slide15
    (78, 0x0cefced5f1c00a8f), // slide16
    (49, 0xaa73ad7529970db3), // slide17
    (76, 0x1e4246db179031f2), // slide18
    (62, 0x9a47a77029961774), // slide19
    (37, 0x6e69956a1d1b1fb5), // slide20
    (65, 0xd5ea3c69c33b3fbd), // slide21
    (61, 0x03c8319c36817b30), // slide22
    (16, 0x17b717868d430156), // slide23
];

#[test]
fn r15_c1_render_pages_are_frozen() {
    let bytes = c1_bytes();
    let pages = render_pptx_pages(&bytes).expect("render_pptx_pages(C1)");
    assert_eq!(pages.len(), 23, "C1 必须是 23 页");

    let measured: Vec<(usize, u64)> = pages.iter().map(|p| page_fingerprint(p)).collect();

    if std::env::var("R15_EQ_PRINT").is_ok() {
        println!("=== R15 render-page fingerprints (C1) ===");
        for (i, (len, digest)) in measured.iter().enumerate() {
            println!("    ({}, 0x{:016x}), // slide{}", len, digest, i + 1);
        }
        println!("=== end ===");
        return;
    }

    // rust: ★基线状态说明（M-PPT-SHAPE-DISPATCH，2026-09-28）——
    //   上表 `EXPECTED` 已**恢复到 §34.4 之前的冻结值**，且当前实测**逐页逐字节相同**。
    //
    //   历史（保留以便读者理解为何曾有"基线更新"的说法）：
    //   · §34.4 曾因 `LineJoin::get_type()` 由影子语义（缺省 0）改为 Java 语义
    //     （`parent == null ? 1 : parent.type()`）而把 13 页基线"更新"过一次；
    //   · §35.4 随后又发现 6 页（slide3/9/11/12/15/19）"条数相同、指纹不同"未闭合。
    //
    //   ★BASELINE §9 的实测归因**推翻了上面两条对本 6 页的因果归属**：
    //     1) 把 `LineJoin::get_type()` 临时改回影子语义 ⇒ 指纹**逐字节不变**
    //        ⇒ `LineJoin` 的机理**不适用于当前的 6 页漂移**；
    //     2) 逐字段二分 ⇒ 唯一驱动源是 `fill_obj.solid`；
    //     3) 根因是 `RgbColorModelHexHandler` 的**转写错误**：它构造一个**游离** `RGB`、
    //        写 r/g/b 后调 `writeSrgb`，而 `writeSrgb` 走 `value.lastBin()` →
    //        `Factory::dup(this.box.get())`，**`putR/putG/putB` 只写 `map` 不刷新 `box`**
    //        ⇒ 落进 `map[3]` 的是空句柄 ⇒ `Color::srgb()` 解出 `r=g=b=0`
    //        ⇒ `solid = 0xFF000000`（黑）⇒ `apply_scheme_fill` 置 `has_fill`
    //        ⇒ 渲染多出 `DrawRect`（slide3：32 → 62 条）；
    //     4) Java 原文是 `this.color.srgb().putR(..)`（在**活** RGB 上就地写）。
    //        Rust 已逐字对齐后，**23 页全部回到本表基线**。
    //
    //   ⇒ 本表**不需要**更新：它是"修复把偏差改回 Java 语义"的证明，而不是"漂移被接受"的记录。
    //     任何后续改动若使本表变红，请先按 BASELINE §9.2 的**四步二分法**定位，
    //     不要直接重写基线。
    //
    // ★W6c-11 基线更新说明（2026-10-02，与 §34.4 同套纪律：先归因、后更新，不"接受漂移"）：
    //   `ShapePr::convert_clone` 的 **geom 槽位**此前只按 `map[3]` 拷贝，漏了 fill/ln 同款的
    //   「活对象优先」⇒ 经 `pr.geom().putPrst(..)` 写入的具体 `prstGeom` 在 harness 拷贝中被
    //   静默丢弃 ⇒ `ConvertTool::convertShapeToPublicShape` 的几何分支读不到 prst ⇒
    //   `getGraphicsPaths()` 返空 ⇒ 渲染回退到末尾的粗粒度 `DrawRect`。
    //   修复（Java-true）后该分支重新取到真实 prst（`04_function_geom_path` 11/11、
    //   `07_function_convert_read_kmo` 4/4 转绿佐证 prst→path 的正确性），
    //   `getGraphicsPaths()` 给出预设几何路径 ⇒ **9 页**（slide3/7/9/14/15/16/17/19/20）
    //   的 `DrawRect` 变为同几何的 `DrawPath`。**逐条比对证明**（273 条 DrawRect→DrawPath）：
    //   · 路径 5 点 = 原矩形外轮廓（含首尾闭合），坐标逐点相等；
    //   · `fill`/`stroke` 原值逐条相等；
    //   · 每页命令条数不变（62/50/62/40/62/78/49/62/37）。
    //   ⇒ 这是**表示层升级**（真 preset 路径替代粗矩形回退），视觉几何逐点相同，
    //   不是功能回归；上表按实测更新仅这 9 页，其余 14 页保持原冻结值。
    //   owner: tests.show.render.06_function_render_equivalence    wave: M-PPT-KMOSLIDE-W6c-11

    if EXPECTED.iter().all(|e| *e == (0, 0)) {
        panic!(
            "R15 基线表尚未填写：先用 `R15_EQ_PRINT=1 cargo test --test 06_function_render_equivalence -- --nocapture` 取表。\n\
             实测：{:?}",
            measured
        );
    }

    for (i, (want, got)) in EXPECTED.iter().zip(measured.iter()).enumerate() {
        assert_eq!(
            got.0,
            want.0,
            "slide{} 渲染命令条数变化（{} → {}）：产物 B 的输出被改动了",
            i + 1,
            want.0,
            got.0
        );
        assert_eq!(
            got.1,
            want.1,
            "slide{} 渲染命令序列字节变化（0x{:016x} → 0x{:016x}）：产物 B 的输出被改动了",
            i + 1,
            want.1,
            got.1
        );
    }
}

/// 换源等价性的**同一份输入必须可重复**：连续两次渲染必须逐页指纹相同。
///
/// 这一条防的是"指纹表恰好匹配但渲染本身不确定"（例如依赖 `HashMap` 迭代序）——
/// 那会让 R15 的等价证明失效。
#[test]
fn r15_render_is_deterministic_across_runs() {
    let bytes = c1_bytes();
    let a: Vec<(usize, u64)> = render_pptx_pages(&bytes)
        .expect("first render")
        .iter()
        .map(|p| page_fingerprint(p))
        .collect();
    let b: Vec<(usize, u64)> = render_pptx_pages(&bytes)
        .expect("second render")
        .iter()
        .map(|p| page_fingerprint(p))
        .collect();
    assert_eq!(a, b, "同一输入的两次渲染必须逐页字节一致");
}
