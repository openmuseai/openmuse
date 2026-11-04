use super::ExpandedDouble::ExpandedDouble;
// java public final class NumberComparer
pub struct NumberComparer;
impl NumberComparer{
// java public static int compare(double a, double b) {
// java long rawBitsA = Double.doubleToLongBits(a);
// java long rawBitsB = Double.doubleToLongBits(b);
// java int biasedExponentA = IEEEDouble.getBiasedExponent(rawBitsA);
// java int biasedExponentB = IEEEDouble.getBiasedExponent(rawBitsB);
// java if (biasedExponentA == 2047) {
// java throw new IllegalArgumentException("Special double values are not allowed: " + toHex(a));
// java } else if (biasedExponentB == 2047) {
// java throw new IllegalArgumentException("Special double values are not allowed: " + toHex(a));
// java } else {
// java boolean aIsNegative = rawBitsA < 0L;
// java boolean bIsNegative = rawBitsB < 0L;
// java if (aIsNegative != bIsNegative) {
// java return aIsNegative ? -1 : 1;
// java } else {
// java int cmp = biasedExponentA - biasedExponentB;
// java int absExpDiff = Math.abs(cmp);
// java if (absExpDiff > 1) {
// java return aIsNegative ? -cmp : cmp;
// java } else if (absExpDiff != 1 && rawBitsA == rawBitsB) {
// java return 0;
// java } else if (biasedExponentA == 0) {
// java return biasedExponentB == 0 ? compareSubnormalNumbers(rawBitsA & 4503599627370495L, rawBitsB & 4503599627370495L, aIsNegative) : -compareAcrossSubnormalThreshold(rawBitsB, rawBitsA, aIsNegative);
// java } else if (biasedExponentB == 0) {
// java return compareAcrossSubnormalThreshold(rawBitsA, rawBitsB, aIsNegative);
// java } else {
// java ExpandedDouble edA = ExpandedDouble.fromRawBitsAndExponent(rawBitsA, biasedExponentA - 1023);
// java ExpandedDouble edB = ExpandedDouble.fromRawBitsAndExponent(rawBitsB, biasedExponentB - 1023);
// java NormalisedDecimal ndA = edA.normaliseBaseTen().roundUnits();
// java NormalisedDecimal ndB = edB.normaliseBaseTen().roundUnits();
// java cmp = ndA.compareNormalised(ndB);
// java return aIsNegative ? -cmp : cmp;
// java }
// java }
// java }
// java }
pub fn compare(a:f64,b:f64)->i32 {
let x=a.to_bits();let y=b.to_bits();let ex=((x>>52)&2047)as i32;let ey=((y>>52)&2047)as i32;assert!(ex!=2047&&ey!=2047,"Special double values are not allowed");let nx=x>>63!=0;let ny=y>>63!=0;if nx!=ny{return if nx{-1}else{1};}let cmp=ex-ey;if cmp.abs()>1{return if nx{-cmp}else{cmp};}if cmp.abs()!=1&&x==y{return 0;}if ex==0{if ey==0{return Self::compareSubnormalNumbers(x&4503599627370495,y&4503599627370495,nx);}return -Self::compareAcrossSubnormalThreshold(y,x,nx);}if ey==0{return Self::compareAcrossSubnormalThreshold(x,y,nx);}let a=ExpandedDouble::fromRawBitsAndExponent(x,ex-1023).normaliseBaseTen().roundUnits();let b=ExpandedDouble::fromRawBitsAndExponent(y,ey-1023).normaliseBaseTen().roundUnits();let cmp=a.compareNormalised(&b);if nx{-cmp}else{cmp}
}
// java private static int compareSubnormalNumbers(long fracA, long fracB, boolean isNegative) {
// java int cmp = fracA > fracB ? 1 : (fracA < fracB ? -1 : 0);
// java return isNegative ? -cmp : cmp;
// java }
pub fn compareSubnormalNumbers(a:u64,b:u64,negative:bool)->i32 {
let cmp=if a>b{1}else if a<b{-1}else{0};if negative{-cmp}else{cmp}
}
// java private static int compareAcrossSubnormalThreshold(long normalRawBitsA, long subnormalRawBitsB, boolean isNegative) {
// java long fracB = subnormalRawBitsB & 4503599627370495L;
// java if (fracB == 0L) {
// java return isNegative ? -1 : 1;
// java } else {
// java long fracA = normalRawBitsA & 4503599627370495L;
// java if (fracA <= 7L && fracB >= 4503599627370490L) {
// java if (fracA == 7L && fracB == 4503599627370490L) {
// java return 0;
// java } else {
// java return isNegative ? 1 : -1;
// java }
// java } else {
// java return isNegative ? -1 : 1;
// java }
// java }
// java }
pub fn compareAcrossSubnormalThreshold(a:u64,b:u64,negative:bool)->i32 {
let b=b&4503599627370495;if b==0{return if negative{-1}else{1};}let a=a&4503599627370495;if a<=7&&b>=4503599627370490{if a==7&&b==4503599627370490{0}else if negative{1}else{-1}}else if negative{-1}else{1}
}
}
