#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use super::PictureFormat;

// Java: public class PictureTool
pub struct PictureTool {
}

// Java: public class PictureTool
impl PictureTool {
    // Java: PictureTool()
    pub fn new() -> Self {
        // Java: super();
        PictureTool {}
    }

    // Java: static boolean isBMP(PictureFormat format)
    pub fn isBMP(_format: i32) -> bool {
        // Java: return format == PictureFormat.BMP24 || format == PictureFormat.BMP24GRAY || format == PictureFormat.BMP8GRAY || format == PictureFormat.BMP1;
        _format == PictureFormat::BMP24 || _format == PictureFormat::BMP24GRAY || _format == PictureFormat::BMP8GRAY || _format == PictureFormat::BMP1
    }

    // Java: static boolean isGray(PictureFormat format)
    pub fn isGray(_format: i32) -> bool {
        // Java: return format == PictureFormat.BMP24GRAY || format == PictureFormat.BMP8GRAY || format == PictureFormat.BMP1;
        _format == PictureFormat::BMP24GRAY || _format == PictureFormat::BMP8GRAY || _format == PictureFormat::BMP1
    }

    // Java: static boolean isColorIndexBMP(PictureFormat format)
    pub fn isColorIndexBMP(_format: i32) -> bool {
        // Java: return format == PictureFormat.BMP1 || format == PictureFormat.BMP8GRAY;
        _format == PictureFormat::BMP1 || _format == PictureFormat::BMP8GRAY
    }

    // Java: static int getColorIndexCount(PictureFormat format)
    pub fn getColorIndexCount(_format: i32) -> i32 {
        // Java: if (format == PictureFormat.BMP1)
        if _format == PictureFormat::BMP1
        {
            // Java: return 2;
            return 2;
        }
        // Java: else
        else
        {
            // Java: return format == PictureFormat.BMP8GRAY ? 256 : 0;
            return if _format == PictureFormat::BMP8GRAY { 256 } else { 0 };
        }
    }

    // Java: static int getBitCount(PictureFormat format)
    pub fn getBitCount(_format: i32) -> i32 {
        // Java: if (format == PictureFormat.BMP1)
        if _format == PictureFormat::BMP1
        {
            // Java: return 1;
            return 1;
        }
        // Java: else
        else
        {
            // Java: return format == PictureFormat.BMP8GRAY ? 8 : 24;
            return if _format == PictureFormat::BMP8GRAY { 8 } else { 24 };
        }
    }
}
