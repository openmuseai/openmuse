//! Trendline maths: least-squares fits behind chart trendlines and moving averages.
//!
//! Exponential, logarithmic and power trendlines are linear fits of transformed data
//! (`ln y` against `x`, `y` against `ln x`, `ln y` against `ln x`), the way Excel documents them;
//! points where the transform is undefined (non-positive values) are skipped and R² is reported
//! for the transformed fit. Polynomials solve the normal equations on centred, scaled x.

use gridcraft_model::{Trendline, TrendlineKind};

/// A fitted regression trendline.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TrendFit {
    kind: TrendlineKind,
    /// Coefficients `c0..=cd` of `Y = Σ cₖ·Xᵏ` in the transformed variables, for the equation.
    pub coef: Vec<f64>,
    /// The same polynomial in `u = (X - center) / scale`, for stable evaluation.
    center: f64,
    scale: f64,
    a: Vec<f64>,
    /// Coefficient of determination; `None` when undefined (all Y equal).
    pub r2: Option<f64>,
}

/// Polynomial degree in the transformed variable; `None` for moving averages.
fn degree(k: TrendlineKind) -> Option<usize> {
    match k {
        TrendlineKind::Linear | TrendlineKind::Exponential | TrendlineKind::Logarithmic | TrendlineKind::Power => Some(1),
        TrendlineKind::Polynomial(o) => Some(usize::from(o.clamp(2, 6))),
        TrendlineKind::MovingAverage(_) => None,
    }
}

fn log_x(k: TrendlineKind) -> bool {
    matches!(k, TrendlineKind::Logarithmic | TrendlineKind::Power)
}

fn log_y(k: TrendlineKind) -> bool {
    matches!(k, TrendlineKind::Exponential | TrendlineKind::Power)
}

/// Fits `t` to the points `(x, y)`. `None` for moving averages and degenerate data (too few
/// usable points, all x equal, a singular system, non-finite results).
pub(crate) fn fit_trendline(t: &Trendline, pts: &[(f64, f64)]) -> Option<TrendFit> {
    let kind = t.kind;
    let d = degree(kind)?;
    let data: Vec<(f64, f64)> = pts
        .iter()
        .filter(|(x, y)| x.is_finite() && y.is_finite() && (!log_x(kind) || *x > 0.0) && (!log_y(kind) || *y > 0.0))
        .map(|&(x, y)| (if log_x(kind) { x.ln() } else { x }, if log_y(kind) { y.ln() } else { y }))
        .collect();
    if data.len() < 2 || data.len() <= d {
        return None;
    }
    // Set Intercept applies to linear, exponential and polynomial trendlines; it fixes Y at x = 0.
    let fixed = match kind {
        TrendlineKind::Linear | TrendlineKind::Polynomial(_) => t.intercept,
        TrendlineKind::Exponential => t.intercept.filter(|b| *b > 0.0).map(f64::ln),
        _ => None,
    }
    .filter(|b| b.is_finite());
    let n = data.len() as f64;
    let center = if fixed.is_some() { 0.0 } else { data.iter().map(|p| p.0).sum::<f64>() / n };
    let scale = data.iter().map(|p| (p.0 - center).abs()).fold(0.0f64, f64::max);
    if !(scale.is_finite() && scale > 0.0) {
        return None;
    }
    let first = usize::from(fixed.is_some());
    let b0 = fixed.unwrap_or(0.0);
    // Normal equations over the unknown powers `first..=d` of u.
    let m = d + 1 - first;
    let mut ata = vec![vec![0.0f64; m]; m];
    let mut aty = vec![0.0f64; m];
    for &(x, y) in &data {
        let u = (x - center) / scale;
        let pows: Vec<f64> = (first..=d).map(|k| u.powi(k as i32)).collect();
        for (i, pi) in pows.iter().enumerate() {
            if let Some(v) = aty.get_mut(i) {
                *v += pi * (y - b0);
            }
            for (j, pj) in pows.iter().enumerate() {
                if let Some(v) = ata.get_mut(i).and_then(|r| r.get_mut(j)) {
                    *v += pi * pj;
                }
            }
        }
    }
    let sol = solve(ata, aty)?;
    let mut a = vec![b0; d + 1];
    for (k, v) in sol.into_iter().enumerate() {
        if let Some(slot) = a.get_mut(k + first) {
            *slot = v;
        }
    }
    let coef = expand(&a, center, scale)?;
    let fit = TrendFit { kind, coef, center, scale, a, r2: None };
    let mean = data.iter().map(|p| p.1).sum::<f64>() / n;
    let (mut ss_res, mut ss_tot) = (0.0f64, 0.0f64);
    for &(x, y) in &data {
        ss_res += (y - fit.poly(x)).powi(2);
        // A set intercept is LINEST(y - b, x, FALSE): the total sum of squares is not centred.
        ss_tot += if fixed.is_some() { (y - b0).powi(2) } else { (y - mean).powi(2) };
    }
    let r2 = (ss_tot > 0.0 && ss_tot.is_finite() && ss_res.is_finite()).then(|| 1.0 - ss_res / ss_tot);
    Some(TrendFit { r2, ..fit })
}

