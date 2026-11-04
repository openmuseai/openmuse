// Java: import org.apache.commons.math3.Field
use crate::org::apache::commons::math3::Field::Field;
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement::FieldElement;
use super::Dfp::Dfp;
use std::sync::{Arc, Mutex};

// Java field: int FLAG_INVALID
pub(crate) const FLAG_INVALID: i32 = 1;
// Java field: int FLAG_DIV_ZERO
pub(crate) const FLAG_DIV_ZERO: i32 = 2;
// Java field: int FLAG_OVERFLOW
pub(crate) const FLAG_OVERFLOW: i32 = 4;
// Java field: int FLAG_UNDERFLOW
pub(crate) const FLAG_UNDERFLOW: i32 = 8;
// Java field: int FLAG_INEXACT
pub(crate) const FLAG_INEXACT: i32 = 16;
// Java field: String sqr2String
static sqr2String: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String sqr2ReciprocalString
static sqr2ReciprocalString: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String sqr3String
static sqr3String: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String sqr3ReciprocalString
static sqr3ReciprocalString: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String piString
static piString: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String eString
static eString: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String ln2String
static ln2String: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String ln5String
static ln5String: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();
// Java field: String ln10String
static ln10String: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();

// Java: public class DfpField implements Field
pub struct DfpField {
    // Java field: int radixDigits
    pub(crate) radixDigits: i32,
    // Java field: Dfp zero
    pub(crate) zero: Arc<Dfp>,
    // Java field: Dfp one
    pub(crate) one: Arc<Dfp>,
    // Java field: Dfp two
    pub(crate) two: Arc<Dfp>,
    // Java field: Dfp sqr2
    pub(crate) sqr2: Arc<Dfp>,
    // Java field: Dfp[] sqr2Split
    pub(crate) sqr2Split: Arc<Vec<Dfp>>,
    // Java field: Dfp sqr2Reciprocal
    pub(crate) sqr2Reciprocal: Arc<Dfp>,
    // Java field: Dfp sqr3
    pub(crate) sqr3: Arc<Dfp>,
    // Java field: Dfp sqr3Reciprocal
    pub(crate) sqr3Reciprocal: Arc<Dfp>,
    // Java field: Dfp pi
    pub(crate) pi: Arc<Dfp>,
    // Java field: Dfp[] piSplit
    pub(crate) piSplit: Arc<Vec<Dfp>>,
    // Java field: Dfp e
    pub(crate) e: Arc<Dfp>,
    // Java field: Dfp[] eSplit
    pub(crate) eSplit: Arc<Vec<Dfp>>,
    // Java field: Dfp ln2
    pub(crate) ln2: Arc<Dfp>,
    // Java field: Dfp[] ln2Split
    pub(crate) ln2Split: Arc<Vec<Dfp>>,
    // Java field: Dfp ln5
    pub(crate) ln5: Arc<Dfp>,
    // Java field: Dfp[] ln5Split
    pub(crate) ln5Split: Arc<Vec<Dfp>>,
    // Java field: Dfp ln10
    pub(crate) ln10: Arc<Dfp>,
    // Java field: RoundingMode rMode
    pub(crate) rMode: Arc<Mutex<RoundingMode>>,
    // Java field: int ieeeFlags
    pub(crate) ieeeFlags: i32,
}

// Java: public enum RoundingMode
pub struct RoundingMode {
}

// Java: public class DfpField implements Field
impl DfpField {
    // Java: DfpField(final int decimalDigits)
    pub fn DfpField(&self, decimalDigits: i32) {
        // Java: this(decimalDigits, true);
    }

