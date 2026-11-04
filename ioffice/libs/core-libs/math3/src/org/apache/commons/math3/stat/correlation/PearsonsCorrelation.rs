// Java: import org.apache.commons.math3.distribution.TDistribution
use crate::org::apache::commons::math3::distribution::TDistribution;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.linear.BlockRealMatrix
use crate::org::apache::commons::math3::linear::BlockRealMatrix;
// Java: import org.apache.commons.math3.stat.regression.SimpleRegression
use crate::org::apache::commons::math3::stat::regression::SimpleRegression;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class PearsonsCorrelation
pub struct PearsonsCorrelation {
    // Java field: RealMatrix correlationMatrix
    pub(crate) correlationMatrix: Box<dyn std::any::Any>,
    // Java field: int nObs
    nObs: i32,
}

// Java: public class PearsonsCorrelation
impl PearsonsCorrelation {
    // Java: PearsonsCorrelation()
    pub fn PearsonsCorrelation(&self) {
        // Java: super();
        // Java: correlationMatrix = null;
        // Java: nObs = 0;
    }

    // Java: PearsonsCorrelation(double[][] data)
    pub fn PearsonsCorrelation_2(&self) {
        // Java: this(new BlockRealMatrix(data));
    }

    // Java: PearsonsCorrelation(RealMatrix matrix)
    pub fn PearsonsCorrelation_3(&self) {
        // Java: nObs = matrix.getRowDimension();
        // Java: correlationMatrix = computeCorrelationMatrix(matrix);
    }

    // Java: PearsonsCorrelation(Covariance covariance)
    pub fn PearsonsCorrelation_4(&self) {
        // Java: RealMatrix covarianceMatrix = covariance.getCovarianceMatrix();
        // Java: if (covarianceMatrix == null)
        {
            // Java: throw new NullArgumentException(LocalizedFormats.COVARIANCE_MATRIX);
        }
        // Java: nObs = covariance.getN();
        // Java: correlationMatrix = covarianceToCorrelation(covarianceMatrix);
    }

    // Java: PearsonsCorrelation(RealMatrix covarianceMatrix, int numberOfObservations)
    pub fn PearsonsCorrelation_5(&self) {
        // Java: nObs = numberOfObservations;
        // Java: correlationMatrix = covarianceToCorrelation(covarianceMatrix);
    }

    // Java: RealMatrix getCorrelationMatrix()
    pub fn getCorrelationMatrix(&self) -> Box<dyn std::any::Any> {
        // Java: return correlationMatrix;
        todo!()
    }

    // Java: RealMatrix getCorrelationStandardErrors()
    pub fn getCorrelationStandardErrors(&self) -> Box<dyn std::any::Any> {
        // Java: int nVars = correlationMatrix.getColumnDimension();
        // Java: double[][] out = new double[nVars][nVars];
        // Java: for (int i = 0; i < nVars; i++)
        {
            // Java: for (int j = 0; j < nVars; j++)
            {
                // Java: double r = correlationMatrix.getEntry(i, j);
                // Java: out[i][j] = FastMath.sqrt((1 - r * r) /(nObs - 2));
            }
        }
        // Java: return new BlockRealMatrix(out);
        todo!()
    }

    // Java: RealMatrix getCorrelationPValues()
    pub fn getCorrelationPValues(&self) -> Box<dyn std::any::Any> {
        // Java: TDistribution tDistribution = new TDistribution(nObs - 2);
        // Java: int nVars = correlationMatrix.getColumnDimension();
        // Java: double[][] out = new double[nVars][nVars];
        // Java: for (int i = 0; i < nVars; i++)
        {
            // Java: for (int j = 0; j < nVars; j++)
            {
                // Java: if (i == j)
                {
                    // Java: out[i][j] = 0d;
                }
                // Java: else
                {
                    // Java: double r = correlationMatrix.getEntry(i, j);
                    // Java: double t = FastMath.abs(r * FastMath.sqrt((nObs - 2)/(1 - r * r)));
                    // Java: out[i][j] = 2 * tDistribution.cumulativeProbability(-t);
                }
            }
        }
        // Java: return new BlockRealMatrix(out);
        todo!()
    }

    // Java: RealMatrix computeCorrelationMatrix(RealMatrix matrix)
    pub fn computeCorrelationMatrix(&self) -> Box<dyn std::any::Any> {
        // Java: checkSufficientData(matrix);
        // Java: int nVars = matrix.getColumnDimension();
        // Java: RealMatrix outMatrix = new BlockRealMatrix(nVars, nVars);
        // Java: for (int i = 0; i < nVars; i++)
        {
            // Java: for (int j = 0; j < i; j++)
            {
              // Java: double corr = correlation(matrix.getColumn(i), matrix.getColumn(j));
              // Java: outMatrix.setEntry(i, j, corr);
              // Java: outMatrix.setEntry(j, i, corr);
            }
            // Java: outMatrix.setEntry(i, i, 1d);
        }
        // Java: return outMatrix;
        todo!()
    }

    // Java: RealMatrix computeCorrelationMatrix(double[][] data)
    pub fn computeCorrelationMatrix_2(&self) -> Box<dyn std::any::Any> {
       // Java: return computeCorrelationMatrix(new BlockRealMatrix(data));
        todo!()
    }

    // Java: double correlation(final double[] xArray, final double[] yArray)
    pub fn correlation(&self) -> f64 {
        // Java: SimpleRegression regression = new SimpleRegression();
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
            // Java: for(int i=0; i<xArray.length; i++)
            {
                // Java: regression.addData(xArray[i], yArray[i]);
            }
            // Java: return regression.getR();
        }
        todo!()
    }

    // Java: RealMatrix covarianceToCorrelation(RealMatrix covarianceMatrix)
    pub fn covarianceToCorrelation(&self) -> Box<dyn std::any::Any> {
        // Java: int nVars = covarianceMatrix.getColumnDimension();
        // Java: RealMatrix outMatrix = new BlockRealMatrix(nVars, nVars);
        // Java: for (int i = 0; i < nVars; i++)
        {
            // Java: double sigma = FastMath.sqrt(covarianceMatrix.getEntry(i, i));
            // Java: outMatrix.setEntry(i, i, 1d);
            // Java: for (int j = 0; j < i; j++)
            {
                // Java: double entry = covarianceMatrix.getEntry(i, j) /
                       // Java: (sigma * FastMath.sqrt(covarianceMatrix.getEntry(j, j)));
                // Java: outMatrix.setEntry(i, j, entry);
                // Java: outMatrix.setEntry(j, i, entry);
            }
        }
        // Java: return outMatrix;
        todo!()
    }

    // Java: void checkSufficientData(final RealMatrix matrix)
    pub fn checkSufficientData(&self) {
        // Java: int nRows = matrix.getRowDimension();
        // Java: int nCols = matrix.getColumnDimension();
        // Java: if (nRows < 2 || nCols < 2)
        {
            // Java: throw new MathIllegalArgumentException(LocalizedFormats.INSUFFICIENT_ROWS_AND_COLUMNS,
                                                   // Java: nRows, nCols);
        }
    }

}
