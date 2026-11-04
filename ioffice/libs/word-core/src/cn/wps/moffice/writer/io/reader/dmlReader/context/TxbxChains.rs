#![allow(non_snake_case, dead_code)]
use std::collections::HashMap;
use std::sync::Arc;
use drawing::cn::wps::moffice::drawing::textbox::Text::Text;
use super::{TxbxChain::TxbxChain, TxbxChainItem::TxbxChainItem};

// Java: public class TxbxChains extends HashMap<Integer, TxbxChain>
#[derive(Default)]
pub struct TxbxChains {
    chains: HashMap<i32, TxbxChain>,
}
impl TxbxChains {
    // Java: public TxbxChains()
    pub fn new() -> Self { Self { chains: HashMap::new() } }
    // Java: public void put(int id, int seq, int shapeId, Text tx)
    pub fn put(&mut self, id: i32, seq: i32, shapeId: i32, tx: Arc<Text>) {
        // Java: TxbxChain chain = (TxbxChain)this.get(id);
        // Java: if (null == chain) { chain = new TxbxChain(); this.put(id, chain); }
        let chain = self.chains.entry(id).or_insert_with(TxbxChain::new);
        // Java: chain.add(new TxbxChainItem(seq, shapeId, tx));
        chain.add(TxbxChainItem::new(seq, shapeId, tx));
    }
    // Java: public void build()
    pub fn build(&mut self) {
        // Java: for(Integer k : this.keySet()) { TxbxChain chain = (TxbxChain)this.get(k); chain.build(); }
        for chain in self.chains.values_mut() { chain.build(); }
    }
}
