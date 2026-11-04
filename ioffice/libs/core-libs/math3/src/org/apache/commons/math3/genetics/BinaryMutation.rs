// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java: public class BinaryMutation implements MutationPolicy
pub struct BinaryMutation {
}

// Java: public class BinaryMutation implements MutationPolicy
impl BinaryMutation {
    // Java: Chromosome mutate(Chromosome original)
    pub fn mutate(&self, _original: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: if (!(original instanceof BinaryChromosome))
        {
            // Java: throw new MathIllegalArgumentException(LocalizedFormats.INVALID_BINARY_CHROMOSOME);
        }
        // Java: BinaryChromosome origChrom = (BinaryChromosome) original;
        // Java: List<Integer> newRepr = new ArrayList<Integer>(origChrom.getRepresentation());
        // Java: // randomly select a gene
        // Java: int geneIndex = GeneticAlgorithm.getRandomGenerator().nextInt(origChrom.getLength());
        // Java: // and change it
        // Java: newRepr.set(geneIndex, origChrom.getRepresentation().get(geneIndex) == 0 ? 1 : 0);
        // Java: Chromosome newChrom = origChrom.newFixedLengthChromosome(newRepr);
        // Java: return newChrom;
        todo!("BinaryMutation::mutate")
    }

}
