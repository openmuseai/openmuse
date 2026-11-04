// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.util.Pair
use crate::org::apache::commons::math3::util::Pair;

use std::collections::BTreeMap;

// Java: public class LegendreRuleFactory extends BaseRuleFactory
pub struct LegendreRuleFactory {
    pointsAndWeights: BTreeMap<usize, Pair::Pair<Vec<f64>, Vec<f64>>>,
}

// Java: public class LegendreRuleFactory extends BaseRuleFactory
#[allow(non_snake_case)]
impl LegendreRuleFactory {
    pub fn new() -> Self {
        LegendreRuleFactory {
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
                value: Some(vec![2.0]),
            };
        }

        let previousPoints = {
            let prev_n = (numberOfPoints - 1) as usize;
            if !self.pointsAndWeights.contains_key(&prev_n) {
                let prev_rule = self.computeRule(numberOfPoints - 1);
                self.pointsAndWeights.insert(prev_n, prev_rule);
            }
            self.pointsAndWeights.get(&prev_n).unwrap()
                .key.as_ref().unwrap().clone()
        };

        let mut points = vec![0.0f64; numberOfPoints as usize];
        let mut weights = vec![0.0f64; numberOfPoints as usize];

        let iMax = (numberOfPoints / 2) as usize;
        for i in 0..iMax {
            let mut a: f64 = if i == 0 { -1.0 } else { previousPoints[i - 1] };
            let mut b: f64 = if iMax == 1 { 1.0 } else { previousPoints[i] };

            let mut pma = 1.0f64;
            let mut pa = a;
            let mut pmb = 1.0f64;
            let mut pb = b;

            for j in 1..numberOfPoints {
                let two_j_p_1 = (2 * j + 1) as f64;
                let j_p_1 = (j + 1) as f64;
                let j_f = j as f64;
                let ppa = (two_j_p_1 * a * pa - j_f * pma) / j_p_1;
                let ppb = (two_j_p_1 * b * pb - j_f * pmb) / j_p_1;
                pma = pa;
                pa = ppa;
                pmb = pb;
                pb = ppb;
            }

            let mut c = 0.5 * (a + b);
            let mut pmc;
            let mut pc;
            let mut done = false;
            while !done {
                done = b - a <= f64::EPSILON * c.abs() * 2.0;
                pmc = 1.0;
                pc = c;
                for j in 1..numberOfPoints {
                    let ppc = ((2 * j + 1) as f64 * c * pc - j as f64 * pmc) / (j + 1) as f64;
                    pmc = pc;
                    pc = ppc;
                }
                if !done {
                    if pa * pc <= 0.0 {
                        b = c;
                        pmb = pmc;
                        pb = pc;
                    } else {
                        a = c;
                        pma = pmc;
                        pa = pc;
                    }
                    c = 0.5 * (a + b);
                }
            }

            pmc = 1.0;
            pc = c;
            for j in 1..numberOfPoints {
                let ppc = ((2 * j + 1) as f64 * c * pc - j as f64 * pmc) / (j + 1) as f64;
                pmc = pc;
                pc = ppc;
            }

            let d = numberOfPoints as f64 * (pmc - c * pc);
            let w = 2.0 * (1.0 - c * c) / (d * d);

            points[i] = c;
            weights[i] = w;

            let idx = numberOfPoints as usize - i - 1;
            points[idx] = -c;
            weights[idx] = w;
        }

        if numberOfPoints % 2 != 0 {
            let mut pmc = 1.0f64;
            let mut j = 1;
            while j < numberOfPoints {
                pmc = -(j as f64) * pmc / (j + 1) as f64;
                j += 2;
            }
            let d = numberOfPoints as f64 * pmc;
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
