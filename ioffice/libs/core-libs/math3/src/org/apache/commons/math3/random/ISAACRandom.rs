// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID
// Java field: int SIZE_L
pub(crate) const SIZE_L: i32 = 0;
// Java field: int SIZE
pub(crate) const SIZE: i32 = 0;
// Java field: int H_SIZE
pub(crate) const H_SIZE: i32 = 0;
// Java field: int MASK
pub(crate) const MASK: i32 = 0;
// Java field: int GLD_RATIO
pub(crate) const GLD_RATIO: i32 = 0;

// Java: public class ISAACRandom extends BitsStreamGenerator implements Serializable
pub struct ISAACRandom {
    // Java field: int[] rsl
    rsl: Vec<i32>,
    // Java field: int[] mem
    mem: Vec<i32>,
    // Java field: int count
    count: i32,
    // Java field: int isaacA
    isaacA: i32,
    // Java field: int isaacB
    isaacB: i32,
    // Java field: int isaacC
    isaacC: i32,
    // Java field: int[] arr
    arr: Vec<i32>,
    // Java field: int isaacX
    isaacX: i32,
    // Java field: int isaacI
    isaacI: i32,
    // Java field: int isaacJ
    isaacJ: i32,
}

// Java: public class ISAACRandom extends BitsStreamGenerator implements Serializable
impl ISAACRandom {
    // Java: ISAACRandom()
    pub fn ISAACRandom(&self) {
        // Java: setSeed(System.currentTimeMillis() + System.identityHashCode(this));
    }

    // Java: ISAACRandom(long seed)
    pub fn ISAACRandom_2(&self) {
        // Java: setSeed(seed);
    }

    // Java: ISAACRandom(int[] seed)
    pub fn ISAACRandom_3(&self) {
        // Java: setSeed(seed);
    }

    // Java: void setSeed(int seed)
    pub fn setSeed(&self) {
        // Java: setSeed(new int[]{seed});
    }

    // Java: void setSeed(long seed)
    pub fn setSeed_2(&self) {
        // Java: setSeed(new int[]{(int) (seed >>> 32), (int) (seed & 0xffffffffL)});
    }

    // Java: void setSeed(int[] seed)
    pub fn setSeed_3(&self) {
        // Java: if (seed == null)
        {
            // Java: setSeed(System.currentTimeMillis() + System.identityHashCode(this));
            // Java: return;
        }
        // Java: final int seedLen = seed.length;
        // Java: final int rslLen = rsl.length;
        // Java: System.arraycopy(seed, 0, rsl, 0, FastMath.min(seedLen, rslLen));
        // Java: if (seedLen < rslLen)
        {
            // Java: for (int j = seedLen; j < rslLen; j++)
            {
                // Java: long k = rsl[j - seedLen];
                // Java: rsl[j] = (int) (0x6c078965L * (k ^ k >> 30) + j & 0xffffffffL);
            }
        }
        // Java: initState();
    }

    // Java: int next(int bits)
    pub fn next(&self) -> i32 {
        // Java: if (count < 0)
        {
            // Java: isaac();
            // Java: count = SIZE - 1;
        }
        // Java: return rsl[count--] >>> 32 - bits;
        todo!()
    }

    // Java: void isaac()
    pub fn isaac(&self) {
        // Java: isaacI = 0;
        // Java: isaacJ = H_SIZE;
        // Java: isaacB += ++isaacC;
        // Java: while (isaacI < H_SIZE)
        {
            // Java: isaac2();
        }
        // Java: isaacJ = 0;
        // Java: while (isaacJ < H_SIZE)
        {
            // Java: isaac2();
        }
    }

    // Java: void isaac2()
    pub fn isaac2(&self) {
        // Java: isaacX = mem[isaacI];
        // Java: isaacA ^= isaacA << 13;
        // Java: isaacA += mem[isaacJ++];
        // Java: isaac3();
        // Java: isaacX = mem[isaacI];
        // Java: isaacA ^= isaacA >>> 6;
        // Java: isaacA += mem[isaacJ++];
        // Java: isaac3();
        // Java: isaacX = mem[isaacI];
        // Java: isaacA ^= isaacA << 2;
        // Java: isaacA += mem[isaacJ++];
        // Java: isaac3();
        // Java: isaacX = mem[isaacI];
        // Java: isaacA ^= isaacA >>> 16;
        // Java: isaacA += mem[isaacJ++];
        // Java: isaac3();
    }

