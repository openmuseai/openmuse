#![allow(non_snake_case, dead_code, unused_imports)]
// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.drawing.GroupShape
// Java: import cn.wps.moffice.drawing.Shape
// Java: import cn.wps.moffice.drawing.util.ShapeIDMap
use drawing::cn::wps::moffice::drawing::util::ShapeIDMap::{
    IHashMapExtension as ShapeMapExtension, IMapEntryNodeTrait, KHashMapListener, Listener,
    ShapeIDMap,
};
// Java: import cn.wps.moffice.transaction.Command
use util::moffice::transaction::Command::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
use util::moffice::transaction::Action::ActionCommand;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.shape.command.KDeleteCommand
// rust: command 未登记在 core/shape 内联块（§1.7.1）
// Java: import cn.wps.moffice.writer.data.PLC_Char
use crate::cn::wps::moffice::writer::data::PLC_Char::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::{nodes_same, Node};
use util::collections::PositionNode::PositionNode;
// Java: import cn.wps.io.dom.Node
// (PLC.Node — not io.dom.Node)
// Java: import cn.wps.moffice.writer.data.ITextStream
use crate::cn::wps::moffice::writer::data::ITextStream::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::*;
use crate::cn::wps::moffice::writer::data::DrawingContainer::SoftShape;
// Java: import cn.wps.moffice.writer.core.shape.ShapeTool
// rust: ShapeTool 未登记在 core/shape 内联块（§1.7.1）
use std::collections::HashMap;
use util::collections::ITreeExtension::ITreeExtension;

// java private static final int INVALIDEDID = -1;
const INVALIDEDID: i32 = -1;

#[derive(Copy, Clone)]
struct MapListenerBridge {
    map: *mut ShapeIDMap,
}

impl MapListenerBridge {
    fn before_key_change(&self, key: i32) {
        if !self.map.is_null() {
            unsafe {
                KHashMapListener::new(self.map).beforeKeyChange(key);
            }
        }
    }

    fn after_key_change(&self, key: i32, value: &dyn std::any::Any) {
        if !self.map.is_null() {
            unsafe {
                KHashMapListener::new(self.map).afterKeyChange(key, value);
            }
        }
    }
}

impl Listener for MapListenerBridge {
    fn beforeKeyChange(&mut self, key: i32) {
        self.before_key_change(key);
    }

    fn afterKeyChange(&mut self, key: i32, value: &dyn std::any::Any) {
        self.after_key_change(key, value);
    }
}

struct ShapeMapTreeExtension {
    map: *mut ShapeIDMap,
}

impl ITreeExtension for ShapeMapTreeExtension {
    fn onCreateHead(&mut self, tree: *mut u8, head: *mut u8) {
        if !self.map.is_null() {
            unsafe {
                ShapeMapExtension::onCreateHead(&mut *self.map, tree, head);
            }
        }
    }

    fn onBinaryTreeInsertedLeft(
        &mut self,
        tree: *mut u8,
        p_node: *mut u8,
        p_new: *mut u8,
        arg1: i32,
        arg2: Option<&dyn std::any::Any>,
    ) {
        if !self.map.is_null() {
            unsafe {
                ShapeMapExtension::onBinaryTreeInsertedLeft(
                    &mut *self.map,
                    tree,
                    p_node,
                    p_new,
                    arg1,
                    arg2,
                );
            }
        }
    }

    fn onBinaryTreeInsertedRight(
        &mut self,
        tree: *mut u8,
        p_node: *mut u8,
        p_new: *mut u8,
        arg1: i32,
        arg2: Option<&dyn std::any::Any>,
    ) {
        if !self.map.is_null() {
            unsafe {
                ShapeMapExtension::onBinaryTreeInsertedRight(
                    &mut *self.map,
                    tree,
                    p_node,
                    p_new,
                    arg1,
                    arg2,
                );
            }
        }
    }

    fn beforeSwap(&mut self, tree: *mut u8, z: *mut u8, y: *mut u8) {
        if !self.map.is_null() {
            unsafe {
                ShapeMapExtension::beforeSwap(&mut *self.map, tree, z, y);
            }
        }
    }

