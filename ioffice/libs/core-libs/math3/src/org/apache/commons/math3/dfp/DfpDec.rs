use super::Dfp::{Dfp, DfpField};

// Java: public class can
pub struct can {
}

// Java: public class DfpDec
pub struct DfpDec {
}

// Java: public class can
impl can {
}

// Java: public class DfpDec
impl DfpDec {
    // Java: DfpDec(final DfpField factory)
    pub fn DfpDec(&self, factory: DfpField) {
        // Java: super(factory);
    }

    // Java: DfpDec(final DfpField factory, byte x)
    pub fn DfpDec_1(&self, factory: DfpField, x: i8) {
        // Java: super(factory, x);
    }

    // Java: DfpDec(final DfpField factory, int x)
    pub fn DfpDec_2(&self, factory: DfpField, x: i32) {
        // Java: super(factory, x);
    }

    // Java: DfpDec(final DfpField factory, long x)
    pub fn DfpDec_3(&self, factory: DfpField, x: i64) {
        // Java: super(factory, x);
    }

    // Java: DfpDec(final DfpField factory, double x)
    pub fn DfpDec_4(&self, factory: DfpField, x: f64) {
        // Java: super(factory, x);
        // Java: round(0);
    }

    // Java: DfpDec(final Dfp d)
    pub fn DfpDec_5(&self, d: &Dfp) {
        // Java: super(d);
        // Java: round(0);
    }

    // Java: DfpDec(final DfpField factory, final String s)
    pub fn DfpDec_6(&self, factory: DfpField, s: &str) {
        // Java: super(factory, s);
        // Java: round(0);
    }

    // Java: DfpDec(final DfpField factory, final byte sign, final byte nans)
    pub fn DfpDec_7(&self, factory: DfpField, sign: i8, nans: i8) {
        // Java: super(factory, sign, nans);
    }

    // Java: Dfp newInstance()
    pub fn newInstance(&self) -> Dfp {
        // Java: return new DfpDec(getField());
        todo!();
    }

    // Java: Dfp newInstance(final byte x)
    pub fn newInstance_1(&self, x: i8) -> Dfp {
        // Java: return new DfpDec(getField(), x);
        todo!();
    }

    // Java: Dfp newInstance(final int x)
    pub fn newInstance_2(&self, x: i32) -> Dfp {
        // Java: return new DfpDec(getField(), x);
        todo!();
    }

    // Java: Dfp newInstance(final long x)
    pub fn newInstance_3(&self, x: i64) -> Dfp {
        // Java: return new DfpDec(getField(), x);
        todo!();
    }

    // Java: Dfp newInstance(final double x)
    pub fn newInstance_4(&self, x: f64) -> Dfp {
        // Java: return new DfpDec(getField(), x);
        todo!();
    }

    // Java: Dfp newInstance(final Dfp d)
    pub fn newInstance_5(&self, d: &Dfp) -> Dfp {
        // Java: // make sure we don't mix number with different precision
        // Java: if (getField().getRadixDigits() != d.getField().getRadixDigits())
        {
            // Java: getField().setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: final Dfp result = newInstance(getZero());
            // Java: result.nans = QNAN;
            // Java: return dotrap(DfpField.FLAG_INVALID, "newInstance", d, result);
        }
        // Java: return new DfpDec(d);
        todo!();
    }

    // Java: Dfp newInstance(final String s)
    pub fn newInstance_6(&self, s: &str) -> Dfp {
        // Java: return new DfpDec(getField(), s);
        todo!();
    }

    // Java: Dfp newInstance(final byte sign, final byte nans)
    pub fn newInstance_7(&self, sign: i8, nans: i8) -> Dfp {
        // Java: return new DfpDec(getField(), sign, nans);
        todo!();
    }

    // Java: int getDecimalDigits()
    pub fn getDecimalDigits(&self) -> i32 {
        // Java: return getRadixDigits() * 4 - 3;
        todo!();
    }

