// Java: import java.io.IOException
// Java: import java.util.Arrays
// Java: import java.util.Comparator
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use super::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use super::PlexOfCps::{PlexOfCps, GenericPropertyNode};
use super::PieceDescriptor::PieceDescriptor;
use super::TextPiece::TextPiece;
use super::CharIndexTranslator::CharIndexTranslator;
use std::sync::{Arc, Mutex};

// Java: public class TextPieceTable implements CharIndexTranslator
pub struct TextPieceTable {
    // Java field: int _cpMin
    pub(crate) _cpMin: i32,
    // Java field: int pieceStart
    pub(crate) pieceStart: i32,
    // Java field: int pieceEnd
    pieceEnd: i32,
    // Java field: int charCount
    pub(crate) charCount: i32,
    // Java field: int tableSize
    tableSize: i32,
    // Java field: TextPiece[] _textPieces
    pub(crate) _textPieces: Vec<TextPiece>,
    // Java field: TextPiece[] _textPiecesFCOrder
    pub(crate) _textPiecesFCOrder: Vec<usize>,
    // Java field: int[] _textPieceCPs
    pub(crate) _textPieceCPs: Vec<i32>,
}

// Java: public class FCComparator
pub struct FCComparator {
}

// Java: public class TextPieceTable implements CharIndexTranslator
impl TextPieceTable {
    // Java: TextPieceTable(DocumentInputStream documentStream, DocumentInputStream tableStream, int offset, int size, int fcMin)
    pub fn new(document_stream: DocumentInputStream, table_stream: DocumentInputStream, offset: i32, size: i32, fc_min: i32) -> Self {
      // Java: super();
      // Java: PlexOfCps pieceTable = new PlexOfCps(tableStream, offset, size, PieceDescriptor.getSizeInBytes());
      let mut piece_table = PlexOfCps::new_from_stream(table_stream, offset, size, PieceDescriptor::getSizeInBytes());
      // Java: int length = pieceTable.length();
      let length = piece_table.length();
      // Java: PieceDescriptor[] pieces = new PieceDescriptor[length];
      let mut pieces: Vec<PieceDescriptor> = Vec::with_capacity(length as usize);
      // Java: this._textPieces = new TextPiece[length];
      let mut text_pieces: Vec<TextPiece> = Vec::with_capacity(length as usize);
      // Java: this._textPieceCPs = new int[length];
      let mut text_piece_cps: Vec<i32> = Vec::with_capacity(length as usize);
      // Java: for(int x = 0; x < length; ++x)
        for x in 0..length
      {
         // Java: GenericPropertyNode node = pieceTable.getProperty(x);
         let node = piece_table.getProperty(x);
         // Java: PieceDescriptor piece = new PieceDescriptor(node.getBytes(), node.getOffset());
         let piece = PieceDescriptor::new(node.getBytes(), node.getOffset());
         // Java: pieces[x] = piece;
         // Java: int startFc = piece.getFilePosition();
         let start_fc = piece.getFilePosition();
         // Java: int startCp = node.getStart();
         let start_cp = node.getStart();
         // Java: int endCp = node.getEnd();
         let end_cp = node.getEnd();
         // Java: boolean unicode = piece.isUnicode();
         let unicode = piece.isUnicode();
         // Java: int multiple = unicode ? 2 : 1;
         let multiple = if unicode { 2 } else { 1 };
         // Java: int textSizeChars = endCp - startCp;
         let _text_size_chars = end_cp - start_cp;
         // Java: int textSizeBytes = textSizeChars * multiple;
         let text_size_bytes = (end_cp - start_cp) * multiple;
         // Java: this._textPieces[x] = new TextPiece(startCp, endCp, documentStream, startFc, textSizeBytes, piece);
         text_pieces.push(TextPiece::new(start_cp, end_cp, document_stream.clone(), start_fc, text_size_bytes, piece.clone()));
         // Java: this._textPieceCPs[x] = startCp;
         text_piece_cps.push(start_cp);
         pieces.push(piece);
      }
      // Java: this._cpMin = pieces[0].getFilePosition() - fcMin;
      let mut cp_min = pieces[0].getFilePosition() - fc_min;
      // Java: for(int x = 0; x < pieces.length; ++x)
        for x in 0..pieces.len()
      {
         // Java: int start = pieces[x].getFilePosition() - fcMin;
         let start = pieces[x].getFilePosition() - fc_min;
         // Java: if (start < this._cpMin)
            if start < cp_min
         {
            // Java: this._cpMin = start;
            cp_min = start;
         }
      }
      // Java: Arrays.sort(this._textPieces);
      // Java: this._textPiecesFCOrder = (TextPiece[])this._textPieces.clone();
      let mut fc_order: Vec<usize> = (0..text_pieces.len()).collect();
      // Java: Arrays.sort(this._textPiecesFCOrder, new FCComparator());
      fc_order.sort_by(|&a, &b| {
          let left_fc = text_pieces[a].pd.lock().unwrap().fc;
          let right_fc = text_pieces[b].pd.lock().unwrap().fc;
          let left_bl = text_pieces[a].bytesLength;
          let right_bl = text_pieces[b].bytesLength;
          left_fc.cmp(&right_fc).then(left_bl.cmp(&right_bl))
      });
      // Java: this.tableSize = this._textPiecesFCOrder.length;
      let table_size = fc_order.len() as i32;

      TextPieceTable {
          _cpMin: cp_min,
          pieceStart: 0,
          pieceEnd: 0,
          charCount: 0,
          tableSize: table_size,
          _textPieces: text_pieces,
          _textPiecesFCOrder: fc_order,
          _textPieceCPs: text_piece_cps,
      }
    }

