// Java: import java.io.PrintStream
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;

// Java field: long HEX_40000000
pub(crate) const HEX_40000000: i64 = 0;
// Java field: String TABLE_START_DECL
pub(crate) const TABLE_START_DECL: &str = "";
// Java field: String TABLE_END_DECL
pub(crate) const TABLE_END_DECL: &str = "";

// Java: public class FastMathCalc
pub struct FastMathCalc {
}

// Java: public class FastMathCalc
impl FastMathCalc {
    // Java: FastMathCalc()
    pub fn FastMathCalc(&self) {
    }

    // Java: static void buildSinCosTables()
    pub fn buildSinCosTables(&self) {
    }

    // Java: static double slowCos(final double x, final double result[])
    pub fn slowCos(&self) -> f64 {
        // Java: final double xs[] = new double[2];
        // Java: final double ys[] = new double[2];
        // Java: final double facts[] = new double[2];
        // Java: final double as[] = new double[2];
        // Java: split(x, xs);
        // Java: ys[0] = ys[1] = 0.0;
        // Java: for (int i = FACT.length-1; i >= 0; i--)
        {
            // Java: splitMult(xs, ys, as);
            // Java: ys[0] = as[0]; ys[1] = as[1];
            // Java: if ( (i & 1) != 0) { // skip odd entries
                // Java: continue;
            // Java: split(FACT[i], as);
            // Java: splitReciprocal(as, facts);
            // Java: if ( (i & 2) != 0 ) { // alternate terms are negative
                // Java: facts[0] = -facts[0];
                // Java: facts[1] = -facts[1];
            // Java: splitAdd(ys, facts, as);
            // Java: ys[0] = as[0]; ys[1] = as[1];
        }
        // Java: if (result != null)
        {
            // Java: result[0] = ys[0];
            // Java: result[1] = ys[1];
        }
        // Java: return ys[0] + ys[1];
        todo!()
    }

    // Java: static double slowSin(final double x, final double result[])
    pub fn slowSin(&self) -> f64 {
        // Java: final double xs[] = new double[2];
        // Java: final double ys[] = new double[2];
        // Java: final double facts[] = new double[2];
        // Java: final double as[] = new double[2];
        // Java: split(x, xs);
        // Java: ys[0] = ys[1] = 0.0;
        // Java: for (int i = FACT.length-1; i >= 0; i--)
        {
            // Java: splitMult(xs, ys, as);
            // Java: ys[0] = as[0]; ys[1] = as[1];
            // Java: if ( (i & 1) == 0) { // Ignore even numbers
                // Java: continue;
            // Java: split(FACT[i], as);
            // Java: splitReciprocal(as, facts);
            // Java: if ( (i & 2) != 0 ) { // alternate terms are negative
                // Java: facts[0] = -facts[0];
                // Java: facts[1] = -facts[1];
            // Java: splitAdd(ys, facts, as);
            // Java: ys[0] = as[0]; ys[1] = as[1];
        }
        // Java: if (result != null)
        {
            // Java: result[0] = ys[0];
            // Java: result[1] = ys[1];
        }
        // Java: return ys[0] + ys[1];
        todo!()
    }

    // Java: static double slowexp(final double x, final double result[])
    pub fn slowexp(&self) -> f64 {
        // Java: final double xs[] = new double[2];
        // Java: final double ys[] = new double[2];
        // Java: final double facts[] = new double[2];
        // Java: final double as[] = new double[2];
        // Java: split(x, xs);
        // Java: ys[0] = ys[1] = 0.0;
        // Java: for (int i = FACT.length-1; i >= 0; i--)
        {
            // Java: splitMult(xs, ys, as);
            // Java: ys[0] = as[0];
            // Java: ys[1] = as[1];
            // Java: split(FACT[i], as);
            // Java: splitReciprocal(as, facts);
            // Java: splitAdd(ys, facts, as);
            // Java: ys[0] = as[0];
            // Java: ys[1] = as[1];
        }
        // Java: if (result != null)
        {
            // Java: result[0] = ys[0];
            // Java: result[1] = ys[1];
        }
        // Java: return ys[0] + ys[1];
        todo!()
    }

    // Java: static void split(final double d, final double split[])
    pub fn split(&self) {
        // Java: if (d < 8e298 && d > -8e298)
        {
            // Java: final double a = d * HEX_40000000;
            // Java: split[0] = (d + a) - a;
            // Java: split[1] = d - split[0];
        }
        // Java: else
        {
            // Java: final double a = d * 9.31322574615478515625E-10;
            // Java: split[0] = (d + a - d) * HEX_40000000;
            // Java: split[1] = d - split[0];
        }
    }

