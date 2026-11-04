use crate::kfc::xpxml::reader::XmlName2Id::XmlName2Id;
use crate::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;
use crate::kfc::xpxml::util::XmlValueBuilder::XmlValueBuilder;
use super::Mode::Mode;

// Java: public class HLLabelParser
pub struct HLLabelParser {
    // Java field: XmlName2Id xmlIds
    xmlIds: Box<dyn XmlName2Id>,
    // Java field: XmlRoAttrList mRoAttrs
    mRoAttrs: Option<XmlRoAttrList>,
    // Java field: int currentLabel
    pub(crate) currentLabel: i32,
    // Java field: int outerLabelPrefix
    outerLabelPrefix: i32,
    // Java field: Mode mMode
    mMode: Mode,
    // Java field: int labelPrefix
    labelPrefix: i32,
}

// Java: public class HLLabelParser
impl HLLabelParser {
    // Java: HLLabelParser(XmlName2Id xmlIds, Mode mode)
    pub fn new(xml_ids: Box<dyn XmlName2Id>, mode: Mode) -> Self {
        // Java: super();
        // Java: this.xmlIds = xmlIds;
        // Java: this.currentLabel = -1;
        // Java: this.mMode = mode;
        HLLabelParser {
            xmlIds: xml_ids,
            mRoAttrs: None,
            currentLabel: -1,
            outerLabelPrefix: -1,
            mMode: mode,
            labelPrefix: -1,
        }
    }

    // Java: void setOuterLabelPrefix(int outerLabelPrefix)
    pub fn set_outer_label_prefix(&mut self, outer_label_prefix: i32) {
        // Java: this.outerLabelPrefix = outerLabelPrefix;
        self.outerLabelPrefix = outer_label_prefix;
    }

    // Java: void setRoAttr(XmlRoAttrList roAttrs)
    pub fn set_ro_attr(&mut self, ro_attrs: XmlRoAttrList) {
        // Java: this.mRoAttrs = roAttrs;
        self.mRoAttrs = Some(ro_attrs);
    }

    pub fn reset_attrs(&mut self) {
        if let Some(attrs) = self.mRoAttrs.as_mut() {
            attrs.reset();
        }
    }

    pub fn take_attrs(&mut self) -> XmlRoAttrList {
        self.mRoAttrs.take().unwrap_or_else(XmlRoAttrList::new)
    }

    // Java: int getLabel()
    pub fn get_label(&self) -> i32 {
        // Java: return this.currentLabel;
        self.currentLabel
    }

    // Java: void parseLabel(char[] chars, int start, int end, int type)
    pub fn parse_label(&mut self, chars: &[char], start: i32, end: i32, label_type: i32) {
        // Java: ++start;
        let mut start = (start + 1) as usize;
        // Java: if (type == 4)
        if label_type == 4 {
            // Java: ++start;
            start += 1;
        }
        // Java: --end;
        let mut end = (end - 1) as usize;
        // Java: if (type == 2 || type == 3)
        if label_type == 2 || label_type == 3 {
            // Java: --end;
            end -= 1;
        }
        // Java: if (start > end)
        if start > end {
            // Java: start = end;
            start = end;
        }
        // Java: if (type != 0 && type != 2 && type != 4)
        if label_type != 0 && label_type != 2 && label_type != 4 {
            // Java: if (type == 1 || type == 3)
            if label_type == 1 || label_type == 3 {
                // Java: int index = getNextEmptySpace(chars, start, end);
                let index = Self::get_next_empty_space(chars, start, end);
                // Java: this.currentLabel = this.getIdByString(chars, start, index - 1, false);
                self.currentLabel = self.get_id_by_string(chars, start, (index as i32 - 1) as usize, false);
                // Java: if (this.currentLabel == -1)
                if self.currentLabel == -1 {
                    // Java: return;
                    return;
                }
                // Java: int space;
                // Java: for(int var9 = passEmptySpace(chars, index, end); var9 < end; var9 = passEmptySpace(chars, space, end))
                let mut var9 = Self::pass_empty_space(chars, index as usize, end);
                while var9 < end {
                    // Java: space = getNextEmptySpaceV2(chars, var9, end);
                    let space = Self::get_next_empty_space_v2(chars, var9, end);
                    // Java: if (space == -1)
                    if space == -1 {
                        // Java: this.parseAttr(chars, var9, end);
                        self.parse_attr(chars, var9, end);
                        // Java: break;
                        break;
                    }
                    // Java: this.parseAttr(chars, var9, space - 1);
                    self.parse_attr(chars, var9, (space as usize) - 1);
                    var9 = Self::pass_empty_space(chars, space as usize, end);
                }
            }
        }
        // Java: else
        else {
            // Java: this.currentLabel = this.getIdByString(chars, start, end, false);
            self.currentLabel = self.get_id_by_string(chars, start, end, false);
        }
    }

