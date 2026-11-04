// Java: import java.io.Serializable

// Java field: long serialVersionUID

// Java: public class is
pub struct r#is {
}

// Java: public class are
pub struct are {
}

// Java: public class WeightedObservedPoint
#[derive(Clone)]
pub struct WeightedObservedPoint {
    // Java field: double weight
    pub(crate) weight: f64,
    // Java field: double x
    pub(crate) x: f64,
    // Java field: double y
    pub(crate) y: f64,
}

// Java: public class is
impl r#is {
}

// Java: public class are
impl are {
}

// Java: public class WeightedObservedPoint
impl WeightedObservedPoint {
    // Java: WeightedObservedPoint(final double weight, final double x, final double y)
    pub fn new(weight: f64, x: f64, y: f64) -> Self {
        // Java: this.weight = weight;
        // Java: this.x      = x;
        // Java: this.y      = y;
        WeightedObservedPoint { weight, x, y }
    }

    // Java: double getWeight()
    pub fn getWeight(&self) -> f64 {
        // Java: return weight;
        self.weight
    }

    // Java: double getX()
    pub fn getX(&self) -> f64 {
        // Java: return x;
        self.x
    }

    // Java: double getY()
    pub fn getY(&self) -> f64 {
        // Java: return y;
        self.y
    }

}
