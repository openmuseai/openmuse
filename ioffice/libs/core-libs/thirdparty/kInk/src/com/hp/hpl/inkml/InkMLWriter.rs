#[allow(unused_imports)]
use std::collections::BTreeMap;
use std::collections::HashMap;

// Java: import cn.wps.moffice.kfs.FileOutputStream
#[allow(unused_imports)]
use kfs_kfs_app::kfs::FileOutputStream;
// Java: import cn.wps.moffice.util.KSLog
#[allow(unused_imports)]
use util::cn::wps::moffice::util::KSLog;
// Java: import java.io.BufferedOutputStream
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.io.OutputStreamWriter
// Java: import java.io.PrintWriter
// Java: import java.io.StringWriter
// Java: import java.io.UnsupportedEncodingException
// Java: import java.io.Writer
// Java: import java.util.HashMap
// Java: import java.util.Iterator
// Java: import java.util.Map
// Java: import java.util.Set
// Java: import java.util.TreeMap

// Java field: String TAG
pub(crate) const TAG: &str = "InkMLWriter";

// Java: public class InkMLWriter
pub struct InkMLWriter {
    // Java field: String tabSpace
    pub(crate) tabSpace: String,
    // Java field: int tabSize
    pub(crate) tabSize: i32,
    // Java field: PrintWriter out
    pub(crate) out: String,
    // Java field: int tagLevel
    pub(crate) tagLevel: i32,
    // Java field: String encoding
    pub(crate) encoding: String,
}

// Java: public class InkMLWriter
impl InkMLWriter {
    // Java: InkMLWriter(OutputStream stream)
    pub fn InkMLWriter_stream(_stream: &[u8]) -> Self {
      // Java: this(stream, "UTF8");
      Self::InkMLWriter_stream_encoding(_stream, "UTF8")
    }

    // Java: InkMLWriter(OutputStream stream, String encoding)
    pub fn InkMLWriter_stream_encoding(_stream: &[u8], encoding: &str) -> Self {
      // Java: super();
      // Java: this.tabSpace = "    ";
      let mut tab_space = "    ".to_string();
      // Java: this.tabSize = 4;
      let tab_size = 4;
      // Java: this.tagLevel = 0;
      let tag_level = 0;
      // Java: if (encoding == null)
      let enc = if encoding.is_empty()
      {
         // Java: this.encoding = "UTF8";
         "UTF8".to_string()
      }
      // Java: else
      else
      {
         // Java: this.encoding = encoding;
         encoding.to_string()
      };
      // Java: Writer writer = new OutputStreamWriter(stream, encoding);
      let _ = &enc;
      // Java: this.out = new PrintWriter(writer);
      Self { tabSpace: tab_space, tabSize: tab_size, out: String::new(), tagLevel: tag_level, encoding: enc }
    }

    // Java: InkMLWriter(String fileName, String encoding)
    pub fn InkMLWriter_filename_encoding(file_name: &str, encoding: &str) -> Self {
      // Java: super();
      // Java: this.tabSpace = "    ";
      let tab_space = "    ".to_string();
      // Java: this.tabSize = 4;
      let tab_size = 4;
      // Java: this.tagLevel = 0;
      let tag_level = 0;
      // Java: FileOutputStream fos = null;
      let _fos: Option<i32> = None;
      // Java: fos = new FileOutputStream(fileName);
      let _ = file_name;
      // Java: OutputStream bout = new BufferedOutputStream(fos);
      let _bout: Option<i32> = None;
      // Java: if (encoding == null)
      let enc = if encoding.is_empty()
      {
         // Java: this.encoding = "UTF8";
         "UTF8".to_string()
      }
      // Java: else
      else
      {
         // Java: this.encoding = encoding;
         encoding.to_string()
      };
      // Java: Writer writer = new OutputStreamWriter(bout, encoding);
      let _ = &enc;
      // Java: this.out = new PrintWriter(writer);
      Self { tabSpace: tab_space, tabSize: tab_size, out: String::new(), tagLevel: tag_level, encoding: enc }
    }

