// Java: import cn.wps.kfc.xpxml.util.XmlValueBuilder
use crate::kfc::xpxml::util::XmlValueBuilder::XmlValueBuilder;
// Java: import java.io.IOException
// Java: import java.util.Vector

// Java: public interface IRun
pub trait IRun {
    // Java: abstract void run(char var1)
    fn run(&self, parser: &mut CompactCoreParser, ch: char);
}

// Java: public class CompactCoreParser
pub struct CompactCoreParser {
    // Java field: CompactInnerAccepter _pAcpt
    pub(crate) _pAcpt: Box<dyn super::CompactInnerAccepter::CompactInnerAccepter>,
    // Java field: int _lxs
    pub(crate) _lxs: i32,
    // Java field: XmlValueBuilder _identifier
    pub(crate) _identifier: XmlValueBuilder,
    // Java field: XmlValueBuilder _esqSaveId
    _esqSaveId: Option<XmlValueBuilder>,
    // Java field: int _esqSaveState
    _esqSaveState: i32,
    // Java field: XmlValueBuilder _cdataText
    pub(crate) _cdataText: XmlValueBuilder,
    // Java field: Vector<IRun> operlist
    operlist: Vec<Box<dyn IRun>>,
    // Java field: char _beginValueFlag
    pub(crate) _beginValueFlag: char,
}

// Java: public class LXS_Invalid
pub struct LXS_Invalid;

// Java: public class LXS_LT
pub struct LXS_LT;

// Java: public class LXS_LTA
pub struct LXS_LTA;

// Java: public class LXS_HT_DORL
pub struct LXS_HT_DORL;

// Java: public class LXS_HT_GT
pub struct LXS_HT_GT;

// Java: public class LXS_EB_IDT
pub struct LXS_EB_IDT;

// Java: public class LXS_EB_IDTE
pub struct LXS_EB_IDTE;

// Java: public class LXS_EB_GT
pub struct LXS_EB_GT;

// Java: public class LXS_EE_IDT
pub struct LXS_EE_IDT;

// Java: public class LXS_EE_IDTC
pub struct LXS_EE_IDTC;

// Java: public class LXS_EE_IDTE
pub struct LXS_EE_IDTE;

// Java: public class LXS_AN_IDT
pub struct LXS_AN_IDT;

// Java: public class LXS_AN_IDTE
pub struct LXS_AN_IDTE;

// Java: public class LXS_AV_BQ
pub struct LXS_AV_BQ;

// Java: public class LXS_AV_VT
pub struct LXS_AV_VT;

// Java: public class LXS_VT
pub struct LXS_VT;

// Java: public class LXS_ESQ
pub struct LXS_ESQ;

// Java: public class LXS_CDATA
pub struct LXS_CDATA;

// Java: public class CompactCoreParser
impl CompactCoreParser {
    // Java: CompactCoreParser(CompactInnerAccepter pAcpt)
    pub fn new(pAcpt: Box<dyn super::CompactInnerAccepter::CompactInnerAccepter>) -> Self {
      // Java: super();
      // Java: this._pAcpt = pAcpt;
      // Java: this._lxs = 1;
      // Java: this.operlist = new Vector(18);
      let mut operlist: Vec<Box<dyn IRun>> = Vec::with_capacity(18);
      // Java: this.operlist.add(new LXS_Invalid());
      operlist.push(Box::new(LXS_Invalid));
      // Java: this.operlist.add(new LXS_LT());
      operlist.push(Box::new(LXS_LT));
      // Java: this.operlist.add(new LXS_LTA());
      operlist.push(Box::new(LXS_LTA));
      // Java: this.operlist.add(new LXS_HT_DORL());
      operlist.push(Box::new(LXS_HT_DORL));
      // Java: this.operlist.add(new LXS_HT_GT());
      operlist.push(Box::new(LXS_HT_GT));
      // Java: this.operlist.add(new LXS_EB_IDT());
      operlist.push(Box::new(LXS_EB_IDT));
      // Java: this.operlist.add(new LXS_EB_IDTE());
      operlist.push(Box::new(LXS_EB_IDTE));
      // Java: this.operlist.add(new LXS_EB_GT());
      operlist.push(Box::new(LXS_EB_GT));
      // Java: this.operlist.add(new LXS_EE_IDT());
      operlist.push(Box::new(LXS_EE_IDT));
      // Java: this.operlist.add(new LXS_EE_IDTC());
      operlist.push(Box::new(LXS_EE_IDTC));
      // Java: this.operlist.add(new LXS_EE_IDTE());
      operlist.push(Box::new(LXS_EE_IDTE));
      // Java: this.operlist.add(new LXS_AN_IDT());
      operlist.push(Box::new(LXS_AN_IDT));
      // Java: this.operlist.add(new LXS_AN_IDTE());
      operlist.push(Box::new(LXS_AN_IDTE));
      // Java: this.operlist.add(new LXS_AV_BQ());
      operlist.push(Box::new(LXS_AV_BQ));
      // Java: this.operlist.add(new LXS_AV_VT());
      operlist.push(Box::new(LXS_AV_VT));
      // Java: this.operlist.add(new LXS_VT());
      operlist.push(Box::new(LXS_VT));
      // Java: this.operlist.add(new LXS_ESQ());
      operlist.push(Box::new(LXS_ESQ));
      // Java: this.operlist.add(new LXS_CDATA());
      operlist.push(Box::new(LXS_CDATA));
      CompactCoreParser {
          _pAcpt: pAcpt,
          _lxs: 1,
          _identifier: XmlValueBuilder::new(),
          _esqSaveId: None,
          _esqSaveState: 0,
          _cdataText: XmlValueBuilder::new(),
          operlist,
          _beginValueFlag: '"',
      }
    }

