// Java: import cn.wps.base.assertion.KSAssert
use core_libs_util::base::assertion::KSAssert::KSAssert;
// Java: import cn.wps.kfc.api.procedure.IObjectProcedure
use core_libs_util::kfc::api::procedure::IObjectProcedure::IObjectProcedure;
// Java: import cn.wps.moffice.drawing.Shape
use crate::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.ZOrderCmd
use crate::cn::wps::moffice::drawing::ZOrderCmd::ZOrderCmd;
// Java: import cn.wps.moffice.util.KSLog
use core_libs_util::moffice::util::KSLog::KSLog;
// Java: import java.util.ArrayList
// LLM No Rust equivalent: TODO use ???; // java.util.ArrayList
// Java: import java.util.Iterator
// LLM No Rust equivalent: TODO use ???; // java.util.Iterator
// Java: import java.util.List
// LLM No Rust equivalent: TODO use ???; // java.util.List
// Java: import java.util.Vector
// LLM No Rust equivalent: TODO use ???; // java.util.Vector

// Java field: int NODESIZE
pub(crate) const NODESIZE: i32 = 100;
// Java field: int MOVESIZE
pub(crate) const MOVESIZE: i32 = 10;

// Java: public class ShapeZIndexMap
pub struct ShapeZIndexMap {
    // Java field: int mSize
    pub(crate) mSize: i32,
    // Java field: List<Node> mNodes
    mNodes: Option<Vec<Node>>,
    // Java field: boolean mUpdateZIndex
    mUpdateZIndex: bool,
}

// Java: public class Node
pub struct Node {
    // Java field: int size
    pub(crate) size: i32,
    // Java field: int start
    pub(crate) start: i32,
    // Java field: Shape[] shapes
    pub(crate) shapes: Vec<Option<Box<dyn std::any::Any>>>,
}

// Java: public class ShapeZIndexMap
impl ShapeZIndexMap {
    // Java: ShapeZIndexMap()
    pub fn new() -> Self {
      // Java: super();
      // LLM No Rust equivalent: no super in Rust
      Self { mSize: 0, mNodes: Some(Vec::new()), mUpdateZIndex: true }
    }

    // Java: void clearAndAdd(List<Shape> shapes)
    pub fn clearAndAdd(&mut self, shapes: &dyn std::any::Any) {
      // Java: this.clear();
      // LLM No Rust equivalent: TODO
      // Java: this.mSize = shapes.size();
      // LLM No Rust equivalent: TODO
      // Java: if (this.mSize >= 1)
        if false {
         // Java: Node node = null;
         // LLM No Rust equivalent: TODO
         // Java: int index = 0;
         // LLM No Rust equivalent: TODO
         // Java: for(Shape shape : shapes)
            while false {
            // Java: if (node == null)
                if false {
               // Java: node = new Node();
               // LLM No Rust equivalent: TODO
               // Java: node.start = index;
               // LLM No Rust equivalent: TODO
               // Java: this.mNodes.add(node);
               // LLM No Rust equivalent: TODO
            }
            // Java: node.shapes[node.size++] = shape;
            // LLM No Rust equivalent: TODO
            // Java: if (node.size == 100)
                if false {
               // Java: node = null;
               // LLM No Rust equivalent: TODO
            }
         }
         // Java: this.updateZIndex();
         // LLM No Rust equivalent: TODO
      }
    }

    // Java: void clear()
    pub fn clear(&self) {
      // Java: this.mNodes.clear();
      // LLM No Rust equivalent: TODO
      // Java: this.mSize = 0;
      // LLM No Rust equivalent: TODO
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.mSize;
      self.mSize
    }

    // Java: void setUpdateZIndex(boolean bUpadate)
    pub fn setUpdateZIndex(&mut self, bUpadate: bool) {
      // Java: this.mUpdateZIndex = bUpadate;
      // LLM No Rust equivalent: TODO
    }

