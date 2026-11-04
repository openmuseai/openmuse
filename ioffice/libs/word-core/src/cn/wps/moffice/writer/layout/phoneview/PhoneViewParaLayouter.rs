// Rust stub for PhoneViewParaLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/phoneview/PhoneViewParaLayouter.java
use crate::*;

pub struct PhoneViewParaLayouter {
    // TODO: fields
}

impl PhoneViewParaLayouter {
    pub fn new() -> Self { todo!() }
    pub fn setIsSameInLayoutPage(&self) { todo!() }
    pub fn onStart(&self, stateEnvoy: ParaLayouter::IStateEnvoy) { todo!() }
    pub fn onEnd(&self, stateEnvoy: ParaLayouter::IStateEnvoy) { todo!() }
    pub fn calcLineSpace(&self, stateEnvoy: ParaLayouter::IStateEnvoy) { todo!() }
    pub fn calcParaAfter(&self, stateEnvoy: ParaLayouter::IStateEnvoy) { todo!() }
    pub fn calcParaBefore(&self, stateEnvoy: ParaLayouter::IStateEnvoy) { todo!() }
    pub fn calAlign(&self, stateEnvoy: ParaLayouter::IStateEnvoy) { todo!() }
    pub fn calcIndents(&self, propEnv: ParaPropEnv, stateEnvoy: ParaLayouter::IStateEnvoy) { todo!() }
    pub fn calcSHD(&self, propEnv: ParaPropEnv) { todo!() }
    pub fn calcBRC(&self, propEnv: ParaPropEnv) { todo!() }
    pub fn calcAutoNum(&self, propEnv: ParaPropEnv, stateEnvoy: ParaLayouter::IStateEnvoy) { todo!() }
    pub fn getLeadingAnchorCount(&self, startCP: i32, endCP: i32, state: ParaLayouter::IStateEnvoy) -> i32 { todo!() }
    pub fn calFirstLineIndent(&self, propEnv: ParaPropEnv, firstLineIndentChars: i32, stateEnvoy: ParaLayouter::IStateEnvoy) { todo!() }
    pub fn getFirstCharSize(&self, runProperty: PropertySet, stateEnvoy: ParaLayouter::IStateEnvoy) -> i32 { todo!() }
    pub fn skipSpaceBeforePara(&self, propEnv: ParaPropEnv, state: ParaLayouter::IStateEnvoy) { todo!() }
    pub fn getFirstNormalCharCP(&self, propEnv: ParaPropEnv, state: ParaLayouter::IStateEnvoy) -> i32 { todo!() }
    pub fn canUseSmartIndent(&self, propEnv: ParaPropEnv, state: ParaLayouter::IStateEnvoy) -> bool { todo!() }
    pub fn isCanSkipCharBeforePara(&self, c: u16) -> bool { todo!() }
    pub fn isTOCFieldPara(&self, document: Box<dyn Document>, paraStart: i32) -> bool { todo!() }
}