    // Java: void parseAttr(char[] chars, int start, int end)
    pub fn parse_attr(&mut self, chars: &[char], start: usize, end: usize) {
        // Java: int equal = getNextChar(chars, start, end, '=');
        let equal = Self::get_next_char(chars, start, end, '=');
        // Java: if (equal != -1)
        if equal != -1 {
            let equal = equal as usize;
            let mut end = end;
            // Java: while(isEmptyChar(chars[end]) && end > equal)
            while Self::is_empty_char(chars[end]) && end > equal {
                // Java: --end;
                end -= 1;
            }
            // Java: if (chars[equal + 1] == '\'' && chars[end] == '\'' || chars[equal + 1] == '"' && chars[end] == '"')
            if (chars[equal + 1] == '\'' && chars[end] == '\'') || (chars[equal + 1] == '"' && chars[end] == '"') {
                // Java: int attrId = this.getIdByString(chars, start, equal - 1, true);
                let attr_id = self.get_id_by_string(chars, start, equal - 1, true);
                // Java: if (attrId == -1)
                if attr_id == -1 {
                    // Java: return;
                    return;
                }
                // Java: String value = buildValue(chars, equal + 2, end - 1, true);
                let value = Self::build_value(chars, equal + 2, if end > 0 { end - 1 } else { 0 }, true);
                // Java: this.mRoAttrs.addAttr(attrId, value);
                if let Some(ref mut ro_attrs) = self.mRoAttrs {
                    ro_attrs.add_attr(attr_id, &value);
                }
            }
        }
    }

    // Java: int getIdByString(char[] chars, int start, int end, boolean isAttr)
    fn get_id_by_string(&mut self, chars: &[char], start: usize, end: usize, is_attr: bool) -> i32 {
        // Java: int pos = XmlValueBuilder.index(chars, start, end + 1, ':');
        let pos = XmlValueBuilder::index_chars(chars, start, end + 1, ':');
        // Java: return Mode.uof == this.mMode ? this.getIdByStringUof(chars, start, end, pos) : this.getIdByStringNormal(chars, start, end, isAttr, pos);
        if self.mMode == Mode::Uof {
            self.get_id_by_string_uof(chars, start, end, pos)
        } else {
            self.get_id_by_string_normal(chars, start, end, is_attr, pos)
        }
    }

    // Java: int getIdByStringUof(char[] chars, int start, int end, int pos)
    fn get_id_by_string_uof(&self, chars: &[char], start: usize, end: usize, pos: i32) -> i32 {
        // Java: int uofIdSep = XmlValueBuilder.index(chars, start, end + 1, '_');
        let uof_id_sep = XmlValueBuilder::index_chars(chars, start, end + 1, '_');
        // Java: if (-1 != uofIdSep)
        if uof_id_sep != -1 {
            // Java: Integer result = XmlValueBuilder.parserHexToInt(chars, uofIdSep + 1, end + 1, 16);
            let result = XmlValueBuilder::parser_hex_to_int(chars, (uof_id_sep + 1) as usize, end + 1, 16);
            // Java: if (null != result)
            if let Some(r) = result {
                // Java: return result;
                return r;
            }
        }
        // Java: return pos == -1 ? XmlValueBuilder.hashcode(chars, start, end + 1) : XmlValueBuilder.hashcode(chars, pos + 1, end + 1);
        if pos == -1 {
            XmlValueBuilder::hashcode_chars(chars, start, end + 1)
        } else {
            XmlValueBuilder::hashcode_chars(chars, (pos + 1) as usize, end + 1)
        }
    }

