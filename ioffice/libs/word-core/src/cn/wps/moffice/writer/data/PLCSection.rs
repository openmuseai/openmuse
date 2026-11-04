#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{empty as PROPERTYSET_EMPTY, PropertySet};
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::{HeapNode, Node, PLC};
use crate::cn::wps::moffice::writer::data::internal_v2::ListIndexNode::ListIndexNode;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;

// java private static final int INVALID_CP = Integer.MIN_VALUE;
const INVALID_CP: i32 = i32::MIN;

/// Heap layout for Java `PLCSection.SectionNode` (extends PLC.Node).
#[repr(C)]
pub(crate) struct HeapSection {
    pub(crate) super_: HeapNode,
    pub(crate) parent: *mut PLCSection,
    pub(crate) sepx: PropertySet,
    pub(crate) evenHeader: *mut HeapNode,
    pub(crate) oddHeader: *mut HeapNode,
    pub(crate) evenFooter: *mut HeapNode,
    pub(crate) oddFooter: *mut HeapNode,
    pub(crate) firstHeader: *mut HeapNode,
    pub(crate) firstFooter: *mut HeapNode,
}

// java public class PLCSection extends PLC
pub struct PLCSection {
    // java PLC super_
    pub(crate) super_: PLC,
    // java private int mLastSecEndCP = Integer.MIN_VALUE;
    mLastSecEndCP: i32,
}

// java public static class SectionNode extends PLC.Node
#[derive(Clone, Copy, Default)]
pub struct SectionNode {
    pub(crate) ptr: *mut HeapSection,
}

impl PartialEq for SectionNode {
    fn eq(&self, other: &Self) -> bool {
        !self.ptr.is_null() && self.ptr == other.ptr
    }
}

fn alloc_section(parent: *mut PLCSection) -> *mut HeapSection {
    Box::into_raw(Box::new(HeapSection {
        super_: HeapNode {
            super_: ListIndexNode::new(-1, -1),
            document: None,
            numStart: 0,
            field_char: 0,
            field_data: 0,
            field_level: 0,
        },
        parent,
        sepx: (*PROPERTYSET_EMPTY).clone(),
        evenHeader: std::ptr::null_mut(),
        oddHeader: std::ptr::null_mut(),
        evenFooter: std::ptr::null_mut(),
        oddFooter: std::ptr::null_mut(),
        firstHeader: std::ptr::null_mut(),
        firstFooter: std::ptr::null_mut(),
    }))
}

pub(crate) fn as_section(n: Node) -> SectionNode {
    SectionNode { ptr: n.ptr as *mut HeapSection }
}

// java public class PLCSection extends PLC
impl PLCSection {
    // java public  PLCSection(Document document)
    pub fn PLCSection(document: Option<&dyn Document>) -> Self {
      Self { super_: PLC::new(document), mLastSecEndCP: INVALID_CP }
    }

    pub fn new(document: Option<&dyn Document>) -> Self {
        Self::PLCSection(document)
    }

    // java public SectionNode getFirstSection()
    pub fn getFirstSection(&self) -> Option<SectionNode> {
      if self.isEmpty() {
        None
      } else {
        Some(as_section(self.super_.begin()))
      }
    }

    // java public final boolean isFirstSection(SectionNode pSection)
    pub fn isFirstSection(&self, pSection: &SectionNode) -> bool {
      match self.getFirstSection() {
        Some(first) => first == *pSection,
        None => false,
      }
    }

    // java public SectionNode getLastSection()
    pub fn getLastSection(&self) -> Option<SectionNode> {
      if self.isEmpty() {
        None
      } else {
        Some(as_section(self.super_.last()))
      }
    }

    // java public void insert(int cp, PLC.Node pNew)
    pub fn insert(&mut self, cp: i32, pNew: Node) {
      if cp == 0 && !self.isEmpty() {
        self.super_.insertAtUpperBound(cp, pNew);
      } else {
        self.super_.insert__int_node(cp, pNew);
      }
    }

