#![allow(non_snake_case, non_upper_case_globals, dead_code)]

// Java: public class ThemeColor
pub struct ThemeColor {
    // Java field: int accent1
    pub accent1: i32,
    // Java field: int accent2
    pub accent2: i32,
    // Java field: int accent3
    pub accent3: i32,
    // Java field: int accent4
    pub accent4: i32,
    // Java field: int accent5
    pub accent5: i32,
    // Java field: int accent6
    pub accent6: i32,
    // Java field: int dk1
    pub dk1: i32,
    // Java field: int dk2
    pub dk2: i32,
    // Java field: int folHlink
    pub folHlink: i32,
    // Java field: int hlink
    pub hlink: i32,
    // Java field: int lt1
    pub lt1: i32,
    // Java field: int lt2
    pub lt2: i32,
    // Java field: int phClr
    pub phClr: i32,
    // Java field: int tx1
    pub tx1: i32,
    // Java field: int tx2
    pub tx2: i32,
    // Java field: int bg1
    pub bg1: i32,
    // Java field: int bg2
    pub bg2: i32,
    // Java field: String accent1Name
    pub accent1Name: String,
    // Java field: String accent2Name
    pub accent2Name: String,
    // Java field: String accent3Name
    pub accent3Name: String,
    // Java field: String accent4Name
    pub accent4Name: String,
    // Java field: String accent5Name
    pub accent5Name: String,
    // Java field: String accent6Name
    pub accent6Name: String,
    // Java field: String folHlinkName
    pub folHlinkName: String,
    // Java field: String hlinkName
    pub hlinkName: String,
    // Java field: String tx1Name
    pub tx1Name: String,
    // Java field: String tx2Name
    pub tx2Name: String,
    // Java field: String bg1Name
    pub bg1Name: String,
    // Java field: String bg2Name
    pub bg2Name: String,
    // Java field: String lt1Name
    pub lt1Name: String,
    // Java field: String lt2Name
    pub lt2Name: String,
    // Java field: String dk1Name
    pub dk1Name: String,
    // Java field: String dk2Name
    pub dk2Name: String,
}

// Java: public class ThemeColor
impl ThemeColor {
    // Java: ThemeColor()
    pub fn new() -> Self {
      // Java: super();
      let mut instance = ThemeColor {
          accent1: 0, accent2: 0, accent3: 0, accent4: 0, accent5: 0, accent6: 0,
          dk1: 0, dk2: 0, folHlink: 0, hlink: 0, lt1: 0, lt2: 0, phClr: 0,
          tx1: 0, tx2: 0, bg1: 0, bg2: 0,
          accent1Name: String::new(), accent2Name: String::new(),
          accent3Name: String::new(), accent4Name: String::new(),
          accent5Name: String::new(), accent6Name: String::new(),
          folHlinkName: String::new(), hlinkName: String::new(),
          tx1Name: String::new(), tx2Name: String::new(),
          bg1Name: String::new(), bg2Name: String::new(),
          lt1Name: String::new(), lt2Name: String::new(),
          dk1Name: String::new(), dk2Name: String::new(),
      };
      instance.init();
      instance
    }

    fn init(&mut self) {
      self.accent1 = -11566659;
      self.accent2 = -4173747;
      self.accent3 = -6571175;
      self.accent4 = -8362846;
      self.accent5 = -11817786;
      self.accent6 = -551354;
      self.dk1 = -16777216;
      self.dk2 = -14726787;
      self.folHlink = -8388480;
      self.hlink = -16776961;
      self.lt1 = -1;
      self.lt2 = -1119007;
      self.phClr = -16777216;
      // Java: this.tx1 = this.dk1;
      self.tx1 = self.dk1;
      // Java: this.tx2 = this.dk2;
      self.tx2 = self.dk2;
      // Java: this.bg1 = this.lt1;
      self.bg1 = self.lt1;
      // Java: this.bg2 = this.lt2;
      self.bg2 = self.lt2;
      // Java: this.accent1Name = "accent1";
      self.accent1Name = "accent1".to_string();
      // Java: this.accent2Name = "accent2";
      self.accent2Name = "accent2".to_string();
      // Java: this.accent3Name = "accent3";
      self.accent3Name = "accent3".to_string();
      // Java: this.accent4Name = "accent4";
      self.accent4Name = "accent4".to_string();
      // Java: this.accent5Name = "accent5";
      self.accent5Name = "accent5".to_string();
      // Java: this.accent6Name = "accent6";
      self.accent6Name = "accent6".to_string();
      // Java: this.folHlinkName = "folHlink";
      self.folHlinkName = "folHlink".to_string();
      // Java: this.hlinkName = "hlink";
      self.hlinkName = "hlink".to_string();
      // Java: this.tx1Name = "dk1";
      self.tx1Name = "dk1".to_string();
      // Java: this.tx2Name = "dk2";
      self.tx2Name = "dk2".to_string();
      // Java: this.bg1Name = "lt1";
      self.bg1Name = "lt1".to_string();
      // Java: this.bg2Name = "lt2";
      self.bg2Name = "lt2".to_string();
      // Java: this.lt1Name = "lt1";
      self.lt1Name = "lt1".to_string();
      // Java: this.lt2Name = "lt2";
      self.lt2Name = "lt2".to_string();
      // Java: this.dk1Name = "dk1";
      self.dk1Name = "dk1".to_string();
      // Java: this.dk2Name = "dk2";
      self.dk2Name = "dk2".to_string();
    }

}
