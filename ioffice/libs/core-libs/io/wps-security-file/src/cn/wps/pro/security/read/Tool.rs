#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.moffice.util.MD5Util
// use util::cn::wps::moffice::util::MD5Util;
// Java: import cn.wps.ole.IOLEStream
// use util_adapter::cn::wps::ole::IOLEStream;
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.UnsupportedEncodingException
// Java: import java.util.Iterator
// Java: import org.apache.poi.EncryptedDocumentException
// use io_poi_compact::org::apache::poi::EncryptedDocumentException;
// Java: import org.apache.poi.poifs.filesystem.DirectoryEntry
// use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryEntry;
// Java: import org.apache.poi.poifs.filesystem.DocumentEntry
// use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentEntry;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.Entry
// use io_poi_compact::org::apache::poi::poifs::filesystem::Entry;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
// use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;
// Java: import org.apache.poi.util.LittleEndian
// use util::org::apache::poi::util::LittleEndian;
// Java: import org.apache.poi.util.LittleEndianByteArrayOutputStream
// use util::org::apache::poi::util::LittleEndianByteArrayOutputStream;

use std::io::{self, Read};

// Java: public class Tool
pub struct Tool {
}

// Java: public class Tool
impl Tool {
    // Java: Tool()
    pub fn new() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static String parsedByteToStr(byte[] data, int length)
    pub fn parsedByteToStr(data: &[u8], length: i32) -> String {
      // Java: int size = length / 2;
      let size = length / 2;
      // Java: char[] src = new char[size];
      let mut src = vec!['\0'; size as usize];
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size
      {
         // Java: src[i] = (char)LittleEndian.getShort(data, 2 * i);
         let offset = (2 * i) as usize;
         let short_val = (data[offset] as u16) | ((data[offset + 1] as u16) << 8);
         src[i as usize] = char::from_u32(short_val as u32).unwrap_or('\0');
      }
      // Java: return new String(src);
      src.into_iter().collect()
    }

    // Java: static char[] parsedByteToCharBuffer(byte[] data, int length)
    pub fn parsedByteToCharBuffer(data: &[u8], length: i32) -> Vec<char> {
      // Java: int size = length / 2;
      let size = length / 2;
      // Java: char[] src = new char[size];
      let mut src = vec!['\0'; size as usize];
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size
      {
         // Java: src[i] = (char)LittleEndian.getShort(data, 2 * i);
         let offset = (2 * i) as usize;
         let short_val = (data[offset] as u16) | ((data[offset + 1] as u16) << 8);
         src[i as usize] = char::from_u32(short_val as u32).unwrap_or('\0');
      }
      // Java: return src;
      src
    }

    // Java: static int readShort(IOLEStream dis)
    pub fn readShort(_dis: &mut dyn std::any::Any) -> i32 {
      // Java: byte[] data = new byte[4];
      let mut data = [0u8; 4];
      // Java: dis.read(data, 0, 4);
      // Java: return LittleEndian.getShort(data, 0);
      todo!()
    }

    // Java: static int readInt(IOLEStream dis)
    pub fn readInt_tool(_dis: &mut dyn std::any::Any) -> i32 {
      // Java: byte[] data = new byte[4];
      let mut data = [0u8; 4];
      // Java: dis.read(data, 0, 4);
      // Java: return LittleEndian.getInt(data, 0);
      todo!()
    }

    // Java: static String getString(IOLEStream dis)
    pub fn getString(_dis: &mut dyn std::any::Any) -> String {
      // Java: int SIZE = 1024;
      let _SIZE: i32 = 1024;
      // Java: byte[] buff = new byte[1024];
      let mut buff = vec![0u8; 1024];
      // Java: StringBuffer buffer = new StringBuffer();
      let mut buffer = String::new();
      // Java: int n = 0;
      let mut n: i32 = 0;
      // Java: try
      {
         // Java: while(true)
         loop
         {
            // Java: n = dis.read(buff, 0, 1024);
            n = 0;
            // Java: if (0 == n)
            if 0 == n
            {
               // Java: break;
               break;
            }
            // Java: buffer.append(parsedByteToCharBuffer(buff, n));
            let chars = Self::parsedByteToCharBuffer(&buff, n);
            buffer.extend(chars.iter());
         }
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
      }
      // Java: return buff.toString();
      buffer
    }

    // Java: static String readString(DocumentInputStream dis)
    pub fn readString(_dis: &mut dyn std::any::Any) -> String {
      // Java: int len = dis.readInt();
      // Java: byte[] buff = new byte[len];
      // Java: dis.read(buff);
      // Java: return new String(buff);
      todo!()
    }

