// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import java.io.IOException
// Java: import java.util.HashSet
// Java: import java.util.Properties

use std::collections::HashMap;
use std::collections::HashSet;

// Java field: Properties sProperties
static sProperties: std::sync::OnceLock<std::sync::Mutex<Option<HashMap<String, String>>>> = std::sync::OnceLock::new();

fn get_sProperties() -> &'static std::sync::Mutex<Option<HashMap<String, String>>> {
    sProperties.get_or_init(|| std::sync::Mutex::new(None))
}

// Java: public class Util
pub struct Util {
}

// Java: public class Util
impl Util {
    // Java: static void loadProperties(String path)
    pub fn loadProperties(_path: &str) {
      // Java: try
      {
         // Java: sProperties.load(new FileInputStream(path));
      }
      // Java: catch (IOException var2)
      {
      }
    }

    // Java: static Properties getConfiguration()
    pub fn getConfiguration() -> HashMap<String, String> {
      // Java: if (sProperties == null)
      {
         // Java: sProperties = new Properties();
         // Java: loadProperties(getMOfficeAppDir() + "/test.properties");
         // Java: loadProperties(System.getProperty("user.home") + "/.test.properties");
      }
      // Java: return sProperties;
      todo!()
    }

    // Java: static String getConfiguration(String key)
    pub fn getConfiguration_by_key(_key: &str) -> Option<String> {
      // Java: return getConfiguration().getProperty(key);
      todo!()
    }

    // Java: static boolean isValidMofficeDir(String path)
    pub fn isValidMofficeDir(_path: &str) -> bool {
      // Java: File file = new File(path + "/assets");
      // Java: if (!file.exists())
      {
         // Java: return false;
      }
      // Java: else
      {
         // Java: file = new File(path + "/res");
         // Java: if (!file.exists())
         {
            // Java: return false;
         }
         // Java: else
         {
            // Java: file = new File(path + "/src");
            // Java: return file.exists();
         }
      }
      todo!()
    }

    // Java: static String getMOfficeAppDir()
    pub fn getMOfficeAppDir() -> String {
      // Java: String dir = System.getenv("MOFFICE_DIR");
      // Java: if (dir == null || dir.length() == 0)
      {
         // Java: dir = System.getProperty("MOFFICE_DIR");
      }
      // Java: if (dir == null || dir.length() == 0)
      {
         // Java: String pwd = System.getProperty("user.dir");
         // Java: for(File dir1 = new File(pwd); dir1 != null; dir1 = dir1.getParentFile())
         {
            // Java: String[] children1 = dir1.list();
            // Java: HashSet<String> children = new HashSet();
            // Java: for(int i = 0; i < children1.length; ++i)
            {
               // Java: children.add(children1[i]);
            }
            // Java: if (children.contains("moffice") && children.contains("libs") && children.contains("base") && isValidMofficeDir(dir1.getAbsolutePath() + "/moffice"))
            {
               // Java: dir = dir1.getAbsolutePath();
               // Java: break;
            }
         }
      }
      // Java: if (dir != null && dir.length() != 0)
      {
         // Java: return dir;
      }
      // Java: else
      {
         // Java: throw new RuntimeException("Please set a valid MOFFICE_DIR");
      }
      todo!()
    }

}
