#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.assertion.KSAssert
// Java: import cn.wps.moffice.util.KSLog
// Java: import com.isf.writer.BitWriter
use crate::com::isf::writer::BitWriter::BitWriter;
// Java: import java.io.IOException
// Java: import java.util.ArrayList

use super::BitArray::BitArray;
use super::BitReader::BitReader;
use super::BytesReader::BytesReader;
use super::DeltaTransform::DeltaTransform;

pub(crate) const HUFFMAN_BASES_NUM: i32 = 10;
pub(crate) const HUFFMAN_BASE_SIZE: i32 = 11;

static BIT_AMOUNTS: std::sync::OnceLock<Vec<Vec<i32>>> = std::sync::OnceLock::new();
static HUFFMAN_BASES: std::sync::OnceLock<Vec<Vec<i32>>> = std::sync::OnceLock::new();

fn get_bit_amounts() -> &'static Vec<Vec<i32>> {
    BIT_AMOUNTS.get_or_init(|| {
        vec![
            vec![0, 1, 2, 4, 6, 8, 12, 16, 24, 32, -1],
            vec![0, 1, 1, 2, 4, 8, 12, 16, 24, 32, -1],
            vec![0, 1, 1, 1, 2, 4, 8, 14, 22, 32, -1],
            vec![0, 2, 2, 3, 5, 8, 12, 16, 24, 32, -1],
            vec![0, 3, 4, 5, 8, 12, 12, 16, 24, 32, -1],
        ]
    })
}

fn get_huffman_bases() -> &'static Vec<Vec<i32>> {
    HUFFMAN_BASES.get_or_init(|| {
        vec![
            vec![0, 1, 2, 4, 12, 44, 172, 2220, 34988, 8423596],
            vec![0, 1, 2, 3, 5, 13, 141, 2189, 34957, 8423725],
            vec![0, 1, 2, 3, 4, 6, 14, 142, 8334, 2105486],
            vec![0, 1, 3, 5, 9, 25, 153, 2201, 34969, 8423737],
            vec![0, 1, 5, 13, 29, 157, 2205, 2205, 34973, 8423741],
        ]
    })
}

// Java: public class Huffman
pub struct Huffman {
    // Java field: int HUFFMAN_BASES_NUM
    // Java field: int HUFFMAN_BASE_SIZE
    // Java field: String TAG
    // Java field: int[][] BitAmounts
    // Java field: int[][] HuffmanBases
    _dummy: i32,
}

// Java: public class Huffman
impl Huffman {
    // Java: Huffman()
    pub fn Huffman() -> Self {
      // Java: super();
      Self { _dummy: 0 }
    }

    // Java: static void inflate(BytesReader in, int index, int[] out)
    pub fn inflate(reader: &mut BytesReader, index: i32, out: &mut [i32]) {
      // Java: KSAssert.assertNotNull("in should not be null!", in);
      // Java: KSAssert.assertNotNull("out should not be null!", out);
      // Java: BitReader reader = new BitReader(in);
      let index = index as usize;
      let mut bit_reader = BitReader::BitReader(reader);
      // Java: int[] bitAmounts = BitAmounts[index];
      let bit_amounts_all = get_bit_amounts();
      let bit_amounts = &bit_amounts_all[index];
      // Java: ArrayList<Integer> huffmanBases = prepareHuffmanBases(bitAmounts);
      let huffman_bases = Self::prepareHuffmanBases(bit_amounts);
      // Java: decode(reader, bitAmounts, huffmanBases, out);
      Self::decode(&mut bit_reader, bit_amounts, &huffman_bases, out);
      // Java: DeltaTransform.inverseTransform(out);
      DeltaTransform::inverseTransform(out);
    }

    // Java: static void decode(BitReader reader, int[] bitAmounts, ArrayList<Integer> huffmanBases, int[] out)
    pub fn decode(reader: &mut BitReader<'_>, bit_amounts: &[i32], huffman_bases: &[i32], out: &mut [i32]) {
      // Java: KSAssert.assertNotNull("reader should not be null!", reader);
      // Java: KSAssert.assertNotNull("bitAmounts should not be null!", bitAmounts);
      // Java: KSAssert.assertNotNull("huffmanBases should not be null!", huffmanBases);
      // Java: KSAssert.assertNotNull("out should not be null!", out);
      // Java: int count = 0;
      let mut count: usize = 0;
      // Java: int value = 0;
      let mut value: i32 = 0;
      // Java: boolean bit = false;
      let mut bit: bool;
      // Java: int outIndex = 0;
      let mut out_index: usize = 0;
      // Java: while(outIndex < out.length)
      while out_index < out.len()
      {
         // Java: bit = reader.getBit();
         bit = reader.getBit();
         // Java: if (bit)
         if bit
         {
            // Java: ++count;
            count += 1;
         }
         // Java: else
         else
         {
            // Java: if (0 == count)
            if 0 == count
            {
               // Java: value = 0;
               value = 0;
            }
            // Java: else if (count < bitAmounts.length)
            else if count < bit_amounts.len()
            {
               // Java: int offset = reader.getBits(bitAmounts[count]);
               let offset = reader.getBits(bit_amounts[count]);
               // Java: int sign = offset & 1;
               let sign = offset & 1;
               // Java: offset /= 2;
               let offset = offset / 2;
               // Java: value = (Integer)huffmanBases.get(count) + offset;
               value = huffman_bases[count] + offset;
               // Java: value *= 1 == sign ? -1 : 1;
               value *= if 1 == sign { -1 } else { 1 };
            }
            // Java: else if (count == bitAmounts.length)
            else if count == bit_amounts.len()
            {
               // Java: KSLog.e(TAG, "Unsupported 64-bit value found!");
               // Java: value = 0;
               value = 0;
            }
            // Java: else
            else
            {
               // Java: KSLog.e(TAG, "Decompression error!");
               // Java: value = 0;
               value = 0;
            }
            // Java: out[outIndex++] = value;
            out[out_index] = value;
            out_index += 1;
            // Java: count = 0;
            count = 0;
         }
      }
    }

