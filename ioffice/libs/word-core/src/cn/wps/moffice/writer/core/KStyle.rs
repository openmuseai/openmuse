// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
use util::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::MutablePropertySet;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.transaction.Atom
use util::moffice::transaction::Atom::Atom;
// Java: import cn.wps.moffice.transaction.AtomMember
use util::moffice::transaction::AtomMember::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::Transaction;
// Java: import cn.wps.moffice.writer.data.DefaultValue
// soft: DefaultValue not in data/mod (large dep graph)
// Java: import cn.wps.moffice.writer.data.value.Toggle
// soft: value/ not exported from data/mod yet
// Java: import cn.wps.moffice.writer.core.TextDocument
// soft: TextDocument not in core/mod
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::KStyles;
use crate::cn::wps::collections::textrope::ITextPool::ITextPool;
// Java: import java.lang.Object
use stubs::java::lang::Object::Object;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

/// Soft stand-in for writer.data.value.Toggle
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Toggle {
    OFF,
    ON,
    CURRENT,
    OPPOSITE,
}

/// Soft stand-in for writer.data.DefaultValue
pub struct DefaultValue;
impl DefaultValue {
    // java public static final Toggle run_bidi = null;
    pub fn run_bidi() -> Option<Toggle> {
        None
    }
}

/// Soft stand-in for core.TextDocument (minimal API used by KStyle and SubDocument parent)
#[derive(Clone)]
pub struct TextDocument {
    pub transaction: Option<Rc<RefCell<Transaction>>>,
    // typeoracle: Java ITextPool mCharBuffer shared by every SubDocument rope
    pub textPool: Option<*mut dyn ITextPool>,
    // typeoracle: Java KStyles styles; SubDocument.getStyles delegates to this pointer
    pub styles: Option<*mut KStyles>,
}
impl Default for TextDocument {
    fn default() -> Self {
        Self {
            transaction: None,
            textPool: None,
            styles: None,
        }
    }
}
impl TextDocument {
    pub fn getTransaction(&self) -> Option<Rc<RefCell<Transaction>>> {
        self.transaction.clone()
    }
    pub fn getTextPool(&self) -> Option<*mut dyn ITextPool> {
        self.textPool
    }
    pub fn getStyles(&self) -> Option<*mut KStyles> {
        self.styles
    }
    pub fn updateDocument(&mut self) {
        // soft stub
    }
}

/// Soft stand-in for core.PropertyTool
pub struct PropertyTool;
impl PropertyTool {
    pub fn clearCache() {
        // soft stub
    }
}

fn property_set_empty() -> PropertySet {
    (*empty).clone()
}

// java static final int[] spaceBeforeTypes = new int[]{217, 218, 219};
pub(crate) static spaceBeforeTypes: std::sync::LazyLock<Vec<i32>> =
                    std::sync::LazyLock::new(|| vec![217, 218, 219]);
// java static final int[] spaceAfterTypes = new int[]{214, 215, 216};
pub(crate) static spaceAfterTypes: std::sync::LazyLock<Vec<i32>> =
                    std::sync::LazyLock::new(|| vec![214, 215, 216]);
