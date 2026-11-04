#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.ArrayList
// (no direct Rust equivalent)
// Java: import java.util.Collection
// (no direct Rust equivalent)
// Java: import java.util.List
// (no direct Rust equivalent)
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Euclidean1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Euclidean1D;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Interval
// use crate::org::apache::commons::math3::geometry::euclidean::oned::Interval; // pending rewrite
// Java: import org.apache.commons.math3.geometry.euclidean.oned.IntervalsSet
use crate::org::apache::commons::math3::geometry::euclidean::oned::IntervalsSet;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Vector1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Vector1D;
// Java: import org.apache.commons.math3.geometry.partitioning.AbstractRegion
use crate::org::apache::commons::math3::geometry::partitioning::AbstractRegion;
// Java: import org.apache.commons.math3.geometry.partitioning.AbstractSubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::AbstractSubHyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTree
use crate::org::apache::commons::math3::geometry::partitioning::BSPTree;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTreeVisitor
use crate::org::apache::commons::math3::geometry::partitioning::BSPTreeVisitor;
// Java: import org.apache.commons.math3.geometry.partitioning.BoundaryAttribute
use crate::org::apache::commons::math3::geometry::partitioning::BoundaryAttribute;
// Java: import org.apache.commons.math3.geometry.partitioning.Hyperplane
use crate::org::apache::commons::math3::geometry::partitioning::Hyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.Side
use crate::org::apache::commons::math3::geometry::partitioning::Side;
// Java: import org.apache.commons.math3.geometry.partitioning.SubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::SubHyperplane;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java field: double DEFAULT_TOLERANCE
pub(crate) const DEFAULT_TOLERANCE: f64 = 1.0e-10;

// Java: public class represents
pub struct represents {
}

// Java: public class PolygonsSet
pub struct PolygonsSet {
    // Java field: Vector2D[][] vertices
    pub(crate) vertices: Option<Vec<Vec<Box<dyn std::any::Any>>>>,
    pub(crate) tolerance: f64,
}

// Java: public class Vertex
pub struct Vertex {
    // Java field: Vector2D location
    pub(crate) location: Box<dyn std::any::Any>,
    // Java field: Edge incoming
    pub(crate) incoming: Option<usize>,
    // Java field: Edge outgoing
    pub(crate) outgoing: Option<usize>,
    // Java field: List<Line> lines
    pub(crate) lines: Vec<Box<dyn std::any::Any>>,
}

// Java: public class Edge
pub struct Edge {
    // Java field: Vertex start
    pub(crate) start: usize,
    // Java field: Vertex end
    pub(crate) end: usize,
    // Java field: Line line
    pub(crate) line: Box<dyn std::any::Any>,
    // Java field: BSPTree<Euclidean2D> node
    pub(crate) node: Option<Box<dyn std::any::Any>>,
}

// Java: public class ConnectableSegment
pub struct ConnectableSegment {
    // Java field: BSPTree<Euclidean2D> node
    pub(crate) node: Box<dyn std::any::Any>,
    // Java field: BSPTree<Euclidean2D> startNode
    pub(crate) startNode: Box<dyn std::any::Any>,
    // Java field: BSPTree<Euclidean2D> endNode
    pub(crate) endNode: Box<dyn std::any::Any>,
    // Java field: ConnectableSegment previous
    pub(crate) previous: Option<usize>,
    // Java field: ConnectableSegment next
    pub(crate) next: Option<usize>,
    // Java field: boolean processed
    pub(crate) processed: bool,
}

// Java: public class SegmentsBuilder
pub struct SegmentsBuilder {
    // Java field: double tolerance
    pub(crate) tolerance: f64,
    // Java field: List<ConnectableSegment> segments
    pub(crate) segments: Vec<ConnectableSegment>,
}

// Java: public class represents
impl represents {
}

// Java: public class PolygonsSet
impl PolygonsSet {
    // Java: PolygonsSet(final double tolerance)
    pub fn new(tolerance: f64) -> Self {
        // Java: super(tolerance);
        PolygonsSet { vertices: None, tolerance }
    }

    // Java: PolygonsSet(final BSPTree<Euclidean2D> tree, final double tolerance)
    pub fn new_from_tree(_tree: &dyn std::any::Any, tolerance: f64) -> Self {
        // Java: super(tree, tolerance);
        PolygonsSet { vertices: None, tolerance }
    }

