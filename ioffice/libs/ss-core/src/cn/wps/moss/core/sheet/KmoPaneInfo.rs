use crate::java_runtime::{Rc,RefCell};use kfc::cn::wps::memor4j::{KmoAtom::KmoAtom,AtomSnapshot::SharedAtom,KmoAtomData::KmoAtomDataSub};use super::PaneInfoData::PaneInfoData;
// java public class KmoPaneInfo extends KmoAtom<PaneInfoData>
pub struct KmoPaneInfo{atom:Rc<RefCell<KmoAtom<PaneInfoData>>>,_owner:SharedAtom}
impl KmoPaneInfo{
// java public KmoPaneInfo() {
// java super(new PaneInfoData());
// java }
pub fn new()->Self{Self::fromData(PaneInfoData::new())}
fn fromData(data:PaneInfoData)->Self{let(atom,owner)=crate::cn::wps::moss::app::xf::snapshot::bind(data);Self{atom,_owner:owner}}
// java public int x() {
// java return ((PaneInfoData)this.rawData()).x;
// java }
pub fn x(&self)->i32{self.atom.borrow().rawData().x}
// java public void setX(int x) {
// java this.beforeWrite();
// java ((PaneInfoData)this.rawData()).x = x;
// java }
pub fn setX(&self,v:i32){let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData_mut().x=v as i32;}
// java public int y() {
// java return ((PaneInfoData)this.rawData()).y;
// java }
pub fn y(&self)->i32{self.atom.borrow().rawData().y}
// java public void setY(int y) {
// java this.beforeWrite();
// java ((PaneInfoData)this.rawData()).y = y;
// java }
pub fn setY(&self,v:i32){let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData_mut().y=v as i32;}
// java public int topRow() {
// java return ((PaneInfoData)this.rawData()).topRow;
// java }
pub fn topRow(&self)->i32{self.atom.borrow().rawData().topRow}
// java public void setTopRow(int topRow) {
// java this.beforeWrite();
// java ((PaneInfoData)this.rawData()).topRow = topRow;
// java }
pub fn setTopRow(&self,v:i32){let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData_mut().topRow=v as i32;}
// java public int leftCol() {
// java return ((PaneInfoData)this.rawData()).leftCol;
// java }
pub fn leftCol(&self)->i32{self.atom.borrow().rawData().leftCol}
// java public void setLeftCol(int leftCol) {
// java this.beforeWrite();
// java ((PaneInfoData)this.rawData()).leftCol = leftCol;
// java }
pub fn setLeftCol(&self,v:i32){let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData_mut().leftCol=v as i32;}
// java public int activePane() {
// java return ((PaneInfoData)this.rawData()).activePane;
// java }
pub fn activePane(&self)->i32{self.atom.borrow().rawData().activePane}
// java public void setActivePane(int activePane) {
// java this.beforeWrite();
// java ((PaneInfoData)this.rawData()).activePane = activePane;
// java }
pub fn setActivePane(&self,v:i32){let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData_mut().activePane=v as i32;}
}
impl Clone for KmoPaneInfo{fn clone(&self)->Self{Self::fromData(self.atom.borrow().rawData().copy())}}
impl Default for KmoPaneInfo{fn default()->Self{Self::new()}}
