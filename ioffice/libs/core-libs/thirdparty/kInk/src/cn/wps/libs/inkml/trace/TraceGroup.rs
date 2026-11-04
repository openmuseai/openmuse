// Java: import android.graphics.RectF
use util::graphics::RectF::RectF;
// Java: import cn.wps.graphics.PointF
use util::graphics::PointF::PointF;
// Java: import com.hp.hpl.inkml.Ink
use crate::com::hp::hpl::inkml::Ink::Ink;
// Java: import com.hp.hpl.inkml.impl.InkCreator
use crate::com::hp::hpl::inkml::r#impl::InkCreator::InkCreator;
// Java: import com.hp.hpl.inkml.impl.InkEditor
use crate::com::hp::hpl::inkml::r#impl::InkEditor::InkEditor;
// Java: import com.hp.hpl.inkml.impl.InkTool
// Java: import java.util.ArrayList
// Java: import java.util.List

use super::TraceListener::TraceListener;
use super::Transaction::Transaction;
use super::TraceCommands::TraceCommands;

// Java: public class TraceGroup extends TraceListener
pub struct TraceGroup {
    // Java field: boolean bManagerInk
    bManagerInk: bool,
    // Java field: List<InkCreator> inkCreators
    pub(crate) inkCreators: Vec<InkCreator>,
    // Java field: InkCreator currentInkCreator
    pub(crate) currentInkCreator: Option<InkCreator>,
    // Java field: InkCreator endCurrentInkCreator
    pub(crate) endCurrentInkCreator: Option<InkCreator>,
    // Java field: PointF eraserPoint
    pub(crate) eraserPoint: PointF,
    // Java field: float lastEraserPointX
    lastEraserPointX: f32,
    // Java field: float lastEraserPointY
    lastEraserPointY: f32,
    // Java field: Transaction transaction
    pub(crate) transaction: Transaction,
    // Java field: TraceCommands traceCommands
    traceCommands: Option<TraceCommands>,
    pub(crate) listener: TraceListener,
}

// Java: public class TraceGroup extends TraceListener
impl TraceGroup {
    // Java: TraceGroup(boolean bManagerInk)
    pub fn TraceGroup(b_manager_ink: bool) -> Self {
      // Java: super(bManagerInk);
      // Java: this.bManagerInk = bManagerInk;
      let transaction = Transaction::Transaction();
      let ink_creators = Vec::new();
      // Java: this.traceCommands = new TraceCommands(this.transaction, this.inkCreators);
      Self {
          bManagerInk: b_manager_ink,
          inkCreators: ink_creators,
          currentInkCreator: None,
          endCurrentInkCreator: None,
          eraserPoint: PointF::new(),
          lastEraserPointX: -1.0,
          lastEraserPointY: -1.0,
          transaction,
          traceCommands: None,
          listener: TraceListener::TraceListener(b_manager_ink),
      }
    }

    // Java: InkCreator createCurrentInkCreator()
    pub fn createCurrentInkCreator(&mut self) -> &InkCreator {
      // Java: if (this.currentInkCreator == null)
      if self.currentInkCreator.is_none()
      {
         // Java: this.currentInkCreator = new InkCreator();
         let mut creator = InkCreator::InkCreator();
         // Java: this.currentInkCreator.setFitToCurve(true);
         creator.setFitToCurve(true);
         self.currentInkCreator = Some(creator);
      }
      // Java: return this.currentInkCreator;
      self.currentInkCreator.as_ref().unwrap()
    }

    // Java: InkCreator getCurrentInkCreator()
    pub fn getCurrentInkCreator(&self) -> Option<&InkCreator> {
      // Java: return this.currentInkCreator;
      self.currentInkCreator.as_ref()
    }

