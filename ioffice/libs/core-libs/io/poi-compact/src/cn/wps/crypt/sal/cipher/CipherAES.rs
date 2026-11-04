#![allow(non_snake_case, dead_code, unused_variables)]

use super::CipherDirection::*;
use super::CipherError::*;
use super::CipherMode::*;

const BLOCK_SIZE: usize = 16;

pub struct CipherAES {
    mMode: i32,
    mDirection: i32,
    mAlgorithm: i32,
    mKeySize: usize,
    mKey: Vec<u8>,
    mIv: Vec<u8>,
    mW: Vec<Vec<u8>>,
    mState: Vec<Vec<u8>>,
    mTmpRot: Vec<u8>,
    mTmpSub: Vec<u8>,
    mTmp2: Vec<Vec<u8>>,
    mNb: usize,
    mNk: usize,
    mNr: usize,
}

impl CipherAES {
    pub fn new(mode: i32, direction: i32, keySize: i32) -> Self {
        CipherAES {
            mMode: mode,
            mDirection: direction,
            mAlgorithm: 3,
            mKeySize: keySize as usize,
            mKey: Vec::new(),
            mIv: Vec::new(),
            mW: Vec::new(),
            mState: vec![Vec::new(); 4],
            mTmpRot: vec![0u8; 4],
            mTmpSub: vec![0u8; 4],
            mTmp2: vec![vec![0u8; 4]; 4],
            mNb: 4,
            mNk: 0,
            mNr: 0,
        }
    }

    pub fn init(&mut self, direction: i32, keyData: &[u8]) -> i32 {
        if keyData.is_empty() || self.mAlgorithm != 3 {
            return CIPHER_E_ARGUMENT;
        }
        if direction == CIPHER_DIRECTION_INVALID {
            return CIPHER_E_DIRECTION;
        }
        self.mNb = 4;
        match self.mKeySize {
            16 => { self.mNk = 4; self.mNr = 10; }
            24 => { self.mNk = 6; self.mNr = 12; }
            _ => { self.mNk = 8; self.mNr = 14; }
        }
        self.mKey = vec![0u8; self.mNk * 4];
        let copy_len = std::cmp::min(keyData.len(), self.mNk * 4);
        self.mKey[..copy_len].copy_from_slice(&keyData[..copy_len]);
        let row_size = self.mNb * (self.mNr + 1);
        self.mW = (0..row_size).map(|_| vec![0u8; 4]).collect();
        for row in 0..self.mNk {
            self.mW[row][0] = self.mKey[4 * row];
            self.mW[row][1] = self.mKey[4 * row + 1];
            self.mW[row][2] = self.mKey[4 * row + 2];
            self.mW[row][3] = self.mKey[4 * row + 3];
        }
        let mut tmp = vec![0u8; 4];
        for i in self.mNk..row_size {
            tmp[0] = self.mW[i - 1][0];
            tmp[1] = self.mW[i - 1][1];
            tmp[2] = self.mW[i - 1][2];
            tmp[3] = self.mW[i - 1][3];
            if i % self.mNk == 0 {
                let rotted = self.rot_word(&tmp);
                tmp = self.sub_word(&rotted);
                tmp[0] ^= RCON[i / self.mNk][0];
                tmp[1] ^= RCON[i / self.mNk][1];
                tmp[2] ^= RCON[i / self.mNk][2];
                tmp[3] ^= RCON[i / self.mNk][3];
            } else if self.mNk > 6 && i % self.mNk == 4 {
                tmp = self.sub_word(&tmp);
            }
            self.mW[i][0] = self.mW[i - self.mNk][0] ^ tmp[0];
            self.mW[i][1] = self.mW[i - self.mNk][1] ^ tmp[1];
            self.mW[i][2] = self.mW[i - self.mNk][2] ^ tmp[2];
            self.mW[i][3] = self.mW[i - self.mNk][3] ^ tmp[3];
        }
        CIPHER_E_NONE
    }

    pub fn setIv(&mut self, iv: &[u8]) {
        self.mIv = iv.to_vec();
    }

