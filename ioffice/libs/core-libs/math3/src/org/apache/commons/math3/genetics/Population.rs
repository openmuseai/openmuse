// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;

// Java: public interface Population extends Iterable
pub trait Population {
    // Java: abstract int getPopulationSize()
    fn getPopulationSize(&self) -> i32;

    // Java: abstract int getPopulationLimit()
    fn getPopulationLimit(&self) -> i32;

    // Java: abstract Population nextGeneration()
    fn nextGeneration(&self) -> Box<dyn Population>;

    // Java: abstract void addChromosome(Chromosome chromosome)
    fn addChromosome(&mut self, chromosome: Box<dyn std::any::Any>);

    // Java: abstract Chromosome getFittestChromosome()
    fn getFittestChromosome(&self) -> Box<dyn std::any::Any>;

}