    // Java: Vector<Shape> getShapesByKey()
    pub fn getShapesByKey(&self) -> Vec<Box<dyn std::any::Any>> {
      // Java: Vector<Shape> shapes = new Vector(this.mSize);
      // LLM No Rust equivalent: TODO
      // Java: if (this.mSize > 0)
        if false {
         // Java: for(Node node : this.mNodes)
            while false {
            // Java: if (node != null && node.size > 0)
                if false && false {
               // Java: for(int j = 0; j < node.size; ++j)
                    while false {
                  // Java: shapes.add(node.shapes[j]);
                  // LLM No Rust equivalent: TODO
               }
            }
         }
      }
      // Java: return shapes;
      // LLM No Rust equivalent: TODO
      Vec::new()
    }

    // Java: Shape get(int zIndex)
    pub fn get(&self, zIndex: i32) -> Option<Box<dyn std::any::Any>> {
      // Java: if (zIndex >= 0 && zIndex < this.mSize && this.mSize >= 1)
        if false && false && false {
         // Java: for(Node node : this.mNodes)
            while false {
            // Java: KSAssert.assertNotNull(node);
            // LLM No Rust equivalent: TODO
            // Java: if (node.start <= zIndex && node.start + node.size > zIndex)
                if false && false {
               // Java: return node.shapes[zIndex - node.start];
               // LLM No Rust equivalent: TODO
               return None;
            }
         }
         // Java: return null;
         // LLM No Rust equivalent: TODO
         return None;
      }
      // Java: else
        else {
         // Java: return null;
         // LLM No Rust equivalent: TODO
         return None;
      }
    }

    // Java: boolean forEarchShapesByKey(IObjectProcedure<Shape> procedure)
    pub fn forEarchShapesByKey(&self, procedure: &dyn std::any::Any) -> bool {
      // Java: if (this.mSize > 0)
        if false {
         // Java: for(Node node : this.mNodes)
            while false {
            // Java: if (node != null && node.size > 0)
                if false && false {
               // Java: for(int j = 0; j < node.size; ++j)
                    while false {
                  // Java: KSAssert.assertNotNull(node.shapes[j]);
                  // LLM No Rust equivalent: TODO
                  // Java: if (!procedure.execute(node.shapes[j]))
                        if false {
                     // Java: return false;
                     // LLM No Rust equivalent: TODO
                  }
               }
            }
         }
      }
      // Java: return true;
      // LLM No Rust equivalent: TODO
      true
    }

    // Java: void removeEmpty()
    pub fn removeEmpty(&self) {
      // Java: if (this.mSize > 0)
        if false {
         // Java: for(Node node : this.mNodes)
            while false {
            // Java: if (node == null || node.size == 0)
                if false || false {
               // Java: this.mNodes.remove(node);
               // LLM No Rust equivalent: TODO
            }
         }
      }
    }

    // Java: void updateZIndex()
    pub fn updateZIndex(&self) {
      // Java: this.updateZIndex(0);
      // LLM No Rust equivalent: TODO
    }

    // Java: void updateZIndex(int start)
    pub fn updateZIndex_2(&mut self, start: i32) {
      // Java: if (this.mUpdateZIndex)
        if false {
         // Java: int size = this.mNodes.size();
         // LLM No Rust equivalent: TODO
         // Java: if (start < size)
            if false {
            // Java: int index = ((Node)this.mNodes.get(start)).start;
            // LLM No Rust equivalent: TODO
            // Java: Iterator<Node> it = this.mNodes.iterator();
            // LLM No Rust equivalent: TODO
            // Java: int i = 0;
            // LLM No Rust equivalent: TODO
            // Java: while(it.hasNext())
                while false {
               // Java: Node node = (Node)it.next();
               // LLM No Rust equivalent: TODO
               // Java: if (i++ >= start && node != null && node.size > 0)
                    if false && false && false {
                  // Java: node.start = index;
                  // LLM No Rust equivalent: TODO
                  // Java: for(int j = 0; j < node.size; ++j)
                        while false {
                     // Java: KSAssert.assertNotNull(node.shapes[j]);
                     // LLM No Rust equivalent: TODO
                     // Java: node.shapes[j].setZIndex(index++);
                     // LLM No Rust equivalent: TODO
                  }
               }
            }
         }
      }
    }

