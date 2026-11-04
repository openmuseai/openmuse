// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public static final int tcHoriNoneMerged = 0;
pub(crate) const tcHoriNoneMerged: i32 = 0;
// java public static final int tcHoriFirstMerge = 3;
pub(crate) const tcHoriFirstMerge: i32 = 3;
// java public static final int tcHoriMerged = 2;
pub(crate) const tcHoriMerged: i32 = 2;
// java public static final int tcVertNoneMerged = 0;
pub(crate) const tcVertNoneMerged: i32 = 0;
// java public static final int tcVertFirstMerge = 3;
pub(crate) const tcVertFirstMerge: i32 = 3;
// java public static final int tcVertMerged = 1;
pub(crate) const tcVertMerged: i32 = 1;
// java public static final int tcVAlignTop = 0;
pub(crate) const tcVAlignTop: i32 = 0;
// java public static final int tcVAlignCenter = 1;
pub(crate) const tcVAlignCenter: i32 = 1;
// java public static final int tcVAlignBottom = 2;
pub(crate) const tcVAlignBottom: i32 = 2;

// java public abstract class TableCell
#[derive(Clone)]
pub struct TableCell {
    // java int horiMerge;
    pub(crate) horiMerge: i32,
    // java int textFlow;
    pub(crate) textFlow: i32,
    // java int vertMerge;
    pub(crate) vertMerge: i32,
    // java int vertAlign;
    pub(crate) vertAlign: i32,
    // java boolean fFitText;
    pub(crate) fFitText: bool,
    // java boolean fNoWrap;
    pub(crate) fNoWrap: bool,
    // java int widthType;
    pub(crate) widthType: i32,
    // java int width;
    pub(crate) width: i32,
    // java BRC topBrc;
    pub(crate) topBrc: Option<BRC>,
    // java BRC leftBrc;
    pub(crate) leftBrc: Option<BRC>,
    // java BRC bottomBrc;
    pub(crate) bottomBrc: Option<BRC>,
    // java BRC rightBrc;
    pub(crate) rightBrc: Option<BRC>,
    // java int hash = 0;
    pub(crate) hash: i32,
}

// java public abstract class TableCell
impl TableCell {
    // java public  TableCell()
    pub fn new() -> Self {
      // java this.initDefault();
      let mut this = Self {
        horiMerge: 0,
        textFlow: 0,
        vertMerge: 0,
        vertAlign: 0,
        fFitText: false,
        fNoWrap: false,
        widthType: 1,
        width: 0,
        topBrc: None,
        leftBrc: None,
        bottomBrc: None,
        rightBrc: None,
        hash: 0,
      };
      this.initDefault();
      this
    }

    // java protected void initDefault()
    pub fn initDefault(&mut self) {
      // java this.horiMerge = 0;
      self.horiMerge = 0;
      // java this.vertMerge = 0;
      self.vertMerge = 0;
      // java this.vertAlign = 0;
      self.vertAlign = 0;
      // java this.textFlow = 0;
      self.textFlow = 0;
      // java this.fFitText = false;
      self.fFitText = false;
      // java this.fNoWrap = false;
      self.fNoWrap = false;
      // java this.width = 0;
      self.width = 0;
      // java this.widthType = 1;
      self.widthType = 1;
      // java this.topBrc = null;
      self.topBrc = None;
      // java this.leftBrc = null;
      self.leftBrc = None;
      // java this.bottomBrc = null;
      self.bottomBrc = None;
      // java this.rightBrc = null;
      self.rightBrc = None;
      // java this.hash = 0;
      self.hash = 0;
    }

    // java public  TableCell(TableCell tc)
    pub fn new__tablecell(tc: &TableCell) -> Self {
      // java this.init(tc);
      let mut this = Self::new();
      this.init(Some(tc));
      this
    }