    // Java: InkMLWriter(String filePath)
    pub fn InkMLWriter_filepath(file_path: &str) -> Self {
      // Java: this(filePath, "UTF8");
      Self::InkMLWriter_filename_encoding(file_path, "UTF8")
    }

    // Java: InkMLWriter(Writer writer)
    pub fn InkMLWriter_writer() -> Self {
      // Java: super();
      // Java: this.tabSpace = "    ";
      let tab_space = "    ".to_string();
      // Java: this.tabSize = 4;
      let tab_size = 4;
      // Java: this.tagLevel = 0;
      let tag_level = 0;
      // Java: this.out = writer instanceof PrintWriter ? (PrintWriter)writer : new PrintWriter(writer);
      Self { tabSpace: tab_space, tabSize: tab_size, out: String::new(), tagLevel: tag_level, encoding: "UTF8".to_string() }
    }

    // Java: void writeProcessingInstruction()
    pub fn writeProcessingInstruction(&mut self) {
      // Java: this.out.println("<?xml version=\"1.0\" encoding=\"" + this.getEncoding() + "\"?>");
      let enc = self.getEncoding();
      self.out.push_str(&format!("<?xml version=\"1.0\" encoding=\"{}\"?>\n", enc));
    }

    // Java: void writeStartTag(String tagName, HashMap<String, String> attrMap)
    pub fn writeStartTag(&mut self, tag_name: &str, attr_map: &HashMap<String, String>) {
      // Java: for(int i = 0; i < this.tagLevel; ++i)
      for _i in 0..self.tagLevel
      {
         // Java: this.out.write(this.tabSpace);
         self.out.push_str(&self.tabSpace.clone());
      }
      // Java: this.writeBaseStartTag(tagName, attrMap, this.out);
      self.writeBaseStartTag_impl(tag_name, Some(attr_map));
      // Java: this.out.print(">");
      self.out.push('>');
      // Java: this.out.println();
      self.out.push('\n');
    }

    // Java: void writeEndTag(String tagName)
    pub fn writeEndTag(&mut self, tag_name: &str) {
      // Java: KSLog.i(TAG, "end tag for " + tagName + ". tagLevel: " + this.tagLevel);
      let _ = format!("end tag for {}. tagLevel: {}", tag_name, self.tagLevel);
      // Java: for(int i = 0; i < this.tagLevel; ++i)
      for _i in 0..self.tagLevel
      {
         // Java: this.out.print(this.tabSpace);
         self.out.push_str(&self.tabSpace.clone());
      }
      // Java: this.out.print("</" + tagName + ">");
      self.out.push_str(&format!("</{}>", tag_name));
      // Java: this.out.println();
      self.out.push('\n');
      // Java: this.out.flush();
    }

    // Java: void writeCharacters(String characterData)
    pub fn writeCharacters(&mut self, character_data: &str) {
      // Java: for(int i = 0; i < this.tagLevel; ++i)
      for _i in 0..self.tagLevel
      {
         // Java: this.out.print(this.tabSpace);
         self.out.push_str(&self.tabSpace.clone());
      }
      // Java: this.writeCharacters(characterData, this.out);
      self.writeCharacters_to_writer(character_data);
      // Java: this.out.println();
      self.out.push('\n');
      // Java: this.out.flush();
    }

    // Java: void writeXML(String xmlString)
    pub fn writeXML(&mut self, xml_string: &str) {
      // Java: for(int i = 0; i < this.tagLevel; ++i)
      for _i in 0..self.tagLevel
      {
         // Java: this.out.print(this.tabSpace);
         self.out.push_str(&self.tabSpace.clone());
      }
      // Java: this.out.write(xmlString);
      self.out.push_str(xml_string);
      // Java: this.out.println();
      self.out.push('\n');
      // Java: this.out.flush();
    }

