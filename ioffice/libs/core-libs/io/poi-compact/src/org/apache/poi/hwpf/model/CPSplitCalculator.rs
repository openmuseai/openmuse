use super::FileInformationBlock::FileInformationBlock;

// Java: public class CPSplitCalculator
pub struct CPSplitCalculator {
    // Java field: FileInformationBlock fib
    pub fib: *const FileInformationBlock,
}

// Java: public class CPSplitCalculator
impl CPSplitCalculator {
    // Java: CPSplitCalculator(FileInformationBlock fib)
    pub fn new(fib: *const FileInformationBlock) -> Self {
        // Java: super();
        // Java: this.fib = fib;
        CPSplitCalculator { fib }
    }

    fn fib(&self) -> &FileInformationBlock {
        unsafe { &*self.fib }
    }

    // Java: int getMainDocumentStart()
    pub fn getMainDocumentStart(&self) -> i32 {
        // Java: return 0;
        0
    }

    // Java: int getMainDocumentEnd()
    pub fn getMainDocumentEnd(&self) -> i32 {
        // Java: return this.fib.getCcpText();
        self.fib().getCcpText()
    }

    // Java: int getFootnoteStart()
    pub fn getFootnoteStart(&self) -> i32 {
        // Java: return this.getMainDocumentEnd();
        self.getMainDocumentEnd()
    }

    // Java: int getFootnoteEnd()
    pub fn getFootnoteEnd(&self) -> i32 {
        // Java: return this.getFootnoteStart() + this.fib.getCcpFtn();
        self.getFootnoteStart() + self.fib().getCcpFtn()
    }

    // Java: int getHeaderStoryStart()
    pub fn getHeaderStoryStart(&self) -> i32 {
        // Java: return this.getFootnoteEnd();
        self.getFootnoteEnd()
    }

    // Java: int getHeaderStoryEnd()
    pub fn getHeaderStoryEnd(&self) -> i32 {
        // Java: return this.getHeaderStoryStart() + this.fib.getCcpHdd();
        self.getHeaderStoryStart() + self.fib().getCcpHdd()
    }

    // Java: int getCommentsStart()
    pub fn getCommentsStart(&self) -> i32 {
        // Java: return this.getHeaderStoryEnd();
        self.getHeaderStoryEnd()
    }

    // Java: int getCommentsEnd()
    pub fn getCommentsEnd(&self) -> i32 {
        // Java: return this.getCommentsStart() + this.fib.getCcpAtn();
        self.getCommentsStart() + self.fib().getCcpAtn()
    }

    // Java: int getEndNoteStart()
    pub fn getEndNoteStart(&self) -> i32 {
        // Java: return this.getCommentsEnd();
        self.getCommentsEnd()
    }

    // Java: int getEndNoteEnd()
    pub fn getEndNoteEnd(&self) -> i32 {
        // Java: return this.getEndNoteStart() + this.fib.getCcpEdn();
        self.getEndNoteStart() + self.fib().getCcpEdn()
    }

    // Java: int getMainTextboxStart()
    pub fn getMainTextboxStart(&self) -> i32 {
        // Java: return this.getEndNoteEnd();
        self.getEndNoteEnd()
    }

    // Java: int getMainTextboxEnd()
    pub fn getMainTextboxEnd(&self) -> i32 {
        // Java: return this.getMainTextboxStart() + this.fib.getCcpTxtBx();
        self.getMainTextboxStart() + self.fib().getCcpTxtBx()
    }

    // Java: int getHeaderTextboxStart()
    pub fn getHeaderTextboxStart(&self) -> i32 {
        // Java: return this.getMainTextboxEnd();
        self.getMainTextboxEnd()
    }

    // Java: int getHeaderTextboxEnd()
    pub fn getHeaderTextboxEnd(&self) -> i32 {
        // Java: return this.getHeaderTextboxStart() + this.fib.getCcpHdrTxtBx();
        self.getHeaderTextboxStart() + self.fib().getCcpHdrTxtBx()
    }
}
