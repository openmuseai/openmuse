#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::ops::{Deref, DerefMut};
use std::{any::Any, cell::RefCell, rc::Rc};
use crate::io::simple::ElementHandlerSimple::{ElementHandlerRef, ElementHandlerSimple};
use crate::io::xwpf::IDmlImporter::DmlImporterRef;

// Java: import cn.wps.base.io.SubDocType
// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.io.simple.ElementHandlerSimple
// Java: import cn.wps.io.xwpf.IDmlImporter
// Java: import org.xml.sax.Attributes

use super::DmlHandlerBase::DmlHandlerBase;
use super::SpprDescendantHandler::SpprDescendantHandler;
use super::XfrmBranchHandler::XfrmBranchHandler;
use super::LnBranchHandler::LnBranchHandler;
use super::CustgeomHandler::CustgeomHandler;
use super::PattFillHandler::PattFillHandler;
use super::BlipFillHandler::BlipFillHandler;
use super::PrstgeomHandler::PrstgeomHandler;
use super::SolidFillBranchHandler::SolidFillBranchHandler;
use super::GrpFillHandler::GrpFillHandler;
use super::EffectLstBranchHandler::EffectLstBranchHandler;
use super::GradFillHandler::GradFillHandler;

// Java: public class SpprHandler extends DmlHandlerBase
pub struct SpprHandler {
    parent: DmlHandlerBase,
    // Java field: SpprDescendantHandler mSpprDescendantHandler
    mSpprDescendantHandler: Option<SpprDescendantHandler>,
    // Java field: XfrmBranchHandler mXfrmBranchHandler
    mXfrmBranchHandler: Option<XfrmBranchHandler>,
    // Java field: LnBranchHandler mLnHandler
    mLnHandler: Option<LnBranchHandler>,
    // Java field: SolidFillBranchHandler mSolidFillHandler
    mSolidFillHandler: Option<Box<dyn std::any::Any>>,
    // Java field: GradFillHandler mGradFillHandler
    mGradFillHandler: Option<Box<dyn std::any::Any>>,
    // Java field: GrpFillHandler mGrpFillHandler
    mGrpFillHandler: Option<Box<dyn std::any::Any>>,
    // Java field: CustgeomHandler mCustgeomHandler
    mCustgeomHandler: Option<CustgeomHandler>,
    // Java field: BlipFillHandler mBlipFillHandler
    mBlipFillHandler: Option<Box<dyn std::any::Any>>,
    // Java field: EffectLstBranchHandler mEffectLstHandler
    mEffectLstHandler: Option<Box<dyn std::any::Any>>,
    // Java field: PattFillHandler mPattFillHandler
    mPattFillHandler: Option<PattFillHandler>,
    // Java field: PrstgeomHandler mPrstgeomHandler
    mPrstgeomHandler: Option<Box<dyn std::any::Any>>,
    mTypedBlipFillHandler: RefCell<Option<ElementHandlerRef>>,
    mTypedXfrmHandler: RefCell<Option<ElementHandlerRef>>,
    mTypedSpprDescendantHandler: RefCell<Option<ElementHandlerRef>>,
    mTypedPrstgeomHandler: RefCell<Option<ElementHandlerRef>>,
    mTypedSolidFillHandler: RefCell<Option<ElementHandlerRef>>,
    mTypedGrpFillHandler: RefCell<Option<ElementHandlerRef>>,
    mTypedEffectLstHandler: RefCell<Option<ElementHandlerRef>>,
    mTypedPattFillHandler: RefCell<Option<ElementHandlerRef>>,
    mTypedGradFillHandler: RefCell<Option<ElementHandlerRef>>,
    mTypedLnHandler: RefCell<Option<ElementHandlerRef>>,
    mTypedCustgeomHandler: RefCell<Option<ElementHandlerRef>>,
    mIsGroup: bool,
}

impl Deref for SpprHandler {
    type Target = DmlHandlerBase;
    fn deref(&self) -> &Self::Target { &self.parent }
}

impl DerefMut for SpprHandler {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parent }
}