    // Java: void writeEmptyStartTag(String tagName, HashMap<String, String> attributesMap)
    pub fn writeEmptyStartTag(&mut self, tag_name: &str, attributes_map: &HashMap<String, String>) {
      // Java: for(int i = 0; i < this.tagLevel; ++i)
      for _i in 0..self.tagLevel
      {
         // Java: this.out.write(this.tabSpace);
         self.out.push_str(&self.tabSpace.clone());
      }
      // Java: this.writeBaseStartTag(tagName, attributesMap, this.out);
      self.writeBaseStartTag_impl(tag_name, Some(attributes_map));
      // Java: this.out.print("/>");
      self.out.push_str("/>");
      // Java: this.out.println();
      self.out.push('\n');
    }

    // Java: String getEmptyStartTagXML(String tagName, HashMap<String, String> attributesMap)
    pub fn getEmptyStartTagXML(&self, tag_name: &str, attributes_map: &HashMap<String, String>) -> String {
      // Java: StringWriter stringWriter = new StringWriter();
      let mut string_writer = String::new();
      // Java: for(int i = 0; i < this.tagLevel; ++i)
      for _i in 0..self.tagLevel
      {
         // Java: stringWriter.write(this.tabSpace);
         string_writer.push_str(&self.tabSpace);
      }
      // Java: this.writeBaseStartTag(tagName, attributesMap, stringWriter);
      string_writer.push('<');
      string_writer.push_str(tag_name);
      let sorted: BTreeMap<&String, &String> = attributes_map.iter().collect();
      for (key, val) in sorted.iter() {
          string_writer.push(' ');
          string_writer.push_str(key);
          string_writer.push_str("=\"");
          for c in val.chars() {
              match c {
                  '\r' => string_writer.push_str("&#xD;"),
                  '"' => string_writer.push_str("&quot;"),
                  '&' => string_writer.push_str("&amp;"),
                  '<' => string_writer.push_str("&lt;"),
                  '>' => string_writer.push_str("&gt;"),
                  _ => string_writer.push(c),
              }
          }
          string_writer.push('"');
      }
      // Java: stringWriter.write("/>\r\n");
      string_writer.push_str("/>\r\n");
      // Java: return stringWriter.toString();
      string_writer
    }

    // Java: void incrementTagLevel()
    pub fn incrementTagLevel(&mut self) {
      // Java: ++this.tagLevel;
      self.tagLevel += 1;
    }

    // Java: void decrementTagLevel()
    pub fn decrementTagLevel(&mut self) {
      // Java: --this.tagLevel;
      self.tagLevel -= 1;
    }

    // Java: int getTagLevel()
    pub fn getTagLevel(&self) -> i32 {
      // Java: return this.tagLevel;
      self.tagLevel
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.out.close();
    }

    // Java: void writeCharacter(char c, Writer out)
    fn writeCharacter_to(c: char, out: &mut String) {
      // Java: switch (c)
      match c
      {
         // Java: case '\r':
         '\r' =>
         {
            // Java: out.write("&#xD;");
            out.push_str("&#xD;");
            // Java: break;
         }
         // Java: case '"':
         '"' =>
         {
            // Java: out.write("&quot;");
            out.push_str("&quot;");
            // Java: break;
         }
         // Java: case '&':
         '&' =>
         {
            // Java: out.write("&amp;");
            out.push_str("&amp;");
            // Java: break;
         }
         // Java: case '<':
         '<' =>
         {
            // Java: out.write("&lt;");
            out.push_str("&lt;");
            // Java: break;
         }
         // Java: case '>':
         '>' =>
         {
            // Java: out.write("&gt;");
            out.push_str("&gt;");
            // Java: break;
         }
         // Java: default:
         _ =>
         {
            // Java: out.write(c);
            out.push(c);
         }
      }
    }

