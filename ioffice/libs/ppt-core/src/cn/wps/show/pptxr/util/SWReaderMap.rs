#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

use std::collections::HashMap;
use std::sync::OnceLock;

// Java: import java.util.HashMap
// LLM: using std::collections::HashMap
// Java: import java.util.Map
// LLM: using HashMap as Map equivalent

// Java field: Map<String, Integer> conformanceClassMap
static conformanceClassMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> slideLayoutTypeMap
static slideLayoutTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> slideSizeMap
static slideSizeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> placeholderSizeMap
static placeholderSizeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> transitionSpeedMap
static transitionSpeedMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> transitionSideDirectionMap
static transitionSideDirectionMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> transitionCornerDirectionMap
static transitionCornerDirectionMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> transitionEightDirMap
static transitionEightDirMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> transitionInOutDirectionMap
static transitionInOutDirectionMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> transitionLeftRightDirectionMap
static transitionLeftRightDirectionMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> transitionCornerAndCenterDirectionMap
static transitionCornerAndCenterDirectionMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> transitionPatternMap
static transitionPatternMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> transitionShredPatternMap
static transitionShredPatternMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> placeholderTypeMap
static placeholderTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> timeNodeFillTypeMap
static timeNodeFillTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> timeNodeMasterRelationMap
static timeNodeMasterRelationMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> timeNodeTypeMap
static timeNodeTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> timeNodePresetClassMap
static timeNodePresetClassMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> timeNodeRestartTypeMap
static timeNodeRestartTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> timeNodeSyncTypeMap
static timeNodeSyncTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> animBehaviorCalcModeMap
static animBehaviorCalcModeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> animBehaviorValueTypeMap
static animBehaviorValueTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> behaviorAccumulateTypeMap
static behaviorAccumulateTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> behaviorAdditiveTypeMap
static behaviorAdditiveTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> behaviorOverrideTypeMap
static behaviorOverrideTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> behaviorTransformTypeMap
static behaviorTransformTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> chartBuildStepMap
static chartBuildStepMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> dgmBuildStepMap
static dgmBuildStepMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> chartSubelementType
static chartSubelementType: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> animColorSpaceMap
static animColorSpaceMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> animColorDirectionMap
static animColorDirectionMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> animEffectTransitionMap
static animEffectTransitionMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> animMotionBehaviorOriginMap
static animMotionBehaviorOriginMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> animMotionPathEditModeMap
static animMotionPathEditModeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> commandTypeMap
static commandTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> previousActionTypeMap
static previousActionTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> nextActionTypeMap
static nextActionTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> triggerEventMap
static triggerEventMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> paraBuildMap
static paraBuildMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> diagramBuildMap
static diagramBuildMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> oleChartBuildMap
static oleChartBuildMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> animationDgmBuildMap
static animationDgmBuildMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> animationChartBuildMap
static animationChartBuildMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> iteratorTypeMap
static iteratorTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> triggerRuntimeNodeMap
static triggerRuntimeNodeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> photoAlbumFrameShapeMap
static photoAlbumFrameShapeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> photoAlbumLayoutMap
static photoAlbumLayoutMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> viewTypeMap
static viewTypeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> splitterBarStateMap
static splitterBarStateMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> behaviorAttributeNameMap
static behaviorAttributeNameMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> presetTransitionMap
static presetTransitionMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> printWhatMap
static printWhatMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> printColorModeMap
static printColorModeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();
// Java field: Map<String, Integer> oleObjectFollowColorSchemeMap
static oleObjectFollowColorSchemeMap: OnceLock<HashMap<&'static str, i32>> = OnceLock::new();

macro_rules! make_map {
    ($($key:expr => $val:expr),+ $(,)?) => {{
        let mut m = HashMap::new();
        $(m.insert($key, $val);)+
        m
    }};
}

// Java: public class SWReaderMap
pub struct SWReaderMap;

// Java: public class SWReaderMap
impl SWReaderMap {
    pub fn get_conformanceClassMap() -> &'static HashMap<&'static str, i32> {
        conformanceClassMap.get_or_init(|| make_map!(
            "strict" => 0, "transitional" => 1
        ))
    }

