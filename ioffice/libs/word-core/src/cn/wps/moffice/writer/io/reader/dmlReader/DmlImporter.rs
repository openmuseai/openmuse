// Java: cn.wps.moffice.writer.io.reader.dmlReader.DmlImporter
#![allow(non_snake_case)]

use std::{any::Any, cell::RefCell, rc::{Rc, Weak}, sync::{Arc, Mutex}};
use drawing::cn::wps::moffice::drawing::IDocument::IDocument;
use drawing::cn::wps::moffice::drawing::Shape::Shape;
use drawing::cn::wps::moffice::drawing::ShapePos::ShapePos;
use io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;
use io_wps_io::cn::wps::io::opt::AttributesImpl::AttributesImpl;
use io_wps_io::cn::wps::io::xwpf::{
    IDmlCommonImporter::IDmlCommonImporter, IDmlImporter::IDmlImporter,
    IDocumentImporter::IDocumentImporter, IImporterBase::IImporterBase, SaxAttributes::SaxAttributes,
};
use util::moffice::property::MutablePropertySet::MutablePropertySet;
use crate::cn::wps::moffice::writer::core::{
    Document::Document, SubDocument::SubDocument,
};
use crate::cn::wps::moffice::writer::io::reader::{
    docxReader::helper::ShapeIDTool::ShapeIDTool,
    helper::IOHelper::IOHelper,
    xml07::Xml07Context::Xml07Context,
};
use super::{
    DmlCommonImporter::{self, DmlCommonImporter as DmlCommon},
    DmlGroupShapeImporter::DmlGroupShapeImporter,
    DmlImporterBase::{forget_dml_mode_key, publish_dml_mode},
    DmlShapeImporter::DmlShapeImporter,
    PositioningContext::PositioningContext,
    ThemeImporter::ThemeImporter,
    context::{DmlSubDocCtxt::DmlSubDocCtxt, ThemeContext::ThemeContext},
    helper::{EffectRefHelper::EffectRefHelper, FillRefHelper::FillRefHelper, LnRefHelper::LnRefHelper},
};

// Java passes DocumentImporter while the SAX bridge owns it. The pointer is the
// same object `getDocumentImporter()` returns for the life of the parse.
struct SubDocumentAlias(*mut SubDocument);
impl IDocument for SubDocumentAlias {
    fn getType(&self) -> i32 { unsafe { (*self.0).getType() } }
}
impl Document for SubDocumentAlias {
    fn getLength(&self) -> i32 { unsafe { (*self.0).getLength() } }
    fn asAny(&self) -> &dyn Any { unsafe { &*self.0 } }
    fn asAny_mut(&mut self) -> &mut dyn Any { unsafe { &mut *self.0 } }
}

/// Java `Stack<DmlShapeImporter>`. Group importers are the same object with `setIsGroup(true)`.
pub struct ShapeSlot {
    pub importer: RefCell<DmlShapeImporter<ShapeSlot, DmlImporter>>,
    pub is_group: bool,
}

// Java: public class DmlImporter extends DmlCommonImporter implements IDmlImporter
pub struct DmlImporter {
    super_: DmlCommon,
    mDocument: *mut SubDocument,
    mDocumentImporter: *const dyn IDocumentImporter,
    // Java mDocumentImporter identity, weak to avoid the bridge/importer cycle.
    mDocumentImporterWeak: Option<Weak<dyn IDocumentImporter>>,
    mRPrSet: Option<io_wps_io::cn::wps::io::xwpf::IDmlImporter::DrawingInfoRef>,
    mShapes: Vec<Arc<Mutex<Shape>>>,
    mShapesImporter: Vec<Rc<RefCell<ShapeSlot>>>,
    mThemeImporter: ThemeImporter,
    mThemeCtxt: Rc<RefCell<ThemeContext>>,
    mSubDocCtxt: Rc<RefCell<DmlSubDocCtxt<ShapeSlot>>>,
    self_weak: Weak<RefCell<DmlImporter>>,
    mode_key: usize,
}

macro_rules! paint_fill {
    ($target:expr, $elementId:expr, $attrs:expr) => {
        match $elementId {
            -1949425841 => $target.createSrgbClr($attrs),
            -1091642274 => $target.setLumMod($attrs),
            -1091640629 => $target.setLumOff($attrs),
            -934911823 => $target.setRedMod($attrs),
            -934910178 => $target.setRedOff($attrs),
            -909500100 => $target.setSatMod($attrs),
            -909498455 => $target.setSatOff($attrs),
            -887375076 => $target.createSysClr($attrs),
            -409889940 => $target.createScrgbClr($attrs),
            -305351098 => $target.createPrstClr($attrs),
            -152914620 => $target.createSchemeClr($attrs),
            -15865784 => $target.setBlueMod($attrs),
            -15864139 => $target.setBlueOff($attrs),
            107524 => $target.setLum($attrs),
            112785 => $target.setRed($attrs),
            113638 => $target.setSat($attrs),
            3027034 => $target.setBlue($attrs),
            3560187 => $target.setTint($attrs),
            92909918 => $target.setAlpha($attrs),
            98619139 => $target.setGreen($attrs),
            109399597 => $target.setShade($attrs),
            205217023 => $target.setGreenMod($attrs),
            205218668 => $target.setGreenOff($attrs),
            _ => {}
        }
    };
}
macro_rules! paint_shadow_end {
    ($target:expr, $elementId:expr) => {
        match $elementId {
            -1949425841 => $target.setSrgbClr(),
            -887375076 => $target.setSysClr(),
            -409889940 => $target.setScrgbClr(),
            -305351098 => $target.setPrstClr(),
            -152914620 => $target.setSchemeClr(),
            _ => {}
        }
    };
}

fn sax_of(attrs: &dyn Any) -> &dyn SaxAttributes {
    attrs.downcast_ref::<AttributesImpl>().expect("DML importer requires SAX AttributesImpl")
}

impl IImporterBase for DmlImporter {
    fn mode(&self) -> i32 { self.super_.mode() }
}

