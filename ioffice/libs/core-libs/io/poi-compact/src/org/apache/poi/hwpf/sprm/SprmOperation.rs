// Java: import org.apache.poi.util.LittleEndian

fn le_get_short(data: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes([data[offset], data[offset + 1]])
}

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

fn le_get_ushort(data: &[u8], offset: usize) -> i32 {
    u16::from_le_bytes([data[offset], data[offset + 1]]) as i32
}

// Java field: int BYTE_SIZE
pub const BYTE_SIZE: i32 = 3;
// Java field: int SHORT_SIZE
pub const SHORT_SIZE: i32 = 4;
// Java field: int INT_SIZE
pub const INT_SIZE: i32 = 6;
// Java field: short LONG_SPRM_TABLE
const LONG_SPRM_TABLE: i16 = -10744_i16;
// Java field: int PAP_TYPE
pub const PAP_TYPE: i32 = 1;
// Java field: int CHP_TYPE
pub const CHP_TYPE: i32 = 2;
// Java field: int PICTURE_TYPE
pub const PICTURE_TYPE: i32 = 3;
// Java field: int SECTION_TYPE
pub const SECTION_TYPE: i32 = 4;
// Java field: int TAP_TYPE
pub const TAP_TYPE: i32 = 5;
// Java field: int SIZE_CODE_0
pub const SIZE_CODE_0: i32 = 0;
// Java field: int SIZE_CODE_1
pub const SIZE_CODE_1: i32 = 1;
// Java field: int SIZE_CODE_2
pub const SIZE_CODE_2: i32 = 2;
// Java field: int SIZE_CODE_3
pub const SIZE_CODE_3: i32 = 3;
// Java field: int SIZE_CODE_4
pub const SIZE_CODE_4: i32 = 4;
// Java field: int SIZE_CODE_5
pub const SIZE_CODE_5: i32 = 5;
// Java field: int SIZE_CODE_6
pub const SIZE_CODE_6: i32 = 6;
// Java field: int SIZE_CODE_7
pub const SIZE_CODE_7: i32 = 7;

// Java: public class SprmOperation
pub struct SprmOperation {
    // Java field: short _sprmStart
    _sprmStart: i16,
    // Java field: short LONG_SPRM_TABLE
    // Java field: int PAP_TYPE
    // Java field: int CHP_TYPE
    // Java field: int PICTURE_TYPE
    // Java field: int SECTION_TYPE
    // Java field: int TAP_TYPE
    // Java field: int SIZE_CODE_0
    // Java field: int SIZE_CODE_1
    // Java field: int SIZE_CODE_2
    // Java field: int SIZE_CODE_3
    // Java field: int SIZE_CODE_4
    // Java field: int SIZE_CODE_5
    // Java field: int SIZE_CODE_6
    // Java field: int SIZE_CODE_7
    // Java field: int _type
    _type: i32,
    // Java field: int _operation
    _operation: i32,
    // Java field: int _gOffset
    _gOffset: i32,
    // Java field: byte[] _grpprl
    _grpprl: Vec<u8>,
    // Java field: int _sizeCode
    _sizeCode: i32,
    // Java field: int _size
    _size: i32,
    // Java field: int _sizeOperand
    _sizeOperand: i32,
}

impl SprmOperation {
    // Java: SprmOperation()
    pub fn new() -> Self {
        // Java: super();
        SprmOperation {
            _sprmStart: 0,
            _type: 0,
            _operation: 0,
            _gOffset: 0,
            _grpprl: Vec::new(),
            _sizeCode: 0,
            _size: 0,
            _sizeOperand: 0,
        }
    }

    // Java: void init(byte[] grpprl, int offset)
    pub fn init(&mut self, grpprl: &[u8], offset: i32) {
        // Java: this._grpprl = grpprl;
        self._grpprl = grpprl.to_vec();
        // Java: this._sprmStart = LittleEndian.getShort(grpprl, offset);
        self._sprmStart = le_get_short(grpprl, offset as usize);
        // Java: this._gOffset = offset + 2;
        self._gOffset = offset + 2;
        // Java: if (this._sprmStart != 0)
        if self._sprmStart != 0 {
            // Java: this._operation = this._sprmStart & 511;
            self._operation = (self._sprmStart as i32) & 511;
            // Java: this._type = (this._sprmStart & 7168) >> 10;
            self._type = ((self._sprmStart as i32) & 7168) >> 10;
            // Java: this._sizeCode = (this._sprmStart & '\ue000') >> 13;
            self._sizeCode = ((self._sprmStart as i32) & 0xE000u16 as i32) >> 13;
            // Java: this._size = this.initSize(this._sprmStart);
            self._size = self.initSize(self._sprmStart);
            // Java: this._sizeOperand = this.initSizeOperand(this._sprmStart);
            self._sizeOperand = self.initSizeOperand(self._sprmStart);
        }
        // Java: else
        else {
            // Java: this._operation = 0;
            self._operation = 0;
            // Java: this._type = 0;
            self._type = 0;
            // Java: this._sizeCode = 0;
            self._sizeCode = 0;
            // Java: this._size = 2;
            self._size = 2;
            // Java: this._sizeOperand = 0;
            self._sizeOperand = 0;
        }
    }

