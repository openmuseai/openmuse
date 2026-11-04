#![allow(unused, non_snake_case, non_upper_case_globals)]
// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.io.IOException
// Java: import java.util.Arrays
// Java: import org.apache.poi.hdf.model.hdftypes.HDFType

pub trait HDFType {}
// Java: import org.apache.poi.hwpf.model.Dop2007
use super::super::Dop2007::Dop2007;
// Java: import org.apache.poi.hwpf.model.DopMath
use super::super::DopMath::DopMath;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.LittleEndian
// Java: import org.apache.poi.util.StringUtil

#[derive(Clone)]
struct BitField { _mask: i32, _shift_count: i32 }
impl BitField {
    fn new(mask: i32) -> Self {
        let mut count = 0i32;
        let mut bp = mask;
        if mask != 0 { while (bp & 1) == 0 { count += 1; bp >>= 1; } }
        BitField { _mask: mask, _shift_count: count }
    }
    fn getValue(&self, holder: i32) -> i32 { ((holder & self._mask) as u32 >> self._shift_count) as i32 }
    fn getShortValue(&self, holder: i16) -> i16 { self.getValue(holder as i32) as i16 }
    fn isSet(&self, holder: i32) -> bool { (holder & self._mask) != 0 }
    fn setValue(&self, holder: i32, value: i32) -> i32 { (holder & !self._mask) | ((value << self._shift_count) & self._mask) }
    fn setShortValue(&self, holder: i16, value: i16) -> i16 { self.setValue(holder as i32, value as i32) as i16 }
    fn setBoolean(&self, holder: i32, value: bool) -> i32 { if value { holder | self._mask } else { holder & !self._mask } }
}

struct StringUtil;
impl StringUtil {
    fn getFromUnicodeLE(buf: &[u8], offset: usize, len: usize) -> String {
        let end = (offset + len * 2).min(buf.len());
        let slice = &buf[offset..end];
        let u16s: Vec<u16> = slice.chunks(2).map(|c| u16::from_le_bytes([c[0], if c.len()>1{c[1]}else{0}])).collect();
        String::from_utf16_lossy(&u16s)
    }
    fn putUnicodeLE(s: &str, buf: &mut [u8], offset: usize) {
        for (i, c) in s.encode_utf16().enumerate() {
            let b = c.to_le_bytes();
            let pos = offset + i * 2;
            if pos + 1 < buf.len() { buf[pos] = b[0]; buf[pos+1] = b[1]; }
        }
    }
}

fn le_get_short(buf: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes([buf[offset], buf[offset + 1]])
}
fn le_put_short(buf: &mut [u8], offset: usize, value: i16) {
    let b = value.to_le_bytes();
    buf[offset] = b[0]; buf[offset + 1] = b[1];
}
fn le_get_int(buf: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes([buf[offset], buf[offset+1], buf[offset+2], buf[offset+3]])
}
fn le_put_int(buf: &mut [u8], offset: usize, value: i32) {
    let b = value.to_le_bytes();
    buf[offset] = b[0]; buf[offset+1] = b[1]; buf[offset+2] = b[2]; buf[offset+3] = b[3];
}

// Java field: BitField fFacingPages
static fFacingPages: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fWidowControl
static fWidowControl: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fPMHMainDoc
static fPMHMainDoc: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField grfSupression
static grfSupression: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fpc
static fpc: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField unused1
static unused1: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField rncFtn
static rncFtn: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField nFtn
static nFtn: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fOnlyMacPics
static fOnlyMacPics: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fOnlyWinPics
static fOnlyWinPics: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fLabelDoc
static fLabelDoc: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fHyphCapitals
static fHyphCapitals: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fAutoHyphen
static fAutoHyphen: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fFormNoFields
static fFormNoFields: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fLinkStyles
static fLinkStyles: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fRevMarking
static fRevMarking: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fBackup
static fBackup: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fExactCWords
static fExactCWords: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fPagHidden
static fPagHidden: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fPagResults
static fPagResults: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fLockAtn
static fLockAtn: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fMirrorMargins
static fMirrorMargins: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField unused3
static unused3: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDfltTrueType
static fDfltTrueType: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fPagSupressTopSpacing
static fPagSupressTopSpacing: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fProtEnabled
static fProtEnabled: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDispFormFldSel
static fDispFormFldSel: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fRMView
static fRMView: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fRMPrint
static fRMPrint: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField unused4
static unused4: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fLockRev
static fLockRev: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fEmbedFonts
static fEmbedFonts: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfNoTabForInd
static oldfNoTabForInd: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfNoSpaceRaiseLower
static oldfNoSpaceRaiseLower: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfSuppressSpbfAfterPageBreak
static oldfSuppressSpbfAfterPageBreak: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfWrapTrailSpaces
static oldfWrapTrailSpaces: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfMapPrintTextColor
static oldfMapPrintTextColor: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfNoColumnBalance
static oldfNoColumnBalance: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfConvMailMergeEsc
static oldfConvMailMergeEsc: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfSupressTopSpacing
static oldfSupressTopSpacing: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfOrigWordTableRules
static oldfOrigWordTableRules: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfTransparentMetafiles
static oldfTransparentMetafiles: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfShowBreaksInFrames
static oldfShowBreaksInFrames: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfSwapBordersFacingPgs
static oldfSwapBordersFacingPgs: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfLeaveBackslashAlone
static oldfLeaveBackslashAlone: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfExpShRtn
static oldfExpShRtn: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfDntULTrlSpc
static oldfDntULTrlSpc: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField oldfDntBlnSbDbWid
static oldfDntBlnSbDbWid: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField unused5
static unused5: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField rncEdn
static rncEdn: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField nEdn
static nEdn: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField epc
static epc: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField nfcFtnRef1
static nfcFtnRef1: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField nfcEdnRef1
static nfcEdnRef1: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fPrintFormData
static fPrintFormData: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fSaveFormData
static fSaveFormData: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fShadeFormData
static fShadeFormData: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fWCFtnEdn
static fWCFtnEdn: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField wvkSaved
static wvkSaved: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField wScaleSaved
static wScaleSaved: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField zkSaved
static zkSaved: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fRotateFontW6
static fRotateFontW6: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField iGutterPos
static iGutterPos: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fNoTabForInd
static fNoTabForInd: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fNoSpaceRaiseLower
static fNoSpaceRaiseLower: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fSupressSpdfAfterPageBreak
static fSupressSpdfAfterPageBreak: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fWrapTrailSpaces
static fWrapTrailSpaces: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fMapPrintTextColor
static fMapPrintTextColor: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fNoColumnBalance
static fNoColumnBalance: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fConvMailMergeEsc
static fConvMailMergeEsc: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fSupressTopSpacing
static fSupressTopSpacing: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fOrigWordTableRules
static fOrigWordTableRules: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fTransparentMetafiles
static fTransparentMetafiles: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fShowBreaksInFrames
static fShowBreaksInFrames: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fSwapBordersFacingPgs
static fSwapBordersFacingPgs: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fLeaveBackslashAlone
static fLeaveBackslashAlone: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fExpShRtn
static fExpShRtn: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDntULTrlSpc
static fDntULTrlSpc: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDntBlnSbDbWid
static fDntBlnSbDbWid: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fSuppressTopSPacingMac5
static fSuppressTopSPacingMac5: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fTruncDxaExpand
static fTruncDxaExpand: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fPrintBodyBeforeHdr
static fPrintBodyBeforeHdr: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fNoLeading
static fNoLeading: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDontMakeSpaceForUL
static fDontMakeSpaceForUL: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fMWSmallCaps
static fMWSmallCaps: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField f2ptExtLeadingOnly
static f2ptExtLeadingOnly: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fTruncFontHeight
static fTruncFontHeight: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fSubOnSize
static fSubOnSize: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fLineWrapLikeWord6
static fLineWrapLikeWord6: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fWW6BorderRules
static fWW6BorderRules: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fExactOnTop
static fExactOnTop: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fExtraAfter
static fExtraAfter: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fWPSpace
static fWPSpace: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fWPJust
static fWPJust: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fPrintMet
static fPrintMet: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fKerningPunct
static fKerningPunct: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField iJustification
static iJustification: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField iLevelOfKinsoku
static iLevelOfKinsoku: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField f2on1
static f2on1: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField iCustomKsu
static iCustomKsu: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField dyGridDisplay
static dyGridDisplay: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField dxGridDisplay
static dxGridDisplay: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fFollowMargins
static fFollowMargins: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField lvl
static lvl: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fGramAllDone
static fGramAllDone: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fGramAllClean
static fGramAllClean: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fSubsetFonts
static fSubsetFonts: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fHideLastVersion
static fHideLastVersion: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fHtmlDoc
static fHtmlDoc: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fSnapBorder
static fSnapBorder: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fIncludeHeader
static fIncludeHeader: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fIncludeFooter
static fIncludeFooter: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fForcePageSizePag
static fForcePageSizePag: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fMinFontSizePag
static fMinFontSizePag: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fHaveVersions
static fHaveVersions: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fAutoVersions
static fAutoVersions: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fVirusPrompted
static fVirusPrompted: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fVirusLoadSafe
static fVirusLoadSafe: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField KeyVirusSession30
static KeyVirusSession30: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fMaybeFitText
static fMaybeFitText: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fSpLayoutLikeWW8
static fSpLayoutLikeWW8: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fFtnLayoutLikeWW8
static fFtnLayoutLikeWW8: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDontUseHTMLParagraphAutoSpacing
static fDontUseHTMLParagraphAutoSpacing: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fNoAdjustLineHeightInTable
static fNoAdjustLineHeightInTable: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fForgetLastTabAlign
static fForgetLastTabAlign: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fUseAutospaceForFullWidthAlpha
static fUseAutospaceForFullWidthAlpha: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fAlignTablesRowByRow
static fAlignTablesRowByRow: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fLayoutRawTableWidth
static fLayoutRawTableWidth: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fLayoutTableRowsApart
static fLayoutTableRowsApart: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fUseWord97LineBreakingRules
static fUseWord97LineBreakingRules: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDontBreakWrappedTables
static fDontBreakWrappedTables: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDontSnapToGridInCell
static fDontSnapToGridInCell: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDontAllowFieldEndSelect
static fDontAllowFieldEndSelect: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fApplyBreakingRules
static fApplyBreakingRules: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDontWrapTextWithPunct
static fDontWrapTextWithPunct: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDontUseAsianBreakRules
static fDontUseAsianBreakRules: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fUseWord2002TableStyleRules
static fUseWord2002TableStyleRules: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fGrowAutoFit
static fGrowAutoFit: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fUseNormalStyleForList
static fUseNormalStyleForList: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDontUseIndentAsNumberingTabStop
static fDontUseIndentAsNumberingTabStop: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fFELineBreak11
static fFELineBreak11: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fAllowSpaceOfSameStyleInTable
static fAllowSpaceOfSameStyleInTable: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fWW11IndentRules
static fWW11IndentRules: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDontAutofitConstrainedTables
static fDontAutofitConstrainedTables: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fAutofitLikeWW11
static fAutofitLikeWW11: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fUnderlineTabInNumList
static fUnderlineTabInNumList: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fHangulWidthLikeWW11
static fHangulWidthLikeWW11: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fSplitPgBreakAndParaMark
static fSplitPgBreakAndParaMark: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDontVertAlignCellWithSp
static fDontVertAlignCellWithSp: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDontBreakConstrainedForcedTables
static fDontBreakConstrainedForcedTables: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDontVertAlignInTxbx
static fDontVertAlignInTxbx: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fWord11KerningPairs
static fWord11KerningPairs: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fCachedColBalance
static fCachedColBalance: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField grfFmtFilter
static grfFmtFilter: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fFolioPrint
static fFolioPrint: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fReverseFolio
static fReverseFolio: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fTreatLockAtnAsReadOnly
static fTreatLockAtnAsReadOnly: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fStyleLock
static fStyleLock: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fStyleLockEnforced
static fStyleLockEnforced: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField iDocProtCur
static iDocProtCur: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fEnforceDocProt
static fEnforceDocProt: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();
// Java field: BitField fDisplayBackgroundShape
static fDisplayBackgroundShape: std::sync::OnceLock<BitField> = std::sync::OnceLock::new();

