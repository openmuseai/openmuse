// Java: import android.graphics.Bitmap
// Java: import android.graphics.BitmapFactory
// Java: import java.io.BufferedInputStream
// Java: import java.io.ByteArrayInputStream
// Java: import java.io.InputStream
// Java: import java.nio.ByteBuffer
// Java: import java.util.Arrays

pub struct Bitmap;
impl Bitmap {
    pub fn decodeByteArray(_data: &[u8], _offset: usize, _len: usize) -> Option<Bitmap> { Some(Bitmap) }
}

struct Arrays;
impl Arrays {
    fn equals(a: &[u8], b: &[u8]) -> bool { a == b }
}

struct System;
impl System {
    fn arraycopy(src: &[u8], src_pos: usize, dst: &mut [u8], dst_pos: usize, length: usize) {
        dst[dst_pos..dst_pos + length].copy_from_slice(&src[src_pos..src_pos + length]);
    }
}

struct ByteBuffer<'a> {
    data: &'a [u8],
}
impl<'a> ByteBuffer<'a> {
    fn wrap(data: &'a [u8], offset: usize, length: usize) -> Self {
        ByteBuffer { data: &data[offset..offset + length] }
    }
    fn get(&self, dst: &mut [u8], offset: usize, length: usize) {
        dst[offset..offset + length].copy_from_slice(&self.data[..length]);
    }
}


fn to_u8_vec(signed: &[i8]) -> Vec<u8> {
    signed.iter().map(|&b| b as u8).collect()
}

// Java field: byte[] msoPattern5Percent
#[allow(non_upper_case_globals)]
static msoPattern5Percent: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPattern10Percent
#[allow(non_upper_case_globals)]
static msoPattern10Percent: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPattern20Percent
#[allow(non_upper_case_globals)]
static msoPattern20Percent: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPattern25Percent
#[allow(non_upper_case_globals)]
static msoPattern25Percent: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPattern30Percent
#[allow(non_upper_case_globals)]
static msoPattern30Percent: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPattern40Percent
#[allow(non_upper_case_globals)]
static msoPattern40Percent: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPattern50Percent
#[allow(non_upper_case_globals)]
static msoPattern50Percent: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPattern60Percent
#[allow(non_upper_case_globals)]
static msoPattern60Percent: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPattern70Percent
#[allow(non_upper_case_globals)]
static msoPattern70Percent: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPattern75Percent
#[allow(non_upper_case_globals)]
static msoPattern75Percent: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPattern80Percent
#[allow(non_upper_case_globals)]
static msoPattern80Percent: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPattern90Percent
#[allow(non_upper_case_globals)]
static msoPattern90Percent: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternDarkDownwardDiagonal
#[allow(non_upper_case_globals)]
static msoPatternDarkDownwardDiagonal: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternDarkHorizontal
#[allow(non_upper_case_globals)]
static msoPatternDarkHorizontal: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternDarkUpwardDiagonal
#[allow(non_upper_case_globals)]
static msoPatternDarkUpwardDiagonal: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternDarkVertical
#[allow(non_upper_case_globals)]
static msoPatternDarkVertical: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternDashedDownwardDiagonal
#[allow(non_upper_case_globals)]
static msoPatternDashedDownwardDiagonal: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternDashedHorizontal
#[allow(non_upper_case_globals)]
static msoPatternDashedHorizontal: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternDashedUpwardDiagonal
#[allow(non_upper_case_globals)]
static msoPatternDashedUpwardDiagonal: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternDashedVertical
#[allow(non_upper_case_globals)]
static msoPatternDashedVertical: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternDiagonalBrick
#[allow(non_upper_case_globals)]
static msoPatternDiagonalBrick: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternOpenDiamond
#[allow(non_upper_case_globals)]
static msoPatternOpenDiamond: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternDivot
#[allow(non_upper_case_globals)]
static msoPatternDivot: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternDottedDiamond
#[allow(non_upper_case_globals)]
static msoPatternDottedDiamond: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternDottedGrid
#[allow(non_upper_case_globals)]
static msoPatternDottedGrid: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternHorizontalBrick
#[allow(non_upper_case_globals)]
static msoPatternHorizontalBrick: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternLargeCheckerBoard
#[allow(non_upper_case_globals)]
static msoPatternLargeCheckerBoard: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternLargeConfetti
#[allow(non_upper_case_globals)]
static msoPatternLargeConfetti: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternLargeGrid
#[allow(non_upper_case_globals)]
static msoPatternLargeGrid: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternLightDownwardDiagonal
#[allow(non_upper_case_globals)]
static msoPatternLightDownwardDiagonal: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternLightHorizontal
#[allow(non_upper_case_globals)]
static msoPatternLightHorizontal: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternLightUpwardDiagonal
#[allow(non_upper_case_globals)]
static msoPatternLightUpwardDiagonal: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternLightVertical
#[allow(non_upper_case_globals)]
static msoPatternLightVertical: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternNarrowHorizontal
#[allow(non_upper_case_globals)]
static msoPatternNarrowHorizontal: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternNarrowVertical
#[allow(non_upper_case_globals)]
static msoPatternNarrowVertical: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternPlaid
#[allow(non_upper_case_globals)]
static msoPatternPlaid: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternShingle
#[allow(non_upper_case_globals)]
static msoPatternShingle: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternSmallCheckerBoard
#[allow(non_upper_case_globals)]
static msoPatternSmallCheckerBoard: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternSmallConfetti
#[allow(non_upper_case_globals)]
static msoPatternSmallConfetti: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternSmallGrid
#[allow(non_upper_case_globals)]
static msoPatternSmallGrid: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternWeave
#[allow(non_upper_case_globals)]
static msoPatternWeave: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternSolidDiamond
#[allow(non_upper_case_globals)]
static msoPatternSolidDiamond: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternWdDnDiag
#[allow(non_upper_case_globals)]
static msoPatternWdDnDiag: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternSphere
#[allow(non_upper_case_globals)]
static msoPatternSphere: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternWdUpDiag
#[allow(non_upper_case_globals)]
static msoPatternWdUpDiag: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternTrellis
#[allow(non_upper_case_globals)]
static msoPatternTrellis: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternZigZag
#[allow(non_upper_case_globals)]
static msoPatternZigZag: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
// Java field: byte[] msoPatternWave
#[allow(non_upper_case_globals)]
static msoPatternWave: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();

