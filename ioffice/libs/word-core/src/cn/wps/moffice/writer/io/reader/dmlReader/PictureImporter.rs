// Java: cn.wps.moffice.writer.io.reader.dmlReader.PictureImporter
use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, sync::{Arc, Mutex}};
use drawing::cn::wps::moffice::drawing::graphics::Picture::Picture;
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use io_wps_io::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    context::{DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt, ThemeContext::ThemeContext},
    BlipFillImporter::BlipFillImporter,
};

// Java: public class PictureImporter extends BlipFillImporter
pub struct PictureImporter<ShapeImporter, DmlCommonImporter: IImporterBase> {
    pub super_: BlipFillImporter<ShapeImporter, DmlCommonImporter>,
}

impl<S, D: IImporterBase> Deref for PictureImporter<S, D> {
    type Target = BlipFillImporter<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for PictureImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> PictureImporter<S, D> {
    // Java: PictureImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext, Xml07Context, DmlImporter)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>,
        ctxt: Rc<RefCell<DmlShapeCtxt>>, themeCtxt: Rc<RefCell<ThemeContext>>,
        xml07Context: Option<Xml07Context>, dmlImporter: Rc<RefCell<D>>) -> Self {
        let mut importer = BlipFillImporter::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter);
        importer.is_picture = true;
        Self { super_: importer }
    }

    // Java: public void excute()
    pub fn excute(&mut self) {}

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        // Java: super.clearContext(); this.mTarget = null;
        self.super_.clearContext();
    }

    // Java: public Picture result()
    pub fn result(&self) -> Option<Arc<Mutex<Picture>>> {
        self.mPicture.clone()
    }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        self.super_.initTarget();
    }

    // Java: public void importBlip(Attributes attrs, POIXMLDocumentPart part)
    pub fn importBlip(&mut self, attrs: &dyn SaxAttributes, part: &POIXMLDocumentPart) {
        self.super_.importBlip(attrs, part);
    }

    // Java: public void importSrcRect(Attributes attrs)
    pub fn importSrcRect(&mut self, attrs: &dyn SaxAttributes) {
        self.super_.importSrcRect(attrs);
    }
}
