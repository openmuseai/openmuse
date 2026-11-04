// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.exception.NotANumberException
use crate::org::apache::commons::math3::exception::NotANumberException;
// Java: import org.apache.commons.math3.random.RandomDataGenerator
use crate::org::apache::commons::math3::random::RandomDataGenerator;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: NaNStrategy DEFAULT_NAN_STRATEGY
static DEFAULT_NAN_STRATEGY: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: TiesStrategy DEFAULT_TIES_STRATEGY
static DEFAULT_TIES_STRATEGY: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();

// Java: public class NaturalRanking implements RankingAlgorithm
pub struct NaturalRanking {
    // Java field: NaNStrategy nanStrategy
    pub(crate) nanStrategy: Box<dyn std::any::Any>,
    // Java field: TiesStrategy tiesStrategy
    pub(crate) tiesStrategy: Box<dyn std::any::Any>,
    // Java field: RandomDataGenerator randomData
    randomData: Option<Box<dyn std::any::Any>>,
}

// Java: public class IntDoublePair
pub struct IntDoublePair {
    // Java field: double value
    pub(crate) value: f64,
    // Java field: int position
    pub(crate) position: i32,
}

// Java: public class NaturalRanking implements RankingAlgorithm
impl NaturalRanking {
    // Java: NaturalRanking()
    pub fn NaturalRanking(&self) {
        // Java: super();
        // Java: tiesStrategy = DEFAULT_TIES_STRATEGY;
        // Java: nanStrategy = DEFAULT_NAN_STRATEGY;
        // Java: randomData = null;
    }

    // Java: NaturalRanking(TiesStrategy tiesStrategy)
    pub fn NaturalRanking_2(&self) {
        // Java: super();
        // Java: this.tiesStrategy = tiesStrategy;
        // Java: nanStrategy = DEFAULT_NAN_STRATEGY;
        // Java: randomData = new RandomDataGenerator();
    }

    // Java: NaturalRanking(NaNStrategy nanStrategy)
    pub fn NaturalRanking_3(&self) {
        // Java: super();
        // Java: this.nanStrategy = nanStrategy;
        // Java: tiesStrategy = DEFAULT_TIES_STRATEGY;
        // Java: randomData = null;
    }

    // Java: NaturalRanking(NaNStrategy nanStrategy, TiesStrategy tiesStrategy)
    pub fn NaturalRanking_4(&self) {
        // Java: super();
        // Java: this.nanStrategy = nanStrategy;
        // Java: this.tiesStrategy = tiesStrategy;
        // Java: randomData = new RandomDataGenerator();
    }

    // Java: NaturalRanking(RandomGenerator randomGenerator)
    pub fn NaturalRanking_5(&self) {
        // Java: super();
        // Java: this.tiesStrategy = TiesStrategy.RANDOM;
        // Java: nanStrategy = DEFAULT_NAN_STRATEGY;
        // Java: randomData = new RandomDataGenerator(randomGenerator);
    }

    // Java: NaturalRanking()
    pub fn NaturalRanking_6(&self) {
    }

    // Java: NaNStrategy getNanStrategy()
    pub fn getNanStrategy(&self) -> Box<dyn std::any::Any> {
        // Java: return nanStrategy;
        todo!()
    }

    // Java: TiesStrategy getTiesStrategy()
    pub fn getTiesStrategy(&self) -> Box<dyn std::any::Any> {
        // Java: return tiesStrategy;
        todo!()
    }

