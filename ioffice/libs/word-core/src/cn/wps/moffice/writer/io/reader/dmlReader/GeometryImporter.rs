// Java: cn.wps.moffice.writer.io.reader.dmlReader.GeometryImporter
use std::{any::Any, cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, sync::{Arc, Mutex}};
use drawing::cn::wps::moffice::drawing::smartshape::{
    GeoAdjust::GeoAdjust, Geometry::Geometry, Segment::Segment,
};
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use util::base::io::TypeConversion::TypeConversion;
use util::graphics::RectF::RectF;
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    DmlImporterBase::DmlImporterBase,
    context::{DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt, Gds::Gds, ThemeContext::ThemeContext},
};

fn java_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, ch| hash.wrapping_mul(31).wrapping_add(ch as i32))
}
fn main_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), name) }
}
fn empty_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri("", name) }
}
fn empty_int(attrs: &dyn SaxAttributes, name: &str) -> Option<i32> {
    TypeConversion::string2Integer(&empty_string(attrs, name)?)
}
fn empty_bool(attrs: &dyn SaxAttributes, name: &str) -> Option<bool> {
    let value = empty_string(attrs, name)?;
    Some(!(value.eq_ignore_ascii_case("off") || value == "0" || value.eq_ignore_ascii_case("false") || value.eq_ignore_ascii_case("f")))
}

// Java: public class GeometryImporter extends DmlImporterBase
pub struct GeometryImporter<ShapeImporter, DmlCommonImporter: IImporterBase> {
    pub super_: DmlImporterBase<ShapeImporter, DmlCommonImporter>,
    // Java: private Geometry mTarget;
    mTarget: Option<Arc<Mutex<Geometry>>>,
}