    // Java: void reset(CompactInnerAccepter pAcpt)
    pub fn reset_accepter(&mut self, pAcpt: Box<dyn super::CompactInnerAccepter::CompactInnerAccepter>) {
      // Java: this._pAcpt = pAcpt;
      self._pAcpt = pAcpt;
    }

    // Java: void parse(char ch)
    pub fn parse(&mut self, ch: char) {
      // Java: ((IRun)this.operlist.get(this._lxs)).run(ch);
      todo!()
    }

    // Java: void parse(char[] buf, int len)
    pub fn parse_buf(&mut self, buf: &[char], len: usize) {
      // Java: for(int idx = 0; idx < len; ++idx)
      for idx in 0..len
      {
         // Java: ((IRun)this.operlist.get(this._lxs)).run(buf[idx]);
         todo!()
      }
    }

    // Java: void _beginEsq()
    pub fn _beginEsq(&mut self) {
      // Java: this._esqSaveState = this._lxs;
      self._esqSaveState = self._lxs;
      // Java: this._esqSaveId = this._identifier.backup();
      self._esqSaveId = Some(self._identifier.backup());
      // Java: this._identifier.clear();
      self._identifier.clear();
      // Java: this._lxs = 16;
      self._lxs = 16;
    }

    // Java: String _parseNCR(String ncr)
    pub fn _parseNCR(&self, ncr: &str) -> String {
      // Java: StringBuilder sb = new StringBuilder();
      let mut sb = String::new();
      // Java: String[] splits = ncr.split("&#");
      let splits: Vec<&str> = ncr.split("&#").collect();
      // Java: if (splits == null)
      if splits.is_empty()
      {
         // Java: return ncr;
         return ncr.to_string();
      }
      // Java: else
      else
      {
         // Java: for(int i = 0; i < splits.length; ++i)
         for i in 0..splits.len()
         {
            // Java: String c = this._parseNumber(splits[i]);
            let c = self._parseNumber(splits[i]);
            // Java: sb.append(c);
            sb.push_str(&c);
         }
         // Java: return sb.toString();
         return sb;
      }
    }

    // Java: String _parseNCRX(String ncr)
    pub fn _parseNCRX(&self, ncr: &str) -> String {
      // Java: StringBuilder sb = new StringBuilder();
      let mut sb = String::new();
      // Java: String[] splits = ncr.split(";");
      let splits: Vec<&str> = ncr.split(";").collect();
      // Java: if (splits == null)
      if splits.is_empty()
      {
         // Java: return ncr;
         return ncr.to_string();
      }
      // Java: else
      else
      {
         // Java: for(int i = 0; i < splits.length; ++i)
         for i in 0..splits.len()
         {
            // Java: String c = this._parseNumberX(splits[i]);
            let c = self._parseNumberX(splits[i]);
            // Java: sb.append(c);
            sb.push_str(&c);
         }
         // Java: return sb.toString();
         return sb;
      }
    }

