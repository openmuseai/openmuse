// Java: public class UofUnit
pub struct UofUnit {
    // Java field: String valueString
    valueString: Option<String>,
    // Java field: Integer value1
    pub(crate) value1: Option<i32>,
    // Java field: Integer value2
    pub(crate) value2: Option<i32>,
    // Java field: String unit
    unit: Option<String>,
}

// Java: public class UofUnit
impl UofUnit {
    // Java: UofUnit(String value)
    pub fn UofUnit(value: String) -> Self {
      // Java: super();
      let mut u = UofUnit {
          valueString: None,
          value1: None,
          value2: None,
          unit: None,
      };
      // Java: this.valueString = value;
      u.valueString = Some(value);
      // Java: this.parse();
      u.parse();
      u
    }

    // Java: void parse()
    fn parse(&mut self) {
      // Java: String[] strings = this.valueString.split(" ");
      let strings: Vec<&str> = self.valueString.as_ref().unwrap().split(' ').collect();
      // Java: for(int i = 0; i < strings.length; ++i)
      for i in 0..strings.len()
      {
         // Java: if (i == 0)
         if i == 0
         {
            // Java: this.value1 = Integer.parseInt(strings[i]);
            self.value1 = strings[i].parse::<i32>().ok();
         }
         // Java: else if (i == 1)
         else if i == 1
         {
            // Java: if (Character.isDigit(strings[i].charAt(0)))
            if strings[i].chars().next().map_or(false, |c| c.is_ascii_digit())
            {
               // Java: this.value2 = Integer.parseInt(strings[2]);
               self.value2 = strings[2].parse::<i32>().ok();
            }
            // Java: else if (Character.isLetter(strings[i].charAt(0)))
            else if strings[i].chars().next().map_or(false, |c| c.is_alphabetic())
            {
               // Java: this.unit = strings[i];
               self.unit = Some(strings[i].to_string());
               // Java: break;
               break;
            }
         }
         // Java: else if (i == 2)
         else if i == 2
         {
            // Java: this.unit = strings[i];
            self.unit = Some(strings[i].to_string());
         }
      }
    }

    // Java: Integer getValue1()
    pub fn getValue1(&self) -> Option<i32> {
      // Java: return this.value1;
      self.value1
    }

    // Java: Integer getValue2()
    pub fn getValue2(&self) -> Option<i32> {
      // Java: return this.value2;
      self.value2
    }

}
