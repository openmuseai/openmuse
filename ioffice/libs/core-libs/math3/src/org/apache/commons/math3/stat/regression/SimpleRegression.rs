// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.distribution.TDistribution
use crate::org::apache::commons::math3::distribution::TDistribution;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java field: long serialVersionUID

// Java: public class SimpleRegression implements Serializable, UpdatingMultipleLinearRegression
pub struct SimpleRegression {
    // Java field: double sumX
    sumX: f64,
    // Java field: double sumXX
    pub(crate) sumXX: f64,
    // Java field: double sumY
    sumY: f64,
    // Java field: double sumYY
    pub(crate) sumYY: f64,
    // Java field: double sumXY
    pub(crate) sumXY: f64,
    // Java field: long n
    pub(crate) n: i64,
    // Java field: double xbar
    xbar: f64,
    // Java field: double ybar
    ybar: f64,
    // Java field: boolean hasIntercept
    pub(crate) hasIntercept: bool,
}

// Java: public class SimpleRegression implements Serializable, UpdatingMultipleLinearRegression
impl SimpleRegression {
    // Java: SimpleRegression()
    pub fn SimpleRegression(&self) {
        // Java: this(true);
    }

    // Java: SimpleRegression(boolean includeIntercept)
    pub fn SimpleRegression_2(&self) {
        // Java: super();
        // Java: hasIntercept = includeIntercept;
    }

    // Java: void addData(final double x, final double y)
    pub fn addData(&self) {
        // Java: if (n == 0)
        {
            // Java: xbar = x;
            // Java: ybar = y;
        }
        // Java: else
        {
            // Java: if( hasIntercept )
            {
                // Java: final double fact1 = 1.0 + n;
                // Java: final double fact2 = n / (1.0 + n);
                // Java: final double dx = x - xbar;
                // Java: final double dy = y - ybar;
                // Java: sumXX += dx * dx * fact2;
                // Java: sumYY += dy * dy * fact2;
                // Java: sumXY += dx * dy * fact2;
                // Java: xbar += dx / fact1;
                // Java: ybar += dy / fact1;
            }
        }
        // Java: if( !hasIntercept )
        {
            // Java: sumXX += x * x ;
            // Java: sumYY += y * y ;
            // Java: sumXY += x * y ;
        }
        // Java: sumX += x;
        // Java: sumY += y;
        // Java: n++;
    }

    // Java: void append(SimpleRegression reg)
    pub fn append(&self) {
        // Java: if (n == 0)
        {
            // Java: xbar = reg.xbar;
            // Java: ybar = reg.ybar;
            // Java: sumXX = reg.sumXX;
            // Java: sumYY = reg.sumYY;
            // Java: sumXY = reg.sumXY;
        }
        // Java: else
        {
            // Java: if (hasIntercept)
            {
                // Java: final double fact1 = reg.n / (double) (reg.n + n);
                // Java: final double fact2 = n * reg.n / (double) (reg.n + n);
                // Java: final double dx = reg.xbar - xbar;
                // Java: final double dy = reg.ybar - ybar;
                // Java: sumXX += reg.sumXX + dx * dx * fact2;
                // Java: sumYY += reg.sumYY + dy * dy * fact2;
                // Java: sumXY += reg.sumXY + dx * dy * fact2;
                // Java: xbar += dx * fact1;
                // Java: ybar += dy * fact1;
            }
            // Java: else
            {
                // Java: sumXX += reg.sumXX;
                // Java: sumYY += reg.sumYY;
                // Java: sumXY += reg.sumXY;
            }
        }
        // Java: sumX += reg.sumX;
        // Java: sumY += reg.sumY;
        // Java: n += reg.n;
    }

    // Java: void removeData(final double x, final double y)
    pub fn removeData(&self) {
        // Java: if (n > 0)
        {
            // Java: if (hasIntercept)
            {
                // Java: final double fact1 = n - 1.0;
                // Java: final double fact2 = n / (n - 1.0);
                // Java: final double dx = x - xbar;
                // Java: final double dy = y - ybar;
                // Java: sumXX -= dx * dx * fact2;
                // Java: sumYY -= dy * dy * fact2;
                // Java: sumXY -= dx * dy * fact2;
                // Java: xbar -= dx / fact1;
                // Java: ybar -= dy / fact1;
            }
            // Java: else
            {
                // Java: final double fact1 = n - 1.0;
                // Java: sumXX -= x * x;
                // Java: sumYY -= y * y;
                // Java: sumXY -= x * y;
                // Java: xbar -= x / fact1;
                // Java: ybar -= y / fact1;
            }
             // Java: sumX -= x;
             // Java: sumY -= y;
             // Java: n--;
        }
    }

