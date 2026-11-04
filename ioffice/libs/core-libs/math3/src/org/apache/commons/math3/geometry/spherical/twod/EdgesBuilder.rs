#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.ArrayList
// Java: import java.util.IdentityHashMap
// Java: import java.util.List
// Java: import java.util.Map
// Java: import org.apache.commons.math3.exception.MathIllegalStateException
use crate::org::apache::commons::math3::exception::MathIllegalStateException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.euclidean.threed.Vector3D
use crate::org::apache::commons::math3::geometry::euclidean::threed::Vector3D;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTree
use crate::org::apache::commons::math3::geometry::partitioning::BSPTree;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTreeVisitor
use crate::org::apache::commons::math3::geometry::partitioning::BSPTreeVisitor;
// Java: import org.apache.commons.math3.geometry.partitioning.BoundaryAttribute
use crate::org::apache::commons::math3::geometry::partitioning::BoundaryAttribute;
// Java: import org.apache.commons.math3.geometry.spherical.oned.Arc
use crate::org::apache::commons::math3::geometry::spherical::oned::Arc;
// Java: import org.apache.commons.math3.geometry.spherical.oned.ArcsSet
use crate::org::apache::commons::math3::geometry::spherical::oned::ArcsSet;
// Java: import org.apache.commons.math3.geometry.spherical.oned.S1Point
use crate::org::apache::commons::math3::geometry::spherical::oned::S1Point;

use std::collections::HashMap;

// Java: public class EdgesBuilder implements BSPTreeVisitor
pub struct EdgesBuilder {
    // Java field: BSPTree<Sphere2D> root
    pub(crate) root: Box<dyn std::any::Any>,
    // Java field: double tolerance
    pub(crate) tolerance: f64,
    // Java field: Map<Edge, BSPTree<Sphere2D>> edgeToNode
    pub(crate) edgeToNode: Vec<(Box<dyn std::any::Any>, Box<dyn std::any::Any>)>,
    // Java field: Map<BSPTree<Sphere2D>, List<Edge>> nodeToEdgesList
    pub(crate) nodeToEdgesList: Vec<(Box<dyn std::any::Any>, Vec<Box<dyn std::any::Any>>)>,
}

// Java: public class EdgesBuilder implements BSPTreeVisitor
impl EdgesBuilder {
    // Java: Order visitOrder(final BSPTree<Sphere2D> node)
    pub fn visitOrder(&self, _node: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return Order.MINUS_SUB_PLUS;
        todo!()
    }

    // Java: void visitInternalNode(final BSPTree<Sphere2D> node)
    pub fn visitInternalNode(&mut self, _node: &dyn std::any::Any) {
        // Java: nodeToEdgesList.put(node, new ArrayList<Edge>());
        // Java: @SuppressWarnings("unchecked")
        // Java: final BoundaryAttribute<Sphere2D> attribute = (BoundaryAttribute<Sphere2D>) node.getAttribute();
        // Java: if (attribute.getPlusOutside() != null)
        // Java: addContribution((SubCircle) attribute.getPlusOutside(), false, node);
        // Java: if (attribute.getPlusInside() != null)
        // Java: addContribution((SubCircle) attribute.getPlusInside(), true, node);
        todo!()
    }

    // Java: void visitLeafNode(final BSPTree<Sphere2D> node)
    pub fn visitLeafNode(&self, _node: &dyn std::any::Any) {
    }

    // Java: void addContribution()
    pub fn addContribution(&mut self) {
        todo!()
    }

    // Java: Edge getFollowingEdge(final Edge previous)
    pub fn getFollowingEdge(&self, _previous: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: List<Edge> getEdges()
    pub fn getEdges(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: // connect the edges
        // Java: for (final Edge previous : edgeToNode.keySet())
        // Java: previous.setNextEdge(getFollowingEdge(previous));
        // Java: return new ArrayList<Edge>(edgeToNode.keySet());
        todo!()
    }

}
