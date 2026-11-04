// Test 01_function_sw_reader_map — SWReaderMap 解析标签→枚举常数表
// 断言常量从 Java 字面量机械提取（SWReaderMap.java），不手工编造。
// picparser 并非本函数的可达依赖；Cargo 清单将其收窄为 opt-in 后，本测试直接验证真实实现。
use ppt_core::cn::wps::show::pptxr::util::SWReaderMap::SWReaderMap;
use std::collections::HashMap;

type MapGetter = fn() -> &'static HashMap<&'static str, i32>;

const ALL_MAP_GETTERS: [MapGetter; 54] = [
    SWReaderMap::get_conformanceClassMap,
    SWReaderMap::get_slideLayoutTypeMap,
    SWReaderMap::get_slideSizeMap,
    SWReaderMap::get_placeholderSizeMap,
    SWReaderMap::get_transitionSpeedMap,
    SWReaderMap::get_transitionSideDirectionMap,
    SWReaderMap::get_transitionCornerDirectionMap,
    SWReaderMap::get_transitionEightDirMap,
    SWReaderMap::get_transitionInOutDirectionMap,
    SWReaderMap::get_transitionLeftRightDirectionMap,
    SWReaderMap::get_transitionCornerAndCenterDirectionMap,
    SWReaderMap::get_transitionPatternMap,
    SWReaderMap::get_transitionShredPatternMap,
    SWReaderMap::get_placeholderTypeMap,
    SWReaderMap::get_timeNodeFillTypeMap,
    SWReaderMap::get_timeNodeMasterRelationMap,
    SWReaderMap::get_timeNodeTypeMap,
    SWReaderMap::get_timeNodePresetClassMap,
    SWReaderMap::get_timeNodeRestartTypeMap,
    SWReaderMap::get_timeNodeSyncTypeMap,
    SWReaderMap::get_animBehaviorCalcModeMap,
    SWReaderMap::get_animBehaviorValueTypeMap,
    SWReaderMap::get_behaviorAccumulateTypeMap,
    SWReaderMap::get_behaviorAdditiveTypeMap,
    SWReaderMap::get_behaviorOverrideTypeMap,
    SWReaderMap::get_behaviorTransformTypeMap,
    SWReaderMap::get_chartBuildStepMap,
    SWReaderMap::get_dgmBuildStepMap,
    SWReaderMap::get_chartSubelementType,
    SWReaderMap::get_animColorSpaceMap,
    SWReaderMap::get_animColorDirectionMap,
    SWReaderMap::get_animEffectTransitionMap,
    SWReaderMap::get_animMotionBehaviorOriginMap,
    SWReaderMap::get_animMotionPathEditModeMap,
    SWReaderMap::get_commandTypeMap,
    SWReaderMap::get_previousActionTypeMap,
    SWReaderMap::get_nextActionTypeMap,
    SWReaderMap::get_triggerEventMap,
    SWReaderMap::get_paraBuildMap,
    SWReaderMap::get_diagramBuildMap,
    SWReaderMap::get_oleChartBuildMap,
    SWReaderMap::get_animationDgmBuildMap,
    SWReaderMap::get_animationChartBuildMap,
    SWReaderMap::get_iteratorTypeMap,
    SWReaderMap::get_triggerRuntimeNodeMap,
    SWReaderMap::get_photoAlbumFrameShapeMap,
    SWReaderMap::get_photoAlbumLayoutMap,
    SWReaderMap::get_viewTypeMap,
    SWReaderMap::get_splitterBarStateMap,
    SWReaderMap::get_behaviorAttributeNameMap,
    SWReaderMap::get_presetTransitionMap,
    SWReaderMap::get_printWhatMap,
    SWReaderMap::get_printColorModeMap,
    SWReaderMap::get_oleObjectFollowColorSchemeMap,
];

#[test]
fn exposes_all_maps_as_stable_singletons() {
    assert_eq!(ALL_MAP_GETTERS.len(), 54);
    for getter in ALL_MAP_GETTERS {
        assert!(std::ptr::eq(getter(), getter()));
    }
}