fn init_patterns() {
    let _ = msoPattern5Percent.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-9,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,127,0,0,0]));
    let _ = msoPattern10Percent.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-1,0,0,0,-9,0,0,0,-1,0,0,0,127,0,0,0,-1,0,0,0,-9,0,0,0,-1,0,0,0,127,0,0,0]));
    let _ = msoPattern20Percent.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-1,0,0,0,-35,0,0,0,-1,0,0,0,119,0,0,0,-1,0,0,0,-35,0,0,0,-1,0,0,0,119,0,0,0]));
    let _ = msoPattern25Percent.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-35,0,0,0,119,0,0,0,-35,0,0,0,119,0,0,0,-35,0,0,0,119,0,0,0,-35,0,0,0,119,0,0,0]));
    let _ = msoPattern30Percent.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-18,0,0,0,85,0,0,0,-69,0,0,0,85,0,0,0,-18,0,0,0,85,0,0,0,-69,0,0,0,85,0,0,0]));
    let _ = msoPattern40Percent.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-22,0,0,0,85,0,0,0,-86,0,0,0,85,0,0,0,-82,0,0,0,85,0,0,0,-86,0,0,0,85,0,0,0]));
    let _ = msoPattern50Percent.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-86,0,0,0,85,0,0,0,-86,0,0,0,85,0,0,0,-86,0,0,0,85,0,0,0,-86,0,0,0,85,0,0,0]));
    let _ = msoPattern60Percent.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-86,0,0,0,68,0,0,0,-86,0,0,0,17,0,0,0,-86,0,0,0,68,0,0,0,-86,0,0,0,17,0,0,0]));
    let _ = msoPattern70Percent.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,34,0,0,0,-120,0,0,0,34,0,0,0,-120,0,0,0,34,0,0,0,-120,0,0,0,34,0,0,0,-120,0,0,0]));
    let _ = msoPattern75Percent.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,0,0,0,0,34,0,0,0,0,0,0,0,-120,0,0,0,0,0,0,0,34,0,0,0,0,0,0,0,-120,0,0,0]));
    let _ = msoPattern80Percent.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,16,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,16,0,0,0]));
    let _ = msoPattern90Percent.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-128,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]));
    let _ = msoPatternDarkDownwardDiagonal.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,102,0,0,0,-52,0,0,0,-103,0,0,0,51,0,0,0,102,0,0,0,-52,0,0,0,-103,0,0,0,51,0,0,0]));
    let _ = msoPatternDarkHorizontal.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-1,0,0,0,-1,0,0,0,0,0,0,0,0,0,0,0,-1,0,0,0,-1,0,0,0,0,0,0,0,0,0,0,0]));
    let _ = msoPatternDarkUpwardDiagonal.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,102,0,0,0,51,0,0,0,-103,0,0,0,-52,0,0,0,102,0,0,0,51,0,0,0,-103,0,0,0,-52,0,0,0]));
    let _ = msoPatternDarkVertical.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,51,0,0,0,51,0,0,0,51,0,0,0,51,0,0,0,51,0,0,0,51,0,0,0,51,0,0,0,51,0,0,0]));
    let _ = msoPatternDashedDownwardDiagonal.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-18,0,0,0,-35,0,0,0,-69,0,0,0,119,0,0,0,-1,0,0,0,-1,0,0,0]));
    let _ = msoPatternDashedHorizontal.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,-16,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,15,0,0,0]));
    let _ = msoPatternDashedUpwardDiagonal.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-1,0,0,0,-1,0,0,0,119,0,0,0,-69,0,0,0,-35,0,0,0,-18,0,0,0,-1,0,0,0,-1,0,0,0]));
    let _ = msoPatternDashedVertical.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-9,0,0,0,-9,0,0,0,-9,0,0,0,-9,0,0,0,127,0,0,0,127,0,0,0,127,0,0,0,127,0,0,0]));
    let _ = msoPatternDiagonalBrick.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,126,0,0,0,-67,0,0,0,-37,0,0,0,-25,0,0,0,-9,0,0,0,-5,0,0,0,-3,0,0,0,-2,0,0,0]));
    let _ = msoPatternOpenDiamond.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-2,0,0,0,125,0,0,0,-69,0,0,0,-41,0,0,0,-17,0,0,0,-41,0,0,0,-69,0,0,0,125,0,0,0]));
    let _ = msoPatternDivot.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,127,0,0,0,-2,0,0,0,127,0,0,0,-1,0,0,0,-17,0,0,0,-9,0,0,0,-17,0,0,0,-1,0,0,0]));
    let _ = msoPatternDottedDiamond.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-1,0,0,0,-35,0,0,0,-1,0,0,0,-9,0,0,0,-1,0,0,0,-35,0,0,0,-1,0,0,0,127,0,0,0]));
    let _ = msoPatternDottedGrid.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-1,0,0,0,127,0,0,0,-1,0,0,0,127,0,0,0,-1,0,0,0,127,0,0,0,-1,0,0,0,85,0,0,0]));
    let _ = msoPatternHorizontalBrick.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-9,0,0,0,-9,0,0,0,-9,0,0,0,0,0,0,0,127,0,0,0,127,0,0,0,127,0,0,0,0,0,0,0]));
    let _ = msoPatternLargeCheckerBoard.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-16,0,0,0,-16,0,0,0,-16,0,0,0,-16,0,0,0,15,0,0,0,15,0,0,0,15,0,0,0,15,0,0,0]));
    let _ = msoPatternLargeConfetti.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,114,0,0,0,-13,0,0,0,63,0,0,0,39,0,0,0,-28,0,0,0,-4,0,0,0,-49,0,0,0,78,0,0,0]));
    let _ = msoPatternLargeGrid.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,127,0,0,0,127,0,0,0,127,0,0,0,127,0,0,0,127,0,0,0,127,0,0,0,127,0,0,0,0,0,0,0]));
    let _ = msoPatternLightDownwardDiagonal.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-18,0,0,0,-35,0,0,0,-69,0,0,0,119,0,0,0,-18,0,0,0,-35,0,0,0,-69,0,0,0,119,0,0,0]));
    let _ = msoPatternLightHorizontal.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,0,0,0,0,-1,0,0,0,-1,0,0,0,-1,0,0,0,0,0,0,0]));
    let _ = msoPatternLightUpwardDiagonal.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,119,0,0,0,-69,0,0,0,-35,0,0,0,-18,0,0,0,119,0,0,0,-69,0,0,0,-35,0,0,0,-18,0,0,0]));
    let _ = msoPatternLightVertical.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,119,0,0,0,119,0,0,0,119,0,0,0,119,0,0,0,119,0,0,0,119,0,0,0,119,0,0,0,119,0,0,0]));
    let _ = msoPatternNarrowHorizontal.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-1,0,0,0,0,0,0,0,-1,0,0,0,0,0,0,0,-1,0,0,0,0,0,0,0,-1,0,0,0,0,0,0,0]));
    let _ = msoPatternNarrowVertical.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-86,0,0,0,-86,0,0,0,-86,0,0,0,-86,0,0,0,-86,0,0,0,-86,0,0,0,-86,0,0,0,-86,0,0,0]));
    let _ = msoPatternPlaid.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,15,0,0,0,15,0,0,0,15,0,0,0,15,0,0,0,-86,0,0,0,85,0,0,0,-86,0,0,0,85,0,0,0]));
    let _ = msoPatternShingle.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-2,0,0,0,-2,0,0,0,-3,0,0,0,-13,0,0,0,-49,0,0,0,-73,0,0,0,123,0,0,0,-4,0,0,0]));
    let _ = msoPatternSmallCheckerBoard.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,102,0,0,0,-103,0,0,0,-103,0,0,0,102,0,0,0,102,0,0,0,-103,0,0,0,-103,0,0,0,102,0,0,0]));
    let _ = msoPatternSmallConfetti.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-5,0,0,0,-33,0,0,0,-2,0,0,0,-17,0,0,0,-3,0,0,0,-65,0,0,0,-9,0,0,0,127,0,0,0]));
    let _ = msoPatternSmallGrid.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,119,0,0,0,119,0,0,0,119,0,0,0,0,0,0,0,119,0,0,0,119,0,0,0,119,0,0,0,0,0,0,0]));
    let _ = msoPatternWeave.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-82,0,0,0,-35,0,0,0,-21,0,0,0,119,0,0,0,-70,0,0,0,-35,0,0,0,-85,0,0,0,119,0,0,0]));
    let _ = msoPatternSolidDiamond.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-1,0,0,0,-17,0,0,0,-57,0,0,0,-125,0,0,0,1,0,0,0,-125,0,0,0,-57,0,0,0,-17,0,0,0]));
    let _ = msoPatternWdDnDiag.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,124,0,0,0,-8,0,0,0,-15,0,0,0,-29,0,0,0,-57,0,0,0,-113,0,0,0,31,0,0,0,62,0,0,0]));
    let _ = msoPatternSphere.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,7,0,0,0,7,0,0,0,103,0,0,0,-120,0,0,0,112,0,0,0,112,0,0,0,118,0,0,0,-120,0,0,0]));
    let _ = msoPatternWdUpDiag.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,62,0,0,0,31,0,0,0,-113,0,0,0,-57,0,0,0,-29,0,0,0,-15,0,0,0,-8,0,0,0,124,0,0,0]));
    let _ = msoPatternTrellis.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,102,0,0,0,0,0,0,0,-103,0,0,0,0,0,0,0,102,0,0,0,0,0,0,0,-103,0,0,0,0,0,0,0]));
    let _ = msoPatternZigZag.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,-25,0,0,0,-37,0,0,0,-67,0,0,0,126,0,0,0,-25,0,0,0,-37,0,0,0,-67,0,0,0,126,0,0,0]));
    let _ = msoPatternWave.get_or_init(|| to_u8_vec(&[66,77,94,0,0,0,0,0,0,0,62,0,0,0,40,0,0,0,8,0,0,0,8,0,0,0,1,0,1,0,0,0,0,0,32,0,0,0,-119,11,0,0,-119,11,0,0,2,0,0,0,2,0,0,0,-1,-1,-1,0,0,0,0,0,63,0,0,0,-38,0,0,0,-25,0,0,0,-1,0,0,0,63,0,0,0,-38,0,0,0,-25,0,0,0,-1,0,0,0]));
}

