// java package cn.wps.et.ss.formula.ptg;
// rust: 以 PtgKind 承载当前已实现的 Java Ptg 子类；完整子类门面仍在 R0b 待办。
// 序列化宽度、sid、操作数类表按 koffice 对应类。

use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;
#[cfg(test)]
#[path="Ptg.output.review.rs"]
mod output_review;
#[cfg(test)]
#[path="Ptg.debug.review.rs"]
mod debug_review;
// java import cn.wps.et.ss.formula.constant.ConstantValueParser;
use crate::cn::wps::et::ss::formula::constant::ConstantValueParser::{ConstantValue, ConstantValueParser};

// java import cn.wps.et.ss.formula.parser.rav.DesiredOperandClass;
pub use crate::cn::wps::et::ss::formula::parser::rav::DesiredOperandClass::DesiredOperandClass as OperandClass;
// java import cn.wps.et.ss.formula.function.FunctionMetadata;
pub use crate::cn::wps::et::ss::formula::function::FunctionMetadata::FunctionMetadata as FuncMeta;
use crate::cn::wps::et::ss::formula::function::FunctionMetadataRegistry;
use crate::cn::wps::et::ss::formula::parser::rav::DesiredOperandClass::FormulaClass;

// java AbstractFunctionPtg.lookupIndex/isBuiltInFunctionName uppercase before registry lookup.
pub fn function_by_name(name: &str) -> Option<&'static FuncMeta> {
    FunctionMetadataRegistry::getFunctionByName(&name.to_ascii_uppercase())
}

// java FunctionMetadataRegistry.getFunctionByIndex(functionIndex)
fn function_by_index(index: u16) -> Option<&'static FuncMeta> {
    if index >= 368 { return None; }
    FunctionMetadataRegistry::getFunctionByIndex(index as i32)
}

