// Java: import android.graphics.Canvas
// Java: import android.view.MotionEvent
// Java: import cn.wps.libs.inkml.event.IEventDataSupplier
use crate::cn::wps::libs::inkml::event::IEventDataSupplier::IEventDataSupplier;
// Java: import cn.wps.libs.inkml.event.MotionEventParser
use crate::cn::wps::libs::inkml::event::MotionEventParser::MotionEventParser;
// Java: import cn.wps.libs.inkml.paint.EraserPaint
use crate::cn::wps::libs::inkml::paint::EraserPaint::EraserPaint;
// Java: import cn.wps.libs.inkml.paint.FramePaint
use crate::cn::wps::libs::inkml::paint::FramePaint::FramePaint;
// Java: import cn.wps.libs.inkml.paint.TraceGroupPaint
use crate::cn::wps::libs::inkml::paint::TraceGroupPaint::TraceGroupPaint;
// Java: import cn.wps.libs.inkml.trace.TraceGroup
use crate::cn::wps::libs::inkml::trace::TraceGroup::TraceGroup;
// Java: import cn.wps.libs.inkml.trace.TraceSetting
use crate::cn::wps::libs::inkml::trace::TraceSetting::TraceSetting;
// Java: import com.hp.hpl.inkml.Ink
use crate::com::hp::hpl::inkml::Ink::Ink;
// Java: import com.hp.hpl.inkml.impl.InkCreator
use crate::com::hp::hpl::inkml::r#impl::InkCreator::InkCreator;
// Java: import java.util.List

use crate::cn::wps::libs::inkml::event::MotionEventParser::MotionEvent;

// Java: public class KInkML
pub struct KInkML {
    // Java field: TraceGroup traceGroup
    pub(crate) traceGroup: TraceGroup,
    // Java field: TraceSetting setting
    pub(crate) setting: TraceSetting,
    // Java field: MotionEventParser eventParser
    pub(crate) eventParser: MotionEventParser,
    // Java field: TraceGroupPaint traceGroupPaint
    pub(crate) traceGroupPaint: TraceGroupPaint,
    // Java field: EraserPaint eraserPaint
    eraserPaint: EraserPaint,
}

// Java: public class KInkML
impl KInkML {
    // Java: KInkML(float dip)
    pub fn KInkML_simple(dip: f32) -> Self {
      // Java: this(dip, false);
      Self::KInkML_manager(dip, false)
    }

    // Java: KInkML(float dip, boolean bManagerInk)
    pub fn KInkML_manager(dip: f32, b_manager_ink: bool) -> Self {
      // Java: this(dip, bManagerInk, true);
      Self::KInkML(dip, b_manager_ink, true)
    }

    // Java: KInkML(float dip, boolean bManagerInk, boolean bUseEstimate)
    pub fn KInkML(dip: f32, b_manager_ink: bool, b_use_estimate: bool) -> Self {
      // Java: super();
      // Java: this.setting = new TraceSetting();
      let setting = TraceSetting::TraceSetting();
      // Java: this.traceGroup = new TraceGroup(bManagerInk);
      let trace_group = TraceGroup::TraceGroup(b_manager_ink);
      // Java: this.eventParser = new MotionEventParser(this.traceGroup, this.setting, dip, bUseEstimate);
      let setting2 = TraceSetting::TraceSetting();
      let trace_group2 = TraceGroup::TraceGroup(b_manager_ink);
      let event_parser = MotionEventParser::MotionEventParser(trace_group2, setting2, dip, b_use_estimate);
      // Java: this.traceGroupPaint = new TraceGroupPaint(this.traceGroup, this.setting);
      let setting3 = TraceSetting::TraceSetting();
      let trace_group3 = TraceGroup::TraceGroup(b_manager_ink);
      let trace_group_paint = TraceGroupPaint::TraceGroupPaint(trace_group3, setting3);
      // Java: this.eraserPaint = new EraserPaint(this.traceGroup, this.setting);
      let setting4 = TraceSetting::TraceSetting();
      let trace_group4 = TraceGroup::TraceGroup(b_manager_ink);
      let eraser_paint = EraserPaint::EraserPaint(trace_group4, setting4);
      Self {
          traceGroup: trace_group,
          setting,
          eventParser: event_parser,
          traceGroupPaint: trace_group_paint,
          eraserPaint: eraser_paint,
      }
    }

