// Java: cn.wps.moffice.writer.data.math.GroupChrProp
use util::moffice::property::PropertySet::PropertySet;

#[derive(Clone, Debug)]
pub struct GroupChrProp {
    pub chr: char,
    pub pos: i32,
    pub vertJc: i32,
}

impl Default for GroupChrProp {
    fn default() -> Self {
        Self { chr: char::from_u32(770).unwrap_or('\u{0302}'), pos: 1, vertJc: 0 }
    }
}

impl GroupChrProp {
    pub fn from_property(property: &PropertySet) -> Self {
        property.getValue(737)
            .and_then(|value| value.as_ref().as_any().downcast_ref::<GroupChrProp>().cloned())
            .unwrap_or_default()
    }
}