#[derive(Clone, Debug)]
pub enum PtgKind {
    // java UnknownPtg sid 0。
    Unknown,
    // java Tbl03Ptg/Tbl10Ptg sid 2。
    Tbl { row: i32, col: u16, wide: bool },
    Int(u16),
    Number(f64),
    Str(String),
    // java StringPtg.read retains the Unicode flag and Java String UTF-16 units.
    StrRaw { units: Vec<u16>, wide: bool },
    // java BoolPtg sid 29, size 2
    Bool(bool),
    // java ErrPtg sid 28, size 2
    Err(u8),
    Ref {
        row: i32,
        col_raw: u16,
        wide: bool,
        relative: bool,
    },
    Area {
        first_row: i32,
        last_row: i32,
        first_col_raw: u16,
        last_col_raw: u16,
        wide: bool,
        relative: bool,
    },
    // java Ref3DPtg sid 58。wide 区分 Ref3D10Ptg / Ref3D03Ptg。
    Ref3D {
        extern_index: i32,
        row: i32,
        col_raw: u16,
        wide: bool,
    },
    // java Ref3D03ExtPtg/Ref3D10ExtPtg 额外存储第二个 sheet index。
    Ref3DExt { sheet_index1: i16, sheet_index2: i16, row: i32, col_raw: u16, wide: bool },
    // java Area3DPtg sid 59。
    Area3D {
        extern_index: i32,
        first_row: i32,
        last_row: i32,
        first_col_raw: u16,
        last_col_raw: u16,
        wide: bool,
    },
    // java Area3D03ExtPtg/Area3D10ExtPtg 额外存储第二个 sheet index。
    Area3DExt { sheet_index1: i16, sheet_index2: i16, first_row: i32, last_row: i32, first_col_raw: u16, last_col_raw: u16, wide: bool },
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    NotEqual,
    Power,
    Percent,
    UnaryPlus,
    UnaryMinus,
    Concat,
    // java IntersectionPtg / UnionPtg / RangePtg sid 15/16/17。
    Intersection,
    Union,
    Range,
    Paren,
    // java MissingArgPtg sid 22。
    MissingArg,
    // java SxNamePtg sid 24。
    SxName { index: i32, eptg: u8, field_name: Option<String>, item_name: Option<String> },
    Attr { options: u8, data: i16, jump_table: Option<Vec<i32>>, choose_offset:i32 },
    FuncVar { meta: &'static FuncMeta, nargs: u8 },
    // java FuncVarPtg.create 对未知函数：index 255，return class 32。
    FuncVarExt { index: u16, nargs: u8, return_class: u8 },
    // java FuncPtg: 固定参数函数通过函数索引查元数据。
    Func { meta: &'static FuncMeta },
    // java MemErrPtg / MemFuncPtg: 保留 Java 的原字段宽度。
    MemErr { reserved: i32, subex_len: i16 },
    MemFunc { subex_len: u16 },
    // java MemAreaPtg.Initial 在 token 区读 7 字节；finishReading 再读尾部区域表。
    MemAreaInitial { reserved: i32, subex_len: i16 },
    // java MemAreaPtg 完整对象；None 对应 Java field_3_areas == null。
    MemArea { reserved: i32, subex_len: i16, areas: Option<Vec<u8>> },
    // java ArrayPtg.Initial: 8-byte plain token; finishReading parses values from tail.
    ArrayInitial { reserved0: i32, reserved1: u16, reserved2: u8 },
    // java ArrayPtg: dimensions and ConstantValueParser values after all plain tokens.
    Array { reserved0: i32, reserved1: u16, reserved2: u8, columns: u16, rows: i32, values: Option<Vec<ConstantValue>> },
    // java NamePtg sid 35, size 5。index 是名称表下标，写出时 label = 1 + index。
    Name { index: i32 },
    // java NameXPtg sid 57, size 7。name_number 已是 1 + nameIndex。
    NameX { sheet_ref: i32, name_number: i32 },
    Exp { row: i32, col: u16, wide: bool },
    // java RefErrorPtg sid 42, size 5
    RefError,
    // java AreaErrPtg sid 43, size 9
    AreaErr,
    // java DeletedRef3DPtg / DeletedArea3DPtg sid 60/61，保留 unused 字段。
    DeletedRef3D { extern_index: u16, unused: i32 },
    DeletedArea3D { extern_index: u16, unused1: i32, unused2: i32 },
}

pub type PtgRef=crate::java_runtime::Rc<crate::java_runtime::RefCell<Ptg>>;
pub type PtgArray=crate::java_runtime::Rc<crate::java_runtime::RefCell<Box<[Option<PtgRef>]>>>;
// Native Java object/array reference transport. References are retained directly;
// numeric identity markers never reconstruct aliases of mutable token objects.
pub fn ptgRef(value:Ptg)->PtgRef{
    use crate::java_runtime::{Rc,RefCell};
    let singleton=value.javaIdentity();
    if singleton<1024{
        static SINGLETONS:std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<u64,PtgRef>>>=std::sync::LazyLock::new(||std::sync::Mutex::new(std::collections::HashMap::new()));
        SINGLETONS.lock().unwrap().entry(singleton).or_insert_with(||Rc::new(RefCell::new(value))).clone()
    }else{Rc::new(RefCell::new(value))}
}
pub fn ptgArray(values:Vec<Ptg>)->PtgArray{crate::java_runtime::Rc::new(crate::java_runtime::RefCell::new(values.into_iter().map(|p|Some(ptgRef(p))).collect::<Vec<_>>().into_boxed_slice()))}
// Read-only transport into the existing slice-based calculation consumers.
// The original mutable objects remain in their shared array; this is not clone().
pub fn ptgValues(array:&PtgArray)->Vec<Ptg>{array.borrow().iter().map(|p|p.as_ref().expect("NullPointerException").borrow().clone()).collect()}
// Native transport for the original getEncodedSize / serializePtgs loops:
// java if (ptgs[i] != null) { ... }
// Encoding skips null entries; calculation's ptgValues intentionally does not.
pub fn ptgEncodingValues(array:&PtgArray)->Vec<Ptg>{array.borrow().iter().filter_map(|p|p.as_ref().map(|p|p.borrow().clone())).collect()}
#[derive(Clone, Debug)]
pub struct Ptg {
    // rust: Java Ptg inherits Object.equals/hashCode; Vec clones preserve token references.
    pub java_identity:u64,
    pub kind: PtgKind,
    pub class: u8,
}

// Native exception transport between the two original readTokens overloads.
// Initial-token RuntimeExceptions are caught by the Java read loop; a negative
// ArrayPtg tail allocation escapes the stream overload and is caught only by
// the byte[] overload's outer catch(RuntimeException).
enum TokenReadFailure{Runtime,NegativeArraySize}
impl From<()> for TokenReadFailure{fn from(_:())->Self{Self::Runtime}}
// Native class metadata for the concrete instances already constructed by the
// original token reader. Method bodies stay in their corresponding class files.
impl Ptg{
    pub(crate) fn javaClassName(&self)->String{
        let name=match self.kind{
            PtgKind::Unknown=>"UnknownPtg",PtgKind::Tbl{wide:false,..}=>"Tbl03Ptg",PtgKind::Tbl{wide:true,..}=>"Tbl10Ptg",
            PtgKind::Int(_)=>"IntPtg",PtgKind::Number(_)=>"NumberPtg",PtgKind::Str(_)|PtgKind::StrRaw{..}=>"StringPtg",PtgKind::Bool(_)=>"BoolPtg",PtgKind::Err(_)=>"ErrPtg",
            PtgKind::Ref{wide:false,relative:false,..}=>"Ref03Ptg",PtgKind::Ref{wide:true,relative:false,..}=>"Ref10Ptg",PtgKind::Ref{wide:false,relative:true,..}=>"Ref03NPtg",PtgKind::Ref{wide:true,relative:true,..}=>"Ref10NPtg",
            PtgKind::Area{wide:false,relative:false,..}=>"Area03Ptg",PtgKind::Area{wide:true,relative:false,..}=>"Area10Ptg",PtgKind::Area{wide:false,relative:true,..}=>"Area03NPtg",PtgKind::Area{wide:true,relative:true,..}=>"Area10NPtg",
            PtgKind::Ref3D{wide:false,..}=>"Ref3D03Ptg",PtgKind::Ref3D{wide:true,..}=>"Ref3D10Ptg",PtgKind::Area3D{wide:false,..}=>"Area3D03Ptg",PtgKind::Area3D{wide:true,..}=>"Area3D10Ptg",
            PtgKind::Ref3DExt{wide:false,..}=>"external.Ref3D03ExtPtg",PtgKind::Ref3DExt{wide:true,..}=>"external.Ref3D10ExtPtg",PtgKind::Area3DExt{wide:false,..}=>"external.Area3D03ExtPtg",PtgKind::Area3DExt{wide:true,..}=>"external.Area3D10ExtPtg",
            PtgKind::Add=>"AddPtg",PtgKind::Subtract=>"SubtractPtg",PtgKind::Multiply=>"MultiplyPtg",PtgKind::Divide=>"DividePtg",PtgKind::Power=>"PowerPtg",PtgKind::Concat=>"ConcatPtg",PtgKind::LessThan=>"LessThanPtg",PtgKind::LessEqual=>"LessEqualPtg",PtgKind::Equal=>"EqualPtg",PtgKind::GreaterEqual=>"GreaterEqualPtg",PtgKind::GreaterThan=>"GreaterThanPtg",PtgKind::NotEqual=>"NotEqualPtg",
            PtgKind::Intersection=>"IntersectionPtg",PtgKind::Union=>"UnionPtg",PtgKind::Range=>"RangePtg",PtgKind::UnaryPlus=>"UnaryPlusPtg",PtgKind::UnaryMinus=>"UnaryMinusPtg",PtgKind::Percent=>"PercentPtg",PtgKind::Paren=>"ParenthesisPtg",PtgKind::MissingArg=>"MissingArgPtg",PtgKind::SxName{..}=>"SxNamePtg",PtgKind::Attr{..}=>"AttrPtg",
            PtgKind::Func{..}=>"FuncPtg",PtgKind::FuncVar{..}|PtgKind::FuncVarExt{..}=>"FuncVarPtg",PtgKind::MemErr{..}=>"MemErrPtg",PtgKind::MemFunc{..}=>"MemFuncPtg",PtgKind::MemArea{..}=>"MemAreaPtg",PtgKind::MemAreaInitial{..}=>"MemAreaPtg$Initial",PtgKind::Array{..}=>"ArrayPtg",PtgKind::ArrayInitial{..}=>"ArrayPtg$Initial",
            PtgKind::Name{..}=>"NamePtg",PtgKind::NameX{..}=>"NameXPtg",PtgKind::Exp{wide:false,..}=>"Exp03Ptg",PtgKind::Exp{wide:true,..}=>"Exp10Ptg",PtgKind::RefError=>"RefErrorPtg",PtgKind::AreaErr=>"AreaErrPtg",PtgKind::DeletedRef3D{..}=>"DeletedRef3DPtg",PtgKind::DeletedArea3D{..}=>"DeletedArea3DPtg",
        };format!("cn.wps.et.ss.formula.ptg.{name}")
    }
    // Native dispatch of the concrete Control/Scalar and Operand/Function
    // classes' constant implementations of abstract Ptg.isBaseToken().
    pub fn isBaseToken(&self)->bool{self.is_base_token()}
    // java public String toString() { return this.getClass().toString(); }
    pub fn toBaseDebugString(&self)->String{format!("class {}",self.javaClassName())}
    // java public final byte getPtgClass() { return this.ptgClass; }
    pub fn getPtgClass(&self)->i8{self.class as i8}
    // java public final char getRVAType() {
    pub fn getRVAType(&self)->char{
        // java if (this.isBaseToken()) { return '.'; } else { switch (this.ptgClass) {
        if self.isBaseToken(){'.'}else{match self.class{
            // java case 0: return 'R'; case 32: return 'V'; case 64: return 'A';
            0=>'R',32=>'V',64=>'A',
            // java default: throw new RuntimeException("Unknown operand class (" + this.ptgClass + ")");
            _=>panic!("RuntimeException: Unknown operand class ({})",self.class as i8),
        }}
    }
    // Native virtual dispatch only: each original algorithm remains in its class.
    pub fn toString(&self)->String{match self.kind{
        PtgKind::Int(_)|PtgKind::Number(_)|PtgKind::Str(_)|PtgKind::StrRaw{..}|PtgKind::Bool(_)|PtgKind::Err(_)|PtgKind::MissingArg|PtgKind::SxName{..}=>self.toScalarDebugString(),
        PtgKind::Ref{..}=>self.toRef2DDebugString(),PtgKind::Area{..}=>self.toArea2DDebugString(),PtgKind::Ref3D{..}=>self.toRef3DDebugString(),PtgKind::Area3D{..}=>self.toArea3DDebugString(),PtgKind::Ref3DExt{..}=>self.toRef3DExtDebugString(),PtgKind::Area3DExt{..}=>self.toArea3DExtDebugString(),
        PtgKind::Func{..}|PtgKind::FuncVar{..}|PtgKind::FuncVarExt{..}=>self.toFunctionDebugString(),PtgKind::Array{..}=>self.toArrayDebugString(),PtgKind::Attr{..}=>self.attrToString(),PtgKind::NameX{..}=>self.toNameXDebugString(),PtgKind::Exp{..}=>self.toExpDebugString(),PtgKind::Tbl{..}=>self.toTblDebugString(),PtgKind::RefError=>self.toRefErrorDebugString(),PtgKind::MemArea{..}=>self.toMemAreaDebugString(),PtgKind::MemFunc{..}=>self.toMemFuncDebugString(),_=>self.toBaseDebugString(),
    }}
}
// Native return-reference metadata: the original early `new Ptg[0]` differs
// from toPtgArray(emptyList)'s shared EMPTY_PTG_ARRAY. Owned slice consumers
// discard only this array identity, not the token objects.
enum TokenReadResult{FreshEmpty,Parsed(Vec<Ptg>)}
impl TokenReadResult{
    fn into_vec(self)->Vec<Ptg>{match self{Self::FreshEmpty=>vec![],Self::Parsed(v)=>v}}
    fn into_array(self)->PtgArray{match self{Self::FreshEmpty=>ptgArray(vec![]),Self::Parsed(v)=>if v.is_empty(){Ptg::EMPTY_PTG_ARRAY()}else{ptgArray(v)}}}
}

impl Ptg {
    // java public static final Ptg[] EMPTY_PTG_ARRAY = new Ptg[0];
    pub fn EMPTY_PTG_ARRAY()->PtgArray{static V:std::sync::OnceLock<PtgArray>=std::sync::OnceLock::new();V.get_or_init(||ptgArray(vec![])).clone()}
    // java public static Ptg[] readTokens(int size, byte[] data, SpreadsheetVersion sv)
    pub fn readTokens(size: i32, data: &[u8], sv: SpreadsheetVersion) -> Vec<Ptg> {
        // java try { return readTokens(size, new LittleEndianByteArrayInputStream(data), false, sv); }
        // java catch (RuntimeException var4) { return EMPTY_PTG_ARRAY; }
        Self::readTokens__input(size, data, false, sv).map(TokenReadResult::into_vec).unwrap_or_default()
    }

    // java public static Ptg[] readTokens(int size, LittleEndianInput in, boolean externalName, SpreadsheetVersion sv)
    pub fn readTokens__stream(size: i32, data: &[u8], externalName: bool, sv: SpreadsheetVersion) -> Vec<Ptg> {
        Self::readTokens__input(size, data, externalName, sv).map(TokenReadResult::into_vec)
            .unwrap_or_else(|e|match e{TokenReadFailure::Runtime=>panic!("RuntimeException: Ptg array size mismatch"),TokenReadFailure::NegativeArraySize=>panic!("NegativeArraySizeException")})
    }
    // Native Java-array transport of the same original stream overload.
    pub fn readTokens__stream__array(size:i32,data:&[u8],externalName:bool,sv:SpreadsheetVersion)->PtgArray{
        Self::readTokens__input(size,data,externalName,sv).map(TokenReadResult::into_array)
            .unwrap_or_else(|e|match e{TokenReadFailure::Runtime=>panic!("RuntimeException: Ptg array size mismatch"),TokenReadFailure::NegativeArraySize=>panic!("NegativeArraySizeException")})
    }
    // Native Java-array transport of the same original byte[] overload.
    pub fn readTokens__array(size:i32,data:&[u8],sv:SpreadsheetVersion)->PtgArray{
        Self::readTokens__input(size,data,false,sv).map(TokenReadResult::into_array).unwrap_or_else(|_|Self::EMPTY_PTG_ARRAY())
    }

    // java public static Ptg[] readTokens(int size, LittleEndianInput in, boolean externalName, SpreadsheetVersion sv)
    fn readTokens__input(size: i32, data: &[u8], externalName: bool, sv: SpreadsheetVersion) -> Result<TokenReadResult, TokenReadFailure> {
        // java int length = 4 + size / 2;
        let length = 4 + size / 2;
        // java if (length < 0) return new Ptg[0];
        if length < 0 { return Ok(TokenReadResult::FreshEmpty); }
        // java List<Ptg> temp = new ArrayList(length);
        let mut temp = Vec::with_capacity(length as usize);
        // java int pos = 0; boolean hasMem = false;
        let mut pos = 0;
        let mut input = TokenInput { data, offset: 0 };
        let mut has_mem = false;
        // java while (pos < size) { Ptg ptg = createPtg(in, externalName, sv); ... }
        while pos < size {
            // java catch (RuntimeException var10) { pos = size; }
            let ptg = match Self::createPtg(&mut input, externalName, sv) {
                Ok(ptg) => ptg,
                Err(()) => { pos = size; break; }
            };
            // java if (ptg.getSid() == 32 || ptg.getSid() == 38) hasMem = true;
            if ptg.getSid() == 32 || ptg.getSid() == 38 { has_mem = true; }
            // java pos += ptg.getSize(); temp.add(ptg);
            pos = pos.wrapping_add(ptg.getSize());
            temp.push(ptg);
        }
        // java if (pos != size) throw new RuntimeException("Ptg array size mismatch");
        if pos != size { return Err(TokenReadFailure::Runtime); }
        // java if (hasMem) finishReading(in) for MemAreaPtg.Initial and ArrayPtg.Initial.
        if has_mem {
            for ptg in &mut temp {
                if let PtgKind::MemAreaInitial { reserved, subex_len } = ptg.kind {
                    // java if (subex_len == 0 || in.available() <= 0) areas = null;
                    let areas = if subex_len == 0 || input.available() == 0 {
                        None
                    } else {
                        // java int areaCount = in.readUShort(); byte[] areas = new byte[areaCount * 8]; in.readFully(areas, 0, areaCount * 8);
                        let area_count = input.read_u16()? as usize;
                        Some(input.read_bytes(area_count.checked_mul(8).ok_or(())?)?)
                    };
                    ptg.kind = PtgKind::MemArea { reserved, subex_len, areas };
                    // java MemAreaPtg.Initial.finishReading 未调用 setClass；完成对象保持默认 CLASS_REF。
                    ptg.class = 0;
                }
                if let PtgKind::ArrayInitial { reserved0, reserved1, reserved2 } = ptg.kind {
                    // java ArrayPtg.Initial.finishReading: readUByte columns; readShort rows; increment both.
                    let columns = input.read_u8()? as u16 + 1;
                    let rows = input.read_i16()?.wrapping_add(1) as i32;
                    // java totalCount = nRows * nColumns; ConstantValueParser.parse(in, totalCount).
                    let count = (rows as i32).checked_mul(columns as i32).ok_or(())?;
                    if count < 0 { return Err(TokenReadFailure::NegativeArraySize); }
                    let values = ConstantValueParser::parse(input.data, &mut input.offset, count as usize)?;
                    ptg.kind = PtgKind::Array { reserved0, reserved1, reserved2, columns, rows, values:Some(values) };
                    // java ArrayPtg.Initial.finishReading calls result.setClass(this.getPtgClass()).
                }
            }
        }
        // java return toPtgArray(temp);
        Ok(TokenReadResult::Parsed(temp))
    }

    // java protected static Ptg createPtg(LittleEndianInput in, boolean externalName, SpreadsheetVersion sv)
    fn createPtg(input: &mut TokenInput<'_>, externalName: bool, sv: SpreadsheetVersion) -> Result<Ptg, ()> {
        // java byte id = in.readByte();
        let id = input.read_byte();
        // java if (id < 32) return createBasePtg(id, in, sv);
        if (id as i8) < 32 { return Self::createBasePtg(id, input, sv); }
        // java Ptg retval = createClassifiedPtg(id, in, externalName, sv);
        let mut retval = Self::createClassifiedPtg(id, input, externalName, sv)?;
        // java if (id >= 96) retval.setClass((byte)64); else if (id >= 64) ... 32; else ... 0;
        retval.setClass(if id >= 96 { 64 } else if id >= 64 { 32 } else { 0 });
        Ok(retval)
    }

    // java protected static Ptg createBasePtg(byte id, LittleEndianInput in, SpreadsheetVersion sv)
    fn createBasePtg(id: u8, input: &mut TokenInput<'_>, sv: SpreadsheetVersion) -> Result<Ptg, ()> {
        // java switch (id) { case 1: return ExpPtg.createExpPtg(sv, in); ... }
        let kind = match id {
            // java case 0: return new UnknownPtg(id);
            0 => PtgKind::Unknown,
            1 => PtgKind::Exp { row: if sv == SpreadsheetVersion::EXCEL97 { input.read_u16()? as i32 } else { input.read_i32()? }, col: input.read_u16()?, wide: sv != SpreadsheetVersion::EXCEL97 },
            // java case 2: return TblPtg.createTblPtg(sv, in);
            2 => PtgKind::Tbl { row: if sv == SpreadsheetVersion::EXCEL97 { input.read_u16()? as i32 } else { input.read_i32()? }, col: input.read_u16()?, wide: sv != SpreadsheetVersion::EXCEL97 },
            // java case 3/4/5/6/7/8: Add/Subtract/Multiply/Divide/Power/ConcatPtg.instance;
            3 => PtgKind::Add, 4 => PtgKind::Subtract, 5 => PtgKind::Multiply,
            6 => PtgKind::Divide, 7 => PtgKind::Power, 8 => PtgKind::Concat,
            // java case 9/10/11/12/13/14: comparison Ptg.instance;
            9 => PtgKind::LessThan, 10 => PtgKind::LessEqual, 11 => PtgKind::Equal,
            12 => PtgKind::GreaterEqual, 13 => PtgKind::GreaterThan, 14 => PtgKind::NotEqual,
            // java case 15/16/17: Intersection/Union/RangePtg.instance;
            15 => PtgKind::Intersection, 16 => PtgKind::Union, 17 => PtgKind::Range,
            // java case 18/19/20/21: UnaryPlus/UnaryMinus/Percent/ParenthesisPtg.instance;
            18 => PtgKind::UnaryPlus, 19 => PtgKind::UnaryMinus,
            20 => PtgKind::Percent, 21 => PtgKind::Paren,
            // java case 22: MissingArgPtg.instance;
            22 => PtgKind::MissingArg,
            // java case 23: return StringPtg.read(in);
            23 => {
                let count = input.read_u8()? as usize;
                let is_16bit = input.read_byte() & 1 != 0;
                // java StringUtil.readUnicodeLE(in,nChars) / readCompressedUnicode(in,nChars): readFully is atomic on buffer overrun.
                let encoded=input.read_bytes(count*if is_16bit{2}else{1})?;
                let units:Vec<u16>=if is_16bit{encoded.chunks_exact(2).map(|b|u16::from_le_bytes([b[0],b[1]])).collect()}else{encoded.iter().map(|b|*b as u16).collect()};
                // java StringPtg.read stores both the decoded String and the original _is16bitUnicode.
                match String::from_utf16(&units) {
                    Ok(text) if multibyte(&text) == is_16bit => PtgKind::Str(text),
                    _ => PtgKind::StrRaw { units, wide: is_16bit },
                }
            }
            // java case 24: new SxNamePtg(in), eptg byte then sxIndex int。
            24 => {let eptg=input.read_byte();let index=input.read_i32()?;PtgKind::SxName { index,eptg,field_name:None,item_name:None }},
            // java case 25: return AttrPtg.read(in);
            25 => {
                let options = input.read_byte();
                let data = input.read_i16()?;
                // java if((options&4)==0){return (options&16)!=0?SUM:new AttrPtg(options,data,null,-1);}
                if options&4==0{if options&16!=0{PtgKind::Attr{options:16,data:0,jump_table:None,choose_offset:-1}}else{PtgKind::Attr{options,data,jump_table:None,choose_offset:-1}}}
                else{
                    // java jumpTable=new int[data];for(int i=0;i<data;++i){jumpTable[i]=in.readUShort();}chooseFuncOffset=in.readUShort();return new AttrPtg(options,data,jumpTable,chooseFuncOffset);
                    if data<0{return Err(());}let mut jump_table=Vec::with_capacity(data as usize);for _ in 0..data{jump_table.push(input.read_u16()?as i32);}let choose_offset=input.read_u16()?as i32;PtgKind::Attr{options,data,jump_table:Some(jump_table),choose_offset}
                }
            }
            // java case 28/29/30/31: Err/Bool/Int/NumberPtg.read(in);
            28 => {let code=input.read_byte();if ![0,7,15,23,29,36,42].contains(&code){return Err(());}PtgKind::Err(code)},
            29 => PtgKind::Bool(input.read_byte() == 1),
            30 => PtgKind::Int(input.read_u16()?),
            31 => PtgKind::Number(f64::from_le_bytes(input.read_array::<8>()?)),
            // java default: throw new RuntimeException("Unexpected base token id");
            _ => return Err(()),
        };
        Ok(Ptg::base(kind))
    }

    // java protected static Ptg createClassifiedPtg(byte id, LittleEndianInput in, boolean externalName, SpreadsheetVersion sv)
    fn createClassifiedPtg(id: u8, input: &mut TokenInput<'_>, externalName: bool, sv: SpreadsheetVersion) -> Result<Ptg, ()> {
        // java int baseId = id & 31 | 32;
        let base_id = id & 31 | 32;
        let wide = sv != SpreadsheetVersion::EXCEL97;
        // java switch (baseId)
        let kind = match base_id {
            // java case 32: return new ArrayPtg.Initial(in);
            32 => PtgKind::ArrayInitial { reserved0: input.read_i32()?, reserved1: input.read_u16()?, reserved2: input.read_u8()? },
            // java case 33: return FuncPtg.create(in);
            33 => PtgKind::Func { meta: function_by_index(input.read_u16()?).ok_or(())? },
            // java case 34: return FuncVarPtg.create(in);
            34 => {
                let nargs = input.read_byte();
                let index = input.read_u16()?;
                // java FunctionMetadataRegistry.getFunctionByIndex 只有 368 项，越界抛异常。
                if index >= 368 { return Err(()); }
                if let Some(meta) = function_by_index(index) { PtgKind::FuncVar { meta, nargs } }
                else { PtgKind::FuncVarExt { index, nargs, return_class: 32 } }
            }
            // java case 35: return new NamePtg(in);
            35 => { let label = input.read_i16()?; input.read_i16()?; PtgKind::Name { index: label as i32 - 1 } }
            // java case 36/44: return new Ref03/10Ptg(in) or Ref03/10NPtg(in);
            36 | 44 => PtgKind::Ref { row: if wide { input.read_i32()? } else { input.read_u16()? as i32 }, col_raw: input.read_u16()?, wide, relative: base_id == 44 },
            // java case 37/45: return new Area03/10Ptg(in) or Area03/10NPtg(in);
            37 | 45 => PtgKind::Area { first_row: if wide { input.read_i32()? } else { input.read_u16()? as i32 }, last_row: if wide { input.read_i32()? } else { input.read_u16()? as i32 }, first_col_raw: input.read_u16()?, last_col_raw: input.read_u16()?, wide, relative: base_id == 45 },
            // java case 38: return new MemAreaPtg.Initial(in);
            38 => PtgKind::MemAreaInitial { reserved: input.read_i32()?, subex_len: input.read_i16()? },
            // java case 39: return new MemErrPtg(in);
            39 => PtgKind::MemErr { reserved: input.read_i32()?, subex_len: input.read_i16()? },
            // java case 41: return new MemFuncPtg(in);
            41 => PtgKind::MemFunc { subex_len: input.read_u16()? },
            // java case 57: return new NameXPtg(in);
            57 => { let sheet_ref = input.read_i16()? as i32; let name_number = input.read_i16()? as i32; input.read_i16()?; PtgKind::NameX { sheet_ref, name_number } }
            // java case 58 externalName: Ref3D03ExtPtg / Ref3D10ExtPtg。
            58 if externalName => PtgKind::Ref3DExt { sheet_index1: input.read_i16()?, sheet_index2: input.read_i16()?, row: if wide { input.read_i32()? } else { input.read_u16()? as i32 }, col_raw: input.read_u16()?, wide },
            // java case 59 externalName: Area3D03ExtPtg / Area3D10ExtPtg。
            59 if externalName => PtgKind::Area3DExt { sheet_index1: input.read_i16()?, sheet_index2: input.read_i16()?, first_row: if wide { input.read_i32()? } else { input.read_u16()? as i32 }, last_row: if wide { input.read_i32()? } else { input.read_u16()? as i32 }, first_col_raw: input.read_u16()?, last_col_raw: input.read_u16()?, wide },
            58 => PtgKind::Ref3D { extern_index: input.read_i16()? as i32, row: if wide { input.read_i32()? } else { input.read_u16()? as i32 }, col_raw: input.read_u16()?, wide },
            59 => PtgKind::Area3D { extern_index: input.read_i16()? as i32, first_row: if wide { input.read_i32()? } else { input.read_u16()? as i32 }, last_row: if wide { input.read_i32()? } else { input.read_u16()? as i32 }, first_col_raw: input.read_u16()?, last_col_raw: input.read_u16()?, wide },
            // java case 60/61: DeletedRef3DPtg/DeletedArea3DPtg 保留写回 unused 字段。
            60 => PtgKind::DeletedRef3D { extern_index: input.read_u16()?, unused: input.read_i32()? },
            61 => PtgKind::DeletedArea3D { extern_index: input.read_u16()?, unused1: input.read_i32()?, unused2: input.read_i32()? },
            // java case 42/43: return new RefErrorPtg(in)/AreaErrPtg(in);
            42 => { input.read_i32()?; PtgKind::RefError },
            43 => { input.read_i32()?; input.read_i32()?; PtgKind::AreaErr },
            // java default: throw new UnsupportedOperationException(" Unknown Ptg in Formula");
            _ => return Err(()),
        };
        Ok(Ptg::base(kind))
    }

    pub fn base(kind: PtgKind) -> Self {
        Self { kind, class: 0,java_identity:Self::newIdentity() }
    }

    // java public abstract byte getSid()
    pub fn getSid(&self) -> u8 {
        match &self.kind {
            PtgKind::Unknown => 0,
            PtgKind::Tbl { .. } => 2,
            PtgKind::Int(_) => 30,
            PtgKind::Number(_) => 31,
            PtgKind::Str(_) | PtgKind::StrRaw { .. } => 23,
            PtgKind::Bool(_) => 29,
            PtgKind::Err(_) => 28,
            PtgKind::Ref { relative, .. } => {
                if *relative {
                    44
                } else {
                    36
                }
            }
            PtgKind::Area { relative, .. } => {
                if *relative {
                    45
                } else {
                    37
                }
            }
            PtgKind::Ref3D { .. } => 58,
            PtgKind::Area3D { .. } => 59,
            PtgKind::Ref3DExt { .. } => 58,
            PtgKind::Area3DExt { .. } => 59,
            PtgKind::Add => 3,
            PtgKind::Subtract => 4,
            PtgKind::Multiply => 5,
            PtgKind::Divide => 6,
            PtgKind::Equal => 11,
            PtgKind::LessThan => 9,
            PtgKind::LessEqual => 10,
            PtgKind::GreaterEqual => 12,
            PtgKind::GreaterThan => 13,
            PtgKind::NotEqual => 14,
            PtgKind::Power => 7,
            PtgKind::UnaryPlus => 18,
            PtgKind::UnaryMinus => 19,
            PtgKind::Percent => 20,
            PtgKind::Concat => 8,
            PtgKind::Intersection => 15,
            PtgKind::Union => 16,
            PtgKind::Range => 17,
            PtgKind::Paren => 21,
            PtgKind::MissingArg => 22,
            PtgKind::SxName { .. } => 24,
            PtgKind::Attr { .. } => 25,
            PtgKind::Func { .. } => 33,
            PtgKind::FuncVar { .. } | PtgKind::FuncVarExt { .. } => 34,
            PtgKind::MemErr { .. } => 39,
            PtgKind::MemFunc { .. } => 41,
            PtgKind::MemAreaInitial { .. } | PtgKind::MemArea { .. } => 38,
            PtgKind::ArrayInitial { .. } | PtgKind::Array { .. } => 32,
            PtgKind::Name { .. } => 35,
            PtgKind::NameX { .. } => 57,
            PtgKind::Exp { .. } => 1,
            PtgKind::RefError => 42,
            PtgKind::AreaErr => 43,
            PtgKind::DeletedRef3D { .. } => 60,
            PtgKind::DeletedArea3D { .. } => 61,
        }
    }

    pub fn is_base_token(&self) -> bool {
        !matches!(
            self.kind,
            PtgKind::Ref { .. }
                | PtgKind::Area { .. }
                | PtgKind::Ref3D { .. }
                | PtgKind::Area3D { .. }
                | PtgKind::Ref3DExt { .. }
                | PtgKind::Area3DExt { .. }
                | PtgKind::FuncVar { .. }
                | PtgKind::FuncVarExt { .. }
                | PtgKind::Func { .. }
                | PtgKind::MemErr { .. }
                | PtgKind::MemFunc { .. }
                | PtgKind::MemAreaInitial { .. }
                | PtgKind::MemArea { .. }
                | PtgKind::ArrayInitial { .. }
                | PtgKind::Array { .. }
                | PtgKind::Name { .. }
                | PtgKind::NameX { .. }
                | PtgKind::RefError
                | PtgKind::AreaErr
                | PtgKind::DeletedRef3D { .. }
                | PtgKind::DeletedArea3D { .. }
        )
    }

    pub fn getDefaultOperandClass(&self) -> u8 {
        match &self.kind {
            PtgKind::FuncVar { meta, .. } => meta.getReturnClassCode() as u8,
            // java FuncPtg.getDefaultOperandClass() 继承 AbstractFunctionPtg。
            PtgKind::Func { meta } => meta.getReturnClassCode() as u8,
            PtgKind::FuncVarExt { return_class, .. } => *return_class,
            // java MemErrPtg.getDefaultOperandClass() = CLASS_VALUE;
            PtgKind::MemErr { .. } => 32,
            // java MemAreaPtg.getDefaultOperandClass() = CLASS_VALUE;
            PtgKind::MemArea { .. } => 32,
            // java MemAreaPtg.Initial.getDefaultOperandClass() throws IllegalStateException.
            PtgKind::MemAreaInitial { .. } => panic!("partially initialised tArray"),
            // java ArrayPtg.getDefaultOperandClass() = CLASS_ARRAY.
            PtgKind::Array { .. } => 64,
            // java ArrayPtg.Initial.getDefaultOperandClass() throws IllegalStateException.
            PtgKind::ArrayInitial { .. } => panic!("partially initialised tArray"),
            // java MemFuncPtg.getDefaultOperandClass() = CLASS_REF;
            PtgKind::MemFunc { .. } => 0,
            PtgKind::Name { .. } => 0,
            PtgKind::NameX { .. } => 32,
            PtgKind::Ref { .. }
            | PtgKind::Area { .. }
            | PtgKind::Ref3D { .. }
            | PtgKind::Area3D { .. }
            | PtgKind::Ref3DExt { .. }
            | PtgKind::Area3DExt { .. }
            | PtgKind::RefError
            | PtgKind::AreaErr => 0,
            // java DeletedRef3DPtg/DeletedArea3DPtg.getDefaultOperandClass() = CLASS_REF。
            PtgKind::DeletedRef3D { .. } | PtgKind::DeletedArea3D { .. } => 0,
            _ => 32,
        }
    }

    // java public final void setClass(byte thePtgClass) {
    pub fn setClass(&mut self, class: u8) {
        // java if (this.isBaseToken()) { throw new RuntimeException("setClass should not be called on a base token"); }
        if self.is_base_token() {
            panic!("RuntimeException: setClass should not be called on a base token");
        }else{
            // java this.ptgClass = thePtgClass;
            self.class = class;
        }
    }

    // java public abstract int getSize()
    pub fn getSize(&self) -> i32 {
        match &self.kind {
            PtgKind::Int(_) => 3,
            PtgKind::Number(_) => 9,
            PtgKind::Bool(_) | PtgKind::Err(_) => 2,
            PtgKind::Str(text) => 3 + (text.encode_utf16().count() as i32) * if multibyte(text) { 2 } else { 1 },
            PtgKind::StrRaw { units, wide } => 3 + units.len() as i32 * if *wide { 2 } else { 1 },
            PtgKind::Ref { wide, .. } | PtgKind::Exp { wide, .. } | PtgKind::Tbl { wide, .. } => {
                if *wide {
                    7
                } else {
                    5
                }
            }
            PtgKind::Area { wide, .. } => {
                if *wide {
                    13
                } else {
                    9
                }
            }
            PtgKind::Ref3D { wide, .. } => {
                if *wide {
                    9
                } else {
                    7
                }
            }
            PtgKind::Area3D { wide, .. } => {
                if *wide {
                    15
                } else {
                    11
                }
            }
            PtgKind::Ref3DExt { wide, .. } => if *wide { 11 } else { 9 },
            PtgKind::Area3DExt { wide, .. } => if *wide { 17 } else { 13 },
            // java AttrPtg.getSize(): this._jumpTable!=null?4+(this._jumpTable.length+1)*2:4;
            PtgKind::Attr { ref jump_table,.. } => jump_table.as_ref().map_or(4,|jt|4i32.wrapping_add((jt.len()as i32).wrapping_add(1).wrapping_mul(2))),
            PtgKind::FuncVar { .. } | PtgKind::FuncVarExt { .. } => 4,
            PtgKind::Name { .. } => 5,
            PtgKind::Func { .. } | PtgKind::MemFunc { .. } => 3,
            PtgKind::MemErr { .. } => 7,
            // java MemAreaPtg.Initial.getSize() = 7; MemAreaPtg.getSize() = 9 + area bytes.
            PtgKind::MemAreaInitial { .. } => 7,
            PtgKind::MemArea { areas, .. } => 9 + areas.as_ref().map_or(0, |a| a.len() as i32),
            // java ArrayPtg.Initial.getSize() = 8; ArrayPtg.getSize() = 11 + ConstantValueParser.getEncodedSize(values).
            PtgKind::ArrayInitial { .. } => 8,
            PtgKind::Array { values, .. } => 11 + ConstantValueParser::getEncodedSize(values.as_ref().expect("NullPointerException")) as i32,
            PtgKind::NameX { .. } => 7,
            PtgKind::RefError => 5,
            PtgKind::AreaErr => 9,
            PtgKind::SxName { .. } => 6,
            PtgKind::DeletedRef3D { .. } => 7,
            PtgKind::DeletedArea3D { .. } => 11,
            PtgKind::Unknown | PtgKind::Intersection | PtgKind::Union | PtgKind::Range | PtgKind::MissingArg => 1,
            PtgKind::Add
            | PtgKind::Subtract
            | PtgKind::Multiply
            | PtgKind::Divide
            | PtgKind::Equal
            | PtgKind::LessThan
            | PtgKind::LessEqual
            | PtgKind::GreaterThan
            | PtgKind::GreaterEqual
            | PtgKind::NotEqual
            | PtgKind::Power
            | PtgKind::Percent
            | PtgKind::UnaryPlus
            | PtgKind::UnaryMinus
            | PtgKind::Concat
            | PtgKind::Paren => 1,
        }
    }

    pub fn is_sum_attr(&self) -> bool {
        matches!(self.kind, PtgKind::Attr { options, .. } if options & 16 != 0)
    }

    pub fn func_name(&self) -> Option<&str> {
        match &self.kind {
            PtgKind::FuncVar { meta, .. } | PtgKind::Func { meta } => Some(meta.getName()),
            _ => None,
        }
    }

    // java public abstract void write(LittleEndianOutput out)
    pub fn write(&self, out: &mut Vec<u8>) {
        let sid = self.getSid().wrapping_add(if self.is_base_token() { 0 } else { self.class });
        match &self.kind {
            PtgKind::Int(value) => {
                out.push(sid);
                out.extend_from_slice(&value.to_le_bytes());
            }
            PtgKind::Bool(value) => {
                out.push(sid);
                out.push(if *value { 1 } else { 0 });
            }
            PtgKind::Err(code) => {
                out.push(sid);
                out.push(*code);
            }
            PtgKind::Number(value) => {
                out.push(sid);
                // java LittleEndianOutput.writeDouble uses Double.doubleToLongBits, canonicalizing every NaN payload.
                let bits=if value.is_nan(){0x7ff8000000000000}else{value.to_bits()};out.extend_from_slice(&bits.to_le_bytes());
            }
            PtgKind::Str(text) => {
                out.push(sid);
                // java String.length() counts UTF-16 code units, including surrogate pairs.
                let units: Vec<u16> = text.encode_utf16().collect();
                out.push(units.len() as u8);
                let wide = multibyte(text);
                out.push(if wide { 1 } else { 0 });
                if wide {
                    for unit in units {
                        out.extend_from_slice(&unit.to_le_bytes());
                    }
                } else {
                    for unit in units {
                        out.push(unit as u8);
                    }
                }
            }
            PtgKind::StrRaw { units, wide } => {
                // java StringPtg.read/write preserves the original Unicode flag.
                out.push(sid);
                out.push(units.len() as u8);
                out.push(if *wide { 1 } else { 0 });
                for unit in units {
                    if *wide { out.extend_from_slice(&unit.to_le_bytes()); }
                    else { out.push(*unit as u8); }
                }
            }
            PtgKind::Ref { row, col_raw, wide, .. } | PtgKind::Exp { row, col: col_raw, wide } | PtgKind::Tbl { row, col: col_raw, wide } => {
                out.push(sid);
                if *wide {
                    out.extend_from_slice(&row.to_le_bytes());
                } else {
                    out.extend_from_slice(&(*row as u16).to_le_bytes());
                }
                out.extend_from_slice(&col_raw.to_le_bytes());
            }
            PtgKind::Area {
                first_row,
                last_row,
                first_col_raw,
                last_col_raw,
                wide,
                ..
            } => {
                out.push(sid);
                if *wide {
                    out.extend_from_slice(&first_row.to_le_bytes());
                    out.extend_from_slice(&last_row.to_le_bytes());
                } else {
                    out.extend_from_slice(&(*first_row as u16).to_le_bytes());
                    out.extend_from_slice(&(*last_row as u16).to_le_bytes());
                }
                out.extend_from_slice(&first_col_raw.to_le_bytes());
                out.extend_from_slice(&last_col_raw.to_le_bytes());
            }
            PtgKind::Ref3D {
                extern_index,
                row,
                col_raw,
                wide,
            } => {
                out.push(sid);
                out.extend_from_slice(&(*extern_index as i16).to_le_bytes());
                if *wide {
                    out.extend_from_slice(&row.to_le_bytes());
                } else {
                    out.extend_from_slice(&(*row as u16).to_le_bytes());
                }
                out.extend_from_slice(&col_raw.to_le_bytes());
            }
            // java Ref3DExtPtg.write: sid, sheetIndex1, sheetIndex2, coordinates。
            PtgKind::Ref3DExt { sheet_index1, sheet_index2, row, col_raw, wide } => {
                out.push(sid);
                out.extend_from_slice(&sheet_index1.to_le_bytes());
                out.extend_from_slice(&sheet_index2.to_le_bytes());
                if *wide { out.extend_from_slice(&row.to_le_bytes()); }
                else { out.extend_from_slice(&(*row as u16).to_le_bytes()); }
                out.extend_from_slice(&col_raw.to_le_bytes());
            }
            PtgKind::Area3D {
                extern_index,
                first_row,
                last_row,
                first_col_raw,
                last_col_raw,
                wide,
            } => {
                out.push(sid);
                out.extend_from_slice(&(*extern_index as i16).to_le_bytes());
                if *wide {
                    out.extend_from_slice(&first_row.to_le_bytes());
                    out.extend_from_slice(&last_row.to_le_bytes());
                } else {
                    out.extend_from_slice(&(*first_row as u16).to_le_bytes());
                    out.extend_from_slice(&(*last_row as u16).to_le_bytes());
                }
                out.extend_from_slice(&first_col_raw.to_le_bytes());
                out.extend_from_slice(&last_col_raw.to_le_bytes());
            }
            // java Area3DExtPtg.write: sid, 两个 sheet index, coordinates。
            PtgKind::Area3DExt { sheet_index1, sheet_index2, first_row, last_row, first_col_raw, last_col_raw, wide } => {
                out.push(sid);
                out.extend_from_slice(&sheet_index1.to_le_bytes());
                out.extend_from_slice(&sheet_index2.to_le_bytes());
                if *wide {
                    out.extend_from_slice(&first_row.to_le_bytes());
                    out.extend_from_slice(&last_row.to_le_bytes());
                } else {
                    out.extend_from_slice(&(*first_row as u16).to_le_bytes());
                    out.extend_from_slice(&(*last_row as u16).to_le_bytes());
                }
                out.extend_from_slice(&first_col_raw.to_le_bytes());
                out.extend_from_slice(&last_col_raw.to_le_bytes());
            }
            PtgKind::Attr { options, data, jump_table, choose_offset } => {
                out.push(sid);
                out.push(*options);
                out.extend_from_slice(&data.to_le_bytes());
                // java int[] jt=this._jumpTable;if(jt!=null){for(int i=0;i<jt.length;++i){out.writeShort(jt[i]);}out.writeShort(this._chooseFuncOffset);}
                if let Some(jt)=jump_table{for v in jt{out.extend_from_slice(&(*v as u16).to_le_bytes());}out.extend_from_slice(&(*choose_offset as u16).to_le_bytes());}
            }
            PtgKind::FuncVar { meta, nargs } => {
                out.push(sid);
                out.push(*nargs);
                out.extend_from_slice(&(meta.getIndex() as u16).to_le_bytes());
            }
            // java FuncPtg.write(out): writeByte(33 + getPtgClass()); writeShort(getFunctionIndex());
            PtgKind::Func { meta } => {
                out.push(sid);
                out.extend_from_slice(&(meta.getIndex() as u16).to_le_bytes());
            }
            // java MemErrPtg.write(out): writeByte(39 + getPtgClass()); writeInt(reserved); writeShort(subexLen);
            PtgKind::MemErr { reserved, subex_len } => {
                out.push(sid);
                out.extend_from_slice(&reserved.to_le_bytes());
                out.extend_from_slice(&subex_len.to_le_bytes());
            }
            // java MemFuncPtg.write(out): writeByte(41 + getPtgClass()); writeShort(subExprLen);
            PtgKind::MemFunc { subex_len } => {
                out.push(sid);
                out.extend_from_slice(&subex_len.to_le_bytes());
            }
            // java MemAreaPtg.Initial.write(out) throws IllegalStateException.
            PtgKind::MemAreaInitial { .. } => panic!("partially initialised tArray"),
            // java MemAreaPtg.write(out) 写 token 区；writeTokenValueBytes 另写尾部。
            PtgKind::MemArea { reserved, subex_len, .. } => {
                out.push(sid);
                out.extend_from_slice(&reserved.to_le_bytes());
                out.extend_from_slice(&subex_len.to_le_bytes());
            }
            // java ArrayPtg.Initial.write(out) throws IllegalStateException.
            PtgKind::ArrayInitial { .. } => panic!("partially initialised tArray"),
            // java ArrayPtg.write(out) writes only the reserved plain token fields.
            PtgKind::Array { reserved0, reserved1, reserved2, .. } => {
                out.push(sid);
                out.extend_from_slice(&reserved0.to_le_bytes());
                out.extend_from_slice(&reserved1.to_le_bytes());
                out.push(*reserved2);
            }
            PtgKind::FuncVarExt { index, nargs, .. } => {
                out.push(sid);
                out.push(*nargs);
                out.extend_from_slice(&index.to_le_bytes());
            }
            PtgKind::Name { index } => {
                // java writeShort(1 + nameIndex); writeShort(0)
                out.push(sid);
                out.extend_from_slice(&(1 + *index as i16).to_le_bytes());
                out.extend_from_slice(&0i16.to_le_bytes());
            }
            PtgKind::NameX {
                sheet_ref,
                name_number,
            } => {
                out.push(sid);
                out.extend_from_slice(&(*sheet_ref as i16).to_le_bytes());
                out.extend_from_slice(&(*name_number as i16).to_le_bytes());
                out.extend_from_slice(&0i16.to_le_bytes());
            }
            // java SxNamePtg.write: sid, eptg, sxIndex。
            PtgKind::SxName { eptg, index, .. } => {
                out.push(sid);
                out.push(*eptg);
                out.extend_from_slice(&index.to_le_bytes());
            }
            // java DeletedRef3DPtg/DeletedArea3DPtg.write 保留读入的 unused 字段。
            PtgKind::DeletedRef3D { extern_index, unused } => {
                out.push(sid);
                out.extend_from_slice(&extern_index.to_le_bytes());
                out.extend_from_slice(&unused.to_le_bytes());
            }
            PtgKind::DeletedArea3D { extern_index, unused1, unused2 } => {
                out.push(sid);
                out.extend_from_slice(&extern_index.to_le_bytes());
                out.extend_from_slice(&unused1.to_le_bytes());
                out.extend_from_slice(&unused2.to_le_bytes());
            }
            PtgKind::Unknown
            | PtgKind::Intersection
            | PtgKind::Union
            | PtgKind::Range
            | PtgKind::MissingArg
            | PtgKind::Add
            | PtgKind::Subtract
            | PtgKind::Multiply
            | PtgKind::Divide
            | PtgKind::Equal
            | PtgKind::LessThan
            | PtgKind::LessEqual
            | PtgKind::GreaterThan
            | PtgKind::GreaterEqual
            | PtgKind::NotEqual
            | PtgKind::Power
            | PtgKind::Percent
            | PtgKind::UnaryPlus
            | PtgKind::UnaryMinus
            | PtgKind::Concat
            | PtgKind::Paren => out.push(sid),
            PtgKind::RefError => {
                out.push(sid);
                out.extend_from_slice(&0i32.to_le_bytes());
            }
            PtgKind::AreaErr => {
                out.push(sid);
                out.extend_from_slice(&0i32.to_le_bytes());
                out.extend_from_slice(&0i32.to_le_bytes());
            }
        }
    }
}

// rust: byte-slice adapter for Java LittleEndianByteArrayInputStream.
struct TokenInput<'a> { data: &'a [u8], offset: usize }
impl TokenInput<'_> {
    fn available(&self) -> usize { self.data.len().saturating_sub(self.offset) }
    fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>, ()> {
        let end = self.offset.checked_add(len).ok_or(())?;
        let bytes = self.data.get(self.offset..end).ok_or(())?.to_vec();
        self.offset = end;
        Ok(bytes)
    }
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N], ()> {
        let end = self.offset.checked_add(N).ok_or(())?;
        let bytes: [u8; N] = self.data.get(self.offset..end).ok_or(())?.try_into().map_err(|_| ())?;
        self.offset = end;
        Ok(bytes)
    }
    fn read_u8(&mut self) -> Result<u8, ()> { Ok(self.read_array::<1>()?[0]) }
    // java LittleEndianByteArrayInputStream.readByte: return 1>available()?0:_buf[_readIndex++];
    fn read_byte(&mut self)->u8{if self.available()<1{0}else{let b=self.data[self.offset];self.offset+=1;b}}
    fn read_u16(&mut self) -> Result<u16, ()> { Ok(u16::from_le_bytes(self.read_array::<2>()?)) }
    fn read_i16(&mut self) -> Result<i16, ()> { Ok(i16::from_le_bytes(self.read_array::<2>()?)) }
    fn read_i32(&mut self) -> Result<i32, ()> { Ok(i32::from_le_bytes(self.read_array::<4>()?)) }
}

