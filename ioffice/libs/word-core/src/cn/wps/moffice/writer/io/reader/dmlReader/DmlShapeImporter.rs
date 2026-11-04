// Java: cn.wps.moffice.writer.io.reader.dmlReader.DmlShapeImporter
use std::{any::Any, cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, sync::{Arc, Mutex}};
use drawing::cn::wps::moffice::drawing::GRF::GRF;
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::graphics::GradFill::GradFill;
use drawing::cn::wps::moffice::drawing::graphics::SolidFill::SolidFill;
use drawing::cn::wps::moffice::drawing::line::LineProperty::LineProperty;
use util::moffice::property::{MutablePropertyMap::MutablePropertyMap, PropertyMap::PropertyMap};
use drawing::cn::wps::moffice::drawing::smartshape::ShapeType::ShapeType;
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use util::base::io::TypeConversion::TypeConversion;
use util::graphics::PointF::PointF;
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::Xml07Context;
use super::{
    BlipFillImporter::BlipFillImporter,
    DmlImporterBase::DmlImporterBase,
    GRFImporter::GRFImporter,
    GeometryImporter::GeometryImporter,
    GlowImporter::GlowImporter,
    InnerShdwImporter::InnerShdwImporter,
    OuterShdwImporter::OuterShdwImporter,
    PrstShdwImporter::PrstShdwImporter,
    TextImporter::TextImporter,
    GradFillImporter::GradFillImporter,
    LineImporter::LineImporter,
    PatternFillImporter::PatternFillImporter,
    PictureImporter::PictureImporter,
    PresetTextWarpImporter::PresetTextWarpImporter,
    ShapePosImporter::ShapePosImporter,
    SolidFillImporter::SolidFillImporter,
    context::{
        Cxn::Cxn, Cxnlst::Cxnlst, DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt,
        Gds::Gds, ThemeContext::ThemeContext,
    },
    PositioningContext::PositioningContext,
    helper::EffectHelper::EffectHelper,
};

fn empty_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri("", name) }
}
fn main_string(attrs: &dyn SaxAttributes, name: &str) -> Option<String> {
    if attrs.getLength() == 0 { None } else { attrs.getValue_by_uri(io_wps_io::cn::wps::io::dom::io::WMap::WMap::MAIN_NAMESPACING(), name) }
}
fn empty_long(attrs: &dyn SaxAttributes, name: &str) -> Option<i64> {
    TypeConversion::string2Long(&empty_string(attrs, name)?)
}

// Java: public class DmlShapeImporter extends DmlImporterBase
pub struct DmlShapeImporter<ShapeImporter, DmlCommonImporter: IImporterBase> {
    pub super_: DmlImporterBase<ShapeImporter, DmlCommonImporter>,
    // Java: Shape mTarget;
    pub mTarget: Arc<Mutex<Shape>>,
    // Java: private PositioningContext mPositioningCtxt;
    mPositioningCtxt: Option<Rc<RefCell<PositioningContext>>>,
    // Java: public ShapePosImporter mShpPosImporter;
    pub mShpPosImporter: ShapePosImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public GRFImporter mGRFImporter;
    pub mGRFImporter: GRFImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public LineImporter mLineImporter;
    pub mLineImporter: LineImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public SolidFillImporter mSolidFillImporter;
    pub mSolidFillImporter: SolidFillImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public BlipFillImporter mBlipFillImporter;
    pub mBlipFillImporter: BlipFillImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public GradFillImporter mGradFillImporter;
    pub mGradFillImporter: GradFillImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public GeometryImporter mGeoImporter;
    pub mGeoImporter: GeometryImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public PatternFillImporter mPatternFillImporter;
    pub mPatternFillImporter: PatternFillImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public GlowImporter mGlowImporter;
    pub mGlowImporter: GlowImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public InnerShdwImporter mInnerShdwImporter;
    pub mInnerShdwImporter: InnerShdwImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public OuterShdwImporter mOuterShdwImporter;
    pub mOuterShdwImporter: OuterShdwImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public PrstShdwImporter mPrstShdwImporter;
    pub mPrstShdwImporter: PrstShdwImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public TextImporter mTextboxImporter;
    pub mTextboxImporter: TextImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public PictureImporter mPictureImporter;
    pub mPictureImporter: PictureImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public PresetTextWarpImporter mPrstTextWarpImporter;
    pub mPrstTextWarpImporter: PresetTextWarpImporter<ShapeImporter, DmlCommonImporter>,
}

