// Java: import java.io.FileInputStream
// Java: import org.mozilla.universalchardet.prober.CharsetProber
use crate::org::mozilla::universalchardet::prober::CharsetProber::ProbingState;
// Java: import org.mozilla.universalchardet.prober.EscCharsetProber
use crate::org::mozilla::universalchardet::prober::EscCharsetProber::EscCharsetProber;
// Java: import org.mozilla.universalchardet.prober.Latin1Prober
use crate::org::mozilla::universalchardet::prober::Latin1Prober::Latin1Prober;
// Java: import org.mozilla.universalchardet.prober.MBCSGroupProber
use crate::org::mozilla::universalchardet::prober::MBCSGroupProber::MBCSGroupProber;
// Java: import org.mozilla.universalchardet.prober.SBCSGroupProber
use crate::org::mozilla::universalchardet::prober::SBCSGroupProber::SBCSGroupProber;
use super::Constants;
use super::CharsetListener::CharsetListener;

// Java field: float SHORTCUT_THRESHOLD
pub(crate) const SHORTCUT_THRESHOLD: f32 = 0.95;
// Java field: float MINIMUM_THRESHOLD
pub(crate) const MINIMUM_THRESHOLD: f32 = 0.2;

// Java: public enum InputState
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputState {
    PureAscii,
    EscAscii,
    HighByte,
}

enum DetectorProber {
    Mbcs(MBCSGroupProber),
    Sbcs(SBCSGroupProber),
    Latin1(Latin1Prober),
}

impl DetectorProber {
    fn handle_data(&mut self, buf: &[u8], offset: usize, length: usize) -> ProbingState {
        match self {
            DetectorProber::Mbcs(p) => p.handle_data(buf, offset, length),
            DetectorProber::Sbcs(p) => p.handle_data(buf, offset, length),
            DetectorProber::Latin1(p) => p.handle_data(buf, offset, length),
        }
    }
    fn get_confidence(&mut self) -> f32 {
        match self {
            DetectorProber::Mbcs(p) => p.get_confidence(),
            DetectorProber::Sbcs(p) => p.get_confidence(),
            DetectorProber::Latin1(p) => p.get_confidence(),
        }
    }
    fn get_charset_name(&mut self) -> String {
        match self {
            DetectorProber::Mbcs(p) => p.get_charset_name().to_string(),
            DetectorProber::Sbcs(p) => p.get_charset_name().to_string(),
            DetectorProber::Latin1(p) => p.get_charset_name().to_string(),
        }
    }
    fn reset(&mut self) {
        match self {
            DetectorProber::Mbcs(p) => p.reset(),
            DetectorProber::Sbcs(p) => p.reset(),
            DetectorProber::Latin1(p) => p.reset(),
        }
    }
}

// Java: public class UniversalDetector
pub struct UniversalDetector {
    // Java field: InputState inputState
    input_state: InputState,
    // Java field: boolean done
    pub(crate) done: bool,
    // Java field: boolean start
    start: bool,
    // Java field: boolean gotData
    got_data: bool,
    // Java field: byte lastChar
    last_char: u8,
    // Java field: String detectedCharset
    pub(crate) detected_charset: Option<String>,
    // Java field: CharsetProber[] probers
    probers: [Option<DetectorProber>; 3],
    // Java field: CharsetProber escCharsetProber
    esc_charset_prober: Option<EscCharsetProber>,
    // Java field: CharsetListener listener
    pub(crate) listener: Option<Box<dyn CharsetListener>>,
}

// Java: public class UniversalDetector
impl UniversalDetector {
    // Java: UniversalDetector(CharsetListener listener)
    pub fn new(listener: Option<Box<dyn CharsetListener>>) -> Self {
        // Java: super();
        // Java: this.listener = listener;
        // Java: this.escCharsetProber = null;
        // Java: this.probers = new CharsetProber[3];
        // Java: for(int i = 0; i < this.probers.length; ++i)
        // Java: this.probers[i] = null;
        let mut detector = Self {
            input_state: InputState::PureAscii,
            done: false,
            start: true,
            got_data: false,
            last_char: 0,
            detected_charset: None,
            probers: [None, None, None],
            esc_charset_prober: None,
            listener,
        };
        // Java: this.reset();
        detector.reset();
        detector
    }

