// Java field: String STANDARD_INDENT
pub(crate) const STANDARD_INDENT: &str = "  ";

// Java: public class OutputFormat implements Cloneable
#[derive(Clone)]
pub struct OutputFormat {
    // Java field: boolean suppressDeclaration
    pub(crate) suppressDeclaration: bool,
    // Java field: boolean newLineAfterDeclaration
    pub(crate) newLineAfterDeclaration: bool,
    // Java field: String encoding
    pub(crate) encoding: String,
    // Java field: boolean omitEncoding
    pub(crate) omitEncoding: bool,
    // Java field: String indent
    pub(crate) indent: Option<String>,
    // Java field: boolean expandEmptyElements
    pub(crate) expandEmptyElements: bool,
    // Java field: boolean newlines
    pub(crate) newlines: bool,
    // Java field: String lineSeparator
    pub(crate) lineSeparator: String,
    // Java field: boolean trimText
    pub(crate) trimText: bool,
    // Java field: boolean padText
    pub(crate) padText: bool,
    // Java field: boolean doXHTML
    pub(crate) doXHTML: bool,
    // Java field: int newLineAfterNTags
    pub(crate) newLineAfterNTags: i32,
    // Java field: char attributeQuoteChar
    pub(crate) attributeQuoteChar: char,
}

// Java: public class OutputFormat implements Cloneable
impl OutputFormat {
    // Java: OutputFormat()
    pub fn new() -> Self {
        // Java: super();
        OutputFormat {
            suppressDeclaration: false,
            newLineAfterDeclaration: true,
            encoding: "UTF-8".to_string(),
            omitEncoding: false,
            indent: None,
            expandEmptyElements: false,
            newlines: false,
            lineSeparator: "\n".to_string(),
            trimText: false,
            padText: false,
            doXHTML: false,
            newLineAfterNTags: 0,
            attributeQuoteChar: '"',
        }
    }

    // Java: OutputFormat(String indent)
    pub fn new_with_indent(indent: &str) -> Self {
        // Java: super();
        let mut fmt = Self::new();
        // Java: this.indent = indent;
        fmt.indent = Some(indent.to_string());
        fmt
    }

    // Java: OutputFormat(String indent, boolean newlines)
    pub fn new_with_indent_and_newlines(indent: &str, newlines: bool) -> Self {
        // Java: super();
        let mut fmt = Self::new();
        // Java: this.indent = indent;
        fmt.indent = Some(indent.to_string());
        // Java: this.newlines = newlines;
        fmt.newlines = newlines;
        fmt
    }

    // Java: OutputFormat(String indent, boolean newlines, String encoding)
    pub fn new_with_indent_newlines_encoding(indent: &str, newlines: bool, encoding: &str) -> Self {
        // Java: super();
        let mut fmt = Self::new();
        // Java: this.indent = indent;
        fmt.indent = Some(indent.to_string());
        // Java: this.newlines = newlines;
        fmt.newlines = newlines;
        // Java: this.encoding = encoding;
        fmt.encoding = encoding.to_string();
        fmt
    }

    // Java: String getLineSeparator()
    pub fn getLineSeparator(&self) -> &str {
        // Java: return this.lineSeparator;
        &self.lineSeparator
    }

    // Java: void setLineSeparator(String separator)
    pub fn setLineSeparator(&mut self, separator: &str) {
        // Java: this.lineSeparator = separator;
        self.lineSeparator = separator.to_string();
    }

    // Java: boolean isNewlines()
    pub fn isNewlines(&self) -> bool {
        // Java: return this.newlines;
        self.newlines
    }

    // Java: void setNewlines(boolean newlines)
    pub fn setNewlines(&mut self, newlines: bool) {
        // Java: this.newlines = newlines;
        self.newlines = newlines;
    }

    // Java: String getEncoding()
    pub fn getEncoding(&self) -> &str {
        // Java: return this.encoding;
        &self.encoding
    }

    // Java: void setEncoding(String encoding)
    pub fn setEncoding(&mut self, encoding: &str) {
        // Java: if (encoding != null)
        if !encoding.is_empty() {
            // Java: this.encoding = encoding;
            self.encoding = encoding.to_string();
        }
    }

    // Java: boolean isOmitEncoding()
    pub fn isOmitEncoding(&self) -> bool {
        // Java: return this.omitEncoding;
        self.omitEncoding
    }

    // Java: void setOmitEncoding(boolean omitEncoding)
    pub fn setOmitEncoding(&mut self, omitEncoding: bool) {
        // Java: this.omitEncoding = omitEncoding;
        self.omitEncoding = omitEncoding;
    }

