// Java: import cn.wps.base.log.IOLog
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.model.io.HWPFFileSystem
use crate::org::apache::poi::hwpf::model::io::HWPFFileSystem::HWPFFileSystem;
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::SEPX::SEPX;
use super::TextPiece::TextPiece;
use super::TextPieceTable::TextPieceTable;

// Java field: String TAG
pub(crate) const TAG: &str = "";
// Java field: int SED_SIZE
pub(crate) const SED_SIZE: i32 = 12;

// Java: public class SectionTable
pub struct SectionTable {
    // Java field: ArrayList<SEPX> _sections
    pub _sections: Vec<SEPX>,
    // Java field: TextPiece[] _text
    pub _text: Vec<TextPiece>,
    // Java field: TextPieceTable tpt
    tpt: Option<TextPieceTable>,
}

// Java: public class SectionTable
impl SectionTable {
    // Java: SectionTable()
    pub fn new() -> Self {
        // Java: super();
        SectionTable {
            _sections: Vec::with_capacity(5),
            _text: Vec::new(),
            tpt: None,
        }
    }

    // Java: SectionTable(DocumentInputStream documentStream, DocumentInputStream tableStream, int offset, int size, int fcMin, TextPieceTable tpt, CPSplitCalculator cps)
    pub fn new_from_stream(_document_stream: &mut DocumentInputStream, _table_stream: &mut DocumentInputStream, _offset: i32, _size: i32, _fc_min: i32, _tpt: TextPieceTable, _cps: i32) -> Self {
        // Java: super();
        // Java: PlexOfCps sedPlex = new PlexOfCps(tableStream, offset, size, 12);
        // Java: this.tpt = tpt;
        // Java: this._text = tpt.getTextPieces();
        // Java: int length = sedPlex.length();
        // Java: try
        // Java: for(int x = 0; x < length; ++x)
        // Java: GenericPropertyNode node = sedPlex.getProperty(x);
        // Java: SectionDescriptor sed = new SectionDescriptor(node.getBytes(), node.getOffset());
        // Java: int fileOffset = sed.getFc();
        // Java: int startAt = this.CPtoFC(node.getStart());
        // Java: int endAt = this.CPtoFC(node.getEnd());
        // Java: if (fileOffset == -1)
        // Java: this._sections.add(new SEPX(sed, startAt, endAt, tpt, new byte[0]));
        // Java: else
        // Java: documentStream.seek((long)fileOffset);
        // Java: int sepxSize = documentStream.readUShort();
        // Java: byte[] buf = new byte[sepxSize];
        // Java: fileOffset += 2;
        // Java: documentStream.read(buf);
        // Java: this._sections.add(new SEPX(sed, startAt, endAt, tpt, buf));
        // Java: catch (Throwable e)
        // Java: IOLog.e(TAG, "Throwable", e);
        SectionTable {
            _sections: Vec::new(),
            _text: Vec::new(),
            tpt: None,
        }
    }

    // Java: void adjustForInsert(int listIndex, int length)
    pub fn adjustForInsert(&mut self, _list_index: i32, _length: i32) {
        // Java: int size = this._sections.size();
        // Java: SEPX sepx = (SEPX)this._sections.get(listIndex);
        // Java: sepx.setEnd(sepx.getEnd() + length);
        // Java: for(int x = listIndex + 1; x < size; ++x)
        // Java: sepx = (SEPX)this._sections.get(x);
        // Java: sepx.setStart(sepx.getStart() + length);
        // Java: sepx.setEnd(sepx.getEnd() + length);
    }

    // Java: int CPtoFC(int CP)
    pub fn CPtoFC(&self, cp: i32) -> i32 {
        // Java: TextPiece TP = null;
        // Java: for(int i = this._text.length - 1; i > -1; --i)
        // Java: TP = this._text[i];
        // Java: if (CP >= TP.getCP())
        // Java: break;
        // Java: int FC = TP.getPieceDescriptor().getFilePosition();
        // Java: int offset = CP - TP.getCP();
        // Java: if (TP.isUnicode())
        // Java: offset *= 2;
        // Java: FC += offset;
        // Java: return FC;
        for i in (0..self._text.len()).rev() {
            let tp = &self._text[i];
            if cp >= tp.getCP() {
                let fc = tp.getPieceDescriptor().lock().unwrap().getFilePosition();
                let mut offset = cp - tp.getCP();
                if tp.isUnicode() {
                    offset *= 2;
                }
                return fc + offset;
            }
        }
        0
    }

    // Java: ArrayList<SEPX> getSections()
    pub fn getSections(&self) -> &Vec<SEPX> {
        // Java: return this._sections;
        &self._sections
    }

    // Java: void writeTo(HWPFFileSystem sys, int fcMin)
    pub fn writeTo(&self, _sys: &mut HWPFFileSystem, _fc_min: i32) {
        // Java: HWPFOutputStream docStream = sys.getStream("WordDocument");
        // Java: HWPFOutputStream tableStream = sys.getStream("0Table");
        // Java: int offset = docStream.getOffset();
        // Java: int len = this._sections.size();
        // Java: PlexOfCps plex = new PlexOfCps(12);
        // Java: for(int x = 0; x < len; ++x)
        // Java: SEPX sepx = (SEPX)this._sections.get(x);
        // Java: byte[] grpprl = sepx.getGrpprl();
        // Java: byte[] shortBuf = new byte[2];
        // Java: LittleEndian.putShort(shortBuf, (short)grpprl.length);
        // Java: docStream.write(shortBuf);
        // Java: docStream.write(grpprl);
        // Java: SectionDescriptor sed = sepx.getSectionDescriptor();
        // Java: sed.setFc(offset);
        // Java: GenericPropertyNode property = new GenericPropertyNode(this.tpt.getCharIndex(sepx.getStart() * 2 + fcMin), this.tpt.getCharIndex(sepx.getEnd() * 2 + fcMin), sed.toByteArray(), 0);
        // Java: plex.addProperty(property);
        // Java: offset = docStream.getOffset();
        // Java: tableStream.write(plex.toByteArray());
    }
}