impl DmlImporter {
    // Java: public DmlImporter(IDocumentImporter, Shapes2Core, Document, ThemeContext, Xml07Context, int)
    pub fn create(
        document: *mut SubDocument,
        document_importer: *const dyn IDocumentImporter,
        document_importer_ref: Option<Rc<dyn IDocumentImporter>>,
        theme: Rc<RefCell<ThemeContext>>,
        xml07: Option<Xml07Context>,
        mode: i32,
    ) -> Rc<RefCell<Self>> {
        let sub = Rc::new(RefCell::new(DmlSubDocCtxt::<ShapeSlot>::new()));
        let this = Rc::new(RefCell::new(Self {
            super_: DmlCommon::new(xml07, mode),
            mDocument: document,
            mDocumentImporter: document_importer,
            mDocumentImporterWeak: document_importer_ref.as_ref().map(Rc::downgrade),
            mRPrSet: None,
            mShapes: Vec::new(),
            mShapesImporter: Vec::new(),
            mThemeImporter: ThemeImporter::new(theme.clone()),
            mThemeCtxt: theme,
            mSubDocCtxt: sub,
            self_weak: Weak::new(),
            mode_key: 0,
        }));
        let key = Rc::as_ptr(&this) as usize;
        this.borrow_mut().self_weak = Rc::downgrade(&this);
        this.borrow_mut().mode_key = key;
        publish_dml_mode(&this, mode);
        this
    }

    fn self_rc(&self) -> Rc<RefCell<DmlImporter>> {
        self.self_weak.upgrade().expect("DmlImporter")
    }

    fn doc_handle(&self) -> Rc<RefCell<Box<dyn Document>>> {
        Rc::new(RefCell::new(Box::new(SubDocumentAlias(self.mDocument))))
    }

    fn top(&self) -> Rc<RefCell<ShapeSlot>> {
        self.mShapesImporter.last().expect("mShapesImporter").clone()
    }

    fn with_shape<R>(&mut self, f: impl FnOnce(&mut DmlShapeImporter<ShapeSlot, DmlImporter>) -> R) -> R {
        let node = self.top();
        let mut slot = node.borrow_mut();
        f(slot.importer.get_mut())
    }

    fn positioning(&self) -> Rc<RefCell<PositioningContext>> {
        self.mSubDocCtxt.borrow().mPositioningCtxt.clone().expect("mPositioningCtxt")
    }
}

impl Drop for DmlImporter {
    fn drop(&mut self) { forget_dml_mode_key(self.mode_key); }
}

impl IDmlCommonImporter for DmlImporter {
    fn startGlow(&mut self, elementId: i32, attrs: &dyn Any) {
        let attrs = sax_of(attrs);
        self.with_shape(|sp| DmlCommonImporter::startGlow(&mut sp.mGlowImporter, elementId, attrs));
    }
    fn endGlow(&mut self, elementId: i32) {
        // Java: case 3175821 importGlow; default super.endGlow
        if elementId == 3175821 {
            self.with_shape(|sp| sp.importGlow());
        } else {
            self.with_shape(|sp| DmlCommonImporter::endGlow(&mut sp.mGlowImporter, elementId));
        }
    }
    fn startReflection(&mut self, attributes: &dyn Any) {
        let attributes = sax_of(attributes);
        self.with_shape(|sp| sp.importReflection(attributes));
    }
    fn startGradFill(&mut self, belonger: i32, elementId: i32, attrs: &dyn Any) {
        let attrs = sax_of(attrs);
        self.with_shape(|sp| {
            sp.importRotWithShape(attrs);
            let grad = match belonger {
                3458 => Some(&mut sp.mLineImporter.mGradFillImporter),
                3536191 | 962562815 => Some(&mut sp.mGradFillImporter),
                _ => None,
            };
            if let Some(grad) = grad {
                if matches!(belonger, 3458 | 3536191 | 962562815) {
                    grad.createGsLstContext();
                }
                let _ = elementId;
            }
        });
    }
    fn endGradFill(&mut self, belonger: i32) {
        self.with_shape(|sp| match belonger {
            3458 => {
                sp.mLineImporter.mGradFillImporter.importGradStopLst();
                sp.mLineImporter.importGradFill();
            }
            3536191 | 962562815 => {
                sp.mGradFillImporter.importGradStopLst();
                sp.importGradFill();
            }
            _ => {}
        });
    }
    fn startGrpFill(&mut self, belonger: i32, _elementId: i32, _attrs: &dyn Any) {
        if belonger == 3536191 { self.with_shape(|sp| sp.importGrpFill()); }
    }
    fn endGrpFill(&mut self, _belonger: i32) {}
    fn startSolidFill(&mut self, belonger: i32, elementId: i32, attrs: &dyn Any) {
        let attrs = sax_of(attrs);
        self.with_shape(|sp| {
            if belonger == 3458 {
                DmlCommonImporter::startSolidFill(&mut sp.mLineImporter.mSolidFillImporter, elementId, attrs);
            } else if belonger == 3536191 || belonger == 962562815 {
                DmlCommonImporter::startSolidFill(&mut sp.mSolidFillImporter, elementId, attrs);
            }
        });
    }
    fn endSolidFill(&mut self, belonger: i32, elementId: i32) {
        if elementId == -1353661746 {
            self.importSolidFill(belonger);
        } else {
            self.with_shape(|sp| {
                if belonger == 3458 {
                    DmlCommonImporter::endSolidFill(&mut sp.mLineImporter.mSolidFillImporter, elementId);
                } else if belonger == 3536191 || belonger == 962562815 {
                    DmlCommonImporter::endSolidFill(&mut sp.mSolidFillImporter, elementId);
                }
            });
        }
    }
    fn startGs(&mut self, belonger: i32, elementId: i32, attrs: &dyn Any) {
        let attrs = sax_of(attrs);
        self.with_shape(|sp| {
            let grad = match belonger {
                3458 => Some(&mut sp.mLineImporter.mGradFillImporter),
                3536191 | 962562815 => Some(&mut sp.mGradFillImporter),
                _ => None,
            };
            if let Some(grad) = grad { DmlCommonImporter::startGs(grad, elementId, attrs); }
        });
    }
    fn endGs(&mut self, belonger: i32, elementId: i32) {
        self.with_shape(|sp| {
            let grad = match belonger {
                3458 => Some(&mut sp.mLineImporter.mGradFillImporter),
                3536191 | 962562815 => Some(&mut sp.mGradFillImporter),
                _ => None,
            };
            if let Some(grad) = grad { DmlCommonImporter::endGs(grad, elementId); }
        });
    }
    fn startLin(&mut self, belonger: i32, elementId: i32, attrs: &dyn Any) {
        let attrs = sax_of(attrs);
        self.with_shape(|sp| {
            if let Some(grad) = grad_of(sp, belonger) {
                DmlCommonImporter::startLin(grad, elementId, attrs);
            }
        });
    }
    fn endLin(&mut self, _belonger: i32) {}
    fn startPath(&mut self, belonger: i32, elementId: i32, attrs: &dyn Any) {
        let attrs = sax_of(attrs);
        self.with_shape(|sp| {
            if let Some(grad) = grad_of(sp, belonger) {
                DmlCommonImporter::startPath(grad, elementId, attrs);
            }
        });
    }
    fn endPath(&mut self, _belonger: i32) {}
    fn startTileRect(&mut self, belonger: i32, elementId: i32, attrs: &dyn Any) {
        let attrs = sax_of(attrs);
        self.with_shape(|sp| {
            if let Some(grad) = grad_of(sp, belonger) {
                DmlCommonImporter::startTileRect(grad, elementId, attrs);
            }
        });
    }
    fn endTileRect(&mut self, _belonger: i32) {}
    fn startLn(&mut self, elementId: i32, attrs: &dyn Any) {
        let attrs = sax_of(attrs);
        self.with_shape(|sp| match elementId {
            -1546470805 => sp.mLineImporter.setTailEnd(attrs),
            3458 => sp.mLineImporter.setLn(attrs),
            795277019 => sp.mLineImporter.setHeadEnd(attrs),
            _ => DmlCommonImporter::startLn(&mut sp.mLineImporter, elementId, attrs),
        });
    }
    fn endLn(&mut self, elementId: i32) {
        if elementId == 3458 {
            self.with_shape(|sp| sp.importLineProperty());
        }
    }
    fn startNoFill(&mut self, _belonger: i32) {}
}

