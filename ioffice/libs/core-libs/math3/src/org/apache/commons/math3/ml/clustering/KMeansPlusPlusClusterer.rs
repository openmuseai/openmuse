// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Collections
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.ml.distance.DistanceMeasure
use crate::org::apache::commons::math3::ml::distance::DistanceMeasure;
// Java: import org.apache.commons.math3.ml.distance.EuclideanDistance
use crate::org::apache::commons::math3::ml::distance::EuclideanDistance;
// Java: import org.apache.commons.math3.random.JDKRandomGenerator
use crate::org::apache::commons::math3::random::JDKRandomGenerator;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Variance
use crate::org::apache::commons::math3::stat::descriptive::moment::Variance;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java: public class KMeansPlusPlusClusterer extends Clusterable
pub struct KMeansPlusPlusClusterer {
    // Java field: int k
    pub(crate) k: i32,
    // Java field: int maxIterations
    pub(crate) maxIterations: i32,
    // Java field: RandomGenerator random
    pub(crate) random: Box<dyn std::any::Any>,
    // Java field: EmptyClusterStrategy emptyStrategy
    pub(crate) emptyStrategy: Box<dyn std::any::Any>,
}

// Java: public enum EmptyClusterStrategy
pub struct EmptyClusterStrategy {
}

// Java: public class KMeansPlusPlusClusterer extends Clusterable
impl KMeansPlusPlusClusterer {
    // Java: KMeansPlusPlusClusterer(final int k)
    pub fn KMeansPlusPlusClusterer(&self) {
        // Java: this(k, -1);
    }

    // Java: KMeansPlusPlusClusterer(final int k, final int maxIterations)
    pub fn KMeansPlusPlusClusterer_2(&self) {
        // Java: this(k, maxIterations, new EuclideanDistance());
    }

    // Java: KMeansPlusPlusClusterer(final int k, final int maxIterations, final DistanceMeasure measure)
    pub fn KMeansPlusPlusClusterer_3(&self) {
        // Java: this(k, maxIterations, measure, new JDKRandomGenerator());
    }

    // Java: KMeansPlusPlusClusterer()
    pub fn KMeansPlusPlusClusterer_4(&self) {
    }

    // Java: KMeansPlusPlusClusterer()
    pub fn KMeansPlusPlusClusterer_5(&self) {
    }

    // Java: int getK()
    pub fn getK(&self) -> i32 {
        // Java: return k;
        todo!()
    }

    // Java: int getMaxIterations()
    pub fn getMaxIterations(&self) -> i32 {
        // Java: return maxIterations;
        todo!()
    }

    // Java: RandomGenerator getRandomGenerator()
    pub fn getRandomGenerator(&self) -> Box<dyn std::any::Any> {
        // Java: return random;
        todo!()
    }

    // Java: EmptyClusterStrategy getEmptyClusterStrategy()
    pub fn getEmptyClusterStrategy(&self) -> Box<dyn std::any::Any> {
        // Java: return emptyStrategy;
        todo!()
    }