    // Java: void setSuppressDeclaration(boolean suppressDeclaration)
    pub fn setSuppressDeclaration(&mut self, suppressDeclaration: bool) {
        // Java: this.suppressDeclaration = suppressDeclaration;
        self.suppressDeclaration = suppressDeclaration;
    }

    // Java: boolean isSuppressDeclaration()
    pub fn isSuppressDeclaration(&self) -> bool {
        // Java: return this.suppressDeclaration;
        self.suppressDeclaration
    }

    // Java: void setNewLineAfterDeclaration(boolean newLineAfterDeclaration)
    pub fn setNewLineAfterDeclaration(&mut self, newLineAfterDeclaration: bool) {
        // Java: this.newLineAfterDeclaration = newLineAfterDeclaration;
        self.newLineAfterDeclaration = newLineAfterDeclaration;
    }

    // Java: boolean isNewLineAfterDeclaration()
    pub fn isNewLineAfterDeclaration(&self) -> bool {
        // Java: return this.newLineAfterDeclaration;
        self.newLineAfterDeclaration
    }

    // Java: boolean isExpandEmptyElements()
    pub fn isExpandEmptyElements(&self) -> bool {
        // Java: return this.expandEmptyElements;
        self.expandEmptyElements
    }

    // Java: void setExpandEmptyElements(boolean expandEmptyElements)
    pub fn setExpandEmptyElements(&mut self, expandEmptyElements: bool) {
        // Java: this.expandEmptyElements = expandEmptyElements;
        self.expandEmptyElements = expandEmptyElements;
    }

    // Java: boolean isTrimText()
    pub fn isTrimText(&self) -> bool {
        // Java: return this.trimText;
        self.trimText
    }

    // Java: void setTrimText(boolean trimText)
    pub fn setTrimText(&mut self, trimText: bool) {
        // Java: this.trimText = trimText;
        self.trimText = trimText;
    }

    // Java: boolean isPadText()
    pub fn isPadText(&self) -> bool {
        // Java: return this.padText;
        self.padText
    }

    // Java: void setPadText(boolean padText)
    pub fn setPadText(&mut self, padText: bool) {
        // Java: this.padText = padText;
        self.padText = padText;
    }

    // Java: String getIndent()
    pub fn getIndent(&self) -> Option<&str> {
        // Java: return this.indent;
        self.indent.as_deref()
    }

    // Java: void setIndent(String indent)
    pub fn setIndent(&mut self, indent: Option<&str>) {
        // Java: if (indent != null && indent.length() <= 0)
        let indent = if indent.is_some() && indent.unwrap().is_empty() {
            // Java: indent = null;
            None
        } else {
            indent
        };
        // Java: this.indent = indent;
        self.indent = indent.map(|s| s.to_string());
    }

    // Java: void setIndent(boolean doIndent)
    pub fn setIndent_enabled(&mut self, doIndent: bool) {
        // Java: if (doIndent)
        if doIndent {
            // Java: this.indent = "  ";
            self.indent = Some("  ".to_string());
        }
        // Java: else
        else {
            // Java: this.indent = null;
            self.indent = None;
        }
    }

    // Java: void setIndentSize(int indentSize)
    pub fn setIndentSize(&mut self, indentSize: i32) {
        // Java: StringBuffer indentBuffer = new StringBuffer();
        let mut indent_buffer = String::new();
        // Java: for(int i = 0; i < indentSize; ++i)
        for _ in 0..indentSize {
            // Java: indentBuffer.append(" ");
            indent_buffer.push(' ');
        }
        // Java: this.indent = indentBuffer.toString();
        self.indent = Some(indent_buffer);
    }

    // Java: boolean isXHTML()
    pub fn isXHTML(&self) -> bool {
        // Java: return this.doXHTML;
        self.doXHTML
    }

    // Java: void setXHTML(boolean xhtml)
    pub fn setXHTML(&mut self, xhtml: bool) {
        // Java: this.doXHTML = xhtml;
        self.doXHTML = xhtml;
    }

    // Java: int getNewLineAfterNTags()
    pub fn getNewLineAfterNTags(&self) -> i32 {
        // Java: return this.newLineAfterNTags;
        self.newLineAfterNTags
    }

    // Java: void setNewLineAfterNTags(int tagCount)
    pub fn setNewLineAfterNTags(&mut self, tagCount: i32) {
        // Java: this.newLineAfterNTags = tagCount;
        self.newLineAfterNTags = tagCount;
    }

