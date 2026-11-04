// java package cn.wps.moss.engine.grid;

use crate::java_runtime::RefCell;
use crate::java_runtime::{Rc,Weak};
use crate::cn::wps::moss::{app::KmoSheet::KmoSheet,util::{KmoRect::KmoRect,KmoUndoableRTree::KmoUndoableRTree,cache::MRULongKeyCache::MRULongKeyCache,AppLogicHelper::AppLogicHelper}};
static mru_cache:std::sync::Mutex<Option<Rc<MRULongKeyCache<Rc<ExpFml>>>>>=std::sync::Mutex::new(None);

use crate::cn::wps::et::ss::formula::Formula::Formula;
use crate::cn::wps::et::ss::formula::ptg::Ptg::Ptg;
// java import cn.wps.et.ss.calcchain.FormulaHelper;
use crate::cn::wps::et::ss::calcchain::FormulaHelper::FormulaHelper;
use crate::org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion;

#[cfg(test)]
#[path = "ExpFmlManager.review.rs"]
mod review;

#[derive(Clone)]
pub struct ExpFml {
    pub firstRow: i32,
    pub lastRow: i32,
    pub firstCol: i32,
    pub lastCol: i32,
    pub fml: Formula,
    // java public KmoSheet sheet;
    sheet:Option<Weak<RefCell<KmoSheet>>>,
    cells: RefCell<Vec<i64>>,
}

impl ExpFml {
    // java public SpreadsheetVersion getVersion() { return this.sheet != null ? this.sheet.getVersion() : SpreadsheetVersion.EXCEL97; }
    pub fn getVersion(&self)->SpreadsheetVersion{self.sheet.as_ref().and_then(Weak::upgrade).map_or(SpreadsheetVersion::EXCEL97,|s|s.borrow().getVersion())}
    // java public void addShareCell(int row, int col) { long pos = (long)row + ((long)col << 21); this.cells.add(pos); }
    pub fn addShareCell__2(&self,row:i32,col:i32){let pos=(row as i64).wrapping_add((col as i64)<<21);self.cells.borrow_mut().push(pos);}
    // java public final void addShareCell(long pos, int row, int col)
    pub fn addShareCell(&self, pos: i64, _row: i32, _col: i32) {
        // java this.cells.add(pos);
        self.cells.borrow_mut().push(pos);
    }

    // java public byte[] getEncodedTokens()
    pub fn getEncodedTokens(&self) -> Vec<u8> {
        // java return this.fml.getByteEncoding();
        // Native owned transport at the existing byte-token interface.
        self.fml.getByteEncoding().expect("NullPointerException").borrow().to_vec()
    }

    // java public int getEncodedTokensLen()
    pub fn getEncodedTokensLen(&self) -> i32 {
        // java return this.fml.getEncodedTokenSize();
        self.fml.getEncodedTokenSize()
    }

    // java public Ptg[] getPtgs()
    pub fn getPtgs(&self) -> Vec<Ptg> {
        // java return this.fml.getTokens();
        crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&self.getPtgs__reference())
    }
    // java public Ptg[] getPtgs() { return this.fml.getTokens(); }
    // Native array-reference transport of the same original getter.
    pub fn getPtgs__reference(&self)->crate::cn::wps::et::ss::formula::ptg::Ptg::PtgArray{self.fml.getTokens__reference()}

    // java public boolean isBelongThisShareFml(int row, int col, KmoSheet sheet)
    pub fn isBelongThisShareFml(&self, row: i32, col: i32) -> bool {
        // java if (row >= this.firstRow && row <= this.lastRow && col >= this.firstCol && col <= this.lastCol)
        if row >= self.firstRow && row <= self.lastRow && col >= self.firstCol && col <= self.lastCol {
            // java int pos = 0;
            let mut pos = 0;
            // java for(int size = this.cells.size(); pos < size; ++pos)
            let cells = self.cells.borrow();
            let size = cells.len();
            while pos < size {
                // java int subRow = (int)(this.cells.get(pos) & 1048575L);
                let cell_pos = cells[pos];
                let sub_row = (cell_pos & 1_048_575) as i32;
                // java int subCol = (int)(this.cells.get(pos) >> 21 & 65535L);
                let sub_col = ((cell_pos >> 21) & 65_535) as i32;
                // java if (subRow == row && subCol == col) return true;
                if sub_row == row && sub_col == col {
                    return true;
                }
                pos += 1;
            }
            // java return false;
            false
        } else {
            // java return false;
            false
        }
    }
}

