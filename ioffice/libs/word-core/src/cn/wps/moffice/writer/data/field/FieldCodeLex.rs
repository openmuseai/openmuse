// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.writer.data.field.exception.UnKnownSwitchAgrument
use crate::cn::wps::moffice::writer::data::field::exception::UnKnownSwitchAgrument::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.Iterator
// Java: import java.util.Map
// Java: import cn.wps.moffice.writer.data.field.ParamScope
use crate::cn::wps::moffice::writer::data::field::ParamScope::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
use crate::cn::wps::moffice::writer::data::field::StrFormatUtil::StrFormatUtil;
use stubs::java::lang::Character::Character;
use stubs::java::lang::Math::Math;
use stubs::java::lang::StringBuilder::StringBuilder;
use std::collections::HashMap;

// java public static final int MAX_PICTURE_PARAM_SIZE = 64;
pub(crate) const MAX_PICTURE_PARAM_SIZE: i32 = 64;
// java public static final int MAX_PARAM_SIZE = 255;
pub(crate) const MAX_PARAM_SIZE: i32 = 255;
// java private static final String[] strLegalFormatString = new String[]{"caps", "firstcap", "upper", "lower", "sbchar", "dbchar", "firstcaponly"};
static strLegalFormatString: &[&str] =
                    &["caps", "firstcap", "upper", "lower", "sbchar", "dbchar", "firstcaponly"];
// java public static final int FORMATCODE_UNKNOWN = -1;
pub(crate) const FORMATCODE_UNKNOWN: i32 = -1;
// java public static final int FORMATCODE_NUMBER = 0;
pub(crate) const FORMATCODE_NUMBER: i32 = 0;
// java public static final int FORMATCODE_STRING = 1;
pub(crate) const FORMATCODE_STRING: i32 = 1;
// java public static final int FORMATCODE_MERGE = 2;
pub(crate) const FORMATCODE_MERGE: i32 = 2;
// java public static final int null_word = 0;
pub(crate) const null_word: i32 = 0;
// java public static final int switch_word = 1;
pub(crate) const switch_word: i32 = 1;
// java public static final int norm_word = 2;
pub(crate) const norm_word: i32 = 2;
// java public static final int fieldcode_err_toomany_picture = 1;
pub(crate) const fieldcode_err_toomany_picture: i32 = 1;
// java public static final int fieldcode_err_unrecognized_switch = 2;
pub(crate) const fieldcode_err_unrecognized_switch: i32 = 2;
// java public static final int fieldcode_err_no_param = 3;
pub(crate) const fieldcode_err_no_param: i32 = 3;
// java public static final int fieldcode_err_picture_format_not_first = 4;
pub(crate) const fieldcode_err_picture_format_not_first: i32 = 4;

// java public class FieldCodeLex
#[derive(Default)]
pub struct FieldCodeLex {
    // java private ArrayList<ParamScope> mParams = new ArrayList();
    mParams: Vec<ParamScope>,
    // java private HashMap<String, Boolean> mFormats = new HashMap();
    mFormats: HashMap<String, bool>,
    // java private HashMap<Character, String> mSwitchs = new HashMap();
    mSwitchs: HashMap<char, String>,
    // java private HashMap<Character, String> mPicture = new HashMap();
    mPicture: HashMap<char, String>,
    // java private HashMap<Character, Boolean> mPrivateSwitchs = new HashMap();
    mPrivateSwitchs: HashMap<char, bool>,
    // java private int mLastError = 0;
    mLastError: i32,
    // java private int mTotalLen = 0;
    mTotalLen: i32,
    // java private int mCurPos = 0;
    mCurPos: i32,
    // java private String mStrCode = null;
    mStrCode: Option<String>,
    // java private String mStrName = null;
    mStrName: Option<String>,
    // java private boolean mHasMergeFormat = false;
    mHasMergeFormat: bool,
}

