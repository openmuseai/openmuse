use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
// java public static enum CfvoTypes {
// java num,
// java percent,
// java max,
// java min,
// java formula,
// java percentile,
// java autoMax,
// java autoMin;
// java }
// java }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum CfvoTypes {num,percent,max,min,formula,percentile,autoMax,autoMin}
impl CfvoTypes {pub fn valueOf(s:&str)->Option<Self>{match s{"num"=>Some(Self::num),"percent"=>Some(Self::percent),"max"=>Some(Self::max),"min"=>Some(Self::min),"formula"=>Some(Self::formula),"percentile"=>Some(Self::percentile),"autoMax"=>Some(Self::autoMax),"autoMin"=>Some(Self::autoMin),_=>None}}}
// java public class KmoCfvo implements Cloneable
pub struct KmoCfvo {pub rtype:Cell<Option<CfvoTypes>>,gte:Cell<bool>,val:Cell<f64>,formula:RefCell<Option<Rc<Vec<Ptg>>>>}
impl KmoCfvo {
// java private boolean gte=true;public CfvoTypes type=null;private double val;private Ptg[] formula;
pub fn new()->Self{Self{rtype:Cell::new(None),gte:Cell::new(true),val:Cell::new(0.0),formula:RefCell::new(None)}}
// java public boolean isEqual() {
// java return this.gte;
// java }
pub fn isEqual(&self)->bool {
self.gte.get()
}
// java public void setEqual(boolean isEqual) {
// java this.gte = isEqual;
// java }
pub fn setEqual(&self,v:bool) {
self.gte.set(v);
}
// java public double value() {
// java return this.val;
// java }
pub fn value(&self)->f64 {
self.val.get()
}
// java public void setValue(double value) {
// java this.val = value;
// java }
pub fn setValue(&self,v:f64) {
self.val.set(v);
}
// java public Ptg[] formula() {
// java return this.formula;
// java }
pub fn formula(&self)->Option<Rc<Vec<Ptg>>> {
self.formula.borrow().clone()
}
// java public void setFormula(Ptg[] formula) {
// java this.formula = formula;
// java }
pub fn setFormula(&self,v:Option<Rc<Vec<Ptg>>>) {
*self.formula.borrow_mut()=v;
}
// java public KmoCfvo clone() {
// java KmoCfvo cfvo = new KmoCfvo();
// java cfvo.gte = this.gte;
// java if (this.type != null) {
// java cfvo.type = KmoCfvo.CfvoTypes.valueOf(this.type.name());
// java }
// java
// java cfvo.val = this.val;
// java cfvo.formula = this.formula;
// java return cfvo;
// java }
pub fn clone(&self)->Self {
Self{rtype:Cell::new(self.rtype.get()),gte:Cell::new(self.gte.get()),val:Cell::new(self.val.get()),formula:RefCell::new(self.formula())}
}
}

impl KmoCfvo{// java public boolean equals(Object obj) {
// java if (this == obj) {
// java return true;
// java } else if (obj == null) {
// java return false;
// java } else if (this.getClass() != obj.getClass()) {
// java return false;
// java } else {
// java KmoCfvo other = (KmoCfvo)obj;
// java if (this.gte != other.gte) {
// java return false;
// java } else if (this.type != other.type) {
// java return false;
// java } else {
// java return Double.doubleToLongBits(this.val) == Double.doubleToLongBits(other.val);
// java }
// java }
// java }
pub fn equals(&self,o:&Self)->bool {
self.isEqual()==o.isEqual()&&self.rtype.get()==o.rtype.get()&&super::java_values::bits(self.value())==super::java_values::bits(o.value())
}
// java public int hashCode() {
// java int prime = 31;
// java int result = 1;
// java result = 31 * result + Arrays.hashCode(this.formula);
// java result = 31 * result + (this.gte ? 1231 : 1237);
// java result = 31 * result + (this.type == null ? 0 : this.type.hashCode());
// java long temp = Double.doubleToLongBits(this.val);
// java result = 31 * result + (int)(temp ^ temp >>> 32);
// java return result;
// java }
pub fn hashCode(&self)->i32 {
use super::java_values::{fold,boolean,formulas_hash,double};fold([formulas_hash(&self.formula().map(|p|p.as_ref().clone())),boolean(self.isEqual()),self.rtype.get().map_or(0,|v|v as i32+768),double(self.value())])
}
}