fn grad_of<'a, S, D: IImporterBase>(
    sp: &'a mut DmlShapeImporter<S, D>,
    belonger: i32,
) -> Option<&'a mut super::GradFillImporter::GradFillImporter<S, D>> {
    match belonger {
        3458 => Some(&mut sp.mLineImporter.mGradFillImporter),
        3536191 | 962562815 => Some(&mut sp.mGradFillImporter),
        _ => None,
    }
}

impl IDmlImporter for DmlImporter {
    fn getDocumentImporter(&self) -> &dyn IDocumentImporter {
        unsafe { &*self.mDocumentImporter }
    }
    fn getDocumentImporterRef(&self) -> Option<Rc<dyn IDocumentImporter>> {
        self.mDocumentImporterWeak.as_ref()?.upgrade()
    }
    fn setDrawingInfo(&mut self, rPrSet: io_wps_io::cn::wps::io::xwpf::IDmlImporter::DrawingInfoRef) {
        self.mRPrSet = Some(rPrSet);
    }
    fn finish(&mut self) {
        // Java: mShape2Core.append(mShapes, mRPrSet); mShapes.clear(); mRPrSet = null;
        // Inline pictures take Shapes2Core.appendShape2CoreWithoutField:
        // control char '\u0001' (not in dgg) or '\b', then addTopLevelShape.
        let shapes = std::mem::take(&mut self.mShapes);
        let mut props = self.mRPrSet.as_ref()
            .map(|p| p.borrow().clone_mps())
            .unwrap_or_else(MutablePropertySet::new);
        self.mRPrSet = None;
        let doc = unsafe { &mut *self.mDocument };
        for shape in shapes {
            let in_dgg = shape.lock().expect("shape").isInDggInfo();
            let ch = if in_dgg { '\u{0008}' } else { '\u{0001}' };
            let cp = IOHelper::appendControlChar(doc, ch, &mut props);
            IOHelper::addTopLevelShape(doc, cp, shape);
        }
    }
    fn startAnchorChild(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        let pos = self.positioning();
        let mut pos = pos.borrow_mut();
        match elementId {
            -2124393662 => pos.setSimplePos(attrs),
            -2079867230 | -1447414265 | -1292678373 | -46018828 | 729290359 => pos.setWrapInfo(elementId, attrs),
            -1289044182 => pos.setExtent(attrs),
            -1102672497 | 109757538 => pos.setWrapcoords(elementId, attrs),
            -781241317 => pos.setEffectExtent(attrs),
            -352578160 => pos.setWrapPolygon(elementId, attrs),
            95756634 => pos.setDocPr(attrs),
            _ => {}
        }
    }
    fn startAnchor(&mut self, _elementId: i32, attrs: &dyn SaxAttributes) {
        let sub = self.mSubDocCtxt.clone();
        let mut sub = sub.borrow_mut();
        if sub.mPositioningCtxt.is_none() {
            sub.mPositioningCtxt = Some(Rc::new(RefCell::new(PositioningContext::new())));
        }
        let pos = sub.mPositioningCtxt.clone().expect("pos");
        drop(sub);
        let mut pos = pos.borrow_mut();
        pos.setAnchorAttrs(attrs);
        pos.mIsInDggInfo = true;
        pos.mInLine = false;
    }
    fn endAnchor(&mut self) {
        let mut sub = self.mSubDocCtxt.borrow_mut();
        sub.mPositioningCtxt = None;
        sub.mLastDmlShapeImporter = None;
    }
    fn startInlineChild(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        let pos = self.positioning();
        let mut pos = pos.borrow_mut();
        match elementId {
            -1289044182 => pos.setExtent(attrs),
            -781241317 => pos.setEffectExtent(attrs),
            95756634 => pos.setDocPr(attrs),
            _ => {}
        }
    }
    fn startInline(&mut self, _elementId: i32, attributes: &dyn SaxAttributes) {
        let sub = self.mSubDocCtxt.clone();
        let mut sub = sub.borrow_mut();
        if sub.mPositioningCtxt.is_none() {
            sub.mPositioningCtxt = Some(Rc::new(RefCell::new(PositioningContext::new())));
        }
        let pos = sub.mPositioningCtxt.clone().expect("pos");
        drop(sub);
        let mut pos = pos.borrow_mut();
        pos.setInlineAttrs(attributes);
        pos.mInLine = true;
        pos.mIsInDggInfo = false;
    }
    fn endInline(&mut self) {
        let mut sub = self.mSubDocCtxt.borrow_mut();
        sub.mPositioningCtxt = None;
        sub.mLastDmlShapeImporter = None;
    }
    fn startSingleShape(&mut self, isGroup: bool) {
        let doc = unsafe { &mut *self.mDocument };
        let shape = {
            let container = doc.getDrawingContainer().expect("drawingContainer");
            IOHelper::createShape(container, isGroup)
        };
        let mut shape = shape;
        let id = {
            let container = doc.getDrawingContainer();
            ShapeIDTool::getNewID__idrawingcontainer(container).unwrap_or(0)
        };
        shape.setShapeID(id);
        shape.setIsFromDml(true);
        let shape = shape.into_shared();
        let has_parent = !self.mShapesImporter.is_empty();
        let positioning = self.mSubDocCtxt.borrow().mPositioningCtxt.clone();
        let importer = DmlShapeImporter::new(
            shape,
            self.doc_handle(),
            self.mSubDocCtxt.clone(),
            self.mThemeCtxt.clone(),
            self.super_.xml07Context(),
            self.self_rc(),
        );
        let slot = Rc::new(RefCell::new(ShapeSlot { importer: RefCell::new(importer), is_group: isGroup }));
        {
            let mut slot_ref = slot.borrow_mut();
            let sp = slot_ref.importer.get_mut();
            if let Some(positioning) = positioning {
                sp.setPositioningContext(positioning);
            }
            sp.mGRFImporter.setIsChild(has_parent);
            if isGroup { sp.mGRFImporter.setIsGroup(true); }
            sp.importGRF();
            sp.importEffectExtent();
            sp.importShapeName();
            sp.importShapeDescr();
        }
        self.mSubDocCtxt.borrow_mut().mPositioningCtxt = None;
        self.mShapesImporter.push(slot);
    }
    fn endSingleShape(&mut self, _isGroup: bool) {
        let node = self.mShapesImporter.pop().expect("mShapesImporter");
        let shape = {
            let mut slot = node.borrow_mut();
            let sp = slot.importer.get_mut();
            sp.excute();
            sp.mTarget.clone()
        };
        self.mSubDocCtxt.borrow_mut().mLastDmlShapeImporter = Some(node.clone());
        if !self.mShapesImporter.is_empty() {
            let parent = self.mShapesImporter.last().expect("parent").clone();
            let parent_shape = parent.borrow().importer.borrow().mTarget.clone();
            let count = parent_shape.lock().expect("parent").getChildCount();
            shape.lock().expect("shape").setZIndex(count);
            parent_shape.lock().expect("parent").addChildShape(shape.clone());
        } else if shape.lock().expect("shape").getShapePos().is_some() {
            self.mShapes.push(shape);
        }
    }
    fn startSppr(&mut self, elementId: i32, _attrs: &dyn SaxAttributes) {
        if elementId == -1041060124 { self.with_shape(|sp| sp.setNoFill()); }
    }
    fn endSppr(&mut self, _content: &str) {}
    fn startBlipFill(&mut self, elementId: i32, attrs: &dyn SaxAttributes, part: &POIXMLDocumentPart, belonger: i32) {
        self.with_shape(|sp| {
            sp.importRotWithShape(attrs);
            let blip = if belonger == 110986 { &mut sp.mPictureImporter.super_ } else { &mut sp.mBlipFillImporter };
            match elementId {
                -1953564184 => blip.importSrcRect(attrs),
                -1881872635 => blip.setFillType(3),
                -825444332 => blip.setRotateWithShape(attrs),
                -729134585 => blip.setFillRect(attrs),
                -139578499 => blip.importBiLevelThresh(attrs),
                107524 => blip.importLum(attrs),
                3026673 => blip.importBlip(attrs, part),
                3560110 => blip.setTile(attrs),
                280621785 => blip.importGrayscl(),
                536515761 => blip.importTransparency(attrs),
                _ => {}
            }
        });
    }
    fn endBlipFill(&mut self, elementId: i32, belonger: i32) {
        if elementId == -825444332 {
            self.with_shape(|sp| {
                if belonger == 110986 { sp.importPicture(); } else { sp.importBlipFill(); }
            });
        } else if elementId == 2011742320 {
            self.with_shape(|sp| {
                let blip = if belonger == 110986 { &mut sp.mPictureImporter.super_ } else { &mut sp.mBlipFillImporter };
                blip.importDuotoneColor();
            });
        }
    }
    fn startEffectLst(&mut self, _elementId: i32, _attrs: &dyn SaxAttributes) {}
    fn endEffectLst(&mut self, _content: &str) {}
    fn startPositionH(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        if elementId == 1707117631 { self.positioning().borrow_mut().setPositionRelH(attrs); }
    }
    fn endPositionH(&mut self, elementId: i32, content: Option<&str>) {
        let pos = self.positioning();
        let mut pos = pos.borrow_mut();
        match elementId {
            -2100663832 => pos.setPctPositionH(content.unwrap_or("")),
            92903173 => pos.setPositionAlignH(content),
            949744647 => pos.setPositionH(content.unwrap_or("")),
            _ => {}
        }
    }
    fn startPositionV(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        if elementId == 1707117645 { self.positioning().borrow_mut().setPositionRelV(attrs); }
    }
    fn endPositionV(&mut self, elementId: i32, content: Option<&str>) {
        let pos = self.positioning();
        let mut pos = pos.borrow_mut();
        match elementId {
            92903173 => pos.setPositionAlignV(content),
            949744647 => pos.setPositionV(content.unwrap_or("")),
            1734453110 => pos.setPctPositionV(content.unwrap_or("")),
            _ => {}
        }
    }
    fn startGrpSppr(&mut self, _elementId: i32, _attrs: &dyn SaxAttributes) {}
    fn endGrpSppr(&mut self) {}
    fn startXfrm(&mut self, elementId: i32, attrs: &dyn SaxAttributes, _isGroup: bool) {
        self.with_shape(|sp| match elementId {
            100897 => sp.setExt(attrs),
            109935 => sp.setOff(attrs),
            3676585 => {
                sp.mShpPosImporter.importRotation(attrs);
                sp.importFlipVH();
            }
            94596988 => sp.setChExt(attrs),
            94606026 => sp.setChOff(attrs),
            _ => {}
        });
    }
    fn endXfrm(&mut self, elementId: i32, isGroup: bool) {
        if elementId == 3676585 {
            self.with_shape(|sp| {
                sp.importShapePos();
                if isGroup { DmlGroupShapeImporter::importCoordBounds(sp); }
            });
        }
    }
    fn startLnRef(&mut self, elementId: i32, attrs: &dyn SaxAttributes) { self.ref_color(elementId, attrs, RefKind::Ln); }
    fn endLnRef(&mut self, elementId: i32) { self.ref_color_end(elementId, RefKind::Ln); }
    fn startFillRef(&mut self, elementId: i32, attrs: &dyn SaxAttributes) { self.ref_color(elementId, attrs, RefKind::Fill); }
    fn endFillRef(&mut self, elementId: i32) { self.ref_color_end(elementId, RefKind::Fill); }
    fn startEffectRef(&mut self, elementId: i32, attrs: &dyn SaxAttributes) { self.ref_color(elementId, attrs, RefKind::Effect); }
    fn endEffectRef(&mut self, elementId: i32) { self.ref_color_end(elementId, RefKind::Effect); }
    fn startFillToRect(&mut self, belonger: i32, _elementId: i32, attributes: &dyn SaxAttributes) {
        self.with_shape(|sp| {
            if let Some(grad) = grad_of(sp, belonger) { grad.setFillToRect(attributes); }
        });
    }
    fn endFillToRect(&mut self, _belonger: i32) {}
    fn startMoveTo(&mut self, elementId: i32, attributes: &dyn SaxAttributes) {
        self.with_shape(|sp| match elementId {
            -1068263892 => sp.mGeoImporter.addSegment(64),
            3588 => sp.mGeoImporter.addVertice(attributes),
            _ => {}
        });
    }
    fn endMoveTo(&mut self, _elementId: i32) {}
    fn startCubicBezto(&mut self, elementId: i32, attributes: &dyn SaxAttributes) {
        self.with_shape(|sp| match elementId {
            3588 => sp.mGeoImporter.addVertice(attributes),
            1594918984 => sp.mGeoImporter.addSegment(32),
            _ => {}
        });
    }
    fn endCubicBezto(&mut self, _elementId: i32) {}
    fn startLineTo(&mut self, elementId: i32, attributes: &dyn SaxAttributes) {
        self.with_shape(|sp| match elementId {
            3588 => sp.mGeoImporter.addVertice(attributes),
            3325853 => sp.mGeoImporter.addSegment(0),
            _ => {}
        });
    }
    fn endLineTo(&mut self, _elementId: i32) {}
    fn starArcTo(&mut self, elementId: i32, attributes: &dyn SaxAttributes) {
        self.with_shape(|sp| if elementId == 93075565 {
            sp.mGeoImporter.addSegment(163);
            sp.mGeoImporter.addArcToVertices(attributes);
        });
    }
    fn endArcTo(&mut self, _elementId: i32) {}
    fn starQuadBezTo(&mut self, elementId: i32, attributes: &dyn SaxAttributes) {
        self.with_shape(|sp| match elementId {
            -1513004949 => sp.mGeoImporter.addSegment(169),
            3588 => sp.mGeoImporter.addVertice(attributes),
            _ => {}
        });
    }
    fn endQuadBezTo(&mut self, _elementId: i32) {}
    fn startPathLst(&mut self) { self.with_shape(|sp| sp.mGeoImporter.createPathContext()); }
    fn endPathLst(&mut self) { self.with_shape(|sp| sp.mGeoImporter.importPath()); }
    fn startPath2(&mut self, attrs: &dyn SaxAttributes) {
        self.with_shape(|sp| {
            sp.mGeoImporter.startSinglePath(attrs);
            sp.mGeoImporter.importGeoCordSize(attrs);
        });
    }
    fn endPath2(&mut self) { self.with_shape(|sp| sp.mGeoImporter.endSinglePath()); }
    fn endCustgeom(&mut self) {
        self.with_shape(|sp| {
            sp.mGeoImporter.importConnectionSites4DOCX();
            sp.mGeoImporter.importGeoAdjust();
            sp.importGeometry();
        });
    }
    fn startClose(&mut self) { self.with_shape(|sp| sp.mGeoImporter.closePath()); }
    fn startPattFill(&mut self, belonger: i32, _elementId: i32, attrs: &dyn SaxAttributes) {
        self.with_shape(|sp| {
            if let Some(patt) = pattern_of(sp, belonger) { patt.importPatternFill(attrs); }
        });
    }
    fn endPattFill(&mut self, belonger: i32, _elementId: i32) {
        self.with_shape(|sp| match belonger {
            3458 => sp.mLineImporter.importPatternFill(),
            3536191 => sp.importPatternFill(),
            _ => {}
        });
    }
    fn startFgClr(&mut self, _belonger: i32, elementId: i32, attrs: &dyn SaxAttributes) {
        self.with_shape(|sp| { paint_fill!(sp.mPatternFillImporter, elementId, attrs); });
    }
    fn endFgClr(&mut self, belonger: i32, elementId: i32) {
        self.with_shape(|sp| {
            if let Some(patt) = pattern_of(sp, belonger) { paint_pattern_end(patt, elementId, false); }
        });
    }
    fn startBgClr(&mut self, _belonger: i32, elementId: i32, attrs: &dyn SaxAttributes) {
        self.with_shape(|sp| { paint_fill!(sp.mPatternFillImporter, elementId, attrs); });
    }
    fn endBgClr(&mut self, belonger: i32, elementId: i32) {
        self.with_shape(|sp| {
            if let Some(patt) = pattern_of(sp, belonger) { paint_pattern_end(patt, elementId, true); }
        });
    }
    // Java: DmlImporter.startTxbx(int elementId, Attributes attrs)
    fn startTxbx(&mut self, _elementId: i32, attrs: &dyn SaxAttributes) {
        // Java: if (null == this.mSubDocCtxt.mTxbxChains) {
        // Java:   this.mSubDocCtxt.mTxbxChains = new TxbxChains();
        // Java: }
        let mut sub = self.mSubDocCtxt.borrow_mut();
        if sub.mTxbxChains.is_none() {
            sub.mTxbxChains = Some(Rc::new(RefCell::new(super::context::TxbxChains::TxbxChains::new())));
        }
        drop(sub);
        // Java: ((DmlShapeImporter)this.mShapesImporter.peek()).mTextboxImporter.startTextbox(attrs);
        self.with_shape(|sp| sp.mTextboxImporter.startTextbox(attrs));
    }
    fn endTxbx(&mut self, _elementId: i32) { self.with_shape(|sp| sp.mTextboxImporter.endTextbox()); }
    fn startContentPart(&mut self, _attrs: &dyn SaxAttributes, _part: &POIXMLDocumentPart) {}
    fn startRelIds(&mut self, _attrs: &dyn SaxAttributes, _part: &POIXMLDocumentPart, _belonger: i32) {}
    fn startChart(&mut self, _attrs: &dyn SaxAttributes, _part: &POIXMLDocumentPart, _belonger: i32) {}
    fn startPic(&mut self, _elementId: i32) { self.with_shape(|sp| sp.setShapeType_int(204)); }
    fn startLinkedTxbx(&mut self, attrs: &dyn SaxAttributes) { self.with_shape(|sp| sp.mTextboxImporter.setLinkedTxbx(attrs)); }
    fn startPrstgeom(&mut self, attrs: &dyn SaxAttributes) { self.with_shape(|sp| sp.setShapeType(attrs)); }
    fn endPrstgeom(&mut self) {
        self.with_shape(|sp| {
            sp.mGeoImporter.importConnectionSites4DOCX();
            sp.mGeoImporter.importGeoAdjust();
            sp.importGeometry();
        });
    }
    fn startAvlst(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        self.with_shape(|sp| match elementId {
            3293 => sp.addGd(93173592, attrs),
            93173592 => sp.startGdlst(),
            _ => {}
        });
    }
    fn startGdlst(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        self.with_shape(|sp| match elementId {
            3293 => sp.addGd(98178480, attrs),
            98178480 => sp.startGdlst(),
            _ => {}
        });
    }
    fn startCxnLst(&mut self, _elementId: i32, _attributes: &dyn SaxAttributes) { self.with_shape(|sp| sp.startCxnlst()); }
    fn startCxn(&mut self, _elementId: i32, attrs: &dyn SaxAttributes) { self.with_shape(|sp| sp.startCxn(attrs)); }
    fn endCxn(&mut self) { self.with_shape(|sp| sp.endCxn()); }
    fn startPos(&mut self, _elementId: i32, attributes: &dyn SaxAttributes) { self.with_shape(|sp| sp.addPos(attributes)); }
    fn startSizeRelH(&mut self, elementId: i32, attributes: &dyn SaxAttributes) {
        let last = self.mSubDocCtxt.borrow().mLastDmlShapeImporter.clone();
        if elementId == 846906992 {
            if let Some(last) = last {
                last.borrow_mut().importer.borrow_mut().mShpPosImporter.setSizeRelH(attributes);
            }
        }
    }
    fn endSizeRelH(&mut self, elementId: i32, content: Option<&str>) {
        let last = self.mSubDocCtxt.borrow().mLastDmlShapeImporter.clone();
        if let Some(last) = last {
            let mut slot = last.borrow_mut();
            let sp = slot.importer.get_mut();
            match elementId {
                -1300631195 => sp.mShpPosImporter.importSizeHPercent(content),
                846906992 => sp.mShpPosImporter.importSizeRelH(),
                _ => {}
            }
        }
    }
    fn startSizeRelV(&mut self, elementId: i32, attributes: &dyn SaxAttributes) {
        let last = self.mSubDocCtxt.borrow().mLastDmlShapeImporter.clone();
        if elementId == 846906992 {
            if let Some(last) = last {
                last.borrow_mut().importer.borrow_mut().mShpPosImporter.setSizeRelV(attributes);
            }
        }
    }
    fn endSizeRelV(&mut self, elementId: i32, content: Option<&str>) {
        let last = self.mSubDocCtxt.borrow().mLastDmlShapeImporter.clone();
        if let Some(last) = last {
            let mut slot = last.borrow_mut();
            let sp = slot.importer.get_mut();
            match elementId {
                1734453110 | -1300631195 => sp.mShpPosImporter.importSizeVPercent(content),
                846906992 => sp.mShpPosImporter.importSizeRelV(),
                _ => {}
            }
        }
    }
    fn startFill_Theme(&mut self, elementId: i32, attrs: &dyn SaxAttributes) { self.mThemeImporter.startFillStyle(elementId, attrs); }
    fn endFill_Theme(&mut self, elementId: i32) { self.mThemeImporter.endFillStyle(elementId); }
    fn startBgFill_Theme(&mut self, elementId: i32, attrs: &dyn SaxAttributes) { self.mThemeImporter.startBgFillStyle(elementId, attrs); }
    fn endBgFill_Theme(&mut self, elementId: i32) { self.mThemeImporter.endBgFillStyle(elementId); }
    fn startLn_Theme(&mut self, elementId: i32, attrs: &dyn SaxAttributes) { self.mThemeImporter.startLnStyle(elementId, attrs); }
    fn endLn_Theme(&mut self, elementId: i32) { self.mThemeImporter.endLnStyle(elementId); }
    fn startEffectStyle_Theme(&mut self, elementId: i32, attrs: &dyn SaxAttributes) { self.mThemeImporter.startEffectStyle(elementId, attrs); }
    fn endEffectStyle_Theme(&mut self, elementId: i32) { self.mThemeImporter.endEffectStyle(elementId); }
    fn startClrScheme(&mut self, elementId: i32, attrs: &dyn SaxAttributes) { self.mThemeImporter.startThemeClr(elementId, attrs); }
    fn endClrScheme(&mut self, elementId: i32) { self.mThemeImporter.endThemeClr(elementId); }
    fn startClrMap(&mut self, attrs: &dyn SaxAttributes) { self.mThemeImporter.startClrMap(attrs); }
    fn startInnerShdw(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        self.with_shape(|sp| {
            if elementId == -527990498 { sp.mInnerShdwImporter.createInnerShdw(attrs); }
            else { paint_fill!(sp.mInnerShdwImporter, elementId, attrs); }
        });
    }
    fn endInnerShdw(&mut self, elementId: i32) { self.with_shape(|sp| { paint_shadow_end!(sp.mInnerShdwImporter, elementId); }); }
    fn startOuterShdw(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        self.with_shape(|sp| {
            if elementId == 1857009859 { sp.mOuterShdwImporter.createOuterShdw(attrs); }
            else { paint_fill!(sp.mOuterShdwImporter, elementId, attrs); }
        });
    }
    fn endOuterShdw(&mut self, elementId: i32) { self.with_shape(|sp| { paint_shadow_end!(sp.mOuterShdwImporter, elementId); }); }
    fn startPrstShdw(&mut self, elementId: i32, attrs: &dyn SaxAttributes) {
        self.with_shape(|sp| {
            // Java case -875476949 falls through into createSysClr.
            if elementId == -875476949 {
                sp.mPrstShdwImporter.createPrstShdw(attrs);
                sp.mPrstShdwImporter.createSysClr(attrs);
            } else {
                paint_fill!(sp.mPrstShdwImporter, elementId, attrs);
            }
        });
    }
    fn endPrstShdw(&mut self, elementId: i32) { self.with_shape(|sp| { paint_shadow_end!(sp.mPrstShdwImporter, elementId); }); }
    fn buildTxbxChain(&mut self) {
        let chains = self.mSubDocCtxt.borrow_mut().mTxbxChains.take();
        if let Some(chains) = chains { chains.borrow_mut().build(); }
    }
    fn startSoftEdge(&mut self, attributes: &dyn SaxAttributes) { self.with_shape(|sp| sp.importSoftEdge(attributes)); }
    fn startWPC(&mut self) { self.startSingleShape(true); }
    fn onCanvasEnd(&mut self) {}
    fn startBodyPr(&mut self, _elementId: i32, attributes: &dyn SaxAttributes) {
        self.with_shape(|sp| sp.mTextboxImporter.importBodyPr(attributes));
    }
    fn startWSP(&mut self, _attributes: &dyn SaxAttributes) {}
    fn startDuotone(&mut self, elementId: i32, attrs: &dyn SaxAttributes, belonger: i32) {
        self.with_shape(|sp| {
            let duo = if belonger == 110986 { &mut sp.mPictureImporter.mDuotoneImporter } else { &mut sp.mBlipFillImporter.mDuotoneImporter };
            if elementId == 2011742320 { duo.createDuotoneContext(); }
            else { paint_fill!(duo, elementId, attrs); }
        });
    }
    fn endDuotone(&mut self, elementId: i32, belonger: i32) {
        self.with_shape(|sp| {
            let duo = if belonger == 110986 { &mut sp.mPictureImporter.mDuotoneImporter } else { &mut sp.mBlipFillImporter.mDuotoneImporter };
            paint_shadow_end!(duo, elementId);
        });
    }
    fn startCnvpr(&mut self, _elementId: i32, attributes: &dyn SaxAttributes) {
        self.with_shape(|sp| sp.importShapeName__Attributes(attributes));
    }
    fn startClrFrom(&mut self, elementId: i32, attrs: &dyn SaxAttributes, belonger: i32) {
        self.with_shape(|sp| {
            let clr = if belonger == 110986 { &mut sp.mPictureImporter.mClrFromImporter } else { &mut sp.mBlipFillImporter.mClrFromImporter };
            paint_fill!(clr, elementId, attrs);
        });
    }
    fn endClrFrom(&mut self, elementId: i32, belonger: i32) {
        self.with_shape(|sp| {
            let clr = if belonger == 110986 { &mut sp.mPictureImporter.mClrFromImporter } else { &mut sp.mBlipFillImporter.mClrFromImporter };
            paint_shadow_end!(clr, elementId);
        });
    }
    fn startPrstTxWarp(&mut self, _elementId: i32, attributes: &dyn SaxAttributes) {
        self.with_shape(|sp| sp.mPrstTextWarpImporter.importWarp(attributes));
    }
}