    // Java: int getCpMin()
    pub fn getCpMin(&self) -> i32 {
      // Java: return this._cpMin;
      self._cpMin
    }

    // Java: TextPiece[] getTextPieces()
    pub fn getTextPieces(&self) -> &[TextPiece] {
      // Java: return this._textPieces;
      &self._textPieces
    }

    // Java: int[] getTextPieceCPs()
    pub fn getTextPieceCPs(&self) -> &[i32] {
      // Java: return this._textPieceCPs;
      &self._textPieceCPs
    }

    // Java: void add(TextPiece piece)
    pub fn add(&mut self, _piece: TextPiece) {
    }

    // Java: byte[] writeTo(HWPFOutputStream docStream)
    pub fn writeTo(&mut self, doc_stream: &mut HWPFOutputStream) -> Vec<u8> {
      // Java: PlexOfCps textPlex = new PlexOfCps(PieceDescriptor.getSizeInBytes());
      let mut text_plex = PlexOfCps::new(PieceDescriptor::getSizeInBytes());
      // Java: for(int x = 0; x < this.tableSize; ++x)
        for x in 0..self.tableSize as usize
      {
         // Java: TextPiece next = this._textPieces[x];
         let next = &mut self._textPieces[x];
         // Java: PieceDescriptor pd = next.getPieceDescriptor();
         let pd_arc = next.getPieceDescriptor();
         // Java: int offset = docStream.getOffset();
         let offset = doc_stream.getOffset();
         // Java: int mod = offset % 512;
         let mod_val = offset % 512;
         // Java: if (mod != 0)
            if mod_val != 0
         {
            // Java: mod = 512 - mod;
            let padding = 512 - mod_val;
            // Java: byte[] buf = new byte[mod];
            let buf = vec![0u8; padding as usize];
            // Java: docStream.write(buf);
            doc_stream.write_all(&buf);
         }
         // Java: pd.setFilePosition(docStream.getOffset());
         pd_arc.lock().unwrap().setFilePosition(doc_stream.getOffset());
         // Java: next.writeRawBytes(docStream);
         next.writeRawBytes(doc_stream);
         // Java: int nodeStart = next.getStart();
         let node_start = next.getStart();
         // Java: int nodeEnd = next.getEnd();
         let node_end = next.getEnd();
         // Java: textPlex.addProperty(new GenericPropertyNode(nodeStart, nodeEnd, pd.toByteArray(), 0));
         let pd_bytes = pd_arc.lock().unwrap().toByteArray();
         text_plex.addProperty(GenericPropertyNode::new(node_start, node_end, pd_bytes, 0));
      }
      // Java: return textPlex.toByteArray();
      text_plex.toByteArray()
    }

