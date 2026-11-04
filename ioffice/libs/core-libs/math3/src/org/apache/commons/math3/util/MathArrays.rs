#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut)]

use crate::org::apache::commons::math3::exception::NonMonotonicSequenceException;

// Java: public class MathArrays
pub struct MathArrays;

// Java: public enum OrderDirection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderDirection {
    Increasing,
    Decreasing,
}

// Java: public class PairDoubleInteger
pub struct PairDoubleInteger {
    // Java field: double key
    pub(crate) key: f64,
    // Java field: int value
    pub(crate) value: i32,
}

// Java: public enum Position
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Head,
    Tail,
}

// Java: public class MathArrays
impl MathArrays {
    // Java: MathArrays()
    pub fn new() -> Self {
        MathArrays
    }

    // Java: static double[] scale(double val, final double[] arr)
    pub fn scale(val: f64, arr: &[f64]) -> Vec<f64> {
        // Java: double[] newArr = new double[arr.length];
        let mut new_arr = vec![0.0; arr.len()];
        // Java: for (int i = 0; i < arr.length; i++)
        for i in 0..arr.len()
        {
            // Java: newArr[i] = arr[i] * val;
            new_arr[i] = arr[i] * val;
        }
        // Java: return newArr;
        new_arr
    }

    // Java: static void scaleInPlace(double val, final double[] arr)
    pub fn scaleInPlace(val: f64, arr: &mut [f64]) {
        // Java: for (int i = 0; i < arr.length; i++)
        for i in 0..arr.len()
        {
            // Java: arr[i] *= val;
            arr[i] *= val;
        }
    }

    // Java: static double[] ebeAdd(double[] a, double[] b)
    pub fn ebeAdd(a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
    }

    // Java: static double[] ebeSubtract(double[] a, double[] b)
    pub fn ebeSubtract(a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()
    }

