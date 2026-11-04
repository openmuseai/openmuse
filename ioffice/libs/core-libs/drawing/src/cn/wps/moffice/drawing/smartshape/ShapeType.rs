use std::collections::HashMap;
use std::sync::OnceLock;
use crate::cn::wps::moffice::drawing::Shape::Shape;

// Java: public class ShapeType
pub struct ShapeType;

impl ShapeType {
    // Java field: int msosptMin
    pub const msosptMin: i32 = 0;
    // Java field: int msosptNotPrimitive
    pub const msosptNotPrimitive: i32 = 0;
    // Java field: int msosptRectangle
    pub const msosptRectangle: i32 = 1;
    // Java field: int msosptRoundRectangle
    pub const msosptRoundRectangle: i32 = 2;
    // Java field: int msosptEllipse
    pub const msosptEllipse: i32 = 3;
    // Java field: int msosptDiamond
    pub const msosptDiamond: i32 = 4;
    // Java field: int msosptIsocelesTriangle
    pub const msosptIsocelesTriangle: i32 = 5;
    // Java field: int msosptRightTriangle
    pub const msosptRightTriangle: i32 = 6;
    // Java field: int msosptParallelogram
    pub const msosptParallelogram: i32 = 7;
    // Java field: int msosptTrapezoid
    pub const msosptTrapezoid: i32 = 8;
    // Java field: int msosptHexagon
    pub const msosptHexagon: i32 = 9;
    // Java field: int msosptOctagon
    pub const msosptOctagon: i32 = 10;
    // Java field: int msosptPlus
    pub const msosptPlus: i32 = 11;
    // Java field: int msosptStar
    pub const msosptStar: i32 = 12;
    // Java field: int msosptArrow
    pub const msosptArrow: i32 = 13;
    // Java field: int msosptThickArrow
    pub const msosptThickArrow: i32 = 14;
    // Java field: int msosptHomePlate
    pub const msosptHomePlate: i32 = 15;
    // Java field: int msosptCube
    pub const msosptCube: i32 = 16;
    // Java field: int msosptBalloon
    pub const msosptBalloon: i32 = 17;
    // Java field: int msosptSeal
    pub const msosptSeal: i32 = 18;
    // Java field: int msosptArc
    pub const msosptArc: i32 = 19;
    // Java field: int msosptLine
    pub const msosptLine: i32 = 20;
    // Java field: int msosptPlaque
    pub const msosptPlaque: i32 = 21;
    // Java field: int msosptCan
    pub const msosptCan: i32 = 22;
    // Java field: int msosptDonut
    pub const msosptDonut: i32 = 23;
    // Java field: int msosptTextSimple
    pub const msosptTextSimple: i32 = 24;
    // Java field: int msosptTextOctagon
    pub const msosptTextOctagon: i32 = 25;
    // Java field: int msosptTextHexagon
    pub const msosptTextHexagon: i32 = 26;
    // Java field: int msosptTextCurve
    pub const msosptTextCurve: i32 = 27;
    // Java field: int msosptTextWave
    pub const msosptTextWave: i32 = 28;
    // Java field: int msosptTextRing
    pub const msosptTextRing: i32 = 29;
    // Java field: int msosptTextOnCurve
    pub const msosptTextOnCurve: i32 = 30;
    // Java field: int msosptTextOnRing
    pub const msosptTextOnRing: i32 = 31;
    // Java field: int msosptStraightConnector1
    pub const msosptStraightConnector1: i32 = 32;
    // Java field: int msosptBentConnector2
    pub const msosptBentConnector2: i32 = 33;
    // Java field: int msosptBentConnector3
    pub const msosptBentConnector3: i32 = 34;
    // Java field: int msosptBentConnector4
    pub const msosptBentConnector4: i32 = 35;
    // Java field: int msosptBentConnector5
    pub const msosptBentConnector5: i32 = 36;
    // Java field: int msosptCurvedConnector2
    pub const msosptCurvedConnector2: i32 = 37;
    // Java field: int msosptCurvedConnector3
    pub const msosptCurvedConnector3: i32 = 38;
    // Java field: int msosptCurvedConnector4
    pub const msosptCurvedConnector4: i32 = 39;
    // Java field: int msosptCurvedConnector5
    pub const msosptCurvedConnector5: i32 = 40;
    // Java field: int msosptCallout1
    pub const msosptCallout1: i32 = 41;
    // Java field: int msosptCallout2
    pub const msosptCallout2: i32 = 42;
    // Java field: int msosptCallout3
    pub const msosptCallout3: i32 = 43;
    // Java field: int msosptAccentCallout1
    pub const msosptAccentCallout1: i32 = 44;
    // Java field: int msosptAccentCallout2
    pub const msosptAccentCallout2: i32 = 45;
    // Java field: int msosptAccentCallout3
    pub const msosptAccentCallout3: i32 = 46;
    // Java field: int msosptBorderCallout1
    pub const msosptBorderCallout1: i32 = 47;
    // Java field: int msosptBorderCallout2
    pub const msosptBorderCallout2: i32 = 48;
    // Java field: int msosptBorderCallout3
    pub const msosptBorderCallout3: i32 = 49;
    // Java field: int msosptAccentBorderCallout1
    pub const msosptAccentBorderCallout1: i32 = 50;
    // Java field: int msosptAccentBorderCallout2
    pub const msosptAccentBorderCallout2: i32 = 51;
    // Java field: int msosptAccentBorderCallout3
    pub const msosptAccentBorderCallout3: i32 = 52;
    // Java field: int msosptRibbon
    pub const msosptRibbon: i32 = 53;
    // Java field: int msosptRibbon2
    pub const msosptRibbon2: i32 = 54;
    // Java field: int msosptChevron
    pub const msosptChevron: i32 = 55;
    // Java field: int msosptPentagon
    pub const msosptPentagon: i32 = 56;
    // Java field: int msosptNoSmoking
    pub const msosptNoSmoking: i32 = 57;
    // Java field: int msosptSeal8
    pub const msosptSeal8: i32 = 58;
    // Java field: int msosptSeal16
    pub const msosptSeal16: i32 = 59;
    // Java field: int msosptSeal32
    pub const msosptSeal32: i32 = 60;
    // Java field: int msosptWedgeRectCallout
    pub const msosptWedgeRectCallout: i32 = 61;
    // Java field: int msosptWedgeRRectCallout
    pub const msosptWedgeRRectCallout: i32 = 62;
    // Java field: int msosptWedgeEllipseCallout
    pub const msosptWedgeEllipseCallout: i32 = 63;
    // Java field: int msosptWave
    pub const msosptWave: i32 = 64;
    // Java field: int msosptFoldedCorner
    pub const msosptFoldedCorner: i32 = 65;
    // Java field: int msosptLeftArrow
    pub const msosptLeftArrow: i32 = 66;
    // Java field: int msosptDownArrow
    pub const msosptDownArrow: i32 = 67;
    // Java field: int msosptUpArrow
    pub const msosptUpArrow: i32 = 68;
    // Java field: int msosptLeftRightArrow
    pub const msosptLeftRightArrow: i32 = 69;
    // Java field: int msosptUpDownArrow
    pub const msosptUpDownArrow: i32 = 70;
    // Java field: int msosptIrregularSeal1
    pub const msosptIrregularSeal1: i32 = 71;
    // Java field: int msosptIrregularSeal2
    pub const msosptIrregularSeal2: i32 = 72;
    // Java field: int msosptLightningBolt
    pub const msosptLightningBolt: i32 = 73;
    // Java field: int msosptHeart
    pub const msosptHeart: i32 = 74;
    // Java field: int msosptPictureFrame
    pub const msosptPictureFrame: i32 = 75;
    // Java field: int msosptQuadArrow
    pub const msosptQuadArrow: i32 = 76;
    // Java field: int msosptLeftArrowCallout
    pub const msosptLeftArrowCallout: i32 = 77;
    // Java field: int msosptRightArrowCallout
    pub const msosptRightArrowCallout: i32 = 78;
    // Java field: int msosptUpArrowCallout
    pub const msosptUpArrowCallout: i32 = 79;
    // Java field: int msosptDownArrowCallout
    pub const msosptDownArrowCallout: i32 = 80;
    // Java field: int msosptLeftRightArrowCallout
    pub const msosptLeftRightArrowCallout: i32 = 81;
    // Java field: int msosptUpDownArrowCallout
    pub const msosptUpDownArrowCallout: i32 = 82;
    // Java field: int msosptQuadArrowCallout
    pub const msosptQuadArrowCallout: i32 = 83;
    // Java field: int msosptBevel
    pub const msosptBevel: i32 = 84;
    // Java field: int msosptLeftBracket
    pub const msosptLeftBracket: i32 = 85;
    // Java field: int msosptRightBracket
    pub const msosptRightBracket: i32 = 86;
    // Java field: int msosptLeftBrace
    pub const msosptLeftBrace: i32 = 87;
    // Java field: int msosptRightBrace
    pub const msosptRightBrace: i32 = 88;
    // Java field: int msosptLeftUpArrow
    pub const msosptLeftUpArrow: i32 = 89;
    // Java field: int msosptBentUpArrow
    pub const msosptBentUpArrow: i32 = 90;
    // Java field: int msosptBentArrow
    pub const msosptBentArrow: i32 = 91;
    // Java field: int msosptSeal24
    pub const msosptSeal24: i32 = 92;
    // Java field: int msosptStripedRightArrow
    pub const msosptStripedRightArrow: i32 = 93;
    // Java field: int msosptNotchedRightArrow
    pub const msosptNotchedRightArrow: i32 = 94;
    // Java field: int msosptBlockArc
    pub const msosptBlockArc: i32 = 95;
    // Java field: int msosptSmileyFace
    pub const msosptSmileyFace: i32 = 96;
    // Java field: int msosptVerticalScroll
    pub const msosptVerticalScroll: i32 = 97;
    // Java field: int msosptHorizontalScroll
    pub const msosptHorizontalScroll: i32 = 98;
    // Java field: int msosptCircularArrow
    pub const msosptCircularArrow: i32 = 99;
    // Java field: int msosptNotchedCircularArrow
    pub const msosptNotchedCircularArrow: i32 = 100;
    // Java field: int msosptUturnArrow
    pub const msosptUturnArrow: i32 = 101;
    // Java field: int msosptCurvedRightArrow
    pub const msosptCurvedRightArrow: i32 = 102;
    // Java field: int msosptCurvedLeftArrow
    pub const msosptCurvedLeftArrow: i32 = 103;
    // Java field: int msosptCurvedUpArrow
    pub const msosptCurvedUpArrow: i32 = 104;
    // Java field: int msosptCurvedDownArrow
    pub const msosptCurvedDownArrow: i32 = 105;
    // Java field: int msosptCloudCallout
    pub const msosptCloudCallout: i32 = 106;
    // Java field: int msosptEllipseRibbon
    pub const msosptEllipseRibbon: i32 = 107;
    // Java field: int msosptEllipseRibbon2
    pub const msosptEllipseRibbon2: i32 = 108;
    // Java field: int msosptFlowChartProcess
    pub const msosptFlowChartProcess: i32 = 109;
    // Java field: int msosptFlowChartDecision
    pub const msosptFlowChartDecision: i32 = 110;
    // Java field: int msosptFlowChartInputOutput
    pub const msosptFlowChartInputOutput: i32 = 111;
    // Java field: int msosptFlowChartPredefinedProcess
    pub const msosptFlowChartPredefinedProcess: i32 = 112;
    // Java field: int msosptFlowChartInternalStorage
    pub const msosptFlowChartInternalStorage: i32 = 113;
    // Java field: int msosptFlowChartDocument
    pub const msosptFlowChartDocument: i32 = 114;
    // Java field: int msosptFlowChartMultidocument
    pub const msosptFlowChartMultidocument: i32 = 115;
    // Java field: int msosptFlowChartTerminator
    pub const msosptFlowChartTerminator: i32 = 116;
    // Java field: int msosptFlowChartPreparation
    pub const msosptFlowChartPreparation: i32 = 117;
    // Java field: int msosptFlowChartManualInput
    pub const msosptFlowChartManualInput: i32 = 118;
    // Java field: int msosptFlowChartManualOperation
    pub const msosptFlowChartManualOperation: i32 = 119;
    // Java field: int msosptFlowChartConnector
    pub const msosptFlowChartConnector: i32 = 120;
    // Java field: int msosptFlowChartPunchedCard
    pub const msosptFlowChartPunchedCard: i32 = 121;
    // Java field: int msosptFlowChartPunchedTape
    pub const msosptFlowChartPunchedTape: i32 = 122;
    // Java field: int msosptFlowChartSummingJunction
    pub const msosptFlowChartSummingJunction: i32 = 123;
    // Java field: int msosptFlowChartOr
    pub const msosptFlowChartOr: i32 = 124;
    // Java field: int msosptFlowChartCollate
    pub const msosptFlowChartCollate: i32 = 125;
    // Java field: int msosptFlowChartSort
    pub const msosptFlowChartSort: i32 = 126;
    // Java field: int msosptFlowChartExtract
    pub const msosptFlowChartExtract: i32 = 127;
    // Java field: int msosptFlowChartMerge
    pub const msosptFlowChartMerge: i32 = 128;
    // Java field: int msosptFlowChartOfflineStorage
    pub const msosptFlowChartOfflineStorage: i32 = 129;
    // Java field: int msosptFlowChartOnlineStorage
    pub const msosptFlowChartOnlineStorage: i32 = 130;
    // Java field: int msosptFlowChartMagneticTape
    pub const msosptFlowChartMagneticTape: i32 = 131;
    // Java field: int msosptFlowChartMagneticDisk
    pub const msosptFlowChartMagneticDisk: i32 = 132;
    // Java field: int msosptFlowChartMagneticDrum
    pub const msosptFlowChartMagneticDrum: i32 = 133;
    // Java field: int msosptFlowChartDisplay
    pub const msosptFlowChartDisplay: i32 = 134;
    // Java field: int msosptFlowChartDelay
    pub const msosptFlowChartDelay: i32 = 135;
    // Java field: int msosptTextPlainText
    pub const msosptTextPlainText: i32 = 136;
    // Java field: int msosptTextStop
    pub const msosptTextStop: i32 = 137;
    // Java field: int msosptTextTriangle
    pub const msosptTextTriangle: i32 = 138;
    // Java field: int msosptTextTriangleInverted
    pub const msosptTextTriangleInverted: i32 = 139;
    // Java field: int msosptTextChevron
    pub const msosptTextChevron: i32 = 140;
    // Java field: int msosptTextChevronInverted
    pub const msosptTextChevronInverted: i32 = 141;
    // Java field: int msosptTextRingInside
    pub const msosptTextRingInside: i32 = 142;
    // Java field: int msosptTextRingOutside
    pub const msosptTextRingOutside: i32 = 143;
    // Java field: int msosptTextArchUpCurve
    pub const msosptTextArchUpCurve: i32 = 144;
    // Java field: int msosptTextArchDownCurve
    pub const msosptTextArchDownCurve: i32 = 145;
    // Java field: int msosptTextCircleCurve
    pub const msosptTextCircleCurve: i32 = 146;
    // Java field: int msosptTextButtonCurve
    pub const msosptTextButtonCurve: i32 = 147;
    // Java field: int msosptTextArchUpPour
    pub const msosptTextArchUpPour: i32 = 148;
    // Java field: int msosptTextArchDownPour
    pub const msosptTextArchDownPour: i32 = 149;
    // Java field: int msosptTextCirclePour
    pub const msosptTextCirclePour: i32 = 150;
    // Java field: int msosptTextButtonPour
    pub const msosptTextButtonPour: i32 = 151;
    // Java field: int msosptTextCurveUp
    pub const msosptTextCurveUp: i32 = 152;
    // Java field: int msosptTextCurveDown
    pub const msosptTextCurveDown: i32 = 153;
    // Java field: int msosptTextCascadeUp
    pub const msosptTextCascadeUp: i32 = 154;
    // Java field: int msosptTextCascadeDown
    pub const msosptTextCascadeDown: i32 = 155;
    // Java field: int msosptTextWave1
    pub const msosptTextWave1: i32 = 156;
    // Java field: int msosptTextWave2
    pub const msosptTextWave2: i32 = 157;
    // Java field: int msosptTextWave3
    pub const msosptTextWave3: i32 = 158;
    // Java field: int msosptTextWave4
    pub const msosptTextWave4: i32 = 159;
    // Java field: int msosptTextInflate
    pub const msosptTextInflate: i32 = 160;
    // Java field: int msosptTextDeflate
    pub const msosptTextDeflate: i32 = 161;
    // Java field: int msosptTextInflateBottom
    pub const msosptTextInflateBottom: i32 = 162;
    // Java field: int msosptTextDeflateBottom
    pub const msosptTextDeflateBottom: i32 = 163;
    // Java field: int msosptTextInflateTop
    pub const msosptTextInflateTop: i32 = 164;
    // Java field: int msosptTextDeflateTop
    pub const msosptTextDeflateTop: i32 = 165;
    // Java field: int msosptTextDeflateInflate
    pub const msosptTextDeflateInflate: i32 = 166;
    // Java field: int msosptTextDeflateInflateDeflate
    pub const msosptTextDeflateInflateDeflate: i32 = 167;
    // Java field: int msosptTextFadeRight
    pub const msosptTextFadeRight: i32 = 168;
    // Java field: int msosptTextFadeLeft
    pub const msosptTextFadeLeft: i32 = 169;
    // Java field: int msosptTextFadeUp
    pub const msosptTextFadeUp: i32 = 170;
    // Java field: int msosptTextFadeDown
    pub const msosptTextFadeDown: i32 = 171;
    // Java field: int msosptTextSlantUp
    pub const msosptTextSlantUp: i32 = 172;
    // Java field: int msosptTextSlantDown
    pub const msosptTextSlantDown: i32 = 173;
    // Java field: int msosptTextCanUp
    pub const msosptTextCanUp: i32 = 174;
    // Java field: int msosptTextCanDown
    pub const msosptTextCanDown: i32 = 175;
    // Java field: int msosptFlowChartAlternateProcess
    pub const msosptFlowChartAlternateProcess: i32 = 176;
    // Java field: int msosptFlowChartOffpageConnector
    pub const msosptFlowChartOffpageConnector: i32 = 177;
    // Java field: int msosptCallout90
    pub const msosptCallout90: i32 = 178;
    // Java field: int msosptAccentCallout90
    pub const msosptAccentCallout90: i32 = 179;
    // Java field: int msosptBorderCallout90
    pub const msosptBorderCallout90: i32 = 180;
    // Java field: int msosptAccentBorderCallout90
    pub const msosptAccentBorderCallout90: i32 = 181;
    // Java field: int msosptLeftRightUpArrow
    pub const msosptLeftRightUpArrow: i32 = 182;
    // Java field: int msosptSun
    pub const msosptSun: i32 = 183;
    // Java field: int msosptMoon
    pub const msosptMoon: i32 = 184;
    // Java field: int msosptBracketPair
    pub const msosptBracketPair: i32 = 185;
    // Java field: int msosptBracePair
    pub const msosptBracePair: i32 = 186;
    // Java field: int msosptSeal4
    pub const msosptSeal4: i32 = 187;
    // Java field: int msosptDoubleWave
    pub const msosptDoubleWave: i32 = 188;
    // Java field: int msosptActionButtonBlank
    pub const msosptActionButtonBlank: i32 = 189;
    // Java field: int msosptActionButtonHome
    pub const msosptActionButtonHome: i32 = 190;
    // Java field: int msosptActionButtonHelp
    pub const msosptActionButtonHelp: i32 = 191;
    // Java field: int msosptActionButtonInformation
    pub const msosptActionButtonInformation: i32 = 192;
    // Java field: int msosptActionButtonForwardNext
    pub const msosptActionButtonForwardNext: i32 = 193;
    // Java field: int msosptActionButtonBackPrevious
    pub const msosptActionButtonBackPrevious: i32 = 194;
    // Java field: int msosptActionButtonEnd
    pub const msosptActionButtonEnd: i32 = 195;
    // Java field: int msosptActionButtonBeginning
    pub const msosptActionButtonBeginning: i32 = 196;
    // Java field: int msosptActionButtonReturn
    pub const msosptActionButtonReturn: i32 = 197;
    // Java field: int msosptActionButtonDocument
    pub const msosptActionButtonDocument: i32 = 198;
    // Java field: int msosptActionButtonSound
    pub const msosptActionButtonSound: i32 = 199;
    // Java field: int msosptActionButtonMovie
    pub const msosptActionButtonMovie: i32 = 200;
    // Java field: int msosptHostControl
    pub const msosptHostControl: i32 = 201;
    // Java field: int msosptTextBox
    pub const msosptTextBox: i32 = 202;
    // Java field: int msosptGroupShape
    pub const msosptGroupShape: i32 = 203;
    // Java field: int msosptPicture
    pub const msosptPicture: i32 = 204;
    // Java field: int msosptVmlMax
    pub const msosptVmlMax: i32 = 205;
    // Java field: int msosptDmlMin
    pub const msosptDmlMin: i32 = 220;
    // Java field: int msosptChartPlus
    pub const msosptChartPlus: i32 = 221;
    // Java field: int msosptChartStar
    pub const msosptChartStar: i32 = 222;
    // Java field: int msosptChartX
    pub const msosptChartX: i32 = 223;
    // Java field: int msosptChord
    pub const msosptChord: i32 = 224;
    // Java field: int msosptCloud
    pub const msosptCloud: i32 = 225;
    // Java field: int msosptCorner
    pub const msosptCorner: i32 = 226;
    // Java field: int msosptCornerTabs
    pub const msosptCornerTabs: i32 = 227;
    // Java field: int msosptDecagon
    pub const msosptDecagon: i32 = 228;
    // Java field: int msosptDiagStripe
    pub const msosptDiagStripe: i32 = 229;
    // Java field: int msosptDodecagon
    pub const msosptDodecagon: i32 = 230;
    // Java field: int msosptFunnel
    pub const msosptFunnel: i32 = 231;
    // Java field: int msosptGear6
    pub const msosptGear6: i32 = 232;
    // Java field: int msosptGear9
    pub const msosptGear9: i32 = 233;
    // Java field: int msosptHalfFrame
    pub const msosptHalfFrame: i32 = 234;
    // Java field: int msosptHeptagon
    pub const msosptHeptagon: i32 = 235;
    // Java field: int msosptLeftCircularArrow
    pub const msosptLeftCircularArrow: i32 = 236;
    // Java field: int msosptLeftRightCircularArrow
    pub const msosptLeftRightCircularArrow: i32 = 237;
    // Java field: int msosptLeftRightRibbon
    pub const msosptLeftRightRibbon: i32 = 238;
    // Java field: int msosptLineInv
    pub const msosptLineInv: i32 = 239;
    // Java field: int msosptMathDivide
    pub const msosptMathDivide: i32 = 240;
    // Java field: int msosptMathEqual
    pub const msosptMathEqual: i32 = 241;
    // Java field: int msosptMathMinus
    pub const msosptMathMinus: i32 = 242;
    // Java field: int msosptMathMultiply
    pub const msosptMathMultiply: i32 = 243;
    // Java field: int msosptMathNotEqual
    pub const msosptMathNotEqual: i32 = 244;
    // Java field: int msosptMathPlus
    pub const msosptMathPlus: i32 = 245;
    // Java field: int msosptNonIsoscelesTrapezoid
    pub const msosptNonIsoscelesTrapezoid: i32 = 246;
    // Java field: int msosptPie
    pub const msosptPie: i32 = 247;
    // Java field: int msosptPieWedge
    pub const msosptPieWedge: i32 = 248;
    // Java field: int msosptPlaqueTabs
    pub const msosptPlaqueTabs: i32 = 249;
    // Java field: int msosptRound1Rect
    pub const msosptRound1Rect: i32 = 250;
    // Java field: int msosptRound2DiagRect
    pub const msosptRound2DiagRect: i32 = 251;
    // Java field: int msosptRound2SameRect
    pub const msosptRound2SameRect: i32 = 252;
    // Java field: int msosptSnip1Rect
    pub const msosptSnip1Rect: i32 = 253;
    // Java field: int msosptSnip2DiagRect
    pub const msosptSnip2DiagRect: i32 = 254;
    // Java field: int msosptSnip2SameRect
    pub const msosptSnip2SameRect: i32 = 255;
    // Java field: int msosptSnipRoundRect
    pub const msosptSnipRoundRect: i32 = 256;
    // Java field: int msosptSquareTabs
    pub const msosptSquareTabs: i32 = 257;
    // Java field: int msosptStar10
    pub const msosptStar10: i32 = 258;
    // Java field: int msosptStar12
    pub const msosptStar12: i32 = 259;
    // Java field: int msosptStar6
    pub const msosptStar6: i32 = 260;
    // Java field: int msosptStar7
    pub const msosptStar7: i32 = 261;
    // Java field: int msosptSwooshArrow
    pub const msosptSwooshArrow: i32 = 262;
    // Java field: int msosptTeardrop
    pub const msosptTeardrop: i32 = 263;
    // Java field: int msosptMax
    pub const msosptMax: i32 = 264;
    // Java field: int msosptNil
    pub const msosptNil: i32 = 4095;
}