// Java: public class SpprHandler extends DmlHandlerBase
impl SpprHandler {
    // Java: SpprHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new(dmlImporter: Box<dyn std::any::Any + Send + Sync>, part: Option<Box<dyn std::any::Any + Send + Sync>>, subDocType: i32) -> Self {
        // Java: super(dmlImporter, part, subDocType);
        SpprHandler {
            parent: DmlHandlerBase::new(dmlImporter, part, subDocType),
            mSpprDescendantHandler: None,
            mXfrmBranchHandler: None,
            mLnHandler: None,
            mSolidFillHandler: None,
            mGradFillHandler: None,
            mGrpFillHandler: None,
            mCustgeomHandler: None,
            mBlipFillHandler: None,
            mEffectLstHandler: None,
            mPattFillHandler: None,
            mPrstgeomHandler: None,
            mTypedBlipFillHandler: RefCell::new(None),
            mTypedXfrmHandler: RefCell::new(None),
            mTypedSpprDescendantHandler: RefCell::new(None),
            mTypedPrstgeomHandler: RefCell::new(None),
            mTypedSolidFillHandler: RefCell::new(None),
            mTypedGrpFillHandler: RefCell::new(None),
            mTypedEffectLstHandler: RefCell::new(None),
            mTypedPattFillHandler: RefCell::new(None),
            mTypedGradFillHandler: RefCell::new(None),
            mTypedLnHandler: RefCell::new(None),
            mTypedCustgeomHandler: RefCell::new(None),
            mIsGroup: false,
        }
    }

    // Java: SpprHandler(IDmlImporter dmlImporter, POIXMLDocumentPart part, SubDocType subDocType)
    pub fn new_typed(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32) -> Self {
        Self {
            parent: DmlHandlerBase::new_typed(dmlImporter, part, subDocType),
            mSpprDescendantHandler: None, mXfrmBranchHandler: None, mLnHandler: None,
            mSolidFillHandler: None, mGradFillHandler: None, mGrpFillHandler: None,
            mCustgeomHandler: None, mBlipFillHandler: None, mEffectLstHandler: None,
            mPattFillHandler: None, mPrstgeomHandler: None,
            mTypedBlipFillHandler: RefCell::new(None),
            mTypedXfrmHandler: RefCell::new(None),
            mTypedSpprDescendantHandler: RefCell::new(None),
            mTypedPrstgeomHandler: RefCell::new(None),
            mTypedSolidFillHandler: RefCell::new(None),
            mTypedGrpFillHandler: RefCell::new(None),
            mTypedEffectLstHandler: RefCell::new(None),
            mTypedPattFillHandler: RefCell::new(None),
            mTypedGradFillHandler: RefCell::new(None),
            mTypedLnHandler: RefCell::new(None),
            mTypedCustgeomHandler: RefCell::new(None),
            mIsGroup: false,
        }
    }

    pub fn as_handler_ref(self) -> ElementHandlerRef { Rc::new(RefCell::new(self)) }

    // Java: private ElementHandlerSimple getBlipFillHandler()
    fn get_typed_blip_fill_handler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedBlipFillHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(BlipFillHandler::new(
                self.parent.mTypedDmlImporter.as_ref()?.clone(),
                self.parent.mTypedPart.clone(), self.parent.mSubDocType, 3536191,
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getXfrmBranchHandler()
    fn get_typed_xfrm_handler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedXfrmHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(XfrmBranchHandler::new_typed(
                self.parent.mTypedDmlImporter.as_ref()?.clone(),
                self.parent.mTypedPart.clone(), self.parent.mSubDocType,
                self.isGroup(),
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getSpprDescendantHandler()
    fn get_typed_sppr_descendant_handler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedSpprDescendantHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(SpprDescendantHandler::new_typed(
                self.parent.mTypedDmlImporter.as_ref()?.clone(),
                self.parent.mTypedPart.clone(), self.parent.mSubDocType,
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getPrstgeomHandler()
    fn get_typed_prstgeom_handler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedPrstgeomHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(PrstgeomHandler::new(
                self.parent.mTypedDmlImporter.as_ref()?.clone(),
                self.parent.mTypedPart.clone(), self.parent.mSubDocType,
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getSolidFillHandler()
    fn get_typed_solid_fill_handler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedSolidFillHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(SolidFillBranchHandler::new(
                self.parent.mTypedDmlImporter.as_ref()?.clone(), 3536191,
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getGrpFillHandler()
    fn get_typed_grp_fill_handler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedGrpFillHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(GrpFillHandler::new(
                self.parent.mTypedDmlImporter.as_ref()?.clone(), 3536191,
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getEffectLstHandler()
    fn get_typed_effect_lst_handler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedEffectLstHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(EffectLstBranchHandler::new(
                self.parent.mTypedDmlImporter.as_ref()?.clone(),
                self.parent.mTypedPart.clone(), self.parent.mSubDocType, 3536191,
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getPattFillHandler()
    fn get_typed_patt_fill_handler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedPattFillHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(PattFillHandler::new(
                self.parent.mTypedDmlImporter.as_ref()?.clone(),
                self.parent.mTypedPart.clone(), self.parent.mSubDocType, 3536191,
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getGradFillHandler()
    fn get_typed_grad_fill_handler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedGradFillHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(GradFillHandler::new(
                self.parent.mTypedDmlImporter.as_ref()?.clone(), 3536191,
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getLnHandler()
    fn get_typed_ln_handler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedLnHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(LnBranchHandler::new(
                self.parent.mTypedDmlImporter.as_ref()?.clone(),
                self.parent.mTypedPart.clone(), self.parent.mSubDocType,
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: private ElementHandlerSimple getCustgeomHandler()
    fn get_typed_custgeom_handler(&self) -> Option<ElementHandlerRef> {
        let mut slot = self.mTypedCustgeomHandler.borrow_mut();
        if slot.is_none() {
            *slot = Some(CustgeomHandler::new(
                self.parent.mTypedDmlImporter.as_ref()?.clone(),
                self.parent.mTypedPart.clone(), self.parent.mSubDocType,
            ).as_handler_ref());
        }
        slot.clone()
    }

    // Java: void onStart(int elementId, String namespaceURI, String localName, String qualifiedName, Attributes attributes)
    pub fn onStart_with_id(&self, elementId: i32, namespaceURI: &str, localName: &str, qualifiedName: &str, attributes: &dyn std::any::Any) {
    }

    // Java: void onEnd(int elementId, String namespaceURI)
    pub fn onEnd_with_id(&self, elementId: i32, namespaceURI: &str) {
    }

    // Java: ElementHandlerSimple getElementHandler(int elementId, String namespaceURI)
    pub fn getElementHandler(&self, elementId: i32, namespaceURI: &str) -> Option<Box<dyn std::any::Any + Send + Sync>> {
      // Java: switch (elementId)
      match elementId {
         // Java: case -1468679044:
         -1468679044_i32 => { None }
         // Java: case -1353661746:
         -1353661746_i32 => { None }
         // Java: case -1041060124:
         -1041060124_i32 => { None }
         // Java: case -875836993:
         -875836993_i32 => { None }
         // Java: case -825444332:
         -825444332_i32 => { None }
         // Java: case 3458:
         3458 => { None }
         // Java: case 3676585:
         3676585 => { None }
         // Java: case 88612081:
         88612081 => { None }
         // Java: case 292945928:
         292945928 => { None }
         // Java: case 604976175:
         604976175 => { None }
         // Java: case 1245072724:
         1245072724 => { None }
         // Java: default:
         _ => { None }
      }
    }

    // Java: boolean isGroup()
    pub fn isGroup(&self) -> bool { self.mIsGroup }

    // Java: GrpSpprHandler.isGroup() overrides SpprHandler.isGroup().
    pub fn new_typed_group(dmlImporter: DmlImporterRef, part: Option<Rc<dyn Any>>, subDocType: i32) -> Self {
        let mut handler = Self::new_typed(dmlImporter, part, subDocType);
        handler.mIsGroup = true;
        handler
    }
}

impl ElementHandlerSimple for SpprHandler {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn filterIndex(&self) -> i32 { -1 }
    fn onStart(&mut self, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd(&mut self, _: &str, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onStart_with_id(&mut self, _: i32, _: &str, _: &str, _: &str, _: &dyn Any) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onEnd_with_id(&mut self, _: i32, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters(&mut self, _: &[char], _: i32, _: i32) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn onCharacters_str(&mut self, _: &str) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
    fn getElementHandler(&self, id: i32, _: &str) -> Option<ElementHandlerRef> {
        match id {
            // Java: case -825444332: return this.getBlipFillHandler();
            -825444332 => self.get_typed_blip_fill_handler(),
            // Java: case 3676585: return this.getXfrmBranchHandler();
            3676585 => self.get_typed_xfrm_handler(),
            // Java: case -1041060124: return this.getSpprDescendantHandler();
            -1041060124 => self.get_typed_sppr_descendant_handler(),
            // Java: case -875836993: return this.getPrstgeomHandler();
            -875836993 => self.get_typed_prstgeom_handler(),
            // Java: case -1353661746: return this.getSolidFillHandler();
            -1353661746 => self.get_typed_solid_fill_handler(),
            // Java: case 292945928: return this.getGrpFillHandler();
            292945928 => self.get_typed_grp_fill_handler(),
            // Java: case -1468679044: return this.getEffectLstHandler();
            -1468679044 => self.get_typed_effect_lst_handler(),
            // Java: case 1245072724: return this.getPattFillHandler();
            1245072724 => self.get_typed_patt_fill_handler(),
            // Java: case 88612081: return this.getGradFillHandler();
            88612081 => self.get_typed_grad_fill_handler(),
            // Java: case 3458: return this.getLnHandler();
            3458 => self.get_typed_ln_handler(),
            // Java: case 604976175: return this.getCustgeomHandler();
            604976175 => self.get_typed_custgeom_handler(),
            _ => None,
        }
    }
}
