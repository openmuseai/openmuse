// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
use crate::cn::wps::moffice::writer::data::internal_v2::ListIndexNode::ListIndexNode;
// Java: import cn.wps.io.dom.Node
// (generator mis-import; PLC::Node)
// Java: import cn.wps.moffice.writer.data.PLCMathEnd
use crate::cn::wps::moffice::writer::data::PLCMathEnd::*;
// Java: import cn.wps.moffice.writer.data.RANGE
use crate::cn::wps::moffice::writer::data::RANGE::*;

// java public class PLCMathStart extends PLC
pub struct PLCMathStart {
    // java PLC super_
    pub(crate) super_: PLC,
}

impl Default for PLCMathStart {
    fn default() -> Self {
        Self { super_: PLC::new(None) }
    }
}

// java public static class OMathStartNode extends PLC.Node
#[derive(Default, Clone)]
pub struct OMathStartNode {
    // java PLC.Node super_
    pub(crate) super_: Node,
    // java PLCMathEnd.OMathEndNode end;
    pub(crate) end: Option<OMathEndNode>,
    // java OMathPara mOMathPara = null;
    pub(crate) mOMathPara: Option<OMathPara>,
}

// java static class OMathPara
#[derive(Default, Clone)]
pub struct OMathPara {
    // java private boolean mHasOMathParaPr;
    pub(crate) mHasOMathParaPr: bool,
    // java Integer mJc;
    pub(crate) mJc: Option<i32>,
}

// Java OMathStartNode / OMathEndNode fields live on the PLC node, after the shared header.
#[repr(C)]
pub(crate) struct HeapOMath {
    pub(crate) super_: HeapNode,
    pub(crate) kind: u8,
    pub(crate) other: Node,
    pub(crate) in_para: bool,
    pub(crate) jc: Option<i32>,
}

pub(crate) fn alloc_omath(kind: u8) -> Node {
    let heap = Box::into_raw(Box::new(HeapOMath {
        super_: HeapNode { super_: ListIndexNode::new(-1, -1), document: None, numStart: 0, field_char: 0, field_data: 0, field_level: 0 },
        kind,
        other: Node::default(),
        in_para: false,
        jc: None,
    }));
    Node { ptr: heap as *mut HeapNode }
}

pub(crate) fn heap_omath(node: Node) -> Option<*mut HeapOMath> {
    if node.ptr.is_null() || node.isHead() { None } else { Some(node.ptr as *mut HeapOMath) }
}

pub(crate) fn load_start(node: Node, link_end: bool) -> OMathStartNode {
    let (in_para, jc, other) = heap_omath(node)
        .map(|h| unsafe { ((*h).in_para, (*h).jc, (*h).other) })
        .unwrap_or((false, None, Node::default()));
    let end = if link_end && !other.ptr.is_null() && !other.isHead() {
        Some(OMathEndNode { super_: other, start: None })
    } else {
        None
    };
    OMathStartNode {
        super_: node,
        end,
        mOMathPara: if in_para { Some(OMathPara { mHasOMathParaPr: jc.is_some(), mJc: jc }) } else { None },
    }
}

pub(crate) fn load_end(node: Node) -> OMathEndNode {
    let other = heap_omath(node).map(|h| unsafe { (*h).other }).unwrap_or(Node::default());
    let start = if !other.ptr.is_null() && !other.isHead() {
        Some(Box::new(load_start(other, false)))
    } else {
        None
    };
    OMathEndNode { super_: node, start }
}

// java public class PLCMathStart extends PLC
impl PLCMathStart {
    // java public  PLCMathStart(Document document)
    pub fn PLCMathStart(document: &dyn Document) -> Self {
      // java super(document);
      Self { super_: PLC::new(Some(document)) }
    }

    // java public OMathStartNode findStartByCP(int cp)
    pub fn findStartByCP(&mut self, cp: i32) -> Option<OMathStartNode> {
      // java PLC.Node node = this.find(cp);
      let node: Node = self.super_.find(cp);
      // java return node == this.head ? null : (OMathStartNode)node;
      if self.super_.isHead(node) {
        return None;
      }
      return Some(load_start(node, true));
    }

    // java public OMathStartNode addOMathStart(int cp)
    pub fn addOMathStart(&mut self, cp: i32) -> OMathStartNode {
      // java OMathStartNode node = new OMathStartNode(); this.insert(cp, node);
      let node = alloc_omath(1);
      self.super_.insert__int_node(cp, node);
      load_start(node, true)
    }