// Java: public class DOPAbstractType implements HDFType
pub struct DOPAbstractType {
    // Java field: byte field_1_formatFlags
    pub field_1_formatFlags: i8,
    // Java field: byte field_2_unused2
    pub field_2_unused2: i8,
    // Java field: short field_3_footnoteInfo
    pub field_3_footnoteInfo: i16,
    // Java field: byte field_4_fOutlineDirtySave
    pub field_4_fOutlineDirtySave: i8,
    // Java field: byte field_5_docinfo
    pub field_5_docinfo: i8,
    // Java field: byte field_6_docinfo1
    pub field_6_docinfo1: i8,
    // Java field: byte field_7_docinfo2
    pub field_7_docinfo2: i8,
    // Java field: short field_8_docinfo3
    pub field_8_docinfo3: i16,
    // Java field: int field_9_dxaTab
    pub field_9_dxaTab: i32,
    // Java field: int field_10_wSpare
    pub field_10_wSpare: i32,
    // Java field: int field_11_dxaHotz
    pub field_11_dxaHotz: i32,
    // Java field: int field_12_cConsexHypLim
    pub field_12_cConsexHypLim: i32,
    // Java field: int field_13_wSpare2
    pub field_13_wSpare2: i32,
    // Java field: int field_14_dttmCreated
    pub field_14_dttmCreated: i32,
    // Java field: int field_15_dttmRevised
    pub field_15_dttmRevised: i32,
    // Java field: int field_16_dttmLastPrint
    pub field_16_dttmLastPrint: i32,
    // Java field: int field_17_nRevision
    pub field_17_nRevision: i32,
    // Java field: int field_18_tmEdited
    pub field_18_tmEdited: i32,
    // Java field: int field_19_cWords
    pub field_19_cWords: i32,
    // Java field: int field_20_cCh
    pub field_20_cCh: i32,
    // Java field: int field_21_cPg
    pub field_21_cPg: i32,
    // Java field: int field_22_cParas
    pub field_22_cParas: i32,
    // Java field: short field_23_Edn
    pub field_23_Edn: i16,
    // Java field: short field_24_Edn1
    pub field_24_Edn1: i16,
    // Java field: int field_25_cLines
    pub field_25_cLines: i32,
    // Java field: int field_26_cWordsFtnEnd
    pub field_26_cWordsFtnEnd: i32,
    // Java field: int field_27_cChFtnEdn
    pub field_27_cChFtnEdn: i32,
    // Java field: short field_28_cPgFtnEdn
    pub field_28_cPgFtnEdn: i16,
    // Java field: int field_29_cParasFtnEdn
    pub field_29_cParasFtnEdn: i32,
    // Java field: int field_30_cLinesFtnEdn
    pub field_30_cLinesFtnEdn: i32,
    // Java field: int field_31_lKeyProtDoc
    pub field_31_lKeyProtDoc: i32,
    // Java field: short field_32_view
    pub field_32_view: i16,
    // Java field: int field_33_docinfo4
    pub field_33_docinfo4: i32,
    // Java field: short field_34_adt
    pub field_34_adt: i16,
    // Java field: byte[] field_35_doptypography
    pub field_35_doptypography: Vec<u8>,
    // Java field: byte[] field_36_dogrid
    pub field_36_dogrid: Vec<u8>,
    // Java field: short field_37_docinfo5
    pub field_37_docinfo5: i16,
    // Java field: short field_38_docinfo6
    pub field_38_docinfo6: i16,
    // Java field: byte[] field_39_asumyi
    pub field_39_asumyi: Vec<u8>,
    // Java field: int field_40_cChWS
    pub field_40_cChWS: i32,
    // Java field: int field_41_cChWSFtnEdn
    pub field_41_cChWSFtnEdn: i32,
    // Java field: int field_42_grfDocEvents
    pub field_42_grfDocEvents: i32,
    // Java field: int field_43_virusinfo
    pub field_43_virusinfo: i32,
    // Java field: byte[] field_44_Spare
    pub field_44_Spare: Vec<u8>,
    // Java field: int field_45_reserved1
    pub field_45_reserved1: i32,
    // Java field: int field_46_reserved2
    pub field_46_reserved2: i32,
    // Java field: int field_47_cDBC
    pub field_47_cDBC: i32,
    // Java field: int field_48_cDBCFtnEdn
    pub field_48_cDBCFtnEdn: i32,
    // Java field: int field_49_reserved
    pub field_49_reserved: i32,
    // Java field: short field_50_nfcFtnRef
    pub field_50_nfcFtnRef: i16,
    // Java field: short field_51_nfcEdnRef
    pub field_51_nfcEdnRef: i16,
    // Java field: short field_52_hpsZoonFontPag
    pub field_52_hpsZoonFontPag: i16,
    // Java field: short field_53_dywDispPag
    pub field_53_dywDispPag: i16,
    // Java field: byte[] field_54_unknown
    pub field_54_unknown: Vec<u8>,
    // Java field: byte[] field_55_Dop2002
    pub field_55_Dop2002: Vec<u8>,
    // Java field: byte[] field_56_Dop2003
    pub field_56_Dop2003: Vec<u8>,
    // Java field: Dop2007 field_57_dop07
    pub field_57_dop07: Dop2007,
    // Java field: byte[] field_58_unknown
    pub field_58_unknown: Vec<u8>,
}

// Java: public class DOPAbstractType implements HDFType
impl DOPAbstractType {
    // Java: DOPAbstractType()
    pub fn new() -> Self {
      // Java: super();
      DOPAbstractType {
        field_1_formatFlags: 0, field_2_unused2: 0, field_3_footnoteInfo: 0,
        field_4_fOutlineDirtySave: 0, field_5_docinfo: 0, field_6_docinfo1: 0,
        field_7_docinfo2: 0, field_8_docinfo3: 0, field_9_dxaTab: 0,
        field_10_wSpare: 0, field_11_dxaHotz: 0, field_12_cConsexHypLim: 0,
        field_13_wSpare2: 0, field_14_dttmCreated: 0, field_15_dttmRevised: 0,
        field_16_dttmLastPrint: 0, field_17_nRevision: 0, field_18_tmEdited: 0,
        field_19_cWords: 0, field_20_cCh: 0, field_21_cPg: 0, field_22_cParas: 0,
        field_23_Edn: 0, field_24_Edn1: 0, field_25_cLines: 0,
        field_26_cWordsFtnEnd: 0, field_27_cChFtnEdn: 0, field_28_cPgFtnEdn: 0,
        field_29_cParasFtnEdn: 0, field_30_cLinesFtnEdn: 0, field_31_lKeyProtDoc: 0,
        field_32_view: 0, field_33_docinfo4: 0, field_34_adt: 0,
        field_35_doptypography: vec![0u8; 310], field_36_dogrid: vec![0u8; 10],
        field_37_docinfo5: 0, field_38_docinfo6: 0, field_39_asumyi: vec![0u8; 12],
        field_40_cChWS: 0, field_41_cChWSFtnEdn: 0, field_42_grfDocEvents: 0,
        field_43_virusinfo: 0, field_44_Spare: vec![0u8; 30],
        field_45_reserved1: 0, field_46_reserved2: 0, field_47_cDBC: 0,
        field_48_cDBCFtnEdn: 0, field_49_reserved: 0,
        field_50_nfcFtnRef: 0, field_51_nfcEdnRef: 0,
        field_52_hpsZoonFontPag: 0, field_53_dywDispPag: 0,
        field_54_unknown: vec![0u8; 4], field_55_Dop2002: vec![0u8; 4],
        field_56_Dop2003: vec![0u8; 4], field_57_dop07: Dop2007::new(vec![0u8; 4]),
        field_58_unknown: Vec::new(),
      }
    }

