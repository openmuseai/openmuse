// Java: import org.apache.poi.hdf.model.hdftypes.HDFType
use super::super::HDFType::HDFType;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.BitField
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.LittleEndian

struct BitField { _mask: i32, _shift_count: i32 }
impl BitField {
    fn new(mask: i32) -> Self {
        let mut count = 0; let mut bp = mask;
        if mask != 0 { while (bp & 1) == 0 { count += 1; bp >>= 1; } }
        BitField { _mask: mask, _shift_count: count }
    }
    fn getValue(&self, holder: i32) -> i32 { ((self.getRawValue(holder) as u32) >> self._shift_count) as i32 }
    fn getRawValue(&self, holder: i32) -> i32 { holder & self._mask }
    fn setValue(&self, holder: i32, value: i32) -> i32 { (holder & !self._mask) | ((value << self._shift_count) & self._mask) }
    fn isSet(&self, holder: i16) -> bool { ((holder as i32) & self._mask) != 0 }
    fn setBoolean(&self, holder: i16, flag: bool) -> i32 { if flag { (holder as i32) | self._mask } else { (holder as i32) & !self._mask } }
}

fn le_get_ushort(buf: &[u8], offset: usize) -> i32 {
    ((buf[offset] as i32) & 0xFF) | (((buf[offset+1] as i32) & 0xFF) << 8)
}
fn le_get_short(buf: &[u8], offset: usize) -> i16 {
    le_get_ushort(buf, offset) as i16
}
fn le_get_int(buf: &[u8], offset: usize) -> i32 {
    (buf[offset] as i32 & 0xFF) | ((buf[offset+1] as i32 & 0xFF) << 8) |
    ((buf[offset+2] as i32 & 0xFF) << 16) | ((buf[offset+3] as i32 & 0xFF) << 24)
}
fn le_put_ushort(buf: &mut [u8], offset: usize, value: i32) {
    buf[offset] = (value & 0xFF) as u8;
    buf[offset+1] = ((value >> 8) & 0xFF) as u8;
}
fn le_put_short(buf: &mut [u8], offset: usize, value: i16) {
    le_put_ushort(buf, offset, value as i32);
}
fn le_put_int(buf: &mut [u8], offset: usize, value: i32) {
    buf[offset] = (value & 0xFF) as u8;
    buf[offset+1] = ((value >> 8) & 0xFF) as u8;
    buf[offset+2] = ((value >> 16) & 0xFF) as u8;
    buf[offset+3] = ((value >> 24) & 0xFF) as u8;
}

// Java field: int FIB_BASE_SIZE
pub(crate) const FIB_BASE_SIZE: i32 = 32;
// Java field: BitField fDot
// Java field: BitField fGlsy
// Java field: BitField fComplex
// Java field: BitField fHasPic
// Java field: BitField cQuickSaves
// Java field: BitField fEncrypted
// Java field: BitField fWhichTblStm
// Java field: BitField fReadOnlyRecommended
// Java field: BitField fWriteReservation
// Java field: BitField fExtChar
// Java field: BitField fLoadOverride
// Java field: BitField fFarEast
// Java field: BitField fCrypto
// Java field: BitField fMac
// Java field: BitField fEmptySpecial
// Java field: BitField fLoadOverridePage
// Java field: BitField fFutureSavedUndo
// Java field: BitField fWord97Saved
// Java field: BitField fSpare0

fn bf_fDot() -> BitField { BitField::new(1) }
fn bf_fGlsy() -> BitField { BitField::new(2) }
fn bf_fComplex() -> BitField { BitField::new(4) }
fn bf_fHasPic() -> BitField { BitField::new(8) }
fn bf_cQuickSaves() -> BitField { BitField::new(240) }
fn bf_fEncrypted() -> BitField { BitField::new(256) }
fn bf_fWhichTblStm() -> BitField { BitField::new(512) }
fn bf_fReadOnlyRecommended() -> BitField { BitField::new(1024) }
fn bf_fWriteReservation() -> BitField { BitField::new(2048) }
fn bf_fExtChar() -> BitField { BitField::new(4096) }
fn bf_fLoadOverride() -> BitField { BitField::new(8192) }
fn bf_fFarEast() -> BitField { BitField::new(16384) }
fn bf_fCrypto() -> BitField { BitField::new(32768) }
fn bf_fMac() -> BitField { BitField::new(1) }
fn bf_fEmptySpecial() -> BitField { BitField::new(2) }
fn bf_fLoadOverridePage() -> BitField { BitField::new(4) }
fn bf_fFutureSavedUndo() -> BitField { BitField::new(8) }
fn bf_fWord97Saved() -> BitField { BitField::new(16) }
fn bf_fSpare0() -> BitField { BitField::new(254) }