    // Java: double[] rank(double[] data)
    pub fn rank(&self) -> Vec<f64> {
        // Java: // Array recording initial positions of data to be ranked
        // Java: IntDoublePair[] ranks = new IntDoublePair[data.length];
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: ranks[i] = new IntDoublePair(data[i], i);
        }
        // Java: // Recode, remove or record positions of NaNs
        // Java: List<Integer> nanPositions = null;
        // Java: switch (nanStrategy)
        {
            // Java: case MAXIMAL: // Replace NaNs with +INFs
                // Java: recodeNaNs(ranks, Double.POSITIVE_INFINITY);
                // Java: break;
            // Java: case MINIMAL: // Replace NaNs with -INFs
                // Java: recodeNaNs(ranks, Double.NEGATIVE_INFINITY);
                // Java: break;
            // Java: case REMOVED: // Drop NaNs from data
                // Java: ranks = removeNaNs(ranks);
                // Java: break;
            // Java: case FIXED:   // Record positions of NaNs
                // Java: nanPositions = getNanPositions(ranks);
                // Java: break;
            // Java: case FAILED:
                // Java: nanPositions = getNanPositions(ranks);
                // Java: if (nanPositions.size() > 0)
                {
                    // Java: throw new NotANumberException();
                }
                // Java: break;
            // Java: default: // this should not happen unless NaNStrategy enum is changed
                // Java: throw new MathInternalError();
        }
        // Java: // Sort the IntDoublePairs
        // Java: Arrays.sort(ranks);
        // Java: // Walk the sorted array, filling output array using sorted positions,
        // Java: // resolving ties as we go
        // Java: double[] out = new double[ranks.length];
        // Java: int pos = 1;  // position in sorted array
        // Java: out[ranks[0].getPosition()] = pos;
        // Java: List<Integer> tiesTrace = new ArrayList<Integer>();
        // Java: tiesTrace.add(ranks[0].getPosition());
        // Java: for (int i = 1; i < ranks.length; i++)
        {
            // Java: if (Double.compare(ranks[i].getValue(), ranks[i - 1].getValue()) > 0)
            {
                // Java: // tie sequence has ended (or had length 1)
                // Java: pos = i + 1;
                // Java: if (tiesTrace.size() > 1) {  // if seq is nontrivial, resolve
                    // Java: resolveTie(out, tiesTrace);
                // Java: tiesTrace = new ArrayList<Integer>();
                // Java: tiesTrace.add(ranks[i].getPosition());
            }
            // Java: else
            {
                // Java: // tie sequence continues
                // Java: tiesTrace.add(ranks[i].getPosition());
            }
            // Java: out[ranks[i].getPosition()] = pos;
        }
        // Java: if (tiesTrace.size() > 1) {  // handle tie sequence at end
            // Java: resolveTie(out, tiesTrace);
        // Java: if (nanStrategy == NaNStrategy.FIXED)
        {
            // Java: restoreNaNs(out, nanPositions);
        }
        // Java: return out;
        todo!()
    }

    // Java: IntDoublePair[] removeNaNs(IntDoublePair[] ranks)
    pub fn removeNaNs(&self) -> Box<dyn std::any::Any> {
        // Java: if (!containsNaNs(ranks))
        {
            // Java: return ranks;
        }
        // Java: IntDoublePair[] outRanks = new IntDoublePair[ranks.length];
        // Java: int j = 0;
        // Java: for (int i = 0; i < ranks.length; i++)
        {
            // Java: if (Double.isNaN(ranks[i].getValue()))
            {
                // Java: // drop, but adjust original ranks of later elements
                // Java: for (int k = i + 1; k < ranks.length; k++)
                {
                    // Java: ranks[k] = new IntDoublePair(
                            // Java: ranks[k].getValue(), ranks[k].getPosition() - 1);
                }
            }
            // Java: else
            {
                // Java: outRanks[j] = new IntDoublePair(
                        // Java: ranks[i].getValue(), ranks[i].getPosition());
                // Java: j++;
            }
        }
        // Java: IntDoublePair[] returnRanks = new IntDoublePair[j];
        // Java: System.arraycopy(outRanks, 0, returnRanks, 0, j);
        // Java: return returnRanks;
        todo!()
    }

    // Java: void recodeNaNs(IntDoublePair[] ranks, double value)
    pub fn recodeNaNs(&self) {
        // Java: for (int i = 0; i < ranks.length; i++)
        {
            // Java: if (Double.isNaN(ranks[i].getValue()))
            {
                // Java: ranks[i] = new IntDoublePair(
                        // Java: value, ranks[i].getPosition());
            }
        }
    }

    // Java: boolean containsNaNs(IntDoublePair[] ranks)
    pub fn containsNaNs(&self) -> bool {
        // Java: for (int i = 0; i < ranks.length; i++)
        {
            // Java: if (Double.isNaN(ranks[i].getValue()))
            {
                // Java: return true;
            }
        }
        // Java: return false;
        todo!()
    }

    // Java: void resolveTie(double[] ranks, List<Integer> tiesTrace)
    pub fn resolveTie(&self) {
        // Java: // constant value of ranks over tiesTrace
        // Java: final double c = ranks[tiesTrace.get(0)];
        // Java: // length of sequence of tied ranks
        // Java: final int length = tiesTrace.size();
        // Java: switch (tiesStrategy)
        {
            // Java: case  AVERAGE:  // Replace ranks with average
                // Java: fill(ranks, tiesTrace, (2 * c + length - 1) / 2d);
                // Java: break;
            // Java: case MAXIMUM:   // Replace ranks with maximum values
                // Java: fill(ranks, tiesTrace, c + length - 1);
                // Java: break;
            // Java: case MINIMUM:   // Replace ties with minimum
                // Java: fill(ranks, tiesTrace, c);
                // Java: break;
            // Java: case RANDOM:    // Fill with random integral values in [c, c + length - 1]
                // Java: Iterator<Integer> iterator = tiesTrace.iterator();
                // Java: long f = FastMath.round(c);
                // Java: while (iterator.hasNext())
                {
                    // Java: // No advertised exception because args are guaranteed valid
                    // Java: ranks[iterator.next()] =
                        // Java: randomData.nextLong(f, f + length - 1);
                }
                // Java: break;
            // Java: case SEQUENTIAL:  // Fill sequentially from c to c + length - 1
                // Java: // walk and fill
                // Java: iterator = tiesTrace.iterator();
                // Java: f = FastMath.round(c);
                // Java: int i = 0;
                // Java: while (iterator.hasNext())
                {
                    // Java: ranks[iterator.next()] = f + i++;
                }
                // Java: break;
            // Java: default: // this should not happen unless TiesStrategy enum is changed
                // Java: throw new MathInternalError();
        }
    }

    // Java: void fill(double[] data, List<Integer> tiesTrace, double value)
    pub fn fill(&self) {
        // Java: Iterator<Integer> iterator = tiesTrace.iterator();
        // Java: while (iterator.hasNext())
        {
            // Java: data[iterator.next()] = value;
        }
    }

    // Java: void restoreNaNs(double[] ranks, List<Integer> nanPositions)
    pub fn restoreNaNs(&self) {
        // Java: if (nanPositions.size() == 0)
        {
            // Java: return;
        }
        // Java: Iterator<Integer> iterator = nanPositions.iterator();
        // Java: while (iterator.hasNext())
        {
            // Java: ranks[iterator.next().intValue()] = Double.NaN;
        }
    }

    // Java: List<Integer> getNanPositions(IntDoublePair[] ranks)
    pub fn getNanPositions(&self) -> Box<dyn std::any::Any> {
        // Java: ArrayList<Integer> out = new ArrayList<Integer>();
        // Java: for (int i = 0; i < ranks.length; i++)
        {
            // Java: if (Double.isNaN(ranks[i].getValue()))
            {
                // Java: out.add(Integer.valueOf(i));
            }
        }
        // Java: return out;
        todo!()
    }

}

// Java: public class IntDoublePair
impl IntDoublePair {
    // Java: int compareTo(IntDoublePair other)
    pub fn compareTo(&self) -> i32 {
            // Java: return Double.compare(value, other.value);
        todo!()
    }

    // Java: double getValue()
    pub fn getValue(&self) -> f64 {
            // Java: return value;
        todo!()
    }

    // Java: int getPosition()
    pub fn getPosition(&self) -> i32 {
            // Java: return position;
        todo!()
    }

}
