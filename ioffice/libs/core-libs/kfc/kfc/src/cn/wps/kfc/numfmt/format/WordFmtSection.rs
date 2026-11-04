// Java field: int MAX_GROUP
#[allow(dead_code)]
pub(crate) const MAX_GROUP: i32 = 3;

// Java: public class WordFmtSection
pub struct WordFmtSection {
    // Java field: String[] mSections
    pub(crate) mSections: Vec<String>,
    // Java field: int mGroupCount
    pub(crate) mGroupCount: i32,
}

// Java: public class WordFmtSection
impl WordFmtSection {
    // Java: WordFmtSection()
    pub fn new() -> Self {
      // Java: super();
      WordFmtSection {
          mSections: vec![String::new(), String::new(), String::new()],
          mGroupCount: 0,
      }
    }

    // Java: String[] getSections()
    pub fn getSections(&self) -> &[String] {
      // Java: return this.mSections;
      &self.mSections
    }

    // Java: int getFmtCount()
    pub fn getFmtCount(&self) -> i32 {
      // Java: return this.mGroupCount;
      self.mGroupCount
    }

    // Java: String getFormatSection(double value)
    pub fn getFormatSection(&self, value: f64) -> &str {
        // Java: if (value > (double)0.0F)
      if value > 0.0
      {
         // Java: return this.mSections[0];
         &self.mSections[0]
      }
        // Java: else
      else
      {
         // Java: return value == (double)0.0F ? this.mSections[2] : this.mSections[1];
         if value == 0.0 { &self.mSections[2] } else { &self.mSections[1] }
      }
    }

    // Java: void reset(String formatCode)
    pub fn reset(&mut self, format_code: &str) {
      // Java: this.mGroupCount = this.collect(formatCode);
      self.mGroupCount = self.collect(format_code);
        // Java: switch (this.mGroupCount)
      match self.mGroupCount {
         // Java: case 1:
         1 => {
            // Java: this.mSections[1] = this.mSections[2] = this.mSections[0];
            let s = self.mSections[0].clone();
            self.mSections[1] = s.clone();
            self.mSections[2] = s;
            // Java: break;
         }
         // Java: case 2:
         2 => {
            // Java: this.mSections[2] = this.mSections[0];
            self.mSections[2] = self.mSections[0].clone();
         }
         _ => {}
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: for(int i = 0; i < this.mSections.length; ++i)
      for i in 0..self.mSections.len()
      {
         // Java: this.mSections[i] = "";
         self.mSections[i] = String::new();
      }
    }

    // Java: int collect(String formatCode)
    pub fn collect(&mut self, format_code: &str) -> i32 {
      // Java: this.clear();
      self.clear();
        // Java: if (formatCode.indexOf(59) == -1)
      if !format_code.contains(';')
      {
         // Java: this.mSections[0] = formatCode;
         self.mSections[0] = format_code.to_string();
         // Java: return 1;
         return 1;
      }
        // Java: else
      else
      {
         // Java: boolean isInRef = false;
         let mut is_in_ref = false;
         // Java: int groupIdx = 0;
         let mut group_idx: usize = 0;
         // Java: int start = 0;
         let mut start: usize = 0;
         // Java: int len = formatCode.length();
         let len = format_code.len();
            // Java: for(int i = 0; i < len && groupIdx < 3; ++i)
         for (i, c) in format_code.char_indices()
         {
            if group_idx >= 3 { break; }
            // Java: char c = formatCode.charAt(i);
                // Java: switch (c)
            match c {
               // Java: case '\'':
               '\'' => {
                  // Java: isInRef = !isInRef;
                  is_in_ref = !is_in_ref;
                  // Java: break;
               }
               // Java: case ';':
               ';' => {
                    // Java: if (!isInRef)
                  if !is_in_ref
                  {
                     // Java: this.mSections[groupIdx++] = formatCode.substring(start, i);
                     self.mSections[group_idx] = format_code[start..i].to_string();
                     group_idx += 1;
                     // Java: start = i + 1;
                     start = i + 1;
                  }
               }
               _ => {}
            }
         }
            // Java: if (start <= len && groupIdx < 3)
         if start <= len && group_idx < 3
         {
            // Java: this.mSections[groupIdx++] = formatCode.substring(start, len);
            self.mSections[group_idx] = format_code[start..len].to_string();
            group_idx += 1;
         }
         // Java: return groupIdx;
         return group_idx as i32;
      }
    }

}
