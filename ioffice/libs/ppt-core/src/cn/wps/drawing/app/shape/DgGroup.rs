#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code, unused_unsafe)]

// Java: import cn.wps.drawing.app.DgStore
use crate::cn::wps::drawing::app::DgStore::DgStore;
// Java: import cn.wps.drawing.data.basic.GroupTransform2D
use super::dg_group_hosts::GroupTransform2D;
// Java: import cn.wps.drawing.data.basic.Transform2D
use super::dg_group_hosts::Transform2D;
// Java: import cn.wps.drawing.data.shape.Shape
use super::dg_group_hosts::Shape;
// Java: import cn.wps.kfc.api.list.IIntList
use super::dg_group_hosts::IIntList;
// Java: import cn.wps.kso.kproto.util.KoIntList
use super::dg_group_hosts::KoIntList;
// Java: import java.util.List
// Java: import cn.wps.drawing.app.shape.Matrix
use super::dg_group_hosts::Matrix;

// java public class DgGroup
pub struct DgGroup {
    // java private int id = -1;
    id: i32,
    // java private final DgStore store;
    // typeoracle: presentation drawingStore alias (Java final field, not owned)
    store: *mut DgStore,
    // java private Shape group = Shape.create();
    group: Shape,
}

// java public class DgGroup
impl DgGroup {
    // java public  DgGroup(DgStore store)
    pub fn DgGroup(store: *mut DgStore) -> Self {
      // java this.store = store;
      Self { id: -1, store: store, group: Shape::create() }
    }

    // java public DgGroup find(int spid)
    pub fn find(&mut self, spid: i32) -> &mut Self {
      // java this.id = spid;
      self.id = spid;
      // java this.group = Shape.open(this.store.query(spid));
      // rust: no-stmt
      // java return this;
      self
    }

