#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Java: import cn.wps.graphics.data.reader.BufferReader
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.Map

use std::collections::HashMap;
use crate::cn::wps::graphics::data::reader::BufferReader::BufferReader;
use crate::cn::wps::graphics::data::tiff::TIFFField::{TIFFField, TIFFData};

// Java field: int[] sizeOfType
static SIZE_OF_TYPE: &[i32] = &[0, 1, 1, 2, 4, 8, 1, 1, 2, 4, 8, 4, 8];

// Java: public class TIFFDirectory
pub struct TIFFDirectory {
    // Java field: String TAG
    // Java field: boolean justReadSize
    justReadSize: bool,
    // Java field: boolean isLittleEndian
    pub(crate) isLittleEndian: bool,
    // Java field: int numEntries
    pub(crate) numEntries: i32,
    // Java field: TIFFField[] fields
    pub(crate) fields: Vec<Option<TIFFField>>,
    // Java field: Map<Integer, Integer> fieldIndex
    pub(crate) fieldIndex: HashMap<i32, i32>,
    // Java field: int IFDOffset
    pub(crate) IFDOffset: i32,
    // Java field: int nextIFDOffset
    pub(crate) nextIFDOffset: i32,
    // Java field: int[] sizeOfType
}

pub(crate) const TAG: &str = "";

// Java: public class TIFFDirectory
impl TIFFDirectory {
    fn new_default() -> Self {
        TIFFDirectory {
            justReadSize: false,
            isLittleEndian: false,
            numEntries: 0,
            fields: Vec::new(),
            fieldIndex: HashMap::new(),
            IFDOffset: 8,
            nextIFDOffset: 0,
        }
    }

    // Java: static boolean isValidEndianTag(int endian)
    pub fn isValidEndianTag(endian: i32) -> bool {
        // Java: return endian == 18761 || endian == 19789;
        endian == 18761 || endian == 19789
    }

    // Java: TIFFDirectory(BufferReader stream, int directory)
    pub fn new_dir(stream: &mut BufferReader, directory: i32) -> Self {
        // Java: this(stream, directory, false);
        TIFFDirectory::new_dir_just_size(stream, directory, false)
    }

    // Java: TIFFDirectory(BufferReader stream, int directory, boolean justReadSize)
    pub fn new_dir_just_size(stream: &mut BufferReader, directory: i32, justReadSize: bool) -> Self {
        // Java: super();
        // Java: this.fieldIndex = new HashMap();
        // Java: this.IFDOffset = 8;
        // Java: this.nextIFDOffset = 0;
        let mut this = TIFFDirectory::new_default();
        // Java: this.justReadSize = justReadSize;
        this.justReadSize = justReadSize;
        // Java: int global_save_offset = stream.getCurrentPos();
        let global_save_offset = stream.getCurrentPos();
        // Java: stream.setCurrentPos(-1);
        stream.setCurrentPos(-1);
        // Java: int endian = stream.readUnsignedShort();
        let endian = stream.readUnsignedShort();
        // Java: if (!isValidEndianTag(endian))
        if !TIFFDirectory::isValidEndianTag(endian) {
            // Java: throw new IllegalArgumentException("TIFFDirectory1");
            panic!("TIFFDirectory1");
        }
        // Java: else
        else {
            // Java: this.isLittleEndian = endian == 18761;
            this.isLittleEndian = endian == 18761;
            // Java: int magic = this.readUnsignedShort(stream);
            let magic = this.readUnsignedShort_inst(stream);
            // Java: if (magic != 42)
            if magic != 42 {
                // Java: throw new IllegalArgumentException("TIFFDirectory2");
                panic!("TIFFDirectory2");
            }
            // Java: else
            else {
                // Java: int ifd_offset = this.readUnsignedInt(stream);
                let mut ifd_offset = this.readUnsignedInt_inst(stream);
                // Java: for(int i = 0; i < directory; ++i)
                for i in 0..directory {
                    // Java: if (ifd_offset == 0)
                    if ifd_offset == 0 {
                        // Java: throw new IllegalArgumentException("TIFFDirectory3");
                        panic!("TIFFDirectory3");
                    }
                    // Java: stream.setCurrentPos(ifd_offset - 1);
                    stream.setCurrentPos(ifd_offset - 1);
                    // Java: int entries = this.readUnsignedShort(stream);
                    let entries = this.readUnsignedShort_inst(stream);
                    // Java: stream.skip(12 * entries);
                    stream.skip(12 * entries);
                    // Java: ifd_offset = this.readUnsignedInt(stream);
                    ifd_offset = this.readUnsignedInt_inst(stream);
                }
                // Java: stream.setCurrentPos(ifd_offset - 1);
                stream.setCurrentPos(ifd_offset - 1);
                // Java: this.initialize(stream);
                this.initialize(stream);
                // Java: stream.setCurrentPos(global_save_offset);
                stream.setCurrentPos(global_save_offset);
            }
        }
        this
    }