    pub fn decode(&mut self, data: &[u8], offset: usize, nLen: usize, buffer: &mut [u8]) -> i32 {
        if self.mAlgorithm != 3 {
            return CIPHER_E_ALGORITHM;
        }
        if self.mDirection == CIPHER_DIRECTION_INVALID {
            return CIPHER_E_DIRECTION;
        }
        self.update(data, offset, nLen, buffer, 0)
    }

    pub fn encode(&mut self, data: &[u8], offset: usize, nLen: usize, buffer: &mut [u8]) -> i32 {
        if self.mAlgorithm != 3 {
            return CIPHER_E_ALGORITHM;
        }
        if self.mDirection == CIPHER_DIRECTION_INVALID {
            return CIPHER_E_DIRECTION;
        }
        self.update(data, offset, nLen, buffer, 0)
    }

    fn update(&mut self, data: &[u8], offset: usize, nLen: usize, buffer: &mut [u8], buf_offset: usize) -> i32 {
        if nLen == 0 || nLen % BLOCK_SIZE != 0 {
            return CIPHER_E_ARGUMENT;
        }
        if self.mMode == CIPHER_MODE_ECB {
            self.update_ecb(data, offset, nLen, buffer, buf_offset);
        } else if self.mMode == CIPHER_MODE_CBC {
            self.update_cbc(data, offset, nLen, buffer, buf_offset);
        } else if self.mMode == CIPHER_MODE_CFB {
            self.update_cfb(data, offset, nLen, buffer, buf_offset);
        } else {
            return CIPHER_E_MODE;
        }
        CIPHER_E_NONE
    }

    fn update_ecb(&mut self, data: &[u8], mut data_off: usize, nLen: usize, buffer: &mut [u8], mut buf_off: usize) {
        if self.mDirection == CIPHER_DIRECTION_ENCODE {
            for _ in 0..(nLen / BLOCK_SIZE) {
                self.cipher(data, data_off, buffer, buf_off);
                data_off += BLOCK_SIZE; buf_off += BLOCK_SIZE;
            }
        } else {
            for _ in 0..(nLen / BLOCK_SIZE) {
                self.inv_cipher(data, data_off, buffer, buf_off);
                data_off += BLOCK_SIZE; buf_off += BLOCK_SIZE;
            }
        }
    }

    fn update_cbc(&mut self, data: &[u8], mut data_off: usize, nLen: usize, buffer: &mut [u8], mut buf_off: usize) {
        let mut iv = vec![0u8; BLOCK_SIZE];
        iv.copy_from_slice(&self.mIv[..BLOCK_SIZE]);
        if self.mDirection == CIPHER_DIRECTION_ENCODE {
            for _ in 0..(nLen / BLOCK_SIZE) {
                xor_block(&mut iv, 0, data, data_off);
                self.cipher(&iv, 0, buffer, buf_off);
                iv.copy_from_slice(&buffer[buf_off..buf_off + BLOCK_SIZE]);
                data_off += BLOCK_SIZE; buf_off += BLOCK_SIZE;
            }
        } else {
            let mut tmp_in = vec![0u8; BLOCK_SIZE];
            for _ in 0..(nLen / BLOCK_SIZE) {
                tmp_in.copy_from_slice(&data[data_off..data_off + BLOCK_SIZE]);
                self.inv_cipher(data, data_off, buffer, buf_off);
                xor_block(buffer, buf_off, &iv, 0);
                iv.copy_from_slice(&tmp_in);
                data_off += BLOCK_SIZE; buf_off += BLOCK_SIZE;
            }
        }
    }