// java final class FMT_CODE
#[derive(Default)]
pub struct FMT_CODE {
}

// java final class WORD_TYPE
#[derive(Default)]
pub struct WORD_TYPE {
}

// java static final class CODE_ERR
#[derive(Default)]
pub struct CODE_ERR {
}

// java public class FieldCodeLex
impl FieldCodeLex {
    // java public void clear()
    pub fn clear(&mut self) {
      // java this.mStrCode = null;
      self.mStrCode = None;
      // java this.mStrName = null;
      self.mStrName = None;
      // java this.mTotalLen = 0;
      self.mTotalLen = 0;
      // java this.mCurPos = 0;
      self.mCurPos = 0;
      // java this.mLastError = 0;
      self.mLastError = 0;
      // java this.mHasMergeFormat = false;
      self.mHasMergeFormat = false;
      // java this.mParams.clear();
      self.mParams.clear();
      // java this.mFormats.clear();
      self.mFormats.clear();
      // java this.mPicture.clear();
      self.mPicture.clear();
      // java this.mSwitchs.clear();
      self.mSwitchs.clear();
      // java this.mPrivateSwitchs.clear();
      self.mPrivateSwitchs.clear();
    }

    // java public void parse(char[] szCode, int nLen)
    pub fn parse(&mut self, szCode: &Vec<char>, nLen: i32) {
      // java this.mStrCode = new String(szCode) + " ";
      self.mStrCode = Some(szCode.iter().collect::<String>() + " ");
      // java this.mTotalLen = Math.min(szCode.length, nLen) + 1;
      self.mTotalLen = Math::min((szCode.len() as i32), nLen) + 1;
      // java StringBuilder strTmp = new StringBuilder();
      let mut strTmp: StringBuilder = StringBuilder::new();
      // java int nWordType = this._NextWord(strTmp);
      let mut nWordType: i32 = self._NextWord(&mut strTmp);
      // java char chLastSwitch = 0;
      let mut chLastSwitch: char = '\0';
      // java switch (nWordType)
      match nWordType {
        // java case 1:
        1 => {
          // java chLastSwitch = strTmp.charAt(0);
          chLastSwitch = strTmp.charAt(0);
        }
        // java case 2:
        2 => {
          // java this.mStrName = strTmp.toString();
          self.mStrName = Some(strTmp.toString());
        }
        // java default:
        _ => {
        }
      }
      // java boolean fContinue = true;
      let mut fContinue: bool = true;
      // java while (fContinue)
      while fContinue {
        // java int nStartPos = this.mCurPos;
        let mut nStartPos: i32 = self.mCurPos;
        // java nWordType = this._NextWord(strTmp);
        nWordType = self._NextWord(&mut strTmp);
        // java switch (nWordType)
        match nWordType {
          // java case 0:
          0 => {
            // java if (0 != chLastSwitch)
            if '\0' != chLastSwitch {
              // java if (0 == this.mLastError && (chLastSwitch == '*' || chLastSwitch == '@' || chLastSwitch == '#'))
              if 0 == self.mLastError && (chLastSwitch == '*' || chLastSwitch == '@' || chLastSwitch == '#') {
                // java this.mLastError = 3;
                self.mLastError = 3;
              }
              // java this.mSwitchs.insert(chLastSwitch, "");
              self.mSwitchs.insert(chLastSwitch, "".to_string());
            }
            // java fContinue = false;
            fContinue = false;
          }
          // java case 1:
          1 => {
            // java if (0 == this.mLastError)
            if 0 == self.mLastError {
              // java if ((strTmp.charAt(0) == '@' || strTmp.charAt(0) == '#') && this.mFormats.size() > 0)
              if (strTmp.charAt(0) == '@' || strTmp.charAt(0) == '#') && self.mFormats.len() > 0 {
                // java this.mLastError = 4;
                self.mLastError = 4;
              }
              // java else if ((chLastSwitch == '*' || chLastSwitch == '@' || chLastSwitch == '#') && 0 != chLastSwitch)
              else if (chLastSwitch == '*' || chLastSwitch == '@' || chLastSwitch == '#') && '\0' != chLastSwitch {
                // java this.mLastError = 3;
                self.mLastError = 3;
              }
              // java else if (chLastSwitch == strTmp.charAt(0))
              else if chLastSwitch == strTmp.charAt(0) {
                // java this.mLastError = 1;
                self.mLastError = 1;
              }
            }
            // java if (0 != chLastSwitch)
            if '\0' != chLastSwitch {
              // java this.mSwitchs.insert(chLastSwitch, "");
              self.mSwitchs.insert(chLastSwitch, "".to_string());
            }
            // java if (0 == this.mLastError && (strTmp.charAt(0) == '@' || strTmp.charAt(0) == '#') && this.mSwitchs.get(strTmp.charAt(0)) != null)
            if 0 == self.mLastError && (strTmp.charAt(0) == '@' || strTmp.charAt(0) == '#') && self.mSwitchs.get(&strTmp.charAt(0)).is_some() {
              // java this.mLastError = 1;
              self.mLastError = 1;
            }
            // java chLastSwitch = strTmp.charAt(0);
            chLastSwitch = strTmp.charAt(0);
          }
          // java case 2:
          2 => {
            // java if (0 != chLastSwitch && this._SwitchHasParam(chLastSwitch))
            if '\0' != chLastSwitch && self._SwitchHasParam(chLastSwitch) {
              // java if (chLastSwitch != '#' && chLastSwitch != '@')
              if chLastSwitch != '#' && chLastSwitch != '@' {
                // java if (chLastSwitch != '*')
                if chLastSwitch != '*' {
                  // java if (strTmp.length() > 255)
                  if (strTmp.length() as i32) > 255 {
                    // java strTmp.delete(255, strTmp.length());
                    strTmp.delete(255, (strTmp.length() as i32));
                  }
                  // java this.mSwitchs.insert(chLastSwitch, strTmp.toString());
                  self.mSwitchs.insert(chLastSwitch, strTmp.toString());
                }
                // java else
                else {
                  // java switch (this._ParseFormatString(strTmp))
                  match self._ParseFormatString(&strTmp) {
                    // java case 0:
                    0 => {
                      // java Iterator<Map.Entry<String, Boolean>> iter = this.mFormats.entrySet().iterator();
                      // java while (iter.hasNext())
                      // java if (!(Boolean)((Map.Entry)iter.next()).getValue())
                      // java iter.remove();
                      self.mFormats.retain(|_k, v| *v);
                      // java this.mFormats.insert(strTmp.toString(), false);
                      self.mFormats.insert(strTmp.toString(), false);
                    }
                    // java case 1:
                    1 => {
                      // java this.mFormats.insert(strTmp.toString(), true);
                      self.mFormats.insert(strTmp.toString(), true);
                    }
                    // java case -1: case 2: default:
                    -1 | 2 | _ => {
                    }
                  }
                }
              }
              // java else
              else {
                // java if (this.mPicture.size() > 0)
                if self.mPicture.len() > 0 {
                  // java this.mLastError = 1;
                  self.mLastError = 1;
                }
                // java int nSize = strTmp.length();
                let mut nSize: i32 = (strTmp.length() as i32);
                // java if (nSize > 64)
                if nSize > 64 {
                  // java strTmp.delete(64, nSize);
                  strTmp.delete(64, nSize);
                }
                // java this.mPicture.insert(chLastSwitch, strTmp.toString());
                self.mPicture.insert(chLastSwitch, strTmp.toString());
              }
            }
            // java else
            else {
              // java if (strTmp.length() > 255)
              if (strTmp.length() as i32) > 255 {
                // java strTmp.delete(255, strTmp.length());
                strTmp.delete(255, (strTmp.length() as i32));
              }
              // java this.mParams.add(new ParamScope(strTmp.toString(), nStartPos, this.mCurPos));
              self.mParams.push(ParamScope::ParamScope(Some(&strTmp.toString()), nStartPos, self.mCurPos));
              // java this.mSwitchs.insert(chLastSwitch, "");
              self.mSwitchs.insert(chLastSwitch, "".to_string());
            }
            // java chLastSwitch = 0;
            chLastSwitch = '\0';
          }
          // java default:
          _ => {
          }
        }
      }
    }

