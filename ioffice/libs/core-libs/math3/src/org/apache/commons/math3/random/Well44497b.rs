// Java field: long serialVersionUID
// Java field: int K
pub(crate) const K: i32 = 0;
// Java field: int M1
pub(crate) const M1: i32 = 0;
// Java field: int M2
pub(crate) const M2: i32 = 0;
// Java field: int M3
pub(crate) const M3: i32 = 0;

// Java: public class implements implements the WELL44497b pseudo
pub struct implements {
}

// Java: public class Well44497b
pub struct Well44497b {
}

// Java: public class implements implements the WELL44497b pseudo
impl implements {
}

// Java: public class Well44497b
impl Well44497b {
    // Java: Well44497b()
    pub fn Well44497b(&self) {
        // Java: super(K, M1, M2, M3);
    }

    // Java: Well44497b(int seed)
    pub fn Well44497b_2(&self) {
        // Java: super(K, M1, M2, M3, seed);
    }

    // Java: Well44497b(int[] seed)
    pub fn Well44497b_3(&self) {
        // Java: super(K, M1, M2, M3, seed);
    }

    // Java: Well44497b(long seed)
    pub fn Well44497b_4(&self) {
        // Java: super(K, M1, M2, M3, seed);
    }

    // Java: int next(final int bits)
    pub fn next(&self) -> i32 {
        // Java: // compute raw value given by WELL44497a generator
        // Java: // which is NOT maximally-equidistributed
        // Java: final int indexRm1 = iRm1[index];
        // Java: final int indexRm2 = iRm2[index];
        // Java: final int v0       = v[index];
        // Java: final int vM1      = v[i1[index]];
        // Java: final int vM2      = v[i2[index]];
        // Java: final int vM3      = v[i3[index]];
        // Java: // the values below include the errata of the original article
        // Java: final int z0       = (0xFFFF8000 & v[indexRm1]) ^ (0x00007FFF & v[indexRm2]);
        // Java: final int z1       = (v0 ^ (v0 << 24))  ^ (vM1 ^ (vM1 >>> 30));
        // Java: final int z2       = (vM2 ^ (vM2 << 10)) ^ (vM3 << 26);
        // Java: final int z3       = z1      ^ z2;
        // Java: final int z2Prime  = ((z2 << 9) ^ (z2 >>> 23)) & 0xfbffffff;
        // Java: final int z2Second = ((z2 & 0x00020000) != 0) ? (z2Prime ^ 0xb729fcec) : z2Prime;
        // Java: int z4             = z0 ^ (z1 ^ (z1 >>> 20)) ^ z2Second ^ z3;
        // Java: v[index]     = z3;
        // Java: v[indexRm1]  = z4;
        // Java: v[indexRm2] &= 0xFFFF8000;
        // Java: index        = indexRm1;
        // Java: // add Matsumoto-Kurita tempering
        // Java: // to get a maximally-equidistributed generator
        // Java: z4 ^= (z4 <<  7) & 0x93dd1400;
        // Java: z4 ^= (z4 << 15) & 0xfa118000;
        // Java: return z4 >>> (32 - bits);
        todo!()
    }

}

impl r#implements {
}
