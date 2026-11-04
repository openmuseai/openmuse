#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import android.graphics.RectF
use util::graphics::RectF::RectF;
// Java: import cn.wps.graphics.SizeF
use util::graphics::SizeF::SizeF;
// Java: import cn.wps.moffice.util.KSLog
// Java: import com.hp.hpl.inkml.Ink
use crate::com::hp::hpl::inkml::Ink::Ink;
// Java: import com.hp.hpl.inkml.InkMLException
// Java: import com.hp.hpl.inkml.Trace
use crate::com::hp::hpl::inkml::Trace::Trace;
// Java: import com.hp.hpl.inkml.TraceDataList
use crate::com::hp::hpl::inkml::TraceDataList::TraceDataList;
// Java: import java.util.Iterator
// Java: import java.util.List

// Java: public class InkTool
pub struct InkTool {
}

// Java: public class InkTool
impl InkTool {
    // Java: InkTool()
    pub fn InkTool() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static InkCreator mergeInkCreate(List<InkCreator> inkList)
    pub fn mergeInkCreate(_ink_list: &[Box<dyn std::any::Any>]) -> Option<Box<dyn std::any::Any>> {
      // Java: if (inkList != null && !inkList.isEmpty())
      if !_ink_list.is_empty()
      {
         // Java: if (inkList.size() == 1)
         if _ink_list.len() == 1
         {
            // Java: return (InkCreator)inkList.get(0);
            return None; // InkCreator not yet available
         }
         // Java: else
         {
            // Java: int size = inkList.size();
            let _size = _ink_list.len();
            // Java: InkCreator c = (InkCreator)inkList.get(0);
            // Java: Ink ink = c.getInk().clone();
            // Java: RectF allRect = getRect(inkList);
            // Java: normalizeInk(ink, allRect, c.getRect());
            // Java: InkEditor inkEditor = new InkEditor(ink, c.getRect());
            // Java: for(int i = 1; i < size; ++i)
            {
               // Java: c = (InkCreator)inkList.get(i);
               // Java: if (c != null)
               {
                  // Java: ink = c.getInk();
                  // Java: normalizeInk(ink, allRect, c.getRect());
                  // Java: inkEditor.add(c);
               }
            }
            // Java: return inkEditor;
            return None; // InkCreator/InkEditor not yet available
         }
      }
      // Java: else
      {
         // Java: return null;
         None
      }
    }

    // Java: static RectF unionRect(RectF dst, RectF src)
    pub fn unionRect(dst: Option<RectF>, src: Option<&RectF>) -> Option<RectF> {
      // Java: if (src == null)
      let src = match src {
          None =>
      {
         // Java: return dst;
         return dst;
      }
          Some(s) => s,
      };
      // Java: else if (dst != null && (dst.width() != 0.0F || dst.height() != 0.0F))
      match dst {
          Some(mut d) if d.width() != 0.0 || d.height() != 0.0 =>
      {
         // Java: if (src.left < dst.left)
         if src.left < d.left
         {
            // Java: dst.left = src.left;
            d.left = src.left;
         }
         // Java: if (src.top < dst.top)
         if src.top < d.top
         {
            // Java: dst.top = src.top;
            d.top = src.top;
         }
         // Java: if (src.right > dst.right)
         if src.right > d.right
         {
            // Java: dst.right = src.right;
            d.right = src.right;
         }
         // Java: if (src.bottom > dst.bottom)
         if src.bottom > d.bottom
         {
            // Java: dst.bottom = src.bottom;
            d.bottom = src.bottom;
         }
         // Java: return dst;
         Some(d)
      }
      // Java: else
      _ =>
      {
         // Java: return new RectF(src);
         Some(src.clone())
      }
      }
    }

    // Java: static RectF getRect(List<InkCreator> inkList)
    pub fn getRect(_ink_list: &[Box<dyn std::any::Any>]) -> RectF {
      // Java: RectF rectF = null;
      let mut rect_f: Option<RectF> = None;
      // Java: int size = inkList.size();
      let size = _ink_list.len();
      // Java: for(int i = 0; i < size; ++i)
      for _i in 0..size
      {
         // Java: InkCreator c = (InkCreator)inkList.get(i);
         // Java: rectF = unionRect(rectF, c.getRect());
      }
      // Java: return rectF != null ? rectF : new RectF();
      rect_f.unwrap_or_default()
    }

