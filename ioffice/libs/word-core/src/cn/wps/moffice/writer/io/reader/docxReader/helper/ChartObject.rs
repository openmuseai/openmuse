// java public class ChartObject
#[derive(Default)]
pub struct ChartObject {
    // java private String contentType;
    contentType: Option<String>,
    // java private String relationID;
    relationID: Option<String>,
    // java private String path;
    path: Option<String>,
    // java public ChartObject[] chartObjects;
    pub(crate) chartObjects: Option<Vec<ChartObject>>,
}

// java public class ChartObject
impl ChartObject {
    // java public  ChartObject(String contentType, String relationID, String path)
    pub fn ChartObject(contentType: &str, relationID: &str, path: &str) -> Self {
      // java this.contentType = contentType;
      Self { contentType: contentType, relationID: relationID, path: path, chartObjects: Default::default() }
      // java this.relationID = relationID;
      // java this.path = path;
    }

    // java public String getId()
    pub fn getId(&self) -> String {
      // java return this.relationID;
      return self.relationID;
    }

    // java public String getContentType()
    pub fn getContentType(&self) -> String {
      // java return this.contentType;
      return self.contentType;
    }

    // java public String getPath()
    pub fn getPath(&self) -> String {
      // java return this.path;
      return self.path;
    }

}