// Java field: Map<Integer, Integer> mAdjMap
static M_ADJ_MAP: OnceLock<HashMap<i32, i32>> = OnceLock::new(); // HashMap for mAdjMap
impl ShapeType {
    // Java field: String rect
    pub const rect: &str = "rect";
    // Java field: String roundRect
    pub const roundRect: &str = "roundRect";
    // Java field: String ellipse
    pub const ellipse: &str = "ellipse";
    // Java field: String diamond
    pub const diamond: &str = "diamond";
    // Java field: String triangle
    pub const triangle: &str = "triangle";
    // Java field: String rtTriangle
    pub const rtTriangle: &str = "rtTriangle";
    // Java field: String parallelogram
    pub const parallelogram: &str = "parallelogram";
    // Java field: String trapezoid
    pub const trapezoid: &str = "trapezoid";
    // Java field: String hexagon
    pub const hexagon: &str = "hexagon";
    // Java field: String octagon
    pub const octagon: &str = "octagon";
    // Java field: String plus
    pub const plus: &str = "plus";
    // Java field: String star5
    pub const star5: &str = "star5";
    // Java field: String rightArrow
    pub const rightArrow: &str = "rightArrow";
    // Java field: String thickArrow
    pub const thickArrow: &str = "thickArrow";
    // Java field: String homePlate
    pub const homePlate: &str = "homePlate";
    // Java field: String cube
    pub const cube: &str = "cube";
    // Java field: String wedgeRoundRectCallout
    pub const wedgeRoundRectCallout: &str = "wedgeRoundRectCallout";
    // Java field: String star16
    pub const star16: &str = "star16";
    // Java field: String arc
    pub const arc: &str = "arc";
    // Java field: String line
    pub const line: &str = "line";
    // Java field: String plaque
    pub const plaque: &str = "plaque";
    // Java field: String can
    pub const can: &str = "can";
    // Java field: String donut
    pub const donut: &str = "donut";
    // Java field: String straightConnector1
    pub const straightConnector1: &str = "straightConnector1";
    // Java field: String bentConnector2
    pub const bentConnector2: &str = "bentConnector2";
    // Java field: String bentConnector3
    pub const bentConnector3: &str = "bentConnector3";
    // Java field: String bentConnector4
    pub const bentConnector4: &str = "bentConnector4";
    // Java field: String bentConnector5
    pub const bentConnector5: &str = "bentConnector5";
    // Java field: String curvedConnector2
    pub const curvedConnector2: &str = "curvedConnector2";
    // Java field: String curvedConnector3
    pub const curvedConnector3: &str = "curvedConnector3";
    // Java field: String curvedConnector4
    pub const curvedConnector4: &str = "curvedConnector4";
    // Java field: String curvedConnector5
    pub const curvedConnector5: &str = "curvedConnector5";
    // Java field: String callout1
    pub const callout1: &str = "callout1";
    // Java field: String callout2
    pub const callout2: &str = "callout2";
    // Java field: String callout3
    pub const callout3: &str = "callout3";
    // Java field: String accentCallout1
    pub const accentCallout1: &str = "accentCallout1";
    // Java field: String accentCallout2
    pub const accentCallout2: &str = "accentCallout2";
    // Java field: String accentCallout3
    pub const accentCallout3: &str = "accentCallout3";
    // Java field: String borderCallout1
    pub const borderCallout1: &str = "borderCallout1";
    // Java field: String borderCallout2
    pub const borderCallout2: &str = "borderCallout2";
    // Java field: String borderCallout3
    pub const borderCallout3: &str = "borderCallout3";
    // Java field: String accentBorderCallout1
    pub const accentBorderCallout1: &str = "accentBorderCallout1";
    // Java field: String accentBorderCallout2
    pub const accentBorderCallout2: &str = "accentBorderCallout2";
    // Java field: String accentBorderCallout3
    pub const accentBorderCallout3: &str = "accentBorderCallout3";
    // Java field: String ribbon
    pub const ribbon: &str = "ribbon";
    // Java field: String ribbon2
    pub const ribbon2: &str = "ribbon2";
    // Java field: String chevron
    pub const chevron: &str = "chevron";
    // Java field: String pentagon
    pub const pentagon: &str = "pentagon";
    // Java field: String noSmoking
    pub const noSmoking: &str = "noSmoking";
    // Java field: String star8
    pub const star8: &str = "star8";
    // Java field: String star32
    pub const star32: &str = "star32";
    // Java field: String wedgeRectCallout
    pub const wedgeRectCallout: &str = "wedgeRectCallout";
    // Java field: String wedgeEllipseCallout
    pub const wedgeEllipseCallout: &str = "wedgeEllipseCallout";
    // Java field: String wave
    pub const wave: &str = "wave";
    // Java field: String foldedCorner
    pub const foldedCorner: &str = "foldedCorner";
    // Java field: String leftArrow
    pub const leftArrow: &str = "leftArrow";
    // Java field: String downArrow
    pub const downArrow: &str = "downArrow";
    // Java field: String upArrow
    pub const upArrow: &str = "upArrow";
    // Java field: String leftRightArrow
    pub const leftRightArrow: &str = "leftRightArrow";
    // Java field: String upDownArrow
    pub const upDownArrow: &str = "upDownArrow";
    // Java field: String irregularSeal1
    pub const irregularSeal1: &str = "irregularSeal1";
    // Java field: String irregularSeal2
    pub const irregularSeal2: &str = "irregularSeal2";
    // Java field: String lightningBolt
    pub const lightningBolt: &str = "lightningBolt";
    // Java field: String heart
    pub const heart: &str = "heart";
    // Java field: String frame
    pub const frame: &str = "frame";
    // Java field: String quadArrow
    pub const quadArrow: &str = "quadArrow";
    // Java field: String leftArrowCallout
    pub const leftArrowCallout: &str = "leftArrowCallout";
    // Java field: String rightArrowCallout
    pub const rightArrowCallout: &str = "rightArrowCallout";
    // Java field: String upArrowCallout
    pub const upArrowCallout: &str = "upArrowCallout";
    // Java field: String downArrowCallout
    pub const downArrowCallout: &str = "downArrowCallout";
    // Java field: String leftRightArrowCallout
    pub const leftRightArrowCallout: &str = "leftRightArrowCallout";
    // Java field: String upDownArrowCallout
    pub const upDownArrowCallout: &str = "upDownArrowCallout";
    // Java field: String quadArrowCallout
    pub const quadArrowCallout: &str = "quadArrowCallout";
    // Java field: String bevel
    pub const bevel: &str = "bevel";
    // Java field: String leftBracket
    pub const leftBracket: &str = "leftBracket";
    // Java field: String rightBracket
    pub const rightBracket: &str = "rightBracket";
    // Java field: String leftBrace
    pub const leftBrace: &str = "leftBrace";
    // Java field: String rightBrace
    pub const rightBrace: &str = "rightBrace";
    // Java field: String leftUpArrow
    pub const leftUpArrow: &str = "leftUpArrow";
    // Java field: String bentUpArrow
    pub const bentUpArrow: &str = "bentUpArrow";
    // Java field: String bentArrow
    pub const bentArrow: &str = "bentArrow";
    // Java field: String star24
    pub const star24: &str = "star24";
    // Java field: String stripedRightArrow
    pub const stripedRightArrow: &str = "stripedRightArrow";
    // Java field: String notchedRightArrow
    pub const notchedRightArrow: &str = "notchedRightArrow";
    // Java field: String blockArc
    pub const blockArc: &str = "blockArc";
    // Java field: String smileyFace
    pub const smileyFace: &str = "smileyFace";
    // Java field: String verticalScroll
    pub const verticalScroll: &str = "verticalScroll";
    // Java field: String horizontalScroll
    pub const horizontalScroll: &str = "horizontalScroll";
    // Java field: String circularArrow
    pub const circularArrow: &str = "circularArrow";
    // Java field: String uturnArrow
    pub const uturnArrow: &str = "uturnArrow";
    // Java field: String curvedRightArrow
    pub const curvedRightArrow: &str = "curvedRightArrow";
    // Java field: String curvedLeftArrow
    pub const curvedLeftArrow: &str = "curvedLeftArrow";
    // Java field: String curvedUpArrow
    pub const curvedUpArrow: &str = "curvedUpArrow";
    // Java field: String curvedDownArrow
    pub const curvedDownArrow: &str = "curvedDownArrow";
    // Java field: String cloudCallout
    pub const cloudCallout: &str = "cloudCallout";
    // Java field: String ellipseRibbon
    pub const ellipseRibbon: &str = "ellipseRibbon";
    // Java field: String ellipseRibbon2
    pub const ellipseRibbon2: &str = "ellipseRibbon2";
    // Java field: String flowChartProcess
    pub const flowChartProcess: &str = "flowChartProcess";
    // Java field: String flowChartDecision
    pub const flowChartDecision: &str = "flowChartDecision";
    // Java field: String flowChartInputOutput
    pub const flowChartInputOutput: &str = "flowChartInputOutput";
    // Java field: String flowChartPredefinedProcess
    pub const flowChartPredefinedProcess: &str = "flowChartPredefinedProcess";
    // Java field: String flowChartInternalStorage
    pub const flowChartInternalStorage: &str = "flowChartInternalStorage";
    // Java field: String flowChartDocument
    pub const flowChartDocument: &str = "flowChartDocument";
    // Java field: String flowChartMultidocument
    pub const flowChartMultidocument: &str = "flowChartMultidocument";
    // Java field: String flowChartTerminator
    pub const flowChartTerminator: &str = "flowChartTerminator";
    // Java field: String flowChartPreparation
    pub const flowChartPreparation: &str = "flowChartPreparation";
    // Java field: String flowChartManualInput
    pub const flowChartManualInput: &str = "flowChartManualInput";
    // Java field: String flowChartManualOperation
    pub const flowChartManualOperation: &str = "flowChartManualOperation";
    // Java field: String flowChartConnector
    pub const flowChartConnector: &str = "flowChartConnector";
    // Java field: String flowChartPunchedCard
    pub const flowChartPunchedCard: &str = "flowChartPunchedCard";
    // Java field: String flowChartPunchedTape
    pub const flowChartPunchedTape: &str = "flowChartPunchedTape";
    // Java field: String flowChartSummingJunction
    pub const flowChartSummingJunction: &str = "flowChartSummingJunction";
    // Java field: String flowChartOr
    pub const flowChartOr: &str = "flowChartOr";
    // Java field: String flowChartCollate
    pub const flowChartCollate: &str = "flowChartCollate";
    // Java field: String flowChartSort
    pub const flowChartSort: &str = "flowChartSort";
    // Java field: String flowChartExtract
    pub const flowChartExtract: &str = "flowChartExtract";
    // Java field: String flowChartMerge
    pub const flowChartMerge: &str = "flowChartMerge";
    // Java field: String flowChartOfflineStorage
    pub const flowChartOfflineStorage: &str = "flowChartOfflineStorage";
    // Java field: String flowChartOnlineStorage
    pub const flowChartOnlineStorage: &str = "flowChartOnlineStorage";
    // Java field: String flowChartMagneticTape
    pub const flowChartMagneticTape: &str = "flowChartMagneticTape";
    // Java field: String flowChartMagneticDisk
    pub const flowChartMagneticDisk: &str = "flowChartMagneticDisk";
    // Java field: String flowChartMagneticDrum
    pub const flowChartMagneticDrum: &str = "flowChartMagneticDrum";
    // Java field: String flowChartDisplay
    pub const flowChartDisplay: &str = "flowChartDisplay";
    // Java field: String flowChartDelay
    pub const flowChartDelay: &str = "flowChartDelay";
    // Java field: String flowChartAlternateProcess
    pub const flowChartAlternateProcess: &str = "flowChartAlternateProcess";
    // Java field: String flowChartOffpageConnector
    pub const flowChartOffpageConnector: &str = "flowChartOffpageConnector";
    // Java field: String leftRightUpArrow
    pub const leftRightUpArrow: &str = "leftRightUpArrow";
    // Java field: String sun
    pub const sun: &str = "sun";
    // Java field: String moon
    pub const moon: &str = "moon";
    // Java field: String bracketPair
    pub const bracketPair: &str = "bracketPair";
    // Java field: String bracePair
    pub const bracePair: &str = "bracePair";
    // Java field: String star4
    pub const star4: &str = "star4";
    // Java field: String doubleWave
    pub const doubleWave: &str = "doubleWave";
    // Java field: String actionButtonBlank
    pub const actionButtonBlank: &str = "actionButtonBlank";
    // Java field: String actionButtonHome
    pub const actionButtonHome: &str = "actionButtonHome";
    // Java field: String actionButtonHelp
    pub const actionButtonHelp: &str = "actionButtonHelp";
    // Java field: String actionButtonInformation
    pub const actionButtonInformation: &str = "actionButtonInformation";
    // Java field: String actionButtonForwardNext
    pub const actionButtonForwardNext: &str = "actionButtonForwardNext";
    // Java field: String actionButtonBackPrevious
    pub const actionButtonBackPrevious: &str = "actionButtonBackPrevious";
    // Java field: String actionButtonEnd
    pub const actionButtonEnd: &str = "actionButtonEnd";
    // Java field: String actionButtonBeginning
    pub const actionButtonBeginning: &str = "actionButtonBeginning";
    // Java field: String actionButtonReturn
    pub const actionButtonReturn: &str = "actionButtonReturn";
    // Java field: String actionButtonDocument
    pub const actionButtonDocument: &str = "actionButtonDocument";
    // Java field: String actionButtonSound
    pub const actionButtonSound: &str = "actionButtonSound";
    // Java field: String actionButtonMovie
    pub const actionButtonMovie: &str = "actionButtonMovie";
    // Java field: String hostControl
    pub const hostControl: &str = "hostControl";
    // Java field: String textBox
    pub const textBox: &str = "textBox";
    // Java field: String groupShape
    pub const groupShape: &str = "groupShape";
    // Java field: String picture
    pub const picture: &str = "picture";
    // Java field: String chartPlus
    pub const chartPlus: &str = "chartPlus";
    // Java field: String chartStar
    pub const chartStar: &str = "chartStar";
    // Java field: String chartX
    pub const chartX: &str = "chartX";
    // Java field: String chord
    pub const chord: &str = "chord";
    // Java field: String cloud
    pub const cloud: &str = "cloud";
    // Java field: String corner
    pub const corner: &str = "corner";
    // Java field: String cornerTabs
    pub const cornerTabs: &str = "cornerTabs";
    // Java field: String decagon
    pub const decagon: &str = "decagon";
    // Java field: String diagStripe
    pub const diagStripe: &str = "diagStripe";
    // Java field: String dodecagon
    pub const dodecagon: &str = "dodecagon";
    // Java field: String funnel
    pub const funnel: &str = "funnel";
    // Java field: String gear6
    pub const gear6: &str = "gear6";
    // Java field: String gear9
    pub const gear9: &str = "gear9";
    // Java field: String halfFrame
    pub const halfFrame: &str = "halfFrame";
    // Java field: String heptagon
    pub const heptagon: &str = "heptagon";
    // Java field: String leftCircularArrow
    pub const leftCircularArrow: &str = "leftCircularArrow";
    // Java field: String leftRightCircularArrow
    pub const leftRightCircularArrow: &str = "leftRightCircularArrow";
    // Java field: String leftRightRibbon
    pub const leftRightRibbon: &str = "leftRightRibbon";
    // Java field: String lineInv
    pub const lineInv: &str = "lineInv";
    // Java field: String mathDivide
    pub const mathDivide: &str = "mathDivide";
    // Java field: String mathEqual
    pub const mathEqual: &str = "mathEqual";
    // Java field: String mathMinus
    pub const mathMinus: &str = "mathMinus";
    // Java field: String mathMultiply
    pub const mathMultiply: &str = "mathMultiply";
    // Java field: String mathNotEqual
    pub const mathNotEqual: &str = "mathNotEqual";
    // Java field: String mathPlus
    pub const mathPlus: &str = "mathPlus";
    // Java field: String nonIsoscelesTrapezoid
    pub const nonIsoscelesTrapezoid: &str = "nonIsoscelesTrapezoid";
    // Java field: String pie
    pub const pie: &str = "pie";
    // Java field: String pieWedge
    pub const pieWedge: &str = "pieWedge";
    // Java field: String plaqueTabs
    pub const plaqueTabs: &str = "plaqueTabs";
    // Java field: String round1Rect
    pub const round1Rect: &str = "round1Rect";
    // Java field: String round2DiagRect
    pub const round2DiagRect: &str = "round2DiagRect";
    // Java field: String round2SameRect
    pub const round2SameRect: &str = "round2SameRect";
    // Java field: String snip1Rect
    pub const snip1Rect: &str = "snip1Rect";
    // Java field: String snip2DiagRect
    pub const snip2DiagRect: &str = "snip2DiagRect";
    // Java field: String snip2SameRect
    pub const snip2SameRect: &str = "snip2SameRect";
    // Java field: String snipRoundRect
    pub const snipRoundRect: &str = "snipRoundRect";
    // Java field: String squareTabs
    pub const squareTabs: &str = "squareTabs";
    // Java field: String star10
    pub const star10: &str = "star10";
    // Java field: String star12
    pub const star12: &str = "star12";
    // Java field: String star6
    pub const star6: &str = "star6";
    // Java field: String star7
    pub const star7: &str = "star7";
    // Java field: String swooshArrow
    pub const swooshArrow: &str = "swooshArrow";
    // Java field: String teardrop
    pub const teardrop: &str = "teardrop";
    // Java field: String textNoShape
    pub const textNoShape: &str = "textNoShape";
    // Java field: String textPlainText
    pub const textPlainText: &str = "textPlain";
    // Java field: String textStop
    pub const textStop: &str = "textStop";
    // Java field: String textTriangle
    pub const textTriangle: &str = "textTriangle";
    // Java field: String textTriangleInverted
    pub const textTriangleInverted: &str = "textTriangleInverted";
    // Java field: String textChevron
    pub const textChevron: &str = "textChevron";
    // Java field: String textChevronInverted
    pub const textChevronInverted: &str = "textChevronInverted";
    // Java field: String textRingInside
    pub const textRingInside: &str = "textRingInside";
    // Java field: String textRingOutside
    pub const textRingOutside: &str = "textRingOutside";
    // Java field: String textArchUpCurve
    pub const textArchUpCurve: &str = "textArchUp";
    // Java field: String textArchDownCurve
    pub const textArchDownCurve: &str = "textArchDown";
    // Java field: String textCircleCurve
    pub const textCircleCurve: &str = "textCircle";
    // Java field: String textButtonCurve
    pub const textButtonCurve: &str = "textButton";
    // Java field: String textArchUpPour
    pub const textArchUpPour: &str = "textArchUpPour";
    // Java field: String textArchDownPour
    pub const textArchDownPour: &str = "textArchDownPour";
    // Java field: String textCirclePour
    pub const textCirclePour: &str = "textCirclePour";
    // Java field: String textButtonPour
    pub const textButtonPour: &str = "textButtonPour";
    // Java field: String textCurveUp
    pub const textCurveUp: &str = "textCurveUp";
    // Java field: String textCurveDown
    pub const textCurveDown: &str = "textCurveDown";
    // Java field: String textCascadeUp
    pub const textCascadeUp: &str = "textCascadeUp";
    // Java field: String textCascadeDown
    pub const textCascadeDown: &str = "textCascadeDown";
    // Java field: String textWave1
    pub const textWave1: &str = "textWave1";
    // Java field: String textWave2
    pub const textWave2: &str = "textWave2";
    // Java field: String textWave3
    pub const textWave3: &str = "textDoubleWave1";
    // Java field: String textWave4
    pub const textWave4: &str = "textWave4";
    // Java field: String textInflate
    pub const textInflate: &str = "textInflate";
    // Java field: String textDeflate
    pub const textDeflate: &str = "textDeflate";
    // Java field: String textInflateBottom
    pub const textInflateBottom: &str = "textInflateBottom";
    // Java field: String textDeflateBottom
    pub const textDeflateBottom: &str = "textDeflateBottom";
    // Java field: String textInflateTop
    pub const textInflateTop: &str = "textInflateTop";
    // Java field: String textDeflateTop
    pub const textDeflateTop: &str = "textDeflateTop";
    // Java field: String textDeflateInflate
    pub const textDeflateInflate: &str = "textDeflateInflate";
    // Java field: String textDeflateInflateDeflate
    pub const textDeflateInflateDeflate: &str = "textDeflateInflateDeflate";
    // Java field: String textFadeRight
    pub const textFadeRight: &str = "textFadeRight";
    // Java field: String textFadeLeft
    pub const textFadeLeft: &str = "textFadeLeft";
    // Java field: String textFadeUp
    pub const textFadeUp: &str = "textFadeUp";
    // Java field: String textFadeDown
    pub const textFadeDown: &str = "textFadeDown";
    // Java field: String textSlantUp
    pub const textSlantUp: &str = "textSlantUp";
    // Java field: String textSlantDown
    pub const textSlantDown: &str = "textSlantDown";
    // Java field: String textCanUp
    pub const textCanUp: &str = "textCanUp";
    // Java field: String textCanDown
    pub const textCanDown: &str = "textCanDown";
}

