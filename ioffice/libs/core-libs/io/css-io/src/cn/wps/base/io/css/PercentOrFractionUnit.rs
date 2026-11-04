// Java: import cn.wps.base.assertion.Assert

// Java: public enum Unit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    FRACTION,
    PERCENT,
    FLOAT_VAL,
}

// Java: public class PercentOrFractionUnit extends cn
pub struct PercentOrFractionUnit {
    // Java field: Unit mUnit
    pub(crate) m_unit: Unit,
    pub(crate) m_magnitude: Option<String>,
}

// Java: public class PercentOrFractionUnit extends cn
impl PercentOrFractionUnit {
    // Java: PercentOrFractionUnit()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.mUnit = PercentOrFractionUnit.Unit.FLOAT_VAL;
        // Java: this.mMagnitude = null;
        PercentOrFractionUnit {
            m_unit: Unit::FLOAT_VAL,
            m_magnitude: None,
        }
    }

    // Java: PercentOrFractionUnit(String value)
    pub fn from_value(value: &str) -> Self {
        // Java: this();
        let mut instance = Self::new();
        // Java: Assert.assertNotNull("value should not be null", value);
        // Java: this.setValue(value);
        instance.set_value(value);
        instance
    }

    // Java: void setUnit(String unit)
    pub fn set_unit(&mut self, unit: &str) {
        // Java: Assert.assertNotNull("unit should not be true", unit);
        // Java: unit = unit.trim();
        let unit = unit.trim();
        // Java: if (null != unit && unit.length() > 0)
        if !unit.is_empty() {
            // Java: if ("f".equalsIgnoreCase(unit))
            if unit.eq_ignore_ascii_case("f") {
                // Java: this.mUnit = PercentOrFractionUnit.Unit.FRACTION;
                self.m_unit = Unit::FRACTION;
            // Java: else if ("%".equalsIgnoreCase(unit))
            } else if unit.eq_ignore_ascii_case("%") {
                // Java: this.mUnit = PercentOrFractionUnit.Unit.PERCENT;
                self.m_unit = Unit::PERCENT;
            // Java: else
            } else {
                // Java: Assert.fail("unreognized Percent unit type is met: " + unit);
                panic!("unrecognized Percent unit type is met: {}", unit);
            }
        // Java: else
        } else {
            // Java: this.mUnit = PercentOrFractionUnit.Unit.FLOAT_VAL;
            self.m_unit = Unit::FLOAT_VAL;
        }
    }

    fn set_value(&mut self, _value: &str) {
    }
}

// Java: public enum Unit
impl Unit {
    // Java: Unit()
    pub fn new() -> Self {
        // Java: super();
        Unit::FLOAT_VAL
    }
}
