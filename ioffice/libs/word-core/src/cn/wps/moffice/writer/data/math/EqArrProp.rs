// Java: cn.wps.moffice.writer.data.math.EqArrProp
#[derive(Clone, Debug)]
pub struct EqArrProp {
    pub baseJc: i32,
    pub maxDist: bool,
    pub objDist: bool,
    pub rSpRule: i32,
    pub rSp: i32,
}

impl Default for EqArrProp {
    fn default() -> Self {
        Self { baseJc: -8, maxDist: false, objDist: false, rSpRule: 0, rSp: 0 }
    }
}
