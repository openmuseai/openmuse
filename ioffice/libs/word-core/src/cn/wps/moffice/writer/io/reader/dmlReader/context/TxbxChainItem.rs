#![allow(non_snake_case, dead_code)]
use std::sync::Arc;
use drawing::cn::wps::moffice::drawing::textbox::Text::Text;

// Java: public class TxbxChainItem
pub struct TxbxChainItem {
    // Java: public int mSeq;
    pub mSeq: i32,
    // Java: public int mShapeId;
    pub mShapeId: i32,
    // Java: public Text mTx;
    pub mTx: Arc<Text>,
}
impl TxbxChainItem {
    // Java: public TxbxChainItem(int seq, int shapeId, Text tx)
    pub fn new(seq: i32, shapeId: i32, tx: Arc<Text>) -> Self {
        Self { mSeq: seq, mShapeId: shapeId, mTx: tx }
    }
    // Java: public String toString()
    pub fn toString(&self) -> String {
        format!("TxbxChainItem[shapeId: {}, seq: {}]", self.mShapeId, self.mSeq)
    }
    // Java mutates the Text object shared with the shape property.
    pub fn mTx_mut(&self) -> &mut Text {
        unsafe { &mut *(Arc::as_ptr(&self.mTx) as *mut Text) }
    }
}

// Java: public static class MyComparator implements Comparator<TxbxChainItem>
pub struct MyComparator;
impl MyComparator {
    // Java: public int compare(TxbxChainItem arg0, TxbxChainItem arg1)
    pub fn compare(arg0: Option<&TxbxChainItem>, arg1: Option<&TxbxChainItem>) -> i32 {
        match (arg0, arg1) {
            // Java: return null != arg0 && null != arg1 ? arg0.mSeq - arg1.mSeq : 0;
            (Some(left), Some(right)) => left.mSeq.wrapping_sub(right.mSeq),
            _ => 0,
        }
    }
}
