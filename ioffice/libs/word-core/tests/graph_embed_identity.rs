use word_core::cn::wps::moffice::writer::layout::base::graph::GraphBuff::{EmbedArg, GraphBuff};
use word_core::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;

#[test]
fn embed_queue_keeps_the_original_graph_identity_and_order() {
    let mut queue = GraphBuff::default();
    let mut first = Graphs::default();
    let mut second = Graphs::default();
    first.graphIndex = 12;
    second.graphIndex = 12; // Equal values must remain distinct Java objects.

    queue.addEmbed(&mut first, 1039);
    queue.addEqation(&mut second, 20, 21, true, false);

    assert_eq!(queue.getEmbedSize(), 2);
    assert_eq!(queue.getGraphsForEmbed(0), Some(&mut first as *mut Graphs));
    assert_eq!(queue.getGraphsForEmbed(1), Some(&mut second as *mut Graphs));
    assert_ne!(queue.getGraphsForEmbed(0), queue.getGraphsForEmbed(1));
    assert_eq!(queue.getArgsForEmbed(0), Some(&EmbedArg::Shape(1039)));
    assert!(matches!(queue.getArgsForEmbed(1), Some(EmbedArg::Eq(_))));

    queue.removeEmbedArgsAt(0);
    assert_eq!(queue.getEmbedSize(), 1);
    assert_eq!(queue.getGraphsForEmbed(0), Some(&mut second as *mut Graphs));
}