#[derive(Clone, Copy)]
enum RefKind { Ln, Fill, Effect }

impl DmlImporter {
    fn importSolidFill(&mut self, belonger: i32) {
        // Java: private void importSolidFill — applied by the shape/line importer excute path.
        self.with_shape(|sp| match belonger {
            3458 => sp.mLineImporter.importSolidFill(),
            3536191 | 962562815 => sp.importSolidFill(),
            _ => {}
        });
    }

    fn ref_color(&mut self, elementId: i32, attrs: &dyn SaxAttributes, kind: RefKind) {
        let ctxt = self.with_shape(|sp| sp.mCtxt.clone());
        let mut ctxt = ctxt.borrow_mut();
        match kind {
            RefKind::Ln => dispatch_ref(elementId, attrs, &mut ctxt, LnRefHelper::createSrgbClr, LnRefHelper::setLumMod, LnRefHelper::setLumOff, LnRefHelper::setRedMod, LnRefHelper::setRedOff, LnRefHelper::setSatMod, LnRefHelper::setSatOff, LnRefHelper::createSysClr, LnRefHelper::createScrgbClr, LnRefHelper::createPrstClr, LnRefHelper::createSchemeClr, LnRefHelper::setBlueMod, LnRefHelper::setBlueOff, LnRefHelper::setLum, LnRefHelper::setRed, LnRefHelper::setSat, LnRefHelper::setBlue, LnRefHelper::setTint, LnRefHelper::setAlpha, LnRefHelper::setGreen, LnRefHelper::setShade, LnRefHelper::setGreenMod, LnRefHelper::setGreenOff, 103099313, LnRefHelper::createLineRefContext),
            RefKind::Fill => dispatch_ref(elementId, attrs, &mut ctxt, FillRefHelper::createSrgbClr, FillRefHelper::setLumMod, FillRefHelper::setLumOff, FillRefHelper::setRedMod, FillRefHelper::setRedOff, FillRefHelper::setSatMod, FillRefHelper::setSatOff, FillRefHelper::createSysClr, FillRefHelper::createScrgbClr, FillRefHelper::createPrstClr, FillRefHelper::createSchemeClr, FillRefHelper::setBlueMod, FillRefHelper::setBlueOff, FillRefHelper::setLum, FillRefHelper::setRed, FillRefHelper::setSat, FillRefHelper::setBlue, FillRefHelper::setTint, FillRefHelper::setAlpha, FillRefHelper::setGreen, FillRefHelper::setShade, FillRefHelper::setGreenMod, FillRefHelper::setGreenOff, 1769218406, FillRefHelper::createFillRefContext),
            RefKind::Effect => dispatch_ref(elementId, attrs, &mut ctxt, EffectRefHelper::createSrgbClr, EffectRefHelper::setLumMod, EffectRefHelper::setLumOff, EffectRefHelper::setRedMod, EffectRefHelper::setRedOff, EffectRefHelper::setSatMod, EffectRefHelper::setSatOff, EffectRefHelper::createSysClr, EffectRefHelper::createScrgbClr, EffectRefHelper::createPrstClr, EffectRefHelper::createSchemeClr, EffectRefHelper::setBlueMod, EffectRefHelper::setBlueOff, EffectRefHelper::setLum, EffectRefHelper::setRed, EffectRefHelper::setSat, EffectRefHelper::setBlue, EffectRefHelper::setTint, EffectRefHelper::setAlpha, EffectRefHelper::setGreen, EffectRefHelper::setShade, EffectRefHelper::setGreenMod, EffectRefHelper::setGreenOff, -902281035, EffectRefHelper::createEffectRefContext),
        }
    }

