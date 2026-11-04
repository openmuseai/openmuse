// Java: import cn.wps.kfc.api.map.IIntObjectMap
// Java: import java.io.IOException
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.hwpf.model.types.FIBAbstractType
use crate::org::apache::poi::hwpf::model::types::FIBAbstractType::FIBAbstractType;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::FIBLongHandler::FIBLongHandler;
use super::FIBShortHandler::FIBShortHandler;
use super::FIBFieldHandler::{FIBFieldHandler, UnhandledDataStructure};
use std::collections::HashMap;

fn le_get_short(buf: &[u8], offset: usize) -> i16 {
    ((buf[offset] as i16) & 0xFF) | (((buf[offset+1] as i16) & 0xFF) << 8)
}

// Java field: int MSOVERSION_2000
pub(crate) const MSOVERSION_2000: i32 = 217;
// Java field: int MSOVERSION_2002
pub(crate) const MSOVERSION_2002: i32 = 257;
// Java field: int MSOVERSION_2003
pub(crate) const MSOVERSION_2003: i32 = 268;
// Java field: int MSOVERSION_2007
pub(crate) const MSOVERSION_2007: i32 = 274;

// Java: public class FileInformationBlock extends FIBAbstractType implements Cloneable
pub struct FileInformationBlock {
    pub(crate) _base: FIBAbstractType,
    // Java field: FIBLongHandler _longHandler
    pub(crate) _longHandler: FIBLongHandler,
    // Java field: FIBShortHandler _shortHandler
    pub(crate) _shortHandler: FIBShortHandler,
    // Java field: FIBFieldHandler _fieldHandler
    pub(crate) _fieldHandler: FIBFieldHandler,
    // Java field: int cswNew
    pub(crate) cswNew: i32,
    // Java field: int _version
    pub(crate) _version: i32,
    // Java field: int[] fibRgCswNew
    pub(crate) fibRgCswNew: Vec<i32>,
    // Java field: int fibRgCswNewSize
    fibRgCswNewSize: i32,
}

// Java: public class FileInformationBlock extends FIBAbstractType implements Cloneable
impl FileInformationBlock {
    // Java: FileInformationBlock(byte[] mainDocument)
    pub fn new_from_bytes(main_document: &[u8]) -> Self {
      // Java: super();
      let mut base = FIBAbstractType::new();
      // Java: this.fillFields(mainDocument, 0);
      base.fillFields_bytes(main_document, 0);
      FileInformationBlock {
          _base: base,
          _longHandler: FIBLongHandler { _longs: Vec::new() },
          _shortHandler: FIBShortHandler { _shorts: Vec::new() },
          _fieldHandler: FIBFieldHandler { _unknownMap: HashMap::new(), _fields: Vec::new(), _numFields: 0 },
          cswNew: 0, _version: 0, fibRgCswNew: Vec::new(), fibRgCswNewSize: 0,
      }
    }

    // Java: void fillVariableFields(byte[] mainDocument, byte[] tableStream)
    pub fn fillVariableFields_bytes(&mut self, main_document: &[u8], table_stream: &[u8]) {
      // Java: int offset = 32;
      let mut offset: usize = 32;
      // Java: this._shortHandler = new FIBShortHandler(mainDocument);
      self._shortHandler = FIBShortHandler::new_from_bytes(main_document);
      // Java: offset += this._shortHandler.sizeInBytes();
      offset += self._shortHandler.sizeInBytes() as usize;
      // Java: this._longHandler = new FIBLongHandler(mainDocument, offset);
      self._longHandler = FIBLongHandler::new_from_bytes(main_document, offset);
      // Java: offset += this._longHandler.sizeInBytes();
      offset += self._longHandler.sizeInBytes() as usize;
      // Java: this._fieldHandler = new FIBFieldHandler(mainDocument, offset, tableStream, true);
      self._fieldHandler = FIBFieldHandler::new_from_bytes(main_document, offset, table_stream, true);
      // Java: offset += this._fieldHandler.sizeInBytes();
      offset += self._fieldHandler.sizeInBytes() as usize;
      // Java: this.cswNew = LittleEndian.getShort(mainDocument, offset);
      self.cswNew = le_get_short(main_document, offset) as i32;
      // Java: this._version = LittleEndian.getShort(mainDocument, offset + 2);
      self._version = le_get_short(main_document, offset + 2) as i32;
    }

    // Java: FileInformationBlock(DocumentInputStream mainDocument)
    pub fn new_from_stream(main_document: &mut DocumentInputStream) -> Self {
      // Java: super();
      let mut base = FIBAbstractType::new();
      // Java: this.fillFields(mainDocument, 0);
      base.fillFields_stream(main_document, 0);
      FileInformationBlock {
          _base: base,
          _longHandler: FIBLongHandler { _longs: Vec::new() },
          _shortHandler: FIBShortHandler { _shorts: Vec::new() },
          _fieldHandler: FIBFieldHandler { _unknownMap: HashMap::new(), _fields: Vec::new(), _numFields: 0 },
          cswNew: 0, _version: 0, fibRgCswNew: Vec::new(), fibRgCswNewSize: 0,
      }
    }