    // Java: boolean processEvent(MotionEvent event)
    pub fn processEvent(&mut self, event: &MotionEvent) -> bool {
      // Java: return this.eventParser.processEvent(event);
      self.eventParser.processEvent(event)
    }

    // Java: void setFilterPointDistance(float distance)
    pub fn setFilterPointDistance(&mut self, distance: f32) {
      // Java: this.eventParser.setFilterPointDistance(distance);
      self.eventParser.setFilterPointDistance(distance);
    }

    // Java: boolean touchCancel()
    pub fn touchCancel(&mut self) -> bool {
      // Java: return this.eventParser.touchCancel();
      self.eventParser.touchCancel()
    }

    // Java: void setTouchListener(InkMLTouchListener touchListener)
    pub fn setTouchListener(&self) {
      // Java: this.traceGroup.setTouchListener(touchListener);
    }

    // Java: void setManagerListener(InkMLManagerListener managerListener)
    pub fn setManagerListener(&self) {
      // Java: this.traceGroup.setManagerListener(managerListener);
    }

    // Java: void setViewListener(InkMLViewListener viewListener)
    pub fn setViewListener(&self) {
      // Java: this.traceGroup.setViewListener(viewListener);
    }

    // Java: void setConverter(InkMLConverter converter)
    pub fn setConverter(&self) {
      // Java: this.traceGroup.setConverter(converter);
    }

    // Java: void setSettingListener(InkMLSettingListener listener)
    pub fn setSettingListener(&self) {
      // Java: this.setting.setListener(listener);
    }

    // Java: void processDraw(Canvas canvas)
    pub fn processDraw_canvas(&self) {
      // Java: this.traceGroupPaint.processDraw(canvas);
      // Java: this.eraserPaint.draw(canvas);
    }

    // Java: boolean needDraw()
    pub fn needDraw(&self) -> bool {
      // Java: return this.traceGroupPaint.needDraw() || this.eraserPaint.needDraw();
      self.traceGroupPaint.needDraw() || self.eraserPaint.needDraw()
    }

    // Java: void initSetting(String tip, int color, float strokeWidth)
    pub fn initSetting(&mut self, tip: &str, color: i32, stroke_width: f32) {
      // Java: this.setTip(tip);
      self.setTip(tip);
      // Java: this.setColor(color);
      self.setColor(color);
      // Java: this.setStrokeWidth(strokeWidth);
      self.setStrokeWidth(stroke_width);
    }

    // Java: void setTip(String tip)
    pub fn setTip(&mut self, tip: &str) {
      // Java: this.setting.setTip(tip);
      self.setting.setTip(tip);
      // Java: this.eventParser.onTipChange();
      self.eventParser.onTipChange();
    }

    // Java: String getTip()
    pub fn getTip(&self) -> &str {
      // Java: return this.setting.getTip();
      self.setting.getTip()
    }

    // Java: boolean isWritingPen()
    pub fn isWritingPen(&self) -> bool {
      // Java: return this.setting.isWritingPen();
      self.setting.isWritingPen()
    }

    // Java: boolean isHighlight()
    pub fn isHighlight(&self) -> bool {
      // Java: return this.setting.isHighlight();
      self.setting.isHighlight()
    }

    // Java: boolean isEraser()
    pub fn isEraser(&self) -> bool {
      // Java: return this.setting.isEraser();
      self.setting.isEraser()
    }

    // Java: void setColor(int color)
    pub fn setColor(&mut self, color: i32) {
      // Java: this.setting.setColor(color);
      self.setting.setColor(color);
    }

    // Java: int getColor()
    pub fn getColor(&self) -> i32 {
      // Java: return this.setting.getColor();
      self.setting.getColor()
    }

    // Java: void setStrokeWidth(float width)
    pub fn setStrokeWidth(&mut self, width: f32) {
      // Java: this.setting.setStrokeWidth(width);
      self.setting.setStrokeWidth(width);
    }

    // Java: float getStrokeWidth()
    pub fn getStrokeWidth(&self) -> f32 {
      // Java: return this.setting.getStrokeWidth();
      self.setting.getStrokeWidth()
    }

    // Java: void setMinStrokeWidth(float width)
    pub fn setMinStrokeWidth(&mut self, width: f32) {
      // Java: this.setting.setMinStrokeWidth(width);
      self.setting.setMinStrokeWidth(width);
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.traceGroup.clear();
      self.traceGroup.clear();
    }

    // Java: void resetLastInkCreator()
    pub fn resetLastInkCreator(&mut self) {
      // Java: this.traceGroup.resetEndCurrentInkCreator();
      self.traceGroup.resetEndCurrentInkCreator();
    }