    // java  int _NextWord(StringBuilder strBuf)
    pub fn _NextWord(&mut self, strBuf: &mut StringBuilder) -> i32 {
      // java strBuf.setLength(0);
      strBuf.setLength(0);
      // java int nType = 0;
      let mut nType: i32 = 0;
      // java if (this.mCurPos >= this.mTotalLen - 1)
      if self.mCurPos >= self.mTotalLen - 1 {
        // java return nType;
        return nType;
      }
      // java else
      else {
        // java this._SkipSpace();
        self._SkipSpace();
        // java char ch = this._NextChar();
        let mut ch: char = self._NextChar();
        // java boolean fInQuote = false;
        let mut fInQuote: bool = false;
        // java if (0 != ch && !this._IsSpace(ch))
        if '\0' != ch && !self._IsSpace(ch) {
          // java if (ch == '=')
          if ch == '=' {
            // java nType = 2;
            nType = 2;
            // java strBuf.append(ch);
            strBuf.append(ch);
            // java return nType;
            return nType;
          }
          // java else
          else {
            // java if (this._IsShift(ch))
            if self._IsShift(ch) {
              // java ch = this._NextChar();
              ch = self._NextChar();
              // java if (this._IsSwitch(ch))
              if self._IsSwitch(ch) {
                // java nType = 1;
                nType = 1;
                // java strBuf.append(Character.toLowerCase(ch));
                strBuf.append(Character::toLowerCase(ch));
                // java return nType;
                return nType;
              }
              // java strBuf.append(ch);
              strBuf.append(ch);
            }
            // java else if (this._IsQuote(ch))
            else if self._IsQuote(ch) {
              // java fInQuote = true;
              fInQuote = true;
            }
            // java else
            else {
              // java strBuf.append(ch);
              strBuf.append(ch);
            }
            // java boolean fContinue = true;
            let mut fContinue: bool = true;
            // java nType = 2;
            nType = 2;
            // java while (fContinue)
            while fContinue {
              // java ch = this._NextChar();
              ch = self._NextChar();
              // java switch (ch)
              match ch {
                // java case '\u0000':
                '\u{0000}' => {
                  // java fContinue = false;
                  fContinue = false;
                }
                // java case ' ':
                ' ' => {
                  // java if (fInQuote)
                  if fInQuote {
                    // java strBuf.append(ch);
                    strBuf.append(ch);
                  }
                  // java else
                  else {
                    // java fContinue = false;
                    fContinue = false;
                  }
                }
                // java case '"': case '“': case '”':
                '"' | '“' | '”' => {
                  // java fContinue = false;
                  fContinue = false;
                  // java if (!fInQuote)
                  if !fInQuote {
                    // java this._StepBack();
                    self._StepBack();
                  }
                }
                // java case '\\':
                '\\' => {
                  // java if (!fInQuote)
                  if !fInQuote {
                    // java ch = this._NextChar();
                    ch = self._NextChar();
                    // java if (this._IsSwitch(ch))
                    if self._IsSwitch(ch) {
                      // java fContinue = false;
                      fContinue = false;
                      // java this._StepBack();
                      self._StepBack();
                      // java this._StepBack();
                      self._StepBack();
                    }
                    // java else if (this._IsShift(ch))
                    else if self._IsShift(ch) {
                      // java strBuf.append('\\');
                      strBuf.append('\\');
                    }
                    // java else if (strBuf.length() > 0 && strBuf.charAt(strBuf.length() - 1) == ':')
                    else if (strBuf.length() as i32) > 0 && strBuf.charAt((strBuf.length() as i32) - 1) == ':' {
                      // java strBuf.append(ch);
                      strBuf.append(ch);
                    }
                  }
                  // java else
                  else {
                    // java ch = this._NextChar();
                    ch = self._NextChar();
                    // java if (ch != 0)
                    if ch != '\0' {
                      // java strBuf.append(ch);
                      strBuf.append(ch);
                    }
                  }
                }
                // java default:
                _ => {
                  // java strBuf.append(ch);
                  strBuf.append(ch);
                }
              }
            }
            // java return nType;
            return nType;
          }
        }
        // java else
        else {
          // java return nType;
          return nType;
        }
      }
    }