    // Java: PolygonsSet(final Collection<SubHyperplane<Euclidean2D>> boundary, final double tolerance)
    pub fn new_from_boundary(_boundary: &dyn std::any::Any, tolerance: f64) -> Self {
        // Java: super(boundary, tolerance);
        PolygonsSet { vertices: None, tolerance }
    }

    // Java: PolygonsSet()
    pub fn new_from_box(_xMin: f64, _xMax: f64, _yMin: f64, _yMax: f64, tolerance: f64) -> Self {
        PolygonsSet { vertices: None, tolerance }
    }

    // Java: PolygonsSet(final double hyperplaneThickness, final Vector2D ... vertices)
    pub fn new_from_vertices(_hyperplaneThickness: f64, _vertices: &[&dyn std::any::Any]) -> Self {
        // Java: super(verticesToTree(hyperplaneThickness, vertices), hyperplaneThickness);
        todo!()
    }

    // Java: PolygonsSet()
    pub fn new_default() -> Self {
        // Java: this(DEFAULT_TOLERANCE);
        Self::new(DEFAULT_TOLERANCE)
    }

    // Java: PolygonsSet(final BSPTree<Euclidean2D> tree)
    pub fn new_from_tree_default(_tree: &dyn std::any::Any) -> Self {
        // Java: this(tree, DEFAULT_TOLERANCE);
        todo!()
    }

    // Java: PolygonsSet(final Collection<SubHyperplane<Euclidean2D>> boundary)
    pub fn new_from_boundary_default(_boundary: &dyn std::any::Any) -> Self {
        // Java: this(boundary, DEFAULT_TOLERANCE);
        todo!()
    }

    // Java: PolygonsSet()
    pub fn new_from_box_default(_xMin: f64, _xMax: f64, _yMin: f64, _yMax: f64) -> Self {
        todo!()
    }

    // Java: static Line[] boxBoundary()
    pub fn boxBoundary(_xMin: f64, _xMax: f64, _yMin: f64, _yMax: f64, _tolerance: f64) -> Vec<Box<dyn std::any::Any>> {
        todo!()
    }

    // Java: static BSPTree<Euclidean2D> verticesToTree()
    pub fn verticesToTree(_hyperplaneThickness: f64, _vertices: &[&dyn std::any::Any]) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static void insertEdges()
    pub fn insertEdges(_hyperplaneThickness: f64, _tree: &dyn std::any::Any, _edges: &dyn std::any::Any) {
        todo!()
    }

    // Java: PolygonsSet buildNew(final BSPTree<Euclidean2D> tree)
    pub fn buildNew(&self, _tree: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return new PolygonsSet(tree, getTolerance());
        todo!()
    }

    // Java: void computeGeometricalProperties()
    pub fn computeGeometricalProperties(&mut self) {
        // Java: final Vector2D[][] v = getVertices();
        // Java: if (v.length == 0)
        // Java: final BSPTree<Euclidean2D> tree = getTree(false);
        // Java: if (tree.getCut() == null && (Boolean) tree.getAttribute())
        // Java: // the instance covers the whole space
        // Java: setSize(Double.POSITIVE_INFINITY);
        // Java: setBarycenter((Point<Euclidean2D>) Vector2D.NaN);
        // Java: else
        // Java: setSize(0);
        // Java: setBarycenter((Point<Euclidean2D>) new Vector2D(0, 0));
        // Java: else if (v[0][0] == null)
        // Java: // there is at least one open-loop: the polygon is infinite
        // Java: setSize(Double.POSITIVE_INFINITY);
        // Java: setBarycenter((Point<Euclidean2D>) Vector2D.NaN);
        // Java: else
        // Java: // all loops are closed, we compute some integrals around the shape
        // Java: double sum  = 0;
        // Java: double sumX = 0;
        // Java: double sumY = 0;
        // Java: for (Vector2D[] loop : v)
        // Java: double x1 = loop[loop.length - 1].getX();
        // Java: double y1 = loop[loop.length - 1].getY();
        // Java: for (final Vector2D point : loop)
        // Java: final double x0 = x1;
        // Java: final double y0 = y1;
        // Java: x1 = point.getX();
        // Java: y1 = point.getY();
        // Java: final double factor = x0 * y1 - y0 * x1;
        // Java: sum  += factor;
        // Java: sumX += factor * (x0 + x1);
        // Java: sumY += factor * (y0 + y1);
        // Java: if (sum < 0)
        // Java: // the polygon as a finite outside surrounded by an infinite inside
        // Java: setSize(Double.POSITIVE_INFINITY);
        // Java: setBarycenter((Point<Euclidean2D>) Vector2D.NaN);
        // Java: else
        // Java: setSize(sum / 2);
        // Java: setBarycenter((Point<Euclidean2D>) new Vector2D(sumX / (3 * sum), sumY / (3 * sum)));
        todo!()
    }