    // Java: DfpField(final int decimalDigits, final boolean computeConstants)
    pub fn DfpField_1(&self, decimalDigits: i32, computeConstants: bool) {
        // Java: this.radixDigits = (decimalDigits < 13) ? 4 : (decimalDigits + 3) / 4;
        // Java: this.rMode       = RoundingMode.ROUND_HALF_EVEN;
        // Java: this.ieeeFlags   = 0;
        // Java: this.zero        = new Dfp(this, 0);
        // Java: this.one         = new Dfp(this, 1);
        // Java: this.two         = new Dfp(this, 2);
        // Java: if (computeConstants)
        {
            // Java: // set up transcendental constants
            // Java: synchronized (DfpField.class)
            {
                // Java: // as a heuristic to circumvent Table-Maker's Dilemma, we set the string
                // Java: // representation of the constants to be at least 3 times larger than the
                // Java: // number of decimal digits, also as an attempt to really compute these
                // Java: // constants only once, we set a minimum number of digits
                // Java: computeStringConstants((decimalDigits < 67) ? 200 : (3 * decimalDigits));
                // Java: // set up the constants at current field accuracy
                // Java: sqr2           = new Dfp(this, sqr2String);
                // Java: sqr2Split      = split(sqr2String);
                // Java: sqr2Reciprocal = new Dfp(this, sqr2ReciprocalString);
                // Java: sqr3           = new Dfp(this, sqr3String);
                // Java: sqr3Reciprocal = new Dfp(this, sqr3ReciprocalString);
                // Java: pi             = new Dfp(this, piString);
                // Java: piSplit        = split(piString);
                // Java: e              = new Dfp(this, eString);
                // Java: eSplit         = split(eString);
                // Java: ln2            = new Dfp(this, ln2String);
                // Java: ln2Split       = split(ln2String);
                // Java: ln5            = new Dfp(this, ln5String);
                // Java: ln5Split       = split(ln5String);
                // Java: ln10           = new Dfp(this, ln10String);
            }
        }
        // Java: else
        {
            // Java: // dummy settings for unused constants
            // Java: sqr2           = null;
            // Java: sqr2Split      = null;
            // Java: sqr2Reciprocal = null;
            // Java: sqr3           = null;
            // Java: sqr3Reciprocal = null;
            // Java: pi             = null;
            // Java: piSplit        = null;
            // Java: e              = null;
            // Java: eSplit         = null;
            // Java: ln2            = null;
            // Java: ln2Split       = null;
            // Java: ln5            = null;
            // Java: ln5Split       = null;
            // Java: ln10           = null;
        }
    }

    // Java: int getRadixDigits()
    pub fn getRadixDigits(&self) -> i32 {
        // Java: return radixDigits;
        todo!();
    }

    // Java: void setRoundingMode(final RoundingMode mode)
    pub fn setRoundingMode(&self, mode: &RoundingMode) {
        // Java: rMode = mode;
    }

    // Java: RoundingMode getRoundingMode()
    pub fn getRoundingMode(&self) -> RoundingMode {
        // Java: return rMode;
        todo!();
    }

    // Java: int getIEEEFlags()
    pub fn getIEEEFlags(&self) -> i32 {
        // Java: return ieeeFlags;
        todo!();
    }

    // Java: void clearIEEEFlags()
    pub fn clearIEEEFlags(&self) {
        // Java: ieeeFlags = 0;
    }

    // Java: void setIEEEFlags(final int flags)
    pub fn setIEEEFlags(&self, flags: i32) {
        // Java: ieeeFlags = flags & (FLAG_INVALID | FLAG_DIV_ZERO | FLAG_OVERFLOW | FLAG_UNDERFLOW | FLAG_INEXACT);
    }

    // Java: void setIEEEFlagsBits(final int bits)
    pub fn setIEEEFlagsBits(&self, bits: i32) {
        // Java: ieeeFlags |= bits & (FLAG_INVALID | FLAG_DIV_ZERO | FLAG_OVERFLOW | FLAG_UNDERFLOW | FLAG_INEXACT);
    }

    // Java: Dfp newDfp()
    pub fn newDfp(&self) -> Dfp {
        // Java: return new Dfp(this);
        todo!();
    }

    // Java: Dfp newDfp(final byte x)
    pub fn newDfp_1(&self, x: i8) -> Dfp {
        // Java: return new Dfp(this, x);
        todo!();
    }

    // Java: Dfp newDfp(final int x)
    pub fn newDfp_2(&self, x: i32) -> Dfp {
        // Java: return new Dfp(this, x);
        todo!();
    }

    // Java: Dfp newDfp(final long x)
    pub fn newDfp_3(&self, x: i64) -> Dfp {
        // Java: return new Dfp(this, x);
        todo!();
    }

    // Java: Dfp newDfp(final double x)
    pub fn newDfp_4(&self, x: f64) -> Dfp {
        // Java: return new Dfp(this, x);
        todo!();
    }

    // Java: Dfp newDfp(Dfp d)
    pub fn newDfp_5(&self, d: &Dfp) -> Dfp {
        // Java: return new Dfp(d);
        todo!();
    }

    // Java: Dfp newDfp(final String s)
    pub fn newDfp_6(&self, s: &str) -> Dfp {
        // Java: return new Dfp(this, s);
        todo!();
    }

    // Java: Dfp newDfp(final byte sign, final byte nans)
    pub fn newDfp_7(&self, sign: i8, nans: i8) -> Dfp {
        // Java: return new Dfp(this, sign, nans);
        todo!();
    }

    // Java: Dfp getZero()
    pub fn getZero(&self) -> Dfp {
        // Java: return zero;
        todo!();
    }

    // Java: Dfp getOne()
    pub fn getOne(&self) -> Dfp {
        // Java: return one;
        todo!();
    }

    // Java: public Class<? extends FieldElement<Dfp>> getRuntimeClass()
    pub fn getRuntimeClass(&self) -> std::any::TypeId {
        // Java: return Dfp.class;
        todo!();
    }