    // java  int _ParseFormatString(StringBuilder strBuf)
    pub fn _ParseFormatString(&mut self, strBuf: &StringBuilder) -> i32 {
      // java String string = strBuf.toString();
      let mut string: String = strBuf.toString();
      // java if ("DollarText".equals(string))
      if (string == "DollarText") {
        // java return -1;
        return -1;
      }
      // java else
      else {
        let mut i: i32 = 0;
        // java for(int i = 0; i < strLegalFormatString.length; ++i)
        while i < (strLegalFormatString.len() as i32) {
          // java if (string.equalsIgnoreCase(strLegalFormatString[i]))
          if string.eq_ignore_ascii_case(strLegalFormatString[(i) as usize]) {
            // java return 1;
            return 1;
          }
          i += 1;
        }
        // java if (-1 != StrFormatUtil.txNfcString2MsoNfc(string))
        if -1 != StrFormatUtil::txNfcString2MsoNfc(string.as_mut_str()) {
          // java return 0;
          return 0;
        }
        // java else if (!string.equalsIgnoreCase("mergeformat") && !string.equalsIgnoreCase("mergeformatinet"))
        else if !string.eq_ignore_ascii_case("mergeformat") && !string.eq_ignore_ascii_case("mergeformatinet") {
          // java if (string.equalsIgnoreCase("charformat"))
          if string.eq_ignore_ascii_case("charformat") {
            // java return 2;
            return 2;
          }
          // java else
          else {
            // java throw new UnKnownSwitchAgrument();
            panic!("UnKnownSwitchAgrument");
          }
        }
        // java else
        else {
          // java this.mHasMergeFormat = true;
          self.mHasMergeFormat = true;
          // java return 2;
          return 2;
        }
      }
    }

