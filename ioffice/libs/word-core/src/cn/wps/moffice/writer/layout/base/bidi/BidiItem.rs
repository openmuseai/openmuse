// Java: cn.wps.moffice.writer.layout.base.bidi.BidiItem
#![allow(dead_code, non_snake_case)]

use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;

// java public class BidiItem extends KList.RecycableNode
pub struct BidiItem {
    pub graphs: *mut Graphs,
    pub children: Vec<BidiItem>,
    pub isRTL: bool,
}

impl Default for BidiItem {
    fn default() -> Self { Self::new() }
}

impl BidiItem {
    pub fn new() -> Self {
        Self { graphs: std::ptr::null_mut(), children: Vec::new(), isRTL: false }
    }

    pub fn hasChildBidi(&self) -> bool {
        !self.children.is_empty()
    }

    // java list.addAll(children); children.addAll(list) — KList.addAll moves nodes
    pub fn addFront(&mut self, list: &mut Vec<BidiItem>) {
        let mut children = std::mem::take(&mut self.children);
        list.append(&mut children);
        self.children.append(list);
    }

    pub fn addBack(&mut self, item: BidiItem) {
        self.children.push(item);
    }

    pub fn addBackList(&mut self, list: &mut Vec<BidiItem>) {
        self.children.append(list);
    }

    pub fn addReversionList(&mut self, list: &mut Vec<BidiItem>) {
        while let Some(item) = list.pop() {
            self.children.push(item);
        }
    }

    pub fn reset(&mut self) {
        self.graphs = std::ptr::null_mut();
        self.isRTL = false;
        self.children.clear();
    }
}
