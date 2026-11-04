#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.simple.StringIntMap
use crate::io::simple::StringIntMap::StringIntMap;
// Java: import cn.wps.io.simple.XmlName2Id
// Java: import cn.wps.moffice.crash.NoSpaceLeftException
// Java: import cn.wps.moffice.kfs.FileWriter
// Java: import java.io.BufferedWriter
// Java: import java.io.IOException
// Java: import java.lang.reflect.Field

// Java field: String PREFIX
pub static PREFIX: &str = "m";
// Java field: String NAMESPACING
pub const NAMESPACING: &str = "http://schemas.openxmlformats.org/officeDocument/2006/math";
// Java field: String OMATHPARA
const OMATHPARA: &str = "oMathPara";
// Java field: String OMATHPARAPR
const OMATHPARAPR: &str = "oMathParaPr";
// Java field: String JC
const JC: &str = "jc";
// Java field: String OMATH
const OMATH: &str = "oMath";
// Java field: String R
const R: &str = "r";
// Java field: String RPR
const RPR: &str = "rPr";
// Java field: String T
const T: &str = "t";
// Java field: String LIT
const LIT: &str = "lit";
// Java field: String BRK
const BRK: &str = "brk";
// Java field: String ALN
const ALN: &str = "aln";
// Java field: String NOR
const NOR: &str = "nor";
// Java field: String SCR
const SCR: &str = "scr";
// Java field: String STY
const STY: &str = "sty";
// Java field: String FRACTION
const FRACTION: &str = "f";
// Java field: String NUM
const NUM: &str = "num";
// Java field: String DEN
const DEN: &str = "den";
// Java field: String FPR
const FPR: &str = "fPr";
// Java field: String FTYPE
const FTYPE: &str = "type";
// Java field: String ACC
const ACC: &str = "acc";
// Java field: String ACCPR
const ACCPR: &str = "accPr";
// Java field: String MATHARG
const MATHARG: &str = "e";
// Java field: String CHR
const CHR: &str = "chr";
// Java field: String ARGPR
const ARGPR: &str = "argPr";
// Java field: String ARGSZ
const ARGSZ: &str = "argSz";
// Java field: String CTRLPR
const CTRLPR: &str = "ctrlPr";
// Java field: String SUB
const SUB: &str = "sub";
// Java field: String SUP
const SUP: &str = "sup";
// Java field: String SSUBSUP
const SSUBSUP: &str = "sSubSup";
// Java field: String SSUBSUPPR
const SSUBSUPPR: &str = "sSubSupPr";
// Java field: String ALNSCR
const ALNSCR: &str = "alnScr";
// Java field: String SPRE
const SPRE: &str = "sPre";
// Java field: String SPREPR
const SPREPR: &str = "sPrePr";
// Java field: String SSUB
const SSUB: &str = "sSub";
// Java field: String SSUBPR
const SSUBPR: &str = "sSubPr";
// Java field: String SSUP
const SSUP: &str = "sSup";
// Java field: String SSUPPR
const SSUPPR: &str = "sSupPr";
// Java field: String RAD
const RAD: &str = "rad";
// Java field: String RADPR
const RADPR: &str = "radPr";
// Java field: String DEG
const DEG: &str = "deg";
// Java field: String DEGHIDE
const DEGHIDE: &str = "degHide";
// Java field: String NARY
const NARY: &str = "nary";
// Java field: String NARYPR
const NARYPR: &str = "naryPr";
// Java field: String SUBHIDE
const SUBHIDE: &str = "subHide";
// Java field: String SUPHIDE
const SUPHIDE: &str = "supHide";
// Java field: String GROW
const GROW: &str = "grow";
// Java field: String LIMLOC
const LIMLOC: &str = "limLoc";
// Java field: String LIMUPP
const LIMUPP: &str = "limUpp";
// Java field: String LIMUPPPR
const LIMUPPPR: &str = "limUppPr";
// Java field: String LIM
const LIM: &str = "lim";
// Java field: String LIMLOW
const LIMLOW: &str = "limLow";
// Java field: String LIMLOWPR
const LIMLOWPR: &str = "limLowPr";
// Java field: String GROUPCHR
const GROUPCHR: &str = "groupChr";
// Java field: String GROUPCHRPR
const GROUPCHRPR: &str = "groupChrPr";
// Java field: String POS
const POS: &str = "pos";
// Java field: String VERTJC
const VERTJC: &str = "vertJc";
// Java field: String FUNC
const FUNC: &str = "func";
// Java field: String FUNCPR
const FUNCPR: &str = "funcPr";
// Java field: String FNAME
const FNAME: &str = "fName";
// Java field: String BAR
const BAR: &str = "bar";
// Java field: String BARPR
const BARPR: &str = "barPr";
// Java field: String BOX
const BOX: &str = "box";
// Java field: String BOXPR
const BOXPR: &str = "boxPr";
// Java field: String OPEMU
const OPEMU: &str = "opEmu";
// Java field: String NOBREAK
const NOBREAK: &str = "noBreak";
// Java field: String DIFF
const DIFF: &str = "diff";
// Java field: String D
const D: &str = "d";
// Java field: String DPR
const DPR: &str = "dPr";
// Java field: String BEGCHR
const BEGCHR: &str = "begChr";
// Java field: String SEPCHR
const SEPCHR: &str = "sepChr";
// Java field: String ENDCHR
const ENDCHR: &str = "endChr";
// Java field: String SHP
const SHP: &str = "shp";
// Java field: String EQARR
const EQARR: &str = "eqArr";
// Java field: String EQARRPR
const EQARRPR: &str = "eqArrPr";
// Java field: String BASEJC
const BASEJC: &str = "baseJc";
// Java field: String MAXDIST
const MAXDIST: &str = "maxDist";
// Java field: String OBJDIST
const OBJDIST: &str = "objDist";
// Java field: String RSPRULE
const RSPRULE: &str = "rSpRule";
// Java field: String RSP
const RSP: &str = "rSp";
// Java field: String BORDERBOX
const BORDERBOX: &str = "borderBox";
// Java field: String BORDERBOXPR
const BORDERBOXPR: &str = "borderBoxPr";
// Java field: String HIDETOP
const HIDETOP: &str = "hideTop";
// Java field: String HIDEBOT
const HIDEBOT: &str = "hideBot";
// Java field: String HIDELEFT
const HIDELEFT: &str = "hideLeft";
// Java field: String HIDERIGHT
const HIDERIGHT: &str = "hideRight";
// Java field: String STRIKEH
const STRIKEH: &str = "strikeH";
// Java field: String STRIKEV
const STRIKEV: &str = "strikeV";
// Java field: String STRIKEBLTR
const STRIKEBLTR: &str = "strikeBLTR";
// Java field: String STRIKETLBR
const STRIKETLBR: &str = "strikeTLBR";
// Java field: String M
const M: &str = "m";
// Java field: String MPR
const MPR: &str = "mPr";
// Java field: String MR
const MR: &str = "mr";
// Java field: String MCS
const MCS: &str = "mcs";
// Java field: String MC
const MC: &str = "mc";
// Java field: String MCPR
const MCPR: &str = "mcPr";
// Java field: String MCJC
const MCJC: &str = "mcJc";
// Java field: String COUNT
const COUNT: &str = "count";
// Java field: String PLCHIDE
const PLCHIDE: &str = "plcHide";
// Java field: String CGPRULE
const CGPRULE: &str = "cGpRule";
// Java field: String CGP
const CGP: &str = "cGp";
// Java field: String CSP
const CSP: &str = "cSp";
// Java field: String MATH_PR
const MATH_PR: &str = "mathPr";
// Java field: String MATH_FONT
const MATH_FONT: &str = "mathFont";
// Java field: String BRK_BIN
const BRK_BIN: &str = "brkBin";
// Java field: String BRK_BIN_SUB
const BRK_BIN_SUB: &str = "brkBinSub";
// Java field: String SMALL_FRAC
const SMALL_FRAC: &str = "smallFrac";
// Java field: String DISP_DEF
const DISP_DEF: &str = "dispDef";
// Java field: String L_MARGIN
const L_MARGIN: &str = "lMargin";
// Java field: String R_MARGIN
const R_MARGIN: &str = "rMargin";
// Java field: String DEF_JC
const DEF_JC: &str = "defJc";
// Java field: String PRE_SP
const PRE_SP: &str = "preSp";
// Java field: String POST_SP
const POST_SP: &str = "postSp";
// Java field: String INTER_SP
const INTER_SP: &str = "interSp";
// Java field: String INTRA_SP
const INTRA_SP: &str = "intraSp";
// Java field: String WRAP_INDENT
const WRAP_INDENT: &str = "wrapIndent";
// Java field: String WRAP_RIGHT
const WRAP_RIGHT: &str = "wrapRight";
// Java field: String INT_LIM
const INT_LIM: &str = "intLim";
// Java field: String NARY_LIM
const NARY_LIM: &str = "naryLim";
// Java field: MMap obj
static mut OBJ: Option<MMap> = None;

