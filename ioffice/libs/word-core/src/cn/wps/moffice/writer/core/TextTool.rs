// Java: import cn.wps.base.io.ControlChar
use util::base::io::ControlChar::*;
// Java: import cn.wps.moffice.define.FontName
use util::moffice::define::FontName::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.text_extractor.TextExtractorTool
use util_adapter::cn::wps::moffice::text_extractor::TextExtractorTool::*;
// Java: import cn.wps.moffice.util.FileUtil
use util::moffice::util::FileUtil::*;
// Java: import cn.wps.moffice.util.StringUtil
use util::moffice::util::StringUtil::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCMathEnd
use crate::cn::wps::moffice::writer::data::PLCMathEnd::*;
// Java: import cn.wps.moffice.writer.data.PLCMathStart
use crate::cn::wps::moffice::writer::data::PLCMathStart::*;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.moffice.writer.data.value.Toggle
use crate::cn::wps::moffice::writer::data::value::Toggle::*;
// Java: import java.lang.Character.UnicodeBlock
use stubs::java::lang::Character as JavaCharacter;
// Java: import java.util.BitSet
use stubs::java::util::BitSet::*;
// Java: import java.util.HashMap
// Java: import java.util.regex.Pattern
use stubs::java::util::regex::Pattern::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.io.sprm.Character
use crate::cn::wps::moffice::writer::io::sprm::Character::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
use std::collections::{HashMap};

// java private static final char INVALID_CHAR = '\u0000';
const INVALID_CHAR: char = '\u{0000}';
// java public static final int TEXT_DIR_RTL = 1;
pub(crate) const TEXT_DIR_RTL: i32 = 1;
// java public static final int TEXT_DIR_LTR = 2;
pub(crate) const TEXT_DIR_LTR: i32 = 2;
// java public static final int TEXT_DIR_NORMAL = 3;
pub(crate) const TEXT_DIR_NORMAL: i32 = 3;
// java public static final String IME_PARAGRAPHEND = "\n";
pub(crate) const IME_PARAGRAPHEND: &str = "\n";
// java public static final String FIRSTLINEINDENT_TO_SPACE = "    ";
pub(crate) const FIRSTLINEINDENT_TO_SPACE: &str = "    ";
// java public static final String CELLEND_TO_SPACE = " ";
pub(crate) const CELLEND_TO_SPACE: &str = " ";
// java public static final String NUMBERING_SUBSTITUTE = "\u0000";
pub(crate) const NUMBERING_SUBSTITUTE: &str = "\u{0000}";
// java public static final int SPACE_TO_FIRSTLINEINDENT = 50;
pub(crate) const SPACE_TO_FIRSTLINEINDENT: i32 = 50;
// java private static final int MAX_NEST_FIELD = 10;
const MAX_NEST_FIELD: i32 = 10;

// java public class TextTool
#[derive(Default)]
pub struct TextTool {
}

// java public class TextTool
impl TextTool {
    // java public static boolean isVisibleChar(char c)
    pub fn isVisibleChar(c: char) -> bool {
      // java if (c != '\t' && c != '\n' && c != '\r' && c != 11 && c != '\f' && c != 14)
      if c != '\t' && c != '\n' && c != '\r' && c != 11 && c != '\u{000c}' && c != 14 {
        // java if (c < ' ')
        if c < ' ' {
          // java return false;
          return false;
        }
        // java else
        else {
          // java return !Character.isISOControl(c);
          return !JavaCharacter::isISOControl(c);
        }
      }
      // java else
      else {
        // java return true;
        return true;
      }
    }

    // java public static String filterText(String text)
    pub fn filterText__string(text: &str) -> Option<String> {
      // java return filterText2(text, (String)null, (int[])null);
      return Self::filterText2(Some(&mut text), Some(&mut String::new()), None);
    }

    // java public static String filterText(String text, String replaceChar)
    pub fn filterText__string_string(text: &str, replaceChar: &str) -> Option<String> {
      // java return filterText2(text, replaceChar, (int[])null);
      return Self::filterText2(Some(&mut text), Some(&mut replaceChar), None);
    }