    // Java: void addData(final double[][] data)
    pub fn addData_2(&self) {
        // Java: for (int i = 0; i < data.length; i++)
        {
            // Java: if( data[i].length < 2 )
            {
               // Java: throw new ModelSpecificationException(LocalizedFormats.INVALID_REGRESSION_OBSERVATION,
                    // Java: data[i].length, 2);
            }
            // Java: addData(data[i][0], data[i][1]);
        }
    }

    // Java: void addObservation(final double[] x, final double y)
    pub fn addObservation(&self) {
    }

    // Java: void addObservations(final double[][] x, final double[] y)
    pub fn addObservations(&self) {
        // Java: if ((x == null) || (y == null) || (x.length != y.length))
        {
            // Java: throw new ModelSpecificationException(
                  // Java: LocalizedFormats.DIMENSIONS_MISMATCH_SIMPLE,
                  // Java: (x == null) ? 0 : x.length,
                  // Java: (y == null) ? 0 : y.length);
        }
        // Java: boolean obsOk=true;
        // Java: for( int i = 0 ; i < x.length; i++)
        {
            // Java: if( x[i] == null || x[i].length == 0 )
            {
                // Java: obsOk = false;
            }
        }
        // Java: if( !obsOk )
        {
            // Java: throw new ModelSpecificationException(
                  // Java: LocalizedFormats.NOT_ENOUGH_DATA_FOR_NUMBER_OF_PREDICTORS,
                  // Java: 0, 1);
        }
        // Java: for( int i = 0 ; i < x.length ; i++)
        {
            // Java: addData( x[i][0], y[i] );
        }
    }

    // Java: void removeData(double[][] data)
    pub fn removeData_2(&self) {
        // Java: for (int i = 0; i < data.length && n > 0; i++)
        {
            // Java: removeData(data[i][0], data[i][1]);
        }
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: sumX = 0d;
        // Java: sumXX = 0d;
        // Java: sumY = 0d;
        // Java: sumYY = 0d;
        // Java: sumXY = 0d;
        // Java: n = 0;
    }

    // Java: long getN()
    pub fn getN(&self) -> i64 {
        // Java: return n;
        todo!()
    }

    // Java: double predict(final double x)
    pub fn predict(&self) -> f64 {
        // Java: final double b1 = getSlope();
        // Java: if (hasIntercept)
        {
            // Java: return getIntercept(b1) + b1 * x;
        }
        // Java: return b1 * x;
        todo!()
    }

    // Java: double getIntercept()
    pub fn getIntercept(&self) -> f64 {
        // Java: return hasIntercept ? getIntercept(getSlope()) : 0.0;
        todo!()
    }

    // Java: boolean hasIntercept()
    pub fn hasIntercept(&self) -> bool {
        // Java: return hasIntercept;
        todo!()
    }

    // Java: double getSlope()
    pub fn getSlope(&self) -> f64 {
        // Java: if (n < 2)
        {
            // Java: return Double.NaN; //not enough data
        }
        // Java: if (FastMath.abs(sumXX) < 10 * Double.MIN_VALUE)
        {
            // Java: return Double.NaN; //not enough variation in x
        }
        // Java: return sumXY / sumXX;
        todo!()
    }

    // Java: double getSumSquaredErrors()
    pub fn getSumSquaredErrors(&self) -> f64 {
        // Java: return FastMath.max(0d, sumYY - sumXY * sumXY / sumXX);
        todo!()
    }

    // Java: double getTotalSumSquares()
    pub fn getTotalSumSquares(&self) -> f64 {
        // Java: if (n < 2)
        {
            // Java: return Double.NaN;
        }
        // Java: return sumYY;
        todo!()
    }

    // Java: double getXSumSquares()
    pub fn getXSumSquares(&self) -> f64 {
        // Java: if (n < 2)
        {
            // Java: return Double.NaN;
        }
        // Java: return sumXX;
        todo!()
    }

    // Java: double getSumOfCrossProducts()
    pub fn getSumOfCrossProducts(&self) -> f64 {
        // Java: return sumXY;
        todo!()
    }

    // Java: double getRegressionSumSquares()
    pub fn getRegressionSumSquares(&self) -> f64 {
        // Java: return getRegressionSumSquares(getSlope());
        todo!()
    }

