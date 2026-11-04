use num_bigint::BigUint;use super::MutableFPNumber::MutableFPNumber;
// java final class NormalisedDecimal
pub struct NormalisedDecimal{whole:i64,frac:i32,exp:i32}
impl NormalisedDecimal{
// java NormalisedDecimal(long wholePart, int fracPart, int decimalExponent) {
// java this._wholePart = wholePart;
// java this._fractionalPart = fracPart;
// java this._relativeDecimalExponent = decimalExponent;
// java }
pub fn new(whole:i64,frac:i32,exp:i32)->Self {
Self{whole,frac,exp}
}
// java public static NormalisedDecimal create(BigInteger frac, int binaryExponent) {
// java int pow10;
// java if (binaryExponent <= 49 && binaryExponent >= 46) {
// java pow10 = 0;
// java } else {
// java int x = 15204352 - binaryExponent * 315653;
// java x += 524288;
// java pow10 = -(x >> 20);
// java }
// java
// java MutableFPNumber cc = new MutableFPNumber(frac, binaryExponent);
// java if (pow10 != 0) {
// java cc.multiplyByPowerOfTen(-pow10);
// java }
// java
// java switch (cc.get64BitNormalisedExponent()) {
// java case 46:
// java if (cc.isAboveMinRep()) {
// java break;
// java }
// java case 44:
// java case 45:
// java cc.multiplyByPowerOfTen(1);
// java --pow10;
// java case 47:
// java case 48:
// java break;
// java case 49:
// java if (cc.isBelowMaxRep()) {
// java break;
// java }
// java case 50:
// java cc.multiplyByPowerOfTen(-1);
// java ++pow10;
// java break;
// java default:
// java throw new IllegalStateException("Bad binary exp " + cc.get64BitNormalisedExponent() + ".");
// java }
// java
// java cc.normalise64bit();
// java return cc.createNormalisedDecimal(pow10);
// java }
pub fn create(frac:BigUint,exp:i32)->Self {
let mut pow10=if (46..=49).contains(&exp){0}else{-((15204352i32.wrapping_sub(exp.wrapping_mul(315653)).wrapping_add(524288))>>20)};let mut cc=MutableFPNumber::new(frac,exp);if pow10!=0{cc.multiplyByPowerOfTen(-pow10);}match cc.get64BitNormalisedExponent(){46 if cc.isAboveMinRep()=>{},44|45|46=>{cc.multiplyByPowerOfTen(1);pow10-=1;},47|48=>{},49 if cc.isBelowMaxRep()=>{},49|50=>{cc.multiplyByPowerOfTen(-1);pow10+=1;},_=>panic!("Bad binary exp")};cc.normalise64bit();cc.createNormalisedDecimal(pow10)
}
// java public NormalisedDecimal roundUnits() {
// java long wholePart = this._wholePart;
// java if (this._fractionalPart >= 8388608) {
// java ++wholePart;
// java }
// java
// java int de = this._relativeDecimalExponent;
// java return wholePart < 1000000000000000L ? new NormalisedDecimal(wholePart, 0, de) : new NormalisedDecimal(wholePart / 10L, 0, de + 1);
// java }
pub fn roundUnits(&self)->Self {
let whole=self.whole+if self.frac>=8388608{1}else{0};if whole<1000000000000000{Self::new(whole,0,self.exp)}else{Self::new(whole/10,0,self.exp+1)}
}
// java public int compareNormalised(NormalisedDecimal other) {
// java int cmp = this._relativeDecimalExponent - other._relativeDecimalExponent;
// java if (cmp != 0) {
// java return cmp;
// java } else if (this._wholePart > other._wholePart) {
// java return 1;
// java } else {
// java return this._wholePart < other._wholePart ? -1 : this._fractionalPart - other._fractionalPart;
// java }
// java }
pub fn compareNormalised(&self,other:&Self)->i32 {
let cmp=self.exp-other.exp;if cmp!=0{cmp}else if self.whole>other.whole{1}else if self.whole<other.whole{-1}else{self.frac-other.frac}
}
// java public int getDecimalExponent() {
// java return this._relativeDecimalExponent + 14;
// java }
pub fn getDecimalExponent(&self)->i32 {
self.exp+14
}
// java public String getSignificantDecimalDigits() {
// java return Long.toString(this._wholePart);
// java }
pub fn getSignificantDecimalDigits(&self)->String {
self.whole.to_string()
}
// java public String getSignificantDecimalDigitsLastDigitRounded() {
// java long wp = this._wholePart + 5L;
// java StringBuilder sb = new StringBuilder(24);
// java sb.append(wp);
// java sb.setCharAt(sb.length() - 1, '0');
// java return sb.toString();
// java }
pub fn getSignificantDecimalDigitsLastDigitRounded(&self)->String {
let mut s=(self.whole+5).to_string();s.pop();s.push('0');s
}
}
