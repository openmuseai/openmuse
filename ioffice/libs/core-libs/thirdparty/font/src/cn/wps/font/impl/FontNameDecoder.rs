// Java: public class FontNameDecoder
pub struct FontNameDecoder {
    // Java field: String fullName
    pub(crate) fullName: String,
    // Java field: int style
    pub(crate) style: i32,
}

// Java field: String[] encodings
const ENCODINGS: &[&str] = &["GB18030", "GBK", "GB2312"];

// Java: public class FontNameDecoder
impl FontNameDecoder {
    // Java: FontNameDecoder(String fullName, int style)
    pub fn new(full_name: String, style: i32) -> Self {
        // Java: super();
        // Java: this.fullName = fullName;
        // Java: this.style = style;
        FontNameDecoder {
            fullName: full_name,
            style,
        }
    }

    // Java: String getFullName()
    pub fn getFullName(&self) -> &str {
        // Java: return this.fullName;
        &self.fullName
    }

    // Java: int getStyle()
    pub fn getStyle(&self) -> i32 {
        // Java: return this.style;
        self.style
    }

    // Java: static FontNameDecoder decode(String fullName)
    pub fn decode(full_name: &str) -> FontNameDecoder {
        // Java: int style = 0;
        let style;
        // Java: if (!fullName.endsWith(" Bold Italic") && !fullName.endsWith("-BoldItalic"))
        if !full_name.ends_with(" Bold Italic") && !full_name.ends_with("-BoldItalic") {
            // Java: if (!fullName.endsWith(" Bold Oblique") && !fullName.endsWith("-BoldOblique"))
            if !full_name.ends_with(" Bold Oblique") && !full_name.ends_with("-BoldOblique") {
                // Java: if (!fullName.endsWith(" Demibold Italic") && !fullName.endsWith("-DemiboldItalic"))
                if !full_name.ends_with(" Demibold Italic") && !full_name.ends_with("-DemiboldItalic") {
                    // Java: if (!fullName.endsWith(" Bold") && !fullName.endsWith("-Bold"))
                    if !full_name.ends_with(" Bold") && !full_name.ends_with("-Bold") {
                        // Java: if (!fullName.endsWith(" Demibold") && !fullName.endsWith("-Demibold"))
                        if !full_name.ends_with(" Demibold") && !full_name.ends_with("-Demibold") {
                            // Java: if (!fullName.endsWith(" Italic") && !fullName.endsWith("-Italic"))
                            if !full_name.ends_with(" Italic") && !full_name.ends_with("-Italic") {
                                // Java: if (!fullName.endsWith(" Oblique") && !fullName.endsWith("-Oblique"))
                                if !full_name.ends_with(" Oblique") && !full_name.ends_with("-Oblique") {
                                    // Java: if (fullName.endsWith(" Regular") || fullName.endsWith("-Regular"))
                                    if full_name.ends_with(" Regular") || full_name.ends_with("-Regular") {
                                        // Java: style = 0;
                                        style = 0;
                                    } else {
                                        style = 0;
                                    }
                                }
                                // Java: else
                                else {
                                    // Java: style = 2;
                                    style = 2;
                                }
                            }
                            // Java: else
                            else {
                                // Java: style = 2;
                                style = 2;
                            }
                        }
                        // Java: else
                        else {
                            // Java: style = 1;
                            style = 1;
                        }
                    }
                    // Java: else
                    else {
                        // Java: style = 1;
                        style = 1;
                    }
                }
                // Java: else
                else {
                    // Java: style = 3;
                    style = 3;
                }
            }
            // Java: else
            else {
                // Java: style = 3;
                style = 3;
            }
        }
        // Java: else
        else {
            // Java: style = 3;
            style = 3;
        }
        // Java: return new FontNameDecoder(fullName, style);
        FontNameDecoder::new(full_name.to_string(), style)
    }

    // Java: static String removeEncodingSuffix(String name)
    pub fn removeEncodingSuffix(name: Option<&str>) -> Option<String> {
        // Java: if (null == name)
        if name.is_none() {
            // Java: return null;
            return None;
        }
        // Java: else
        {
            let name_str = name.unwrap();
            let mut result = name_str.to_string();
            // Java: int index = name.lastIndexOf(95);
            let index = name_str.rfind('_');
            // Java: if (index > 0)
            if let Some(idx) = index {
                if idx > 0 {
                    // Java: for(String encoding : encodings)
                    for encoding in ENCODINGS {
                        // Java: if (startsWithNoCase(name, index + 1, encoding))
                        if Self::startsWithNoCase(name_str, idx + 1, encoding) {
                            // Java: name = name.substring(0, index);
                            result = name_str[..idx].to_string();
                            // Java: break;
                            break;
                        }
                    }
                }
            }
            // Java: return name;
            Some(result)
        }
    }

    // Java: static boolean startsWithNoCase(String s, int index, String pattern)
    fn startsWithNoCase(s: &str, index: usize, pattern: &str) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = pattern.chars().collect();
        // Java: for(int i = index; i < index + pattern.length(); ++i)
        for i in 0..p_chars.len() {
            // Java: if (i >= s.length())
            if index + i >= s_chars.len() {
                // Java: return false;
                return false;
            }
            // Java: char a = s.charAt(i);
            let a = s_chars[index + i];
            // Java: char b = pattern.charAt(i - index);
            let b = p_chars[i];
            // Java: a = Character.toLowerCase(a);
            let a = a.to_ascii_lowercase();
            // Java: b = Character.toLowerCase(b);
            let b = b.to_ascii_lowercase();
            // Java: if (a != b)
            if a != b {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }
}
