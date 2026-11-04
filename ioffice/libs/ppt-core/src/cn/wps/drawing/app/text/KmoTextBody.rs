// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.app.util.FieldTypeUtil
use crate::cn::wps::drawing::app::util::FieldTypeUtil::FieldTypeUtil;
// Java: import cn.wps.drawing.data.basic.IndexRange
use crate::cn::wps::drawing::data::basic::IndexRange::IndexRange;
// Java: import cn.wps.drawing.data.blip.Blip
use crate::cn::wps::drawing::data::blip::Blip::Blip;
// Java: import cn.wps.drawing.data.text.TextBody
use crate::cn::wps::drawing::data::text::TextBody::TextBody;
// Java: import cn.wps.drawing.data.text.TextBodyProperties
use crate::cn::wps::drawing::data::text::TextBodyProperties::TextBodyProperties;
// Java: import cn.wps.drawing.data.text.TextCharacterProperties
use crate::cn::wps::drawing::data::text::TextCharacterProperties::TextCharacterProperties;
// Java: import cn.wps.drawing.data.text.TextListStyle
use crate::cn::wps::drawing::data::text::TextListStyle::TextListStyle;
// Java: import cn.wps.drawing.data.text.TextParagraph
use crate::cn::wps::drawing::data::text::TextParagraph::TextParagraph;
// Java: import cn.wps.drawing.data.text.TextParagraphProperties
use crate::cn::wps::drawing::data::text::TextParagraphProperties::TextParagraphProperties;
// Java: import cn.wps.drawing.data.text.TextRun
use crate::cn::wps::drawing::data::text::TextRun::TextRun;
// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::KoBin;
// Java: import cn.wps.memor4j.KmoAtom
// use kfc::cn::wps::memor4j::KmoAtom;
// Java: import java.util.ArrayList
// LLM No Rust equivalent
// Java: import java.util.Arrays
// LLM No Rust equivalent
// Java: import java.util.Collection
// LLM No Rust equivalent
// Java: import java.util.List
// LLM No Rust equivalent

use std::ops::Deref;

// Java field: String TEXTBODY_LINE_BREAK_STR
pub(crate) const TEXTBODY_LINE_BREAK_STR: &str = "\u{000b}";
// Java field: String TEXTBODY_PARAGRAPH_BREAK_STR
pub(crate) const TEXTBODY_PARAGRAPH_BREAK_STR: &str = "\r";
// Java field: int TEXTBODY_PARAGRAPH
pub(crate) const TEXTBODY_PARAGRAPH: i32 = 0;
// Java field: int TEXTBODY_RUN
pub(crate) const TEXTBODY_RUN: i32 = 1;

// Java: public class KmoTextBody extends KmoAtom implements KmoProtocol
pub struct KmoTextBody {
    pub(crate) parent: KmoAtom,
    pub(crate) atomData: KmoTextBodyAtomData,
    // Java field: int startRun
    startRun: i32,
    // Java field: int startPara
    startPara: i32,
    // Java field: int idxRun
    idxRun: i32,
    // Java field: int idxParagraph
    idxParagraph: i32,
    // Java field: ThreadLocal<TextRun> runCacheThreadLocal
    // LLM Rust Renamed: ThreadLocal simplified to Option for single-thread context
    runCache: Option<TextRun>,
    // Java field: ThreadLocal<TextParagraph> paragraphCacheThreadLocal
    // LLM Rust Renamed: ThreadLocal simplified to Option for single-thread context
    paragraphCache: Option<TextParagraph>,
    // Java field: List<TextRun> runsCache
    runsCache: Vec<TextRun>,
    // Java field: List<TextParagraph> paragraphsCache
    paragraphsCache: Vec<TextParagraph>,
}

impl Deref for KmoTextBody {
    type Target = KmoAtom;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

// Java: public class KmoTextBody extends KmoAtom implements KmoProtocol
impl KmoTextBody {
    // Java: KmoTextBody()
    pub fn new() -> Self {
      // Java: super(new KmoTextBodyAtomData());
      KmoTextBody {
          parent: KmoAtom::new(),
          atomData: KmoTextBodyAtomData::new(),
          startRun: 0,
          startPara: 0,
          idxRun: 0,
          idxParagraph: 0,
          runCache: None,
          paragraphCache: None,
          runsCache: Vec::new(),
          paragraphsCache: Vec::new(),
      }
    }

    fn data(&self) -> &KmoTextBodyAtomData {
        &self.atomData
    }

    fn data_mut(&mut self) -> &mut KmoTextBodyAtomData {
        &mut self.atomData
    }

    pub fn paragraphList(&self) -> &TextBody {
        &self.data().txBody
    }

    pub fn beforeWrite(&mut self) {
        // KmoAtom lifecycle - notify parent
    }

    pub fn onDataChangeAfter(&mut self, _clearCache: bool) {
        // KmoAtom lifecycle - notify data changed
    }

    // Java: KoBin protocol()
    pub fn protocol(&self) -> KoBin {
      // Java: return ((KmoTextBodyAtomData)this.rawData()).txBody.save();
      self.data().txBody.save()
    }

    // Java: void updateProtocol(KoBin data)
    pub fn updateProtocol(&mut self, data: KoBin) {
      // Java: this.clearIndex();
      self.clearIndex();
      // Java: this.clearCache();
      self.clearCache();
      // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.load(data);
      self.data_mut().txBody.load(data);
    }

    // Java: void update(KoBin data)
    pub fn update(&mut self, data: KoBin) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: this.updateProtocol(data);
      self.updateProtocol(data);
    }

    // Java: KmoProtocol copy()
    pub fn copy(&self) -> KmoTextBody {
      // Java: KmoTextBody kmoTextBody = new KmoTextBody();
      let mut kmoTextBody = KmoTextBody::new();
      // Java: KmoTextBodyAtomData newData = kmoTextBody.data();
      // LLM JavaLine Aggregation: newData not used separately in Rust
      // Java: KmoTextBodyAtomData oldData = this.data();
      // LLM JavaLine Aggregation: oldData not used separately in Rust
      // Java: kmoTextBody.updateProtocol(this.data().txBody.save());
      let saved = self.data().txBody.save();
      kmoTextBody.updateProtocol(saved);
      // Java: kmoTextBody.startRun = this.startRun;
      kmoTextBody.startRun = self.startRun;
      // Java: kmoTextBody.startPara = this.startPara;
      kmoTextBody.startPara = self.startPara;
      // Java: kmoTextBody.idxParagraph = this.idxParagraph;
      kmoTextBody.idxParagraph = self.idxParagraph;
      // Java: kmoTextBody.idxRun = this.idxRun;
      kmoTextBody.idxRun = self.idxRun;
      // Java: kmoTextBody.paragraphsCache = new ArrayList();
      kmoTextBody.paragraphsCache = Vec::new();
      // Java: this.paragraphList().read(kmoTextBody.paragraphsCache);
      self.paragraphList().read(&mut kmoTextBody.paragraphsCache);
      // Java: return kmoTextBody;
      kmoTextBody
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.clearIndex();
      self.clearIndex();
      // Java: this.clearCache();
      self.clearCache();
    }

    // Java: void clearData()
    pub fn clearData(&mut self) {
      // Java: this.clearIndex();
      self.clearIndex();
      // Java: this.clearCache();
      self.clearCache();
      // Java: this.data().txBody.close();
      self.data_mut().txBody.close();
    }

    // Java: boolean hasListStyle()
    pub fn hasListStyle(&self) -> bool {
      // Java: return ((KmoTextBodyAtomData)this.rawData()).txBody.hasListStyle();
      self.data().txBody.hasListStyle()
    }

    // Java: TextListStyle textListStyle()
    pub fn textListStyle(&self) -> TextListStyle {
      // Java: return ((KmoTextBodyAtomData)this.rawData()).txBody.listStyle();
      self.data().txBody.listStyle()
    }

    // Java: boolean hasTextBodyProperties()
    pub fn hasTextBodyProperties(&self) -> bool {
      // Java: return ((KmoTextBodyAtomData)this.rawData()).txBody.hasBodyPr();
      self.data().txBody.hasBodyPr()
    }

    // Java: TextBodyProperties textBodyProperties()
    pub fn textBodyProperties(&self) -> TextBodyProperties {
      // Java: return ((KmoTextBodyAtomData)this.rawData()).txBody.bodyPr();
      self.data().txBody.bodyPr()
    }

    // Java: boolean hasParagraphList()
    pub fn hasParagraphList(&self) -> bool {
      // Java: return ((KmoTextBodyAtomData)this.rawData()).txBody.hasParaLst();
      self.data().txBody.hasParaLst()
    }

    // Java: boolean hasAutoFitType()
    pub fn hasAutoFitType(&self) -> bool {
      // Java: return ((KmoTextBodyAtomData)this.rawData()).txBody.bodyPr().textAutofit().hasType();
      self.data().txBody.bodyPr().textAutofit().hasType()
    }

