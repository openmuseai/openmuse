// Java: cn.wps.moffice.writer.io.reader.dmlReader.GradFillImporter
use std::{any::Any, cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, sync::{Arc, Mutex}};
use drawing::cn::wps::moffice::drawing::graphics::{
    GradFill::GradFill, GradStop::GradStop, GradStopList::GradStopList,
};
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use util::base::assertion::IOAssert::IOAssert;
use util::base::io::MetricUtility::MetricUtility;
use util::graphics::RectF::RectF;
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    FillBaseImporter::FillBaseImporter,
    context::{
        ColorTransform::ColorTransformValue, DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt,
        Gs::Gs, ThemeContext::ThemeContext,
    },
    helper::ColorsTrans::ColorsTrans,
};

fn java_hash(value: &str) -> i32 {
    value.encode_utf16().fold(0_i32, |hash, ch| hash.wrapping_mul(31).wrapping_add(ch as i32))
}
fn main_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), name) }
}
fn empty_bool(attrs: &dyn SaxAttributes, name: &str) -> Option<bool> {
    let value = if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri("", name) }?;
    Some(!(value.eq_ignore_ascii_case("off") || value == "0" || value.eq_ignore_ascii_case("false") || value.eq_ignore_ascii_case("f")))
}

// Java: public class GradFillImporter extends FillBaseImporter
pub struct GradFillImporter<ShapeImporter, DmlCommonImporter: IImporterBase> {
    pub super_: FillBaseImporter<ShapeImporter, DmlCommonImporter, GradFill>,
    // Java: private GradFill mTarget;
    mTarget: Option<Arc<Mutex<GradFill>>>,
    // Java: private boolean mIsText;
    mIsText: bool,
}

