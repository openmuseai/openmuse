use word_core::cn::wps::moffice::writer::cache::TypoPara::TypoPara;
use word_core::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
use word_core::cn::wps::moffice::writer::layout::base::graph::GraphArrayDeque::GraphArrayDeque;
use word_core::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
use util::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;

#[test]
fn drawing_handles_preserve_java_embed_list_order() {
    let mut para = TypoPara::default();
    assert_eq!(para.getEmbedSize(), 0);
    assert_eq!(para.getEmbed(0), 0);

    para.addEmbed(41);
    para.addEmbed(73);
    assert_eq!(para.getEmbedSize(), 2);
    assert_eq!(para.getEmbed(0), 41);
    assert_eq!(para.getEmbed(1), 73);
}

#[test]
fn paragraph_clear_drops_old_drawing_handles() {
    let mut para = TypoPara::default();
    para.addEmbed(41);
    para.clear();
    assert_eq!(para.getEmbedSize(), 0);
    assert_eq!(para.getEmbed(0), 0);
}

#[test]
fn layout_state_resolves_the_original_graph_to_its_drawing_handle() {
    let mut state = LayoutState::default();
    let mut first = Graphs::default();
    let mut second = Graphs::default();
    let absent = Graphs::default();
    state.typoPara.addEmbed(41);
    state.typoPara.addEmbed(73);
    let mut graphs = GraphArrayDeque::new();
    graphs.add(&mut first as *mut Graphs);
    graphs.add(&mut second as *mut Graphs);
    let mut indices = WIntArrayList::new();
    indices.add(1);
    indices.add(0);
    state.mGraphsForEmbed = Some(graphs);
    state.mEmbedIndex = Some(indices);

    assert_eq!(state.getDrawing(&first), 73);
    assert_eq!(state.getDrawing(&second), 41);
    assert_eq!(state.getDrawing(&absent), 0);
}
