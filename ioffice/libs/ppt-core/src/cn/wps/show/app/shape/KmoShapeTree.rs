#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unreachable_code, unused_unsafe)]

// Java: import cn.wps.drawing.app.DgCanvas
use super::kmo_shape_tree_hosts::DgCanvas;
// Java: import cn.wps.drawing.app.KmoProtocol
use super::kmo_shape_tree_hosts::KmoProtocol;
// Java: import cn.wps.drawing.app.chart.KmoOleChart
use super::kmo_shape_tree_hosts::KmoOleChart;
// Java: import cn.wps.drawing.app.shape.DgGroup
use crate::cn::wps::drawing::app::shape::DgGroup::DgGroup;
// Java: import cn.wps.drawing.data.basic.GroupTransform2D
use super::kmo_shape_tree_hosts::GroupTransform2D;
// Java: import cn.wps.drawing.data.basic.Transform2D
use super::kmo_shape_tree_hosts::Transform2D;
// Java: import cn.wps.drawing.data.table.TableStyle
use super::kmo_shape_tree_hosts::TableStyle;
// Java: import cn.wps.graphics.PointF
use super::kmo_shape_tree_hosts::PointF;
// Java: import cn.wps.graphics.Rect
use super::kmo_shape_tree_hosts::Rect;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::KoBin;
// Java: import cn.wps.moffice.kfs.File
use super::kmo_shape_tree_hosts::File;
// Java: import cn.wps.moffice.util.FileDataStorage
use super::kmo_shape_tree_hosts::FileDataStorage;
// Java: import cn.wps.show.app.media.KmoMediaUtil
use super::kmo_shape_tree_hosts::KmoMediaUtil;
// Java: import cn.wps.show.app.slide.ISlide
use crate::cn::wps::show::app::slide::ISlide::ISlide;
// Java: import cn.wps.show.app.theme.KmoTheme
use super::kmo_shape_tree_hosts::KmoTheme;
// Java: import cn.wps.show.app.vml.KmoVmlDrawing
use super::kmo_shape_tree_hosts::KmoVmlDrawing;
// Java: import cn.wps.show.data.shape.ShowShape
use super::kmo_shape_tree_hosts::ShowShape;
// Java: import com.hp.hpl.inkml.Ink
use super::kmo_shape_tree_hosts::Ink;
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import java.lang.Object
use stubs::java::lang::Object::Object;

// java public class KmoShapeTree implements KmoProtocol
pub struct KmoShapeTree {
    // java private ISlide parent;
    parent: Option<*const dyn ISlide>,
    // java private KmoShowShape root;
    root: KmoShowShape,
    // java private KmoVmlDrawing vmlDrawing;
    vmlDrawing: Option<KmoVmlDrawing>,
}

// java public interface KmoProtocol
impl KmoProtocol for KmoShapeTree {
    // java public KoBin protocol()
    fn protocol(&self) -> Box<dyn KoBin> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.root.protocol();
      return self.root.protocol();
      */
    }

    // java public void updateProtocol(KoBin data)
    fn updateProtocol(&mut self, data: &dyn KoBin) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.root.updateProtocol(data);
      self.root.updateProtocol(data);
      */
    }

}

// java public class KmoShapeTree implements KmoProtocol
impl KmoShapeTree {
    // java public  KmoShapeTree(ISlide parent)
    pub fn KmoShapeTree(parent: &dyn ISlide) -> Self {
      // java this.parent = parent;
      let parent_ptr = Some(unsafe {
        std::mem::transmute::<&dyn ISlide, *const dyn ISlide>(parent)
      });
      // java this.root = new KmoShowShape(parent, 0);
      let root = KmoShowShape::KmoShowShape__islide_int(parent, 0);
      KmoShapeTree { parent: parent_ptr, root, vmlDrawing: None }
    }

    // java public void initRoot(KmoShowShape spTree)
    pub fn initRoot(&mut self, spTree: KmoShowShape) {
      // java this.root = spTree;
      self.root = spTree;
    }

    // java public KmoShowShape getRoot()
    pub fn getRoot(&mut self) -> &mut KmoShowShape {
      // java return this.root;
      return &mut self.root;
    }

    // java public ISlide parent()
    pub fn parent(&self) -> Option<*const dyn ISlide> {
      // java return this.parent;
      return self.parent;
    }

