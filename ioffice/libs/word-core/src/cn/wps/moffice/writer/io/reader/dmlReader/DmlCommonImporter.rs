// Java: cn.wps.moffice.writer.io.reader.dmlReader.DmlCommonImporter
#![allow(non_snake_case)]

use io_wps_io::cn::wps::io::xwpf::{IImporterBase::IImporterBase, SaxAttributes::SaxAttributes};
use crate::cn::wps::moffice::writer::io::reader::{
    xml07::Xml07Context::Xml07Context, ImporterBase::ImporterBase,
};
use super::{
    GlowImporter::GlowImporter, GradFillImporter::GradFillImporter, LineImporter::LineImporter,
    SolidFillImporter::SolidFillImporter,
};

// Java: public abstract class DmlCommonImporter extends ImporterBase implements IDmlCommonImporter
pub struct DmlCommonImporter {
    pub super_: ImporterBase,
}

impl DmlCommonImporter {
    // Java: DmlCommonImporter(Xml07Context xml07Cxt, int mode)
    pub fn new(xml07Cxt: Option<Xml07Context>, mode: i32) -> Self {
        // Java: super(mode, xml07Cxt, (Xml03Context)null);
        Self { super_: ImporterBase::ImporterBase(mode, xml07Cxt, None) }
    }

    pub fn mode(&self) -> i32 { self.super_.mode() }

    pub fn xml07Context(&self) -> Option<Xml07Context> {
        self.super_.xml07Context().cloned()
    }
}

// Java color switches. The abstract getters stay on DmlImporter; these methods
// are the parent bodies `super.start*` / `super.end*` call.
pub fn startGlow<S, D: IImporterBase>(glow: &mut GlowImporter<S, D>, elementId: i32, attrs: &dyn SaxAttributes) {
    // Java: if (null != glowImporter) switch (elementId)
    match elementId {
        -1949425841 => glow.createSrgbClr(attrs),
        -1091642274 => glow.setLumMod(attrs),
        -1091640629 => glow.setLumOff(attrs),
        -934911823 => glow.setRedMod(attrs),
        -934910178 => glow.setRedOff(attrs),
        -909500100 => glow.setSatMod(attrs),
        -909498455 => glow.setSatOff(attrs),
        -887375076 => glow.createSysClr(attrs),
        -409889940 => glow.createScrgbClr(attrs),
        -305351098 => glow.createPrstClr(attrs),
        -152914620 => glow.createSchemeClr(attrs),
        -15865784 => glow.setBlueMod(attrs),
        -15864139 => glow.setBlueOff(attrs),
        107524 => glow.setLum(attrs),
        112785 => glow.setRed(attrs),
        113638 => glow.setSat(attrs),
        3027034 => glow.setBlue(attrs),
        3175821 => glow.createGlowContext(attrs),
        3560187 => glow.setTint(attrs),
        92909918 => glow.setAlpha(attrs),
        98619139 => glow.setGreen(attrs),
        109399597 => glow.setShade(attrs),
        205217023 => glow.setGreenMod(attrs),
        205218668 => glow.setGreenOff(attrs),
        _ => {}
    }
}

pub fn endGlow<S, D: IImporterBase>(glow: &mut GlowImporter<S, D>, elementId: i32) {
    match elementId {
        -1949425841 => glow.setSrgbClr(),
        -887375076 => glow.setSysClr(),
        -409889940 => glow.setScrgbClr(),
        -305351098 => glow.setPrstClr(),
        -152914620 => glow.setSchemeClr(),
        _ => {}
    }
}

pub fn startSolidFill<S, D: IImporterBase>(solid: &mut SolidFillImporter<S, D>, elementId: i32, attrs: &dyn SaxAttributes) {
    match elementId {
        -1949425841 => solid.createSrgbClr(attrs),
        -1353661746 => solid.createSolidFillContext(),
        -1091642274 => solid.setLumMod(attrs),
        -1091640629 => solid.setLumOff(attrs),
        -934911823 => solid.setRedMod(attrs),
        -934910178 => solid.setRedOff(attrs),
        -909500100 => solid.setSatMod(attrs),
        -909498455 => solid.setSatOff(attrs),
        -887375076 => solid.createSysClr(attrs),
        -409889940 => solid.createScrgbClr(attrs),
        -305351098 => solid.createPrstClr(attrs),
        -152914620 => solid.createSchemeClr(attrs),
        -15865784 => solid.setBlueMod(attrs),
        -15864139 => solid.setBlueOff(attrs),
        107524 => solid.setLum(attrs),
        112785 => solid.setRed(attrs),
        113638 => solid.setSat(attrs),
        3027034 => solid.setBlue(attrs),
        3560187 => solid.setTint(attrs),
        92909918 => solid.setAlpha(attrs),
        98619139 => solid.setGreen(attrs),
        109399597 => solid.setShade(attrs),
        205217023 => solid.setGreenMod(attrs),
        205218668 => solid.setGreenOff(attrs),
        _ => {}
    }
}