#[test]
fn map_conformance_class_map() {
    let m = SWReaderMap::get_conformanceClassMap();
    assert_eq!(m.len(), 2, "conformanceClassMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("strict"), Some(&0), "conformanceClassMap[strict]");
    assert_eq!(
        m.get("transitional"),
        Some(&1),
        "conformanceClassMap[transitional]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "conformanceClassMap 不应含未知键"
    );
}

#[test]
fn map_slide_layout_type_map() {
    let m = SWReaderMap::get_slideLayoutTypeMap();
    assert_eq!(m.len(), 36, "slideLayoutTypeMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("blank"), Some(&16), "slideLayoutTypeMap[blank]");
    assert_eq!(m.get("chart"), Some(&19), "slideLayoutTypeMap[chart]");
    assert_eq!(
        m.get("chartAndTx"),
        Some(&20),
        "slideLayoutTypeMap[chartAndTx]"
    );
    assert_eq!(
        m.get("clipArtAndTx"),
        Some(&21),
        "slideLayoutTypeMap[clipArtAndTx]"
    );
    assert_eq!(
        m.get("clipArtAndVertTx"),
        Some(&22),
        "slideLayoutTypeMap[clipArtAndVertTx]"
    );
    assert_eq!(m.get("cust"), Some(&23), "slideLayoutTypeMap[cust]");
    assert_eq!(m.get("dgm"), Some(&24), "slideLayoutTypeMap[dgm]");
    assert_eq!(m.get("fourObj"), Some(&14), "slideLayoutTypeMap[fourObj]");
    assert_eq!(
        m.get("mediaAndTx"),
        Some(&25),
        "slideLayoutTypeMap[mediaAndTx]"
    );
    assert_eq!(m.get("obj"), Some(&26), "slideLayoutTypeMap[obj]");
    assert_eq!(
        m.get("objAndTwoObj"),
        Some(&27),
        "slideLayoutTypeMap[objAndTwoObj]"
    );
    assert_eq!(m.get("objAndTx"), Some(&28), "slideLayoutTypeMap[objAndTx]");
    assert_eq!(m.get("objOnly"), Some(&15), "slideLayoutTypeMap[objOnly]");
    assert_eq!(
        m.get("objOverTx"),
        Some(&29),
        "slideLayoutTypeMap[objOverTx]"
    );
    assert_eq!(m.get("objTx"), Some(&31), "slideLayoutTypeMap[objTx]");
    assert_eq!(m.get("picTx"), Some(&32), "slideLayoutTypeMap[picTx]");
    assert_eq!(m.get("secHead"), Some(&33), "slideLayoutTypeMap[secHead]");
    assert_eq!(m.get("tbl"), Some(&34), "slideLayoutTypeMap[tbl]");
    assert_eq!(m.get("title"), Some(&0), "slideLayoutTypeMap[title]");
    assert_eq!(
        m.get("titleOnly"),
        Some(&7),
        "slideLayoutTypeMap[titleOnly]"
    );
    assert_eq!(m.get("twoColTx"), Some(&8), "slideLayoutTypeMap[twoColTx]");
    assert_eq!(m.get("twoObj"), Some(&35), "slideLayoutTypeMap[twoObj]");
    assert_eq!(
        m.get("twoObjAndObj"),
        Some(&36),
        "slideLayoutTypeMap[twoObjAndObj]"
    );
    assert_eq!(
        m.get("twoObjAndTx"),
        Some(&11),
        "slideLayoutTypeMap[twoObjAndTx]"
    );
    assert_eq!(
        m.get("twoObjOverTx"),
        Some(&13),
        "slideLayoutTypeMap[twoObjOverTx]"
    );
    assert_eq!(
        m.get("twoTxTwoObj"),
        Some(&37),
        "slideLayoutTypeMap[twoTxTwoObj]"
    );
    assert_eq!(m.get("tx"), Some(&1), "slideLayoutTypeMap[tx]");
    assert_eq!(
        m.get("txAndChart"),
        Some(&38),
        "slideLayoutTypeMap[txAndChart]"
    );
    assert_eq!(
        m.get("txAndClipArt"),
        Some(&39),
        "slideLayoutTypeMap[txAndClipArt]"
    );
    assert_eq!(
        m.get("txAndMedia"),
        Some(&40),
        "slideLayoutTypeMap[txAndMedia]"
    );
    assert_eq!(m.get("txAndObj"), Some(&41), "slideLayoutTypeMap[txAndObj]");
    assert_eq!(
        m.get("txAndTwoObj"),
        Some(&10),
        "slideLayoutTypeMap[txAndTwoObj]"
    );
    assert_eq!(
        m.get("txOverObj"),
        Some(&42),
        "slideLayoutTypeMap[txOverObj]"
    );
    assert_eq!(
        m.get("vertTitleAndTx"),
        Some(&17),
        "slideLayoutTypeMap[vertTitleAndTx]"
    );
    assert_eq!(
        m.get("vertTitleAndTxOverChart"),
        Some(&18),
        "slideLayoutTypeMap[vertTitleAndTxOverChart]"
    );
    assert_eq!(m.get("vertTx"), Some(&43), "slideLayoutTypeMap[vertTx]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "slideLayoutTypeMap 不应含未知键"
    );
}

#[test]
fn map_slide_size_map() {
    let m = SWReaderMap::get_slideSizeMap();
    assert_eq!(m.len(), 16, "slideSizeMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("35mm"), Some(&3), "slideSizeMap[35mm]");
    assert_eq!(m.get("A3"), Some(&7), "slideSizeMap[A3]");
    assert_eq!(m.get("A4"), Some(&2), "slideSizeMap[A4]");
    assert_eq!(m.get("B4ISO"), Some(&8), "slideSizeMap[B4ISO]");
    assert_eq!(m.get("B4JIS"), Some(&9), "slideSizeMap[B4JIS]");
    assert_eq!(m.get("B5ISO"), Some(&10), "slideSizeMap[B5ISO]");
    assert_eq!(m.get("B5JIS"), Some(&11), "slideSizeMap[B5JIS]");
    assert_eq!(m.get("banner"), Some(&5), "slideSizeMap[banner]");
    assert_eq!(m.get("custom"), Some(&6), "slideSizeMap[custom]");
    assert_eq!(m.get("hagakiCard"), Some(&12), "slideSizeMap[hagakiCard]");
    assert_eq!(m.get("ledger"), Some(&13), "slideSizeMap[ledger]");
    assert_eq!(m.get("letter"), Some(&1), "slideSizeMap[letter]");
    assert_eq!(m.get("overhead"), Some(&4), "slideSizeMap[overhead]");
    assert_eq!(m.get("screen16x10"), Some(&14), "slideSizeMap[screen16x10]");
    assert_eq!(m.get("screen16x9"), Some(&15), "slideSizeMap[screen16x9]");
    assert_eq!(m.get("screen4x3"), Some(&0), "slideSizeMap[screen4x3]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "slideSizeMap 不应含未知键"
    );
}

#[test]
fn map_placeholder_size_map() {
    let m = SWReaderMap::get_placeholderSizeMap();
    assert_eq!(m.len(), 3, "placeholderSizeMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("full"), Some(&0), "placeholderSizeMap[full]");
    assert_eq!(m.get("half"), Some(&1), "placeholderSizeMap[half]");
    assert_eq!(m.get("quarter"), Some(&2), "placeholderSizeMap[quarter]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "placeholderSizeMap 不应含未知键"
    );
}

#[test]
fn map_transition_speed_map() {
    let m = SWReaderMap::get_transitionSpeedMap();
    assert_eq!(m.len(), 3, "transitionSpeedMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("slow"), Some(&0), "transitionSpeedMap[slow]");
    assert_eq!(m.get("med"), Some(&1), "transitionSpeedMap[med]");
    assert_eq!(m.get("fast"), Some(&2), "transitionSpeedMap[fast]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "transitionSpeedMap 不应含未知键"
    );
}

#[test]
fn map_transition_side_direction_map() {
    let m = SWReaderMap::get_transitionSideDirectionMap();
    assert_eq!(
        m.len(),
        4,
        "transitionSideDirectionMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("l"), Some(&0), "transitionSideDirectionMap[l]");
    assert_eq!(m.get("u"), Some(&1), "transitionSideDirectionMap[u]");
    assert_eq!(m.get("r"), Some(&2), "transitionSideDirectionMap[r]");
    assert_eq!(m.get("d"), Some(&3), "transitionSideDirectionMap[d]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "transitionSideDirectionMap 不应含未知键"
    );
}

#[test]
fn map_transition_corner_direction_map() {
    let m = SWReaderMap::get_transitionCornerDirectionMap();
    assert_eq!(
        m.len(),
        4,
        "transitionCornerDirectionMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("lu"), Some(&4), "transitionCornerDirectionMap[lu]");
    assert_eq!(m.get("ru"), Some(&5), "transitionCornerDirectionMap[ru]");
    assert_eq!(m.get("rd"), Some(&7), "transitionCornerDirectionMap[rd]");
    assert_eq!(m.get("ld"), Some(&6), "transitionCornerDirectionMap[ld]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "transitionCornerDirectionMap 不应含未知键"
    );
}

#[test]
fn map_transition_eight_dir_map() {
    let m = SWReaderMap::get_transitionEightDirMap();
    assert_eq!(
        m.len(),
        8,
        "transitionEightDirMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("l"), Some(&0), "transitionEightDirMap[l]");
    assert_eq!(m.get("u"), Some(&1), "transitionEightDirMap[u]");
    assert_eq!(m.get("r"), Some(&2), "transitionEightDirMap[r]");
    assert_eq!(m.get("d"), Some(&3), "transitionEightDirMap[d]");
    assert_eq!(m.get("lu"), Some(&4), "transitionEightDirMap[lu]");
    assert_eq!(m.get("ru"), Some(&5), "transitionEightDirMap[ru]");
    assert_eq!(m.get("rd"), Some(&7), "transitionEightDirMap[rd]");
    assert_eq!(m.get("ld"), Some(&6), "transitionEightDirMap[ld]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "transitionEightDirMap 不应含未知键"
    );
}

#[test]
fn map_transition_in_out_direction_map() {
    let m = SWReaderMap::get_transitionInOutDirectionMap();
    assert_eq!(
        m.len(),
        2,
        "transitionInOutDirectionMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("out"), Some(&0), "transitionInOutDirectionMap[out]");
    assert_eq!(m.get("in"), Some(&1), "transitionInOutDirectionMap[in]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "transitionInOutDirectionMap 不应含未知键"
    );
}

#[test]
fn map_transition_left_right_direction_map() {
    let m = SWReaderMap::get_transitionLeftRightDirectionMap();
    assert_eq!(
        m.len(),
        2,
        "transitionLeftRightDirectionMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("l"), Some(&0), "transitionLeftRightDirectionMap[l]");
    assert_eq!(m.get("r"), Some(&1), "transitionLeftRightDirectionMap[r]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "transitionLeftRightDirectionMap 不应含未知键"
    );
}

#[test]
fn map_transition_corner_and_center_direction_map() {
    let m = SWReaderMap::get_transitionCornerAndCenterDirectionMap();
    assert_eq!(
        m.len(),
        5,
        "transitionCornerAndCenterDirectionMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(
        m.get("center"),
        Some(&0),
        "transitionCornerAndCenterDirectionMap[center]"
    );
    assert_eq!(
        m.get("lu"),
        Some(&2),
        "transitionCornerAndCenterDirectionMap[lu]"
    );
    assert_eq!(
        m.get("ru"),
        Some(&4),
        "transitionCornerAndCenterDirectionMap[ru]"
    );
    assert_eq!(
        m.get("rd"),
        Some(&3),
        "transitionCornerAndCenterDirectionMap[rd]"
    );
    assert_eq!(
        m.get("ld"),
        Some(&1),
        "transitionCornerAndCenterDirectionMap[ld]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "transitionCornerAndCenterDirectionMap 不应含未知键"
    );
}

#[test]
fn map_transition_pattern_map() {
    let m = SWReaderMap::get_transitionPatternMap();
    assert_eq!(
        m.len(),
        2,
        "transitionPatternMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("diamond"), Some(&1), "transitionPatternMap[diamond]");
    assert_eq!(m.get("hexagon"), Some(&0), "transitionPatternMap[hexagon]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "transitionPatternMap 不应含未知键"
    );
}

#[test]
fn map_transition_shred_pattern_map() {
    let m = SWReaderMap::get_transitionShredPatternMap();
    assert_eq!(
        m.len(),
        2,
        "transitionShredPatternMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("strip"), Some(&1), "transitionShredPatternMap[strip]");
    assert_eq!(
        m.get("rectangle"),
        Some(&0),
        "transitionShredPatternMap[rectangle]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "transitionShredPatternMap 不应含未知键"
    );
}

#[test]
fn map_placeholder_type_map() {
    let m = SWReaderMap::get_placeholderTypeMap();
    assert_eq!(m.len(), 16, "placeholderTypeMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("body"), Some(&1), "placeholderTypeMap[body]");
    assert_eq!(m.get("chart"), Some(&9), "placeholderTypeMap[chart]");
    assert_eq!(m.get("clipArt"), Some(&11), "placeholderTypeMap[clipArt]");
    assert_eq!(m.get("ctrTitle"), Some(&2), "placeholderTypeMap[ctrTitle]");
    assert_eq!(m.get("dgm"), Some(&12), "placeholderTypeMap[dgm]");
    assert_eq!(m.get("dt"), Some(&4), "placeholderTypeMap[dt]");
    assert_eq!(m.get("ftr"), Some(&6), "placeholderTypeMap[ftr]");
    assert_eq!(m.get("hdr"), Some(&7), "placeholderTypeMap[hdr]");
    assert_eq!(m.get("media"), Some(&13), "placeholderTypeMap[media]");
    assert_eq!(m.get("obj"), Some(&8), "placeholderTypeMap[obj]");
    assert_eq!(m.get("pic"), Some(&15), "placeholderTypeMap[pic]");
    assert_eq!(m.get("sldImg"), Some(&14), "placeholderTypeMap[sldImg]");
    assert_eq!(m.get("sldNum"), Some(&5), "placeholderTypeMap[sldNum]");
    assert_eq!(m.get("subTitle"), Some(&3), "placeholderTypeMap[subTitle]");
    assert_eq!(m.get("tbl"), Some(&10), "placeholderTypeMap[tbl]");
    assert_eq!(m.get("title"), Some(&0), "placeholderTypeMap[title]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "placeholderTypeMap 不应含未知键"
    );
}

#[test]
fn map_time_node_fill_type_map() {
    let m = SWReaderMap::get_timeNodeFillTypeMap();
    assert_eq!(m.len(), 4, "timeNodeFillTypeMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("freeze"), Some(&2), "timeNodeFillTypeMap[freeze]");
    assert_eq!(m.get("hold"), Some(&3), "timeNodeFillTypeMap[hold]");
    assert_eq!(m.get("remove"), Some(&1), "timeNodeFillTypeMap[remove]");
    assert_eq!(
        m.get("transition"),
        Some(&4),
        "timeNodeFillTypeMap[transition]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "timeNodeFillTypeMap 不应含未知键"
    );
}

#[test]
fn map_time_node_master_relation_map() {
    let m = SWReaderMap::get_timeNodeMasterRelationMap();
    assert_eq!(
        m.len(),
        3,
        "timeNodeMasterRelationMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(
        m.get("sameClick"),
        Some(&0),
        "timeNodeMasterRelationMap[sameClick]"
    );
    assert_eq!(
        m.get("lastClick"),
        Some(&1),
        "timeNodeMasterRelationMap[lastClick]"
    );
    assert_eq!(
        m.get("nextClick"),
        Some(&2),
        "timeNodeMasterRelationMap[nextClick]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "timeNodeMasterRelationMap 不应含未知键"
    );
}

#[test]
fn map_time_node_type_map() {
    let m = SWReaderMap::get_timeNodeTypeMap();
    assert_eq!(m.len(), 9, "timeNodeTypeMap 条目数与 Java 字面量不一致");
    assert_eq!(
        m.get("clickEffect"),
        Some(&1),
        "timeNodeTypeMap[clickEffect]"
    );
    assert_eq!(m.get("withEffect"), Some(&2), "timeNodeTypeMap[withEffect]");
    assert_eq!(
        m.get("afterEffect"),
        Some(&3),
        "timeNodeTypeMap[afterEffect]"
    );
    assert_eq!(m.get("mainSeq"), Some(&4), "timeNodeTypeMap[mainSeq]");
    assert_eq!(
        m.get("interactiveSeq"),
        Some(&5),
        "timeNodeTypeMap[interactiveSeq]"
    );
    assert_eq!(m.get("clickPar"), Some(&6), "timeNodeTypeMap[clickPar]");
    assert_eq!(m.get("withGroup"), Some(&7), "timeNodeTypeMap[withGroup]");
    assert_eq!(m.get("afterGroup"), Some(&8), "timeNodeTypeMap[afterGroup]");
    assert_eq!(m.get("tmRoot"), Some(&9), "timeNodeTypeMap[tmRoot]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "timeNodeTypeMap 不应含未知键"
    );
}

#[test]
fn map_time_node_preset_class_map() {
    let m = SWReaderMap::get_timeNodePresetClassMap();
    assert_eq!(
        m.len(),
        7,
        "timeNodePresetClassMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(
        m.get("undefine"),
        Some(&0),
        "timeNodePresetClassMap[undefine]"
    );
    assert_eq!(m.get("entr"), Some(&1), "timeNodePresetClassMap[entr]");
    assert_eq!(m.get("exit"), Some(&2), "timeNodePresetClassMap[exit]");
    assert_eq!(m.get("emph"), Some(&3), "timeNodePresetClassMap[emph]");
    assert_eq!(m.get("path"), Some(&4), "timeNodePresetClassMap[path]");
    assert_eq!(m.get("verb"), Some(&5), "timeNodePresetClassMap[verb]");
    assert_eq!(
        m.get("mediacall"),
        Some(&6),
        "timeNodePresetClassMap[mediacall]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "timeNodePresetClassMap 不应含未知键"
    );
}

#[test]
fn map_time_node_restart_type_map() {
    let m = SWReaderMap::get_timeNodeRestartTypeMap();
    assert_eq!(
        m.len(),
        3,
        "timeNodeRestartTypeMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("always"), Some(&1), "timeNodeRestartTypeMap[always]");
    assert_eq!(
        m.get("whenNotActive"),
        Some(&2),
        "timeNodeRestartTypeMap[whenNotActive]"
    );
    assert_eq!(m.get("never"), Some(&0), "timeNodeRestartTypeMap[never]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "timeNodeRestartTypeMap 不应含未知键"
    );
}

#[test]
fn map_time_node_sync_type_map() {
    let m = SWReaderMap::get_timeNodeSyncTypeMap();
    assert_eq!(m.len(), 2, "timeNodeSyncTypeMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("canSlip"), Some(&0), "timeNodeSyncTypeMap[canSlip]");
    assert_eq!(m.get("locked"), Some(&1), "timeNodeSyncTypeMap[locked]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "timeNodeSyncTypeMap 不应含未知键"
    );
}

#[test]
fn map_anim_behavior_calc_mode_map() {
    let m = SWReaderMap::get_animBehaviorCalcModeMap();
    assert_eq!(
        m.len(),
        3,
        "animBehaviorCalcModeMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(
        m.get("discrete"),
        Some(&0),
        "animBehaviorCalcModeMap[discrete]"
    );
    assert_eq!(m.get("lin"), Some(&1), "animBehaviorCalcModeMap[lin]");
    assert_eq!(m.get("fmla"), Some(&2), "animBehaviorCalcModeMap[fmla]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "animBehaviorCalcModeMap 不应含未知键"
    );
}

#[test]
fn map_anim_behavior_value_type_map() {
    let m = SWReaderMap::get_animBehaviorValueTypeMap();
    assert_eq!(
        m.len(),
        3,
        "animBehaviorValueTypeMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("str"), Some(&0), "animBehaviorValueTypeMap[str]");
    assert_eq!(m.get("num"), Some(&1), "animBehaviorValueTypeMap[num]");
    assert_eq!(m.get("clr"), Some(&2), "animBehaviorValueTypeMap[clr]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "animBehaviorValueTypeMap 不应含未知键"
    );
}

#[test]
fn map_behavior_accumulate_type_map() {
    let m = SWReaderMap::get_behaviorAccumulateTypeMap();
    assert_eq!(
        m.len(),
        2,
        "behaviorAccumulateTypeMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("none"), Some(&0), "behaviorAccumulateTypeMap[none]");
    assert_eq!(
        m.get("always"),
        Some(&1),
        "behaviorAccumulateTypeMap[always]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "behaviorAccumulateTypeMap 不应含未知键"
    );
}

#[test]
fn map_behavior_additive_type_map() {
    let m = SWReaderMap::get_behaviorAdditiveTypeMap();
    assert_eq!(
        m.len(),
        5,
        "behaviorAdditiveTypeMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("base"), Some(&0), "behaviorAdditiveTypeMap[base]");
    assert_eq!(m.get("sum"), Some(&4), "behaviorAdditiveTypeMap[sum]");
    assert_eq!(m.get("repl"), Some(&3), "behaviorAdditiveTypeMap[repl]");
    assert_eq!(m.get("mult"), Some(&1), "behaviorAdditiveTypeMap[mult]");
    assert_eq!(m.get("none"), Some(&2), "behaviorAdditiveTypeMap[none]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "behaviorAdditiveTypeMap 不应含未知键"
    );
}

#[test]
fn map_behavior_override_type_map() {
    let m = SWReaderMap::get_behaviorOverrideTypeMap();
    assert_eq!(
        m.len(),
        2,
        "behaviorOverrideTypeMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("normal"), Some(&0), "behaviorOverrideTypeMap[normal]");
    assert_eq!(
        m.get("childStyle"),
        Some(&1),
        "behaviorOverrideTypeMap[childStyle]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "behaviorOverrideTypeMap 不应含未知键"
    );
}

#[test]
fn map_behavior_transform_type_map() {
    let m = SWReaderMap::get_behaviorTransformTypeMap();
    assert_eq!(
        m.len(),
        2,
        "behaviorTransformTypeMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("image"), Some(&0), "behaviorTransformTypeMap[image]");
    assert_eq!(m.get("img"), Some(&0), "behaviorTransformTypeMap[img]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "behaviorTransformTypeMap 不应含未知键"
    );
}

#[test]
fn map_chart_build_step_map() {
    let m = SWReaderMap::get_chartBuildStepMap();
    assert_eq!(m.len(), 6, "chartBuildStepMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("category"), Some(&1), "chartBuildStepMap[category]");
    assert_eq!(
        m.get("ptInCategory"),
        Some(&3),
        "chartBuildStepMap[ptInCategory]"
    );
    assert_eq!(m.get("series"), Some(&5), "chartBuildStepMap[series]");
    assert_eq!(
        m.get("ptInSeries"),
        Some(&4),
        "chartBuildStepMap[ptInSeries]"
    );
    assert_eq!(m.get("allPts"), Some(&0), "chartBuildStepMap[allPts]");
    assert_eq!(
        m.get("gridLegend"),
        Some(&2),
        "chartBuildStepMap[gridLegend]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "chartBuildStepMap 不应含未知键"
    );
}

#[test]
fn map_dgm_build_step_map() {
    let m = SWReaderMap::get_dgmBuildStepMap();
    assert_eq!(m.len(), 2, "dgmBuildStepMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("sp"), Some(&1), "dgmBuildStepMap[sp]");
    assert_eq!(m.get("bg"), Some(&0), "dgmBuildStepMap[bg]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "dgmBuildStepMap 不应含未知键"
    );
}

#[test]
fn map_chart_subelement_type() {
    let m = SWReaderMap::get_chartSubelementType();
    assert_eq!(m.len(), 5, "chartSubelementType 条目数与 Java 字面量不一致");
    assert_eq!(
        m.get("gridLegend"),
        Some(&1),
        "chartSubelementType[gridLegend]"
    );
    assert_eq!(m.get("series"), Some(&4), "chartSubelementType[series]");
    assert_eq!(m.get("category"), Some(&0), "chartSubelementType[category]");
    assert_eq!(
        m.get("ptInSeries"),
        Some(&3),
        "chartSubelementType[ptInSeries]"
    );
    assert_eq!(
        m.get("ptInCategory"),
        Some(&2),
        "chartSubelementType[ptInCategory]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "chartSubelementType 不应含未知键"
    );
}

#[test]
fn map_anim_color_space_map() {
    let m = SWReaderMap::get_animColorSpaceMap();
    assert_eq!(m.len(), 2, "animColorSpaceMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("rgb"), Some(&0), "animColorSpaceMap[rgb]");
    assert_eq!(m.get("hsl"), Some(&1), "animColorSpaceMap[hsl]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "animColorSpaceMap 不应含未知键"
    );
}

#[test]
fn map_anim_color_direction_map() {
    let m = SWReaderMap::get_animColorDirectionMap();
    assert_eq!(
        m.len(),
        2,
        "animColorDirectionMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("cw"), Some(&0), "animColorDirectionMap[cw]");
    assert_eq!(m.get("ccw"), Some(&1), "animColorDirectionMap[ccw]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "animColorDirectionMap 不应含未知键"
    );
}

#[test]
fn map_anim_effect_transition_map() {
    let m = SWReaderMap::get_animEffectTransitionMap();
    assert_eq!(
        m.len(),
        3,
        "animEffectTransitionMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("in"), Some(&0), "animEffectTransitionMap[in]");
    assert_eq!(m.get("out"), Some(&1), "animEffectTransitionMap[out]");
    assert_eq!(m.get("none"), Some(&2), "animEffectTransitionMap[none]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "animEffectTransitionMap 不应含未知键"
    );
}

#[test]
fn map_anim_motion_behavior_origin_map() {
    let m = SWReaderMap::get_animMotionBehaviorOriginMap();
    assert_eq!(
        m.len(),
        2,
        "animMotionBehaviorOriginMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(
        m.get("layout"),
        Some(&2),
        "animMotionBehaviorOriginMap[layout]"
    );
    assert_eq!(
        m.get("parent"),
        Some(&1),
        "animMotionBehaviorOriginMap[parent]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "animMotionBehaviorOriginMap 不应含未知键"
    );
}

#[test]
fn map_anim_motion_path_edit_mode_map() {
    let m = SWReaderMap::get_animMotionPathEditModeMap();
    assert_eq!(
        m.len(),
        2,
        "animMotionPathEditModeMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("fixed"), Some(&0), "animMotionPathEditModeMap[fixed]");
    assert_eq!(
        m.get("relative"),
        Some(&1),
        "animMotionPathEditModeMap[relative]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "animMotionPathEditModeMap 不应含未知键"
    );
}

#[test]
fn map_command_type_map() {
    let m = SWReaderMap::get_commandTypeMap();
    assert_eq!(m.len(), 3, "commandTypeMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("call"), Some(&1), "commandTypeMap[call]");
    assert_eq!(m.get("evt"), Some(&0), "commandTypeMap[evt]");
    assert_eq!(m.get("verb"), Some(&2), "commandTypeMap[verb]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "commandTypeMap 不应含未知键"
    );
}

#[test]
fn map_previous_action_type_map() {
    let m = SWReaderMap::get_previousActionTypeMap();
    assert_eq!(
        m.len(),
        2,
        "previousActionTypeMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("none"), Some(&0), "previousActionTypeMap[none]");
    assert_eq!(
        m.get("skipTimed"),
        Some(&1),
        "previousActionTypeMap[skipTimed]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "previousActionTypeMap 不应含未知键"
    );
}

#[test]
fn map_next_action_type_map() {
    let m = SWReaderMap::get_nextActionTypeMap();
    assert_eq!(m.len(), 2, "nextActionTypeMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("none"), Some(&0), "nextActionTypeMap[none]");
    assert_eq!(m.get("seek"), Some(&1), "nextActionTypeMap[seek]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "nextActionTypeMap 不应含未知键"
    );
}

#[test]
fn map_trigger_event_map() {
    let m = SWReaderMap::get_triggerEventMap();
    assert_eq!(m.len(), 12, "triggerEventMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("onBegin"), Some(&1), "triggerEventMap[onBegin]");
    assert_eq!(m.get("onEnd"), Some(&2), "triggerEventMap[onEnd]");
    assert_eq!(m.get("begin"), Some(&3), "triggerEventMap[begin]");
    assert_eq!(m.get("end"), Some(&4), "triggerEventMap[end]");
    assert_eq!(m.get("onClick"), Some(&5), "triggerEventMap[onClick]");
    assert_eq!(m.get("onDblClick"), Some(&6), "triggerEventMap[onDblClick]");
    assert_eq!(
        m.get("onMouseOver"),
        Some(&7),
        "triggerEventMap[onMouseOver]"
    );
    assert_eq!(m.get("onMouseOut"), Some(&8), "triggerEventMap[onMouseOut]");
    assert_eq!(m.get("onNext"), Some(&9), "triggerEventMap[onNext]");
    assert_eq!(m.get("onPrev"), Some(&10), "triggerEventMap[onPrev]");
    assert_eq!(
        m.get("onStopAudio"),
        Some(&11),
        "triggerEventMap[onStopAudio]"
    );
    assert_eq!(
        m.get("onMediaBookmark"),
        Some(&12),
        "triggerEventMap[onMediaBookmark]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "triggerEventMap 不应含未知键"
    );
}

#[test]
fn map_para_build_map() {
    let m = SWReaderMap::get_paraBuildMap();
    assert_eq!(m.len(), 4, "paraBuildMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("allAtOnce"), Some(&0), "paraBuildMap[allAtOnce]");
    assert_eq!(m.get("p"), Some(&1), "paraBuildMap[p]");
    assert_eq!(m.get("cust"), Some(&2), "paraBuildMap[cust]");
    assert_eq!(m.get("whole"), Some(&3), "paraBuildMap[whole]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "paraBuildMap 不应含未知键"
    );
}

#[test]
fn map_diagram_build_map() {
    let m = SWReaderMap::get_diagramBuildMap();
    // Java 对 "cw" 连续 put 两次；HashMap 的可观察结果是 17 个唯一键。
    assert_eq!(
        m.len(),
        17,
        "diagramBuildMap 唯一键数与 Java HashMap 语义不一致"
    );
    assert_eq!(m.get("whole"), Some(&16), "diagramBuildMap[whole]");
    assert_eq!(
        m.get("depthByNode"),
        Some(&11),
        "diagramBuildMap[depthByNode]"
    );
    assert_eq!(
        m.get("depthByBranch"),
        Some(&10),
        "diagramBuildMap[depthByBranch]"
    );
    assert_eq!(
        m.get("breadthByNode"),
        Some(&2),
        "diagramBuildMap[breadthByNode]"
    );
    assert_eq!(
        m.get("breadthByLvl"),
        Some(&1),
        "diagramBuildMap[breadthByLvl]"
    );
    assert_eq!(m.get("cwIn"), Some(&8), "diagramBuildMap[cwIn]");
    assert_eq!(m.get("cwOut"), Some(&9), "diagramBuildMap[cwOut]");
    assert_eq!(m.get("ccw"), Some(&3), "diagramBuildMap[ccw]");
    assert_eq!(m.get("cw"), Some(&7), "diagramBuildMap[cw]");
    assert_eq!(m.get("ccwIn"), Some(&4), "diagramBuildMap[ccwIn]");
    assert_eq!(m.get("ccwOut"), Some(&5), "diagramBuildMap[ccwOut]");
    assert_eq!(m.get("inByRing"), Some(&13), "diagramBuildMap[inByRing]");
    assert_eq!(m.get("outByRing"), Some(&14), "diagramBuildMap[outByRing]");
    assert_eq!(m.get("up"), Some(&15), "diagramBuildMap[up]");
    assert_eq!(m.get("down"), Some(&12), "diagramBuildMap[down]");
    assert_eq!(m.get("allAtOnce"), Some(&0), "diagramBuildMap[allAtOnce]");
    assert_eq!(m.get("cust"), Some(&6), "diagramBuildMap[cust]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "diagramBuildMap 不应含未知键"
    );
}

#[test]
fn map_ole_chart_build_map() {
    let m = SWReaderMap::get_oleChartBuildMap();
    assert_eq!(m.len(), 5, "oleChartBuildMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("allAtOnce"), Some(&0), "oleChartBuildMap[allAtOnce]");
    assert_eq!(m.get("series"), Some(&3), "oleChartBuildMap[series]");
    assert_eq!(m.get("category"), Some(&1), "oleChartBuildMap[category]");
    assert_eq!(m.get("seriesEl"), Some(&4), "oleChartBuildMap[seriesEl]");
    assert_eq!(
        m.get("categoryEl"),
        Some(&2),
        "oleChartBuildMap[categoryEl]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "oleChartBuildMap 不应含未知键"
    );
}

#[test]
fn map_animation_dgm_build_map() {
    let m = SWReaderMap::get_animationDgmBuildMap();
    assert_eq!(
        m.len(),
        4,
        "animationDgmBuildMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(
        m.get("allAtOnce"),
        Some(&0),
        "animationDgmBuildMap[allAtOnce]"
    );
    assert_eq!(m.get("one"), Some(&3), "animationDgmBuildMap[one]");
    assert_eq!(m.get("lvlOne"), Some(&2), "animationDgmBuildMap[lvlOne]");
    assert_eq!(
        m.get("lvlAtOnce"),
        Some(&1),
        "animationDgmBuildMap[lvlAtOnce]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "animationDgmBuildMap 不应含未知键"
    );
}

#[test]
fn map_animation_chart_build_map() {
    let m = SWReaderMap::get_animationChartBuildMap();
    assert_eq!(
        m.len(),
        5,
        "animationChartBuildMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(
        m.get("allAtOnce"),
        Some(&0),
        "animationChartBuildMap[allAtOnce]"
    );
    assert_eq!(m.get("series"), Some(&3), "animationChartBuildMap[series]");
    assert_eq!(
        m.get("category"),
        Some(&1),
        "animationChartBuildMap[category]"
    );
    assert_eq!(
        m.get("seriesEl"),
        Some(&4),
        "animationChartBuildMap[seriesEl]"
    );
    assert_eq!(
        m.get("categoryEl"),
        Some(&2),
        "animationChartBuildMap[categoryEl]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "animationChartBuildMap 不应含未知键"
    );
}

#[test]
fn map_iterator_type_map() {
    let m = SWReaderMap::get_iteratorTypeMap();
    assert_eq!(m.len(), 3, "iteratorTypeMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("el"), Some(&0), "iteratorTypeMap[el]");
    assert_eq!(m.get("wd"), Some(&1), "iteratorTypeMap[wd]");
    assert_eq!(m.get("lt"), Some(&2), "iteratorTypeMap[lt]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "iteratorTypeMap 不应含未知键"
    );
}

#[test]
fn map_trigger_runtime_node_map() {
    let m = SWReaderMap::get_triggerRuntimeNodeMap();
    assert_eq!(
        m.len(),
        3,
        "triggerRuntimeNodeMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(m.get("first"), Some(&1), "triggerRuntimeNodeMap[first]");
    assert_eq!(m.get("last"), Some(&2), "triggerRuntimeNodeMap[last]");
    assert_eq!(m.get("all"), Some(&0), "triggerRuntimeNodeMap[all]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "triggerRuntimeNodeMap 不应含未知键"
    );
}

#[test]
fn map_photo_album_frame_shape_map() {
    let m = SWReaderMap::get_photoAlbumFrameShapeMap();
    assert_eq!(
        m.len(),
        7,
        "photoAlbumFrameShapeMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(
        m.get("frameStyle1"),
        Some(&0),
        "photoAlbumFrameShapeMap[frameStyle1]"
    );
    assert_eq!(
        m.get("frameStyle2"),
        Some(&1),
        "photoAlbumFrameShapeMap[frameStyle2]"
    );
    assert_eq!(
        m.get("frameStyle3"),
        Some(&2),
        "photoAlbumFrameShapeMap[frameStyle3]"
    );
    assert_eq!(
        m.get("frameStyle4"),
        Some(&3),
        "photoAlbumFrameShapeMap[frameStyle4]"
    );
    assert_eq!(
        m.get("frameStyle5"),
        Some(&4),
        "photoAlbumFrameShapeMap[frameStyle5]"
    );
    assert_eq!(
        m.get("frameStyle6"),
        Some(&5),
        "photoAlbumFrameShapeMap[frameStyle6]"
    );
    assert_eq!(
        m.get("frameStyle7"),
        Some(&6),
        "photoAlbumFrameShapeMap[frameStyle7]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "photoAlbumFrameShapeMap 不应含未知键"
    );
}

#[test]
fn map_photo_album_layout_map() {
    let m = SWReaderMap::get_photoAlbumLayoutMap();
    assert_eq!(m.len(), 7, "photoAlbumLayoutMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("1pic"), Some(&1), "photoAlbumLayoutMap[1pic]");
    assert_eq!(
        m.get("1picTitle"),
        Some(&4),
        "photoAlbumLayoutMap[1picTitle]"
    );
    assert_eq!(m.get("2pic"), Some(&2), "photoAlbumLayoutMap[2pic]");
    assert_eq!(
        m.get("2picTitle"),
        Some(&5),
        "photoAlbumLayoutMap[2picTitle]"
    );
    assert_eq!(m.get("4pic"), Some(&3), "photoAlbumLayoutMap[4pic]");
    assert_eq!(
        m.get("4picTitle"),
        Some(&6),
        "photoAlbumLayoutMap[4picTitle]"
    );
    assert_eq!(
        m.get("fitToSlide"),
        Some(&0),
        "photoAlbumLayoutMap[fitToSlide]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "photoAlbumLayoutMap 不应含未知键"
    );
}

#[test]
fn map_view_type_map() {
    let m = SWReaderMap::get_viewTypeMap();
    assert_eq!(m.len(), 8, "viewTypeMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("sldView"), Some(&1), "viewTypeMap[sldView]");
    assert_eq!(
        m.get("sldMasterView"),
        Some(&2),
        "viewTypeMap[sldMasterView]"
    );
    assert_eq!(m.get("notesView"), Some(&3), "viewTypeMap[notesView]");
    assert_eq!(m.get("handoutView"), Some(&4), "viewTypeMap[handoutView]");
    assert_eq!(
        m.get("notesMasterView"),
        Some(&5),
        "viewTypeMap[notesMasterView]"
    );
    assert_eq!(m.get("outlineView"), Some(&7), "viewTypeMap[outlineView]");
    assert_eq!(
        m.get("sldSorterView"),
        Some(&8),
        "viewTypeMap[sldSorterView]"
    );
    assert_eq!(
        m.get("sldThumbnailView"),
        Some(&15),
        "viewTypeMap[sldThumbnailView]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "viewTypeMap 不应含未知键"
    );
}

#[test]
fn map_splitter_bar_state_map() {
    let m = SWReaderMap::get_splitterBarStateMap();
    assert_eq!(m.len(), 3, "splitterBarStateMap 条目数与 Java 字面量不一致");
    assert_eq!(
        m.get("minimized"),
        Some(&0),
        "splitterBarStateMap[minimized]"
    );
    assert_eq!(m.get("restored"), Some(&1), "splitterBarStateMap[restored]");
    assert_eq!(
        m.get("maximized"),
        Some(&2),
        "splitterBarStateMap[maximized]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "splitterBarStateMap 不应含未知键"
    );
}

#[test]
fn map_behavior_attribute_name_map() {
    let m = SWReaderMap::get_behaviorAttributeNameMap();
    assert_eq!(
        m.len(),
        117,
        "behaviorAttributeNameMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(
        m.get("style.opacity"),
        Some(&0),
        "behaviorAttributeNameMap[style.opacity]"
    );
    assert_eq!(
        m.get("style.rotation"),
        Some(&1),
        "behaviorAttributeNameMap[style.rotation]"
    );
    assert_eq!(
        m.get("style.visibility"),
        Some(&2),
        "behaviorAttributeNameMap[style.visibility]"
    );
    assert_eq!(
        m.get("style.color"),
        Some(&3),
        "behaviorAttributeNameMap[style.color]"
    );
    assert_eq!(
        m.get("style.fontSize"),
        Some(&4),
        "behaviorAttributeNameMap[style.fontSize]"
    );
    assert_eq!(
        m.get("style.fontWeight"),
        Some(&5),
        "behaviorAttributeNameMap[style.fontWeight]"
    );
    assert_eq!(
        m.get("style.fontStyle"),
        Some(&6),
        "behaviorAttributeNameMap[style.fontStyle]"
    );
    assert_eq!(
        m.get("style.fontFamily"),
        Some(&7),
        "behaviorAttributeNameMap[style.fontFamily]"
    );
    assert_eq!(
        m.get("style.textEffectEmboss"),
        Some(&8),
        "behaviorAttributeNameMap[style.textEffectEmboss]"
    );
    assert_eq!(
        m.get("style.textShadow"),
        Some(&9),
        "behaviorAttributeNameMap[style.textShadow]"
    );
    assert_eq!(
        m.get("style.textTransform"),
        Some(&10),
        "behaviorAttributeNameMap[style.textTransform]"
    );
    assert_eq!(
        m.get("style.textDecorationUnderline"),
        Some(&11),
        "behaviorAttributeNameMap[style.textDecorationUnderline]"
    );
    assert_eq!(
        m.get("style.textEffectOutline"),
        Some(&12),
        "behaviorAttributeNameMap[style.textEffectOutline]"
    );
    assert_eq!(
        m.get("style.textDecorationLineThrough"),
        Some(&13),
        "behaviorAttributeNameMap[style.textDecorationLineThrough]"
    );
    assert_eq!(
        m.get("style.sRotation"),
        Some(&14),
        "behaviorAttributeNameMap[style.sRotation]"
    );
    assert_eq!(
        m.get("imageData.cropTop"),
        Some(&15),
        "behaviorAttributeNameMap[imageData.cropTop]"
    );
    assert_eq!(
        m.get("imageData.cropBottom"),
        Some(&16),
        "behaviorAttributeNameMap[imageData.cropBottom]"
    );
    assert_eq!(
        m.get("imageData.cropLeft"),
        Some(&17),
        "behaviorAttributeNameMap[imageData.cropLeft]"
    );
    assert_eq!(
        m.get("imageData.cropRight"),
        Some(&18),
        "behaviorAttributeNameMap[imageData.cropRight]"
    );
    assert_eq!(
        m.get("imageData.gain"),
        Some(&19),
        "behaviorAttributeNameMap[imageData.gain]"
    );
    assert_eq!(
        m.get("imageData.blacklevel"),
        Some(&20),
        "behaviorAttributeNameMap[imageData.blacklevel]"
    );
    assert_eq!(
        m.get("imageData.gamma"),
        Some(&21),
        "behaviorAttributeNameMap[imageData.gamma]"
    );
    assert_eq!(
        m.get("imageData.grayscale"),
        Some(&22),
        "behaviorAttributeNameMap[imageData.grayscale]"
    );
    assert_eq!(
        m.get("imageData.chromakey"),
        Some(&23),
        "behaviorAttributeNameMap[imageData.chromakey]"
    );
    assert_eq!(
        m.get("fill.on"),
        Some(&24),
        "behaviorAttributeNameMap[fill.on]"
    );
    assert_eq!(
        m.get("fill.type"),
        Some(&25),
        "behaviorAttributeNameMap[fill.type]"
    );
    assert_eq!(
        m.get("fillColor"),
        Some(&26),
        "behaviorAttributeNameMap[fillColor]"
    );
    assert_eq!(
        m.get("fillcolor"),
        Some(&26),
        "behaviorAttributeNameMap[fillcolor]"
    );
    assert_eq!(
        m.get("fill.color"),
        Some(&26),
        "behaviorAttributeNameMap[fill.color]"
    );
    assert_eq!(
        m.get("fill.opacity"),
        Some(&27),
        "behaviorAttributeNameMap[fill.opacity]"
    );
    assert_eq!(
        m.get("fill.color2"),
        Some(&28),
        "behaviorAttributeNameMap[fill.color2]"
    );
    assert_eq!(
        m.get("fill.method"),
        Some(&29),
        "behaviorAttributeNameMap[fill.method]"
    );
    assert_eq!(
        m.get("fill.opacity2"),
        Some(&30),
        "behaviorAttributeNameMap[fill.opacity2]"
    );
    assert_eq!(
        m.get("fill.angle"),
        Some(&31),
        "behaviorAttributeNameMap[fill.angle]"
    );
    assert_eq!(
        m.get("fill.focus"),
        Some(&32),
        "behaviorAttributeNameMap[fill.focus]"
    );
    assert_eq!(
        m.get("fill.focusposition.x"),
        Some(&33),
        "behaviorAttributeNameMap[fill.focusposition.x]"
    );
    assert_eq!(
        m.get("fill.focusposition.y"),
        Some(&34),
        "behaviorAttributeNameMap[fill.focusposition.y]"
    );
    assert_eq!(
        m.get("fill.focussize.x"),
        Some(&35),
        "behaviorAttributeNameMap[fill.focussize.x]"
    );
    assert_eq!(
        m.get("fill.focussize.y"),
        Some(&36),
        "behaviorAttributeNameMap[fill.focussize.y]"
    );
    assert_eq!(
        m.get("stroke.on"),
        Some(&37),
        "behaviorAttributeNameMap[stroke.on]"
    );
    assert_eq!(
        m.get("stroke.color"),
        Some(&38),
        "behaviorAttributeNameMap[stroke.color]"
    );
    assert_eq!(
        m.get("stroke.weight"),
        Some(&39),
        "behaviorAttributeNameMap[stroke.weight]"
    );
    assert_eq!(
        m.get("stroke.opacity"),
        Some(&40),
        "behaviorAttributeNameMap[stroke.opacity]"
    );
    assert_eq!(
        m.get("stroke.linestyle"),
        Some(&41),
        "behaviorAttributeNameMap[stroke.linestyle]"
    );
    assert_eq!(
        m.get("stroke.dashstyle"),
        Some(&42),
        "behaviorAttributeNameMap[stroke.dashstyle]"
    );
    assert_eq!(
        m.get("stroke.filltype"),
        Some(&43),
        "behaviorAttributeNameMap[stroke.filltype]"
    );
    assert_eq!(
        m.get("stroke.src"),
        Some(&44),
        "behaviorAttributeNameMap[stroke.src]"
    );
    assert_eq!(
        m.get("stroke.color2"),
        Some(&45),
        "behaviorAttributeNameMap[stroke.color2]"
    );
    assert_eq!(
        m.get("stroke.imagesize.x"),
        Some(&46),
        "behaviorAttributeNameMap[stroke.imagesize.x]"
    );
    assert_eq!(
        m.get("stroke.imagesize.y"),
        Some(&47),
        "behaviorAttributeNameMap[stroke.imagesize.y]"
    );
    assert_eq!(
        m.get("stroke.startArrow"),
        Some(&48),
        "behaviorAttributeNameMap[stroke.startArrow]"
    );
    assert_eq!(
        m.get("stroke.endArrow"),
        Some(&49),
        "behaviorAttributeNameMap[stroke.endArrow]"
    );
    assert_eq!(
        m.get("stroke.startArrowWidth"),
        Some(&50),
        "behaviorAttributeNameMap[stroke.startArrowWidth]"
    );
    assert_eq!(
        m.get("stroke.startArrowLength"),
        Some(&51),
        "behaviorAttributeNameMap[stroke.startArrowLength]"
    );
    assert_eq!(
        m.get("stroke.endArrowWidth"),
        Some(&52),
        "behaviorAttributeNameMap[stroke.endArrowWidth]"
    );
    assert_eq!(
        m.get("stroke.endArrowLength"),
        Some(&53),
        "behaviorAttributeNameMap[stroke.endArrowLength]"
    );
    assert_eq!(
        m.get("shadow.on"),
        Some(&54),
        "behaviorAttributeNameMap[shadow.on]"
    );
    assert_eq!(
        m.get("shadow.type"),
        Some(&55),
        "behaviorAttributeNameMap[shadow.type]"
    );
    assert_eq!(
        m.get("shadow.color"),
        Some(&56),
        "behaviorAttributeNameMap[shadow.color]"
    );
    assert_eq!(
        m.get("shadow.color2"),
        Some(&57),
        "behaviorAttributeNameMap[shadow.color2]"
    );
    assert_eq!(
        m.get("shadow.opacity"),
        Some(&58),
        "behaviorAttributeNameMap[shadow.opacity]"
    );
    assert_eq!(
        m.get("shadow.offset.x"),
        Some(&59),
        "behaviorAttributeNameMap[shadow.offset.x]"
    );
    assert_eq!(
        m.get("shadow.offset.y"),
        Some(&60),
        "behaviorAttributeNameMap[shadow.offset.y]"
    );
    assert_eq!(
        m.get("shadow.offset2.x"),
        Some(&61),
        "behaviorAttributeNameMap[shadow.offset2.x]"
    );
    assert_eq!(
        m.get("shadow.offset2.y"),
        Some(&62),
        "behaviorAttributeNameMap[shadow.offset2.y]"
    );
    assert_eq!(
        m.get("shadow.origin.x"),
        Some(&63),
        "behaviorAttributeNameMap[shadow.origin.x]"
    );
    assert_eq!(
        m.get("shadow.origin.y"),
        Some(&64),
        "behaviorAttributeNameMap[shadow.origin.y]"
    );
    assert_eq!(
        m.get("shadow.matrix.xtox"),
        Some(&65),
        "behaviorAttributeNameMap[shadow.matrix.xtox]"
    );
    assert_eq!(
        m.get("shadow.matrix.ytox"),
        Some(&66),
        "behaviorAttributeNameMap[shadow.matrix.ytox]"
    );
    assert_eq!(
        m.get("shadow.matrix.ytoy"),
        Some(&67),
        "behaviorAttributeNameMap[shadow.matrix.ytoy]"
    );
    assert_eq!(
        m.get("shadow.matrix.perspectiveX"),
        Some(&68),
        "behaviorAttributeNameMap[shadow.matrix.perspectiveX]"
    );
    assert_eq!(
        m.get("shadow.matrix.perspectiveY"),
        Some(&69),
        "behaviorAttributeNameMap[shadow.matrix.perspectiveY]"
    );
    assert_eq!(
        m.get("skew.on"),
        Some(&70),
        "behaviorAttributeNameMap[skew.on]"
    );
    assert_eq!(
        m.get("skew.offset.x"),
        Some(&71),
        "behaviorAttributeNameMap[skew.offset.x]"
    );
    assert_eq!(
        m.get("skew.offset.y"),
        Some(&72),
        "behaviorAttributeNameMap[skew.offset.y]"
    );
    assert_eq!(
        m.get("skew.origin.x"),
        Some(&73),
        "behaviorAttributeNameMap[skew.origin.x]"
    );
    assert_eq!(
        m.get("skew.origin.y"),
        Some(&74),
        "behaviorAttributeNameMap[skew.origin.y]"
    );
    assert_eq!(
        m.get("skew.matrix.xtox"),
        Some(&75),
        "behaviorAttributeNameMap[skew.matrix.xtox]"
    );
    assert_eq!(
        m.get("skew.matrix.ytox"),
        Some(&76),
        "behaviorAttributeNameMap[skew.matrix.ytox]"
    );
    assert_eq!(
        m.get("skew.matrix.ytoy"),
        Some(&77),
        "behaviorAttributeNameMap[skew.matrix.ytoy]"
    );
    assert_eq!(
        m.get("skew.matrix.perspectiveX"),
        Some(&78),
        "behaviorAttributeNameMap[skew.matrix.perspectiveX]"
    );
    assert_eq!(
        m.get("skew.matrix.perspectiveY"),
        Some(&79),
        "behaviorAttributeNameMap[skew.matrix.perspectiveY]"
    );
    assert_eq!(
        m.get("extrusion.on"),
        Some(&80),
        "behaviorAttributeNameMap[extrusion.on]"
    );
    assert_eq!(
        m.get("extrusion.type"),
        Some(&81),
        "behaviorAttributeNameMap[extrusion.type]"
    );
    assert_eq!(
        m.get("extrusion.render"),
        Some(&82),
        "behaviorAttributeNameMap[extrusion.render]"
    );
    assert_eq!(
        m.get("extrusion.viewpointorigin.x"),
        Some(&83),
        "behaviorAttributeNameMap[extrusion.viewpointorigin.x]"
    );
    assert_eq!(
        m.get("extrusion.viewpointorigin.y"),
        Some(&84),
        "behaviorAttributeNameMap[extrusion.viewpointorigin.y]"
    );
    assert_eq!(
        m.get("extrusion.viewpoint.x"),
        Some(&85),
        "behaviorAttributeNameMap[extrusion.viewpoint.x]"
    );
    assert_eq!(
        m.get("extrusion.viewpoint.y"),
        Some(&86),
        "behaviorAttributeNameMap[extrusion.viewpoint.y]"
    );
    assert_eq!(
        m.get("extrusion.viewpoint.z"),
        Some(&87),
        "behaviorAttributeNameMap[extrusion.viewpoint.z]"
    );
    assert_eq!(
        m.get("extrusion.plane"),
        Some(&88),
        "behaviorAttributeNameMap[extrusion.plane]"
    );
    assert_eq!(
        m.get("extrusion.skewangle"),
        Some(&89),
        "behaviorAttributeNameMap[extrusion.skewangle]"
    );
    assert_eq!(
        m.get("extrusion.skewamt"),
        Some(&90),
        "behaviorAttributeNameMap[extrusion.skewamt]"
    );
    assert_eq!(
        m.get("extrusion.backdepth"),
        Some(&91),
        "behaviorAttributeNameMap[extrusion.backdepth]"
    );
    assert_eq!(
        m.get("extrusion.foredepth"),
        Some(&92),
        "behaviorAttributeNameMap[extrusion.foredepth]"
    );
    assert_eq!(
        m.get("extrusion.orientation.x"),
        Some(&93),
        "behaviorAttributeNameMap[extrusion.orientation.x]"
    );
    assert_eq!(
        m.get("extrusion.orientation.y"),
        Some(&94),
        "behaviorAttributeNameMap[extrusion.orientation.y]"
    );
    assert_eq!(
        m.get("extrusion.orientation.z"),
        Some(&95),
        "behaviorAttributeNameMap[extrusion.orientation.z]"
    );
    assert_eq!(
        m.get("extrusion.orientationangle"),
        Some(&96),
        "behaviorAttributeNameMap[extrusion.orientationangle]"
    );
    assert_eq!(
        m.get("extrusion.color"),
        Some(&97),
        "behaviorAttributeNameMap[extrusion.color]"
    );
    assert_eq!(
        m.get("extrusion.rotationangle.x"),
        Some(&98),
        "behaviorAttributeNameMap[extrusion.rotationangle.x]"
    );
    assert_eq!(
        m.get("extrusion.rotationangle.y"),
        Some(&99),
        "behaviorAttributeNameMap[extrusion.rotationangle.y]"
    );
    assert_eq!(
        m.get("extrusion.lockrotationcenter"),
        Some(&100),
        "behaviorAttributeNameMap[extrusion.lockrotationcenter]"
    );
    assert_eq!(
        m.get("extrusion.autorotationcenter"),
        Some(&101),
        "behaviorAttributeNameMap[extrusion.autorotationcenter]"
    );
    assert_eq!(
        m.get("extrusion.rotationcenter.x"),
        Some(&102),
        "behaviorAttributeNameMap[extrusion.rotationcenter.x]"
    );
    assert_eq!(
        m.get("extrusion.rotationcenter.y"),
        Some(&103),
        "behaviorAttributeNameMap[extrusion.rotationcenter.y]"
    );
    assert_eq!(
        m.get("extrusion.rotationcenter.z"),
        Some(&104),
        "behaviorAttributeNameMap[extrusion.rotationcenter.z]"
    );
    assert_eq!(
        m.get("extrusion.colormode"),
        Some(&105),
        "behaviorAttributeNameMap[extrusion.colormode]"
    );
    assert_eq!(
        m.get("ppt_x"),
        Some(&106),
        "behaviorAttributeNameMap[ppt_x]"
    );
    assert_eq!(
        m.get("ppt_y"),
        Some(&107),
        "behaviorAttributeNameMap[ppt_y]"
    );
    assert_eq!(
        m.get("ppt_x,ppt_y"),
        Some(&108),
        "behaviorAttributeNameMap[ppt_x,ppt_y]"
    );
    assert_eq!(
        m.get("ppt_w"),
        Some(&109),
        "behaviorAttributeNameMap[ppt_w]"
    );
    assert_eq!(
        m.get("ppt_h"),
        Some(&110),
        "behaviorAttributeNameMap[ppt_h]"
    );
    assert_eq!(
        m.get("ppt_c"),
        Some(&111),
        "behaviorAttributeNameMap[ppt_c]"
    );
    assert_eq!(
        m.get("xshear"),
        Some(&112),
        "behaviorAttributeNameMap[xshear]"
    );
    assert_eq!(m.get("r"), Some(&113), "behaviorAttributeNameMap[r]");
    assert_eq!(
        m.get("num.show"),
        Some(&115),
        "behaviorAttributeNameMap[num.show]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "behaviorAttributeNameMap 不应含未知键"
    );
}

#[test]
fn map_preset_transition_map() {
    let m = SWReaderMap::get_presetTransitionMap();
    assert_eq!(
        m.len(),
        12,
        "presetTransitionMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(
        m.get("fallOver"),
        Some(&201),
        "presetTransitionMap[fallOver]"
    );
    assert_eq!(m.get("drape"), Some(&202), "presetTransitionMap[drape]");
    assert_eq!(
        m.get("curtains"),
        Some(&203),
        "presetTransitionMap[curtains]"
    );
    assert_eq!(m.get("wind"), Some(&204), "presetTransitionMap[wind]");
    assert_eq!(
        m.get("prestige"),
        Some(&205),
        "presetTransitionMap[prestige]"
    );
    assert_eq!(
        m.get("fracture"),
        Some(&206),
        "presetTransitionMap[fracture]"
    );
    assert_eq!(m.get("crush"), Some(&207), "presetTransitionMap[crush]");
    assert_eq!(m.get("peelOff"), Some(&208), "presetTransitionMap[peelOff]");
    assert_eq!(
        m.get("pageCurlDouble"),
        Some(&209),
        "presetTransitionMap[pageCurlDouble]"
    );
    assert_eq!(
        m.get("pageCurlSingle"),
        Some(&210),
        "presetTransitionMap[pageCurlSingle]"
    );
    assert_eq!(
        m.get("airplane"),
        Some(&211),
        "presetTransitionMap[airplane]"
    );
    assert_eq!(m.get("origami"), Some(&212), "presetTransitionMap[origami]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "presetTransitionMap 不应含未知键"
    );
}

#[test]
fn map_print_what_map() {
    let m = SWReaderMap::get_printWhatMap();
    assert_eq!(m.len(), 9, "printWhatMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("slides"), Some(&8), "printWhatMap[slides]");
    assert_eq!(m.get("handouts1"), Some(&0), "printWhatMap[handouts1]");
    assert_eq!(m.get("handouts2"), Some(&1), "printWhatMap[handouts2]");
    assert_eq!(m.get("handouts3"), Some(&2), "printWhatMap[handouts3]");
    assert_eq!(m.get("handouts4"), Some(&3), "printWhatMap[handouts4]");
    assert_eq!(m.get("handouts6"), Some(&4), "printWhatMap[handouts6]");
    assert_eq!(m.get("handouts9"), Some(&5), "printWhatMap[handouts9]");
    assert_eq!(m.get("notes"), Some(&6), "printWhatMap[notes]");
    assert_eq!(m.get("outline"), Some(&7), "printWhatMap[outline]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "printWhatMap 不应含未知键"
    );
}

#[test]
fn map_print_color_mode_map() {
    let m = SWReaderMap::get_printColorModeMap();
    assert_eq!(m.len(), 3, "printColorModeMap 条目数与 Java 字面量不一致");
    assert_eq!(m.get("bw"), Some(&0), "printColorModeMap[bw]");
    assert_eq!(m.get("gray"), Some(&2), "printColorModeMap[gray]");
    assert_eq!(m.get("clr"), Some(&1), "printColorModeMap[clr]");
    assert!(
        !m.contains_key("__no_such_tag__"),
        "printColorModeMap 不应含未知键"
    );
}

#[test]
fn map_ole_object_follow_color_scheme_map() {
    let m = SWReaderMap::get_oleObjectFollowColorSchemeMap();
    assert_eq!(
        m.len(),
        3,
        "oleObjectFollowColorSchemeMap 条目数与 Java 字面量不一致"
    );
    assert_eq!(
        m.get("full"),
        Some(&0),
        "oleObjectFollowColorSchemeMap[full]"
    );
    assert_eq!(
        m.get("none"),
        Some(&1),
        "oleObjectFollowColorSchemeMap[none]"
    );
    assert_eq!(
        m.get("textAndBackground"),
        Some(&2),
        "oleObjectFollowColorSchemeMap[textAndBackground]"
    );
    assert!(
        !m.contains_key("__no_such_tag__"),
        "oleObjectFollowColorSchemeMap 不应含未知键"
    );
}
