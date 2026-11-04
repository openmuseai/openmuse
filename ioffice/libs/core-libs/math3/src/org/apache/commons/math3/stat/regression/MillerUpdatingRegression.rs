#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.Arrays
// (no import needed — Vec fill/sort are built-in)
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays::MathArrays;
use super::UpdatingMultipleLinearRegression::UpdatingMultipleLinearRegression;

// Java: public class is
pub struct is {
}

// Java: public class MillerUpdatingRegression implements UpdatingMultipleLinearRegression
pub struct MillerUpdatingRegression {
    // Java field: int nvars
    nvars: i32,
    // Java field: double[] d
    d: Vec<f64>,
    // Java field: double[] rhs
    rhs: Vec<f64>,
    // Java field: double[] r
    r: Vec<f64>,
    // Java field: double[] tol
    tol: Vec<f64>,
    // Java field: double[] rss
    rss: Vec<f64>,
    // Java field: int[] vorder
    vorder: Vec<i32>,
    // Java field: double[] work_tolset
    work_tolset: Vec<f64>,
    // Java field: long nobs
    pub(crate) nobs: i64,
    // Java field: double sserr
    sserr: f64,
    // Java field: boolean rss_set
    rss_set: bool,
    // Java field: boolean tol_set
    tol_set: bool,
    // Java field: boolean[] lindep
    lindep: Vec<bool>,
    // Java field: double[] x_sing
    x_sing: Vec<f64>,
    // Java field: double[] work_sing
    work_sing: Vec<f64>,
    // Java field: double sumy
    sumy: f64,
    // Java field: double sumsqy
    sumsqy: f64,
    // Java field: boolean hasIntercept
    pub(crate) hasIntercept: bool,
    // Java field: double epsilon
    epsilon: f64,
}

// Java: public class is
impl is {
}

// Java: public class MillerUpdatingRegression implements UpdatingMultipleLinearRegression
impl MillerUpdatingRegression {
    // Java: MillerUpdatingRegression()
    pub fn MillerUpdatingRegression() -> Self {
        // Java: this(-1, false, Double.NaN);
        Self::MillerUpdatingRegression_3(-1, false, f64::NAN)
    }

    // Java: MillerUpdatingRegression(int numberOfVariables, boolean includeConstant, double errorTolerance)
    pub fn MillerUpdatingRegression_3(numberOfVariables: i32, includeConstant: bool, errorTolerance: f64) -> Self {
        if numberOfVariables < 1 {
            panic!("ModelSpecificationException: {:?}", LocalizedFormats::NO_REGRESSORS());
        }
        let nvars = if includeConstant {
            numberOfVariables + 1
        } else {
            numberOfVariables
        };
        let n = nvars as usize;
        let mut vorder = vec![0i32; n];
        for i in 0..n {
            vorder[i] = i as i32;
        }
        let epsilon = if errorTolerance > 0.0 {
            errorTolerance
        } else {
            -errorTolerance
        };
        Self {
            nvars,
            d: vec![0.0; n],
            rhs: vec![0.0; n],
            r: vec![0.0; n * (n - 1) / 2],
            tol: vec![0.0; n],
            rss: vec![0.0; n],
            vorder,
            x_sing: vec![0.0; n],
            work_sing: vec![0.0; n],
            work_tolset: vec![0.0; n],
            lindep: vec![false; n],
            nobs: 0,
            sserr: 0.0,
            rss_set: false,
            tol_set: false,
            sumy: 0.0,
            sumsqy: 0.0,
            hasIntercept: includeConstant,
            epsilon,
        }
    }

    // Java: MillerUpdatingRegression(int numberOfVariables, boolean includeConstant)
    pub fn MillerUpdatingRegression_2(numberOfVariables: i32, includeConstant: bool) -> Self {
        // Java: this(numberOfVariables, includeConstant, Precision.EPSILON);
        Self::MillerUpdatingRegression_3(numberOfVariables, includeConstant, Precision::EPSILON)
    }

    // Java: boolean hasIntercept()
    pub fn hasIntercept(&self) -> bool {
        // Java: return this.hasIntercept;
        self.hasIntercept
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return this.nobs;
        self.nobs
    }

    // Java: void addObservation(final double[] x, final double y)
    pub fn addObservation(&mut self, x: &[f64], y: f64) {
        if (!self.hasIntercept && x.len() != self.nvars as usize) ||
           (self.hasIntercept && x.len() + 1 != self.nvars as usize) {
            panic!("ModelSpecificationException: {:?}, x.len={}, nvars={}",
                   LocalizedFormats::INVALID_REGRESSION_OBSERVATION(), x.len(), self.nvars);
        }
        if !self.hasIntercept {
            let mut x_copy = MathArrays::copyOf_f64_len(x, x.len());
            self.include(&mut x_copy, 1.0, y);
        } else {
            let mut tmp = vec![0.0; x.len() + 1];
            tmp[0] = 1.0;
            tmp[1..].copy_from_slice(x);
            self.include(&mut tmp, 1.0, y);
        }
        self.nobs += 1;
    }

    // Java: void addObservations(double[][] x, double[] y)
    pub fn addObservations(&mut self, x: &[Vec<f64>], y: &[f64]) {
        // Java: if ((x == null) || (y == null) || (x.length != y.length))
        if x.len() != y.len()
        {
            // Java: throw new ModelSpecificationException(
            panic!("ModelSpecificationException: {:?}, x.len={}, y.len={}",
                  // Java: LocalizedFormats.DIMENSIONS_MISMATCH_SIMPLE,
                  LocalizedFormats::DIMENSIONS_MISMATCH_SIMPLE(),
                  // Java: (x == null) ? 0 : x.length,
                  x.len(),
                  // Java: (y == null) ? 0 : y.length);
                  y.len());
        }
        // Java: if (x.length == 0) {  // Must be no y data either
        if x.is_empty() {
            // Java: throw new ModelSpecificationException(
            panic!("ModelSpecificationException: {:?}",
                    // Java: LocalizedFormats.NO_DATA);
                    LocalizedFormats::NO_DATA());
        }
        // Java: if (x[0].length + 1 > x.length)
        if x[0].len() + 1 > x.len()
        {
            // Java: throw new ModelSpecificationException(
            panic!("ModelSpecificationException: {:?}, x.len={}, x[0].len={}",
                  // Java: LocalizedFormats.NOT_ENOUGH_DATA_FOR_NUMBER_OF_PREDICTORS,
                  LocalizedFormats::NOT_ENOUGH_DATA_FOR_NUMBER_OF_PREDICTORS(),
                  // Java: x.length, x[0].length);
                  x.len(), x[0].len());
        }
        // Java: for (int i = 0; i < x.length; i++)
        for i in 0..x.len()
        {
            // Java: addObservation(x[i], y[i]);
            self.addObservation(&x[i], y[i]);
        }
    }