    // Java: void setAutoFit(int type)
    pub fn setAutoFit(&mut self, r#type: i32) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.bodyPr().textAutofit().putType(type);
      self.data_mut().txBody.bodyPr().textAutofit().putType(r#type);
      // Java: this.onDataChangeAfter(true);
      self.onDataChangeAfter(true);
    }

    // Java: TextBody textBody()
    pub fn textBody(&self) -> &TextBody {
      // Java: return ((KmoTextBodyAtomData)this.rawData()).txBody;
      &self.data().txBody
    }

    // Java: void replaceParagraphList(TextBody.ParagraphList paragraphList)
    pub fn replaceParagraphList(&mut self, paragraphList: TextBody) {
      // Java: this.beforeWrite();
      self.beforeWrite();
      // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.forceWriteParaLst(paragraphList);
      self.data_mut().txBody.forceWriteParaLst(paragraphList);
      // Java: this.onDataChangeAfter(true);
      self.onDataChangeAfter(true);
    }

    // Java: boolean hasText()
    pub fn hasText(&self) -> bool {
      // Java: boolean hasText = false;
      let mut hasText = false;
      // Java: List<TextParagraph> paragraphs = this.paragraphsCache;
      let paragraphs;
      // Java: if (0 == paragraphs.size())
      if 0 == self.paragraphsCache.len() {
         // Java: paragraphs = new ArrayList();
         let mut temp_paragraphs: Vec<TextParagraph> = Vec::new();
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(paragraphs);
         self.data().txBody.paraLst().read(&mut temp_paragraphs);
         paragraphs = temp_paragraphs;
      } else {
         paragraphs = self.paragraphsCache.clone();
      }
      // Java: if (1 < paragraphs.size())
      if 1 < paragraphs.len() {
         // Java: hasText = true;
         hasText = true;
      }
      // Java: else if (1 == paragraphs.size())
      else if 1 == paragraphs.len() {
         // Java: TextParagraph paragraph = (TextParagraph)paragraphs.get(0);
         let paragraph = &paragraphs[0];
         // Java: if (paragraph.hasTextruns())
         if paragraph.hasTextruns() {
            // Java: List<TextRun> runs = new ArrayList();
            let mut runs: Vec<TextRun> = Vec::new();
            // Java: paragraph.textruns().read(runs);
            paragraph.textruns().read(&mut runs);
            // Java: for(int i = 0; i < runs.size(); ++i)
            for i in 0..runs.len() {
               // Java: if (0 < ((TextRun)runs.get(i)).length())
               if 0 < runs[i].length() {
                  // Java: hasText = true;
                  hasText = true;
                  // Java: break;
                  break;
               }
            }
         }
      }
      // Java: return hasText;
      hasText
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
      // Java: int length = 0;
      let mut length: i32 = 0;
      // Java: List<TextParagraph> paragraphs = null;
      // LLM JavaLine Aggregation: merged with var6 below
      // Java: Object var6;
      let var6: Vec<TextParagraph>;
      // Java: if (0 < this.paragraphsCache.size())
      if 0 < self.paragraphsCache.len() {
         // Java: var6 = this.paragraphsCache;
         var6 = self.paragraphsCache.clone();
      }
      // Java: else
      else {
         // Java: var6 = new ArrayList();
         let mut temp: Vec<TextParagraph> = Vec::new();
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read((Collection)var6);
         self.data().txBody.paraLst().read(&mut temp);
         var6 = temp;
      }
      // Java: List<TextRun> runLst = new ArrayList();
      let mut runLst: Vec<TextRun> = Vec::new();
      // Java: for(int i = 0; i < ((List)var6).size(); ++i)
      for i in 0..var6.len() {
         // Java: runLst.clear();
         runLst.clear();
         // Java: ((TextParagraph)((List)var6).get(i)).textruns().read(runLst);
         var6[i].textruns().read(&mut runLst);
         // Java: for(int j = 0; j < runLst.size(); ++j)
         for j in 0..runLst.len() {
            // Java: length += ((TextRun)runLst.get(j)).length();
            length += runLst[j].length();
         }
         // Java: ++length;
         length += 1;
      }
      // Java: return length;
      length
    }

    // Java: int cp(int type)
    pub fn cp(&self, r#type: i32) -> i32 {
      // Java: return 1 == type ? this.startRun : this.startPara;
      if 1 == r#type { self.startRun } else { self.startPara }
    }

    // Java: boolean seek(int cp, boolean isExpandEnd)
    pub fn seek(&mut self, cp: i32, isExpandEnd: bool) -> bool {
      // Java: return this.__seek(cp, isExpandEnd);
      self.__seek(cp, isExpandEnd)
    }

    // Java: boolean __seek(int cp, boolean isExpandEnd)
    pub fn __seek(&mut self, cp: i32, isExpandEnd: bool) -> bool {
      // Java: if (this.__seekInCache(cp, isExpandEnd))
      if self.__seekInCache(cp, isExpandEnd) {
         // Java: return true;
         return true;
      }
      // Java: else
      else {
         // Java: this.clearIndex();
         self.clearIndex();
         // Java: this.clearRunCache();
         self.clearRunCache();
         // Java: this.setParagraphCache((TextParagraph)null);
         self.setParagraphCache(None);
         // Java: if (0 == cp)
         if 0 == cp {
            // Java: this.meansureCache();
            self.meansureCache();
            // Java: return true;
            return true;
         }
         // Java: else
         else {
            // Java: if (0 == this.paragraphsCache.size())
            if 0 == self.paragraphsCache.len() {
               // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(this.paragraphsCache);
               self.data().txBody.paraLst().read(&mut self.paragraphsCache);
            }
            // Java: for(this.idxParagraph = 0; this.idxParagraph < this.paragraphsCache.size(); ++this.idxParagraph)
            self.idxParagraph = 0;
            while self.idxParagraph < self.paragraphsCache.len() as i32 {
               // Java: this.clearRunCache();
               self.clearRunCache();
               // Java: this.setParagraphCache((TextParagraph)this.paragraphsCache.get(this.idxParagraph));
               let p = self.paragraphsCache[self.idxParagraph as usize].clone();
               self.setParagraphCache(Some(p));
               // Java: if (this.__seekInCache(cp, isExpandEnd))
               if self.__seekInCache(cp, isExpandEnd) {
                  // Java: return true;
                  return true;
               }
               // Java: this.startPara = ++this.startRun;
               self.startRun += 1;
               self.startPara = self.startRun;
               self.idxParagraph += 1;
            }
            // Java: this.clearRunCache();
            self.clearRunCache();
            // Java: this.setParagraphCache((TextParagraph)null);
            self.setParagraphCache(None);
            // Java: return false;
            return false;
         }
      }
    }

    // Java: boolean __seekInCache(int cp, boolean isExpandEnd)
    pub fn __seekInCache(&mut self, cp: i32, isExpandEnd: bool) -> bool {
      // Java: TextParagraph paragraphCache = this.getParagraphCache();
      let paragraphCache = self.getParagraphCache();
      // Java: if (null == paragraphCache)
      if paragraphCache.is_none() {
         // Java: return false;
         return false;
      }
      // Java: else if (cp < this.startPara)
      else if cp < self.startPara {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: TextRun runCache = this.getRunCache();
         let mut runCache = self.getRunCache();
         // Java: if (null != runCache)
         if runCache.is_some() {
            // Java: int endRun = this.startRun + runCache.length();
            let endRun = self.startRun + runCache.as_ref().unwrap().length();
            // Java: if (this.startRun < cp && cp < endRun)
            if self.startRun < cp && cp < endRun {
               // Java: return true;
               return true;
            }
            // Java: if (isExpandEnd && cp == endRun)
            if isExpandEnd && cp == endRun {
               // Java: return true;
               return true;
            }
         }
         // Java: if (0 == this.runsCache.size())
         if 0 == self.runsCache.len() {
            // Java: paragraphCache.textruns().read(this.runsCache);
            let pc = self.paragraphCache.as_ref().unwrap().clone();
            pc.textruns().read(&mut self.runsCache);
            // Java: this.setRunCache((TextRun)null);
            self.setRunCache(None);
         }
         // Java: this.startRun = this.startPara;
         self.startRun = self.startPara;
         // Java: for(this.idxRun = 0; this.idxRun < this.runsCache.size(); ++this.idxRun)
         self.idxRun = 0;
         while self.idxRun < self.runsCache.len() as i32 {
            // Java: runCache = (TextRun)this.runsCache.get(this.idxRun);
            runCache = Some(self.runsCache[self.idxRun as usize].clone());
            // Java: this.setRunCache(runCache);
            self.setRunCache(runCache.clone());
            // Java: int endRun = this.startRun + runCache.length();
            let endRun = self.startRun + runCache.as_ref().unwrap().length();
            // Java: if (isExpandEnd)
            if isExpandEnd {
               // Java: if (cp <= endRun)
               if cp <= endRun {
                  // Java: return true;
                  return true;
               }
            }
            // Java: else if (cp < endRun)
            else if cp < endRun {
               // Java: return true;
               return true;
            }
            // Java: this.startRun = endRun;
            self.startRun = endRun;
            self.idxRun += 1;
         }
         // Java: this.setRunCache((TextRun)null);
         self.setRunCache(None);
         // Java: if (cp < this.startRun + 1)
         if cp < self.startRun + 1 {
            // Java: return true;
            return true;
         }
         // Java: else
         else {
            // Java: return false;
            return false;
         }
      }
    }

    // Java: boolean prev(int type, boolean isParagraphLimited)
    pub fn prev(&mut self, r#type: i32, isParagraphLimited: bool) -> bool {
      // Java: return 0 == type ? this.prevParagraph() : this.prevRun(isParagraphLimited);
      if 0 == r#type { self.prevParagraph() } else { self.prevRun(isParagraphLimited) }
    }

    // Java: boolean prevRun(boolean isParagraphLimited)
    pub fn prevRun(&mut self, _isParagraphLimited: bool) -> bool {
      // Java: return false;
      false
    }

    // Java: boolean prevParagraph()
    pub fn prevParagraph(&mut self) -> bool {
      // Java: if (0 == this.paragraphsCache.size())
      if 0 == self.paragraphsCache.len() {
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(this.paragraphsCache);
         self.data().txBody.paraLst().read(&mut self.paragraphsCache);
         // Java: this.setParagraphCache((TextParagraph)null);
         self.setParagraphCache(None);
         // Java: this.clearRunCache();
         self.clearRunCache();
      }
      // Java: TextParagraph paragraphCache = this.getParagraphCache();
      let paragraphCache = self.getParagraphCache();
      // Java: if (0 < this.idxParagraph && this.idxParagraph < this.paragraphsCache.size())
      if 0 < self.idxParagraph && (self.idxParagraph as usize) < self.paragraphsCache.len() {
         // Java: this.clearRunCache();
         self.clearRunCache();
         // Java: --this.idxParagraph;
         self.idxParagraph -= 1;
         // Java: paragraphCache = (TextParagraph)this.paragraphsCache.get(this.idxParagraph);
         let pc = self.paragraphsCache[self.idxParagraph as usize].clone();
         // Java: this.setParagraphCache(paragraphCache);
         self.setParagraphCache(Some(pc.clone()));
         // Java: paragraphCache.textruns().read(this.runsCache);
         pc.textruns().read(&mut self.runsCache);
         // Java: if (0 < this.runsCache.size())
         if 0 < self.runsCache.len() {
            // Java: this.setRunCache((TextRun)this.runsCache.get(0));
            let r = self.runsCache[0].clone();
            self.setRunCache(Some(r));
         }
         // Java: for(int i = 0; i < this.runsCache.size(); ++i)
         for i in 0..self.runsCache.len() {
            // Java: this.startPara -= ((TextRun)this.runsCache.get(i)).length();
            self.startPara -= self.runsCache[i].length();
         }
         // Java: this.startRun = --this.startPara;
         self.startPara -= 1;
         self.startRun = self.startPara;
         // Java: return true;
         return true;
      }
      // Java: else if (null == paragraphCache && 0 < this.paragraphsCache.size())
      else if paragraphCache.is_none() && 0 < self.paragraphsCache.len() {
         // Java: this.clearRunCache();
         self.clearRunCache();
         // Java: paragraphCache = (TextParagraph)this.paragraphsCache.get(0);
         let pc = self.paragraphsCache[0].clone();
         // Java: this.setParagraphCache(paragraphCache);
         self.setParagraphCache(Some(pc.clone()));
         // Java: paragraphCache.textruns().read(this.runsCache);
         pc.textruns().read(&mut self.runsCache);
         // Java: if (0 < this.runsCache.size())
         if 0 < self.runsCache.len() {
            // Java: this.setRunCache((TextRun)this.runsCache.get(0));
            let r = self.runsCache[0].clone();
            self.setRunCache(Some(r));
         }
         // Java: this.startRun = this.startPara;
         self.startRun = self.startPara;
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: return false;
         return false;
      }
    }

    // Java: boolean next(int type, boolean isParagraphLimited)
    pub fn next(&mut self, r#type: i32, isParagraphLimited: bool) -> bool {
      // Java: return this.__next(type, isParagraphLimited);
      self.__next(r#type, isParagraphLimited)
    }

    // Java: boolean __next(int type, boolean isParagraphLimited)
    pub fn __next(&mut self, r#type: i32, isParagraphLimited: bool) -> bool {
      // Java: return 0 == type ? this.nextParagraph() : this.nextRun(isParagraphLimited);
      if 0 == r#type { self.nextParagraph() } else { self.nextRun(isParagraphLimited) }
    }

    // Java: boolean nextRun(boolean isParagraphLimited)
    pub fn nextRun(&mut self, isParagraphLimited: bool) -> bool {
      // Java: TextRun runCache = this.getRunCache();
      let mut runCache = self.getRunCache();
      // Java: if (null == runCache)
      if runCache.is_none() {
         // Java: if (0 == this.paragraphsCache.size())
         if 0 == self.paragraphsCache.len() {
            // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(this.paragraphsCache);
            self.data().txBody.paraLst().read(&mut self.paragraphsCache);
            // Java: this.setParagraphCache((TextParagraph)null);
            self.setParagraphCache(None);
            // Java: this.clearRunCache();
            self.clearRunCache();
         }
         // Java: TextParagraph paragraphCache = this.getParagraphCache();
         let mut paragraphCache = self.getParagraphCache();
         // Java: if (null == paragraphCache && 0 <= this.idxParagraph && this.idxParagraph < this.paragraphsCache.size())
         if paragraphCache.is_none() && 0 <= self.idxParagraph && (self.idxParagraph as usize) < self.paragraphsCache.len() {
            // Java: paragraphCache = (TextParagraph)this.paragraphsCache.get(this.idxParagraph);
            paragraphCache = Some(self.paragraphsCache[self.idxParagraph as usize].clone());
            // Java: this.setParagraphCache(paragraphCache);
            self.setParagraphCache(paragraphCache.clone());
            // Java: this.clearRunCache();
            self.clearRunCache();
         }
         // Java: if (null != paragraphCache)
         if paragraphCache.is_some() {
            // Java: if (0 == this.runsCache.size())
            if 0 == self.runsCache.len() {
               // Java: paragraphCache.textruns().read(this.runsCache);
               paragraphCache.as_ref().unwrap().textruns().read(&mut self.runsCache);
            }
            // Java: if (0 <= this.idxRun && this.idxRun < this.runsCache.size())
            if 0 <= self.idxRun && (self.idxRun as usize) < self.runsCache.len() {
               // Java: runCache = (TextRun)this.runsCache.get(this.idxRun);
               runCache = Some(self.runsCache[self.idxRun as usize].clone());
               // Java: this.setRunCache(runCache);
               self.setRunCache(runCache.clone());
            }
         }
      }
      // Java: ++this.idxRun;
      self.idxRun += 1;
      // Java: if (null != runCache)
      if runCache.is_some() {
         // Java: this.startRun += runCache.length();
         self.startRun += runCache.as_ref().unwrap().length();
      }
      // Java: if (0 <= this.idxRun && this.idxRun < this.runsCache.size())
      if 0 <= self.idxRun && (self.idxRun as usize) < self.runsCache.len() {
         // Java: this.setRunCache((TextRun)this.runsCache.get(this.idxRun));
         let r = self.runsCache[self.idxRun as usize].clone();
         self.setRunCache(Some(r));
         // Java: return true;
         return true;
      }
      // Java: else if (this.runsCache.size() == this.idxRun)
      else if self.runsCache.len() as i32 == self.idxRun {
         // Java: this.setRunCache((TextRun)null);
         self.setRunCache(None);
         // Java: return true;
         return true;
      }
      // Java: else
      else {
         // Java: return isParagraphLimited ? false : this.nextParagraph();
         if isParagraphLimited { false } else { self.nextParagraph() }
      }
    }

    // Java: boolean nextParagraph()
    pub fn nextParagraph(&mut self) -> bool {
      // Java: if (0 == this.paragraphsCache.size())
      if 0 == self.paragraphsCache.len() {
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(this.paragraphsCache);
         self.data().txBody.paraLst().read(&mut self.paragraphsCache);
         // Java: this.setParagraphCache((TextParagraph)null);
         self.setParagraphCache(None);
         // Java: this.clearRunCache();
         self.clearRunCache();
      }
      // Java: TextParagraph paragraphCache = this.getParagraphCache();
      let mut paragraphCache = self.getParagraphCache();
      // Java: if (null == paragraphCache && 0 <= this.idxParagraph && this.idxParagraph < this.paragraphsCache.size())
      if paragraphCache.is_none() && 0 <= self.idxParagraph && (self.idxParagraph as usize) < self.paragraphsCache.len() {
         // Java: paragraphCache = (TextParagraph)this.paragraphsCache.get(this.idxParagraph);
         paragraphCache = Some(self.paragraphsCache[self.idxParagraph as usize].clone());
         // Java: this.setParagraphCache(paragraphCache);
         self.setParagraphCache(paragraphCache.clone());
         // Java: this.clearRunCache();
         self.clearRunCache();
      }
      // Java: if (null != paragraphCache)
      if paragraphCache.is_some() {
         // Java: if (0 == this.runsCache.size())
         if 0 == self.runsCache.len() {
            // Java: paragraphCache.textruns().read(this.runsCache);
            paragraphCache.as_ref().unwrap().textruns().read(&mut self.runsCache);
         }
         // Java: for(int i = 0; i < this.runsCache.size(); ++i)
         for i in 0..self.runsCache.len() {
            // Java: if (this.runsCache.get(i) != null)
            if true {
               // Java: this.startPara += ((TextRun)this.runsCache.get(i)).length();
               self.startPara += self.runsCache[i].length();
            }
         }
         // Java: this.startRun = ++this.startPara;
         self.startPara += 1;
         self.startRun = self.startPara;
      }
      // Java: this.idxRun = 0;
      self.idxRun = 0;
      // Java: ++this.idxParagraph;
      self.idxParagraph += 1;
      // Java: if (this.idxParagraph < this.paragraphsCache.size())
      if (self.idxParagraph as usize) < self.paragraphsCache.len() {
         // Java: this.clearRunCache();
         self.clearRunCache();
         // Java: paragraphCache = (TextParagraph)this.paragraphsCache.get(this.idxParagraph);
         let pc = self.paragraphsCache[self.idxParagraph as usize].clone();
         // Java: this.setParagraphCache(paragraphCache);
         self.setParagraphCache(Some(pc.clone()));
         // Java: paragraphCache.textruns().read(this.runsCache);
         pc.textruns().read(&mut self.runsCache);
         // Java: if (this.idxRun < this.runsCache.size())
         if (self.idxRun as usize) < self.runsCache.len() {
            // Java: this.setRunCache((TextRun)this.runsCache.get(this.idxRun));
            let r = self.runsCache[self.idxRun as usize].clone();
            self.setRunCache(Some(r));
         }
         // Java: return true;
         return true;
      }
      // Java: else
      else {
         // Java: return false;
         return false;
      }
    }

    // Java: TextParagraph paragraph()
    pub fn paragraph(&self) -> TextParagraph {
      // Java: TextParagraph textParagraph = this.getParagraphCache();
      let textParagraph = self.getParagraphCache();
      // Java: return textParagraph == null ? TextParagraph.create() : TextParagraph.open(textParagraph.save());
      if textParagraph.is_none() { TextParagraph::create() } else { TextParagraph::open(textParagraph.unwrap().save()) }
    }

    // Java: IndexRange range(int type)
    pub fn range(&self, r#type: i32) -> IndexRange {
      // Java: return this.__range(type);
      self.__range(r#type)
    }

    // Java: IndexRange __range(int type)
    pub fn __range(&self, r#type: i32) -> IndexRange {
      // Java: int start = this.startRun;
      let mut start = self.startRun;
      // Java: if (0 == type)
      if 0 == r#type {
         // Java: start = this.startPara;
         start = self.startPara;
      }
      // Java: int length = 0;
      let mut length: i32 = 0;
      // Java: if (0 == type && 0 < this.runsCache.size())
      if 0 == r#type && 0 < self.runsCache.len() {
         // Java: for(int i = 0; i < this.runsCache.size(); ++i)
         for i in 0..self.runsCache.len() {
            // Java: length += ((TextRun)this.runsCache.get(i)).length();
            length += self.runsCache[i].length();
         }
         // Java: ++length;
         length += 1;
      }
      // Java: else if (1 == type && null != this.getRunCache())
      else if 1 == r#type && self.getRunCache().is_some() {
         // Java: length = this.getRunCache().length();
         length = self.getRunCache().unwrap().length();
      }
      // Java: else if (1 == type && null != this.getParagraphCache() && this.runsCache.size() == this.idxRun)
      else if 1 == r#type && self.getParagraphCache().is_some() && self.runsCache.len() as i32 == self.idxRun {
         // Java: length = 1;
         length = 1;
      }
      // Java: else
      else {
         // Java: List<TextRun> runLst = new ArrayList();
         let mut runLst: Vec<TextRun> = Vec::new();
         // Java: List<TextParagraph> paragraphs = new ArrayList();
         let mut paragraphs: Vec<TextParagraph> = Vec::new();
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(paragraphs);
         self.data().txBody.paraLst().read(&mut paragraphs);
         // Java: if (0 <= this.idxParagraph && this.idxParagraph < paragraphs.size())
         if 0 <= self.idxParagraph && (self.idxParagraph as usize) < paragraphs.len() {
            // Java: ((TextParagraph)paragraphs.get(this.idxParagraph)).textruns().read(runLst);
            paragraphs[self.idxParagraph as usize].textruns().read(&mut runLst);
         }
         // Java: if (0 == type)
         if 0 == r#type {
            // Java: for(int i = 0; i < runLst.size(); ++i)
            for i in 0..runLst.len() {
               // Java: length += ((TextRun)runLst.get(i)).length();
               length += runLst[i].length();
            }
            // Java: ++length;
            length += 1;
         }
         // Java: else if (runLst.size() == this.idxRun)
         else if runLst.len() as i32 == self.idxRun {
            // Java: length = 1;
            length = 1;
         }
         // Java: else if (0 <= this.idxRun && this.idxRun < runLst.size())
         else if 0 <= self.idxRun && (self.idxRun as usize) < runLst.len() {
            // Java: length = ((TextRun)runLst.get(this.idxRun)).length();
            length = runLst[self.idxRun as usize].length();
         }
      }
      // Java: IndexRange range = IndexRange.create();
      let mut range = IndexRange::create();
      // Java: range.putStart(start);
      range.putStart(start);
      // Java: range.putEnd(start + length);
      range.putEnd(start + length);
      // Java: return range;
      range
    }

    // Java: String text(int type)
    pub fn text(&self, r#type: i32) -> String {
      // Java: return this.__text(type);
      // TODO_RUST_CODE
    }

    // Java: String text(int start, int end)
    pub fn text(&self, start: i32, end: i32) -> String {
      // Java: return this.__text(start, end);
      // TODO_RUST_CODE
    }

    // Java: String __text(int type)
    pub fn __text(&self, r#type: i32) -> String {
      // Java: String textInCache = this.__textInCache(type);
      // TODO_RUST_CODE
      // Java: if (!textInCache.isEmpty())
        // TODO_RUST_CODE
      {
         // Java: return textInCache;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: StringBuilder text = new StringBuilder("");
         // TODO_RUST_CODE
         // Java: List<TextRun> runLst = new ArrayList();
         // TODO_RUST_CODE
         // Java: List<TextParagraph> paragraphs = new ArrayList();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(paragraphs);
         // TODO_RUST_CODE
         // Java: if (0 <= this.idxParagraph && this.idxParagraph < paragraphs.size())
            // TODO_RUST_CODE
         {
            // Java: ((TextParagraph)paragraphs.get(this.idxParagraph)).textruns().read(runLst);
            // TODO_RUST_CODE
         }
         // Java: if (0 == type)
            // TODO_RUST_CODE
         {
            // Java: for(int i = 0; i < runLst.size(); ++i)
                // TODO_RUST_CODE
            {
               // Java: text.append(this.getRunText((TextRun)runLst.get(i)));
               // TODO_RUST_CODE
            }
            // Java: text.append("\r");
            // TODO_RUST_CODE
         }
         // Java: else if (runLst.size() == this.idxRun)
            // TODO_RUST_CODE
         {
            // Java: text.append("\r");
            // TODO_RUST_CODE
         }
         // Java: else if (0 <= this.idxRun && this.idxRun < runLst.size())
            // TODO_RUST_CODE
         {
            // Java: text.append(this.getRunText((TextRun)runLst.get(this.idxRun)));
            // TODO_RUST_CODE
         }
         // Java: return text.toString();
         // TODO_RUST_CODE
      }
    }

    // Java: String __text(int start, int end)
    pub fn __text(&self, start: i32, end: i32) -> String {
      // Java: StringBuilder textBuilder = new StringBuilder("");
      // TODO_RUST_CODE
      // Java: int textEnd;
      // TODO_RUST_CODE
      // Java: if (this.__seek(start, false))
        // TODO_RUST_CODE
      {
         // Java: do
            // TODO_RUST_CODE
         {
            // Java: String text = this.__text(1);
            // TODO_RUST_CODE
            // Java: int textStart = this.startRun;
            // TODO_RUST_CODE
            // Java: textEnd = textStart + text.length();
            // TODO_RUST_CODE
            // Java: int subStart = Math.max(0, start - textStart);
            // TODO_RUST_CODE
            // Java: int count = Math.max(0, end - textStart - subStart);
            // TODO_RUST_CODE
            // Java: int subEnd = Math.min(subStart + count, text.length());
            // TODO_RUST_CODE
            // Java: textBuilder.append(text.substring(subStart, subEnd));
            // TODO_RUST_CODE
         }
      }
      // Java: return textBuilder.toString();
      // TODO_RUST_CODE
    }

    // Java: String __textInCache(int type)
    pub fn __textInCache(&self, r#type: i32) -> String {
      // Java: StringBuilder text = new StringBuilder("");
      // TODO_RUST_CODE
      // Java: if (0 == type && 0 < this.runsCache.size())
        // TODO_RUST_CODE
      {
         // Java: for(int i = 0; i < this.runsCache.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: text.append(this.getRunText((TextRun)this.runsCache.get(i)));
            // TODO_RUST_CODE
         }
         // Java: text.append("\r");
         // TODO_RUST_CODE
      }
      // Java: else if (1 == type && null != this.getRunCache())
        // TODO_RUST_CODE
      {
         // Java: text.append(this.getRunText(this.getRunCache()));
         // TODO_RUST_CODE
      }
      // Java: else if (1 == type && null != this.getParagraphCache() && this.runsCache.size() == this.idxRun)
        // TODO_RUST_CODE
      {
         // Java: text.append("\r");
         // TODO_RUST_CODE
      }
      // Java: return text.toString();
      // TODO_RUST_CODE
    }

    // Java: TextCharacterProperties characterProperties()
    pub fn characterProperties(&self) -> TextCharacterProperties {
      // Java: TextCharacterProperties prop = null;
      // TODO_RUST_CODE
      // Java: if (null != this.getRunCache())
        // TODO_RUST_CODE
      {
         // Java: prop = this.getCharacterProperties(this.getRunCache());
         // TODO_RUST_CODE
      }
      // Java: else if (null != this.getParagraphCache() && this.runsCache.size() == this.idxRun)
        // TODO_RUST_CODE
      {
         // Java: TextParagraph paragraphCache = this.getParagraphCache();
         // TODO_RUST_CODE
         // Java: if (paragraphCache.hasEndParaRPr())
            // TODO_RUST_CODE
         {
            // Java: prop = TextCharacterProperties.open(paragraphCache.endParaRPr().save());
            // TODO_RUST_CODE
         }
         // Java: else if (0 < this.runsCache.size())
            // TODO_RUST_CODE
         {
            // Java: prop = this.getCharacterProperties((TextRun)this.runsCache.get(this.runsCache.size() - 1));
            // TODO_RUST_CODE
         }
      }
      // Java: if (null == prop)
        // TODO_RUST_CODE
      {
         // Java: prop = this.getCharacterProperties(this.idxParagraph, this.idxRun);
         // TODO_RUST_CODE
      }
      // Java: return null == prop ? TextCharacterProperties.create() : prop;
      // TODO_RUST_CODE
    }

    // Java: List<Object> data(int startData, int endData)
    pub fn data(&self, startData: i32, endData: i32) -> List<Object> {
      // Java: List<Object> resultData = new ArrayList();
      // TODO_RUST_CODE
      // Java: List<TextParagraph> textparagraphs = null;
      // TODO_RUST_CODE
      // Java: if (0 < this.paragraphsCache.size())
        // TODO_RUST_CODE
      {
         // Java: textparagraphs = this.paragraphsCache;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: textparagraphs = new ArrayList();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(textparagraphs);
         // TODO_RUST_CODE
      }
      // Java: int start = 0;
      // TODO_RUST_CODE
      // Java: int end = 0;
      // TODO_RUST_CODE
      // Java: TextParagraph.TextRunList runListData = null;
      // TODO_RUST_CODE
      // Java: List<TextParagraph> textparagraphData = new ArrayList();
      // TODO_RUST_CODE
      // Java: for(int idxPara = 0; idxPara < textparagraphs.size(); ++idxPara)
        // TODO_RUST_CODE
      {
         // Java: List<TextRun> textrunData = new ArrayList();
         // TODO_RUST_CODE
         // Java: List<TextRun> textruns = new ArrayList();
         // TODO_RUST_CODE
         // Java: TextParagraph paragraph = (TextParagraph)textparagraphs.get(idxPara);
         // TODO_RUST_CODE
         // Java: paragraph.textruns().read(textruns);
         // TODO_RUST_CODE
         // Java: for(int idxRun = 0; idxRun < textruns.size(); ++idxRun)
            // TODO_RUST_CODE
         {
            // Java: TextRun textrun = (TextRun)textruns.get(idxRun);
            // TODO_RUST_CODE
            // Java: end = start + textrun.length();
            // TODO_RUST_CODE
            // Java: if (start < endData)
                // TODO_RUST_CODE
            {
               // Java: if (startData <= start)
                    // TODO_RUST_CODE
               {
                  // Java: if (end <= endData)
                        // TODO_RUST_CODE
                  {
                     // Java: textrunData.add(TextRun.open(textrun.save()));
                     // TODO_RUST_CODE
                  }
                  // Java: else if (start < endData)
                        // TODO_RUST_CODE
                  {
                     // Java: TextRun newTextRun = TextRun.open(textrun.save());
                     // TODO_RUST_CODE
                     // Java: String text = this.getRunText(textrun);
                     // TODO_RUST_CODE
                     // Java: this.setRunText(newTextRun, text.substring(0, endData - start));
                     // TODO_RUST_CODE
                     // Java: textrunData.add(newTextRun);
                     // TODO_RUST_CODE
                  }
               }
               // Java: else if (startData < end && end <= endData)
                    // TODO_RUST_CODE
               {
                  // Java: TextRun newTextRun = TextRun.open(textrun.save());
                  // TODO_RUST_CODE
                  // Java: String text = this.getRunText(textrun);
                  // TODO_RUST_CODE
                  // Java: this.setRunText(newTextRun, text.substring(startData - start, text.length()));
                  // TODO_RUST_CODE
                  // Java: textrunData.add(newTextRun);
                  // TODO_RUST_CODE
               }
               // Java: else if (start < startData && endData < end)
                    // TODO_RUST_CODE
               {
                  // Java: TextRun newTextRun = TextRun.open(textrun.save());
                  // TODO_RUST_CODE
                  // Java: String text = this.getRunText(textrun);
                  // TODO_RUST_CODE
                  // Java: this.setRunText(newTextRun, text.substring(startData - start, endData - start));
                  // TODO_RUST_CODE
                  // Java: textrunData.add(newTextRun);
                  // TODO_RUST_CODE
               }
            }
            // Java: start = end;
            // TODO_RUST_CODE
         }
         // Java: if (0 < textrunData.size())
            // TODO_RUST_CODE
         {
            // Java: if (end < endData)
                // TODO_RUST_CODE
            {
               // Java: TextParagraph newParagraph = TextParagraph.open(paragraph.save());
               // TODO_RUST_CODE
               // Java: newParagraph.textruns().write(textrunData);
               // TODO_RUST_CODE
               // Java: newParagraph.save();
               // TODO_RUST_CODE
               // Java: textparagraphData.add(newParagraph);
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: runListData = TextParagraph.TextRunList.create();
               // TODO_RUST_CODE
               // Java: runListData.write(textrunData);
               // TODO_RUST_CODE
               // Java: runListData.save();
               // TODO_RUST_CODE
            }
         }
         // Java: else if (startData <= start && end < endData)
            // TODO_RUST_CODE
         {
            // Java: TextParagraph newParagraph = TextParagraph.open(paragraph.save());
            // TODO_RUST_CODE
            // Java: newParagraph.save();
            // TODO_RUST_CODE
            // Java: textparagraphData.add(newParagraph);
            // TODO_RUST_CODE
         }
         // Java: ++end;
         // TODO_RUST_CODE
         // Java: start = end;
         // TODO_RUST_CODE
         // Java: if (endData <= end)
            // TODO_RUST_CODE
         {
            // Java: break;
            // TODO_RUST_CODE
         }
      }
      // Java: int count = textparagraphData.size();
      // TODO_RUST_CODE
      // Java: if (0 < count)
        // TODO_RUST_CODE
      {
         // Java: TextBody.ParagraphList paraList = TextBody.ParagraphList.create();
         // TODO_RUST_CODE
         // Java: paraList.write(textparagraphData);
         // TODO_RUST_CODE
         // Java: paraList.save();
         // TODO_RUST_CODE
         // Java: resultData.add(0, paraList);
         // TODO_RUST_CODE
      }
      // Java: if (null != runListData)
        // TODO_RUST_CODE
      {
         // Java: resultData.add(runListData);
         // TODO_RUST_CODE
      }
      // Java: return resultData;
      // TODO_RUST_CODE
    }

    // Java: KoAr format(int type)
    pub fn format(&self, r#type: i32) -> KoAr {
      // Java: KoAr prop = null;
      // TODO_RUST_CODE
      // Java: prop = this.__formatInCache(type);
      // TODO_RUST_CODE
      // Java: if (null == prop)
        // TODO_RUST_CODE
      {
         // Java: if (0 == type)
            // TODO_RUST_CODE
         {
            // Java: prop = this.getParagraphProperties(this.idxParagraph);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: prop = this.getCharacterProperties(this.idxParagraph, this.idxRun);
            // TODO_RUST_CODE
         }
      }
      // Java: return prop;
      // TODO_RUST_CODE
    }

    // Java: KoAr __formatInCache(int type)
    pub fn __formatInCache(&self, r#type: i32) -> KoAr {
      // Java: KoAr prop = null;
      // TODO_RUST_CODE
      // Java: if (0 == type)
        // TODO_RUST_CODE
      {
         // Java: TextParagraph paragraphCache = this.getParagraphCache();
         // TODO_RUST_CODE
         // Java: if (null != paragraphCache)
            // TODO_RUST_CODE
         {
            // Java: prop = TextParagraphProperties.open(paragraphCache.paraPr().save());
            // TODO_RUST_CODE
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: TextRun runCache = this.getRunCache();
         // TODO_RUST_CODE
         // Java: if (null != runCache)
            // TODO_RUST_CODE
         {
            // Java: prop = this.getCharacterProperties(runCache);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: TextParagraph paragraphCache = this.getParagraphCache();
            // TODO_RUST_CODE
            // Java: if (this.runsCache.size() == this.idxRun && null != paragraphCache)
                // TODO_RUST_CODE
            {
               // Java: if (this.__textInCache(type).equals("\r"))
                    // TODO_RUST_CODE
               {
                  // Java: if (0 < this.runsCache.size())
                        // TODO_RUST_CODE
                  {
                     // Java: prop = this.getCharacterProperties((TextRun)this.runsCache.get(this.runsCache.size() - 1));
                     // TODO_RUST_CODE
                  }
               }
               // Java: else if (paragraphCache.hasEndParaRPr())
                    // TODO_RUST_CODE
               {
                  // Java: prop = TextCharacterProperties.open(paragraphCache.endParaRPr().save());
                  // TODO_RUST_CODE
               }
               // Java: else if (0 < this.runsCache.size())
                    // TODO_RUST_CODE
               {
                  // Java: prop = this.getCharacterProperties((TextRun)this.runsCache.get(this.runsCache.size() - 1));
                  // TODO_RUST_CODE
               }
            }
         }
      }
      // Java: return prop;
      // TODO_RUST_CODE
    }

    // Java: boolean insertParagraph(int cp)
    pub fn insertParagraph(&self, cp: i32) -> bool {
      // Java: boolean seekResult = this.__seek(cp, true);
      // TODO_RUST_CODE
      // Java: List<TextParagraph> paragraphs = this.paragraphsCache;
      // TODO_RUST_CODE
      // Java: if (seekResult && paragraphs.size() > 0)
        // TODO_RUST_CODE
      {
         // Java: TextParagraph paragraph = (TextParagraph)paragraphs.get(this.idxParagraph);
         // TODO_RUST_CODE
         // Java: List<TextRun> runs = new ArrayList();
         // TODO_RUST_CODE
         // Java: List<TextRun> newruns = new ArrayList();
         // TODO_RUST_CODE
         // Java: paragraph.textruns().read(runs);
         // TODO_RUST_CODE
         // Java: TextParagraph newparagraph = TextParagraph.open(paragraph.save());
         // TODO_RUST_CODE
         // Java: newparagraph.deleteTextruns();
         // TODO_RUST_CODE
         // Java: if (this.startRun == cp)
            // TODO_RUST_CODE
         {
            // Java: int i = this.idxRun;
            // TODO_RUST_CODE
            // Java: while(i < runs.size())
                // TODO_RUST_CODE
            {
               // Java: newruns.add(TextRun.open(((TextRun)runs.get(i)).save()));
               // TODO_RUST_CODE
               // Java: runs.remove(i);
               // TODO_RUST_CODE
            }
         }
         // Java: else if (0 <= this.idxRun && this.idxRun < runs.size())
            // TODO_RUST_CODE
         {
            // Java: if (this.getRunText((TextRun)runs.get(this.idxRun)).length() + this.startRun == cp)
                // TODO_RUST_CODE
            {
               // Java: int i = this.idxRun + 1;
               // TODO_RUST_CODE
               // Java: while(i < runs.size())
                    // TODO_RUST_CODE
               {
                  // Java: newruns.add(TextRun.open(((TextRun)runs.get(i)).save()));
                  // TODO_RUST_CODE
                  // Java: runs.remove(i);
                  // TODO_RUST_CODE
               }
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: int i = this.idxRun + 1;
               // TODO_RUST_CODE
               // Java: while(i < runs.size())
                    // TODO_RUST_CODE
               {
                  // Java: newruns.add(TextRun.open(((TextRun)runs.get(i)).save()));
                  // TODO_RUST_CODE
                  // Java: runs.remove(i);
                  // TODO_RUST_CODE
               }
               // Java: TextRun run = (TextRun)runs.get(this.idxRun);
               // TODO_RUST_CODE
               // Java: TextRun newrun = TextRun.open(run.save());
               // TODO_RUST_CODE
               // Java: String runtext = this.getRunText(run);
               // TODO_RUST_CODE
               // Java: int cpInsert = Math.min(cp - this.startRun, runtext.length());
               // TODO_RUST_CODE
               // Java: this.setRunText(run, runtext.substring(0, cpInsert));
               // TODO_RUST_CODE
               // Java: this.setRunText(newrun, runtext.substring(cpInsert, runtext.length()));
               // TODO_RUST_CODE
               // Java: newruns.add(0, newrun);
               // TODO_RUST_CODE
            }
         }
         // Java: if (0 < runs.size())
            // TODO_RUST_CODE
         {
            // Java: TextRun lastRun = (TextRun)runs.get(runs.size() - 1);
            // TODO_RUST_CODE
            // Java: this.mergeTextCharacterProperties(this.getCharacterProperties(lastRun), paragraph.endParaRPr());
            // TODO_RUST_CODE
            // Java: paragraph.endParaRPr().save();
            // TODO_RUST_CODE
            // Java: paragraph.textruns().write(runs);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: paragraph.deleteTextruns();
            // TODO_RUST_CODE
            // Java: paragraph.forceWriteTextruns(TextParagraph.TextRunList.create());
            // TODO_RUST_CODE
         }
         // Java: paragraph.save();
         // TODO_RUST_CODE
         // Java: TextParagraph.TextRunList runList = TextParagraph.TextRunList.create();
         // TODO_RUST_CODE
         // Java: if (0 < newruns.size())
            // TODO_RUST_CODE
         {
            // Java: runList.write(newruns);
            // TODO_RUST_CODE
         }
         // Java: runList.save();
         // TODO_RUST_CODE
         // Java: newparagraph.writeTextruns(runList);
         // TODO_RUST_CODE
         // Java: if (!newparagraph.hasEndParaRPr())
            // TODO_RUST_CODE
         {
            // Java: newparagraph.writeEndParaRPr(this.defaultEndParaRPr());
            // TODO_RUST_CODE
         }
         // Java: newparagraph.save();
         // TODO_RUST_CODE
         // Java: paragraphs.add(this.idxParagraph + 1, newparagraph);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: TextParagraph newparagraph = TextParagraph.create();
         // TODO_RUST_CODE
         // Java: newparagraph.writeTextruns(TextParagraph.TextRunList.create());
         // TODO_RUST_CODE
         // Java: newparagraph.writeEndParaRPr(this.defaultEndParaRPr());
         // TODO_RUST_CODE
         // Java: newparagraph.save();
         // TODO_RUST_CODE
         // Java: paragraphs.add(newparagraph);
         // TODO_RUST_CODE
      }
      // Java: this.beforeWrite();
      // TODO_RUST_CODE
      // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(paragraphs);
      // TODO_RUST_CODE
      // Java: this.onDataChangeAfter(true);
      // TODO_RUST_CODE
      // Java: return true;
      // TODO_RUST_CODE
    }

    // Java: boolean insertLineBreak(int cp)
    pub fn insertLineBreak(&self, cp: i32) -> bool {
      // Java: boolean result = false;
      // TODO_RUST_CODE
      // Java: boolean seekResult = this.__seek(cp, true);
      // TODO_RUST_CODE
      // Java: if (seekResult)
        // TODO_RUST_CODE
      {
         // Java: TextRun newrun = TextRun.create();
         // TODO_RUST_CODE
         // Java: TextRun.Linebreak linebreakRun = TextRun.Linebreak.create();
         // TODO_RUST_CODE
         // Java: linebreakRun.writeRPr(TextCharacterProperties.open(((TextCharacterProperties)this.format(1)).save()));
         // TODO_RUST_CODE
         // Java: linebreakRun.save();
         // TODO_RUST_CODE
         // Java: newrun.putMode(1);
         // TODO_RUST_CODE
         // Java: newrun.writeLinebreakRun(linebreakRun);
         // TODO_RUST_CODE
         // Java: newrun.putLength(1);
         // TODO_RUST_CODE
         // Java: newrun.save();
         // TODO_RUST_CODE
         // Java: if (0 == this.paragraphsCache.size())
            // TODO_RUST_CODE
         {
            // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(this.paragraphsCache);
            // TODO_RUST_CODE
         }
         // Java: List<TextParagraph> paragraphs = this.paragraphsCache;
         // TODO_RUST_CODE
         // Java: TextParagraph paragraph = (TextParagraph)paragraphs.get(this.idxParagraph);
         // TODO_RUST_CODE
         // Java: List<TextRun> runs = new ArrayList();
         // TODO_RUST_CODE
         // Java: paragraph.textruns().read(runs);
         // TODO_RUST_CODE
         // Java: if (this.startRun == cp)
            // TODO_RUST_CODE
         {
            // Java: runs.add(this.idxRun, newrun);
            // TODO_RUST_CODE
         }
         // Java: else if (0 <= this.idxRun && this.idxRun < runs.size())
            // TODO_RUST_CODE
         {
            // Java: if (this.getRunText((TextRun)runs.get(this.idxRun)).length() + this.startRun == cp)
                // TODO_RUST_CODE
            {
               // Java: runs.add(this.idxRun + 1, newrun);
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: TextRun run = (TextRun)runs.get(this.idxRun);
               // TODO_RUST_CODE
               // Java: TextRun runAfter = TextRun.open(run.save());
               // TODO_RUST_CODE
               // Java: String runtext = this.getRunText(run);
               // TODO_RUST_CODE
               // Java: int cpInsert = Math.min(cp - this.startRun, runtext.length());
               // TODO_RUST_CODE
               // Java: this.setRunText(run, runtext.substring(0, cpInsert));
               // TODO_RUST_CODE
               // Java: this.setRunText(runAfter, runtext.substring(cpInsert, runtext.length()));
               // TODO_RUST_CODE
               // Java: runs.add(this.idxRun + 1, newrun);
               // TODO_RUST_CODE
               // Java: runs.add(this.idxRun + 2, runAfter);
               // TODO_RUST_CODE
            }
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: runs.add(newrun);
            // TODO_RUST_CODE
         }
         // Java: paragraph.textruns().write(runs);
         // TODO_RUST_CODE
         // Java: if (!paragraph.hasEndParaRPr())
            // TODO_RUST_CODE
         {
            // Java: paragraph.writeEndParaRPr(this.defaultEndParaRPr());
            // TODO_RUST_CODE
         }
         // Java: paragraph.save();
         // TODO_RUST_CODE
         // Java: this.beforeWrite();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(paragraphs);
         // TODO_RUST_CODE
         // Java: this.onDataChangeAfter(true);
         // TODO_RUST_CODE
         // Java: result = true;
         // TODO_RUST_CODE
      }
      // Java: return result;
      // TODO_RUST_CODE
    }

    // Java: boolean insert(int cp, String text)
    pub fn insert(&self, cp: i32, text: String) -> bool {
      // Java: return null != text && text.length() != 0 ? this.__insert(cp, text) : false;
      // TODO_RUST_CODE
    }

    // Java: boolean insert(int cp, List<Object> dataList, boolean bCovertToRunList)
    pub fn insert(&self, cp: i32, dataList: List<Object>, bCovertToRunList: bool) -> bool {
      // Java: return this.__insert(cp, dataList, bCovertToRunList);
      // TODO_RUST_CODE
    }

    // Java: boolean replaceTextContentByPara(String text)
    pub fn replaceTextContentByPara(&self, text: String) -> bool {
      // Java: if (text != null && text.length() != 0)
        // TODO_RUST_CODE
      {
         // Java: if (!this.hasParagraphList())
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: this.seek(0, false);
            // TODO_RUST_CODE
            // Java: String[] textPara = text.split("\r");
            // TODO_RUST_CODE
            // Java: TextParagraph lastParagraphCache = (TextParagraph)this.paragraphsCache.get(0);
            // TODO_RUST_CODE
            // Java: TextCharacterProperties lastRProp = null;
            // TODO_RUST_CODE
            // Java: TextParagraphProperties lastPProp = lastParagraphCache.paraPr();
            // TODO_RUST_CODE
            // Java: for(int idxParagraph = 0; idxParagraph < this.paragraphsCache.size(); ++idxParagraph)
                // TODO_RUST_CODE
            {
               // Java: lastRProp = null;
               // TODO_RUST_CODE
               // Java: this.clearRunCache();
               // TODO_RUST_CODE
               // Java: lastParagraphCache = (TextParagraph)this.paragraphsCache.get(idxParagraph);
               // TODO_RUST_CODE
               // Java: if (lastParagraphCache.hasTextruns())
                    // TODO_RUST_CODE
               {
                  // Java: TextParagraph.TextRunList textRunList = lastParagraphCache.textruns();
                  // TODO_RUST_CODE
                  // Java: ArrayList<TextRun> textRuns = new ArrayList();
                  // TODO_RUST_CODE
                  // Java: textRunList.read(textRuns);
                  // TODO_RUST_CODE
                  // Java: if (((TextRun)textRuns.get(0)).hasRegularRun() && ((TextRun)textRuns.get(0)).regularRun().hasRPr())
                        // TODO_RUST_CODE
                  {
                     // Java: lastRProp = ((TextRun)textRuns.get(0)).regularRun().rPr();
                     // TODO_RUST_CODE
                  }
               }
               // Java: if (lastRProp == null)
                    // TODO_RUST_CODE
               {
                  // Java: if (lastParagraphCache.hasEndParaRPr())
                        // TODO_RUST_CODE
                  {
                     // Java: lastRProp = lastParagraphCache.endParaRPr();
                     // TODO_RUST_CODE
                  }
                  // Java: else
                        // TODO_RUST_CODE
                  {
                     // Java: lastRProp = this.defaultEndParaRPr();
                     // TODO_RUST_CODE
                  }
               }
               // Java: lastParagraphCache.deleteTextruns();
               // TODO_RUST_CODE
               // Java: List<TextRun> newRuns = new ArrayList();
               // TODO_RUST_CODE
               // Java: TextRun newrun = TextRun.create();
               // TODO_RUST_CODE
               // Java: if (textPara.length > idxParagraph)
                    // TODO_RUST_CODE
               {
                  // Java: this.setRunText(newrun, textPara[idxParagraph]);
                  // TODO_RUST_CODE
               }
               // Java: this.setTextCharacterProperties(lastRProp, newrun);
               // TODO_RUST_CODE
               // Java: newRuns.add(newrun);
               // TODO_RUST_CODE
               // Java: lastParagraphCache.textruns().write(newRuns);
               // TODO_RUST_CODE
               // Java: lastParagraphCache.save();
               // TODO_RUST_CODE
            }
            // Java: if (textPara.length > this.paragraphsCache.size())
                // TODO_RUST_CODE
            {
               // Java: int paragraphsSize = this.paragraphsCache.size();
               // TODO_RUST_CODE
               // Java: int paraToAdd = textPara.length - paragraphsSize;
               // TODO_RUST_CODE
               // Java: for(int i = 0; i < paraToAdd; ++i)
                    // TODO_RUST_CODE
               {
                  // Java: TextParagraph newparagraph = TextParagraph.create();
                  // TODO_RUST_CODE
                  // Java: newparagraph.forceWriteEndParaRPr(lastRProp);
                  // TODO_RUST_CODE
                  // Java: newparagraph.forceWriteParaPr(TextParagraphProperties.open(lastPProp.save()));
                  // TODO_RUST_CODE
                  // Java: List<TextRun> newRuns = new ArrayList();
                  // TODO_RUST_CODE
                  // Java: TextRun newrun = TextRun.create();
                  // TODO_RUST_CODE
                  // Java: this.setRunText(newrun, textPara[paragraphsSize + i]);
                  // TODO_RUST_CODE
                  // Java: this.setTextCharacterProperties(lastRProp, newrun);
                  // TODO_RUST_CODE
                  // Java: newRuns.add(newrun);
                  // TODO_RUST_CODE
                  // Java: newparagraph.textruns().write(newRuns);
                  // TODO_RUST_CODE
                  // Java: newparagraph.save();
                  // TODO_RUST_CODE
                  // Java: this.paragraphsCache.add(newparagraph);
                  // TODO_RUST_CODE
               }
            }
            // Java: this.beforeWrite();
            // TODO_RUST_CODE
            // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(this.paragraphsCache);
            // TODO_RUST_CODE
            // Java: this.onDataChangeAfter(true);
            // TODO_RUST_CODE
            // Java: return true;
            // TODO_RUST_CODE
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean __insert(int cp, String text)
    pub fn __insert(&self, cp: i32, text: String) -> bool {
      // Java: if (!this.__seek(cp, true))
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: List<TextRun> runs = this.runsCache;
         // TODO_RUST_CODE
         // Java: TextParagraph paragraph = this.getParagraphCache();
         // TODO_RUST_CODE
         // Java: List<TextParagraph> paragraphs = this.paragraphsCache;
         // TODO_RUST_CODE
         // Java: boolean isClearCache = true;
         // TODO_RUST_CODE
         // Java: TextRun runCache = this.getRunCache();
         // TODO_RUST_CODE
         // Java: if (null != runCache)
            // TODO_RUST_CODE
         {
            // Java: if (1 != runCache.mode())
                // TODO_RUST_CODE
            {
               // Java: int cpInsert = cp - this.startRun;
               // TODO_RUST_CODE
               // Java: StringBuilder textBuilder = new StringBuilder("");
               // TODO_RUST_CODE
               // Java: textBuilder.append(this.getRunText(runCache));
               // TODO_RUST_CODE
               // Java: textBuilder.insert(cpInsert, text);
               // TODO_RUST_CODE
               // Java: this.setRunText(runCache, textBuilder.toString());
               // TODO_RUST_CODE
               // Java: isClearCache = false;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: TextRun newrun = TextRun.create();
               // TODO_RUST_CODE
               // Java: newrun.putMode(0);
               // TODO_RUST_CODE
               // Java: this.setRunText(newrun, text);
               // TODO_RUST_CODE
               // Java: this.__formatTextCharacterProperties(this.getCharacterProperties(runCache), newrun, 0);
               // TODO_RUST_CODE
               // Java: newrun.save();
               // TODO_RUST_CODE
               // Java: if (this.startRun == cp)
                    // TODO_RUST_CODE
               {
                  // Java: runs.add(this.idxRun, newrun);
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: runs.add(this.idxRun + 1, newrun);
                  // TODO_RUST_CODE
               }
            }
            // Java: paragraph.textruns().write(this.runsCache);
            // TODO_RUST_CODE
            // Java: if (!paragraph.hasEndParaRPr())
                // TODO_RUST_CODE
            {
               // Java: paragraph.writeEndParaRPr(this.defaultEndParaRPr());
               // TODO_RUST_CODE
            }
            // Java: paragraph.save();
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: if (0 != runs.size())
                // TODO_RUST_CODE
            {
               // Java: return false;
               // TODO_RUST_CODE
            }
            // Java: if (null == paragraph)
                // TODO_RUST_CODE
            {
               // Java: paragraph = TextParagraph.create();
               // TODO_RUST_CODE
               // Java: paragraph.writeEndParaRPr(this.defaultEndParaRPr());
               // TODO_RUST_CODE
               // Java: paragraphs.add(this.idxParagraph, paragraph);
               // TODO_RUST_CODE
            }
            // Java: TextRun newrun = TextRun.create();
            // TODO_RUST_CODE
            // Java: newrun.putMode(0);
            // TODO_RUST_CODE
            // Java: this.setRunText(newrun, text);
            // TODO_RUST_CODE
            // Java: if (paragraph.hasEndParaRPr())
                // TODO_RUST_CODE
            {
               // Java: this.__formatTextCharacterProperties(paragraph.endParaRPr(), newrun, 0);
               // TODO_RUST_CODE
            }
            // Java: newrun.save();
            // TODO_RUST_CODE
            // Java: runs.add(newrun);
            // TODO_RUST_CODE
            // Java: TextParagraph.TextRunList runList = TextParagraph.TextRunList.create();
            // TODO_RUST_CODE
            // Java: runList.write(runs);
            // TODO_RUST_CODE
            // Java: runList.save();
            // TODO_RUST_CODE
            // Java: paragraph.writeTextruns(runList);
            // TODO_RUST_CODE
            // Java: if (!paragraph.hasEndParaRPr())
                // TODO_RUST_CODE
            {
               // Java: paragraph.writeEndParaRPr(this.defaultEndParaRPr());
               // TODO_RUST_CODE
            }
            // Java: paragraph.save();
            // TODO_RUST_CODE
         }
         // Java: this.beforeWrite();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(paragraphs);
         // TODO_RUST_CODE
         // Java: this.onDataChangeAfter(isClearCache);
         // TODO_RUST_CODE
         // Java: return true;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean __insert(int cp, List<Object> dataList, boolean bCovertToRunList)
    pub fn __insert(&self, cp: i32, dataList: List<Object>, bCovertToRunList: bool) -> bool {
      // Java: int size = dataList.size();
      // TODO_RUST_CODE
      // Java: if (size <= 0)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: boolean result = false;
         // TODO_RUST_CODE
         // Java: Object data = dataList.get(size - 1);
         // TODO_RUST_CODE
         // Java: if (data instanceof TextParagraph.TextRunList)
            // TODO_RUST_CODE
         {
            // Java: result = this.insert(cp, (TextParagraph.TextRunList)data);
            // TODO_RUST_CODE
            // Java: --size;
            // TODO_RUST_CODE
         }
         // Java: if (0 < size)
            // TODO_RUST_CODE
         {
            // Java: data = dataList.get(size - 1);
            // TODO_RUST_CODE
            // Java: if (data instanceof KmoTextBody)
                // TODO_RUST_CODE
            {
               // Java: if (1 == size)
                    // TODO_RUST_CODE
               {
                  // Java: KmoTextBody textBody = (KmoTextBody)dataList.get(size - 1);
                  // TODO_RUST_CODE
                  // Java: data = TextBody.ParagraphList.open(textBody.paragraphList().save());
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: TextBody.ParagraphList textparagraphList = TextBody.ParagraphList.create();
                  // TODO_RUST_CODE
                  // Java: List<TextParagraph> textParagraphs = new ArrayList();
                  // TODO_RUST_CODE
                  // Java: for(int i = 0; i < dataList.size(); ++i)
                        // TODO_RUST_CODE
                  {
                     // Java: KmoTextBody textBody = (KmoTextBody)dataList.get(i);
                     // TODO_RUST_CODE
                     // Java: List<TextParagraph> tmp = new ArrayList();
                     // TODO_RUST_CODE
                     // Java: textBody.paragraphList().read(tmp);
                     // TODO_RUST_CODE
                     // Java: for(int idx = 0; idx < tmp.size(); ++idx)
                            // TODO_RUST_CODE
                     {
                        // Java: TextParagraph paragraph = (TextParagraph)tmp.get(idx);
                        // TODO_RUST_CODE
                        // Java: String text = this.getParagraphText(paragraph);
                        // TODO_RUST_CODE
                        // Java: if (1 < text.length())
                                // TODO_RUST_CODE
                        {
                           // Java: TextRun.RegularTextRun regularRun = TextRun.RegularTextRun.create();
                           // TODO_RUST_CODE
                           // Java: regularRun.putText(text.substring(0, text.length() - 1));
                           // TODO_RUST_CODE
                           // Java: regularRun.save();
                           // TODO_RUST_CODE
                           // Java: TextRun run = TextRun.create();
                           // TODO_RUST_CODE
                           // Java: run.putMode(0);
                           // TODO_RUST_CODE
                           // Java: run.writeRegularRun(regularRun);
                           // TODO_RUST_CODE
                           // Java: run.putLength(text.length() - 1);
                           // TODO_RUST_CODE
                           // Java: run.save();
                           // TODO_RUST_CODE
                           // Java: List<TextRun> textruns = new ArrayList();
                           // TODO_RUST_CODE
                           // Java: textruns.add(run);
                           // TODO_RUST_CODE
                           // Java: TextParagraph.TextRunList textRunList = TextParagraph.TextRunList.create();
                           // TODO_RUST_CODE
                           // Java: textRunList.write(textruns);
                           // TODO_RUST_CODE
                           // Java: textRunList.save();
                           // TODO_RUST_CODE
                           // Java: TextParagraph newparagraph = TextParagraph.create();
                           // TODO_RUST_CODE
                           // Java: newparagraph.writeTextruns(textRunList);
                           // TODO_RUST_CODE
                           // Java: newparagraph.writeEndParaRPr(this.defaultEndParaRPr());
                           // TODO_RUST_CODE
                           // Java: textParagraphs.add(TextParagraph.open(paragraph.save()));
                           // TODO_RUST_CODE
                        }
                     }
                  }
                  // Java: textparagraphList.write(textParagraphs);
                  // TODO_RUST_CODE
                  // Java: textparagraphList.save();
                  // TODO_RUST_CODE
                  // Java: data = textparagraphList;
                  // TODO_RUST_CODE
               }
            }
            // Java: if (data instanceof TextBody.ParagraphList)
                // TODO_RUST_CODE
            {
               // Java: if (bCovertToRunList)
                    // TODO_RUST_CODE
               {
                  // Java: TextParagraph.TextRunList runList = this.covertToRunList((TextBody.ParagraphList)data);
                  // TODO_RUST_CODE
                  // Java: result = this.insert(cp, runList);
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: result = this.insert(cp, (TextBody.ParagraphList)data);
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: this.onDataChangeAfter(true);
         // TODO_RUST_CODE
         // Java: return result;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean delete(int startDelete, int endDelete)
    pub fn delete(&self, startDelete: i32, endDelete: i32) -> bool {
      // Java: return this.__delete(startDelete, endDelete);
      // TODO_RUST_CODE
    }

    // Java: boolean __delete(int start, int end)
    pub fn __delete(&self, start: i32, end: i32) -> bool {
      // Java: if (start >= end)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else if (0 == start && end == Integer.MAX_VALUE)
        // TODO_RUST_CODE
      {
         // Java: return this.__deleteAll();
         // TODO_RUST_CODE
      }
      // Java: else if (this.__deleteInRun(start, end))
        // TODO_RUST_CODE
      {
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else if (this.__deleteInPara(start, end))
        // TODO_RUST_CODE
      {
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else if (!this.__seek(start, false))
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: TextParagraph firstparagraph = TextParagraph.open(this.getParagraphCache().save());
         // TODO_RUST_CODE
         // Java: TextParagraphProperties firstprop = TextParagraphProperties.open(firstparagraph.paraPr().save());
         // TODO_RUST_CODE
         // Java: List<TextRun> firstruns = new ArrayList();
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < this.idxRun; ++i)
            // TODO_RUST_CODE
         {
            // Java: TextRun newRun = TextRun.open(((TextRun)this.runsCache.get(i)).save());
            // TODO_RUST_CODE
            // Java: newRun.save();
            // TODO_RUST_CODE
            // Java: firstruns.add(newRun);
            // TODO_RUST_CODE
         }
         // Java: for(int i = this.idxRun; i < this.runsCache.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: TextRun runCache = (TextRun)this.runsCache.get(i);
            // TODO_RUST_CODE
            // Java: int endRun = this.startRun + runCache.length();
            // TODO_RUST_CODE
            // Java: if ((start > this.startRun || endRun > end) && 0 != runCache.length())
                // TODO_RUST_CODE
            {
               // Java: if (end > this.startRun && endRun > start)
                    // TODO_RUST_CODE
               {
                  // Java: StringBuilder textBuilder = new StringBuilder("");
                  // TODO_RUST_CODE
                  // Java: textBuilder.append(this.getRunText(runCache));
                  // TODO_RUST_CODE
                  // Java: int deleteStart = Math.max(0, start - this.startRun);
                  // TODO_RUST_CODE
                  // Java: int deleteEnd = Math.min(end - this.startRun, endRun - this.startRun);
                  // TODO_RUST_CODE
                  // Java: textBuilder.delete(deleteStart, deleteEnd);
                  // TODO_RUST_CODE
                  // Java: this.setRunText(runCache, textBuilder.toString());
                  // TODO_RUST_CODE
                  // Java: TextRun newrun = TextRun.open(runCache.save());
                  // TODO_RUST_CODE
                  // Java: newrun.save();
                  // TODO_RUST_CODE
                  // Java: firstruns.add(newrun);
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: TextRun newrun = TextRun.open(runCache.save());
                  // TODO_RUST_CODE
                  // Java: newrun.save();
                  // TODO_RUST_CODE
                  // Java: firstruns.add(newrun);
                  // TODO_RUST_CODE
               }
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: this.runsCache.remove(i--);
               // TODO_RUST_CODE
            }
            // Java: this.startRun = endRun;
            // TODO_RUST_CODE
         }
         // Java: this.startPara = ++this.startRun;
         // TODO_RUST_CODE
         // Java: int idxFirstParagraph = this.idxParagraph;
         // TODO_RUST_CODE
         // Java: int idxLastParagraph = 0;
         // TODO_RUST_CODE
         // Java: TextCharacterProperties lastRProp = null;
         // TODO_RUST_CODE
         // Java: for(idxLastParagraph = idxFirstParagraph + 1; idxLastParagraph < this.paragraphsCache.size(); ++idxLastParagraph)
            // TODO_RUST_CODE
         {
            // Java: this.clearRunCache();
            // TODO_RUST_CODE
            // Java: TextParagraph paragraphCache = (TextParagraph)this.paragraphsCache.get(idxLastParagraph);
            // TODO_RUST_CODE
            // Java: paragraphCache.textruns().read(this.runsCache);
            // TODO_RUST_CODE
            // Java: int endPara = this.startPara;
            // TODO_RUST_CODE
            // Java: for(int i = 0; i < this.runsCache.size(); ++i)
                // TODO_RUST_CODE
            {
               // Java: endPara += ((TextRun)this.runsCache.get(i)).length();
               // TODO_RUST_CODE
            }
            // Java: ++endPara;
            // TODO_RUST_CODE
            // Java: if (end <= endPara || idxLastParagraph + 1 == this.paragraphsCache.size())
                // TODO_RUST_CODE
            {
               // Java: lastRProp = TextCharacterProperties.open(paragraphCache.endParaRPr().save());
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            }
            // Java: this.startPara = endPara;
            // TODO_RUST_CODE
         }
         // Java: this.startRun = this.startPara;
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < this.runsCache.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: TextRun runCache = (TextRun)this.runsCache.get(i);
            // TODO_RUST_CODE
            // Java: int endRun = this.startRun + runCache.length();
            // TODO_RUST_CODE
            // Java: if ((start > this.startRun || endRun > end) && 0 != runCache.length())
                // TODO_RUST_CODE
            {
               // Java: if (end > this.startRun && endRun > start)
                    // TODO_RUST_CODE
               {
                  // Java: StringBuilder textBuilder = new StringBuilder("");
                  // TODO_RUST_CODE
                  // Java: textBuilder.append(this.getRunText(runCache));
                  // TODO_RUST_CODE
                  // Java: int deleteStart = Math.max(0, start - this.startRun);
                  // TODO_RUST_CODE
                  // Java: int deleteEnd = Math.min(end - this.startRun, endRun - this.startRun);
                  // TODO_RUST_CODE
                  // Java: textBuilder.delete(deleteStart, deleteEnd);
                  // TODO_RUST_CODE
                  // Java: this.setRunText(runCache, textBuilder.toString());
                  // TODO_RUST_CODE
                  // Java: TextRun newrun = TextRun.open(runCache.save());
                  // TODO_RUST_CODE
                  // Java: newrun.save();
                  // TODO_RUST_CODE
                  // Java: firstruns.add(newrun);
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: TextRun newrun = TextRun.open(runCache.save());
                  // TODO_RUST_CODE
                  // Java: newrun.save();
                  // TODO_RUST_CODE
                  // Java: firstruns.add(newrun);
                  // TODO_RUST_CODE
               }
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: this.runsCache.remove(i--);
               // TODO_RUST_CODE
            }
            // Java: this.startRun = endRun;
            // TODO_RUST_CODE
         }
         // Java: idxLastParagraph = Math.min(idxLastParagraph, this.paragraphsCache.size() - 1);
         // TODO_RUST_CODE
         // Java: for(int i = idxLastParagraph; idxFirstParagraph <= i; --i)
            // TODO_RUST_CODE
         {
            // Java: this.paragraphsCache.remove(i);
            // TODO_RUST_CODE
         }
         // Java: if (0 < firstruns.size())
            // TODO_RUST_CODE
         {
            // Java: firstparagraph.textruns().write(firstruns);
            // TODO_RUST_CODE
            // Java: firstparagraph.forceWriteEndParaRPr(lastRProp);
            // TODO_RUST_CODE
            // Java: firstparagraph.save();
            // TODO_RUST_CODE
            // Java: this.paragraphsCache.add(idxFirstParagraph, firstparagraph);
            // TODO_RUST_CODE
         }
         // Java: else if (0 == this.paragraphsCache.size())
            // TODO_RUST_CODE
         {
            // Java: TextParagraph newparagraph = TextParagraph.create();
            // TODO_RUST_CODE
            // Java: newparagraph.forceWriteEndParaRPr(lastRProp);
            // TODO_RUST_CODE
            // Java: newparagraph.forceWriteParaPr(firstprop);
            // TODO_RUST_CODE
            // Java: newparagraph.save();
            // TODO_RUST_CODE
            // Java: this.paragraphsCache.add(newparagraph);
            // TODO_RUST_CODE
         }
         // Java: this.beforeWrite();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(this.paragraphsCache);
         // TODO_RUST_CODE
         // Java: this.onDataChangeAfter(true);
         // TODO_RUST_CODE
         // Java: return true;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean __deleteAll()
    pub fn __deleteAll(&self) -> bool {
      // Java: if (0 == this.paragraphsCache.size())
        // TODO_RUST_CODE
      {
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(this.paragraphsCache);
         // TODO_RUST_CODE
      }
      // Java: if (0 < this.paragraphsCache.size())
        // TODO_RUST_CODE
      {
         // Java: TextParagraph first = TextParagraph.open(((TextParagraph)this.paragraphsCache.get(0)).save());
         // TODO_RUST_CODE
         // Java: if (!first.hasEndParaRPr())
            // TODO_RUST_CODE
         {
            // Java: List<TextRun> runs = new ArrayList();
            // TODO_RUST_CODE
            // Java: first.textruns().read(runs);
            // TODO_RUST_CODE
            // Java: if (0 < runs.size())
                // TODO_RUST_CODE
            {
               // Java: first.writeEndParaRPr(this.getCharacterProperties((TextRun)runs.get(runs.size() - 1)));
               // TODO_RUST_CODE
            }
         }
         // Java: first.deleteTextruns();
         // TODO_RUST_CODE
         // Java: first.save();
         // TODO_RUST_CODE
         // Java: this.paragraphsCache.clear();
         // TODO_RUST_CODE
         // Java: this.paragraphsCache.add(first);
         // TODO_RUST_CODE
         // Java: this.beforeWrite();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(this.paragraphsCache);
         // TODO_RUST_CODE
         // Java: this.onDataChangeAfter(true);
         // TODO_RUST_CODE
      }
      // Java: return true;
      // TODO_RUST_CODE
    }

    // Java: boolean __deleteInRun(int start, int end)
    pub fn __deleteInRun(&self, start: i32, end: i32) -> bool {
      // Java: if (!this.__seek(start, false))
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: TextRun runCache = this.getRunCache();
         // TODO_RUST_CODE
         // Java: if (null == runCache)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: int endRun = this.startRun + runCache.length();
            // TODO_RUST_CODE
            // Java: if (this.startRun <= start && end <= endRun)
                // TODO_RUST_CODE
            {
               // Java: boolean isClearCache = true;
               // TODO_RUST_CODE
               // Java: if (this.startRun == start && end == endRun)
                    // TODO_RUST_CODE
               {
                  // Java: this.runsCache.remove(this.idxRun);
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: StringBuilder textBuilder = new StringBuilder("");
                  // TODO_RUST_CODE
                  // Java: textBuilder.append(this.getRunText(runCache));
                  // TODO_RUST_CODE
                  // Java: textBuilder.delete(start - this.startRun, end - this.startRun);
                  // TODO_RUST_CODE
                  // Java: this.setRunText(runCache, textBuilder.toString());
                  // TODO_RUST_CODE
                  // Java: isClearCache = false;
                  // TODO_RUST_CODE
               }
               // Java: TextParagraph paragraphCache = this.getParagraphCache();
               // TODO_RUST_CODE
               // Java: if (0 < this.runsCache.size())
                    // TODO_RUST_CODE
               {
                  // Java: paragraphCache.textruns().write(this.runsCache);
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: paragraphCache.deleteTextruns();
                  // TODO_RUST_CODE
               }
               // Java: paragraphCache.save();
               // TODO_RUST_CODE
               // Java: this.beforeWrite();
               // TODO_RUST_CODE
               // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(this.paragraphsCache);
               // TODO_RUST_CODE
               // Java: this.onDataChangeAfter(isClearCache);
               // TODO_RUST_CODE
               // Java: return true;
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: return false;
               // TODO_RUST_CODE
            }
         }
      }
    }

    // Java: boolean __deleteInPara(int start, int end)
    pub fn __deleteInPara(&self, start: i32, end: i32) -> bool {
      // Java: if (!this.__seek(start, false))
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: int endPara = this.startPara;
         // TODO_RUST_CODE
         // Java: for(int i = 0; i < this.runsCache.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: endPara += ((TextRun)this.runsCache.get(i)).length();
            // TODO_RUST_CODE
         }
         // Java: label81:
            // TODO_RUST_CODE
         {
            // Java: if (this.startPara <= start)
                // TODO_RUST_CODE
            {
               // Java: ++endPara;
               // TODO_RUST_CODE
               // Java: if (end <= endPara)
                    // TODO_RUST_CODE
               {
                  // Java: break label81;
                  // TODO_RUST_CODE
               }
            }
            // Java: if (this.idxParagraph != this.paragraphsCache.size() - 1)
                // TODO_RUST_CODE
            {
               // Java: return false;
               // TODO_RUST_CODE
            }
         }
         // Java: for(int i = this.idxRun; i < this.runsCache.size(); ++i)
            // TODO_RUST_CODE
         {
            // Java: TextRun runCache = (TextRun)this.runsCache.get(i);
            // TODO_RUST_CODE
            // Java: int endRun = this.startRun + runCache.length();
            // TODO_RUST_CODE
            // Java: if (start <= this.startRun && endRun <= end)
                // TODO_RUST_CODE
            {
               // Java: this.runsCache.remove(i--);
               // TODO_RUST_CODE
            }
            // Java: else if (end > this.startRun && endRun > start)
                // TODO_RUST_CODE
            {
               // Java: StringBuilder textBuilder = new StringBuilder("");
               // TODO_RUST_CODE
               // Java: textBuilder.append(this.getRunText(runCache));
               // TODO_RUST_CODE
               // Java: int deleteStart = Math.max(0, start - this.startRun);
               // TODO_RUST_CODE
               // Java: int deleteEnd = Math.min(end - this.startRun, endRun - this.startRun);
               // TODO_RUST_CODE
               // Java: textBuilder.delete(deleteStart, deleteEnd);
               // TODO_RUST_CODE
               // Java: this.setRunText(runCache, textBuilder.toString());
               // TODO_RUST_CODE
            }
            // Java: if (end <= endRun)
                // TODO_RUST_CODE
            {
               // Java: break;
               // TODO_RUST_CODE
            }
            // Java: this.startRun = endRun;
            // TODO_RUST_CODE
         }
         // Java: TextParagraph paragraphCache = this.getParagraphCache();
         // TODO_RUST_CODE
         // Java: if (endPara != end)
            // TODO_RUST_CODE
         {
            // Java: if (0 < this.runsCache.size())
                // TODO_RUST_CODE
            {
               // Java: paragraphCache.textruns().write(this.runsCache);
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: paragraphCache.deleteTextruns();
               // TODO_RUST_CODE
            }
            // Java: paragraphCache.save();
            // TODO_RUST_CODE
         }
         // Java: else if (0 < this.runsCache.size())
            // TODO_RUST_CODE
         {
            // Java: if (this.idxParagraph + 1 < this.paragraphsCache.size())
                // TODO_RUST_CODE
            {
               // Java: TextParagraph nextparagraph = (TextParagraph)this.paragraphsCache.get(this.idxParagraph + 1);
               // TODO_RUST_CODE
               // Java: List<TextRun> nextruns = new ArrayList();
               // TODO_RUST_CODE
               // Java: nextparagraph.textruns().read(nextruns);
               // TODO_RUST_CODE
               // Java: for(int i = 0; i < nextruns.size(); ++i)
                    // TODO_RUST_CODE
               {
                  // Java: TextRun nextrun = TextRun.open(((TextRun)nextruns.get(i)).save());
                  // TODO_RUST_CODE
                  // Java: nextrun.save();
                  // TODO_RUST_CODE
                  // Java: this.runsCache.add(nextrun);
                  // TODO_RUST_CODE
               }
               // Java: this.paragraphsCache.remove(this.idxParagraph + 1);
               // TODO_RUST_CODE
            }
            // Java: paragraphCache.textruns().write(this.runsCache);
            // TODO_RUST_CODE
            // Java: paragraphCache.save();
            // TODO_RUST_CODE
         }
         // Java: else if (0 >= this.idxParagraph && this.idxParagraph + 1 >= this.paragraphsCache.size())
            // TODO_RUST_CODE
         {
            // Java: paragraphCache.deleteTextruns();
            // TODO_RUST_CODE
            // Java: paragraphCache.save();
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: this.paragraphsCache.remove(this.idxParagraph);
            // TODO_RUST_CODE
         }
         // Java: this.beforeWrite();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(this.paragraphsCache);
         // TODO_RUST_CODE
         // Java: this.onDataChangeAfter(true);
         // TODO_RUST_CODE
         // Java: return true;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean format(KoAr format, int mode)
    pub fn format(&self, format: KoAr, mode: i32) -> bool {
      // Java: return this.__format(format, mode);
      // TODO_RUST_CODE
    }

    // Java: boolean format(int cp, int len, KoAr format, int mode)
    pub fn format(&self, cp: i32, len: i32, format: KoAr, mode: i32) -> bool {
      // Java: return this.__format(cp, len, format, mode);
      // TODO_RUST_CODE
    }

    // Java: boolean __format(KoAr format, int mode)
    pub fn __format(&self, format: KoAr, mode: i32) -> bool {
      // Java: if (null == format)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: boolean result = false;
         // TODO_RUST_CODE
         // Java: if (format instanceof TextParagraphProperties)
            // TODO_RUST_CODE
         {
            // Java: result = this.formatTextParagraphProperties((TextParagraphProperties)format, mode);
            // TODO_RUST_CODE
         }
         // Java: else if (format instanceof TextCharacterProperties)
            // TODO_RUST_CODE
         {
            // Java: result = this.formatTextCharacterProperties((TextCharacterProperties)format, mode);
            // TODO_RUST_CODE
         }
         // Java: else if (format instanceof TextBodyProperties)
            // TODO_RUST_CODE
         {
            // Java: result = this.formatTextBodyProperties((TextBodyProperties)format);
            // TODO_RUST_CODE
         }
         // Java: else if (format instanceof TextListStyle)
            // TODO_RUST_CODE
         {
            // Java: result = this.formatTextListProperties((TextListStyle)format, mode);
            // TODO_RUST_CODE
         }
         // Java: this.onDataChangeAfter(true);
         // TODO_RUST_CODE
         // Java: return result;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean __format(int cp, int len, KoAr format, int mode)
    pub fn __format(&self, cp: i32, len: i32, format: KoAr, mode: i32) -> bool {
      // Java: if (null == format)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: boolean result = false;
         // TODO_RUST_CODE
         // Java: if (format instanceof TextCharacterProperties)
            // TODO_RUST_CODE
         {
            // Java: result = this.formatTextCharacterProperties(cp, len, (TextCharacterProperties)format, mode);
            // TODO_RUST_CODE
         }
         // Java: else if (format instanceof TextParagraphProperties)
            // TODO_RUST_CODE
         {
            // Java: result = this.formatTextParagraphProperties(cp, len, (TextParagraphProperties)format, mode);
            // TODO_RUST_CODE
         }
         // Java: this.onDataChangeAfter(true);
         // TODO_RUST_CODE
         // Java: return result;
         // TODO_RUST_CODE
      }
    }

    // Java: void setRunText(TextRun txRun, String text)
    pub fn setRunText(&self, txRun: TextRun, text: String) {
      // Java: StringBuilder strRun = new StringBuilder("");
      // TODO_RUST_CODE
      // Java: if (null != text && 0 < text.length())
        // TODO_RUST_CODE
      {
         // Java: strRun.append(text);
         // TODO_RUST_CODE
      }
      // Java: if (2 == txRun.mode())
        // TODO_RUST_CODE
      {
         // Java: if (!txRun.hasFieldRun())
            // TODO_RUST_CODE
         {
            // Java: TextRun.TextField textField = TextRun.TextField.create();
            // TODO_RUST_CODE
            // Java: textField.putText(text);
            // TODO_RUST_CODE
            // Java: textField.save();
            // TODO_RUST_CODE
            // Java: txRun.writeFieldRun(textField);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: txRun.fieldRun().putText(strRun.toString());
            // TODO_RUST_CODE
         }
      }
      // Java: else if (0 == txRun.mode())
        // TODO_RUST_CODE
      {
         // Java: if (!txRun.hasRegularRun())
            // TODO_RUST_CODE
         {
            // Java: TextRun.RegularTextRun regularTextRun = TextRun.RegularTextRun.create();
            // TODO_RUST_CODE
            // Java: regularTextRun.putText(text);
            // TODO_RUST_CODE
            // Java: regularTextRun.save();
            // TODO_RUST_CODE
            // Java: txRun.writeRegularRun(regularTextRun);
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: txRun.regularRun().putText(strRun.toString());
            // TODO_RUST_CODE
         }
      }
      // Java: txRun.putLength(text.length());
      // TODO_RUST_CODE
      // Java: txRun.save();
      // TODO_RUST_CODE
    }

    // Java: List<TextRun> getRuns(int idxPara)
    pub fn getRuns(&self, idxPara: i32) -> List<TextRun> {
      // Java: List<TextParagraph> paragraphs = null;
      // TODO_RUST_CODE
      // Java: Object var4;
      // TODO_RUST_CODE
      // Java: if (0 < this.paragraphsCache.size())
        // TODO_RUST_CODE
      {
         // Java: var4 = this.paragraphsCache;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: var4 = new ArrayList();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read((Collection)var4);
         // TODO_RUST_CODE
      }
      // Java: List<TextRun> runs = new ArrayList();
      // TODO_RUST_CODE
      // Java: if (0 <= idxPara && idxPara < ((List)var4).size())
        // TODO_RUST_CODE
      {
         // Java: ((TextParagraph)((List)var4).get(idxPara)).textruns().read(runs);
         // TODO_RUST_CODE
      }
      // Java: return runs;
      // TODO_RUST_CODE
    }

    // Java: void clearIndex()
    pub fn clearIndex(&mut self) {
      // Java: this.idxRun = 0;
      self.idxRun = 0;
      // Java: this.idxParagraph = 0;
      self.idxParagraph = 0;
      // Java: this.startRun = 0;
      self.startRun = 0;
      // Java: this.startPara = 0;
      self.startPara = 0;
    }

    // Java: void clearCache()
    pub fn clearCache(&mut self) {
      // Java: this.clearRunCache();
      self.clearRunCache();
      // Java: this.clearParagraphCache();
      self.clearParagraphCache();
    }

    // Java: boolean formatTextCharacterProperties(int cp, int len, TextCharacterProperties format, int mode)
    pub fn formatTextCharacterProperties(&self, cp: i32, len: i32, format: TextCharacterProperties, mode: i32) -> bool {
      // Java: int formatStart = cp;
      // TODO_RUST_CODE
      // Java: int formatEnd = cp + len;
      // TODO_RUST_CODE
      // Java: if (null == format)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: if (0 == this.paragraphsCache.size())
            // TODO_RUST_CODE
         {
            // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(this.paragraphsCache);
            // TODO_RUST_CODE
         }
         // Java: List<TextParagraph> textparagraphs = this.paragraphsCache;
         // TODO_RUST_CODE
         // Java: this.clearIndex();
         // TODO_RUST_CODE
         // Java: int start = 0;
         // TODO_RUST_CODE
         // Java: int end = 0;
         // TODO_RUST_CODE
         // Java: for(this.idxParagraph = 0; this.idxParagraph < textparagraphs.size() && start < formatEnd; ++this.idxParagraph)
            // TODO_RUST_CODE
         {
            // Java: TextParagraph paragraph = (TextParagraph)textparagraphs.get(this.idxParagraph);
            // TODO_RUST_CODE
            // Java: List<TextRun> textruns = new ArrayList();
            // TODO_RUST_CODE
            // Java: paragraph.textruns().read(textruns);
            // TODO_RUST_CODE
            // Java: TextRun unformatRun = null;
            // TODO_RUST_CODE
            // Java: boolean isFormatChanged = false;
            // TODO_RUST_CODE
            // Java: for(this.idxRun = 0; this.idxRun < textruns.size() && start < formatEnd; ++this.idxRun)
                // TODO_RUST_CODE
            {
               // Java: TextRun textrun = (TextRun)textruns.get(this.idxRun);
               // TODO_RUST_CODE
               // Java: unformatRun = textrun;
               // TODO_RUST_CODE
               // Java: end = start + textrun.length();
               // TODO_RUST_CODE
               // Java: if (formatStart <= start)
                    // TODO_RUST_CODE
               {
                  // Java: if (end <= formatEnd)
                        // TODO_RUST_CODE
                  {
                     // Java: this.__formatTextCharacterProperties(format, textrun, mode);
                     // TODO_RUST_CODE
                     // Java: isFormatChanged = true;
                     // TODO_RUST_CODE
                  }
                  // Java: else if (start < formatEnd)
                        // TODO_RUST_CODE
                  {
                     // Java: String text = this.getRunText(textrun);
                     // TODO_RUST_CODE
                     // Java: TextRun newTextRun = TextRun.open(textrun.save());
                     // TODO_RUST_CODE
                     // Java: this.setRunText(textrun, text.substring(0, formatEnd - start));
                     // TODO_RUST_CODE
                     // Java: this.__formatTextCharacterProperties(format, textrun, mode);
                     // TODO_RUST_CODE
                     // Java: this.setRunText(newTextRun, text.substring(formatEnd - start, text.length()));
                     // TODO_RUST_CODE
                     // Java: textruns.add(++this.idxRun, newTextRun);
                     // TODO_RUST_CODE
                     // Java: isFormatChanged = true;
                     // TODO_RUST_CODE
                  }
               }
               // Java: else if (formatStart < end && end <= formatEnd)
                    // TODO_RUST_CODE
               {
                  // Java: String text = this.getRunText(textrun);
                  // TODO_RUST_CODE
                  // Java: TextRun newTextRun = TextRun.open(textrun.save());
                  // TODO_RUST_CODE
                  // Java: this.setRunText(textrun, text.substring(0, formatStart - start));
                  // TODO_RUST_CODE
                  // Java: this.setRunText(newTextRun, text.substring(formatStart - start, text.length()));
                  // TODO_RUST_CODE
                  // Java: this.__formatTextCharacterProperties(format, newTextRun, mode);
                  // TODO_RUST_CODE
                  // Java: textruns.add(++this.idxRun, newTextRun);
                  // TODO_RUST_CODE
                  // Java: isFormatChanged = true;
                  // TODO_RUST_CODE
               }
               // Java: else if (start < formatStart && formatEnd < end)
                    // TODO_RUST_CODE
               {
                  // Java: String text = this.getRunText(textrun);
                  // TODO_RUST_CODE
                  // Java: TextRun formatTextRun = TextRun.open(textrun.save());
                  // TODO_RUST_CODE
                  // Java: TextRun nextTextRun = TextRun.open(textrun.save());
                  // TODO_RUST_CODE
                  // Java: this.setRunText(textrun, text.substring(0, formatStart - start));
                  // TODO_RUST_CODE
                  // Java: this.setRunText(formatTextRun, text.substring(formatStart - start, formatEnd - start));
                  // TODO_RUST_CODE
                  // Java: this.__formatTextCharacterProperties(format, formatTextRun, mode);
                  // TODO_RUST_CODE
                  // Java: textruns.add(++this.idxRun, formatTextRun);
                  // TODO_RUST_CODE
                  // Java: this.setRunText(nextTextRun, text.substring(formatEnd - start, text.length()));
                  // TODO_RUST_CODE
                  // Java: textruns.add(++this.idxRun, nextTextRun);
                  // TODO_RUST_CODE
                  // Java: isFormatChanged = true;
                  // TODO_RUST_CODE
               }
               // Java: start = end;
               // TODO_RUST_CODE
            }
            // Java: if (isFormatChanged)
                // TODO_RUST_CODE
            {
               // Java: paragraph.textruns().write(textruns);
               // TODO_RUST_CODE
            }
            // Java: if (textruns.size() == this.idxRun && (formatStart <= start && end < formatEnd || 0 < len && end == formatEnd) && 4 != mode)
                // TODO_RUST_CODE
            {
               // Java: TextCharacterProperties defProp = null;
               // TODO_RUST_CODE
               // Java: if (!paragraph.hasEndParaRPr() && 0 < textruns.size())
                    // TODO_RUST_CODE
               {
                  // Java: TextRun lastRun;
                  // TODO_RUST_CODE
                  // Java: if (unformatRun != null)
                        // TODO_RUST_CODE
                  {
                     // Java: lastRun = unformatRun;
                     // TODO_RUST_CODE
                  }
                  // Java: else
                        // TODO_RUST_CODE
                  {
                     // Java: lastRun = (TextRun)textruns.get(textruns.size() - 1);
                     // TODO_RUST_CODE
                  }
                  // Java: defProp = TextCharacterProperties.open(this.getCharacterProperties(lastRun).save());
                  // TODO_RUST_CODE
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: defProp = TextCharacterProperties.open(paragraph.endParaRPr().save());
                  // TODO_RUST_CODE
               }
               // Java: TextCharacterProperties endRpr = TextCharacterProperties.open(format.save());
               // TODO_RUST_CODE
               // Java: if (endRpr.hasHlinkClick())
                    // TODO_RUST_CODE
               {
                  // Java: endRpr.deleteHlinkClick();
                  // TODO_RUST_CODE
                  // Java: endRpr.save();
                  // TODO_RUST_CODE
               }
               // Java: this.mergeTextCharacterProperties(endRpr, defProp);
               // TODO_RUST_CODE
               // Java: defProp.save();
               // TODO_RUST_CODE
               // Java: paragraph.writeEndParaRPr(defProp);
               // TODO_RUST_CODE
            }
            // Java: paragraph.save();
            // TODO_RUST_CODE
            // Java: ++end;
            // TODO_RUST_CODE
            // Java: start = end;
            // TODO_RUST_CODE
         }
         // Java: this.beforeWrite();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(textparagraphs);
         // TODO_RUST_CODE
         // Java: return true;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean formatTextParagraphProperties(int cp, int len, TextParagraphProperties format, int mode)
    pub fn formatTextParagraphProperties(&self, cp: i32, len: i32, format: TextParagraphProperties, mode: i32) -> bool {
      // Java: int formatStart = cp;
      // TODO_RUST_CODE
      // Java: int formatEnd = cp + Math.max(len, 1);
      // TODO_RUST_CODE
      // Java: if (null == format)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else if (!this.__seek(cp, false))
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: List<TextParagraph> textparagraphs = this.paragraphsCache;
         // TODO_RUST_CODE
         // Java: int start = this.startPara;
         // TODO_RUST_CODE
         // Java: int end = this.startPara;
         // TODO_RUST_CODE
         // Java: while(this.idxParagraph < textparagraphs.size() && start < formatEnd)
            // TODO_RUST_CODE
         {
            // Java: TextParagraph paragraph = (TextParagraph)textparagraphs.get(this.idxParagraph);
            // TODO_RUST_CODE
            // Java: List<TextRun> runs = new ArrayList();
            // TODO_RUST_CODE
            // Java: paragraph.textruns().read(runs);
            // TODO_RUST_CODE
            // Java: for(int i = 0; i < runs.size(); ++i)
                // TODO_RUST_CODE
            {
               // Java: end += ((TextRun)runs.get(i)).length();
               // TODO_RUST_CODE
            }
            // Java: ++end;
            // TODO_RUST_CODE
            // Java: if (formatStart <= start)
                // TODO_RUST_CODE
            {
               // Java: if (end <= formatEnd || start < formatEnd)
                    // TODO_RUST_CODE
               {
                  // Java: this.__formatTextParagraphProperties(format, paragraph);
                  // TODO_RUST_CODE
               }
            }
            // Java: else if (end <= formatEnd || start < formatStart && formatEnd < end)
                // TODO_RUST_CODE
            {
               // Java: this.__formatTextParagraphProperties(format, paragraph);
               // TODO_RUST_CODE
            }
            // Java: start = end;
            // TODO_RUST_CODE
            // Java: ++this.idxParagraph;
            // TODO_RUST_CODE
            // Java: paragraph.save();
            // TODO_RUST_CODE
         }
         // Java: this.beforeWrite();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(textparagraphs);
         // TODO_RUST_CODE
         // Java: return true;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean formatTextCharacterProperties(TextCharacterProperties format, int mode)
    pub fn formatTextCharacterProperties(&self, format: TextCharacterProperties, mode: i32) -> bool {
      // Java: if (null == format)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: TextRun runCache = this.getRunCache();
         // TODO_RUST_CODE
         // Java: TextParagraph paragraphCache = this.getParagraphCache();
         // TODO_RUST_CODE
         // Java: if (null == paragraphCache)
            // TODO_RUST_CODE
         {
            // Java: this.meansureCache();
            // TODO_RUST_CODE
            // Java: paragraphCache = this.getParagraphCache();
            // TODO_RUST_CODE
         }
         // Java: if (null != runCache)
            // TODO_RUST_CODE
         {
            // Java: this.__formatTextCharacterProperties(format, runCache, mode);
            // TODO_RUST_CODE
            // Java: paragraphCache.textruns().write(this.runsCache);
            // TODO_RUST_CODE
            // Java: paragraphCache.save();
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: if (null == paragraphCache)
                // TODO_RUST_CODE
            {
               // Java: return false;
               // TODO_RUST_CODE
            }
            // Java: TextCharacterProperties newEndParaRPr = TextCharacterProperties.open(paragraphCache.endParaRPr().save());
            // TODO_RUST_CODE
            // Java: switch (mode)
                // TODO_RUST_CODE
            {
               // Java: case 1:
               // TODO_RUST_CODE
                  // Java: newEndParaRPr = TextCharacterProperties.open(format.save());
                  // TODO_RUST_CODE
                  // Java: break;
                  // TODO_RUST_CODE
               // Java: case 2:
               // TODO_RUST_CODE
                  // Java: this.clearTextCharacterProperties(format, newEndParaRPr);
                  // TODO_RUST_CODE
                  // Java: break;
                  // TODO_RUST_CODE
               // Java: case 8:
               // TODO_RUST_CODE
                  // Java: this.brushTextCharacterProperties(format, newEndParaRPr);
                  // TODO_RUST_CODE
                  // Java: break;
                  // TODO_RUST_CODE
               // Java: default:
               // TODO_RUST_CODE
                  // Java: this.mergeTextCharacterProperties(format, newEndParaRPr);
                  // TODO_RUST_CODE
            }
            // Java: newEndParaRPr.save();
            // TODO_RUST_CODE
            // Java: paragraphCache.writeEndParaRPr(newEndParaRPr);
            // TODO_RUST_CODE
            // Java: paragraphCache.save();
            // TODO_RUST_CODE
         }
         // Java: this.beforeWrite();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(this.paragraphsCache);
         // TODO_RUST_CODE
         // Java: return true;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean formatTextListProperties(TextListStyle listStyle, int mode)
    pub fn formatTextListProperties(&self, listStyle: TextListStyle, mode: i32) -> bool {
      // Java: if (null == listStyle)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else if (!this.__seek(0, false))
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: List<TextParagraph> textparagraphs;
         // TODO_RUST_CODE
         // Java: for(textparagraphs = this.paragraphsCache; this.idxParagraph < textparagraphs.size(); ++this.idxParagraph)
            // TODO_RUST_CODE
         {
            // Java: TextParagraph paragraph = (TextParagraph)textparagraphs.get(this.idxParagraph);
            // TODO_RUST_CODE
            // Java: List<TextRun> runs = new ArrayList();
            // TODO_RUST_CODE
            // Java: paragraph.textruns().read(runs);
            // TODO_RUST_CODE
            // Java: TextParagraphProperties prop = TextParagraphProperties.open(paragraph.paraPr().save());
            // TODO_RUST_CODE
            // Java: TextParagraphProperties newPPr;
            // TODO_RUST_CODE
            // Java: TextCharacterProperties newRPr;
            // TODO_RUST_CODE
            // Java: if (mode == 2)
                // TODO_RUST_CODE
            {
               // Java: newPPr = TextParagraphProperties.create();
               // TODO_RUST_CODE
               // Java: newPPr.putLvl(prop.lvl());
               // TODO_RUST_CODE
               // Java: newRPr = TextCharacterProperties.create();
               // TODO_RUST_CODE
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: switch (prop.lvl())
                    // TODO_RUST_CODE
               {
                  // Java: case 0:
                  // TODO_RUST_CODE
                     // Java: if (listStyle.hasLvl1pPr())
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.lvl1pPr();
                        // TODO_RUST_CODE
                     }
                     // Java: else
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.defpPr();
                        // TODO_RUST_CODE
                     }
                     // Java: break;
                     // TODO_RUST_CODE
                  // Java: case 1:
                  // TODO_RUST_CODE
                     // Java: if (listStyle.hasLvl2pPr())
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.lvl2pPr();
                        // TODO_RUST_CODE
                     }
                     // Java: else
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.defpPr();
                        // TODO_RUST_CODE
                     }
                     // Java: break;
                     // TODO_RUST_CODE
                  // Java: case 2:
                  // TODO_RUST_CODE
                     // Java: if (listStyle.hasLvl3pPr())
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.lvl3pPr();
                        // TODO_RUST_CODE
                     }
                     // Java: else
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.defpPr();
                        // TODO_RUST_CODE
                     }
                     // Java: break;
                     // TODO_RUST_CODE
                  // Java: case 3:
                  // TODO_RUST_CODE
                     // Java: if (listStyle.hasLvl4pPr())
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.lvl4pPr();
                        // TODO_RUST_CODE
                     }
                     // Java: else
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.defpPr();
                        // TODO_RUST_CODE
                     }
                     // Java: break;
                     // TODO_RUST_CODE
                  // Java: case 4:
                  // TODO_RUST_CODE
                     // Java: if (listStyle.hasLvl5pPr())
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.lvl5pPr();
                        // TODO_RUST_CODE
                     }
                     // Java: else
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.defpPr();
                        // TODO_RUST_CODE
                     }
                     // Java: break;
                     // TODO_RUST_CODE
                  // Java: case 5:
                  // TODO_RUST_CODE
                     // Java: if (listStyle.hasLvl6pPr())
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.lvl6pPr();
                        // TODO_RUST_CODE
                     }
                     // Java: else
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.defpPr();
                        // TODO_RUST_CODE
                     }
                     // Java: break;
                     // TODO_RUST_CODE
                  // Java: case 6:
                  // TODO_RUST_CODE
                     // Java: if (listStyle.hasLvl7pPr())
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.lvl7pPr();
                        // TODO_RUST_CODE
                     }
                     // Java: else
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.defpPr();
                        // TODO_RUST_CODE
                     }
                     // Java: break;
                     // TODO_RUST_CODE
                  // Java: case 7:
                  // TODO_RUST_CODE
                     // Java: if (listStyle.hasLvl8pPr())
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.lvl8pPr();
                        // TODO_RUST_CODE
                     }
                     // Java: else
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.defpPr();
                        // TODO_RUST_CODE
                     }
                     // Java: break;
                     // TODO_RUST_CODE
                  // Java: case 8:
                  // TODO_RUST_CODE
                     // Java: if (listStyle.hasLvl9pPr())
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.lvl9pPr();
                        // TODO_RUST_CODE
                     }
                     // Java: else
                        // TODO_RUST_CODE
                     {
                        // Java: newPPr = listStyle.defpPr();
                        // TODO_RUST_CODE
                     }
                     // Java: break;
                     // TODO_RUST_CODE
                  // Java: default:
                  // TODO_RUST_CODE
                     // Java: newPPr = listStyle.defpPr();
                     // TODO_RUST_CODE
               }
               // Java: newRPr = newPPr.defRPr();
               // TODO_RUST_CODE
            }
            // Java: TextParagraphProperties newProp = TextParagraphProperties.open(newPPr.save());
            // TODO_RUST_CODE
            // Java: newProp.deleteDefRPr();
            // TODO_RUST_CODE
            // Java: newProp.deleteLvl();
            // TODO_RUST_CODE
            // Java: if (prop.hasLvl())
                // TODO_RUST_CODE
            {
               // Java: newProp.putLvl(prop.lvl());
               // TODO_RUST_CODE
            }
            // Java: newProp.save();
            // TODO_RUST_CODE
            // Java: paragraph.forceWriteParaPr(newProp);
            // TODO_RUST_CODE
            // Java: for(TextRun run : runs)
                // TODO_RUST_CODE
            {
               // Java: if (run.regularRun().rPr().hasHlinkClick())
                    // TODO_RUST_CODE
               {
                  // Java: newRPr.writeHlinkClick(run.regularRun().rPr().hlinkClick());
                  // TODO_RUST_CODE
               }
               // Java: if (run.regularRun().rPr().hasHlinkMouseOver())
                    // TODO_RUST_CODE
               {
                  // Java: newRPr.writeHlinkMouseOver(run.regularRun().rPr().hlinkMouseOver());
                  // TODO_RUST_CODE
               }
               // Java: newRPr.save();
               // TODO_RUST_CODE
               // Java: run.regularRun().forceWriteRPr(newRPr);
               // TODO_RUST_CODE
               // Java: run.save();
               // TODO_RUST_CODE
            }
            // Java: if (paragraph.hasEndParaRPr())
                // TODO_RUST_CODE
            {
               // Java: paragraph.forceWriteEndParaRPr(newRPr);
               // TODO_RUST_CODE
            }
            // Java: paragraph.textruns().write(runs);
            // TODO_RUST_CODE
            // Java: paragraph.save();
            // TODO_RUST_CODE
         }
         // Java: this.beforeWrite();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(textparagraphs);
         // TODO_RUST_CODE
         // Java: return true;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean formatTextParagraphProperties(TextParagraphProperties format, int mode)
    pub fn formatTextParagraphProperties(&self, format: TextParagraphProperties, mode: i32) -> bool {
      // Java: TextParagraph paragraphCache = this.getParagraphCache();
      // TODO_RUST_CODE
      // Java: if (null == paragraphCache)
        // TODO_RUST_CODE
      {
         // Java: this.meansureCache();
         // TODO_RUST_CODE
         // Java: paragraphCache = this.getParagraphCache();
         // TODO_RUST_CODE
      }
      // Java: if (null != format && null != paragraphCache)
        // TODO_RUST_CODE
      {
         // Java: switch (mode)
            // TODO_RUST_CODE
         {
            // Java: case 1:
            // TODO_RUST_CODE
               // Java: TextParagraphProperties newParaProp = TextParagraphProperties.open(format.save());
               // TODO_RUST_CODE
               // Java: newParaProp.save();
               // TODO_RUST_CODE
               // Java: paragraphCache.forceWriteParaPr(newParaProp);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: case 2:
            // TODO_RUST_CODE
               // Java: this.__clearTextParagraphProperties(format, paragraphCache);
               // TODO_RUST_CODE
               // Java: break;
               // TODO_RUST_CODE
            // Java: default:
            // TODO_RUST_CODE
               // Java: this.__formatTextParagraphProperties(format, paragraphCache);
               // TODO_RUST_CODE
         }
         // Java: paragraphCache.save();
         // TODO_RUST_CODE
         // Java: this.beforeWrite();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(this.paragraphsCache);
         // TODO_RUST_CODE
         // Java: return true;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean formatTextBodyProperties(TextBodyProperties bodyProperties)
    pub fn formatTextBodyProperties(&self, bodyProperties: TextBodyProperties) -> bool {
      // Java: this.beforeWrite();
      // TODO_RUST_CODE
      // Java: bodyProperties.save();
      // TODO_RUST_CODE
      // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.writeBodyPr(bodyProperties);
      // TODO_RUST_CODE
      // Java: return true;
      // TODO_RUST_CODE
    }

    // Java: void mergeTextCharacterProperties(TextCharacterProperties runProp, TextCharacterProperties fotmatProp)
    pub fn mergeTextCharacterProperties(&self, runProp: TextCharacterProperties, fotmatProp: TextCharacterProperties) {
      // Java: if (runProp.hasAltLang())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.putAltLang(runProp.altLang());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasBaseline())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.putBaseline(runProp.baseline());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().hasBold())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.options().putBold(runProp.options().isBold());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasBookmarkLinkTarget())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.putBookmarkLinkTarget(runProp.bookmarkLinkTarget());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasCap())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.putCap(runProp.cap());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().hasDirty())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.options().putDirty(runProp.options().dirty());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasFontSize())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.putFontSize(runProp.fontSize());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().hasItalics())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.options().putItalics(runProp.options().isItalics());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasKern())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.putKern(runProp.kern());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().hasKumimoji())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.options().putKumimoji(runProp.options().isKumimoji());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasLang())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.putLang(runProp.lang());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().hasNoProof())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.options().putNoProof(runProp.options().isNoProof());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().hasNormalizeH())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.options().putNormalizeH(runProp.options().isNormalizeH());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasSpacing())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.putSpacing(runProp.spacing());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().hasSpellingErr())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.options().putSpellingErr(runProp.options().isSpellingErr());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasStrike())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.putStrike(runProp.strike());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasUnderline())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.putUnderline(runProp.underline());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasSmtId())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.putSmtId(runProp.smtId());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().hasSmtClean())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.options().putSmtClean(runProp.options().isSmtClean());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasOutline())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.writeOutline(runProp.outline());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasFill())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.writeFill(runProp.fill());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasEffect())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.forceWriteEffect(runProp.effect());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasHighlight())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.writeHighlight(runProp.highlight());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasUnderlineFollowText())
        // TODO_RUST_CODE
      {
         // Java: if (runProp.isUnderlineFillFollowText())
            // TODO_RUST_CODE
         {
            // Java: fotmatProp.putUnderlineFillFollowText(runProp.isUnderlineFillFollowText());
            // TODO_RUST_CODE
         }
         // Java: else if (fotmatProp.hasUnderlineFillFollowText())
            // TODO_RUST_CODE
         {
            // Java: fotmatProp.deleteUnderlineFillFollowText();
            // TODO_RUST_CODE
         }
      }
      // Java: if (runProp.hasUnderlineStroke())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.writeUnderlineStroke(runProp.underlineStroke());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasUnderlineFill())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.forceWriteUnderlineFill(runProp.underlineFill());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasLatinFont())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.writeLatinFont(runProp.latinFont());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasEastAsianFont())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.writeEastAsianFont(runProp.eastAsianFont());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasComplexScriptFont())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.writeComplexScriptFont(runProp.complexScriptFont());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasSymbolFont())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.writeSymbolFont(runProp.symbolFont());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasHlinkClick())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.writeHlinkClick(runProp.hlinkClick());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasHlinkMouseOver())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.writeHlinkMouseOver(runProp.hlinkMouseOver());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().hasRightToLeft())
        // TODO_RUST_CODE
      {
         // Java: fotmatProp.options().putRightToLeft(runProp.options().isRightToLeft());
         // TODO_RUST_CODE
      }
    }

    // Java: void mergeTextCharacterProperties(TextCharacterProperties runProp, TextRun textrun)
    pub fn mergeTextCharacterProperties(&self, runProp: TextCharacterProperties, textrun: TextRun) {
      // Java: int mode = textrun.mode();
      // TODO_RUST_CODE
      // Java: if (0 == mode)
        // TODO_RUST_CODE
      {
         // Java: this.mergeTextCharacterProperties(runProp, textrun.regularRun().rPr());
         // TODO_RUST_CODE
      }
      // Java: else if (2 == mode)
        // TODO_RUST_CODE
      {
         // Java: this.mergeTextCharacterProperties(runProp, textrun.fieldRun().rPr());
         // TODO_RUST_CODE
      }
      // Java: else if (1 == mode)
        // TODO_RUST_CODE
      {
         // Java: this.mergeTextCharacterProperties(runProp, textrun.linebreakRun().rPr());
         // TODO_RUST_CODE
      }
      // Java: textrun.save();
      // TODO_RUST_CODE
    }

    // Java: void brushTextCharacterProperties(TextCharacterProperties runProp, TextCharacterProperties formatProp)
    pub fn brushTextCharacterProperties(&self, runProp: TextCharacterProperties, formatProp: TextCharacterProperties) {
      // Java: if (runProp.baseline() != formatProp.baseline())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putBaseline(runProp.baseline());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().isBold() != formatProp.options().isBold())
        // TODO_RUST_CODE
      {
         // Java: formatProp.options().putBold(runProp.options().isBold());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.cap() != formatProp.cap())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putCap(runProp.cap());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.fontSize() != formatProp.fontSize())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putFontSize(runProp.fontSize());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().isItalics() != formatProp.options().isItalics())
        // TODO_RUST_CODE
      {
         // Java: formatProp.options().putItalics(runProp.options().isItalics());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.kern() != formatProp.kern())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putKern(runProp.kern());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().isNormalizeH() != formatProp.options().isNormalizeH())
        // TODO_RUST_CODE
      {
         // Java: formatProp.options().putNormalizeH(runProp.options().isNormalizeH());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.spacing() != formatProp.spacing())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putSpacing(runProp.spacing());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.strike() != formatProp.strike())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putStrike(runProp.strike());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.underline() != formatProp.underline())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putUnderline(runProp.underline());
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasFill())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeFill(runProp.fill());
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: formatProp.deleteFill();
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasEffect())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeEffect(runProp.effect());
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: formatProp.deleteEffect();
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasOutline())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeOutline(runProp.outline());
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: formatProp.deleteOutline();
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasUnderlineStroke())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeUnderlineStroke(runProp.underlineStroke());
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: formatProp.deleteUnderlineStroke();
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasUnderlineFill())
        // TODO_RUST_CODE
      {
         // Java: formatProp.forceWriteUnderlineFill(runProp.underlineFill());
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: formatProp.deleteUnderlineFill();
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasLatinFont())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeLatinFont(runProp.latinFont());
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: formatProp.deleteLatinFont();
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasEastAsianFont())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeEastAsianFont(runProp.eastAsianFont());
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: formatProp.deleteEastAsianFont();
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasComplexScriptFont())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeComplexScriptFont(runProp.complexScriptFont());
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: formatProp.deleteComplexScriptFont();
         // TODO_RUST_CODE
      }
      // Java: if (runProp.hasSymbolFont())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeSymbolFont(runProp.symbolFont());
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: formatProp.deleteSymbolFont();
         // TODO_RUST_CODE
      }
      // Java: if (runProp.options().hasRightToLeft())
        // TODO_RUST_CODE
      {
         // Java: formatProp.options().putRightToLeft(runProp.options().isRightToLeft());
         // TODO_RUST_CODE
      }
    }

    // Java: void brushTextCharacterProperties(TextCharacterProperties runProp, TextRun textrun)
    pub fn brushTextCharacterProperties(&self, runProp: TextCharacterProperties, textrun: TextRun) {
      // Java: int mode = textrun.mode();
      // TODO_RUST_CODE
      // Java: if (0 == mode)
        // TODO_RUST_CODE
      {
         // Java: this.brushTextCharacterProperties(runProp, textrun.regularRun().rPr());
         // TODO_RUST_CODE
      }
      // Java: else if (2 == mode)
        // TODO_RUST_CODE
      {
         // Java: this.brushTextCharacterProperties(runProp, textrun.fieldRun().rPr());
         // TODO_RUST_CODE
      }
      // Java: else if (1 == mode)
        // TODO_RUST_CODE
      {
         // Java: this.brushTextCharacterProperties(runProp, textrun.linebreakRun().rPr());
         // TODO_RUST_CODE
      }
      // Java: textrun.save();
      // TODO_RUST_CODE
    }

    // Java: void setTextCharacterProperties(TextCharacterProperties runProp, TextRun textrun)
    pub fn setTextCharacterProperties(&self, runProp: TextCharacterProperties, textrun: TextRun) {
      // Java: switch (textrun.mode())
        // TODO_RUST_CODE
      {
         // Java: case 0:
         // TODO_RUST_CODE
            // Java: if (textrun.hasRegularRun())
            // TODO_RUST_CODE
            {
               // Java: textrun.regularRun().writeRPr(TextCharacterProperties.open(runProp.save()));
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: TextRun.RegularTextRun regTextRun = TextRun.RegularTextRun.create();
               // TODO_RUST_CODE
               // Java: regTextRun.writeRPr(TextCharacterProperties.open(runProp.save()));
               // TODO_RUST_CODE
               // Java: regTextRun.save();
               // TODO_RUST_CODE
               // Java: textrun.writeRegularRun(regTextRun);
               // TODO_RUST_CODE
            }
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 1:
         // TODO_RUST_CODE
            // Java: if (textrun.hasLinebreakRun())
            // TODO_RUST_CODE
            {
               // Java: textrun.linebreakRun().writeRPr(TextCharacterProperties.open(runProp.save()));
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: TextRun.Linebreak lbTextRun = TextRun.Linebreak.create();
               // TODO_RUST_CODE
               // Java: lbTextRun.writeRPr(TextCharacterProperties.open(runProp.save()));
               // TODO_RUST_CODE
               // Java: lbTextRun.save();
               // TODO_RUST_CODE
               // Java: textrun.writeLinebreakRun(lbTextRun);
               // TODO_RUST_CODE
            }
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 2:
         // TODO_RUST_CODE
            // Java: if (textrun.hasFieldRun())
            // TODO_RUST_CODE
            {
               // Java: textrun.fieldRun().writeRPr(TextCharacterProperties.open(runProp.save()));
               // TODO_RUST_CODE
            }
            // Java: else
            // TODO_RUST_CODE
            {
               // Java: TextRun.TextField fieldTextRun = TextRun.TextField.create();
               // TODO_RUST_CODE
               // Java: fieldTextRun.writeRPr(TextCharacterProperties.open(runProp.save()));
               // TODO_RUST_CODE
               // Java: fieldTextRun.save();
               // TODO_RUST_CODE
               // Java: textrun.writeFieldRun(fieldTextRun);
               // TODO_RUST_CODE
            }
      }
      // Java: textrun.save();
      // TODO_RUST_CODE
    }

    // Java: TextCharacterProperties cleanTextCharacterProperties(TextCharacterProperties runProp, TextRun run)
    pub fn cleanTextCharacterProperties(&self, runProp: TextCharacterProperties, run: TextRun) -> TextCharacterProperties {
      // Java: TextCharacterProperties anotherProp = TextCharacterProperties.create();
      // TODO_RUST_CODE
      // Java: int mode = run.mode();
      // TODO_RUST_CODE
      // Java: if (0 == mode)
        // TODO_RUST_CODE
      {
         // Java: anotherProp = run.regularRun().rPr();
         // TODO_RUST_CODE
      }
      // Java: else if (2 == mode)
        // TODO_RUST_CODE
      {
         // Java: anotherProp = run.fieldRun().rPr();
         // TODO_RUST_CODE
      }
      // Java: else if (1 == mode)
        // TODO_RUST_CODE
      {
         // Java: anotherProp = run.linebreakRun().rPr();
         // TODO_RUST_CODE
      }
      // Java: if (null == runProp)
        // TODO_RUST_CODE
      {
         // Java: return TextCharacterProperties.open(anotherProp.save());
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: runProp.putFontSize(Math.min(runProp.fontSize(), anotherProp.fontSize()));
         // TODO_RUST_CODE
         // Java: if (runProp.hasLang() == anotherProp.hasLang())
            // TODO_RUST_CODE
         {
            // Java: if (runProp.hasLang() && 0 != runProp.lang().compareTo(anotherProp.lang()))
                // TODO_RUST_CODE
            {
               // Java: runProp.deleteLang();
               // TODO_RUST_CODE
            }
         }
         // Java: else if (runProp.hasLang())
            // TODO_RUST_CODE
         {
            // Java: runProp.deleteLang();
            // TODO_RUST_CODE
         }
         // Java: if (runProp.hasLatinFont() == anotherProp.hasLatinFont())
            // TODO_RUST_CODE
         {
            // Java: if (runProp.hasLatinFont() && !this.isEqual(runProp.latinFont().save(), anotherProp.latinFont().save()))
                // TODO_RUST_CODE
            {
               // Java: runProp.deleteLatinFont();
               // TODO_RUST_CODE
            }
         }
         // Java: else if (runProp.hasLatinFont())
            // TODO_RUST_CODE
         {
            // Java: runProp.deleteLatinFont();
            // TODO_RUST_CODE
         }
         // Java: if (runProp.hasEastAsianFont() == anotherProp.hasEastAsianFont())
            // TODO_RUST_CODE
         {
            // Java: if (runProp.hasEastAsianFont() && !this.isEqual(runProp.eastAsianFont().save(), anotherProp.eastAsianFont().save()))
                // TODO_RUST_CODE
            {
               // Java: runProp.deleteEastAsianFont();
               // TODO_RUST_CODE
            }
         }
         // Java: else if (runProp.hasEastAsianFont())
            // TODO_RUST_CODE
         {
            // Java: runProp.deleteEastAsianFont();
            // TODO_RUST_CODE
         }
         // Java: if (runProp.hasComplexScriptFont() == anotherProp.hasComplexScriptFont())
            // TODO_RUST_CODE
         {
            // Java: if (runProp.hasComplexScriptFont() && !this.isEqual(runProp.complexScriptFont().save(), anotherProp.complexScriptFont().save()))
                // TODO_RUST_CODE
            {
               // Java: runProp.deleteComplexScriptFont();
               // TODO_RUST_CODE
            }
         }
         // Java: else if (runProp.hasComplexScriptFont())
            // TODO_RUST_CODE
         {
            // Java: runProp.deleteComplexScriptFont();
            // TODO_RUST_CODE
         }
         // Java: if (runProp.hasOptions() == anotherProp.hasOptions())
            // TODO_RUST_CODE
         {
            // Java: if (runProp.options().hasBold() == anotherProp.options().hasBold())
                // TODO_RUST_CODE
            {
               // Java: if (runProp.options().hasBold() && runProp.options().isBold() != anotherProp.options().isBold())
                    // TODO_RUST_CODE
               {
                  // Java: runProp.options().deleteBold();
                  // TODO_RUST_CODE
               }
            }
            // Java: else if (runProp.options().hasBold())
                // TODO_RUST_CODE
            {
               // Java: runProp.options().deleteBold();
               // TODO_RUST_CODE
            }
            // Java: if (runProp.options().hasDirty() == anotherProp.options().hasDirty())
                // TODO_RUST_CODE
            {
               // Java: if (runProp.options().hasDirty() && runProp.options().dirty() != anotherProp.options().dirty())
                    // TODO_RUST_CODE
               {
                  // Java: runProp.options().deleteDirty();
                  // TODO_RUST_CODE
               }
            }
            // Java: else if (runProp.options().hasDirty())
                // TODO_RUST_CODE
            {
               // Java: runProp.options().deleteDirty();
               // TODO_RUST_CODE
            }
            // Java: if (runProp.options().hasItalics() == anotherProp.options().hasItalics())
                // TODO_RUST_CODE
            {
               // Java: if (runProp.options().hasItalics() && runProp.options().isItalics() != anotherProp.options().isItalics())
                    // TODO_RUST_CODE
               {
                  // Java: runProp.options().deleteItalics();
                  // TODO_RUST_CODE
               }
            }
            // Java: else if (runProp.options().hasItalics())
                // TODO_RUST_CODE
            {
               // Java: runProp.options().deleteItalics();
               // TODO_RUST_CODE
            }
            // Java: if (runProp.options().hasKumimoji() == anotherProp.options().hasBold())
                // TODO_RUST_CODE
            {
               // Java: if (runProp.options().hasKumimoji() && runProp.options().isKumimoji() != anotherProp.options().isKumimoji())
                    // TODO_RUST_CODE
               {
                  // Java: runProp.options().deleteKumimoji();
                  // TODO_RUST_CODE
               }
            }
            // Java: else if (runProp.options().hasKumimoji())
                // TODO_RUST_CODE
            {
               // Java: runProp.options().deleteKumimoji();
               // TODO_RUST_CODE
            }
            // Java: if (runProp.options().hasNoProof() == anotherProp.options().hasNoProof())
                // TODO_RUST_CODE
            {
               // Java: if (runProp.options().hasNoProof() && runProp.options().isNoProof() != anotherProp.options().isNoProof())
                    // TODO_RUST_CODE
               {
                  // Java: runProp.options().deleteNoProof();
                  // TODO_RUST_CODE
               }
            }
            // Java: else if (runProp.options().hasNoProof())
                // TODO_RUST_CODE
            {
               // Java: runProp.options().deleteNoProof();
               // TODO_RUST_CODE
            }
            // Java: if (runProp.options().hasSpellingErr() == anotherProp.options().hasSpellingErr())
                // TODO_RUST_CODE
            {
               // Java: if (runProp.options().hasSpellingErr() && runProp.options().isSpellingErr() != anotherProp.options().isSpellingErr())
                    // TODO_RUST_CODE
               {
                  // Java: runProp.options().deleteSpellingErr();
                  // TODO_RUST_CODE
               }
            }
            // Java: else if (runProp.options().hasSpellingErr())
                // TODO_RUST_CODE
            {
               // Java: runProp.options().deleteSpellingErr();
               // TODO_RUST_CODE
            }
            // Java: if (runProp.options().hasNormalizeH() == anotherProp.options().hasNormalizeH())
                // TODO_RUST_CODE
            {
               // Java: if (runProp.options().hasNormalizeH() && runProp.options().isNormalizeH() != anotherProp.options().isNormalizeH())
                    // TODO_RUST_CODE
               {
                  // Java: runProp.options().deleteNormalizeH();
                  // TODO_RUST_CODE
               }
            }
            // Java: else if (runProp.options().hasNormalizeH())
                // TODO_RUST_CODE
            {
               // Java: runProp.options().deleteNormalizeH();
               // TODO_RUST_CODE
            }
            // Java: if (runProp.options().hasRightToLeft() == anotherProp.options().hasRightToLeft())
                // TODO_RUST_CODE
            {
               // Java: if (runProp.options().hasRightToLeft() && runProp.options().isRightToLeft() != anotherProp.options().isRightToLeft())
                    // TODO_RUST_CODE
               {
                  // Java: runProp.options().deleteRightToLeft();
                  // TODO_RUST_CODE
               }
            }
            // Java: else if (runProp.options().hasRightToLeft())
                // TODO_RUST_CODE
            {
               // Java: runProp.options().deleteRightToLeft();
               // TODO_RUST_CODE
            }
         }
         // Java: else if (runProp.hasOptions())
            // TODO_RUST_CODE
         {
            // Java: runProp.deleteOptions();
            // TODO_RUST_CODE
         }
         // Java: if (runProp.hasUnderline() == anotherProp.hasUnderline())
            // TODO_RUST_CODE
         {
            // Java: if (runProp.hasUnderline() && runProp.underline() != anotherProp.underline())
                // TODO_RUST_CODE
            {
               // Java: runProp.deleteUnderline();
               // TODO_RUST_CODE
            }
         }
         // Java: else if (runProp.hasUnderline())
            // TODO_RUST_CODE
         {
            // Java: runProp.deleteUnderline();
            // TODO_RUST_CODE
         }
         // Java: runProp.save();
         // TODO_RUST_CODE
         // Java: return runProp;
         // TODO_RUST_CODE
      }
    }

    // Java: void __formatTextCharacterProperties(TextCharacterProperties runProp, TextRun textrun, int mode)
    pub fn __formatTextCharacterProperties(&self, runProp: TextCharacterProperties, textrun: TextRun, mode: i32) {
      // Java: switch (mode)
        // TODO_RUST_CODE
      {
         // Java: case 1:
         // TODO_RUST_CODE
            // Java: this.setTextCharacterProperties(runProp, textrun);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 2:
         // TODO_RUST_CODE
            // Java: this.clearTextCharacterProperties(runProp, textrun);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 8:
         // TODO_RUST_CODE
            // Java: this.brushTextCharacterProperties(runProp, textrun);
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: default:
         // TODO_RUST_CODE
            // Java: this.mergeTextCharacterProperties(runProp, textrun);
            // TODO_RUST_CODE
      }
    }

    // Java: void clearTextCharacterProperties(TextCharacterProperties runProp, TextRun textrun)
    pub fn clearTextCharacterProperties(&self, runProp: TextCharacterProperties, textrun: TextRun) {
      // Java: int mode = textrun.mode();
      // TODO_RUST_CODE
      // Java: if (0 == mode)
        // TODO_RUST_CODE
      {
         // Java: this.clearTextCharacterProperties(runProp, textrun.regularRun().rPr());
         // TODO_RUST_CODE
      }
      // Java: else if (2 == mode)
        // TODO_RUST_CODE
      {
         // Java: this.clearTextCharacterProperties(runProp, textrun.fieldRun().rPr());
         // TODO_RUST_CODE
      }
      // Java: else if (1 == mode)
        // TODO_RUST_CODE
      {
         // Java: this.clearTextCharacterProperties(runProp, textrun.linebreakRun().rPr());
         // TODO_RUST_CODE
      }
      // Java: textrun.save();
      // TODO_RUST_CODE
    }

    // Java: void clearTextCharacterProperties(TextCharacterProperties runProp, TextCharacterProperties formatProp)
    pub fn clearTextCharacterProperties(&self, runProp: TextCharacterProperties, formatProp: TextCharacterProperties) {
      // Java: if (runProp.hasHlinkClick())
        // TODO_RUST_CODE
      {
         // Java: formatProp.deleteHlinkClick();
         // TODO_RUST_CODE
      }
    }

    // Java: void __clearTextParagraphProperties(TextParagraphProperties paraProp, TextParagraph paragraph)
    pub fn __clearTextParagraphProperties(&self, paraProp: TextParagraphProperties, paragraph: TextParagraph) {
      // Java: if (paragraph.hasParaPr())
        // TODO_RUST_CODE
      {
         // Java: TextParagraphProperties newProp = TextParagraphProperties.open(paragraph.paraPr().save());
         // TODO_RUST_CODE
         // Java: if (paraProp.hasAlignment())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteAlignment();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasDefTabSize())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteDefTabSize();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.options().hasEaLnBrk())
            // TODO_RUST_CODE
         {
            // Java: newProp.options().deleteEaLnBrk();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasFontAlgn())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteFontAlgn();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.options().hasHangingPunct())
            // TODO_RUST_CODE
         {
            // Java: newProp.options().deleteHangingPunct();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasIndent())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteIndent();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.options().hasLatinLnBrk())
            // TODO_RUST_CODE
         {
            // Java: newProp.options().deleteLatinLnBrk();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasLvl())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteLvl();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasMarginLeft())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteMarginLeft();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasMarginRight())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteMarginRight();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.options().hasRightToLeft())
            // TODO_RUST_CODE
         {
            // Java: newProp.options().deleteRightToLeft();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasLnSpc())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteLnSpc();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasSpcBef())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteSpcBef();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasSpcAft())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteSpcAft();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasBuClr())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteBuClr();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasBuSzPct())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteBuSzPct();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasBuFont())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteBuFont();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasTabStopLst())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteTabStopLst();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasDefRPr())
            // TODO_RUST_CODE
         {
            // Java: newProp.deleteDefRPr();
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.options().hasBuNone() && paraProp.options().isBuNone())
            // TODO_RUST_CODE
         {
            // Java: if (newProp.hasBuChar())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuChar();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuFont())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuFont();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuBlip())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuBlip();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuSzPts())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuSzPts();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuSzPct())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuSzPct();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuAutonNum())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuAutonNum();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuClr())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuClr();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasOptions() && newProp.options().hasBuNone())
                // TODO_RUST_CODE
            {
               // Java: newProp.options().deleteBuNone();
               // TODO_RUST_CODE
            }
         }
         // Java: else if (paraProp.hasBuChar())
            // TODO_RUST_CODE
         {
            // Java: if (newProp.hasBuChar())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuChar();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuBlip())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuBlip();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuAutonNum())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuAutonNum();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasOptions() && newProp.options().hasBuNone())
                // TODO_RUST_CODE
            {
               // Java: newProp.options().deleteBuNone();
               // TODO_RUST_CODE
            }
         }
         // Java: else if (paraProp.hasBuBlip())
            // TODO_RUST_CODE
         {
            // Java: if (newProp.hasBuBlip())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuBlip();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuChar())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuChar();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuAutonNum())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuAutonNum();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasOptions() && newProp.options().hasBuNone())
                // TODO_RUST_CODE
            {
               // Java: newProp.options().deleteBuNone();
               // TODO_RUST_CODE
            }
         }
         // Java: else if (paraProp.hasBuAutonNum())
            // TODO_RUST_CODE
         {
            // Java: if (newProp.hasBuAutonNum())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuAutonNum();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuChar())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuChar();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuBlip())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuBlip();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.options().hasBuNone())
                // TODO_RUST_CODE
            {
               // Java: newProp.options().deleteBuNone();
               // TODO_RUST_CODE
            }
         }
         // Java: newProp.save();
         // TODO_RUST_CODE
         // Java: paragraph.forceWriteParaPr(newProp);
         // TODO_RUST_CODE
      }
    }

    // Java: void __formatTextParagraphProperties(TextParagraphProperties paraProp, TextParagraph paragraph)
    pub fn __formatTextParagraphProperties(&self, paraProp: TextParagraphProperties, paragraph: TextParagraph) {
      // Java: if (!paragraph.hasParaPr())
        // TODO_RUST_CODE
      {
         // Java: TextParagraphProperties newProp = TextParagraphProperties.open(paraProp.save());
         // TODO_RUST_CODE
         // Java: newProp.save();
         // TODO_RUST_CODE
         // Java: paragraph.forceWriteParaPr(newProp);
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: TextParagraphProperties newProp = TextParagraphProperties.open(paragraph.paraPr().save());
         // TODO_RUST_CODE
         // Java: if (paraProp.hasAlignment())
            // TODO_RUST_CODE
         {
            // Java: newProp.putAlignment(paraProp.alignment());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasDefTabSize())
            // TODO_RUST_CODE
         {
            // Java: newProp.putDefTabSize(paraProp.defTabSize());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.options().hasEaLnBrk())
            // TODO_RUST_CODE
         {
            // Java: newProp.options().putEaLnBrk(paraProp.options().isEaLnBrk());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasFontAlgn())
            // TODO_RUST_CODE
         {
            // Java: newProp.putFontAlgn(paraProp.fontAlgn());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.options().hasHangingPunct())
            // TODO_RUST_CODE
         {
            // Java: newProp.options().putHangingPunct(paraProp.options().isHangingPunct());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasIndent())
            // TODO_RUST_CODE
         {
            // Java: newProp.putIndent(paraProp.indent());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.options().hasLatinLnBrk())
            // TODO_RUST_CODE
         {
            // Java: newProp.options().putLatinLnBrk(paraProp.options().isLatinLnBrk());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasLvl())
            // TODO_RUST_CODE
         {
            // Java: newProp.putLvl(paraProp.lvl());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasMarginLeft())
            // TODO_RUST_CODE
         {
            // Java: newProp.putMarginLeft(paraProp.marginLeft());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasMarginRight())
            // TODO_RUST_CODE
         {
            // Java: newProp.putMarginRight(paraProp.marginRight());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.options().hasRightToLeft())
            // TODO_RUST_CODE
         {
            // Java: newProp.options().putRightToLeft(paraProp.options().isRightToLeft());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasLnSpc())
            // TODO_RUST_CODE
         {
            // Java: newProp.writeLnSpc(paraProp.lnSpc());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasSpcBef())
            // TODO_RUST_CODE
         {
            // Java: newProp.writeSpcBef(paraProp.spcBef());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasSpcAft())
            // TODO_RUST_CODE
         {
            // Java: newProp.writeSpcAft(paraProp.spcAft());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasBuClr())
            // TODO_RUST_CODE
         {
            // Java: newProp.writeBuClr(paraProp.buClr());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasBuSzPct())
            // TODO_RUST_CODE
         {
            // Java: newProp.putBuSzPct(paraProp.buSzPct());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasBuFont())
            // TODO_RUST_CODE
         {
            // Java: newProp.writeBuFont(paraProp.buFont());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasTabStopLst())
            // TODO_RUST_CODE
         {
            // Java: newProp.writeTabStopLst(paraProp.tabStopLst());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.hasDefRPr())
            // TODO_RUST_CODE
         {
            // Java: newProp.writeDefRPr(paraProp.defRPr());
            // TODO_RUST_CODE
         }
         // Java: if (paraProp.options().hasBuNone() && paraProp.options().isBuNone())
            // TODO_RUST_CODE
         {
            // Java: TextParagraphProperties.TextParaOptions paraOptions = TextParagraphProperties.TextParaOptions.open(newProp.options().save());
            // TODO_RUST_CODE
            // Java: if (newProp.hasBuChar())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuChar();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuFont())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuFont();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuBlip())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuBlip();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuSzPts())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuSzPts();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuSzPct())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuSzPct();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuAutonNum())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuAutonNum();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuClr())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuClr();
               // TODO_RUST_CODE
            }
            // Java: paraOptions.putBuNone(true);
            // TODO_RUST_CODE
            // Java: paraOptions.save();
            // TODO_RUST_CODE
            // Java: newProp.forceWriteOptions(paraOptions);
            // TODO_RUST_CODE
         }
         // Java: else if (paraProp.hasBuChar())
            // TODO_RUST_CODE
         {
            // Java: newProp.putBuChar(paraProp.buChar());
            // TODO_RUST_CODE
            // Java: if (newProp.hasBuBlip())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuBlip();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuAutonNum())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuAutonNum();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.options().hasBuNone())
                // TODO_RUST_CODE
            {
               // Java: newProp.options().deleteBuNone();
               // TODO_RUST_CODE
            }
         }
         // Java: else if (paraProp.hasBuBlip())
            // TODO_RUST_CODE
         {
            // Java: newProp.forceWriteBuBlip(Blip.open(paraProp.buBlip().save()));
            // TODO_RUST_CODE
            // Java: if (newProp.hasBuChar())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuChar();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuAutonNum())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuAutonNum();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.options().hasBuNone())
                // TODO_RUST_CODE
            {
               // Java: newProp.options().deleteBuNone();
               // TODO_RUST_CODE
            }
         }
         // Java: else if (paraProp.hasBuAutonNum())
            // TODO_RUST_CODE
         {
            // Java: newProp.forceWriteBuAutonNum(TextParagraphProperties.TextAutonumberBullet.open(paraProp.buAutonNum().save()));
            // TODO_RUST_CODE
            // Java: if (newProp.hasBuChar())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuChar();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.hasBuBlip())
                // TODO_RUST_CODE
            {
               // Java: newProp.deleteBuBlip();
               // TODO_RUST_CODE
            }
            // Java: if (newProp.options().hasBuNone())
                // TODO_RUST_CODE
            {
               // Java: newProp.options().deleteBuNone();
               // TODO_RUST_CODE
            }
         }
         // Java: newProp.save();
         // TODO_RUST_CODE
         // Java: paragraph.forceWriteParaPr(newProp);
         // TODO_RUST_CODE
      }
    }

    // Java: void mergeTextParagraphProperties(TextParagraphProperties paraProp, TextParagraphProperties formatProp)
    pub fn mergeTextParagraphProperties(&self, paraProp: TextParagraphProperties, formatProp: TextParagraphProperties) {
      // Java: if (paraProp.hasAlignment())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putAlignment(paraProp.alignment());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasDefTabSize())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putDefTabSize(paraProp.defTabSize());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.options().hasEaLnBrk())
        // TODO_RUST_CODE
      {
         // Java: formatProp.options().putEaLnBrk(paraProp.options().isEaLnBrk());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasFontAlgn())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putFontAlgn(paraProp.fontAlgn());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.options().hasHangingPunct())
        // TODO_RUST_CODE
      {
         // Java: formatProp.options().putHangingPunct(paraProp.options().isHangingPunct());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasIndent())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putIndent(paraProp.indent());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.options().hasLatinLnBrk())
        // TODO_RUST_CODE
      {
         // Java: formatProp.options().putLatinLnBrk(paraProp.options().isLatinLnBrk());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasLvl())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putLvl(paraProp.lvl());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasMarginLeft())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putMarginLeft(paraProp.marginLeft());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasMarginRight())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putMarginRight(paraProp.marginRight());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.options().hasRightToLeft())
        // TODO_RUST_CODE
      {
         // Java: formatProp.options().putRightToLeft(paraProp.options().isRightToLeft());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasLnSpc())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeLnSpc(paraProp.lnSpc());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasSpcBef())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeSpcBef(paraProp.spcBef());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasSpcAft())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeSpcAft(paraProp.spcAft());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasBuClr())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeBuClr(paraProp.buClr());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasBuSzPct())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putBuSzPct(paraProp.buSzPct());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasBuFont())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeBuFont(paraProp.buFont());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasTabStopLst())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeTabStopLst(paraProp.tabStopLst());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.hasDefRPr())
        // TODO_RUST_CODE
      {
         // Java: formatProp.writeDefRPr(paraProp.defRPr());
         // TODO_RUST_CODE
      }
      // Java: if (paraProp.options().hasBuNone() && paraProp.options().isBuNone())
        // TODO_RUST_CODE
      {
         // Java: TextParagraphProperties.TextParaOptions paraOptions = TextParagraphProperties.TextParaOptions.open(formatProp.options().save());
         // TODO_RUST_CODE
         // Java: if (formatProp.hasBuChar())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuChar();
            // TODO_RUST_CODE
         }
         // Java: if (formatProp.hasBuFont())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuFont();
            // TODO_RUST_CODE
         }
         // Java: if (formatProp.hasBuBlip())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuBlip();
            // TODO_RUST_CODE
         }
         // Java: if (formatProp.hasBuSzPts())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuSzPts();
            // TODO_RUST_CODE
         }
         // Java: if (formatProp.hasBuSzPct())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuSzPct();
            // TODO_RUST_CODE
         }
         // Java: if (formatProp.hasBuAutonNum())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuAutonNum();
            // TODO_RUST_CODE
         }
         // Java: if (formatProp.hasBuClr())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuClr();
            // TODO_RUST_CODE
         }
         // Java: paraOptions.putBuNone(true);
         // TODO_RUST_CODE
         // Java: paraOptions.save();
         // TODO_RUST_CODE
         // Java: formatProp.forceWriteOptions(paraOptions);
         // TODO_RUST_CODE
      }
      // Java: else if (paraProp.hasBuChar())
        // TODO_RUST_CODE
      {
         // Java: formatProp.putBuChar(paraProp.buChar());
         // TODO_RUST_CODE
         // Java: if (formatProp.hasBuBlip())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuBlip();
            // TODO_RUST_CODE
         }
         // Java: if (formatProp.hasBuAutonNum())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuAutonNum();
            // TODO_RUST_CODE
         }
         // Java: if (formatProp.options().hasBuNone())
            // TODO_RUST_CODE
         {
            // Java: formatProp.options().deleteBuNone();
            // TODO_RUST_CODE
         }
      }
      // Java: else if (paraProp.hasBuBlip())
        // TODO_RUST_CODE
      {
         // Java: formatProp.forceWriteBuBlip(Blip.open(paraProp.buBlip().save()));
         // TODO_RUST_CODE
         // Java: if (formatProp.hasBuChar())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuChar();
            // TODO_RUST_CODE
         }
         // Java: if (formatProp.hasBuAutonNum())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuAutonNum();
            // TODO_RUST_CODE
         }
         // Java: if (formatProp.options().hasBuNone())
            // TODO_RUST_CODE
         {
            // Java: formatProp.options().deleteBuNone();
            // TODO_RUST_CODE
         }
      }
      // Java: else if (paraProp.hasBuAutonNum())
        // TODO_RUST_CODE
      {
         // Java: formatProp.forceWriteBuAutonNum(TextParagraphProperties.TextAutonumberBullet.open(paraProp.buAutonNum().save()));
         // TODO_RUST_CODE
         // Java: if (formatProp.hasBuChar())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuChar();
            // TODO_RUST_CODE
         }
         // Java: if (formatProp.hasBuBlip())
            // TODO_RUST_CODE
         {
            // Java: formatProp.deleteBuBlip();
            // TODO_RUST_CODE
         }
         // Java: if (formatProp.options().hasBuNone())
            // TODO_RUST_CODE
         {
            // Java: formatProp.options().deleteBuNone();
            // TODO_RUST_CODE
         }
      }
    }

    // Java: TextParagraphProperties getParagraphProperties(int idxParagraph)
    pub fn getParagraphProperties(&self, idxParagraph: i32) -> TextParagraphProperties {
      // Java: TextParagraphProperties paraProp = TextParagraphProperties.create();
      // TODO_RUST_CODE
      // Java: List<TextParagraph> paragraphs = null;
      // TODO_RUST_CODE
      // Java: Object var4;
      // TODO_RUST_CODE
      // Java: if (0 < this.paragraphsCache.size())
        // TODO_RUST_CODE
      {
         // Java: var4 = this.paragraphsCache;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: var4 = new ArrayList();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read((Collection)var4);
         // TODO_RUST_CODE
      }
      // Java: if (0 <= idxParagraph && idxParagraph < ((List)var4).size())
        // TODO_RUST_CODE
      {
         // Java: paraProp.load(((TextParagraph)((List)var4).get(idxParagraph)).paraPr().save());
         // TODO_RUST_CODE
      }
      // Java: paraProp.save();
      // TODO_RUST_CODE
      // Java: return paraProp;
      // TODO_RUST_CODE
    }

    // Java: TextCharacterProperties getCharacterProperties(TextRun textrun)
    pub fn getCharacterProperties(&self, textrun: TextRun) -> TextCharacterProperties {
      // Java: TextCharacterProperties chProp = null;
      // TODO_RUST_CODE
      // Java: switch (textrun.mode())
        // TODO_RUST_CODE
      {
         // Java: case 0:
         // TODO_RUST_CODE
            // Java: chProp = TextCharacterProperties.open(textrun.regularRun().rPr().save());
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 1:
         // TODO_RUST_CODE
            // Java: chProp = TextCharacterProperties.open(textrun.linebreakRun().rPr().save());
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 2:
         // TODO_RUST_CODE
            // Java: chProp = TextCharacterProperties.open(textrun.fieldRun().rPr().save());
            // TODO_RUST_CODE
      }
      // Java: return chProp;
      // TODO_RUST_CODE
    }

    // Java: TextCharacterProperties getCharacterProperties(int idxPara, int idxRun)
    pub fn getCharacterProperties(&self, idxPara: i32, idxRun: i32) -> TextCharacterProperties {
      // Java: TextCharacterProperties runProp = null;
      // TODO_RUST_CODE
      // Java: List<TextParagraph> textparagraphs = null;
      // TODO_RUST_CODE
      // Java: if (0 < this.paragraphsCache.size())
        // TODO_RUST_CODE
      {
         // Java: textparagraphs = this.paragraphsCache;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: textparagraphs = new ArrayList();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(textparagraphs);
         // TODO_RUST_CODE
      }
      // Java: if (0 <= idxPara && idxPara < textparagraphs.size())
        // TODO_RUST_CODE
      {
         // Java: TextParagraph paragraph = null;
         // TODO_RUST_CODE
         // Java: paragraph = (TextParagraph)textparagraphs.get(idxPara);
         // TODO_RUST_CODE
         // Java: List<TextRun> runs = new ArrayList();
         // TODO_RUST_CODE
         // Java: paragraph.textruns().read(runs);
         // TODO_RUST_CODE
         // Java: if (0 <= idxRun && idxRun < runs.size())
            // TODO_RUST_CODE
         {
            // Java: runProp = this.getCharacterProperties((TextRun)runs.get(idxRun));
            // TODO_RUST_CODE
         }
         // Java: else if (runs.size() == idxRun)
            // TODO_RUST_CODE
         {
            // Java: if (paragraph.hasEndParaRPr())
                // TODO_RUST_CODE
            {
               // Java: runProp = TextCharacterProperties.open(paragraph.endParaRPr().save());
               // TODO_RUST_CODE
            }
            // Java: else if (0 < runs.size())
                // TODO_RUST_CODE
            {
               // Java: runProp = this.getCharacterProperties((TextRun)runs.get(runs.size() - 1));
               // TODO_RUST_CODE
            }
         }
      }
      // Java: return null == runProp ? TextCharacterProperties.create() : runProp;
      // TODO_RUST_CODE
    }

    // Java: String getParagraphText(TextParagraph paragraph)
    pub fn getParagraphText(&self, paragraph: TextParagraph) -> String {
      // Java: StringBuilder textBuilder = new StringBuilder("");
      // TODO_RUST_CODE
      // Java: List<TextRun> textruns = new ArrayList();
      // TODO_RUST_CODE
      // Java: paragraph.textruns().read(textruns);
      // TODO_RUST_CODE
      // Java: for(int i = 0; i < textruns.size(); ++i)
        // TODO_RUST_CODE
      {
         // Java: String text = this.getRunText((TextRun)textruns.get(i));
         // TODO_RUST_CODE
         // Java: textBuilder.append(text);
         // TODO_RUST_CODE
      }
      // Java: textBuilder.append("\r");
      // TODO_RUST_CODE
      // Java: return textBuilder.toString();
      // TODO_RUST_CODE
    }

    // Java: String getParagraphText(int idxParagraph)
    pub fn getParagraphText(&self, idxParagraph: i32) -> String {
      // Java: StringBuilder textBuilder = new StringBuilder("");
      // TODO_RUST_CODE
      // Java: List<TextParagraph> textparagraphs = this.paragraphsCache;
      // TODO_RUST_CODE
      // Java: if (0 == textparagraphs.size())
        // TODO_RUST_CODE
      {
         // Java: textparagraphs = new ArrayList();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(textparagraphs);
         // TODO_RUST_CODE
      }
      // Java: if (0 <= idxParagraph && idxParagraph < textparagraphs.size())
        // TODO_RUST_CODE
      {
         // Java: TextParagraph paragraph = (TextParagraph)textparagraphs.get(idxParagraph);
         // TODO_RUST_CODE
         // Java: textBuilder.append(this.getParagraphText(paragraph));
         // TODO_RUST_CODE
      }
      // Java: return textBuilder.toString();
      // TODO_RUST_CODE
    }

    // Java: String getRunText(TextRun txRun)
    pub fn getRunText(&self, txRun: TextRun) -> String {
      // Java: StringBuilder strRunText = new StringBuilder("");
      // TODO_RUST_CODE
      // Java: switch (txRun.mode())
        // TODO_RUST_CODE
      {
         // Java: case 0:
         // TODO_RUST_CODE
            // Java: strRunText.append(txRun.regularRun().text());
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 1:
         // TODO_RUST_CODE
            // Java: strRunText.append("\u000b");
            // TODO_RUST_CODE
            // Java: break;
            // TODO_RUST_CODE
         // Java: case 2:
         // TODO_RUST_CODE
            // Java: strRunText.append(txRun.fieldRun().text());
            // TODO_RUST_CODE
      }
      // Java: return strRunText.toString();
      // TODO_RUST_CODE
    }

    // Java: String getRunText(int idxParagraph, int idxRun)
    pub fn getRunText(&self, idxParagraph: i32, idxRun: i32) -> String {
      // Java: StringBuilder textBuilder = new StringBuilder("");
      // TODO_RUST_CODE
      // Java: List<TextRun> textruns = this.getRuns(idxParagraph);
      // TODO_RUST_CODE
      // Java: if (0 <= idxRun && idxRun < textruns.size())
        // TODO_RUST_CODE
      {
         // Java: String text = this.getRunText((TextRun)textruns.get(idxRun));
         // TODO_RUST_CODE
         // Java: if (0 < text.length())
            // TODO_RUST_CODE
         {
            // Java: textBuilder.append(text);
            // TODO_RUST_CODE
         }
      }
      // Java: else if (textruns.size() == idxRun)
        // TODO_RUST_CODE
      {
         // Java: textBuilder.append("\r");
         // TODO_RUST_CODE
      }
      // Java: return textBuilder.toString();
      // TODO_RUST_CODE
    }

    // Java: boolean insert(int cp, TextParagraph.TextRunList lstRuns)
    pub fn insert_runs(&self, cp: i32, lstRuns: TextParagraph::TextRunList) -> bool {
      // Java: if (null == lstRuns)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: List<TextRun> textrunsInsert = new ArrayList();
         // TODO_RUST_CODE
         // Java: lstRuns.read(textrunsInsert);
         // TODO_RUST_CODE
         // Java: if (textrunsInsert.size() <= 0)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else if (!this.__seek(cp, false))
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: if (0 == this.paragraphsCache.size())
                // TODO_RUST_CODE
            {
               // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(this.paragraphsCache);
               // TODO_RUST_CODE
            }
            // Java: List<TextParagraph> textparagraphs = this.paragraphsCache;
            // TODO_RUST_CODE
            // Java: List<TextRun> textruns = new ArrayList();
            // TODO_RUST_CODE
            // Java: TextParagraph paragraph = TextParagraph.create();
            // TODO_RUST_CODE
            // Java: if (0 <= this.idxParagraph && this.idxParagraph < textparagraphs.size())
                // TODO_RUST_CODE
            {
               // Java: paragraph = (TextParagraph)textparagraphs.get(this.idxParagraph);
               // TODO_RUST_CODE
            }
            // Java: paragraph.textruns().read(textruns);
            // TODO_RUST_CODE
            // Java: if (0 <= this.idxRun && this.idxRun < textruns.size())
                // TODO_RUST_CODE
            {
               // Java: if (this.startRun != cp)
                    // TODO_RUST_CODE
               {
                  // Java: TextRun textrun = (TextRun)textruns.get(this.idxRun);
                  // TODO_RUST_CODE
                  // Java: StringBuilder textBuilder = new StringBuilder("");
                  // TODO_RUST_CODE
                  // Java: textBuilder.append(this.getRunText(textrun));
                  // TODO_RUST_CODE
                  // Java: this.setRunText(textrun, textBuilder.substring(0, cp - this.startRun));
                  // TODO_RUST_CODE
                  // Java: TextRun newTextRun = TextRun.open(textrun.save());
                  // TODO_RUST_CODE
                  // Java: this.setRunText(newTextRun, textBuilder.substring(cp - this.startRun, textBuilder.length()));
                  // TODO_RUST_CODE
                  // Java: textruns.add(++this.idxRun, newTextRun);
                  // TODO_RUST_CODE
               }
               // Java: for(int i = 0; i < textrunsInsert.size(); ++i)
                    // TODO_RUST_CODE
               {
                  // Java: textruns.add(this.idxRun++, (TextRun)textrunsInsert.get(i));
                  // TODO_RUST_CODE
               }
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: for(int i = 0; i < textrunsInsert.size(); ++i)
                    // TODO_RUST_CODE
               {
                  // Java: textruns.add(this.idxRun++, (TextRun)textrunsInsert.get(i));
                  // TODO_RUST_CODE
               }
            }
            // Java: paragraph.textruns().write(textruns);
            // TODO_RUST_CODE
            // Java: if (!paragraph.hasEndParaRPr())
                // TODO_RUST_CODE
            {
               // Java: paragraph.writeEndParaRPr(this.defaultEndParaRPr());
               // TODO_RUST_CODE
            }
            // Java: paragraph.save();
            // TODO_RUST_CODE
            // Java: this.beforeWrite();
            // TODO_RUST_CODE
            // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(textparagraphs);
            // TODO_RUST_CODE
            // Java: this.onDataChangeAfter(true);
            // TODO_RUST_CODE
            // Java: return true;
            // TODO_RUST_CODE
         }
      }
    }

    // Java: boolean insert(int cp, TextBody.ParagraphList parasInsert)
    pub fn insert_paras(&self, cp: i32, parasInsert: TextBody::ParagraphList) -> bool {
      // Java: if (cp >= 0 && null != parasInsert)
        // TODO_RUST_CODE
      {
         // Java: List<TextParagraph> paragraphsInsert = new ArrayList();
         // TODO_RUST_CODE
         // Java: parasInsert.read(paragraphsInsert);
         // TODO_RUST_CODE
         // Java: if (paragraphsInsert.size() <= 0)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else if (!this.__seek(cp, false))
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: if (0 == this.paragraphsCache.size())
                // TODO_RUST_CODE
            {
               // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(this.paragraphsCache);
               // TODO_RUST_CODE
            }
            // Java: List<TextParagraph> paragraphs = this.paragraphsCache;
            // TODO_RUST_CODE
            // Java: if (0 <= this.idxParagraph && this.idxParagraph < paragraphs.size())
                // TODO_RUST_CODE
            {
               // Java: if (this.startPara == cp)
                    // TODO_RUST_CODE
               {
                  // Java: assert this.idxParagraph < paragraphs.size();
                  // TODO_RUST_CODE
                  // Java: for(int idx = 0; idx < paragraphsInsert.size(); ++idx)
                        // TODO_RUST_CODE
                  {
                     // Java: paragraphs.add(this.idxParagraph++, (TextParagraph)paragraphsInsert.get(idx));
                     // TODO_RUST_CODE
                  }
               }
               // Java: else
                    // TODO_RUST_CODE
               {
                  // Java: List<TextRun> textruns = new ArrayList();
                  // TODO_RUST_CODE
                  // Java: TextParagraph paragraph = (TextParagraph)paragraphs.get(this.idxParagraph);
                  // TODO_RUST_CODE
                  // Java: paragraph.textruns().read(textruns);
                  // TODO_RUST_CODE
                  // Java: if (this.startRun != cp)
                        // TODO_RUST_CODE
                  {
                     // Java: TextRun textrunBef = (TextRun)textruns.get(this.idxRun);
                     // TODO_RUST_CODE
                     // Java: StringBuilder textBuilder = new StringBuilder("");
                     // TODO_RUST_CODE
                     // Java: textBuilder.append(this.getRunText(textrunBef));
                     // TODO_RUST_CODE
                     // Java: String textBef = textBuilder.substring(0, cp - this.startRun);
                     // TODO_RUST_CODE
                     // Java: String textAft = textBuilder.substring(cp - this.startRun, textBuilder.length());
                     // TODO_RUST_CODE
                     // Java: if (0 < textBef.length())
                            // TODO_RUST_CODE
                     {
                        // Java: this.setRunText(textrunBef, textBef);
                        // TODO_RUST_CODE
                     }
                     // Java: if (0 < textAft.length())
                            // TODO_RUST_CODE
                     {
                        // Java: TextRun textrunAft = TextRun.open(textrunBef.save());
                        // TODO_RUST_CODE
                        // Java: this.setRunText(textrunAft, textAft);
                        // TODO_RUST_CODE
                        // Java: textruns.add(++this.idxRun, textrunAft);
                        // TODO_RUST_CODE
                     }
                  }
                  // Java: TextParagraph paragraphAft = TextParagraph.open(paragraph.save());
                  // TODO_RUST_CODE
                  // Java: TextParagraph paragraphBef = TextParagraph.open(((TextParagraph)paragraphsInsert.get(0)).save());
                  // TODO_RUST_CODE
                  // Java: List<TextRun> textrunsbefore = new ArrayList();
                  // TODO_RUST_CODE
                  // Java: paragraphBef.textruns().read(textrunsbefore);
                  // TODO_RUST_CODE
                  // Java: for(int i = this.idxRun - 1; 0 <= i; --i)
                        // TODO_RUST_CODE
                  {
                     // Java: textrunsbefore.add(0, (TextRun)textruns.get(i));
                     // TODO_RUST_CODE
                  }
                  // Java: paragraphBef.textruns().write(textrunsbefore);
                  // TODO_RUST_CODE
                  // Java: if (!paragraphBef.hasEndParaRPr())
                        // TODO_RUST_CODE
                  {
                     // Java: paragraphBef.writeEndParaRPr(this.defaultEndParaRPr());
                     // TODO_RUST_CODE
                  }
                  // Java: paragraphBef.save();
                  // TODO_RUST_CODE
                  // Java: paragraphs.remove(this.idxParagraph);
                  // TODO_RUST_CODE
                  // Java: paragraphs.add(this.idxParagraph, paragraphBef);
                  // TODO_RUST_CODE
                  // Java: for(int idx = 1; idx < paragraphsInsert.size(); ++idx)
                        // TODO_RUST_CODE
                  {
                     // Java: paragraphs.add(++this.idxParagraph, (TextParagraph)paragraphsInsert.get(idx));
                     // TODO_RUST_CODE
                  }
                  // Java: List<TextRun> textrunsAfter = new ArrayList();
                  // TODO_RUST_CODE
                  // Java: for(int i = this.idxRun; i < textruns.size(); ++i)
                        // TODO_RUST_CODE
                  {
                     // Java: textrunsAfter.add((TextRun)textruns.get(i));
                     // TODO_RUST_CODE
                  }
                  // Java: paragraphAft.textruns().write(textrunsAfter);
                  // TODO_RUST_CODE
                  // Java: if (!paragraphAft.hasEndParaRPr())
                        // TODO_RUST_CODE
                  {
                     // Java: paragraphAft.writeEndParaRPr(this.defaultEndParaRPr());
                     // TODO_RUST_CODE
                  }
                  // Java: paragraphAft.save();
                  // TODO_RUST_CODE
                  // Java: paragraphs.add(++this.idxParagraph, paragraphAft);
                  // TODO_RUST_CODE
               }
            }
            // Java: else
                // TODO_RUST_CODE
            {
               // Java: paragraphs = paragraphsInsert;
               // TODO_RUST_CODE
            }
            // Java: this.beforeWrite();
            // TODO_RUST_CODE
            // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(paragraphs);
            // TODO_RUST_CODE
            // Java: this.onDataChangeAfter(true);
            // TODO_RUST_CODE
            // Java: return true;
            // TODO_RUST_CODE
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean isEqual(KoBin src, KoBin another)
    pub fn isEqual(&self, src: KoBin, another: KoBin) -> bool {
      // Java: if (null != src && null != another && src.size() == another.size())
        // TODO_RUST_CODE
      {
         // Java: if (src.size() == 0)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: int size = src.size();
            // TODO_RUST_CODE
            // Java: byte[] srcByte = new byte[size];
            // TODO_RUST_CODE
            // Java: src.read(srcByte, 0);
            // TODO_RUST_CODE
            // Java: byte[] anotherByte = new byte[size];
            // TODO_RUST_CODE
            // Java: another.read(anotherByte, 0);
            // TODO_RUST_CODE
            // Java: return Arrays.equals(srcByte, anotherByte);
            // TODO_RUST_CODE
         }
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
    }

    // Java: boolean onDataChangeBefore()
    pub fn onDataChangeBefore(&self) -> bool {
      // Java: return true;
      // TODO_RUST_CODE
    }

    // Java: boolean onDataChangeAfter(boolean isClearCache)
    pub fn onDataChangeAfter(&self, isClearCache: bool) -> bool {
      // Java: this.clearCache();
      // TODO_RUST_CODE
      // Java: return true;
      // TODO_RUST_CODE
    }

    // Java: void clearRunCache()
    pub fn clearRunCache(&mut self) {
      // Java: this.setRunCache((TextRun)null);
      self.setRunCache(None);
      // Java: this.runsCache.clear();
      self.runsCache.clear();
    }

    // Java: void clearParagraphCache()
    pub fn clearParagraphCache(&mut self) {
      // Java: this.setParagraphCache((TextParagraph)null);
      self.setParagraphCache(None);
      // Java: this.paragraphsCache.clear();
      self.paragraphsCache.clear();
    }

    // Java: void meansureCache()
    pub fn meansureCache(&mut self) {
      // Java: if (0 == this.paragraphsCache.size())
      if 0 == self.paragraphsCache.len() {
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(this.paragraphsCache);
         self.data().txBody.paraLst().read(&mut self.paragraphsCache);
         // Java: this.setParagraphCache((TextParagraph)null);
         self.setParagraphCache(None);
         // Java: this.clearRunCache();
         self.clearRunCache();
      }
      // Java: if (null == this.getParagraphCache() && 0 <= this.idxParagraph && this.idxParagraph < this.paragraphsCache.size())
      if self.getParagraphCache().is_none() && 0 <= self.idxParagraph && (self.idxParagraph as usize) < self.paragraphsCache.len() {
         // Java: this.setParagraphCache((TextParagraph)this.paragraphsCache.get(this.idxParagraph));
         let p = self.paragraphsCache[self.idxParagraph as usize].clone();
         self.setParagraphCache(Some(p));
         // Java: this.clearRunCache();
         self.clearRunCache();
      }
      // Java: TextParagraph paragraph = this.getParagraphCache();
      let paragraph = self.getParagraphCache();
      // Java: if (0 == this.runsCache.size() && null != paragraph)
      if 0 == self.runsCache.len() && paragraph.is_some() {
         // Java: paragraph.textruns().read(this.runsCache);
         paragraph.as_ref().unwrap().textruns().read(&mut self.runsCache);
         // Java: this.setRunCache((TextRun)null);
         self.setRunCache(None);
      }
      // Java: TextRun run = this.getRunCache();
      let run = self.getRunCache();
      // Java: if (null == run && 0 <= this.idxRun && this.idxRun < this.runsCache.size())
      if run.is_none() && 0 <= self.idxRun && (self.idxRun as usize) < self.runsCache.len() {
         // Java: this.setRunCache((TextRun)this.runsCache.get(this.idxRun));
         let r = self.runsCache[self.idxRun as usize].clone();
         self.setRunCache(Some(r));
      }
    }

    // Java: boolean resetFieldTextRuns(int fieldType, double time, int slideIndex)
    pub fn resetFieldTextRuns(&self, fieldType: i32, time: f64, slideIndex: i32) -> bool {
      // Java: boolean findedField = false;
      // TODO_RUST_CODE
      // Java: if (0 == this.paragraphsCache.size())
        // TODO_RUST_CODE
      {
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().read(this.paragraphsCache);
         // TODO_RUST_CODE
      }
      // Java: List<TextParagraph> paragraphs = this.paragraphsCache;
      // TODO_RUST_CODE
      // Java: int i = 0;
      // TODO_RUST_CODE
      // Java: for(int pCount = paragraphs.size(); i < pCount; ++i)
        // TODO_RUST_CODE
      {
         // Java: TextParagraph paragraph = (TextParagraph)paragraphs.get(i);
         // TODO_RUST_CODE
         // Java: List<TextRun> runs = new ArrayList();
         // TODO_RUST_CODE
         // Java: paragraph.textruns().read(runs);
         // TODO_RUST_CODE
         // Java: boolean runChanged = false;
         // TODO_RUST_CODE
         // Java: int j = 0;
         // TODO_RUST_CODE
         // Java: for(int runCount = runs.size(); j < runCount; ++j)
            // TODO_RUST_CODE
         {
            // Java: TextRun run = (TextRun)runs.get(j);
            // TODO_RUST_CODE
            // Java: if (run.mode() == 2)
                // TODO_RUST_CODE
            {
               // Java: String text = null;
               // TODO_RUST_CODE
               // Java: if (fieldType == 0)
                    // TODO_RUST_CODE
               {
                  // Java: TextCharacterProperties rpr = run.fieldRun().rPr();
                  // TODO_RUST_CODE
                  // Java: String lang = rpr.hasLang() ? rpr.lang() : rpr.altLang();
                  // TODO_RUST_CODE
                  // Java: text = FieldTypeUtil.parseDateString(time, lang, run.fieldRun().type());
                  // TODO_RUST_CODE
               }
               // Java: else if (fieldType == 1)
                    // TODO_RUST_CODE
               {
                  // Java: if (run.fieldRun().type().equals("slidenum"))
                        // TODO_RUST_CODE
                  {
                     // Java: text = Integer.toString(slideIndex);
                     // TODO_RUST_CODE
                  }
               }
               // Java: else if (fieldType == 2)
                    // TODO_RUST_CODE
               {
                  // Java: if (run.fieldRun().type().equals("slidenum"))
                        // TODO_RUST_CODE
                  {
                     // Java: text = Integer.toString(slideIndex);
                     // TODO_RUST_CODE
                  }
                  // Java: else
                        // TODO_RUST_CODE
                  {
                     // Java: TextCharacterProperties rpr = run.fieldRun().rPr();
                     // TODO_RUST_CODE
                     // Java: String lang = rpr.hasLang() ? rpr.lang() : rpr.altLang();
                     // TODO_RUST_CODE
                     // Java: text = FieldTypeUtil.parseDateString(time, lang, run.fieldRun().type());
                     // TODO_RUST_CODE
                  }
               }
               // Java: if (text != null && !text.equals(run.fieldRun().text()))
                    // TODO_RUST_CODE
               {
                  // Java: run.fieldRun().putText(text);
                  // TODO_RUST_CODE
                  // Java: run.putLength(text.length());
                  // TODO_RUST_CODE
                  // Java: run.save();
                  // TODO_RUST_CODE
                  // Java: findedField = true;
                  // TODO_RUST_CODE
                  // Java: runChanged = true;
                  // TODO_RUST_CODE
               }
            }
         }
         // Java: if (runChanged)
            // TODO_RUST_CODE
         {
            // Java: paragraph.textruns().write(runs);
            // TODO_RUST_CODE
            // Java: paragraph.save();
            // TODO_RUST_CODE
         }
      }
      // Java: if (findedField)
        // TODO_RUST_CODE
      {
         // Java: this.beforeWrite();
         // TODO_RUST_CODE
         // Java: ((KmoTextBodyAtomData)this.rawData()).txBody.paraLst().forceWrite(paragraphs);
         // TODO_RUST_CODE
         // Java: this.onDataChangeAfter(true);
         // TODO_RUST_CODE
      }
      // Java: return findedField;
      // TODO_RUST_CODE
    }

    // Java: KmoTextBodyAtomData data()
    // LLM Rust Renamed: data() moved to top of impl block as helper

    // Java: TextCharacterProperties defaultEndParaRPr()
    pub fn defaultEndParaRPr(&self) -> TextCharacterProperties {
      // Java: TextCharacterProperties defaultEndParaRPr = TextCharacterProperties.create();
      let mut defaultEndParaRPr = TextCharacterProperties::create();
      // Java: defaultEndParaRPr.putLang("zh-CN");
      defaultEndParaRPr.putLang("zh-CN");
      // Java: defaultEndParaRPr.putAltLang("en-US");
      defaultEndParaRPr.putAltLang("en-US");
      // Java: defaultEndParaRPr.save();
      defaultEndParaRPr.save();
      // Java: return defaultEndParaRPr;
      defaultEndParaRPr
    }

    // Java: TextRun getRunCache()
    pub fn getRunCache(&self) -> Option<TextRun> {
      // Java: return (TextRun)this.runCacheThreadLocal.get();
      self.runCache.clone()
    }

    // Java: void setRunCache(TextRun run)
    pub fn setRunCache(&mut self, run: Option<TextRun>) {
      // Java: if (null == run)
      if run.is_none() {
         // Java: this.runCacheThreadLocal.remove();
         self.runCache = None;
      }
      // Java: else
      else {
         // Java: this.runCacheThreadLocal.set(run);
         self.runCache = run;
      }
    }

    // Java: TextParagraph getParagraphCache()
    pub fn getParagraphCache(&self) -> Option<TextParagraph> {
      // Java: return (TextParagraph)this.paragraphCacheThreadLocal.get();
      self.paragraphCache.clone()
    }

    // Java: void setParagraphCache(TextParagraph paragraph)
    pub fn setParagraphCache(&mut self, paragraph: Option<TextParagraph>) {
      // Java: if (null == paragraph)
      if paragraph.is_none() {
         // Java: this.paragraphCacheThreadLocal.remove();
         self.paragraphCache = None;
      }
      // Java: else
      else {
         // Java: this.paragraphCacheThreadLocal.set(paragraph);
         self.paragraphCache = paragraph;
      }
    }

    // Java: List<TextParagraph> getTextParagraphList()
    pub fn getTextParagraphList(&self) -> Vec<TextParagraph> {
      // Java: List<TextParagraph> list = new ArrayList();
      let mut list: Vec<TextParagraph> = Vec::new();
      // Java: this.paragraphList().read(list);
      self.paragraphList().read(&mut list);
      // Java: return list;
      list
    }

}

impl KmoProtocol for KmoTextBody {
}