// Java: public class FIBAbstractType implements HDFType
pub struct FIBAbstractType {
    // Java field: int field_1_wIdent
    pub(crate) field_1_wIdent: i32,
    // Java field: int field_2_nFib
    pub(crate) field_2_nFib: i32,
    // Java field: int field_3_nProduct
    pub(crate) field_3_nProduct: i32,
    // Java field: int field_4_lid
    pub(crate) field_4_lid: i32,
    // Java field: int field_5_pnNext
    pub(crate) field_5_pnNext: i32,
    // Java field: short field_6_options
    pub(crate) field_6_options: i16,
    // Java field: int field_7_nFibBack
    pub(crate) field_7_nFibBack: i32,
    // Java field: int field_8_lKey
    pub(crate) field_8_lKey: i32,
    // Java field: int field_9_envr
    pub(crate) field_9_envr: i32,
    // Java field: short field_10_history
    pub(crate) field_10_history: i16,
    // Java field: int field_11_chs
    pub(crate) field_11_chs: i32,
    // Java field: int field_12_chsTables
    pub(crate) field_12_chsTables: i32,
    // Java field: int field_13_fcMin
    pub(crate) field_13_fcMin: i32,
    // Java field: int field_14_fcMac
    pub(crate) field_14_fcMac: i32,
}

// Java: public class FIBAbstractType implements HDFType
impl FIBAbstractType {
    // Java: FIBAbstractType()
    pub fn new() -> Self {
      // Java: super();
      FIBAbstractType {
          field_1_wIdent: 0, field_2_nFib: 0, field_3_nProduct: 0, field_4_lid: 0,
          field_5_pnNext: 0, field_6_options: 0, field_7_nFibBack: 0, field_8_lKey: 0,
          field_9_envr: 0, field_10_history: 0, field_11_chs: 0, field_12_chsTables: 0,
          field_13_fcMin: 0, field_14_fcMac: 0,
      }
    }

    // Java: void fillFields(byte[] data, int offset)
    pub fn fillFields_bytes(&mut self, data: &[u8], offset: usize) {
      // Java: this.field_1_wIdent = LittleEndian.getUShort(data, 0 + offset);
      self.field_1_wIdent = le_get_ushort(data, 0 + offset);
      // Java: this.field_2_nFib = LittleEndian.getUShort(data, 2 + offset);
      self.field_2_nFib = le_get_ushort(data, 2 + offset);
      // Java: this.field_3_nProduct = LittleEndian.getUShort(data, 4 + offset);
      self.field_3_nProduct = le_get_ushort(data, 4 + offset);
      // Java: this.field_4_lid = LittleEndian.getUShort(data, 6 + offset);
      self.field_4_lid = le_get_ushort(data, 6 + offset);
      // Java: this.field_5_pnNext = LittleEndian.getUShort(data, 8 + offset);
      self.field_5_pnNext = le_get_ushort(data, 8 + offset);
      // Java: this.field_6_options = LittleEndian.getShort(data, 10 + offset);
      self.field_6_options = le_get_short(data, 10 + offset);
      // Java: this.field_7_nFibBack = LittleEndian.getUShort(data, 12 + offset);
      self.field_7_nFibBack = le_get_ushort(data, 12 + offset);
      // Java: this.field_8_lKey = LittleEndian.getInt(data, 14 + offset);
      self.field_8_lKey = le_get_int(data, 14 + offset);
      // Java: this.field_9_envr = LittleEndian.getByte(data, 18 + offset);
      self.field_9_envr = data[18 + offset] as i8 as i32;
      // Java: this.field_10_history = (short)LittleEndian.getByte(data, 19 + offset);
      self.field_10_history = data[19 + offset] as i8 as i16;
      // Java: this.field_11_chs = LittleEndian.getUShort(data, 20 + offset);
      self.field_11_chs = le_get_ushort(data, 20 + offset);
      // Java: this.field_12_chsTables = LittleEndian.getUShort(data, 22 + offset);
      self.field_12_chsTables = le_get_ushort(data, 22 + offset);
      // Java: this.field_13_fcMin = LittleEndian.getInt(data, 24 + offset);
      self.field_13_fcMin = le_get_int(data, 24 + offset);
      // Java: this.field_14_fcMac = LittleEndian.getInt(data, 28 + offset);
      self.field_14_fcMac = le_get_int(data, 28 + offset);
    }

