// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.PLC_Char
use crate::cn::wps::moffice::writer::data::PLC_Char::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
use crate::cn::wps::moffice::writer::data::internal_v2::ListIndexNode::ListIndexNode;
// Java: import cn.wps.io.dom.Node
// (generator mis-import; PLC::Node)

// java public class PLCMathFunction extends PLC_Char
pub struct PLCMathFunction {
    // java PLC_Char super_
    pub(crate) super_: PLC_Char,
}

impl Default for PLCMathFunction {
    fn default() -> Self {
        Self {
            super_: PLC_Char {
                super_: PLC::new(None),
            },
        }
    }
}

// java public class MathNode extends PLC.Node
#[derive(Default, Clone)]
pub struct MathNode {
    // java PLC.Node super_
    pub(crate) super_: Node,
    // java @AtomMember private char mMathChar;
    pub(crate) mMathChar: char,
}

// java public class MathStartNode extends MathNode
#[derive(Default)]
pub struct MathStartNode {
    // java MathNode super_
    pub(crate) super_: MathNode,
    // java @AtomMember private PropertySet mPropSet;
    pub(crate) mPropSet: Option<PropertySet>,
    // java @AtomMember private int mType;
    pub(crate) mType: i32,
    // java private Shape mShape = null;
    pub(crate) mShape: Option<Shape>,
}

// Java MathNode / MathStartNode fields live on the PLC node. HeapNode is the shared header;
// extra fields sit after it, the same way section nodes extend the header.
#[repr(C)]
struct HeapMath {
    super_: HeapNode,
    math_char: char,
    prop: Option<PropertySet>,
    math_type: i32,
    shape: Option<Shape>,
}

fn alloc_math(ch: char, prop: Option<PropertySet>, math_type: i32) -> Node {
    let heap = Box::into_raw(Box::new(HeapMath {
        super_: HeapNode { super_: ListIndexNode::new(-1, -1), document: None, numStart: 0, field_char: 0, field_data: 0, field_level: 0 },
        math_char: ch,
        prop,
        math_type,
        shape: None,
    }));
    Node { ptr: heap as *mut HeapNode }
}

fn heap_math(node: Node) -> Option<*mut HeapMath> {
    if node.ptr.is_null() || node.isHead() { None } else { Some(node.ptr as *mut HeapMath) }
}

fn as_math(node: Node, ch: char) -> MathNode {
    let math_char = heap_math(node).map(|p| unsafe { (*p).math_char }).unwrap_or(ch);
    MathNode { super_: node, mMathChar: math_char }
}

pub(crate) fn is_math_start(node: Node) -> bool {
    heap_math(node).is_some_and(|p| unsafe { (*p).math_char == '\u{001c}' })
}

pub(crate) fn math_start_from(node: Node) -> Option<MathStartNode> {
    let heap = heap_math(node)?;
    unsafe {
        if (*heap).math_char != '\u{001c}' {
            return None;
        }
        Some(MathStartNode {
            super_: MathNode { super_: node, mMathChar: '\u{001c}' },
            mPropSet: (*heap).prop.clone(),
            mType: (*heap).math_type,
            mShape: None,
        })
    }
}

// java public class PLCMathFunction extends PLC_Char
impl PLCMathFunction {
    // java public  PLCMathFunction(Document document)
    pub fn PLCMathFunction(document: &dyn Document) -> Self {
      // java super(document);
      Self { super_: PLC_Char::PLC_Char(document) }
    }

    // java public MathNode addMathEndNode(int cp)
    pub fn addMathEndNode(&mut self, cp: i32) -> MathNode {
      // java MathNode mathNode = new MathNode(); mathNode.setMathChar('\u001b'); super.insert(cp, mathNode);
      let node = alloc_math('\u{001b}', None, 0);
      self.super_.insert__int_node(cp, node);
      MathNode { super_: node, mMathChar: '\u{001b}' }
    }