    // Java: void fillFields(DocumentInputStream data, int offset, int size)
    pub fn fillFields(&mut self, data: &mut DocumentInputStream, offset: i32, size: i32) {
      // Java: data.seek((long)offset);
      data.seek(offset as i64);
      // Java: this.field_1_formatFlags = data.readByte();
      self.field_1_formatFlags = data.readByte();
      // Java: this.field_2_unused2 = data.readByte();
      self.field_2_unused2 = data.readByte();
      // Java: this.field_3_footnoteInfo = data.readShort();
      self.field_3_footnoteInfo = data.readShort();
      // Java: this.field_4_fOutlineDirtySave = data.readByte();
      self.field_4_fOutlineDirtySave = data.readByte();
      // Java: this.field_5_docinfo = data.readByte();
      self.field_5_docinfo = data.readByte();
      // Java: this.field_6_docinfo1 = data.readByte();
      self.field_6_docinfo1 = data.readByte();
      // Java: this.field_7_docinfo2 = data.readByte();
      self.field_7_docinfo2 = data.readByte();
      // Java: this.field_8_docinfo3 = data.readShort();
      self.field_8_docinfo3 = data.readShort();
      // Java: this.field_9_dxaTab = data.readShort();
      self.field_9_dxaTab = data.readShort() as i32;
      // Java: this.field_10_wSpare = data.readShort();
      self.field_10_wSpare = data.readShort() as i32;
      // Java: this.field_11_dxaHotz = data.readShort();
      self.field_11_dxaHotz = data.readShort() as i32;
      // Java: this.field_12_cConsexHypLim = data.readShort();
      self.field_12_cConsexHypLim = data.readShort() as i32;
      // Java: this.field_13_wSpare2 = data.readShort();
      self.field_13_wSpare2 = data.readShort() as i32;
      // Java: this.field_14_dttmCreated = data.readInt();
      self.field_14_dttmCreated = data.readInt();
      // Java: this.field_15_dttmRevised = data.readInt();
      self.field_15_dttmRevised = data.readInt();
      // Java: this.field_16_dttmLastPrint = data.readInt();
      self.field_16_dttmLastPrint = data.readInt();
      // Java: this.field_17_nRevision = data.readShort();
      self.field_17_nRevision = data.readShort() as i32;
      // Java: this.field_18_tmEdited = data.readInt();
      self.field_18_tmEdited = data.readInt();
      // Java: this.field_19_cWords = data.readInt();
      self.field_19_cWords = data.readInt();
      // Java: this.field_20_cCh = data.readInt();
      self.field_20_cCh = data.readInt();
      // Java: this.field_21_cPg = data.readShort();
      self.field_21_cPg = data.readShort() as i32;
      // Java: this.field_22_cParas = data.readInt();
      self.field_22_cParas = data.readInt();
      // Java: this.field_23_Edn = data.readShort();
      self.field_23_Edn = data.readShort();
      // Java: this.field_24_Edn1 = data.readShort();
      self.field_24_Edn1 = data.readShort();
      // Java: this.field_25_cLines = data.readInt();
      self.field_25_cLines = data.readInt();
      // Java: this.field_26_cWordsFtnEnd = data.readInt();
      self.field_26_cWordsFtnEnd = data.readInt();
      // Java: this.field_27_cChFtnEdn = data.readInt();
      self.field_27_cChFtnEdn = data.readInt();
      // Java: this.field_28_cPgFtnEdn = data.readShort();
      self.field_28_cPgFtnEdn = data.readShort();
      // Java: this.field_29_cParasFtnEdn = data.readInt();
      self.field_29_cParasFtnEdn = data.readInt();
      // Java: this.field_30_cLinesFtnEdn = data.readInt();
      self.field_30_cLinesFtnEdn = data.readInt();
      // Java: this.field_31_lKeyProtDoc = data.readInt();
      self.field_31_lKeyProtDoc = data.readInt();
      // Java: this.field_32_view = data.readShort();
      self.field_32_view = data.readShort();
      // Java: this.field_35_doptypography = new byte[310];
      self.field_35_doptypography = vec![0u8; 310];
      // Java: this.field_36_dogrid = new byte[10];
      self.field_36_dogrid = vec![0u8; 10];
      // Java: this.field_39_asumyi = new byte[12];
      self.field_39_asumyi = vec![0u8; 12];
      // Java: this.field_44_Spare = new byte[30];
      self.field_44_Spare = vec![0u8; 30];
      // Java: int DOP97_SIZE = 500;
      let _dop97_size = 500;
      // Java: if (size >= 500)
      if size >= 500
      {
         // Java: this.field_33_docinfo4 = data.readInt();
         self.field_33_docinfo4 = data.readInt();
         // Java: this.field_34_adt = data.readShort();
         self.field_34_adt = data.readShort();
         // Java: data.read(this.field_35_doptypography);
         data.read(&mut self.field_35_doptypography);
         // Java: data.read(this.field_36_dogrid);
         data.read(&mut self.field_36_dogrid);
         // Java: this.field_37_docinfo5 = data.readShort();
         self.field_37_docinfo5 = data.readShort();
         // Java: this.field_38_docinfo6 = data.readShort();
         self.field_38_docinfo6 = data.readShort();
         // Java: data.read(this.field_39_asumyi);
         data.read(&mut self.field_39_asumyi);
         // Java: this.field_40_cChWS = data.readInt();
         self.field_40_cChWS = data.readInt();
         // Java: this.field_41_cChWSFtnEdn = data.readInt();
         self.field_41_cChWSFtnEdn = data.readInt();
         // Java: this.field_42_grfDocEvents = data.readInt();
         self.field_42_grfDocEvents = data.readInt();
         // Java: this.field_43_virusinfo = data.readInt();
         self.field_43_virusinfo = data.readInt();
         // Java: data.read(this.field_44_Spare);
         data.read(&mut self.field_44_Spare);
         // Java: this.field_45_reserved1 = data.readInt();
         self.field_45_reserved1 = data.readInt();
         // Java: this.field_46_reserved2 = data.readInt();
         self.field_46_reserved2 = data.readInt();
         // Java: this.field_47_cDBC = data.readInt();
         self.field_47_cDBC = data.readInt();
         // Java: this.field_48_cDBCFtnEdn = data.readInt();
         self.field_48_cDBCFtnEdn = data.readInt();
         // Java: this.field_49_reserved = data.readInt();
         self.field_49_reserved = data.readInt();
         // Java: this.field_50_nfcFtnRef = data.readShort();
         self.field_50_nfcFtnRef = data.readShort();
         // Java: this.field_51_nfcEdnRef = data.readShort();
         self.field_51_nfcEdnRef = data.readShort();
         // Java: this.field_52_hpsZoonFontPag = data.readShort();
         self.field_52_hpsZoonFontPag = data.readShort();
         // Java: this.field_53_dywDispPag = data.readShort();
         self.field_53_dywDispPag = data.readShort();
      }
      // Java: int DOP2000_SIZE = 544;
      let _dop2000_size = 544;
      // Java: if (size > 500)
      if size > 500
      {
         // Java: this.field_54_unknown = new byte[44];
         self.field_54_unknown = vec![0u8; 44];
         // Java: data.seek((long)(500 + offset));
         data.seek((500 + offset) as i64);
         // Java: data.read(this.field_54_unknown, 0, 44);
         data.read_off(&mut self.field_54_unknown, 0, 44);
      }
      // Java: else
      else
      {
         // Java: this.field_54_unknown = new byte[4];
         self.field_54_unknown = vec![0u8; 4];
      }
      // Java: int DOP2002_SIZE = 594;
      let _dop2002_size = 594;
      // Java: if (size > 544)
      if size > 544
      {
         // Java: this.field_55_Dop2002 = new byte[50];
         self.field_55_Dop2002 = vec![0u8; 50];
         // Java: data.seek((long)(544 + offset));
         data.seek((544 + offset) as i64);
         // Java: data.read(this.field_55_Dop2002, 0, 50);
         data.read_off(&mut self.field_55_Dop2002, 0, 50);
      }
      // Java: else
      else
      {
         // Java: this.field_55_Dop2002 = new byte[4];
         self.field_55_Dop2002 = vec![0u8; 4];
      }
      // Java: int DOP2003_SIZE = 616;
      let _dop2003_size = 616;
      // Java: if (size > 594)
      if size > 594
      {
         // Java: this.field_56_Dop2003 = new byte[22];
         self.field_56_Dop2003 = vec![0u8; 22];
         // Java: data.seek((long)(594 + offset));
         data.seek((594 + offset) as i64);
         // Java: data.read(this.field_56_Dop2003, 0, 22);
         data.read_off(&mut self.field_56_Dop2003, 0, 22);
      }
      // Java: else
      else
      {
         // Java: this.field_56_Dop2003 = new byte[4];
         self.field_56_Dop2003 = vec![0u8; 4];
      }
      // Java: int DOP2007SIZE = 58;
      let _dop2007_size = 58;
      // Java: if (size >= 674)
      if size >= 674
      {
         // Java: byte[] field_57_Dop2007 = new byte[size - 616];
         let mut field_57_dop2007_buf = vec![0u8; (size - 616) as usize];
         // Java: data.seek((long)(616 + offset));
         data.seek((616 + offset) as i64);
         // Java: data.read(field_57_Dop2007, 0, size - 616);
         data.read_off(&mut field_57_dop2007_buf, 0, size - 616);
         // Java: this.field_57_dop07 = new Dop2007(field_57_Dop2007);
         self.field_57_dop07 = Dop2007::new(field_57_dop2007_buf);
         // Java: this.field_57_dop07.fillFields();
         self.field_57_dop07.fillFields();
      }
      // Java: else if (size > 616)
      else if size > 616
      {
         // Java: this.field_57_dop07 = new Dop2007(new byte[74], new DopMath());
         self.field_57_dop07 = Dop2007::new_with_dop_math(vec![0u8; 74], DopMath::new());
      }
      // Java: else
      else
      {
         // Java: this.field_57_dop07 = new Dop2007(new byte[4]);
         self.field_57_dop07 = Dop2007::new(vec![0u8; 4]);
      }
    }

