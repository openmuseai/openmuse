// Java field: float RATIO_EMUS_PER_INCH
pub(crate) const RATIO_EMUS_PER_INCH: f32 = 914400.0;
// Java field: float RATIO_POINTS_PER_INCH
pub(crate) const RATIO_POINTS_PER_INCH: f32 = 72.0;
// Java field: float RATIO_TWIPS_PER_POINT
pub(crate) const RATIO_TWIPS_PER_POINT: f32 = 20.0;
// Java field: float RATIO_HALFPOINTS_PER_POINT
pub(crate) const RATIO_HALFPOINTS_PER_POINT: f32 = 2.0;
// Java field: float RATIO_MM_PER_CM
pub(crate) const RATIO_MM_PER_CM: f32 = 10.0;
// Java field: float RATIO_CM_PER_INCH
pub(crate) const RATIO_CM_PER_INCH: f32 = 2.54;
// Java field: float RATIO_PICA_PER_CM
pub(crate) const RATIO_PICA_PER_CM: f32 = 2.3622048;
// Java field: float RATIO_EIGHTH_POINTS_PER_POINT
pub(crate) const RATIO_EIGHTH_POINTS_PER_POINT: f32 = 8.0;
// Java field: float RATIO_ST_ANGLE_DEGREES_PER_DEGREE
pub(crate) const RATIO_ST_ANGLE_DEGREES_PER_DEGREE: f32 = 60000.0;
// Java field: float RATIO_FRACTION_PER_FLOAT
pub(crate) const RATIO_FRACTION_PER_FLOAT: f32 = 65536.0;
// Java field: float RATIO_PERCENT_PER_FLOAT
pub(crate) const RATIO_PERCENT_PER_FLOAT: f32 = 100.0;
// Java field: long NANOSECOND_PER_SECOND
pub(crate) const NANOSECOND_PER_SECOND: i64 = 1000000000;
// Java field: long HUNDREDS_OF_NANOSECONDS_PER_MILLLISECCOND
pub(crate) const HUNDREDS_OF_NANOSECONDS_PER_MILLLISECCOND: i64 = 10000;
// Java field: long MILLISECOND_PER_SECOND
pub(crate) const MILLISECOND_PER_SECOND: i64 = 1000;
// Java field: float SECONDS_PER_MINUTE
pub(crate) const SECONDS_PER_MINUTE: f32 = 60.0;
// Java field: float RATIO_ST_PERCENTAGE_DECIMAL_PER_FLOAT
pub(crate) const RATIO_ST_PERCENTAGE_DECIMAL_PER_FLOAT: f32 = 100000.0;
// Java field: float QUARTER_PER_POINT
pub(crate) const QUARTER_PER_POINT: f32 = 4.0;
// Java field: float PITCH_PER_INCH
pub(crate) const PITCH_PER_INCH: f32 = 294912.0;
// Java field: double RAD_PER_RAD
pub(crate) const RAD_PER_RAD: f64 = std::f64::consts::PI / 180.0;

// Java: public class MetricUtility
pub struct MetricUtility;

// Java: public class MetricUtility
impl MetricUtility {
    // Java: MetricUtility()
    pub fn new() -> Self {
        // Java: super();
        MetricUtility
    }

    // Java: static double degree2Rad(double value)
    pub fn degree2Rad(value: f64) -> f64 {
        // Java: return value * (Math.PI / 180D);
        value * (std::f64::consts::PI / 180.0)
    }

    // Java: static double rad2Degree(double value)
    pub fn rad2Degree(value: f64) -> f64 {
        // Java: return value / (Math.PI / 180D);
        value / (std::f64::consts::PI / 180.0)
    }

    // Java: static float STPercentageDecimal2Float(long value)
    pub fn STPercentageDecimal2Float(value: i64) -> f32 {
        // Java: return (float)value / 100000.0F;
        (value as f32) / 100000.0
    }

    // Java: static int Float2STPercentageDecimal(float value)
    pub fn Float2STPercentageDecimal(value: f32) -> i32 {
        // Java: return (int)Math.ceil((double)(value * 100000.0F));
        ((value * 100000.0) as f64).ceil() as i32
    }

    // Java: static long Minute2MillSecond(float value)
    pub fn Minute2MillSecond(value: f32) -> i64 {
        // Java: return Second2MilliSecond((long)(value * 60.0F));
        Self::Second2MilliSecond((value * 60.0) as i64)
    }

