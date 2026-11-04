#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.ArrayList
// Java: import java.util.List

use std::any::Any;

// Java field: S2Point location
// Java field: Edge incoming
// Java field: Edge outgoing
// Java field: List<Circle> circles

// Java: public class Vertex
pub struct Vertex {
    pub location: Box<dyn Any>,
    pub incoming: Option<Box<dyn Any>>,
    pub outgoing: Option<Box<dyn Any>>,
    pub circles: Vec<Box<dyn Any>>,
}

// Java: public class Vertex
impl Vertex {
    // Java: Vertex(final S2Point location)
    pub fn new(location: Box<dyn Any>) -> Self {
        Vertex {
            location,
            incoming: None,
            outgoing: None,
            circles: Vec::new(),
        }
    }

    // Java: S2Point getLocation()
    pub fn getLocation(&self) -> &Box<dyn Any> {
        // Java: return location;
        &self.location
    }

    // Java: void bindWith(final Circle circle)
    pub fn bindWith(&mut self, circle: Box<dyn Any>) {
        // Java: circles.add(circle);
        self.circles.push(circle);
    }

    // Java: Circle sharedCircleWith(final Vertex vertex)
    pub fn sharedCircleWith(&self, _vertex: &Vertex) -> Option<&Box<dyn Any>> {
        // Java: for (final Circle circle1 : circles)
        // Java: for (final Circle circle2 : vertex.circles)
        // Java: if (circle1 == circle2)
        // Java: return circle1;
        // Java: return null;
        todo!()
    }

    // Java: void setIncoming(final Edge incoming)
    pub fn setIncoming(&mut self, incoming: Box<dyn Any>) {
        // Java: this.incoming = incoming;
        self.incoming = Some(incoming);
        // Java: bindWith(incoming.getCircle());
    }

    // Java: Edge getIncoming()
    pub fn getIncoming(&self) -> &Option<Box<dyn Any>> {
        // Java: return incoming;
        &self.incoming
    }

    // Java: void setOutgoing(final Edge outgoing)
    pub fn setOutgoing(&mut self, outgoing: Box<dyn Any>) {
        // Java: this.outgoing = outgoing;
        self.outgoing = Some(outgoing);
        // Java: bindWith(outgoing.getCircle());
    }

    // Java: Edge getOutgoing()
    pub fn getOutgoing(&self) -> &Option<Box<dyn Any>> {
        // Java: return outgoing;
        &self.outgoing
    }
}