    // java public void groupUp(List<Integer> shapeSet)
    pub fn groupUp(&mut self, shapeSet: ()) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KoIntList children = this.group.children();
      let mut children: KoIntList = self.group.children();
      // java for (int spid : shapeSet)
      for spid in shapeSet.iter() {
        // java children.add(spid);
        children.add(spid);
      }
      // java this.initGroupXfrmAfterGroup();
      self.initGroupXfrmAfterGroup();
      // java this.store.update(this.id, this.group.save());
      self.store.update(self.id, self.group.save());
      */
    }

    // java public int[] unGroup()
    pub fn unGroup(&mut self) -> Vec<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int[] unGroupChilds = new int[this.group.children().size()];
      let mut unGroupChilds: Vec<i32> = vec![Default::default(); (self.group.children().len() as i32) as usize];
      // java GroupTransform2D groupTrans = this.group.groupXfrm();
      let mut groupTrans: GroupTransform2D = self.group.groupXfrm();
      // java float widthScale = (float)this.group.spPr().xfrm().extents().cx() / (float)groupTrans.childExtents().cx();
      let mut widthScale: f32 = ((self.group.spPr().xfrm().extents().cx()) as f32) / ((groupTrans.childExtents().cx()) as f32);
      // java float heightScale = (float)this.group.spPr().xfrm().extents().cy() / (float)groupTrans.childExtents().cy();
      let mut heightScale: f32 = ((self.group.spPr().xfrm().extents().cy()) as f32) / ((groupTrans.childExtents().cy()) as f32);
      // java int offsetX = this.group.spPr().xfrm().point().x() - (int)((float)groupTrans.childPoint().x() * widthScale);
      let mut offsetX: i32 = self.group.spPr().xfrm().point().x() - ((((groupTrans.childPoint().x()) as f32) * widthScale) as i32);
      // java int offsetY = this.group.spPr().xfrm().point().y() - (int)((float)groupTrans.childPoint().y() * heightScale);
      let mut offsetY: i32 = self.group.spPr().xfrm().point().y() - ((((groupTrans.childPoint().y()) as f32) * heightScale) as i32);
      // java int i = 0;
      let mut i: i32 = 0;
      let mut count: i32 = (self.group.children().len() as i32);
      // java for (int count = this.group.children().size(); i < count ++i)
      while i < count {
        // java int childSpid = this.group.children().get(i);
        let mut childSpid: i32 = self.group.children().get(i);
        // java unGroupChilds[i] = childSpid;
        unGroupChilds[(i) as usize] = childSpid;
        // java Shape child = Shape.open(this.store.query(childSpid));
        let mut child: Shape = Shape::open(self.store.query(childSpid));
        // java Transform2D childTransform = child.spPr().xfrm();
        let mut childTransform: Transform2D = child.spPr().xfrm();
        // java childTransform.point().putX((int)((float)childTransform.point().x() * widthScale) + offsetX);
        childTransform.point_mut().putX(((((childTransform.point().x()) as f32) * widthScale) as i32) + offsetX);
        // java childTransform.point().putY((int)((float)childTransform.point().y() * heightScale) + offsetY);
        childTransform.point_mut().putY(((((childTransform.point().y()) as f32) * heightScale) as i32) + offsetY);
        // java childTransform.extents().putCx((int)((float)childTransform.extents().cx() * widthScale));
        childTransform.extents_mut().putCx(((((childTransform.extents().cx()) as f32) * widthScale) as i32));
        // java childTransform.extents().putCy((int)((float)childTransform.extents().cy() * heightScale));
        childTransform.extents_mut().putCy(((((childTransform.extents().cy()) as f32) * heightScale) as i32));
        // java this.store.update(childSpid, child.save());
        self.store.update(childSpid, child.save());
      }
      // java return unGroupChilds;
      return unGroupChilds;
      */
    }

    // java public int[] unGroup(Transform2D[] childTransforms)
    pub fn unGroup__transform2darr(&mut self, childTransforms: ()) -> Vec<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int[] unGroupChilds = new int[this.group.children().size()];
      let mut unGroupChilds: Vec<i32> = vec![Default::default(); (self.group.children().len() as i32) as usize];
      // java int i = 0;
      let mut i: i32 = 0;
      let mut count: i32 = (self.group.children().len() as i32);
      // java for (int count = this.group.children().size(); i < count ++i)
      while i < count {
        // java int childSpid = this.group.children().get(i);
        let mut childSpid: i32 = self.group.children().get(i);
        // java unGroupChilds[i] = childSpid;
        unGroupChilds[(i) as usize] = childSpid;
        // java Shape child = Shape.open(this.store.query(childSpid));
        let mut child: Shape = Shape::open(self.store.query(childSpid));
        // java Transform2D childTransform = child.spPr().xfrm();
        let mut childTransform: Transform2D = child.spPr().xfrm();
        // java Transform2D newChildTrans = childTransforms[i];
        let mut newChildTrans: Transform2D = childTransforms[(i) as usize];
        // java childTransform.point().putX(newChildTrans.point().x());
        childTransform.point_mut().putX(newChildTrans.point().x());
        // java childTransform.point().putY(newChildTrans.point().y());
        childTransform.point_mut().putY(newChildTrans.point().y());
        // java childTransform.extents().putCx(newChildTrans.extents().cx());
        childTransform.extents_mut().putCx(newChildTrans.extents().cx());
        // java childTransform.extents().putCy(newChildTrans.extents().cy());
        childTransform.extents_mut().putCy(newChildTrans.extents().cy());
        // java if (newChildTrans.hasRotation() || childTransform.hasRotation())
        if newChildTrans.hasRotation() || childTransform.hasRotation() {
          // java childTransform.putRotation(newChildTrans.rotation());
          childTransform.putRotation(newChildTrans.rotation());
        }
        // java if (newChildTrans.hasFlipVertical() || childTransform.hasFlipVertical())
        if newChildTrans.hasFlipVertical() || childTransform.hasFlipVertical() {
          // java childTransform.putFlipVertical(newChildTrans.isFlipVertical());
          childTransform.putFlipVertical(newChildTrans.isFlipVertical());
        }
        // java if (newChildTrans.hasFlipHorizontal() || childTransform.hasFlipHorizontal())
        if newChildTrans.hasFlipHorizontal() || childTransform.hasFlipHorizontal() {
          // java childTransform.putFlipHorizontal(newChildTrans.isFlipHorizontal());
          childTransform.putFlipHorizontal(newChildTrans.isFlipHorizontal());
        }
        // java this.store.update(childSpid, child.save());
        self.store.update(childSpid, child.save());
      }
      // java return unGroupChilds;
      return unGroupChilds;
      */
    }

    // java private void initGroupXfrmAfterGroup()
    pub fn initGroupXfrmAfterGroup(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Transform2D transform2D = Transform2D.create();
      let mut transform2D: Transform2D = Transform2D::create();
      // java int left = Integer.MAX_VALUE;
      let mut left: i32 = i32::MAX;
      // java int right = Integer.MIN_VALUE;
      let mut right: i32 = i32::MIN;
      // java int top = Integer.MAX_VALUE;
      let mut top: i32 = i32::MAX;
      // java int bottom = Integer.MIN_VALUE;
      let mut bottom: i32 = i32::MIN;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut count: i32 = (self.group.children().len() as i32);
      // java for (int count = this.group.children().size(); i < count ++i)
      while i < count {
        // java Shape shape = Shape.open(this.store.query(this.group.children().get(i)));
        let mut shape: Shape = Shape::open(self.store.query(self.group.children().get(i)));
        // java Transform2D shapeTransform2D = shape.spPr().xfrm();
        let mut shapeTransform2D: Transform2D = shape.spPr().xfrm();
        // java int x = shapeTransform2D.point().x();
        let mut x: i32 = shapeTransform2D.point().x();
        // java int y = shapeTransform2D.point().y();
        let mut y: i32 = shapeTransform2D.point().y();
        // java int cx = shapeTransform2D.extents().cx();
        let mut cx: i32 = shapeTransform2D.extents().cx();
        // java int cy = shapeTransform2D.extents().cy();
        let mut cy: i32 = shapeTransform2D.extents().cy();
        // java if (x < left)
        if x < left {
          // java left = x;
          left = x;
        }
        // java if (y < top)
        if y < top {
          // java top = y;
          top = y;
        }
        // java if (x + cx > right)
        if x + cx > right {
          // java right = x + cx;
          right = x + cx;
        }
        // java if (y + cy > bottom)
        if y + cy > bottom {
          // java bottom = y + cy;
          bottom = y + cy;
        }
      }
      // java transform2D.point().putX(left);
      transform2D.point_mut().putX(left);
      // java transform2D.point().putY(top);
      transform2D.point_mut().putY(top);
      // java transform2D.extents().putCx(right - left);
      transform2D.extents_mut().putCx(right - left);
      // java transform2D.extents().putCy(bottom - top);
      transform2D.extents_mut().putCy(bottom - top);
      // java transform2D.save();
      transform2D.save();
      // java this.group.spPr().forceWriteXfrm(transform2D);
      self.group.spPr().forceWriteXfrm(transform2D);
      // java this.group.groupXfrm().writeChildExtents(transform2D.extents());
      self.group.groupXfrm().writeChildExtents(transform2D.extents());
      // java this.group.groupXfrm().writeChildPoint(transform2D.point());
      self.group.groupXfrm().writeChildPoint(transform2D.point());
      */
    }

    // java public int layerOp(int opType, int spid)
    pub fn layerOp(&mut self, opType: i32, spid: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KoIntList children = this.group.children();
      let mut children: KoIntList = self.group.children();
      // java int childIndex = children.indexOf(spid);
      let mut childIndex: i32 = children.indexOf(spid);
      // java if (childIndex < 0)
      if childIndex < 0 {
        // java return -1;
        return -1;
      }
      // java else
      else {
        // java int newIndex = -1;
        let mut newIndex: i32 = -1;
        // java switch (opType)
        match opType {
          // java case 0:
          0 => {
            // java newIndex = childIndex + 1;
            newIndex = childIndex + 1;
            // java if (newIndex >= children.size())
            if newIndex >= (children.len() as i32) {
              // java newIndex = children.size() - 1;
              newIndex = (children.len() as i32) - 1;
            }
          }
          // java case 1:
          1 => {
            // java newIndex = childIndex - 1;
            newIndex = childIndex - 1;
            // java if (newIndex < 0)
            if newIndex < 0 {
              // java newIndex = 0;
              newIndex = 0;
            }
          }
          // java case 2:
          2 => {
            // java newIndex = children.size() - 1;
            newIndex = (children.len() as i32) - 1;
          }
          // java case 3:
          3 => {
            // java newIndex = 0;
            newIndex = 0;
          }
          // java default:
          _ => {
            // java throw new IllegalArgumentException("error layer type");
            panic!("{}", IllegalArgumentException::new("error layer type"));
          }
        }
        // java children.removeAt(childIndex);
        children.removeAt(childIndex);
        // java children.insert(newIndex, spid);
        children.insert(newIndex, spid);
        // java this.store.update(this.id, this.group.save());
        self.store.update(self.id, self.group.save());
        // java return newIndex;
        return newIndex;
      }
      */
    }

    // java public int size()
    pub fn size(&self) -> i32 {
      // java return this.group.children().size();
      return (self.group.children().len() as i32);
    }

    // java public int childAt(int index)
    pub fn childAt(&self, index: i32) -> i32 {
      // java return this.group.children().get(index);
      return self.group.children()[index as usize];
    }

    // java public int insertAll(KoIntList childIds, int index)
    pub fn insertAll(&mut self, childIds: &KoIntList, index: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int i = 0;
      let mut i: i32 = 0;
      let mut count: i32 = childIds.size();
      // java for (int count = childIds.size(); i < count ++i)
      while i < count {
        // java this.group.children().insert(index + i, childIds.get(i));
        self.group.children().insert(index + i, childIds.get(i));
      }
      // java this.store.update(this.id, this.group.save());
      self.store.update(self.id, self.group.save());
      // java return this.group.children().size() - 1;
      return (self.group.children().len() as i32) - 1;
      */
    }

    // java public int addAll(IIntList childIds)
    pub fn addAll(&mut self, childIds: &dyn IIntList) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int i = 0;
      let mut i: i32 = 0;
      let mut count: i32 = childIds.size();
      // java for (int count = childIds.size(); i < count ++i)
      while i < count {
        // java this.group.children().add(childIds.get(i));
        self.group.children().add(childIds.get(i));
      }
      // java this.store.update(this.id, this.group.save());
      self.store.update(self.id, self.group.save());
      // java return this.group.children().size() - 1;
      return (self.group.children().len() as i32) - 1;
      */
    }

    // java public int add(int childId)
    pub fn add(&mut self, childId: i32) -> i32 {
      // java this.group.children().add(childId);
      self.group.children_mut().push(childId);
      // java this.store.update(this.id, this.group.save());
      if !self.store.is_null() {
        let bin = self.group.save();
        unsafe { (*self.store).update(self.id, bin.as_ref()); }
      }
      // java return this.group.children().size() - 1;
      return (self.group.children().len() as i32) - 1;
    }

    // java public void insert(int index, int childId)
    pub fn insert(&mut self, index: i32, childId: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.group.children().insert(index, childId);
      self.group.children().insert(index, childId);
      // java this.store.update(this.id, this.group.save());
      self.store.update(self.id, self.group.save());
      */
    }

    // java public boolean remove(int childId)
    pub fn remove(&mut self, childId: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.group.children().remove(childId);
      self.group.children().remove(childId);
      // java this.store.update(this.id, this.group.save());
      self.store.update(self.id, self.group.save());
      // java return true;
      return true;
      */
    }

    // java public int removeAt(int index)
    pub fn removeAt(&mut self, index: i32) -> i32 {
      // java int id = this.group.children().removeAt(index);
      // rust: `KoIntList.removeAt` ↔ `Vec::remove`（越界 panic 与 Java IndexOutOfBoundsException 同构）。
      let id: i32 = self.group.children_mut().remove(index as usize);
      // java if (this.group.children().size() == 0)
      if self.group.children().is_empty() {
        // java this.group.deleteChildren();
        self.group.deleteChildren();
      }
      // java this.store.update(this.id, this.group.save());
      // rust: `store` 是 `*mut DgStore`（Java final 别名），空指针检查对应 Java 的不可能为 null。
      if !self.store.is_null() {
        let bin = self.group.save();
        unsafe { (*self.store).update(self.id, bin.as_ref()); }
      }
      // java return id;
      return id;
    }

    // java public void removeAllChild()
    pub fn removeAllChild(&mut self) {
      // java this.group.deleteChildren();
      self.group.deleteChildren();
      // java this.store.update(this.id, this.group.save());
      if !self.store.is_null() {
        let bin = self.group.save();
        unsafe { (*self.store).update(self.id, bin.as_ref()); }
      }
    }

    // java public int childIndex(int spid)
    pub fn childIndex(&self, spid: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.group.children().indexOf(spid);
      return self.group.children().indexOf(spid);
      */
    }

    // java public void adjustGroupXfrm()
    pub fn adjustGroupXfrm(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KoIntList childSpids = this.group.children();
      let mut childSpids: KoIntList = self.group.children();
      // java int left = Integer.MAX_VALUE;
      let mut left: i32 = i32::MAX;
      // java int top = Integer.MAX_VALUE;
      let mut top: i32 = i32::MAX;
      // java int bottom = Integer.MIN_VALUE;
      let mut bottom: i32 = i32::MIN;
      // java int right = Integer.MIN_VALUE;
      let mut right: i32 = i32::MIN;
      // java Shape cacheChild = Shape.create();
      let mut cacheChild: Shape = Shape::create();
      // java if (!childSpids.isEmpty())
      if !childSpids.is_empty() {
        // java cacheChild.load(this.store.query(childSpids.get(0)));
        cacheChild.load(self.store.query(childSpids.get(0)));
        // java int i = 0;
        let mut i: i32 = 0;
        let mut count: i32 = (childSpids.len() as i32);
        // java for (int count = childSpids.size(); i < count ++i)
        while i < count {
          // java cacheChild.load(this.store.query(childSpids.get(i)));
          cacheChild.load(self.store.query(childSpids.get(i)));
          // java Transform2D childXfrm = DgShape.realShapeTransformWithRotation(cacheChild);
          let mut childXfrm: Transform2D = DgShape::realShapeTransformWithRotation(cacheChild);
          // java left = Math.min(childXfrm.point().x(), left);
          left = (childXfrm.point().x()).min(left);
          // java top = Math.min(childXfrm.point().y(), top);
          top = (childXfrm.point().y()).min(top);
          // java right = Math.max(childXfrm.point().x() + childXfrm.extents().cx(), right);
          right = (childXfrm.point().x() + childXfrm.extents().cx()).max(right);
          // java bottom = Math.max(childXfrm.point().y() + childXfrm.extents().cy(), bottom);
          bottom = (childXfrm.point().y() + childXfrm.extents().cy()).max(bottom);
        }
      }
      // java GroupTransform2D groupXfrm = this.group.groupXfrm();
      let mut groupXfrm: GroupTransform2D = self.group.groupXfrm();
      // java Transform2D groupTransform = this.group.spPr().xfrm();
      let mut groupTransform: Transform2D = self.group.spPr().xfrm();
      // java float widthScale = (float)groupTransform.extents().cx() / (float)groupXfrm.childExtents().cx();
      let mut widthScale: f32 = ((groupTransform.extents().cx()) as f32) / ((groupXfrm.childExtents().cx()) as f32);
      // java float heightScale = (float)groupTransform.extents().cy() / (float)groupXfrm.childExtents().cy();
      let mut heightScale: f32 = ((groupTransform.extents().cy()) as f32) / ((groupXfrm.childExtents().cy()) as f32);
      // java int xOffsetOfParent = groupTransform.point().x() - (int)((float)groupXfrm.childPoint().x() * widthScale);
      let mut xOffsetOfParent: i32 = groupTransform.point().x() - ((((groupXfrm.childPoint().x()) as f32) * widthScale) as i32);
      // java int yOffsetOfParent = groupTransform.point().y() - (int)((float)groupXfrm.childPoint().y() * heightScale);
      let mut yOffsetOfParent: i32 = groupTransform.point().y() - ((((groupXfrm.childPoint().y()) as f32) * heightScale) as i32);
      // java groupXfrm.childPoint().putX(left);
      groupXfrm.childPoint().putX(left);
      // java groupXfrm.childPoint().putY(top);
      groupXfrm.childPoint().putY(top);
      // java groupXfrm.childExtents().putCx(right - left);
      groupXfrm.childExtents().putCx(right - left);
      // java groupXfrm.childExtents().putCy(bottom - top);
      groupXfrm.childExtents().putCy(bottom - top);
      // java float rotation = (float)(groupTransform.rotation() / '\uea60');
      let mut rotation: f32 = ((groupTransform.rotation() / '\u{ea60}') as f32);
      // java if (rotation % 360.0F == 0.0F)
      if rotation % 360 == 0 {
        // java groupTransform.point().putX((int)((float)left * widthScale) + xOffsetOfParent);
        groupTransform.point_mut().putX(((((left) as f32) * widthScale) as i32) + xOffsetOfParent);
        // java groupTransform.point().putY((int)((float)top * heightScale) + yOffsetOfParent);
        groupTransform.point_mut().putY(((((top) as f32) * heightScale) as i32) + yOffsetOfParent);
      }
      // java else
      else {
        // java int flipH = groupTransform.isFlipHorizontal() ? -1 : 1;
        let mut flipH: i32 = (if groupTransform.isFlipHorizontal() { -1 } else { 1 });
        // java int flipV = groupTransform.isFlipVertical() ? -1 : 1;
        let mut flipV: i32 = (if groupTransform.isFlipVertical() { -1 } else { 1 });
        // java double angle = (double)rotation * Math.PI / (double)180.0F;
        let mut angle: f64 = ((rotation) as f64) * Math::PI / ((180) as f64);
        // java double sin = Math.sin(angle);
        let mut sin: f64 = Math::sin(angle);
        // java double cos = Math.cos(angle);
        let mut cos: f64 = Math::cos(angle);
        // java float oldCenterX = (float)(groupTransform.point().x() + groupTransform.extents().cx() / 2);
        let mut oldCenterX: f32 = ((groupTransform.point().x() + groupTransform.extents().cx() / 2) as f32);
        // java float oldCenterY = (float)(groupTransform.point().y() + groupTransform.extents().cy() / 2);
        let mut oldCenterY: f32 = ((groupTransform.point().y() + groupTransform.extents().cy() / 2) as f32);
        // java Matrix matrix1 = new Matrix();
        let mut matrix1: Matrix = Matrix::new();
        // java matrix1.preTranslate(oldCenterX, oldCenterY);
        matrix1.preTranslate(oldCenterX, oldCenterY);
        // java matrix1.preRotate(rotation);
        matrix1.preRotate(rotation);
        // java matrix1.preScale((float)flipH, (float)flipV);
        matrix1.preScale(((flipH) as f32), ((flipV) as f32));
        // java matrix1.preTranslate(-oldCenterX, -oldCenterY);
        matrix1.preTranslate(-oldCenterX, -oldCenterY);
        // java float[] pts = new float[2];
        let mut pts: Vec<f32> = vec![Default::default(); 2usize];
        // java float w = (float)(right - left) * widthScale;
        let mut w: f32 = ((right - left) as f32) * widthScale;
        // java float h = (float)(bottom - top) * heightScale;
        let mut h: f32 = ((bottom - top) as f32) * heightScale;
        // java pts[0] = (float)left * widthScale + (float)xOffsetOfParent;
        pts[0usize] = ((left) as f32) * widthScale + ((xOffsetOfParent) as f32);
        // java pts[1] = (float)top * heightScale + (float)yOffsetOfParent;
        pts[1usize] = ((top) as f32) * heightScale + ((yOffsetOfParent) as f32);
        // java matrix1.mapPoint(pts);
        matrix1.mapPoint(pts);
        // java double newLeft = (double)pts[0] + ((double)flipH * cos - (double)1.0F) * (double)w / (double)2.0F - (double)flipV * sin * (double)h / (double)2.0F;
        let mut newLeft: f64 = ((pts[0usize]) as f64) + ((flipH) as f64) * cos - ((1) as f64) * ((w) as f64) / ((2) as f64) - ((flipV) as f64) * sin * ((h) as f64) / ((2) as f64);
        // java double newTop = (double)pts[1] + ((double)flipV * cos - (double)1.0F) * (double)h / (double)2.0F + (double)flipH * sin * (double)w / (double)2.0F;
        let mut newTop: f64 = ((pts[1usize]) as f64) + ((flipV) as f64) * cos - ((1) as f64) * ((h) as f64) / ((2) as f64) + ((flipH) as f64) * sin * ((w) as f64) / ((2) as f64);
        // java groupTransform.point().putX((int)newLeft);
        groupTransform.point_mut().putX(((newLeft) as i32));
        // java groupTransform.point().putY((int)newTop);
        groupTransform.point_mut().putY(((newTop) as i32));
      }
      // java groupTransform.extents().putCx((int)((float)(right - left) * widthScale));
      groupTransform.extents_mut().putCx(((((right - left) as f32) * widthScale) as i32));
      // java groupTransform.extents().putCy((int)((float)(bottom - top) * heightScale));
      groupTransform.extents_mut().putCy(((((bottom - top) as f32) * heightScale) as i32));
      // java this.store.update(this.id, this.group.save());
      self.store.update(self.id, self.group.save());
      */
    }

    // java public void setGrpTransform2D(GroupTransform2D groupTransform2D)
    pub fn setGrpTransform2D(&mut self, groupTransform2D: GroupTransform2D) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.group.writeGroupXfrm(groupTransform2D);
      self.group.writeGroupXfrm(groupTransform2D);
      // java this.store.update(this.id, this.group.save());
      self.store.update(self.id, self.group.save());
      */
    }

    // java public void setTransform2D(Transform2D transform2D)
    pub fn setTransform2D(&mut self, transform2D: Transform2D) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.group.spPr().writeXfrm(transform2D);
      self.group.spPr().writeXfrm(transform2D);
      // java this.store.update(this.id, this.group.save());
      self.store.update(self.id, self.group.save());
      */
    }

    // java public void adjustChildTransform2D(int spId)
    pub fn adjustChildTransform2D(&mut self, spId: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape cacheChild = Shape.create();
      let mut cacheChild: Shape = Shape::create();
      // java cacheChild.load(this.store.query(spId));
      cacheChild.load(self.store.query(spId));
      // java GroupTransform2D groupXfrm = this.group.groupXfrm();
      let mut groupXfrm: GroupTransform2D = self.group.groupXfrm();
      // java Transform2D groupTransform = this.group.spPr().xfrm();
      let mut groupTransform: Transform2D = self.group.spPr().xfrm();
      // java float widthScale = (float)groupTransform.extents().cx() / (float)groupXfrm.childExtents().cx();
      let mut widthScale: f32 = ((groupTransform.extents().cx()) as f32) / ((groupXfrm.childExtents().cx()) as f32);
      // java float heightScale = (float)groupTransform.extents().cy() / (float)groupXfrm.childExtents().cy();
      let mut heightScale: f32 = ((groupTransform.extents().cy()) as f32) / ((groupXfrm.childExtents().cy()) as f32);
      // java int groupOffsetX = groupTransform.point().x();
      let mut groupOffsetX: i32 = groupTransform.point().x();
      // java int groupOffsetY = groupTransform.point().y();
      let mut groupOffsetY: i32 = groupTransform.point().y();
      // java int chOffSetX = groupXfrm.childPoint().x();
      let mut chOffSetX: i32 = groupXfrm.childPoint().x();
      // java int chOffsetY = groupXfrm.childPoint().y();
      let mut chOffsetY: i32 = groupXfrm.childPoint().y();
      // java Transform2D childTransform2D = cacheChild.spPr().xfrm();
      let mut childTransform2D: Transform2D = cacheChild.spPr().xfrm();
      // java int childOffsetX = childTransform2D.point().x();
      let mut childOffsetX: i32 = childTransform2D.point().x();
      // java int childOffsetY = childTransform2D.point().y();
      let mut childOffsetY: i32 = childTransform2D.point().y();
      // java childTransform2D.point().putX((int)((float)(childOffsetX - groupOffsetX) / widthScale) + chOffSetX);
      childTransform2D.point_mut().putX(((((childOffsetX - groupOffsetX) as f32) / widthScale) as i32) + chOffSetX);
      // java childTransform2D.point().putY((int)((float)(childOffsetY - groupOffsetY) / heightScale) + chOffsetY);
      childTransform2D.point_mut().putY(((((childOffsetY - groupOffsetY) as f32) / heightScale) as i32) + chOffsetY);
      // java float originCx = (float)childTransform2D.extents().cx();
      let mut originCx: f32 = ((childTransform2D.extents().cx()) as f32);
      // java float originCy = (float)childTransform2D.extents().cy();
      let mut originCy: f32 = ((childTransform2D.extents().cy()) as f32);
      // java childTransform2D.extents().putCx((int)(originCx / widthScale));
      childTransform2D.extents_mut().putCx(((originCx / widthScale) as i32));
      // java childTransform2D.extents().putCy((int)(originCy / heightScale));
      childTransform2D.extents_mut().putCy(((originCy / heightScale) as i32));
      // java this.store.update(spId, cacheChild.save());
      self.store.update(spId, cacheChild.save());
      */
    }

    // java public void disconnect(int spId)
    pub fn disconnect(&mut self, spId: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.disconnectByGroup(this.id, spId);
      self.disconnectByGroup(self.id, spId);
      */
    }

    // java private void disconnectByGroup(int groupId, int spId)
    pub fn disconnectByGroup(&mut self, groupId: i32, spId: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Shape group = Shape.open(this.store.query(groupId));
      let mut group: Shape = Shape::open(self.store.query(groupId));
      // java KoIntList children = group.children();
      let mut children: KoIntList = group.children();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut count: i32 = (children.len() as i32);
      // java for (int count = children.size(); i < count ++i)
      while i < count {
        // java int childType = DgStore.queryType(children.get(i));
        let mut childType: i32 = DgStore::queryType(children.get(i));
        // java if (childType == 3)
        if childType == 3 {
          // java this.disconnect(children.get(i), spId);
          self.disconnect__int_int(children.get(i), spId);
        }
        // java else if (childType == 0)
        else if childType == 0 {
          // java this.disconnectByGroup(children.get(i), spId);
          self.disconnectByGroup(children.get(i), spId);
        }
      }
      */
    }

    // java private void disconnect(int connectorId, int spId)
    pub fn disconnect__int_int(&mut self, connectorId: i32, spId: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java assert DgStore.queryType(connectorId) == 3;
      todo!();
      // java Shape connector = Shape.open(this.store.query(connectorId));
      let mut connector: Shape = Shape::open(self.store.query(connectorId));
      // java boolean isDisconnected = false;
      let mut isDisconnected: bool = false;
      // java if (connector.hasStCxn() && connector.stCxn().id() == spId)
      if connector.hasStCxn() && connector.stCxn().id() == spId {
        // java connector.deleteStCxn();
        connector.deleteStCxn();
        // java isDisconnected = true;
        isDisconnected = true;
      }
      // java if (connector.hasEndCxn() && connector.endCxn().id() == spId)
      if connector.hasEndCxn() && connector.endCxn().id() == spId {
        // java connector.deleteEndCxn();
        connector.deleteEndCxn();
        // java isDisconnected = true;
        isDisconnected = true;
      }
      // java if (isDisconnected)
      if isDisconnected {
        // java this.store.update(connectorId, connector.save());
        self.store.update(connectorId, connector.save());
      }
      */
    }

}