// Java field: Map<Integer, String> vml2DmlMap
static VML2DML_MAP: OnceLock<HashMap<i32, &'static str>> = OnceLock::new(); // HashMap for vml2DmlMap
// Java field: Map<String, Integer> dml2VmlMap
static DML2VML_MAP: OnceLock<HashMap<&'static str, i32>> = OnceLock::new(); // HashMap for dml2VmlMap
// Java field: Map<Integer, String> docWordArt2DmlMap
static DOC_WORDART2DML_MAP: OnceLock<HashMap<i32, &'static str>> = OnceLock::new(); // HashMap for docWordArt2DmlMap
impl ShapeType {
    // Java: ShapeType()
    // LLM No Rust equivalent: Rust uses struct, no constructor needed
    // Java: super();
    // LLM No Rust equivalent: no super in Rust

    // Java: int getAdjCount(int shapeType)
    pub fn getAdjCount(shapeType: i32) -> i32 {
        // synchronized: OnceLock provides Mutex-equivalent lock for thread-safe lazy init
        // Java: if (mAdjMap == null)
        // LLM JavaLine Aggregation: Rust uses OnceLock::get_or_init for lazy initialization
        let map = M_ADJ_MAP.get_or_init(|| {
            // Java: mAdjMap = new HashMap();
            let mut m: HashMap<i32, i32> = HashMap::new();
            // Java: mAdjMap.put(2, 1);
            m.insert(2, 1);
            // Java: mAdjMap.put(5, 1);
            m.insert(5, 1);
            // Java: mAdjMap.put(7, 1);
            m.insert(7, 1);
            // Java: mAdjMap.put(8, 1);
            m.insert(8, 1);
            // Java: mAdjMap.put(9, 1);
            m.insert(9, 1);
            // Java: mAdjMap.put(10, 1);
            m.insert(10, 1);
            // Java: mAdjMap.put(11, 1);
            m.insert(11, 1);
            // Java: mAdjMap.put(12, 1);
            m.insert(12, 1);
            // Java: mAdjMap.put(13, 2);
            m.insert(13, 2);
            // Java: mAdjMap.put(15, 1);
            m.insert(15, 1);
            // Java: mAdjMap.put(16, 1);
            m.insert(16, 1);
            // Java: mAdjMap.put(17, 2);
            m.insert(17, 2);
            // Java: mAdjMap.put(18, 1);
            m.insert(18, 1);
            // Java: mAdjMap.put(19, 2);
            m.insert(19, 2);
            // Java: mAdjMap.put(21, 1);
            m.insert(21, 1);
            // Java: mAdjMap.put(22, 1);
            m.insert(22, 1);
            // Java: mAdjMap.put(23, 1);
            m.insert(23, 1);
            // Java: mAdjMap.put(34, 1);
            m.insert(34, 1);
            // Java: mAdjMap.put(35, 2);
            m.insert(35, 2);
            // Java: mAdjMap.put(36, 3);
            m.insert(36, 3);
            // Java: mAdjMap.put(38, 1);
            m.insert(38, 1);
            // Java: mAdjMap.put(39, 2);
            m.insert(39, 2);
            // Java: mAdjMap.put(40, 3);
            m.insert(40, 3);
            // Java: mAdjMap.put(41, 4);
            m.insert(41, 4);
            // Java: mAdjMap.put(42, 6);
            m.insert(42, 6);
            // Java: mAdjMap.put(43, 8);
            m.insert(43, 8);
            // Java: mAdjMap.put(44, 4);
            m.insert(44, 4);
            // Java: mAdjMap.put(45, 6);
            m.insert(45, 6);
            // Java: mAdjMap.put(46, 8);
            m.insert(46, 8);
            // Java: mAdjMap.put(47, 4);
            m.insert(47, 4);
            // Java: mAdjMap.put(48, 6);
            m.insert(48, 6);
            // Java: mAdjMap.put(49, 8);
            m.insert(49, 8);
            // Java: mAdjMap.put(50, 4);
            m.insert(50, 4);
            // Java: mAdjMap.put(51, 6);
            m.insert(51, 6);
            // Java: mAdjMap.put(52, 8);
            m.insert(52, 8);
            // Java: mAdjMap.put(53, 2);
            m.insert(53, 2);
            // Java: mAdjMap.put(54, 2);
            m.insert(54, 2);
            // Java: mAdjMap.put(55, 1);
            m.insert(55, 1);
            // Java: mAdjMap.put(57, 1);
            m.insert(57, 1);
            // Java: mAdjMap.put(58, 1);
            m.insert(58, 1);
            // Java: mAdjMap.put(59, 1);
            m.insert(59, 1);
            // Java: mAdjMap.put(60, 1);
            m.insert(60, 1);
            // Java: mAdjMap.put(61, 2);
            m.insert(61, 2);
            // Java: mAdjMap.put(62, 2);
            m.insert(62, 2);
            // Java: mAdjMap.put(63, 2);
            m.insert(63, 2);
            // Java: mAdjMap.put(64, 2);
            m.insert(64, 2);
            // Java: mAdjMap.put(65, 1);
            m.insert(65, 1);
            // Java: mAdjMap.put(66, 2);
            m.insert(66, 2);
            // Java: mAdjMap.put(67, 2);
            m.insert(67, 2);
            // Java: mAdjMap.put(68, 2);
            m.insert(68, 2);
            // Java: mAdjMap.put(69, 2);
            m.insert(69, 2);
            // Java: mAdjMap.put(70, 2);
            m.insert(70, 2);
            // Java: mAdjMap.put(75, 1);
            m.insert(75, 1);
            // Java: mAdjMap.put(76, 3);
            m.insert(76, 3);
            // Java: mAdjMap.put(77, 4);
            m.insert(77, 4);
            // Java: mAdjMap.put(78, 4);
            m.insert(78, 4);
            // Java: mAdjMap.put(79, 4);
            m.insert(79, 4);
            // Java: mAdjMap.put(80, 4);
            m.insert(80, 4);
            // Java: mAdjMap.put(81, 4);
            m.insert(81, 4);
            // Java: mAdjMap.put(82, 4);
            m.insert(82, 4);
            // Java: mAdjMap.put(83, 4);
            m.insert(83, 4);
            // Java: mAdjMap.put(84, 1);
            m.insert(84, 1);
            // Java: mAdjMap.put(85, 1);
            m.insert(85, 1);
            // Java: mAdjMap.put(86, 1);
            m.insert(86, 1);
            // Java: mAdjMap.put(87, 2);
            m.insert(87, 2);
            // Java: mAdjMap.put(88, 2);
            m.insert(88, 2);
            // Java: mAdjMap.put(89, 3);
            m.insert(89, 3);
            // Java: mAdjMap.put(90, 3);
            m.insert(90, 3);
            // Java: mAdjMap.put(91, 4);
            m.insert(91, 4);
            // Java: mAdjMap.put(92, 1);
            m.insert(92, 1);
            // Java: mAdjMap.put(93, 2);
            m.insert(93, 2);
            // Java: mAdjMap.put(94, 2);
            m.insert(94, 2);
            // Java: mAdjMap.put(95, 3);
            m.insert(95, 3);
            // Java: mAdjMap.put(96, 1);
            m.insert(96, 1);
            // Java: mAdjMap.put(97, 1);
            m.insert(97, 1);
            // Java: mAdjMap.put(98, 1);
            m.insert(98, 1);
            // Java: mAdjMap.put(99, 5);
            m.insert(99, 5);
            // Java: mAdjMap.put(100, 5);
            m.insert(100, 5);
            // Java: mAdjMap.put(101, 5);
            m.insert(101, 5);
            // Java: mAdjMap.put(102, 3);
            m.insert(102, 3);
            // Java: mAdjMap.put(103, 3);
            m.insert(103, 3);
            // Java: mAdjMap.put(104, 3);
            m.insert(104, 3);
            // Java: mAdjMap.put(105, 3);
            m.insert(105, 3);
            // Java: mAdjMap.put(106, 2);
            m.insert(106, 2);
            // Java: mAdjMap.put(107, 3);
            m.insert(107, 3);
            // Java: mAdjMap.put(108, 3);
            m.insert(108, 3);
            // Java: mAdjMap.put(170, 1);
            m.insert(170, 1);
            // Java: mAdjMap.put(172, 1);
            m.insert(172, 1);
            // Java: mAdjMap.put(178, 4);
            m.insert(178, 4);
            // Java: mAdjMap.put(179, 4);
            m.insert(179, 4);
            // Java: mAdjMap.put(180, 4);
            m.insert(180, 4);
            // Java: mAdjMap.put(181, 4);
            m.insert(181, 4);
            // Java: mAdjMap.put(182, 3);
            m.insert(182, 3);
            // Java: mAdjMap.put(183, 1);
            m.insert(183, 1);
            // Java: mAdjMap.put(184, 1);
            m.insert(184, 1);
            // Java: mAdjMap.put(185, 1);
            m.insert(185, 1);
            // Java: mAdjMap.put(186, 1);
            m.insert(186, 1);
            // Java: mAdjMap.put(187, 1);
            m.insert(187, 1);
            // Java: mAdjMap.put(188, 2);
            m.insert(188, 2);
            // Java: mAdjMap.put(224, 2);
            m.insert(224, 2);
            // Java: mAdjMap.put(226, 2);
            m.insert(226, 2);
            // Java: mAdjMap.put(229, 1);
            m.insert(229, 1);
            // Java: mAdjMap.put(232, 2);
            m.insert(232, 2);
            // Java: mAdjMap.put(233, 2);
            m.insert(233, 2);
            // Java: mAdjMap.put(234, 2);
            m.insert(234, 2);
            // Java: mAdjMap.put(236, 5);
            m.insert(236, 5);
            // Java: mAdjMap.put(237, 5);
            m.insert(237, 5);
            // Java: mAdjMap.put(238, 3);
            m.insert(238, 3);
            // Java: mAdjMap.put(240, 3);
            m.insert(240, 3);
            // Java: mAdjMap.put(241, 2);
            m.insert(241, 2);
            // Java: mAdjMap.put(242, 1);
            m.insert(242, 1);
            // Java: mAdjMap.put(243, 1);
            m.insert(243, 1);
            // Java: mAdjMap.put(244, 3);
            m.insert(244, 3);
            // Java: mAdjMap.put(245, 1);
            m.insert(245, 1);
            // Java: mAdjMap.put(246, 2);
            m.insert(246, 2);
            // Java: mAdjMap.put(247, 2);
            m.insert(247, 2);
            // Java: mAdjMap.put(250, 1);
            m.insert(250, 1);
            // Java: mAdjMap.put(251, 2);
            m.insert(251, 2);
            // Java: mAdjMap.put(252, 2);
            m.insert(252, 2);
            // Java: mAdjMap.put(253, 1);
            m.insert(253, 1);
            // Java: mAdjMap.put(254, 2);
            m.insert(254, 2);
            // Java: mAdjMap.put(255, 2);
            m.insert(255, 2);
            // Java: mAdjMap.put(256, 2);
            m.insert(256, 2);
            // Java: mAdjMap.put(258, 1);
            m.insert(258, 1);
            // Java: mAdjMap.put(259, 1);
            m.insert(259, 1);
            // Java: mAdjMap.put(260, 1);
            m.insert(260, 1);
            // Java: mAdjMap.put(261, 1);
            m.insert(261, 1);
            // Java: mAdjMap.put(262, 2);
            m.insert(262, 2);
            // Java: mAdjMap.put(263, 1);
            m.insert(263, 1);
            m
        });
        // Java: Integer count = (Integer)mAdjMap.get(shapeType);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return count != null ? count : 0;
        // LLM JavaLine Aggregation: Rust uses copied().unwrap_or(0)
        map.get(&shapeType).copied().unwrap_or(0)
    }