    // Java: boolean _isdigit(char c)
    pub fn _isdigit(&self, c: char) -> bool {
      // Java: return c >= '0' && c <= '9';
      c >= '0' && c <= '9'
    }

    // Java: boolean _isdigitX(char c)
    pub fn _isdigitX(&self, c: char) -> bool {
      // Java: if (c >= '0' && c <= '9')
      if c >= '0' && c <= '9'
      {
         // Java: return true;
         return true;
      }
      // Java: else if (c >= 'a' && c <= 'f')
      else if c >= 'a' && c <= 'f'
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return c >= 'A' && c <= 'F';
         return c >= 'A' && c <= 'F';
      }
    }

    // Java: String _parseNumber(String ncr)
    pub fn _parseNumber(&self, ncr: &str) -> String {
      // Java: StringBuilder sb = new StringBuilder();
      let mut sb = String::new();
      // Java: int length = ncr.length();
      let length = ncr.len();
      // Java: int index;
      let mut index;
      // Java: for(index = 0; index < length; ++index)
      index = 0;
      while index < length
      {
         // Java: char c = ncr.charAt(index);
         let c = ncr.chars().nth(index).unwrap();
         // Java: if (!this._isdigit(c))
         if !self._isdigit(c)
         {
            // Java: break;
            break;
         }
         index += 1;
      }
      // Java: if (index == length)
      if index == length
      {
         // Java: char c = (char)Integer.parseInt(ncr);
         if let Ok(n) = ncr.parse::<u32>() {
             if let Some(c) = char::from_u32(n) {
                 // Java: sb.append(c);
                 sb.push(c);
             }
         }
      }
      // Java: else if (index > 0)
      else if index > 0
      {
         // Java: char c = (char)Integer.parseInt(ncr.substring(0, index));
         if let Ok(n) = ncr[..index].parse::<u32>() {
             if let Some(c) = char::from_u32(n) {
                 // Java: sb.append(c);
                 sb.push(c);
             }
         }
         // Java: sb.append(ncr.substring(index));
         sb.push_str(&ncr[index..]);
      }
      // Java: else
      else
      {
         // Java: sb.append("&#");
         sb.push_str("&#");
         // Java: sb.append(ncr.substring(index));
         sb.push_str(&ncr[index..]);
      }
      // Java: return sb.toString();
      sb
    }

    // Java: String _parseNumberX(String ncr)
    pub fn _parseNumberX(&self, ncr: &str) -> String {
      // Java: int length = ncr.length();
      let length = ncr.len();
      // Java: if (length < 4)
      if length < 4
      {
         // Java: return ncr;
         return ncr.to_string();
      }
      // Java: else
      else
      {
         // Java: int index = 0;
         let mut index;
         // Java: index = ncr.indexOf("&#x");
         index = match ncr.find("&#x") { Some(i) => i as i32, None => -1 };
         // Java: StringBuilder sb = new StringBuilder();
         let mut sb = String::new();
         // Java: if (index < 0)
         if index < 0
         {
            // Java: return ncr;
            return ncr.to_string();
         }
         // Java: else
         else
         {
            // Java: if (index > 0)
            if index > 0
            {
               // Java: for(int i = 0; i < index; ++i)
               for i in 0..(index as usize)
               {
                  // Java: sb.append(ncr.charAt(i));
                  sb.push(ncr.chars().nth(i).unwrap());
               }
            }
            // Java: int bIndex = index + 3;
            let bIndex = (index + 3) as usize;
            // Java: for(index = bIndex; index < length; ++index)
            let mut idx = bIndex;
            while idx < length
            {
               // Java: char c = ncr.charAt(index);
               let c = ncr.chars().nth(idx).unwrap();
               // Java: if (!this._isdigitX(c))
               if !self._isdigitX(c)
               {
                  // Java: break;
                  break;
               }
               idx += 1;
            }
            // Java: if (index == length)
            if idx == length
            {
               // Java: if (bIndex < length)
               if bIndex < length
               {
                  // Java: String x = ncr.substring(bIndex);
                  let x = &ncr[bIndex..];
                  // Java: char c = (char)Integer.parseInt(x, 16);
                  if let Ok(n) = u32::from_str_radix(x, 16) {
                      if let Some(c) = char::from_u32(n) {
                          // Java: sb.append(c);
                          sb.push(c);
                      }
                  }
               }
            }
            // Java: else if (bIndex < index)
            else if bIndex < idx
            {
               // Java: char c = (char)Integer.parseInt(ncr.substring(bIndex, index), 16);
               if let Ok(n) = u32::from_str_radix(&ncr[bIndex..idx], 16) {
                   if let Some(c) = char::from_u32(n) {
                       // Java: sb.append(c);
                       sb.push(c);
                   }
               }
               // Java: sb.append(ncr.substring(index));
               sb.push_str(&ncr[idx..]);
            }
            // Java: return sb.toString();
            return sb;
         }
      }
    }

    // Java: void _endEsq()
    pub fn _endEsq(&mut self) {
      // Java: char chEsq = ' ';
      let mut chEsq = ' ';
      // Java: String tmp = this._identifier.toString();
      let tmp = self._identifier.to_string();
      // Java: if (tmp.equals("amp"))
      if tmp == "amp"
      {
         // Java: chEsq = '&';
         chEsq = '&';
      }
      // Java: else if (tmp.equals("lt"))
      else if tmp == "lt"
      {
         // Java: chEsq = '<';
         chEsq = '<';
      }
      // Java: else if (tmp.equals("gt"))
      else if tmp == "gt"
      {
         // Java: chEsq = '>';
         chEsq = '>';
      }
      // Java: else if (tmp.equals("quot"))
      else if tmp == "quot"
      {
         // Java: chEsq = '"';
         chEsq = '"';
      }
      // Java: else if (tmp.equals("apos"))
      else if tmp == "apos"
      {
         // Java: chEsq = '\'';
         chEsq = '\'';
      }
      // Java: else if (!tmp.equals("#xA") && !tmp.equals("#x0A"))
      else if tmp != "#xA" && tmp != "#x0A"
      {
         // Java: if (!tmp.equals("#xD") && !tmp.equals("#x0D"))
         if tmp != "#xD" && tmp != "#x0D"
         {
            // Java: if (tmp.length() > 1 && tmp.charAt(0) == '#' && tmp.charAt(1) == 'x')
            if tmp.len() > 1 && tmp.starts_with("#x")
            {
               // Java: tmp = "&" + tmp;
               let tmp2 = format!("&{}", tmp);
               // Java: tmp = this._parseNCRX(tmp);
               let tmp2 = self._parseNCRX(&tmp2);
               // Java: this._identifier.restore(this._esqSaveId);
               if let Some(saved) = self._esqSaveId.as_ref() { self._identifier.restore(saved); }
               // Java: this._identifier.append(tmp);
               self._identifier.append_str(&tmp2);
               // Java: this._lxs = this._esqSaveState;
               self._lxs = self._esqSaveState;
               // Java: return;
               return;
            }
            // Java: if (tmp.length() > 0 && tmp.charAt(0) == '#')
            if !tmp.is_empty() && tmp.starts_with('#')
            {
               // Java: tmp = tmp.substring(1);
               let tmp2 = &tmp[1..];
               // Java: tmp = this._parseNCR(tmp);
               let tmp2 = self._parseNCR(tmp2);
               // Java: this._identifier.restore(this._esqSaveId);
               if let Some(saved) = self._esqSaveId.as_ref() { self._identifier.restore(saved); }
               // Java: this._identifier.append(tmp);
               self._identifier.append_str(&tmp2);
               // Java: this._lxs = this._esqSaveState;
               self._lxs = self._esqSaveState;
               // Java: return;
               return;
            }
            // Java: this._identifier.restore(this._esqSaveId);
            if let Some(saved) = self._esqSaveId.as_ref() { self._identifier.restore(saved); }
            // Java: this._identifier.append('&');
            self._identifier.append_char('&');
            // Java: this._identifier.append(tmp);
            self._identifier.append_str(&tmp);
            // Java: this._lxs = this._esqSaveState;
            self._lxs = self._esqSaveState;
            // Java: return;
            return;
         }
         // Java: chEsq = '\r';
         chEsq = '\r';
      }
      // Java: else
      else
      {
         // Java: chEsq = '\n';
         chEsq = '\n';
      }
      // Java: this._identifier.restore(this._esqSaveId);
      if let Some(saved) = self._esqSaveId.as_ref() { self._identifier.restore(saved); }
      // Java: this._identifier.append(chEsq);
      self._identifier.append_char(chEsq);
      // Java: this._lxs = this._esqSaveState;
      self._lxs = self._esqSaveState;
    }

    // Java: boolean _isSpace(char ch)
    pub fn _isSpace(&self, ch: char) -> bool {
      // Java: return ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n';
      ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n'
    }

    // Java: boolean _isIdfHdr(char ch)
    pub fn _isIdfHdr(&self, ch: char) -> bool {
      // Java: return ch >= 'A' && ch <= 'Z' || ch >= 'a' && ch <= 'z' || ch == '_';
      (ch >= 'A' && ch <= 'Z') || (ch >= 'a' && ch <= 'z') || ch == '_'
    }

    // Java: boolean _isIdf(char ch)
    pub fn _isIdf(&self, ch: char) -> bool {
      // Java: return this._isIdfHdr(ch) || ch >= '0' && ch <= '9' || ch == '-' || ch == ':';
      self._isIdfHdr(ch) || (ch >= '0' && ch <= '9') || ch == '-' || ch == ':'
    }

    // Java: boolean _isValueChar(char ch)
    pub fn _isValueChar(&self, ch: char) -> bool {
      // Java: return ch != '<' && ch != '&';
      ch != '<' && ch != '&'
    }

    // Java: void reset(XmlValueBuilder sb, char ch)
    pub fn reset_builder(sb: &mut XmlValueBuilder, ch: char) {
      // Java: if (sb.length() > 0)
      if sb.length() > 0
      {
         // Java: sb.clear();
         sb.clear();
      }
      // Java: sb.append(ch);
      sb.append_char(ch);
    }

}

