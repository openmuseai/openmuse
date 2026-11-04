// Java: import java.util.Collections
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class ElitisticListPopulation extends ListPopulation
pub struct ElitisticListPopulation {
    // Java field: double elitismRate
    pub(crate) elitismRate: f64,
    pub(crate) population_limit: i32,
    pub(crate) chromosomes: Vec<Box<dyn std::any::Any>>,
}

// Java: public class ElitisticListPopulation extends ListPopulation
impl ElitisticListPopulation {
    // Java: ElitisticListPopulation()
    pub fn new(chromosomes: Vec<Box<dyn std::any::Any>>, population_limit: i32, elitism_rate: f64) -> Self {
        ElitisticListPopulation { elitismRate: elitism_rate, population_limit, chromosomes }
    }

    // Java: ElitisticListPopulation(final int populationLimit, final double elitismRate)
    pub fn new_with_limit(population_limit: i32, elitism_rate: f64) -> Self {
        ElitisticListPopulation { elitismRate: elitism_rate, population_limit, chromosomes: Vec::new() }
    }

    // Java: Population nextGeneration()
    pub fn nextGeneration(&self) -> Box<dyn std::any::Any> {
        // Java: // initialize a new generation with the same parameters
        // Java: ElitisticListPopulation nextGeneration =
                // Java: new ElitisticListPopulation(getPopulationLimit(), getElitismRate());
        // Java: final List<Chromosome> oldChromosomes = getChromosomeList();
        // Java: Collections.sort(oldChromosomes);
        // Java: // index of the last "not good enough" chromosome
        // Java: int boundIndex = (int) FastMath.ceil((1.0 - getElitismRate()) * oldChromosomes.size());
        // Java: for (int i = boundIndex; i < oldChromosomes.size(); i++)
        {
            // Java: nextGeneration.addChromosome(oldChromosomes.get(i));
        }
        // Java: return nextGeneration;
        todo!("ElitisticListPopulation::nextGeneration")
    }

    // Java: void setElitismRate(final double elitismRate)
    pub fn setElitismRate(&mut self, elitism_rate: f64) {
        // Java: if (elitismRate < 0 || elitismRate > 1)
        if elitism_rate < 0.0 || elitism_rate > 1.0 {
            // Java: throw new OutOfRangeException(LocalizedFormats.ELITISM_RATE, elitismRate, 0, 1);
            panic!("Elitism rate out of range: {}", elitism_rate);
        }
        // Java: this.elitismRate = elitismRate;
        self.elitismRate = elitism_rate;
    }

    // Java: double getElitismRate()
    pub fn getElitismRate(&self) -> f64 {
        // Java: return this.elitismRate;
        self.elitismRate
    }

}
