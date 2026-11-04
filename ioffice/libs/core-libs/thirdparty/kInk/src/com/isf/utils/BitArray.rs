// Java: import java.io.ByteArrayOutputStream
// Java: import java.util.Arrays

pub(crate) const BITS_PER_UNIT: i32 = 8;
static NYBBLE: std::sync::OnceLock<Vec<Vec<u8>>> = std::sync::OnceLock::new();
pub(crate) const BYTES_PER_LINE: i32 = 8;

fn get_nybble() -> &'static Vec<Vec<u8>> {
    NYBBLE.get_or_init(|| {
        vec![
            vec![b'0', b'0', b'0', b'0'], vec![b'0', b'0', b'0', b'1'],
            vec![b'0', b'0', b'1', b'0'], vec![b'0', b'0', b'1', b'1'],
            vec![b'0', b'1', b'0', b'0'], vec![b'0', b'1', b'0', b'1'],
            vec![b'0', b'1', b'1', b'0'], vec![b'0', b'1', b'1', b'1'],
            vec![b'1', b'0', b'0', b'0'], vec![b'1', b'0', b'0', b'1'],
            vec![b'1', b'0', b'1', b'0'], vec![b'1', b'0', b'1', b'1'],
            vec![b'1', b'1', b'0', b'0'], vec![b'1', b'1', b'0', b'1'],
            vec![b'1', b'1', b'1', b'0'], vec![b'1', b'1', b'1', b'1'],
        ]
    })
}

// Java: public class BitArray implements Cloneable
#[derive(Clone)]
pub struct BitArray {
    // Java field: byte[] repn
    pub(crate) repn: Option<Vec<u8>>,
    // Java field: int length
    pub(crate) length: i32,
    // Java field: int BITS_PER_UNIT
    _bits_per_unit: i32,
    // Java field: byte[][] NYBBLE
    _nybble: i32,
    // Java field: int BYTES_PER_LINE
    _bytes_per_line: i32,
}

// Java: public class BitArray implements Cloneable
impl BitArray {
    // Java: static int subscript(int idx)
    pub fn subscript(idx: i32) -> i32 {
      // Java: return idx / 8;
      idx / 8
    }

    // Java: static int position(int idx)
    pub fn position(idx: i32) -> i32 {
      // Java: return 1 << 7 - idx % 8;
      1 << (7 - idx % 8)
    }

    // Java: BitArray(int length)
    pub fn BitArray(length: i32) -> Self {
      // Java: super();
      // Java: if (length < 0)
      if length < 0
      {
         // Java: throw new IllegalArgumentException("Negative length for BitArray");
         panic!("Negative length for BitArray");
      }
      // Java: else
      {
         // Java: this.length = length;
         // Java: this.repn = new byte[(length + 8 - 1) / 8];
         Self {
             length,
             repn: Some(vec![0u8; ((length + 8 - 1) / 8) as usize]),
             _bits_per_unit: 0, _nybble: 0, _bytes_per_line: 0,
         }
      }
    }

    // Java: BitArray(int length, byte[] a)
    pub fn BitArray_with_bytes(length: i32, a: &[u8]) -> Self {
      // Java: super();
      // Java: if (length < 0)
      if length < 0
      {
         // Java: throw new IllegalArgumentException("Negative length for BitArray");
         panic!("Negative length for BitArray");
      }
      // Java: else if (a.length * 8 < length)
      else if (a.len() as i32) * 8 < length
      {
         // Java: throw new IllegalArgumentException("Byte array too short to represent bit array of given length");
         panic!("Byte array too short to represent bit array of given length");
      }
      // Java: else
      {
         // Java: this.length = length;
         let rep_length = ((length + 8 - 1) / 8) as usize;
         // Java: int repLength = (length + 8 - 1) / 8;
         // Java: int unusedBits = repLength * 8 - length;
         let unused_bits = (rep_length as i32) * 8 - length;
         // Java: byte bitMask = (byte)(255 << unusedBits);
         let bit_mask: u8 = if unused_bits < 8 { (0xFFu32 << unused_bits as u32) as u8 } else { 0 };
         // Java: this.repn = new byte[repLength];
         let mut repn = vec![0u8; rep_length];
         // Java: System.arraycopy(a, 0, this.repn, 0, repLength);
         repn[..rep_length].copy_from_slice(&a[..rep_length]);
         // Java: if (repLength > 0)
         if rep_length > 0
         {
            // Java: byte[] var10000 = this.repn;
            // Java: var10000[repLength - 1] &= bitMask;
            repn[rep_length - 1] &= bit_mask;
         }
         Self { length, repn: Some(repn), _bits_per_unit: 0, _nybble: 0, _bytes_per_line: 0 }
      }
    }