// Java: public class MMap implements XmlName2Id
pub struct MMap {
    // Java field: StringIntMap mapobj
    mapobj: StringIntMap,
}

// Java: public class MMap implements XmlName2Id
impl MMap {
    // Java: void init()
    fn init(&mut self) {
        // Java: this.mapobj.put("oMathPara", 1601478807);
        self.mapobj.put("oMathPara", 1601478807);
        // Java: this.mapobj.put("oMathParaPr", 1422844153);
        self.mapobj.put("oMathParaPr", 1422844153);
        // Java: this.mapobj.put("jc", 3385);
        self.mapobj.put("jc", 3385);
        // Java: this.mapobj.put("oMath", 104901655);
        self.mapobj.put("oMath", 104901655);
        // Java: this.mapobj.put("r", 114);
        self.mapobj.put("r", 114);
        // Java: this.mapobj.put("rPr", 112148);
        self.mapobj.put("rPr", 112148);
        // Java: this.mapobj.put("t", 116);
        self.mapobj.put("t", 116);
        // Java: this.mapobj.put("lit", 107159);
        self.mapobj.put("lit", 107159);
        // Java: this.mapobj.put("brk", 97819);
        self.mapobj.put("brk", 97819);
        // Java: this.mapobj.put("aln", 96675);
        self.mapobj.put("aln", 96675);
        // Java: this.mapobj.put("nor", 109265);
        self.mapobj.put("nor", 109265);
        // Java: this.mapobj.put("scr", 113698);
        self.mapobj.put("scr", 113698);
        // Java: this.mapobj.put("sty", 114232);
        self.mapobj.put("sty", 114232);
        // Java: this.mapobj.put("f", 102);
        self.mapobj.put("f", 102);
        // Java: this.mapobj.put("num", 109446);
        self.mapobj.put("num", 109446);
        // Java: this.mapobj.put("den", 99341);
        self.mapobj.put("den", 99341);
        // Java: this.mapobj.put("fPr", 100616);
        self.mapobj.put("fPr", 100616);
        // Java: this.mapobj.put("type", 3575610);
        self.mapobj.put("type", 3575610);
        // Java: this.mapobj.put("acc", 96385);
        self.mapobj.put("acc", 96385);
        // Java: this.mapobj.put("accPr", 92628579);
        self.mapobj.put("accPr", 92628579);
        // Java: this.mapobj.put("e", 101);
        self.mapobj.put("e", 101);
        // Java: this.mapobj.put("chr", 98477);
        self.mapobj.put("chr", 98477);
        // Java: this.mapobj.put("argPr", 93079288);
        self.mapobj.put("argPr", 93079288);
        // Java: this.mapobj.put("argSz", 93079389);
        self.mapobj.put("argSz", 93079389);
        // Java: this.mapobj.put("ctrlPr", -1350050355);
        self.mapobj.put("ctrlPr", -1350050355);
        // Java: this.mapobj.put("sub", 114240);
        self.mapobj.put("sub", 114240);
        // Java: this.mapobj.put("sup", 114254);
        self.mapobj.put("sup", 114254);
        // Java: this.mapobj.put("sSubSup", 1470982721);
        self.mapobj.put("sSubSup", 1470982721);
        // Java: this.mapobj.put("sSubSupPr", 570157091);
        self.mapobj.put("sSubSupPr", 570157091);
        // Java: this.mapobj.put("alnScr", -1414839425);
        self.mapobj.put("alnScr", -1414839425);
        // Java: this.mapobj.put("sPre", 3506480);
        self.mapobj.put("sPre", 3506480);
        // Java: this.mapobj.put("sPrePr", -925237422);
        self.mapobj.put("sPrePr", -925237422);
        // Java: this.mapobj.put("sSub", 3509453);
        self.mapobj.put("sSub", 3509453);
        // Java: this.mapobj.put("sSubPr", -922380369);
        self.mapobj.put("sSubPr", -922380369);
        // Java: this.mapobj.put("sSup", 3509467);
        self.mapobj.put("sSup", 3509467);
        // Java: this.mapobj.put("sSupPr", -922366915);
        self.mapobj.put("sSupPr", -922366915);
        // Java: this.mapobj.put("rad", 112661);
        self.mapobj.put("rad", 112661);
        // Java: this.mapobj.put("radPr", 108269815);
        self.mapobj.put("radPr", 108269815);
        // Java: this.mapobj.put("deg", 99334);
        self.mapobj.put("deg", 99334);
        // Java: this.mapobj.put("degHide", 1544970856);
        self.mapobj.put("degHide", 1544970856);
        // Java: this.mapobj.put("nary", 3373882);
        self.mapobj.put("nary", 3373882);
        // Java: this.mapobj.put("naryPr", -1052664100);
        self.mapobj.put("naryPr", -1052664100);
        // Java: this.mapobj.put("subHide", -1868894302);
        self.mapobj.put("subHide", -1868894302);
        // Java: this.mapobj.put("supHide", -1855965008);
        self.mapobj.put("supHide", -1855965008);
        // Java: this.mapobj.put("grow", 3181587);
        self.mapobj.put("grow", 3181587);
        // Java: this.mapobj.put("limLoc", -1102725488);
        self.mapobj.put("limLoc", -1102725488);
        // Java: this.mapobj.put("limUpp", -1102716795);
        self.mapobj.put("limUpp", -1102716795);
        // Java: this.mapobj.put("limUppPr", 1146084711);
        self.mapobj.put("limUppPr", 1146084711);
        // Java: this.mapobj.put("lim", 107152);
        self.mapobj.put("lim", 107152);
        // Java: this.mapobj.put("limLow", -1102725468);
        self.mapobj.put("limLow", -1102725468);
        // Java: this.mapobj.put("limLowPr", 1137749958);
        self.mapobj.put("limLowPr", 1137749958);
        // Java: this.mapobj.put("groupChr", 506334638);
        self.mapobj.put("groupChr", 506334638);
        // Java: this.mapobj.put("groupChrPr", 1256285264);
        self.mapobj.put("groupChrPr", 1256285264);
        // Java: this.mapobj.put("pos", 111188);
        self.mapobj.put("pos", 111188);
        // Java: this.mapobj.put("vertJc", -819941814);
        self.mapobj.put("vertJc", -819941814);
        // Java: this.mapobj.put("func", 3154628);
        self.mapobj.put("func", 3154628);
        // Java: this.mapobj.put("funcPr", -1263367194);
        self.mapobj.put("funcPr", -1263367194);
        // Java: this.mapobj.put("fName", 96619537);
        self.mapobj.put("fName", 96619537);
        // Java: this.mapobj.put("bar", 97299);
        self.mapobj.put("bar", 97299);
        // Java: this.mapobj.put("barPr", 93506933);
        self.mapobj.put("barPr", 93506933);
        // Java: this.mapobj.put("box", 97739);
        self.mapobj.put("box", 97739);
        // Java: this.mapobj.put("boxPr", 93929773);
        self.mapobj.put("boxPr", 93929773);
        // Java: this.mapobj.put("opEmu", 105917228);
        self.mapobj.put("opEmu", 105917228);
        // Java: this.mapobj.put("noBreak", 2083441598);
        self.mapobj.put("noBreak", 2083441598);
        // Java: this.mapobj.put("diff", 3083269);
        self.mapobj.put("diff", 3083269);
        // Java: this.mapobj.put("d", 100);
        self.mapobj.put("d", 100);
        // Java: this.mapobj.put("shp", 113851);
        self.mapobj.put("shp", 113851);
        // Java: this.mapobj.put("dPr", 98694);
        self.mapobj.put("dPr", 98694);
        // Java: this.mapobj.put("begChr", -1392898679);
        self.mapobj.put("begChr", -1392898679);
        // Java: this.mapobj.put("sepChr", -905934993);
        self.mapobj.put("sepChr", -905934993);
        // Java: this.mapobj.put("endChr", -1298788910);
        self.mapobj.put("endChr", -1298788910);
        // Java: this.mapobj.put("eqArr", 96708117);
        self.mapobj.put("eqArr", 96708117);
        // Java: this.mapobj.put("eqArrPr", -1552777481);
        self.mapobj.put("eqArrPr", -1552777481);
        // Java: this.mapobj.put("baseJc", -1396203542);
        self.mapobj.put("baseJc", -1396203542);
        // Java: this.mapobj.put("maxDist", 843633962);
        self.mapobj.put("maxDist", 843633962);
        // Java: this.mapobj.put("objDist", -1660626115);
        self.mapobj.put("objDist", -1660626115);
        // Java: this.mapobj.put("rSpRule", 578417163);
        self.mapobj.put("rSpRule", 578417163);
        // Java: this.mapobj.put("rSp", 112239);
        self.mapobj.put("rSp", 112239);
        // Java: this.mapobj.put("borderBox", 197399039);
        self.mapobj.put("borderBox", 197399039);
        // Java: this.mapobj.put("borderBoxPr", 721918049);
        self.mapobj.put("borderBoxPr", 721918049);
        // Java: this.mapobj.put("hideTop", 912608435);
        self.mapobj.put("hideTop", 912608435);
        // Java: this.mapobj.put("hideLeft", -1774157719);
        self.mapobj.put("hideLeft", -1774157719);
        // Java: this.mapobj.put("hideBot", 912591141);
        self.mapobj.put("hideBot", 912591141);
        // Java: this.mapobj.put("hideRight", 841346554);
        self.mapobj.put("hideRight", 841346554);
        // Java: this.mapobj.put("strikeH", -1881762090);
        self.mapobj.put("strikeH", -1881762090);
        // Java: this.mapobj.put("strikeV", -1881762076);
        self.mapobj.put("strikeV", -1881762076);
        // Java: this.mapobj.put("strikeBLTR", -1661378822);
        self.mapobj.put("strikeBLTR", -1661378822);
        // Java: this.mapobj.put("strikeTLBR", -1660843142);
        self.mapobj.put("strikeTLBR", -1660843142);
        // Java: this.mapobj.put("m", 109);
        self.mapobj.put("m", 109);
        // Java: this.mapobj.put("mPr", 107343);
        self.mapobj.put("mPr", 107343);
        // Java: this.mapobj.put("mr", 3493);
        self.mapobj.put("mr", 3493);
        // Java: this.mapobj.put("mc", 3478);
        self.mapobj.put("mc", 3478);
        // Java: this.mapobj.put("mcs", 107933);
        self.mapobj.put("mcs", 107933);
        // Java: this.mapobj.put("mcPr", 3344952);
        self.mapobj.put("mcPr", 3344952);
        // Java: this.mapobj.put("mcJc", 3344751);
        self.mapobj.put("mcJc", 3344751);
        // Java: this.mapobj.put("count", 94851343);
        self.mapobj.put("count", 94851343);
        // Java: this.mapobj.put("plcHide", -493176887);
        self.mapobj.put("plcHide", -493176887);
        // Java: this.mapobj.put("cGpRule", -192785976);
        self.mapobj.put("cGpRule", -192785976);
        // Java: this.mapobj.put("cGp", 97452);
        self.mapobj.put("cGp", 97452);
        // Java: this.mapobj.put("cSp", 97824);
        self.mapobj.put("cSp", 97824);
        // Java: this.mapobj.put("mathPr", -1081250006);
        self.mapobj.put("mathPr", -1081250006);
        // Java: this.mapobj.put("mathFont", 300532599);
        self.mapobj.put("mathFont", 300532599);
        // Java: this.mapobj.put("brkBin", -1380774676);
        self.mapobj.put("brkBin", -1380774676);
        // Java: this.mapobj.put("brkBinSub", -1756495436);
        self.mapobj.put("brkBinSub", -1756495436);
        // Java: this.mapobj.put("smallFrac", -1623637099);
        self.mapobj.put("smallFrac", -1623637099);
        // Java: this.mapobj.put("dispDef", 1671725827);
        self.mapobj.put("dispDef", 1671725827);
        // Java: this.mapobj.put("lMargin", -636325574);
        self.mapobj.put("lMargin", -636325574);
        // Java: this.mapobj.put("rMargin", 393729216);
        self.mapobj.put("rMargin", 393729216);
        // Java: this.mapobj.put("defJc", 95461406);
        self.mapobj.put("defJc", 95461406);
        // Java: this.mapobj.put("preSp", 106930272);
        self.mapobj.put("preSp", 106930272);
        // Java: this.mapobj.put("postSp", -982451427);
        self.mapobj.put("postSp", -982451427);
        // Java: this.mapobj.put("interSp", 1958062169);
        self.mapobj.put("interSp", 1958062169);
        // Java: this.mapobj.put("intraSp", 1958433115);
        self.mapobj.put("intraSp", 1958433115);
        // Java: this.mapobj.put("wrapIndent", -1736979050);
        self.mapobj.put("wrapIndent", -1736979050);
        // Java: this.mapobj.put("wrapRight", -47865870);
        self.mapobj.put("wrapRight", -47865870);
        // Java: this.mapobj.put("intLim", -1183786975);
        self.mapobj.put("intLim", -1183786975);
        // Java: this.mapobj.put("naryLim", 1727147254);
        self.mapobj.put("naryLim", 1727147254);
    }

