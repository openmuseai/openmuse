// Java: public class SymbolConverter
pub struct SymbolConverter;

// Java: public class SymbolConverter
impl SymbolConverter {
    // Java: SymbolConverter()
    pub fn new() -> Self {
        // Java: super();
        SymbolConverter
    }

    // Java: static String convertToSymbol(String run, String fontName)
    pub fn convertToSymbol(run: Option<&str>, font_name: &str) -> Option<String> {
        // Java: if (run != null && isSymbolFont(fontName))
        if let Some(run) = run {
            if Self::isSymbolFont(font_name) {
                // Java: _buffer: &[char] = new char[run.length()];
                let mut buffer: Vec<char> = run.chars().collect();
                // Java: int i = 0;
                // Java: for(int len = run.length(); i < len; ++i)
                for i in 0..buffer.len() {
                    // Java: char c = run.charAt(i);
                    let c = buffer[i];
                    // Java: char hi = (char)(c >> 8);
                    let hi = (c as u32) >> 8;
                    // Java: if (hi == 0 && isVisibleChar(c))
                    if hi == 0 && Self::isVisibleChar(c) {
                        // Java: buffer[i] = (char)(c | '\uf000');
                        buffer[i] = char::from_u32((c as u32) | 0xF000).unwrap_or(c);
                    // Java: else
                    } else {
                        // Java: buffer[i] = c;
                        buffer[i] = c;
                    }
                }
                // Java: return new String(buffer);
                return Some(buffer.into_iter().collect());
            }
        // Java: else
        }
        // Java: return run;
        run.map(|s| s.to_string())
    }

    // Java: static void convertToSymbol(StringBuffer source, int start, int end, String fontName)
    pub fn convertToSymbol_range(source: &mut String, start: usize, end: usize, font_name: &str) {
        // Java: if (source != null && isSymbolFont(fontName))
        if Self::isSymbolFont(font_name) {
            // Java: assert start >= 0 && end <= source.length();
            assert!(end <= source.len());
            let mut chars: Vec<char> = source.chars().collect();
            // Java: for(int i = start; i < end; ++i)
            for i in start..end {
                // Java: char c = source.charAt(i);
                let c = chars[i];
                // Java: if (!isCtrlChar(c))
                if !Self::isCtrlChar(c) {
                    // Java: char hi = (char)(c >> 8);
                    let hi = (c as u32) >> 8;
                    // Java: if (hi == 0)
                    if hi == 0 {
                        // Java: source.setCharAt(i, (char)(c | '\uf000'));
                        chars[i] = char::from_u32((c as u32) | 0xF000).unwrap_or(c);
                    }
                }
            }
            *source = chars.into_iter().collect();
        }
    }

    // Java: static boolean isSymbolFont(String fontName)
    pub fn isSymbolFont(font_name: &str) -> bool {
        // Java: return "Wingdings".equalsIgnoreCase(fontName) || "MT Extra".equalsIgnoreCase(fontName) || "Symbol".equalsIgnoreCase(fontName) || "Webdings".equalsIgnoreCase(fontName) || "Wingdings 2".equalsIgnoreCase(fontName) || "Wingdings 3".equalsIgnoreCase(fontName) || "Kingsoft Sign".equalsIgnoreCase(fontName) || "Kingsoft Confetti".equalsIgnoreCase(fontName) || "Kingsoft Extra".equalsIgnoreCase(fontName) || "Kingsoft Mark".equalsIgnoreCase(fontName);
        let lower = font_name.to_lowercase();
        matches!(lower.as_str(),
            "wingdings" | "mt extra" | "symbol" | "webdings" |
            "wingdings 2" | "wingdings 3" | "kingsoft sign" |
            "kingsoft confetti" | "kingsoft extra" | "kingsoft mark"
        )
    }

    // Java: static boolean isCtrlChar(char c)
    pub fn isCtrlChar(c: char) -> bool {
        // Java: return !isVisibleChar(c);
        !Self::isVisibleChar(c)
    }

    // Java: static boolean isVisibleChar(char c)
    pub fn isVisibleChar(c: char) -> bool {
        // Java: return c >= '!' && c <= '~' || c >= 160 && c <= 255;
        let code = c as u32;
        (code >= 0x21 && code <= 0x7E) || (code >= 160 && code <= 255)
    }
}