    // java public MathStartNode addMathStartNode(int cp)
    pub fn addMathStartNode(&mut self, cp: i32) -> MathStartNode {
      // java MathStartNode mathNode = new MathStartNode(); setPropertySet(empty); setMathChar('\u001c'); super.insert(cp, mathNode);
      let prop = Some(PropertySet::empty());
      let node = alloc_math('\u{001c}', prop.clone(), 0);
      self.super_.insert__int_node(cp, node);
      MathStartNode {
        super_: MathNode { super_: node, mMathChar: '\u{001c}' },
        mPropSet: prop,
        mType: 0,
        mShape: None,
      }
    }

    // java public MathStartNode findMathStartNodeByCP(int cp)
    pub fn findMathStartNodeByCP(&mut self, cp: i32) -> Option<MathStartNode> {
      // java PLC.Node node = this.find(cp);
      let node: Node = self.super_.find(cp);
      // java if (this.head == node)
      if self.super_.isHead(node) {
        // java return null;
        return None;
      }
      // java else
      else {
        // java MathNode mathNode = (MathNode)node;
        // java return mathNode.isBeginNode() ? (MathStartNode)mathNode : null;
        return math_start_from(node);
      }
    }

    // java public MathNode findMathEndNodeByCP(int cp)
    pub fn findMathEndNodeByCP(&mut self, cp: i32) -> Option<MathNode> {
      // java PLC.Node node = this.find(cp);
      let node: Node = self.super_.find(cp);
      // java if (this.head == node)
      if self.super_.isHead(node) {
        // java return null;
        return None;
      }
      // java else
      else {
        // java MathNode mathNode = (MathNode)node;
        let mut mathNode: MathNode = as_math(node, '\u{001b}');
        // java return mathNode.isBeginNode() ? null : mathNode;
        if mathNode.isBeginNode() {
          return None;
        }
        return Some(mathNode);
      }
    }

}

impl std::ops::Deref for PLCMathFunction {
    type Target = PLC_Char;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLCMathFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public class MathNode extends PLC.Node
impl MathNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn getCP(&self) -> i32 {
        self.super_.getCP()
    }

    // java public void setMathChar(char mathChar)
    pub fn setMathChar(&mut self, mathChar: char) {
      // java this.mMathChar = mathChar;
      self.mMathChar = mathChar;
      if let Some(heap) = heap_math(self.super_) {
        unsafe { (*heap).math_char = mathChar; }
      }
    }

    // java public char getMathChar()
    pub fn getMathChar(&self) -> char {
      // java return this.mMathChar;
      return self.mMathChar;
    }

    // java public MathStartNode getStartNode()
    pub fn getStartNode(&self) -> Option<MathStartNode> {
      // java PLC.Node node = this;
      let mut node: Node = self.super_.clone();
      // java int startCount = 0;
      let mut startCount: i32 = 0;
      // java int endCount = 0;
      let mut endCount: i32 = 0;
      // java if (this.mMathChar != 27)
      if self.mMathChar != '\u{001b}' {
        // java ++endCount;
        endCount += 1;
      }
      // java for(; node instanceof MathNode; node = node.getPrev())
      loop {
        // Stop at head (not a MathNode)
        // java MathNode tmp = (MathNode)node;
        let tmp_char = if nodes_same(node, self.super_) { self.mMathChar } else { '\0' };
        let mut tmp: MathNode = as_math(node.clone(), tmp_char);
        // java if (tmp.mMathChar == 28)
        if tmp.mMathChar == '\u{001c}' {
          // java ++startCount;
          startCount += 1;
          // java if (startCount == endCount)
          if startCount == endCount {
            // java return (MathStartNode)tmp;
            return math_start_from(tmp.super_);
          }
        }
        // java else if (tmp.mMathChar == 27)
        else if tmp.mMathChar == '\u{001b}' {
          // java ++endCount;
          endCount += 1;
        }
        let prev = node.getPrev();
        if nodes_same(prev, node) {
          break;
        }
        node = prev;
        // head has no MathNode type — break when CP would not progress / null-like
        if node.getCP() < 0 {
          break;
        }
      }
      // java return null;
      return None;
    }