    // Java: void writeBaseStartTag(String tagName, HashMap<String, String> attrs, Writer out)
    fn writeBaseStartTag_impl(&mut self, tag_name: &str, attrs: Option<&HashMap<String, String>>) {
      // Java: try
      {
         // Java: if (null == out)
         if false
         {
            // Java: KSLog.w(TAG, "No writer assigned. Can not write InkML data");
            // Java: return;
            return;
         }
         // Java: out.write(60);
         self.out.push('<');
         // Java: out.write(tagName);
         self.out.push_str(tag_name);
         // Java: if (attrs != null)
         if let Some(attrs) = attrs
         {
            // Java: Map<String, String> sortedAttrs = new TreeMap(attrs);
            let sorted_attrs: BTreeMap<&String, &String> = attrs.iter().collect();
            // Java: Set<String> keys = sortedAttrs.keySet();
            // Java: Iterator<String> itr = keys.iterator();
            // Java: while(itr.hasNext())
            for (key, val) in sorted_attrs.iter()
            {
               // Java: out.write(32);
               self.out.push(' ');
               // Java: String key = (String)itr.next();
               // Java: out.write(key);
               self.out.push_str(key);
               // Java: out.write("=\"");
               self.out.push_str("=\"");
               // Java: this.writeCharacters((String)attrs.get(key), out);
               for c in val.chars() {
                   Self::writeCharacter_to(c, &mut self.out);
               }
               // Java: out.write(34);
               self.out.push('"');
            }
         }
      }
      // Java: catch (IOException e)
      // no-op in Rust
      {
         // Java: KSLog.w(TAG, "Error while writing StartTag,  " + tagName + ".\nMessage: " + e.getMessage());
      }
    }

    // Java: void writeCharacters(String charcaterData, Writer out)
    fn writeCharacters_to_writer(&mut self, charcater_data: &str) {
      // Java: if (null != charcaterData)
      if !charcater_data.is_empty()
      {
         // Java: for(int i = 0; i < charcaterData.length(); ++i)
         for c in charcater_data.chars()
         {
            // Java: char c = charcaterData.charAt(i);
            // Java: try
            {
               // Java: this.writeCharacter(c, out);
               Self::writeCharacter_to(c, &mut self.out);
            }
            // Java: catch (IOException e)
            {
               // Java: KSLog.w(TAG, "Error while encoding character data." + e.getMessage());
            }
         }
      }
    }

    // Java: String getEncoding()
    pub fn getEncoding(&self) -> String {
      // Java: String encodingStr = null;
      let mut encoding_str: Option<String> = None;
      // Java: if (this.encoding.equalsIgnoreCase("UTF8"))
      if self.encoding.eq_ignore_ascii_case("UTF8")
      {
         // Java: encodingStr = "UTF-8";
         encoding_str = Some("UTF-8".to_string());
      }
      // Java: else if (this.encoding.equalsIgnoreCase("8859_1"))
      else if self.encoding.eq_ignore_ascii_case("8859_1")
      {
         // Java: encodingStr = "ISO-8859-1";
         encoding_str = Some("ISO-8859-1".to_string());
      }
      // Java: return encodingStr;
      encoding_str.unwrap_or_default()
    }

    // Java: String getTabSpace()
    pub fn getTabSpace(&self) -> String {
      // Java: return this.tabSpace;
      self.tabSpace.clone()
    }

    // Java: int getTabSize()
    pub fn getTabSize(&self) -> i32 {
      // Java: return this.tabSize;
      self.tabSize
    }

    // Java: void setTabSize(int tabSize)
    pub fn setTabSize(&mut self, tab_size: i32) {
      // Java: this.tabSize = tabSize;
      self.tabSize = tab_size;
      // Java: if (tabSize == 0)
      if tab_size == 0
      {
         // Java: this.tabSpace = "";
         self.tabSpace = String::new();
      }
      // Java: StringBuffer buffer = new StringBuffer();
      let mut buffer = String::new();
      // Java: for(int i = 0; i < tabSize; ++i)
      for _i in 0..tab_size
      {
         // Java: buffer.append(' ');
         buffer.push(' ');
      }
      // Java: this.tabSpace = buffer.toString();
      self.tabSpace = buffer;
    }

}