// Java: public class PatternFillUtil
pub struct PatternFillUtil;

// Java: public enum PatternFillType
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PatternFillType {
    Pct5, Pct10, Pct20, Pct25, Pct30, Pct40, Pct50, Pct60, Pct70, Pct75, Pct80, Pct90,
    Horz, Vert, LtHorz, LtVert, DkHorz, DkVert, NarHorz, NarVert, DashHorz, DashVert,
    Cross, DnDiag, UpDiag, LtDnDiag, LtUpDiag, DkDnDiag, DkUpDiag, WdDnDiag, WdUpDiag,
    DashDnDiag, DashUpDiag, DiagCross, SmCheck, LgCheck, SmGrid, LgGrid, DotGrid,
    SmConfetti, LgConfetti, HorzBrick, DiagBrick, SolidDmnd, OpenDmnd, DotDmnd,
    Plaid, Sphere, Weave, Divot, Shingle, Wave, Trellis, ZigZag,
}

// Java: public class PatternFillUtil
impl PatternFillUtil {
    // Java: PatternFillUtil()
    pub fn new() -> Self {
        // Java: super();
        init_patterns();
        PatternFillUtil
    }

    // Java: static InputStream getStreamIDByName(String prst, int fgColor, int bgColor)
    pub fn getStreamIDByName_with_colors(prst: &str, fgColor: i32, bgColor: i32) -> Option<Vec<u8>> {
        // Java: byte[] prstData = prstStrMatcheToBitByte(prst);
        let prstData = PatternFillUtil::prstStrMatcheToBitByte(prst);
        // Java: return new BufferedInputStream(new ByteArrayInputStream(resetPatternData(prstData, fgColor, bgColor)));
        PatternFillUtil::resetPatternData(Some(&prstData), fgColor, bgColor)
    }