    fn beforeErase(&mut self, tree: *mut u8, z: *mut u8, swp: *mut u8) {
        if !self.map.is_null() {
            unsafe {
                ShapeMapExtension::beforeErase(&mut *self.map, tree, z, swp);
            }
        }
    }

    fn onBinaryTreeErased(
        &mut self,
        tree: *mut u8,
        z: *mut u8,
        x: *mut u8,
        x_parent: *mut u8,
    ) {
        if !self.map.is_null() {
            unsafe {
                ShapeMapExtension::onBinaryTreeErased(&mut *self.map, tree, z, x, x_parent);
            }
        }
    }

    fn beforeRotateLeft(&mut self, tree: *mut u8, x: *mut u8, y: *mut u8) {
        if !self.map.is_null() {
            unsafe {
                ShapeMapExtension::beforeRotateLeft(&mut *self.map, tree, x, y);
            }
        }
    }

    fn beforeRotateRight(&mut self, tree: *mut u8, y: *mut u8, x: *mut u8) {
        if !self.map.is_null() {
            unsafe {
                ShapeMapExtension::beforeRotateRight(&mut *self.map, tree, y, x);
            }
        }
    }

    fn getTreeCreator(&self) -> Option<Box<dyn std::any::Any>> {
        if self.map.is_null() {
            None
        } else {
            Some(Box::new(unsafe { (*self.map).getTreeCreator() }))
        }
    }
}

#[derive(Clone, Default)]
struct ShapeAnchorMeta {
    mShapeId: i32,
    mLastValidShapeId: i32,
}

fn anchor_node_ptr(n: &Node) -> *mut PositionNode {
    n.as_pos()
}

fn node_key(n: &Node) -> usize {
    anchor_node_ptr(n) as usize
}

// java public class PLCShapeAnchor extends PLC_Char
pub struct PLCShapeAnchor {
    // java PLC_Char super_
    pub(crate) super_: PLC_Char,
    // java private ShapeIDMap mMapExtension = new ShapeIDMap();
    mMapExtension: Box<ShapeIDMap>,
    anchor_meta: HashMap<usize, ShapeAnchorMeta>,
}

// java public static class ShapeAnchor extends PLC.Node implements ShapeIDMap.IMapEntryNode, Cloneable
#[derive(Clone)]
pub struct ShapeAnchor {
    // java PLC.Node super_
    pub(crate) super_: Node,
    // java private int mShapeId;
    mShapeId: i32,
    // java private IHashMapExtension.Listener mListener = null;
    mListener: MapListenerBridge,
    // java private PLCShapeAnchor mParent;
    mParent: *mut PLCShapeAnchor,
    // java int mLastValidShapeId = Integer.MIN_VALUE;
    pub(crate) mLastValidShapeId: i32,
}

// java  class CmdShapeIdChange extends Command
pub struct CmdShapeIdChange {
    // java Command super_
    pub(crate) super_: Command,
    // java ShapeAnchor node;
    pub(crate) node: *mut ShapeAnchor,
    // java int shapeId;
    pub(crate) shapeId: i32,
    outer_listener: MapListenerBridge,
}

// java public class PLCShapeAnchor extends PLC_Char
impl PLCShapeAnchor {
    fn map_ptr(&mut self) -> *mut ShapeIDMap {
        self.mMapExtension.as_mut() as *mut ShapeIDMap
    }

    fn map_listener(&mut self) -> MapListenerBridge {
        MapListenerBridge { map: self.map_ptr() }
    }

    fn register_tree_extension(&mut self) {
        let map_ptr = self.map_ptr();
        self.super_
            .super_
            .super_
            .super_
            .super_
            .base
            .addExtension(Box::new(ShapeMapTreeExtension { map: map_ptr }), 7);
    }

    fn meta_for_node(&self, node: &Node) -> ShapeAnchorMeta {
        self.anchor_meta
            .get(&node_key(node))
            .cloned()
            .unwrap_or_default()
    }

    fn register_anchor_node(&mut self, node: &Node, meta: ShapeAnchorMeta) {
        self.anchor_meta.insert(node_key(node), meta);
    }

