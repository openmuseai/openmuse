// java package cn.wps.et.ss.formula.functions.util;
use crate::java_runtime::{BigDecimal, RoundingMode};
pub struct MathX;
impl MathX {
    // java public static BigDecimal round15(double n)
    pub fn round15(n: f64) -> BigDecimal {
        // java BigDecimal nn = BigDecimal.valueOf(n);
        let nn = BigDecimal::valueOf(n);
        // java return nn.setScale(15 + nn.scale() - nn.precision(), RoundingMode.HALF_UP);
        nn.setScale(15 + nn.scale() - nn.precision(), RoundingMode::HALF_UP)
    }
// java public static double min(double[] values, int begin, int end) {
// java double m = values[begin];
// java
// java for(int i = begin + 1; i < end; ++i) {
// java if (m > values[i]) {
// java m = values[i];
// java }
// java }
// java
// java return m;
// java }
pub fn min(values:&[f64],begin:usize,end:usize)->f64 {
let mut m=values[begin];for i in begin+1..end{if m>values[i]{m=values[i];}}m
}
// java public static double max(double[] values, int begin, int end) {
// java double m = values[begin];
// java
// java for(int i = begin + 1; i < end; ++i) {
// java if (m < values[i]) {
// java m = values[i];
// java }
// java }
// java
// java return m;
// java }
pub fn max(values:&[f64],begin:usize,end:usize)->f64 {
let mut m=values[begin];for i in begin+1..end{if m<values[i]{m=values[i];}}m
}
}
impl MathX{// java public static double sum(double[] values, int begin, int end) {
// java double sum = (double)0.0F;
// java
// java for(int i = begin; i < end; ++i) {
// java sum += values[i];
// java }
// java
// java return sum;
// java }
pub fn sum(v:&[f64])->f64 {
let mut sum=0.0;for x in v{sum+=*x;}sum
}
// java public static double average(double[] values, int begin, int end) {
// java return sum(values, begin, end) / (double)(end - begin);
// java }
pub fn average(v:&[f64])->f64 {
Self::sum(v)/v.len()as f64
}
}
// rust: java.util.Random native adapter for MathX.RAND; randomized pivots remain randomized.
pub struct JavaRandom{seed:u64}impl JavaRandom{pub fn new(seed:u64)->Self{Self{seed:(seed^0x5deece66d)&((1u64<<48)-1)}}fn next(&mut self,bits:u32)->i32{self.seed=self.seed.wrapping_mul(0x5deece66d).wrapping_add(11)&((1u64<<48)-1);(self.seed>>(48-bits))as i32}pub fn nextInt(&mut self,bound:i32)->i32{assert!(bound>0);if bound&-bound==bound{return ((bound as i64*self.next(31)as i64)>>31)as i32;}loop{let bits=self.next(31);let val=bits%bound;if bits.wrapping_sub(val).wrapping_add(bound-1)>=0{return val;}}}}
pub static RAND:std::sync::LazyLock<std::sync::Mutex<JavaRandom>>=std::sync::LazyLock::new(||std::sync::Mutex::new(JavaRandom::new(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()as u64)));
impl MathX{// java public static double floor(double n, double s) {
// java double f;
// java if ((!(n < (double)0.0F) || !(s > (double)0.0F)) && (!(n > (double)0.0F) || !(s < (double)0.0F)) && (s != (double)0.0F || n == (double)0.0F)) {
// java f = n != (double)0.0F && s != (double)0.0F ? Math.floor(round15(n / s).doubleValue()) * s : (double)0.0F;
// java } else {
// java f = Double.NaN;
// java }
// java
// java return f;
// java }
pub fn floor(n:f64,s:f64)->f64 {
if (n<0.0&&s>0.0)||(n>0.0&&s<0.0)||(s==0.0&&n!=0.0){f64::NAN}else if n!=0.0&&s!=0.0{Self::round15(n/s).doubleValue().floor()*s}else{0.0}
}
// java public static double roundDown(double n, int p) {
// java if (!Double.isNaN(n) && !Double.isInfinite(n)) {
// java BigDecimal nn = round15(n);
// java return nn.setScale(p, RoundingMode.DOWN).doubleValue();
// java } else {
// java return Double.NaN;
// java }
// java }
pub fn roundDown(n:f64,p:i32)->f64 {
if n.is_finite(){Self::round15(n).setScale(p,crate::java_runtime::RoundingMode::DOWN).doubleValue()}else{f64::NAN}
}
}
impl JavaRandom{pub fn nextBoolean(&mut self)->bool{self.next(1)!=0}}