    // java public SectionNode addSection(int cp)
    pub fn addSection(&mut self, cp: i32) -> SectionNode {
      let heap = alloc_section(self as *mut PLCSection);
      let pNode = Node { ptr: heap as *mut HeapNode };
      self.insert(cp, pNode);
      SectionNode { ptr: heap }
    }

    // java public void setLastSectionEndCP(int cp)
    pub fn setLastSectionEndCP(&mut self, cp: i32) {
      self.mLastSecEndCP = cp;
    }

    // java public void onEndLoad()
    pub fn onEndLoad(&mut self) {
      self.mLastSecEndCP = i32::MIN;
    }

    // java private int getLastSectionEndCP()
    pub fn getLastSectionEndCP(&self) -> i32 {
      if i32::MIN == self.mLastSecEndCP {
        self.super_.getDocument().map(|d| d.getLength()).unwrap_or(0)
      } else {
        self.mLastSecEndCP
      }
    }

    // java public SectionNode getSection(int cp)
    pub fn getSection(&self, cp: i32) -> Option<SectionNode> {
      let mut secBreakNode = self.head.getNext();
      let mut iter = self.super_.createIterator__node(secBreakNode);
      while iter.hasNext() {
        iter.next();
        if iter.getStart() >= cp {
          break;
        }
        secBreakNode = iter.getNode();
      }
      if self.super_.isHead(secBreakNode) {
        None
      } else {
        Some(as_section(secBreakNode))
      }
    }

    // java public long getSectionRange(SectionNode pSection)
    pub fn getSectionRange(&self, pSection: &SectionNode) -> i64 {
      let start = self.getSectionStart(pSection);
      let end = self.getSectionEnd(pSection);
      RANGEFactory::getRANGE(start, end)
    }

    // java public int getSectionEnd(SectionNode pSection)
    pub fn getSectionEnd(&self, pSection: &SectionNode) -> i32 {
      match self.getLastSection() {
        Some(last) if last == *pSection => self.getLastSectionEndCP(),
        _ => pSection.getNext().position() + 1,
      }
    }

    // java public int getSectionStart(SectionNode pSection)
    pub fn getSectionStart(&self, pSection: &SectionNode) -> i32 {
      let mut start = pSection.position();
      if !self.isFirstSection(pSection) {
        start += 1;
      }
      start
    }

    // java  void doOnInsertText(int start, int end)
    pub fn doOnInsertText(&mut self, start: i32, end: i32) {
      let len = end - start;
      let mut upper = self.super_.lowerBound__int(start);
      if start == 0 && !self.super_.isEnd(upper) {
        upper = upper.getNext();
      }
      self.super_.insertSpaceBefore(upper, len);
    }

    // java  void doOnRemoveText(int start, int end)
    pub fn doOnRemoveText(&mut self, start: i32, end: i32) {
      let mut s = self.super_.lowerBound__int(start);
      let mut e = self.super_.lowerBound__int(end);
      while s != e {
        let is_first = self.getFirstSection().map(|f| f.as_node() == s).unwrap_or(false);
        if !is_first {
          s = s.getPrev();
          e = e.getPrev();
          break;
        }
        s = s.getNext();
      }
      let newCp = s.position();
      let fDelete = s != e;
      while s != e {
        let next = s.getNext();
        self.super_.remove(s);
        s = next;
      }
      self.super_.eraseSpaceBefore(e, end - start);
      if fDelete && newCp != e.position() {
        if e.position() > newCp {
          self.super_.moveLeft(e, e.position() - newCp);
        } else {
          self.super_.moveRight(e, newCp - e.position());
        }
      }
    }

    // java public void remove(PLC.Node node)
    pub fn remove(&mut self, node: Node) {
      // DEFERRED_NON_FUNCTION: header/footer cascade (removeSectionNode). 09 only needs PLCSection::new.
      self.removeSectionNode(as_section(node));
    }