    // Java: List<CentroidCluster<T>> cluster(final Collection<T> points)
    pub fn cluster(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: int assignPointsToClusters()
    pub fn assignPointsToClusters(&self) -> i32 {
        todo!()
    }

    // Java: List<CentroidCluster<T>> chooseInitialCenters(final Collection<T> points)
    pub fn chooseInitialCenters(&self) -> Box<dyn std::any::Any> {
        // Java: // Convert to list for indexed access. Make it unmodifiable, since removal of items
        // Java: // would screw up the logic of this method.
        // Java: final List<T> pointList = Collections.unmodifiableList(new ArrayList<T> (points));
        // Java: // The number of points in the list.
        // Java: final int numPoints = pointList.size();
        // Java: // Set the corresponding element in this array to indicate when
        // Java: // elements of pointList are no longer available.
        // Java: final boolean[] taken = new boolean[numPoints];
        // Java: // The resulting list of initial centers.
        // Java: final List<CentroidCluster<T>> resultSet = new ArrayList<CentroidCluster<T>>();
        // Java: // Choose one center uniformly at random from among the data points.
        // Java: final int firstPointIndex = random.nextInt(numPoints);
        // Java: final T firstPoint = pointList.get(firstPointIndex);
        // Java: resultSet.add(new CentroidCluster<T>(firstPoint));
        // Java: // Must mark it as taken
        // Java: taken[firstPointIndex] = true;
        // Java: // To keep track of the minimum distance squared of elements of
        // Java: // pointList to elements of resultSet.
        // Java: final double[] minDistSquared = new double[numPoints];
        // Java: // Initialize the elements.  Since the only point in resultSet is firstPoint,
        // Java: // this is very easy.
        // Java: for (int i = 0; i < numPoints; i++)
        {
            // Java: if (i != firstPointIndex) { // That point isn't considered
                // Java: double d = distance(firstPoint, pointList.get(i));
                // Java: minDistSquared[i] = d*d;
        }
        // Java: while (resultSet.size() < k)
        {
            // Java: // Sum up the squared distances for the points in pointList not
            // Java: // already taken.
            // Java: double distSqSum = 0.0;
            // Java: for (int i = 0; i < numPoints; i++)
            {
                // Java: if (!taken[i])
                {
                    // Java: distSqSum += minDistSquared[i];
                }
            }
            // Java: // Add one new data point as a center. Each point x is chosen with
            // Java: // probability proportional to D(x)2
            // Java: final double r = random.nextDouble() * distSqSum;
            // Java: // The index of the next point to be added to the resultSet.
            // Java: int nextPointIndex = -1;
            // Java: // Sum through the squared min distances again, stopping when
            // Java: // sum >= r.
            // Java: double sum = 0.0;
            // Java: for (int i = 0; i < numPoints; i++)
            {
                // Java: if (!taken[i])
                {
                    // Java: sum += minDistSquared[i];
                    // Java: if (sum >= r)
                    {
                        // Java: nextPointIndex = i;
                        // Java: break;
                    }
                }
            }
            // Java: // If it's not set to >= 0, the point wasn't found in the previous
            // Java: // for loop, probably because distances are extremely small.  Just pick
            // Java: // the last available point.
            // Java: if (nextPointIndex == -1)
            {
                // Java: for (int i = numPoints - 1; i >= 0; i--)
                {
                    // Java: if (!taken[i])
                    {
                        // Java: nextPointIndex = i;
                        // Java: break;
                    }
                }
            }
            // Java: // We found one.
            // Java: if (nextPointIndex >= 0)
            {
                // Java: final T p = pointList.get(nextPointIndex);
                // Java: resultSet.add(new CentroidCluster<T> (p));
                // Java: // Mark it as taken.
                // Java: taken[nextPointIndex] = true;
                // Java: if (resultSet.size() < k)
                {
                    // Java: // Now update elements of minDistSquared.  We only have to compute
                    // Java: // the distance to the new center to do this.
                    // Java: for (int j = 0; j < numPoints; j++)
                    {
                        // Java: // Only have to worry about the points still not taken.
                        // Java: if (!taken[j])
                        {
                            // Java: double d = distance(p, pointList.get(j));
                            // Java: double d2 = d * d;
                            // Java: if (d2 < minDistSquared[j])
                            {
                                // Java: minDistSquared[j] = d2;
                            }
                        }
                    }
                }
            }
            // Java: else
            {
                // Java: // None found --
                // Java: // Break from the while loop to prevent
                // Java: // an infinite loop.
                // Java: break;
            }
        }
        // Java: return resultSet;
        todo!()
    }

    // Java: T getPointFromLargestVarianceCluster(final Collection<CentroidCluster<T>> clusters)
    pub fn getPointFromLargestVarianceCluster(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T getPointFromLargestNumberCluster(final Collection<? extends Cluster<T>> clusters)
    pub fn getPointFromLargestNumberCluster(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T getFarthestPoint(final Collection<CentroidCluster<T>> clusters)
    pub fn getFarthestPoint(&self) -> Box<dyn std::any::Any> {
        // Java: double maxDistance = Double.NEGATIVE_INFINITY;
        // Java: Cluster<T> selectedCluster = null;
        // Java: int selectedPoint = -1;
        // Java: for (final CentroidCluster<T> cluster : clusters)
        {
            // Java: // get the farthest point
            // Java: final Clusterable center = cluster.getCenter();
            // Java: final List<T> points = cluster.getPoints();
            // Java: for (int i = 0; i < points.size(); ++i)
            {
                // Java: final double distance = distance(points.get(i), center);
                // Java: if (distance > maxDistance)
                {
                    // Java: maxDistance     = distance;
                    // Java: selectedCluster = cluster;
                    // Java: selectedPoint   = i;
                }
            }
        }
        // Java: // did we find at least one non-empty cluster ?
        // Java: if (selectedCluster == null)
        {
            // Java: throw new ConvergenceException(LocalizedFormats.EMPTY_CLUSTER_IN_K_MEANS);
        }
        // Java: return selectedCluster.getPoints().remove(selectedPoint);
        todo!()
    }

    // Java: int getNearestCluster(final Collection<CentroidCluster<T>> clusters, final T point)
    pub fn getNearestCluster(&self) -> i32 {
        // Java: double minDistance = Double.MAX_VALUE;
        // Java: int clusterIndex = 0;
        // Java: int minCluster = 0;
        // Java: for (final CentroidCluster<T> c : clusters)
        {
            // Java: final double distance = distance(point, c.getCenter());
            // Java: if (distance < minDistance)
            {
                // Java: minDistance = distance;
                // Java: minCluster = clusterIndex;
            }
            // Java: clusterIndex++;
        }
        // Java: return minCluster;
        todo!()
    }

    // Java: Clusterable centroidOf(final Collection<T> points, final int dimension)
    pub fn centroidOf(&self) -> Box<dyn std::any::Any> {
        // Java: final double[] centroid = new double[dimension];
        // Java: for (final T p : points)
        {
            // Java: final double[] point = p.getPoint();
            // Java: for (int i = 0; i < centroid.length; i++)
            {
                // Java: centroid[i] += point[i];
            }
        }
        // Java: for (int i = 0; i < centroid.length; i++)
        {
            // Java: centroid[i] /= points.size();
        }
        // Java: return new DoublePoint(centroid);
        todo!()
    }

}

// Java: public enum EmptyClusterStrategy
impl EmptyClusterStrategy {
}