fn multibyte(text: &str) -> bool {
    text.chars().any(|ch| ch as u32 > 0xff)
}

pub fn int_ptg(value: i32) -> Ptg {
    Ptg::base(PtgKind::Int(value as u16))
}

pub fn number_ptg(value: f64) -> Ptg {
    Ptg::base(PtgKind::Number(value))
}

pub fn string_ptg(value: String) -> Ptg {
    // java StringPtg.valueOf rejects values longer than 255 UTF-16 code units.
    assert!(value.encode_utf16().count() <= 255, "String literals in formulas can't be bigger than 255 characters ASCII");
    Ptg::base(PtgKind::Str(value))
}

pub fn attr_if(dist: i32) -> Ptg {
    Ptg::base(PtgKind::Attr {
        options: 2,
        data: dist as i16,
        jump_table:None,choose_offset:-1,
    })
}

pub fn attr_skip(dist: i32) -> Ptg {
    Ptg::base(PtgKind::Attr {
        options: 8,
        data: dist as i16,
        jump_table:None,choose_offset:-1,
    })
}

pub fn attr_sum() -> Ptg {
    Ptg::base(PtgKind::Attr {
        options: 16,
        data: 0,
        jump_table:None,choose_offset:-1,
    })
}

pub fn func_var(meta: &'static FuncMeta, nargs: i32) -> Ptg {
    Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
        kind: PtgKind::FuncVar {
            meta,
            nargs: nargs as u8,
        },
        class: 0,
    }
}

