// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

#[derive(Clone, Debug)]
pub struct DfpField;

// Java field: int RADIX
pub(crate) const RADIX: i32 = 10000;
// Java field: int MIN_EXP
pub(crate) const MIN_EXP: i32 = -32767;
// Java field: int MAX_EXP
pub(crate) const MAX_EXP: i32 = 32768;
// Java field: int ERR_SCALE
pub(crate) const ERR_SCALE: i32 = 32760;
// Java field: byte FINITE
pub(crate) const FINITE: i8 = 0;
// Java field: byte INFINITE
pub(crate) const INFINITE: i8 = 1;
// Java field: byte SNAN
pub(crate) const SNAN: i8 = 2;
// Java field: byte QNAN
pub(crate) const QNAN: i8 = 3;
// Java field: String NAN_STRING
pub(crate) const NAN_STRING: &str = "NaN";
// Java field: String POS_INFINITY_STRING
pub(crate) const POS_INFINITY_STRING: &str = "Infinity";
// Java field: String NEG_INFINITY_STRING
pub(crate) const NEG_INFINITY_STRING: &str = "-Infinity";
// Java field: String ADD_TRAP
pub(crate) const ADD_TRAP: &str = "add";
// Java field: String MULTIPLY_TRAP
pub(crate) const MULTIPLY_TRAP: &str = "multiply";
// Java field: String DIVIDE_TRAP
pub(crate) const DIVIDE_TRAP: &str = "divide";
// Java field: String SQRT_TRAP
pub(crate) const SQRT_TRAP: &str = "sqrt";
// Java field: String ALIGN_TRAP
pub(crate) const ALIGN_TRAP: &str = "align";
// Java field: String TRUNC_TRAP
pub(crate) const TRUNC_TRAP: &str = "trunc";
// Java field: String NEXT_AFTER_TRAP
pub(crate) const NEXT_AFTER_TRAP: &str = "nextAfter";
// Java field: String LESS_THAN_TRAP
pub(crate) const LESS_THAN_TRAP: &str = "lessThan";
// Java field: String GREATER_THAN_TRAP
pub(crate) const GREATER_THAN_TRAP: &str = "greaterThan";
// Java field: String NEW_INSTANCE_TRAP
pub(crate) const NEW_INSTANCE_TRAP: &str = "newInstance";

// Java: public class does
pub struct does {
}

// Java: public class Dfp
#[derive(Clone, Debug)]
pub struct Dfp {
    // Java field: int[] mant
    pub(crate) mant: Vec<i32>,
    // Java field: byte sign
    pub(crate) sign: i8,
    // Java field: int exp
    pub(crate) exp: i32,
    // Java field: byte nans
    pub(crate) nans: i8,
    // Java field: DfpField field
    pub(crate) field: Option<DfpField>,
}

// Java: public class does
impl does {
}

// Java: public class Dfp
impl Dfp {
    // Java: Dfp(final DfpField field)
    pub fn Dfp(&self, field: DfpField) {
        // Java: mant = new int[field.getRadixDigits()];
        // Java: sign = 1;
        // Java: exp = 0;
        // Java: nans = FINITE;
        // Java: this.field = field;
    }

    // Java: Dfp(final DfpField field, byte x)
    pub fn Dfp_1(&self, field: DfpField, x: i8) {
        // Java: this(field, (long) x);
    }

    // Java: Dfp(final DfpField field, int x)
    pub fn Dfp_2(&self, field: DfpField, x: i32) {
        // Java: this(field, (long) x);
    }

    // Java: Dfp(final DfpField field, long x)
    pub fn Dfp_3(&self, field: DfpField, x: i64) {
        // Java: // initialize as if 0
        // Java: mant = new int[field.getRadixDigits()];
        // Java: nans = FINITE;
        // Java: this.field = field;
        // Java: boolean isLongMin = false;
        // Java: if (x == Long.MIN_VALUE)
        {
            // Java: // special case for Long.MIN_VALUE (-9223372036854775808)
            // Java: // we must shift it before taking its absolute value
            // Java: isLongMin = true;
            // Java: ++x;
        }
        // Java: // set the sign
        // Java: if (x < 0)
        {
            // Java: sign = -1;
            // Java: x = -x;
        }
        // Java: else
        {
            // Java: sign = 1;
        }
        // Java: exp = 0;
        // Java: while (x != 0)
        {
            // Java: System.arraycopy(mant, mant.length - exp, mant, mant.length - 1 - exp, exp);
            // Java: mant[mant.length - 1] = (int) (x % RADIX);
            // Java: x /= RADIX;
            // Java: exp++;
        }
        // Java: if (isLongMin)
        {
            // Java: // remove the shift added for Long.MIN_VALUE
            // Java: // we know in this case that fixing the last digit is sufficient
            // Java: for (int i = 0; i < mant.length - 1; i++)
            {
                // Java: if (mant[i] != 0)
                {
                    // Java: mant[i]++;
                    // Java: break;
                }
            }
        }
    }

    // Java: Dfp(final DfpField field, double x)
    pub fn Dfp_4(&self, field: DfpField, x: f64) {
        // Java: // initialize as if 0
        // Java: mant = new int[field.getRadixDigits()];
        // Java: sign = 1;
        // Java: exp = 0;
        // Java: nans = FINITE;
        // Java: this.field = field;
        // Java: long bits = Double.doubleToLongBits(x);
        // Java: long mantissa = bits & 0x000fffffffffffffL;
        // Java: int exponent = (int) ((bits & 0x7ff0000000000000L) >> 52) - 1023;
        // Java: if (exponent == -1023)
        {
            // Java: // Zero or sub-normal
            // Java: if (x == 0)
            {
                // Java: // make sure 0 has the right sign
                // Java: if ((bits & 0x8000000000000000L) != 0)
                {
                    // Java: sign = -1;
                }
                // Java: return;
            }
            // Java: exponent++;
            // Java: // Normalize the subnormal number
            // Java: while ( (mantissa & 0x0010000000000000L) == 0)
            {
                // Java: exponent--;
                // Java: mantissa <<= 1;
            }
            // Java: mantissa &= 0x000fffffffffffffL;
        }
        // Java: if (exponent == 1024)
        {
            // Java: // infinity or NAN
            // Java: if (x != x)
            {
                // Java: sign = (byte) 1;
                // Java: nans = QNAN;
            }
            // Java: else if (x < 0)
            {
                // Java: sign = (byte) -1;
                // Java: nans = INFINITE;
            }
            // Java: else
            {
                // Java: sign = (byte) 1;
                // Java: nans = INFINITE;
            }
            // Java: return;
        }
        // Java: Dfp xdfp = new Dfp(field, mantissa);
        // Java: xdfp = xdfp.divide(new Dfp(field, 4503599627370496l)).add(field.getOne());  // Divide by 2^52, then add one
        // Java: xdfp = xdfp.multiply(DfpMath.pow(field.getTwo(), exponent));
        // Java: if ((bits & 0x8000000000000000L) != 0)
        {
            // Java: xdfp = xdfp.negate();
        }
        // Java: System.arraycopy(xdfp.mant, 0, mant, 0, mant.length);
        // Java: sign = xdfp.sign;
        // Java: exp  = xdfp.exp;
        // Java: nans = xdfp.nans;
    }

    // Java: Dfp(final Dfp d)
    pub fn Dfp_5(&self, d: &Dfp) {
        // Java: mant  = d.mant.clone();
        // Java: sign  = d.sign;
        // Java: exp   = d.exp;
        // Java: nans  = d.nans;
        // Java: field = d.field;
    }

