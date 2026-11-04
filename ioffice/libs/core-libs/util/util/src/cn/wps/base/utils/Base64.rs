#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use std::sync::OnceLock;

// Java field: int BASELENGTH
pub(crate) const BASELENGTH: i32 = 128;
// Java field: int LOOKUPLENGTH
pub(crate) const LOOKUPLENGTH: i32 = 64;
// Java field: int TWENTYFOURBITGROUP
pub(crate) const TWENTYFOURBITGROUP: i32 = 24;
// Java field: int EIGHTBIT
pub(crate) const EIGHTBIT: i32 = 8;
// Java field: int SIXTEENBIT
pub(crate) const SIXTEENBIT: i32 = 16;
// Java field: int SIXBIT
pub(crate) const SIXBIT: i32 = 6;
// Java field: int FOURBYTE
pub(crate) const FOURBYTE: i32 = 4;
// Java field: int SIGN
pub(crate) const SIGN: i32 = -128;
// Java field: char PAD
pub(crate) const PAD: char = '=';
// Java field: boolean fDebug
pub(crate) const F_DEBUG: bool = false;
// Java field: _base64Alphabet: &[u8]
static BASE64_ALPHABET: OnceLock<[i8; 128]> = OnceLock::new();
// Java field: _lookUpBase64Alphabet: &[char]
static LOOKUP_BASE64_ALPHABET: OnceLock<[char; 64]> = OnceLock::new();

fn get_base64_alphabet() -> &'static [i8; 128] {
    BASE64_ALPHABET.get_or_init(|| {
        let mut arr = [-1i8; 128];
        // Java: for(int i = 90; i >= 65; --i)
        for i in (65..=90).rev() {
            // Java: base64Alphabet[i] = (byte)(i - 65);
            arr[i as usize] = (i - 65) as i8;
        }
        // Java: for(int i = 122; i >= 97; --i)
        for i in (97..=122).rev() {
            // Java: base64Alphabet[i] = (byte)(i - 97 + 26);
            arr[i as usize] = (i - 97 + 26) as i8;
        }
        // Java: for(int i = 57; i >= 48; --i)
        for i in (48..=57).rev() {
            // Java: base64Alphabet[i] = (byte)(i - 48 + 52);
            arr[i as usize] = (i - 48 + 52) as i8;
        }
        // Java: base64Alphabet[43] = 62;
        arr[43] = 62;
        // Java: base64Alphabet[47] = 63;
        arr[47] = 63;
        arr
    })
}

fn get_lookup_base64_alphabet() -> &'static [char; 64] {
    LOOKUP_BASE64_ALPHABET.get_or_init(|| {
        let mut arr = ['\0'; 64];
        // Java: for(int i = 0; i <= 25; ++i)
        for i in 0..=25 {
            // Java: lookUpBase64Alphabet[i] = (char)(65 + i);
            arr[i] = (65 + i as u8) as char;
        }
        // Java: int i = 26;
        let mut idx = 26;
        // Java: for(int j = 0; i <= 51; ++j)
        let mut j = 0;
        while idx <= 51 {
            // Java: lookUpBase64Alphabet[i] = (char)(97 + j);
            arr[idx] = (97 + j as u8) as char;
            // Java: ++i;
            idx += 1;
            j += 1;
        }
        // Java: i = 52;
        idx = 52;
        // Java: for(int j = 0; i <= 61; ++j)
        j = 0;
        while idx <= 61 {
            // Java: lookUpBase64Alphabet[i] = (char)(48 + j);
            arr[idx] = (48 + j as u8) as char;
            // Java: ++i;
            idx += 1;
            j += 1;
        }
        // Java: lookUpBase64Alphabet[62] = '+';
        arr[62] = '+';
        // Java: lookUpBase64Alphabet[63] = '/';
        arr[63] = '/';
        arr
    })
}

// Java: public class Base64
pub struct Base64;

// Java: public class Base64
impl Base64 {
    // Java: Base64()
    pub fn new() -> Self {
        // Java: super();
        Base64
    }

    // Java: static boolean isWhiteSpace(char octect)
    pub fn isWhiteSpace(octect: char) -> bool {
        // Java: return octect == ' ' || octect == '\r' || octect == '\n' || octect == '\t';
        octect == ' ' || octect == '\r' || octect == '\n' || octect == '\t'
    }

