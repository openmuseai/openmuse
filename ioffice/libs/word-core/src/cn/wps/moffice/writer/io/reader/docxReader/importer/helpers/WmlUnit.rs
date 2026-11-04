// Java: import cn.wps.base.assertion.IOAssert
use util::base::assertion::IOAssert::*;
// Java: import cn.wps.base.io.Unit
use util::base::io::Unit::Unit as BaseUnit;

// java public class WmlUnit extends cn.wps.base.io.Unit
pub struct WmlUnit {
    // java cn.wps.base.io.Unit super_
    pub(crate) super_: BaseUnit,
    // java public Unit mUnit;
    pub(crate) mUnit: Option<Unit>,
}

// java public static enum Unit
#[derive(Copy, Clone, Debug)]
pub enum Unit {
    EMU,
    PT,
    MM,
    CM,
    IN,
    PC,
    PI,
}

// java public class WmlUnit extends cn.wps.base.io.Unit
impl WmlUnit {
    // java public  WmlUnit(String value)
    pub fn WmlUnit(value: &str) -> Self {
      // java IOAssert.assertNotNull("value should not be null", value);
      let value_opt: Option<&str> = Some(value);
      IOAssert::assertNotNull_msg("value should not be null", &value_opt);
      // java this.mUnit = null;
      let mut this = Self { super_: BaseUnit::new(), mUnit: None };
      // java this.setValue(value);
      this.super_.setValue(value);
      // Java Unit.setValue ends with this.setUnit(unit) on the WmlUnit receiver.
      if let Some(trimmed) = BaseUnit::trim(value) {
        if !trimmed.is_empty() {
          let i = util::moffice::util::StringUtil::StringUtil::findLastDigitFromEnd(&trimmed);
          if i >= 0 {
            let idx = (i + 1) as usize;
            this.setUnit(&trimmed[idx..]);
          }
        }
      }
      this
    }

    // java protected void setUnit(String unit)
    pub fn setUnit(&mut self, unit: &str) {
      // java IOAssert.assertNotNull("unit should not be null", unit);
      let unit_opt: Option<&str> = Some(unit);
      IOAssert::assertNotNull_msg("unit should not be null", &unit_opt);
      // java unit = unit.trim();
      let unit = unit.trim();
      // java if (null != unit && unit.length() > 0)
      if !unit.is_empty() {
        // java if ("pt".equalsIgnoreCase(unit))
        if unit.eq_ignore_ascii_case("pt") {
          // java this.mUnit = WmlUnit.Unit.PT;
          self.mUnit = Some(Unit::PT);
        }
        // java else if ("mm".equalsIgnoreCase(unit))
        else if unit.eq_ignore_ascii_case("mm") {
          // java this.mUnit = WmlUnit.Unit.MM;
          self.mUnit = Some(Unit::MM);
        }
        // java else if ("cm".equalsIgnoreCase(unit))
        else if unit.eq_ignore_ascii_case("cm") {
          // java this.mUnit = WmlUnit.Unit.CM;
          self.mUnit = Some(Unit::CM);
        }
        // java else if ("in".equalsIgnoreCase(unit))
        else if unit.eq_ignore_ascii_case("in") {
          // java this.mUnit = WmlUnit.Unit.IN;
          self.mUnit = Some(Unit::IN);
        }
        // java else if ("pc".equalsIgnoreCase(unit))
        else if unit.eq_ignore_ascii_case("pc") {
          // java this.mUnit = WmlUnit.Unit.PC;
          self.mUnit = Some(Unit::PC);
        }
        // java else if ("pi".equalsIgnoreCase(unit))
        else if unit.eq_ignore_ascii_case("pi") {
          // java this.mUnit = WmlUnit.Unit.PI;
          self.mUnit = Some(Unit::PI);
        }
        // java else
        else {
          // java IOAssert.fail("unreognized unit type of VmlUnit is met:" + unit);
          IOAssert::fail_msg(&format!("unreognized unit type of VmlUnit is met:{}", unit));
        }
      }
      // java else
      else {
        // java this.mUnit = null;
        self.mUnit = None;
      }
    }

}

impl std::ops::Deref for WmlUnit {
    type Target = BaseUnit;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for WmlUnit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static enum Unit
impl Unit {
}