    // java private static String filterText2(String text, String replaceChar, int[] originCPs)
    pub fn filterText2(text: Option<&mut str>, replaceChar: Option<&mut str>, originCPs: Option<&mut Vec<i32>>) -> String {
      // java if (text == null)
      if text.is_none() {
        // java return "";
        return "";
      }
      // java else
      else {
        // java int n = text.length();
        let mut n: i32 = (text.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
        // java if (n == 0)
        if n == 0 {
          // java return "";
          return "";
        }
        // java else
        else {
          // java char[] buf = new char[n];
          let mut buf: Vec<char> = vec![Default::default(); (n) as usize];
          // java int nchar = 0;
          let mut nchar: i32 = 0;
          // java BitSet stack = new BitSet();
          let mut stack: BitSet = BitSet::new();
          // java int nstack = 0;
          let mut nstack: i32 = 0;
          // java stack.set(nstack++);
          stack.set({ let _tmp = nstack; nstack += 1; _tmp });
          let mut i: i32 = 0;
          // java for(int i = 0; i < n; ++i)
          while i < n {
            // java char c = text.charAt(i);
            let mut c: char = text.charAt(i);
            // java boolean skip = true;
            let mut skip: bool = true;
            // java if (c == 19)
            if c == 19 {
              // java stack.clear(nstack++);
              stack.clear({ let _tmp = nstack; nstack += 1; _tmp });
            }
            // java else if (c == 20)
            else if c == 20 {
              // java stack.set(nstack - 1);
              stack.set(nstack - 1);
            }
            // java else if (c == 21)
            else if c == 21 {
              // java nstack = nstack > 1 ? nstack - 1 : 1;
              nstack = (if nstack > 1 { nstack - 1 } else { 1 });
            }
            // java else
            else {
              // java skip = false;
              skip = false;
            }
            // java if (!skip && stack.get(nstack - 1) && nstack < 10)
            if !skip && stack.get(nstack - 1) && nstack < 10 {
              // java boolean skiped = false;
              let mut skiped: bool = false;
              // java if (isNeedReplace(c))
              if Self::isNeedReplace(c) {
                // java buf[nchar++] = '\n';
                buf[({ let _tmp = nchar; nchar += 1; _tmp }) as usize] = '\n';
              }
              // java else if (isVisibleChar(c))
              else if Self::isVisibleChar(c) {
                // java buf[nchar++] = c;
                buf[({ let _tmp = nchar; nchar += 1; _tmp }) as usize] = c;
              }
              // java else if (replaceChar != null)
              else if let Some(replaceChar) = replaceChar {
                // java buf[nchar++] = replaceChar.charAt(0);
                buf[({ let _tmp = nchar; nchar += 1; _tmp }) as usize] = replaceChar.charAt(0);
              }
              // java else
              else {
                // java skiped = true;
                skiped = true;
              }
              // java if (!skiped && originCPs != null && i < originCPs.length)
              if let Some(originCPs) = originCPs {
                if !skiped && i < (originCPs.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) {
                  // java originCPs[nchar - 1] = originCPs[i];
                  originCPs[(nchar - 1) as usize] = originCPs[(i) as usize];
                }
              }
            }
          }
          // java return new String(buf, 0, nchar);
          return String::new(buf, 0, nchar);
        }
      }
    }

    // java private static void filter_omath(char[] chars, int len, Document document, int text_start)
    pub fn filter_omath(chars: &mut Vec<char>, len: i32, document: &dyn Document, text_start: i32) {
      // java PLCMathStart starts = document.getOMathStarts();
      let mut starts: PLCMathStart = document.getOMathStarts();
      // java PLCMathEnd ends = document.getOMathEnds();
      let mut ends: PLCMathEnd = document.getOMathEnds();
      // java if (starts != null && !starts.isEmpty())
      if starts.is_some() && !starts.is_empty() {
        // java PLC.Iterator iter1 = starts.createIterator();
        let mut iter1: Vec<()> = starts.createIterator();
        // java PLC.Iterator iter2 = ends.createIterator();
        let mut iter2: Vec<()> = ends.createIterator();
        // java while (iter1.hasNext())
        while iter1.hasNext() {
          // java iter1.next();
          iter1.next();
          // java iter2.next();
          iter2.next();
          // java int start_cp = iter1.getStart() - text_start;
          let mut start_cp: i32 = iter1.getStart() - text_start;
          // java int end_cp = iter2.getStart() - text_start;
          let mut end_cp: i32 = iter2.getStart() - text_start;
          // java if (end_cp > 0 && start_cp < len)
          if end_cp > 0 && start_cp < len {
            // java start_cp = Math.max(0, start_cp);
            start_cp = Math::max(0, start_cp);
            // java end_cp = Math.min(end_cp, len);
            end_cp = Math::min(end_cp, len);
            let mut i: i32 = start_cp;
            // java for(int i = start_cp; i < end_cp; ++i)
            while i < end_cp {
              // java chars[i] = 0;
              chars[(i) as usize] = 0;
            }
          }
        }
      }
    }

    // java public static int filterMath(char[] chars, int len, Document document, int text_start)
    pub fn filterMath__chararr_int_document_int(chars: &mut Vec<char>, len: i32, document: &dyn Document, text_start: i32) -> i32 {
      // java filter_omath(chars, len, document, text_start);
      Self::filter_omath(&mut chars, len, &document, text_start);
      // java int n = 0;
      let mut n: i32 = 0;
      let mut i: i32 = 0;
      // java for(int i = 0; i < len; ++i)
      while i < len {
        // java if (chars[i] != 0)
        if chars[(i) as usize] != 0 {
          // java chars[n++] = chars[i];
          chars[({ let _tmp = n; n += 1; _tmp }) as usize] = chars[(i) as usize];
        }
      }
      // java return n;
      return n;
    }

    // java public static String filterMath(KRange range)
    pub fn filterMath__krange(range: &KRange) -> String {
      // java Document document = range.getSubDocument();
      let mut document: Box<dyn Document> = range.getSubDocument();
      // java int start = range.getStart();
      let mut start: i32 = range.getStart();
      // java int end = range.getEnd();
      let mut end: i32 = range.getEnd();
      // java if (start >= end)
      if start >= end {
        // java return "";
        return "";
      }
      // java else
      else {
        // java char[] buf = new char[end - start];
        let mut buf: Vec<char> = vec![Default::default(); (end - start) as usize];
        // java int len = document.getText(start, end, buf, 0);
        let mut len: i32 = document.getText(start, end, buf, 0);
        // java filter_omath(buf, len, document, start);
        Self::filter_omath(&mut buf, len, &document, start);
        // java StringBuilder sb = new StringBuilder(len);
        let mut sb: String = StringBuilder::new(len);
        let mut i: i32 = 0;
        // java for(int i = 0; i < len; ++i)
        while i < len {
          // java if (buf[i] != 0)
          if buf[(i) as usize] != 0 {
            // java sb.append(buf[i]);
            sb.append(buf[(i) as usize]);
          }
        }
        // java return sb.toString();
        return sb.toString();
      }
    }

    // java public static String filterCopyText(KRange range)
    pub fn filterCopyText(range: &KRange) -> Option<String> {
      // java Document document = range.getSubDocument();
      let mut document: Box<dyn Document> = range.getSubDocument();
      // java int start = range.getStart();
      let mut start: i32 = range.getStart();
      // java int end = range.getEnd();
      let mut end: i32 = range.getEnd();
      // java if (start >= end)
      if start >= end {
        // java return "";
        return "";
      }
      // java else
      else {
        // java char[] buf = new char[end - start];
        let mut buf: Vec<char> = vec![Default::default(); (end - start) as usize];
        // java int len = document.getText(start, end, buf, 0);
        let mut len: i32 = document.getText(start, end, buf, 0);
        let mut i: i32 = 0;
        // java for(int i = 0; i < len; ++i)
        while i < len {
          // java if (buf[i] == '\r' || buf[i] == 7)
          if buf[(i) as usize] == '\r' || buf[(i) as usize] == 7 {
            // java buf[i] = '\n';
            buf[(i) as usize] = '\n';
          }
        }
        // java filter_omath(buf, len, document, start);
        Self::filter_omath(&mut buf, len, &document, start);
        // java filter_revisionDelete(buf, len, document, start);
        Self::filter_revisionDelete(&mut buf, len, &document, start);
        // java StringBuilder sb = new StringBuilder(len);
        let mut sb: String = StringBuilder::new(len);
        let mut i: i32 = 0;
        // java for(int i = 0; i < len; ++i)
        while i < len {
          // java if (buf[i] != 0)
          if buf[(i) as usize] != 0 {
            // java sb.append(buf[i]);
            sb.append(buf[(i) as usize]);
          }
        }
        // java return filterText2(sb.toString(), (String)null, (int[])null);
        return Self::filterText2(Some(&mut sb.toString()), Some(&mut String::new()), None);
      }
    }

    // java public static String filterRange(KRange range)
    pub fn filterRange__krange(range: &KRange) -> Option<String> {
      // java return filterRange(range, false, (String)null, (int[])null);
      return Self::filterRange__krange_boolean_string_intarr(&range, false, &String::new(), None);
    }

    // java public static String filterRange(KRange range, String replaceChar)
    pub fn filterRange__krange_string(range: &KRange, replaceChar: &str) -> Option<String> {
      // java return filterRange(range, false, replaceChar, (int[])null);
      return Self::filterRange__krange_boolean_string_intarr(&range, false, &replaceChar, None);
    }

    // java public static String filterRange(KRange range, boolean fFilterRevisionDelete)
    pub fn filterRange__krange_boolean(range: &KRange, fFilterRevisionDelete: bool) -> Option<String> {
      // java return filterRange(range, fFilterRevisionDelete, (String)null, (int[])null);
      return Self::filterRange__krange_boolean_string_intarr(&range, fFilterRevisionDelete, &String::new(), None);
    }

    // java public static String filterRange(KRange range, boolean fFilterRevisionDelete, String replaceChar, int[] originCPs)
    pub fn filterRange__krange_boolean_string_intarr(range: &KRange, fFilterRevisionDelete: bool, replaceChar: &str, originCPs: Option<&mut Vec<i32>>) -> String {
      // java Document document = range.getSubDocument();
      let mut document: Box<dyn Document> = range.getSubDocument();
      // java int start = range.getStart();
      let mut start: i32 = range.getStart();
      // java int end = range.getEnd();
      let mut end: i32 = range.getEnd();
      // java if (start >= end)
      if start >= end {
        // java return "";
        return "";
      }
      // java else
      else {
        // java char[] buf = new char[end - start];
        let mut buf: Vec<char> = vec![Default::default(); (end - start) as usize];
        // java int len = document.getText(start, end, buf, 0);
        let mut len: i32 = document.getText(start, end, buf, 0);
        // java filter_omath(buf, len, document, start);
        Self::filter_omath(&mut buf, len, &document, start);
        // java if (fFilterRevisionDelete)
        if fFilterRevisionDelete {
          // java filter_revisionDelete(buf, len, document, start);
          Self::filter_revisionDelete(&mut buf, len, &document, start);
        }
        // java StringBuilder sb = new StringBuilder(len);
        let mut sb: String = StringBuilder::new(len);
        let mut i: i32 = 0;
        // java for(int i = 0; i < len; ++i)
        while i < len {
          // java if (buf[i] != 0)
          if buf[(i) as usize] != 0 {
            // java sb.append(buf[i]);
            sb.append(buf[(i) as usize]);
            // java if (originCPs != null && sb.length() <= originCPs.length)
            if let Some(originCPs) = originCPs {
              if (sb.len() as i32) <= (originCPs.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) {
                // java originCPs[sb.length() - 1] = start + i;
                originCPs[((sb.len() as i32) - 1) as usize] = start + i;
              }
            }
          }
        }
        // java String text = sb.toString();
        let mut text: String = sb.toString();
        // java return filterText2(text, replaceChar, originCPs);
        return Self::filterText2(Some(&mut text), Some(&mut replaceChar), Some(&mut originCPs));
      }
    }

    // java private static void filter_revisionDelete(char[] buf, int len, Document document, int start)
    pub fn filter_revisionDelete(buf: &mut Vec<char>, len: i32, document: &dyn Document, start: i32) {
      // java IChpxTable chpx = document.getChpxTable();
      let mut chpx: Box<dyn IChpxTable> = document.getChpxTable();
      // java IChpxTable.Entry run = chpx.seek(start);
      let mut run: Box<dyn Entry> = chpx.seek(start);
      // java int i = 0;
      let mut i: i32 = 0;
      let mut begin: i32 = start;
      // java for(int begin = start; i < buf.length && !run.isEnd() run = chpx.seek(begin))
      while i < (buf.len() as i32) && !run.isEnd() {
        // java int end = run.endCp();
        let mut end: i32 = run.endCp();
        // java if (RangeTool.isRunDelete(run))
        if RangeTool::isRunDelete__entry(&run) {
          // java while (begin < end && i < buf.length)
          while begin < end && i < (buf.len() as i32) {
            // java buf[i] = 0;
            buf[(i) as usize] = 0;
            // java ++i;
            { let _tmp = i; i += 1; _tmp };
            // java ++begin;
            { let _tmp = begin; begin += 1; _tmp };
          }
        }
        // java else
        else {
          // java i += end - begin;
          i = i + end - begin;
          // java begin = end;
          begin = end;
        }
      }
    }

    // java public static String filterTextForComment(String text)
    pub fn filterTextForComment(text: Option<&mut str>) -> String {
      // java if (text == null)
      if text.is_none() {
        // java return "";
        return "";
      }
      // java else
      else {
        // java int n = text.length();
        let mut n: i32 = (text.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
        // java if (n == 0)
        if n == 0 {
          // java return "";
          return "";
        }
        // java else
        else {
          // java char[] buf = new char[n];
          let mut buf: Vec<char> = vec![Default::default(); (n) as usize];
          // java int nchar = 0;
          let mut nchar: i32 = 0;
          // java BitSet stack = new BitSet();
          let mut stack: BitSet = BitSet::new();
          // java int nstack = 0;
          let mut nstack: i32 = 0;
          // java stack.set(nstack++);
          stack.set({ let _tmp = nstack; nstack += 1; _tmp });
          let mut i: i32 = 0;
          // java for(int i = 0; i < n; ++i)
          while i < n {
            // java char c = text.charAt(i);
            let mut c: char = text.charAt(i);
            // java boolean skip = true;
            let mut skip: bool = true;
            // java if (c == 19)
            if c == 19 {
              // java stack.clear(nstack++);
              stack.clear({ let _tmp = nstack; nstack += 1; _tmp });
            }
            // java else if (c == 20)
            else if c == 20 {
              // java stack.set(nstack - 1);
              stack.set(nstack - 1);
            }
            // java else if (c == 21)
            else if c == 21 {
              // java nstack = nstack > 1 ? nstack - 1 : 1;
              nstack = (if nstack > 1 { nstack - 1 } else { 1 });
            }
            // java else
            else {
              // java skip = false;
              skip = false;
            }
            // java if (!skip && stack.get(nstack - 1) && nstack < 10)
            if !skip && stack.get(nstack - 1) && nstack < 10 {
              // java if ('\f' == c)
              if '\u{000c}' == c {
                // java buf[nchar++] = ' ';
                buf[({ let _tmp = nchar; nchar += 1; _tmp }) as usize] = ' ';
              }
              // java else if (isVisibleChar(c))
              else if Self::isVisibleChar(c) {
                // java buf[nchar++] = c;
                buf[({ let _tmp = nchar; nchar += 1; _tmp }) as usize] = c;
              }
            }
          }
          // java return new String(buf, 0, nchar);
          return String::new(buf, 0, nchar);
        }
      }
    }

    // java private static boolean isNeedReplace(char c)
    pub fn isNeedReplace(c: char) -> bool {
      // java return '\f' == c || 11 == c;
      return '\u{000c}' == c || 11 == c;
    }

    // java public static void filterText(char[] text, int offset, int count, StringBuilder buffer)
    pub fn filterText__chararr_int_int_stringbuilder(text: Option<&Vec<char>>, offset: i32, count: i32, buffer: Option<&mut str>) {
      // java if (text != null && buffer != null && offset >= 0 && count != 0)
      if let (Some(text), Some(buffer)) = (text, buffer) {
        if offset >= 0 && count != 0 {
          // java BitSet stack = new BitSet();
          let mut stack: BitSet = BitSet::new();
          // java int nstack = 0;
          let mut nstack: i32 = 0;
          // java stack.set(nstack++);
          stack.set({ let _tmp = nstack; nstack += 1; _tmp });
          let mut i: i32 = offset;
          // java for(int i = offset; i < count; ++i)
          while i < count {
            // java char c = text[i];
            let mut c: char = text[(i) as usize];
            // java boolean skip = true;
            let mut skip: bool = true;
            // java if (c == 19)
            if c == 19 {
              // java stack.clear(nstack++);
              stack.clear({ let _tmp = nstack; nstack += 1; _tmp });
            }
            // java else if (c == 20)
            else if c == 20 {
              // java stack.set(nstack - 1);
              stack.set(nstack - 1);
            }
            // java else if (c == 21)
            else if c == 21 {
              // java nstack = nstack > 1 ? nstack - 1 : 1;
              nstack = (if nstack > 1 { nstack - 1 } else { 1 });
            }
            // java else
            else {
              // java skip = false;
              skip = false;
            }
            // java if (!skip && stack.get(nstack - 1) && nstack < 10 && isVisibleChar(c))
            if !skip && stack.get(nstack - 1) && nstack < 10 && Self::isVisibleChar(c) {
              // java buffer.append(c);
              buffer.append(c);
            }
          }
        }
      }
    }

    // java public static void reverseCharArray(char[] buf)
    pub fn reverseCharArray(buf: &mut Vec<char>) {
      // java int j = buf.length - 1;
      let mut j: i32 = (buf.len() as i32) - 1;
      let mut a: i32 = 0;
      // java for(int a = 0; a < buf.length / 2 --j)
      while a < (buf.len() as i32) / 2 {
        // java char temp = buf[a];
        let mut temp: char = buf[(a) as usize];
        // java buf[a] = buf[j];
        buf[(a) as usize] = buf[(j) as usize];
        // java buf[j] = temp;
        buf[(j) as usize] = temp;
        // java ++a;
        { let _tmp = a; a += 1; _tmp };
      }
    }

    // java public static Toggle getTextBidi(String text)
    pub fn getTextBidi(text: &mut str) -> Option<Toggle> {
      // java boolean bRTL = false;
      let mut bRTL: bool = false;
      // java if (text.length() > 0)
      if (text.len() as i32) > 0 {
        // java bRTL = CharParse.getInstance().charIsRTL(text.charAt(0));
        bRTL = CharParse::getInstance().charIsRTL(text.charAt(0));
      }
      // java return bRTL ? Toggle.ON : null;
      return (if bRTL { Some(Toggle::ON) } else { None });
    }

    // java public static int getTextDirectionality(char c)
    pub fn getTextDirectionality__char(c: char) -> i32 {
      // java int directionality = Character.getDirectionality(c);
      let mut directionality: i32 = JavaCharacter::getDirectionality(c);
      // java if (directionality != 0 && directionality != 3 && directionality != 6)
      if directionality != 0 && directionality != 3 && directionality != 6 {
        // java return directionality != 1 && directionality != 2 ? 3 : 1;
        return (if directionality != 1 && directionality != 2 { 3 } else { 1 });
      }
      // java else
      else {
        // java return 2;
        return 2;
      }
    }

    // java public static int getTextDirectionality(char[] text)
    pub fn getTextDirectionality__chararr(text: Option<&Vec<char>>) -> Option<i32> {
      // java return text != null && text.length != 0 ? getTextDirectionality(text[0]) : 3;
      return (if text.is_some() && (text.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) != 0 { Some(Self::getTextDirectionality__char(text.as_ref().unwrap()[0usize])) } else { Some(3) });
    }

    // java public static int getTextDirectionality(String text)
    pub fn getTextDirectionality__string(text: Option<&mut str>) -> Option<i32> {
      // java return text != null && text.length() != 0 ? getTextDirectionality(text.charAt(0)) : 3;
      return (if text.is_some() && (text.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) != 0 { Some(Self::getTextDirectionality__char(text.charAt(0))) } else { Some(3) });
    }

    // java public static String getNewFileName(Document document)
    pub fn getNewFileName(document: &dyn Document) -> String {
      // java int n = Math.min(1000, document.getLength());
      let mut n: i32 = Math::min(1000, document.getLength());
      // java char[] buf = new char[n];
      let mut buf: Vec<char> = vec![Default::default(); (n) as usize];
      // java document.getText(0, buf.length, buf, 0);
      document.getText(0, (buf.len() as i32), &buf, 0);
      // java int MAX = 15;
      let mut MAX: i32 = 15;
      // java char GAP = '_';
      let mut GAP: char = '_';
      // java int start = getCanBeginNewFileName(buf);
      let mut start: i32 = Self::getCanBeginNewFileName(&buf);
      // java if (start < 0)
      if start < 0 {
        // java return "";
        return "";
      }
      // java else
      else {
        // java StringBuilder buffer = new StringBuilder();
        let mut buffer: String = StringBuilder::new();
        // java boolean isGap_ = false;
        let mut isGap_: bool = false;
        let mut i: i32 = start;
        // java for(int i = start; i < Math.min(start + 15, buf.length); ++i)
        while i < Math::min(start + 15, (buf.len() as i32)) {
          // java char c = buf[i];
          let mut c: char = buf[(i) as usize];
          // java if (c == 19 || ControlChar.isNewLine(c) || c == 1 || c == '\b' || c == 5)
          if c == 19 || ControlChar::isNewLine(c) || c == 1 || c == '\u{0008}' || c == 5 {
            // java break;
            break;
          }
          // java if (!FileUtil.isFileNameValid(c))
          if !FileUtil::isFileNameValid__string(&c) {
            // java c = '_';
            c = '_';
            // java if (!isGap_)
            if !isGap_ {
              // java buffer.append(c);
              buffer.append(c);
            }
            // java isGap_ = true;
            isGap_ = true;
          }
          // java else
          else {
            // java isGap_ = false;
            isGap_ = false;
            // java buffer.append(c);
            buffer.append(c);
          }
        }
        // java if (isGap_)
        if isGap_ {
          // java int last = buffer.length() - 1;
          let mut last: i32 = (buffer.len() as i32) - 1;
          // java if (buffer.charAt(last) == '_')
          if buffer.charAt(last) == '_' {
            // java buffer.deleteCharAt(last);
            buffer.deleteCharAt(last);
          }
        }
        // java String res;
        let mut res: String = String::new();
        res = buffer.toString().trim();
        // java for(res = buffer.toString().trim() res.endsWith("　") res = res.substring(0, res.length() - 1).trim())
        while res.endsWith("　") {
        }
        // java return res;
        return res;
      }
    }

    // java private static int getCanBeginNewFileName(char[] buf)
    pub fn getCanBeginNewFileName(buf: &Vec<char>) -> i32 {
      // java int fieldLevel = 0;
      let mut fieldLevel: i32 = 0;
      let mut i: i32 = 0;
      // java for(int i = 0; i < buf.length; ++i)
      while i < (buf.len() as i32) {
        // java char c = buf[i];
        let mut c: char = buf[(i) as usize];
        // java if (c == 19)
        if c == 19 {
          // java ++fieldLevel;
          { let _tmp = fieldLevel; fieldLevel += 1; _tmp };
        }
        // java else if (c == 21)
        else if c == 21 {
          // java --fieldLevel;
          { let _tmp = fieldLevel; fieldLevel -= 1; _tmp };
        }
        // java else if (fieldLevel <= 0 && c > ' ' && c != 12288 && FileUtil.isFileNameValid(c) && !ControlChar.isNewLine(c) && !ControlChar.isParagraphBreak(c) && !StringUtil.isPunctuation(c))
        else if fieldLevel <= 0 && c > ' ' && c != 12288 && FileUtil::isFileNameValid__string(&c) && !ControlChar::isNewLine(c) && !ControlChar::isParagraphBreak(c) && !StringUtil::isPunctuation(c) {
          // java return i;
          return i;
        }
      }
      // java return -1;
      return -1;
    }

    // java public static void ModifyF0(PropertySet chpx, char[] text)
    pub fn ModifyF0(chpx: &PropertySet, text: &mut Vec<char>) {
      // java PropertySet prop = PropertyUtil.getFinalRunFormat(chpx);
      let mut prop: PropertySet = PropertyUtil::getFinalRunFormat(&chpx);
      // java Object charSymbol = prop.getValue(20);
      let mut charSymbol: Box<dyn Object> = prop.getValue(20);
      // java Object fontSymbol = prop.getValue(19);
      let mut fontSymbol: Box<dyn Object> = prop.getValue(19);
      // java if (null == charSymbol || null == fontSymbol)
      if charSymbol.is_none() || fontSymbol.is_none() {
        // java int hint = prop.getInt(40, 0);
        let mut hint: i32 = prop.getInt(40, 0);
        // java int propId = CharFontTool.getMultiPropertyId(4, hint, text[0]);
        let mut propId: i32 = CharFontTool::getMultiPropertyId(4, hint, text[0usize]);
        // java Object value = prop.getValue(propId);
        let mut value: Box<dyn Object> = prop.getValue(propId);
        // java if (null != value && value instanceof String && FontName.isSpecialFontName((String)value))
        if value.is_some() && value.downcast_ref::<String>().is_some() && FontName::isSpecialFontName(&value.to_string()) {
          let mut i: i32 = 0;
          // java for(int i = 0; i < text.length; ++i)
          while i < (text.len() as i32) {
            // java char c = text[i];
            let mut c: char = text[(i) as usize];
            // java if (FontName.isNormalChar(c))
            if FontName::isNormalChar(c) {
              // java text[i] = (char)(c | '\uf000');
              text[(i) as usize] = todo!("cast → char");
            }
          }
        }
      }
    }

    // java public static String getFont(char[] text)
    pub fn getFont(text: &Vec<char>) -> Option<String> {
      // java if (isBengali(text[0]))
      if Self::isBengali(text[0usize]) {
        // java return "Vrinda";
        return "Vrinda";
      }
      // java else if (isLao(text[0]))
      else if Self::isLao(text[0usize]) {
        // java return "DokChampa";
        return "DokChampa";
      }
      // java else if (isDevanagari(text[0]))
      else if Self::isDevanagari(text[0usize]) {
        // java return "Mangal";
        return "Mangal";
      }
      // java else
      else {
        // java return isThai(text[0]) ? "TH Sarabun PSK" : null;
        return (if Self::isThai(text[0usize]) { Some("TH Sarabun PSK".to_string()) } else { None });
      }
    }

    // java public static boolean isComplexScript(char ch)
    pub fn isComplexScript(ch: char) -> bool {
      // java return isDevanagari(ch) || isBengali(ch) || isLao(ch) || isThai(ch);
      return Self::isDevanagari(ch) || Self::isBengali(ch) || Self::isLao(ch) || Self::isThai(ch);
    }

    // java private static boolean isThai(char ch)
    pub fn isThai(ch: char) -> bool {
      // java return 3584 <= ch && ch <= 3711;
      return 3584 <= ch && ch <= 3711;
    }

    // java public static boolean isDevanagari(char ch)
    pub fn isDevanagari(ch: char) -> bool {
      // java return 2304 <= ch && ch <= 2431;
      return 2304 <= ch && ch <= 2431;
    }

    // java public static boolean isBengali(char ch)
    pub fn isBengali(ch: char) -> bool {
      // java return 2432 <= ch && ch <= 2559;
      return 2432 <= ch && ch <= 2559;
    }

    // java public static boolean isLao(char ch)
    pub fn isLao(ch: char) -> bool {
      // java return 3712 <= ch && ch <= 3839;
      return 3712 <= ch && ch <= 3839;
    }

    // java public static boolean isIndic(char c)
    pub fn isIndic(c: char) -> bool {
      // java Character.UnicodeBlock block = UnicodeBlock.of(c);
      // CARGO_BLOCKER: java.lang.Character.UnicodeBlock stub not generated
      let _ = c;
      // java return block == UnicodeBlock.DEVANAGARI || ...
      return false;
    }

    // java public static boolean isPhoneNum(String text)
    pub fn isPhoneNum(text: Option<&mut str>) -> bool {
      // java if (text == null)
      if text.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java text = text.trim();
        text = text.trim();
        // java if (text.length() < 5)
        if (text.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) < 5 {
          // java return false;
          return false;
        }
        // java else
        else {
          // java String reg = "^[\\f\\t]*(＋|\\+)?[0-9]{5,20}[ \\f\\t]*\\n{0,1}$";
          let mut reg: String = "^[\\u{000c}\\t]*(＋|\\+)?[0-9]{5,20}[ \\u{000c}\\t]*\\n{0,1}$".to_string();
          // java Pattern p = Pattern.compile(reg);
          let mut p: Box<dyn Pattern> = Pattern::compile(&reg);
          // java return p.matcher(text).matches();
          return p.matcher(text).matches();
        }
      }
    }

    // java public static String getDocText(TextDocument textDocument, int maxCount)
    pub fn getDocText(textDocument: Option<&TextDocument>, maxCount: i32) -> String {
      // java if (textDocument != null && maxCount > 0)
      if let Some(textDocument) = textDocument {
        if maxCount > 0 {
          // java Document mainDocument = textDocument.getMainDocument();
          let mut mainDocument: Box<dyn Document> = textDocument.getMainDocument();
          // java if (mainDocument == null)
          if mainDocument.is_none() {
            // java return "";
            return "";
          }
          // java else
          else {
            // java int maxCp = Math.min(maxCount, mainDocument.getLength());
            let mut maxCp: i32 = Math::min(maxCount, mainDocument.getLength());
            // java char[] mBuffer = new char[maxCp];
            let mut mBuffer: Vec<char> = vec![Default::default(); (maxCp) as usize];
            // java mainDocument.getText(0, maxCp, mBuffer, 0);
            mainDocument.getText(0, maxCp, mBuffer, 0);
            // java return new String(mBuffer);
            return String::new(mBuffer);
          }
        }
      }
      // java else
      else {
        // java return "";
        return "";
      }
    }

    // java public static HashMap<Integer, String> getPaperInfo(TextDocument textDocument, int maxCount)
    pub fn getPaperInfo(textDocument: &TextDocument, maxCount: i32) -> HashMap<i32, String> {
      // java String content = getDocText(textDocument, maxCount);
      let mut content: String = Self::getDocText(Some(&textDocument), maxCount);
      // java return TextExtractorTool.getPaperInfo(textDocument.getPath(), content);
      return TextExtractorTool::getPaperInfo__string_string(&textDocument.getPath(), &content);
    }

    // java public static boolean isEmpty(String str)
    pub fn isEmpty(str: Option<&str>) -> Option<bool> {
      // java return str == null || str.length() == 0;
      return str.is_none() || (str.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) == 0;
    }

}
