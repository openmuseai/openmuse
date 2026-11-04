use super::PacketProperty::PacketProperty;
use super::MetricScale::MetricScale;

// Java: public class Metric
pub struct Metric {
    // Java field: int mType
    pub(crate) mType: i32,
    // Java field: long mMin
    pub(crate) mMin: i64,
    // Java field: long mMax
    pub(crate) mMax: i64,
    // Java field: int mUnits
    pub(crate) mUnits: i32,
    // Java field: float mResolution
    pub(crate) mResolution: f32,
}

// Java: public class Metric
impl Metric {
    // Java: Metric()
    pub fn Metric() -> Self {
        // Java: super();
        Self {
            mType: 0,
            mMin: 0,
            mMax: 0,
            mUnits: 0,
            mResolution: 0.0,
        }
    }

    // Java: Metric(int type, long min, long max, int units, float resolution)
    pub fn Metric_with_params(r#type: i32, min: i64, max: i64, units: i32, resolution: f32) -> Self {
        // Java: super();
        // Java: this.mType = type;
        // Java: this.mMin = min;
        // Java: this.mMax = max;
        // Java: this.mUnits = units;
        // Java: this.mResolution = resolution;
        Self {
            mType: r#type,
            mMin: min,
            mMax: max,
            mUnits: units,
            mResolution: resolution,
        }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder sb = new StringBuilder();
        let mut sb = String::new();
        // Java: sb.append(PacketProperty.toName(this.mType));
        sb.push_str(&PacketProperty::toName(self.mType));
        // Java: sb.append('\n');
        sb.push('\n');
        // Java: sb.append("mMin:");
        sb.push_str("mMin:");
        // Java: sb.append(this.mMin);
        sb.push_str(&self.mMin.to_string());
        // Java: sb.append(" ");
        sb.push(' ');
        // Java: sb.append("mMax:");
        sb.push_str("mMax:");
        // Java: sb.append(this.mMax);
        sb.push_str(&self.mMax.to_string());
        // Java: sb.append(" ");
        sb.push(' ');
        // Java: sb.append("mUnits:");
        sb.push_str("mUnits:");
        // Java: sb.append(MetricScale.toName(this.mUnits));
        sb.push_str(&MetricScale::toName(self.mUnits));
        // Java: sb.append(" ");
        sb.push(' ');
        // Java: sb.append("mResolution:");
        sb.push_str("mResolution:");
        // Java: sb.append(this.mResolution);
        sb.push_str(&self.mResolution.to_string());
        // Java: sb.append(" \n");
        sb.push_str(" \n");
        // Java: return sb.toString();
        sb
    }
}
