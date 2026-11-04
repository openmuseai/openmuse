use crate::java_runtime::Cell;
// java public class Window2
pub struct Window2{pub(crate) grbit:Cell<i16>,pub(crate) topRow:Cell<i32>,pub(crate) leftCol:Cell<i32>,pub(crate) icvHdr:Cell<i32>,pub(crate) scaleSLV:Cell<i16>,pub(crate) scaleNormal:Cell<i16>}
impl Window2{
// java public Window2() {
// java this.grbit = 182;
// java this.topRow = 0;
// java this.leftCol = 0;
// java this.icvHdr = 64;
// java this.scaleSLV = 100;
// java this.scaleNormal = 0;
// java }
pub fn Window2()->Self{Self{grbit:Cell::new(182),topRow:Cell::new(0),leftCol:Cell::new(0),icvHdr:Cell::new(64),scaleSLV:Cell::new(100),scaleNormal:Cell::new(0)}}
// java public boolean isFrozen() {
// java return frozen.isSet(this.grbit);
// java }
pub fn isFrozen(&self)->bool{self.grbit.get()&8!=0}
// java public void setFrozen(boolean f) {
// java this.grbit = frozen.setShortBoolean(this.grbit, f);
// java }
pub fn setFrozen(&self,v:bool){self.grbit.set((self.grbit.get()&!8)|if v{8}else{0});}
// java public boolean isFrozenNoSplit() {
// java return frozenNoSplit.isSet(this.grbit);
// java }
pub fn isFrozenNoSplit(&self)->bool{self.grbit.get()&256!=0}
// java public void setFrozenNoSplit(boolean f) {
// java this.grbit = frozenNoSplit.setShortBoolean(this.grbit, f);
// java }
pub fn setFrozenNoSplit(&self,v:bool){self.grbit.set((self.grbit.get()&!256)|if v{256}else{0});}
// java public boolean isDisplayGrid() {
// java return dspGrid.isSet(this.grbit);
// java }
pub fn isDisplayGrid(&self)->bool{self.grbit.get()&2!=0}
// java public void setDisplayGrid(boolean d) {
// java this.grbit = dspGrid.setShortBoolean(this.grbit, d);
// java }
pub fn setDisplayGrid(&self,v:bool){self.grbit.set((self.grbit.get()&!2)|if v{2}else{0});}
// java public boolean isDisplayRwCol() {
// java return dspRwCol.isSet(this.grbit);
// java }
pub fn isDisplayRwCol(&self)->bool{self.grbit.get()&4!=0}
// java public void setDisplayRwCol(boolean d) {
// java this.grbit = dspRwCol.setShortBoolean(this.grbit, d);
// java }
pub fn setDisplayRwCol(&self,v:bool){self.grbit.set((self.grbit.get()&!4)|if v{4}else{0});}
// java public boolean isDefaultHdr() {
// java return defaultHdr.isAllSet(this.grbit);
// java }
pub fn isDefaultHdr(&self)->bool{self.grbit.get()&32!=0}
// java public void setDefaultHdr(boolean d) {
// java this.grbit = defaultHdr.setShortBoolean(this.grbit, d);
// java }
pub fn setDefaultHdr(&self,v:bool){self.grbit.set((self.grbit.get()&!32)|if v{32}else{0});}
// java public boolean isSelected() {
// java return selected.isSet(this.grbit);
// java }
pub fn isSelected(&self)->bool{self.grbit.get()&512!=0}
// java public void setSelected(boolean d) {
// java this.grbit = selected.setShortBoolean(this.grbit, d);
// java this.grbit = paged.setShortBoolean(this.grbit, d);
// java }
pub fn setSelected(&self,v:bool){self.grbit.set((self.grbit.get()&!512)|if v{512}else{0});}
// java public boolean isShowZeros() {
// java return dspZeros.isSet(this.grbit);
// java }
pub fn isShowZeros(&self)->bool{self.grbit.get()&16!=0}
// java public void setIsShowZeros(boolean d) {
// java this.grbit = dspZeros.setShortBoolean(this.grbit, d);
// java }
pub fn setIsShowZeros(&self,v:bool){self.grbit.set((self.grbit.get()&!16)|if v{16}else{0});}
// java public boolean getIsRTL() {
// java return rightToLeft.isSet(this.grbit);
// java }
pub fn getIsRTL(&self)->bool{self.grbit.get()&64!=0}
// java public void setIsRTL(boolean isRightToLeft) {
// java this.grbit = rightToLeft.setShortBoolean(this.grbit, isRightToLeft);
// java }
pub fn setIsRTL(&self,v:bool){self.grbit.set((self.grbit.get()&!64)|if v{64}else{0});}
// java public boolean isDisplayGuts() {
// java return dspGuts.isSet(this.grbit);
// java }
pub fn isDisplayGuts(&self)->bool{self.grbit.get()&128!=0}
// java public void setIsDisplayGuts(boolean d) {
// java this.grbit = dspGuts.setShortBoolean(this.grbit, d);
// java }
pub fn setIsDisplayGuts(&self,v:bool){self.grbit.set((self.grbit.get()&!128)|if v{128}else{0});}
// java public int getTopRow() {
// java return this.topRow;
// java }
pub fn getTopRow(&self)->i32{self.topRow.get()}
// java public void setTopRow(int r) {
// java this.topRow = r;
// java }
pub fn setTopRow(&self,v:i32){self.topRow.set(v);}
// java public int getLeftCol() {
// java return this.leftCol;
// java }
pub fn getLeftCol(&self)->i32{self.leftCol.get()}
// java public void setLeftCol(int c) {
// java this.leftCol = c;
// java }
pub fn setLeftCol(&self,v:i32){self.leftCol.set(v);}
// java public int getHeaderColor() {
// java return this.icvHdr;
// java }
pub fn getHeaderColor(&self)->i32{self.icvHdr.get()}
// java public void setHeaderColor(int col) {
// java this.icvHdr = col;
// java }
pub fn setHeaderColor(&self,v:i32){self.icvHdr.set(v);}
}
