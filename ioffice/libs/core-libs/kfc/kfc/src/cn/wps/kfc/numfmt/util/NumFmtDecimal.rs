// Java: import cn.wps.kfc.numfmt.format.NumFmtSection
#[allow(unused_imports)] use crate::cn::wps::kfc::numfmt::format::NumFmtSection;
// Java: import java.util.ArrayList
// Java: import java.util.List

// Java: public class NumFmtDecimal
pub struct NumFmtDecimal {
    // Java field: int mDecimalIndex
    pub(crate) mDecimalIndex: i32,
    // Java field: boolean mDefaultChar
    pub(crate) mDefaultChar: bool,
    // Java field: StringBuffer mFormatCode
    pub(crate) mFormatCode: String,
    // Java field: NumFmtCategorizer categorizer
    categorizer: i32,
    // Java field: List<Integer> mValidCharList
    mValidCharList: Vec<i32>,
}

// Java: public class NumFmtDecimal
impl NumFmtDecimal {
    // Java: NumFmtDecimal()
    pub fn new() -> Self {
        // Java: super();
        NumFmtDecimal {
            mDecimalIndex: -1,
            mDefaultChar: false,
            mFormatCode: String::new(),
            categorizer: 0,
            mValidCharList: Vec::new(),
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mDecimalIndex = -1;
        self.mDecimalIndex = -1;
        // Java: this.mDefaultChar = false;
        self.mDefaultChar = false;
        // Java: this.mFormatCode.setLength(0);
        self.mFormatCode.clear();
        // Java: this.mValidCharList.clear();
        self.mValidCharList.clear();
    }

    // Java: String changeAllDecimalDigits(String fullFormatCode, int offset)
    pub fn changeAllDecimalDigits(&mut self, _full_format_code: &str, _offset: i32) -> String {
        // Java: String[] codeParts = new String[4];
        // Java: NumFmtSection.splitGroup(fullFormatCode, codeParts);
        // Java: StringBuffer buffer = new StringBuffer();
        // Java: for(String part : codeParts)
        {
            // Java: if (part != null)
            {
                // Java: short partCodeType = this.categorizer.getNumfmtCat(part, null);
                // Java: buffer.append(this.changeDecimalDigits(part, offset, partCodeType));
                // Java: buffer.append(';');
            }
        }
        // Java: if (buffer.length() > 0)
        {
            // Java: buffer.deleteCharAt(buffer.length() - 1);
            // Java: return buffer.toString();
        }
        // Java: else
        {
            // Java: return fullFormatCode;
        }
        todo!("NumFmtDecimal::changeAllDecimalDigits")
    }

    // Java: String changeDecimalDigits(String formatPart, int offset, short numfmtType)
    pub fn changeDecimalDigits(&mut self, _format_part: &str, _offset: i32, _numfmt_type: i16) -> String {
        // Java: this.clear();
        // Java: if (validDecimalDigit(numfmtType) && formatPart != null && formatPart.length() != 0)
        {
            // Java: if (numfmtType == 1)
            {
                // Java: formatPart = "0";
            }
            // Java: this.mFormatCode.append(formatPart);
            // Java: if (this.colletInfo(formatPart) && offset < 0)
            {
                // Java: return formatPart;
            }
            // Java: else
            {
                // Java: this.alterDecimalDigit(offset);
                // Java: return this.mFormatCode.toString();
            }
        }
        // Java: else
        {
            // Java: return formatPart;
        }
        todo!("NumFmtDecimal::changeDecimalDigits")
    }

    // Java: static boolean validDecimalDigit(short numfmtType)
    pub fn validDecimalDigit(_numfmt_type: i16) -> bool {
        // Java: return numfmtType != 0 && numfmtType != 8193 && numfmtType != 8194 && numfmtType != 8192 && numfmtType != 8 && numfmtType != 64 && numfmtType != -1;
        todo!("NumFmtDecimal::validDecimalDigit")
    }

    // Java: boolean colletInfo(String formatCode)
    pub fn colletInfo(&mut self, _format_code: &str) -> bool {
        // Java: int speChar = -1;
        // Java: int validChar = -1;
        // Java: int scienceIndex = -1;
        // Java: int squareIndex = -1;
        // Java: boolean isInSquare = false;
        // Java: boolean isInRef = false;
        // Java: for(int i = 0; i < formatCode.length(); ++i)
        {
            // Java: char c = formatCode.charAt(i);
            // Java: boolean isMetaChar = !isInRef && !isInSquare;
            // Java: switch (c)
            {
                // Java: case '!':
                // Java: case '*':
                // Java: case '\\':
                // Java: case '_':
                    // Java: if (speChar == -1 && validChar != -1)
                    {
                        // Java: speChar = i;
                    }
                    // Java: ++i;
                    // Java: isMetaChar = false;
                    // Java: break;
                // Java: case '"':
                    // Java: isInRef = !isInRef;
                    // Java: isMetaChar = false;
                    // Java: if (speChar == -1 && validChar != -1)
                    {
                        // Java: speChar = i;
                    }
                    // Java: break;
                // Java: case 'E':
                    // Java: if (isInRef || isInSquare)
                    {
                        // Java: break;
                    }
                    // Java: scienceIndex = i;
                // Java: case ' ':
                // Java: case '%':
                // Java: case '(':
                // Java: case ')':
                    // Java: if (speChar == -1 && validChar != -1)
                    {
                        // Java: speChar = i;
                    }
                    // Java: isMetaChar = false;
                    // Java: break;
                // Java: case '[':
                    // Java: if (!isInRef)
                    {
                        // Java: isInSquare = true;
                        // Java: isMetaChar = false;
                        // Java: squareIndex = i;
                    }
                    // Java: break;
                // Java: case ']':
                    // Java: if (!isInRef)
                    {
                        // Java: isInSquare = false;
                    }
            }
            // Java: if (c == '.' && !isInRef && !isInSquare)
            {
                // Java: this.mDecimalIndex = i;
            }
            // Java: if (isMetaChar && scienceIndex == -1)
            {
                // Java: this.mValidCharList.add(i);
                // Java: validChar = i;
            }
        }
        // Java: if (this.mDecimalIndex == -1)
        {
            // Java: if (squareIndex != -1)
            {
                // Java: speChar = this.matchSquare(speChar, squareIndex);
            }
            // Java: if (speChar != -1)
            {
                // Java: if (validChar > speChar)
                {
                    // Java: this.mDecimalIndex = formatCode.length();
                }
                // Java: else
                {
                    // Java: this.mDecimalIndex = speChar;
                }
            }
            // Java: else
            {
                // Java: this.mDecimalIndex = formatCode.length();
            }
            // Java: this.mFormatCode.insert(this.mDecimalIndex, '.');
            // Java: this.mDefaultChar = true;
            // Java: if (scienceIndex != -1)
            {
                // Java: ++scienceIndex;
            }
            // Java: return true;
        }
        // Java: else
        {
            // Java: return false;
        }
        todo!("NumFmtDecimal::colletInfo")
    }

    // Java: int matchSquare(int speIndex, int squareIndex)
    pub fn matchSquare(&self, _spe_index: i32, _square_index: i32) -> i32 {
        // Java: boolean afterSquare = (Integer)this.mValidCharList.get(this.mValidCharList.size() - 1) > squareIndex;
        // Java: if (afterSquare)
        {
            // Java: return speIndex;
        }
        // Java: else
        {
            // Java: return speIndex != -1 && speIndex < squareIndex ? speIndex : squareIndex;
        }
        todo!("NumFmtDecimal::matchSquare")
    }

    // Java: void alterDecimalDigit(int offset)
    pub fn alterDecimalDigit(&mut self, _offset: i32) {
        // Java: if (offset > 0)
        {
            // Java: this.addDecimalCount(offset);
        }
        // Java: else
        {
            // Java: this.subDecimalCount(offset);
        }
        todo!("NumFmtDecimal::alterDecimalDigit")
    }

    // Java: void addDecimalCount(int offset)
    pub fn addDecimalCount(&mut self, _offset: i32) {
        // Java: int validIndex = (Integer)this.mValidCharList.get(this.mValidCharList.size() - 1);
        // Java: int insertIndex = validIndex <= this.mDecimalIndex - 1 ? this.mDecimalIndex : validIndex;
        // Java: char c = this.mFormatCode.charAt(validIndex);
        // Java: if (this.mDefaultChar)
        {
            // Java: c = '0';
        }
        // Java: this.mFormatCode.insert(insertIndex + 1, NumFmtUtil.fillChar(c, offset));
        todo!("NumFmtDecimal::addDecimalCount")
    }

    // Java: void subDecimalCount(int offset)
    pub fn subDecimalCount(&mut self, _offset: i32) {
        // Java: int signCount = 0;
        // Java: int len = this.mValidCharList.size();
        // Java: int charPointer = -1;
        // Java: for(int i = len - 1; i >= 0; --i)
        {
            // Java: ++signCount;
            // Java: if (signCount >= -offset || (Integer)this.mValidCharList.get(i) == this.mDecimalIndex)
            {
                // Java: int j;
                // Java: for(j = 0; j < signCount; ++j)
                {
                    // Java: charPointer = (Integer)this.mValidCharList.get(len - j - 1);
                    // Java: this.mFormatCode.replace(charPointer, charPointer + 1, "");
                }
                // Java: if ((Integer)this.mValidCharList.get(len - j - 1) == this.mDecimalIndex)
                {
                    // Java: this.mFormatCode.replace(this.mDecimalIndex, this.mDecimalIndex + 1, "");
                }
                // Java: break;
            }
        }
        // Java: this.mFormatCode = combineQutation(this.mFormatCode);
        todo!("NumFmtDecimal::subDecimalCount")
    }

    // Java: static StringBuffer combineQutation(StringBuffer formatCode)
    pub fn combineQutation(_format_code: String) -> String {
        // Java: int indexQutation = formatCode.indexOf("\"\"");
        // Java: if (indexQutation != -1)
        {
            // Java: formatCode.replace(indexQutation, indexQutation + 2, "");
        }
        // Java: return formatCode;
        todo!("NumFmtDecimal::combineQutation")
    }
}
