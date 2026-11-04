#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.io.Serializable
// Java: import java.io.ObjectInputStream
// Java: import org.apache.commons.math3.ml.neuralnet.Network
use crate::org::apache::commons::math3::ml::neuralnet::Network;
// Java: import org.apache.commons.math3.ml.neuralnet.FeatureInitializer
use crate::org::apache::commons::math3::ml::neuralnet::FeatureInitializer;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID_NeuronString: i64 = 2286812134405i64;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID_SerializationProxy: i64 = -3695875267187033778i64;

// Java: public class NeuronString implements Serializable
pub struct NeuronString {
    // Java field: Network network
    pub(crate) network: Box<dyn std::any::Any>,
    // Java field: int size
    pub(crate) size: i32,
    // Java field: boolean wrap
    pub(crate) wrap: bool,
    // Java field: long[] identifiers
    pub(crate) identifiers: Vec<i64>,
}

// Java: public class SerializationProxy
pub struct SerializationProxy {
    // Java field: boolean wrap
    pub(crate) wrap: bool,
    // Java field: double[][] featuresList
    pub(crate) featuresList: Vec<Vec<f64>>,
}

// Java: public class NeuronString implements Serializable
impl NeuronString {
    // Java: NeuronString()
    pub fn new(size: i32, wrap: bool, _featureInit: &[Box<dyn std::any::Any>]) -> Self {
        // Java: if (size < 2) throw new NumberIsTooSmallException(size, 2, true);
        if size < 2 {
            panic!("NumberIsTooSmallException: size {} is too small, minimum 2", size);
        }
        NeuronString {
            network: Box::new(0i32),
            size,
            wrap,
            identifiers: Vec::new(),
        }
    }

    // Java: Network getNetwork()
    pub fn getNetwork(&self) -> &dyn std::any::Any {
        // Java: return network;
        self.network.as_ref()
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return size;
        self.size
    }

    // Java: double[] getFeatures(int i)
    pub fn getFeatures(&self, i: i32) -> Vec<f64> {
        // Java: if (i < 0 ||
        // Java: i >= size)
        if i < 0 || i >= self.size {
            // Java: throw new OutOfRangeException(i, 0, size - 1);
            panic!("OutOfRangeException: {} not in [0, {}]", i, self.size - 1);
        }
        // Java: return network.getNeuron(identifiers[i]).getFeatures();
        todo!()
    }

    // Java: void createLinks()
    pub fn createLinks(&mut self) {
        // Java: for (int i = 0; i < size - 1; i++)
        // Java: network.addLink(network.getNeuron(i), network.getNeuron(i + 1));
        // Java: for (int i = size - 1; i > 0; i--)
        // Java: network.addLink(network.getNeuron(i), network.getNeuron(i - 1));
        // Java: if (wrap)
        // Java: network.addLink(network.getNeuron(0), network.getNeuron(size - 1));
        // Java: network.addLink(network.getNeuron(size - 1), network.getNeuron(0));
        todo!()
    }

    // Java: void readObject(ObjectInputStream in)
    pub fn readObject(&self) {
        // Java: throw new IllegalStateException();
        panic!("IllegalStateException");
    }

    // Java: Object writeReplace()
    pub fn writeReplace(&self) -> Box<dyn std::any::Any> {
        // Java: final double[][] featuresList = new double[size][];
        // Java: for (int i = 0; i < size; i++)
        // Java: featuresList[i] = getFeatures(i);
        // Java: return new SerializationProxy(wrap,
        // Java: featuresList);
        todo!()
    }

}

// Java: public class SerializationProxy
impl SerializationProxy {
    // Java: Object readResolve()
    pub fn readResolve(&self) -> Box<dyn std::any::Any> {
        // Java: return new NeuronString(wrap,
        // Java: featuresList);
        todo!()
    }

}
