// Java: import java.util.List
// Java: import org.apache.commons.math3.ml.clustering.Cluster
use crate::org::apache::commons::math3::ml::clustering::Cluster;
// Java: import org.apache.commons.math3.ml.clustering.Clusterable
use crate::org::apache::commons::math3::ml::clustering::Clusterable;
// Java: import org.apache.commons.math3.ml.distance.DistanceMeasure
use crate::org::apache::commons::math3::ml::distance::DistanceMeasure;
// Java: import org.apache.commons.math3.stat.descriptive.moment.Variance
use crate::org::apache::commons::math3::stat::descriptive::moment::Variance;

// Java: public class SumOfClusterVariances extends Clusterable
pub struct SumOfClusterVariances {
}

// Java: public class SumOfClusterVariances extends Clusterable
impl SumOfClusterVariances {
    // Java: SumOfClusterVariances(final DistanceMeasure measure)
    pub fn SumOfClusterVariances(&self) {
        // Java: super(measure);
        todo!()
    }

    // Java: double score(final List<? extends Cluster<T>> clusters)
    pub fn score(&self) -> f64 {
        // Java: double varianceSum = 0.0;
        // Java: for (final Cluster<T> cluster : clusters)
        {
            // Java: if (!cluster.getPoints().isEmpty())
            {
                // Java: final Clusterable center = centroidOf(cluster);
                // Java: // compute the distance variance of the current cluster
                // Java: final Variance stat = new Variance();
                // Java: for (final T point : cluster.getPoints())
                {
                    // Java: stat.increment(distance(point, center));
                }
                // Java: varianceSum += stat.getResult();
            }
        }
        // Java: return varianceSum;
        todo!()
    }

}