    // Java: Vector2D[][] getVertices()
    pub fn getVertices(&self) -> Box<dyn std::any::Any> {
        // Java: if (vertices == null)
        // Java: if (getTree(false).getCut() == null)
        // Java: vertices = new Vector2D[0][];
        // Java: else
        // Java: // build the unconnected segments
        // Java: final SegmentsBuilder visitor = new SegmentsBuilder(getTolerance());
        // Java: getTree(true).visit(visitor);
        // Java: final List<ConnectableSegment> segments = visitor.getSegments();
        // Java: // connect all segments, using topological criteria first
        // Java: // and using Euclidean distance only as a last resort
        // Java: int pending = segments.size();
        // Java: pending -= naturalFollowerConnections(segments);
        // Java: if (pending > 0)
        // Java: pending -= splitEdgeConnections(segments);
        // Java: if (pending > 0)
        // Java: pending -= closeVerticesConnections(segments);
        // Java: // create the segment loops
        // Java: final ArrayList<List<Segment>> loops = new ArrayList<List<Segment>>();
        // Java: for (ConnectableSegment s = getUnprocessed(segments); s != null; s = getUnprocessed(segments))
        // Java: final List<Segment> loop = followLoop(s);
        // Java: if (loop != null)
        // Java: if (loop.get(0).getStart() == null)
        // Java: // this is an open loop, we put it on the front
        // Java: loops.add(0, loop);
        // Java: else
        // Java: // this is a closed loop, we put it on the back
        // Java: loops.add(loop);
        // Java: // transform the loops in an array of arrays of points
        // Java: vertices = new Vector2D[loops.size()][];
        // Java: int i = 0;
        // Java: for (final List<Segment> loop : loops)
        // Java: if (loop.size() < 2 ||
        // Java: (loop.size() == 2 && loop.get(0).getStart() == null && loop.get(1).getEnd() == null))
        // Java: // single infinite line
        // Java: final Line line = loop.get(0).getLine();
        // Java: vertices[i++] = new Vector2D[]
        // Java: null,
        // Java: line.toSpace((Point<Euclidean1D>) new Vector1D(-Float.MAX_VALUE)),
        // Java: line.toSpace((Point<Euclidean1D>) new Vector1D(+Float.MAX_VALUE))
        // Java: else if (loop.get(0).getStart() == null)
        // Java: // open loop with at least one real point
        // Java: final Vector2D[] array = new Vector2D[loop.size() + 2];
        // Java: int j = 0;
        // Java: for (Segment segment : loop)
        // Java: if (j == 0)
        // Java: // null point and first dummy point
        // Java: double x = segment.getLine().toSubSpace((Point<Euclidean2D>) segment.getEnd()).getX();
        // Java: x -= FastMath.max(1.0, FastMath.abs(x / 2));
        // Java: array[j++] = null;
        // Java: array[j++] = segment.getLine().toSpace((Point<Euclidean1D>) new Vector1D(x));
        // Java: if (j < (array.length - 1))
        // Java: // current point
        // Java: array[j++] = segment.getEnd();
        // Java: if (j == (array.length - 1))
        // Java: // last dummy point
        // Java: double x = segment.getLine().toSubSpace((Point<Euclidean2D>) segment.getStart()).getX();
        // Java: x += FastMath.max(1.0, FastMath.abs(x / 2));
        // Java: array[j++] = segment.getLine().toSpace((Point<Euclidean1D>) new Vector1D(x));
        // Java: vertices[i++] = array;
        // Java: else
        // Java: final Vector2D[] array = new Vector2D[loop.size()];
        // Java: int j = 0;
        // Java: for (Segment segment : loop)
        // Java: array[j++] = segment.getStart();
        // Java: vertices[i++] = array;
        // Java: return vertices.clone();
        todo!()
    }