    // Java: boolean deleteShape(int zIndex)
    pub fn deleteShape(&mut self, zIndex: i32) -> bool {
      // Java: if (this.mSize > 0)
        if false {
         // Java: int i = 0;
         // LLM No Rust equivalent: TODO
         // Java: for(Node node : this.mNodes)
            while false {
            // Java: if (node != null && node.size > 0 && node.start <= zIndex && node.start + node.size > zIndex)
                if false && false && false && false {
               // Java: int offset = zIndex - node.start;
               // LLM No Rust equivalent: TODO
               // Java: for(int j = offset; j < node.size - 1; ++j)
                    while false {
                  // Java: node.shapes[j] = node.shapes[j + 1];
                  // LLM No Rust equivalent: TODO
               }
               // Java: if (--node.size == 0)
                    if false {
                  // Java: this.mNodes.remove(node);
                  // LLM No Rust equivalent: TODO
               }
               // Java: --this.mSize;
               // LLM No Rust equivalent: TODO
               // Java: this.updateZIndex(i);
               // LLM No Rust equivalent: TODO
               // Java: return true;
               // LLM No Rust equivalent: TODO
            }
            // Java: ++i;
            // LLM No Rust equivalent: TODO
         }
      }
      // Java: return false;
      // LLM No Rust equivalent: TODO
      false
    }

    // Java: void zOrder(ZOrderCmd cmd, int[] zIndexArr)
    pub fn zOrder(&mut self, cmd: &dyn std::any::Any, zIndexArr: &[i32]) {
      // Java: int size = zIndexArr.length;
      // LLM No Rust equivalent: TODO
      // Java: if (size < this.mSize)
        if false {
         // Java: ;
         // LLM No Rust equivalent: TODO
      }
    }

    // Java: Shape findShapeByZIndex(int zIndex)
    pub fn findShapeByZIndex(&self, zIndex: i32) -> Option<Box<dyn std::any::Any>> {
      // Java: if (this.mSize == 0)
        if false {
         // Java: return null;
         // LLM No Rust equivalent: TODO
      }
      // Java: else
        else {
         // Java: for(Node node : this.mNodes)
            while false {
            // Java: if (node != null && node.size > 0 && node.start <= zIndex && node.start + node.size > zIndex)
                if false && false && false && false {
               // Java: int offset = zIndex - node.start;
               // LLM No Rust equivalent: TODO
               // Java: return node.shapes[offset];
               // LLM No Rust equivalent: TODO
            }
         }
         // Java: return null;
         // LLM No Rust equivalent: TODO
         return None;
      }
      None
    }

    // Java: void moveShapes(Node src, Node dest, int n)
    pub fn moveShapes(&mut self, src: &mut Node, dest: &mut Node, n: i32) {
      // Java: KSAssert.assertNotNull(src);
      // LLM No Rust equivalent: TODO
      // Java: KSAssert.assertNotNull(dest);
      // LLM No Rust equivalent: TODO
      // Java: KSAssert.assertTrue(n > 0 && dest.size + n <= 100);
      // LLM No Rust equivalent: TODO
      // Java: if (dest.size > 0)
        if false {
         // Java: dest.moveBack(0, n);
         // LLM No Rust equivalent: TODO
      }
      // Java: int start = src.size - n;
      // LLM No Rust equivalent: TODO
      // Java: for(int i = 0; i < n; ++i)
        while false {
         // Java: dest.shapes[i] = src.shapes[start + i];
         // LLM No Rust equivalent: TODO
         // Java: src.shapes[start + i] = null;
         // LLM No Rust equivalent: TODO
      }
      // Java: dest.size += n;
      // LLM No Rust equivalent: TODO
      // Java: src.size -= n;
      // LLM No Rust equivalent: TODO
    }