    // Java: int adjustForInsert(int listIndex, int length)
    pub fn adjustForInsert(&mut self, list_index: i32, length: i32) -> i32 {
      // Java: TextPiece tp = this._textPieces[listIndex];
      // Java: tp.setEnd(tp.getEnd() + length);
      let end_val = self._textPieces[list_index as usize].getEnd() + length;
      self._textPieces[list_index as usize].setEnd(end_val);
      // Java: for(int x = listIndex + 1; x < this.tableSize; ++x)
        for x in (list_index + 1)..self.tableSize
      {
         // Java: tp = this._textPieces[x];
         // Java: tp.setStart(tp.getStart() + length);
         let start_val = self._textPieces[x as usize].getStart() + length;
         self._textPieces[x as usize].setStart(start_val);
         // Java: tp.setEnd(tp.getEnd() + length);
         let end_val = self._textPieces[x as usize].getEnd() + length;
         self._textPieces[x as usize].setEnd(end_val);
      }
      // Java: return length;
      length
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return super.hashCode();
      0
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, _o: &TextPieceTable) -> bool {
      // Java: if (o == null)
      {
         // Java: return false;
      }
      // Java: else
      {
         // Java: TextPieceTable tpt = (TextPieceTable)o;
         // Java: int size = tpt._textPieces.length;
         let size = _o._textPieces.len();
         // Java: if (size == this._textPieces.length)
            if size == self._textPieces.len()
         {
            // Java: for(int x = 0; x < size; ++x)
                for _x in 0..size
            {
               // Java: if (!tpt._textPieces[x].equals(this._textPieces[x]))
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: return true;
            return true;
         }
         // Java: else
            else
         {
            // Java: return false;
            return false;
         }
      }
    }

    // Java: int getCharIndex(int bytePos)
    pub fn getCharIndex_pos(&self, byte_pos: i32) -> i32 {
      // Java: return this.getCharIndex(bytePos, 0);
      self.getCharIndex_cp(byte_pos, 0)
    }

    // Java: int getCharIndex(int bytePos, boolean isStart)
    pub fn getCharIndex_start(&self, byte_pos: i32, is_start: bool) -> i32 {
      // Java: return this.getCharIndex(bytePos, 0, isStart);
      self.getCharIndex_full(byte_pos, 0, is_start)
    }

    // Java: int getCharIndex(int bytePos, int startCP, boolean isStart)
    pub fn getCharIndex_full(&self, mut byte_pos: i32, start_cp: i32, is_start: bool) -> i32 {
      // Java: this.charCount = 0;
      let mut char_count = 0i32;
      // Java: bytePos = this.lookIndexForward(bytePos, isStart);
      byte_pos = self.lookIndexForward_start(byte_pos, is_start);
      // Java: for(int i = 0; i < this.tableSize; ++i)
        for i in 0..self.tableSize as usize
      {
         // Java: TextPiece tp = this._textPieces[i];
         let tp = &self._textPieces[i];
         // Java: this.pieceStart = tp.pd.fc;
         let piece_start = tp.pd.lock().unwrap().fc;
         // Java: this.pieceEnd = this.pieceStart + tp.bytesLength;
         let piece_end = piece_start + tp.bytesLength;
         // Java: if (bytePos >= this.pieceStart && bytePos <= this.pieceEnd)
            if byte_pos >= piece_start && byte_pos <= piece_end
         {
            // Java: if (tp.usesUnicode)
                if tp.usesUnicode
            {
               // Java: this.charCount += (bytePos - this.pieceStart) / 2;
               char_count += (byte_pos - piece_start) / 2;
            }
            // Java: else
                else
            {
               // Java: this.charCount += bytePos - this.pieceStart;
               char_count += byte_pos - piece_start;
            }
         }
         // Java: else
            else
         {
            // Java: this.charCount += tp.charCount;
            char_count += tp.charCount;
         }
         // Java: if (bytePos >= this.pieceStart && bytePos <= this.pieceEnd && this.charCount >= startCP)
            if byte_pos >= piece_start && byte_pos <= piece_end && char_count >= start_cp
         {
            // Java: break;
            break;
         }
      }
      // Java: return this.charCount;
      char_count
    }

    // Java: int getCharIndex(int bytePos, int startCP)
    pub fn getCharIndex_cp(&self, byte_pos: i32, start_cp: i32) -> i32 {
      // Java: return this.getCharIndex(bytePos, startCP, false);
      self.getCharIndex_full(byte_pos, start_cp, false)
    }

    // Java: int lookIndexForward(int bytePos)
    pub fn lookIndexForward_pos(&self, byte_pos: i32) -> i32 {
      // Java: return this.lookIndexForward(bytePos, false);
      self.lookIndexForward_start(byte_pos, false)
    }

    // Java: int lookIndexForward(int bytePos, boolean isStart)
    pub fn lookIndexForward_start(&self, mut byte_pos: i32, is_start: bool) -> i32 {
      // Java: for(int i = 0; i < this.tableSize; ++i)
        for i in 0..self.tableSize as usize
      {
         // Java: TextPiece tp = this._textPiecesFCOrder[i];
         let idx = self._textPiecesFCOrder[i];
         let tp = &self._textPieces[idx];
         // Java: this.pieceStart = tp.pd.fc;
         let piece_start = tp.pd.lock().unwrap().fc;
         // Java: this.pieceEnd = this.pieceStart + tp.bytesLength;
         let piece_end = piece_start + tp.bytesLength;
         // Java: if (bytePos <= this.pieceEnd && (!isStart || bytePos != this.pieceEnd))
            if byte_pos <= piece_end && (!is_start || byte_pos != piece_end)
         {
            // Java: if (this.pieceStart > bytePos)
                if piece_start > byte_pos
            {
               // Java: bytePos = this.pieceStart;
               byte_pos = piece_start;
            }
            // Java: break;
            break;
         }
      }
      // Java: return bytePos;
      byte_pos
    }

    // Java: int lookIndexBackward(int bytePos)
    pub fn lookIndexBackward(&self, mut byte_pos: i32) -> i32 {
      // Java: int lastEnd = 0;
      let mut last_end = 0;
      // Java: for(int i = 0; i < this.tableSize; ++i)
        for i in 0..self.tableSize as usize
      {
         // Java: TextPiece tp = this._textPiecesFCOrder[i];
         let idx = self._textPiecesFCOrder[i];
         let tp = &self._textPieces[idx];
         // Java: int pieceStart = tp.pd.fc;
         let piece_start = tp.pd.lock().unwrap().fc;
         // Java: if (bytePos <= pieceStart + tp.bytesLength)
            if byte_pos <= piece_start + tp.bytesLength
         {
            // Java: if (pieceStart > bytePos)
                if piece_start > byte_pos
            {
               // Java: bytePos = lastEnd;
               byte_pos = last_end;
            }
            // Java: break;
            break;
         }
         // Java: lastEnd = pieceStart + tp.bytesLength;
         last_end = piece_start + tp.bytesLength;
      }
      // Java: return bytePos;
      byte_pos
    }

    // Java: boolean isIndexInTable(int bytePos)
    pub fn isIndexInTable(&self, byte_pos: i32) -> bool {
      // Java: int count = this._textPiecesFCOrder.length;
      let count = self._textPiecesFCOrder.len();
      // Java: for(int i = 0; i < count; ++i)
        for i in 0..count
      {
         // Java: TextPiece tp = this._textPiecesFCOrder[i];
         let idx = self._textPiecesFCOrder[i];
         let tp = &self._textPieces[idx];
         // Java: int pieceStart = tp.pd.fc;
         let piece_start = tp.pd.lock().unwrap().fc;
         // Java: if (bytePos <= pieceStart + tp.bytesLength)
            if byte_pos <= piece_start + tp.bytesLength
         {
            // Java: if (pieceStart > bytePos)
                if piece_start > byte_pos
            {
               // Java: return false;
               return false;
            }
            // Java: return true;
            return true;
         }
      }
      // Java: return false;
      false
    }

    // Java: int getLastRightIndexInTable(int bytePos)
    pub fn getLastRightIndexInTable(&self, byte_pos: i32) -> i32 {
      // Java: int count = this._textPiecesFCOrder.length;
      let count = self._textPiecesFCOrder.len();
      // Java: for(int i = 0; i < count; ++i)
        for i in 0..count
      {
         // Java: TextPiece tp = this._textPiecesFCOrder[i];
         let idx = self._textPiecesFCOrder[i];
         let tp = &self._textPieces[idx];
         // Java: int pieceStart = tp.pd.fc;
         let piece_start = tp.pd.lock().unwrap().fc;
         // Java: if (bytePos < pieceStart + tp.bytesLength && pieceStart >= bytePos)
            if byte_pos < piece_start + tp.bytesLength && piece_start >= byte_pos
         {
            // Java: return pieceStart;
            return piece_start;
         }
      }
      // Java: return -1;
      -1
    }

    // Java: int getLastLeftIndexInTable(int bytePos)
    pub fn getLastLeftIndexInTable(&self, byte_pos: i32) -> i32 {
      // Java: int count = this._textPiecesFCOrder.length;
      let count = self._textPiecesFCOrder.len();
      // Java: for(int i = count - 1; i >= 0; --i)
        for i in (0..count).rev()
      {
         // Java: TextPiece tp = this._textPiecesFCOrder[i];
         let idx = self._textPiecesFCOrder[i];
         let tp = &self._textPieces[idx];
         // Java: int pieceStart = tp.pd.fc;
         let piece_start = tp.pd.lock().unwrap().fc;
         // Java: if (bytePos >= pieceStart + tp.bytesLength)
            if byte_pos >= piece_start + tp.bytesLength
         {
            // Java: return pieceStart + tp.bytesLength;
            return piece_start + tp.bytesLength;
         }
      }
      // Java: return -1;
      -1
    }

    // Java: int getCharIndexForNormal(int bytePos)
    pub fn getCharIndexForNormal_pos(&self, byte_pos: i32) -> i32 {
      // Java: return this.getCharIndexForNormal(bytePos, 0);
      self.getCharIndexForNormal_cp(byte_pos, 0)
    }

    // Java: int getCharIndexForNormal(int bytePos, int startCP)
    pub fn getCharIndexForNormal_cp(&self, byte_pos: i32, start_cp: i32) -> i32 {
      // Java: return this.getCharIndexForNormal(bytePos, startCP, false);
      self.getCharIndexForNormal_full(byte_pos, start_cp, false)
    }

    // Java: int getCharIndexForNormal(int bytePos, boolean isStart)
    pub fn getCharIndexForNormal_start(&self, byte_pos: i32, is_start: bool) -> i32 {
      // Java: return this.getCharIndexForNormal(bytePos, 0, isStart);
      self.getCharIndexForNormal_full(byte_pos, 0, is_start)
    }

    // Java: int getCharIndexForNormal(int bytePos, int startCP, boolean isStart)
    pub fn getCharIndexForNormal_full(&self, mut byte_pos: i32, start_cp: i32, is_start: bool) -> i32 {
      // Java: this.charCount = 0;
      let mut char_count = 0i32;
      // Java: for(int i = 0; i < this.tableSize; ++i)
        for i in 0..self.tableSize as usize
      {
         // Java: TextPiece tp = this._textPieces[i];
         let tp = &self._textPieces[i];
         // Java: this.pieceStart = tp.pd.fc;
         let piece_start = tp.pd.lock().unwrap().fc;
         // Java: this.pieceEnd = this.pieceStart + tp.bytesLength;
         let piece_end = piece_start + tp.bytesLength;
         // Java: if (bytePos <= this.pieceEnd && (!isStart || bytePos != this.pieceEnd))
            if byte_pos <= piece_end && (!is_start || byte_pos != piece_end)
         {
            // Java: if (bytePos < this.pieceStart)
                if byte_pos < piece_start
            {
               // Java: bytePos = this.pieceStart;
               byte_pos = piece_start;
            }
            // Java: if (bytePos >= this.pieceStart)
                if byte_pos >= piece_start
            {
               // Java: if (tp.usesUnicode)
                    if tp.usesUnicode
               {
                  // Java: this.charCount += bytePos - this.pieceStart >> 1;
                  char_count += (byte_pos - piece_start) >> 1;
               }
               // Java: else
                    else
               {
                  // Java: this.charCount += bytePos - this.pieceStart;
                  char_count += byte_pos - piece_start;
               }
            }
            // Java: if (bytePos >= this.pieceStart && bytePos <= this.pieceEnd && this.charCount >= startCP)
                if byte_pos >= piece_start && byte_pos <= piece_end && char_count >= start_cp
            {
               // Java: break;
               break;
            }
         }
         // Java: else
            else
         {
            // Java: this.charCount += tp.charCount;
            char_count += tp.charCount;
         }
      }
      // Java: return this.charCount;
      char_count
    }

    // Java: static int getFC(TextPiece tp, int offsetCp)
    pub fn getFC_tp(tp: &TextPiece, offset_cp: i32) -> i32 {
      // Java: return tp.pd.fc + offsetCp * (tp.usesUnicode ? 2 : 1);
      tp.pd.lock().unwrap().fc + offset_cp * if tp.usesUnicode { 2 } else { 1 }
    }

    // Java: int getFC(int cp)
    pub fn getFC_cp(&self, mut cp: i32) -> i32 {
      // Java: int bytePos = 0;
      let mut byte_pos = 0;
      // Java: for(int i = 0; i < this.tableSize; ++i)
        for i in 0..self.tableSize as usize
      {
         // Java: TextPiece tp = this._textPieces[i];
         let tp = &self._textPieces[i];
         // Java: if (cp < tp.charCount)
            if cp < tp.charCount
         {
            // Java: bytePos = tp.pd.fc + cp * (tp.usesUnicode ? 2 : 1);
            byte_pos = tp.pd.lock().unwrap().fc + cp * if tp.usesUnicode { 2 } else { 1 };
            // Java: break;
            break;
         }
         // Java: cp -= tp.charCount;
         cp -= tp.charCount;
      }
      // Java: return bytePos;
      byte_pos
    }

}

// Java: public class FCComparator
impl FCComparator {
    // Java: FCComparator()
    pub fn new() -> Self {
         // Java: super();
         FCComparator {}
    }