    // java public void init(TableCell tc)
    pub fn init(&mut self, tc: Option<&TableCell>) {
      // java if (tc == null)
      if let Some(tc) = tc {
        // java this.horiMerge = tc.horiMerge;
        self.horiMerge = tc.horiMerge;
        // java this.vertMerge = tc.vertMerge;
        self.vertMerge = tc.vertMerge;
        // java this.vertAlign = tc.vertAlign;
        self.vertAlign = tc.vertAlign;
        // java this.textFlow = tc.textFlow;
        self.textFlow = tc.textFlow;
        // java this.fFitText = tc.fFitText;
        self.fFitText = tc.fFitText;
        // java this.fNoWrap = tc.fNoWrap;
        self.fNoWrap = tc.fNoWrap;
        // java this.width = tc.width;
        self.width = tc.width;
        // java this.widthType = tc.widthType;
        self.widthType = tc.widthType;
        // java this.topBrc = tc.topBrc;
        self.topBrc = tc.topBrc.clone();
        // java this.leftBrc = tc.leftBrc;
        self.leftBrc = tc.leftBrc.clone();
        // java this.bottomBrc = tc.bottomBrc;
        self.bottomBrc = tc.bottomBrc.clone();
        // java this.rightBrc = tc.rightBrc;
        self.rightBrc = tc.rightBrc.clone();
        // java this.hash = 0;
        self.hash = 0;
      }
      // java else
      else {
        // java this.initDefault();
        self.initDefault();
      }
    }

    // java public int getHoriMerge()
    pub fn getHoriMerge(&self) -> i32 {
      // java return this.horiMerge;
      return self.horiMerge;
    }

    // java public int getWidth()
    pub fn getWidth(&self) -> i32 {
      // java return this.width;
      return self.width;
    }

    // java public int getWidthType()
    pub fn getWidthType(&self) -> i32 {
      // java return this.widthType;
      return self.widthType;
    }

    // java public int getVertMerge()
    pub fn getVertMerge(&self) -> i32 {
      // java return this.vertMerge;
      return self.vertMerge;
    }

    // java public int getVertAlign()
    pub fn getVertAlign(&self) -> i32 {
      // java return this.vertAlign;
      return self.vertAlign;
    }

    // java public int getTextFlow()
    pub fn getTextFlow(&self) -> i32 {
      // java return this.textFlow;
      return self.textFlow;
    }

    // java public boolean getFitText()
    pub fn getFitText(&self) -> bool {
      // java return this.fFitText;
      return self.fFitText;
    }

    // java public boolean getNoWrap()
    pub fn getNoWrap(&self) -> bool {
      // java return this.fNoWrap;
      return self.fNoWrap;
    }

    // java public BRC getBrcTop()
    pub fn getBrcTop(&self) -> Option<BRC> {
      // java return this.topBrc;
      return self.topBrc.clone();
    }

    // java public BRC getBrcLeft()
    pub fn getBrcLeft(&self) -> Option<BRC> {
      // java return this.leftBrc;
      return self.leftBrc.clone();
    }

    // java public BRC getBrcBottom()
    pub fn getBrcBottom(&self) -> Option<BRC> {
      // java return this.bottomBrc;
      return self.bottomBrc.clone();
    }

    // java public BRC getBrcRight()
    pub fn getBrcRight(&self) -> Option<BRC> {
      // java return this.rightBrc;
      return self.rightBrc.clone();
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder sb = new StringBuilder();
      let mut sb = String::new();
      // java sb.append("TableCell:");
      sb.push_str("TableCell:");
      // java sb.append("\nhoriMerge = " + this.horiMerge);
      sb.push_str(&format!("\nhoriMerge = {}", self.horiMerge));
      // java sb.append("\nvertMerge = " + this.vertMerge);
      sb.push_str(&format!("\nvertMerge = {}", self.vertMerge));
      // java sb.append("\ntextFlow = " + this.textFlow);
      sb.push_str(&format!("\ntextFlow = {}", self.textFlow));
      // java sb.append("\nfFitText = " + this.fFitText);
      sb.push_str(&format!("\nfFitText = {}", self.fFitText));
      // java sb.append("\nfNoWrap = " + this.fNoWrap);
      sb.push_str(&format!("\nfNoWrap = {}", self.fNoWrap));
      // java sb.append("\nwidth = " + this.width);
      sb.push_str(&format!("\nwidth = {}", self.width));
      // java sb.append("\nwidthType = " + this.widthType);
      sb.push_str(&format!("\nwidthType = {}", self.widthType));
      // java sb.append("\nbrc = {");
      sb.push_str("\nbrc = {");
      // java sb.append("\n\t" + this.topBrc);
      sb.push_str(&format!("\n\t{}", Self::brc_text(self.topBrc.as_ref())));
      // java sb.append("\n\t" + this.leftBrc);
      sb.push_str(&format!("\n\t{}", Self::brc_text(self.leftBrc.as_ref())));
      // java sb.append("\n\t" + this.bottomBrc);
      sb.push_str(&format!("\n\t{}", Self::brc_text(self.bottomBrc.as_ref())));
      // java sb.append("\n\t" + this.rightBrc);
      sb.push_str(&format!("\n\t{}", Self::brc_text(self.rightBrc.as_ref())));
      // java sb.append("\n}");
      sb.push_str("\n}");
      // java return sb.toString();
      return sb;
    }

