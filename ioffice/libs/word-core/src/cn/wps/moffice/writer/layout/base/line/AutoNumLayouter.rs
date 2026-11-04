// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::IChpxTable;
// Java: import cn.wps.moffice.writer.data.autonum.AutoNumCount
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::data::autonum::AutoNumCount::AutoNumCount;
// Java: import cn.wps.moffice.writer.data.autonum.LstLfoTool
use crate::cn::wps::moffice::writer::data::autonum::LstLfoTool::LstLfoTool;
// Java: import cn.wps.moffice.writer.data.autonum.LvlfData
use crate::cn::wps::moffice::writer::data::autonum::LvlfData::LvlfData;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.IPropertyTool
use crate::cn::wps::moffice::writer::layout::base::env::IPropertyTool::IPropertyTool;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaEnv
use crate::cn::wps::moffice::writer::layout::base::env::ParaEnv::ParaEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaPropEnv
use crate::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
use crate::cn::wps::moffice::writer::layout::base::ParaLayouter::IStateEnvoy;
// Java: import cn.wps.moffice.writer.layout.base.graph.symbolconvert.ISymbolConvert
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::graph::symbolconvert::ISymbolConvert::ISymbolConvert;
// Java: import cn.wps.moffice.writer.layout.numformat.Numformat
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::numformat::Numformat::Numformat;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::Entry;


// CARGO_OFF_SURFACE_TYPES
#[derive(Default, Clone)]
pub struct AutoNumCount;
#[derive(Default, Clone)]
pub struct AutoNum {
    pub mILfo: i32,
    pub mILvl: i32,
    pub mCounts: Option<Vec<i32>>,
}
pub trait ISymbolConvert {}
// Java: AutoNumLayouter.getResult walks xst placeholders and Numformat.txFormatNum.
fn format_auto_num(out: &mut String, auto_num: &AutoNum, tool: &LstLfoTool, lvlf_data: &LvlfData) -> Option<()> {
    use crate::cn::wps::moffice::writer::layout::numformat::Numformat::Numformat;
    use stubs::java::lang::StringBuffer::StringBuffer;
    let xst = lvlf_data.getXstStr()?;
    let xch = lvlf_data.getrgbxchNums();
    let counts = auto_num.mCounts.as_ref()?;
    out.clear();
    let units: Vec<u16> = xst.encode_utf16().collect();
    let mut ixch = 0usize;
    let mut ixst = 0usize;
    while ixst < units.len() {
        let mark = xch.get(ixch).map(|b| (*b as i8) as i32).unwrap_or(i32::MIN);
        if ixch < xch.len() && ixst as i32 == mark - 1 {
            ixch += 1;
            let n_ilvl = units[ixst] as i32;
            if n_ilvl >= 0 && n_ilvl < tool.getLevelCount() && (n_ilvl as usize) < counts.len() {
                let lvlf2 = tool.getLvlfData__int(n_ilvl);
                let count2 = counts[n_ilvl as usize];
                let mut mso_nfc = if lvlf_data.isLegal() {
                    if n_ilvl == 0 { 0 } else { lvlf_data.getNfc() }
                } else {
                    lvlf2.as_ref().map(|level| level.getNfc()).unwrap_or(-1)
                };
                if mso_nfc == -1 {
                    mso_nfc = 255;
                }
                if mso_nfc != 255 {
                    let mut buf = StringBuffer::new();
                    Numformat::txFormatNum__int_int_stringbuffer(count2, mso_nfc, Some(&mut buf));
                    out.push_str(&buf.toString());
                }
            }
        } else {
            let ch = char::from_u32(units[ixst] as u32).unwrap_or('\u{FFFD}');
            if ch != '\t' {
                out.push(ch);
            }
        }
        ixst += 1;
    }
    Some(())
}

// java public class AutoNumLayouter
#[derive(Default)]
pub struct AutoNumLayouter {
    // java private DocumentEnv mDocEnv;
    pub(crate) mDocEnv: Option<DocumentEnv>,
    // java private ViewEnv mViewEnv;
    pub(crate) mViewEnv: Option<ViewEnv>,
    // java private StringBuilder mResult = new StringBuilder();
    mResult: String,
}

