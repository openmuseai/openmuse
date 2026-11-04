// Java: cn.wps.moffice.writer.io.reader.dmlReader.PatternFillImporter
use std::{any::Any, cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, sync::{Arc, Mutex}};
use drawing::cn::wps::moffice::drawing::graphics::PatternFill::PatternFill;
use drawing::cn::wps::moffice::drawing::graphics::PatternFillUtil::PatternFillUtil;
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::MediaTypeEnum;
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use crate::cn::wps::moffice::writer::drawing::DrawingContainer::DrawingContainerHandle;
use crate::cn::wps::moffice::writer::io::reader::helper::IOHelper::IOHelper;
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    FillBaseImporter::FillBaseImporter,
    context::{
        BgClr::BgClr, ColorTransform::ColorTransformValue, DmlShapeCtxt::DmlShapeCtxt,
        DmlSubDocCtxt::DmlSubDocCtxt, FgClr::FgClr, ThemeContext::ThemeContext,
    },
    helper::ColorsTrans::ColorsTrans,
};

fn main_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), name) }
}

// Java: public class PatternFillImporter extends FillBaseImporter
pub struct PatternFillImporter<ShapeImporter, DmlCommonImporter: IImporterBase> {
    pub super_: FillBaseImporter<ShapeImporter, DmlCommonImporter, PatternFill>,
    // Java: private PatternFill mTarget; shadows FillBaseImporter.mTarget.
    mTarget: Option<Arc<Mutex<PatternFill>>>,
}