// java FuncPtg.create(int functionIndex): getFunctionByIndex 并构造固定参数 token。
pub fn func_ptg(meta: &'static FuncMeta) -> Ptg {
    Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(), kind: PtgKind::Func { meta }, class: 0 }
}

// java FuncVarPtg.create(name, numArgs) 在函数表里找不到时 index = 255
pub fn func_var_ext(nargs: i32) -> Ptg {
    Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
        kind: PtgKind::FuncVarExt {
            index: 255,
            nargs: nargs as u8,
            return_class: 32,
        },
        class: 0,
    }
}

pub fn name_ptg(index: i32) -> Ptg {
    Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
        kind: PtgKind::Name { index },
        class: 0,
    }
}

pub fn name_x_ptg(sheet_ref: i32, name_index: i32) -> Ptg {
    Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
        kind: PtgKind::NameX {
            sheet_ref,
            name_number: name_index + 1,
        },
        class: 0,
    }
}

pub fn err_ptg(code: u8) -> Ptg {
    Ptg::base(PtgKind::Err(code))
}

pub(crate) fn pack_col(col: i32, row_rel: bool, col_rel: bool) -> u16 {
    let mut raw = (col & 0x3fff) as u16;
    if row_rel {
        raw |= 0x8000;
    }
    if col_rel {
        raw |= 0x4000;
    }
    raw
}

