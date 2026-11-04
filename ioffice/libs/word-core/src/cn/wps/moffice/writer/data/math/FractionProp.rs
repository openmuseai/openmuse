// Java: cn.wps.moffice.writer.data.math.FractionProp
#[derive(Clone, Debug)]
pub struct FractionProp {
    pub fType: i32,
}

impl Default for FractionProp {
    fn default() -> Self {
        Self { fType: 1 }
    }
}
