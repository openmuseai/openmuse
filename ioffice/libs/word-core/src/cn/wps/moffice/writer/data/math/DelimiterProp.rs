// Java: cn.wps.moffice.writer.data.math.DelimiterProp
#[derive(Clone, Debug)]
pub struct DelimiterProp {
    pub begChr: char,
    pub sepChr: char,
    pub endChr: char,
    pub grow: bool,
    pub shp: i32,
}

impl Default for DelimiterProp {
    fn default() -> Self {
        Self { begChr: '(', sepChr: '|', endChr: ')', grow: true, shp: 1 }
    }
}