    // Java: short getSprmStart()
    pub fn getSprmStart(&self) -> i16 {
        // Java: return this._sprmStart;
        self._sprmStart
    }

    // Java: static int getOperationFromOpcode(short opcode)
    pub fn getOperationFromOpcode(opcode: i16) -> i32 {
        // Java: return opcode & 511;
        (opcode as i32) & 511
    }

    // Java: static int getTypeFromOpcode(short opcode)
    pub fn getTypeFromOpcode(opcode: i16) -> i32 {
        // Java: return (opcode & 7168) >> 10;
        ((opcode as i32) & 7168) >> 10
    }

    // Java: int getType()
    pub fn getType(&self) -> i32 {
        // Java: return this._type;
        self._type
    }

    // Java: int getOperation()
    pub fn getOperation(&self) -> i32 {
        // Java: return this._operation;
        self._operation
    }

    // Java: int getGrpprlOffset()
    pub fn getGrpprlOffset(&self) -> i32 {
        // Java: return this._gOffset;
        self._gOffset
    }

    // Java: int getOperand()
    pub fn getOperand(&self) -> i32 {
        // Java: switch (this._sizeCode)
        match self._sizeCode {
            // Java: case 0:
            // Java: case 1:
            0 | 1 => {
                // Java: return this._grpprl[this._gOffset];
                return self._grpprl[self._gOffset as usize] as i8 as i32;
            },
            // Java: case 2:
            // Java: case 4:
            // Java: case 5:
            2 | 4 | 5 => {
                // Java: return LittleEndian.getShort(this._grpprl, this._gOffset);
                return le_get_short(&self._grpprl, self._gOffset as usize) as i32;
            },
            // Java: case 3:
            3 => {
                // Java: return LittleEndian.getInt(this._grpprl, this._gOffset);
                return le_get_int(&self._grpprl, self._gOffset as usize);
            },
            // Java: case 6:
            6 => {
                // Java: byte operandLength = this._grpprl[this._gOffset + 1];
                let _operandLength = self._grpprl[(self._gOffset + 1) as usize] as i8;
                // Java: if (this._grpprl.length - this._gOffset < operandLength)
                if (self._grpprl.len() as i32 - self._gOffset) < _operandLength as i32 {
                    // Java: byte var10000 = (byte)(this._grpprl.length - this._gOffset);
                    let _var10000 = (self._grpprl.len() as i32 - self._gOffset) as u8;
                }
                // Java: byte[] codeBytes = new byte[4];
                let mut codeBytes = vec![0u8; 4];
                // Java: for(int i = 0; i < codeBytes.length; ++i)
                for i in 0..4usize {
                    // Java: if (this._gOffset + i < this._grpprl.length)
                    if (self._gOffset as usize + i) < self._grpprl.len() {
                        // Java: codeBytes[i] = this._grpprl[this._gOffset + 1 + i];
                        codeBytes[i] = self._grpprl[self._gOffset as usize + 1 + i];
                    }
                }
                // Java: return LittleEndian.getInt(codeBytes, 0);
                return le_get_int(&codeBytes, 0);
            },
            // Java: case 7:
            7 => {
                // Java: return (this._grpprl[this._gOffset + 2] & 255) << 16 | (this._grpprl[this._gOffset + 1] & 255) << 8 | this._grpprl[this._gOffset] & 255;
                return ((self._grpprl[(self._gOffset + 2) as usize] as i32 & 255) << 16)
                    | ((self._grpprl[(self._gOffset + 1) as usize] as i32 & 255) << 8)
                    | (self._grpprl[self._gOffset as usize] as i32 & 255);
            },
            // Java: default:
            _ => {
                // Java: throw new IllegalArgumentException("SPRM contains an invalid size code");
                panic!("SPRM contains an invalid size code");
            }
        }
    }

