#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]
// Java: import cn.wps.drawing.app.KmoProtocol
use crate::cn::wps::drawing::app::KmoProtocol::KmoProtocol;
// Java: import cn.wps.drawing.app.format.fill.DgFillFormat
use crate::cn::wps::drawing::app::format::fill::DgFillFormat::DgFillFormat;
// Java: import cn.wps.drawing.app.util.PrecisionUtil
// LLM: PrecisionUtil module not yet available, using stub
// Java: import cn.wps.drawing.app.util.UnitsConverter
// LLM: UnitsConverter module not yet available, using stub
// Java: import cn.wps.drawing.data.fill.Fill
// LLM: Fill is a KProto type, using stub
// Java: import cn.wps.drawing.data.outline.Outline
// LLM: Outline is a KProto type, using stub
// Java: import cn.wps.kso.kproto.KoBin
// LLM: KoBin stub

pub struct Outline;
impl Outline {
    pub fn create() -> Self { Outline }
    pub fn save(&self) -> KoBin { KoBin }
    pub fn load(&mut self, _bin: KoBin) {}
    pub fn fill(&mut self) -> FillStub { FillStub }
    pub fn writeFill(&mut self, _f: ()) {}
    pub fn lineJoin(&mut self) -> LineJoinStub { LineJoinStub }
    pub fn headEnd(&mut self) -> LineEndStub { LineEndStub }
    pub fn tailEnd(&mut self) -> LineEndStub { LineEndStub }
    pub fn hasWidth(&self) -> bool { false }
    pub fn width(&self) -> i32 { 0 }
    pub fn putWidth(&mut self, _v: i32) {}
    pub fn compoundType(&self) -> i32 { 0 }
    pub fn putCompoundType(&mut self, _v: i32) {}
    pub fn lineCap(&self) -> i32 { 0 }
    pub fn putLineCap(&mut self, _v: i32) {}
    pub fn hasPresetDash(&self) -> bool { false }
    pub fn presetDash(&self) -> i32 { 0 }
    pub fn putPresetDash(&mut self, _v: i32) {}
}
pub struct FillStub;
impl FillStub {
    pub fn r#type(&self) -> i32 { 5 }
    pub fn hasType(&self) -> bool { false }
    pub fn putType(&mut self, _t: i32) {}
    pub fn save(&self) -> KoBin { KoBin }
}
pub struct LineJoinStub;
impl LineJoinStub {
    pub fn r#type(&self) -> i32 { 0 }
    pub fn putType(&mut self, _v: i32) {}
}
pub struct LineEndStub;
impl LineEndStub {
    pub fn hasType(&self) -> bool { false }
    pub fn r#type(&self) -> i32 { 0 }
    pub fn putType(&mut self, _v: i32) {}
    pub fn width(&self) -> i32 { 0 }
    pub fn putWidth(&mut self, _v: i32) {}
    pub fn len(&self) -> i32 { 0 }
    pub fn putLen(&mut self, _v: i32) {}
}
pub struct KoBin;
pub struct UnitsConverter;
impl UnitsConverter {
    pub fn emu2pt(v: f64) -> f64 { v / 12700.0 }
    pub fn pt2emu(v: f64) -> f64 { v * 12700.0 }
}
pub struct PrecisionUtil;
impl PrecisionUtil {
    pub fn equal(a: f64, b: f64) -> bool { (a - b).abs() < 1e-6 }
}
pub trait Cloneable {}

// Java: public class DgOutlineFormat implements KmoProtocol, Cloneable
pub struct DgOutlineFormat {
    // Java field: Outline outline
    pub(crate) outline: Outline,
    // Java field: DgFillFormat fillFormat
    pub(crate) fillFormat: DgFillFormat,
}

