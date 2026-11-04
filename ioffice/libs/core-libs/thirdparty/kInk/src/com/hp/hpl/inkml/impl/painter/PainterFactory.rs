// Java: import com.hp.hpl.inkml.impl.Brush
use crate::com::hp::hpl::inkml::r#impl::Brush::Brush;

pub trait IPainter {}

// Java: public class PainterFactory
pub struct PainterFactory {
}

// Java: public class PainterFactory
impl PainterFactory {
    // Java: PainterFactory()
    pub fn PainterFactory() -> Self {
      // Java: super();
      Self {}
    }

    // Java: static IPainter create(Brush brush)
    pub fn create(_brush: &Brush) -> Box<dyn IPainter> {
      // Java: if (brush.getShape() == Brush.Shape.rectangle)
      {
         // Java: return new RectPainter();
      }
      // Java: else if (brush.getMaxFChannel() > 0.0F)
      {
         // Java: return new WritingBrush();
      }
      // Java: else
      {
         // Java: return (IPainter)(brush.isFitToCurve() ? new CurvePainter() : new NormalPainter());
      }
      struct DummyPainter;
      impl IPainter for DummyPainter {}
      Box::new(DummyPainter)
    }

    // Java: static boolean isWritingBrush(Brush brush)
    pub fn isWritingBrush(brush: Option<&Brush>) -> bool {
      // Java: if (null == brush)
      if brush.is_none()
      {
         // Java: return false;
         return false;
      }
      // Java: else if (brush.getShape() == Brush.Shape.rectangle)
      else if false
      {
         // Java: return false;
         return false;
      }
      // Java: else
      else
      {
         // Java: return brush.getMaxFChannel() > 0.0F;
         false
      }
    }

}