    // Java: boolean isDone()
    pub fn is_done(&self) -> bool {
        // Java: return this.done;
        self.done
    }

    // Java: String getDetectedCharset()
    pub fn get_detected_charset(&self) -> Option<&str> {
        // Java: return this.detectedCharset;
        self.detected_charset.as_deref()
    }

    // Java: void setListener(CharsetListener listener)
    pub fn set_listener(&mut self, listener: Option<Box<dyn CharsetListener>>) {
        // Java: this.listener = listener;
        self.listener = listener;
    }

    // Java: CharsetListener getListener()
    pub fn get_listener(&self) -> &Option<Box<dyn CharsetListener>> {
        // Java: return this.listener;
        &self.listener
    }

    // Java: void handleData(byte[] buf, int offset, int length)
    pub fn handle_data(&mut self, buf: &[u8], offset: usize, length: usize) {
        // Java: if (!this.done)
        if !self.done {
            // Java: if (length > 0)
            if length > 0 {
                // Java: this.gotData = true;
                self.got_data = true;
            }
            // Java: if (this.start)
            if self.start {
                // Java: this.start = false;
                self.start = false;
                // Java: if (length > 3)
                if length > 3 {
                    // Java: int b1 = buf[offset] & 255;
                    let b1 = buf[offset] as i32 & 255;
                    // Java: int b2 = buf[offset + 1] & 255;
                    let b2 = buf[offset + 1] as i32 & 255;
                    // Java: int b3 = buf[offset + 2] & 255;
                    let b3 = buf[offset + 2] as i32 & 255;
                    // Java: int b4 = buf[offset + 3] & 255;
                    let b4 = buf[offset + 3] as i32 & 255;
                    // Java: switch (b1)
                    match b1 {
                        // Java: case 0:
                        0 => {
                            // Java: if (b2 == 0 && b3 == 254 && b4 == 255)
                            if b2 == 0 && b3 == 254 && b4 == 255 {
                                // Java: this.detectedCharset = Constants.CHARSET_UTF_32BE;
                                self.detected_charset = Some(Constants::CHARSET_UTF_32BE.to_string());
                            // Java: else if (b2 == 0 && b3 == 255 && b4 == 254)
                            } else if b2 == 0 && b3 == 255 && b4 == 254 {
                                // Java: this.detectedCharset = Constants.CHARSET_X_ISO_10646_UCS_4_2143;
                                self.detected_charset = Some(Constants::CHARSET_X_ISO_10646_UCS_4_2143.to_string());
                            }
                        }
                        // Java: break;
                        // Java: case 239:
                        239 => {
                            // Java: if (b2 == 187 && b3 == 191)
                            if b2 == 187 && b3 == 191 {
                                // Java: this.detectedCharset = Constants.CHARSET_UTF_8;
                                self.detected_charset = Some(Constants::CHARSET_UTF_8.to_string());
                            }
                        }
                        // Java: break;
                        // Java: case 254:
                        254 => {
                            // Java: if (b2 == 255 && b3 == 0 && b4 == 0)
                            if b2 == 255 && b3 == 0 && b4 == 0 {
                                // Java: this.detectedCharset = Constants.CHARSET_X_ISO_10646_UCS_4_3412;
                                self.detected_charset = Some(Constants::CHARSET_X_ISO_10646_UCS_4_3412.to_string());
                            // Java: else if (b2 == 255)
                            } else if b2 == 255 {
                                // Java: this.detectedCharset = Constants.CHARSET_UTF_16BE;
                                self.detected_charset = Some(Constants::CHARSET_UTF_16BE.to_string());
                            }
                        }
                        // Java: break;
                        // Java: case 255:
                        255 => {
                            // Java: if (b2 == 254 && b3 == 0 && b4 == 0)
                            if b2 == 254 && b3 == 0 && b4 == 0 {
                                // Java: this.detectedCharset = Constants.CHARSET_UTF_32LE;
                                self.detected_charset = Some(Constants::CHARSET_UTF_32LE.to_string());
                            // Java: else if (b2 == 254)
                            } else if b2 == 254 {
                                // Java: this.detectedCharset = Constants.CHARSET_UTF_16LE;
                                self.detected_charset = Some(Constants::CHARSET_UTF_16LE.to_string());
                            }
                        }
                        _ => {}
                    }
                    // Java: if (this.detectedCharset != null)
                    if self.detected_charset.is_some() {
                        // Java: this.done = true;
                        self.done = true;
                        // Java: return;
                        return;
                    }
                }
            }
            // Java: int maxPos = offset + length;
            let max_pos = offset + length;
            // Java: for(int i = offset; i < maxPos; ++i)
            for i in offset..max_pos {
                // Java: int c = buf[i] & 255;
                let c = buf[i] as i32 & 255;
                // Java: if ((c & 128) != 0 && c != 160)
                if (c & 128) != 0 && c != 160 {
                    // Java: if (this.inputState != UniversalDetector.InputState.HIGHBYTE)
                    if self.input_state != InputState::HighByte {
                        // Java: this.inputState = UniversalDetector.InputState.HIGHBYTE;
                        self.input_state = InputState::HighByte;
                        // Java: if (this.escCharsetProber != null)
                        if self.esc_charset_prober.is_some() {
                            // Java: this.escCharsetProber = null;
                            self.esc_charset_prober = None;
                        }
                        // Java: if (this.probers[0] == null)
                        if self.probers[0].is_none() {
                            // Java: this.probers[0] = new MBCSGroupProber();
                            self.probers[0] = Some(DetectorProber::Mbcs(MBCSGroupProber::new()));
                        }
                        // Java: if (this.probers[1] == null)
                        if self.probers[1].is_none() {
                            // Java: this.probers[1] = new SBCSGroupProber();
                            self.probers[1] = Some(DetectorProber::Sbcs(SBCSGroupProber::new()));
                        }
                        // Java: if (this.probers[2] == null)
                        if self.probers[2].is_none() {
                            // Java: this.probers[2] = new Latin1Prober();
                            self.probers[2] = Some(DetectorProber::Latin1(Latin1Prober::new()));
                        }
                    }
                // Java: else
                } else {
                    // Java: if (this.inputState == UniversalDetector.InputState.PURE_ASCII && (c == 27 || c == 123 && this.lastChar == 126))
                    if self.input_state == InputState::PureAscii && (c == 27 || (c == 123 && self.last_char == 126)) {
                        // Java: this.inputState = UniversalDetector.InputState.ESC_ASCII;
                        self.input_state = InputState::EscAscii;
                    }
                    // Java: this.lastChar = buf[i];
                    self.last_char = buf[i];
                }
            }
            // Java: if (this.inputState == UniversalDetector.InputState.ESC_ASCII)
            if self.input_state == InputState::EscAscii {
                // Java: if (this.escCharsetProber == null)
                if self.esc_charset_prober.is_none() {
                    // Java: this.escCharsetProber = new EscCharsetProber();
                    self.esc_charset_prober = Some(EscCharsetProber::new());
                }
                // Java: CharsetProber.ProbingState st = this.escCharsetProber.handleData(buf, offset, length);
                let st = self.esc_charset_prober.as_mut().unwrap().handle_data(buf, offset, length);
                // Java: if (st == CharsetProber.ProbingState.FOUND_IT || 0.99F == this.escCharsetProber.getConfidence())
                if st == ProbingState::FoundIt || 0.99 == self.esc_charset_prober.as_ref().unwrap().get_confidence() {
                    // Java: this.done = true;
                    self.done = true;
                    // Java: this.detectedCharset = this.escCharsetProber.getCharSetName();
                    self.detected_charset = Some(self.esc_charset_prober.as_ref().unwrap().get_charset_name().to_string());
                }
            // Java: else if (this.inputState == UniversalDetector.InputState.HIGHBYTE)
            } else if self.input_state == InputState::HighByte {
                // Java: for(int i = 0; i < this.probers.length; ++i)
                for i in 0..self.probers.len() {
                    if let Some(ref mut prober) = self.probers[i] {
                        // Java: CharsetProber.ProbingState st = this.probers[i].handleData(buf, offset, length);
                        let st = prober.handle_data(buf, offset, length);
                        // Java: if (st == CharsetProber.ProbingState.FOUND_IT)
                        if st == ProbingState::FoundIt {
                            // Java: this.done = true;
                            self.done = true;
                            // Java: this.detectedCharset = this.probers[i].getCharSetName();
                            self.detected_charset = Some(prober.get_charset_name());
                            // Java: return;
                            return;
                        }
                    }
                }
            }
        }
    }

