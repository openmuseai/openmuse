// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java: public class RandomKeyMutation implements MutationPolicy
pub struct RandomKeyMutation {
}

// Java: public class RandomKeyMutation implements MutationPolicy
impl RandomKeyMutation {
    // Java: Chromosome mutate(final Chromosome original)
    pub fn mutate(&self, _original: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: if (!(original instanceof RandomKey<?>))
        {
            // Java: throw new MathIllegalArgumentException(LocalizedFormats.RANDOMKEY_MUTATION_WRONG_CLASS,
                                                   // Java: original.getClass().getSimpleName());
        }
        // Java: RandomKey<?> originalRk = (RandomKey<?>) original;
        // Java: List<Double> repr = originalRk.getRepresentation();
        // Java: int rInd = GeneticAlgorithm.getRandomGenerator().nextInt(repr.size());
        // Java: List<Double> newRepr = new ArrayList<Double> (repr);
        // Java: newRepr.set(rInd, GeneticAlgorithm.getRandomGenerator().nextDouble());
        // Java: return originalRk.newFixedLengthChromosome(newRepr);
        todo!("RandomKeyMutation::mutate")
    }

}
