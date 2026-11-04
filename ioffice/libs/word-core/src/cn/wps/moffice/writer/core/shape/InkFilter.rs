// Java: import cn.wps.collections.spanset.Span
use util::collections::spanset::Span::*;
// Java: import cn.wps.collections.spanset.SpanSet
use util::collections::spanset::SpanSet::*;
// Java: import cn.wps.kfc.impl.arrayList.WLongArrayList
use util::kfc::r#impl::arrayList::WLongArrayList::*;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::*;
// Java: import cn.wps.moffice.util.StringUtil
use util::moffice::util::StringUtil::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.IFilter
use crate::cn::wps::moffice::writer::core::IFilter::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCShapeAnchor
use crate::cn::wps::moffice::writer::data::PLCShapeAnchor::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.TableTool
use crate::cn::wps::moffice::writer::data::TableTool::*;
// Java: import com.hp.hpl.inkml.Ink
use thirdparty_kInk::com::hp::hpl::inkml::Ink::*;

// java public class InkFilter implements IFilter
#[derive(Default)]
pub struct InkFilter {
}

// java public interface IFilter
impl IFilter for InkFilter {
    // java public WLongArrayList getFilterRanges(Document doc, long range)
    fn getFilterRanges(&mut self, doc: &dyn Document, range: i64) -> WLongArrayList {
      // java SpanSet set = new SpanSet();
      let mut set: SpanSet = SpanSet::new();
      // java int start = RANGEFactory.getStart(range);
      let mut start: i32 = RANGEFactory::getStart(range);
      // java int end = RANGEFactory.getEnd(range);
      let mut end: i32 = RANGEFactory::getEnd(range);
      // java set.add(start, end);
      set.add(start, end);
      // java this.excludeInkOfDiffUserID(doc, set);
      self.excludeInkOfDiffUserID(&doc, &mut set);
      // java WLongArrayList results = new WLongArrayList();
      let mut results: WLongArrayList = WLongArrayList::new();
      // java for(Span span : set)
      for span in set.iter() {
        // java results.add(RANGEFactory.getRANGE(span.getStart(), span.getEnd()));
        results.add(RANGEFactory::getRANGE(span.getStart(), span.getEnd()));
      }
      // java return results;
      return results;
    }

}

// java public class InkFilter implements IFilter
impl InkFilter {
    // java private void excludeInkOfDiffUserID(Document doc, SpanSet set)
    pub fn excludeInkOfDiffUserID(&mut self, doc: &dyn Document, set: &mut SpanSet) {
      // java String globalInkUserID = doc.getTextDocument().getGlobalInkUserID();
      let mut globalInkUserID: String = doc.getTextDocument().getGlobalInkUserID();
      // java if (!StringUtil.isEmpty(globalInkUserID))
      if !StringUtil::isEmpty(Some(&mut globalInkUserID)) {
        // java PLCShapeAnchor shapeAnchor = doc.getShapeAnchors();
        let mut shapeAnchor: PLCShapeAnchor = doc.getShapeAnchors();
        // java PLC.Iterator iter = shapeAnchor.createIterator();
        let mut iter: Vec<()> = shapeAnchor.createIterator();
        // java while (iter.hasNext())
        while iter.hasNext() {
          // java PLCShapeAnchor.ShapeAnchor anchor = (PLCShapeAnchor.ShapeAnchor)iter.next();
          let mut anchor: PLCShapeAnchor::ShapeAnchor = (iter.next()).downcast_ref::<PLCShapeAnchor::ShapeAnchor>().unwrap();
          // java if (!TableTool.isInTable(doc, anchor.getCP()))
          if !TableTool::isInTable__propertyset_int(&doc, anchor.getCP()) {
            // java Shape shape = cn.wps.moffice.writer.data.ShapeTool.findShape(doc, anchor);
            let mut shape: Shape = cn.wps.moffice.writer.data.ShapeTool.findShape(doc, anchor);
            // java if (!this.checkPermission(globalInkUserID, shape))
            if !self.checkPermission(&globalInkUserID, Some(&shape)) {
              // java long inkShapeRange = ShapeTool.getShapeRange(doc, shape);
              let mut inkShapeRange: i64 = ShapeTool::getShapeRange(doc, shape);
              // java set.subtract(RANGEFactory.getStart(inkShapeRange), RANGEFactory.getEnd(inkShapeRange));
              set.subtract__int_int(RANGEFactory::getStart(inkShapeRange), RANGEFactory::getEnd(inkShapeRange));
            }
          }
        }
      }
    }

    // java private boolean checkPermission(String globalInkUserID, Shape shape)
    pub fn checkPermission(&mut self, globalInkUserID: &str, shape: Option<&Shape>) -> bool {
      // java if (null != shape && shape.hasInk())
      if let Some(shape) = shape {
        if shape.hasInk() {
          // java Ink ink = shape.getInk();
          let mut ink: Ink = shape.getInk();
          // java String inkUserID = ink.getDocID();
          let mut inkUserID: String = ink.getDocID();
          // java return StringUtil.isEmpty(inkUserID) || inkUserID.equals(globalInkUserID);
          return StringUtil::isEmpty(Some(&mut inkUserID)) || inkUserID.equals(globalInkUserID);
        }
      }
      // java else
      else {
        // java return true;
        return true;
      }
    }

}