pub fn ref_ptg(row: i32, col: i32, row_rel: bool, col_rel: bool, sv: SpreadsheetVersion) -> Ptg {
    Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
        kind: PtgKind::Ref {
            row,
            col_raw: pack_col(col, row_rel, col_rel),
            wide: sv != SpreadsheetVersion::EXCEL97,
            relative: false,
        },
        class: 0,
    }
}

pub fn area_ptg(
    mut first_row: i32,
    mut last_row: i32,
    mut first_col: i32,
    mut last_col: i32,
    mut first_row_rel: bool,
    mut last_row_rel: bool,
    mut first_col_rel: bool,
    mut last_col_rel: bool,
    sv: SpreadsheetVersion,
) -> Ptg {
    // java AreaPtgBase 在首尾颠倒时交换相对标志。
    if last_row < first_row {
        std::mem::swap(&mut first_row, &mut last_row);
        std::mem::swap(&mut first_row_rel, &mut last_row_rel);
    }
    if last_col < first_col {
        std::mem::swap(&mut first_col, &mut last_col);
        std::mem::swap(&mut first_col_rel, &mut last_col_rel);
    }
    Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
        kind: PtgKind::Area {
            first_row,
            last_row,
            first_col_raw: pack_col(first_col, first_row_rel, first_col_rel),
            last_col_raw: pack_col(last_col, last_row_rel, last_col_rel),
            wide: sv != SpreadsheetVersion::EXCEL97,
            relative: false,
        },
        class: 0,
    }
}

