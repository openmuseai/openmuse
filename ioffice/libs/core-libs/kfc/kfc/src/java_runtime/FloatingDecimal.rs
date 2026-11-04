/*
 * Copyright (c) 1996, 2016, Oracle and/or its affiliates. All rights reserved.
 * DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
 *
 * This code is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License version 2 only, as
 * published by the Free Software Foundation.  Oracle designates this
 * particular file as subject to the "Classpath" exception as provided
 * by Oracle in the LICENSE file that accompanied this code.
 *
 * This code is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
 * version 2 for more details (a copy is included in the LICENSE file that
 * accompanied this code).
 *
 * You should have received a copy of the GNU General Public License version
 * 2 along with this work; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
 *
 * Please contact Oracle, 500 Oracle Parkway, Redwood Shores, CA 94065 USA
 * or visit www.oracle.com if you need additional information or have any
 * questions.
 */
// Adapted from OpenJDK 17 jdk.internal.math.FloatingDecimal, BinaryToASCIIBuffer.
// Copyright (c) 1996, 2016, Oracle and/or its affiliates. GPL-2.0 with Classpath exception.
// Native runtime adapter: keep legacy Double/Float.toString, including its integer
// fast paths and strict stopping tests. BigUint implements FDBigInteger arithmetic.
use num_bigint::BigUint;
use num_traits::{ToPrimitive,Zero};
const EXP_SHIFT:i32=52;
const FRACT_HOB:u64=1<<52;
const MASK:u64=0x000fffffffffffff;
const N_5_BITS:[i32;27]=[0,3,5,7,10,12,14,17,19,21,24,26,28,31,33,35,38,40,42,45,47,49,52,54,56,59,61];
const INSIGNIFICANT:[i32;64]=[0,0,0,0,1,1,1,2,2,2,3,3,3,3,4,4,4,5,5,5,6,6,6,6,7,7,7,8,8,8,9,9,9,9,10,10,10,11,11,11,12,12,12,12,13,13,13,14,14,14,15,15,15,15,16,16,16,17,17,17,18,18,18,19];
pub struct FloatingDecimal;
impl FloatingDecimal {
    // java static BinaryToASCIIConverter getBinaryToASCIIConverter(double d, boolean isCompatibleFormat) {
    pub fn toJavaFormatString(d:f64)->String{
        // java long dBits = Double.doubleToRawLongBits(d); boolean isNegative = (dBits&DoubleConsts.SIGN_BIT_MASK) != 0;
        let dBits=d.to_bits();let negative=dBits>>63!=0;
        // java long fractBits = dBits & DoubleConsts.SIGNIF_BIT_MASK; int binExp = (int)((dBits&DoubleConsts.EXP_BIT_MASK) >> EXP_SHIFT);
        let mut fractBits=dBits&MASK;let mut binExp=((dBits>>52)&2047)as i32;
        // java if (binExp == (int)(DoubleConsts.EXP_BIT_MASK>>EXP_SHIFT)) {
        if binExp==2047{
            // java if (fractBits == 0L) { return isNegative ? B2AC_NEGATIVE_INFINITY : B2AC_POSITIVE_INFINITY; } else { return B2AC_NOT_A_NUMBER; }
            return if fractBits==0{if negative{"-Infinity"}else{"Infinity"}}else{"NaN"}.into();
        }
        // java int nSignificantBits; if (binExp == 0) {
        let nSignificantBits;if binExp==0{
            // java if (fractBits == 0L) { return isNegative ? B2AC_NEGATIVE_ZERO : B2AC_POSITIVE_ZERO; }
            if fractBits==0{return if negative{"-0.0"}else{"0.0"}.into();}
            // java int leadingZeros = Long.numberOfLeadingZeros(fractBits); int shift = leadingZeros-(63-EXP_SHIFT);
            let leadingZeros=fractBits.leading_zeros()as i32;let shift=leadingZeros-(63-EXP_SHIFT);
            // java fractBits <<= shift; binExp = 1 - shift; nSignificantBits = 64-leadingZeros;
            fractBits<<=shift;binExp=1-shift;nSignificantBits=64-leadingZeros;
        }else{
            // java fractBits |= FRACT_HOB; nSignificantBits = EXP_SHIFT+1;
            fractBits|=FRACT_HOB;nSignificantBits=EXP_SHIFT+1;
        }
        // java binExp -= DoubleConsts.EXP_BIAS; BinaryToASCIIBuffer buf = getBinaryToASCIIBuffer(); buf.setSign(isNegative);
        binExp-=1023;let mut buf=BinaryToASCIIBuffer::new(negative);
        // java buf.dtoa(binExp, fractBits, nSignificantBits, isCompatibleFormat); return buf;
        buf.dtoa(binExp,fractBits,nSignificantBits,true);buf.getChars()
    }
    // java private static BinaryToASCIIConverter getBinaryToASCIIConverter(float f) {
    pub fn toJavaFormatStringFloat(f:f32)->String{
        // java int fBits = Float.floatToRawIntBits(f); boolean isNegative = (fBits&FloatConsts.SIGN_BIT_MASK) != 0;
        let bits=f.to_bits();let negative=bits>>31!=0;
        // java int fractBits = fBits&FloatConsts.SIGNIF_BIT_MASK; int binExp = (fBits&FloatConsts.EXP_BIT_MASK) >> SINGLE_EXP_SHIFT;
        let mut fractBits=bits&0x7fffff;let mut binExp=((bits>>23)&255)as i32;
        // java if (binExp == (FloatConsts.EXP_BIT_MASK>>SINGLE_EXP_SHIFT)) {
        if binExp==255{
            // java if (fractBits == 0L) { return isNegative ? B2AC_NEGATIVE_INFINITY : B2AC_POSITIVE_INFINITY; } else { return B2AC_NOT_A_NUMBER; }
            return if fractBits==0{if negative{"-Infinity"}else{"Infinity"}}else{"NaN"}.into();
        }
        // java int nSignificantBits; if (binExp == 0) {
        let nSignificantBits;if binExp==0{
            // java if (fractBits == 0) { return isNegative ? B2AC_NEGATIVE_ZERO : B2AC_POSITIVE_ZERO; }
            if fractBits==0{return if negative{"-0.0"}else{"0.0"}.into();}
            // java int leadingZeros = Integer.numberOfLeadingZeros(fractBits); int shift = leadingZeros-(31-SINGLE_EXP_SHIFT);
            let leadingZeros=fractBits.leading_zeros()as i32;let shift=leadingZeros-8;
            // java fractBits <<= shift; binExp = 1 - shift; nSignificantBits = 32 - leadingZeros;
            fractBits<<=shift;binExp=1-shift;nSignificantBits=32-leadingZeros;
        }else{
            // java fractBits |= SINGLE_FRACT_HOB; nSignificantBits = SINGLE_EXP_SHIFT+1;
            fractBits|=1<<23;nSignificantBits=24;
        }
        // java binExp -= FloatConsts.EXP_BIAS; BinaryToASCIIBuffer buf = getBinaryToASCIIBuffer(); buf.setSign(isNegative);
        binExp-=127;let mut buf=BinaryToASCIIBuffer::new(negative);
        // java buf.dtoa(binExp, ((long)fractBits)<<(EXP_SHIFT-SINGLE_EXP_SHIFT), nSignificantBits, true); return buf;
        buf.dtoa(binExp,(fractBits as u64)<<29,nSignificantBits,true);buf.getChars()
    }
}
struct BinaryToASCIIBuffer{isNegative:bool,decExponent:i32,firstDigitIndex:usize,nDigits:usize,digits:[u8;20]}
impl BinaryToASCIIBuffer{
    fn new(isNegative:bool)->Self{Self{isNegative,decExponent:0,firstDigitIndex:0,nDigits:0,digits:[0;20]}}
    // java private void developLongDigits(int decExponent, long lvalue, int insignificantDigits) {
    fn developLongDigits(&mut self,mut decExponent:i32,mut lvalue:u64,insignificantDigits:i32){
        // java if (insignificantDigits != 0) {
        if insignificantDigits!=0{
            // java long pow10 = FDBigInteger.LONG_5_POW[insignificantDigits] << insignificantDigits; long residue = lvalue % pow10;
            let pow10=5u64.pow(insignificantDigits as u32)<<insignificantDigits;let residue=lvalue%pow10;
            // java lvalue /= pow10; decExponent += insignificantDigits; if (residue >= (pow10>>1)) { lvalue++; }
            lvalue/=pow10;decExponent+=insignificantDigits;if residue>=pow10>>1{lvalue+=1;}
        }
        // java int digitno = digits.length -1; int c;
        let mut digitno=self.digits.len()-1;let mut c;
        // java if (lvalue <= Integer.MAX_VALUE) {
        if lvalue<=i32::MAX as u64{
            // java int ivalue = (int)lvalue; c = ivalue%10; ivalue /= 10;
            let mut ivalue=lvalue as i32;c=ivalue%10;ivalue/=10;
            // java while (c == 0) { decExponent++; c = ivalue%10; ivalue /= 10; }
            while c==0{decExponent+=1;c=ivalue%10;ivalue/=10;}
            // java while (ivalue != 0) { digits[digitno--] = (char)(c+'0'); decExponent++; c = ivalue%10; ivalue /= 10; }
            while ivalue!=0{self.digits[digitno]=c as u8+48;digitno-=1;decExponent+=1;c=ivalue%10;ivalue/=10;}
            // java digits[digitno] = (char)(c+'0');
            self.digits[digitno]=c as u8+48;
        }else{
            // java c = (int)(lvalue%10L); lvalue /= 10L;
            c=(lvalue%10)as i32;lvalue/=10;
            // java while (c == 0) { decExponent++; c = (int)(lvalue%10L); lvalue /= 10L; }
            while c==0{decExponent+=1;c=(lvalue%10)as i32;lvalue/=10;}
            // java while (lvalue != 0L) { digits[digitno--] = (char)(c+'0'); decExponent++; c = (int)(lvalue%10L); lvalue /= 10; }
            while lvalue!=0{self.digits[digitno]=c as u8+48;digitno-=1;decExponent+=1;c=(lvalue%10)as i32;lvalue/=10;}
            // java digits[digitno] = (char)(c+'0');
            self.digits[digitno]=c as u8+48;
        }
        // java this.decExponent = decExponent+1; this.firstDigitIndex = digitno; this.nDigits = this.digits.length - digitno;
        self.decExponent=decExponent+1;self.firstDigitIndex=digitno;self.nDigits=self.digits.len()-digitno;
    }
    // java private void dtoa(int binExp, long fractBits, int nSignificantBits, boolean isCompatibleFormat) {
    fn dtoa(&mut self,binExp:i32,mut fractBits:u64,nSignificantBits:i32,isCompatibleFormat:bool){
        // java final int tailZeros = Long.numberOfTrailingZeros(fractBits); final int nFractBits = EXP_SHIFT+1-tailZeros;
        let tailZeros=fractBits.trailing_zeros()as i32;let nFractBits=EXP_SHIFT+1-tailZeros;
        // java int nTinyBits = Math.max(0, nFractBits - binExp - 1);
        let nTinyBits=0.max(nFractBits-binExp-1);
        // java if (binExp <= MAX_SMALL_BIN_EXP && binExp >= MIN_SMALL_BIN_EXP) {
        if (-21..=62).contains(&binExp){
            // java if ((nTinyBits < FDBigInteger.LONG_5_POW.length) && ((nFractBits + N_5_BITS[nTinyBits]) < 64)) {
            if nTinyBits<27&&nFractBits+N_5_BITS[nTinyBits as usize]<64{
                // java if (nTinyBits == 0) {
                if nTinyBits==0{
                    // java if (binExp > nSignificantBits) { insignificant = insignificantDigitsForPow2(binExp-nSignificantBits-1); } else { insignificant = 0; }
                    let insignificant=if binExp>nSignificantBits{let p=binExp-nSignificantBits-1;if p>1&&p<64{INSIGNIFICANT[p as usize]}else{0}}else{0};
                    // java if (binExp >= EXP_SHIFT) { fractBits <<= (binExp-EXP_SHIFT); } else { fractBits >>>= (EXP_SHIFT-binExp); }
                    if binExp>=EXP_SHIFT{fractBits<<=binExp-EXP_SHIFT;}else{fractBits>>=EXP_SHIFT-binExp;}
                    // java developLongDigits(0, fractBits, insignificant); return;
                    self.developLongDigits(0,fractBits,insignificant);return;
                }
            }
        }
        // java int decExp = estimateDecExp(fractBits,binExp);
        let mut decExp=Self::estimateDecExp(fractBits,binExp);
        // java B5 = Math.max(0, -decExp); B2 = B5 + nTinyBits + binExp;
        let B5=0.max(-decExp);let mut B2=B5+nTinyBits+binExp;
        // java S5 = Math.max(0, decExp); S2 = S5 + nTinyBits;
        let S5=0.max(decExp);let mut S2=S5+nTinyBits;
        // java M5 = B5; M2 = B2 - nSignificantBits;
        let M5=B5;let mut M2=B2-nSignificantBits;
        // java fractBits >>>= tailZeros; B2 -= nFractBits-1; int common2factor = Math.min(B2, S2);
        fractBits>>=tailZeros;B2-=nFractBits-1;let common2factor=B2.min(S2);
        // java B2 -= common2factor; S2 -= common2factor; M2 -= common2factor;
        B2-=common2factor;S2-=common2factor;M2-=common2factor;
        // java if (nFractBits == 1) { M2 -= 1; }
        if nFractBits==1{M2-=1;}
        // java if (M2 < 0) { B2 -= M2; S2 -= M2; M2 = 0; }
        if M2<0{B2-=M2;S2-=M2;M2=0;}
        // java int ndigit = 0; boolean low, high; long lowDigitDifference; int q;
        let mut ndigit=0;let(mut low,mut high);let lowDigitDifference:i64;
        // java int Bbits = nFractBits + B2 + ((B5 < N_5_BITS.length) ? N_5_BITS[B5] : (B5*3));
        let Bbits=nFractBits+B2+if B5<27{N_5_BITS[B5 as usize]}else{B5*3};
        // java int tenSbits = S2+1 + (((S5+1) < N_5_BITS.length) ? N_5_BITS[S5+1] : ((S5+1)*3));
        let tenSbits=S2+1+if S5+1<27{N_5_BITS[(S5+1)as usize]}else{(S5+1)*3};
        // rust: the two original int/long branches share identical source statements.
        // Instantiate at the original width; every arithmetic overflow wraps as Java.
        macro_rules! integer_branch {($t:ty)=>{{
            // java int/long b = (fractBits * FDBigInteger.SMALL/LONG_5_POW[B5]) << B2;
            let mut b=(fractBits as $t).wrapping_mul((5 as $t).pow(B5 as u32)).wrapping_shl(B2 as u32);
            // java int/long s = FDBigInteger.SMALL/LONG_5_POW[S5] << S2;
            let s=(5 as $t).pow(S5 as u32).wrapping_shl(S2 as u32);
            // java int/long m = FDBigInteger.SMALL/LONG_5_POW[M5] << M2; int/long tens = s * 10;
            let mut m=(5 as $t).pow(M5 as u32).wrapping_shl(M2 as u32);let tens=s.wrapping_mul(10);
            // java q = b / s; b = 10 * (b % s); m *= 10; low = (b < m); high = (b+m > tens);
            let mut q=b/s;b=(b%s).wrapping_mul(10);m=m.wrapping_mul(10);low=b<m;high=b.wrapping_add(m)>tens;
            // java if ((q == 0) && !high) { decExp--; } else { digits[ndigit++] = (char)('0' + q); }
            if q==0&&!high{decExp-=1;}else{self.digits[ndigit]=48+q as u8;ndigit+=1;}
            // java if (!isCompatibleFormat || decExp < -3 || decExp >= 8) { high = low = false; }
            if !isCompatibleFormat||decExp < -3||decExp>=8{high=false;low=false;}
            // java while (!low && !high) {
            while !low&&!high{
                // java q = b / s; b = 10 * (b % s); m *= 10;
                q=b/s;b=(b%s).wrapping_mul(10);m=m.wrapping_mul(10);
                // java if (m > 0L) { low = (b < m); high = (b+m > tens); } else { low = true; high = true; }
                if m>0{low=b<m;high=b.wrapping_add(m)>tens;}else{low=true;high=true;}
                // java digits[ndigit++] = (char)('0' + q);
                self.digits[ndigit]=48+q as u8;ndigit+=1;
            }
            // java lowDigitDifference = (b<<1) - tens;
            lowDigitDifference=b.wrapping_shl(1).wrapping_sub(tens)as i64;
        }}}
        // java if (Bbits < 64 && tenSbits < 64) { if (Bbits < 32 && tenSbits < 32) {
        if Bbits<64&&tenSbits<64{if Bbits<32&&tenSbits<32{integer_branch!(i32);}else{integer_branch!(i64);}}
        else{
            // java FDBigInteger Sval = FDBigInteger.valueOfPow52(S5, S2); int shiftBias = Sval.getNormalizationBias(); Sval = Sval.leftShift(shiftBias);
            let mut Sval=BigUint::from(5u8).pow(S5 as u32)<<S2 as usize;
            let highWordBits=(Sval.bits()-1)%32+1;
            let shiftBias=if highWordBits<=28{28-highWordBits}else{60-highWordBits}as usize;Sval<<=shiftBias;
            // java FDBigInteger Bval = FDBigInteger.valueOfMulPow52(fractBits, B5, B2 + shiftBias);
            let mut Bval=(BigUint::from(fractBits)*BigUint::from(5u8).pow(B5 as u32))<<(B2 as usize+shiftBias);
            // java FDBigInteger Mval = FDBigInteger.valueOfPow52(M5 + 1, M2 + shiftBias + 1);
            let mut Mval=BigUint::from(5u8).pow((M5+1)as u32)<<(M2 as usize+shiftBias+1);
            // java FDBigInteger tenSval = FDBigInteger.valueOfPow52(S5 + 1, S2 + shiftBias + 1);
            let tenSval=BigUint::from(5u8).pow((S5+1)as u32)<<(S2 as usize+shiftBias+1);
            // java q = Bval.quoRemIteration(Sval); low = (Bval.cmp(Mval) < 0); high = tenSval.addAndCmp(Bval,Mval)<=0;
            let mut q=(&Bval/&Sval).to_u8().unwrap();Bval=(&Bval%&Sval)*10u8;low=Bval<Mval;high=tenSval<=&Bval+&Mval;
            // java if ((q == 0) && !high) { decExp--; } else { digits[ndigit++] = (char)('0' + q); }
            if q==0&&!high{decExp-=1;}else{self.digits[ndigit]=48+q;ndigit+=1;}
            // java if (!isCompatibleFormat || decExp < -3 || decExp >= 8) { high = low = false; }
            if !isCompatibleFormat||decExp < -3||decExp>=8{high=false;low=false;}
            // java while (!low && !high) {
            while !low&&!high{
                // java q = Bval.quoRemIteration(Sval); Mval = Mval.multBy10(); low = (Bval.cmp(Mval) < 0); high = tenSval.addAndCmp(Bval,Mval)<=0;
                q=(&Bval/&Sval).to_u8().unwrap();Bval=(&Bval%&Sval)*10u8;Mval*=10u8;low=Bval<Mval;high=tenSval<=&Bval+&Mval;
                // java digits[ndigit++] = (char)('0' + q);
                self.digits[ndigit]=48+q;ndigit+=1;
            }
            // java if (high && low) { Bval = Bval.leftShift(1); lowDigitDifference = Bval.cmp(tenSval); } else { lowDigitDifference = 0L; }
            lowDigitDifference=if high&&low{Bval<<=1;match Bval.cmp(&tenSval){std::cmp::Ordering::Less=>-1,std::cmp::Ordering::Equal=>0,std::cmp::Ordering::Greater=>1}}else{0};
        }
        // java this.decExponent = decExp+1; this.firstDigitIndex = 0; this.nDigits = ndigit;
        self.decExponent=decExp+1;self.firstDigitIndex=0;self.nDigits=ndigit;
        // java if (high) { if (low) { if (lowDigitDifference == 0L) { if ((digits[firstDigitIndex+nDigits-1]&1) != 0) { roundup(); } } else if (lowDigitDifference > 0) { roundup(); } } else { roundup(); } }
        if high{if low{if lowDigitDifference==0{if self.digits[self.firstDigitIndex+self.nDigits-1]&1!=0{self.roundup();}}else if lowDigitDifference>0{self.roundup();}}else{self.roundup();}}
    }
    // java private void roundup() {
    fn roundup(&mut self){
        // java int i = (firstDigitIndex + nDigits - 1); int q = digits[i];
        let mut i=self.firstDigitIndex+self.nDigits-1;let mut q=self.digits[i];
        // java if (q == '9') {
        if q==57{
            // java while (q == '9' && i > firstDigitIndex) { digits[i] = '0'; q = digits[--i]; }
            while q==57&&i>self.firstDigitIndex{self.digits[i]=48;i-=1;q=self.digits[i];}
            // java if (q == '9') { decExponent += 1; digits[firstDigitIndex] = '1'; return; }
            if q==57{self.decExponent+=1;self.digits[self.firstDigitIndex]=49;return;}
        }
        // java digits[i] = (char)(q + 1);
        self.digits[i]=q+1;
    }
    // java static int estimateDecExp(long fractBits, int binExp) {
    fn estimateDecExp(fractBits:u64,binExp:i32)->i32{
        // java double d2 = Double.longBitsToDouble(EXP_ONE | (fractBits & DoubleConsts.SIGNIF_BIT_MASK));
        let d2=f64::from_bits(0x3ff0000000000000|(fractBits&MASK));
        // java double d = (d2-1.5D)*0.289529654D + 0.176091259 + (double)binExp * 0.301029995663981;
        let d=(d2-1.5)*0.289529654+0.176091259+binExp as f64*0.301029995663981;
        // java long dBits = Double.doubleToRawLongBits(d); int exponent = (int)((dBits & DoubleConsts.EXP_BIT_MASK) >> EXP_SHIFT) - DoubleConsts.EXP_BIAS; boolean isNegative = (dBits & DoubleConsts.SIGN_BIT_MASK) != 0;
        let dBits=d.to_bits();let exponent=((dBits>>52)&2047)as i32-1023;let isNegative=dBits>>63!=0;
        // java if (exponent>=0 && exponent<52) {
        if (0..52).contains(&exponent){
            // java long mask = DoubleConsts.SIGNIF_BIT_MASK >> exponent; int r = (int)(((dBits&DoubleConsts.SIGNIF_BIT_MASK) | FRACT_HOB)>>(EXP_SHIFT-exponent));
            let mask=MASK>>exponent;let r=(((dBits&MASK)|FRACT_HOB)>>(EXP_SHIFT-exponent))as i32;
            // java return isNegative ? (((mask & dBits) == 0L) ? -r : -r-1) : r;
            if isNegative{if mask&dBits==0{-r}else{-r-1}}else{r}
        // java } else if (exponent < 0) { return (((dBits&~DoubleConsts.SIGN_BIT_MASK) == 0) ? 0 : ((isNegative) ? -1 : 0)); } else { return (int)d; }
        }else if exponent<0{if dBits&0x7fffffffffffffff==0{0}else if isNegative{-1}else{0}}else{d as i32}
    }
    // java private int getChars(char[] result) {
    fn getChars(&self)->String{
        // java if (isNegative) { result[0] = '-'; i = 1; }
        let mut result=if self.isNegative{String::from("-")}else{String::new()};
        let digits=std::str::from_utf8(&self.digits[self.firstDigitIndex..self.firstDigitIndex+self.nDigits]).unwrap();
        // java if (decExponent > 0 && decExponent < 8) {
        if self.decExponent>0&&self.decExponent<8{
            // java int charLength = Math.min(nDigits, decExponent); System.arraycopy(digits, firstDigitIndex, result, i, charLength); i += charLength;
            let charLength=self.nDigits.min(self.decExponent as usize);result.push_str(&digits[..charLength]);
            // java if (charLength < decExponent) { charLength = decExponent - charLength; Arrays.fill(result,i,i+charLength,'0'); i += charLength; result[i++] = '.'; result[i++] = '0'; }
            if charLength<self.decExponent as usize{result.extend(std::iter::repeat_n('0',self.decExponent as usize-charLength));result.push_str(".0");}
            else{
                // java result[i++] = '.'; if (charLength < nDigits) { int t = nDigits - charLength; System.arraycopy(digits, firstDigitIndex+charLength, result, i, t); i += t; } else { result[i++] = '0'; }
                result.push('.');if charLength<self.nDigits{result.push_str(&digits[charLength..]);}else{result.push('0');}
            }
        // java } else if (decExponent <= 0 && decExponent > -3) {
        }else if self.decExponent<=0&&self.decExponent > -3{
            // java result[i++] = '0'; result[i++] = '.'; if (decExponent != 0) { Arrays.fill(result, i, i-decExponent, '0'); i -= decExponent; } System.arraycopy(digits, firstDigitIndex, result, i, nDigits); i += nDigits;
            result.push_str("0.");if self.decExponent!=0{result.extend(std::iter::repeat_n('0',(-self.decExponent)as usize));}result.push_str(digits);
        }else{
            // java result[i++] = digits[firstDigitIndex]; result[i++] = '.'; if (nDigits > 1) { System.arraycopy(digits, firstDigitIndex+1, result, i, nDigits - 1); i += nDigits - 1; } else { result[i++] = '0'; }
            result.push(digits.as_bytes()[0]as char);result.push('.');if self.nDigits>1{result.push_str(&digits[1..]);}else{result.push('0');}
            // java result[i++] = 'E'; if (decExponent <= 0) { result[i++] = '-'; e = -decExponent + 1; } else { e = decExponent - 1; }
            result.push('E');let e=if self.decExponent<=0{result.push('-');-self.decExponent+1}else{self.decExponent-1};
            // java decimal exponent digits: 1, 2, or 3 characters.
            result.push_str(&e.to_string());
        }
        // java return i;
        result
    }
}
