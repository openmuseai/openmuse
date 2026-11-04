// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.kfc.api.iterator.IIntIterator
// Java: import cn.wps.kfc.api.iterator.IIntObjectIterator
// Java: import cn.wps.kfc.api.map.IIntObjectMap
// Java: import cn.wps.kfc.api.set.IIntSet
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
// Java: import java.io.ByteArrayOutputStream
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.Collection
// Java: import java.util.NoSuchElementException
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFFileSystem::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::ListData::ListData;
use super::ListFormatOverride::ListFormatOverride;
use super::ListFormatOverrideLevel::ListFormatOverrideLevel;
use super::ListLevel::ListLevel;
use std::collections::HashMap;

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    let bytes = value.to_le_bytes();
    data[offset..offset + 4].copy_from_slice(&bytes);
}

// Java field: int LFO_ZERO
pub const LFO_ZERO: i32 = 0;
// Java field: int LFO_NOT_IN_LIST
pub const LFO_NOT_IN_LIST: i32 = 63489;
// Java field: int LFO_POSITIVE_MIN
pub const LFO_POSITIVE_MIN: i32 = 1;
// Java field: int LFO_POSITIVE_MAX
pub const LFO_POSITIVE_MAX: i32 = 2046;
// Java field: int LFO_NEGATION_MIN
pub const LFO_NEGATION_MIN: i32 = 63490;
// Java field: int LFO_NEGATION_MAX
pub const LFO_NEGATION_MAX: i32 = 65535;
// Java field: int LIST_DATA_SIZE
pub(crate) const LIST_DATA_SIZE: i32 = 28;
// Java field: int LIST_FORMAT_OVERRIDE_SIZE
pub(crate) const LIST_FORMAT_OVERRIDE_SIZE: i32 = 16;
// Java field: int INIT_SIZE
pub(crate) const INIT_SIZE: i32 = -1;

// Java: public class ListMap
pub struct ListMap {
    // Java field: IIntObjectMap<ListData> parent
    parent: HashMap<i32, ListData>,
}

// Java: public class ListTables
pub struct ListTables {
    // Java field: int _bytesOfOverrideList
    pub _bytesOfOverrideList: i32,
    // Java field: ListMap _listMap
    pub _listMap: ListMap,
    // Java field: ArrayList<ListFormatOverride> _overrideList
    _overrideList: Vec<ListFormatOverride>,
    // Java field: DocumentInputStream _documentStream
    _documentStream: Option<*mut DocumentInputStream>,
    // Java field: int _lstSize
    pub _lstSize: i32,
    // Java field: int _lfoSize
    pub _lfoSize: i32,
    // Java field: int _lstOffset
    _lstOffset: i32,
    // Java field: int _lfoOffset
    _lfoOffset: i32,
    // Java field: int _lstLevelOffset
    _lstLevelOffset: i32,
    // Java field: int _lfolvlOffset
    _lfolvlOffset: i32,
}

// Java: public class ListTables
impl ListTables {
    // Java: ListTables()
    pub fn new() -> Self {
        // Java: super();
        ListTables {
            _bytesOfOverrideList: 0,
            // Java: this._documentStream = null;
            _documentStream: None,
            _listMap: ListMap::new(),
            _overrideList: Vec::new(),
            _lstSize: INIT_SIZE,
            _lfoSize: INIT_SIZE,
            // Java: this._lstOffset = 0;
            _lstOffset: 0,
            // Java: this._lfoOffset = 0;
            _lfoOffset: 0,
            _lstLevelOffset: 0,
            _lfolvlOffset: 0,
        }
    }

