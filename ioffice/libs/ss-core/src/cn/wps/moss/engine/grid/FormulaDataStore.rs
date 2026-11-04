// java final class FormulaDataStore
pub struct FormulaDataStore {
    // java private byte[] buffer = new byte[176];
    buffer: Vec<u8>,
    // java private int size = 0;
    size: i32,
}

// java final class FormulaDataStore
impl FormulaDataStore {
    pub fn FormulaDataStore() -> Self {
        FormulaDataStore {
            buffer: vec![0u8; 176],
            size: 0,
        }
    }

    // java private void ensureCap(int newSize)
    fn ensureCap(&mut self, newSize: i32) {
        // java if (this.buffer.length < newSize)
        if (self.buffer.len() as i32) < newSize {
            // java int newCap = this.buffer.length * 3 / 2;
            let mut newCap = (self.buffer.len() as i32) * 3 / 2;
            // java if (newCap < newSize)
            if newCap < newSize {
                // java newCap = newSize;
                newCap = newSize;
            }
            // java byte[] newBuf = new byte[newCap];
            let mut newBuf = vec![0u8; newCap as usize];
            // java System.arraycopy(this.buffer, 0, newBuf, 0, this.size);
            let n = self.size as usize;
            newBuf[..n].copy_from_slice(&self.buffer[..n]);
            // java this.buffer = newBuf;
            self.buffer = newBuf;
        }
    }

    // java public void get(int id, byte[] out)
    pub fn get(&self, id: i32, out: &mut [u8]) {
        // java if (id == 0)
        if id == 0 {
            // java Arrays.fill(out, (byte)0);
            out.fill(0);
        } else {
            // java int srcPos = (id - 1) * 11;
            let srcPos = (id - 1) * 11;
            // java if (srcPos + 11 > this.buffer.length)
            if srcPos + 11 > self.buffer.len() as i32 {
                // java Arrays.fill(out, (byte)0);
                out.fill(0);
            } else {
                // java System.arraycopy(this.buffer, srcPos, out, 0, 11);
                let start = srcPos as usize;
                out[..11].copy_from_slice(&self.buffer[start..start + 11]);
            }
        }
    }

    // java public int get(int id)
    pub fn get__int(&self, id: i32) -> i32 {
        // java if (id == 0)
        if id == 0 {
            // java return -1;
            -1
        } else {
            // java int srcPos = (id - 1) * 11;
            let srcPos = (id - 1) * 11;
            // java return srcPos + 11 > this.buffer.length ? -1 : srcPos;
            if srcPos + 11 > self.buffer.len() as i32 {
                -1
            } else {
                srcPos
            }
        }
    }

    // java public byte[] data()
    pub fn data(&self) -> &[u8] {
        // java return this.buffer;
        &self.buffer
    }

    // java public int add(byte[] data)
    pub fn add(&mut self, data: Option<&[u8]>) -> i32 {
        // java if (data != null && data.length >= 11)
        if data.is_some() && data.unwrap().len() >= 11 {
            // java this.ensureCap(this.size + 11);
            self.ensureCap(self.size + 11);
            // java System.arraycopy(data, 0, this.buffer, this.size, 11);
            let start = self.size as usize;
            self.buffer[start..start + 11].copy_from_slice(&data.unwrap()[..11]);
            // java this.size += 11;
            self.size += 11;
            // java return this.size / 11;
            self.size / 11
        } else {
            // java return 0;
            0
        }
    }

    // java public void setCalcFlag(int id, byte calcFlag)
    pub fn setCalcFlag(&mut self, id: i32, calcFlag: i8) {
        // java int index = id * 11 - 1;
        let index = id.wrapping_mul(11).wrapping_sub(1);
        // java if (index >= 0 && index < this.buffer.length)
        if index >= 0 && index < self.buffer.len() as i32 {
            // java this.buffer[index] = calcFlag;
            self.buffer[index as usize] = calcFlag as u8;
        }
    }

    // java public byte getCalcFlag(int id)
    pub fn getCalcFlag(&self, id: i32) -> i8 {
        // java int index = id * 11 - 1;
        let index = id.wrapping_mul(11).wrapping_sub(1);
        // java return index >= 0 && index < this.buffer.length ? this.buffer[index] : 1;
        if index >= 0 && index < self.buffer.len() as i32 {
            self.buffer[index as usize] as i8
        } else {
            1
        }
    }
}
