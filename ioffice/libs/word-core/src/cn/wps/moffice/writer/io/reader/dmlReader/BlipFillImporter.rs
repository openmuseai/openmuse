#![allow(non_snake_case)]

use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc, sync::{Arc, Mutex}};
use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use drawing::cn::wps::moffice::drawing::graphics::{BlipFill::BlipFill, Picture::Picture, TileInfo::TileInfo};
use io_wps_io::cn::wps::io::dom::io::WMap::WMap;
use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use io_wps_io::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use util::{base::io::TypeConversion::TypeConversion, cn::wps::graphics::RectF::RectF};
use crate::cn::wps::moffice::writer::core::Document::Document;
use crate::cn::wps::moffice::writer::core::SubDocument::SubDocument;
use crate::cn::wps::moffice::writer::core::TextDocument::TextDocument;
use crate::cn::wps::moffice::writer::io::reader::xml07::cl::ClRefInfo::ClRefInfo;
use crate::cn::wps::moffice::writer::io::reader::{helper::IOHelper::IOHelper,
    xml07::Xml07Context::Xml07Context};
use super::{ClrFromImporter::ClrFromImporter, DuotoneImporter::DuotoneImporter,
    FillBaseImporter::FillBaseImporter,
    helper::{ColorsTrans::ColorsTrans, MetricHelper::MetricHelper},
    context::{DmlShapeCtxt::DmlShapeCtxt, DmlSubDocCtxt::DmlSubDocCtxt,
        ThemeContext::ThemeContext, Tile::Tile}};

// Java: public class BlipFillImporter extends FillBaseImporter
pub struct BlipFillImporter<ShapeImporter, DmlCommonImporter: IImporterBase> {
    pub super_: FillBaseImporter<ShapeImporter, DmlCommonImporter, BlipFill>,
    // Java: protected BlipFill mTarget; shadows FillBaseImporter.mTarget.
    // PictureImporter assigns the same object to its Picture field and this slot.
    pub mTarget: Option<Arc<Mutex<BlipFill>>>,
    // Java: PictureImporter.mTarget when this importer is the picture subclass.
    pub mPicture: Option<Arc<Mutex<Picture>>>,
    // Java: PictureImporter overrides initTarget / importBlip / excute / importSrcRect.
    pub(crate) is_picture: bool,
    // Java: public DuotoneImporter mDuotoneImporter;
    pub mDuotoneImporter: DuotoneImporter<ShapeImporter, DmlCommonImporter>,
    // Java: public ClrFromImporter mClrFromImporter;
    pub mClrFromImporter: ClrFromImporter<ShapeImporter, DmlCommonImporter>,
}
impl<S, D: IImporterBase> Deref for BlipFillImporter<S, D> {
    type Target = FillBaseImporter<S, D, BlipFill>;
    fn deref(&self) -> &Self::Target { &self.super_ }
}
impl<S, D: IImporterBase> DerefMut for BlipFillImporter<S, D> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.super_ }
}

