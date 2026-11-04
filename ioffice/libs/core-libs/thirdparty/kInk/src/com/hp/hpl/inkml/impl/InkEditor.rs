// Java: import android.graphics.RectF
use util::graphics::RectF::RectF;
// Java: import com.hp.hpl.inkml.Ink
use crate::com::hp::hpl::inkml::Ink::Ink;
// Java: import com.hp.hpl.inkml.InkMLException
// Java: import com.hp.hpl.inkml.Trace
// Java: import java.util.Iterator

use crate::com::hp::hpl::inkml::r#impl::InkCreator::InkCreator;

// Java field: String TAG
pub(crate) const TAG: &str = "InkEditor";

// Java: public class InkEditor extends InkCreator
pub struct InkEditor {
    pub(crate) inner: InkCreator,
}

// Java: public class InkEditor extends InkCreator
impl InkEditor {
    // Java: InkEditor()
    pub fn InkEditor() -> Self {
      // Java: super();
      Self {
          inner: InkCreator::InkCreator(),
      }
    }

    // Java: InkEditor(Ink ink, float l, float t, float r, float b)
    pub fn InkEditor_with_bounds(ink: Ink, l: f32, t: f32, r: f32, b: f32) -> Self {
      // Java: super();
      let mut editor = Self {
          inner: InkCreator::InkCreator(),
      };
      // Java: this.mInk = ink;
      editor.inner.mInk = Some(ink);
      // Java: this.mSelectRect.set(l, t, r, b);
      editor.inner.mSelectRect = RectF::new_with(l, t, r, b);
      editor
    }

    // Java: InkEditor(Ink ink, RectF rect)
    pub fn InkEditor_with_rect(ink: Ink, rect: &RectF) -> Self {
      // Java: super();
      let mut editor = Self {
          inner: InkCreator::InkCreator(),
      };
      // Java: this.mInk = ink;
      editor.inner.mInk = Some(ink);
      // Java: this.mSelectRect.set(rect);
      editor.inner.mSelectRect = RectF::new_with(rect.left, rect.top, rect.right, rect.bottom);
      editor
    }

    // Java: void add(InkCreator c)
    pub fn add(&mut self, _c: &InkCreator) {
      // Java: try
      {
         // Java: for(Iterator it = c.getInk().getTraceIterator(); it.hasNext(); this.mSelectRect = InkTool.unionRect(this.mSelectRect, c.getRect()))
         {
            // Java: Trace trace = (Trace)it.next();
            // Java: this.getInk().addTrace(trace);
         }
      }
      // Java: catch (InkMLException var4)
      {
      }
    }

}