// java public static final int PARAGRAPH_STYLE = 1;
pub(crate) const PARAGRAPH_STYLE: i32 = 1;
// java public static final int CHARACTER_STYLE = 2;
pub(crate) const CHARACTER_STYLE: i32 = 2;
// java public static final int TABLE_STYLE = 3;
pub(crate) const TABLE_STYLE: i32 = 3;
// java public static final int NUMBERING_STYLE = 4;
pub(crate) const NUMBERING_STYLE: i32 = 4;
// java public static final int LINKED_STYLE = 5;
pub(crate) const LINKED_STYLE: i32 = 5;
// java public static final int stiNormal = 0;
pub(crate) const stiNormal: i32 = 0;
// java public static final int stiHeading1 = 1;
pub(crate) const stiHeading1: i32 = 1;
// java public static final int stiHeading2 = 2;
pub(crate) const stiHeading2: i32 = 2;
// java public static final int stiHeading3 = 3;
pub(crate) const stiHeading3: i32 = 3;
// java public static final int stiHeading4 = 4;
pub(crate) const stiHeading4: i32 = 4;
// java public static final int stiHeading5 = 5;
pub(crate) const stiHeading5: i32 = 5;
// java public static final int stiHeading6 = 6;
pub(crate) const stiHeading6: i32 = 6;
// java public static final int stiHeading7 = 7;
pub(crate) const stiHeading7: i32 = 7;
// java public static final int stiHeading8 = 8;
pub(crate) const stiHeading8: i32 = 8;
// java public static final int stiHeading9 = 9;
pub(crate) const stiHeading9: i32 = 9;
// java public static final int stiHeadingFirst = 1;
pub(crate) const stiHeadingFirst: i32 = 1;
// java public static final int stiHeadingLast = 9;
pub(crate) const stiHeadingLast: i32 = 9;
// java public static final int stiIndex1 = 10;
pub(crate) const stiIndex1: i32 = 10;
// java public static final int stiIndex2 = 11;
pub(crate) const stiIndex2: i32 = 11;
// java public static final int stiIndex3 = 12;
pub(crate) const stiIndex3: i32 = 12;
// java public static final int stiIndex4 = 13;
pub(crate) const stiIndex4: i32 = 13;
// java public static final int stiIndex5 = 14;
pub(crate) const stiIndex5: i32 = 14;
// java public static final int stiIndex6 = 15;
pub(crate) const stiIndex6: i32 = 15;
// java public static final int stiIndex7 = 16;
pub(crate) const stiIndex7: i32 = 16;
// java public static final int stiIndex8 = 17;
pub(crate) const stiIndex8: i32 = 17;
// java public static final int stiIndex9 = 18;
pub(crate) const stiIndex9: i32 = 18;
// java public static final int stiToc1 = 19;
pub(crate) const stiToc1: i32 = 19;
// java public static final int stiToc2 = 20;
pub(crate) const stiToc2: i32 = 20;
// java public static final int stiToc3 = 21;
pub(crate) const stiToc3: i32 = 21;
// java public static final int stiToc4 = 22;
pub(crate) const stiToc4: i32 = 22;
// java public static final int stiToc5 = 23;
pub(crate) const stiToc5: i32 = 23;
// java public static final int stiToc6 = 24;
pub(crate) const stiToc6: i32 = 24;
// java public static final int stiToc7 = 25;
pub(crate) const stiToc7: i32 = 25;
// java public static final int stiToc8 = 26;
pub(crate) const stiToc8: i32 = 26;
// java public static final int stiToc9 = 27;
pub(crate) const stiToc9: i32 = 27;
// java public static final int stiNormalIndent = 28;
pub(crate) const stiNormalIndent: i32 = 28;
// java public static final int stiFootnoteText = 29;
pub(crate) const stiFootnoteText: i32 = 29;
// java public static final int stiAnnotationText = 30;
pub(crate) const stiAnnotationText: i32 = 30;
// java public static final int stiHeader = 31;
pub(crate) const stiHeader: i32 = 31;
// java public static final int stiFooter = 32;
pub(crate) const stiFooter: i32 = 32;
// java public static final int stiIndexHeading = 33;
pub(crate) const stiIndexHeading: i32 = 33;
// java public static final int stiCaption = 34;
pub(crate) const stiCaption: i32 = 34;
// java public static final int stiTableOfFigures = 35;
pub(crate) const stiTableOfFigures: i32 = 35;
// java public static final int stiEnvelopeAddress = 36;
pub(crate) const stiEnvelopeAddress: i32 = 36;
// java public static final int stiEnvelopeReturn = 37;
pub(crate) const stiEnvelopeReturn: i32 = 37;
// java public static final int stiFootnoteReference = 38;
pub(crate) const stiFootnoteReference: i32 = 38;
// java public static final int stiAnnotationReference = 39;
pub(crate) const stiAnnotationReference: i32 = 39;
// java public static final int stiLineNumber = 40;
pub(crate) const stiLineNumber: i32 = 40;
// java public static final int stiPageNumber = 41;
pub(crate) const stiPageNumber: i32 = 41;
// java public static final int stiEndnoteReference = 42;
pub(crate) const stiEndnoteReference: i32 = 42;
// java public static final int stiEndnoteText = 43;
pub(crate) const stiEndnoteText: i32 = 43;
// java public static final int stiTableOfAuthorities = 44;
pub(crate) const stiTableOfAuthorities: i32 = 44;
// java public static final int stiMacro = 45;
pub(crate) const stiMacro: i32 = 45;
// java public static final int stiToaHeading = 46;
pub(crate) const stiToaHeading: i32 = 46;
// java public static final int stiList = 47;
pub(crate) const stiList: i32 = 47;
// java public static final int stiListBullet = 48;
pub(crate) const stiListBullet: i32 = 48;
// java public static final int stiListNumber = 49;
pub(crate) const stiListNumber: i32 = 49;
// java public static final int stiList2 = 50;
pub(crate) const stiList2: i32 = 50;
// java public static final int stiList3 = 51;
pub(crate) const stiList3: i32 = 51;
// java public static final int stiList4 = 52;
pub(crate) const stiList4: i32 = 52;
// java public static final int stiList5 = 53;
pub(crate) const stiList5: i32 = 53;
// java public static final int stiListBullet2 = 54;
pub(crate) const stiListBullet2: i32 = 54;
// java public static final int stiListBullet3 = 55;
pub(crate) const stiListBullet3: i32 = 55;
// java public static final int stiListBullet4 = 56;
pub(crate) const stiListBullet4: i32 = 56;
// java public static final int stiListBullet5 = 57;
pub(crate) const stiListBullet5: i32 = 57;
// java public static final int stiListNumber2 = 58;
pub(crate) const stiListNumber2: i32 = 58;
// java public static final int stiListNumber3 = 59;
pub(crate) const stiListNumber3: i32 = 59;
// java public static final int stiListNumber4 = 60;
pub(crate) const stiListNumber4: i32 = 60;
// java public static final int stiListNumber5 = 61;
pub(crate) const stiListNumber5: i32 = 61;
// java public static final int stiTitle = 62;
pub(crate) const stiTitle: i32 = 62;
// java public static final int stiClosing = 63;
pub(crate) const stiClosing: i32 = 63;
// java public static final int stiSignature = 64;
pub(crate) const stiSignature: i32 = 64;
// java public static final int stiDefaultParagraphFont = 65;
pub(crate) const stiDefaultParagraphFont: i32 = 65;
// java public static final int stiBodyText = 66;
pub(crate) const stiBodyText: i32 = 66;
// java public static final int stiBodyTextIndent = 67;
pub(crate) const stiBodyTextIndent: i32 = 67;
// java public static final int stiListContinue = 68;
pub(crate) const stiListContinue: i32 = 68;
// java public static final int stiListContinue2 = 69;
pub(crate) const stiListContinue2: i32 = 69;
// java public static final int stiListContinue3 = 70;
pub(crate) const stiListContinue3: i32 = 70;
// java public static final int stiListContinue4 = 71;
pub(crate) const stiListContinue4: i32 = 71;
// java public static final int stiListContinue5 = 72;
pub(crate) const stiListContinue5: i32 = 72;
// java public static final int stiMessageHeader = 73;
pub(crate) const stiMessageHeader: i32 = 73;
// java public static final int stiSubtitle = 74;
pub(crate) const stiSubtitle: i32 = 74;
// java public static final int stiSalutation = 75;
pub(crate) const stiSalutation: i32 = 75;
// java public static final int stiDate = 76;
pub(crate) const stiDate: i32 = 76;
// java public static final int stiBodyTextFirstIndent = 77;
pub(crate) const stiBodyTextFirstIndent: i32 = 77;
// java public static final int stiBodyTextFirstIndent2 = 78;
pub(crate) const stiBodyTextFirstIndent2: i32 = 78;
// java public static final int stiNoteHeading = 79;
pub(crate) const stiNoteHeading: i32 = 79;
// java public static final int stiBodyText2 = 80;
pub(crate) const stiBodyText2: i32 = 80;
// java public static final int stiBodyText3 = 81;
pub(crate) const stiBodyText3: i32 = 81;
// java public static final int stiBodyTextIndent2 = 82;
pub(crate) const stiBodyTextIndent2: i32 = 82;
// java public static final int stiBodyTextIndent3 = 83;
pub(crate) const stiBodyTextIndent3: i32 = 83;
// java public static final int stiBlockText = 84;
pub(crate) const stiBlockText: i32 = 84;
// java public static final int stiHyperlink = 85;
pub(crate) const stiHyperlink: i32 = 85;
// java public static final int stiFollowedHyperlink = 86;
pub(crate) const stiFollowedHyperlink: i32 = 86;
// java public static final int stiStrong = 87;
pub(crate) const stiStrong: i32 = 87;
// java public static final int stiEmphasis = 88;
pub(crate) const stiEmphasis: i32 = 88;
// java public static final int stiDocumentMap = 89;
pub(crate) const stiDocumentMap: i32 = 89;
// java public static final int stiPlainText = 90;
pub(crate) const stiPlainText: i32 = 90;
// java public static final int stiEmailSignature = 91;
pub(crate) const stiEmailSignature: i32 = 91;
// java public static final int stiHTMLTopOfForm = 92;
pub(crate) const stiHTMLTopOfForm: i32 = 92;
// java public static final int stiHTMLBottomOfForm = 93;
pub(crate) const stiHTMLBottomOfForm: i32 = 93;
// java public static final int stiNormalWeb = 94;
pub(crate) const stiNormalWeb: i32 = 94;
// java public static final int stiHTMLAcronym = 95;
pub(crate) const stiHTMLAcronym: i32 = 95;
// java public static final int stiHTMLAddress = 96;
pub(crate) const stiHTMLAddress: i32 = 96;
// java public static final int stiHTMLCite = 97;
pub(crate) const stiHTMLCite: i32 = 97;
// java public static final int stiHTMLCode = 98;
pub(crate) const stiHTMLCode: i32 = 98;
// java public static final int stiHTMLDefinition = 99;
pub(crate) const stiHTMLDefinition: i32 = 99;
// java public static final int stiHTMLKeyboard = 100;
pub(crate) const stiHTMLKeyboard: i32 = 100;
// java public static final int stiHTMLPreformatted = 101;
pub(crate) const stiHTMLPreformatted: i32 = 101;
// java public static final int stiHTMLSample = 102;
pub(crate) const stiHTMLSample: i32 = 102;
// java public static final int stiHTMLTypewriter = 103;
pub(crate) const stiHTMLTypewriter: i32 = 103;
// java public static final int stiHTMLVariable = 104;
pub(crate) const stiHTMLVariable: i32 = 104;
// java public static final int stiNormalTable = 105;
pub(crate) const stiNormalTable: i32 = 105;
// java public static final int stiAnnotationSubject = 106;
pub(crate) const stiAnnotationSubject: i32 = 106;
// java public static final int stiNoList = 107;
pub(crate) const stiNoList: i32 = 107;
// java public static final int stiOutlineList1 = 108;
pub(crate) const stiOutlineList1: i32 = 108;
// java public static final int stiOutlineList2 = 109;
pub(crate) const stiOutlineList2: i32 = 109;
// java public static final int stiOutlineList3 = 110;
pub(crate) const stiOutlineList3: i32 = 110;
// java public static final int stiTableSimple1 = 111;
pub(crate) const stiTableSimple1: i32 = 111;
// java public static final int stiTableSimple2 = 112;
pub(crate) const stiTableSimple2: i32 = 112;
// java public static final int stiTableSimple3 = 113;
pub(crate) const stiTableSimple3: i32 = 113;
// java public static final int stiTableClassic1 = 114;
pub(crate) const stiTableClassic1: i32 = 114;
// java public static final int stiTableClassic2 = 115;
pub(crate) const stiTableClassic2: i32 = 115;
// java public static final int stiTableClassic3 = 116;
pub(crate) const stiTableClassic3: i32 = 116;
// java public static final int stiTableClassic4 = 117;
pub(crate) const stiTableClassic4: i32 = 117;
// java public static final int stiTableColorful1 = 118;
pub(crate) const stiTableColorful1: i32 = 118;
// java public static final int stiTableColorful2 = 119;
pub(crate) const stiTableColorful2: i32 = 119;
// java public static final int stiTableColorful3 = 120;
pub(crate) const stiTableColorful3: i32 = 120;
// java public static final int stiTableColumns1 = 121;
pub(crate) const stiTableColumns1: i32 = 121;
// java public static final int stiTableColumns2 = 122;
pub(crate) const stiTableColumns2: i32 = 122;
// java public static final int stiTableColumns3 = 123;
pub(crate) const stiTableColumns3: i32 = 123;
// java public static final int stiTableColumns4 = 124;
pub(crate) const stiTableColumns4: i32 = 124;
// java public static final int stiTableColumns5 = 125;
pub(crate) const stiTableColumns5: i32 = 125;
// java public static final int stiTableGrid1 = 126;
pub(crate) const stiTableGrid1: i32 = 126;
// java public static final int stiTableGrid2 = 127;
pub(crate) const stiTableGrid2: i32 = 127;
// java public static final int stiTableGrid3 = 128;
pub(crate) const stiTableGrid3: i32 = 128;
// java public static final int stiTableGrid4 = 129;
pub(crate) const stiTableGrid4: i32 = 129;
// java public static final int stiTableGrid5 = 130;
pub(crate) const stiTableGrid5: i32 = 130;
// java public static final int stiTableGrid6 = 131;
pub(crate) const stiTableGrid6: i32 = 131;
// java public static final int stiTableGrid7 = 132;
pub(crate) const stiTableGrid7: i32 = 132;
// java public static final int stiTableGrid8 = 133;
pub(crate) const stiTableGrid8: i32 = 133;
// java public static final int stiTableList1 = 134;
pub(crate) const stiTableList1: i32 = 134;
// java public static final int stiTableList2 = 135;
pub(crate) const stiTableList2: i32 = 135;
// java public static final int stiTableList3 = 136;
pub(crate) const stiTableList3: i32 = 136;
// java public static final int stiTableList4 = 137;
pub(crate) const stiTableList4: i32 = 137;
// java public static final int stiTableList5 = 138;
pub(crate) const stiTableList5: i32 = 138;
// java public static final int stiTableList6 = 139;
pub(crate) const stiTableList6: i32 = 139;
// java public static final int stiTableList7 = 140;
pub(crate) const stiTableList7: i32 = 140;
// java public static final int stiTableList8 = 141;
pub(crate) const stiTableList8: i32 = 141;
// java public static final int stiTable3DEffects1 = 142;
pub(crate) const stiTable3DEffects1: i32 = 142;
// java public static final int stiTable3DEffects2 = 143;
pub(crate) const stiTable3DEffects2: i32 = 143;
// java public static final int stiTable3DEffects3 = 144;
pub(crate) const stiTable3DEffects3: i32 = 144;
// java public static final int stiTableContemporary = 145;
pub(crate) const stiTableContemporary: i32 = 145;
// java public static final int stiTableElegant = 146;
pub(crate) const stiTableElegant: i32 = 146;
// java public static final int stiTableProfessional = 147;
pub(crate) const stiTableProfessional: i32 = 147;
// java public static final int stiTableSubtle1 = 148;
pub(crate) const stiTableSubtle1: i32 = 148;
// java public static final int stiTableSubtle2 = 149;
pub(crate) const stiTableSubtle2: i32 = 149;
// java public static final int stiTableWeb1 = 150;
pub(crate) const stiTableWeb1: i32 = 150;
// java public static final int stiTableWeb2 = 151;
pub(crate) const stiTableWeb2: i32 = 151;
// java public static final int stiTableWeb3 = 152;
pub(crate) const stiTableWeb3: i32 = 152;
// java public static final int stiBalloonText = 153;
pub(crate) const stiBalloonText: i32 = 153;
// java public static final int stiTableGrid = 154;
pub(crate) const stiTableGrid: i32 = 154;
// java public static final int stiTableTheme = 155;
pub(crate) const stiTableTheme: i32 = 155;
// java public static final int stiPlaceholderText = 156;
pub(crate) const stiPlaceholderText: i32 = 156;
// java public static final int stiNoSpacing = 157;
pub(crate) const stiNoSpacing: i32 = 157;
// java public static final int stiLightShading = 158;
pub(crate) const stiLightShading: i32 = 158;
// java public static final int stiLightList = 159;
pub(crate) const stiLightList: i32 = 159;
// java public static final int stiLightGrid = 160;
pub(crate) const stiLightGrid: i32 = 160;
// java public static final int stiMediumShading1 = 161;
pub(crate) const stiMediumShading1: i32 = 161;
// java public static final int stiMediumShading2 = 162;
pub(crate) const stiMediumShading2: i32 = 162;
// java public static final int stiMediumList1 = 163;
pub(crate) const stiMediumList1: i32 = 163;
// java public static final int stiMediumList2 = 164;
pub(crate) const stiMediumList2: i32 = 164;
// java public static final int stiMediumGrid1 = 165;
pub(crate) const stiMediumGrid1: i32 = 165;
// java public static final int stiMediumGrid2 = 166;
pub(crate) const stiMediumGrid2: i32 = 166;
// java public static final int stiMediumGrid3 = 167;
pub(crate) const stiMediumGrid3: i32 = 167;
// java public static final int stiDarkList = 168;
pub(crate) const stiDarkList: i32 = 168;
// java public static final int stiColorfulShading = 169;
pub(crate) const stiColorfulShading: i32 = 169;
// java public static final int stiColorfulList = 170;
pub(crate) const stiColorfulList: i32 = 170;
// java public static final int stiColorfulGrid = 171;
pub(crate) const stiColorfulGrid: i32 = 171;
// java public static final int stiLightShadingAccent1 = 172;
pub(crate) const stiLightShadingAccent1: i32 = 172;
// java public static final int stiLightListAccent1 = 173;
pub(crate) const stiLightListAccent1: i32 = 173;
// java public static final int stiLightGridAccent1 = 174;
pub(crate) const stiLightGridAccent1: i32 = 174;
// java public static final int stiMediumShading1Accent1 = 175;
pub(crate) const stiMediumShading1Accent1: i32 = 175;
// java public static final int stiMediumShading2Accent1 = 176;
pub(crate) const stiMediumShading2Accent1: i32 = 176;
// java public static final int stiMediumList1Accent1 = 177;
pub(crate) const stiMediumList1Accent1: i32 = 177;
// java public static final int stiRevision = 178;
pub(crate) const stiRevision: i32 = 178;
// java public static final int stiListParagraph = 179;
pub(crate) const stiListParagraph: i32 = 179;
// java public static final int stiQuote = 180;
pub(crate) const stiQuote: i32 = 180;
// java public static final int stiIntenseQuote = 181;
pub(crate) const stiIntenseQuote: i32 = 181;
// java public static final int stiMediumList2Accent1 = 182;
pub(crate) const stiMediumList2Accent1: i32 = 182;
// java public static final int stiMediumGrid1Accent1 = 183;
pub(crate) const stiMediumGrid1Accent1: i32 = 183;
// java public static final int stiMediumGrid2Accent1 = 184;
pub(crate) const stiMediumGrid2Accent1: i32 = 184;
// java public static final int stiMediumGrid3Accent1 = 185;
pub(crate) const stiMediumGrid3Accent1: i32 = 185;
// java public static final int stiDarkListAccent1 = 186;
pub(crate) const stiDarkListAccent1: i32 = 186;
// java public static final int stiColorfulShadingAccent1 = 187;
pub(crate) const stiColorfulShadingAccent1: i32 = 187;
// java public static final int stiColorfulListAccent1 = 188;
pub(crate) const stiColorfulListAccent1: i32 = 188;
// java public static final int stiColorfulGridAccent1 = 189;
pub(crate) const stiColorfulGridAccent1: i32 = 189;
// java public static final int stiLightShadingAccent2 = 190;
pub(crate) const stiLightShadingAccent2: i32 = 190;
// java public static final int stiLightListAccent2 = 191;
pub(crate) const stiLightListAccent2: i32 = 191;
// java public static final int stiLightGridAccent2 = 192;
pub(crate) const stiLightGridAccent2: i32 = 192;
// java public static final int stiMediumShading1Accent2 = 193;
pub(crate) const stiMediumShading1Accent2: i32 = 193;
// java public static final int stiMediumShading2Accent2 = 194;
pub(crate) const stiMediumShading2Accent2: i32 = 194;
// java public static final int stiMediumList1Accent2 = 195;
pub(crate) const stiMediumList1Accent2: i32 = 195;
// java public static final int stiMediumList2Accent2 = 196;
pub(crate) const stiMediumList2Accent2: i32 = 196;
// java public static final int stiMediumGrid1Accent2 = 197;
pub(crate) const stiMediumGrid1Accent2: i32 = 197;
// java public static final int stiMediumGrid2Accent2 = 198;
pub(crate) const stiMediumGrid2Accent2: i32 = 198;
// java public static final int stiMediumGrid3Accent2 = 199;
pub(crate) const stiMediumGrid3Accent2: i32 = 199;
// java public static final int stiDarkListAccent2 = 200;
pub(crate) const stiDarkListAccent2: i32 = 200;
// java public static final int stiColorfulShadingAccent2 = 201;
pub(crate) const stiColorfulShadingAccent2: i32 = 201;
// java public static final int stiColorfulListAccent2 = 202;
pub(crate) const stiColorfulListAccent2: i32 = 202;
// java public static final int stiColorfulGridAccent2 = 203;
pub(crate) const stiColorfulGridAccent2: i32 = 203;
// java public static final int stiLightShadingAccent3 = 204;
pub(crate) const stiLightShadingAccent3: i32 = 204;
// java public static final int stiLightListAccent3 = 205;
pub(crate) const stiLightListAccent3: i32 = 205;
// java public static final int stiLightGridAccent3 = 206;
pub(crate) const stiLightGridAccent3: i32 = 206;
// java public static final int stiMediumShading1Accent3 = 207;
pub(crate) const stiMediumShading1Accent3: i32 = 207;
// java public static final int stiMediumShading2Accent3 = 208;
pub(crate) const stiMediumShading2Accent3: i32 = 208;
// java public static final int stiMediumList1Accent3 = 209;
pub(crate) const stiMediumList1Accent3: i32 = 209;
// java public static final int stiMediumList2Accent3 = 210;
pub(crate) const stiMediumList2Accent3: i32 = 210;
// java public static final int stiMediumGrid1Accent3 = 211;
pub(crate) const stiMediumGrid1Accent3: i32 = 211;
// java public static final int stiMediumGrid2Accent3 = 212;
pub(crate) const stiMediumGrid2Accent3: i32 = 212;
// java public static final int stiMediumGrid3Accent3 = 213;
pub(crate) const stiMediumGrid3Accent3: i32 = 213;
// java public static final int stiDarkListAccent3 = 214;
pub(crate) const stiDarkListAccent3: i32 = 214;
// java public static final int stiColorfulShadingAccent3 = 215;
pub(crate) const stiColorfulShadingAccent3: i32 = 215;
// java public static final int stiColorfulListAccent3 = 216;
pub(crate) const stiColorfulListAccent3: i32 = 216;
// java public static final int stiColorfulGridAccent3 = 217;
pub(crate) const stiColorfulGridAccent3: i32 = 217;
// java public static final int stiLightShadingAccent4 = 218;
pub(crate) const stiLightShadingAccent4: i32 = 218;
// java public static final int stiLightListAccent4 = 219;
pub(crate) const stiLightListAccent4: i32 = 219;
// java public static final int stiLightGridAccent4 = 220;
pub(crate) const stiLightGridAccent4: i32 = 220;
// java public static final int stiMediumShading1Accent4 = 221;
pub(crate) const stiMediumShading1Accent4: i32 = 221;
// java public static final int stiMediumShading2Accent4 = 222;
pub(crate) const stiMediumShading2Accent4: i32 = 222;
// java public static final int stiMediumList1Accent4 = 223;
pub(crate) const stiMediumList1Accent4: i32 = 223;
// java public static final int stiMediumList2Accent4 = 224;
pub(crate) const stiMediumList2Accent4: i32 = 224;
// java public static final int stiMediumGrid1Accent4 = 225;
pub(crate) const stiMediumGrid1Accent4: i32 = 225;
// java public static final int stiMediumGrid2Accent4 = 226;
pub(crate) const stiMediumGrid2Accent4: i32 = 226;
// java public static final int stiMediumGrid3Accent4 = 227;
pub(crate) const stiMediumGrid3Accent4: i32 = 227;
// java public static final int stiDarkListAccent4 = 228;
pub(crate) const stiDarkListAccent4: i32 = 228;
// java public static final int stiColorfulShadingAccent4 = 229;
pub(crate) const stiColorfulShadingAccent4: i32 = 229;
// java public static final int stiColorfulListAccent4 = 230;
pub(crate) const stiColorfulListAccent4: i32 = 230;
// java public static final int stiColorfulGridAccent4 = 231;
pub(crate) const stiColorfulGridAccent4: i32 = 231;
// java public static final int stiLightShadingAccent5 = 232;
pub(crate) const stiLightShadingAccent5: i32 = 232;
// java public static final int stiLightListAccent5 = 233;
pub(crate) const stiLightListAccent5: i32 = 233;
// java public static final int stiLightGridAccent5 = 234;
pub(crate) const stiLightGridAccent5: i32 = 234;
// java public static final int stiMediumShading1Accent5 = 235;
pub(crate) const stiMediumShading1Accent5: i32 = 235;
// java public static final int stiMediumShading2Accent5 = 236;
pub(crate) const stiMediumShading2Accent5: i32 = 236;
// java public static final int stiMediumList1Accent5 = 237;
pub(crate) const stiMediumList1Accent5: i32 = 237;
// java public static final int stiMediumList2Accent5 = 238;
pub(crate) const stiMediumList2Accent5: i32 = 238;
// java public static final int stiMediumGrid1Accent5 = 239;
pub(crate) const stiMediumGrid1Accent5: i32 = 239;
// java public static final int stiMediumGrid2Accent5 = 240;
pub(crate) const stiMediumGrid2Accent5: i32 = 240;
// java public static final int stiMediumGrid3Accent5 = 241;
pub(crate) const stiMediumGrid3Accent5: i32 = 241;
// java public static final int stiDarkListAccent5 = 242;
pub(crate) const stiDarkListAccent5: i32 = 242;
// java public static final int stiColorfulShadingAccent5 = 243;
pub(crate) const stiColorfulShadingAccent5: i32 = 243;
// java public static final int stiColorfulListAccent5 = 244;
pub(crate) const stiColorfulListAccent5: i32 = 244;
// java public static final int stiColorfulGridAccent5 = 245;
pub(crate) const stiColorfulGridAccent5: i32 = 245;
// java public static final int stiLightShadingAccent6 = 246;
pub(crate) const stiLightShadingAccent6: i32 = 246;
// java public static final int stiLightListAccent6 = 247;
pub(crate) const stiLightListAccent6: i32 = 247;
// java public static final int stiLightGridAccent6 = 248;
pub(crate) const stiLightGridAccent6: i32 = 248;
// java public static final int stiMediumShading1Accent6 = 249;
pub(crate) const stiMediumShading1Accent6: i32 = 249;
// java public static final int stiMediumShading2Accent6 = 250;
pub(crate) const stiMediumShading2Accent6: i32 = 250;
// java public static final int stiMediumList1Accent6 = 251;
pub(crate) const stiMediumList1Accent6: i32 = 251;
// java public static final int stiMediumList2Accent6 = 252;
pub(crate) const stiMediumList2Accent6: i32 = 252;
// java public static final int stiMediumGrid1Accent6 = 253;
pub(crate) const stiMediumGrid1Accent6: i32 = 253;
// java public static final int stiMediumGrid2Accent6 = 254;
pub(crate) const stiMediumGrid2Accent6: i32 = 254;
// java public static final int stiMediumGrid3Accent6 = 255;
pub(crate) const stiMediumGrid3Accent6: i32 = 255;
// java public static final int stiDarkListAccent6 = 256;
pub(crate) const stiDarkListAccent6: i32 = 256;
// java public static final int stiColorfulShadingAccent6 = 257;
pub(crate) const stiColorfulShadingAccent6: i32 = 257;
// java public static final int stiColorfulListAccent6 = 258;
pub(crate) const stiColorfulListAccent6: i32 = 258;
// java public static final int stiColorfulGridAccent6 = 259;
pub(crate) const stiColorfulGridAccent6: i32 = 259;
// java public static final int stiSubtleEmphasis = 260;
pub(crate) const stiSubtleEmphasis: i32 = 260;
// java public static final int stiIntenseEmphasis = 261;
pub(crate) const stiIntenseEmphasis: i32 = 261;
// java public static final int stiSubtleReference = 262;
pub(crate) const stiSubtleReference: i32 = 262;
// java public static final int stiIntenseReference = 263;
pub(crate) const stiIntenseReference: i32 = 263;
// java public static final int stiBookTitle = 264;
pub(crate) const stiBookTitle: i32 = 264;
// java public static final int stiBibliography = 265;
pub(crate) const stiBibliography: i32 = 265;
// java public static final int stiTOCHeading = 266;
pub(crate) const stiTOCHeading: i32 = 266;
// java public static final int stiBuildinStiEnd = 267;
pub(crate) const stiBuildinStiEnd: i32 = 267;
// java public static final int stiUser = 4094;
pub(crate) const stiUser: i32 = 4094;
// java public static final int stiNil = 4095;
pub(crate) const stiNil: i32 = 4095;
// java public static final int stiCustom = 4096;
pub(crate) const stiCustom: i32 = 4096;
// java public static final int stiLinkedStyle_char_begin = 12288;
pub(crate) const stiLinkedStyle_char_begin: i32 = 12288;
// java public static final int stiHeading1Char = 12289;
pub(crate) const stiHeading1Char: i32 = 12289;
// java public static final int stiHeading2Char = 12290;
pub(crate) const stiHeading2Char: i32 = 12290;
// java public static final int stiHeading3Char = 12291;
pub(crate) const stiHeading3Char: i32 = 12291;
// java public static final int stiHeading4Char = 12292;
pub(crate) const stiHeading4Char: i32 = 12292;
// java public static final int stiHeading5Char = 12293;
pub(crate) const stiHeading5Char: i32 = 12293;
// java public static final int stiHeading6Char = 12294;
pub(crate) const stiHeading6Char: i32 = 12294;
// java public static final int stiHeading7Char = 12295;
pub(crate) const stiHeading7Char: i32 = 12295;
// java public static final int stiHeading8Char = 12296;
pub(crate) const stiHeading8Char: i32 = 12296;
// java public static final int stiHeading9Char = 12297;
pub(crate) const stiHeading9Char: i32 = 12297;
// java public static final int stiTitleChar = 12350;
pub(crate) const stiTitleChar: i32 = 12350;
// java public static final int stiSubtitleChar = 12362;
pub(crate) const stiSubtitleChar: i32 = 12362;
// java public static final int stiQuoteChar = 12468;
pub(crate) const stiQuoteChar: i32 = 12468;
// java public static final int stiIntenseQuoteChar = 12469;
pub(crate) const stiIntenseQuoteChar: i32 = 12469;
// java public static final int stiHeaderChar = 12319;
pub(crate) const stiHeaderChar: i32 = 12319;
// java public static final int stiFooterChar = 12320;
pub(crate) const stiFooterChar: i32 = 12320;

