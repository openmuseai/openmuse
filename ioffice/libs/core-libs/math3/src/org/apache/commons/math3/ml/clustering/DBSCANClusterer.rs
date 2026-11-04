// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.HashMap
// Java: import java.util.HashSet
// Java: import java.util.List
// Java: import java.util.Map
// Java: import java.util.Set
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.ml.distance.DistanceMeasure
use crate::org::apache::commons::math3::ml::distance::DistanceMeasure;
// Java: import org.apache.commons.math3.ml.distance.EuclideanDistance
use crate::org::apache::commons::math3::ml::distance::EuclideanDistance;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java: public class DBSCANClusterer extends Clusterable
pub struct DBSCANClusterer {
    // Java field: double eps
    pub(crate) eps: f64,
    // Java field: int minPts
    pub(crate) minPts: i32,
}

// Java: public enum PointStatus
pub struct PointStatus {
}

// Java: public class DBSCANClusterer extends Clusterable
impl DBSCANClusterer {
    // Java: DBSCANClusterer(final double eps, final int minPts)
    pub fn DBSCANClusterer(&self) {
    }

    // Java: DBSCANClusterer(final double eps, final int minPts, final DistanceMeasure measure)
    pub fn DBSCANClusterer_2(&self) {
    }

    // Java: double getEps()
    pub fn getEps(&self) -> f64 {
        // Java: return eps;
        todo!()
    }

    // Java: int getMinPts()
    pub fn getMinPts(&self) -> i32 {
        // Java: return minPts;
        todo!()
    }

    // Java: List<Cluster<T>> cluster(final Collection<T> points)
    pub fn cluster(&self) -> Box<dyn std::any::Any> {
        // Java: // sanity checks
        // Java: MathUtils.checkNotNull(points);
        // Java: final List<Cluster<T>> clusters = new ArrayList<Cluster<T>>();
        // Java: final Map<Clusterable, PointStatus> visited = new HashMap<Clusterable, PointStatus>();
        // Java: for (final T point : points)
        {
            // Java: if (visited.get(point) != null)
            {
                // Java: continue;
            }
            // Java: final List<T> neighbors = getNeighbors(point, points);
            // Java: if (neighbors.size() >= minPts)
            {
                // Java: // DBSCAN does not care about center points
                // Java: final Cluster<T> cluster = new Cluster<T>();
                // Java: clusters.add(expandCluster(cluster, point, neighbors, points, visited));
            }
            // Java: else
            {
                // Java: visited.put(point, PointStatus.NOISE);
            }
        }
        // Java: return clusters;
        todo!()
    }

    // Java: Cluster<T> expandCluster()
    pub fn expandCluster(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: List<T> getNeighbors(final T point, final Collection<T> points)
    pub fn getNeighbors(&self) -> Box<dyn std::any::Any> {
        // Java: final List<T> neighbors = new ArrayList<T>();
        // Java: for (final T neighbor : points)
        {
            // Java: if (point != neighbor && distance(neighbor, point) <= eps)
            {
                // Java: neighbors.add(neighbor);
            }
        }
        // Java: return neighbors;
        todo!()
    }

    // Java: List<T> merge(final List<T> one, final List<T> two)
    pub fn merge(&self) -> Box<dyn std::any::Any> {
        // Java: final Set<T> oneSet = new HashSet<T>(one);
        // Java: for (T item : two)
        {
            // Java: if (!oneSet.contains(item))
            {
                // Java: one.add(item);
            }
        }
        // Java: return one;
        todo!()
    }

}

// Java: public enum PointStatus
impl PointStatus {
}
