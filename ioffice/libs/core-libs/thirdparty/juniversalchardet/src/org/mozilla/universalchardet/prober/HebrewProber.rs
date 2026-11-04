use crate::org::mozilla::universalchardet::Constants;
use super::CharsetProber::ProbingState;

// Java field: int FINAL_KAF
pub(crate) const FINAL_KAF: i32 = 234;
// Java field: int NORMAL_KAF
pub(crate) const NORMAL_KAF: i32 = 235;
// Java field: int FINAL_MEM
pub(crate) const FINAL_MEM: i32 = 237;
// Java field: int NORMAL_MEM
pub(crate) const NORMAL_MEM: i32 = 238;
// Java field: int FINAL_NUN
pub(crate) const FINAL_NUN: i32 = 239;
// Java field: int NORMAL_NUN
pub(crate) const NORMAL_NUN: i32 = 240;
// Java field: int FINAL_PE
pub(crate) const FINAL_PE: i32 = 243;
// Java field: int NORMAL_PE
pub(crate) const NORMAL_PE: i32 = 244;
// Java field: int FINAL_TSADI
pub(crate) const FINAL_TSADI: i32 = 245;
// Java field: int NORMAL_TSADI
pub(crate) const NORMAL_TSADI: i32 = 246;
// Java field: byte SPACE
pub(crate) const SPACE: u8 = 32;
// Java field: int MIN_FINAL_CHAR_DISTANCE
pub(crate) const MIN_FINAL_CHAR_DISTANCE: i32 = 5;
// Java field: float MIN_MODEL_DISTANCE
pub(crate) const MIN_MODEL_DISTANCE: f32 = 0.01;

// Java: public class HebrewProber extends CharsetProber
pub struct HebrewProber {
    // Java field: int finalCharLogicalScore
    final_char_logical_score: i32,
    // Java field: int finalCharVisualScore
    final_char_visual_score: i32,
    // Java field: byte prev
    prev: u8,
    // Java field: byte beforePrev
    before_prev: u8,
    // Java field: CharsetProber logicalProber
    pub(crate) logical_prober_index: Option<usize>,
    // Java field: CharsetProber visualProber
    pub(crate) visual_prober_index: Option<usize>,
}

// Java: public class HebrewProber extends CharsetProber
impl HebrewProber {
    // Java: HebrewProber()
    pub fn new() -> Self {
        // Java: super();
        let mut prober = Self {
            final_char_logical_score: 0,
            final_char_visual_score: 0,
            prev: SPACE,
            before_prev: SPACE,
            logical_prober_index: None,
            visual_prober_index: None,
        };
        // Java: this.reset();
        prober.reset();
        prober
    }

    // Java: void setModalProbers(CharsetProber logicalProber, CharsetProber visualProber)
    pub fn set_modal_probers(&mut self, logical_index: usize, visual_index: usize) {
        // Java: this.logicalProber = logicalProber;
        self.logical_prober_index = Some(logical_index);
        // Java: this.visualProber = visualProber;
        self.visual_prober_index = Some(visual_index);
    }