    // Java: void resetCurrentInkCreator()
    pub fn resetCurrentInkCreator(&mut self) {
      // Java: if (this.bManagerInk && this.currentInkCreator != null)
      if self.bManagerInk && self.currentInkCreator.is_some()
      {
         // Java: this.inkCreators.add(this.currentInkCreator);
         // Java: this.traceCommands.onInkAdd(this.inkCreators.size() - 1);
         // Java: super.onInkAdd(this.currentInkCreator);
      }
      // Java: if (!this.bManagerInk && this.currentInkCreator != null)
      if !self.bManagerInk && self.currentInkCreator.is_some()
      {
         // Java: this.endCurrentInkCreator = this.currentInkCreator;
         self.endCurrentInkCreator = self.currentInkCreator.take();
      }
      // Java: this.currentInkCreator = null;
      self.currentInkCreator = None;
    }

    // Java: InkCreator getEndCurrentInkCreator()
    pub fn getEndCurrentInkCreator(&self) -> Option<&InkCreator> {
      // Java: return this.endCurrentInkCreator;
      self.endCurrentInkCreator.as_ref()
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.inkCreators.size();
      self.inkCreators.len() as i32
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.inkCreators.isEmpty();
      self.inkCreators.is_empty()
    }

    // Java: InkCreator getInkCreator(int index)
    pub fn getInkCreator(&self, index: i32) -> Option<&InkCreator> {
      // Java: return index >= 0 && index < this.inkCreators.size() ? (InkCreator)this.inkCreators.get(index) : null;
      if index >= 0 && (index as usize) < self.inkCreators.len() {
          Some(&self.inkCreators[index as usize])
      } else {
          None
      }
    }

    // Java: Ink getInk(int index)
    pub fn getInk(&self, _index: i32) -> Option<&Ink> {
      // Java: return index >= 0 && index < this.inkCreators.size() ? ((InkCreator)this.inkCreators.get(index)).getInk() : null;
      None
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: if (this.inkCreators.size() > 0 || this.currentInkCreator != null || this.endCurrentInkCreator != null)
      if !self.inkCreators.is_empty() || self.currentInkCreator.is_some() || self.endCurrentInkCreator.is_some()
      {
         // Java: this.inkCreators.clear();
         self.inkCreators.clear();
         // Java: this.currentInkCreator = null;
         self.currentInkCreator = None;
         // Java: this.endCurrentInkCreator = null;
         self.endCurrentInkCreator = None;
         // Java: super.postInvalidate();
      }
    }

    // Java: void resetEndCurrentInkCreator()
    pub fn resetEndCurrentInkCreator(&mut self) {
      // Java: if (this.endCurrentInkCreator != null)
      if self.endCurrentInkCreator.is_some()
      {
         // Java: this.endCurrentInkCreator = null;
         self.endCurrentInkCreator = None;
         // Java: super.postInvalidate();
      }
    }

    // Java: InkCreator mergeInkCreators()
    pub fn mergeInkCreators(&self) -> Option<InkCreator> {
      // Java: return InkTool.mergeInkCreate(this.inkCreators);
      None
    }

    // Java: List<InkCreator> getInkCreators()
    pub fn getInkCreators(&self) -> &Vec<InkCreator> {
      // Java: return this.inkCreators;
      &self.inkCreators
    }

    // Java: void importInk(Ink ink)
    pub fn importInk(&mut self, ink: Ink) {
      // Java: if (ink != null)
      {
         // Java: InkEditor inkEditor = new InkEditor(ink, ink.getInkRect());
         // Java: this.inkCreators.add(inkEditor);
      }
    }

    // Java: void setEraserPoint(float x, float y)
    pub fn setEraserPoint(&mut self, x: f32, y: f32) {
      // Java: this.lastEraserPointX = this.eraserPoint.x;
      self.lastEraserPointX = self.eraserPoint.x;
      // Java: this.lastEraserPointY = this.eraserPoint.y;
      self.lastEraserPointY = self.eraserPoint.y;
      // Java: this.eraserPoint.x = x;
      self.eraserPoint.x = x;
      // Java: this.eraserPoint.y = y;
      self.eraserPoint.y = y;
      // Java: this.eraserInks();
      self.eraserInks();
    }

