// java package cn.wps.et.ss.calcchain;

use crate::cn::wps::et::ss::formula::ptg::Ptg::{
    area_err_ptg, col_index, col_rel, pack_col, ref_error_ptg, row_rel, Ptg, PtgKind,
};
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;
// java import cn.wps.et.ss.interfaces.IEvalFormulaBase/IEvalWorkbook;
use crate::cn::wps::et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase;
use crate::cn::wps::et::ss::interfaces::IEvalWorkbook::IEvalWorkbook;
// java IEvalNameBase instanceof IEvalName; concrete Rust implementation is EvaluationName.
use crate::cn::wps::moss::engine::eval::EvaluationName::EvaluationName;

pub struct FormulaHelper;
#[cfg(test)]
#[path="FormulaHelper.replace.review.rs"]
pub(crate) mod replace_review;

impl FormulaHelper {
    // java private static final int[] volatileFuncNum = {63, 74, 78, 125, 148, 221, 244, 255, 257, 344};
    // java public static boolean checkVolatileFunc(int funcIdx)
    pub fn checkVolatileFunc(func_idx: i32) -> bool {
        let volatile_func_num = [63, 74, 78, 125, 148, 221, 244, 255, 257, 344];
        // java int low = -1; int high = volatileFuncNum.length;
        let mut low = -1;
        let mut high = volatile_func_num.len() as i32;
        // java while (low + 1 < high)
        while low + 1 < high {
            let middle = (low + high) / 2;
            if volatile_func_num[middle as usize] == func_idx { return true; }
            if volatile_func_num[middle as usize] > func_idx { high = middle; }
            else { low = middle; }
        }
        // java return false;
        false
    }

    // java public static boolean hasSubtotal(Ptg[] ptgs)
    pub fn hasSubtotal(ptgs: &[Ptg]) -> bool {
        // java for (int i = 0, len = ptgs.length; i < len; ++i)
        for ptg in ptgs {
            // java if (ptgs[i] instanceof AbstractFunctionPtg) and getFunctionIndex() == 344
            match &ptg.kind {
                PtgKind::Func { meta } | PtgKind::FuncVar { meta, .. } if meta.getIndex() == 344 => return true,
                PtgKind::FuncVarExt { index, .. } if *index == 344 => return true,
                _ => {}
            }
        }
        // java return false;
        false
    }