    // Java: static void resplit(final double a[])
    pub fn resplit(&self) {
        // Java: final double c = a[0] + a[1];
        // Java: final double d = -(c - a[0] - a[1]);
        // Java: if (c < 8e298 && c > -8e298) { // MAGIC NUMBER
            // Java: double z = c * HEX_40000000;
            // Java: a[0] = (c + z) - z;
            // Java: a[1] = c - a[0] + d;
        // Java: else
        {
            // Java: double z = c * 9.31322574615478515625E-10;
            // Java: a[0] = (c + z - c) * HEX_40000000;
            // Java: a[1] = c - a[0] + d;
        }
    }

    // Java: static void splitMult(double a[], double b[], double ans[])
    pub fn splitMult(&self) {
        // Java: ans[0] = a[0] * b[0];
        // Java: ans[1] = a[0] * b[1] + a[1] * b[0] + a[1] * b[1];
        // Java: /* Resplit */
        // Java: resplit(ans);
    }

    // Java: static void splitAdd(final double a[], final double b[], final double ans[])
    pub fn splitAdd(&self) {
        // Java: ans[0] = a[0] + b[0];
        // Java: ans[1] = a[1] + b[1];
        // Java: resplit(ans);
    }

    // Java: static void splitReciprocal(final double in[], final double result[])
    pub fn splitReciprocal(&self) {
        // Java: final double b = 1.0/4194304.0;
        // Java: final double a = 1.0 - b;
        // Java: if (in[0] == 0.0)
        {
            // Java: in[0] = in[1];
            // Java: in[1] = 0.0;
        }
        // Java: result[0] = a / in[0];
        // Java: result[1] = (b*in[0]-a*in[1]) / (in[0]*in[0] + in[0]*in[1]);
        // Java: if (result[1] != result[1]) { // can happen if result[1] is NAN
            // Java: result[1] = 0.0;
        // Java: /* Resplit */
        // Java: resplit(result);
        // Java: for (int i = 0; i < 2; i++)
        {
            // Java: /* this may be overkill, probably once is enough */
            // Java: double err = 1.0 - result[0] * in[0] - result[0] * in[1] -
            // Java: result[1] * in[0] - result[1] * in[1];
            // Java: /*err = 1.0 - err; */
            // Java: err *= result[0] + result[1];
            // Java: /*printf("err = %16e\n", err); */
            // Java: result[1] += err;
        }
    }

    // Java: static void quadMult(final double a[], final double b[], final double result[])
    pub fn quadMult(&self) {
        // Java: final double xs[] = new double[2];
        // Java: final double ys[] = new double[2];
        // Java: final double zs[] = new double[2];
        // Java: /* a[0] * b[0] */
        // Java: split(a[0], xs);
        // Java: split(b[0], ys);
        // Java: splitMult(xs, ys, zs);
        // Java: result[0] = zs[0];
        // Java: result[1] = zs[1];
        // Java: /* a[0] * b[1] */
        // Java: split(b[1], ys);
        // Java: splitMult(xs, ys, zs);
        // Java: double tmp = result[0] + zs[0];
        // Java: result[1] -= tmp - result[0] - zs[0];
        // Java: result[0] = tmp;
        // Java: tmp = result[0] + zs[1];
        // Java: result[1] -= tmp - result[0] - zs[1];
        // Java: result[0] = tmp;
        // Java: /* a[1] * b[0] */
        // Java: split(a[1], xs);
        // Java: split(b[0], ys);
        // Java: splitMult(xs, ys, zs);
        // Java: tmp = result[0] + zs[0];
        // Java: result[1] -= tmp - result[0] - zs[0];
        // Java: result[0] = tmp;
        // Java: tmp = result[0] + zs[1];
        // Java: result[1] -= tmp - result[0] - zs[1];
        // Java: result[0] = tmp;
        // Java: /* a[1] * b[0] */
        // Java: split(a[1], xs);
        // Java: split(b[1], ys);
        // Java: splitMult(xs, ys, zs);
        // Java: tmp = result[0] + zs[0];
        // Java: result[1] -= tmp - result[0] - zs[0];
        // Java: result[0] = tmp;
        // Java: tmp = result[0] + zs[1];
        // Java: result[1] -= tmp - result[0] - zs[1];
        // Java: result[0] = tmp;
    }