impl<S, D: IImporterBase> Deref for GeometryImporter<S, D> {
    type Target = DmlImporterBase<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for GeometryImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> GeometryImporter<S, D> {
    // Java: GeometryImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext, Xml07Context, DmlImporter)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>, ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>, xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<D>>) -> Self {
        Self {
            super_: DmlImporterBase::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter),
            mTarget: None,
        }
    }

    // Java: public void excute()
    pub fn excute(&mut self) {}

    // Java: public Geometry result()
    pub fn result(&self) -> Option<Arc<Mutex<Geometry>>> { self.mTarget.clone() }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        if self.mTarget.is_none() {
            self.mTarget = Some(Arc::new(Mutex::new(Geometry::new())));
        }
    }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mCxnlst = None;
        ctxt.mVertices = None;
        ctxt.mSegments = None;
        ctxt.mGdlst = None;
        ctxt.mPathFillType = None;
        ctxt.mPathStrokeOn = None;
        ctxt.mPathW = None;
        ctxt.mPathH = None;
    }

    // Java: public void importGeoCordSize(Attributes attrs)
    pub fn importGeoCordSize(&mut self, attrs: &dyn SaxAttributes) {
        self.initTarget();
        let (path_w, path_h) = {
            let ctxt = self.mCtxt.borrow();
            (ctxt.mPathW, ctxt.mPathH)
        };
        let w = empty_int(attrs, "w").or(path_w).expect("w");
        let h = empty_int(attrs, "h").or(path_h).expect("h");
        let rect = RectF::new_with(0.0, 0.0, w as f32, h as f32);
        self.mTarget.as_ref().expect("mTarget").lock().expect("geometry")
            .setGeoCordSize(Some(Arc::new(rect) as Arc<dyn Any + Send + Sync>));
    }

    // Java: public void importPath()
    pub fn importPath(&mut self) {
        self.initTarget();
        let (verts, segments) = {
            let mut ctxt = self.mCtxt.borrow_mut();
            (ctxt.mVertices.take().expect("mVertices"), ctxt.mSegments.take().expect("mSegments"))
        };
        let mut geometry = self.mTarget.as_ref().expect("mTarget").lock().expect("geometry");
        geometry.setVertices(Some(Arc::new(verts) as Arc<dyn Any + Send + Sync>));
        geometry.setSegment(Some(Arc::new(segments) as Arc<dyn Any + Send + Sync>));
    }

    // Java: public void importConnectionSites4DOCX()
    pub fn importConnectionSites4DOCX(&mut self) {
        let sites = {
            let ctxt = self.mCtxt.borrow();
            let Some(cxns) = ctxt.mCxnlst.as_ref() else { return; };
            let gds = ctxt.mGdlst.as_ref().expect("mGdlst");
            let gds = gds.borrow();
            let mut sites = Vec::with_capacity(cxns.len() * 2);
            for cxn in cxns.iter() {
                let x = gds.get(cxn.mX, ctxt.mPathW, ctxt.mPathH).unwrap_or(0) as i32;
                let y = gds.get(cxn.mY, ctxt.mPathW, ctxt.mPathH).unwrap_or(0) as i32;
                sites.push(x);
                sites.push(y);
            }
            sites
        };
        self.initTarget();
        self.mTarget.as_ref().expect("mTarget").lock().expect("geometry")
            .setConnectionSites4DOCX(Some(Arc::new(sites) as Arc<dyn Any + Send + Sync>));
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mCxnlst = None;
        ctxt.mGdlst = None;
    }

    // Java: public void importGeoAdjust()
    pub fn importGeoAdjust(&mut self) {
        let adjust = {
            let ctxt = self.mCtxt.borrow();
            let Some(gds) = ctxt.mGdlst.as_ref() else { return; };
            let gds = gds.borrow();
            if gds.isEmpty() { return; }
            let mut adjust = GeoAdjust::new();
            if let Some(adj) = gds.get(Some(96423), ctxt.mPathW, ctxt.mPathH) {
                adjust.setAdjust(0, adj as i32);
            } else {
                gds.gds2GeaoAdjust(&mut adjust);
            }
            if adjust.isEmpty() { None } else { Some(adjust) }
        };
        if let Some(adjust) = adjust {
            self.initTarget();
            self.mTarget.as_ref().expect("mTarget").lock().expect("geometry")
                .setGeoAdjust(Some(Arc::new(adjust) as Arc<dyn Any + Send + Sync>));
        }
    }

    // Java: public void createPathContext()
    pub fn createPathContext(&mut self) {
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mVertices = Some(Vec::new());
        ctxt.mSegments = Some(Vec::new());
    }

    // Java: public void addSegment(short cmd)
    pub fn addSegment(&mut self, cmd: i16) {
        let mut ctxt = self.mCtxt.borrow_mut();
        let vertex_count = ctxt.mVertices.as_ref().expect("mVertices").len() as i32;
        if cmd == 32 {
            if ctxt.mPathFillType.as_deref() == Some("none") {
                ctxt.mSegments.as_mut().expect("mSegments").push(Segment::with_params(170, vertex_count));
                ctxt.mPathFillType = None;
            } else if ctxt.mPathStrokeOn == Some(false) {
                ctxt.mSegments.as_mut().expect("mSegments").push(Segment::with_params(171, vertex_count));
                ctxt.mPathStrokeOn = None;
            }
        }
        let vertex_count = ctxt.mVertices.as_ref().expect("mVertices").len() as i32;
        ctxt.mSegments.as_mut().expect("mSegments").push(Segment::with_params(cmd, vertex_count));
    }

    // Java: public void addArcToVertices(Attributes attrs)
    pub fn addArcToVertices(&mut self, attrs: &dyn SaxAttributes) {
        let w_r = self.getValue(main_string(attrs, "wR").as_deref());
        let h_r = self.getValue(main_string(attrs, "hR").as_deref());
        let sw_ang = self.getValue(main_string(attrs, "swAng").as_deref());
        let st_ang = self.getValue(main_string(attrs, "stAng").as_deref());
        self.addVertice_xy(w_r, h_r);
        self.addVertice_xy(st_ang, sw_ang);
    }

    // Java: private int getValue(String str)
    fn getValue(&self, str: Option<&str>) -> i32 {
        let ctxt = self.mCtxt.borrow();
        let gds = ctxt.mGdlst.as_ref().expect("mGdlst");
        let gds = gds.borrow();
        let value = if gds.contains(str) {
            gds.get(str.map(java_hash), ctxt.mPathW, ctxt.mPathH)
        } else if let Some(str) = str {
            Gds::getPredefinedConstant(java_hash(str), ctxt.mPathW.unwrap_or(0), ctxt.mPathH.unwrap_or(0))
        } else {
            None
        };
        value.unwrap_or(0) as i32
    }

    // Java: public void addVertice(Attributes attrs)
    pub fn addVertice(&mut self, attrs: &dyn SaxAttributes) {
        let mut x = empty_int(attrs, "x");
        let mut y = empty_int(attrs, "y");
        if x.is_none() {
            if let Some(strx) = main_string(attrs, "x") {
                let long_x: Option<i64> = {
                    let (gds, path_w, path_h) = {
                        let ctxt = self.mCtxt.borrow();
                        let _ = Gds::getPredefinedConstant(java_hash(&strx), ctxt.mPathW.unwrap_or(0), ctxt.mPathH.unwrap_or(0));
                        (ctxt.mGdlst.clone(), ctxt.mPathW, ctxt.mPathH)
                    };
                    let guard = gds.as_ref().expect("mGdlst").borrow();
                    guard.get(Some(java_hash(&strx)), path_w, path_h)
                };
                if let Some(long_x) = long_x { x = Some(long_x as i32); }
            }
        }
        if y.is_none() {
            if let Some(stry) = main_string(attrs, "y") {
                let long_y: Option<i64> = {
                    let (gds, path_w, path_h) = {
                        let ctxt = self.mCtxt.borrow();
                        let _ = Gds::getPredefinedConstant(java_hash(&stry), ctxt.mPathW.unwrap_or(0), ctxt.mPathH.unwrap_or(0));
                        (ctxt.mGdlst.clone(), ctxt.mPathW, ctxt.mPathH)
                    };
                    let guard = gds.as_ref().expect("mGdlst").borrow();
                    guard.get(Some(java_hash(&stry)), path_w, path_h)
                };
                if let Some(long_y) = long_y { y = Some(long_y as i32); }
            }
        }
        if let (Some(x), Some(y)) = (x, y) {
            self.addVertice_xy(x, y);
        }
    }

    // Java: public void closePath()
    pub fn closePath(&mut self) {
        let mut ctxt = self.mCtxt.borrow_mut();
        if ctxt.mVertices.as_ref().expect("mVertices").len() >= 2 {
            let count = ctxt.mVertices.as_ref().expect("mVertices").len() as i32;
            ctxt.mSegments.as_mut().expect("mSegments").push(Segment::with_params(96, count));
        }
    }

    // Java: public void startSinglePath(Attributes attrs)
    pub fn startSinglePath(&mut self, attrs: &dyn SaxAttributes) {
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mPathW = empty_int(attrs, "w");
        if ctxt.mPathW.is_none() {
            ctxt.mPathW = Some(ctxt.mExtX.unwrap_or(0) as i32);
        }
        ctxt.mPathH = empty_int(attrs, "h");
        if ctxt.mPathH.is_none() {
            ctxt.mPathH = Some(ctxt.mExtY.unwrap_or(0) as i32);
        }
        ctxt.mPathFillType = main_string(attrs, "fill");
        ctxt.mPathStrokeOn = empty_bool(attrs, "stroke");
    }

    // Java: public void endSinglePath()
    pub fn endSinglePath(&mut self) {
        let mut ctxt = self.mCtxt.borrow_mut();
        let count = ctxt.mVertices.as_ref().expect("mVertices").len() as i32;
        ctxt.mSegments.as_mut().expect("mSegments").push(Segment::with_params(128, count));
        ctxt.mPathFillType = None;
    }

    // Java: private void addVertice(int x, int y)
    fn addVertice_xy(&mut self, x: i32, y: i32) {
        let mut ctxt = self.mCtxt.borrow_mut();
        let vertices = ctxt.mVertices.as_mut().expect("mVertices");
        vertices.push(x);
        vertices.push(y);
    }
}