    // Java: int getIdByStringNormal(char[] chars, int start, int end, boolean isAttr, int pos)
    fn get_id_by_string_normal(&mut self, chars: &[char], start: usize, end: usize, is_attr: bool, pos: i32) -> i32 {
        // Java: if (isAttr)
        if is_attr {
            // Java: if (pos == -1)
            if pos == -1 {
                // Java: int name = XmlValueBuilder.hashcode(chars, start, end + 1);
                let name = XmlValueBuilder::hashcode_chars(chars, start, end + 1);
                // Java: return this.xmlIds.getId(this.labelPrefix, name);
                return self.xmlIds.get_id(self.labelPrefix, name);
            }
            // Java: else
            else {
                // Java: int prefix = XmlValueBuilder.hashcode(chars, start, pos);
                let prefix = XmlValueBuilder::hashcode_chars(chars, start, pos as usize);
                // Java: int name = XmlValueBuilder.hashcode(chars, pos + 1, end + 1);
                let name = XmlValueBuilder::hashcode_chars(chars, (pos + 1) as usize, end + 1);
                // Java: return this.xmlIds.getId(prefix, name);
                return self.xmlIds.get_id(prefix, name);
            }
        }
        // Java: else if (pos == -1)
        else if pos == -1 {
            // Java: this.labelPrefix = this.outerLabelPrefix;
            self.labelPrefix = self.outerLabelPrefix;
            // Java: int name = XmlValueBuilder.hashcode(chars, start, end + 1);
            let name = XmlValueBuilder::hashcode_chars(chars, start, end + 1);
            // Java: return this.xmlIds.getId(this.labelPrefix, name);
            return self.xmlIds.get_id(self.labelPrefix, name);
        }
        // Java: else
        else {
            // Java: this.labelPrefix = XmlValueBuilder.hashcode(chars, start, pos);
            self.labelPrefix = XmlValueBuilder::hashcode_chars(chars, start, pos as usize);
            // Java: int name = XmlValueBuilder.hashcode(chars, pos + 1, end + 1);
            let name = XmlValueBuilder::hashcode_chars(chars, (pos + 1) as usize, end + 1);
            // Java: return this.xmlIds.getId(this.labelPrefix, name);
            return self.xmlIds.get_id(self.labelPrefix, name);
        }
    }

