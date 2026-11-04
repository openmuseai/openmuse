// Java: import cn.wps.graphics.PointF
use util::graphics::PointF::PointF as GraphicsPointF;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.ShapeDefaultProperties
use drawing::cn::wps::moffice::drawing::ShapeDefaultProperties::*;
// Java: import cn.wps.moffice.drawing.ZOrderCmd
use drawing::cn::wps::moffice::drawing::ZOrderCmd::*;
// Java: import cn.wps.moffice.drawing.core.EditType
use drawing::cn::wps::moffice::drawing::core::EditType::{EditType};
// Java: import cn.wps.moffice.drawing.core.HitPos
use drawing::cn::wps::moffice::drawing::core::HitPos::HitPos as DrawingHitPos;
// Java: import cn.wps.moffice.drawing.core.ShapeRange
use drawing::cn::wps::moffice::drawing::core::ShapeRange::*;
// Java: import cn.wps.moffice.transaction.Command
use util::moffice::transaction::Command::*;
// Java: import cn.wps.moffice.writer.VersionState
use crate::cn::wps::moffice::writer::VersionState::*;
// Java: import cn.wps.moffice.writer.core.DefaultModelEvent
use crate::cn::wps::moffice::writer::core::DefaultModelEvent::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.DocumentEvent
use crate::cn::wps::moffice::writer::core::DocumentEvent::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.shape.command.KPosCommand
use crate::cn::wps::moffice::writer::core::shape::command::KPosCommand::*;
// Java: import cn.wps.moffice.writer.data.CPieceTable
use crate::cn::wps::moffice::writer::data::CPieceTable::*;
// Java: import cn.wps.util.concurrent.locks.RangeWriteLock
use crate::cn::wps::util::concurrent::locks::RangeWriteLock::*;
// Java: import java.util.Vector
use stubs::java::util::Vector::*;
// Java: import cn.wps.moffice.writer.core.shape.KInlineShapes
use crate::cn::wps::moffice::writer::core::shape::KInlineShapes::*;
// Java: import cn.wps.moffice.writer.core.shape.KShapes
use crate::cn::wps::moffice::writer::core::shape::KShapes::*;
// Java: import cn.wps.moffice.writer.core.shape.KShape
use crate::cn::wps::moffice::writer::core::shape::KShape::*;
// Java: import cn.wps.moffice.writer.core.shape.KPictureFormat
use crate::cn::wps::moffice::writer::core::shape::KPictureFormat::*;
// Java: import cn.wps.moffice.writer.core.shape.KLineFormat
use crate::cn::wps::moffice::writer::core::shape::KLineFormat::*;
// Java: import cn.wps.moffice.writer.core.shape.KFillFormat
use crate::cn::wps::moffice::writer::core::shape::KFillFormat::*;
// Java: import cn.wps.moffice.writer.core.shape.KShapeScale
use crate::cn::wps::moffice::writer::core::shape::KShapeScale::*;
// Java: import cn.wps.moffice.writer.core.shape.KShapeAdjust
use crate::cn::wps::moffice::writer::core::shape::KShapeAdjust::*;
// Java: import cn.wps.moffice.writer.core.shape.KGroup
use crate::cn::wps::moffice::writer::core::shape::KGroup::*;
// Java: import cn.wps.moffice.writer.core.shape.WrapType
use crate::cn::wps::moffice::writer::core::shape::WrapType::*;
// Java: import cn.wps.moffice.writer.core.shape.KWrapSwitch
use crate::cn::wps::moffice::writer::core::shape::KWrapSwitch::*;

// java static ShapeDefaultProperties defaultShpPr;
pub(crate) static defaultShpPr: std::sync::OnceLock<std::sync::Arc<Option<ShapeDefaultProperties>>> =
                std::sync::OnceLock::new();

// java public class KShapeRange extends ShapeRange
#[derive(Default)]
pub struct KShapeRange {
    // java ShapeRange super_
    pub(crate) super_: ShapeRange,
    // java protected Document document;
    pub(crate) document: Option<Document>,
    // java protected CPieceTable mCPieceTable;
    pub(crate) mCPieceTable: Option<CPieceTable>,
    // java KInlineShapes mInlineShapes = null;
    pub(crate) mInlineShapes: Option<KInlineShapes>,
    // java KShapes mShapes = null;
    pub(crate) mShapes: Option<KShapes>,
    // java private final Vector<KShape> shapes = new Vector();
    shapes: Option<Vec<KShape>>,
    // java private final Vector<KShape> childShapes = new Vector();
    childShapes: Option<Vec<KShape>>,
    // java EditType type;
    pub(crate) r#type: Option<EditType>,
    // java GraphicsPointF ptLast;
    pub(crate) ptLast: Option<GraphicsPointF>,
    // java DrawingHitPos hitPos;
    pub(crate) hitPos: Option<DrawingHitPos>,
    // java private KShape curGrpShape;
    curGrpShape: Option<KShape>,
    // java private KShape curChildShape;
    curChildShape: Option<KShape>,
    // java private KShape curActionShape;
    curActionShape: Option<KShape>,
    // java private Shape curEditShape;
    curEditShape: Option<Shape>,
    // java private KShape curReadShape;
    curReadShape: Option<KShape>,
    // java KPictureFormat mPictrueFormat;
    pub(crate) mPictrueFormat: Option<KPictureFormat>,
    // java KLineFormat mLine;
    pub(crate) mLine: Option<KLineFormat>,
    // java KFillFormat mFill;
    pub(crate) mFill: Option<KFillFormat>,
    // java boolean bChange;
    pub(crate) bChange: bool,
    // java float mRot;
    pub(crate) mRot: f32,
    // java Vector<Float> mRots;
    pub(crate) mRots: Option<Vec<f32>>,
    // java KShapeScale mShapeScale;
    pub(crate) mShapeScale: Option<KShapeScale>,
    // java KShapeAdjust mShapeAdjust;
    pub(crate) mShapeAdjust: Option<KShapeAdjust>,
    // java KGroup mGroup;
    pub(crate) mGroup: Option<KGroup>,
    // java private KRange mKRange;
    mKRange: Option<KRange>,
}

