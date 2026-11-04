// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.core.runtime.Platform
use util_adapter::cn::wps::core::runtime::Platform::*;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::File::*;
// Java: import cn.wps.moffice.kfs.FileInputStream
use kfs_kfs_app::cn::wps::moffice::kfs::FileInputStream::*;
// Java: import cn.wps.moffice.util.FileUtil
use util::moffice::util::FileUtil::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.function.card.CardPaper
use crate::cn::wps::moffice::writer::function::card::CardPaper::*;
// Java: import cn.wps.moffice.writer.function.card.online.KeyWords
use crate::cn::wps::moffice::writer::function::card::online::KeyWords::*;
// Java: import cn.wps.moffice.writer.function.card.online.PaperKeyWordParam
use crate::cn::wps::moffice::writer::function::card::online::PaperKeyWordParam::*;
// Java: import cn.wps.util.concurrent.locks.RangeReadLock
use crate::cn::wps::util::concurrent::locks::RangeReadLock::*;
// Java: import java.io.BufferedReader
use stubs::java::io::BufferedReader::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;
// Java: import java.io.InputStream
use stubs::java::io::InputStream::*;
// Java: import java.io.InputStreamReader
use stubs::java::io::InputStreamReader::*;
// Java: import java.nio.charset.StandardCharsets
use stubs::java::nio::charset::StandardCharsets::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Map
// Java: import java.util.concurrent.atomic.AtomicInteger
use stubs::java::util::concurrent::atomic::AtomicInteger::*;
// Java: import java.util.regex.Matcher
use stubs::java::util::regex::Matcher::*;
// Java: import java.util.regex.Pattern
use stubs::java::util::regex::Pattern::*;
// Java: import java.lang.CharSequence
use stubs::java::lang::CharSequence::*;
use std::collections::{HashMap};

// java static final String ASSERT_PATH;
pub(crate) const ASSERT_PATH: &str = "";

// java public class PaperRecognition
#[derive(Default)]
pub struct PaperRecognition {
    // java private List<String> mP0Keys = new ArrayList();
    mP0Keys: Option<Vec<String>>,
    // java private List<String> mP1Keys = new ArrayList();
    mP1Keys: Option<Vec<String>>,
    // java private List<String> mDenys = new ArrayList();
    mDenys: Option<Vec<String>>,
    // java private Map<String, LazzyPattern> mDenyExcludePatternMap = new HashMap();
    mDenyExcludePatternMap: Option<HashMap<String, LazzyPattern>>,
    // java private LazzyPattern mP0KeyPattern;
    mP0KeyPattern: Option<LazzyPattern>,
    // java private LazzyPattern mP1KeyPattern;
    mP1KeyPattern: Option<LazzyPattern>,
    // java private LazzyPattern mDenyPattern;
    mDenyPattern: Option<LazzyPattern>,
    // java private String mMatcherTip;
    mMatcherTip: Option<String>,
    // java private boolean mLoadPatternOk = false;
    mLoadPatternOk: bool,
    // java MatchResult mCacheRetult;
    pub(crate) mCacheRetult: Option<MatchResult>,
}

// java public static enum MatchResult
#[derive(Copy, Clone, Debug)]
pub enum MatchResult {
}

// java public static class LazzyPattern
#[derive(Default)]
pub struct LazzyPattern {
    // java volatile Pattern mPattern;
    pub(crate) mPattern: Option<Pattern>,
    // java List<String> mKeys;
    pub(crate) mKeys: Option<Vec<String>>,
}

// java public class PaperRecognition
impl PaperRecognition {
    // java @Deprecated public  PaperRecognition()
    pub fn new() -> Self {
      // java this.loadPattern();
      this.loadPattern();
    }

    // java public  PaperRecognition(PaperKeyWordParam paperKeyWordParam)
    pub fn new__paperkeywordparam(paperKeyWordParam: &PaperKeyWordParam) -> Self {
      // java this.init(paperKeyWordParam);
      this.init(Some(&paperKeyWordParam));
    }

