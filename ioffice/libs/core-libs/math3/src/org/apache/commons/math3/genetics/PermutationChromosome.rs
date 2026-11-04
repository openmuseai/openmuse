// Java: import java.util.List

// Java: public interface PermutationChromosome
pub trait PermutationChromosome {
    // Java: abstract List<T> decode(List<T> sequence)
    fn decode(&self, sequence: Vec<Box<dyn std::any::Any>>) -> Vec<Box<dyn std::any::Any>>;

}