    // Java: static String readUNICODELPP4(DocumentInputStream dis)
    pub fn readUNICODELPP4(_dis: &mut dyn std::any::Any) -> String {
      // Java: int len = dis.readInt();
      // Java: byte[] buff = new byte[getUnicodeLength4(len)];
      // Java: dis.read(buff);
      // Java: return parsedByteToStr(buff, len);
      todo!()
    }

    // Java: static int getUNICODELPP4(String value)
    pub fn getUNICODELPP4_size(value: &str) -> i32 {
      // Java: return 4 + getUnicodeLength4(getStringByte(value).length);
      4 + Self::getUnicodeLength4(Self::getStringByte(value).len() as i32)
    }

    // Java: static byte[] getStringByte(String value)
    pub fn getStringByte(value: &str) -> Vec<u8> {
      // Java: try
      {
         // Java: byte[] bytes = value.getBytes("UTF-16LE");
         let bytes: Vec<u8> = value.encode_utf16()
             .flat_map(|c| c.to_le_bytes())
             .collect();
         // Java: return bytes;
         return bytes;
      }
      // Java: catch (UnsupportedEncodingException var3)
      {
         // Java: throw new EncryptedDocumentException("UTF16 not supported");
         panic!("UTF16 not supported");
      }
    }

    // Java: static void writeUNICODELPP4(LittleEndianByteArrayOutputStream outputStream, String value)
    pub fn writeUNICODELPP4(_outputStream: &mut dyn std::any::Any, value: &str) {
      // Java: byte[] bytes = getStringByte(value);
      let bytes = Self::getStringByte(value);
      // Java: int size = bytes.length;
      let size = bytes.len() as i32;
      // Java: int multiple = getUnicodeLength4(size);
      let multiple = Self::getUnicodeLength4(size);
      // Java: byte[] newBytes = new byte[multiple];
      let mut newBytes = vec![0u8; multiple as usize];
      // Java: System.arraycopy(bytes, 0, newBytes, 0, size);
      newBytes[..size as usize].copy_from_slice(&bytes);
      // Java: outputStream.writeInt(size);
      // Java: outputStream.write(newBytes);
      todo!()
    }

    // Java: static int getUnicodeLength4(int size)
    pub fn getUnicodeLength4(size: i32) -> i32 {
      // Java: int multiple = size;
      let mut multiple = size;
      // Java: if (size % 4 != 0)
      if size % 4 != 0
      {
         // Java: multiple = (size / 4 + 1) * 4;
         multiple = (size / 4 + 1) * 4;
      }
      // Java: return multiple;
      multiple
    }

    // Java: static String getMD5(InputStream fileInputStream)
    pub fn getMD5(_fileInputStream: &mut dyn Read) -> String {
      // Java: return MD5Util.getMD5(fileInputStream);
      todo!()
    }

    // Java: static void copyNodes(POIFSFileSystem source, POIFSFileSystem target)
    pub fn copyNodes(_source: &mut dyn std::any::Any, _target: &mut dyn std::any::Any) {
      // Java: try
      {
         // Java: DirectoryEntry root = source.getRoot();
         // Java: DirectoryEntry newRoot = target.getRoot();
         // Java: Iterator<Entry> entries = root.getEntries();
         // Java: while(entries.hasNext())
         {
            // Java: Entry entry = (Entry)entries.next();
            // Java: copyNodeRecursively(entry, newRoot);
         }
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
      }
      todo!()
    }

    // Java: static void copyNodeRecursively(Entry entry, DirectoryEntry target)
    pub fn copyNodeRecursively(_entry: &dyn std::any::Any, _target: &mut dyn std::any::Any) {
      // Java: DirectoryEntry newTarget = null;
      // Java: if (entry.isDirectoryEntry())
      {
         // Java: newTarget = target.createDirectory(entry.getName());
         // Java: Iterator<Entry> entries = ((DirectoryEntry)entry).getEntries();
         // Java: while(entries.hasNext())
         {
            // Java: copyNodeRecursively((Entry)entries.next(), newTarget);
         }
      }
      // Java: else
      {
         // Java: DocumentEntry dentry = (DocumentEntry)entry;
         // Java: DocumentInputStream dstream = new DocumentInputStream(dentry);
         // Java: target.createDocument(dentry.getName(), dstream, true);
         // Java: dstream.close();
      }
      todo!()
    }

}