    // java public void init(PaperKeyWordParam paperKeyWordParam)
    pub fn init(&mut self, paperKeyWordParam: Option<&PaperKeyWordParam>) {
      // java if (null != paperKeyWordParam)
      if let Some(paperKeyWordParam) = paperKeyWordParam {
        // java this.mP0Keys = paperKeyWordParam.getP0Keys();
        self.mP0Keys = paperKeyWordParam.getP0Keys();
        // java this.mP1Keys = paperKeyWordParam.getP1Keys();
        self.mP1Keys = paperKeyWordParam.getP1Keys();
        // java this.mDenys = paperKeyWordParam.getDenys();
        self.mDenys = paperKeyWordParam.getDenys();
        // java for(KeyWords denyKey : paperKeyWordParam.getDenyExcludeKeys())
        for denyKey in paperKeyWordParam.getDenyExcludeKeys().iter() {
          // java if (this.mDenys.contains(denyKey.getKey()))
          if self.mDenys.contains(denyKey.getKey()) {
            // java this.mDenyExcludePatternMap.put(denyKey.getKey(), new LazzyPattern(denyKey.getWords()));
            self.mDenyExcludePatternMap.put(denyKey.getKey(), LazzyPattern::new(denyKey.getWords()));
          }
        }
        // java if (!this.mP0Keys.isEmpty())
        if !self.mP0Keys.is_empty() {
          // java this.mLoadPatternOk = true;
          self.mLoadPatternOk = true;
          // java this.mP0KeyPattern = new LazzyPattern(this.mP0Keys);
          self.mP0KeyPattern = LazzyPattern::new(self.mP0Keys);
          // java this.mP1KeyPattern = new LazzyPattern(this.mP1Keys);
          self.mP1KeyPattern = LazzyPattern::new(self.mP1Keys);
          // java this.mDenyPattern = new LazzyPattern(this.mDenys);
          self.mDenyPattern = LazzyPattern::new(self.mDenys);
        }
      }
    }

    // java public MatchResult checkTestPaper(String path)
    pub fn checkTestPaper__string(&mut self, path: &str) -> Option<MatchResult> {
      // java return this.checkTestPaper(path, (TextDocument)null);
      return self.checkTestPaper__string_textdocument(Some(&path), None);
    }