pub struct ExpFmlManager {
    // java private KmoSheet sheet;
    sheet:Option<Weak<RefCell<KmoSheet>>>,
    // java private KmoUndoableRTree<ExpFml> arrFmls = new KmoUndoableRTree<ExpFml>();
    arrFmls:KmoUndoableRTree<ExpFml>,
    // java private KmoUndoableRTree<ExpFml> sharedFmls = new KmoUndoableRTree<ExpFml>();
    sharedFmls:KmoUndoableRTree<ExpFml>,
}

impl ExpFmlManager {
    // java public static Ptg[] adjustSharedFormula(ExpFml shrfml, int row, int col, SpreadsheetVersion ss)
    pub fn adjustSharedFormula(shrfml: &ExpFml, row: i32, col: i32, ss: SpreadsheetVersion) -> Vec<Ptg> {
        // java return FormulaHelper.replaceNPtgs(shrfml.getPtgs(), row, col, ss);
        crate::cn::wps::et::ss::formula::ptg::Ptg::ptgValues(&Self::adjustSharedFormula__reference(Some(shrfml),row,col,Some(ss)))
    }
    // java public static Ptg[] adjustSharedFormula(ExpFml shrfml, int row, int col, SpreadsheetVersion ss) { return FormulaHelper.replaceNPtgs(shrfml.getPtgs(), row, col, ss); }
    pub fn adjustSharedFormula__reference(shrfml:Option<&ExpFml>,row:i32,col:i32,ss:Option<SpreadsheetVersion>)->crate::cn::wps::et::ss::formula::ptg::Ptg::PtgArray{let ptgs=shrfml.expect("NullPointerException").getPtgs__reference();FormulaHelper::replaceNPtgs__array(Some(&ptgs),row,col,ss)}

    pub fn ExpFmlManager() -> Self {
        Self {
            sheet:None,
            arrFmls:KmoUndoableRTree::new(),
            sharedFmls:KmoUndoableRTree::new(),
        }
    }
    // java public ExpFmlManager(KmoSheet sheet) { this.sheet = sheet; }
    pub fn ExpFmlManager__sheet(sheet:Weak<RefCell<KmoSheet>>)->Self{let mut value=Self::ExpFmlManager();value.sheet=Some(sheet);value}

    // java public ExpFml registerSharedFml(int firstRow, int lastRow, int firstCol, int lastCol, Formula fml)
    pub fn registerSharedFml(
        &self,
        firstRow: i32,
        lastRow: i32,
        firstCol: i32,
        lastCol: i32,
        fml: Formula,
    ) -> Rc<ExpFml> {
        // java ExpFml efml = new ExpFml(firstRow, lastRow, firstCol, lastCol, fml.copy(), this.sheet);
        let efml = Rc::new(ExpFml {
            firstRow,
            lastRow,
            firstCol,
            lastCol,
            fml: fml.copy(),
            sheet:self.sheet.clone(),
            cells: RefCell::new(Vec::new()),
        });
        // java this.sharedFmls.insert(new KmoRect(firstRow, firstCol, lastRow, lastCol), efml);
        self.sharedFmls.insert(&KmoRect::KmoRect__bounds(firstRow,firstCol,lastRow,lastCol),Rc::clone(&efml));
        efml
    }

    // java public ExpFml registerArrayFml(...)
    pub fn registerArrayFml(
        &self,
        firstRow: i32,
        lastRow: i32,
        firstCol: i32,
        lastCol: i32,
        fml: Formula,
    ) -> Rc<ExpFml> {
        // java ExpFml efml = new ExpFml(..., fml.copy(), this.sheet);
        let efml = Rc::new(ExpFml {
            firstRow,
            lastRow,
            firstCol,
            lastCol,
            fml: fml.copy(),
            sheet:self.sheet.clone(),
            cells: RefCell::new(Vec::new()),
        });
        // java this.arrFmls.insert(new KmoRect(firstRow, firstCol, lastRow, lastCol), efml);
        self.arrFmls.insert(&KmoRect::KmoRect__bounds(firstRow,firstCol,lastRow,lastCol),Rc::clone(&efml));
        efml
    }