    // Java: static boolean isPad(char octect)
    pub fn isPad(octect: char) -> bool {
        // Java: return octect == '=';
        octect == '='
    }

    // Java: static boolean isData(char octect)
    pub fn isData(octect: char) -> bool {
        // Java: return octect < 128 && base64Alphabet[octect] != -1;
        let o = octect as u32;
        o < 128 && get_base64_alphabet()[o as usize] != -1
    }

    // Java: static boolean isBase64(char octect)
    pub fn isBase64(octect: char) -> bool {
        // Java: return isWhiteSpace(octect) || isPad(octect) || isData(octect);
        Self::isWhiteSpace(octect) || Self::isPad(octect) || Self::isData(octect)
    }

    // Java: static String encode(_binaryData: &[u8])
    pub fn encode(binary_data: &[u8]) -> Option<String> {
        let base64_alphabet = get_base64_alphabet();
        let lookup = get_lookup_base64_alphabet();
        // Java: if (binaryData == null)
        // Java: return null;
        // Java: else
        {
            // Java: int lengthDataBits = binaryData.length * 8;
            let length_data_bits = binary_data.len() * 8;
            // Java: if (lengthDataBits == 0)
            if length_data_bits == 0 {
                // Java: return "";
                return Some(String::new());
            }
            // Java: else
            {
                // Java: int fewerThan24bits = lengthDataBits % 24;
                let fewer_than_24bits = length_data_bits % 24;
                // Java: int numberTriplets = lengthDataBits / 24;
                let number_triplets = length_data_bits / 24;
                // Java: int numberQuartet = fewerThan24bits != 0 ? numberTriplets + 1 : numberTriplets;
                let number_quartet = if fewer_than_24bits != 0 { number_triplets + 1 } else { number_triplets };
                // Java: _encodedData: &[char] = null;
                // Java: encodedData = new char[numberQuartet * 4];
                let mut encoded_data = vec!['\0'; number_quartet * 4];
                // Java: byte k = 0;
                let mut k: i8;
                // Java: byte l = 0;
                let mut l: i8;
                // Java: byte b1 = 0;
                let mut b1: i8;
                // Java: byte b2 = 0;
                let mut b2: i8;
                // Java: byte b3 = 0;
                let mut b3: i8;
                // Java: int encodedIndex = 0;
                let mut encoded_index: usize = 0;
                // Java: int dataIndex = 0;
                let mut data_index: usize = 0;

                // Java: for(int i = 0; i < numberTriplets; ++i)
                for _i in 0..number_triplets {
                    // Java: b1 = binaryData[dataIndex++];
                    b1 = binary_data[data_index] as i8; data_index += 1;
                    // Java: b2 = binaryData[dataIndex++];
                    b2 = binary_data[data_index] as i8; data_index += 1;
                    // Java: b3 = binaryData[dataIndex++];
                    b3 = binary_data[data_index] as i8; data_index += 1;
                    // Java: l = (byte)(b2 & 15);
                    l = (b2 & 15) as i8;
                    // Java: k = (byte)(b1 & 3);
                    k = (b1 & 3) as i8;
                    // Java: byte val1 = (b1 & -128) == 0 ? (byte)(b1 >> 2) : (byte)(b1 >> 2 ^ 192);
                    let val1: i8 = if (b1 as i32 & SIGN) == 0 { (b1 >> 2) } else { (b1 >> 2) ^ -64 };
                    // Java: byte val2 = (b2 & -128) == 0 ? (byte)(b2 >> 4) : (byte)(b2 >> 4 ^ 240);
                    let val2: i8 = if (b2 as i32 & SIGN) == 0 { (b2 >> 4) } else { (b2 >> 4) ^ -16 };
                    // Java: byte val3 = (b3 & -128) == 0 ? (byte)(b3 >> 6) : (byte)(b3 >> 6 ^ 252);
                    let val3: i8 = if (b3 as i32 & SIGN) == 0 { (b3 >> 6) } else { (b3 >> 6) ^ -4 };
                    // Java: encodedData[encodedIndex++] = lookUpBase64Alphabet[val1];
                    encoded_data[encoded_index] = lookup[val1 as usize]; encoded_index += 1;
                    // Java: encodedData[encodedIndex++] = lookUpBase64Alphabet[val2 | k << 4];
                    encoded_data[encoded_index] = lookup[(val2 | (k << 4)) as usize]; encoded_index += 1;
                    // Java: encodedData[encodedIndex++] = lookUpBase64Alphabet[l << 2 | val3];
                    encoded_data[encoded_index] = lookup[((l << 2) | val3) as usize]; encoded_index += 1;
                    // Java: encodedData[encodedIndex++] = lookUpBase64Alphabet[b3 & 63];
                    encoded_data[encoded_index] = lookup[(b3 & 63) as usize]; encoded_index += 1;
                }

                // Java: if (fewerThan24bits == 8)
                if fewer_than_24bits == 8 {
                    // Java: b1 = binaryData[dataIndex];
                    b1 = binary_data[data_index] as i8;
                    // Java: k = (byte)(b1 & 3);
                    k = (b1 & 3) as i8;
                    // Java: byte val1 = (b1 & -128) == 0 ? (byte)(b1 >> 2) : (byte)(b1 >> 2 ^ 192);
                    let val1: i8 = if (b1 as i32 & SIGN) == 0 { (b1 >> 2) } else { (b1 >> 2) ^ -64 };
                    // Java: encodedData[encodedIndex++] = lookUpBase64Alphabet[val1];
                    encoded_data[encoded_index] = lookup[val1 as usize]; encoded_index += 1;
                    // Java: encodedData[encodedIndex++] = lookUpBase64Alphabet[k << 4];
                    encoded_data[encoded_index] = lookup[(k << 4) as usize]; encoded_index += 1;
                    // Java: encodedData[encodedIndex++] = '=';
                    encoded_data[encoded_index] = '='; encoded_index += 1;
                    // Java: encodedData[encodedIndex++] = '=';
                    encoded_data[encoded_index] = '='; encoded_index += 1;
                }
                // Java: else if (fewerThan24bits == 16)
                else if fewer_than_24bits == 16 {
                    // Java: b1 = binaryData[dataIndex];
                    b1 = binary_data[data_index] as i8;
                    // Java: b2 = binaryData[dataIndex + 1];
                    b2 = binary_data[data_index + 1] as i8;
                    // Java: l = (byte)(b2 & 15);
                    l = (b2 & 15) as i8;
                    // Java: k = (byte)(b1 & 3);
                    k = (b1 & 3) as i8;
                    // Java: byte val1 = (b1 & -128) == 0 ? (byte)(b1 >> 2) : (byte)(b1 >> 2 ^ 192);
                    let val1: i8 = if (b1 as i32 & SIGN) == 0 { (b1 >> 2) } else { (b1 >> 2) ^ -64 };
                    // Java: byte val2 = (b2 & -128) == 0 ? (byte)(b2 >> 4) : (byte)(b2 >> 4 ^ 240);
                    let val2: i8 = if (b2 as i32 & SIGN) == 0 { (b2 >> 4) } else { (b2 >> 4) ^ -16 };
                    // Java: encodedData[encodedIndex++] = lookUpBase64Alphabet[val1];
                    encoded_data[encoded_index] = lookup[val1 as usize]; encoded_index += 1;
                    // Java: encodedData[encodedIndex++] = lookUpBase64Alphabet[val2 | k << 4];
                    encoded_data[encoded_index] = lookup[(val2 | (k << 4)) as usize]; encoded_index += 1;
                    // Java: encodedData[encodedIndex++] = lookUpBase64Alphabet[l << 2];
                    encoded_data[encoded_index] = lookup[(l << 2) as usize]; encoded_index += 1;
                    // Java: encodedData[encodedIndex++] = '=';
                    encoded_data[encoded_index] = '='; encoded_index += 1;
                }

                // Java: return new String(encodedData);
                Some(encoded_data.into_iter().collect())
            }
        }
    }

