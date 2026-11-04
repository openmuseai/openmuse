// Java: cn.wps.moffice.writer.io.reader.dmlReader.DmlGroupShapeImporter
#![allow(non_snake_case)]

use std::sync::{Arc, Mutex};
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::diagram::Diagram::Diagram;
use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use util::cn::wps::graphics::RectF::RectF;
use super::DmlShapeImporter::DmlShapeImporter;

// Java: public class DmlGroupShapeImporter extends DmlShapeImporter
// The stack stores DmlShapeImporter. Group methods run on that same object
// after GRFImporter.setIsGroup(true), which is what the Java constructor does.
pub struct DmlGroupShapeImporter;

impl DmlGroupShapeImporter {
    // Java: public void importCoordBounds()
    pub fn importCoordBounds<S, D: IImporterBase>(sp: &mut DmlShapeImporter<S, D>) {
        let (l, t, w, h) = {
            let ctxt = sp.mCtxt.borrow();
            (
                ctxt.mChOffX.unwrap_or(0),
                ctxt.mChOffY.unwrap_or(0),
                ctxt.mChExtX.unwrap_or(0),
                ctxt.mChExtY.unwrap_or(0),
            )
        };
        let coord = RectF {
            left: l as f32,
            top: t as f32,
            right: (l + w) as f32,
            bottom: (t + h) as f32,
        };
        Self::importCoordBounds_rect(sp, Some(coord));
        Self::clearCoordBounds(sp);
    }

    // Java: public void importCoordBounds(RectF coordBound)
    pub fn importCoordBounds_rect<S, D: IImporterBase>(sp: &mut DmlShapeImporter<S, D>, coordBound: Option<RectF>) {
        // Java: if (null != coordBound) this.mTarget.setCoordBounds(coordBound);
        // setCoordBounds stores property 733.
        if let Some(coord) = coordBound {
            let value: Arc<dyn std::any::Any + Send + Sync> = Arc::new(coord);
            sp.mTarget.lock().expect("shape").setPropertyValue(733, Some(value));
        }
    }

    // Java: public void importDiagram()
    pub fn importDiagram<S, D: IImporterBase>(sp: &mut DmlShapeImporter<S, D>) {
        let mut diagram = Diagram::new();
        // Java: diagram.setDiagramType(0); this.mTarget.setDiagram(diagram);
        diagram.setDiagramType(0);
        let value: Arc<dyn std::any::Any + Send + Sync> = Arc::new(Mutex::new(diagram));
        sp.mTarget.lock().expect("shape").setDiagram(Some(value));
    }

    fn clearCoordBounds<S, D: IImporterBase>(sp: &mut DmlShapeImporter<S, D>) {
        let mut ctxt = sp.mCtxt.borrow_mut();
        ctxt.mChOffX = None;
        ctxt.mChOffY = None;
        ctxt.mChExtX = None;
        ctxt.mChExtY = None;
    }
}