    // Java: void isaac3()
    pub fn isaac3(&self) {
        // Java: mem[isaacI] = mem[(isaacX & MASK) >> 2] + isaacA + isaacB;
        // Java: isaacB = mem[(mem[isaacI] >> SIZE_L & MASK) >> 2] + isaacX;
        // Java: rsl[isaacI++] = isaacB;
    }

    // Java: void initState()
    pub fn initState(&self) {
        // Java: isaacA = 0;
        // Java: isaacB = 0;
        // Java: isaacC = 0;
        // Java: for (int j = 0; j < arr.length; j++)
        {
            // Java: arr[j] = GLD_RATIO;
        }
        // Java: for (int j = 0; j < 4; j++)
        {
            // Java: shuffle();
        }
        // Java: // fill in mem[] with messy stuff
        // Java: for (int j = 0; j < SIZE; j += 8)
        {
            // Java: arr[0] += rsl[j];
            // Java: arr[1] += rsl[j + 1];
            // Java: arr[2] += rsl[j + 2];
            // Java: arr[3] += rsl[j + 3];
            // Java: arr[4] += rsl[j + 4];
            // Java: arr[5] += rsl[j + 5];
            // Java: arr[6] += rsl[j + 6];
            // Java: arr[7] += rsl[j + 7];
            // Java: shuffle();
            // Java: setState(j);
        }
        // Java: // second pass makes all of seed affect all of mem
        // Java: for (int j = 0; j < SIZE; j += 8)
        {
            // Java: arr[0] += mem[j];
            // Java: arr[1] += mem[j + 1];
            // Java: arr[2] += mem[j + 2];
            // Java: arr[3] += mem[j + 3];
            // Java: arr[4] += mem[j + 4];
            // Java: arr[5] += mem[j + 5];
            // Java: arr[6] += mem[j + 6];
            // Java: arr[7] += mem[j + 7];
            // Java: shuffle();
            // Java: setState(j);
        }
        // Java: isaac();
        // Java: count = SIZE - 1;
        // Java: clear();
    }

    // Java: void shuffle()
    pub fn shuffle(&self) {
        // Java: arr[0] ^= arr[1] << 11;
        // Java: arr[3] += arr[0];
        // Java: arr[1] += arr[2];
        // Java: arr[1] ^= arr[2] >>> 2;
        // Java: arr[4] += arr[1];
        // Java: arr[2] += arr[3];
        // Java: arr[2] ^= arr[3] << 8;
        // Java: arr[5] += arr[2];
        // Java: arr[3] += arr[4];
        // Java: arr[3] ^= arr[4] >>> 16;
        // Java: arr[6] += arr[3];
        // Java: arr[4] += arr[5];
        // Java: arr[4] ^= arr[5] << 10;
        // Java: arr[7] += arr[4];
        // Java: arr[5] += arr[6];
        // Java: arr[5] ^= arr[6] >>> 4;
        // Java: arr[0] += arr[5];
        // Java: arr[6] += arr[7];
        // Java: arr[6] ^= arr[7] << 8;
        // Java: arr[1] += arr[6];
        // Java: arr[7] += arr[0];
        // Java: arr[7] ^= arr[0] >>> 9;
        // Java: arr[2] += arr[7];
        // Java: arr[0] += arr[1];
    }

    // Java: void setState(int start)
    pub fn setState(&self) {
        // Java: mem[start] = arr[0];
        // Java: mem[start + 1] = arr[1];
        // Java: mem[start + 2] = arr[2];
        // Java: mem[start + 3] = arr[3];
        // Java: mem[start + 4] = arr[4];
        // Java: mem[start + 5] = arr[5];
        // Java: mem[start + 6] = arr[6];
        // Java: mem[start + 7] = arr[7];
    }

}

