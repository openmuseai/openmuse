// java  class Stack
#[derive(Default)]
pub struct Stack {
    // java public String[] stackOperator = null;
    pub(crate) stackOperator: Option<Vec<Option<String>>>,
    // java public int currentStackLength = -1;
    pub(crate) currentStackLength: i32,
}

// java  class Stack
impl Stack {
    // java private  Stack()
    fn new_private() -> Self {
      Self {
        stackOperator: None,
        currentStackLength: -1,
      }
    }

    // java public static Stack getStackInstance()
    pub fn getStackInstance() -> Stack {
      // java return new Stack();
      return Self::new_private();
    }

    // java public void posh(String ch)
    pub fn posh(&mut self, ch: &str) {
      // java if (!this.isFull())
      if !self.isFull() {
        // java this.stackOperator[++this.currentStackLength] = ch;
        self.currentStackLength += 1;
        if let Some(ref mut ops) = self.stackOperator {
          ops[self.currentStackLength as usize] = Some(ch.to_string());
        }
      }
    }

    // java public String pop()
    pub fn pop(&mut self) -> Option<String> {
      // java if (!this.isEmpty())
      if !self.isEmpty() {
        // java String tem = this.stackOperator[this.currentStackLength--];
        let idx = self.currentStackLength;
        self.currentStackLength -= 1;
        // java return tem;
        return self.stackOperator.as_mut().and_then(|ops| ops[idx as usize].take());
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // java return this.currentStackLength == -1;
      return self.currentStackLength == -1;
    }

    // java public boolean isFull()
    pub fn isFull(&self) -> bool {
      // java return this.currentStackLength == this.stackOperator.length - 1;
      let len = self.stackOperator.as_ref().map(|a| a.len() as i32).unwrap_or(0);
      return self.currentStackLength == len - 1;
    }

    // java public int getStackLength()
    pub fn getStackLength(&self) -> i32 {
      // java return this.currentStackLength;
      return self.currentStackLength;
    }

    // java public String getTop()
    pub fn getTop(&self) -> Option<String> {
      // java return !this.isEmpty() ? this.stackOperator[this.currentStackLength] : null;
      return if !self.isEmpty() {
        self.stackOperator.as_ref().and_then(|ops| ops[self.currentStackLength as usize].clone())
      } else {
        None
      };
    }

    // java public void setStackLength(int length)
    pub fn setStackLength(&mut self, length: i32) {
      // java this.stackOperator = new String[length];
      self.stackOperator = Some(vec![None; length as usize]);
    }

    // java public String getBottom()
    pub fn getBottom(&self) -> Option<String> {
      // java return this.stackOperator[0];
      return self.stackOperator.as_ref().and_then(|ops| ops.get(0).and_then(|s| s.clone()));
    }

    // java public String getIndex(int index)
    pub fn getIndex(&self, index: i32) -> Option<String> {
      // java return index >= 0 && index <= this.stackOperator.length ? this.stackOperator[index] : null;
      let len = self.stackOperator.as_ref().map(|a| a.len() as i32).unwrap_or(0);
      return if index >= 0 && index <= len {
        self.stackOperator.as_ref().and_then(|ops| ops.get(index as usize).and_then(|s| s.clone()))
      } else {
        None
      };
    }

    // java public void recycle()
    pub fn recycle(&mut self) {
      // java this.stackOperator = null;
      self.stackOperator = None;
      // java this.currentStackLength = -1;
      self.currentStackLength = -1;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.stackOperator = null;
      self.stackOperator = None;
    }

}