    // Java: Dfp getTwo()
    pub fn getTwo(&self) -> Dfp {
        // Java: return two;
        todo!();
    }

    // Java: Dfp getSqr2()
    pub fn getSqr2(&self) -> Dfp {
        // Java: return sqr2;
        todo!();
    }

    // Java: Dfp[] getSqr2Split()
    pub fn getSqr2Split(&self) -> Vec<Dfp> {
        // Java: return sqr2Split.clone();
        todo!();
    }

    // Java: Dfp getSqr2Reciprocal()
    pub fn getSqr2Reciprocal(&self) -> Dfp {
        // Java: return sqr2Reciprocal;
        todo!();
    }

    // Java: Dfp getSqr3()
    pub fn getSqr3(&self) -> Dfp {
        // Java: return sqr3;
        todo!();
    }

    // Java: Dfp getSqr3Reciprocal()
    pub fn getSqr3Reciprocal(&self) -> Dfp {
        // Java: return sqr3Reciprocal;
        todo!();
    }

    // Java: Dfp getPi()
    pub fn getPi(&self) -> Dfp {
        // Java: return pi;
        todo!();
    }

    // Java: Dfp[] getPiSplit()
    pub fn getPiSplit(&self) -> Vec<Dfp> {
        // Java: return piSplit.clone();
        todo!();
    }

    // Java: Dfp getE()
    pub fn getE(&self) -> Dfp {
        // Java: return e;
        todo!();
    }

    // Java: Dfp[] getESplit()
    pub fn getESplit(&self) -> Vec<Dfp> {
        // Java: return eSplit.clone();
        todo!();
    }

    // Java: Dfp getLn2()
    pub fn getLn2(&self) -> Dfp {
        // Java: return ln2;
        todo!();
    }

    // Java: Dfp[] getLn2Split()
    pub fn getLn2Split(&self) -> Vec<Dfp> {
        // Java: return ln2Split.clone();
        todo!();
    }

    // Java: Dfp getLn5()
    pub fn getLn5(&self) -> Dfp {
        // Java: return ln5;
        todo!();
    }

    // Java: Dfp[] getLn5Split()
    pub fn getLn5Split(&self) -> Vec<Dfp> {
        // Java: return ln5Split.clone();
        todo!();
    }

    // Java: Dfp getLn10()
    pub fn getLn10(&self) -> Dfp {
        // Java: return ln10;
        todo!();
    }

    // Java: Dfp[] split(final String a)
    pub fn split(&self, a: &str) -> Vec<Dfp> {
      // Java: Dfp result[] = new Dfp[2];
      // Java: boolean leading = true;
      // Java: int sp = 0;
      // Java: int sig = 0;
      // Java: char[] buf = new char[a.length()];
      // Java: for (int i = 0; i < buf.length; i++)
      {
        // Java: buf[i] = a.charAt(i);
        // Java: if (buf[i] >= '1' && buf[i] <= '9')
        {
            // Java: leading = false;
        }
        // Java: if (buf[i] == '.')
        {
          // Java: sig += (400 - sig) % 4;
          // Java: leading = false;
        }
        // Java: if (sig == (radixDigits / 2) * 4)
        {
          // Java: sp = i;
          // Java: break;
        }
        // Java: if (buf[i] >= '0' && buf[i] <= '9' && !leading)
        {
            // Java: sig ++;
        }
      }
      // Java: result[0] = new Dfp(this, new String(buf, 0, sp));
      // Java: for (int i = 0; i < buf.length; i++)
      {
        // Java: buf[i] = a.charAt(i);
        // Java: if (buf[i] >= '0' && buf[i] <= '9' && i < sp)
        {
            // Java: buf[i] = '0';
        }
      }
      // Java: result[1] = new Dfp(this, new String(buf));
      // Java: return result;
        todo!();
    }

    // Java: static void computeStringConstants(final int highPrecisionDecimalDigits)
    pub fn computeStringConstants(&self, highPrecisionDecimalDigits: i32) {
        // Java: if (sqr2String == null || sqr2String.length() < highPrecisionDecimalDigits - 3)
        {
            // Java: // recompute the string representation of the transcendental constants
            // Java: final DfpField highPrecisionField = new DfpField(highPrecisionDecimalDigits, false);
            // Java: final Dfp highPrecisionOne        = new Dfp(highPrecisionField, 1);
            // Java: final Dfp highPrecisionTwo        = new Dfp(highPrecisionField, 2);
            // Java: final Dfp highPrecisionThree      = new Dfp(highPrecisionField, 3);
            // Java: final Dfp highPrecisionSqr2 = highPrecisionTwo.sqrt();
            // Java: sqr2String           = highPrecisionSqr2.toString();
            // Java: sqr2ReciprocalString = highPrecisionOne.divide(highPrecisionSqr2).toString();
            // Java: final Dfp highPrecisionSqr3 = highPrecisionThree.sqrt();
            // Java: sqr3String           = highPrecisionSqr3.toString();
            // Java: sqr3ReciprocalString = highPrecisionOne.divide(highPrecisionSqr3).toString();
            // Java: piString   = computePi(highPrecisionOne, highPrecisionTwo, highPrecisionThree).toString();
            // Java: eString    = computeExp(highPrecisionOne, highPrecisionOne).toString();
            // Java: ln2String  = computeLn(highPrecisionTwo, highPrecisionOne, highPrecisionTwo).toString();
            // Java: ln5String  = computeLn(new Dfp(highPrecisionField, 5),  highPrecisionOne, highPrecisionTwo).toString();
            // Java: ln10String = computeLn(new Dfp(highPrecisionField, 10), highPrecisionOne, highPrecisionTwo).toString();
        }
    }

