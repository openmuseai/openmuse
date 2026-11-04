// Java: cn.wps.moffice.writer.data.math.AccProp
#[derive(Clone, Debug)]
pub struct AccProp {
    pub chr: char,
}

impl Default for AccProp {
    fn default() -> Self {
        Self { chr: char::from_u32(770).unwrap_or('\u{0302}') }
    }
}
