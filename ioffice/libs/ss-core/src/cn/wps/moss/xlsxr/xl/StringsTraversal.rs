use crate::{java_runtime::{Rc,Weak,RefCell,Cell},cn::wps::moss::xlsxr::util::MapManager::MapManager};
use kfc_kfc_alpha::kfc::xpxml::{hlxml::traversal::{HLTByteAccepter::HLTByteAccepter,HLTForwardByte::HLTForwardByte,HLElementByteParser::HLElementByteParser},reader::{XmlName2Id::XmlName2Id,XmlRoAttrList::XmlRoAttrList}};
use super::StringBlock::StringBlock;
use std::{io::{self,Read},fs::File,sync::Mutex};
pub(super) type Blocks=Rc<RefCell<Vec<Rc<StringBlock>>>>;
// java class StringsTraversal implements HLTByteAccepter {
pub(super) struct StringsTraversal{
    // java private String mStringPath;
    mStringPath:Option<String>,
    // java private ArrayList<StringBlock> mBlocks;
    mBlocks:Option<Blocks>,
    // java private StringBlock mCurrentBlock=null;
    mCurrentBlock:RefCell<Option<Rc<StringBlock>>>,
    // java private boolean bInSI=false;private int mCurrentIndex=-1;private int mUniqueCount=0;private int mCount=0;
    bInSI:Cell<bool>,mCurrentIndex:Cell<i32>,mUniqueCount:Cell<i32>,mCount:Cell<i32>,
    // java private HLTForwardByte mForward=null;
    mForward:RefCell<Option<Rc<HLTForwardByte>>>,
    selfRef:Weak<Self>,
}
struct Accepter(Weak<StringsTraversal>);
impl HLTByteAccepter for Accepter{fn element(&self,b:&[u8],s:usize,e:usize){self.0.upgrade().expect("StringsTraversal owner").element(b,s,e);}fn characters(&self,b:&[u8],s:usize,e:usize){self.0.upgrade().expect("StringsTraversal owner").characters(b,s,e);}}
struct Ids;
impl XmlName2Id for Ids{fn get_id(&self,p:i32,n:i32)->i32{MapManager::getInstance().getId(p,n)}}
impl StringsTraversal{
    // java public StringsTraversal(String stringPath,ArrayList<StringBlock> blocks){this.mStringPath=stringPath;this.mBlocks=blocks;}
    pub fn StringsTraversal(stringPath:Option<String>,blocks:Option<Blocks>)->Rc<Self>{Rc::new_cyclic(|s|Self{mStringPath:stringPath,mBlocks:blocks,mCurrentBlock:RefCell::new(None),bInSI:Cell::new(false),mCurrentIndex:Cell::new(-1),mUniqueCount:Cell::new(0),mCount:Cell::new(0),mForward:RefCell::new(None),selfRef:s.clone()})}
    // java public void traversal()throws IOException{
    pub fn traversal(&self)->io::Result<()>{
        // java RandomAccessFile file=new RandomAccessFile(this.mStringPath,"r");int bufferSize=8192;byte[] buffer=new byte[bufferSize];int pos=0;int length=bufferSize;
        let mut file=File::open(self.mStringPath.as_ref().expect("NullPointerException"))?;let bufferSize=8192;let mut buffer=vec![0u8;bufferSize];let mut pos=0i32;let mut length=bufferSize;
        // java try{for(this.mForward=new HLTForwardByte(this);(length=file.read(buffer,0,length))>0;length=bufferSize){this.mForward.parse(buffer,length,pos);pos+=length;}this.mForward=null;}finally{file.close();}
        let result=(||->io::Result<()>{*self.mForward.borrow_mut()=Some(Rc::new(HLTForwardByte::new(Rc::new(Accepter(self.selfRef.clone())))));loop{length=file.read(&mut buffer[..length])?;if length==0{break;}let forward=self.mForward.borrow().clone().unwrap();forward.parse(&buffer,length,pos);pos=pos.wrapping_add(length as i32);length=bufferSize;}*self.mForward.borrow_mut()=None;Ok(())})();drop(file);result?;
        // java if(this.mUniqueCount<=this.mCurrentIndex){this.mUniqueCount=this.mCurrentIndex+1;}
        if self.mUniqueCount.get()<=self.mCurrentIndex.get(){self.mUniqueCount.set(self.mCurrentIndex.get().wrapping_add(1));}Ok(())
    }
    // java public int getUniqueCount(){return this.mUniqueCount;}
    pub fn getUniqueCount(&self)->i32{self.mUniqueCount.get()}
    // java private boolean shouldSkip(int offset){return offset-this.mCurrentBlock.mBeginOffset<this.getBlockMaxSize();}
    fn shouldSkip(&self,offset:i32)->bool{offset.wrapping_sub(self.mCurrentBlock.borrow().as_ref().expect("NullPointerException").mBeginOffset.get())<self.getBlockMaxSize()}
    // java private int getBlockMaxSize(){return this.mBlocks.size()<3?7936:16128;}
    fn getBlockMaxSize(&self)->i32{if self.mBlocks.as_ref().expect("NullPointerException").borrow().len()<3{7936}else{16128}}
    // java public void element(byte[] chars,int start,int end){
    pub fn element(&self,chars:&[u8],start:usize,end:usize){
        // java if(this.bInSI){if(end-start+1==5&&chars[start+1]==47&&chars[start+2]==115&&chars[start+3]==105){this.bInSI=false;int offset=this.mForward.getLastElemPos()+this.mForward.getLastElemLength();if(!this.shouldSkip(offset)){
        if self.bInSI.get(){if end-start+1==5&&chars[start+1]==47&&chars[start+2]==115&&chars[start+3]==105{self.bInSI.set(false);let forward=self.mForward.borrow().clone().expect("NullPointerException");let offset=forward.getLastElemPos().wrapping_add(forward.getLastElemLength());if !self.shouldSkip(offset){
            // java this.mCurrentBlock.mEndIndex=this.mCurrentIndex;this.mCurrentBlock.mEndOffset=offset;this.mBlocks.add(this.mCurrentBlock);this.mCurrentBlock=null;
            let block=self.mCurrentBlock.borrow().clone().expect("NullPointerException");block.mEndIndex.set(self.mCurrentIndex.get());block.mEndOffset.set(offset);self.mBlocks.as_ref().expect("NullPointerException").borrow_mut().push(block);*self.mCurrentBlock.borrow_mut()=None;
        }}
        // java }else if(end-start+1==4&&chars[start+1]==115&&chars[start+2]==105){this.bInSI=true;++this.mCurrentIndex;if(this.mCurrentBlock==null){int offset=this.mForward.getLastElemPos();this.mCurrentBlock=new StringBlock(this.mCurrentIndex,-1,offset,-1);}}
        }else if end-start+1==4&&chars[start+1]==115&&chars[start+2]==105{self.bInSI.set(true);self.mCurrentIndex.set(self.mCurrentIndex.get().wrapping_add(1));if self.mCurrentBlock.borrow().is_none(){let offset=self.mForward.borrow().as_ref().expect("NullPointerException").getLastElemPos();*self.mCurrentBlock.borrow_mut()=Some(Rc::new(StringBlock::StringBlock(self.mCurrentIndex.get(),-1,offset,-1)));}
        // java }else if(end-start+1>5&&chars[start+1]==115&&chars[start+2]==115&&chars[start+3]==116&&chars[start+4]==32){HLElementByteParser parser=new HLElementByteParser(MapManager.getInstance());XmlRoAttrList roAttr=new XmlRoAttrList();parser.setRoAttr(roAttr);parser.parseElement(chars,start,end,1);XmlRoAttr tmp;
        }else if end-start+1>5&&chars[start+1]==115&&chars[start+2]==115&&chars[start+3]==116&&chars[start+4]==32{let mut parser=HLElementByteParser::HLElementByteParser(Some(Rc::new(Ids)));let roAttr=Rc::new(Mutex::new(XmlRoAttrList::new()));parser.setRoAttr(Some(roAttr.clone()));parser.parseElement(chars,start as i32,end as i32,1);
            // java if((tmp=roAttr.getByName(4650))!=null){this.mUniqueCount=tmp.toInt();}if((tmp=roAttr.getByName(4258))!=null){this.mCount=tmp.toInt();}
            let roAttr=roAttr.lock().unwrap();if let Some(tmp)=roAttr.get_by_name(4650){self.mUniqueCount.set(tmp.to_int());}if let Some(tmp)=roAttr.get_by_name(4258){self.mCount.set(tmp.to_int());}
        // java }else if(end-start+1==6&&chars[start+1]==47&&chars[start+2]==115&&chars[start+3]==115&&chars[start+4]==116){int offset=this.mForward.getLastElemPos();if(this.mCurrentBlock!=null){this.mCurrentBlock.mEndIndex=this.mCurrentIndex;this.mCurrentBlock.mEndOffset=offset;this.mBlocks.add(this.mCurrentBlock);this.mCurrentBlock=null;}}
        }else if end-start+1==6&&chars[start+1]==47&&chars[start+2]==115&&chars[start+3]==115&&chars[start+4]==116{let offset=self.mForward.borrow().as_ref().expect("NullPointerException").getLastElemPos();let current=self.mCurrentBlock.borrow().clone();if let Some(block)=current{block.mEndIndex.set(self.mCurrentIndex.get());block.mEndOffset.set(offset);self.mBlocks.as_ref().expect("NullPointerException").borrow_mut().push(block);*self.mCurrentBlock.borrow_mut()=None;}
        // java }else if(end-start+1==5&&chars[start+1]==115&&chars[start+2]==105&&chars[start+3]==47){++this.mCurrentIndex;if(this.mCurrentBlock==null){int offset=this.mForward.getLastElemPos();this.mCurrentBlock=new StringBlock(this.mCurrentIndex,-1,offset,-1);}}
        }else if end-start+1==5&&chars[start+1]==115&&chars[start+2]==105&&chars[start+3]==47{self.mCurrentIndex.set(self.mCurrentIndex.get().wrapping_add(1));if self.mCurrentBlock.borrow().is_none(){let offset=self.mForward.borrow().as_ref().expect("NullPointerException").getLastElemPos();*self.mCurrentBlock.borrow_mut()=Some(Rc::new(StringBlock::StringBlock(self.mCurrentIndex.get(),-1,offset,-1)));}}
    }
    // java public void characters(byte[] chars,int start,int end){}
    pub fn characters(&self,_chars:&[u8],_start:usize,_end:usize){}
    // java public String toString(){StringBuilder builder=new StringBuilder();builder.append("mUniqueCount="+this.mUniqueCount+" mCount="+this.mCount+"\r\n");for(StringBlock block:this.mBlocks){builder.append(block.toString()+"\r\n");}return builder.toString();}
    pub fn toString(&self)->String{let mut builder=String::new();builder.push_str(&format!("mUniqueCount={} mCount={}\r\n",self.mUniqueCount.get(),self.mCount.get()));for block in self.mBlocks.as_ref().expect("NullPointerException").borrow().iter(){builder.push_str(&block.toString());builder.push_str("\r\n");}builder}
}