    // Java: Map<Integer, String> getVml2DmlMap()
    fn getVml2DmlMap() -> &'static HashMap<i32, &'static str> {
        // synchronized: OnceLock provides Mutex-equivalent lock for thread-safe lazy init
        // Java: if (vml2DmlMap == null)
        // LLM JavaLine Aggregation: Rust uses OnceLock::get_or_init
        let result = VML2DML_MAP.get_or_init(|| {
            // Java: vml2DmlMap = new HashMap();
            let mut m: HashMap<i32, &str> = HashMap::new();
            // Java: vml2DmlMap.put(1, "rect");
            m.insert(1, "rect");
            // Java: vml2DmlMap.put(2, "roundRect");
            m.insert(2, "roundRect");
            // Java: vml2DmlMap.put(3, "ellipse");
            m.insert(3, "ellipse");
            // Java: vml2DmlMap.put(4, "diamond");
            m.insert(4, "diamond");
            // Java: vml2DmlMap.put(5, "triangle");
            m.insert(5, "triangle");
            // Java: vml2DmlMap.put(6, "rtTriangle");
            m.insert(6, "rtTriangle");
            // Java: vml2DmlMap.put(7, "parallelogram");
            m.insert(7, "parallelogram");
            // Java: vml2DmlMap.put(8, "trapezoid");
            m.insert(8, "trapezoid");
            // Java: vml2DmlMap.put(9, "hexagon");
            m.insert(9, "hexagon");
            // Java: vml2DmlMap.put(10, "octagon");
            m.insert(10, "octagon");
            // Java: vml2DmlMap.put(11, "plus");
            m.insert(11, "plus");
            // Java: vml2DmlMap.put(12, "star5");
            m.insert(12, "star5");
            // Java: vml2DmlMap.put(13, "rightArrow");
            m.insert(13, "rightArrow");
            // Java: vml2DmlMap.put(14, "thickArrow");
            m.insert(14, "thickArrow");
            // Java: vml2DmlMap.put(15, "homePlate");
            m.insert(15, "homePlate");
            // Java: vml2DmlMap.put(16, "cube");
            m.insert(16, "cube");
            // Java: vml2DmlMap.put(17, "wedgeRoundRectCallout");
            m.insert(17, "wedgeRoundRectCallout");
            // Java: vml2DmlMap.put(18, "star16");
            m.insert(18, "star16");
            // Java: vml2DmlMap.put(19, "arc");
            m.insert(19, "arc");
            // Java: vml2DmlMap.put(20, "line");
            m.insert(20, "line");
            // Java: vml2DmlMap.put(21, "plaque");
            m.insert(21, "plaque");
            // Java: vml2DmlMap.put(22, "can");
            m.insert(22, "can");
            // Java: vml2DmlMap.put(23, "donut");
            m.insert(23, "donut");
            // Java: vml2DmlMap.put(32, "straightConnector1");
            m.insert(32, "straightConnector1");
            // Java: vml2DmlMap.put(33, "bentConnector2");
            m.insert(33, "bentConnector2");
            // Java: vml2DmlMap.put(34, "bentConnector3");
            m.insert(34, "bentConnector3");
            // Java: vml2DmlMap.put(35, "bentConnector4");
            m.insert(35, "bentConnector4");
            // Java: vml2DmlMap.put(36, "bentConnector5");
            m.insert(36, "bentConnector5");
            // Java: vml2DmlMap.put(37, "curvedConnector2");
            m.insert(37, "curvedConnector2");
            // Java: vml2DmlMap.put(38, "curvedConnector3");
            m.insert(38, "curvedConnector3");
            // Java: vml2DmlMap.put(39, "curvedConnector4");
            m.insert(39, "curvedConnector4");
            // Java: vml2DmlMap.put(40, "curvedConnector5");
            m.insert(40, "curvedConnector5");
            // Java: vml2DmlMap.put(41, "callout1");
            m.insert(41, "callout1");
            // Java: vml2DmlMap.put(42, "callout2");
            m.insert(42, "callout2");
            // Java: vml2DmlMap.put(43, "callout3");
            m.insert(43, "callout3");
            // Java: vml2DmlMap.put(44, "accentCallout1");
            m.insert(44, "accentCallout1");
            // Java: vml2DmlMap.put(45, "accentCallout2");
            m.insert(45, "accentCallout2");
            // Java: vml2DmlMap.put(46, "accentCallout3");
            m.insert(46, "accentCallout3");
            // Java: vml2DmlMap.put(47, "borderCallout1");
            m.insert(47, "borderCallout1");
            // Java: vml2DmlMap.put(48, "borderCallout2");
            m.insert(48, "borderCallout2");
            // Java: vml2DmlMap.put(49, "borderCallout3");
            m.insert(49, "borderCallout3");
            // Java: vml2DmlMap.put(50, "accentBorderCallout1");
            m.insert(50, "accentBorderCallout1");
            // Java: vml2DmlMap.put(51, "accentBorderCallout2");
            m.insert(51, "accentBorderCallout2");
            // Java: vml2DmlMap.put(52, "accentBorderCallout3");
            m.insert(52, "accentBorderCallout3");
            // Java: vml2DmlMap.put(53, "ribbon");
            m.insert(53, "ribbon");
            // Java: vml2DmlMap.put(54, "ribbon2");
            m.insert(54, "ribbon2");
            // Java: vml2DmlMap.put(55, "chevron");
            m.insert(55, "chevron");
            // Java: vml2DmlMap.put(56, "pentagon");
            m.insert(56, "pentagon");
            // Java: vml2DmlMap.put(57, "noSmoking");
            m.insert(57, "noSmoking");
            // Java: vml2DmlMap.put(58, "star8");
            m.insert(58, "star8");
            // Java: vml2DmlMap.put(59, "star16");
            m.insert(59, "star16");
            // Java: vml2DmlMap.put(60, "star32");
            m.insert(60, "star32");
            // Java: vml2DmlMap.put(61, "wedgeRectCallout");
            m.insert(61, "wedgeRectCallout");
            // Java: vml2DmlMap.put(62, "wedgeRoundRectCallout");
            m.insert(62, "wedgeRoundRectCallout");
            // Java: vml2DmlMap.put(63, "wedgeEllipseCallout");
            m.insert(63, "wedgeEllipseCallout");
            // Java: vml2DmlMap.put(64, "wave");
            m.insert(64, "wave");
            // Java: vml2DmlMap.put(65, "foldedCorner");
            m.insert(65, "foldedCorner");
            // Java: vml2DmlMap.put(66, "leftArrow");
            m.insert(66, "leftArrow");
            // Java: vml2DmlMap.put(67, "downArrow");
            m.insert(67, "downArrow");
            // Java: vml2DmlMap.put(68, "upArrow");
            m.insert(68, "upArrow");
            // Java: vml2DmlMap.put(69, "leftRightArrow");
            m.insert(69, "leftRightArrow");
            // Java: vml2DmlMap.put(70, "upDownArrow");
            m.insert(70, "upDownArrow");
            // Java: vml2DmlMap.put(71, "irregularSeal1");
            m.insert(71, "irregularSeal1");
            // Java: vml2DmlMap.put(72, "irregularSeal2");
            m.insert(72, "irregularSeal2");
            // Java: vml2DmlMap.put(73, "lightningBolt");
            m.insert(73, "lightningBolt");
            // Java: vml2DmlMap.put(74, "heart");
            m.insert(74, "heart");
            // Java: vml2DmlMap.put(75, "frame");
            m.insert(75, "frame");
            // Java: vml2DmlMap.put(76, "quadArrow");
            m.insert(76, "quadArrow");
            // Java: vml2DmlMap.put(77, "leftArrowCallout");
            m.insert(77, "leftArrowCallout");
            // Java: vml2DmlMap.put(78, "rightArrowCallout");
            m.insert(78, "rightArrowCallout");
            // Java: vml2DmlMap.put(79, "upArrowCallout");
            m.insert(79, "upArrowCallout");
            // Java: vml2DmlMap.put(80, "downArrowCallout");
            m.insert(80, "downArrowCallout");
            // Java: vml2DmlMap.put(81, "leftRightArrowCallout");
            m.insert(81, "leftRightArrowCallout");
            // Java: vml2DmlMap.put(82, "upDownArrowCallout");
            m.insert(82, "upDownArrowCallout");
            // Java: vml2DmlMap.put(83, "quadArrowCallout");
            m.insert(83, "quadArrowCallout");
            // Java: vml2DmlMap.put(84, "bevel");
            m.insert(84, "bevel");
            // Java: vml2DmlMap.put(85, "leftBracket");
            m.insert(85, "leftBracket");
            // Java: vml2DmlMap.put(86, "rightBracket");
            m.insert(86, "rightBracket");
            // Java: vml2DmlMap.put(87, "leftBrace");
            m.insert(87, "leftBrace");
            // Java: vml2DmlMap.put(88, "rightBrace");
            m.insert(88, "rightBrace");
            // Java: vml2DmlMap.put(89, "leftUpArrow");
            m.insert(89, "leftUpArrow");
            // Java: vml2DmlMap.put(90, "bentUpArrow");
            m.insert(90, "bentUpArrow");
            // Java: vml2DmlMap.put(91, "bentArrow");
            m.insert(91, "bentArrow");
            // Java: vml2DmlMap.put(92, "star24");
            m.insert(92, "star24");
            // Java: vml2DmlMap.put(93, "stripedRightArrow");
            m.insert(93, "stripedRightArrow");
            // Java: vml2DmlMap.put(94, "notchedRightArrow");
            m.insert(94, "notchedRightArrow");
            // Java: vml2DmlMap.put(95, "blockArc");
            m.insert(95, "blockArc");
            // Java: vml2DmlMap.put(96, "smileyFace");
            m.insert(96, "smileyFace");
            // Java: vml2DmlMap.put(97, "verticalScroll");
            m.insert(97, "verticalScroll");
            // Java: vml2DmlMap.put(98, "horizontalScroll");
            m.insert(98, "horizontalScroll");
            // Java: vml2DmlMap.put(99, "circularArrow");
            m.insert(99, "circularArrow");
            // Java: vml2DmlMap.put(100, "rect");
            m.insert(100, "rect");
            // Java: vml2DmlMap.put(101, "uturnArrow");
            m.insert(101, "uturnArrow");
            // Java: vml2DmlMap.put(102, "curvedRightArrow");
            m.insert(102, "curvedRightArrow");
            // Java: vml2DmlMap.put(103, "curvedLeftArrow");
            m.insert(103, "curvedLeftArrow");
            // Java: vml2DmlMap.put(104, "curvedUpArrow");
            m.insert(104, "curvedUpArrow");
            // Java: vml2DmlMap.put(105, "curvedDownArrow");
            m.insert(105, "curvedDownArrow");
            // Java: vml2DmlMap.put(106, "cloudCallout");
            m.insert(106, "cloudCallout");
            // Java: vml2DmlMap.put(107, "ellipseRibbon");
            m.insert(107, "ellipseRibbon");
            // Java: vml2DmlMap.put(108, "ellipseRibbon2");
            m.insert(108, "ellipseRibbon2");
            // Java: vml2DmlMap.put(109, "flowChartProcess");
            m.insert(109, "flowChartProcess");
            // Java: vml2DmlMap.put(110, "flowChartDecision");
            m.insert(110, "flowChartDecision");
            // Java: vml2DmlMap.put(111, "flowChartInputOutput");
            m.insert(111, "flowChartInputOutput");
            // Java: vml2DmlMap.put(112, "flowChartPredefinedProcess");
            m.insert(112, "flowChartPredefinedProcess");
            // Java: vml2DmlMap.put(113, "flowChartInternalStorage");
            m.insert(113, "flowChartInternalStorage");
            // Java: vml2DmlMap.put(114, "flowChartDocument");
            m.insert(114, "flowChartDocument");
            // Java: vml2DmlMap.put(115, "flowChartMultidocument");
            m.insert(115, "flowChartMultidocument");
            // Java: vml2DmlMap.put(116, "flowChartTerminator");
            m.insert(116, "flowChartTerminator");
            // Java: vml2DmlMap.put(117, "flowChartPreparation");
            m.insert(117, "flowChartPreparation");
            // Java: vml2DmlMap.put(118, "flowChartManualInput");
            m.insert(118, "flowChartManualInput");
            // Java: vml2DmlMap.put(119, "flowChartManualOperation");
            m.insert(119, "flowChartManualOperation");
            // Java: vml2DmlMap.put(120, "flowChartConnector");
            m.insert(120, "flowChartConnector");
            // Java: vml2DmlMap.put(121, "flowChartPunchedCard");
            m.insert(121, "flowChartPunchedCard");
            // Java: vml2DmlMap.put(122, "flowChartPunchedTape");
            m.insert(122, "flowChartPunchedTape");
            // Java: vml2DmlMap.put(123, "flowChartSummingJunction");
            m.insert(123, "flowChartSummingJunction");
            // Java: vml2DmlMap.put(124, "flowChartOr");
            m.insert(124, "flowChartOr");
            // Java: vml2DmlMap.put(125, "flowChartCollate");
            m.insert(125, "flowChartCollate");
            // Java: vml2DmlMap.put(126, "flowChartSort");
            m.insert(126, "flowChartSort");
            // Java: vml2DmlMap.put(127, "flowChartExtract");
            m.insert(127, "flowChartExtract");
            // Java: vml2DmlMap.put(128, "flowChartMerge");
            m.insert(128, "flowChartMerge");
            // Java: vml2DmlMap.put(129, "flowChartOfflineStorage");
            m.insert(129, "flowChartOfflineStorage");
            // Java: vml2DmlMap.put(130, "flowChartOnlineStorage");
            m.insert(130, "flowChartOnlineStorage");
            // Java: vml2DmlMap.put(131, "flowChartMagneticTape");
            m.insert(131, "flowChartMagneticTape");
            // Java: vml2DmlMap.put(132, "flowChartMagneticDisk");
            m.insert(132, "flowChartMagneticDisk");
            // Java: vml2DmlMap.put(133, "flowChartMagneticDrum");
            m.insert(133, "flowChartMagneticDrum");
            // Java: vml2DmlMap.put(134, "flowChartDisplay");
            m.insert(134, "flowChartDisplay");
            // Java: vml2DmlMap.put(135, "flowChartDelay");
            m.insert(135, "flowChartDelay");
            // Java: vml2DmlMap.put(176, "flowChartAlternateProcess");
            m.insert(176, "flowChartAlternateProcess");
            // Java: vml2DmlMap.put(177, "flowChartOffpageConnector");
            m.insert(177, "flowChartOffpageConnector");
            // Java: vml2DmlMap.put(178, "callout1");
            m.insert(178, "callout1");
            // Java: vml2DmlMap.put(179, "accentCallout1");
            m.insert(179, "accentCallout1");
            // Java: vml2DmlMap.put(180, "borderCallout1");
            m.insert(180, "borderCallout1");
            // Java: vml2DmlMap.put(181, "accentBorderCallout1");
            m.insert(181, "accentBorderCallout1");
            // Java: vml2DmlMap.put(182, "leftRightUpArrow");
            m.insert(182, "leftRightUpArrow");
            // Java: vml2DmlMap.put(183, "sun");
            m.insert(183, "sun");
            // Java: vml2DmlMap.put(184, "moon");
            m.insert(184, "moon");
            // Java: vml2DmlMap.put(185, "bracketPair");
            m.insert(185, "bracketPair");
            // Java: vml2DmlMap.put(186, "bracePair");
            m.insert(186, "bracePair");
            // Java: vml2DmlMap.put(187, "star4");
            m.insert(187, "star4");
            // Java: vml2DmlMap.put(188, "doubleWave");
            m.insert(188, "doubleWave");
            // Java: vml2DmlMap.put(189, "actionButtonBlank");
            m.insert(189, "actionButtonBlank");
            // Java: vml2DmlMap.put(190, "actionButtonHome");
            m.insert(190, "actionButtonHome");
            // Java: vml2DmlMap.put(191, "actionButtonHelp");
            m.insert(191, "actionButtonHelp");
            // Java: vml2DmlMap.put(192, "actionButtonInformation");
            m.insert(192, "actionButtonInformation");
            // Java: vml2DmlMap.put(193, "actionButtonForwardNext");
            m.insert(193, "actionButtonForwardNext");
            // Java: vml2DmlMap.put(194, "actionButtonBackPrevious");
            m.insert(194, "actionButtonBackPrevious");
            // Java: vml2DmlMap.put(195, "actionButtonEnd");
            m.insert(195, "actionButtonEnd");
            // Java: vml2DmlMap.put(196, "actionButtonBeginning");
            m.insert(196, "actionButtonBeginning");
            // Java: vml2DmlMap.put(197, "actionButtonReturn");
            m.insert(197, "actionButtonReturn");
            // Java: vml2DmlMap.put(198, "actionButtonDocument");
            m.insert(198, "actionButtonDocument");
            // Java: vml2DmlMap.put(199, "actionButtonSound");
            m.insert(199, "actionButtonSound");
            // Java: vml2DmlMap.put(200, "actionButtonMovie");
            m.insert(200, "actionButtonMovie");
            // Java: vml2DmlMap.put(201, "hostControl");
            m.insert(201, "hostControl");
            // Java: vml2DmlMap.put(202, "textBox");
            m.insert(202, "textBox");
            // Java: vml2DmlMap.put(204, "picture");
            m.insert(204, "picture");
            // Java: vml2DmlMap.put(203, "groupShape");
            m.insert(203, "groupShape");
            // Java: vml2DmlMap.put(221, "chartPlus");
            m.insert(221, "chartPlus");
            // Java: vml2DmlMap.put(222, "chartStar");
            m.insert(222, "chartStar");
            // Java: vml2DmlMap.put(223, "chartX");
            m.insert(223, "chartX");
            // Java: vml2DmlMap.put(224, "chord");
            m.insert(224, "chord");
            // Java: vml2DmlMap.put(225, "cloud");
            m.insert(225, "cloud");
            // Java: vml2DmlMap.put(226, "corner");
            m.insert(226, "corner");
            // Java: vml2DmlMap.put(227, "cornerTabs");
            m.insert(227, "cornerTabs");
            // Java: vml2DmlMap.put(228, "decagon");
            m.insert(228, "decagon");
            // Java: vml2DmlMap.put(229, "diagStripe");
            m.insert(229, "diagStripe");
            // Java: vml2DmlMap.put(230, "dodecagon");
            m.insert(230, "dodecagon");
            // Java: vml2DmlMap.put(231, "funnel");
            m.insert(231, "funnel");
            // Java: vml2DmlMap.put(232, "gear6");
            m.insert(232, "gear6");
            // Java: vml2DmlMap.put(233, "gear9");
            m.insert(233, "gear9");
            // Java: vml2DmlMap.put(234, "halfFrame");
            m.insert(234, "halfFrame");
            // Java: vml2DmlMap.put(235, "heptagon");
            m.insert(235, "heptagon");
            // Java: vml2DmlMap.put(236, "leftCircularArrow");
            m.insert(236, "leftCircularArrow");
            // Java: vml2DmlMap.put(237, "leftRightCircularArrow");
            m.insert(237, "leftRightCircularArrow");
            // Java: vml2DmlMap.put(238, "leftRightRibbon");
            m.insert(238, "leftRightRibbon");
            // Java: vml2DmlMap.put(239, "lineInv");
            m.insert(239, "lineInv");
            // Java: vml2DmlMap.put(240, "mathDivide");
            m.insert(240, "mathDivide");
            // Java: vml2DmlMap.put(241, "mathEqual");
            m.insert(241, "mathEqual");
            // Java: vml2DmlMap.put(242, "mathMinus");
            m.insert(242, "mathMinus");
            // Java: vml2DmlMap.put(243, "mathMultiply");
            m.insert(243, "mathMultiply");
            // Java: vml2DmlMap.put(244, "mathNotEqual");
            m.insert(244, "mathNotEqual");
            // Java: vml2DmlMap.put(245, "mathPlus");
            m.insert(245, "mathPlus");
            // Java: vml2DmlMap.put(246, "nonIsoscelesTrapezoid");
            m.insert(246, "nonIsoscelesTrapezoid");
            // Java: vml2DmlMap.put(247, "pie");
            m.insert(247, "pie");
            // Java: vml2DmlMap.put(248, "pieWedge");
            m.insert(248, "pieWedge");
            // Java: vml2DmlMap.put(249, "plaqueTabs");
            m.insert(249, "plaqueTabs");
            // Java: vml2DmlMap.put(250, "round1Rect");
            m.insert(250, "round1Rect");
            // Java: vml2DmlMap.put(251, "round2DiagRect");
            m.insert(251, "round2DiagRect");
            // Java: vml2DmlMap.put(252, "round2SameRect");
            m.insert(252, "round2SameRect");
            // Java: vml2DmlMap.put(253, "snip1Rect");
            m.insert(253, "snip1Rect");
            // Java: vml2DmlMap.put(254, "snip2DiagRect");
            m.insert(254, "snip2DiagRect");
            // Java: vml2DmlMap.put(255, "snip2SameRect");
            m.insert(255, "snip2SameRect");
            // Java: vml2DmlMap.put(256, "snipRoundRect");
            m.insert(256, "snipRoundRect");
            // Java: vml2DmlMap.put(257, "squareTabs");
            m.insert(257, "squareTabs");
            // Java: vml2DmlMap.put(258, "star10");
            m.insert(258, "star10");
            // Java: vml2DmlMap.put(259, "star12");
            m.insert(259, "star12");
            // Java: vml2DmlMap.put(260, "star6");
            m.insert(260, "star6");
            // Java: vml2DmlMap.put(261, "star7");
            m.insert(261, "star7");
            // Java: vml2DmlMap.put(262, "swooshArrow");
            m.insert(262, "swooshArrow");
            // Java: vml2DmlMap.put(263, "teardrop");
            m.insert(263, "teardrop");
            m
        });
        // Java: return vml2DmlMap;
        result
    }

