use crate::{java_runtime::{Rc,RefCell,Cell},cn::wps::moss::{filefmt::biff8::record::PaletteRecord::PaletteRecord,util::KmoColor::{KmoColor,Color,Triplet,AUTOMATIC}}};
use util::org::apache::poi::util::LittleEndianOutput::LittleEndianOutput;
// java public final class KmoPalette
pub struct KmoPalette {_palette:Rc<PaletteRecord>,basicColors:Rc<RefCell<[i32;40]>>,isPaletteMod:Cell<bool>}
impl KmoPalette {
pub const BASIC_PALETTE_SIZE:i32=40;
// java public KmoPalette() {
// java this.isPaletteMod = false;
// java this._palette = new PaletteRecord();
// java this.createBasicColor();
// java }
pub fn new()->Self {
let p=Self{_palette:Rc::new(PaletteRecord::new()),basicColors:Rc::new(RefCell::new([0;40])),isPaletteMod:Cell::new(false)};p.createBasicColor();p
}
// java public KmoPalette(PaletteRecord record) {
// java this.isPaletteMod = true;
// java this._palette = record;
// java this.createBasicColor();
// java }
pub fn fromRecord(record:Rc<PaletteRecord>)->Self {
let p=Self{_palette:record,basicColors:Rc::new(RefCell::new([0;40])),isPaletteMod:Cell::new(true)};p.createBasicColor();p
}
// java private void createBasicColor() {
// java int j = 0;
// java
// java for(int i = 8; i < 64; ++i) {
// java int color = this._palette.getRGBColor(i);
// java boolean isRepeat = false;
// java
// java for(int k = 0; k < j; ++k) {
// java if (color == this.basicColors[k]) {
// java isRepeat = true;
// java break;
// java }
// java }
// java
// java if (!isRepeat) {
// java this.basicColors[j++] = color;
// java }
// java
// java if (j == 40) {
// java break;
// java }
// java }
// java
// java if (j < 40) {
// java for(int i = 8; j < 40; ++j) {
// java this.basicColors[j] = this._palette.getRGBColor(i);
// java }
// java }
// java
// java }
pub fn createBasicColor(&self) {
let mut colors=self.basicColors.borrow_mut();let mut j=0;for i in 8..64{let color=self._palette.getRGBColor(i);if !colors[..j].contains(&color){colors[j]=color;j+=1;}if j==40{break;}}while j<40{colors[j]=self._palette.getRGBColor(8);j+=1;}
}
// java public KmoColor getColor(short index) {
// java if (index == 64) {
// java return KmoColor.AUTOMATIC.getInstance();
// java } else {
// java byte[] b = this._palette.getColor(index);
// java return b != null ? new CustomColor(index, b) : null;
// java }
// java }
pub fn getColor(&self,index:i16)->Option<Rc<dyn Color>> {
if index==64{Some(AUTOMATIC::getInstance())}else{self._palette.getColor(index as i32).map(|b|Rc::new(CustomColor::new(index,b)) as Rc<dyn Color>)}
}
// java public KmoColor getColor(int index) {
// java return this.getColor((short)index);
// java }
pub fn getColor__2(&self,index:i32)->Option<Rc<dyn Color>> {
self.getColor(index as i16)
}
// java public int getRGBColor(short index) {
// java return index == 64 ? 16777215 : this._palette.getRGBColor(index);
// java }
pub fn getRGBColor(&self,index:i16)->i32 {
if index==64{16777215}else{self._palette.getRGBColor(index as i32)}
}
// java public KmoColor findColor(byte red, byte green, byte blue) {
// java byte[] b = this._palette.getColor(8);
// java
// java for(short i = 8; b != null; b = this._palette.getColor(i)) {
// java if (b[0] == red && b[1] == green && b[2] == blue) {
// java return new CustomColor(i, b);
// java }
// java
// java ++i;
// java }
// java
// java return null;
// java }
pub fn findColor(&self,red:i8,green:i8,blue:i8)->Option<Rc<dyn Color>> {
let mut i=8i16;while let Some(b)=self._palette.getColor(i as i32){if b==[red,green,blue]{return Some(Rc::new(CustomColor::new(i,b)));}i=i.wrapping_add(1);}None
}
// java public int findSimilarColorIndex(int red, int green, int blue) {
// java KmoColorSimilar SimilaColor = new KmoColorSimilar(this._palette, red, green, blue);
// java return SimilaColor.DoGetSimilarColor();
// java }
pub fn findSimilarColorIndex(&self,red:i32,green:i32,blue:i32)->i32 {
super::KmoColorSimilar::KmoColorSimilar::new(self._palette.clone(),red,green,blue).DoGetSimilarColor()
}
// java public int findSimilarColorIndex(int rgb) {
// java int red = rgb >> 16 & 255;
// java int green = rgb >> 8 & 255;
// java int blue = rgb & 255;
// java KmoColorSimilar SimilaColor = new KmoColorSimilar(this._palette, red, green, blue);
// java return SimilaColor.DoGetSimilarColor();
// java }
pub fn findSimilarColorIndex__2(&self,rgb:i32)->i32 {
self.findSimilarColorIndex((rgb>>16)&255,(rgb>>8)&255,rgb&255)
}
// java public KmoColor findSimilarColor(byte red, byte green, byte blue) {
// java return this.findSimilarColor(this.unsignedInt(red), this.unsignedInt(green), this.unsignedInt(blue));
// java }
pub fn findSimilarColor(&self,red:i8,green:i8,blue:i8)->Option<Rc<dyn Color>> {
self.findSimilarColor__2(red as u8 as i32,green as u8 as i32,blue as u8 as i32)
}
// java public KmoColor findSimilarColor(int red, int green, int blue) {
// java return this.getColor(this.findSimilarColorIndex(red, green, blue));
// java }
pub fn findSimilarColor__2(&self,red:i32,green:i32,blue:i32)->Option<Rc<dyn Color>> {
self.getColor__2(self.findSimilarColorIndex(red,green,blue))
}
// java public void setColorAtIndex(short index, byte red, byte green, byte blue) {
// java this._palette.setColor(index, red, green, blue);
// java }
pub fn setColorAtIndex(&self,index:i16,red:i8,green:i8,blue:i8) {
self._palette.setColor(index,red,green,blue);
}
// java public KmoColor addColor(byte red, byte green, byte blue) {
// java byte[] b = this._palette.getColor(8);
// java
// java for(short i = 8; i < 64; b = this._palette.getColor(i)) {
// java if (b == null) {
// java this.setColorAtIndex(i, red, green, blue);
// java return this.getColor(i);
// java }
// java
// java ++i;
// java }
// java
// java throw new RuntimeException("Could not find free color index");
// java }
pub fn addColor(&self,red:i8,green:i8,blue:i8)->Rc<dyn Color> {
for i in 8..64{if self._palette.getColor(i).is_none(){self.setColorAtIndex(i as i16,red,green,blue);return self.getColor(i as i16).unwrap();}}panic!("Could not find free color index")
}
// java public boolean isPaletteMod() {
// java return this.isPaletteMod;
// java }
pub fn isPaletteMod(&self)->bool {
self.isPaletteMod.get()
}
// java public void setIsPaletteMod(boolean isMod) {
// java this.isPaletteMod = isMod;
// java }
pub fn setIsPaletteMod(&self,isMod:bool) {
self.isPaletteMod.set(isMod);
}
// java public int[] getBasicColors() {
// java return this.basicColors;
// java }
pub fn getBasicColors(&self)->Rc<RefCell<[i32;40]>> {
self.basicColors.clone()
}
// java public void serialize(LittleEndianOutput out) {
// java this._palette.serialize(out);
// java }
pub fn serialize(&self,out:&mut dyn LittleEndianOutput) {
self._palette.serialize(out);
}
// java public PaletteRecord getPaletteRecord() {
// java return this._palette;
// java }
pub fn getPaletteRecord(&self)->Rc<PaletteRecord> {
self._palette.clone()
}
// java public int getFormatColor(int color) {
// java if (ColorToolKit.isPaletteIndex(color)) {
// java return this.getRGBColor((short)color);
// java } else {
// java return KmoColor.isAutomaticColor(color) ? -16777216 : color;
// java }
// java }
pub fn getFormatColor(&self,color:i32)->i32 {
if ((color>>24)&255)==0{self.getRGBColor(color as i16)}else if KmoColor::isAutomaticColor(color){-16777216}else{color}
}
}
// java private static final class CustomColor extends KmoColor
pub struct CustomColor {_byteOffset:i16,_red:i8,_green:i8,_blue:i8}
impl CustomColor {pub fn new(index:i16,b:[i8;3])->Self{Self{_byteOffset:index,_red:b[0],_green:b[1],_blue:b[2]}}
// java private String getGnumericPart(byte color) {
// java String s;
// java if (color == 0) {
// java s = "0";
// java } else {
// java int c = color & 255;
// java c = c << 8 | c;
// java
// java for(s = Integer.toHexString(c).toUpperCase(); s.length() < 4; s = "0" + s) {
// java }
// java }
// java
// java return s;
// java }
pub fn getGnumericPart(color:i8)->String {
if color==0{"0".into()}else{let c=color as u8 as i32;format!("{:04X}",c<<8|c)}
}
}
impl Color for CustomColor {
fn getIndex(&self)->i16{self._byteOffset}
fn getTriplet(&self)->Triplet{Rc::new(RefCell::new([self._red as u8 as i16,self._green as u8 as i16,self._blue as u8 as i16]))}
fn getHexString(&self)->String{format!("{}:{}:{}",Self::getGnumericPart(self._red),Self::getGnumericPart(self._green),Self::getGnumericPart(self._blue))}
}
