use std::cell::RefCell;
use std::rc::Rc;

/// Compile-surface stand-in for the `HandlerCallback` methods this fill function
/// actually calls (`onBlipEmbed` / `onBlipLink` / `onAdjustThemeColor`).
#[derive(Clone, Debug, Default)]
pub struct FillNotify {
    pub blip_embed: Vec<String>,
    pub blip_link: Vec<String>,
    pub theme_belonger: Vec<i32>,
}

pub type FillNotifyRef = Option<Rc<RefCell<FillNotify>>>;
