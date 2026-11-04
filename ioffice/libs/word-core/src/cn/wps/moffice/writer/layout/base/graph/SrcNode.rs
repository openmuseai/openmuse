// Java: import cn.wps.collections.KPList
use util::collections::KPList::{KPList, KPNode};
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.io.dom.Node
// CARGO_OFF_SURFACE use io_wps_io::cn::wps::io::dom::Node::Node;
use core::marker::PhantomData as _;

// java public static final int TYPE_SRC_INVALID = -1;
pub const TYPE_SRC_INVALID: i32 = -1;
// java public static final int TYPE_SRC_CORECHARS = 0;
pub const TYPE_SRC_CORECHARS: i32 = 0;
// java public static final int TYPE_SRC_AUTONUM = 1;
pub const TYPE_SRC_AUTONUM: i32 = 1;
// java public static final int TYPE_SRC_BALLOONINFO = 2;
pub const TYPE_SRC_BALLOONINFO: i32 = 2;
// java public static final int TYPE_SRC_MATH = 3;
pub const TYPE_SRC_MATH: i32 = 3;

// java public class SrcNode extends KPList.Node
pub struct SrcNode {
    // java KPList.Node super_
    pub super_: KPNode,
    // java public int type;
    pub r#type: i32,
    // java public boolean revisionFinal;
    pub revisionFinal: bool,
    // java public Document doc;
    pub doc: Option<*mut dyn Document>,
    // java public int cp;
    pub cp: i32,
    // java public int length;
    pub length: i32,
    // java public String data;
    pub data: Option<String>,
    // java public PropertySet finalRunProp;
    pub finalRunProp: Option<PropertySet>,
}

// java public static class Alloc implements KPList.INodeAlloc<SrcNode>
#[derive(Default)]
pub struct Alloc {
}

// java public class SrcNode extends KPList.Node
impl SrcNode {
    // java public void init()
    pub fn init(&mut self) {
      // java this.type = -1;
      self.r#type = -1;
      // java this.revisionFinal = true;
      self.revisionFinal = true;
      // java this.doc = null;
      self.doc = None;
      // java this.cp = -1;
      self.cp = -1;
      // java this.length = 0;
      self.length = 0;
      // java this.data = null;
      self.data = None;
      // java this.finalRunProp = null;
      self.finalRunProp = None;
    }

    // java private  SrcNode()
    pub fn SrcNode() -> Self {
      // java this.type = -1;
      // java this.cp = -1;
      Self {
        super_: KPNode::new(),
        r#type: -1,
        revisionFinal: false,
        doc: None,
        cp: -1,
        length: 0,
        data: None,
        finalRunProp: None,
      }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.doc = null;
      self.doc = None;
      // java this.data = null;
      self.data = None;
      // java this.finalRunProp = null;
      self.finalRunProp = None;
      // java super.dispose();
      self.super_.dispose();
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuffer strBuff = new StringBuffer("type=");
      let mut strBuff: String = String::from("type=");
      // java switch (this.type)
      match self.r#type {
        // java case 0:
        0 => {
          // java strBuff.append("CORECHARS cp=");
          strBuff.push_str("CORECHARS cp=");
          // java strBuff.append(this.cp);
          strBuff.push_str(&self.cp.to_string());
          // java strBuff.append(" length=");
          strBuff.push_str(" length=");
          // java strBuff.append(this.length);
          strBuff.push_str(&self.length.to_string());
          // java break;
        }
        // java case 1:
        1 => {
          // java strBuff.append("AUTONUM data=");
          strBuff.push_str("AUTONUM data=");
          // java strBuff.append(this.data);
          strBuff.push_str(self.data.as_deref().unwrap_or("null"));
          // java break;
        }
        // java case 2:
        2 => {
          // java strBuff.append("BALLOONINFO data=");
          strBuff.push_str("BALLOONINFO data=");
          // java strBuff.append(this.data);
          strBuff.push_str(self.data.as_deref().unwrap_or("null"));
          // java break;
        }
        // java case 3:
        3 => {
          // java strBuff.append("MATH data=");
          strBuff.push_str("MATH data=");
          // java strBuff.append(this.data);
          strBuff.push_str(self.data.as_deref().unwrap_or("null"));
          // java break;
        }
        // java default:
        _ => {
          // java strBuff.append("INVALID type=");
          strBuff.push_str("INVALID type=");
          // java strBuff.append(this.type);
          strBuff.push_str(&self.r#type.to_string());
        }
      }
      // java return strBuff.toString();
      return strBuff;
    }

}

// java public static class Alloc implements KPList.INodeAlloc<SrcNode>
impl Alloc {
    // java public SrcNode alloc()
    pub fn alloc(&mut self) -> SrcNode {
      // java return new SrcNode();
      return SrcNode::SrcNode();
    }

}

// rust: wr-29 GR-D1 —— KPList 真实节点池按 T::default() 构造；Java new SrcNode() 等价默认态
impl Default for SrcNode {
    fn default() -> Self {
        SrcNode::SrcNode()
    }
}