// Java: public class LXS_Invalid
impl LXS_Invalid {
    // Java: LXS_Invalid()
    pub fn new() -> Self {
         // Java: super();
         LXS_Invalid
    }

    // Java: void run(char ch)
    pub fn run_char(&self, _parser: &mut CompactCoreParser, _ch: char) {
    }

}

impl IRun for LXS_Invalid {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_LT
impl LXS_LT {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (ch == '<')
         if ch == '<'
         {
            // Java: CompactCoreParser.this._lxs = 2;
            parser._lxs = 2;
         }
         // Java: else if (ch == '&')
         else if ch == '&'
         {
            // Java: CompactCoreParser.this._identifier.clear();
            parser._identifier.clear();
            // Java: CompactCoreParser.this._lxs = 15;
            parser._lxs = 15;
            // Java: CompactCoreParser.this._beginEsq();
            parser._beginEsq();
         }
         // Java: else
         else
         {
            // Java: if (!CompactCoreParser.this._isValueChar(ch))
            if !parser._isValueChar(ch)
            {
               // Java: throw new IllegalStateException("LXS_LT");
               panic!("LXS_LT");
            }
            // Java: CompactCoreParser.this.reset(CompactCoreParser.this._identifier, ch);
            CompactCoreParser::reset_builder(&mut parser._identifier, ch);
            // Java: CompactCoreParser.this._lxs = 15;
            parser._lxs = 15;
         }
    }

}

