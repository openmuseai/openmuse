// Java field: long serialVersionUID
// Java field: int K
pub(crate) const K: i32 = 0;
// Java field: int M1
pub(crate) const M1: i32 = 0;
// Java field: int M2
pub(crate) const M2: i32 = 0;
// Java field: int M3
pub(crate) const M3: i32 = 0;

// Java: public class implements implements the WELL1024a pseudo
pub struct implements {
}

// Java: public class Well1024a
pub struct Well1024a {
}

// Java: public class implements implements the WELL1024a pseudo
impl implements {
}

// Java: public class Well1024a
impl Well1024a {
    // Java: Well1024a()
    pub fn Well1024a(&self) {
        // Java: super(K, M1, M2, M3);
    }

    // Java: Well1024a(int seed)
    pub fn Well1024a_2(&self) {
        // Java: super(K, M1, M2, M3, seed);
    }

    // Java: Well1024a(int[] seed)
    pub fn Well1024a_3(&self) {
        // Java: super(K, M1, M2, M3, seed);
    }

    // Java: Well1024a(long seed)
    pub fn Well1024a_4(&self) {
        // Java: super(K, M1, M2, M3, seed);
    }

    // Java: int next(final int bits)
    pub fn next(&self) -> i32 {
        // Java: final int indexRm1 = iRm1[index];
        // Java: final int v0       = v[index];
        // Java: final int vM1      = v[i1[index]];
        // Java: final int vM2      = v[i2[index]];
        // Java: final int vM3      = v[i3[index]];
        // Java: final int z0 = v[indexRm1];
        // Java: final int z1 = v0  ^ (vM1 ^ (vM1 >>> 8));
        // Java: final int z2 = (vM2 ^ (vM2 << 19)) ^ (vM3 ^ (vM3 << 14));
        // Java: final int z3 = z1      ^ z2;
        // Java: final int z4 = (z0 ^ (z0 << 11)) ^ (z1 ^ (z1 << 7)) ^ (z2 ^ (z2 << 13));
        // Java: v[index]     = z3;
        // Java: v[indexRm1]  = z4;
        // Java: index        = indexRm1;
        // Java: return z4 >>> (32 - bits);
        todo!()
    }

}

impl r#implements {
}
