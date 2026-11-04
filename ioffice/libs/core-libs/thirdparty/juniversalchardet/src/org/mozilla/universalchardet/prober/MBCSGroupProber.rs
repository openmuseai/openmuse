use super::CharsetProber::{self, ProbingState};

// Java: public class MBCSGroupProber extends CharsetProber
pub struct MBCSGroupProber {
    // Java field: CharsetProber.ProbingState state
    state: ProbingState,
    // Java field: CharsetProber[] probers
    pub(crate) probers: Vec<Box<dyn CharsetProber::CharsetProber>>,
    // Java field: boolean[] isActive
    is_active: Vec<bool>,
    // Java field: int bestGuess
    best_guess: i32,
    // Java field: int activeNum
    active_num: i32,
}

// Java: public class MBCSGroupProber extends CharsetProber
impl MBCSGroupProber {
    // Java: MBCSGroupProber()
    pub fn new() -> Self {
        // Java: super();
        let mut prober = Self {
            state: ProbingState::Detecting,
            probers: Vec::new(),
            is_active: Vec::new(),
            best_guess: -1,
            active_num: 0,
        };
        // Java: this.probers[0] = new GB18030Prober();
        // Java: this.probers[1] = new UTF8Prober();
        // Java: this.probers[2] = new Big5Prober();
        // Java: this.probers[3] = new SJISProber();
        // Java: this.probers[4] = new EUCJPProber();
        // Java: this.probers[5] = new EUCKRProber();
        // Java: this.probers[6] = new EUCTWProber();
        // TODO: probers will be initialized when concrete prober types are available
        // Java: this.reset();
        prober.reset();
        prober
    }

    // Java: String getCharSetName()
    pub fn get_charset_name(&self) -> &str {
        // Java: if (this.bestGuess == -1)
        if self.best_guess == -1 {
            // Java: this.getConfidence();
            // Note: getConfidence is called for side-effect of setting bestGuess
            // Java: if (this.bestGuess == -1)
            // Java: this.bestGuess = 0;
        }
        let idx = if self.best_guess < 0 { 0 } else { self.best_guess as usize };
        // Java: return this.probers[this.bestGuess].getCharSetName();
        self.probers[idx].get_charset_name()
    }

    // Java: float getConfidence()
    pub fn get_confidence(&mut self) -> f32 {
        // Java: float bestConf = 0.0F;
        let mut best_conf = 0.0f32;
        // Java: if (this.state == CharsetProber.ProbingState.FOUND_IT)
        if self.state == ProbingState::FoundIt {
            // Java: return 0.99F;
            return 0.99;
        }
        // Java: else if (this.state == CharsetProber.ProbingState.NOT_ME)
        else if self.state == ProbingState::NotMe {
            // Java: return 0.01F;
            return 0.01;
        }
        // Java: else
        else {
            // Java: for(int i = 0; i < this.probers.length; ++i)
            for i in 0..self.probers.len() {
                // Java: if (this.isActive[i])
                if self.is_active[i] {
                    // Java: float cf = this.probers[i].getConfidence();
                    let cf = self.probers[i].get_confidence();
                    // Java: if (bestConf < cf)
                    if best_conf < cf {
                        // Java: bestConf = cf;
                        best_conf = cf;
                        // Java: this.bestGuess = i;
                        self.best_guess = i as i32;
                    }
                }
            }
            // Java: return bestConf;
            return best_conf;
        }
    }

    // Java: ProbingState getState()
    pub fn get_state(&self) -> ProbingState {
        // Java: return this.state;
        self.state
    }

    // Java: ProbingState handleData(byte[] buf, int offset, int length)
    pub fn handle_data(&mut self, buf: &[u8], offset: usize, length: usize) -> ProbingState {
        // Java: boolean keepNext = true;
        let mut keep_next = true;
        // Java: byte[] highbyteBuf = new byte[length];
        let mut highbyte_buf = vec![0u8; length];
        // Java: int highpos = 0;
        let mut highpos = 0usize;
        // Java: int maxPos = offset + length;
        let max_pos = offset + length;
        // Java: for(int i = offset; i < maxPos; ++i)
        for i in offset..max_pos {
            // Java: if ((buf[i] & 128) != 0)
            if (buf[i] & 128) != 0 {
                // Java: highbyteBuf[highpos++] = buf[i];
                highbyte_buf[highpos] = buf[i];
                highpos += 1;
                // Java: keepNext = true;
                keep_next = true;
            }
            // Java: else if (keepNext)
            else if keep_next {
                // Java: highbyteBuf[highpos++] = buf[i];
                highbyte_buf[highpos] = buf[i];
                highpos += 1;
                // Java: keepNext = false;
                keep_next = false;
            }
        }
        // Java: for(int i = 0; i < this.probers.length; ++i)
        for i in 0..self.probers.len() {
            // Java: if (this.isActive[i])
            if self.is_active[i] {
                // Java: CharsetProber.ProbingState st = this.probers[i].handleData(highbyteBuf, 0, highpos);
                let st = self.probers[i].handle_data(&highbyte_buf, 0, highpos);
                // Java: if (st == CharsetProber.ProbingState.FOUND_IT || 0.99F == this.probers[i].getConfidence())
                if st == ProbingState::FoundIt || self.probers[i].get_confidence() == 0.99 {
                    // Java: this.bestGuess = i;
                    self.best_guess = i as i32;
                    // Java: this.state = CharsetProber.ProbingState.FOUND_IT;
                    self.state = ProbingState::FoundIt;
                    // Java: break;
                    break;
                }
                // Java: if (st == CharsetProber.ProbingState.NOT_ME)
                if st == ProbingState::NotMe {
                    // Java: this.isActive[i] = false;
                    self.is_active[i] = false;
                    // Java: --this.activeNum;
                    self.active_num -= 1;
                    // Java: if (this.activeNum <= 0)
                    if self.active_num <= 0 {
                        // Java: this.state = CharsetProber.ProbingState.NOT_ME;
                        self.state = ProbingState::NotMe;
                        // Java: break;
                        break;
                    }
                }
            }
        }
        // Java: return this.state;
        self.state
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.activeNum = 0;
        self.active_num = 0;
        // Java: for(int i = 0; i < this.probers.length; ++i)
        for i in 0..self.probers.len() {
            // Java: this.probers[i].reset();
            self.probers[i].reset();
            // Java: this.isActive[i] = true;
            self.is_active[i] = true;
            // Java: ++this.activeNum;
            self.active_num += 1;
        }
        // Java: this.bestGuess = -1;
        self.best_guess = -1;
        // Java: this.state = CharsetProber.ProbingState.DETECTING;
        self.state = ProbingState::Detecting;
    }

    // Java: void setOption()
    pub fn set_option(&mut self) {
    }
}
