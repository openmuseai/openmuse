#![allow(non_snake_case, dead_code, unused_imports)]
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::data::PLC::{PLC, Node};
use crate::cn::wps::moffice::writer::data::PLCSection::{as_section, SectionNode};

// java public class PLCHeader extends PLC_Range
/// Soft shell for PLCSection wiring (full PLCHeader rewrite deferred).
pub struct PLCHeader {
    pub(crate) super_: PLC,
}

impl PLCHeader {
    pub fn new(document: Option<&dyn crate::cn::wps::moffice::writer::core::Document::Document>) -> Self {
        Self { super_: PLC::new(document) }
    }

    pub fn getRange(&self, node: &Node) -> i64 {
        self.super_.getRange(*node)
    }

    pub fn remove(&mut self, entry: &mut Node) {
        self.super_.remove(*entry);
    }

    pub fn addNodeEvenHeader(&mut self, cp: i32) -> Node {
        self.addNodeHeaderFooter(PLC::alloc_node(), cp)
    }
    pub fn addNodeOddHeader(&mut self, cp: i32) -> Node {
        self.addNodeHeaderFooter(PLC::alloc_node(), cp)
    }
    pub fn addNodeEvenFooter(&mut self, cp: i32) -> Node {
        self.addNodeHeaderFooter(PLC::alloc_node(), cp)
    }
    pub fn addNodeOddFooter(&mut self, cp: i32) -> Node {
        self.addNodeHeaderFooter(PLC::alloc_node(), cp)
    }
    pub fn addNodeFirstHeader(&mut self, cp: i32) -> Node {
        self.addNodeHeaderFooter(PLC::alloc_node(), cp)
    }
    pub fn addNodeFirstFooter(&mut self, cp: i32) -> Node {
        self.addNodeHeaderFooter(PLC::alloc_node(), cp)
    }
    fn addNodeHeaderFooter(&mut self, node: Node, cp: i32) -> Node {
        self.super_.push_back(cp, node);
        node
    }
    pub fn insertNodeEvenHeader(&mut self, cp: i32, index: i32) -> Node {
        self.insertNodeHeaderFooter(PLC::alloc_node(), cp, index)
    }
    pub fn insertNodeOddHeader(&mut self, cp: i32, index: i32) -> Node {
        self.insertNodeHeaderFooter(PLC::alloc_node(), cp, index)
    }
    fn insertNodeHeaderFooter(&mut self, node: Node, cp: i32, index: i32) -> Node {
        self.super_.insert__node_int_int(node, cp, index);
        node
    }

    // Java: public PLCSection.SectionNode getSection(int cp)
    pub fn getSection(&self, cp: i32) -> Option<SectionNode> {
        // java PLC.Node headerNode = this.findFloor(cp);
        let header_node = self.find_floor(cp);
        let Some(document) = self.super_.getDocument() else {
            return None;
        };
        let Some(main) = document.getMainDocument() else {
            return None;
        };
        let Some(sections) = main.getPLCSection() else {
            return None;
        };
        let sections = unsafe { &*sections };
        // java PLC.Iterator iter = this.mDocument.getMainDocument().getPLCSection().createIterator();
        let mut iter = sections.super_.createIterator();
        while iter.hasNext() {
            let section = as_section(iter.next());
            // java reference equality, including null == null
            if section.getFirstHeader().ptr == header_node.ptr
                || section.getEvenHeader().ptr == header_node.ptr
                || section.getOddHeader().ptr == header_node.ptr
                || section.getFirstFooter().ptr == header_node.ptr
                || section.getEvenFooter().ptr == header_node.ptr
                || section.getOddFooter().ptr == header_node.ptr
            {
                return Some(section);
            }
        }
        // java return this.mDocument.getMainDocument().getPLCSection().getFirstSection();
        sections.getFirstSection()
    }

    fn find_floor(&self, cp: i32) -> Node {
        let mut lower = self.super_.lowerBound__int(cp);
        if self.super_.isHead(lower) {
            if self.super_.size() > 0 {
                lower = lower.getPrev();
                if self.super_.isHead(lower) {
                    return Node::default();
                }
                return lower;
            }
            return Node::default();
        }
        if lower.position() != cp {
            let prev = lower.getPrev();
            if self.super_.isHead(prev) {
                return Node::default();
            }
            return prev;
        }
        lower
    }
}

impl std::ops::Deref for PLCHeader {
    type Target = PLC;
    fn deref(&self) -> &Self::Target { &self.super_ }
}

impl std::ops::DerefMut for PLCHeader {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl Default for PLCHeader {
    fn default() -> Self {
        Self::new(None)
    }
}
