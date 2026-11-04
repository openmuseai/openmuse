// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;

// Java field: String pathForXmlSchema
static PATH_FOR_XML_SCHEMA: std::sync::OnceLock<std::sync::Mutex<String>> = std::sync::OnceLock::new();

// Java: public class Configuration
pub struct Configuration {
}

// Java: public class Configuration
impl Configuration {
    // Java field: String pathForXmlSchema

    // Java: Configuration()
    pub fn new() -> Self {
      // Java: super();
      Configuration {}
    }

    // Java: static String getPathForXmlSchema()
    pub fn getPathForXmlSchema() -> String {
      // Java: return pathForXmlSchema;
      let lock = PATH_FOR_XML_SCHEMA.get_or_init(|| std::sync::Mutex::new(String::new()));
      lock.lock().unwrap().clone()
    }

    // Java: static void setPathForXmlSchema(String pathForXmlSchema)
    pub fn setPathForXmlSchema(path: &str) {
      // Java: Configuration.pathForXmlSchema = pathForXmlSchema;
      let lock = PATH_FOR_XML_SCHEMA.get_or_init(|| std::sync::Mutex::new(String::new()));
      *lock.lock().unwrap() = path.to_string();
    }
}