    // java protected void notifyOnAdd(PLC.Node node)
    pub fn notifyOnAdd(&mut self, _node: Node) {
      // DEFERRED_NON_FUNCTION: IO-state section-changed event
    }

    // java protected void notifyOnRemoved(PLC.Node node)
    pub fn notifyOnRemoved(&mut self, _node: Node) {
      // DEFERRED_NON_FUNCTION: IO-state section-changed event
    }

    // java private void removeSectionNode(SectionNode node)
    pub fn removeSectionNode(&mut self, node: SectionNode) {
      // DEFERRED_NON_FUNCTION: PLCHeader / HelperHeaderFooter cascade; keep node drop on the PLC tree.
      self.super_.remove(node.as_node());
    }

    // java private void removeHeaderFooterNode(Document headerDocument, PLCHeader plc, PLC.Node headerNode)
    pub fn removeHeaderFooterNode(
      &mut self,
      _headerDocument: Option<&dyn Document>,
      _plc: *mut (),
      _headerNode: Node,
    ) {
      // DEFERRED_NON_FUNCTION: not 09 / not wc-02 parse-critical (needs KRange.removeText)
    }
}

impl std::ops::Deref for PLCSection {
    type Target = PLC;
    fn deref(&self) -> &Self::Target { &self.super_ }
}

impl std::ops::DerefMut for PLCSection {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl PLCSection {
    pub fn getIndex(&self, sectionNode: &SectionNode) -> i32 {
        self.super_.index(sectionNode.as_node())
    }
}

// java public static class SectionNode extends PLC.Node
impl SectionNode {
    pub fn as_node(&self) -> Node {
      Node { ptr: self.ptr as *mut HeapNode }
    }

    pub fn getNext(&self) -> Node {
      self.as_node().getNext()
    }

    pub fn getPrev(&self) -> Node {
      self.as_node().getPrev()
    }

    pub fn position(&self) -> i32 {
      self.as_node().position()
    }

    // java private SectionNode(PLCSection parent)
    pub fn SectionNode(parent: *mut PLCSection) -> Self {
      SectionNode { ptr: alloc_section(parent) }
    }

    // java public PropertySet getPropertySet()
    pub fn getPropertySet(&self) -> PropertySet {
      if self.ptr.is_null() {
        return (*PROPERTYSET_EMPTY).clone();
      }
      unsafe { (*self.ptr).sepx.clone() }
    }

    // java public void setPropertySet(PropertySet propertySet)
    pub fn setPropertySet(&mut self, propertySet: &PropertySet) {
      if !self.ptr.is_null() {
        unsafe { (*self.ptr).sepx = propertySet.clone(); }
      }
    }

