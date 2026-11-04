// java import cn.wps.kfc.xpxml.compactxml.CompactCoreParser
use crate::kfc::xpxml::compactxml::CompactCoreParser::CompactCoreParser;
// java import java.io.IOException
// java import java.io.InputStream
use std::io::Read;
// java import java.io.InputStreamReader
// java import java.text.ParseException
// java import java.text.SimpleDateFormat
// java import java.util.Date
// java import java.util.TimeZone

// java public class OpcUtil
pub struct OpcUtil {
}

// java public class OpcUtil
impl OpcUtil {
    // java OpcUtil()
    pub fn new() -> Self {
      // java super();
      OpcUtil {}
    }

    // java static String getRelsesName(String partName)
    pub fn getRelsesName(partName: &str) -> String {
      // java if (partName == null)
      // rust: &str cannot be null; empty parent is the package root `_rels/.rels`.
      {
         let mut part = partName;
         // java if (partName.startsWith("/"))
         if part.starts_with('/')
         {
            // java partName = partName.substring(1);
            part = &part[1..];
         }
         // java int index = partName.lastIndexOf("/");
         let index = part.rfind('/');
         // java if (index != -1)
         if let Some(idx) = index
         {
            // java StringBuilder ret = new StringBuilder();
            let mut ret = String::new();
            // java ret.append(partName.substring(0, index));
            ret.push_str(&part[..idx]);
            // java ret.append("/_rels");
            ret.push_str("/_rels");
            // java ret.append(partName.substring(index));
            ret.push_str(&part[idx..]);
            // java ret.append(".rels");
            ret.push_str(".rels");
            // java return ret.toString();
            return ret;
         }
         // java else
         else
         {
            // java StringBuilder ret = new StringBuilder();
            let mut ret = String::new();
            // java ret.append("_rels/");
            ret.push_str("_rels/");
            // java ret.append(partName);
            ret.push_str(part);
            // java ret.append(".rels");
            ret.push_str(".rels");
            // java return ret.toString();
            return ret;
         }
      }
    }

    // java static String getRelsTarget(String parentPartName, String partName)
    pub fn getRelsTarget(parentPartName: Option<&str>, partName: Option<&str>) -> String {
      // java if (parentPartName != null && partName != null)
      if let (Some(parentPartName),Some(partName))=(parentPartName,partName)
      {
         let mut parent = parentPartName.to_string();
         let mut part = partName.to_string();
         // java if (parentPartName.startsWith("/"))
         if parent.starts_with('/')
         {
            // java parentPartName = parentPartName.substring(1);
            parent = parent[1..].to_string();
         }
         // java if (partName.startsWith("/"))
         if part.starts_with('/')
         {
            // java partName = partName.substring(1);
            part = part[1..].to_string();
         }
         // java int index = parentPartName.lastIndexOf("/");
         let index = parent.rfind('/');
         // java if (index == -1)
         if index.is_none()
         {
            // java return partName;
            return part;
         }
         // java else
         else
         {
            let idx = index.unwrap();
            // java String tmp = parentPartName.substring(0, index + 1);
            let tmp = &parent[..idx + 1];
            // java if (partName.startsWith(tmp))
            if part.starts_with(tmp)
            {
               // java return partName.substring(index + 1);
               return part[idx + 1..].to_string();
            }
            // java else
            else
            {
               // java index = tmp.lastIndexOf("/", tmp.length() - 2);
               let tmp_owned = tmp.to_string();
               let index2 = tmp_owned[..tmp_owned.len() - 1].rfind('/');
               // java tmp = tmp.substring(0, index + 1);
               let tmp2 = match index2 { Some(i) => &tmp_owned[..i + 1], None => "" };
               // java if (partName.startsWith(tmp))
               if part.starts_with(tmp2)
               {
                  // java return "../" + partName.substring(index + 1);
                  return format!("../{}", &part[index2.map_or(0,|i|i+1)..]);
               }
               // java else
               else
               {
                  // java throw new IllegalArgumentException("The wrong arguments: " + parentPartName + ", " + partName);
                  panic!("IllegalArgumentException: The wrong arguments: {}, {}", parentPartName, partName);
               }
            }
         }
      }
      // java else
      else
      {
         // java throw new IllegalArgumentException("The argument should not null.");
         panic!("IllegalArgumentException: The argument should not null.");
      }
    }