    // Java: TIFFDirectory(BufferReader stream, int ifd_offset, int directory)
    pub fn new_ifd(stream: &mut BufferReader, mut ifd_offset: i32, directory: i32) -> Self {
        // Java: super();
        // Java: this.fieldIndex = new HashMap();
        // Java: this.IFDOffset = 8;
        // Java: this.nextIFDOffset = 0;
        let mut this = TIFFDirectory::new_default();
        // Java: int global_save_offset = stream.getCurrentPos();
        let global_save_offset = stream.getCurrentPos();
        // Java: stream.setCurrentPos(-1);
        stream.setCurrentPos(-1);
        // Java: int endian = stream.readUnsignedShort();
        let endian = stream.readUnsignedShort();
        // Java: if (!isValidEndianTag(endian))
        if !TIFFDirectory::isValidEndianTag(endian) {
            // Java: throw new IllegalArgumentException("TIFFDirectory1");
            panic!("TIFFDirectory1");
        }
        // Java: else
        else {
            // Java: this.isLittleEndian = endian == 18761;
            this.isLittleEndian = endian == 18761;
            // Java: stream.setCurrentPos(ifd_offset - 1);
            stream.setCurrentPos(ifd_offset - 1);
            // Java: for(int dirNum = 0; dirNum < directory; ++dirNum)
            for dirNum in 0..directory {
                // Java: int numEntries = this.readUnsignedShort(stream);
                let numEntries = this.readUnsignedShort_inst(stream);
                // Java: stream.skip(12 * numEntries);
                stream.skip(12 * numEntries);
                // Java: ifd_offset = this.readUnsignedInt(stream);
                ifd_offset = this.readUnsignedInt_inst(stream);
                // Java: stream.setCurrentPos(ifd_offset - 1);
                stream.setCurrentPos(ifd_offset - 1);
            }
            // Java: this.initialize(stream);
            this.initialize(stream);
            // Java: stream.setCurrentPos(global_save_offset);
            stream.setCurrentPos(global_save_offset);
        }
        this
    }

