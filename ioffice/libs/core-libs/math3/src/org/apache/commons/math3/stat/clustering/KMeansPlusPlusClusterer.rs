// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Collections
// Java: import java.util.List
// Java: import java.util.Random
// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Variance
use crate::org::apache::commons::math3::stat::descriptive::moment::Variance;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java: public class KMeansPlusPlusClusterer extends Clusterable
pub struct KMeansPlusPlusClusterer {
    // Java field: Random random
    random: Box<dyn std::any::Any>,
    // Java field: EmptyClusterStrategy emptyStrategy
    emptyStrategy: Box<dyn std::any::Any>,
}

// Java: public enum EmptyClusterStrategy
pub struct EmptyClusterStrategy {
}

// Java: public class KMeansPlusPlusClusterer extends Clusterable
impl KMeansPlusPlusClusterer {
    // Java: KMeansPlusPlusClusterer(final Random random)
    pub fn KMeansPlusPlusClusterer(&self) {
        // Java: this(random, EmptyClusterStrategy.LARGEST_VARIANCE);
    }

    // Java: KMeansPlusPlusClusterer(final Random random, final EmptyClusterStrategy emptyStrategy)
    pub fn KMeansPlusPlusClusterer_2(&self) {
        // Java: this.random        = random;
        // Java: this.emptyStrategy = emptyStrategy;
    }

    // Java: List<Cluster<T>> cluster()
    pub fn cluster(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: List<Cluster<T>> cluster()
    pub fn cluster_2(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T getPointFromLargestVarianceCluster(final Collection<Cluster<T>> clusters)
    pub fn getPointFromLargestVarianceCluster(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: T getPointFromLargestNumberCluster(final Collection<Cluster<T>> clusters)
    pub fn getPointFromLargestNumberCluster(&self) -> Box<dyn std::any::Any> {
        // Java: int maxNumber = 0;
        // Java: Cluster<T> selected = null;
        // Java: for (final Cluster<T> cluster : clusters)
        {
            // Java: // get the number of points of the current cluster
            // Java: final int number = cluster.getPoints().size();
            // Java: // select the cluster with the largest number of points
            // Java: if (number > maxNumber)
            {
                // Java: maxNumber = number;
                // Java: selected = cluster;
            }
        }
        // Java: // did we find at least one non-empty cluster ?
        // Java: if (selected == null)
        {
            // Java: throw new ConvergenceException(LocalizedFormats.EMPTY_CLUSTER_IN_K_MEANS);
        }
        // Java: // extract a random point from the cluster
        // Java: final List<T> selectedPoints = selected.getPoints();
        // Java: return selectedPoints.remove(random.nextInt(selectedPoints.size()));
        todo!()
    }

    // Java: T getFarthestPoint(final Collection<Cluster<T>> clusters)
    pub fn getFarthestPoint(&self) -> Box<dyn std::any::Any> {
        // Java: double maxDistance = Double.NEGATIVE_INFINITY;
        // Java: Cluster<T> selectedCluster = null;
        // Java: int selectedPoint = -1;
        // Java: for (final Cluster<T> cluster : clusters)
        {
            // Java: // get the farthest point
            // Java: final T center = cluster.getCenter();
            // Java: final List<T> points = cluster.getPoints();
            // Java: for (int i = 0; i < points.size(); ++i)
            {
                // Java: final double distance = points.get(i).distanceFrom(center);
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

}

// Java: public enum EmptyClusterStrategy
impl EmptyClusterStrategy {
}
