use super::ConstToken::TokenChildMap;
use crate::cn::wps::kfc::numfmt::{global::GlobalEnv::GlobalEnv,resource::{ResourceDef::ResourceDef,ResourceLoader::ResourceLoader}};
// java public class VariantToken {
pub struct VariantToken{
    // java public final char mKeyTokenSharp='#', mKeyTokenZero='0', mKeyTokenAsk='?', mKeyTokenAskFull='？';
    pub mKeyTokenSharp:char,pub mKeyTokenZero:char,pub mKeyTokenAsk:char,pub mKeyTokenAskFull:char,
    // java public final char mKeyTokenPercent='%',mKeyTokenFraction='/',mKeyTokenEscape='\\',mKeyTokenQuatation='"',mKeyTokenFill='*',mKeyTokenUndlerline='_',mKeyTokenText='@',mKeyTokenColon=':';
    pub mKeyTokenPercent:char,pub mKeyTokenFraction:char,pub mKeyTokenEscape:char,pub mKeyTokenQuatation:char,pub mKeyTokenFill:char,pub mKeyTokenUndlerline:char,pub mKeyTokenText:char,pub mKeyTokenColon:char,
    // java public char mKeyTokenDecimal,mKeyTokenThous,mKeyTokenComment,mKeyTokenGeneral;
    pub mKeyTokenDecimal:char,pub mKeyTokenThous:char,pub mKeyTokenComment:char,pub mKeyTokenGeneral:char,
    // java public String mTokenGeneralStr; public String[] mTokenGeneralChildren=new String[1];
    pub mTokenGeneralStr:Option<String>,pub mTokenGeneralChildren:Vec<String>,
    // java public char mKeyToken_1='1',mKeyToken_2='2',mKeyToken_3='3',mKeyToken_4='4',mKeyToken_5='5',mKeyToken_6='6',mKeyToken_7='7',mKeyToken_8='8',mKeyToken_9='9';
    pub mKeyToken_1:char,pub mKeyToken_2:char,pub mKeyToken_3:char,pub mKeyToken_4:char,pub mKeyToken_5:char,pub mKeyToken_6:char,pub mKeyToken_7:char,pub mKeyToken_8:char,pub mKeyToken_9:char,
    // java public char mKeyTokenYear,mKeyTokenMonth,mKeyTokenDay,mKeyTokenHour,mKeyTokenMin,mKeyTokenSec;
    pub mKeyTokenYear:char,pub mKeyTokenMonth:char,pub mKeyTokenDay:char,pub mKeyTokenHour:char,pub mKeyTokenMin:char,pub mKeyTokenSec:char,
    // java public final char TOKEN_CH_AP=19978;private final String TOKEN_CH_AMPM="上午/下午";public final String[] TOKEN_CH_AP_CHILDREN=new String[]{"上午/下午"};
    pub TOKEN_CH_AP:char,TOKEN_CH_AMPM:String,pub TOKEN_CH_AP_CHILDREN:Vec<String>,
    // java public char mKeyToken_a;private String mToken_a_ap,mToken_a_ampm;public final String[] mToken_a_children=new String[2];
    pub mKeyToken_a:char,mToken_a_ap:Option<String>,mToken_a_ampm:Option<String>,pub mToken_a_children:Vec<String>,
    // java public final char mKeyToken_b='b';public String[] mToken_b_children;public final char mKeyToken_e;public String[] mToken_e_children;public final char mKeyToken_g;public String[] mToken_g_children;
    pub mKeyToken_b:char,pub mToken_b_children:Vec<String>,pub mKeyToken_e:char,pub mToken_e_children:Vec<String>,pub mKeyToken_g:char,pub mToken_g_children:Vec<String>,
    // java public final char mKeyTokenBracket;private String mTokenNCL,mTokenColor,mTokenDBNum,mTokenCondition,mTokenHourElapsed,mTokenMinElapsed,mTokenSecElapse;
    pub mKeyTokenBracket:char,mTokenNCL:Option<String>,mTokenColor:Option<String>,mTokenDBNum:Option<String>,mTokenCondition:Option<String>,mTokenHourElapsed:Option<String>,mTokenMinElapsed:Option<String>,mTokenSecElapse:Option<String>,
    // java public String[] mTokenBracketChildren;public ConstToken.TokenChildMap mTokenChildMap;
    pub mTokenBracketChildren:Vec<String>,pub mTokenChildMap:TokenChildMap,
}
impl VariantToken{
    // java private static final String DEFAULT_NCL="\\$(?:[^-]*)(-(.+))?"; DEFAULT_COLOR="(?i)Black|Blue|Cyan|Green|Magenta|Red|White|Yellow|Color\\s*(\\d{1,2})"; DEFAULT_DBNUM="(?i)DBNum([1234])"; DEFAULT_CONDITION="<>|[<>]=?|="; DEFAULT_h_ELAPSED="(?i)h+"; DEFAULT_m_ELAPSED="(?i)m+"; DEFAULT_s_ELAPSED="(?i)s+";
    // java public static final String[] DEFAULT_BRACKET_CHILDREN = new String[]{DEFAULT_NCL,DEFAULT_COLOR,DEFAULT_DBNUM,DEFAULT_CONDITION,DEFAULT_h_ELAPSED,DEFAULT_m_ELAPSED,DEFAULT_s_ELAPSED};
    pub const DEFAULT_BRACKET_CHILDREN:[&'static str;7]=[r"\$(?:[^-]*)(-(.+))?",r"(?i)Black|Blue|Cyan|Green|Magenta|Red|White|Yellow|Color\s*(\d{1,2})",r"(?i)DBNum([1234])",r"<>|[<>]=?|=",r"(?i)h+",r"(?i)m+",r"(?i)s+"];
    // java public VariantToken() {super();this.mToken_b_children=ConstToken.TOKEN_b_CHILDREN;this.mKeyToken_e='e';this.mToken_e_children=ConstToken.TOKEN_e_CHILDREN;this.mKeyToken_g='g';this.mToken_g_children=new String[1];this.mKeyTokenBracket='[';this.mTokenBracketChildren=new String[7];this.mTokenChildMap=new ConstToken.TokenChildMap();}
    pub fn new()->Self{Self{mKeyTokenSharp:'#',mKeyTokenZero:'0',mKeyTokenAsk:'?',mKeyTokenAskFull:'？',mKeyTokenPercent:'%',mKeyTokenFraction:'/',mKeyTokenEscape:'\\',mKeyTokenQuatation:'"',mKeyTokenFill:'*',mKeyTokenUndlerline:'_',mKeyTokenText:'@',mKeyTokenColon:':',mKeyTokenDecimal:'\0',mKeyTokenThous:'\0',mKeyTokenComment:'\0',mKeyTokenGeneral:'\0',mTokenGeneralStr:None,mTokenGeneralChildren:vec![String::new()],mKeyToken_1:'1',mKeyToken_2:'2',mKeyToken_3:'3',mKeyToken_4:'4',mKeyToken_5:'5',mKeyToken_6:'6',mKeyToken_7:'7',mKeyToken_8:'8',mKeyToken_9:'9',mKeyTokenYear:'\0',mKeyTokenMonth:'\0',mKeyTokenDay:'\0',mKeyTokenHour:'\0',mKeyTokenMin:'\0',mKeyTokenSec:'\0',TOKEN_CH_AP:'上',TOKEN_CH_AMPM:"上午/下午".into(),TOKEN_CH_AP_CHILDREN:vec!["上午/下午".into()],mKeyToken_a:'\0',mToken_a_ap:None,mToken_a_ampm:None,mToken_a_children:vec![String::new();2],mKeyToken_b:'b',mToken_b_children:vec!["B1".into(),"B2".into()],mKeyToken_e:'e',mToken_e_children:vec!["E+".into(),"E-".into()],mKeyToken_g:'g',mToken_g_children:vec![String::new()],mKeyTokenBracket:'[',mTokenNCL:None,mTokenColor:None,mTokenDBNum:None,mTokenCondition:None,mTokenHourElapsed:None,mTokenMinElapsed:None,mTokenSecElapse:None,mTokenBracketChildren:vec![String::new();7],mTokenChildMap:TokenChildMap::new()}}
    // java public void load(short lcid) {
    pub fn load(&mut self,lcid:i16){
        // java String resName = ResourceDef.getTokenResName(lcid); IResourceParser parser = ResourceLoader.getSaxParser(resName);
        let mut resName=ResourceDef::getTokenResName(lcid);let mut parser=ResourceLoader::getSaxParser(&resName);
        // java if (parser == null) { resName = ResourceDef.getTokenResName(GlobalEnv.matchLCID(lcid)); parser = ResourceLoader.getSaxParser(resName); if (parser == null) { resName = ResourceDef.getTokenResName((short)1033); parser = ResourceLoader.getSaxParser(resName); } assert parser != null; }
        if parser.is_none(){resName=ResourceDef::getTokenResName(GlobalEnv::matchLCID(lcid));parser=ResourceLoader::getSaxParser(&resName);if parser.is_none(){resName=ResourceDef::getTokenResName(1033);parser=ResourceLoader::getSaxParser(&resName);}assert!(parser.is_some());}
        // Native nullable SAX adapter: the older nonnullable trait discards missing keys.
        let parser=parser.unwrap();let item=|key:&str|parser.mProperties.get(key).and_then(|v|v.first()).cloned();let array=|key:&str|parser.mProperties.get(key).cloned();let first=|s:&str|char::from_u32(s.encode_utf16().next().expect("StringIndexOutOfBoundsException")as u32).expect("UTF16 char adapter");
        // java String item = parser.getItem("token-decimal"); this.mKeyTokenDecimal = item != null ? item.charAt(0) : 46;
        self.mKeyTokenDecimal=item("token-decimal").map_or('.',|s|first(&s));
        // java item = parser.getItem("token-thous"); this.mKeyTokenThous = item != null ? item.charAt(0) : 44;
        self.mKeyTokenThous=item("token-thous").map_or(',',|s|first(&s));
        // java item = parser.getItem("token-escape-ex"); this.mKeyTokenComment = item != null ? item.charAt(0) : 92;
        self.mKeyTokenComment=item("token-escape-ex").map_or('\\',|s|first(&s));
        // java List<String> generals = parser.getArray("token-general"); if (generals != null) { this.mKeyTokenGeneral = generals.get(0).charAt(0); this.mTokenGeneralStr = generals.get(1); } else { this.mKeyTokenGeneral='g';this.mTokenGeneralStr=ConstToken.TOKEN_g_CHILDREN[0]; }
        let generals=array("token-general");if let Some(generals)=generals{self.mKeyTokenGeneral=first(&generals[0]);self.mTokenGeneralStr=Some(generals[1].clone());}else{self.mKeyTokenGeneral='g';self.mTokenGeneralStr=Some("General".into());}
        // java this.mTokenGeneralChildren[0]=this.mTokenGeneralStr;this.mToken_g_children[0]='g'==this.mKeyTokenGeneral?this.mTokenGeneralStr:"";
        self.mTokenGeneralChildren[0]=self.mTokenGeneralStr.clone().unwrap();self.mToken_g_children[0]=if self.mKeyTokenGeneral=='g'{self.mTokenGeneralStr.clone().unwrap()}else{String::new()};
        // java item=parser.getItem("token-year");this.mKeyTokenYear=item!=null?item.charAt(0):121;
        self.mKeyTokenYear=item("token-year").map_or('y',|s|first(&s));
        // java item=parser.getItem("token-month");this.mKeyTokenMonth=item!=null?item.charAt(0):109;
        self.mKeyTokenMonth=item("token-month").map_or('m',|s|first(&s));
        // java item=parser.getItem("token-day");this.mKeyTokenDay=item!=null?item.charAt(0):100;
        self.mKeyTokenDay=item("token-day").map_or('d',|s|first(&s));
        // java item=parser.getItem("token-hour");this.mKeyTokenHour=item!=null?item.charAt(0):104;
        self.mKeyTokenHour=item("token-hour").map_or('h',|s|first(&s));
        // java item=parser.getItem("token-min");this.mKeyTokenMin=item!=null?item.charAt(0):109;
        self.mKeyTokenMin=item("token-min").map_or('m',|s|first(&s));
        // java item=parser.getItem("token-sec");this.mKeyTokenSec=item!=null?item.charAt(0):115;
        self.mKeyTokenSec=item("token-sec").map_or('s',|s|first(&s));
        // java List<String> a_children=parser.getArray("token-a");if(a_children!=null){this.mKeyToken_a=a_children.get(0).charAt(0);this.mToken_a_ap=a_children.get(1);this.mToken_a_ampm=a_children.get(2);}else{this.mKeyToken_a='a';this.mToken_a_ap=ConstToken.TOKEN_a_CHILDREN[0];this.mToken_a_ampm=ConstToken.TOKEN_a_CHILDREN[1];}
        if let Some(a_children)=array("token-a"){self.mKeyToken_a=first(&a_children[0]);self.mToken_a_ap=Some(a_children[1].clone());self.mToken_a_ampm=Some(a_children[2].clone());}else{self.mKeyToken_a='a';self.mToken_a_ap=Some("a/p".into());self.mToken_a_ampm=Some("am/pm".into());}
        // java this.mToken_a_children[0]=this.mToken_a_ap;this.mToken_a_children[1]=this.mToken_a_ampm;
        self.mToken_a_children[0]=self.mToken_a_ap.clone().unwrap();self.mToken_a_children[1]=self.mToken_a_ampm.clone().unwrap();
        // java List<String> brackets=parser.getArray("token-bracket");
        let brackets=array("token-bracket");
        // java this.mTokenNCL=this.getBracketItem(brackets,0);this.mTokenColor=this.getBracketItem(brackets,1);this.mTokenDBNum=this.getBracketItem(brackets,2);this.mTokenCondition=this.getBracketItem(brackets,3);this.mTokenHourElapsed=this.getBracketItem(brackets,4);this.mTokenMinElapsed=this.getBracketItem(brackets,5);this.mTokenSecElapse=this.getBracketItem(brackets,6);this.reset();
        self.mTokenNCL=Some(self.getBracketItem(brackets.as_deref(),0));self.mTokenColor=Some(self.getBracketItem(brackets.as_deref(),1));self.mTokenDBNum=Some(self.getBracketItem(brackets.as_deref(),2));self.mTokenCondition=Some(self.getBracketItem(brackets.as_deref(),3));self.mTokenHourElapsed=Some(self.getBracketItem(brackets.as_deref(),4));self.mTokenMinElapsed=Some(self.getBracketItem(brackets.as_deref(),5));self.mTokenSecElapse=Some(self.getBracketItem(brackets.as_deref(),6));self.reset();
    }
    // java private String getBracketItem(List<String> brackets,int id){String child=null;if(brackets!=null){String item=brackets.get(id+1);child=item!=null&&item.length()>0?item:DEFAULT_BRACKET_CHILDREN[id];}else{child=DEFAULT_BRACKET_CHILDREN[id];}this.mTokenBracketChildren[id]=child;return child;}
    fn getBracketItem(&mut self,brackets:Option<&[String]>,id:usize)->String{let child=if let Some(brackets)=brackets{let item=&brackets[id+1];if !item.is_empty(){item.clone()}else{Self::DEFAULT_BRACKET_CHILDREN[id].into()}}else{Self::DEFAULT_BRACKET_CHILDREN[id].into()};self.mTokenBracketChildren[id]=child.clone();child}
    // java protected void reset() {
    pub fn reset(&mut self){
        // java this.mTokenChildMap.clear();
        self.mTokenChildMap.clear();
        // java this.mTokenChildMap.mChildtoHolder.put(this.mTokenGeneralStr,(short)1);
        self.mTokenChildMap.mChildtoHolder.insert(self.mTokenGeneralStr.clone().unwrap(),1);
        // java this.mTokenChildMap.mChildtoHolder.put(this.mToken_a_ap,(short)34);this.mTokenChildMap.mChildtoHolder.put(this.mToken_a_ampm,(short)34);
        self.mTokenChildMap.mChildtoHolder.insert(self.mToken_a_ap.clone().unwrap(),34);self.mTokenChildMap.mChildtoHolder.insert(self.mToken_a_ampm.clone().unwrap(),34);
        // java this.mTokenChildMap.mChildtoHolder.put(this.mToken_e_children[0],(short)7);this.mTokenChildMap.mChildtoHolder.put(this.mToken_e_children[1],(short)7);
        self.mTokenChildMap.mChildtoHolder.insert(self.mToken_e_children[0].clone(),7);self.mTokenChildMap.mChildtoHolder.insert(self.mToken_e_children[1].clone(),7);
        // java this.mTokenChildMap.mChildtoHolder.put(this.mToken_b_children[0],(short)22);this.mTokenChildMap.mChildtoHolder.put(this.mToken_b_children[1],(short)22);
        self.mTokenChildMap.mChildtoHolder.insert(self.mToken_b_children[0].clone(),22);self.mTokenChildMap.mChildtoHolder.insert(self.mToken_b_children[1].clone(),22);
        // java this.mTokenChildMap.mChildtoHolder.put(this.mTokenNCL,(short)18);this.mTokenChildMap.mChildtoHolder.put(this.mTokenColor,(short)16);this.mTokenChildMap.mChildtoHolder.put(this.mTokenCondition,(short)17);this.mTokenChildMap.mChildtoHolder.put(this.mTokenDBNum,(short)19);
        self.mTokenChildMap.mChildtoHolder.insert(self.mTokenNCL.clone().unwrap(),18);self.mTokenChildMap.mChildtoHolder.insert(self.mTokenColor.clone().unwrap(),16);self.mTokenChildMap.mChildtoHolder.insert(self.mTokenCondition.clone().unwrap(),17);self.mTokenChildMap.mChildtoHolder.insert(self.mTokenDBNum.clone().unwrap(),19);
        // java this.mTokenChildMap.mChildtoHolder.put(this.mTokenHourElapsed,(short)29);this.mTokenChildMap.mChildtoHolder.put(this.mTokenMinElapsed,(short)30);this.mTokenChildMap.mChildtoHolder.put(this.mTokenSecElapse,(short)31);this.mTokenChildMap.mChildtoHolder.put("上午/下午",(short)34);
        self.mTokenChildMap.mChildtoHolder.insert(self.mTokenHourElapsed.clone().unwrap(),29);self.mTokenChildMap.mChildtoHolder.insert(self.mTokenMinElapsed.clone().unwrap(),30);self.mTokenChildMap.mChildtoHolder.insert(self.mTokenSecElapse.clone().unwrap(),31);self.mTokenChildMap.mChildtoHolder.insert("上午/下午".into(),34);
    }
}