    // java public static boolean checkVolatile(Ptg[] ptgs, IEvalFormulaBase fml, IEvalWorkbook cc)
    pub fn checkVolatile(ptgs: &[Ptg], fml: &dyn IEvalFormulaBase, cw: &dyn IEvalWorkbook) -> bool {
        // java for (int i = 0, len = ptgs.length; i < len; ++i)
        for ptg in ptgs {
            // java switch (ptg.getSid())
            match &ptg.kind {
                // java case 17: return true;
                PtgKind::Range => return true,
                // java case 25: AttrPtg.isSemiVolatile()。
                PtgKind::Attr { options, .. } if options & 1 != 0 => return true,
                // java case 33/34: checkVolatileFunc(functionIndex)。
                PtgKind::Func { meta } | PtgKind::FuncVar { meta, .. }
                    if Self::checkVolatileFunc(meta.getIndex()) => return true,
                PtgKind::FuncVarExt { index, .. }
                    if Self::checkVolatileFunc(*index as i32) => return true,
                // java case 35: resolve NamePtg; DISPIMG function name returns false immediately.
                PtgKind::Name { index } => {
                    if let Some(name) = cw.getNameByNamePtg(*index) {
                        if name.isFunctionName() && Self::isUnVolatileName(&name.getNameText(), true) {
                            return false;
                        }
                        if name.hasFormula() {
                            let definition = Self::replaceRelativeNamePtgs(&crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), cw.getSpreadsheetVersion());
                            if !Self::isSame(Some(&definition), Some(ptgs)) && Self::checkVolatile(&definition, fml, cw) {
                                return true;
                            }
                        }
                    }
                }
                // java case 57: try NameXPtg -> IEvalName, catch(Throwable) and continue.
                PtgKind::NameX { sheet_ref, name_number } => {
                    let found = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        let Some(name) = cw.getNameByNameXPtg(*sheet_ref, *name_number - 1) else { return false; };
                        let Some(name) = name.as_any().downcast_ref::<EvaluationName>() else { return false; };
                        let definition = Self::replaceRelativeNamePtgs(&crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&name.getNameDefinition()), fml.getFmlRow(), fml.getFmlCol(), cw.getSpreadsheetVersion());
                        !Self::isSame(Some(&definition), Some(ptgs)) && Self::checkVolatile(&definition, fml, cw)
                    })).unwrap_or(false);
                    if found { return true; }
                }
                _ => {}
            }
        }
        // java return false;
        false
    }

    // java private static boolean isUnVolatileName(NamePtg namePtg, IEvalWorkbook workbook)
    fn isUnVolatileName(name_text: &str, is_function_name: bool) -> bool {
        // java NamePtg.toFormulaString(default RendererOption) strips six characters for function names.
        let units: Vec<u16> = name_text.encode_utf16().collect();
        let shortened = is_function_name && units.len() > 6 && units.first() == Some(&(b'_' as u16)) && units.contains(&(b'.' as u16));
        let rendered = if shortened { &units[6..] } else { units.as_slice() };
        // java noVolatileFuncName = {"DISPIMG"}; String.equals is case-sensitive.
        rendered == "DISPIMG".encode_utf16().collect::<Vec<_>>().as_slice()
    }

    // java public static boolean isSame(Ptg[] p1, Ptg[] p2)
    pub fn isSame(p1: Option<&[Ptg]>, p2: Option<&[Ptg]>) -> bool {
        // java if (p1 != null && p2 != null)
        let (Some(p1), Some(p2)) = (p1, p2) else { return false; };
        // java if (p1.length != p2.length) return false;
        if p1.len() != p2.len() { return false; }
        // java for (int i = 0; i < p1.length; ++i) if (p1[i].getPtgClass() != p2[i].getPtgClass()) return false;
        for (left, right) in p1.iter().zip(p2) {
            if left.class != right.class { return false; }
        }
        // java return true;
        true
    }

    // java public static Ptg[] replaceRelativeNamePtgs(Ptg[] ptgs, int row, int col, SpreadsheetVersion sv)
    pub fn replaceRelativeNamePtgs(ptgs: &[Ptg], row: i32, col: i32, sv: SpreadsheetVersion) -> Vec<Ptg> {
        // java int len = ptgs.length; Ptg[] namePtgs = new Ptg[len];
        let mut name_ptgs = Vec::with_capacity(ptgs.len());
        // java for (int i = 0; i < len; ++i)
        for ptg in ptgs {
            // java switch (ptg.getSid())
            let mut replacement = ptg.clone();
            match &mut replacement.kind {
                // java case 58: RefPtgBase rptg = (RefPtgBase)ptg;
                PtgKind::Ref3D { row: ref_row, col_raw, .. }
                | PtgKind::Ref3DExt { row: ref_row, col_raw, .. } => {
                    let relative_row = row_rel(*col_raw);
                    let relative_col = col_rel(*col_raw);
                    // java if (rptg.isRowRelative() || rptg.isColRelative()) rptg = (RefPtgBase)rptg.clone();
                    if relative_row || relative_col {
                        // java if (rptg.isRowRelative()) { setRow((row + getRow()) % maxRows); setRowRelative(false); }
                        if relative_row { *ref_row = (row + *ref_row) % sv.getMaxRows(); }
                        // java if (rptg.isColRelative()) { setColumn((col + getColumn()) % maxColumns); setColRelative(false); }
                        let column = if relative_col { (col + col_index(*col_raw)) % sv.getMaxColumns() } else { col_index(*col_raw) };
                        *col_raw = pack_col(column, false, false);
                    }
                }
                // java case 59: AreaPtgBase aptg = (AreaPtgBase)ptg;
                PtgKind::Area3D { first_row, last_row, first_col_raw, last_col_raw, .. }
                | PtgKind::Area3DExt { first_row, last_row, first_col_raw, last_col_raw, .. } => {
                    let (first_row_rel, last_row_rel) = (row_rel(*first_col_raw), row_rel(*last_col_raw));
                    let (first_col_rel, last_col_rel) = (col_rel(*first_col_raw), col_rel(*last_col_raw));
                    // java if (any endpoint is relative) aptg = (AreaPtgBase)aptg.clone();
                    if first_row_rel || last_row_rel || first_col_rel || last_col_rel {
                        // java each relative row/column adds the formula origin modulo SpreadsheetVersion bounds.
                        if first_row_rel { *first_row = (row + *first_row) % sv.getMaxRows(); }
                        if last_row_rel { *last_row = (row + *last_row) % sv.getMaxRows(); }
                        let first_col = if first_col_rel { (col + col_index(*first_col_raw)) % sv.getMaxColumns() } else { col_index(*first_col_raw) };
                        let last_col = if last_col_rel { (col + col_index(*last_col_raw)) % sv.getMaxColumns() } else { col_index(*last_col_raw) };
                        *first_col_raw = pack_col(first_col, false, false);
                        *last_col_raw = pack_col(last_col, false, false);
                        // java if (firstRow > lastRow) swap endpoints.
                        if *first_row > *last_row { std::mem::swap(first_row, last_row); }
                        // java if (firstColumn > lastColumn) swap endpoints.
                        if col_index(*first_col_raw) > col_index(*last_col_raw) { std::mem::swap(first_col_raw, last_col_raw); }
                    }
                }
                // java default: namePtgs[i] = ptg;
                _ => {}
            }
            name_ptgs.push(replacement);
        }
        // java return namePtgs;
        name_ptgs
    }

    // java public static RecogniseResult getFormulaSuggestedNumfmt(Ptg[] ptgs, LanguageType language) {
    pub fn getFormulaSuggestedNumfmt(ptgs: &[Ptg], _language: Option<util::org::apache::poi::util::LanguageType::LanguageType>) -> crate::cn::wps::et::ss::constant::RecogniseResult::RecogniseResult {
        use crate::cn::wps::et::ss::{constant::RecogniseResult::RecogniseResult, statemachine::RecogniseResultRl::Value};
        // java RecogniseResult ret = new RecogniseResult((double)0.0F);
        let mut ret = RecogniseResult::fromValue(Value::Number(0.0));
        // java if (ptgs[ptgs.length - 1] instanceof AbstractFunctionPtg) {
        let last = ptgs.last().expect("ArrayIndexOutOfBoundsException");
        let function = match &last.kind {
            PtgKind::Func { meta } | PtgKind::FuncVar { meta, .. } => Some(meta.getIndex()),
            PtgKind::FuncVarExt { index, .. } => Some(*index as i32),
            _ => None,
        };
        if let Some(funcIndex) = function {
            // java AbstractFunctionPtg fptg = (AbstractFunctionPtg)ptgs[ptgs.length - 1]; int funcIndex = fptg.getFunctionIndex();
            // java if (funcIndex == 74) {
            if funcIndex == 74 {
                // java ret.setSuggestedNumfmt(22); ret.setDateFlag(); ret.setTimeFlag();
                ret.setSuggestedNumfmt__2(22); ret.setDateFlag(); ret.setTimeFlag();
            // java } else if (funcIndex != 221 && funcIndex != 65) {
            } else if funcIndex != 221 && funcIndex != 65 {
                // java if (funcIndex == 66) { ret.setSuggestedNumfmt(18); ret.setTimeFlag(); }
                if funcIndex == 66 { ret.setSuggestedNumfmt__2(18); ret.setTimeFlag(); }
            // java } else { ret.setSuggestedNumfmt(14); ret.setDateFlag(); }
            } else { ret.setSuggestedNumfmt__2(14); ret.setDateFlag(); }
        }
        // java return ret;
        ret
    }

    // java public static Ptg[] replaceNPtgs(Ptg[] ptgs, int row, int col, SpreadsheetVersion sv)
    pub fn replaceNPtgs(ptgs: &[Ptg], row: i32, col: i32, sv: SpreadsheetVersion) -> Vec<Ptg> {
        // Native owned read transport for the existing slice-based consumers.
        use crate::cn::wps::et::ss::formula::ptg::Ptg::{ptgArray,ptgValues};
        ptgValues(&Self::replaceNPtgs__array(Some(&ptgArray(ptgs.to_vec())),row,col,Some(sv)))
    }

    // java public static Ptg[] replaceNPtgs(Ptg[] ptgs, int row, int col, SpreadsheetVersion sv) {
    pub fn replaceNPtgs__array(ptgs:Option<&crate::cn::wps::et::ss::formula::ptg::Ptg::PtgArray>,row:i32,col:i32,sv:Option<SpreadsheetVersion>)->crate::cn::wps::et::ss::formula::ptg::Ptg::PtgArray{
        use crate::java_runtime::{Rc,RefCell};
        // java Ptg[] ret = new Ptg[ptgs.length]; int i = 0;
        let ptgs=ptgs.expect("NullPointerException");let ret=Rc::new(RefCell::new(vec![None;ptgs.borrow().len()].into_boxed_slice()));let mut i=0;
        // java for(int len = ret.length; i < len; ++i) {
        let len=ret.borrow().len();while i<len{
            // java Ptg ptg = ptgs[i]; ret[i] = replaceNPtg(ptg, row, col, sv);
            let ptg=ptgs.borrow()[i].clone();ret.borrow_mut()[i]=Some(Self::replaceNPtg__reference(ptg.as_ref(),row,col,sv));i+=1;
        }
        // java return ret;
        ret
    }

    // java public static Ptg replaceNPtg(Ptg ptg, int row, int col, SpreadsheetVersion sv)
    pub fn replaceNPtg(ptg: &Ptg, row: i32, col: i32, sv: SpreadsheetVersion) -> Ptg {
        // Native owned read transport; the reference overload owns original control flow.
        let reference=crate::cn::wps::et::ss::formula::ptg::Ptg::ptgRef(ptg.clone());let result=Self::replaceNPtg__reference(Some(&reference),row,col,Some(sv));let value=result.borrow().clone();value
    }
    // java public static Ptg replaceNPtg(Ptg ptg, int row, int col, SpreadsheetVersion sv) {
    pub fn replaceNPtg__reference(ptg:Option<&crate::cn::wps::et::ss::formula::ptg::Ptg::PtgRef>,row:i32,col:i32,sv:Option<SpreadsheetVersion>)->crate::cn::wps::et::ss::formula::ptg::Ptg::PtgRef{
        use crate::cn::wps::et::ss::formula::ptg::{RefPtg::RefPtg,AreaPtg::AreaPtg,Ref3DPtg::Ref3DPtg,Area3DPtg::Area3DPtg};
        // java Ptg ret = ptg; int sid = ptg.getSid();
        let ptg=ptg.expect("NullPointerException");let mut ret=ptg.clone();let sid=ptg.borrow().getSid();
        // java if (sid == 44) { RefNPtg rnptg = (RefNPtg)ptg; ret = RefPtg.createFromNptg(rnptg, row, col, sv); }
        if sid==44{if !matches!(ptg.borrow().kind,PtgKind::Ref{relative:true,..}){panic!("ClassCastException");}ret=RefPtg::createFromNptg(Some(ptg),row,col,sv);}
        // java else if (sid == 45) { AreaNPtg anptg = (AreaNPtg)ptg; ret = AreaPtg.createFromNptg(anptg, row, col, sv); }
        else if sid==45{if !matches!(ptg.borrow().kind,PtgKind::Area{relative:true,..}){panic!("ClassCastException");}ret=AreaPtg::createFromNptg(Some(ptg),row,col,sv);}
        // java else if (sid == 58) { Ref3DPtg rptg = (Ref3DPtg)ptg; ret = Ref3DPtg.createFromNptg(rptg, row, col, sv); }
        else if sid==58{if !matches!(ptg.borrow().kind,PtgKind::Ref3D{..}){panic!("ClassCastException");}ret=Ref3DPtg::createFromNptg(Some(ptg),row,col,sv);}
        // java else if (sid == 59) { Area3DPtg aptg = (Area3DPtg)ptg; ret = Area3DPtg.createFromNptg(aptg, row, col, sv); }
        else if sid==59{if !matches!(ptg.borrow().kind,PtgKind::Area3D{..}){panic!("ClassCastException");}ret=Area3DPtg::createFromNptg(Some(ptg),row,col,sv);}
        // java return ret;
        ret
    }

    // java public static Ptg[] convertFromExcel97ToExcel2007(Ptg[] ptgs,int formulaType,SpreadsheetVersion sv){
    pub fn convertFromExcel97ToExcel2007(ptgs:Option<crate::cn::wps::et::ss::formula::ptg::Ptg::PtgArray>,formulaType:i32,sv:Option<SpreadsheetVersion>)->Option<crate::cn::wps::et::ss::formula::ptg::Ptg::PtgArray>{
        use crate::java_runtime::{Rc,RefCell};
        // java if(sv==SpreadsheetVersion.EXCEL2007){return ptgs;}
        if sv==Some(SpreadsheetVersion::EXCEL2007){return ptgs;}
        // java int len=ptgs.length;
        let ptgs=ptgs.expect("NullPointerException");let len=ptgs.borrow().len();
        // java int relative_row_sign_bit=SpreadsheetVersion.EXCEL97.getMaxRows()>>1;int relative_col_sign_bit=SpreadsheetVersion.EXCEL97.getMaxColumns()>>1;
        let relative_row_sign_bit=SpreadsheetVersion::EXCEL97.getMaxRows()>>1;let relative_col_sign_bit=SpreadsheetVersion::EXCEL97.getMaxColumns()>>1;
        // java int max_row_delta=SpreadsheetVersion.EXCEL2007.getMaxRows()-SpreadsheetVersion.EXCEL97.getMaxRows();int max_col_delta=SpreadsheetVersion.EXCEL2007.getMaxColumns()-SpreadsheetVersion.EXCEL97.getMaxColumns();
        let max_row_delta=SpreadsheetVersion::EXCEL2007.getMaxRows()-SpreadsheetVersion::EXCEL97.getMaxRows();let max_col_delta=SpreadsheetVersion::EXCEL2007.getMaxColumns()-SpreadsheetVersion::EXCEL97.getMaxColumns();
        // java Ptg[] ret=new Ptg[len];for(int i=0;i<len;++i){Ptg ptg=ptgs[i];switch(formulaType){
        let ret=Rc::new(RefCell::new(vec![None;len].into_boxed_slice()));let mut i=0;while i<len{let ptg=ptgs.borrow()[i].clone();match formulaType{
            // java case 0:ret[i]=ptgs[i];break;
            0=>ret.borrow_mut()[i]=ptg,
            // java case 4:
            4=>{
                let ptg=ptg.expect("NullPointerException");let value=ptg.borrow();
                // java if(ptg.getSid()==58){Ref3DPtg rptg=(Ref3DPtg)ptg;
                if value.getSid()==58{
                    let PtgKind::Ref3D{col_raw,..}=&value.kind else{panic!("ClassCastException");};
                    // java if(rptg.isRowRelative()||rptg.isColRelative()){rptg=(Ref3DPtg)rptg.clone();
                    let rptg=if row_rel(*col_raw)||col_rel(*col_raw){let mut copy=value.clone();copy.java_identity=Ptg::newIdentity();let PtgKind::Ref3D{row,col_raw,..}=&mut copy.kind else{unreachable!()};
                        // java if(rptg.isRowRelative()){rptg.setRow((rptg.getRow()&relative_row_sign_bit)==0?rptg.getRow():rptg.getRow()+max_row_delta);}
                        if row_rel(*col_raw){*row=if *row&relative_row_sign_bit==0{*row}else{row.wrapping_add(max_row_delta)};}
                        // java if(rptg.isColRelative()){rptg.setColumn((rptg.getColumn()&relative_col_sign_bit)==0?rptg.getColumn():rptg.getColumn()+max_col_delta);}
                        if col_rel(*col_raw){let col=col_index(*col_raw);let col=if col&relative_col_sign_bit==0{col}else{col.wrapping_add(max_col_delta)};*col_raw=(*col_raw&0xc000)|(col as u16&0x3fff);}
                        Rc::new(RefCell::new(copy))
                    }else{ptg.clone()};
                    // java ret[i]=rptg;
                    ret.borrow_mut()[i]=Some(rptg);
                // java }else if(ptg.getSid()==59){Area3DPtg aptg=(Area3DPtg)ptg;
                }else if value.getSid()==59{
                    let PtgKind::Area3D{first_col_raw,last_col_raw,..}=&value.kind else{panic!("ClassCastException");};
                    // java if(aptg.isFirstRowRelative()||aptg.isLastRowRelative()||aptg.isFirstColRelative()||aptg.isLastColRelative()){aptg=(Area3DPtg)aptg.clone();
                    let aptg=if row_rel(*first_col_raw)||row_rel(*last_col_raw)||col_rel(*first_col_raw)||col_rel(*last_col_raw){let mut copy=value.clone();copy.java_identity=Ptg::newIdentity();let PtgKind::Area3D{first_row,last_row,first_col_raw,last_col_raw,..}=&mut copy.kind else{unreachable!()};
                        // java if(aptg.isFirstRowRelative()){aptg.setFirstRow((aptg.getFirstRow()&relative_row_sign_bit)==0?aptg.getFirstRow():aptg.getFirstRow()+max_row_delta);}
                        if row_rel(*first_col_raw){*first_row=if *first_row&relative_row_sign_bit==0{*first_row}else{first_row.wrapping_add(max_row_delta)};}
                        // java if(aptg.isLastRowRelative()){aptg.setLastRow((aptg.getLastRow()&relative_row_sign_bit)==0?aptg.getLastRow():aptg.getLastRow()+max_row_delta);}
                        if row_rel(*last_col_raw){*last_row=if *last_row&relative_row_sign_bit==0{*last_row}else{last_row.wrapping_add(max_row_delta)};}
                        // java if(aptg.isFirstColRelative()){aptg.setFirstColumn((aptg.getFirstColumn()&relative_col_sign_bit)==0?aptg.getFirstColumn():aptg.getFirstColumn()+max_col_delta);}
                        if col_rel(*first_col_raw){let col=col_index(*first_col_raw);let col=if col&relative_col_sign_bit==0{col}else{col.wrapping_add(max_col_delta)};*first_col_raw=(*first_col_raw&0xc000)|(col as u16&0x3fff);}
                        // java if(aptg.isLastColRelative()){aptg.setLastColumn((aptg.getLastColumn()&relative_col_sign_bit)==0?aptg.getLastColumn():aptg.getLastColumn()+max_col_delta);}
                        if col_rel(*last_col_raw){let col=col_index(*last_col_raw);let col=if col&relative_col_sign_bit==0{col}else{col.wrapping_add(max_col_delta)};*last_col_raw=(*last_col_raw&0xc000)|(col as u16&0x3fff);}
                        Rc::new(RefCell::new(copy))
                    }else{ptg.clone()};
                    // java ret[i]=aptg;
                    ret.borrow_mut()[i]=Some(aptg);
                // java }else{ret[i]=ptg;}break;
                }else{ret.borrow_mut()[i]=Some(ptg.clone());}
            }
            // java default:throw new IllegalArgumentException("not implemented");
            _=>panic!("IllegalArgumentException: not implemented"),
        }i+=1;}
        // java return ret;
        Some(ret)
    }

    // java public static Ptg[] convertFromExcel2007ToExcel97(Ptg[] ptgs, int formulaType, boolean isConvertForWPS)
    pub fn convertFromExcel2007ToExcel97(
        ptgs: &[Ptg],
        formula_type: i32,
        is_convert_for_wps: bool,
    ) -> Vec<Ptg> {
        // java int len = ptgs.length;
        let len = ptgs.len();
        let mut ret = Vec::with_capacity(len);
        // java for (int i = 0; i < len; ++i)
        let mut i = 0;
        while i < len {
            let ptg = &ptgs[i];
            // java int sid = ptg.getSid();
            let sid = ptg.getSid();
            match formula_type {
                0 | 3 => {
                    if sid == 36 || sid == 44 || sid == 58 {
                        ret.push(adjust_ref_ptg_base(ptg, is_convert_for_wps));
                    } else if sid == 37 || sid == 45 || sid == 59 {
                        ret.push(adjust_area_ptg_base(ptg, is_convert_for_wps));
                    } else {
                        ret.push(ptg.clone());
                    }
                }
                4 => {
                    // java formulaType 4 只改 sid 58/59 的行列掩码，相对标志留在列字里。
                    ret.push(mask_formula_type_four(ptg, is_convert_for_wps));
                }
                _ => panic!("IllegalArgumentException: not implemented"),
            }
            i += 1;
        }
        ret
    }

    // java public static boolean fmlCanBeSharedfml(Ptg[] ptgs) {
    pub fn fmlCanBeSharedfml(ptgs: &[Ptg]) -> bool {
        // java int i = 0;
        let mut i = 0;
        // java for(int len = ptgs.length; i < len; ++i) {
        let len = ptgs.len();
        while i < len {
            // java if (ptgs[i].getSid() == 58 || ptgs[i].getSid() == 59) { return false; }
            if ptgs[i].getSid() == 58 || ptgs[i].getSid() == 59 { return false; }
            i += 1;
        }
        // java return true;
        true
    }

    // java public static Ptg[] replaceWithNPtgs(Ptg[] ptgs, int row, int col, SpreadsheetVersion sv)
    pub fn replaceWithNPtgs(ptgs: &[Ptg], row: i32, col: i32, sv: SpreadsheetVersion) -> Vec<Ptg> {
        // java int len = ptgs.length;
        let len = ptgs.len();
        let mut ret = Vec::with_capacity(len);
        // java for (int i = 0; i < len; ++i)
        let mut i = 0;
        while i < len {
            ret.push(replace_with_n_ptg(&ptgs[i], row, col, sv));
            i += 1;
        }
        ret
    }
}