    pub fn get_slideLayoutTypeMap() -> &'static HashMap<&'static str, i32> {
        slideLayoutTypeMap.get_or_init(|| make_map!(
            "blank" => 16, "chart" => 19, "chartAndTx" => 20, "clipArtAndTx" => 21,
            "clipArtAndVertTx" => 22, "cust" => 23, "dgm" => 24, "fourObj" => 14,
            "mediaAndTx" => 25, "obj" => 26, "objAndTwoObj" => 27, "objAndTx" => 28,
            "objOnly" => 15, "objOverTx" => 29, "objTx" => 31, "picTx" => 32,
            "secHead" => 33, "tbl" => 34, "title" => 0, "titleOnly" => 7,
            "twoColTx" => 8, "twoObj" => 35, "twoObjAndObj" => 36, "twoObjAndTx" => 11,
            "twoObjOverTx" => 13, "twoTxTwoObj" => 37, "tx" => 1, "txAndChart" => 38,
            "txAndClipArt" => 39, "txAndMedia" => 40, "txAndObj" => 41,
            "txAndTwoObj" => 10, "txOverObj" => 42, "vertTitleAndTx" => 17,
            "vertTitleAndTxOverChart" => 18, "vertTx" => 43
        ))
    }

    pub fn get_slideSizeMap() -> &'static HashMap<&'static str, i32> {
        slideSizeMap.get_or_init(|| make_map!(
            "35mm" => 3, "A3" => 7, "A4" => 2, "B4ISO" => 8, "B4JIS" => 9,
            "B5ISO" => 10, "B5JIS" => 11, "banner" => 5, "custom" => 6,
            "hagakiCard" => 12, "ledger" => 13, "letter" => 1, "overhead" => 4,
            "screen16x10" => 14, "screen16x9" => 15, "screen4x3" => 0
        ))
    }

    pub fn get_placeholderSizeMap() -> &'static HashMap<&'static str, i32> {
        placeholderSizeMap.get_or_init(|| make_map!(
            "full" => 0, "half" => 1, "quarter" => 2
        ))
    }

    pub fn get_transitionSpeedMap() -> &'static HashMap<&'static str, i32> {
        transitionSpeedMap.get_or_init(|| make_map!(
            "slow" => 0, "med" => 1, "fast" => 2
        ))
    }

    pub fn get_transitionSideDirectionMap() -> &'static HashMap<&'static str, i32> {
        transitionSideDirectionMap.get_or_init(|| make_map!(
            "l" => 0, "u" => 1, "r" => 2, "d" => 3
        ))
    }

    pub fn get_transitionCornerDirectionMap() -> &'static HashMap<&'static str, i32> {
        transitionCornerDirectionMap.get_or_init(|| make_map!(
            "lu" => 4, "ru" => 5, "rd" => 7, "ld" => 6
        ))
    }

    pub fn get_transitionEightDirMap() -> &'static HashMap<&'static str, i32> {
        transitionEightDirMap.get_or_init(|| make_map!(
            "l" => 0, "u" => 1, "r" => 2, "d" => 3,
            "lu" => 4, "ru" => 5, "rd" => 7, "ld" => 6
        ))
    }

    pub fn get_transitionInOutDirectionMap() -> &'static HashMap<&'static str, i32> {
        transitionInOutDirectionMap.get_or_init(|| make_map!(
            "out" => 0, "in" => 1
        ))
    }

    pub fn get_transitionLeftRightDirectionMap() -> &'static HashMap<&'static str, i32> {
        transitionLeftRightDirectionMap.get_or_init(|| make_map!(
            "l" => 0, "r" => 1
        ))
    }

    pub fn get_transitionCornerAndCenterDirectionMap() -> &'static HashMap<&'static str, i32> {
        transitionCornerAndCenterDirectionMap.get_or_init(|| make_map!(
            "center" => 0, "lu" => 2, "ru" => 4, "rd" => 3, "ld" => 1
        ))
    }

    pub fn get_transitionPatternMap() -> &'static HashMap<&'static str, i32> {
        transitionPatternMap.get_or_init(|| make_map!(
            "diamond" => 1, "hexagon" => 0
        ))
    }

    pub fn get_transitionShredPatternMap() -> &'static HashMap<&'static str, i32> {
        transitionShredPatternMap.get_or_init(|| make_map!(
            "strip" => 1, "rectangle" => 0
        ))
    }

