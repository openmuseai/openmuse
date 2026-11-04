// Java: cn.wps.moffice.writer.io.reader.dmlReader.GRFImporter
use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, sync::Arc};
use drawing::cn::wps::moffice::drawing::GRF::GRF;
use io_wps_io::cn::wps::io::xwpf::IImporterBase::IImporterBase;
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    DmlImporterBase::DmlImporterBase,
    context::{DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt, ThemeContext::ThemeContext},
};

// Java: public class GRFImporter extends DmlImporterBase
pub struct GRFImporter<ShapeImporter, DmlCommonImporter: IImporterBase> {
    pub super_: DmlImporterBase<ShapeImporter, DmlCommonImporter>,
    // Java: private GRF mTarget;
    mTarget: Option<Arc<GRF>>,
}

impl<S, D: IImporterBase> Deref for GRFImporter<S, D> {
    type Target = DmlImporterBase<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for GRFImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> GRFImporter<S, D> {
    // Java: GRFImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext, Xml07Context, DmlImporter)
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
        let (is_group, is_child) = {
            let ctxt = self.mCtxt.borrow();
            (ctxt.mIsGroup, ctxt.mIsChild)
        };
        let grf = Arc::get_mut(self.mTarget.as_mut().expect("mTarget")).expect("grf");
        // Java: if (null != this.mCtxt.mIsGroup) { setGroup; setHaveSpt(!isGroup); }
        if let Some(is_group) = is_group {
            grf.setGroup(is_group);
            grf.setHaveSpt(!is_group);
        }
        // Java: if (null != this.mCtxt.mIsChild) this.mTarget.setChild(this.mCtxt.mIsChild);
        if let Some(is_child) = is_child {
            grf.setChild(is_child);
        }
    }

    // Java: public GRF result()
    pub fn result(&self) -> Option<Arc<GRF>> {
        self.mTarget.clone()
    }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        if self.mTarget.is_none() {
            self.mTarget = Some(Arc::new(GRF::new()));
        }
    }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        self.mTarget = None;
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mIsGroup = None;
        ctxt.mIsChild = None;
    }

    // Java: public void setIsGroup(boolean isGroup)
    pub fn setIsGroup(&mut self, isGroup: bool) {
        self.mCtxt.borrow_mut().mIsGroup = Some(isGroup);
    }

    // Java: public void setIsChild(boolean isChild)
    pub fn setIsChild(&mut self, isChild: bool) {
        self.mCtxt.borrow_mut().mIsChild = Some(isChild);
    }
}
