use serde::{Serialize,Deserialize};
// rust: Android DisplayMetrics is supplied by the native host, in physical pixels.
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct DisplayMetrics {pub scaledDensity:f32,pub density:f32,pub xdpi:f32,pub ydpi:f32,pub widthPixels:i32,pub heightPixels:i32}
impl Default for DisplayMetrics{fn default()->Self{Self{scaledDensity:1.0,density:1.0,xdpi:96.0,ydpi:96.0,widthPixels:0,heightPixels:0}}}
// java public final class UnitsConverter implements Cloneable
#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct UnitsConverter{pub mDisplayMetrics:DisplayMetrics,pub mScaleFactor:f32,pub mXDPI:f32,pub mYDPI:f32}
impl UnitsConverter{
pub const DPI_SCALE:f32=0.75;
// java public UnitsConverter(Context context) {
// java this.mDisplayMetrics = new DisplayMetrics();
// java this.fillDisplayMetrics(context, this.mDisplayMetrics);
// java this.mScaleFactor = this.mDisplayMetrics.scaledDensity;
// java this.mXDPI = this.mDisplayMetrics.xdpi > 64.0F ? this.mDisplayMetrics.xdpi : 96.0F;
// java this.mYDPI = this.mDisplayMetrics.ydpi > 64.0F ? this.mDisplayMetrics.ydpi : 96.0F;
// java if (Math.abs(this.mXDPI - this.mYDPI) / this.mXDPI >= 0.2F) {
// java this.mYDPI = this.mXDPI;
// java }
// java
// java this.mXDPI = 96.0F * (1.0F + 96.0F / this.mXDPI);
// java this.mYDPI = 96.0F * (1.0F + 96.0F / this.mYDPI);
// java this.mXDPI *= 0.75F;
// java this.mYDPI *= 0.75F;
// java }
pub fn UnitsConverter(metrics:DisplayMetrics)->Self{let mut x=if metrics.xdpi>64.0{metrics.xdpi}else{96.0};let mut y=if metrics.ydpi>64.0{metrics.ydpi}else{96.0};if (x-y).abs()/x>=0.2{y=x;}x=96.0*(1.0+96.0/x);y=96.0*(1.0+96.0/y);x*=0.75;y*=0.75;Self{mScaleFactor:metrics.scaledDensity,mDisplayMetrics:metrics,mXDPI:x,mYDPI:y}}
// java public UnitsConverter(Context context, float xDPI, float yDPI) {
// java this.mDisplayMetrics = new DisplayMetrics();
// java this.mDisplayMetrics.scaledDensity = 1.0F;
// java this.mScaleFactor = this.mDisplayMetrics.scaledDensity;
// java this.mXDPI = xDPI;
// java this.mYDPI = yDPI;
// java }
pub fn UnitsConverter__2(x:f32,y:f32)->Self{Self{mDisplayMetrics:DisplayMetrics::default(),mScaleFactor:1.0,mXDPI:x,mYDPI:y}}
// java public float getXDpi() {
// java return this.mXDPI;
// java }
pub fn getXDpi(&self)->f32{self.mXDPI}
// java public int displayWidth() {
// java return this.mDisplayMetrics.widthPixels;
// java }
pub fn displayWidth(&self)->i32{self.mDisplayMetrics.widthPixels}
// java public int displayHeight() {
// java return this.mDisplayMetrics.heightPixels;
// java }
pub fn displayHeight(&self)->i32{self.mDisplayMetrics.heightPixels}
// java public static float calculateXDPI(float xdpi) {
// java float XDPI = xdpi > 64.0F ? xdpi : 96.0F;
// java XDPI = 96.0F * (1.0F + 96.0F / XDPI) * 0.75F;
// java return XDPI;
// java }
pub fn calculateXDPI(xdpi:f32)->f32{{let x=if xdpi>64.0{xdpi}else{96.0};96.0*(1.0+96.0/x)*0.75}}
// java public static float calculateYDPI(float ydpi) {
// java float YDPI = ydpi > 64.0F ? ydpi : 96.0F;
// java YDPI = 96.0F * (1.0F + 96.0F / YDPI) * 0.75F;
// java return YDPI;
// java }
pub fn calculateYDPI(ydpi:f32)->f32{{let y=if ydpi>64.0{ydpi}else{96.0};96.0*(1.0+96.0/y)*0.75}}
// java public void setDPI(float xDPI, float yDPI) {
// java this.mXDPI = xDPI;
// java this.mYDPI = yDPI;
// java }
pub fn setDPI(&mut self,x:f32,y:f32){self.mXDPI=x;self.mYDPI=y;}
// java public float cmToPixels(float cm) {
// java float pt = cm * 28.35F;
// java return this.PointsToPixels(pt);
// java }
pub fn cmToPixels(&self,cm:f32)->f32{self.PointsToPixels(cm*28.35)}
// java public float PixelsToPoints(float pixels) {
// java return pixels / this.mScaleFactor / this.mXDPI * 72.0F;
// java }
pub fn PixelsToPoints(&self,pixels:f32)->f32{pixels/self.mScaleFactor/self.mXDPI*72.0}
// java public float PixelsToPointsY(float pixels) {
// java return pixels / this.mScaleFactor / this.mYDPI * 72.0F;
// java }
pub fn PixelsToPointsY(&self,pixels:f32)->f32{pixels/self.mScaleFactor/self.mYDPI*72.0}
// java public float PointsToPixels(float points) {
// java return points * this.mScaleFactor * this.mXDPI * 0.013888889F;
// java }
pub fn PointsToPixels(&self,points:f32)->f32{points*self.mScaleFactor*self.mXDPI*0.013888889}
// java public float PointsToPixelsUnScale(float points) {
// java return points * this.mDisplayMetrics.scaledDensity * this.mXDPI * 0.013888889F;
// java }
pub fn PointsToPixelsUnScale(&self,points:f32)->f32{points*self.mDisplayMetrics.scaledDensity*self.mXDPI*0.013888889}
// java public float PointsToPixelsY(float points) {
// java return points * this.mScaleFactor * this.mYDPI * 0.013888889F;
// java }
pub fn PointsToPixelsY(&self,points:f32)->f32{points*self.mScaleFactor*self.mYDPI*0.013888889}
// java public float TwipsToPixels(float twips) {
// java float points = twips / 20.0F;
// java return this.PointsToPixels(points);
// java }
pub fn TwipsToPixels(&self,twips:f32)->f32{self.PointsToPixels(twips/20.0)}
// java public float TwipsToPixelsY(float twips) {
// java float points = twips / 20.0F;
// java return this.PointsToPixelsY(points);
// java }
pub fn TwipsToPixelsY(&self,twips:f32)->f32{self.PointsToPixelsY(twips/20.0)}
// java public float twipsToPixelsYUnScale(float twips) {
// java float points = twips / 20.0F;
// java return points * this.mDisplayMetrics.scaledDensity * this.mYDPI * 0.013888889F;
// java }
pub fn twipsToPixelsYUnScale(&self,twips:f32)->f32{(twips/20.0)*self.mDisplayMetrics.scaledDensity*self.mYDPI*0.013888889}
// java public int PixelsToTwips(float pixels) {
// java float points = this.PixelsToPoints(pixels);
// java return (int)(points * 20.0F);
// java }
pub fn PixelsToTwips(&self,pixels:f32)->i32{(self.PixelsToPoints(pixels)*20.0)as i32}
// java public int PixelsToTwipsY(int pixels) {
// java float points = this.PixelsToPointsY((float)pixels);
// java return (int)(points * 20.0F);
// java }
pub fn PixelsToTwipsY(&self,pixels:i32)->i32{(self.PixelsToPointsY(pixels as f32)*20.0)as i32}
// java public int SpToPixels(float sp) {
// java return (int)(sp * this.mDisplayMetrics.scaledDensity + 0.5F);
// java }
pub fn SpToPixels(&self,sp:f32)->i32{(sp*self.mDisplayMetrics.scaledDensity+0.5)as i32}
// java public void setZoom(int factor) {
// java this.mScaleFactor = this.mDisplayMetrics.scaledDensity * (float)factor / 100.0F;
// java }
pub fn setZoom(&mut self,factor:i32){self.mScaleFactor=self.mDisplayMetrics.scaledDensity*factor as f32/100.0;}
// java public float getZoom() {
// java return this.mScaleFactor;
// java }
pub fn getZoom(&self)->f32{self.mScaleFactor}
// java public float getUnscaleZoom() {
// java return this.mScaleFactor / this.mDisplayMetrics.scaledDensity;
// java }
pub fn getUnscaleZoom(&self)->f32{self.mScaleFactor/self.mDisplayMetrics.scaledDensity}
// java public float getScaledDensity() {
// java return this.mDisplayMetrics.scaledDensity;
// java }
pub fn getScaledDensity(&self)->f32{self.mDisplayMetrics.scaledDensity}
// java public void modifyScale(float pixel2Points) {
// java float scale = pixel2Points * this.mXDPI * 0.013888889F;
// java this.mScaleFactor = scale;
// java }
pub fn modifyScale(&mut self,pixel2Points:f32){self.mScaleFactor=pixel2Points*self.mXDPI*0.013888889;}
// java public static final int columnUnitsToPixels(float w, float cw) {
// java return (int)((w + (float)((int)(128.0F / cw))) / 256.0F * cw);
// java }
pub fn columnUnitsToPixels(w:f32,cw:f32)->i32{((w+(128.0/cw)as i32 as f32)/256.0*cw)as i32}
// java public static final int pixelsToColumnUnits(float w, float cw) {
// java float charWidth = pixelsToCharWidth(w, cw);
// java return charWidthToColumnUnits(charWidth, cw);
// java }
pub fn pixelsToColumnUnits(w:f32,cw:f32)->i32{Self::charWidthToColumnUnits(Self::pixelsToCharWidth(w,cw),cw)}
// java public static final float pixelsToCharWidth(float w, float cw) {
// java return (float)((int)((w - 5.0F) / cw * 100.0F + 0.5F)) * 0.01F;
// java }
pub fn pixelsToCharWidth(w:f32,cw:f32)->f32{(((w-5.0)/cw*100.0+0.5)as i32)as f32*0.01}
// java public static final int charWidthToColumnUnits(float w, float cw) {
// java return (int)((w * cw + 5.0F) / cw * 256.0F);
// java }
pub fn charWidthToColumnUnits(w:f32,cw:f32)->i32{((w*cw+5.0)/cw*256.0)as i32}
// java public static final float columnUnitsToCharWidth(float w, float cw) {
// java float px = (float)columnUnitsToPixels(w, cw);
// java return pixelsToCharWidth(px, cw);
// java }
pub fn columnUnitsToCharWidth(w:f32,cw:f32)->f32{Self::pixelsToCharWidth(Self::columnUnitsToPixels(w,cw)as f32,cw)}
// java public static final int dp2pix(float dp) {
// java return (int)(Platform.getDisplayMetrics().density * dp + 0.5F);
// java }
pub fn dp2pix(dp:f32,density:f32)->i32{(density*dp+0.5)as i32}
// java public static final int twip2point(int twips) {
// java return twips / 20;
// java }
pub fn twip2point(twips:i32)->i32{twips/20}
// java public static final int point2twip(int points) {
// java return points * 20;
// java }
pub fn point2twip(points:i32)->i32{points.wrapping_mul(20)}
// java public static final float inch2point(float inch) {
// java return inch * 72.0F;
// java }
pub fn inch2point(inch:f32)->f32{inch*72.0}
// java public static final float point2inch(float point) {
// java return point * 0.013888889F;
// java }
pub fn point2inch(point:f32)->f32{point*0.013888889}
// java public int pixelsToEmus(int p) {
// java return (int)((float)p / this.mXDPI * 72.0F / 28.35F * 360000.0F);
// java }
pub fn pixelsToEmus(&self,p:i32)->i32{(p as f32/self.mXDPI*72.0/28.35*360000.0)as i32}
// java public int emusToPixel(float emus) {
// java return (int)(emus / 360000.0F / 72.0F * 28.35F * this.mXDPI);
// java }
pub fn emusToPixel(&self,emus:f32)->i32{(emus/360000.0/72.0*28.35*self.mXDPI)as i32}
// java public static final float twip2point(float twips) {
// java return twips / 20.0F;
// java }
pub fn twip2point__2(twips:f32)->f32{twips/20.0}
// java public static final float point2twip(float points) {
// java return points * 20.0F;
// java }
pub fn point2twip__2(points:f32)->f32{points*20.0}
// java public UnitsConverter clone() throws CloneNotSupportedException {
// java UnitsConverter converter = new UnitsConverter((Context)null, this.mXDPI, this.mYDPI);
// java converter.mDisplayMetrics = new DisplayMetrics();
// java converter.mDisplayMetrics.scaledDensity = this.mDisplayMetrics.scaledDensity;
// java converter.mScaleFactor = this.mScaleFactor;
// java return converter;
// java }
pub fn clone(&self)->Self{Clone::clone(self)}
}