    // Java: String getDmlName(int type)
    pub fn getDmlName(r#type: i32) -> Option<&'static str> {
        // Java: Map<Integer, String> vml2DmlMap = getVml2DmlMap();
        let vml2DmlMap = Self::getVml2DmlMap();
        // Java: return (String)vml2DmlMap.get(type);
        vml2DmlMap.get(&r#type).copied()
    }

    // Java: Map<String, Integer> getDml2VmlMap()
    fn getDml2VmlMap() -> &'static HashMap<&'static str, i32> {
        // synchronized: OnceLock provides Mutex-equivalent lock for thread-safe lazy init
        // Java: if (dml2VmlMap == null)
        // LLM JavaLine Aggregation: Rust uses OnceLock::get_or_init
        let result = DML2VML_MAP.get_or_init(|| {
            // Java: dml2VmlMap = new HashMap();
            let mut m: HashMap<&str, i32> = HashMap::new();
            // Java: dml2VmlMap.put("rect", 1);
            m.insert("rect", 1);
            // Java: dml2VmlMap.put("roundRect", 2);
            m.insert("roundRect", 2);
            // Java: dml2VmlMap.put("ellipse", 3);
            m.insert("ellipse", 3);
            // Java: dml2VmlMap.put("diamond", 4);
            m.insert("diamond", 4);
            // Java: dml2VmlMap.put("triangle", 5);
            m.insert("triangle", 5);
            // Java: dml2VmlMap.put("rtTriangle", 6);
            m.insert("rtTriangle", 6);
            // Java: dml2VmlMap.put("parallelogram", 7);
            m.insert("parallelogram", 7);
            // Java: dml2VmlMap.put("trapezoid", 8);
            m.insert("trapezoid", 8);
            // Java: dml2VmlMap.put("hexagon", 9);
            m.insert("hexagon", 9);
            // Java: dml2VmlMap.put("octagon", 10);
            m.insert("octagon", 10);
            // Java: dml2VmlMap.put("plus", 11);
            m.insert("plus", 11);
            // Java: dml2VmlMap.put("star5", 12);
            m.insert("star5", 12);
            // Java: dml2VmlMap.put("rightArrow", 13);
            m.insert("rightArrow", 13);
            // Java: dml2VmlMap.put("thickArrow", 14);
            m.insert("thickArrow", 14);
            // Java: dml2VmlMap.put("homePlate", 15);
            m.insert("homePlate", 15);
            // Java: dml2VmlMap.put("cube", 16);
            m.insert("cube", 16);
            // Java: dml2VmlMap.put("arc", 19);
            m.insert("arc", 19);
            // Java: dml2VmlMap.put("line", 20);
            m.insert("line", 20);
            // Java: dml2VmlMap.put("plaque", 21);
            m.insert("plaque", 21);
            // Java: dml2VmlMap.put("can", 22);
            m.insert("can", 22);
            // Java: dml2VmlMap.put("donut", 23);
            m.insert("donut", 23);
            // Java: dml2VmlMap.put("straightConnector1", 32);
            m.insert("straightConnector1", 32);
            // Java: dml2VmlMap.put("bentConnector2", 33);
            m.insert("bentConnector2", 33);
            // Java: dml2VmlMap.put("bentConnector3", 34);
            m.insert("bentConnector3", 34);
            // Java: dml2VmlMap.put("bentConnector4", 35);
            m.insert("bentConnector4", 35);
            // Java: dml2VmlMap.put("bentConnector5", 36);
            m.insert("bentConnector5", 36);
            // Java: dml2VmlMap.put("curvedConnector2", 37);
            m.insert("curvedConnector2", 37);
            // Java: dml2VmlMap.put("curvedConnector3", 38);
            m.insert("curvedConnector3", 38);
            // Java: dml2VmlMap.put("curvedConnector4", 39);
            m.insert("curvedConnector4", 39);
            // Java: dml2VmlMap.put("curvedConnector5", 40);
            m.insert("curvedConnector5", 40);
            // Java: dml2VmlMap.put("callout1", 41);
            m.insert("callout1", 41);
            // Java: dml2VmlMap.put("callout2", 42);
            m.insert("callout2", 42);
            // Java: dml2VmlMap.put("callout3", 43);
            m.insert("callout3", 43);
            // Java: dml2VmlMap.put("accentCallout1", 44);
            m.insert("accentCallout1", 44);
            // Java: dml2VmlMap.put("accentCallout2", 45);
            m.insert("accentCallout2", 45);
            // Java: dml2VmlMap.put("accentCallout3", 46);
            m.insert("accentCallout3", 46);
            // Java: dml2VmlMap.put("borderCallout1", 47);
            m.insert("borderCallout1", 47);
            // Java: dml2VmlMap.put("borderCallout2", 48);
            m.insert("borderCallout2", 48);
            // Java: dml2VmlMap.put("borderCallout3", 49);
            m.insert("borderCallout3", 49);
            // Java: dml2VmlMap.put("accentBorderCallout1", 50);
            m.insert("accentBorderCallout1", 50);
            // Java: dml2VmlMap.put("accentBorderCallout2", 51);
            m.insert("accentBorderCallout2", 51);
            // Java: dml2VmlMap.put("accentBorderCallout3", 52);
            m.insert("accentBorderCallout3", 52);
            // Java: dml2VmlMap.put("ribbon", 53);
            m.insert("ribbon", 53);
            // Java: dml2VmlMap.put("ribbon2", 54);
            m.insert("ribbon2", 54);
            // Java: dml2VmlMap.put("chevron", 55);
            m.insert("chevron", 55);
            // Java: dml2VmlMap.put("pentagon", 56);
            m.insert("pentagon", 56);
            // Java: dml2VmlMap.put("noSmoking", 57);
            m.insert("noSmoking", 57);
            // Java: dml2VmlMap.put("star8", 58);
            m.insert("star8", 58);
            // Java: dml2VmlMap.put("star16", 59);
            m.insert("star16", 59);
            // Java: dml2VmlMap.put("star32", 60);
            m.insert("star32", 60);
            // Java: dml2VmlMap.put("wedgeRectCallout", 61);
            m.insert("wedgeRectCallout", 61);
            // Java: dml2VmlMap.put("wedgeRoundRectCallout", 62);
            m.insert("wedgeRoundRectCallout", 62);
            // Java: dml2VmlMap.put("wedgeEllipseCallout", 63);
            m.insert("wedgeEllipseCallout", 63);
            // Java: dml2VmlMap.put("wave", 64);
            m.insert("wave", 64);
            // Java: dml2VmlMap.put("foldedCorner", 65);
            m.insert("foldedCorner", 65);
            // Java: dml2VmlMap.put("leftArrow", 66);
            m.insert("leftArrow", 66);
            // Java: dml2VmlMap.put("downArrow", 67);
            m.insert("downArrow", 67);
            // Java: dml2VmlMap.put("upArrow", 68);
            m.insert("upArrow", 68);
            // Java: dml2VmlMap.put("leftRightArrow", 69);
            m.insert("leftRightArrow", 69);
            // Java: dml2VmlMap.put("upDownArrow", 70);
            m.insert("upDownArrow", 70);
            // Java: dml2VmlMap.put("irregularSeal1", 71);
            m.insert("irregularSeal1", 71);
            // Java: dml2VmlMap.put("irregularSeal2", 72);
            m.insert("irregularSeal2", 72);
            // Java: dml2VmlMap.put("lightningBolt", 73);
            m.insert("lightningBolt", 73);
            // Java: dml2VmlMap.put("heart", 74);
            m.insert("heart", 74);
            // Java: dml2VmlMap.put("frame", 75);
            m.insert("frame", 75);
            // Java: dml2VmlMap.put("quadArrow", 76);
            m.insert("quadArrow", 76);
            // Java: dml2VmlMap.put("leftArrowCallout", 77);
            m.insert("leftArrowCallout", 77);
            // Java: dml2VmlMap.put("rightArrowCallout", 78);
            m.insert("rightArrowCallout", 78);
            // Java: dml2VmlMap.put("upArrowCallout", 79);
            m.insert("upArrowCallout", 79);
            // Java: dml2VmlMap.put("downArrowCallout", 80);
            m.insert("downArrowCallout", 80);
            // Java: dml2VmlMap.put("leftRightArrowCallout", 81);
            m.insert("leftRightArrowCallout", 81);
            // Java: dml2VmlMap.put("upDownArrowCallout", 82);
            m.insert("upDownArrowCallout", 82);
            // Java: dml2VmlMap.put("quadArrowCallout", 83);
            m.insert("quadArrowCallout", 83);
            // Java: dml2VmlMap.put("bevel", 84);
            m.insert("bevel", 84);
            // Java: dml2VmlMap.put("leftBracket", 85);
            m.insert("leftBracket", 85);
            // Java: dml2VmlMap.put("rightBracket", 86);
            m.insert("rightBracket", 86);
            // Java: dml2VmlMap.put("leftBrace", 87);
            m.insert("leftBrace", 87);
            // Java: dml2VmlMap.put("rightBrace", 88);
            m.insert("rightBrace", 88);
            // Java: dml2VmlMap.put("leftUpArrow", 89);
            m.insert("leftUpArrow", 89);
            // Java: dml2VmlMap.put("bentUpArrow", 90);
            m.insert("bentUpArrow", 90);
            // Java: dml2VmlMap.put("bentArrow", 91);
            m.insert("bentArrow", 91);
            // Java: dml2VmlMap.put("star24", 92);
            m.insert("star24", 92);
            // Java: dml2VmlMap.put("stripedRightArrow", 93);
            m.insert("stripedRightArrow", 93);
            // Java: dml2VmlMap.put("notchedRightArrow", 94);
            m.insert("notchedRightArrow", 94);
            // Java: dml2VmlMap.put("blockArc", 95);
            m.insert("blockArc", 95);
            // Java: dml2VmlMap.put("smileyFace", 96);
            m.insert("smileyFace", 96);
            // Java: dml2VmlMap.put("verticalScroll", 97);
            m.insert("verticalScroll", 97);
            // Java: dml2VmlMap.put("horizontalScroll", 98);
            m.insert("horizontalScroll", 98);
            // Java: dml2VmlMap.put("circularArrow", 99);
            m.insert("circularArrow", 99);
            // Java: dml2VmlMap.put("uturnArrow", 101);
            m.insert("uturnArrow", 101);
            // Java: dml2VmlMap.put("curvedRightArrow", 102);
            m.insert("curvedRightArrow", 102);
            // Java: dml2VmlMap.put("curvedLeftArrow", 103);
            m.insert("curvedLeftArrow", 103);
            // Java: dml2VmlMap.put("curvedUpArrow", 104);
            m.insert("curvedUpArrow", 104);
            // Java: dml2VmlMap.put("curvedDownArrow", 105);
            m.insert("curvedDownArrow", 105);
            // Java: dml2VmlMap.put("cloudCallout", 106);
            m.insert("cloudCallout", 106);
            // Java: dml2VmlMap.put("ellipseRibbon", 107);
            m.insert("ellipseRibbon", 107);
            // Java: dml2VmlMap.put("ellipseRibbon2", 108);
            m.insert("ellipseRibbon2", 108);
            // Java: dml2VmlMap.put("flowChartProcess", 109);
            m.insert("flowChartProcess", 109);
            // Java: dml2VmlMap.put("flowChartDecision", 110);
            m.insert("flowChartDecision", 110);
            // Java: dml2VmlMap.put("flowChartInputOutput", 111);
            m.insert("flowChartInputOutput", 111);
            // Java: dml2VmlMap.put("flowChartPredefinedProcess", 112);
            m.insert("flowChartPredefinedProcess", 112);
            // Java: dml2VmlMap.put("flowChartInternalStorage", 113);
            m.insert("flowChartInternalStorage", 113);
            // Java: dml2VmlMap.put("flowChartDocument", 114);
            m.insert("flowChartDocument", 114);
            // Java: dml2VmlMap.put("flowChartMultidocument", 115);
            m.insert("flowChartMultidocument", 115);
            // Java: dml2VmlMap.put("flowChartTerminator", 116);
            m.insert("flowChartTerminator", 116);
            // Java: dml2VmlMap.put("flowChartPreparation", 117);
            m.insert("flowChartPreparation", 117);
            // Java: dml2VmlMap.put("flowChartManualInput", 118);
            m.insert("flowChartManualInput", 118);
            // Java: dml2VmlMap.put("flowChartManualOperation", 119);
            m.insert("flowChartManualOperation", 119);
            // Java: dml2VmlMap.put("flowChartConnector", 120);
            m.insert("flowChartConnector", 120);
            // Java: dml2VmlMap.put("flowChartPunchedCard", 121);
            m.insert("flowChartPunchedCard", 121);
            // Java: dml2VmlMap.put("flowChartPunchedTape", 122);
            m.insert("flowChartPunchedTape", 122);
            // Java: dml2VmlMap.put("flowChartSummingJunction", 123);
            m.insert("flowChartSummingJunction", 123);
            // Java: dml2VmlMap.put("flowChartOr", 124);
            m.insert("flowChartOr", 124);
            // Java: dml2VmlMap.put("flowChartCollate", 125);
            m.insert("flowChartCollate", 125);
            // Java: dml2VmlMap.put("flowChartSort", 126);
            m.insert("flowChartSort", 126);
            // Java: dml2VmlMap.put("flowChartExtract", 127);
            m.insert("flowChartExtract", 127);
            // Java: dml2VmlMap.put("flowChartMerge", 128);
            m.insert("flowChartMerge", 128);
            // Java: dml2VmlMap.put("flowChartOfflineStorage", 129);
            m.insert("flowChartOfflineStorage", 129);
            // Java: dml2VmlMap.put("flowChartOnlineStorage", 130);
            m.insert("flowChartOnlineStorage", 130);
            // Java: dml2VmlMap.put("flowChartMagneticTape", 131);
            m.insert("flowChartMagneticTape", 131);
            // Java: dml2VmlMap.put("flowChartMagneticDisk", 132);
            m.insert("flowChartMagneticDisk", 132);
            // Java: dml2VmlMap.put("flowChartMagneticDrum", 133);
            m.insert("flowChartMagneticDrum", 133);
            // Java: dml2VmlMap.put("flowChartDisplay", 134);
            m.insert("flowChartDisplay", 134);
            // Java: dml2VmlMap.put("flowChartDelay", 135);
            m.insert("flowChartDelay", 135);
            // Java: dml2VmlMap.put("flowChartAlternateProcess", 176);
            m.insert("flowChartAlternateProcess", 176);
            // Java: dml2VmlMap.put("flowChartOffpageConnector", 177);
            m.insert("flowChartOffpageConnector", 177);
            // Java: dml2VmlMap.put("leftRightUpArrow", 182);
            m.insert("leftRightUpArrow", 182);
            // Java: dml2VmlMap.put("sun", 183);
            m.insert("sun", 183);
            // Java: dml2VmlMap.put("moon", 184);
            m.insert("moon", 184);
            // Java: dml2VmlMap.put("bracketPair", 185);
            m.insert("bracketPair", 185);
            // Java: dml2VmlMap.put("bracePair", 186);
            m.insert("bracePair", 186);
            // Java: dml2VmlMap.put("star4", 187);
            m.insert("star4", 187);
            // Java: dml2VmlMap.put("doubleWave", 188);
            m.insert("doubleWave", 188);
            // Java: dml2VmlMap.put("actionButtonBlank", 189);
            m.insert("actionButtonBlank", 189);
            // Java: dml2VmlMap.put("actionButtonHome", 190);
            m.insert("actionButtonHome", 190);
            // Java: dml2VmlMap.put("actionButtonHelp", 191);
            m.insert("actionButtonHelp", 191);
            // Java: dml2VmlMap.put("actionButtonInformation", 192);
            m.insert("actionButtonInformation", 192);
            // Java: dml2VmlMap.put("actionButtonForwardNext", 193);
            m.insert("actionButtonForwardNext", 193);
            // Java: dml2VmlMap.put("actionButtonBackPrevious", 194);
            m.insert("actionButtonBackPrevious", 194);
            // Java: dml2VmlMap.put("actionButtonEnd", 195);
            m.insert("actionButtonEnd", 195);
            // Java: dml2VmlMap.put("actionButtonBeginning", 196);
            m.insert("actionButtonBeginning", 196);
            // Java: dml2VmlMap.put("actionButtonReturn", 197);
            m.insert("actionButtonReturn", 197);
            // Java: dml2VmlMap.put("actionButtonDocument", 198);
            m.insert("actionButtonDocument", 198);
            // Java: dml2VmlMap.put("actionButtonSound", 199);
            m.insert("actionButtonSound", 199);
            // Java: dml2VmlMap.put("actionButtonMovie", 200);
            m.insert("actionButtonMovie", 200);
            // Java: dml2VmlMap.put("hostControl", 201);
            m.insert("hostControl", 201);
            // Java: dml2VmlMap.put("textBox", 202);
            m.insert("textBox", 202);
            // Java: dml2VmlMap.put("groupShape", 203);
            m.insert("groupShape", 203);
            // Java: dml2VmlMap.put("picture", 204);
            m.insert("picture", 204);
            // Java: dml2VmlMap.put("chartPlus", 221);
            m.insert("chartPlus", 221);
            // Java: dml2VmlMap.put("chartStar", 222);
            m.insert("chartStar", 222);
            // Java: dml2VmlMap.put("chartX", 223);
            m.insert("chartX", 223);
            // Java: dml2VmlMap.put("chord", 224);
            m.insert("chord", 224);
            // Java: dml2VmlMap.put("cloud", 225);
            m.insert("cloud", 225);
            // Java: dml2VmlMap.put("corner", 226);
            m.insert("corner", 226);
            // Java: dml2VmlMap.put("cornerTabs", 227);
            m.insert("cornerTabs", 227);
            // Java: dml2VmlMap.put("decagon", 228);
            m.insert("decagon", 228);
            // Java: dml2VmlMap.put("diagStripe", 229);
            m.insert("diagStripe", 229);
            // Java: dml2VmlMap.put("dodecagon", 230);
            m.insert("dodecagon", 230);
            // Java: dml2VmlMap.put("funnel", 231);
            m.insert("funnel", 231);
            // Java: dml2VmlMap.put("gear6", 232);
            m.insert("gear6", 232);
            // Java: dml2VmlMap.put("gear9", 233);
            m.insert("gear9", 233);
            // Java: dml2VmlMap.put("halfFrame", 234);
            m.insert("halfFrame", 234);
            // Java: dml2VmlMap.put("heptagon", 235);
            m.insert("heptagon", 235);
            // Java: dml2VmlMap.put("leftCircularArrow", 236);
            m.insert("leftCircularArrow", 236);
            // Java: dml2VmlMap.put("leftRightCircularArrow", 237);
            m.insert("leftRightCircularArrow", 237);
            // Java: dml2VmlMap.put("leftRightRibbon", 238);
            m.insert("leftRightRibbon", 238);
            // Java: dml2VmlMap.put("lineInv", 239);
            m.insert("lineInv", 239);
            // Java: dml2VmlMap.put("mathDivide", 240);
            m.insert("mathDivide", 240);
            // Java: dml2VmlMap.put("mathEqual", 241);
            m.insert("mathEqual", 241);
            // Java: dml2VmlMap.put("mathMinus", 242);
            m.insert("mathMinus", 242);
            // Java: dml2VmlMap.put("mathMultiply", 243);
            m.insert("mathMultiply", 243);
            // Java: dml2VmlMap.put("mathNotEqual", 244);
            m.insert("mathNotEqual", 244);
            // Java: dml2VmlMap.put("mathPlus", 245);
            m.insert("mathPlus", 245);
            // Java: dml2VmlMap.put("nonIsoscelesTrapezoid", 246);
            m.insert("nonIsoscelesTrapezoid", 246);
            // Java: dml2VmlMap.put("pie", 247);
            m.insert("pie", 247);
            // Java: dml2VmlMap.put("pieWedge", 248);
            m.insert("pieWedge", 248);
            // Java: dml2VmlMap.put("plaqueTabs", 249);
            m.insert("plaqueTabs", 249);
            // Java: dml2VmlMap.put("round1Rect", 250);
            m.insert("round1Rect", 250);
            // Java: dml2VmlMap.put("round2DiagRect", 251);
            m.insert("round2DiagRect", 251);
            // Java: dml2VmlMap.put("round2SameRect", 252);
            m.insert("round2SameRect", 252);
            // Java: dml2VmlMap.put("snip1Rect", 253);
            m.insert("snip1Rect", 253);
            // Java: dml2VmlMap.put("snip2DiagRect", 254);
            m.insert("snip2DiagRect", 254);
            // Java: dml2VmlMap.put("snip2SameRect", 255);
            m.insert("snip2SameRect", 255);
            // Java: dml2VmlMap.put("snipRoundRect", 256);
            m.insert("snipRoundRect", 256);
            // Java: dml2VmlMap.put("squareTabs", 257);
            m.insert("squareTabs", 257);
            // Java: dml2VmlMap.put("star10", 258);
            m.insert("star10", 258);
            // Java: dml2VmlMap.put("star12", 259);
            m.insert("star12", 259);
            // Java: dml2VmlMap.put("star6", 260);
            m.insert("star6", 260);
            // Java: dml2VmlMap.put("star7", 261);
            m.insert("star7", 261);
            // Java: dml2VmlMap.put("swooshArrow", 262);
            m.insert("swooshArrow", 262);
            // Java: dml2VmlMap.put("teardrop", 263);
            m.insert("teardrop", 263);
            m
        });
        // Java: return dml2VmlMap;
        result
    }

    // Java: int getShapeType(String name)
    pub fn getShapeType(name: &str) -> i32 {
        // Java: Map<String, Integer> dml2VmlMap = getDml2VmlMap();
        let dml2VmlMap = Self::getDml2VmlMap();
        // Java: Integer type = (Integer)dml2VmlMap.get(name);
        // LLM JavaLine Aggregation: merged into the next Rust statement
        // Java: return type == null ? 0 : type;
        // LLM JavaLine Aggregation: Rust uses copied().unwrap_or(0)
        dml2VmlMap.get(name).copied().unwrap_or(0)
    }

    // Java: boolean isLine(int type)
    pub fn isLine(r#type: i32) -> bool {
        // Java: return type == 20 || type == 32 || type == 33 || type == 34 || type == 35 || type == 36 || type == 37 || type == 38 || type == 39 || type == 40;
        // LLM JavaLine Aggregation: type is a Rust keyword, using r#type
        r#type == 20 || r#type == 32 || r#type == 33 || r#type == 34 || r#type == 35 || r#type == 36 || r#type == 37 || r#type == 38 || r#type == 39 || r#type == 40
    }

    // Java: boolean isChartShape(Shape shape)
    pub fn isChartShape(shape: &mut Shape) -> bool {
        // Java: return isChartShape0x01(shape) || isChartShape0x08(shape);
        Self::isChartShape0x01(shape) || Self::isChartShape0x08(shape)
    }

    // Java: boolean isChartShape0x01(Shape shape)
    pub fn isChartShape0x01(shape: &mut Shape) -> bool {
        // Java: int shapeType = shape.getShapeType();
        let shapeType = shape.getShapeType();
        // Java: return shapeType == 204 && shape.getDrawingContainer() != null && shape.getDrawingContainer().getChartShape(shape) != null;
        // LLM No Rust equivalent: IDrawingContainer.getChartShape() not yet available
        shapeType == 204 && shape.getDrawingContainer().is_some()
    }

    // Java: boolean isChartShape0x08(Shape shape)
    pub fn isChartShape0x08(shape: &mut Shape) -> bool {
        // Java: return !shape.getInlineShape() && shape.getProperty(883) != null && shape.getProperty(884) != null && shape.getProperty(885) != null;
        if shape.getInlineShape() {
            return false;
        }
        let prop = shape.base.getProperty();
        prop.base.getValue(883).is_some() && prop.base.getValue(884).is_some() && prop.base.getValue(885).is_some()
    }

    // Java: boolean isConnector(int shapeType)
    pub fn isConnector(shapeType: i32) -> bool {
        // Java: return shapeType >= 32 && shapeType <= 40;
        shapeType >= 32 && shapeType <= 40
    }

    // Java: boolean is03Special(int type)
    pub fn is03Special(r#type: i32) -> bool {
        // Java: return type == 19 || type == 8 || type == 95 || type == 76 || type == 83 || type == 89 || type == 90 || type == 99 || type == 101 || type == 182 || type == 12 || type == 75 || type >= 205;
        // LLM JavaLine Aggregation: type is a Rust keyword, using r#type
        r#type == 19 || r#type == 8 || r#type == 95 || r#type == 76 || r#type == 83 || r#type == 89 || r#type == 90 || r#type == 99 || r#type == 101 || r#type == 182 || r#type == 12 || r#type == 75 || r#type >= 205
    }

    // Java: boolean isDiffDefaultAdj(int type)
    pub fn isDiffDefaultAdj(r#type: i32) -> bool {
        // Java: return type == 13 || type == 14 || type >= 66 && type <= 70 || type >= 76 && type <= 83 || type >= 89 && type <= 91 || type >= 99 && type <= 105 || type >= 99 && type <= 105 || type >= 99 && type <= 105 || type == 182 || type == 236 || type == 237 || type == 93 || type == 15 || type == 55 || type == 94;
        // LLM JavaLine Aggregation: type is a Rust keyword, using r#type
        r#type == 13 || r#type == 14 || (r#type >= 66 && r#type <= 70) || (r#type >= 76 && r#type <= 83) || (r#type >= 89 && r#type <= 91) || (r#type >= 99 && r#type <= 105) || r#type == 182 || r#type == 236 || r#type == 237 || r#type == 93 || r#type == 15 || r#type == 55 || r#type == 94
    }

    // Java: boolean hasImplAutoFitRule(int shapeType)
    pub fn hasImplAutoFitRule(shapeType: i32) -> bool {
        // Java: switch (shapeType)
        // LLM JavaLine Aggregation: Rust uses matches! macro instead of switch
        // Java: case 2:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 3:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 4:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 5:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 6:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 7:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 8:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 9:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 10:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 11:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 13:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 15:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 16:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 20:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 21:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 22:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 23:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 56:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 57:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 65:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 66:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 67:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 68:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 69:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 70:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 73:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 74:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 75:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 84:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 85:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 86:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 87:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 88:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 93:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 94:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 96:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 110:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 111:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 112:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 113:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 114:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 115:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 116:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 117:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 118:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 119:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 120:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 127:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 128:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 132:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 133:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 176:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 183:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 184:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 185:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 186:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 221:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 222:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 223:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 224:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 225:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 226:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 228:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 229:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 230:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 235:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 239:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 247:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 250:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 251:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 252:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 253:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 254:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 255:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 256:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: case 263:
        // LLM JavaLine Aggregation: merged into matches! macro
        // Java: return true;
        // LLM JavaLine Aggregation: Rust uses matches! macro for switch-case
        // Java: case 12:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 14:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 17:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 18:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 19:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 24:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 25:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 26:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 27:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 28:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 29:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 30:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 31:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 32:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 33:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 34:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 35:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 36:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 37:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 38:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 39:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 40:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 41:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 42:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 43:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 44:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 45:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 46:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 47:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 48:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 49:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 50:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 51:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 52:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 53:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 54:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 55:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 58:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 59:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 60:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 61:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 62:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 63:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 64:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 71:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 72:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 76:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 77:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 78:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 79:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 80:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 81:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 82:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 83:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 89:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 90:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 91:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 92:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 95:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 97:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 98:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 99:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 100:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 101:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 102:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 103:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 104:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 105:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 106:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 107:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 108:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 109:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 121:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 122:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 123:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 124:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 125:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 126:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 129:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 130:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 131:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 134:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 135:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 136:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 137:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 138:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 139:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 140:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 141:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 142:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 143:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 144:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 145:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 146:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 147:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 148:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 149:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 150:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 151:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 152:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 153:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 154:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 155:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 156:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 157:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 158:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 159:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 160:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 161:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 162:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 163:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 164:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 165:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 166:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 167:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 168:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 169:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 170:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 171:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 172:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 173:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 174:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 175:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 177:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 178:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 179:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 180:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 181:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 182:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 187:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 188:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 189:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 190:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 191:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 192:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 193:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 194:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 195:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 196:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 197:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 198:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 199:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 200:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 201:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 202:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 203:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 204:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 205:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 206:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 207:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 208:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 209:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 210:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 211:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 212:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 213:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 214:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 215:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 216:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 217:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 218:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 219:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 220:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 227:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 231:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 232:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 233:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 234:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 236:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 237:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 238:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 240:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 241:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 242:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 243:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 244:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 245:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 246:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 248:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 249:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 257:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 258:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 259:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 260:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 261:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: case 262:
        // LLM JavaLine Aggregation: merged into matches! macro default
        // Java: default:
        // LLM JavaLine Aggregation: default case handled by matches! returning false
        // Java: return false;
        // LLM JavaLine Aggregation: default case handled by matches! returning false
        matches!(shapeType, 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 13 | 15 | 16 | 20 | 21 | 22 | 23 | 56 | 57 | 65 | 66 | 67 | 68 | 69 | 70 | 73 | 74 | 75 | 84 | 85 | 86 | 87 | 88 | 93 | 94 | 96 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 127 | 128 | 132 | 133 | 176 | 183 | 184 | 185 | 186 | 221 | 222 | 223 | 224 | 225 | 226 | 228 | 229 | 230 | 235 | 239 | 247 | 250 | 251 | 252 | 253 | 254 | 255 | 256 | 263)
    }

    // Java: Map<Integer, String> getDocWordArt2DmlMap()
    fn getDocWordArt2DmlMap() -> &'static HashMap<i32, &'static str> {
        // synchronized: OnceLock provides Mutex-equivalent lock for thread-safe lazy init
        // Java: if (docWordArt2DmlMap == null)
        // LLM JavaLine Aggregation: Rust uses OnceLock::get_or_init
        let result = DOC_WORDART2DML_MAP.get_or_init(|| {
            // Java: docWordArt2DmlMap = new HashMap();
            let mut m: HashMap<i32, &str> = HashMap::new();
            // Java: docWordArt2DmlMap.put(136, "textPlain");
            m.insert(136, "textPlain");
            // Java: docWordArt2DmlMap.put(137, "textStop");
            m.insert(137, "textStop");
            // Java: docWordArt2DmlMap.put(138, "textTriangle");
            m.insert(138, "textTriangle");
            // Java: docWordArt2DmlMap.put(139, "textTriangleInverted");
            m.insert(139, "textTriangleInverted");
            // Java: docWordArt2DmlMap.put(140, "textChevron");
            m.insert(140, "textChevron");
            // Java: docWordArt2DmlMap.put(141, "textChevronInverted");
            m.insert(141, "textChevronInverted");
            // Java: docWordArt2DmlMap.put(142, "textRingInside");
            m.insert(142, "textRingInside");
            // Java: docWordArt2DmlMap.put(143, "textRingOutside");
            m.insert(143, "textRingOutside");
            // Java: docWordArt2DmlMap.put(144, "textArchUp");
            m.insert(144, "textArchUp");
            // Java: docWordArt2DmlMap.put(145, "textArchDown");
            m.insert(145, "textArchDown");
            // Java: docWordArt2DmlMap.put(146, "textCircle");
            m.insert(146, "textCircle");
            // Java: docWordArt2DmlMap.put(147, "textButton");
            m.insert(147, "textButton");
            // Java: docWordArt2DmlMap.put(148, "textArchUpPour");
            m.insert(148, "textArchUpPour");
            // Java: docWordArt2DmlMap.put(149, "textArchDownPour");
            m.insert(149, "textArchDownPour");
            // Java: docWordArt2DmlMap.put(150, "textCirclePour");
            m.insert(150, "textCirclePour");
            // Java: docWordArt2DmlMap.put(151, "textButtonPour");
            m.insert(151, "textButtonPour");
            // Java: docWordArt2DmlMap.put(152, "textCurveUp");
            m.insert(152, "textCurveUp");
            // Java: docWordArt2DmlMap.put(153, "textCurveDown");
            m.insert(153, "textCurveDown");
            // Java: docWordArt2DmlMap.put(154, "textCascadeUp");
            m.insert(154, "textCascadeUp");
            // Java: docWordArt2DmlMap.put(155, "textCascadeDown");
            m.insert(155, "textCascadeDown");
            // Java: docWordArt2DmlMap.put(156, "textWave1");
            m.insert(156, "textWave1");
            // Java: docWordArt2DmlMap.put(157, "textWave2");
            m.insert(157, "textWave2");
            // Java: docWordArt2DmlMap.put(158, "textDoubleWave1");
            m.insert(158, "textDoubleWave1");
            // Java: docWordArt2DmlMap.put(159, "textWave4");
            m.insert(159, "textWave4");
            // Java: docWordArt2DmlMap.put(160, "textInflate");
            m.insert(160, "textInflate");
            // Java: docWordArt2DmlMap.put(161, "textDeflate");
            m.insert(161, "textDeflate");
            // Java: docWordArt2DmlMap.put(162, "textInflateBottom");
            m.insert(162, "textInflateBottom");
            // Java: docWordArt2DmlMap.put(163, "textDeflateBottom");
            m.insert(163, "textDeflateBottom");
            // Java: docWordArt2DmlMap.put(164, "textInflateTop");
            m.insert(164, "textInflateTop");
            // Java: docWordArt2DmlMap.put(165, "textDeflateTop");
            m.insert(165, "textDeflateTop");
            // Java: docWordArt2DmlMap.put(166, "textDeflateInflate");
            m.insert(166, "textDeflateInflate");
            // Java: docWordArt2DmlMap.put(167, "textDeflateInflateDeflate");
            m.insert(167, "textDeflateInflateDeflate");
            // Java: docWordArt2DmlMap.put(168, "textFadeRight");
            m.insert(168, "textFadeRight");
            // Java: docWordArt2DmlMap.put(169, "textFadeLeft");
            m.insert(169, "textFadeLeft");
            // Java: docWordArt2DmlMap.put(170, "textFadeUp");
            m.insert(170, "textFadeUp");
            // Java: docWordArt2DmlMap.put(171, "textFadeDown");
            m.insert(171, "textFadeDown");
            // Java: docWordArt2DmlMap.put(172, "textSlantUp");
            m.insert(172, "textSlantUp");
            // Java: docWordArt2DmlMap.put(173, "textSlantDown");
            m.insert(173, "textSlantDown");
            // Java: docWordArt2DmlMap.put(174, "textCanUp");
            m.insert(174, "textCanUp");
            // Java: docWordArt2DmlMap.put(175, "textCanDown");
            m.insert(175, "textCanDown");
            m
        });
        // Java: return docWordArt2DmlMap;
        result
    }

    // Java: String getDmlWordArtName(int type)
    pub fn getDmlWordArtName(r#type: i32) -> Option<&'static str> {
        // Java: Map<Integer, String> docWordArt2DmlMap = getDocWordArt2DmlMap();
        let docWordArt2DmlMap = Self::getDocWordArt2DmlMap();
        // Java: return (String)docWordArt2DmlMap.get(type);
        docWordArt2DmlMap.get(&r#type).copied()
    }

    // Java: boolean isCircleShape(int type)
    pub fn isCircleShape(r#type: i32) -> bool {
        // Java: return type == 3 || type == 96 || type == 120;
        // LLM JavaLine Aggregation: type is a Rust keyword, using r#type
        r#type == 3 || r#type == 96 || r#type == 120
    }
}
