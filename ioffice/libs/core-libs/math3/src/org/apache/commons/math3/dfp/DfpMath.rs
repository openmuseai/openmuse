// Java field: String POW_TRAP
use super::Dfp::{Dfp, DfpField};

pub(crate) const POW_TRAP: &str = "pow";

// Java: public class DfpMath
pub struct DfpMath {
}

// Java: public class DfpMath
impl DfpMath {
    // Java: DfpMath()
    pub fn DfpMath(&self) {
    }

    // Java: static Dfp[] split(final DfpField field, final String a)
    pub fn split(&self, field: DfpField, a: &str) -> Vec<Dfp> {
        // Java: Dfp result[] = new Dfp[2];
        // Java: char[] buf;
        // Java: boolean leading = true;
        // Java: int sp = 0;
        // Java: int sig = 0;
        // Java: buf = new char[a.length()];
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
            // Java: if (sig == (field.getRadixDigits() / 2) * 4)
            {
                // Java: sp = i;
                // Java: break;
            }
            // Java: if (buf[i] >= '0' && buf[i] <= '9' && !leading)
            {
                // Java: sig ++;
            }
        }
        // Java: result[0] = field.newDfp(new String(buf, 0, sp));
        // Java: for (int i = 0; i < buf.length; i++)
        {
            // Java: buf[i] = a.charAt(i);
            // Java: if (buf[i] >= '0' && buf[i] <= '9' && i < sp)
            {
                // Java: buf[i] = '0';
            }
        }
        // Java: result[1] = field.newDfp(new String(buf));
        // Java: return result;
        todo!();
    }

    // Java: static Dfp[] split(final Dfp a)
    pub fn split_1(&self, a: &Dfp) -> Vec<Dfp> {
        // Java: final Dfp[] result = new Dfp[2];
        // Java: final Dfp shift = a.multiply(a.power10K(a.getRadixDigits() / 2));
        // Java: result[0] = a.add(shift).subtract(shift);
        // Java: result[1] = a.subtract(result[0]);
        // Java: return result;
        todo!();
    }

    // Java: static Dfp[] splitMult(final Dfp[] a, final Dfp[] b)
    pub fn splitMult(&self, a: &[Dfp], b: &[Dfp]) -> Vec<Dfp> {
        // Java: final Dfp[] result = new Dfp[2];
        // Java: result[1] = a[0].getZero();
        // Java: result[0] = a[0].multiply(b[0]);
        // Java: /* If result[0] is infinite or zero, don't compute result[1].
         // Java: * Attempting to do so may produce NaNs.
         // Java: */
        // Java: if (result[0].classify() == Dfp.INFINITE || result[0].equals(result[1]))
        {
            // Java: return result;
        }
        // Java: result[1] = a[0].multiply(b[1]).add(a[1].multiply(b[0])).add(a[1].multiply(b[1]));
        // Java: return result;
        todo!();
    }

    // Java: static Dfp[] splitDiv(final Dfp[] a, final Dfp[] b)
    pub fn splitDiv(&self, a: &[Dfp], b: &[Dfp]) -> Vec<Dfp> {
        // Java: final Dfp[] result;
        // Java: result = new Dfp[2];
        // Java: result[0] = a[0].divide(b[0]);
        // Java: result[1] = a[1].multiply(b[0]).subtract(a[0].multiply(b[1]));
        // Java: result[1] = result[1].divide(b[0].multiply(b[0]).add(b[0].multiply(b[1])));
        // Java: return result;
        todo!();
    }

    // Java: static Dfp splitPow(final Dfp[] base, int a)
    pub fn splitPow(&self, base: &[Dfp], a: i32) -> Dfp {
        // Java: boolean invert = false;
        // Java: Dfp[] r = new Dfp[2];
        // Java: Dfp[] result = new Dfp[2];
        // Java: result[0] = base[0].getOne();
        // Java: result[1] = base[0].getZero();
        // Java: if (a == 0)
        {
            // Java: // Special case a = 0
            // Java: return result[0].add(result[1]);
        }
        // Java: if (a < 0)
        {
            // Java: // If a is less than zero
            // Java: invert = true;
            // Java: a = -a;
        }
        // Java: // Exponentiate by successive squaring
        // Java: do
        {
            // Java: r[0] = new Dfp(base[0]);
            // Java: r[1] = new Dfp(base[1]);
            // Java: int trial = 1;
            // Java: int prevtrial;
            // Java: while (true)
            {
                // Java: prevtrial = trial;
                // Java: trial *= 2;
                // Java: if (trial > a)
                {
                    // Java: break;
                }
                // Java: r = splitMult(r, r);
            }
            // Java: trial = prevtrial;
            // Java: a -= trial;
            // Java: result = splitMult(result, r);
        }
        // Java: result[0] = result[0].add(result[1]);
        // Java: if (invert)
        {
            // Java: result[0] = base[0].getOne().divide(result[0]);
        }
        // Java: return result[0];
        todo!();
    }

    // Java: static Dfp pow(Dfp base, int a)
    pub fn pow(&self, base: &Dfp, a: i32) -> Dfp {
        // Java: boolean invert = false;
        // Java: Dfp result = base.getOne();
        // Java: if (a == 0)
        {
            // Java: // Special case
            // Java: return result;
        }
        // Java: if (a < 0)
        {
            // Java: invert = true;
            // Java: a = -a;
        }
        // Java: // Exponentiate by successive squaring
        // Java: do
        {
            // Java: Dfp r = new Dfp(base);
            // Java: Dfp prevr;
            // Java: int trial = 1;
            // Java: int prevtrial;
            // Java: do
            {
                // Java: prevr = new Dfp(r);
                // Java: prevtrial = trial;
                // Java: r = r.multiply(r);
                // Java: trial *= 2;
            }
            // Java: r = prevr;
            // Java: trial = prevtrial;
            // Java: a -= trial;
            // Java: result = result.multiply(r);
        }
        // Java: if (invert)
        {
            // Java: result = base.getOne().divide(result);
        }
        // Java: return base.newInstance(result);
        todo!();
    }

    // Java: static Dfp exp(final Dfp a)
    pub fn exp(&self, a: &Dfp) -> Dfp {
        // Java: final Dfp inta = a.rint();
        // Java: final Dfp fraca = a.subtract(inta);
        // Java: final int ia = inta.intValue();
        // Java: if (ia > 2147483646)
        {
            // Java: // return +Infinity
            // Java: return a.newInstance((byte)1, Dfp.INFINITE);
        }
        // Java: if (ia < -2147483646)
        {
            // Java: // return 0;
            // Java: return a.newInstance();
        }
        // Java: final Dfp einta = splitPow(a.getField().getESplit(), ia);
        // Java: final Dfp efraca = expInternal(fraca);
        // Java: return einta.multiply(efraca);
        todo!();
    }

    // Java: static Dfp expInternal(final Dfp a)
    pub fn expInternal(&self, a: &Dfp) -> Dfp {
        // Java: Dfp y = a.getOne();
        // Java: Dfp x = a.getOne();
        // Java: Dfp fact = a.getOne();
        // Java: Dfp py = new Dfp(y);
        // Java: for (int i = 1; i < 90; i++)
        {
            // Java: x = x.multiply(a);
            // Java: fact = fact.divide(i);
            // Java: y = y.add(x.multiply(fact));
            // Java: if (y.equals(py))
            {
                // Java: break;
            }
            // Java: py = new Dfp(y);
        }
        // Java: return y;
        todo!();
    }

    // Java: static Dfp log(Dfp a)
    pub fn log(&self, a: &Dfp) -> Dfp {
        // Java: int lr;
        // Java: Dfp x;
        // Java: int ix;
        // Java: int p2 = 0;
        // Java: // Check the arguments somewhat here
        // Java: if (a.equals(a.getZero()) || a.lessThan(a.getZero()) || a.isNaN())
        {
            // Java: // negative, zero or NaN
            // Java: a.getField().setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: return a.dotrap(DfpField.FLAG_INVALID, "ln", a, a.newInstance((byte)1, Dfp.QNAN));
        }
        // Java: if (a.classify() == Dfp.INFINITE)
        {
            // Java: return a;
        }
        // Java: x = new Dfp(a);
        // Java: lr = x.log10K();
        // Java: x = x.divide(pow(a.newInstance(10000), lr));  /* This puts x in the range 0-10000 */
        // Java: ix = x.floor().intValue();
        // Java: while (ix > 2)
        {
            // Java: ix >>= 1;
            // Java: p2++;
        }
        // Java: Dfp[] spx = split(x);
        // Java: Dfp[] spy = new Dfp[2];
        // Java: spy[0] = pow(a.getTwo(), p2);          // use spy[0] temporarily as a divisor
        // Java: spx[0] = spx[0].divide(spy[0]);
        // Java: spx[1] = spx[1].divide(spy[0]);
        // Java: spy[0] = a.newInstance("1.33333");    // Use spy[0] for comparison
        // Java: while (spx[0].add(spx[1]).greaterThan(spy[0]))
        {
            // Java: spx[0] = spx[0].divide(2);
            // Java: spx[1] = spx[1].divide(2);
            // Java: p2++;
        }
        // Java: // X is now in the range of 2/3 < x < 4/3
        // Java: Dfp[] spz = logInternal(spx);
        // Java: spx[0] = a.newInstance(new StringBuilder().append(p2+4*lr).toString());
        // Java: spx[1] = a.getZero();
        // Java: spy = splitMult(a.getField().getLn2Split(), spx);
        // Java: spz[0] = spz[0].add(spy[0]);
        // Java: spz[1] = spz[1].add(spy[1]);
        // Java: spx[0] = a.newInstance(new StringBuilder().append(4*lr).toString());
        // Java: spx[1] = a.getZero();
        // Java: spy = splitMult(a.getField().getLn5Split(), spx);
        // Java: spz[0] = spz[0].add(spy[0]);
        // Java: spz[1] = spz[1].add(spy[1]);
        // Java: return a.newInstance(spz[0].add(spz[1]));
        todo!();
    }

    // Java: static Dfp[] logInternal(final Dfp a[])
    pub fn logInternal(&self) -> Vec<Dfp> {
        // Java: /* Now we want to compute x = (a-1)/(a+1) but this is prone to
         // Java: * loss of precision.  So instead, compute x = (a/4 - 1/4) / (a/4 + 1/4)
         // Java: */
        // Java: Dfp t = a[0].divide(4).add(a[1].divide(4));
        // Java: Dfp x = t.add(a[0].newInstance("-0.25")).divide(t.add(a[0].newInstance("0.25")));
        // Java: Dfp y = new Dfp(x);
        // Java: Dfp num = new Dfp(x);
        // Java: Dfp py = new Dfp(y);
        // Java: int den = 1;
        // Java: for (int i = 0; i < 10000; i++)
        {
            // Java: num = num.multiply(x);
            // Java: num = num.multiply(x);
            // Java: den += 2;
            // Java: t = num.divide(den);
            // Java: y = y.add(t);
            // Java: if (y.equals(py))
            {
                // Java: break;
            }
            // Java: py = new Dfp(y);
        }
        // Java: y = y.multiply(a[0].getTwo());
        // Java: return split(y);
        todo!();
    }

    // Java: static Dfp pow(Dfp x, final Dfp y)
    pub fn pow_1(&self, x: &Dfp, y: &Dfp) -> Dfp {
        // Java: // make sure we don't mix number with different precision
        // Java: if (x.getField().getRadixDigits() != y.getField().getRadixDigits())
        {
            // Java: x.getField().setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: final Dfp result = x.newInstance(x.getZero());
            // Java: result.nans = Dfp.QNAN;
            // Java: return x.dotrap(DfpField.FLAG_INVALID, POW_TRAP, x, result);
        }
        // Java: final Dfp zero = x.getZero();
        // Java: final Dfp one  = x.getOne();
        // Java: final Dfp two  = x.getTwo();
        // Java: boolean invert = false;
        // Java: int ui;
        // Java: /* Check for special cases */
        // Java: if (y.equals(zero))
        {
            // Java: return x.newInstance(one);
        }
        // Java: if (y.equals(one))
        {
            // Java: if (x.isNaN())
            {
                // Java: // Test for NaNs
                // Java: x.getField().setIEEEFlagsBits(DfpField.FLAG_INVALID);
                // Java: return x.dotrap(DfpField.FLAG_INVALID, POW_TRAP, x, x);
            }
            // Java: return x;
        }
        // Java: if (x.isNaN() || y.isNaN())
        {
            // Java: // Test for NaNs
            // Java: x.getField().setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: return x.dotrap(DfpField.FLAG_INVALID, POW_TRAP, x, x.newInstance((byte)1, Dfp.QNAN));
        }
        // Java: // X == 0
        // Java: if (x.equals(zero))
        {
            // Java: if (Dfp.copysign(one, x).greaterThan(zero))
            {
                // Java: // X == +0
                // Java: if (y.greaterThan(zero))
                {
                    // Java: return x.newInstance(zero);
                }
                // Java: else
                {
                    // Java: return x.newInstance(x.newInstance((byte)1, Dfp.INFINITE));
                }
            }
            // Java: else
            {
                // Java: // X == -0
                // Java: if (y.classify() == Dfp.FINITE && y.rint().equals(y) && !y.remainder(two).equals(zero))
                {
                    // Java: // If y is odd integer
                    // Java: if (y.greaterThan(zero))
                    {
                        // Java: return x.newInstance(zero.negate());
                    }
                    // Java: else
                    {
                        // Java: return x.newInstance(x.newInstance((byte)-1, Dfp.INFINITE));
                    }
                }
                // Java: else
                {
                    // Java: // Y is not odd integer
                    // Java: if (y.greaterThan(zero))
                    {
                        // Java: return x.newInstance(zero);
                    }
                    // Java: else
                    {
                        // Java: return x.newInstance(x.newInstance((byte)1, Dfp.INFINITE));
                    }
                }
            }
        }
        // Java: if (x.lessThan(zero))
        {
            // Java: // Make x positive, but keep track of it
            // Java: x = x.negate();
            // Java: invert = true;
        }
        // Java: if (x.greaterThan(one) && y.classify() == Dfp.INFINITE)
        {
            // Java: if (y.greaterThan(zero))
            {
                // Java: return y;
            }
            // Java: else
            {
                // Java: return x.newInstance(zero);
            }
        }
        // Java: if (x.lessThan(one) && y.classify() == Dfp.INFINITE)
        {
            // Java: if (y.greaterThan(zero))
            {
                // Java: return x.newInstance(zero);
            }
            // Java: else
            {
                // Java: return x.newInstance(Dfp.copysign(y, one));
            }
        }
        // Java: if (x.equals(one) && y.classify() == Dfp.INFINITE)
        {
            // Java: x.getField().setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: return x.dotrap(DfpField.FLAG_INVALID, POW_TRAP, x, x.newInstance((byte)1, Dfp.QNAN));
        }
        // Java: if (x.classify() == Dfp.INFINITE)
        {
            // Java: // x = +/- inf
            // Java: if (invert)
            {
                // Java: // negative infinity
                // Java: if (y.classify() == Dfp.FINITE && y.rint().equals(y) && !y.remainder(two).equals(zero))
                {
                    // Java: // If y is odd integer
                    // Java: if (y.greaterThan(zero))
                    {
                        // Java: return x.newInstance(x.newInstance((byte)-1, Dfp.INFINITE));
                    }
                    // Java: else
                    {
                        // Java: return x.newInstance(zero.negate());
                    }
                }
                // Java: else
                {
                    // Java: // Y is not odd integer
                    // Java: if (y.greaterThan(zero))
                    {
                        // Java: return x.newInstance(x.newInstance((byte)1, Dfp.INFINITE));
                    }
                    // Java: else
                    {
                        // Java: return x.newInstance(zero);
                    }
                }
            }
            // Java: else
            {
                // Java: // positive infinity
                // Java: if (y.greaterThan(zero))
                {
                    // Java: return x;
                }
                // Java: else
                {
                    // Java: return x.newInstance(zero);
                }
            }
        }
        // Java: if (invert && !y.rint().equals(y))
        {
            // Java: x.getField().setIEEEFlagsBits(DfpField.FLAG_INVALID);
            // Java: return x.dotrap(DfpField.FLAG_INVALID, POW_TRAP, x, x.newInstance((byte)1, Dfp.QNAN));
        }
        // Java: // End special cases
        // Java: Dfp r;
        // Java: if (y.lessThan(x.newInstance(100000000)) && y.greaterThan(x.newInstance(-100000000)))
        {
            // Java: final Dfp u = y.rint();
            // Java: ui = u.intValue();
            // Java: final Dfp v = y.subtract(u);
            // Java: if (v.unequal(zero))
            {
                // Java: final Dfp a = v.multiply(log(x));
                // Java: final Dfp b = a.divide(x.getField().getLn2()).rint();
                // Java: final Dfp c = a.subtract(b.multiply(x.getField().getLn2()));
                // Java: r = splitPow(split(x), ui);
                // Java: r = r.multiply(pow(two, b.intValue()));
                // Java: r = r.multiply(exp(c));
            }
            // Java: else
            {
                // Java: r = splitPow(split(x), ui);
            }
        }
        // Java: else
        {
            // Java: // very large exponent.  |y| > 1e8
            // Java: r = exp(log(x).multiply(y));
        }
        // Java: if (invert && y.rint().equals(y) && !y.remainder(two).equals(zero))
        {
            // Java: // if y is odd integer
            // Java: r = r.negate();
        }
        // Java: return x.newInstance(r);
        todo!();
    }

    // Java: static Dfp sinInternal(Dfp a[])
    pub fn sinInternal(&self) -> Dfp {
        // Java: Dfp c = a[0].add(a[1]);
        // Java: Dfp y = c;
        // Java: c = c.multiply(c);
        // Java: Dfp x = y;
        // Java: Dfp fact = a[0].getOne();
        // Java: Dfp py = new Dfp(y);
        // Java: for (int i = 3; i < 90; i += 2)
        {
            // Java: x = x.multiply(c);
            // Java: x = x.negate();
            // Java: fact = fact.divide((i-1)*i);  // 1 over fact
            // Java: y = y.add(x.multiply(fact));
            // Java: if (y.equals(py))
            {
                // Java: break;
            }
            // Java: py = new Dfp(y);
        }
        // Java: return y;
        todo!();
    }

    // Java: static Dfp cosInternal(Dfp a[])
    pub fn cosInternal(&self) -> Dfp {
        // Java: final Dfp one = a[0].getOne();
        // Java: Dfp x = one;
        // Java: Dfp y = one;
        // Java: Dfp c = a[0].add(a[1]);
        // Java: c = c.multiply(c);
        // Java: Dfp fact = one;
        // Java: Dfp py = new Dfp(y);
        // Java: for (int i = 2; i < 90; i += 2)
        {
            // Java: x = x.multiply(c);
            // Java: x = x.negate();
            // Java: fact = fact.divide((i - 1) * i);  // 1 over fact
            // Java: y = y.add(x.multiply(fact));
            // Java: if (y.equals(py))
            {
                // Java: break;
            }
            // Java: py = new Dfp(y);
        }
        // Java: return y;
        todo!();
    }

    // Java: static Dfp sin(final Dfp a)
    pub fn sin(&self, a: &Dfp) -> Dfp {
        // Java: final Dfp pi = a.getField().getPi();
        // Java: final Dfp zero = a.getField().getZero();
        // Java: boolean neg = false;
        // Java: /* First reduce the argument to the range of +/- PI */
        // Java: Dfp x = a.remainder(pi.multiply(2));
        // Java: /* if x < 0 then apply identity sin(-x) = -sin(x) */
        // Java: /* This puts x in the range 0 < x < PI            */
        // Java: if (x.lessThan(zero))
        {
            // Java: x = x.negate();
            // Java: neg = true;
        }
        // Java: /* Since sine(x) = sine(pi - x) we can reduce the range to
         // Java: * 0 < x < pi/2
         // Java: */
        // Java: if (x.greaterThan(pi.divide(2)))
        {
            // Java: x = pi.subtract(x);
        }
        // Java: Dfp y;
        // Java: if (x.lessThan(pi.divide(4)))
        {
            // Java: y = sinInternal(split(x));
        }
        // Java: else
        {
            // Java: final Dfp c[] = new Dfp[2];
            // Java: final Dfp[] piSplit = a.getField().getPiSplit();
            // Java: c[0] = piSplit[0].divide(2).subtract(x);
            // Java: c[1] = piSplit[1].divide(2);
            // Java: y = cosInternal(c);
        }
        // Java: if (neg)
        {
            // Java: y = y.negate();
        }
        // Java: return a.newInstance(y);
        todo!();
    }

    // Java: static Dfp cos(Dfp a)
    pub fn cos(&self, a: &Dfp) -> Dfp {
        // Java: final Dfp pi = a.getField().getPi();
        // Java: final Dfp zero = a.getField().getZero();
        // Java: boolean neg = false;
        // Java: /* First reduce the argument to the range of +/- PI */
        // Java: Dfp x = a.remainder(pi.multiply(2));
        // Java: /* if x < 0 then apply identity cos(-x) = cos(x) */
        // Java: /* This puts x in the range 0 < x < PI           */
        // Java: if (x.lessThan(zero))
        {
            // Java: x = x.negate();
        }
        // Java: /* Since cos(x) = -cos(pi - x) we can reduce the range to
         // Java: * 0 < x < pi/2
         // Java: */
        // Java: if (x.greaterThan(pi.divide(2)))
        {
            // Java: x = pi.subtract(x);
            // Java: neg = true;
        }
        // Java: Dfp y;
        // Java: if (x.lessThan(pi.divide(4)))
        {
            // Java: Dfp c[] = new Dfp[2];
            // Java: c[0] = x;
            // Java: c[1] = zero;
            // Java: y = cosInternal(c);
        }
        // Java: else
        {
            // Java: final Dfp c[] = new Dfp[2];
            // Java: final Dfp[] piSplit = a.getField().getPiSplit();
            // Java: c[0] = piSplit[0].divide(2).subtract(x);
            // Java: c[1] = piSplit[1].divide(2);
            // Java: y = sinInternal(c);
        }
        // Java: if (neg)
        {
            // Java: y = y.negate();
        }
        // Java: return a.newInstance(y);
        todo!();
    }

    // Java: static Dfp tan(final Dfp a)
    pub fn tan(&self, a: &Dfp) -> Dfp {
        // Java: return sin(a).divide(cos(a));
        todo!();
    }

    // Java: static Dfp atanInternal(final Dfp a)
    pub fn atanInternal(&self, a: &Dfp) -> Dfp {
        // Java: Dfp y = new Dfp(a);
        // Java: Dfp x = new Dfp(y);
        // Java: Dfp py = new Dfp(y);
        // Java: for (int i = 3; i < 90; i += 2)
        {
            // Java: x = x.multiply(a);
            // Java: x = x.multiply(a);
            // Java: x = x.negate();
            // Java: y = y.add(x.divide(i));
            // Java: if (y.equals(py))
            {
                // Java: break;
            }
            // Java: py = new Dfp(y);
        }
        // Java: return y;
        todo!();
    }

    // Java: static Dfp atan(final Dfp a)
    pub fn atan(&self, a: &Dfp) -> Dfp {
        // Java: final Dfp   zero      = a.getField().getZero();
        // Java: final Dfp   one       = a.getField().getOne();
        // Java: final Dfp[] sqr2Split = a.getField().getSqr2Split();
        // Java: final Dfp[] piSplit   = a.getField().getPiSplit();
        // Java: boolean recp = false;
        // Java: boolean neg = false;
        // Java: boolean sub = false;
        // Java: final Dfp ty = sqr2Split[0].subtract(one).add(sqr2Split[1]);
        // Java: Dfp x = new Dfp(a);
        // Java: if (x.lessThan(zero))
        {
            // Java: neg = true;
            // Java: x = x.negate();
        }
        // Java: if (x.greaterThan(one))
        {
            // Java: recp = true;
            // Java: x = one.divide(x);
        }
        // Java: if (x.greaterThan(ty))
        {
            // Java: Dfp sty[] = new Dfp[2];
            // Java: sub = true;
            // Java: sty[0] = sqr2Split[0].subtract(one);
            // Java: sty[1] = sqr2Split[1];
            // Java: Dfp[] xs = split(x);
            // Java: Dfp[] ds = splitMult(xs, sty);
            // Java: ds[0] = ds[0].add(one);
            // Java: xs[0] = xs[0].subtract(sty[0]);
            // Java: xs[1] = xs[1].subtract(sty[1]);
            // Java: xs = splitDiv(xs, ds);
            // Java: x = xs[0].add(xs[1]);
            // Java: //x = x.subtract(ty).divide(dfp.one.add(x.multiply(ty)));
        }
        // Java: Dfp y = atanInternal(x);
        // Java: if (sub)
        {
            // Java: y = y.add(piSplit[0].divide(8)).add(piSplit[1].divide(8));
        }
        // Java: if (recp)
        {
            // Java: y = piSplit[0].divide(2).subtract(y).add(piSplit[1].divide(2));
        }
        // Java: if (neg)
        {
            // Java: y = y.negate();
        }
        // Java: return a.newInstance(y);
        todo!();
    }

    // Java: static Dfp asin(final Dfp a)
    pub fn asin(&self, a: &Dfp) -> Dfp {
        // Java: return atan(a.divide(a.getOne().subtract(a.multiply(a)).sqrt()));
        todo!();
    }

    // Java: static Dfp acos(Dfp a)
    pub fn acos(&self, a: &Dfp) -> Dfp {
        // Java: Dfp result;
        // Java: boolean negative = false;
        // Java: if (a.lessThan(a.getZero()))
        {
            // Java: negative = true;
        }
        // Java: a = Dfp.copysign(a, a.getOne());  // absolute value
        // Java: result = atan(a.getOne().subtract(a.multiply(a)).sqrt().divide(a));
        // Java: if (negative)
        {
            // Java: result = a.getField().getPi().subtract(result);
        }
        // Java: return a.newInstance(result);
        todo!();
    }

}