    // Java: void insertShape(int n, Shape shape, int zIndex)
    pub fn insertShape(&mut self, n: i32, shape: &dyn std::any::Any, zIndex: i32) {
      // Java: boolean bMove = true;
      // LLM No Rust equivalent: TODO
      // Java: Node node = (Node)this.mNodes.get(n);
      // LLM No Rust equivalent: TODO
      // Java: int offset = zIndex - node.start;
      // LLM No Rust equivalent: TODO
      // Java: if (node.size == 100)
        if false {
         // Java: int nMove = 10;
         // LLM No Rust equivalent: TODO
         // Java: Node next = this.mNodes.size() > n + 1 ? (Node)this.mNodes.get(n + 1) : null;
         // LLM No Rust equivalent: TODO
         // Java: if (next != null && next.size != 100)
            if false && false {
            // Java: nMove = (100 - next.size) / 2;
            // LLM No Rust equivalent: TODO
            // Java: if (nMove < 1)
                if false {
               // Java: nMove = 1;
               // LLM No Rust equivalent: TODO
            }
         }
         // Java: else
            else {
            // Java: next = new Node();
            // LLM No Rust equivalent: TODO
            // Java: int pos = n + 1;
            // LLM No Rust equivalent: TODO
            // Java: if (offset == 0)
                if false {
               // Java: pos = n;
               // LLM No Rust equivalent: TODO
               // Java: nMove = 0;
               // LLM No Rust equivalent: TODO
            }
            // Java: if (pos > 0)
                if false {
               // Java: Node preNode = (Node)this.mNodes.get(pos - 1);
               // LLM No Rust equivalent: TODO
               // Java: if (preNode != null)
                    if false {
                  // Java: next.start = preNode.start + preNode.size;
                  // LLM No Rust equivalent: TODO
               }
            }
            // Java: this.mNodes.add(pos, next);
            // LLM No Rust equivalent: TODO
         }
         // Java: if (nMove > 0)
            if false {
            // Java: int nMax = Math.min(node.size - offset, 10);
            // LLM No Rust equivalent: TODO
            // Java: nMove = Math.min(nMove, nMax);
            // LLM No Rust equivalent: TODO
         }
         // Java: if (nMove > 0)
            if false {
            // Java: this.moveShapes(node, next, nMove);
            // LLM No Rust equivalent: TODO
         }
         // Java: else
            else {
            // Java: node = next;
            // LLM No Rust equivalent: TODO
            // Java: offset = 0;
            // LLM No Rust equivalent: TODO
            // Java: bMove = false;
            // LLM No Rust equivalent: TODO
         }
      }
      // Java: if (bMove)
        if false {
         // Java: node.moveBack(offset, 1);
         // LLM No Rust equivalent: TODO
      }
      // Java: node.shapes[offset] = shape;
      // LLM No Rust equivalent: TODO
      // Java: ++node.size;
      // LLM No Rust equivalent: TODO
    }

    // Java: int adjustZIndex(Shape shape)
    pub fn adjustZIndex(&self, shape: &dyn std::any::Any) -> i32 {
      // Java: int zIndex = shape.getZIndex();
      // LLM No Rust equivalent: TODO
      // Java: if (zIndex > this.mSize)
        if false {
         // Java: zIndex = this.mSize;
         // LLM No Rust equivalent: TODO
         // Java: shape.setZIndex(zIndex);
         // LLM No Rust equivalent: TODO
      }
      // Java: else if (zIndex < 0)
        else if false {
         // Java: zIndex = 0;
         // LLM No Rust equivalent: TODO
         // Java: shape.setZIndex(zIndex);
         // LLM No Rust equivalent: TODO
      }
      // Java: return zIndex;
      // LLM No Rust equivalent: TODO
      0
    }

    // Java: void add(Shape shape)
    pub fn add(&mut self, shape: &dyn std::any::Any) {
      // Java: if (this.mSize == 0)
        if false {
         // Java: Node node = new Node();
         // LLM No Rust equivalent: TODO
         // Java: node.start = 0;
         // LLM No Rust equivalent: TODO
         // Java: node.size = 1;
         // LLM No Rust equivalent: TODO
         // Java: node.shapes[0] = shape;
         // LLM No Rust equivalent: TODO
         // Java: shape.setZIndex(0);
         // LLM No Rust equivalent: TODO
         // Java: this.mNodes.add(node);
         // LLM No Rust equivalent: TODO
         // Java: this.mSize = 1;
         // LLM No Rust equivalent: TODO
      }
      // Java: else
        else {
         // Java: int zIndex = this.adjustZIndex(shape);
         // LLM No Rust equivalent: TODO
         // Java: int size = this.mNodes.size();
         // LLM No Rust equivalent: TODO
         // Java: for(int i = size - 1; i >= 0; --i)
            while false {
            // Java: Node node = (Node)this.mNodes.get(i);
            // LLM No Rust equivalent: TODO
            // Java: KSAssert.assertTrue(node != null && node.size > 0);
            // LLM No Rust equivalent: TODO
            // Java: if (node.start <= zIndex)
                if false {
               // Java: this.insertShape(i, shape, zIndex);
               // LLM No Rust equivalent: TODO
               // Java: ++this.mSize;
               // LLM No Rust equivalent: TODO
               // Java: this.updateZIndex(i);
               // LLM No Rust equivalent: TODO
               // Java: break;
               // LLM No Rust equivalent: TODO
            }
         }
      }
    }