    // Java: void include(final double[] x, final double wi, final double yi)
    pub fn include(&mut self, x: &mut [f64], wi: f64, yi: f64) {
        // Java: int nextr = 0;
        let mut nextr: usize = 0;
        // Java: double w = wi;
        let mut w = wi;
        // Java: double y = yi;
        let mut y = yi;
        // Java: double xi;
        let mut xi: f64;
        // Java: double di;
        let mut di: f64;
        // Java: double wxi;
        let mut wxi: f64;
        // Java: double dpi;
        let mut dpi: f64;
        // Java: double xk;
        let mut xk: f64;
        // Java: double _w;
        let mut _w: f64;
        // Java: this.rss_set = false;
        self.rss_set = false;
        // Java: sumy = smartAdd(yi, sumy);
        self.sumy = self.smartAdd(yi, self.sumy);
        // Java: sumsqy = smartAdd(sumsqy, yi * yi);
        self.sumsqy = self.smartAdd(self.sumsqy, yi * yi);
        // Java: for (int i = 0; i < x.length; i++)
        for i in 0..x.len()
        {
            // Java: if (w == 0.0)
            if w == 0.0
            {
                // Java: return;
                return;
            }
            // Java: xi = x[i];
            xi = x[i];
            // Java: if (xi == 0.0)
            if xi == 0.0
            {
                // Java: nextr += nvars - i - 1;
                nextr += (self.nvars as usize) - i - 1;
                // Java: continue;
                continue;
            }
            // Java: di = d[i];
            di = self.d[i];
            // Java: wxi = w * xi;
            wxi = w * xi;
            // Java: _w = w;
            _w = w;
            // Java: if (di != 0.0)
            if di != 0.0
            {
                // Java: dpi = smartAdd(di, wxi * xi);
                dpi = self.smartAdd(di, wxi * xi);
                // Java: final double tmp = wxi * xi / di;
                let tmp = wxi * xi / di;
                // Java: if (FastMath.abs(tmp) > Precision.EPSILON)
                if FastMath::abs(tmp) > Precision::EPSILON
                {
                    // Java: w = (di * w) / dpi;
                    w = (di * w) / dpi;
                }
            }
            // Java: else
            else
            {
                // Java: dpi = wxi * xi;
                dpi = wxi * xi;
                // Java: w = 0.0;
                w = 0.0;
            }
            // Java: d[i] = dpi;
            self.d[i] = dpi;
            // Java: for (int k = i + 1; k < nvars; k++)
            for k in (i + 1)..(self.nvars as usize)
            {
                // Java: xk = x[k];
                xk = x[k];
                // Java: x[k] = smartAdd(xk, -xi * r[nextr]);
                x[k] = self.smartAdd(xk, -xi * self.r[nextr]);
                // Java: if (di != 0.0)
                if di != 0.0
                {
                    // Java: r[nextr] = smartAdd(di * r[nextr], (_w * xi) * xk) / dpi;
                    self.r[nextr] = self.smartAdd(di * self.r[nextr], (_w * xi) * xk) / dpi;
                }
                // Java: else
                else
                {
                    // Java: r[nextr] = xk / xi;
                    self.r[nextr] = xk / xi;
                }
                // Java: ++nextr;
                nextr += 1;
            }
            // Java: xk = y;
            xk = y;
            // Java: y = smartAdd(xk, -xi * rhs[i]);
            y = self.smartAdd(xk, -xi * self.rhs[i]);
            // Java: if (di != 0.0)
            if di != 0.0
            {
                // Java: rhs[i] = smartAdd(di * rhs[i], wxi * xk) / dpi;
                self.rhs[i] = self.smartAdd(di * self.rhs[i], wxi * xk) / dpi;
            }
            // Java: else
            else
            {
                // Java: rhs[i] = xk / xi;
                self.rhs[i] = xk / xi;
            }
        }
        // Java: sserr = smartAdd(sserr, w * y * y);
        self.sserr = self.smartAdd(self.sserr, w * y * y);
    }

