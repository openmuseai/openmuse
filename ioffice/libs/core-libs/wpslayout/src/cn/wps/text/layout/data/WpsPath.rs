// Java: import android.graphics.Matrix
pub struct Matrix;

impl Matrix {
    pub fn mapRect(&self, rect: &mut RectF) { let _ = rect; }
}

// Java: import android.graphics.Path
pub struct Path;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PathDirection {
    CW,
    CCW,
}

// Java: import android.graphics.RectF
#[derive(Clone)]
pub struct RectF {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl RectF {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        RectF { left, top, right, bottom }
    }

    pub fn offset(&mut self, dx: f32, dy: f32) {
        self.left += dx;
        self.right += dx;
        self.top += dy;
        self.bottom += dy;
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.left && x <= self.right && y >= self.top && y <= self.bottom
    }
}

// Java: public class WpsPath extends Path
pub struct WpsPath {
    // Java field: ArrayList<RectF> mRects
    pub(crate) mRects: Vec<RectF>,
    // Java field: int mLayoutIndex
    pub(crate) mLayoutIndex: i32,
}

// Java: public class WpsPath extends Path
impl WpsPath {
    // Java: WpsPath(int layoutIndex)
    pub fn new(layoutIndex: i32) -> Self {
      // Java: super();
      // Java: this.mLayoutIndex = layoutIndex;
      WpsPath {
          mRects: Vec::new(),
          mLayoutIndex: layoutIndex,
      }
    }

    // Java: void addRect(float left, float top, float right, float bottom, Path.Direction dir)
    pub fn addRect(&mut self, mut left: f32, mut top: f32, mut right: f32, mut bottom: f32, _dir: PathDirection) {
      // Java: if (left > right)
      if left > right
      {
         // Java: float temp = left;
         let temp = left;
         // Java: left = right;
         left = right;
         // Java: right = temp;
         right = temp;
      }
      // Java: if (top > bottom)
      if top > bottom
      {
         // Java: float temp = top;
         let temp = top;
         // Java: top = bottom;
         top = bottom;
         // Java: bottom = temp;
         bottom = temp;
      }
      // Java: super.addRect(left, top, right, bottom, dir);
      // Java: this.mRects.add(new RectF(left, top, right, bottom));
      self.mRects.push(RectF::new(left, top, right, bottom));
    }

    // Java: void transform(Matrix matrix)
    pub fn transform(&mut self, matrix: &Matrix) {
      // Java: super.transform(matrix);
      // Java: for(int i = 0; i < this.mRects.size(); ++i)
      for i in 0..self.mRects.len()
      {
         // Java: matrix.mapRect((RectF)this.mRects.get(i));
         matrix.mapRect(&mut self.mRects[i]);
      }
    }

    // Java: void offset(float dx, float dy)
    pub fn offset(&mut self, dx: f32, dy: f32) {
      // Java: super.offset(dx, dy);
      // Java: for(int i = 0; i < this.mRects.size(); ++i)
      for i in 0..self.mRects.len()
      {
         // Java: ((RectF)this.mRects.get(i)).offset(dx, dy);
         self.mRects[i].offset(dx, dy);
      }
    }

    // Java: ArrayList<RectF> getRects()
    pub fn getRects(&self) -> &Vec<RectF> {
      // Java: return this.mRects;
      &self.mRects
    }

    // Java: int getLayoutIndex()
    pub fn getLayoutIndex(&self) -> i32 {
      // Java: return this.mLayoutIndex;
      self.mLayoutIndex
    }

    // Java: void mergeRects(boolean isVert)
    pub fn mergeRects(&mut self, isVert: bool) {
      // Java: for(int i = 0; i < this.mRects.size(); ++i)
      let mut i = 0;
      while i < self.mRects.len()
      {
         // Java: RectF first = (RectF)this.mRects.get(i);
         // Java: for(int j = i + 1; j < this.mRects.size(); ++j)
         let mut j = i + 1;
         while j < self.mRects.len()
         {
            // Java: RectF second = (RectF)this.mRects.get(j);
            let second = self.mRects[j].clone();
            let first = &self.mRects[i];
            // Java: int compareTop = isVert ? Float.compare(first.left, second.left) : Float.compare(first.top, second.top);
            let compare_top = if isVert {
                first.left.partial_cmp(&second.left).unwrap() as i32
            } else {
                first.top.partial_cmp(&second.top).unwrap() as i32
            };
            // Java: if (0 != compareTop)
            if 0 != compare_top
            {
               // Java: break;
               break;
            }
            // Java: boolean containRect = isVert ? first.contains(second.left, second.top) || first.contains(second.left, second.bottom) : first.contains(second.left, second.top) || first.contains(second.right, second.top);
            let contain_rect = if isVert {
                first.contains(second.left, second.top) || first.contains(second.left, second.bottom)
            } else {
                first.contains(second.left, second.top) || first.contains(second.right, second.top)
            };
            // Java: if (containRect)
            if contain_rect
            {
               // Java: RectF merge = isVert ? new RectF(first.left, Math.min(first.top, second.top), first.right, Math.max(first.bottom, second.bottom)) : new RectF(Math.min(first.left, second.left), first.top, Math.max(first.right, second.right), first.bottom);
               let merge = if isVert {
                   RectF::new(first.left, first.top.min(second.top), first.right, first.bottom.max(second.bottom))
               } else {
                   RectF::new(first.left.min(second.left), first.top, first.right.max(second.right), first.bottom)
               };
               // Java: this.mRects.remove(i);
               // Java: this.mRects.add(i, merge);
               self.mRects[i] = merge;
               // Java: this.mRects.remove(j);
               self.mRects.remove(j);
               // Java: first = merge;
               // Java: --j;
            } else {
                j += 1;
            }
         }
         i += 1;
      }
    }

}
