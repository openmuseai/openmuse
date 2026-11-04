#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;

// Java field: long SIGN_MASK
pub(crate) const SIGN_MASK: i64 = 0x8000000000000000u64 as i64;
// Java field: long EXPONENT_MASK
pub(crate) const EXPONENT_MASK: i64 = 0x7ff0000000000000u64 as i64;
// Java field: long MANTISSA_MASK
pub(crate) const MANTISSA_MASK: i64 = 0x000fffffffffffffu64 as i64;
// Java field: long IMPLICIT_ONE
pub(crate) const IMPLICIT_ONE: i64 = 0x0010000000000000u64 as i64;

// Java: public class implements implements an ordering operation for T
pub struct implements {
}

// Java: public class is
pub struct is {
}

// Java: public class OrderedTuple
pub struct OrderedTuple {
    // Java field: double[] components
    pub(crate) components: Vec<f64>,
    // Java field: int offset
    pub(crate) offset: i32,
    // Java field: int lsb
    pub(crate) lsb: i32,
    // Java field: long[] encoding
    pub(crate) encoding: Vec<i64>,
    // Java field: boolean posInf
    pub(crate) posInf: bool,
    // Java field: boolean negInf
    pub(crate) negInf: bool,
    // Java field: boolean nan
    pub(crate) nan: bool,
}

// Java: public class implements implements an ordering operation for T
impl implements {
}

// Java: public class is
impl is {
}

// Java: public class OrderedTuple
impl OrderedTuple {
    // Java: OrderedTuple(final double ... components)
    pub fn new(components: Vec<f64>) -> Self {
        // Java: this.components = components.clone();
        let mut msb: i32 = i32::MIN;
        // Java: int msb = Integer.MIN_VALUE;
        let mut lsb: i32 = i32::MAX;
        // Java: lsb     = Integer.MAX_VALUE;
        let mut pos_inf = false;
        // Java: posInf  = false;
        let mut neg_inf = false;
        // Java: negInf  = false;
        let mut nan = false;
        // Java: nan     = false;
        // Java: for (int i = 0; i < components.length; ++i)
        for i in 0..components.len() {
            // Java: if (Double.isInfinite(components[i]))
            if components[i].is_infinite() {
                // Java: if (components[i] < 0)
                if components[i] < 0.0 {
                    // Java: negInf = true;
                    neg_inf = true;
                }
                // Java: else
                else {
                    // Java: posInf = true;
                    pos_inf = true;
                }
            }
            // Java: else if (Double.isNaN(components[i]))
            else if components[i].is_nan() {
                // Java: nan = true;
                nan = true;
            }
            // Java: else
            else {
                // Java: final long b = Double.doubleToLongBits(components[i]);
                let b = components[i].to_bits() as i64;
                // Java: final long m = mantissa(b);
                let m = Self::mantissa_static(b);
                // Java: if (m != 0)
                if m != 0 {
                    // Java: final int e = exponent(b);
                    let e = Self::exponent_static(b);
                    // Java: msb = FastMath.max(msb, e + computeMSB(m));
                    msb = msb.max(e + Self::computeMSB_static(m));
                    // Java: lsb = FastMath.min(lsb, e + computeLSB(m));
                    lsb = lsb.min(e + Self::computeLSB_static(m));
                }
            }
        }
        // Java: if (posInf && negInf)
        if pos_inf && neg_inf {
            // Java: // instance cannot be sorted logically
            // Java: posInf = false;
            pos_inf = false;
            // Java: negInf = false;
            neg_inf = false;
            // Java: nan    = true;
            nan = true;
        }

        let mut result = OrderedTuple {
            components,
            offset: 0,
            lsb,
            encoding: vec![0i64],
            posInf: pos_inf,
            negInf: neg_inf,
            nan,
        };

        // Java: if (lsb <= msb)
        if lsb <= msb {
            // Java: // encode the T-upple with the specified offset
            // Java: encode(msb + 16);
            result.encode(msb + 16);
        }
        // Java: else
        else {
            // Java: encoding = new long[]
            // Java: 0x0L
            result.encoding = vec![0i64];
        }

        result
    }