fn mask_formula_type_four(ptg: &Ptg, is_convert_for_wps: bool) -> Ptg {
    let limit = if is_convert_for_wps {
        SpreadsheetVersion::EXCEL2007
    } else {
        SpreadsheetVersion::EXCEL97
    };
    let row_mask = limit.getMaxRows() - 1;
    let col_mask = limit.getMaxColumns() - 1;
    let mut out = ptg.clone();
    match &mut out.kind {
        // java formulaType 4 casts sid 58/59 to Ref3DPtg/Area3DPtg before cloning.
        PtgKind::Ref3DExt { .. } | PtgKind::Area3DExt { .. } => panic!("ClassCastException"),
        PtgKind::Ref3D { row, col_raw, .. } => {
            *row &= row_mask;
            let rel_row = row_rel(*col_raw);
            let rel_col = col_rel(*col_raw);
            *col_raw = pack_col(col_index(*col_raw) & col_mask, rel_row, rel_col);
        }
        PtgKind::Area3D {
            first_row,
            last_row,
            first_col_raw,
            last_col_raw,
            ..
        } => {
            *first_row &= row_mask;
            *last_row &= row_mask;
            *first_col_raw = pack_col(
                col_index(*first_col_raw) & col_mask,
                row_rel(*first_col_raw),
                col_rel(*first_col_raw),
            );
            *last_col_raw = pack_col(
                col_index(*last_col_raw) & col_mask,
                row_rel(*last_col_raw),
                col_rel(*last_col_raw),
            );
        }
        _ => {}
    }
    out
}