    // Java: ListTables(DocumentInputStream tableStream, int lstOffset, int lfoOffset)
    pub fn new_from_stream(table_stream: *mut DocumentInputStream, lst_offset: i32, lfo_offset: i32) -> Self {
        // Java: super();
        // Java: this._documentStream = tableStream;
        let stream = unsafe { &mut *table_stream };
        // Java: tableStream.seek((long)lstOffset);
        stream.seek(lst_offset as i64);
        // Java: this._lstSize = tableStream.readUShort();
        let _lstSize = stream.readUShort() as i32;
        // Java: this._lstOffset = lstOffset + 2;
        let _lstOffset = lst_offset + 2;
        // Java: this._lstLevelOffset = this._lstOffset + this._lstSize * 28;
        let _lstLevelOffset = _lstOffset + _lstSize * LIST_DATA_SIZE;
        // Java: tableStream.seek((long)lfoOffset);
        stream.seek(lfo_offset as i64);
        // Java: this._lfoSize = tableStream.readInt();
        let _lfoSize = stream.readInt();
        // Java: this._lfoOffset = lfoOffset + 4;
        let _lfoOffset = lfo_offset + 4;
        // Java: this._lfolvlOffset = this._lfoOffset + 16 * this._lfoSize;
        let _lfolvlOffset = _lfoOffset + LIST_FORMAT_OVERRIDE_SIZE * _lfoSize;

        ListTables {
            _bytesOfOverrideList: 0,
            _documentStream: Some(table_stream),
            _listMap: ListMap::new(),
            _overrideList: Vec::new(),
            _lstSize,
            _lfoSize,
            _lstOffset,
            _lfoOffset,
            _lstLevelOffset,
            _lfolvlOffset,
        }
    }

    // Java: ListData getLstFromStream(int index)
    pub fn getLstFromStream(&mut self, index: i32) -> ListData {
        let stream = unsafe { &mut *self._documentStream.unwrap() };
        // Java: ListData lst = new ListData(this._documentStream, this._lstOffset + 28 * index);
        let mut lst = ListData::new_from_stream(stream, self._lstOffset + LIST_DATA_SIZE * index);
        // Java: int num = lst.numLevels();
        let num = lst.numLevels();
        // Java: for(int y = 0; y < num; ++y)
        for _y in 0..num {
            // Java: ListLevel lvl = new ListLevel(this._documentStream, this._lstLevelOffset);
            let lvl = ListLevel::new_from_stream(stream, self._lstLevelOffset);
            // Java: lst.setLevel(y, lvl);
            lst.setLevel(_y, lvl);
            // Java: this._lstLevelOffset += lvl.getSizeInBytes();
            self._lstLevelOffset += lst.getLevelAt(_y).getSizeInBytes();
        }
        // Java: return lst;
        lst
    }

    // Java: ListFormatOverride getLfoFromStream(int index)
    pub fn getLfoFromStream(&mut self, index: i32) -> ListFormatOverride {
        let stream = unsafe { &mut *self._documentStream.unwrap() };
        // Java: ListFormatOverride lfo = new ListFormatOverride(this._documentStream, this._lfoOffset + 16 * index);
        let mut lfo = ListFormatOverride::new_from_stream(stream, self._lfoOffset + LIST_FORMAT_OVERRIDE_SIZE * index);
        // Java: int num = lfo.numOverrides();
        let num = lfo.numOverrides();
        // Java: this._lfolvlOffset += 4;
        self._lfolvlOffset += 4;
        // Java: if (num > 0)
        if num > 0 {
            // Java: this._documentStream.seek((long)this._lfolvlOffset);
            stream.seek(self._lfolvlOffset as i64);
            // Java: for(int y = 0; y < num; ++y)
            for _y in 0..num {
                // Java: ListFormatOverrideLevel lfolvl = new ListFormatOverrideLevel(this._documentStream, this._lfolvlOffset);
                let lfolvl = ListFormatOverrideLevel::new_from_stream(stream, self._lfolvlOffset);
                // Java: lfo.setOverride(y, lfolvl);
                let size = lfolvl.getSizeInBytes();
                lfo.setOverride(_y, lfolvl);
                // Java: this._lfolvlOffset += lfolvl.getSizeInBytes();
                self._lfolvlOffset += size;
            }
        }
        // Java: return lfo;
        lfo
    }

    // Java: int addList(ListData lst, ListFormatOverride override)
    pub fn addList(&mut self, lst: ListData, lfo: ListFormatOverride) -> i32 {
        // Java: Integer lsid = lst.getLsid();
        let lsid = lst.getLsid();
        // Java: if (null == this._listMap.get(lsid))
        if self._listMap.get(lsid).is_none() {
            // Java: this._listMap.put(lsid, lst);
            self._listMap.put(lsid, lst);
        }
        // Java: this._overrideList.add(override);
        self._overrideList.push(lfo);
        // Java: return lsid;
        lsid
    }