    // Java: void encode(final int minOffset)
    pub fn encode(&mut self, min_offset: i32) {
        // Java: // choose an offset with some margins
        // Java: offset  = minOffset + 31;
        self.offset = min_offset + 31;
        // Java: offset -= offset % 32;
        self.offset -= self.offset % 32;
        // Java: if ((encoding != null) && (encoding.length == 1) && (encoding[0] == 0x0L))
        if self.encoding.len() == 1 && self.encoding[0] == 0i64 {
            // Java: // the components are all zeroes
            // Java: return;
            return;
        }
        // Java: // allocate an integer array to encode the components (we use only
        // Java: // 63 bits per element because there is no unsigned long in Java)
        // Java: final int neededBits  = offset + 1 - lsb;
        let needed_bits = self.offset + 1 - self.lsb;
        // Java: final int neededLongs = (neededBits + 62) / 63;
        let needed_longs = (needed_bits + 62) / 63;
        // Java: encoding = new long[components.length * neededLongs];
        self.encoding = vec![0i64; (self.components.len() as i32 * needed_longs) as usize];
        // Java: // mix the bits from all components
        // Java: int  eIndex = 0;
        let mut e_index: usize = 0;
        // Java: int  shift  = 62;
        let mut shift: i32 = 62;
        // Java: long word   = 0x0L;
        let mut word: i64 = 0i64;
        // Java: for (int k = offset; eIndex < encoding.length; --k)
        let mut k = self.offset;
        while e_index < self.encoding.len() {
            // Java: for (int vIndex = 0; vIndex < components.length; ++vIndex)
            for v_index in 0..self.components.len() {
                // Java: if (getBit(vIndex, k) != 0)
                if self.getBit(v_index as i32, k) != 0 {
                    // Java: word |= 0x1L << shift;
                    word |= 1i64 << shift;
                }
                // Java: if (shift-- == 0)
                if shift == 0 {
                    // Java: encoding[eIndex++] = word;
                    self.encoding[e_index] = word;
                    e_index += 1;
                    // Java: word  = 0x0L;
                    word = 0i64;
                    // Java: shift = 62;
                    shift = 62;
                } else {
                    shift -= 1;
                }
            }
            k -= 1;
        }
    }

    // Java: int compareTo(final OrderedTuple ot)
    pub fn compareTo(&self, ot: &OrderedTuple) -> i32 {
        // Java: if (components.length == ot.components.length)
        if self.components.len() == ot.components.len() {
            // Java: if (nan)
            if self.nan {
                // Java: return +1;
                return 1;
            }
            // Java: else if (ot.nan)
            else if ot.nan {
                // Java: return -1;
                return -1;
            }
            // Java: else if (negInf || ot.posInf)
            else if self.negInf || ot.posInf {
                // Java: return -1;
                return -1;
            }
            // Java: else if (posInf || ot.negInf)
            else if self.posInf || ot.negInf {
                // Java: return +1;
                return 1;
            }
            // Java: else
            else {
                // Java: if (offset < ot.offset)
                // Java: encode(ot.offset);
                // Java: else if (offset > ot.offset)
                // Java: ot.encode(offset);
                // Java: final int limit = FastMath.min(encoding.length, ot.encoding.length);
                let limit = self.encoding.len().min(ot.encoding.len());
                // Java: for (int i = 0; i < limit; ++i)
                for i in 0..limit {
                    // Java: if (encoding[i] < ot.encoding[i])
                    if self.encoding[i] < ot.encoding[i] {
                        // Java: return -1;
                        return -1;
                    }
                    // Java: else if (encoding[i] > ot.encoding[i])
                    else if self.encoding[i] > ot.encoding[i] {
                        // Java: return +1;
                        return 1;
                    }
                }
                // Java: if (encoding.length < ot.encoding.length)
                if self.encoding.len() < ot.encoding.len() {
                    // Java: return -1;
                    return -1;
                }
                // Java: else if (encoding.length > ot.encoding.length)
                else if self.encoding.len() > ot.encoding.len() {
                    // Java: return +1;
                    return 1;
                }
                // Java: else
                else {
                    // Java: return 0;
                    return 0;
                }
            }
        }
        // Java: return components.length - ot.components.length;
        self.components.len() as i32 - ot.components.len() as i32
    }

    // Java: boolean equals(final Object other)
    pub fn equals_obj(&self, other: &OrderedTuple) -> bool {
        // Java: if (this == other)
        // Java: return true;
        // Java: else if (other instanceof OrderedTuple)
        // Java: return compareTo((OrderedTuple) other) == 0;
        // Java: else
        // Java: return false;
        self.compareTo(other) == 0
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: // the following constants are arbitrary small primes
        // Java: final int multiplier = 37;
        let multiplier: i32 = 37;
        // Java: final int trueHash   = 97;
        let true_hash: i32 = 97;
        // Java: final int falseHash  = 71;
        let false_hash: i32 = 71;
        // Java: // hash fields and combine them
        // Java: // (we rely on the multiplier to have different combined weights
        // Java: //  for all int fields and all boolean fields)
        // Java: int hash = Arrays.hashCode(components);
        let mut hash: i32 = 0;
        for c in &self.components {
            hash = hash.wrapping_mul(31).wrapping_add(c.to_bits() as i32);
        }
        // Java: hash = hash * multiplier + offset;
        hash = hash.wrapping_mul(multiplier).wrapping_add(self.offset);
        // Java: hash = hash * multiplier + lsb;
        hash = hash.wrapping_mul(multiplier).wrapping_add(self.lsb);
        // Java: hash = hash * multiplier + (posInf ? trueHash : falseHash);
        hash = hash.wrapping_mul(multiplier).wrapping_add(if self.posInf { true_hash } else { false_hash });
        // Java: hash = hash * multiplier + (negInf ? trueHash : falseHash);
        hash = hash.wrapping_mul(multiplier).wrapping_add(if self.negInf { true_hash } else { false_hash });
        // Java: hash = hash * multiplier + (nan    ? trueHash : falseHash);
        hash = hash.wrapping_mul(multiplier).wrapping_add(if self.nan { true_hash } else { false_hash });
        // Java: return hash;
        hash
    }