pub fn ref3d_ptg(
    extern_index: i32,
    row: i32,
    col: i32,
    row_rel: bool,
    col_rel: bool,
    sv: SpreadsheetVersion,
) -> Ptg {
    Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
        kind: PtgKind::Ref3D {
            extern_index,
            row,
            col_raw: pack_col(col, row_rel, col_rel),
            wide: sv != SpreadsheetVersion::EXCEL97,
        },
        class: 0,
    }
}

pub fn area3d_ptg(
    extern_index: i32,
    mut first_row: i32,
    mut last_row: i32,
    mut first_col: i32,
    mut last_col: i32,
    mut first_row_rel: bool,
    mut last_row_rel: bool,
    mut first_col_rel: bool,
    mut last_col_rel: bool,
    sv: SpreadsheetVersion,
) -> Ptg {
    if last_row < first_row {
        std::mem::swap(&mut first_row, &mut last_row);
        std::mem::swap(&mut first_row_rel, &mut last_row_rel);
    }
    if last_col < first_col {
        std::mem::swap(&mut first_col, &mut last_col);
        std::mem::swap(&mut first_col_rel, &mut last_col_rel);
    }
    Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
        kind: PtgKind::Area3D {
            extern_index,
            first_row,
            last_row,
            first_col_raw: pack_col(first_col, first_row_rel, first_col_rel),
            last_col_raw: pack_col(last_col, last_row_rel, last_col_rel),
            wide: sv != SpreadsheetVersion::EXCEL97,
        },
        class: 0,
    }
}

pub fn exp_ptg(row: i32, col: i32, sv: SpreadsheetVersion) -> Ptg {
    Ptg::base(PtgKind::Exp {
        row,
        col: col as u16,
        wide: sv != SpreadsheetVersion::EXCEL97,
    })
}

pub(crate) fn col_index(raw: u16) -> i32 {
    (raw & 0x3fff) as i32
}
pub(crate) fn row_rel(raw: u16) -> bool {
    raw & 0x8000 != 0
}
pub(crate) fn col_rel(raw: u16) -> bool {
    raw & 0x4000 != 0
}

pub fn ref_error_ptg() -> Ptg {
    Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
        kind: PtgKind::RefError,
        class: 0,
    }
}

pub fn area_err_ptg() -> Ptg {
    Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
        kind: PtgKind::AreaErr,
        class: 0,
    }
}