    // java  boolean _SwitchHasParam(char ch)
    pub fn _SwitchHasParam(&self, ch: char) -> bool {
      // java if (ch != '@' && ch != '*' && ch != '#')
      if ch != '@' && ch != '*' && ch != '#' {
        // java char key = Character.toLowerCase(ch);
        let mut key: char = Character::toLowerCase(ch);
        // java Boolean b = (Boolean)this.mPrivateSwitchs.get(key);
        let b: Option<&bool> = self.mPrivateSwitchs.get(&key);
        // java return b != null ? b : false;
        return *b.unwrap_or(&false);
      }
      // java else
      else {
        // java return true;
        return true;
      }
    }

    // java  char _NextChar()
    pub fn _NextChar(&mut self) -> char {
      // java return this.mCurPos >= 0 && this.mCurPos < this.mTotalLen ? this.mStrCode.charAt(this.mCurPos++) : '\u0000';
      return (if self.mCurPos >= 0 && self.mCurPos < self.mTotalLen {
        let idx = self.mCurPos;
        self.mCurPos += 1;
        self.mStrCode.as_ref().map(|s| s.chars().nth(idx as usize).unwrap_or('\0')).unwrap_or('\0')
      } else {
        '\u{0000}'
      });
    }

    // java  boolean _StepBack()
    pub fn _StepBack(&mut self) -> bool {
      // java boolean fBacked = true;
      let mut fBacked: bool = true;
      // java if (this.mCurPos > 0)
      if self.mCurPos > 0 {
        // java --this.mCurPos;
        { let _tmp = self.mCurPos; self.mCurPos -= 1; _tmp };
      }
      // java else
      else {
        // java fBacked = false;
        fBacked = false;
      }
      // java return fBacked;
      return fBacked;
    }

