use crate::org::mozilla::universalchardet::prober::sequence::SequenceModel::SequenceModel;
use super::CharsetProber::ProbingState;

// Java field: int SAMPLE_SIZE
pub(crate) const SAMPLE_SIZE: i32 = 64;
// Java field: int SB_ENOUGH_REL_THRESHOLD
pub(crate) const SB_ENOUGH_REL_THRESHOLD: i32 = 1024;
// Java field: float POSITIVE_SHORTCUT_THRESHOLD
pub(crate) const POSITIVE_SHORTCUT_THRESHOLD: f32 = 0.95;
// Java field: float NEGATIVE_SHORTCUT_THRESHOLD
pub(crate) const NEGATIVE_SHORTCUT_THRESHOLD: f32 = 0.05;
// Java field: int SYMBOL_CAT_ORDER
pub(crate) const SYMBOL_CAT_ORDER: i32 = 250;
// Java field: int NUMBER_OF_SEQ_CAT
pub(crate) const NUMBER_OF_SEQ_CAT: i32 = 4;
// Java field: int POSITIVE_CAT
pub(crate) const POSITIVE_CAT: i32 = 3;
// Java field: int NEGATIVE_CAT
pub(crate) const NEGATIVE_CAT: i32 = 0;

// Java: public class SingleByteCharsetProber extends CharsetProber
pub struct SingleByteCharsetProber {
    // Java field: CharsetProber.ProbingState state
    state: ProbingState,
    // Java field: SequenceModel model
    model: SequenceModel,
    // Java field: boolean reversed
    reversed: bool,
    // Java field: short lastOrder
    last_order: i16,
    // Java field: int totalSeqs
    total_seqs: i32,
    // Java field: int[] seqCounters
    seq_counters: [i32; 4],
    // Java field: int totalChar
    total_char: i32,
    // Java field: int freqChar
    freq_char: i32,
    // Java field: CharsetProber nameProber
    pub(crate) name_prober_index: Option<usize>,
}

// Java: public class SingleByteCharsetProber extends CharsetProber
impl SingleByteCharsetProber {
    // Java: SingleByteCharsetProber(SequenceModel model)
    pub fn new(model: SequenceModel) -> Self {
        // Java: super();
        // Java: this.model = model;
        // Java: this.reversed = false;
        // Java: this.nameProber = null;
        // Java: this.seqCounters = new int[4];
        let mut prober = Self {
            state: ProbingState::Detecting,
            model,
            reversed: false,
            last_order: 255,
            total_seqs: 0,
            seq_counters: [0; 4],
            total_char: 0,
            freq_char: 0,
            name_prober_index: None,
        };
        // Java: this.reset();
        prober.reset();
        prober
    }

    // Java: SingleByteCharsetProber(SequenceModel model, boolean reversed, CharsetProber nameProber)
    pub fn new_with_name_prober(model: SequenceModel, reversed: bool, name_prober_index: usize) -> Self {
        // Java: super();
        // Java: this.model = model;
        // Java: this.reversed = reversed;
        // Java: this.nameProber = nameProber;
        // Java: this.seqCounters = new int[4];
        let mut prober = Self {
            state: ProbingState::Detecting,
            model,
            reversed,
            last_order: 255,
            total_seqs: 0,
            seq_counters: [0; 4],
            total_char: 0,
            freq_char: 0,
            name_prober_index: Some(name_prober_index),
        };
        // Java: this.reset();
        prober.reset();
        prober
    }

    // Java: boolean keepEnglishLetters()
    pub fn keep_english_letters(&self) -> bool {
        // Java: return this.model.getKeepEnglishLetter();
        self.model.get_keep_english_letter()
    }

    // Java: String getCharSetName()
    pub fn get_charset_name(&self) -> &str {
        // Java: return this.nameProber == null ? this.model.getCharsetName() : this.nameProber.getCharSetName();
        self.model.get_charset_name()
    }

    // Java: float getConfidence()
    pub fn get_confidence(&self) -> f32 {
        // Java: if (this.totalSeqs > 0)
        if self.total_seqs > 0 {
            // Java: float r = 1.0F * (float)this.seqCounters[3] / (float)this.totalSeqs / this.model.getTypicalPositiveRatio();
            let mut r = 1.0 * self.seq_counters[3] as f32 / self.total_seqs as f32 / self.model.get_typical_positive_ratio();
            // Java: r = r * (float)this.freqChar / (float)this.totalChar;
            r = r * self.freq_char as f32 / self.total_char as f32;
            // Java: if (r >= 1.0F)
            if r >= 1.0 {
                // Java: r = 0.99F;
                r = 0.99;
            }
            // Java: return r;
            r
        // Java: else
        } else {
            // Java: return 0.01F;
            0.01
        }
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
            // Java: short order = this.model.getOrder(buf[i]);
            let order = self.model.get_order(buf[i]);
            // Java: if (order < 250)
            if order < 250 {
                // Java: ++this.totalChar;
                self.total_char += 1;
            }
            // Java: if (order < 64)
            if order < 64 {
                // Java: ++this.freqChar;
                self.freq_char += 1;
                // Java: if (this.lastOrder < 64)
                if self.last_order < 64 {
                    // Java: ++this.totalSeqs;
                    self.total_seqs += 1;
                    // Java: if (!this.reversed)
                    if !self.reversed {
                        // Java: int var10002 = this.seqCounters[this.model.getPrecedence(this.lastOrder * 64 + order)]++;
                        let idx = self.model.get_precedence((self.last_order as usize) * 64 + (order as usize));
                        self.seq_counters[idx as usize] += 1;
                    // Java: else
                    } else {
                        // Java: int var8 = this.seqCounters[this.model.getPrecedence(order * 64 + this.lastOrder)]++;
                        let idx = self.model.get_precedence((order as usize) * 64 + (self.last_order as usize));
                        self.seq_counters[idx as usize] += 1;
                    }
                }
            }
            // Java: this.lastOrder = order;
            self.last_order = order;
        }
        // Java: if (this.state == CharsetProber.ProbingState.DETECTING && this.totalSeqs > 1024)
        if self.state == ProbingState::Detecting && self.total_seqs > SB_ENOUGH_REL_THRESHOLD {
            // Java: float cf = this.getConfidence();
            let cf = self.get_confidence();
            // Java: if (cf > 0.95F)
            if cf > POSITIVE_SHORTCUT_THRESHOLD {
                // Java: this.state = CharsetProber.ProbingState.FOUND_IT;
                self.state = ProbingState::FoundIt;
            // Java: else if (cf < 0.05F)
            } else if cf < NEGATIVE_SHORTCUT_THRESHOLD {
                // Java: this.state = CharsetProber.ProbingState.NOT_ME;
                self.state = ProbingState::NotMe;
            }
        }
        // Java: return this.state;
        self.state
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.state = CharsetProber.ProbingState.DETECTING;
        self.state = ProbingState::Detecting;
        // Java: this.lastOrder = 255;
        self.last_order = 255;
        // Java: for(int i = 0; i < 4; ++i)
        for i in 0..4 {
            // Java: this.seqCounters[i] = 0;
            self.seq_counters[i] = 0;
        }
        // Java: this.totalSeqs = 0;
        self.total_seqs = 0;
        // Java: this.totalChar = 0;
        self.total_char = 0;
        // Java: this.freqChar = 0;
        self.freq_char = 0;
    }

    // Java: void setOption()
    pub fn set_option(&self) {
    }
}