    pub fn shared_count(&self) -> usize {
        let mut out=vec![];self.sharedFmls.lookup(&KmoRect::KmoRect__bounds(0,0,1048575,16383),&mut out);out.len()
    }

    // java public List<ExpFml> getMatchedSharedFmls(int masterCellRow, int masterCellCol)
    pub fn getMatchedSharedFmls(&self, row: i32, col: i32) -> Vec<Rc<ExpFml>> {
        // java KmoRect rect = SharedObjects.kmoRectPool.borrowObject(); List<ExpFml> rs = new ArrayList();
        let rect=KmoRect::KmoRect__bounds(row,col,row,col);let mut rs=vec![];
        // java this.sharedFmls.lookup((KmoRect)rect, rs); SharedObjects.kmoRectPool.returnObject(rect); return rs;
        self.sharedFmls.lookup(&rect,&mut rs);rs
    }

    // java public ExpFml query(int masterCellRow, int masterCellCol, boolean isSharedFml)
    pub fn query(&self, row: i32, col: i32, is_shared: bool) -> Option<Rc<ExpFml>> {
        // java long key = AppLogicHelper.cellPos2Long(this.sheet.getSheetUid(), masterCellRow, masterCellCol);
        let sheet=self.sheet.as_ref().and_then(Weak::upgrade);let uid=sheet.as_ref().map_or(0,|s|s.borrow().getSheetUid());
        let key=AppLogicHelper::cellPos2Long(uid,row,col);
        // java ExpFml expfml = mru_cache == null ? null : (ExpFml)mru_cache.get(key); if (expfml != null) { return expfml; }
        let cache=mru_cache.lock().unwrap().clone();if let Some(hit)=cache.as_ref().and_then(|c|c.get(key)){return Some(hit);}
        // java ExpFml fml = (ExpFml)this.sheet.sharedFormulaMap.get(masterCellRow << 14 | masterCellCol & 16383); if (fml != null) { return fml; }
        if let Some(s)=sheet{if let Some(f)=s.borrow().sharedFormulaMap.borrow().get(&(row.wrapping_shl(14)|(col&16383))).cloned(){return Some(f);}}
        // java KmoRect rect = SharedObjects.kmoRectPool.borrowObject(); try { KmoUndoableRTree<ExpFml> fmls = isSharedFml ? this.sharedFmls : this.arrFmls;
        let rect=KmoRect::KmoRect__bounds(row,col,row,col);let mut hits=vec![];
        // java List<ExpFml> rs = new ArrayList(); rect.set(masterCellRow, masterCellCol, masterCellRow, masterCellCol); fmls.lookup((KmoRect)rect, rs);
        self.query__rect(&rect,is_shared,&mut hits);
        // java if (rs != null && rs.size() >= 1)
        if hits.is_empty() {
            return None;
        }
        // java if (isSharedFml && cand.isBelongThisShareFml(...)) return cand;
        if is_shared {
            for cand in &hits {
                if cand.isBelongThisShareFml(row, col) {
                    return Some(Rc::clone(cand));
                }
            }
        }
        // java if (cand.firstRow == masterCellRow && cand.firstCol == masterCellCol) return cand;
        for cand in &hits {
            if cand.firstRow == row && cand.firstCol == col {
                // java if (mru_cache != null) { mru_cache.put(key, cand); }
                if let Some(cache)=&cache{cache.put(key,Some(cand.clone()));}
                return Some(Rc::clone(cand));
            }
        }
        // java ExpFml first = rs.get(0); if (isSharedFml) first = registerSharedFml(...)
        let first = Rc::clone(&hits[0]);
        if is_shared {
            Some(self.registerSharedFml(
                row,
                first.lastRow,
                col,
                first.lastCol,
                first.fml.copy(),
            ))
        } else {
            Some(first)
        }
    }