// java public class AutoNumLayouter
impl AutoNumLayouter {
    // java public  AutoNumLayouter(DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn AutoNumLayouter(docEnv: DocumentEnv, viewEnv: &ViewEnv) -> Self {
      let _ = viewEnv;
      // java this.mDocEnv = docEnv;
      let mDocEnv = Some(docEnv);
      // java this.mViewEnv = viewEnv;
      let mViewEnv = Some(ViewEnv::default());
      Self { mDocEnv, mViewEnv, mResult: String::new() }
    }

    // java public void verifyData(IStateEnvoy state)
    pub fn verifyData(&self, state: Option<&dyn IStateEnvoy>) {
      // java if (null == state)
      if state.is_none() {
        // java throw new IllegalStateException("illegal data");
        panic!("illegal data");
      }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mDocEnv = null;
      self.mDocEnv = None;
      // java this.mViewEnv = null;
      self.mViewEnv = None;
    }

    // java private AutoNumCount.AutoNum getCurAutoNum(ParaPropEnv curProp, IStateEnvoy state)
    pub fn getCurAutoNum(&mut self, curProp: &ParaPropEnv, state: &dyn IStateEnvoy) -> Option<AutoNum> {
      // java PropertySet paraProperty = curProp.paraProperty;
      let paraProperty = curProp.paraProperty.as_ref()?;
      // java if (paraProperty.contains(235))
      if paraProperty.contains(235) {
        // java int lfo = paraProperty.getInt(235);
        let lfo: i32 = paraProperty.getInt(235);
        // java int lvl = paraProperty.getInt(234, 0);
        let lvl: i32 = paraProperty.getInt__int_int(234, 0);
        // java LstLfoTool lstLfoTool = this.mDocEnv.textDocument.getAutoNumTable().getLstLfoTool();
        let existing = self.mDocEnv.as_mut()
          .and_then(|env| env.textDocument_mut())
          .is_some_and(|doc| doc.getAutoNumTable().level_exists(lfo, lvl));
        if std::env::var("WORD_PARITY_AUTONUM_TRACE_CP").ok().and_then(|s| s.parse::<i32>().ok())
          .is_some_and(|cp| curProp.start <= cp && cp < curProp.end) {
          eprintln!("autonum level start={} lfo={} lvl={} exists={}", curProp.start, lfo, lvl, existing);
        }
        // java if (lstLfoTool.isExisting(lfo, lvl))
        if existing {
          // java AutoNumCount autoNumCount = state.getDocument().getAutoNumCount();
          // java autoNumCount.constructTo(curProp.end);
          // java return autoNumCount.getAutoNum(lfo, lvl, curProp.start);
          let Some(counter) = state.getDocument().getAutoNumCount() else { return None };
          if counter.is_null() { return None; }
          let counter = unsafe { &mut *counter };
          counter.constructTo(curProp.end);
          let real = counter.getAutoNum(lfo, lvl, curProp.start)?;
          return Some(AutoNum { mILfo: real.mILfo, mILvl: real.mILvl as i32, mCounts: Some(real.mCounts) });
        }
      }
      // java return null;
      return None;
    }

    // java public void calAutoNumString(ParaPropEnv propEnv, IStateEnvoy state)
    pub fn calAutoNumString(&mut self, propEnv: &mut ParaPropEnv, state: &dyn IStateEnvoy) {
      // java this.verifyData(state);
      self.verifyData(Some(state));
      // java AutoNumCount autoNumCount = state.getDocument().getAutoNumCount();
      // java if (null != autoNumCount) autoNumCount.init();
      if let Some(counter) = state.getDocument().getAutoNumCount() {
        if !counter.is_null() {
          unsafe { &mut *counter }.init();
        }
      }
      // java AutoNumCount.AutoNum autoNum = this.getCurAutoNum(propEnv, state);
      let autoNum = self.getCurAutoNum(propEnv, state);
      if std::env::var("WORD_PARITY_AUTONUM_TRACE_CP").ok().and_then(|s| s.parse::<i32>().ok())
        .is_some_and(|cp| propEnv.start <= cp && cp < propEnv.end) {
        eprintln!("autonum count start={} found={}", propEnv.start, autoNum.is_some());
      }
      // java if (null == autoNum)
      if autoNum.is_none() {
        // java this.initEmptyAutoNum(propEnv, state);
        self.initEmptyAutoNum(propEnv, state);
      }
      // java else
      else {
        let autoNum = autoNum.unwrap();
        // java LstLfoTool lstLfoTool = this.mDocEnv.textDocument.getAutoNumTable().getLstLfoTool();
        let lvlfData = self.mDocEnv.as_mut().and_then(|env| {
          let doc = env.textDocument_mut()?;
          let tool = doc.getAutoNumTable().getLstLfoTool();
          // java if (lstLfoTool.init(autoNum.mILfo, autoNum.mILvl))
          if tool.init(autoNum.mILfo, autoNum.mILvl) {
            // java LvlfData lvlfData = lstLfoTool.getLvlfData();
            tool.getLvlfData()
          } else {
            None
          }
        });
        if let Some(lvlfData) = lvlfData {
          // java Assert.assertNotNull("lvlfData should not be null.", lvlfData);
          // rust: no-stmt
          // java if (!lvlfData.getRunProp().getBoolean(18, false))
          let compat = lvlfData.getRunProp().as_ref().is_some_and(|p| p.getBoolean__int_boolean(18, false));
          if !compat {
            // java PropertySet runProp = this.getAutoNumStrRunProperty(propEnv, lvlfData, state);
            let Some(runProp) = self.getAutoNumStrRunProperty(propEnv, &lvlfData, state) else { return; };
            let strResult = if lvlfData.getNfc() == 23 {
              self.getBulletChar(&lvlfData, &runProp.getString__int_string(3, ""))
            } else {
              self.mDocEnv.as_mut().and_then(|env| {
                let doc = env.textDocument_mut()?;
                let tool = doc.getAutoNumTable().getLstLfoTool();
                if !tool.init(autoNum.mILfo, autoNum.mILvl) {
                  return None;
                }
                let mut buf = String::new();
                format_auto_num(&mut buf, &autoNum, tool, &lvlfData)?;
                Some(buf)
              })
            };
            let mut sb = String::new();
            if let Some(s) = strResult.as_ref() {
              if !s.is_empty() { sb.push_str(s); }
            }
            if let Some(f) = self.getFollowStr(&lvlfData) {
              if !f.is_empty() { sb.push_str(&f); }
            }
            if !sb.is_empty() {
              propEnv.hasAutoNums = true;
              propEnv.autoNumString = Some(sb);
              // java propEnv.autoNumRunProp = runProp;
              propEnv.autoNumRunProp = Some(runProp);
            }
            propEnv.autoNumsJC = lvlfData.getJc() as i32;
          }
        }
      }
    }


    // java private void initEmptyAutoNum(ParaPropEnv propEnv, IStateEnvoy state)
    pub fn initEmptyAutoNum(&mut self, propEnv: &mut ParaPropEnv, state: &dyn IStateEnvoy) {
      // java PropertySet paraProperty = propEnv.paraProperty;
      if let Some(paraProperty) = propEnv.paraProperty.as_ref() {
      // java if (paraProperty.contains(235))
      if paraProperty.contains(235) {
        // java int lfo = paraProperty.getInt(235);
        let lfo: i32 = paraProperty.getInt(235);
        // java int lvl = paraProperty.getInt(234, 0);
        let lvl: i32 = paraProperty.getInt__int_int(234, 0);
        // java if (lvl == 12 && lfo == 0 && !paraProperty.getBoolean(223, false))
        if lvl == 12 && lfo == 0 && !paraProperty.getBoolean__int_boolean(223, false) {
          // java String strRlt = "\t";
          let strRlt: String = "\t".to_string();
          // java IChpxTable.Entry run = state.getDocument().getChpxTable().seek(propEnv.start);
          let run = state.getDocument().getChpxTable().and_then(|table|
            IChpxTable::seek(unsafe { &*table }, propEnv.start));
          // java IPropertyTool propTool = this.mDocEnv.propToolFactory.create(state.getDocument());
          if let (Some(run), Some(env)) = (run, self.mDocEnv.as_mut()) {
            let factory = env.propToolFactory.get_or_insert_with(Default::default);
            let mut prop_tool = factory.create(state.getDocument());
            // java propEnv.autoNumRunProp = propTool.getFinalRunPropert(run, paraProperty, this.mViewEnv.isRevisionFinal());
            let revision_final = self.mViewEnv.as_ref().is_some_and(|view| view.isRevisionFinal());
            propEnv.autoNumRunProp = Some(prop_tool.getFinalRunPropert__entry_propertyset_boolean(run.as_ref(), paraProperty, revision_final).clone());
          }
          // java propEnv.autoNumString = strRlt;
          propEnv.autoNumString = Some(strRlt);
          // java propEnv.hasAutoNums = true;
          propEnv.hasAutoNums = true;
        }
      }
      }
    }

    // java protected String getFollowStr(LvlfData lvlfData)
    pub fn getFollowStr(&self, lvlfData: &LvlfData) -> Option<String> {
      // java String strRlt = null;
      let mut strRlt: Option<String> = None;
      // java switch (lvlfData.getFollow())
      match lvlfData.getFollow() as i32 {
        // java case 0:
        0 => {
          // java strRlt = "\t";
          strRlt = Some("\t".to_string());
        }
        // java case 1:
        1 => {
          // java strRlt = " ";
          strRlt = Some(" ".to_string());
        }
        // java default:
        _ => {
        }
      }
      // java return strRlt;
      return strRlt;
    }

    // java private PropertySet getAutoNumStrRunProperty(ParaPropEnv propEnv, LvlfData lvlf, IStateEnvoy state)
    pub fn getAutoNumStrRunProperty(&mut self, propEnv: &ParaPropEnv, lvlf: &LvlfData, state: &dyn IStateEnvoy) -> Option<PropertySet> {
      if std::env::var("WORD_PARITY_AUTONUM_TRACE_CP").ok().and_then(|s| s.parse::<i32>().ok())
        .is_some_and(|cp| propEnv.start <= cp && cp < propEnv.end) {
        let entry = state.getDocument().getChpxTable().and_then(|table|
          IChpxTable::seek(unsafe { &*table }, propEnv.end - 1));
        eprintln!("autonum runprop chpx_entry={} chpx_prop={} factory={} para_prop={} view={} lvl_prop={}",
          entry.is_some(), entry.as_ref().is_some_and(|e|
            crate::cn::wps::moffice::writer::data::IChpxTable::Entry::getProperty(e.as_ref()).is_some()),
          self.mDocEnv.as_ref().is_some_and(|env| env.propToolFactory.is_some()),
          propEnv.paraProperty.is_some(), self.mViewEnv.is_some(), lvlf.getRunProp().is_some());
      }
      // java IChpxTable.Entry paraBreakRun = state.getDocument().getChpxTable().seek(propEnv.end - 1);
      let table = state.getDocument().getChpxTable()?;
      let paraBreakRun = IChpxTable::seek(unsafe { &*table }, propEnv.end - 1)?;
      // java IPropertyTool propTool = this.mDocEnv.propToolFactory.create(state.getDocument());
      let factory = self.mDocEnv.as_mut()?.propToolFactory.get_or_insert_with(Default::default);
      let mut propTool = factory.create(state.getDocument());
      // java MutablePropertySet prop = new MutablePropertySet();
      let mut prop = MutablePropertySet::new();
      // java propTool.getFinalRunPropert(prop, paraBreakRun.getProperty(), propEnv.paraProperty, this.mViewEnv.isRevisionFinal());
      propTool.getFinalRunPropert__mutablepropertyset_propertyset_propertyset_boolean(
        &mut prop, crate::cn::wps::moffice::writer::data::IChpxTable::Entry::getProperty(paraBreakRun.as_ref())?, propEnv.paraProperty.as_ref()?,
        self.mViewEnv.as_ref()?.isRevisionFinal());
      // java prop.remove(8);
      prop.remove(8);
      // java prop.remove(9);
      prop.remove(9);
      // java prop.add(lvlf.getRunProp());
      if let Some(run_prop) = lvlf.getRunProp() { prop.add(&run_prop); }
      // java prop.setBoolean(18, false);
      prop.setBoolean(18, false);
      // java return prop.freeze();
      Some(prop.freeze())
    }

    // java protected String getResult(AutoNumCount.AutoNum autoNum, LstLfoTool tool, LvlfData lvlfData)
    pub fn getResult(&mut self, autoNum: &AutoNum, tool: &LstLfoTool, lvlfData: &LvlfData) -> Option<String> {
      self.mResult.clear();
      format_auto_num(&mut self.mResult, autoNum, tool, lvlfData)?;
      Some(self.mResult.clone())
    }

    // java protected String getBulletChar(LvlfData lvlfData, String fontName)
    pub fn getBulletChar(&self, lvlfData: &LvlfData, _fontName: &str) -> Option<String> {
      // java String xstStr = lvlfData.getXstStr();
      let xstStr = lvlfData.getXstStr();
      // java if (null == xstStr)
      if xstStr.is_none() {
        // java return xstStr;
        return None;
      }
      xstStr
    }

    // java private ISymbolConvert getConvertFont(String string)
    pub fn getConvertFont(&self, string: &str) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != string && string.length() > 0)
      if true && (string.len() as i32) > 0 {
        // java switch (string.charAt(0))
        match string.chars().next().unwrap_or('\0') {
          // java case 'K': case 'k':
          'K' | 'k' => {
            // java if ("Kingsoft Confetti".equalsIgnoreCase(string))
            if "Kingsoft Confetti".eq_ignore_ascii_case(string) {
              // java return this.mDocEnv.layoutToolsManager.getKingsoftConfettiConvert();
              // rust: no-stmt
            }
            // java if ("Kingsoft Extra".equalsIgnoreCase(string))
            if "Kingsoft Extra".eq_ignore_ascii_case(string) {
              // java return this.mDocEnv.layoutToolsManager.getKingsoftExtraConvert();
              // rust: no-stmt
            }
            // java if ("Kingsoft Sign".equalsIgnoreCase(string))
            if "Kingsoft Sign".eq_ignore_ascii_case(string) {
              // java return this.mDocEnv.layoutToolsManager.getKingsoftSignConvert();
              // rust: no-stmt
            }
          }
          // java case 'M': case 'm':
          'M' | 'm' => {
            // java if ("MT Extra".equalsIgnoreCase(string))
            if "MT Extra".eq_ignore_ascii_case(string) {
              // java return this.mDocEnv.layoutToolsManager.getExtraConvert();
              // rust: no-stmt
            }
          }
          // java case 'S': case 's':
          'S' | 's' => {
            // java if ("Symbol".equalsIgnoreCase(string) || "WPS Special 3".equalsIgnoreCase(string))
            if "Symbol".eq_ignore_ascii_case(string) || "WPS Special 3".eq_ignore_ascii_case(string) {
              // java return this.mDocEnv.layoutToolsManager.getSymbolConvert();
              // rust: no-stmt
            }
            // java if ("WPS Special 1".equalsIgnoreCase(string))
            if "WPS Special 1".eq_ignore_ascii_case(string) {
              // java return this.mDocEnv.layoutToolsManager.getWingdingsConvert();
              // rust: no-stmt
            }
            // java if ("WPS Special 2".equalsIgnoreCase(string))
            if "WPS Special 2".eq_ignore_ascii_case(string) {
              // java return this.mDocEnv.layoutToolsManager.getExtraConvert();
              // rust: no-stmt
            }
          }
          // java case 'W': case 'w':
          'W' | 'w' => {
            // java if ("Wingdings".equalsIgnoreCase(string))
            if "Wingdings".eq_ignore_ascii_case(string) {
              // java return this.mDocEnv.layoutToolsManager.getWingdingsConvert();
              // rust: no-stmt
            }
          }
          // java default:
          _ => {
          }
        }
      }
      // java return null;
      */
    }

    // java public static int getBulletPicIndex(int iPicBullet, Document document)
    pub fn getBulletPicIndex(iPicBullet: i32, document: &dyn Document) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return iPicBullet < 0 ? -1 : -1;
      return (if iPicBullet < 0 { -1 } else { -1 });
      */
    }

    // java protected char symbol2char(String fontName, char key)
    pub fn symbol2char(&self, fontName: &str, key: u16) -> u16 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return fontName.equals("Symbol") && key == 'l' ? '·' : key;
      return (if fontName.equals("Symbol") && key == 'l' { '·' } else { key });
      */
    }

}