    // Java: char getAttributeQuoteCharacter()
    pub fn getAttributeQuoteCharacter(&self) -> char {
        // Java: return this.attributeQuoteChar;
        self.attributeQuoteChar
    }

    // Java: void setAttributeQuoteCharacter(char quoteChar)
    pub fn setAttributeQuoteCharacter(&mut self, quoteChar: char) {
        // Java: if (quoteChar != '\'' && quoteChar != '"')
        if quoteChar != '\'' && quoteChar != '"' {
            // Java: throw new IllegalArgumentException("Invalid attribute quote character (" + quoteChar + ")");
            panic!("Invalid attribute quote character ({})", quoteChar);
        }
        // Java: else
        else {
            // Java: this.attributeQuoteChar = quoteChar;
            self.attributeQuoteChar = quoteChar;
        }
    }

    // Java: int parseOptions(String[] args, int i)
    pub fn parseOptions(&mut self, args: &[&str], mut i: usize) -> usize {
        // Java: for(int size = args.length; i < size; ++i)
        let size = args.len();
        while i < size {
            // Java: if (args[i].equals("-suppressDeclaration"))
            if args[i] == "-suppressDeclaration" {
                // Java: this.setSuppressDeclaration(true);
                self.setSuppressDeclaration(true);
            }
            // Java: else if (args[i].equals("-omitEncoding"))
            else if args[i] == "-omitEncoding" {
                // Java: this.setOmitEncoding(true);
                self.setOmitEncoding(true);
            }
            // Java: else if (args[i].equals("-indent"))
            else if args[i] == "-indent" {
                // Java: ++i;
                i += 1;
                // Java: this.setIndent(args[i]);
                self.setIndent(Some(args[i]));
            }
            // Java: else if (args[i].equals("-indentSize"))
            else if args[i] == "-indentSize" {
                // Java: ++i;
                i += 1;
                // Java: this.setIndentSize(Integer.parseInt(args[i]));
                self.setIndentSize(args[i].parse::<i32>().unwrap());
            }
            // Java: else if (args[i].startsWith("-expandEmpty"))
            else if args[i].starts_with("-expandEmpty") {
                // Java: this.setExpandEmptyElements(true);
                self.setExpandEmptyElements(true);
            }
            // Java: else if (args[i].equals("-encoding"))
            else if args[i] == "-encoding" {
                // Java: ++i;
                i += 1;
                // Java: this.setEncoding(args[i]);
                self.setEncoding(args[i]);
            }
            // Java: else if (args[i].equals("-newlines"))
            else if args[i] == "-newlines" {
                // Java: this.setNewlines(true);
                self.setNewlines(true);
            }
            // Java: else if (args[i].equals("-lineSeparator"))
            else if args[i] == "-lineSeparator" {
                // Java: ++i;
                i += 1;
                // Java: this.setLineSeparator(args[i]);
                self.setLineSeparator(args[i]);
            }
            // Java: else if (args[i].equals("-trimText"))
            else if args[i] == "-trimText" {
                // Java: this.setTrimText(true);
                self.setTrimText(true);
            }
            // Java: else if (args[i].equals("-padText"))
            else if args[i] == "-padText" {
                // Java: this.setPadText(true);
                self.setPadText(true);
            }
            // Java: else
            else {
                // Java: if (!args[i].startsWith("-xhtml"))
                if !args[i].starts_with("-xhtml") {
                    // Java: return i;
                    return i;
                }
                // Java: this.setXHTML(true);
                self.setXHTML(true);
            }
            i += 1;
        }
        // Java: return i;
        i
    }

    // Java: static OutputFormat createPrettyPrint()
    pub fn createPrettyPrint() -> OutputFormat {
        // Java: OutputFormat format = new OutputFormat();
        let mut format = OutputFormat::new();
        // Java: format.setIndentSize(2);
        format.setIndentSize(2);
        // Java: format.setNewlines(true);
        format.setNewlines(true);
        // Java: format.setTrimText(true);
        format.setTrimText(true);
        // Java: format.setPadText(true);
        format.setPadText(true);
        // Java: return format;
        format
    }

    // Java: static OutputFormat createCompactFormat()
    pub fn createCompactFormat() -> OutputFormat {
        // Java: OutputFormat format = new OutputFormat();
        let mut format = OutputFormat::new();
        // Java: format.setIndent(false);
        format.setIndent_enabled(false);
        // Java: format.setNewlines(false);
        format.setNewlines(false);
        // Java: format.setTrimText(true);
        format.setTrimText(true);
        // Java: return format;
        format
    }
}
