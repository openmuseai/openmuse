use crate::java_runtime::{Rc,RefCell,Cell};
use crate::cn::wps::moss::app::KmoSheet::KmoSheet;
use crate::cn::wps::moss::app::condfmt::{KmoCfRule::KmoCfRule,KmoConditionFormatterDefine::KmoConditionFormatterDefine,rule::{Rule::{RuleObject,CfRuleTypes},KmoColorScale::KmoColorScale,KmoDataBar::KmoDataBar,KmoIconSet::{KmoIconSet,IconSetTypes}}};
use crate::cn::wps::moss::xlsxr::model::StylesTable::StylesTable;
use kfc_kfc_alpha::kfc::xpxml::reader::{ElementHandler::ElementHandler,XmlRoAttr::XmlRoAttr};
use crate::cn::wps::moss::{app::condfmt::rule::{KmoRuleCellIs::{KmoRuleCellIs,CellIsOperatorType},KmoRuleAverage::KmoRuleAverage,KmoRuleTop::KmoRuleTop,KmoRuleSpecialText::{KmoRuleSpecialText,SpecialTextOperatorType},KmoRuleTimePeriod::{KmoRuleTimePeriod,TimePeriodType}},filefmt::biff8::record::cf::{FontFormatting::FontFormatting,BorderFormatting::BorderFormatting,NumberFormatFormatting::NumberFormatFormatting},xlsxr::handler::style::{DxfHandler::{DxfHandler,DxfHandlerRef},StylesShared}};
#[derive(Clone)]
pub struct CfRuleHandler{stylesTable:Rc<RefCell<StylesTable>>,condFmt10:Rc<KmoConditionFormatterDefine>,sheet:Rc<RefCell<KmoSheet>>,kmoCfRule:Rc<RefCell<Rc<KmoCfRule>>>,formulas:Rc<RefCell<Vec<String>>>}
impl CfRuleHandler{
// java public CfRuleHandler(StylesTable stylesTable, KmoConditionFormatterDefine condFmt10, KmoSheet sheet) {
// java this(stylesTable, condFmt10, sheet, false);
// java }
pub fn new(stylesTable:Rc<RefCell<StylesTable>>,condFmt10:Rc<KmoConditionFormatterDefine>,sheet:Rc<RefCell<KmoSheet>>,isFromExt:bool)->Self {
let rule=Rc::new(KmoCfRule::new());rule.setExt(isFromExt);Self{stylesTable,condFmt10,sheet,kmoCfRule:Rc::new(RefCell::new(rule)),formulas:Rc::new(RefCell::new(vec![]))}
}
// java public void init(boolean isExt) {
// java this.formulas.clear();
// java this.kmoCfRule = new KmoCfRule();
// java this.kmoCfRule.setExt(isExt);
// java }
pub fn init(&self,isFromExt:bool) {
self.formulas.borrow_mut().clear();let rule=Rc::new(KmoCfRule::new());rule.setExt(isFromExt);*self.kmoCfRule.borrow_mut()=rule;
}
// java public ArrayList<String> getFormulas() {
// java return this.formulas;
// java }
pub fn getFormulas(&self)->Rc<RefCell<Vec<String>>> {
self.formulas.clone()
}
// java public static void dealBorderFormatting(BorderFormatting bo) {
// java if (bo.getBorderBottom() == 0 && bo.getBottomBorderColor() == 0) {
// java bo.setBottomBorderColor(64);
// java }
// java
// java if (bo.getBorderLeft() == 0 && bo.getLeftBorderColor() == 0) {
// java bo.setLeftBorderColor(64);
// java }
// java
// java if (bo.getBorderRight() == 0 && bo.getRightBorderColor() == 0) {
// java bo.setRightBorderColor(64);
// java }
// java
// java if (bo.getBorderTop() == 0 && bo.getTopBorderColor() == 0) {
// java bo.setTopBorderColor(64);
// java }
// java
// java }
pub fn dealBorderFormatting(bo:&mut BorderFormatting) {
if bo.getBorderBottom()==0&&bo.getBottomBorderColor()==0{bo.setBottomBorderColor(64);}if bo.getBorderLeft()==0&&bo.getLeftBorderColor()==0{bo.setLeftBorderColor(64);}if bo.getBorderRight()==0&&bo.getRightBorderColor()==0{bo.setRightBorderColor(64);}if bo.getBorderTop()==0&&bo.getTopBorderColor()==0{bo.setTopBorderColor(64);}
}
// java public static void iniFontFormat(StylesTable stylesTable, FontFormatting font, int fontId) {
// java FontRecord hfont = stylesTable.getFontRecordAt(fontId);
// java font.setFontStyleModified(true);
// java if (hfont.isItalic()) {
// java font.setItalic(true);
// java } else {
// java font.setItalic(false);
// java }
// java
// java if (hfont.getSuperSubScript() != 0) {
// java font.setEscapementType(hfont.getSuperSubScript());
// java }
// java
// java if (hfont.getBoldWeight() == 700) {
// java font.setBold(true);
// java font.setFontWieghtModified(true);
// java } else {
// java font.setBold(false);
// java }
// java
// java font.setFontColorIndex(hfont.getColorPaletteIndex());
// java if (hfont.isStruckout()) {
// java font.setStrikeout(true);
// java font.setEscapementTypeModified(true);
// java font.setFontCancellationModified(true);
// java }
// java
// java if (hfont.getUnderline() != 0) {
// java font.setUnderlineTypeModified(true);
// java font.setUnderlineType((short)hfont.getUnderline());
// java }
// java
// java font.setFontHeight(hfont.getFontHeight());
// java if (hfont.hasBold()) {
// java font.setBoldOption();
// java }
// java
// java if (hfont.hasItalic()) {
// java font.setItalicOption();
// java }
// java
// java if (hfont.hasUnderline()) {
// java font.setUnderlineOption();
// java }
// java
// java if (hfont.hasColor()) {
// java font.setColorOption();
// java }
// java
// java if (hfont.hasStrike()) {
// java font.setStrikeOption();
// java }
// java
// java }
pub fn iniFontFormat(stylesTable:&StylesTable,font:&FontFormatting,fontId:i32) {
let hfont=stylesTable.getFontRecordAt(fontId);let hfont=hfont.borrow();font.setFontStyleModified(true);font.setItalic(hfont.isItalic());if hfont.getSuperSubScript()!=0{font.setEscapementType(hfont.getSuperSubScript());}if hfont.getBoldWeight()==700{font.setBold(true);font.setFontWieghtModified(true);}else{font.setBold(false);}font.setFontColorIndex(hfont.getColorPaletteIndex());if hfont.isStruckout(){font.setStrikeout(true);font.setEscapementTypeModified(true);font.setFontCancellationModified(true);}if hfont.getUnderline()!=0{font.setUnderlineTypeModified(true);font.setUnderlineType(hfont.getUnderline() as i16);}font.setFontHeight(hfont.getFontHeight() as i32);if hfont.hasBold(){font.setBoldOption();}if hfont.hasItalic(){font.setItalicOption();}if hfont.hasUnderline(){font.setUnderlineOption();}if hfont.hasColor(){font.setColorOption();}if hfont.hasStrike(){font.setStrikeOption();}
}
}
impl ElementHandler for CfRuleHandler{
// java public void startElement(int element, XmlRoAttr attrsNotCopy) {
// java Rule rule = null;
// java XmlRoAttr tmp;
// java if ((tmp = attrsNotCopy.getByName(4125)) != null) {
// java String str = tmp.toString();
// java Rule.CfRuleTypes ctype = Rule.CfRuleTypes.error;
// java
// java try {
// java ctype = Rule.CfRuleTypes.valueOf(str);
// java } catch (IllegalArgumentException var12) {
// java }
// java
// java rule = KmoCfRule.buildRule(ctype, this.sheet.getVersion());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4117)) != null) {
// java String str = tmp.toString();
// java Rule.CfRuleTypes type = rule.type();
// java if (type.equals(Rule.CfRuleTypes.cellIs)) {
// java KmoRuleCellIs cellIs = (KmoRuleCellIs)rule;
// java
// java try {
// java cellIs.setCellIsOpType(KmoRuleCellIs.CellIsOperatorType.valueOf(str));
// java } catch (IllegalArgumentException var11) {
// java cellIs.setCellIsOpType(KmoRuleCellIs.CellIsOperatorType.equal);
// java }
// java } else if (rule instanceof KmoRuleSpecialText) {
// java KmoRuleSpecialText text = (KmoRuleSpecialText)rule;
// java
// java try {
// java text.setOpType(KmoRuleSpecialText.SpecialTextOperatorType.valueOf(str));
// java } catch (IllegalArgumentException var10) {
// java text.setOpType(KmoRuleSpecialText.SpecialTextOperatorType.containsText);
// java }
// java }
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4121)) != null) {
// java rule.setDxfId(tmp.toInt());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4949)) != null && rule instanceof KmoRuleAverage) {
// java KmoRuleAverage average = (KmoRuleAverage)rule;
// java average.setAboveAverage(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4950)) != null && rule instanceof KmoRuleTop) {
// java KmoRuleTop top = (KmoRuleTop)rule;
// java top.setBottom(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4954)) != null && rule instanceof KmoRuleAverage) {
// java KmoRuleAverage average = (KmoRuleAverage)rule;
// java average.setEqualAverage(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4119)) != null && rule instanceof KmoRuleTop) {
// java KmoRuleTop top = (KmoRuleTop)rule;
// java top.setPercent(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4576)) != null) {
// java rule.setPriority(tmp.toInt());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4952)) != null && rule instanceof KmoRuleTop) {
// java KmoRuleTop top = (KmoRuleTop)rule;
// java top.setRank(tmp.toInt());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4953)) != null && rule instanceof KmoRuleAverage) {
// java KmoRuleAverage average = (KmoRuleAverage)rule;
// java average.setStdDev(tmp.toInt());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4948)) != null) {
// java rule.setStopIfTrue(tmp.toBoolean());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4162)) != null && rule instanceof KmoRuleSpecialText) {
// java KmoRuleSpecialText text = (KmoRuleSpecialText)rule;
// java text.setText(tmp.toString());
// java }
// java
// java if ((tmp = attrsNotCopy.getByName(4951)) != null && rule instanceof KmoRuleTimePeriod) {
// java KmoRuleTimePeriod time = (KmoRuleTimePeriod)rule;
// java
// java try {
// java time.setTimePeriodType(KmoRuleTimePeriod.TimePeriodType.valueOf(tmp.toString()));
// java } catch (Exception var9) {
// java time.setTimePeriodType(KmoRuleTimePeriod.TimePeriodType.today);
// java }
// java }
// java
// java this.kmoCfRule.setRule(rule);
// java }
fn startElement(&self,_e:i32,a:&dyn XmlRoAttr){let ctype=a.get_by_name(4125).and_then(|a|CfRuleTypes::valueOf(&a.to_string())).unwrap_or(CfRuleTypes::error);let rule=KmoCfRule::buildRule(ctype,self.sheet.borrow().getVersion());if let Some(a)=a.get_by_name(4117){if let Some(r)=rule.as_any().downcast_ref::<KmoRuleCellIs>(){r.setCellIsOpType(Some(CellIsOperatorType::valueOf(&a.to_string()).unwrap_or(CellIsOperatorType::equal)));}if let Some(r)=rule.as_any().downcast_ref::<KmoRuleSpecialText>(){r.setOpType(Some(SpecialTextOperatorType::valueOf(&a.to_string()).unwrap_or(SpecialTextOperatorType::containsText)));}}if let Some(a)=a.get_by_name(4121){rule.setDxfId(a.to_int());}if let Some(a)=a.get_by_name(4949){if let Some(r)=rule.as_any().downcast_ref::<KmoRuleAverage>(){r.setAboveAverage(a.to_boolean());}}if let Some(a)=a.get_by_name(4950){if let Some(r)=rule.as_any().downcast_ref::<KmoRuleTop>(){r.setBottom(a.to_boolean());}}if let Some(a)=a.get_by_name(4954){if let Some(r)=rule.as_any().downcast_ref::<KmoRuleAverage>(){r.setEqualAverage(a.to_boolean());}}if let Some(a)=a.get_by_name(4119){if let Some(r)=rule.as_any().downcast_ref::<KmoRuleTop>(){r.setPercent(a.to_boolean());}}if let Some(a)=a.get_by_name(4576){rule.setPriority(a.to_int());}if let Some(a)=a.get_by_name(4952){if let Some(r)=rule.as_any().downcast_ref::<KmoRuleTop>(){r.setRank(a.to_int());}}if let Some(a)=a.get_by_name(4953){if let Some(r)=rule.as_any().downcast_ref::<KmoRuleAverage>(){r.setStdDev(a.to_int());}}if let Some(a)=a.get_by_name(4948){rule.setStopIfTrue(a.to_boolean());}if let Some(a)=a.get_by_name(4162){if let Some(r)=rule.as_any().downcast_ref::<KmoRuleSpecialText>(){r.setText(a.to_string());}}if let Some(a)=a.get_by_name(4951){if let Some(r)=rule.as_any().downcast_ref::<KmoRuleTimePeriod>(){r.setTimePeriodType(Some(TimePeriodType::valueOf(&a.to_string()).unwrap_or(TimePeriodType::today)));}}self.kmoCfRule.borrow().setRule(rule);}
// java public ElementHandler enterSubElement(int subElement) {
// java switch (subElement) {
// java case 4123:
// java return new IconSetHandler(this.kmoCfRule, this.sheet);
// java case 4320:
// java FormulaHandler formulaHandler = new FormulaHandler();
// java return formulaHandler;
// java case 4359:
// java return new FExtHandler();
// java case 4729:
// java return new DxfExHandler(this.sheet.getBook(), this.stylesTable);
// java case 4946:
// java return new ColorScaleHandler(this.kmoCfRule, this.sheet);
// java case 4947:
// java return new DataBarHandler(this.kmoCfRule, this.sheet);
// java default:
// java return null;
// java }
// java }
fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{let rule=self.kmoCfRule.borrow().clone();match e{4123=>Some(Box::new(super::condfmt::IconSetHandler::IconSetHandler::new(rule,self.sheet.clone()))),4946=>Some(Box::new(super::condfmt::ColorScaleHandler::ColorScaleHandler::new(rule,self.sheet.clone()))),4947=>Some(Box::new(super::condfmt::DataBarHandler::DataBarHandler::new(rule,self.sheet.clone()))),4320=>Some(Box::new(FormulaHandler(self.clone()))),4359=>Some(Box::new(FExtHandler(self.formulas.clone()))),4729=>{let shared=StylesShared::new(self.sheet.borrow().getBook(),self.stylesTable.clone());Some(Box::new(DxfExHandler{base:DxfHandlerRef(std::rc::Rc::new(DxfHandler::new(shared))),owner:self.clone()}))},_=>None}}
// java public void endElement(int element) {
// java this.kmoCfRule.checkNeedCreateFormula(this.condFmt10.getRefRangeList(), this.sheet);
// java int bordid = -1;
// java int patid = -1;
// java int fontid = -1;
// java int numFmtId = -1;
// java Rule rule = this.kmoCfRule.getRule();
// java int dxfId = rule.getDxfId();
// java if (dxfId == -1) {
// java this.condFmt10.addCfRule(this.kmoCfRule);
// java } else {
// java DxfPropertyId dxf = this.stylesTable.getdxf(dxfId);
// java if (null != dxf) {
// java bordid = dxf.getBordid();
// java patid = dxf.getPatid();
// java fontid = dxf.getFontid();
// java numFmtId = dxf.getNumFmtId();
// java }
// java
// java if (patid != -1) {
// java PatternFormatting pa = this.stylesTable.getPatternFillAt(patid);
// java if (pa != null) {
// java rule.setPattern(pa);
// java }
// java }
// java
// java if (bordid != -1) {
// java BorderFormatting bo = this.stylesTable.getBorderAt(bordid);
// java if (bo != null) {
// java dealBorderFormatting(bo);
// java KmoCfRule.exchangeBorderHairAndDotted(bo);
// java rule.setBorder(bo);
// java }
// java }
// java
// java if (fontid != -1) {
// java FontFormatting font = new FontFormatting();
// java iniFontFormat(this.stylesTable, font, fontid);
// java rule.setFont(font);
// java }
// java
// java if (numFmtId != -1) {
// java KmoFormat format = this.kmoXFManager.getFormat(numFmtId);
// java String fmtCode = null;
// java if (format != null) {
// java fmtCode = format.getFormatString();
// java }
// java
// java NumberFormatFormatting numFmt = new NumberFormatFormatting(numFmtId, fmtCode);
// java numFmt.setFmtCode(fmtCode);
// java rule.setNumFmt(numFmt);
// java }
// java
// java this.condFmt10.addCfRule(this.kmoCfRule);
// java if (this.kmoCfRule.isExt()) {
// java this.stylesTable.removedxf(dxf);
// java }
// java
// java }
// java }
fn endElement(&self,_e:i32){let cf=self.kmoCfRule.borrow().clone();cf.checkNeedCreateFormula(&self.condFmt10.getRefRangeList(),&self.sheet.borrow());let rule=cf.getRule();let dxfId=rule.getDxfId();if dxfId== -1{self.condFmt10.addCfRule(cf);return;}let table=self.stylesTable.borrow();let Some(dxf)=table.getdxf(dxfId)else{self.condFmt10.addCfRule(cf);return;};if dxf.getPatid()!= -1{rule.setPattern(table.getPatternFillAt(dxf.getPatid()));}if dxf.getBordid()!= -1{let border=table.getBorderAt(dxf.getBordid()).unwrap();Self::dealBorderFormatting(&mut border.borrow_mut());KmoCfRule::exchangeBorderHairAndDotted(&mut border.borrow_mut());rule.setBorder(Some(border));}if dxf.getFontid()!= -1{let font=Rc::new(FontFormatting::new());Self::iniFontFormat(&table,&font,dxf.getFontid());rule.setFont(Some(font));}if dxf.getNumFmtId()!= -1{let index=dxf.getNumFmtId();let book=self.sheet.borrow().getBook();let manager=book.getXFManager().borrow();let code=manager.getFormat(index).and_then(|f|f.getFormatString());let mut numfmt=NumberFormatFormatting::new(index,code.clone());numfmt.setFmtCode(code);rule.setNumFmt(Some(Rc::new(numfmt)));}self.condFmt10.addCfRule(cf.clone());if cf.isExt(){table.removedxf(&dxf);}}
fn characters(&self,_s:&str){}
}
struct FormulaHandler(CfRuleHandler);impl ElementHandler for FormulaHandler{fn startElement(&self,_e:i32,_a:&dyn XmlRoAttr){}fn endElement(&self,_e:i32){}fn enterSubElement(&self,_e:i32)->Option<Box<dyn ElementHandler>>{None}// java public void characters(String chars) {
// java if (!CfRuleHandler.this.condFmt10.getRefRangeList().isEmpty()) {
// java CfRuleHandler.this.kmoCfRule.addFormula(chars, CfRuleHandler.this.condFmt10.getEnclosingRange(), CfRuleHandler.this.sheet);
// java }
// java
// java }
fn characters(&self,s:&str){if !self.0.condFmt10.getRefRangeList().is_empty(){self.0.kmoCfRule.borrow().addFormula(s,&self.0.condFmt10.getEnclosingRange(),&self.0.sheet.borrow());}}}
struct FExtHandler(Rc<RefCell<Vec<String>>>);impl ElementHandler for FExtHandler{fn startElement(&self,_e:i32,_a:&dyn XmlRoAttr){}fn endElement(&self,_e:i32){}fn enterSubElement(&self,_e:i32)->Option<Box<dyn ElementHandler>>{None}// java public void characters(String chars) {
// java CfRuleHandler.this.formulas.add(chars);
// java }
fn characters(&self,s:&str){self.0.borrow_mut().push(s.to_string());}}
struct DxfExHandler{base:DxfHandlerRef,owner:CfRuleHandler}impl ElementHandler for DxfExHandler{fn startElement(&self,e:i32,a:&dyn XmlRoAttr){self.base.startElement(e,a);}fn characters(&self,s:&str){self.base.characters(s);}fn enterSubElement(&self,e:i32)->Option<Box<dyn ElementHandler>>{self.base.enterSubElement(e)}fn endElement(&self,e:i32){self.base.endElement(e);self.owner.kmoCfRule.borrow().getRule().setDxfId(self.owner.stylesTable.borrow().getDxfListSize()-1);}}