    // java  boolean _IsShift(char ch)
    pub fn _IsShift(&self, ch: char) -> bool {
      // java return ch == '\\';
      return ch == '\\';
    }

    // java  boolean _IsQuote(char ch)
    pub fn _IsQuote(&self, ch: char) -> bool {
      // java return ch == '"' || ch == 8220 || ch == 8221;
      return ch == '"' || ch == char::from_u32(8220).unwrap() || ch == char::from_u32(8221).unwrap();
    }

    // java  boolean _IsSpace(char ch)
    pub fn _IsSpace(&self, ch: char) -> bool {
      // java return ch == '\n' || ch == '\r' || ch == ' ' || ch == 12288 || ch == 11 || ch == '\r' || ch == 160 || ch == '\f' || ch == 15 || ch == 14 || ch == 2 || ch == 5 || ch == '\b' || ch == 1 || ch == '\t';
      return ch == '\n' || ch == '\r' || ch == ' ' || ch == char::from_u32(12288).unwrap() || ch == char::from_u32(11).unwrap() || ch == '\r' || ch == char::from_u32(160).unwrap() || ch == '\u{000c}' || ch == char::from_u32(15).unwrap() || ch == char::from_u32(14).unwrap() || ch == char::from_u32(2).unwrap() || ch == char::from_u32(5).unwrap() || ch == '\u{0008}' || ch == char::from_u32(1).unwrap() || ch == '\t';
    }

    // java  boolean _IsSwitch(char ch)
    pub fn _IsSwitch(&self, ch: char) -> bool {
      // java return this._IsPrivateSwitch(ch) || ch == '*' || ch == '@' || ch == '#';
      return self._IsPrivateSwitch(ch) || ch == '*' || ch == '@' || ch == '#';
    }

    // java  boolean _IsPrivateSwitch(char ch)
    pub fn _IsPrivateSwitch(&self, ch: char) -> bool {
      // java char key = Character.toLowerCase(ch);
      let mut key: char = Character::toLowerCase(ch);
      // java return this.mPrivateSwitchs.get(key) != null;
      return self.mPrivateSwitchs.get(&key).is_some();
    }

    // java  void _SkipSpace()
    pub fn _SkipSpace(&mut self) {
      // java while (this._IsSpace(this._NextChar()))
      while {
        let ch = self._NextChar();
        self._IsSpace(ch)
      } {
      }
      // java if (this.mCurPos <= this.mTotalLen - 1)
      if self.mCurPos <= self.mTotalLen - 1 {
        // java this._StepBack();
        self._StepBack();
      }
    }

    // java public int getLastError()
    pub fn getLastError(&self) -> i32 {
      // java return this.mLastError;
      return self.mLastError;
    }

    // java public String getPicture(char[] ch)
    pub fn getPicture(&self, ch: Option<&mut Vec<char>>) -> String {
      // java Iterator<Map.Entry<Character, String>> iter = this.mPicture.entrySet().iterator();
      let mut iter = self.mPicture.iter();
      // java if (iter.hasNext())
      if let Some((key, value)) = iter.next() {
        // java Map.Entry<Character, String> entry = (Map.Entry)iter.next();
        // java if (ch != null)
        if let Some(ch) = ch {
          // java ch[0] = (Character)entry.getKey();
          if !ch.is_empty() {
            ch[0usize] = *key;
          }
        }
        // java return (String)entry.getValue();
        return value.clone();
      }
      // java else
      else {
        // java return "";
        return "".to_string();
      }
    }

