// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::File::File;
// Java: import java.util.HashMap
use std::collections::HashMap;

// Java field: String INSTITUE
const INSTITUE: &str = "学院";
// Java field: String COLLEGE
const COLLEGE: &str = "大学";
// Java field: String BACHELOR
const BACHELOR: &str = "学士";
// Java field: String UNDERGRADUTE
const UNDERGRADUTE: &str = "本科";
// Java field: String MASTER
const MASTER: &str = "硕士";
// Java field: String DOCTOR
const DOCTOR: &str = "博士";
// Java field: int TYPE_SCHOOL
pub(crate) const TYPE_SCHOOL: i32 = 0;
// Java field: int TYPE_DEGREE
pub(crate) const TYPE_DEGREE: i32 = 1;
// Java field: HashMap<Integer, String> info
static info: std::sync::OnceLock<std::sync::Mutex<HashMap<i32, String>>> = std::sync::OnceLock::new();

// Java: public class TextExtractorTool
pub struct TextExtractorTool {
}

// Java: public class TextExtractorTool
impl TextExtractorTool {
    // Java: TextExtractorTool()
    pub fn new() -> Self {
      // Java: super();
      TextExtractorTool {}
    }

    // Java: static public static HashMap<Integer, String> getPaperInfo(String path, String content)
    pub fn getPaperInfo(_path: &str, _content: &str) -> HashMap<i32, String> {
      // Java: info.clear();

      // Java: File file = new File(path);

      // Java: String fileName = file.getName();

      // Java: String school = splitkey(fileName, 0);

      // Java: String degree = splitkey(fileName, 1);

      // Java: boolean findContent = school == null || degree == null;

      // Java: if (findContent)

      {
         // Java: if (school == null)

         {
            // Java: school = splitkey(content, 0);

         }
         // Java: if (degree == null)

         {
            // Java: degree = splitkey(content, 1);

         }
      }
      // Java: info.put(0, school);

      // Java: info.put(1, degree);

      // Java: return info;

      todo!()
    }

    // Java: static public static HashMap<Integer, String> getPaperInfo(String path)
    pub fn getPaperInfo_from_path(_path: &str) -> Option<HashMap<i32, String>> {
      // Java: File file = new File(path);

      // Java: if (!file.exists())

      {
         // Java: return null;

      }
      // Java: else

      {
         // Java: info.clear();

         // Java: String fileName = file.getName();

         // Java: String school = splitkey(fileName, 0);

         // Java: String degree = splitkey(fileName, 1);

         // Java: boolean findContent = school == null || degree == null;

         // Java: if (findContent)

         {
            // Java: TextExtractor extractor = new TextExtractor(path, (String)null, 500, (ITEListener)null);

            // Java: String result = filterString(extractor.result());

            // Java: if (school == null)

            {
               // Java: school = splitkey(result, 0);

            }
            // Java: if (degree == null)

            {
               // Java: degree = splitkey(result, 1);

            }
         }
         // Java: info.put(0, school);

         // Java: info.put(1, degree);

         // Java: return info;

      }
      todo!()
    }

    // Java: static String filterString(String str)
    pub fn filterString(_str: &str) -> String {
      // Java: char[] chars = str.toCharArray();

      // Java: for(int i = 0; i < chars.length; ++i)

      {
         // Java: char c = chars[i];

         // Java: if (c == 12288)

         {
            // Java: chars[i] = ' ';

         }
         // Java: else if (c > '\uff00' && c < '｟')

         {
            // Java: chars[i] = (char)(c - 'ﻠ');

         }
         // Java: if (c == 160)

         {
            // Java: chars[i] = ' ';

         }
      }
      // Java: str = new String(chars);

      // Java: str = str.replaceAll("\\s", "");

      // Java: return str;

      todo!()
    }

    // Java: static String splitkey(String src, int type)
    pub fn splitkey(_src: &str, _type_val: i32) -> Option<String> {
      // Java: if (src == null)

      {
         // Java: return null;

      }
      // Java: else

      {
         // Java: if (type == 0)

         {
            // Java: int index = src.indexOf("学院");

            // Java: if (index > 0 && index <= 20)

            {
               // Java: return src.substring(0, index + 2);

            }
            // Java: index = src.indexOf("大学");

            // Java: if (index > 0 && index <= 20)

            {
               // Java: return src.substring(0, index + 2);

            }
         }
         // Java: else if (type == 1)

         {
            // Java: int index = src.indexOf("本科");

            // Java: int index1 = src.indexOf("学士");

            // Java: int index_col = Math.min(index, index1);

            // Java: if (index_col < 0)

            {
               // Java: index_col = Math.max(index, index1);

            }
            // Java: int index_master = src.indexOf("硕士");

            // Java: int index_doc = src.indexOf("博士");

            // Java: if (index_col >= 0 && (index_col < index_master || index_master < 0) && (index_col < index_doc || index_doc < 0))

            {
               // Java: return "学士";

            }
            // Java: if (index_master >= 0 && (index_master < index_col || index_col < 0) && (index_master < index_doc || index_doc < 0))

            {
               // Java: return "硕士";

            }
            // Java: if (index_doc >= 0 && (index_doc < index_master || index_master < 0) && (index_doc < index_col || index_col < 0))

            {
               // Java: return "博士";

            }
         }
         // Java: return null;

      }
      todo!()
    }

    // Java: static boolean isEncryptedFile(String path)
    pub fn isEncryptedFile(_path: &str) -> bool {
      // Java: if (path == null)

      {
         // Java: return false;

      }
      // Java: else

      {
         // Java: File file = new File(path);

         // Java: if (!file.exists())

         {
            // Java: return false;

         }
         // Java: else

         {
            // Java: TextExtractor extractor = new TextExtractor(path, (String)null, 1, (ITEListener)null);

            // Java: return extractor.isEncryptedFile();

         }
      }
      todo!()
    }

    // Java: static boolean isWpsSecurityFile(String path)
    pub fn isWpsSecurityFile(_path: &str) -> bool {
      // Java: if (path == null)

      {
         // Java: return false;

      }
      // Java: else

      {
         // Java: File file = new File(path);

         // Java: if (!file.exists())

         {
            // Java: return false;

         }
         // Java: else

         {
            // Java: TextExtractor extractor = new TextExtractor(path, (String)null, 1, (ITEListener)null);

            // Java: return extractor.isWpsSecurityFile(path);

         }
      }
      todo!()
    }

}
