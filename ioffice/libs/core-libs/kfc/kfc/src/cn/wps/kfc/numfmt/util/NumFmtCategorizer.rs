// Java: import cn.wps.kfc.numfmt.format.NumFmtSection
#[allow(unused_imports)] use crate::cn::wps::kfc::numfmt::format::NumFmtSection;
// Java: import cn.wps.kfc.numfmt.placeholder.ConstPieceDic
// use crate::cn::wps::kfc::numfmt::placeholder::ConstPieceDic;
// Java: import cn.wps.kfc.numfmt.placeholder.PieceContext
#[allow(unused_imports)] use crate::cn::wps::kfc::numfmt::placeholder::PieceContext;
// Java: import cn.wps.kfc.numfmt.placeholder.PlaceHolderPiece
#[allow(unused_imports)] use crate::cn::wps::kfc::numfmt::placeholder::PlaceHolderPiece;
// Java: import java.util.ArrayList
#[allow(unused_imports)] use std::collections::HashMap;

// Java: public class NumFmtCategorizer
pub struct NumFmtCategorizer {
    // Java field: ArrayList<PlaceHolderPiece> pieces
    pieces: Vec<i32>,
    // Java field: ConstPieceDic mConstPieceDic
    mConstPieceDic: i32,
    // Java field: PieceContext mPieceContext
    mPieceContext: i32,
    // Java field: NumFmtSection mSection
    mSection: i32,
}

// Java field: String F800
const F800: &str = "$-F800";
// Java field: String F400
const F400: &str = "$-F400";

// Java: public class NumFmtCategorizer
impl NumFmtCategorizer {
    // Java: NumFmtCategorizer()
    pub fn new() -> Self {
        // Java: super();
        NumFmtCategorizer { pieces: Vec::new(), mConstPieceDic: 0, mPieceContext: 0, mSection: 0 }
    }

    // Java: void reset(String formatStr)
    pub fn reset(&mut self, _format_str: &str) {
        // Java: this.pieces.clear();
        // Java: this.mPieceContext.clear();
        // Java: this.mSection.reset(formatStr);
        todo!("NumFmtCategorizer::reset")
    }

    // Java: short getNumfmtCat(String formatStr, Object value)
    pub fn getNumfmtCat(&mut self, _format_str: &str, _value: i32) -> i16 {
        // Java: this.reset(formatStr);
        // Java: short type = 0;
        // Java: String formatSection = formatStr;
        // Java: if (this.mSection.getFmtCount() > 1)
        {
            // Java: if (value instanceof Number)
            {
                // Java: formatSection = this.mSection.getFormatSection((Double)value);
            }
            // Java: else
            {
                // Java: formatSection = this.mSection.getPosCode();
            }
        }
        // Java: this.collection(formatSection);
        // Java: type = this.getMainType(this.mPieceContext.mPieceType);
        // Java: if (type == 8192)
        {
            // Java: return this.distinguishDateTimeFmt(this.pieces, formatStr);
        }
        // Java: else
        {
            // Java: return type == 512 ? this.distinguishCurrencyFmt(formatStr) : type;
        }
        todo!("NumFmtCategorizer::getNumfmtCat")
    }

    // Java: void collection(String formatPart)
    pub fn collection(&mut self, _format_part: &str) {
        // Java: boolean ignoreCase = false;
        // Java: int i = 0;
        // Java: for(int length = formatPart.length(); i < length; ++i)
        {
            // Java: PlaceHolderPiece piece = this.mConstPieceDic.searchPiece(formatPart.charAt(i));
            // Java: int len = piece.identify(formatPart, i, formatPart.length(), ignoreCase, this.mPieceContext);
            // Java: i += len - 1;
            // Java: if (ignoreCase && len == 0)
            {
                // Java: ++i;
                // Java: ignoreCase = false;
            }
            // Java: else
            {
                // Java: ignoreCase = len == 0;
            }
            // Java: if (piece.mHolderToken != null)
            {
                // Java: this.pieces.add(piece);
            }
        }
        todo!("NumFmtCategorizer::collection")
    }