pub fn endSolidFill<S, D: IImporterBase>(solid: &mut SolidFillImporter<S, D>, elementId: i32) {
    match elementId {
        -1949425841 => solid.setSrgbClr(),
        -887375076 => solid.setSysClr(),
        -409889940 => solid.setScrgbClr(),
        -305351098 => solid.setPrstClr(),
        -152914620 => solid.setSchemeClr(),
        _ => {}
    }
}

pub fn startGs<S, D: IImporterBase>(grad: &mut GradFillImporter<S, D>, elementId: i32, attrs: &dyn SaxAttributes) {
    match elementId {
        -1949425841 => grad.createSrgbClr(attrs),
        -1091642274 => grad.setLumMod(attrs),
        -1091640629 => grad.setLumOff(attrs),
        -934911823 => grad.setRedMod(attrs),
        -934910178 => grad.setRedOff(attrs),
        -909500100 => grad.setSatMod(attrs),
        -909498455 => grad.setSatOff(attrs),
        -887375076 => grad.createSysClr(attrs),
        -409889940 => grad.createScrgbClr(attrs),
        -305351098 => grad.createPrstClr(attrs),
        -152914620 => grad.createSchemeClr(attrs),
        -15865784 => grad.setBlueMod(attrs),
        -15864139 => grad.setBlueOff(attrs),
        3308 => grad.createGsContext(attrs),
        107524 => grad.setLum(attrs),
        112785 => grad.setRed(attrs),
        113638 => grad.setSat(attrs),
        3027034 => grad.setBlue(attrs),
        3560187 => grad.setTint(attrs),
        92909918 => grad.setAlpha(attrs),
        98619139 => grad.setGreen(attrs),
        109399597 => grad.setShade(attrs),
        205217023 => grad.setGreenMod(attrs),
        205218668 => grad.setGreenOff(attrs),
        _ => {}
    }
}

pub fn endGs<S, D: IImporterBase>(grad: &mut GradFillImporter<S, D>, elementId: i32) {
    match elementId {
        -1949425841 => grad.setSrgbClr(),
        -887375076 => grad.setSysClr(),
        -409889940 => grad.setScrgbClr(),
        -305351098 => grad.setPrstClr(),
        -152914620 => grad.setSchemeClr(),
        3308 => grad.importGradStop(),
        _ => {}
    }
}

pub fn startLin<S, D: IImporterBase>(grad: &mut GradFillImporter<S, D>, _elementId: i32, attributes: &dyn SaxAttributes) {
    // Java: gradFillImporter.setLin(attributes);
    grad.setLin(attributes);
}

pub fn startPath<S, D: IImporterBase>(grad: &mut GradFillImporter<S, D>, elementId: i32, attributes: &dyn SaxAttributes) {
    match elementId {
        -552691102 => grad.setFillToRect(attributes),
        3433509 => grad.setPath(attributes),
        _ => {}
    }
}

pub fn startTileRect<S, D: IImporterBase>(grad: &mut GradFillImporter<S, D>, _elementId: i32, attributes: &dyn SaxAttributes) {
    grad.setTileRect(attributes);
}

pub fn startLn<S, D: IImporterBase>(line: &mut LineImporter<S, D>, elementId: i32, attrs: &dyn SaxAttributes) {
    match elementId {
        -1041060124 => line.setNoFill(),
        -875930091 => line.setDashing(attrs),
        93630586 | 103906565 | 108704142 => line.setJoinStyle(elementId),
        _ => {}
    }
}

// Java: public void endLin / endPath / endTileRect / endLn / startNoFill are empty.
pub fn endLn(_elementId: i32) {}
pub fn endLin() {}
pub fn endPath() {}
pub fn endTileRect() {}
pub fn startNoFill() {}