    // java public MatchResult checkTestPaper(String path, TextDocument textDocument)
    pub fn checkTestPaper__string_textdocument(&mut self, path: Option<&str>, textDocument: Option<&TextDocument>) -> MatchResult {
      // java if (this.mLoadPatternOk && path != null && path.length() != 0)
      if let Some(path) = path {
        if self.mLoadPatternOk && (path.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) != 0 {
          // java String fileName = FileUtil.getFileName(path);
          let mut fileName: String = FileUtil::getFileName(Some(&path));
          // java String deny = this.findDeny(fileName);
          let mut deny: String = self.findDeny(&fileName);
          // java if (deny != null)
          if deny.is_some() {
            // java this.setMatcherTip("have deny,d:" + deny, path);
            self.setMatcherTip(&"have deny,d:" + deny, &path);
            // java return PaperRecognition.MatchResult.hasDeny;
            return PaperRecognition::MatchResult.hasDeny;
          }
          // java else
          else {
            // java String p0key = this.mP0KeyPattern.firstMatcherString(fileName);
            let mut p0key: String = self.mP0KeyPattern.firstMatcherString(fileName);
            // java if (p0key != null)
            if p0key.is_some() {
              // java if (textDocument != null && !this.checkPaperSplit(textDocument))
              if let Some(textDocument) = textDocument {
                if !self.checkPaperSplit(&textDocument) {
                  // java this.setMatcherTip("has p0 key but splitPaper failed!,k:" + p0key, path);
                  self.setMatcherTip(&"has p0 key but splitPaper failed!,k:" + p0key, &path);
                  // java return PaperRecognition.MatchResult.splitFailed;
                  return PaperRecognition::MatchResult.splitFailed;
                }
              }
              // java else
              else {
                // java this.setMatcherTip("has p0 key,k:" + p0key, path);
                self.setMatcherTip(&"has p0 key,k:" + p0key, &path);
                // java return PaperRecognition.MatchResult.p0Match;
                return PaperRecognition::MatchResult.p0Match;
              }
            }
            // java else
            else {
              // java Matcher p1Match = this.mP1KeyPattern.matcher(fileName);
              let mut p1Match: Matcher = self.mP1KeyPattern.matcher(&fileName);
              // java if (p1Match.find())
              if p1Match.find() {
                // java StringBuilder p1key = new StringBuilder();
                let mut p1key: String = StringBuilder::new();
                // java p1key.append(p1Match.group());
                p1key.append(p1Match.group());
                // java if (p1Match.find())
                if p1Match.find() {
                  // java p1key.append(',').append(p1Match.group());
                  p1key.append(',').append(p1Match.group());
                  // java if (textDocument != null && !this.checkPaperSplit(textDocument))
                  if let Some(textDocument) = textDocument {
                    if !self.checkPaperSplit(&textDocument) {
                      // java this.setMatcherTip("has p1 keys but split paper failed:" + p1key, path);
                      self.setMatcherTip(&"has p1 keys but split paper failed:" + p1key, &path);
                      // java return PaperRecognition.MatchResult.splitFailed;
                      return PaperRecognition::MatchResult.splitFailed;
                    }
                  }
                  // java else
                  else {
                    // java this.setMatcherTip("has p1 keys:" + p1key, path);
                    self.setMatcherTip(&"has p1 keys:" + p1key, &path);
                    // java return PaperRecognition.MatchResult.p1Match;
                    return PaperRecognition::MatchResult.p1Match;
                  }
                }
                // java else
                else {
                  // java if (textDocument != null)
                  if let Some(textDocument) = textDocument {
                    // java MatchResult fromContent = this.checkMoreKeyFromContent(textDocument, path);
                    let mut fromContent: MatchResult = self.checkMoreKeyFromContent(&textDocument, &path);
                    // java if (fromContent.isPaper() && this.checkPaperSplit(textDocument))
                    if fromContent.isPaper() && self.checkPaperSplit(&textDocument) {
                      // java return fromContent;
                      return fromContent;
                    }
                  }
                  // java this.setMatcherTip("no enough p1 keys:" + p1key, path);
                  self.setMatcherTip(&"no enough p1 keys:" + p1key, &path);
                  // java return PaperRecognition.MatchResult.p1NotEnough;
                  return PaperRecognition::MatchResult.p1NotEnough;
                }
              }
              // java else
              else {
                // java this.setMatcherTip("no p0 or p1 key", path);
                self.setMatcherTip(&"no p0 or p1 key", &path);
                // java return PaperRecognition.MatchResult.noKey;
                return PaperRecognition::MatchResult.noKey;
              }
            }
          }
        }
      }
      // java else
      else {
        // java this.setMatcherTip("error mLoadPatternOk:" + this.mLoadPatternOk, path);
        self.setMatcherTip(&"error mLoadPatternOk:" + self.mLoadPatternOk, &path);
        // java return PaperRecognition.MatchResult.failed;
        return PaperRecognition::MatchResult.failed;
      }
    }

    // java  String findDeny(String str)
    pub fn findDeny(&mut self, str: &str) -> Option<String> {
      // java String deny = this.mDenyPattern.firstMatcherString(str);
      let mut deny: String = self.mDenyPattern.firstMatcherString(str);
      // java if (deny != null)
      if deny.is_some() {
        // java LazzyPattern excludePattern = (LazzyPattern)this.mDenyExcludePatternMap.get(deny);
        let mut excludePattern: LazzyPattern = (self.mDenyExcludePatternMap.get(deny)).downcast_ref::<LazzyPattern>().unwrap();
        // java if (excludePattern == null || !excludePattern.find(str))
        if excludePattern.is_none() || !excludePattern.find(str) {
          // java return deny;
          return deny;
        }
      }
      // java return null;
      return None;
    }

