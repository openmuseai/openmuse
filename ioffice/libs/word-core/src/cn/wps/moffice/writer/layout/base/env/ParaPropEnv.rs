// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::ITextRope;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::SHD;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
// Java: import cn.wps.moffice.writer.data.PLCField
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::PLCField::PLCField;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::BRC;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::Entry;

// java public class ParaPropEnv
#[derive(Clone)]
pub struct ParaPropEnv {
    // java public BRC brcTop = null;
    pub brcTop: Option<BRC>,
    // java public BRC brcLeft = null;
    pub brcLeft: Option<BRC>,
    // java public BRC brcRight = null;
    pub brcRight: Option<BRC>,
    // java public BRC brcBottom = null;
    pub brcBottom: Option<BRC>,
    // java public BRC lastBrcBetween = null;
    pub lastBrcBetween: Option<BRC>,
    // java public SHD paraSHD = null;
    pub paraSHD: Option<SHD>,
    // java public int leftIndent = 0;
    pub leftIndent: i32,
    // java public int rightIndent = 0;
    pub rightIndent: i32,
    // java public int firstLineIndent = 0;
    pub firstLineIndent: i32,
    // java public int paraBefore = 0;
    pub paraBefore: i32,
    // java public int paraAfter = 0;
    pub paraAfter: i32,
    // java public PropertySet paraProperty = null;
    pub paraProperty: Option<PropertySet>,
    // java public PropertySet firstNormalParaProperty = null;
    pub firstNormalParaProperty: Option<PropertySet>,
    // java public boolean vaildBRC = false;
    pub vaildBRC: bool,
    // java public int start = -1;
    pub start: i32,
    // java public int end = -1;
    pub end: i32,
    // java public boolean hasAutoNums = false;
    pub hasAutoNums: bool,
    // java public String autoNumString;
    pub autoNumString: Option<String>,
    // java public PropertySet autoNumRunProp;
    pub autoNumRunProp: Option<PropertySet>,
    // java public int autoNumsJC;
    pub autoNumsJC: i32,
}

// java public class ParaPropEnv
impl ParaPropEnv {
    // java public void init()
    pub fn init(&mut self) {
      // java this.brcTop = null;
      self.brcTop = None;
      // java this.brcLeft = null;
      self.brcLeft = None;
      // java this.brcRight = null;
      self.brcRight = None;
      // java this.brcBottom = null;
      self.brcBottom = None;
      // java this.lastBrcBetween = null;
      self.lastBrcBetween = None;
      // java this.paraSHD = null;
      self.paraSHD = None;
      // java this.paraProperty = null;
      self.paraProperty = None;
      // java this.firstNormalParaProperty = null;
      self.firstNormalParaProperty = None;
      // java this.vaildBRC = false;
      self.vaildBRC = false;
      // java this.hasAutoNums = false;
      self.hasAutoNums = false;
      // java this.autoNumString = null;
      self.autoNumString = None;
      // java this.autoNumRunProp = null;
      self.autoNumRunProp = None;
      // java this.autoNumsJC = 0;
      self.autoNumsJC = 0;
    }

