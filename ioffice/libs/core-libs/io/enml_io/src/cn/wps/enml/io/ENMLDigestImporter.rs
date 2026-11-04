// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.base.assertion.KSAssert
use util::base::assertion::KSAssert::KSAssert;
use adapter::cn::wps::enml::io::IENMLDocumentImporter::IENMLDocumentImporter;

// Java: public class ENMLDigestImporter implements IENMLDocumentImporter
pub struct ENMLDigestImporter {
    // Java field: StringBuffer mBuffer
    mBuffer: Option<String>,
    // Java field: int mCount
    mCount: i32,
    // Java field: boolean mStartNewBlock
    mStartNewBlock: bool,
    // Java field: boolean mFirstPara
    mFirstPara: bool,
}

// Java: public class ENMLDigestImporter implements IENMLDocumentImporter
impl ENMLDigestImporter {
    // Java: ENMLDigestImporter(StringBuffer buffer, int count)
    pub fn new(buffer: String, count: i32) -> Self {
      // Java: super();
      // Java: KSAssert.assertNotNull("buffer should not be null.", buffer);
      // Java: KSAssert.assertTrue("count > 0 should be true", count > 0);
      Assert::assertTrue_msg("count > 0 should be true", count > 0);
      // Java: this.mBuffer = buffer;
      // Java: this.mCount = count;
      // Java: this.mStartNewBlock = false;
      // Java: this.mFirstPara = true;
      ENMLDigestImporter {
          mBuffer: Some(buffer),
          mCount: count,
          mStartNewBlock: false,
          mFirstPara: true,
      }
    }

    // Java: void onStartDocument()
    pub fn onStartDocument(&mut self) {
      // Java: this.mStartNewBlock = false;
      self.mStartNewBlock = false;
    }

    // Java: void onEndDocument()
    pub fn onEndDocument(&mut self) {
      // Java: this.mStartNewBlock = false;
      self.mStartNewBlock = false;
    }

    // Java: void onStartBlock()
    pub fn onStartBlock(&mut self) {
      // Java: if (!this.mFirstPara)
      if !self.mFirstPara
      {
         // Java: this.mStartNewBlock = true;
         self.mStartNewBlock = true;
      }
    }

    // Java: void onCharacters(String chars)
    pub fn onCharacters(&mut self, chars: &str) {
      // Java: if (this.mStartNewBlock)
      if self.mStartNewBlock
      {
         // Java: this.onInsertParaBreak();
         self.onInsertParaBreak();
      }
      // Java: this.appendTextFragment(chars);
      self.appendTextFragment(chars);
      // Java: if (this.mBuffer.length() >= this.mCount)
      if self.mBuffer.as_ref().map_or(0, |b| b.len() as i32) >= self.mCount
      {
         // Java: throw new ENMLEndOfImportException();
         return;
      }
      // Java: else
      else
      {
         // Java: this.mFirstPara = false;
         self.mFirstPara = false;
      }
    }

    // Java: void onEndBlock()
    pub fn onEndBlock(&mut self) {
      // Java: this.mStartNewBlock = true;
      self.mStartNewBlock = true;
    }

    // Java: void onInsertLineBreak()
    pub fn onInsertLineBreak(&mut self) {
      // Java: this.onCharacters(" ");
      self.onCharacters(" ");
    }

    // Java: void onInsertParaBreak()
    pub fn onInsertParaBreak(&mut self) {
      // Java: this.appendParagraphBreak(' ');
      self.appendParagraphBreak(' ');
      // Java: this.mStartNewBlock = false;
      self.mStartNewBlock = false;
    }

    // Java: void appendTextFragment(String chars)
    pub fn appendTextFragment(&mut self, chars: &str) {
      // Java: Assert.assertNotNull("chars should not be null", chars);
      // Java: Assert.assertNotNull("mBuffer should not be null", this.mBuffer);
      Assert::assertNotNull_msg("mBuffer should not be null", &self.mBuffer);
      // Java: this.mBuffer.append(chars);
      if let Some(ref mut buf) = self.mBuffer {
          buf.push_str(chars);
      }
    }

    // Java: void appendParagraphBreak(char ch)
    pub fn appendParagraphBreak(&mut self, ch: char) {
      // Java: Assert.assertTrue("(ControlChar.SPACE == ch) should be true", ' ' == ch);
      Assert::assertTrue_msg("(ControlChar.SPACE == ch) should be true", ' ' == ch);
      // Java: this.mBuffer.append(ch);
      if let Some(ref mut buf) = self.mBuffer {
          buf.push(ch);
      }
    }

    // Java: void onImportMedia(IENMLMedia media)
    pub fn onImportMedia(&mut self, _media: &dyn std::any::Any) {
    }

}

impl IENMLDocumentImporter for ENMLDigestImporter {
    fn onStartDocument(&mut self) {
        self.onStartDocument();
    }

    fn onEndDocument(&mut self) {
        self.onEndDocument();
    }

    fn onStartBlock(&mut self) {
        self.onStartBlock();
    }

    fn onEndBlock(&mut self) {
        self.onEndBlock();
    }

    fn onCharacters(&mut self, var1: &str) {
        self.onCharacters(var1);
    }

    fn onInsertLineBreak(&mut self) {
        self.onInsertLineBreak();
    }

    fn onInsertParaBreak(&mut self) {
        self.onInsertParaBreak();
    }

    fn onImportMedia(&mut self, var1: &dyn std::any::Any) {
        self.onImportMedia(var1);
    }
}
