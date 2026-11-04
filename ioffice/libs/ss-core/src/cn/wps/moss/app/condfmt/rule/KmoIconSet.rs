use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
use super::Rule::{Rule,RuleObject,CfRuleTypes};
use super::super::{KmoCfvo::KmoCfvo,KmoCfColor::KmoCfColor};
// java public static enum IconSetTypes {
// java $3Arrows(1),
// java $3ArrowsGray(2),
// java $3Flags(3),
// java $3TrafficLights1(4),
// java $3TrafficLights2(5),
// java $3Signs(6),
// java $3Symbols(7),
// java $3Symbols2(8),
// java $4Arrows(9),
// java $4ArrowsGray(10),
// java $4RedToBlack(11),
// java $4Rating(12),
// java $4TrafficLights(13),
// java $5Arrows(14),
// java $5ArrowsGray(15),
// java $5Rating(16),
// java $5Quarters(17),
// java $3Stars(18),
// java $3Triangles(19),
// java $5Boxes(20);
// java
// java public final int tag;
// java
// java private IconSetTypes(int tag) {
// java this.tag = tag;
// java }
// java }
// java }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[repr(i32)]
pub enum IconSetTypes{I3Arrows=1,I3ArrowsGray=2,I3Flags=3,I3TrafficLights1=4,I3TrafficLights2=5,I3Signs=6,I3Symbols=7,I3Symbols2=8,I4Arrows=9,I4ArrowsGray=10,I4RedToBlack=11,I4Rating=12,I4TrafficLights=13,I5Arrows=14,I5ArrowsGray=15,I5Rating=16,I5Quarters=17,I3Stars=18,I3Triangles=19,I5Boxes=20}
impl IconSetTypes{pub fn valueOf(s:&str)->Option<Self>{match s{"$3Arrows"=>Some(Self::I3Arrows),"$3ArrowsGray"=>Some(Self::I3ArrowsGray),"$3Flags"=>Some(Self::I3Flags),"$3TrafficLights1"=>Some(Self::I3TrafficLights1),"$3TrafficLights2"=>Some(Self::I3TrafficLights2),"$3Signs"=>Some(Self::I3Signs),"$3Symbols"=>Some(Self::I3Symbols),"$3Symbols2"=>Some(Self::I3Symbols2),"$4Arrows"=>Some(Self::I4Arrows),"$4ArrowsGray"=>Some(Self::I4ArrowsGray),"$4RedToBlack"=>Some(Self::I4RedToBlack),"$4Rating"=>Some(Self::I4Rating),"$4TrafficLights"=>Some(Self::I4TrafficLights),"$5Arrows"=>Some(Self::I5Arrows),"$5ArrowsGray"=>Some(Self::I5ArrowsGray),"$5Rating"=>Some(Self::I5Rating),"$5Quarters"=>Some(Self::I5Quarters),"$3Stars"=>Some(Self::I3Stars),"$3Triangles"=>Some(Self::I3Triangles),"$5Boxes"=>Some(Self::I5Boxes),_=>None}}}
// java public class KmoIconSet extends Rule implements Cloneable
pub struct KmoIconSet{base:Rule,itype:Cell<Option<IconSetTypes>>,percent:Cell<bool>,reverse:Cell<bool>,showValue:Cell<bool>,cfvoList:RefCell<Rc<RefCell<Vec<Rc<KmoCfvo>>>>>}
impl std::ops::Deref for KmoIconSet{type Target=Rule;fn deref(&self)->&Rule{&self.base}}
impl RuleObject for KmoIconSet{fn javaEquals(&self,o:&dyn RuleObject)->bool{o.as_any().downcast_ref::<Self>().is_some_and(|o|self.equals(o))}fn javaHashCode(&self)->i32{self.hashCode()}fn base(&self)->&Rule{&self.base}fn as_any(&self)->&dyn std::any::Any{self}}
impl KmoIconSet{
// java public KmoIconSet(SpreadsheetVersion sv) {
// java super(sv);
// java this.setType(Rule.CfRuleTypes.iconSet);
// java this.cfvoList = new ArrayList();
// java }
pub fn new(sv:SpreadsheetVersion)->Self {
let base=Rule::new(sv);base.setType(Some(CfRuleTypes::iconSet));Self{base,itype:Cell::new(None),percent:Cell::new(false),reverse:Cell::new(false),showValue:Cell::new(true),cfvoList:RefCell::new(Rc::new(RefCell::new(vec![])))}
}
// java public List<KmoCfvo> cfvos() {
// java return this.cfvoList;
// java }
pub fn cfvos(&self)->Rc<RefCell<Vec<Rc<KmoCfvo>>>> {
self.cfvoList.borrow().clone()
}
// java public void addCfvo(KmoCfvo cfvo) {
// java this.cfvoList.add(cfvo);
// java }
pub fn addCfvo(&self,v:Rc<KmoCfvo>) {
self.cfvoList.borrow().borrow_mut().push(v);
}
// java public void setCfvos(List<KmoCfvo> cfvos) {
// java this.cfvoList = cfvos;
// java }
pub fn setCfvos(&self,v:Rc<RefCell<Vec<Rc<KmoCfvo>>>>) {
*self.cfvoList.borrow_mut()=v;
}
// java public List<Ptg[]> formulas() {
// java List<Ptg[]> ptgs = new ArrayList();
// java
// java for(KmoCfvo cfvo : this.cfvoList) {
// java Ptg[] formula = cfvo.formula();
// java if (formula != null) {
// java ptgs.add(formula);
// java }
// java }
// java
// java return ptgs;
// java }
pub fn formulas(&self)->Vec<Rc<Vec<Ptg>>> {
self.cfvos().borrow().iter().filter_map(|v|v.formula()).collect()
}
// java public boolean isShowValue() {
// java return this.showValue;
// java }
pub fn isShowValue(&self)->bool {
self.showValue.get()
}
// java public void setShowValue(boolean showValue) {
// java this.showValue = showValue;
// java }
pub fn setShowValue(&self,v:bool) {
self.showValue.set(v);
}
// java public IconSetTypes getIconSetType() {
// java return this.type;
// java }
pub fn getIconSetType(&self)->Option<IconSetTypes> {
self.itype.get()
}
// java public void setIconSetType(IconSetTypes type) {
// java this.type = type;
// java }
pub fn setIconSetType(&self,v:Option<IconSetTypes>) {
self.itype.set(v);
}
// java public boolean isPercent() {
// java return this.percent;
// java }
pub fn isPercent(&self)->bool {
self.percent.get()
}
// java public void setPercent(boolean percent) {
// java this.percent = percent;
// java }
pub fn setPercent(&self,v:bool) {
self.percent.set(v);
}
// java public boolean isReverse() {
// java return this.reverse;
// java }
pub fn isReverse(&self)->bool {
self.reverse.get()
}
// java public void setReverse(boolean reverse) {
// java this.reverse = reverse;
// java }
pub fn setReverse(&self,v:bool) {
self.reverse.set(v);
}
}