    fn brc_text(brc: Option<&BRC>) -> String {
      match brc {
        None => "null".to_string(),
        Some(brc) => format!("BRC(type={}, color={})", brc.brcType, brc.color),
      }
    }

    // java public boolean equals(Object value)
    pub fn equals__object(&self, value: &dyn Object) -> bool {
      // java if (value == this)
      if let Some(tableCell) = value.as_any().downcast_ref::<TableCell>() {
        if std::ptr::eq(self, tableCell) {
          // java return true;
          return true;
        }
        // java boolean bEqual = this.horiMerge == tableCell.horiMerge && this.textFlow == tableCell.textFlow && this.vertAlign == tableCell.vertAlign && this.vertMerge == tableCell.vertMerge && this.fFitText == tableCell.fFitText && this.fNoWrap == tableCell.fNoWrap && this.width == tableCell.width && this.widthType == tableCell.widthType;
        let bEqual = self.horiMerge == tableCell.horiMerge && self.textFlow == tableCell.textFlow && self.vertAlign == tableCell.vertAlign && self.vertMerge == tableCell.vertMerge && self.fFitText == tableCell.fFitText && self.fNoWrap == tableCell.fNoWrap && self.width == tableCell.width && self.widthType == tableCell.widthType;
        // java if (!bEqual)
        if !bEqual {
          // java return false;
          return false;
        }
        // java else
        // java return equals(this.topBrc, tableCell.topBrc) && equals(this.leftBrc, tableCell.leftBrc) && equals(this.bottomBrc, tableCell.bottomBrc) && equals(this.rightBrc, tableCell.rightBrc);
        return Self::equals__brc_brc(self.topBrc.as_ref(), tableCell.topBrc.as_ref())
          && Self::equals__brc_brc(self.leftBrc.as_ref(), tableCell.leftBrc.as_ref())
          && Self::equals__brc_brc(self.bottomBrc.as_ref(), tableCell.bottomBrc.as_ref())
          && Self::equals__brc_brc(self.rightBrc.as_ref(), tableCell.rightBrc.as_ref());
      }
      // java else if (!(value instanceof TableCell))
      // java return false;
      return false;
    }

    // java private static final boolean equals(BRC brc1, BRC brc2)
    pub fn equals__brc_brc(brc1: Option<&BRC>, brc2: Option<&BRC>) -> bool {
      // java return brc1 == null ? brc2 == null : brc1.equals(brc2);
      match (brc1, brc2) {
        (None, None) => true,
        (Some(a), Some(b)) => {
          (a.dptLineWidth * 8.0) as i32 == (b.dptLineWidth * 8.0) as i32
            && a.brcType == b.brcType
            && a.color == b.color
            && (a.dptSpace * 8.0) as i32 == (b.dptSpace * 8.0) as i32
            && a.fShadow == b.fShadow
            && a.fFrame == b.fFrame
        }
        _ => false,
      }
    }

    // java private static final int hash(BRC brc)
    pub fn hash(brc: Option<&BRC>) -> i32 {
      // java return brc == null ? 0 : brc.hashCode();
      match brc {
        None => 0,
        Some(brc) => {
          (brc.dptLineWidth * 8.0) as i32
            + brc.brcType
            + brc.color
            + (brc.dptSpace * 8.0) as i32
            + if brc.fShadow { 1 } else { 0 }
            + if brc.fFrame { 1 } else { 0 }
        }
      }
    }

    // java public int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // java if (this.hash == 0)
      if self.hash == 0 {
        // java this.hash = this.horiMerge + this.textFlow + this.vertAlign + this.vertMerge + (this.fFitText ? 1 : 0) + (this.fNoWrap ? 1 : 0) + this.width + this.widthType + hash(this.topBrc) + hash(this.leftBrc) + hash(this.bottomBrc) + hash(this.rightBrc);
        self.hash = self.horiMerge + self.textFlow + self.vertAlign + self.vertMerge + (if self.fFitText { 1 } else { 0 }) + (if self.fNoWrap { 1 } else { 0 }) + self.width + self.widthType + Self::hash(self.topBrc.as_ref()) + Self::hash(self.leftBrc.as_ref()) + Self::hash(self.bottomBrc.as_ref()) + Self::hash(self.rightBrc.as_ref());
      }
      // java return this.hash;
      return self.hash;
    }

}

impl Default for TableCell {
    fn default() -> Self {
        Self::new()
    }
}