    // Java: static InputStream getStreamIDByName(String prst)
    pub fn getStreamIDByName(prst: &str) -> Vec<u8> {
        // Java: byte[] prstData = prstStrMatcheToBitByte(prst);
        let prstData = PatternFillUtil::prstStrMatcheToBitByte(prst);
        // Java: return new BufferedInputStream(new ByteArrayInputStream(prstData));
        prstData
    }

    // Java: static Bitmap getPatternBitmap(String prst, int fgColor, int bgColor)
    pub fn getPatternBitmap(prst: &str, fgColor: i32, bgColor: i32) -> Option<Bitmap> {
        // Java: byte[] prstData = prstStrMatcheToBitByte(prst);
        let prstData = PatternFillUtil::prstStrMatcheToBitByte(prst);
        // Java: return decodeBitmap(prstData, fgColor, bgColor);
        PatternFillUtil::decodeBitmap(&prstData, fgColor, bgColor)
    }

    // Java: static String bitByteMatcheToPrstStr(byte[] data)
    pub fn bitByteMatcheToPrstStr(data: &[u8]) -> &'static str {
        init_patterns();
        // Java: if (Arrays.equals(data, msoPattern5Percent))
        if data == msoPattern5Percent.get().unwrap().as_slice() {
            // Java: return "pct5";
            return "pct5";
        }
        // Java: else if (Arrays.equals(data, msoPattern10Percent))
        else if data == msoPattern10Percent.get().unwrap().as_slice() {
            // Java: return "pct10";
            return "pct10";
        }
        // Java: else if (Arrays.equals(data, msoPattern20Percent))
        else if data == msoPattern20Percent.get().unwrap().as_slice() {
            // Java: return "pct20";
            return "pct20";
        }
        // Java: else if (Arrays.equals(data, msoPattern25Percent))
        else if data == msoPattern25Percent.get().unwrap().as_slice() {
            // Java: return "pct25";
            return "pct25";
        }
        // Java: else if (Arrays.equals(data, msoPattern30Percent))
        else if data == msoPattern30Percent.get().unwrap().as_slice() {
            // Java: return "pct30";
            return "pct30";
        }
        // Java: else if (Arrays.equals(data, msoPattern40Percent))
        else if data == msoPattern40Percent.get().unwrap().as_slice() {
            // Java: return "pct40";
            return "pct40";
        }
        // Java: else if (Arrays.equals(data, msoPattern50Percent))
        else if data == msoPattern50Percent.get().unwrap().as_slice() {
            // Java: return "pct50";
            return "pct50";
        }
        // Java: else if (Arrays.equals(data, msoPattern60Percent))
        else if data == msoPattern60Percent.get().unwrap().as_slice() {
            // Java: return "pct60";
            return "pct60";
        }
        // Java: else if (Arrays.equals(data, msoPattern70Percent))
        else if data == msoPattern70Percent.get().unwrap().as_slice() {
            // Java: return "pct70";
            return "pct70";
        }
        // Java: else if (Arrays.equals(data, msoPattern75Percent))
        else if data == msoPattern75Percent.get().unwrap().as_slice() {
            // Java: return "pct75";
            return "pct75";
        }
        // Java: else if (Arrays.equals(data, msoPattern80Percent))
        else if data == msoPattern80Percent.get().unwrap().as_slice() {
            // Java: return "pct80";
            return "pct80";
        }
        // Java: else if (Arrays.equals(data, msoPattern90Percent))
        else if data == msoPattern90Percent.get().unwrap().as_slice() {
            // Java: return "pct90";
            return "pct90";
        }
        // Java: else if (Arrays.equals(data, msoPatternLightHorizontal))
        else if data == msoPatternLightHorizontal.get().unwrap().as_slice() {
            // Java: return "ltHorz";
            return "ltHorz";
        }
        // Java: else if (Arrays.equals(data, msoPatternLightVertical))
        else if data == msoPatternLightVertical.get().unwrap().as_slice() {
            // Java: return "ltVert";
            return "ltVert";
        }
        // Java: else if (Arrays.equals(data, msoPatternDarkHorizontal))
        else if data == msoPatternDarkHorizontal.get().unwrap().as_slice() {
            // Java: return "dkHorz";
            return "dkHorz";
        }
        // Java: else if (Arrays.equals(data, msoPatternDarkVertical))
        else if data == msoPatternDarkVertical.get().unwrap().as_slice() {
            // Java: return "dkVert";
            return "dkVert";
        }
        // Java: else if (Arrays.equals(data, msoPatternNarrowHorizontal))
        else if data == msoPatternNarrowHorizontal.get().unwrap().as_slice() {
            // Java: return "narHorz";
            return "narHorz";
        }
        // Java: else if (Arrays.equals(data, msoPatternNarrowVertical))
        else if data == msoPatternNarrowVertical.get().unwrap().as_slice() {
            // Java: return "narVert";
            return "narVert";
        }
        // Java: else if (Arrays.equals(data, msoPatternDashedHorizontal))
        else if data == msoPatternDashedHorizontal.get().unwrap().as_slice() {
            // Java: return "dashHorz";
            return "dashHorz";
        }
        // Java: else if (Arrays.equals(data, msoPatternDashedVertical))
        else if data == msoPatternDashedVertical.get().unwrap().as_slice() {
            // Java: return "dashVert";
            return "dashVert";
        }
        // Java: else if (Arrays.equals(data, msoPatternLightDownwardDiagonal))
        else if data == msoPatternLightDownwardDiagonal.get().unwrap().as_slice() {
            // Java: return "ltDnDiag";
            return "ltDnDiag";
        }
        // Java: else if (Arrays.equals(data, msoPatternLightUpwardDiagonal))
        else if data == msoPatternLightUpwardDiagonal.get().unwrap().as_slice() {
            // Java: return "ltUpDiag";
            return "ltUpDiag";
        }
        // Java: else if (Arrays.equals(data, msoPatternDarkDownwardDiagonal))
        else if data == msoPatternDarkDownwardDiagonal.get().unwrap().as_slice() {
            // Java: return "dkDnDiag";
            return "dkDnDiag";
        }
        // Java: else if (Arrays.equals(data, msoPatternDarkUpwardDiagonal))
        else if data == msoPatternDarkUpwardDiagonal.get().unwrap().as_slice() {
            // Java: return "dkUpDiag";
            return "dkUpDiag";
        }
        // Java: else if (Arrays.equals(data, msoPatternDashedDownwardDiagonal))
        else if data == msoPatternDashedDownwardDiagonal.get().unwrap().as_slice() {
            // Java: return "dashDnDiag";
            return "dashDnDiag";
        }
        // Java: else if (Arrays.equals(data, msoPatternDashedUpwardDiagonal))
        else if data == msoPatternDashedUpwardDiagonal.get().unwrap().as_slice() {
            // Java: return "dashUpDiag";
            return "dashUpDiag";
        }
        // Java: else if (Arrays.equals(data, msoPatternSmallCheckerBoard))
        else if data == msoPatternSmallCheckerBoard.get().unwrap().as_slice() {
            // Java: return "smCheck";
            return "smCheck";
        }
        // Java: else if (Arrays.equals(data, msoPatternLargeCheckerBoard))
        else if data == msoPatternLargeCheckerBoard.get().unwrap().as_slice() {
            // Java: return "lgCheck";
            return "lgCheck";
        }
        // Java: else if (Arrays.equals(data, msoPatternSmallGrid))
        else if data == msoPatternSmallGrid.get().unwrap().as_slice() {
            // Java: return "smGrid";
            return "smGrid";
        }
        // Java: else if (Arrays.equals(data, msoPatternLargeGrid))
        else if data == msoPatternLargeGrid.get().unwrap().as_slice() {
            // Java: return "lgGrid";
            return "lgGrid";
        }
        // Java: else if (Arrays.equals(data, msoPatternDottedGrid))
        else if data == msoPatternDottedGrid.get().unwrap().as_slice() {
            // Java: return "dotGrid";
            return "dotGrid";
        }
        // Java: else if (Arrays.equals(data, msoPatternSmallConfetti))
        else if data == msoPatternSmallConfetti.get().unwrap().as_slice() {
            // Java: return "smConfetti";
            return "smConfetti";
        }
        // Java: else if (Arrays.equals(data, msoPatternLargeConfetti))
        else if data == msoPatternLargeConfetti.get().unwrap().as_slice() {
            // Java: return "lgConfetti";
            return "lgConfetti";
        }
        // Java: else if (Arrays.equals(data, msoPatternHorizontalBrick))
        else if data == msoPatternHorizontalBrick.get().unwrap().as_slice() {
            // Java: return "horzBrick";
            return "horzBrick";
        }
        // Java: else if (Arrays.equals(data, msoPatternDiagonalBrick))
        else if data == msoPatternDiagonalBrick.get().unwrap().as_slice() {
            // Java: return "diagBrick";
            return "diagBrick";
        }
        // Java: else if (Arrays.equals(data, msoPatternWdDnDiag))
        else if data == msoPatternWdDnDiag.get().unwrap().as_slice() {
            // Java: return "wdDnDiag";
            return "wdDnDiag";
        }
        // Java: else if (Arrays.equals(data, msoPatternSolidDiamond))
        else if data == msoPatternSolidDiamond.get().unwrap().as_slice() {
            // Java: return "solidDmnd";
            return "solidDmnd";
        }
        // Java: else if (Arrays.equals(data, msoPatternOpenDiamond))
        else if data == msoPatternOpenDiamond.get().unwrap().as_slice() {
            // Java: return "openDmnd";
            return "openDmnd";
        }
        // Java: else if (Arrays.equals(data, msoPatternDottedDiamond))
        else if data == msoPatternDottedDiamond.get().unwrap().as_slice() {
            // Java: return "dotDmnd";
            return "dotDmnd";
        }
        // Java: else if (Arrays.equals(data, msoPatternPlaid))
        else if data == msoPatternPlaid.get().unwrap().as_slice() {
            // Java: return "plaid";
            return "plaid";
        }
        // Java: else if (Arrays.equals(data, msoPatternWdUpDiag))
        else if data == msoPatternWdUpDiag.get().unwrap().as_slice() {
            // Java: return "wdUpDiag";
            return "wdUpDiag";
        }
        // Java: else if (Arrays.equals(data, msoPatternSphere))
        else if data == msoPatternSphere.get().unwrap().as_slice() {
            // Java: return "sphere";
            return "sphere";
        }
        // Java: else if (Arrays.equals(data, msoPatternWeave))
        else if data == msoPatternWeave.get().unwrap().as_slice() {
            // Java: return "weave";
            return "weave";
        }
        // Java: else if (Arrays.equals(data, msoPatternDivot))
        else if data == msoPatternDivot.get().unwrap().as_slice() {
            // Java: return "divot";
            return "divot";
        }
        // Java: else if (Arrays.equals(data, msoPatternShingle))
        else if data == msoPatternShingle.get().unwrap().as_slice() {
            // Java: return "shingle";
            return "shingle";
        }
        // Java: else if (Arrays.equals(data, msoPatternWave))
        else if data == msoPatternWave.get().unwrap().as_slice() {
            // Java: return "wave";
            return "wave";
        }
        // Java: else if (Arrays.equals(data, msoPatternTrellis))
        else if data == msoPatternTrellis.get().unwrap().as_slice() {
            // Java: return "trellis";
            return "trellis";
        }
        // Java: else
        else {
            // Java: return Arrays.equals(data, msoPatternZigZag) ? "zigZag" : "pct5";
            if Arrays::equals(data, msoPatternZigZag.get().unwrap().as_slice()) { return "zigZag"; } else { return "pct5"; }
        }
    }

    // Java: static byte[] prstStrMatcheToBitByte(String prst)
    pub fn prstStrMatcheToBitByte(prst: &str) -> Vec<u8> {
        init_patterns();
        // Java: byte[] result = msoPattern5Percent;
        let result = msoPattern5Percent.get().unwrap();
        // Java: if ("pct5".equals(prst))
        if prst == "pct5" {
            // Java: return msoPattern5Percent;
            return msoPattern5Percent.get().unwrap().clone();
        }
        // Java: else if ("pct10".equals(prst))
        else if prst == "pct10" {
            // Java: return msoPattern10Percent;
            return msoPattern10Percent.get().unwrap().clone();
        }
        // Java: else if ("pct20".equals(prst))
        else if prst == "pct20" {
            // Java: return msoPattern20Percent;
            return msoPattern20Percent.get().unwrap().clone();
        }
        // Java: else if ("pct25".equals(prst))
        else if prst == "pct25" {
            // Java: return msoPattern25Percent;
            return msoPattern25Percent.get().unwrap().clone();
        }
        // Java: else if ("pct30".equals(prst))
        else if prst == "pct30" {
            // Java: return msoPattern30Percent;
            return msoPattern30Percent.get().unwrap().clone();
        }
        // Java: else if ("pct40".equals(prst))
        else if prst == "pct40" {
            // Java: return msoPattern40Percent;
            return msoPattern40Percent.get().unwrap().clone();
        }
        // Java: else if ("pct50".equals(prst))
        else if prst == "pct50" {
            // Java: return msoPattern50Percent;
            return msoPattern50Percent.get().unwrap().clone();
        }
        // Java: else if ("pct60".equals(prst))
        else if prst == "pct60" {
            // Java: return msoPattern60Percent;
            return msoPattern60Percent.get().unwrap().clone();
        }
        // Java: else if ("pct70".equals(prst))
        else if prst == "pct70" {
            // Java: return msoPattern70Percent;
            return msoPattern70Percent.get().unwrap().clone();
        }
        // Java: else if ("pct75".equals(prst))
        else if prst == "pct75" {
            // Java: return msoPattern75Percent;
            return msoPattern75Percent.get().unwrap().clone();
        }
        // Java: else if ("pct80".equals(prst))
        else if prst == "pct80" {
            // Java: return msoPattern80Percent;
            return msoPattern80Percent.get().unwrap().clone();
        }
        // Java: else if ("pct90".equals(prst))
        else if prst == "pct90" {
            // Java: return msoPattern90Percent;
            return msoPattern90Percent.get().unwrap().clone();
        }
        // Java: else if ("ltHorz".equals(prst))
        else if prst == "ltHorz" {
            // Java: return msoPatternLightHorizontal;
            return msoPatternLightHorizontal.get().unwrap().clone();
        }
        // Java: else if ("ltVert".equals(prst))
        else if prst == "ltVert" {
            // Java: return msoPatternLightVertical;
            return msoPatternLightVertical.get().unwrap().clone();
        }
        // Java: else if ("dkHorz".equals(prst))
        else if prst == "dkHorz" {
            // Java: return msoPatternDarkHorizontal;
            return msoPatternDarkHorizontal.get().unwrap().clone();
        }
        // Java: else if ("narHorz".equals(prst))
        else if prst == "narHorz" {
            // Java: return msoPatternNarrowHorizontal;
            return msoPatternNarrowHorizontal.get().unwrap().clone();
        }
        // Java: else if ("narVert".equals(prst))
        else if prst == "narVert" {
            // Java: return msoPatternNarrowVertical;
            return msoPatternNarrowVertical.get().unwrap().clone();
        }
        // Java: else if ("dashHorz".equals(prst))
        else if prst == "dashHorz" {
            // Java: return msoPatternDashedHorizontal;
            return msoPatternDashedHorizontal.get().unwrap().clone();
        }
        // Java: else if ("dashVert".equals(prst))
        else if prst == "dashVert" {
            // Java: return msoPatternDashedVertical;
            return msoPatternDashedVertical.get().unwrap().clone();
        }
        // Java: else if ("dkVert".equals(prst))
        else if prst == "dkVert" {
            // Java: return msoPatternDarkVertical;
            return msoPatternDarkVertical.get().unwrap().clone();
        }
        // Java: else if ("ltDnDiag".equals(prst))
        else if prst == "ltDnDiag" {
            // Java: return msoPatternLightDownwardDiagonal;
            return msoPatternLightDownwardDiagonal.get().unwrap().clone();
        }
        // Java: else if ("ltUpDiag".equals(prst))
        else if prst == "ltUpDiag" {
            // Java: return msoPatternLightUpwardDiagonal;
            return msoPatternLightUpwardDiagonal.get().unwrap().clone();
        }
        // Java: else if ("dkDnDiag".equals(prst))
        else if prst == "dkDnDiag" {
            // Java: return msoPatternDarkDownwardDiagonal;
            return msoPatternDarkDownwardDiagonal.get().unwrap().clone();
        }
        // Java: else if ("wdUpDiag".equals(prst))
        else if prst == "wdUpDiag" {
            // Java: return msoPatternWdUpDiag;
            return msoPatternWdUpDiag.get().unwrap().clone();
        }
        // Java: else if ("dkUpDiag".equals(prst))
        else if prst == "dkUpDiag" {
            // Java: return msoPatternDarkUpwardDiagonal;
            return msoPatternDarkUpwardDiagonal.get().unwrap().clone();
        }
        // Java: else if ("wdDnDiag".equals(prst))
        else if prst == "wdDnDiag" {
            // Java: return msoPatternWdDnDiag;
            return msoPatternWdDnDiag.get().unwrap().clone();
        }
        // Java: else if ("dashDnDiag".equals(prst))
        else if prst == "dashDnDiag" {
            // Java: return msoPatternDashedDownwardDiagonal;
            return msoPatternDashedDownwardDiagonal.get().unwrap().clone();
        }
        // Java: else if ("dashUpDiag".equals(prst))
        else if prst == "dashUpDiag" {
            // Java: return msoPatternDashedUpwardDiagonal;
            return msoPatternDashedUpwardDiagonal.get().unwrap().clone();
        }
        // Java: else if ("smCheck".equals(prst))
        else if prst == "smCheck" {
            // Java: return msoPatternSmallCheckerBoard;
            return msoPatternSmallCheckerBoard.get().unwrap().clone();
        }
        // Java: else if ("lgCheck".equals(prst))
        else if prst == "lgCheck" {
            // Java: return msoPatternLargeCheckerBoard;
            return msoPatternLargeCheckerBoard.get().unwrap().clone();
        }
        // Java: else if ("smGrid".equals(prst))
        else if prst == "smGrid" {
            // Java: return msoPatternSmallGrid;
            return msoPatternSmallGrid.get().unwrap().clone();
        }
        // Java: else if ("lgGrid".equals(prst))
        else if prst == "lgGrid" {
            // Java: return msoPatternLargeGrid;
            return msoPatternLargeGrid.get().unwrap().clone();
        }
        // Java: else if ("dotGrid".equals(prst))
        else if prst == "dotGrid" {
            // Java: return msoPatternDottedGrid;
            return msoPatternDottedGrid.get().unwrap().clone();
        }
        // Java: else if ("smConfetti".equals(prst))
        else if prst == "smConfetti" {
            // Java: return msoPatternSmallConfetti;
            return msoPatternSmallConfetti.get().unwrap().clone();
        }
        // Java: else if ("lgConfetti".equals(prst))
        else if prst == "lgConfetti" {
            // Java: return msoPatternLargeConfetti;
            return msoPatternLargeConfetti.get().unwrap().clone();
        }
        // Java: else if ("horzBrick".equals(prst))
        else if prst == "horzBrick" {
            // Java: return msoPatternHorizontalBrick;
            return msoPatternHorizontalBrick.get().unwrap().clone();
        }
        // Java: else if ("diagBrick".equals(prst))
        else if prst == "diagBrick" {
            // Java: return msoPatternDiagonalBrick;
            return msoPatternDiagonalBrick.get().unwrap().clone();
        }
        // Java: else if ("solidDmnd".equals(prst))
        else if prst == "solidDmnd" {
            // Java: return msoPatternSolidDiamond;
            return msoPatternSolidDiamond.get().unwrap().clone();
        }
        // Java: else if ("openDmnd".equals(prst))
        else if prst == "openDmnd" {
            // Java: return msoPatternOpenDiamond;
            return msoPatternOpenDiamond.get().unwrap().clone();
        }
        // Java: else if ("dotDmnd".equals(prst))
        else if prst == "dotDmnd" {
            // Java: return msoPatternDottedDiamond;
            return msoPatternDottedDiamond.get().unwrap().clone();
        }
        // Java: else if ("plaid".equals(prst))
        else if prst == "plaid" {
            // Java: return msoPatternPlaid;
            return msoPatternPlaid.get().unwrap().clone();
        }
        // Java: else if ("sphere".equals(prst))
        else if prst == "sphere" {
            // Java: return msoPatternSphere;
            return msoPatternSphere.get().unwrap().clone();
        }
        // Java: else if ("weave".equals(prst))
        else if prst == "weave" {
            // Java: return msoPatternWeave;
            return msoPatternWeave.get().unwrap().clone();
        }
        // Java: else if ("divot".equals(prst))
        else if prst == "divot" {
            // Java: return msoPatternDivot;
            return msoPatternDivot.get().unwrap().clone();
        }
        // Java: else if ("shingle".equals(prst))
        else if prst == "shingle" {
            // Java: return msoPatternShingle;
            return msoPatternShingle.get().unwrap().clone();
        }
        // Java: else if ("wave".equals(prst))
        else if prst == "wave" {
            // Java: return msoPatternWave;
            return msoPatternWave.get().unwrap().clone();
        }
        // Java: else if ("trellis".equals(prst))
        else if prst == "trellis" {
            // Java: return msoPatternTrellis;
            return msoPatternTrellis.get().unwrap().clone();
        }
        // Java: else
        else {
            // Java: return "zigZag".equals(prst) ? msoPatternZigZag : result;
            if prst == "zigZag" { return msoPatternZigZag.get().unwrap().clone(); }
        }
        // Java: return result;
        result.clone()
    }

    // Java: static byte[] resetPatternData(Object fillBlips, int fgColor, int bgColor)
    pub fn resetPatternData(fillBlips: Option<&[u8]>, fgColor: i32, bgColor: i32) -> Option<Vec<u8>> {
        // Java: if (fillBlips == null)
        if fillBlips.is_none() {
            // Java: return null;
            return None;
        // Java: else
        } else {
            // Java: Bitmap bitmap = null;
            let mut bitmap: Option<Bitmap> = None;
            // Java: byte[] fillBlip = (byte[])fillBlips;
            let mut fillBlip: Vec<u8> = fillBlips.unwrap().to_vec();
            // Java: bitmap = BitmapFactory.decodeByteArray(fillBlip, 0, fillBlip.length);
            bitmap = Bitmap::decodeByteArray(&fillBlip, 0, fillBlip.len());
            // Java: byte[] pictureData = null;
            let mut pictureData: Option<Vec<u8>> = None;
            // Java: byte[] pictureDataETBMP = null;
            let mut pictureDataETBMP: Option<Vec<u8>> = None;
            // Java: byte[] bmpHead = null;
            let mut bmpHead: Option<Vec<u8>> = None;
            // Java: int pictureSize = 0;
            let mut pictureSize: i32 = 0;
            // Java: if (fillBlip.length > 25)
            if fillBlip.len() > 25 {
                // Java: pictureData = new byte[fillBlip.length - 25];
                let mut pictureDataArr = vec![0u8; fillBlip.len() - 25];
                // Java: ByteBuffer byteBuffer = null;
                let mut byteBuffer: Option<ByteBuffer> = None;
                // Java: pictureSize = fillBlip.length - 9;
                pictureSize = fillBlip.len() as i32 - 9;
                // Java: byteBuffer = ByteBuffer.wrap(fillBlip, 25, fillBlip.length - 25);
                byteBuffer = Some(ByteBuffer::wrap(&fillBlip, 25, fillBlip.len() - 25));
                // Java: byteBuffer.get(pictureData, 0, pictureData.length);
                { let len = pictureDataArr.len(); byteBuffer.as_ref().unwrap().get(&mut pictureDataArr, 0, len); }
                // Java: bmpHead = new byte[]{66, 77, (byte)(pictureSize >> 24 & 255), (byte)(pictureSize >> 16 & 255), (byte)(pictureSize >> 8 & 255), (byte)(pictureSize & 255), 0, 0, 0, 0, 54, 0, 0, 0};
                let bmpHeadArr: Vec<u8> = vec![66, 77, ((pictureSize >> 24) & 255) as u8, ((pictureSize >> 16) & 255) as u8, ((pictureSize >> 8) & 255) as u8, (pictureSize & 255) as u8, 0, 0, 0, 0, 54, 0, 0, 0];
                // Java: fillBlip[65] = (byte)(fgColor & 255);
                fillBlip[65] = (fgColor & 255) as u8;
                // Java: fillBlip[66] = (byte)((fgColor & '\uff00') >> 8);
                fillBlip[66] = ((fgColor & 0xff00) >> 8) as u8;
                // Java: fillBlip[67] = (byte)((fgColor & 16711680) >> 16);
                fillBlip[67] = ((fgColor & 16711680) >> 16) as u8;
                // Java: fillBlip[69] = (byte)(bgColor & 255);
                fillBlip[69] = (bgColor & 255) as u8;
                // Java: fillBlip[70] = (byte)((bgColor & '\uff00') >> 8);
                fillBlip[70] = ((bgColor & 0xff00) >> 8) as u8;
                // Java: fillBlip[71] = (byte)((bgColor & 16711680) >> 16);
                fillBlip[71] = ((bgColor & 16711680) >> 16) as u8;
                // Java: pictureDataETBMP = new byte[fillBlip.length - 11];
                let mut pictureDataETBMPArr = vec![0u8; fillBlip.len() - 11];
                // Java: byteBuffer = ByteBuffer.wrap(fillBlip, 25, fillBlip.length - 25);
                byteBuffer = Some(ByteBuffer::wrap(&fillBlip, 25, fillBlip.len() - 25));
                // Java: byteBuffer.get(pictureData, 0, pictureData.length);
                { let len = pictureDataArr.len(); byteBuffer.as_ref().unwrap().get(&mut pictureDataArr, 0, len); }
                // Java: System.arraycopy(bmpHead, 0, pictureDataETBMP, 0, bmpHead.length);
                System::arraycopy(&bmpHeadArr, 0, &mut pictureDataETBMPArr, 0, bmpHeadArr.len());
                // Java: System.arraycopy(pictureData, 0, pictureDataETBMP, bmpHead.length, pictureData.length);
                System::arraycopy(&pictureDataArr, 0, &mut pictureDataETBMPArr, bmpHeadArr.len(), pictureDataArr.len());
                // Java: bitmap = BitmapFactory.decodeByteArray(pictureDataETBMP, 0, pictureDataETBMP.length);
                bitmap = Bitmap::decodeByteArray(&pictureDataETBMPArr, 0, pictureDataETBMPArr.len());
                // Java: return null != bitmap ? pictureDataETBMP : null;
                if bitmap.is_some() { Some(pictureDataETBMPArr) } else { None }
            // Java: else
            } else {
                // Java: return null;
                None
            }
        }
    }

    // Java: static Bitmap decodeBitmap(byte[] fillBlips, int fgColor, int bgColor)
    pub fn decodeBitmap(fillBlips: &[u8], fgColor: i32, bgColor: i32) -> Option<Bitmap> {
        // Java: byte[] fillBlip = replaceFgBgColor(fillBlips, fgColor, bgColor);
        let fillBlip = PatternFillUtil::replaceFgBgColor(fillBlips, fgColor, bgColor);
        // Java: Bitmap bitmap = BitmapFactory.decodeByteArray(fillBlip, 0, fillBlip.length);
        let bitmap = Bitmap::decodeByteArray(&fillBlip, 0, fillBlip.len());
        // Java: return bitmap;
        bitmap
    }

    // Java: static byte[] replaceFgBgColor(byte[] fillBlips, int fgColor, int bgColor)
    pub fn replaceFgBgColor(fillBlips: &[u8], fgColor: i32, bgColor: i32) -> Vec<u8> {
        // Java: byte[] fillBlip = new byte[fillBlips.length];
        let mut fillBlip = vec![0u8; fillBlips.len()];
        // Java: System.arraycopy(fillBlips, 0, fillBlip, 0, fillBlips.length);
        System::arraycopy(fillBlips, 0, &mut fillBlip, 0, fillBlips.len());
        // Java: fillBlip[54] = (byte)(fgColor & 255);
        fillBlip[54] = (fgColor & 255) as u8;
        // Java: fillBlip[55] = (byte)((fgColor & '\uff00') >> 8);
        fillBlip[55] = ((fgColor & 0xff00) >> 8) as u8;
        // Java: fillBlip[56] = (byte)((fgColor & 16711680) >> 16);
        fillBlip[56] = ((fgColor & 16711680) >> 16) as u8;
        // Java: fillBlip[58] = (byte)(bgColor & 255);
        fillBlip[58] = (bgColor & 255) as u8;
        // Java: fillBlip[59] = (byte)((bgColor & '\uff00') >> 8);
        fillBlip[59] = ((bgColor & 0xff00) >> 8) as u8;
        // Java: fillBlip[60] = (byte)((bgColor & 16711680) >> 16);
        fillBlip[60] = ((bgColor & 16711680) >> 16) as u8;
        // Java: return fillBlip;
        fillBlip
    }
}

// Java: public enum PatternFillType
impl PatternFillType {
    // Java: PatternFillType()
    fn new() -> Self {
        Self::Cross
    }
}