    // Java: int naturalFollowerConnections(final List<ConnectableSegment> segments)
    pub fn naturalFollowerConnections(&self, _segments: &mut Vec<ConnectableSegment>) -> i32 {
        // Java: int connected = 0;
        // Java: for (final ConnectableSegment segment : segments)
        // Java: if (segment.getNext() == null)
        // Java: final BSPTree<Euclidean2D> node = segment.getNode();
        // Java: final BSPTree<Euclidean2D> end  = segment.getEndNode();
        // Java: for (final ConnectableSegment candidateNext : segments)
        // Java: if (candidateNext.getPrevious()  == null &&
        // Java: candidateNext.getNode()      == end &&
        // Java: candidateNext.getStartNode() == node)
        // Java: // connect the two segments
        // Java: segment.setNext(candidateNext);
        // Java: candidateNext.setPrevious(segment);
        // Java: ++connected;
        // Java: break;
        // Java: return connected;
        todo!()
    }

    // Java: int splitEdgeConnections(final List<ConnectableSegment> segments)
    pub fn splitEdgeConnections(&self, _segments: &mut Vec<ConnectableSegment>) -> i32 {
        // Java: int connected = 0;
        // Java: for (final ConnectableSegment segment : segments)
        // Java: if (segment.getNext() == null)
        // Java: final Hyperplane<Euclidean2D> hyperplane = segment.getNode().getCut().getHyperplane();
        // Java: final BSPTree<Euclidean2D> end  = segment.getEndNode();
        // Java: for (final ConnectableSegment candidateNext : segments)
        // Java: if (candidateNext.getPrevious()                      == null &&
        // Java: candidateNext.getNode().getCut().getHyperplane() == hyperplane &&
        // Java: candidateNext.getStartNode()                     == end)
        // Java: // connect the two segments
        // Java: segment.setNext(candidateNext);
        // Java: candidateNext.setPrevious(segment);
        // Java: ++connected;
        // Java: break;
        // Java: return connected;
        todo!()
    }

    // Java: int closeVerticesConnections(final List<ConnectableSegment> segments)
    pub fn closeVerticesConnections(&self, _segments: &mut Vec<ConnectableSegment>) -> i32 {
        // Java: int connected = 0;
        // Java: for (final ConnectableSegment segment : segments)
        // Java: if (segment.getNext() == null && segment.getEnd() != null)
        // Java: final Vector2D end = segment.getEnd();
        // Java: ConnectableSegment selectedNext = null;
        // Java: double min = Double.POSITIVE_INFINITY;
        // Java: for (final ConnectableSegment candidateNext : segments)
        // Java: if (candidateNext.getPrevious() == null && candidateNext.getStart() != null)
        // Java: final double distance = Vector2D.distance(end, candidateNext.getStart());
        // Java: if (distance < min)
        // Java: selectedNext = candidateNext;
        // Java: min          = distance;
        // Java: if (min <= getTolerance())
        // Java: // connect the two segments
        // Java: segment.setNext(selectedNext);
        // Java: selectedNext.setPrevious(segment);
        // Java: ++connected;
        // Java: return connected;
        todo!()
    }

    // Java: ConnectableSegment getUnprocessed(final List<ConnectableSegment> segments)
    pub fn getUnprocessed(&self, _segments: &[ConnectableSegment]) -> Option<usize> {
        // Java: for (final ConnectableSegment segment : segments)
        // Java: if (!segment.isProcessed())
        // Java: return segment;
        // Java: return null;
        todo!()
    }

