// Java: import cn.wps.collections.KPList
use util::collections::KPList::{KPList, KPNode};
// Java: import cn.wps.moffice.drawing.geotext.GeoText
// CARGO_OFF_SURFACE use drawing::cn::wps::moffice::drawing::geotext::GeoText::GeoText;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.cache.style.TypoFontStyle
// rust: ★wr-29 04 —— fontStyles 用真实 cache::style::TypoFontStyle（Java cache.style.TypoFontStyle）
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle;
// Java: import cn.wps.io.dom.Node
// CARGO_OFF_SURFACE use io_wps_io::cn::wps::io::dom::Node::Node;
// Java: import cn.wps.moffice.writer.layout.base.graph.SrcNode
use crate::cn::wps::moffice::writer::layout::base::graph::SrcNode::SrcNode;

// java static final int TYPE_CHARS_INVALID = -1;
pub const TYPE_CHARS_INVALID: i32 = -1;
// java static final int TYPE_CHARS_REF = 0;
pub const TYPE_CHARS_REF: i32 = 0;
// java static final int TYPE_CHARS_RESULT = 1;
pub const TYPE_CHARS_RESULT: i32 = 1;
// java static final int TYPE_CHARS_OMATH = 2;
pub const TYPE_CHARS_OMATH: i32 = 2;
// java static final int TYPE_CHARS_MATH_FUNCTION = 3;
pub const TYPE_CHARS_MATH_FUNCTION: i32 = 3;
// java static final int TYPE_CHARS_EQUATION = 4;
pub const TYPE_CHARS_EQUATION: i32 = 4;

// java public class CharsNode extends KPList.Node
pub struct CharsNode {
    // java KPList.Node super_
    pub super_: KPNode,
    // java public int type;
    pub r#type: i32,
    // java public int offsetOfRef;
    pub offsetOfRef: i32,
    // java public int lengthOfRef;
    pub lengthOfRef: i32,
    // java public int hyperlinkIndex;
    pub hyperlinkIndex: i32,
    // java public SrcNode srcRef;
    pub srcRef: Option<*mut SrcNode>,
    // java public TypoFontStyle[] fontStyles;
    pub fontStyles: [Option<TypoFontStyle>; 10],
    // java boolean isFootEndNote;
    pub isFootEndNote: bool,
    // java boolean isResult;
    pub isResult: bool,
    // java public boolean isField;
    pub isField: bool,
    // java public boolean isRevisionDelete;
    pub isRevisionDelete: bool,
    // java public boolean isRevisionInsert;
    pub isRevisionInsert: bool,
    // java public boolean isRevisionFormatChange;
    pub isRevisionFormatChange: bool,
    // java public boolean isSnapToGrid;
    pub isSnapToGrid: bool,
    // java public boolean isVanishBreak;
    pub isVanishBreak: bool,
    // java public PropertySet finalRunProp;
    pub finalRunProp: Option<PropertySet>,
    // java public GeoText geoText;
    pub geoText: Option<()>,
    // java public boolean isInMath;
    pub isInMath: bool,
    // java public int hpsKern;
    pub hpsKern: i32,
    // java public boolean isBalloonAuthor;
    pub isBalloonAuthor: bool,
    // java public int revisionInsAuthor;
    pub revisionInsAuthor: i32,
    // java public int revisionDelAuthor;
    pub revisionDelAuthor: i32,
    // java public int revisionAuthor;
    pub revisionAuthor: i32,
}

// java public static class Alloc implements KPList.INodeAlloc<CharsNode>
#[derive(Default)]
pub struct Alloc {
}

// java public class CharsNode extends KPList.Node
impl CharsNode {
    // java public void init()
    pub fn init(&mut self) {
      // java this.type = -1;
      self.r#type = -1;
      // java this.offsetOfRef = -1;
      self.offsetOfRef = -1;
      // java this.lengthOfRef = 0;
      self.lengthOfRef = 0;
      // java this.hyperlinkIndex = -1;
      self.hyperlinkIndex = -1;
      // java this.srcRef = null;
      self.srcRef = None;
      // java this.finalRunProp = null;
      self.finalRunProp = None;
      // java TypoFontStyle[] styles = this.fontStyles;
      let styles: &mut [Option<TypoFontStyle>; 10] = &mut self.fontStyles;
      // java int i = 0;
      let mut i: i32 = 0;
      // java for(int size = styles.length; i < size; ++i)
      let size: i32 = styles.len() as i32;
      while i < size {
        // java styles[i] = null;
        styles[i as usize] = None;
        i += 1;
      }
      // java this.isFootEndNote = false;
      self.isFootEndNote = false;
      // java this.isResult = false;
      self.isResult = false;
      // java this.isField = false;
      self.isField = false;
      // java this.isRevisionDelete = false;
      self.isRevisionDelete = false;
      // java this.isRevisionInsert = false;
      self.isRevisionInsert = false;
      // java this.isRevisionFormatChange = false;
      self.isRevisionFormatChange = false;
      // java this.isSnapToGrid = false;
      self.isSnapToGrid = false;
      // java this.isVanishBreak = false;
      self.isVanishBreak = false;
      // java this.geoText = null;
      self.geoText = None;
      // java this.isInMath = false;
      self.isInMath = false;
      // java this.hpsKern = 0;
      self.hpsKern = 0;
      // java this.isBalloonAuthor = false;
      self.isBalloonAuthor = false;
      // java this.revisionInsAuthor = -1;
      self.revisionInsAuthor = -1;
      // java this.revisionDelAuthor = -1;
      self.revisionDelAuthor = -1;
      // java this.revisionAuthor = -1;
      self.revisionAuthor = -1;
    }