    // java public void unregister(int masterCellRow, int masterCellCol, boolean isSharedFml) {
    pub fn unregister(&self,row:i32,col:i32,isSharedFml:bool){
        // java ExpFml fml = this.query(masterCellRow, (short)masterCellCol, isSharedFml); if (fml != null) { this.unregister(fml, isSharedFml); }
        if let Some(fml)=self.query(row,col as i16 as i32,isSharedFml){self.unregister__fml(fml,isSharedFml);}
    }
    // java public void unregister(ExpFml fml, boolean isSharedFml) {
    pub fn unregister__fml(&self,fml:Rc<ExpFml>,isSharedFml:bool){
        // java KmoUndoableRTree<ExpFml> fmls = isSharedFml ? this.sharedFmls : this.arrFmls;
        let fmls=if isSharedFml{&self.sharedFmls}else{&self.arrFmls};
        // java fmls.remove(new KmoRect(fml.firstRow, fml.firstCol, fml.lastRow, fml.lastCol), fml);
        let rect=KmoRect::KmoRect__bounds(fml.firstRow,fml.firstCol,fml.lastRow,fml.lastCol);fmls.remove(&rect,fml);
        // java if (mru_cache != null) { mru_cache.clear(); }
        if let Some(cache)=mru_cache.lock().unwrap().as_ref(){cache.clear();}
    }
    // java public static void enableCache() { mru_cache = new MRULongKeyCache<ExpFml>(100); }
    pub fn enableCache(){*mru_cache.lock().unwrap()=Some(Rc::new(MRULongKeyCache::new(100)));}
    // java public static void disableCache() { mru_cache = null; }
    pub fn disableCache(){*mru_cache.lock().unwrap()=None;}
    // java public boolean isMultiArrayFml(int masterCellRow, int masterCellCol) {
    pub fn isMultiArrayFml(&self,row:i32,col:i32)->bool{
        // java ExpFml expFml = this.query(masterCellRow, masterCellCol, false); return expFml != null && (expFml.firstRow != expFml.lastRow || expFml.firstCol != expFml.lastCol);
        self.query(row,col,false).is_some_and(|f|f.firstRow!=f.lastRow||f.firstCol!=f.lastCol)
    }
    // java public void query(KmoRect rect, boolean isSharedFml, List<ExpFml> out) {
    pub fn query__rect(&self,rect:&KmoRect,isSharedFml:bool,out:&mut Vec<Rc<ExpFml>>){
        // java if (isSharedFml) { this.sharedFmls.lookup((KmoRect)rect, out); } else { this.arrFmls.lookup((KmoRect)rect, out); }
        if isSharedFml{self.sharedFmls.lookup(rect,out);}else{self.arrFmls.lookup(rect,out);}
    }
    // java public void unlinkSharedFormula(ExpFml shrfml) {
    pub fn unlinkSharedFormula(&self,shrfml:Rc<ExpFml>){
        use crate::cn::wps::{moss::engine::eval::{EvaluationWorkbook::EvaluationWorkbook,EvaluationCellFormula::EvaluationCellFormula},et::ss::interfaces::IEvalFormulaBase::IEvalFormulaBase};
        use super::{KmoCellData::KmoCellData,KmoFormulaData::KmoFormulaData};
        let sheet=self.sheet.as_ref().and_then(Weak::upgrade).expect("KmoSheet");let s=sheet.borrow();let book=s.getBook();
        // java CalculationChain cc = this.sheet.getBook().getCalculationChain(); SpreadsheetVersion ss = this.sheet.getBook().getSpreadsheetVersion();
        let cc=book.getCalculationChain().expect("CalculationChain");let ss=book.getSpreadsheetVersion();
        // java Dependent dep = this.sheet.getBook().getDependent(); EvaluationWorkbook cw = new EvaluationWorkbook(this.sheet.getBook());
        let dep=book.getDependent().expect("Dependent");let cw=EvaluationWorkbook::EvaluationWorkbook(&book);
        // java KmoCellData cell = new KmoCellData(); this.sheet.getCellData(shrfml.firstRow, shrfml.firstCol, cell);
        let mut cell=KmoCellData::default();s.getCellData__2(shrfml.firstRow,shrfml.firstCol,&mut cell);
        // java if (cell.formulaType == 3) { dep.unregisterFml(new EvaluationCellFormula(this.sheet, shrfml.firstRow, shrfml.firstCol), cw); }
        if cell.formulaType==3{let f:Rc<dyn IEvalFormulaBase>=Rc::new(EvaluationCellFormula::EvaluationCellFormula(sheet.clone(),shrfml.firstRow,shrfml.firstCol));dep.borrow_mut().unregisterFml(&f,&cw);}
        // java KmoCellValueStore valueStore = this.sheet.getCellValueStore(); KmoFormulaData fml = new KmoFormulaData(); int sheetId = this.sheet.getSheetIndex(); int row = shrfml.firstRow;
        let valueStore=s.getCellValueStore();let mut fml=KmoFormulaData::KmoFormulaData();let sheetId=s.getSheetIndex();
        // java for(int lastrow = shrfml.lastRow; row <= lastrow; ++row) { int col = shrfml.firstCol; for(int lastcol = shrfml.lastCol; col <= lastcol; ++col) {
        for row in shrfml.firstRow..=shrfml.lastRow{for col in shrfml.firstCol..=shrfml.lastCol{
            // java this.sheet.getCellData(row, col, cell); if (cell.formulaType == 3) {
            s.getCellData__2(row,col,&mut cell);if cell.formulaType==3{
                // java valueStore.formulaValue(cell.id, fml); byte[] tokens = valueStore.formulaTokens(fml.getTokensId()); Ptg[] ptgs = Ptg.readTokens(fml.getTokensPlainSize(), tokens, this.sheet.getBook().getSpreadsheetVersion());
                valueStore.borrow().formulaValue(cell.id,&mut fml);let tokens=valueStore.borrow().formulaTokens(fml.getTokensId());let ptgs=Ptg::readTokens(fml.getTokensPlainSize(),&tokens,ss);
                // java if (ptgs.length == 1 && ptgs[0].getSid() == 1) {
                if ptgs.len()==1&&ptgs[0].getSid()==1{
                    // java cc.unregisterFormula(sheetId, row, col);
                    cc.unregisterFormula(sheetId,row,col);
                    // java ExpFml realExpFml = this.sheet.getExpFmlManager().query(row, col, true); Ptg[] newptgs = adjustSharedFormula(realExpFml, row, col, ss);
                    let realExpFml=self.query(row,col,true).expect("ExpFml");let newptgs=Self::adjustSharedFormula(&realExpFml,row,col,ss);
                    // java tokens = new byte[Ptg.getEncodedSize(newptgs)]; Ptg.serializePtgs(newptgs, tokens, 0);
                    let tokens=crate::cn::wps::et::ss::formula::ptg::Ptg::serializePtgs(&newptgs);
                    // java fml.setTokens(valueStore.newFormulaTokens(tokens), Ptg.getEncodedSizeWithoutArrayData(newptgs)); int fmlid = valueStore.newFormulaValue(fml);
                    fml.setTokens(valueStore.borrow_mut().newFormulaTokens(Some(&tokens)),crate::cn::wps::et::ss::formula::ptg::Ptg::getEncodedSizeWithoutArrayData(&newptgs));let fmlid=valueStore.borrow_mut().newFormulaValue(&fml);
                    // java cell.id = fmlid; cell.formulaType = 1; this.sheet.setCellData(row, col, cell);
                    cell.id=fmlid;cell.formulaType=1;s.setCellData(row,col,&cell);
                    // java EvaluationCellFormula cellfml = new EvaluationCellFormula(this.sheet, row, col); cellfml.updateTokens(newptgs, false); cc.registerFormula(cellfml);
                    let cellfml=EvaluationCellFormula::EvaluationCellFormula(sheet.clone(),row,col);cellfml.updateTokens(&newptgs,false);cc.registerFormula__formula(Rc::new(cellfml));
                }
            }
        }}
        // java cc.markRecalculation(sheetId, shrfml.firstRow, shrfml.lastRow, shrfml.firstCol, shrfml.lastCol); this.unregister(shrfml, true);
        cc.markRecalculation__area(sheetId,shrfml.firstRow,shrfml.lastRow,shrfml.firstCol,shrfml.lastCol,true);self.unregister__fml(shrfml,true);
    }
    // java public void unlinkSharedFormula(KmoSheet sheet, int masterCellRow, short masterCellCol) {
    pub fn unlinkSharedFormula__pos(&self,_sheet:&Rc<RefCell<KmoSheet>>,row:i32,col:i16){
        // java ExpFml shrfml = this.query(masterCellRow, masterCellCol, true); if (shrfml != null) { this.unlinkSharedFormula(shrfml); }
        if let Some(shrfml)=self.query(row,col as i32,true){self.unlinkSharedFormula(shrfml);}
    }
    // java public void unlinkSharedFormulaByCellPos(KmoSheet sheet, int row, int col) {
    pub fn unlinkSharedFormulaByCellPos(&self,sheet:&Rc<RefCell<KmoSheet>>,row:i32,col:i32){
        use super::{KmoCellData::KmoCellData,KmoFormulaData::KmoFormulaData};
        // java KmoCellData cell = new KmoCellData(); sheet.getCellData(row, col, cell); if (cell.formulaType != 3) { throw new IllegalArgumentException(); }
        let mut cell=KmoCellData::default();let s=sheet.borrow();s.getCellData__2(row,col,&mut cell);assert_eq!(cell.formulaType,3,"IllegalArgumentException");
        // java KmoCellValueStore cvs = sheet.getCellValueStore(); KmoFormulaData fml = new KmoFormulaData(); cvs.formulaValue(cell.id, fml);
        let cvs=s.getCellValueStore();let mut fml=KmoFormulaData::KmoFormulaData();cvs.borrow().formulaValue(cell.id,&mut fml);
        // java Ptg[] ptgs = Ptg.readTokens(fml.getTokensPlainSize(), cvs.formulaTokens(fml.getTokensId()), sheet.getVersion());
        let ptgs=Ptg::readTokens(fml.getTokensPlainSize(),&cvs.borrow().formulaTokens(fml.getTokensId()),s.getVersion());
        // java if (ptgs.length == 1 && ptgs[0].getSid() == 1) { ExpPtg eptg = (ExpPtg)ptgs[0]; ExpFml shrfml = this.query(eptg.getRow(), (short)eptg.getColumn(), true);
        if ptgs.len()==1&&ptgs[0].getSid()==1{
            let crate::cn::wps::et::ss::formula::ptg::Ptg::PtgKind::Exp{row,col,..}=ptgs[0].kind else{panic!("ClassCastException");};
            // java if (shrfml == null) { throw new IllegalStateException(); } else { this.unlinkSharedFormula(shrfml); }
            let shrfml=self.query(row,col as i16 as i32,true).expect("IllegalStateException");self.unlinkSharedFormula(shrfml);
        // java } else { throw new IllegalStateException(); }
        }else{panic!("IllegalStateException");}
    }
    // java public void unlinkSharedFormula(KmoRect range) {
    pub fn unlinkSharedFormula__range(&self,range:&KmoRect){
        // java for(ExpFml fml : this.getAffectedShrFmls(range)) { this.unlinkSharedFormula(fml); }
        for fml in self.getAffectedShrFmls(range){self.unlinkSharedFormula(fml);}
    }
    // java private List<ExpFml> getAffectedShrFmls(KmoRect affectedRange) {
    fn getAffectedShrFmls(&self,affectedRange:&KmoRect)->Vec<Rc<ExpFml>>{
        // java List<ExpFml> allShrFmls = new ArrayList(); this.sharedFmls.lookup((KmoRect)this.sheet.getUsedRange(), allShrFmls);
        let mut allShrFmls=vec![];let sheet=self.sheet.as_ref().and_then(Weak::upgrade).expect("KmoSheet");self.sharedFmls.lookup(&sheet.borrow().getUsedRange(),&mut allShrFmls);
        // java for(int i = allShrFmls.size() - 1; i >= 0; --i) {
        for i in (0..allShrFmls.len()).rev(){
            // java if (!this.isShrFmlsAffectedByRange(affectedRange, (ExpFml)allShrFmls.get(i))) { allShrFmls.remove(i); }
            if !self.isShrFmlsAffectedByRange(affectedRange,&allShrFmls[i]){allShrFmls.remove(i);}
        }
        // java return allShrFmls;
        allShrFmls
    }
    // java private boolean isShrFmlsAffectedByRange(KmoRect range, ExpFml fml) {
    fn isShrFmlsAffectedByRange(&self,range:&KmoRect,fml:&ExpFml)->bool{
        use crate::cn::wps::et::ss::formula::ptg::Ptg::PtgKind;
        // java if (range.intersects(new KmoRect(fml.firstRow, fml.firstCol, fml.lastRow, fml.lastCol))) { return true; } else {
        if range.intersects(&KmoRect::KmoRect__bounds(fml.firstRow,fml.firstCol,fml.lastRow,fml.lastCol)){true}else{
            // java Ptg[] ptgs = fml.getPtgs(); KmoRect refBoxRect = new KmoRect(); KmoRect areaBoxRect = new KmoRect();
            let ptgs=fml.getPtgs();let mut refBoxRect=KmoRect::KmoRect();let mut areaBoxRect=KmoRect::KmoRect();
            // java for(int i = 0; i < ptgs.length; ++i) {
            for i in 0..ptgs.len(){
                // java if (ptgs[i] instanceof RefPtgBase) {
                if matches!(ptgs[i].kind,PtgKind::Ref{..}|PtgKind::Ref3D{..}|PtgKind::Ref3DExt{..}){
                    // java this.getRefBox((RefPtgBase)ptgs[i], fml.firstRow, fml.lastRow, fml.firstCol, fml.lastCol, refBoxRect); if (range.intersects(refBoxRect)) { return true; }
                    self.getRefBox(&ptgs[i],fml.firstRow,fml.lastRow,fml.firstCol,fml.lastCol,&mut refBoxRect);if range.intersects(&refBoxRect){return true;}
                // java } else if (ptgs[i] instanceof AreaPtgBase) {
                }else if matches!(ptgs[i].kind,PtgKind::Area{..}|PtgKind::Area3D{..}|PtgKind::Area3DExt{..}){
                    // java this.getAreaRefBox((AreaPtgBase)ptgs[i], fml.firstRow, fml.lastRow, fml.firstCol, fml.lastCol, areaBoxRect); if (range.intersects(areaBoxRect)) { return true; }
                    self.getAreaRefBox(&ptgs[i],fml.firstRow,fml.lastRow,fml.firstCol,fml.lastCol,&mut areaBoxRect);if range.intersects(&areaBoxRect){return true;}
                }
            }
            // java return false;
            false
        }
    }
    // java private int fixupRelativeRow(int nrow, int currentrow, boolean relative) {
    fn fixupRelativeRow(&self,nrow:i32,currentrow:i32,relative:bool)->i32{
        // java return relative ? (nrow + currentrow) % this.sheet.getBook().getSpreadsheetVersion().getMaxRows() : nrow;
        if relative{nrow.wrapping_add(currentrow)%self.sheet.as_ref().and_then(Weak::upgrade).expect("KmoSheet").borrow().getBook().getSpreadsheetVersion().getMaxRows()}else{nrow}
    }
    // java private int fixupRelativeCol(int ncol, int currentcol, boolean relative) {
    fn fixupRelativeCol(&self,ncol:i32,currentcol:i32,relative:bool)->i32{
        // java return relative ? (ncol + currentcol) % this.sheet.getBook().getSpreadsheetVersion().getMaxColumns() : ncol;
        if relative{ncol.wrapping_add(currentcol)%self.sheet.as_ref().and_then(Weak::upgrade).expect("KmoSheet").borrow().getBook().getSpreadsheetVersion().getMaxColumns()}else{ncol}
    }
    // java private void getAreaRefBox(AreaPtgBase areaPtg, int firstRow, int lastRow, int firstCol, int lastCol, KmoRect refBox) {
    fn getAreaRefBox(&self,areaPtg:&Ptg,firstRow:i32,lastRow:i32,firstCol:i32,lastCol:i32,refBox:&mut KmoRect){
        use crate::cn::wps::et::ss::formula::ptg::Ptg::{PtgKind,row_rel,col_rel,col_index};
        let (fr,lr,fc,lc)=match areaPtg.kind{PtgKind::Area{first_row,last_row,first_col_raw,last_col_raw,..}|PtgKind::Area3D{first_row,last_row,first_col_raw,last_col_raw,..}|PtgKind::Area3DExt{first_row,last_row,first_col_raw,last_col_raw,..}=>(first_row,last_row,first_col_raw,last_col_raw),_=>panic!("ClassCastException AreaPtgBase")};
        // java refBox.tl.row = this.fixupRelativeRow(areaPtg.getFirstRow(), firstRow, areaPtg.isFirstRowRelative());
        refBox.tl.row=self.fixupRelativeRow(fr,firstRow,row_rel(fc));
        // java refBox.tl.col = this.fixupRelativeCol(areaPtg.getFirstColumn(), firstCol, areaPtg.isFirstColRelative());
        refBox.tl.col=self.fixupRelativeCol(col_index(fc)as i32,firstCol,col_rel(fc));
        // java refBox.br.row = this.fixupRelativeRow(areaPtg.getLastRow(), lastRow, areaPtg.isLastRowRelative());
        refBox.br.row=self.fixupRelativeRow(lr,lastRow,row_rel(lc));
        // java refBox.br.col = this.fixupRelativeCol(areaPtg.getLastColumn(), lastCol, areaPtg.isLastColRelative());
        refBox.br.col=self.fixupRelativeCol(col_index(lc)as i32,lastCol,col_rel(lc));
    }
    // java private void getRefBox(RefPtgBase refPtg, int firstRow, int lastRow, int firstCol, int lastCol, KmoRect refBox) {
    fn getRefBox(&self,refPtg:&Ptg,firstRow:i32,lastRow:i32,firstCol:i32,lastCol:i32,refBox:&mut KmoRect){
        use crate::cn::wps::et::ss::formula::ptg::Ptg::{PtgKind,row_rel,col_rel,col_index};
        let (row,col)=match refPtg.kind{PtgKind::Ref{row,col_raw,..}|PtgKind::Ref3D{row,col_raw,..}|PtgKind::Ref3DExt{row,col_raw,..}=>(row,col_raw),_=>panic!("ClassCastException RefPtgBase")};
        // java refBox.tl.row = this.fixupRelativeRow(refPtg.getRow(), firstRow, refPtg.isRowRelative());
        refBox.tl.row=self.fixupRelativeRow(row,firstRow,row_rel(col));
        // java refBox.tl.col = this.fixupRelativeCol(refPtg.getColumn(), firstCol, refPtg.isColRelative());
        refBox.tl.col=self.fixupRelativeCol(col_index(col)as i32,firstCol,col_rel(col));
        // java refBox.br.row = this.fixupRelativeRow(refPtg.getRow(), lastRow, refPtg.isRowRelative());
        refBox.br.row=self.fixupRelativeRow(row,lastRow,row_rel(col));
        // java refBox.br.col = this.fixupRelativeCol(refPtg.getColumn(), lastCol, refPtg.isColRelative());
        refBox.br.col=self.fixupRelativeCol(col_index(col)as i32,lastCol,col_rel(col));
    }
    // java public boolean isIntersectWithArrFml(KmoRect rect) {
    pub fn isIntersectWithArrFml(&self,rect:&KmoRect)->bool{
        // java List<ExpFml> fml = new ArrayList(); this.arrFmls.lookup((KmoRect)rect, fml); return fml.size() != 0;
        let mut fml=vec![];self.arrFmls.lookup(rect,&mut fml);!fml.is_empty()
    }
    // java public boolean isPartOfArrFml(int row, int col) {
    pub fn isPartOfArrFml(&self,row:i32,col:i32)->bool{
        // java List<ExpFml> fml = new ArrayList(); this.arrFmls.lookup((KmoPoint)(new KmoPoint(row, col)), fml); return fml.size() != 0;
        let mut fml=vec![];self.arrFmls.lookup(&KmoRect::KmoRect__bounds(row,col,row,col),&mut fml);!fml.is_empty()
    }
    // java public boolean isContainsAllIntersectArrFml(KmoRect rect) {
    pub fn isContainsAllIntersectArrFml(&self,rect:&KmoRect)->bool{
        // java List<ExpFml> fmls = new ArrayList(); this.arrFmls.lookup((KmoRect)rect, fmls);
        let mut fmls=vec![];self.arrFmls.lookup(rect,&mut fmls);
        // java for(ExpFml fml : fmls) { if (!rect.isContains(fml.firstRow, fml.firstCol, fml.lastRow, fml.lastCol)) { return false; } }
        for fml in fmls{if !rect.isContains__span(fml.firstRow,fml.firstCol,fml.lastRow,fml.lastCol){return false;}}
        // java return true;
        true
    }
}

fn contains(fml: &ExpFml, row: i32, col: i32) -> bool {
    fml.firstRow <= row && row <= fml.lastRow && fml.firstCol <= col && col <= fml.lastCol
}
