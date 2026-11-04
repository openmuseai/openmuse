// java public class PageEnv
pub struct PageEnv {
    // java public int width;
    pub width: i32,
    // java public int height;
    pub height: i32,
    // java public int marginLeft;
    pub marginLeft: i32,
    // java public int marginRight;
    pub marginRight: i32,
    // java public int marginTop;
    pub marginTop: i32,
    // java public int marginBottom;
    pub marginBottom: i32,
    // java public int orgPageMarginLeft;
    pub orgPageMarginLeft: i32,
    // java public int sectionMarginOffset = 0;
    pub sectionMarginOffset: i32,
    // java public int gridModel = 2;
    pub gridModel: i32,
    // java public int gridHeight;
    pub gridHeight: i32,
    // java public int gridWidth;
    pub gridWidth: i32,
    // java public float charSpaceForGrid;
    pub charSpaceForGrid: f32,
    // java public int minLayoutHeight;
    pub minLayoutHeight: i32,
    // java public int maxLayoutHeight;
    pub maxLayoutHeight: i32,
    // java public boolean allowOverCoreHeight;
    pub allowOverCoreHeight: bool,
    // java public boolean allowBreakLine;
    pub allowBreakLine: bool,
    // java public boolean isFirstPageOfSection;
    pub isFirstPageOfSection: bool,
    // java public boolean isShowCardInfo;
    pub isShowCardInfo: bool,
    // java public int textFlow = 0;
    pub textFlow: i32,
}

// java public class PageEnv
impl PageEnv {
    // java public int getCoreHeight()
    pub fn getCoreHeight(&self) -> i32 {
      // java switch (this.textFlow)
      match self.textFlow {
        // java case 1: case 3: case 5:
        1 | 3 | 5 => {
          // java return this.width - this.marginLeft - this.marginRight;
          return self.width - self.marginLeft - self.marginRight;
        }
        // java case 2: case 4: default:
        2 | 4 | _ => {
          // java return this.height - this.marginTop - this.marginBottom;
          return self.height - self.marginTop - self.marginBottom;
        }
      }
    }

    // java public int getCoreWidth()
    pub fn getCoreWidth(&self) -> i32 {
      // java switch (this.textFlow)
      match self.textFlow {
        // java case 1: case 3: case 5:
        1 | 3 | 5 => {
          // java return this.height - this.marginTop - this.marginBottom;
          return self.height - self.marginTop - self.marginBottom;
        }
        // java case 2: case 4: default:
        2 | 4 | _ => {
          // java return this.width - this.marginLeft - this.marginRight;
          return self.width - self.marginLeft - self.marginRight;
        }
      }
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
    }

    // java public void init()
    pub fn init(&mut self) {
      // java this.width = 0;
      self.width = 0;
      // java this.height = 0;
      self.height = 0;
      // java this.marginLeft = 0;
      self.marginLeft = 0;
      // java this.marginRight = 0;
      self.marginRight = 0;
      // java this.marginTop = 0;
      self.marginTop = 0;
      // java this.marginBottom = 0;
      self.marginBottom = 0;
      // java this.orgPageMarginLeft = 0;
      self.orgPageMarginLeft = 0;
      // java this.sectionMarginOffset = 0;
      self.sectionMarginOffset = 0;
      // java this.minLayoutHeight = 0;
      self.minLayoutHeight = 0;
      // java this.maxLayoutHeight = 0;
      self.maxLayoutHeight = 0;
      // java this.allowOverCoreHeight = false;
      self.allowOverCoreHeight = false;
      // java this.allowBreakLine = false;
      self.allowBreakLine = false;
      // java this.isFirstPageOfSection = false;
      self.isFirstPageOfSection = false;
      // java this.gridModel = 2;
      self.gridModel = 2;
      // java this.gridHeight = 0;
      self.gridHeight = 0;
      // java this.gridWidth = 0;
      self.gridWidth = 0;
      // java this.charSpaceForGrid = 0.0F;
      self.charSpaceForGrid = 0.0;
      // java this.textFlow = 0;
      self.textFlow = 0;
      // java this.isShowCardInfo = false;
      self.isShowCardInfo = false;
    }

    // java public boolean isShowCardInfo()
    pub fn isShowCardInfo(&self) -> bool {
      // java return this.isShowCardInfo;
      return self.isShowCardInfo;
    }

    // java public void setShowCardInfo(boolean showCardInfo)
    pub fn setShowCardInfo(&mut self, showCardInfo: bool) {
      // java this.isShowCardInfo = showCardInfo;
      self.isShowCardInfo = showCardInfo;
    }

}

impl Default for PageEnv {
    fn default() -> Self {
        PageEnv {
            width: 0,
            height: 0,
            marginLeft: 0,
            marginRight: 0,
            marginTop: 0,
            marginBottom: 0,
            orgPageMarginLeft: 0,
            sectionMarginOffset: 0,
            gridModel: 2,
            gridHeight: 0,
            gridWidth: 0,
            charSpaceForGrid: 0.0,
            minLayoutHeight: 0,
            maxLayoutHeight: 0,
            allowOverCoreHeight: false,
            allowBreakLine: false,
            isFirstPageOfSection: false,
            isShowCardInfo: false,
            textFlow: 0,
        }
    }
}

