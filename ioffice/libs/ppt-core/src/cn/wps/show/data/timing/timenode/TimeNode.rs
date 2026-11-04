// Rust stub for TimeNode (auto-generated)
// Original: cn/wps/show/data/timing/timenode/TimeNode.java
use crate::*;

pub struct TimeNode {
    // TODO: fields
}

impl TimeNode {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> TimeNode { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> TimeNode { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: TimeNode) { todo!() }
    pub fn saveBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: TimeNode) -> bool { todo!() }
    pub fn equals(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_1(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn hashCode(&self) -> i32 { todo!() }
    pub fn dirty(&self) -> bool { todo!() }
    pub fn save(&self) -> Box<dyn KoBin> { todo!() }
    pub fn lastBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn discard(&self, zone: KoZone) { todo!() }
    pub fn close(&self) { todo!() }
    pub fn cleanFollow(&self) { todo!() }
    pub fn cleanCache(&self) { todo!() }
    pub fn follow(&self, parent: TimeNode) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> TimeNode { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn animate(&self) -> Animate { todo!() }
    pub fn hasAnimate(&self) -> bool { todo!() }
    pub fn deleteAnimate(&self) { todo!() }
    pub fn writeAnimate(&self, value: Animate) { todo!() }
    pub fn forceWriteAnimate(&self, value: Animate) { todo!() }
    pub fn animateColorBehavior(&self) -> AnimateColorBehavior { todo!() }
    pub fn hasAnimateColorBehavior(&self) -> bool { todo!() }
    pub fn deleteAnimateColorBehavior(&self) { todo!() }
    pub fn writeAnimateColorBehavior(&self, value: AnimateColorBehavior) { todo!() }
    pub fn forceWriteAnimateColorBehavior(&self, value: AnimateColorBehavior) { todo!() }
    pub fn animateEffect(&self) -> AnimateEffect { todo!() }
    pub fn hasAnimateEffect(&self) -> bool { todo!() }
    pub fn deleteAnimateEffect(&self) { todo!() }
    pub fn writeAnimateEffect(&self, value: AnimateEffect) { todo!() }
    pub fn forceWriteAnimateEffect(&self, value: AnimateEffect) { todo!() }
    pub fn animateMotion(&self) -> AnimateMotion { todo!() }
    pub fn hasAnimateMotion(&self) -> bool { todo!() }
    pub fn deleteAnimateMotion(&self) { todo!() }
    pub fn writeAnimateMotion(&self, value: AnimateMotion) { todo!() }
    pub fn forceWriteAnimateMotion(&self, value: AnimateMotion) { todo!() }
    pub fn animateRotation(&self) -> AnimateRotation { todo!() }
    pub fn hasAnimateRotation(&self) -> bool { todo!() }
    pub fn deleteAnimateRotation(&self) { todo!() }
    pub fn writeAnimateRotation(&self, value: AnimateRotation) { todo!() }
    pub fn forceWriteAnimateRotation(&self, value: AnimateRotation) { todo!() }
    pub fn animateScale(&self) -> AnimateScale { todo!() }
    pub fn hasAnimateScale(&self) -> bool { todo!() }
    pub fn deleteAnimateScale(&self) { todo!() }
    pub fn writeAnimateScale(&self, value: AnimateScale) { todo!() }
    pub fn forceWriteAnimateScale(&self, value: AnimateScale) { todo!() }
    pub fn audio(&self) -> Audio { todo!() }
    pub fn hasAudio(&self) -> bool { todo!() }
    pub fn deleteAudio(&self) { todo!() }
    pub fn writeAudio(&self, value: Audio) { todo!() }
    pub fn forceWriteAudio(&self, value: Audio) { todo!() }
    pub fn command(&self) -> Command { todo!() }
    pub fn hasCommand(&self) -> bool { todo!() }
    pub fn deleteCommand(&self) { todo!() }
    pub fn writeCommand(&self, value: Command) { todo!() }
    pub fn forceWriteCommand(&self, value: Command) { todo!() }
    pub fn exclusive(&self) -> CommonTimeNode { todo!() }
    pub fn hasExclusive(&self) -> bool { todo!() }
    pub fn deleteExclusive(&self) { todo!() }
    pub fn writeExclusive(&self, value: CommonTimeNode) { todo!() }
    pub fn forceWriteExclusive(&self, value: CommonTimeNode) { todo!() }
    pub fn parallelTimeNode(&self) -> CommonTimeNode { todo!() }
    pub fn hasParallelTimeNode(&self) -> bool { todo!() }
    pub fn deleteParallelTimeNode(&self) { todo!() }
    pub fn writeParallelTimeNode(&self, value: CommonTimeNode) { todo!() }
    pub fn forceWriteParallelTimeNode(&self, value: CommonTimeNode) { todo!() }
    pub fn sequenceTimeNode(&self) -> SequenceTimeNode { todo!() }
    pub fn hasSequenceTimeNode(&self) -> bool { todo!() }
    pub fn deleteSequenceTimeNode(&self) { todo!() }
    pub fn writeSequenceTimeNode(&self, value: SequenceTimeNode) { todo!() }
    pub fn forceWriteSequenceTimeNode(&self, value: SequenceTimeNode) { todo!() }
    pub fn setTimeNodeBehavior(&self) -> SetTimeNodeBehavior { todo!() }
    pub fn hasSetTimeNodeBehavior(&self) -> bool { todo!() }
    pub fn deleteSetTimeNodeBehavior(&self) { todo!() }
    pub fn writeSetTimeNodeBehavior(&self, value: SetTimeNodeBehavior) { todo!() }
    pub fn forceWriteSetTimeNodeBehavior(&self, value: SetTimeNodeBehavior) { todo!() }
    pub fn video(&self) -> Video { todo!() }
    pub fn hasVideo(&self) -> bool { todo!() }
    pub fn deleteVideo(&self) { todo!() }
    pub fn writeVideo(&self, value: Video) { todo!() }
    pub fn forceWriteVideo(&self, value: Video) { todo!() }
    pub fn r#type(&self) -> i32 { todo!() }
    pub fn hasType(&self) -> bool { todo!() }
    pub fn deleteType(&self) { todo!() }
    pub fn putType(&self, value: i32) { todo!() }
}
