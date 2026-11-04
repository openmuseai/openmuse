// Java: cn.wps.moffice.writer.layout.base.languages.ILangLayout
// rust: ★wr-29 04 —— 真实移植（Java 82 行）：trait + GraphOperationEnv + RollBackResult。
//       GraphOperationEnv 内指针字段对应 Java 的对象引用（null 语义）。
#![allow(dead_code, unused_imports, non_snake_case, unused_variables)]

// Java: import cn.wps.moffice.writer.cache.style.TypoFontStyle
use crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStyle;
// Java: import cn.wps.moffice.writer.layout.base.graph.Graphs
use crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs;
// Java: import cn.wps.moffice.writer.core.CharParse.CharInfo
use crate::cn::wps::moffice::writer::core::CharParse::CharInfo as CharInfoLocal;

// java public static class RollBackResult
// rust: ★wr-29 05 —— 压缩回滚（DEFERRED 面保留）
#[derive(Default)]
pub struct RollBackResult {
    // java public boolean adjusted;
    pub adjusted: bool,
    // java public boolean rollbackedAll;
    pub rollbackedAll: bool,
}

// java public static class GraphOperationEnv
// rust: ★wr-29 04 —— 指针字段对应 Java 对象引用（null 语义）；块数组独立持有
//       （Java 的 blockResult 是 GraphReader.mBlockResult 的别名引用；Rust 内仅写入
//       typoPara.getCharBlock 的 out 参数，独立数组等值）。
pub struct GraphOperationEnv {
    // java public DocumentEnv documentEnv;
    pub documentEnv: *mut crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv,
    // java public LayoutState layoutState;
    pub layoutState: *mut crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState,
    // java public TypoFontStyle.TypoFontStylePool typoFontStylePool;
    pub typoFontStylePool: *mut crate::cn::wps::moffice::writer::cache::style::TypoFontStyle::TypoFontStylePool,
    // java public ISymbolConvert symbolConvert;
    pub symbolConvert: Option<()>,
    // java public CharParse.CharInfo charInfo;
    pub charInfo: CharInfoLocal,
    // java public KPList<ArabicNode> arabicNodeList;
    pub arabicNodeList: Option<()>,
    // java public Graphs langGraphNode;
    // rust: ★wr-29 04 —— 池节点指针（Java 对象引用；calcGraphNode 就地改 mWidth）
    pub langGraphNode: *mut crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs,
    // java public Graphs commonGraphNode;
    pub commonGraphNode: *mut crate::cn::wps::moffice::writer::layout::base::graph::Graphs::Graphs,
    // java public CharsNode charsNode;
    pub charsNode: *mut crate::cn::wps::moffice::writer::layout::base::graph::CharsNode::CharsNode,
    // java public FontMetrics fontMetrics;
    // rust: FONTMETRICS(thirdparty) —— 页断 serif 分支用
    pub fontMetrics: Option<thirdparty_font::cn::wps::font::ITypeface::FontMetrics>,
    // java public char[] buf;
    pub buf: Option<Vec<char>>,
    // java public char[] nonBMPChars;
    pub nonBMPChars: [char; 2],
    // java public int[] blockResult;
    pub blockResult: [i32; 2],
    // java public ICapsHandler capsHandler;
    pub capsHandler: Option<()>,
    // java public int runType = 1;
    pub runType: i32,
    // java public int autoSpaceType;
    pub autoSpaceType: i32,
    // java public int mathUnderline;
    pub mathUnderline: i32,
    // java public int mathUnderlineColor;
    pub mathUnderlineColor: i32,
    // java public int highSurrogateType;
    pub highSurrogateType: i32,
    // java public int lowSurrogateType;
    pub lowSurrogateType: i32,
    // java public boolean isMathContent;
    pub isMathContent: bool,
    // java public boolean isProccessMathOp;
    pub isProccessMathOp: bool,
    // java public boolean isInMath;
    pub isInMath: bool,
    // java public boolean isScralForSmallCaps;
    pub isScralForSmallCaps: bool,
    // java public boolean isScralForTmpGrpahNode;
    pub isScralForTmpGrpahNode: bool,
    // java public boolean isIgnoreLeading;
    pub isIgnoreLeading: bool,
    // java public boolean isCalcGraphHeight;
    pub isCalcGraphHeight: bool,
    // java public boolean isIsolate;
    pub isIsolate: bool,
    // java public boolean isNoWidth;
    pub isNoWidth: bool,
    // java public boolean isUnderScore;
    pub isUnderScore: bool,
}

