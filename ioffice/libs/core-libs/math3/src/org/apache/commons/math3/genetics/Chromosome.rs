// Java field: double NO_FITNESS
pub(crate) const NO_FITNESS: f64 = f64::NEG_INFINITY;

// Java: public class Chromosome implements Comparable, Fitness
pub struct Chromosome {
    // Java field: double fitness
    pub(crate) fitness: f64,
}

// Java: public class Chromosome implements Comparable, Fitness
impl Chromosome {
    pub fn new() -> Self {
        Chromosome { fitness: NO_FITNESS }
    }

    // Java: double getFitness()
    pub fn getFitness(&self) -> f64 {
        // Java: if (this.fitness == NO_FITNESS)
        {
            // Java: // no cache - compute the fitness
            // Java: this.fitness = fitness();
        }
        // Java: return this.fitness;
        self.fitness
    }

    // Java: int compareTo(final Chromosome another)
    pub fn compareTo(&self, another: &Chromosome) -> i32 {
        // Java: return Double.compare(getFitness(), another.getFitness());
        self.getFitness().partial_cmp(&another.getFitness()).map_or(0, |o| o as i32)
    }

    // Java: boolean isSame(final Chromosome another)
    pub fn isSame(&self, _another: &Chromosome) -> bool {
        // Java: return false;
        false
    }

    // Java: Chromosome findSameChromosome(final Population population)
    pub fn findSameChromosome(&self, _population: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
        // Java: for (Chromosome anotherChr : population)
        {
            // Java: if (this.isSame(anotherChr))
            {
                // Java: return anotherChr;
            }
        }
        // Java: return null;
        None
    }

    // Java: void searchForFitnessUpdate(final Population population)
    pub fn searchForFitnessUpdate(&mut self, _population: &dyn std::any::Any) {
        // Java: Chromosome sameChromosome = findSameChromosome(population);
        // Java: if (sameChromosome != null)
        {
            // Java: fitness = sameChromosome.getFitness();
        }
    }

}
