// Java: import cn.wps.base.assertion.KSAssert
// Java: import java.util.HashMap
use std::collections::HashMap;
use super::Metric::Metric;

// Java: public class Metrics
pub struct Metrics {
    // Java field: HashMap<Integer, Metric> mItems
    pub(crate) mItems: HashMap<i32, Metric>,
}

// Java: public class Metrics
impl Metrics {
    // Java: Metrics()
    pub fn Metrics() -> Self {
        // Java: super();
        Self {
            mItems: HashMap::new(),
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: KSAssert.assertNotNull("mItems should not be null!", this.mItems);
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append("Metrics\n");
        sb.push_str("Metrics\n");
        // Java: for(Integer key : this.mItems.keySet())
        for (_key, metric) in &self.mItems {
            // Java: Metric metric = (Metric)this.mItems.get(key);
            // Java: sb.append(metric.toString());
            sb.push_str(&metric.toString());
            // Java: sb.append("\n");
            sb.push('\n');
        }
        // Java: return sb.toString();
        sb
    }
}