    // Java: boolean isMoving()
    pub fn isMoving(&self) -> bool {
      // Java: return this.eventParser.isMoving();
      self.eventParser.isMoving()
    }

    // Java: void setMoving(boolean moving)
    pub fn setMoving(&mut self, moving: bool) {
      // Java: this.eventParser.setMoving(moving);
      self.eventParser.setMoving(moving);
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.traceGroup.size();
      self.traceGroup.size()
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.traceGroup.isEmpty();
      self.traceGroup.isEmpty()
    }

    // Java: Ink getInk(int index)
    pub fn getInk(&self, index: i32) -> Option<&Ink> {
      // Java: return this.traceGroup.getInk(index);
      self.traceGroup.getInk(index)
    }

    // Java: InkCreator getInkCreator(int index)
    pub fn getInkCreator(&self, index: i32) -> Option<&InkCreator> {
      // Java: return this.traceGroup.getInkCreator(index);
      self.traceGroup.getInkCreator(index)
    }

    // Java: InkCreator mergeInkCreators()
    pub fn mergeInkCreators(&self) -> Option<InkCreator> {
      // Java: return this.traceGroup.mergeInkCreators();
      self.traceGroup.mergeInkCreators()
    }

    // Java: List<InkCreator> getInkCreators()
    pub fn getInkCreators(&self) -> &Vec<InkCreator> {
      // Java: return this.traceGroup.getInkCreators();
      self.traceGroup.getInkCreators()
    }

    // Java: void importInk(Ink ink)
    pub fn importInk(&mut self, ink: Ink) {
      // Java: this.traceGroup.importInk(ink);
      self.traceGroup.importInk(ink);
    }

    // Java: boolean canUndo()
    pub fn canUndo(&self) -> bool {
      // Java: return this.traceGroup.canUndo();
      self.traceGroup.canUndo()
    }

    // Java: void undo()
    pub fn undo(&mut self) {
      // Java: if (!this.isMoving())
      if !self.isMoving()
      {
         // Java: this.traceGroup.undo();
         self.traceGroup.undo();
      }
    }

    // Java: boolean canRedo()
    pub fn canRedo(&self) -> bool {
      // Java: return this.traceGroup.canRedo();
      self.traceGroup.canRedo()
    }

    // Java: void redo()
    pub fn redo(&mut self) {
      // Java: if (!this.isMoving())
      if !self.isMoving()
      {
         // Java: this.traceGroup.redo();
         self.traceGroup.redo();
      }
    }

    // Java: void setEventDataSupplier(IEventDataSupplier supplier)
    pub fn setEventDataSupplier(&mut self, _supplier: Option<&dyn IEventDataSupplier>) {
      // Java: this.eventParser.setPretendParser(supplier);
      self.eventParser.setPretendParser(_supplier);
    }

    // Java: static void drawFrameDashLine(float startX, float startY, float stopX, float stopY, Canvas canvas, float strokeWidth, int color)
    pub fn drawFrameDashLine(start_x: f32, start_y: f32, stop_x: f32, stop_y: f32, stroke_width: f32, color: i32) {
      // Java: FramePaint.drawDashLine(startX, startY, stopX, stopY, canvas, strokeWidth, color);
      FramePaint::drawDashLine(start_x, start_y, stop_x, stop_y, stroke_width, color);
    }

    // Java: static boolean isFinger(MotionEvent event)
    pub fn isFinger(_event: &MotionEvent) -> bool {
      // Java: return event.getToolType(0) == 1;
      false
    }

    // Java: static boolean isStylus(MotionEvent event)
    pub fn isStylus(_event: &MotionEvent) -> bool {
      // Java: return event.getToolType(0) == 2;
      false
    }

    // Java: static boolean isMouse(MotionEvent event)
    pub fn isMouse(_event: &MotionEvent) -> bool {
      // Java: return event.getToolType(0) == 3;
      false
    }

    // Java: static boolean isEraser(MotionEvent event)
    pub fn isEraser_event(_event: &MotionEvent) -> bool {
      // Java: return event.getToolType(0) == 4;
      false
    }

    // Java: static void processDraw(Canvas canvas, InkCreator inkCreator, InkMLConverter converter)
    pub fn processDraw_static(_ink_creator: &InkCreator) {
      // Java: TraceGroupPaint.processDraw(canvas, inkCreator, converter);
    }

}
