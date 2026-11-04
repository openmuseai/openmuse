// Java: import java.util.ArrayList
// Java: import java.util.List

// Java: public class WordArtVertData
pub struct WordArtVertData {
    // Java field: int mLayoutIndex
    pub(crate) mLayoutIndex: i32,
    // Java field: List<TextUnitRenderData> mTextUnitList
    pub(crate) mTextUnitList: Vec<TextUnitRenderData>,
}

// Java: public class TextUnitRenderData
pub struct TextUnitRenderData {
    // Java field: int mSpanIndex
    pub(crate) mSpanIndex: i32,
    // Java field: int mStart
    pub(crate) mStart: i32,
    // Java field: int mEnd
    pub(crate) mEnd: i32,
    // Java field: float mWidth
    pub(crate) mWidth: f32,
    // Java field: String mText
    pub(crate) mText: String,
    // Java field: float mStartX
    pub(crate) mStartX: f32,
    // Java field: float mStartY
    pub(crate) mStartY: f32,
}

// Java: public class WordArtVertData
impl WordArtVertData {
    // Java: WordArtVertData(int layoutIndex)
    pub fn new(layoutIndex: i32) -> Self {
      // Java: super();
      // Java: this.mLayoutIndex = layoutIndex;
      WordArtVertData { mLayoutIndex: layoutIndex, mTextUnitList: Vec::new() }
    }

    // Java: int getLayoutIndex()
    pub fn getLayoutIndex(&self) -> i32 {
      // Java: return this.mLayoutIndex;
      self.mLayoutIndex
    }

    // Java: void setLayoutIndex(int index)
    pub fn setLayoutIndex(&mut self, index: i32) {
      // Java: this.mLayoutIndex = index;
      self.mLayoutIndex = index;
    }

    // Java: List<TextUnitRenderData> getTextUnitList()
    pub fn getTextUnitList(&self) -> &Vec<TextUnitRenderData> {
      // Java: return this.mTextUnitList;
      &self.mTextUnitList
    }

    // Java: void addTextUnitData(TextUnitRenderData data)
    pub fn addTextUnitData(&mut self, data: TextUnitRenderData) {
      // Java: if (this.mTextUnitList == null)
      // Java: this.mTextUnitList = new ArrayList();
      // Java: this.mTextUnitList.add(data);
      self.mTextUnitList.push(data);
    }

}

// Java: public class TextUnitRenderData
impl TextUnitRenderData {
    // Java: TextUnitRenderData()
    pub fn new() -> Self {
         // Java: super();
         TextUnitRenderData {
             mSpanIndex: 0, mStart: 0, mEnd: 0,
             mWidth: 0.0, mText: String::new(),
             mStartX: 0.0, mStartY: 0.0,
         }
    }

}