    fn node_to_anchor(&self, node: Node, meta: ShapeAnchorMeta) -> ShapeAnchor {
        ShapeAnchor {
            super_: node,
            mShapeId: meta.mShapeId,
            mListener: MapListenerBridge {
                map: std::ptr::null_mut(),
            },
            mParent: self as *const _ as *mut _,
            mLastValidShapeId: meta.mLastValidShapeId,
        }
    }

    fn document(&self) -> Option<&dyn Document> {
        self.super_.super_.mDocument.map(|p| unsafe { &*p })
    }

    // java public  PLCShapeAnchor(Document document)
    pub fn PLCShapeAnchor(document: &dyn Document) -> Self {
      // java super(document);
      let mut this = Self {
        super_: PLC_Char::PLC_Char(document),
        // Java stores ShapeIDMap as an independently allocated object.
        // Keep its address stable when PLCShapeAnchor moves into SubDocument:
        // tree extensions and listeners retain a pointer to this exact map.
        mMapExtension: Box::new(ShapeIDMap::new()),
        anchor_meta: HashMap::new(),
      };
      // java ShapeIDMap var10000 = this.mMapExtension;
      // java ShapeIDMap var10003 = this.mMapExtension;
      // java Objects.requireNonNull(var10003);
      // java var10000.setListener(var10003.new KHashMapListener());
      let map_ptr = this.mMapExtension.as_mut() as *mut ShapeIDMap;
      unsafe {
        (*map_ptr).setListener(Box::new(MapListenerBridge { map: map_ptr }));
      }
      // java super.addExtension(this.mMapExtension, 7);
      this.register_tree_extension();
      this
    }

    pub fn new(document: &dyn Document) -> Self {
        Self::PLCShapeAnchor(document)
    }

    // java protected ShapeAnchor createNode(PLCShapeAnchor parent)
    pub fn createNode(parent: *mut PLCShapeAnchor) -> ShapeAnchor {
      // java return new ShapeAnchor(parent);
      ShapeAnchor::new(parent)
    }

    // java public ShapeAnchor findByCP(int cp)
    pub fn findByCP(&mut self, cp: i32) -> Option<ShapeAnchor> {
      // java PLC.Node p = this.lowerBound(cp);
      let p = self.super_.lowerBound__int(cp);
      // java return p != this.head && p.position() == cp ? (ShapeAnchor)p : null;
      if !self.super_.isHead(p) && p.position() == cp {
        let meta = self.meta_for_node(&p);
        Some(self.node_to_anchor(p, meta))
      } else {
        None
      }
    }

    // java public ShapeAnchor findById(int nShapeId)
    pub fn findById(&self, nShapeId: i32) -> Option<ShapeAnchor> {
      // java return null != this.mMapExtension ? (ShapeAnchor)this.mMapExtension.getValue(nShapeId) : null;
      if self.mMapExtension.getValue(nShapeId).is_some() {
        let mut iter = self.createIterator();
        while iter.hasNext() {
          let node = iter.next();
          let meta = self.meta_for_node(&node);
          if meta.mShapeId == nShapeId {
            return Some(self.node_to_anchor(node, meta));
          }
        }
      }
      None
    }

    // java public ShapeAnchor addShapeAnchor(int cp, int nShapeId)
    pub fn addShapeAnchor(&mut self, cp: i32, nShapeId: i32) -> ShapeAnchor {
      // java ShapeAnchor node = this.createNode(this);
      let parent_ptr = self as *mut PLCShapeAnchor;
      let listener = self.map_listener();
      let mut node = Self::createNode(parent_ptr);
      // java this.insert(cp, node);
      self.super_.insert__int_node(cp, node.super_);
      self.register_anchor_node(&node.super_, ShapeAnchorMeta::default());
      // java node.init(nShapeId, this.mMapExtension.getListener());
      node.init(nShapeId, listener);
      // java return node;
      node
    }

