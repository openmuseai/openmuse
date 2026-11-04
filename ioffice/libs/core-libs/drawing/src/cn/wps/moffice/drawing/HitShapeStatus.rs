#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::sync::Mutex;

// Java field: ThreadLocal<HitShapeStatus> sHitStatus
static sHitStatus: Mutex<Option<HitShapeStatus>> = Mutex::new(None);

// Java: public class HitShapeStatus
#[derive(Clone)]
pub struct HitShapeStatus {
    // Java field: boolean isFromMouse
    isFromMouse: bool,
    // Java field: boolean isInClip
    isInClip: bool,
    // Java field: boolean isHoverEvent
    isHoverEvent: bool,
    // Java field: boolean isMultiGridState
    isMultiGridState: bool,
}

// Java: public class HitShapeStatus
impl HitShapeStatus {
    // Java: HitShapeStatus()
    pub fn new() -> Self {
        // Java: super();
        HitShapeStatus {
            isFromMouse: false,
            isInClip: false,
            isHoverEvent: false,
            isMultiGridState: false,
        }
    }

    // Java: static HitShapeStatus getStatus(boolean fourceCreate)
    fn getStatus(fourceCreate: bool) -> Option<HitShapeStatus> {
        // Java: HitShapeStatus status = (HitShapeStatus)sHitStatus.get();
        let mut guard = sHitStatus.lock().unwrap();
        let status = guard.clone();
        // Java: if (status == null && fourceCreate)
        if status.is_none() && fourceCreate {
            // Java: status = new HitShapeStatus();
            let new_status = HitShapeStatus::new();
            // Java: sHitStatus.set(status);
            *guard = Some(new_status.clone());
            return Some(new_status);
        }
        // Java: return status;
        status
    }

    // Java: static void setIsFromMouse(boolean value)
    pub fn set_is_from_mouse(value: bool) {
        // Java: getStatus(true).isFromMouse = value;
        if let Some(mut status) = Self::getStatus(true) {
            status.isFromMouse = value;
            *sHitStatus.lock().unwrap() = Some(status);
        }
    }

    // Java: static void setIsInClip(boolean value)
    pub fn set_is_in_clip(value: bool) {
        // Java: getStatus(true).isInClip = value;
        if let Some(mut status) = Self::getStatus(true) {
            status.isInClip = value;
            *sHitStatus.lock().unwrap() = Some(status);
        }
    }

    // Java: static void setIsHoverEvent(boolean value)
    pub fn set_is_hover_event(value: bool) {
        // Java: getStatus(true).isHoverEvent = value;
        if let Some(mut status) = Self::getStatus(true) {
            status.isHoverEvent = value;
            *sHitStatus.lock().unwrap() = Some(status);
        }
    }

    // Java: static void setIsMultiGridState(boolean value)
    pub fn set_is_multi_grid_state(value: bool) {
        // Java: getStatus(true).isMultiGridState = value;
        if let Some(mut status) = Self::getStatus(true) {
            status.isMultiGridState = value;
            *sHitStatus.lock().unwrap() = Some(status);
        }
    }

    // Java: static boolean isFromMouse()
    pub fn is_from_mouse() -> bool {
        // Java: HitShapeStatus status = getStatus(false);
        let status = Self::getStatus(false);
        // Java: return status != null ? status.isFromMouse : false;
        status.map_or(false, |s| s.isFromMouse)
    }

    // Java: static boolean isInClip()
    pub fn is_in_clip() -> bool {
        // Java: HitShapeStatus status = getStatus(false);
        let status = Self::getStatus(false);
        // Java: return status != null ? status.isInClip : false;
        status.map_or(false, |s| s.isInClip)
    }

    // Java: static boolean isHoverEvent()
    pub fn is_hover_event() -> bool {
        // Java: HitShapeStatus status = getStatus(false);
        let status = Self::getStatus(false);
        // Java: return status != null ? status.isHoverEvent : false;
        status.map_or(false, |s| s.isHoverEvent)
    }

    // Java: static boolean isMultiGridState()
    pub fn is_multi_grid_state() -> bool {
        // Java: HitShapeStatus status = getStatus(false);
        let status = Self::getStatus(false);
        // Java: return status != null ? status.isMultiGridState : false;
        status.map_or(false, |s| s.isMultiGridState)
    }
}
