// java package cn.wps.moss.core.drawing;

// java public class KmoCellPic
#[derive(Clone)]
pub struct KmoCellPic {
    // java public int mType = -1;
    pub mType: i32,
}

impl KmoCellPic {
    // java public static final KmoCellPic emptyKmoCellPic = new KmoCellPic();
    pub fn emptyKmoCellPic() -> Self {
        // java this.mType = -1;
        Self { mType: -1 }
    }
}