    // java  MatchResult checkMoreKeyFromContent(TextDocument textDocument, String path)
    pub fn checkMoreKeyFromContent(&mut self, textDocument: &TextDocument, path: &str) -> MatchResult {
      // java AtomicInteger paraCount = new AtomicInteger(0);
      let mut paraCount: std::sync::atomic::AtomicI32 = AtomicInteger::new(0);
      // java this.mCacheRetult = PaperRecognition.MatchResult.failed;
      self.mCacheRetult = PaperRecognition::MatchResult.failed;
      // java enumParas(textDocument.getMainDocument(), (paraRange) -> { int count = paraCount.incrementAndGet(); String trimText = paraRange.getFilterText(128).replaceAll("(\\s+)|( )", ""); if (trimText.length() > 0 && this.findDeny(trimText) == null) { String p0key = this.mP0KeyPattern.firstMatcherString(trimText); if (p0key != null) { this.setMatcherTip("find p0 key:" + p0key + " from title:" + trimText, path); this.mCacheRetult = PaperRecognition.MatchResult.p0Match; return false; } Matcher p1Match = this.mP1KeyPattern.matcher(trimText); if (p1Match.find()) { StringBuilder p1key = new StringBuilder(); p1key.append(p1Match.group()); if (p1Match.find()) { p1key.append(',').append(p1Match.group()); this.setMatcherTip("find p1 keys:" + p1key + " from title:" + trimText, path); this.mCacheRetult = PaperRecognition.MatchResult.p1Match; return false; } } } return count < 5; });
      Self::enumParas(&mut textDocument.getMainDocument(), todo!());
      // java return this.mCacheRetult;
      return self.mCacheRetult;
    }

    // java static void enumParas(Document document, IEnumPara callBack)
    pub fn enumParas(document: &mut dyn Document, callBack: Option<&mut dyn IEnumPara>) {
      // java if (callBack != null)
      if let Some(callBack) = callBack {
        // java RangeReadLock readLock = document.readLock();
        let mut readLock: RangeReadLock = document.readLock();
            // java int start = 0;
            let mut start: i32 = 0;
            // java int end = document.getLength();
            let mut end: i32 = document.getLength();
            // java IPapxTable papxTable = document.getPapxTable();
            let mut papxTable: Box<dyn IPapxTable> = document.getPapxTable();
            // java Assert.assertNotNull("papxTable should not be null.", papxTable);
            Assert::assertNotNull__string_object(&"papxTable should not be null.", Some(&papxTable));
            // java IPapxTable.Entry s = papxTable.seek(0);
            let mut s: Box<dyn Entry> = papxTable.seek(0);
            // java IPapxTable.Entry e = papxTable.seek(end);
            let mut e: Box<dyn Entry> = papxTable.seek(end);
            // java if (!e.isEnd() && e.position() != end)
            if !e.isEnd() && e.position() != end {
              // java e = e.getNext();
              e = e.getNext();
            }
            // java if (!e.isEnd() && s == e)
            if !e.isEnd() && s == e {
              // java e = e.getNext();
              e = e.getNext();
            }
            // java KRange kRange = KRange.newInstance(document);
            let mut kRange: KRange = KRange::newInstance__document(&document);
            let mut p: Box<dyn Entry> = s;
            // java for(IPapxTable.Entry p = s; p != e p = p.getNext())
            while p != e {
              // java kRange.setRange(Math.max(0, p.position()), Math.min(p.endCp() - 1, end));
              kRange.setRange(Math::max(0, p.position()), Math::min(p.endCp() - 1, end));
              // java if (!callBack.onNextPara(kRange))
              if !callBack.onNextPara(kRange) {
                // java break;
                break;
              }
            }
              // java readLock.unlock();
              readLock.unlock();
      }
    }

    // java  boolean checkPaperSplit(TextDocument textDocument)
    pub fn checkPaperSplit(&mut self, textDocument: &TextDocument) -> bool {
      // java Document document = textDocument.getMainDocument();
      let mut document: Box<dyn Document> = textDocument.getMainDocument();
      // java return CardPaper.isPaper(document);
      return CardPaper::isPaper(&document);
    }

    // java  void setMatcherTip(String result, String path)
    pub fn setMatcherTip(&mut self, result: &str, path: &str) {
      // java this.mMatcherTip = path + "\t" + result;
      self.mMatcherTip = path + "\t" + result;
    }

    // java public String getMatcherTip()
    pub fn getMatcherTip(&self) -> String {
      // java return this.mMatcherTip;
      return self.mMatcherTip;
    }