// java public class KShapeRange extends ShapeRange
impl KShapeRange {
    // java public final Document getStory()
    pub fn getStory(&self) -> Box<dyn Document> {
      // java return this.document;
      return self.document;
    }

    // java public final int getStoryType()
    pub fn getStoryType(&self) -> i32 {
      // java return this.getStory().getType();
      return self.getStory().getType();
    }

    // java public final int getStoryLength()
    pub fn getStoryLength(&self) -> i32 {
      // java return this.getStory().getLength();
      return self.getStory().getLength();
    }

    // java public final Document getMainDocument()
    pub fn getMainDocument(&self) -> Box<dyn Document> {
      // java return this.document.getMainDocument();
      return self.document.getMainDocument();
    }

    // java public final TextDocument getTextDocument()
    pub fn getTextDocument(&self) -> TextDocument {
      // java return this.document.getTextDocument();
      return self.document.getTextDocument();
    }

    // java public final void setSubDocument(Document document)
    pub fn setSubDocument(&mut self, document: Option<&dyn Document>) {
      // java this.document = document;
      self.document = document;
      // java if (document != null)
      if let Some(document) = document {
        // java this.mCPieceTable = document.getPieceTable();
        self.mCPieceTable = document.getPieceTable();
      }
    }

    // java public  KShapeRange(Document document, KRange range)
    pub fn KShapeRange(document: &dyn Document, range: &KRange) -> Self {
      // java this.type = EditType.type_none;
      Self { super_: ShapeRange::new(), document: Some(document), mCPieceTable: Default::default(), mInlineShapes: Default::default(), mShapes: Default::default(), childShapes: Default::default(), r#type: EditType::type_none, ptLast: GraphicsPointF::new(), hitPos: DrawingHitPos::None, curGrpShape: Default::default(), curChildShape: None, curActionShape: None, curEditShape: None, curReadShape: None, mPictrueFormat: None, mLine: None, mFill: None, bChange: false, mRot: 0, mRots: Default::default(), mShapeScale: KShapeScale::new(self), mShapeAdjust: KShapeAdjust::new(self), mGroup: KGroup::new(self), mKRange: range }
      // java this.ptLast = new GraphicsPointF();
      // java this.hitPos = DrawingHitPos.None;
      // java this.curChildShape = null;
      // java this.curActionShape = null;
      // java this.curEditShape = null;
      // java this.curReadShape = null;
      // java this.mPictrueFormat = null;
      // java this.mLine = null;
      // java this.mFill = null;
      // java this.bChange = false;
      // java this.mRot = 0.0F;
      // java this.document = document;
      // java this.mShapeScale = new KShapeScale(this);
      // java this.mShapeAdjust = new KShapeAdjust(this);
      // java this.mGroup = new KGroup(this);
      // java this.mKRange = range;
    }

    // java public KShapeRange editShapeRange()
    pub fn editShapeRange(&self) -> KShapeRange {
      // java if (this.count() <= 0 && this.curEditShape != null)
      if let Some(curEditShape) = self.curEditShape.as_ref() {
        if self.count() <= 0 {
          // java KShapeRange shapeRange = new KShapeRange(this.document, (KRange)null);
          let mut shapeRange: KShapeRange = KShapeRange::new(self.document, None);
          // java shapeRange.addShape(new KShape(this.curEditShape));
          shapeRange.addShape(KShape::new(curEditShape));
          // java return shapeRange;
          return shapeRange;
        }
      }
      // java else
      else {
        // java return this;
        return self;
      }
    }

    // java public KShape item(int n)
    pub fn item(&mut self, n: i32) -> Option<KShape> {
      // java return n >= 0 && n < this.shapes.size() ? (KShape)this.shapes.elementAt(n) : null;
      return (if n >= 0 && n < (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) { Some((self.shapes.elementAt(n)).downcast_ref::<KShape>().unwrap()) } else { None });
    }

    // java public EditType getEditType()
    pub fn getEditType(&self) -> EditType {
      // java return this.type;
      return self.r#type.unwrap();
    }

    // java public boolean canOpenOle(Shape shape)
    pub fn canOpenOle(&mut self, shape: &Shape) -> bool {
      // java return ShapeTool.canOpenOle(shape);
      return ShapeTool::canOpenOle(shape);
    }

    // java public void setEditType(EditType type)
    pub fn setEditType(&mut self, r#type: &EditType) {
      // java this.type = type;
      self.r#type = r#type;
    }

    // java public KShape getCurShape()
    pub fn getCurShape(&self) -> KShape {
      // java return this.curActionShape;
      return self.curActionShape;
    }

    // java public void setCurShape(KShape kShape)
    pub fn setCurShape(&mut self, kShape: &KShape) {
      // java this.curActionShape = kShape;
      self.curActionShape = kShape;
    }

    // java public void setCurGrpShape(KShape kShape)
    pub fn setCurGrpShape(&mut self, kShape: &KShape) {
      // java this.curGrpShape = kShape;
      self.curGrpShape = kShape;
    }

    // java public KShape getCurGrpShape()
    pub fn getCurGrpShape(&self) -> KShape {
      // java return this.curGrpShape;
      return self.curGrpShape;
    }

    // java public void setCurReadShape(KShape kShape)
    pub fn setCurReadShape(&mut self, kShape: &KShape) {
      // java this.curReadShape = kShape;
      self.curReadShape = kShape;
    }

    // java public KShape getCurReadShape()
    pub fn getCurReadShape(&self) -> KShape {
      // java return this.curReadShape;
      return self.curReadShape;
    }

    // java public void clearChildItems()
    pub fn clearChildItems(&mut self) {
      // java this.childShapes.clear();
      self.childShapes.clear();
    }

    // java public KShape childItem(int n)
    pub fn childItem(&mut self, n: i32) -> Option<KShape> {
      // java return n >= 0 && n < this.childShapes.size() ? (KShape)this.childShapes.elementAt(n) : null;
      return (if n >= 0 && n < (self.childShapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) { Some((self.childShapes.elementAt(n)).downcast_ref::<KShape>().unwrap()) } else { None });
    }

    // java public int childCount()
    pub fn childCount(&self) -> i32 {
      // java return this.childShapes.size();
      return (self.childShapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
    }

    // java public void addChildShape(KShape kShape, boolean isMultiSelect)
    pub fn addChildShape(&mut self, kShape: Option<&KShape>, isMultiSelect: bool) {
      // java this.curChildShape = kShape;
      self.curChildShape = kShape;
      // java if (isMultiSelect)
      if isMultiSelect {
        // java if (kShape == null)
        if kShape.is_none() {
          // java return;
          return;
        }
        // java int size = this.childShapes.size();
        let mut size: i32 = (self.childShapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
        let mut i: i32 = 0;
        // java for(int i = 0; i < size; ++i)
        while i < size {
          // java KShape coreShape = (KShape)this.childShapes.elementAt(i);
          let mut coreShape: KShape = (self.childShapes.elementAt(i)).downcast_ref::<KShape>().unwrap();
          // java if (coreShape.getShape().getShapeID() == kShape.getShape().getShapeID())
          if coreShape.getShape().getShapeID() == kShape.getShape().getShapeID() {
            // java return;
            return;
          }
        }
        // java this.childShapes.add(kShape);
        self.childShapes.add(kShape);
      }
      // java else
      else {
        // java this.childShapes.clear();
        self.childShapes.clear();
        // java this.childShapes.add(kShape);
        self.childShapes.add(kShape);
      }
    }

    // java public void removeChildItem(KShape shape)
    pub fn removeChildItem(&mut self, shape: Option<&KShape>) {
      // java if (shape != null)
      if let Some(shape) = shape {
        // java int size = this.childShapes.size();
        let mut size: i32 = (self.childShapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
        let mut i: i32 = 0;
        // java for(int i = 0; i < size; ++i)
        while i < size {
          // java KShape coreShape = (KShape)this.childShapes.elementAt(i);
          let mut coreShape: KShape = (self.childShapes.elementAt(i)).downcast_ref::<KShape>().unwrap();
          // java if (coreShape.getShape().getShapeID() == shape.getShape().getShapeID())
          if coreShape.getShape().getShapeID() == shape.getShape().getShapeID() {
            // java this.childShapes.remove(i);
            self.childShapes.remove(i);
            // java return;
            return;
          }
        }
      }
    }

    // java public KShape getCurChildShape()
    pub fn getCurChildShape(&self) -> KShape {
      // java return this.curChildShape;
      return self.curChildShape;
    }

    // java public int count()
    pub fn count(&self) -> i32 {
      // java return this.shapes.size();
      return (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
    }

    // java public void addShape(KShape shape)
    pub fn addShape(&mut self, shape: Option<&KShape>) {
      // java if (shape != null)
      if let Some(shape) = shape {
        // java int size = this.shapes.size();
        let mut size: i32 = (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
        let mut i: i32 = 0;
        // java for(int i = 0; i < size; ++i)
        while i < size {
          // java KShape coreShape = (KShape)this.shapes.elementAt(i);
          let mut coreShape: KShape = (self.shapes.elementAt(i)).downcast_ref::<KShape>().unwrap();
          // java if (coreShape.getShape().getShapeID() == shape.getShape().getShapeID())
          if coreShape.getShape().getShapeID() == shape.getShape().getShapeID() {
            // java return;
            return;
          }
        }
        // java this.shapes.add(shape);
        self.shapes.add(shape);
      }
    }

    // java public void removeShape(KShape shape)
    pub fn removeShape(&mut self, shape: Option<&KShape>) {
      // java if (shape != null)
      if let Some(shape) = shape {
        // java int size = this.shapes.size();
        let mut size: i32 = (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
        let mut i: i32 = 0;
        // java for(int i = 0; i < size; ++i)
        while i < size {
          // java KShape coreShape = (KShape)this.shapes.elementAt(i);
          let mut coreShape: KShape = (self.shapes.elementAt(i)).downcast_ref::<KShape>().unwrap();
          // java if (coreShape.getShape().getShapeID() == shape.getShape().getShapeID())
          if coreShape.getShape().getShapeID() == shape.getShape().getShapeID() {
            // java this.shapes.remove(i);
            self.shapes.remove(i);
            // java return;
            return;
          }
        }
      }
    }

    // java public Shape setcurEditShape(Shape shape)
    pub fn setcurEditShape(&mut self, shape: &Shape) -> Shape {
      // java Shape oldShape = this.curEditShape;
      let mut oldShape: Shape = self.curEditShape;
      // java this.curEditShape = shape;
      self.curEditShape = shape;
      // java return oldShape;
      return oldShape;
    }

    // java public Shape getcurEditShape()
    pub fn getcurEditShape(&self) -> Shape {
      // java return this.curEditShape;
      return self.curEditShape;
    }

    // java public KPictureFormat getPictrueFormat()
    pub fn getPictrueFormat(&mut self) -> KPictureFormat {
      // java if (this.mPictrueFormat == null)
      if self.mPictrueFormat.is_none() {
        // java this.mPictrueFormat = new KPictureFormat(this);
        self.mPictrueFormat = KPictureFormat::new(self);
      }
      // java return this.mPictrueFormat;
      return self.mPictrueFormat;
    }

    // java public void start(boolean bLock)
    pub fn start(&self, bLock: bool) {
      // java this.getTextDocument().start(bLock);
      self.getTextDocument().start(bLock);
    }

    // java public void commit(String description, boolean bUnlock)
    pub fn commit(&self, description: &str, bUnlock: bool) {
      // java this.getTextDocument().commit(description, bUnlock);
      self.getTextDocument().commit(description, bUnlock);
    }

    // java public void cancel()
    pub fn cancel(&self) {
      // java this.getTextDocument().rollback();
      self.getTextDocument().rollback();
    }

    // java public void cancel(boolean bRollback)
    pub fn cancel__boolean(&self, bRollback: bool) {
      // java this.getTextDocument().rollback(bRollback);
      self.getTextDocument().rollback(bRollback);
    }

    // java public HitPos getHitPos()
    pub fn getHitPos(&self) -> DrawingHitPos {
      // java return this.hitPos;
      return self.hitPos;
    }

    // java public boolean beginRotate()
    pub fn beginRotate(&mut self) -> bool {
      // java if (this.shapes.size() != 0 && this.type == EditType.type_none)
      if (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) != 0 && self.r#type == EditType::type_none {
        // java this.start(false);
        self.start(false);
        // java this.bChange = false;
        self.bChange = false;
        // java this.type = EditType.type_rotation;
        self.r#type = EditType::type_rotation;
        // java this.getRotations();
        self.getRotations();
        // java return true;
        return true;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java private void getRotations()
    pub fn getRotations(&mut self) {
      // java int size = this.shapes.size();
      let mut size: i32 = (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java this.mRots = new Vector();
      self.mRots = Vec::new();
      // java this.mRots.setSize(size);
      self.mRots.setSize(size);
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java KShape coreShape = (KShape)this.shapes.elementAt(i);
        let mut coreShape: KShape = (self.shapes.elementAt(i)).downcast_ref::<KShape>().unwrap();
        // java if (coreShape != null)
        if coreShape.is_some() {
          // java this.mRots.set(i, coreShape.getShape().getBaseShape().getShapePos().getRotation());
          self.mRots.set(i, coreShape.getShape().getBaseShape().getShapePos().getRotation());
        }
      }
    }

    // java private void genRotUndoCommand()
    pub fn genRotUndoCommand(&mut self) {
      // java int size = this.shapes.size();
      let mut size: i32 = (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java KShape coreShape = (KShape)this.shapes.elementAt(i);
        let mut coreShape: KShape = (self.shapes.elementAt(i)).downcast_ref::<KShape>().unwrap();
        // java if (coreShape != null)
        if coreShape.is_some() {
          // java Shape shape = coreShape.getShape().getBaseShape();
          let mut shape: Shape = coreShape.getShape().getBaseShape();
          // java this.getTextDocument().submit(new KPosCommand(shape, 24, this.mRots.elementAt(i), shape.getShapePos().getRotation()));
          self.getTextDocument().submit(KPosCommand::new(shape, 24, self.mRots.elementAt(i), shape.getShapePos().getRotation()));
        }
      }
    }

    // java public boolean rotate(float increment)
    pub fn rotate(&mut self, increment: f32) -> bool {
      // java RangeWriteLock lock = this.getMainDocument().writeLock();
      let mut lock: RangeWriteLock = self.getMainDocument().writeLock();
      // java boolean var3;
      let mut var3: bool;
          // java var3 = this.rotate_locked(increment);
          var3 = self.rotate_locked(increment);
            // java lock.unlock();
            lock.unlock();
      // java return var3;
      return var3;
    }

    // java private boolean rotate_locked(float increment)
    pub fn rotate_locked(&mut self, increment: f32) -> bool {
      // java int size = this.shapes.size();
      let mut size: i32 = (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java if (size == 0)
      if size == 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        let mut i: i32 = 0;
        // java for(int i = 0; i < size; ++i)
        while i < size {
          // java KShape coreShape = (KShape)this.shapes.elementAt(i);
          let mut coreShape: KShape = (self.shapes.elementAt(i)).downcast_ref::<KShape>().unwrap();
          // java if (coreShape != null)
          if coreShape.is_some() {
            // java Shape shape = coreShape.getShape().getBaseShape();
            let mut shape: Shape = coreShape.getShape().getBaseShape();
            // java int rot = (int)shape.getShapePos().getRotation();
            let mut rot: i32 = ((shape.getShapePos().getRotation()) as i32);
            // java rot = (int)((float)rot + increment);
            rot = ((((rot) as f32) + increment) as i32);
            // java rot %= 360;
            rot = rot % 360;
            // java this.bChange = true;
            self.bChange = true;
            // java shape.getShapePos().setRotation((float)rot);
            shape.getShapePos().setRotation(((rot) as f32));
          }
        }
        // java this.updateRanges(15, false);
        self.updateRanges__int_boolean(15, false);
        // java return true;
        return true;
      }
    }

    // java public int getTextBoxCpStart()
    pub fn getTextBoxCpStart(&mut self) -> i32 {
      // java return ((KShape)this.shapes.elementAt(0)).getTextBoxCpStart();
      return (self.shapes.elementAt(0)).downcast_ref::<KShape>().unwrap().getTextBoxCpStart();
    }

    // java public int getTextBoxVisitableCpStart()
    pub fn getTextBoxVisitableCpStart(&mut self) -> i32 {
      // java return this.childShapes.size() > 0 ? ((KShape)this.childShapes.elementAt(0)).getTextBoxVisitableCpStart() : ((KShape)this.shapes.elementAt(0)).getTextBoxVisitableCpStart();
      return (if (self.childShapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 { (self.childShapes.elementAt(0)).downcast_ref::<KShape>().unwrap().getTextBoxVisitableCpStart() } else { (self.shapes.elementAt(0)).downcast_ref::<KShape>().unwrap().getTextBoxVisitableCpStart() });
    }

    // java public int getTextBoxCpEnd()
    pub fn getTextBoxCpEnd(&mut self) -> i32 {
      // java return ((KShape)this.shapes.elementAt(0)).getTextBoxCpEnd();
      return (self.shapes.elementAt(0)).downcast_ref::<KShape>().unwrap().getTextBoxCpEnd();
    }

    // java public boolean endRotate()
    pub fn endRotate(&mut self) -> bool {
      // java if (this.shapes.size() != 0 && this.type == EditType.type_rotation)
      if (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) != 0 && self.r#type == EditType::type_rotation {
        // java this.type = EditType.type_none;
        self.r#type = EditType::type_none;
        // java this.hitPos = DrawingHitPos.None;
        self.hitPos = DrawingHitPos::None;
        // java if (this.bChange)
        if self.bChange {
          // java this.genRotUndoCommand();
          self.genRotUndoCommand();
          // java this.updateRanges(true);
          self.updateRanges__boolean(true);
          // java this.commit("rotate shape", false);
          self.commit(&"rotate shape", false);
        }
        // java else
        else {
          // java this.cancel();
          self.cancel();
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

    // java public void rotation(float increment)
    pub fn rotation(&mut self, increment: f32) {
      // java if (this.shapes.size() != 0)
      if (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) != 0 {
        // java this.clearGroupState();
        self.clearGroupState();
        // java this.start(true);
        self.start(true);
            // java this.getRotations();
            self.getRotations();
            // java this.rotate(increment);
            self.rotate(increment);
            // java this.genRotUndoCommand();
            self.genRotUndoCommand();
            // java this.updateRanges(true);
            self.updateRanges__boolean(true);
              // java this.commit("rotate shape", true);
              self.commit(&"rotate shape", true);
      }
    }

    // java public static void updateShapeRange(Shape shape, boolean bSubmit)
    pub fn updateShapeRange__shape_boolean(shape: &Shape, bSubmit: bool) {
      // java updateShapeRange(shape, 17, bSubmit);
      Self::updateShapeRange__shape_int_boolean(&shape, 17, bSubmit);
    }

    // java protected static void updateShapeRange(Shape shape, int subEventType, boolean bSubmit)
    pub fn updateShapeRange__shape_int_boolean(shape: &Shape, subEventType: i32, bSubmit: bool) {
      // java Document doc = (Document)shape.getSubDocument();
      let mut doc: Box<dyn Document> = todo!("cast → Box<dyn Document>");
      // java int cp = ShapeTool.position(doc, shape);
      let mut cp: i32 = ShapeTool::position(Some(&doc), Some(&shape));
      // java updateRange(doc, cp, cp + 1, subEventType, bSubmit);
      Self::updateRange(&doc, cp, cp + 1, subEventType, bSubmit);
    }

    // java public static void updateRange(Document document, int start, int end, int subEventType, boolean bSubmit)
    pub fn updateRange(document: &dyn Document, start: i32, end: i32, subEventType: i32, bSubmit: bool) {
      // java class CmdUpdateRange extends Command { Document document; int start; int end; int subEventType; public CmdUpdateRange(Document document, int start, int end, int subEventType) { this.document = document; this.start = start; this.end = end; this.subEventType = subEventType; } public void execute() { DocumentEvent e = DefaultModelEvent.obtain(this.document, this.start, this.end - this.start, 6, this.subEventType); this.document.fireDocumentUpdate(e); e.decreaseRef(); } public void unexecute() { this.execute(); }
      todo!();
      // java CmdUpdateRange cmd = new CmdUpdateRange(document, start, end, subEventType);
      let mut cmd: CmdUpdateRange = CmdUpdateRange::new(document, start, end, subEventType);
      // java cmd.execute();
      cmd.execute();
      // java if (bSubmit)
      if bSubmit {
        // java document.getTextDocument().getTransaction().submitCommand(cmd);
        document.getTextDocument().getTransaction().submitCommand(cmd);
      }
    }

    // java protected void updateRanges(int subEventType, boolean bSubmit)
    pub fn updateRanges__int_boolean(&mut self, subEventType: i32, bSubmit: bool) {
      // java int size = this.shapes.size();
      let mut size: i32 = (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java KShape coreShape = (KShape)this.shapes.elementAt(i);
        let mut coreShape: KShape = (self.shapes.elementAt(i)).downcast_ref::<KShape>().unwrap();
        // java if (coreShape != null)
        if coreShape.is_some() {
          // java Shape shape = coreShape.getShape().getBaseShape();
          let mut shape: Shape = coreShape.getShape().getBaseShape();
          // java updateShapeRange(shape, subEventType, bSubmit);
          Self::updateShapeRange__shape_int_boolean(&shape, subEventType, bSubmit);
        }
      }
    }

    // java protected void updateRanges(boolean bSubmit)
    pub fn updateRanges__boolean(&mut self, bSubmit: bool) {
      // java this.updateRanges(17, bSubmit);
      self.updateRanges__int_boolean(17, bSubmit);
    }

    // java public boolean isInlineShape()
    pub fn isInlineShape(&mut self) -> bool {
      // java return this.shapes.size() > 0 && ((KShape)this.shapes.elementAt(0)).isInlineShape();
      return (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 && (self.shapes.elementAt(0)).downcast_ref::<KShape>().unwrap().isInlineShape();
    }

    // java public boolean isPicture()
    pub fn isPicture(&mut self) -> bool {
      // java return this.shapes.size() > 0 && ((KShape)this.shapes.elementAt(0)).isPicture();
      return (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 && (self.shapes.elementAt(0)).downcast_ref::<KShape>().unwrap().isPicture();
    }

    // java public boolean isTextBox()
    pub fn isTextBox(&mut self) -> bool {
      // java return this.shapes.size() > 0 && ((KShape)this.shapes.elementAt(0)).isTextBox();
      return (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 && (self.shapes.elementAt(0)).downcast_ref::<KShape>().unwrap().isTextBox();
    }

    // java public boolean isGroupShape()
    pub fn isGroupShape(&mut self) -> bool {
      // java return this.shapes.size() > 0 && ((KShape)this.shapes.elementAt(0)).isGroupShape();
      return (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 && (self.shapes.elementAt(0)).downcast_ref::<KShape>().unwrap().isGroupShape();
    }

    // java public boolean isInk()
    pub fn isInk(&mut self) -> bool {
      // java return this.shapes.size() > 0 && ((KShape)this.shapes.elementAt(0)).hasInk();
      return (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 && (self.shapes.elementAt(0)).downcast_ref::<KShape>().unwrap().hasInk();
    }

    // java public boolean beginScale(KShape coreShape, DrawingHitPos hitPos, float x, float y)
    pub fn beginScale(&mut self, coreShape: &KShape, hitPos: &DrawingHitPos, x: f32, y: f32) -> bool {
      // java this.clearGroupState();
      self.clearGroupState();
      // java return this.getShapeScale().beginScale(coreShape, hitPos, x, y);
      return self.getShapeScale().beginScale(coreShape, hitPos, x, y);
    }

    // java public boolean scale(KShape coreShape, GraphicsPointF from, GraphicsPointF to, boolean needRatio)
    pub fn scale(&self, coreShape: &KShape, from: &GraphicsPointF, to: &GraphicsPointF, needRatio: bool) -> bool {
      // java return this.getShapeScale().scale(coreShape, from, to, needRatio);
      return self.getShapeScale().scale(coreShape, from, to, needRatio);
    }

    // java public boolean isRatioScale()
    pub fn isRatioScale(&self) -> bool {
      // java return this.getShapeScale().mIsRatio;
      return self.getShapeScale().mIsRatio;
    }

    // java public boolean endScale(float x, float y)
    pub fn endScale(&self, x: f32, y: f32) -> bool {
      // java return this.getShapeScale().endScale(x, y);
      return self.getShapeScale().endScale(x, y);
    }

    // java public void zOrder(ZOrderCmd cmd)
    pub fn zOrder(&mut self, cmd: &ZOrderCmd) {
      // java if (this.shapes.size() != 0)
      if (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) != 0 {
        // java this.start(true);
        self.start(true);
            // java if (cmd != ZOrderCmd.BringInFrontOfText && cmd != ZOrderCmd.SendBehindText)
            if cmd != ZOrderCmd::BringInFrontOfText && cmd != ZOrderCmd::SendBehindText {
              // java int size = this.shapes.size();
              let mut size: i32 = (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
              // java int[] zIndexArr = new int[size];
              let mut zIndexArr: Vec<i32> = vec![Default::default(); (size) as usize];
              let mut i: i32 = 0;
              // java for(int i = 0; i < size; ++i)
              while i < size {
                // java KShape coreShape = (KShape)this.shapes.elementAt(i);
                let mut coreShape: KShape = (self.shapes.elementAt(i)).downcast_ref::<KShape>().unwrap();
                // java if (coreShape != null)
                if coreShape.is_some() {
                  // java zIndexArr[i] = coreShape.getShape().getBaseShape().getZIndex();
                  zIndexArr[(i) as usize] = coreShape.getShape().getBaseShape().getZIndex();
                }
              }
              // java this.getSubDocument().getDrawingContainer().zOrder(cmd, zIndexArr);
              self.getSubDocument().getDrawingContainer().zOrder(cmd, zIndexArr);
              // java this.updateRanges(true);
              self.updateRanges__boolean(true);
            }
            // java else
            else {
              // java boolean behindDocument = cmd == ZOrderCmd.SendBehindText;
              let mut behindDocument: bool = cmd == ZOrderCmd::SendBehindText;
              // java int size = this.shapes.size();
              let mut size: i32 = (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
              let mut i: i32 = 0;
              // java for(int i = 0; i < size; ++i)
              while i < size {
                // java KShape coreShape = (KShape)this.shapes.elementAt(i);
                let mut coreShape: KShape = (self.shapes.elementAt(i)).downcast_ref::<KShape>().unwrap();
                // java if (coreShape != null)
                if coreShape.is_some() {
                  // java Shape shape = coreShape.getShape().getBaseShape();
                  let mut shape: Shape = coreShape.getShape().getBaseShape();
                  // java int wrap = shape.getShapePos().getTextWrap();
                  let mut wrap: i32 = shape.getShapePos().getTextWrap();
                  // java if (wrap == 3)
                  if wrap == 3 {
                    // java shape.setBehindDocument(behindDocument);
                    shape.setBehindDocument(behindDocument);
                  }
                }
              }
            }
              // java this.commit("edit shape", true);
              self.commit(&"edit shape", true);
      }
    }

    // java public Long setWrap(KShape coreShape, WrapType type, int relX, int relY, float x, float y)
    pub fn setWrap(&mut self, coreShape: &KShape, r#type: &WrapType, relX: i32, relY: i32, x: f32, y: f32) -> i64 {
      // java KWrapSwitch wrapSwitch = new KWrapSwitch(this);
      let mut wrapSwitch: KWrapSwitch = KWrapSwitch::new(self);
      // java return wrapSwitch.setWrap(coreShape, type, relX, relY, x, y);
      return wrapSwitch.setWrap(coreShape, r#type, relX, relY, x, y);
    }

    // java public WrapType getWrap()
    pub fn getWrap(&mut self) -> WrapType {
      // java return this.shapes.size() == 0 ? WrapType.None : WrapTool.getWrapType((KShape)this.shapes.elementAt(0));
      return (if (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) == 0 { WrapType::None } else { WrapTool::getWrapType(Some(&(self.shapes.elementAt(0)).downcast_ref::<KShape>().unwrap())) });
    }

    // java public void delete()
    pub fn delete(&mut self) {
      // java if (this.shapes.size() != 0)
      if (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) != 0 {
        // java this.start(true);
        self.start(true);
            // java int size = this.shapes.size();
            let mut size: i32 = (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
            let mut i: i32 = 0;
            // java for(int i = 0; i < size; ++i)
            while i < size {
              // java KShape coreShape = (KShape)this.shapes.elementAt(i);
              let mut coreShape: KShape = (self.shapes.elementAt(i)).downcast_ref::<KShape>().unwrap();
              // java if (coreShape != null)
              if coreShape.is_some() {
                // java ShapeTool.deleteShape(this.getSubDocument(), coreShape);
                ShapeTool::deleteShape(self.getSubDocument(), coreShape);
              }
            }
              // java this.commit("delete shape", true);
              self.commit(&"delete shape", true);
              // java this.shapes.clear();
              self.shapes.clear();
      }
    }

    // java public void deleteChild()
    pub fn deleteChild(&self) {
      // java this.getGroup().delChildShape();
      self.getGroup().delChildShape();
    }

    // java public KShape group()
    pub fn group(&self) -> KShape {
      // java return this.getGroup().group();
      return self.getGroup().group();
    }

    // java public Vector<KShape> ungroup()
    pub fn ungroup(&self) -> Vec<KShape> {
      // java this.getGroup().ungroup();
      self.getGroup().ungroup();
      // java return this.shapes;
      return self.shapes;
    }

    // java public void clear()
    pub fn clear(&mut self) {
      // java this.shapes.clear();
      self.shapes.clear();
    }

    // java public void clearReadState()
    pub fn clearReadState(&mut self) {
      // java this.curReadShape = null;
      self.curReadShape = None;
    }

    // java public void clearGroupState()
    pub fn clearGroupState(&mut self) {
      // java this.childShapes.clear();
      self.childShapes.clear();
      // java this.curGrpShape = null;
      self.curGrpShape = None;
    }

    // java public KRange getAnchor()
    pub fn getAnchor(&self) -> Option<KRange> {
      // java return null;
      return None;
    }

    // java  int getLeft()
    pub fn getLeft(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java  int getTop()
    pub fn getTop(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java  int getWidth()
    pub fn getWidth(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java  int getHeight()
    pub fn getHeight(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java  int getType()
    pub fn getType(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java public Document getSubDocument()
    pub fn getSubDocument(&self) -> Box<dyn Document> {
      // java return this.shapes.size() > 0 ? (Document)((KShape)this.shapes.get(0)).getShape().getBaseShape().getSubDocument() : this.document;
      return (if (self.shapes.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 { todo!("cast → Box<dyn Document>") } else { self.document });
    }

    // java public Shape getShape(int i)
    pub fn getShape(&mut self, i: i32) -> Option<Shape> {
      // java KShape kShape = this.item(i);
      let mut kShape: KShape = self.item(i);
      // java return null != kShape ? kShape.getShape() : null;
      return (if kShape.is_some() { Some(kShape.getShape()) } else { None });
    }

    // java public boolean beginAdjust(KShape coreShape, DrawingHitPos hitPos, float x, float y, boolean async)
    pub fn beginAdjust(&mut self, coreShape: &KShape, hitPos: &DrawingHitPos, x: f32, y: f32, r#async: bool) -> bool {
      // java return this.mShapeAdjust.beginAdjust(coreShape, hitPos, x, y, async);
      return self.mShapeAdjust.beginAdjust(Some(&coreShape), &hitPos, x, y, r#async);
    }

    // java public boolean adjust(float x, float y)
    pub fn adjust(&mut self, x: f32, y: f32) -> bool {
      // java return this.mShapeAdjust.adjust(x, y);
      return self.mShapeAdjust.adjust(x, y);
    }

    // java public boolean endAdjust(boolean async)
    pub fn endAdjust(&mut self, r#async: bool) -> bool {
      // java return this.mShapeAdjust.endAdjust(async);
      return self.mShapeAdjust.endAdjust(r#async);
    }

    // java public KInlineShapes getInlineShapes()
    pub fn getInlineShapes(&mut self) -> KInlineShapes {
      // java if (this.mInlineShapes == null)
      if self.mInlineShapes.is_none() {
        // java if (VersionState.isProVersion())
        if VersionState::isProVersion() {
          // java if (null == this.mKRange)
          if self.mKRange.is_none() {
            // java this.mInlineShapes = new KInlineShapes(this.document.getRange(0, this.document.getLength()));
            self.mInlineShapes = KInlineShapes::new(self.document.getRange__int_int(0, self.document.getLength()));
          }
          // java else
          else {
            // java this.mInlineShapes = new KInlineShapes(this.mKRange);
            self.mInlineShapes = KInlineShapes::new(self.mKRange);
          }
        }
        // java else
        else {
          // java this.mInlineShapes = new KInlineShapes();
          self.mInlineShapes = KInlineShapes::new();
        }
      }
      // java return this.mInlineShapes;
      return self.mInlineShapes;
    }

    // java public KShapes getShapes()
    pub fn getShapes(&mut self) -> KShapes {
      // java if (this.mShapes == null)
      if self.mShapes.is_none() {
        // java this.mShapes = new KShapes(this);
        self.mShapes = KShapes::new(self);
      }
      // java return this.mShapes;
      return self.mShapes;
    }

    // java private KShapeScale getShapeScale()
    pub fn getShapeScale(&mut self) -> KShapeScale {
      // java if (this.mShapeScale == null)
      if self.mShapeScale.is_none() {
        // java this.mShapeScale = new KShapeScale(this);
        self.mShapeScale = KShapeScale::new(self);
      }
      // java return this.mShapeScale;
      return self.mShapeScale;
    }

    // java private KGroup getGroup()
    pub fn getGroup(&mut self) -> KGroup {
      // java if (this.mGroup == null)
      if self.mGroup.is_none() {
        // java this.mGroup = new KGroup(this);
        self.mGroup = KGroup::new(self);
      }
      // java this.getSubDocument().getDrawingContainer().setGroupDelListener(this.mGroup);
      self.getSubDocument().getDrawingContainer().setGroupDelListener(self.mGroup);
      // java return this.mGroup;
      return self.mGroup;
    }

    // java public KLineFormat getLine()
    pub fn getLine(&mut self) -> KLineFormat {
      // java if (this.mLine == null)
      if self.mLine.is_none() {
        // java this.mLine = new KLineFormat(this);
        self.mLine = KLineFormat::new(self);
      }
      // java return this.mLine;
      return self.mLine;
    }

    // java public KFillFormat getFill()
    pub fn getFill(&mut self) -> KFillFormat {
      // java if (this.mFill == null)
      if self.mFill.is_none() {
        // java this.mFill = new KFillFormat(this);
        self.mFill = KFillFormat::new(self);
      }
      // java return this.mFill;
      return self.mFill;
    }

    // java public static void setShapesDefaultProperties(Shape shape)
    pub fn setShapesDefaultProperties(shape: &Shape) {
      // java getDefaultProperties().setDefaultProperties(shape);
      Self::getDefaultProperties().setDefaultProperties(shape);
    }

    // java public static Shape getShapesDefaultProperties()
    pub fn getShapesDefaultProperties() -> Shape {
      // java return getDefaultProperties().getDefaultProperties();
      return Self::getDefaultProperties().getDefaultProperties();
    }

    // java static ShapeDefaultProperties getDefaultProperties()
    pub fn getDefaultProperties() -> ShapeDefaultProperties {
      // java if (defaultShpPr == null)
      if defaultShpPr.is_none() {
        // java defaultShpPr = new ShapeDefaultProperties();
        defaultShpPr = ShapeDefaultProperties::new();
      }
      // java return defaultShpPr;
      return defaultShpPr;
    }

}

impl std::ops::Deref for KShapeRange {
    type Target = ShapeRange;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KShapeRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
