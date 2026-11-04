#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.Collection
// Java: import org.apache.commons.math3.ml.neuralnet.Neuron
use crate::org::apache::commons::math3::ml::neuralnet::Neuron;
// Java: import org.apache.commons.math3.ml.neuralnet.Network
use crate::org::apache::commons::math3::ml::neuralnet::Network;
// Java: import org.apache.commons.math3.ml.neuralnet.twod.NeuronSquareMesh2D
use crate::org::apache::commons::math3::ml::neuralnet::twod::NeuronSquareMesh2D;
// Java: import org.apache.commons.math3.ml.distance.DistanceMeasure
use crate::org::apache::commons::math3::ml::distance::DistanceMeasure;

use std::any::Any;
use super::MapVisualization::MapVisualization;

// Java: public class UnifiedDistanceMatrix implements MapVisualization
pub struct UnifiedDistanceMatrix {
    // Java field: boolean individualDistances
    pub(crate) individualDistances: bool,
    // Java field: DistanceMeasure distance
    distance: Box<dyn Any>,
}

// Java: public class UnifiedDistanceMatrix implements MapVisualization
impl UnifiedDistanceMatrix {
    // Java: UnifiedDistanceMatrix()
    pub fn new(individual_distances: bool, distance: Box<dyn Any>) -> Self {
        // Java: this.individualDistances = individualDistances;
        // Java: this.distance = distance;
        UnifiedDistanceMatrix {
            individualDistances: individual_distances,
            distance,
        }
    }

    // Java: double[][] computeImage(NeuronSquareMesh2D map)
    pub fn computeImage_impl(&self, _map: &dyn Any) -> Vec<Vec<f64>> {
        // Java: if (individualDistances)
        if self.individualDistances {
            // Java: return individualDistances(map);
            self.individualDistances_compute(_map)
        }
        // Java: else
        else {
            // Java: return averageDistances(map);
            self.averageDistances(_map)
        }
    }

    // Java: double[][] individualDistances(NeuronSquareMesh2D map)
    pub fn individualDistances_compute(&self, _map: &dyn Any) -> Vec<Vec<f64>> {
        // Java: final int numRows = map.getNumberOfRows();
        // Java: final int numCols = map.getNumberOfColumns();
        // Java: final double[][] uMatrix = new double[numRows * 2 + 1][numCols * 2 + 1];
        // Java: // 1.
        // Java: // Fill right and bottom slots of each unit's location with the
        // Java: // distance between the current unit and each of the two neighbours,
        // Java: // respectively.
        // Java: for (int i = 0; i < numRows; i++)
        // Java: // Current unit's row index in result image.
        // Java: final int iR = 2 * i + 1;
        // Java: for (int j = 0; j < numCols; j++)
        // Java: // Current unit's column index in result image.
        // Java: final int jR = 2 * j + 1;
        // Java: final double[] current = map.getNeuron(i, j).getFeatures();
        // Java: Neuron neighbour;
        // Java: // Right neighbour.
        // Java: neighbour = map.getNeuron(i, j,
        // Java: NeuronSquareMesh2D.HorizontalDirection.RIGHT,
        // Java: NeuronSquareMesh2D.VerticalDirection.CENTER);
        // Java: if (neighbour != null)
        // Java: uMatrix[iR][jR + 1] = distance.compute(current,
        // Java: neighbour.getFeatures());
        // Java: // Bottom-center neighbour.
        // Java: neighbour = map.getNeuron(i, j,
        // Java: NeuronSquareMesh2D.HorizontalDirection.CENTER,
        // Java: NeuronSquareMesh2D.VerticalDirection.DOWN);
        // Java: if (neighbour != null)
        // Java: uMatrix[iR + 1][jR] = distance.compute(current,
        // Java: neighbour.getFeatures());
        // Java: // 2.
        // Java: // Fill the bottom-rigth slot of each unit's location with the average
        // Java: // of the distances between
        // Java: //  * the current unit and its bottom-right neighbour, and
        // Java: //  * the bottom-center neighbour and the right neighbour.
        // Java: for (int i = 0; i < numRows; i++)
        // Java: // Current unit's row index in result image.
        // Java: final int iR = 2 * i + 1;
        // Java: for (int j = 0; j < numCols; j++)
        // Java: // Current unit's column index in result image.
        // Java: final int jR = 2 * j + 1;
        // Java: final Neuron current = map.getNeuron(i, j);
        // Java: final Neuron right = map.getNeuron(i, j,
        // Java: NeuronSquareMesh2D.HorizontalDirection.RIGHT,
        // Java: NeuronSquareMesh2D.VerticalDirection.CENTER);
        // Java: final Neuron bottom = map.getNeuron(i, j,
        // Java: NeuronSquareMesh2D.HorizontalDirection.CENTER,
        // Java: NeuronSquareMesh2D.VerticalDirection.DOWN);
        // Java: final Neuron bottomRight = map.getNeuron(i, j,
        // Java: NeuronSquareMesh2D.HorizontalDirection.RIGHT,
        // Java: NeuronSquareMesh2D.VerticalDirection.DOWN);
        // Java: final double current2BottomRight = bottomRight == null ?
        // Java: 0 :
        // Java: distance.compute(current.getFeatures(),
        // Java: bottomRight.getFeatures());
        // Java: final double right2Bottom = (right == null ||
        // Java: bottom == null) ?
        // Java: 0 :
        // Java: distance.compute(right.getFeatures(),
        // Java: bottom.getFeatures());
        // Java: // Bottom-right slot.
        // Java: uMatrix[iR + 1][jR + 1] = 0.5 * (current2BottomRight + right2Bottom);
        // Java: // 3. Copy last row into first row.
        // Java: final int lastRow = uMatrix.length - 1;
        // Java: uMatrix[0] = uMatrix[lastRow];
        // Java: // 4.
        // Java: // Copy last column into first column.
        // Java: final int lastCol = uMatrix[0].length - 1;
        // Java: for (int r = 0; r < lastRow; r++)
        // Java: uMatrix[r][0] = uMatrix[r][lastCol];
        // Java: return uMatrix;
        todo!()
    }

    // Java: double[][] averageDistances(NeuronSquareMesh2D map)
    pub fn averageDistances(&self, _map: &dyn Any) -> Vec<Vec<f64>> {
        // Java: final int numRows = map.getNumberOfRows();
        // Java: final int numCols = map.getNumberOfColumns();
        // Java: final double[][] uMatrix = new double[numRows][numCols];
        // Java: final Network net = map.getNetwork();
        // Java: for (int i = 0; i < numRows; i++)
        // Java: for (int j = 0; j < numCols; j++)
        // Java: final Neuron neuron = map.getNeuron(i, j);
        // Java: final Collection<Neuron> neighbours = net.getNeighbours(neuron);
        // Java: final double[] features = neuron.getFeatures();
        // Java: double d = 0;
        // Java: int count = 0;
        // Java: for (Neuron n : neighbours)
        // Java: ++count;
        // Java: d += distance.compute(features, n.getFeatures());
        // Java: uMatrix[i][j] = d / count;
        // Java: return uMatrix;
        todo!()
    }
}

impl MapVisualization for UnifiedDistanceMatrix {
    // Java: abstract double[][] computeImage(NeuronSquareMesh2D map)
    fn computeImage(&self, map: &dyn Any) -> Vec<Vec<f64>> {
        self.computeImage_impl(map)
    }
}