impl DgOutlineFormat {
    // Java: DgOutlineFormat()
    pub fn DgOutlineFormat_new() -> Self {
        DgOutlineFormat { outline: Outline::create(), fillFormat: DgFillFormat::DgFillFormat_new() }
    }
    // Java: DgOutlineFormat(Outline o)
    pub fn DgOutlineFormat_from(o: Option<Outline>) -> Self {
        let mut _self = DgOutlineFormat { outline: Outline::create(), fillFormat: DgFillFormat::DgFillFormat_new() };
        // Java: if (o != null)
        if let Some(o_val) = o {
            // Java: this.updateProtocol(o.save());
            let bin = o_val.save();
            // LLM: self.updateProtocol mapped to updateProtocol_local to avoid trait method conflict
            _self.updateProtocol_local(bin);
        }
        _self
    }
    // Java: void updateProtocol(KoBin outlineData)
    pub fn updateProtocol_local(&mut self, outlineData: KoBin) {
        // Java: this.outline.load(outlineData);
        self.outline.load(outlineData);
        // Java: this.fillFormat.updateProtocol(this.outline.fill().save());
        // LLM: cross-module KoBin types differ; outline.fill().save() depends on KProto chain
        todo!()
    }
    // Java: KoBin protocol()
    pub fn protocol_local(&mut self) -> KoBin {
        // Java: this.outline.writeFill(Fill.open(this.fillFormat.protocol()));
        // LLM: Fill.open depends on KProto; fillFormat.protocol() returns local KoBin
        // Java: return this.outline.save();
        self.outline.save()
    }
    // Java: Outline getOutline()
    pub fn getOutline(&self) -> &Outline {
        // Java: return this.outline;
        &self.outline
    }
    // Java: DgFillFormat fillFormat()
    pub fn fillFormat(&self) -> &DgFillFormat {
        // Java: return this.fillFormat;
        &self.fillFormat
    }
    // Java: int type()
    pub fn r#type(&mut self) -> i32 {
        // Java: return this.outline.fill().type();
        self.outline.fill().r#type()
    }
    // Java: void setType(int type)
    pub fn setType(&mut self, r#type: i32) {
        // Java: if (type != 5 && type != 0 && type != 4)
        if r#type != 5 && r#type != 0 && r#type != 4 {
            // Java: throw new IllegalArgumentException("线条中，只支持三种'无线条'，'实线'，'渐变线'");
            panic!("IllegalArgumentException");
        }
        // Java: else
        else {
            // Java: this.outline.fill().putType(type);
            self.outline.fill().putType(r#type);
            // Java: this.fillFormat.setType(type);
            self.fillFormat.setType(r#type);
        }
    }
    // Java: boolean isEmpty()
    pub fn isEmpty(&mut self) -> bool {
        // Java: return !this.outline.fill().hasType();
        !self.outline.fill().hasType()
    }
    // Java: void defaultLineWidth(int newType)
    pub fn defaultLineWidth(&mut self, newType: i32) {
        // Java: if (newType != 5)
        if newType != 5 {
            // Java: this.setLineWidth((double)1.0F);
            self.setLineWidth(1.0);
        }
    }
    // Java: int joinType()
    pub fn joinType(&mut self) -> i32 {
        // Java: return this.outline.lineJoin().type();
        self.outline.lineJoin().r#type()
    }
    // Java: void setJoinType(int joinType)
    pub fn setJoinType(&mut self, joinType: i32) {
        // Java: this.outline.lineJoin().putType(joinType);
        self.outline.lineJoin().putType(joinType);
    }
    // Java: int lineHeadType()
    pub fn lineHeadType(&mut self) -> i32 {
        // Java: return this.outline.headEnd().hasType() ? this.outline.headEnd().type() : this.lineTailType();
        if self.outline.headEnd().hasType() { self.outline.headEnd().r#type() } else { self.lineTailType() }
    }
    // Java: void setLineHeadType(int type)
    pub fn setLineHeadType(&mut self, r#type: i32) {
        // Java: this.outline.headEnd().putType(type);
        self.outline.headEnd().putType(r#type);
    }
    // Java: int lineHeadSizeType()
    pub fn lineHeadSizeType(&mut self) -> i32 {
        // Java: Outline.LineEnd head = this.outline.headEnd();
        let head = self.outline.headEnd();
        // Java: if (head.width() == 0 && head.len() == 0)
        if head.width() == 0 && head.len() == 0 {
            // Java: return 0;
            return 0;
        }
        // Java: else if (head.width() == 0 && head.len() == 1)
        else if head.width() == 0 && head.len() == 1 {
            // Java: return 1;
            return 1;
        }
        // Java: else if (head.width() == 0 && head.len() == 2)
        else if head.width() == 0 && head.len() == 2 {
            // Java: return 2;
            return 2;
        }
        // Java: else if (head.width() == 1 && head.len() == 0)
        else if head.width() == 1 && head.len() == 0 {
            // Java: return 3;
            return 3;
        }
        // Java: else if (head.width() == 1 && head.len() == 2)
        else if head.width() == 1 && head.len() == 2 {
            // Java: return 5;
            return 5;
        }
        // Java: else if (head.width() == 2 && head.len() == 0)
        else if head.width() == 2 && head.len() == 0 {
            // Java: return 6;
            return 6;
        }
        // Java: else if (head.width() == 2 && head.len() == 1)
        else if head.width() == 2 && head.len() == 1 {
            // Java: return 7;
            return 7;
        }
        // Java: else
        else {
            // Java: return head.width() == 2 && head.len() == 2 ? 8 : 4;
            // LLM: head is declared above as let head
            return if head.width() == 2 && head.len() == 2 { 8 } else { 4 };
        }
    }
    // Java: void setLineHeadSizeType(int type)
    pub fn setLineHeadSizeType(&mut self, r#type: i32) {
        // Java: Outline.LineEnd head = this.outline.headEnd();
        let mut head = self.outline.headEnd();
        // Java: switch (type)
        match r#type {
            // Java: case 0:
            0 => {
                // Java: head.putWidth(0);
                head.putWidth(0);
                // Java: head.putLen(0);
                head.putLen(0);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 1:
            1 => {
                // Java: head.putWidth(0);
                head.putWidth(0);
                // Java: head.putLen(1);
                head.putLen(1);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 2:
            2 => {
                // Java: head.putWidth(0);
                head.putWidth(0);
                // Java: head.putLen(2);
                head.putLen(2);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 3:
            3 => {
                // Java: head.putWidth(1);
                head.putWidth(1);
                // Java: head.putLen(0);
                head.putLen(0);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 4:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: default:
            4 => {
                // Java: head.putWidth(1);
                head.putWidth(1);
                // Java: head.putLen(1);
                head.putLen(1);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 5:
            5 => {
                // Java: head.putWidth(1);
                head.putWidth(1);
                // Java: head.putLen(2);
                head.putLen(2);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 6:
            6 => {
                // Java: head.putWidth(2);
                head.putWidth(2);
                // Java: head.putLen(0);
                head.putLen(0);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 7:
            7 => {
                // Java: head.putWidth(2);
                head.putWidth(2);
                // Java: head.putLen(1);
                head.putLen(1);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 8:
            8 => {
                // Java: head.putWidth(2);
                head.putWidth(2);
                // Java: head.putLen(2);
                head.putLen(2);
            }
            _ => {
                // LLM: default same as case 4
                head.putWidth(1);
                head.putLen(1);
            }
        }
    }
    // Java: int lineTailType()
    pub fn lineTailType(&mut self) -> i32 {
        // Java: return this.outline.tailEnd().type();
        self.outline.tailEnd().r#type()
    }
    // Java: void setLineTailType(int type)
    pub fn setLineTailType(&mut self, r#type: i32) {
        // Java: this.outline.tailEnd().putType(type);
        self.outline.tailEnd().putType(r#type);
    }
    // Java: int lineTailSizeType()
    pub fn lineTailSizeType(&mut self) -> i32 {
        // Java: Outline.LineEnd tail = this.outline.tailEnd();
        let tail = self.outline.tailEnd();
        // Java: if (tail.width() == 0 && tail.len() == 0)
        if tail.width() == 0 && tail.len() == 0 {
            // Java: return 0;
            return 0;
        }
        // Java: else if (tail.width() == 0 && tail.len() == 1)
        else if tail.width() == 0 && tail.len() == 1 {
            // Java: return 1;
            return 1;
        }
        // Java: else if (tail.width() == 0 && tail.len() == 2)
        else if tail.width() == 0 && tail.len() == 2 {
            // Java: return 2;
            return 2;
        }
        // Java: else if (tail.width() == 1 && tail.len() == 0)
        else if tail.width() == 1 && tail.len() == 0 {
            // Java: return 3;
            return 3;
        }
        // Java: else if (tail.width() == 1 && tail.len() == 2)
        else if tail.width() == 1 && tail.len() == 2 {
            // Java: return 5;
            return 5;
        }
        // Java: else if (tail.width() == 2 && tail.len() == 0)
        else if tail.width() == 2 && tail.len() == 0 {
            // Java: return 6;
            return 6;
        }
        // Java: else if (tail.width() == 2 && tail.len() == 1)
        else if tail.width() == 2 && tail.len() == 1 {
            // Java: return 7;
            return 7;
        }
        // Java: else
        else {
            // Java: return tail.width() == 2 && tail.len() == 2 ? 8 : 4;
            // LLM: tail is declared above as let tail
            return if tail.width() == 2 && tail.len() == 2 { 8 } else { 4 };
        }
    }
    // Java: void setLineTailSizeType(int type)
    pub fn setLineTailSizeType(&mut self, r#type: i32) {
        // Java: Outline.LineEnd tailEnd = this.outline.tailEnd();
        let mut tailEnd = self.outline.tailEnd();
        // Java: switch (type)
        match r#type {
            // Java: case 0:
            0 => {
                // Java: tailEnd.putWidth(0);
                tailEnd.putWidth(0);
                // Java: tailEnd.putLen(0);
                tailEnd.putLen(0);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 1:
            1 => {
                // Java: tailEnd.putWidth(0);
                tailEnd.putWidth(0);
                // Java: tailEnd.putLen(1);
                tailEnd.putLen(1);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 2:
            2 => {
                // Java: tailEnd.putWidth(0);
                tailEnd.putWidth(0);
                // Java: tailEnd.putLen(2);
                tailEnd.putLen(2);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 3:
            3 => {
                // Java: tailEnd.putWidth(1);
                tailEnd.putWidth(1);
                // Java: tailEnd.putLen(0);
                tailEnd.putLen(0);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 4:
            // LLM JavaLine Aggregation: merged into the next Rust statement
            // Java: default:
            4 => {
                // Java: tailEnd.putWidth(1);
                tailEnd.putWidth(1);
                // Java: tailEnd.putLen(1);
                tailEnd.putLen(1);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 5:
            5 => {
                // Java: tailEnd.putWidth(1);
                tailEnd.putWidth(1);
                // Java: tailEnd.putLen(2);
                tailEnd.putLen(2);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 6:
            6 => {
                // Java: tailEnd.putWidth(2);
                tailEnd.putWidth(2);
                // Java: tailEnd.putLen(0);
                tailEnd.putLen(0);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 7:
            7 => {
                // Java: tailEnd.putWidth(2);
                tailEnd.putWidth(2);
                // Java: tailEnd.putLen(1);
                tailEnd.putLen(1);
                // Java: break;
                // LLM No Rust equivalent: Rust match arms don't need break
            }
            // Java: case 8:
            8 => {
                // Java: tailEnd.putWidth(2);
                tailEnd.putWidth(2);
                // Java: tailEnd.putLen(2);
                tailEnd.putLen(2);
            }
            _ => {
                // LLM: default same as case 4
                tailEnd.putWidth(1);
                tailEnd.putLen(1);
            }
        }
    }
    // Java: boolean hasLineWidth()
    pub fn hasLineWidth(&self) -> bool {
        // Java: return this.outline.hasWidth();
        self.outline.hasWidth()
    }
    // Java: double lineWidth()
    pub fn lineWidth(&self) -> f64 {
        // Java: return UnitsConverter.emu2pt((double)this.outline.width());
        UnitsConverter::emu2pt(self.outline.width() as f64)
    }
    // Java: void setLineWidth(double lineWidth)
    pub fn setLineWidth(&mut self, lineWidth: f64) {
        // Java: this.outline.putWidth((int)UnitsConverter.pt2emu(lineWidth));
        self.outline.putWidth(UnitsConverter::pt2emu(lineWidth) as i32);
    }
    // Java: int compoundType()
    pub fn compoundType(&self) -> i32 {
        // Java: return this.outline.compoundType();
        self.outline.compoundType()
    }
    // Java: void setCompoundType(int type)
    pub fn setCompoundType(&mut self, r#type: i32) {
        // Java: this.outline.putCompoundType(type);
        self.outline.putCompoundType(r#type);
    }
    // Java: int lineEndingCapType()
    pub fn lineEndingCapType(&self) -> i32 {
        // Java: return this.outline.lineCap();
        self.outline.lineCap()
    }
    // Java: void setLineEndingCapType(int capType)
    pub fn setLineEndingCapType(&mut self, capType: i32) {
        // Java: this.outline.putLineCap(capType);
        self.outline.putLineCap(capType);
    }
    // Java: boolean hasDashType()
    pub fn hasDashType(&self) -> bool {
        // Java: return this.outline.hasPresetDash();
        self.outline.hasPresetDash()
    }
    // Java: int dashType()
    pub fn dashType(&self) -> i32 {
        // Java: return this.outline.presetDash();
        self.outline.presetDash()
    }
    // Java: void setDashType(int dash)
    pub fn setDashType(&mut self, dash: i32) {
        // Java: this.outline.putPresetDash(dash);
        self.outline.putPresetDash(dash);
    }
    // Java: DgOutlineFormat clone()
    pub fn clone(&self) -> DgOutlineFormat {
        // Java: DgOutlineFormat outlineFormat = new DgOutlineFormat();
        let mut outlineFormat = DgOutlineFormat::DgOutlineFormat_new();
        // Java: outlineFormat.updateProtocol(this.protocol());
        // LLM: protocol() depends on KoBin serialization chain
        // Java: return outlineFormat;
        outlineFormat
    }
    // Java: static boolean isEqual(DgOutlineFormat outlineFormat1, DgOutlineFormat outlineFormat2)
    pub fn isEqual(outlineFormat1: &mut DgOutlineFormat, outlineFormat2: &mut DgOutlineFormat) -> bool {
        // Java: boolean isEqualType = outlineFormat1.type() == outlineFormat2.type();
        let isEqualType = outlineFormat1.r#type() == outlineFormat2.r#type();
        // Java: if (!isEqualType)
        if !isEqualType {
            // Java: return false;
            return false;
        }
        // Java: else if (!DgFillFormat.isEqual(outlineFormat1.fillFormat, outlineFormat2.fillFormat))
        else if !DgFillFormat::isEqual(&mut outlineFormat1.fillFormat, &mut outlineFormat2.fillFormat) {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: return isEqualSolidPart(outlineFormat1, outlineFormat2);
            return DgOutlineFormat::isEqualSolidPart(outlineFormat1, outlineFormat2);
        }
    }
    // Java: static boolean isEqualSolidPart(DgOutlineFormat outlineFormat1, DgOutlineFormat outlineFormat2)
    pub fn isEqualSolidPart(outlineFormat1: &mut DgOutlineFormat, outlineFormat2: &mut DgOutlineFormat) -> bool {
        // Java: boolean isEqualOther = PrecisionUtil.equal(outlineFormat1.lineWidth(), outlineFormat2.lineWidth()) && outlineFormat1.compoundType() == outlineFormat2.compoundType() && outlineFormat1.dashType() == outlineFormat2.dashType() && outlineFormat1.lineEndingCapType() == outlineFormat2.lineEndingCapType() && outlineFormat1.joinType() == outlineFormat2.joinType() && outlineFormat1.lineHeadSizeType() == outlineFormat2.lineHeadSizeType() && outlineFormat1.lineHeadType() == outlineFormat2.lineHeadType() && outlineFormat1.lineTailSizeType() == outlineFormat2.lineTailSizeType() && outlineFormat1.lineTailType() == outlineFormat2.lineTailType();
        let isEqualOther = PrecisionUtil::equal(outlineFormat1.lineWidth(), outlineFormat2.lineWidth())
            && outlineFormat1.compoundType() == outlineFormat2.compoundType()
            && outlineFormat1.dashType() == outlineFormat2.dashType()
            && outlineFormat1.lineEndingCapType() == outlineFormat2.lineEndingCapType()
            && outlineFormat1.joinType() == outlineFormat2.joinType()
            && outlineFormat1.lineHeadSizeType() == outlineFormat2.lineHeadSizeType()
            && outlineFormat1.lineHeadType() == outlineFormat2.lineHeadType()
            && outlineFormat1.lineTailSizeType() == outlineFormat2.lineTailSizeType()
            && outlineFormat1.lineTailType() == outlineFormat2.lineTailType();
        // Java: return isEqualOther;
        isEqualOther
    }
}

impl KmoProtocol for DgOutlineFormat {
    fn protocol(&self) -> Box<dyn crate::cn::wps::kso::kproto::KoBin::KoBin> { todo!() }
    fn updateProtocol(&mut self, _var1: &dyn crate::cn::wps::kso::kproto::KoBin::KoBin) { todo!() }
}
impl Cloneable for DgOutlineFormat {}