impl<S, D: IImporterBase> Deref for PatternFillImporter<S, D> {
    type Target = FillBaseImporter<S, D, PatternFill>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for PatternFillImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> PatternFillImporter<S, D> {
    // Java: PatternFillImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext, Xml07Context, DmlImporter)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>, ctxt: Rc<RefCell<DmlShapeCtxt>>,
        themeCtxt: Rc<RefCell<ThemeContext>>, xml07Context: Option<Xml07Context>,
        dmlImporter: Rc<RefCell<D>>) -> Self {
        Self {
            super_: FillBaseImporter::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter),
            mTarget: None,
        }
    }

    // Java: public void excute()
    pub fn excute(&mut self) {
        self.initTarget();
    }

    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        if self.mTarget.is_none() {
            self.mTarget = Some(Arc::new(Mutex::new(PatternFill::new())));
        }
    }

    // Java: public PatternFill result()
    pub fn result(&self) -> Option<Arc<Mutex<PatternFill>>> {
        self.mTarget.clone()
    }

    // Java: public void importPatternFill(Attributes attributes)
    pub fn importPatternFill(&mut self, attributes: &dyn SaxAttributes) {
        let Some(prst) = main_string(attributes, "prst") else { return; };
        // Java: byte[] patternFill = PatternFillUtil.prstStrMatcheToBitByte(prst);
        let pattern_fill = PatternFillUtil::prstStrMatcheToBitByte(&prst);
        // Java: String path = IOHelper.dumpPatternFill(patternFill);
        let Some(path) = IOHelper::dumpPatternFill(&pattern_fill) else { return; };
        self.initTarget();
        let shape = self.mCtxt.borrow().mShape.clone().expect("mShape");
        let shape = shape.lock().expect("shape");
        let container = shape.getDrawingContainer().clone();
        let handle = container.expect("drawingContainer").downcast::<DrawingContainerHandle>().expect("DrawingContainerHandle");
        // Java: mediaLib.addMedia(path, path, MediaTypeEnum.PICTURE);
        let media_id = handle.with_media_lib(|lib| lib.addMedia_impl(&path, &path, MediaTypeEnum::PICTURE));
        self.mTarget.as_ref().expect("mTarget").lock().expect("pattern").setMediaID(media_id);
    }

    // Java: public void importColor()
    pub fn importColor(&mut self) {
        let prepared = {
            let ctxt = self.mCtxt.borrow();
            let Some(fg) = ctxt.mFgClr.as_ref() else { return; };
            let theme = self.mThemeCtxt.borrow();
            let clr = ColorsTrans::toColor(None, &fg.base, Some(&theme));
            let transparency = ColorsTrans::toTransparency(&fg.base, false);
            clr.map(|clr| (clr, transparency))
        };
        if let Some((clr, transparency)) = prepared {
            self.initTarget();
            let color = ColorsTrans::rgb2Argb(clr, 1.0 - transparency);
            {
                let mut target = self.mTarget.as_ref().expect("mTarget").lock().expect("pattern");
                target.setColor(color);
                target.setTransparency(transparency);
            }
            self.mCtxt.borrow_mut().mFgClr = None;
        }
    }

    // Java: public void importBackColor()
    pub fn importBackColor(&mut self) {
        let prepared = {
            let ctxt = self.mCtxt.borrow();
            let Some(bg) = ctxt.mBgClr.as_ref() else { return; };
            let theme = self.mThemeCtxt.borrow();
            let clr = ColorsTrans::toColor(None, &bg.base, Some(&theme));
            let transparency = ColorsTrans::toTransparency(&bg.base, false);
            clr.map(|clr| (clr, transparency))
        };
        if let Some((clr, transparency)) = prepared {
            self.initTarget();
            let color = ColorsTrans::rgb2Argb(clr, 1.0 - transparency);
            {
                let mut target = self.mTarget.as_ref().expect("mTarget").lock().expect("pattern");
                target.setBackColor(color);
                target.setBackTransparency(transparency);
            }
            self.mCtxt.borrow_mut().mBgClr = None;
        }
    }

    // Java: public void createFgClr(Attributes attrs)
    pub fn createFgClr(&mut self, _attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mFgClr = Some(FgClr::new());
    }

    // Java: public void createBgClr(Attributes attrs)
    pub fn createBgClr(&mut self, _attrs: &dyn SaxAttributes) {
        self.mCtxt.borrow_mut().mBgClr = Some(BgClr::new());
    }

    // Java: public void setSysClr(boolean isBg)
    pub fn setSysClr(&mut self, isBg: bool) {
        self.move_clr(isBg, |value| match value {
            ColorTransformValue::SysClr(clr) => Ok(clr),
            other => Err(other),
        }, |fg, clr| fg.base.mSysClr = Some(clr), |bg, clr| bg.base.mSysClr = Some(clr));
    }

    // Java: public void setPrstClr(boolean isBg)
    pub fn setPrstClr(&mut self, isBg: bool) {
        self.move_clr(isBg, |value| match value {
            ColorTransformValue::PrstClr(clr) => Ok(clr),
            other => Err(other),
        }, |fg, clr| fg.base.mPrstClr = Some(clr), |bg, clr| bg.base.mPrstClr = Some(clr));
    }

    // Java: public void setSchemeClr(boolean isBg)
    pub fn setSchemeClr(&mut self, isBg: bool) {
        self.move_clr(isBg, |value| match value {
            ColorTransformValue::SchemeClr(clr) => Ok(clr),
            other => Err(other),
        }, |fg, clr| fg.base.mSchemeClr = Some(clr), |bg, clr| bg.base.mSchemeClr = Some(clr));
    }

    // Java: public void setSrgbClr(boolean isBg)
    pub fn setSrgbClr(&mut self, isBg: bool) {
        self.move_clr(isBg, |value| match value {
            ColorTransformValue::SrgbClr(clr) => Ok(clr),
            other => Err(other),
        }, |fg, clr| fg.base.mSrgbClr = Some(clr), |bg, clr| bg.base.mSrgbClr = Some(clr));
    }

    // Java: public void setScrgbClr(boolean isBg)
    pub fn setScrgbClr(&mut self, isBg: bool) {
        self.move_clr(isBg, |value| match value {
            ColorTransformValue::ScrgbClr(clr) => Ok(clr),
            other => Err(other),
        }, |fg, clr| fg.base.mScrgbClr = Some(clr), |bg, clr| bg.base.mScrgbClr = Some(clr));
    }

    fn move_clr<T>(&mut self, isBg: bool,
        cast: impl FnOnce(ColorTransformValue) -> Result<T, ColorTransformValue>,
        set_fg: impl FnOnce(&mut FgClr, T), set_bg: impl FnOnce(&mut BgClr, T)) {
        let mut ctxt = self.mCtxt.borrow_mut();
        let value = ctxt.mClrTransform.take().expect("mClrTransform");
        let clr = cast(value).unwrap_or_else(|_| panic!("color cast"));
        if isBg {
            set_bg(ctxt.mBgClr.as_mut().expect("mBgClr"), clr);
        } else {
            set_fg(ctxt.mFgClr.as_mut().expect("mFgClr"), clr);
        }
    }
}