    // Java: short getMainType(short multiType)
    pub fn getMainType(&self, _multi_type: i16) -> i16 {
        // Java: if ((multiType & 8192) > 0)
        {
            // Java: return 8192;
        }
        // Java: else if ((multiType & 8) > 0)
        {
            // Java: return 8;
        }
        // Java: else if ((multiType & 16) > 0)
        {
            // Java: return 16;
        }
        // Java: else if ((multiType & 4) > 0)
        {
            // Java: return 4;
        }
        // Java: else if ((multiType & 512) > 0)
        {
            // Java: return 512;
        }
        // Java: else if ((multiType & 1) > 0)
        {
            // Java: return 1;
        }
        // Java: else if ((multiType & 64) > 0)
        {
            // Java: return 64;
        }
        // Java: else
        {
            // Java: return (short)((multiType & 128) > 0 ? 8192 : 0);
        }
        todo!("NumFmtCategorizer::getMainType")
    }

    // Java: short distinguishDateTimeFmt(ArrayList<PlaceHolderPiece> pieces, String formatString)
    pub fn distinguishDateTimeFmt(&self, _pieces: &[i32], _format_string: &str) -> i16 {
        // Java: short dateTime = 0;
        // Java: int hIndex = -1;
        // Java: int mIndex = -1;
        // Java: int sIndex = -1;
        // Java: if (formatString != null && formatString.length() != 0)
        {
            // Java: for(int i = 0; i < pieces.size(); ++i)
            {
                // Java: short placeHolderID = ((PlaceHolderPiece)pieces.get(i)).mHolderToken.getHolderID();
                // Java: switch (placeHolderID)
                {
                    // Java: case 12:
                        // Java: dateTime = (short)(dateTime | 64);
                    // Java: case 13:
                    // Java: case 14:
                    // Java: case 15:
                    // Java: case 16:
                    // Java: case 17:
                    // Java: case 19:
                    // Java: default:
                        // Java: break;
                    // Java: case 18:
                        // Java: String ncl = ((PlaceHolderPiece)pieces.get(i)).mHolderToken.mToken;
                        // Java: if (ncl.equals("$-F800"))
                        {
                            // Java: return 8193;
                        }
                        // Java: if (ncl.equals("$-F400"))
                        {
                            // Java: return 8194;
                        }
                        // Java: break;
                    // Java: case 20:
                    // Java: case 21:
                    // Java: case 22:
                    // Java: case 23:
                    // Java: case 24:
                    // Java: case 25:
                    // Java: case 32:
                    // Java: case 33:
                        // Java: dateTime = (short)(dateTime | 8193);
                        // Java: break;
                    // Java: case 26:
                        // Java: dateTime = (short)(dateTime | 8194);
                        // Java: hIndex = i;
                        // Java: break;
                    // Java: case 27:
                        // Java: mIndex = i;
                        // Java: break;
                    // Java: case 28:
                        // Java: sIndex = i;
                    // Java: case 29:
                    // Java: case 30:
                    // Java: case 31:
                    // Java: case 34:
                        // Java: dateTime = (short)(dateTime | 8194);
                }
            }
            // Java: if (mIndex != -1)
            {
                // Java: if ((hIndex != -1 || sIndex != -1) && (hIndex == -1 || hIndex <= mIndex))
                {
                    // Java: dateTime = (short)(dateTime | 8194);
                }
                // Java: else
                {
                    // Java: dateTime = (short)(dateTime | 8193);
                }
            }
            // Java: return dateTime == 8195 ? 8192 : dateTime;
        }
        // Java: else
        {
            // Java: return dateTime;
        }
        todo!("NumFmtCategorizer::distinguishDateTimeFmt")
    }

    // Java: short distinguishCurrencyFmt(String formatStr)
    pub fn distinguishCurrencyFmt(&self, _format_str: &str) -> i16 {
        // Java: return (short)(!formatStr.equalsIgnoreCase("\"$\"#,##0_);[Red](\"$\"#,##0)") && !formatStr.equalsIgnoreCase("￥#,##0;[Red]￥-#,##0") && !formatStr.equalsIgnoreCase("\"$\"#,##0.00_);[Red](\"$\"#,##0.00)") && !formatStr.equalsIgnoreCase("￥#,##0.00;[Red]￥-#,##0.00") && !formatStr.equalsIgnoreCase("\"$\"#,##0_);(\"$\"#,##0)") && !formatStr.equalsIgnoreCase("\"$\"#,##0.00);(\"$\"#,##0.00)") ? 512 : 513);
        todo!("NumFmtCategorizer::distinguishCurrencyFmt")
    }
}