    fn ref_color_end(&mut self, elementId: i32, kind: RefKind) {
        let ctxt_rc = self.with_shape(|sp| sp.mCtxt.clone());
        let theme = self.mThemeCtxt.clone();
        let mut ctxt = ctxt_rc.borrow_mut();
        let theme = theme.borrow();
        match elementId {
            -1949425841 => match kind {
                RefKind::Ln => LnRefHelper::setSrgbClr(&mut ctxt),
                RefKind::Fill => FillRefHelper::setSrgbClr(&mut ctxt),
                RefKind::Effect => EffectRefHelper::setSrgbClr(&mut ctxt),
            },
            -887375076 => match kind {
                RefKind::Ln => LnRefHelper::setSysClr(&mut ctxt),
                RefKind::Fill => FillRefHelper::setSysClr(&mut ctxt),
                RefKind::Effect => EffectRefHelper::setSysClr(&mut ctxt),
            },
            -409889940 => match kind {
                RefKind::Ln => LnRefHelper::setScrgbClr(&mut ctxt),
                RefKind::Fill => FillRefHelper::setScrgbClr(&mut ctxt),
                RefKind::Effect => EffectRefHelper::setScrgbClr(&mut ctxt),
            },
            -305351098 => match kind {
                RefKind::Ln => LnRefHelper::setPrstClr(&mut ctxt),
                RefKind::Fill => FillRefHelper::setPrstClr(&mut ctxt),
                RefKind::Effect => EffectRefHelper::setPrstClr(&mut ctxt),
            },
            -152914620 => match kind {
                RefKind::Ln => LnRefHelper::setSchemeClr(&mut ctxt),
                RefKind::Fill => FillRefHelper::setSchemeClr(&mut ctxt),
                RefKind::Effect => EffectRefHelper::setSchemeClr(&mut ctxt),
            },
            103099313 if matches!(kind, RefKind::Ln) => {
                drop(ctxt);
                drop(theme);
                let theme_rc = self.mThemeCtxt.clone();
                let mut ctxt = ctxt_rc.borrow_mut();
                let theme = theme_rc.borrow();
                LnRefHelper::importLinePropertyByLnRef(self, &mut ctxt, &theme);
            }
            _ => {}
        }
    }
}