// java private static Ptg adjustRefPtgBase(RefPtgBase ptg, boolean isConvertForWPS)
fn adjust_ref_ptg_base(ptg: &Ptg, is_convert_for_wps: bool) -> Ptg {
    let (row, column) = match &ptg.kind {
        PtgKind::Ref { row, col_raw, .. } | PtgKind::Ref3D { row, col_raw, .. }
        | PtgKind::Ref3DExt { row, col_raw, .. } => {
            (*row, col_index(*col_raw))
        }
        _ => panic!("ClassCastException"),
    };
    let limit = if is_convert_for_wps {
        SpreadsheetVersion::EXCEL2007
    } else {
        SpreadsheetVersion::EXCEL97
    };
    // java ptg.getRow() < maxRows && ptg.getColumn() < maxColumns ? ptg : new RefErrorPtg()
    if row < limit.getMaxRows() && column < limit.getMaxColumns() {
        ptg.clone()
    } else {
        ref_error_ptg()
    }
}

// java private static Ptg adjustAreaPtgBase(AreaPtgBase ptg, boolean isConvertForWPS)
fn adjust_area_ptg_base(ptg: &Ptg, is_convert_for_wps: bool) -> Ptg {
    let max2007_rows = SpreadsheetVersion::EXCEL2007.getMaxRows();
    let max2007_cols = SpreadsheetVersion::EXCEL2007.getMaxColumns();
    let (row_limit, col_limit, full_last_row, full_last_col) = if is_convert_for_wps {
        (max2007_rows, max2007_cols, max2007_rows - 1, max2007_cols - 1)
    } else {
        (
            SpreadsheetVersion::EXCEL97.getMaxRows(),
            SpreadsheetVersion::EXCEL97.getMaxColumns(),
            SpreadsheetVersion::EXCEL97.getMaxRows() - 1,
            SpreadsheetVersion::EXCEL97.getMaxColumns() - 1,
        )
    };
    let mut out = ptg.clone();
    let ok = match &mut out.kind {
        PtgKind::Area {
            first_row,
            last_row,
            first_col_raw,
            last_col_raw,
            ..
        }
        | PtgKind::Area3D {
            first_row,
            last_row,
            first_col_raw,
            last_col_raw,
            ..
        }
        | PtgKind::Area3DExt {
            first_row,
            last_row,
            first_col_raw,
            last_col_raw,
            ..
        } => {
            // java 整行/整列从 Excel2007 上限改到目标版本上限
            if *first_row == 0 && *last_row == max2007_rows - 1 {
                *last_row = full_last_row;
            }
            if col_index(*first_col_raw) == 0 && col_index(*last_col_raw) == max2007_cols - 1 {
                *last_col_raw = pack_col(full_last_col, row_rel(*last_col_raw), col_rel(*last_col_raw));
            }
            *first_row < row_limit
                && *last_row < row_limit
                && col_index(*first_col_raw) < col_limit
                && col_index(*last_col_raw) < col_limit
        }
        _ => panic!("ClassCastException"),
    };
    if ok {
        out
    } else {
        area_err_ptg()
    }
}