// java public class KStyle extends Atom
pub struct KStyle {
    // java Atom super_
    pub(crate) super_: Atom,
    // java private TextDocument mTextDocument;
    mTextDocument: Option<TextDocument>,
    // java @AtomMember int id;
    pub(crate) id: i32,
    // java @AtomMember String name;
    pub(crate) name: Option<String>,
    // java String mDisplayName;
    pub(crate) mDisplayName: Option<String>,
    // java @AtomMember int base;
    pub(crate) base: i32,
    // java @AtomMember int next;
    pub(crate) next: i32,
    // java @AtomMember int link;
    pub(crate) link: i32,
    // java @AtomMember int type;
    pub(crate) r#type: i32,
    // java @AtomMember boolean qFormat;
    pub(crate) qFormat: bool,
    // java @AtomMember long uiPriority;
    pub(crate) uiPriority: i64,
    // java KStyles styleSheet;
    pub(crate) styleSheet: Option<*mut KStyles>,
    // java @AtomMember PropertySet property;
    pub(crate) property: Option<PropertySet>,
    // java PropertySet finalProperty;
    pub(crate) finalProperty: Option<PropertySet>,
    // java private WIntArrayList mCachePath;
    mCachePath: Option<WIntArrayList>,
}

/// Soft stand-in for core.KStyles (minimal API used by KStyle before KStyles leaf)
#[derive(Default, Clone)]
pub struct SoftKStylesRef;
impl SoftKStylesRef {
    pub fn getBuiltinStyles() -> SoftStyleMap {
        SoftStyleMap
    }
    pub fn getStyleById__int(&self, _id: i32) -> Option<KStyle> {
        None
    }
}
#[derive(Default, Clone)]
pub struct SoftStyleMap;
impl SoftStyleMap {
    pub fn get(&self, _id: i32) -> Option<KStyle> {
        None
    }
}