    // Java: void fillVariableFields(DocumentInputStream mainDocument, DocumentInputStream tableStream)
    pub fn fillVariableFields_stream(&mut self, main_document: &mut DocumentInputStream, table_stream: &mut DocumentInputStream) {
      // Java: int offset = 32;
      let mut offset: i32 = 32;
      // Java: mainDocument.seek(0L);
      main_document.seek(0);
      // Java: this._shortHandler = new FIBShortHandler(mainDocument);
      self._shortHandler = FIBShortHandler::new_from_stream(main_document);
      // Java: offset += this._shortHandler.sizeInBytes();
      offset += self._shortHandler.sizeInBytes();
      // Java: this._longHandler = new FIBLongHandler(mainDocument, offset);
      self._longHandler = FIBLongHandler::new_from_stream(main_document, offset);
      // Java: offset += this._longHandler.sizeInBytes();
      offset += self._longHandler.sizeInBytes();
      // Java: this._fieldHandler = new FIBFieldHandler(mainDocument, offset, tableStream, true);
      self._fieldHandler = FIBFieldHandler::new_from_stream(main_document, offset, table_stream, true);
      // Java: int var10000 = offset + this._fieldHandler.sizeInBytes();
      let _var10000 = offset + self._fieldHandler.sizeInBytes();
      // Java: this.cswNew = mainDocument.readShort();
      self.cswNew = main_document.readShort() as i32;
      // Java: this._version = mainDocument.readShort();
      self._version = main_document.readShort() as i32;
      // Java: if (this._version == 274)
      if self._version == 274 {
         // Java: this.fibRgCswNewSize = 8;
         self.fibRgCswNewSize = 8;
      }
      // Java: else
      else {
         // Java: this.fibRgCswNewSize = 2;
         self.fibRgCswNewSize = 2;
      }
      // Java: this.fibRgCswNew = new int[this.fibRgCswNewSize];
      self.fibRgCswNew = vec![0i32; self.fibRgCswNewSize as usize];
      // Java: for(int i = 0; i < this.fibRgCswNewSize; ++i)
      for i in 0..self.fibRgCswNewSize as usize {
         // Java: this.fibRgCswNew[i] = mainDocument.readShort();
         self.fibRgCswNew[i] = main_document.readShort() as i32;
      }
    }

    // Java: int getVersion()
    pub fn getVersion(&self) -> i32 {
      // Java: return this._version;
      self._version
    }

    // Java: int getCswNew()
    pub fn getCswNew(&self) -> i32 {
      // Java: return this.cswNew;
      self.cswNew
    }

    // Java: int[] getfibRgCswNew()
    pub fn getfibRgCswNew(&self) -> &[i32] {
      // Java: return this.fibRgCswNew;
      &self.fibRgCswNew
    }

