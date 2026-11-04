use num_bigint::BigUint;use super::NormalisedDecimal::NormalisedDecimal;
// java final class ExpandedDouble
pub struct ExpandedDouble{significand:BigUint,exp:i32}
impl ExpandedDouble{
// java public static ExpandedDouble fromRawBitsAndExponent(long rawBits, int exp) {
// java return new ExpandedDouble(getFrac(rawBits), exp);
// java }
pub fn fromRawBitsAndExponent(raw:u64,exp:i32)->Self {
Self{significand:BigUint::from((raw&4503599627370495)|4503599627370496)<<11,exp}
}
// java public NormalisedDecimal normaliseBaseTen() {
// java return NormalisedDecimal.create(this._significand, this._binaryExponent);
// java }
pub fn normaliseBaseTen(&self)->NormalisedDecimal {
NormalisedDecimal::create(self.significand.clone(),self.exp)
}
// java public ExpandedDouble(long rawBits) {
// java int biasedExp = (int)(rawBits >> 52);
// java if (biasedExp == 0) {
// java BigInteger frac = BigInteger.valueOf(rawBits).and(BI_FRAC_MASK);
// java int expAdj = 64 - frac.bitLength();
// java this._significand = frac.shiftLeft(expAdj);
// java this._binaryExponent = (biasedExp & 2047) - 1023 - expAdj;
// java } else {
// java BigInteger frac = getFrac(rawBits);
// java this._significand = frac;
// java this._binaryExponent = (biasedExp & 2047) - 1023;
// java }
// java
// java }
pub fn ExpandedDouble(raw:u64)->Self {
let exp=(raw>>52)as i32;if exp==0{let frac=BigUint::from(raw&4503599627370495);let adj=64-frac.bits() as i32;Self{significand:frac<<adj as usize,exp:(exp&2047)-1023-adj}}else{Self::fromRawBitsAndExponent(raw,(exp&2047)-1023)}
}
// java ExpandedDouble(BigInteger frac, int binaryExp) {
// java if (frac.bitLength() != 64) {
// java throw new IllegalArgumentException("bad bit length");
// java } else {
// java this._significand = frac;
// java this._binaryExponent = binaryExp;
// java }
// java }
pub fn new(frac:BigUint,exp:i32)->Self {
assert_eq!(frac.bits(),64,"bad bit length");Self{significand:frac,exp}
}
// java public int getBinaryExponent() {
// java return this._binaryExponent;
// java }
pub fn getBinaryExponent(&self)->i32 {
self.exp
}
// java public BigInteger getSignificand() {
// java return this._significand;
// java }
pub fn getSignificand(&self)->BigUint {
self.significand.clone()
}
}
