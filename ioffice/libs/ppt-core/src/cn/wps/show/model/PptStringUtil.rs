#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// LLM: DocumentInputStream is from external crate, using trait stub
// Java: import org.apache.poi.util.LittleEndianInput
// LLM: LittleEndianInput is from external crate, using trait stub
// Java: import org.apache.poi.util.LittleEndianOutput
// LLM: LittleEndianOutput is from external crate, using trait stub

pub trait DocumentInputStream {
    fn readShort(&mut self) -> i16;
    fn readByte(&mut self) -> i8;
    fn skip(&mut self, n: i64);
}

pub trait LittleEndianInput {
    fn readShort(&mut self) -> i16;
    fn skip(&mut self, n: i64);
}

pub trait LittleEndianOutput {
    fn writeShort(&mut self, v: i16);
    fn writeByte(&mut self, v: i32);
}

// Java: public class PptStringUtil
pub struct PptStringUtil {
}

// Java: public class PptStringUtil
impl PptStringUtil {
    // Java: static String readTextChar(DocumentInputStream inputStream, int maxLength)
    pub fn readTextChar_doc(inputStream: &mut dyn DocumentInputStream, mut maxLength: i32) -> String {
      // Java: StringBuffer strBuffer = new StringBuffer();
      let mut strBuffer = String::new();
      // Java: char ch = '\u0000';
      let mut ch: u16;
      // Java: do
        loop {
         // Java: ch = (char)inputStream.readShort();
         ch = inputStream.readShort() as u16;
         // Java: maxLength -= 2;
         maxLength -= 2;
         // Java: if (ch == 0)
            if ch == 0 {
            // Java: break;
            break;
         }
         // Java: strBuffer.append(ch);
         if let Some(c) = char::from_u32(ch as u32) { strBuffer.push(c); }
         if maxLength <= 0 { break; }
      }
      // Java: inputStream.skip((long)maxLength);
      if maxLength > 0 { inputStream.skip(maxLength as i64); }
      // Java: return strBuffer.toString();
      return strBuffer;
    }

    // Java: static String readTextChar(LittleEndianInput inputStream, int maxLength)
    pub fn readTextChar_lei(inputStream: &mut dyn LittleEndianInput, mut maxLength: i32) -> String {
      // Java: StringBuffer strBuffer = new StringBuffer();
      let mut strBuffer = String::new();
      // Java: char ch = '\u0000';
      let mut ch: u16;
      // Java: while(maxLength > 0)
        while maxLength > 0 {
         // Java: ch = (char)inputStream.readShort();
         ch = inputStream.readShort() as u16;
         // Java: maxLength -= 2;
         maxLength -= 2;
         // Java: if (ch == 0)
            if ch == 0 {
            // Java: break;
            break;
         }
         // Java: strBuffer.append(ch);
         if let Some(c) = char::from_u32(ch as u32) { strBuffer.push(c); }
      }
      // Java: inputStream.skip((long)maxLength);
      if maxLength > 0 { inputStream.skip(maxLength as i64); }
      // Java: return strBuffer.toString();
      return strBuffer;
    }

    // Java: static void writeTextChar(LittleEndianOutput out, String string, int maxLength)
    pub fn writeTextChar(out: &mut dyn LittleEndianOutput, string: &str, maxLength: i32) {
      // Java: int strLen = string.length() * 2;
      let strLen = (string.len() as i32) * 2;
      // Java: if (strLen > maxLength)
        if strLen > maxLength {
         // Java: throw new RuntimeException("string too large");
         panic!("string too large");
      }
      // Java: else
        else {
         // Java: char[] chars = string.toCharArray();
         let chars: Vec<u16> = string.encode_utf16().collect();
         // Java: for(char ch : chars)
            for &ch in &chars {
            // Java: out.writeShort(ch);
            out.writeShort(ch as i16);
         }
         // Java: int remain = maxLength - strLen;
         let remain = maxLength - strLen;
         // Java: for(int i = 0; i < remain; ++i)
            for _i in 0..remain {
            // Java: out.writeByte(0);
            out.writeByte(0);
         }
      }
    }

    // Java: static String readTextByte(DocumentInputStream inputStream, int maxLength)
    pub fn readTextByte(inputStream: &mut dyn DocumentInputStream, mut maxLength: i32) -> String {
      // Java: StringBuffer strBuffer = new StringBuffer();
      let mut strBuffer = String::new();
      // Java: char ch = '\u0000';
      let mut ch: u8;
      // Java: do
        loop {
         // Java: ch = (char)inputStream.readByte();
         ch = inputStream.readByte() as u8;
         // Java: --maxLength;
         maxLength -= 1;
         // Java: if (ch == 0)
            if ch == 0 {
            // Java: break;
            break;
         }
         // Java: strBuffer.append(ch);
         strBuffer.push(ch as char);
         if maxLength <= 0 { break; }
      }
      // Java: inputStream.skip((long)maxLength);
      if maxLength > 0 { inputStream.skip(maxLength as i64); }
      // Java: return strBuffer.toString();
      return strBuffer;
    }

}
