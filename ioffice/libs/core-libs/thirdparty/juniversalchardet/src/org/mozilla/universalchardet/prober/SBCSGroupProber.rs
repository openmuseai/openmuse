// Java: import java.nio.ByteBuffer
// Java: import org.mozilla.universalchardet.prober.sequence.HebrewModel
use crate::org::mozilla::universalchardet::prober::sequence::HebrewModel::HebrewModel;
// Java: import org.mozilla.universalchardet.prober.sequence.Ibm855Model
use crate::org::mozilla::universalchardet::prober::sequence::Ibm855Model::Ibm855Model;
// Java: import org.mozilla.universalchardet.prober.sequence.Ibm866Model
use crate::org::mozilla::universalchardet::prober::sequence::Ibm866Model::Ibm866Model;
// Java: import org.mozilla.universalchardet.prober.sequence.Koi8rModel
use crate::org::mozilla::universalchardet::prober::sequence::Koi8rModel::Koi8rModel;
// Java: import org.mozilla.universalchardet.prober.sequence.Latin5BulgarianModel
use crate::org::mozilla::universalchardet::prober::sequence::Latin5BulgarianModel::Latin5BulgarianModel;
// Java: import org.mozilla.universalchardet.prober.sequence.Latin5Model
use crate::org::mozilla::universalchardet::prober::sequence::Latin5Model::Latin5Model;
// Java: import org.mozilla.universalchardet.prober.sequence.Latin7Model
use crate::org::mozilla::universalchardet::prober::sequence::Latin7Model::Latin7Model;
// Java: import org.mozilla.universalchardet.prober.sequence.MacCyrillicModel
use crate::org::mozilla::universalchardet::prober::sequence::MacCyrillicModel::MacCyrillicModel;
// Java: import org.mozilla.universalchardet.prober.sequence.SequenceModel
// Java: import org.mozilla.universalchardet.prober.sequence.Win1251BulgarianModel
use crate::org::mozilla::universalchardet::prober::sequence::Win1251BulgarianModel::Win1251BulgarianModel;
// Java: import org.mozilla.universalchardet.prober.sequence.Win1251Model
use crate::org::mozilla::universalchardet::prober::sequence::Win1251Model::Win1251Model;
// Java: import org.mozilla.universalchardet.prober.sequence.Win1253Model
use crate::org::mozilla::universalchardet::prober::sequence::Win1253Model::Win1253Model;
use super::CharsetProber::{self, ProbingState};
use super::SingleByteCharsetProber::SingleByteCharsetProber;
use super::HebrewProber::HebrewProber;

enum ProberKind {
    SingleByte(SingleByteCharsetProber),
    Hebrew(HebrewProber),
}

impl ProberKind {
    fn get_confidence(&self) -> f32 {
        match self {
            ProberKind::SingleByte(p) => p.get_confidence(),
            ProberKind::Hebrew(p) => p.get_confidence(),
        }
    }
    fn get_state(&self) -> ProbingState {
        match self {
            ProberKind::SingleByte(p) => p.get_state(),
            ProberKind::Hebrew(_) => ProbingState::Detecting,
        }
    }
    fn reset(&mut self) {
        match self {
            ProberKind::SingleByte(p) => p.reset(),
            ProberKind::Hebrew(p) => p.reset(),
        }
    }
}

// Java: public class SBCSGroupProber extends CharsetProber
pub struct SBCSGroupProber {
    // Java field: CharsetProber.ProbingState state
    state: ProbingState,
    // Java field: CharsetProber[] probers
    pub(crate) probers: Vec<ProberKind>,
    // Java field: boolean[] isActive
    is_active: Vec<bool>,
    // Java field: int bestGuess
    best_guess: i32,
    // Java field: int activeNum
    active_num: i32,
}

