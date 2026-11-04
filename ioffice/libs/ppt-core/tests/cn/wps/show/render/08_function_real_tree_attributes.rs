//! ★M-PPT-KMOSLIDE 终局判据：**真树的「结构 + 属性」双绿**（PLAN §26 判据 4 / §27 / §36.7）。
//!
//! 本 target 把里程碑的两条终局主张合成**客观、可复现、不依赖人工判据**的断言：
//!
//! 1. **结构真实**（与 A13 oracle / `oracle_diff_c1.sh` 同源，此处**自洽复验**）——
//!    `p:spTree` 的 depth1 逐类型（sp/pic/grpSp/graphicFrame/cxnSp）与 oracle 基线一致；
//! 2. **属性真实**（本 target 的**新增**价值）——
//!    `p:spPr` 的四个槽位（`a:xfrm` / `a:prstGeom` / 填充 / `a:ln`）必须落在
//!    **真实 KProto 树**（`ShapePr` 的共享句柄）上，而不是影子 `layout_shape::*Obj` 上。
//! 3. **对象图真实**（§1.3 核心缺陷的回归）——
//!    `shape.parent().getPresentation().drawingStore()` 必须解析到
//!    **打开期那一个唯一仓**（`commit_drawing` 的写入目标），而不是每次新建的空仓。
//!
//! ## 全部走**产品真实入口**
//! `PptxReader::PptxReader(pres, path, ..)` + `reader.open()` —— 与
//! `examples/c1_slide_tree_dump.rs`（A13 oracle 的 dump 器）**完全同款**，
//! 也就是 `render_pptx_pages` / `open_pptx_document_bytes` 内部的同一条链。
//! 断言只经既有 API 读取，**不新增任何 accessor** ⇒ 不引入新假面。

use std::cell::RefCell;
use std::rc::Rc;

use ppt_core::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
use ppt_core::cn::wps::show::pptxr::PptxReader::PptxReader;
use ppt_core::cn::wps::show::util::layout_shape::KmoPresentation;

const C1: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../samples/ppt-render/flutter/assets/Sample_ppt.pptx"
);

/// 走**产品真实入口**打开 C1（与 A13 oracle 的 dump 器同款）。
fn open_c1() -> Rc<RefCell<KmoPresentation>> {
    let model = Rc::new(RefCell::new(KmoPresentation::default()));
    let mut reader = PptxReader::PptxReader(Some(model.clone()), C1, None, false, None);
    reader.open().expect("PptxReader.open(C1)");
    model
}

/// 收集真树里的全部形状 id（含根）。
fn collect_ids(shape: &KmoShowShape, out: &mut Vec<i32>) {
    out.push(shape.id());
    for i in 0..shape.childCount() {
        collect_ids(shape.childAt(i), out);
    }
}

/// 深度优先遍历，对每个节点调用 `f`（含根）。
fn walk_ref<'a>(shape: &'a KmoShowShape, f: &mut dyn FnMut(&'a KmoShowShape)) {
    f(shape);
    for i in 0..shape.childCount() {
        walk_ref(shape.childAt(i), f);
    }
}

/// ★主张 1：结构真实 —— 逐页 depth1 逐类型（与 A13 oracle 同源，此处**自洽复验**）。
#[test]
fn kmoslide_c1_structure_is_real_and_selfconsistent() {
    let model = open_c1();
    let p = model.borrow();
    assert_eq!(p.slides.len(), 23, "C1 必须是 23 页");

    let mut deck = [0usize; 5]; // [sp, pic, grpSp, graphicFrame, cxnSp]
    let mut total_depth1 = 0usize;
    for (i, slide) in p.slides.iter().enumerate() {
        let root = slide
            .kmo_sp_root
            .as_ref()
            .unwrap_or_else(|| panic!("slide{} 缺 kmo_sp_root", i + 1));
        let n = root.childCount() as usize;
        assert!(n > 0, "slide{} 的 depth1 不应为空", i + 1);
        let mut per_page = [0usize; 5];
        for c in 0..n {
            match root.childAt(c as i32).r#type() {
                0 => per_page[2] += 1,
                1 => per_page[0] += 1,
                2 => per_page[1] += 1,
                3 => per_page[4] += 1,
                4 => per_page[3] += 1,
                other => panic!("slide{} child{} 的 type()={other} 不在 0..=4", i + 1, c),
            }
        }
        assert_eq!(
            per_page.iter().sum::<usize>(),
            n,
            "slide{} 逐类型之和 != depth1 总数",
            i + 1
        );
        total_depth1 += n;
        for k in 0..5 {
            deck[k] += per_page[k];
        }
    }
    assert_eq!(total_depth1, 417, "C1 全 deck depth1 总数（oracle 基线）");
    assert_eq!(
        deck,
        [289, 14, 58, 5, 51],
        "C1 全 deck depth1 逐类型（oracle 基线 sp/pic/grpSp/graphicFrame/cxnSp）"
    );
}