    // Java: List<Segment> followLoop(final ConnectableSegment defining)
    pub fn followLoop(&self, _defining: &ConnectableSegment) -> Option<Vec<Box<dyn std::any::Any>>> {
        // Java: final List<Segment> loop = new ArrayList<Segment>();
        // Java: loop.add(defining);
        // Java: defining.setProcessed(true);
        // Java: // add segments in connection order
        // Java: ConnectableSegment next = defining.getNext();
        // Java: while (next != defining && next != null)
        // Java: loop.add(next);
        // Java: next.setProcessed(true);
        // Java: next = next.getNext();
        // Java: if (next == null)
        // Java: // the loop is open and we have found its end,
        // Java: // we need to find its start too
        // Java: ConnectableSegment previous = defining.getPrevious();
        // Java: while (previous != null)
        // Java: loop.add(0, previous);
        // Java: previous.setProcessed(true);
        // Java: previous = previous.getPrevious();
        // Java: // filter out spurious vertices
        // Java: filterSpuriousVertices(loop);
        // Java: if (loop.size() == 2 && loop.get(0).getStart() != null)
        // Java: // this is a degenerated infinitely thin closed loop, we simply ignore it
        // Java: return null;
        // Java: else
        // Java: return loop;
        todo!()
    }

    // Java: void filterSpuriousVertices(final List<Segment> loop)
    pub fn filterSpuriousVertices(&self, _loop_segments: &mut Vec<Box<dyn std::any::Any>>) {
        // Java: for (int i = 0; i < loop.size(); ++i)
        // Java: final Segment previous = loop.get(i);
        // Java: int j = (i + 1) % loop.size();
        // Java: final Segment next = loop.get(j);
        // Java: if (next != null &&
        // Java: Precision.equals(previous.getLine().getAngle(), next.getLine().getAngle(), Precision.EPSILON))
        // Java: // the vertex between the two edges is a spurious one
        // Java: // replace the two segments by a single one
        // Java: loop.set(j, new Segment(previous.getStart(), next.getEnd(), previous.getLine()));
        // Java: loop.remove(i--);
        todo!()
    }

    pub fn getTolerance(&self) -> f64 {
        self.tolerance
    }

}

// Java: public class Vertex
impl Vertex {
    // Java: Vector2D getLocation()
    pub fn getLocation(&self) -> &dyn std::any::Any {
        // Java: return location;
        &*self.location
    }

    // Java: void bindWith(final Line line)
    pub fn bindWith(&mut self, line: Box<dyn std::any::Any>) {
        // Java: lines.add(line);
        self.lines.push(line);
    }

    // Java: Line sharedLineWith(final Vertex vertex)
    pub fn sharedLineWith(&self, _vertex: &Vertex) -> Option<&dyn std::any::Any> {
        // Java: for (final Line line1 : lines)
        // Java: for (final Line line2 : vertex.lines)
        // Java: if (line1 == line2)
        // Java: return line1;
        // Java: return null;
        todo!()
    }

    // Java: void setIncoming(final Edge incoming)
    pub fn setIncoming(&mut self, incoming: usize) {
        // Java: this.incoming = incoming;
        // Java: bindWith(incoming.getLine());
        self.incoming = Some(incoming);
    }

    // Java: Edge getIncoming()
    pub fn getIncoming(&self) -> Option<usize> {
        // Java: return incoming;
        self.incoming
    }

    // Java: void setOutgoing(final Edge outgoing)
    pub fn setOutgoing(&mut self, outgoing: usize) {
        // Java: this.outgoing = outgoing;
        // Java: bindWith(outgoing.getLine());
        self.outgoing = Some(outgoing);
    }

    // Java: Edge getOutgoing()
    pub fn getOutgoing(&self) -> Option<usize> {
        // Java: return outgoing;
        self.outgoing
    }

}

// Java: public class Edge
impl Edge {
    // Java: Vertex getStart()
    pub fn getStart(&self) -> usize {
        // Java: return start;
        self.start
    }

    // Java: Vertex getEnd()
    pub fn getEnd(&self) -> usize {
        // Java: return end;
        self.end
    }

    // Java: Line getLine()
    pub fn getLine(&self) -> &dyn std::any::Any {
        // Java: return line;
        &*self.line
    }

    // Java: void setNode(final BSPTree<Euclidean2D> node)
    pub fn setNode(&mut self, node: Box<dyn std::any::Any>) {
        // Java: this.node = node;
        self.node = Some(node);
    }

    // Java: BSPTree<Euclidean2D> getNode()
    pub fn getNode(&self) -> Option<&dyn std::any::Any> {
        // Java: return node;
        self.node.as_deref()
    }