// java public static int getEncodedSize(Ptg[] ptgs)
pub fn getEncodedSize(ptgs: &[Ptg]) -> i32 {
    // java int result=0;for(int i=0;i<ptgs.length;++i){if(ptgs[i]!=null){result+=ptgs[i].getSize();}}return result;
    let mut result=0i32;let mut i=0;while i<ptgs.len(){result=result.wrapping_add(ptgs[i].getSize());i+=1;}result
}

// java public static int getEncodedSizeWithoutArrayData(Ptg[] ptgs)
pub fn getEncodedSizeWithoutArrayData(ptgs: &[Ptg]) -> i32 {
    // java MemAreaPtg/ArrayPtg 的 token 区大小与尾部值分开求和。
    let mut result=0i32;let mut i=0;while i<ptgs.len(){let ptg=&ptgs[i];let sid=ptg.getSid();result=result.wrapping_add(if sid==32{8}else if sid==38{7}else{ptg.getSize()});i+=1;}result
}

// java public static int serializePtgs(Ptg[] ptgs, byte[] array, int offset)
pub fn serializePtgs(ptgs: &[Ptg]) -> Vec<u8> {
    let mut out = Vec::new();
    for ptg in ptgs {
        ptg.write(&mut out);
    }
    // java serializePtgs 第二轮按 token 顺序 writeTokenValueBytes。
    for ptg in ptgs {
        if let PtgKind::MemArea { areas, .. } = &ptg.kind {
            let count = areas.as_ref().map_or(0, |bytes| bytes.len() / 8);
            out.extend_from_slice(&(count as u16).to_le_bytes());
            if let Some(bytes) = areas { out.extend_from_slice(&bytes[..count * 8]); }
        }
        if let PtgKind::Array { columns, rows, values, .. } = &ptg.kind {
            // java ArrayPtg.writeTokenValueBytes: columns - 1, rows - 1, ConstantValueParser.encode.
            out.push(columns.wrapping_sub(1) as u8);
            out.extend_from_slice(&(rows.wrapping_sub(1)as i16).to_le_bytes());
            ConstantValueParser::encode(&mut out, values.as_ref().expect("NullPointerException"));
        }
    }
    out
}

// rust: Java FormulaClass enum is passed through the existing formula-type entry point.
fn transform_class(formula_class: usize, default_class: u8, desired: OperandClass) -> u8 {
    let fc = match formula_class {
        0 => FormulaClass::CELL,
        1 => FormulaClass::ARRAY,
        2 => FormulaClass::NAME,
        _ => panic!("IllegalStateException"),
    };
    OperandClass::transform(fc, default_class as i8, desired) as u8
}

pub struct ParseNode {
    pub token: Ptg,
    pub children: Vec<ParseNode>,
}

impl ParseNode {
    pub fn leaf(token: Ptg) -> Self {
        Self {
            token,
            children: Vec::new(),
        }
    }
    pub fn with(token: Ptg, children: Vec<ParseNode>) -> Self {
        Self { token, children }
    }

    fn token_count(&self) -> usize {
        let mut count = 1 + self.children.iter().map(ParseNode::token_count).sum::<usize>();
        if self.token.func_name() == Some("IF") {
            count += self.children.len();
        }
        count
    }

    fn encoded_size(&self) -> i32 {
        self.token.getSize() + self.children.iter().map(ParseNode::encoded_size).sum::<i32>()
    }
}

pub fn to_token_array(root: &ParseNode) -> Vec<Ptg> {
    let mut out = Vec::with_capacity(root.token_count());
    collect(root, &mut out);
    out
}

fn collect(node: &ParseNode, out: &mut Vec<Ptg>) {
    if node.token.func_name() == Some("IF") {
        collect_if(node, out);
        return;
    }
    for child in &node.children {
        collect(child, out);
    }
    out.push(node.token.clone());
}

fn collect_if(node: &ParseNode, out: &mut Vec<Ptg>) {
    collect(&node.children[0], out);
    let if_at = out.len();
    out.push(attr_if(0));
    collect(&node.children[1], out);
    let skip_true_at = out.len();
    out.push(attr_skip(0));
    let true_size = out[if_at + 1..skip_true_at]
        .iter()
        .map(Ptg::getSize)
        .sum::<i32>();
    if node.children.len() > 2 {
        collect(&node.children[2], out);
        let skip_false_at = out.len();
        out.push(attr_skip(0));
        let false_size = out[skip_true_at + 1..skip_false_at]
            .iter()
            .map(Ptg::getSize)
            .sum::<i32>();
        out[if_at] = attr_if(true_size + 4);
        out[skip_true_at] = attr_skip(false_size + 4 + 4 - 1);
        out[skip_false_at] = attr_skip(3);
    } else {
        out[if_at] = attr_if(true_size + 4);
        out[skip_true_at] = attr_skip(3);
    }
    out.push(node.token.clone());
}

pub fn transform_formula(root: &mut ParseNode, formula_type: i32) -> Result<(), String> {
    // java switch (formulaType) CELL / ARRAY / NAME
    let (formula_class, desired) = match formula_type {
        0 | 1 | 6 => (0, OperandClass::V),
        2 => (1, OperandClass::V),
        4 | 5 | 7 => (2, OperandClass::R),
        _ => return Err("IllegalArgumentException".to_string()),
    };
    transform_node(root, formula_class, desired);
    Ok(())
}

fn transform_node(node: &mut ParseNode, formula_class: usize, desired: OperandClass) {
    // java OperandClassTransformer.transformToken(fc, token, desiredOperandClass)
    if !node.token.is_base_token() {
        let class = transform_class(
            formula_class,
            node.token.getDefaultOperandClass(),
            desired,
        );
        node.token.setClass(class);
    }
    // java isSingleArgSum(token) 后使用 FuncVarPtg.SUM 判定参数类。
    let summed = node.token.is_sum_attr();
    // java ParenthesisPtg 子节点保持原 desiredOperandClass。
    if matches!(node.token.kind, PtgKind::Paren) {
        for child in &mut node.children {
            transform_node(child, formula_class, desired);
        }
        return;
    }
    // java IntersectionPtg / RangePtg / UnionPtg 的子节点强制 DesiredOperandClass.R。
    if matches!(node.token.kind, PtgKind::Intersection | PtgKind::Range | PtgKind::Union) {
        for child in &mut node.children {
            transform_node(child, formula_class, OperandClass::R);
        }
        return;
    }
    if summed {
        let meta = function_by_name("SUM").unwrap();
        for child in &mut node.children {
            transform_node(child, formula_class, meta.param_class(0));
        }
        return;
    }
    // java AbstractFunctionPtg.getParameterClass 同时服务 FuncPtg 与 FuncVarPtg。
    if let PtgKind::FuncVar { meta, .. } | PtgKind::Func { meta } = &node.token.kind {
        // java callerForceArrayFlag = default CLASS_VALUE && current CLASS_ARRAY。
        let caller_force_array = node.token.getDefaultOperandClass() == 32 && node.token.class == 64;
        let offset_force_array = meta.getName() == "OFFSET" && node.token.class == 64;
        let classes: Vec<OperandClass> = (0..node.children.len())
            .map(|index| {
                if caller_force_array || (index == 0 && offset_force_array) {
                    OperandClass::A
                } else {
                    meta.param_class(index)
                }
            })
            .collect();
        for (child, class) in node.children.iter_mut().zip(classes) {
            transform_node(child, formula_class, class);
        }
        return;
    }
    // java FuncVarPtg.create(index) 对缺失的索引采用 CLASS_VALUE、paramV。
    if let PtgKind::FuncVarExt { index, .. } = &node.token.kind {
        let class = if node.token.getDefaultOperandClass() == 32 && node.token.class == 64 {
            OperandClass::A
        } else if *index == 255 {
            OperandClass::R
        } else {
            OperandClass::V
        };
        for child in &mut node.children {
            transform_node(child, formula_class, class);
        }
        return;
    }
    // java ValueOperatorPtg / ControlPtg / MemFuncPtg / MemAreaPtg 的子节点规则。
    let child_desired = match desired {
        OperandClass::R => OperandClass::L,
        OperandClass::X => OperandClass::V,
        OperandClass::Y => OperandClass::A,
        OperandClass::N => OperandClass::M,
        other => other,
    };
    for child in &mut node.children {
        transform_node(child, formula_class, child_desired);
    }
}

pub fn encoded_size_of(node: &ParseNode) -> i32 {
    node.encoded_size()
}

#[cfg(test)]
mod function_metadata_tests {
    use super::*;

    #[test]
    fn java_registry_entries_and_footnotes() {
        // java FunctionDataHolder.createFunctionMetadataRegister: 249 add、两个 addFootnote。
        assert_eq!(FunctionMetadataRegistry::getFunctionByIndex(0).unwrap().getName(), "COUNT");
        assert_eq!(function_by_name("PI").unwrap().getMaxParams(), 0);
        assert_eq!(function_by_name("YEN").unwrap().getIndex(), 204);
        assert_eq!(function_by_name("JIS").unwrap().getIndex(), 215);
        assert_eq!(function_by_name("OFFSET").unwrap().getParameterClassCodes(), vec![OperandClass::R, OperandClass::V, OperandClass::V, OperandClass::V, OperandClass::V]);
        assert!(function_by_name("RAND").unwrap().isVolatile());
        assert!(function_by_name("LINEST").unwrap().hasFootnote());
        assert!(function_by_index(53).is_none());
        assert_eq!(function_by_index(367).unwrap().getName(), "VARA");
    }

    #[test]
    fn java_desired_operand_class_tables_cover_all_formula_classes() {
        // java DesiredOperandClass.transform: CELL/ARRAY/NAME × REF/VALUE/ARRAY。
        let all = [
            (OperandClass::R, [0, 0, 0, 32, 64, 64, 64, 64, 64]),
            (OperandClass::V, [32, 32, 64, 32, 32, 64, 32, 32, 64]),
            (OperandClass::A, [64, 64, 64, 64, 64, 64, 64, 64, 64]),
            (OperandClass::X, [0, 0, 0, 32, 32, 64, 32, 32, 64]),
            (OperandClass::Y, [0, 0, 0, 64, 64, 64, 64, 64, 64]),
            (OperandClass::Z, [0, 0, 0, 32, 32, 32, 32, 32, 32]),
            (OperandClass::L, [32, 64, 64, 32, 64, 64, 64, 64, 64]),
            (OperandClass::M, [32, 32, 32, 32, 32, 32, 32, 32, 32]),
            (OperandClass::N, [0, 0, 0, 32, 32, 32, 32, 32, 32]),
        ];
        for (desired, expected) in all {
            for (default, offset) in [(0, 0), (32, 3), (64, 6)] {
                for formula_class in 0..3 {
                    assert_eq!(transform_class(formula_class, default, desired), expected[offset + formula_class]);
                }
            }
        }
    }