    // Java: int[] getSortedLsid()
    pub fn getSortedLsid(&self) -> Vec<i32> {
        // Java: return this._listMap.sortedKeys();
        self._listMap.sortedKeys()
    }

    // Java: void writeListDataTo(HWPFOutputStream tableStream)
    pub fn writeListDataTo(&self, table_stream: &mut HWPFOutputStream) {
        // Java: int listSize = this._listMap.size();
        let list_size = self._listMap.size();
        // Java: byte[] shortHolder = new byte[2];
        let mut short_holder = [0u8; 2];
        // Java: LittleEndian.putShort(shortHolder, (short)listSize);
        le_put_short(&mut short_holder, 0, list_size as i16);
        // Java: tableStream.write(shortHolder);
        table_stream.write(&short_holder);
        // Java: ByteArrayOutputStream levelBuf = new ByteArrayOutputStream();
        let mut level_buf: Vec<u8> = Vec::new();
        // Java: int[] sortedKeys = this._listMap.sortedKeys();
        let sorted_keys = self._listMap.sortedKeys();
        // Java: for(int i = 0; i < sortedKeys.length; ++i)
        for i in 0..sorted_keys.len() {
            // Java: ListData lst = this._listMap.get(sortedKeys[i]);
            let lst = self._listMap.get(sorted_keys[i]).unwrap();
            // Java: IOAssert.assertNotNull("lst should not be null.", lst);
            // Java: byte[] lstContent = lst.toByteArray();
            let lst_content = lst.toByteArray();
            // Java: tableStream.write(lstContent);
            table_stream.write(&lst_content);
        }
        // Java: for(int i = 0; i < sortedKeys.length; ++i)
        for i in 0..sorted_keys.len() {
            // Java: ListData lst = this._listMap.get(sortedKeys[i]);
            let lst = self._listMap.get(sorted_keys[i]).unwrap();
            // Java: IOAssert.assertNotNull("lst should not be null.", lst);
            // Java: ListLevel[] lvls = lst.getLevels();
            let lvls = lst.getLevels();
            // Java: int y = 0;
            // Java: for(int len = lvls.length; y < len; ++y)
            for y in 0..lvls.len() {
                // Java: ListLevel lvl = lvls[y];
                let lvl = &lvls[y];
                // Java: IOAssert.assertNotNull("lvl should not be null.", lvl);
                // Java: byte[] lvlContent = lvl.toByteArray();
                let lvl_content = lvl.toByteArray();
                // Java: levelBuf.write(lvlContent);
                level_buf.extend_from_slice(&lvl_content);
            }
        }
        // Java: byte[] levelBufContent = levelBuf.toByteArray();
        // Java: tableStream.write(levelBufContent);
        table_stream.write(&level_buf);
    }

    // Java: void writeListOverridesTo(HWPFOutputStream tableStream)
    pub fn writeListOverridesTo(&mut self, table_stream: &mut HWPFOutputStream) {
        // Java: ByteArrayOutputStream levelBuf = new ByteArrayOutputStream();
        let mut level_buf: Vec<u8> = Vec::new();
        // Java: int size = this._overrideList.size();
        let size = self._overrideList.len();
        // Java: byte[] intHolder = new byte[4];
        let mut int_holder = [0u8; 4];
        // Java: this._bytesOfOverrideList = intHolder.length;
        self._bytesOfOverrideList = 4;
        // Java: LittleEndian.putInt(intHolder, size);
        le_put_int(&mut int_holder, 0, size as i32);
        // Java: tableStream.write(intHolder);
        table_stream.write(&int_holder);
        // Java: for(int x = 0; x < size; ++x)
        for x in 0..size {
            // Java: ListFormatOverride lfo = (ListFormatOverride)this._overrideList.get(x);
            let lfo = &self._overrideList[x];
            // Java: byte[] lfoContent = lfo.toByteArray();
            let lfo_content = lfo.toByteArray();
            // Java: this._bytesOfOverrideList += lfoContent.length;
            self._bytesOfOverrideList += lfo_content.len() as i32;
            // Java: tableStream.write(lfoContent);
            table_stream.write(&lfo_content);
        }
        // Java: byte[] head = new byte[4];
        let mut head = [0u8; 4];
        // Java: LittleEndian.putInt(head, -1);
        le_put_int(&mut head, 0, -1);
        // Java: for(int x = 0; x < size; ++x)
        for x in 0..size {
            // Java: ListFormatOverride lfo = (ListFormatOverride)this._overrideList.get(x);
            let lfo = &self._overrideList[x];
            // Java: ListFormatOverrideLevel[] lfolvls = lfo.getLevelOverrides();
            let lfolvls = lfo.getLevelOverrides();
            // Java: if (lfolvls.length == 0)
            if lfolvls.is_empty() {
                // Java: levelBuf.write(head);
                level_buf.extend_from_slice(&head);
            }
            // Java: else
            else {
                // Java: levelBuf.write(head);
                level_buf.extend_from_slice(&head);
                // Java: int y = 0;
                // Java: for(int lfoLvlsLen = lfolvls.length; y < lfoLvlsLen; ++y)
                for y in 0..lfolvls.len() {
                    // Java: levelBuf.write(lfolvls[y].toByteArray());
                    level_buf.extend_from_slice(&lfolvls[y].toByteArray());
                }
            }
        }
        // Java: byte[] levelBufContent = levelBuf.toByteArray();
        // Java: this._bytesOfOverrideList += levelBufContent.length;
        self._bytesOfOverrideList += level_buf.len() as i32;
        // Java: tableStream.write(levelBufContent);
        table_stream.write(&level_buf);
    }

