#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_assignments, unused_mut)]

// Java field: int CLEAR_CODE
pub(crate) const CLEAR_CODE: i32 = 256;
// Java field: int EOI_CODE
pub(crate) const EOI_CODE: i32 = 257;
// Java field: int[] indexTable
// Java field: int[] lengthTable

// Java: public class LZWDecoder
pub struct LZWDecoder {
    // Java field: byte[] data
    data: Vec<u8>,
    // Java field: byte[] uncompData
    pub(crate) uncompData: Vec<u8>,
    // Java field: int tableIndex
    tableIndex: i32,
    // Java field: int bitsToGet
    bitsToGet: i32,
    // Java field: int bytePointer
    bytePointer: i32,
    // Java field: int bitPointer
    bitPointer: i32,
    // Java field: int dstIndex
    dstIndex: i32,
    // Java field: int w
    w: i32,
    // Java field: int h
    h: i32,
    // Java field: int predictor
    predictor: i32,
    // Java field: int samplesPerPixel
    samplesPerPixel: i32,
    // Java field: int nextData
    nextData: i32,
    // Java field: int nextBits
    nextBits: i32,
    // Java field: int[] andTable
    andTable: Vec<i32>,
    indexTable: Vec<i32>,
    lengthTable: Vec<i32>,
}

// Java: public class LZWDecoder
impl LZWDecoder {
    // Java: LZWDecoder(int w, int predictor, int samplesPerPixel)
    pub fn new(w: i32, predictor: i32, samplesPerPixel: i32) -> Self {
        // Java: super();
        // Java: this.w = w;
        // Java: this.predictor = predictor;
        // Java: this.samplesPerPixel = samplesPerPixel;
        LZWDecoder {
            data: Vec::new(),
            uncompData: Vec::new(),
            tableIndex: 0,
            bitsToGet: 9,
            bytePointer: 0,
            bitPointer: 0,
            dstIndex: 0,
            w,
            h: 0,
            predictor,
            samplesPerPixel,
            nextData: 0,
            nextBits: 0,
            andTable: vec![511, 1023, 2047, 4095],
            indexTable: Vec::new(),
            lengthTable: Vec::new(),
        }
    }

    // Java: void initialize(byte[] data, int readPos, byte[] uncompressData, int h)
    fn initialize(&mut self, data: Vec<u8>, readPos: i32, uncompressData: Vec<u8>, h: i32) {
        // Java: this.data = data;
        self.data = data;
        // Java: this.bytePointer = readPos;
        self.bytePointer = readPos;
        // Java: this.uncompData = uncompressData;
        self.uncompData = uncompressData;
        // Java: this.h = h;
        self.h = h;
        // Java: this.bitPointer = 0;
        self.bitPointer = 0;
        // Java: this.dstIndex = 0;
        self.dstIndex = 0;
        // Java: this.nextData = 0;
        self.nextData = 0;
        // Java: this.nextBits = 0;
        self.nextBits = 0;
    }

