#![allow(unused_imports, dead_code, unused_variables, non_snake_case, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.graphics.RectF
// use util::cn::wps::graphics::RectF;
// LLM No Rust equivalent: using local stub
// Java: import java.io.IOException
// LLM No Rust equivalent: IOException mapped to std::io::Error
// Java: import java.io.ObjectInput
// LLM No Rust equivalent: ObjectInput mapped to std::io::Read
// Java: import java.io.ObjectOutput
// LLM No Rust equivalent: ObjectOutput mapped to std::io::Write

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

pub struct Shape { _private: () }
impl Shape {
    fn new(_dc: Element) -> Self { Shape { _private: () } }
    fn setShapeType(&mut self, _t: i32) {}
    fn count(&self) -> i32 { 1 }
    fn isGroupShape(&self) -> bool { false }
    fn setParent(&mut self, _p: Element) {}
    fn getShapeID(&self) -> i32 { 0 }
    fn getZIndex(&self) -> i32 { 0 }
    fn clone(&self) -> Result<Shape, String> { Ok(Shape { _private: () }) }
    fn cloneAll(&self, _dc: Element) -> Result<Shape, String> { Ok(Shape { _private: () }) }
    fn cloneLightWeight(&self) -> Result<Shape, String> { Ok(Shape { _private: () }) }
    fn readExternal(&mut self, _input: &dyn Any) -> Result<(), String> { Ok(()) }
    fn writeExternal(&self, _output: &dyn Any) -> Result<(), String> { Ok(()) }
}

struct ShapeVector { items: Vec<Shape> }
impl ShapeVector {
    fn new() -> Self { ShapeVector { items: Vec::new() } }
    fn size(&self) -> i32 { self.items.len() as i32 }
    fn elementAt(&self, i: i32) -> Option<&Shape> { self.items.get(i as usize) }
    fn add(&mut self, shape: Shape) { self.items.push(shape); }
    fn remove(&mut self, i: i32) { if (i as usize) < self.items.len() { self.items.remove(i as usize); } }
    fn readExternal(&mut self, _input: &dyn Any, _dc: Element) {}
    fn writeExternal(&self, _output: &dyn Any) {}
}

struct RectF { left: f32, top: f32, right: f32, bottom: f32 }

struct Property { _private: () }
impl Property {
    fn setValue(&mut self, _id: i32, _val: RectF) {}
    fn getObject(&self, _id: i32) -> Option<RectF> { None }
}

// Java: public class GroupShape extends Shape
pub struct GroupShape {
    _shape: Shape,
    // Java field: ShapeVector childShapes
    childShapes: ShapeVector,
    mDrawingContainer: Element,
}

impl std::ops::Deref for GroupShape {
    type Target = Shape;
    fn deref(&self) -> &Shape {
        &self._shape
    }
}

impl std::ops::DerefMut for GroupShape {
    fn deref_mut(&mut self) -> &mut Shape {
        &mut self._shape
    }
}

// Java: public class GroupShape extends Shape
impl GroupShape {
    // Java: GroupShape(IDrawingContainer drawingContainer)
    pub fn new(drawingContainer: Element) -> Self {
      // Java: super(drawingContainer);
      // LLM JavaLine Aggregation: merged into struct initialization below
      // Java: this.setShapeType(0);
      // LLM JavaLine Aggregation: merged into struct initialization
      let mut s = GroupShape {
          _shape: Shape::new(drawingContainer.clone()),
          childShapes: ShapeVector::new(),
          mDrawingContainer: drawingContainer,
      };
      s.setShapeType(0);
      s
    }

    // Java: int getChildCount()
    pub fn getChildCount(&self) -> i32 {
      // Java: return this.childShapes.size();
      self.childShapes.size()
    }

    // Java: int count()
    pub fn count(&self) -> i32 {
      // Java: int count = 1;
      let mut count = 1;
      // Java: for(Shape sp : this.childShapes)
      for sp in &self.childShapes.items {
         // Java: count += sp.count();
         count += sp.count();
      }
      // Java: return count;
      count
    }

    // Java: Shape getChildShape(int n)
    pub fn getChildShape(&self, n: i32) -> Option<&Shape> {
      // Java: return n >= 0 && n < this.getChildCount() ? (Shape)this.childShapes.elementAt(n) : null;
      if n >= 0 && n < self.getChildCount() {
          self.childShapes.elementAt(n)
      } else {
          None
      }
    }

    // Java: Shape getLastChildShape()
    pub fn getLastChildShape(&self) -> Option<&Shape> {
      // Java: Shape sp = (Shape)this.childShapes.elementAt(this.getChildCount() - 1);
      let sp = self.childShapes.elementAt(self.getChildCount() - 1);
      // Java: return sp.isGroupShape() ? ((GroupShape)sp).getLastChildShape() : sp;
      // LLM No Rust equivalent: TODO requires downcasting Shape to GroupShape
      sp
    }

    // Java: void addChildShape(Shape shape)
    pub fn addChildShape(&mut self, mut shape: Shape) {
      // Java: shape.setParent(this);
      shape.setParent(None);
      // Java: this.childShapes.add(shape);
      self.childShapes.add(shape);
    }

