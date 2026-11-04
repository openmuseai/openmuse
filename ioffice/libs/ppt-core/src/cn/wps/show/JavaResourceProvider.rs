#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import java.io.FileInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.net.URL
use std::path::Path;
use std::fs;

pub type InputStream = Vec<u8>;

fn get_resource(path: &str) -> Option<String> {
    let full_path = Path::new(path);
    if full_path.exists() { Some(path.to_string()) } else { None }
}

fn open_file(path: &str) -> Result<InputStream, std::io::Error> {
    fs::read(path)
}

pub trait ResourcePrivider {
    fn getTheme(&self) -> Result<InputStream, std::io::Error>;
    fn getThemePath(&self) -> Option<String>;
    fn getSlideLayout(&self, slideSizeType: i32) -> Result<InputStream, std::io::Error>;
    fn getSlideMaster(&self, slideSizeType: i32) -> Result<InputStream, std::io::Error>;
    fn getNotesMaster(&self) -> Result<InputStream, std::io::Error>;
    fn getViewProps(&self) -> Result<InputStream, std::io::Error>;
    fn getBlankPptx(&self) -> Result<InputStream, std::io::Error>;
}

// Java: public class JavaResourceProvider implements PptResourceFactory.ResourcePrivider
pub struct JavaResourceProvider {
}

// Java: public class JavaResourceProvider implements PptResourceFactory.ResourcePrivider
impl JavaResourceProvider {
    // Java: InputStream getTheme()
    pub fn getTheme(&self) -> Result<InputStream, std::io::Error> {
      // Java: URL url = KmoBootstrap.class.getResource("/dev/ppt/theme1.xml");
      let url = get_resource("/dev/ppt/theme1.xml");
      // Java: return url == null ? null : new FileInputStream(url.getPath());
      match url { Some(path) => open_file(&path), None => Ok(Vec::new()) }
    }

    // Java: String getThemePath()
    pub fn getThemePath(&self) -> Option<String> {
      // Java: URL url = KmoBootstrap.class.getResource("/dev/ppt/theme1.xml");
      let url = get_resource("/dev/ppt/theme1.xml");
      // Java: return url == null ? null : url.getPath();
      url
    }

    // Java: InputStream getSlideLayout(int slideSizeType)
    pub fn getSlideLayout(&self, slideSizeType: i32) -> Result<InputStream, std::io::Error> {
      // Java: String slideLayoutXmlPath = "/dev/ppt/slideLayout1_4x3.xml";
      let mut slideLayoutXmlPath = "/dev/ppt/slideLayout1_4x3.xml";
      // Java: switch (slideSizeType)
      match slideSizeType {
         // Java: case 0:
         0 => {
            // Java: slideLayoutXmlPath = "/dev/ppt/slideLayout1_4x3.xml";
            slideLayoutXmlPath = "/dev/ppt/slideLayout1_4x3.xml";
            // Java: break;
         }
         // Java: case 15:
         15 => {
            // Java: slideLayoutXmlPath = "/dev/ppt/slideLayout1_16x9.xml";
            slideLayoutXmlPath = "/dev/ppt/slideLayout1_16x9.xml";
         }
         _ => {}
      }
      // Java: URL url = KmoBootstrap.class.getResource(slideLayoutXmlPath);
      let url = get_resource(slideLayoutXmlPath);
      // Java: return url == null ? null : new FileInputStream(url.getPath());
      match url { Some(path) => open_file(&path), None => Ok(Vec::new()) }
    }

    // Java: InputStream getSlideMaster(int slideSizeType)
    pub fn getSlideMaster(&self, slideSizeType: i32) -> Result<InputStream, std::io::Error> {
      // Java: String slideMasterXmlPath = "/dev/ppt/slideMaster1_4x3.xml";
      let mut slideMasterXmlPath = "/dev/ppt/slideMaster1_4x3.xml";
      // Java: switch (slideSizeType)
      match slideSizeType {
         // Java: case 0:
         0 => {
            // Java: slideMasterXmlPath = "/dev/ppt/slideMaster1_4x3.xml";
            slideMasterXmlPath = "/dev/ppt/slideMaster1_4x3.xml";
            // Java: break;
         }
         // Java: case 15:
         15 => {
            // Java: slideMasterXmlPath = "/dev/ppt/slideMaster1_16x9.xml";
            slideMasterXmlPath = "/dev/ppt/slideMaster1_16x9.xml";
         }
         _ => {}
      }
      // Java: URL url = KmoBootstrap.class.getResource(slideMasterXmlPath);
      let url = get_resource(slideMasterXmlPath);
      // Java: return url == null ? null : new FileInputStream(url.getPath());
      match url { Some(path) => open_file(&path), None => Ok(Vec::new()) }
    }

    // Java: InputStream getNotesMaster()
    pub fn getNotesMaster(&self) -> Result<InputStream, std::io::Error> {
      // Java: URL url = KmoBootstrap.class.getResource("/dev/ppt/notesMaster1.xml");
      let url = get_resource("/dev/ppt/notesMaster1.xml");
      // Java: return url == null ? null : new FileInputStream(url.getPath());
      match url { Some(path) => open_file(&path), None => Ok(Vec::new()) }
    }

    // Java: InputStream getViewProps()
    pub fn getViewProps(&self) -> Result<InputStream, std::io::Error> {
      // Java: URL url = KmoBootstrap.class.getResource("/dev/ppt/viewProps.xml");
      let url = get_resource("/dev/ppt/viewProps.xml");
      // Java: return url == null ? null : new FileInputStream(url.getPath());
      match url { Some(path) => open_file(&path), None => Ok(Vec::new()) }
    }

    // Java: InputStream getBlankPptx()
    pub fn getBlankPptx(&self) -> Result<InputStream, std::io::Error> {
      // Java: URL url = KmoBootstrap.class.getResource("/dev/ppt/template/blank.pptx");
      let url = get_resource("/dev/ppt/template/blank.pptx");
      // Java: return url == null ? null : new FileInputStream(url.getPath());
      match url { Some(path) => open_file(&path), None => Ok(Vec::new()) }
    }

}

// Java: implements PptResourceFactory.ResourcePrivider
impl ResourcePrivider for JavaResourceProvider {
    fn getTheme(&self) -> Result<InputStream, std::io::Error> { JavaResourceProvider::getTheme(self) }
    fn getThemePath(&self) -> Option<String> { JavaResourceProvider::getThemePath(self) }
    fn getSlideLayout(&self, slideSizeType: i32) -> Result<InputStream, std::io::Error> { JavaResourceProvider::getSlideLayout(self, slideSizeType) }
    fn getSlideMaster(&self, slideSizeType: i32) -> Result<InputStream, std::io::Error> { JavaResourceProvider::getSlideMaster(self, slideSizeType) }
    fn getNotesMaster(&self) -> Result<InputStream, std::io::Error> { JavaResourceProvider::getNotesMaster(self) }
    fn getViewProps(&self) -> Result<InputStream, std::io::Error> { JavaResourceProvider::getViewProps(self) }
    fn getBlankPptx(&self) -> Result<InputStream, std::io::Error> { JavaResourceProvider::getBlankPptx(self) }
}
