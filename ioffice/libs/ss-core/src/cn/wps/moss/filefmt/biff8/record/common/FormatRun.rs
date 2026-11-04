use crate::java_runtime::{Rc,RefCell};use kfc::cn::wps::memor4j::{KmoAtom::KmoAtom,AtomSnapshot::SharedAtom,KmoAtomData::KmoAtomDataSub};use super::KmoFormatRun::KmoFormatRun;
// java public class FormatRun extends KmoAtom<KmoFormatRun>
pub struct FormatRun{atom:Rc<RefCell<KmoAtom<KmoFormatRun>>>,_owner:SharedAtom}
impl FormatRun{
// java public FormatRun(short character, short fontIndex) {
// java super(new KmoFormatRun(character, fontIndex));
// java }
pub fn new(_character:i16,_fontIndex:i16)->Self{Self::fromData(KmoFormatRun::new(_character,_fontIndex))}
fn fromData(data:KmoFormatRun)->Self{let(atom,owner)=crate::cn::wps::moss::app::xf::snapshot::bind(data);Self{atom,_owner:owner}}
// java public short getCharacterPos() {
// java return ((KmoFormatRun)this.rawData())._character;
// java }
pub fn getCharacterPos(&self)->i16{self.atom.borrow().rawData()._character}
// java public void setCharPos(short pos) {
// java this.beforeWrite();
// java ((KmoFormatRun)this.rawData())._character = pos;
// java }
pub fn setCharPos(&self,v:i16){let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData_mut()._character=v as i16;}
// java public short getFontIndex() {
// java return ((KmoFormatRun)this.rawData())._fontIndex;
// java }
pub fn getFontIndex(&self)->i16{self.atom.borrow().rawData()._fontIndex}
// java public void setFontIndex(int index) {
// java this.beforeWrite();
// java ((KmoFormatRun)this.rawData())._fontIndex = (short)index;
// java }
pub fn setFontIndex(&self,v:i32){let mut atom=self.atom.borrow_mut();atom.beforeWrite();atom.rawData_mut()._fontIndex=v as i16;}
}
impl Clone for FormatRun{fn clone(&self)->Self{Self::fromData(self.atom.borrow().rawData().copy())}}
impl std::fmt::Debug for FormatRun{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"character={},fontIndex={}",self.getCharacterPos(),self.getFontIndex())}}
impl PartialEq for FormatRun{fn eq(&self,r:&Self)->bool{self.getCharacterPos()==r.getCharacterPos()&&self.getFontIndex()==r.getFontIndex()}}impl Eq for FormatRun{}
