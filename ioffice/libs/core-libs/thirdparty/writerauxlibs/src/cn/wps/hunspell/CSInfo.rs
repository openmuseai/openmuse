// Java: public class CSInfo
#[derive(Clone)]
pub struct CSInfo {
    // Java field: char[][] tbl
    tbl: Vec<Vec<char>>,
}

// Java: public class CSInfo
impl CSInfo {
    // Java: CSInfo(char[][] tbl)
    pub fn new(tbl: Vec<Vec<char>>) -> Self {
        // Java: super();
        // Java: this.tbl = tbl;
        Self { tbl }
    }

    // Java: char getCCase(char c)
    pub fn getCCase(&self, c: char) -> char {
        // Java: if (c >= this.tbl.length)
        if (c as usize) >= self.tbl.len() {
            // Java: return '\u0000';
            return '\0';
        }
        // Java: else
        {
            // Java: char[] info = this.tbl[c];
            let info = &self.tbl[c as usize];
            // Java: return info[0];
            info[0]
        }
    }

    // Java: char getLower(char c)
    pub fn getLower(&self, c: char) -> char {
        // Java: if (c >= this.tbl.length)
        if (c as usize) >= self.tbl.len() {
            // Java: return '\u0000';
            return '\0';
        }
        // Java: else
        {
            // Java: char[] info = this.tbl[c];
            let info = &self.tbl[c as usize];
            // Java: return info[1];
            info[1]
        }
    }

    // Java: char getUpper(char c)
    pub fn getUpper(&self, c: char) -> char {
        // Java: if (c >= this.tbl.length)
        if (c as usize) >= self.tbl.len() {
            // Java: return '\u0000';
            return '\0';
        }
        // Java: else
        {
            // Java: char[] info = this.tbl[c];
            let info = &self.tbl[c as usize];
            // Java: return info[2];
            info[2]
        }
    }
}