    // java public void setLayoutRange(int layoutStart, int layoutEnd, Document document)
    pub fn setLayoutRange(&mut self, layoutStart: i32, layoutEnd: i32, document: &dyn Document) {
      // java int startCheckLen = 8000;
      let startCheckLen: i32 = 8000;
      // java int limitMaxLeng = 10000;
      let limitMaxLeng: i32 = 10000;
      // java int oriLen = layoutEnd - layoutStart;
      let oriLen: i32 = layoutEnd - layoutStart;
      // java if (oriLen > 10000 && this.allowCutFlow(layoutStart + 10000, document))
      if oriLen > limitMaxLeng && self.allowCutFlow(layoutStart + limitMaxLeng, document) {
        // java this.end = layoutStart + 10000;
        self.end = layoutStart + limitMaxLeng;
      }
      // java else
      else {
        // java if (oriLen > 8000)
        if oriLen > startCheckLen {
          // java char[] buffer = new char[256];
          let mut buffer: Vec<char> = vec!['\0'; 256];
          // java int thisRead;
          let mut thisRead: i32;
          let mut start: i32 = layoutStart;
          // java for(int start = layoutStart; start < layoutEnd start += thisRead)
          while start < layoutEnd {
            // java thisRead = Math.min(layoutEnd - start, buffer.length);
            thisRead = (layoutEnd - start).min(buffer.len() as i32);
            // java document.getText(start, start + thisRead, buffer, 0);
            document.getText(start, start + thisRead, &mut buffer, 0);
            let mut i: i32 = 0;
            // java for(int i = 0; i < thisRead; ++i)
            while i < thisRead {
              // java char c = buffer[i];
              let c: char = buffer[i as usize];
              // java if (c == 11 || c == '\n')
              if c == '\u{000b}' || c == '\n' {
                // java this.end = start + i + 1;
                self.end = start + i + 1;
                // java return;
                return;
              }
              i += 1;
            }
            start += thisRead;
          }
        }
        // java this.end = layoutEnd;
        self.end = layoutEnd;
      }
    }

    // java private boolean allowCutFlow(int flowCp, Document document)
    pub fn allowCutFlow(&self, flowCp: i32, document: &dyn Document) -> bool {
      // java IPapxTable.Entry para = document.getPapxTable().seek(flowCp);
      let Some(papx) = document.getPapxTable() else { return false; };
      let Some(para) = (unsafe { crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable::seek(&*papx, flowCp) }) else { return false; };
      // java PLCField.FieldTriple fieldTriple = document.getPLCField().getContainingField(flowCp);
      let fieldTriple = document.getPLCField().and_then(|field| unsafe { (&mut *field).getContainingField(flowCp) });
      // java if (null != fieldTriple)
      if fieldTriple.is_some() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java int paraStart = para.position();
        let paraStart: i32 = para.position();
        // java ITextRope textRope = document.getTextRope();
        let Some(textRope) = document.getTextRope() else { return false; };
        // java if (this.hasAntherChar(textRope.charAt(flowCp)))
        if self.hasAntherChar(unsafe { (&mut *textRope).charAt(flowCp) }) {
          // java return false;
          return false;
        }
        // java else
        else {
          // java if (flowCp > paraStart)
          if flowCp > paraStart {
            // java --flowCp;
            let flowCp = flowCp - 1;
            // java if (this.hasAntherChar(textRope.charAt(flowCp)))
            if self.hasAntherChar(unsafe { (&mut *textRope).charAt(flowCp) }) {
              // java return false;
              return false;
            }
          }
          // java return true;
          return true;
        }
      }
    }

    // java private boolean hasAntherChar(char ch)
    pub fn hasAntherChar(&self, ch: char) -> bool {
      // java boolean result = ch == '\b' || ch == 5 || ch == 1 || ch == 19 || ch == 21;
      let result: bool = ch == '\u{0008}' || ch == '\u{0005}' || ch == '\u{0001}' || ch == '\u{0013}' || ch == '\u{0015}';
      // java return result;
      return result;
    }

}

impl Default for ParaPropEnv {
    fn default() -> Self {
        Self {
            brcTop: None,
            brcLeft: None,
            brcRight: None,
            brcBottom: None,
            lastBrcBetween: None,
            paraSHD: None,
            leftIndent: 0,
            rightIndent: 0,
            firstLineIndent: 0,
            paraBefore: 0,
            paraAfter: 0,
            paraProperty: None,
            firstNormalParaProperty: None,
            vaildBRC: false,
            start: -1,
            end: -1,
            hasAutoNums: false,
            autoNumString: None,
            autoNumRunProp: None,
            autoNumsJC: 0,
        }
    }
}
