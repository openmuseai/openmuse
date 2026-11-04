pub struct XmlStringUtils;impl XmlStringUtils{// java public static String xml2JavaString(String originStr) {
// java if (originStr == null) {
// java return originStr;
// java } else {
// java Pattern pattern = Pattern.compile("_x([0-9a-fA-F]{4})_|&#x([0-9a-fA-F]{4});|(&quot;)|(&apos;)|(&amp;)|(&gt;)|(&lt;)");
// java Matcher matcher = pattern.matcher(originStr);
// java if (!matcher.find()) {
// java return originStr;
// java } else {
// java StringBuffer sb = new StringBuffer();
// java
// java do {
// java String group = matcher.group(1);
// java int groupIndex = 1;
// java
// java for(int i = 2; group == null && i < XML_ESCAPE_REPLACEMENT.length; groupIndex = i++) {
// java group = matcher.group(i);
// java }
// java
// java if (groupIndex != 1 && groupIndex != 2) {
// java matcher.appendReplacement(sb, XML_ESCAPE_REPLACEMENT[groupIndex]);
// java } else {
// java matcher.appendReplacement(sb, String.valueOf((char)Integer.parseInt(group, 16)));
// java }
// java } while(matcher.find());
// java
// java matcher.appendTail(sb);
// java return sb.toString();
// java }
// java }
// java }
pub fn xml2JavaString(s:&str)->String {
let p=regex::Regex::new(r"_x([0-9a-fA-F]{4})_|&#x([0-9a-fA-F]{4});|(&quot;)|(&apos;)|(&amp;)|(&gt;)|(&lt;)").unwrap();let mut out=String::new();let mut end=0;for cap in p.captures_iter(s){let m=cap.get(0).unwrap();out.push_str(&s[end..m.start()]);let mut index=1;while cap.get(index).is_none(){index+=1;}let replacement=match index{1|2=>{let unit=u16::from_str_radix(cap.get(index).unwrap().as_str(),16).unwrap();String::from_utf16(&[unit]).expect("UTF16 unpaired surrogate outside String facade")},3=>"\"".into(),4=>"'".into(),5=>"&".into(),6=>">".into(),7=>"<".into(),_=>unreachable!()};let mut chars=replacement.chars();while let Some(c)=chars.next(){if c=='$'{panic!("IllegalArgumentException Illegal group reference");}else if c=='\\'{out.push(chars.next().expect("IllegalArgumentException character to be escaped is missing"));}else{out.push(c);}}end=m.end();}out.push_str(&s[end..]);out
}
// java public static String java2XmlString(String originStr) {
pub fn java2XmlString(originStr:Option<&str>)->Option<String>{
    // java if (originStr == null) { return originStr; }
    let originStr=originStr?;
    // java Pattern pattern = Pattern.compile("([\\x00-\\x1f\\x80-\\x9f])"); Matcher matcher = pattern.matcher(originStr);
    let pattern=regex::Regex::new(r"([\x00-\x1f\x80-\x9f])").unwrap();let mut matcher=pattern.captures_iter(originStr);
    // java if (!matcher.find()) { return originStr; }
    let Some(mut current)=matcher.next()else{return Some(originStr.into());};
    // java StringBuffer sb = new StringBuffer(); do {
    let mut sb=String::new();let mut tail=0;loop{
        // java String group = matcher.group(1); int groupIndex = 1;
        let mut group=current.get(1);let mut groupIndex=1;const JAVA_ESCAPE_REPLACEMENT:[&str;2]=["0","1"];
        // java for(int i=2;group==null&&i<JAVA_ESCAPE_REPLACEMENT.length;groupIndex=i++){group=matcher.group(i);}
        let mut i=2;while group.is_none()&&i<JAVA_ESCAPE_REPLACEMENT.len(){group=current.get(i);groupIndex=i;i+=1;}
        let matched=current.get(0).unwrap();sb.push_str(&originStr[tail..matched.start()]);
        // java if(groupIndex==1){int code=Character.codePointAt(group.toCharArray(),0);matcher.appendReplacement(sb,getXmlFormatHexString(code));}
        if groupIndex==1{let code=group.unwrap().as_str().chars().next().unwrap()as i32;sb.push_str(&Self::getXmlFormatHexString(code));}
        // java else{matcher.appendReplacement(sb,JAVA_ESCAPE_REPLACEMENT[groupIndex]);}
        else{sb.push_str(JAVA_ESCAPE_REPLACEMENT[groupIndex]);}tail=matched.end();
        // java } while(matcher.find());
        let Some(next)=matcher.next()else{break;};current=next;
    }
    // java matcher.appendTail(sb); return sb.toString();
    sb.push_str(&originStr[tail..]);Some(sb)
}
// java private static String getXmlFormatHexString(int code) {
fn getXmlFormatHexString(code:i32)->String{
    // java String codeStr = Integer.toHexString(code); StringBuilder sb = new StringBuilder(); sb.append("_x");
    let codeStr=format!("{:x}",code as u32);let mut sb=String::from("_x");
    // java int zeroSize = 4 - codeStr.length(); for(int i = 0; i < zeroSize; ++i) { sb.append("0"); }
    let zeroSize=4-codeStr.len()as i32;for _ in 0..zeroSize{sb.push('0');}
    // java sb.append(codeStr).append("_"); return sb.toString();
    sb.push_str(&codeStr);sb.push('_');sb
}
// java public static void formatNumFmtCode2Java(NumberFormatFormatting numFmtFormatting) {
pub fn formatNumFmtCode2Java(numFmtFormatting:Option<&mut crate::cn::wps::moss::filefmt::biff8::record::cf::NumberFormatFormatting::NumberFormatFormatting>){
    // java if (numFmtFormatting != null && numFmtFormatting.fmtCode() != null) { numFmtFormatting.setFmtCode(xml2JavaString(numFmtFormatting.fmtCode())); }
    if let Some(n)=numFmtFormatting{if let Some(code)=n.fmtCode(){n.setFmtCode(Some(Self::xml2JavaString(&code)));}}
}
// java public static void formatNumFmtCode2Xml(NumberFormatFormatting numFmtFormatting) {
pub fn formatNumFmtCode2Xml(numFmtFormatting:Option<&mut crate::cn::wps::moss::filefmt::biff8::record::cf::NumberFormatFormatting::NumberFormatFormatting>){
    // java if (numFmtFormatting != null && numFmtFormatting.fmtCode() != null) {
    if let Some(n)=numFmtFormatting{if n.fmtCode().is_some(){
        // java String fmtCode = numFmtFormatting.fmtCode(); INumFormatter formatter = KmoApp.sharedApp().numFormatter();
        let fmtCode=n.fmtCode().unwrap();let app=crate::cn::wps::moss::app::KmoApp::KmoApp::sharedApp();
        // java if (formatter.compile(fmtCode) != null) { numFmtFormatting.setFmtCode(java2XmlString(fmtCode)); }
        if app.numFormatter(|f|f.expect("NullPointerException").compile(&fmtCode).is_some()){n.setFmtCode(Self::java2XmlString(Some(&fmtCode)));}
        // java else { numFmtFormatting.setFmtCode(""); }
        else{n.setFmtCode(Some(String::new()));}
    }}
}
}