    // Java: static double[] ebeMultiply(double[] a, double[] b)
    pub fn ebeMultiply(a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).collect()
    }

    // Java: static double[] ebeDivide(double[] a, double[] b)
    pub fn ebeDivide(a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter().zip(b.iter()).map(|(x, y)| x / y).collect()
    }

    // Java: static double distance1(double[] p1, double[] p2)
    pub fn distance1_f64(p1: &[f64], p2: &[f64]) -> f64 {
        p1.iter().zip(p2.iter()).map(|(a, b)| (a - b).abs()).sum()
    }

    // Java: static int distance1(int[] p1, int[] p2)
    pub fn distance1_i32(p1: &[i32], p2: &[i32]) -> i32 {
        p1.iter().zip(p2.iter()).map(|(a, b)| (a - b).abs()).sum()
    }

    // Java: static double distance(double[] p1, double[] p2)
    pub fn distance_f64(p1: &[f64], p2: &[f64]) -> f64 {
        p1.iter().zip(p2.iter()).map(|(a, b)| (a - b) * (a - b)).sum::<f64>().sqrt()
    }

    // Java: static double cosAngle(double[] v1, double[] v2)
    pub fn cosAngle(v1: &[f64], v2: &[f64]) -> f64 {
        // Java: return linearCombination(v1, v2) / (safeNorm(v1) * safeNorm(v2));
        Self::linearCombination_arr(v1, v2) / (Self::safeNorm(v1) * Self::safeNorm(v2))
    }

    // Java: static double distance(int[] p1, int[] p2)
    pub fn distance_i32(p1: &[i32], p2: &[i32]) -> f64 {
        p1.iter().zip(p2.iter()).map(|(a, b)| ((a - b) * (a - b)) as f64).sum::<f64>().sqrt()
    }

    // Java: static double distanceInf(double[] p1, double[] p2)
    pub fn distanceInf_f64(p1: &[f64], p2: &[f64]) -> f64 {
        p1.iter().zip(p2.iter()).map(|(a, b)| (a - b).abs()).fold(0.0_f64, f64::max)
    }

    // Java: static int distanceInf(int[] p1, int[] p2)
    pub fn distanceInf_i32(p1: &[i32], p2: &[i32]) -> i32 {
        p1.iter().zip(p2.iter()).map(|(a, b)| (a - b).abs()).max().unwrap_or(0)
    }

    // Java: static public static  <T extends Comparable<? super T>> boolean isMonotonic()
    pub fn isMonotonic_generic<T: PartialOrd>(_val: &[T], _dir: OrderDirection, _strict: bool) -> bool {
        todo!()
    }

    // Java: static boolean isMonotonic(double[] val, OrderDirection dir, boolean strict)
    pub fn isMonotonic(val: &[f64], dir: OrderDirection, strict: bool) -> bool {
        // Java: return checkOrder(val, dir, strict, false);
        Self::checkOrder_full(val, dir, strict, false)
    }

    // Java: static boolean checkEqualLength()
    pub fn checkEqualLength_bool_f64(a: &[f64], b: &[f64]) -> bool {
        a.len() == b.len()
    }

    // Java: static void checkEqualLength()
    pub fn checkEqualLength_f64(a: &[f64], b: &[f64]) {
        if a.len() != b.len() { panic!("DimensionMismatchException"); }
    }

    // Java: static boolean checkEqualLength()
    pub fn checkEqualLength_bool_i32(a: &[i32], b: &[i32]) -> bool {
        a.len() == b.len()
    }

    // Java: static void checkEqualLength()
    pub fn checkEqualLength_i32(a: &[i32], b: &[i32]) {
        if a.len() != b.len() { panic!("DimensionMismatchException"); }
    }

    // Java: static boolean checkOrder()
    pub fn checkOrder_full(val: &[f64], dir: OrderDirection, strict: bool, abort: bool) -> bool {
        for i in 1..val.len() {
            let ok = match (dir, strict) {
                (OrderDirection::Increasing, true) => val[i] > val[i - 1],
                (OrderDirection::Increasing, false) => val[i] >= val[i - 1],
                (OrderDirection::Decreasing, true) => val[i] < val[i - 1],
                (OrderDirection::Decreasing, false) => val[i] <= val[i - 1],
            };
            if !ok {
                if abort { panic!("NonMonotonicSequenceException"); }
                return false;
            }
        }
        true
    }

    // Java: static void checkOrder()
    pub fn checkOrder_dir(val: &[f64], dir: OrderDirection, strict: bool) {
        Self::checkOrder_full(val, dir, strict, true);
    }

    // Java: static void checkOrder(double[] val)
    pub fn checkOrder(val: &[f64]) {
        // Java: checkOrder(val, OrderDirection.INCREASING, true);
        Self::checkOrder_dir(val, OrderDirection::Increasing, true);
    }

    // Java: static void checkRectangular(final long[][] in)
    pub fn checkRectangular(arr: &[Vec<i64>]) {
        todo!()
    }

    // Java: static void checkPositive(final double[] in)
    pub fn checkPositive(arr: &[f64]) {
        todo!()
    }

    // Java: static void checkNotNaN(final double[] in)
    pub fn checkNotNaN(arr: &[f64]) {
        todo!()
    }

    // Java: static void checkNonNegative(final long[] in)
    pub fn checkNonNegative_1d(arr: &[i64]) {
        todo!()
    }

    // Java: static void checkNonNegative(final long[][] in)
    pub fn checkNonNegative_2d(arr: &[Vec<i64>]) {
        todo!()
    }

    // Java: static double safeNorm(double[] v)
    pub fn safeNorm(v: &[f64]) -> f64 {
        // Java: double rdwarf = 3.834e-20;
        let rdwarf = 3.834e-20_f64;
        // Java: double rgiant = 1.304e+19;
        let rgiant = 1.304e+19_f64;
        // Java: double s1 = 0;
        let mut s1 = 0.0_f64;
        // Java: double s2 = 0;
        let mut s2 = 0.0_f64;
        // Java: double s3 = 0;
        let mut s3 = 0.0_f64;
        // Java: double x1max = 0;
        let mut x1max = 0.0_f64;
        // Java: double x3max = 0;
        let mut x3max = 0.0_f64;
        // Java: double floatn = v.length;
        let floatn = v.len() as f64;
        // Java: double agiant = rgiant / floatn;
        let agiant = rgiant / floatn;
        // Java: for (int i = 0; i < v.length; i++)
        for i in 0..v.len()
        {
            // Java: double xabs = FastMath.abs(v[i]);
            let xabs = v[i].abs();
            // Java: if (xabs < rdwarf || xabs > agiant)
            if xabs < rdwarf || xabs > agiant
            {
                // Java: if (xabs > rdwarf)
                if xabs > rdwarf
                {
                    // Java: if (xabs > x1max)
                    if xabs > x1max
                    {
                        // Java: double r = x1max / xabs;
                        let r = x1max / xabs;
                        // Java: s1= 1 + s1 * r * r;
                        s1 = 1.0 + s1 * r * r;
                        // Java: x1max = xabs;
                        x1max = xabs;
                    }
                    // Java: else
                    else
                    {
                        // Java: double r = xabs / x1max;
                        let r = xabs / x1max;
                        // Java: s1 += r * r;
                        s1 += r * r;
                    }
                }
                // Java: else
                else
                {
                    // Java: if (xabs > x3max)
                    if xabs > x3max
                    {
                        // Java: double r = x3max / xabs;
                        let r = x3max / xabs;
                        // Java: s3= 1 + s3 * r * r;
                        s3 = 1.0 + s3 * r * r;
                        // Java: x3max = xabs;
                        x3max = xabs;
                    }
                    // Java: else
                    else
                    {
                        // Java: if (xabs != 0)
                        if xabs != 0.0
                        {
                            // Java: double r = xabs / x3max;
                            let r = xabs / x3max;
                            // Java: s3 += r * r;
                            s3 += r * r;
                        }
                    }
                }
            }
            // Java: else
            else
            {
                // Java: s2 += xabs * xabs;
                s2 += xabs * xabs;
            }
        }
        // Java: double norm;
        let norm: f64;
        // Java: if (s1 != 0)
        if s1 != 0.0
        {
            // Java: norm = x1max * Math.sqrt(s1 + (s2 / x1max) / x1max);
            norm = x1max * (s1 + (s2 / x1max) / x1max).sqrt();
        }
        // Java: else
        else
        {
            // Java: if (s2 == 0)
            if s2 == 0.0
            {
                // Java: norm = x3max * Math.sqrt(s3);
                norm = x3max * s3.sqrt();
            }
            // Java: else
            else
            {
                // Java: if (s2 >= x3max)
                if s2 >= x3max
                {
                    // Java: norm = Math.sqrt(s2 * (1 + (x3max / s2) * (x3max * s3)));
                    norm = (s2 * (1.0 + (x3max / s2) * (x3max * s3))).sqrt();
                }
                // Java: else
                else
                {
                    // Java: norm = Math.sqrt(x3max * ((s2 / x3max) + (x3max * s3)));
                    norm = (x3max * ((s2 / x3max) + (x3max * s3))).sqrt();
                }
            }
        }
        // Java: return norm;
        norm
    }

    // Java: static void sortInPlace(double[] x, double[] ... yList)
    pub fn sortInPlace_default(x: &mut [f64], y_list: &mut [&mut [f64]]) {
        todo!()
    }

    // Java: static void sortInPlace()
    pub fn sortInPlace_with_dir(x: &mut [f64], dir: OrderDirection, y_list: &mut [&mut [f64]]) {
        todo!()
    }

    // Java: static void sortInPlace()
    pub fn sortInPlace_internal() {
        todo!()
    }

    // Java: static int[] copyOf(int[] source)
    pub fn copyOf_i32(source: &[i32]) -> Vec<i32> {
         // Java: return copyOf(source, source.length);
         source.to_vec()
    }

    // Java: static double[] copyOf(double[] source)
    pub fn copyOf_f64(source: &[f64]) -> Vec<f64> {
         // Java: return copyOf(source, source.length);
         source.to_vec()
    }

    // Java: static int[] copyOf(int[] source, int len)
    pub fn copyOf_i32_len(source: &[i32], len: usize) -> Vec<i32> {
         // Java: final int[] output = new int[len];
         let mut output = vec![0i32; len];
         // Java: System.arraycopy(source, 0, output, 0, FastMath.min(len, source.length));
         let copy_len = len.min(source.len());
         output[..copy_len].copy_from_slice(&source[..copy_len]);
         // Java: return output;
         output
    }

    // Java: static double[] copyOf(double[] source, int len)
    pub fn copyOf_f64_len(source: &[f64], len: usize) -> Vec<f64> {
         // Java: final double[] output = new double[len];
         let mut output = vec![0.0f64; len];
         // Java: System.arraycopy(source, 0, output, 0, FastMath.min(len, source.length));
         let copy_len = len.min(source.len());
         output[..copy_len].copy_from_slice(&source[..copy_len]);
         // Java: return output;
         output
    }

    // Java: static double[] copyOfRange(double[] source, int from, int to)
    pub fn copyOfRange(source: &[f64], from: usize, to: usize) -> Vec<f64> {
        // Java: final int len = to - from;
        let len = to - from;
        // Java: final double[] output = new double[len];
        let mut output = vec![0.0f64; len];
        // Java: System.arraycopy(source, from, output, 0, FastMath.min(len, source.length - from));
        let copy_len = len.min(source.len().saturating_sub(from));
        output[..copy_len].copy_from_slice(&source[from..from + copy_len]);
        // Java: return output;
        output
    }

    // Java: static double linearCombination(final double[] a, final double[] b)
    pub fn linearCombination_arr(a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }

    // Java: static double linearCombination()
    pub fn linearCombination_2(a1: f64, b1: f64, a2: f64, b2: f64) -> f64 {
        a1 * b1 + a2 * b2
    }

    // Java: static double linearCombination()
    pub fn linearCombination_3(a1: f64, b1: f64, a2: f64, b2: f64, a3: f64, b3: f64) -> f64 {
        a1 * b1 + a2 * b2 + a3 * b3
    }

    // Java: static double linearCombination()
    pub fn linearCombination_4(a1: f64, b1: f64, a2: f64, b2: f64, a3: f64, b3: f64, a4: f64, b4: f64) -> f64 {
        a1 * b1 + a2 * b2 + a3 * b3 + a4 * b4
    }

    // Java: static boolean equals(float[] x, float[] y)
    pub fn equals_f32(x: &[f32], y: &[f32]) -> bool {
        // Java: if ((x == null) || (y == null))
        {
            // Java: return !((x == null) ^ (y == null));
        }
        // Java: if (x.length != y.length)
        if x.len() != y.len()
        {
            // Java: return false;
            return false;
        }
        // Java: for (int i = 0; i < x.length; ++i)
        for i in 0..x.len()
        {
            // Java: if (!Precision.equals(x[i], y[i]))
            if x[i] != y[i]
            {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: static boolean equalsIncludingNaN(float[] x, float[] y)
    pub fn equalsIncludingNaN_f32(x: &[f32], y: &[f32]) -> bool {
        // Java: if ((x == null) || (y == null))
        {
            // Java: return !((x == null) ^ (y == null));
        }
        // Java: if (x.length != y.length)
        if x.len() != y.len()
        {
            // Java: return false;
            return false;
        }
        // Java: for (int i = 0; i < x.length; ++i)
        for i in 0..x.len()
        {
            // Java: if (!Precision.equalsIncludingNaN(x[i], y[i]))
            if x[i] != y[i] && !(x[i].is_nan() && y[i].is_nan())
            {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: static boolean equals(double[] x, double[] y)
    pub fn equals_f64(x: &[f64], y: &[f64]) -> bool {
        // Java: if ((x == null) || (y == null))
        {
            // Java: return !((x == null) ^ (y == null));
        }
        // Java: if (x.length != y.length)
        if x.len() != y.len()
        {
            // Java: return false;
            return false;
        }
        // Java: for (int i = 0; i < x.length; ++i)
        for i in 0..x.len()
        {
            // Java: if (!Precision.equals(x[i], y[i]))
            if x[i] != y[i]
            {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: static boolean equalsIncludingNaN(double[] x, double[] y)
    pub fn equalsIncludingNaN_f64(x: &[f64], y: &[f64]) -> bool {
        // Java: if ((x == null) || (y == null))
        {
            // Java: return !((x == null) ^ (y == null));
        }
        // Java: if (x.length != y.length)
        if x.len() != y.len()
        {
            // Java: return false;
            return false;
        }
        // Java: for (int i = 0; i < x.length; ++i)
        for i in 0..x.len()
        {
            // Java: if (!Precision.equalsIncludingNaN(x[i], y[i]))
            if x[i] != y[i] && !(x[i].is_nan() && y[i].is_nan())
            {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: static double[] normalizeArray(double[] values, double normalizedSum)
    pub fn normalizeArray(values: &[f64], normalized_sum: f64) -> Vec<f64> {
        todo!()
    }

    // Java: static public static <T> T[] buildArray(final Field<T> field, final int length)
    pub fn buildArray_1d() {
        // Java: @SuppressWarnings("unchecked") // OK because field must be correct class
        // Java: T[] array = (T[]) Array.newInstance(field.getRuntimeClass(), length);
        // Java: Arrays.fill(array, field.getZero());
        // Java: return array;
        todo!()
    }

    // Java: static public static <T> T[][] buildArray(final Field<T> field, final int rows, final int columns)
    pub fn buildArray_2d() {
        // Java: final T[][] array;
        // Java: if (columns < 0)
        {
            // Java: T[] dummyRow = buildArray(field, 0);
            // Java: array = (T[][]) Array.newInstance(dummyRow.getClass(), rows);
        }
        // Java: else
        {
            // Java: array = (T[][]) Array.newInstance(field.getRuntimeClass(),
                                              // Java: new int[]
                                              {
                                                  // Java: rows, columns
                                              }
            // Java: for (int i = 0; i < rows; ++i)
            {
                // Java: Arrays.fill(array[i], field.getZero());
            }
        }
        // Java: return array;
        todo!()
    }

    // Java: static double[] convolve(double[] x, double[] h)
    pub fn convolve(x: &[f64], h: &[f64]) -> Vec<f64> {
        todo!()
    }

    // Java: static void shuffle()
    pub fn shuffle_full() {
        todo!()
    }

    // Java: static void shuffle()
    pub fn shuffle_with_rng() {
        todo!()
    }

    // Java: static void shuffle()
    pub fn shuffle_with_pos() {
        todo!()
    }

    // Java: static void shuffle(int[] list)
    pub fn shuffle(list: &mut [i32]) {
        // Java: shuffle(list, new Well19937c());
        todo!()
    }

    // Java: static int[] natural(int n)
    pub fn natural(n: i32) -> Vec<i32> {
        // Java: return sequence(n, 0, 1);
        Self::sequence(n, 0, 1)
    }

    // Java: static int[] sequence()
    pub fn sequence(size: i32, start: i32, stride: i32) -> Vec<i32> {
        (0..size).map(|i| start + i * stride).collect()
    }

    // Java: static boolean verifyValues(final double[] values, final int begin, final int length)
    pub fn verifyValues_basic(values: &[f64], begin: usize, length: usize) -> bool {
        begin + length <= values.len()
    }

    // Java: static boolean verifyValues()
    pub fn verifyValues_allow_empty(values: &[f64], begin: usize, length: usize, allow_empty: bool) -> bool {
        if allow_empty && length == 0 { return true; }
        begin + length <= values.len()
    }

    // Java: static boolean verifyValues()
    pub fn verifyValues_weighted(values: &[f64], weights: &[f64], begin: usize, length: usize) -> bool {
        begin + length <= values.len() && begin + length <= weights.len()
    }

    // Java: static boolean verifyValues()
    pub fn verifyValues_weighted_empty(values: &[f64], weights: &[f64], begin: usize, length: usize, allow_empty: bool) -> bool {
        if allow_empty && length == 0 { return true; }
        begin + length <= values.len() && begin + length <= weights.len()
    }

    // Java: static double[] concatenate(double[] ...x)
    pub fn concatenate(arrays: &[&[f64]]) -> Vec<f64> {
        // Java: int combinedLength = 0;
        let mut combined_length = 0usize;
        // Java: for (double[] a : x)
        for a in arrays {
            // Java: combinedLength += a.length;
            combined_length += a.len();
        }
        // Java: int offset = 0;
        let mut offset = 0usize;
        // Java: int curLength = 0;
        let mut cur_length: usize;
        // Java: final double[] combined = new double[combinedLength];
        let mut combined = vec![0.0f64; combined_length];
        // Java: for (int i = 0; i < x.length; i++)
        for i in 0..arrays.len() {
            // Java: curLength = x[i].length;
            cur_length = arrays[i].len();
            // Java: System.arraycopy(x[i], 0, combined, offset, curLength);
            combined[offset..offset + cur_length].copy_from_slice(arrays[i]);
            // Java: offset += curLength;
            offset += cur_length;
        }
        // Java: return combined;
        combined
    }

    // Java: static double[] unique(double[] data)
    pub fn unique(data: &[f64]) -> Vec<f64> {
        // Java: TreeSet<Double> values = new TreeSet<Double>();
        let mut values = std::collections::BTreeSet::new();
        // Java: for (int i = 0; i < data.length; i++)
        for i in 0..data.len()
        {
            // Java: values.add(data[i]);
            values.insert(data[i].to_bits());
        }
        // Java: final int count = values.size();
        let count = values.len();
        // Java: final double[] out = new double[count];
        let mut out = vec![0.0f64; count];
        // Java: Iterator<Double> iterator = values.iterator();
        let mut iterator = values.iter();
        // Java: int i = 0;
        let mut i = 0;
        // Java: while (iterator.hasNext())
        while let Some(&bits) = iterator.next()
        {
            // Java: out[count - ++i] = iterator.next();
            i += 1;
            out[count - i] = f64::from_bits(bits);
        }
        // Java: return out;
        out
    }
}

// Java: public interface Function
pub trait Function {
    // Java: abstract double evaluate(double[] array)
    fn evaluate(&self, array: &[f64]) -> f64;
    // Java: abstract double evaluate()
    fn evaluate_range(&self, array: &[f64], start_index: usize, num_elements: usize) -> f64;
}

impl PairDoubleInteger {
    pub fn new(key: f64, value: i32) -> Self {
        PairDoubleInteger { key, value }
    }

    // Java: double getKey()
    pub fn getKey(&self) -> f64 {
        // Java: return key;
        self.key
    }

    // Java: int getValue()
    pub fn getValue(&self) -> i32 {
        // Java: return value;
        self.value
    }
}
