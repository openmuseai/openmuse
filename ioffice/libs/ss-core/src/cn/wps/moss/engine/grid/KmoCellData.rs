// Java: import cn.wps.moss.util.KmoConstraintCheck
use crate::cn::wps::moss::util::KmoConstraintCheck::KmoConstraintCheck;

use super::KmoCellBitLayout;

// java public class KmoCellData
#[derive(Default)]
pub struct KmoCellData {
    // java public int ixf = 0;
    pub ixf: i32,
    // java public int formulaType = 0;
    pub formulaType: i32,
    // java public boolean hasComment = false;
    pub hasComment: bool,
    // java public boolean isPicInCell = false;
    pub isPicInCell: bool,
    // java public int id = 0;
    pub id: i32,
}

// java public class KmoCellData
impl KmoCellData {
    // java public static final int getId(byte[] buff)
    pub fn getId(buff: &[u8]) -> i32 {
        // java return buff[3] & 255 | (buff[4] & 255) << 8 | (buff[5] & 255) << 16 | (buff[6] & 255) << 24;
        ((buff[3] as u32)
            | ((buff[4] as u32) << 8)
            | ((buff[5] as u32) << 16)
            | ((buff[6] as u32) << 24)) as i32
    }

    // java public static final void setId(byte[] buff, int id)
    pub fn setId(buff: &mut [u8], id: i32) {
        // java buff[3] = (byte)id;
        buff[3] = id as u8;
        // java buff[4] = (byte)(id >> 8);
        buff[4] = (id >> 8) as u8;
        // java buff[5] = (byte)(id >> 16);
        buff[5] = (id >> 16) as u8;
        // java buff[6] = (byte)(id >> 24);
        buff[6] = (id >> 24) as u8;
    }

    // java public static final int getIXF(byte[] buff)
    pub fn getIXF(buff: &[u8]) -> i32 {
        // java return buff[0] & 255 | (buff[1] & 255) << 8;
        (buff[0] as i32 & 255) | (((buff[1] as i32) & 255) << 8)
    }

    // java public static final void setIXF(byte[] buff, int ixf)
    pub fn setIXF(buff: &mut [u8], ixf: i32) {
        // java buff[0] = (byte)ixf;
        buff[0] = ixf as u8;
        // java buff[1] = (byte)(ixf >> 8);
        buff[1] = (ixf >> 8) as u8;
    }

    // java public static final int getType(byte[] buff)
    pub fn getType(buff: &[u8]) -> i32 {
        // java return KmoCellBitLayout.fomulaType.getValue(buff[2]);
        KmoCellBitLayout::fomulaType.getValue(buff[2] as i8 as i32)
    }

    // java public static void setType(byte[] buff, int type)
    pub fn setType(buff: &mut [u8], r#type: i32) {
        // java buff[2] = (byte)type;
        buff[2] = r#type as u8;
    }

    // java public boolean isNull()
    pub fn isNull(&self) -> bool {
        // java return this.ixf == 0 && this.formulaType == 0 && !this.hasComment && this.id == 0 && !this.isPicInCell;
        self.ixf == 0 && self.formulaType == 0 && !self.hasComment && self.id == 0 && !self.isPicInCell
    }

    // java public void clear()
    pub fn clear(&mut self) {
        // java this.ixf = 0;
        self.ixf = 0;
        // java this.formulaType = 0;
        self.formulaType = 0;
        // java this.hasComment = false;
        self.hasComment = false;
        // java this.isPicInCell = false;
        self.isPicInCell = false;
        // java this.id = 0;
        self.id = 0;
    }

    // java public boolean isBlank()
    pub fn isBlank(&self) -> bool {
        // java return this.ixf != 0 && this.formulaType == 0 && !this.hasComment && !this.isPicInCell && this.id == 0;
        self.ixf != 0 && self.formulaType == 0 && !self.hasComment && !self.isPicInCell && self.id == 0
    }

    // java public void read(byte[] buff, int offset)
    pub fn read__bytearr_int(&mut self, buff: &[u8], offset: i32) {
        // java int var10001 = buff[offset] & 255;
        let mut var10001: i32 = buff[offset as usize] as i32 & 255;
        // java int cur = offset + 1;
        let mut cur: i32 = offset + 1;
        // java this.ixf = var10001 | (buff[cur] & 255) << 8;
        self.ixf = var10001 | (((buff[cur as usize] as i32) & 255) << 8);
        // java ++cur;
        cur += 1;
        // java this.formulaType = buff[cur] & 3;
        self.formulaType = (buff[cur as usize] as i32) & 3;
        // java this.hasComment = (buff[cur] & 8) != 0;
        self.hasComment = ((buff[cur as usize] as i32) & 8) != 0;
        // java this.isPicInCell = (buff[cur] & 16) != 0;
        self.isPicInCell = ((buff[cur as usize] as i32) & 16) != 0;
        // java ++cur;
        cur += 1;
        // java var10001 = buff[cur] & 255;
        var10001 = buff[cur as usize] as i32 & 255;
        // java ++cur;
        cur += 1;
        // java var10001 |= (buff[cur] & 255) << 8;
        var10001 |= ((buff[cur as usize] as i32) & 255) << 8;
        // java ++cur;
        cur += 1;
        // java var10001 |= (buff[cur] & 255) << 16;
        var10001 |= ((buff[cur as usize] as i32) & 255) << 16;
        // java ++cur;
        cur += 1;
        // java this.id = var10001 | (buff[cur] & 255) << 24;
        self.id = (var10001 as u32 | (((buff[cur as usize] as u32) & 255) << 24)) as i32;
    }