    // Java: BitArray(boolean[] bits)
    pub fn BitArray_from_bools(bits: &[bool]) -> Self {
      // Java: super();
      // Java: this.length = bits.length;
      let length = bits.len() as i32;
      // Java: this.repn = new byte[(this.length + 7) / 8];
      let mut instance = Self {
          length,
          repn: Some(vec![0u8; ((length + 7) / 8) as usize]),
          _bits_per_unit: 0, _nybble: 0, _bytes_per_line: 0,
      };
      // Java: for(int i = 0; i < this.length; ++i)
      for i in 0..length
      {
         // Java: this.set(i, bits[i]);
         instance.set(i, bits[i as usize]);
      }
      instance
    }

    // Java: BitArray(BitArray ba)
    pub fn BitArray_copy(ba: &BitArray) -> Self {
      // Java: super();
      // Java: this.length = ba.length;
      // Java: this.repn = (byte[])ba.repn.clone();
      Self {
          length: ba.length,
          repn: ba.repn.clone(),
          _bits_per_unit: 0, _nybble: 0, _bytes_per_line: 0,
      }
    }

    // Java: boolean get(int index)
    pub fn get(&self, index: i32) -> bool {
      // Java: if (index >= 0 && index < this.length)
      if index >= 0 && index < self.length
      {
         // Java: return (this.repn[subscript(index)] & position(index)) != 0;
         let repn = self.repn.as_ref().unwrap();
         (repn[Self::subscript(index) as usize] as i32 & Self::position(index)) != 0
      }
      // Java: else
      else
      {
         // Java: throw new ArrayIndexOutOfBoundsException(Integer.toString(index));
         panic!("ArrayIndexOutOfBoundsException: {}", index);
      }
    }

    // Java: void set(int index, boolean value)
    pub fn set(&mut self, index: i32, value: bool) {
      // Java: if (index >= 0 && index < this.length)
      if index >= 0 && index < self.length
      {
         // Java: int idx = subscript(index);
         let idx = Self::subscript(index) as usize;
         // Java: int bit = position(index);
         let bit = Self::position(index) as u8;
         // Java: if (value)
         if value
         {
            // Java: byte[] var10000 = this.repn;
            let repn = self.repn.as_mut().unwrap();
            // Java: var10000[idx] = (byte)(var10000[idx] | bit);
            repn[idx] = repn[idx] | bit;
         }
         // Java: else
         else
         {
            // Java: byte[] var5 = this.repn;
            let repn = self.repn.as_mut().unwrap();
            // Java: var5[idx] = (byte)(var5[idx] & ~bit);
            repn[idx] = repn[idx] & !bit;
         }
      }
      // Java: else
      else
      {
         // Java: throw new ArrayIndexOutOfBoundsException(Integer.toString(index));
         panic!("ArrayIndexOutOfBoundsException: {}", index);
      }
    }

    // Java: int length()
    pub fn length(&self) -> i32 {
      // Java: return this.length;
      self.length
    }

    // Java: byte[] toByteArray()
    pub fn toByteArray(&self) -> Vec<u8> {
      // Java: return (byte[])this.repn.clone();
      self.repn.clone().unwrap_or_default()
    }

