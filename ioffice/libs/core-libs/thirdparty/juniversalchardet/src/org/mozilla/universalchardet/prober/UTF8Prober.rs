// Java: import org.mozilla.universalchardet.Constants
use crate::org::mozilla::universalchardet::Constants;
// Java: import org.mozilla.universalchardet.prober.statemachine.CodingStateMachine
use crate::org::mozilla::universalchardet::prober::statemachine::CodingStateMachine::CodingStateMachine;
// Java: import org.mozilla.universalchardet.prober.statemachine.UTF8SMModel
use crate::org::mozilla::universalchardet::prober::statemachine::UTF8SMModel::UTF8SMModel;
// Java: import org.mozilla.universalchardet.prober.statemachine.SMModel
use super::CharsetProber::ProbingState;

// Java field: float ONE_CHAR_PROB
pub(crate) const ONE_CHAR_PROB: f32 = 0.5;

// Java: public class UTF8Prober extends CharsetProber
pub struct UTF8Prober {
    // Java field: CodingStateMachine codingSM
    coding_sm: CodingStateMachine,
    // Java field: CharsetProber.ProbingState state
    state: ProbingState,
    // Java field: int numOfMBChar
    num_of_mb_char: i32,
}

// Java field: SMModel smModel

// Java: public class UTF8Prober extends CharsetProber
impl UTF8Prober {
    // Java: UTF8Prober()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.codingSM = new CodingStateMachine(smModel);
        let mut prober = Self {
            coding_sm: CodingStateMachine::new(UTF8SMModel::new()),
            state: ProbingState::Detecting,
            num_of_mb_char: 0,
        };
        // Java: this.reset();
        prober.reset();
        prober
    }

    // Java: String getCharSetName()
    pub fn get_charset_name(&self) -> &str {
        // Java: return Constants.CHARSET_UTF_8;
        Constants::CHARSET_UTF_8
    }

    // Java: CharsetProber.ProbingState handleData(byte[] buf, int offset, int length)
    pub fn handle_data(&mut self, buf: &[u8], offset: usize, length: usize) -> ProbingState {
        // Java: int maxPos = offset + length;
        let max_pos = offset + length;
        // Java: for(int i = offset; i < maxPos; ++i)
        for i in offset..max_pos {
            // Java: int codingState = this.codingSM.nextState(buf[i]);
            let coding_state = self.coding_sm.next_state(buf[i]);
            // Java: if (codingState == 1)
            if coding_state == 1 {
                // Java: this.state = CharsetProber.ProbingState.NOT_ME;
                self.state = ProbingState::NotMe;
                // Java: break;
                break;
            }
            // Java: if (codingState == 2)
            if coding_state == 2 {
                // Java: this.state = CharsetProber.ProbingState.FOUND_IT;
                self.state = ProbingState::FoundIt;
                // Java: break;
                break;
            }
            // Java: if (codingState == 0 && this.codingSM.getCurrentCharLen() >= 2)
            if coding_state == 0 && self.coding_sm.get_current_char_len() >= 2 {
                // Java: ++this.numOfMBChar;
                self.num_of_mb_char += 1;
            }
        }
        // Java: if (this.state == CharsetProber.ProbingState.DETECTING && this.getConfidence() > 0.95F)
        if self.state == ProbingState::Detecting && self.get_confidence() > 0.95 {
            // Java: this.state = CharsetProber.ProbingState.FOUND_IT;
            self.state = ProbingState::FoundIt;
        }
        // Java: return this.state;
        self.state
    }

    // Java: CharsetProber.ProbingState getState()
    pub fn get_state(&self) -> ProbingState {
        // Java: return this.state;
        self.state
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.codingSM.reset();
        self.coding_sm.reset();
        // Java: this.numOfMBChar = 0;
        self.num_of_mb_char = 0;
        // Java: this.state = CharsetProber.ProbingState.DETECTING;
        self.state = ProbingState::Detecting;
    }

    // Java: float getConfidence()
    pub fn get_confidence(&self) -> f32 {
        // Java: float unlike = 0.99F;
        let mut unlike: f32 = 0.99;
        // Java: if (this.numOfMBChar >= 6)
        if self.num_of_mb_char >= 6 {
            // Java: return 0.99F;
            return 0.99;
        // Java: else
        } else {
            // Java: for(int i = 0; i < this.numOfMBChar; ++i)
            for _i in 0..self.num_of_mb_char {
                // Java: unlike *= 0.5F;
                unlike *= ONE_CHAR_PROB;
            }
            // Java: return 1.0F - unlike;
            return 1.0 - unlike;
        }
    }

    // Java: void setOption()
    pub fn set_option(&self) {
    }
}