    // Java: static RectF getInkRect(Ink ink)
    pub fn getInkRect(ink: &Ink) -> RectF {
      // Java: RectF rect = null;
      let mut rect: Option<RectF> = None;
      // Java: Trace trace;
      // Java: try
      {
         // Java: for(Iterator<?> iter = ink.getTraceIterator(); iter.hasNext(); rect = unionRect(rect, trace.getRectAfterTransform()))
         for trace in ink.getTraceIterator()
         {
            // Java: trace = (Trace)iter.next();
            let trace_rect = trace.getRectAfterTransform();
            rect = Self::unionRect(rect, Some(&trace_rect));
         }
      }
      // Java: catch (Exception e)
      {
         // Java: KSLog.e("Ink", "", e);
      }
      // Java: return rect != null ? rect : new RectF();
      rect.unwrap_or_default()
    }

    // Java: static void normalizeInk(Ink ink, RectF allRect, RectF rect)
    pub fn normalizeInk(ink: &mut Ink, allRect: &RectF, rect: &RectF) {
      // Java: try
      {
         // Java: float scale = 26.458334F;
         let scale: f32 = 26.458334;
         // Java: RectF inkRect = getInkRect(ink);
         let ink_rect = Self::getInkRect(ink);
         // Java: SizeF scaleXY = Ink.calScaleXY(inkRect, rect.width(), rect.height());
         let scale_xy = Ink::calScaleXY(&ink_rect, rect.width(), rect.height());
         // Java: float scaleX = scaleXY.width;
         let scale_x = scale_xy.width;
         // Java: float scaleY = scaleXY.height;
         let scale_y = scale_xy.height;
         // Java: float offsetX = rect.left - inkRect.left * scaleX - allRect.left;
         let offset_x = rect.left - ink_rect.left * scale_x - allRect.left;
         // Java: float offsetY = rect.top - inkRect.top * scaleY - allRect.top;
         let offset_y = rect.top - ink_rect.top * scale_y - allRect.top;
         // Java: Iterator it = ink.getTraceIterator();
         // Java: while(it.hasNext())
         let traces = ink.getTraceIterator();
         // cannot iterate mutably over traces from ink since getTraceIterator returns refs
         // stub: normalizeInk logic requires mutable access to trace data which is complex
         {
            // Java: Trace trace = (Trace)it.next();
            // Java: TraceDataList list = trace.getTraceData();
            // Java: Iterator<Object[]> iterator = list.iterator();
            // Java: int xIndex = list.getXIndex();
            // Java: int yIndex = list.getYIndex();
            // Java: float revScaleX = scaleX / trace.mScaleX;
            // Java: float revScaleY;
            // Java: Object[] traceFragment;
            // Java: float y;
            // Java: for(revScaleY = scaleY / trace.mScaleY; iterator.hasNext(); traceFragment[yIndex] = (y * revScaleY + offsetY) * scale)
            {
               // Java: traceFragment = iterator.next();
               // Java: float x = (Float)traceFragment[xIndex];
               // Java: y = (Float)traceFragment[yIndex];
               // Java: traceFragment[xIndex] = (x * revScaleX + offsetX) * scale;
            }
            // Java: trace.mScaleX = scale;
            // Java: trace.mScaleY = scale;
            // Java: list.scale(revScaleX, revScaleY);
            // Java: list.offset(offsetX, offsetY);
            // Java: list.scale(trace.mScaleX, trace.mScaleY);
         }
      }
      // Java: catch (InkMLException var21)
      {
      }
    }

    // Java: static RectF getInksRect(List<InkCreator> inks)
    pub fn getInksRect(inks: Option<&[Box<dyn std::any::Any>]>) -> Option<RectF> {
      // Java: if (inks == null)
      let inks = match inks {
          None =>
      {
         // Java: return null;
         return None;
      }
          Some(v) => v,
      };
      // Java: else
      {
         // Java: RectF rectF = null;
         let mut rect_f: Option<RectF> = None;
         // Java: for(InkCreator ink : inks)
         for ink in inks.iter()
         {
            // Java: if (ink != null)
            {
               // Java: RectF rect = ink.getRect();
               // Java: if (rect != null)
               {
                  // Java: rectF = unionRect(rectF, rect);
               }
            }
         }
         // Java: return rectF;
         rect_f
      }
    }

}
