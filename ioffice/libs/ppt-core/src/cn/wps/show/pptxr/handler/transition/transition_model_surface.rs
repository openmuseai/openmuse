use kfc_kfc_alpha::cn::wps::kfc::xpxml::reader::XmlRoAttrList::XmlRoAttrList;

use crate::cn::wps::drawing::dmlr::handler::ElementHandler;
pub use crate::cn::wps::drawing::dmlr::handler::shape::textshape::text::text_model_surface::HandlerCallback;

#[derive(Clone, Debug, Default)]
pub struct Transition {
    pub adv_click: Option<bool>,
    pub adv_tm: Option<i32>,
    pub spd: Option<i32>,
    pub dur: Option<i32>,
    pub mode: Option<i32>,
    pub inv_x: Option<bool>,
    pub inv_y: Option<bool>,
    pub shred_pattern: Option<i32>,
    pub shred_direction: Option<i32>,
    pub ripple_direction: Option<i32>,
    pub reveal_thru_blk: Option<bool>,
    pub reveal_direction: Option<i32>,
    pub prism_is_content: Option<bool>,
    pub prism_is_inverted: Option<bool>,
    pub prism_direction: Option<i32>,
    pub glitter_pattern: Option<i32>,
    pub glitter_direction: Option<i32>,
    pub flythrough_direction: Option<i32>,
    pub flythrough_has_bounce: Option<bool>,
    pub switch_direction: Option<i32>,
    pub flip_direction: Option<i32>,
    pub ferris_direction: Option<i32>,
    pub gallery_direction: Option<i32>,
    pub conveyor_direction: Option<i32>,
    pub blinds_direction: Option<i32>,
    pub checker_direction: Option<i32>,
    pub comb_direction: Option<i32>,
    pub random_bar_direction: Option<i32>,
    pub doors_direction: Option<i32>,
    pub window_direction: Option<i32>,
    pub cover_direction: Option<i32>,
    pub pull_direction: Option<i32>,
    pub strips_direction: Option<i32>,
    pub push_direction: Option<i32>,
    pub wipe_direction: Option<i32>,
    pub vortex_direction: Option<i32>,
    pub pan_direction: Option<i32>,
    pub zoom_direction: Option<i32>,
    pub warp_direction: Option<i32>,
    pub split_in_out_direction: Option<i32>,
    pub split_orientation_direction: Option<i32>,
    pub wheel_spokes: Option<i32>,
    pub wheel_revere_spokes: Option<i32>,
    pub cut_optional_black: Option<i32>,
    pub fade_optional_black: Option<bool>,
}

impl Transition {
    pub fn create() -> Self { Self::default() }
    pub fn putAdvClick(&mut self, v: bool) { self.adv_click = Some(v); }
    pub fn putAdvTm(&mut self, v: i32) { self.adv_tm = Some(v); }
    pub fn putSpd(&mut self, v: i32) { self.spd = Some(v); }
    pub fn putDur(&mut self, v: i32) { self.dur = Some(v); }
    pub fn putMode(&mut self, v: i32) { self.mode = Some(v); }
    pub fn putInvX(&mut self, v: bool) { self.inv_x = Some(v); }
    pub fn putInvY(&mut self, v: bool) { self.inv_y = Some(v); }
    pub fn putShredPattern(&mut self, v: i32) { self.shred_pattern = Some(v); }
    pub fn putShredDirection(&mut self, v: i32) { self.shred_direction = Some(v); }
    pub fn putRippleDirection(&mut self, v: i32) { self.ripple_direction = Some(v); }
    pub fn putRevealThruBlk(&mut self, v: bool) { self.reveal_thru_blk = Some(v); }
    pub fn putRevealDirection(&mut self, v: i32) { self.reveal_direction = Some(v); }
    pub fn putPrismIsContent(&mut self, v: bool) { self.prism_is_content = Some(v); }
    pub fn putPrismIsInverted(&mut self, v: bool) { self.prism_is_inverted = Some(v); }
    pub fn putPrismDirection(&mut self, v: i32) { self.prism_direction = Some(v); }
    pub fn putGlitterPattern(&mut self, v: i32) { self.glitter_pattern = Some(v); }
    pub fn putGlitterDirection(&mut self, v: i32) { self.glitter_direction = Some(v); }
    pub fn putFlythroughDirection(&mut self, v: i32) { self.flythrough_direction = Some(v); }
    pub fn putFlythroughHasBounce(&mut self, v: bool) { self.flythrough_has_bounce = Some(v); }
    pub fn putSwitchDirection(&mut self, v: i32) { self.switch_direction = Some(v); }
    pub fn putFlipDirection(&mut self, v: i32) { self.flip_direction = Some(v); }
    pub fn putFerrisDirection(&mut self, v: i32) { self.ferris_direction = Some(v); }
    pub fn putGalleryDirection(&mut self, v: i32) { self.gallery_direction = Some(v); }
    pub fn putConveyorDirection(&mut self, v: i32) { self.conveyor_direction = Some(v); }
    pub fn putBlindsDirection(&mut self, v: i32) { self.blinds_direction = Some(v); }
    pub fn putCheckerDirection(&mut self, v: i32) { self.checker_direction = Some(v); }
    pub fn putCombDirection(&mut self, v: i32) { self.comb_direction = Some(v); }
    pub fn putRandomBarDirection(&mut self, v: i32) { self.random_bar_direction = Some(v); }
    pub fn putDoorsDirection(&mut self, v: i32) { self.doors_direction = Some(v); }
    pub fn putWindowDirection(&mut self, v: i32) { self.window_direction = Some(v); }
    pub fn putCoverDirection(&mut self, v: i32) { self.cover_direction = Some(v); }
    pub fn putPullDirection(&mut self, v: i32) { self.pull_direction = Some(v); }
    pub fn putStripsDirection(&mut self, v: i32) { self.strips_direction = Some(v); }
    pub fn putPushDirection(&mut self, v: i32) { self.push_direction = Some(v); }
    pub fn putWipeDirection(&mut self, v: i32) { self.wipe_direction = Some(v); }
    pub fn putVortexDirection(&mut self, v: i32) { self.vortex_direction = Some(v); }
    pub fn putPanDirection(&mut self, v: i32) { self.pan_direction = Some(v); }
    pub fn putZoomDirection(&mut self, v: i32) { self.zoom_direction = Some(v); }
    pub fn putWarpDirection(&mut self, v: i32) { self.warp_direction = Some(v); }
    pub fn putSplitInOutDirection(&mut self, v: i32) { self.split_in_out_direction = Some(v); }
    pub fn putSplitOrientationDirection(&mut self, v: i32) { self.split_orientation_direction = Some(v); }
    pub fn putWheelSpokes(&mut self, v: i32) { self.wheel_spokes = Some(v); }
    pub fn putWheelRevereSpokes(&mut self, v: i32) { self.wheel_revere_spokes = Some(v); }
    pub fn putCutOptionalBlackTransition(&mut self, v: i32) { self.cut_optional_black = Some(v); }
    pub fn putFadeOptionalBlackTransition(&mut self, v: bool) { self.fade_optional_black = Some(v); }
}

pub struct ParkedHandler {
    pub kind_name: &'static str,
}

impl ElementHandler for ParkedHandler {
    fn kind(&self) -> &'static str { self.kind_name }
    fn startElement(&mut self, _e: i32, _a: &XmlRoAttrList) {}
    fn enterSubElement(&mut self, _s: i32) -> Option<Box<dyn ElementHandler>> { None }
    fn endElement(&mut self, _e: i32) {}
}