    // Java: Dfp(final DfpField field, final String s)
    pub fn Dfp_6(&self, field: DfpField, s: &str) {
        // Java: // initialize as if 0
        // Java: mant = new int[field.getRadixDigits()];
        // Java: sign = 1;
        // Java: exp = 0;
        // Java: nans = FINITE;
        // Java: this.field = field;
        // Java: boolean decimalFound = false;
        // Java: final int rsize = 4;   // size of radix in decimal digits
        // Java: final int offset = 4;  // Starting offset into Striped
        // Java: final char[] striped = new char[getRadixDigits() * rsize + offset * 2];
        // Java: // Check some special cases
        // Java: if (s.equals(POS_INFINITY_STRING))
        {
            // Java: sign = (byte) 1;
            // Java: nans = INFINITE;
            // Java: return;
        }
        // Java: if (s.equals(NEG_INFINITY_STRING))
        {
            // Java: sign = (byte) -1;
            // Java: nans = INFINITE;
            // Java: return;
        }
        // Java: if (s.equals(NAN_STRING))
        {
            // Java: sign = (byte) 1;
            // Java: nans = QNAN;
            // Java: return;
        }
        // Java: // Check for scientific notation
        // Java: int p = s.indexOf("e");
        // Java: if (p == -1) { // try upper case?
            // Java: p = s.indexOf("E");
        // Java: final String fpdecimal;
        // Java: int sciexp = 0;
        // Java: if (p != -1)
        {
            // Java: // scientific notation
            // Java: fpdecimal = s.substring(0, p);
            // Java: String fpexp = s.substring(p+1);
            // Java: boolean negative = false;
            // Java: for (int i=0; i<fpexp.length(); i++)
                // Java: if (fpexp.charAt(i) == '-')
                    // Java: negative = true;
                    // Java: continue;
                // Java: if (fpexp.charAt(i) >= '0' && fpexp.charAt(i) <= '9')
                {
                    // Java: sciexp = sciexp * 10 + fpexp.charAt(i) - '0';
                }
            // Java: if (negative)
            {
                // Java: sciexp = -sciexp;
            }
        }
        // Java: else
        {
            // Java: // normal case
            // Java: fpdecimal = s;
        }
        // Java: // If there is a minus sign in the number then it is negative
        // Java: if (fpdecimal.indexOf("-") !=  -1)
        {
            // Java: sign = -1;
        }
        // Java: // First off, find all of the leading zeros, trailing zeros, and significant digits
        // Java: p = 0;
        // Java: // Move p to first significant digit
        // Java: int decimalPos = 0;
        // Java: for (;;)
        {
            // Java: if (fpdecimal.charAt(p) >= '1' && fpdecimal.charAt(p) <= '9')
            {
                // Java: break;
            }
            // Java: if (decimalFound && fpdecimal.charAt(p) == '0')
            {
                // Java: decimalPos--;
            }
            // Java: if (fpdecimal.charAt(p) == '.')
            {
                // Java: decimalFound = true;
            }
            // Java: p++;
            // Java: if (p == fpdecimal.length())
            {
                // Java: break;
            }
        }
        // Java: // Copy the string onto Stripped
        // Java: int q = offset;
        // Java: striped[0] = '0';
        // Java: striped[1] = '0';
        // Java: striped[2] = '0';
        // Java: striped[3] = '0';
        // Java: int significantDigits=0;
        // Java: for(;;)
        {
            // Java: if (p == (fpdecimal.length()))
            {
                // Java: break;
            }
            // Java: // Don't want to run pass the end of the array
            // Java: if (q == mant.length*rsize+offset+1)
            {
                // Java: break;
            }
            // Java: if (fpdecimal.charAt(p) == '.')
            {
                // Java: decimalFound = true;
                // Java: decimalPos = significantDigits;
                // Java: p++;
                // Java: continue;
            }
            // Java: if (fpdecimal.charAt(p) < '0' || fpdecimal.charAt(p) > '9')
            {
                // Java: p++;
                // Java: continue;
            }
            // Java: striped[q] = fpdecimal.charAt(p);
            // Java: q++;
            // Java: p++;
            // Java: significantDigits++;
        }
        // Java: // If the decimal point has been found then get rid of trailing zeros.
        // Java: if (decimalFound && q != offset)
        {
            // Java: for (;;)
            {
                // Java: q--;
                // Java: if (q == offset)
                {
                    // Java: break;
                }
                // Java: if (striped[q] == '0')
                {
                    // Java: significantDigits--;
                }
                // Java: else
                {
                    // Java: break;
                }
            }
        }
        // Java: // special case of numbers like "0.00000"
        // Java: if (decimalFound && significantDigits == 0)
        {
            // Java: decimalPos = 0;
        }
        // Java: // Implicit decimal point at end of number if not present
        // Java: if (!decimalFound)
        {
            // Java: decimalPos = q-offset;
        }
        // Java: // Find the number of significant trailing zeros
        // Java: q = offset;  // set q to point to first sig digit
        // Java: p = significantDigits-1+offset;
        // Java: while (p > q)
        {
            // Java: if (striped[p] != '0')
            {
                // Java: break;
            }
            // Java: p--;
        }
        // Java: // Make sure the decimal is on a mod 10000 boundary
        // Java: int i = ((rsize * 100) - decimalPos - sciexp % rsize) % rsize;
        // Java: q -= i;
        // Java: decimalPos += i;
        // Java: // Make the mantissa length right by adding zeros at the end if necessary
        // Java: while ((p - q) < (mant.length * rsize))
        {
            // Java: for (i = 0; i < rsize; i++)
            {
                // Java: striped[++p] = '0';
            }
        }
        // Java: // Ok, now we know how many trailing zeros there are,
        // Java: // and where the least significant digit is
        // Java: for (i = mant.length - 1; i >= 0; i--)
        {
            // Java: mant[i] = (striped[q]   - '0') * 1000 +
                      // Java: (striped[q+1] - '0') * 100  +
                      // Java: (striped[q+2] - '0') * 10   +
                      // Java: (striped[q+3] - '0');
            // Java: q += 4;
        }
        // Java: exp = (decimalPos+sciexp) / rsize;
        // Java: if (q < striped.length)
        {
            // Java: // Is there possible another digit?
            // Java: round((striped[q] - '0')*1000);
        }
    }

    // Java: Dfp(final DfpField field, final byte sign, final byte nans)
    pub fn Dfp_7(&self, field: DfpField, sign: i8, nans: i8) {
        // Java: this.field = field;
        // Java: this.mant    = new int[field.getRadixDigits()];
        // Java: this.sign    = sign;
        // Java: this.exp     = 0;
        // Java: this.nans    = nans;
    }

    // Java: Dfp newInstance()
    pub fn newInstance(&self) -> Dfp {
        // Java: return new Dfp(getField());
        todo!();
    }

    // Java: Dfp newInstance(final byte x)
    pub fn newInstance_1(&self, x: i8) -> Dfp {
        // Java: return new Dfp(getField(), x);
        todo!();
    }

    // Java: Dfp newInstance(final int x)
    pub fn newInstance_2(&self, x: i32) -> Dfp {
        // Java: return new Dfp(getField(), x);
        todo!();
    }

    // Java: Dfp newInstance(final long x)
    pub fn newInstance_3(&self, x: i64) -> Dfp {
        // Java: return new Dfp(getField(), x);
        todo!();
    }

    // Java: Dfp newInstance(final double x)
    pub fn newInstance_4(&self, x: f64) -> Dfp {
        // Java: return new Dfp(getField(), x);
        todo!();
    }