impl<S, D: IImporterBase> Deref for DmlShapeImporter<S, D> {
    type Target = DmlImporterBase<S, D>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for DmlShapeImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> DmlShapeImporter<S, D> {
    // Java: DmlShapeImporter(Shape, Document, DmlSubDocCtxt, ThemeContext, Xml07Context, DmlImporter)
    pub fn new(shape: Arc<Mutex<Shape>>, doc: Rc<RefCell<Box<dyn Document>>>,
        subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>, themeCtxt: Rc<RefCell<ThemeContext>>,
        xml07Context: Option<Xml07Context>, dmlImporter: Rc<RefCell<D>>) -> Self {
        // Java: super(subDocCtxt, new DmlShapeCtxt(), themeCtxt, xml07Context, dmlImporter);
        let ctxt = Rc::new(RefCell::new(DmlShapeCtxt::new()));
        {
            let mut shape_ctxt = ctxt.borrow_mut();
            // Java: this.mCtxt.mShape = shape; this.mCtxt.mDoc = doc;
            shape_ctxt.mShape = Some(shape.clone());
            shape_ctxt.mDoc = Some(doc);
        }
        let super_ = DmlImporterBase::new(subDocCtxt.clone(), ctxt.clone(), themeCtxt.clone(),
            xml07Context.clone(), dmlImporter.clone());
        Self {
            mShpPosImporter: ShapePosImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            // Java: new SolidFillImporter(..., false); new GlowImporter(..., false);
            mGRFImporter: GRFImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            mLineImporter: LineImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone(), false),
            mSolidFillImporter: SolidFillImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone(), false),
            // Java: new BlipFillImporter(this.mSubDocCtxt, this.mCtxt, this.mThemeCtxt, xml07Context, dmlImporter);
            mBlipFillImporter: BlipFillImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            mGradFillImporter: GradFillImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone(), false),
            mGeoImporter: GeometryImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            mPatternFillImporter: PatternFillImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            mGlowImporter: GlowImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone(), false),
            mInnerShdwImporter: InnerShdwImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            mOuterShdwImporter: OuterShdwImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            mPrstShdwImporter: PrstShdwImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            mTextboxImporter: TextImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            // Java: new PictureImporter(this.mSubDocCtxt, this.mCtxt, this.mThemeCtxt, xml07Context, dmlImporter);
            mPictureImporter: PictureImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            mPrstTextWarpImporter: PresetTextWarpImporter::new(subDocCtxt, ctxt, themeCtxt, xml07Context, dmlImporter),
            super_,
            mPositioningCtxt: None,
            // Java: this.mTarget = shape;
            mTarget: shape,
        }
    }

    // Java: public void importRotWithShape(Attributes attributes)
    pub fn importRotWithShape(&mut self, attributes: &dyn SaxAttributes) {
        // Java: Boolean tmp = ConvertHelper.toBoolean(attributes, "rotWithShape");
        let tmp = attributes.getValue_by_uri("", "rotWithShape").map(|value| {
            !(value.eq_ignore_ascii_case("off")
                || value == "0"
                || value.eq_ignore_ascii_case("false")
                || value.eq_ignore_ascii_case("f"))
        });
        // Java: if (null != tmp) this.mTarget.setRotWithShape(tmp);
        if let Some(tmp) = tmp {
            self.mTarget.lock().expect("shape").setRotWithShape(tmp);
        }
    }

    // Java: public void setPositioningContext(PositioningContext ctxt)
    pub fn setPositioningContext(&mut self, ctxt: Rc<RefCell<PositioningContext>>) {
        self.mPositioningCtxt = Some(ctxt.clone());
        self.mShpPosImporter.setPositioningContext(Some(ctxt));
    }

    // Java: public Shape result()
    pub fn result(&self) -> Arc<Mutex<Shape>> {
        self.mTarget.clone()
    }

    // Java: public void excute()
    pub fn excute(&mut self) {
        if let Some(positioning) = &self.mPositioningCtxt {
            let (in_dgg, inline, z_index) = {
                let positioning = positioning.borrow();
                (positioning.mIsInDggInfo, positioning.mInLine, positioning.mRelativeHeight)
            };
            let mut shape = self.mTarget.lock().expect("shape");
            shape.setInDggInfo(in_dgg);
            shape.setInlineShape(inline);
            if let Some(z_index) = z_index {
                shape.setZIndex(z_index);
            }
        }
        self.importShapeType();
        self.importPrstTextWarp();
    }

    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        if let Some(positioning) = &self.mPositioningCtxt {
            let mut positioning = positioning.borrow_mut();
            // Java: mIsInDggInfo = false; mInLine = false; mRelativeHeight = null;
            positioning.mIsInDggInfo = false;
            positioning.mInLine = false;
            positioning.mRelativeHeight = None;
        }
    }

    // Java: public void importGRF()
    pub fn importGRF(&mut self) {
        self.mGRFImporter.excute();
        let grf = self.mGRFImporter.result();
        // Java: this.mTarget.setGRF(this.mGRFImporter.result());
        self.mTarget.lock().expect("shape").setGRF(grf.map(|grf| grf as Arc<dyn Any + Send + Sync>));
        self.mGRFImporter.clearContext();
    }

    // Java: private void importPrstTextWarp()
    pub fn importPrstTextWarp(&mut self) {
        self.mPrstTextWarpImporter.excute();
        let warp = self.mPrstTextWarpImporter.result();
        // Java: this.mTarget.setPrstTxWarp(this.mPrstTextWarpImporter.result());
        self.mTarget.lock().expect("shape").setPrstTxWarp(warp.map(|warp| warp as Arc<dyn Any + Send + Sync>));
        self.mPrstTextWarpImporter.clearContext();
    }

    // Java: public void importGrpFill()
    pub fn importGrpFill(&mut self) {
        self.mTarget.lock().expect("shape").setGrpFill(true);
    }

    // Java: public void importBlipFill()
    pub fn importBlipFill(&mut self) {
        // Java: this.mBlipFillImporter.excute();
        self.mBlipFillImporter.excute();
        // Java: BlipFill blipFill = this.mBlipFillImporter.result();
        let blip_fill = self.mBlipFillImporter.result();
        let rot_with_shape = blip_fill.as_ref().map(|fill| fill.lock().expect("blip fill").getRotateWithShape());
        {
            let mut shape = self.mTarget.lock().expect("shape");
            // Java: this.mTarget.setBlipFill(blipFill);
            shape.setBlipFill(blip_fill.as_ref().map(|fill| fill.clone() as Arc<dyn Any + Send + Sync>));
            // Java: if (null != blipFill) this.mTarget.setRotWithShape(blipFill.getRotateWithShape());
            if let Some(rot_with_shape) = rot_with_shape {
                shape.setRotWithShape(rot_with_shape);
            }
        }
        // Java: this.mBlipFillImporter.clearContext();
        self.mBlipFillImporter.clearContext();
    }

    // Java: public void importLineProperty()
    pub fn importLineProperty(&mut self) {
        self.mLineImporter.excute();
        let new_prop = self.mLineImporter.result().expect("line");
        let old = self.mTarget.lock().expect("shape").getLineProperty();
        if let Some(old) = old {
            let old = old.downcast::<Mutex<LineProperty>>().expect("LineProperty");
            let merged = {
                let mut new_lock = new_prop.lock().expect("line");
                let mut old_lock = old.lock().expect("old line");
                PropertyMap::merge(&new_lock.base.getProperty().base, &old_lock.base.getProperty().base)
            };
            let mut map = MutablePropertyMap::new();
            map.base = merged;
            new_prop.lock().expect("line").base.setProperty(Some(map));
        }
        self.mTarget.lock().expect("shape").setLineProperty(Some(new_prop as Arc<dyn Any + Send + Sync>));
        self.mLineImporter.clearContext();
    }

    // Java: public void importGradFill()
    pub fn importGradFill(&mut self) {
        self.mGradFillImporter.excute();
        let new_prop = self.mGradFillImporter.result().expect("grad");
        let old = self.mTarget.lock().expect("shape").getGradFill();
        if let Some(old) = old {
            let old = old.downcast::<Mutex<GradFill>>().expect("GradFill");
            let merged = {
                let mut new_lock = new_prop.lock().expect("grad");
                let mut old_lock = old.lock().expect("old grad");
                PropertyMap::merge(&new_lock.base.base.getProperty().base, &old_lock.base.base.getProperty().base)
            };
            let mut map = MutablePropertyMap::new();
            map.base = merged;
            new_prop.lock().expect("grad").base.base.setProperty(Some(map));
        }
        self.mTarget.lock().expect("shape").setGradFill(Some(new_prop as Arc<dyn Any + Send + Sync>));
        self.mGradFillImporter.clearContext();
    }

    // Java: public void importSolidFill()
    pub fn importSolidFill(&mut self) {
        let old = self.mTarget.lock().expect("shape").getFill();
        let replace = match old.as_ref() {
            None => true,
            Some(old) => old.downcast_ref::<SolidFill>().is_some(),
        };
        if !replace { return; }
        self.mSolidFillImporter.excute();
        let new_prop = self.mSolidFillImporter.result().expect("solid");
        let mut stored = SolidFill { base: new_prop.borrow().base.clone() };
        let mut old_owned = old.as_ref().and_then(|old| old.downcast_ref::<SolidFill>()).map(|old| SolidFill { base: old.base.clone() });
        if let Some(old_owned) = old_owned.as_mut() {
            let merged = PropertyMap::merge(&stored.base.base.getProperty().base, &old_owned.base.base.getProperty().base);
            let mut map = MutablePropertyMap::new();
            map.base = merged;
            stored.base.base.setProperty(Some(map));
        }
        self.mTarget.lock().expect("shape").setSolidFill(Some(Arc::new(stored) as Arc<dyn Any + Send + Sync>));
        self.mSolidFillImporter.clearContext();
    }

    // Java: public void importGeometry()
    pub fn importGeometry(&mut self) {
        self.mGeoImporter.excute();
        let geometry = self.mGeoImporter.result();
        self.mTarget.lock().expect("shape").setGeometry(geometry.map(|geometry| geometry as Arc<dyn Any + Send + Sync>));
        self.mGeoImporter.clearContext();
    }

    // Java: public void importPatternFill()
    pub fn importPatternFill(&mut self) {
        self.mPatternFillImporter.excute();
        let pattern = self.mPatternFillImporter.result();
        // Java: this.mTarget.setBlipFill(this.mPatternFillImporter.result());
        self.mTarget.lock().expect("shape").setBlipFill(
            pattern.map(|pattern| pattern as Arc<dyn Any + Send + Sync>));
        // Java: this.mPatternFillImporter.clearContext(); parent field only.
        self.mPatternFillImporter.clearContext();
    }

    // Java: public void importPicture()
    pub fn importPicture(&mut self) {
        // Java: this.mPictureImporter.excute();
        self.mPictureImporter.excute();
        // Java: this.mTarget.setPicture(this.mPictureImporter.result());
        let picture = self.mPictureImporter.result();
        self.mTarget.lock().expect("shape").setPicture(
            picture.as_ref().map(|picture| picture.clone() as Arc<dyn Any + Send + Sync>));
        // Java: this.mPictureImporter.clearContext();
        self.mPictureImporter.clearContext();
    }

    // Java: public void importEffectExtent()
    pub fn importEffectExtent(&mut self) {
        if let Some(positioning) = &self.mPositioningCtxt {
            let (left, top, right, bottom) = {
                let positioning = positioning.borrow();
                (positioning.mEffectExtentLeft, positioning.mEffectExtentTop,
                    positioning.mEffectExtentRight, positioning.mEffectExtentBottom)
            };
            {
                let mut shape = self.mTarget.lock().expect("shape");
                shape.setEffectExtentLeftEdge(left);
                shape.setEffectExtentTopEdge(top);
                shape.setEffectExtentRightEdge(right);
                shape.setEffectExtentBottomEdge(bottom);
            }
            self.clearEffectExtentContext();
        }
    }

    // Java: public void importShapePos()
    pub fn importShapePos(&mut self) {
        self.mShpPosImporter.excute();
        let inline = self.isInline();
        let behind = self.isBehindDocument();
        let locked = self.isLocked();
        let edited = self.isEditedWrap();
        let wrap = self.getWrapCoords();
        let shape_pos = self.mShpPosImporter.result();
        let mut shape = self.mTarget.lock().expect("shape");
        if inline {
            shape.setInlineShape(true);
            shape.setAnchorLock(true);
        } else {
            shape.setBehindDocument(behind);
            shape.setAnchorLock(locked);
            if let Some(edited) = edited {
                shape.setEditedWrap(edited);
            }
            if let Some(wrap) = wrap {
                shape.setWrapcoords(Some(Arc::new(wrap) as Arc<dyn Any + Send + Sync>));
            }
        }
        shape.setShapePos(shape_pos.map(|pos| pos as Arc<dyn Any + Send + Sync>));
    }

    // Java: private void importShapeType()
    fn importShapeType(&mut self) {
        let shape_type = self.mCtxt.borrow_mut().mShapeType.take();
        if let Some(shape_type) = shape_type {
            self.mTarget.lock().expect("shape").setShapeType(shape_type);
        }
    }

    // Java: public void importLineLeftProperty(LineProperty line)
    pub fn importLineLeftProperty(&mut self, _line: &LineProperty) {}
    // Java: public void importLineTopProperty(LineProperty line)
    pub fn importLineTopProperty(&mut self, _line: &LineProperty) {}
    // Java: public void importLineRightProperty(LineProperty line)
    pub fn importLineRightProperty(&mut self, _line: &LineProperty) {}
    // Java: public void importLineBottomProperty(LineProperty line)
    pub fn importLineBottomProperty(&mut self, _line: &LineProperty) {}
    // Java: public void importHidden(boolean bHidden)
    pub fn importHidden(&mut self, _bHidden: bool) {}

    // Java: public void setShapeType(Attributes attributes)
    pub fn setShapeType(&mut self, attributes: &dyn SaxAttributes) {
        let Some(st) = main_string(attributes, "prst") else { return; };
        self.mTarget.lock().expect("shape").setShapeType(0);
        let shape_type = ShapeType::getShapeType(&st);
        let mut ctxt = self.mCtxt.borrow_mut();
        if ctxt.mShapeType.is_none() || (ctxt.mShapeType == Some(204) && shape_type != 1) {
            ctxt.mShapeType = Some(shape_type);
        }
    }

    // Java: public void setShapeType(int st)
    pub fn setShapeType_int(&mut self, st: i32) {
        self.mCtxt.borrow_mut().mShapeType = Some(st);
    }

    // Java: public void setOff(Attributes attrs)
    pub fn setOff(&mut self, attrs: &dyn SaxAttributes) {
        let x = empty_long(attrs, "x");
        let y = empty_long(attrs, "y");
        {
            let mut ctxt = self.mCtxt.borrow_mut();
            ctxt.mOffX = x;
            ctxt.mOffY = y;
        }
        if self.mPositioningCtxt.is_some() && self.super_.super_.isGVml() {
            let mut positioning = self.mPositioningCtxt.as_ref().expect("positioning").borrow_mut();
            positioning.mPositionH = x;
            positioning.mPositionV = y;
        }
    }

    // Java: public void setExt(Attributes attrs)
    pub fn setExt(&mut self, attrs: &dyn SaxAttributes) {
        let cx = empty_long(attrs, "cx");
        let cy = empty_long(attrs, "cy");
        {
            let mut ctxt = self.mCtxt.borrow_mut();
            ctxt.mExtX = cx;
            ctxt.mExtY = cy;
        }
        if self.mPositioningCtxt.is_some() && self.super_.super_.isGVml() {
            let mut positioning = self.mPositioningCtxt.as_ref().expect("positioning").borrow_mut();
            positioning.mWidth = cx;
            positioning.mHeight = cy;
        }
    }

    // Java: public void setChOff(Attributes attrs)
    pub fn setChOff(&mut self, attrs: &dyn SaxAttributes) {
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mChOffX = empty_long(attrs, "x");
        ctxt.mChOffY = empty_long(attrs, "y");
    }

    // Java: public void setChExt(Attributes attrs)
    pub fn setChExt(&mut self, attrs: &dyn SaxAttributes) {
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mChExtX = empty_long(attrs, "cx");
        ctxt.mChExtY = empty_long(attrs, "cy");
    }

    // Java: public void setNoFill()
    pub fn setNoFill(&mut self) {
        self.mCtxt.borrow_mut().mFillOn = false;
    }

    // Java: public void startGdlst()
    pub fn startGdlst(&mut self) {
        let mut ctxt = self.mCtxt.borrow_mut();
        if ctxt.mGdlst.is_none() {
            ctxt.mGdlst = Some(Gds::new());
        }
    }

    // Java: public void addGd(int belonger, Attributes attrs)
    pub fn addGd(&mut self, belonger: i32, attrs: &dyn SaxAttributes) {
        let name = main_string(attrs, "name");
        let fmla = main_string(attrs, "fmla");
        if let (Some(name), Some(fmla)) = (name, fmla) {
            if belonger == 93173592 || belonger == 98178480 {
                self.mCtxt.borrow_mut().mGdlst.as_ref().expect("mGdlst").borrow_mut().add(&name, &fmla);
            }
        }
    }

    // Java: public void startCxnlst()
    pub fn startCxnlst(&mut self) {
        self.mCtxt.borrow_mut().mCxnlst = Some(Cxnlst::new());
    }

    // Java: public void startCxn(Attributes attrs)
    pub fn startCxn(&mut self, attrs: &dyn SaxAttributes) {
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mCxn = Some(Cxn::new());
        if let Some(ang) = main_string(attrs, "ang") {
            ctxt.mCxn.as_mut().expect("mCxn").setAng(&ang);
        }
    }

    // Java: public void endCxn()
    pub fn endCxn(&mut self) {
        let mut ctxt = self.mCtxt.borrow_mut();
        let cxn = ctxt.mCxn.take().expect("mCxn");
        ctxt.mCxnlst.as_mut().expect("mCxnlst").push(cxn);
    }

    // Java: public void addPos(Attributes attrs)
    pub fn addPos(&mut self, attrs: &dyn SaxAttributes) {
        let x = main_string(attrs, "x");
        let y = main_string(attrs, "y");
        if let (Some(x), Some(y)) = (x, y) {
            self.mCtxt.borrow_mut().mCxn.as_mut().expect("mCxn").setXY(&x, &y);
        }
    }

    // Java: public void importFlipVH()
    pub fn importFlipVH(&mut self) {
        let (flip_v, flip_h) = {
            let ctxt = self.mCtxt.borrow();
            (ctxt.mFlipV == Some(true), ctxt.mFlipH == Some(true))
        };
        if !flip_v && !flip_h {
            return;
        }
        let mut shape = self.mTarget.lock().expect("shape");
        let current = shape.getGRF();
        if let Some(arc) = current {
            if let Ok(grf_arc) = arc.downcast::<GRF>() {
                let mut grf = <GRF as Clone>::clone(&grf_arc);
                if flip_v { grf.setFlipV(true); }
                if flip_h { grf.setFlipH(true); }
                shape.setGRF(Some(Arc::new(grf) as Arc<dyn Any + Send + Sync>));
            }
        }
    }

    // Java: public void importShapeName()
    pub fn importShapeName(&mut self) {
        let name = self.mPositioningCtxt.as_ref().and_then(|positioning| positioning.borrow().mName.clone());
        if let Some(name) = name {
            self.mTarget.lock().expect("shape").setShapeName(name);
        }
    }

    // Java: public void importShapeDescr()
    pub fn importShapeDescr(&mut self) {
        let descr = self.mPositioningCtxt.as_ref().and_then(|positioning| positioning.borrow().mDescr.clone());
        if let Some(descr) = descr {
            self.mTarget.lock().expect("shape").setDecription(descr);
        }
    }

    // Java: public void importShapeName(Attributes attributes)
    pub fn importShapeName__Attributes(&mut self, attributes: &dyn SaxAttributes) {
        if let Some(name) = main_string(attributes, "name") {
            self.mTarget.lock().expect("shape").setShapeName(name);
        }
    }

    // Java: private void clearEffectExtentContext()
    fn clearEffectExtentContext(&mut self) {
        let mut positioning = self.mPositioningCtxt.as_ref().expect("positioning").borrow_mut();
        positioning.mEffectExtentLeft = 0.0;
        positioning.mEffectExtentTop = 0.0;
        positioning.mEffectExtentRight = 0.0;
        positioning.mEffectExtentBottom = 0.0;
    }

    // Java: private PointF[] getWrapCoords()
    fn getWrapCoords(&self) -> Option<Vec<PointF>> {
        self.mPositioningCtxt.as_ref().and_then(|positioning| positioning.borrow().mWrapcoords.clone())
    }

    // Java: private Boolean isEditedWrap()
    fn isEditedWrap(&self) -> Option<bool> {
        self.mPositioningCtxt.as_ref().and_then(|positioning| positioning.borrow().mEdited)
    }

    // Java: private boolean isBehindDocument()
    fn isBehindDocument(&self) -> bool {
        self.mPositioningCtxt.as_ref().map(|positioning| positioning.borrow().mBehindDoc).unwrap_or(false)
    }

    // Java: private boolean isLocked()
    fn isLocked(&self) -> bool {
        self.mPositioningCtxt.as_ref().map(|positioning| positioning.borrow().mLocked).unwrap_or(false)
    }

    // Java: private boolean isInline()
    fn isInline(&self) -> bool {
        self.mPositioningCtxt.as_ref().map(|positioning| positioning.borrow().mInLine).unwrap_or(false)
    }

    // Java: public void importInnerShadow()
    pub fn importInnerShadow(&mut self) {
        self.mInnerShdwImporter.excute();
        let shadow = self.mInnerShdwImporter.result();
        {
            let mut shape = self.mTarget.lock().expect("shape");
            if shape.getShadow().is_none() {
                shape.setShadow(shadow.map(|shadow| shadow as Arc<dyn Any + Send + Sync>));
            }
        }
        self.mInnerShdwImporter.clearContext();
    }

    // Java: public void importOuterShadow()
    pub fn importOuterShadow(&mut self) {
        self.mOuterShdwImporter.excute();
        let shadow = self.mOuterShdwImporter.result();
        {
            let mut shape = self.mTarget.lock().expect("shape");
            if shape.getShadow().is_none() {
                shape.setShadow(shadow.map(|shadow| shadow as Arc<dyn Any + Send + Sync>));
            }
        }
        self.mOuterShdwImporter.clearContext();
    }

    // Java: public void importPrstShadow()
    pub fn importPrstShadow(&mut self) {
        self.mPrstShdwImporter.excute();
        let shadow = self.mPrstShdwImporter.result();
        self.mTarget.lock().expect("shape")
            .setShadow(shadow.map(|shadow| shadow as Arc<dyn Any + Send + Sync>));
        self.mPrstShdwImporter.clearContext();
    }

    // Java: public void importTextbox()
    pub fn importTextbox(&mut self) {
        let text = self.mTextboxImporter.result();
        self.mTarget.lock().expect("shape")
            .setText(text.map(|text| text as Arc<dyn Any + Send + Sync>));
    }

    // Java: public void importGlow()
    pub fn importGlow(&mut self) {
        self.mGlowImporter.excute();
        let glow = self.mGlowImporter.result().and_then(|glow| glow.borrow().clone().ok());
        self.mTarget.lock().expect("shape")
            .setGlow(glow.map(|glow| Arc::new(glow) as Arc<dyn Any + Send + Sync>));
        self.mGlowImporter.clearContext();
    }

    // Java: public void importReflection(Attributes attributes)
    pub fn importReflection(&mut self, attributes: &dyn SaxAttributes) {
        if let Some(reflection) = EffectHelper::createReflection(Some(attributes)) {
            self.mTarget.lock().expect("shape")
                .setReflection(Some(Arc::new(reflection) as Arc<dyn Any + Send + Sync>));
        }
    }

    // Java: public void importSoftEdge(Attributes attributes)
    pub fn importSoftEdge(&mut self, attributes: &dyn SaxAttributes) {
        if let Some(soft_edge) = EffectHelper::createSoftEdge(Some(attributes)) {
            self.mTarget.lock().expect("shape")
                .setSoftEdge(Some(Arc::new(soft_edge) as Arc<dyn Any + Send + Sync>));
        }
    }
}