    // java public void doOnInsertText(int start, int end)
    pub fn doOnInsertText(&mut self, start: i32, end: i32) {
      // java PLC.Node node = this.lowerBound(start);
      let node: Node = self.super_.lowerBound__int(start);
      // java this.insertSpaceBefore(node, end - start);
      self.super_.insertSpaceBefore(node, end - start);
    }

    // java public void doOnRemoveText(int start, int end)
    pub fn doOnRemoveText(&mut self, start: i32, end: i32) {
      // java PLC.Node s = this.lowerBound(start);
      let mut s: Node = self.super_.lowerBound__int(start);
      // java PLC.Node s_next;
      let mut s_next: Node;
      let mut e: Node = self.super_.lowerBound__int(end);
      // java for(PLC.Node e = this.lowerBound(end); s != e; s = s_next)
      while !nodes_same(s, e) {
        // java s_next = s.getNext();
        s_next = s.getNext();
        // java if (!this.isHead(s))
        if !self.super_.isHead(s) {
          // java OMathStartNode startNode = (OMathStartNode)s;
          let mut startNode: OMathStartNode = load_start(s.clone(), true);
          // java int startCp = startNode.getCP();
          let mut startCp: i32 = startNode.getCP();
          // java int endCp = startNode.getEnd().getCP();
          let mut endCp: i32 = startNode.getEnd().map(|n| n.getCP()).unwrap_or(startCp);
          // java if (endCp <= end && (startCp != endCp || startCp != start))
          if endCp <= end && (startCp != endCp || startCp != start) {
            // java this.removeOMathNode(startNode, startNode.getEnd());
            if let Some(mut end_node) = startNode.end.clone() {
              self.removeOMathNode(&mut startNode, &mut end_node);
            }
          }
        }
        s = s_next;
      }
      // java this._doOnRemoveText(start, end);
      self._doOnRemoveText(start, end);
    }

    // java private void _doOnRemoveText(int start, int end)
    pub fn _doOnRemoveText(&mut self, start: i32, end: i32) {
      // java PLC.Node s = this.lowerBound(start);
      let mut s: Node = self.super_.lowerBound__int(start);
      // java PLC.Node e;
      let mut e: Node;
      // java PLC.Node s_next;
      let mut s_next: Node;
      e = self.super_.lowerBound__int(end);
      // java for(e = this.lowerBound(end); s != e; s = s_next)
      while !nodes_same(s, e) {
        // java s_next = s.getNext();
        s_next = s.getNext();
        // java this.moveLeft(s, s.getCP() - start);
        self.super_.moveLeft(s, s.getCP() - start);
        s = s_next;
      }
      // java this.eraseSpaceBefore(e, end - start);
      self.super_.eraseSpaceBefore(e, end - start);
    }

    // java private void removeOMathNode(OMathStartNode startNode, PLCMathEnd.OMathEndNode endNode)
    pub fn removeOMathNode(&mut self, startNode: &mut OMathStartNode, endNode: &mut OMathEndNode) {
      // java this.removeOMathStart(startNode);
      self.removeOMathStart(startNode);
      // java this.getDocument().getOMathEnds().removeOMathEnd(endNode);
      if let Some(doc) = self.super_.getDocument() {
        if let Some(ends) = doc.getOMathEnds() {
          unsafe { (*ends).removeOMathEnd(endNode); }
        }
      }
    }

    // java public void removeOMathStart(OMathStartNode node)
    pub fn removeOMathStart(&mut self, node: &mut OMathStartNode) {
      // java this.remove(node);
      self.super_.remove(node.super_);
    }

    // java public OMathStartNode findClose(int cp)
    pub fn findClose(&mut self, cp: i32) -> Option<OMathStartNode> {
      // java PLC.Node findEntry = this.head;
      let mut findEntry: Node = self.super_.getHead().as_node();
      // java PLC.Node entry;
      let mut entry: Node;
      let mut iter = self.super_.createIterator();
      // java for(PLC.Iterator iter = this.createIterator(); iter.hasNext(); findEntry = entry)
      while iter.hasNext() {
        // java entry = iter.next();
        entry = iter.next();
        // java if (iter.getStart() > cp)
        if iter.getStart() > cp {
          // java findEntry = entry.getPrev();
          findEntry = entry.getPrev();
          // java break;
          break;
        }
        // java if (iter.getStart() == cp)
        if iter.getStart() == cp {
          // java findEntry = entry;
          findEntry = entry;
          // java break;
          break;
        }
        findEntry = entry;
      }
      // java return findEntry == this.head ? null : (OMathStartNode)findEntry;
      if self.super_.isHead(findEntry) {
        return None;
      }
      return Some(load_start(findEntry, true));
    }

