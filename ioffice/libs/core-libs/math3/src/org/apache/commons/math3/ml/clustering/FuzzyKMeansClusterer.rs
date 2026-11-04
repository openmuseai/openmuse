// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Collections
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.linear.MatrixUtils
use crate::org::apache::commons::math3::linear::MatrixUtils;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;
// Java: import org.apache.commons.math3.ml.distance.DistanceMeasure
use crate::org::apache::commons::math3::ml::distance::DistanceMeasure;
// Java: import org.apache.commons.math3.ml.distance.EuclideanDistance
use crate::org::apache::commons::math3::ml::distance::EuclideanDistance;
// Java: import org.apache.commons.math3.random.JDKRandomGenerator
use crate::org::apache::commons::math3::random::JDKRandomGenerator;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: double DEFAULT_EPSILON
pub(crate) const DEFAULT_EPSILON: f64 = 0.0;

// Java: public class FuzzyKMeansClusterer extends Clusterable
pub struct FuzzyKMeansClusterer {
    // Java field: int k
    pub(crate) k: i32,
    // Java field: int maxIterations
    pub(crate) maxIterations: i32,
    // Java field: double fuzziness
    pub(crate) fuzziness: f64,
    // Java field: double epsilon
    pub(crate) epsilon: f64,
    // Java field: RandomGenerator random
    pub(crate) random: Box<dyn std::any::Any>,
    // Java field: double[][] membershipMatrix
    membershipMatrix: Option<Vec<Vec<f64>>>,
    // Java field: List<T> points
    pub(crate) points: Box<dyn std::any::Any>,
    // Java field: List<CentroidCluster<T>> clusters
    pub(crate) clusters: Box<dyn std::any::Any>,
}

// Java: public class FuzzyKMeansClusterer extends Clusterable
impl FuzzyKMeansClusterer {
    // Java: FuzzyKMeansClusterer(final int k, final double fuzziness)
    pub fn FuzzyKMeansClusterer(&self) {
        // Java: this(k, fuzziness, -1, new EuclideanDistance());
    }

    // Java: FuzzyKMeansClusterer()
    pub fn FuzzyKMeansClusterer_2(&self) {
    }

    // Java: FuzzyKMeansClusterer()
    pub fn FuzzyKMeansClusterer_3(&self) {
    }

    // Java: int getK()
    pub fn getK(&self) -> i32 {
        // Java: return k;
        todo!()
    }

    // Java: double getFuzziness()
    pub fn getFuzziness(&self) -> f64 {
        // Java: return fuzziness;
        todo!()
    }

    // Java: int getMaxIterations()
    pub fn getMaxIterations(&self) -> i32 {
        // Java: return maxIterations;
        todo!()
    }

    // Java: double getEpsilon()
    pub fn getEpsilon(&self) -> f64 {
        // Java: return epsilon;
        todo!()
    }

    // Java: RandomGenerator getRandomGenerator()
    pub fn getRandomGenerator(&self) -> Box<dyn std::any::Any> {
        // Java: return random;
        todo!()
    }

    // Java: RealMatrix getMembershipMatrix()
    pub fn getMembershipMatrix(&self) -> Box<dyn std::any::Any> {
        // Java: if (membershipMatrix == null)
        {
            // Java: throw new MathIllegalStateException();
        }
        // Java: return MatrixUtils.createRealMatrix(membershipMatrix);
        todo!()
    }

    // Java: List<T> getDataPoints()
    pub fn getDataPoints(&self) -> Box<dyn std::any::Any> {
        // Java: return points;
        todo!()
    }

    // Java: List<CentroidCluster<T>> getClusters()
    pub fn getClusters(&self) -> Box<dyn std::any::Any> {
        // Java: return clusters;
        todo!()
    }

    // Java: double getObjectiveFunctionValue()
    pub fn getObjectiveFunctionValue(&self) -> f64 {
        // Java: if (points == null || clusters == null)
        {
            // Java: throw new MathIllegalStateException();
        }
        // Java: int i = 0;
        // Java: double objFunction = 0.0;
        // Java: for (final T point : points)
        {
            // Java: int j = 0;
            // Java: for (final CentroidCluster<T> cluster : clusters)
            {
                // Java: final double dist = distance(point, cluster.getCenter());
                // Java: objFunction += (dist * dist) * FastMath.pow(membershipMatrix[i][j], fuzziness);
                // Java: j++;
            }
            // Java: i++;
        }
        // Java: return objFunction;
        todo!()
    }