    // java public void remove(PLC.Node entry)
    pub fn remove(&mut self, entry: &mut Node) {
      // java if (entry instanceof ShapeAnchor)
      let meta = self.meta_for_node(entry);
      if meta.mShapeId != 0 || self.anchor_meta.contains_key(&node_key(entry)) {
        // java this.removeShapeAnchor((ShapeAnchor)entry);
        let mut anchor = self.node_to_anchor(entry.clone(), meta);
        self.removeShapeAnchor(&mut anchor);
      }
    }

    // java public void removeShapeAnchor(ShapeAnchor anchor)
    pub fn removeShapeAnchor(&mut self, anchor: &mut ShapeAnchor) {
      // java this.removeShapeInfo(anchor);
      self.removeShapeInfo(anchor);
      // java super.remove(anchor);
      self.super_.remove(anchor.super_);
      self.anchor_meta.remove(&node_key(&anchor.super_));
    }

    // java private void removeShapeInfo(ShapeAnchor anchor)
    pub fn removeShapeInfo(&mut self, anchor: &ShapeAnchor) {
      // java int shapeId = anchor.getShapeId();
      let shapeId = anchor.getShapeId();
      // java if (shapeId > 0)
      if shapeId > 0 {
        // java IDrawingContainer drawingContainer = this.mDocument.getDrawingContainer();
        // rust: no-stmt — Document::getDrawingContainer 仍为 `*mut ()`（§1.7.1）
        let _ = self.document();
        // java Shape shape = drawingContainer.findShapeByID(shapeId);
        // rust: no-stmt
        let _ = shapeId;
        // java Document textboxDocument = this.mDocument.getTextboxDocument();
        // rust: no-stmt
        // java ITextStream textStream = textboxDocument.getTextStream();
        // rust: no-stmt
        // java boolean bRevision = textStream.getRevisionFileter();
        // rust: no-stmt
        // java textStream.disableRevisionFileter();
        // rust: no-stmt
        // java ShapeTool.deleteText / drawingContainer.delShape / KDeleteCommand
        // rust: no-stmt — ShapeTool/KDeleteCommand 未登记（§1.7.1）
        // java textStream.enableRevisionFileter();
        // rust: no-stmt
      }
    }

    // java public Vector<ShapeAnchor> getShapeInlineAnchors(int start, int end)
    pub fn getShapeInlineAnchors(&mut self, start: i32, end: i32) -> Option<Vec<ShapeAnchor>> {
      // java if (start >= end)
      if start >= end {
        // java return null;
        return None;
      }
      // java else
      else {
        // java Vector<ShapeAnchor> anchors = new Vector();
        let mut anchors: Vec<ShapeAnchor> = Vec::new();
        // java PLC.Node head = this.end();
        let head = self.super_.head.as_node();
        // java PLC.Node startNode = this.lowerBound(start);
        let start_node = self.super_.lowerBound__int(start);
        // java PLC.Node endNode = this.upperBound(end - 1);
        let end_node = self.super_.upperBound__int(end - 1);
        // java if (startNode == endNode && startNode != head)
        if nodes_same(start_node, end_node) && !nodes_same(start_node, head) {
          // java anchors.add((ShapeAnchor)startNode);
          let meta = self.meta_for_node(&start_node);
          anchors.push(self.node_to_anchor(start_node.clone(), meta));
        }
        // java PLCField plcField = this.mDocument.getPLCField();
        let plc_field_ptr = self.document().and_then(|d| d.getPLCField());
        let mut entry = start_node;
        // java for(PLC.Node entry = startNode; entry != head && entry != endNode entry = entry.getNext())
        while !nodes_same(entry, head) && !nodes_same(entry, end_node) {
          // java ShapeAnchor anchor = (ShapeAnchor)entry;
          let meta = self.meta_for_node(&entry);
          let anchor = self.node_to_anchor(entry, meta);
          // java int cp = this.position(anchor);
          let cp = anchor.getCP();
          // java int cType = 1;
          let mut c_type: i32 = 1;
          // java if (plcField != null)
          if let Some(plc_field_ptr) = plc_field_ptr {
            if !plc_field_ptr.is_null() {
              // java PLCField.FieldTriple triple = plcField.getContainingField(cp);
              let triple = unsafe { (*plc_field_ptr).getContainingField(cp) };
              // java if (triple != null && cn.wps.moffice.writer.core.shape.ShapeTool.isShapeTriple(triple))
              if triple.is_some() {
                // rust: no-stmt — ShapeTool 未登记（§1.7.1）；triple 非空时仍按 Java 进入 isShapeTriple 分支前半
                // rust: ShapeTool::isShapeTriple 停车 ⇒ 不把 cType 改成 8
                let _ = &mut c_type;
              }
            }
          }
          // java if (this.mDocument.charAt(cp) == cType)
          if let Some(doc) = self.super_.getDocument_mut() {
            let ch = doc.charAt(cp);
            if ch as i32 == c_type {
              // java anchors.add(anchor);
              anchors.push(anchor);
            }
          }
          entry = entry.getNext();
        }
        // java return anchors;
        return Some(anchors);
      }
    }

}