    // Java: double[] getComponents()
    pub fn getComponents(&self) -> Vec<f64> {
        // Java: return components.clone();
        self.components.clone()
    }

    // Java: static long sign(final long bits)
    pub fn sign(bits: i64) -> i64 {
        // Java: return bits & SIGN_MASK;
        bits & SIGN_MASK
    }

    // Java: static int exponent(final long bits)
    pub fn exponent(&self, bits: i64) -> i32 {
        // Java: return ((int) ((bits & EXPONENT_MASK) >> 52)) - 1075;
        Self::exponent_static(bits)
    }

    fn exponent_static(bits: i64) -> i32 {
        (((bits & EXPONENT_MASK) >> 52) as i32) - 1075
    }

    // Java: static long mantissa(final long bits)
    pub fn mantissa(&self, bits: i64) -> i64 {
        // Java: return ((bits & EXPONENT_MASK) == 0) ?
        // Java: ((bits & MANTISSA_MASK) << 1) :          // subnormal number
        // Java: (IMPLICIT_ONE | (bits & MANTISSA_MASK)); // normal number
        Self::mantissa_static(bits)
    }

    fn mantissa_static(bits: i64) -> i64 {
        if (bits & EXPONENT_MASK) == 0 {
            (bits & MANTISSA_MASK) << 1
        } else {
            IMPLICIT_ONE | (bits & MANTISSA_MASK)
        }
    }

    // Java: static int computeMSB(final long l)
    pub fn computeMSB(&self, l: i64) -> i32 {
        // Java: long ll = l;
        Self::computeMSB_static(l)
    }

    fn computeMSB_static(l: i64) -> i32 {
        let mut ll = l;
        // Java: long mask  = 0xffffffffL;
        let mut mask: i64 = 0xffffffff;
        // Java: int  scale = 32;
        let mut scale: i32 = 32;
        // Java: int  msb   = 0;
        let mut msb: i32 = 0;
        // Java: while (scale != 0)
        while scale != 0 {
            // Java: if ((ll & mask) != ll)
            if (ll & mask) != ll {
                // Java: msb |= scale;
                msb |= scale;
                // Java: ll >>= scale;
                ll >>= scale;
            }
            // Java: scale >>= 1;
            scale >>= 1;
            // Java: mask >>= scale;
            mask >>= scale;
        }
        // Java: return msb;
        msb
    }

    // Java: static int computeLSB(final long l)
    pub fn computeLSB(&self, l: i64) -> i32 {
        Self::computeLSB_static(l)
    }

    fn computeLSB_static(l: i64) -> i32 {
        // Java: long ll = l;
        let mut ll = l;
        // Java: long mask  = 0xffffffff00000000L;
        let mut mask: i64 = -0x100000000i64; // 0xffffffff00000000
        // Java: int  scale = 32;
        let mut scale: i32 = 32;
        // Java: int  lsb   = 0;
        let mut lsb: i32 = 0;
        // Java: while (scale != 0)
        while scale != 0 {
            // Java: if ((ll & mask) == ll)
            if (ll & mask) == ll {
                // Java: lsb |= scale;
                lsb |= scale;
                // Java: ll >>= scale;
                ll >>= scale;
            }
            // Java: scale >>= 1;
            scale >>= 1;
            // Java: mask >>= scale;
            mask >>= scale;
        }
        // Java: return lsb;
        lsb
    }

    // Java: int getBit(final int i, final int k)
    pub fn getBit(&self, i: i32, k: i32) -> i32 {
        // Java: final long bits = Double.doubleToLongBits(components[i]);
        let bits = self.components[i as usize].to_bits() as i64;
        // Java: final int e = exponent(bits);
        let e = Self::exponent_static(bits);
        // Java: if ((k < e) || (k > offset))
        if k < e || k > self.offset {
            // Java: return 0;
            return 0;
        }
        // Java: else if (k == offset)
        else if k == self.offset {
            // Java: return (sign(bits) == 0L) ? 1 : 0;
            return if Self::sign(bits) == 0 { 1 } else { 0 };
        }
        // Java: else if (k > (e + 52))
        else if k > (e + 52) {
            // Java: return (sign(bits) == 0L) ? 0 : 1;
            return if Self::sign(bits) == 0 { 0 } else { 1 };
        }
        // Java: else
        else {
            // Java: final long m = (sign(bits) == 0L) ? mantissa(bits) : -mantissa(bits);
            let m = if Self::sign(bits) == 0 { Self::mantissa_static(bits) } else { -Self::mantissa_static(bits) };
            // Java: return (int) ((m >> (k - e)) & 0x1L);
            return ((m >> (k - e)) & 0x1) as i32;
        }
    }

}
