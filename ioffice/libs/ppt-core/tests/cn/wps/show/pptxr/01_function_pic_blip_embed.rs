#[path = "visual_zip.rs"]
mod visual_zip;

use ppt_core::cn::wps::show::pptxr::PptxReader::{open_pptx_document_bytes, pic_blip_embed};
use visual_zip::visual_pptx;

#[test]
fn t01_pic_embed_rid() {
    let pic = r#"<p:pic><p:blipFill><a:blip r:embed="rIdImg"/></p:blipFill><p:spPr><a:xfrm><a:off x="1" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:pic>"#;
    assert_eq!(pic_blip_embed(pic).as_deref(), Some("rIdImg"));
    let doc = open_pptx_document_bytes(&visual_pptx(pic, false, false, true)).unwrap();
    let pic = doc.slides[0]
        .sp_tree
        .children
        .iter()
        .find(|s| s.shape_type == 2)
        .unwrap();
    assert_eq!(pic.media_r_id.as_deref(), Some("rIdImg"));
}

#[test]
fn t02_pic_without_blip_has_no_embed() {
    let pic = r#"<p:pic><p:spPr><a:xfrm><a:off x="1" y="0"/><a:ext cx="10" cy="10"/></a:xfrm></p:spPr></p:pic>"#;
    assert!(pic_blip_embed(pic).is_none());
    let doc = open_pptx_document_bytes(&visual_pptx(pic, false, false, false)).unwrap();
    assert!(doc.slides[0].sp_tree.children[0].media_r_id.is_none());
}

#[test]
fn t03_sp_is_not_pic_embed() {
    let sp = r#"<p:sp><p:spPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="1" cy="1"/></a:xfrm></p:spPr></p:sp>"#;
    assert!(pic_blip_embed(pic_like_missing()).is_none() || pic_blip_embed(sp).is_none());
    assert!(pic_blip_embed(sp).is_none());
}

fn pic_like_missing() -> &'static str {
    "<p:pic></p:pic>"
}

// rust: ★W6c-9 —— **产品喂入端 `p:spPr` 派发 + 真身活句柄**的回归断言。
//   Java: `new ShapePropertiesHandler(this.dgPic.spPr(), ...)` —— `spPr()` 是 `dgPic`
//   **自己**的 `ShapePr` 引用，子处理器写入直接落在 `dgPic` 上，`endElement` 的
//   `dgStore.update(id, dgPic.save())` 随即持久化。
//   本 port 原先 `SwPictureHandler::ShapePr_handle` 返回 `convert_clone()` 的**游离副本**
//   ⇒ `p:spPr` 写入被静默丢弃（实测 `DgStore::spPrById` 为空、`a:xfrm` 全丢）。
//   W6c-9 改为真身活句柄（`ShapePr_handle_live`）并把喂入端补齐到 pic/cxnSp。
//   断言语义：**该形状的 `ShapePr` 活句柄必须已登记进 `DgStore`，且 `hasXfrm()` 为真**
//   （`a:xfrm` 只存在于 `p:spPr` 内 ⇒ 为真即证明派发与活句柄两条链路都通）。
//   owner: tests.show.pptxr.01_function_pic_blip_embed    wave: M-PPT-KMOSLIDE-W6c-9

fn assert_child_sppr_live(body: &str, shape_type: i32) {
    let doc = open_pptx_document_bytes(&visual_pptx(body, false, false, false)).unwrap();
    // rust: 经 `films[0]`（真身 handler 链产物）取子形状 id，而非影子 `sp_tree`
    //   （影子解析器自己会 `extract_tag("spPr")`，用它断言会恒真 ⇒ 假绿）。
    let film = doc.films[0].clone();
    let mut f = film.borrow_mut();
    let root = f.spTree().getRoot();
    assert!(root.childCount() > 0, "handler 链必须装出子形状");
    let child_id = root.childAt(0).id();
    let store = doc.store().expect("打开期真身 DgStore 必须存在");
    let pr = store.shapePr(child_id).unwrap_or_else(|| {
        panic!(
            "形状 id={} 的 ShapePr 活句柄必须已登记进 DgStore（W6c-9 活句柄修复）",
            child_id
        )
    });
    assert!(
        pr.borrow_mut().hasXfrm(),
        "shape_type={} 的 p:spPr/a:xfrm 必须写进活句柄（W6c-9 喂入端派发）",
        shape_type
    );
}

#[test]
fn t07_pic_sppr_writes_live_handle() {
    let pic = r#"<p:pic><p:blipFill><a:blip r:embed="rIdImg"/></p:blipFill><p:spPr><a:xfrm><a:off x="1270000" y="635000"/><a:ext cx="5080000" cy="2540000"/></a:xfrm></p:spPr></p:pic>"#;
    assert_child_sppr_live(pic, 2);
}

#[test]
fn t08_cxnsp_sppr_writes_live_handle() {
    let cxn = r#"<p:cxnSp><p:spPr><a:xfrm><a:off x="2540000" y="1270000"/><a:ext cx="1016000" cy="508000"/></a:xfrm></p:spPr></p:cxnSp>"#;
    assert_child_sppr_live(cxn, 3);
}