impl std::ops::Deref for PLCShapeAnchor {
    type Target = PLC_Char;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for PLCShapeAnchor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class ShapeAnchor extends PLC.Node implements ShapeIDMap.IMapEntryNode, Cloneable
impl ShapeAnchor {
    // java public  ShapeAnchor(PLCShapeAnchor parent)
    pub fn ShapeAnchor(parent: *mut PLCShapeAnchor) -> Self {
      Self::new(parent)
    }

    pub fn new(parent: *mut PLCShapeAnchor) -> Self {
      // java this.mParent = parent;
      Self {
        super_: Node::new(),
        mShapeId: 0,
        mListener: MapListenerBridge {
          map: if parent.is_null() {
            std::ptr::null_mut()
          } else {
            unsafe { (*parent).mMapExtension.as_mut() as *mut ShapeIDMap }
          },
        },
        mParent: parent,
        mLastValidShapeId: i32::MIN,
      }
    }

    pub fn getCP(&self) -> i32 {
        self.super_.getCP()
    }

    // java public ShapeAnchor clone()
    pub fn clone(&self) -> ShapeAnchor {
      // java ShapeAnchor anchor = null;
      // java anchor = (ShapeAnchor)super.clone();
      self.clone()
    }

    // java public int getShapeId()
    pub fn getShapeId(&self) -> i32 {
      // java return this.mShapeId;
      return self.mShapeId;
    }

    // java public int getLastValidShapeId()
    pub fn getLastValidShapeId(&self) -> i32 {
      // java return this.mShapeId >= 0 ? this.mShapeId : this.mLastValidShapeId;
      return if self.mShapeId >= 0 {
        self.mShapeId
      } else {
        self.mLastValidShapeId
      };
    }

    // java public void setInvalidShapeId()
    pub fn setInvalidShapeId(&mut self) {
      // java this.setShapeId(-1);
      self.setShapeId(-1);
    }

    // java public void setShapeId(int shapeId)
    pub fn setShapeId(&mut self, shapeId: i32) {
      // java Assert.assertNotNull("mListener should not be null!", this.mListener);
      Assert::assertNotNull__string_object(
        "mListener should not be null!",
        if self.mListener.map.is_null() {
          None::<()>
        } else {
          Some(())
        },
      );
      // java this.mListener.beforeKeyChange(this.mShapeId);
      self.mListener.before_key_change(self.mShapeId);
      // java int pOldShapeId = this.mShapeId;
      let p_old_shape_id = self.mShapeId;
      // java if (-1 != shapeId)
      if -1 != shapeId {
        // java this.mLastValidShapeId = shapeId;
        self.mLastValidShapeId = shapeId;
      }
      // java this.mShapeId = shapeId;
      self.mShapeId = shapeId;
      if !self.mParent.is_null() {
        unsafe {
          let parent = &mut *self.mParent;
          parent
            .anchor_meta
            .insert(node_key(&self.super_), ShapeAnchorMeta {
              mShapeId: self.mShapeId,
              mLastValidShapeId: self.mLastValidShapeId,
            });
        }
      }
      // java this.mListener.afterKeyChange(this.mShapeId, this);
      self.mListener.after_key_change(self.mShapeId, self);
      // java Transaction t = this.mParent.getTransaction();
      let transaction = if self.mParent.is_null() {
        None
      } else {
        unsafe { (*self.mParent).getTransaction() }
      };
      // java if (t != null && t.getState() == 1)
      if let Some(t) = transaction {
        if t.getState() == 1 {
          // java t.submitCommand(new CmdShapeIdChange(this, pOldShapeId));
          let cmd = CmdShapeIdChange::new(self as *mut _, p_old_shape_id, self.mListener);
          let tx_ptr = if self.mParent.is_null() {
            std::ptr::null_mut()
          } else {
            unsafe { (*self.mParent).super_.super_.mTransaction }
          };
          if !tx_ptr.is_null() {
            unsafe {
              (*tx_ptr).submitCommand(Some(Box::new(cmd)));
            }
          }
        }
      }
    }

