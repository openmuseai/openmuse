// Java field: int DENSITY_HIGH
pub const DENSITY_HIGH: i32 = 240;
// Java field: int DENSITY_LOW
pub const DENSITY_LOW: i32 = 120;
// Java field: int DENSITY_MEDIUM
pub const DENSITY_MEDIUM: i32 = 160;
// Java field: int DENSITY_XHIGH
pub const DENSITY_XHIGH: i32 = 320;
// Java field: int DENSITY_DEFAULT
pub const DENSITY_DEFAULT: i32 = 160;

// Java: public class KDisplayMetrics
pub struct KDisplayMetrics {
    // Java field: float density
    pub density: f32,
    // Java field: int densityDpi
    pub densityDpi: i32,
    // Java field: int heightPixels
    pub heightPixels: i32,
    // Java field: float scaledDensity
    pub scaledDensity: f32,
    // Java field: int widthPixels
    pub widthPixels: i32,
    // Java field: float xdpi
    pub xdpi: f32,
    // Java field: float ydpi
    pub ydpi: f32,
}

// Java: public class KDisplayMetrics
impl KDisplayMetrics {
    // Java: KDisplayMetrics()
    pub fn new() -> Self {
        // Java: super();
        KDisplayMetrics {
            density: 0.0,
            densityDpi: 0,
            heightPixels: 0,
            scaledDensity: 0.0,
            widthPixels: 0,
            xdpi: 0.0,
            ydpi: 0.0,
        }
    }

    // Java: KDisplayMetrics(int widthPixels, int heightPixels, float density, int densityDpi, float scaledDensity, float xdpi, float ydpi)
    pub fn new_with_params(widthPixels: i32, heightPixels: i32, density: f32, densityDpi: i32, scaledDensity: f32, xdpi: f32, ydpi: f32) -> Self {
        // Java: super();
        let mut s = KDisplayMetrics::new();
        s._init(widthPixels, heightPixels, density, densityDpi, scaledDensity, xdpi, ydpi);
        s
    }

    fn _init(&mut self, widthPixels: i32, heightPixels: i32, density: f32, densityDpi: i32, scaledDensity: f32, xdpi: f32, ydpi: f32) {
        // Java: this.widthPixels = widthPixels;
        self.widthPixels = widthPixels;
        // Java: this.heightPixels = heightPixels;
        self.heightPixels = heightPixels;
        // Java: this.density = density;
        self.density = density;
        // Java: this.densityDpi = densityDpi;
        self.densityDpi = densityDpi;
        // Java: this.scaledDensity = scaledDensity;
        self.scaledDensity = scaledDensity;
        // Java: this.xdpi = xdpi;
        self.xdpi = xdpi;
        // Java: this.ydpi = ydpi;
        self.ydpi = ydpi;
    }

    // Java: void setTo(KDisplayMetrics o)
    pub fn setTo(&mut self, o: &KDisplayMetrics) {
        // Java: this.widthPixels = o.widthPixels;
        self.widthPixels = o.widthPixels;
        // Java: this.heightPixels = o.heightPixels;
        self.heightPixels = o.heightPixels;
        // Java: this.density = o.density;
        self.density = o.density;
        // Java: this.densityDpi = o.densityDpi;
        self.densityDpi = o.densityDpi;
        // Java: this.scaledDensity = o.scaledDensity;
        self.scaledDensity = o.scaledDensity;
        // Java: this.xdpi = o.xdpi;
        self.xdpi = o.xdpi;
        // Java: this.ydpi = o.ydpi;
        self.ydpi = o.ydpi;
    }
}
