use std::sync::{Arc,Mutex};
use crate::{kfc::xpxml::{reader::{XmlName2Id::XmlName2Id,XmlRoAttrList::XmlRoAttrList},util::XmlValueBuilder::XmlValueBuilder},native_runtime::DecimalInteger};
// Native java.nio.charset.Charset transport; the original parser chooses when
// bytes are decoded. The host supplies any non-default charset implementation.
pub trait Charset:Send+Sync{fn decode(&self,bytes:&[u8])->String;}
pub struct Utf8;
impl Charset for Utf8{fn decode(&self,bytes:&[u8])->String{String::from_utf8_lossy(bytes).into_owned()}}
// java public final class HLElementByteParser {
pub struct HLElementByteParser{
    // java private XmlName2Id xmlIds;
    xmlIds:Option<Arc<dyn XmlName2Id+Send+Sync>>,
    // java private XmlRoAttrList mRoAttrs;
    mRoAttrs:Option<Arc<Mutex<XmlRoAttrList>>>,
    // java private int currentLabel;
    currentLabel:i32,
    // java private Charset mCharset;
    mCharset:Option<Arc<dyn Charset>>,
    // java private int labelPrefix;
    labelPrefix:i32,
}
impl HLElementByteParser{
    // java public HLElementByteParser(XmlName2Id xmlIds){this(xmlIds,Charset.defaultCharset());}
    pub fn HLElementByteParser(xmlIds:Option<Arc<dyn XmlName2Id+Send+Sync>>)->Self{Self::HLElementByteParser__charset(xmlIds,Some(Arc::new(Utf8)))}
    // java public HLElementByteParser(XmlName2Id xmlIds,Charset charset){super();this.labelPrefix=-1;IOAssert.assertNotNull("charset should not be null!",charset);this.xmlIds=xmlIds;this.currentLabel=-1;this.mCharset=charset;}
    pub fn HLElementByteParser__charset(xmlIds:Option<Arc<dyn XmlName2Id+Send+Sync>>,charset:Option<Arc<dyn Charset>>)->Self{Self{xmlIds,mRoAttrs:None,currentLabel:-1,mCharset:charset,labelPrefix:-1}}
    // java public void setRoAttr(XmlRoAttrList roAttrs){this.mRoAttrs=roAttrs;}
    pub fn setRoAttr(&mut self,roAttrs:Option<Arc<Mutex<XmlRoAttrList>>>){self.mRoAttrs=roAttrs;}
    // java public int getLabel(){return this.currentLabel;}
    pub fn getLabel(&self)->i32{self.currentLabel}
    // java public void parseElement(byte[] chars,int start,int end,int type){
    pub fn parseElement(&mut self,chars:&[u8],mut start:i32,mut end:i32,r#type:i32){
        // java ++start;if(type==4){++start;}--end;if(type==2||type==3){--end;}
        start=start.wrapping_add(1);if r#type==4{start=start.wrapping_add(1);}end=end.wrapping_sub(1);if r#type==2||r#type==3{end=end.wrapping_sub(1);}
        // java if(start>end){throw new IllegalStateException();}else{if(type!=0&&type!=2&&type!=4){if(type==1||type==3){
        if start>end{panic!("IllegalStateException");}else{if r#type!=0&&r#type!=2&&r#type!=4{if r#type==1||r#type==3{
            // java int index=getNextEmptySpace(chars,start,end);this.currentLabel=this.getIdByString(chars,start,index-1,false);if(this.currentLabel==-1){return;}
            let index=Self::getNextEmptySpace(chars,start,end);self.currentLabel=self.getIdByString(chars,start,index.wrapping_sub(1),false);if self.currentLabel == -1{return;}
            // java int space;for(int var9=passEmptySpace(chars,index,end);var9<end;var9=passEmptySpace(chars,space,end)){
            let mut var9=Self::passEmptySpace(chars,index,end);while var9<end{
                // java space=getNextEmptySpaceV2(chars,var9,end);if(space==-1){this.parseAttr(chars,var9,end);break;}this.parseAttr(chars,var9,space-1);
                let space=Self::getNextEmptySpaceV2(chars,var9,end);if space == -1{self.parseAttr(chars,var9,end);break;}self.parseAttr(chars,var9,space.wrapping_sub(1));var9=Self::passEmptySpace(chars,space,end);
            }
        // java }}else{this.currentLabel=this.getIdByString(chars,start,end,false);}
        }}else{self.currentLabel=self.getIdByString(chars,start,end,false);}}
    }
    // java public void parseAttr(byte[] chars,int start,int end){int equal=getNextChar(chars,start,end,'=');if(equal!=-1){if(chars[equal+1]==39&&chars[end]==39||chars[equal+1]==34&&chars[end]==34){
    pub fn parseAttr(&mut self,chars:&[u8],start:i32,end:i32){let equal=Self::getNextChar(chars,start,end,b'=');if equal != -1{if Self::byte(chars,equal.wrapping_add(1))==39&&Self::byte(chars,end)==39||Self::byte(chars,equal.wrapping_add(1))==34&&Self::byte(chars,end)==34{
        // java int attrId=this.getIdByString(chars,start,equal-1,true);if(attrId==-1){return;}String value=this.buildValue(chars,equal+2,end-1);this.mRoAttrs.addAttr(attrId,value);
        let attrId=self.getIdByString(chars,start,equal.wrapping_sub(1),true);if attrId == -1{return;}let value=self.buildValue(chars,equal.wrapping_add(2),end.wrapping_sub(1));self.mRoAttrs.as_ref().expect("NullPointerException").lock().unwrap().add_attr(attrId,&value);
    }}}
    // java private int getIdByString(byte[] chars,int start,int end,boolean isAttr){
    fn getIdByString(&mut self,chars:&[u8],start:i32,end:i32,isAttr:bool)->i32{
        // java int pos=XmlValueBuilder.index(chars,start,end+1,':');if(isAttr){if(pos==-1){int name=XmlValueBuilder.hashcode(chars,start,end+1);return this.xmlIds.getId(this.labelPrefix,name);}else{int prefix=XmlValueBuilder.hashcode(chars,start,pos);int name=XmlValueBuilder.hashcode(chars,pos+1,end+1);return this.xmlIds.getId(prefix,name);}}
        let pos=XmlValueBuilder::index_bytes__int(Some(chars),start,end.wrapping_add(1),b':');if isAttr{if pos == -1{let name=XmlValueBuilder::hashcode_bytes__int(Some(chars),start,end.wrapping_add(1));self.xmlIds.as_ref().expect("NullPointerException").get_id(self.labelPrefix,name)}else{let prefix=XmlValueBuilder::hashcode_bytes__int(Some(chars),start,pos);let name=XmlValueBuilder::hashcode_bytes__int(Some(chars),pos.wrapping_add(1),end.wrapping_add(1));self.xmlIds.as_ref().expect("NullPointerException").get_id(prefix,name)}
        // java else if(pos==-1){this.labelPrefix=-1;int name=XmlValueBuilder.hashcode(chars,start,end+1);return this.xmlIds.getId(this.labelPrefix,name);}else{this.labelPrefix=XmlValueBuilder.hashcode(chars,start,pos);int name=XmlValueBuilder.hashcode(chars,pos+1,end+1);return this.xmlIds.getId(this.labelPrefix,name);}
        }else if pos == -1{self.labelPrefix=-1;let name=XmlValueBuilder::hashcode_bytes__int(Some(chars),start,end.wrapping_add(1));self.xmlIds.as_ref().expect("NullPointerException").get_id(self.labelPrefix,name)}else{self.labelPrefix=XmlValueBuilder::hashcode_bytes__int(Some(chars),start,pos);let name=XmlValueBuilder::hashcode_bytes__int(Some(chars),pos.wrapping_add(1),end.wrapping_add(1));self.xmlIds.as_ref().expect("NullPointerException").get_id(self.labelPrefix,name)}
    }
    // java private static int getNextEmptySpace(byte[] chars,int start,int end){for(int index=start;index<=end;++index){if(chars[index]==32){return index;}}return -1;}
    fn getNextEmptySpace(chars:&[u8],start:i32,end:i32)->i32{let mut index=start;while index<=end{if Self::byte(chars,index)==32{return index;}index=index.wrapping_add(1);}-1}
    // java private static int getNextEmptySpaceV2(byte[] chars,int start,int end){boolean bInSingleQuote=false;boolean bInDoubleQuote=false;for(int index=start;index<=end;++index){switch(chars[index]){
    fn getNextEmptySpaceV2(chars:&[u8],start:i32,end:i32)->i32{let mut bInSingleQuote=false;let mut bInDoubleQuote=false;let mut index=start;while index<=end{match Self::byte(chars,index){
        // java case 32:if(!bInDoubleQuote&&!bInSingleQuote){return index;}break;case 34:if(!bInSingleQuote){bInDoubleQuote=!bInDoubleQuote;}break;case 39:if(!bInDoubleQuote){bInSingleQuote=!bInSingleQuote;}
        32=>{if !bInDoubleQuote&&!bInSingleQuote{return index;}},34=>{if !bInSingleQuote{bInDoubleQuote=!bInDoubleQuote;}},39=>{if !bInDoubleQuote{bInSingleQuote=!bInSingleQuote;}},_=>{}
    }index=index.wrapping_add(1);}-1}
    // java private static int getNextChar(byte[] chars,int start,int end,char ch){for(int index=start;index<=end;++index){if(chars[index]==ch){return index;}}return -1;}
    fn getNextChar(chars:&[u8],start:i32,end:i32,ch:u8)->i32{let mut index=start;while index<=end{if Self::byte(chars,index)==ch{return index;}index=index.wrapping_add(1);}-1}
    // java private static int passEmptySpace(byte[] chars,int start,int end){for(int index=start;index<=end;++index){byte ch=chars[index];if(ch!=32&&ch!=13&&ch!=10){return index;}}return end+1;}
    fn passEmptySpace(chars:&[u8],start:i32,end:i32)->i32{let mut index=start;while index<=end{let ch=Self::byte(chars,index);if ch!=32&&ch!=13&&ch!=10{return index;}index=index.wrapping_add(1);}end.wrapping_add(1)}
    // java private String buildValue(byte[] chars,int start,int end){if(start>end){return "";}else if(getNextChar(chars,start,end,'&')==-1){return new String(chars,start,end-start+1,this.mCharset);}else{
    fn buildValue(&self,chars:&[u8],start:i32,end:i32)->String{if start>end{String::new()}else if Self::getNextChar(chars,start,end,b'&') == -1{self.decode(chars,start,end.wrapping_sub(start).wrapping_add(1))}else{
        // java byte[] tmp=new byte[end-start+1];int top=0;for(int index=start;index<=end;++index){
        let mut tmp=vec![0u8;end.wrapping_sub(start).wrapping_add(1)as usize];let mut top=0;let mut index=start;while index<=end{
            // java if(chars[index]!=38){tmp[top++]=chars[index];}else if(isAMP(chars,index+1,end)){tmp[top++]=38;index+=4;}
            if Self::byte(chars,index)!=38{tmp[top]=Self::byte(chars,index);top+=1;}else if Self::isAMP(chars,index.wrapping_add(1),end){tmp[top]=38;top+=1;index=index.wrapping_add(4);
            // java else if(isLT(chars,index+1,end)){tmp[top++]=60;index+=3;}else if(isGT(chars,index+1,end)){tmp[top++]=62;index+=3;}
            }else if Self::isLT(chars,index.wrapping_add(1),end){tmp[top]=60;top+=1;index=index.wrapping_add(3);}else if Self::isGT(chars,index.wrapping_add(1),end){tmp[top]=62;top+=1;index=index.wrapping_add(3);
            // java else if(isQUOT(chars,index+1,end)){tmp[top++]=34;index+=5;}else if(isAPOS(chars,index+1,end)){tmp[top++]=39;index+=5;}
            }else if Self::isQUOT(chars,index.wrapping_add(1),end){tmp[top]=34;top+=1;index=index.wrapping_add(5);}else if Self::isAPOS(chars,index.wrapping_add(1),end){tmp[top]=39;top+=1;index=index.wrapping_add(5);
            // java else{int e=getNextChar(chars,index+1,end,';');if(e!=-1){if(chars[index+1]!=35){throw new IllegalStateException(new String(chars,start,end-start+1,this.mCharset));}
            }else{let e=Self::getNextChar(chars,index.wrapping_add(1),end,b';');if e != -1{if Self::byte(chars,index.wrapping_add(1))!=35{panic!("IllegalStateException {}",self.decode(chars,start,end.wrapping_sub(start).wrapping_add(1)));}
                // java if(chars[index+2]==120){String data=new String(chars,index+3,e-(index+3),this.mCharset);byte ch=(byte)(Integer.parseInt(data,16)&255);tmp[top++]=ch;index=e;}
                if Self::byte(chars,index.wrapping_add(2))==120{let data=self.decode(chars,index.wrapping_add(3),e.wrapping_sub(index.wrapping_add(3)));let ch=(DecimalInteger::parseIntRadix(&data.encode_utf16().collect::<Vec<_>>(),16).expect("NumberFormatException")&255)as u8;tmp[top]=ch;top+=1;index=e;
                // java else{String data=new String(chars,index+3,e-(index+3),this.mCharset);byte ch=(byte)(Integer.parseInt(data)&255);tmp[top++]=ch;index=e;}
                }else{let data=self.decode(chars,index.wrapping_add(3),e.wrapping_sub(index.wrapping_add(3)));let ch=(DecimalInteger::parseInt(&data.encode_utf16().collect::<Vec<_>>()).expect("NumberFormatException")&255)as u8;tmp[top]=ch;top+=1;index=e;}
            // java }else{tmp[top++]=chars[index];}}
            }else{tmp[top]=Self::byte(chars,index);top+=1;}}index=index.wrapping_add(1);
        }
        // java return new String(tmp,0,top,this.mCharset);
        self.mCharset.as_ref().expect("NullPointerException").decode(&tmp[..top])
    }}
    // java private static boolean isAMP(byte[] chars,int start,int end){return end-start>=2&&chars[start]==97&&chars[start+1]==109&&chars[start+2]==112&&chars[start+3]==59;}
    fn isAMP(chars:&[u8],start:i32,end:i32)->bool{end.wrapping_sub(start)>=2&&Self::byte(chars,start)==97&&Self::byte(chars,start.wrapping_add(1))==109&&Self::byte(chars,start.wrapping_add(2))==112&&Self::byte(chars,start.wrapping_add(3))==59}
    // java private static boolean isLT(byte[] chars,int start,int end){return end-start>=1&&chars[start]==108&&chars[start+1]==116;}
    fn isLT(chars:&[u8],start:i32,end:i32)->bool{end.wrapping_sub(start)>=1&&Self::byte(chars,start)==108&&Self::byte(chars,start.wrapping_add(1))==116}
    // java private static boolean isGT(byte[] chars,int start,int end){return end-start>=2&&chars[start]==103&&chars[start+1]==116;}
    fn isGT(chars:&[u8],start:i32,end:i32)->bool{end.wrapping_sub(start)>=2&&Self::byte(chars,start)==103&&Self::byte(chars,start.wrapping_add(1))==116}
    // java private static boolean isQUOT(byte[] chars,int start,int end){return end-start>=3&&chars[start]==113&&chars[start+1]==117&&chars[start+2]==111&&chars[start+3]==116;}
    fn isQUOT(chars:&[u8],start:i32,end:i32)->bool{end.wrapping_sub(start)>=3&&Self::byte(chars,start)==113&&Self::byte(chars,start.wrapping_add(1))==117&&Self::byte(chars,start.wrapping_add(2))==111&&Self::byte(chars,start.wrapping_add(3))==116}
    // java private static boolean isAPOS(byte[] chars,int start,int end){return end-start>=3&&chars[start]==97&&chars[start+1]==112&&chars[start+2]==111&&chars[start+3]==115;}
    fn isAPOS(chars:&[u8],start:i32,end:i32)->bool{end.wrapping_sub(start)>=3&&Self::byte(chars,start)==97&&Self::byte(chars,start.wrapping_add(1))==112&&Self::byte(chars,start.wrapping_add(2))==111&&Self::byte(chars,start.wrapping_add(3))==115}
    // Native bounds and Charset view of Java byte[] / String(byte[],off,len,charset).
    fn byte(chars:&[u8],i:i32)->u8{*chars.get(i as usize).expect("ArrayIndexOutOfBoundsException")}
    fn decode(&self,chars:&[u8],off:i32,len:i32)->String{let data=chars.get(off as usize..off.wrapping_add(len)as usize).expect("StringIndexOutOfBoundsException");self.mCharset.as_ref().expect("NullPointerException").decode(data)}
}