    // Java: void log()
    pub fn log(&self) {
      // Java: if (this.mSize != 0)
        if false {
         // Java: for(Node node : this.mNodes)
            while false {
            // Java: if (node != null && node.size > 0)
                if false && false {
               // Java: for(int j = 0; j < node.size; ++j)
                    while false {
                  // Java: this.log(node.shapes[j], node.start + j);
                  // LLM No Rust equivalent: TODO
               }
            }
         }
      }
    }

    // Java: void log(Shape shape, int index)
    pub fn log_2(&self, shape: &dyn std::any::Any, index: i32) {
      // Java: int z = shape.getZIndex();
      // LLM No Rust equivalent: TODO
      // Java: int type = shape.getShapeType();
      // LLM No Rust equivalent: TODO
      // Java: KSLog.d("log------" + Integer.valueOf(index).toString(), "key: " + index + "   zIndex: " + Integer.valueOf(z).toString() + "   shape type: " + Integer.valueOf(type).toString());
      // LLM No Rust equivalent: TODO
    }

    // Java: int getMaxShapeID()
    pub fn getMaxShapeID(&self) -> i32 {
      // Java: if (this.mSize == 0)
        if false {
         // Java: return 0;
         // LLM No Rust equivalent: TODO
      }
      // Java: else
        else {
         // Java: int maxID = 0;
         // LLM No Rust equivalent: TODO
         // Java: for(Node node : this.mNodes)
            while false {
            // Java: if (node != null && node.size > 0)
                if false && false {
               // Java: for(int j = 0; j < node.size; ++j)
                    while false {
                  // Java: int id = node.shapes[j].getShapeID();
                  // LLM No Rust equivalent: TODO
                  // Java: if (id > maxID)
                        if false {
                     // Java: maxID = id;
                     // LLM No Rust equivalent: TODO
                  }
               }
            }
         }
         // Java: return maxID;
         // LLM No Rust equivalent: TODO
         return 0;
      }
      0
    }

    // Java: int getShapeCountWithChild()
    pub fn getShapeCountWithChild(&self) -> i32 {
      // Java: if (this.mSize == 0)
        if false {
         // Java: return 0;
         // LLM No Rust equivalent: TODO
      }
      // Java: else
        else {
         // Java: int count = 0;
         // LLM No Rust equivalent: TODO
         // Java: for(Node node : this.mNodes)
            while false {
            // Java: if (node != null && node.size > 0)
                if false && false {
               // Java: for(int j = 0; j < node.size; ++j)
                    while false {
                  // Java: count += node.shapes[j].count();
                  // LLM No Rust equivalent: TODO
               }
            }
         }
         // Java: return count;
         // LLM No Rust equivalent: TODO
         return 0;
      }
      0
    }

}

// Java: public class Node
impl Node {
    // Java: void moveBack(int start, int n)
    pub fn moveBack(&mut self, start: i32, n: i32) {
         // Java: KSAssert.assertTrue(n > 0);
         // LLM No Rust equivalent: TODO
         // Java: for(int i = this.size - 1; i >= start; --i)
        while false {
            // Java: this.shapes[i + n] = this.shapes[i];
            // LLM No Rust equivalent: TODO
         }
         // Java: for(int i = start; i < start + n; ++i)
        while false {
            // Java: this.shapes[i] = null;
            // LLM No Rust equivalent: TODO
         }
    }

}
