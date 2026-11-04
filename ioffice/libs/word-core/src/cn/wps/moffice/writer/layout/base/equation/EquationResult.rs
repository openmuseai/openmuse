#![allow(dead_code, non_snake_case)]
// Java: public class EquationResult

pub const STATE_NONE: i32 = 0;
pub const STATE_TEXT: i32 = 1;
pub const STATE_COMMENT: i32 = 2;
pub const STATE_ERROR: i32 = 5;
pub const STATE_FINISH: i32 = 6;

// java public class EquationResult
#[derive(Clone)]
pub struct EquationResult {
    // java private int mState = 0;
    mState: i32,
    // java private int mStartCP = -1;
    mStartCP: i32,
    // java private int mEndCP = -1;
    mEndCP: i32,
    // java private ArrayList<String> mOPs;
    mOPs: Option<Vec<String>>,
    // java private StringBuilder mTmpSB;
    mTmpSB: Option<String>,
    // java private WIntArrayList mTextAndValues;
    mTextAndValues: Option<Vec<i32>>,
    // java private WIntArrayList mOPStartCP;
    mOPStartCP: Option<Vec<i32>>,
    // java private boolean mHasStartValue;
    mHasStartValue: bool,
}

impl EquationResult {
    // java public EquationResult(int startCP)
    pub fn EquationResult(start_cp: i32) -> Self {
        Self {
            mState: STATE_NONE,
            mStartCP: start_cp,
            mEndCP: -1,
            mOPs: None,
            mTmpSB: None,
            mTextAndValues: None,
            mOPStartCP: None,
            mHasStartValue: false,
        }
    }

    // java public int getStartCP()
    pub fn getStartCP(&self) -> i32 {
        self.mStartCP
    }

    // java public void addOP(char ch, int cp)
    pub fn addOP(&mut self, ch: char, cp: i32) {
        // java if (null == this.mTmpSB)
        if self.mTmpSB.is_none() {
            self.mTmpSB = Some(String::new());
            // java if (null == this.mOPStartCP) this.mOPStartCP = new WIntArrayList(); this.mOPStartCP.add(cp);
            if self.mOPStartCP.is_none() {
                self.mOPStartCP = Some(Vec::new());
            }
            self.mOPStartCP.as_mut().unwrap().push(cp);
        }
        // java this.mTmpSB.append(ch);
        self.mTmpSB.as_mut().unwrap().push(ch);
    }

    // java public int getOPStartCP(int index)
    pub fn getOPStartCP(&self, index: i32) -> i32 {
        self.mOPStartCP.as_ref().unwrap()[index as usize]
    }

    // java public void addValue(int curCP)
    pub fn addValue(&mut self, cur_cp: i32) {
        self.push_span(cur_cp);
    }

    // java public void addText(int curCP)
    pub fn addText(&mut self, cur_cp: i32) {
        self.push_span(cur_cp);
    }

    fn push_span(&mut self, cur_cp: i32) {
        // java if (this.mHasStartValue) this.mTextAndValues.setQuick(size - 1, curCP);
        if self.mHasStartValue {
            let values = self.mTextAndValues.as_mut().unwrap();
            let last = values.len() - 1;
            values[last] = cur_cp;
        } else {
            // java this.mTextAndValues.add(curCP); this.mTextAndValues.add(curCP); this.mHasStartValue = true;
            if self.mTextAndValues.is_none() {
                self.mTextAndValues = Some(Vec::new());
            }
            let values = self.mTextAndValues.as_mut().unwrap();
            values.push(cur_cp);
            values.push(cur_cp);
            self.mHasStartValue = true;
        }
    }

    // java public void setError()
    pub fn setError(&mut self) {
        self.mState = STATE_ERROR;
    }

    // java public void commitValue()
    pub fn commitValue(&mut self) {
        self.mHasStartValue = false;
    }

    // java public void commitOP()
    pub fn commitOP(&mut self) {
        // java if (null != this.mTmpSB)
        if let Some(text) = self.mTmpSB.take() {
            if self.mOPs.is_none() {
                self.mOPs = Some(Vec::new());
            }
            self.mOPs.as_mut().unwrap().push(text);
        }
        // java this.mHasStartValue = false;
        self.mHasStartValue = false;
    }

    // java public void commitEquationResult(int curCP)
    pub fn commitEquationResult(&mut self, cur_cp: i32) {
        // java this.mEndCP = curCP + 1; this.mState = 6;
        self.mEndCP = cur_cp + 1;
        self.mState = STATE_FINISH;
    }

    // java public void commitTextResult(int curCP)
    pub fn commitTextResult(&mut self, cur_cp: i32) {
        // java this.mEndCP = curCP; this.mState = 1;
        self.mEndCP = cur_cp;
        self.mState = STATE_TEXT;
    }

    // java public int getState()
    pub fn getState(&self) -> i32 {
        self.mState
    }

    // java public boolean isError()
    pub fn isError(&self) -> bool {
        self.mState == STATE_ERROR
    }

    // java public int getEndCP()
    pub fn getEndCP(&self) -> i32 {
        self.mEndCP
    }

    // java public char getMainOP()
    pub fn getMainOP(&self) -> char {
        // java null != mOPs && size > 0 ? mOPs.get(0).toLowerCase().charAt(0) : ' '
        match self.mOPs.as_ref().and_then(|ops| ops.first()) {
            Some(op) => op.to_lowercase().chars().next().unwrap_or(' '),
            None => ' ',
        }
    }

    // java public int getTextStart()
    pub fn getTextStart(&self) -> i32 {
        match self.mTextAndValues.as_ref() {
            Some(values) if !values.is_empty() => values[0],
            _ => -1,
        }
    }

    // java public int getTextEnd()
    pub fn getTextEnd(&self) -> i32 {
        match self.mTextAndValues.as_ref() {
            Some(values) if values.len() >= 2 => values[1] + 1,
            _ => -1,
        }
    }

    // java public int getValuesSize()
    pub fn getValuesSize(&self) -> i32 {
        match self.mTextAndValues.as_ref() {
            Some(values) => (values.len() / 2) as i32,
            None => 0,
        }
    }

    // java public int getStart(int index)
    pub fn getStart(&self, index: i32) -> i32 {
        match self.mTextAndValues.as_ref() {
            Some(values) if (index as usize) < values.len() / 2 => values[index as usize * 2],
            _ => -1,
        }
    }

    // java public int getEnd(int index)
    pub fn getEnd(&self, index: i32) -> i32 {
        match self.mTextAndValues.as_ref() {
            Some(values) if (index as usize) < values.len() / 2 => values[index as usize * 2 + 1] + 1,
            _ => -1,
        }
    }

    // java public int getOPSize()
    pub fn getOPSize(&self) -> i32 {
        self.mOPs.as_ref().unwrap().len() as i32
    }

    // java public String getOP(int index)
    pub fn getOP(&self, index: i32) -> String {
        self.mOPs.as_ref().unwrap()[index as usize].clone()
    }

    // java public void addEmptyValue(int endCP)
    pub fn addEmptyValue(&mut self, end_cp: i32) {
        if self.mTextAndValues.is_none() {
            self.mTextAndValues = Some(Vec::new());
        }
        let values = self.mTextAndValues.as_mut().unwrap();
        values.push(end_cp);
        values.push(end_cp - 1);
    }

    // java public void commitCommentResult(int curCP)
    pub fn commitCommentResult(&mut self, cur_cp: i32) {
        // java this.mEndCP = curCP; this.mState = 2;
        self.mEndCP = cur_cp;
        self.mState = STATE_COMMENT;
    }
}