impl IRun for LXS_LT {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_LTA
impl LXS_LTA {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (ch == '?')
         if ch == '?'
         {
            // Java: CompactCoreParser.this._lxs = 3;
            parser._lxs = 3;
         }
         // Java: else if (CompactCoreParser.this._isIdfHdr(ch))
         else if parser._isIdfHdr(ch)
         {
            // Java: CompactCoreParser.this.reset(CompactCoreParser.this._identifier, ch);
            CompactCoreParser::reset_builder(&mut parser._identifier, ch);
            // Java: CompactCoreParser.this._lxs = 5;
            parser._lxs = 5;
         }
         // Java: else if (ch == '/')
         else if ch == '/'
         {
            // Java: CompactCoreParser.this._lxs = 8;
            parser._lxs = 8;
         }
         // Java: else if (ch == '!')
         else if ch == '!'
         {
            // Java: CompactCoreParser.this.reset(CompactCoreParser.this._cdataText, '<');
            CompactCoreParser::reset_builder(&mut parser._cdataText, '<');
            // Java: CompactCoreParser.this._cdataText.append(ch);
            parser._cdataText.append_char(ch);
            // Java: CompactCoreParser.this._lxs = 17;
            parser._lxs = 17;
         }
         // Java: else if (!CompactCoreParser.this._isSpace(ch))
         else if !parser._isSpace(ch)
         {
            // Java: CompactCoreParser.this._lxs = 5;
            parser._lxs = 5;
         }
    }

}

