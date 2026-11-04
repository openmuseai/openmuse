use crate::cn::wps::kso::kproto::KoBin::{IntBin, KoBin, Null};
use crate::cn::wps::kso::kproto::internal::KpPaged::KpPaged;

// Java: KcBitsOp(2130706432) / KcBitsOp(8388607)
const KEY_SHIFT: i32 = 24;
const SIZE_MASK: i32 = 0x7F_FFFF;

// Java: public class KpMapBinTbl
pub struct KpMapBinTbl {
    bh: i32,
    key_count: i32,
}

impl KpMapBinTbl {
    pub fn new() -> Self {
        Self { bh: 0, key_count: 0 }
    }

    // Java: KpMapBinTbl bind(int bh)
    pub fn bind(&mut self, bh: i32) -> &mut Self {
        self.bh = bh;
        let buff = KpPaged::memory1().bh_buff(bh);
        self.key_count = if buff.is_empty() { 0 } else { buff[0] as i32 };
        self
    }

    pub fn bound(bh: i32) -> Self {
        let mut tbl = Self::new();
        tbl.bind(bh);
        tbl
    }

    pub fn bh(&self) -> i32 {
        self.bh
    }

    pub fn size(&self) -> i32 {
        self.key_count
    }

    fn header_count(&self) -> i32 {
        1 + self.key_count * 2
    }

    // Java: int keyAt(int index)
    pub fn key_at(&self, index: i32) -> i32 {
        let buff = KpPaged::memory1().bh_buff(self.bh);
        let off = (1 + index) as usize;
        if off < buff.len() {
            buff[off] as i8 as i32
        } else {
            0
        }
    }

    fn off_at0(&self, index: i32) -> i32 {
        let buff = KpPaged::memory1().bh_buff(self.bh);
        let byte_off = (1 + self.key_count + index) as usize;
        let rel = if byte_off < buff.len() { buff[byte_off] as i32 } else { 0 };
        rel + self.header_count()
    }

    // Java: int sizeAt(int index)
    pub fn size_at(&self, index: i32) -> i32 {
        let buff = KpPaged::memory1().bh_buff(self.bh);
        let off = self.off_at0(index);
        if index == self.key_count - 1 {
            buff.len() as i32 - off
        } else {
            self.off_at0(index + 1) - off
        }
    }

    // Java: KoBin valAt(int index)
    pub fn val_at(&self, index: i32) -> Box<dyn KoBin> {
        // java int off = this.offAt0(index);
        // java KoBin.Factory.ByteArrayBin bb = new KoBin.Factory.ByteArrayBin();
        // java return bb.wrap(this.bhBuff, off, this.sizeAt(index, off));
        //
        // rust: ★W6c-6 —— **转写纠错**。原实现把任意宽度的行**硬编码成 4 字节 `IntBin`**
        //   且按大端装配（`value << 8 | byte`），而 Java 返回的是
        //   **`ByteArrayBin.wrap(bhBuff, off, sizeAt(index))`** —— 即**按原始字节、宽度保真**：
        //   · 4 字节行解出 `IntBin`（`ByteArrayBin.asInt` → `byteArrayToInt`，**小端**）；
        //   · 8 字节行（`RealBin` 写入的 `double`）解出 `double`
        //     （`Decode.toDouble` → `byteArrayToLong` → `longBitsToDouble`）。
        //   硬编码 4 字节 + 大端 ⇒ `GradientStop.pos()` 读回垃圾（实测 `1.27e-260` 级），
        //   且 `RealBin` 的 f64 属性整体不可能读回。
        //   owner: ppt.kso.kproto.internal.KpMapBinTbl    wave: M-PPT-KMOSLIDE-W6c-6
        let buff = KpPaged::memory1().bh_buff(self.bh);
        let off = self.off_at0(index) as usize;
        let n = self.size_at(index).max(0) as usize;
        let end = (off + n).min(buff.len());
        let slice: Vec<u8> = if off < buff.len() {
            buff[off..end].to_vec()
        } else {
            Vec::new()
        };
        let mut bab = crate::cn::wps::kso::kproto::KoBin::ByteArrayBin::default();
        bab.from__bytearr(&slice);
        Box::new(bab)
    }

    // Java: void putVal(int index, KoBin bin)
    pub fn put_val(&mut self, index: i32, bin: &dyn KoBin) {
        if !Null::notEmpty(Some(bin)) {
            return;
        }
        let n = bin.size().max(0) as usize;
        let mut chunk = vec![0u8; n];
        let _ = bin.read(&mut chunk, 0);
        let off = self.off_at0(index) as usize;
        KpPaged::memory1().write_at(self.bh, off, &chunk[..n.min(chunk.len())]);
    }

    fn put_key_at0(&self, index: i32, key: i32) {
        let off = (1 + index) as usize;
        KpPaged::memory1().write_at(self.bh, off, &[key as u8]);
    }

    fn put_off_at0(&self, index: i32, off: i32) {
        let byte_off = (1 + self.key_count + index) as usize;
        KpPaged::memory1().write_at(self.bh, byte_off, &[off as u8]);
    }
}

// Java: KpMapBinTbl.Builder
pub struct Builder {
    tbl: KpMapBinTbl,
    entries: Vec<i32>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            tbl: KpMapBinTbl::new(),
            entries: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    // Java: Builder appendEntry(int key, int size)
    pub fn append_entry(&mut self, key: i32, size: i32) {
        // Java: $KEY.setValue($SIZE.setValue(0, size), key)
        let packed = (key << KEY_SHIFT) | (size & SIZE_MASK);
        self.entries.push(packed);
    }

    pub fn sort(&mut self) {
        self.entries.sort();
    }

    pub fn build(&mut self) -> KpMapBinTbl {
        let bh = self.allocate();
        self.entries.clear();
        KpMapBinTbl::bound(bh)
    }

    fn allocate(&mut self) -> i32 {
        let header = 1 + self.entries.len() as i32 * 2;
        let mut data_size = 0i32;
        for entry in &self.entries {
            data_size += entry & SIZE_MASK;
        }
        let bh = KpPaged::memory1().allocate(header + data_size, self.entries.len() as i32, true);
        self.tbl.bind(bh);
        let mut off = 0i32;
        for i in 0..self.entries.len() {
            let entry = self.entries[i];
            let key = entry >> KEY_SHIFT;
            let size = entry & SIZE_MASK;
            self.tbl.put_key_at0(i as i32, key);
            self.tbl.put_off_at0(i as i32, off);
            off += size;
        }
        bh
    }
}