    pub fn get_placeholderTypeMap() -> &'static HashMap<&'static str, i32> {
        placeholderTypeMap.get_or_init(|| make_map!(
            "body" => 1, "chart" => 9, "clipArt" => 11, "ctrTitle" => 2,
            "dgm" => 12, "dt" => 4, "ftr" => 6, "hdr" => 7, "media" => 13,
            "obj" => 8, "pic" => 15, "sldImg" => 14, "sldNum" => 5,
            "subTitle" => 3, "tbl" => 10, "title" => 0
        ))
    }

    pub fn get_timeNodeFillTypeMap() -> &'static HashMap<&'static str, i32> {
        timeNodeFillTypeMap.get_or_init(|| make_map!(
            "freeze" => 2, "hold" => 3, "remove" => 1, "transition" => 4
        ))
    }

    pub fn get_timeNodeMasterRelationMap() -> &'static HashMap<&'static str, i32> {
        timeNodeMasterRelationMap.get_or_init(|| make_map!(
            "sameClick" => 0, "lastClick" => 1, "nextClick" => 2
        ))
    }

    pub fn get_timeNodeTypeMap() -> &'static HashMap<&'static str, i32> {
        timeNodeTypeMap.get_or_init(|| make_map!(
            "clickEffect" => 1, "withEffect" => 2, "afterEffect" => 3,
            "mainSeq" => 4, "interactiveSeq" => 5, "clickPar" => 6,
            "withGroup" => 7, "afterGroup" => 8, "tmRoot" => 9
        ))
    }

    pub fn get_timeNodePresetClassMap() -> &'static HashMap<&'static str, i32> {
        timeNodePresetClassMap.get_or_init(|| make_map!(
            "undefine" => 0, "entr" => 1, "exit" => 2, "emph" => 3,
            "path" => 4, "verb" => 5, "mediacall" => 6
        ))
    }

    pub fn get_timeNodeRestartTypeMap() -> &'static HashMap<&'static str, i32> {
        timeNodeRestartTypeMap.get_or_init(|| make_map!(
            "always" => 1, "whenNotActive" => 2, "never" => 0
        ))
    }

    pub fn get_timeNodeSyncTypeMap() -> &'static HashMap<&'static str, i32> {
        timeNodeSyncTypeMap.get_or_init(|| make_map!(
            "canSlip" => 0, "locked" => 1
        ))
    }

    pub fn get_animBehaviorCalcModeMap() -> &'static HashMap<&'static str, i32> {
        animBehaviorCalcModeMap.get_or_init(|| make_map!(
            "discrete" => 0, "lin" => 1, "fmla" => 2
        ))
    }

    pub fn get_animBehaviorValueTypeMap() -> &'static HashMap<&'static str, i32> {
        animBehaviorValueTypeMap.get_or_init(|| make_map!(
            "str" => 0, "num" => 1, "clr" => 2
        ))
    }

    pub fn get_behaviorAccumulateTypeMap() -> &'static HashMap<&'static str, i32> {
        behaviorAccumulateTypeMap.get_or_init(|| make_map!(
            "none" => 0, "always" => 1
        ))
    }

    pub fn get_behaviorAdditiveTypeMap() -> &'static HashMap<&'static str, i32> {
        behaviorAdditiveTypeMap.get_or_init(|| make_map!(
            "base" => 0, "sum" => 4, "repl" => 3, "mult" => 1, "none" => 2
        ))
    }

    pub fn get_behaviorOverrideTypeMap() -> &'static HashMap<&'static str, i32> {
        behaviorOverrideTypeMap.get_or_init(|| make_map!(
            "normal" => 0, "childStyle" => 1
        ))
    }

    pub fn get_behaviorTransformTypeMap() -> &'static HashMap<&'static str, i32> {
        behaviorTransformTypeMap.get_or_init(|| make_map!(
            "image" => 0, "img" => 0
        ))
    }

    pub fn get_chartBuildStepMap() -> &'static HashMap<&'static str, i32> {
        chartBuildStepMap.get_or_init(|| make_map!(
            "category" => 1, "ptInCategory" => 3, "series" => 5,
            "ptInSeries" => 4, "allPts" => 0, "gridLegend" => 2
        ))
    }