fn dispatch_ref(
    elementId: i32,
    attrs: &dyn SaxAttributes,
    ctxt: &mut super::context::DmlShapeCtxt::DmlShapeCtxt,
    create_srgb: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_lum_mod: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_lum_off: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_red_mod: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_red_off: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_sat_mod: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_sat_off: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    create_sys: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    create_scrgb: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    create_prst: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    create_scheme: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_blue_mod: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_blue_off: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_lum: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_red: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_sat: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_blue: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_tint: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_alpha: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_green: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_shade: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_green_mod: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    set_green_off: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
    extra_id: i32,
    extra: fn(&mut super::context::DmlShapeCtxt::DmlShapeCtxt, &dyn SaxAttributes),
) {
    match elementId {
        -1949425841 => create_srgb(ctxt, attrs),
        -1091642274 => set_lum_mod(ctxt, attrs),
        -1091640629 => set_lum_off(ctxt, attrs),
        -934911823 => set_red_mod(ctxt, attrs),
        -934910178 => set_red_off(ctxt, attrs),
        -909500100 => set_sat_mod(ctxt, attrs),
        -909498455 => set_sat_off(ctxt, attrs),
        -887375076 => create_sys(ctxt, attrs),
        -409889940 => create_scrgb(ctxt, attrs),
        -305351098 => create_prst(ctxt, attrs),
        -152914620 => create_scheme(ctxt, attrs),
        -15865784 => set_blue_mod(ctxt, attrs),
        -15864139 => set_blue_off(ctxt, attrs),
        107524 => set_lum(ctxt, attrs),
        112785 => set_red(ctxt, attrs),
        113638 => set_sat(ctxt, attrs),
        3027034 => set_blue(ctxt, attrs),
        3560187 => set_tint(ctxt, attrs),
        92909918 => set_alpha(ctxt, attrs),
        98619139 => set_green(ctxt, attrs),
        109399597 => set_shade(ctxt, attrs),
        205217023 => set_green_mod(ctxt, attrs),
        205218668 => set_green_off(ctxt, attrs),
        id if id == extra_id => extra(ctxt, attrs),
        _ => {}
    }
}

fn pattern_of<'a, S, D: IImporterBase>(
    sp: &'a mut DmlShapeImporter<S, D>,
    belonger: i32,
) -> Option<&'a mut super::PatternFillImporter::PatternFillImporter<S, D>> {
    match belonger {
        3458 => Some(&mut sp.mLineImporter.mPatternFillImporter),
        3536191 => Some(&mut sp.mPatternFillImporter),
        _ => None,
    }
}

fn paint_pattern_end<S, D: IImporterBase>(
    patt: &mut super::PatternFillImporter::PatternFillImporter<S, D>,
    elementId: i32,
    is_bg: bool,
) {
    match elementId {
        -1949425841 => patt.setSrgbClr(is_bg),
        -887375076 => patt.setSysClr(is_bg),
        -409889940 => patt.setScrgbClr(is_bg),
        -305351098 => patt.setPrstClr(is_bg),
        -152914620 => patt.setSchemeClr(is_bg),
        97335464 if !is_bg => patt.importColor(),
        93641380 if is_bg => patt.importBackColor(),
        _ => {}
    }
}
