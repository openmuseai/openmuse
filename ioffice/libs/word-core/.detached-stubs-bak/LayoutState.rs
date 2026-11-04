// Rust stub for LayoutState (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/env/LayoutState.java
use crate::*;

pub struct LayoutState {
    // TODO: fields
}

impl LayoutState {
    pub fn new() -> Self { todo!() }
    pub fn clearPredictSpaces(&self) { todo!() }
    pub fn getEmbedSize(&self) -> i32 { todo!() }
    pub fn getEnvoy(&self) -> SuperEnvoy { todo!() }
    pub fn isFinishPage(&self) -> bool { todo!() }
    pub fn setFinishPage(&self, finishPage: bool) { todo!() }
    pub fn setRange(&self, start: i32, end: i32) { todo!() }
    pub fn setRange_1(&self, range: i64) { todo!() }
    pub fn getRange(&self) -> i64 { todo!() }
    pub fn getRangeEnd(&self) -> i32 { todo!() }
    pub fn getRangeStart(&self) -> i32 { todo!() }
    pub fn getPosString(&self) -> String { todo!() }
    pub fn getGraphList(&self) -> KPList { todo!() }
    pub fn nextGraphs(&self) -> Graphs { todo!() }
    pub fn getDrawing(&self, run: Graphs) -> i32 { todo!() }
    pub fn getIndexOfDrawing(&self, graphs: Graphs) -> i32 { todo!() }
    pub fn getDrawings(&self) -> KPList { todo!() }
    pub fn push(&self, firstRun: Graphs) { todo!() }
    pub fn push_1(&self, firstRun: Graphs, restoreFirst: Graphs) { todo!() }
    pub fn breakGraphs(&self, run: Graphs, isRTL: bool, graphIndex: i32, pushBreak: bool) -> Graphs { todo!() }
    pub fn getAbsSum(&self, run: Graphs, graphIndex: i32, length: i32, isRTL: bool) -> i32 { todo!() }
    pub fn get(&self) -> Graphs { todo!() }
    pub fn getNextSectionBreak(&self) -> Graphs { todo!() }
    pub fn getCP(&self, graphIndex: i32) -> i32 { todo!() }
    pub fn extendWidths(&self, run: Graphs, spacePerChar: i32, graphIndex: i32, extendLen: i32) -> i32 { todo!() }
    pub fn extendFloatWidths(&self, run: Graphs, spacePerChar: f32, graphIndex: i32, extendLen: i32) -> i32 { todo!() }
    pub fn scaleWidths(&self, run: Graphs, scale: f32, graphIndex: i32, extendLen: i32, space: i32) -> i32 { todo!() }
    pub fn hasContents(&self) -> bool { todo!() }
    pub fn hasBidiGraph(&self) -> bool { todo!() }
    pub fn hasCoreCharsContent(&self) -> bool { todo!() }
    pub fn addMathParaRange(&self, mathParaStart: i32, mathParaEnd: i32, startNode: PLCMathStart::OMathStartNode) { todo!() }
    pub fn saveOrgWidth(&self, graph: Graphs) { todo!() }
    pub fn checkSaveOrgWidth(&self, graph: Graphs) { todo!() }
    pub fn restoreToOrgWidth(&self, graph: Graphs) { todo!() }
    pub fn addDrawPosOffsetX(&self, graphIndex: i32, offset: i32) { todo!() }
    pub fn addDrawPosOffsetX_1(&self, graphIndex: i32, offset: i32, addOffset: bool) { todo!() }
    pub fn getDrawPosOffsetIndexes(&self) -> WIntArrayList { todo!() }
    pub fn getDrawPosOffsetValues(&self) -> WIntArrayList { todo!() }
    pub fn getDrawPosOffset(&self, graphIndex: i32) -> i32 { todo!() }
    pub fn hasMaths(&self) -> bool { todo!() }
    pub fn hasEmbed(&self) -> bool { todo!() }
    pub fn hasEquation(&self) -> bool { todo!() }
    pub fn resetDrawPosOffset(&self) { todo!() }
    pub fn resetGraphsIt(&self) { todo!() }
    pub fn resetMathPara(&self) { todo!() }
    pub fn processRTLGraphWidth(&self, run: Graphs) { todo!() }
    pub fn clearWitchCache(&self) { todo!() }
    pub fn consumeGraphsBefore(&self, lastGraphs: Graphs, isEmbedOnlyInPhoneVeiw: bool) { todo!() }
    pub fn hasDrawing(&self) -> bool { todo!() }
    pub fn addBidi(&self, graphIndex: i32, ch: u16, documentEnv: DocumentEnv) { todo!() }
    pub fn cleanDrawPosOffsetBefore(&self, graphIndex: i32) { todo!() }
    pub fn isInMathParaLine(&self) -> bool { todo!() }
    pub fn isInMathPara(&self) -> bool { todo!() }
    pub fn needFlowOMathInDrawing(&self) -> bool { todo!() }
    pub fn isMathContentState(&self) -> bool { todo!() }
    pub fn addDrawing(&self, mDocument: Box<dyn Document>, shape: Shape, cp: i32) { todo!() }
    pub fn dump(&self, sb: StringBuilder) { todo!() }
    pub fn initCoreContent(&self, maxCP: i32, revisionFinal: bool) { todo!() }
    pub fn addBalloonInfoString(&self, str: String, finalRunProp: PropertySet) { todo!() }
    pub fn addMathString(&self, str: String, doc: Box<dyn Document>, cp: i32) { todo!() }
    pub fn extendsCoreContent(&self, maxCP: i32, revisionFinal: bool) { todo!() }
    pub fn setDrawing(&self, graphs: Graphs, pDrawing: i32, wordAlingFont: i32, useWord97LineBreakRules: bool) { todo!() }
    pub fn setRangeStart(&self, start: i32) { todo!() }
    pub fn setRangeEnd(&self, end: i32) { todo!() }
    pub fn getFirstCoreSrc(&self) -> SrcNode { todo!() }
    pub fn skipSpaceBeforePara(&self, firstNormalCharCP: i32) -> bool { todo!() }
    pub fn checkPropertyHasUnderLine(&self, cp: i32) -> bool { todo!() }
    pub fn isIsolatedEmbed(&self, run: Graphs) -> bool { todo!() }
    pub fn checkIsRtlToKeepSpace(&self, param: PhoneViewParam) -> bool { todo!() }
    pub fn isAllowTransForSpace(&self) -> bool { todo!() }
    pub fn setAllowTransForSpace(&self) { todo!() }
    pub fn checkIsColon(&self, run: Graphs) -> bool { todo!() }
    pub fn needLineBreak(&self, graphIndex: i32) -> bool { todo!() }
    pub fn isUiCardStart(&self, graphIndex: i32) -> bool { todo!() }
    pub fn isTitleEnd(&self, graphIndex: i32) -> bool { todo!() }
    pub fn isUiCardEnd(&self, graphIndex: i32, length: i32) -> bool { todo!() }
    pub fn isInUiCard(&self) -> bool { todo!() }
    pub fn isUiCardStartCP(&self, startCP: i32) -> bool { todo!() }
    pub fn isUiCardEndCP(&self, endCP: i32) -> bool { todo!() }
    pub fn invalidCardState(&self) { todo!() }
    pub fn init(&self, env: LayoutState) { todo!() }
}

pub trait HostEnvoy {}
