use kfc::cn::wps::memor4j::KmoAtomData::{KmoAtomData,KmoAtomDataSub};
// java package cn.wps.moss.filefmt.biff8.record.common;
// java
// java import cn.wps.memor4j.KmoAtomData;
// java
// java class KmoFormatRun extends KmoAtomData {
// java short _character;
// java short _fontIndex;
// java
// java public KmoFormatRun(short character, short fontIndex) {
// java this._character = character;
// java this._fontIndex = fontIndex;
// java }
// java
// java public KmoAtomData copy() {
// java KmoFormatRun formatRun = new KmoFormatRun(this._character, this._fontIndex);
// java return formatRun;
// java }
// java }
#[derive(Clone)]pub struct KmoFormatRun{pub base:KmoAtomData,pub _character:i16,pub _fontIndex:i16}
impl KmoFormatRun{pub fn new(_character:i16,_fontIndex:i16)->Self{Self{base:KmoAtomData::new(),_character,_fontIndex}}}
impl KmoAtomDataSub for KmoFormatRun{fn as_base(&self)->&KmoAtomData{&self.base}fn as_base_mut(&mut self)->&mut KmoAtomData{&mut self.base}fn copy(&self)->Self{Self{base:KmoAtomData::new(),_character:self._character,_fontIndex:self._fontIndex}}}
