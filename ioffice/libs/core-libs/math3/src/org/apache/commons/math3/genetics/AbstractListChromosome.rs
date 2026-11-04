// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import java.util.Collections
// Java: import java.util.List

// Java: public class AbstractListChromosome extends Chromosome
pub struct AbstractListChromosome {
    // Java field: List<T> representation
    representation: Vec<Box<dyn std::any::Any>>,
}

// Java: public class AbstractListChromosome extends Chromosome
impl AbstractListChromosome {
    // Java: AbstractListChromosome(final List<T> representation)
    pub fn new(representation: Vec<Box<dyn std::any::Any>>) -> Self {
        // Java: this(representation, true);
        AbstractListChromosome { representation }
    }

    // Java: AbstractListChromosome(final T[] representation)
    pub fn new_from_array(representation: Vec<Box<dyn std::any::Any>>) -> Self {
        // Java: this(Arrays.asList(representation));
        Self::new(representation)
    }

    // Java: AbstractListChromosome(final List<T> representation, final boolean copyList)
    pub fn new_with_copy(representation: Vec<Box<dyn std::any::Any>>, _copy_list: bool) -> Self {
        // Java: checkValidity(representation);
        // Java: this.representation =
                // Java: Collections.unmodifiableList(copyList ? new ArrayList<T>(representation) : representation);
        AbstractListChromosome { representation }
    }

    // Java: abstract void checkValidity(List<T> chromosomeRepresentation)
    pub fn checkValidity(&self, _chromosome_representation: &dyn std::any::Any) {
        todo!("AbstractListChromosome::checkValidity")
    }

    // Java: List<T> getRepresentation()
    pub fn getRepresentation(&self) -> &Vec<Box<dyn std::any::Any>> {
        // Java: return representation;
        &self.representation
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
        // Java: return getRepresentation().size();
        self.representation.len() as i32
    }

    // Java: abstract AbstractListChromosome<T> newFixedLengthChromosome(final List<T> chromosomeRepresentation)
    pub fn newFixedLengthChromosome(&self, _chromosome_representation: Vec<Box<dyn std::any::Any>>) -> Box<dyn std::any::Any> {
        todo!("AbstractListChromosome::newFixedLengthChromosome")
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return String.format("(f=%s %s)", getFitness(), getRepresentation());
        format!("(representation len={})", self.representation.len())
    }

}
