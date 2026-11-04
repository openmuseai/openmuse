use super::super::typo::layout::Layout::{Alignment, Layout};
use super::MeasuredText::MeasuredText;
use crate::cn::wps::moffice::spreadsheet::control::grid::{
    layout::data::{SpanInfo::SpanInfo, TextData::TextData},
    render::graphics::{Canvas, FontMetricsInt, Paint},
};
// java public class WpsLayout extends Layout
pub struct WpsLayout {
    pub text: Vec<u16>,
    pub paints: Vec<(usize, usize, Paint)>,
    pub strikeDecorations: Vec<(usize, usize, i32, f32)>,
    pub mWidth: i32,
    pub alignment: Alignment,
    pub mAngle: i16,
    pub mSingleLine: bool,
    pub charWidths: Vec<f32>,
    pub starts: Vec<i32>,
    pub tops: Vec<f32>,
    pub descents: Vec<f32>,
    pub mLineHeights: Vec<f32>,
    pub mLineWidths: Vec<f32>,
    pub mAngleRectWidth: i32,
    pub mAngleRectHeight: i32,
    pub sin: f32,
    pub cos: f32,
    pub tan: f32,
    dx: i32,
    lastLineWidthX: f32,
}
impl WpsLayout {
    // rust: ET's original constructor passes no word paragraph, zero tabSize, maxLines=i32::MAX and vertical=0.
    pub fn new(data: &TextData, width: i32, alignment: Alignment, paint: Paint) -> Self {
        let angle = data.mPara.mAngle;
        let r = ((angle as f64).abs() * 0.017453292519943295) as f32;
        let mut l = Self {
            text: data.source().to_vec(),
            paints: vec![],
            strikeDecorations: vec![],
            mWidth: width,
            alignment: if angle > 0 && angle <= 90 {
                Alignment::ALIGN_NORMAL
            } else if angle >= -90 && angle < 0 {
                Alignment::ALIGN_OPPOSITE
            } else {
                alignment
            },
            mAngle: angle,
            mSingleLine: !data.mPara.mIsWrapText,
            charWidths: vec![0.0; data.source().len()],
            starts: vec![],
            tops: vec![],
            descents: vec![],
            mLineHeights: vec![],
            mLineWidths: vec![],
            mAngleRectWidth: 0,
            mAngleRectHeight: 0,
            sin: (r as f64).sin() as f32,
            cos: (r as f64).cos() as f32,
            tan: (r as f64).tan() as f32,
            dx: 0,
            lastLineWidthX: 0.0,
        };
        l.generate(data, paint);
        l
    }
    // java private int getLineVisibleEnd(int line, int start, int end) {
    // java CharSequence text = this.mText;
    // java if (line == this.getLineCount() - 1) {
    // java return end;
    // java } else {
    // java while(end > start) {
    // java char ch = text.charAt(end - 1);
    // java if (ch == '\n') {
    // java return end - 1;
    // java }
    // java
    // java if (!TextLine.isLineEndSpace(ch)) {
    // java break;
    // java }
    // java
    // java --end;
    // java }
    // java
    // java return end;
    // java }
    // java }
    pub fn getLineVisibleEnd(&self, line: i32) -> usize {
        let start = self.starts[line as usize] as usize;
        let mut end = self.starts[line as usize + 1] as usize;
        if line == self.descents.len() as i32 - 1 {
            return end;
        }
        while end > start {
            let c = self.text[end - 1];
            if c == 10 {
                return end - 1;
            }
            if !super::super::typo::layout::TextLine::TextLine::isLineEndSpace(c) {
                break;
            }
            end -= 1;
        }
        end
    }
    // java private boolean isSpace(char c) {
    // java return c == ' ' || c == 8203 || c == '\r' || c == 12288 || c == 160;
    // java }
    fn isSpace(c: u16) -> bool {
        [32, 8203, 13, 12288, 160].contains(&c)
    }
    // java private boolean isBreak(char c) {
    // java return isMandatoryBreaks(c) || c == '\r' || c == '\n' || c == 133;
    // java }
    fn isBreak(c: u16) -> bool {
        Self::isMandatoryBreaks(c) || c == 13 || c == 10 || c == 133
    }
    // java public static boolean isMandatoryBreaks(char c) {
    // java return Arrays.binarySearch(BK, c) >= 0;
    // java }
    fn isMandatoryBreaks(c: u16) -> bool {
        [11, 12, 0x2028, 0x2029].contains(&c)
    }
    // java private boolean isKorean(char c) {
    // java return c >= '가' && c <= '\ud7af' || c >= 4352 && c <= 4607 || c >= 12592 && c <= 12687;
    // java }
    fn isKorean(c: u16) -> bool {
        (0xac00..=0xd7af).contains(&c) || (4352..=4607).contains(&c) || (12592..=12687).contains(&c)
    }
    // java public static final boolean isPunctuation(char c) {
    // java return ",.!:：\"?!，。？”！、;；)）》>}]】︴︾".indexOf(c) > -1;
    // java }
    fn isPunctuation(c: u16) -> bool {
        ",.!:：\"?!，。？”！、;；)）》>}]】︴︾"
            .encode_utf16()
            .any(|v| v == c)
    }
    // java private boolean isEAChar(char ch) {
    // java if ((ch < 1424 || ch > 1871) && (ch < 1920 || ch > 1983) && (ch < 6016 || ch > 6319) && (ch < 2304 || ch > 4255) && (ch < 8204 || ch > 8207) && (ch < 8234 || ch > 8239) && (ch < 9840 || ch > 9841) && (ch < 'יִ' || ch > 'ﭏ')) {
    // java if ((ch < 0 || ch > 127) && (ch < 128 || ch > 166) && (ch < 169 || ch > 175) && (ch < 178 || ch > 179) && (ch < 181 || ch > 214) && (ch < 216 || ch > 246) && (ch < 248 || ch > 1423) && (ch < 4256 || ch > 4351) && (ch < 4608 || ch > 4991) && (ch < 5024 || ch > 6015) && (ch < 7424 || ch > 7551) && (ch < 7680 || ch > 8191) && (ch < 8192 || ch > 8203) && (ch < 8240 || ch > 8262) && (ch < 8266 || ch > 9311) && (ch < 10176 || ch > 11263) && ch != '\ud835' && (ch < 'ﬀ' || ch > 'ﬗ') && (ch < 'ﭐ' || ch > '\ufe6f')) {
    // java if (ch >= 8208 && ch <= 8233) {
    // java return false;
    // java } else if (ch <= '\uf0ff' && ch >= '\uf000') {
    // java return false;
    // java } else {
    // java return (ch < 'ﹰ' || ch > '\ufeff') && (ch < 2144 || ch > 2159) && (ch < 2048 || ch > 2111) && (ch < 2112 || ch > 2143);
    // java }
    // java } else {
    // java return false;
    // java }
    // java } else {
    // java return false;
    // java }
    // java }
    fn isEAChar(ch: u16) -> bool {
        let ch = ch as i32;
        if !(1424..=1871).contains(&ch)
            && !(1920..=1983).contains(&ch)
            && !(6016..=6319).contains(&ch)
            && !(2304..=4255).contains(&ch)
            && !(8204..=8207).contains(&ch)
            && !(8234..=8239).contains(&ch)
            && !(9840..=9841).contains(&ch)
            && !(0xfb1d..=0xfb4f).contains(&ch)
        {
            if !(0..=127).contains(&ch)
                && !(128..=166).contains(&ch)
                && !(169..=175).contains(&ch)
                && !(178..=179).contains(&ch)
                && !(181..=214).contains(&ch)
                && !(216..=246).contains(&ch)
                && !(248..=1423).contains(&ch)
                && !(4256..=4351).contains(&ch)
                && !(4608..=4991).contains(&ch)
                && !(5024..=6015).contains(&ch)
                && !(7424..=7551).contains(&ch)
                && !(7680..=8191).contains(&ch)
                && !(8192..=8203).contains(&ch)
                && !(8240..=8262).contains(&ch)
                && !(8266..=9311).contains(&ch)
                && !(10176..=11263).contains(&ch)
                && ch != 0xd835
                && !(0xfb00..=0xfb17).contains(&ch)
                && !(0xfb50..=0xfe6f).contains(&ch)
            {
                if (8208..=8233).contains(&ch) {
                    false
                } else if (0xf000..=0xf0ff).contains(&ch) {
                    false
                } else {
                    !(0xfe70..=0xfeff).contains(&ch)
                        && !(2144..=2159).contains(&ch)
                        && !(2048..=2111).contains(&ch)
                        && !(2112..=2143).contains(&ch)
                }
            } else {
                false
            }
        } else {
            false
        }
    }
    // java void generate(CharSequence source, int bufStart, int bufEnd, TextPaint paint, float outerWidth, int textDir, float spacingmult, float spacingadd, float ellipsizedWidth, TextUtils.TruncateAt ellipsize) throws UnsupportTextException {
    // java this.mLineCount = 0;
    // java this.marginLefts = new ArrayList(4);
    // java if (this.isAngle()) {
    // java this.mLineHeights = new ArrayList(4);
    // java this.mLineWidths = new ArrayList(4);
    // java float mRadiansAngle = (float)Math.toRadians((double)MathUtil.abs(this.mAngle));
    // java this.sin = (float)Math.sin((double)mRadiansAngle);
    // java this.cos = (float)Math.cos((double)mRadiansAngle);
    // java this.tan = (float)Math.tan((double)mRadiansAngle);
    // java }
    // java
    // java List<Integer> tabStopIndexList = new ArrayList();
    // java float v = 0.0F;
    // java boolean needMultiply = spacingmult != 1.0F || spacingadd != 0.0F;
    // java Paint.FontMetricsInt fm = this.mFontMetricsInt;
    // java int[] chooseHtv = null;
    // java int[] wordPositions = null;
    // java this.spacings = new float[this.getText().length()];
    // java MeasuredText measured = this.mMeasured;
    // java measured.mSpacing = this.spacings;
    // java measured.spacingmult = spacingmult;
    // java Spanned spanned = null;
    // java if (source instanceof Spanned) {
    // java spanned = (Spanned)source;
    // java WordSplitSpan[] wordSplitSpans = (WordSplitSpan[])spanned.getSpans(0, spanned.length(), WordSplitSpan.class);
    // java if (wordSplitSpans.length > 0) {
    // java wordPositions = wordSplitSpans[0].positions;
    // java }
    // java }
    // java
    // java int posIndex = 1;
    // java
    // java int paraEnd;
    // java for(int paraStart = bufStart; paraStart <= bufEnd; paraStart = paraEnd) {
    // java paraEnd = this.indexOfNewline(source, paraStart, bufEnd);
    // java if (paraEnd < 0) {
    // java paraEnd = bufEnd;
    // java } else {
    // java ++paraEnd;
    // java }
    // java
    // java int firstWidthLineLimit = this.mLineCount + 1;
    // java float firstWidth = outerWidth - (float)this.getLeadMargin(0);
    // java float restWidth = outerWidth - (float)this.getLeadMargin(1);
    // java measured.setPara(source, paraStart, paraEnd);
    // java char[] chs = measured.mChars;
    // java float[] widths = measured.mWidths;
    // java byte[] chdirs = measured.mLevels;
    // java int dir = measured.mDir;
    // java boolean easy = measured.mEasy;
    // java float layoutWidth = firstWidth;
    // java float width = firstWidth;
    // java float w = 0.0F;
    // java int here = paraStart;
    // java int ok = paraStart;
    // java float okWidth = w;
    // java float okHeight = 0.0F;
    // java int zhOk = paraStart;
    // java int lastZhOk = paraStart;
    // java float okZhWidth = w;
    // java float okZhHeight = 0.0F;
    // java int fit = paraStart;
    // java float fitWidth = w;
    // java float fitHeight = 0.0F;
    // java boolean hasTabOrEmoji = false;
    // java boolean hasTab = false;
    // java int trailPuncCnt = 0;
    // java boolean forceNewLine = false;
    // java
    // java int spanEnd;
    // java for(int spanStart = paraStart; spanStart < paraEnd; spanStart = spanEnd) {
    // java float fmHeightOri;
    // java if (spanned == null) {
    // java spanEnd = paraEnd;
    // java int spanLen = paraEnd - spanStart;
    // java fmHeightOri = measured.addStyleRun(paint, spanLen, fm, (ITypeface)null);
    // java } else {
    // java spanEnd = spanned.nextSpanTransition(spanStart, paraEnd, MetricAffectingSpan.class);
    // java int spanLen = spanEnd - spanStart;
    // java MetricAffectingSpan[] spans = (MetricAffectingSpan[])spanned.getSpans(spanStart, spanEnd, MetricAffectingSpan.class);
    // java spans = (MetricAffectingSpan[])this.removeEmptySpans(spans, spanned, MetricAffectingSpan.class);
    // java fmHeightOri = measured.addStyleRun(paint, spans, spanLen, fm, this.isAllVerticalText());
    // java }
    // java
    // java if (this.originCharWidths != null) {
    // java System.arraycopy(widths, spanStart - paraStart, this.originCharWidths, spanStart, spanEnd - spanStart);
    // java }
    // java
    // java for(int j = spanStart; j < spanEnd; ++j) {
    // java char c = chs[j - paraStart];
    // java if (this.isAllVerticalText()) {
    // java widths[j - paraStart] = fmHeightOri;
    // java }
    // java
    // java if (!isMandatoryBreaks(c) && c != '\r' && c != '\n' && c != 133) {
    // java if (c == '\t') {
    // java int charIndex = j - paraStart;
    // java if (!tabStopIndexList.contains(charIndex)) {
    // java tabStopIndexList.add(charIndex);
    // java }
    // java
    // java if (!hasTab) {
    // java hasTab = true;
    // java hasTabOrEmoji = true;
    // java }
    // java
    // java int tabSize = this.mDefTabSize;
    // java int tabIndex = tabStopIndexList.indexOf(charIndex);
    // java if (this.mtabStops != null && tabIndex < this.mtabStops.length) {
    // java tabSize = (int)this.mtabStops[tabIndex];
    // java }
    // java
    // java int isBreakLineChar;
    // java if (this.mIndent < 0) {
    // java if (this.mbulletWidth > 0.0F) {
    // java isBreakLineChar = tabSize - this.mFirstLead % tabSize;
    // java } else {
    // java isBreakLineChar = Math.abs(this.mIndent) % tabSize;
    // java }
    // java } else {
    // java isBreakLineChar = tabSize - this.leadMargin % tabSize;
    // java }
    // java
    // java if (w < (float)isBreakLineChar) {
    // java if (this.mLineCount == 0) {
    // java tabSize = (int)((float)isBreakLineChar - w);
    // java } else {
    // java tabSize = (int)((float)tabSize - (w + (float)this.mMarginL) % (float)tabSize);
    // java }
    // java } else if (this.mLineCount == 0) {
    // java tabSize -= (int)((w + (float)this.mFirstLead) % (float)tabSize);
    // java } else {
    // java tabSize -= (int)((w + (float)this.mMarginL) % (float)tabSize);
    // java }
    // java
    // java if (spanned instanceof SpannableStringBuilder) {
    // java ((SpannableStringBuilder)spanned).setSpan(new WpsTabStop(tabSize), j, j + 1, 33);
    // java }
    // java
    // java widths[j - paraStart] = (float)tabSize;
    // java w += widths[j - paraStart];
    // java } else if (Emoji.emojiSurrogate(c) && j + 1 < spanEnd) {
    // java int emoji = Character.codePointAt(chs, j - paraStart);
    // java if (this.mEmoJi.contains(emoji)) {
    // java float emojiW = this.mEmoJi.measureWidth(emoji);
    // java if (emojiW != -1.0F) {
    // java w += emojiW;
    // java hasTabOrEmoji = true;
    // java ++j;
    // java } else {
    // java w += widths[j - paraStart];
    // java }
    // java } else {
    // java w += widths[j - paraStart];
    // java }
    // java } else {
    // java w += widths[j - paraStart];
    // java }
    // java } else {
    // java widths[j - paraStart] = 0.0F;
    // java }
    // java
    // java boolean isSpace = this.isSpace(c);
    // java boolean isPunc = isPunctuation(c);
    // java char nc = j + 1 - paraStart < paraEnd - paraStart ? chs[j + 1 - paraStart] : 0;
    // java boolean isBreakLineChar = isMandatoryBreaks(c) && j == paraEnd - 1;
    // java if (this.isAngle()) {
    // java width = (layoutWidth - Math.max(fmHeightOri, fitHeight) * this.cos) / this.sin;
    // java }
    // java
    // java if (this.mSingleLine || w <= width || isSpace || isBreakLineChar || !forceNewLine && this.mHangingPunct && isPunc && trailPuncCnt < 1) {
    // java fitWidth = w;
    // java fit = j + 1;
    // java if (isSpace && w > width) {
    // java forceNewLine = true;
    // java }
    // java
    // java if (w >= width && isPunc && this.mHangingPunct) {
    // java ++trailPuncCnt;
    // java if (hasTab) {
    // java WpsTabStop[] wpsTabStops = (WpsTabStop[])((Spanned)this.getText()).getSpans(0, paraEnd, WpsTabStop.class);
    // java int index = tabStopIndexList.size() - 1;
    // java if (wpsTabStops != null && wpsTabStops.length > 0 && wpsTabStops.length > index) {
    // java int newTabWidth = (int)((float)wpsTabStops[index].getTabWidth() - (w - width));
    // java newTabWidth = Math.max(newTabWidth, 0);
    // java wpsTabStops[index].setTabWidth(newTabWidth);
    // java widths[(Integer)tabStopIndexList.get(tabStopIndexList.size() - 1)] = (float)newTabWidth;
    // java w = width;
    // java }
    // java }
    // java }
    // java
    // java if (fmHeightOri > fitHeight) {
    // java fitHeight = fmHeightOri;
    // java }
    // java
    // java boolean isEa = this.isEAChar(c);
    // java isEa &= "￥".indexOf(c) == -1;
    // java char nnc = j + 2 - paraStart < paraEnd - paraStart ? chs[j + 2 - paraStart] : 0;
    // java boolean breakRule = nc == '\t';
    // java boolean enBreak = !isEa && (this.mLatinLnBrk || isSpace || ",!:?!，。？”！、;；".indexOf(c) > -1 && !Character.isDigit(nc) || ",!:?!，。？”！、;；".indexOf(nc) > -1 && !Character.isDigit(nnc) || (c == '-' || c == 8211) && !Character.isDigit(nc) || " -".indexOf(nc) > -1 || Character.isDigit(c) && this.isEAChar(nc) || this.isEAChar(nc));
    // java boolean cjkBreak = isEa && (!this.isKorean(c) || !this.isKorean(nc) || this.mLatinLnBrk) && (!this.mEaLnBrk || ",.!:：\"?!，。？”！、;；)）》>}]】︴︾".indexOf(nc) == -1) && ("([{‘“〔〈《「『〖【（［｛．".indexOf(c) == -1 || !this.mEaLnBrk);
    // java boolean wordBreak = null != wordPositions && posIndex < wordPositions.length && wordPositions[posIndex] == j + 1;
    // java if (wordBreak) {
    // java posIndex += 2;
    // java }
    // java
    // java if (enBreak || cjkBreak || wordBreak || breakRule) {
    // java if ((cjkBreak || enBreak) && !isSpace) {
    // java okZhWidth = w - widths[j - paraStart];
    // java zhOk = j;
    // java if (fitHeight > okZhHeight) {
    // java okZhHeight = fitHeight;
    // java }
    // java }
    // java
    // java okWidth = w;
    // java ok = j + 1;
    // java if (fitHeight > okHeight) {
    // java okHeight = fitHeight;
    // java }
    // java }
    // java } else {
    // java boolean moreChars = j + 1 < spanEnd;
    // java int endPos;
    // java float height;
    // java float currentTextWidth;
    // java if (this.mEaLnBrk && ok < paraEnd && isPunctuation(chs[ok - paraStart]) && lastZhOk != zhOk) {
    // java endPos = zhOk;
    // java height = okZhHeight;
    // java currentTextWidth = okZhWidth;
    // java } else if (ok != here && ok < fit && lastZhOk != zhOk) {
    // java if (ok < paraEnd && isPunctuation(chs[ok - paraStart])) {
    // java ++ok;
    // java }
    // java
    // java endPos = ok;
    // java height = okHeight;
    // java currentTextWidth = okWidth;
    // java } else if (fit != here) {
    // java endPos = fit;
    // java height = fitHeight;
    // java currentTextWidth = fitWidth;
    // java } else {
    // java here = Math.min(here, paraEnd - 1);
    // java endPos = here + 1;
    // java height = fmHeightOri;
    // java currentTextWidth = widths[here - paraStart];
    // java }
    // java
    // java trailPuncCnt = 0;
    // java forceNewLine = false;
    // java this.addSpacing(paraStart, here, endPos, widths, width, endPos == bufEnd, chs);
    // java v = this.out(source, here, endPos, height, v, spacingmult, spacingadd, chooseHtv, hasTabOrEmoji, needMultiply, chdirs, dir, easy, bufEnd, chs, widths, paraStart, ellipsize, ellipsizedWidth, currentTextWidth, paint, moreChars);
    // java here = endPos;
    // java j = endPos - 1;
    // java fit = endPos;
    // java ok = endPos;
    // java zhOk = endPos;
    // java lastZhOk = endPos;
    // java w = 0.0F;
    // java hasTab = false;
    // java fitHeight = 0.0F;
    // java okHeight = 0.0F;
    // java okZhHeight = 0.0F;
    // java --firstWidthLineLimit;
    // java if (firstWidthLineLimit <= 0) {
    // java width = restWidth;
    // java }
    // java
    // java if (endPos < spanStart) {
    // java measured.setPos(endPos);
    // java spanEnd = endPos;
    // java break;
    // java }
    // java
    // java if (this.mLineCount >= this.mMaximumVisibleLineCount) {
    // java break;
    // java }
    // java }
    // java }
    // java }
    // java
    // java if (paraEnd != here && this.mLineCount < this.mMaximumVisibleLineCount) {
    // java if (fitHeight == 0.0F) {
    // java paint.getFontMetricsInt(fm);
    // java fitHeight = (float)(fm.descent - fm.ascent);
    // java }
    // java
    // java this.addSpacing(paraStart, here, paraEnd, widths, width, paraEnd == bufEnd, chs);
    // java v = this.out(source, here, paraEnd, fitHeight, v, spacingmult, spacingadd, chooseHtv, hasTabOrEmoji, needMultiply, chdirs, dir, easy, bufEnd, chs, widths, paraStart, ellipsize, ellipsizedWidth, w, paint, paraEnd != bufEnd);
    // java }
    // java
    // java if (paraEnd == bufEnd) {
    // java break;
    // java }
    // java }
    // java
    // java if (bufEnd == bufStart && this.mLineCount < this.mMaximumVisibleLineCount) {
    // java measured.setPara(source, bufStart, bufEnd);
    // java paint.getFontMetricsInt(fm);
    // java this.out(source, bufEnd, bufEnd, (float)(fm.descent - fm.ascent), v, spacingmult, spacingadd, (int[])null, false, needMultiply, measured.mLevels, measured.mDir, measured.mEasy, bufEnd, (char[])null, (float[])null, bufStart, ellipsize, ellipsizedWidth, 0.0F, paint, false);
    // java }
    // java
    // java }
    fn generate(&mut self, data: &TextData, basePaint: Paint) {
        let source = self.text.clone();
        let outerWidth = self.mWidth as f32;
        let end = source.len();
        let mut paraStart = 0usize;
        let mut v = 0.0f32;
        let mut spans = vec![&data.mFirstSpan];
        spans.extend(data.mOtherSpans.iter());
        loop {
            let mut paraEnd = paraStart;
            while paraEnd < end && !Self::isBreak(source[paraEnd]) {
                paraEnd += 1;
            }
            if paraEnd < end {
                if source[paraEnd] == 13 && paraEnd + 1 < end && source[paraEnd + 1] == 10 {
                    paraEnd += 1;
                }
                paraEnd += 1;
            }
            let mut measured = MeasuredText::new();
            measured.mSpacing = vec![0.0; source.len()];
            measured.setPara(&source, paraStart, paraEnd);
            let mut width = outerWidth;
            let mut w = 0.0f32;
            let (mut here, mut ok, mut fit, mut zhOk, mut lastZhOk) =
                (paraStart, paraStart, paraStart, paraStart, paraStart);
            let (
                mut okWidth,
                mut okHeight,
                mut okZhWidth,
                mut okZhHeight,
                mut fitWidth,
                mut fitHeight,
            ) = (0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);
            let mut forceNewLine = false;
            let mut spanStart = paraStart;
            while spanStart < paraEnd {
                let span = spans
                    .iter()
                    .find(|s| s.contains(spanStart as i32))
                    .copied()
                    .expect("Metric span coverage");
                let start = spanStart;
                let stop = (span.mEndPos as usize).min(paraEnd);
                let mut spanEnd = stop;
                let mut p = basePaint.clone();
                if data.mIsRichText {
                    p.textSize = span.mFontSize as i32 as f32;
                    p.color = span.mFontColor;
                    p.fakeBold = span.mIsBold;
                    p.skewX = if span.mIsItalic { -0.25 } else { 0.0 };
                    p.strike = false;
                    let shift = (p.getFontMetrics().ascent / 2.0) as i32;
                    p.baselineShift = if span.mIsSuperScript {
                        shift
                    } else if span.mIsSubScript {
                        -shift
                    } else {
                        0
                    };
                    if span.mIsStrikeout {
                        let thick = (span.mFontSize * 0.05) as i32;
                        let pos = span.getAscent() / 3.0 + thick as f32;
                        self.strikeDecorations.push((start, stop, thick, pos));
                    }
                    p.fontName = span.mFontName.clone();
                    p.useFontHost = true;
                    p.bold = span.mIsBold;
                    p.italic = span.mIsItalic;
                    p.setTypeface(&crate::cn::wps::moffice::spreadsheet::control::grid::layout::typo::PaintFontHelper::PaintFontHelper::getTypeface(&span.mFontName,span.mIsBold,span.mIsItalic));
                }
                measured.setPos(start);
                let mut fm = FontMetricsInt::default();
                let height =
                    measured.addStyleRun__span(&p, span, data.mIsRichText, stop - start, &mut fm);
                self.paints.push((start, stop, p));
                let fmHeight = height;
                let mut j = spanStart as i32;
                while j < spanEnd as i32 {
                    let ju = j as usize;
                    let c = source[ju];
                    if !Self::isBreak(c) {
                        if c == 9 {
                            let tab = 72;
                            let size = if w < 72.0 {
                                if self.starts.is_empty() {
                                    (72.0 - w) as i32
                                } else {
                                    (72.0 - w % 72.0) as i32
                                }
                            } else {
                                tab - (w % tab as f32) as i32
                            };
                            measured.mWidths[ju - paraStart] = size as f32;
                        }
                        w += measured.mWidths[ju - paraStart];
                    } else {
                        measured.mWidths[ju - paraStart] = 0.0;
                    }
                    let space = Self::isSpace(c);
                    let punc = Self::isPunctuation(c);
                    let nc = source
                        .get(ju + 1)
                        .filter(|_| ju + 1 < paraEnd)
                        .copied()
                        .unwrap_or(0);
                    let mandatory = Self::isMandatoryBreaks(c) && ju == paraEnd - 1;
                    if self.mAngle != 0 {
                        width = (outerWidth - fmHeight.max(fitHeight) * self.cos) / self.sin;
                    }
                    if self.mSingleLine || w <= width || space || mandatory {
                        fitWidth = w;
                        fit = ju + 1;
                        if space && w > width {
                            forceNewLine = true;
                        }
                        if fmHeight > fitHeight {
                            fitHeight = fmHeight;
                        }
                        let ea = Self::isEAChar(c) && c != 0xffe5;
                        let nnc = source
                            .get(ju + 2)
                            .filter(|_| ju + 2 < paraEnd)
                            .copied()
                            .unwrap_or(0);
                        let digit = |c: u16| {
                            crate::cn::wps::moffice::spreadsheet::control::grid::render::native::host(serde_json::json!({"op":"characterIsDigit","char":c}))["isDigit"].as_bool().expect("native Character.isDigit")
                        };
                        let br = ",!:?!，。？”！、;；";
                        let contains = |s: &str, c: u16| s.encode_utf16().any(|v| v == c);
                        let en = !ea
                            && (space
                                || contains(br, c) && !digit(nc)
                                || contains(br, nc) && !digit(nnc)
                                || (c == 45 || c == 8211) && !digit(nc)
                                || contains(" -", nc)
                                || digit(c) && Self::isEAChar(nc)
                                || Self::isEAChar(nc));
                        let cjk = ea && (!Self::isKorean(c) || !Self::isKorean(nc));
                        if en || cjk || nc == 9 {
                            if (cjk || en) && !space {
                                okZhWidth = w - measured.mWidths[ju - paraStart];
                                zhOk = ju;
                                if fitHeight > okZhHeight {
                                    okZhHeight = fitHeight;
                                }
                            }
                            okWidth = w;
                            ok = ju + 1;
                            if fitHeight > okHeight {
                                okHeight = fitHeight;
                            }
                        }
                    } else {
                        let (endPos, height, textWidth) =
                            if ok != here && ok < fit && lastZhOk != zhOk {
                                if ok < paraEnd && Self::isPunctuation(source[ok]) {
                                    ok += 1;
                                }
                                (ok, okHeight, okWidth)
                            } else if fit != here {
                                (fit, fitHeight, fitWidth)
                            } else {
                                here = here.min(paraEnd - 1);
                                (here + 1, fmHeight, measured.mWidths[here - paraStart])
                            };
                        self.addSpacing(
                            paraStart,
                            here,
                            endPos,
                            &mut measured.mWidths,
                            width,
                            endPos == end,
                            &source,
                        );
                        v = self.out(
                            here,
                            endPos,
                            height,
                            v,
                            textWidth,
                            &measured.mWidths,
                            paraStart,
                        );
                        here = endPos;
                        j = endPos as i32 - 1;
                        fit = endPos;
                        ok = endPos;
                        zhOk = endPos;
                        lastZhOk = endPos;
                        w = 0.0;
                        forceNewLine = false;
                        fitHeight = 0.0;
                        okHeight = 0.0;
                        okZhHeight = 0.0;
                        width = outerWidth;
                        if endPos < spanStart {
                            measured.setPos(endPos);
                            spanEnd = endPos;
                            break;
                        }
                    }
                    j += 1;
                }
                spanStart = spanEnd;
            }
            if paraEnd != here {
                if fitHeight == 0.0 {
                    let fm = basePaint.getFontMetricsInt();
                    fitHeight = (fm.descent - fm.ascent) as f32;
                }
                self.addSpacing(
                    paraStart,
                    here,
                    paraEnd,
                    &mut measured.mWidths,
                    width,
                    paraEnd == end,
                    &source,
                );
                v = self.out(here, paraEnd, fitHeight, v, w, &measured.mWidths, paraStart);
            }
            if paraEnd == end {
                break;
            }
            paraStart = paraEnd;
        }
        if source.is_empty() {
            let fm = basePaint.getFontMetricsInt();
            self.out(0, 0, (fm.descent - fm.ascent) as f32, 0.0, 0.0, &[], 0);
        }
    }
    // java private void addSpacing(int paraStart, int start, int end, float[] widths, float layoutWidth, boolean isEndLine, char[] chs) {
    // java if (!this.mSingleLine) {
    // java int pStart = start - paraStart;
    // java int pEnd = end - paraStart;
    // java boolean isJustify = this.getAlignment() == Layout.Alignment.ALIGN_JUSTIFY;
    // java boolean isDistributed = this.getAlignment() == Layout.Alignment.ALIGN_DISTRIBUTED;
    // java if (isJustify || isDistributed) {
    // java if (this.spacBefCharWidths != null) {
    // java System.arraycopy(widths, start - paraStart, this.spacBefCharWidths, start, end - start);
    // java }
    // java
    // java if (!isJustify || !isEndLine) {
    // java float lineTextWidth = 0.0F;
    // java int needSpacingStart = pStart;
    // java int needSpacingEnd = pEnd - 1;
    // java
    // java for(int index = pStart; index < pEnd; ++index) {
    // java char c = chs[index];
    // java if (!this.isSpace(c) && c != '\t') {
    // java break;
    // java }
    // java
    // java needSpacingStart = index + 1;
    // java }
    // java
    // java boolean isLast = true;
    // java
    // java for(int index = pEnd - 1; index > needSpacingStart; --index) {
    // java char c = chs[index];
    // java if (!isLast || !this.isSpace(c) && !this.isBreak(c)) {
    // java if (c == '\t') {
    // java needSpacingStart = index + 1;
    // java break;
    // java }
    // java } else {
    // java needSpacingEnd = index - 1;
    // java }
    // java
    // java isLast = false;
    // java }
    // java
    // java for(int index = pStart; index < pEnd; ++index) {
    // java if (index >= needSpacingStart && index < needSpacingEnd) {
    // java lineTextWidth += widths[index];
    // java } else {
    // java layoutWidth -= widths[index];
    // java }
    // java }
    // java
    // java if (needSpacingEnd > needSpacingStart) {
    // java float addSpace = (layoutWidth - lineTextWidth) / (float)(needSpacingEnd - needSpacingStart);
    // java if (addSpace <= 0.0F) {
    // java return;
    // java }
    // java
    // java for(int index = needSpacingStart; index < needSpacingEnd; ++index) {
    // java widths[index] += addSpace;
    // java }
    // java }
    // java
    // java }
    // java }
    // java }
    // java }
    fn addSpacing(
        &self,
        paraStart: usize,
        start: usize,
        end: usize,
        widths: &mut [f32],
        mut width: f32,
        isEnd: bool,
        chars: &[u16],
    ) {
        if self.mSingleLine {
            return;
        }
        let justify = self.alignment == Alignment::ALIGN_JUSTIFY;
        let distributed = self.alignment == Alignment::ALIGN_DISTRIBUTED;
        if !justify && !distributed {
            return;
        }
        if justify && isEnd {
            return;
        }
        let a = start - paraStart;
        let b = end - paraStart;
        let mut first = a;
        let mut last = b as i32 - 1;
        for i in a..b {
            let c = chars[i + paraStart];
            if !Self::isSpace(c) && c != 9 {
                break;
            }
            first = i + 1;
        }
        let mut isLast = true;
        for i in (first + 1..b).rev() {
            let c = chars[i + paraStart];
            if !isLast || !Self::isSpace(c) && !Self::isBreak(c) {
                if c == 9 {
                    first = i + 1;
                    break;
                }
            } else {
                last = i as i32 - 1;
            }
            isLast = false;
        }
        let mut textWidth = 0.0;
        for i in a..b {
            if i >= first && (i as i32) < last {
                textWidth += widths[i];
            } else {
                width -= widths[i];
            }
        }
        if last > first as i32 {
            let add = (width - textWidth) / (last - first as i32) as f32;
            if add <= 0.0 {
                return;
            }
            for i in first..last as usize {
                widths[i] += add;
            }
        }
    }
    // java private float out(CharSequence text, int start, int end, float height, float v, float spacingmult, float spacingadd, int[] chooseHtv, boolean hasTabOrEmoji, boolean needMultiply, byte[] chdirs, int dir, boolean easy, int bufEnd, char[] chs, float[] widths, int widthStart, TextUtils.TruncateAt ellipsize, float ellipsisWidth, float textWidth, TextPaint paint, boolean moreChars) {
    // java int j = this.mLineCount;
    // java int off = j * this.mColumns;
    // java int want = off + this.mColumns + 1;
    // java int[] lines = this.mLines;
    // java if (want >= lines.length) {
    // java int nlen = ArrayUtils.idealIntArraySize(want + 1);
    // java int[] grow = new int[nlen];
    // java System.arraycopy(lines, 0, grow, 0, lines.length);
    // java this.mLines = grow;
    // java lines = grow;
    // java Layout.Directions[] grow2 = new Layout.Directions[nlen];
    // java System.arraycopy(this.mLineDirections, 0, grow2, 0, this.mLineDirections.length);
    // java this.mLineDirections = grow2;
    // java }
    // java
    // java boolean hasLineSpace = true;
    // java float lineheight;
    // java if (spacingmult == 0.0F) {
    // java hasLineSpace = spacingadd != 0.0F;
    // java lineheight = hasLineSpace ? spacingadd : height * 1.2F;
    // java } else if (this.isAllVerticalText()) {
    // java lineheight = height * spacingmult;
    // java } else {
    // java lineheight = height * spacingmult * 1.2F;
    // java }
    // java
    // java if (this.isAngle()) {
    // java this.mLineHeights.add(lineheight);
    // java this.mLineWidths.add(textWidth);
    // java if (this.mLineHeights.size() == 1) {
    // java this.lastLineWidthX = textWidth * this.cos;
    // java this.mAngleRectWidth = (int)Math.ceil((double)(lineheight * this.sin + this.lastLineWidthX));
    // java this.mWidth = (int)textWidth;
    // java } else {
    // java this.dx = (int)((float)this.dx + lineheight / this.sin);
    // java float cur = (float)this.dx + textWidth * this.cos;
    // java if (cur > this.lastLineWidthX) {
    // java this.mAngleRectWidth = (int)((float)this.mAngleRectWidth - this.lastLineWidthX + cur);
    // java this.dx = 0;
    // java this.lastLineWidthX = textWidth * this.cos;
    // java }
    // java }
    // java
    // java this.mAngleRectHeight = (int)Math.max((float)this.mAngleRectHeight, lineheight * this.cos + textWidth * this.sin);
    // java } else {
    // java this.mAngleRectWidth = (int)Math.max((float)this.mAngleRectWidth, textWidth);
    // java }
    // java
    // java float baseline = hasLineSpace ? lineheight * 0.8F : 0.0F;
    // java float descent = hasLineSpace ? (float)Math.round(lineheight - baseline) : (float)Math.round(lineheight - lineheight * 0.8F);
    // java this.mlineDatas.add(this.mLineCount * 2 + 0, v);
    // java this.mlineDatas.add(this.mLineCount * 2 + 1, descent);
    // java lines[off + 0] = start;
    // java v = hasLineSpace ? v + lineheight : lineheight;
    // java lines[off + this.mColumns + 0] = end;
    // java this.mlineDatas.add(this.mLineCount * 2 + 2 + 0, v);
    // java if (hasTabOrEmoji) {
    // java lines[off + 0] |= 536870912;
    // java }
    // java
    // java lines[off + 0] |= dir << 30;
    // java this.mLineDirections[j] = this.getDefaultDirections();
    // java if (widths != null) {
    // java System.arraycopy(widths, start - widthStart, this.charWidths, start, end - start);
    // java if (text instanceof SpannableStringBuilder) {
    // java this.updateText((SpannableStringBuilder)text, start, end, true);
    // java int leadMargin = this.getLeadMargin(this.mLineCount);
    // java switch (this.getAlignment()) {
    // java case ALIGN_NORMAL:
    // java case ALIGN_JUSTIFY:
    // java if (this.mIsRightToLeft) {
    // java leadMargin = -leadMargin;
    // java }
    // java break;
    // java case ALIGN_OPPOSITE:
    // java if (!this.mIsRightToLeft) {
    // java leadMargin = 0;
    // java } else {
    // java leadMargin = -leadMargin;
    // java }
    // java break;
    // java case ALIGN_DISTRIBUTED:
    // java case ALIGN_CENTER:
    // java if (!this.mIsRightToLeft) {
    // java leadMargin /= 2;
    // java } else {
    // java leadMargin = -leadMargin / 2;
    // java }
    // java }
    // java
    // java this.marginLefts.add(leadMargin);
    // java }
    // java }
    // java
    // java ++this.mLineCount;
    // java return v;
    // java }
    fn out(
        &mut self,
        start: usize,
        end: usize,
        height: f32,
        v: f32,
        textWidth: f32,
        widths: &[f32],
        widthStart: usize,
    ) -> f32 {
        let lineheight = height * 1.0 * 1.2;
        if self.mAngle != 0 {
            self.mLineHeights.push(lineheight);
            self.mLineWidths.push(textWidth);
            if self.mLineHeights.len() == 1 {
                self.lastLineWidthX = textWidth * self.cos;
                self.mAngleRectWidth = (lineheight * self.sin + self.lastLineWidthX).ceil() as i32;
                self.mWidth = textWidth as i32;
            } else {
                self.dx = (self.dx as f32 + lineheight / self.sin) as i32;
                let cur = self.dx as f32 + textWidth * self.cos;
                if cur > self.lastLineWidthX {
                    self.mAngleRectWidth =
                        (self.mAngleRectWidth as f32 - self.lastLineWidthX + cur) as i32;
                    self.dx = 0;
                    self.lastLineWidthX = textWidth * self.cos;
                }
            }
            self.mAngleRectHeight = (self.mAngleRectHeight as f32)
                .max(lineheight * self.cos + textWidth * self.sin)
                as i32;
        } else {
            self.mAngleRectWidth = (self.mAngleRectWidth as f32).max(textWidth) as i32;
        }
        let baseline = lineheight * 0.8;
        let descent = crate::java_runtime::javaMathRoundFloat(lineheight - baseline) as f32;
        if self.starts.is_empty() {
            self.starts.push(start as i32);
            self.tops.push(v);
        }
        self.descents.push(descent);
        self.starts.push(end as i32);
        self.tops.push(v + lineheight);
        if !widths.is_empty() {
            self.charWidths[start..end]
                .copy_from_slice(&widths[start - widthStart..end - widthStart]);
        }
        v + lineheight
    }
}
impl Layout for WpsLayout {
    fn getWidth(&self) -> i32 {
        self.mWidth
    }
    fn getAlignment(&self) -> Alignment {
        self.alignment
    }
    fn getLineCount(&self) -> i32 {
        self.descents.len() as i32
    }
    fn getLineTop(&self, l: i32) -> f32 {
        self.tops[l as usize]
    }
    fn getLineDescent(&self, l: i32) -> f32 {
        self.descents[l as usize]
    }
    fn getLineStart(&self, l: i32) -> i32 {
        self.starts[l as usize]
    }
    fn getLineWidth(&self, l: i32) -> f32 {
        let mut w = 0.0f32;
        for c in
            &self.charWidths[self.starts[l as usize] as usize..self.starts[l as usize + 1] as usize]
        {
            w += c;
        }
        w
    }
    // java public float getLineMax(int line) {
    // java float width = (float)this.getLeadMargin(line);
    // java int lineStart = this.getLineStart(line);
    // java int lineEnd = this.getLineEnd(line);
    // java boolean isTailWhiteSpace = true;
    // java for(int i = lineEnd - 1; i >= lineStart; --i) {
    // java if (!isTailWhiteSpace || !this.isBreak(this.getText().charAt(i)) && !this.isSpace(this.getText().charAt(i))) {
    // java isTailWhiteSpace = false;
    // java width += this.charWidths[i];
    // java }
    // java }
    // java return width;
    // java }
    fn getLineMax(&self, l: i32) -> f32 {
        let mut w = 0.0f32;
        let mut tail = true;
        for i in (self.starts[l as usize] as usize..self.starts[l as usize + 1] as usize).rev() {
            if !tail || !Self::isBreak(self.text[i]) && !Self::isSpace(self.text[i]) {
                tail = false;
                w += self.charWidths[i];
            }
        }
        w
    }

