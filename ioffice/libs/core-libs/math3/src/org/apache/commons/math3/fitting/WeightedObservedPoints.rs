// Java: import java.util.List
// Java: import java.util.ArrayList
// Java: import java.io.Serializable

use crate::org::apache::commons::math3::fitting::WeightedObservedPoint;

// Java field: long serialVersionUID

// Java: public class WeightedObservedPoints implements Serializable
pub struct WeightedObservedPoints {
    observations: Vec<WeightedObservedPoint::WeightedObservedPoint>,
}

// Java: public class WeightedObservedPoints implements Serializable
impl WeightedObservedPoints {
    pub fn new() -> Self {
        WeightedObservedPoints { observations: Vec::new() }
    }

    // Java: void add(double x, double y)
    pub fn add_xy(&mut self, x: f64, y: f64) {
        // Java: add(1d, x, y);
        self.add_weighted(1.0, x, y);
    }

    // Java: void add(double weight, double x, double y)
    pub fn add_weighted(&mut self, weight: f64, x: f64, y: f64) {
        // Java: observations.add(new WeightedObservedPoint(weight, x, y));
        self.observations.push(WeightedObservedPoint::WeightedObservedPoint::new(weight, x, y));
    }

    // Java: void add(WeightedObservedPoint observed)
    pub fn add(&mut self, observed: WeightedObservedPoint::WeightedObservedPoint) {
        // Java: observations.add(observed);
        self.observations.push(observed);
    }

    // Java: List<WeightedObservedPoint> toList()
    pub fn toList(&self) -> Vec<WeightedObservedPoint::WeightedObservedPoint> {
        // Java: // The copy is necessary to ensure thread-safety because of the
        // Java: // "clear" method (which otherwise would be able to empty the
        // Java: // list of points while it is being used by another thread).
        // Java: return new ArrayList<WeightedObservedPoint>(observations);
        self.observations.clone()
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: observations.clear();
        self.observations.clear();
    }

}