    fn update_cfb(&mut self, data: &[u8], mut data_off: usize, nLen: usize, buffer: &mut [u8], mut buf_off: usize) {
        let mut iv = vec![0u8; BLOCK_SIZE];
        iv.copy_from_slice(&self.mIv[..BLOCK_SIZE]);
        let mut tmp_in = vec![0u8; BLOCK_SIZE];
        if self.mDirection == CIPHER_DIRECTION_ENCODE {
            for _ in 0..(nLen / BLOCK_SIZE) {
                tmp_in.copy_from_slice(&data[data_off..data_off + BLOCK_SIZE]);
                self.cipher(&iv, 0, buffer, buf_off);
                xor_block(buffer, buf_off, &tmp_in, 0);
                iv.copy_from_slice(&buffer[buf_off..buf_off + BLOCK_SIZE]);
                data_off += BLOCK_SIZE; buf_off += BLOCK_SIZE;
            }
        } else {
            for _ in 0..(nLen / BLOCK_SIZE) {
                tmp_in.copy_from_slice(&data[data_off..data_off + BLOCK_SIZE]);
                self.cipher(&iv, 0, buffer, buf_off);
                xor_block(buffer, buf_off, &tmp_in, 0);
                iv.copy_from_slice(&tmp_in);
                data_off += BLOCK_SIZE; buf_off += BLOCK_SIZE;
            }
        }
    }

    fn cipher(&mut self, input: &[u8], in_off: usize, output: &mut [u8], out_off: usize) {
        for i in 0..4 { if self.mState[i].is_empty() { self.mState[i] = vec![0u8; self.mNb]; } }
        for i in 0..(4 * self.mNb) { self.mState[i % 4][i / 4] = input[in_off + i]; }
        self.add_round_key(0);
        for round in 1..self.mNr {
            self.sub_bytes(); self.shift_rows(); self.mix_columns(); self.add_round_key(round);
        }
        self.sub_bytes(); self.shift_rows(); self.add_round_key(self.mNr);
        for i in 0..(4 * self.mNb) { output[out_off + i] = self.mState[i % 4][i / 4]; }
    }

    fn inv_cipher(&mut self, input: &[u8], in_off: usize, output: &mut [u8], out_off: usize) {
        for i in 0..4 { if self.mState[i].is_empty() { self.mState[i] = vec![0u8; self.mNb]; } }
        for i in 0..(4 * self.mNb) { self.mState[i % 4][i / 4] = input[in_off + i]; }
        self.add_round_key(self.mNr);
        for round in (1..self.mNr).rev() {
            self.inv_shift_rows(); self.inv_sub_bytes(); self.add_round_key(round); self.inv_mix_columns();
        }
        self.inv_shift_rows(); self.inv_sub_bytes(); self.add_round_key(0);
        for i in 0..(4 * self.mNb) { output[out_off + i] = self.mState[i % 4][i / 4]; }
    }

    fn rot_word(&self, word: &[u8]) -> Vec<u8> { vec![word[1], word[2], word[3], word[0]] }
    fn sub_word(&self, word: &[u8]) -> Vec<u8> {
        vec![
            (SBOX[((word[0] as usize) & 0xFF) >> 4][(word[0] as usize) & 0xF]) as u8,
            (SBOX[((word[1] as usize) & 0xFF) >> 4][(word[1] as usize) & 0xF]) as u8,
            (SBOX[((word[2] as usize) & 0xFF) >> 4][(word[2] as usize) & 0xF]) as u8,
            (SBOX[((word[3] as usize) & 0xFF) >> 4][(word[3] as usize) & 0xF]) as u8,
        ]
    }

    fn add_round_key(&mut self, round: usize) {
        for r in 0..4 { for c in 0..4 {
            self.mState[r][c] = ((self.mState[r][c] as i32 & 0xFF) ^ (self.mW[round * 4 + c][r] as i32 & 0xFF)) as u8;
        }}
    }

    fn sub_bytes(&mut self) {
        for r in 0..4 { for c in 0..4 {
            self.mState[r][c] = SBOX[((self.mState[r][c] as usize) & 0xFF) >> 4][(self.mState[r][c] as usize) & 0xF] as u8;
        }}
    }

    fn inv_sub_bytes(&mut self) {
        for r in 0..4 { for c in 0..4 {
            self.mState[r][c] = ISBOX[((self.mState[r][c] as usize) & 0xFF) >> 4][(self.mState[r][c] as usize) & 0xF] as u8;
        }}
    }