    // java  void loadPattern()
    pub fn loadPattern(&mut self) {
      // java this.loadkeys("key_words.txt");
      self.loadkeys(&"key_words.txt");
      // java if (!this.mP0Keys.isEmpty())
      if !self.mP0Keys.is_empty() {
        // java this.mLoadPatternOk = true;
        self.mLoadPatternOk = true;
        // java this.mP0KeyPattern = new LazzyPattern(this.mP0Keys);
        self.mP0KeyPattern = LazzyPattern::new(self.mP0Keys);
        // java this.mP1KeyPattern = new LazzyPattern(this.mP1Keys);
        self.mP1KeyPattern = LazzyPattern::new(self.mP1Keys);
        // java this.mDenyPattern = new LazzyPattern(this.mDenys);
        self.mDenyPattern = LazzyPattern::new(self.mDenys);
      }
    }

    // java  void loadkeys(String resName)
    pub fn loadkeys(&mut self, resName: &str) {
          // java InputStream fis = Platform.getAssetManager().open(ASSERT_PATH + resName);
          let mut fis: Box<dyn std::io::Read> = Platform::getAssetManager().open(ASSERT_PATH + resName);
          // java if (fis != null)
          if fis.is_some() {
            // java InputStreamReader isr = new InputStreamReader(fis, StandardCharsets.UTF_8);
            let mut isr: InputStreamReader = InputStreamReader::new(fis, StandardCharsets::UTF_8);
            // java BufferedReader br = new BufferedReader(isr);
            let mut br: Box<dyn std::any::Any + Send + Sync> = BufferedReader::new(isr);
            // java String line = br.readLine();
            let mut line: String = br.readLine();
            // java while (true)
              // java label39: while(true) { if (line == null) { br.close(); isr.close(); return; } String[] ks = line.split("\t"); if (ks.length <= 1 || ks[0] == null) { break; } String key = ks[0]; if (key != null) { String pre = null; int i = 1; while(true) { if (i >= ks.length) { break label39; } this.addKey(key, ks[i], pre); pre = ks[i]; ++i; } }
              loop {
if line.is_none() { br.close();
isr.close();
return; }
let mut ks: Vec<String> = line.split("\t");
if (ks.len() as i32) <= 1 || ks[0usize].is_none() { break; }
let mut key: String = ks[0usize];
if key.is_some() { let mut pre: String = String::new();
let mut i: i32 = 1;
loop {
if i >= (ks.len() as i32) { break; }
self.addKey(&key, Some(&mut ks[(i) as usize]), Some(&pre));
pre = ks[(i) as usize];
{ let _tmp = i; i += 1; _tmp };
} }
}
              // java line = br.readLine();
              line = br.readLine();
          }
          // java IOException var10
          todo!();
    }

    // java public static boolean loadLinesFrom(String fileName, Map<String, Boolean> outList)
    pub fn loadLinesFrom(fileName: &str, outList: &mut HashMap<String, bool>) -> bool {
          // java InputStream fis = new FileInputStream(fileName);
          let mut fis: Box<dyn std::io::Read> = FileInputStream::new(fileName);
          // java InputStreamReader isr = new InputStreamReader(fis, StandardCharsets.UTF_8);
          let mut isr: InputStreamReader = InputStreamReader::new(fis, StandardCharsets::UTF_8);
          // java BufferedReader br = new BufferedReader(isr);
          let mut br: Box<dyn std::any::Any + Send + Sync> = BufferedReader::new(isr);
          let mut line: String = br.readLine();
          // java for(String line = br.readLine(); line != null line = br.readLine())
          while line.is_some() {
            // java outList.put(line, true);
            outList.put(line, true);
          }
          // java br.close();
          br.close();
          // java isr.close();
          isr.close();
          // java return true;
          return true;
          // java IOException var6
          todo!();
            // java return false;
            return false;
    }

