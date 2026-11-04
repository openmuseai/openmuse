// Java: public class SequenceModel
pub struct SequenceModel {
    // Java field: short[] charToOrderMap
    pub(crate) char_to_order_map: &'static [i16],
    // Java field: byte[] precedenceMatrix
    pub(crate) precedence_matrix: &'static [u8],
    // Java field: float typicalPositiveRatio
    pub(crate) typical_positive_ratio: f32,
    // Java field: boolean keepEnglishLetter
    pub(crate) keep_english_letter: bool,
    // Java field: String charsetName
    pub(crate) charset_name: &'static str,
}

// Java: public class SequenceModel
impl SequenceModel {
    // Java: SequenceModel(short[] charToOrderMap, byte[] precedenceMatrix, float typicalPositiveRatio, boolean keepEnglishLetter, String charsetName)
    pub fn new(
        char_to_order_map: &'static [i16],
        precedence_matrix: &'static [u8],
        typical_positive_ratio: f32,
        keep_english_letter: bool,
        charset_name: &'static str,
    ) -> Self {
        // Java: super();
        // Java: this.charToOrderMap = charToOrderMap;
        // Java: this.precedenceMatrix = precedenceMatrix;
        // Java: this.typicalPositiveRatio = typicalPositiveRatio;
        // Java: this.keepEnglishLetter = keepEnglishLetter;
        // Java: this.charsetName = charsetName;
        Self {
            char_to_order_map,
            precedence_matrix,
            typical_positive_ratio,
            keep_english_letter,
            charset_name,
        }
    }

    // Java: short getOrder(byte b)
    pub fn get_order(&self, b: u8) -> i16 {
        // Java: int c = b & 255;
        let c = (b & 0xFF) as usize;
        // Java: return this.charToOrderMap[c];
        self.char_to_order_map[c]
    }

    // Java: byte getPrecedence(int pos)
    pub fn get_precedence(&self, pos: usize) -> u8 {
        // Java: return this.precedenceMatrix[pos];
        self.precedence_matrix[pos]
    }

    // Java: float getTypicalPositiveRatio()
    pub fn get_typical_positive_ratio(&self) -> f32 {
        // Java: return this.typicalPositiveRatio;
        self.typical_positive_ratio
    }

    // Java: boolean getKeepEnglishLetter()
    pub fn get_keep_english_letter(&self) -> bool {
        // Java: return this.keepEnglishLetter;
        self.keep_english_letter
    }

    // Java: String getCharsetName()
    pub fn get_charset_name(&self) -> &str {
        // Java: return this.charsetName;
        self.charset_name
    }
}