    // java private  CharsNode()
    pub fn CharsNode() -> Self {
      // java this.type = -1;
      // java this.offsetOfRef = -1;
      // java this.hyperlinkIndex = -1;
      // java this.fontStyles = new TypoFontStyle[10];
      // java this.revisionInsAuthor = -1;
      // java this.revisionDelAuthor = -1;
      // java this.revisionAuthor = -1;
      Self {
        super_: KPNode::new(),
        r#type: -1,
        offsetOfRef: -1,
        lengthOfRef: 0,
        hyperlinkIndex: -1,
        srcRef: None,
        fontStyles: Default::default(),
        isFootEndNote: false,
        isResult: false,
        isField: false,
        isRevisionDelete: false,
        isRevisionInsert: false,
        isRevisionFormatChange: false,
        isSnapToGrid: false,
        isVanishBreak: false,
        finalRunProp: None,
        geoText: None,
        isInMath: false,
        hpsKern: 0,
        isBalloonAuthor: false,
        revisionInsAuthor: -1,
        revisionDelAuthor: -1,
        revisionAuthor: -1,
      }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.srcRef = null;
      self.srcRef = None;
      // java this.finalRunProp = null;
      self.finalRunProp = None;
      // java super.dispose();
      self.super_.dispose();
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuffer stringBuff = new StringBuffer();
      let mut stringBuff: String = String::new();
      // java switch (this.type)
      match self.r#type {
        // java case 0:
        0 => {
          // java stringBuff.append("REF");
          stringBuff.push_str("REF");
          // java break;
        }
        // java case 1:
        1 => {
          // java stringBuff.append("RESULT");
          stringBuff.push_str("RESULT");
          // java break;
        }
        // java case 2:
        2 => {
          // java stringBuff.append("OMATH");
          stringBuff.push_str("OMATH");
          // java break;
        }
        // java case 3:
        3 => {
          // java stringBuff.append("MATH_FUNCTION");
          stringBuff.push_str("MATH_FUNCTION");
          // java break;
        }
        // java case 4:
        4 => {
          // java stringBuff.append("EQUATION");
          stringBuff.push_str("EQUATION");
          // java break;
        }
        // java default:
        _ => {
          // java stringBuff.append("INVALID ");
          stringBuff.push_str("INVALID ");
          // java stringBuff.append("type=" + this.type);
          stringBuff.push_str(&format!("type={}", self.r#type));
        }
      }
      // java stringBuff.append(" offsetOfRef=" + this.offsetOfRef);
      stringBuff.push_str(&format!(" offsetOfRef={}", self.offsetOfRef));
      // java stringBuff.append(" lengthOfRef=" + this.lengthOfRef);
      stringBuff.push_str(&format!(" lengthOfRef={}", self.lengthOfRef));
      // java stringBuff.append("[");
      stringBuff.push_str("[");
      // java stringBuff.append(this.srcRef.toString());
      // rust: Java NPE if srcRef==null; tests cover None as "null"
      match self.srcRef {
        Some(p) => stringBuff.push_str(&unsafe { (*p).toString() }),
        None => stringBuff.push_str("null"),
      }
      // java stringBuff.append("]");
      stringBuff.push_str("]");
      // java return stringBuff.toString();
      return stringBuff;
    }

}

// java public static class Alloc implements KPList.INodeAlloc<CharsNode>
impl Alloc {
    // java public CharsNode alloc()
    pub fn alloc(&mut self) -> CharsNode {
      // java return new CharsNode();
      return CharsNode::CharsNode();
    }

}

// rust: wr-29 GR-D1 —— KPList 真实节点池按 T::default() 构造；Java new CharsNode() 等价默认态
impl Default for CharsNode {
    fn default() -> Self {
        CharsNode::CharsNode()
    }
}

impl Clone for CharsNode {
    fn clone(&self) -> Self {
        Self {
            super_: KPNode { next: self.super_.next, prev: self.super_.prev },
            r#type: self.r#type,
            offsetOfRef: self.offsetOfRef,
            lengthOfRef: self.lengthOfRef,
            hyperlinkIndex: self.hyperlinkIndex,
            srcRef: self.srcRef,
            fontStyles: self.fontStyles.clone(),
            isFootEndNote: self.isFootEndNote,
            isResult: self.isResult,
            isField: self.isField,
            isRevisionDelete: self.isRevisionDelete,
            isRevisionInsert: self.isRevisionInsert,
            isRevisionFormatChange: self.isRevisionFormatChange,
            isSnapToGrid: self.isSnapToGrid,
            isVanishBreak: self.isVanishBreak,
            finalRunProp: self.finalRunProp.as_ref().map(|p| p.clone()),
            geoText: self.geoText,
            isInMath: self.isInMath,
            hpsKern: self.hpsKern,
            isBalloonAuthor: self.isBalloonAuthor,
            revisionInsAuthor: self.revisionInsAuthor,
            revisionDelAuthor: self.revisionDelAuthor,
            revisionAuthor: self.revisionAuthor,
        }
    }
}

