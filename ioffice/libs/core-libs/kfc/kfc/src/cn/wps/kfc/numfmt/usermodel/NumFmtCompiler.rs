use crate::cn::wps::kfc::numfmt::{
    format::{
        r#type::IFormat::FormatRef, FormatFactory::FormatFactory, NumFmtSection::NumFmtSection,
    },
    global::GlobalEnv::GlobalEnv,
    i18n::KmoLang::KmoLang,
    placeholder::{
        ConstPieceDic::ConstPieceDic,
        PieceContext::PieceContext,
        PlaceHolderToken::{PlaceHolderToken, TokenRef},
    },
};
// java public class NumFmtCompiler
pub struct NumFmtCompiler {
    pub mCompileHolders: Vec<TokenRef>,
    pub mNumFmtSection: NumFmtSection,
    mFormatterFactory: FormatFactory,
    pub mPieceContext: PieceContext,
    mPieceDic: ConstPieceDic,
}
impl NumFmtCompiler {
    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumFmtCompiler.java:24
    // java public NumFmtCompiler() {
    // java super();
    // java }
    pub fn new() -> Self {
        Self {
            mCompileHolders: Vec::new(),
            mNumFmtSection: NumFmtSection::new(),
            mFormatterFactory: FormatFactory::new(),
            mPieceContext: PieceContext::new(),
            mPieceDic: ConstPieceDic::new(),
        }
    }
    pub fn compile_text(&mut self, text: &str, code: &str) -> Option<FormatRef> {
        // rust: reset(formatCode);String formatSection=mNumFmtSection.getFormatSection(text);IFormat formatter=compileSection(...);return isTextFormat(formatter.getType())?formatter:null;
        self.reset(code);
        let section = self.mNumFmtSection.getFormatSection_str(text).to_string();
        let formatter = self.compileSection(&section)?;
        let ty = formatter.borrow().getType();
        if matches!(ty, 64 | 1 | 128 | 0) {
            Some(formatter)
        } else {
            None
        }
    }
    pub fn compile(&mut self, value: f64, code: &str) -> FormatRef {
        self.reset(code);
        let section = self.mNumFmtSection.getFormatSection_double(value);
        // rust: invalidNumberSignString reference identity is represented by the original missing-section branch, not by comparing user format text.
        if self.mNumFmtSection.isMissingSection(value) {
            return self
                .mFormatterFactory
                .getFormatter(-1, &self.mCompileHolders, &self.mPieceContext)
                .unwrap();
        }
        // rust: if(value<0&&(getFmtCount()==1||hasCondition()))mAddMinus=true;
        if value < 0.0
            && (self.mNumFmtSection.getFmtCount() == 1 || self.mNumFmtSection.hasCondition())
        {
            self.mPieceContext.mAddMinus = true;
        }
        self.compileSection(&section).expect("null formatter")
    }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumFmtCompiler.java:50
    // java protected void reset(String formatCode) {
    // java this.mCompileHolders.clear();
    // java this.mPieceContext.clear();
    // java this.mNumFmtSection.reset(formatCode);
    // java }
    pub fn reset(&mut self, code: &str) {
        self.mCompileHolders.clear();
        self.mPieceContext.clear();
        self.mNumFmtSection.reset(code);
    }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumFmtCompiler.java:56
    // java protected IFormat compileSection(String formatPart) {
    // java int partLength = formatPart.length();
    // java boolean ignoreCase = false;
    // java
    // java for(int i = 0; i < partLength; ++i) {
    // java PlaceHolderPiece piece = this.mPieceDic.searchPiece(formatPart.charAt(i));
    // java int len = piece.identify(formatPart, i, partLength, ignoreCase, this.mPieceContext);
    // java i += len - 1;
    // java if (ignoreCase && len == 0) {
    // java ++i;
    // java ignoreCase = false;
    // java } else {
    // java ignoreCase = len == 0;
    // java }
    // java
    // java if (piece.mHolderToken != null) {
    // java this.mCompileHolders.add(piece.mHolderToken);
    // java }
    // java }
    // java
    // java short formatterType = getMainType(this.mPieceContext.mPieceType);
    // java if (formatterType == 8192 || formatterType == 64) {
    // java this.mPieceContext.mAddMinus = false;
    // java }
    // java
    // java this.processArabic();
    // java return this.mFormatterFactory.getFormatter(formatterType, this.mCompileHolders, this.mPieceContext);
    // java }
    pub fn compileSection(&mut self, part: &str) -> Option<FormatRef> {
        let code: Vec<_> = part.encode_utf16().collect();
        let mut ignoreCase = false;
        let mut i = 0i32;
        while (i as usize) < code.len() {
            // rust: PlaceHolderPiece piece=mPieceDic.searchPiece(formatPart.charAt(i));int len=piece.identify(...);i+=len-1;
            let piece = self.mPieceDic.searchPiece(code[i as usize]);
            let len = piece.identify(
                &code,
                i as usize,
                code.len(),
                ignoreCase,
                &mut self.mPieceContext,
            );
            i += len - 1;
            // rust: if(ignoreCase&&len==0){++i;ignoreCase=false;}else ignoreCase=len==0;
            if ignoreCase && len == 0 {
                i += 1;
                ignoreCase = false;
            } else {
                ignoreCase = len == 0;
            }
            // rust: if(piece.mHolderToken!=null)mCompileHolders.add(piece.mHolderToken);
            if let Some(token) = &piece.base().mHolderToken {
                self.mCompileHolders.push(token.clone());
            }
            i += 1;
        }
        let ty = Self::getMainType(self.mPieceContext.mPieceType);
        if ty == 8192 || ty == 64 {
            self.mPieceContext.mAddMinus = false;
        }
        self.processArabic();
        self.mFormatterFactory
            .getFormatter(ty, &self.mCompileHolders, &self.mPieceContext)
    }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumFmtCompiler.java:85
    // java private void processArabic() {
    // java if (!this.mPieceContext.mHasFill && KmoLang.getPrimaryLang(GlobalEnv.getDefaultLCID()) == KmoLang.getPrimaryLang((short)1025) && this.mCompileHolders.size() != 0) {
    // java int index = 0;
    // java
    // java int size;
    // java label33:
    // java for(size = this.mCompileHolders.size(); index < size; ++index) {
    // java short holderID = ((PlaceHolderToken)this.mCompileHolders.get(index)).getHolderID();
    // java switch (holderID) {
    // java case 16:
    // java case 18:
    // java case 19:
    // java case 17:
    // java default:
    // java break label33;
    // java }
    // java }
    // java
    // java if (index < size && ((PlaceHolderToken)this.mCompileHolders.get(index)).mPlaceHolder.mPlaceHolderID == 13) {
    // java String token = ((PlaceHolderToken)this.mCompileHolders.get(index)).mToken;
    // java if (token.length() > 0 && isArabic(token.charAt(0))) {
    // java this.mPieceContext.mForceArbicShape = true;
    // java }
    // java
    // java }
    // java }
    // java }
    fn processArabic(&mut self) {
        // rust: without fill, Arabic primary language and nonempty token list: skip leading color/NCL/DBNum then inspect quoted literal for Arabic codepoints.
        if !self.mPieceContext.mHasFill
            && KmoLang::getPrimaryLang(GlobalEnv::getDefaultLCID()) == KmoLang::getPrimaryLang(1025)
            && !self.mCompileHolders.is_empty()
        {
            let mut index = 0;
            while index < self.mCompileHolders.len() {
                let id = self.mCompileHolders[index]
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .getHolderID();
                if !matches!(id, 16 | 18 | 19) {
                    break;
                }
                index += 1;
            }
            if index < self.mCompileHolders.len() {
                let token = self.mCompileHolders[index]
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                if token.getHolderID() == 13
                    && token
                        .mToken
                        .encode_utf16()
                        .next()
                        .is_some_and(Self::isArabic)
                {
                    self.mPieceContext.mForceArbicShape = true;
                }
            }
        }
    }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumFmtCompiler.java:113
    // java private static boolean isArabic(char c) {
    // java return c >= 1536 && c <= 1791 || c >= 1872 && c <= 1919;
    // java }
    fn isArabic(c: u16) -> bool {
        (1536..=1791).contains(&c) || (1872..=1919).contains(&c)
    }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumFmtCompiler.java:159
    // java private static short getMainType(short multiType) {
    // java if ((multiType & 8192) > 0) {
    // java return 8192;
    // java } else if ((multiType & 8) > 0) {
    // java return 8;
    // java } else if ((multiType & 4) > 0) {
    // java return 4;
    // java } else if ((multiType & 16) > 0) {
    // java return 16;
    // java } else if ((multiType & 512) > 0) {
    // java return 512;
    // java } else if ((multiType & 1) > 0) {
    // java return 1;
    // java } else if ((multiType & 64) > 0) {
    // java return 64;
    // java } else {
    // java return (short)((multiType & 128) > 0 ? 128 : 0);
    // java }
    // java }
    pub fn getMainType(multi: i16) -> i16 {
        for ty in [8192, 8, 4, 16, 512, 1, 64, 128] {
            if multi & ty > 0 {
                return ty;
            }
        }
        0
    }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumFmtCompiler.java:117
    // java public String decompile(String sourceCode) {
    // java String[] sectionGroups = new String[4];
    // java PieceContext context = new PieceContext();
    // java StringBuffer result = new StringBuffer(sourceCode.length());
    // java int sectCount = NumFmtSection.splitGroup(sourceCode, sectionGroups);
    // java
    // java for(int index = 0; index < sectCount; ++index) {
    // java if (index > 0) {
    // java result.append(';');
    // java }
    // java
    // java this.decompileSection(sectionGroups[index], result, context);
    // java }
    // java
    // java return result.toString();
    // java }
    pub fn decompile(&mut self, source: &str) -> String {
        let mut groups = std::array::from_fn(|_| None);
        let count = NumFmtSection::splitGroup(source, &mut groups);
        let mut context = PieceContext::new();
        let mut result = String::new();
        for index in 0..count as usize {
            if index > 0 {
                result.push(';');
            }
            self.decompileSection(groups[index].as_deref().unwrap(), &mut result, &mut context);
        }
        result
    }