    // Java: List<CentroidCluster<T>> cluster(final Collection<T> dataPoints)
    pub fn cluster(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void updateClusterCenters()
    pub fn updateClusterCenters(&self) {
        // Java: int j = 0;
        // Java: final List<CentroidCluster<T>> newClusters = new ArrayList<CentroidCluster<T>>(k);
        // Java: for (final CentroidCluster<T> cluster : clusters)
        {
            // Java: final Clusterable center = cluster.getCenter();
            // Java: int i = 0;
            // Java: double[] arr = new double[center.getPoint().length];
            // Java: double sum = 0.0;
            // Java: for (final T point : points)
            {
                // Java: final double u = FastMath.pow(membershipMatrix[i][j], fuzziness);
                // Java: final double[] pointArr = point.getPoint();
                // Java: for (int idx = 0; idx < arr.length; idx++)
                {
                    // Java: arr[idx] += u * pointArr[idx];
                }
                // Java: sum += u;
                // Java: i++;
            }
            // Java: MathArrays.scaleInPlace(1.0 / sum, arr);
            // Java: newClusters.add(new CentroidCluster<T>(new DoublePoint(arr)));
            // Java: j++;
        }
        // Java: clusters.clear();
        // Java: clusters = newClusters;
    }

    // Java: void updateMembershipMatrix()
    pub fn updateMembershipMatrix(&self) {
        // Java: for (int i = 0; i < points.size(); i++)
        {
            // Java: final T point = points.get(i);
            // Java: double maxMembership = Double.MIN_VALUE;
            // Java: int newCluster = -1;
            // Java: for (int j = 0; j < clusters.size(); j++)
            {
                // Java: double sum = 0.0;
                // Java: final double distA = FastMath.abs(distance(point, clusters.get(j).getCenter()));
                // Java: if (distA != 0.0)
                {
                    // Java: for (final CentroidCluster<T> c : clusters)
                    {
                        // Java: final double distB = FastMath.abs(distance(point, c.getCenter()));
                        // Java: if (distB == 0.0)
                        {
                            // Java: sum = Double.POSITIVE_INFINITY;
                            // Java: break;
                        }
                        // Java: sum += FastMath.pow(distA / distB, 2.0 / (fuzziness - 1.0));
                    }
                }
                // Java: double membership;
                // Java: if (sum == 0.0)
                {
                    // Java: membership = 1.0;
                }
                // Java: else if (sum == Double.POSITIVE_INFINITY)
                {
                    // Java: membership = 0.0;
                }
                // Java: else
                {
                    // Java: membership = 1.0 / sum;
                }
                // Java: membershipMatrix[i][j] = membership;
                // Java: if (membershipMatrix[i][j] > maxMembership)
                {
                    // Java: maxMembership = membershipMatrix[i][j];
                    // Java: newCluster = j;
                }
            }
            // Java: clusters.get(newCluster).addPoint(point);
        }
    }

    // Java: void initializeMembershipMatrix()
    pub fn initializeMembershipMatrix(&self) {
        // Java: for (int i = 0; i < points.size(); i++)
        {
            // Java: for (int j = 0; j < k; j++)
            {
                // Java: membershipMatrix[i][j] = random.nextDouble();
            }
            // Java: membershipMatrix[i] = MathArrays.normalizeArray(membershipMatrix[i], 1.0);
        }
    }

    // Java: double calculateMaxMembershipChange(final double[][] matrix)
    pub fn calculateMaxMembershipChange(&self) -> f64 {
        // Java: double maxMembership = 0.0;
        // Java: for (int i = 0; i < points.size(); i++)
        {
            // Java: for (int j = 0; j < clusters.size(); j++)
            {
                // Java: double v = FastMath.abs(membershipMatrix[i][j] - matrix[i][j]);
                // Java: maxMembership = FastMath.max(v, maxMembership);
            }
        }
        // Java: return maxMembership;
        todo!()
    }

    // Java: void saveMembershipMatrix(final double[][] matrix)
    pub fn saveMembershipMatrix(&self) {
        // Java: for (int i = 0; i < points.size(); i++)
        {
            // Java: System.arraycopy(membershipMatrix[i], 0, matrix[i], 0, clusters.size());
        }
    }

}