    // Java: int getFcDop()
    pub fn getFcDop(&self) -> i32 {
      // Java: return this._fieldHandler.getFieldOffset(31);
      self._fieldHandler.getFieldOffset(31)
    }
    // Java: void setFcDop(int fcDop)
    pub fn setFcDop(&mut self, fc_dop: i32) {
      // Java: this._fieldHandler.setFieldOffset(31, fcDop);
      self._fieldHandler.setFieldOffset(31, fc_dop);
    }
    // Java: int getLcbDop()
    pub fn getLcbDop(&self) -> i32 {
      // Java: return this._fieldHandler.getFieldSize(31);
      self._fieldHandler.getFieldSize(31)
    }
    // Java: void setLcbDop(int lcbDop)
    pub fn setLcbDop(&mut self, lcb_dop: i32) {
      // Java: this._fieldHandler.setFieldSize(31, lcbDop);
      self._fieldHandler.setFieldSize(31, lcb_dop);
    }
    // Java: int getFcSttbfAssoc()
    pub fn getFcSttbfAssoc(&self) -> i32 {
      // Java: return this._fieldHandler.getFieldOffset(32);
      self._fieldHandler.getFieldOffset(32)
    }
    // Java: void setFcSttbfAssoc(int fcSttbfAssoc)
    pub fn setFcSttbfAssoc(&mut self, v: i32) {
      // Java: this._fieldHandler.setFieldOffset(32, fcSttbfAssoc);
      self._fieldHandler.setFieldOffset(32, v);
    }
    // Java: int getLcbSttbfAssoc()
    pub fn getLcbSttbfAssoc(&self) -> i32 {
      // Java: return this._fieldHandler.getFieldSize(32);
      self._fieldHandler.getFieldSize(32)
    }
    // Java: void setLcbSttbfAssoc(int lcbSttbfAssoc)
    pub fn setLcbSttbfAssoc(&mut self, v: i32) {
      // Java: this._fieldHandler.setFieldSize(32, lcbSttbfAssoc);
      self._fieldHandler.setFieldSize(32, v);
    }
    // Java: int getFcStshf()
    pub fn getFcStshf(&self) -> i32 {
      // Java: return this._fieldHandler.getFieldOffset(1);
      self._fieldHandler.getFieldOffset(1)
    }
    // Java: int getLcbStshf()
    pub fn getLcbStshf(&self) -> i32 {
      // Java: return this._fieldHandler.getFieldSize(1);
      self._fieldHandler.getFieldSize(1)
    }
    // Java: void setFcStshf(int fcStshf)
    pub fn setFcStshf(&mut self, v: i32) {
      // Java: this._fieldHandler.setFieldOffset(1, fcStshf);
      self._fieldHandler.setFieldOffset(1, v);
      // Java: this._fieldHandler.setFieldOffset(0, fcStshf);
      self._fieldHandler.setFieldOffset(0, v);
    }
    // Java: void setLcbStshf(int lcbStshf)
    pub fn setLcbStshf(&mut self, v: i32) {
      // Java: this._fieldHandler.setFieldSize(1, lcbStshf);
      self._fieldHandler.setFieldSize(1, v);
      // Java: this._fieldHandler.setFieldSize(0, lcbStshf);
      self._fieldHandler.setFieldSize(0, v);
    }
    // Java: int getFcClx()
    pub fn getFcClx(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(33);
      self._fieldHandler.getFieldOffset(33) }
    // Java: int getLcbClx()
    pub fn getLcbClx(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(33);
      self._fieldHandler.getFieldSize(33) }
    // Java: void setFcClx(int fcClx)
    pub fn setFcClx(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(33, fcClx);
      self._fieldHandler.setFieldOffset(33, v); }
    // Java: void setLcbClx(int lcbClx)
    pub fn setLcbClx(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(33, lcbClx);
      self._fieldHandler.setFieldSize(33, v); }
    // Java: int getFcPlcfbteChpx()
    pub fn getFcPlcfbteChpx(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(12);
      self._fieldHandler.getFieldOffset(12) }
    // Java: int getLcbPlcfbteChpx()
    pub fn getLcbPlcfbteChpx(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(12);
      self._fieldHandler.getFieldSize(12) }
    // Java: void setFcPlcfbteChpx(int fcPlcfBteChpx)
    pub fn setFcPlcfbteChpx(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(12, fcPlcfBteChpx);
      self._fieldHandler.setFieldOffset(12, v); }
    // Java: void setLcbPlcfbteChpx(int lcbPlcfBteChpx)
    pub fn setLcbPlcfbteChpx(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(12, lcbPlcfBteChpx);
      self._fieldHandler.setFieldSize(12, v); }
    // Java: int getFcPlcfbtePapx()
    pub fn getFcPlcfbtePapx(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(13);
      self._fieldHandler.getFieldOffset(13) }
    // Java: int getLcbPlcfbtePapx()
    pub fn getLcbPlcfbtePapx(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(13);
      self._fieldHandler.getFieldSize(13) }
    // Java: void setFcPlcfbtePapx(int fcPlcfBtePapx)
    pub fn setFcPlcfbtePapx(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(13, fcPlcfBtePapx);
      self._fieldHandler.setFieldOffset(13, v); }
    // Java: void setLcbPlcfbtePapx(int lcbPlcfBtePapx)
    pub fn setLcbPlcfbtePapx(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(13, lcbPlcfBtePapx);
      self._fieldHandler.setFieldSize(13, v); }
    // Java: int getFcPlcfsed()
    pub fn getFcPlcfsed(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(6);
      self._fieldHandler.getFieldOffset(6) }
    // Java: int getLcbPlcfsed()
    pub fn getLcbPlcfsed(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(6);
      self._fieldHandler.getFieldSize(6) }
    // Java: void setFcPlcfsed(int fcPlcfSed)
    pub fn setFcPlcfsed(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(6, fcPlcfSed);
      self._fieldHandler.setFieldOffset(6, v); }
    // Java: void setLcbPlcfsed(int lcbPlcfSed)
    pub fn setLcbPlcfsed(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(6, lcbPlcfSed);
      self._fieldHandler.setFieldSize(6, v); }
    // Java: int getFcPlcfLst()
    pub fn getFcPlcfLst(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(73);
      self._fieldHandler.getFieldOffset(73) }
    // Java: int getLcbPlcfLst()
    pub fn getLcbPlcfLst(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(73);
      self._fieldHandler.getFieldSize(73) }
    // Java: void setFcPlcfLst(int fcPlcfLst)
    pub fn setFcPlcfLst(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(73, fcPlcfLst);
      self._fieldHandler.setFieldOffset(73, v); }
    // Java: void setLcbPlcfLst(int lcbPlcfLst)
    pub fn setLcbPlcfLst(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(73, lcbPlcfLst);
      self._fieldHandler.setFieldSize(73, v); }
    // Java: int getFcPlfLfo()
    pub fn getFcPlfLfo(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(74);
      self._fieldHandler.getFieldOffset(74) }
    // Java: int getLcbPlfLfo()
    pub fn getLcbPlfLfo(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(74);
      self._fieldHandler.getFieldSize(74) }
    // Java: void setFcPlfLfo(int fcPlfLfo)
    pub fn setFcPlfLfo(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(74, fcPlfLfo);
      self._fieldHandler.setFieldOffset(74, v); }
    // Java: void setLcbPlfLfo(int lcbPlfLfo)
    pub fn setLcbPlfLfo(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(74, lcbPlfLfo);
      self._fieldHandler.setFieldSize(74, v); }
    // Java: int getFcSttbfffn()
    pub fn getFcSttbfffn(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(15);
      self._fieldHandler.getFieldOffset(15) }
    // Java: int getLcbSttbfffn()
    pub fn getLcbSttbfffn(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(15);
      self._fieldHandler.getFieldSize(15) }
    // Java: void setFcSttbfffn(int fcSttbFffn)
    pub fn setFcSttbfffn(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(15, fcSttbFffn);
      self._fieldHandler.setFieldOffset(15, v); }
    // Java: void setLcbSttbfffn(int lcbSttbFffn)
    pub fn setLcbSttbfffn(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(15, lcbSttbFffn);
      self._fieldHandler.setFieldSize(15, v); }
    // Java: int getFcSttbfRMark()
    pub fn getFcSttbfRMark(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(51);
      self._fieldHandler.getFieldOffset(51) }
    // Java: int getLcbSttbfRMark()
    pub fn getLcbSttbfRMark(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(51);
      self._fieldHandler.getFieldSize(51) }
    // Java: void setFcSttbfRMark(int fcSttbfRMark)
    pub fn setFcSttbfRMark(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(51, fcSttbfRMark);
      self._fieldHandler.setFieldOffset(51, v); }
    // Java: void setLcbSttbfRMark(int lcbSttbfRMark)
    pub fn setLcbSttbfRMark(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(51, lcbSttbfRMark);
      self._fieldHandler.setFieldSize(51, v); }
    // Java: int getFcSttbRgtplc()
    pub fn getFcSttbRgtplc(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(96);
      self._fieldHandler.getFieldOffset(96) }
    // Java: int getLcbSttbRgtplc()
    pub fn getLcbSttbRgtplc(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(96);
      self._fieldHandler.getFieldSize(96) }
    // Java: void setFcSttbRgtplc(int fcSttbRgtplc)
    pub fn setFcSttbRgtplc(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(96, fcSttbRgtplc);
      self._fieldHandler.setFieldOffset(96, v); }
    // Java: void setLcbSttbRgtplc(int fcSttbRgtplc)
    pub fn setLcbSttbRgtplc(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(96, fcSttbRgtplc);
      self._fieldHandler.setFieldSize(96, v); }
    // Java: int getFcSttbListNames()
    pub fn getFcSttbListNames(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(91);
      self._fieldHandler.getFieldOffset(91) }
    // Java: int getLcbSttbListNames()
    pub fn getLcbSttbListNames(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(91);
      self._fieldHandler.getFieldSize(91) }
    // Java: void setFcSttbListNames(int fcSttbListNames)
    pub fn setFcSttbListNames(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(91, fcSttbListNames);
      self._fieldHandler.setFieldOffset(91, v); }
    // Java: void setLcbSttbListNames(int fcSttbListNames)
    pub fn setLcbSttbListNames(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(91, fcSttbListNames);
      self._fieldHandler.setFieldSize(91, v); }
    // Java: int getPlcfHddOffset()
    pub fn getPlcfHddOffset(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(11);
      self._fieldHandler.getFieldOffset(11) }
    // Java: int getPlcfHddSize()
    pub fn getPlcfHddSize(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(11);
      self._fieldHandler.getFieldSize(11) }
    // Java: void setPlcfHddOffset(int fcPlcfHdd)
    pub fn setPlcfHddOffset(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(11, fcPlcfHdd);
      self._fieldHandler.setFieldOffset(11, v); }
    // Java: void setPlcfHddSize(int lcbPlcfHdd)
    pub fn setPlcfHddSize(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(11, lcbPlcfHdd);
      self._fieldHandler.setFieldSize(11, v); }
    // Java: int getFcPlcffldHdr()
    pub fn getFcPlcffldHdr(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(17);
      self._fieldHandler.getFieldOffset(17) }
    // Java: int getLcbPlcffldHdr()
    pub fn getLcbPlcffldHdr(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(17);
      self._fieldHandler.getFieldSize(17) }
    // Java: void setFcPlcffldHdr(int fcPlcffldHdr)
    pub fn setFcPlcffldHdr(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(17, fcPlcffldHdr);
      self._fieldHandler.setFieldOffset(17, v); }
    // Java: void setLcbPlcffldHdr(int lcbPlcffldHdr)
    pub fn setLcbPlcffldHdr(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(17, lcbPlcffldHdr);
      self._fieldHandler.setFieldSize(17, v); }
    // Java: int getFcPlcfHdrTxbxtxt()
    pub fn getFcPlcfHdrTxbxtxt(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(58);
      self._fieldHandler.getFieldOffset(58) }
    // Java: int getLcbPlcfHdrTxbxtxt()
    pub fn getLcbPlcfHdrTxbxtxt(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(58);
      self._fieldHandler.getFieldSize(58) }
    // Java: void setFcPlcfHdrTxbxtxt(int fcPlcfHdrTxbxtxt)
    pub fn setFcPlcfHdrTxbxtxt(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(58, fcPlcfHdrTxbxtxt);
      self._fieldHandler.setFieldOffset(58, v); }
    // Java: void setLcbPlcfHdrTxbxtxt(int lcbPlcfHdrTxbxtxt)
    pub fn setLcbPlcfHdrTxbxtxt(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(58, lcbPlcfHdrTxbxtxt);
      self._fieldHandler.setFieldSize(58, v); }
    // Java: int getFcPlcfTxbxHdrBkd()
    pub fn getFcPlcfTxbxHdrBkd(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(76);
      self._fieldHandler.getFieldOffset(76) }
    // Java: int getLcbPlcfTxbxHdrBkd()
    pub fn getLcbPlcfTxbxHdrBkd(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(76);
      self._fieldHandler.getFieldSize(76) }
    // Java: void setFcPlcfTxbxHdrBkd(int fcPlcfTxbxHdrBkd)
    pub fn setFcPlcfTxbxHdrBkd(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(76, fcPlcfTxbxHdrBkd);
      self._fieldHandler.setFieldOffset(76, v); }
    // Java: void setLcbPlcfTxbxHdrBkd(int lcbPlcfTxbxHdrBkd)
    pub fn setLcbPlcfTxbxHdrBkd(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(76, lcbPlcfTxbxHdrBkd);
      self._fieldHandler.setFieldSize(76, v); }
    // Java: int getFcPlcffldHdrTxbx()
    pub fn getFcPlcffldHdrTxbx(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(59);
      self._fieldHandler.getFieldOffset(59) }
    // Java: int getLcbPlcffldHdrTxbx()
    pub fn getLcbPlcffldHdrTxbx(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(59);
      self._fieldHandler.getFieldSize(59) }
    // Java: void setFcPlcffldHdrTxbx(int fcPlcffldHdrTxbx)
    pub fn setFcPlcffldHdrTxbx(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(59, fcPlcffldHdrTxbx);
      self._fieldHandler.setFieldOffset(59, v); }
    // Java: void setLcbPlcffldHdrTxbx(int lcbPlcffldHdrTxbx)
    pub fn setLcbPlcffldHdrTxbx(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(59, lcbPlcffldHdrTxbx);
      self._fieldHandler.setFieldSize(59, v); }
    // Java: int getFcSttbSavedBy()
    pub fn getFcSttbSavedBy(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(71);
      self._fieldHandler.getFieldOffset(71) }
    // Java: int getLcbSttbSavedBy()
    pub fn getLcbSttbSavedBy(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(71);
      self._fieldHandler.getFieldSize(71) }
    // Java: void setFcSttbSavedBy(int fcSttbSavedBy)
    pub fn setFcSttbSavedBy(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(71, fcSttbSavedBy);
      self._fieldHandler.setFieldOffset(71, v); }
    // Java: void setLcbSttbSavedBy(int fcSttbSavedBy)
    pub fn setLcbSttbSavedBy(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(71, fcSttbSavedBy);
      self._fieldHandler.setFieldSize(71, v); }
    // Java: int getModifiedLow()
    pub fn getModifiedLow(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(74);
      self._fieldHandler.getFieldOffset(74) }
    // Java: int getModifiedHigh()
    pub fn getModifiedHigh(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(74);
      self._fieldHandler.getFieldSize(74) }
    // Java: void setModifiedLow(int modifiedLow)
    pub fn setModifiedLow(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(74, modifiedLow);
      self._fieldHandler.setFieldOffset(74, v); }
    // Java: void setModifiedHigh(int modifiedHigh)
    pub fn setModifiedHigh(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(74, modifiedHigh);
      self._fieldHandler.setFieldSize(74, v); }
    // Java: int getCbMac()
    pub fn getCbMac(&self) -> i32 { // Java: return this._longHandler.getLong(0);
      self._longHandler.getLong(0) }
    // Java: void setCbMac(int cbMac)
    pub fn setCbMac(&mut self, v: i32) { // Java: this._longHandler.setLong(0, cbMac);
      self._longHandler.setLong(0, v); }
    // Java: int getCcpText()
    pub fn getCcpText(&self) -> i32 { // Java: return this._longHandler.getLong(3);
      self._longHandler.getLong(3) }
    // Java: void setCcpText(int ccpText)
    pub fn setCcpText(&mut self, v: i32) { // Java: this._longHandler.setLong(3, ccpText);
      self._longHandler.setLong(3, v); }
    // Java: int getCcpFtn()
    pub fn getCcpFtn(&self) -> i32 { // Java: return this._longHandler.getLong(4);
      self._longHandler.getLong(4) }
    // Java: void setCcpFtn(int ccpFtn)
    pub fn setCcpFtn(&mut self, v: i32) { // Java: this._longHandler.setLong(4, ccpFtn);
      self._longHandler.setLong(4, v); }
    // Java: int getCcpHdd()
    pub fn getCcpHdd(&self) -> i32 { // Java: return this._longHandler.getLong(5);
      self._longHandler.getLong(5) }
    // Java: void setCcpHdd(int ccpHdd)
    pub fn setCcpHdd(&mut self, v: i32) { // Java: this._longHandler.setLong(5, ccpHdd);
      self._longHandler.setLong(5, v); }
    // Java: int getCcpAtn()
    pub fn getCcpAtn(&self) -> i32 { // Java: return this._longHandler.getLong(7);
      self._longHandler.getLong(7) }
    // Java: void setCcpAtn(int ccpAtn)
    pub fn setCcpAtn(&mut self, v: i32) { // Java: this._longHandler.setLong(7, ccpAtn);
      self._longHandler.setLong(7, v); }
    // Java: int getCcpEdn()
    pub fn getCcpEdn(&self) -> i32 { // Java: return this._longHandler.getLong(8);
      self._longHandler.getLong(8) }
    // Java: void setCcpEdn(int ccpEdn)
    pub fn setCcpEdn(&mut self, v: i32) { // Java: this._longHandler.setLong(8, ccpEdn);
      self._longHandler.setLong(8, v); }
    // Java: int getCcpTxtBx()
    pub fn getCcpTxtBx(&self) -> i32 { // Java: return this._longHandler.getLong(9);
      self._longHandler.getLong(9) }
    // Java: void setCcpTxtBx(int ccpTxtBx)
    pub fn setCcpTxtBx(&mut self, v: i32) { // Java: this._longHandler.setLong(9, ccpTxtBx);
      self._longHandler.setLong(9, v); }
    // Java: int getCcpHdrTxtBx()
    pub fn getCcpHdrTxtBx(&self) -> i32 { // Java: return this._longHandler.getLong(10);
      self._longHandler.getLong(10) }
    // Java: void setCcpHdrTxtBx(int ccpTxtBx)
    pub fn setCcpHdrTxtBx(&mut self, v: i32) { // Java: this._longHandler.setLong(10, ccpTxtBx);
      self._longHandler.setLong(10, v); }
    // Java: void clearOffsetsSizes()
    pub fn clearOffsetsSizes(&mut self) {
      // Java: this._fieldHandler.clearFields();
      self._fieldHandler.clearFields();
    }
    // Java: int getFcPlcffldMom()
    pub fn getFcPlcffldMom(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(16);
      self._fieldHandler.getFieldOffset(16) }
    // Java: int getLcbPlcffldMom()
    pub fn getLcbPlcffldMom(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(16);
      self._fieldHandler.getFieldSize(16) }
    // Java: void setFcPlcffldMom(int fcPlcffldMom)
    pub fn setFcPlcffldMom(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(16, fcPlcffldMom);
      self._fieldHandler.setFieldOffset(16, v); }
    // Java: void setLcbPlcffldMom(int lcbPlcffldMom)
    pub fn setLcbPlcffldMom(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(16, lcbPlcffldMom);
      self._fieldHandler.setFieldSize(16, v); }
    // Java: int getFcPlcspaMom()
    pub fn getFcPlcspaMom(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(40);
      self._fieldHandler.getFieldOffset(40) }
    // Java: int getLcbPlcspaMom()
    pub fn getLcbPlcspaMom(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(40);
      self._fieldHandler.getFieldSize(40) }
    // Java: void setFcPlcspaMom(int fcPlcspaMom)
    pub fn setFcPlcspaMom(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(40, fcPlcspaMom);
      self._fieldHandler.setFieldOffset(40, v); }
    // Java: void setLcbPlcspaMom(int lcbPlcspaMom)
    pub fn setLcbPlcspaMom(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(40, lcbPlcspaMom);
      self._fieldHandler.setFieldSize(40, v); }
    // Java: int getFcPlcspaHdr()
    pub fn getFcPlcspaHdr(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(41);
      self._fieldHandler.getFieldOffset(41) }
    // Java: int getLcbPlcspaHdr()
    pub fn getLcbPlcspaHdr(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(41);
      self._fieldHandler.getFieldSize(41) }
    // Java: void setFcPlcspaHdr(int fcPlcspaHdr)
    pub fn setFcPlcspaHdr(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(41, fcPlcspaHdr);
      self._fieldHandler.setFieldOffset(41, v); }
    // Java: void setLcbPlcspaHdr(int lcb)
    pub fn setLcbPlcspaHdr(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(41, lcb);
      self._fieldHandler.setFieldSize(41, v); }
    // Java: int getFcDggInfo()
    pub fn getFcDggInfo(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(50);
      self._fieldHandler.getFieldOffset(50) }
    // Java: int getLcbDggInfo()
    pub fn getLcbDggInfo(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(50);
      self._fieldHandler.getFieldSize(50) }
    // Java: void setFcDggInfo(int fcDggInfo)
    pub fn setFcDggInfo(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(50, fcDggInfo);
      self._fieldHandler.setFieldOffset(50, v); }
    // Java: void setLcbDggInfo(int lcb)
    pub fn setLcbDggInfo(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(50, lcb);
      self._fieldHandler.setFieldSize(50, v); }
    // Java: int getFcPlcftxbxTxt()
    pub fn getFcPlcftxbxTxt(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(56);
      self._fieldHandler.getFieldOffset(56) }
    // Java: int getLcbPlcftxbxTxt()
    pub fn getLcbPlcftxbxTxt(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(56);
      self._fieldHandler.getFieldSize(56) }
    // Java: void setFcPlcftxbxTxt(int fcPlcftxbxTxt)
    pub fn setFcPlcftxbxTxt(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(56, fcPlcftxbxTxt);
      self._fieldHandler.setFieldOffset(56, v); }
    // Java: void setLcbPlcftxbxTxt(int lcb)
    pub fn setLcbPlcftxbxTxt(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(56, lcb);
      self._fieldHandler.setFieldSize(56, v); }
    // Java: int getFcPlcffldTxbx()
    pub fn getFcPlcffldTxbx(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(57);
      self._fieldHandler.getFieldOffset(57) }
    // Java: int getLcbPlcffldTxbx()
    pub fn getLcbPlcffldTxbx(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(57);
      self._fieldHandler.getFieldSize(57) }
    // Java: void setFcPlcffldTxbx(int fcPlcffldtxbx)
    pub fn setFcPlcffldTxbx(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(57, fcPlcffldtxbx);
      self._fieldHandler.setFieldOffset(57, v); }
    // Java: void setLcbPlcffldTxbx(int lcb)
    pub fn setLcbPlcffldTxbx(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(57, lcb);
      self._fieldHandler.setFieldSize(57, v); }
    // Java: int getFcPlcftxbxbkd()
    pub fn getFcPlcftxbxbkd(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(75);
      self._fieldHandler.getFieldOffset(75) }
    // Java: int getLcbPlcftxbxbkd()
    pub fn getLcbPlcftxbxbkd(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(75);
      self._fieldHandler.getFieldSize(75) }
    // Java: void setFcPlcftxbxbkd(int fcPlcftxbxbkd)
    pub fn setFcPlcftxbxbkd(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(75, fcPlcftxbxbkd);
      self._fieldHandler.setFieldOffset(75, v); }
    // Java: void setLcbPlcftxbxbkd(int lcb)
    pub fn setLcbPlcftxbxbkd(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(75, lcb);
      self._fieldHandler.setFieldSize(75, v); }
    // Java: int getFcPlrsid()
    pub fn getFcPlrsid(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(113);
      self._fieldHandler.getFieldOffset(113) }
    // Java: int getLcbPlrsid()
    pub fn getLcbPlrsid(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(113);
      self._fieldHandler.getFieldSize(113) }
    // Java: void setFcPlrsid(int fcPlrsid)
    pub fn setFcPlrsid(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(113, fcPlrsid);
      self._fieldHandler.setFieldOffset(113, v); }
    // Java: void setLcbPlrsid(int lcb)
    pub fn setLcbPlrsid(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(113, lcb);
      self._fieldHandler.setFieldSize(113, v); }
    // Java: int getFcPlcfandRef()
    pub fn getFcPlcfandRef(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(4);
      self._fieldHandler.getFieldOffset(4) }
    // Java: int getLcbPlcfandRef()
    pub fn getLcbPlcfandRef(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(4);
      self._fieldHandler.getFieldSize(4) }
    // Java: void setFcPlcfandRef(int fcPlcfandRef)
    pub fn setFcPlcfandRef(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(4, fcPlcfandRef);
      self._fieldHandler.setFieldOffset(4, v); }
    // Java: void setLcbPlcfandRef(int lcb)
    pub fn setLcbPlcfandRef(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(4, lcb);
      self._fieldHandler.setFieldSize(4, v); }
    // Java: int getFcPlcfandTxt()
    pub fn getFcPlcfandTxt(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(5);
      self._fieldHandler.getFieldOffset(5) }
    // Java: int getLcbPlcfandTxt()
    pub fn getLcbPlcfandTxt(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(5);
      self._fieldHandler.getFieldSize(5) }
    // Java: void setFcPlcfandTxt(int fcPlcfandTxt)
    pub fn setFcPlcfandTxt(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(5, fcPlcfandTxt);
      self._fieldHandler.setFieldOffset(5, v); }
    // Java: void setLcbPlcfandTxt(int lcb)
    pub fn setLcbPlcfandTxt(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(5, lcb);
      self._fieldHandler.setFieldSize(5, v); }
    // Java: int getFcGrpXstAtnOwners()
    pub fn getFcGrpXstAtnOwners(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(36);
      self._fieldHandler.getFieldOffset(36) }
    // Java: int getLcbGrpXstAtnOwners()
    pub fn getLcbGrpXstAtnOwners(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(36);
      self._fieldHandler.getFieldSize(36) }
    // Java: void setFcGrpXstAtnOwners(int fcGrpXstAtnOwners)
    pub fn setFcGrpXstAtnOwners(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(36, fcGrpXstAtnOwners);
      self._fieldHandler.setFieldOffset(36, v); }
    // Java: void setLcbGrpXstAtnOwners(int lcb)
    pub fn setLcbGrpXstAtnOwners(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(36, lcb);
      self._fieldHandler.setFieldSize(36, v); }
    // Java: int getFcPlcfAtnbkf()
    pub fn getFcPlcfAtnbkf(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(42);
      self._fieldHandler.getFieldOffset(42) }
    // Java: int getLcbPlcfAtnbkf()
    pub fn getLcbPlcfAtnbkf(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(42);
      self._fieldHandler.getFieldSize(42) }
    // Java: void setFcPlcfAtnbkf(int fcPlcfAtnbkf)
    pub fn setFcPlcfAtnbkf(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(42, fcPlcfAtnbkf);
      self._fieldHandler.setFieldOffset(42, v); }
    // Java: void setLcbPlcfAtnbkf(int lcb)
    pub fn setLcbPlcfAtnbkf(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(42, lcb);
      self._fieldHandler.setFieldSize(42, v); }
    // Java: int getFcPlcfAtnbkl()
    pub fn getFcPlcfAtnbkl(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(43);
      self._fieldHandler.getFieldOffset(43) }
    // Java: int getLcbPlcfAtnbkl()
    pub fn getLcbPlcfAtnbkl(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(43);
      self._fieldHandler.getFieldSize(43) }
    // Java: void setFcPlcfAtnbkl(int fcPlcfAtnbkl)
    pub fn setFcPlcfAtnbkl(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(43, fcPlcfAtnbkl);
      self._fieldHandler.setFieldOffset(43, v); }
    // Java: void setLcbPlcfAtnbkl(int lcb)
    pub fn setLcbPlcfAtnbkl(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(43, lcb);
      self._fieldHandler.setFieldSize(43, v); }
    // Java: int getFcAtrdExtra()
    pub fn getFcAtrdExtra(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(112);
      self._fieldHandler.getFieldOffset(112) }
    // Java: int getLcbAtrdExtra()
    pub fn getLcbAtrdExtra(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(112);
      self._fieldHandler.getFieldSize(112) }
    // Java: void setFcAtrdExtra(int fcAtrdExtra)
    pub fn setFcAtrdExtra(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(112, fcAtrdExtra);
      self._fieldHandler.setFieldOffset(112, v); }
    // Java: void setLcbAtrdExtra(int lcb)
    pub fn setLcbAtrdExtra(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(112, lcb);
      self._fieldHandler.setFieldSize(112, v); }
    // Java: int getFcSttbfAtnbkmk()
    pub fn getFcSttbfAtnbkmk(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(37);
      self._fieldHandler.getFieldOffset(37) }
    // Java: int getLcbSttbfAtnbkmk()
    pub fn getLcbSttbfAtnbkmk(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(37);
      self._fieldHandler.getFieldSize(37) }
    // Java: void setFcSttbfAtnbkmk(int fcSttbfAtnbkmk)
    pub fn setFcSttbfAtnbkmk(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(37, fcSttbfAtnbkmk);
      self._fieldHandler.setFieldOffset(37, v); }
    // Java: void setLcbSttbfAtnbkmk(int lcb)
    pub fn setLcbSttbfAtnbkmk(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(37, lcb);
      self._fieldHandler.setFieldSize(37, v); }
    // Java: int getFcPlcffldAtn()
    pub fn getFcPlcffldAtn(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(19);
      self._fieldHandler.getFieldOffset(19) }
    // Java: int getLcbPlcffldAtn()
    pub fn getLcbPlcffldAtn(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(19);
      self._fieldHandler.getFieldSize(19) }
    // Java: void setFcPlcffldAtn(int fcPlcffldAtn)
    pub fn setFcPlcffldAtn(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(19, fcPlcffldAtn);
      self._fieldHandler.setFieldOffset(19, v); }
    // Java: void setLcbPlcffldAtn(int lcbPlcffldAtn)
    pub fn setLcbPlcffldAtn(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(19, lcbPlcffldAtn);
      self._fieldHandler.setFieldSize(19, v); }
    // Java: int getFcSttbfbkmk()
    pub fn getFcSttbfbkmk(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(21);
      self._fieldHandler.getFieldOffset(21) }
    // Java: int getLcbSttbfbkmk()
    pub fn getLcbSttbfbkmk(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(21);
      self._fieldHandler.getFieldSize(21) }
    // Java: void setFcSttbfbkmk(int fcSttbfbkmk)
    pub fn setFcSttbfbkmk(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(21, fcSttbfbkmk);
      self._fieldHandler.setFieldOffset(21, v); }
    // Java: void setLcbSttbfbkmk(int lcbSttbfbkmk)
    pub fn setLcbSttbfbkmk(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(21, lcbSttbfbkmk);
      self._fieldHandler.setFieldSize(21, v); }
    // Java: int getFcPlcfbkf()
    pub fn getFcPlcfbkf(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(22);
      self._fieldHandler.getFieldOffset(22) }
    // Java: int getLcbPlcfbkf()
    pub fn getLcbPlcfbkf(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(22);
      self._fieldHandler.getFieldSize(22) }
    // Java: void setFcPlcfbkf(int fcPlcfbkf)
    pub fn setFcPlcfbkf(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(22, fcPlcfbkf);
      self._fieldHandler.setFieldOffset(22, v); }
    // Java: void setLcbPlcfbkf(int lcbPlcfbkf)
    pub fn setLcbPlcfbkf(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(22, lcbPlcfbkf);
      self._fieldHandler.setFieldSize(22, v); }
    // Java: int getFcPlcfbkl()
    pub fn getFcPlcfbkl(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(23);
      self._fieldHandler.getFieldOffset(23) }
    // Java: int getLcbPlcfbkl()
    pub fn getLcbPlcfbkl(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(23);
      self._fieldHandler.getFieldSize(23) }
    // Java: void setFcPlcfbkl(int fcPlcfbkl)
    pub fn setFcPlcfbkl(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(23, fcPlcfbkl);
      self._fieldHandler.setFieldOffset(23, v); }
    // Java: void setLcbPlcfbkl(int lcbPlcfbkl)
    pub fn setLcbPlcfbkl(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(23, lcbPlcfbkl);
      self._fieldHandler.setFieldSize(23, v); }
    // Java: int getCreatedPrivate()
    pub fn getCreatedPrivate(&self) -> i32 { // Java: return this._shortHandler.getShort(2);
      self._shortHandler.getShort(2) }
    // Java: void setCreatedPrivate(short wMagicCreatedPrivate)
    pub fn setCreatedPrivate(&mut self, v: i16) { // Java: this._shortHandler.setShort(2, wMagicCreatedPrivate);
      self._shortHandler.setShort(2, v); }
    // Java: int getFcPlcffndRef()
    pub fn getFcPlcffndRef(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(2);
      self._fieldHandler.getFieldOffset(2) }
    // Java: int getLcbPlcffndRef()
    pub fn getLcbPlcffndRef(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(2);
      self._fieldHandler.getFieldSize(2) }
    // Java: void setFcPlcffndRef(int fcPlcffndRef)
    pub fn setFcPlcffndRef(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(2, fcPlcffndRef);
      self._fieldHandler.setFieldOffset(2, v); }
    // Java: void setLcbPlcffndRef(int lcbPlcffndRef)
    pub fn setLcbPlcffndRef(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(2, lcbPlcffndRef);
      self._fieldHandler.setFieldSize(2, v); }
    // Java: int getFcPlcffndTxt()
    pub fn getFcPlcffndTxt(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(3);
      self._fieldHandler.getFieldOffset(3) }
    // Java: int getLcbPlcffndTxt()
    pub fn getLcbPlcffndTxt(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(3);
      self._fieldHandler.getFieldSize(3) }
    // Java: void setFcPlcffndTxt(int fcPlcffndTxt)
    pub fn setFcPlcffndTxt(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(3, fcPlcffndTxt);
      self._fieldHandler.setFieldOffset(3, v); }
    // Java: void setLcbPlcffndTxt(int lcbPlcffndTxt)
    pub fn setLcbPlcffndTxt(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(3, lcbPlcffndTxt);
      self._fieldHandler.setFieldSize(3, v); }
    // Java: int getFcPlcffldFtn()
    pub fn getFcPlcffldFtn(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(18);
      self._fieldHandler.getFieldOffset(18) }
    // Java: int getLcbPlcffldFtn()
    pub fn getLcbPlcffldFtn(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(18);
      self._fieldHandler.getFieldSize(18) }
    // Java: void setFcPlcffldFtn(int fcPlcffldFtn)
    pub fn setFcPlcffldFtn(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(18, fcPlcffldFtn);
      self._fieldHandler.setFieldOffset(18, v); }
    // Java: void setLcbPlcffldFtn(int lcbPlcffldFtn)
    pub fn setLcbPlcffldFtn(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(18, lcbPlcffldFtn);
      self._fieldHandler.setFieldSize(18, v); }
    // Java: int getFcPlcfendRef()
    pub fn getFcPlcfendRef(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(46);
      self._fieldHandler.getFieldOffset(46) }
    // Java: int getLcbPlcfendRef()
    pub fn getLcbPlcfendRef(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(46);
      self._fieldHandler.getFieldSize(46) }
    // Java: void setFcPlcfendRef(int fcPlcfendRef)
    pub fn setFcPlcfendRef(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(46, fcPlcfendRef);
      self._fieldHandler.setFieldOffset(46, v); }
    // Java: void setLcbPlcfendRef(int lcbPlcfendRef)
    pub fn setLcbPlcfendRef(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(46, lcbPlcfendRef);
      self._fieldHandler.setFieldSize(46, v); }
    // Java: int getFcPlcfendTxt()
    pub fn getFcPlcfendTxt(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(47);
      self._fieldHandler.getFieldOffset(47) }
    // Java: int getLcbPlcfendTxt()
    pub fn getLcbPlcfendTxt(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(47);
      self._fieldHandler.getFieldSize(47) }
    // Java: void setFcPlcfendTxt(int fcPlcfendTxt)
    pub fn setFcPlcfendTxt(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(47, fcPlcfendTxt);
      self._fieldHandler.setFieldOffset(47, v); }
    // Java: void setLcbPlcfendTxt(int lcbPlcfendTxt)
    pub fn setLcbPlcfendTxt(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(47, lcbPlcfendTxt);
      self._fieldHandler.setFieldSize(47, v); }
    // Java: int getFcPlcffldEdn()
    pub fn getFcPlcffldEdn(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(48);
      self._fieldHandler.getFieldOffset(48) }
    // Java: int getLcbPlcffldEdn()
    pub fn getLcbPlcffldEdn(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(48);
      self._fieldHandler.getFieldSize(48) }
    // Java: void setFcPlcffldEdn(int fcPlcffldEdn)
    pub fn setFcPlcffldEdn(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(48, fcPlcffldEdn);
      self._fieldHandler.setFieldOffset(48, v); }
    // Java: void setLcbPlcffldEdn(int lcbPlcffldEdn)
    pub fn setLcbPlcffldEdn(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(48, lcbPlcffldEdn);
      self._fieldHandler.setFieldSize(48, v); }
    // Java: int getFcStwUser()
    pub fn getFcStwUser(&self) -> i32 { // Java: return this._fieldHandler.getFieldOffset(60);
      self._fieldHandler.getFieldOffset(60) }
    // Java: int getLcbStwUser()
    pub fn getLcbStwUser(&self) -> i32 { // Java: return this._fieldHandler.getFieldSize(60);
      self._fieldHandler.getFieldSize(60) }
    // Java: void setFcStwUser(int fcStwUser)
    pub fn setFcStwUser(&mut self, v: i32) { // Java: this._fieldHandler.setFieldOffset(60, fcStwUser);
      self._fieldHandler.setFieldOffset(60, v); }
    // Java: void setLcbStwUser(int lcbStwUser)
    pub fn setLcbStwUser(&mut self, v: i32) { // Java: this._fieldHandler.setFieldSize(60, lcbStwUser);
      self._fieldHandler.setFieldSize(60, v); }