impl KmoIconSet{// java public boolean equals(Object obj) {
// java if (obj == null) {
// java return false;
// java } else if (this == obj) {
// java return true;
// java } else if (this.getClass() != obj.getClass()) {
// java return false;
// java } else {
// java KmoIconSet other = (KmoIconSet)obj;
// java if (this.cfvoList == null) {
// java if (other.cfvoList != null) {
// java return false;
// java }
// java } else if (!this.cfvoList.equals(other.cfvoList)) {
// java return false;
// java }
// java
// java if (this.percent != other.percent) {
// java return false;
// java } else if (this.reverse != other.reverse) {
// java return false;
// java } else if (this.showValue != other.showValue) {
// java return false;
// java } else {
// java return this.type == other.type;
// java }
// java }
// java }
pub fn equals(&self,o:&Self)->bool{std::ptr::eq(self,o)||(self.isPercent()==o.isPercent()&&self.isReverse()==o.isReverse()&&self.isShowValue()==o.isShowValue()&&self.getIconSetType()==o.getIconSetType()&&{let a=self.cfvos();let b=o.cfvos();let a=a.borrow();let b=b.borrow();a.len()==b.len()&&a.iter().zip(b.iter()).all(|(a,b)|a.equals(b))})}// java public int hashCode() {
// java int prime = 31;
// java int result = 1;
// java result = 31 * result + (this.cfvoList == null ? 0 : this.cfvoList.hashCode());
// java result = 31 * result + (this.percent ? 1231 : 1237);
// java result = 31 * result + (this.reverse ? 1231 : 1237);
// java result = 31 * result + (this.showValue ? 1231 : 1237);
// java result = 31 * result + (this.type == null ? 0 : this.type.hashCode());
// java return result;
// java }
pub fn hashCode(&self)->i32{use super::super::java_values::{fold,boolean};fold([fold(self.cfvos().borrow().iter().map(|v|v.hashCode())),boolean(self.isPercent()),boolean(self.isReverse()),boolean(self.isShowValue()),self.getIconSetType().map_or(0,|v|v as i32+1024)])}}

impl IconSetTypes{
// rust: Java Enum.name() adapter retains the original dollar-prefixed identifiers.
pub fn name(&self)->&'static str{match self{Self::I3Arrows=>"$3Arrows",Self::I3ArrowsGray=>"$3ArrowsGray",Self::I3Flags=>"$3Flags",Self::I3TrafficLights1=>"$3TrafficLights1",Self::I3TrafficLights2=>"$3TrafficLights2",Self::I3Signs=>"$3Signs",Self::I3Symbols=>"$3Symbols",Self::I3Symbols2=>"$3Symbols2",Self::I4Arrows=>"$4Arrows",Self::I4ArrowsGray=>"$4ArrowsGray",Self::I4RedToBlack=>"$4RedToBlack",Self::I4Rating=>"$4Rating",Self::I4TrafficLights=>"$4TrafficLights",Self::I5Arrows=>"$5Arrows",Self::I5ArrowsGray=>"$5ArrowsGray",Self::I5Rating=>"$5Rating",Self::I5Quarters=>"$5Quarters",Self::I3Stars=>"$3Stars",Self::I3Triangles=>"$3Triangles",Self::I5Boxes=>"$5Boxes"}}}
