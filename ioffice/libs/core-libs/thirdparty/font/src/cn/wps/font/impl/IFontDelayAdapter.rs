// Java: import cn.wps.font.IFontFamily
use crate::cn::wps::font::IFontFamily::IFontFamily;
use std::sync::Arc;

// Java: public interface IFontDelayAdapter
pub trait IFontDelayAdapter: Send + Sync {
    // Java: abstract IFontFamily validFont()
    fn validFont(&self) -> Option<Arc<dyn IFontFamily>>;
}
