// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double X_CRIT
pub(crate) const X_CRIT: f64 = 0.0;

// Java: public class that
pub struct that {
}

// Java: public class Erf
pub struct Erf {
}

// Java: public class that
impl that {
}

// Java: public class Erf
impl Erf {
    // Java: Erf()
    pub fn Erf(&self) {
    }

    // Java: static double erf(double x)
    pub fn erf(&self, x: f64) -> f64 {
        // Java: if (FastMath.abs(x) > 40)
        {
            // Java: return x > 0 ? 1 : -1;
        }
        // Java: final double ret = Gamma.regularizedGammaP(0.5, x * x, 1.0e-15, 10000);
        // Java: return x < 0 ? -ret : ret;
        todo!();
    }

    // Java: static double erfc(double x)
    pub fn erfc(&self, x: f64) -> f64 {
        // Java: if (FastMath.abs(x) > 40)
        {
            // Java: return x > 0 ? 0 : 2;
        }
        // Java: final double ret = Gamma.regularizedGammaQ(0.5, x * x, 1.0e-15, 10000);
        // Java: return x < 0 ? 2 - ret : ret;
        todo!();
    }

    // Java: static double erf(double x1, double x2)
    pub fn erf_1(&self, x1: f64, x2: f64) -> f64 {
        // Java: if(x1 > x2)
        {
            // Java: return -erf(x2, x1);
        }
        // Java: return
        // Java: x1 < -X_CRIT ?
            // Java: x2 < 0.0 ?
                // Java: erfc(-x2) - erfc(-x1) :
                // Java: erf(x2) - erf(x1) :
            // Java: x2 > X_CRIT && x1 > 0.0 ?
                // Java: erfc(x1) - erfc(x2) :
                // Java: erf(x2) - erf(x1);
        todo!();
    }

    // Java: static double erfInv(final double x)
    pub fn erfInv(&self, x: f64) -> f64 {
        // Java: // beware that the logarithm argument must be
        // Java: // commputed as (1.0 - x) * (1.0 + x),
        // Java: // it must NOT be simplified as 1.0 - x * x as this
        // Java: // would induce rounding errors near the boundaries +/-1
        // Java: double w = - FastMath.log((1.0 - x) * (1.0 + x));
        // Java: double p;
        // Java: if (w < 6.25)
        {
            // Java: w -= 3.125;
            // Java: p =  -3.6444120640178196996e-21;
            // Java: p =   -1.685059138182016589e-19 + p * w;
            // Java: p =   1.2858480715256400167e-18 + p * w;
            // Java: p =    1.115787767802518096e-17 + p * w;
            // Java: p =   -1.333171662854620906e-16 + p * w;
            // Java: p =   2.0972767875968561637e-17 + p * w;
            // Java: p =   6.6376381343583238325e-15 + p * w;
            // Java: p =  -4.0545662729752068639e-14 + p * w;
            // Java: p =  -8.1519341976054721522e-14 + p * w;
            // Java: p =   2.6335093153082322977e-12 + p * w;
            // Java: p =  -1.2975133253453532498e-11 + p * w;
            // Java: p =  -5.4154120542946279317e-11 + p * w;
            // Java: p =    1.051212273321532285e-09 + p * w;
            // Java: p =  -4.1126339803469836976e-09 + p * w;
            // Java: p =  -2.9070369957882005086e-08 + p * w;
            // Java: p =   4.2347877827932403518e-07 + p * w;
            // Java: p =  -1.3654692000834678645e-06 + p * w;
            // Java: p =  -1.3882523362786468719e-05 + p * w;
            // Java: p =    0.0001867342080340571352 + p * w;
            // Java: p =  -0.00074070253416626697512 + p * w;
            // Java: p =   -0.0060336708714301490533 + p * w;
            // Java: p =      0.24015818242558961693 + p * w;
            // Java: p =       1.6536545626831027356 + p * w;
        }
        // Java: else if (w < 16.0)
        {
            // Java: w = FastMath.sqrt(w) - 3.25;
            // Java: p =   2.2137376921775787049e-09;
            // Java: p =   9.0756561938885390979e-08 + p * w;
            // Java: p =  -2.7517406297064545428e-07 + p * w;
            // Java: p =   1.8239629214389227755e-08 + p * w;
            // Java: p =   1.5027403968909827627e-06 + p * w;
            // Java: p =   -4.013867526981545969e-06 + p * w;
            // Java: p =   2.9234449089955446044e-06 + p * w;
            // Java: p =   1.2475304481671778723e-05 + p * w;
            // Java: p =  -4.7318229009055733981e-05 + p * w;
            // Java: p =   6.8284851459573175448e-05 + p * w;
            // Java: p =   2.4031110387097893999e-05 + p * w;
            // Java: p =   -0.0003550375203628474796 + p * w;
            // Java: p =   0.00095328937973738049703 + p * w;
            // Java: p =   -0.0016882755560235047313 + p * w;
            // Java: p =    0.0024914420961078508066 + p * w;
            // Java: p =   -0.0037512085075692412107 + p * w;
            // Java: p =     0.005370914553590063617 + p * w;
            // Java: p =       1.0052589676941592334 + p * w;
            // Java: p =       3.0838856104922207635 + p * w;
        }
        // Java: else if (!Double.isInfinite(w))
        {
            // Java: w = FastMath.sqrt(w) - 5.0;
            // Java: p =  -2.7109920616438573243e-11;
            // Java: p =  -2.5556418169965252055e-10 + p * w;
            // Java: p =   1.5076572693500548083e-09 + p * w;
            // Java: p =  -3.7894654401267369937e-09 + p * w;
            // Java: p =   7.6157012080783393804e-09 + p * w;
            // Java: p =  -1.4960026627149240478e-08 + p * w;
            // Java: p =   2.9147953450901080826e-08 + p * w;
            // Java: p =  -6.7711997758452339498e-08 + p * w;
            // Java: p =   2.2900482228026654717e-07 + p * w;
            // Java: p =  -9.9298272942317002539e-07 + p * w;
            // Java: p =   4.5260625972231537039e-06 + p * w;
            // Java: p =  -1.9681778105531670567e-05 + p * w;
            // Java: p =   7.5995277030017761139e-05 + p * w;
            // Java: p =  -0.00021503011930044477347 + p * w;
            // Java: p =  -0.00013871931833623122026 + p * w;
            // Java: p =       1.0103004648645343977 + p * w;
            // Java: p =       4.8499064014085844221 + p * w;
        }
        // Java: else
        {
            // Java: // this branch does not appears in the original code, it
            // Java: // was added because the previous branch does not handle
            // Java: // x = +/-1 correctly. In this case, w is positive infinity
            // Java: // and as the first coefficient (-2.71e-11) is negative.
            // Java: // Once the first multiplication is done, p becomes negative
            // Java: // infinity and remains so throughout the polynomial evaluation.
            // Java: // So the branch above incorrectly returns negative infinity
            // Java: // instead of the correct positive infinity.
            // Java: p = Double.POSITIVE_INFINITY;
        }
        // Java: return p * x;
        todo!();
    }

    // Java: static double erfcInv(final double x)
    pub fn erfcInv(&self, x: f64) -> f64 {
        // Java: return erfInv(1 - x);
        todo!();
    }

}
