use crate::{java_runtime::{Rc,RefCell},org::apache::poi::ss::{SpreadsheetVersion::SpreadsheetVersion,util::CellReference::CellReference}};
use super::ptg::{Ptg::{Ptg,getEncodedSize,getEncodedSizeWithoutArrayData,serializePtgs},ExpPtg::ExpPtg};
use util::org::apache::poi::util::{LittleEndianInput::LittleEndianInput,LittleEndianOutput::LittleEndianOutput};
pub type ByteArray=Rc<RefCell<Box<[u8]>>>;
// java public class Formula implements Cloneable {
pub struct Formula{
    // java private byte[] byteEncoding;
    byteEncoding:Option<ByteArray>,
    // java private int encodedTokenLen;
    encodedTokenLen:i32,
    // java private SpreadsheetVersion sv;
    sv:Option<SpreadsheetVersion>,
}
impl Formula{
    // Native references for the three original static final objects.
    pub fn EMPTY_TOKENS()->ByteArray{static V:std::sync::OnceLock<ByteArray>=std::sync::OnceLock::new();V.get_or_init(||Rc::new(RefCell::new(Vec::new().into_boxed_slice()))).clone()}
    pub fn EMPTY_PTGS()->super::ptg::Ptg::PtgArray{static V:std::sync::OnceLock<super::ptg::Ptg::PtgArray>=std::sync::OnceLock::new();V.get_or_init(||Rc::new(RefCell::new(Vec::new().into_boxed_slice()))).clone()}
    pub fn EMPTY_FORMULA()->Rc<RefCell<Self>>{static V:std::sync::OnceLock<Rc<RefCell<Formula>>>=std::sync::OnceLock::new();V.get_or_init(||Rc::new(RefCell::new(Self::Formula(Some(Self::EMPTY_TOKENS()),0,Some(SpreadsheetVersion::EXCEL97))))).clone()}
    // java public Formula(byte[] byteEncoding,int encodedTokenLen,SpreadsheetVersion sv){this.byteEncoding=byteEncoding;this.encodedTokenLen=encodedTokenLen;this.sv=sv;}
    pub fn Formula(byteEncoding:Option<ByteArray>,encodedTokenLen:i32,sv:Option<SpreadsheetVersion>)->Self{Self{byteEncoding,encodedTokenLen,sv}}
    // java public static Formula read(int encodedTokenLen,LittleEndianInput in,SpreadsheetVersion sv){return read(encodedTokenLen,in,encodedTokenLen,sv);}
    pub fn read(encodedTokenLen:i32,input:&mut dyn LittleEndianInput,sv:Option<SpreadsheetVersion>)->Self{Self::read__4(encodedTokenLen,input,encodedTokenLen,sv)}
    // java public SpreadsheetVersion getVersion(){return this.sv;}
    pub fn getVersion(&self)->Option<SpreadsheetVersion>{self.sv}
    // java public void setVersion(SpreadsheetVersion sv){this.sv=sv;}
    pub fn setVersion(&mut self,sv:Option<SpreadsheetVersion>){self.sv=sv;}
    // java public static Formula read(int encodedTokenLen,LittleEndianInput in,int totalEncodedLen,SpreadsheetVersion sv){byte[] byteEncoding=new byte[totalEncodedLen];in.readFully(byteEncoding);return new Formula(byteEncoding,encodedTokenLen,sv);}
    pub fn read__4(encodedTokenLen:i32,input:&mut dyn LittleEndianInput,totalEncodedLen:i32,sv:Option<SpreadsheetVersion>)->Self{assert!(totalEncodedLen>=0,"NegativeArraySizeException");let mut byteEncoding=vec![0u8;totalEncodedLen as usize];input.readFully(&mut byteEncoding);Self::Formula(Some(Rc::new(RefCell::new(byteEncoding.into_boxed_slice()))),encodedTokenLen,sv)}
    // java public Ptg[] getTokens(boolean externalName){LittleEndianInput in=new LittleEndianByteArrayInputStream(this.byteEncoding);return Ptg.readTokens(this.encodedTokenLen,in,externalName,this.sv);}
    pub fn getTokens__external(&self,externalName:bool)->Vec<Ptg>{let bytes=self.byteEncoding.as_ref().expect("NullPointerException").borrow();Ptg::readTokens__stream(self.encodedTokenLen,&bytes,externalName,self.sv.unwrap_or(SpreadsheetVersion::EXCEL2007))}
    // java public Ptg[] getTokens(){return this.getTokens(false);}
    pub fn getTokens(&self)->Vec<Ptg>{self.getTokens__external(false)}
    // Native mutable-array references for these same two original getters.
    // java public Ptg[] getTokens(boolean externalName){LittleEndianInput in=new LittleEndianByteArrayInputStream(this.byteEncoding);return Ptg.readTokens(this.encodedTokenLen,in,externalName,this.sv);}
    pub fn getTokens__array(&self,externalName:bool)->super::ptg::Ptg::PtgArray{let bytes=self.byteEncoding.as_ref().expect("NullPointerException").borrow();Ptg::readTokens__stream__array(self.encodedTokenLen,&bytes,externalName,self.sv.unwrap_or(SpreadsheetVersion::EXCEL2007))}
    // java public Ptg[] getTokens(){return this.getTokens(false);}
    pub fn getTokens__reference(&self)->super::ptg::Ptg::PtgArray{self.getTokens__array(false)}
    // java public void serialize(LittleEndianOutput out){out.writeShort(this.encodedTokenLen);out.write(this.byteEncoding);}
    pub fn serialize(&self,out:&mut dyn LittleEndianOutput){out.writeShort(self.encodedTokenLen);out.write(&self.byteEncoding.as_ref().expect("NullPointerException").borrow());}
    // java public void serializeTokens(LittleEndianOutput out){out.write(this.byteEncoding,0,this.encodedTokenLen);}
    pub fn serializeTokens(&self,out:&mut dyn LittleEndianOutput){out.write_with_offset(&self.byteEncoding.as_ref().expect("NullPointerException").borrow(),0,self.encodedTokenLen);}
    // java public void serializeArrayConstantData(LittleEndianOutput out){int len=this.byteEncoding.length-this.encodedTokenLen;out.write(this.byteEncoding,this.encodedTokenLen,len);}
    pub fn serializeArrayConstantData(&self,out:&mut dyn LittleEndianOutput){let bytes=self.byteEncoding.as_ref().expect("NullPointerException").borrow();let len=(bytes.len()as i32).wrapping_sub(self.encodedTokenLen);out.write_with_offset(&bytes,self.encodedTokenLen,len);}
    // java public int getEncodedSize(){return 2+this.byteEncoding.length;}
    pub fn getEncodedSize(&self)->i32{2i32.wrapping_add(self.byteEncoding.as_ref().expect("NullPointerException").borrow().len()as i32)}
    // java public int getEncodedTokenSize(){return this.encodedTokenLen;}
    pub fn getEncodedTokenSize(&self)->i32{self.encodedTokenLen}
    // Native non-null argument view retained for existing parser call sites.
    pub fn create(ptgs:&[Ptg],sv:SpreadsheetVersion)->Self{Self::create__nullable(Some(ptgs),Some(sv))}
    // java public static Formula create(Ptg[] ptgs,SpreadsheetVersion sv){
    pub fn create__nullable(ptgs:Option<&[Ptg]>,sv:Option<SpreadsheetVersion>)->Self{
        // java if(ptgs!=null&&ptgs.length>=1){int totalSize=Ptg.getEncodedSize(ptgs);byte[] encodedData=new byte[totalSize];Ptg.serializePtgs(ptgs,encodedData,0);int encodedTokenLen=Ptg.getEncodedSizeWithoutArrayData(ptgs);return new Formula(encodedData,encodedTokenLen,sv==null?SpreadsheetVersion.EXCEL97:sv);}
        if let Some(ptgs)=ptgs.filter(|p|!p.is_empty()){let totalSize=getEncodedSize(ptgs);assert!(totalSize>=0,"NegativeArraySizeException");let encodedData=serializePtgs(ptgs).into_boxed_slice();let encodedTokenLen=getEncodedSizeWithoutArrayData(ptgs);Self::Formula(Some(Rc::new(RefCell::new(encodedData))),encodedTokenLen,Some(sv.unwrap_or(SpreadsheetVersion::EXCEL97)))
        // java else{return new Formula(EMPTY_TOKENS,0,sv);}
        }else{Self::Formula(Some(Self::EMPTY_TOKENS()),0,sv)}
    }
    // java public static Formula create(Ptg[] ptgs,SpreadsheetVersion sv){
    pub fn create__array(ptgs:Option<&super::ptg::Ptg::PtgArray>,sv:Option<SpreadsheetVersion>)->Self{
        // java if(ptgs!=null&&ptgs.length>=1){
        if let Some(ptgs)=ptgs.filter(|p|!p.borrow().is_empty()){
            let values=super::ptg::Ptg::ptgEncodingValues(ptgs);
            // java int totalSize=Ptg.getEncodedSize(ptgs);byte[] encodedData=new byte[totalSize];
            let totalSize=getEncodedSize(&values);assert!(totalSize>=0,"NegativeArraySizeException");
            // java Ptg.serializePtgs(ptgs,encodedData,0);
            let encodedData=serializePtgs(&values).into_boxed_slice();
            // java int encodedTokenLen=Ptg.getEncodedSizeWithoutArrayData(ptgs);return new Formula(encodedData,encodedTokenLen,sv==null?SpreadsheetVersion.EXCEL97:sv);
            let encodedTokenLen=getEncodedSizeWithoutArrayData(&values);Self::Formula(Some(Rc::new(RefCell::new(encodedData))),encodedTokenLen,Some(sv.unwrap_or(SpreadsheetVersion::EXCEL97)))
        }else{
            // java return new Formula(EMPTY_TOKENS,0,sv);
            Self::Formula(Some(Self::EMPTY_TOKENS()),0,sv)
        }
    }
    // java public static Ptg[] getTokens(Formula formula,boolean externalName){return formula==null?null:formula.getTokens(externalName);}
    pub fn getTokens__formula(formula:Option<&Self>,externalName:bool)->Option<Vec<Ptg>>{formula.map(|f|f.getTokens__external(externalName))}
    // java public CellReference getExpReference(){byte[] data=this.byteEncoding;switch(data[0]){case 1:case 2:int[] rowCol=ExpPtg.getRowCol(this.sv,data);return new CellReference(rowCol[0],rowCol[1]);default:return null;}}
    pub fn getExpReference(&self)->Option<CellReference>{let data=self.byteEncoding.as_ref().expect("NullPointerException").borrow();match *data.first().expect("ArrayIndexOutOfBoundsException"){1|2=>{let rowCol=ExpPtg::getRowCol(self.sv.unwrap_or(SpreadsheetVersion::EXCEL2007),&data);Some(CellReference::CellReference(rowCol[0],rowCol[1]))},_=>None}}
    // java public boolean isSame(Formula other){return Arrays.equals(this.byteEncoding,other.byteEncoding);}
    pub fn isSame(&self,other:Option<&Self>)->bool{let other=other.expect("NullPointerException");match(&self.byteEncoding,&other.byteEncoding){(None,None)=>true,(Some(a),Some(b))=>*a.borrow()==*b.borrow(),_=>false}}
    // java public Formula copy(){byte[] b=new byte[this.byteEncoding.length];System.arraycopy(this.byteEncoding,0,b,0,this.byteEncoding.length);return new Formula(b,this.encodedTokenLen,this.sv);}
    pub fn copy(&self)->Self{let b=self.byteEncoding.as_ref().expect("NullPointerException").borrow().clone();Self::Formula(Some(Rc::new(RefCell::new(b))),self.encodedTokenLen,self.sv)}
    // java public void setValue(byte[] byteEncoding,int encodedTokenLen){this.byteEncoding=byteEncoding;this.encodedTokenLen=encodedTokenLen;}
    pub fn setValue(&mut self,byteEncoding:Option<ByteArray>,encodedTokenLen:i32){self.byteEncoding=byteEncoding;self.encodedTokenLen=encodedTokenLen;}
    // java public byte[] getByteEncoding(){return this.byteEncoding;}
    pub fn getByteEncoding(&self)->Option<ByteArray>{self.byteEncoding.clone()}
    // Original reviseData/FormulaReviser and BIFF5 getREFRanges are separate dependencies.
    // java public int hashCode(){int prime=31;int result=1;result=31*result+Arrays.hashCode(this.byteEncoding);result=31*result+this.encodedTokenLen;return result;}
    pub fn hashCode(&self)->i32{let arrayHash=self.byteEncoding.as_ref().map_or(0,|v|v.borrow().iter().fold(1i32,|h,b|h.wrapping_mul(31).wrapping_add(*b as i8 as i32)));let mut result=1i32;result=31i32.wrapping_mul(result).wrapping_add(arrayHash);result=31i32.wrapping_mul(result).wrapping_add(self.encodedTokenLen);result}
    // java public boolean equals(Object obj){if(obj==null){return false;}else{if(obj instanceof Formula){Formula formula=(Formula)obj;if(this.encodedTokenLen==formula.getEncodedTokenSize()){byte[] formulaByte=formula.getByteEncoding();for(int i=0;i<this.encodedTokenLen;++i){if(this.byteEncoding[i]!=formulaByte[i]){return false;}}return true;}}return false;}}
    pub fn equals(&self,obj:Option<&dyn std::any::Any>)->bool{let Some(obj)=obj else{return false;};if let Some(formula)=obj.downcast_ref::<Self>(){if self.encodedTokenLen==formula.getEncodedTokenSize(){let formulaByte=formula.getByteEncoding();let mut i=0;while i<self.encodedTokenLen{let a=*self.byteEncoding.as_ref().expect("NullPointerException").borrow().get(i as usize).expect("ArrayIndexOutOfBoundsException");let b=*formulaByte.as_ref().expect("NullPointerException").borrow().get(i as usize).expect("ArrayIndexOutOfBoundsException");if a!=b{return false;}i+=1;}return true;}}false}
}
// Native owned-copy operation; Java reference owners use Rc<RefCell<Formula>>.
impl Clone for Formula{fn clone(&self)->Self{self.copy()}}