    // Java: void initialize(BufferReader stream)
    pub fn initialize(&mut self, stream: &mut BufferReader) {
        // Java: this.IFDOffset = stream.getCurrentPos();
        self.IFDOffset = stream.getCurrentPos();
        // Java: this.numEntries = this.readUnsignedShort(stream);
        self.numEntries = self.readUnsignedShort_inst(stream);
        // Java: this.fields = new TIFFField[this.numEntries];
        self.fields = Vec::with_capacity(self.numEntries as usize);
        for _ in 0..self.numEntries { self.fields.push(None); }
        // Java: for(int i = 0; i < this.numEntries; ++i)
        let num = self.numEntries;
        for i in 0..num {
            // Java: int tag = this.readUnsignedShort(stream);
            let tag = self.readUnsignedShort_inst(stream);
            // Java: int type = this.readUnsignedShort(stream);
            let field_type = self.readUnsignedShort_inst(stream);
            // Java: int count = this.readUnsignedInt(stream);
            let mut count = self.readUnsignedInt_inst(stream);
            // Java: int value = 0;
            let mut value = 0;
            // Java: int nextTagOffset = stream.getCurrentPos() + 4;
            let nextTagOffset = stream.getCurrentPos() + 4;
            // Java: try
            let try_ok = {
                let type_idx = field_type as usize;
                // Java: if (count * sizeOfType[type] > 4)
                if type_idx < SIZE_OF_TYPE.len() && count * SIZE_OF_TYPE[type_idx] > 4 {
                    // Java: value = this.readUnsignedInt(stream);
                    value = self.readUnsignedInt_inst(stream);
                    // Java: stream.setCurrentPos(value - 1);
                    stream.setCurrentPos(value - 1);
                }
                true
            };
            // Java: catch (ArrayIndexOutOfBoundsException var19)
            if !try_ok {
                // Java: System.err.println(tag + " TIFFDirectory4");
                eprintln!("{} TIFFDirectory4", tag);
                // Java: stream.setCurrentPos(nextTagOffset - 1);
                stream.setCurrentPos(nextTagOffset - 1);
                // Java: continue;
                continue;
            }
            // Java: this.fieldIndex.put(new Integer(tag), new Integer(i));
            self.fieldIndex.insert(tag, i);
            // Java: Object obj = null;
            let mut obj: Option<TIFFData> = None;
            // Java: switch (type)
            match field_type {
                // Java: case 1:
                // Java: case 2:
                // Java: case 6:
                // Java: case 7:
                1 | 2 | 6 | 7 => {
                    // Java: byte[] bvalues = new byte[count];
                    let mut bvalues = vec![0u8; count as usize];
                    // Java: stream.read(bvalues);
                    stream.read_n(&mut bvalues, count);
                    // Java: if (type != 2)
                    if field_type != 2 {
                        // Java: obj = bvalues;
                        obj = Some(TIFFData::Bytes(bvalues));
                        // Java: break;
                    } else {
                        // Java: int index = 0;
                        let mut index = 0usize;
                        // Java: int prevIndex = 0;
                        let mut prevIndex = 0usize;
                        // Java: List<String> v;
                        // Java: for(v = new ArrayList(); index < count; prevIndex = index)
                        let mut v: Vec<String> = Vec::new();
                        while index < count as usize {
                            // Java: while(index < count && bvalues[index++] != 0)
                            while index < count as usize {
                                let b = bvalues[index];
                                index += 1;
                                if b == 0 { break; }
                            }
                            // Java: v.add(new String(bvalues, prevIndex, index - prevIndex));
                            let s = String::from_utf8_lossy(&bvalues[prevIndex..index]).to_string();
                            v.push(s);
                            prevIndex = index;
                        }
                        // Java: count = v.size();
                        count = v.len() as i32;
                        // Java: String[] strings = new String[count];
                        // Java: v.toArray(strings);
                        // Java: obj = strings;
                        obj = Some(TIFFData::Strings(v));
                        // Java: break;
                    }
                }
                // Java: case 3:
                3 => {
                    // Java: char[] cvalues = new char[count];
                    let mut cvalues = vec![0u16; count as usize];
                    // Java: for(int j = 0; j < count; ++j)
                    for j in 0..count as usize {
                        // Java: cvalues[j] = (char)this.readUnsignedShort(stream);
                        cvalues[j] = self.readUnsignedShort_inst(stream) as u16;
                    }
                    // Java: obj = cvalues;
                    obj = Some(TIFFData::Chars(cvalues));
                    // Java: break;
                }
                // Java: case 4:
                4 => {
                    // Java: long[] lvalues = new long[count];
                    let mut lvalues = vec![0i64; count as usize];
                    // Java: for(int j = 0; j < count; ++j)
                    for j in 0..count as usize {
                        // Java: lvalues[j] = (long)this.readUnsignedInt(stream);
                        lvalues[j] = self.readUnsignedInt_inst(stream) as i64;
                    }
                    // Java: obj = lvalues;
                    obj = Some(TIFFData::Longs(lvalues));
                    // Java: break;
                }
                // Java: case 5:
                5 => {
                    // Java: int[][] llvalues = new int[count][2];
                    let mut llvalues = vec![[0i32; 2]; count as usize];
                    // Java: for(int j = 0; j < count; ++j)
                    for j in 0..count as usize {
                        // Java: llvalues[j][0] = this.readUnsignedInt(stream);
                        llvalues[j][0] = self.readUnsignedInt_inst(stream);
                        // Java: llvalues[j][1] = this.readUnsignedInt(stream);
                        llvalues[j][1] = self.readUnsignedInt_inst(stream);
                    }
                    // Java: obj = llvalues;
                    obj = Some(TIFFData::Rationals(llvalues.iter().map(|r| (r[0] as i64, r[1] as i64)).collect()));
                    // Java: break;
                }
                // Java: case 8:
                8 => {
                    // Java: short[] svalues = new short[count];
                    let mut svalues = vec![0i16; count as usize];
                    // Java: for(int j = 0; j < count; ++j)
                    for j in 0..count as usize {
                        // Java: svalues[j] = this.readShort(stream);
                        svalues[j] = self.readShort(stream);
                    }
                    // Java: obj = svalues;
                    obj = Some(TIFFData::Shorts(svalues));
                    // Java: break;
                }
                // Java: case 9:
                9 => {
                    // Java: int[] ivalues = new int[count];
                    let mut ivalues = vec![0i32; count as usize];
                    // Java: for(int j = 0; j < count; ++j)
                    for j in 0..count as usize {
                        // Java: ivalues[j] = this.readInt(stream);
                        ivalues[j] = self.readInt(stream);
                    }
                    // Java: obj = ivalues;
                    obj = Some(TIFFData::Ints(ivalues));
                    // Java: break;
                }
                // Java: case 10:
                10 => {
                    // Java: int[][] iivalues = new int[count][2];
                    let mut iivalues = vec![[0i32; 2]; count as usize];
                    // Java: for(int j = 0; j < count; ++j)
                    for j in 0..count as usize {
                        // Java: iivalues[j][0] = this.readInt(stream);
                        iivalues[j][0] = self.readInt(stream);
                        // Java: iivalues[j][1] = this.readInt(stream);
                        iivalues[j][1] = self.readInt(stream);
                    }
                    // Java: obj = iivalues;
                    obj = Some(TIFFData::SRationals(iivalues.iter().map(|r| (r[0], r[1])).collect()));
                    // Java: break;
                }
                // Java: case 11:
                11 => {
                    // Java: float[] fvalues = new float[count];
                    let mut fvalues = vec![0.0f32; count as usize];
                    // Java: for(int j = 0; j < count; ++j)
                    for j in 0..count as usize {
                        // Java: fvalues[j] = this.readFloat(stream);
                        fvalues[j] = self.readFloat(stream);
                    }
                    // Java: obj = fvalues;
                    obj = Some(TIFFData::Floats(fvalues));
                    // Java: break;
                }
                // Java: case 12:
                12 => {
                    // Java: double[] dvalues = new double[count];
                    let mut dvalues = vec![0.0f64; count as usize];
                    // Java: for(int j = 0; j < count; ++j)
                    for j in 0..count as usize {
                        // Java: dvalues[j] = this.readDouble(stream);
                        dvalues[j] = self.readDouble(stream);
                    }
                    // Java: obj = dvalues;
                    obj = Some(TIFFData::Doubles(dvalues));
                }
                _ => {}
            }
            // Java: this.fields[i] = new TIFFField(tag, type, count, obj);
            if let Some(data) = obj {
                self.fields[i as usize] = Some(TIFFField::new(tag, field_type, count, data));
            }
            // Java: stream.setCurrentPos(nextTagOffset);
            stream.setCurrentPos(nextTagOffset);
            // Java: if (this.justReadSize && this.getField(256) != null && this.getField(257) != null && this.getField(282) != null && this.getField(283) != null)
            if self.justReadSize && self.getField(256).is_some() && self.getField(257).is_some() && self.getField(282).is_some() && self.getField(283).is_some() {
                // Java: return;
                return;
            }
        }
        // Java: this.nextIFDOffset = this.readUnsignedInt(stream);
        self.nextIFDOffset = self.readUnsignedInt_inst(stream);
    }

