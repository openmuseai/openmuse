// Java: import java.lang.Object
use stubs::java::lang::Object::Object;

// java private static final int SCALE = 1000;
const SCALE: i32 = 1000;

// java public class LineSpacing
#[derive(Default, Clone)]
pub struct LineSpacing {
    // java private int rule;
    rule: i32,
    // java private float spacing;
    spacing: f32,
}

// java public class LineSpacing
impl LineSpacing {
    // java public  LineSpacing(int rule, float spacing)
    pub fn LineSpacing(rule: i32, spacing: f32) -> Self {
      // java this.rule = rule;
      let rule = rule;
      // java this.spacing = spacing;
      let spacing = spacing;
      Self { rule, spacing }
    }

    pub fn new(rule: i32, spacing: f32) -> Self {
      Self::LineSpacing(rule, spacing)
    }

    // java public void setRule(int rule)
    pub fn setRule(&mut self, rule: i32) {
      // java this.rule = rule;
      self.rule = rule;
    }

    // java public void setSpacing(float line)
    pub fn setSpacing(&mut self, line: f32) {
      // java this.spacing = line;
      self.spacing = line;
    }

    // java public int getRule()
    pub fn getRule(&self) -> i32 {
      // java return this.rule;
      return self.rule;
    }

    // java public float getSpacing()
    pub fn getSpacing(&self) -> f32 {
      // java return this.spacing;
      return self.spacing;
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return this.rule + (int)(this.spacing * 1000.0F);
      return self.rule + ((self.spacing * 1000.0) as i32);
    }

    // java public boolean equals(Object obj)
    pub fn equals(&self, obj: Option<&dyn Object>) -> bool {
      // java if (null != obj && obj instanceof LineSpacing)
      if let Some(obj) = obj {
        if obj.as_any().downcast_ref::<LineSpacing>().is_some() {
          // java if (obj == this)
          if std::ptr::eq(
            obj.as_any() as *const dyn std::any::Any as *const (),
            self as *const LineSpacing as *const dyn std::any::Any as *const (),
          ) {
            // java return true;
            return true;
          }
          // java else
          else {
            // java LineSpacing lineSpacing = (LineSpacing)obj;
            let lineSpacing: &LineSpacing = obj.as_any().downcast_ref::<LineSpacing>().unwrap();
            // java return this.rule == lineSpacing.rule && (int)(this.spacing * 1000.0F) == (int)(lineSpacing.spacing * 1000.0F);
            return self.rule == lineSpacing.rule && ((self.spacing * 1000.0) as i32) == ((lineSpacing.spacing * 1000.0) as i32);
          }
        } else {
          // rust: no-stmt
          return false;
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder sb = new StringBuilder();
      let mut sb = String::new();
      // java sb.append("{lineRule = ");
      sb.push_str("{lineRule = ");
      // java if (this.rule == 0)
      if self.rule == 0 {
        // java sb.append("auto");
        sb.push_str("auto");
      }
      // java if (this.rule == 1)
      if self.rule == 1 {
        // java sb.append("exact");
        sb.push_str("exact");
      }
      // java if (this.rule == 2)
      if self.rule == 2 {
        // java sb.append("atleast");
        sb.push_str("atleast");
      }
      // java sb.append("\t line = " + this.spacing + "}");
      sb.push_str(&format!("\t line = {}}}", self.spacing));
      // java return sb.toString();
      return sb;
    }

}