    fn shift_rows(&mut self) {
        for r in 0..4 { for c in 0..4 { self.mTmp2[r][c] = self.mState[r][c]; } }
        for r in 1..4 { for c in 0..4 { self.mState[r][c] = self.mTmp2[r][(c + r) % self.mNb]; } }
    }

    fn inv_shift_rows(&mut self) {
        for r in 0..4 { for c in 0..4 { self.mTmp2[r][c] = self.mState[r][c]; } }
        for r in 1..4 { for c in 0..4 { self.mState[r][(c + r) % self.mNb] = self.mTmp2[r][c]; } }
    }

    fn mix_columns(&mut self) {
        for r in 0..4 { for c in 0..4 { self.mTmp2[r][c] = self.mState[r][c]; } }
        for c in 0..4 {
            self.mState[0][c] = (gf02(self.mTmp2[0][c]) ^ gf03(self.mTmp2[1][c]) ^ self.mTmp2[2][c] ^ self.mTmp2[3][c]) as u8;
            self.mState[1][c] = (self.mTmp2[0][c] ^ gf02(self.mTmp2[1][c]) ^ gf03(self.mTmp2[2][c]) ^ self.mTmp2[3][c]) as u8;
            self.mState[2][c] = (self.mTmp2[0][c] ^ self.mTmp2[1][c] ^ gf02(self.mTmp2[2][c]) ^ gf03(self.mTmp2[3][c])) as u8;
            self.mState[3][c] = (gf03(self.mTmp2[0][c]) ^ self.mTmp2[1][c] ^ self.mTmp2[2][c] ^ gf02(self.mTmp2[3][c])) as u8;
        }
    }

    fn inv_mix_columns(&mut self) {
        for r in 0..4 { for c in 0..4 { self.mTmp2[r][c] = self.mState[r][c]; } }
        for c in 0..4 {
            self.mState[0][c] = (gf0e(self.mTmp2[0][c]) ^ gf0b(self.mTmp2[1][c]) ^ gf0d(self.mTmp2[2][c]) ^ gf09(self.mTmp2[3][c])) as u8;
            self.mState[1][c] = (gf09(self.mTmp2[0][c]) ^ gf0e(self.mTmp2[1][c]) ^ gf0b(self.mTmp2[2][c]) ^ gf0d(self.mTmp2[3][c])) as u8;
            self.mState[2][c] = (gf0d(self.mTmp2[0][c]) ^ gf09(self.mTmp2[1][c]) ^ gf0e(self.mTmp2[2][c]) ^ gf0b(self.mTmp2[3][c])) as u8;
            self.mState[3][c] = (gf0b(self.mTmp2[0][c]) ^ gf0d(self.mTmp2[1][c]) ^ gf09(self.mTmp2[2][c]) ^ gf0e(self.mTmp2[3][c])) as u8;
        }
    }
}

fn xor_block(block1: &mut [u8], off1: usize, block2: &[u8], off2: usize) {
    for i in 0..BLOCK_SIZE { block1[off1 + i] ^= block2[off2 + i]; }
}

fn gf02(b: u8) -> u8 { if (b as u32) < 128 { b << 1 } else { (b << 1) ^ 27 } }
fn gf03(b: u8) -> u8 { gf02(b) ^ b }
fn gf09(b: u8) -> u8 { gf02(gf02(gf02(b))) ^ b }
fn gf0b(b: u8) -> u8 { gf02(gf02(gf02(b))) ^ gf02(b) ^ b }
fn gf0d(b: u8) -> u8 { gf02(gf02(gf02(b))) ^ gf02(gf02(b)) ^ b }
fn gf0e(b: u8) -> u8 { gf02(gf02(gf02(b))) ^ gf02(gf02(b)) ^ gf02(b) }