impl<S, D: IImporterBase> BlipFillImporter<S, D> {
    // Java: BlipFillImporter(DmlSubDocCtxt, DmlShapeCtxt, ThemeContext,
    //                        Xml07Context, DmlCommonImporter)
    pub fn new(subDocCtxt: Rc<RefCell<DmlSubDocCtxt<S>>>,
        ctxt: Rc<RefCell<DmlShapeCtxt>>, themeCtxt: Rc<RefCell<ThemeContext>>,
        xml07Context: Option<Xml07Context>, dmlImporter: Rc<RefCell<D>>) -> Self {
        Self {
            super_: FillBaseImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            mTarget: None,
            mPicture: None,
            is_picture: false,
            // Java: this.mDuotoneImporter = new DuotoneImporter(...);
            mDuotoneImporter: DuotoneImporter::new(subDocCtxt.clone(), ctxt.clone(),
                themeCtxt.clone(), xml07Context.clone(), dmlImporter.clone()),
            // Java: this.mClrFromImporter = new ClrFromImporter(...);
            mClrFromImporter: ClrFromImporter::new(subDocCtxt, ctxt,
                themeCtxt, xml07Context, dmlImporter),
        }
    }
    // Java: protected void initTarget()
    pub fn initTarget(&mut self) {
        // Java: PictureImporter.initTarget assigns one Picture to both fields.
        if self.is_picture {
            if self.mPicture.is_none() {
                self.mPicture = Some(Arc::new(Mutex::new(Picture::new())));
            }
            return;
        }
        // Java: if (null == this.mTarget) this.mTarget = new BlipFill();
        if self.mTarget.is_none() { self.mTarget = Some(Arc::new(Mutex::new(BlipFill::new()))); }
    }
    // Java: public BlipFill result()
    pub fn result(&self) -> Option<Arc<Mutex<BlipFill>>> { self.mTarget.clone() }
    // Java: public void clearContext()
    pub fn clearContext(&mut self) {
        // Java: super.clearContext(); this.mTarget = null;
        self.super_.clearContext(); self.mTarget = None; self.mPicture = None;
    }
    // Java: public void excute()
    pub fn excute(&mut self) {
        // Java: PictureImporter.excute() is empty.
        if self.is_picture { return; }
        // Java: this.initTarget(); IOAssert.assertNotNull(...);
        self.initTarget();
        // Java: this.importFillType(); this.importFillRect(); this.importFillTile();
        self.importFillType(); self.importFillRect(); self.importFillTile();
    }
    fn with_fill(&self, f: impl FnOnce(&mut BlipFill)) {
        if let Some(picture) = &self.mPicture {
            f(&mut *picture.lock().expect("picture lock"));
            return;
        }
        let fill = self.mTarget.as_ref().expect("mTarget should not be null!");
        f(&mut *fill.lock().expect("blip fill lock"));
    }
    // Java: public void setFillType(int fillType)
    pub fn setFillType(&mut self, fillType: i32) { self.mCtxt.borrow_mut().mFillType = Some(fillType); }
    // Java: private void importFillType()
    pub fn importFillType(&mut self) {
        // Java: if (null != mCtxt.mFillType) mTarget.setFillType(mCtxt.mFillType);
        if let Some(fill_type) = self.mCtxt.borrow_mut().mFillType.take() {
            self.with_fill(|fill| fill.setFillType(fill_type));
        }
        // Java: this.mCtxt.mFillType = null;
    }
    // Java: public void setFillRect(Attributes attributes)
    pub fn setFillRect(&mut self, attrs: &dyn SaxAttributes) {
        // Java: ConvertHelper.getString(attrs, "l"/"t"/"r"/"b") then Percentage2Float.
        let get = |name| IOHelper::Percentage2Float(
            if attrs.getLength() == 0 { None } else {
                attrs.getValue_by_uri(io_wps_io::cn::wps::io::dom::io::WMap::WMap::MAIN_NAMESPACING(), name)
            }.as_deref());
        let left = get("l"); let top = get("t"); let right = get("r"); let bottom = get("b");
        // Java: if any coordinate is non-null, missing l/t become 0; missing r/b copy l/t.
        if left.is_some() || top.is_some() || right.is_some() || bottom.is_some() {
            let left = left.unwrap_or(0.0); let top = top.unwrap_or(0.0);
            self.mCtxt.borrow_mut().mFillRect = Some(RectF::new_with(left, top,
                right.unwrap_or(left), bottom.unwrap_or(top)));
        }
    }
    // Java: private void importFillRect()
    pub fn importFillRect(&mut self) {
        // Java: if (null != mCtxt.mFillRect) mTarget.setFillRect(mCtxt.mFillRect);
        if let Some(rect) = self.mCtxt.borrow_mut().mFillRect.take() {
            self.with_fill(|fill| fill.setFillRect(rect));
        }
        // Java: this.mCtxt.mFillRect = null;
    }
    // Java: public void setTile(Attributes attrs)
    pub fn setTile(&mut self, attrs: &dyn SaxAttributes) {
        // Java: this.mCtxt.mFillType = 2; this.mCtxt.mTile = new Tile(attrs);
        let mut ctxt = self.mCtxt.borrow_mut();
        ctxt.mFillType = Some(2); ctxt.mTile = Some(Tile::new(attrs));
    }
    // Java: private void importFillTile()
    pub fn importFillTile(&mut self) {
        // Java: if (null != this.mCtxt.mTile)
        let Some(tile) = self.mCtxt.borrow_mut().mTile.take() else { return; };
        // Java: TileInfo tileInfo = new TileInfo();
        let mut info = TileInfo::new();
        if let Some(value) = tile.mTx { info.setOffsetX(value); }
        if let Some(value) = tile.mTy { info.setOffsetY(value); }
        if let Some(value) = tile.mSx { info.setScaleX(value); }
        if let Some(value) = tile.mSy { info.setScaleY(value); }
        // Java: tileInfo.setAlign(mAlgn); tileInfo.setFlipMode(mFlip);
        info.setAlign(tile.mAlgn); info.setFlipMode(tile.mFlip);
        // Java: this.mTarget.setTileInfo(tileInfo); this.mCtxt.mTile = null;
        self.with_fill(|fill| fill.setTileInfo(info));
    }
    // Java: private int transBrightnessContrast(Float value)
    fn transBrightnessContrast(&self, value: f32) -> i32 {
        // Java: return (int)(value * 50.0F) + 50;
        (value * 50.0_f32) as i32 + 50
    }
    // Java: public void importBiLevelThresh(Attributes attrs)
    pub fn importBiLevelThresh(&mut self, attrs: &dyn SaxAttributes) {
        self.with_fill(|fill| fill.setPicBiLevel(true));
        let value = attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), "thresh");
        if let Some(thresh) = IOHelper::Percentage2Float(value.as_deref()) {
            self.with_fill(|fill| fill.setBiLevelThresh(thresh));
        }
    }
    // Java: public void importTransparency(Attributes attrs)
    pub fn importTransparency(&mut self, attrs: &dyn SaxAttributes) {
        if let Some(amount) = MetricHelper::percent2Float(attrs, "amt") {
            self.initTarget();
            self.with_fill(|fill| fill.setTransparency(IOHelper::toOpacity(amount)));
        }
    }
    // Java: public void importSrcRect(Attributes attrs)
    pub fn importSrcRect(&mut self, attrs: &dyn SaxAttributes) {
        // Java: PictureImporter.importSrcRect sets crop on each present edge.
        if self.is_picture {
            let picture = self.mPicture.clone().expect("mTarget");
            let mut picture = picture.lock().expect("picture lock");
            let value = |name| {
                if attrs.getLength() == 0 { None } else {
                    attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), name)
                }
            };
            if let Some(left) = value("l").as_deref().and_then(|v| IOHelper::Percentage2Float(Some(v))) {
                picture.setCropLeft(left);
            }
            if let Some(top) = value("t").as_deref().and_then(|v| IOHelper::Percentage2Float(Some(v))) {
                picture.setCropTop(top);
            }
            if let Some(right) = value("r").as_deref().and_then(|v| IOHelper::Percentage2Float(Some(v))) {
                picture.setCropRight(right);
            }
            if let Some(bottom) = value("b").as_deref().and_then(|v| IOHelper::Percentage2Float(Some(v))) {
                picture.setCropBottom(bottom);
            }
            return;
        }
        let get = |name| IOHelper::Percentage2Float(attrs.getValue_by_uri(
            WMap::MAIN_NAMESPACING(), name).as_deref()).unwrap_or(0.0);
        self.with_fill(|fill| fill.setSrcRect(RectF::new_with(get("l"), get("t"), get("r"), get("b"))));
    }
    // Java: public void importDuotoneColor()
    pub fn importDuotoneColor(&mut self) {
        let duotone = self.mCtxt.borrow_mut().mDuotone.take();
        let Some(duotone) = duotone else { return; };
        if duotone.size() == 0 {
            self.mCtxt.borrow_mut().mDuotone = Some(duotone);
            return;
        }
        let theme = self.mThemeCtxt.borrow();
        let color1 = ColorsTrans::toColor(None, duotone.getColorChoiceByIndex(0), Some(&theme));
        let color2 = if duotone.size() >= 2 {
            ColorsTrans::toColor(None, duotone.getColorChoiceByIndex(1), Some(&theme))
        } else { None }.unwrap_or(32767);
        self.with_fill(|target| {
            target.setPictureDuotone(true);
            if let Some(color1) = color1 { target.setPictureDuoColor1(color1); }
            target.setPictureDuoColor2(color2);
        });
    }
    // Java: public void importLum(Attributes attrs)
    pub fn importLum(&mut self, attrs: &dyn SaxAttributes) {
        if let Some(brightness) = MetricHelper::percent2Float(attrs, "bright") {
            let brightness = self.transBrightnessContrast(brightness) as f32;
            self.with_fill(|fill| fill.setPicBrightness(brightness));
        }
        if let Some(contrast) = MetricHelper::percent2Float(attrs, "contrast") {
            let contrast = self.transBrightnessContrast(contrast) as f32;
            self.with_fill(|fill| fill.setPicContrast(contrast));
        }
    }
    // Java: public void importGrayscl()
    pub fn importGrayscl(&mut self) { self.with_fill(|fill| fill.setPictureGray(true)); }
    // Java: public void importClrFromColor()
    pub fn importClrFromColor(&mut self) {
        let ctxt = self.mCtxt.borrow();
        if let Some(clr_from) = ctxt.mClrFrom.as_ref() {
            let theme = self.mThemeCtxt.borrow();
            if let Some(color) = ColorsTrans::toColor(None, &clr_from.base, Some(&theme)) {
                self.with_fill(|fill| fill.setPicTransparency(color));
            }
        }
    }
    // Java: public void setRotateWithShape(Attributes attributes)
    pub fn setRotateWithShape(&mut self, attrs: &dyn SaxAttributes) {
        // Java: Long dpi = ConvertHelper.getLong4EmptyNamespace(attributes, "dpi");
        let _dpi = attrs.getValue_by_uri("", "dpi").and_then(|v| TypeConversion::string2Long(&v));
        // Java: Boolean rotWithShape = ConvertHelper.toBoolean(attributes, "rotWithShape");
        let rotate = attrs.getValue_by_uri("", "rotWithShape").map(|v| {
            !(v.eq_ignore_ascii_case("off") || v == "0" ||
                v.eq_ignore_ascii_case("false") || v.eq_ignore_ascii_case("f"))
        });
        // Java: this.mCtxt.mBlipFillRotWithShape = null == rotWithShape ? true : rotWithShape;
        self.mCtxt.borrow_mut().mBlipFillRotWithShape = rotate.unwrap_or(true);
    }

    // Java: public void importBlip(Attributes attrs, POIXMLDocumentPart part)
    pub fn importBlip(&mut self, attrs: &dyn SaxAttributes, part: &POIXMLDocumentPart) {
        // Java: String rId = ConvertHelper.getString(attrs, "embed");
        let r_id = if attrs.getLength() == 0 {
            None
        } else {
            attrs.getValue_by_uri(WMap::MAIN_NAMESPACING(), "embed")
        };
        if self.is_picture {
            // Java: PictureImporter.importBlip
            self.import_picture_blip(r_id.as_deref(), part);
            return;
        }
        if self.super_.super_.super_.isXml07() {
            // Java: if (null == rId) return; initTarget(); collect CoBlipFill
            let Some(r_id) = r_id.as_deref() else { return; };
            self.initTarget();
            self.collect_xml07_blip(r_id);
        } else {
            // Java: PictureData picData = IOHelper.getPictureData(rId, part);
            let Some(pic_data) = IOHelper::getPictureData(r_id.as_deref(), part) else { return; };
            // Java: this.mTarget = IOHelper.dumpPicture4BlipFill(shape, picData, doc.getTextDocument(), 2);
            self.mPicture = None;
            self.mTarget = Some(Arc::new(Mutex::new(self.dump_blip_fill(&pic_data))));
        }
        // Java: this.mCtxt.mFillType = 2;
        let rot = self.mCtxt.borrow().mBlipFillRotWithShape;
        self.mCtxt.borrow_mut().mFillType = Some(2);
        // Java: this.mTarget.setRotateWithShape(this.mCtxt.mBlipFillRotWithShape);
        self.with_fill(|fill| fill.setRotateWithShape(rot));
        // Java: this.mCtxt.mBlipFillRotWithShape = true;
        self.mCtxt.borrow_mut().mBlipFillRotWithShape = true;
    }

    // Java: PictureImporter.importBlip body
    fn import_picture_blip(&mut self, r_id: Option<&str>, part: &POIXMLDocumentPart) {
        if self.super_.super_.super_.isXml07() {
            self.initTarget();
            if let Some(r_id) = r_id {
                self.collect_xml07_blip(r_id);
            }
            return;
        }
        // Java: PictureData picData = IOHelper.getPictureData(rId, part);
        match IOHelper::getPictureData(r_id, part) {
            None => {
                // Java: if (null == picData) this.initTarget();
                self.initTarget();
            }
            Some(pic_data) => {
                // Java: this.mTarget = IOHelper.dumpPicture(...); super.mTarget = this.mTarget;
                let shape = self.mCtxt.borrow().mShape.clone().expect("mShape");
                let text_document = text_document_ptr(&self.mCtxt.borrow());
                let picture = unsafe { IOHelper::dumpPicture(&shape, &pic_data, &mut *text_document) };
                self.mPicture = Some(Arc::new(Mutex::new(picture)));
                self.mTarget = None;
            }
        }
    }

    fn dump_blip_fill(&self, pic_data: &io_wps_io::io::xwpf::usermodel::XWPFPictureData::XWPFPictureData) -> BlipFill {
        let shape = self.mCtxt.borrow().mShape.clone().expect("mShape");
        let text_document = text_document_ptr(&self.mCtxt.borrow());
        unsafe { IOHelper::dumpPicture4BlipFill(&shape, pic_data, &mut *text_document, 2) }
    }

    // Java: new CoBlipFill(mShape.getShapeID(), mDoc.getType(), 0)
    fn collect_xml07_blip(&mut self, r_id: &str) {
        let (shape_id, doc_type) = {
            let ctxt = self.mCtxt.borrow();
            let shape_id = ctxt.mShape.as_ref().expect("mShape").lock().expect("shape").getShapeID();
            let doc = ctxt.mDoc.as_ref().expect("mDoc");
            let doc = doc.borrow();
            (shape_id, IDocument::getType(doc.as_ref()))
        };
        let mut info = ClRefInfo::default();
        info.mRid = Some(r_id.to_string());
        let rid = info.mRid.clone();
        // Java: this.xml07Context().collectRid2CoreObject(info, new CoBlipFill(...))
        self.super_.super_.super_.xml07Context_mut().expect("xml07Context").collectRid2CoreObject(rid, shape_id, doc_type, 0);
    }
}

fn text_document_ptr(ctxt: &DmlShapeCtxt) -> *mut TextDocument {
    let doc = ctxt.mDoc.as_ref().expect("mDoc");
    let doc = doc.borrow();
    if let Some(sub_document) = doc.asAny().downcast_ref::<SubDocument>() {
        return SubDocument::getTextDocument(sub_document);
    }
    let text_document = doc.asAny().downcast_ref::<TextDocument>().expect("TextDocument");
    text_document as *const TextDocument as *mut TextDocument
}