    // Java: double getMeanSquareError()
    pub fn getMeanSquareError(&self) -> f64 {
        // Java: if (n < 3)
        {
            // Java: return Double.NaN;
        }
        // Java: return hasIntercept ? (getSumSquaredErrors() / (n - 2)) : (getSumSquaredErrors() / (n - 1));
        todo!()
    }

    // Java: double getR()
    pub fn getR(&self) -> f64 {
        // Java: double b1 = getSlope();
        // Java: double result = FastMath.sqrt(getRSquare());
        // Java: if (b1 < 0)
        {
            // Java: result = -result;
        }
        // Java: return result;
        todo!()
    }

    // Java: double getRSquare()
    pub fn getRSquare(&self) -> f64 {
        // Java: double ssto = getTotalSumSquares();
        // Java: return (ssto - getSumSquaredErrors()) / ssto;
        todo!()
    }

    // Java: double getInterceptStdErr()
    pub fn getInterceptStdErr(&self) -> f64 {
        // Java: if( !hasIntercept )
        {
            // Java: return Double.NaN;
        }
        // Java: return FastMath.sqrt(
            // Java: getMeanSquareError() * ((1d / n) + (xbar * xbar) / sumXX));
        todo!()
    }

    // Java: double getSlopeStdErr()
    pub fn getSlopeStdErr(&self) -> f64 {
        // Java: return FastMath.sqrt(getMeanSquareError() / sumXX);
        todo!()
    }

    // Java: double getSlopeConfidenceInterval()
    pub fn getSlopeConfidenceInterval(&self) -> f64 {
        // Java: return getSlopeConfidenceInterval(0.05d);
        todo!()
    }

    // Java: double getSlopeConfidenceInterval(final double alpha)
    pub fn getSlopeConfidenceInterval_2(&self) -> f64 {
        todo!()
    }

    // Java: double getSignificance()
    pub fn getSignificance(&self) -> f64 {
        // Java: if (n < 3)
        {
            // Java: return Double.NaN;
        }
        // Java: // No advertised NotStrictlyPositiveException here - will return NaN above
        // Java: TDistribution distribution = new TDistribution(n - 2);
        // Java: return 2d * (1.0 - distribution.cumulativeProbability(
                    // Java: FastMath.abs(getSlope()) / getSlopeStdErr()));
        todo!()
    }

    // Java: double getIntercept(final double slope)
    pub fn getIntercept_2(&self) -> f64 {
      // Java: if( hasIntercept)
      {
        // Java: return (sumY - slope * sumX) / n;
      }
      // Java: return 0.0;
        todo!()
    }

    // Java: double getRegressionSumSquares(final double slope)
    pub fn getRegressionSumSquares_2(&self) -> f64 {
        // Java: return slope * slope * sumXX;
        todo!()
    }

    // Java: RegressionResults regress()
    pub fn regress(&self) -> Box<dyn std::any::Any> {
        // Java: if (hasIntercept)
        {
            // Java: if (n < 3)
            {
                // Java: throw new NoDataException(LocalizedFormats.NOT_ENOUGH_DATA_REGRESSION);
            }
            // Java: if (FastMath.abs(sumXX) > Precision.SAFE_MIN)
            {
                // Java: final double[] params = new double[] { getIntercept(), getSlope() };
                // Java: final double mse = getMeanSquareError();
                // Java: final double _syy = sumYY + sumY * sumY / n;
                // Java: final double[] vcv = new double[] { mse * (xbar * xbar / sumXX + 1.0 / n), -xbar * mse / sumXX, mse / sumXX };
                // Java: return new RegressionResults(params, new double[][] { vcv }, true, n, 2, sumY, _syy, getSumSquaredErrors(), true,
                        // Java: false);
            }
            // Java: else
            {
                // Java: final double[] params = new double[] { sumY / n, Double.NaN };
                // Java: // final double mse = getMeanSquareError();
                // Java: final double[] vcv = new double[] { ybar / (n - 1.0), Double.NaN, Double.NaN };
                // Java: return new RegressionResults(params, new double[][] { vcv }, true, n, 1, sumY, sumYY, getSumSquaredErrors(), true,
                        // Java: false);
            }
        }
        // Java: else
        {
            // Java: if (n < 2)
            {
                // Java: throw new NoDataException(LocalizedFormats.NOT_ENOUGH_DATA_REGRESSION);
            }
            // Java: if (!Double.isNaN(sumXX))
            {
                // Java: final double[] vcv = new double[] { getMeanSquareError() / sumXX };
                // Java: final double[] params = new double[] { sumXY / sumXX };
                // Java: return new RegressionResults(params, new double[][] { vcv }, true, n, 1, sumY, sumYY, getSumSquaredErrors(), false,
                        // Java: false);
            }
            // Java: else
            {
                // Java: final double[] vcv = new double[] { Double.NaN };
                // Java: final double[] params = new double[] { Double.NaN };
                // Java: return new RegressionResults(params, new double[][] { vcv }, true, n, 1, Double.NaN, Double.NaN, Double.NaN, false,
                        // Java: false);
            }
        }
        todo!()
    }

