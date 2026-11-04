// Java: package org.apache.poi.hwpf;

// Java: import cn.wps.moffice.kfs.FileInputStream;
// Java: import cn.wps.moffice.kfs.RandomAccessFile;
// Java: import java.io.IOException;
// Java: import java.io.InputStream;
// Java: import java.io.PushbackInputStream;
// Java: import java.nio.ByteBuffer;
// Java: import java.nio.channels.FileChannel;
// Java: import java.nio.channels.FileChannel.MapMode;
// Java: import org.apache.poi.POIDocument;
// Java: import org.apache.poi.hwpf.model.CHPBinTable;
// Java: import org.apache.poi.hwpf.model.FileInformationBlock;
// Java: import org.apache.poi.hwpf.model.FontTable;
// Java: import org.apache.poi.hwpf.model.ListTables;
// Java: import org.apache.poi.hwpf.model.PAPBinTable;
// Java: import org.apache.poi.hwpf.model.SectionTable;
// Java: import org.apache.poi.hwpf.model.StyleSheet;
// Java: import org.apache.poi.hwpf.model.TextPieceTable;
// Java: import org.apache.poi.hwpf.usermodel.Range;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem;

use crate::org::apache::poi::hwpf::model::CHPBinTable::CHPBinTable;
use crate::org::apache::poi::hwpf::model::FileInformationBlock::FileInformationBlock;
use crate::org::apache::poi::hwpf::model::PAPBinTable::PAPBinTable;
use crate::org::apache::poi::poifs::filesystem::DirectoryNode::DirectoryNode;
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem::POIFSFileSystem;

// Java: public abstract class HWPFDocumentCore extends POIDocument {
pub struct HWPFDocumentCore {
    // Java: public static final String STREAM_ENTRY_WORDDOCUMENT = "WordDocument";
    // Java: public static final int BODY = 0;
    // Java: public static final int HEADER = 1;
    // Java: public static final int ANNOTATION = 2;
    // Java: public static final int TEXTBOX = 3;
    // Java: public static final int HEADERTEXTBOX = 4;
    // Java: protected FileInformationBlock _fib;
    pub _fib: Option<FileInformationBlock>,
    // Java: protected StyleSheet _ss;
    pub _ss: i32,
    // Java: protected CHPBinTable _cbt;
    pub _cbt: Option<CHPBinTable>,
    // Java: protected CHPBinTable mCbt_Textbox;
    pub mCbt_Textbox: Option<CHPBinTable>,
    // Java: protected PAPBinTable _pbt;
    pub _pbt: Option<PAPBinTable>,
    // Java: protected PAPBinTable mPbt_Textbox;
    pub mPbt_Textbox: Option<PAPBinTable>,
    // Java: protected SectionTable _st;
    pub _st: i32,
    // Java: protected FontTable _ft;
    pub _ft: i32,
    // Java: protected ListTables _lt;
    pub _lt: i32,
    // Java: protected DocumentInputStream _mainStream;
    pub _mainStream: Option<*mut DocumentInputStream>,
    // Java: protected DocumentInputStream _mainStream2;
    pub _mainStream2: Option<*mut DocumentInputStream>,
    // Java: protected DocumentInputStream _mainStream3;
    pub _mainStream3: Option<*mut DocumentInputStream>,
}

impl HWPFDocumentCore {
    pub const STREAM_ENTRY_WORDDOCUMENT: &'static str = "WordDocument";
    pub const BODY: i32 = 0;
    pub const HEADER: i32 = 1;
    pub const ANNOTATION: i32 = 2;
    pub const TEXTBOX: i32 = 3;
    pub const HEADERTEXTBOX: i32 = 4;

    // Java: protected HWPFDocumentCore() {
    pub fn new() -> Self {
        // Java: super((DirectoryNode)null);
        HWPFDocumentCore {
            _fib: None,
            _ss: 0,
            _cbt: None,
            mCbt_Textbox: None,
            _pbt: None,
            mPbt_Textbox: None,
            _st: 0,
            _ft: 0,
            _lt: 0,
            _mainStream: None,
            _mainStream2: None,
            _mainStream3: None,
        }
    }

    // Java: public void dispose() {
    pub fn dispose(&mut self) {
        // Java: if (null != this._mainStream) {
        if self._mainStream.is_some() {
            // Java: this._mainStream.close();
            // Java: this._mainStream = null;
            self._mainStream = None;
        }
        // Java: if (null != this._mainStream2) {
        if self._mainStream2.is_some() {
            // Java: this._mainStream2.close();
            // Java: this._mainStream2 = null;
            self._mainStream2 = None;
        }
        // Java: if (null != this._mainStream3) {
        if self._mainStream3.is_some() {
            // Java: this._mainStream3.close();
            // Java: this._mainStream3 = null;
            self._mainStream3 = None;
        }
    }

    // Java: public static POIFSFileSystem verifyAndBuildPOIFS(InputStream istream) throws IOException {
    pub fn verifyAndBuildPOIFS_stream(istream: &[u8]) -> Option<*mut POIFSFileSystem> {
        // Java: PushbackInputStream pis = new PushbackInputStream(istream, 6);
        // Java: byte[] first6 = new byte[6];
        // Java: pis.read(first6);
        if istream.len() < 6 {
            return None;
        }
        let first6 = &istream[0..6];
        // Java: if (first6[0] == 123 && first6[1] == 92 && first6[2] == 114 && first6[3] == 116 && first6[4] == 102) {
        if first6[0] == 123 && first6[1] == 92 && first6[2] == 114 && first6[3] == 116 && first6[4] == 102 {
            // Java: throw new IllegalArgumentException("The document is really a RTF file");
            return None;
        }
        // Java: pis.unread(first6);
        // Java: return new POIFSFileSystem(pis);
        None
    }

    // Java: public static POIFSFileSystem verifyAndBuildPOIFS(RandomAccessFile raf) throws IOException {
    pub fn verifyAndBuildPOIFS_raf(_raf: i64) -> Option<*mut POIFSFileSystem> {
        // Java: long pos = raf.getFilePointer();
        // Java: byte[] first6 = new byte[6];
        // Java: raf.read(first6);
        // Java: if (first6[0] == 123 && first6[1] == 92 && first6[2] == 114 && first6[3] == 116 && first6[4] == 102) {
        // Java: throw new IllegalArgumentException("The document is really a RTF file");
        // Java: raf.seek(pos);
        // Java: return new POIFSFileSystem(raf);
        None
    }

    // Java: public final CHPBinTable getCharacterTable_Texbbox() {
    pub fn getCharacterTable_Texbbox(&self) -> &Option<CHPBinTable> {
        // Java: return this.mCbt_Textbox;
        &self.mCbt_Textbox
    }

    // Java: public final PAPBinTable getParagraphTable_Textbox() {
    pub fn getParagraphTable_Textbox(&self) -> &Option<PAPBinTable> {
        // Java: return this.mPbt_Textbox;
        &self.mPbt_Textbox
    }

    // Java: public final CHPBinTable getCharacterTable() {
    pub fn getCharacterTable(&self) -> &Option<CHPBinTable> {
        // Java: return this._cbt;
        &self._cbt
    }

    // Java: public final PAPBinTable getParagraphTable() {
    pub fn getParagraphTable(&self) -> &Option<PAPBinTable> {
        // Java: return this._pbt;
        &self._pbt
    }

    // Java: public final FileInformationBlock getFileInformationBlock() {
    pub fn getFileInformationBlock(&self) -> &Option<FileInformationBlock> {
        // Java: return this._fib;
        &self._fib
    }
}
