// java public class FindOptions
#[derive(Default)]
pub struct FindOptions {
    // java boolean matchCase = false;
    pub(crate) matchCase: bool,
    // java boolean matchWord = false;
    pub(crate) matchWord: bool,
    // java boolean excludeTable = false;
    pub(crate) excludeTable: bool,
    // java boolean excludeTextbox = false;
    pub(crate) excludeTextbox: bool,
    // java boolean excludeParaHide = false;
    pub(crate) excludeParaHide: bool,
}

// java public class FindOptions
impl FindOptions {
    // java public  FindOptions()
    pub fn new() -> Self {
      Self { matchCase: Default::default(), matchWord: Default::default(), excludeTable: Default::default(), excludeTextbox: Default::default(), excludeParaHide: Default::default() }
    }

    // java public  FindOptions(boolean matchCase, boolean matchWord)
    pub fn new__boolean_boolean(matchCase: bool, matchWord: bool) -> Self {
      // java this.matchCase = matchCase;
      Self { matchCase: matchCase, matchWord: matchWord, excludeTable: Default::default(), excludeTextbox: Default::default(), excludeParaHide: Default::default() }
      // java this.matchWord = matchWord;
    }

    // java public boolean isMatchCase()
    pub fn isMatchCase(&self) -> bool {
      // java return this.matchCase;
      return self.matchCase;
    }

    // java public void setMatchCase(boolean matchCase)
    pub fn setMatchCase(&mut self, matchCase: bool) {
      // java this.matchCase = matchCase;
      self.matchCase = matchCase;
    }

    // java public boolean isMatchWord()
    pub fn isMatchWord(&self) -> bool {
      // java return this.matchWord;
      return self.matchWord;
    }

    // java public void setMatchWord(boolean matchWord)
    pub fn setMatchWord(&mut self, matchWord: bool) {
      // java this.matchWord = matchWord;
      self.matchWord = matchWord;
    }

    // java public boolean isExcludeTable()
    pub fn isExcludeTable(&self) -> bool {
      // java return this.excludeTable;
      return self.excludeTable;
    }

    // java public void setExcludeTable(boolean excludeTable)
    pub fn setExcludeTable(&mut self, excludeTable: bool) {
      // java this.excludeTable = excludeTable;
      self.excludeTable = excludeTable;
    }

    // java public boolean isExcludeTextbox()
    pub fn isExcludeTextbox(&self) -> bool {
      // java return this.excludeTextbox;
      return self.excludeTextbox;
    }

    // java public void setExcludeTextbox(boolean excludeTextbox)
    pub fn setExcludeTextbox(&mut self, excludeTextbox: bool) {
      // java this.excludeTextbox = excludeTextbox;
      self.excludeTextbox = excludeTextbox;
    }

    // java public boolean isExcludeParaHide()
    pub fn isExcludeParaHide(&self) -> bool {
      // java return this.excludeParaHide;
      return self.excludeParaHide;
    }

    // java public void setExcludeParaHide(boolean excludeParaHide)
    pub fn setExcludeParaHide(&mut self, excludeParaHide: bool) {
      // java this.excludeParaHide = excludeParaHide;
      self.excludeParaHide = excludeParaHide;
    }

}