    // Java: int round(int in)
    pub fn round(&self, r#in: i32) -> i32 {
        // Java: int msb = mant[mant.length-1];
        // Java: if (msb == 0)
        {
            // Java: // special case -- this == zero
            // Java: return 0;
        }
        // Java: int cmaxdigits = mant.length * 4;
        // Java: int lsbthreshold = 1000;
        // Java: while (lsbthreshold > msb)
        {
            // Java: lsbthreshold /= 10;
            // Java: cmaxdigits --;
        }
        // Java: final int digits = getDecimalDigits();
        // Java: final int lsbshift = cmaxdigits - digits;
        // Java: final int lsd = lsbshift / 4;
        // Java: lsbthreshold = 1;
        // Java: for (int i = 0; i < lsbshift % 4; i++)
        {
            // Java: lsbthreshold *= 10;
        }
        // Java: final int lsb = mant[lsd];
        // Java: if (lsbthreshold <= 1 && digits == 4 * mant.length - 3)
        {
            // Java: return super.round(in);
        }
        // Java: int discarded = in;  // not looking at this after this point
        // Java: final int n;
        // Java: if (lsbthreshold == 1)
        {
            // Java: // look to the next digit for rounding
            // Java: n = (mant[lsd-1] / 1000) % 10;
            // Java: mant[lsd-1] %= 1000;
            // Java: discarded |= mant[lsd-1];
        }
        // Java: else
        {
            // Java: n = (lsb * 10 / lsbthreshold) % 10;
            // Java: discarded |= lsb % (lsbthreshold/10);
        }
        // Java: for (int i = 0; i < lsd; i++)
        {
            // Java: discarded |= mant[i];    // need to know if there are any discarded bits
            // Java: mant[i] = 0;
        }
        // Java: mant[lsd] = lsb / lsbthreshold * lsbthreshold;
        // Java: final boolean inc;
        // Java: switch (getField().getRoundingMode())
        {
            // Java: case ROUND_DOWN:
                // Java: inc = false;
                // Java: break;
            // Java: case ROUND_UP:
                // Java: inc = (n != 0) || (discarded != 0); // round up if n!=0
                // Java: break;
            // Java: case ROUND_HALF_UP:
                // Java: inc = n >= 5;  // round half up
                // Java: break;
            // Java: case ROUND_HALF_DOWN:
                // Java: inc = n > 5;  // round half down
                // Java: break;
            // Java: case ROUND_HALF_EVEN:
                // Java: inc = (n > 5) ||
                      // Java: (n == 5 && discarded != 0) ||
                      // Java: (n == 5 && discarded == 0 && ((lsb / lsbthreshold) & 1) == 1);  // round half-even
                // Java: break;
            // Java: case ROUND_HALF_ODD:
                // Java: inc = (n > 5) ||
                      // Java: (n == 5 && discarded != 0) ||
                      // Java: (n == 5 && discarded == 0 && ((lsb / lsbthreshold) & 1) == 0);  // round half-odd
                // Java: break;
            // Java: case ROUND_CEIL:
                // Java: inc = (sign == 1) && (n != 0 || discarded != 0);  // round ceil
                // Java: break;
            // Java: case ROUND_FLOOR:
            // Java: default:
                // Java: inc = (sign == -1) && (n != 0 || discarded != 0);  // round floor
                // Java: break;
        }
        // Java: if (inc)
        {
            // Java: // increment if necessary
            // Java: int rh = lsbthreshold;
            // Java: for (int i = lsd; i < mant.length; i++)
            {
                // Java: final int r = mant[i] + rh;
                // Java: rh = r / RADIX;
                // Java: mant[i] = r % RADIX;
            }
            // Java: if (rh != 0)
            {
                // Java: shiftRight();
                // Java: mant[mant.length-1]=rh;
            }
        }
        // Java: // Check for exceptional cases and raise signals if necessary
        // Java: if (exp < MIN_EXP)
        {
            // Java: // Gradual Underflow
            // Java: getField().setIEEEFlagsBits(DfpField.FLAG_UNDERFLOW);
            // Java: return DfpField.FLAG_UNDERFLOW;
        }
        // Java: if (exp > MAX_EXP)
        {
            // Java: // Overflow
            // Java: getField().setIEEEFlagsBits(DfpField.FLAG_OVERFLOW);
            // Java: return DfpField.FLAG_OVERFLOW;
        }
        // Java: if (n != 0 || discarded != 0)
        {
            // Java: // Inexact
            // Java: getField().setIEEEFlagsBits(DfpField.FLAG_INEXACT);
            // Java: return DfpField.FLAG_INEXACT;
        }
        // Java: return 0;
        todo!();
    }

    // Java: Dfp nextAfter(Dfp x)
    pub fn nextAfter(&self, x: &Dfp) -> Dfp {
        // Java: final String trapName = "nextAfter";
        // Java: // make sure we don't mix number with different precision
        // Java: if (getField().getRadixDigits() != x.getField().getRadixDigits())
        {
            // Java: getField().setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: final Dfp result = newInstance(getZero());
            // Java: result.nans = QNAN;
            // Java: return dotrap(DfpField.FLAG_INVALID, trapName, x, result);
        }
        // Java: boolean up = false;
        // Java: Dfp result;
        // Java: Dfp inc;
        // Java: // if this is greater than x
        // Java: if (this.lessThan(x))
        {
            // Java: up = true;
        }
        // Java: if (equals(x))
        {
            // Java: return newInstance(x);
        }
        // Java: if (lessThan(getZero()))
        {
            // Java: up = !up;
        }
        // Java: if (up)
        {
            // Java: inc = power10(intLog10() - getDecimalDigits() + 1);
            // Java: inc = copysign(inc, this);
            // Java: if (this.equals(getZero()))
            {
                // Java: inc = power10K(MIN_EXP-mant.length-1);
            }
            // Java: if (inc.equals(getZero()))
            {
                // Java: result = copysign(newInstance(getZero()), this);
            }
            // Java: else
            {
                // Java: result = add(inc);
            }
        }
        // Java: else
        {
            // Java: inc = power10(intLog10());
            // Java: inc = copysign(inc, this);
            // Java: if (this.equals(inc))
            {
                // Java: inc = inc.divide(power10(getDecimalDigits()));
            }
            // Java: else
            {
                // Java: inc = inc.divide(power10(getDecimalDigits() - 1));
            }
            // Java: if (this.equals(getZero()))
            {
                // Java: inc = power10K(MIN_EXP-mant.length-1);
            }
            // Java: if (inc.equals(getZero()))
            {
                // Java: result = copysign(newInstance(getZero()), this);
            }
            // Java: else
            {
                // Java: result = subtract(inc);
            }
        }
        // Java: if (result.classify() == INFINITE && this.classify() != INFINITE)
        {
            // Java: getField().setIEEEFlagsBits(DfpField.FLAG_INEXACT);
            // Java: result = dotrap(DfpField.FLAG_INEXACT, trapName, x, result);
        }
        // Java: if (result.equals(getZero()) && this.equals(getZero()) == false)
        {
            // Java: getField().setIEEEFlagsBits(DfpField.FLAG_INEXACT);
            // Java: result = dotrap(DfpField.FLAG_INEXACT, trapName, x, result);
        }
        // Java: return result;
        todo!();
    }

}
