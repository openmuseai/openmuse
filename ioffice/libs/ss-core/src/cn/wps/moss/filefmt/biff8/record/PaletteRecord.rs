use crate::java_runtime::{Rc,RefCell};
use util::org::apache::poi::util::LittleEndianOutput::LittleEndianOutput;
// java public final class PaletteRecord extends StandardRecord
pub struct PaletteRecord {_colors:Rc<RefCell<Vec<PColor>>>,_buildInColors:Vec<PColor>}
// java private static final class PColor
#[derive(Clone)]
pub struct PColor {_red:i32,_green:i32,_blue:i32}
impl PColor {
// java public PColor(int red, int green, int blue) {
// java this._red = red;
// java this._green = green;
// java this._blue = blue;
// java }
pub fn new(red:i32,green:i32,blue:i32)->Self {
Self{_red:red,_green:green,_blue:blue}
}
// java public byte[] getTriplet() {
// java return new byte[]{(byte)this._red, (byte)this._green, (byte)this._blue};
// java }
pub fn getTriplet(&self)->[i8;3] {
[self._red as i8,self._green as i8,self._blue as i8]
}
// java public int getRGBColor() {
// java return -16777216 | (this._red & 255) << 16 | (this._green & 255) << 8 | this._blue & 255;
// java }
pub fn getRGBColor(&self)->i32 {
-16777216 | ((self._red&255)<<16) | ((self._green&255)<<8) | (self._blue&255)
}
// java public void serialize(LittleEndianOutput out) {
// java out.writeByte(this._red);
// java out.writeByte(this._green);
// java out.writeByte(this._blue);
// java out.writeByte(0);
// java }
pub fn serialize(&self,out:&mut dyn LittleEndianOutput) {
out.writeByte(self._red);out.writeByte(self._green);out.writeByte(self._blue);out.writeByte(0);
}
}
impl PaletteRecord {
// java public PaletteRecord() {
// java PColor[] defaultPalette = createDefaultPalette();
// java this._colors = new ArrayList(defaultPalette.length);
// java
// java for(int i = 0; i < defaultPalette.length; ++i) {
// java this._colors.add(defaultPalette[i]);
// java }
// java
// java this._buildInColors = createBuildInColors();
// java }
pub fn new()->Self {
Self{_colors:Rc::new(RefCell::new(Self::createDefaultPalette())),_buildInColors:Self::createBuildInColors()}
}
// java public PaletteRecord(List<PColor> _colors) {
// java this._colors = _colors;
// java this._buildInColors = createBuildInColors();
// java }
pub fn withColors(colors:Rc<RefCell<Vec<PColor>>>)->Self {
Self{_colors:colors,_buildInColors:Self::createBuildInColors()}
}
// java private static PColor[] createBuildInColors() {
// java return new PColor[]{pc(0, 0, 0), pc(255, 255, 255), pc(255, 0, 0), pc(0, 255, 0), pc(0, 0, 255), pc(255, 255, 0), pc(255, 0, 255), pc(0, 255, 255)};
// java }
pub fn createBuildInColors()->Vec<PColor> {
vec![PColor::new(0,0,0),PColor::new(255,255,255),PColor::new(255,0,0),PColor::new(0,255,0),PColor::new(0,0,255),PColor::new(255,255,0),PColor::new(255,0,255),PColor::new(0,255,255)]
}
// java private static PColor[] createDefaultPalette() {
// java return new PColor[]{pc(0, 0, 0), pc(255, 255, 255), pc(255, 0, 0), pc(0, 255, 0), pc(0, 0, 255), pc(255, 255, 0), pc(255, 0, 255), pc(0, 255, 255), pc(128, 0, 0), pc(0, 128, 0), pc(0, 0, 128), pc(128, 128, 0), pc(128, 0, 128), pc(0, 128, 128), pc(192, 192, 192), pc(128, 128, 128), pc(153, 153, 255), pc(153, 51, 102), pc(255, 255, 204), pc(204, 255, 255), pc(102, 0, 102), pc(255, 128, 128), pc(0, 102, 204), pc(204, 204, 255), pc(0, 0, 128), pc(255, 0, 255), pc(255, 255, 0), pc(0, 255, 255), pc(128, 0, 128), pc(128, 0, 0), pc(0, 128, 128), pc(0, 0, 255), pc(0, 204, 255), pc(204, 255, 255), pc(204, 255, 204), pc(255, 255, 153), pc(153, 204, 255), pc(255, 153, 204), pc(204, 153, 255), pc(255, 204, 153), pc(51, 102, 255), pc(51, 204, 204), pc(153, 204, 0), pc(255, 204, 0), pc(255, 153, 0), pc(255, 102, 0), pc(102, 102, 153), pc(150, 150, 150), pc(0, 51, 102), pc(51, 153, 102), pc(0, 51, 0), pc(51, 51, 0), pc(153, 51, 0), pc(153, 51, 102), pc(51, 51, 153), pc(51, 51, 51)};
// java }
pub fn createDefaultPalette()->Vec<PColor> {
vec![PColor::new(0,0,0),PColor::new(255,255,255),PColor::new(255,0,0),PColor::new(0,255,0),PColor::new(0,0,255),PColor::new(255,255,0),PColor::new(255,0,255),PColor::new(0,255,255),PColor::new(128,0,0),PColor::new(0,128,0),PColor::new(0,0,128),PColor::new(128,128,0),PColor::new(128,0,128),PColor::new(0,128,128),PColor::new(192,192,192),PColor::new(128,128,128),PColor::new(153,153,255),PColor::new(153,51,102),PColor::new(255,255,204),PColor::new(204,255,255),PColor::new(102,0,102),PColor::new(255,128,128),PColor::new(0,102,204),PColor::new(204,204,255),PColor::new(0,0,128),PColor::new(255,0,255),PColor::new(255,255,0),PColor::new(0,255,255),PColor::new(128,0,128),PColor::new(128,0,0),PColor::new(0,128,128),PColor::new(0,0,255),PColor::new(0,204,255),PColor::new(204,255,255),PColor::new(204,255,204),PColor::new(255,255,153),PColor::new(153,204,255),PColor::new(255,153,204),PColor::new(204,153,255),PColor::new(255,204,153),PColor::new(51,102,255),PColor::new(51,204,204),PColor::new(153,204,0),PColor::new(255,204,0),PColor::new(255,153,0),PColor::new(255,102,0),PColor::new(102,102,153),PColor::new(150,150,150),PColor::new(0,51,102),PColor::new(51,153,102),PColor::new(0,51,0),PColor::new(51,51,0),PColor::new(153,51,0),PColor::new(153,51,102),PColor::new(51,51,153),PColor::new(51,51,51)]
}
// java private static PColor[] createDefaultBIFF5Palette() {
// java return new PColor[]{pc(0, 0, 0), pc(255, 255, 255), pc(255, 0, 0), pc(0, 255, 0), pc(0, 0, 255), pc(255, 255, 0), pc(255, 0, 255), pc(0, 255, 255), pc(128, 0, 0), pc(0, 128, 0), pc(0, 0, 128), pc(128, 128, 0), pc(128, 0, 128), pc(0, 128, 128), pc(192, 192, 192), pc(128, 128, 128), pc(128, 128, 255), pc(128, 32, 96), pc(255, 255, 192), pc(160, 224, 240), pc(96, 0, 128), pc(255, 128, 128), pc(0, 128, 192), pc(192, 192, 255), pc(0, 0, 128), pc(255, 0, 255), pc(255, 255, 0), pc(0, 255, 255), pc(128, 0, 128), pc(128, 0, 0), pc(0, 128, 128), pc(0, 0, 255), pc(0, 207, 255), pc(105, 255, 255), pc(224, 255, 224), pc(255, 255, 128), pc(166, 207, 240), pc(221, 156, 179), pc(179, 143, 238), pc(227, 227, 227), pc(42, 111, 249), pc(63, 184, 205), pc(72, 132, 54), pc(149, 140, 65), pc(142, 94, 66), pc(160, 98, 122), pc(98, 79, 172), pc(150, 150, 150), pc(29, 47, 190), pc(40, 102, 118), pc(0, 69, 0), pc(69, 62, 1), pc(106, 40, 19), pc(133, 57, 106), pc(74, 50, 133), pc(66, 66, 66)};
// java }
pub fn createDefaultBIFF5Palette()->Vec<PColor> {
vec![PColor::new(0,0,0),PColor::new(255,255,255),PColor::new(255,0,0),PColor::new(0,255,0),PColor::new(0,0,255),PColor::new(255,255,0),PColor::new(255,0,255),PColor::new(0,255,255),PColor::new(128,0,0),PColor::new(0,128,0),PColor::new(0,0,128),PColor::new(128,128,0),PColor::new(128,0,128),PColor::new(0,128,128),PColor::new(192,192,192),PColor::new(128,128,128),PColor::new(128,128,255),PColor::new(128,32,96),PColor::new(255,255,192),PColor::new(160,224,240),PColor::new(96,0,128),PColor::new(255,128,128),PColor::new(0,128,192),PColor::new(192,192,255),PColor::new(0,0,128),PColor::new(255,0,255),PColor::new(255,255,0),PColor::new(0,255,255),PColor::new(128,0,128),PColor::new(128,0,0),PColor::new(0,128,128),PColor::new(0,0,255),PColor::new(0,207,255),PColor::new(105,255,255),PColor::new(224,255,224),PColor::new(255,255,128),PColor::new(166,207,240),PColor::new(221,156,179),PColor::new(179,143,238),PColor::new(227,227,227),PColor::new(42,111,249),PColor::new(63,184,205),PColor::new(72,132,54),PColor::new(149,140,65),PColor::new(142,94,66),PColor::new(160,98,122),PColor::new(98,79,172),PColor::new(150,150,150),PColor::new(29,47,190),PColor::new(40,102,118),PColor::new(0,69,0),PColor::new(69,62,1),PColor::new(106,40,19),PColor::new(133,57,106),PColor::new(74,50,133),PColor::new(66,66,66)]
}
// java public byte[] getColor(int byteIndex) {
// java if (byteIndex >= 0 && byteIndex < 8) {
// java return this._buildInColors[byteIndex].getTriplet();
// java } else {
// java int i = byteIndex - 8;
// java return i >= 0 && i < this._colors.size() ? ((PColor)this._colors.get(i)).getTriplet() : null;
// java }
// java }
pub fn getColor(&self,byteIndex:i32)->Option<[i8;3]> {
if (0..8).contains(&byteIndex){Some(self._buildInColors[byteIndex as usize].getTriplet())}else{let i=byteIndex-8;self._colors.borrow().get(usize::try_from(i).ok()?).map(PColor::getTriplet)}
}
// java public int getRGBColor(int byteIndex) {
// java if (byteIndex >= 0 && byteIndex < 8) {
// java return this._buildInColors[byteIndex].getRGBColor();
// java } else {
// java int i = byteIndex - 8;
// java return i >= 0 && i < this._colors.size() ? ((PColor)this._colors.get(i)).getRGBColor() : 16777215;
// java }
// java }
pub fn getRGBColor(&self,byteIndex:i32)->i32 {
if (0..8).contains(&byteIndex){self._buildInColors[byteIndex as usize].getRGBColor()}else{self._colors.borrow().get((byteIndex-8) as usize).map_or(16777215,PColor::getRGBColor)}
}
// java public void setColor(short byteIndex, byte red, byte green, byte blue) {
// java int i = byteIndex - 8;
// java if (i >= 0 && i < 56) {
// java while(this._colors.size() <= i) {
// java this._colors.add(new PColor(0, 0, 0));
// java }
// java
// java PColor custColor = new PColor(red, green, blue);
// java this._colors.set(i, custColor);
// java }
// java }
pub fn setColor(&self,byteIndex:i16,red:i8,green:i8,blue:i8) {
let i=byteIndex as i32-8;if (0..56).contains(&i){let mut colors=self._colors.borrow_mut();while colors.len()<=i as usize{colors.push(PColor::new(0,0,0));}colors[i as usize]=PColor::new(red as i32,green as i32,blue as i32);}
}
// java public void serializeData(LittleEndianOutput out) {
// java out.writeShort(this._colors.size());
// java
// java for(int i = 0; i < this._colors.size(); ++i) {
// java ((PColor)this._colors.get(i)).serialize(out);
// java }
// java
// java }
pub fn serializeData(&self,out:&mut dyn LittleEndianOutput) {
let colors=self._colors.borrow();out.writeShort(colors.len() as i32);for color in colors.iter(){color.serialize(out);}
}
// java protected int getDataSize() {
// java return 2 + this._colors.size() * 4;
// java }
pub fn getDataSize(&self)->i32 {
2+self._colors.borrow().len() as i32*4
}
// java public short getSid() {
// java return 146;
// java }
pub fn getSid(&self)->i16 {
146
}
// rust: StandardRecord.serialize header before the original palette payload.
pub fn serialize(&self,out:&mut dyn LittleEndianOutput){out.writeShort(self.getSid() as i32);out.writeShort(self.getDataSize());self.serializeData(out);}
}