impl IRun for LXS_LTA {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_HT_DORL
impl LXS_HT_DORL {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (ch == '?')
         if ch == '?'
         {
            // Java: CompactCoreParser.this._lxs = 4;
            parser._lxs = 4;
         }
    }

}

impl IRun for LXS_HT_DORL {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_HT_GT
impl LXS_HT_GT {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (ch == '>')
         if ch == '>'
         {
            // Java: CompactCoreParser.this._lxs = 1;
            parser._lxs = 1;
         }
         // Java: else if (!CompactCoreParser.this._isSpace(ch))
         else if !parser._isSpace(ch)
         {
            // Java: throw new IllegalStateException("LXS_HT_GT");
            panic!("LXS_HT_GT");
         }
    }

}

impl IRun for LXS_HT_GT {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_EB_IDT
impl LXS_EB_IDT {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (CompactCoreParser.this._isIdf(ch))
         if parser._isIdf(ch)
         {
            // Java: CompactCoreParser.this._identifier.append(ch);
            parser._identifier.append_char(ch);
         }
         // Java: else if (CompactCoreParser.this._isSpace(ch))
         else if parser._isSpace(ch)
         {
            // Java: CompactCoreParser.this._pAcpt.beginElement(CompactCoreParser.this._identifier);
            parser._pAcpt.beginElement(&parser._identifier.to_string());
            // Java: CompactCoreParser.this._lxs = 6;
            parser._lxs = 6;
         }
         // Java: else if (ch == '/')
         else if ch == '/'
         {
            // Java: CompactCoreParser.this._pAcpt.beginElement(CompactCoreParser.this._identifier);
            parser._pAcpt.beginElement(&parser._identifier.to_string());
            // Java: CompactCoreParser.this._pAcpt.endElement();
            parser._pAcpt.endElement();
            // Java: CompactCoreParser.this._lxs = 7;
            parser._lxs = 7;
         }
         // Java: else if (ch == '>')
         else if ch == '>'
         {
            // Java: CompactCoreParser.this._pAcpt.beginElement(CompactCoreParser.this._identifier);
            parser._pAcpt.beginElement(&parser._identifier.to_string());
            // Java: CompactCoreParser.this._lxs = 1;
            parser._lxs = 1;
         }
    }

}

impl IRun for LXS_EB_IDT {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_EB_IDTE
impl LXS_EB_IDTE {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (CompactCoreParser.this._isIdfHdr(ch))
         if parser._isIdfHdr(ch)
         {
            // Java: CompactCoreParser.this.reset(CompactCoreParser.this._identifier, ch);
            CompactCoreParser::reset_builder(&mut parser._identifier, ch);
            // Java: CompactCoreParser.this._lxs = 11;
            parser._lxs = 11;
         }
         // Java: else if (ch == '/')
         else if ch == '/'
         {
            // Java: CompactCoreParser.this._pAcpt.endElement();
            parser._pAcpt.endElement();
            // Java: CompactCoreParser.this._lxs = 7;
            parser._lxs = 7;
         }
         // Java: else if (ch == '>')
         else if ch == '>'
         {
            // Java: CompactCoreParser.this._lxs = 1;
            parser._lxs = 1;
         }
         // Java: else if (!CompactCoreParser.this._isSpace(ch))
         else if !parser._isSpace(ch)
         {
            // Java: throw new IllegalStateException("LXS_EB_IDTE");
            panic!("LXS_EB_IDTE");
         }
    }

}

impl IRun for LXS_EB_IDTE {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_EB_GT
impl LXS_EB_GT {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (ch == '>')
         if ch == '>'
         {
            // Java: CompactCoreParser.this._lxs = 1;
            parser._lxs = 1;
         }
         // Java: else if (!CompactCoreParser.this._isSpace(ch))
         else if !parser._isSpace(ch)
         {
            // Java: throw new IllegalStateException("LXS_EB_GT");
            panic!("LXS_EB_GT");
         }
    }

}

