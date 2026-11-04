// Java: import java.io.BufferedReader
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.InputStreamReader
// Java: import java.nio.charset.Charset
// Java: import java.util.Arrays
// Java: import java.util.NoSuchElementException
// Java: import java.util.StringTokenizer
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.exception.MathParseException
use crate::org::apache::commons::math3::exception::MathParseException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: int BITS
pub(crate) const BITS: i32 = 0;
// Java field: double SCALE
pub(crate) const SCALE: f64 = 0.0;
// Java field: int MAX_DIMENSION
pub(crate) const MAX_DIMENSION: i32 = 0;
// Java field: String RESOURCE_NAME
pub(crate) const RESOURCE_NAME: &str = "";
// Java field: String FILE_CHARSET
pub(crate) const FILE_CHARSET: &str = "";

// Java: public class SobolSequenceGenerator implements RandomVectorGenerator
pub struct SobolSequenceGenerator {
    // Java field: int dimension
    dimension: i32,
    // Java field: int count
    pub(crate) count: i32,
    // Java field: long[][] direction
    direction: Vec<Vec<i64>>,
    // Java field: long[] x
    x: Vec<i64>,
}

// Java: public class SobolSequenceGenerator implements RandomVectorGenerator
impl SobolSequenceGenerator {
    // Java: SobolSequenceGenerator(final int dimension)
    pub fn SobolSequenceGenerator(&self) {
        // Java: if (dimension < 1 || dimension > MAX_DIMENSION)
        {
            // Java: throw new OutOfRangeException(dimension, 1, MAX_DIMENSION);
        }
        // Java: // initialize the other dimensions with direction numbers from a resource
        // Java: final InputStream is = getClass().getResourceAsStream(RESOURCE_NAME);
        // Java: if (is == null)
        {
            // Java: throw new MathInternalError();
        }
        // Java: this.dimension = dimension;
        // Java: // init data structures
        // Java: direction = new long[dimension][BITS + 1];
        // Java: x = new long[dimension];
        // Java: try
        {
            // Java: initFromStream(is);
        }
        // Java: catch (IOException e)
        {
            // Java: // the internal resource file could not be read -> should not happen
            // Java: throw new MathInternalError();
        }
        // Java: catch (MathParseException e)
        {
            // Java: // the internal resource file could not be parsed -> should not happen
            // Java: throw new MathInternalError();
        }
        // Java: finally
        {
            // Java: try
            {
                // Java: is.close();
            }
                // Java: // ignore
        }
    }

    // Java: SobolSequenceGenerator(final int dimension, final InputStream is)
    pub fn SobolSequenceGenerator_2(&self) {
    }

    // Java: int initFromStream(final InputStream is)
    pub fn initFromStream(&self) -> i32 {
        // Java: // special case: dimension 1 -> use unit initialization
        // Java: for (int i = 1; i <= BITS; i++)
        {
            // Java: direction[0][i] = 1l << (BITS - i);
        }
        // Java: final Charset charset = Charset.forName(FILE_CHARSET);
        // Java: final BufferedReader reader = new BufferedReader(new InputStreamReader(is, charset));
        // Java: int dim = -1;
        // Java: try
        {
            // Java: // ignore first line
            // Java: reader.readLine();
            // Java: int lineNumber = 2;
            // Java: int index = 1;
            // Java: String line = null;
            // Java: while ( (line = reader.readLine()) != null)
            {
                // Java: StringTokenizer st = new StringTokenizer(line, " ");
                // Java: try
                {
                    // Java: dim = Integer.parseInt(st.nextToken());
                    // Java: if (dim >= 2 && dim <= dimension) { // we have found the right dimension
                        // Java: final int s = Integer.parseInt(st.nextToken());
                        // Java: final int a = Integer.parseInt(st.nextToken());
                        // Java: final int[] m = new int[s + 1];
                        // Java: for (int i = 1; i <= s; i++)
                        {
                            // Java: m[i] = Integer.parseInt(st.nextToken());
                        }
                        // Java: initDirectionVector(index++, a, m);
                    // Java: if (dim > dimension)
                    {
                        // Java: return dim;
                    }
                }
                // Java: catch (NoSuchElementException e)
                {
                    // Java: throw new MathParseException(line, lineNumber);
                }
                // Java: catch (NumberFormatException e)
                {
                    // Java: throw new MathParseException(line, lineNumber);
                }
                // Java: lineNumber++;
            }
        }
        // Java: finally
        {
            // Java: reader.close();
        }
        // Java: return dim;
        todo!()
    }

    // Java: void initDirectionVector(final int d, final int a, final int[] m)
    pub fn initDirectionVector(&self) {
        // Java: final int s = m.length - 1;
        // Java: for (int i = 1; i <= s; i++)
        {
            // Java: direction[d][i] = ((long) m[i]) << (BITS - i);
        }
        // Java: for (int i = s + 1; i <= BITS; i++)
        {
            // Java: direction[d][i] = direction[d][i - s] ^ (direction[d][i - s] >> s);
            // Java: for (int k = 1; k <= s - 1; k++)
            {
                // Java: direction[d][i] ^= ((a >> (s - 1 - k)) & 1) * direction[d][i - k];
            }
        }
    }

    // Java: double[] nextVector()
    pub fn nextVector(&self) -> Vec<f64> {
        // Java: final double[] v = new double[dimension];
        // Java: if (count == 0)
        {
            // Java: count++;
            // Java: return v;
        }
        // Java: // find the index c of the rightmost 0
        // Java: int c = 1;
        // Java: int value = count - 1;
        // Java: while ((value & 1) == 1)
        {
            // Java: value >>= 1;
            // Java: c++;
        }
        // Java: for (int i = 0; i < dimension; i++)
        {
            // Java: x[i] ^= direction[i][c];
            // Java: v[i] = (double) x[i] / SCALE;
        }
        // Java: count++;
        // Java: return v;
        todo!()
    }

    // Java: double[] skipTo(final int index)
    pub fn skipTo(&self) -> Vec<f64> {
        // Java: if (index == 0)
        {
            // Java: // reset x vector
            // Java: Arrays.fill(x, 0);
        }
        // Java: else
        {
            // Java: final int i = index - 1;
            // Java: final long grayCode = i ^ (i >> 1); // compute the gray code of i = i XOR floor(i / 2)
            // Java: for (int j = 0; j < dimension; j++)
            {
                // Java: long result = 0;
                // Java: for (int k = 1; k <= BITS; k++)
                {
                    // Java: final long shift = grayCode >> (k - 1);
                    // Java: if (shift == 0)
                    {
                        // Java: // stop, as all remaining bits will be zero
                        // Java: break;
                    }
                    // Java: // the k-th bit of i
                    // Java: final long ik = shift & 1;
                    // Java: result ^= ik * direction[j][k];
                }
                // Java: x[j] = result;
            }
        }
        // Java: count = index;
        // Java: return nextVector();
        todo!()
    }

    // Java: int getNextIndex()
    pub fn getNextIndex(&self) -> i32 {
        // Java: return count;
        todo!()
    }

}

