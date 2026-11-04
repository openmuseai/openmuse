// Java: import java.nio.ByteBuffer
// Java: import org.mozilla.universalchardet.Constants
use crate::org::mozilla::universalchardet::Constants;
use super::CharsetProber::{self, ProbingState, filter_with_english_letters};

// Java field: byte UDF
pub(crate) const UDF: u8 = 0;
// Java field: byte OTH
pub(crate) const OTH: u8 = 1;
// Java field: byte ASC
pub(crate) const ASC: u8 = 2;
// Java field: byte ASS
pub(crate) const ASS: u8 = 3;
// Java field: byte ACV
pub(crate) const ACV: u8 = 4;
// Java field: byte ACO
pub(crate) const ACO: u8 = 5;
// Java field: byte ASV
pub(crate) const ASV: u8 = 6;
// Java field: byte ASO
pub(crate) const ASO: u8 = 7;
// Java field: int CLASS_NUM
pub(crate) const CLASS_NUM: i32 = 8;
// Java field: int FREQ_CAT_NUM
pub(crate) const FREQ_CAT_NUM: i32 = 4;

// Java: public class Latin1Prober extends CharsetProber
pub struct Latin1Prober {
    // Java field: CharsetProber.ProbingState state
    state: ProbingState,
    // Java field: byte lastCharClass
    last_char_class: u8,
    // Java field: int[] freqCounter
    freq_counter: [i32; 4],
}

// Java field: byte[] latin1CharToClass
pub(crate) const LATIN1_CHAR_TO_CLASS: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1,
    1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 1, 1, 1, 1,
    1, 0, 1, 7, 1, 1, 1, 1, 1, 1, 5, 1, 5, 0, 5, 0,
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 7, 1, 7, 0, 7, 5,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    4, 4, 4, 4, 4, 4, 5, 5, 4, 4, 4, 4, 4, 4, 4, 4,
    5, 5, 4, 4, 4, 4, 4, 1, 4, 4, 4, 4, 4, 5, 5, 5,
    6, 6, 6, 6, 6, 6, 7, 7, 6, 6, 6, 6, 6, 6, 6, 6,
    7, 7, 6, 6, 6, 6, 6, 1, 6, 6, 6, 6, 6, 7, 7, 7,
];
// Java field: byte[] latin1ClassModel
pub(crate) const LATIN1_CLASS_MODEL: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 3, 3, 3, 3, 3, 3, 3,
    0, 3, 3, 3, 3, 3, 3, 3,
    0, 3, 3, 3, 1, 1, 3, 3,
    0, 3, 3, 3, 1, 2, 1, 2,
    0, 3, 3, 3, 3, 3, 3, 3,
    0, 3, 1, 3, 1, 1, 1, 3,
    0, 3, 1, 3, 1, 1, 3, 3,
];

// Java: public class Latin1Prober extends CharsetProber
impl Latin1Prober {
    // Java: Latin1Prober()
    pub fn new() -> Self {
        // Java: super();
        let mut prober = Self {
            state: ProbingState::Detecting,
            last_char_class: OTH,
            freq_counter: [0; 4],
        };
        // Java: this.reset();
        prober.reset();
        prober
    }

    // Java: String getCharSetName()
    pub fn get_charset_name(&self) -> &str {
        // Java: return Constants.CHARSET_WINDOWS_1252;
        Constants::CHARSET_WINDOWS_1252
    }

    // Java: float getConfidence()
    pub fn get_confidence(&self) -> f32 {
        // Java: if (this.state == CharsetProber.ProbingState.NOT_ME)
        if self.state == ProbingState::NotMe {
            // Java: return 0.01F;
            return 0.01;
        }
        // Java: else
        else {
            // Java: int total = 0;
            let mut total = 0;
            // Java: for(int i = 0; i < this.freqCounter.length; ++i)
            for i in 0..self.freq_counter.len() {
                // Java: total += this.freqCounter[i];
                total += self.freq_counter[i];
            }
            // Java: float confidence;
            let mut confidence: f32;
            // Java: if (total <= 0)
            if total <= 0 {
                // Java: confidence = 0.0F;
                confidence = 0.0;
            }
            // Java: else
            else {
                // Java: confidence = (float)this.freqCounter[3] * 1.0F / (float)total;
                confidence = self.freq_counter[3] as f32 * 1.0 / total as f32;
                // Java: confidence -= (float)this.freqCounter[1] * 20.0F / (float)total;
                confidence -= self.freq_counter[1] as f32 * 20.0 / total as f32;
            }
            // Java: if (confidence < 0.0F)
            if confidence < 0.0 {
                // Java: confidence = 0.0F;
                confidence = 0.0;
            }
            // Java: confidence *= 0.5F;
            confidence *= 0.5;
            // Java: return confidence;
            return confidence;
        }
    }

    // Java: ProbingState getState()
    pub fn get_state(&self) -> ProbingState {
        // Java: return this.state;
        self.state
    }

    // Java: ProbingState handleData(byte[] buf, int offset, int length)
    pub fn handle_data(&mut self, buf: &[u8], offset: usize, length: usize) -> ProbingState {
        // Java: ByteBuffer newBufTmp = this.filterWithEnglishLetters(buf, offset, length);
        let new_buf = filter_with_english_letters(buf, offset, length);
        // Java: byte[] newBuf = newBufTmp.array();
        // Java: int newBufLen = newBufTmp.position();
        let new_buf_len = new_buf.len();
        // Java: for(int i = 0; i < newBufLen; ++i)
        for i in 0..new_buf_len {
            // Java: int c = newBuf[i] & 255;
            let c = (new_buf[i] & 0xFF) as usize;
            // Java: byte charClass = latin1CharToClass[c];
            let char_class = LATIN1_CHAR_TO_CLASS[c];
            // Java: byte freq = latin1ClassModel[this.lastCharClass * 8 + charClass];
            let freq = LATIN1_CLASS_MODEL[self.last_char_class as usize * 8 + char_class as usize];
            // Java: if (freq == 0)
            if freq == 0 {
                // Java: this.state = CharsetProber.ProbingState.NOT_ME;
                self.state = ProbingState::NotMe;
                // Java: break;
                break;
            }
            // Java: int var10002 = this.freqCounter[freq]++;
            self.freq_counter[freq as usize] += 1;
            // Java: this.lastCharClass = charClass;
            self.last_char_class = char_class;
        }
        // Java: return this.state;
        self.state
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.state = CharsetProber.ProbingState.DETECTING;
        self.state = ProbingState::Detecting;
        // Java: this.lastCharClass = 1;
        self.last_char_class = OTH;
        // Java: for(int i = 0; i < this.freqCounter.length; ++i)
        for i in 0..self.freq_counter.len() {
            // Java: this.freqCounter[i] = 0;
            self.freq_counter[i] = 0;
        }
    }

    // Java: void setOption()
    pub fn set_option(&mut self) {
    }
}

impl CharsetProber::CharsetProber for Latin1Prober {
    fn get_charset_name(&self) -> &str { self.get_charset_name() }
    fn handle_data(&mut self, buf: &[u8], offset: usize, length: usize) -> ProbingState { self.handle_data(buf, offset, length) }
    fn get_state(&self) -> ProbingState { self.get_state() }
    fn reset(&mut self) { self.reset() }
    fn get_confidence(&self) -> f32 { self.get_confidence() }
    fn set_option(&mut self) { self.set_option() }
}