    // Java: int getNumEntries()
    pub fn getNumEntries(&self) -> i32 {
        // Java: return this.numEntries;
        self.numEntries
    }

    // Java: TIFFField getField(int tag)
    pub fn getField(&self, tag: i32) -> Option<&TIFFField> {
        // Java: Integer i = (Integer)this.fieldIndex.get(new Integer(tag));
        let i = self.fieldIndex.get(&tag);
        // Java: return i == null ? null : this.fields[i];
        match i {
            None => None,
            Some(&idx) => self.fields[idx as usize].as_ref(),
        }
    }

    // Java: boolean isTagPresent(int tag)
    pub fn isTagPresent(&self, tag: i32) -> bool {
        // Java: return this.fieldIndex.containsKey(new Integer(tag));
        self.fieldIndex.contains_key(&tag)
    }

    // Java: int[] getTags()
    pub fn getTags(&self) -> Vec<i32> {
        // Java: int[] tags = new int[this.fieldIndex.size()];
        // Java: Iterator<Integer> iter = this.fieldIndex.keySet().iterator();
        // Java: for(int i = 0; iter.hasNext(); tags[i++] = (Integer)iter.next())
        // Java: return tags;
        self.fieldIndex.keys().copied().collect()
    }

    // Java: TIFFField[] getFields()
    pub fn getFields(&self) -> &[Option<TIFFField>] {
        // Java: return this.fields;
        &self.fields
    }

