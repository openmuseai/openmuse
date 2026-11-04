// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;

// Java: public class NodesSet extends Space implements Iterable
pub struct NodesSet {
    // Java field: List<BSPTree<S>> list
    pub(crate) list: Vec<Box<dyn std::any::Any>>,
}

// Java: public class NodesSet extends Space implements Iterable
impl NodesSet {
    // Java: NodesSet()
    pub fn NodesSet(&self) {
        // Java: list = new ArrayList<BSPTree<S>>();
    }

    // Java: void add(final BSPTree<S> node)
    pub fn add(&self) {
        // Java: for (final BSPTree<S> existing : list)
        {
            // Java: if (node == existing)
            {
                // Java: // the node is already known, don't add it
                // Java: return;
            }
        }
        // Java: // the node was not known, add it
        // Java: list.add(node);
    }

    // Java: void addAll(final Iterable<BSPTree<S>> iterator)
    pub fn addAll(&self) {
        // Java: for (final BSPTree<S> node : iterator)
        {
            // Java: add(node);
        }
    }

    // Java: Iterator<BSPTree<S>> iterator()
    pub fn iterator(&self) -> Box<dyn std::any::Any> {
        // Java: return list.iterator();
        todo!()
    }

}
