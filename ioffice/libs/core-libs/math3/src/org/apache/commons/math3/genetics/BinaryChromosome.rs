// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java: public class BinaryChromosome extends AbstractListChromosome
pub struct BinaryChromosome {
    pub(crate) representation: Vec<i32>,
}

// Java: public class BinaryChromosome extends AbstractListChromosome
impl BinaryChromosome {
    // Java: BinaryChromosome(List<Integer> representation)
    pub fn new(representation: Vec<i32>) -> Self {
        // Java: super(representation);
        BinaryChromosome { representation }
    }

    // Java: BinaryChromosome(Integer[] representation)
    pub fn new_from_array(representation: Vec<i32>) -> Self {
        // Java: super(representation);
        Self::new(representation)
    }

    // Java: void checkValidity(List<Integer> chromosomeRepresentation)
    pub fn checkValidity(&self, chromosome_representation: &[i32]) {
        // Java: for (int i : chromosomeRepresentation)
        for &i in chromosome_representation {
            // Java: if (i < 0 || i >1)
            if i < 0 || i > 1 {
                // Java: throw new InvalidRepresentationException(LocalizedFormats.INVALID_BINARY_DIGIT,
                                                         // Java: i);
                panic!("Invalid binary digit: {}", i);
            }
        }
    }

    // Java: static List<Integer> randomBinaryRepresentation(int length)
    pub fn randomBinaryRepresentation(length: i32) -> Vec<i32> {
        // Java: // random binary list
        // Java: List<Integer> rList= new ArrayList<Integer> (length);
        let mut r_list = Vec::with_capacity(length as usize);
        // Java: for (int j=0; j<length; j++)
        for _j in 0..length {
            // Java: rList.add(GeneticAlgorithm.getRandomGenerator().nextInt(2));
            r_list.push(0);
        }
        // Java: return rList;
        r_list
    }

    // Java: boolean isSame(Chromosome another)
    pub fn isSame(&self, _another: &dyn std::any::Any) -> bool {
        // Java: // type check
        // Java: if (! (another instanceof BinaryChromosome))
        {
            // Java: return false;
        }
        // Java: BinaryChromosome anotherBc = (BinaryChromosome) another;
        // Java: // size check
        // Java: if (getLength() != anotherBc.getLength())
        {
            // Java: return false;
        }
        // Java: for (int i=0; i< getRepresentation().size(); i++)
        {
            // Java: if (!(getRepresentation().get(i).equals(anotherBc.getRepresentation().get(i))))
            {
                // Java: return false;
            }
        }
        // Java: // all is ok
        // Java: return true;
        todo!("BinaryChromosome::isSame")
    }

}
