// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.Collections
// Java: import java.util.Comparator
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java: public class RandomKey extends AbstractListChromosome implements PermutationChromosome
pub struct RandomKey {
    // Java field: List<Double> sortedRepresentation
    sortedRepresentation: Vec<f64>,
    // Java field: List<Integer> baseSeqPermutation
    baseSeqPermutation: Vec<i32>,
    pub(crate) representation: Vec<f64>,
}

// Java: public class RandomKey extends AbstractListChromosome implements PermutationChromosome
impl RandomKey {
    // Java: RandomKey(final List<Double> representation)
    pub fn new(representation: Vec<f64>) -> Self {
        // Java: super(representation);
        // Java: // store the sorted representation
        // Java: List<Double> sortedRepr = new ArrayList<Double> (getRepresentation());
        let mut sorted_repr = representation.clone();
        // Java: Collections.sort(sortedRepr);
        sorted_repr.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        // Java: sortedRepresentation = Collections.unmodifiableList(sortedRepr);
        let sorted_representation = sorted_repr;
        // Java: // store the permutation of [0,1,...,n-1] list for toString() and isSame() methods
        // Java: baseSeqPermutation = Collections.unmodifiableList(
            // Java: decodeGeneric(baseSequence(getLength()), getRepresentation(), sortedRepresentation)
        // Java: );
        let len = representation.len() as i32;
        let base_seq = Self::baseSequence_static(len);
        let base_seq_permutation = Self::decodeGeneric_static(&base_seq, &representation, &sorted_representation);
        RandomKey {
            sortedRepresentation: sorted_representation,
            baseSeqPermutation: base_seq_permutation,
            representation,
        }
    }

    // Java: RandomKey(final Double[] representation)
    pub fn new_from_array(representation: &[f64]) -> Self {
        // Java: this(Arrays.asList(representation));
        Self::new(representation.to_vec())
    }

    // Java: List<T> decode(final List<T> sequence)
    pub fn decode(&self, _sequence: Vec<Box<dyn std::any::Any>>) -> Vec<Box<dyn std::any::Any>> {
        // Java: return decodeGeneric(sequence, getRepresentation(), sortedRepresentation);
        todo!("RandomKey::decode")
    }

    // Java: static private static <S> List<S> decodeGeneric()
    fn decodeGeneric_static(base_seq: &[i32], _representation: &[f64], _sorted_representation: &[f64]) -> Vec<i32> {
        todo!("RandomKey::decodeGeneric")
    }

    // Java: boolean isSame(final Chromosome another)
    pub fn isSame(&self, another: &RandomKey) -> bool {
        // Java: // type check
        // Java: if (! (another instanceof RandomKey<?>))
        {
            // Java: return false;
        }
        // Java: RandomKey<?> anotherRk = (RandomKey<?>) another;
        // Java: // size check
        // Java: if (getLength() != anotherRk.getLength())
        if self.getLength() != another.getLength()
        {
            // Java: return false;
            return false;
        }
        // Java: // two different representations can still encode the same permutation
        // Java: // the ordering is what counts
        // Java: List<Integer> thisPerm = this.baseSeqPermutation;
        let this_perm = &self.baseSeqPermutation;
        // Java: List<Integer> anotherPerm = anotherRk.baseSeqPermutation;
        let another_perm = &another.baseSeqPermutation;
        // Java: for (int i=0; i<getLength(); i++)
        for i in 0..self.getLength() as usize
        {
            // Java: if (thisPerm.get(i) != anotherPerm.get(i))
            if this_perm[i] != another_perm[i]
            {
                // Java: return false;
                return false;
            }
        }
        // Java: // the permutations are the same
        // Java: return true;
        true
    }

    // Java: void checkValidity(final List<Double> chromosomeRepresentation)
    pub fn checkValidity(_chromosome_representation: &[f64]) {
        // Java: for each value, check it's in [0, 1]
    }

    // Java: static List<Double> randomPermutation(final int l)
    pub fn randomPermutation(l: i32) -> Vec<f64> {
        // Java: List<Double> repr = new ArrayList<Double>(l);
        let mut repr = Vec::with_capacity(l as usize);
        // Java: for (int i=0; i<l; i++)
        for _i in 0..l {
            // Java: repr.add(GeneticAlgorithm.getRandomGenerator().nextDouble());
            repr.push(rand_f64());
        }
        // Java: return repr;
        repr
    }

    // Java: static List<Double> identityPermutation(final int l)
    pub fn identityPermutation(l: i32) -> Vec<f64> {
        // Java: List<Double> repr = new ArrayList<Double>(l);
        let mut repr = Vec::with_capacity(l as usize);
        // Java: for (int i=0; i<l; i++)
        for i in 0..l {
            // Java: repr.add((double)i/l);
            repr.push(i as f64 / l as f64);
        }
        // Java: return repr;
        repr
    }

    // Java: static public static <S> List<Double> comparatorPermutation()
    pub fn comparatorPermutation(_data: &[Box<dyn std::any::Any>]) -> Vec<f64> {
        todo!("RandomKey::comparatorPermutation")
    }

    // Java: static public static <S> List<Double> inducedPermutation()
    pub fn inducedPermutation(_original_data: &[Box<dyn std::any::Any>], _permuted_data: &[Box<dyn std::any::Any>]) -> Vec<f64> {
        todo!("RandomKey::inducedPermutation")
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return String.format("(f=%s pi=(%s))", getFitness(), baseSeqPermutation);
        format!("(pi=({:?}))", self.baseSeqPermutation)
    }

    // Java: static List<Integer> baseSequence(final int l)
    fn baseSequence_static(l: i32) -> Vec<i32> {
        // Java: List<Integer> baseSequence = new ArrayList<Integer> (l);
        let mut base_sequence = Vec::with_capacity(l as usize);
        // Java: for (int i=0; i<l; i++)
        for i in 0..l {
            // Java: baseSequence.add(i);
            base_sequence.push(i);
        }
        // Java: return baseSequence;
        base_sequence
    }

    pub fn getLength(&self) -> i32 {
        self.representation.len() as i32
    }

    pub fn getRepresentation(&self) -> &[f64] {
        &self.representation
    }

}

fn rand_f64() -> f64 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (t as f64) / (u32::MAX as f64)
}