    // rust: source koffice/core-libs/kfc/kfc/src/cn/wps/kfc/numfmt/usermodel/NumFmtCompiler.java:134
    // java private void decompileSection(String sectCode, StringBuffer result, PieceContext context) {
    // java context.clear();
    // java boolean ignoreCase = false;
    // java int i = 0;
    // java
    // java for(int length = sectCode.length(); i < length; ++i) {
    // java PlaceHolderPiece piece = this.mPieceDic.searchPiece(sectCode.charAt(i));
    // java int len = piece.identify(sectCode, i, length, ignoreCase, context);
    // java i += len - 1;
    // java if (ignoreCase && len == 0) {
    // java ++i;
    // java ignoreCase = false;
    // java } else {
    // java ignoreCase = len == 0;
    // java }
    // java
    // java if (piece.mHolderToken != null) {
    // java PlaceHolder token = piece.mHolderToken.doMutexHolder();
    // java token.decompile(result, context);
    // java piece.mHolderToken.recycle();
    // java }
    // java }
    // java
    // java }
    fn decompileSection(&mut self, part: &str, result: &mut String, context: &mut PieceContext) {
        context.clear();
        let code: Vec<_> = part.encode_utf16().collect();
        let mut ignore = false;
        let mut i = 0i32;
        while (i as usize) < code.len() {
            let piece = self.mPieceDic.searchPiece(code[i as usize]);
            let len = piece.identify(&code, i as usize, code.len(), ignore, context);
            i += len - 1;
            if ignore && len == 0 {
                i += 1;
                ignore = false;
            } else {
                ignore = len == 0;
            }
            if let Some(token) = &piece.base().mHolderToken {
                {
                    let data = token
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    data.doMutexHolder().decompile_value(result, context);
                }
                PlaceHolderToken::recycle(token);
            }
            i += 1;
        }
    }
}
