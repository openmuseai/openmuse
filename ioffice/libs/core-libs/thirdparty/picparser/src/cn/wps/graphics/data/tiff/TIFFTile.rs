#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_assignments, unused_mut)]

use std::sync::Mutex;

// Java field: int inSample
static inSample: Mutex<i32> = Mutex::new(1);
// Java field: boolean[] boolSamples
static boolSamples: Mutex<Option<Vec<bool>>> = Mutex::new(None);
// Java field: byte[] byteSamples
static byteSamples: Mutex<Option<Vec<u8>>> = Mutex::new(None);
// Java field: int[] intSamples
static intSamples: Mutex<Option<Vec<i32>>> = Mutex::new(None);
// Java field: int[] pixels
static pixels: Mutex<Option<Vec<i32>>> = Mutex::new(None);

// Java: public class TIFFTile
pub struct TIFFTile {
    // Java field: int offsetX
    pub(crate) offsetX: i32,
    // Java field: int offsetY
    pub(crate) offsetY: i32,
    // Java field: int width
    pub(crate) width: i32,
    // Java field: int height
    pub(crate) height: i32,
    // Java field: int stride
    pub(crate) stride: i32,
    // Java field: int sampledWidth
    pub(crate) sampledWidth: i32,
    // Java field: int sampledHeight
    pub(crate) sampledHeight: i32,
    // Java field: int mNoUseBits
    pub(crate) mNoUseBits: i32,
}

// Java: public class TIFFTile
impl TIFFTile {
    // Java: TIFFTile(int x, int y, int width, int height, int noUseBits)
    pub fn new(x: i32, y: i32, width: i32, height: i32, noUseBits: i32) -> Self {
        // Java: super();
        let in_sample = *inSample.lock().unwrap();
        // Java: this.width = width;
        // Java: this.height = height;
        // Java: this.offsetX = x * width % inSample;
        let offsetX = x * width % in_sample;
        // Java: this.offsetY = y * height % inSample;
        let offsetY = y * height % in_sample;
        // Java: this.sampledWidth = (width - this.offsetX + inSample - 1) / inSample;
        let sampledWidth = (width - offsetX + in_sample - 1) / in_sample;
        // Java: this.sampledHeight = (height - this.offsetY + inSample - 1) / inSample;
        let sampledHeight = (height - offsetY + in_sample - 1) / in_sample;
        // Java: this.mNoUseBits = noUseBits == 8 ? 0 : noUseBits;
        let mNoUseBits = if noUseBits == 8 { 0 } else { noUseBits };
        // Java: if (pixels == null)
        {
            let mut p = pixels.lock().unwrap();
            if p.is_none() {
                // Java: pixels = new int[this.sampledWidth * this.sampledHeight];
                *p = Some(vec![0i32; (sampledWidth * sampledHeight) as usize]);
            }
        }
        TIFFTile {
            offsetX,
            offsetY,
            width,
            height,
            stride: 0,
            sampledWidth,
            sampledHeight,
            mNoUseBits,
        }
    }

    // Java: static void init(int inSample)
    pub fn init(in_sample_val: i32) {
        // Java: TIFFTile.inSample = inSample;
        *inSample.lock().unwrap() = in_sample_val;
        // Java: boolSamples = null;
        *boolSamples.lock().unwrap() = None;
        // Java: byteSamples = null;
        *byteSamples.lock().unwrap() = None;
        // Java: intSamples = null;
        *intSamples.lock().unwrap() = None;
        // Java: pixels = null;
        *pixels.lock().unwrap() = None;
    }

    // Java: int getNoUseBits()
    pub fn getNoUseBits(&self) -> i32 {
        // Java: return this.mNoUseBits;
        self.mNoUseBits
    }
}
