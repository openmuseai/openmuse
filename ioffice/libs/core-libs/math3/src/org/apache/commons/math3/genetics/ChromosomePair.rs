use crate::org::apache::commons::math3::genetics::Chromosome;

// Java: public class ChromosomePair
pub struct ChromosomePair {
    // Java field: Chromosome first
    pub(crate) first: Chromosome::Chromosome,
    // Java field: Chromosome second
    pub(crate) second: Chromosome::Chromosome,
}

// Java: public class ChromosomePair
impl ChromosomePair {
    // Java: ChromosomePair(final Chromosome c1, final Chromosome c2)
    pub fn new(c1: Chromosome::Chromosome, c2: Chromosome::Chromosome) -> Self {
        // Java: super();
        // Java: first = c1;
        // Java: second = c2;
        ChromosomePair { first: c1, second: c2 }
    }

    // Java: Chromosome getFirst()
    pub fn getFirst(&self) -> &Chromosome::Chromosome {
        // Java: return first;
        &self.first
    }

    // Java: Chromosome getSecond()
    pub fn getSecond(&self) -> &Chromosome::Chromosome {
        // Java: return second;
        &self.second
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return String.format("(%s,%s)", getFirst(), getSecond());
        format!("(first, second)")
    }

}
