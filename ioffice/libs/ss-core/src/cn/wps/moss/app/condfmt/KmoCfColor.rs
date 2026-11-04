use crate::{java_runtime::{Rc,RefCell,Cell},org::apache::poi::ss::SpreadsheetVersion::SpreadsheetVersion,cn::wps::et::ss::formula::ptg::Ptg::Ptg};
use crate::cn::wps::moss::core::theme::ColorScheme::ColorScheme;
// java public class KmoCfColor implements Cloneable
#[derive(Clone)]
pub struct KmoCfColor {pub auto:bool,pub indexed:i32,pub rgb:String,pub theme:i32,pub tint:f64,gradientRange:f64}
impl KmoCfColor {
// java auto=false;indexed=-1;rgb="";theme=-1;tint=-2;gradientRange=0;
pub fn new()->Self{Self{auto:false,indexed:-1,rgb:String::new(),theme:-1,tint:-2.0,gradientRange:0.0}}
// java public void setGrandientRange(double gradientRange) {
// java this.gradientRange = gradientRange;
// java }
pub fn setGrandientRange(&mut self,v:f64) {
self.gradientRange=v;
}
// java public double gradientRange() {
// java return this.gradientRange;
// java }
pub fn gradientRange(&self)->f64 {
self.gradientRange
}
// java public int getColor() {
// java if (this.theme != -1) {
// java return this.tint == (double)-2.0F ? ColorScheme.getColor(this.theme, 0.0F) : ColorScheme.getColor(this.theme, (float)this.tint);
// java } else if (this.auto) {
// java return 16777215;
// java } else if (this.indexed != -1) {
// java return this.indexed;
// java } else {
// java int ret = CfRuleUtil.colorToInt(this.rgb);
// java ret |= -16777216;
// java return ret;
// java }
// java }
pub fn getColor(&self)->i32 {
if self.theme!= -1{ColorScheme::getColor(self.theme,if self.tint== -2.0{0.0}else{self.tint as f32})}else if self.auto{16777215}else if self.indexed!= -1{self.indexed}else{let b=self.rgb.as_bytes();let v=match b.len(){8=>i32::from_str_radix(&self.rgb[2..],16).expect("NumberFormatException"),6=>i32::from_str_radix(&self.rgb,16).expect("NumberFormatException"),_=>0};v| -16777216}
}
}

impl KmoCfColor{// java public boolean equals(Object obj) {
// java if (this == obj) {
// java return true;
// java } else if (obj == null) {
// java return false;
// java } else if (this.getClass() != obj.getClass()) {
// java return false;
// java } else {
// java KmoCfColor other = (KmoCfColor)obj;
// java if (this.auto != other.auto) {
// java return false;
// java } else if (Double.doubleToLongBits(this.gradientRange) != Double.doubleToLongBits(other.gradientRange)) {
// java return false;
// java } else if (this.indexed != other.indexed) {
// java return false;
// java } else {
// java if (this.rgb == null) {
// java if (other.rgb != null) {
// java return false;
// java }
// java } else if (!this.rgb.equals(other.rgb)) {
// java return false;
// java }
// java
// java if (this.theme != other.theme) {
// java return false;
// java } else {
// java return Double.doubleToLongBits(this.tint) == Double.doubleToLongBits(other.tint);
// java }
// java }
// java }
// java }
pub fn equals(&self,o:&Self)->bool {
use super::java_values::bits;self.auto==o.auto&&bits(self.gradientRange)==bits(o.gradientRange)&&self.indexed==o.indexed&&self.rgb==o.rgb&&self.theme==o.theme&&bits(self.tint)==bits(o.tint)
}
// java public int hashCode() {
// java int prime = 31;
// java int result = 1;
// java result = 31 * result + (this.auto ? 1231 : 1237);
// java long temp = Double.doubleToLongBits(this.gradientRange);
// java result = 31 * result + (int)(temp ^ temp >>> 32);
// java result = 31 * result + this.indexed;
// java result = 31 * result + (this.rgb == null ? 0 : this.rgb.hashCode());
// java result = 31 * result + this.theme;
// java temp = Double.doubleToLongBits(this.tint);
// java result = 31 * result + (int)(temp ^ temp >>> 32);
// java return result;
// java }
pub fn hashCode(&self)->i32 {
use super::java_values::{fold,boolean,double,string};fold([boolean(self.auto),double(self.gradientRange),self.indexed,string(&self.rgb),self.theme,double(self.tint)])
}
}