    // Java: static Dfp computePi(final Dfp one, final Dfp two, final Dfp three)
    pub fn computePi(&self, one: &Dfp, two: &Dfp, three: &Dfp) -> Dfp {
        // Java: Dfp sqrt2   = two.sqrt();
        // Java: Dfp yk      = sqrt2.subtract(one);
        // Java: Dfp four    = two.add(two);
        // Java: Dfp two2kp3 = two;
        // Java: Dfp ak      = two.multiply(three.subtract(two.multiply(sqrt2)));
        // Java: // The formula converges quartically. This means the number of correct
        // Java: // digits is multiplied by 4 at each iteration! Five iterations are
        // Java: // sufficient for about 160 digits, eight iterations give about
        // Java: // 10000 digits (this has been checked) and 20 iterations more than
        // Java: // 160 billions of digits (this has NOT been checked).
        // Java: // So the limit here is considered sufficient for most purposes ...
        // Java: for (int i = 1; i < 20; i++)
        {
            // Java: final Dfp ykM1 = yk;
            // Java: final Dfp y2         = yk.multiply(yk);
            // Java: final Dfp oneMinusY4 = one.subtract(y2.multiply(y2));
            // Java: final Dfp s          = oneMinusY4.sqrt().sqrt();
            // Java: yk = one.subtract(s).divide(one.add(s));
            // Java: two2kp3 = two2kp3.multiply(four);
            // Java: final Dfp p = one.add(yk);
            // Java: final Dfp p2 = p.multiply(p);
            // Java: ak = ak.multiply(p2.multiply(p2)).subtract(two2kp3.multiply(yk).multiply(one.add(yk).add(yk.multiply(yk))));
            // Java: if (yk.equals(ykM1))
            {
                // Java: break;
            }
        }
        // Java: return one.divide(ak);
        todo!();
    }

    // Java: static Dfp computeExp(final Dfp a, final Dfp one)
    pub fn computeExp(&self, a: &Dfp, one: &Dfp) -> Dfp {
        // Java: Dfp y  = new Dfp(one);
        // Java: Dfp py = new Dfp(one);
        // Java: Dfp f  = new Dfp(one);
        // Java: Dfp fi = new Dfp(one);
        // Java: Dfp x  = new Dfp(one);
        // Java: for (int i = 0; i < 10000; i++)
        {
            // Java: x = x.multiply(a);
            // Java: y = y.add(x.divide(f));
            // Java: fi = fi.add(one);
            // Java: f = f.multiply(fi);
            // Java: if (y.equals(py))
            {
                // Java: break;
            }
            // Java: py = new Dfp(y);
        }
        // Java: return y;
        todo!();
    }

    // Java: static Dfp computeLn(final Dfp a, final Dfp one, final Dfp two)
    pub fn computeLn(&self, a: &Dfp, one: &Dfp, two: &Dfp) -> Dfp {
        // Java: int den = 1;
        // Java: Dfp x = a.add(new Dfp(a.getField(), -1)).divide(a.add(one));
        // Java: Dfp y = new Dfp(x);
        // Java: Dfp num = new Dfp(x);
        // Java: Dfp py = new Dfp(y);
        // Java: for (int i = 0; i < 10000; i++)
        {
            // Java: num = num.multiply(x);
            // Java: num = num.multiply(x);
            // Java: den += 2;
            // Java: Dfp t = num.divide(den);
            // Java: y = y.add(t);
            // Java: if (y.equals(py))
            {
                // Java: break;
            }
            // Java: py = new Dfp(y);
        }
        // Java: return y.multiply(two);
        todo!();
    }

}

// Java: public enum RoundingMode
impl RoundingMode {
}

impl Field for DfpField {
    type T = Dfp;

    fn getZero(&self) -> Self::T {
        todo!();
    }

    fn getOne(&self) -> Self::T {
        todo!();
    }

    fn getRuntimeClass(&self) -> std::any::TypeId {
        todo!();
    }
}