    pub fn get_dgmBuildStepMap() -> &'static HashMap<&'static str, i32> {
        dgmBuildStepMap.get_or_init(|| make_map!(
            "sp" => 1, "bg" => 0
        ))
    }

    pub fn get_chartSubelementType() -> &'static HashMap<&'static str, i32> {
        chartSubelementType.get_or_init(|| make_map!(
            "gridLegend" => 1, "series" => 4, "category" => 0,
            "ptInSeries" => 3, "ptInCategory" => 2
        ))
    }

    pub fn get_animColorSpaceMap() -> &'static HashMap<&'static str, i32> {
        animColorSpaceMap.get_or_init(|| make_map!(
            "rgb" => 0, "hsl" => 1
        ))
    }

    pub fn get_animColorDirectionMap() -> &'static HashMap<&'static str, i32> {
        animColorDirectionMap.get_or_init(|| make_map!(
            "cw" => 0, "ccw" => 1
        ))
    }

    pub fn get_animEffectTransitionMap() -> &'static HashMap<&'static str, i32> {
        animEffectTransitionMap.get_or_init(|| make_map!(
            "in" => 0, "out" => 1, "none" => 2
        ))
    }

    pub fn get_animMotionBehaviorOriginMap() -> &'static HashMap<&'static str, i32> {
        animMotionBehaviorOriginMap.get_or_init(|| make_map!(
            "layout" => 2, "parent" => 1
        ))
    }

    pub fn get_animMotionPathEditModeMap() -> &'static HashMap<&'static str, i32> {
        animMotionPathEditModeMap.get_or_init(|| make_map!(
            "fixed" => 0, "relative" => 1
        ))
    }

    pub fn get_commandTypeMap() -> &'static HashMap<&'static str, i32> {
        commandTypeMap.get_or_init(|| make_map!(
            "call" => 1, "evt" => 0, "verb" => 2
        ))
    }

    pub fn get_previousActionTypeMap() -> &'static HashMap<&'static str, i32> {
        previousActionTypeMap.get_or_init(|| make_map!(
            "none" => 0, "skipTimed" => 1
        ))
    }

    pub fn get_nextActionTypeMap() -> &'static HashMap<&'static str, i32> {
        nextActionTypeMap.get_or_init(|| make_map!(
            "none" => 0, "seek" => 1
        ))
    }

    pub fn get_triggerEventMap() -> &'static HashMap<&'static str, i32> {
        triggerEventMap.get_or_init(|| make_map!(
            "onBegin" => 1, "onEnd" => 2, "begin" => 3, "end" => 4,
            "onClick" => 5, "onDblClick" => 6, "onMouseOver" => 7,
            "onMouseOut" => 8, "onNext" => 9, "onPrev" => 10,
            "onStopAudio" => 11, "onMediaBookmark" => 12
        ))
    }

    pub fn get_paraBuildMap() -> &'static HashMap<&'static str, i32> {
        paraBuildMap.get_or_init(|| make_map!(
            "allAtOnce" => 0, "p" => 1, "cust" => 2, "whole" => 3
        ))
    }

    pub fn get_diagramBuildMap() -> &'static HashMap<&'static str, i32> {
        diagramBuildMap.get_or_init(|| make_map!(
            "whole" => 16, "depthByNode" => 11, "depthByBranch" => 10,
            "breadthByNode" => 2, "breadthByLvl" => 1, "cw" => 7,
            "cwIn" => 8, "cwOut" => 9, "ccw" => 3, "ccwIn" => 4,
            "ccwOut" => 5, "inByRing" => 13, "outByRing" => 14,
            "up" => 15, "down" => 12, "allAtOnce" => 0, "cust" => 6
        ))
    }

    pub fn get_oleChartBuildMap() -> &'static HashMap<&'static str, i32> {
        oleChartBuildMap.get_or_init(|| make_map!(
            "allAtOnce" => 0, "series" => 3, "category" => 1,
            "seriesEl" => 4, "categoryEl" => 2
        ))
    }

