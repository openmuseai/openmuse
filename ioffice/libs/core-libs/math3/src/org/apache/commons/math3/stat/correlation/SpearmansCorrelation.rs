// Java: import java.util.ArrayList
// Java: import java.util.HashSet
// Java: import java.util.List
// Java: import java.util.Set
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.linear.BlockRealMatrix
use crate::org::apache::commons::math3::linear::BlockRealMatrix;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.stat.ranking.NaNStrategy
use crate::org::apache::commons::math3::stat::ranking::NaNStrategy;
// Java: import org.apache.commons.math3.stat.ranking.NaturalRanking
use crate::org::apache::commons::math3::stat::ranking::NaturalRanking;
// Java: import org.apache.commons.math3.stat.ranking.RankingAlgorithm
use crate::org::apache::commons::math3::stat::ranking::RankingAlgorithm;

// Java: public class SpearmansCorrelation
pub struct SpearmansCorrelation {
    // Java field: RealMatrix data
    data: Option<Box<dyn std::any::Any>>,
    // Java field: RankingAlgorithm rankingAlgorithm
    rankingAlgorithm: Box<dyn std::any::Any>,
    // Java field: PearsonsCorrelation rankCorrelation
    pub(crate) rankCorrelation: Box<dyn std::any::Any>,
}

// Java: public class SpearmansCorrelation
impl SpearmansCorrelation {
    // Java: SpearmansCorrelation()
    pub fn SpearmansCorrelation(&self) {
        // Java: this(new NaturalRanking());
    }

    // Java: SpearmansCorrelation(final RankingAlgorithm rankingAlgorithm)
    pub fn SpearmansCorrelation_2(&self) {
        // Java: data = null;
        // Java: this.rankingAlgorithm = rankingAlgorithm;
        // Java: rankCorrelation = null;
    }

    // Java: SpearmansCorrelation(final RealMatrix dataMatrix)
    pub fn SpearmansCorrelation_3(&self) {
        // Java: this(dataMatrix, new NaturalRanking());
    }

    // Java: SpearmansCorrelation(final RealMatrix dataMatrix, final RankingAlgorithm rankingAlgorithm)
    pub fn SpearmansCorrelation_4(&self) {
        // Java: this.rankingAlgorithm = rankingAlgorithm;
        // Java: this.data = rankTransform(dataMatrix);
        // Java: rankCorrelation = new PearsonsCorrelation(data);
    }

    // Java: RealMatrix getCorrelationMatrix()
    pub fn getCorrelationMatrix(&self) -> Box<dyn std::any::Any> {
        // Java: return rankCorrelation.getCorrelationMatrix();
        todo!()
    }

    // Java: PearsonsCorrelation getRankCorrelation()
    pub fn getRankCorrelation(&self) -> Box<dyn std::any::Any> {
        // Java: return rankCorrelation;
        todo!()
    }

    // Java: RealMatrix computeCorrelationMatrix(final RealMatrix matrix)
    pub fn computeCorrelationMatrix(&self) -> Box<dyn std::any::Any> {
        // Java: final RealMatrix matrixCopy = rankTransform(matrix);
        // Java: return new PearsonsCorrelation().computeCorrelationMatrix(matrixCopy);
        todo!()
    }

    // Java: RealMatrix computeCorrelationMatrix(final double[][] matrix)
    pub fn computeCorrelationMatrix_2(&self) -> Box<dyn std::any::Any> {
       // Java: return computeCorrelationMatrix(new BlockRealMatrix(matrix));
        todo!()
    }

    // Java: double correlation(final double[] xArray, final double[] yArray)
    pub fn correlation(&self) -> f64 {
        // Java: if (xArray.length != yArray.length)
        {
            // Java: throw new DimensionMismatchException(xArray.length, yArray.length);
        }
        // Java: else if (xArray.length < 2)
        {
            // Java: throw new MathIllegalArgumentException(LocalizedFormats.INSUFFICIENT_DIMENSION,
                                                   // Java: xArray.length, 2);
        }
        // Java: else
        {
            // Java: double[] x = xArray;
            // Java: double[] y = yArray;
            // Java: if (rankingAlgorithm instanceof NaturalRanking &&
                // Java: NaNStrategy.REMOVED == ((NaturalRanking) rankingAlgorithm).getNanStrategy())
                {
                // Java: final Set<Integer> nanPositions = new HashSet<Integer>();
                // Java: nanPositions.addAll(getNaNPositions(xArray));
                // Java: nanPositions.addAll(getNaNPositions(yArray));
                // Java: x = removeValues(xArray, nanPositions);
                // Java: y = removeValues(yArray, nanPositions);
                }
            // Java: return new PearsonsCorrelation().correlation(rankingAlgorithm.rank(x), rankingAlgorithm.rank(y));
        }
        todo!()
    }

    // Java: RealMatrix rankTransform(final RealMatrix matrix)
    pub fn rankTransform(&self) -> Box<dyn std::any::Any> {
        // Java: RealMatrix transformed = null;
        // Java: if (rankingAlgorithm instanceof NaturalRanking &&
                // Java: ((NaturalRanking) rankingAlgorithm).getNanStrategy() == NaNStrategy.REMOVED)
                {
            // Java: final Set<Integer> nanPositions = new HashSet<Integer>();
            // Java: for (int i = 0; i < matrix.getColumnDimension(); i++)
            {
                // Java: nanPositions.addAll(getNaNPositions(matrix.getColumn(i)));
            }
            // Java: // if we have found NaN values, we have to update the matrix size
            // Java: if (!nanPositions.isEmpty())
            {
                // Java: transformed = new BlockRealMatrix(matrix.getRowDimension() - nanPositions.size(),
                                                  // Java: matrix.getColumnDimension());
                // Java: for (int i = 0; i < transformed.getColumnDimension(); i++)
                {
                    // Java: transformed.setColumn(i, removeValues(matrix.getColumn(i), nanPositions));
                }
            }
                }
        // Java: if (transformed == null)
        {
            // Java: transformed = matrix.copy();
        }
        // Java: for (int i = 0; i < transformed.getColumnDimension(); i++)
        {
            // Java: transformed.setColumn(i, rankingAlgorithm.rank(transformed.getColumn(i)));
        }
        // Java: return transformed;
        todo!()
    }

    // Java: List<Integer> getNaNPositions(final double[] input)
    pub fn getNaNPositions(&self) -> Box<dyn std::any::Any> {
        // Java: final List<Integer> positions = new ArrayList<Integer>();
        // Java: for (int i = 0; i < input.length; i++)
        {
            // Java: if (Double.isNaN(input[i]))
            {
                // Java: positions.add(i);
            }
        }
        // Java: return positions;
        todo!()
    }

    // Java: double[] removeValues(final double[] input, final Set<Integer> indices)
    pub fn removeValues(&self) -> Vec<f64> {
        // Java: if (indices.isEmpty())
        {
            // Java: return input;
        }
        // Java: final double[] result = new double[input.length - indices.size()];
        // Java: for (int i = 0, j = 0; i < input.length; i++)
        {
            // Java: if (!indices.contains(i))
            {
                // Java: result[j++] = input[i];
            }
        }
        // Java: return result;
        todo!()
    }

}