    // Java: byte[] decode(byte[] data, int readPos, byte[] uncompressData, int h)
    pub fn decode(&mut self, data: Vec<u8>, readPos: i32, mut uncompressData: Vec<u8>, h: i32) -> Vec<u8> {
        // Java: if (data[0] == 0 && data[1] == 1)
        if data[0] == 0 && data[1] == 1 {
            // Java: throw new UnsupportedOperationException("TIFFLZWDecoder0");
            panic!("TIFFLZWDecoder0");
        }
        // Java: else
        else {
            // Java: this.initialize(data, readPos, uncompressData, h);
            self.initialize(data, readPos, uncompressData, h);
            // Java: this.initializeStringTable();
            self.initializeStringTable();
            // Java: int code = -1;
            let mut code: i32 = -1;
            // Java: int oldCode = 0;
            let mut oldCode: i32 = 0;
            // Java: int oldIndex = 0;
            let mut oldIndex: i32 = 0;
            // Java: try
            let _catch_result: Result<(), ()> = (|| -> Result<(), ()> {
                // Java: while((code = this.getNextCode()) != 257)
                loop {
                    code = self.getNextCode();
                    if code == 257 { break; }
                    // Java: oldIndex = this.dstIndex;
                    oldIndex = self.dstIndex;
                    // Java: if (code == 256)
                    if code == 256 {
                        // Java: this.clearStringTable();
                        self.clearStringTable();
                        // Java: code = this.getNextCode();
                        code = self.getNextCode();
                        // Java: if (code == 257)
                        if code == 257 {
                            // Java: break;
                            break;
                        }
                        // Java: this.uncompData[this.dstIndex++] = (byte)code;
                        self.uncompData[self.dstIndex as usize] = code as u8;
                        self.dstIndex += 1;
                        // Java: oldCode = code;
                        oldCode = code;
                    }
                    // Java: else
                    else {
                        // Java: if (code < this.tableIndex)
                        if code < self.tableIndex {
                            // Java: if (code < 256)
                            if code < 256 {
                                // Java: this.uncompData[this.dstIndex++] = (byte)code;
                                self.uncompData[self.dstIndex as usize] = code as u8;
                                self.dstIndex += 1;
                            }
                            // Java: else
                            else {
                                // Java: System.arraycopy(this.uncompData, indexTable[code], this.uncompData, this.dstIndex, lengthTable[code]);
                                let src = self.indexTable[code as usize] as usize;
                                let len = self.lengthTable[code as usize] as usize;
                                let dst = self.dstIndex as usize;
                                self.uncompData.copy_within(src..src + len, dst);
                                // Java: this.dstIndex += lengthTable[code];
                                self.dstIndex += self.lengthTable[code as usize];
                            }
                            // Java: indexTable[this.tableIndex] = oldIndex - lengthTable[oldCode];
                            self.indexTable[self.tableIndex as usize] = oldIndex - self.lengthTable[oldCode as usize];
                            // Java: lengthTable[this.tableIndex] = lengthTable[oldCode] + 1;
                            self.lengthTable[self.tableIndex as usize] = self.lengthTable[oldCode as usize] + 1;
                            // Java: oldCode = code;
                            oldCode = code;
                        }
                        // Java: else
                        else {
                            // Java: if (oldCode < 256)
                            if oldCode < 256 {
                                // Java: this.uncompData[this.dstIndex++] = (byte)oldCode;
                                self.uncompData[self.dstIndex as usize] = oldCode as u8;
                                self.dstIndex += 1;
                            }
                            // Java: else
                            else {
                                // Java: System.arraycopy(this.uncompData, indexTable[oldCode], this.uncompData, this.dstIndex, lengthTable[oldCode]);
                                let src = self.indexTable[oldCode as usize] as usize;
                                let len = self.lengthTable[oldCode as usize] as usize;
                                let dst = self.dstIndex as usize;
                                self.uncompData.copy_within(src..src + len, dst);
                                // Java: this.dstIndex += lengthTable[oldCode];
                                self.dstIndex += self.lengthTable[oldCode as usize];
                            }
                            // Java: this.uncompData[this.dstIndex++] = this.uncompData[oldIndex];
                            let val = self.uncompData[oldIndex as usize];
                            self.uncompData[self.dstIndex as usize] = val;
                            self.dstIndex += 1;
                            // Java: indexTable[this.tableIndex] = oldIndex;
                            self.indexTable[self.tableIndex as usize] = oldIndex;
                            // Java: lengthTable[this.tableIndex] = lengthTable[oldCode] + 1;
                            self.lengthTable[self.tableIndex as usize] = self.lengthTable[oldCode as usize] + 1;
                            // Java: oldCode = code;
                            oldCode = code;
                        }
                        // Java: ++this.tableIndex;
                        self.tableIndex += 1;
                        // Java: this.changeBitsToGet();
                        self.changeBitsToGet();
                    }
                }
                Ok(())
            })();
            // Java: catch (Exception var9)
            // silently ignore exceptions (matches Java behavior)
            // Java: if (this.predictor == 2)
            if self.predictor == 2 {
                // Java: this.predictor();
                self.predictor();
            }
            // Java: return this.uncompData;
            return std::mem::take(&mut self.uncompData);
        }
    }

    // Java: void predictor()
    fn predictor(&mut self) {
        // Java: for(int j = 0; j < this.h; ++j)
        for j in 0..self.h {
            // Java: int count = this.samplesPerPixel * (j * this.w + 1);
            let mut count = (self.samplesPerPixel * (j * self.w + 1)) as usize;
            // Java: for(int i = this.samplesPerPixel; i < this.w * this.samplesPerPixel; ++i)
            for _i in self.samplesPerPixel..(self.w * self.samplesPerPixel) {
                // Java: byte[] var10000 = this.uncompData;
                // Java: var10000[count] += this.uncompData[count - this.samplesPerPixel];
                let prev = self.uncompData[count - self.samplesPerPixel as usize];
                self.uncompData[count] = self.uncompData[count].wrapping_add(prev);
                // Java: ++count;
                count += 1;
            }
        }
    }