    // java public boolean findSwitch(char ch, StringBuffer strSwitch)
    pub fn findSwitch(&self, ch: char, strSwitch: Option<&mut String>) -> bool {
      // java String string = (String)this.mSwitchs.get(Character.toLowerCase(ch));
      let string: Option<&String> = self.mSwitchs.get(&Character::toLowerCase(ch));
      // java if (string != null)
      if let Some(string) = string {
        // java if (strSwitch != null)
        if let Some(strSwitch) = strSwitch {
          // java strSwitch.append(string);
          strSwitch.push_str(string);
        }
        // java return true;
        return true;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public int getFormatCount()
    pub fn getFormatCount(&self) -> i32 {
      // java return this.mFormats.size();
      return self.mFormats.len() as i32;
    }

    // java public HashMap<String, Boolean> getFormats()
    pub fn getFormats(&self) -> &HashMap<String, bool> {
      // java return this.mFormats;
      return &self.mFormats;
    }

    // java public int getParamCount()
    pub fn getParamCount(&self) -> i32 {
      // java return this.mParams.size();
      return self.mParams.len() as i32;
    }

    // java public ParamScope getParam(int idx)
    pub fn getParam(&self, idx: i32) -> Option<&ParamScope> {
      // java return idx >= 0 && idx < this.mParams.size() ? (ParamScope)this.mParams.get(idx) : null;
      return (if idx >= 0 && idx < (self.mParams.len() as i32) { self.mParams.get(idx as usize) } else { None });
    }

    // java public String getName()
    pub fn getName(&self) -> Option<&str> {
      // java return this.mStrName;
      return self.mStrName.as_deref();
    }

    // java public int getFieldType()
    pub fn getFieldType(&self) -> i32 {
      // java StringBuffer buffer = new StringBuffer();
      let mut buffer: StringBuilder = StringBuilder::new();
      // java int i = 0;
      let mut i: i32 = 0;
      let name = self.mStrName.as_deref().unwrap_or("");
      let mut len: i32 = name.chars().count() as i32;
      // java for(int len = this.mStrName.length(); i < len; ++i)
      while i < len {
        // java if (this.mStrName.charAt(i) != 11 && this.mStrName.charAt(i) != '\r')
        let ch = name.chars().nth(i as usize).unwrap_or('\0');
        if ch != char::from_u32(11).unwrap() && ch != '\r' {
          // java buffer.append(this.mStrName.charAt(i));
          buffer.append(ch);
        }
        i += 1;
      }
      // java KFieldType result = KFieldType.valueOf(buffer.toString());
      let mut result: Option<KFieldType> = KFieldType::getKFieldType__String(Some(&buffer.toString()));
      // java Assert.assertNotNull(result);
      Assert::assertNotNull(&result);
      // java return result.getFlt();
      return result.unwrap().getFlt();
    }

    // java public void registerPrivateSwitch(char c, boolean hasParam)
    pub fn registerPrivateSwitch__char_boolean(&mut self, mut c: char, hasParam: bool) {
      // java c = Character.toLowerCase(c);
      c = Character::toLowerCase(c);
      // java this.mPrivateSwitchs.insert(c, hasParam);
      self.mPrivateSwitchs.insert(c, hasParam);
    }

    // java public void registerPrivateSwitch(char c)
    pub fn registerPrivateSwitch__char(&mut self, c: char) {
      // java this.registerPrivateSwitch(c, false);
      self.registerPrivateSwitch__char_boolean(c, false);
    }

    // java public boolean getMergeFormat()
    pub fn getMergeFormat(&self) -> bool {
      // java return this.mHasMergeFormat;
      return self.mHasMergeFormat;
    }

}

// java final class FMT_CODE
impl FMT_CODE {
}

// java final class WORD_TYPE
impl WORD_TYPE {
}

// java static final class CODE_ERR
impl CODE_ERR {
}
