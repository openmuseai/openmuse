/*
 * Copyright (c) 2021, 2022, Oracle and/or its affiliates. All rights reserved.
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
// Android API 36 jdk.internal.math.FloatToDecimal / MathUtils native leaf port.
// Copyright (c) 2021, 2023, Oracle and/or its affiliates. GPL-2.0 with Classpath exception.
// Double.toString still uses FloatingDecimal on this target; Float.toString uses
// this class. Keep that runtime distinction outside the ET/KFC algorithms.
const P:i32=24;
const Q_MIN:i32=-149;
const C_MIN:i32=1<<23;
const MASK_28:i32=(1<<28)-1;
const MASK_32:i64=(1i64<<32)-1;
// java private final byte[] bytes = new byte[MAX_CHARS]; private int index;
pub struct FloatToDecimal{bytes:[u8;15],index:i32}
impl FloatToDecimal{
    // java public static String toString(float v) { return new FloatToDecimal().toDecimalString(v); }
    pub fn toString(v:f32)->String{Self{bytes:[0;15],index:0}.toDecimalString(v)}
    // java private String toDecimalString(float v) {
    fn toDecimalString(&mut self,v:f32)->String{
        // java return switch (toDecimal(v)) { case NON_SPECIAL -> charsToString(); case PLUS_ZERO -> "0.0"; case MINUS_ZERO -> "-0.0"; case PLUS_INF -> "Infinity"; case MINUS_INF -> "-Infinity"; default -> "NaN"; };
        match self.toDecimal(v){0=>self.charsToString(),1=>"0.0".into(),2=>"-0.0".into(),3=>"Infinity".into(),4=>"-Infinity".into(),_=>"NaN".into()}
    }
    // java private int toDecimal(float v) {
    fn toDecimal(&mut self,v:f32)->i32{
        // java int bits = floatToRawIntBits(v); int t = bits & T_MASK; int bq = (bits >>> P - 1) & BQ_MASK;
        let bits=v.to_bits();let t=(bits&0x7fffff)as i32;let bq=((bits>>23)&255)as i32;
        // java if (bq < BQ_MASK) {
        if bq<255{
            // java index = -1; if (bits < 0) { append('-'); }
            self.index=-1;if (bits as i32)<0{self.append(45);}
            // java if (bq != 0) {
            if bq!=0{
                // java int mq = -Q_MIN + 1 - bq; int c = C_MIN | t;
                let mq=-Q_MIN+1-bq;let c=C_MIN|t;
                // java if (0 < mq & mq < P) { int f = c >> mq; if (f << mq == c) { return toChars(f, 0); } }
                if 0<mq&&mq<P{let f=c>>mq;if f<<mq==c{return self.toChars(f,0);}}
                // java return toDecimal(-mq, c, 0);
                return self.toDecimal__q(-mq,c,0);
            }
            // java if (t != 0) { return t < C_TINY ? toDecimal(Q_MIN, 10 * t, -1) : toDecimal(Q_MIN, t, 0); }
            if t!=0{return if t<8{self.toDecimal__q(Q_MIN,10*t,-1)}else{self.toDecimal__q(Q_MIN,t,0)};}
            // java return bits == 0 ? PLUS_ZERO : MINUS_ZERO;
            return if bits==0{1}else{2};
        }
        // java if (t != 0) { return NAN; } return bits > 0 ? PLUS_INF : MINUS_INF;
        if t!=0{5}else if bits as i32>0{3}else{4}
    }
    // java private int toDecimal(int q, int c, int dk) {
    fn toDecimal__q(&mut self,q:i32,c:i32,dk:i32)->i32{
        // java int out = c & 0x1; long cb = c << 2; long cbr = cb + 2; long cbl; int k;
        let out=c&1;let cb=(c<<2)as i64;let cbr=cb+2;let(cbl,k);
        // java if (c != C_MIN | q == Q_MIN) { cbl = cb - 2; k = flog10pow2(q); } else { cbl = cb - 1; k = flog10threeQuartersPow2(q); }
        if c!=C_MIN||q==Q_MIN{cbl=cb-2;k=flog10pow2(q);}else{cbl=cb-1;k=flog10threeQuartersPow2(q);}
        // java int h = q + flog2pow10(-k) + 33; long g = g1(k) + 1;
        let h=q+flog2pow10(-k)+33;let g=g1(k)+1;
        // java int vb = rop(g, cb << h); int vbl = rop(g, cbl << h); int vbr = rop(g, cbr << h);
        let vb=Self::rop(g,cb<<h);let vbl=Self::rop(g,cbl<<h);let vbr=Self::rop(g,cbr<<h);
        // java int s = vb >> 2; if (s >= 100) {
        let s=vb>>2;if s>=100{
            // java int sp10 = 10 * (int)(s * 1_717_986_919L >>> 34); int tp10 = sp10 + 10;
            let sp10=10*((s as i64*1_717_986_919)as u64>>34)as i32;let tp10=sp10+10;
            // java boolean upin = vbl + out <= sp10 << 2; boolean wpin = (tp10 << 2) + out <= vbr;
            let upin=vbl+out<=sp10<<2;let wpin=(tp10<<2)+out<=vbr;
            // java if (upin != wpin) { return toChars(upin ? sp10 : tp10, k); }
            if upin!=wpin{return self.toChars(if upin{sp10}else{tp10},k);}
        }
        // java int t = s + 1; boolean uin = vbl + out <= s << 2; boolean win = (t << 2) + out <= vbr;
        let t=s+1;let uin=vbl+out<=s<<2;let win=(t<<2)+out<=vbr;
        // java if (uin != win) { return toChars(uin ? s : t, k + dk); }
        if uin!=win{return self.toChars(if uin{s}else{t},k+dk);}
        // java int cmp = vb - (s + t << 1); return toChars(cmp < 0 || cmp == 0 && (s & 0x1) == 0 ? s : t, k + dk);
        let cmp=vb-((s+t)<<1);self.toChars(if cmp<0||cmp==0&&s&1==0{s}else{t},k+dk)
    }
    // java private static int rop(long g, long cp) {
    fn rop(g:i64,cp:i64)->i32{
        // java long x1 = multiplyHigh(g, cp); long vbp = x1 >>> 31; return (int)(vbp | (x1 & MASK_32) + MASK_32 >>> 32);
        let x1=multiplyHigh(g,cp);let vbp=(x1 as u64)>>31;(vbp|(((x1&MASK_32)+MASK_32)as u64>>32))as i32
    }
    // java private int toChars(int f, int e) {
    fn toChars(&mut self,mut f:i32,mut e:i32)->i32{
        // java int len = flog10pow2(Integer.SIZE - numberOfLeadingZeros(f)); if (f >= pow10(len)) { len += 1; }
        let mut len=flog10pow2(32-f.leading_zeros()as i32);if f as i64>=pow10(len){len+=1;}
        // java f *= (int)pow10(H - len); e += len;
        f*=pow10(9-len)as i32;e+=len;
        // java int h = (int)(f * 1_441_151_881L >>> 57); int l = f - 100_000_000 * h;
        let h=((f as i64*1_441_151_881)as u64>>57)as i32;let l=f-100_000_000*h;
        // java if (0 < e && e <= 7) { return toChars1(h, l, e); } if (-3 < e && e <= 0) { return toChars2(h, l, e); } return toChars3(h, l, e);
        if 0<e&&e<=7{self.toChars1(h,l,e)}else if -3<e&&e<=0{self.toChars2(h,l,e)}else{self.toChars3(h,l,e)}
    }
    // java private int toChars1(int h, int l, int e) {
    fn toChars1(&mut self,h:i32,l:i32,e:i32)->i32{
        // java appendDigit(h); int y = y(l); int t; int i = 1;
        self.appendDigit(h);let mut y=Self::y(l);let mut i=1;
        // java for (; i < e; ++i) { t = 10 * y; appendDigit(t >>> 28); y = t & MASK_28; }
        while i<e{let t=y.wrapping_mul(10);self.appendDigit(((t as u32)>>28)as i32);y=t&MASK_28;i+=1;}
        // java append('.');
        self.append(46);
        // java for (; i <= 8; ++i) { t = 10 * y; appendDigit(t >>> 28); y = t & MASK_28; }
        while i<=8{let t=y.wrapping_mul(10);self.appendDigit(((t as u32)>>28)as i32);y=t&MASK_28;i+=1;}
        // java removeTrailingZeroes(); return NON_SPECIAL;
        self.removeTrailingZeroes();0
    }
    // java private int toChars2(int h, int l, int e) {
    fn toChars2(&mut self,h:i32,l:i32,mut e:i32)->i32{
        // java appendDigit(0); append('.'); for (; e < 0; ++e) { appendDigit(0); }
        self.appendDigit(0);self.append(46);while e<0{self.appendDigit(0);e+=1;}
        // java appendDigit(h); append8Digits(l); removeTrailingZeroes(); return NON_SPECIAL;
        self.appendDigit(h);self.append8Digits(l);self.removeTrailingZeroes();0
    }
    // java private int toChars3(int h, int l, int e) {
    fn toChars3(&mut self,h:i32,l:i32,e:i32)->i32{
        // java appendDigit(h); append('.'); append8Digits(l); removeTrailingZeroes(); exponent(e - 1); return NON_SPECIAL;
        self.appendDigit(h);self.append(46);self.append8Digits(l);self.removeTrailingZeroes();self.exponent(e-1);0
    }
    // java private void append8Digits(int m) {
    fn append8Digits(&mut self,m:i32){
        // java int y = y(m); for (int i = 0; i < 8; ++i) { int t = 10 * y; appendDigit(t >>> 28); y = t & MASK_28; }
        let mut y=Self::y(m);for _ in 0..8{let t=y.wrapping_mul(10);self.appendDigit(((t as u32)>>28)as i32);y=t&MASK_28;}
    }
    // java private void removeTrailingZeroes() {
    fn removeTrailingZeroes(&mut self){
        // java while (bytes[index] == '0') { --index; } if (bytes[index] == '.') { ++index; }
        while self.bytes[self.index as usize]==48{self.index-=1;}if self.bytes[self.index as usize]==46{self.index+=1;}
    }
    // java private int y(int a) { return (int)(multiplyHigh((long)(a + 1) << 28, 193_428_131_138_340_668L) >>> 20) - 1; }
    fn y(a:i32)->i32{((multiplyHigh(((a+1)as i64)<<28,193_428_131_138_340_668)as u64)>>20)as i32-1}
    // java private void exponent(int e) {
    fn exponent(&mut self,mut e:i32){
        // java append('E'); if (e < 0) { append('-'); e = -e; }
        self.append(69);if e<0{self.append(45);e=-e;}
        // java if (e < 10) { appendDigit(e); return; }
        if e<10{self.appendDigit(e);return;}
        // java int d = e * 103 >>> 10; appendDigit(d); appendDigit(e - 10 * d);
        let d=((e*103)as u32>>10)as i32;self.appendDigit(d);self.appendDigit(e-10*d);
    }
    // java private void append(int c) { bytes[++index] = (byte)c; }
    fn append(&mut self,c:i32){self.index+=1;self.bytes[self.index as usize]=c as u8;}
    // java private void appendDigit(int d) { bytes[++index] = (byte)('0' + d); }
    fn appendDigit(&mut self,d:i32){self.index+=1;self.bytes[self.index as usize]=(48+d)as u8;}
    // java private String charsToString() { return new String(bytes, 0, 0, index + 1); }
    fn charsToString(&self)->String{String::from_utf8(self.bytes[..=self.index as usize].to_vec()).unwrap()}
}
// java MathUtils.flog10pow2: return (int)(e * C_10 >> Q_10);
fn flog10pow2(e:i32)->i32{(e as i64*661_971_961_083>>41)as i32}
// java MathUtils.flog10threeQuartersPow2: return (int)(e * C_10 + A_10 >> Q_10);
fn flog10threeQuartersPow2(e:i32)->i32{((e as i64*661_971_961_083-274_743_187_321)>>41)as i32}
// java MathUtils.flog2pow10: return (int)(e * C_2 >> Q_2);
fn flog2pow10(e:i32)->i32{(e as i64*913_124_641_741>>38)as i32}
// java MathUtils.pow10: return pow10[e];
fn pow10(e:i32)->i64{const POW:[i64;10]=[1,10,100,1000,10000,100000,1000000,10000000,100000000,1000000000];POW[e as usize]}
// java MathUtils.g1: return g[k - K_MIN << 1];
fn g1(k:i32)->i64{let i=(k+45)as usize*8;i64::from_le_bytes(include_bytes!("float-g1.bin")[i..i+8].try_into().unwrap())}
// java Math.multiplyHigh(long x, long y): high signed 64 bits of the 128-bit product.
fn multiplyHigh(x:i64,y:i64)->i64{(x as i128*y as i128>>64)as i64}