    // java public byte[] read(byte[] buff)
    pub fn read__1<'a>(&mut self, buff: &'a mut [u8]) -> &'a mut [u8] {
        // java this.ixf = buff[0] & 255 | (buff[1] & 255) << 8;
        self.ixf = (buff[0] as i32 & 255) | (((buff[1] as i32) & 255) << 8);
        // java this.formulaType = buff[2] & 3;
        self.formulaType = (buff[2] as i32) & 3;
        // java this.hasComment = (buff[2] & 8) != 0;
        self.hasComment = ((buff[2] as i32) & 8) != 0;
        // java this.isPicInCell = (buff[2] & 16) != 0;
        self.isPicInCell = ((buff[2] as i32) & 16) != 0;
        // java this.id = buff[3] & 255 | (buff[4] & 255) << 8 | (buff[5] & 255) << 16 | (buff[6] & 255) << 24;
        self.id = ((buff[3] as u32)
            | ((buff[4] as u32) << 8)
            | ((buff[5] as u32) << 16)
            | ((buff[6] as u32) << 24)) as i32;
        // java return buff;
        buff
    }

    // java public byte[] write(byte[] buff)
    pub fn write<'a>(&self, buff: &'a mut [u8]) -> &'a mut [u8] {
        // java KmoConstraintCheck.checkNotNull(buff);
        // rust: no-stmt — Rust 切片不能表示 Java 的 null 数组；null 会在进入本方法前就无法构造。见 §1.7.1
        // java KmoConstraintCheck.checkArgument(buff.length == 7);
        KmoConstraintCheck::checkArgument(buff.len() == 7);
        // java int lower = 0;
        let mut lower: i32 = 0;
        // java lower = KmoCellBitLayout.fomulaType.setValue(lower, this.formulaType);
        lower = KmoCellBitLayout::fomulaType.setValue(lower, self.formulaType);
        // java lower = this.hasComment ? KmoCellBitLayout.hasComment.set(lower) : lower;
        lower = if self.hasComment {
            KmoCellBitLayout::hasComment.set(lower)
        } else {
            lower
        };
        // java lower = this.isPicInCell ? KmoCellBitLayout.isPicInCell.set(lower) : lower;
        lower = if self.isPicInCell {
            KmoCellBitLayout::isPicInCell.set(lower)
        } else {
            lower
        };
        // java buff[0] = (byte)this.ixf;
        buff[0] = self.ixf as u8;
        // java buff[1] = (byte)(this.ixf >> 8);
        buff[1] = (self.ixf >> 8) as u8;
        // java buff[2] = (byte)lower;
        buff[2] = lower as u8;
        // java buff[3] = (byte)this.id;
        buff[3] = self.id as u8;
        // java buff[4] = (byte)(this.id >> 8);
        buff[4] = (self.id >> 8) as u8;
        // java buff[5] = (byte)(this.id >> 16);
        buff[5] = (self.id >> 16) as u8;
        // java buff[6] = (byte)(this.id >> 24);
        buff[6] = (self.id >> 24) as u8;
        // java return buff;
        buff
    }

    // java public boolean equals(Object o)
    pub fn equals(&self, o: Option<&KmoCellData>) -> bool {
        // java if (!(o instanceof KmoCellData))
        if o.is_none() {
            // java return false;
            return false;
        }
        // java else
        else {
            // java KmoCellData cell = (KmoCellData)o;
            let cell = o.unwrap();
            // java return this.formulaType == cell.formulaType && this.hasComment == cell.hasComment && this.isPicInCell == cell.isPicInCell && this.ixf == cell.ixf && this.id == cell.id;
            self.formulaType == cell.formulaType
                && self.hasComment == cell.hasComment
                && self.isPicInCell == cell.isPicInCell
                && self.ixf == cell.ixf
                && self.id == cell.id
        }
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
        // java int result = 17;
        let mut result: i32 = 17;
        // java result = 31 * result + this.formulaType;
        result = result.wrapping_mul(31).wrapping_add(self.formulaType);
        // java result = 31 * result + this.id;
        result = result.wrapping_mul(31).wrapping_add(self.id);
        // java result = 31 * result + this.ixf;
        result = result.wrapping_mul(31).wrapping_add(self.ixf);
        // java return result;
        result
    }
}