/// Gaussian elimination with partial pivoting; `None` when (nearly) singular.
fn solve(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    let big = a.iter().flatten().fold(0.0f64, |m, v| m.max(v.abs()));
    if !(big.is_finite() && big > 0.0) {
        return None;
    }
    for col in 0..n {
        let piv = (col..n).max_by(|&i, &j| {
            let vi = a.get(i).and_then(|r| r.get(col)).map_or(0.0, |v| v.abs());
            let vj = a.get(j).and_then(|r| r.get(col)).map_or(0.0, |v| v.abs());
            vi.total_cmp(&vj)
        })?;
        a.swap(col, piv);
        b.swap(col, piv);
        let p = *a.get(col)?.get(col)?;
        if !p.is_finite() || p.abs() <= big * 1e-13 {
            return None;
        }
        let prow = a.get(col)?.clone();
        let pb = *b.get(col)?;
        for r in col + 1..n {
            let row = a.get_mut(r)?;
            let f = *row.get(col)? / p;
            for (v, pv) in row.iter_mut().zip(&prow).skip(col) {
                *v -= f * pv;
            }
            *b.get_mut(r)? -= f * pb;
        }
    }
    let mut x = vec![0.0f64; n];
    for r in (0..n).rev() {
        let row = a.get(r)?;
        let s: f64 = (r + 1..n).map(|c| row.get(c).copied().unwrap_or(0.0) * x.get(c).copied().unwrap_or(0.0)).sum();
        let v = (b.get(r)? - s) / row.get(r)?;
        *x.get_mut(r)? = v;
    }
    x.iter().all(|v| v.is_finite()).then_some(x)
}

/// Coefficients in X of `Σ aₖ·((X - c) / s)ᵏ` (binomial expansion).
fn expand(a: &[f64], c: f64, s: f64) -> Option<Vec<f64>> {
    let mut out = vec![0.0f64; a.len()];
    for (k, ak) in a.iter().enumerate() {
        let lead = ak / s.powi(k as i32);
        let mut binom = 1.0f64;
        for j in 0..=k {
            // C(k, j) · (-c)^(k-j) · X^j
            if let Some(o) = out.get_mut(j) {
                *o += lead * binom * (-c).powi((k - j) as i32);
            }
            binom = binom * (k - j) as f64 / (j + 1) as f64;
        }
    }
    out.iter().all(|v| v.is_finite()).then_some(out)
}

impl TrendFit {
    /// The fitted polynomial at transformed `x` (Horner in u).
    fn poly(&self, x: f64) -> f64 {
        let u = (x - self.center) / self.scale;
        self.a.iter().rev().fold(0.0, |acc, c| acc * u + c)
    }

    /// The trendline's y at `x`; `None` outside its domain (x ≤ 0 for log/power) or when not finite.
    pub(crate) fn eval(&self, x: f64) -> Option<f64> {
        let xx = if log_x(self.kind) { if x > 0.0 { x.ln() } else { return None } } else { x };
        let p = self.poly(xx);
        let y = if log_y(self.kind) { p.exp() } else { p };
        y.is_finite().then_some(y)
    }

