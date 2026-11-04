// Rust stub for Transition (auto-generated)
// Original: cn/wps/show/data/transition/Transition.java
use crate::*;

pub struct Transition {
    // TODO: fields
}

impl Transition {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> Transition { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> Transition { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: Transition) { todo!() }
    pub fn saveBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: Transition) -> bool { todo!() }
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
    pub fn follow(&self, parent: Transition) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> Transition { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn isAdvClick(&self) -> bool { todo!() }
    pub fn hasAdvClick(&self) -> bool { todo!() }
    pub fn deleteAdvClick(&self) { todo!() }
    pub fn putAdvClick(&self, value: bool) { todo!() }
    pub fn advTm(&self) -> i32 { todo!() }
    pub fn hasAdvTm(&self) -> bool { todo!() }
    pub fn deleteAdvTm(&self) { todo!() }
    pub fn putAdvTm(&self, value: i32) { todo!() }
    pub fn spd(&self) -> i32 { todo!() }
    pub fn hasSpd(&self) -> bool { todo!() }
    pub fn deleteSpd(&self) { todo!() }
    pub fn putSpd(&self, value: i32) { todo!() }
    pub fn isHidden(&self) -> bool { todo!() }
    pub fn hasHidden(&self) -> bool { todo!() }
    pub fn deleteHidden(&self) { todo!() }
    pub fn putHidden(&self, value: bool) { todo!() }
    pub fn mode(&self) -> i32 { todo!() }
    pub fn hasMode(&self) -> bool { todo!() }
    pub fn deleteMode(&self) { todo!() }
    pub fn putMode(&self, value: i32) { todo!() }
    pub fn blindsDirection(&self) -> i32 { todo!() }
    pub fn hasBlindsDirection(&self) -> bool { todo!() }
    pub fn deleteBlindsDirection(&self) { todo!() }
    pub fn putBlindsDirection(&self, value: i32) { todo!() }
    pub fn checkerDirection(&self) -> i32 { todo!() }
    pub fn hasCheckerDirection(&self) -> bool { todo!() }
    pub fn deleteCheckerDirection(&self) { todo!() }
    pub fn putCheckerDirection(&self, value: i32) { todo!() }
    pub fn combDirection(&self) -> i32 { todo!() }
    pub fn hasCombDirection(&self) -> bool { todo!() }
    pub fn deleteCombDirection(&self) { todo!() }
    pub fn putCombDirection(&self, value: i32) { todo!() }
    pub fn coverDirection(&self) -> i32 { todo!() }
    pub fn hasCoverDirection(&self) -> bool { todo!() }
    pub fn deleteCoverDirection(&self) { todo!() }
    pub fn putCoverDirection(&self, value: i32) { todo!() }
    pub fn cutOptionalBlackTransition(&self) -> i32 { todo!() }
    pub fn hasCutOptionalBlackTransition(&self) -> bool { todo!() }
    pub fn deleteCutOptionalBlackTransition(&self) { todo!() }
    pub fn putCutOptionalBlackTransition(&self, value: i32) { todo!() }
    pub fn isFadeOptionalBlackTransition(&self) -> bool { todo!() }
    pub fn hasFadeOptionalBlackTransition(&self) -> bool { todo!() }
    pub fn deleteFadeOptionalBlackTransition(&self) { todo!() }
    pub fn putFadeOptionalBlackTransition(&self, value: bool) { todo!() }
    pub fn pullDirection(&self) -> i32 { todo!() }
    pub fn hasPullDirection(&self) -> bool { todo!() }
    pub fn deletePullDirection(&self) { todo!() }
    pub fn putPullDirection(&self, value: i32) { todo!() }
    pub fn pushDirection(&self) -> i32 { todo!() }
    pub fn hasPushDirection(&self) -> bool { todo!() }
    pub fn deletePushDirection(&self) { todo!() }
    pub fn putPushDirection(&self, value: i32) { todo!() }
    pub fn randomBarDirection(&self) -> i32 { todo!() }
    pub fn hasRandomBarDirection(&self) -> bool { todo!() }
    pub fn deleteRandomBarDirection(&self) { todo!() }
    pub fn putRandomBarDirection(&self, value: i32) { todo!() }
    pub fn splitInOutDirection(&self) -> i32 { todo!() }
    pub fn hasSplitInOutDirection(&self) -> bool { todo!() }
    pub fn deleteSplitInOutDirection(&self) { todo!() }
    pub fn putSplitInOutDirection(&self, value: i32) { todo!() }
    pub fn splitOrientationDirection(&self) -> i32 { todo!() }
    pub fn hasSplitOrientationDirection(&self) -> bool { todo!() }
    pub fn deleteSplitOrientationDirection(&self) { todo!() }
    pub fn putSplitOrientationDirection(&self, value: i32) { todo!() }
    pub fn stripsDirection(&self) -> i32 { todo!() }
    pub fn hasStripsDirection(&self) -> bool { todo!() }
    pub fn deleteStripsDirection(&self) { todo!() }
    pub fn putStripsDirection(&self, value: i32) { todo!() }
    pub fn wheelSpokes(&self) -> i32 { todo!() }
    pub fn hasWheelSpokes(&self) -> bool { todo!() }
    pub fn deleteWheelSpokes(&self) { todo!() }
    pub fn putWheelSpokes(&self, value: i32) { todo!() }
    pub fn wipeDirection(&self) -> i32 { todo!() }
    pub fn hasWipeDirection(&self) -> bool { todo!() }
    pub fn deleteWipeDirection(&self) { todo!() }
    pub fn putWipeDirection(&self, value: i32) { todo!() }
    pub fn zoomDirection(&self) -> i32 { todo!() }
    pub fn hasZoomDirection(&self) -> bool { todo!() }
    pub fn deleteZoomDirection(&self) { todo!() }
    pub fn putZoomDirection(&self, value: i32) { todo!() }
    pub fn soundAction(&self) -> SoundAction { todo!() }
    pub fn hasSoundAction(&self) -> bool { todo!() }
    pub fn deleteSoundAction(&self) { todo!() }
    pub fn writeSoundAction(&self, value: SoundAction) { todo!() }
    pub fn forceWriteSoundAction(&self, value: SoundAction) { todo!() }
    pub fn isInvX(&self) -> bool { todo!() }
    pub fn hasInvX(&self) -> bool { todo!() }
    pub fn deleteInvX(&self) { todo!() }
    pub fn putInvX(&self, value: bool) { todo!() }
    pub fn isInvY(&self) -> bool { todo!() }
    pub fn hasInvY(&self) -> bool { todo!() }
    pub fn deleteInvY(&self) { todo!() }
    pub fn putInvY(&self, value: bool) { todo!() }
    pub fn dur(&self) -> i32 { todo!() }
    pub fn hasDur(&self) -> bool { todo!() }
    pub fn deleteDur(&self) { todo!() }
    pub fn putDur(&self, value: i32) { todo!() }
    pub fn vortexDirection(&self) -> i32 { todo!() }
    pub fn hasVortexDirection(&self) -> bool { todo!() }
    pub fn deleteVortexDirection(&self) { todo!() }
    pub fn putVortexDirection(&self, value: i32) { todo!() }
    pub fn switchDirection(&self) -> i32 { todo!() }
    pub fn hasSwitchDirection(&self) -> bool { todo!() }
    pub fn deleteSwitchDirection(&self) { todo!() }
    pub fn putSwitchDirection(&self, value: i32) { todo!() }
    pub fn flipDirection(&self) -> i32 { todo!() }
    pub fn hasFlipDirection(&self) -> bool { todo!() }
    pub fn deleteFlipDirection(&self) { todo!() }
    pub fn putFlipDirection(&self, value: i32) { todo!() }
    pub fn rippleDirection(&self) -> i32 { todo!() }
    pub fn hasRippleDirection(&self) -> bool { todo!() }
    pub fn deleteRippleDirection(&self) { todo!() }
    pub fn putRippleDirection(&self, value: i32) { todo!() }
    pub fn prismDirection(&self) -> i32 { todo!() }
    pub fn hasPrismDirection(&self) -> bool { todo!() }
    pub fn deletePrismDirection(&self) { todo!() }
    pub fn putPrismDirection(&self, value: i32) { todo!() }
    pub fn isPrismIsContent(&self) -> bool { todo!() }
    pub fn hasPrismIsContent(&self) -> bool { todo!() }
    pub fn deletePrismIsContent(&self) { todo!() }
    pub fn putPrismIsContent(&self, value: bool) { todo!() }
    pub fn isPrismIsInverted(&self) -> bool { todo!() }
    pub fn hasPrismIsInverted(&self) -> bool { todo!() }
    pub fn deletePrismIsInverted(&self) { todo!() }
    pub fn putPrismIsInverted(&self, value: bool) { todo!() }
    pub fn doorsDirection(&self) -> i32 { todo!() }
    pub fn hasDoorsDirection(&self) -> bool { todo!() }
    pub fn deleteDoorsDirection(&self) { todo!() }
    pub fn putDoorsDirection(&self, value: i32) { todo!() }
    pub fn windowDirection(&self) -> i32 { todo!() }
    pub fn hasWindowDirection(&self) -> bool { todo!() }
    pub fn deleteWindowDirection(&self) { todo!() }
    pub fn putWindowDirection(&self, value: i32) { todo!() }
    pub fn ferrisDirection(&self) -> i32 { todo!() }
    pub fn hasFerrisDirection(&self) -> bool { todo!() }
    pub fn deleteFerrisDirection(&self) { todo!() }
    pub fn putFerrisDirection(&self, value: i32) { todo!() }
    pub fn galleryDirection(&self) -> i32 { todo!() }
    pub fn hasGalleryDirection(&self) -> bool { todo!() }
    pub fn deleteGalleryDirection(&self) { todo!() }
    pub fn putGalleryDirection(&self, value: i32) { todo!() }
    pub fn conveyorDirection(&self) -> i32 { todo!() }
    pub fn hasConveyorDirection(&self) -> bool { todo!() }
    pub fn deleteConveyorDirection(&self) { todo!() }
    pub fn putConveyorDirection(&self, value: i32) { todo!() }
    pub fn panDirection(&self) -> i32 { todo!() }
    pub fn hasPanDirection(&self) -> bool { todo!() }
    pub fn deletePanDirection(&self) { todo!() }
    pub fn putPanDirection(&self, value: i32) { todo!() }
    pub fn glitterDirection(&self) -> i32 { todo!() }
    pub fn hasGlitterDirection(&self) -> bool { todo!() }
    pub fn deleteGlitterDirection(&self) { todo!() }
    pub fn putGlitterDirection(&self, value: i32) { todo!() }
    pub fn glitterPattern(&self) -> i32 { todo!() }
    pub fn hasGlitterPattern(&self) -> bool { todo!() }
    pub fn deleteGlitterPattern(&self) { todo!() }
    pub fn putGlitterPattern(&self, value: i32) { todo!() }
    pub fn warpDirection(&self) -> i32 { todo!() }
    pub fn hasWarpDirection(&self) -> bool { todo!() }
    pub fn deleteWarpDirection(&self) { todo!() }
    pub fn putWarpDirection(&self, value: i32) { todo!() }
    pub fn flythroughDirection(&self) -> i32 { todo!() }
    pub fn hasFlythroughDirection(&self) -> bool { todo!() }
    pub fn deleteFlythroughDirection(&self) { todo!() }
    pub fn putFlythroughDirection(&self, value: i32) { todo!() }
    pub fn isFlythroughHasBounce(&self) -> bool { todo!() }
    pub fn hasFlythroughHasBounce(&self) -> bool { todo!() }
    pub fn deleteFlythroughHasBounce(&self) { todo!() }
    pub fn putFlythroughHasBounce(&self, value: bool) { todo!() }
    pub fn shredDirection(&self) -> i32 { todo!() }
    pub fn hasShredDirection(&self) -> bool { todo!() }
    pub fn deleteShredDirection(&self) { todo!() }
    pub fn putShredDirection(&self, value: i32) { todo!() }
    pub fn shredPattern(&self) -> i32 { todo!() }
    pub fn hasShredPattern(&self) -> bool { todo!() }
    pub fn deleteShredPattern(&self) { todo!() }
    pub fn putShredPattern(&self, value: i32) { todo!() }
    pub fn revealDirection(&self) -> i32 { todo!() }
    pub fn hasRevealDirection(&self) -> bool { todo!() }
    pub fn deleteRevealDirection(&self) { todo!() }
    pub fn putRevealDirection(&self, value: i32) { todo!() }
    pub fn isRevealThruBlk(&self) -> bool { todo!() }
    pub fn hasRevealThruBlk(&self) -> bool { todo!() }
    pub fn deleteRevealThruBlk(&self) { todo!() }
    pub fn putRevealThruBlk(&self, value: bool) { todo!() }
    pub fn wheelRevereSpokes(&self) -> i32 { todo!() }
    pub fn hasWheelRevereSpokes(&self) -> bool { todo!() }
    pub fn deleteWheelRevereSpokes(&self) { todo!() }
    pub fn putWheelRevereSpokes(&self, value: i32) { todo!() }
    pub fn appearDirection(&self) -> i32 { todo!() }
    pub fn hasAppearDirection(&self) -> bool { todo!() }
    pub fn deleteAppearDirection(&self) { todo!() }
    pub fn putAppearDirection(&self, value: i32) { todo!() }
    pub fn twistDirection(&self) -> i32 { todo!() }
    pub fn hasTwistDirection(&self) -> bool { todo!() }
    pub fn deleteTwistDirection(&self) { todo!() }
    pub fn putTwistDirection(&self, value: i32) { todo!() }
    pub fn invertDirection(&self) -> i32 { todo!() }
    pub fn hasInvertDirection(&self) -> bool { todo!() }
    pub fn deleteInvertDirection(&self) { todo!() }
    pub fn putInvertDirection(&self, value: i32) { todo!() }
    pub fn explodeDirection(&self) -> i32 { todo!() }
    pub fn hasExplodeDirection(&self) -> bool { todo!() }
    pub fn deleteExplodeDirection(&self) { todo!() }
    pub fn putExplodeDirection(&self, value: i32) { todo!() }
    pub fn teeterDirection(&self) -> i32 { todo!() }
    pub fn hasTeeterDirection(&self) -> bool { todo!() }
    pub fn deleteTeeterDirection(&self) { todo!() }
    pub fn putTeeterDirection(&self, value: i32) { todo!() }
    pub fn initialize_1(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create_1() -> SoundAction { todo!() }
    pub fn open_1(bin: Box<dyn KoBin>) -> SoundAction { todo!() }
    pub fn loadBin_1(&self, bin: Box<dyn KoBin>, zone: KoZone) { todo!() }
    pub fn load_2(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_3(&self, value: SoundAction) { todo!() }
    pub fn saveBin_1(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax_1(&self, o: SoundAction) -> bool { todo!() }
    pub fn equals_2(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_3(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn hashCode_1(&self) -> i32 { todo!() }
    pub fn dirty_1(&self) -> bool { todo!() }
    pub fn save_1(&self) -> Box<dyn KoBin> { todo!() }
    pub fn lastBin_1(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn discard_2(&self, zone: KoZone) { todo!() }
    pub fn close_1(&self) { todo!() }
    pub fn cleanFollow_1(&self) { todo!() }
    pub fn cleanCache_1(&self) { todo!() }
    pub fn follow_1(&self, parent: SoundAction) { todo!() }
    pub fn unFollow_1(&self) { todo!() }
    pub fn hasParent_1(&self) -> bool { todo!() }
    pub fn parent_1(&self) -> SoundAction { todo!() }
    pub fn discard_3(&self) { todo!() }
    pub fn cleanParent_1(&self) { todo!() }
    pub fn mode_1(&self) -> i32 { todo!() }
    pub fn hasMode_1(&self) -> bool { todo!() }
    pub fn deleteMode_1(&self) { todo!() }
    pub fn putMode_1(&self, value: i32) { todo!() }
    pub fn hasStartSound(&self) -> bool { todo!() }
    pub fn deleteStartSound(&self) { todo!() }
    pub fn writeStartSound(&self, value: Media::EmbeddedWAVAudioFile) { todo!() }
    pub fn forceWriteStartSound(&self, value: Media::EmbeddedWAVAudioFile) { todo!() }
    pub fn isLoop(&self) -> bool { todo!() }
    pub fn hasLoop(&self) -> bool { todo!() }
    pub fn deleteLoop(&self) { todo!() }
    pub fn putLoop(&self, value: bool) { todo!() }
}