static SBOX: [[u8; 16]; 16] = [
    [99, 124, 119, 123, 242, 107, 111, 197, 48, 1, 103, 43, 254, 215, 171, 118],
    [202, 130, 201, 125, 250, 89, 71, 240, 173, 212, 162, 175, 156, 164, 114, 192],
    [183, 253, 147, 38, 54, 63, 247, 204, 52, 165, 229, 241, 113, 216, 49, 21],
    [4, 199, 35, 195, 24, 150, 5, 154, 7, 18, 128, 226, 235, 39, 178, 117],
    [9, 131, 44, 26, 27, 110, 90, 160, 82, 59, 214, 179, 41, 227, 47, 132],
    [83, 209, 0, 237, 32, 252, 177, 91, 106, 203, 190, 57, 74, 76, 88, 207],
    [208, 239, 170, 251, 67, 77, 51, 133, 69, 249, 2, 127, 80, 60, 159, 168],
    [81, 163, 64, 143, 146, 157, 56, 245, 188, 182, 218, 33, 16, 255, 243, 210],
    [205, 12, 19, 236, 95, 151, 68, 23, 196, 167, 126, 61, 100, 93, 25, 115],
    [96, 129, 79, 220, 34, 42, 144, 136, 70, 238, 184, 20, 222, 94, 11, 219],
    [224, 50, 58, 10, 73, 6, 36, 92, 194, 211, 172, 98, 145, 149, 228, 121],
    [231, 200, 55, 109, 141, 213, 78, 169, 108, 86, 244, 234, 101, 122, 174, 8],
    [186, 120, 37, 46, 28, 166, 180, 198, 232, 221, 116, 31, 75, 189, 139, 138],
    [112, 62, 181, 102, 72, 3, 246, 14, 97, 53, 87, 185, 134, 193, 29, 158],
    [225, 248, 152, 17, 105, 217, 142, 148, 155, 30, 135, 233, 206, 85, 40, 223],
    [140, 161, 137, 13, 191, 230, 66, 104, 65, 153, 45, 15, 176, 84, 187, 22],
];

static ISBOX: [[u8; 16]; 16] = [
    [82, 9, 106, 213, 48, 54, 165, 56, 191, 64, 163, 158, 129, 243, 215, 251],
    [124, 227, 57, 130, 155, 47, 255, 135, 52, 142, 67, 68, 196, 222, 233, 203],
    [84, 123, 148, 50, 166, 194, 35, 61, 238, 76, 149, 11, 66, 250, 195, 78],
    [8, 46, 161, 102, 40, 217, 36, 178, 118, 91, 162, 73, 109, 139, 209, 37],
    [114, 248, 246, 100, 134, 104, 152, 22, 212, 164, 92, 204, 93, 101, 182, 146],
    [108, 112, 72, 80, 253, 237, 185, 218, 94, 21, 70, 87, 167, 141, 157, 132],
    [144, 216, 171, 0, 140, 188, 211, 10, 247, 228, 88, 5, 184, 179, 69, 6],
    [208, 44, 30, 143, 202, 63, 15, 2, 193, 175, 189, 3, 1, 19, 138, 107],
    [58, 145, 17, 65, 79, 103, 220, 234, 151, 242, 207, 206, 240, 180, 230, 115],
    [150, 172, 116, 34, 231, 173, 53, 133, 226, 249, 55, 232, 28, 117, 223, 110],
    [71, 241, 26, 113, 29, 41, 197, 137, 111, 183, 98, 14, 170, 24, 190, 27],
    [252, 86, 62, 75, 198, 210, 121, 32, 154, 219, 192, 254, 120, 205, 90, 244],
    [31, 221, 168, 51, 136, 7, 199, 49, 177, 18, 16, 89, 39, 128, 236, 95],
    [96, 81, 127, 169, 25, 181, 74, 13, 45, 229, 122, 159, 147, 201, 156, 239],
    [160, 224, 59, 77, 174, 42, 245, 176, 200, 235, 187, 60, 131, 83, 153, 97],
    [23, 43, 4, 126, 186, 119, 214, 38, 225, 105, 20, 99, 85, 33, 12, 125],
];

static RCON: [[u8; 4]; 11] = [
    [0, 0, 0, 0], [1, 0, 0, 0], [2, 0, 0, 0], [4, 0, 0, 0],
    [8, 0, 0, 0], [16, 0, 0, 0], [32, 0, 0, 0], [64, 0, 0, 0],
    [128, 0, 0, 0], [27, 0, 0, 0], [54, 0, 0, 0],
];