// Java field: SequenceModel win1251Model
// Java field: SequenceModel koi8rModel
// Java field: SequenceModel latin5Model
// Java field: SequenceModel macCyrillicModel
// Java field: SequenceModel ibm866Model
// Java field: SequenceModel ibm855Model
// Java field: SequenceModel latin7Model
// Java field: SequenceModel win1253Model
// Java field: SequenceModel latin5BulgarianModel
// Java field: SequenceModel win1251BulgarianModel
// Java field: SequenceModel hebrewModel

// Java: public class SBCSGroupProber extends CharsetProber
impl SBCSGroupProber {
    // Java: SBCSGroupProber()
    pub fn new() -> Self {
        // Java: super();
        let mut probers = Vec::with_capacity(13);
        // Java: this.probers[0] = new SingleByteCharsetProber(win1251Model);
        probers.push(ProberKind::SingleByte(SingleByteCharsetProber::new(Win1251Model::new())));
        // Java: this.probers[1] = new SingleByteCharsetProber(koi8rModel);
        probers.push(ProberKind::SingleByte(SingleByteCharsetProber::new(Koi8rModel::new())));
        // Java: this.probers[2] = new SingleByteCharsetProber(latin5Model);
        probers.push(ProberKind::SingleByte(SingleByteCharsetProber::new(Latin5Model::new())));
        // Java: this.probers[3] = new SingleByteCharsetProber(macCyrillicModel);
        probers.push(ProberKind::SingleByte(SingleByteCharsetProber::new(MacCyrillicModel::new())));
        // Java: this.probers[4] = new SingleByteCharsetProber(ibm866Model);
        probers.push(ProberKind::SingleByte(SingleByteCharsetProber::new(Ibm866Model::new())));
        // Java: this.probers[5] = new SingleByteCharsetProber(ibm855Model);
        probers.push(ProberKind::SingleByte(SingleByteCharsetProber::new(Ibm855Model::new())));
        // Java: this.probers[6] = new SingleByteCharsetProber(latin7Model);
        probers.push(ProberKind::SingleByte(SingleByteCharsetProber::new(Latin7Model::new())));
        // Java: this.probers[7] = new SingleByteCharsetProber(win1253Model);
        probers.push(ProberKind::SingleByte(SingleByteCharsetProber::new(Win1253Model::new())));
        // Java: this.probers[8] = new SingleByteCharsetProber(latin5BulgarianModel);
        probers.push(ProberKind::SingleByte(SingleByteCharsetProber::new(Latin5BulgarianModel::new())));
        // Java: this.probers[9] = new SingleByteCharsetProber(win1251BulgarianModel);
        probers.push(ProberKind::SingleByte(SingleByteCharsetProber::new(Win1251BulgarianModel::new())));
        // Java: HebrewProber hebprober = new HebrewProber();
        let hebprober = HebrewProber::new();
        // Java: this.probers[10] = hebprober;
        probers.push(ProberKind::Hebrew(hebprober));
        // Java: this.probers[11] = new SingleByteCharsetProber(hebrewModel, false, hebprober);
        probers.push(ProberKind::SingleByte(SingleByteCharsetProber::new_with_name_prober(HebrewModel::new(), false, 10)));
        // Java: this.probers[12] = new SingleByteCharsetProber(hebrewModel, true, hebprober);
        probers.push(ProberKind::SingleByte(SingleByteCharsetProber::new_with_name_prober(HebrewModel::new(), true, 10)));
        // Java: hebprober.setModalProbers(this.probers[11], this.probers[12]);
        if let ProberKind::Hebrew(ref mut hp) = probers[10] {
            hp.set_modal_probers(11, 12);
        }

        let is_active = vec![true; probers.len()];
        let active_num = probers.len() as i32;
        let mut prober = Self {
            state: ProbingState::Detecting,
            probers,
            is_active,
            best_guess: -1,
            active_num,
        };
        // Java: this.reset();
        prober.reset();
        prober
    }