    // Java: Dfp newInstance(final Dfp d)
    pub fn newInstance_5(&self, d: &Dfp) -> Dfp {
        // Java: // make sure we don't mix number with different precision
        // Java: if (field.getRadixDigits() != d.field.getRadixDigits())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: final Dfp result = newInstance(getZero());
            // Java: result.nans = QNAN;
            // Java: return dotrap(DfpField.FLAG_INVALID, NEW_INSTANCE_TRAP, d, result);
        }
        // Java: return new Dfp(d);
        todo!();
    }

    // Java: Dfp newInstance(final String s)
    pub fn newInstance_6(&self, s: &str) -> Dfp {
        // Java: return new Dfp(field, s);
        todo!();
    }

    // Java: Dfp newInstance(final byte sig, final byte code)
    pub fn newInstance_7(&self, sig: i8, code: i8) -> Dfp {
        // Java: return field.newDfp(sig, code);
        todo!();
    }

    // Java: DfpField getField()
    pub fn getField(&self) -> DfpField {
        // Java: return field;
        todo!();
    }

    // Java: int getRadixDigits()
    pub fn getRadixDigits(&self) -> i32 {
        // Java: return field.getRadixDigits();
        todo!();
    }

    // Java: Dfp getZero()
    pub fn getZero(&self) -> Dfp {
        // Java: return field.getZero();
        todo!();
    }

    // Java: Dfp getOne()
    pub fn getOne(&self) -> Dfp {
        // Java: return field.getOne();
        todo!();
    }

    // Java: Dfp getTwo()
    pub fn getTwo(&self) -> Dfp {
        // Java: return field.getTwo();
        todo!();
    }

    // Java: void shiftLeft()
    pub fn shiftLeft(&self) {
        // Java: for (int i = mant.length - 1; i > 0; i--)
        {
            // Java: mant[i] = mant[i-1];
        }
        // Java: mant[0] = 0;
        // Java: exp--;
    }

    // Java: uses shiftRight()
    pub fn shiftRight(&self) {
    }

    // Java: void shiftRight()
    pub fn shiftRight_1(&self) {
        // Java: for (int i = 0; i < mant.length - 1; i++)
        {
            // Java: mant[i] = mant[i+1];
        }
        // Java: mant[mant.length - 1] = 0;
        // Java: exp++;
    }

    // Java: int align(int e)
    pub fn align(&self, e: i32) -> i32 {
        // Java: int lostdigit = 0;
        // Java: boolean inexact = false;
        // Java: int diff = exp - e;
        // Java: int adiff = diff;
        // Java: if (adiff < 0)
        {
            // Java: adiff = -adiff;
        }
        // Java: if (diff == 0)
        {
            // Java: return 0;
        }
        // Java: if (adiff > (mant.length + 1))
        {
            // Java: // Special case
            // Java: Arrays.fill(mant, 0);
            // Java: exp = e;
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INEXACT);
            // Java: dotrap(DfpField.FLAG_INEXACT, ALIGN_TRAP, this, this);
            // Java: return 0;
        }
        // Java: for (int i = 0; i < adiff; i++)
        {
            // Java: if (diff < 0)
            {
                // Java: /* Keep track of loss -- only signal inexact after losing 2 digits.
                 // Java: * the first lost digit is returned to add() and may be incorporated
                 // Java: * into the result.
                 // Java: */
                // Java: if (lostdigit != 0)
                {
                    // Java: inexact = true;
                }
                // Java: lostdigit = mant[0];
                // Java: shiftRight();
            }
            // Java: else
            {
                // Java: shiftLeft();
            }
        }
        // Java: if (inexact)
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INEXACT);
            // Java: dotrap(DfpField.FLAG_INEXACT, ALIGN_TRAP, this, this);
        }
        // Java: return lostdigit;
        todo!();
    }

    // Java: boolean lessThan(final Dfp x)
    pub fn lessThan(&self, x: &Dfp) -> bool {
        // Java: // make sure we don't mix number with different precision
        // Java: if (field.getRadixDigits() != x.field.getRadixDigits())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: final Dfp result = newInstance(getZero());
            // Java: result.nans = QNAN;
            // Java: dotrap(DfpField.FLAG_INVALID, LESS_THAN_TRAP, x, result);
            // Java: return false;
        }
        // Java: /* if a nan is involved, signal invalid and return false */
        // Java: if (isNaN() || x.isNaN())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: dotrap(DfpField.FLAG_INVALID, LESS_THAN_TRAP, x, newInstance(getZero()));
            // Java: return false;
        }
        // Java: return compare(this, x) < 0;
        todo!();
    }

    // Java: boolean greaterThan(final Dfp x)
    pub fn greaterThan(&self, x: &Dfp) -> bool {
        // Java: // make sure we don't mix number with different precision
        // Java: if (field.getRadixDigits() != x.field.getRadixDigits())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: final Dfp result = newInstance(getZero());
            // Java: result.nans = QNAN;
            // Java: dotrap(DfpField.FLAG_INVALID, GREATER_THAN_TRAP, x, result);
            // Java: return false;
        }
        // Java: /* if a nan is involved, signal invalid and return false */
        // Java: if (isNaN() || x.isNaN())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: dotrap(DfpField.FLAG_INVALID, GREATER_THAN_TRAP, x, newInstance(getZero()));
            // Java: return false;
        }
        // Java: return compare(this, x) > 0;
        todo!();
    }

    // Java: boolean negativeOrNull()
    pub fn negativeOrNull(&self) -> bool {
        // Java: if (isNaN())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: dotrap(DfpField.FLAG_INVALID, LESS_THAN_TRAP, this, newInstance(getZero()));
            // Java: return false;
        }
        // Java: return (sign < 0) || ((mant[mant.length - 1] == 0) && !isInfinite());
        todo!();
    }

    // Java: boolean strictlyNegative()
    pub fn strictlyNegative(&self) -> bool {
        // Java: if (isNaN())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: dotrap(DfpField.FLAG_INVALID, LESS_THAN_TRAP, this, newInstance(getZero()));
            // Java: return false;
        }
        // Java: return (sign < 0) && ((mant[mant.length - 1] != 0) || isInfinite());
        todo!();
    }

    // Java: boolean positiveOrNull()
    pub fn positiveOrNull(&self) -> bool {
        // Java: if (isNaN())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: dotrap(DfpField.FLAG_INVALID, LESS_THAN_TRAP, this, newInstance(getZero()));
            // Java: return false;
        }
        // Java: return (sign > 0) || ((mant[mant.length - 1] == 0) && !isInfinite());
        todo!();
    }

    // Java: boolean strictlyPositive()
    pub fn strictlyPositive(&self) -> bool {
        // Java: if (isNaN())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: dotrap(DfpField.FLAG_INVALID, LESS_THAN_TRAP, this, newInstance(getZero()));
            // Java: return false;
        }
        // Java: return (sign > 0) && ((mant[mant.length - 1] != 0) || isInfinite());
        todo!();
    }

    // Java: Dfp abs()
    pub fn abs(&self) -> Dfp {
        // Java: Dfp result = newInstance(this);
        // Java: result.sign = 1;
        // Java: return result;
        todo!();
    }

    // Java: boolean isInfinite()
    pub fn isInfinite(&self) -> bool {
        // Java: return nans == INFINITE;
        todo!();
    }

    // Java: boolean isNaN()
    pub fn isNaN(&self) -> bool {
        // Java: return (nans == QNAN) || (nans == SNAN);
        todo!();
    }

    // Java: boolean isZero()
    pub fn isZero(&self) -> bool {
        // Java: if (isNaN())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: dotrap(DfpField.FLAG_INVALID, LESS_THAN_TRAP, this, newInstance(getZero()));
            // Java: return false;
        }
        // Java: return (mant[mant.length - 1] == 0) && !isInfinite();
        todo!();
    }

    // Java: boolean equals(final Object other)
    pub fn equals(&self, other: &dyn std::any::Any) -> bool {
        // Java: if (other instanceof Dfp)
        {
            // Java: final Dfp x = (Dfp) other;
            // Java: if (isNaN() || x.isNaN() || field.getRadixDigits() != x.field.getRadixDigits())
            {
                // Java: return false;
            }
            // Java: return compare(this, x) == 0;
        }
        // Java: return false;
        todo!();
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return 17 + (isZero() ? 0 : (sign << 8)) + (nans << 16) + exp + Arrays.hashCode(mant);
        todo!();
    }

    // Java: boolean unequal(final Dfp x)
    pub fn unequal(&self, x: &Dfp) -> bool {
        // Java: if (isNaN() || x.isNaN() || field.getRadixDigits() != x.field.getRadixDigits())
        {
            // Java: return false;
        }
        // Java: return greaterThan(x) || lessThan(x);
        todo!();
    }

    // Java: static int compare(final Dfp a, final Dfp b)
    pub fn compare(&self, a: &Dfp, b: &Dfp) -> i32 {
        // Java: // Ignore the sign of zero
        // Java: if (a.mant[a.mant.length - 1] == 0 && b.mant[b.mant.length - 1] == 0 &&
            // Java: a.nans == FINITE && b.nans == FINITE)
            {
            // Java: return 0;
            }
        // Java: if (a.sign != b.sign)
        {
            // Java: if (a.sign == -1)
            {
                // Java: return -1;
            }
            // Java: else
            {
                // Java: return 1;
            }
        }
        // Java: // deal with the infinities
        // Java: if (a.nans == INFINITE && b.nans == FINITE)
        {
            // Java: return a.sign;
        }
        // Java: if (a.nans == FINITE && b.nans == INFINITE)
        {
            // Java: return -b.sign;
        }
        // Java: if (a.nans == INFINITE && b.nans == INFINITE)
        {
            // Java: return 0;
        }
        // Java: // Handle special case when a or b is zero, by ignoring the exponents
        // Java: if (b.mant[b.mant.length-1] != 0 && a.mant[b.mant.length-1] != 0)
        {
            // Java: if (a.exp < b.exp)
            {
                // Java: return -a.sign;
            }
            // Java: if (a.exp > b.exp)
            {
                // Java: return a.sign;
            }
        }
        // Java: // compare the mantissas
        // Java: for (int i = a.mant.length - 1; i >= 0; i--)
        {
            // Java: if (a.mant[i] > b.mant[i])
            {
                // Java: return a.sign;
            }
            // Java: if (a.mant[i] < b.mant[i])
            {
                // Java: return -a.sign;
            }
        }
        // Java: return 0;
        todo!();
    }

    // Java: Dfp rint()
    pub fn rint(&self) -> Dfp {
        // Java: return trunc(DfpField.RoundingMode.ROUND_HALF_EVEN);
        todo!();
    }

    // Java: Dfp floor()
    pub fn floor(&self) -> Dfp {
        // Java: return trunc(DfpField.RoundingMode.ROUND_FLOOR);
        todo!();
    }

    // Java: Dfp ceil()
    pub fn ceil(&self) -> Dfp {
        // Java: return trunc(DfpField.RoundingMode.ROUND_CEIL);
        todo!();
    }

    // Java: Dfp remainder(final Dfp d)
    pub fn remainder(&self, d: &Dfp) -> Dfp {
        // Java: final Dfp result = this.subtract(this.divide(d).rint().multiply(d));
        // Java: // IEEE 854-1987 says that if the result is zero, then it carries the sign of this
        // Java: if (result.mant[mant.length-1] == 0)
        {
            // Java: result.sign = sign;
        }
        // Java: return result;
        todo!();
    }

    // Java: Dfp trunc(final DfpField.RoundingMode rmode)
    pub fn trunc(&self, rmode: i32) -> Dfp {
        // Java: boolean changed = false;
        // Java: if (isNaN())
        {
            // Java: return newInstance(this);
        }
        // Java: if (nans == INFINITE)
        {
            // Java: return newInstance(this);
        }
        // Java: if (mant[mant.length-1] == 0)
        {
            // Java: // a is zero
            // Java: return newInstance(this);
        }
        // Java: /* If the exponent is less than zero then we can certainly
         // Java: * return zero */
        // Java: if (exp < 0)
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INEXACT);
            // Java: Dfp result = newInstance(getZero());
            // Java: result = dotrap(DfpField.FLAG_INEXACT, TRUNC_TRAP, this, result);
            // Java: return result;
        }
        // Java: /* If the exponent is greater than or equal to digits, then it
         // Java: * must already be an integer since there is no precision left
         // Java: * for any fractional part */
        // Java: if (exp >= mant.length)
        {
            // Java: return newInstance(this);
        }
        // Java: /* General case:  create another dfp, result, that contains the
         // Java: * a with the fractional part lopped off.  */
        // Java: Dfp result = newInstance(this);
        // Java: for (int i = 0; i < mant.length-result.exp; i++)
        {
            // Java: changed |= result.mant[i] != 0;
            // Java: result.mant[i] = 0;
        }
        // Java: if (changed)
        {
            // Java: switch (rmode)
            {
                // Java: case ROUND_FLOOR:
                    // Java: if (result.sign == -1)
                    {
                        // Java: // then we must increment the mantissa by one
                        // Java: result = result.add(newInstance(-1));
                    }
                    // Java: break;
                // Java: case ROUND_CEIL:
                    // Java: if (result.sign == 1)
                    {
                        // Java: // then we must increment the mantissa by one
                        // Java: result = result.add(getOne());
                    }
                    // Java: break;
                // Java: case ROUND_HALF_EVEN:
                // Java: default:
                    // Java: final Dfp half = newInstance("0.5");
                    // Java: Dfp a = subtract(result);  // difference between this and result
                    // Java: a.sign = 1;            // force positive (take abs)
                    // Java: if (a.greaterThan(half))
                    {
                        // Java: a = newInstance(getOne());
                        // Java: a.sign = sign;
                        // Java: result = result.add(a);
                    }
                    // Java: /** If exactly equal to 1/2 and odd then increment */
                    // Java: if (a.equals(half) && result.exp > 0 && (result.mant[mant.length-result.exp]&1) != 0)
                    {
                        // Java: a = newInstance(getOne());
                        // Java: a.sign = sign;
                        // Java: result = result.add(a);
                    }
                    // Java: break;
            }
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INEXACT);  // signal inexact
            // Java: result = dotrap(DfpField.FLAG_INEXACT, TRUNC_TRAP, this, result);
            // Java: return result;
        }
        // Java: return result;
        todo!();
    }

    // Java: int intValue()
    pub fn intValue(&self) -> i32 {
        // Java: Dfp rounded;
        // Java: int result = 0;
        // Java: rounded = rint();
        // Java: if (rounded.greaterThan(newInstance(2147483647)))
        {
            // Java: return 2147483647;
        }
        // Java: if (rounded.lessThan(newInstance(-2147483648)))
        {
            // Java: return -2147483648;
        }
        // Java: for (int i = mant.length - 1; i >= mant.length - rounded.exp; i--)
        {
            // Java: result = result * RADIX + rounded.mant[i];
        }
        // Java: if (rounded.sign == -1)
        {
            // Java: result = -result;
        }
        // Java: return result;
        todo!();
    }

    // Java: int log10K()
    pub fn log10K(&self) -> i32 {
        // Java: return exp - 1;
        todo!();
    }

    // Java: Dfp power10K(final int e)
    pub fn power10K(&self, e: i32) -> Dfp {
        // Java: Dfp d = newInstance(getOne());
        // Java: d.exp = e + 1;
        // Java: return d;
        todo!();
    }

    // Java: int intLog10()
    pub fn intLog10(&self) -> i32 {
        // Java: if (mant[mant.length-1] > 1000)
        {
            // Java: return exp * 4 - 1;
        }
        // Java: if (mant[mant.length-1] > 100)
        {
            // Java: return exp * 4 - 2;
        }
        // Java: if (mant[mant.length-1] > 10)
        {
            // Java: return exp * 4 - 3;
        }
        // Java: return exp * 4 - 4;
        todo!();
    }

    // Java: Dfp power10(final int e)
    pub fn power10(&self, e: i32) -> Dfp {
        // Java: Dfp d = newInstance(getOne());
        // Java: if (e >= 0)
        {
            // Java: d.exp = e / 4 + 1;
        }
        // Java: else
        {
            // Java: d.exp = (e + 1) / 4;
        }
        // Java: switch ((e % 4 + 4) % 4)
        {
            // Java: case 0:
                // Java: break;
            // Java: case 1:
                // Java: d = d.multiply(10);
                // Java: break;
            // Java: case 2:
                // Java: d = d.multiply(100);
                // Java: break;
            // Java: default:
                // Java: d = d.multiply(1000);
        }
        // Java: return d;
        todo!();
    }

    // Java: int complement(int extra)
    pub fn complement(&self, extra: i32) -> i32 {
        // Java: extra = RADIX-extra;
        // Java: for (int i = 0; i < mant.length; i++)
        {
            // Java: mant[i] = RADIX-mant[i]-1;
        }
        // Java: int rh = extra / RADIX;
        // Java: extra -= rh * RADIX;
        // Java: for (int i = 0; i < mant.length; i++)
        {
            // Java: final int r = mant[i] + rh;
            // Java: rh = r / RADIX;
            // Java: mant[i] = r - rh * RADIX;
        }
        // Java: return extra;
        todo!();
    }

    // Java: Dfp add(final Dfp x)
    pub fn add(&self, x: &Dfp) -> Dfp {
        // Java: // make sure we don't mix number with different precision
        // Java: if (field.getRadixDigits() != x.field.getRadixDigits())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: final Dfp result = newInstance(getZero());
            // Java: result.nans = QNAN;
            // Java: return dotrap(DfpField.FLAG_INVALID, ADD_TRAP, x, result);
        }
        // Java: /* handle special cases */
        // Java: if (nans != FINITE || x.nans != FINITE)
        {
            // Java: if (isNaN())
            {
                // Java: return this;
            }
            // Java: if (x.isNaN())
            {
                // Java: return x;
            }
            // Java: if (nans == INFINITE && x.nans == FINITE)
            {
                // Java: return this;
            }
            // Java: if (x.nans == INFINITE && nans == FINITE)
            {
                // Java: return x;
            }
            // Java: if (x.nans == INFINITE && nans == INFINITE && sign == x.sign)
            {
                // Java: return x;
            }
            // Java: if (x.nans == INFINITE && nans == INFINITE && sign != x.sign)
            {
                // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
                // Java: Dfp result = newInstance(getZero());
                // Java: result.nans = QNAN;
                // Java: result = dotrap(DfpField.FLAG_INVALID, ADD_TRAP, x, result);
                // Java: return result;
            }
        }
        // Java: /* copy this and the arg */
        // Java: Dfp a = newInstance(this);
        // Java: Dfp b = newInstance(x);
        // Java: /* initialize the result object */
        // Java: Dfp result = newInstance(getZero());
        // Java: /* Make all numbers positive, but remember their sign */
        // Java: final byte asign = a.sign;
        // Java: final byte bsign = b.sign;
        // Java: a.sign = 1;
        // Java: b.sign = 1;
        // Java: /* The result will be signed like the arg with greatest magnitude */
        // Java: byte rsign = bsign;
        // Java: if (compare(a, b) > 0)
        {
            // Java: rsign = asign;
        }
        // Java: /* Handle special case when a or b is zero, by setting the exponent
       // Java: of the zero number equal to the other one.  This avoids an alignment
       // Java: which would cause catastropic loss of precision */
        // Java: if (b.mant[mant.length-1] == 0)
        {
            // Java: b.exp = a.exp;
        }
        // Java: if (a.mant[mant.length-1] == 0)
        {
            // Java: a.exp = b.exp;
        }
        // Java: /* align number with the smaller exponent */
        // Java: int aextradigit = 0;
        // Java: int bextradigit = 0;
        // Java: if (a.exp < b.exp)
        {
            // Java: aextradigit = a.align(b.exp);
        }
        // Java: else
        {
            // Java: bextradigit = b.align(a.exp);
        }
        // Java: /* complement the smaller of the two if the signs are different */
        // Java: if (asign != bsign)
        {
            // Java: if (asign == rsign)
            {
                // Java: bextradigit = b.complement(bextradigit);
            }
            // Java: else
            {
                // Java: aextradigit = a.complement(aextradigit);
            }
        }
        // Java: /* add the mantissas */
        // Java: int rh = 0; /* acts as a carry */
        // Java: for (int i = 0; i < mant.length; i++)
        {
            // Java: final int r = a.mant[i]+b.mant[i]+rh;
            // Java: rh = r / RADIX;
            // Java: result.mant[i] = r - rh * RADIX;
        }
        // Java: result.exp = a.exp;
        // Java: result.sign = rsign;
        // Java: /* handle overflow -- note, when asign!=bsign an overflow is
         // Java: * normal and should be ignored.  */
        // Java: if (rh != 0 && (asign == bsign))
        {
            // Java: final int lostdigit = result.mant[0];
            // Java: result.shiftRight();
            // Java: result.mant[mant.length-1] = rh;
            // Java: final int excp = result.round(lostdigit);
            // Java: if (excp != 0)
            {
                // Java: result = dotrap(excp, ADD_TRAP, x, result);
            }
        }
        // Java: /* normalize the result */
        // Java: for (int i = 0; i < mant.length; i++)
        {
            // Java: if (result.mant[mant.length-1] != 0)
            {
                // Java: break;
            }
            // Java: result.shiftLeft();
            // Java: if (i == 0)
            {
                // Java: result.mant[0] = aextradigit+bextradigit;
                // Java: aextradigit = 0;
                // Java: bextradigit = 0;
            }
        }
        // Java: /* result is zero if after normalization the most sig. digit is zero */
        // Java: if (result.mant[mant.length-1] == 0)
        {
            // Java: result.exp = 0;
            // Java: if (asign != bsign)
            {
                // Java: // Unless adding 2 negative zeros, sign is positive
                // Java: result.sign = 1;  // Per IEEE 854-1987 Section 6.3
            }
        }
        // Java: /* Call round to test for over/under flows */
        // Java: final int excp = result.round(aextradigit + bextradigit);
        // Java: if (excp != 0)
        {
            // Java: result = dotrap(excp, ADD_TRAP, x, result);
        }
        // Java: return result;
        todo!();
    }

    // Java: Dfp negate()
    pub fn negate(&self) -> Dfp {
        // Java: Dfp result = newInstance(this);
        // Java: result.sign = (byte) - result.sign;
        // Java: return result;
        todo!();
    }

    // Java: Dfp subtract(final Dfp x)
    pub fn subtract(&self, x: &Dfp) -> Dfp {
        // Java: return add(x.negate());
        todo!();
    }

    // Java: int round(int n)
    pub fn round(&self, n: i32) -> i32 {
        // Java: boolean inc = false;
        // Java: switch (field.getRoundingMode())
        {
            // Java: case ROUND_DOWN:
                // Java: inc = false;
                // Java: break;
            // Java: case ROUND_UP:
                // Java: inc = n != 0;       // round up if n!=0
                // Java: break;
            // Java: case ROUND_HALF_UP:
                // Java: inc = n >= 5000;  // round half up
                // Java: break;
            // Java: case ROUND_HALF_DOWN:
                // Java: inc = n > 5000;  // round half down
                // Java: break;
            // Java: case ROUND_HALF_EVEN:
                // Java: inc = n > 5000 || (n == 5000 && (mant[0] & 1) == 1);  // round half-even
                // Java: break;
            // Java: case ROUND_HALF_ODD:
                // Java: inc = n > 5000 || (n == 5000 && (mant[0] & 1) == 0);  // round half-odd
                // Java: break;
            // Java: case ROUND_CEIL:
                // Java: inc = sign == 1 && n != 0;  // round ceil
                // Java: break;
            // Java: case ROUND_FLOOR:
            // Java: default:
                // Java: inc = sign == -1 && n != 0;  // round floor
                // Java: break;
        }
        // Java: if (inc)
        {
            // Java: // increment if necessary
            // Java: int rh = 1;
            // Java: for (int i = 0; i < mant.length; i++)
            {
                // Java: final int r = mant[i] + rh;
                // Java: rh = r / RADIX;
                // Java: mant[i] = r - rh * RADIX;
            }
            // Java: if (rh != 0)
            {
                // Java: shiftRight();
                // Java: mant[mant.length-1] = rh;
            }
        }
        // Java: // check for exceptional cases and raise signals if necessary
        // Java: if (exp < MIN_EXP)
        {
            // Java: // Gradual Underflow
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_UNDERFLOW);
            // Java: return DfpField.FLAG_UNDERFLOW;
        }
        // Java: if (exp > MAX_EXP)
        {
            // Java: // Overflow
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_OVERFLOW);
            // Java: return DfpField.FLAG_OVERFLOW;
        }
        // Java: if (n != 0)
        {
            // Java: // Inexact
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INEXACT);
            // Java: return DfpField.FLAG_INEXACT;
        }
        // Java: return 0;
        todo!();
    }

    // Java: Dfp multiply(final Dfp x)
    pub fn multiply(&self, x: &Dfp) -> Dfp {
        // Java: // make sure we don't mix number with different precision
        // Java: if (field.getRadixDigits() != x.field.getRadixDigits())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: final Dfp result = newInstance(getZero());
            // Java: result.nans = QNAN;
            // Java: return dotrap(DfpField.FLAG_INVALID, MULTIPLY_TRAP, x, result);
        }
        // Java: Dfp result = newInstance(getZero());
        // Java: /* handle special cases */
        // Java: if (nans != FINITE || x.nans != FINITE)
        {
            // Java: if (isNaN())
            {
                // Java: return this;
            }
            // Java: if (x.isNaN())
            {
                // Java: return x;
            }
            // Java: if (nans == INFINITE && x.nans == FINITE && x.mant[mant.length-1] != 0)
            {
                // Java: result = newInstance(this);
                // Java: result.sign = (byte) (sign * x.sign);
                // Java: return result;
            }
            // Java: if (x.nans == INFINITE && nans == FINITE && mant[mant.length-1] != 0)
            {
                // Java: result = newInstance(x);
                // Java: result.sign = (byte) (sign * x.sign);
                // Java: return result;
            }
            // Java: if (x.nans == INFINITE && nans == INFINITE)
            {
                // Java: result = newInstance(this);
                // Java: result.sign = (byte) (sign * x.sign);
                // Java: return result;
            }
            // Java: if ( (x.nans == INFINITE && nans == FINITE && mant[mant.length-1] == 0) ||
                    // Java: (nans == INFINITE && x.nans == FINITE && x.mant[mant.length-1] == 0) )
                    {
                // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
                // Java: result = newInstance(getZero());
                // Java: result.nans = QNAN;
                // Java: result = dotrap(DfpField.FLAG_INVALID, MULTIPLY_TRAP, x, result);
                // Java: return result;
                    }
        }
        // Java: int[] product = new int[mant.length*2];  // Big enough to hold even the largest result
        // Java: for (int i = 0; i < mant.length; i++)
        {
            // Java: int rh = 0;  // acts as a carry
            // Java: for (int j=0; j<mant.length; j++)
            {
                // Java: int r = mant[i] * x.mant[j];    // multiply the 2 digits
                // Java: r += product[i+j] + rh;  // add to the product digit with carry in
                // Java: rh = r / RADIX;
                // Java: product[i+j] = r - rh * RADIX;
            }
            // Java: product[i+mant.length] = rh;
        }
        // Java: // Find the most sig digit
        // Java: int md = mant.length * 2 - 1;  // default, in case result is zero
        // Java: for (int i = mant.length * 2 - 1; i >= 0; i--)
        {
            // Java: if (product[i] != 0)
            {
                // Java: md = i;
                // Java: break;
            }
        }
        // Java: // Copy the digits into the result
        // Java: for (int i = 0; i < mant.length; i++)
        {
            // Java: result.mant[mant.length - i - 1] = product[md - i];
        }
        // Java: // Fixup the exponent.
        // Java: result.exp = exp + x.exp + md - 2 * mant.length + 1;
        // Java: result.sign = (byte)((sign == x.sign)?1:-1);
        // Java: if (result.mant[mant.length-1] == 0)
        {
            // Java: // if result is zero, set exp to zero
            // Java: result.exp = 0;
        }
        // Java: final int excp;
        // Java: if (md > (mant.length-1))
        {
            // Java: excp = result.round(product[md-mant.length]);
        }
        // Java: else
        {
            // Java: excp = result.round(0); // has no effect except to check status
        }
        // Java: if (excp != 0)
        {
            // Java: result = dotrap(excp, MULTIPLY_TRAP, x, result);
        }
        // Java: return result;
        todo!();
    }

    // Java: Dfp multiply(final int x)
    pub fn multiply_1(&self, x: i32) -> Dfp {
        // Java: if (x >= 0 && x < RADIX)
        {
            // Java: return multiplyFast(x);
        }
        // Java: else
        {
            // Java: return multiply(newInstance(x));
        }
        todo!();
    }

    // Java: Dfp multiplyFast(final int x)
    pub fn multiplyFast(&self, x: i32) -> Dfp {
        // Java: Dfp result = newInstance(this);
        // Java: /* handle special cases */
        // Java: if (nans != FINITE)
        {
            // Java: if (isNaN())
            {
                // Java: return this;
            }
            // Java: if (nans == INFINITE && x != 0)
            {
                // Java: result = newInstance(this);
                // Java: return result;
            }
            // Java: if (nans == INFINITE && x == 0)
            {
                // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
                // Java: result = newInstance(getZero());
                // Java: result.nans = QNAN;
                // Java: result = dotrap(DfpField.FLAG_INVALID, MULTIPLY_TRAP, newInstance(getZero()), result);
                // Java: return result;
            }
        }
        // Java: /* range check x */
        // Java: if (x < 0 || x >= RADIX)
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: result = newInstance(getZero());
            // Java: result.nans = QNAN;
            // Java: result = dotrap(DfpField.FLAG_INVALID, MULTIPLY_TRAP, result, result);
            // Java: return result;
        }
        // Java: int rh = 0;
        // Java: for (int i = 0; i < mant.length; i++)
        {
            // Java: final int r = mant[i] * x + rh;
            // Java: rh = r / RADIX;
            // Java: result.mant[i] = r - rh * RADIX;
        }
        // Java: int lostdigit = 0;
        // Java: if (rh != 0)
        {
            // Java: lostdigit = result.mant[0];
            // Java: result.shiftRight();
            // Java: result.mant[mant.length-1] = rh;
        }
        // Java: if (result.mant[mant.length-1] == 0) { // if result is zero, set exp to zero
            // Java: result.exp = 0;
        // Java: final int excp = result.round(lostdigit);
        // Java: if (excp != 0)
        {
            // Java: result = dotrap(excp, MULTIPLY_TRAP, result, result);
        }
        // Java: return result;
        todo!();
    }

    // Java: Dfp divide(Dfp divisor)
    pub fn divide(&self, divisor: &Dfp) -> Dfp {
        // Java: int dividend[]; // current status of the dividend
        // Java: int quotient[]; // quotient
        // Java: int remainder[];// remainder
        // Java: int qd;         // current quotient digit we're working with
        // Java: int nsqd;       // number of significant quotient digits we have
        // Java: int trial=0;    // trial quotient digit
        // Java: int minadj;     // minimum adjustment
        // Java: boolean trialgood; // Flag to indicate a good trail digit
        // Java: int md=0;       // most sig digit in result
        // Java: int excp;       // exceptions
        // Java: // make sure we don't mix number with different precision
        // Java: if (field.getRadixDigits() != divisor.field.getRadixDigits())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: final Dfp result = newInstance(getZero());
            // Java: result.nans = QNAN;
            // Java: return dotrap(DfpField.FLAG_INVALID, DIVIDE_TRAP, divisor, result);
        }
        // Java: Dfp result = newInstance(getZero());
        // Java: /* handle special cases */
        // Java: if (nans != FINITE || divisor.nans != FINITE)
        {
            // Java: if (isNaN())
            {
                // Java: return this;
            }
            // Java: if (divisor.isNaN())
            {
                // Java: return divisor;
            }
            // Java: if (nans == INFINITE && divisor.nans == FINITE)
            {
                // Java: result = newInstance(this);
                // Java: result.sign = (byte) (sign * divisor.sign);
                // Java: return result;
            }
            // Java: if (divisor.nans == INFINITE && nans == FINITE)
            {
                // Java: result = newInstance(getZero());
                // Java: result.sign = (byte) (sign * divisor.sign);
                // Java: return result;
            }
            // Java: if (divisor.nans == INFINITE && nans == INFINITE)
            {
                // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
                // Java: result = newInstance(getZero());
                // Java: result.nans = QNAN;
                // Java: result = dotrap(DfpField.FLAG_INVALID, DIVIDE_TRAP, divisor, result);
                // Java: return result;
            }
        }
        // Java: /* Test for divide by zero */
        // Java: if (divisor.mant[mant.length-1] == 0)
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_DIV_ZERO);
            // Java: result = newInstance(getZero());
            // Java: result.sign = (byte) (sign * divisor.sign);
            // Java: result.nans = INFINITE;
            // Java: result = dotrap(DfpField.FLAG_DIV_ZERO, DIVIDE_TRAP, divisor, result);
            // Java: return result;
        }
        // Java: dividend = new int[mant.length+1];  // one extra digit needed
        // Java: quotient = new int[mant.length+2];  // two extra digits needed 1 for overflow, 1 for rounding
        // Java: remainder = new int[mant.length+1]; // one extra digit needed
        // Java: /* Initialize our most significant digits to zero */
        // Java: dividend[mant.length] = 0;
        // Java: quotient[mant.length] = 0;
        // Java: quotient[mant.length+1] = 0;
        // Java: remainder[mant.length] = 0;
        // Java: /* copy our mantissa into the dividend, initialize the
       // Java: quotient while we are at it */
        // Java: for (int i = 0; i < mant.length; i++)
        {
            // Java: dividend[i] = mant[i];
            // Java: quotient[i] = 0;
            // Java: remainder[i] = 0;
        }
        // Java: /* outer loop.  Once per quotient digit */
        // Java: nsqd = 0;
        // Java: for (qd = mant.length+1; qd >= 0; qd--)
        {
            // Java: /* Determine outer limits of our quotient digit */
            // Java: // r =  most sig 2 digits of dividend
            // Java: final int divMsb = dividend[mant.length]*RADIX+dividend[mant.length-1];
            // Java: int min = divMsb       / (divisor.mant[mant.length-1]+1);
            // Java: int max = (divMsb + 1) / divisor.mant[mant.length-1];
            // Java: trialgood = false;
            // Java: while (!trialgood)
            {
                // Java: // try the mean
                // Java: trial = (min+max)/2;
                // Java: /* Multiply by divisor and store as remainder */
                // Java: int rh = 0;
                // Java: for (int i = 0; i < mant.length + 1; i++)
                {
                    // Java: int dm = (i<mant.length)?divisor.mant[i]:0;
                    // Java: final int r = (dm * trial) + rh;
                    // Java: rh = r / RADIX;
                    // Java: remainder[i] = r - rh * RADIX;
                }
                // Java: /* subtract the remainder from the dividend */
                // Java: rh = 1;  // carry in to aid the subtraction
                // Java: for (int i = 0; i < mant.length + 1; i++)
                {
                    // Java: final int r = ((RADIX-1) - remainder[i]) + dividend[i] + rh;
                    // Java: rh = r / RADIX;
                    // Java: remainder[i] = r - rh * RADIX;
                }
                // Java: /* Lets analyze what we have here */
                // Java: if (rh == 0)
                {
                    // Java: // trial is too big -- negative remainder
                    // Java: max = trial-1;
                    // Java: continue;
                }
                // Java: /* find out how far off the remainder is telling us we are */
                // Java: minadj = (remainder[mant.length] * RADIX)+remainder[mant.length-1];
                // Java: minadj /= divisor.mant[mant.length-1] + 1;
                // Java: if (minadj >= 2)
                {
                    // Java: min = trial+minadj;  // update the minimum
                    // Java: continue;
                }
                // Java: /* May have a good one here, check more thoroughly.  Basically
           // Java: its a good one if it is less than the divisor */
                // Java: trialgood = false;  // assume false
                // Java: for (int i = mant.length - 1; i >= 0; i--)
                {
                    // Java: if (divisor.mant[i] > remainder[i])
                    {
                        // Java: trialgood = true;
                    }
                    // Java: if (divisor.mant[i] < remainder[i])
                    {
                        // Java: break;
                    }
                }
                // Java: if (remainder[mant.length] != 0)
                {
                    // Java: trialgood = false;
                }
                // Java: if (trialgood == false)
                {
                    // Java: min = trial+1;
                }
            }
            // Java: /* Great we have a digit! */
            // Java: quotient[qd] = trial;
            // Java: if (trial != 0 || nsqd != 0)
            {
                // Java: nsqd++;
            }
            // Java: if (field.getRoundingMode() == DfpField.RoundingMode.ROUND_DOWN && nsqd == mant.length)
            {
                // Java: // We have enough for this mode
                // Java: break;
            }
            // Java: if (nsqd > mant.length)
            {
                // Java: // We have enough digits
                // Java: break;
            }
            // Java: /* move the remainder into the dividend while left shifting */
            // Java: dividend[0] = 0;
            // Java: for (int i = 0; i < mant.length; i++)
            {
                // Java: dividend[i + 1] = remainder[i];
            }
        }
        // Java: /* Find the most sig digit */
        // Java: md = mant.length;  // default
        // Java: for (int i = mant.length + 1; i >= 0; i--)
        {
            // Java: if (quotient[i] != 0)
            {
                // Java: md = i;
                // Java: break;
            }
        }
        // Java: /* Copy the digits into the result */
        // Java: for (int i=0; i<mant.length; i++)
        {
            // Java: result.mant[mant.length-i-1] = quotient[md-i];
        }
        // Java: /* Fixup the exponent. */
        // Java: result.exp = exp - divisor.exp + md - mant.length;
        // Java: result.sign = (byte) ((sign == divisor.sign) ? 1 : -1);
        // Java: if (result.mant[mant.length-1] == 0) { // if result is zero, set exp to zero
            // Java: result.exp = 0;
        // Java: if (md > (mant.length-1))
        {
            // Java: excp = result.round(quotient[md-mant.length]);
        }
        // Java: else
        {
            // Java: excp = result.round(0);
        }
        // Java: if (excp != 0)
        {
            // Java: result = dotrap(excp, DIVIDE_TRAP, divisor, result);
        }
        // Java: return result;
        todo!();
    }

    // Java: Dfp divide(int divisor)
    pub fn divide_1(&self, divisor: i32) -> Dfp {
        // Java: // Handle special cases
        // Java: if (nans != FINITE)
        {
            // Java: if (isNaN())
            {
                // Java: return this;
            }
            // Java: if (nans == INFINITE)
            {
                // Java: return newInstance(this);
            }
        }
        // Java: // Test for divide by zero
        // Java: if (divisor == 0)
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_DIV_ZERO);
            // Java: Dfp result = newInstance(getZero());
            // Java: result.sign = sign;
            // Java: result.nans = INFINITE;
            // Java: result = dotrap(DfpField.FLAG_DIV_ZERO, DIVIDE_TRAP, getZero(), result);
            // Java: return result;
        }
        // Java: // range check divisor
        // Java: if (divisor < 0 || divisor >= RADIX)
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: Dfp result = newInstance(getZero());
            // Java: result.nans = QNAN;
            // Java: result = dotrap(DfpField.FLAG_INVALID, DIVIDE_TRAP, result, result);
            // Java: return result;
        }
        // Java: Dfp result = newInstance(this);
        // Java: int rl = 0;
        // Java: for (int i = mant.length-1; i >= 0; i--)
        {
            // Java: final int r = rl*RADIX + result.mant[i];
            // Java: final int rh = r / divisor;
            // Java: rl = r - rh * divisor;
            // Java: result.mant[i] = rh;
        }
        // Java: if (result.mant[mant.length-1] == 0)
        {
            // Java: // normalize
            // Java: result.shiftLeft();
            // Java: final int r = rl * RADIX;        // compute the next digit and put it in
            // Java: final int rh = r / divisor;
            // Java: rl = r - rh * divisor;
            // Java: result.mant[0] = rh;
        }
        // Java: final int excp = result.round(rl * RADIX / divisor);  // do the rounding
        // Java: if (excp != 0)
        {
            // Java: result = dotrap(excp, DIVIDE_TRAP, result, result);
        }
        // Java: return result;
        todo!();
    }

    // Java: Dfp reciprocal()
    pub fn reciprocal(&self) -> Dfp {
        // Java: return field.getOne().divide(this);
        todo!();
    }

    // Java: Dfp sqrt()
    pub fn sqrt(&self) -> Dfp {
        // Java: // check for unusual cases
        // Java: if (nans == FINITE && mant[mant.length-1] == 0)
        {
            // Java: // if zero
            // Java: return newInstance(this);
        }
        // Java: if (nans != FINITE)
        {
            // Java: if (nans == INFINITE && sign == 1)
            {
                // Java: // if positive infinity
                // Java: return newInstance(this);
            }
            // Java: if (nans == QNAN)
            {
                // Java: return newInstance(this);
            }
            // Java: if (nans == SNAN)
            {
                // Java: Dfp result;
                // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
                // Java: result = newInstance(this);
                // Java: result = dotrap(DfpField.FLAG_INVALID, SQRT_TRAP, null, result);
                // Java: return result;
            }
        }
        // Java: if (sign == -1)
        {
            // Java: // if negative
            // Java: Dfp result;
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: result = newInstance(this);
            // Java: result.nans = QNAN;
            // Java: result = dotrap(DfpField.FLAG_INVALID, SQRT_TRAP, null, result);
            // Java: return result;
        }
        // Java: Dfp x = newInstance(this);
        // Java: /* Lets make a reasonable guess as to the size of the square root */
        // Java: if (x.exp < -1 || x.exp > 1)
        {
            // Java: x.exp = this.exp / 2;
        }
        // Java: /* Coarsely estimate the mantissa */
        // Java: switch (x.mant[mant.length-1] / 2000)
        {
            // Java: case 0:
                // Java: x.mant[mant.length-1] = x.mant[mant.length-1]/2+1;
                // Java: break;
            // Java: case 2:
                // Java: x.mant[mant.length-1] = 1500;
                // Java: break;
            // Java: case 3:
                // Java: x.mant[mant.length-1] = 2200;
                // Java: break;
            // Java: default:
                // Java: x.mant[mant.length-1] = 3000;
        }
        // Java: Dfp dx = newInstance(x);
        // Java: /* Now that we have the first pass estimate, compute the rest
       // Java: by the formula dx = (y - x*x) / (2x); */
        // Java: Dfp px  = getZero();
        // Java: Dfp ppx = getZero();
        // Java: while (x.unequal(px))
        {
            // Java: dx = newInstance(x);
            // Java: dx.sign = -1;
            // Java: dx = dx.add(this.divide(x));
            // Java: dx = dx.divide(2);
            // Java: ppx = px;
            // Java: px = x;
            // Java: x = x.add(dx);
            // Java: if (x.equals(ppx))
            {
                // Java: // alternating between two values
                // Java: break;
            }
            // Java: // if dx is zero, break.  Note testing the most sig digit
            // Java: // is a sufficient test since dx is normalized
            // Java: if (dx.mant[mant.length-1] == 0)
            {
                // Java: break;
            }
        }
        // Java: return x;
        todo!();
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: if (nans != FINITE)
        {
            // Java: // if non-finite exceptional cases
            // Java: if (nans == INFINITE)
            {
                // Java: return (sign < 0) ? NEG_INFINITY_STRING : POS_INFINITY_STRING;
            }
            // Java: else
            {
                // Java: return NAN_STRING;
            }
        }
        // Java: if (exp > mant.length || exp < -1)
        {
            // Java: return dfp2sci();
        }
        // Java: return dfp2string();
        todo!();
    }

    // Java: String dfp2sci()
    pub fn dfp2sci(&self) -> String {
        // Java: char rawdigits[]    = new char[mant.length * 4];
        // Java: char outputbuffer[] = new char[mant.length * 4 + 20];
        // Java: int p;
        // Java: int q;
        // Java: int e;
        // Java: int ae;
        // Java: int shf;
        // Java: // Get all the digits
        // Java: p = 0;
        // Java: for (int i = mant.length - 1; i >= 0; i--)
        {
            // Java: rawdigits[p++] = (char) ((mant[i] / 1000) + '0');
            // Java: rawdigits[p++] = (char) (((mant[i] / 100) %10) + '0');
            // Java: rawdigits[p++] = (char) (((mant[i] / 10) % 10) + '0');
            // Java: rawdigits[p++] = (char) (((mant[i]) % 10) + '0');
        }
        // Java: // Find the first non-zero one
        // Java: for (p = 0; p < rawdigits.length; p++)
        {
            // Java: if (rawdigits[p] != '0')
            {
                // Java: break;
            }
        }
        // Java: shf = p;
        // Java: // Now do the conversion
        // Java: q = 0;
        // Java: if (sign == -1)
        {
            // Java: outputbuffer[q++] = '-';
        }
        // Java: if (p != rawdigits.length)
        {
            // Java: // there are non zero digits...
            // Java: outputbuffer[q++] = rawdigits[p++];
            // Java: outputbuffer[q++] = '.';
            // Java: while (p<rawdigits.length)
            {
                // Java: outputbuffer[q++] = rawdigits[p++];
            }
        }
        // Java: else
        {
            // Java: outputbuffer[q++] = '0';
            // Java: outputbuffer[q++] = '.';
            // Java: outputbuffer[q++] = '0';
            // Java: outputbuffer[q++] = 'e';
            // Java: outputbuffer[q++] = '0';
            // Java: return new String(outputbuffer, 0, 5);
        }
        // Java: outputbuffer[q++] = 'e';
        // Java: // Find the msd of the exponent
        // Java: e = exp * 4 - shf - 1;
        // Java: ae = e;
        // Java: if (e < 0)
        {
            // Java: ae = -e;
        }
        // Java: // Find the largest p such that p < e
        // Java: for (p = 1000000000; p > ae; p /= 10)
        {
            // Java: // nothing to do
        }
        // Java: if (e < 0)
        {
            // Java: outputbuffer[q++] = '-';
        }
        // Java: while (p > 0)
        {
            // Java: outputbuffer[q++] = (char)(ae / p + '0');
            // Java: ae %= p;
            // Java: p /= 10;
        }
        // Java: return new String(outputbuffer, 0, q);
        todo!();
    }

    // Java: String dfp2string()
    pub fn dfp2string(&self) -> String {
        // Java: char buffer[] = new char[mant.length*4 + 20];
        // Java: int p = 1;
        // Java: int q;
        // Java: int e = exp;
        // Java: boolean pointInserted = false;
        // Java: buffer[0] = ' ';
        // Java: if (e <= 0)
        {
            // Java: buffer[p++] = '0';
            // Java: buffer[p++] = '.';
            // Java: pointInserted = true;
        }
        // Java: while (e < 0)
        {
            // Java: buffer[p++] = '0';
            // Java: buffer[p++] = '0';
            // Java: buffer[p++] = '0';
            // Java: buffer[p++] = '0';
            // Java: e++;
        }
        // Java: for (int i = mant.length - 1; i >= 0; i--)
        {
            // Java: buffer[p++] = (char) ((mant[i] / 1000) + '0');
            // Java: buffer[p++] = (char) (((mant[i] / 100) % 10) + '0');
            // Java: buffer[p++] = (char) (((mant[i] / 10) % 10) + '0');
            // Java: buffer[p++] = (char) (((mant[i]) % 10) + '0');
            // Java: if (--e == 0)
            {
                // Java: buffer[p++] = '.';
                // Java: pointInserted = true;
            }
        }
        // Java: while (e > 0)
        {
            // Java: buffer[p++] = '0';
            // Java: buffer[p++] = '0';
            // Java: buffer[p++] = '0';
            // Java: buffer[p++] = '0';
            // Java: e--;
        }
        // Java: if (!pointInserted)
        {
            // Java: // Ensure we have a radix point!
            // Java: buffer[p++] = '.';
        }
        // Java: // Suppress leading zeros
        // Java: q = 1;
        // Java: while (buffer[q] == '0')
        {
            // Java: q++;
        }
        // Java: if (buffer[q] == '.')
        {
            // Java: q--;
        }
        // Java: // Suppress trailing zeros
        // Java: while (buffer[p-1] == '0')
        {
            // Java: p--;
        }
        // Java: // Insert sign
        // Java: if (sign < 0)
        {
            // Java: buffer[--q] = '-';
        }
        // Java: return new String(buffer, q, p - q);
        todo!();
    }

    // Java: Dfp dotrap(int type, String what, Dfp oper, Dfp result)
    pub fn dotrap(&self, r#type: i32, what: &str, oper: &Dfp, result: &Dfp) -> Dfp {
        // Java: Dfp def = result;
        // Java: switch (type)
        {
            // Java: case DfpField.FLAG_INVALID:
                // Java: def = newInstance(getZero());
                // Java: def.sign = result.sign;
                // Java: def.nans = QNAN;
                // Java: break;
            // Java: case DfpField.FLAG_DIV_ZERO:
                // Java: if (nans == FINITE && mant[mant.length-1] != 0)
                {
                    // Java: // normal case, we are finite, non-zero
                    // Java: def = newInstance(getZero());
                    // Java: def.sign = (byte)(sign*oper.sign);
                    // Java: def.nans = INFINITE;
                }
                // Java: if (nans == FINITE && mant[mant.length-1] == 0)
                {
                    // Java: //  0/0
                    // Java: def = newInstance(getZero());
                    // Java: def.nans = QNAN;
                }
                // Java: if (nans == INFINITE || nans == QNAN)
                {
                    // Java: def = newInstance(getZero());
                    // Java: def.nans = QNAN;
                }
                // Java: if (nans == INFINITE || nans == SNAN)
                {
                    // Java: def = newInstance(getZero());
                    // Java: def.nans = QNAN;
                }
                // Java: break;
            // Java: case DfpField.FLAG_UNDERFLOW:
                // Java: if ( (result.exp+mant.length) < MIN_EXP)
                {
                    // Java: def = newInstance(getZero());
                    // Java: def.sign = result.sign;
                }
                // Java: else
                {
                    // Java: def = newInstance(result);  // gradual underflow
                }
                // Java: result.exp += ERR_SCALE;
                // Java: break;
            // Java: case DfpField.FLAG_OVERFLOW:
                // Java: result.exp -= ERR_SCALE;
                // Java: def = newInstance(getZero());
                // Java: def.sign = result.sign;
                // Java: def.nans = INFINITE;
                // Java: break;
            // Java: default: def = result; break;
        }
        // Java: return trap(type, what, oper, def, result);
        todo!();
    }

    // Java: Dfp trap(int type, String what, Dfp oper, Dfp def, Dfp result)
    pub fn trap(&self, r#type: i32, what: &str, oper: &Dfp, def: &Dfp, result: &Dfp) -> Dfp {
        // Java: return def;
        todo!();
    }

    // Java: int classify()
    pub fn classify(&self) -> i32 {
        // Java: return nans;
        todo!();
    }

    // Java: static Dfp copysign(final Dfp x, final Dfp y)
    pub fn copysign(&self, x: &Dfp, y: &Dfp) -> Dfp {
        // Java: Dfp result = x.newInstance(x);
        // Java: result.sign = y.sign;
        // Java: return result;
        todo!();
    }

    // Java: Dfp nextAfter(final Dfp x)
    pub fn nextAfter(&self, x: &Dfp) -> Dfp {
        // Java: // make sure we don't mix number with different precision
        // Java: if (field.getRadixDigits() != x.field.getRadixDigits())
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: final Dfp result = newInstance(getZero());
            // Java: result.nans = QNAN;
            // Java: return dotrap(DfpField.FLAG_INVALID, NEXT_AFTER_TRAP, x, result);
        }
        // Java: // if this is greater than x
        // Java: boolean up = false;
        // Java: if (this.lessThan(x))
        {
            // Java: up = true;
        }
        // Java: if (compare(this, x) == 0)
        {
            // Java: return newInstance(x);
        }
        // Java: if (lessThan(getZero()))
        {
            // Java: up = !up;
        }
        // Java: final Dfp inc;
        // Java: Dfp result;
        // Java: if (up)
        {
            // Java: inc = newInstance(getOne());
            // Java: inc.exp = this.exp-mant.length+1;
            // Java: inc.sign = this.sign;
            // Java: if (this.equals(getZero()))
            {
                // Java: inc.exp = MIN_EXP-mant.length;
            }
            // Java: result = add(inc);
        }
        // Java: else
        {
            // Java: inc = newInstance(getOne());
            // Java: inc.exp = this.exp;
            // Java: inc.sign = this.sign;
            // Java: if (this.equals(inc))
            {
                // Java: inc.exp = this.exp-mant.length;
            }
            // Java: else
            {
                // Java: inc.exp = this.exp-mant.length+1;
            }
            // Java: if (this.equals(getZero()))
            {
                // Java: inc.exp = MIN_EXP-mant.length;
            }
            // Java: result = this.subtract(inc);
        }
        // Java: if (result.classify() == INFINITE && this.classify() != INFINITE)
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INEXACT);
            // Java: result = dotrap(DfpField.FLAG_INEXACT, NEXT_AFTER_TRAP, x, result);
        }
        // Java: if (result.equals(getZero()) && this.equals(getZero()) == false)
        {
            // Java: field.setIEEEFlagsBits(DfpField.FLAG_INEXACT);
            // Java: result = dotrap(DfpField.FLAG_INEXACT, NEXT_AFTER_TRAP, x, result);
        }
        // Java: return result;
        todo!();
    }

    // Java: double toDouble()
    pub fn toDouble(&self) -> f64 {
        // Java: if (isInfinite())
        {
            // Java: if (lessThan(getZero()))
            {
                // Java: return Double.NEGATIVE_INFINITY;
            }
            // Java: else
            {
                // Java: return Double.POSITIVE_INFINITY;
            }
        }
        // Java: if (isNaN())
        {
            // Java: return Double.NaN;
        }
        // Java: Dfp y = this;
        // Java: boolean negate = false;
        // Java: int cmp0 = compare(this, getZero());
        // Java: if (cmp0 == 0)
        {
            // Java: return sign < 0 ? -0.0 : +0.0;
        }
        // Java: else if (cmp0 < 0)
        {
            // Java: y = negate();
            // Java: negate = true;
        }
        // Java: /* Find the exponent, first estimate by integer log10, then adjust.
         // Java: Should be faster than doing a natural logarithm.  */
        // Java: int exponent = (int)(y.intLog10() * 3.32);
        // Java: if (exponent < 0)
        {
            // Java: exponent--;
        }
        // Java: Dfp tempDfp = DfpMath.pow(getTwo(), exponent);
        // Java: while (tempDfp.lessThan(y) || tempDfp.equals(y))
        {
            // Java: tempDfp = tempDfp.multiply(2);
            // Java: exponent++;
        }
        // Java: exponent--;
        // Java: /* We have the exponent, now work on the mantissa */
        // Java: y = y.divide(DfpMath.pow(getTwo(), exponent));
        // Java: if (exponent > -1023)
        {
            // Java: y = y.subtract(getOne());
        }
        // Java: if (exponent < -1074)
        {
            // Java: return 0;
        }
        // Java: if (exponent > 1023)
        {
            // Java: return negate ? Double.NEGATIVE_INFINITY : Double.POSITIVE_INFINITY;
        }
        // Java: y = y.multiply(newInstance(4503599627370496l)).rint();
        // Java: String str = y.toString();
        // Java: str = str.substring(0, str.length()-1);
        // Java: long mantissa = Long.parseLong(str);
        // Java: if (mantissa == 4503599627370496L)
        {
            // Java: // Handle special case where we round up to next power of two
            // Java: mantissa = 0;
            // Java: exponent++;
        }
        // Java: /* Its going to be subnormal, so make adjustments */
        // Java: if (exponent <= -1023)
        {
            // Java: exponent--;
        }
        // Java: while (exponent < -1023)
        {
            // Java: exponent++;
            // Java: mantissa >>>= 1;
        }
        // Java: long bits = mantissa | ((exponent + 1023L) << 52);
        // Java: double x = Double.longBitsToDouble(bits);
        // Java: if (negate)
        {
            // Java: x = -x;
        }
        // Java: return x;
        todo!();
    }

    // Java: double[] toSplitDouble()
    pub fn toSplitDouble(&self) -> [f64; 2] {
        // Java: double split[] = new double[2];
        // Java: long mask = 0xffffffffc0000000L;
        // Java: split[0] = Double.longBitsToDouble(Double.doubleToLongBits(toDouble()) & mask);
        // Java: split[1] = subtract(newInstance(split[0])).toDouble();
        // Java: return split;
        todo!();
    }

    // Java: double getReal()
    pub fn getReal(&self) -> f64 {
        // Java: return toDouble();
        todo!();
    }

    // Java: Dfp add(final double a)
    pub fn add_1(&self, a: f64) -> Dfp {
        // Java: return add(newInstance(a));
        todo!();
    }

    // Java: Dfp subtract(final double a)
    pub fn subtract_1(&self, a: f64) -> Dfp {
        // Java: return subtract(newInstance(a));
        todo!();
    }

    // Java: Dfp multiply(final double a)
    pub fn multiply_2(&self, a: f64) -> Dfp {
        // Java: return multiply(newInstance(a));
        todo!();
    }

    // Java: Dfp divide(final double a)
    pub fn divide_2(&self, a: f64) -> Dfp {
        // Java: return divide(newInstance(a));
        todo!();
    }

    // Java: Dfp remainder(final double a)
    pub fn remainder_1(&self, a: f64) -> Dfp {
        // Java: return remainder(newInstance(a));
        todo!();
    }

    // Java: long round()
    pub fn round_1(&self) -> i64 {
        // Java: return FastMath.round(toDouble());
        todo!();
    }

    // Java: Dfp signum()
    pub fn signum(&self) -> Dfp {
        // Java: if (isNaN() || isZero())
        {
            // Java: return this;
        }
        // Java: else
        {
            // Java: return newInstance(sign > 0 ? +1 : -1);
        }
        todo!();
    }

    // Java: Dfp copySign(final Dfp s)
    pub fn copySign(&self, s: &Dfp) -> Dfp {
        // Java: if ((sign >= 0 && s.sign >= 0) || (sign < 0 && s.sign < 0)) { // Sign is currently OK
            // Java: return this;
        // Java: return negate(); // flip sign
        todo!();
    }

    // Java: Dfp copySign(final double s)
    pub fn copySign_1(&self, s: f64) -> Dfp {
        // Java: long sb = Double.doubleToLongBits(s);
        // Java: if ((sign >= 0 && sb >= 0) || (sign < 0 && sb < 0)) { // Sign is currently OK
            // Java: return this;
        // Java: return negate(); // flip sign
        todo!();
    }

    // Java: Dfp scalb(final int n)
    pub fn scalb(&self, n: i32) -> Dfp {
        // Java: return multiply(DfpMath.pow(getTwo(), n));
        todo!();
    }

    // Java: Dfp hypot(final Dfp y)
    pub fn hypot(&self, y: &Dfp) -> Dfp {
        // Java: return multiply(this).add(y.multiply(y)).sqrt();
        todo!();
    }

    // Java: Dfp cbrt()
    pub fn cbrt(&self) -> Dfp {
        // Java: return rootN(3);
        todo!();
    }

    // Java: Dfp rootN(final int n)
    pub fn rootN(&self, n: i32) -> Dfp {
        // Java: return (sign >= 0) ?
               // Java: DfpMath.pow(this, getOne().divide(n)) :
               // Java: DfpMath.pow(negate(), getOne().divide(n)).negate();
        todo!();
    }

    // Java: Dfp pow(final double p)
    pub fn pow(&self, p: f64) -> Dfp {
        // Java: return DfpMath.pow(this, newInstance(p));
        todo!();
    }

    // Java: Dfp pow(final int n)
    pub fn pow_1(&self, n: i32) -> Dfp {
        // Java: return DfpMath.pow(this, n);
        todo!();
    }

    // Java: Dfp pow(final Dfp e)
    pub fn pow_2(&self, e: &Dfp) -> Dfp {
        // Java: return DfpMath.pow(this, e);
        todo!();
    }

    // Java: Dfp exp()
    pub fn exp(&self) -> Dfp {
        // Java: return DfpMath.exp(this);
        todo!();
    }

    // Java: Dfp expm1()
    pub fn expm1(&self) -> Dfp {
        // Java: return DfpMath.exp(this).subtract(getOne());
        todo!();
    }

    // Java: Dfp log()
    pub fn log(&self) -> Dfp {
        // Java: return DfpMath.log(this);
        todo!();
    }

    // Java: Dfp log1p()
    pub fn log1p(&self) -> Dfp {
        // Java: return DfpMath.log(this.add(getOne()));
        todo!();
    }

    // Java: int log10()
    pub fn log10(&self) -> i32 {
        // Java: return intLog10();
        todo!();
    }

    // Java: Dfp cos()
    pub fn cos(&self) -> Dfp {
        // Java: return DfpMath.cos(this);
        todo!();
    }

    // Java: Dfp sin()
    pub fn sin(&self) -> Dfp {
        // Java: return DfpMath.sin(this);
        todo!();
    }

    // Java: Dfp tan()
    pub fn tan(&self) -> Dfp {
        // Java: return DfpMath.tan(this);
        todo!();
    }

    // Java: Dfp acos()
    pub fn acos(&self) -> Dfp {
        // Java: return DfpMath.acos(this);
        todo!();
    }

    // Java: Dfp asin()
    pub fn asin(&self) -> Dfp {
        // Java: return DfpMath.asin(this);
        todo!();
    }

    // Java: Dfp atan()
    pub fn atan(&self) -> Dfp {
        // Java: return DfpMath.atan(this);
        todo!();
    }

    // Java: Dfp atan2(final Dfp x)
    pub fn atan2(&self, x: &Dfp) -> Dfp {
        todo!();
    }

    // Java: Dfp cosh()
    pub fn cosh(&self) -> Dfp {
        // Java: return DfpMath.exp(this).add(DfpMath.exp(negate())).divide(2);
        todo!();
    }

    // Java: Dfp sinh()
    pub fn sinh(&self) -> Dfp {
        // Java: return DfpMath.exp(this).subtract(DfpMath.exp(negate())).divide(2);
        todo!();
    }

    // Java: Dfp tanh()
    pub fn tanh(&self) -> Dfp {
        // Java: final Dfp ePlus  = DfpMath.exp(this);
        // Java: final Dfp eMinus = DfpMath.exp(negate());
        // Java: return ePlus.subtract(eMinus).divide(ePlus.add(eMinus));
        todo!();
    }

    // Java: Dfp acosh()
    pub fn acosh(&self) -> Dfp {
        // Java: return multiply(this).subtract(getOne()).sqrt().add(this).log();
        todo!();
    }

    // Java: Dfp asinh()
    pub fn asinh(&self) -> Dfp {
        // Java: return multiply(this).add(getOne()).sqrt().add(this).log();
        todo!();
    }

    // Java: Dfp atanh()
    pub fn atanh(&self) -> Dfp {
        // Java: return getOne().add(this).divide(getOne().subtract(this)).log().divide(2);
        todo!();
    }

    // Java: Dfp linearCombination(final Dfp[] a, final Dfp[] b)
    pub fn linearCombination(&self, a: &[Dfp], b: &[Dfp]) -> Dfp {
        todo!();
    }

    // Java: Dfp linearCombination(final double[] a, final Dfp[] b)
    pub fn linearCombination_1(&self, a: &[f64], b: &[Dfp]) -> Dfp {
        todo!();
    }

    // Java: Dfp linearCombination(final Dfp a1, final Dfp b1, final Dfp a2, final Dfp b2)
    pub fn linearCombination_2(&self, a1: &Dfp, b1: &Dfp, a2: &Dfp, b2: &Dfp) -> Dfp {
        // Java: return a1.multiply(b1).add(a2.multiply(b2));
        todo!();
    }

    // Java: Dfp linearCombination(final double a1, final Dfp b1, final double a2, final Dfp b2)
    pub fn linearCombination_3(&self, a1: f64, b1: &Dfp, a2: f64, b2: &Dfp) -> Dfp {
        // Java: return b1.multiply(a1).add(b2.multiply(a2));
        todo!();
    }

    // Java: Dfp linearCombination()
    pub fn linearCombination_4(&self) -> Dfp {
        todo!();
    }

    // Java: Dfp linearCombination()
    pub fn linearCombination_5(&self) -> Dfp {
        todo!();
    }

    // Java: Dfp linearCombination()
    pub fn linearCombination_6(&self) -> Dfp {
        todo!();
    }

    // Java: Dfp linearCombination()
    pub fn linearCombination_7(&self) -> Dfp {
        todo!();
    }

}