    pub fn get_animationDgmBuildMap() -> &'static HashMap<&'static str, i32> {
        animationDgmBuildMap.get_or_init(|| make_map!(
            "allAtOnce" => 0, "one" => 3, "lvlOne" => 2, "lvlAtOnce" => 1
        ))
    }

    pub fn get_animationChartBuildMap() -> &'static HashMap<&'static str, i32> {
        animationChartBuildMap.get_or_init(|| make_map!(
            "allAtOnce" => 0, "series" => 3, "category" => 1,
            "seriesEl" => 4, "categoryEl" => 2
        ))
    }

    pub fn get_iteratorTypeMap() -> &'static HashMap<&'static str, i32> {
        iteratorTypeMap.get_or_init(|| make_map!(
            "el" => 0, "wd" => 1, "lt" => 2
        ))
    }

    pub fn get_triggerRuntimeNodeMap() -> &'static HashMap<&'static str, i32> {
        triggerRuntimeNodeMap.get_or_init(|| make_map!(
            "first" => 1, "last" => 2, "all" => 0
        ))
    }

    pub fn get_photoAlbumFrameShapeMap() -> &'static HashMap<&'static str, i32> {
        photoAlbumFrameShapeMap.get_or_init(|| make_map!(
            "frameStyle1" => 0, "frameStyle2" => 1, "frameStyle3" => 2,
            "frameStyle4" => 3, "frameStyle5" => 4, "frameStyle6" => 5,
            "frameStyle7" => 6
        ))
    }

    pub fn get_photoAlbumLayoutMap() -> &'static HashMap<&'static str, i32> {
        photoAlbumLayoutMap.get_or_init(|| make_map!(
            "1pic" => 1, "1picTitle" => 4, "2pic" => 2, "2picTitle" => 5,
            "4pic" => 3, "4picTitle" => 6, "fitToSlide" => 0
        ))
    }

    pub fn get_viewTypeMap() -> &'static HashMap<&'static str, i32> {
        viewTypeMap.get_or_init(|| make_map!(
            "sldView" => 1, "sldMasterView" => 2, "notesView" => 3,
            "handoutView" => 4, "notesMasterView" => 5, "outlineView" => 7,
            "sldSorterView" => 8, "sldThumbnailView" => 15
        ))
    }

    pub fn get_splitterBarStateMap() -> &'static HashMap<&'static str, i32> {
        splitterBarStateMap.get_or_init(|| make_map!(
            "minimized" => 0, "restored" => 1, "maximized" => 2
        ))
    }