    // Java: void fillFields(DocumentInputStream data, int offset)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32) {
      // Java: data.seek((long)offset);
      data.seek(offset as i64);
      // Java: this.field_1_wIdent = data.readUShort();
      self.field_1_wIdent = data.readUShort();
      // Java: this.field_2_nFib = data.readUShort();
      self.field_2_nFib = data.readUShort();
      // Java: this.field_3_nProduct = data.readUShort();
      self.field_3_nProduct = data.readUShort();
      // Java: this.field_4_lid = data.readUShort();
      self.field_4_lid = data.readUShort();
      // Java: this.field_5_pnNext = data.readUShort();
      self.field_5_pnNext = data.readUShort();
      // Java: this.field_6_options = data.readShort();
      self.field_6_options = data.readShort();
      // Java: this.field_7_nFibBack = data.readUShort();
      self.field_7_nFibBack = data.readUShort();
      // Java: this.field_8_lKey = data.readInt();
      self.field_8_lKey = data.readInt();
      // Java: this.field_9_envr = data.readByte();
      self.field_9_envr = data.readByte() as i32;
      // Java: this.field_10_history = (short)data.readByte();
      self.field_10_history = data.readByte() as i16;
      // Java: this.field_11_chs = data.readUShort();
      self.field_11_chs = data.readUShort();
      // Java: this.field_12_chsTables = data.readUShort();
      self.field_12_chsTables = data.readUShort();
      // Java: this.field_13_fcMin = data.readInt();
      self.field_13_fcMin = data.readInt();
      // Java: this.field_14_fcMac = data.readInt();
      self.field_14_fcMac = data.readInt();
    }

    // Java: void serialize(byte[] data, int offset)
    pub fn serialize(&self, data: &mut [u8], offset: usize) {
      // Java: LittleEndian.putUShort(data, 0 + offset, this.field_1_wIdent);
      le_put_ushort(data, 0 + offset, self.field_1_wIdent);
      // Java: LittleEndian.putUShort(data, 2 + offset, this.field_2_nFib);
      le_put_ushort(data, 2 + offset, self.field_2_nFib);
      // Java: LittleEndian.putUShort(data, 4 + offset, this.field_3_nProduct);
      le_put_ushort(data, 4 + offset, self.field_3_nProduct);
      // Java: LittleEndian.putUShort(data, 6 + offset, this.field_4_lid);
      le_put_ushort(data, 6 + offset, self.field_4_lid);
      // Java: LittleEndian.putUShort(data, 8 + offset, this.field_5_pnNext);
      le_put_ushort(data, 8 + offset, self.field_5_pnNext);
      // Java: LittleEndian.putShort(data, 10 + offset, this.field_6_options);
      le_put_short(data, 10 + offset, self.field_6_options);
      // Java: LittleEndian.putUShort(data, 12 + offset, this.field_7_nFibBack);
      le_put_ushort(data, 12 + offset, self.field_7_nFibBack);
      // Java: LittleEndian.putInt(data, 14 + offset, this.field_8_lKey);
      le_put_int(data, 14 + offset, self.field_8_lKey);
      // Java: LittleEndian.putByte(data, 18 + offset, this.field_9_envr);
      data[18 + offset] = self.field_9_envr as u8;
      // Java: LittleEndian.putByte(data, 19 + offset, this.field_10_history);
      data[19 + offset] = self.field_10_history as u8;
      // Java: LittleEndian.putUShort(data, 20 + offset, this.field_11_chs);
      le_put_ushort(data, 20 + offset, self.field_11_chs);
      // Java: LittleEndian.putUShort(data, 22 + offset, this.field_12_chsTables);
      le_put_ushort(data, 22 + offset, self.field_12_chsTables);
      // Java: LittleEndian.putInt(data, 24 + offset, this.field_13_fcMin);
      le_put_int(data, 24 + offset, self.field_13_fcMin);
      // Java: LittleEndian.putInt(data, 28 + offset, this.field_14_fcMac);
      le_put_int(data, 28 + offset, self.field_14_fcMac);
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuffer buffer = new StringBuffer();
      let mut buffer = String::new();
      // Java: buffer.append("[FIB]\n");
      buffer.push_str("[FIB]\n");
      // Java: buffer.append("    .wIdent               = ");
      // Java: buffer.append(" (").append(this.getWIdent()).append(" )\n");
      buffer.push_str(&format!("    .wIdent               =  ({} )\n", self.getWIdent()));
      // Java: buffer.append("    .nFib                 = ");
      // Java: buffer.append(" (").append(this.getNFib()).append(" )\n");
      buffer.push_str(&format!("    .nFib                 =  ({} )\n", self.getNFib()));
      // Java: buffer.append("    .nProduct             = ");
      // Java: buffer.append(" (").append(this.getNProduct()).append(" )\n");
      buffer.push_str(&format!("    .nProduct             =  ({} )\n", self.getNProduct()));
      // Java: buffer.append("    .lid                  = ");
      // Java: buffer.append(" (").append(this.getLid()).append(" )\n");
      buffer.push_str(&format!("    .lid                  =  ({} )\n", self.getLid()));
      // Java: buffer.append("    .pnNext               = ");
      // Java: buffer.append(" (").append(this.getPnNext()).append(" )\n");
      buffer.push_str(&format!("    .pnNext               =  ({} )\n", self.getPnNext()));
      // Java: buffer.append("    .options              = ");
      // Java: buffer.append(" (").append(this.getOptions()).append(" )\n");
      buffer.push_str(&format!("    .options              =  ({} )\n", self.getOptions()));
      // Java: buffer.append("         .fDot                     = ").append(this.isFDot()).append('\n');
      buffer.push_str(&format!("         .fDot                     = {}\n", self.isFDot()));
      // Java: buffer.append("         .fGlsy                    = ").append(this.isFGlsy()).append('\n');
      buffer.push_str(&format!("         .fGlsy                    = {}\n", self.isFGlsy()));
      // Java: buffer.append("         .fComplex                 = ").append(this.isFComplex()).append('\n');
      buffer.push_str(&format!("         .fComplex                 = {}\n", self.isFComplex()));
      // Java: buffer.append("         .fHasPic                  = ").append(this.isFHasPic()).append('\n');
      buffer.push_str(&format!("         .fHasPic                  = {}\n", self.isFHasPic()));
      // Java: buffer.append("         .cQuickSaves              = ").append(this.getCQuickSaves()).append('\n');
      buffer.push_str(&format!("         .cQuickSaves              = {}\n", self.getCQuickSaves()));
      // Java: buffer.append("         .fEncrypted               = ").append(this.isFEncrypted()).append('\n');
      buffer.push_str(&format!("         .fEncrypted               = {}\n", self.isFEncrypted()));
      // Java: buffer.append("         .fWhichTblStm             = ").append(this.isFWhichTblStm()).append('\n');
      buffer.push_str(&format!("         .fWhichTblStm             = {}\n", self.isFWhichTblStm()));
      // Java: buffer.append("         .fReadOnlyRecommended     = ").append(this.isFReadOnlyRecommended()).append('\n');
      buffer.push_str(&format!("         .fReadOnlyRecommended     = {}\n", self.isFReadOnlyRecommended()));
      // Java: buffer.append("         .fWriteReservation        = ").append(this.isFWriteReservation()).append('\n');
      buffer.push_str(&format!("         .fWriteReservation        = {}\n", self.isFWriteReservation()));
      // Java: buffer.append("         .fExtChar                 = ").append(this.isFExtChar()).append('\n');
      buffer.push_str(&format!("         .fExtChar                 = {}\n", self.isFExtChar()));
      // Java: buffer.append("         .fLoadOverride            = ").append(this.isFLoadOverride()).append('\n');
      buffer.push_str(&format!("         .fLoadOverride            = {}\n", self.isFLoadOverride()));
      // Java: buffer.append("         .fFarEast                 = ").append(this.isFFarEast()).append('\n');
      buffer.push_str(&format!("         .fFarEast                 = {}\n", self.isFFarEast()));
      // Java: buffer.append("         .fCrypto                  = ").append(this.isFCrypto()).append('\n');
      buffer.push_str(&format!("         .fCrypto                  = {}\n", self.isFCrypto()));
      // Java: buffer.append("    .nFibBack             = ");
      // Java: buffer.append(" (").append(this.getNFibBack()).append(" )\n");
      buffer.push_str(&format!("    .nFibBack             =  ({} )\n", self.getNFibBack()));
      // Java: buffer.append("    .lKey                 = ");
      // Java: buffer.append(" (").append(this.getLKey()).append(" )\n");
      buffer.push_str(&format!("    .lKey                 =  ({} )\n", self.getLKey()));
      // Java: buffer.append("    .envr                 = ");
      // Java: buffer.append(" (").append(this.getEnvr()).append(" )\n");
      buffer.push_str(&format!("    .envr                 =  ({} )\n", self.getEnvr()));
      // Java: buffer.append("    .history              = ");
      // Java: buffer.append(" (").append(this.getHistory()).append(" )\n");
      buffer.push_str(&format!("    .history              =  ({} )\n", self.getHistory()));
      // Java: buffer.append("         .fMac                     = ").append(this.isFMac()).append('\n');
      buffer.push_str(&format!("         .fMac                     = {}\n", self.isFMac()));
      // Java: buffer.append("         .fEmptySpecial            = ").append(this.isFEmptySpecial()).append('\n');
      buffer.push_str(&format!("         .fEmptySpecial            = {}\n", self.isFEmptySpecial()));
      // Java: buffer.append("         .fLoadOverridePage        = ").append(this.isFLoadOverridePage()).append('\n');
      buffer.push_str(&format!("         .fLoadOverridePage        = {}\n", self.isFLoadOverridePage()));
      // Java: buffer.append("         .fFutureSavedUndo         = ").append(this.isFFutureSavedUndo()).append('\n');
      buffer.push_str(&format!("         .fFutureSavedUndo         = {}\n", self.isFFutureSavedUndo()));
      // Java: buffer.append("         .fWord97Saved             = ").append(this.isFWord97Saved()).append('\n');
      buffer.push_str(&format!("         .fWord97Saved             = {}\n", self.isFWord97Saved()));
      // Java: buffer.append("         .fSpare0                  = ").append(this.getFSpare0()).append('\n');
      buffer.push_str(&format!("         .fSpare0                  = {}\n", self.getFSpare0()));
      // Java: buffer.append("    .chs                  = ");
      // Java: buffer.append(" (").append(this.getChs()).append(" )\n");
      buffer.push_str(&format!("    .chs                  =  ({} )\n", self.getChs()));
      // Java: buffer.append("    .chsTables            = ");
      // Java: buffer.append(" (").append(this.getChsTables()).append(" )\n");
      buffer.push_str(&format!("    .chsTables            =  ({} )\n", self.getChsTables()));
      // Java: buffer.append("    .fcMin                = ");
      // Java: buffer.append(" (").append(this.getFcMin()).append(" )\n");
      buffer.push_str(&format!("    .fcMin                =  ({} )\n", self.getFcMin()));
      // Java: buffer.append("    .fcMac                = ");
      // Java: buffer.append(" (").append(this.getFcMac()).append(" )\n");
      buffer.push_str(&format!("    .fcMac                =  ({} )\n", self.getFcMac()));
      // Java: buffer.append("[/FIB]\n");
      buffer.push_str("[/FIB]\n");
      // Java: return buffer.toString();
      buffer
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return 32;
      32
    }

    // Java: int getWIdent()
    pub fn getWIdent(&self) -> i32 {
      // Java: return this.field_1_wIdent;
      self.field_1_wIdent
    }
    // Java: void setWIdent(int field_1_wIdent)
    pub fn setWIdent(&mut self, v: i32) {
      // Java: this.field_1_wIdent = field_1_wIdent;
      self.field_1_wIdent = v;
    }
    // Java: int getNFib()
    pub fn getNFib(&self) -> i32 {
      // Java: return this.field_2_nFib;
      self.field_2_nFib
    }
    // Java: void setNFib(int field_2_nFib)
    pub fn setNFib(&mut self, v: i32) {
      // Java: this.field_2_nFib = field_2_nFib;
      self.field_2_nFib = v;
    }
    // Java: int getNProduct()
    pub fn getNProduct(&self) -> i32 {
      // Java: return this.field_3_nProduct;
      self.field_3_nProduct
    }
    // Java: void setNProduct(int field_3_nProduct)
    pub fn setNProduct(&mut self, v: i32) {
      // Java: this.field_3_nProduct = field_3_nProduct;
      self.field_3_nProduct = v;
    }
    // Java: int getLid()
    pub fn getLid(&self) -> i32 {
      // Java: return this.field_4_lid;
      self.field_4_lid
    }
    // Java: void setLid(int field_4_lid)
    pub fn setLid(&mut self, v: i32) {
      // Java: this.field_4_lid = field_4_lid;
      self.field_4_lid = v;
    }
    // Java: int getPnNext()
    pub fn getPnNext(&self) -> i32 {
      // Java: return this.field_5_pnNext;
      self.field_5_pnNext
    }
    // Java: void setPnNext(int field_5_pnNext)
    pub fn setPnNext(&mut self, v: i32) {
      // Java: this.field_5_pnNext = field_5_pnNext;
      self.field_5_pnNext = v;
    }
    // Java: short getOptions()
    pub fn getOptions(&self) -> i16 {
      // Java: return this.field_6_options;
      self.field_6_options
    }
    // Java: void setOptions(short field_6_options)
    pub fn setOptions(&mut self, v: i16) {
      // Java: this.field_6_options = field_6_options;
      self.field_6_options = v;
    }
    // Java: int getNFibBack()
    pub fn getNFibBack(&self) -> i32 {
      // Java: return this.field_7_nFibBack;
      self.field_7_nFibBack
    }
    // Java: void setNFibBack(int field_7_nFibBack)
    pub fn setNFibBack(&mut self, v: i32) {
      // Java: this.field_7_nFibBack = field_7_nFibBack;
      self.field_7_nFibBack = v;
    }
    // Java: int getLKey()
    pub fn getLKey(&self) -> i32 {
      // Java: return this.field_8_lKey;
      self.field_8_lKey
    }
    // Java: void setLKey(int field_8_lKey)
    pub fn setLKey(&mut self, v: i32) {
      // Java: this.field_8_lKey = field_8_lKey;
      self.field_8_lKey = v;
    }
    // Java: int getEnvr()
    pub fn getEnvr(&self) -> i32 {
      // Java: return this.field_9_envr;
      self.field_9_envr
    }
    // Java: void setEnvr(int field_9_envr)
    pub fn setEnvr(&mut self, v: i32) {
      // Java: this.field_9_envr = field_9_envr;
      self.field_9_envr = v;
    }
    // Java: short getHistory()
    pub fn getHistory(&self) -> i16 {
      // Java: return this.field_10_history;
      self.field_10_history
    }
    // Java: void setHistory(short field_10_history)
    pub fn setHistory(&mut self, v: i16) {
      // Java: this.field_10_history = field_10_history;
      self.field_10_history = v;
    }
    // Java: int getChs()
    pub fn getChs(&self) -> i32 {
      // Java: return this.field_11_chs;
      self.field_11_chs
    }
    // Java: void setChs(int field_11_chs)
    pub fn setChs(&mut self, v: i32) {
      // Java: this.field_11_chs = field_11_chs;
      self.field_11_chs = v;
    }
    // Java: int getChsTables()
    pub fn getChsTables(&self) -> i32 {
      // Java: return this.field_12_chsTables;
      self.field_12_chsTables
    }
    // Java: void setChsTables(int field_12_chsTables)
    pub fn setChsTables(&mut self, v: i32) {
      // Java: this.field_12_chsTables = field_12_chsTables;
      self.field_12_chsTables = v;
    }
    // Java: int getFcMin()
    pub fn getFcMin(&self) -> i32 {
      // Java: return this.field_13_fcMin;
      self.field_13_fcMin
    }
    // Java: void setFcMin(int field_13_fcMin)
    pub fn setFcMin(&mut self, v: i32) {
      // Java: this.field_13_fcMin = field_13_fcMin;
      self.field_13_fcMin = v;
    }
    // Java: int getFcMac()
    pub fn getFcMac(&self) -> i32 {
      // Java: return this.field_14_fcMac;
      self.field_14_fcMac
    }
    // Java: void setFcMac(int field_14_fcMac)
    pub fn setFcMac(&mut self, v: i32) {
      // Java: this.field_14_fcMac = field_14_fcMac;
      self.field_14_fcMac = v;
    }

    // Java: void setFDot(boolean value)
    pub fn setFDot(&mut self, value: bool) {
      // Java: this.field_6_options = (short)fDot.setBoolean(this.field_6_options, value);
      self.field_6_options = bf_fDot().setBoolean(self.field_6_options, value) as i16; }
    // Java: boolean isFDot()
    pub fn isFDot(&self) -> bool {
      // Java: return fDot.isSet(this.field_6_options);
      bf_fDot().isSet(self.field_6_options) }
    // Java: void setFGlsy(boolean value)
    pub fn setFGlsy(&mut self, value: bool) {
      // Java: this.field_6_options = (short)fGlsy.setBoolean(this.field_6_options, value);
      self.field_6_options = bf_fGlsy().setBoolean(self.field_6_options, value) as i16; }
    // Java: boolean isFGlsy()
    pub fn isFGlsy(&self) -> bool {
      // Java: return fGlsy.isSet(this.field_6_options);
      bf_fGlsy().isSet(self.field_6_options) }
    // Java: void setFComplex(boolean value)
    pub fn setFComplex(&mut self, value: bool) {
      // Java: this.field_6_options = (short)fComplex.setBoolean(this.field_6_options, value);
      self.field_6_options = bf_fComplex().setBoolean(self.field_6_options, value) as i16; }
    // Java: boolean isFComplex()
    pub fn isFComplex(&self) -> bool {
      // Java: return fComplex.isSet(this.field_6_options);
      bf_fComplex().isSet(self.field_6_options) }
    // Java: void setFHasPic(boolean value)
    pub fn setFHasPic(&mut self, value: bool) {
      // Java: this.field_6_options = (short)fHasPic.setBoolean(this.field_6_options, value);
      self.field_6_options = bf_fHasPic().setBoolean(self.field_6_options, value) as i16; }
    // Java: boolean isFHasPic()
    pub fn isFHasPic(&self) -> bool {
      // Java: return fHasPic.isSet(this.field_6_options);
      bf_fHasPic().isSet(self.field_6_options) }
    // Java: void setCQuickSaves(byte value)
    pub fn setCQuickSaves(&mut self, value: i8) {
      // Java: this.field_6_options = (short)cQuickSaves.setValue(this.field_6_options, value);
      self.field_6_options = bf_cQuickSaves().setValue(self.field_6_options as i32, value as i32) as i16; }
    // Java: byte getCQuickSaves()
    pub fn getCQuickSaves(&self) -> i8 {
      // Java: return (byte)cQuickSaves.getValue(this.field_6_options);
      bf_cQuickSaves().getValue(self.field_6_options as i32) as i8 }
    // Java: void setFEncrypted(boolean value)
    pub fn setFEncrypted(&mut self, value: bool) {
      // Java: this.field_6_options = (short)fEncrypted.setBoolean(this.field_6_options, value);
      self.field_6_options = bf_fEncrypted().setBoolean(self.field_6_options, value) as i16; }
    // Java: boolean isFEncrypted()
    pub fn isFEncrypted(&self) -> bool {
      // Java: return fEncrypted.isSet(this.field_6_options);
      bf_fEncrypted().isSet(self.field_6_options) }
    // Java: void setFWhichTblStm(boolean value)
    pub fn setFWhichTblStm(&mut self, value: bool) {
      // Java: this.field_6_options = (short)fWhichTblStm.setBoolean(this.field_6_options, value);
      self.field_6_options = bf_fWhichTblStm().setBoolean(self.field_6_options, value) as i16; }
    // Java: boolean isFWhichTblStm()
    pub fn isFWhichTblStm(&self) -> bool {
      // Java: return fWhichTblStm.isSet(this.field_6_options);
      bf_fWhichTblStm().isSet(self.field_6_options) }
    // Java: void setFReadOnlyRecommended(boolean value)
    pub fn setFReadOnlyRecommended(&mut self, value: bool) {
      // Java: this.field_6_options = (short)fReadOnlyRecommended.setBoolean(this.field_6_options, value);
      self.field_6_options = bf_fReadOnlyRecommended().setBoolean(self.field_6_options, value) as i16; }
    // Java: boolean isFReadOnlyRecommended()
    pub fn isFReadOnlyRecommended(&self) -> bool {
      // Java: return fReadOnlyRecommended.isSet(this.field_6_options);
      bf_fReadOnlyRecommended().isSet(self.field_6_options) }
    // Java: void setFWriteReservation(boolean value)
    pub fn setFWriteReservation(&mut self, value: bool) {
      // Java: this.field_6_options = (short)fWriteReservation.setBoolean(this.field_6_options, value);
      self.field_6_options = bf_fWriteReservation().setBoolean(self.field_6_options, value) as i16; }
    // Java: boolean isFWriteReservation()
    pub fn isFWriteReservation(&self) -> bool {
      // Java: return fWriteReservation.isSet(this.field_6_options);
      bf_fWriteReservation().isSet(self.field_6_options) }
    // Java: void setFExtChar(boolean value)
    pub fn setFExtChar(&mut self, value: bool) {
      // Java: this.field_6_options = (short)fExtChar.setBoolean(this.field_6_options, value);
      self.field_6_options = bf_fExtChar().setBoolean(self.field_6_options, value) as i16; }
    // Java: boolean isFExtChar()
    pub fn isFExtChar(&self) -> bool {
      // Java: return fExtChar.isSet(this.field_6_options);
      bf_fExtChar().isSet(self.field_6_options) }
    // Java: void setFLoadOverride(boolean value)
    pub fn setFLoadOverride(&mut self, value: bool) {
      // Java: this.field_6_options = (short)fLoadOverride.setBoolean(this.field_6_options, value);
      self.field_6_options = bf_fLoadOverride().setBoolean(self.field_6_options, value) as i16; }
    // Java: boolean isFLoadOverride()
    pub fn isFLoadOverride(&self) -> bool {
      // Java: return fLoadOverride.isSet(this.field_6_options);
      bf_fLoadOverride().isSet(self.field_6_options) }
    // Java: void setFFarEast(boolean value)
    pub fn setFFarEast(&mut self, value: bool) {
      // Java: this.field_6_options = (short)fFarEast.setBoolean(this.field_6_options, value);
      self.field_6_options = bf_fFarEast().setBoolean(self.field_6_options, value) as i16; }
    // Java: boolean isFFarEast()
    pub fn isFFarEast(&self) -> bool {
      // Java: return fFarEast.isSet(this.field_6_options);
      bf_fFarEast().isSet(self.field_6_options) }
    // Java: void setFCrypto(boolean value)
    pub fn setFCrypto(&mut self, value: bool) {
      // Java: this.field_6_options = (short)fCrypto.setBoolean(this.field_6_options, value);
      self.field_6_options = bf_fCrypto().setBoolean(self.field_6_options, value) as i16; }
    // Java: boolean isFCrypto()
    pub fn isFCrypto(&self) -> bool {
      // Java: return fCrypto.isSet(this.field_6_options);
      bf_fCrypto().isSet(self.field_6_options) }
    // Java: void setFMac(boolean value)
    pub fn setFMac(&mut self, value: bool) {
      // Java: this.field_10_history = (short)fMac.setBoolean(this.field_10_history, value);
      self.field_10_history = bf_fMac().setBoolean(self.field_10_history, value) as i16; }
    // Java: boolean isFMac()
    pub fn isFMac(&self) -> bool {
      // Java: return fMac.isSet(this.field_10_history);
      bf_fMac().isSet(self.field_10_history) }
    // Java: void setFEmptySpecial(boolean value)
    pub fn setFEmptySpecial(&mut self, value: bool) {
      // Java: this.field_10_history = (short)fEmptySpecial.setBoolean(this.field_10_history, value);
      self.field_10_history = bf_fEmptySpecial().setBoolean(self.field_10_history, value) as i16; }
    // Java: boolean isFEmptySpecial()
    pub fn isFEmptySpecial(&self) -> bool {
      // Java: return fEmptySpecial.isSet(this.field_10_history);
      bf_fEmptySpecial().isSet(self.field_10_history) }
    // Java: void setFLoadOverridePage(boolean value)
    pub fn setFLoadOverridePage(&mut self, value: bool) {
      // Java: this.field_10_history = (short)fLoadOverridePage.setBoolean(this.field_10_history, value);
      self.field_10_history = bf_fLoadOverridePage().setBoolean(self.field_10_history, value) as i16; }
    // Java: boolean isFLoadOverridePage()
    pub fn isFLoadOverridePage(&self) -> bool {
      // Java: return fLoadOverridePage.isSet(this.field_10_history);
      bf_fLoadOverridePage().isSet(self.field_10_history) }
    // Java: void setFFutureSavedUndo(boolean value)
    pub fn setFFutureSavedUndo(&mut self, value: bool) {
      // Java: this.field_10_history = (short)fFutureSavedUndo.setBoolean(this.field_10_history, value);
      self.field_10_history = bf_fFutureSavedUndo().setBoolean(self.field_10_history, value) as i16; }
    // Java: boolean isFFutureSavedUndo()
    pub fn isFFutureSavedUndo(&self) -> bool {
      // Java: return fFutureSavedUndo.isSet(this.field_10_history);
      bf_fFutureSavedUndo().isSet(self.field_10_history) }
    // Java: void setFWord97Saved(boolean value)
    pub fn setFWord97Saved(&mut self, value: bool) {
      // Java: this.field_10_history = (short)fWord97Saved.setBoolean(this.field_10_history, value);
      self.field_10_history = bf_fWord97Saved().setBoolean(self.field_10_history, value) as i16; }
    // Java: boolean isFWord97Saved()
    pub fn isFWord97Saved(&self) -> bool {
      // Java: return fWord97Saved.isSet(this.field_10_history);
      bf_fWord97Saved().isSet(self.field_10_history) }
    // Java: void setFSpare0(byte value)
    pub fn setFSpare0(&mut self, value: i8) {
      // Java: this.field_10_history = (short)fSpare0.setValue(this.field_10_history, value);
      self.field_10_history = bf_fSpare0().setValue(self.field_10_history as i32, value as i32) as i16; }
    // Java: byte getFSpare0()
    pub fn getFSpare0(&self) -> i8 {
      // Java: return (byte)fSpare0.getValue(this.field_10_history);
      bf_fSpare0().getValue(self.field_10_history as i32) as i8 }

}

impl HDFType for FIBAbstractType {
}