    // Java: void dataEnd()
    pub fn data_end(&mut self) {
        // Java: if (this.gotData)
        if self.got_data {
            // Java: if (this.detectedCharset != null)
            if self.detected_charset.is_some() {
                // Java: this.done = true;
                self.done = true;
                // Java: if (this.listener != null)
                if let Some(ref listener) = self.listener {
                    // Java: this.listener.report(this.detectedCharset);
                    listener.report(self.detected_charset.as_deref().unwrap_or(""));
                }
            // Java: else
            } else {
                // Java: if (this.inputState == UniversalDetector.InputState.HIGHBYTE)
                if self.input_state == InputState::HighByte {
                    // Java: float maxProberConfidence = 0.0F;
                    let mut max_prober_confidence: f32 = 0.0;
                    // Java: int maxProber = 0;
                    let mut max_prober: usize = 0;
                    // Java: for(int i = 0; i < this.probers.length; ++i)
                    for i in 0..self.probers.len() {
                        if let Some(ref mut prober) = self.probers[i] {
                            // Java: float proberConfidence = this.probers[i].getConfidence();
                            let prober_confidence = prober.get_confidence();
                            // Java: if (proberConfidence > maxProberConfidence)
                            if prober_confidence > max_prober_confidence {
                                // Java: maxProberConfidence = proberConfidence;
                                max_prober_confidence = prober_confidence;
                                // Java: maxProber = i;
                                max_prober = i;
                            }
                        }
                    }
                    // Java: if (maxProberConfidence > 0.2F)
                    if max_prober_confidence > MINIMUM_THRESHOLD {
                        // Java: this.detectedCharset = this.probers[maxProber].getCharSetName();
                        if let Some(ref mut prober) = self.probers[max_prober] {
                            self.detected_charset = Some(prober.get_charset_name());
                        }
                        // Java: if (this.listener != null)
                        if let Some(ref listener) = self.listener {
                            // Java: this.listener.report(this.detectedCharset);
                            listener.report(self.detected_charset.as_deref().unwrap_or(""));
                        }
                    }
                // Java: else if (this.inputState == UniversalDetector.InputState.ESC_ASCII)
                } else if self.input_state == InputState::EscAscii {
                }
            }
        }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.done = false;
        self.done = false;
        // Java: this.start = true;
        self.start = true;
        // Java: this.detectedCharset = null;
        self.detected_charset = None;
        // Java: this.gotData = false;
        self.got_data = false;
        // Java: this.inputState = UniversalDetector.InputState.PURE_ASCII;
        self.input_state = InputState::PureAscii;
        // Java: this.lastChar = 0;
        self.last_char = 0;
        // Java: if (this.escCharsetProber != null)
        if let Some(ref mut esc) = self.esc_charset_prober {
            // Java: this.escCharsetProber.reset();
            esc.reset();
        }
        // Java: for(int i = 0; i < this.probers.length; ++i)
        for i in 0..self.probers.len() {
            // Java: if (this.probers[i] != null)
            if let Some(ref mut prober) = self.probers[i] {
                // Java: this.probers[i].reset();
                prober.reset();
            }
        }
    }

    // Java: static void main(String[] args)
    // Java: if (args.length != 1)
    // Java: System.out.println("USAGE: java UniversalDetector filename");
    // Java: else
    // Java: UniversalDetector detector = new UniversalDetector(new CharsetListener()
    // Java: public void report(String name)
    // Java: System.out.println("charset = " + name);
    // Java: byte[] buf = new byte[4096];
    // Java: FileInputStream fis = new FileInputStream(args[0]);
    // Java: int nread;
    // Java: while((nread = fis.read(buf)) > 0 && !detector.isDone())
    // Java: detector.handleData(buf, 0, nread);
    // Java: detector.dataEnd();
}

// Java: public enum InputState
impl InputState {
    // Java: InputState()
    pub fn new() -> Self {
        InputState::PureAscii
    }
}
