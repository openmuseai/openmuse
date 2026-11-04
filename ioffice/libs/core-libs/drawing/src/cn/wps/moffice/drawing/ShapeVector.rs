#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_imports)]

// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput
// Java: import java.util.Vector
use std::io::{Read, Write, Result};
use std::ops::Deref;

use crate::cn::wps::moffice::drawing::Shape::Shape;
use crate::cn::wps::moffice::drawing::IDrawingContainer::IDrawingContainer;

type Vector<T> = Vec<T>;

// Java field: long serialVersionUID
const serialVersionUID: i64 = 1;

// Java: public class ShapeVector extends Vector
pub struct ShapeVector {
    _vector: Vector<Shape>,
}

impl Deref for ShapeVector {
    type Target = Vector<Shape>;
    fn deref(&self) -> &Self::Target {
        &self._vector
    }
}

// Java: public class ShapeVector extends Vector
impl ShapeVector {
    // Java: ShapeVector()
    pub fn new() -> Self {
        // Java: super();
        ShapeVector { _vector: Vec::new() }
    }

    pub fn size(&self) -> i32 {
        self._vector.len() as i32
    }

    pub fn clear(&mut self) {
        self._vector.clear();
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: int n = this.size();
        let n = self.size();
        // Java: for(int i = 0; i < n; ++i)
        for i in 0..n as usize {
            // Java: Shape shape = (Shape)this.get(i);
            let shape = self._vector.get_mut(i);
            // Java: if (shape != null)
            if let Some(shape) = shape {
                // Java: shape.dispose();
                shape.dispose();
            }
        }
        // Java: this.clear();
        self.clear();
    }

    // Java: boolean add(Shape shape)
    pub fn add(&mut self, shape: Shape) -> bool {
        // Java: int zIndex = shape.getZIndex();
        let zIndex = shape.getZIndex();
        // Java: for(int i = 0; i < super.size(); ++i)
        for i in 0..self._vector.len() {
            // Java: Shape eleShape = (Shape)super.elementAt(i);
            let eleShape = &self._vector[i];
            // Java: assert eleShape != null;
            assert!(self._vector.get(i).is_some());
            // Java: int z = eleShape.getZIndex();
            let z = eleShape.getZIndex();
            // Java: if (zIndex <= z)
            if zIndex <= z {
                // Java: super.add(i, shape);
                self._vector.insert(i, shape);
                // Java: this.adjustLast(i + 1, zIndex + 1);
                self.adjustLast((i + 1) as i32, zIndex + 1);
                // Java: return true;
                return true;
            }
        }
        // Java: return super.add(shape);
        self._vector.push(shape);
        true
    }

    // Java: void adjustLast(int location, int zIndex)
    fn adjustLast(&mut self, location: i32, mut zIndex: i32) {
        // Java: for(int i = location; i < super.size(); ++i)
        for i in location as usize..self._vector.len() {
            // Java: Shape shape = (Shape)super.elementAt(i);
            let shape = &mut self._vector[i];
            // Java: assert shape != null;
            assert!(!std::ptr::addr_of!(*shape).is_null());
            // Java: if (shape.getZIndex() < zIndex)
            if shape.getZIndex() < zIndex {
                // Java: shape.setZIndex(zIndex);
                shape.setZIndex(zIndex);
            }
            // Java: zIndex = shape.getZIndex() + 1;
            zIndex = shape.getZIndex() + 1;
        }
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut dyn Write) -> Result<()> {
        // Java: int n = this.size();
        let n = self.size();
        // Java: output.writeInt(this.size());
        output.write_all(&n.to_be_bytes())?;
        // Java: for(int i = 0; i < n; ++i)
        for i in 0..n as usize {
            // Java: Shape shape = (Shape)this.get(i);
            let shape = self._vector.get(i);
            // Java: if (shape != null)
            if let Some(shape) = shape {
                // Java: shape.writeExternal(output);
                let mut buf = Vec::new();
                shape.base.writeExternal(&mut buf);
                output.write_all(&buf)?;
            }
        }
        Ok(())
    }

    // Java: void readExternal(ObjectInput input, IDrawingContainer drawingContainer)
    pub fn readExternal(&mut self, input: &mut dyn Read, _drawingContainer: &dyn IDrawingContainer) -> Result<()> {
        let mut buf = [0u8; 4];
        // Java: int n = input.readInt();
        input.read_exact(&mut buf)?;
        let n = i32::from_be_bytes(buf);
        // Java: for(int i = 0; i < n; ++i)
        for _i in 0..n {
            // Java: Shape shape = new Shape(drawingContainer);
            let shape: Option<Shape> = Some(Shape::Shape_new(None));
            // Java: if (shape != null)
            if let Some(mut shape) = shape {
                // Java: shape.readExternal(input);
                shape.base.readExternal(&[]);
                // Java: this.add(shape);
                self.add(shape);
            }
        }
        Ok(())
    }
}