    // Java: static ArrayList<Integer> prepareHuffmanBases(int[] bitAmounts)
    pub fn prepareHuffmanBases(bit_amounts: &[i32]) -> Vec<i32> {
      // Java: KSAssert.assertNotNull("bitAmounts should not be null!", bitAmounts);
      // Java: ArrayList<Integer> huffmanBases = new ArrayList();
      let mut huffman_bases: Vec<i32> = Vec::new();
      // Java: int base = 1;
      let mut base: i32 = 1;
      // Java: huffmanBases.add(0);
      huffman_bases.push(0);
      // Java: for(byte i = 0; i < bitAmounts.length; ++i)
      for i in 0..bit_amounts.len()
      {
         // Java: int value = bitAmounts[i];
         let value = bit_amounts[i];
         // Java: if (-1 == value)
         if -1 == value
         {
            // Java: break;
            break;
         }
         // Java: if (0 != value)
         if 0 != value
         {
            // Java: huffmanBases.add(base);
            huffman_bases.push(base);
            // Java: base = (int)((double)base + Math.pow((double)2.0F, (double)(value - 1)));
            base = base + (2.0_f64).powi(value - 1) as i32;
         }
      }
      // Java: return huffmanBases;
      huffman_bases
    }

    // Java: static boolean deflate(BitWriter writer, int index, int[] src)
    pub fn deflate(writer: &mut BitWriter, index: usize, src: &[i32]) -> bool {
      // Java: int size = src.length;
      let size = src.len();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size
      {
         // Java: if (!deflateValue(writer, index, src[i]))
         if !Self::deflateValue(writer, index, src[i])
         {
            // Java: KSLog.e(TAG, "Deflating failure for value:" + src[i]);
            // Java: return false;
            return false;
         }
      }
      // Java: return true;
      true
    }

    // Java: static boolean deflateValue(BitWriter writer, int index, int value)
    pub fn deflateValue(writer: &mut BitWriter, index: usize, value: i32) -> bool {
      // Java: int temp = value;
      let mut temp = value;
      // Java: int requiredBits = 0;
      let mut required_bits: i32 = 0;
      // Java: ArrayList<Integer> bitAmounts = new ArrayList(BitAmounts[index].length);
      let bit_amounts_all = get_bit_amounts();
      let mut bit_amounts: Vec<i32> = Vec::new();
      // Java: for(int i = 0; i < BitAmounts[index].length; ++i)
      for i in 0..bit_amounts_all[index].len()
      {
         // Java: bitAmounts.add(BitAmounts[index][i]);
         bit_amounts.push(bit_amounts_all[index][i]);
      }
      // Java: while(0 != temp)
      while 0 != temp
      {
         // Java: temp /= 2;
         temp /= 2;
         // Java: ++requiredBits;
         required_bits += 1;
      }
      // Java: while(true)
      loop
      {
         // Java: int i = bitAmounts.indexOf(requiredBits);
         let idx = bit_amounts.iter().position(|&x| x == required_bits);
         // Java: if (-1 != i)
         if let Some(_idx_val) = idx
         {
            // Java: i = value < 0 ? -value : value;
            let abs_val = if value < 0 { -value } else { value };
            // Java: byte prefixLength;
            let mut prefix_length: usize = 1;
            let huffman_bases_all = get_huffman_bases();
            // Java: for(prefixLength = 1; prefixLength < 10 && i >= HuffmanBases[index][prefixLength]; ++prefixLength)
            while prefix_length < 10 && abs_val >= huffman_bases_all[index][prefix_length]
            {
               prefix_length += 1;
            }
            // Java: BitArray bits = new BitArray(prefixLength);
            let mut bits = BitArray::BitArray(prefix_length as i32);
            // Java: bits.fill(true);
            bits.fill(true);
            // Java: bits.set(bits.length() - 1, false);
            let bits_len = bits.length();
            bits.set(bits_len - 1, false);
            // Java: int size = BitAmounts[index][prefixLength - 1];
            let size = bit_amounts_all[index][prefix_length - 1];
            // Java: int mask = (1 << size - 1) - 1;
            let mask = (1 << (size - 1)) - 1;
            // Java: BitArray valueBits = new BitArray(size);
            let mut value_bits = BitArray::BitArray(size);
            // Java: i = i - HuffmanBases[index][prefixLength - 1] & mask;
            let mut i_val = (abs_val - huffman_bases_all[index][prefix_length - 1]) & mask;
            // Java: i <<= 1;
            i_val <<= 1;
            // Java: i |= value < 0 ? 1 : 0;
            i_val |= if value < 0 { 1 } else { 0 };
            // Java: for(int k = 0; k < size; ++k)
            for k in 0..size
            {
               // Java: int b = k & 1 << size - k - 1;
               let b = i_val & (1 << (size - k - 1));
               // Java: boolean bb = 0 != b;
               let bb = 0 != b;
               // Java: valueBits.set(k, bb);
               value_bits.set(k, bb);
            }
            // Java: try
            {
               // Java: writer.write(bits);
               writer.write_bit_array(&bits);
               // Java: writer.write(valueBits);
               writer.write_bit_array(&value_bits);
            }
            // Java: catch (IOException e)
            {
               // Java: KSLog.e(TAG, "IOException", e);
            }
            // Java: return true;
            return true;
         }
         // Java: ++requiredBits;
         required_bits += 1;
      }
    }

}
