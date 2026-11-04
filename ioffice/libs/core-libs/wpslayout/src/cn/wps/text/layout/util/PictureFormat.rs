#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public enum PictureFormat implements Parcelable
pub struct PictureFormat {
    // Java field: PictureFormat[] mFormats
    // Java field: int format
    format: i32,
}

impl PictureFormat {
    // Java: PictureFormat(int format)
    pub fn new(_format: i32) -> Self {
        // Java: this.format = format;
        PictureFormat { format: _format }
    }

    // Java: int describeContents()
    pub fn describeContents(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: void writeToParcel(Parcel dest, int flags)
    pub fn writeToParcel(&self, _dest: i32, _flags: i32) {
        // Java: dest.writeInt(this.format);
    }
}

pub const JPEG: i32 = 0;
pub const PNG: i32 = 1;
pub const BMP24: i32 = 2;
pub const BMP24GRAY: i32 = 3;
pub const BMP8GRAY: i32 = 4;
pub const BMP1: i32 = 5;
