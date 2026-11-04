// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Collections
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;

// Java: public class ListPopulation implements Population
pub struct ListPopulation {
    // Java field: List<Chromosome> chromosomes
    pub(crate) chromosomes: Vec<Box<dyn std::any::Any>>,
    // Java field: int populationLimit
    pub(crate) populationLimit: i32,
}

// Java: public class ListPopulation implements Population
impl ListPopulation {
    // Java: ListPopulation(final int populationLimit)
    pub fn new(population_limit: i32) -> Self {
        // Java: this(Collections.<Chromosome> emptyList(), populationLimit);
        ListPopulation { chromosomes: Vec::new(), populationLimit: population_limit }
    }

    // Java: ListPopulation(final List<Chromosome> chromosomes, final int populationLimit)
    pub fn new_with_chromosomes(chromosomes: Vec<Box<dyn std::any::Any>>, population_limit: i32) -> Self {
        ListPopulation { chromosomes, populationLimit: population_limit }
    }

    // Java: void setChromosomes(final List<Chromosome> chromosomes)
    pub fn setChromosomes(&mut self, chromosomes: Vec<Box<dyn std::any::Any>>) {
        self.chromosomes = chromosomes;
    }

    // Java: void addChromosomes(final Collection<Chromosome> chromosomeColl)
    pub fn addChromosomes(&mut self, _chromosome_coll: &dyn std::any::Any) {
        // Java: if (chromosomes.size() + chromosomeColl.size() > populationLimit)
        {
            // Java: throw new NumberIsTooLargeException(LocalizedFormats.LIST_OF_CHROMOSOMES_BIGGER_THAN_POPULATION_SIZE,
                                                // Java: chromosomes.size(), populationLimit, false);
        }
        // Java: this.chromosomes.addAll(chromosomeColl);
        todo!("ListPopulation::addChromosomes")
    }

    // Java: List<Chromosome> getChromosomes()
    pub fn getChromosomes(&self) -> &Vec<Box<dyn std::any::Any>> {
        // Java: return Collections.unmodifiableList(chromosomes);
        &self.chromosomes
    }

    // Java: List<Chromosome> getChromosomeList()
    pub fn getChromosomeList(&mut self) -> &mut Vec<Box<dyn std::any::Any>> {
        // Java: return chromosomes;
        &mut self.chromosomes
    }

    // Java: void addChromosome(final Chromosome chromosome)
    pub fn addChromosome(&mut self, chromosome: Box<dyn std::any::Any>) {
        // Java: if (chromosomes.size() >= populationLimit)
        if self.chromosomes.len() as i32 >= self.populationLimit {
            // Java: throw new NumberIsTooLargeException(LocalizedFormats.LIST_OF_CHROMOSOMES_BIGGER_THAN_POPULATION_SIZE,
                                                // Java: chromosomes.size(), populationLimit, false);
            panic!("List of chromosomes bigger than population size");
        }
        // Java: this.chromosomes.add(chromosome);
        self.chromosomes.push(chromosome);
    }

    // Java: Chromosome getFittestChromosome()
    pub fn getFittestChromosome(&self) -> &dyn std::any::Any {
        // Java: // best so far
        // Java: Chromosome bestChromosome = this.chromosomes.get(0);
        // Java: for (Chromosome chromosome : this.chromosomes)
        {
            // Java: if (chromosome.compareTo(bestChromosome) > 0)
            {
                // Java: // better chromosome found
                // Java: bestChromosome = chromosome;
            }
        }
        // Java: return bestChromosome;
        &*self.chromosomes[0]
    }

    // Java: int getPopulationLimit()
    pub fn getPopulationLimit(&self) -> i32 {
        // Java: return this.populationLimit;
        self.populationLimit
    }

    // Java: void setPopulationLimit(final int populationLimit)
    pub fn setPopulationLimit(&mut self, population_limit: i32) {
        // Java: if (populationLimit <= 0)
        if population_limit <= 0 {
            // Java: throw new NotPositiveException(LocalizedFormats.POPULATION_LIMIT_NOT_POSITIVE, populationLimit);
            panic!("Population limit not positive: {}", population_limit);
        }
        // Java: if (populationLimit < chromosomes.size())
        if population_limit < self.chromosomes.len() as i32 {
            // Java: throw new NumberIsTooSmallException(populationLimit, chromosomes.size(), true);
            panic!("Population limit too small: {}", population_limit);
        }
        // Java: this.populationLimit = populationLimit;
        self.populationLimit = population_limit;
    }

    // Java: int getPopulationSize()
    pub fn getPopulationSize(&self) -> i32 {
        // Java: return this.chromosomes.size();
        self.chromosomes.len() as i32
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return this.chromosomes.toString();
        format!("ListPopulation(size={})", self.chromosomes.len())
    }

    // Java: Iterator<Chromosome> iterator()
    pub fn iterator(&self) -> Box<dyn std::any::Any> {
        // Java: return getChromosomes().iterator();
        todo!("ListPopulation::iterator")
    }

}
