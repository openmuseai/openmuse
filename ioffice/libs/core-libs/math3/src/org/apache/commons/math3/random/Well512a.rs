// Java field: long serialVersionUID
// Java field: int K
pub(crate) const K: i32 = 0;
// Java field: int M1
pub(crate) const M1: i32 = 0;
// Java field: int M2
pub(crate) const M2: i32 = 0;
// Java field: int M3
pub(crate) const M3: i32 = 0;

// Java: public class implements implements the WELL512a pseudo
pub struct implements {
}

// Java: public class Well512a
pub struct Well512a {
}

// Java: public class implements implements the WELL512a pseudo
impl implements {
}

// Java: public class Well512a
impl Well512a {
    // Java: Well512a()
    pub fn Well512a(&self) {
        // Java: super(K, M1, M2, M3);
    }

    // Java: Well512a(int seed)
    pub fn Well512a_2(&self) {
        // Java: super(K, M1, M2, M3, seed);
    }

    // Java: Well512a(int[] seed)
    pub fn Well512a_3(&self) {
        // Java: super(K, M1, M2, M3, seed);
    }

    // Java: Well512a(long seed)
    pub fn Well512a_4(&self) {
        // Java: super(K, M1, M2, M3, seed);
    }

    // Java: int next(final int bits)
    pub fn next(&self) -> i32 {
        // Java: final int indexRm1 = iRm1[index];
        // Java: final int vi = v[index];
        // Java: final int vi1 = v[i1[index]];
        // Java: final int vi2 = v[i2[index]];
        // Java: final int z0 = v[indexRm1];
        // Java: // the values below include the errata of the original article
        // Java: final int z1 = (vi ^ (vi << 16))   ^ (vi1 ^ (vi1 << 15));
        // Java: final int z2 = vi2 ^ (vi2 >>> 11);
        // Java: final int z3 = z1 ^ z2;
        // Java: final int z4 = (z0 ^ (z0 << 2)) ^ (z1 ^ (z1 << 18)) ^ (z2 << 28) ^ (z3 ^ ((z3 << 5) & 0xda442d24));
        // Java: v[index] = z3;
        // Java: v[indexRm1]  = z4;
        // Java: index    = indexRm1;
        // Java: return z4 >>> (32 - bits);
        todo!()
    }

}

impl r#implements {
}