    // Java: int compare(TextPiece textPiece, TextPiece textPiece1)
    pub fn compare(&self, text_piece: &TextPiece, text_piece1: &TextPiece) -> i32 {
         // Java: int leftFc = textPiece.pd.fc;
         let left_fc = text_piece.pd.lock().unwrap().fc;
         // Java: int rightFc = textPiece1.pd.fc;
         let right_fc = text_piece1.pd.lock().unwrap().fc;
         // Java: int leftBytesLength = textPiece.bytesLength;
         let left_bytes_length = text_piece.bytesLength;
         // Java: int rightBytesLength = textPiece1.bytesLength;
         let right_bytes_length = text_piece1.bytesLength;
         // Java: if (leftFc > rightFc)
        if left_fc > right_fc
         {
            // Java: return 1;
            return 1;
         }
         // Java: else if (leftFc < rightFc)
        else if left_fc < right_fc
         {
            // Java: return -1;
            return -1;
         }
         // Java: else if (leftBytesLength > rightBytesLength)
        else if left_bytes_length > right_bytes_length
         {
            // Java: return 1;
            return 1;
         }
         // Java: else
        else
         {
            // Java: return leftBytesLength < rightBytesLength ? -1 : 0;
            if left_bytes_length < right_bytes_length { -1 } else { 0 }
         }
    }

}

impl CharIndexTranslator for TextPieceTable {
    fn getCharIndex(&self, var1: i32) -> i32 {
        self.getCharIndex_pos(var1)
    }
    fn getCharIndex_with_flag(&self, var1: i32, var2: bool) -> i32 {
        self.getCharIndex_start(var1, var2)
    }
    fn getCharIndex_with_limit(&self, var1: i32, var2: i32) -> i32 {
        self.getCharIndex_cp(var1, var2)
    }
    fn isIndexInTable(&self, var1: i32) -> bool {
        TextPieceTable::isIndexInTable(self, var1)
    }
    fn lookIndexForward(&self, var1: i32) -> i32 {
        self.lookIndexForward_pos(var1)
    }
    fn lookIndexBackward(&self, var1: i32) -> i32 {
        TextPieceTable::lookIndexBackward(self, var1)
    }
    fn getCharIndexForNormal(&self, var1: i32) -> i32 {
        self.getCharIndexForNormal_pos(var1)
    }
    fn getCharIndexForNormal_with_flag(&self, var1: i32, var2: bool) -> i32 {
        self.getCharIndexForNormal_start(var1, var2)
    }
    fn getCharIndexForNormal_with_limit(&self, var1: i32, var2: i32) -> i32 {
        self.getCharIndexForNormal_cp(var1, var2)
    }
}