    // java public void copyFrom(KmoShapeTree srcTree)
    pub fn copyFrom(&mut self, srcTree: &KmoShapeTree) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.doCopy(srcTree, false);
      self.doCopy(srcTree, false);
      */
    }

    // java public void copyFromEx(KmoShapeTree srcTree)
    pub fn copyFromEx(&mut self, srcTree: &KmoShapeTree) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.doCopy(srcTree, true);
      self.doCopy(srcTree, true);
      */
    }

    // java public void doCopy(KmoShapeTree srcTree, boolean copyMedia)
    pub fn doCopy(&mut self, srcTree: &KmoShapeTree, copyMedia: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (copyMedia)
      if copyMedia {
        // java this.root.copyEx(srcTree.root);
        self.root.copyEx(srcTree.root);
      }
      // java else
      else {
        // java this.root.copy(srcTree.root);
        self.root.copy(srcTree.root);
      }
      // java this.vmlDrawing = srcTree.vmlDrawing;
      self.vmlDrawing = srcTree.vmlDrawing;
      // java this.disconnectShape(this.root);
      self.disconnectShape(&mut self.root);
      */
    }

    // java public void initSpTreeGroupTransform()
    pub fn initSpTreeGroupTransform(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java DgGroup spTreeRoot = this.getRoot().getDgGroup();
      let mut spTreeRoot: DgGroup = self.getRoot().getDgGroup();
      // java GroupTransform2D groupTransform = GroupTransform2D.create();
      let mut groupTransform: GroupTransform2D = GroupTransform2D::create();
      // java groupTransform.childExtents().putCy(0);
      groupTransform.childExtents().putCy(0);
      // java groupTransform.childPoint().putX(0);
      groupTransform.childPoint().putX(0);
      // java groupTransform.save();
      groupTransform.save();
      // java spTreeRoot.setGrpTransform2D(groupTransform);
      spTreeRoot.setGrpTransform2D(groupTransform);
      // java Transform2D transform2D = Transform2D.create();
      let mut transform2D: Transform2D = Transform2D::create();
      // java transform2D.extents().putCx(0);
      transform2D.extents().putCx(0);
      // java transform2D.point().putX(0);
      transform2D.point().putX(0);
      // java transform2D.save();
      transform2D.save();
      // java spTreeRoot.setTransform2D(transform2D);
      spTreeRoot.setTransform2D(transform2D);
      */
    }

    // java public void disconnectShape(KmoShowShape sp)
    pub fn disconnectShape(&mut self, sp: &mut KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int spType = sp.type();
      let mut spType: i32 = sp.r#type();
      // java if (spType == 3)
      if spType == 3 {
        // java sp.disconnect();
        sp.disconnect();
      }
      // java else if (spType == 0)
      else if spType == 0 {
        // java int i = 0;
        let mut i: i32 = 0;
        let mut count: i32 = sp.childCount();
        // java for (int count = sp.childCount(); i < count ++i)
        while i < count {
          // java this.disconnectShape(sp.childAt(i));
          self.disconnectShape(&mut sp.childAt(i));
        }
      }
      */
    }

    // java public KmoShowShape groupUp(KmoShowShape[] sps)
    pub fn groupUp(&mut self, sps: ()) -> KmoShowShape {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoShowShape group = new KmoShowShape(this.root.parent(), 0);
      let mut group: KmoShowShape = KmoShowShape::new(self.root.parent(), 0);
      // java int groupIndex = -1;
      let mut groupIndex: i32 = -1;
      // java for (KmoShowShape child : sps)
      for child in sps.iter() {
        // java int childIndex = this.root.indexOfChild(child);
        let mut childIndex: i32 = self.root.indexOfChild(child);
        // java this.root.removeChildAt(childIndex);
        self.root.removeChildAt(childIndex);
        // java --groupIndex;
        { let _tmp = groupIndex; groupIndex -= 1; _tmp };
        // java groupIndex = groupIndex < childIndex ? childIndex : groupIndex;
        groupIndex = (if groupIndex < childIndex { childIndex } else { groupIndex });
      }
      // java group.groupUp(sps);
      group.groupUp(sps);
      // java this.root.insertChild(groupIndex, group);
      self.root.insertChild(groupIndex, group);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onShapeContentsChanged(5, sps);
      self.parent.eventListeners().onShapeContentsChanged(5, sps);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      // java return group;
      return group;
      */
    }

    // java public void ungroup(KmoShowShape group)
    pub fn ungroup(&mut self, group: &mut KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.ungroup(group, true, true);
      self.ungroup__kmoshowshape_boolean_boolean(group, true, true);
      */
    }

    // java protected void ungroup(KmoShowShape group, boolean notify, boolean setDirty)
    pub fn ungroup__showshape_boolean_boolean(&mut self, group: &mut KmoShowShape, notify: bool, setDirty: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoShowShape ancestor = group.ancestor();
      let mut ancestor: KmoShowShape = group.ancestor();
      // java if (ancestor.type() == 0)
      if ancestor.r#type() == 0 {
        // java ancestor.ungroup();
        ancestor.ungroup();
        // java int groupIndex = this.root.indexOfChild(ancestor);
        let mut groupIndex: i32 = self.root.indexOfChild(ancestor);
        // java KmoShowShape[] children = new KmoShowShape[ancestor.childCount() + 1];
        let mut children: Vec<KmoShowShape> = vec![Default::default(); (ancestor.childCount() + 1) as usize];
        // java children[ancestor.childCount()] = ancestor;
        children[(ancestor.childCount()) as usize] = ancestor;
        let mut index: i32 = ancestor.childCount() - 1;
        // java for (int index = ancestor.childCount() - 1; index >= 0 --index)
        while index >= 0 {
          // java children[index] = ancestor.childAt(index);
          children[(index) as usize] = ancestor.childAt(index);
          // java this.root.insertChild(groupIndex + 1, children[index]);
          self.root.insertChild(groupIndex + 1, children[(index) as usize]);
        }
        // java this.root.removeChildAt(groupIndex);
        self.root.removeChildAt(groupIndex);
        // java if (setDirty)
        if setDirty {
          // java this.parent.getPresentation().setDirty();
          self.parent.getPresentation().setDirty();
        }
        // java this.parent.eventListeners().onShapeContentsChanged(6, children);
        self.parent.eventListeners().onShapeContentsChanged(6, children);
        // java if (notify)
        if notify {
          // java this.parent.eventListeners().onSlideContentsChanged();
          self.parent.eventListeners().onSlideContentsChanged();
        }
      }
      */
    }

    // java private void ungroupself(KmoShowShape group, boolean notify, boolean setDirty)
    pub fn ungroupself(&mut self, group: &mut KmoShowShape, notify: bool, setDirty: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (group.type() == 0)
      if group.r#type() == 0 {
        // java KmoShowShape parentGroup = group.parentGroup();
        let mut parentGroup: KmoShowShape = group.parentGroup();
        // java if (parentGroup == null)
        if false {
          // java parentGroup = this.root;
          parentGroup = self.root;
        }
        // java group.ungroup();
        group.ungroup();
        // java int groupIndex = parentGroup.indexOfChild(group);
        let mut groupIndex: i32 = parentGroup.indexOfChild(group);
        // java int parentChildCount = parentGroup.childCount();
        let mut parentChildCount: i32 = parentGroup.childCount();
        // java KmoShowShape[] children = new KmoShowShape[parentChildCount + group.childCount()];
        let mut children: Vec<KmoShowShape> = vec![Default::default(); (parentChildCount + group.childCount()) as usize];
        let mut index: i32 = 0;
        // java for (int index = 0; index <= parentChildCount - 1 ++index)
        while index <= parentChildCount - 1 {
          // java children[index] = parentGroup.childAt(index);
          children[(index) as usize] = parentGroup.childAt(index);
        }
        let mut index: i32 = 0;
        // java for (int index = 0; index <= group.childCount() - 1 ++index)
        while index <= group.childCount() - 1 {
          // java children[parentChildCount + index] = group.childAt(index);
          children[(parentChildCount + index) as usize] = group.childAt(index);
          // java KmoShowShape leftChild = group.childAt(index);
          let mut leftChild: KmoShowShape = group.childAt(index);
          // java parentGroup.insertChild(groupIndex + index + 1, group.childAt(index));
          parentGroup.insertChild(groupIndex + index + 1, group.childAt(index));
          // java if (group.parentGroup() != null)
          if group.parentGroup().is_some() {
            // java parentGroup.getDgGroup().adjustChildTransform2D(leftChild.id());
            parentGroup.getDgGroup().adjustChildTransform2D(leftChild.id());
          }
        }
        // java parentGroup.removeChildAt(groupIndex);
        parentGroup.removeChildAt(groupIndex);
        // java KmoShowShape tmpGroup = parentGroup;
        let mut tmpGroup: KmoShowShape = parentGroup;
        // java int depth = this.depth(parentGroup);
        let mut depth: i32 = self.depth(&parentGroup);
        // java KmoShowShape[] changedShapes = new KmoShowShape[depth];
        let mut changedShapes: Vec<KmoShowShape> = vec![Default::default(); (depth) as usize];
        let mut var14: i32 = 0;
        // java for (int var14 = 0; tmpGroup != this.root && tmpGroup != null tmpGroup = tmpGroup.parentGroup())
        while tmpGroup != self.root && true {
          // java changedShapes[var14++] = tmpGroup;
          changedShapes[({ let _tmp = var14; var14 += 1; _tmp }) as usize] = tmpGroup;
          // java tmpGroup.getDgGroup().adjustGroupXfrm();
          tmpGroup.getDgGroup().adjustGroupXfrm();
        }
        // java if (setDirty)
        if setDirty {
          // java this.parent.getPresentation().setDirty();
          self.parent.getPresentation().setDirty();
        }
        // java this.parent.eventListeners().onShapeContentsChanged(6, children);
        self.parent.eventListeners().onShapeContentsChanged(6, children);
        // java if (changedShapes.length > 0)
        if (changedShapes.len() as i32) > 0 {
          // java this.parent.eventListeners().onShapeContentsChanged(1, changedShapes);
          self.parent.eventListeners().onShapeContentsChanged(1, changedShapes);
        }
        // java if (notify)
        if notify {
          // java this.parent.eventListeners().onSlideContentsChanged();
          self.parent.eventListeners().onSlideContentsChanged();
        }
      }
      */
    }

    // java public void addPicture(String path, int width, int height)
    pub fn addPicture(&mut self, path: String, width: i32, height: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.addPicture(path, width, height, (String)null);
      self.addPicture__string_int_int_string(path, width, height, Default::default());
      */
    }

    // java public void addPicture(String path, int width, int height, String descr)
    pub fn addPicture__string_int_int_string(&mut self, path: String, width: i32, height: i32, descr: String) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int left = (this.parent.getPresentation().slideWidth() - width) / 2;
      let mut left: i32 = self.parent.getPresentation().slideWidth() - width / 2;
      // java int top = (this.parent.getPresentation().slideHeight() - height) / 2;
      let mut top: i32 = self.parent.getPresentation().slideHeight() - height / 2;
      // java this.addPicture(path, 0, left, top, width, height, descr);
      self.addPicture__string_int_int_int_int_int_string(path, 0, left, top, width, height, descr);
      */
    }

    // java public void addPicture(String path, int rotation, int left, int top, int width, int height)
    pub fn addPicture__string_int_int_int_int_int(&mut self, path: String, rotation: i32, left: i32, top: i32, width: i32, height: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.addPicture(path, rotation, left, top, width, height, (String)null);
      self.addPicture__string_int_int_int_int_int_string(path, rotation, left, top, width, height, Default::default());
      */
    }

    // java public void addPicture(String path, int rotation, int left, int top, int width, int height, String descr)
    pub fn addPicture__string_int_int_int_int_int_string(&mut self, path: String, rotation: i32, left: i32, top: i32, width: i32, height: i32, descr: String) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int picSpId = this.addPictureData(path, rotation, left, top, width, height);
      let mut picSpId: i32 = self.addPictureData__string_int_int_int_int_int(path, rotation, left, top, width, height);
      // java KmoShowShape pic = new KmoShowShape(this.parent, 2, picSpId);
      let mut pic: KmoShowShape = KmoShowShape::new(self.parent, 2, picSpId);
      // java pic.getDgShape().setDescr(descr);
      pic.getDgShape().setDescr(descr);
      // java this.root.addChild(pic);
      self.root.addChild(pic);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(pic);
      self.parent.getPresentation().selection().selectShape(pic);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void addPenKit(String penkitPath, String picPath, String descr, String oleName)
    pub fn addPenKit(&mut self, penkitPath: String, picPath: String, descr: String, oleName: String) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int shapeId = this.parent.getPresentation().canvas().addEmptyShape(4);
      let mut shapeId: i32 = self.parent.getPresentation().canvas().addEmptyShape(4);
      // java ShapeTool.addOleObject(this.parent, penkitPath, picPath, shapeId, oleName, "Package", true);
      ShapeTool::addOleObject(self.parent, penkitPath, picPath, shapeId, oleName, "Package", true);
      // java KmoShowShape penkit = new KmoShowShape(this.parent, 4, shapeId);
      let mut penkit: KmoShowShape = KmoShowShape::new(self.parent, 4, shapeId);
      // java this.root.addChild(penkit);
      self.root.addChild(penkit);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(penkit);
      self.parent.getPresentation().selection().selectShape(penkit);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void add3dObj(String filePath, String coverPath, String filename)
    pub fn add3dObj(&mut self, filePath: String, coverPath: String, filename: String) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int shapeId = this.parent.getPresentation().canvas().addEmptyShape(4);
      let mut shapeId: i32 = self.parent.getPresentation().canvas().addEmptyShape(4);
      // java ShapeTool.addOleObject(this.parent, filePath, coverPath, shapeId, "3dobj", "Package", true);
      ShapeTool::addOleObject(self.parent, filePath, coverPath, shapeId, "3dobj", "Package", true);
      // java KmoShowShape shape3D = new KmoShowShape(this.parent, 4, shapeId);
      let mut shape3D: KmoShowShape = KmoShowShape::new(self.parent, 4, shapeId);
      // java if (filename != null)
      if true {
        // java shape3D.addCustTags("OLE_FILE_NAME", filename);
        shape3D.addCustTags("OLE_FILE_NAME", filename);
      }
      // java shape3D.addCustTags("OLE_FILE_TYPE", "3dobj");
      shape3D.addCustTags("OLE_FILE_TYPE", "3dobj");
      // java this.root.addChild(shape3D);
      self.root.addChild(shape3D);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(shape3D);
      self.parent.getPresentation().selection().selectShape(shape3D);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void addCurveLine(List<PointF> linPoints, Rect bound, int fillColor, int lineColor, int lineWidth, boolean isQuad)
    pub fn addCurveLine(&mut self, linPoints: (), bound: Rect, fillColor: i32, lineColor: i32, lineWidth: i32, isQuad: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (linPoints != null && !linPoints.isEmpty() && bound != null)
      if true && !linPoints.is_empty() && true {
        // java int shapeId = this.parent.getPresentation().canvas().addCurveLine(linPoints, bound, fillColor, lineColor, lineWidth, isQuad);
        let mut shapeId: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
        // java KmoShowShape lineShape = new KmoShowShape(this.parent, 1, shapeId);
        let mut lineShape: KmoShowShape = KmoShowShape::new(self.parent, 1, shapeId);
        // java this.root.addChild(lineShape);
        self.root.addChild(lineShape);
        // java this.parent.getPresentation().setDirty();
        self.parent.getPresentation().setDirty();
        // java this.parent.getPresentation().selection().selectShape(lineShape);
        self.parent.getPresentation().selection().selectShape(lineShape);
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      */
    }

    // java public int addAudio(String audioPath, String picPath, int duration, int left, int top, int width, int height)
    pub fn addAudio(&mut self, audioPath: String, picPath: String, duration: i32, left: i32, top: i32, width: i32, height: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int picSpId = this.addPictureData(picPath, 0, left, top, width, height);
      let mut picSpId: i32 = self.addPictureData__string_int_int_int_int_int(picPath, 0, left, top, width, height);
      // java KmoShowShape pic = new KmoShowShape(this.parent, 2, picSpId);
      let mut pic: KmoShowShape = KmoShowShape::new(self.parent, 2, picSpId);
      // java int audioMediaId = this.addMediaData(audioPath);
      let mut audioMediaId: i32 = self.addMediaData(audioPath);
      // java ShowShape swPic = ShowShape.open(pic.protocol());
      let mut swPic: ShowShape = ShowShape::open(pic.protocol());
      // java swPic.media().putMode(2);
      swPic.media().putMode(2);
      // java swPic.media().audioFile().putMediaID(audioMediaId);
      swPic.media().audioFile().putMediaID(audioMediaId);
      // java pic.updateProtocol(swPic.save());
      pic.updateProtocol(swPic.save());
      // java this.root.addChild(pic);
      self.root.addChild(pic);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(pic);
      self.parent.getPresentation().selection().selectShape(pic);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      // java return picSpId;
      return picSpId;
      */
    }

    // java public int addAudio(String audioPath, String picPath, int duration, int width, int height)
    pub fn addAudio__string_string_int_int_int(&mut self, audioPath: String, picPath: String, duration: i32, width: i32, height: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int picSpId = this.addPictureData(picPath, 0, width, height);
      let mut picSpId: i32 = self.addPictureData__string_int_int_int(picPath, 0, width, height);
      // java KmoShowShape pic = new KmoShowShape(this.parent, 2, picSpId);
      let mut pic: KmoShowShape = KmoShowShape::new(self.parent, 2, picSpId);
      // java int audioMediaId = this.addMediaData(audioPath);
      let mut audioMediaId: i32 = self.addMediaData(audioPath);
      // java ShowShape swPic = ShowShape.open(pic.protocol());
      let mut swPic: ShowShape = ShowShape::open(pic.protocol());
      // java swPic.media().putMode(2);
      swPic.media().putMode(2);
      // java swPic.media().audioFile().putMediaID(audioMediaId);
      swPic.media().audioFile().putMediaID(audioMediaId);
      // java pic.updateProtocol(swPic.save());
      pic.updateProtocol(swPic.save());
      // java this.root.addChild(pic);
      self.root.addChild(pic);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(pic);
      self.parent.getPresentation().selection().selectShape(pic);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      // java return picSpId;
      return picSpId;
      */
    }

    // java public KmoShowShape addNoteAudio(String audioPath, String picPath, int duration, int width, int height)
    pub fn addNoteAudio(&mut self, audioPath: String, picPath: String, duration: i32, width: i32, height: i32) -> KmoShowShape {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int picSpId = this.addPictureData(picPath, 0, width, height);
      let mut picSpId: i32 = self.addPictureData__string_int_int_int(picPath, 0, width, height);
      // java KmoShowShape pic = new KmoShowShape(this.parent, 2, picSpId);
      let mut pic: KmoShowShape = KmoShowShape::new(self.parent, 2, picSpId);
      // java int audioMediaId = this.addMediaData(audioPath);
      let mut audioMediaId: i32 = self.addMediaData(audioPath);
      // java ShowShape swPic = ShowShape.open(pic.protocol());
      let mut swPic: ShowShape = ShowShape::open(pic.protocol());
      // java swPic.media().putMode(2);
      swPic.media().putMode(2);
      // java swPic.media().audioFile().putMediaID(audioMediaId);
      swPic.media().audioFile().putMediaID(audioMediaId);
      // java pic.updateProtocol(swPic.save());
      pic.updateProtocol(swPic.save());
      // java this.root.addChild(pic);
      self.root.addChild(pic);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      // java return pic;
      return pic;
      */
    }

    // java public int addVideo(String videoPath, String picPath, int rotation, int width, int height)
    pub fn addVideo(&mut self, videoPath: String, picPath: String, rotation: i32, width: i32, height: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int picSpId = this.addPictureData(picPath, rotation, width, height);
      let mut picSpId: i32 = self.addPictureData__string_int_int_int(picPath, rotation, width, height);
      // java KmoShowShape pic = new KmoShowShape(this.parent, 2, picSpId);
      let mut pic: KmoShowShape = KmoShowShape::new(self.parent, 2, picSpId);
      // java int videoMediaId = this.addMediaData(videoPath);
      let mut videoMediaId: i32 = self.addMediaData(videoPath);
      // java ShowShape swPic = ShowShape.open(pic.protocol());
      let mut swPic: ShowShape = ShowShape::open(pic.protocol());
      // java swPic.media().putMode(3);
      swPic.media().putMode(3);
      // java swPic.media().videoFile().putMediaID(videoMediaId);
      swPic.media().videoFile().putMediaID(videoMediaId);
      // java pic.updateProtocol(swPic.save());
      pic.updateProtocol(swPic.save());
      // java this.root.addChild(pic);
      self.root.addChild(pic);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(pic);
      self.parent.getPresentation().selection().selectShape(pic);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      // java return picSpId;
      return picSpId;
      */
    }

    // java public int addVideo(String videoPath, String picPath, int rotation, int left, int top, int width, int height)
    pub fn addVideo__string_string_int_int_int_int_int(&mut self, videoPath: String, picPath: String, rotation: i32, left: i32, top: i32, width: i32, height: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int picSpId = this.addPictureData(picPath, rotation, left, top, width, height);
      let mut picSpId: i32 = self.addPictureData__string_int_int_int_int_int(picPath, rotation, left, top, width, height);
      // java KmoShowShape pic = new KmoShowShape(this.parent, 2, picSpId);
      let mut pic: KmoShowShape = KmoShowShape::new(self.parent, 2, picSpId);
      // java int videoMediaId = this.addMediaData(videoPath);
      let mut videoMediaId: i32 = self.addMediaData(videoPath);
      // java ShowShape swPic = ShowShape.open(pic.protocol());
      let mut swPic: ShowShape = ShowShape::open(pic.protocol());
      // java swPic.media().putMode(3);
      swPic.media().putMode(3);
      // java swPic.media().videoFile().putMediaID(videoMediaId);
      swPic.media().videoFile().putMediaID(videoMediaId);
      // java pic.updateProtocol(swPic.save());
      pic.updateProtocol(swPic.save());
      // java this.root.addChild(pic);
      self.root.addChild(pic);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(pic);
      self.parent.getPresentation().selection().selectShape(pic);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      // java return picSpId;
      return picSpId;
      */
    }

    // java private int addPictureData(String path, int rotation, int width, int height)
    pub fn addPictureData(&mut self, path: String, rotation: i32, width: i32, height: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int left = (this.parent.getPresentation().slideWidth() - width) / 2;
      let mut left: i32 = self.parent.getPresentation().slideWidth() - width / 2;
      // java int top = (this.parent.getPresentation().slideHeight() - height) / 2;
      let mut top: i32 = self.parent.getPresentation().slideHeight() - height / 2;
      // java return this.addPictureData(path, rotation, left, top, width, height);
      return self.addPictureData__string_int_int_int_int_int(path, rotation, left, top, width, height);
      */
    }

    // java private int addPictureData(String path, int rotation, int left, int top, int width, int height)
    pub fn addPictureData__string_int_int_int_int_int(&mut self, path: String, rotation: i32, left: i32, top: i32, width: i32, height: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int mediaId = this.addMediaData(path);
      let mut mediaId: i32 = self.addMediaData(path);
      // java return this.parent.getPresentation().canvas().addPic(mediaId, rotation, top, left, width, height);
      return self.parent.getPresentation().canvas().addPic(mediaId, rotation, top, left, width, height);
      */
    }

    // java private int addMediaData(String mediaPath)
    pub fn addMediaData(&mut self, mediaPath: String) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java String fileExtension = mediaPath.substring(mediaPath.lastIndexOf(".") + 1);
      let mut fileExtension: String = mediaPath.substring(mediaPath.lastIndexOf(".") + 1);
      // java byte mediaType = KmoMediaUtil.getMediaType(fileExtension);
      let mut mediaType: u8 = KmoMediaUtil::getMediaType(fileExtension);
      // java return this.parent.getPresentation().mediaLib().addMediaData((FileDataStorage)(new FileDataStorage(new File(mediaPath))), mediaType);
      return self.parent.getPresentation().mediaLib().addMediaData((FileDataStorage::new(File::new(mediaPath))).downcast_ref::<FileDataStorage>().unwrap(), mediaType);
      */
    }

    // java private int depth(KmoShowShape shape)
    pub fn depth(&self, shape: &KmoShowShape) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int depth;
      let mut depth: i32;
      depth = 0;
      // java for (depth = 0 shape != this.root && shape != null shape = shape.parentGroup())
      while shape != self.root && true {
        // java ++depth;
        { let _tmp = depth; depth += 1; _tmp };
      }
      // java return depth;
      return depth;
      */
    }

    // java public KmoShowShape replacePicture2PlaceHolder(KmoShowShape phShape, String picPath, int width, int height)
    pub fn replacePicture2PlaceHolder(&mut self, phShape: KmoShowShape, picPath: String, width: i32, height: i32) -> KmoShowShape {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.replacePicture2PlaceHolder(phShape, picPath, width, height, (String)null);
      return self.replacePicture2PlaceHolder__kmoshowshape_string_int_int_string(phShape, picPath, width, height, Default::default());
      */
    }

    // java public KmoShowShape replacePicture2PlaceHolder(KmoShowShape phShape, String picPath, int width, int height, String descr)
    pub fn replacePicture2PlaceHolder__showshape_string_int_int_string(&mut self, phShape: KmoShowShape, picPath: String, width: i32, height: i32, descr: String) -> KmoShowShape {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java String fileExtension = picPath.substring(picPath.lastIndexOf(".") + 1);
      let mut fileExtension: String = picPath.substring(picPath.lastIndexOf(".") + 1);
      // java byte mediaType = KmoMediaUtil.getMediaType(fileExtension);
      let mut mediaType: u8 = KmoMediaUtil::getMediaType(fileExtension);
      // java int mediaId = this.parent.getPresentation().mediaLib().addMediaData((FileDataStorage)(new FileDataStorage(new File(picPath))), mediaType);
      let mut mediaId: i32 = self.parent.getPresentation().mediaLib().addMediaData((FileDataStorage::new(File::new(picPath))).downcast_ref::<FileDataStorage>().unwrap(), mediaType);
      // java Transform2D phTransform = PlaceHolderMerger.mergeTransform(phShape);
      let mut phTransform: Transform2D = PlaceHolderMerger::mergeTransform(phShape);
      // java float heightScale = 0.0F;
      let mut heightScale: f32 = 0;
      // java float widthScale = 0.0F;
      let mut widthScale: f32 = 0;
      // java if (width < height)
      if width < height {
        // java heightScale = (float)width / (float)height * ((float)phTransform.extents().cy() / (float)phTransform.extents().cx());
        heightScale = ((width) as f32) / ((height) as f32) * ((phTransform.extents().cy()) as f32) / ((phTransform.extents().cx()) as f32);
        // java heightScale = (1.0F - heightScale) * 0.5F;
        heightScale = 1 - heightScale * 0.5;
      }
      // java else
      else {
        // java widthScale = (float)height / (float)width * ((float)phTransform.extents().cx() / (float)phTransform.extents().cy());
        widthScale = ((height) as f32) / ((width) as f32) * ((phTransform.extents().cx()) as f32) / ((phTransform.extents().cy()) as f32);
        // java widthScale = (1.0F - widthScale) * 0.5F;
        widthScale = 1 - widthScale * 0.5;
      }
      // java int picSpId = this.parent.getPresentation().canvas().replacePicture2PlaceHolder(mediaId, phShape.id(), widthScale, widthScale, heightScale, heightScale);
      let mut picSpId: i32 = self.parent.getPresentation().canvas().replacePicture2PlaceHolder(mediaId, phShape.id(), widthScale, widthScale, heightScale, heightScale);
      // java KmoShowShape pic = new KmoShowShape(this.parent, 2, picSpId);
      let mut pic: KmoShowShape = KmoShowShape::new(self.parent, 2, picSpId);
      // java int audioMediaId = phShape.audioMediaId();
      let mut audioMediaId: i32 = phShape.audioMediaId();
      // java if (-1 != phShape.audioMediaId())
      if -1 != phShape.audioMediaId() {
        // java ShowShape swPic = ShowShape.open(pic.protocol());
        let mut swPic: ShowShape = ShowShape::open(pic.protocol());
        // java swPic.media().putMode(2);
        swPic.media().putMode(2);
        // java swPic.media().audioFile().putMediaID(audioMediaId);
        swPic.media().audioFile().putMediaID(audioMediaId);
        // java pic.updateProtocol(swPic.save());
        pic.updateProtocol(swPic.save());
      }
      // java pic.getDgShape().setDescr(descr);
      pic.getDgShape().setDescr(descr);
      // java int index = this.root.indexOfChild(phShape);
      let mut index: i32 = self.root.indexOfChild(phShape);
      // java if (index == -1 && phShape.isChildOfGroup() && phShape.modularGroupType().equals("pic"))
      if index == -1 && phShape.isChildOfGroup() && phShape.modularGroupType().equals("pic") {
        // java KmoShowShape parentGroup = phShape.parentGroup();
        let mut parentGroup: KmoShowShape = phShape.parentGroup();
        // java int innerIndex = parentGroup.indexOfChild(phShape);
        let mut innerIndex: i32 = parentGroup.indexOfChild(phShape);
        // java parentGroup.removeChildAt(innerIndex);
        parentGroup.removeChildAt(innerIndex);
        // java parentGroup.insertChild(innerIndex, pic);
        parentGroup.insertChild(innerIndex, pic);
        // java pic.putModularGroupExt("pic", "", false);
        pic.putModularGroupExt("pic", "", false);
      }
      // java else
      else {
        // java this.root.removeChildAt(index);
        self.root.removeChildAt(index);
        // java this.root.insertChild(index, pic);
        self.root.insertChild(index, pic);
        // java pic.writePh(phShape.ph());
        pic.writePh(phShape.ph());
      }
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(pic);
      self.parent.getPresentation().selection().selectShape(pic);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      // java return pic;
      return pic;
      */
    }

    // java public void replaceAudio(KmoShowShape phShape, String audioPath)
    pub fn replaceAudio(&mut self, phShape: &mut KmoShowShape, audioPath: String) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java KmoShowShape pic = new KmoShowShape(phShape.parent(), phShape.type());
      let mut pic: KmoShowShape = KmoShowShape::new(phShape.parent(), phShape.r#type());
      // java pic.copy(phShape);
      pic.copy(phShape);
      // java int audioMediaId = this.addMediaData(audioPath);
      let mut audioMediaId: i32 = self.addMediaData(audioPath);
      // java ShowShape swPic = ShowShape.open(pic.protocol());
      let mut swPic: ShowShape = ShowShape::open(pic.protocol());
      // java swPic.media().putMode(2);
      swPic.media().putMode(2);
      // java swPic.media().audioFile().putMediaID(audioMediaId);
      swPic.media().audioFile().putMediaID(audioMediaId);
      // java pic.updateProtocol(swPic.save());
      pic.updateProtocol(swPic.save());
      // java KmoShowShape directParent = phShape.isChildOfGroup() ? phShape.parentGroup() : this.root;
      let mut directParent: KmoShowShape = (if phShape.isChildOfGroup() { phShape.parentGroup() } else { self.root });
      // java if (directParent != null)
      if true {
        // java int index = directParent.indexOfChild(phShape);
        let mut index: i32 = directParent.indexOfChild(phShape);
        // java directParent.removeChildAt(index);
        directParent.removeChildAt(index);
        // java directParent.insertChild(index, pic);
        directParent.insertChild(index, pic);
      }
      // java pic.writePh(phShape.ph());
      pic.writePh(phShape.ph());
      // java this.parent.replaceSpid(phShape.id(), pic.id());
      self.parent.replaceSpid(phShape.id(), pic.id());
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(pic);
      self.parent.getPresentation().selection().selectShape(pic);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void addPresetShape(int presetType, int fillColor, int lineColor, int lineWidth, int width, int height)
    pub fn addPresetShape(&mut self, presetType: i32, fillColor: i32, lineColor: i32, lineWidth: i32, width: i32, height: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int left = (this.parent.getPresentation().slideWidth() - width) / 2;
      let mut left: i32 = self.parent.getPresentation().slideWidth() - width / 2;
      // java int top = (this.parent.getPresentation().slideHeight() - height) / 2;
      let mut top: i32 = self.parent.getPresentation().slideHeight() - height / 2;
      // java this.addPresetShape(presetType, fillColor, lineColor, lineWidth, left, top, width, height, 0, false, false);
      self.addPresetShape__int_int_int_int_int_int_int_int_int_boolean_boolean(presetType, fillColor, lineColor, lineWidth, left, top, width, height, 0, false, false);
      */
    }

    // java public void addPresetShape(int presetType, int fillColor, int lineColor, int lineWidth, int left, int top, int width, int height, int rotation, boolean flipH, boolean flipV)
    pub fn addPresetShape__int_int_int_int_int_int_int_int_int_boolean_boolean(&mut self, presetType: i32, fillColor: i32, lineColor: i32, lineWidth: i32, left: i32, top: i32, width: i32, height: i32, rotation: i32, flipH: bool, flipV: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int shapeId = this.parent.getPresentation().canvas().addPresetShape(false, presetType, fillColor, lineColor, lineWidth, left, top, width, height, (String)null, 0, 0, flipH, flipV);
      let mut shapeId: i32 = self.parent.getPresentation().canvas().addPresetShape(false, presetType, fillColor, lineColor, lineWidth, left, top, width, height, String::new(), 0, 0, flipH, flipV);
      // java KmoShowShape shape = new KmoShowShape(this.parent, 1, shapeId);
      let mut shape: KmoShowShape = KmoShowShape::new(self.parent, 1, shapeId);
      // java if (rotation > 0)
      if rotation > 0 {
        // java shape.rotate(rotation);
        shape.rotate(rotation);
      }
      // java this.root.addChild(shape);
      self.root.addChild(shape);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(shape);
      self.parent.getPresentation().selection().selectShape(shape);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void addInk(Ink ink, int left, int top, int width, int height)
    pub fn addInk(&mut self, ink: &Ink, left: i32, top: i32, width: i32, height: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int inkSourceId = this.parent.getPresentation().inkLib().addInk(ink);
      let mut inkSourceId: i32 = self.parent.getPresentation().inkLib().addInk(ink);
      // java int inkId = this.parent.getPresentation().canvas().addInk(inkSourceId, top, left, width, height);
      let mut inkId: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
      // java KmoShowShape inkShape = new KmoShowShape(this.parent, 1, inkId);
      let mut inkShape: KmoShowShape = KmoShowShape::new(self.parent, 1, inkId);
      // java inkShape.putInkId(inkSourceId);
      inkShape.putInkId(inkSourceId);
      // java int shapeIndex = this.root.childCount();
      let mut shapeIndex: i32 = self.root.childCount();
      // java this.root.addChild(inkShape);
      self.root.addChild(inkShape);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.eventListeners().onPartContentChanged(shapeIndex);
      self.parent.eventListeners().onPartContentChanged(shapeIndex);
      // java this.parent.eventListeners().onAddInkShape(inkShape);
      self.parent.eventListeners().onAddInkShape(inkShape);
      */
    }

    // java public KmoShowShape addTextBox(String text, int fontSize, int fontColor, int width, int height)
    pub fn addTextBox(&mut self, text: String, fontSize: i32, fontColor: i32, width: i32, height: i32) -> KmoShowShape {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int left = (this.parent.getPresentation().slideWidth() - width) / 2;
      let mut left: i32 = self.parent.getPresentation().slideWidth() - width / 2;
      // java int top = (this.parent.getPresentation().slideHeight() - height) / 2;
      let mut top: i32 = self.parent.getPresentation().slideHeight() - height / 2;
      // java return this.addTextBox(text, left, top, width, height, fontSize, fontColor);
      return self.addTextBox__string_int_int_int_int_int_int(text, left, top, width, height, fontSize, fontColor);
      */
    }

    // java public KmoShowShape addTextBox(String text, Rect pos, int fontSize, int fontColor)
    pub fn addTextBox__string_rect_int_int(&mut self, text: String, pos: Rect, fontSize: i32, fontColor: i32) -> KmoShowShape {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int left = pos.left;
      let mut left: i32 = pos.left;
      // java int top = pos.top;
      let mut top: i32 = pos.top;
      // java int width = pos.width();
      let mut width: i32 = pos.width();
      // java int height = pos.height();
      let mut height: i32 = pos.height();
      // java return this.addTextBox(text, left, top, width, height, fontSize, fontColor);
      return self.addTextBox__string_int_int_int_int_int_int(text, left, top, width, height, fontSize, fontColor);
      */
    }

    // java public KmoShowShape addTextBox(String text, int left, int top, int width, int height, int fontSize, int fontColor)
    pub fn addTextBox__string_int_int_int_int_int_int(&mut self, text: String, left: i32, top: i32, width: i32, height: i32, fontSize: i32, fontColor: i32) -> KmoShowShape {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int shapeId = this.parent.getPresentation().canvas().addPresetShape(true, 1, -1, -1, -1, left, top, width, height, text, fontColor, fontSize, false, false);
      let mut shapeId: i32 = self.parent.getPresentation().canvas().addPresetShape(true, 1, -1, -1, -1, left, top, width, height, text, fontColor, fontSize, false, false);
      // java KmoShowShape textBox = new KmoShowShape(this.parent, 1, shapeId);
      let mut textBox: KmoShowShape = KmoShowShape::new(self.parent, 1, shapeId);
      // java this.root.addChild(textBox);
      self.root.addChild(textBox);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(textBox);
      self.parent.getPresentation().selection().selectShape(textBox);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      // java return textBox;
      return textBox;
      */
    }

    // java public void addConnector(int connectorType, int fillColor, int lineColor, int lineWidth, boolean hasBeginArrow, boolean hasEndArrow, int width, int height)
    pub fn addConnector(&mut self, connectorType: i32, fillColor: i32, lineColor: i32, lineWidth: i32, hasBeginArrow: bool, hasEndArrow: bool, width: i32, height: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int left = (this.parent.getPresentation().slideWidth() - width) / 2;
      let mut left: i32 = self.parent.getPresentation().slideWidth() - width / 2;
      // java int top = (this.parent.getPresentation().slideHeight() - height) / 2;
      let mut top: i32 = self.parent.getPresentation().slideHeight() - height / 2;
      // java this.addConnector(connectorType, fillColor, lineColor, lineWidth, hasBeginArrow, hasEndArrow, left, top, width, height, false, false);
      self.addConnector__int_int_int_int_boolean_boolean_int_int_int_int_boolean_boolean(connectorType, fillColor, lineColor, lineWidth, hasBeginArrow, hasEndArrow, left, top, width, height, false, false);
      */
    }

    // java public void addConnector(int connectorType, int fillColor, int lineColor, int lineWidth, boolean hasBeginArrow, boolean hasEndArrow, int left, int top, int width, int height, boolean hasFlipH, boolean hasBlipV)
    pub fn addConnector__int_int_int_int_boolean_boolean_int_int_int_int_boolean_boolean(&mut self, connectorType: i32, fillColor: i32, lineColor: i32, lineWidth: i32, hasBeginArrow: bool, hasEndArrow: bool, left: i32, top: i32, width: i32, height: i32, hasFlipH: bool, hasBlipV: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int connId = this.parent.getPresentation().canvas().addConnector(connectorType, fillColor, lineColor, lineWidth, left, top, width, height, hasBeginArrow, hasEndArrow, hasFlipH, hasBlipV);
      let mut connId: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
      // java KmoShowShape conn = new KmoShowShape(this.parent, 3, connId);
      let mut conn: KmoShowShape = KmoShowShape::new(self.parent, 3, connId);
      // java this.root.addChild(conn);
      self.root.addChild(conn);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(conn);
      self.parent.getPresentation().selection().selectShape(conn);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void addTable(String styleId, int colCount, int rowCount, int width)
    pub fn addTable(&mut self, styleId: String, colCount: i32, rowCount: i32, width: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int height = rowCount * 741680;
      let mut height: i32 = rowCount * 741680;
      // java int left = (this.parent.getPresentation().slideWidth() - width) / 2;
      let mut left: i32 = self.parent.getPresentation().slideWidth() - width / 2;
      // java int top = (this.parent.getPresentation().slideHeight() - height) / 2;
      let mut top: i32 = self.parent.getPresentation().slideHeight() - height / 2;
      // java int tableShapeId = this.parent.getPresentation().canvas().addTable(styleId, colCount, rowCount, left, top, width, height);
      let mut tableShapeId: i32 = self.parent.getPresentation().canvas().addTable(styleId, colCount, rowCount, left, top, width, height);
      // java KmoShowShape tableShape = new KmoShowShape(this.parent, 4, tableShapeId);
      let mut tableShape: KmoShowShape = KmoShowShape::new(self.parent, 4, tableShapeId);
      // java if (this.parent.getPresentation().tableStyleLst().getStyleById(styleId) == null)
      if self.parent.getPresentation().tableStyleLst().getStyleById(styleId).is_none() {
        // java TableStyle tableStyle = this.parent.getPresentation().tableStyleFactory().getTableStyleByStyleId(styleId);
        let mut tableStyle: TableStyle = self.parent.getPresentation().tableStyleFactory().getTableStyleByStyleId(styleId);
        // java if (null != tableStyle)
        if true {
          // java this.parent.getPresentation().tableStyleLst().addStyle(tableStyle);
          self.parent.getPresentation().tableStyleLst().addStyle(tableStyle);
        }
      }
      // java this.root.addChild(tableShape);
      self.root.addChild(tableShape);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(tableShape);
      self.parent.getPresentation().selection().selectShape(tableShape);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public void addChart(int type, int styleId, int colorId, int width, int height)
    pub fn addChart(&mut self, r#type: (), styleId: i32, colorId: i32, width: i32, height: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int slideWidth = this.parent.getPresentation().slideWidth();
      let mut slideWidth: i32 = self.parent.getPresentation().slideWidth();
      // java int slideHeight = this.parent.getPresentation().slideHeight();
      let mut slideHeight: i32 = self.parent.getPresentation().slideHeight();
      // java int chartWidth = width;
      let mut chartWidth: i32 = width;
      // java int chartHeight = height;
      let mut chartHeight: i32 = height;
      // java int twentyThree = 8280400;
      let mut twentyThree: i32 = 8280400;
      // java if (slideWidth <= twentyThree && slideWidth <= width)
      if slideWidth <= twentyThree && slideWidth <= width {
        // java chartWidth = 2 * slideWidth / 3;
        chartWidth = 2 * slideWidth / 3;
        // java chartHeight = (int)((float)height / (float)width * (float)chartWidth);
        chartHeight = ((((height) as f32) / ((width) as f32) * ((chartWidth) as f32)) as i32);
      }
      // java else if (slideHeight <= twentyThree && slideHeight <= height)
      else if slideHeight <= twentyThree && slideHeight <= height {
        // java chartHeight = 2 * slideHeight / 3;
        chartHeight = 2 * slideHeight / 3;
        // java chartWidth = (int)((float)width / (float)height * (float)chartHeight);
        chartWidth = ((((width) as f32) / ((height) as f32) * ((chartHeight) as f32)) as i32);
      }
      // java int left = (slideWidth - chartWidth) / 2;
      let mut left: i32 = slideWidth - chartWidth / 2;
      // java int top = (slideHeight - chartHeight) / 2;
      let mut top: i32 = slideHeight - chartHeight / 2;
      // java DgCanvas dgCanvas = this.parent.getPresentation().canvas();
      let mut dgCanvas: DgCanvas = self.parent.getPresentation().canvas();
      // java int chartShapeId = dgCanvas.addChart(left, top, chartWidth, chartHeight);
      let mut chartShapeId: i32 = dgCanvas.addChart(left, top, chartWidth, chartHeight);
      // java KmoShowShape chartShape = new KmoShowShape(this.parent, 4, chartShapeId);
      let mut chartShape: KmoShowShape = KmoShowShape::new(self.parent, 4, chartShapeId);
      // java KmoOleChart kmoOleChart = (KmoOleChart)chartShape.graphic();
      let mut kmoOleChart: KmoOleChart = (chartShape.graphic()).downcast_ref::<KmoOleChart>().unwrap();
      // java KmoTheme theme = this.parent.theme();
      let mut theme: KmoTheme = self.parent.theme();
      // java kmoOleChart.create(type, styleId, colorId, theme.getThemePath(), theme.getRid2MediaId());
      kmoOleChart.create(r#type, styleId, colorId, theme.getThemePath(), theme.getRid2MediaId());
      // java this.root.addChild(chartShape);
      self.root.addChild(chartShape);
      // java this.parent.getPresentation().setDirty();
      self.parent.getPresentation().setDirty();
      // java this.parent.getPresentation().selection().selectShape(chartShape);
      self.parent.getPresentation().selection().selectShape(chartShape);
      // java this.parent.eventListeners().onSlideContentsChanged();
      self.parent.eventListeners().onSlideContentsChanged();
      */
    }

    // java public KmoShowShape shapeAt(int index)
    pub fn shapeAt(&self, index: i32) -> &KmoShowShape {
      // java return this.root.childAt(index);
      return self.root.childAt(index);
    }

    // java public int indexOfShape(KmoShowShape sp)
    pub fn indexOfShape(&self, sp: &KmoShowShape) -> i32 {
      // java return this.root.indexOfChild(sp);
      return self.root.indexOfChild(sp);
    }

    // java public int shapeCount()
    pub fn shapeCount(&self) -> i32 {
      // java return this.root.childCount();
      return self.root.childCount();
    }

    // java public void addShape(KmoShowShape shape)
    pub fn addShape(&mut self, shape: KmoShowShape) {
      // java this.root.addChild(shape);
      self.root.addChild(shape);
    }

    // java public void addShapes(List<KmoShowShape> shapes)
    pub fn addShapes(&mut self, shapes: Vec<KmoShowShape>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.root.addChilds(shapes);
      self.root.addChilds(shapes);
      */
    }

    // java public void insertShapes(List<KmoShowShape> shapes, int index)
    pub fn insertShapes(&mut self, shapes: Vec<KmoShowShape>, index: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.root.insertChilds(index, shapes);
      self.root.insertChilds(index, shapes);
      */
    }

    // java public KmoShowShape findPhByType(int type)
    pub fn findPhByType(&self, r#type: ()) -> Option<&KmoShowShape> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (type < 0)
      if r#type < 0 {
        // java return null;
        return None;
      }
      // java else
      else {
        // java int i = 0;
        let mut i: i32 = 0;
        let mut count: i32 = self.root.childCount();
        // java for (int count = this.root.childCount(); i < count ++i)
        while i < count {
          // java KmoShowShape sp = this.root.childAt(i);
          let mut sp: KmoShowShape = self.root.childAt(i);
          // java if (sp.isPlaceHolder() && sp.ph().type() == type)
          if sp.isPlaceHolder() && sp.ph().r#type() == r#type {
            // java return sp;
            return Some(&sp);
          }
        }
        // java return null;
        return None;
      }
      */
    }

    // java public KmoShowShape findPhByIndex(int index)
    pub fn findPhByIndex(&self, index: i32) -> Option<&KmoShowShape> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (index < 0)
      if index < 0 {
        // java return null;
        return None;
      }
      // java else
      else {
        // java int i = 0;
        let mut i: i32 = 0;
        let mut count: i32 = self.root.childCount();
        // java for (int count = this.root.childCount(); i < count ++i)
        while i < count {
          // java KmoShowShape sp = this.root.childAt(i);
          let mut sp: KmoShowShape = self.root.childAt(i);
          // java if (sp.isPlaceHolder() && sp.ph().idx() == index)
          if sp.isPlaceHolder() && sp.ph().idx() == index {
            // java return sp;
            return Some(&sp);
          }
        }
        // java return null;
        return None;
      }
      */
    }

    // java public void deleteShape(KmoShowShape sp)
    pub fn deleteShape(&mut self, sp: &mut KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.deleteShape(sp, true);
      self.deleteShape__kmoshowshape_boolean(sp, true);
      */
    }

    // java public void deleteShape(KmoShowShape sp, boolean needNotify)
    pub fn deleteShape__showshape_boolean(&mut self, sp: &mut KmoShowShape, needNotify: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.deleteShape(sp, needNotify, true);
      self.deleteShape__kmoshowshape_boolean_boolean(sp, needNotify, true);
      */
    }

    // java public void deleteShape(KmoShowShape sp, boolean needNotify, boolean setDirty)
    pub fn deleteShape__showshape_boolean_boolean(&mut self, sp: &mut KmoShowShape, needNotify: bool, setDirty: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (sp.type() != 3 && sp.type() != 0)
      if sp.r#type() != 3 && sp.r#type() != 0 {
        // java DgGroup dgShape = this.root.getDgGroup();
        let mut dgShape: DgGroup = self.root.getDgGroup();
        // java dgShape.disconnect(sp.id());
        dgShape.disconnect(sp.id());
      }
      // java KmoShowShape pSp = sp.parentGroup();
      let mut pSp: KmoShowShape = sp.parentGroup();
      // java if (pSp == null)
      if false {
        // java pSp = this.root;
        pSp = self.root;
      }
      // java boolean remove = pSp.removeChildAt(pSp.indexOfChild(sp));
      let mut remove: bool = pSp.removeChildAt(pSp.indexOfChild(sp));
      // java if (remove)
      if remove {
        // java this.parent.getPresentation().inkLib().removeInk(pSp.inkId());
        self.parent.getPresentation().inkLib().removeInk(pSp.inkId());
      }
      // java if (remove && pSp != this.root && pSp.type() == 0)
      if remove && pSp != self.root && pSp.r#type() == 0 {
        // java if (pSp.childCount() == 1)
        if pSp.childCount() == 1 {
          // java this.ungroupself(pSp, false, setDirty);
          self.ungroupself(&mut pSp, false, setDirty);
        }
        // java else
        else {
          // java this.updateParentXfrm(pSp);
          self.updateParentXfrm(&mut pSp);
        }
      }
      // java if (setDirty)
      if setDirty {
        // java this.parent.getPresentation().setDirty();
        self.parent.getPresentation().setDirty();
      }
      // java sp.parent().eventListeners().onShapeContentsChanged(7, sp);
      sp.parent().eventListeners().onShapeContentsChanged(7, sp);
      // java if (needNotify)
      if needNotify {
        // java this.parent.eventListeners().onSlideContentsChanged();
        self.parent.eventListeners().onSlideContentsChanged();
      }
      */
    }

    // java private void updateParentXfrm(KmoShowShape shape)
    pub fn updateParentXfrm(&mut self, shape: &mut KmoShowShape) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java shape.getDgGroup().adjustGroupXfrm();
      shape.getDgGroup().adjustGroupXfrm();
      // java this.parent.eventListeners().onShapeContentsChanged(1, shape);
      self.parent.eventListeners().onShapeContentsChanged(1, shape);
      // java KmoShowShape parent = shape.parentGroup();
      let mut parent: KmoShowShape = shape.parentGroup();
      // java if (parent != null)
      if let Some(parent) = self.parent.as_mut() {
        // java Transform2D parentXfrm = parent.transform2D();
        let mut parentXfrm: Transform2D = parent.transform2D();
        // java Transform2D selfXfrm = shape.transform2D();
        let mut selfXfrm: Transform2D = shape.transform2D();
        // java if (parentXfrm.extents().cx() != selfXfrm.extents().cx() || parentXfrm.extents().cy() != selfXfrm.extents().cy())
        if parentXfrm.extents().cx() != selfXfrm.extents().cx() || parentXfrm.extents().cy() != selfXfrm.extents().cy() {
          // java this.updateParentXfrm(parent);
          self.updateParentXfrm(&mut parent);
        }
      }
      */
    }

    // java public ArrayList<KmoShowShape> getAllPh()
    pub fn getAllPh(&self) -> Vec<KmoShowShape> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ArrayList<KmoShowShape> phs = new ArrayList();
      let mut phs: Vec<KmoShowShape> = { let __x: Vec<KmoShowShape> = todo!("no align strategy matched"); __x };
      // java int i = 0;
      let mut i: i32 = 0;
      let mut count: i32 = self.root.childCount();
      // java for (int count = this.root.childCount(); i < count ++i)
      while i < count {
        // java KmoShowShape shape = this.root.childAt(i);
        let mut shape: KmoShowShape = self.root.childAt(i);
        // java if (shape.isPlaceHolder())
        if shape.isPlaceHolder() {
          // java phs.add(shape);
          phs.add(shape);
        }
      }
      // java return phs;
      return phs;
      */
    }

    // java public KmoVmlDrawing getVmlDrawing()
    pub fn getVmlDrawing(&self) -> Option<&KmoVmlDrawing> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.vmlDrawing;
      return self.vmlDrawing.as_ref();
      */
    }

    // java public void setVmlDrawing(KmoVmlDrawing vmlDrawing)
    pub fn setVmlDrawing(&mut self, vmlDrawing: Option<KmoVmlDrawing>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.vmlDrawing = vmlDrawing;
      self.vmlDrawing = vmlDrawing;
      */
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.root.hashCode();
      return self.root.hashCode();
      */
    }

    // java public boolean equals(Object o)
    pub fn equals(&self, o: Option<&dyn std::any::Any>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.equals(o, true);
      return self.equals__object_boolean(o, true);
      */
    }

    // java public boolean equalsLax(KmoShapeTree tree)
    pub fn equalsLax(&self, tree: &KmoShapeTree) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.equals(tree, false);
      return self.equals__object_boolean({ let __x: Option<&dyn std::any::Any> = todo!("no align strategy matched"); __x }, false);
      */
    }

    // java private boolean equals(Object o, boolean isStrict)
    pub fn equals__object_boolean(&self, o: Option<&dyn std::any::Any>, isStrict: bool) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != o && o instanceof KmoShapeTree)
      if let Some(o) = o {
        if o.downcast_ref::<KmoShapeTree>().is_some() {
          // java KmoShapeTree dstSpTree = (KmoShapeTree)o;
          let mut dstSpTree: KmoShapeTree = (o).downcast_ref::<KmoShapeTree>().unwrap();
          // java if (this == dstSpTree)
          if self == dstSpTree {
            // java return true;
            return true;
          }
          // java else
          else {
            // java KmoShowShape srcRoot = this.getRoot();
            let mut srcRoot: KmoShowShape = self.getRoot().clone();
            // java KmoShowShape dstRoot = dstSpTree.getRoot();
            let mut dstRoot: KmoShowShape = dstSpTree.getRoot().clone();
            // java if ((null != srcRoot || null == dstRoot) && (null == srcRoot || null != dstRoot))
            if true || false && false || true {
              // java if (null != srcRoot && null != dstRoot)
              if true && true {
                // java if (isStrict && !srcRoot.equals(dstRoot))
                if isStrict && !srcRoot.equals(dstRoot) {
                  // java return false;
                  return false;
                }
                // java if (!isStrict && !srcRoot.equalsLax(dstRoot))
                if !isStrict && !srcRoot.equalsLax(dstRoot) {
                  // java return false;
                  return false;
                }
              }
              // java return true;
              return true;
            }
            // java else
            else {
              // java return false;
              return false;
            }
          }
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
      */
    }

}
