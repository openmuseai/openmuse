// Java: public class AffEntry
pub struct AffEntry {
    // Java field: String strip
    pub(crate) strip: Option<String>,
    // Java field: String appnd
    pub(crate) appnd: Option<String>,
    // Java field: int numconds
    pub(crate) numconds: i32,
    // Java field: char[] conds
    pub(crate) conds: Option<Vec<char>>,
    // Java field: char opts
    pub(crate) opts: char,
    // Java field: char aflag
    pub(crate) aflag: char,
    // Java field: char[] contclass
    pub(crate) contclass: Option<Vec<char>>,
}

// Java: public class AffEntry
impl AffEntry {
    // Java: AffEntry()
    pub fn new() -> Self {
        // Java: super();
        Self {
            strip: None,
            appnd: None,
            numconds: 0,
            conds: None,
            opts: '\0',
            aflag: '\0',
            contclass: None,
        }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.strip = null;
        self.strip = None;
        // Java: this.appnd = null;
        self.appnd = None;
        // Java: this.numconds = 0;
        self.numconds = 0;
        // Java: this.conds = null;
        self.conds = None;
        // Java: this.opts = 0;
        self.opts = '\0';
        // Java: this.aflag = 0;
        self.aflag = '\0';
        // Java: this.contclass = null;
        self.contclass = None;
    }
}