    fn getLineForVertical(&self, y: i32) -> i32 {
        let (mut hi, mut lo) = (self.getLineCount(), -1);
        while hi - lo > 1 {
            let guess = (hi + lo) >> 1;
            if self.tops[guess as usize] > y as f32 {
                hi = guess;
            } else {
                lo = guess;
            }
        }
        lo.max(0)
    }
    fn draw(&self, canvas: &mut Canvas) {
        let (first, last) = if self.mAngle != 0 {
            (0, self.getLineCount() - 1)
        } else {
            let clip = canvas.getClipBounds();
            if clip.isEmpty() {
                return;
            }
            let top = clip.top.max(0);
            let bottom = self.getHeight().min(clip.bottom as f32) as i32;
            if top >= bottom {
                return;
            }
            (
                self.getLineForVertical(top),
                self.getLineForVertical(bottom),
            )
        };
        canvas.save();
        for line in first..=last {
            if self.mAngle != 0 && self.mAngle != 90 && self.mAngle != -90 && line != 0 {
                let dx = self.mLineHeights[line as usize] / self.tan;
                canvas.translate(if self.mAngle > 0 { dx } else { -dx }, 0.0);
            }
            super::super::typo::layout::TextLine::TextLine::new(self, line).draw(
                canvas,
                self.getAlignmentOffset(line),
                self.getLineBaseline(line),
            );
        }
        canvas.restore();
    }
}
