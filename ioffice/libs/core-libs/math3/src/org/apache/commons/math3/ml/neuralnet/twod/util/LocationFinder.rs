#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.Map
// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import org.apache.commons.math3.ml.neuralnet.Neuron
use crate::org::apache::commons::math3::ml::neuralnet::Neuron;
// Java: import org.apache.commons.math3.ml.neuralnet.twod.NeuronSquareMesh2D
use crate::org::apache::commons::math3::ml::neuralnet::twod::NeuronSquareMesh2D;
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;

use std::any::Any;

// Java: public class to
pub struct to {}

// Java: public class LocationFinder
pub struct LocationFinder {
    // Java field: Map<Long, Location> locations
    pub(crate) locations: HashMap<i64, Location>,
}

// Java: public class Location
#[derive(Clone)]
pub struct Location {
    // Java field: int row
    pub(crate) row: i32,
    // Java field: int column
    pub(crate) column: i32,
}

// Java: public class to
impl to {}

// Java: public class LocationFinder
impl LocationFinder {
    // Java: LocationFinder(NeuronSquareMesh2D map)
    pub fn new(_map: &dyn Any) -> Self {
        // Java: final int nR = map.getNumberOfRows();
        // Java: final int nC = map.getNumberOfColumns();
        // Java: for (int r = 0; r < nR; r++)
        // Java: for (int c = 0; c < nC; c++)
        // Java: final Long id = map.getNeuron(r, c).getIdentifier();
        // Java: if (locations.get(id) != null)
        // Java: throw new MathIllegalStateException();
        // Java: locations.put(id, new Location(r, c));
        LocationFinder {
            locations: HashMap::new(),
        }
    }

    // Java: Location getLocation(Neuron n)
    pub fn getLocation(&self, _n: &dyn Any) -> Option<&Location> {
        // Java: return locations.get(n.getIdentifier());
        todo!()
    }
}

// Java: public class Location
impl Location {
    // Java: Location()
    pub fn new(row: i32, column: i32) -> Self {
        Location { row, column }
    }

    // Java: int getRow()
    pub fn getRow(&self) -> i32 {
        // Java: return row;
        self.row
    }

    // Java: int getColumn()
    pub fn getColumn(&self) -> i32 {
        // Java: return column;
        self.column
    }
}