// java public static Ptg replaceWithNPtg(Ptg ptg, int row, int col, SpreadsheetVersion sv)
fn replace_with_n_ptg(ptg: &Ptg, row: i32, col: i32, sv: SpreadsheetVersion) -> Ptg {
    let max_rows = sv.getMaxRows();
    let max_cols = sv.getMaxColumns();
    // rust: sid 36 → RefNPtg，sid 37 → AreaNPtg。sid 58/59 的 3D 分支还没有对应 token。
    match &ptg.kind {
        // java replaceWithNPtg sid 58/59 casts to Ref3DPtg/Area3DPtg.
        PtgKind::Ref3DExt { .. } | PtgKind::Area3DExt { .. } => panic!("ClassCastException"),
        PtgKind::Ref {
            row: ref_row,
            col_raw,
            wide,
            relative: false,
        } => {
            let rel_row = row_rel(*col_raw);
            let rel_col = col_rel(*col_raw);
            let new_row = if rel_row {
                (ref_row + max_rows - row).rem_euclid(max_rows)
            } else {
                *ref_row
            };
            let new_col = if rel_col {
                (col_index(*col_raw) + max_cols - col).rem_euclid(max_cols)
            } else {
                col_index(*col_raw)
            };
            Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
                kind: PtgKind::Ref {
                    row: new_row,
                    col_raw: pack_col(new_col, rel_row, rel_col),
                    wide: *wide,
                    relative: true,
                },
                class: ptg.class,
            }
        }
        PtgKind::Ref3D {
            extern_index,
            row: ref_row,
            col_raw,
            wide,
        } => {
            // java replaceWithNPtg：相对引用才新建 Ref3D，且不复制 ptg class。
            let rel_row = row_rel(*col_raw);
            let rel_col = col_rel(*col_raw);
            if !rel_row && !rel_col {
                return ptg.clone();
            }
            let new_row = if rel_row {
                (ref_row + max_rows - row).rem_euclid(max_rows)
            } else {
                *ref_row
            };
            let new_col = if rel_col {
                (col_index(*col_raw) + max_cols - col).rem_euclid(max_cols)
            } else {
                col_index(*col_raw)
            };
            return Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
                kind: PtgKind::Ref3D {
                    extern_index: *extern_index,
                    row: new_row,
                    col_raw: pack_col(new_col, rel_row, rel_col),
                    wide: *wide,
                },
                class: 0,
            };
        }
        PtgKind::Area {
            first_row,
            last_row,
            first_col_raw,
            last_col_raw,
            wide,
            relative: false,
        } => {
            let adj = |value: i32, master: i32, rel: bool, max: i32| {
                if rel {
                    (value + max - master).rem_euclid(max)
                } else {
                    value
                }
            };
            Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
                kind: PtgKind::Area {
                    first_row: adj(*first_row, row, row_rel(*first_col_raw), max_rows),
                    last_row: adj(*last_row, row, row_rel(*last_col_raw), max_rows),
                    first_col_raw: pack_col(
                        adj(col_index(*first_col_raw), col, col_rel(*first_col_raw), max_cols),
                        row_rel(*first_col_raw),
                        col_rel(*first_col_raw),
                    ),
                    last_col_raw: pack_col(
                        adj(col_index(*last_col_raw), col, col_rel(*last_col_raw), max_cols),
                        row_rel(*last_col_raw),
                        col_rel(*last_col_raw),
                    ),
                    wide: *wide,
                    relative: true,
                },
                class: ptg.class,
            }
        }
        PtgKind::Area3D {
            extern_index,
            first_row,
            last_row,
            first_col_raw,
            last_col_raw,
            wide,
        } => {
            let rel = row_rel(*first_col_raw)
                || row_rel(*last_col_raw)
                || col_rel(*first_col_raw)
                || col_rel(*last_col_raw);
            if !rel {
                return ptg.clone();
            }
            let adj = |value: i32, master: i32, relative: bool, max: i32| {
                if relative {
                    (value + max - master).rem_euclid(max)
                } else {
                    value
                }
            };
            Ptg { java_identity:crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg::newIdentity(),
                kind: PtgKind::Area3D {
                    extern_index: *extern_index,
                    first_row: adj(*first_row, row, row_rel(*first_col_raw), max_rows),
                    last_row: adj(*last_row, row, row_rel(*last_col_raw), max_rows),
                    first_col_raw: pack_col(
                        adj(col_index(*first_col_raw), col, col_rel(*first_col_raw), max_cols),
                        row_rel(*first_col_raw),
                        col_rel(*first_col_raw),
                    ),
                    last_col_raw: pack_col(
                        adj(col_index(*last_col_raw), col, col_rel(*last_col_raw), max_cols),
                        row_rel(*last_col_raw),
                        col_rel(*last_col_raw),
                    ),
                    wide: *wide,
                },
                class: 0,
            }
        }
        _ => ptg.clone(),
    }
}
