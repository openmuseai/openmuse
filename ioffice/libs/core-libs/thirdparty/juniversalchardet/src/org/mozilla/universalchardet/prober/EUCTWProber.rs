// Java: import java.util.Arrays
// Java: import org.mozilla.universalchardet.Constants
use crate::org::mozilla::universalchardet::Constants;
// Java: import org.mozilla.universalchardet.prober.distributionanalysis.EUCTWDistributionAnalysis
use crate::org::mozilla::universalchardet::prober::distributionanalysis::EUCTWDistributionAnalysis::EUCTWDistributionAnalysis;
// Java: import org.mozilla.universalchardet.prober.statemachine.CodingStateMachine
use crate::org::mozilla::universalchardet::prober::statemachine::CodingStateMachine::CodingStateMachine;
// Java: import org.mozilla.universalchardet.prober.statemachine.EUCTWSMModel
use crate::org::mozilla::universalchardet::prober::statemachine::EUCTWSMModel::EUCTWSMModel;
// Java: import org.mozilla.universalchardet.prober.statemachine.SMModel
use super::CharsetProber::ProbingState;
use crate::org::mozilla::universalchardet::prober::distributionanalysis::CharDistributionAnalysis::CharDistributionAnalysis;

// Java: public class EUCTWProber extends CharsetProber
pub struct EUCTWProber {
    // Java field: CodingStateMachine codingSM
    coding_sm: CodingStateMachine,
    // Java field: CharsetProber.ProbingState state
    state: ProbingState,
    // Java field: EUCTWDistributionAnalysis distributionAnalyzer
    distribution_analyzer: CharDistributionAnalysis,
    // Java field: byte[] lastChar
    last_char: [u8; 2],
}

// Java field: SMModel smModel

// Java: public class EUCTWProber extends CharsetProber
impl EUCTWProber {
    // Java: EUCTWProber()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.codingSM = new CodingStateMachine(smModel);
        // Java: this.distributionAnalyzer = new EUCTWDistributionAnalysis();
        // Java: this.lastChar = new byte[2];
        let mut prober = Self {
            coding_sm: CodingStateMachine::new(EUCTWSMModel::new()),
            state: ProbingState::Detecting,
            distribution_analyzer: EUCTWDistributionAnalysis::new(),
            last_char: [0; 2],
        };
        // Java: this.reset();
        prober.reset();
        prober
    }

    // Java: String getCharSetName()
    pub fn get_charset_name(&self) -> &str {
        // Java: return Constants.CHARSET_EUC_TW;
        Constants::CHARSET_EUC_TW
    }

    // Java: float getConfidence()
    pub fn get_confidence(&self) -> f32 {
        // Java: float distribCf = this.distributionAnalyzer.getConfidence();
        let distrib_cf = self.distribution_analyzer.get_confidence();
        // Java: return distribCf;
        distrib_cf
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
            // Java: if (codingState == 0)
            if coding_state == 0 {
                // Java: int charLen = this.codingSM.getCurrentCharLen();
                let char_len = self.coding_sm.get_current_char_len();
                // Java: if (i == offset)
                if i == offset {
                    // Java: this.lastChar[1] = buf[offset];
                    self.last_char[1] = buf[offset];
                    // Java: this.distributionAnalyzer.handleOneChar(this.lastChar, 0, charLen);
                    self.distribution_analyzer.handle_one_char(&self.last_char.clone(), 0, char_len);
                // Java: else
                } else {
                    // Java: this.distributionAnalyzer.handleOneChar(buf, i - 1, charLen);
                    self.distribution_analyzer.handle_one_char(buf, i - 1, char_len);
                }
            }
        }
        // Java: this.lastChar[0] = buf[maxPos - 1];
        self.last_char[0] = buf[max_pos - 1];
        // Java: if (this.state == CharsetProber.ProbingState.DETECTING && this.distributionAnalyzer.gotEnoughData() && this.getConfidence() > 0.95F)
        if self.state == ProbingState::Detecting && self.distribution_analyzer.got_enough_data() && self.get_confidence() > 0.95 {
            // Java: this.state = CharsetProber.ProbingState.FOUND_IT;
            self.state = ProbingState::FoundIt;
        }
        // Java: return this.state;
        self.state
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.codingSM.reset();
        self.coding_sm.reset();
        // Java: this.state = CharsetProber.ProbingState.DETECTING;
        self.state = ProbingState::Detecting;
        // Java: this.distributionAnalyzer.reset();
        self.distribution_analyzer.reset();
        // Java: Arrays.fill(this.lastChar, (byte)0);
        self.last_char = [0; 2];
    }

    // Java: void setOption()
    pub fn set_option(&self) {
    }
}