    #[test]
    fn java_offset_array_result_forces_first_argument_array() {
        // java OperandClassTransformer: OFFSET return CLASS_REF；当结果为 ARRAY 时仅首参数强制 A。
        let offset = function_by_name("OFFSET").unwrap();
        let reference = ref_ptg(0, 0, false, false, SpreadsheetVersion::EXCEL2007);
        let mut node = ParseNode::with(func_var(offset, 3), vec![
            ParseNode::leaf(reference), ParseNode::leaf(int_ptg(1)), ParseNode::leaf(int_ptg(1)),
        ]);
        transform_node(&mut node, 0, OperandClass::A);
        assert_eq!(node.token.class, 64);
        assert_eq!(node.children[0].token.class, 64);
    }
}

impl Ptg{pub fn newIdentity()->u64{static NEXT:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1024);NEXT.fetch_add(1,std::sync::atomic::Ordering::Relaxed)}
// java Base operation tokens, BoolPtg, ErrPtg and MissingArgPtg are shared singleton objects.
pub fn javaIdentity(&self)->u64{match self.kind{PtgKind::Add|PtgKind::Subtract|PtgKind::Multiply|PtgKind::Divide|PtgKind::Power|PtgKind::Concat|PtgKind::LessThan|PtgKind::LessEqual|PtgKind::Equal|PtgKind::GreaterEqual|PtgKind::GreaterThan|PtgKind::NotEqual|PtgKind::Intersection|PtgKind::Union|PtgKind::Range|PtgKind::UnaryPlus|PtgKind::UnaryMinus|PtgKind::Percent|PtgKind::Paren|PtgKind::MissingArg=>self.getSid()as u64,PtgKind::Bool(v)=>64+v as u64,PtgKind::Err(v)=>128+v as u64,_=>self.java_identity}}
}

impl Ptg{
// java StringPtg/RefPtg/AreaPtg/Ref3DPtg/Area3DPtg override Object.equals; relative-name token classes retain identity.
pub fn javaEquals(&self,o:&Self)->bool{use PtgKind::*;if self.javaIdentity()==o.javaIdentity(){return true;}if self.class!=o.class&&!matches!(self.kind,Str(_)|StrRaw{..}){return false;}match(&self.kind,&o.kind){(Ref{row:a,col_raw:b,relative:false,..},Ref{row:c,col_raw:d,relative:false,..})=>a==c&&b==d,(Area{first_row:a,last_row:b,first_col_raw:c,last_col_raw:d,relative:false,..},Area{first_row:e,last_row:f,first_col_raw:g,last_col_raw:h,relative:false,..})=>a==e&&b==f&&c==g&&d==h,(Ref3D{extern_index:a,row:b,col_raw:c,..},Ref3D{extern_index:d,row:e,col_raw:f,..})=>a==d&&b==e&&c==f,(Area3D{extern_index:a,first_row:b,last_row:c,first_col_raw:d,last_col_raw:e,..},Area3D{extern_index:f,first_row:g,last_row:h,first_col_raw:i,last_col_raw:j,..})=>a==f&&b==g&&c==h&&d==i&&e==j,(Str(_)|StrRaw{..},Str(_)|StrRaw{..})=>self.javaString()==o.javaString(),_=>false}}
fn javaString(&self)->(Vec<u16>,bool){match &self.kind{PtgKind::Str(s)=>{let v=s.encode_utf16().collect::<Vec<_>>();let wide=v.iter().any(|c|*c>255);(v,wide)},PtgKind::StrRaw{units,wide}=>(units.clone(),*wide),_=>panic!("ClassCastException StringPtg")}}
pub fn javaHashCode(&self)->i32{match &self.kind{PtgKind::Str(_)|PtgKind::StrRaw{..}=>{let(v,wide)=self.javaString();let h=v.iter().fold(0i32,|h,u|h.wrapping_mul(31).wrapping_add(*u as i32));h.wrapping_shl(1).wrapping_add((h>>31)&1).wrapping_add(wide as i32)},PtgKind::Ref3D{extern_index,..}|PtgKind::Area3D{extern_index,..}=>31i32.wrapping_add(*extern_index),_=>self.javaIdentity()as i32}}
}

// rust: virtual output dispatch only. Each original Java method body lives in
// its matching class file; this carrier is not the complete Java class facade.
impl Ptg{
    // java public abstract String toFormulaString();
    pub fn toFormulaString(&self)->Option<String>{use super::*;Some(match &self.kind{
        PtgKind::Unknown=>UnknownPtg::UnknownPtg::toFormulaString(),PtgKind::Tbl{..}=>TblPtg::TblPtg::toFormulaString(),
        PtgKind::Int(_)=>self.toIntFormulaString(),PtgKind::Number(_)=>self.toNumberFormulaString(),PtgKind::Str(_)|PtgKind::StrRaw{..}=>self.toStringFormulaString(),PtgKind::Bool(_)=>self.toBoolFormulaString(),PtgKind::Err(_)=>self.toErrFormulaString(),
        PtgKind::Ref{..}=>self.formatReferenceAsString(),PtgKind::Ref3D{..}=>self.toRef3DFormulaString__plain(),PtgKind::Area{..}|PtgKind::Area3DExt{..}=>self.formatAreaReferenceAsString(),
        PtgKind::Ref3DExt{..}=>return None,
        PtgKind::Area3D{..}=>self.toArea3DFormulaString__plain(),PtgKind::Name{..}=>self.toNameFormulaString__plain(),PtgKind::NameX{..}=>self.toNameXFormulaString__plain(),PtgKind::DeletedRef3D{..}=>self.toDeletedRef3DFormulaString__plain(),PtgKind::DeletedArea3D{..}=>self.toDeletedArea3DFormulaString__plain(),
        PtgKind::Union=>UnionPtg::UnionPtg::toFormulaString(),PtgKind::Intersection=>IntersectionPtg::IntersectionPtg::toFormulaString(),PtgKind::Range=>RangePtg::RangePtg::toFormulaString(),
        PtgKind::Add|PtgKind::Subtract|PtgKind::Multiply|PtgKind::Divide|PtgKind::Power|PtgKind::Concat|PtgKind::Equal|PtgKind::LessThan|PtgKind::GreaterThan|PtgKind::LessEqual|PtgKind::GreaterEqual|PtgKind::NotEqual|PtgKind::UnaryMinus|PtgKind::UnaryPlus|PtgKind::Percent=>panic!("RuntimeException: toFormulaString(String[] operands) should be used for subclasses of OperationPtgs"),
        PtgKind::Paren=>ParenthesisPtg::ParenthesisPtg::toFormulaString(),PtgKind::MissingArg=>MissingArgPtg::MissingArgPtg::toFormulaString(),PtgKind::SxName{..}=>self.toSxNameFormulaString(),PtgKind::Attr{..}=>self.toAttrFormulaString(),
        PtgKind::Func{..}|PtgKind::FuncVar{..}|PtgKind::FuncVarExt{..}=>self.toFunctionFormulaString(),
        PtgKind::MemArea{..}=>MemAreaPtg::MemAreaPtg::toFormulaString(),PtgKind::MemErr{..}=>MemErrPtg::MemErrPtg::toFormulaString(),PtgKind::MemFunc{..}=>MemFuncPtg::MemFuncPtg::toFormulaString(),
        PtgKind::Array{..}=>self.toArrayFormulaString(),PtgKind::ArrayInitial{..}|PtgKind::MemAreaInitial{..}=>panic!("IllegalStateException: This object is a partially initialised tArray, and cannot be used as a Ptg"),
        PtgKind::Exp{..}=>ExpPtg::ExpPtg::toFormulaString(),PtgKind::RefError=>RefErrorPtg::RefErrorPtg::toFormulaString(),PtgKind::AreaErr=>AreaErrPtg::AreaErrPtg::toFormulaString(),
    })}
    // java public String toRCFormulaString(CellRangeAddressBase formulaAddress,CellRangeAddressBase originAddress,SpreadsheetVersion version,boolean isName){return this.toFormulaString();}
    pub fn toRCFormulaString(&self,formulaAddress:Option<&crate::org::apache::poi::ss::util::CellRangeAddressBase::CellRangeAddressBase>,originAddress:Option<&crate::org::apache::poi::ss::util::CellRangeAddressBase::CellRangeAddressBase>,version:SpreadsheetVersion,isName:bool)->Option<String>{match self.kind{PtgKind::Ref{..}=>Some(self.formatRCReferenceAsString(formulaAddress,originAddress,version,isName)),PtgKind::Area{..}=>Some(self.formatAreaRCReferenceAsString(formulaAddress,originAddress,version,isName)),_=>self.toFormulaString()}}
    // rust: OperationPtg virtual dispatch includes original AbstractFunctionPtg overrides.
    pub(crate) fn getNumberOfOperands(&self)->i32{match self.kind{PtgKind::Func{..}|PtgKind::FuncVar{..}|PtgKind::FuncVarExt{..}=>self.getFunctionOperandCount(),PtgKind::Attr{..}=>self.getAttrOperandCount(),PtgKind::MemFunc{subex_len}=>subex_len as i32,_=>self.operatorOperandCount()}}
    pub(crate) fn toFormulaString__operands(&self,operands:Option<&[Option<String>]>,argSeparator:char,unionSeparator:char)->Option<String>{match self.kind{PtgKind::Func{..}|PtgKind::FuncVar{..}|PtgKind::FuncVarExt{..}=>Some(self.toFunctionFormulaString__operands(operands,argSeparator,unionSeparator)),PtgKind::Attr{..}=>self.toAttrFormulaString__operands(operands),_=>Some(self.operatorFormulaString(operands,argSeparator,unionSeparator))}}
}

