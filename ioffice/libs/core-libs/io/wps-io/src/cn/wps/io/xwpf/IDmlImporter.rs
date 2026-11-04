#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
use crate::io::xwpf::SaxAttributes::SaxAttributes;
use core_libs_util::moffice::property::MutablePropertySet::MutablePropertySet;
use crate::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart;

// Java: import cn.wps.io.opc.model.POIXMLDocumentPart
// Java: import cn.wps.moffice.property.MutablePropertySet
// Java: import org.xml.sax.Attributes

// Java: public interface IDmlImporter extends IDmlCommonImporter
pub type DmlImporterRef = std::rc::Rc<std::cell::RefCell<dyn IDmlImporter>>;
/// Java passes the same mutable property object from DrawingHandler to DmlImporter.
pub type DrawingInfoRef = std::rc::Rc<std::cell::RefCell<MutablePropertySet>>;

pub trait IDmlImporter: crate::io::xwpf::IDmlCommonImporter::IDmlCommonImporter {
    // Java: abstract IDocumentImporter getDocumentImporter()
    fn getDocumentImporter(&self) -> &dyn crate::io::xwpf::IDocumentImporter::IDocumentImporter;
    // Java returns the same importer object; the SAX handler tree needs a shared
    // handle to keep that identity while the DML importer is borrowed.
    fn getDocumentImporterRef(&self) -> Option<std::rc::Rc<dyn crate::io::xwpf::IDocumentImporter::IDocumentImporter>> { None }
    // Java: abstract void setDrawingInfo(MutablePropertySet var1)
    fn setDrawingInfo(&mut self, _var1: DrawingInfoRef);
    // Java: abstract void finish()
    fn finish(&mut self);
    // Java: abstract void startAnchorChild(int var1, Attributes var2)
    fn startAnchorChild(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void startAnchor(int var1, Attributes var2)
    fn startAnchor(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endAnchor()
    fn endAnchor(&mut self);
    // Java: abstract void startInlineChild(int var1, Attributes var2)
    fn startInlineChild(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void startInline(int var1, Attributes var2)
    fn startInline(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endInline()
    fn endInline(&mut self);
    // Java: abstract void startSingleShape(boolean var1)
    fn startSingleShape(&mut self, _var1: bool);
    // Java: abstract void endSingleShape(boolean var1)
    fn endSingleShape(&mut self, _var1: bool);
    // Java: abstract void startSppr(int var1, Attributes var2)
    fn startSppr(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endSppr(String var1)
    fn endSppr(&mut self, _var1: &str);
    // Java: abstract void startBlipFill(int var1, Attributes var2, POIXMLDocumentPart var3, int var4)
    fn startBlipFill(&mut self, _var1: i32, _var2: &dyn SaxAttributes, _var3: &POIXMLDocumentPart, _var4: i32);
    // Java: abstract void endBlipFill(int var1, int var2)
    fn endBlipFill(&mut self, _var1: i32, _var2: i32);
    // Java: abstract void startEffectLst(int var1, Attributes var2)
    fn startEffectLst(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endEffectLst(String var1)
    fn endEffectLst(&mut self, _var1: &str);
    // Java: abstract void startPositionH(int var1, Attributes var2)
    fn startPositionH(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endPositionH(int var1, String var2)
    fn endPositionH(&mut self, _var1: i32, _var2: Option<&str>);
    // Java: abstract void startPositionV(int var1, Attributes var2)
    fn startPositionV(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endPositionV(int var1, String var2)
    fn endPositionV(&mut self, _var1: i32, _var2: Option<&str>);
    // Java: abstract void startGrpSppr(int var1, Attributes var2)
    fn startGrpSppr(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endGrpSppr()
    fn endGrpSppr(&mut self);
    // Java: abstract void startXfrm(int var1, Attributes var2, boolean var3)
    fn startXfrm(&mut self, _var1: i32, _var2: &dyn SaxAttributes, _var3: bool);
    // Java: abstract void endXfrm(int var1, boolean var2)
    fn endXfrm(&mut self, _var1: i32, _var2: bool);
    // Java: abstract void startLnRef(int var1, Attributes var2)
    fn startLnRef(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endLnRef(int var1)
    fn endLnRef(&mut self, _var1: i32);
    // Java: abstract void startFillRef(int var1, Attributes var2)
    fn startFillRef(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endFillRef(int var1)
    fn endFillRef(&mut self, _var1: i32);
    // Java: abstract void startEffectRef(int var1, Attributes var2)
    fn startEffectRef(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endEffectRef(int var1)
    fn endEffectRef(&mut self, _var1: i32);
    // Java: abstract void startFillToRect(int var1, int var2, Attributes var3)
    fn startFillToRect(&mut self, _var1: i32, _var2: i32, _var3: &dyn SaxAttributes);
    // Java: abstract void endFillToRect(int var1)
    fn endFillToRect(&mut self, _var1: i32);
    // Java: abstract void startMoveTo(int var1, Attributes var2)
    fn startMoveTo(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endMoveTo(int var1)
    fn endMoveTo(&mut self, _var1: i32);
    // Java: abstract void startCubicBezto(int var1, Attributes var2)
    fn startCubicBezto(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endCubicBezto(int var1)
    fn endCubicBezto(&mut self, _var1: i32);
    // Java: abstract void startLineTo(int var1, Attributes var2)
    fn startLineTo(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endLineTo(int var1)
    fn endLineTo(&mut self, _var1: i32);
    // Java: abstract void starArcTo(int var1, Attributes var2)
    fn starArcTo(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endArcTo(int var1)
    fn endArcTo(&mut self, _var1: i32);
    // Java: abstract void starQuadBezTo(int var1, Attributes var2)
    fn starQuadBezTo(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endQuadBezTo(int var1)
    fn endQuadBezTo(&mut self, _var1: i32);
    // Java: abstract void startPathLst()
    fn startPathLst(&mut self);
    // Java: abstract void endPathLst()
    fn endPathLst(&mut self);
    // Java: abstract void startPath2(Attributes var1)
    fn startPath2(&mut self, _var1: &dyn SaxAttributes);
    // Java: abstract void endPath2()
    fn endPath2(&mut self);
    // Java: abstract void endCustgeom()
    fn endCustgeom(&mut self);
    // Java: abstract void startClose()
    fn startClose(&mut self);
    // Java: abstract void startPattFill(int var1, int var2, Attributes var3)
    fn startPattFill(&mut self, _var1: i32, _var2: i32, _var3: &dyn SaxAttributes);
    // Java: abstract void endPattFill(int var1, int var2)
    fn endPattFill(&mut self, _var1: i32, _var2: i32);
    // Java: abstract void startFgClr(int var1, int var2, Attributes var3)
    fn startFgClr(&mut self, _var1: i32, _var2: i32, _var3: &dyn SaxAttributes);
    // Java: abstract void endFgClr(int var1, int var2)
    fn endFgClr(&mut self, _var1: i32, _var2: i32);
    // Java: abstract void startBgClr(int var1, int var2, Attributes var3)
    fn startBgClr(&mut self, _var1: i32, _var2: i32, _var3: &dyn SaxAttributes);
    // Java: abstract void endBgClr(int var1, int var2)
    fn endBgClr(&mut self, _var1: i32, _var2: i32);
    // Java: abstract void startTxbx(int var1, Attributes var2)
    fn startTxbx(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endTxbx(int var1)
    fn endTxbx(&mut self, _var1: i32);
    // Java: abstract void startContentPart(Attributes var1, POIXMLDocumentPart var2)
    fn startContentPart(&mut self, _var1: &dyn SaxAttributes, _var2: &POIXMLDocumentPart);
    // Java: abstract void startRelIds(Attributes var1, POIXMLDocumentPart var2, int var3)
    fn startRelIds(&mut self, _var1: &dyn SaxAttributes, _var2: &POIXMLDocumentPart, _var3: i32);
    // Java: abstract void startChart(Attributes var1, POIXMLDocumentPart var2, int var3)
    fn startChart(&mut self, _var1: &dyn SaxAttributes, _var2: &POIXMLDocumentPart, _var3: i32);
    // Java: abstract void startPic(int var1)
    fn startPic(&mut self, _var1: i32);
    // Java: abstract void startLinkedTxbx(Attributes var1)
    fn startLinkedTxbx(&mut self, _var1: &dyn SaxAttributes);
    // Java: abstract void startPrstgeom(Attributes var1)
    fn startPrstgeom(&mut self, _var1: &dyn SaxAttributes);
    // Java: abstract void endPrstgeom()
    fn endPrstgeom(&mut self);
    // Java: abstract void startAvlst(int var1, Attributes var2)
    fn startAvlst(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void startGdlst(int var1, Attributes var2)
    fn startGdlst(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void startCxnLst(int var1, Attributes var2)
    fn startCxnLst(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void startCxn(int var1, Attributes var2)
    fn startCxn(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endCxn()
    fn endCxn(&mut self);
    // Java: abstract void startPos(int var1, Attributes var2)
    fn startPos(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void startSizeRelH(int var1, Attributes var2)
    fn startSizeRelH(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endSizeRelH(int var1, String var2)
    fn endSizeRelH(&mut self, _var1: i32, _var2: Option<&str>);
    // Java: abstract void startSizeRelV(int var1, Attributes var2)
    fn startSizeRelV(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endSizeRelV(int var1, String var2)
    fn endSizeRelV(&mut self, _var1: i32, _var2: Option<&str>);
    // Java: abstract void startFill_Theme(int var1, Attributes var2)
    fn startFill_Theme(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endFill_Theme(int var1)
    fn endFill_Theme(&mut self, _var1: i32);
    // Java: abstract void startBgFill_Theme(int var1, Attributes var2)
    fn startBgFill_Theme(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endBgFill_Theme(int var1)
    fn endBgFill_Theme(&mut self, _var1: i32);
    // Java: abstract void startLn_Theme(int var1, Attributes var2)
    fn startLn_Theme(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endLn_Theme(int var1)
    fn endLn_Theme(&mut self, _var1: i32);
    // Java: abstract void startEffectStyle_Theme(int var1, Attributes var2)
    fn startEffectStyle_Theme(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endEffectStyle_Theme(int var1)
    fn endEffectStyle_Theme(&mut self, _var1: i32);
    // Java: abstract void startClrScheme(int var1, Attributes var2)
    fn startClrScheme(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endClrScheme(int var1)
    fn endClrScheme(&mut self, _var1: i32);
    // Java: abstract void startClrMap(Attributes var1)
    fn startClrMap(&mut self, _var1: &dyn SaxAttributes);
    // Java: abstract void startInnerShdw(int var1, Attributes var2)
    fn startInnerShdw(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endInnerShdw(int var1)
    fn endInnerShdw(&mut self, _var1: i32);
    // Java: abstract void startOuterShdw(int var1, Attributes var2)
    fn startOuterShdw(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endOuterShdw(int var1)
    fn endOuterShdw(&mut self, _var1: i32);
    // Java: abstract void startPrstShdw(int var1, Attributes var2)
    fn startPrstShdw(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void endPrstShdw(int var1)
    fn endPrstShdw(&mut self, _var1: i32);
    // Java: abstract void buildTxbxChain()
    fn buildTxbxChain(&mut self);
    // Java: abstract void startSoftEdge(Attributes var1)
    fn startSoftEdge(&mut self, _var1: &dyn SaxAttributes);
    // Java: abstract void startWPC()
    fn startWPC(&mut self);
    // Java: abstract void onCanvasEnd()
    fn onCanvasEnd(&mut self);
    // Java: abstract void startBodyPr(int var1, Attributes var2)
    fn startBodyPr(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void startWSP(Attributes var1)
    fn startWSP(&mut self, _var1: &dyn SaxAttributes);
    // Java: abstract void startDuotone(int var1, Attributes var2, int var3)
    fn startDuotone(&mut self, _var1: i32, _var2: &dyn SaxAttributes, _var3: i32);
    // Java: abstract void endDuotone(int var1, int var2)
    fn endDuotone(&mut self, _var1: i32, _var2: i32);
    // Java: abstract void startCnvpr(int var1, Attributes var2)
    fn startCnvpr(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
    // Java: abstract void startClrFrom(int var1, Attributes var2, int var3)
    fn startClrFrom(&mut self, _var1: i32, _var2: &dyn SaxAttributes, _var3: i32);
    // Java: abstract void endClrFrom(int var1, int var2)
    fn endClrFrom(&mut self, _var1: i32, _var2: i32);
    // Java: abstract void startPrstTxWarp(int var1, Attributes var2)
    fn startPrstTxWarp(&mut self, _var1: i32, _var2: &dyn SaxAttributes);
}