    // java  void addKey(String key, String type, String preType)
    pub fn addKey(&mut self, key: &str, r#type: Option<&mut str>, preType: Option<&str>) {
      // java if (type != null)
      if let Some(r#type) = r#type {
        // java if (type.length() == 1)
        if (r#type.len() as i32) == 1 {
          // java char c = type.charAt(0);
          let mut c: char = r#type.charAt(0);
          // java if (c == '0')
          if c == '0' {
            // java this.mP0Keys.add(key);
            self.mP0Keys.add(key);
          }
          // java else if (c == '1')
          else if c == '1' {
            // java this.mP1Keys.add(key);
            self.mP1Keys.add(key);
          }
          // java else if (c == 'd')
          else if c == 'd' {
            // java this.mDenys.add(key);
            self.mDenys.add(key);
          }
        }
        // java else if (preType != null && preType.equals("d") && type.contains(key))
        else if let Some(preType) = preType {
          if preType.equals("d") && r#type.contains(key) {
            // java this.mDenyExcludePatternMap.put(key, new LazzyPattern(type));
            self.mDenyExcludePatternMap.put(key, LazzyPattern::new(r#type));
          }
        }
      }
    }

}

// java public static enum MatchResult
impl MatchResult {
}

// java public static class LazzyPattern
impl LazzyPattern {
    // java private String tranToPatternFromKeyWords(List<String> list)
    pub fn tranToPatternFromKeyWords(&mut self, list: Option<&Vec<String>>) -> String {
      // java if (list != null && !list.isEmpty())
      if let Some(list) = list {
        if !list.isEmpty() {
          // java StringBuffer sb = new StringBuffer();
          let mut sb: String = StringBuffer::new();
          // java sb.append('(').append((String)list.get(0));
          sb.append('(').append(list.get__int(0).to_string());
          // java int size = list.size();
          let mut size: i32 = list.size();
          // java if (size > 1)
          if size > 1 {
            let mut i: i32 = 1;
            // java for(int i = 1; i < size; ++i)
            while i < size {
              // java sb.append('|').append((String)list.get(i));
              sb.append('|').append(list.get(i).to_string());
            }
          }
          // java sb.append(")");
          sb.append(")");
          // java return sb.toString();
          return sb.toString();
        }
      }
      // java else
      else {
        // java return "";
        return "";
      }
    }

    // java public  LazzyPattern(List<String> keys)
    pub fn new__liststring(keys: &Vec<String>) -> Self {
      // java this.mKeys = keys;
      Self { mPattern: Default::default(), mKeys: keys }
    }

    // java public  LazzyPattern(String sepStr)
    pub fn new__string(sepStr: &mut str) -> Self {
      // java String[] list = sepStr.split(",");
      let mut list: Vec<String> = sepStr.split(",");
      // java this.mKeys = new ArrayList();
      let mut this = Self { mPattern: Default::default(), mKeys: Vec::new() };
      // java for(String it : list) { this.mKeys.add(it);
      this.mKeys.add(it);
      this
    }

    // java private Pattern get()
    pub fn get(&mut self) -> Box<dyn Pattern> {
      // java if (this.mPattern == null)
      if self.mPattern.is_none() {
          // java (this)
          self;
            // java if (this.mPattern == null)
            if self.mPattern.is_none() {
              // java this.mPattern = Pattern.compile(this.tranToPatternFromKeyWords(this.mKeys));
              self.mPattern = Pattern::compile(&self.tranToPatternFromKeyWords(Some(&self.mKeys)));
            }
      }
      // java return this.mPattern;
      return self.mPattern;
    }

    // java public Matcher matcher(CharSequence txt)
    pub fn matcher(&self, txt: &CharSequence) -> Matcher {
      // java return this.get().matcher(txt);
      return self.get().matcher(txt);
    }

    // java public boolean find(CharSequence txt)
    pub fn find(&mut self, txt: &CharSequence) -> bool {
      // java return this.matcher(txt).find();
      return self.matcher(&txt).find();
    }

    // java public String firstMatcherString(CharSequence txt)
    pub fn firstMatcherString(&mut self, txt: &CharSequence) -> Option<String> {
      // java Matcher matcher = this.matcher(txt);
      let mut matcher: Matcher = self.matcher(&txt);
      // java return matcher.find() ? matcher.group() : null;
      return (if matcher.find() { Some(matcher.group()) } else { None });
    }

}