    // Java: byte getFieldAsByte(int tag, int index)
    pub fn getFieldAsByte_idx(&self, tag: i32, index: i32) -> i8 {
        // Java: Integer i = (Integer)this.fieldIndex.get(new Integer(tag));
        let i = self.fieldIndex[&tag];
        // Java: byte[] b = this.fields[i].getAsBytes();
        let b = self.fields[i as usize].as_ref().unwrap().getAsBytes();
        // Java: return b[index];
        b[index as usize] as i8
    }

    // Java: byte getFieldAsByte(int tag)
    pub fn getFieldAsByte(&self, tag: i32) -> i8 {
        // Java: return this.getFieldAsByte(tag, 0);
        self.getFieldAsByte_idx(tag, 0)
    }

    // Java: long getFieldAsLong(int tag, int index)
    pub fn getFieldAsLong_idx(&self, tag: i32, index: i32) -> i64 {
        // Java: Integer i = (Integer)this.fieldIndex.get(new Integer(tag));
        let i = self.fieldIndex[&tag];
        // Java: return this.fields[i].getAsLong(index);
        self.fields[i as usize].as_ref().unwrap().getAsLong(index)
    }

    // Java: long getFieldAsLong(int tag)
    pub fn getFieldAsLong(&self, tag: i32) -> i64 {
        // Java: return this.getFieldAsLong(tag, 0);
        self.getFieldAsLong_idx(tag, 0)
    }

    // Java: float getFieldAsFloat(int tag, int index)
    pub fn getFieldAsFloat_idx(&self, tag: i32, index: i32) -> f32 {
        // Java: Integer i = (Integer)this.fieldIndex.get(new Integer(tag));
        let i = self.fieldIndex[&tag];
        // Java: return this.fields[i].getAsFloat(index);
        self.fields[i as usize].as_ref().unwrap().getAsFloat(index)
    }

    // Java: float getFieldAsFloat(int tag)
    pub fn getFieldAsFloat(&self, tag: i32) -> f32 {
        // Java: return this.getFieldAsFloat(tag, 0);
        self.getFieldAsFloat_idx(tag, 0)
    }

    // Java: double getFieldAsDouble(int tag, int index)
    pub fn getFieldAsDouble_idx(&self, tag: i32, index: i32) -> f64 {
        // Java: Integer i = (Integer)this.fieldIndex.get(new Integer(tag));
        let i = self.fieldIndex[&tag];
        // Java: return this.fields[i].getAsDouble(index);
        self.fields[i as usize].as_ref().unwrap().getAsDouble(index)
    }

    // Java: double getFieldAsDouble(int tag)
    pub fn getFieldAsDouble(&self, tag: i32) -> f64 {
        // Java: return this.getFieldAsDouble(tag, 0);
        self.getFieldAsDouble_idx(tag, 0)
    }

    // Java: short readShort(BufferReader stream)
    fn readShort(&self, stream: &mut BufferReader) -> i16 {
        // Java: return this.isLittleEndian ? stream.readShort() : stream.readShortBE();
        if self.isLittleEndian { stream.readShort_one() } else { stream.readShortBE() }
    }

    // Java: int readUnsignedShort(BufferReader stream)
    fn readUnsignedShort_inst(&self, stream: &mut BufferReader) -> i32 {
        // Java: return this.isLittleEndian ? stream.readUnsignedShort() : stream.readUnsignedShortBE();
        if self.isLittleEndian { stream.readUnsignedShort() } else { stream.readUnsignedShortBE() }
    }

    // Java: int readInt(BufferReader stream)
    fn readInt(&self, stream: &mut BufferReader) -> i32 {
        // Java: return this.isLittleEndian ? stream.readInt() : stream.readIntBE();
        if self.isLittleEndian { stream.readInt_one() } else { stream.readIntBE() }
    }

    // Java: int readUnsignedInt(BufferReader stream)
    fn readUnsignedInt_inst(&self, stream: &mut BufferReader) -> i32 {
        // Java: return this.isLittleEndian ? stream.readUnsignedInt() : stream.readUnsignedIntBE();
        if self.isLittleEndian { stream.readUnsignedInt_one() } else { stream.readUnsignedIntBE() }
    }

    // Java: long readLong(BufferReader stream)
    fn readLong(&self, stream: &mut BufferReader) -> i64 {
        // Java: return this.isLittleEndian ? stream.readLong() : stream.readLongBE();
        if self.isLittleEndian { stream.readLong() } else { stream.readLongBE() }
    }

