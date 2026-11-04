use std::{cell::RefCell, rc::Rc};
use drawing::cn::wps::moffice::drawing::textbox::Text::Text;
use word_core::cn::wps::moffice::writer::io::reader::dmlReader::context::{
    TxbxChain::TxbxChain, TxbxChainItem::{MyComparator, TxbxChainItem}, TxbxChains::TxbxChains,
};

fn text() -> Rc<RefCell<Text>> { Rc::new(RefCell::new(Text::new_with_shape(None))) }

#[test]
fn builds_each_java_chain_in_sequence_order() {
    let first = text();
    let second = text();
    let third = text();
    let independent = text();
    let mut chains = TxbxChains::new();
    chains.put(7, 2, 202, second.clone());
    chains.put(7, 1, 101, first.clone());
    chains.put(7, 3, 303, third.clone());
    chains.put(8, 1, 404, independent.clone());
    chains.build();
    assert_eq!((first.borrow().getTextNext(), first.borrow().getTextLinkedID()), (202, 101));
    assert_eq!((second.borrow().getTextNext(), second.borrow().getTextLinkedID()), (303, 101));
    assert_eq!((third.borrow().getTextNext(), third.borrow().getTextLinkedID()), (-1, -1));
    assert_eq!((independent.borrow().getTextNext(), independent.borrow().getTextLinkedID()), (-1, -1));
}

#[test]
fn comparator_matches_java_wrapping_subtraction_and_null() {
    let tx = text();
    let first = TxbxChainItem::new(i32::MAX, 1, tx.clone());
    let second = TxbxChainItem::new(-1, 2, tx);
    assert_eq!(MyComparator::compare(Some(&first), Some(&second)), i32::MIN);
    assert_eq!(MyComparator::compare(None, Some(&second)), 0);
    let mut chain = TxbxChain::new();
    chain.add(first);
    chain.add(second);
    // Java PriorityQueue inserts the second element using compare(second, first).
    // Signed overflow makes that comparison negative too, so it becomes the root.
    assert_eq!(chain.poll().unwrap().mShapeId, 2);
}
