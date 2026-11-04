// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java: public class TournamentSelection implements SelectionPolicy
pub struct TournamentSelection {
    // Java field: int arity
    pub(crate) arity: i32,
}

// Java: public class TournamentSelection implements SelectionPolicy
impl TournamentSelection {
    // Java: TournamentSelection(final int arity)
    pub fn new(arity: i32) -> Self {
        // Java: this.arity = arity;
        TournamentSelection { arity }
    }

    // Java: ChromosomePair select(final Population population)
    pub fn select(&self, _population: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return new ChromosomePair(tournament((ListPopulation) population),
                                  // Java: tournament((ListPopulation) population));
        todo!("TournamentSelection::select")
    }

    // Java: Chromosome tournament(final ListPopulation population)
    pub fn tournament(&self, _population: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: if (population.getPopulationSize() < this.arity)
        {
            // Java: throw new MathIllegalArgumentException(LocalizedFormats.TOO_LARGE_TOURNAMENT_ARITY,
                                                   // Java: arity, population.getPopulationSize());
        }
        // Java: // auxiliary population
        // Java: ListPopulation tournamentPopulation = new ListPopulation(this.arity)
        {
            // Java: /** {@inheritDoc} */
            // Java: public Population nextGeneration()
            {
                // Java: // not useful here
                // Java: return null;
            }
        }
        // Java: // create a copy of the chromosome list
        // Java: List<Chromosome> chromosomes = new ArrayList<Chromosome> (population.getChromosomes());
        // Java: for (int i=0; i<this.arity; i++)
        {
            // Java: // select a random individual and add it to the tournament
            // Java: int rind = GeneticAlgorithm.getRandomGenerator().nextInt(chromosomes.size());
            // Java: tournamentPopulation.addChromosome(chromosomes.get(rind));
            // Java: // do not select it again
            // Java: chromosomes.remove(rind);
        }
        // Java: // the winner takes it all
        // Java: return tournamentPopulation.getFittestChromosome();
        todo!("TournamentSelection::tournament")
    }

    // Java: int getArity()
    pub fn getArity(&self) -> i32 {
        // Java: return arity;
        self.arity
    }

    // Java: void setArity(final int arity)
    pub fn setArity(&mut self, arity: i32) {
        // Java: this.arity = arity;
        self.arity = arity;
    }

}