    // Java: boolean equals(Object obj)
    pub fn equals(&self, obj: &BitArray) -> bool {
      // Java: if (obj == this)
      if std::ptr::eq(self, obj)
      {
         // Java: return true;
         return true;
      }
      // Java: else if (obj != null && obj instanceof BitArray)
      {
         // Java: BitArray ba = (BitArray)obj;
         let ba = obj;
         // Java: if (ba.length != this.length)
         if ba.length != self.length
         {
            // Java: return false;
            return false;
         }
         // Java: else
         {
            // Java: for(int i = 0; i < this.repn.length; ++i)
            let repn = self.repn.as_ref().unwrap();
            let ba_repn = ba.repn.as_ref().unwrap();
            for i in 0..repn.len()
            {
               // Java: if (this.repn[i] != ba.repn[i])
               if repn[i] != ba_repn[i]
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: return true;
            return true;
         }
      }
      // Java: else
      {
         // Java: return false;
         false
      }
    }

    // Java: boolean[] toBooleanArray()
    pub fn toBooleanArray(&self) -> Vec<bool> {
      // Java: boolean[] bits = new boolean[this.length];
      let mut bits = vec![false; self.length as usize];
      // Java: for(int i = 0; i < this.length; ++i)
      for i in 0..self.length
      {
         // Java: bits[i] = this.get(i);
         bits[i as usize] = self.get(i);
      }
      // Java: return bits;
      bits
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int hashCode = 0;
      let mut hash_code: i32 = 0;
      // Java: for(int i = 0; i < this.repn.length; ++i)
      let repn = self.repn.as_ref().unwrap();
      for i in 0..repn.len()
      {
         // Java: hashCode = 31 * hashCode + this.repn[i];
         hash_code = 31_i32.wrapping_mul(hash_code).wrapping_add(repn[i] as i32);
      }
      // Java: return hashCode ^ this.length;
      hash_code ^ self.length
    }

    // Java: Object clone()
    pub fn clone_bit_array(&self) -> BitArray {
      // Java: return new BitArray(this);
      Self::BitArray_copy(self)
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: ByteArrayOutputStream out = new ByteArrayOutputStream();
      let mut out: Vec<u8> = Vec::new();
      let nybble = get_nybble();
      let repn = self.repn.as_ref().unwrap();
      // Java: for(int i = 0; i < this.repn.length - 1; ++i)
      if repn.len() > 0 {
      for i in 0..(repn.len() - 1)
      {
         // Java: out.write(NYBBLE[this.repn[i] >> 4 & 15], 0, 4);
         let hi = ((repn[i] >> 4) & 0x0F) as usize;
         out.extend_from_slice(&nybble[hi][0..4]);
         // Java: out.write(NYBBLE[this.repn[i] & 15], 0, 4);
         let lo = (repn[i] & 0x0F) as usize;
         out.extend_from_slice(&nybble[lo][0..4]);
         // Java: if (i % 8 == 7)
         if i % 8 == 7
         {
            // Java: out.write(10);
            out.push(10);
         }
         // Java: else
         else
         {
            // Java: out.write(32);
            out.push(32);
         }
      }
      }
      // Java: for(int i = 8 * (this.repn.length - 1); i < this.length; ++i)
      let start = if repn.len() > 0 { 8 * (repn.len() as i32 - 1) } else { 0 };
      for i in start..self.length
      {
         // Java: out.write(this.get(i) ? 49 : 48);
         out.push(if self.get(i) { 49 } else { 48 });
      }
      // Java: return new String(out.toByteArray());
      String::from_utf8(out).unwrap_or_default()
    }

    // Java: BitArray truncate()
    pub fn truncate(&self) -> BitArray {
      // Java: for(int i = this.length - 1; i >= 0; --i)
      for i in (0..self.length).rev()
      {
         // Java: if (this.get(i))
         if self.get(i)
         {
            // Java: return new BitArray(i + 1, Arrays.copyOf(this.repn, (i + 8) / 8));
            let repn = self.repn.as_ref().unwrap();
            let new_len = ((i + 8) / 8) as usize;
            let mut new_repn = vec![0u8; new_len];
            let copy_len = new_len.min(repn.len());
            new_repn[..copy_len].copy_from_slice(&repn[..copy_len]);
            return Self::BitArray_with_bytes(i + 1, &new_repn);
         }
      }
      // Java: return new BitArray(1);
      Self::BitArray(1)
    }

    // Java: void fill(boolean b)
    pub fn fill(&mut self, b: bool) {
      // Java: for(int i = 0; i < this.length(); ++i)
      for i in 0..self.length
      {
         // Java: this.set(i, b);
         self.set(i, b);
      }
    }

}
