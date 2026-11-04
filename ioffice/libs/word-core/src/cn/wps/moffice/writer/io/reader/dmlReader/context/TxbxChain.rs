#![allow(non_snake_case, dead_code)]
use super::TxbxChainItem::{MyComparator, TxbxChainItem};

// Java: public class TxbxChain extends PriorityQueue<TxbxChainItem>
#[derive(Default)]
pub struct TxbxChain {
    // Java PriorityQueue stores items by mSeq using TxbxChainItem.MyComparator.
    items: Vec<TxbxChainItem>,
}
impl TxbxChain {
    // Java: public TxbxChain() { super(5, new TxbxChainItem.MyComparator()); }
    pub fn new() -> Self { Self { items: Vec::with_capacity(5) } }
    // Java: inherited PriorityQueue.add(TxbxChainItem)
    pub fn add(&mut self, item: TxbxChainItem) {
        // Java PriorityQueue.siftUpUsingComparator: equal keys preserve insertion order in the heap.
        self.items.push(item);
        let mut k = self.items.len() - 1;
        while k > 0 {
            let parent = (k - 1) / 2;
            if MyComparator::compare(Some(&self.items[k]), Some(&self.items[parent])) >= 0 { break; }
            self.items.swap(k, parent);
            k = parent;
        }
    }
    // Java: inherited PriorityQueue.poll()
    pub fn poll(&mut self) -> Option<TxbxChainItem> {
        if self.items.is_empty() { return None; }
        let result = self.items.swap_remove(0);
        let size = self.items.len();
        if size > 0 {
            let mut k = 0;
            while k < size / 2 {
                let mut child = k * 2 + 1;
                if child + 1 < size && MyComparator::compare(Some(&self.items[child]), Some(&self.items[child + 1])) > 0 {
                    child += 1;
                }
                if MyComparator::compare(Some(&self.items[k]), Some(&self.items[child])) <= 0 { break; }
                self.items.swap(k, child);
                k = child;
            }
        }
        Some(result)
    }
    pub fn isEmpty(&self) -> bool { self.items.is_empty() }
    // Java: public void build()
    pub fn build(&mut self) {
        // Java: TxbxChainItem pre = (TxbxChainItem)this.poll();
        let mut pre = self.poll().expect("TxbxChain.build requires the first item");
        // Java: for(int linkedID = pre.mShapeId; !this.isEmpty(); pre = curr)
        let linkedID = pre.mShapeId;
        while !self.isEmpty() {
            // Java: curr = (TxbxChainItem)this.poll();
            let curr = self.poll().expect("nonempty chain");
            // Java: pre.mTx.setTextNext(curr.mShapeId);
            pre.mTx_mut().setTextNext(curr.mShapeId);
            pre.mTx_mut().setTextLinkedID(linkedID);
            pre = curr;
        }
    }
}
