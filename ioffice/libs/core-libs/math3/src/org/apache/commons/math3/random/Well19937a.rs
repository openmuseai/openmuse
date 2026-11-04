// Java field: long serialVersionUID
// Java field: int K
pub(crate) const K: i32 = 0;
// Java field: int M1
pub(crate) const M1: i32 = 0;
// Java field: int M2
pub(crate) const M2: i32 = 0;
// Java field: int M3
pub(crate) const M3: i32 = 0;

// Java: public class implements implements the WELL19937a pseudo
pub struct implements {
}

// Java: public class Well19937a
pub struct Well19937a {
}

// Java: public class implements implements the WELL19937a pseudo
impl implements {
}

// Java: public class Well19937a
impl Well19937a {
    // Java: Well19937a()
    pub fn Well19937a(&self) {
        // Java: super(K, M1, M2, M3);
    }

    // Java: Well19937a(int seed)
    pub fn Well19937a_2(&self) {
        // Java: super(K, M1, M2, M3, seed);
    }

    // Java: Well19937a(int[] seed)
    pub fn Well19937a_3(&self) {
        // Java: super(K, M1, M2, M3, seed);
    }

    // Java: Well19937a(long seed)
    pub fn Well19937a_4(&self) {
        // Java: super(K, M1, M2, M3, seed);
    }

    // Java: int next(final int bits)
    pub fn next(&self) -> i32 {
        // Java: final int indexRm1 = iRm1[index];
        // Java: final int indexRm2 = iRm2[index];
        // Java: final int v0       = v[index];
        // Java: final int vM1      = v[i1[index]];
        // Java: final int vM2      = v[i2[index]];
        // Java: final int vM3      = v[i3[index]];
        // Java: final int z0 = (0x80000000 & v[indexRm1]) ^ (0x7FFFFFFF & v[indexRm2]);
        // Java: final int z1 = (v0 ^ (v0 << 25))  ^ (vM1 ^ (vM1 >>> 27));
        // Java: final int z2 = (vM2 >>> 9) ^ (vM3 ^ (vM3 >>> 1));
        // Java: final int z3 = z1      ^ z2;
        // Java: final int z4 = z0 ^ (z1 ^ (z1 << 9)) ^ (z2 ^ (z2 << 21)) ^ (z3 ^ (z3 >>> 21));
        // Java: v[index]     = z3;
        // Java: v[indexRm1]  = z4;
        // Java: v[indexRm2] &= 0x80000000;
        // Java: index        = indexRm1;
        // Java: return z4 >>> (32 - bits);
        todo!()
    }

}

impl r#implements {
}