    // Java: String getCharSetName()
    pub fn get_charset_name_with_probers(&self, logical_conf: f32, visual_conf: f32) -> &'static str {
        // Java: int finalsub = this.finalCharLogicalScore - this.finalCharVisualScore;
        let finalsub = self.final_char_logical_score - self.final_char_visual_score;
        // Java: if (finalsub >= 5)
        if finalsub >= MIN_FINAL_CHAR_DISTANCE {
            // Java: return Constants.CHARSET_WINDOWS_1255;
            Constants::CHARSET_WINDOWS_1255
        // Java: else if (finalsub <= -5)
        } else if finalsub <= -MIN_FINAL_CHAR_DISTANCE {
            // Java: return Constants.CHARSET_ISO_8859_8;
            Constants::CHARSET_ISO_8859_8
        // Java: else
        } else {
            // Java: float modelsub = this.logicalProber.getConfidence() - this.visualProber.getConfidence();
            let modelsub = logical_conf - visual_conf;
            // Java: if (modelsub > 0.01F)
            if modelsub > MIN_MODEL_DISTANCE {
                // Java: return Constants.CHARSET_WINDOWS_1255;
                Constants::CHARSET_WINDOWS_1255
            // Java: else if (modelsub < -0.01F)
            } else if modelsub < -MIN_MODEL_DISTANCE {
                // Java: return Constants.CHARSET_ISO_8859_8;
                Constants::CHARSET_ISO_8859_8
            // Java: else
            } else {
                // Java: return finalsub < 0 ? Constants.CHARSET_ISO_8859_8 : Constants.CHARSET_WINDOWS_1255;
                if finalsub < 0 { Constants::CHARSET_ISO_8859_8 } else { Constants::CHARSET_WINDOWS_1255 }
            }
        }
    }

    // Java: float getConfidence()
    pub fn get_confidence(&self) -> f32 {
        // Java: return 0.0F;
        0.0
    }

    // Java: CharsetProber.ProbingState getState()
    pub fn get_state_with_probers(&self, logical_state: ProbingState, visual_state: ProbingState) -> ProbingState {
        // Java: return this.logicalProber.getState() == CharsetProber.ProbingState.NOT_ME && this.visualProber.getState() == CharsetProber.ProbingState.NOT_ME ? CharsetProber.ProbingState.NOT_ME : CharsetProber.ProbingState.DETECTING;
        if logical_state == ProbingState::NotMe && visual_state == ProbingState::NotMe {
            ProbingState::NotMe
        } else {
            ProbingState::Detecting
        }
    }

    // Java: CharsetProber.ProbingState handleData(byte[] buf, int offset, int length)
    pub fn handle_data(&mut self, buf: &[u8], offset: usize, length: usize, logical_state: ProbingState, visual_state: ProbingState) -> ProbingState {
        let current_state = self.get_state_with_probers(logical_state, visual_state);
        // Java: if (this.getState() == CharsetProber.ProbingState.NOT_ME)
        if current_state == ProbingState::NotMe {
            // Java: return CharsetProber.ProbingState.NOT_ME;
            return ProbingState::NotMe;
        // Java: else
        } else {
            // Java: int maxPos = offset + length;
            let max_pos = offset + length;
            // Java: for(int i = offset; i < maxPos; ++i)
            for i in offset..max_pos {
                // Java: byte c = buf[i];
                let c = buf[i];
                // Java: if (c == 32)
                if c == SPACE {
                    // Java: if (this.beforePrev != 32)
                    if self.before_prev != SPACE {
                        // Java: if (isFinal(this.prev))
                        if Self::is_final(self.prev) {
                            // Java: ++this.finalCharLogicalScore;
                            self.final_char_logical_score += 1;
                        // Java: else if (isNonFinal(this.prev))
                        } else if Self::is_non_final(self.prev) {
                            // Java: ++this.finalCharVisualScore;
                            self.final_char_visual_score += 1;
                        }
                    }
                // Java: } else if (this.beforePrev == 32 && isFinal(this.prev) && c != 32)
                } else if self.before_prev == SPACE && Self::is_final(self.prev) && c != SPACE {
                    // Java: ++this.finalCharVisualScore;
                    self.final_char_visual_score += 1;
                }
                // Java: this.beforePrev = this.prev;
                self.before_prev = self.prev;
                // Java: this.prev = c;
                self.prev = c;
            }
            // Java: return CharsetProber.ProbingState.DETECTING;
            ProbingState::Detecting
        }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.finalCharLogicalScore = 0;
        self.final_char_logical_score = 0;
        // Java: this.finalCharVisualScore = 0;
        self.final_char_visual_score = 0;
        // Java: this.prev = 32;
        self.prev = SPACE;
        // Java: this.beforePrev = 32;
        self.before_prev = SPACE;
    }

    // Java: void setOption()
    pub fn set_option(&self) {
    }

    // Java: static boolean isFinal(byte b)
    fn is_final(b: u8) -> bool {
        // Java: int c = b & 255;
        let c = (b & 0xFF) as i32;
        // Java: return c == 234 || c == 237 || c == 239 || c == 243 || c == 245;
        c == FINAL_KAF || c == FINAL_MEM || c == FINAL_NUN || c == FINAL_PE || c == FINAL_TSADI
    }

    // Java: static boolean isNonFinal(byte b)
    fn is_non_final(b: u8) -> bool {
        // Java: int c = b & 255;
        let c = (b & 0xFF) as i32;
        // Java: return c == 235 || c == 238 || c == 240 || c == 244;
        c == NORMAL_KAF || c == NORMAL_MEM || c == NORMAL_NUN || c == NORMAL_PE
    }
}