    // Java: int getLfoTableSize()
    pub fn getLfoTableSize(&self) -> i32 {
        // Java: return this._lfoSize != -1 ? this._lfoSize : this._overrideList.size();
        if self._lfoSize != -1 { self._lfoSize } else { self._overrideList.len() as i32 }
    }

    // Java: void setOverrideList(ArrayList<ListFormatOverride> overrideList)
    pub fn setOverrideList(&mut self, override_list: Option<Vec<ListFormatOverride>>) {
        // Java: this._overrideList.clear();
        self._overrideList.clear();
        // Java: if (overrideList != null)
        if let Some(list) = override_list {
            // Java: this._overrideList.addAll(overrideList);
            self._overrideList.extend(list);
        }
    }

    // Java: int getOverrideIndexFromListID(int lstid)
    pub fn getOverrideIndexFromListID(&self, lstid: i32) -> i32 {
        // Java: int returnVal = -1;
        let mut return_val: i32 = -1;
        // Java: int size = this._overrideList.size();
        let size = self._overrideList.len();
        // Java: for(int x = 0; x < size; ++x)
        for x in 0..size {
            // Java: ListFormatOverride next = (ListFormatOverride)this._overrideList.get(x);
            let next = &self._overrideList[x];
            // Java: if (next.getLsid() == lstid)
            if next.getLsid() == lstid {
                // Java: returnVal = x + 1;
                return_val = (x + 1) as i32;
                // Java: break;
                break;
            }
        }
        // Java: if (returnVal == -1)
        if return_val == -1 {
            // Java: throw new NoSuchElementException("No list found with the specified ID");
            panic!("No list found with the specified ID");
        }
        // Java: else
        else {
            // Java: return returnVal;
            return_val
        }
    }

    // Java: ListLevel getLevel(int listID, int level)
    pub fn getLevel(&self, list_id: i32, level: i32) -> Option<&ListLevel> {
        // Java: ListData lst = this._listMap.get(Integer.valueOf(listID));
        let lst = self._listMap.get(list_id)?;
        // Java: if (level < lst.numLevels())
        if level < lst.numLevels() {
            // Java: ListLevel lvl = lst.getLevels()[level];
            // Java: return lvl;
            Some(&lst.getLevels()[level as usize])
        }
        // Java: else
        else {
            // Java: return null;
            None
        }
    }

    // Java: ListData getListData(int listID)
    pub fn getListData(&self, list_id: i32) -> Option<&ListData> {
        // Java: return this._listMap.get(Integer.valueOf(listID));
        self._listMap.get(list_id)
    }

    // Java: int getLstTableSize()
    pub fn getLstTableSize(&self) -> i32 {
        // Java: return this._lstSize != -1 ? this._lstSize : this._listMap.size();
        if self._lstSize != -1 { self._lstSize } else { self._listMap.size() }
    }

    // Java: Collection<ListData> getValues()
    pub fn getValues(&self) -> Vec<&ListData> {
        // Java: return this._listMap.values();
        self._listMap.values()
    }

