#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.Iterator
// Java: import org.apache.commons.math3.ml.neuralnet.Network
use crate::org::apache::commons::math3::ml::neuralnet::Network;

use super::KohonenUpdateAction::KohonenUpdateAction;

// Java: public class KohonenTrainingTask implements Runnable
pub struct KohonenTrainingTask {
    // Java field: Network net
    pub(crate) net: Box<dyn std::any::Any>,
    // Java field: Iterator<double[]> featuresIterator
    pub(crate) featuresIterator: Box<dyn std::any::Any>,
    // Java field: KohonenUpdateAction updateAction
    pub(crate) updateAction: Box<dyn std::any::Any>,
}

// Java: public class KohonenTrainingTask implements Runnable
impl KohonenTrainingTask {
    // Java: KohonenTrainingTask()
    pub fn new(
        net: Box<dyn std::any::Any>,
        featuresIterator: Box<dyn std::any::Any>,
        updateAction: Box<dyn std::any::Any>,
    ) -> Self {
        KohonenTrainingTask {
            net,
            featuresIterator,
            updateAction,
        }
    }

    // Java: void run()
    pub fn run(&self) {
        // Java: while (featuresIterator.hasNext())
        // Java: updateAction.update(net, featuresIterator.next());
        todo!()
    }

}