    /// Equation label text, e.g. `y = 2x + 1`, `y = 0.5x² - 2x + 1`, `y = 3e^(0.2x)`.
    pub(crate) fn equation(&self) -> String {
        let c0 = self.coef.first().copied().unwrap_or(0.0);
        let c1 = self.coef.get(1).copied().unwrap_or(0.0);
        match self.kind {
            TrendlineKind::Exponential => format!("y = {}e^({}x)", num_text(c0.exp()), num_text(c1)),
            TrendlineKind::Power => format!("y = {}x^{}", num_text(c0.exp()), num_text(c1)),
            TrendlineKind::Logarithmic => terms(&[(c1, "ln(x)"), (c0, "")]),
            _ => {
                let parts: Vec<(f64, &str)> = self.coef.iter().enumerate().rev().map(|(k, c)| (*c, POWERS.get(k).copied().unwrap_or("x"))).collect();
                terms(&parts)
            }
        }
    }

    /// R² label text, e.g. `R² = 0.9963`.
    pub(crate) fn r_squared(&self) -> String {
        match self.r2 {
            Some(r) => format!("R² = {}", num_text(r)),
            None => "R² = #N/A".into(),
        }
    }
}

const POWERS: [&str; 7] = ["", "x", "x²", "x³", "x⁴", "x⁵", "x⁶"];

/// `y = a·t1 ± b·t2 …`, skipping terms whose coefficient shows as 0.
fn terms(parts: &[(f64, &str)]) -> String {
    let mut s = String::from("y = ");
    let mut first = true;
    for (c, t) in parts {
        let txt = num_text(c.abs());
        if txt == "0" {
            continue;
        }
        let neg = *c < 0.0;
        if first {
            if neg {
                s.push('-');
            }
        } else {
            s.push_str(if neg { " - " } else { " + " });
        }
        // A unit coefficient on a power of x is left out: `y = x² - x + 1`.
        if txt != "1" || t.is_empty() {
            s.push_str(&txt);
        }
        s.push_str(t);
        first = false;
    }
    if first {
        s.push('0');
    }
    s
}

/// A coefficient as trendline labels show it: up to four decimals, scientific when tiny or huge.
pub(crate) fn num_text(v: f64) -> String {
    if !v.is_finite() {
        return "#NUM!".into();
    }
    let a = v.abs();
    if a == 0.0 {
        return "0".into();
    }
    if !(1e-4..1e11).contains(&a) {
        let s = format!("{v:.4E}");
        let (mant, exp) = s.split_once('E').unwrap_or((s.as_str(), "0"));
        let mant = trim_zeros(mant);
        let e: i32 = exp.parse().unwrap_or(0);
        return format!("{mant}E{}{:02}", if e < 0 { '-' } else { '+' }, e.unsigned_abs());
    }
    let s = trim_zeros(&format!("{v:.4}"));
    if s == "-0" { "0".into() } else { s }
}

fn trim_zeros(s: &str) -> String {
    if s.contains('.') { s.trim_end_matches('0').trim_end_matches('.').to_string() } else { s.to_string() }
}

/// Moving average of `period` consecutive points in plotted order: point `i` (from
/// `period - 1`) is `(xᵢ, mean(yᵢ₋ₚ₊₁..=yᵢ))`. Empty when there are fewer points than `period`.
pub(crate) fn moving_average(pts: &[(f64, f64)], period: u32) -> Vec<(f64, f64)> {
    let p = period.clamp(2, 255) as usize;
    let pts: Vec<(f64, f64)> = pts.iter().copied().filter(|(x, y)| x.is_finite() && y.is_finite()).collect();
    pts.windows(p).filter_map(|w| w.last().map(|(x, _)| (*x, w.iter().map(|q| q.1).sum::<f64>() / p as f64))).collect()
}

/// Excel's automatic trendline name for the legend, e.g. `Linear (Sales)`.
pub(crate) fn auto_name(kind: TrendlineKind, series: &str) -> String {
    let k = match kind {
        TrendlineKind::Linear => "Linear".to_string(),
        TrendlineKind::Exponential => "Expon.".into(),
        TrendlineKind::Logarithmic => "Log.".into(),
        TrendlineKind::Polynomial(_) => "Poly.".into(),
        TrendlineKind::Power => "Power".into(),
        TrendlineKind::MovingAverage(p) => format!("{} per. Mov. Avg.", p.clamp(2, 255)),
    };
    format!("{k} ({series})")
}
