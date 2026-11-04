// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID

// Java: public class implements implements the WELL class of pseudo
pub struct implements {
}

// Java: public class AbstractWell
pub struct AbstractWell {
    // Java field: int index
    pub(crate) index: i32,
    // Java field: int[] v
    pub(crate) v: Box<dyn std::any::Any>,
    // Java field: int[] iRm1
    pub(crate) iRm1: Box<dyn std::any::Any>,
    // Java field: int[] iRm2
    pub(crate) iRm2: Box<dyn std::any::Any>,
    // Java field: int[] i1
    pub(crate) i1: Box<dyn std::any::Any>,
    // Java field: int[] i2
    pub(crate) i2: Box<dyn std::any::Any>,
    // Java field: int[] i3
    pub(crate) i3: Box<dyn std::any::Any>,
}

// Java: public class implements implements the WELL class of pseudo
impl implements {
}

// Java: public class AbstractWell
impl AbstractWell {
    // Java: AbstractWell(final int k, final int m1, final int m2, final int m3)
    pub fn AbstractWell(&self) {
        // Java: this(k, m1, m2, m3, null);
    }

    // Java: AbstractWell(final int k, final int m1, final int m2, final int m3, final int seed)
    pub fn AbstractWell_2(&self) {
        // Java: this(k, m1, m2, m3, new int[] { seed });
    }

    // Java: AbstractWell(final int k, final int m1, final int m2, final int m3, final int[] seed)
    pub fn AbstractWell_3(&self) {
        // Java: // the bits pool contains k bits, k = r w - p where r is the number
        // Java: // of w bits blocks, w is the block size (always 32 in the original paper)
        // Java: // and p is the number of unused bits in the last block
        // Java: final int w = 32;
        // Java: final int r = (k + w - 1) / w;
        // Java: this.v      = new int[r];
        // Java: this.index  = 0;
        // Java: // precompute indirection index tables. These tables are used for optimizing access
        // Java: // they allow saving computations like "(j + r - 2) % r" with costly modulo operations
        // Java: iRm1 = new int[r];
        // Java: iRm2 = new int[r];
        // Java: i1   = new int[r];
        // Java: i2   = new int[r];
        // Java: i3   = new int[r];
        // Java: for (int j = 0; j < r; ++j)
        {
            // Java: iRm1[j] = (j + r - 1) % r;
            // Java: iRm2[j] = (j + r - 2) % r;
            // Java: i1[j]   = (j + m1)    % r;
            // Java: i2[j]   = (j + m2)    % r;
            // Java: i3[j]   = (j + m3)    % r;
        }
        // Java: // initialize the pool content
        // Java: setSeed(seed);
    }

    // Java: AbstractWell(final int k, final int m1, final int m2, final int m3, final long seed)
    pub fn AbstractWell_4(&self) {
        // Java: this(k, m1, m2, m3, new int[] { (int) (seed >>> 32), (int) (seed & 0xffffffffl) });
    }

    // Java: void setSeed(final int seed)
    pub fn setSeed(&self) {
        // Java: setSeed(new int[] { seed });
    }

    // Java: void setSeed(final int[] seed)
    pub fn setSeed_2(&self) {
        // Java: if (seed == null)
        {
            // Java: setSeed(System.currentTimeMillis() + System.identityHashCode(this));
            // Java: return;
        }
        // Java: System.arraycopy(seed, 0, v, 0, FastMath.min(seed.length, v.length));
        // Java: if (seed.length < v.length)
        {
            // Java: for (int i = seed.length; i < v.length; ++i)
            {
                // Java: final long l = v[i - seed.length];
                // Java: v[i] = (int) ((1812433253l * (l ^ (l >> 30)) + i) & 0xffffffffL);
            }
        }
        // Java: index = 0;
        // Java: clear();  // Clear normal deviate cache
    }

    // Java: void setSeed(final long seed)
    pub fn setSeed_3(&self) {
        // Java: setSeed(new int[] { (int) (seed >>> 32), (int) (seed & 0xffffffffl) });
    }

    // Java: abstract int next(final int bits)
    pub fn next(&self) -> i32 {
        todo!()
    }

}

impl r#implements {
}
