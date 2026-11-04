//! 05_function_slide_handler — 真实 `SlideHandler` 的 p:sld 装配。
//!
//! W5 测试面收口（见 M_PPT_SLIDE_ASM_BASELINE.md §NN）：原文件用影子 crate
//! （`pub mod cn { ... }` + `#[path]` 包含 `src`）编译一份按 `.codeTrees` 伪设计写的
//! `SlideHandler`：假 `KmoSlide`、假 `DgStore.updates`、假 `save_count`。
//! shipped `SlideHandler` 改走 `ImportHost` + 真实 `KmoShowShape` 树 + 真实
//! `dmlr::handler::ElementHandler`，影子 crate 里那些依赖类型已不存在。
//!
//! 因此本 target 按真实 API 重写：用真实 `ImportHost::leak()` / 真实 `XmlRoAttrList`
//! 驱动真实 `SlideHandler`，断言只描述真实可观测的状态（`take_root` / `cnvpr_count` /
//! `show_master_sp` / 宿主 `DgStore`）。

#![allow(non_snake_case, dead_code, unused_imports)]

use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use ppt_core::cn::wps::drawing::dmlr::handler::ElementHandler;
use ppt_core::cn::wps::kso::kproto::KoBin::Null;
use ppt_core::cn::wps::show::pptxr::handler::shape::SwGroupShapeHandler::ImportHost;
use ppt_core::cn::wps::show::pptxr::handler::slides::SlideHandler::SlideHandler;

/// 真实 `XmlRoAttrList`：`p:sld` 的 `startElement` 就吃这个类型。
fn attrs(pairs: &[(i32, &str)]) -> XmlRoAttrList {
    let mut list = XmlRoAttrList::new();
    for (name, value) in pairs {
        list.add_attr(*name, value);
    }
    list
}

/// 宿主 `DgStore` 里是否登记了 spid（产品侧 `assert_store_holds` 的同一判据）。
fn store_holds(host: &'static ImportHost, spid: i32) -> bool {
    let bin = unsafe { (*host.store).query(spid) };
    !Null::equalsEmpty(Some(&*bin))
}

// Java: case 3211600 — p:sld @showMasterSp
const SLD_SHOW_MASTER_SP: i32 = 3211600;
// Java: case 3211608 — p:cSld
const SLD_CSLD: i32 = 3211608;
// Java: case 3211606 — p:spTree
const CSLD_SP_TREE: i32 = 3211606;
// Java: case 3211593 — p:sp
const GRP_SP: i32 = 3211593;
// Java: case 3211577 — p:nvSpPr
const GRP_NV_SP: i32 = 3211577;
// Java: case 3211574 — p:cNvPr
const NV_CNV_PR: i32 = 3211574;
// Java: case 3211342 — p:cNvPr @id
const CNV_PR_ID: i32 = 3211342;