    // java public PLC.Node getEvenHeader()
    pub fn getEvenHeader(&self) -> Node {
      if self.ptr.is_null() {
        return Node::Node();
      }
      Node { ptr: unsafe { (*self.ptr).evenHeader } }
    }
    // java public void setEvenHeader(PLC.Node evenHeader)
    pub fn setEvenHeader(&mut self, evenHeader: Node) {
      if !self.ptr.is_null() {
        unsafe { (*self.ptr).evenHeader = evenHeader.ptr; }
      }
    }
    // java public PLC.Node getOddHeader()
    pub fn getOddHeader(&self) -> Node {
      if self.ptr.is_null() {
        return Node::Node();
      }
      Node { ptr: unsafe { (*self.ptr).oddHeader } }
    }
    // java public void setOddHeader(PLC.Node oddHeader)
    pub fn setOddHeader(&mut self, oddHeader: Node) {
      if !self.ptr.is_null() {
        unsafe { (*self.ptr).oddHeader = oddHeader.ptr; }
      }
    }
    // java public PLC.Node getEvenFooter()
    pub fn getEvenFooter(&self) -> Node {
      if self.ptr.is_null() {
        return Node::Node();
      }
      Node { ptr: unsafe { (*self.ptr).evenFooter } }
    }
    // java public void setEvenFooter(PLC.Node evenFooter)
    pub fn setEvenFooter(&mut self, evenFooter: Node) {
      if !self.ptr.is_null() {
        unsafe { (*self.ptr).evenFooter = evenFooter.ptr; }
      }
    }
    // java public PLC.Node getOddFooter()
    pub fn getOddFooter(&self) -> Node {
      if self.ptr.is_null() {
        return Node::Node();
      }
      Node { ptr: unsafe { (*self.ptr).oddFooter } }
    }
    // java public void setOddFooter(PLC.Node oddFooter)
    pub fn setOddFooter(&mut self, oddFooter: Node) {
      if !self.ptr.is_null() {
        unsafe { (*self.ptr).oddFooter = oddFooter.ptr; }
      }
    }
    // java public PLC.Node getFirstHeader()
    pub fn getFirstHeader(&self) -> Node {
      if self.ptr.is_null() {
        return Node::Node();
      }
      Node { ptr: unsafe { (*self.ptr).firstHeader } }
    }
    // java public void setFirstHeader(PLC.Node firstHeader)
    pub fn setFirstHeader(&mut self, firstHeader: Node) {
      if !self.ptr.is_null() {
        unsafe { (*self.ptr).firstHeader = firstHeader.ptr; }
      }
    }
    // java public PLC.Node getFirstFooter()
    pub fn getFirstFooter(&self) -> Node {
      if self.ptr.is_null() {
        return Node::Node();
      }
      Node { ptr: unsafe { (*self.ptr).firstFooter } }
    }
    // java public void setFirstFooter(PLC.Node firstFooter)
    pub fn setFirstFooter(&mut self, firstFooter: Node) {
      if !self.ptr.is_null() {
        unsafe { (*self.ptr).firstFooter = firstFooter.ptr; }
      }
    }

    // java private void setHeaderFooter(NodeType type, PLC.Node headerFooter)
    pub fn setHeaderFooter(&mut self, r#type: NodeType, headerFooter: Node) {
      // DEFERRED_NON_FUNCTION: undo CmdModify; 09 does not write header nodes
      match r#type {
        NodeType::evenHeader => self.setEvenHeader(headerFooter),
        NodeType::oddHeader => self.setOddHeader(headerFooter),
        NodeType::evenFooter => self.setEvenFooter(headerFooter),
        NodeType::oddFooter => self.setOddFooter(headerFooter),
        NodeType::firstHeader => self.setFirstHeader(headerFooter),
        NodeType::firstFooter => self.setFirstFooter(headerFooter),
      }
    }

    // java private void recordHeaderChange(PLC.Node oldHeaderFooter, NodeType type)
    pub fn recordHeaderChange(&mut self, _oldHeaderFooter: Node, _type: NodeType) {
      // DEFERRED_NON_FUNCTION: transaction CmdModify
    }

    // java private PLC.Node get(NodeType type)
    pub fn get__nodetype(&self, r#type: NodeType) -> Node {
      match r#type {
        NodeType::firstFooter => self.getFirstFooter(),
        NodeType::firstHeader => self.getFirstHeader(),
        NodeType::evenFooter => self.getEvenFooter(),
        NodeType::evenHeader => self.getEvenHeader(),
        NodeType::oddFooter => self.getOddFooter(),
        NodeType::oddHeader => self.getOddHeader(),
      }
    }

    // java public void notifySectionChanged()
    pub fn notifySectionChanged(&self) {
      // DEFERRED_NON_FUNCTION: DocumentEventDispatcher.update — not 09 I/O
    }
}

// java static enum NodeType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeType {
    // java firstFooter
    firstFooter,
    // java firstHeader
    firstHeader,
    // java evenFooter
    evenFooter,
    // java evenHeader
    evenHeader,
    // java oddFooter
    oddFooter,
    // java oddHeader
    oddHeader,
}