impl IRun for LXS_EB_GT {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_EE_IDT
impl LXS_EE_IDT {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (CompactCoreParser.this._isIdfHdr(ch))
         if parser._isIdfHdr(ch)
         {
            // Java: CompactCoreParser.this.reset(CompactCoreParser.this._identifier, ch);
            CompactCoreParser::reset_builder(&mut parser._identifier, ch);
            // Java: CompactCoreParser.this._lxs = 9;
            parser._lxs = 9;
         }
         // Java: else if (!CompactCoreParser.this._isSpace(ch))
         else if !parser._isSpace(ch)
         {
            // Java: CompactCoreParser.this._lxs = 9;
            parser._lxs = 9;
         }
    }

}

impl IRun for LXS_EE_IDT {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_EE_IDTC
impl LXS_EE_IDTC {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (CompactCoreParser.this._isIdf(ch))
         if parser._isIdf(ch)
         {
            // Java: CompactCoreParser.this._identifier.append(ch);
            parser._identifier.append_char(ch);
         }
         // Java: else if (CompactCoreParser.this._isSpace(ch))
         else if parser._isSpace(ch)
         {
            // Java: CompactCoreParser.this._pAcpt.endElement();
            parser._pAcpt.endElement();
            // Java: CompactCoreParser.this._lxs = 10;
            parser._lxs = 10;
         }
         // Java: else if (ch == '>')
         else if ch == '>'
         {
            // Java: CompactCoreParser.this._pAcpt.endElement();
            parser._pAcpt.endElement();
            // Java: CompactCoreParser.this._lxs = 1;
            parser._lxs = 1;
         }
    }

}

impl IRun for LXS_EE_IDTC {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_EE_IDTE
impl LXS_EE_IDTE {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (ch == '>')
         if ch == '>'
         {
            // Java: CompactCoreParser.this._lxs = 1;
            parser._lxs = 1;
         }
         // Java: else
         else
         {
            // Java: throw new IllegalStateException("LXS_EE_IDTE");
            panic!("LXS_EE_IDTE");
         }
    }

}

impl IRun for LXS_EE_IDTE {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_AN_IDT
impl LXS_AN_IDT {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (CompactCoreParser.this._isIdf(ch))
         if parser._isIdf(ch)
         {
            // Java: CompactCoreParser.this._identifier.append(ch);
            parser._identifier.append_char(ch);
         }
         // Java: else if (CompactCoreParser.this._isSpace(ch))
         else if parser._isSpace(ch)
         {
            // Java: CompactCoreParser.this._pAcpt.addAttribName(CompactCoreParser.this._identifier);
            parser._pAcpt.addAttribName(&parser._identifier.to_string());
            // Java: CompactCoreParser.this._identifier.clear();
            parser._identifier.clear();
            // Java: CompactCoreParser.this._lxs = 12;
            parser._lxs = 12;
         }
         // Java: else
         else
         {
            // Java: if (ch != '=')
            if ch != '='
            {
               // Java: throw new IllegalStateException("LXS_AN_IDT");
               panic!("LXS_AN_IDT");
            }
            // Java: CompactCoreParser.this._pAcpt.addAttribName(CompactCoreParser.this._identifier);
            parser._pAcpt.addAttribName(&parser._identifier.to_string());
            // Java: CompactCoreParser.this._identifier.clear();
            parser._identifier.clear();
            // Java: CompactCoreParser.this._lxs = 13;
            parser._lxs = 13;
         }
    }

}

impl IRun for LXS_AN_IDT {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_AN_IDTE
impl LXS_AN_IDTE {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (ch == '=')
         if ch == '='
         {
            // Java: CompactCoreParser.this._lxs = 13;
            parser._lxs = 13;
         }
         // Java: else if (!CompactCoreParser.this._isSpace(ch))
         else if !parser._isSpace(ch)
         {
            // Java: throw new IllegalStateException("LXS_AN_IDTE");
            panic!("LXS_AN_IDTE");
         }
    }

}