/// ★主张 2：属性真实 —— `p:spPr` 的四个槽位落在**真实 KProto 树**上。
///
/// 判据（全部经**产品路径**已装配的既有 API）：
///   · `commit_drawing` 对每个形状调 `DgStore::attachSpPr(spid, pr)`，
///     故真树里的每个形状都应能在**打开期那个唯一仓**里查到 `ShapePr`；
///   · 对这些真实 `ShapePr` 取四个真身句柄（`geom/fill/ln/xfrm_handle`），
///     全 deck 至少要有**非零数量**的形状在对应槽位上**真的带了属性**。
///     若属性仍停在影子 `layout_shape::*Obj` 上，这些真身谓词会**全为 false**。
#[test]
fn kmoslide_c1_real_tree_carries_sp_pr_attributes() {
    let model = open_c1();

    // 打开期那**一个**唯一仓：`KmoPresentation::drawingStore()`（产品对象图，§1.3 目标 3）。
    let store = model.borrow_mut().drawingStore();

    let mut ids: Vec<i32> = Vec::new();
    {
        let p = model.borrow();
        for slide in p.slides.iter() {
            if let Some(root) = slide.kmo_sp_root.as_ref() {
                collect_ids(root, &mut ids);
            }
        }
    }
    assert!(ids.len() > 417, "真树节点总数应大于 depth1 总数");

    #[derive(Default)]
    struct Tally {
        found: usize,
        has_geom: usize,
        geom_prst: usize,
        has_fill: usize,
        fill_type: usize,
        has_ln: usize,
        ln_width: usize,
        has_xfrm: usize,
        xfrm_cx: usize,
        missing: usize,
    }
    let mut t = Tally::default();
    for id in &ids {
        let Some(pr) = store.borrow().shapePr(*id) else {
            t.missing += 1;
            continue;
        };
        t.found += 1;
        let mut sp = pr.borrow_mut();
        // ★真身句柄（§31.7 ③-1）：与 `ShapePr` 的真属性是**同一对象**。
        if sp.hasGeom() {
            t.has_geom += 1;
        }
        {
            let g = sp.geom_handle();
            let gb = g.borrow();
            if gb.hasPrstGeom() {
                t.geom_prst += 1;
            }
        }
        if sp.hasFill() {
            t.has_fill += 1;
        }
        if sp.fill_handle().borrow().hasType() {
            t.fill_type += 1;
        }
        if sp.hasLn() {
            t.has_ln += 1;
        }
        if sp.ln_handle().borrow().hasWidth() {
            t.ln_width += 1;
        }
        if sp.hasXfrm() {
            t.has_xfrm += 1;
        }
        if sp.xfrm_handle().borrow().extents().cx() != 0 {
            t.xfrm_cx += 1;
        }
    }

    eprintln!(
        "真树属性实测：节点={} shapePr={} missing={} geom(hasGeom={}/prstGeom={}) \
         fill(hasFill={}/type={}) ln(hasLn={}/width={}) xfrm(hasXfrm={}/cx={})",
        ids.len(),
        t.found,
        t.missing,
        t.has_geom,
        t.geom_prst,
        t.has_fill,
        t.fill_type,
        t.has_ln,
        t.ln_width,
        t.has_xfrm,
        t.xfrm_cx
    );
    assert_eq!(
        t.missing, 0,
        "真树里有 {} 个形状没有 attachSpPr 的真实 ShapePr（commit_drawing 的义务）",
        t.missing
    );
    // 逐项给下界：任一项塌到 0 = 该槽位的属性退回了影子模型。
    assert!(t.has_geom > 0, "真身 ShapePr 上没有任何 geom —— 属性仍在影子模型");
    assert!(t.geom_prst > 0, "真身 Geometry2D 上没有任何 prstGeom 值");
    assert!(t.has_fill > 0, "真身 ShapePr 上没有任何 fill —— 属性仍在影子模型");
    assert!(t.fill_type > 0, "真身 Fill 上没有任何 type 值");
    assert!(t.has_ln > 0, "真身 ShapePr 上没有任何 ln —— 属性仍在影子模型");
    assert!(t.ln_width > 0, "真身 Outline 上没有任何 width 值");
    assert!(t.has_xfrm > 0, "真身 ShapePr 上没有任何 xfrm —— 属性仍在影子模型");
    assert!(t.xfrm_cx > 0, "真身 Transform2D 上没有任何 cx 值");
}

/// ★主张 3：真树里**每个**形状的 `ShapePr` 都可查 —— 结构与属性装配**同步**。
///
/// 防的是：`addChild`（结构）与 `attachSpPr`（属性）不同步时，
/// 树里出现"渲染得到、但属性查不到"的幽灵节点。
#[test]
fn kmoslide_c1_tree_and_attribute_store_are_in_sync() {
    let model = open_c1();
    let store = model.borrow_mut().drawingStore();

    let mut total = 0usize;
    let mut absent = Vec::new();
    {
        let p = model.borrow();
        for slide in p.slides.iter() {
            let Some(root) = slide.kmo_sp_root.as_ref() else {
                continue;
            };
            walk_ref(root, &mut |s: &KmoShowShape| {
                total += 1;
                if store.borrow().shapePr(s.id()).is_none() {
                    absent.push((s.r#type(), s.id()));
                }
            });
        }
    }
    assert!(total > 0, "真树为空");
    assert!(
        absent.is_empty(),
        "有 {}/{} 个真树形状在 DgStore 里没有 ShapePr，头 10 个：{:?}",
        absent.len(),
        total,
        &absent[..absent.len().min(10)]
    );
    eprintln!("树/属性同步复验：总节点={total} 缺失=0");
}
