// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.util.Pair
use crate::org::apache::commons::math3::util::Pair;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

use std::collections::BTreeMap;

// Java field: double SQRT_PI
pub(crate) const SQRT_PI: f64 = 1.77245385090551602729;
// Java field: double H0
pub(crate) const H0: f64 = 7.5112554446494248286e-1;
// Java field: double H1
pub(crate) const H1: f64 = 1.0622519320271969145;

// Java: public class HermiteRuleFactory extends BaseRuleFactory
pub struct HermiteRuleFactory {
    pointsAndWeights: BTreeMap<usize, Pair::Pair<Vec<f64>, Vec<f64>>>,
}

// Java: public class HermiteRuleFactory extends BaseRuleFactory
#[allow(non_snake_case)]
impl HermiteRuleFactory {
    pub fn new() -> Self {
        HermiteRuleFactory {
            pointsAndWeights: BTreeMap::new(),
        }
    }

    pub fn getRuleInternal(&mut self, numberOfPoints: usize) -> &Pair::Pair<Vec<f64>, Vec<f64>> {
        if !self.pointsAndWeights.contains_key(&numberOfPoints) {
            let rule = self.computeRule(numberOfPoints as i32);
            self.pointsAndWeights.insert(numberOfPoints, rule);
        }
        self.pointsAndWeights.get(&numberOfPoints).unwrap()
    }

    // Java: protected Pair<Double[], Double[]> computeRule(int numberOfPoints)
    pub fn computeRule(&mut self, numberOfPoints: i32) -> Pair::Pair<Vec<f64>, Vec<f64>> {
        if numberOfPoints == 1 {
            return Pair::Pair {
                key: Some(vec![0.0]),
                value: Some(vec![SQRT_PI]),
            };
        }

        let lastNumPoints = (numberOfPoints - 1) as usize;
        let previousPoints = {
            if !self.pointsAndWeights.contains_key(&lastNumPoints) {
                let prev_rule = self.computeRule(numberOfPoints - 1);
                self.pointsAndWeights.insert(lastNumPoints, prev_rule);
            }
            self.pointsAndWeights.get(&lastNumPoints).unwrap()
                .key.as_ref().unwrap().clone()
        };

        let mut points = vec![0.0f64; numberOfPoints as usize];
        let mut weights = vec![0.0f64; numberOfPoints as usize];

        let sqrtTwoTimesLastNumPoints = (2.0 * lastNumPoints as f64).sqrt();
        let sqrtTwoTimesNumPoints = (2.0 * numberOfPoints as f64).sqrt();

        let iMax = (numberOfPoints / 2) as usize;
        for i in 0..iMax {
            let mut a = if i == 0 {
                -sqrtTwoTimesLastNumPoints
            } else {
                previousPoints[i - 1]
            };
            let mut b = if iMax == 1 {
                -0.5
            } else {
                previousPoints[i]
            };

            let mut hma = H0;
            let mut ha = H1 * a;
            let mut hmb = H0;
            let mut hb = H1 * b;
            for j in 1..numberOfPoints {
                let jp1 = (j + 1) as f64;
                let s = (2.0 / jp1).sqrt();
                let sm = (j as f64 / jp1).sqrt();
                let hpa = s * a * ha - sm * hma;
                let hpb = s * b * hb - sm * hmb;
                hma = ha;
                ha = hpa;
                hmb = hb;
                hb = hpb;
            }

            let mut c = 0.5 * (a + b);
            let mut hmc;
            let mut hc;
            let mut done = false;
            while !done {
                done = b - a <= f64::EPSILON * c.abs() * 2.0;
                hmc = H0;
                hc = H1 * c;
                for j in 1..numberOfPoints {
                    let jp1 = (j + 1) as f64;
                    let s = (2.0 / jp1).sqrt();
                    let sm = (j as f64 / jp1).sqrt();
                    let hpc = s * c * hc - sm * hmc;
                    hmc = hc;
                    hc = hpc;
                }
                if !done {
                    if ha * hc < 0.0 {
                        b = c;
                        hmb = hmc;
                        hb = hc;
                    } else {
                        a = c;
                        hma = hmc;
                        ha = hc;
                    }
                    c = 0.5 * (a + b);
                }
            }

            hmc = H0;
            hc = H1 * c;
            for j in 1..numberOfPoints {
                let jp1 = (j + 1) as f64;
                let s = (2.0 / jp1).sqrt();
                let sm = (j as f64 / jp1).sqrt();
                let hpc = s * c * hc - sm * hmc;
                hmc = hc;
                hc = hpc;
            }

            let d = sqrtTwoTimesNumPoints * hmc;
            let w = 2.0 / (d * d);

            points[i] = c;
            weights[i] = w;

            let idx = lastNumPoints - i;
            points[idx] = -c;
            weights[idx] = w;
        }

        if numberOfPoints % 2 != 0 {
            let mut hm = H0;
            let mut j = 1;
            while j < numberOfPoints {
                let jp1 = (j + 1) as f64;
                hm = -(j as f64 / jp1).sqrt() * hm;
                j += 2;
            }
            let d = sqrtTwoTimesNumPoints * hm;
            let w = 2.0 / (d * d);

            points[iMax] = 0.0;
            weights[iMax] = w;
        }

        Pair::Pair {
            key: Some(points),
            value: Some(weights),
        }
    }
}
