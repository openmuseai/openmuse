// Java: import java.util.HashMap
use stubs::java::util::HashMap::*;
// Java: import java.util.Iterator
use stubs::java::util::Iterator::*;
// Java: import java.util.Map
use stubs::java::util::Map::*;

// Java field:    public static final short PRINT_TO_VIRTUAL = 0;
pub const PRINT_TO_VIRTUAL: i16 = 0;
// Java field:    public static final short PRINT_TO_PICTURE = 1;
pub const PRINT_TO_PICTURE: i16 = 1;
// Java field:    public static final short PRINT_TO_PDF = 2;
pub const PRINT_TO_PDF: i16 = 2;
// Java field:    public static final short PRINT_TO_PRINTER = 3;
pub const PRINT_TO_PRINTER: i16 = 3;
// Java field:    public static final short PRINT_TO_PS = 4;
pub const PRINT_TO_PS: i16 = 4;
// Java field:    public static final short PRINT_FILE_FAIL = 0;
pub const PRINT_FILE_FAIL: i16 = 0;
// Java field:    public static final short PRINT_FILE_EMPTY = 1;
pub const PRINT_FILE_EMPTY: i16 = 1;
// Java field:    public static final short PRINT_FILE_OK = 2;
pub const PRINT_FILE_OK: i16 = 2;
// Java field:    public static final short PRINT_FILE_CANCEL = 3;
pub const PRINT_FILE_CANCEL: i16 = 3;
// Java field:    public static final short PRINT_FILE_SENDED = 4;
pub const PRINT_FILE_SENDED: i16 = 4;
// Java field:    public static final short PRINT_WORKBOOK = 0;
pub const PRINT_WORKBOOK: i16 = 0;
// Java field:    public static final short PRINT_WORKSHEET = 1;
pub const PRINT_WORKSHEET: i16 = 1;
// Java field:    public static final short PRINT_SHEETAREA = 2;
pub const PRINT_SHEETAREA: i16 = 2;
// Java field:    public static final HashMap<Integer, PageSize> mPages = new HashMap();
pub static mPages: std::sync::LazyLock<HashMap<i32, PageSize>> =
                    std::sync::LazyLock::new(|| HashMap::new());
// Java field:       private static final int OFFSET = 10;
const OFFSET: i32 = 10;

// Java: public class PrintOptionDef 
#[derive(Default)]
pub struct PrintOptionDef {
}

// Java: public static class PageSize 
#[derive(Default)]
pub struct PageSize {
    // Java field:       public int mIndex;
    pub mIndex: i32,
    // Java field:       public String mPageName;
    pub mPageName: String,
    // Java field:       public float mWidth;
    pub mWidth: f32,
    // Java field:       public float mHeigth;
    pub mHeigth: f32,
}

// Java: public class PrintOptionDef 
impl PrintOptionDef {
    // Java: public PrintOptionDef() 
    pub fn new() -> PrintOptionDef {
      Self {}
    }

    // Java: public static PageSize getPageSizeByIndex(int index) 
    pub fn getPageSizeByIndex(index: i32) -> PageSize {
      // Java: return (PageSize)mPages.get(index);
      return ((mPages.get(index)) as PageSize);
    }

    // Java: public static PageSize getA4() 
    pub fn getA4() -> PageSize {
      // Java: return (PageSize)mPages.get(9);
      return ((mPages.get(9)) as PageSize);
    }

    // Java: public static PageSize getPageSize(int index) 
    pub fn getPageSize(index: i32) -> PageSize {
      // Java: return (PageSize)mPages.get(index);
      return ((mPages.get(index)) as PageSize);
    }

    // Java: public static PageSize getPageSize(float width, float height) 
    pub fn getPageSize__float_float(width: f32, height: f32) -> PageSize {
      // Java: for(Map.Entry<Integer, PageSize> entry : mPages.entrySet())
      for entry in mPages.entrySet().iter() {
         // Java: PageSize pageSize = (PageSize)entry.getValue();
         let pageSize: PageSize = ((entry.getValue()) as PageSize);
         // Java: if (pageSize.equalsWidth(width, height))
         if pageSize.equalsWidth(width, height) {
            // Java: return pageSize;
            return pageSize;
         }
      }
      // Java: return getA4();
      return self.getA4();
    }

    // Java: public static PageSize[] getPageSizeTypes() 
    pub fn getPageSizeTypes() -> Vec<PageSize> {
      // Java: PageSize[] mPageSizes = new PageSize[mPages.size()];
      let mPageSizes: Vec<PageSize> = vec![Default::default(); mPages.size()];
      // Java: Iterator<Map.Entry<Integer, PageSize>> iterator = mPages.entrySet().iterator();
      let iterator: Iterator<Entry<i32, PageSize>> = mPages.entrySet().iterator();
      let index: i32 = 0;
      // Java: for(int index = 0; iterator.hasNext(); ++index)
      while iterator.hasNext() {
         // Java: Map.Entry<Integer, PageSize> entry = (Map.Entry)iterator.next();
         let entry: Entry<i32, PageSize> = ((iterator.next()) as Entry);
         // Java: mPageSizes[index] = (PageSize)entry.getValue();
         mPageSizes[index] = ((entry.getValue()) as PageSize);
         index += 1;
      }
      // Java: return mPageSizes;
      return mPageSizes;
    }

}

// Java: public static class PageSize 
impl PageSize {
    // Java: public PageSize(int index, String pageName, float width, float height) 
    pub fn new(index: i32, pageName: String, width: f32, height: f32) -> PageSize {
      Self { mIndex: index, mPageName: pageName, mWidth: width, mHeigth: height }
    }

    // Java: public boolean equalsWidth(float width, float height) 
    pub fn equalsWidth(&mut self, width: f32, height: f32) -> bool {
         // Java: return width - this.mWidth <= 10.0F && height - this.mHeigth <= 10.0F;
         return width - self.mWidth <= 10 && height - self.mHeigth <= 10;
    }

    // Java: public boolean equals(Object o) 
    pub fn equals(&mut self, o: Box<dyn std::any::Any>) -> bool {
         // Java: PageSize page = (PageSize)o;
         let page: PageSize = ((o) as PageSize);
         // Java: if (page == null)
         if page.is_none() {
            // Java: return false;
            return false;
         }
         // Java: else
         else {
            // Java: return this.mIndex == page.mIndex && this.mPageName.equals(page.mPageName);
            return self.mIndex == page.mIndex && self.mPageName.equals(page.mPageName);
         }
    }

    // Java: public int hashCode() 
    pub fn hashCode(&mut self) -> i32 {
         // Java: int prime = 31;
         let prime: i32 = 31;
         // Java: int result = 1;
         let result: i32 = 1;
         // Java: result = 31 * result + this.mIndex;
         result = 31 * result + self.mIndex;
         // Java: result = 31 * result + (this.mPageName == null ? 0 : this.mPageName.hashCode());
         result = 31 * result + (if self.mPageName.is_none() { 0 } else { self.mPageName.hashCode() });
         // Java: return result;
         return result;
    }

}
