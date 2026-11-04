// Java: cn.wps.moffice.writer.data.math.BarProp
#[derive(Clone, Debug)]
pub struct BarProp {
    pub pos: i32,
}

impl Default for BarProp {
    fn default() -> Self {
        Self { pos: 1 }
    }
}