    // Java: Vertex split(final Line splitLine)
    pub fn split(&self, _splitLine: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final Vertex splitVertex = new Vertex(line.intersection(splitLine));
        // Java: splitVertex.bindWith(splitLine);
        // Java: final Edge startHalf = new Edge(start, splitVertex, line);
        // Java: final Edge endHalf   = new Edge(splitVertex, end, line);
        // Java: startHalf.node = node;
        // Java: endHalf.node   = node;
        // Java: return splitVertex;
        todo!()
    }

}

// Java: public class ConnectableSegment
impl ConnectableSegment {
    // Java: BSPTree<Euclidean2D> getNode()
    pub fn getNode(&self) -> &dyn std::any::Any {
        // Java: return node;
        &*self.node
    }

    // Java: BSPTree<Euclidean2D> getStartNode()
    pub fn getStartNode(&self) -> &dyn std::any::Any {
        // Java: return startNode;
        &*self.startNode
    }

    // Java: BSPTree<Euclidean2D> getEndNode()
    pub fn getEndNode(&self) -> &dyn std::any::Any {
        // Java: return endNode;
        &*self.endNode
    }

    // Java: ConnectableSegment getPrevious()
    pub fn getPrevious(&self) -> Option<usize> {
        // Java: return previous;
        self.previous
    }

    // Java: void setPrevious(final ConnectableSegment previous)
    pub fn setPrevious(&mut self, previous: usize) {
        // Java: this.previous = previous;
        self.previous = Some(previous);
    }

    // Java: ConnectableSegment getNext()
    pub fn getNext(&self) -> Option<usize> {
        // Java: return next;
        self.next
    }

    // Java: void setNext(final ConnectableSegment next)
    pub fn setNext(&mut self, next: usize) {
        // Java: this.next = next;
        self.next = Some(next);
    }

    // Java: void setProcessed(final boolean processed)
    pub fn setProcessed(&mut self, processed: bool) {
        // Java: this.processed = processed;
        self.processed = processed;
    }

    // Java: boolean isProcessed()
    pub fn isProcessed(&self) -> bool {
        // Java: return processed;
        self.processed
    }

}

// Java: public class SegmentsBuilder
impl SegmentsBuilder {
    // Java: Order visitOrder(final BSPTree<Euclidean2D> node)
    pub fn visitOrder(&self, _node: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return Order.MINUS_SUB_PLUS;
        todo!()
    }

    // Java: void visitInternalNode(final BSPTree<Euclidean2D> node)
    pub fn visitInternalNode(&mut self, _node: &dyn std::any::Any) {
        // Java: @SuppressWarnings("unchecked")
        // Java: final BoundaryAttribute<Euclidean2D> attribute = (BoundaryAttribute<Euclidean2D>) node.getAttribute();
        // Java: final Iterable<BSPTree<Euclidean2D>> splitters = attribute.getSplitters();
        // Java: if (attribute.getPlusOutside() != null)
        // Java: addContribution(attribute.getPlusOutside(), node, splitters, false);
        // Java: if (attribute.getPlusInside() != null)
        // Java: addContribution(attribute.getPlusInside(), node, splitters, true);
        todo!()
    }

    // Java: void visitLeafNode(final BSPTree<Euclidean2D> node)
    pub fn visitLeafNode(&self, _node: &dyn std::any::Any) {
        // (empty body in Java)
    }

    // Java: void addContribution()
    pub fn addContribution(&mut self, _sub: &dyn std::any::Any, _node: &dyn std::any::Any, _splitters: &dyn std::any::Any, _reversed: bool) {
        todo!()
    }

    // Java: BSPTree<Euclidean2D> selectClosest(final Vector2D point, final Iterable<BSPTree<Euclidean2D>> candidates)
    pub fn selectClosest(&self, _point: &dyn std::any::Any, _candidates: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
        // Java: BSPTree<Euclidean2D> selected = null;
        // Java: double min = Double.POSITIVE_INFINITY;
        // Java: for (final BSPTree<Euclidean2D> node : candidates)
        // Java: final double distance = FastMath.abs(node.getCut().getHyperplane().getOffset(point));
        // Java: if (distance < min)
        // Java: selected = node;
        // Java: min      = distance;
        // Java: return min <= tolerance ? selected : null;
        todo!()
    }

    // Java: List<ConnectableSegment> getSegments()
    pub fn getSegments(&self) -> &Vec<ConnectableSegment> {
        // Java: return segments;
        &self.segments
    }

}