    // Java: void resetEraserPoint()
    pub fn resetEraserPoint(&mut self) {
      // Java: this.lastEraserPointX = -1.0F;
      self.lastEraserPointX = -1.0;
      // Java: this.lastEraserPointY = -1.0F;
      self.lastEraserPointY = -1.0;
      // Java: this.eraserPoint.x = -1.0F;
      self.eraserPoint.x = -1.0;
      // Java: this.eraserPoint.y = -1.0F;
      self.eraserPoint.y = -1.0;
      // Java: this.traceCommands.onInkEraserEnd();
      if let Some(ref mut cmds) = self.traceCommands {
          cmds.onInkEraserEnd();
      }
    }

    // Java: PointF getEraserPoint()
    pub fn getEraserPoint(&self) -> Option<&PointF> {
      // Java: return this.eraserPoint.x != -1.0F && this.eraserPoint.y != -1.0F ? this.eraserPoint : null;
      if self.eraserPoint.x != -1.0 && self.eraserPoint.y != -1.0 {
          Some(&self.eraserPoint)
      } else {
          None
      }
    }

    // Java: void eraserInks()
    pub fn eraserInks(&mut self) {
      // Java: if (this.bManagerInk && this.lastEraserPointX != -1.0F && this.lastEraserPointY != -1.0F && this.eraserPoint.x != -1.0F && this.eraserPoint.y != -1.0F && !this.inkCreators.isEmpty())
      if !(self.bManagerInk && self.lastEraserPointX != -1.0 && self.lastEraserPointY != -1.0 && self.eraserPoint.x != -1.0 && self.eraserPoint.y != -1.0 && !self.inkCreators.is_empty()) {
          return;
      }
      {
         // Java: boolean bRemove = false;
         let mut _b_remove = false;
         // Java: PointF lastPointF = this.adjustEventXY(this.lastEraserPointX, this.lastEraserPointY, PointF.obtain());
         // Java: PointF pointF = this.adjustEventXY(this.eraserPoint.x, this.eraserPoint.y, PointF.obtain());
         // Java: for(int i = this.inkCreators.size() - 1; i >= 0; --i)
         {
            // Java: InkCreator inkCreator = (InkCreator)this.inkCreators.get(i);
            // Java: Ink ink = inkCreator.getInk();
            // Java: if (ink != null)
            {
               // Java: RectF rectF = inkCreator.getRect();
               // Java: cn.wps.graphics.RectF rectF2 = new cn.wps.graphics.RectF();
               // Java: rectF2.set(rectF.left, rectF.top, rectF.right, rectF.bottom);
               // Java: if (ink.hitLine(rectF2, lastPointF.x, lastPointF.y, pointF.x, pointF.y, 10.0F) >= 0)
               {
                  // Java: this.inkCreators.remove(i);
                  // Java: this.traceCommands.onInkEraser(i, inkCreator);
                  // Java: super.onInkDelete(inkCreator);
                  // Java: bRemove = true;
               }
            }
         }
         // Java: if (bRemove)
         {
            // Java: super.invalidate();
         }
         // Java: lastPointF.recycle();
         // Java: pointF.recycle();
      }
    }

    // Java: boolean canUndo()
    pub fn canUndo(&self) -> bool {
      // Java: return this.transaction.canUndo();
      self.transaction.canUndo()
    }

    // Java: void undo()
    pub fn undo(&mut self) {
      // Java: if (this.canUndo())
      if self.transaction.canUndo()
      {
         // Java: this.transaction.undo();
         self.transaction.undo();
         // Java: super.invalidate();
      }
    }

    // Java: boolean canRedo()
    pub fn canRedo(&self) -> bool {
      // Java: return this.transaction.canRedo();
      self.transaction.canRedo()
    }

    // Java: void redo()
    pub fn redo(&mut self) {
      // Java: if (this.canRedo())
      if self.transaction.canRedo()
      {
         // Java: this.transaction.redo();
         self.transaction.redo();
         // Java: super.invalidate();
      }
    }

}
