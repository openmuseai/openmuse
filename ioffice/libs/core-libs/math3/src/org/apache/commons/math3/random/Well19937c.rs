// Java field: long serialVersionUID
#[allow(dead_code)]
pub(crate) const serialVersionUID: i64 = -7203498180754925124;
// Java field: int K
#[allow(dead_code)]
pub(crate) const K: i32 = 19937;
// Java field: int M1
#[allow(dead_code)]
pub(crate) const M1: i32 = 70;
// Java field: int M2
#[allow(dead_code)]
pub(crate) const M2: i32 = 179;
// Java field: int M3
#[allow(dead_code)]
pub(crate) const M3: i32 = 449;

const BLOCK_SIZE: usize = ((K as usize) + 31) / 32; // 624

// Java: public class implements implements the WELL19937c pseudo
pub struct _Well19937cDoc;

// Java: public class Well19937c
pub struct Well19937c {
    index: usize,
    v: Vec<i32>,
    i_rm1: Vec<usize>,
    i_rm2: Vec<usize>,
    i1: Vec<usize>,
    i2: Vec<usize>,
    i3: Vec<usize>,
}

#[allow(non_snake_case)]
fn build_index_table(offset: usize, size: usize) -> Vec<usize> {
    let mut table = vec![0usize; size];
    for i in 0..size {
        table[i] = (i as isize + offset as isize).rem_euclid(size as isize) as usize;
    }
    table
}

// Java: public class Well19937c
#[allow(non_snake_case)]
impl Well19937c {
    // Java: Well19937c()
    pub fn new() -> Self {
        // Java: super(K, M1, M2, M3);
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i32;
        Self::new_with_int_seed(seed)
    }

    // Java: Well19937c(int seed)
    pub fn new_with_int_seed(seed: i32) -> Self {
        // Java: super(K, M1, M2, M3, seed);
        Self::new_with_arr_seed(&[seed])
    }

    // Java: Well19937c(int[] seed)
    pub fn new_with_arr_seed(seed: &[i32]) -> Self {
        // Java: super(K, M1, M2, M3, seed);
        let r = BLOCK_SIZE;
        let mut v = vec![0i32; r];
        for i in 0..r {
            v[i] = if i < seed.len() { seed[i] } else { 0 };
        }
        let i_rm1 = build_index_table(r - 1, r);
        let i_rm2 = build_index_table(r - 2, r);
        let i1 = build_index_table(M1 as usize, r);
        let i2 = build_index_table(M2 as usize, r);
        let i3 = build_index_table(M3 as usize, r);
        Well19937c { index: 0, v, i_rm1, i_rm2, i1, i2, i3 }
    }

    // Java: Well19937c(long seed)
    pub fn new_with_long_seed(seed: i64) -> Self {
        // Java: super(K, M1, M2, M3, seed);
        Self::new_with_arr_seed(&[(seed as i32), ((seed >> 32) as i32)])
    }

    // Java: int next(final int bits)
    pub fn next(&mut self, bits: i32) -> i32 {
        // Java: final int indexRm1 = iRm1[index];
        let index_rm1 = self.i_rm1[self.index];
        // Java: final int indexRm2 = iRm2[index];
        let index_rm2 = self.i_rm2[self.index];

        // Java: final int v0       = v[index];
        let v0 = self.v[self.index];
        // Java: final int vM1      = v[i1[index]];
        let v_m1 = self.v[self.i1[self.index]];
        // Java: final int vM2      = v[i2[index]];
        let v_m2 = self.v[self.i2[self.index]];
        // Java: final int vM3      = v[i3[index]];
        let v_m3 = self.v[self.i3[self.index]];

        // Java: final int z0 = (0x80000000 & v[indexRm1]) ^ (0x7FFFFFFF & v[indexRm2]);
        let z0 = (0x80000000u32 as i32 & self.v[index_rm1]) ^ (0x7FFFFFFFi32 & self.v[index_rm2]);
        // Java: final int z1 = (v0 ^ (v0 << 25))  ^ (vM1 ^ (vM1 >>> 27));
        let z1 = (v0 ^ (v0 << 25)) ^ (v_m1 ^ ((v_m1 as u32 >> 27) as i32));
        // Java: final int z2 = (vM2 >>> 9) ^ (vM3 ^ (vM3 >>> 1));
        let z2 = ((v_m2 as u32 >> 9) as i32) ^ (v_m3 ^ ((v_m3 as u32 >> 1) as i32));
        // Java: final int z3 = z1      ^ z2;
        let z3 = z1 ^ z2;
        // Java: int z4 = z0 ^ (z1 ^ (z1 << 9)) ^ (z2 ^ (z2 << 21)) ^ (z3 ^ (z3 >>> 21));
        let mut z4 = z0 ^ (z1 ^ (z1 << 9)) ^ (z2 ^ (z2 << 21)) ^ (z3 ^ ((z3 as u32 >> 21) as i32));

        // Java: v[index]     = z3;
        self.v[self.index] = z3;
        // Java: v[indexRm1]  = z4;
        self.v[index_rm1] = z4;
        // Java: v[indexRm2] &= 0x80000000;
        self.v[index_rm2] &= 0x80000000u32 as i32;
        // Java: index        = indexRm1;
        self.index = index_rm1;

        // Java: // add Matsumoto-Kurita tempering
        // Java: // to get a maximally-equidistributed generator
        // Java: z4 ^= (z4 <<  7) & 0xe46e1700;
        z4 ^= (z4 << 7) & 0xe46e1700u32 as i32;
        // Java: z4 ^= (z4 << 15) & 0x9b868000;
        z4 ^= (z4 << 15) & 0x9b868000u32 as i32;

        // Java: return z4 >>> (32 - bits);
        (z4 as u32 >> (32 - bits)) as i32
    }

}