    // Java: void initializeStringTable()
    pub fn initializeStringTable(&mut self) {
        // Java: if (indexTable == null || lengthTable == null)
        if self.indexTable.is_empty() || self.lengthTable.is_empty() {
            // Java: indexTable = new int[4096];
            self.indexTable = vec![0; 4096];
            // Java: lengthTable = new int[4096];
            self.lengthTable = vec![0; 4096];
            // Java: for(int i = 0; i < 256; ++i)
            for i in 0..256 {
                // Java: lengthTable[i] = 1;
                self.lengthTable[i] = 1;
            }
        }
        // Java: this.tableIndex = 258;
        self.tableIndex = 258;
        // Java: this.bitsToGet = 9;
        self.bitsToGet = 9;
    }

    // Java: void clearStringTable()
    pub fn clearStringTable(&mut self) {
        // Java: this.tableIndex = 258;
        self.tableIndex = 258;
        // Java: this.bitsToGet = 9;
        self.bitsToGet = 9;
    }

    // Java: void writeString(int code, boolean isKnown)
    pub fn writeString(&mut self, code: i32, isKnown: bool) {
        // Java: int srcPos = 0;
        let mut srcPos: i32 = 0;
        // Java: if (code < 256)
        if code < 256 {
            // Java: srcPos = this.dstIndex;
            srcPos = self.dstIndex;
            // Java: this.uncompData[this.dstIndex++] = (byte)code;
            self.uncompData[self.dstIndex as usize] = code as u8;
            self.dstIndex += 1;
        }
        // Java: else
        else {
            // Java: srcPos = code;
            srcPos = code;
            // Java: int length = lengthTable[code];
            let length = self.lengthTable[code as usize];
            // Java: System.arraycopy(this.uncompData, code, this.uncompData, this.dstIndex, length);
            let src = code as usize;
            let len = length as usize;
            let dst = self.dstIndex as usize;
            self.uncompData.copy_within(src..src + len, dst);
            // Java: this.dstIndex += length;
            self.dstIndex += length;
        }
        // Java: if (!isKnown)
        if !isKnown {
            // Java: this.uncompData[this.dstIndex++] = this.uncompData[srcPos];
            let val = self.uncompData[srcPos as usize];
            self.uncompData[self.dstIndex as usize] = val;
            self.dstIndex += 1;
        }
    }

    // Java: void addStringToTable(int idx, int len)
    pub fn addStringToTable(&mut self, idx: i32, len: i32) {
        // Java: indexTable[this.tableIndex] = idx;
        self.indexTable[self.tableIndex as usize] = idx;
        // Java: lengthTable[this.tableIndex] = len;
        self.lengthTable[self.tableIndex as usize] = len;
        // Java: ++this.tableIndex;
        self.tableIndex += 1;
        // Java: this.changeBitsToGet();
        self.changeBitsToGet();
    }

    // Java: void changeBitsToGet()
    fn changeBitsToGet(&mut self) {
        // Java: if (this.tableIndex == 511)
        if self.tableIndex == 511 {
            // Java: this.bitsToGet = 10;
            self.bitsToGet = 10;
        }
        // Java: else if (this.tableIndex == 1023)
        else if self.tableIndex == 1023 {
            // Java: this.bitsToGet = 11;
            self.bitsToGet = 11;
        }
        // Java: else if (this.tableIndex == 2047)
        else if self.tableIndex == 2047 {
            // Java: this.bitsToGet = 12;
            self.bitsToGet = 12;
        }
    }

    // Java: int getNextCode()
    pub fn getNextCode(&mut self) -> i32 {
        // Java: this.nextData = this.nextData << 8 | this.data[this.bytePointer++] & 255;
        if self.bytePointer as usize >= self.data.len() { return EOI_CODE; }
        self.nextData = (self.nextData << 8) | (self.data[self.bytePointer as usize] as i32 & 255);
        self.bytePointer += 1;
        // Java: this.nextBits += 8;
        self.nextBits += 8;
        // Java: if (this.nextBits < this.bitsToGet)
        if self.nextBits < self.bitsToGet {
            // Java: this.nextData = this.nextData << 8 | this.data[this.bytePointer++] & 255;
            if self.bytePointer as usize >= self.data.len() { return EOI_CODE; }
            self.nextData = (self.nextData << 8) | (self.data[self.bytePointer as usize] as i32 & 255);
            self.bytePointer += 1;
            // Java: this.nextBits += 8;
            self.nextBits += 8;
        }
        // Java: int code = this.nextData >> this.nextBits - this.bitsToGet & this.andTable[this.bitsToGet - 9];
        let code = (self.nextData >> (self.nextBits - self.bitsToGet)) & self.andTable[(self.bitsToGet - 9) as usize];
        // Java: this.nextBits -= this.bitsToGet;
        self.nextBits -= self.bitsToGet;
        // Java: return code;
        code
    }
}