    // Java: void writeTo(byte[] mainStream, HWPFOutputStream tableStream)
    pub fn writeTo(&mut self, main_stream: &mut [u8], table_stream: &mut HWPFOutputStream) {
      // Java: super.serialize(mainStream, 0);
      self._base.serialize(main_stream, 0);
      // Java: int size = super.getSize();
      let size = self._base.getSize() as usize;
      // Java: this._shortHandler.serialize(mainStream);
      self._shortHandler.serialize(main_stream);
      // Java: this._longHandler.serialize(mainStream, size + this._shortHandler.sizeInBytes());
      let long_offset = size + self._shortHandler.sizeInBytes() as usize;
      self._longHandler.serialize(main_stream, long_offset);
      // Java: this._fieldHandler.writeTo(mainStream, super.getSize() + this._shortHandler.sizeInBytes() + this._longHandler.sizeInBytes(), tableStream);
      let field_offset = self._base.getSize() as usize + self._shortHandler.sizeInBytes() as usize + self._longHandler.sizeInBytes() as usize;
      self._fieldHandler.writeTo(main_stream, field_offset, table_stream);
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return super.getSize() + this._shortHandler.sizeInBytes() + this._longHandler.sizeInBytes() + this._fieldHandler.sizeInBytes();
      self._base.getSize() + self._shortHandler.sizeInBytes() + self._longHandler.sizeInBytes() + self._fieldHandler.sizeInBytes()
    }

    // Java: IIntObjectMap<UnhandledDataStructure> getUnhandleDataMap()
    pub fn getUnhandleDataMap(&mut self) -> &mut HashMap<i32, UnhandledDataStructure> {
      // Java: return this._fieldHandler.getUnhandleDataMap();
      self._fieldHandler.getUnhandleDataMap()
    }

    // Java: FIBLongHandler getFIBLongHandler()
    pub fn getFIBLongHandler(&self) -> &FIBLongHandler {
      // Java: return this._longHandler;
      &self._longHandler
    }

    // Java: FIBShortHandler getFIBShortHandler()
    pub fn getFIBShortHandler(&self) -> &FIBShortHandler {
      // Java: return this._shortHandler;
      &self._shortHandler
    }

    // Java: FIBFieldHandler getFIBFieldHandler()
    pub fn getFIBFieldHandler(&self) -> &FIBFieldHandler {
      // Java: return this._fieldHandler;
      &self._fieldHandler
    }

}