    // Java: void setCoordBounds(RectF rectF)
    pub fn setCoordBounds(&mut self, rectF: RectF) {
      // Java: this.getProperty().setValue(733, rectF);
      // LLM No Rust equivalent: TODO requires Property from Shape
      todo!("requires getProperty from Shape");
    }

    // Java: RectF getCoordBounds()
    pub fn getCoordBounds(&self) -> Option<RectF> {
      // Java: return (RectF)this.getProperty().getObject(733);
      // LLM No Rust equivalent: TODO requires Property from Shape
      todo!("requires getProperty from Shape")
    }

    // Java: GroupShape clone()
    pub fn clone(&self) -> Result<GroupShape, String> {
      // Java: GroupShape groupShape = (GroupShape)super.clone();
      let mut groupShape = GroupShape::new(self.mDrawingContainer.clone());
      // Java: groupShape.childShapes = new ShapeVector();
      groupShape.childShapes = ShapeVector::new();
      // Java: for(Shape shape : this.childShapes)
      for _shape in &self.childShapes.items {
         // Java: groupShape.childShapes.add(shape);
         // LLM No Rust equivalent: TODO requires cloning Shape
      }
      // Java: return groupShape;
      Ok(groupShape)
    }

    // Java: GroupShape cloneAll()
    pub fn cloneAll(&self) -> Result<GroupShape, String> {
      // Java: return this.cloneAll(this.mDrawingContainer);
      // LLM: self.cloneAll() overload resolved to cloneAll_2
      self.cloneAll_2(self.mDrawingContainer.clone())
    }

    // Java: GroupShape cloneAll(IDrawingContainer drawingContainer)
    pub fn cloneAll_2(&self, drawingContainer: Element) -> Result<GroupShape, String> {
      // Java: GroupShape groupShape = (GroupShape)super.cloneAll(drawingContainer);
      let mut groupShape = GroupShape::new(drawingContainer.clone());
      // Java: groupShape.childShapes = new ShapeVector();
      groupShape.childShapes = ShapeVector::new();
      // Java: for(Shape shape : this.childShapes)
      for _shape in &self.childShapes.items {
         // Java: groupShape.addChildShape(shape.cloneAll(drawingContainer));
         // LLM No Rust equivalent: TODO requires Shape.cloneAll
      }
      // Java: return groupShape;
      Ok(groupShape)
    }

    // Java: Shape cloneLightWeight()
    pub fn cloneLightWeight(&self) -> Result<GroupShape, String> {
      // Java: GroupShape groupShape = (GroupShape)super.cloneLightWeight();
      let mut groupShape = GroupShape::new(self.mDrawingContainer.clone());
      // Java: groupShape.childShapes = new ShapeVector();
      groupShape.childShapes = ShapeVector::new();
      // Java: for(Shape shape : this.childShapes)
      for _shape in &self.childShapes.items {
         // Java: groupShape.childShapes.add(shape);
         // LLM No Rust equivalent: TODO requires cloning Shape
      }
      // Java: return groupShape;
      Ok(groupShape)
    }

    // Java: RectF getRefreshRect(RectF rectF)
    pub fn getRefreshRect(&self, rectF: RectF) -> RectF {
      // Java: return rectF;
      rectF
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &dyn Any) -> Result<(), String> {
      // Java: this.childShapes.readExternal(input, this.mDrawingContainer);
      self.childShapes.readExternal(input, self.mDrawingContainer.clone());
      // Java: super.readExternal(input);
      self._shape.readExternal(input)?;
      Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &dyn Any) -> Result<(), String> {
      // Java: this.childShapes.writeExternal(output);
      self.childShapes.writeExternal(output);
      // Java: super.writeExternal(output);
      self._shape.writeExternal(output)?;
      Ok(())
    }

    // Java: Shape cloneToShape()
    pub fn cloneToShape(&self) -> Result<Shape, String> {
      // Java: return super.clone();
      self._shape.clone()
    }

    // Java: boolean removeChildShape(Shape shape)
    pub fn removeChildShape(&mut self, shape: Option<&Shape>) -> bool {
      // Java: if (shape == null)
      if shape.is_none() {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         let shape = shape.unwrap();
         // Java: int size = this.childShapes.size();
         let size = self.childShapes.size();
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size {
            // Java: Shape coreShape = (Shape)this.childShapes.elementAt(i);
            let coreShape = self.childShapes.elementAt(i);
            let core_id = coreShape.map(|s| s.getShapeID()).unwrap_or(-1);
            // Java: if (coreShape.getShapeID() == shape.getShapeID())
            if core_id == shape.getShapeID() {
               // Java: this.childShapes.remove(i);
               self.childShapes.remove(i);
               // Java: coreShape.setParent((Shape)null);
               // LLM No Rust equivalent: TODO already removed from vector
               // Java: return true;
               return true;
            }
         }
         // Java: return false;
         return false;
      }
    }

}

// Java: public interface OnDeleteEventListener
pub struct OnDeleteEventListener;

// Java: public interface OnDeleteEventListener
pub trait OnDeleteEventListenerTrait {
    // Java: void notifyDelEvent(Shape var1)
    fn notifyDelEvent(&self, var1: &dyn Any);
}
