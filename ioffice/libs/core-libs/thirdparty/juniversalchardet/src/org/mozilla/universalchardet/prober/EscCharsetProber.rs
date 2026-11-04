// Java: import org.mozilla.universalchardet.prober.statemachine.CodingStateMachine
use crate::org::mozilla::universalchardet::prober::statemachine::CodingStateMachine::CodingStateMachine;
// Java: import org.mozilla.universalchardet.prober.statemachine.HZSMModel
use crate::org::mozilla::universalchardet::prober::statemachine::HZSMModel::HZSMModel;
// Java: import org.mozilla.universalchardet.prober.statemachine.ISO2022CNSMModel
use crate::org::mozilla::universalchardet::prober::statemachine::ISO2022CNSMModel::ISO2022CNSMModel;
// Java: import org.mozilla.universalchardet.prober.statemachine.ISO2022JPSMModel
use crate::org::mozilla::universalchardet::prober::statemachine::ISO2022JPSMModel::ISO2022JPSMModel;
// Java: import org.mozilla.universalchardet.prober.statemachine.ISO2022KRSMModel
use crate::org::mozilla::universalchardet::prober::statemachine::ISO2022KRSMModel::ISO2022KRSMModel;
use super::CharsetProber::ProbingState;

// Java: public class EscCharsetProber extends CharsetProber
pub struct EscCharsetProber {
    // Java field: CodingStateMachine[] codingSM
    coding_sm: Vec<CodingStateMachine>,
    // Java field: int activeSM
    active_sm: i32,
    // Java field: CharsetProber.ProbingState state
    state: ProbingState,
    // Java field: String detectedCharset
    pub(crate) detected_charset: Option<String>,
}

// Java field: HZSMModel hzsModel
// Java field: ISO2022CNSMModel iso2022cnModel
// Java field: ISO2022JPSMModel iso2022jpModel
// Java field: ISO2022KRSMModel iso2022krModel

// Java: public class EscCharsetProber extends CharsetProber
impl EscCharsetProber {
    // Java: EscCharsetProber()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.codingSM[0] = new CodingStateMachine(hzsModel);
        // Java: this.codingSM[1] = new CodingStateMachine(iso2022cnModel);
        // Java: this.codingSM[2] = new CodingStateMachine(iso2022jpModel);
        // Java: this.codingSM[3] = new CodingStateMachine(iso2022krModel);
        let coding_sm = vec![
            CodingStateMachine::new(HZSMModel::new()),
            CodingStateMachine::new(ISO2022CNSMModel::new()),
            CodingStateMachine::new(ISO2022JPSMModel::new()),
            CodingStateMachine::new(ISO2022KRSMModel::new()),
        ];
        let active_sm = coding_sm.len() as i32;
        let mut prober = Self {
            coding_sm,
            active_sm,
            state: ProbingState::Detecting,
            detected_charset: None,
        };
        // Java: this.reset();
        prober.reset();
        prober
    }

    // Java: String getCharSetName()
    pub fn get_charset_name(&self) -> &str {
        // Java: return this.detectedCharset;
        self.detected_charset.as_deref().unwrap_or("")
    }

    // Java: float getConfidence()
    pub fn get_confidence(&self) -> f32 {
        // Java: return 0.99F;
        0.99
    }

    // Java: CharsetProber.ProbingState getState()
    pub fn get_state(&self) -> ProbingState {
        // Java: return this.state;
        self.state
    }

    // Java: CharsetProber.ProbingState handleData(byte[] buf, int offset, int length)
    pub fn handle_data(&mut self, buf: &[u8], offset: usize, length: usize) -> ProbingState {
        // Java: int maxPos = offset + length;
        let max_pos = offset + length;
        // Java: for(int i = offset; i < maxPos && this.state == CharsetProber.ProbingState.DETECTING; ++i)
        let mut i = offset;
        while i < max_pos && self.state == ProbingState::Detecting {
            // Java: for(int j = this.activeSM - 1; j >= 0; --j)
            let mut j = self.active_sm - 1;
            while j >= 0 {
                // Java: int codingState = this.codingSM[j].nextState(buf[i]);
                let coding_state = self.coding_sm[j as usize].next_state(buf[i]);
                // Java: if (codingState == 1)
                if coding_state == 1 {
                    // Java: --this.activeSM;
                    self.active_sm -= 1;
                    // Java: if (this.activeSM <= 0)
                    if self.active_sm <= 0 {
                        // Java: this.state = CharsetProber.ProbingState.NOT_ME;
                        self.state = ProbingState::NotMe;
                        // Java: return this.state;
                        return self.state;
                    }
                    // Java: if (j != this.activeSM)
                    if j != self.active_sm {
                        // Java: CodingStateMachine t = this.codingSM[this.activeSM];
                        // Java: this.codingSM[this.activeSM] = this.codingSM[j];
                        // Java: this.codingSM[j] = t;
                        self.coding_sm.swap(self.active_sm as usize, j as usize);
                    }
                // Java: } else if (codingState == 2)
                } else if coding_state == 2 {
                    // Java: this.state = CharsetProber.ProbingState.FOUND_IT;
                    self.state = ProbingState::FoundIt;
                    // Java: this.detectedCharset = this.codingSM[j].getCodingStateMachine();
                    self.detected_charset = Some(self.coding_sm[j as usize].get_coding_state_machine().to_string());
                    // Java: return this.state;
                    return self.state;
                }
                j -= 1;
            }
            i += 1;
        }
        // Java: return this.state;
        self.state
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.state = CharsetProber.ProbingState.DETECTING;
        self.state = ProbingState::Detecting;
        // Java: for(int i = 0; i < this.codingSM.length; ++i)
        for i in 0..self.coding_sm.len() {
            // Java: this.codingSM[i].reset();
            self.coding_sm[i].reset();
        }
        // Java: this.activeSM = this.codingSM.length;
        self.active_sm = self.coding_sm.len() as i32;
        // Java: this.detectedCharset = null;
        self.detected_charset = None;
    }

    // Java: void setOption()
    pub fn set_option(&self) {
    }
}