    // java public int getKey()
    pub fn getKey(&self) -> i32 {
      // java return this.mShapeId;
      return self.mShapeId;
    }

    // java public void init(int nShapeId, IHashMapExtension.Listener listener)
    pub fn init(&mut self, nShapeId: i32, listener: MapListenerBridge) {
      // java this.mShapeId = nShapeId;
      self.mShapeId = nShapeId;
      // java this.mLastValidShapeId = nShapeId;
      self.mLastValidShapeId = nShapeId;
      // java this.mListener = listener;
      self.mListener = listener;
      if !self.mParent.is_null() {
        unsafe {
          let parent = &mut *self.mParent;
          parent.register_anchor_node(
            &self.super_,
            ShapeAnchorMeta {
              mShapeId: self.mShapeId,
              mLastValidShapeId: self.mLastValidShapeId,
            },
          );
        }
      }
      // java this.mListener.afterKeyChange(nShapeId, this);
      self.mListener.after_key_change(nShapeId, self);
    }

}

impl IMapEntryNodeTrait for ShapeAnchor {
    fn getKey(&self) -> i32 {
        self.mShapeId
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl std::ops::Deref for ShapeAnchor {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for ShapeAnchor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java  class CmdShapeIdChange extends Command
impl CmdShapeIdChange {
    // java   CmdShapeIdChange(ShapeAnchor node, int pOldShapeId)
    pub fn CmdShapeIdChange(node: *mut ShapeAnchor, p_old_shape_id: i32) -> Self {
      Self::new(node, p_old_shape_id, MapListenerBridge {
        map: std::ptr::null_mut(),
      })
    }

    pub fn new(
        node: *mut ShapeAnchor,
        p_old_shape_id: i32,
        outer_listener: MapListenerBridge,
    ) -> Self {
      // java this.node = node;
      // java this.shapeId = pOldShapeId;
      Self {
        super_: Command::new(),
        node,
        shapeId: p_old_shape_id,
        outer_listener,
      }
    }

    // java public void execute()
    pub fn execute(&mut self) {
      // java int pOldData = this.node.mShapeId;
      if self.node.is_null() {
        return;
      }
      let node = unsafe { &mut *self.node };
      let p_old_data = node.mShapeId;
      // java Assert.assertNotNull("mListener should not be null!", ShapeAnchor.this.mListener);
      Assert::assertNotNull__string_object(
        "mListener should not be null!",
        if self.outer_listener.map.is_null() {
          None::<()>
        } else {
          Some(())
        },
      );
      // java ShapeAnchor.this.mListener.beforeKeyChange(pOldData);
      self.outer_listener.before_key_change(p_old_data);
      // java this.node.setShapeId(this.shapeId);
      node.setShapeId(self.shapeId);
      // java ShapeAnchor.this.mListener.afterKeyChange(this.shapeId, this.node);
      self.outer_listener.after_key_change(self.shapeId, node);
      // java this.shapeId = pOldData;
      self.shapeId = p_old_data;
    }

    // java public void unexecute()
    pub fn unexecute(&mut self) {
      // java this.execute();
      self.execute();
    }

}

impl ActionCommand for CmdShapeIdChange {
    fn execute(&self) {}
    fn unexecute(&self) {}
    fn dispose(&self) {}
    fn get_level(&self) -> i32 {
        self.super_.level
    }
}

impl std::ops::Deref for CmdShapeIdChange {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CmdShapeIdChange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
