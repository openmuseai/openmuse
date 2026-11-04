#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]

// Java: import android.content.Context
// Java: import java.io.IOException
// Java: import java.io.InputStream

pub type InputStream = Vec<u8>;

struct AssetManager;
impl AssetManager {
    fn open(&self, path: &str) -> Result<InputStream, std::io::Error> {
        Ok(Vec::new())
    }
}

pub struct Context;
impl Context {
    fn getAssets(&self) -> AssetManager {
        AssetManager
    }
}

pub mod PptResourceFactory {
    use super::InputStream;

    pub trait ResourcePrivider {
        fn getTheme(&self) -> Result<InputStream, std::io::Error>;
        fn getThemePath(&self) -> String;
        fn getSlideLayout(&self, slideSizeType: i32) -> Result<InputStream, std::io::Error>;
        fn getSlideMaster(&self, slideSizeType: i32) -> Result<InputStream, std::io::Error>;
        fn getNotesMaster(&self) -> Result<InputStream, std::io::Error>;
        fn getViewProps(&self) -> Result<InputStream, std::io::Error>;
        fn getBlankPptx(&self) -> Result<InputStream, std::io::Error>;
    }
}

// Java: public class AndroidResourceProvider implements PptResourceFactory.ResourcePrivider
pub struct AndroidResourceProvider {
    // Java field: Context mContext
    pub(crate) mContext: Context,
}

// Java: public class AndroidResourceProvider implements PptResourceFactory.ResourcePrivider
impl AndroidResourceProvider {
    // Java: AndroidResourceProvider(Context context)
    pub fn new(context: Context) -> Self {
      // Java: this.mContext = context;
      AndroidResourceProvider { mContext: context }
    }

    // Java: InputStream getTheme()
    pub fn getTheme(&self) -> Result<InputStream, std::io::Error> {
      // Java: return this.mContext.getAssets().open("dev/ppt/theme1.xml");
      self.mContext.getAssets().open("dev/ppt/theme1.xml")
    }

    // Java: String getThemePath()
    pub fn getThemePath(&self) -> String {
      // Java: return "assets:dev/ppt/theme1.xml";
      "assets:dev/ppt/theme1.xml".to_string()
    }

    // Java: InputStream getSlideLayout(int slideSizeType)
    pub fn getSlideLayout(&self, slideSizeType: i32) -> Result<InputStream, std::io::Error> {
      // Java: String slideLayoutXmlPath = "dev/ppt/slideLayout1_4x3.xml";
      let mut slideLayoutXmlPath = "dev/ppt/slideLayout1_4x3.xml";
      // Java: switch (slideSizeType)
      match slideSizeType {
         // Java: case 0:
         0 => {
            // Java: slideLayoutXmlPath = "dev/ppt/slideLayout1_4x3.xml";
            slideLayoutXmlPath = "dev/ppt/slideLayout1_4x3.xml";
            // Java: break;
         }
         // Java: case 15:
         15 => {
            // Java: slideLayoutXmlPath = "dev/ppt/slideLayout1_16x9.xml";
            slideLayoutXmlPath = "dev/ppt/slideLayout1_16x9.xml";
         }
         _ => {}
      }
      // Java: return this.mContext.getAssets().open(slideLayoutXmlPath);
      self.mContext.getAssets().open(slideLayoutXmlPath)
    }

    // Java: InputStream getSlideMaster(int slideSizeType)
    pub fn getSlideMaster(&self, slideSizeType: i32) -> Result<InputStream, std::io::Error> {
      // Java: String slideMasterXmlPath = "dev/ppt/slideMaster1_4x3.xml";
      let mut slideMasterXmlPath = "dev/ppt/slideMaster1_4x3.xml";
      // Java: switch (slideSizeType)
      match slideSizeType {
         // Java: case 0:
         0 => {
            // Java: slideMasterXmlPath = "dev/ppt/slideMaster1_4x3.xml";
            slideMasterXmlPath = "dev/ppt/slideMaster1_4x3.xml";
            // Java: break;
         }
         // Java: case 15:
         15 => {
            // Java: slideMasterXmlPath = "dev/ppt/slideMaster1_16x9.xml";
            slideMasterXmlPath = "dev/ppt/slideMaster1_16x9.xml";
         }
         _ => {}
      }
      // Java: return this.mContext.getAssets().open(slideMasterXmlPath);
      self.mContext.getAssets().open(slideMasterXmlPath)
    }

    // Java: InputStream getNotesMaster()
    pub fn getNotesMaster(&self) -> Result<InputStream, std::io::Error> {
      // Java: return this.mContext.getAssets().open("dev/ppt/notesMaster1.xml");
      self.mContext.getAssets().open("dev/ppt/notesMaster1.xml")
    }

    // Java: InputStream getViewProps()
    pub fn getViewProps(&self) -> Result<InputStream, std::io::Error> {
      // Java: return this.mContext.getAssets().open("dev/ppt/viewProps.xml");
      self.mContext.getAssets().open("dev/ppt/viewProps.xml")
    }

    // Java: InputStream getBlankPptx()
    pub fn getBlankPptx(&self) -> Result<InputStream, std::io::Error> {
      // Java: return this.mContext.getAssets().open("template/default/ppt/blank.pptx");
      self.mContext.getAssets().open("template/default/ppt/blank.pptx")
    }

}

impl PptResourceFactory::ResourcePrivider for AndroidResourceProvider {
    fn getTheme(&self) -> Result<InputStream, std::io::Error> { self.getTheme() }
    fn getThemePath(&self) -> String { self.getThemePath() }
    fn getSlideLayout(&self, slideSizeType: i32) -> Result<InputStream, std::io::Error> { self.getSlideLayout(slideSizeType) }
    fn getSlideMaster(&self, slideSizeType: i32) -> Result<InputStream, std::io::Error> { self.getSlideMaster(slideSizeType) }
    fn getNotesMaster(&self) -> Result<InputStream, std::io::Error> { self.getNotesMaster() }
    fn getViewProps(&self) -> Result<InputStream, std::io::Error> { self.getViewProps() }
    fn getBlankPptx(&self) -> Result<InputStream, std::io::Error> { self.getBlankPptx() }
}