    // Java: double smartAdd(double a, double b)
    pub fn smartAdd(&self, a: f64, b: f64) -> f64 {
        // Java: final double _a = FastMath.abs(a);
        let _a = FastMath::abs(a);
        // Java: final double _b = FastMath.abs(b);
        let _b = FastMath::abs(b);
        // Java: if (_a > _b)
        if _a > _b
        {
            // Java: final double eps = _a * Precision.EPSILON;
            let eps = _a * Precision::EPSILON;
            // Java: if (_b > eps)
            if _b > eps
            {
                // Java: return a + b;
                return a + b;
            }
            // Java: return a;
            return a;
        }
        // Java: else
        else
        {
            // Java: final double eps = _b * Precision.EPSILON;
            let eps = _b * Precision::EPSILON;
            // Java: if (_a > eps)
            if _a > eps
            {
                // Java: return a + b;
                return a + b;
            }
            // Java: return b;
            return b;
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: Arrays.fill(this.d, 0.0);
        self.d.fill(0.0);
        // Java: Arrays.fill(this.rhs, 0.0);
        self.rhs.fill(0.0);
        // Java: Arrays.fill(this.r, 0.0);
        self.r.fill(0.0);
        // Java: Arrays.fill(this.tol, 0.0);
        self.tol.fill(0.0);
        // Java: Arrays.fill(this.rss, 0.0);
        self.rss.fill(0.0);
        // Java: Arrays.fill(this.work_tolset, 0.0);
        self.work_tolset.fill(0.0);
        // Java: Arrays.fill(this.work_sing, 0.0);
        self.work_sing.fill(0.0);
        // Java: Arrays.fill(this.x_sing, 0.0);
        self.x_sing.fill(0.0);
        // Java: Arrays.fill(this.lindep, false);
        self.lindep.fill(false);
        // Java: for (int i = 0; i < nvars; i++)
        for i in 0..(self.nvars as usize)
        {
            // Java: this.vorder[i] = i;
            self.vorder[i] = i as i32;
        }
        // Java: this.nobs = 0;
        self.nobs = 0;
        // Java: this.sserr = 0.0;
        self.sserr = 0.0;
        // Java: this.sumy = 0.0;
        self.sumy = 0.0;
        // Java: this.sumsqy = 0.0;
        self.sumsqy = 0.0;
        // Java: this.rss_set = false;
        self.rss_set = false;
        // Java: this.tol_set = false;
        self.tol_set = false;
    }

    // Java: void tolset()
    pub fn tolset(&mut self) {
        // Java: int pos;
        let mut pos: usize;
        // Java: double total;
        let mut total: f64;
        // Java: final double eps = this.epsilon;
        let eps = self.epsilon;
        // Java: for (int i = 0; i < nvars; i++)
        for i in 0..(self.nvars as usize)
        {
            // Java: this.work_tolset[i] = FastMath.sqrt(d[i]);
            self.work_tolset[i] = FastMath::sqrt(self.d[i]);
        }
        // Java: tol[0] = eps * this.work_tolset[0];
        self.tol[0] = eps * self.work_tolset[0];
        // Java: for (int col = 1; col < nvars; col++)
        for col in 1..(self.nvars as usize)
        {
            // Java: pos = col - 1;
            pos = col - 1;
            // Java: total = work_tolset[col];
            total = self.work_tolset[col];
            // Java: for (int row = 0; row < col; row++)
            for row in 0..col
            {
                // Java: total += FastMath.abs(r[pos]) * work_tolset[row];
                total += FastMath::abs(self.r[pos]) * self.work_tolset[row];
                // Java: pos += nvars - row - 2;
                pos += (self.nvars as usize) - row - 2;
            }
            // Java: tol[col] = eps * total;
            self.tol[col] = eps * total;
        }
        // Java: tol_set = true;
        self.tol_set = true;
    }

    // Java: double[] regcf(int nreq)
    pub fn regcf(&mut self, nreq: i32) -> Vec<f64> {
        // Java: int nextr;
        let mut nextr: usize;
        // Java: if (nreq < 1)
        if nreq < 1
        {
            // Java: throw new ModelSpecificationException(LocalizedFormats.NO_REGRESSORS);
            panic!("ModelSpecificationException: {:?}", LocalizedFormats::NO_REGRESSORS());
        }
        // Java: if (nreq > this.nvars)
        if nreq > self.nvars
        {
            // Java: throw new ModelSpecificationException(
            panic!("ModelSpecificationException: {:?}, nreq={}, nvars={}",
                    // Java: LocalizedFormats.TOO_MANY_REGRESSORS, nreq, this.nvars);
                    LocalizedFormats::TOO_MANY_REGRESSORS(), nreq, self.nvars);
        }
        // Java: if (!this.tol_set)
        if !self.tol_set
        {
            // Java: tolset();
            self.tolset();
        }
        let nreq_u = nreq as usize;
        // Java: final double[] ret = new double[nreq];
        let mut ret = vec![0.0; nreq_u];
        // Java: boolean rankProblem = false;
        let mut rankProblem = false;
        // Java: for (int i = nreq - 1; i > -1; i--)
        for i in (0..nreq_u).rev()
        {
            // Java: if (FastMath.sqrt(d[i]) < tol[i])
            if FastMath::sqrt(self.d[i]) < self.tol[i]
            {
                // Java: ret[i] = 0.0;
                ret[i] = 0.0;
                // Java: d[i] = 0.0;
                self.d[i] = 0.0;
                // Java: rankProblem = true;
                rankProblem = true;
            }
            // Java: else
            else
            {
                // Java: ret[i] = rhs[i];
                ret[i] = self.rhs[i];
                // Java: nextr = i * (nvars + nvars - i - 1) / 2;
                nextr = i * ((self.nvars as usize) + (self.nvars as usize) - i - 1) / 2;
                // Java: for (int j = i + 1; j < nreq; j++)
                for j in (i + 1)..nreq_u
                {
                    // Java: ret[i] = smartAdd(ret[i], -r[nextr] * ret[j]);
                    ret[i] = self.smartAdd(ret[i], -self.r[nextr] * ret[j]);
                    // Java: ++nextr;
                    nextr += 1;
                }
            }
        }
        // Java: if (rankProblem)
        if rankProblem
        {
            // Java: for (int i = 0; i < nreq; i++)
            for i in 0..nreq_u
            {
                // Java: if (this.lindep[i])
                if self.lindep[i]
                {
                    // Java: ret[i] = Double.NaN;
                    ret[i] = f64::NAN;
                }
            }
        }
        // Java: return ret;
        ret
    }

    // Java: void singcheck()
    pub fn singcheck(&mut self) {
        // Java: int pos;
        let mut pos: usize;
        // Java: for (int i = 0; i < nvars; i++)
        for i in 0..(self.nvars as usize)
        {
            // Java: work_sing[i] = FastMath.sqrt(d[i]);
            self.work_sing[i] = FastMath::sqrt(self.d[i]);
        }
        // Java: for (int col = 0; col < nvars; col++)
        for col in 0..(self.nvars as usize)
        {
            // Java: // Set elements within R to zero if they are less than tol(col) in
            // Java: // absolute value after being scaled by the square root of their row
            // Java: // multiplier
            // Java: final double temp = tol[col];
            let temp = self.tol[col];
            // Java: pos = col - 1;
            pos = if col > 0 { col - 1 } else { 0 };
            // Java: for (int row = 0; row < col - 1; row++)
            if col > 1 {
                for row in 0..(col - 1)
                {
                    // Java: if (FastMath.abs(r[pos]) * work_sing[row] < temp)
                    if FastMath::abs(self.r[pos]) * self.work_sing[row] < temp
                    {
                        // Java: r[pos] = 0.0;
                        self.r[pos] = 0.0;
                    }
                    // Java: pos += nvars - row - 2;
                    pos += (self.nvars as usize) - row - 2;
                }
            }
            // Java: // If diagonal element is near zero, set it to zero, set appropriate
            // Java: // element of LINDEP, and use INCLUD to augment the projections in
            // Java: // the lower rows of the orthogonalization.
            // Java: lindep[col] = false;
            self.lindep[col] = false;
            // Java: if (work_sing[col] < temp)
            if self.work_sing[col] < temp
            {
                // Java: lindep[col] = true;
                self.lindep[col] = true;
                // Java: if (col < nvars - 1)
                if col < (self.nvars as usize) - 1
                {
                    // Java: Arrays.fill(x_sing, 0.0);
                    self.x_sing.fill(0.0);
                    // Java: int _pi = col * (nvars + nvars - col - 1) / 2;
                    let mut _pi = col * ((self.nvars as usize) + (self.nvars as usize) - col - 1) / 2;
                    // Java: for (int _xi = col + 1; _xi < nvars; _xi++, _pi++)
                    for _xi in (col + 1)..(self.nvars as usize)
                    {
                        // Java: x_sing[_xi] = r[_pi];
                        self.x_sing[_xi] = self.r[_pi];
                        // Java: r[_pi] = 0.0;
                        self.r[_pi] = 0.0;
                        _pi += 1;
                    }
                    // Java: final double y = rhs[col];
                    let y = self.rhs[col];
                    // Java: final double weight = d[col];
                    let weight = self.d[col];
                    // Java: d[col] = 0.0;
                    self.d[col] = 0.0;
                    // Java: rhs[col] = 0.0;
                    self.rhs[col] = 0.0;
                    // Java: this.include(x_sing, weight, y);
                    let mut x_sing_copy = self.x_sing.clone();
                    self.include(&mut x_sing_copy, weight, y);
                }
                // Java: else
                else
                {
                    // Java: sserr += d[col] * rhs[col] * rhs[col];
                    self.sserr += self.d[col] * self.rhs[col] * self.rhs[col];
                }
            }
        }
    }

    // Java: void ss()
    pub fn ss(&mut self) {
        // Java: double total = sserr;
        let mut total = self.sserr;
        // Java: rss[nvars - 1] = sserr;
        let nvars_u = self.nvars as usize;
        self.rss[nvars_u - 1] = self.sserr;
        // Java: for (int i = nvars - 1; i > 0; i--)
        for i in (1..nvars_u).rev()
        {
            // Java: total += d[i] * rhs[i] * rhs[i];
            total += self.d[i] * self.rhs[i] * self.rhs[i];
            // Java: rss[i - 1] = total;
            self.rss[i - 1] = total;
        }
        // Java: rss_set = true;
        self.rss_set = true;
    }

    // Java: double[] cov(int nreq)
    pub fn cov(&mut self, nreq: i32) -> Option<Vec<f64>> {
        let nreq_u = nreq as usize;
        // Java: if (this.nobs <= nreq)
        if self.nobs <= nreq as i64
        {
            // Java: return null;
            return None;
        }
        // Java: double rnk = 0.0;
        let mut rnk = 0.0_f64;
        // Java: for (int i = 0; i < nreq; i++)
        for i in 0..nreq_u
        {
            // Java: if (!this.lindep[i])
            if !self.lindep[i]
            {
                // Java: rnk += 1.0;
                rnk += 1.0;
            }
        }
        // Java: final double var = rss[nreq - 1] / (nobs - rnk);
        let var = self.rss[nreq_u - 1] / (self.nobs as f64 - rnk);
        // Java: final double[] rinv = new double[nreq * (nreq - 1) / 2];
        let mut rinv = vec![0.0; nreq_u * (nreq_u - 1) / 2];
        // Java: inverse(rinv, nreq);
        self.inverse(&mut rinv, nreq);
        // Java: final double[] covmat = new double[nreq * (nreq + 1) / 2];
        let mut covmat = vec![f64::NAN; nreq_u * (nreq_u + 1) / 2];
        // Java: Arrays.fill(covmat, Double.NaN);
        // (already filled with NAN above)
        // Java: int pos2;
        let mut pos2: usize;
        // Java: int pos1;
        let mut pos1: usize;
        // Java: int start = 0;
        let mut start: usize = 0;
        // Java: double total = 0;
        let mut total: f64 = 0.0;
        // Java: for (int row = 0; row < nreq; row++)
        for row in 0..nreq_u
        {
            // Java: pos2 = start;
            pos2 = start;
            // Java: if (!this.lindep[row])
            if !self.lindep[row]
            {
                // Java: for (int col = row; col < nreq; col++)
                for col in row..nreq_u
                {
                    // Java: if (!this.lindep[col])
                    if !self.lindep[col]
                    {
                        // Java: pos1 = start + col - row;
                        pos1 = start + col - row;
                        // Java: if (row == col)
                        if row == col
                        {
                            // Java: total = 1.0 / d[col];
                            total = 1.0 / self.d[col];
                        }
                        // Java: else
                        else
                        {
                            // Java: total = rinv[pos1 - 1] / d[col];
                            total = rinv[pos1 - 1] / self.d[col];
                        }
                        // Java: for (int k = col + 1; k < nreq; k++)
                        for _k in (col + 1)..nreq_u
                        {
                            // Java: if (!this.lindep[k])
                            if !self.lindep[_k]
                            {
                                // Java: total += rinv[pos1] * rinv[pos2] / d[k];
                                total += rinv[pos1] * rinv[pos2] / self.d[_k];
                            }
                            // Java: ++pos1;
                            pos1 += 1;
                            // Java: ++pos2;
                            pos2 += 1;
                        }
                        // Java: covmat[ (col + 1) * col / 2 + row] = total * var;
                        covmat[(col + 1) * col / 2 + row] = total * var;
                    }
                    // Java: else
                    else
                    {
                        // Java: pos2 += nreq - col - 1;
                        pos2 += nreq_u - col - 1;
                    }
                }
            }
            // Java: start += nreq - row - 1;
            start += nreq_u - row - 1;
        }
        // Java: return covmat;
        Some(covmat)
    }

    // Java: void inverse(double[] rinv, int nreq)
    pub fn inverse(&self, rinv: &mut [f64], nreq: i32) {
        let nreq_u = nreq as usize;
        let nvars_u = self.nvars as usize;
        // Java: int pos = nreq * (nreq - 1) / 2 - 1;
        let mut pos: i64 = (nreq_u * (nreq_u - 1) / 2) as i64 - 1;
        // Java: int pos1 = -1;
        let mut pos1: i64;
        // Java: int pos2 = -1;
        let mut pos2: i64;
        // Java: double total = 0.0;
        let mut total: f64;
        // Java: Arrays.fill(rinv, Double.NaN);
        rinv.fill(f64::NAN);
        // Java: for (int row = nreq - 1; row > 0; --row)
        for row in (1..nreq_u).rev()
        {
            // Java: if (!this.lindep[row])
            if !self.lindep[row]
            {
                // Java: final int start = (row - 1) * (nvars + nvars - row) / 2;
                let start = (row - 1) * (nvars_u + nvars_u - row) / 2;
                // Java: for (int col = nreq; col > row; --col)
                for col in ((row + 1)..=nreq_u).rev()
                {
                    // Java: pos1 = start;
                    pos1 = start as i64;
                    // Java: pos2 = pos;
                    pos2 = pos;
                    // Java: total = 0.0;
                    total = 0.0;
                    // Java: for (int k = row; k < col - 1; k++)
                    for k in row..(col - 1)
                    {
                        // Java: pos2 += nreq - k - 1;
                        pos2 += (nreq_u - k - 1) as i64;
                        // Java: if (!this.lindep[k])
                        if !self.lindep[k]
                        {
                            // Java: total += -r[pos1] * rinv[pos2];
                            total += -self.r[pos1 as usize] * rinv[pos2 as usize];
                        }
                        // Java: ++pos1;
                        pos1 += 1;
                    }
                    // Java: rinv[pos] = total - r[pos1];
                    rinv[pos as usize] = total - self.r[pos1 as usize];
                    // Java: --pos;
                    pos -= 1;
                }
            }
            // Java: else
            else
            {
                // Java: pos -= nreq - row;
                pos -= (nreq_u - row) as i64;
            }
        }
    }

    // Java: double[] getPartialCorrelations(int in)
    pub fn getPartialCorrelations(&self, in_val: i32) -> Option<Vec<f64>> {
        let nvars_u = self.nvars as usize;
        let in_u = in_val as usize;
        // Java: final double[] output = new double[(nvars - in + 1) * (nvars - in) / 2];
        let out_size = ((self.nvars - in_val + 1) * (self.nvars - in_val) / 2) as usize;
        let mut output = vec![0.0; out_size];
        // Java: int pos;
        let mut pos: usize;
        // Java: int pos1;
        let mut pos1: usize;
        // Java: int pos2;
        let mut pos2: usize;
        // Java: final int rms_off = -in;
        let rms_off: i32 = -in_val;
        // Java: final int wrk_off = -(in + 1);
        let wrk_off: i32 = -(in_val + 1);
        // Java: final double[] rms = new double[nvars - in];
        let mut rms = vec![0.0; (self.nvars - in_val) as usize];
        // Java: final double[] work = new double[nvars - in - 1];
        let mut work = vec![0.0; (self.nvars - in_val - 1) as usize];
        // Java: double sumxx;
        let mut sumxx: f64;
        // Java: double sumxy;
        let mut sumxy: f64;
        // Java: double sumyy;
        let mut sumyy: f64;
        // Java: final int offXX = (nvars - in) * (nvars - in - 1) / 2;
        let offXX = ((self.nvars - in_val) * (self.nvars - in_val - 1) / 2) as usize;
        // Java: if (in < -1 || in >= nvars)
        if in_val < -1 || in_val >= self.nvars
        {
            // Java: return null;
            return None;
        }
        // Java: final int nvm = nvars - 1;
        let nvm = nvars_u - 1;
        // Java: final int base_pos = r.length - (nvm - in) * (nvm - in + 1) / 2;
        let base_pos = self.r.len() - (nvm - in_u) * (nvm - in_u + 1) / 2;
        // Java: if (d[in] > 0.0)
        if self.d[in_u] > 0.0
        {
            // Java: rms[in + rms_off] = 1.0 / FastMath.sqrt(d[in]);
            rms[(in_val + rms_off) as usize] = 1.0 / FastMath::sqrt(self.d[in_u]);
        }
        // Java: for (int col = in + 1; col < nvars; col++)
        for col in (in_u + 1)..nvars_u
        {
            // Java: pos = base_pos + col - 1 - in;
            pos = base_pos + col - 1 - in_u;
            // Java: sumxx = d[col];
            sumxx = self.d[col];
            // Java: for (int row = in; row < col; row++)
            for row in in_u..col
            {
                // Java: sumxx += d[row] * r[pos] * r[pos];
                sumxx += self.d[row] * self.r[pos] * self.r[pos];
                // Java: pos += nvars - row - 2;
                pos += nvars_u - row - 2;
            }
            // Java: if (sumxx > 0.0)
            if sumxx > 0.0
            {
                // Java: rms[col + rms_off] = 1.0 / FastMath.sqrt(sumxx);
                rms[(col as i32 + rms_off) as usize] = 1.0 / FastMath::sqrt(sumxx);
            }
            // Java: else
            else
            {
                // Java: rms[col + rms_off] = 0.0;
                rms[(col as i32 + rms_off) as usize] = 0.0;
            }
        }
        // Java: sumyy = sserr;
        sumyy = self.sserr;
        // Java: for (int row = in; row < nvars; row++)
        for row in in_u..nvars_u
        {
            // Java: sumyy += d[row] * rhs[row] * rhs[row];
            sumyy += self.d[row] * self.rhs[row] * self.rhs[row];
        }
        // Java: if (sumyy > 0.0)
        if sumyy > 0.0
        {
            // Java: sumyy = 1.0 / FastMath.sqrt(sumyy);
            sumyy = 1.0 / FastMath::sqrt(sumyy);
        }
        // Java: pos = 0;
        pos = 0;
        // Java: for (int col1 = in; col1 < nvars; col1++)
        for col1 in in_u..nvars_u
        {
            // Java: sumxy = 0.0;
            sumxy = 0.0;
            // Java: Arrays.fill(work, 0.0);
            work.fill(0.0);
            // Java: pos1 = base_pos + col1 - in - 1;
            pos1 = base_pos + col1 - in_u - 1;
            // Java: for (int row = in; row < col1; row++)
            for row in in_u..col1
            {
                // Java: pos2 = pos1 + 1;
                pos2 = pos1 + 1;
                // Java: for (int col2 = col1 + 1; col2 < nvars; col2++)
                for col2 in (col1 + 1)..nvars_u
                {
                    // Java: work[col2 + wrk_off] += d[row] * r[pos1] * r[pos2];
                    work[(col2 as i32 + wrk_off) as usize] += self.d[row] * self.r[pos1] * self.r[pos2];
                    // Java: pos2++;
                    pos2 += 1;
                }
                // Java: sumxy += d[row] * r[pos1] * rhs[row];
                sumxy += self.d[row] * self.r[pos1] * self.rhs[row];
                // Java: pos1 += nvars - row - 2;
                pos1 += nvars_u - row - 2;
            }
            // Java: pos2 = pos1 + 1;
            pos2 = pos1 + 1;
            // Java: for (int col2 = col1 + 1; col2 < nvars; col2++)
            for col2 in (col1 + 1)..nvars_u
            {
                // Java: work[col2 + wrk_off] += d[col1] * r[pos2];
                work[(col2 as i32 + wrk_off) as usize] += self.d[col1] * self.r[pos2];
                // Java: ++pos2;
                pos2 += 1;
                // Java: output[ (col2 - 1 - in) * (col2 - in) / 2 + col1 - in] =
                output[(col2 - 1 - in_u) * (col2 - in_u) / 2 + col1 - in_u] =
                        // Java: work[col2 + wrk_off] * rms[col1 + rms_off] * rms[col2 + rms_off];
                        work[(col2 as i32 + wrk_off) as usize] * rms[(col1 as i32 + rms_off) as usize] * rms[(col2 as i32 + rms_off) as usize];
                // Java: ++pos;
                pos += 1;
            }
            // Java: sumxy += d[col1] * rhs[col1];
            sumxy += self.d[col1] * self.rhs[col1];
            // Java: output[col1 + rms_off + offXX] = sumxy * rms[col1 + rms_off] * sumyy;
            output[(col1 as i32 + rms_off) as usize + offXX] = sumxy * rms[(col1 as i32 + rms_off) as usize] * sumyy;
        }
        // Java: return output;
        Some(output)
    }

    // Java: void vmove(int from, int to)
    pub fn vmove(&mut self, from: i32, to: i32) {
        let nvars_u = self.nvars as usize;
        // Java: double d1;
        let mut d1: f64;
        // Java: double d2;
        let mut d2: f64;
        // Java: double X;
        #[allow(non_snake_case)]
        let mut X: f64;
        // Java: double d1new;
        let mut d1new: f64;
        // Java: double d2new;
        let mut d2new: f64;
        // Java: double cbar;
        let mut cbar: f64;
        // Java: double sbar;
        let mut sbar: f64;
        // Java: double Y;
        #[allow(non_snake_case)]
        let mut Y: f64;
        // Java: int first;
        let first: usize;
        // Java: int inc;
        let inc: i32;
        // Java: int m1;
        let mut m1: usize;
        // Java: int m2;
        let mut m2: usize;
        // Java: int mp1;
        let mut mp1: usize;
        // Java: int pos;
        let mut pos: usize;
        // Java: boolean bSkipTo40 = false;
        let mut bSkipTo40: bool;
        // Java: if (from == to)
        if from == to
        {
            // Java: return;
            return;
        }
        // Java: if (!this.rss_set)
        if !self.rss_set
        {
            // Java: ss();
            self.ss();
        }
        // Java: int count = 0;
        let count: usize;
        // Java: if (from < to)
        if from < to
        {
            // Java: first = from;
            first = from as usize;
            // Java: inc = 1;
            inc = 1;
            // Java: count = to - from;
            count = (to - from) as usize;
        }
        // Java: else
        else
        {
            // Java: first = from - 1;
            first = (from - 1) as usize;
            // Java: inc = -1;
            inc = -1;
            // Java: count = from - to;
            count = (from - to) as usize;
        }
        // Java: int m = first;
        let mut m: usize = first;
        // Java: int idx = 0;
        let mut idx: usize = 0;
        // Java: while (idx < count)
        while idx < count
        {
            bSkipTo40 = false;
            // Java: m1 = m * (nvars + nvars - m - 1) / 2;
            m1 = m * (nvars_u + nvars_u - m - 1) / 2;
            // Java: m2 = m1 + nvars - m - 1;
            m2 = m1 + nvars_u - m - 1;
            // Java: mp1 = m + 1;
            mp1 = m + 1;
            // Java: d1 = d[m];
            d1 = self.d[m];
            // Java: d2 = d[mp1];
            d2 = self.d[mp1];
            // Java: // Special cases.
            // Java: if (d1 > this.epsilon || d2 > this.epsilon)
            if d1 > self.epsilon || d2 > self.epsilon
            {
                // Java: X = r[m1];
                X = self.r[m1];
                // Java: if (FastMath.abs(X) * FastMath.sqrt(d1) < tol[mp1])
                if FastMath::abs(X) * FastMath::sqrt(d1) < self.tol[mp1]
                {
                    // Java: X = 0.0;
                    X = 0.0;
                }
                // Java: if (d1 < this.epsilon || FastMath.abs(X) < this.epsilon)
                if d1 < self.epsilon || FastMath::abs(X) < self.epsilon
                {
                    // Java: d[m] = d2;
                    self.d[m] = d2;
                    // Java: d[mp1] = d1;
                    self.d[mp1] = d1;
                    // Java: r[m1] = 0.0;
                    self.r[m1] = 0.0;
                    // Java: for (int col = m + 2; col < nvars; col++)
                    for _col in (m + 2)..nvars_u
                    {
                        // Java: ++m1;
                        m1 += 1;
                        // Java: X = r[m1];
                        X = self.r[m1];
                        // Java: r[m1] = r[m2];
                        self.r[m1] = self.r[m2];
                        // Java: r[m2] = X;
                        self.r[m2] = X;
                        // Java: ++m2;
                        m2 += 1;
                    }
                    // Java: X = rhs[m];
                    X = self.rhs[m];
                    // Java: rhs[m] = rhs[mp1];
                    self.rhs[m] = self.rhs[mp1];
                    // Java: rhs[mp1] = X;
                    self.rhs[mp1] = X;
                    // Java: bSkipTo40 = true;
                    bSkipTo40 = true;
                    // Java: //break;
                }
                // Java: else if (d2 < this.epsilon)
                else if d2 < self.epsilon
                {
                    // Java: d[m] = d1 * X * X;
                    self.d[m] = d1 * X * X;
                    // Java: r[m1] = 1.0 / X;
                    self.r[m1] = 1.0 / X;
                    // Java: for (int _i = m1 + 1; _i < m1 + nvars - m - 1; _i++)
                    let m1_saved = m1;
                    for _i in (m1_saved + 1)..(m1_saved + nvars_u - m - 1)
                    {
                        // Java: r[_i] /= X;
                        self.r[_i] /= X;
                    }
                    // Java: rhs[m] /= X;
                    self.rhs[m] /= X;
                    // Java: bSkipTo40 = true;
                    bSkipTo40 = true;
                    // Java: //break;
                }
                // Java: if (!bSkipTo40)
                if !bSkipTo40
                {
                    // Java: d1new = d2 + d1 * X * X;
                    d1new = d2 + d1 * X * X;
                    // Java: cbar = d2 / d1new;
                    cbar = d2 / d1new;
                    // Java: sbar = X * d1 / d1new;
                    sbar = X * d1 / d1new;
                    // Java: d2new = d1 * cbar;
                    d2new = d1 * cbar;
                    // Java: d[m] = d1new;
                    self.d[m] = d1new;
                    // Java: d[mp1] = d2new;
                    self.d[mp1] = d2new;
                    // Java: r[m1] = sbar;
                    self.r[m1] = sbar;
                    // Java: for (int col = m + 2; col < nvars; col++)
                    for _col in (m + 2)..nvars_u
                    {
                        // Java: ++m1;
                        m1 += 1;
                        // Java: Y = r[m1];
                        Y = self.r[m1];
                        // Java: r[m1] = cbar * r[m2] + sbar * Y;
                        self.r[m1] = cbar * self.r[m2] + sbar * Y;
                        // Java: r[m2] = Y - X * r[m2];
                        self.r[m2] = Y - X * self.r[m2];
                        // Java: ++m2;
                        m2 += 1;
                    }
                    // Java: Y = rhs[m];
                    Y = self.rhs[m];
                    // Java: rhs[m] = cbar * rhs[mp1] + sbar * Y;
                    self.rhs[m] = cbar * self.rhs[mp1] + sbar * Y;
                    // Java: rhs[mp1] = Y - X * rhs[mp1];
                    self.rhs[mp1] = Y - X * self.rhs[mp1];
                }
            }
            // Java: if (m > 0)
            if m > 0
            {
                // Java: pos = m;
                pos = m;
                // Java: for (int row = 0; row < m; row++)
                for row in 0..m
                {
                    // Java: X = r[pos];
                    X = self.r[pos];
                    // Java: r[pos] = r[pos - 1];
                    self.r[pos] = self.r[pos - 1];
                    // Java: r[pos - 1] = X;
                    self.r[pos - 1] = X;
                    // Java: pos += nvars - row - 2;
                    pos += nvars_u - row - 2;
                }
            }
            // Java: // Adjust variable order (VORDER), the tolerances (TOL) and
            // Java: // the vector of residual sums of squares (RSS).
            // Java: m1 = vorder[m];
            let vm = self.vorder[m];
            // Java: vorder[m] = vorder[mp1];
            self.vorder[m] = self.vorder[mp1];
            // Java: vorder[mp1] = m1;
            self.vorder[mp1] = vm;
            // Java: X = tol[m];
            X = self.tol[m];
            // Java: tol[m] = tol[mp1];
            self.tol[m] = self.tol[mp1];
            // Java: tol[mp1] = X;
            self.tol[mp1] = X;
            // Java: rss[m] = rss[mp1] + d[mp1] * rhs[mp1] * rhs[mp1];
            self.rss[m] = self.rss[mp1] + self.d[mp1] * self.rhs[mp1] * self.rhs[mp1];
            // Java: m += inc;
            m = (m as i32 + inc) as usize;
            // Java: ++idx;
            idx += 1;
        }
    }

    // Java: int reorderRegressors(int[] list, int pos1)
    pub fn reorderRegressors(&mut self, list: &[i32], pos1: i32) -> i32 {
        let nvars_u = self.nvars as usize;
        // Java: int next;
        let mut next: i32;
        // Java: int i;
        let mut i: usize;
        // Java: int l;
        let mut l: i32;
        // Java: if (list.length < 1 || list.length > nvars + 1 - pos1)
        if list.is_empty() || list.len() as i32 > self.nvars + 1 - pos1
        {
            // Java: return -1;
            return -1;
        }
        // Java: next = pos1;
        next = pos1;
        // Java: i = pos1;
        i = pos1 as usize;
        // Java: while (i < nvars)
        while i < nvars_u
        {
            // Java: l = vorder[i];
            l = self.vorder[i];
            // Java: for (int j = 0; j < list.length; j++)
            for j in 0..list.len()
            {
                // Java: if (l == list[j] && i > next)
                if l == list[j] && (i as i32) > next
                {
                    // Java: this.vmove(i, next);
                    self.vmove(i as i32, next);
                    // Java: ++next;
                    next += 1;
                    // Java: if (next >= list.length + pos1)
                    if next >= list.len() as i32 + pos1
                    {
                        // Java: return 0;
                        return 0;
                    }
                    // Java: else
                    else
                    {
                        // Java: break;
                        break;
                    }
                }
            }
            // Java: ++i;
            i += 1;
        }
        // Java: return 0;
        0
    }

    // Java: double getDiagonalOfHatMatrix(double[] row_data)
    pub fn getDiagonalOfHatMatrix(&mut self, row_data: &[f64]) -> f64 {
        let nvars_u = self.nvars as usize;
        // Java: double[] wk = new double[this.nvars];
        let mut wk = vec![0.0; nvars_u];
        // Java: int pos;
        let mut pos: usize;
        // Java: double total;
        let mut total: f64;
        // Java: if (row_data.length > nvars)
        if row_data.len() > nvars_u
        {
            // Java: return Double.NaN;
            return f64::NAN;
        }
        // Java: double[] xrow;
        let xrow: Vec<f64>;
        // Java: if (this.hasIntercept)
        if self.hasIntercept
        {
            // Java: xrow = new double[row_data.length + 1];
            let mut tmp = vec![0.0; row_data.len() + 1];
            // Java: xrow[0] = 1.0;
            tmp[0] = 1.0;
            // Java: System.arraycopy(row_data, 0, xrow, 1, row_data.length);
            tmp[1..].copy_from_slice(row_data);
            xrow = tmp;
        }
        // Java: else
        else
        {
            // Java: xrow = row_data;
            xrow = row_data.to_vec();
        }
        // Java: double hii = 0.0;
        let mut hii = 0.0;
        // Java: for (int col = 0; col < xrow.length; col++)
        for col in 0..xrow.len()
        {
            // Java: if (FastMath.sqrt(d[col]) < tol[col])
            if FastMath::sqrt(self.d[col]) < self.tol[col]
            {
                // Java: wk[col] = 0.0;
                wk[col] = 0.0;
            }
            // Java: else
            else
            {
                // Java: pos = col - 1;
                pos = if col > 0 { col - 1 } else { 0 };
                // Java: total = xrow[col];
                total = xrow[col];
                // Java: for (int row = 0; row < col; row++)
                for row in 0..col
                {
                    // Java: total = smartAdd(total, -wk[row] * r[pos]);
                    total = self.smartAdd(total, -wk[row] * self.r[pos]);
                    // Java: pos += nvars - row - 2;
                    pos += nvars_u - row - 2;
                }
                // Java: wk[col] = total;
                wk[col] = total;
                // Java: hii = smartAdd(hii, (total * total) / d[col]);
                hii = self.smartAdd(hii, (total * total) / self.d[col]);
            }
        }
        // Java: return hii;
        hii
    }

    // Java: int[] getOrderOfRegressors()
    pub fn getOrderOfRegressors(&self) -> Vec<i32> {
        // Java: return MathArrays.copyOf(vorder);
        MathArrays::copyOf_i32(&self.vorder)
    }

    // Java: RegressionResults regress()
    pub fn regress(&mut self) -> Box<dyn std::any::Any> {
        // Java: return regress(this.nvars);
        let nvars = self.nvars;
        self.regress_int(nvars)
    }

    // Java: RegressionResults regress(int numberOfRegressors)
    pub fn regress_int(&mut self, numberOfRegressors: i32) -> Box<dyn std::any::Any> {
        // Java: if (this.nobs <= numberOfRegressors)
        if self.nobs <= numberOfRegressors as i64
        {
           // Java: throw new ModelSpecificationException(
           panic!("ModelSpecificationException: {:?}, nobs={}, numberOfRegressors={}",
                   // Java: LocalizedFormats.NOT_ENOUGH_DATA_FOR_NUMBER_OF_PREDICTORS,
                   LocalizedFormats::NOT_ENOUGH_DATA_FOR_NUMBER_OF_PREDICTORS(),
                   // Java: this.nobs, numberOfRegressors);
                   self.nobs, numberOfRegressors);
        }
        // Java: if( numberOfRegressors > this.nvars )
        if numberOfRegressors > self.nvars
        {
            // Java: throw new ModelSpecificationException(
            panic!("ModelSpecificationException: {:?}, numberOfRegressors={}, nvars={}",
                    // Java: LocalizedFormats.TOO_MANY_REGRESSORS, numberOfRegressors, this.nvars);
                    LocalizedFormats::TOO_MANY_REGRESSORS(), numberOfRegressors, self.nvars);
        }
        // Java: tolset();
        self.tolset();
        // Java: singcheck();
        self.singcheck();
        // Java: double[] beta = this.regcf(numberOfRegressors);
        let beta = self.regcf(numberOfRegressors);
        // Java: ss();
        self.ss();
        // Java: double[] cov = this.cov(numberOfRegressors);
        let cov = self.cov(numberOfRegressors);
        // Java: int rnk = 0;
        let mut rnk = 0i32;
        // Java: for (int i = 0; i < this.lindep.length; i++)
        for i in 0..self.lindep.len()
        {
            // Java: if (!this.lindep[i])
            if !self.lindep[i]
            {
                // Java: ++rnk;
                rnk += 1;
            }
        }
        // Java: boolean needsReorder = false;
        let mut needsReorder = false;
        // Java: for (int i = 0; i < numberOfRegressors; i++)
        for i in 0..(numberOfRegressors as usize)
        {
            // Java: if (this.vorder[i] != i)
            if self.vorder[i] != i as i32
            {
                // Java: needsReorder = true;
                needsReorder = true;
                // Java: break;
                break;
            }
        }
        // Java: if (!needsReorder)
        if !needsReorder
        {
            // Java: return new RegressionResults(
            // Java: beta, new double[][]{cov}, true, this.nobs, rnk,
            // Java: this.sumy, this.sumsqy, this.sserr, this.hasIntercept, false);
            todo!()
        }
        // Java: else
        else
        {
            let nvars_u = self.nvars as usize;
            let nregs = numberOfRegressors as usize;
            // Java: double[] betaNew = new double[beta.length];
            let mut betaNew = vec![0.0; beta.len()];
            // Java: double[] covNew = new double[cov.length];
            let cov_ref = cov.unwrap();
            let mut covNew = vec![0.0; cov_ref.len()];
            // Java: int[] newIndices = new int[beta.length];
            let mut newIndices = vec![0usize; beta.len()];
            // Java: for (int i = 0; i < nvars; i++)
            for i in 0..nvars_u
            {
                // Java: for (int j = 0; j < numberOfRegressors; j++)
                for j in 0..nregs
                {
                    // Java: if (this.vorder[j] == i)
                    if self.vorder[j] == i as i32
                    {
                        // Java: betaNew[i] = beta[ j];
                        betaNew[i] = beta[j];
                        // Java: newIndices[i] = j;
                        newIndices[i] = j;
                    }
                }
            }
            // Java: int idx1 = 0;
            let mut idx1: usize = 0;
            // Java: int idx2;
            let mut idx2: usize;
            // Java: int _i;
            let mut _i: usize;
            // Java: int _j;
            let mut _j: usize;
            // Java: for (int i = 0; i < beta.length; i++)
            for i in 0..beta.len()
            {
                // Java: _i = newIndices[i];
                _i = newIndices[i];
                // Java: for (int j = 0; j <= i; j++, idx1++)
                for j in 0..=i
                {
                    // Java: _j = newIndices[j];
                    _j = newIndices[j];
                    // Java: if (_i > _j)
                    if _i > _j
                    {
                        // Java: idx2 = _i * (_i + 1) / 2 + _j;
                        idx2 = _i * (_i + 1) / 2 + _j;
                    }
                    // Java: else
                    else
                    {
                        // Java: idx2 = _j * (_j + 1) / 2 + _i;
                        idx2 = _j * (_j + 1) / 2 + _i;
                    }
                    // Java: covNew[idx1] = cov[idx2];
                    covNew[idx1] = cov_ref[idx2];
                    idx1 += 1;
                }
            }
            // Java: return new RegressionResults(
            // Java: betaNew, new double[][]{covNew}, true, this.nobs, rnk,
            // Java: this.sumy, this.sumsqy, this.sserr, this.hasIntercept, false);
            todo!()
        }
    }

    // Java: RegressionResults regress(int[] variablesToInclude)
    pub fn regress_arr(&mut self, variablesToInclude: &mut [i32]) -> Box<dyn std::any::Any> {
        let nvars_u = self.nvars as usize;
        // Java: if (variablesToInclude.length > this.nvars)
        if variablesToInclude.len() > nvars_u
        {
            // Java: throw new ModelSpecificationException(
            panic!("ModelSpecificationException: {:?}, len={}, nvars={}",
                    // Java: LocalizedFormats.TOO_MANY_REGRESSORS, variablesToInclude.length, this.nvars);
                    LocalizedFormats::TOO_MANY_REGRESSORS(), variablesToInclude.len(), self.nvars);
        }
        // Java: if (this.nobs <= this.nvars)
        if self.nobs <= self.nvars as i64
        {
            // Java: throw new ModelSpecificationException(
            panic!("ModelSpecificationException: {:?}, nobs={}, nvars={}",
                    // Java: LocalizedFormats.NOT_ENOUGH_DATA_FOR_NUMBER_OF_PREDICTORS,
                    LocalizedFormats::NOT_ENOUGH_DATA_FOR_NUMBER_OF_PREDICTORS(),
                    // Java: this.nobs, this.nvars);
                    self.nobs, self.nvars);
        }
        // Java: Arrays.sort(variablesToInclude);
        variablesToInclude.sort();
        // Java: int iExclude = 0;
        let mut iExclude: usize = 0;
        // Java: for (int i = 0; i < variablesToInclude.length; i++)
        for i in 0..variablesToInclude.len()
        {
            // Java: if (i >= this.nvars)
            if i >= nvars_u
            {
                // Java: throw new ModelSpecificationException(
                panic!("ModelSpecificationException: {:?}, i={}, nvars={}",
                        // Java: LocalizedFormats.INDEX_LARGER_THAN_MAX, i, this.nvars);
                        LocalizedFormats::INDEX_LARGER_THAN_MAX(), i, self.nvars);
            }
            // Java: if (i > 0 && variablesToInclude[i] == variablesToInclude[i - 1])
            if i > 0 && variablesToInclude[i] == variablesToInclude[i - 1]
            {
                // Java: variablesToInclude[i] = -1;
                variablesToInclude[i] = -1;
                // Java: ++iExclude;
                iExclude += 1;
            }
        }
        // Java: int[] series;
        let series: Vec<i32>;
        // Java: if (iExclude > 0)
        if iExclude > 0
        {
            // Java: int j = 0;
            let mut j: usize = 0;
            // Java: series = new int[variablesToInclude.length - iExclude];
            let mut s = vec![0i32; variablesToInclude.len() - iExclude];
            // Java: for (int i = 0; i < variablesToInclude.length; i++)
            for i in 0..variablesToInclude.len()
            {
                // Java: if (variablesToInclude[i] > -1)
                if variablesToInclude[i] > -1
                {
                    // Java: series[j] = variablesToInclude[i];
                    s[j] = variablesToInclude[i];
                    // Java: ++j;
                    j += 1;
                }
            }
            series = s;
        }
        // Java: else
        else
        {
            // Java: series = variablesToInclude;
            series = variablesToInclude.to_vec();
        }
        // Java: reorderRegressors(series, 0);
        self.reorderRegressors(&series, 0);
        // Java: tolset();
        self.tolset();
        // Java: singcheck();
        self.singcheck();
        let series_len = series.len() as i32;
        // Java: double[] beta = this.regcf(series.length);
        let beta = self.regcf(series_len);
        // Java: ss();
        self.ss();
        // Java: double[] cov = this.cov(series.length);
        let cov = self.cov(series_len);
        // Java: int rnk = 0;
        let mut rnk = 0i32;
        // Java: for (int i = 0; i < this.lindep.length; i++)
        for i in 0..self.lindep.len()
        {
            // Java: if (!this.lindep[i])
            if !self.lindep[i]
            {
                // Java: ++rnk;
                rnk += 1;
            }
        }
        // Java: boolean needsReorder = false;
        let mut needsReorder = false;
        // Java: for (int i = 0; i < this.nvars; i++)
        for i in 0..nvars_u
        {
            // Java: if (this.vorder[i] != series[i])
            if self.vorder[i] != series[i]
            {
                // Java: needsReorder = true;
                needsReorder = true;
                // Java: break;
                break;
            }
        }
        // Java: if (!needsReorder)
        if !needsReorder
        {
            // Java: return new RegressionResults(
            // Java: beta, new double[][]{cov}, true, this.nobs, rnk,
            // Java: this.sumy, this.sumsqy, this.sserr, this.hasIntercept, false);
            todo!()
        }
        // Java: else
        else
        {
            // Java: double[] betaNew = new double[beta.length];
            let mut betaNew = vec![0.0; beta.len()];
            // Java: int[] newIndices = new int[beta.length];
            let mut newIndices = vec![0usize; beta.len()];
            // Java: for (int i = 0; i < series.length; i++)
            for i in 0..series.len()
            {
                // Java: for (int j = 0; j < this.vorder.length; j++)
                for j in 0..self.vorder.len()
                {
                    // Java: if (this.vorder[j] == series[i])
                    if self.vorder[j] == series[i]
                    {
                        // Java: betaNew[i] = beta[ j];
                        betaNew[i] = beta[j];
                        // Java: newIndices[i] = j;
                        newIndices[i] = j;
                    }
                }
            }
            let cov_ref = cov.unwrap();
            // Java: double[] covNew = new double[cov.length];
            let mut covNew = vec![0.0; cov_ref.len()];
            // Java: int idx1 = 0;
            let mut idx1: usize = 0;
            // Java: int idx2;
            let mut idx2: usize;
            // Java: int _i;
            let mut _i: usize;
            // Java: int _j;
            let mut _j: usize;
            // Java: for (int i = 0; i < beta.length; i++)
            for i in 0..beta.len()
            {
                // Java: _i = newIndices[i];
                _i = newIndices[i];
                // Java: for (int j = 0; j <= i; j++, idx1++)
                for j in 0..=i
                {
                    // Java: _j = newIndices[j];
                    _j = newIndices[j];
                    // Java: if (_i > _j)
                    if _i > _j
                    {
                        // Java: idx2 = _i * (_i + 1) / 2 + _j;
                        idx2 = _i * (_i + 1) / 2 + _j;
                    }
                    // Java: else
                    else
                    {
                        // Java: idx2 = _j * (_j + 1) / 2 + _i;
                        idx2 = _j * (_j + 1) / 2 + _i;
                    }
                    // Java: covNew[idx1] = cov[idx2];
                    covNew[idx1] = cov_ref[idx2];
                    idx1 += 1;
                }
            }
            // Java: return new RegressionResults(
            // Java: betaNew, new double[][]{covNew}, true, this.nobs, rnk,
            // Java: this.sumy, this.sumsqy, this.sserr, this.hasIntercept, false);
            todo!()
        }
    }

}

impl UpdatingMultipleLinearRegression for MillerUpdatingRegression {
    fn hasIntercept(&self) -> bool {
        self.hasIntercept()
    }

    fn getN(&self) -> i64 {
        self.getN()
    }

    fn addObservation(&self) {
        unimplemented!("Use addObservation(&mut self, x, y) instead")
    }

    fn addObservations(&self) {
        unimplemented!("Use addObservations(&mut self, x, y) instead")
    }

    fn clear(&self) {
        unimplemented!("Use clear(&mut self) instead")
    }

    fn regress(&self) -> Box<dyn std::any::Any> {
        unimplemented!("Use regress(&mut self) instead")
    }

    fn regress_2(&self) -> Box<dyn std::any::Any> {
        unimplemented!("Use regress_arr(&mut self, variablesToInclude) instead")
    }
}
