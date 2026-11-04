// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.TextTool
use crate::cn::wps::moffice::writer::core::TextTool::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.core.env.text.FilterArgs
use crate::cn::wps::moffice::writer::core::env::text::FilterArgs::*;
// Java: import cn.wps.moffice.writer.core.env.text.ITextFilter
use crate::cn::wps::moffice::writer::core::env::text::ITextFilter::*;
// Java: import cn.wps.moffice.writer.core.env.text.DrawingFilter
use crate::cn::wps::moffice::writer::core::env::text::DrawingFilter::*;
// Java: import cn.wps.moffice.writer.core.env.text.TableFilter
use crate::cn::wps::moffice::writer::core::env::text::TableFilter::*;
// Java: import cn.wps.moffice.writer.core.env.text.FieldFilter
use crate::cn::wps::moffice::writer::core::env::text::FieldFilter::*;

// java static String DISPLAYTEXTFORMAT = "<%s>";
pub(crate) static DISPLAYTEXTFORMAT: std::sync::LazyLock<Option<String>> =
                    std::sync::LazyLock::new(|| "<%s>");

// java public class TextFilterTool
#[derive(Default)]
pub struct TextFilterTool {
    // java private int mCurCP;
    mCurCP: i32,
    // java private int mMaxCP;
    mMaxCP: i32,
    // java private FilterArgs mFilterArgs;
    mFilterArgs: Option<FilterArgs>,
    // java private ArrayList<ITextFilter> mFilters;
    mFilters: Option<Vec<Box<dyn ITextFilter>>>,
    // java private Document mDocument;
    mDocument: Option<Document>,
    // java private StringBuffer mStringBuffer;
    mStringBuffer: Option<String>,
    // java private int mStart;
    mStart: i32,
    // java private int mEnd;
    mEnd: i32,
}

// java public class TextFilterTool
impl TextFilterTool {
    // java public  TextFilterTool(KRange range)
    pub fn TextFilterTool(range: &KRange) -> Self {
      // java this.initTextFilterTool(range);
      this.initTextFilterTool(&range);
    }

    // java private void initTextFilterTool(KRange range)
    pub fn initTextFilterTool(&mut self, range: &KRange) {
      // java this.mDocument = range.getSubDocument();
      self.mDocument = range.getSubDocument();
      // java this.mFilterArgs = new FilterArgs();
      self.mFilterArgs = FilterArgs::new();
      // java this.mFilters = new ArrayList();
      self.mFilters = Vec::new();
      // java this.mStringBuffer = new StringBuffer();
      self.mStringBuffer = StringBuffer::new();
      // java this.mCurCP = range.getStart();
      self.mCurCP = range.getStart();
      // java this.mMaxCP = range.getEnd();
      self.mMaxCP = range.getEnd();
      // java this.mStart = range.getStart();
      self.mStart = range.getStart();
      // java this.mEnd = range.getEnd();
      self.mEnd = range.getEnd();
    }

    // java public String getCommentMainText()
    pub fn getCommentMainText(&self) -> String {
      // java return this.getCommentMainText(Integer.MAX_VALUE);
      return self.getCommentMainText__int(Integer::MAX_VALUE);
    }

    // java public String getCommentMainText(int maxLen)
    pub fn getCommentMainText__int(&mut self, maxLen: i32) -> String {
      // java this.initCommentMainFilter();
      self.initCommentMainFilter();
      // java KRange rg = new KRange(this.mDocument, 0, 0);
      let mut rg: KRange = KRange::new(self.mDocument, 0, 0);
      // java while (this.mCurCP < this.mMaxCP && this.mStringBuffer.length() <= maxLen)
      while self.mCurCP < self.mMaxCP && (self.mStringBuffer.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) <= maxLen {
        // java this.mFilterArgs.init(this.mCurCP, this.mMaxCP);
        self.mFilterArgs.init(self.mCurCP, self.mMaxCP);
        // java boolean hasDoFilter = false;
        let mut hasDoFilter: bool = false;
        // java int i = 0;
        let mut i: i32 = 0;
        let mut size: i32 = (self.mFilters.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
        // java for(int size = this.mFilters.size(); i < size; ++i)
        while i < size {
          // java hasDoFilter = ((ITextFilter)this.mFilters.get(i)).doFilter(this.mFilterArgs);
          hasDoFilter = todo!("cast → Box<dyn ITextFilter>").doFilter(self.mFilterArgs);
          // java if (hasDoFilter)
          /* TODO block */ {
            // java break;
            break;
          }
        }
        // java if (hasDoFilter)
        /* TODO block */ {
          // java if (this.mFilterArgs.next > this.mCurCP)
          /* TODO block */ {
            // java this.mCurCP = this.mFilterArgs.next;
            self.mCurCP = self.mFilterArgs.next;
          }
        }
        // java else
        /* TODO block */ {
          // java IChpxTable.Entry run = this.mDocument.getChpxTable().seek(this.mCurCP);
          let mut run: Box<dyn Entry> = self.mDocument.getChpxTable().seek(self.mCurCP);
          // java int runEnd = run.endCp();
          let mut runEnd: i32 = run.endCp();
          // java rg.setRange(this.mCurCP, Math.min(runEnd, this.mMaxCP));
          rg.setRange(self.mCurCP, Math::min(runEnd, self.mMaxCP));
          // java this.mStringBuffer.append(TextTool.filterRange(rg, true));
          self.mStringBuffer.append(TextTool::filterRange(rg, true));
          // java this.mCurCP = runEnd;
          self.mCurCP = runEnd;
        }
      }
      // java rg.dispose();
      rg.dispose();
      // java return this.mStringBuffer.toString();
      return self.mStringBuffer.toString();
    }

    // java private void initCommentMainFilter()
    pub fn initCommentMainFilter(&mut self) {
      // java this.mFilters.clear();
      self.mFilters.clear();
      // java this.mFilters.add(new DrawingFilter(this.mDocument, this.mStringBuffer, this.mStart, this.mEnd));
      self.mFilters.add(DrawingFilter::new(self.mDocument, self.mStringBuffer, self.mStart, self.mEnd));
      // java this.mFilters.add(new TableFilter(this.mDocument, this.mStringBuffer, this.mStart, this.mEnd));
      self.mFilters.add(TableFilter::new(self.mDocument, self.mStringBuffer, self.mStart, self.mEnd));
      // java this.mFilters.add(new FieldFilter(this.mDocument, this.mStringBuffer, this.mStart, this.mEnd));
      self.mFilters.add(FieldFilter::new(self.mDocument, self.mStringBuffer, self.mStart, self.mEnd));
    }

}