// java public class KStyle extends Atom
impl KStyle {
    // java public  KStyle()
    pub fn KStyle() -> Self {
      // java this.styleSheet = null;
      // java this.property = PropertySet.empty;
      // java this.finalProperty = null;
      // java this.mCachePath = new WIntArrayList();
      Self {
        super_: Atom::new(),
        mTextDocument: None,
        id: 0,
        name: None,
        mDisplayName: None,
        base: 0,
        next: 0,
        link: 0,
        r#type: 0,
        qFormat: false,
        uiPriority: 0,
        styleSheet: None,
        property: Some(property_set_empty()),
        finalProperty: None,
        mCachePath: Some(WIntArrayList::new()),
      }
    }

    pub fn new() -> Self {
      Self::KStyle()
    }

    // java public  KStyle(TextDocument textDocument, int id, String name, int base, int next, int type, PropertySet property)
    pub fn KStyle__textdocument_int_string_int_int_int_propertyset(textDocument: &TextDocument, id: i32, name: &str, base: i32, next: i32, r#type: i32, property: &PropertySet) -> Self {
      // java this(id, name, base, next, type, property);
      let mut this = Self::new__int_string_int_int_int_propertyset(id, name, base, next, r#type, Some(property));
      // java this.setTextDocument(textDocument);
      this.setTextDocument(textDocument);
      this
    }

    pub fn new__textdocument_int_string_int_int_int_propertyset(textDocument: &TextDocument, id: i32, name: &str, base: i32, next: i32, r#type: i32, property: &PropertySet) -> Self {
      Self::KStyle__textdocument_int_string_int_int_int_propertyset(textDocument, id, name, base, next, r#type, property)
    }

    // java   KStyle(int id, String name, int base, int next, int type, PropertySet property)
    pub fn new__int_string_int_int_int_propertyset(id: i32, name: &str, base: i32, next: i32, r#type: i32, property: Option<&PropertySet>) -> Self {
      // java this.styleSheet = null;
      // java this.property = PropertySet.empty;
      // java this.finalProperty = null;
      // java this.mCachePath = new WIntArrayList();
      // java this.id = id;
      // java this.name = name;
      // java this.base = base;
      // java this.next = next;
      // java this.type = type;
      // java this.property = property != null ? property : PropertySet.empty;
      // java this.link = 4095;
      // java this.qFormat = false;
      // java this.uiPriority = -1L;
      Self {
        super_: Atom::new(),
        mTextDocument: None,
        id,
        name: Some(name.to_string()),
        mDisplayName: None,
        base,
        next,
        link: 4095,
        r#type,
        qFormat: false,
        uiPriority: -1,
        styleSheet: None,
        property: Some(match property {
          Some(p) => p.clone(),
          None => property_set_empty(),
        }),
        finalProperty: None,
        mCachePath: Some(WIntArrayList::new()),
      }
    }

    // java public Object clone()
    pub fn clone(&self) -> KStyle {
      // java KStyle pStyle = new KStyle(this.id, this.name, this.base, this.next, this.type, this.property);
      let mut pStyle: KStyle = KStyle::new__int_string_int_int_int_propertyset(
        self.id,
        self.name.as_deref().unwrap_or(""),
        self.base,
        self.next,
        self.r#type,
        self.property.as_ref(),
      );
      // java pStyle.mDisplayName = this.mDisplayName;
      pStyle.mDisplayName = self.mDisplayName.clone();
      // java pStyle.link = this.link;
      pStyle.link = self.link;
      // java pStyle.qFormat = this.qFormat;
      pStyle.qFormat = self.qFormat;
      // java pStyle.uiPriority = this.uiPriority;
      pStyle.uiPriority = self.uiPriority;
      // java pStyle.styleSheet = null;
      pStyle.styleSheet = None;
      // java pStyle.finalProperty = null;
      pStyle.finalProperty = None;
      // java return pStyle;
      return pStyle;
    }

    /// Ownership glue: clone style for HashMap/Vec (Java shared refs).
    pub fn clone_style(&self) -> KStyle {
      self.clone()
    }

    // java public boolean equals(Object r)
    pub fn equals(&self, r: Option<&dyn Object>) -> bool {
      // java if (null != r && r instanceof KStyle)
      if let Some(r) = r {
        if r.as_any().downcast_ref::<KStyle>().is_some() {
          // java if (r == this)
          if std::ptr::eq(
            r.as_any() as *const dyn std::any::Any,
            self as *const dyn std::any::Any,
          ) {
            // java return true;
            return true;
          }
          // java else
          else {
            // java KStyle p = (KStyle)r;
            let p: &KStyle = r.as_any().downcast_ref::<KStyle>().unwrap();
            // java return this.id == p.id && is_string_equal(this.name, p.name) && this.base == p.base && this.next == p.next && this.type == p.type && PropertySet.isPropertyEqual(this.property, p.property);
            return self.id == p.id
              && Self::is_string_equal(self.name.as_deref(), p.name.as_deref())
              && self.base == p.base
              && self.next == p.next
              && self.r#type == p.r#type
              && PropertySet::isPropertyEqual(self.property.as_ref(), p.property.as_ref());
          }
        }
      }
      // java else
      // java return false;
      return false;
    }
    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return this.id + this.name.hashCode() + this.type + this.property.hashCode();
      let name_hash = self.name.as_ref().map(|s| {
        let mut h: i32 = 0;
        for b in s.bytes() { h = h.wrapping_mul(31).wrapping_add(b as i32); }
        h
      }).unwrap_or(0);
      let prop_hash = self.property.as_ref().map(|p| {
        let mut p = p.clone();
        p.hashCode()
      }).unwrap_or(0);
      return self.id + name_hash + self.r#type + prop_hash;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.name = null;
      self.name = None;
      // java this.mDisplayName = null;
      self.mDisplayName = None;
      // java this.styleSheet = null;
      self.styleSheet = None;
      // java this.property = null;
      self.property = None;
      // java if (this.finalProperty != null)
      if let Some(mut finalProperty) = self.finalProperty.take() {
        // java this.finalProperty.dispose();
        finalProperty.dispose();
        // java this.finalProperty = null;
      }
    }

    // java static boolean is_string_equal(String a, String b)
    pub fn is_string_equal(a: Option<&str>, b: Option<&str>) -> bool {
      // java if (a == b)
      if a == b {
        // java return true;
        return true;
      }
      // java else
      else {
        // java if (a == null)
        let a = if a.is_none() {
          // java a = "";
          ""
        } else {
          a.unwrap()
        };
        // java if (b == null)
        let b = if b.is_none() {
          // java b = "";
          ""
        } else {
          b.unwrap()
        };
        // java return a.equals(b);
        return a == b;
      }
    }

    // java public int getId()
    pub fn getId(&self) -> i32 {
      // java return this.id;
      return self.id;
    }

    // java public void setId(int newID)
    pub fn setId(&mut self, newID: i32) {
      // java this.id = newID;
      self.id = newID;
    }

    // java public void setTextDocument(TextDocument textDocument)
    pub fn setTextDocument(&mut self, textDocument: &TextDocument) {
      // java Assert.assertNotNull("textDocument should not be null.", textDocument);
      let td_opt = Some(textDocument);
      Assert::assertNotNull_msg("textDocument should not be null.", &td_opt);
      // java this.mTextDocument = textDocument;
      self.mTextDocument = Some(textDocument.clone());
      let _ = textDocument;
      // java Transaction transaction = textDocument.getTransaction();
      let transaction = textDocument.getTransaction();
      // java Assert.assertNotNull("transaction should not be null.", transaction);
      Assert::assertNotNull_msg("transaction should not be null.", &transaction);
      // java this.setTransaction(transaction, false);
      self.super_.setTransaction_recursive(transaction, false);
    }

    // java public String getDisplayName()
    pub fn getDisplayName(&self) -> Option<String> {
      // java KStyle pBuiltin = (KStyle)KStyles.getBuiltinStyles().get(this.id);
      let pBuiltin: Option<KStyle> = SoftKStylesRef::getBuiltinStyles().get(self.id);
      // java return pBuiltin != null && pBuiltin.mDisplayName != null ? pBuiltin.mDisplayName : this.name;
      return (if pBuiltin.as_ref().map(|p| p.mDisplayName.is_some()).unwrap_or(false) {
        pBuiltin.unwrap().mDisplayName
      } else {
        self.name.clone()
      });
    }

    // java public void setDisplayName(String displayName)
    pub fn setDisplayName(&mut self, displayName: &str) {
      // java this.mDisplayName = displayName;
      self.mDisplayName = Some(displayName.to_string());
    }

    // java public String getName()
    pub fn getName(&self) -> String {
      // java return this.name;
      return self.name.clone().unwrap_or_default();
    }

    // java public void setName(String str)
    pub fn setName(&mut self, str: &str) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.name = str;
      self.name = Some(str.to_string());
    }
    // java public int getBase()
    pub fn getBase(&self) -> i32 {
      // java return this.base;
      return self.base;
    }

