use super::super::xf::KmoXF::KmoXF;use crate::cn::wps::moss::filefmt::biff8::record::cf::{PatternFormatting::PatternFormatting,BorderFormatting::BorderFormatting,FontFormatting::FontFormatting};
use super::valuebased::{CfColorScaleValue::CfColorScaleValue,CfDataBarValue::CfDataBarValue,CfIconSetValue::CfIconSetValue};
use super::KmoCfRule::KmoCfRule;
use crate::{
    cn::wps::moss::{
        app::{xf::KmoXFManager::KmoXFManager, KmoSheet::KmoSheet},
        util::{cache::MRULongKeyCache::MRULongKeyCache, AppLogicHelper::AppLogicHelper},
    },
    java_runtime::{Rc, RefCell, Weak},
};
// java public class KmoConditionFormatter
pub struct KmoConditionFormatter {
    sheet: Weak<RefCell<KmoSheet>>,
    results: MRULongKeyCache<Rc<CfCacheItem>>,
    cfRuleTree:crate::cn::wps::moss::util::KmoUndoableRTree::KmoUndoableRTree<KmoCfRule>,counter:super::CfRuleCounter::CfRuleCounter,conditionCount:crate::java_runtime::Cell<i32>,
}
// java public static class CfRuleResult
#[derive(Default)]
pub struct CfRuleResult {
 pub dataBarValue:Option<Rc<CfDataBarValue>>,pub colorScaleValue:Option<Rc<CfColorScaleValue>>,pub iconSetValue:Option<Rc<CfIconSetValue>>,
    pub generals: Option<Vec<Rc<KmoCfRule>>>,
}
// java public static class CfCacheItem {public CfRuleResult rules;public KmoTipStamp tip;}
pub struct CfCacheItem {
    pub rules: RefCell<CfRuleResult>,
    pub tip:RefCell<Option<Rc<dyn kfc::cn::wps::memor4j::KmoTipStamp::KmoTipStamp + Send + Sync>>>,
}
impl KmoConditionFormatter {
    // java this.sheet=sheet;private MRULongKeyCache<CfCacheItem> results=new MRULongKeyCache(5000);
    pub fn new(sheet: Weak<RefCell<KmoSheet>>) -> Self {
        Self {
            sheet,
            results: MRULongKeyCache::new(5000),cfRuleTree:crate::cn::wps::moss::util::KmoUndoableRTree::KmoUndoableRTree::new(),counter:super::CfRuleCounter::CfRuleCounter::new(),conditionCount:crate::java_runtime::Cell::new(-1),
        }
    }
    // java public MRULongKeyCache<CfCacheItem> getResultCache(){return this.results;}
    pub fn getResultCache(&self) -> &MRULongKeyCache<Rc<CfCacheItem>> {
        &self.results
    }
    // java public String getNumFmtCode(KmoXFManager xfManager,int row,int col)
    pub fn getNumFmtCode(&self, xfManager: &KmoXFManager, row: i32, col: i32) -> Option<String> {
        // java CfCacheItem item=this.results.get(AppLogicHelper.rc2long(row,col));if(item!=null&&item.rules.generals!=null){...}else return null;
        let item = self.results.get(AppLogicHelper::rc2long(row, col))?;
        let result=item.rules.borrow();let rules=result.generals.as_ref()?;
        // java for(...) {numfmt=rule.getRule().getNumFmt();if(numfmt!=null){formatCode=numfmt.fmtCode();if(formatCode!=null)return formatCode;formatCode=xfManager.getFormat(numfmt.numFmtId()).getFormatString();if(formatCode!=null)return formatCode;}}return formatCode;
        for rule in rules {
            if let Some(numfmt) = rule.getRule().getNumFmt() {
                if let Some(code) = numfmt.fmtCode() {
                    return Some(code);
                }
                if let Some(code) = xfManager
                    .getFormat(numfmt.numFmtId())
                    .expect("null conditional number format")
                    .getFormatString()
                {
                    return Some(code);
                }
            }
        }
        None
    }
// java public int getConditionCount() {
// java return this.conditionCount;
// java }
pub fn getConditionCount(&self)->i32 {
self.conditionCount.get()
}
// java public void addConditionCount() {
// java ++this.conditionCount;
// java }
pub fn addConditionCount(&self) {
self.conditionCount.set(self.conditionCount.get().wrapping_add(1));
}
// java public boolean isEmpty() {
// java return this.counter.count() <= 0;
// java }
pub fn isEmpty(&self)->bool {
self.counter.count()<=0
}
// java public void addCfRule(KmoCfRule cfRule) {
// java if (cfRule == null) {
// java throw new IllegalArgumentException();
// java } else {
// java KmoRect[] ranges = cfRule.ranges();
// java if (ranges != null && ranges.length != 0) {
// java this.counter.plus();
// java int i = 0;
// java
// java for(int size = ranges.length; i < size; ++i) {
// java this.cfRuleTree.insert(ranges[i], cfRule);
// java }
// java
// java } else {
// java throw new IllegalArgumentException("Rule ranges为空");
// java }
// java }
// java }
pub fn addCfRule(&self,cfRule:Rc<KmoCfRule>) {
let ranges=cfRule.ranges().expect("IllegalArgumentException");assert!(!ranges.is_empty(),"Rule ranges为空");self.counter.plus();for rect in ranges.iter(){self.cfRuleTree.insert(rect,cfRule.clone());}
}
// java public void delCfRule(KmoCfRule cfRule) {
// java if (cfRule == null) {
// java throw new IllegalArgumentException();
// java } else {
// java KmoRect[] ranges = cfRule.ranges();
// java if (ranges != null && ranges.length != 0) {
// java this.counter.minus();
// java int i = 0;
// java
// java for(int size = ranges.length; i < size; ++i) {
// java this.cfRuleTree.remove(ranges[i], cfRule);
// java }
// java
// java } else {
// java throw new IllegalArgumentException("Rule ranges为空");
// java }
// java }
// java }
pub fn delCfRule(&self,cfRule:Rc<KmoCfRule>) {
let ranges=cfRule.ranges().expect("IllegalArgumentException");assert!(!ranges.is_empty(),"Rule ranges为空");self.counter.minus();for rect in ranges.iter(){self.cfRuleTree.remove(rect,cfRule.clone());}
}
// java public void getRangeCFs(KmoRect range, Collection<KmoCfRule> out) {
// java Set<KmoCfRule> cfs = new HashSet();
// java this.cfRuleTree.lookup((KmoRect)range, cfs);
// java out.addAll(cfs);
// java }
pub fn getRangeCFs(&self,range:&crate::cn::wps::moss::util::KmoRect::KmoRect,out:&mut Vec<Rc<KmoCfRule>>) {
let mut cfs=vec![];self.cfRuleTree.lookup(range,&mut cfs);// rust: Native collection adapter preserves Java HashSet value hash/equality and unordered iteration.
let mut set=std::collections::HashSet::new();for r in cfs{set.insert(CfSetKey(r));}out.extend(set.into_iter().map(|k|k.0));
}
// java public void getCellRules(int row, int col, Collection<KmoCfRule> out) {
// java KmoRect r = SharedObjects.kmoRectPool.borrowObject();
// java r.tl.row = row;
// java r.br.row = row;
// java r.tl.col = col;
// java r.br.col = col;
// java Set<KmoCfRule> cfs = new HashSet();
// java this.cfRuleTree.lookup((KmoRect)r, cfs);
// java r = SharedObjects.kmoRectPool.returnObject(r);
// java out.addAll(cfs);
// java }
pub fn getCellRules(&self,row:i32,col:i32,out:&mut Vec<Rc<KmoCfRule>>) {
self.getRangeCFs(&crate::cn::wps::moss::util::KmoRect::KmoRect::KmoRect__bounds(row,col,row,col),out);
}
// java public List<KmoCfRule> getAllCfRule() {
// java Set<KmoCfRule> cfs = new HashSet();
// java KmoRect rect = new KmoRect(0, 0, this.sheet.getMaxRows() - 1, this.sheet.getMaxColumns() - 1);
// java this.cfRuleTree.lookup((KmoRect)rect, cfs);
// java List<KmoCfRule> defines = new ArrayList();
// java defines.addAll(cfs);
// java return defines;
// java }
pub fn getAllCfRule(&self)->Vec<Rc<KmoCfRule>> {
let sheet=self.sheet.upgrade().unwrap();let sheet=sheet.borrow();let mut out=vec![];self.getRangeCFs(&crate::cn::wps::moss::util::KmoRect::KmoRect::KmoRect__bounds(0,0,sheet.getMaxRows()-1,sheet.getMaxColumns()-1),&mut out);out
}
// java public static void turnDefineToCfRuleFromIO(KmoConditionFormatterDefine define, KmoConditionFormatter formatter) {
// java List<KmoCfRule> cfRules = define.ruleList;
// java List<KmoRect> ranges = define.getRefRangeList();
// java int i = 0;
// java
// java for(int size = cfRules.size(); i < size; ++i) {
// java KmoCfRule rule = (KmoCfRule)cfRules.get(i);
// java rule.setRanges(ranges);
// java if (rule.isExt()) {
// java rule.setFormatterGroupIndex(formatter.getConditionCount());
// java }
// java
// java formatter.addCfRule(rule);
// java }
// java
// java }
pub fn turnDefineToCfRuleFromIO(define:&super::KmoConditionFormatterDefine::KmoConditionFormatterDefine,formatter:&Self) {
let ranges=define.getRefRangeList();for rule in define.ruleList.borrow().iter(){rule.setRanges(ranges.as_ref().clone());if rule.isExt(){rule.setFormatterGroupIndex(formatter.getConditionCount());}formatter.addCfRule(rule.clone());}
}
}
impl KmoConditionFormatter{// java public CfDataBarValue getDataBar(int row, int col) {
// java CfCacheItem item = this.results.get(AppLogicHelper.rc2long(row, col));
// java return item == null ? null : item.rules.dataBarValue;
// java }
pub fn getDataBar(&self,row:i32,col:i32)->Option<Rc<CfDataBarValue>> {
self.results.get(AppLogicHelper::rc2long(row,col)).and_then(|i|i.rules.borrow().dataBarValue.clone())
}
// java public CfColorScaleValue getColorScale(int row, int col) {
// java CfCacheItem item = this.results.get(AppLogicHelper.rc2long(row, col));
// java return item == null ? null : item.rules.colorScaleValue;
// java }
pub fn getColorScale(&self,row:i32,col:i32)->Option<Rc<CfColorScaleValue>> {
self.results.get(AppLogicHelper::rc2long(row,col)).and_then(|i|i.rules.borrow().colorScaleValue.clone())
}
// java public CfIconSetValue getIconSet(int row, int col) {
// java CfCacheItem item = this.results.get(AppLogicHelper.rc2long(row, col));
// java return item == null ? null : item.rules.iconSetValue;
// java }
pub fn getIconSet(&self,row:i32,col:i32)->Option<Rc<CfIconSetValue>> {
self.results.get(AppLogicHelper::rc2long(row,col)).and_then(|i|i.rules.borrow().iconSetValue.clone())
}
// java public List<KmoCfRule> getGenerals(int row, int col) {
// java CfCacheItem item = this.results.get(AppLogicHelper.rc2long(row, col));
// java return item == null ? null : item.rules.generals;
// java }
pub fn getGenerals(&self,row:i32,col:i32)->Option<Vec<Rc<KmoCfRule>>> {
self.results.get(AppLogicHelper::rc2long(row,col)).and_then(|i|i.rules.borrow().generals.clone())
}
// java public boolean isShowValue(int row, int col) {
// java CfCacheItem item = this.results.get(AppLogicHelper.rc2long(row, col));
// java if (item == null) {
// java return true;
// java } else {
// java boolean dbShowValue = true;
// java boolean iconShowValue = true;
// java if (item.rules.dataBarValue != null) {
// java dbShowValue = item.rules.dataBarValue.isShowValue();
// java }
// java
// java if (item.rules.iconSetValue != null) {
// java iconShowValue = item.rules.iconSetValue.isShowValue();
// java }
// java
// java return dbShowValue & iconShowValue;
// java }
// java }
pub fn isShowValue(&self,row:i32,col:i32)->bool {
let Some(item)=self.results.get(AppLogicHelper::rc2long(row,col))else{return true;};let r=item.rules.borrow();let d=r.dataBarValue.as_ref().is_none_or(|v|v.isShowValue());let i=r.iconSetValue.as_ref().is_none_or(|v|v.isShowValue());d&i
}
}
impl CfRuleResult{// java public boolean isEmpty() {
// java return this.generals == null && this.dataBarValue == null && this.colorScaleValue == null && this.iconSetValue == null;
// java }
pub fn isEmpty(&self)->bool {
self.generals.is_none()&&self.dataBarValue.is_none()&&self.colorScaleValue.is_none()&&self.iconSetValue.is_none()
}
// java public boolean contains07Result() {
// java return this.dataBarValue != null || this.colorScaleValue != null || this.iconSetValue != null;
// java }
pub fn contains07Result(&self)->bool {
self.colorScaleValue.is_some()||self.dataBarValue.is_some()||self.iconSetValue.is_some()
}
// java CfRuleResult.equals compares only the non-null members of the argument.
pub fn equals(&self,other:&Self)->bool{let g=other.generals.as_ref().is_none_or(|a|self.generals.as_ref().is_some_and(|b|a.len()==b.len()&&a.iter().zip(b).all(|(a,b)|a.equals(b))));let d=other.dataBarValue.as_ref().is_none_or(|a|self.dataBarValue.as_ref().is_some_and(|b|a.equals(b)));let c=other.colorScaleValue.as_ref().is_none_or(|a|self.colorScaleValue.as_ref().is_some_and(|b|a.equals(b)));let i=other.iconSetValue.as_ref().is_none_or(|a|self.iconSetValue.as_ref().is_some_and(|b|a.equals(b)));g&d&c&i}}
impl KmoConditionFormatter{pub const Pattern:i8=0;pub const Border:i8=1;pub const Font:i8=2;
// java public static KmoXF mergePatternFormat(PatternFormatting patternFormat, KmoXF xf, int priorityColor) {
// java KmoXF newXF = KmoXF.shareEmptyCellXF();
// java newXF = KmoXF.createXF(newXF);
// java newXF.completeCopy(xf);
// java int foreColor = patternFormat.getFillForegroundColor();
// java if (foreColor == 0) {
// java foreColor = 64;
// java }
// java
// java int backColor = patternFormat.getFillBackgroundColor();
// java if (backColor == 0) {
// java backColor = 64;
// java }
// java
// java if (priorityColor != 64) {
// java backColor = priorityColor;
// java }
// java
// java int fillStyle = patternFormat.getFillPattern();
// java if (fillStyle == 1) {
// java newXF.setFillForeground(backColor);
// java newXF.setAdtlFillPattern((short)1);
// java } else if (fillStyle == 32) {
// java newXF.setXFGradient(patternFormat.getXFGradient());
// java newXF.setAdtlFillPattern((short)32);
// java } else if (fillStyle == 0) {
// java if (backColor != 64 && backColor != 65) {
// java newXF.setFillForeground(backColor);
// java newXF.setAdtlFillPattern((short)1);
// java } else {
// java newXF.setAdtlFillPattern((short)0);
// java }
// java } else {
// java newXF.setFillForeground(foreColor);
// java newXF.setFillBackground(backColor);
// java newXF.setAdtlFillPattern((short)fillStyle);
// java }
// java
// java return newXF;
// java }
pub fn mergePatternFormat(p:&PatternFormatting,xf:&KmoXF,priorityColor:i32)->Rc<KmoXF> {
let new=Rc::new(KmoXF::createXF(Some(&KmoXF::shareEmptyCellXF())));new.completeCopy(xf);let mut fore=p.getFillForegroundColor();if fore==0{fore=64;}let mut back=p.getFillBackgroundColor();if back==0{back=64;}if priorityColor!=64{back=priorityColor;}let style=p.getFillPattern();if style==1{new.setFillForeground(back);new.setAdtlFillPattern(1);}else if style==32{new.setXFGradient(p.getXFGradient());new.setAdtlFillPattern(32);}else if style==0{if back!=64&&back!=65{new.setFillForeground(back);new.setAdtlFillPattern(1);}else{new.setAdtlFillPattern(0);}}else{new.setFillForeground(fore);new.setFillBackground(back);new.setAdtlFillPattern(style as i16);}new
}
// java public static KmoXF mergeBorderFormat(BorderFormatting borderFormat, KmoXF xf) {
// java KmoXF newXF = KmoXF.shareEmptyCellXF();
// java newXF = KmoXF.createXF(newXF);
// java newXF.completeCopy(xf);
// java int borderLineStyle = borderFormat.getBorderLeft();
// java newXF.setBorderLeft((short)borderLineStyle);
// java borderLineStyle = borderFormat.getBorderTop();
// java newXF.setBorderTop((short)borderLineStyle);
// java borderLineStyle = borderFormat.getBorderRight();
// java newXF.setBorderRight((short)borderLineStyle);
// java borderLineStyle = borderFormat.getBorderBottom();
// java newXF.setBorderBottom((short)borderLineStyle);
// java borderLineStyle = borderFormat.getLeftBorderColor();
// java newXF.setLeftBorderPaletteIdx(borderLineStyle);
// java borderLineStyle = borderFormat.getRightBorderColor();
// java newXF.setRightBorderPaletteIdx(borderLineStyle);
// java borderLineStyle = borderFormat.getTopBorderColor();
// java newXF.setTopBorderPaletteIdx(borderLineStyle);
// java borderLineStyle = borderFormat.getBottomBorderColor();
// java newXF.setBottomBorderPaletteIdx(borderLineStyle);
// java return newXF;
// java }
pub fn mergeBorderFormat(p:&BorderFormatting,xf:&KmoXF)->Rc<KmoXF> {
let new=Rc::new(KmoXF::createXF(Some(&KmoXF::shareEmptyCellXF())));new.completeCopy(xf);new.setBorderLeft(p.getBorderLeft() as i16);new.setBorderTop(p.getBorderTop() as i16);new.setBorderRight(p.getBorderRight() as i16);new.setBorderBottom(p.getBorderBottom() as i16);new.setLeftBorderPaletteIdx(p.getLeftBorderColor());new.setRightBorderPaletteIdx(p.getRightBorderColor());new.setTopBorderPaletteIdx(p.getTopBorderColor());new.setBottomBorderPaletteIdx(p.getBottomBorderColor());new
}
// java public static KmoXF mergeFontFormat(FontFormatting fontFormat, KmoXF xf) {
// java KmoFont font = xf.getFont();
// java KmoXF newXf = KmoXF.shareEmptyCellXF();
// java newXf = KmoXF.createXF(newXf);
// java newXf.completeCopy(xf);
// java KmoFont newFont = newXf.getFont();
// java short fontHeight = (short)fontFormat.getFontHeight();
// java if (fontHeight != -1 && fontHeight != 0 && fontHeight != font.getFontHeight()) {
// java newFont.setFontHeight((short)fontFormat.getFontHeight());
// java }
// java
// java if (fontFormat.isFontStyleModified()) {
// java if (fontFormat.getFontWeight() != font.getBoldWeight()) {
// java newFont.setBoldWeight(fontFormat.getFontWeight());
// java }
// java
// java if (fontFormat.isItalic() != font.isItalic()) {
// java newFont.setItalic(fontFormat.isItalic());
// java }
// java }
// java
// java int icv = fontFormat.getFontColorIndex();
// java if (icv != 32767 && icv != font.getColorPaletteIndex()) {
// java newFont.setColorPaletteIndex(icv);
// java }
// java
// java if (fontFormat.isUnderlineTypeModified()) {
// java newFont.setUnderline((byte)fontFormat.getUnderlineType());
// java }
// java
// java if (fontFormat.isFontCancellationModified()) {
// java newFont.setStrikeout(fontFormat.isStruckout());
// java }
// java
// java if (fontFormat.isEscapementTypeModified()) {
// java newFont.setSuperSubScript(fontFormat.getEscapementType());
// java }
// java
// java return newXf;
// java }
pub fn mergeFontFormat(p:&FontFormatting,xf:&KmoXF)->Rc<KmoXF> {
let font=xf.getFont().expect("null font");let new=Rc::new(KmoXF::createXF(Some(&KmoXF::shareEmptyCellXF())));new.completeCopy(xf);let f=new.getFont().expect("null copied font");let h=p.getFontHeight() as i16;if h!= -1&&h!=0&&h!=font.getFontHeight(){f.setFontHeight(h);}if p.isFontStyleModified(){if p.getFontWeight()!=font.getBoldWeight(){f.setBoldWeight(p.getFontWeight());}if p.isItalic()!=font.isItalic(){f.setItalic(p.isItalic());}}let color=p.getFontColorIndex();if color!=32767&&color!=font.getColorPaletteIndex(){f.setColorPaletteIndex(color);}if p.isUnderlineTypeModified(){f.setUnderline(p.getUnderlineType()as i8);}if p.isFontCancellationModified(){f.setStrikeout(p.isStruckout());}if p.isEscapementTypeModified(){f.setSuperSubScript(p.getEscapementType());}new
}
// java public KmoXF getMergeXF(int row, int col, KmoXF xf, byte type) {
// java CfCacheItem item = this.results.get(AppLogicHelper.rc2long(row, col));
// java if (item != null && item.rules.generals != null && item.rules.generals.size() != 0) {
// java int i = 0;
// java
// java for(int size = item.rules.generals.size(); i < size; ++i) {
// java KmoCfRule rule = (KmoCfRule)item.rules.generals.get(i);
// java switch (type) {
// java case 0:
// java PatternFormatting pattern = rule.getRule().getPattern();
// java CfColorScaleValue colorScale = item.rules.colorScaleValue;
// java int priorityColor = 64;
// java if (colorScale != null && colorScale.getPriority() < rule.getRule().getPriority()) {
// java priorityColor = colorScale.getColor();
// java }
// java
// java if (pattern != null) {
// java return mergePatternFormat(pattern, xf, priorityColor);
// java }
// java break;
// java case 1:
// java BorderFormatting border = rule.getRule().getBorder();
// java if (border != null) {
// java return mergeBorderFormat(border, xf);
// java }
// java break;
// java case 2:
// java FontFormatting font = rule.getRule().getFont();
// java if (font != null) {
// java return mergeFontFormat(font, xf);
// java }
// java }
// java }
// java
// java return xf;
// java } else {
// java return xf;
// java }
// java }
pub fn getMergeXF(&self,row:i32,col:i32,xf:Rc<KmoXF>,kind:i8)->Rc<KmoXF> {
if let Some(item)=self.results.get(AppLogicHelper::rc2long(row,col)){let result=item.rules.borrow();if let Some(generals)=&result.generals{for rule in generals{let rule=rule.getRule();match kind{0=>{let priorityColor=result.colorScaleValue.as_ref().filter(|v|v.getPriority()<rule.getPriority()).map_or(64,|v|v.getColor());if let Some(p)=rule.getPattern(){return Self::mergePatternFormat(&p.borrow(),&xf,priorityColor);}},1=>if let Some(b)=rule.getBorder(){return Self::mergeBorderFormat(&b.borrow(),&xf);},2=>if let Some(f)=rule.getFont(){return Self::mergeFontFormat(&f,&xf);},_=>{}}}}}xf
}
}

struct CfSetKey(Rc<KmoCfRule>);impl PartialEq for CfSetKey{fn eq(&self,o:&Self)->bool{self.0.equals(&o.0)}}impl Eq for CfSetKey{}impl std::hash::Hash for CfSetKey{fn hash<H:std::hash::Hasher>(&self,h:&mut H){h.write_i32(self.0.hashCode());}}

impl KmoConditionFormatter{
// java public boolean hasRuleIn(KmoRect range) {
// java List<KmoCfRule> out = new ArrayList();
// java this.cfRuleTree.lookup((KmoRect)range, out);
// java return !out.isEmpty();
// java }
pub fn hasRuleIn(&self,r:&crate::cn::wps::moss::util::KmoRect::KmoRect)->bool{let mut out=vec![];self.cfRuleTree.lookup(r,&mut out);!out.is_empty()}
}