impl IRun for LXS_AN_IDTE {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_AV_BQ
impl LXS_AV_BQ {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (ch != '"' && ch != '\'')
         if ch != '"' && ch != '\''
         {
            // Java: if (!CompactCoreParser.this._isSpace(ch))
            if !parser._isSpace(ch)
            {
               // Java: throw new IllegalStateException("LXS_AV_BQ");
               panic!("LXS_AV_BQ");
            }
         }
         // Java: else
         else
         {
            // Java: CompactCoreParser.this._beginValueFlag = ch;
            parser._beginValueFlag = ch;
            // Java: CompactCoreParser.this._lxs = 14;
            parser._lxs = 14;
         }
    }

}

impl IRun for LXS_AV_BQ {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_AV_VT
impl LXS_AV_VT {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (CompactCoreParser.this._beginValueFlag == ch)
         if parser._beginValueFlag == ch
         {
            // Java: CompactCoreParser.this._pAcpt.addAttribValue(CompactCoreParser.this._identifier.toString());
            let val = parser._identifier.to_string();
            parser._pAcpt.addAttribValue(&val);
            // Java: CompactCoreParser.this._lxs = 6;
            parser._lxs = 6;
         }
         // Java: else if (ch == '&')
         else if ch == '&'
         {
            // Java: CompactCoreParser.this._beginEsq();
            parser._beginEsq();
         }
         // Java: else
         else
         {
            // Java: CompactCoreParser.this._identifier.append(ch);
            parser._identifier.append_char(ch);
         }
    }

}

impl IRun for LXS_AV_VT {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_VT
impl LXS_VT {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (ch == '<')
         if ch == '<'
         {
            // Java: CompactCoreParser.this._pAcpt.addText(CompactCoreParser.this._identifier.toString());
            let val = parser._identifier.to_string();
            parser._pAcpt.addText(&val);
            // Java: CompactCoreParser.this._lxs = 2;
            parser._lxs = 2;
         }
         // Java: else if (ch == '&')
         else if ch == '&'
         {
            // Java: CompactCoreParser.this._beginEsq();
            parser._beginEsq();
         }
         // Java: else
         else
         {
            // Java: if (!CompactCoreParser.this._isValueChar(ch))
            if !parser._isValueChar(ch)
            {
               // Java: throw new IllegalStateException("LXS_VT");
               panic!("LXS_VT");
            }
            // Java: CompactCoreParser.this._identifier.append(ch);
            parser._identifier.append_char(ch);
         }
    }

}

impl IRun for LXS_VT {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_ESQ
impl LXS_ESQ {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: if (ch != ';')
         if ch != ';'
         {
            // Java: if (ch != '<' && ch != '"')
            if ch != '<' && ch != '"'
            {
               // Java: CompactCoreParser.this._identifier.append(ch);
               parser._identifier.append_char(ch);
            }
            // Java: else
            else
            {
               // Java: CompactCoreParser.this._endEsq();
               parser._endEsq();
               // Java: CompactCoreParser.this.parse(ch);
               parser.parse(ch);
            }
         }
         // Java: else
         else
         {
            // Java: CompactCoreParser.this._endEsq();
            parser._endEsq();
         }
    }

}

impl IRun for LXS_ESQ {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}

// Java: public class LXS_CDATA
impl LXS_CDATA {
    // Java: void run(char ch)
    pub fn run_char(&self, parser: &mut CompactCoreParser, ch: char) {
         // Java: CompactCoreParser.this._cdataText.append(ch);
         parser._cdataText.append_char(ch);
         // Java: if (ch == '>')
         if ch == '>'
         {
            // Java: if (CompactCoreParser.this._cdataText.isCDATAEnd())
            if parser._cdataText.is_cdata_end()
            {
               // Java: CompactCoreParser.this._pAcpt.addText(CompactCoreParser.this._cdataText.toCDATAString());
               let val = parser._cdataText.to_cdata_string();
               parser._pAcpt.addText(&val);
               // Java: CompactCoreParser.this._lxs = 1;
               parser._lxs = 1;
            }
            // Java: else if (CompactCoreParser.this._cdataText.isAnnotationEnd())
            else if parser._cdataText.is_annotation_end()
            {
               // Java: CompactCoreParser.this._lxs = 1;
               parser._lxs = 1;
            }
         }
    }

}

impl IRun for LXS_CDATA {
    fn run(&self, parser: &mut CompactCoreParser, ch: char) { self.run_char(parser, ch); }
}