    // Java: RegressionResults regress(int[] variablesToInclude)
    pub fn regress_2(&self) -> Box<dyn std::any::Any> {
        // Java: if( variablesToInclude == null || variablesToInclude.length == 0)
        {
          // Java: throw new MathIllegalArgumentException(LocalizedFormats.ARRAY_ZERO_LENGTH_OR_NULL_NOT_ALLOWED);
        }
        // Java: if( variablesToInclude.length > 2 || (variablesToInclude.length > 1 && !hasIntercept) )
        {
            // Java: throw new ModelSpecificationException(
                    // Java: LocalizedFormats.ARRAY_SIZE_EXCEEDS_MAX_VARIABLES,
                    // Java: (variablesToInclude.length > 1 && !hasIntercept) ? 1 : 2);
        }
        // Java: if( hasIntercept )
        {
            // Java: if( variablesToInclude.length == 2 )
            {
                // Java: if( variablesToInclude[0] == 1 )
                {
                    // Java: throw new ModelSpecificationException(LocalizedFormats.NOT_INCREASING_SEQUENCE);
                }
                // Java: else if( variablesToInclude[0] != 0 )
                {
                    // Java: throw new OutOfRangeException( variablesToInclude[0], 0,1 );
                }
                // Java: if( variablesToInclude[1] != 1)
                {
                     // Java: throw new OutOfRangeException( variablesToInclude[0], 0,1 );
                }
                // Java: return regress();
            }
            // Java: else
            {
                // Java: if( variablesToInclude[0] != 1 && variablesToInclude[0] != 0 )
                {
                     // Java: throw new OutOfRangeException( variablesToInclude[0],0,1 );
                }
                // Java: final double _mean = sumY * sumY / n;
                // Java: final double _syy = sumYY + _mean;
                // Java: if( variablesToInclude[0] == 0 )
                {
                    // Java: //just the mean
                    // Java: final double[] vcv = new double[]{ sumYY/(((n-1)*n)) };
                    // Java: final double[] params = new double[]{ ybar };
                    // Java: return new RegressionResults(
                      // Java: params, new double[][]{vcv}, true, n, 1,
                      // Java: sumY, _syy+_mean, sumYY,true,false);
                }
                // Java: else if( variablesToInclude[0] == 1)
                {
                    // Java: //final double _syy = sumYY + sumY * sumY / ((double) n);
                    // Java: final double _sxx = sumXX + sumX * sumX / n;
                    // Java: final double _sxy = sumXY + sumX * sumY / n;
                    // Java: final double _sse = FastMath.max(0d, _syy - _sxy * _sxy / _sxx);
                    // Java: final double _mse = _sse/((n-1));
                    // Java: if( !Double.isNaN(_sxx) )
                    {
                        // Java: final double[] vcv = new double[]{ _mse / _sxx };
                        // Java: final double[] params = new double[]{ _sxy/_sxx };
                        // Java: return new RegressionResults(
                                    // Java: params, new double[][]{vcv}, true, n, 1,
                                    // Java: sumY, _syy, _sse,false,false);
                    }
                    // Java: else
                    {
                        // Java: final double[] vcv = new double[]{Double.NaN };
                        // Java: final double[] params = new double[]{ Double.NaN };
                        // Java: return new RegressionResults(
                                    // Java: params, new double[][]{vcv}, true, n, 1,
                                    // Java: Double.NaN, Double.NaN, Double.NaN,false,false);
                    }
                }
            }
        }
        // Java: else
        {
            // Java: if( variablesToInclude[0] != 0 )
            {
                // Java: throw new OutOfRangeException(variablesToInclude[0],0,0);
            }
            // Java: return regress();
        }
        // Java: return null;
        todo!()
    }

}