#[test]
fn sh1_ctor_builds_empty_type0_root_registered_in_host_store() {
    let host = ImportHost::leak();
    let h = SlideHandler::SlideHandler(host);
    let root = h.take_root();
    assert_eq!(root.r#type(), 0, "spTree 根是 type 0（组）");
    assert_eq!(root.childCount(), 0);
    assert_eq!(h.cnvpr_count(), 0);
    assert_eq!(h.show_master_sp(), None, "构造时不带 p:sld 属性");
    assert!(
        store_holds(host, root.id()),
        "根 spid {} 必须已登记进宿主 DgStore",
        root.id()
    );
}

#[test]
fn sh2_start_element_records_show_master_sp_only_when_present() {
    let host = ImportHost::leak();
    let mut h = SlideHandler::SlideHandler(host);
    // 空属性表：缺省保持 None
    h.startElement(3211598, &XmlRoAttrList::new());
    assert_eq!(h.show_master_sp(), None);
    // p:sld @showMasterSp
    h.startElement(3211598, &attrs(&[(SLD_SHOW_MASTER_SP, "1")]));
    assert_eq!(h.show_master_sp(), Some(true));
    h.startElement(3211598, &attrs(&[(SLD_SHOW_MASTER_SP, "0")]));
    assert_eq!(h.show_master_sp(), Some(false));
    // 只有 3211601 / 3211610 时不动 showMasterSp
    h.startElement(3211598, &attrs(&[(3211601, "0"), (3211610, "0")]));
    assert_eq!(h.show_master_sp(), Some(false));
}

#[test]
fn sh3_enter_sub_element_dispatch_table() {
    let host = ImportHost::leak();
    let mut h = SlideHandler::SlideHandler(host);
    // 本里程碑不展开的 p:sld 子树，以及未知元素
    // （★W6c-3：3211599 `p:clrMapOvr` 已接真身 `ColorMapOverrideHandler`，移出本表）
    for id in [2162692, 3211295, 3211402, 3211609, 3211607, -1, 0] {
        assert!(
            h.enterSubElement(id).is_none(),
            "subElement {id} 不应派发 handler"
        );
    }
    // Java: case 3211599 → new ColorMapOverrideHandler(this.slideData.clrMap())
    let mut ovr = h
        .enterSubElement(3211599)
        .expect("3211599 → ColorMapOverrideHandler");
    assert_eq!(ovr.kind(), "ColorMapOverrideHandler");
    // Java: case 1114458 → new ColorMapHandler(this.colorMapping)
    assert_eq!(
        ovr.enterSubElement(1114458).expect("1114458 → ColorMapHandler").kind(),
        "ColorMapHandler"
    );
    // p:cSld
    let csld = h
        .enterSubElement(SLD_CSLD)
        .expect("3211608 → CommonSlideDataHandler");
    assert_eq!(csld.kind(), "CommonSlideDataHandler");
}

#[test]
fn sh4_csld_sp_tree_builds_child_shape_into_root() {
    let host = ImportHost::leak();
    let mut h = SlideHandler::SlideHandler(host);
    assert_eq!(h.take_root().childCount(), 0);

    let mut csld = h.enterSubElement(SLD_CSLD).expect("3211608 → csld");
    let mut group = csld.enterSubElement(CSLD_SP_TREE).expect("3211606 → 组装配");
    assert_eq!(group.kind(), "SwGroupShapeHandler");
    // p:sp：子节点在自己的 endElement 上挂到父组
    let mut leaf = group.enterSubElement(GRP_SP).expect("3211593 → 形状");
    leaf.endElement(GRP_SP);
    group.endElement(CSLD_SP_TREE);
    csld.endElement(SLD_CSLD);

    let root = h.take_root();
    assert_eq!(root.childCount(), 1, "p:sp 应成为 spTree 根的唯一子节点");
    assert_eq!(root.childAt(0).r#type(), 1, "p:sp 的节点 type 是 1");
    let child_id = root.childAt(0).id();
    assert!(
        store_holds(host, child_id),
        "子形状 spid {child_id} 必须已登记进宿主 DgStore"
    );
}

#[test]
fn sh5_cnvpr_count_follows_c_nv_pr_events() {
    let host = ImportHost::leak();
    let mut h = SlideHandler::SlideHandler(host);
    assert_eq!(h.cnvpr_count(), 0);
    let mut csld = h.enterSubElement(SLD_CSLD).unwrap();
    let mut group = csld.enterSubElement(CSLD_SP_TREE).unwrap();
    // p:nvSpPr → p:cNvPr
    let mut nv = group.enterSubElement(GRP_NV_SP).expect("3211577 → nv props");
    let mut cnv = nv.enterSubElement(NV_CNV_PR).expect("3211574 → cNvPr");
    // p:cNvPr @id 出现才计数
    cnv.startElement(NV_CNV_PR, &XmlRoAttrList::new());
    assert_eq!(h.cnvpr_count(), 0, "无 @id 不计数");
    cnv.startElement(NV_CNV_PR, &attrs(&[(CNV_PR_ID, "2")]));
    assert_eq!(h.cnvpr_count(), 1);
}

#[test]
fn sh6_end_element_is_a_noop_and_keeps_root() {
    let host = ImportHost::leak();
    let mut h = SlideHandler::SlideHandler(host);
    h.startElement(3211598, &attrs(&[(SLD_SHOW_MASTER_SP, "1")]));
    h.endElement(3211598);
    assert_eq!(
        h.show_master_sp(),
        Some(true),
        "shipped endElement 是空实现：不清状态也不改树"
    );
    assert_eq!(h.take_root().childCount(), 0);
}