    // java public MathNode getEndNode()
    pub fn getEndNode(&self) -> Option<MathNode> {
      // java PLC.Node node = this;
      let mut node: Node = self.super_.clone();
      // java int startCount = 0;
      let mut startCount: i32 = 0;
      // java int endCount = 0;
      let mut endCount: i32 = 0;
      // java if (this.mMathChar != 28)
      if self.mMathChar != '\u{001c}' {
        // java ++startCount;
        startCount += 1;
      }
      // java for(; node instanceof MathNode; node = node.getNext())
      loop {
        // java MathNode tmp = (MathNode)node;
        let tmp_char = if nodes_same(node, self.super_) { self.mMathChar } else { '\0' };
        let mut tmp: MathNode = as_math(node.clone(), tmp_char);
        // java if (tmp.mMathChar == 28)
        if tmp.mMathChar == '\u{001c}' {
          // java ++startCount;
          startCount += 1;
        }
        // java else if (tmp.mMathChar == 27)
        else if tmp.mMathChar == '\u{001b}' {
          // java ++endCount;
          endCount += 1;
          // java if (startCount == endCount)
          if startCount == endCount {
            // java return tmp;
            return Some(tmp);
          }
        }
        let next = node.getNext();
        if nodes_same(next, node) {
          break;
        }
        node = next;
      }
      // java return null;
      return None;
    }

    // java public boolean isBeginNode()
    pub fn isBeginNode(&self) -> bool {
      // java return this.mMathChar == 28;
      return self.mMathChar == '\u{001c}';
    }

    // java public boolean isEndNode()
    pub fn isEndNode(&self) -> bool {
      // java return this.mMathChar == 27;
      return self.mMathChar == '\u{001b}';
    }

}

impl std::ops::Deref for MathNode {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for MathNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public class MathStartNode extends MathNode
impl MathStartNode {
    pub fn new() -> Self {
        let mut n = Self::default();
        n.super_.mMathChar = '\u{001c}';
        n
    }

    pub fn setMathChar(&mut self, mathChar: char) {
        self.super_.setMathChar(mathChar);
    }

    pub fn getEndNode(&self) -> Option<MathNode> {
        self.super_.getEndNode()
    }

    pub fn getCP(&self) -> i32 {
        self.super_.getCP()
    }

    // java public void setPicture(Shape shape)
    pub fn setPicture(&mut self, shape: Option<Shape>) {
      // java this.mShape = shape;
      if let Some(heap) = heap_math(self.super_.super_) {
        unsafe { (*heap).shape = shape; }
        return;
      }
      self.mShape = shape;
    }

    // java public Shape getPicture()
    pub fn getPicture(&self) -> Option<&Shape> {
      // java return this.mShape;
      if let Some(heap) = heap_math(self.super_.super_) {
        let shape = unsafe { &(*heap).shape };
        if shape.is_some() {
          return shape.as_ref();
        }
      }
      self.mShape.as_ref()
    }

    // java public void setType(int type)
    pub fn setType(&mut self, r#type: i32) {
      // java this.mType = type;
      self.mType = r#type;
      if let Some(heap) = heap_math(self.super_.super_) {
        unsafe { (*heap).math_type = r#type; }
      }
    }

    // java public int getType()
    pub fn getType(&self) -> i32 {
      // java return this.mType;
      if let Some(heap) = heap_math(self.super_.super_) {
        return unsafe { (*heap).math_type };
      }
      self.mType
    }

    // java public void setPropertySet(PropertySet propSet)
    pub fn setPropertySet(&mut self, propSet: Option<PropertySet>) {
      // java this.mPropSet = propSet;
      if let Some(heap) = heap_math(self.super_.super_) {
        unsafe { (*heap).prop = propSet.clone(); }
      }
      self.mPropSet = propSet;
    }

    // java public PropertySet getPropertySet()
    pub fn getPropertySet(&self) -> Option<&PropertySet> {
      // java return this.mPropSet;
      if let Some(heap) = heap_math(self.super_.super_) {
        let prop = unsafe { &(*heap).prop };
        if prop.is_some() {
          return prop.as_ref();
        }
      }
      self.mPropSet.as_ref()
    }

}

impl std::ops::Deref for MathStartNode {
    type Target = MathNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for MathStartNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
