//! Native java.math.BigDecimal(String).toPlainString call adapter for TypeConversion.
//! Coefficient and scale stay decimal; no floating-point conversion is involved.
//! Other BigDecimal operations are outside this adapter.
use kfc_kfc_alpha::native_runtime::DecimalInteger::characterDigit;
struct DecimalText{digits:String,scale:i32,negative:bool}
impl DecimalText {
    // java new BigDecimal(str): optional sign; decimal significand; optional E/e exponent.
    // The original constructor uses compact/inflated coefficient storage. This
    // native representation stores the same unscaled value's decimal digits.
    fn new(text:&str)->Option<Self>{
        let units:Vec<_>=text.encode_utf16().collect();let mut i=0;let mut negative=false;
        if matches!(units.first(),Some(43|45)){negative=units[0]==45;i+=1;}
        let mut digits=String::new();let mut dot=false;let mut scale=0i64;
        while i<units.len(){let c=units[i];if matches!(c,69|101){break;}if c==46{if dot{return None;}dot=true;}else{let d=characterDigit(c);if d<0{return None;}digits.push((b'0'+d as u8)as char);if dot{scale+=1;}}i+=1;}
        if digits.is_empty(){return None;}
        if i<units.len(){i+=1;let mut neg=false;if matches!(units.get(i),Some(43|45)){neg=units[i]==45;i+=1;}if i==units.len(){return None;}let mut exponent=0i64;while i<units.len(){let d=characterDigit(units[i]);if d<0{return None;}exponent=exponent.checked_mul(10)?.checked_add(d as i64)?;i+=1;}if neg{exponent= -exponent;}if exponent<i32::MIN as i64||exponent>i32::MAX as i64{return None;}scale-=exponent;}
        let scale=i32::try_from(scale).ok()?;let significant=digits.trim_start_matches('0');let digits=if significant.is_empty(){negative=false;"0".into()}else{significant.into()};Some(Self{digits,scale,negative})
    }
    // java public String toPlainString() {
    fn toPlainString(&self)->String{
        // java if (scale == 0) { return intCompact != INFLATED ? Long.toString(intCompact) : intVal.toString(); }
        if self.scale==0{return format!("{}{}",if self.negative{"-"}else{""},self.digits);}
        // java if (scale < 0) { if (signum() == 0) { return "0"; } ... }
        if self.scale<0{if self.digits=="0"{return "0".into();}let mut out=format!("{}{}",if self.negative{"-"}else{""},self.digits);out.extend(std::iter::repeat('0').take(-(self.scale as i64)as usize));return out;}
        // java String str = intCompact != INFLATED ? Long.toString(Math.abs(intCompact)) : intVal.abs().toString();
        let intString=&self.digits;
        // java return getValueString(signum(), str, scale);
        self.getValueString(intString)
    }
    // java private String getValueString(int signum, String intString, int scale) {
    fn getValueString(&self,intString:&str)->String{
        // java int insertionPoint = intString.length() - scale;
        let insertionPoint=intString.len()as i64-self.scale as i64;
        // java if (insertionPoint == 0) { return (signum<0 ? "-0." : "0.") + intString; }
        if insertionPoint==0{return format!("{}{}",if self.negative{"-0."}else{"0."},intString);}
        // java else if (insertionPoint > 0) { ... append(sign).append(prefix).append('.').append(suffix); }
        if insertionPoint>0{let p=insertionPoint as usize;format!("{}{}.{}",if self.negative{"-"}else{""},&intString[..p],&intString[p..])}
        // java else { buf.append(signum<0 ? "-0." : "0."); for (int i=0; i<-insertionPoint; i++) { buf.append('0'); } buf.append(intString); }
        else{let mut out=String::from(if self.negative{"-0."}else{"0."});out.extend(std::iter::repeat('0').take((-insertionPoint)as usize));out.push_str(intString);out}
    }
}
pub fn toPlainString(text:&str)->Option<String>{Some(DecimalText::new(text)?.toPlainString())}