    // java static String getPartName(String parentPartDir, String relsTarget)
    pub fn getPartName(parentPartDir: Option<&str>, relsTarget: Option<&str>) -> String {
      let relsTarget=relsTarget.expect("NullPointerException");
      // java if (relsTarget.startsWith("/"))
      if relsTarget.starts_with('/')
      {
         // java return relsTarget;
         return relsTarget.to_string();
      }
      // java else if (parentPartDir == null)
      else if parentPartDir.is_none()
      {
         // java throw new IllegalArgumentException("The argument should not null.");
         panic!("IllegalArgumentException: The argument should not null.");
      }
      // java else
      else
      {
         let parentPartDir=parentPartDir.unwrap();let mut dir = parentPartDir.to_string();
         // java if (parentPartDir.startsWith("/"))
         if dir.starts_with('/')
         {
            // java parentPartDir = parentPartDir.substring(1);
            dir = dir[1..].to_string();
         }
         // java if (relsTarget.startsWith("../"))
         if relsTarget.starts_with("../")
         {
            // java int index = parentPartDir.lastIndexOf("/", parentPartDir.length() - 2);
            let search_end=dir.encode_utf16().count()as i32-2;
            let mut units=0i32;let mut index=None;
            for (byte,ch)in dir.char_indices(){if units>search_end{break;}if ch=='/'{index=Some(byte);}units+=ch.len_utf16()as i32;}
            // java if (index != -1)
            if let Some(idx) = index
            {
               // java return parentPartDir.substring(0, index + 1) + relsTarget.substring(3);
               return format!("{}{}", &dir[..idx + 1], &relsTarget[3..]);
            }
            // java else if (parentPartDir.endsWith("/"))
            else if dir.ends_with('/')
            {
               // java return relsTarget.substring(3);
               return relsTarget[3..].to_string();
            }
            // java else
            else
            {
               // java throw new IllegalStateException(parentPartDir + " and " + relsTarget + " is not match.");
               panic!("IllegalStateException: {} and {} is not match.", dir, relsTarget);
            }
         }
         // java else
         else
         {
            // java return parentPartDir + relsTarget;
            return format!("{}{}", dir, relsTarget);
         }
      }
    }

    // java static String getPartNamex(String parentPartName, String relsTarget)
    pub fn getPartNamex(parentPartName: &str, relsTarget: &str) -> String {
      // java if (parentPartName == null)
      // rust: empty parent is package-root rels; Java only throws on null.
      {
         let mut parent = parentPartName;
         // java if (parentPartName.startsWith("/"))
         if parent.starts_with('/')
         {
            // java parentPartName = parentPartName.substring(1);
            parent = &parent[1..];
         }
         // java int index = parentPartName.lastIndexOf("/");
         let index = parent.rfind('/');
         // java return index != -1 ? getPartName(parentPartName.substring(0, index + 1), relsTarget) : relsTarget;
         match index {
             Some(idx) => Self::getPartName(Some(&parent[..idx + 1]), Some(relsTarget)),
             None => relsTarget.to_string(),
         }
      }
    }

    // java static String getCurrentDate()
    pub fn getCurrentDate() -> String {
      // java SimpleDateFormat df = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss'Z'");
      // java df.setTimeZone(TimeZone.getTimeZone("UTC"));
      // java return df.format(new Date());
      let millis=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("Date epoch").as_millis()as i64;
      super::nativeDate::format(millis)
    }

    // java static String getDateString(Date d)
    pub fn getDateString(d: i64) -> String {
      // java SimpleDateFormat df = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss'Z'");
      // java df.setTimeZone(TimeZone.getTimeZone("UTC"));
      // java return df.format(d);
      super::nativeDate::format(d)
    }

    // java static Date getDate(String str)
    pub fn getDate(str: &str) -> Option<i64> {
      // java SimpleDateFormat df = new SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss'Z'");
      // java try
      // java return df.parse(str);
      // java catch (ParseException e)
      // java e.printStackTrace();
      // java return null;
      let result=super::nativeDate::parse(str);
      if result.is_none(){eprintln!("ParseException: Unparseable date: {str:?}");}
      result
    }

    // java static void parserXml(CompactCoreParser parser, InputStream input)
    pub fn parserXml(parser: &mut CompactCoreParser, input: &mut dyn Read) {
      // java InputStreamReader reader = null;
      // java try
      {
         // java reader = new InputStreamReader(input, "UTF-8");
         // java int bufferSize = 8192;
         let buffer_size = 8192;
         // java if (bufferSize > input.available())
         // java bufferSize = input.available();
         // java char[] buffer = new char[bufferSize];
         let mut buffer = vec![0u8; buffer_size];
         // java int length = 0;
         // java while((length = reader.read(buffer, 0, bufferSize)) > 0)
         loop {
            let length = input.read(&mut buffer).unwrap_or(0);
            if length == 0 { break; }
            {
               // java parser.parse(buffer, length);
               let chars: Vec<char> = String::from_utf8_lossy(&buffer[..length]).chars().collect();
               parser.parse_buf(&chars, chars.len());
            }
         }
      }
      // java finally
      {
         // java if (reader != null)
         {
            // java reader.close();
         }
      }
    }

}