    // Java: static long Second2MilliSecond(long value)
    pub fn Second2MilliSecond(value: i64) -> i64 {
        // Java: return value * 1000L;
        value * 1000
    }

    // Java: static long Minute2NanoSecond(float value)
    pub fn Minute2NanoSecond(value: f32) -> i64 {
        // Java: return Second2NanoSecond((long)(value * 60.0F));
        Self::Second2NanoSecond((value * 60.0) as i64)
    }

    // Java: static long Second2NanoSecond(long value)
    pub fn Second2NanoSecond(value: i64) -> i64 {
        // Java: return 1000000000L * value;
        1000000000 * value
    }

    // Java: static long MilliSecond2Second(long value)
    pub fn MilliSecond2Second(value: i64) -> i64 {
        // Java: return value / 1000L;
        value / 1000
    }

    // Java: static float MilliSecond2Minute(long value)
    pub fn MilliSecond2Minute(value: i64) -> f32 {
        // Java: return Second2Minute(MilliSecond2Second(value));
        Self::Second2Minute(Self::MilliSecond2Second(value))
    }

    // Java: static long NanoSecond2Second(long value)
    pub fn NanoSecond2Second(value: i64) -> i64 {
        // Java: return value / 1000000000L;
        value / 1000000000
    }

    // Java: static float NanoSecond2Minute(long value)
    pub fn NanoSecond2Minute(value: i64) -> f32 {
        // Java: return Second2Minute(NanoSecond2Second(value));
        Self::Second2Minute(Self::NanoSecond2Second(value))
    }

    // Java: static float Second2Minute(long value)
    pub fn Second2Minute(value: i64) -> f32 {
        // Java: return (float)value / 60.0F;
        (value as f32) / 60.0
    }

    // Java: static float Fraction2Float(float value)
    pub fn Fraction2Float(value: f32) -> f32 {
        // Java: return value / 65536.0F;
        value / 65536.0
    }

    // Java: static long HundredsOfNanoseconds2MilliSeccond(long value)
    pub fn HundredsOfNanoseconds2MilliSeccond(value: i64) -> i64 {
        // Java: return value / 10000L;
        value / 10000
    }

    // Java: static float HundredsOfNanoseconds2Minute(long value)
    pub fn HundredsOfNanoseconds2Minute(value: i64) -> f32 {
        // Java: return MilliSecond2Minute(HundredsOfNanoseconds2MilliSeccond(value));
        Self::MilliSecond2Minute(Self::HundredsOfNanoseconds2MilliSeccond(value))
    }

    // Java: static long Minute2HundredsOfNanoSeconds(float value)
    pub fn Minute2HundredsOfNanoSeconds(value: f32) -> i64 {
        // Java: return MilliSecond2HundredsOfNanoseconds(Minute2MillSecond(value));
        Self::MilliSecond2HundredsOfNanoseconds(Self::Minute2MillSecond(value))
    }

    // Java: static long MilliSecond2HundredsOfNanoseconds(long value)
    pub fn MilliSecond2HundredsOfNanoseconds(value: i64) -> i64 {
        // Java: return value * 10000L;
        value * 10000
    }

    // Java: static float Float2Fraction(float value)
    pub fn Float2Fraction(value: f32) -> f32 {
        // Java: return value * 65536.0F;
        value * 65536.0
    }

    // Java: static float Percent2Float(float value)
    pub fn Percent2Float(value: f32) -> f32 {
        // Java: return value / 100.0F;
        value / 100.0
    }

    // Java: static float Float2Percent(float value)
    pub fn Float2Percent(value: f32) -> f32 {
        // Java: return value * 100.0F;
        value * 100.0
    }

    // Java: static float EMU2Points(float value)
    pub fn EMU2Points(value: f32) -> f32 {
        // Java: return EMU2Inches(value) * 72.0F;
        Self::EMU2Inches(value) * 72.0
    }

    // Java: static float Points2EMU(float value)
    pub fn Points2EMU(value: f32) -> f32 {
        // Java: return Inches2EMU(Points2Inches(value));
        Self::Inches2EMU(Self::Points2Inches(value))
    }

    // Java: static float EMU2Inches(float value)
    pub fn EMU2Inches(value: f32) -> f32 {
        // Java: return value / 914400.0F;
        value / 914400.0
    }

