use crate::{java_runtime::Rc,cn::wps::moss::filefmt::biff8::record::PaletteRecord::PaletteRecord};
// java public class KmoColorSimilar
pub struct KmoColorSimilar {_palette:Rc<PaletteRecord>,red:i32,green:i32,blue:i32,g_gray:Vec<paletteColor>,g_total:Vec<paletteColor>,g_range:Vec<Vec<paletteColor>>,H:i32,S:i32,L:i32,TH:f32,TS:f32,TL:f32}
#[derive(Clone)]
pub struct paletteColor {red:i32,green:i32,blue:i32,no:i32}
impl KmoColorSimilar {
// java public KmoColorSimilar(PaletteRecord _palette, int red, int green, int blue) {
// java this._palette = _palette;
// java this.red = red;
// java this.green = green;
// java this.blue = blue;
// java
// java for(int i = 0; i < 10; ++i) {
// java Vector<paletteColor> ve = new Vector();
// java this.g_range.add(ve);
// java }
// java
// java }
pub fn new(palette:Rc<PaletteRecord>,red:i32,green:i32,blue:i32)->Self {
Self{_palette:palette,red,green,blue,g_gray:Vec::new(),g_total:Vec::new(),g_range:vec![Vec::new();10],H:0,S:0,L:0,TH:0.0,TS:0.0,TL:0.0}
}
// java public boolean _EQ(int A, int B) {
// java return Math.abs(A - B) < 10;
// java }
pub fn _EQ(&self,a:i32,b:i32)->bool {
a.wrapping_sub(b).wrapping_abs()<10
}
// java public int _GetHueIndex(int H) {
// java for(int i = 0; i < this.RANGE_COUNT - 1; ++i) {
// java if (H >= this.RANGE_HUE[i] && H < this.RANGE_HUE[i + 1]) {
// java return i;
// java }
// java }
// java
// java return 0;
// java }
pub fn _GetHueIndex(&self,h:i32)->usize {
let range=[0,30,50,110,170,240];for i in 0..5{if h>=range[i]&&h<range[i+1]{return i;}}0
}
// java public void _RGB2HSL(int R, int G, int B) {
// java float r = (float)R / 255.0F;
// java float g = (float)G / 255.0F;
// java float b = (float)B / 255.0F;
// java float max = this.max(r, this.max(g, b));
// java float min = this.min(r, this.min(g, b));
// java float sum = max + min;
// java float del = max - min;
// java this.TH = 240.0F;
// java this.TS = 0.0F;
// java this.TL = sum / 2.0F;
// java if (del != 0.0F) {
// java this.TS = del / (this.TL <= 0.5F ? sum : 2.0F - sum);
// java if (r == max) {
// java this.TH = 60.0F * (g - b) / del;
// java } else if (g == max) {
// java this.TH = 60.0F * (b - r) / del + 120.0F;
// java } else {
// java this.TH = 60.0F * (r - g) / del + 240.0F;
// java }
// java
// java if (this.TH < 0.0F) {
// java this.TH += 360.0F;
// java }
// java
// java if (this.TH >= 360.0F) {
// java this.TH -= 360.0F;
// java }
// java }
// java
// java this.H = (int)(this.TH / 360.0F * 240.0F);
// java this.S = (int)(this.TS * 240.0F);
// java this.L = (int)(this.TL * 240.0F);
// java }
pub fn _RGB2HSL(&mut self,red:i32,green:i32,blue:i32) {
let r=red as f32/255.0;let g=green as f32/255.0;let b=blue as f32/255.0;let max=if g>b{g}else{b};let max=if r>max{r}else{max};let min=if g<b{g}else{b};let min=if r<min{r}else{min};let sum=max+min;let del=max-min;self.TH=240.0;self.TS=0.0;self.TL=sum/2.0;if del!=0.0{self.TS=del/if self.TL<=0.5{sum}else{2.0-sum};self.TH=if r==max{60.0*(g-b)/del}else if g==max{60.0*(b-r)/del+120.0}else{60.0*(r-g)/del+240.0};if self.TH<0.0{self.TH+=360.0;}if self.TH>=360.0{self.TH-=360.0;}}self.H=(self.TH/360.0*240.0) as i32;self.S=(self.TS*240.0) as i32;self.L=(self.TL*240.0) as i32;
}
// java public int _DoMatchByHSL() {
// java this._RGB2HSL(this.red, this.green, this.blue);
// java if (this.H >= 226) {
// java this.H = 0;
// java }
// java
// java Vector<paletteColor> pVect = this.g_total;
// java float PH = 0.02F;
// java float PS = 0.08F;
// java float PL = 0.9F;
// java if (this.S > 15 && this.L < 230 && this.L > 15) {
// java PH = 0.34F;
// java PS = 0.16F;
// java PL = 0.5F;
// java pVect = (Vector)this.g_range.get(this._GetHueIndex(this.H));
// java }
// java
// java int nIndex = 0;
// java double dMinDist = (double)Integer.MAX_VALUE;
// java int H1 = this.H;
// java int S1 = this.S;
// java int L1 = this.L;
// java
// java for(int i = 0; i < pVect.size(); ++i) {
// java this._RGB2HSL(((paletteColor)pVect.get(i)).red, ((paletteColor)pVect.get(i)).green, ((paletteColor)pVect.get(i)).blue);
// java int DH = Math.abs(H1 - this.H);
// java int DS = Math.abs(S1 - this.S);
// java int DL = Math.abs(L1 - this.L);
// java double dDist = (double)((float)DH * PH + (float)DS * PS + (float)DL * PL);
// java if (dDist < dMinDist) {
// java nIndex = ((paletteColor)pVect.get(i)).no;
// java dMinDist = dDist;
// java }
// java }
// java
// java return nIndex;
// java }
pub fn _DoMatchByHSL(&mut self)->i32 {
self._RGB2HSL(self.red,self.green,self.blue);if self.H>=226{self.H=0;}let(mut ph,mut ps,mut pl)=(0.02f32,0.08f32,0.9f32);let mut candidates=self.g_total.clone();if self.S>15&&self.L<230&&self.L>15{ph=0.34;ps=0.16;pl=0.5;candidates=self.g_range[self._GetHueIndex(self.H)].clone();}let(mut index,mut min_dist)=(0,i32::MAX as f64);let(h,s,l)=(self.H,self.S,self.L);for c in candidates{self._RGB2HSL(c.red,c.green,c.blue);let dist=((h-self.H).abs() as f32*ph+(s-self.S).abs() as f32*ps+(l-self.L).abs() as f32*pl) as f64;if dist<min_dist{index=c.no;min_dist=dist;}}index
}
// java public int DoGetSimilarColor() {
// java byte[] b = this._palette.getColor(8);
// java
// java for(short i = 8; b != null; b = this._palette.getColor(i)) {
// java int r = this.unsignedInt(b[0]);
// java int g = this.unsignedInt(b[1]);
// java int d = this.unsignedInt(b[2]);
// java paletteColor ColorCell = new paletteColor(r, g, d, i);
// java if (this._EQ(r, g) && this._EQ(g, d)) {
// java this.g_gray.add(ColorCell);
// java } else {
// java this._RGB2HSL(r, g, d);
// java int no = this._GetHueIndex(this.H);
// java Vector<paletteColor> ve = (Vector)this.g_range.get(no);
// java ve.add(ColorCell);
// java }
// java
// java this.g_total.add(ColorCell);
// java ++i;
// java }
// java
// java if (this._EQ(this.red, this.green) && this._EQ(this.green, this.blue)) {
// java int nIndex = 0;
// java int nMinDist = 256;
// java
// java for(int i = 0; i < this.g_gray.size(); ++i) {
// java int nDist = Math.abs(((paletteColor)this.g_gray.get(i)).red - this.red);
// java if (nDist < nMinDist) {
// java nIndex = ((paletteColor)this.g_gray.get(i)).no;
// java nMinDist = nDist;
// java }
// java }
// java
// java return nIndex;
// java } else {
// java for(short i = 8; b != null; b = this._palette.getColor(i)) {
// java int r = this.unsignedInt(b[0]);
// java int g = this.unsignedInt(b[1]);
// java int d = this.unsignedInt(b[2]);
// java if (this._EQ(this.red, r) && this._EQ(this.green, g) && this._EQ(this.blue, d)) {
// java return i;
// java }
// java
// java ++i;
// java }
// java
// java return this._DoMatchByHSL();
// java }
// java }
pub fn DoGetSimilarColor(&mut self)->i32 {
let mut i=8i16;while let Some(b)=self._palette.getColor(i as i32){let(r,g,d)=(b[0] as u8 as i32,b[1] as u8 as i32,b[2] as u8 as i32);let cell=paletteColor{red:r,green:g,blue:d,no:i as i32};if self._EQ(r,g)&&self._EQ(g,d){self.g_gray.push(cell.clone());}else{self._RGB2HSL(r,g,d);let no=self._GetHueIndex(self.H);self.g_range[no].push(cell.clone());}self.g_total.push(cell);i=i.wrapping_add(1);}if self._EQ(self.red,self.green)&&self._EQ(self.green,self.blue){let(mut index,mut min_dist)=(0,256);for c in &self.g_gray{let distance=(c.red-self.red).abs();if distance<min_dist{index=c.no;min_dist=distance;}}index}else{
// rust: Java's second for loop reuses b after the preceding loop exhausted it to null.
self._DoMatchByHSL()}
}
}