    // Java: int getSizeCode()
    pub fn getSizeCode(&self) -> i32 {
        // Java: return this._sizeCode;
        self._sizeCode
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this._size;
        self._size
    }

    // Java: int getSizeOperand()
    pub fn getSizeOperand(&self) -> i32 {
        // Java: return this._sizeOperand;
        self._sizeOperand
    }

    // Java: byte[] getGrpprl()
    pub fn getGrpprl(&self) -> &[u8] {
        // Java: return this._grpprl;
        &self._grpprl
    }

    // Java: int initSizeOperand(short sprm)
    fn initSizeOperand(&self, sprm: i16) -> i32 {
        // Java: switch (this._sizeCode)
        match self._sizeCode {
            // Java: case 0:
            // Java: case 1:
            // Java: case 2:
            // Java: case 3:
            // Java: case 4:
            // Java: case 5:
            // Java: case 7:
            0 | 1 | 2 | 3 | 4 | 5 | 7 => {
                // Java: return this._size - 2;
                return self._size - 2;
            },
            // Java: case 6:
            6 => {
                // Java: if (sprm == -10744)
                if sprm == -10744_i16 {
                    // Java: return this._size - 4;
                    return self._size - 4;
                }
                // Java: return this._size - 3;
                return self._size - 3;
            },
            // Java: default:
            _ => {
                // Java: throw new IllegalArgumentException("SPRM contains an invalid size code");
                panic!("SPRM contains an invalid size code");
            }
        }
    }

    // Java: int initSize(short sprm)
    fn initSize(&mut self, sprm: i16) -> i32 {
        // Java: switch (this._sizeCode)
        match self._sizeCode {
            // Java: case 0:
            // Java: case 1:
            0 | 1 => {
                // Java: return 3;
                return 3;
            },
            // Java: case 2:
            // Java: case 4:
            // Java: case 5:
            2 | 4 | 5 => {
                // Java: return 4;
                return 4;
            },
            // Java: case 3:
            3 => {
                // Java: return 6;
                return 6;
            },
            // Java: case 6:
            6 => {
                // Java: if (sprm == -10744)
                if sprm == -10744_i16 {
                    // Java: int retVal = LittleEndian.getUShort(this._grpprl, this._gOffset) + 3;
                    let retVal = le_get_ushort(&self._grpprl, self._gOffset as usize) + 3;
                    // Java: this._gOffset += 2;
                    self._gOffset += 2;
                    // Java: return retVal;
                    return retVal;
                }
                // Java: return (255 & this._grpprl[this._gOffset++]) + 3;
                let val = (255 & self._grpprl[self._gOffset as usize] as i32) + 3;
                self._gOffset += 1;
                return val;
            },
            // Java: case 7:
            7 => {
                // Java: return 5;
                return 5;
            },
            // Java: default:
            _ => {
                // Java: throw new IllegalArgumentException("SPRM contains an invalid size code");
                panic!("SPRM contains an invalid size code");
            }
        }
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, buf: &mut [u8], offset: i32) {
        // Java: switch (this._sizeCode)
        match self._sizeCode {
            // Java: case 0:
            // Java: case 1:
            // Java: case 2:
            // Java: case 3:
            // Java: case 4:
            // Java: case 5:
            // Java: case 7:
            0 | 1 | 2 | 3 | 4 | 5 | 7 => {
                // Java: System.arraycopy(this._grpprl, this._gOffset - 2, buf, offset, this.size());
                let src_start = (self._gOffset - 2) as usize;
                let dst_start = offset as usize;
                let sz = self._size as usize;
                buf[dst_start..dst_start + sz].copy_from_slice(&self._grpprl[src_start..src_start + sz]);
                // Java: break;
            },
            // Java: case 6:
            6 => {
                // Java: System.arraycopy(this._grpprl, this._gOffset - 3, buf, offset, this.size());
                let src_start = (self._gOffset - 3) as usize;
                let dst_start = offset as usize;
                let sz = self._size as usize;
                buf[dst_start..dst_start + sz].copy_from_slice(&self._grpprl[src_start..src_start + sz]);
                // Java: break;
            },
            // Java: default:
            _ => {
                // Java: throw new IllegalArgumentException("SPRM contains an invalid size code");
                panic!("SPRM contains an invalid size code");
            }
        }
    }
}
