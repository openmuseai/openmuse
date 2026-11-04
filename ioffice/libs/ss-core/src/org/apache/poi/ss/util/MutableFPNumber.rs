use num_bigint::BigUint;use num_traits::{One,ToPrimitive};use super::NormalisedDecimal::NormalisedDecimal;
// java final class MutableFPNumber
pub struct MutableFPNumber{significand:BigUint,exponent:i32}
fn shift(v:BigUint,n:i32)->BigUint{if n>=0{v<<n as usize}else{v>>(-n) as usize}}
impl MutableFPNumber{
// java public MutableFPNumber(BigInteger frac, int binaryExponent) {
// java this._significand = frac;
// java this._binaryExponent = binaryExponent;
// java }
pub fn new(frac:BigUint,exponent:i32)->Self {
Self{significand:frac,exponent}
}
// java public void normalise64bit() {
// java int oldBitLen = this._significand.bitLength();
// java int sc = oldBitLen - 64;
// java if (sc != 0) {
// java if (sc < 0) {
// java throw new IllegalStateException("Not enough precision");
// java } else {
// java this._binaryExponent += sc;
// java if (sc > 32) {
// java int highShift = sc - 1 & 16777184;
// java this._significand = this._significand.shiftRight(highShift);
// java sc -= highShift;
// java oldBitLen -= highShift;
// java }
// java
// java if (sc < 1) {
// java throw new IllegalStateException();
// java } else {
// java this._significand = MutableFPNumber.Rounder.round(this._significand, sc);
// java if (this._significand.bitLength() > oldBitLen) {
// java ++sc;
// java ++this._binaryExponent;
// java }
// java
// java this._significand = this._significand.shiftRight(sc);
// java }
// java }
// java }
// java }
pub fn normalise64bit(&mut self) {
let mut old=self.significand.bits() as i32;let mut sc=old-64;if sc!=0{assert!(sc>=0,"Not enough precision");self.exponent+=sc;if sc>32{let high=(sc-1)&16777184;self.significand>>=high as usize;sc-=high;old-=high;}assert!(sc>=1);self.significand+=BigUint::one()<<(sc-1) as usize;if self.significand.bits() as i32>old{sc+=1;self.exponent+=1;}self.significand>>=sc as usize;}
}
// java public int get64BitNormalisedExponent() {
// java return this._binaryExponent + this._significand.bitLength() - 64;
// java }
pub fn get64BitNormalisedExponent(&self)->i32 {
self.exponent+self.significand.bits() as i32-64
}
// java public boolean isBelowMaxRep() {
// java int sc = this._significand.bitLength() - 64;
// java return this._significand.compareTo(BI_MAX_BASE.shiftLeft(sc)) < 0;
// java }
pub fn isBelowMaxRep(&self)->bool {
self.significand < shift(BigUint::parse_bytes(b"0E35FA9319FFFE000",16).unwrap(),self.significand.bits() as i32-64)
}
// java public boolean isAboveMinRep() {
// java int sc = this._significand.bitLength() - 64;
// java return this._significand.compareTo(BI_MIN_BASE.shiftLeft(sc)) > 0;
// java }
pub fn isAboveMinRep(&self)->bool {
self.significand > shift(BigUint::parse_bytes(b"0B5E620F47FFFE666",16).unwrap(),self.significand.bits() as i32-64)
}
// java public NormalisedDecimal createNormalisedDecimal(int pow10) {
// java int missingUnderBits = this._binaryExponent - 39;
// java int fracPart = this._significand.intValue() << missingUnderBits & 16777088;
// java long wholePart = this._significand.shiftRight(64 - this._binaryExponent - 1).longValue();
// java return new NormalisedDecimal(wholePart, fracPart, pow10);
// java }
pub fn createNormalisedDecimal(&self,pow10:i32)->NormalisedDecimal {
let low=(&self.significand&BigUint::from(u32::MAX)).to_u32().unwrap() as i32;let frac=low.wrapping_shl((self.exponent-39) as u32)&16777088;let whole=shift(self.significand.clone(),-(64-self.exponent-1)).to_u64().unwrap() as i64;NormalisedDecimal::new(whole,frac,pow10)
}
// java public void multiplyByPowerOfTen(int pow10) {
// java TenPower tp = MutableFPNumber.TenPower.getInstance(Math.abs(pow10));
// java if (pow10 < 0) {
// java this.mulShift(tp._divisor, tp._divisorShift);
// java } else {
// java this.mulShift(tp._multiplicand, tp._multiplierShift);
// java }
// java
// java }
pub fn multiplyByPowerOfTen(&mut self,pow10:i32) {
let t=TenPower::new(pow10.unsigned_abs());if pow10<0{self.mulShift(t.divisor,t.divisorShift);}else{self.mulShift(t.multiplicand,t.multiplierShift);}
}
// java private void mulShift(BigInteger multiplicand, int multiplierShift) {
// java this._significand = this._significand.multiply(multiplicand);
// java this._binaryExponent += multiplierShift;
// java int sc = this._significand.bitLength() - 72 & -32;
// java if (sc > 0) {
// java this._significand = this._significand.shiftRight(sc);
// java this._binaryExponent += sc;
// java }
// java
// java }
pub fn mulShift(&mut self,m:BigUint,exp:i32) {
self.significand*=m;self.exponent+=exp;let sc=(self.significand.bits() as i32-72)& -32;if sc>0{self.significand>>=sc as usize;self.exponent+=sc;}
}
}
struct TenPower{multiplicand:BigUint,divisor:BigUint,divisorShift:i32,multiplierShift:i32}
impl TenPower{// java private TenPower(int index) {
// java BigInteger fivePowIndex = FIVE.pow(index);
// java int bitsDueToFiveFactors = fivePowIndex.bitLength();
// java int px = 80 + bitsDueToFiveFactors;
// java BigInteger fx = BigInteger.ONE.shiftLeft(px).divide(fivePowIndex);
// java int adj = fx.bitLength() - 80;
// java this._divisor = fx.shiftRight(adj);
// java bitsDueToFiveFactors -= adj;
// java this._divisorShift = -(bitsDueToFiveFactors + index + 80);
// java int sc = fivePowIndex.bitLength() - 68;
// java if (sc > 0) {
// java this._multiplierShift = index + sc;
// java this._multiplicand = fivePowIndex.shiftRight(sc);
// java } else {
// java this._multiplierShift = index;
// java this._multiplicand = fivePowIndex;
// java }
// java
// java }
pub fn new(index:u32)->Self {
let five=BigUint::from(5u32).pow(index);let mut bits=five.bits() as i32;let px=80+bits;let fx=(BigUint::one()<<px as usize)/&five;let adj=fx.bits() as i32-80;let divisor=shift(fx,-adj);bits-=adj;let ds=-(bits+index as i32+80);let sc=five.bits() as i32-68;let(ms,m)=if sc>0{(index as i32+sc,five>>sc as usize)}else{(index as i32,five)};Self{multiplicand:m,divisor,divisorShift:ds,multiplierShift:ms}
}
}