    // Java: static _decode: &[u8](String encoded)
    pub fn decode(encoded: &str) -> Option<Vec<u8>> {
        let base64_alphabet = get_base64_alphabet();
        let lookup = get_lookup_base64_alphabet();
        // Java: if (encoded == null)
        // Java: return null;
        // Java: else
        {
            // Java: _base64Data: &[char] = encoded.toCharArray();
            let mut base64_data: Vec<char> = encoded.chars().collect();
            // Java: int len = removeWhiteSpace(base64Data);
            let len = Self::removeWhiteSpace(&mut base64_data);
            // Java: if (len % 4 != 0)
            if len % 4 != 0 {
                // Java: return null;
                return None;
            }
            // Java: else
            {
                // Java: int numberQuadruple = len / 4;
                let number_quadruple = len / 4;
                // Java: if (numberQuadruple == 0)
                if number_quadruple == 0 {
                    // Java: return new byte[0];
                    return Some(Vec::new());
                }
                // Java: else
                {
                    // Java: _decodedData: &[u8] = null;
                    // Java: byte b1 = 0;
                    let mut b1: i8;
                    // Java: byte b2 = 0;
                    let mut b2: i8;
                    // Java: byte b3 = 0;
                    let mut b3: i8;
                    // Java: byte b4 = 0;
                    let mut b4: i8;
                    // Java: char d1 = '\u0000';
                    let mut d1: char;
                    // Java: char d2 = '\u0000';
                    let mut d2: char;
                    // Java: char d3 = '\u0000';
                    let mut d3: char;
                    // Java: char d4 = '\u0000';
                    let mut d4: char;
                    // Java: int i = 0;
                    let mut i: usize = 0;
                    // Java: int encodedIndex = 0;
                    let mut encoded_index: usize = 0;
                    // Java: int dataIndex = 0;
                    let mut data_index: usize = 0;

                    // Java: for(decodedData = new byte[numberQuadruple * 3]; i < numberQuadruple - 1; ++i)
                    let mut decoded_data = vec![0u8; number_quadruple * 3];
                    while i < number_quadruple - 1 {
                        // Java: if (!isData(d1 = base64Data[dataIndex++]) || !isData(d2 = base64Data[dataIndex++]) || !isData(d3 = base64Data[dataIndex++]) || !isData(d4 = base64Data[dataIndex++]))
                        d1 = base64_data[data_index]; data_index += 1;
                        d2 = base64_data[data_index]; data_index += 1;
                        d3 = base64_data[data_index]; data_index += 1;
                        d4 = base64_data[data_index]; data_index += 1;
                        if !Self::isData(d1) || !Self::isData(d2) || !Self::isData(d3) || !Self::isData(d4) {
                            // Java: return null;
                            return None;
                        }

                        // Java: b1 = base64Alphabet[d1];
                        b1 = base64_alphabet[d1 as usize];
                        // Java: b2 = base64Alphabet[d2];
                        b2 = base64_alphabet[d2 as usize];
                        // Java: b3 = base64Alphabet[d3];
                        b3 = base64_alphabet[d3 as usize];
                        // Java: b4 = base64Alphabet[d4];
                        b4 = base64_alphabet[d4 as usize];
                        // Java: decodedData[encodedIndex++] = (byte)(b1 << 2 | b2 >> 4);
                        decoded_data[encoded_index] = ((b1 << 2) | (b2 >> 4)) as u8; encoded_index += 1;
                        // Java: decodedData[encodedIndex++] = (byte)((b2 & 15) << 4 | b3 >> 2 & 15);
                        decoded_data[encoded_index] = (((b2 & 15) << 4) | ((b3 >> 2) & 15)) as u8; encoded_index += 1;
                        // Java: decodedData[encodedIndex++] = (byte)(b3 << 6 | b4);
                        decoded_data[encoded_index] = ((b3 << 6) | b4) as u8; encoded_index += 1;

                        i += 1;
                    }

                    // Java: if (isData(d1 = base64Data[dataIndex++]) && isData(d2 = base64Data[dataIndex++]))
                    d1 = base64_data[data_index]; data_index += 1;
                    d2 = base64_data[data_index]; data_index += 1;
                    if Self::isData(d1) && Self::isData(d2) {
                        // Java: b1 = base64Alphabet[d1];
                        b1 = base64_alphabet[d1 as usize];
                        // Java: b2 = base64Alphabet[d2];
                        b2 = base64_alphabet[d2 as usize];
                        // Java: d3 = base64Data[dataIndex++];
                        d3 = base64_data[data_index]; data_index += 1;
                        // Java: d4 = base64Data[dataIndex++];
                        d4 = base64_data[data_index]; data_index += 1;
                        // Java: if (isData(d3) && isData(d4))
                        if Self::isData(d3) && Self::isData(d4) {
                            // Java: b3 = base64Alphabet[d3];
                            b3 = base64_alphabet[d3 as usize];
                            // Java: b4 = base64Alphabet[d4];
                            b4 = base64_alphabet[d4 as usize];
                            // Java: decodedData[encodedIndex++] = (byte)(b1 << 2 | b2 >> 4);
                            decoded_data[encoded_index] = ((b1 << 2) | (b2 >> 4)) as u8; encoded_index += 1;
                            // Java: decodedData[encodedIndex++] = (byte)((b2 & 15) << 4 | b3 >> 2 & 15);
                            decoded_data[encoded_index] = (((b2 & 15) << 4) | ((b3 >> 2) & 15)) as u8; encoded_index += 1;
                            // Java: decodedData[encodedIndex++] = (byte)(b3 << 6 | b4);
                            decoded_data[encoded_index] = ((b3 << 6) | b4) as u8; encoded_index += 1;
                            // Java: return decodedData;
                            return Some(decoded_data);
                        }
                        // Java: else if (isPad(d3) && isPad(d4))
                        else if Self::isPad(d3) && Self::isPad(d4) {
                            // Java: if ((b2 & 15) != 0)
                            if (b2 & 15) != 0 {
                                // Java: return null;
                                return None;
                            }
                            // Java: else
                            else {
                                // Java: _tmp: &[u8] = new byte[i * 3 + 1];
                                let mut tmp = vec![0u8; i * 3 + 1];
                                // Java: System.arraycopy(decodedData, 0, tmp, 0, i * 3);
                                tmp[..i * 3].copy_from_slice(&decoded_data[..i * 3]);
                                // Java: tmp[encodedIndex] = (byte)(b1 << 2 | b2 >> 4);
                                tmp[encoded_index] = ((b1 << 2) | (b2 >> 4)) as u8;
                                // Java: return tmp;
                                return Some(tmp);
                            }
                        }
                        // Java: else if (!isPad(d3) && isPad(d4))
                        else if !Self::isPad(d3) && Self::isPad(d4) {
                            // Java: b3 = base64Alphabet[d3];
                            b3 = base64_alphabet[d3 as usize];
                            // Java: if ((b3 & 3) != 0)
                            if (b3 & 3) != 0 {
                                // Java: return null;
                                return None;
                            }
                            // Java: else
                            else {
                                // Java: _tmp: &[u8] = new byte[i * 3 + 2];
                                let mut tmp = vec![0u8; i * 3 + 2];
                                // Java: System.arraycopy(decodedData, 0, tmp, 0, i * 3);
                                tmp[..i * 3].copy_from_slice(&decoded_data[..i * 3]);
                                // Java: tmp[encodedIndex++] = (byte)(b1 << 2 | b2 >> 4);
                                tmp[encoded_index] = ((b1 << 2) | (b2 >> 4)) as u8; encoded_index += 1;
                                // Java: tmp[encodedIndex] = (byte)((b2 & 15) << 4 | b3 >> 2 & 15);
                                tmp[encoded_index] = (((b2 & 15) << 4) | ((b3 >> 2) & 15)) as u8;
                                // Java: return tmp;
                                return Some(tmp);
                            }
                        }
                        // Java: else
                        else {
                            // Java: return null;
                            return None;
                        }
                    }
                    // Java: else
                    else {
                        // Java: return null;
                        return None;
                    }
                }
            }
        }
    }

    // Java: static int removeWhiteSpace(_data: &[char])
    pub fn removeWhiteSpace(data: &mut Vec<char>) -> usize {
        // Java: if (data == null)
        // Java: return 0;
        // Java: else
        {
            // Java: int newSize = 0;
            let mut new_size: usize = 0;
            // Java: int len = data.length;
            let len = data.len();
            // Java: for(int i = 0; i < len; ++i)
            for i in 0..len {
                // Java: if (!isWhiteSpace(data[i]))
                if !Self::isWhiteSpace(data[i]) {
                    // Java: data[newSize++] = data[i];
                    data[new_size] = data[i];
                    new_size += 1;
                }
            }
            // Java: return newSize;
            new_size
        }
    }
}