    // Java: static double expint(int p, final double result[])
    pub fn expint(&self) -> f64 {
        // Java: //double x = M_E;
        // Java: final double xs[] = new double[2];
        // Java: final double as[] = new double[2];
        // Java: final double ys[] = new double[2];
        // Java: //split(x, xs);
        // Java: //xs[1] = (double)(2.7182818284590452353602874713526625L - xs[0]);
        // Java: //xs[0] = 2.71827697753906250000;
        // Java: //xs[1] = 4.85091998273542816811e-06;
        // Java: //xs[0] = Double.longBitsToDouble(0x4005bf0800000000L);
        // Java: //xs[1] = Double.longBitsToDouble(0x3ed458a2bb4a9b00L);
        // Java: /* E */
        // Java: xs[0] = 2.718281828459045;
        // Java: xs[1] = 1.4456468917292502E-16;
        // Java: split(1.0, ys);
        // Java: while (p > 0)
        {
            // Java: if ((p & 1) != 0)
            {
                // Java: quadMult(ys, xs, as);
                // Java: ys[0] = as[0]; ys[1] = as[1];
            }
            // Java: quadMult(xs, xs, as);
            // Java: xs[0] = as[0]; xs[1] = as[1];
            // Java: p >>= 1;
        }
        // Java: if (result != null)
        {
            // Java: result[0] = ys[0];
            // Java: result[1] = ys[1];
            // Java: resplit(result);
        }
        // Java: return ys[0] + ys[1];
        todo!()
    }

    // Java: static double[] slowLog(double xi)
    pub fn slowLog(&self) -> Vec<f64> {
        // Java: double x[] = new double[2];
        // Java: double x2[] = new double[2];
        // Java: double y[] = new double[2];
        // Java: double a[] = new double[2];
        // Java: split(xi, x);
        // Java: /* Set X = (x-1)/(x+1) */
        // Java: x[0] += 1.0;
        // Java: resplit(x);
        // Java: splitReciprocal(x, a);
        // Java: x[0] -= 2.0;
        // Java: resplit(x);
        // Java: splitMult(x, a, y);
        // Java: x[0] = y[0];
        // Java: x[1] = y[1];
        // Java: /* Square X -> X2*/
        // Java: splitMult(x, x, x2);
        // Java: //x[0] -= 1.0;
        // Java: //resplit(x);
        // Java: y[0] = LN_SPLIT_COEF[LN_SPLIT_COEF.length-1][0];
        // Java: y[1] = LN_SPLIT_COEF[LN_SPLIT_COEF.length-1][1];
        // Java: for (int i = LN_SPLIT_COEF.length-2; i >= 0; i--)
        {
            // Java: splitMult(y, x2, a);
            // Java: y[0] = a[0];
            // Java: y[1] = a[1];
            // Java: splitAdd(y, LN_SPLIT_COEF[i], a);
            // Java: y[0] = a[0];
            // Java: y[1] = a[1];
        }
        // Java: splitMult(y, x, a);
        // Java: y[0] = a[0];
        // Java: y[1] = a[1];
        // Java: return y;
        todo!()
    }

    // Java: static void printarray(PrintStream out, String name, int expectedLen, double[][] array2d)
    pub fn printarray(&self) {
        // Java: out.println(name);
        // Java: checkLen(expectedLen, array2d.length);
        // Java: out.println(TABLE_START_DECL + " ");
        // Java: int i = 0;
        // Java: for(double[] array : array2d) { // "double array[]" causes PMD parsing error
            // Java: out.print("        {");
            // Java: for(double d : array) { // assume inner array has very few entries
                // Java: out.printf("%-25.25s", format(d)); // multiple entries per line
            // Java: out.println("}, // " + i++);
        // Java: out.println(TABLE_END_DECL);
    }

    // Java: static void printarray(PrintStream out, String name, int expectedLen, double[] array)
    pub fn printarray_2(&self) {
        // Java: out.println(name + "=");
        // Java: checkLen(expectedLen, array.length);
        // Java: out.println(TABLE_START_DECL);
        // Java: for(double d : array)
        {
            // Java: out.printf("        %s%n", format(d)); // one entry per line
        }
        // Java: out.println(TABLE_END_DECL);
    }

    // Java: static String format(double d)
    pub fn format(&self) -> String {
        // Java: if (d != d)
        {
            // Java: return "Double.NaN,";
        }
        // Java: else
        {
            // Java: return ((d >= 0) ? "+" : "") + Double.toString(d) + "d,";
        }
        todo!()
    }

    // Java: static void checkLen(int expectedLen, int actual)
    pub fn checkLen(&self) {
    }

}