    fn prober_get_charset_name(&self, index: usize) -> &str {
        match &self.probers[index] {
            ProberKind::SingleByte(p) => {
                if let Some(name_idx) = p.name_prober_index {
                    if let ProberKind::Hebrew(hp) = &self.probers[name_idx] {
                        let logical_conf = self.probers[hp.logical_prober_index.unwrap_or(11)].get_confidence();
                        let visual_conf = self.probers[hp.visual_prober_index.unwrap_or(12)].get_confidence();
                        return hp.get_charset_name_with_probers(logical_conf, visual_conf);
                    }
                }
                p.get_charset_name()
            }
            ProberKind::Hebrew(p) => {
                let logical_conf = self.probers[p.logical_prober_index.unwrap_or(11)].get_confidence();
                let visual_conf = self.probers[p.visual_prober_index.unwrap_or(12)].get_confidence();
                p.get_charset_name_with_probers(logical_conf, visual_conf)
            }
        }
    }

    // Java: String getCharSetName()
    pub fn get_charset_name(&mut self) -> &str {
        // Java: if (this.bestGuess == -1)
        if self.best_guess == -1 {
            // Java: this.getConfidence();
            self.get_confidence();
            // Java: if (this.bestGuess == -1)
            if self.best_guess == -1 {
                // Java: this.bestGuess = 0;
                self.best_guess = 0;
            }
        }
        // Java: return this.probers[this.bestGuess].getCharSetName();
        self.prober_get_charset_name(self.best_guess as usize)
    }

    // Java: float getConfidence()
    pub fn get_confidence(&mut self) -> f32 {
        // Java: float bestConf = 0.0F;
        let mut best_conf: f32 = 0.0;
        // Java: if (this.state == CharsetProber.ProbingState.FOUND_IT)
        if self.state == ProbingState::FoundIt {
            // Java: return 0.99F;
            return 0.99;
        // Java: else if (this.state == CharsetProber.ProbingState.NOT_ME)
        } else if self.state == ProbingState::NotMe {
            // Java: return 0.01F;
            return 0.01;
        // Java: else
        } else {
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
            best_conf
        }
    }

    // Java: CharsetProber.ProbingState getState()
    pub fn get_state(&self) -> ProbingState {
        // Java: return this.state;
        self.state
    }

    // Java: CharsetProber.ProbingState handleData(byte[] buf, int offset, int length)
    pub fn handle_data(&mut self, buf: &[u8], offset: usize, length: usize) -> ProbingState {
        // Java: ByteBuffer newbuf = this.filterWithoutEnglishLetters(buf, offset, length);
        let newbuf = CharsetProber::filter_without_english_letters(buf, offset, length);
        // Java: if (newbuf.position() != 0)
        if !newbuf.is_empty() {
            // Java: for(int i = 0; i < this.probers.length; ++i)
            for i in 0..self.probers.len() {
                // Java: if (this.isActive[i])
                if self.is_active[i] {
                    let st = {
                        let is_hebrew = matches!(&self.probers[i], ProberKind::Hebrew(_));
                        if is_hebrew {
                            let (logical_idx, visual_idx) = match &self.probers[i] {
                                ProberKind::Hebrew(p) => (p.logical_prober_index.unwrap_or(11), p.visual_prober_index.unwrap_or(12)),
                                _ => unreachable!(),
                            };
                            let logical_st = self.probers[logical_idx].get_state();
                            let visual_st = self.probers[visual_idx].get_state();
                            match &mut self.probers[i] {
                                ProberKind::Hebrew(p) => p.handle_data(&newbuf, 0, newbuf.len(), logical_st, visual_st),
                                _ => unreachable!(),
                            }
                        } else {
                            // Java: CharsetProber.ProbingState st = this.probers[i].handleData(newbuf.array(), 0, newbuf.position());
                            match &mut self.probers[i] {
                                ProberKind::SingleByte(p) => p.handle_data(&newbuf, 0, newbuf.len()),
                                _ => unreachable!(),
                            }
                        }
                    };
                    // Java: if (st == CharsetProber.ProbingState.FOUND_IT || 0.99F == this.probers[i].getConfidence())
                    if st == ProbingState::FoundIt || 0.99 == self.probers[i].get_confidence() {
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
    pub fn set_option(&self) {
    }
}
