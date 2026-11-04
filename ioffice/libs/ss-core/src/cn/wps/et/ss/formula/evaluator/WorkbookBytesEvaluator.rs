// java package cn.wps.et.ss.formula.evaluator;
use super::{
    EvalFailure, EvalResult, EvaluationContext::EvaluationContext,
    WorkbookEvaluator::WorkbookEvaluator,
};
use crate::cn::wps::et::ss::formula::eval::{
    BoolEval::BoolEval, ErrorEval::ErrorEval, MissingArgEval::MissingArgEval,
    NumberEval::NumberEval, StringEval::StringEval, ValueRef,
};
use crate::cn::wps::et::ss::formula::functions::builtin::TwoOperandNumericOperation::TwoOperandNumericOperation as Numeric;
use std::{collections::VecDeque, sync::Arc};
// java public final class WorkbookBytesEvaluator
pub struct WorkbookBytesEvaluator {
    workbookEvaluator: Arc<WorkbookEvaluator>,
}
impl WorkbookBytesEvaluator {
    // java this.workbookEvaluator = we;
    pub fn WorkbookBytesEvaluator(we: Arc<WorkbookEvaluator>) -> Self {
        Self {
            workbookEvaluator: we,
        }
    }
    // java public LinkedList<IValueEval> evaluateFormula(EvaluationContext ec, byte[] codes, int encodedTokenSize)
    pub fn evaluateFormula(
        &self,
        ec: &mut EvaluationContext,
        codes: &[u8],
        encodedTokenSize: i32,
    ) -> EvalResult<VecDeque<ValueRef>> {
        let mut stack: VecDeque<ValueRef> = VecDeque::new();
        let mut pc = 0usize;
        // java boolean isSharedFormula = ec.isSharedFormula(); SpreadsheetVersion sv = this.workbookEvaluator.getWorkbook().getSpreadsheetVersion(); int MAX_ROWS = sv.getMaxRows(); int MAX_COLS = sv.getMaxColumns(); int skip_to = 0;
        let isSharedFormula=ec.isSharedFormula();let sv=self.workbookEvaluator.getWorkbook().getSpreadsheetVersion();let MAX_ROWS=sv.getMaxRows();let MAX_COLS=sv.getMaxColumns();let skip_to=0usize;
        let size = usize::try_from(encodedTokenSize)
            .map_err(|_| EvalFailure::Runtime("negative token size"))?;
        if size > codes.len() {
            return Err(EvalFailure::Runtime("ArrayIndexOutOfBoundsException"));
        }
        // java ec.setEndOfCalc(false); while (pc < encodedTokenSize)
        ec.setEndOfCalc(false);
        while pc < size {
            // java byte op = codes[pc++]; byte sid = (byte)(op < 32 ? op : op & 31 | 32); byte ptgclass = (byte)(op - sid);
            let op = codes[pc];
            pc += 1;
            let sid = if (op as i8) < 32 { op } else { op & 31 | 32 };
            let _ptgclass = op.wrapping_sub(sid);
            match sid {
                // java case 1/2: pc += ExpPtg/TblPtg.getSize(sv) - 1;
                1 | 2 => {
                    pc += crate::cn::wps::et::ss::formula::ptg::ExpPtg::ExpPtg::getSize(
                        self.workbookEvaluator.getWorkbook().getSpreadsheetVersion(),
                    ) as usize
                        - 1;
                }
                3..=7 => {
                    // java args[1] = stack.removeFirst(); args[0] = stack.removeFirst();
                    let right = stack
                        .pop_front()
                        .ok_or(EvalFailure::Runtime("NoSuchElementException"))?;
                    let left = stack
                        .pop_front()
                        .ok_or(EvalFailure::Runtime("NoSuchElementException"))?;
                    if pc == size {
                        ec.setEndOfCalc(true);
                    }
                    // java terminal Add/Subtract select RoundAddEval/RoundSubtractEval.
                    let function = match sid {
                        3 if pc == size => &Numeric::RoundAddEval,
                        3 => &Numeric::AddEval,
                        4 if pc == size => &Numeric::RoundSubtractEval,
                        4 => &Numeric::SubtractEval,
                        5 => &Numeric::MultiplyEval,
                        6 => &Numeric::DivideEval,
                        _ => &Numeric::PowerEval,
                    };
                    stack.push_front(function.evaluateOnce(_ptgclass as i8, &[left, right], ec)?);
                }
                // java case 21: break;
                21 => (),
                // java case 22: stack.addFirst(MissingArgEval.instance);
                22 => stack.push_front(Arc::new(MissingArgEval::instance)),
                23 => {
                    // java nChars = codes[pc++] & 255; is16bitUnicode = (codes[pc++] & 1) != 0;
                    let n = read(codes, &mut pc, 1)?[0] as usize;
                    let wide = read(codes, &mut pc, 1)?[0] & 1 != 0;
                    let raw = read(codes, &mut pc, n * if wide { 2 } else { 1 })?;
                    let units = if wide {
                        raw.chunks_exact(2)
                            .map(|b| u16::from_le_bytes([b[0], b[1]]))
                            .collect()
                    } else {
                        raw.iter().map(|b| *b as u16).collect()
                    };
                    // java empty string uses EMPTY_INSTANCE, otherwise new StringEval(sb.toString()).
                    stack.push_front(if n == 0 {
                        Arc::new(StringEval::EMPTY_INSTANCE)
                    } else {
                        Arc::new(StringEval::from_utf16(units))
                    });
                }
                // java case 28: stack.addFirst(ErrorEval.valueOf(codes[pc++]));
                28 => {
                    let code = read(codes, &mut pc, 1)?[0] as i8 as i32;
                    stack.push_front(Arc::new(ErrorEval::valueOf(code)));
                }
                // java case 29: codes[pc++] == 1 ? BoolEval.TRUE : BoolEval.FALSE;
                29 => {
                    let b = read(codes, &mut pc, 1)?[0] == 1;
                    stack.push_front(Arc::new(BoolEval::valueOf(b)));
                }
                30 => {
                    // java int v = LittleEndian.getUShort(codes, pc); if (0 == v) NumberEval.ZERO; pc += 2;
                    let b = read(codes, &mut pc, 2)?;
                    let n = u16::from_le_bytes([b[0], b[1]]);
                    stack.push_front(if n == 0 {
                        Arc::new(NumberEval::ZERO)
                    } else {
                        Arc::new(NumberEval::NumberEval(n as f64))
                    });
                }
                // java case 31: new NumberEval(LittleEndian.getDouble(codes, pc)); pc += 8;
                31 => {
                    let b = read(codes, &mut pc, 8)?;
                    stack.push_front(Arc::new(NumberEval::NumberEval(f64::from_le_bytes(
                        b.try_into().unwrap(),
                    ))));
                }
                // java case 36: little-endian row (2/4 bytes by sv), ushort column & 16383; ec.getRefEval(row,col).
                36 => {
                    let wide=self.workbookEvaluator.getWorkbook().getSpreadsheetVersion()!=crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion::EXCEL97;
                    let b = read(codes, &mut pc, if wide { 4 } else { 2 })?;
                    let row = if wide {
                        i32::from_le_bytes(b.try_into().unwrap())
                    } else {
                        u16::from_le_bytes(b.try_into().unwrap()) as i32
                    };
                    let b = read(codes, &mut pc, 2)?;
                    let col = (u16::from_le_bytes(b.try_into().unwrap()) & 16383) as i32;
                    stack.push_front(ec.getRefEval(row, col));
                }
                // Each unported Java opcode remains visible, and never supplies a cached/fabricated value.
                // java case 44: {
                44=>{
                    let (mut row,mut col,raw_col);
                    // java if (sv == SpreadsheetVersion.EXCEL97) {
                    if sv==crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion::EXCEL97{
                        // java if (pc - 1 < skip_to) { pc += 4; break; }
                        if pc-1<skip_to{pc+=4;continue;}
                        // java if (!isSharedFormula) { pc += 4; stack.addFirst(ErrorEval.REF_INVALID); break; }
                        if !isSharedFormula{pc+=4;stack.push_front(Arc::new(ErrorEval::REF_INVALID));continue;}
                        // java row = LittleEndian.getUShort(codes, pc); pc += 2; raw_col = LittleEndian.getUShort(codes, pc); pc += 2; col = raw_col & 16383;
                        row=u16::from_le_bytes(read(codes,&mut pc,2)?.try_into().unwrap())as i32;raw_col=u16::from_le_bytes(read(codes,&mut pc,2)?.try_into().unwrap());col=(raw_col&16383)as i32;
                    }else{
                        // java if (pc - 1 < skip_to) { pc += 6; break; }
                        if pc-1<skip_to{pc+=6;continue;}
                        // java if (!isSharedFormula) { pc += 6; stack.addFirst(ErrorEval.REF_INVALID); break; }
                        if !isSharedFormula{pc+=6;stack.push_front(Arc::new(ErrorEval::REF_INVALID));continue;}
                        // java row = LittleEndian.getInt(codes, pc); pc += 4; raw_col = LittleEndian.getUShort(codes, pc); pc += 2; col = raw_col & 16383;
                        row=i32::from_le_bytes(read(codes,&mut pc,4)?.try_into().unwrap());raw_col=u16::from_le_bytes(read(codes,&mut pc,2)?.try_into().unwrap());col=(raw_col&16383)as i32;
                    }
                    // java if ((raw_col & '耀') != 0) { row = (row + ec.getRowIndex()) % MAX_ROWS; }
                    if raw_col&32768!=0{row=row.wrapping_add(ec.getRowIndex())%MAX_ROWS;}
                    // java if ((raw_col & 16384) != 0) { col = (col + ec.getColumnIndex()) % MAX_COLS; }
                    if raw_col&16384!=0{col=col.wrapping_add(ec.getColumnIndex())%MAX_COLS;}
                    // java stack.addFirst(ec.getRefEval(row, col)); break;
                    stack.push_front(ec.getRefEval(row,col));
                }
                // java case 45: {
                45=>{
                    // java int size; if (sv == SpreadsheetVersion.EXCEL97) { size = 9; } else { size = 13; }
                    let size=if sv==crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion::EXCEL97{9}else{13};
                    // java if (pc - 1 < skip_to) { pc += size - 1; }
                    if pc-1<skip_to{pc+=size-1;}
                    // java else if (!isSharedFormula) { pc += size - 1; stack.addFirst(ErrorEval.REF_INVALID); }
                    else if !isSharedFormula{pc+=size-1;stack.push_front(Arc::new(ErrorEval::REF_INVALID));}
                    else{
                        let(mut fr,mut lr);
                        // java if (sv == SpreadsheetVersion.EXCEL97) { fr = LittleEndian.getUShort(codes, pc); pc += 2; lr = LittleEndian.getUShort(codes, pc); pc += 2; }
                        if size==9{fr=u16::from_le_bytes(read(codes,&mut pc,2)?.try_into().unwrap())as i32;lr=u16::from_le_bytes(read(codes,&mut pc,2)?.try_into().unwrap())as i32;}
                        // java else { fr = LittleEndian.getInt(codes, pc); pc += 4; lr = LittleEndian.getInt(codes, pc); pc += 4; }
                        else{fr=i32::from_le_bytes(read(codes,&mut pc,4)?.try_into().unwrap());lr=i32::from_le_bytes(read(codes,&mut pc,4)?.try_into().unwrap());}
                        // java int raw_fc = LittleEndian.getUShort(codes, pc); pc += 2; int raw_lc = LittleEndian.getUShort(codes, pc); pc += 2; int fc = raw_fc & 16383; int lc = raw_lc & 16383;
                        let raw_fc=u16::from_le_bytes(read(codes,&mut pc,2)?.try_into().unwrap());let raw_lc=u16::from_le_bytes(read(codes,&mut pc,2)?.try_into().unwrap());let mut fc=(raw_fc&16383)as i32;let mut lc=(raw_lc&16383)as i32;
                        // java if ((raw_fc & '耀') != 0) { fr = (fr + ec.getRowIndex()) % MAX_ROWS; }
                        if raw_fc&32768!=0{fr=fr.wrapping_add(ec.getRowIndex())%MAX_ROWS;}
                        // java if ((raw_lc & '耀') != 0) { lr = (lr + ec.getRowIndex()) % MAX_ROWS; }
                        if raw_lc&32768!=0{lr=lr.wrapping_add(ec.getRowIndex())%MAX_ROWS;}
                        // java if ((raw_fc & 16384) != 0) { fc = (fc + ec.getColumnIndex()) % MAX_COLS; }
                        if raw_fc&16384!=0{fc=fc.wrapping_add(ec.getColumnIndex())%MAX_COLS;}
                        // java if ((raw_lc & 16384) != 0) { lc = (lc + ec.getColumnIndex()) % MAX_COLS; }
                        if raw_lc&16384!=0{lc=lc.wrapping_add(ec.getColumnIndex())%MAX_COLS;}
                        // java if (fc > lc) { int tmp = lc; lc = fc; fc = tmp; }
                        if fc>lc{let tmp=lc;lc=fc;fc=tmp;}
                        // java stack.addFirst(ec.getAreaEval(fr, fc, lr, lc, ptgclass));
                        stack.push_front(ec.getAreaEval(fr,fc,lr,lc,_ptgclass as i8));
                    }
                }
                // java case 58: {
                58 => {
                    // java int size; if (sv == SpreadsheetVersion.EXCEL97) { size = 7; } else { size = 9; }
                    let size = if sv == crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion::EXCEL97 { 7 } else { 9 };
                    // java if (pc - 1 < skip_to) { pc += size - 1; } else {
                    if pc - 1 < skip_to { pc += size - 1; } else {
                        // java int extSheet = LittleEndian.getShort(codes, pc); pc += 2;
                        let extSheet = i16::from_le_bytes(read(codes, &mut pc, 2)?.try_into().unwrap()) as i32;
                        let mut row;
                        // java if (sv == SpreadsheetVersion.EXCEL97) { row = LittleEndian.getUShort(codes, pc); pc += 2; }
                        if sv == crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion::EXCEL97 { row = u16::from_le_bytes(read(codes, &mut pc, 2)?.try_into().unwrap()) as i32; }
                        // java else { row = LittleEndian.getInt(codes, pc); pc += 4; }
                        else { row = i32::from_le_bytes(read(codes, &mut pc, 4)?.try_into().unwrap()); }
                        // java int raw_col = LittleEndian.getUShort(codes, pc); pc += 2; int col = raw_col & 16383;
                        let raw_col = u16::from_le_bytes(read(codes, &mut pc, 2)?.try_into().unwrap()); let mut col = (raw_col & 16383) as i32;
                        // java if (isSharedFormula) {
                        if isSharedFormula {
                            // java if ((raw_col & '耀') != 0) { row = (row + ec.getRowIndex()) % MAX_ROWS; }
                            if raw_col & 32768 != 0 { row = row.wrapping_add(ec.getRowIndex()) % MAX_ROWS; }
                            // java if ((raw_col & 16384) != 0) { col = (col + ec.getColumnIndex()) % MAX_COLS; }
                            if raw_col & 16384 != 0 { col = col.wrapping_add(ec.getColumnIndex()) % MAX_COLS; }
                        }
                        // java IValueEval rs = ec.getRef3DEval(row, col, extSheet); stack.addFirst(rs);
                        let rs = ec.getRef3DEval(row, col, extSheet)?; stack.push_front(rs);
                    }
                }
                // java case 59: {
                59 => {
                    // java int size; if (sv == SpreadsheetVersion.EXCEL97) { size = 11; } else { size = 15; }
                    let size = if sv == crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion::EXCEL97 { 11 } else { 15 };
                    // java if (pc - 1 < skip_to) { pc += size - 1; } else {
                    if pc - 1 < skip_to { pc += size - 1; } else {
                        // java int extSheet = LittleEndian.getShort(codes, pc); pc += 2;
                        let extSheet = i16::from_le_bytes(read(codes, &mut pc, 2)?.try_into().unwrap()) as i32;
                        let (mut fr, mut lr);
                        // java if (sv == SpreadsheetVersion.EXCEL97) { fr = LittleEndian.getUShort(codes, pc); pc += 2; lr = LittleEndian.getUShort(codes, pc); pc += 2; }
                        if sv == crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion::EXCEL97 { fr = u16::from_le_bytes(read(codes, &mut pc, 2)?.try_into().unwrap()) as i32; lr = u16::from_le_bytes(read(codes, &mut pc, 2)?.try_into().unwrap()) as i32; }
                        // java else { fr = LittleEndian.getInt(codes, pc); pc += 4; lr = LittleEndian.getInt(codes, pc); pc += 4; }
                        else { fr = i32::from_le_bytes(read(codes, &mut pc, 4)?.try_into().unwrap()); lr = i32::from_le_bytes(read(codes, &mut pc, 4)?.try_into().unwrap()); }
                        // java int raw_fc = LittleEndian.getUShort(codes, pc); pc += 2; int raw_lc = LittleEndian.getUShort(codes, pc); pc += 2; int fc = raw_fc & 16383; int lc = raw_lc & 16383;
                        let raw_fc = u16::from_le_bytes(read(codes, &mut pc, 2)?.try_into().unwrap()); let raw_lc = u16::from_le_bytes(read(codes, &mut pc, 2)?.try_into().unwrap()); let mut fc = (raw_fc & 16383) as i32; let mut lc = (raw_lc & 16383) as i32;
                        // java if (isSharedFormula) {
                        if isSharedFormula {
                            // java if ((raw_fc & '耀') != 0) { fr = (fr + ec.getRowIndex()) % MAX_ROWS; }
                            if raw_fc & 32768 != 0 { fr = fr.wrapping_add(ec.getRowIndex()) % MAX_ROWS; }
                            // java if ((raw_lc & '耀') != 0) { lr = (lr + ec.getRowIndex()) % MAX_ROWS; }
                            if raw_lc & 32768 != 0 { lr = lr.wrapping_add(ec.getRowIndex()) % MAX_ROWS; }
                            // java if ((raw_fc & 16384) != 0) { fc = (fc + ec.getColumnIndex()) % MAX_COLS; }
                            if raw_fc & 16384 != 0 { fc = fc.wrapping_add(ec.getColumnIndex()) % MAX_COLS; }
                            // java if ((raw_lc & 16384) != 0) { lc = (lc + ec.getColumnIndex()) % MAX_COLS; }
                            if raw_lc & 16384 != 0 { lc = lc.wrapping_add(ec.getColumnIndex()) % MAX_COLS; }
                        }
                        // java IValueEval rs = ec.getArea3DEval(fr, fc, lr, lc, extSheet, ptgclass); stack.addFirst(rs);
                        let rs = ec.getArea3DEval(fr, fc, lr, lc, extSheet, _ptgclass as i8)?; stack.push_front(rs);
                    }
                }
                8..=20 | 25 | 32..=47 | 57 | 60..=61 => {
                    return Err(EvalFailure::Unported(match sid {
                        35 => "WorkbookBytesEvaluator.case35/NamePtg",
                        33 | 34 => "FunctionEval",
                        25 => "AttrPtg control flow",
                        37 => "LazyAreaEval",
                        _ => "WorkbookBytesEvaluator opcode",
                    }))
                }
                // java default: throw new RuntimeException("");
                _ => return Err(EvalFailure::Runtime("")),
            }
        }
        Ok(stack)
    }
}
// rust: checked byte-array reads preserve Java's bounds exception at the evaluator boundary.
fn read<'a>(codes: &'a [u8], pc: &mut usize, len: usize) -> EvalResult<&'a [u8]> {
    let end = pc
        .checked_add(len)
        .ok_or(EvalFailure::Runtime("ArrayIndexOutOfBoundsException"))?;
    let bytes = codes
        .get(*pc..end)
        .ok_or(EvalFailure::Runtime("ArrayIndexOutOfBoundsException"))?;
    *pc = end;
    Ok(bytes)
}