    // Java: float readFloat(BufferReader stream)
    fn readFloat(&self, stream: &mut BufferReader) -> f32 {
        // Java: return this.isLittleEndian ? stream.readFloat() : stream.readFloatBE();
        if self.isLittleEndian { stream.readFloat() } else { stream.readFloatBE() }
    }

    // Java: double readDouble(BufferReader stream)
    fn readDouble(&self, stream: &mut BufferReader) -> f64 {
        // Java: return this.isLittleEndian ? stream.readDouble() : stream.readDoubleBE();
        if self.isLittleEndian { stream.readDouble() } else { stream.readDoubleBE() }
    }

    // Java: static int readUnsignedShort(BufferReader stream, boolean isBigEndian)
    pub fn readUnsignedShort_static(stream: &mut BufferReader, isBigEndian: bool) -> i32 {
        // Java: return isBigEndian ? stream.readUnsignedShort() : stream.readUnsignedShortBE();
        if isBigEndian { stream.readUnsignedShort() } else { stream.readUnsignedShortBE() }
    }

    // Java: static int readUnsignedInt(BufferReader stream, boolean isBigEndian)
    pub fn readUnsignedInt_static(stream: &mut BufferReader, isBigEndian: bool) -> i32 {
        // Java: return isBigEndian ? stream.readUnsignedInt() : stream.readUnsignedIntBE();
        if isBigEndian { stream.readUnsignedInt_one() } else { stream.readUnsignedIntBE() }
    }

    // Java: static int getNumDirectories(BufferReader stream)
    pub fn getNumDirectories(stream: &mut BufferReader) -> i32 {
        // Java: int pointer = stream.getCurrentPos();
        let pointer = stream.getCurrentPos();
        // Java: stream.setCurrentPos(-1);
        stream.setCurrentPos(-1);
        // Java: int endian = stream.readUnsignedShort();
        let endian = stream.readUnsignedShort();
        // Java: if (!isValidEndianTag(endian))
        if !TIFFDirectory::isValidEndianTag(endian) {
            // Java: throw new IllegalArgumentException("TIFFDirectory1");
            panic!("TIFFDirectory1");
        }
        // Java: else
        else {
            // Java: boolean isLittleEndian = endian == 18761;
            let isLittleEndian = endian == 18761;
            // Java: int magic = readUnsignedShort(stream, isLittleEndian);
            let magic = TIFFDirectory::readUnsignedShort_static(stream, isLittleEndian);
            // Java: if (magic != 42)
            if magic != 42 {
                // Java: throw new IllegalArgumentException("TIFFDirectory2");
                panic!("TIFFDirectory2");
            }
            // Java: else
            else {
                // Java: int offset = readUnsignedInt(stream, isLittleEndian);
                let mut offset = TIFFDirectory::readUnsignedInt_static(stream, isLittleEndian);
                // Java: int numDirectories = 0;
                let mut numDirectories = 0;
                // Java: try
                let _result: Result<(), ()> = (|| {
                    // Java: while((long)offset != 0L)
                    while offset as i64 != 0 {
                        // Java: ++numDirectories;
                        numDirectories += 1;
                        // Java: stream.setCurrentPos(offset - 1);
                        stream.setCurrentPos(offset - 1);
                        // Java: int entries = readUnsignedShort(stream, isLittleEndian);
                        let entries = TIFFDirectory::readUnsignedShort_static(stream, isLittleEndian);
                        // Java: stream.skip(12 * entries);
                        stream.skip(12 * entries);
                        // Java: offset = readUnsignedInt(stream, isLittleEndian);
                        offset = TIFFDirectory::readUnsignedInt_static(stream, isLittleEndian);
                    }
                    Ok(())
                })();
                // Java: catch (ArrayIndexOutOfBoundsException e)
                if _result.is_err() {
                    // Java: e.printStackTrace();
                    eprintln!("ArrayIndexOutOfBoundsException in getNumDirectories");
                }
                // Java: stream.setCurrentPos(pointer);
                stream.setCurrentPos(pointer);
                // Java: return numDirectories;
                numDirectories
            }
        }
    }

    // Java: boolean isBigEndian()
    pub fn isBigEndian(&self) -> bool {
        // Java: return this.isLittleEndian;
        self.isLittleEndian
    }

    // Java: int getIFDOffset()
    pub fn getIFDOffset(&self) -> i32 {
        // Java: return this.IFDOffset;
        self.IFDOffset
    }

    // Java: int getNextIFDOffset()
    pub fn getNextIFDOffset(&self) -> i32 {
        // Java: return this.nextIFDOffset;
        self.nextIFDOffset
    }
}
