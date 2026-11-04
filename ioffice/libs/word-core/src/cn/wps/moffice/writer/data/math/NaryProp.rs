// Java: cn.wps.moffice.writer.data.math.NaryProp
use util::moffice::property::PropertySet::PropertySet;

#[derive(Clone, Debug)]
pub struct NaryProp {
    pub chr: char,
    pub grow: bool,
    pub subHide: bool,
    pub supHide: bool,
    pub limLoc: Option<i32>,
}

impl Default for NaryProp {
    fn default() -> Self {
        Self { chr: char::from_u32(8747).unwrap_or('\u{222b}'), grow: false, subHide: false, supHide: false, limLoc: None }
    }
}

impl NaryProp {
    pub fn from_property(property: &PropertySet) -> Self {
        property.getValue(737)
            .and_then(|value| value.as_ref().as_any().downcast_ref::<NaryProp>().cloned())
            .unwrap_or_default()
    }
}
