use crate::java_runtime::{Rc,RefCell,Cell};
// java public final class FontFormatting implements Cloneable
pub struct FontFormatting {_rawData:Rc<RefCell<[u8;118]>>,dxfOptions:Cell<i8>}
impl FontFormatting {
// java public FontFormatting() {
// java this(new byte[118]);
// java this.setFontHeight(-1);
// java this.setItalic(false);
// java this.setFontWieghtModified(false);
// java this.setOutline(false);
// java this.setShadow(false);
// java this.setStrikeout(false);
// java this.setEscapementType((short)0);
// java this.setUnderlineType((short)0);
// java this.setFontColorIndex(32767);
// java this.setFontStyleModified(false);
// java this.setFontOutlineModified(false);
// java this.setFontShadowModified(false);
// java this.setFontCancellationModified(false);
// java this.setEscapementTypeModified(false);
// java this.setUnderlineTypeModified(false);
// java this.setShort(0, 0);
// java this.setInt(104, 1);
// java this.setInt(108, 0);
// java this.setInt(112, Integer.MAX_VALUE);
// java this.setShort(116, 1);
// java }
pub fn new()->Self {
let f=Self{_rawData:Rc::new(RefCell::new([0;118])),dxfOptions:Cell::new(0)};f.setFontHeight(-1);f.setItalic(false);f.setFontWieghtModified(false);f.setOutline(false);f.setShadow(false);f.setStrikeout(false);f.setEscapementType(0);f.setUnderlineType(0);f.setFontColorIndex(32767);f.setFontStyleModified(false);f.setFontOutlineModified(false);f.setFontShadowModified(false);f.setFontCancellationModified(false);f.setEscapementTypeModified(false);f.setUnderlineTypeModified(false);f.setShort(0,0);f.setInt(104,1);f.setInt(108,0);f.setInt(112,i32::MAX);f.setShort(116,1);f
}
// java private short getShort(int offset) {
// java return LittleEndian.getShort(this._rawData, offset);
// java }
pub fn getShort(&self,o:usize)->i16 {
i16::from_le_bytes(self._rawData.borrow()[o..o+2].try_into().unwrap())
}
// java private void setShort(int offset, int value) {
// java LittleEndian.putShort(this._rawData, offset, (short)value);
// java }
pub fn setShort(&self,o:usize,v:i16) {
self._rawData.borrow_mut()[o..o+2].copy_from_slice(&v.to_le_bytes());
}
// java private int getInt(int offset) {
// java return LittleEndian.getInt(this._rawData, offset);
// java }
pub fn getInt(&self,o:usize)->i32 {
i32::from_le_bytes(self._rawData.borrow()[o..o+4].try_into().unwrap())
}
// java private void setInt(int offset, int value) {
// java LittleEndian.putInt(this._rawData, offset, value);
// java }
pub fn setInt(&self,o:usize,v:i32) {
self._rawData.borrow_mut()[o..o+4].copy_from_slice(&v.to_le_bytes());
}
// java public byte[] getRawRecord() {
// java return this._rawData;
// java }
pub fn getRawRecord(&self)->Rc<RefCell<[u8;118]>> {
self._rawData.clone()
}
// java public int getFontHeight() {
// java return this.getInt(64);
// java }
pub fn getFontHeight(&self)->i32 {
self.getInt(64)
}
// java public void setFontHeight(int height) {
// java this.setInt(64, height);
// java }
pub fn setFontHeight(&self,v:i32) {
self.setInt(64,v);
}
// java public short getEscapementType() {
// java return this.getShort(74);
// java }
pub fn getEscapementType(&self)->i16 {
self.getShort(74)
}
// java public void setEscapementType(short escapementType) {
// java this.setShort(74, escapementType);
// java }
pub fn setEscapementType(&self,v:i16) {
self.setShort(74,v);
}
// java public short getUnderlineType() {
// java return this.getShort(76);
// java }
pub fn getUnderlineType(&self)->i16 {
self.getShort(76)
}
// java public void setUnderlineType(short underlineType) {
// java this.setShort(76, underlineType);
// java }
pub fn setUnderlineType(&self,v:i16) {
self.setShort(76,v);
}
// java public int getFontColorIndex() {
// java return this.getInt(80);
// java }
pub fn getFontColorIndex(&self)->i32 {
self.getInt(80)
}
// java public void setFontColorIndex(int fci) {
// java this.setInt(80, fci);
// java }
pub fn setFontColorIndex(&self,v:i32) {
self.setInt(80,v);
}
fn bit(&self,o:usize,m:i32,v:bool){let old=self.getInt(o);self.setInt(o,if v{old|m}else{old&!m});}
// java public boolean isItalic() {
// java return this.getFontOption(posture);
// java }
pub fn isItalic(&self)->bool {
self.getInt(68)&2!=0
}
// java public void setItalic(boolean italic) {
// java this.setFontOption(italic, posture);
// java }
pub fn setItalic(&self,v:bool) {
self.bit(68,2,v);
}
// java public boolean isOutlineOn() {
// java return this.getFontOption(outline);
// java }
pub fn isOutlineOn(&self)->bool {
self.getInt(68)&8!=0
}
// java public void setOutline(boolean on) {
// java this.setFontOption(on, outline);
// java }
pub fn setOutline(&self,v:bool) {
self.bit(68,8,v);
}
// java public boolean isShadowOn() {
// java return this.getFontOption(shadow);
// java }
pub fn isShadowOn(&self)->bool {
self.getInt(68)&16!=0
}
// java public void setShadow(boolean on) {
// java this.setFontOption(on, shadow);
// java }
pub fn setShadow(&self,v:bool) {
self.bit(68,16,v);
}
// java public boolean isStruckout() {
// java return this.getFontOption(cancellation);
// java }
pub fn isStruckout(&self)->bool {
self.getInt(68)&128!=0
}
// java public void setStrikeout(boolean strike) {
// java this.setFontOption(strike, cancellation);
// java }
pub fn setStrikeout(&self,v:bool) {
self.bit(68,128,v);
}
// java public boolean isFontStyleModified() {
// java return this.getOptionFlag(styleModified);
// java }
pub fn isFontStyleModified(&self)->bool {
self.getInt(88)&2==0
}
// java public void setFontStyleModified(boolean modified) {
// java this.setOptionFlag(modified, styleModified);
// java }
pub fn setFontStyleModified(&self,v:bool) {
self.bit(88,2,!v);
}
// java public boolean isFontOutlineModified() {
// java return this.getOptionFlag(outlineModified);
// java }
pub fn isFontOutlineModified(&self)->bool {
self.getInt(88)&8==0
}
// java public void setFontOutlineModified(boolean modified) {
// java this.setOptionFlag(modified, outlineModified);
// java }
pub fn setFontOutlineModified(&self,v:bool) {
self.bit(88,8,!v);
}
// java public boolean isFontShadowModified() {
// java return this.getOptionFlag(shadowModified);
// java }
pub fn isFontShadowModified(&self)->bool {
self.getInt(88)&16==0
}
// java public void setFontShadowModified(boolean modified) {
// java this.setOptionFlag(modified, shadowModified);
// java }
pub fn setFontShadowModified(&self,v:bool) {
self.bit(88,16,!v);
}
// java public boolean isFontCancellationModified() {
// java return this.getOptionFlag(cancellationModified);
// java }
pub fn isFontCancellationModified(&self)->bool {
self.getInt(88)&128==0
}
// java public void setFontCancellationModified(boolean modified) {
// java this.setOptionFlag(modified, cancellationModified);
// java }
pub fn setFontCancellationModified(&self,v:bool) {
self.bit(88,128,!v);
}
// java public boolean isEscapementTypeModified() {
// java int escapementModified = this.getInt(92);
// java return escapementModified == 0;
// java }
pub fn isEscapementTypeModified(&self)->bool {
self.getInt(92)==0
}
// java public void setEscapementTypeModified(boolean modified) {
// java int value = modified ? 0 : 1;
// java this.setInt(92, value);
// java }
pub fn setEscapementTypeModified(&self,v:bool) {
self.setInt(92,if v{0}else{1});
}
// java public boolean isUnderlineTypeModified() {
// java int underlineModified = this.getInt(96);
// java return underlineModified == 0;
// java }
pub fn isUnderlineTypeModified(&self)->bool {
self.getInt(96)==0
}
// java public void setUnderlineTypeModified(boolean modified) {
// java int value = modified ? 0 : 1;
// java this.setInt(96, value);
// java }
pub fn setUnderlineTypeModified(&self,v:bool) {
self.setInt(96,if v{0}else{1});
}
// java public boolean isFontWeightModified() {
// java int fontStyleModified = this.getInt(100);
// java return fontStyleModified == 0;
// java }
pub fn isFontWeightModified(&self)->bool {
self.getInt(100)==0
}
// java public void setFontWieghtModified(boolean modified) {
// java int value = modified ? 0 : 1;
// java this.setInt(100, value);
// java }
pub fn setFontWieghtModified(&self,v:bool) {
self.setInt(100,if v{0}else{1});
}
// java private void setFontWeight(short pbw) {
// java short bw = pbw;
// java if (pbw < 100) {
// java bw = 100;
// java }
// java
// java if (bw > 1000) {
// java bw = 1000;
// java }
// java
// java this.setShort(72, bw);
// java }
pub fn setFontWeight(&self,v:i16) {
self.setShort(72,v.clamp(100,1000));
}
// java public void setBold(boolean bold) {
// java this.setFontWeight((short)(bold ? 700 : 400));
// java }
pub fn setBold(&self,v:bool) {
self.setFontWeight(if v{700}else{400});
}
// java public short getFontWeight() {
// java return this.getShort(72);
// java }
pub fn getFontWeight(&self)->i16 {
self.getShort(72)
}
// java public boolean isBold() {
// java return this.getFontWeight() == 700;
// java }
pub fn isBold(&self)->bool {
self.getFontWeight()==700
}
// java public boolean hasItalic() {
// java return italic_option.isSet(this.dxfOptions);
// java }
pub fn hasItalic(&self)->bool {
self.dxfOptions.get()&2!=0
}
// java public void setItalicOption() {
// java this.dxfOptions = italic_option.setByteBoolean(this.dxfOptions, true);
// java }
pub fn setItalicOption(&self) {
self.dxfOptions.set(self.dxfOptions.get()|2);
}
// java public boolean hasBold() {
// java return bold_option.isSet(this.dxfOptions);
// java }
pub fn hasBold(&self)->bool {
self.dxfOptions.get()&1!=0
}
// java public void setBoldOption() {
// java this.dxfOptions = bold_option.setByteBoolean(this.dxfOptions, true);
// java }
pub fn setBoldOption(&self) {
self.dxfOptions.set(self.dxfOptions.get()|1);
}
// java public boolean hasUnderline() {
// java return underline_option.isSet(this.dxfOptions);
// java }
pub fn hasUnderline(&self)->bool {
self.dxfOptions.get()&4!=0
}
// java public void setUnderlineOption() {
// java this.dxfOptions = underline_option.setByteBoolean(this.dxfOptions, true);
// java }
pub fn setUnderlineOption(&self) {
self.dxfOptions.set(self.dxfOptions.get()|4);
}
// java public boolean hasColor() {
// java return color_option.isSet(this.dxfOptions);
// java }
pub fn hasColor(&self)->bool {
self.dxfOptions.get()&8!=0
}
// java public void setColorOption() {
// java this.dxfOptions = color_option.setByteBoolean(this.dxfOptions, true);
// java }
pub fn setColorOption(&self) {
self.dxfOptions.set(self.dxfOptions.get()|8);
}
// java public boolean hasStrike() {
// java return strike_option.isSet(this.dxfOptions);
// java }
pub fn hasStrike(&self)->bool {
self.dxfOptions.get()&16!=0
}
// java public void setStrikeOption() {
// java this.dxfOptions = strike_option.setByteBoolean(this.dxfOptions, true);
// java }
pub fn setStrikeOption(&self) {
self.dxfOptions.set(self.dxfOptions.get()|16);
}
// java public Object clone() {
// java byte[] rawData = (byte[])this._rawData.clone();
// java FontFormatting fontFormatting = new FontFormatting(rawData);
// java fontFormatting.dxfOptions = this.dxfOptions;
// java return fontFormatting;
// java }
pub fn clone(&self)->Self {
Self{_rawData:Rc::new(RefCell::new(*self._rawData.borrow())),dxfOptions:Cell::new(self.dxfOptions.get())}
}
// java public boolean equals(Object obj) {
// java if (this == obj) {
// java return true;
// java } else if (obj == null) {
// java return false;
// java } else if (this.getClass() != obj.getClass()) {
// java return false;
// java } else {
// java FontFormatting other = (FontFormatting)obj;
// java return Arrays.equals(this._rawData, other._rawData);
// java }
// java }
pub fn equals(&self,other:&Self)->bool {
*self._rawData.borrow()==*other._rawData.borrow()
}
}

impl FontFormatting{// java public int hashCode() {
// java int prime = 31;
// java int result = 1;
// java result = 31 * result + Arrays.hashCode(this._rawData);
// java return result;
// java }
pub fn hashCode(&self)->i32 {
let h=crate::cn::wps::moss::app::condfmt::java_values::fold(self._rawData.borrow().iter().map(|b|*b as i8 as i32));31i32.wrapping_add(h)
}
// java public boolean equals(Object obj) {
// java if (this == obj) {
// java return true;
// java } else if (obj == null) {
// java return false;
// java } else if (this.getClass() != obj.getClass()) {
// java return false;
// java } else {
// java FontFormatting other = (FontFormatting)obj;
// java return Arrays.equals(this._rawData, other._rawData);
// java }
// java }

}