    // java public void setBase(int str)
    pub fn setBase(&mut self, str: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.base = str;
      self.base = str;
    }

    // java public int getNext()
    pub fn getNext(&self) -> i32 {
      // java return this.next;
      return self.next;
    }

    // java public void setNext(int str)
    pub fn setNext(&mut self, str: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.next = str;
      self.next = str;
    }

    // java public int getLink()
    pub fn getLink(&self) -> i32 {
      // java return this.link;
      return self.link;
    }

    // java public void setLink(int iLink)
    pub fn setLink(&mut self, iLink: i32) {
      // java this.link = iLink;
      self.link = iLink;
    }

    // java public int getType()
    pub fn getType(&self) -> i32 {
      // java return this.type;
      return self.r#type;
    }

    // java public void setType(int n)
    pub fn setType(&mut self, n: i32) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.type = n;
      self.r#type = n;
    }

    // java public boolean getQFormat()
    pub fn getQFormat(&self) -> bool {
      // java return this.qFormat;
      return self.qFormat;
    }

    // java public void setQFormat(boolean b)
    pub fn setQFormat(&mut self, b: bool) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.qFormat = b;
      self.qFormat = b;
    }

    // java public long getUiPriority()
    pub fn getUiPriority(&self) -> i64 {
      // java return this.uiPriority;
      return self.uiPriority;
    }

    // java public void setUiPriority(long val)
    pub fn setUiPriority(&mut self, val: i64) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.uiPriority = val;
      self.uiPriority = val;
    }

    // java public boolean isDefault()
    pub fn isDefault(&self) -> bool {
      // java return this.id == 0 || this.id == 65 || this.id == 105 || this.id == 107;
      return self.id == 0 || self.id == 65 || self.id == 105 || self.id == 107;
    }

    // java public boolean isUserCustom()
    pub fn isUserCustom(&self) -> bool {
      // java return this.id >= 4096;
      return self.id >= 4096;
    }

    // java public PropertySet getProperty()
    pub fn getProperty(&self) -> Option<PropertySet> {
      // java return this.property;
      return self.property.clone();
    }

    // java public void setProperty(PropertySet prop)
    pub fn setProperty(&mut self, prop: &PropertySet) {
      // java this.writeLockAtom();
      self.super_.writeLockAtom();
      // java this.finalProperty = null;
      self.finalProperty = None;
      // java this.property = prop;
      self.property = Some(prop.clone());
    }

    // java public PropertySet getFinalProperty()
    pub fn getFinalProperty(&mut self) -> Option<PropertySet> {
      // java this.mCachePath.resetQuick();
      if let Some(cache) = self.mCachePath.as_mut() {
        cache.resetQuick();
      }
      // java return this.getFinalProperty(this.mCachePath);
      let mut path = self.mCachePath.take().unwrap_or_else(WIntArrayList::new);
      let result = self.getFinalProperty__wintarraylist(&mut path);
      self.mCachePath = Some(path);
      result
    }

    // java private PropertySet getFinalProperty(WIntArrayList path)
    pub fn getFinalProperty__wintarraylist(&mut self, path: &mut WIntArrayList) -> Option<PropertySet> {
      // java if (this.finalProperty != null)
      if let Some(finalProperty) = self.finalProperty.clone() {
        // java return this.finalProperty;
        return Some(finalProperty);
      }
      // java else if (this.styleSheet == null)
      else if self.styleSheet.is_none() {
        // java return this.property;
        return self.property.clone();
      }
      // java else
      else {
        // java path.add(this.id);
        path.add(self.id);
        // java MutablePropertySet pFinal = new MutablePropertySet();
        let mut pFinal: MutablePropertySet = MutablePropertySet::new();
        // java if (this.base != 4095 && !path.contains(this.base))
        if self.base != 4095 && !path.contains(self.base) {
          // java KStyle baseStyle = this.styleSheet.getStyleById(this.base);
          let mut baseStyle: Option<KStyle> = self.styleSheet.and_then(|ss| {
            if ss.is_null() {
              None
            } else {
              unsafe { (*ss).getStyleByIdForInheritance(self.base) }
            }
          });
          // java if (baseStyle != null)
          if let Some(mut baseStyle) = baseStyle {
            // java pFinal.add(baseStyle.getFinalProperty(path));
            if let Some(fp) = baseStyle.getFinalProperty__wintarraylist(path) {
              pFinal.add(&fp);
            }
          }
        }
        // java this.justifyParagraphMargin(spaceBeforeTypes, pFinal, this.property);
        if let Some(prop) = self.property.clone() {
          self.justifyParagraphMargin(&spaceBeforeTypes, &mut pFinal, &prop);
          // java this.justifyParagraphMargin(spaceAfterTypes, pFinal, this.property);
          self.justifyParagraphMargin(&spaceAfterTypes, &mut pFinal, &prop);
          // java Toggle runToggle = (Toggle)pFinal.getObject(57, DefaultValue.run_bidi);
          let runToggle: Option<Toggle> = pFinal
            .getValue(57)
            .and_then(|v| v.as_ref().as_any().downcast_ref::<Toggle>().copied())
            .or_else(|| DefaultValue::run_bidi());
          // java pFinal.add(this.property);
          pFinal.add(&prop);
          // java this.justifyToggle(pFinal, this.property, 57, runToggle);
          self.justifyToggle(&mut pFinal, &prop, 57, runToggle);
        }
        // java this.finalProperty = pFinal.freeze();
        self.finalProperty = Some(pFinal.freeze());
        // java return this.finalProperty;
        return self.finalProperty.clone();
      }
    }

    // java private void justifyParagraphMargin(int[] types, MutablePropertySet newProperty, PropertySet baseProperty)
    pub fn justifyParagraphMargin(&mut self, types: &Vec<i32>, newProperty: &mut MutablePropertySet, baseProperty: &PropertySet) {
      // java if (baseProperty.contains(types[0]))
      if baseProperty.contains(types[0usize]) {
        // java newProperty.remove(types[1]);
        newProperty.remove(types[1usize]);
        // java newProperty.remove(types[2]);
        newProperty.remove(types[2usize]);
      }
      // java else if (baseProperty.contains(types[1]))
      else if baseProperty.contains(types[1usize]) {
        // java newProperty.remove(types[2]);
        newProperty.remove(types[2usize]);
      }
    }

    // java private void justifyToggle(MutablePropertySet pFinal, PropertySet property, int id, Toggle orgToggle)
    pub fn justifyToggle(&mut self, pFinal: &mut MutablePropertySet, property: &PropertySet, id: i32, orgToggle: Option<Toggle>) {
      // java if (property.contains(id))
      if property.contains(id) {
        // java Toggle des = Toggle.OFF;
        let mut des: Toggle = Toggle::OFF;
        // java switch ((Toggle)property.getValue(id))
        let toggle_val = property.getValue(id).and_then(|v| v.as_ref().as_any().downcast_ref::<Toggle>().copied());
        match toggle_val {
          // java case OFF:
          Some(Toggle::OFF) => {
            // java des = Toggle.OFF;
            des = Toggle::OFF;
          }
          // java case ON:
          Some(Toggle::ON) => {
            // java des = Toggle.ON;
            des = Toggle::ON;
          }
          // java case CURRENT:
          Some(Toggle::CURRENT) => {
            // java des = orgToggle;
            des = orgToggle.unwrap_or(Toggle::OFF);
          }
          // java case OPPOSITE:
          Some(Toggle::OPPOSITE) => {
            // java if (null == orgToggle)
            if orgToggle.is_none() {
              // java des = Toggle.ON;
              des = Toggle::ON;
            }
            // java else if (Toggle.OFF == orgToggle)
            else if orgToggle == Some(Toggle::OFF) {
              // java des = Toggle.ON;
              des = Toggle::ON;
            }
            // java else
            else {
              // java des = Toggle.OFF;
              des = Toggle::OFF;
            }
          }
          // java default:
          _ => {
          }
        }
        // java pFinal.setValue(id, des);
        pFinal.setValue(id, Some(Arc::new(des)));
      }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder sb = new StringBuilder();
      let mut sb = String::new();
      // java sb.append(this.getClass().getSimpleName());
      sb.push_str("KStyle");
      // java sb.append(": {\n");
      sb.push_str(": {\n");
      // java sb.append("name = " + this.getName() + "\n");
      sb.push_str(&format!("name = {}\n", self.getName()));
      // java sb.append("display name = " + this.getDisplayName() + "\n");
      sb.push_str(&format!("display name = {:?}\n", self.getDisplayName()));
      // java sb.append("id = " + this.getId() + "\n");
      sb.push_str(&format!("id = {}\n", self.getId()));
      // java if (this.getProperty() == null)
      if self.getProperty().is_none() {
        // java sb.append("property = null\n");
        sb.push_str("property = null\n");
      }
      // java else
      else {
        // java sb.append("property = " + this.getProperty().toString() + "\n");
        sb.push_str(&format!("property = {}\n", self.getProperty().unwrap().toString()));
      }
      // java sb.append("}\n");
      sb.push_str("}\n");
      // java return sb.toString();
      return sb;
    }

    // java public void onSwapped(Object[] prev, Object arg)
    pub fn onSwapped(&mut self, prev: &dyn Object, arg: &dyn Object) {
      let _ = (prev, arg);
      // java this.finalProperty = null;
      self.finalProperty = None;
      // java PropertyTool.clearCache();
      PropertyTool::clearCache();
      // java this.mTextDocument.updateDocument();
      if let Some(doc) = self.mTextDocument.as_mut() {
        doc.updateDocument();
      }
    }

    // java public static boolean isHeadStyle(int styleId)
    pub fn isHeadStyle(styleId: i32) -> bool {
      // java if (styleId >= 1 && styleId <= 9)
      if styleId >= 1 && styleId <= 9 {
        // java return true;
        return true;
      }
      // java else
      else {
        // java return styleId == 62 || styleId == 74;
        return styleId == 62 || styleId == 74;
      }
    }

}

impl std::ops::Deref for KStyle {
    type Target = Atom;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