    // Java: void setListMaps(ListMap listMap)
    pub fn setListMaps(&mut self, list_map: Option<ListMap>) {
        // Java: if (listMap != null)
        if let Some(lm) = list_map {
            // Java: this._listMap.putAll(listMap);
            self._listMap.putAll(&lm);
        }
        // Java: else
        else {
            // Java: this._listMap.clear();
            self._listMap.clear();
        }
    }

    // Java: int getLcbPlcfLst()
    pub fn getLcbPlcfLst(&self) -> i32 {
        // Java: return this._listMap.size() * 28 + 2;
        self._listMap.size() * LIST_DATA_SIZE + 2
    }

    // Java: int getLcbPlcfLfo()
    pub fn getLcbPlcfLfo(&self) -> i32 {
        // Java: return this._bytesOfOverrideList;
        self._bytesOfOverrideList
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return super.hashCode();
        let ptr = self as *const Self as usize;
        ptr as i32
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, obj: &ListTables) -> bool {
        // Java: if (obj == null)
        // Java: return false;
        // Java: else
        {
            // Java: ListTables tables = (ListTables)obj;
            // Java: if (this._listMap.size() == tables._listMap.size())
            if self._listMap.size() == obj._listMap.size() {
                // Java: IIntIterator it = this._listMap.keySet().iterator();
                let keys = self._listMap.sortedKeys();
                // Java: while(it.hasNext())
                for key in &keys {
                    // Java: int key = it.next();
                    // Java: ListData lst1 = this._listMap.get(key);
                    let lst1 = self._listMap.get(*key);
                    // Java: ListData lst2 = tables._listMap.get(key);
                    let lst2 = obj._listMap.get(*key);
                    // Java: if (!lst1.equals(lst2))
                    if lst1.is_none() || lst2.is_none() || !lst1.unwrap().equals(lst2.unwrap()) {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: int size = this._overrideList.size();
                let size = self._overrideList.len();
                // Java: if (size == tables._overrideList.size())
                if size == obj._overrideList.len() {
                    // Java: for(int x = 0; x < size; ++x)
                    for x in 0..size {
                        // Java: if (!((ListFormatOverride)this._overrideList.get(x)).equals(tables._overrideList.get(x)))
                        if !self._overrideList[x].equals(&obj._overrideList[x]) {
                            // Java: return false;
                            return false;
                        }
                    }
                    // Java: return true;
                    return true;
                }
            }
            // Java: return false;
            false
        }
    }
}

// Java: public class ListMap
impl ListMap {
    // Java: ListMap()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.parent = new WIntObjectHashMap();
        ListMap {
            parent: HashMap::new(),
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.parent.clear();
        self.parent.clear();
    }

    // Java: ListData get(int key)
    pub fn get(&self, key: i32) -> Option<&ListData> {
        // Java: return (ListData)this.parent.get(key);
        self.parent.get(&key)
    }

    // Java: ListData put(int key, ListData value)
    pub fn put(&mut self, key: i32, value: ListData) -> Option<ListData> {
        // Java: return (ListData)this.parent.put(key, value);
        self.parent.insert(key, value)
    }

    // Java: void putAll(ListMap listMap)
    pub fn putAll(&mut self, list_map: &ListMap) {
        // Java: IIntObjectIterator<ListData> it = listMap.parent.iterator();
        // Java: while(it.hasNext())
        for (key, value) in &list_map.parent {
            // Java: it.advance();
            // Java: this.put(it.key(), (ListData)it.value());
            self.parent.insert(*key, value.clone());
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.parent.size();
        self.parent.len() as i32
    }

    // Java: int[] sortedKeys()
    pub fn sortedKeys(&self) -> Vec<i32> {
        // Java: int[] keys = this.parent.keys();
        let mut keys: Vec<i32> = self.parent.keys().copied().collect();
        // Java: Arrays.sort(keys);
        keys.sort();
        // Java: return keys;
        keys
    }

    // Java: IIntSet keySet()
    pub fn keySet(&self) -> Vec<i32> {
        // Java: return this.parent.keySet();
        self.parent.keys().copied().collect()
    }

    // Java: Collection<ListData> values()
    pub fn values(&self) -> Vec<&ListData> {
        // Java: return this.parent.valueCollection();
        self.parent.values().collect()
    }
}