    // Java: static float Inches2EMU(float value)
    pub fn Inches2EMU(value: f32) -> f32 {
        // Java: return value * 914400.0F;
        value * 914400.0
    }

    // Java: static float pixels2Twips(float pixels, int densityDpi)
    pub fn pixels2Twips(pixels: f32, density_dpi: i32) -> f32 {
        // Java: return Inches2Twips(pixels2Inches(pixels, densityDpi));
        Self::Inches2Twips(Self::pixels2Inches_f32(pixels, density_dpi))
    }

    // Java: static float pixels2Inches(int pixels, int densityDpi)
    pub fn pixels2Inches_i32(pixels: i32, density_dpi: i32) -> f32 {
        // Java: return pixels2Inches((float)pixels, densityDpi);
        Self::pixels2Inches_f32(pixels as f32, density_dpi)
    }

    // Java: static float pixels2Inches(float pixels, int densityDpi)
    pub fn pixels2Inches_f32(pixels: f32, density_dpi: i32) -> f32 {
        // Java: return pixels / (float)densityDpi;
        pixels / (density_dpi as f32)
    }

    // Java: static float inches2Pixels(float inches, int densityDpi)
    pub fn inches2Pixels(inches: f32, density_dpi: i32) -> f32 {
        // Java: return inches * (float)densityDpi;
        inches * (density_dpi as f32)
    }

    // Java: static float Inches2Points(float value)
    pub fn Inches2Points(value: f32) -> f32 {
        // Java: return value * 72.0F;
        value * 72.0
    }

    // Java: static float Inches2Twips(float value)
    pub fn Inches2Twips(value: f32) -> f32 {
        // Java: return Points2Twips(Inches2Points(value));
        Self::Points2Twips_f32(Self::Inches2Points(value))
    }

    // Java: static float Points2Inches(float value)
    pub fn Points2Inches(value: f32) -> f32 {
        // Java: return value / 72.0F;
        value / 72.0
    }

    // Java: static float ST_ANGLE_Degrees2Degree(int value)
    pub fn ST_ANGLE_Degrees2Degree(value: i32) -> f32 {
        // Java: return (float)value / 60000.0F;
        (value as f32) / 60000.0
    }

    // Java: static int Degree2ST_ANGLE_Degrees(float value)
    pub fn Degree2ST_ANGLE_Degrees_f32(value: f32) -> i32 {
        // Java: return (int)(value * 60000.0F);
        (value * 60000.0) as i32
    }

    // Java: static double Degree2ST_ANGLE_Degrees(double value)
    pub fn Degree2ST_ANGLE_Degrees_f64(value: f64) -> f64 {
        // Java: return value * (double)60000.0F;
        value * 60000.0
    }

    // Java: static float rad2ST_ANGLE_Degrees(float value)
    pub fn rad2ST_ANGLE_Degrees(value: f32) -> f32 {
        // Java: return (float)Degree2ST_ANGLE_Degrees(rad2Degree((double)value));
        Self::Degree2ST_ANGLE_Degrees_f64(Self::rad2Degree(value as f64)) as f32
    }

    // Java: static int rad2ST_FIXED_ANGLE_Degrees(float value)
    pub fn rad2ST_FIXED_ANGLE_Degrees(value: f32) -> i32 {
        // Java: double degree = rad2Degree((double)value);
        let mut degree = Self::rad2Degree(value as f64);
        // Java: degree %= (double)360.0F;
        degree %= 360.0;
        // Java: degree += (double)360.0F;
        degree += 360.0;
        // Java: degree %= (double)360.0F;
        degree %= 360.0;
        // Java: return Degree2ST_ANGLE_Degrees((float)degree);
        Self::Degree2ST_ANGLE_Degrees_f32(degree as f32)
    }

    // Java: static float ST_ANGLE_Degrees2Rad(int value)
    pub fn ST_ANGLE_Degrees2Rad(value: i32) -> f32 {
        // Java: return (float)degree2Rad((double)ST_ANGLE_Degrees2Degree(value));
        Self::degree2Rad(Self::ST_ANGLE_Degrees2Degree(value) as f64) as f32
    }

    // Java: static int Twips2Points(int value)
    pub fn Twips2Points_i32(value: i32) -> i32 {
        // Java: return value / 20;
        value / 20
    }