    // java public void expandRange(RANGE result)
    pub fn expandRange(&mut self, result: &mut RANGE) {
      // java this.expandAt(result, result.end);
      self.expandAt(result, result.end);
      // java this.expandAt(result, result.start);
      self.expandAt(result, result.start);
    }

    // java private void expandAt(RANGE result, int cp)
    pub fn expandAt(&mut self, result: &mut RANGE, cp: i32) {
      // java OMathStartNode startNode = this.findClose(cp);
      let startNode: Option<OMathStartNode> = self.findClose(cp);
      // java if (startNode != null && result.hasIntersection(startNode.getCP(), startNode.getEnd().getCP()))
      if let Some(startNode) = startNode {
        let end_cp = startNode.getEnd().map(|n| n.getCP()).unwrap_or(startNode.getCP());
        if result.hasIntersection(startNode.getCP(), end_cp) {
          // java result.start = Math.min(result.start, startNode.getCP());
          result.start = result.start.min(startNode.getCP());
          // java result.end = Math.max(result.end, startNode.getEnd().getCP());
          result.end = result.end.max(end_cp);
        }
      }
    }

}

impl std::ops::Deref for PLCMathStart {
    type Target = PLC;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLCMathStart {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class OMathStartNode extends PLC.Node
impl OMathStartNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getCP(&self) -> i32 {
        self.super_.getCP()
    }

    // java public PLCMathEnd.OMathEndNode getEnd()
    pub fn getEnd(&self) -> Option<&OMathEndNode> {
      // java return this.end;
      return self.end.as_ref();
    }

    // java public void setEnd(PLCMathEnd.OMathEndNode end)
    pub fn setEnd(&mut self, end: Option<OMathEndNode>) {
      // java this.end = end;
      if let Some(heap) = heap_omath(self.super_) {
        let other = end.as_ref().map(|n| n.super_).unwrap_or(Node::default());
        unsafe { (*heap).other = other; }
        if let Some(end_heap) = heap_omath(other) {
          unsafe { (*end_heap).other = self.super_; }
        }
      }
      self.end = end;
    }

    // java public boolean isInOMathPara()
    pub fn isInOMathPara(&self) -> bool {
      // java return null != this.mOMathPara;
      if self.mOMathPara.is_some() {
        return true;
      }
      heap_omath(self.super_).is_some_and(|h| unsafe { (*h).in_para })
    }

    // java public void createOMathPara()
    pub fn createOMathPara(&mut self) {
      // java this.mOMathPara = new OMathPara();
      self.mOMathPara = Some(OMathPara::new());
      if let Some(heap) = heap_omath(self.super_) {
        unsafe { (*heap).in_para = true; }
      }
    }

    // java public void setOMathParaPr(Integer jc)
    pub fn setOMathParaPr__integer(&mut self, jc: Option<i32>) {
      // java this.mOMathPara.setJc(jc);
      if let Some(p) = self.mOMathPara.as_mut() {
        p.setJc(jc);
      }
      if let Some(heap) = heap_omath(self.super_) {
        unsafe {
          (*heap).jc = jc;
          (*heap).in_para = true;
        }
      }
    }

    // java public boolean hasOMathParaPr()
    pub fn hasOMathParaPr(&self) -> bool {
      // java return this.mOMathPara.mHasOMathParaPr;
      return self.mOMathPara.as_ref().map(|p| p.mHasOMathParaPr).unwrap_or(false);
    }

    // java public void setOMathParaPr(boolean hasOMathParaPr)
    pub fn setOMathParaPr__boolean(&mut self, hasOMathParaPr: bool) {
      // java this.mOMathPara.setOMathParaPr(hasOMathParaPr);
      if let Some(p) = self.mOMathPara.as_mut() {
        p.setOMathParaPr(hasOMathParaPr);
      }
    }

    // java public Integer getOMathParaPrJc()
    pub fn getOMathParaPrJc(&self) -> Option<i32> {
      // java return this.mOMathPara.mJc;
      return self.mOMathPara.as_ref().and_then(|p| p.mJc);
    }

}

impl std::ops::Deref for OMathStartNode {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for OMathStartNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java static class OMathPara
impl OMathPara {
    pub fn new() -> Self {
        Self::default()
    }

    // java  void setJc(Integer jc)
    pub fn setJc(&mut self, jc: Option<i32>) {
      // java this.mJc = jc;
      self.mJc = jc;
    }

    // java  void setOMathParaPr(boolean hasOMathParaPr)
    pub fn setOMathParaPr(&mut self, hasOMathParaPr: bool) {
      // java this.mHasOMathParaPr = hasOMathParaPr;
      self.mHasOMathParaPr = hasOMathParaPr;
    }

}