    pub fn get_behaviorAttributeNameMap() -> &'static HashMap<&'static str, i32> {
        behaviorAttributeNameMap.get_or_init(|| make_map!(
            "style.opacity" => 0, "style.rotation" => 1, "style.visibility" => 2,
            "style.color" => 3, "style.fontSize" => 4, "style.fontWeight" => 5,
            "style.fontStyle" => 6, "style.fontFamily" => 7,
            "style.textEffectEmboss" => 8, "style.textShadow" => 9,
            "style.textTransform" => 10, "style.textDecorationUnderline" => 11,
            "style.textEffectOutline" => 12, "style.textDecorationLineThrough" => 13,
            "style.sRotation" => 14,
            "imageData.cropTop" => 15, "imageData.cropBottom" => 16,
            "imageData.cropLeft" => 17, "imageData.cropRight" => 18,
            "imageData.gain" => 19, "imageData.blacklevel" => 20,
            "imageData.gamma" => 21, "imageData.grayscale" => 22,
            "imageData.chromakey" => 23,
            "fill.on" => 24, "fill.type" => 25,
            "fillColor" => 26, "fillcolor" => 26, "fill.color" => 26,
            "fill.opacity" => 27, "fill.color2" => 28, "fill.method" => 29,
            "fill.opacity2" => 30, "fill.angle" => 31, "fill.focus" => 32,
            "fill.focusposition.x" => 33, "fill.focusposition.y" => 34,
            "fill.focussize.x" => 35, "fill.focussize.y" => 36,
            "stroke.on" => 37, "stroke.color" => 38, "stroke.weight" => 39,
            "stroke.opacity" => 40, "stroke.linestyle" => 41,
            "stroke.dashstyle" => 42, "stroke.filltype" => 43,
            "stroke.src" => 44, "stroke.color2" => 45,
            "stroke.imagesize.x" => 46, "stroke.imagesize.y" => 47,
            "stroke.startArrow" => 48, "stroke.endArrow" => 49,
            "stroke.startArrowWidth" => 50, "stroke.startArrowLength" => 51,
            "stroke.endArrowWidth" => 52, "stroke.endArrowLength" => 53,
            "shadow.on" => 54, "shadow.type" => 55, "shadow.color" => 56,
            "shadow.color2" => 57, "shadow.opacity" => 58,
            "shadow.offset.x" => 59, "shadow.offset.y" => 60,
            "shadow.offset2.x" => 61, "shadow.offset2.y" => 62,
            "shadow.origin.x" => 63, "shadow.origin.y" => 64,
            "shadow.matrix.xtox" => 65, "shadow.matrix.ytox" => 66,
            "shadow.matrix.ytoy" => 67, "shadow.matrix.perspectiveX" => 68,
            "shadow.matrix.perspectiveY" => 69,
            "skew.on" => 70, "skew.offset.x" => 71, "skew.offset.y" => 72,
            "skew.origin.x" => 73, "skew.origin.y" => 74,
            "skew.matrix.xtox" => 75, "skew.matrix.ytox" => 76,
            "skew.matrix.ytoy" => 77, "skew.matrix.perspectiveX" => 78,
            "skew.matrix.perspectiveY" => 79,
            "extrusion.on" => 80, "extrusion.type" => 81,
            "extrusion.render" => 82,
            "extrusion.viewpointorigin.x" => 83, "extrusion.viewpointorigin.y" => 84,
            "extrusion.viewpoint.x" => 85, "extrusion.viewpoint.y" => 86,
            "extrusion.viewpoint.z" => 87,
            "extrusion.plane" => 88, "extrusion.skewangle" => 89,
            "extrusion.skewamt" => 90, "extrusion.backdepth" => 91,
            "extrusion.foredepth" => 92,
            "extrusion.orientation.x" => 93, "extrusion.orientation.y" => 94,
            "extrusion.orientation.z" => 95,
            "extrusion.orientationangle" => 96, "extrusion.color" => 97,
            "extrusion.rotationangle.x" => 98, "extrusion.rotationangle.y" => 99,
            "extrusion.lockrotationcenter" => 100,
            "extrusion.autorotationcenter" => 101,
            "extrusion.rotationcenter.x" => 102,
            "extrusion.rotationcenter.y" => 103,
            "extrusion.rotationcenter.z" => 104,
            "extrusion.colormode" => 105,
            "ppt_x" => 106, "ppt_y" => 107, "ppt_x,ppt_y" => 108,
            "ppt_w" => 109, "ppt_h" => 110, "ppt_c" => 111,
            "xshear" => 112, "r" => 113, "num.show" => 115
        ))
    }

    pub fn get_presetTransitionMap() -> &'static HashMap<&'static str, i32> {
        presetTransitionMap.get_or_init(|| make_map!(
            "fallOver" => 201, "drape" => 202, "curtains" => 203,
            "wind" => 204, "prestige" => 205, "fracture" => 206,
            "crush" => 207, "peelOff" => 208, "pageCurlDouble" => 209,
            "pageCurlSingle" => 210, "airplane" => 211, "origami" => 212
        ))
    }

    pub fn get_printWhatMap() -> &'static HashMap<&'static str, i32> {
        printWhatMap.get_or_init(|| make_map!(
            "slides" => 8, "handouts1" => 0, "handouts2" => 1,
            "handouts3" => 2, "handouts4" => 3, "handouts6" => 4,
            "handouts9" => 5, "notes" => 6, "outline" => 7
        ))
    }

    pub fn get_printColorModeMap() -> &'static HashMap<&'static str, i32> {
        printColorModeMap.get_or_init(|| make_map!(
            "bw" => 0, "gray" => 2, "clr" => 1
        ))
    }

    pub fn get_oleObjectFollowColorSchemeMap() -> &'static HashMap<&'static str, i32> {
        oleObjectFollowColorSchemeMap.get_or_init(|| make_map!(
            "full" => 0, "none" => 1, "textAndBackground" => 2
        ))
    }
}
