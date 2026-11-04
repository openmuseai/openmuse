// Java: cn.wps.moffice.writer.io.reader.dmlReader.PresetTextWarpImporter
use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, sync::Arc};
use drawing::cn::wps::moffice::drawing::geotext::PresetTextWarp::PresetTextWarp;
use drawing::cn::wps::moffice::drawing::smartshape::GeoAdjust::GeoAdjust;
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    DmlImporterBase::DmlImporterBase,
    context::{DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt, ThemeContext::ThemeContext},
    helper::TextShapeType::TextShapeType,
};

// Java: public class PresetTextWarpImporter extends DmlImporterBase
pub struct PresetTextWarpImporter<ShapeImporter, DmlCommonImporter: IImporterBase> {
    pub super_: DmlImporterBase<ShapeImporter, DmlCommonImporter>,
    // Java: private PresetTextWarp mTarget;
    mTarget: Option<Arc<PresetTextWarp>>,
}

impl<S, D: IImporterBase> Deref for PresetTextWarpImporter<S, D> {
    type Target = DmlImporterBase<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for PresetTextWarpImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> PresetTextWarpImporter<S, D> {
    // Java: PresetTextWarpImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext, Xml07Context, DmlImporter)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>, ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>, xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<D>>) -> Self {
        Self {
            super_: DmlImporterBase::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter),
            mTarget: None,
        }
    }

    // Java: public void excute()
    pub fn excute(&mut self) {
        self.initTarget();
        self.importGeoAdjust();
    }

    // Java: public PresetTextWarp result()
    pub fn result(&self) -> Option<Arc<PresetTextWarp>> {
        self.mTarget.clone()
    }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        if self.mTarget.is_none() {
            self.mTarget = Some(Arc::new(PresetTextWarp::new()));
        }
    }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        self.mTarget = None;
        self.mCtxt.borrow_mut().mGdlst = None;
    }

    // Java: public void importWarp(Attributes attrs)
    pub fn importWarp(&mut self, attrs: &dyn SaxAttributes) {
        // Java: String prst = ConvertHelper.getString(attrs, "prst");
        let prst = if attrs.getLength() == 0 {
            None
        } else {
            attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), "prst")
        };
        if let Some(prst) = prst {
            self.initTarget();
            let warp = TextShapeType::toPrstTextShapeType_name(&prst);
            Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("warp").setWarp(warp);
        }
    }

    // Java: private void importGeoAdjust()
    fn importGeoAdjust(&mut self) {
        let adjust = {
            let ctxt = self.mCtxt.borrow();
            let Some(gds) = ctxt.mGdlst.as_ref() else { return; };
            let gds = gds.borrow();
            if gds.isEmpty() {
                return;
            }
            let mut adjust = GeoAdjust::new();
            // Java: Long adj = this.mCtxt.mGdlst.get(96423, this.mCtxt.mPathW, this.mCtxt.mPathH);
            if let Some(adj) = gds.get(Some(96423), ctxt.mPathW, ctxt.mPathH) {
                adjust.setAdjust(0, adj as i32);
            } else {
                gds.gds2GeaoAdjust(&mut adjust);
            }
            if !adjust.isEmpty() { Some(adjust) } else { None }
        };
        if let Some(adjust) = adjust {
            self.initTarget();
            Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("warp").setGeoAdjust(adjust);
        }
    }
}
