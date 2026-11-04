impl IImporterBase for RecordingDmlImporter {
    fn mode(&self) -> i32 { 0 }
}

impl io_wps_io::cn::wps::io::xwpf::IDmlCommonImporter::IDmlCommonImporter for RecordingDmlImporter {
    fn startGlow(&mut self, var1: i32, var2: &dyn std::any::Any) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endGlow(&mut self, var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startReflection(&mut self, var1: &dyn std::any::Any) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startGradFill(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endGradFill(&mut self, var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startGrpFill(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endGrpFill(&mut self, var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startSolidFill(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endSolidFill(&mut self, var1: i32, var2: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startGs(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endGs(&mut self, var1: i32, var2: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startLin(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endLin(&mut self, var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startPath(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endPath(&mut self, var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startTileRect(&mut self, var1: i32, var2: i32, var3: &dyn std::any::Any) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endTileRect(&mut self, var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startLn(&mut self, var1: i32, var2: &dyn std::any::Any) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endLn(&mut self, var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startNoFill(&mut self, var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
}

impl IDmlImporter for RecordingDmlImporter {
    fn getDocumentImporter(&self) -> &dyn IDocumentImporter { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn setDrawingInfo(&mut self, _var1: io_wps_io::cn::wps::io::xwpf::IDmlImporter::DrawingInfoRef) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn finish(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startAnchorChild(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startAnchor(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endAnchor(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startInlineChild(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startInline(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endInline(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startSingleShape(&mut self, _var1: bool) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endSingleShape(&mut self, _var1: bool) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startSppr(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endSppr(&mut self, _var1: &str) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startBlipFill(&mut self, _var1: i32, _var2: &dyn SaxAttributes, _var3: &io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart, _var4: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endBlipFill(&mut self, _var1: i32, _var2: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startEffectLst(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endEffectLst(&mut self, _var1: &str) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startPositionH(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endPositionH(&mut self, _var1: i32, _var2: Option<&str>) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startPositionV(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endPositionV(&mut self, _var1: i32, _var2: Option<&str>) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startGrpSppr(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endGrpSppr(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startXfrm(&mut self, _var1: i32, _var2: &dyn SaxAttributes, _var3: bool) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endXfrm(&mut self, _var1: i32, _var2: bool) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startLnRef(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endLnRef(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startFillRef(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endFillRef(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startEffectRef(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endEffectRef(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startFillToRect(&mut self, _var1: i32, _var2: i32, _var3: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endFillToRect(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startMoveTo(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endMoveTo(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startCubicBezto(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endCubicBezto(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startLineTo(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endLineTo(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn starArcTo(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endArcTo(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn starQuadBezTo(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endQuadBezTo(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startPathLst(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endPathLst(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startPath2(&mut self, _var1: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endPath2(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endCustgeom(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startClose(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startPattFill(&mut self, _var1: i32, _var2: i32, _var3: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endPattFill(&mut self, _var1: i32, _var2: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startFgClr(&mut self, _var1: i32, _var2: i32, _var3: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endFgClr(&mut self, _var1: i32, _var2: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startBgClr(&mut self, _var1: i32, _var2: i32, _var3: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endBgClr(&mut self, _var1: i32, _var2: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startTxbx(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endTxbx(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startContentPart(&mut self, _var1: &dyn SaxAttributes, _var2: &io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startRelIds(&mut self, _var1: &dyn SaxAttributes, _var2: &io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart, _var3: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startChart(&mut self, _var1: &dyn SaxAttributes, _var2: &io_wps_io::cn::wps::io::opc::model::POIXMLDocumentPart::POIXMLDocumentPart, _var3: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startPic(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startLinkedTxbx(&mut self, _var1: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startPrstgeom(&mut self, _var1: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endPrstgeom(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startAvlst(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startGdlst(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startCxnLst(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startCxn(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endCxn(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startPos(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startSizeRelH(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endSizeRelH(&mut self, _var1: i32, _var2: Option<&str>) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startSizeRelV(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endSizeRelV(&mut self, _var1: i32, _var2: Option<&str>) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startFill_Theme(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endFill_Theme(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startBgFill_Theme(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endBgFill_Theme(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startLn_Theme(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endLn_Theme(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startEffectStyle_Theme(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endEffectStyle_Theme(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startClrScheme(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endClrScheme(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startClrMap(&mut self, _var1: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startInnerShdw(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endInnerShdw(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startOuterShdw(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endOuterShdw(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startPrstShdw(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endPrstShdw(&mut self, _var1: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn buildTxbxChain(&mut self) { self.count.fetch_add(1, Ordering::SeqCst); }
    fn startSoftEdge(&mut self, _var1: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startWPC(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn onCanvasEnd(&mut self) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startBodyPr(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startWSP(&mut self, _var1: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startDuotone(&mut self, _var1: i32, _var2: &dyn SaxAttributes, _var3: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endDuotone(&mut self, _var1: i32, _var2: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startCnvpr(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startClrFrom(&mut self, _var1: i32, _var2: &dyn SaxAttributes, _var3: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn endClrFrom(&mut self, _var1: i32, _var2: i32) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
    fn startPrstTxWarp(&mut self, _var1: i32, _var2: &dyn SaxAttributes) { unreachable!("FinishReadingWork only calls buildTxbxChain") }
}