    // Java: void serialize(byte[] data, int offset)
    pub fn serialize(&mut self, data: &mut [u8], offset: i32) {
      let o = offset as usize;
      // Java: data[0 + offset] = this.field_1_formatFlags;
      data[0 + o] = self.field_1_formatFlags as u8;
      // Java: data[1 + offset] = this.field_2_unused2;
      data[1 + o] = self.field_2_unused2 as u8;
      // Java: LittleEndian.putShort(data, 2 + offset, this.field_3_footnoteInfo);
      le_put_short(data, 2 + o, self.field_3_footnoteInfo);
      // Java: data[4 + offset] = this.field_4_fOutlineDirtySave;
      data[4 + o] = self.field_4_fOutlineDirtySave as u8;
      // Java: data[5 + offset] = this.field_5_docinfo;
      data[5 + o] = self.field_5_docinfo as u8;
      // Java: data[6 + offset] = this.field_6_docinfo1;
      data[6 + o] = self.field_6_docinfo1 as u8;
      // Java: data[7 + offset] = this.field_7_docinfo2;
      data[7 + o] = self.field_7_docinfo2 as u8;
      // Java: LittleEndian.putShort(data, 8 + offset, this.field_8_docinfo3);
      le_put_short(data, 8 + o, self.field_8_docinfo3);
      // Java: LittleEndian.putShort(data, 10 + offset, (short)this.field_9_dxaTab);
      le_put_short(data, 10 + o, self.field_9_dxaTab as i16);
      // Java: LittleEndian.putShort(data, 12 + offset, (short)this.field_10_wSpare);
      le_put_short(data, 12 + o, self.field_10_wSpare as i16);
      // Java: LittleEndian.putShort(data, 14 + offset, (short)this.field_11_dxaHotz);
      le_put_short(data, 14 + o, self.field_11_dxaHotz as i16);
      // Java: LittleEndian.putShort(data, 16 + offset, (short)this.field_12_cConsexHypLim);
      le_put_short(data, 16 + o, self.field_12_cConsexHypLim as i16);
      // Java: LittleEndian.putShort(data, 18 + offset, (short)this.field_13_wSpare2);
      le_put_short(data, 18 + o, self.field_13_wSpare2 as i16);
      // Java: LittleEndian.putInt(data, 20 + offset, this.field_14_dttmCreated);
      le_put_int(data, 20 + o, self.field_14_dttmCreated);
      // Java: LittleEndian.putInt(data, 24 + offset, this.field_15_dttmRevised);
      le_put_int(data, 24 + o, self.field_15_dttmRevised);
      // Java: LittleEndian.putInt(data, 28 + offset, this.field_16_dttmLastPrint);
      le_put_int(data, 28 + o, self.field_16_dttmLastPrint);
      // Java: LittleEndian.putShort(data, 32 + offset, (short)this.field_17_nRevision);
      le_put_short(data, 32 + o, self.field_17_nRevision as i16);
      // Java: LittleEndian.putInt(data, 34 + offset, this.field_18_tmEdited);
      le_put_int(data, 34 + o, self.field_18_tmEdited);
      // Java: LittleEndian.putInt(data, 38 + offset, this.field_19_cWords);
      le_put_int(data, 38 + o, self.field_19_cWords);
      // Java: LittleEndian.putInt(data, 42 + offset, this.field_20_cCh);
      le_put_int(data, 42 + o, self.field_20_cCh);
      // Java: LittleEndian.putShort(data, 46 + offset, (short)this.field_21_cPg);
      le_put_short(data, 46 + o, self.field_21_cPg as i16);
      // Java: LittleEndian.putInt(data, 48 + offset, this.field_22_cParas);
      le_put_int(data, 48 + o, self.field_22_cParas);
      // Java: LittleEndian.putShort(data, 52 + offset, this.field_23_Edn);
      le_put_short(data, 52 + o, self.field_23_Edn);
      // Java: LittleEndian.putShort(data, 54 + offset, this.field_24_Edn1);
      le_put_short(data, 54 + o, self.field_24_Edn1);
      // Java: LittleEndian.putInt(data, 56 + offset, this.field_25_cLines);
      le_put_int(data, 56 + o, self.field_25_cLines);
      // Java: LittleEndian.putInt(data, 60 + offset, this.field_26_cWordsFtnEnd);
      le_put_int(data, 60 + o, self.field_26_cWordsFtnEnd);
      // Java: LittleEndian.putInt(data, 64 + offset, this.field_27_cChFtnEdn);
      le_put_int(data, 64 + o, self.field_27_cChFtnEdn);
      // Java: LittleEndian.putShort(data, 68 + offset, this.field_28_cPgFtnEdn);
      le_put_short(data, 68 + o, self.field_28_cPgFtnEdn);
      // Java: LittleEndian.putInt(data, 70 + offset, this.field_29_cParasFtnEdn);
      le_put_int(data, 70 + o, self.field_29_cParasFtnEdn);
      // Java: LittleEndian.putInt(data, 74 + offset, this.field_30_cLinesFtnEdn);
      le_put_int(data, 74 + o, self.field_30_cLinesFtnEdn);
      // Java: LittleEndian.putInt(data, 78 + offset, this.field_31_lKeyProtDoc);
      le_put_int(data, 78 + o, self.field_31_lKeyProtDoc);
      // Java: LittleEndian.putShort(data, 82 + offset, this.field_32_view);
      le_put_short(data, 82 + o, self.field_32_view);
      // Java: if (data.length > 84 + offset)
      if data.len() > 84 + o
      {
         // Java: LittleEndian.putInt(data, 84 + offset, this.field_33_docinfo4);
         le_put_int(data, 84 + o, self.field_33_docinfo4);
      }
      // Java: if (data.length > 88 + offset)
      if data.len() > 88 + o
      {
         // Java: LittleEndian.putShort(data, 88 + offset, this.field_34_adt);
         le_put_short(data, 88 + o, self.field_34_adt);
         // Java: System.arraycopy(this.field_35_doptypography, 0, data, 90 + offset, this.field_35_doptypography.length);
         data[90+o..90+o+self.field_35_doptypography.len()].copy_from_slice(&self.field_35_doptypography);
         // Java: System.arraycopy(this.field_36_dogrid, 0, data, 400 + offset, this.field_36_dogrid.length);
         data[400+o..400+o+self.field_36_dogrid.len()].copy_from_slice(&self.field_36_dogrid);
         // Java: LittleEndian.putShort(data, 410 + offset, this.field_37_docinfo5);
         le_put_short(data, 410 + o, self.field_37_docinfo5);
         // Java: LittleEndian.putShort(data, 412 + offset, this.field_38_docinfo6);
         le_put_short(data, 412 + o, self.field_38_docinfo6);
         // Java: System.arraycopy(this.field_39_asumyi, 0, data, 414 + offset, this.field_39_asumyi.length);
         data[414+o..414+o+self.field_39_asumyi.len()].copy_from_slice(&self.field_39_asumyi);
         // Java: LittleEndian.putInt(data, 426 + offset, this.field_40_cChWS);
         le_put_int(data, 426 + o, self.field_40_cChWS);
         // Java: LittleEndian.putInt(data, 430 + offset, this.field_41_cChWSFtnEdn);
         le_put_int(data, 430 + o, self.field_41_cChWSFtnEdn);
         // Java: LittleEndian.putInt(data, 434 + offset, this.field_42_grfDocEvents);
         le_put_int(data, 434 + o, self.field_42_grfDocEvents);
         // Java: LittleEndian.putInt(data, 438 + offset, self.field_43_virusinfo);
         le_put_int(data, 438 + o, self.field_43_virusinfo);
         // Java: System.arraycopy(this.field_44_Spare, 0, data, 442 + offset, this.field_44_Spare.length);
         data[442+o..442+o+self.field_44_Spare.len()].copy_from_slice(&self.field_44_Spare);
         // Java: LittleEndian.putInt(data, 472 + offset, this.field_45_reserved1);
         le_put_int(data, 472 + o, self.field_45_reserved1);
         // Java: LittleEndian.putInt(data, 476 + offset, this.field_46_reserved2);
         le_put_int(data, 476 + o, self.field_46_reserved2);
         // Java: LittleEndian.putInt(data, 480 + offset, this.field_47_cDBC);
         le_put_int(data, 480 + o, self.field_47_cDBC);
         // Java: LittleEndian.putInt(data, 484 + offset, this.field_48_cDBCFtnEdn);
         le_put_int(data, 484 + o, self.field_48_cDBCFtnEdn);
         // Java: LittleEndian.putInt(data, 488 + offset, this.field_49_reserved);
         le_put_int(data, 488 + o, self.field_49_reserved);
         // Java: LittleEndian.putShort(data, 492 + offset, this.field_50_nfcFtnRef);
         le_put_short(data, 492 + o, self.field_50_nfcFtnRef);
         // Java: LittleEndian.putShort(data, 494 + offset, this.field_51_nfcEdnRef);
         le_put_short(data, 494 + o, self.field_51_nfcEdnRef);
         // Java: LittleEndian.putShort(data, 496 + offset, this.field_52_hpsZoonFontPag);
         le_put_short(data, 496 + o, self.field_52_hpsZoonFontPag);
         // Java: LittleEndian.putShort(data, 498 + offset, this.field_53_dywDispPag);
         le_put_short(data, 498 + o, self.field_53_dywDispPag);
      }
      // Java: if (data.length > 500 + offset)
      if data.len() > 500 + o
      {
         // Java: LittleEndian.putInt(this.field_54_unknown, 8, this.field_33_docinfo4);
         if self.field_54_unknown.len() > 11 {
           le_put_int(&mut self.field_54_unknown.clone(), 8, self.field_33_docinfo4);
         }
         // Java: System.arraycopy(this.field_54_unknown, 0, data, 500 + offset, this.field_54_unknown.length);
         let len54 = self.field_54_unknown.len().min(data.len() - (500+o));
         data[500+o..500+o+len54].copy_from_slice(&self.field_54_unknown[..len54]);
      }
      // Java: if (data.length > 544 + offset)
      if data.len() > 544 + o
      {
         // Java: System.arraycopy(this.field_55_Dop2002, 0, data, 544 + offset, this.field_55_Dop2002.length);
         let len55 = self.field_55_Dop2002.len().min(data.len() - (544+o));
         data[544+o..544+o+len55].copy_from_slice(&self.field_55_Dop2002[..len55]);
      }
      // Java: if (data.length > 594 + offset)
      if data.len() > 594 + o
      {
         // Java: System.arraycopy(this.field_56_Dop2003, 0, data, 594 + offset, this.field_56_Dop2003.length);
         let len56 = self.field_56_Dop2003.len().min(data.len() - (594+o));
         data[594+o..594+o+len56].copy_from_slice(&self.field_56_Dop2003[..len56]);
      }
      // Java: if (data.length > 616 + offset)
      if data.len() > 616 + o
      {
         // Java: this.field_57_dop07.serialize(data, 616 + offset);
         self.field_57_dop07.serialize(data, (616 + offset));
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuffer buffer = new StringBuffer();
      let mut buffer = String::new();
      // Java: buffer.append("[DOP]\n");
      buffer.push_str("[DOP]\n");
      // Java: buffer.append("[/DOP]\n");
      buffer.push_str("[/DOP]\n");
      // Java: return buffer.toString();
      buffer
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this.field_57_dop07.getSize() + this.field_56_Dop2003.length + this.field_55_Dop2002.length + this.field_54_unknown.length + 1 + 1 + 2 + 1 + 1 + 1 + 1 + 2 + 2 + 2 + 2 + 2 + 2 + 4 + 4 + 4 + 2 + 4 + 4 + 4 + 2 + 4 + 2 + 2 + 4 + 4 + 4 + 2 + 4 + 4 + 4 + 2 + 4 + 2 + 310 + 10 + 2 + 2 + 12 + 4 + 4 + 4 + 4 + 30 + 4 + 4 + 4 + 4 + 4 + 2 + 2 + 2 + 2;
      self.field_57_dop07.getSize() + self.field_56_Dop2003.len() as i32 + self.field_55_Dop2002.len() as i32 + self.field_54_unknown.len() as i32 + 1+1+2+1+1+1+1+2+2+2+2+2+2+4+4+4+2+4+4+4+2+4+2+2+4+4+4+2+4+4+4+2+4+2+310+10+2+2+12+4+4+4+4+30+4+4+4+4+4+2+2+2+2
    }

    // Java: byte getFormatFlags()
    pub fn getFormatFlags(&self) -> i8 { // Java: return this.field_1_formatFlags;
      self.field_1_formatFlags }
    // Java: void setFormatFlags(byte field_1_formatFlags)
    pub fn setFormatFlags(&mut self, v: i8) { // Java: this.field_1_formatFlags = field_1_formatFlags;
      self.field_1_formatFlags = v; }
    // Java: byte getUnused2()
    pub fn getUnused2(&self) -> i8 { // Java: return this.field_2_unused2;
      self.field_2_unused2 }
    // Java: void setUnused2(byte field_2_unused2)
    pub fn setUnused2(&mut self, v: i8) { // Java: this.field_2_unused2 = field_2_unused2;
      self.field_2_unused2 = v; }
    // Java: short getFootnoteInfo()
    pub fn getFootnoteInfo(&self) -> i16 { // Java: return this.field_3_footnoteInfo;
      self.field_3_footnoteInfo }
    // Java: void setFootnoteInfo(short field_3_footnoteInfo)
    pub fn setFootnoteInfo(&mut self, v: i16) { // Java: this.field_3_footnoteInfo = field_3_footnoteInfo;
      self.field_3_footnoteInfo = v; }
    // Java: byte getFOutlineDirtySave()
    pub fn getFOutlineDirtySave(&self) -> i8 { // Java: return this.field_4_fOutlineDirtySave;
      self.field_4_fOutlineDirtySave }
    // Java: void setFOutlineDirtySave(byte field_4_fOutlineDirtySave)
    pub fn setFOutlineDirtySave(&mut self, v: i8) { // Java: this.field_4_fOutlineDirtySave = field_4_fOutlineDirtySave;
      self.field_4_fOutlineDirtySave = v; }
    // Java: byte getDocinfo()
    pub fn getDocinfo(&self) -> i8 { // Java: return this.field_5_docinfo;
      self.field_5_docinfo }
    // Java: void setDocinfo(byte field_5_docinfo)
    pub fn setDocinfo(&mut self, v: i8) { // Java: this.field_5_docinfo = field_5_docinfo;
      self.field_5_docinfo = v; }
    // Java: byte getDocinfo1()
    pub fn getDocinfo1(&self) -> i8 { // Java: return this.field_6_docinfo1;
      self.field_6_docinfo1 }
    // Java: void setDocinfo1(byte field_6_docinfo1)
    pub fn setDocinfo1(&mut self, v: i8) { // Java: this.field_6_docinfo1 = field_6_docinfo1;
      self.field_6_docinfo1 = v; }
    // Java: byte getDocinfo2()
    pub fn getDocinfo2(&self) -> i8 { // Java: return this.field_7_docinfo2;
      self.field_7_docinfo2 }
    // Java: void setDocinfo2(byte field_7_docinfo2)
    pub fn setDocinfo2(&mut self, v: i8) { // Java: this.field_7_docinfo2 = field_7_docinfo2;
      self.field_7_docinfo2 = v; }
    // Java: short getDocinfo3()
    pub fn getDocinfo3(&self) -> i16 { // Java: return this.field_8_docinfo3;
      self.field_8_docinfo3 }
    // Java: void setDocinfo3(short field_8_docinfo3)
    pub fn setDocinfo3(&mut self, v: i16) { // Java: this.field_8_docinfo3 = field_8_docinfo3;
      self.field_8_docinfo3 = v; }
    // Java: int getDxaTab()
    pub fn getDxaTab(&self) -> i32 { // Java: return this.field_9_dxaTab;
      self.field_9_dxaTab }
    // Java: void setDxaTab(int field_9_dxaTab)
    pub fn setDxaTab(&mut self, v: i32) { // Java: this.field_9_dxaTab = field_9_dxaTab;
      self.field_9_dxaTab = v; }
    // Java: int getWSpare()
    pub fn getWSpare(&self) -> i32 { // Java: return this.field_10_wSpare;
      self.field_10_wSpare }
    // Java: void setWSpare(int field_10_wSpare)
    pub fn setWSpare(&mut self, v: i32) { // Java: this.field_10_wSpare = field_10_wSpare;
      self.field_10_wSpare = v; }
    // Java: int getDxaHotz()
    pub fn getDxaHotz(&self) -> i32 { // Java: return this.field_11_dxaHotz;
      self.field_11_dxaHotz }
    // Java: void setDxaHotz(int field_11_dxaHotz)
    pub fn setDxaHotz(&mut self, v: i32) { // Java: this.field_11_dxaHotz = field_11_dxaHotz;
      self.field_11_dxaHotz = v; }
    // Java: int getCConsexHypLim()
    pub fn getCConsexHypLim(&self) -> i32 { // Java: return this.field_12_cConsexHypLim;
      self.field_12_cConsexHypLim }
    // Java: void setCConsexHypLim(int field_12_cConsexHypLim)
    pub fn setCConsexHypLim(&mut self, v: i32) { // Java: this.field_12_cConsexHypLim = field_12_cConsexHypLim;
      self.field_12_cConsexHypLim = v; }
    // Java: int getWSpare2()
    pub fn getWSpare2(&self) -> i32 { // Java: return this.field_13_wSpare2;
      self.field_13_wSpare2 }
    // Java: void setWSpare2(int field_13_wSpare2)
    pub fn setWSpare2(&mut self, v: i32) { // Java: this.field_13_wSpare2 = field_13_wSpare2;
      self.field_13_wSpare2 = v; }
    // Java: int getDttmCreated()
    pub fn getDttmCreated(&self) -> i32 { // Java: return this.field_14_dttmCreated;
      self.field_14_dttmCreated }
    // Java: void setDttmCreated(int field_14_dttmCreated)
    pub fn setDttmCreated(&mut self, v: i32) { // Java: this.field_14_dttmCreated = field_14_dttmCreated;
      self.field_14_dttmCreated = v; }
    // Java: int getDttmRevised()
    pub fn getDttmRevised(&self) -> i32 { // Java: return this.field_15_dttmRevised;
      self.field_15_dttmRevised }
    // Java: void setDttmRevised(int field_15_dttmRevised)
    pub fn setDttmRevised(&mut self, v: i32) { // Java: this.field_15_dttmRevised = field_15_dttmRevised;
      self.field_15_dttmRevised = v; }
    // Java: int getDttmLastPrint()
    pub fn getDttmLastPrint(&self) -> i32 { // Java: return this.field_16_dttmLastPrint;
      self.field_16_dttmLastPrint }
    // Java: void setDttmLastPrint(int field_16_dttmLastPrint)
    pub fn setDttmLastPrint(&mut self, v: i32) { // Java: this.field_16_dttmLastPrint = field_16_dttmLastPrint;
      self.field_16_dttmLastPrint = v; }
    // Java: int getNRevision()
    pub fn getNRevision(&self) -> i32 { // Java: return this.field_17_nRevision;
      self.field_17_nRevision }
    // Java: void setNRevision(int field_17_nRevision)
    pub fn setNRevision(&mut self, v: i32) { // Java: this.field_17_nRevision = field_17_nRevision;
      self.field_17_nRevision = v; }
    // Java: int getTmEdited()
    pub fn getTmEdited(&self) -> i32 { // Java: return this.field_18_tmEdited;
      self.field_18_tmEdited }
    // Java: void setTmEdited(int field_18_tmEdited)
    pub fn setTmEdited(&mut self, v: i32) { // Java: this.field_18_tmEdited = field_18_tmEdited;
      self.field_18_tmEdited = v; }
    // Java: int getCWords()
    pub fn getCWords(&self) -> i32 { // Java: return this.field_19_cWords;
      self.field_19_cWords }
    // Java: void setCWords(int field_19_cWords)
    pub fn setCWords(&mut self, v: i32) { // Java: this.field_19_cWords = field_19_cWords;
      self.field_19_cWords = v; }
    // Java: int getCCh()
    pub fn getCCh(&self) -> i32 { // Java: return this.field_20_cCh;
      self.field_20_cCh }
    // Java: void setCCh(int field_20_cCh)
    pub fn setCCh(&mut self, v: i32) { // Java: this.field_20_cCh = field_20_cCh;
      self.field_20_cCh = v; }
    // Java: int getCPg()
    pub fn getCPg(&self) -> i32 { // Java: return this.field_21_cPg;
      self.field_21_cPg }
    // Java: void setCPg(int field_21_cPg)
    pub fn setCPg(&mut self, v: i32) { // Java: this.field_21_cPg = field_21_cPg;
      self.field_21_cPg = v; }
    // Java: int getCParas()
    pub fn getCParas(&self) -> i32 { // Java: return this.field_22_cParas;
      self.field_22_cParas }
    // Java: void setCParas(int field_22_cParas)
    pub fn setCParas(&mut self, v: i32) { // Java: this.field_22_cParas = field_22_cParas;
      self.field_22_cParas = v; }
    // Java: short getEdn()
    pub fn getEdn(&self) -> i16 { // Java: return this.field_23_Edn;
      self.field_23_Edn }
    // Java: void setEdn(short field_23_Edn)
    pub fn setEdn(&mut self, v: i16) { // Java: this.field_23_Edn = field_23_Edn;
      self.field_23_Edn = v; }
    // Java: short getEdn1()
    pub fn getEdn1(&self) -> i16 { // Java: return this.field_24_Edn1;
      self.field_24_Edn1 }
    // Java: void setEdn1(short field_24_Edn1)
    pub fn setEdn1(&mut self, v: i16) { // Java: this.field_24_Edn1 = field_24_Edn1;
      self.field_24_Edn1 = v; }
    // Java: int getCLines()
    pub fn getCLines(&self) -> i32 { // Java: return this.field_25_cLines;
      self.field_25_cLines }
    // Java: void setCLines(int field_25_cLines)
    pub fn setCLines(&mut self, v: i32) { // Java: this.field_25_cLines = field_25_cLines;
      self.field_25_cLines = v; }
    // Java: int getCWordsFtnEnd()
    pub fn getCWordsFtnEnd(&self) -> i32 { // Java: return this.field_26_cWordsFtnEnd;
      self.field_26_cWordsFtnEnd }
    // Java: void setCWordsFtnEnd(int field_26_cWordsFtnEnd)
    pub fn setCWordsFtnEnd(&mut self, v: i32) { // Java: this.field_26_cWordsFtnEnd = field_26_cWordsFtnEnd;
      self.field_26_cWordsFtnEnd = v; }
    // Java: int getCChFtnEdn()
    pub fn getCChFtnEdn(&self) -> i32 { // Java: return this.field_27_cChFtnEdn;
      self.field_27_cChFtnEdn }
    // Java: void setCChFtnEdn(int field_27_cChFtnEdn)
    pub fn setCChFtnEdn(&mut self, v: i32) { // Java: this.field_27_cChFtnEdn = field_27_cChFtnEdn;
      self.field_27_cChFtnEdn = v; }
    // Java: short getCPgFtnEdn()
    pub fn getCPgFtnEdn(&self) -> i16 { // Java: return this.field_28_cPgFtnEdn;
      self.field_28_cPgFtnEdn }
    // Java: void setCPgFtnEdn(short field_28_cPgFtnEdn)
    pub fn setCPgFtnEdn(&mut self, v: i16) { // Java: this.field_28_cPgFtnEdn = field_28_cPgFtnEdn;
      self.field_28_cPgFtnEdn = v; }
    // Java: int getCParasFtnEdn()
    pub fn getCParasFtnEdn(&self) -> i32 { // Java: return this.field_29_cParasFtnEdn;
      self.field_29_cParasFtnEdn }
    // Java: void setCParasFtnEdn(int field_29_cParasFtnEdn)
    pub fn setCParasFtnEdn(&mut self, v: i32) { // Java: this.field_29_cParasFtnEdn = field_29_cParasFtnEdn;
      self.field_29_cParasFtnEdn = v; }
    // Java: int getCLinesFtnEdn()
    pub fn getCLinesFtnEdn(&self) -> i32 { // Java: return this.field_30_cLinesFtnEdn;
      self.field_30_cLinesFtnEdn }
    // Java: void setCLinesFtnEdn(int field_30_cLinesFtnEdn)
    pub fn setCLinesFtnEdn(&mut self, v: i32) { // Java: this.field_30_cLinesFtnEdn = field_30_cLinesFtnEdn;
      self.field_30_cLinesFtnEdn = v; }
    // Java: int getLKeyProtDoc()
    pub fn getLKeyProtDoc(&self) -> i32 { // Java: return this.field_31_lKeyProtDoc;
      self.field_31_lKeyProtDoc }
    // Java: void setLKeyProtDoc(int field_31_lKeyProtDoc)
    pub fn setLKeyProtDoc(&mut self, v: i32) { // Java: this.field_31_lKeyProtDoc = field_31_lKeyProtDoc;
      self.field_31_lKeyProtDoc = v; }
    // Java: short getView()
    pub fn getView(&self) -> i16 { // Java: return this.field_32_view;
      self.field_32_view }
    // Java: void setView(short field_32_view)
    pub fn setView(&mut self, v: i16) { // Java: this.field_32_view = field_32_view;
      self.field_32_view = v; }
    // Java: int getDocinfo4()
    pub fn getDocinfo4(&self) -> i32 { // Java: return this.field_33_docinfo4;
      self.field_33_docinfo4 }
    // Java: void setDocinfo4(int field_33_docinfo4)
    pub fn setDocinfo4(&mut self, v: i32) { // Java: this.field_33_docinfo4 = field_33_docinfo4;
      self.field_33_docinfo4 = v; }
    // Java: short getAdt()
    pub fn getAdt(&self) -> i16 { // Java: return this.field_34_adt;
      self.field_34_adt }
    // Java: void setAdt(short field_34_adt)
    pub fn setAdt(&mut self, v: i16) { // Java: this.field_34_adt = field_34_adt;
      self.field_34_adt = v; }
    // Java: byte[] getDoptypography()
    pub fn getDoptypography(&self) -> &Vec<u8> { // Java: return this.field_35_doptypography;
      &self.field_35_doptypography }
    // Java: void setDoptypography(byte[] field_35_doptypography)
    pub fn setDoptypography(&mut self, v: Vec<u8>) { // Java: this.field_35_doptypography = field_35_doptypography;
      self.field_35_doptypography = v; }
    // Java: byte[] getDogrid()
    pub fn getDogrid(&self) -> &Vec<u8> { // Java: return this.field_36_dogrid;
      &self.field_36_dogrid }
    // Java: void setDogrid(byte[] field_36_dogrid)
    pub fn setDogrid(&mut self, v: Vec<u8>) { // Java: this.field_36_dogrid = field_36_dogrid;
      self.field_36_dogrid = v; }
    // Java: short getDocinfo5()
    pub fn getDocinfo5(&self) -> i16 { // Java: return this.field_37_docinfo5;
      self.field_37_docinfo5 }
    // Java: void setDocinfo5(short field_37_docinfo5)
    pub fn setDocinfo5(&mut self, v: i16) { // Java: this.field_37_docinfo5 = field_37_docinfo5;
      self.field_37_docinfo5 = v; }
    // Java: short getDocinfo6()
    pub fn getDocinfo6(&self) -> i16 { // Java: return this.field_38_docinfo6;
      self.field_38_docinfo6 }
    // Java: void setDocinfo6(short field_38_docinfo6)
    pub fn setDocinfo6(&mut self, v: i16) { // Java: this.field_38_docinfo6 = field_38_docinfo6;
      self.field_38_docinfo6 = v; }
    // Java: byte[] getAsumyi()
    pub fn getAsumyi(&self) -> &Vec<u8> { // Java: return this.field_39_asumyi;
      &self.field_39_asumyi }
    // Java: void setAsumyi(byte[] field_39_asumyi)
    pub fn setAsumyi(&mut self, v: Vec<u8>) { // Java: this.field_39_asumyi = field_39_asumyi;
      self.field_39_asumyi = v; }
    // Java: int getCChWS()
    pub fn getCChWS(&self) -> i32 { // Java: return this.field_40_cChWS;
      self.field_40_cChWS }
    // Java: void setCChWS(int field_40_cChWS)
    pub fn setCChWS(&mut self, v: i32) { // Java: this.field_40_cChWS = field_40_cChWS;
      self.field_40_cChWS = v; }
    // Java: int getCChWSFtnEdn()
    pub fn getCChWSFtnEdn(&self) -> i32 { // Java: return this.field_41_cChWSFtnEdn;
      self.field_41_cChWSFtnEdn }
    // Java: void setCChWSFtnEdn(int field_41_cChWSFtnEdn)
    pub fn setCChWSFtnEdn(&mut self, v: i32) { // Java: this.field_41_cChWSFtnEdn = field_41_cChWSFtnEdn;
      self.field_41_cChWSFtnEdn = v; }
    // Java: int getGrfDocEvents()
    pub fn getGrfDocEvents(&self) -> i32 { // Java: return this.field_42_grfDocEvents;
      self.field_42_grfDocEvents }
    // Java: void setGrfDocEvents(int field_42_grfDocEvents)
    pub fn setGrfDocEvents(&mut self, v: i32) { // Java: this.field_42_grfDocEvents = field_42_grfDocEvents;
      self.field_42_grfDocEvents = v; }
    // Java: int getVirusinfo()
    pub fn getVirusinfo(&self) -> i32 { // Java: return this.field_43_virusinfo;
      self.field_43_virusinfo }
    // Java: void setVirusinfo(int field_43_virusinfo)
    pub fn setVirusinfo(&mut self, v: i32) { // Java: this.field_43_virusinfo = field_43_virusinfo;
      self.field_43_virusinfo = v; }
    // Java: byte[] getSpare()
    pub fn getSpare(&self) -> &Vec<u8> { // Java: return this.field_44_Spare;
      &self.field_44_Spare }
    // Java: void setSpare(byte[] field_44_Spare)
    pub fn setSpare(&mut self, v: Vec<u8>) { // Java: this.field_44_Spare = field_44_Spare;
      self.field_44_Spare = v; }
    // Java: int getReserved1()
    pub fn getReserved1(&self) -> i32 { // Java: return this.field_45_reserved1;
      self.field_45_reserved1 }
    // Java: void setReserved1(int field_45_reserved1)
    pub fn setReserved1(&mut self, v: i32) { // Java: this.field_45_reserved1 = field_45_reserved1;
      self.field_45_reserved1 = v; }
    // Java: int getReserved2()
    pub fn getReserved2(&self) -> i32 { // Java: return this.field_46_reserved2;
      self.field_46_reserved2 }
    // Java: void setReserved2(int field_46_reserved2)
    pub fn setReserved2(&mut self, v: i32) { // Java: this.field_46_reserved2 = field_46_reserved2;
      self.field_46_reserved2 = v; }
    // Java: int getCDBC()
    pub fn getCDBC(&self) -> i32 { // Java: return this.field_47_cDBC;
      self.field_47_cDBC }
    // Java: void setCDBC(int field_47_cDBC)
    pub fn setCDBC(&mut self, v: i32) { // Java: this.field_47_cDBC = field_47_cDBC;
      self.field_47_cDBC = v; }
    // Java: int getCDBCFtnEdn()
    pub fn getCDBCFtnEdn(&self) -> i32 { // Java: return this.field_48_cDBCFtnEdn;
      self.field_48_cDBCFtnEdn }
    // Java: void setCDBCFtnEdn(int field_48_cDBCFtnEdn)
    pub fn setCDBCFtnEdn(&mut self, v: i32) { // Java: this.field_48_cDBCFtnEdn = field_48_cDBCFtnEdn;
      self.field_48_cDBCFtnEdn = v; }
    // Java: int getReserved()
    pub fn getReserved(&self) -> i32 { // Java: return this.field_49_reserved;
      self.field_49_reserved }
    // Java: void setReserved(int field_49_reserved)
    pub fn setReserved(&mut self, v: i32) { // Java: this.field_49_reserved = field_49_reserved;
      self.field_49_reserved = v; }
    // Java: short getNfcFtnRef()
    pub fn getNfcFtnRef(&self) -> i16 { // Java: return this.field_50_nfcFtnRef;
      self.field_50_nfcFtnRef }
    // Java: void setNfcFtnRef(short field_50_nfcFtnRef)
    pub fn setNfcFtnRef(&mut self, v: i16) { // Java: this.field_50_nfcFtnRef = field_50_nfcFtnRef;
      self.field_50_nfcFtnRef = v; }
    // Java: short getNfcEdnRef()
    pub fn getNfcEdnRef(&self) -> i16 { // Java: return this.field_51_nfcEdnRef;
      self.field_51_nfcEdnRef }
    // Java: void setNfcEdnRef(short field_51_nfcEdnRef)
    pub fn setNfcEdnRef(&mut self, v: i16) { // Java: this.field_51_nfcEdnRef = field_51_nfcEdnRef;
      self.field_51_nfcEdnRef = v; }
    // Java: short getHpsZoonFontPag()
    pub fn getHpsZoonFontPag(&self) -> i16 { // Java: return this.field_52_hpsZoonFontPag;
      self.field_52_hpsZoonFontPag }
    // Java: void setHpsZoonFontPag(short field_52_hpsZoonFontPag)
    pub fn setHpsZoonFontPag(&mut self, v: i16) { // Java: this.field_52_hpsZoonFontPag = field_52_hpsZoonFontPag;
      self.field_52_hpsZoonFontPag = v; }
    // Java: short getDywDispPag()
    pub fn getDywDispPag(&self) -> i16 { // Java: return this.field_53_dywDispPag;
      self.field_53_dywDispPag }
    // Java: void setDywDispPag(short field_53_dywDispPag)
    pub fn setDywDispPag(&mut self, v: i16) { // Java: this.field_53_dywDispPag = field_53_dywDispPag;
      self.field_53_dywDispPag = v; }
    // Java: Dop2007 getDop07()
    pub fn getDop07(&self) -> &Dop2007 { // Java: return this.field_57_dop07;
      &self.field_57_dop07 }

    // BitField-based accessors using inline BitField::new(mask)
    // Java: boolean isFFacingPages()
    pub fn isFFacingPages(&self) -> bool { // Java: return fFacingPages.isSet(this.field_1_formatFlags);
      BitField::new(1).isSet(self.field_1_formatFlags as i32) }
    // Java: void setFFacingPages(boolean value)
    pub fn setFFacingPages(&mut self, value: bool) { // Java: this.field_1_formatFlags = (byte)fFacingPages.setBoolean(this.field_1_formatFlags, value);
      self.field_1_formatFlags = BitField::new(1).setBoolean(self.field_1_formatFlags as i32, value) as i8; }
    // Java: boolean isFWidowControl()
    pub fn isFWidowControl(&self) -> bool { // Java: return fWidowControl.isSet(this.field_1_formatFlags);
      BitField::new(2).isSet(self.field_1_formatFlags as i32) }
    // Java: void setFWidowControl(boolean value)
    pub fn setFWidowControl(&mut self, value: bool) { // Java: this.field_1_formatFlags = (byte)fWidowControl.setBoolean(this.field_1_formatFlags, value);
      self.field_1_formatFlags = BitField::new(2).setBoolean(self.field_1_formatFlags as i32, value) as i8; }
    // Java: boolean isFPMHMainDoc()
    pub fn isFPMHMainDoc(&self) -> bool { // Java: return fPMHMainDoc.isSet(this.field_1_formatFlags);
      BitField::new(4).isSet(self.field_1_formatFlags as i32) }
    // Java: void setFPMHMainDoc(boolean value)
    pub fn setFPMHMainDoc(&mut self, value: bool) { // Java: this.field_1_formatFlags = (byte)fPMHMainDoc.setBoolean(this.field_1_formatFlags, value);
      self.field_1_formatFlags = BitField::new(4).setBoolean(self.field_1_formatFlags as i32, value) as i8; }
    // Java: void setGrfSupression(byte value)
    pub fn setGrfSupression(&mut self, value: i8) { // Java: this.field_1_formatFlags = (byte)grfSupression.setValue(this.field_1_formatFlags, value);
      self.field_1_formatFlags = BitField::new(24).setValue(self.field_1_formatFlags as i32, value as i32) as i8; }
    // Java: byte getGrfSupression()
    pub fn getGrfSupression(&self) -> i8 { // Java: return (byte)grfSupression.getValue(this.field_1_formatFlags);
      BitField::new(24).getValue(self.field_1_formatFlags as i32) as i8 }
    // Java: void setFpc(byte value)
    pub fn setFpc(&mut self, value: i8) { // Java: this.field_1_formatFlags = (byte)fpc.setValue(this.field_1_formatFlags, value);
      self.field_1_formatFlags = BitField::new(96).setValue(self.field_1_formatFlags as i32, value as i32) as i8; }
    // Java: byte getFpc()
    pub fn getFpc(&self) -> i8 { // Java: return (byte)fpc.getValue(this.field_1_formatFlags);
      BitField::new(96).getValue(self.field_1_formatFlags as i32) as i8 }
    // Java: boolean isUnused1()
    pub fn isUnused1(&self) -> bool { // Java: return unused1.isSet(this.field_1_formatFlags);
      BitField::new(128).isSet(self.field_1_formatFlags as i32) }
    // Java: void setUnused1(boolean value)
    pub fn setUnused1(&mut self, value: bool) { // Java: this.field_1_formatFlags = (byte)unused1.setBoolean(this.field_1_formatFlags, value);
      self.field_1_formatFlags = BitField::new(128).setBoolean(self.field_1_formatFlags as i32, value) as i8; }
    // Java: void setRncFtn(byte value)
    pub fn setRncFtn(&mut self, value: i8) { // Java: this.field_3_footnoteInfo = (short)rncFtn.setValue(this.field_3_footnoteInfo, value);
      self.field_3_footnoteInfo = BitField::new(3).setValue(self.field_3_footnoteInfo as i32, value as i32) as i16; }
    // Java: byte getRncFtn()
    pub fn getRncFtn(&self) -> i8 { // Java: return (byte)rncFtn.getValue(this.field_3_footnoteInfo);
      BitField::new(3).getValue(self.field_3_footnoteInfo as i32) as i8 }
    // Java: void setNFtn(short value)
    pub fn setNFtn(&mut self, value: i16) { // Java: this.field_3_footnoteInfo = (short)nFtn.setValue(this.field_3_footnoteInfo, value);
      self.field_3_footnoteInfo = BitField::new(65532).setValue(self.field_3_footnoteInfo as i32, value as i32) as i16; }
    // Java: short getNFtn()
    pub fn getNFtn(&self) -> i16 { // Java: return (short)nFtn.getValue(this.field_3_footnoteInfo);
      BitField::new(65532).getValue(self.field_3_footnoteInfo as i32) as i16 }
    // Java: boolean isFOnlyMacPics()
    pub fn isFOnlyMacPics(&self) -> bool { BitField::new(1).isSet(self.field_5_docinfo as i32) }
    // Java: boolean isFOnlyWinPics()
    pub fn isFOnlyWinPics(&self) -> bool { BitField::new(2).isSet(self.field_5_docinfo as i32) }
    // Java: boolean isFLabelDoc()
    pub fn isFLabelDoc(&self) -> bool { BitField::new(4).isSet(self.field_5_docinfo as i32) }
    // Java: boolean isFHyphCapitals()
    pub fn isFHyphCapitals(&self) -> bool { BitField::new(8).isSet(self.field_5_docinfo as i32) }
    // Java: boolean isFAutoHyphen()
    pub fn isFAutoHyphen(&self) -> bool { BitField::new(16).isSet(self.field_5_docinfo as i32) }
    // Java: boolean isFFormNoFields()
    pub fn isFFormNoFields(&self) -> bool { BitField::new(32).isSet(self.field_5_docinfo as i32) }
    // Java: boolean isFLinkStyles()
    pub fn isFLinkStyles(&self) -> bool { BitField::new(64).isSet(self.field_5_docinfo as i32) }
    // Java: boolean isFRevMarking()
    pub fn isFRevMarking(&self) -> bool { BitField::new(128).isSet(self.field_5_docinfo as i32) }
    // Java: boolean isFBackup()
    pub fn isFBackup(&self) -> bool { BitField::new(1).isSet(self.field_6_docinfo1 as i32) }
    // Java: boolean isFExactCWords()
    pub fn isFExactCWords(&self) -> bool { BitField::new(2).isSet(self.field_6_docinfo1 as i32) }
    // Java: boolean isFPagHidden()
    pub fn isFPagHidden(&self) -> bool { BitField::new(4).isSet(self.field_6_docinfo1 as i32) }
    // Java: boolean isFPagResults()
    pub fn isFPagResults(&self) -> bool { BitField::new(8).isSet(self.field_6_docinfo1 as i32) }
    // Java: boolean isFLockAtn()
    pub fn isFLockAtn(&self) -> bool { BitField::new(16).isSet(self.field_6_docinfo1 as i32) }
    // Java: boolean isFMirrorMargins()
    pub fn isFMirrorMargins(&self) -> bool { BitField::new(32).isSet(self.field_6_docinfo1 as i32) }
    // Java: boolean isUnused3()
    pub fn isUnused3(&self) -> bool { BitField::new(64).isSet(self.field_6_docinfo1 as i32) }
    // Java: boolean isFDfltTrueType()
    pub fn isFDfltTrueType(&self) -> bool { BitField::new(128).isSet(self.field_6_docinfo1 as i32) }
    // Java: boolean isFPagSupressTopSpacing()
    pub fn isFPagSupressTopSpacing(&self) -> bool { BitField::new(1).isSet(self.field_7_docinfo2 as i32) }
    // Java: boolean isFProtEnabled()
    pub fn isFProtEnabled(&self) -> bool { BitField::new(2).isSet(self.field_7_docinfo2 as i32) }
    // Java: boolean isFDispFormFldSel()
    pub fn isFDispFormFldSel(&self) -> bool { BitField::new(4).isSet(self.field_7_docinfo2 as i32) }
    // Java: boolean isFRMView()
    pub fn isFRMView(&self) -> bool { BitField::new(8).isSet(self.field_7_docinfo2 as i32) }
    // Java: boolean isFRMPrint()
    pub fn isFRMPrint(&self) -> bool { BitField::new(16).isSet(self.field_7_docinfo2 as i32) }
    // Java: boolean isUnused4()
    pub fn isUnused4(&self) -> bool { BitField::new(32).isSet(self.field_7_docinfo2 as i32) }
    // Java: boolean isFLockRev()
    pub fn isFLockRev(&self) -> bool { BitField::new(64).isSet(self.field_7_docinfo2 as i32) }
    // Java: boolean isFEmbedFonts()
    pub fn isFEmbedFonts(&self) -> bool { BitField::new(128).isSet(self.field_7_docinfo2 as i32) }
    // Java: boolean isFVirusPrompted()
    pub fn isFVirusPrompted(&self) -> bool { BitField::new(1).isSet(self.field_43_virusinfo) }
    // Java: boolean isFVirusLoadSafe()
    pub fn isFVirusLoadSafe(&self) -> bool { BitField::new(2).isSet(self.field_43_virusinfo) }
    // Java: int getKeyVirusSession30()
    pub fn getKeyVirusSession30(&self) -> i32 { BitField::new(-4).getValue(self.field_43_virusinfo) }
    // Java: boolean isFHaveVersions()
    pub fn isFHaveVersions(&self) -> bool { BitField::new(1).isSet(self.field_38_docinfo6 as i32) }
    // Java: boolean isFAutoVersions()
    pub fn isFAutoVersions(&self) -> bool { BitField::new(2).isSet(self.field_38_docinfo6 as i32) }
    // Java: boolean isFTreatLockAtnAsReadOnly()
    pub fn isFTreatLockAtnAsReadOnly(&self) -> bool {
      if self.field_56_Dop2003.len() > 0 { BitField::new(1).isSet(le_get_short(&self.field_56_Dop2003, 0) as i32) } else { false }
    }
    // Java: boolean isFStyleLock()
    pub fn isFStyleLock(&self) -> bool {
      if self.field_56_Dop2003.len() > 0 { BitField::new(2).isSet(le_get_short(&self.field_56_Dop2003, 0) as i32) } else { false }
    }
    // Java: boolean isFStyleLockEnforced()
    pub fn isFStyleLockEnforced(&self) -> bool {
      if self.field_56_Dop2003.len() > 0 { BitField::new(32).isSet(le_get_short(&self.field_56_Dop2003, 0) as i32) } else { false }
    }
    // Java: void setFTreatLockAtnAsReadOnly(boolean value)
    pub fn setFTreatLockAtnAsReadOnly(&mut self, value: bool) {
      if self.field_56_Dop2003.len() > 0 {
        let info = BitField::new(1).setBoolean(le_get_short(&self.field_56_Dop2003, 0) as i32, value) as i16;
        le_put_short(&mut self.field_56_Dop2003, 0, info);
      }
    }
    // Java: void setFStyleLock(boolean value)
    pub fn setFStyleLock(&mut self, value: bool) {
      if self.field_56_Dop2003.len() > 0 {
        let info = BitField::new(2).setBoolean(le_get_short(&self.field_56_Dop2003, 0) as i32, value) as i16;
        le_put_short(&mut self.field_56_Dop2003, 0, info);
      }
    }
    // Java: void setFStyleLockEnforced(boolean value)
    pub fn setFStyleLockEnforced(&mut self, value: bool) {
      if self.field_56_Dop2003.len() > 0 {
        let info = BitField::new(32).setBoolean(le_get_short(&self.field_56_Dop2003, 0) as i32, value) as i16;
        le_put_short(&mut self.field_56_Dop2003, 0, info);
      }
    }
    // Java: byte getCustomKsu()
    pub fn getCustomKsu(&self) -> i8 {
      let info = le_get_short(&self.field_35_doptypography, 0);
      BitField::new(896).getShortValue(info) as i8
    }
    // Java: void setCustomKsu(int customKsu)
    pub fn setCustomKsu(&mut self, v: i32) {
      let info = le_get_short(&self.field_35_doptypography, 0);
      let info2 = BitField::new(896).setValue(info as i32, v) as i16;
      le_put_short(&mut self.field_35_doptypography, 0, info2);
    }
    // Java: int getEndPunctLength()
    pub fn getEndPunctLength(&self) -> i32 { le_get_short(&self.field_35_doptypography, 2) as i32 }
    // Java: void setEndPunctLength(int endPunctLen)
    pub fn setEndPunctLength(&mut self, v: i32) { le_put_short(&mut self.field_35_doptypography, 2, v as i16); }
    // Java: String getEndPunct()
    pub fn getEndPunct(&self) -> String {
      let mut length = self.getEndPunctLength() as usize;
      if length > 101 { length = 101; }
      let buf = &self.field_35_doptypography[6..208.min(self.field_35_doptypography.len())];
      StringUtil::getFromUnicodeLE(buf, 0, length)
    }
    // Java: void setEndPunct(String endPunct)
    pub fn setEndPunct(&mut self, s: &str) {
      self.setEndPunctLength(s.len() as i32);
      StringUtil::putUnicodeLE(s, &mut self.field_35_doptypography, 6);
    }
    // Java: short getStartPunctLength()
    pub fn getStartPunctLength(&self) -> i16 { le_get_short(&self.field_35_doptypography, 4) }
    // Java: void setStartPunctLength(int startPunctLen)
    pub fn setStartPunctLength(&mut self, v: i32) { le_put_short(&mut self.field_35_doptypography, 4, v as i16); }
    // Java: String getStartPunct()
    pub fn getStartPunct(&self) -> String {
      let mut length = self.getStartPunctLength() as usize;
      if length > 51 { length = 51; }
      let buf = &self.field_35_doptypography[208..310.min(self.field_35_doptypography.len())];
      StringUtil::getFromUnicodeLE(buf, 0, length)
    }
    // Java: void setStartPunct(String startPunct)
    pub fn setStartPunct(&mut self, s: &str) {
      self.setStartPunctLength(s.len() as i32);
      StringUtil::putUnicodeLE(s, &mut self.field_35_doptypography, 208);
    }
    // Java: byte getJustification()
    pub fn getJustification(&self) -> i8 { BitField::new(6).getShortValue(self.field_35_doptypography[0] as i16) as i8 }
    // Java: void setJustification(int compressLevel)
    pub fn setJustification(&mut self, v: i32) { self.field_35_doptypography[0] = BitField::new(6).setShortValue(self.field_35_doptypography[0] as i16, v as i16) as u8; }
    // Java: byte getLevelOfKinsoku()
    pub fn getLevelOfKinsoku(&self) -> i8 { BitField::new(24).getShortValue(self.field_35_doptypography[0] as i16) as i8 }
    // Java: void setLevelOfKinsoku(int levelOfKinsoku)
    pub fn setLevelOfKinsoku(&mut self, v: i32) { self.field_35_doptypography[0] = BitField::new(24).setShortValue(self.field_35_doptypography[0] as i16, v as i16) as u8; }
    // Java: boolean isF2on1()
    pub fn isF2on1(&self) -> bool { BitField::new(32).isSet(self.field_35_doptypography[0] as i32) }
    // Java: void setF2on1(boolean value)
    pub fn setF2on1(&mut self, value: bool) { self.field_35_doptypography[0] = BitField::new(32).setBoolean(self.field_35_doptypography[0] as i32, value) as u8; }
    // Java: boolean isFKerningPunct()
    pub fn isFKerningPunct(&self) -> bool { BitField::new(1).isSet(self.field_35_doptypography[0] as i32) }
    // Java: void setFKerningPunct(boolean kerningPunct)
    pub fn setFKerningPunct(&mut self, value: bool) { self.field_35_doptypography[0] = BitField::new(1).setBoolean(self.field_35_doptypography[0] as i32, value) as u8; }
    // Java: short getXaGrid()
    pub fn getXaGrid(&self) -> i16 { le_get_short(&self.field_36_dogrid, 0) }
    // Java: void setXaGrid(short value)
    pub fn setXaGrid(&mut self, v: i16) { le_put_short(&mut self.field_36_dogrid, 0, v); }
    // Java: short getYaGrid()
    pub fn getYaGrid(&self) -> i16 { le_get_short(&self.field_36_dogrid, 2) }
    // Java: void setYaGrid(short value)
    pub fn setYaGrid(&mut self, v: i16) { le_put_short(&mut self.field_36_dogrid, 2, v); }
    // Java: short getDxaGrid()
    pub fn getDxaGrid(&self) -> i16 { le_get_short(&self.field_36_dogrid, 4) }
    // Java: void setDxaGrid(short value)
    pub fn setDxaGrid(&mut self, v: i16) { le_put_short(&mut self.field_36_dogrid, 4, v); }
    // Java: short getDyaGrid()
    pub fn getDyaGrid(&self) -> i16 { le_get_short(&self.field_36_dogrid, 6) }
    // Java: void setDyaGrid(short value)
    pub fn setDyaGrid(&mut self, v: i16) { le_put_short(&mut self.field_36_dogrid, 6, v); }
}

impl HDFType for DOPAbstractType {}
