use super::util::{MathX::MathX,statistics::StatsLib::StatsLib};use super::super::util::DoubleList::DoubleList;
pub struct Percentile;impl Percentile{
// java public static double percentile(DoubleList dl, double x) {
// java if (!(x > (double)1.0F) && !(x < (double)0.0F) && dl.size() != 0) {
// java if (dl.size() == 1) {
// java return dl.get(0);
// java } else if (x == (double)0.0F) {
// java return MathX.min(dl);
// java } else if (x == (double)1.0F) {
// java return MathX.max(dl);
// java } else {
// java double pos = x * (double)(dl.size() - 1);
// java int intPos = (int)pos;
// java StatsLib.nth_value(dl.innerArray(), 0, dl.size(), intPos);
// java double ret = dl.get(intPos);
// java double decPos = pos - (double)intPos;
// java if (decPos != (double)0.0F) {
// java double rightBound = MathX.min(dl.innerArray(), intPos + 1, dl.size());
// java ret += decPos * (rightBound - ret);
// java }
// java
// java return ret;
// java }
// java } else {
// java throw new IllegalArgumentException();
// java }
// java }
pub fn percentile(dl:&mut DoubleList,x:f64)->f64 {
assert!(!(x>1.0)&&!(x<0.0)&&dl.size()!=0,"IllegalArgumentException");let size=dl.size();if size==1{dl.get(0)}else if x==0.0{MathX::min(dl.innerArray(),0,size as usize)}else if x==1.0{MathX::max(dl.innerArray(),0,size as usize)}else{let pos=x*(size-1)as f64;let ipos=pos as i32;StatsLib::nth_value(dl.innerArray(),0,size as usize,ipos as usize);let mut ret=dl.get(ipos);let dec=pos-ipos as f64;if dec!=0.0{let bound=MathX::min(dl.innerArray(),ipos as usize+1,size as usize);ret+=dec*(bound-ret);}ret}
}
}