    // Java: static float Twips2Points(float value)
    pub fn Twips2Points_f32(value: f32) -> f32 {
        // Java: return value / 20.0F;
        value / 20.0
    }

    // Java: static double Twips2Points(double value)
    pub fn Twips2Points_f64(value: f64) -> f64 {
        // Java: return value / (double)20.0F;
        value / 20.0
    }

    // Java: static int Twips2Pixels(int value, int densityDpi)
    pub fn Twips2Pixels(value: i32, density_dpi: i32) -> i32 {
        // Java: return (int)(Points2Inches((float)Twips2Points(value)) * (float)densityDpi);
        (Self::Points2Inches(Self::Twips2Points_i32(value) as f32) * (density_dpi as f32)) as i32
    }

    // Java: static float Twips2EMU(float value)
    pub fn Twips2EMU(value: f32) -> f32 {
        // Java: return Points2EMU(Twips2Points(value));
        Self::Points2EMU(Self::Twips2Points_f32(value))
    }

    // Java: static float EMU2Twips(float value)
    pub fn EMU2Twips(value: f32) -> f32 {
        // Java: return Points2Twips(EMU2Points(value));
        Self::Points2Twips_f32(Self::EMU2Points(value))
    }

    // Java: static float Points2Twips(float value)
    pub fn Points2Twips_f32(value: f32) -> f32 {
        // Java: return value * 20.0F;
        value * 20.0
    }

    // Java: static double Points2Twips(double value)
    pub fn Points2Twips_f64(value: f64) -> f64 {
        // Java: return value * (double)20.0F;
        value * 20.0
    }

    // Java: static float HalfPoints2Points(int value)
    pub fn HalfPoints2Points(value: i32) -> f32 {
        // Java: return (float)value / 2.0F;
        (value as f32) / 2.0
    }

    // Java: static int Points2HalfPoints(float value)
    pub fn Points2HalfPoints(value: f32) -> i32 {
        // Java: return (int)(value * 2.0F);
        (value * 2.0) as i32
    }

    // Java: static float MM2CM(float value)
    pub fn MM2CM(value: f32) -> f32 {
        // Java: return value / 10.0F;
        value / 10.0
    }

    // Java: static float CM2MM(float value)
    pub fn CM2MM(value: f32) -> f32 {
        // Java: return value * 10.0F;
        value * 10.0
    }

    // Java: static float CM2Inches(float value)
    pub fn CM2Inches(value: f32) -> f32 {
        // Java: return value / 2.54F;
        value / 2.54
    }

    // Java: static float Inches2CM(float value)
    pub fn Inches2CM(value: f32) -> f32 {
        // Java: return value * 2.54F;
        value * 2.54
    }

    // Java: static float MM2Inches(float value)
    pub fn MM2Inches(value: f32) -> f32 {
        // Java: return CM2Inches(MM2CM(value));
        Self::CM2Inches(Self::MM2CM(value))
    }

    // Java: static float Inches2MM(float value)
    pub fn Inches2MM(value: f32) -> f32 {
        // Java: return CM2MM(Inches2CM(value));
        Self::CM2MM(Self::Inches2CM(value))
    }

    // Java: static float MM2Points(float value)
    pub fn MM2Points(value: f32) -> f32 {
        // Java: return Inches2Points(MM2Inches(value));
        Self::Inches2Points(Self::MM2Inches(value))
    }

    // Java: static float MM2Twips(float value)
    pub fn MM2Twips(value: f32) -> f32 {
        // Java: return Points2Twips(MM2Points(value));
        Self::Points2Twips_f32(Self::MM2Points(value))
    }

    // Java: static float Points2MM(float value)
    pub fn Points2MM(value: f32) -> f32 {
        // Java: return Inches2MM(Points2Inches(value));
        Self::Inches2MM(Self::Points2Inches(value))
    }

    // Java: static float CM2Points(float value)
    pub fn CM2Points(value: f32) -> f32 {
        // Java: return Inches2Points(CM2Inches(value));
        Self::Inches2Points(Self::CM2Inches(value))
    }

    // Java: static float CM2Twips(float value)
    pub fn CM2Twips(value: f32) -> f32 {
        // Java: return Points2Twips(CM2Points(value));
        Self::Points2Twips_f32(Self::CM2Points(value))
    }

