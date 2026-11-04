#[test]
fn probe_e2e_sppr_reaches_store() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../samples/ppt-render/flutter/assets/Sample_ppt.pptx");
    let bytes = std::fs::read(path).unwrap();
    let doc = ppt_core::cn::wps::show::pptxr::PptxReader::open_pptx_document_bytes(&bytes).unwrap();
    for idx in [0usize, 2] {
        let sp_tree = &doc.slides[idx].sp_tree;
        println!("=== slide{} shapes={} ===", idx + 1, sp_tree.shapeCount());
        for i in 0..sp_tree.shapeCount() {
            let sp = sp_tree.shapeAt(i);
            println!("  i={} id={} type={} prst={:?} ft={:?} ow={:?}",
                i, sp.id(), sp.r#type(), sp.sp_pr.geom_obj.prst,
                sp.sp_pr.fill_obj.fill_type, sp.sp_pr.outline_obj.width);
        }
    }
}