impl<S, D: IImporterBase> Deref for GradFillImporter<S, D> {
    type Target = FillBaseImporter<S, D, GradFill>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for GradFillImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> GradFillImporter<S, D> {
    // Java: GradFillImporter(..., boolean isText)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>, ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>, xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<D>>, isText: bool) -> Self {
        Self {
            super_: FillBaseImporter::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter),
            mTarget: None,
            mIsText: isText,
        }
    }

    // Java: public void excute()
    pub fn excute(&mut self) {
        self.initTarget();
        self.importFillType();
        self.importAngle();
        self.importFillRect();
        self.importTileRect();
    }

    // Java: public GradFill result()
    pub fn result(&self) -> Option<Arc<Mutex<GradFill>>> {
        self.mTarget.clone()
    }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        self.super_.clearContext();
        self.mTarget = None;
    }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        if self.mTarget.is_none() {
            let mut fill = GradFill::new();
            fill.setFocus(100.0);
            self.mTarget = Some(Arc::new(Mutex::new(fill)));
        }
    }

    // Java: public void importGradStop()
    pub fn importGradStop(&mut self) {
        let stop = {
            let ctxt = self.mCtxt.borrow();
            let gs = ctxt.mGs.as_ref().expect("mGs");
            let theme = self.mThemeCtxt.borrow();
            let mut ret = GradStop::new();
            if let Some(clr) = ColorsTrans::toColor(ctxt.mPhClr, &gs.base, Some(&theme)) {
                let alpha = ColorsTrans::toTransparency(&gs.base, self.mIsText);
                ret.setColr(ColorsTrans::rgb2Argb(clr, 1.0 - alpha));
            }
            if let Some(pos) = gs.mPos {
                ret.setPostion(pos);
            }
            ret
        };
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mGs = None;
        let list = ctxt.mGsLst.as_mut().expect("mGsLst");
        if list.len() < 10 {
            list.push(stop);
        }
    }

    // Java: public void importGradStopLst()
    pub fn importGradStopLst(&mut self) {
        self.initTarget();
        let list = {
            let mut ctxt = self.mCtxt.borrow_mut();
            let empty = ctxt.mGsLst.as_ref().expect("mGsLst").is_empty();
            if empty { None } else { ctxt.mGsLst.take() }
        };
        let Some(list) = list else { return; };
        let mut grad_stop_lst = GradStopList::new();
        for gs in list {
            grad_stop_lst.add(gs);
        }
        grad_stop_lst.sort();
        let alpha = grad_stop_lst.at(0).expect("grad stop").getAlpha();
        let mut target = self.mTarget.as_ref().expect("mTarget").lock().expect("grad");
        target.setGradStopList(Some(Arc::new(grad_stop_lst) as Arc<dyn Any + Send + Sync>));
        target.base.setBackTransparency(IOHelper::toOpacity(alpha));
    }

    // Java: private void importAngle()
    fn importAngle(&mut self) {
        let ang = self.mCtxt.borrow().mAng.clone();
        let Some(ang) = ang else { return; };
        let Some(ang) = IOHelper::PositiveFixedAngle2Integer(Some(&ang)) else { return; };
        let mut f_ang = MetricUtility::ST_ANGLE_Degrees2Degree(ang);
        f_ang = (90.0 - f_ang + 360.0) % 360.0;
        self.mTarget.as_ref().expect("mTarget").lock().expect("grad").setAngle(f_ang);
        self.mCtxt.borrow_mut().mAng = None;
    }

    // Java: private void importFillType()
    fn importFillType(&mut self) {
        let scaled = self.mCtxt.borrow_mut().mScaled.take();
        if let Some(scaled) = scaled {
            let fill_type = if scaled { 7 } else { 4 };
            self.mTarget.as_ref().expect("mTarget").lock().expect("grad").base.setFillType(fill_type);
        }
        let path = self.mCtxt.borrow_mut().mPath.take();
        if let Some(path) = path {
            let fill_type = match java_hash(&path) {
                -1360216880 => Some(10),
                3496420 => Some(5),
                109399969 => Some(6),
                _ => {
                    IOAssert::fail();
                    None
                }
            };
            if let Some(fill_type) = fill_type {
                self.mTarget.as_ref().expect("mTarget").lock().expect("grad").base.setFillType(fill_type);
            }
        }
    }

    // Java: private void importFillRect()
    fn importFillRect(&mut self) {
        if let Some(rect) = self.mCtxt.borrow_mut().mFillRect.take() {
            self.mTarget.as_ref().expect("mTarget").lock().expect("grad").base.setFillToRect(rect);
        }
    }

    // Java: private void importTileRect()
    fn importTileRect(&mut self) {
        if let Some(rect) = self.mCtxt.borrow_mut().mTileRect.take() {
            self.mTarget.as_ref().expect("mTarget").lock().expect("grad")
                .setTileRect(Some(Arc::new(rect) as Arc<dyn Any + Send + Sync>));
        }
    }

    // Java: public void setLin(Attributes attributes)
    pub fn setLin(&mut self, attributes: &dyn SaxAttributes) {
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mAng = main_string(attributes, "ang");
        ctxt.mScaled = Some(empty_bool(attributes, "scaled").unwrap_or(false));
    }

    // Java: public void setPath(Attributes attributes)
    pub fn setPath(&mut self, attributes: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mPath = Some(main_string(attributes, "path").unwrap_or_else(|| "rect".to_string()));
    }

    // Java: public void createGsLstContext()
    pub fn createGsLstContext(&mut self) {
        self.mCtxt.borrow_mut().mGsLst = Some(Vec::new());
    }

    // Java: public void createGsContext(Attributes attrs)
    pub fn createGsContext(&mut self, attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mGs = Some(Gs::new(attrs));
    }

    // Java: public void setSysClr()
    pub fn setSysClr(&mut self) { self.move_gs(|value| match value { ColorTransformValue::SysClr(clr) => Ok(clr), other => Err(other) }, |gs, clr| gs.base.mSysClr = Some(clr)); }
    // Java: public void setPrstClr()
    pub fn setPrstClr(&mut self) { self.move_gs(|value| match value { ColorTransformValue::PrstClr(clr) => Ok(clr), other => Err(other) }, |gs, clr| gs.base.mPrstClr = Some(clr)); }
    // Java: public void setSchemeClr()
    pub fn setSchemeClr(&mut self) { self.move_gs(|value| match value { ColorTransformValue::SchemeClr(clr) => Ok(clr), other => Err(other) }, |gs, clr| gs.base.mSchemeClr = Some(clr)); }
    // Java: public void setSrgbClr()
    pub fn setSrgbClr(&mut self) { self.move_gs(|value| match value { ColorTransformValue::SrgbClr(clr) => Ok(clr), other => Err(other) }, |gs, clr| gs.base.mSrgbClr = Some(clr)); }
    // Java: public void setScrgbClr()
    pub fn setScrgbClr(&mut self) { self.move_gs(|value| match value { ColorTransformValue::ScrgbClr(clr) => Ok(clr), other => Err(other) }, |gs, clr| gs.base.mScrgbClr = Some(clr)); }

    fn move_gs<T>(&mut self, cast: impl FnOnce(ColorTransformValue) -> Result<T, ColorTransformValue>,
        set: impl FnOnce(&mut Gs, T)) {
        let mut ctxt = self.mCtxt.borrow_mut();
        let value = ctxt.mClrTransform.take().expect("mClrTransform");
        let clr = cast(value).unwrap_or_else(|_| panic!("color cast"));
        set(ctxt.mGs.as_mut().expect("mGs"), clr);
    }

    // Java: public void setFillToRect(Attributes attributes)
    pub fn setFillToRect(&mut self, attributes: &dyn SaxAttributes) {
        let left = Self::percent(attributes, "l").unwrap_or(0.0);
        let top = Self::percent(attributes, "t").unwrap_or(0.0);
        let right = Self::percent(attributes, "r").unwrap_or(left);
        let bottom = Self::percent(attributes, "b").unwrap_or(top);
        let right = self.gradTransform(right);
        let bottom = self.gradTransform(bottom);
        self.mCtxt.borrow_mut().mFillRect = Some(RectF::new_with(left, top, right, bottom));
    }

    // Java: public void setTileRect(Attributes attributes)
    pub fn setTileRect(&mut self, attributes: &dyn SaxAttributes) {
        let left = Self::percent(attributes, "l").unwrap_or(0.0);
        let top = Self::percent(attributes, "t").unwrap_or(0.0);
        let right = Self::percent(attributes, "r").unwrap_or(0.0);
        let bottom = Self::percent(attributes, "b").unwrap_or(0.0);
        self.mCtxt.borrow_mut().mTileRect = Some(RectF::new_with(left, top, right, bottom));
    }

    // Java: private float gradTransform(float value)
    fn gradTransform(&self, value: f32) -> f32 {
        if self.mIsText { value } else { 1.0 - value }
    }

    fn percent(attributes: &dyn SaxAttributes, name: &str) -> Option<f32> {
        IOHelper::Percentage2Float(main_string(attributes, name).as_deref())
    }
}