    // Java: MMap()
    fn new() -> Self {
        // Java: super();
        let mut m = MMap {
            mapobj: StringIntMap::new(7),
        };
        // Java: this.init();
        m.init();
        m
    }

    // Java: static MMap getInstance()
    pub fn getInstance() -> &'static MMap {
        // Java: if (obj == null)
        // Java: obj = new MMap();
        // Java: return obj;
        unsafe {
            if OBJ.is_none() {
                OBJ = Some(MMap::new());
            }
            OBJ.as_ref().unwrap()
        }
    }

    // Java: int getId(String uri, String name)
    pub fn getId_str(&self, _uri: &str, name: &str) -> i32 {
        // Java: return this.mapobj.get(name);
        self.mapobj.get(name)
    }

    // Java: int getId(int prefix, int name)
    pub fn getId_int(&self, _prefix: i32, name: i32) -> i32 {
        // Java: return this.mapobj.get(name);
        self.mapobj.get_by_int(name)
    }

    // Java: static void fill_property_names(String fileName)
    pub fn fill_property_names(_file_name: &str) {
        // Java: Class<?> pClass = MMap.class;
        // Java: Field[] fields = pClass.getDeclaredFields();
        // Java: try
        // Java: for(Field field : fields)
        // Java: String name = field.getName();
        // Java: Object pValue = field.get(pClass);
        // Java: if (pValue instanceof String)
        // Java: String value = (String)pValue;
        // Java: int id = value.hashCode();
        // Java: writeTimes(fileName, value, id);
        // Java: catch (Exception var11)
        todo!()
    }

    // Java: static void writeTimes(String fileName, String name, Integer id)
    pub fn writeTimes(_file_name: &str, _name: &str, _id: i32) {
        // Java: try
        // Java: BufferedWriter out = new BufferedWriter(new FileWriter(fileName + ".csv", true));
        // Java: out.append("\r\n");
        // Java: out.append(name + "," + name.toUpperCase() + ", 0x" + Integer.toHexString(id));
        // Java: out.close();
        // Java: catch (IOException e)
        // Java: e.printStackTrace();
        // Java: if (NoSpaceLeftException.isNoSpaceLeftException(e))
        // Java: throw new RuntimeException(e);
        todo!()
    }
}