    // Java: static int getNextEmptySpace(char[] chars, int start, int end)
    pub fn get_next_empty_space(chars: &[char], start: usize, end: usize) -> i32 {
        // Java: for(int index = start; index <= end; ++index)
        for index in start..=end {
            // Java: if (isEmptyChar(chars[index]))
            if Self::is_empty_char(chars[index]) {
                // Java: return index;
                return index as i32;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: static int getNextEmptySpaceV2(char[] chars, int start, int end)
    pub fn get_next_empty_space_v2(chars: &[char], start: usize, end: usize) -> i32 {
        // Java: boolean bInSingleQuote = false;
        let mut b_in_single_quote = false;
        // Java: boolean bInDoubleQuote = false;
        let mut b_in_double_quote = false;
        // Java: for(int index = start; index <= end; ++index)
        for index in start..=end {
            // Java: switch (chars[index])
            match chars[index] {
                // Java: case '\t':
                // Java: case '\n':
                // Java: case '\r':
                // Java: case ' ':
                '\t' | '\n' | '\r' | ' ' => {
                    // Java: if (!bInDoubleQuote && !bInSingleQuote)
                    if !b_in_double_quote && !b_in_single_quote {
                        // Java: return index;
                        return index as i32;
                    }
                    // Java: break;
                }
                // Java: case '"':
                '"' => {
                    // Java: if (!bInSingleQuote)
                    if !b_in_single_quote {
                        // Java: bInDoubleQuote = !bInDoubleQuote;
                        b_in_double_quote = !b_in_double_quote;
                    }
                    // Java: break;
                }
                // Java: case '\'':
                '\'' => {
                    // Java: if (!bInDoubleQuote)
                    if !b_in_double_quote {
                        // Java: bInSingleQuote = !bInSingleQuote;
                        b_in_single_quote = !b_in_single_quote;
                    }
                }
                _ => {}
            }
        }
        // Java: return -1;
        -1
    }

    // Java: static int getNextChar(char[] chars, int start, int end, char ch)
    pub fn get_next_char(chars: &[char], start: usize, end: usize, ch: char) -> i32 {
        // Java: for(int index = start; index <= end; ++index)
        for index in start..=end {
            // Java: if (chars[index] == ch)
            if chars[index] == ch {
                // Java: return index;
                return index as i32;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: static int passEmptySpace(char[] chars, int start, int end)
    pub fn pass_empty_space(chars: &[char], start: usize, end: usize) -> usize {
        // Java: for(int index = start; index <= end; ++index)
        for index in start..=end {
            // Java: char ch = chars[index];
            let ch = chars[index];
            // Java: if (!isEmptyChar(ch))
            if !Self::is_empty_char(ch) {
                // Java: return index;
                return index;
            }
        }
        // Java: return end + 1;
        end + 1
    }

    // Java: static String buildValue(char[] chars, int start, int end, boolean bEscape)
    pub fn build_value(chars: &[char], start: usize, end: usize, b_escape: bool) -> String {
        // Java: if (start > end)
        if start > end {
            // Java: return "";
            return String::new();
        }
        // Java: else if (bEscape && getNextChar(chars, start, end, '&') != -1)
        if b_escape && Self::get_next_char(chars, start, end, '&') != -1 {
            // Java: StringBuilder builder = new StringBuilder(end - start + 1);
            let mut builder = String::with_capacity(end - start + 1);
            // Java: for(int index = start; index <= end; ++index)
            let mut index = start;
            while index <= end {
                // Java: if (chars[index] != '&')
                if chars[index] != '&' {
                    // Java: builder.append(chars[index]);
                    builder.push(chars[index]);
                }
                // Java: else if (isAMP(chars, index + 1, end))
                else if Self::is_amp(chars, index + 1, end) {
                    // Java: builder.append('&');
                    builder.push('&');
                    // Java: index += 4;
                    index += 4;
                }
                // Java: else if (isLT(chars, index + 1, end))
                else if Self::is_lt(chars, index + 1, end) {
                    // Java: builder.append('<');
                    builder.push('<');
                    // Java: index += 3;
                    index += 3;
                }
                // Java: else if (isGT(chars, index + 1, end))
                else if Self::is_gt(chars, index + 1, end) {
                    // Java: builder.append('>');
                    builder.push('>');
                    // Java: index += 3;
                    index += 3;
                }
                // Java: else if (isQUOT(chars, index + 1, end))
                else if Self::is_quot(chars, index + 1, end) {
                    // Java: builder.append('"');
                    builder.push('"');
                    // Java: index += 5;
                    index += 5;
                }
                // Java: else if (isAPOS(chars, index + 1, end))
                else if Self::is_apos(chars, index + 1, end) {
                    // Java: builder.append('\'');
                    builder.push('\'');
                    // Java: index += 5;
                    index += 5;
                }
                // Java: else
                else {
                    // Java: int e = getNextChar(chars, index + 1, end, ';');
                    let e = Self::get_next_char(chars, index + 1, end, ';');
                    // Java: if (e != -1)
                    if e != -1 {
                        let e = e as usize;
                        // Java: if (chars[index + 1] != '#')
                        if chars[index + 1] != '#' {
                            // Java: throw new IllegalStateException(new String(chars, start, end - start + 1));
                            panic!("IllegalStateException: {}",chars[start..=end].iter().collect::<String>());
                        }
                        // Java: if (chars[index + 2] == 'x')
                        else if chars[index + 2] == 'x' {
                            // Java: String tmp = new String(chars, index + 3, e - (index + 3));
                            let tmp: String = chars[index + 3..e].iter().collect();
                            // Java: char ch = (char)Integer.parseInt(tmp, 16);
                            let code=crate::native_runtime::DecimalInteger::parseIntRadix(&tmp.encode_utf16().collect::<Vec<_>>(),16).expect("NumberFormatException");
                            // Java casts the parsed int to one char, including truncation.
                            let ch=char::from_u32(code as u16 as u32).expect("Unported isolated UTF16 HL value");
                            // Java: builder.append(ch);
                            builder.push(ch);
                            // Java: index = e;
                            index = e;
                        }
                        // Java: else
                        else {
                            // Java: String tmp = new String(chars, index + 2, e - (index + 2));
                            let tmp: String = chars[index + 2..e].iter().collect();
                            // Java: char ch = (char)Integer.parseInt(tmp);
                            let code=crate::native_runtime::DecimalInteger::parseInt(&tmp.encode_utf16().collect::<Vec<_>>()).expect("NumberFormatException");
                            let ch=char::from_u32(code as u16 as u32).expect("Unported isolated UTF16 HL value");
                            // Java: builder.append(ch);
                            builder.push(ch);
                            // Java: index = e;
                            index = e;
                        }
                    }
                    // Java: else
                    else {
                        // Java: builder.append(chars[index]);
                        builder.push(chars[index]);
                    }
                }
                index += 1;
            }
            // Java: return builder.toString();
            return builder;
        }
        // Java: else
        // Java: return new String(chars, start, end - start + 1);
        chars[start..=end].iter().collect()
    }

    // Java: static boolean isAMP(char[] chars, int start, int end)
    fn is_amp(chars: &[char], start: usize, end: usize) -> bool {
        // Java: return end - start >= 3 && chars[start] == 'a' && chars[start + 1] == 'm' && chars[start + 2] == 'p' && chars[start + 3] == ';';
        end >= start + 3 && chars[start] == 'a' && chars[start + 1] == 'm' && chars[start + 2] == 'p' && chars[start + 3] == ';'
    }

    // Java: static boolean isLT(char[] chars, int start, int end)
    fn is_lt(chars: &[char], start: usize, end: usize) -> bool {
        // Java: return end - start >= 2 && chars[start] == 'l' && chars[start + 1] == 't' && chars[start + 2] == ';';
        end >= start + 2 && chars[start] == 'l' && chars[start + 1] == 't' && chars[start + 2] == ';'
    }

    // Java: static boolean isGT(char[] chars, int start, int end)
    fn is_gt(chars: &[char], start: usize, end: usize) -> bool {
        // Java: return end - start >= 2 && chars[start] == 'g' && chars[start + 1] == 't' && chars[start + 2] == ';';
        end >= start + 2 && chars[start] == 'g' && chars[start + 1] == 't' && chars[start + 2] == ';'
    }

    // Java: static boolean isQUOT(char[] chars, int start, int end)
    fn is_quot(chars: &[char], start: usize, end: usize) -> bool {
        // Java: return end - start >= 4 && chars[start] == 'q' && chars[start + 1] == 'u' && chars[start + 2] == 'o' && chars[start + 3] == 't' && chars[start + 4] == ';';
        end >= start + 4 && chars[start] == 'q' && chars[start + 1] == 'u' && chars[start + 2] == 'o' && chars[start + 3] == 't' && chars[start + 4] == ';'
    }

    // Java: static boolean isAPOS(char[] chars, int start, int end)
    fn is_apos(chars: &[char], start: usize, end: usize) -> bool {
        // Java: return end - start >= 4 && chars[start] == 'a' && chars[start + 1] == 'p' && chars[start + 2] == 'o' && chars[start + 3] == 's' && chars[start + 4] == ';';
        end >= start + 4 && chars[start] == 'a' && chars[start + 1] == 'p' && chars[start + 2] == 'o' && chars[start + 3] == 's' && chars[start + 4] == ';'
    }

    // Java: static boolean isEmptyChar(char ch)
    pub fn is_empty_char(ch: char) -> bool {
        // Java: return ch == ' ' || ch == '\r' || ch == '\n' || ch == '\t';
        ch == ' ' || ch == '\r' || ch == '\n' || ch == '\t'
    }
}
