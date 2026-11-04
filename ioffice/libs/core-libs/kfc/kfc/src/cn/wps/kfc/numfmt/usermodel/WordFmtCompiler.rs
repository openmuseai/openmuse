#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

// Java: import cn.wps.kfc.numfmt.format.FormatFactory
use crate::cn::wps::kfc::numfmt::format::FormatFactory;
// Java: import cn.wps.kfc.numfmt.format.WordFmtSection
use crate::cn::wps::kfc::numfmt::format::WordFmtSection;
// Java: import cn.wps.kfc.numfmt.format.type.IFormat
use crate::cn::wps::kfc::numfmt::format::r#type::IFormat;
// Java: import cn.wps.kfc.numfmt.global.NCLAccesser
use crate::cn::wps::kfc::numfmt::global::NCLAccesser;
// Java: import cn.wps.kfc.numfmt.placeholder.PieceContext
use crate::cn::wps::kfc::numfmt::placeholder::PieceContext;
// Java: import cn.wps.kfc.numfmt.placeholder.PlaceHolderPiece
use crate::cn::wps::kfc::numfmt::placeholder::PlaceHolderPiece;
// Java: import cn.wps.kfc.numfmt.placeholder.PlaceHolderToken
use crate::cn::wps::kfc::numfmt::placeholder::PlaceHolderToken;
// Java: import cn.wps.kfc.numfmt.placeholder.WConstPieceDic
// use crate::cn::wps::kfc::numfmt::placeholder::WConstPieceDic;
// Java: import java.util.ArrayList

// Java: public class WordFmtCompiler
pub struct WordFmtCompiler {
    // Java field: ArrayList<PlaceHolderToken> mCompileHolders
    mCompileHolders: Vec<i32>,
    // Java field: WordFmtSection mWordFmtSection
    mWordFmtSection: i32,
    // Java field: FormatFactory mFormatterFactory
    mFormatterFactory: i32,
    // Java field: PieceContext mPieceContext
    mPieceContext: i32,
}

// Java: public class WordFmtCompiler
impl WordFmtCompiler {
    // Java: WordFmtCompiler()
    pub fn new() -> Self {
        // Java: super();
        WordFmtCompiler {
            mCompileHolders: Vec::new(),
            mWordFmtSection: 0,
            mFormatterFactory: 0,
            mPieceContext: 0,
        }
    }

    // Java: IFormat compile(double value, String formatCode, short language, byte calendar)
    pub fn compile_with_lang(&mut self, _value: f64, _format_code: &str, _language: i16, _calendar: i8) -> i32 {
        // Java: this.clear();
        // Java: this.compileSection(formatCode, (short)2);
        // Java: IFormat formatter = null;
        // Java: if ((this.mPieceContext.mPieceType & 8192) > 0)
        {
            // Java: this.mPieceContext.mNumAreaCode = NCLAccesser.genNCLCode((byte)0, calendar, language);
            // Java: formatter = this.mFormatterFactory.getFormatter((short)8192, this.mCompileHolders, this.mPieceContext);
        }
        // Java: else
        {
            // Java: this.recycle();
        }
        // Java: return formatter;
        todo!("WordFmtCompiler::compile_with_lang")
    }

    // Java: IFormat compile(double value, String formatCode)
    pub fn compile(&mut self, _value: f64, _format_code: &str) -> i32 {
        // Java: this.clear();
        // Java: this.mWordFmtSection.reset(formatCode);
        // Java: String formatSection = this.mWordFmtSection.getFormatSection(value);
        // Java: if (this.mWordFmtSection.getFmtCount() == 1 && value < (double)0.0F)
        {
            // Java: this.mPieceContext.mAddMinus = true;
        }
        // Java: this.compileSection(formatSection, (short)1);
        // Java: short type = 0;
        // Java: IFormat formatter = null;
        // Java: if ((this.mPieceContext.mPieceType & 512) > 0)
        {
            // Java: type = 512;
            // Java: formatter = this.mFormatterFactory.getFormatter(type, this.mCompileHolders, this.mPieceContext);
        }
        // Java: else
        {
            // Java: this.recycle();
        }
        // Java: return formatter;
        todo!("WordFmtCompiler::compile")
    }

    // Java: void compileSection(String formatPart, short dicType)
    pub fn compileSection(&mut self, _format_part: &str, _dic_type: i16) {
        // Java: int partLength = formatPart.length();
        // Java: boolean ignoreCase = false;
        // Java: for(int i = 0; i < partLength; ++i)
        {
            // Java: PlaceHolderPiece piece = WConstPieceDic.searchPiece(formatPart.charAt(i), dicType);
            // Java: int len = piece.identify(formatPart, i, partLength, true, this.mPieceContext);
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
                // Java: this.mCompileHolders.add(piece.mHolderToken);
            }
        }
        todo!("WordFmtCompiler::compileSection")
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mCompileHolders.clear();
        // Java: this.mPieceContext.clear();
        todo!("WordFmtCompiler::clear")
    }

    // Java: void recycle()
    pub fn recycle(&mut self) {
        // Java: for(int i = 0; i < this.mCompileHolders.size(); ++i)
        {
            // Java: ((PlaceHolderToken)this.mCompileHolders.get(i)).recycle();
        }
        todo!("WordFmtCompiler::recycle")
    }
}