impl Default for GraphOperationEnv {
    fn default() -> Self {
        Self {
            documentEnv: std::ptr::null_mut(),
            layoutState: std::ptr::null_mut(),
            typoFontStylePool: std::ptr::null_mut(),
            symbolConvert: None,
            charInfo: Default::default(),
            arabicNodeList: None,
            langGraphNode: std::ptr::null_mut(),
            commonGraphNode: std::ptr::null_mut(),
            charsNode: std::ptr::null_mut(),
            fontMetrics: None,
            buf: None,
            nonBMPChars: ['\0'; 2],
            blockResult: [0; 2],
            capsHandler: None,
            runType: 1,
            autoSpaceType: 0,
            mathUnderline: 0,
            mathUnderlineColor: 0,
            highSurrogateType: 30,
            lowSurrogateType: 31,
            isMathContent: false,
            isProccessMathOp: false,
            isInMath: false,
            isScralForSmallCaps: false,
            isScralForTmpGrpahNode: false,
            isIgnoreLeading: false,
            isCalcGraphHeight: false,
            isIsolate: false,
            isNoWidth: false,
            isUnderScore: false,
        }
    }
}

impl GraphOperationEnv {
    // java public void dispose()
    pub fn dispose(&mut self) {
        // java if (null != this.arabicNodeList) { this.arabicNodeList.dispose(); this.arabicNodeList = null; }
        // rust: arabicNodeList 未接线（GR-D3）→ 恒空
        self.arabicNodeList = None;
    }
}

// java public interface ILangLayout
// rust: 节点返回用 `*mut Graphs`（Java 对象引用语义：调用方就地改字段）
pub trait ILangLayout {
    // java Graphs addGraphNode(int var1, int var2, GraphOperationEnv var3);
    fn addGraphNode(&mut self, var1: i32, var2: i32, var3: &mut GraphOperationEnv) -> *mut Graphs;

    // java int addLangGraph(int var1, int var2, int var3, int var4, int[] var5, GraphOperationEnv var6);
    fn addLangGraph(&mut self, var1: i32, var2: i32, var3: i32, var4: i32, var5: &[i32], var6: &mut GraphOperationEnv) -> i32;

    // java void commitLangGraphNode(GraphOperationEnv var1);
    fn commitLangGraphNode(&mut self, var1: &mut GraphOperationEnv);

    // java int addCommonGraph(int var1, GraphOperationEnv var2);
    fn addCommonGraph(&mut self, var1: i32, var2: &mut GraphOperationEnv) -> i32;

    // java void commitCommonGraphNode(GraphOperationEnv var1);
    fn commitCommonGraphNode(&mut self, var1: &mut GraphOperationEnv);

    // java TypoFontStyle processMathUnderline(TypoFontStyle var1, GraphOperationEnv var2);
    fn processMathUnderline(&mut self, var1: TypoFontStyle, var2: &mut GraphOperationEnv) -> TypoFontStyle;

    // java void rollBackForPartialWord(...);
    // rust: ★wr-29 05 —— 压缩回滚（DEFERRED 面保留；参数以 i32 承载 LineRuleGraphsPos 说明位）
    fn rollBackForPartialWord(&mut self, var1: i32, var2: i32, var3: i32, var4: i32, var5: i32, var6: i32, var7: &mut RollBackResult);

    // java boolean hasMergeChar();
    fn hasMergeChar(&self) -> bool;
}