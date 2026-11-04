// Rust stub for SectionLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/SectionLayouter.java
use crate::*;

pub struct SectionLayouter {
    // TODO: fields
}

impl SectionLayouter {
    pub fn new() -> Self { todo!() }
    pub fn setLayouterWhileLoading(&self, layouter: Box<dyn ILayouterWhileLoading>) { todo!() }
    pub fn hasFinishIoLoading(&self) -> bool { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn onStart(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getHeaderFooterRange(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn calcPageSetup(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getSecPropSet(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn calcGrid(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn calcPageBrc(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn onEnd(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn calcColumns(&self) { todo!() }
}

pub trait IStateEnvoy {}
