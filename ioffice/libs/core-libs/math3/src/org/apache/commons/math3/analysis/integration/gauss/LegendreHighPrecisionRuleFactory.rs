// Java: import java.math.BigDecimal
// Java: import java.math.MathContext
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.util.Pair
use crate::org::apache::commons::math3::util::Pair;

use std::collections::BTreeMap;

// Java: public class LegendreHighPrecisionRuleFactory extends BaseRuleFactory
pub struct LegendreHighPrecisionRuleFactory {
    // Java field: MathContext mContext
    // Java field: BigDecimal two
    two: f64,
    // Java field: BigDecimal minusOne
    minusOne: f64,
    // Java field: BigDecimal oneHalf
    oneHalf: f64,
    pointsAndWeights: BTreeMap<usize, Pair::Pair<Vec<f64>, Vec<f64>>>,
}

// Java: public class LegendreHighPrecisionRuleFactory extends BaseRuleFactory
#[allow(non_snake_case)]
impl LegendreHighPrecisionRuleFactory {
    // Java: LegendreHighPrecisionRuleFactory()
    pub fn new() -> Self {
        // Java: this(MathContext.DECIMAL128);
        Self::new_with_precision()
    }

    // Java: LegendreHighPrecisionRuleFactory(MathContext mContext)
    pub fn new_with_precision() -> Self {
        // Java: this.mContext = mContext;
        // Java: two = new BigDecimal("2", mContext);
        // Java: minusOne = new BigDecimal("-1", mContext);
        // Java: oneHalf = new BigDecimal("0.5", mContext);
        LegendreHighPrecisionRuleFactory {
            two: 2.0,
            minusOne: -1.0,
            oneHalf: 0.5,
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

    // Java: protected Pair<BigDecimal[], BigDecimal[]> computeRule(int numberOfPoints)
    pub fn computeRule(&mut self, numberOfPoints: i32) -> Pair::Pair<Vec<f64>, Vec<f64>> {
        if numberOfPoints == 1 {
            return Pair::Pair {
                key: Some(vec![0.0]),
                value: Some(vec![self.two]),
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
            let mut a = if i == 0 { self.minusOne } else { previousPoints[i - 1] };
            let mut b = if iMax == 1 { 1.0 } else { previousPoints[i] };

            let mut pma = 1.0f64;
            let mut pa = a;
            let mut pmb = 1.0f64;
            let mut pb = b;

            for j in 1..numberOfPoints {
                let b_two_j_p_1 = (2 * j + 1) as f64;
                let b_j = j as f64;
                let b_j_p_1 = (j + 1) as f64;

                let ppa = (b_two_j_p_1 * a * pa - b_j * pma) / b_j_p_1;
                let ppb = (b_two_j_p_1 * b * pb - b_j * pmb) / b_j_p_1;

                pma = pa;
                pa = ppa;
                pmb = pb;
                pb = ppb;
            }

            let mut c = (a + b) * self.oneHalf;
            let mut pmc;
            let mut pc;
            let mut done = false;
            while !done {
                let tmp_diff = b - a;
                let ulp_c = f64::EPSILON * c.abs().max(f64::MIN_POSITIVE);
                done = tmp_diff <= ulp_c * 10.0;
                pmc = 1.0;
                pc = c;
                for j in 1..numberOfPoints {
                    let b_two_j_p_1 = (2 * j + 1) as f64;
                    let b_j = j as f64;
                    let b_j_p_1 = (j + 1) as f64;

                    let ppc = (b_two_j_p_1 * c * pc - b_j * pmc) / b_j_p_1;
                    pmc = pc;
                    pc = ppc;
                }
                if !done {
                    if pa.signum() * pc.signum() <= 0.0 {
                        b = c;
                        pmb = pmc;
                        pb = pc;
                    } else {
                        a = c;
                        pma = pmc;
                        pa = pc;
                    }
                    c = (a + b) * self.oneHalf;
                }
            }

            pmc = 1.0;
            pc = c;
            for j in 1..numberOfPoints {
                let b_two_j_p_1 = (2 * j + 1) as f64;
                let b_j = j as f64;
                let b_j_p_1 = (j + 1) as f64;
                let ppc = (b_two_j_p_1 * c * pc - b_j * pmc) / b_j_p_1;
                pmc = pc;
                pc = ppc;
            }

            let nP = numberOfPoints as f64;
            let tmp1 = (pmc - c * pc) * nP;
            let tmp1_sq = tmp1 * tmp1;
            let tmp2 = self.two * (1.0 - c * c) / tmp1_sq;

            points[i] = c;
            weights[i] = tmp2;

            let idx = numberOfPoints as usize - i - 1;
            points[idx] = -c;
            weights[idx] = tmp2;
        }

        if numberOfPoints % 2 != 0 {
            let mut pmc = 1.0f64;
            let mut j = 1;
            while j < numberOfPoints {
                let b_j = j as f64;
                let b_j_p_1 = (j + 1) as f64;
                pmc = -pmc * b_j / b_j_p_1;
                j += 2;
            }

            let nP = numberOfPoints as f64;
            let tmp1 = pmc * nP;
            let tmp2 = self.two / (tmp1 * tmp1);

            points[iMax] = 0.0;
            weights[iMax] = tmp2;
        }

        Pair::Pair {
            key: Some(points),
            value: Some(weights),
        }
    }
}
