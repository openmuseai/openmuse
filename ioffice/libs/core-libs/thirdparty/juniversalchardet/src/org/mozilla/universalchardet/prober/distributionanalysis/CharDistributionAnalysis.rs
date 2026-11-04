// Java field: float SURE_NO
pub(crate) const SURE_NO: f32 = 0.01;
// Java field: float SURE_YES
pub(crate) const SURE_YES: f32 = 0.99;
// Java field: int ENOUGH_DATA_THRESHOLD
pub(crate) const ENOUGH_DATA_THRESHOLD: i32 = 1024;
// Java field: int MINIMUM_DATA_THRESHOLD
pub(crate) const MINIMUM_DATA_THRESHOLD: i32 = 4;

// Java: public class CharDistributionAnalysis
pub struct CharDistributionAnalysis {
    // Java field: int freqChars
    freq_chars: i32,
    // Java field: int totalChars
    pub(crate) total_chars: i32,
    // Java field: int[] charToFreqOrder
    pub(crate) char_to_freq_order: &'static [i32],
    // Java field: float typicalDistributionRatio
    pub(crate) typical_distribution_ratio: f32,
    // Java field: boolean done
    pub(crate) done: bool,
    pub(crate) get_order_fn: fn(&[u8], usize) -> i32,
}

// Java: public class CharDistributionAnalysis
impl CharDistributionAnalysis {
    // Java: CharDistributionAnalysis()
    pub fn new(char_to_freq_order: &'static [i32], typical_distribution_ratio: f32, get_order_fn: fn(&[u8], usize) -> i32) -> Self {
        // Java: super();
        let mut analysis = Self {
            freq_chars: 0,
            total_chars: 0,
            char_to_freq_order,
            typical_distribution_ratio,
            done: false,
            get_order_fn,
        };
        // Java: this.reset();
        analysis.reset();
        analysis
    }

    // Java: void handleData(byte[] buf, int offset, int length)
    pub fn handle_data(&self, _buf: &[u8], _offset: usize, _length: usize) {
    }

    // Java: void handleOneChar(byte[] buf, int offset, int charLength)
    pub fn handle_one_char(&mut self, buf: &[u8], offset: usize, char_length: i32) {
        // Java: int order = -1;
        let mut order: i32 = -1;
        // Java: if (charLength == 2)
        if char_length == 2 {
            // Java: order = this.getOrder(buf, offset);
            order = (self.get_order_fn)(buf, offset);
        }
        // Java: if (order >= 0)
        if order >= 0 {
            // Java: ++this.totalChars;
            self.total_chars += 1;
            // Java: if (order < this.charToFreqOrder.length && 512 > this.charToFreqOrder[order])
            if (order as usize) < self.char_to_freq_order.len() && 512 > self.char_to_freq_order[order as usize] {
                // Java: ++this.freqChars;
                self.freq_chars += 1;
            }
        }
    }

    // Java: float getConfidence()
    pub fn get_confidence(&self) -> f32 {
        // Java: if (this.totalChars > 0 && this.freqChars > 4)
        if self.total_chars > 0 && self.freq_chars > MINIMUM_DATA_THRESHOLD {
            // Java: if (this.totalChars != this.freqChars)
            if self.total_chars != self.freq_chars {
                // Java: float r = (float)this.freqChars / ((float)(this.totalChars - this.freqChars) * this.typicalDistributionRatio);
                let r = self.freq_chars as f32 / ((self.total_chars - self.freq_chars) as f32 * self.typical_distribution_ratio);
                // Java: if (r < 0.99F)
                if r < SURE_YES {
                    // Java: return r;
                    return r;
                }
            }
            // Java: return 0.99F;
            SURE_YES
        // Java: else
        } else {
            // Java: return 0.01F;
            SURE_NO
        }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.done = false;
        self.done = false;
        // Java: this.totalChars = 0;
        self.total_chars = 0;
        // Java: this.freqChars = 0;
        self.freq_chars = 0;
    }

    // Java: void setOption()
    pub fn set_option(&self) {
    }

    // Java: boolean gotEnoughData()
    pub fn got_enough_data(&self) -> bool {
        // Java: return this.totalChars > 1024;
        self.total_chars > ENOUGH_DATA_THRESHOLD
    }

    // Java: abstract int getOrder(byte[] var1, int var2)
    pub fn get_order(&self, buf: &[u8], offset: usize) -> i32 {
        (self.get_order_fn)(buf, offset)
    }
}