    // Java: static float Points2CM(float value)
    pub fn Points2CM(value: f32) -> f32 {
        // Java: return Inches2CM(Points2Inches(value));
        Self::Inches2CM(Self::Points2Inches(value))
    }

    // Java: static float Pica2CM(float value)
    pub fn Pica2CM(value: f32) -> f32 {
        // Java: return value * 2.3622048F;
        value * 2.3622048
    }

    // Java: static float CM2Pica(float value)
    pub fn CM2Pica(value: f32) -> f32 {
        // Java: return value / 2.3622048F;
        value / 2.3622048
    }

    // Java: static float Pica2MM(float value)
    pub fn Pica2MM(value: f32) -> f32 {
        // Java: return CM2MM(Pica2CM(value));
        Self::CM2MM(Self::Pica2CM(value))
    }

    // Java: static float MM2Pica(float value)
    pub fn MM2Pica(value: f32) -> f32 {
        // Java: return CM2Pica(MM2CM(value));
        Self::CM2Pica(Self::MM2CM(value))
    }

    // Java: static float Pica2Points(float value)
    pub fn Pica2Points(value: f32) -> f32 {
        // Java: return CM2Points(Pica2CM(value));
        Self::CM2Points(Self::Pica2CM(value))
    }

    // Java: static float PC2Twips(float value)
    pub fn PC2Twips(value: f32) -> f32 {
        // Java: return CM2Twips(Pica2CM(value));
        Self::CM2Twips(Self::Pica2CM(value))
    }

    // Java: static float Points2Pica(float value)
    pub fn Points2Pica(value: f32) -> f32 {
        // Java: return CM2Pica(Points2CM(value));
        Self::CM2Pica(Self::Points2CM(value))
    }

    // Java: static float EighthPoints2Points(float value)
    pub fn EighthPoints2Points(value: f32) -> f32 {
        // Java: return value / 8.0F;
        value / 8.0
    }

    // Java: static float Points2EighthPoints(float value)
    pub fn Points2EighthPoints(value: f32) -> f32 {
        // Java: return value * 8.0F;
        value * 8.0
    }

    // Java: static float FixedPoint2Float(int value)
    pub fn FixedPoint2Float(value: i32) -> f32 {
        // Java: int fractional = value & '\uffff';
        let fractional = value & 0xFFFF;
        // Java: int integral = (value & -65536) >> 16;
        let integral = (value & -65536i32) >> 16;
        // Java: return (float)integral + (float)fractional / 65536.0F;
        (integral as f32) + (fractional as f32) / 65536.0
    }

    // Java: static float pixels2Points(float value, int densityDpi)
    pub fn pixels2Points(value: f32, density_dpi: i32) -> f32 {
        // Java: return Inches2Points(pixels2Inches(value, densityDpi));
        Self::Inches2Points(Self::pixels2Inches_f32(value, density_dpi))
    }

    // Java: static float points2Pixels(float value, int densityDpi)
    pub fn points2Pixels(value: f32, density_dpi: i32) -> f32 {
        // Java: return inches2Pixels(Points2Inches(value), densityDpi);
        Self::inches2Pixels(Self::Points2Inches(value), density_dpi)
    }

    // Java: static float pixels2EMU(float value, int densityDpi)
    pub fn pixels2EMU(value: f32, density_dpi: i32) -> f32 {
        // Java: return Inches2EMU(pixels2Inches(value, densityDpi));
        Self::Inches2EMU(Self::pixels2Inches_f32(value, density_dpi))
    }

    // Java: static float Points2Quarter(float point)
    pub fn Points2Quarter(point: f32) -> f32 {
        // Java: return point * 4.0F;
        point * 4.0
    }

    // Java: static float Quarters2Point(float quarter)
    pub fn Quarters2Point(quarter: f32) -> f32 {
        // Java: return quarter / 4.0F;
        quarter / 4.0
    }

    // Java: static float Pitch2Inch(float pitch)
    pub fn Pitch2Inch(pitch: f32) -> f32 {
        // Java: return pitch / 294912.0F;
        pitch / 294912.0
    }

    // Java: static float Inch2Pitch(float inch)
    pub fn Inch2Pitch(inch: f32) -> f32 {
        // Java: return inch * 294912.0F;
        inch * 294912.0
    }
}
