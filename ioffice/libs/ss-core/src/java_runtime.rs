// Rust ownership adapter shared with the imported Java utility classes.
pub use util::java_runtime::*;
pub mod ConcurrentLongMap;
pub mod DecimalInteger;
pub mod Character;
pub mod Pattern;
pub mod KfsZip;
pub mod RuntimeException;
pub mod FileChannel;

// rust: java.lang.Double.parseDouble host adapter. ET still owns when parsing is
// attempted and which exceptions it catches. None transports NumberFormatException.
// Java accepts ASCII trim, type suffixes and hexadecimal binary exponents; hex
// rounding below is exact at normal/subnormal boundaries and retains signed zero.
pub fn parseDouble(text: &str) -> Option<f64> {
    use num_bigint::BigUint;
    use num_traits::{One, ToPrimitive, Zero};
    static GRAMMAR: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| regex::Regex::new(
        r"\A[+-]?(?:NaN|Infinity|(?:(?:[0-9]+(?:\.[0-9]*)?|\.[0-9]+)(?:[eE][+-]?[0-9]+)?|0[xX](?:[0-9a-fA-F]+\.?|[0-9a-fA-F]*\.[0-9a-fA-F]+)[pP][+-]?[0-9]+)[fFdD]?)\z"
    ).unwrap());
    let text = text.trim_matches(|c: char| c <= '\u{20}');
    if !GRAMMAR.is_match(text) { return None; }
    let negative = text.starts_with('-');
    let unsigned = text.trim_start_matches(['+', '-']);
    if unsigned == "NaN" { return Some(f64::NAN); }
    if unsigned == "Infinity" { return Some(if negative { f64::NEG_INFINITY } else { f64::INFINITY }); }
    let text = text.strip_suffix(['f', 'F', 'd', 'D']).unwrap_or(text);
    let unsigned = text.trim_start_matches(['+', '-']);
    if !unsigned.starts_with("0x") && !unsigned.starts_with("0X") { return text.parse().ok(); }
    let (mantissa, exponent) = unsigned[2..].split_once(['p', 'P']).unwrap();
    let fraction = mantissa.split_once('.').map_or(0, |(_, f)| f.len()) as i64;
    let coefficient = BigUint::parse_bytes(mantissa.replace('.', "").as_bytes(), 16).unwrap();
    let sign = if negative { 1u64 << 63 } else { 0 };
    if coefficient.is_zero() { return Some(f64::from_bits(sign)); }
    let exponent: i64 = exponent.parse().unwrap_or(if exponent.starts_with('-') { i64::MIN } else { i64::MAX });
    let exponent = exponent.saturating_sub(fraction.saturating_mul(4));
    let mut high = (coefficient.bits() as i64 - 1).saturating_add(exponent);
    if high > 1023 { return Some(f64::from_bits(sign | (0x7ffu64 << 52))); }
    if high < -1075 { return Some(f64::from_bits(sign)); }
    let quantum = (high - 52).max(-1074);
    let shift = exponent - quantum;
    let rounded = if shift >= 0 { coefficient << shift as usize } else {
        let shift = (-shift) as usize;
        let mut q = &coefficient >> shift;
        let remainder = coefficient - (&q << shift);
        let half = BigUint::one() << (shift - 1);
        if remainder > half || remainder == half && q.bit(0) { q += BigUint::one(); }
        q
    };
    let mut bits = rounded.to_u64().unwrap();
    if high < -1022 { return Some(f64::from_bits(sign | bits)); }
    if bits == 1u64 << 53 { bits >>= 1; high += 1; }
    if high > 1023 { return Some(f64::from_bits(sign | (0x7ffu64 << 52))); }
    Some(f64::from_bits(sign | ((high + 1023) as u64) << 52 | (bits - (1u64 << 52))))
}

// rust: JDK BigDecimal value adapter used by the unchanged MathX.round15 call
// sequence. Keep the decimal coefficient and scale until Java doubleValue().
#[derive(Clone)]
pub struct BigDecimal {
    coefficient: num_bigint::BigInt,
    scale: i32,
}
#[allow(non_camel_case_types)]
pub enum RoundingMode {
    HALF_UP,
    DOWN,
}
impl BigDecimal {
    // java BigDecimal.valueOf(double): decimal representation of Double.toString.
    pub fn valueOf(value: f64) -> Self {
        assert!(value.is_finite(), "NumberFormatException");
        let mut text = kfc::java_runtime::FloatingDecimal::FloatingDecimal::toJavaFormatString(value);
        let exp_at = text.find(['e', 'E']);
        let mantissa_end = exp_at.unwrap_or(text.len());
        if !text[..mantissa_end].contains('.') {
            text.insert_str(mantissa_end, ".0");
        }
        let (mantissa, exponent) = text
            .split_once(['e', 'E'])
            .map_or((text.as_str(), 0), |(m, e)| (m, e.parse::<i32>().unwrap()));
        let fraction = mantissa.split_once('.').map_or(0, |(_, f)| f.len()) as i32;
        let digits = mantissa.replace('.', "");
        Self {
            coefficient: digits.parse().unwrap(),
            scale: fraction - exponent,
        }
    }
    pub fn scale(&self) -> i32 {
        self.scale
    }
    pub fn precision(&self) -> i32 {
        self.coefficient.to_string().trim_start_matches('-').len() as i32
    }
    // java BigDecimal.setScale(newScale,HALF_UP) keeps all decimal arithmetic exact.
    pub fn setScale(&self, new_scale: i32, _rounding: RoundingMode) -> Self {
        use num_traits::{Signed, Zero};
        let shift = new_scale - self.scale;
        let coefficient = if shift >= 0 {
            &self.coefficient * num_bigint::BigInt::from(10u8).pow(shift as u32)
        } else {
            let divisor = num_bigint::BigInt::from(10u8).pow((-shift) as u32);
            let mut whole = &self.coefficient / &divisor;
            let remainder = (&self.coefficient % &divisor).abs();
            if matches!(_rounding,RoundingMode::HALF_UP) && !remainder.is_zero() && remainder * 2 >= divisor {
                whole += if self.coefficient.is_negative() {
                    -1
                } else {
                    1
                };
            }
            whole
        };
        Self {
            coefficient,
            scale: new_scale,
        }
    }
    pub fn doubleValue(&self) -> f64 {
        format!("{}e{}", self.coefficient, -self.scale)
            .parse()
            .unwrap()
    }
}

// rust: JDK BigDecimal operations used by StateMachine; coefficient/scale remain exact.
impl BigDecimal{
 pub fn valueOfLong(v:i64)->Self{Self{coefficient:v.into(),scale:0}}
 pub fn add(&self,rhs:&Self)->Self{let scale=self.scale.max(rhs.scale);let ten=num_bigint::BigInt::from(10u8);Self{coefficient:&self.coefficient*ten.pow((scale-self.scale)as u32)+&rhs.coefficient*ten.pow((scale-rhs.scale)as u32),scale}}
 pub fn multiply(&self,rhs:&Self)->Self{Self{coefficient:&self.coefficient*&rhs.coefficient,scale:self.scale+rhs.scale}}
 pub fn pow(&self,n:i32)->Self{assert!(n>=0,"ArithmeticException");Self{coefficient:self.coefficient.pow(n as u32),scale:self.scale*n}}
 pub fn divide(&self,rhs:&Self)->Self{use num_traits::{One,Zero};assert!(!rhs.coefficient.is_zero(),"ArithmeticException");let mut n=self.coefficient.clone();let d=rhs.coefficient.clone();let ten=num_bigint::BigInt::from(10u8);let mut scale=self.scale-rhs.scale;while &n%&d!=num_bigint::BigInt::zero(){n*=&ten;scale+=1;assert!(scale<100000,"Non-terminating decimal expansion");}let mut c=n/d;while scale>0&&(&c%&ten).is_zero(){c/=10;scale-=1;}Self{coefficient:c,scale}}
 fn integral(&self)->num_bigint::BigInt{let ten=num_bigint::BigInt::from(10u8);if self.scale>=0{&self.coefficient/ten.pow(self.scale as u32)}else{&self.coefficient*ten.pow((-self.scale)as u32)}}
 pub fn intValue(&self)->i32{use num_traits::ToPrimitive;let bound:num_bigint::BigInt=num_bigint::BigInt::from(1u64)<<32usize;let v=((self.integral()%&bound)+&bound)%&bound;v.to_u32().unwrap()as i32}
 pub fn longValue(&self)->i64{use num_traits::ToPrimitive;let bound:num_bigint::BigInt=num_bigint::BigInt::from(1u64)<<64usize;let v=((self.integral()%&bound)+&bound)%&bound;v.to_u64().unwrap()as i64}
}
// rust: java.util.Calendar's local Gregorian host fields. Spreadsheet serial conversion stays in GregCalc.
pub struct Calendar{value:chrono::NaiveDateTime}
impl Calendar{
 pub fn getInstance()->Self{Self{value:chrono::Local::now().naive_local()}}
 pub fn setTimeInMillis(&mut self,millis:i64){use chrono::TimeZone;self.value=chrono::Local.timestamp_millis_opt(millis).single().expect("Calendar range").naive_local();}
 pub fn get(&self,field:i32)->i32{use chrono::{Datelike,Timelike};match field{1=>self.value.year(),2=>self.value.month0()as i32,5=>self.value.day()as i32,11=>self.value.hour()as i32,12=>self.value.minute()as i32,13=>self.value.second()as i32,14=>(self.value.nanosecond()/1000000)as i32,_=>panic!("Calendar field outside dependency closure")}}
 pub fn set(&mut self,year:i32,month:i32,day:i32,hour:i32,minute:i32,second:i32){let y=year+month.div_euclid(12);let m=month.rem_euclid(12)+1;self.value=chrono::NaiveDate::from_ymd_opt(y,m as u32,1).expect("Calendar range").and_hms_opt(0,0,0).unwrap()+chrono::Duration::days(day as i64-1)+chrono::Duration::hours(hour as i64)+chrono::Duration::minutes(minute as i64)+chrono::Duration::seconds(second as i64);}
 pub fn setField(&mut self,field:i32,v:i32){use chrono::Timelike;assert_eq!(field,14);self.value=self.value.with_nanosecond(v as u32*1000000).expect("Calendar millisecond");}
}
impl kfc::cn::wps::kfc::numfmt::util::DateUtil::CalendarFields for Calendar{fn get(&self,field:i32)->i32{self.get(field)}}
// rust: JDK Math.round(double) native adapter, including the half-way neighbors where floor(a+0.5) differs.
pub fn javaMathRound(a:f64)->i64{let bits=a.to_bits()as i64;let biased=((bits as u64&0x7ff0000000000000)>>52)as i32;let shift=1074-biased;if shift&-64==0{let mut r=((bits as u64&0x000fffffffffffff)|0x0010000000000000)as i64;if bits<0{r=-r;}((r>>shift)+1)>>1}else{a as i64}}

// rust: JDK Math.round(float), preserving half-way neighbors and saturated int conversion.
pub fn javaMathRoundFloat(a:f32)->i32{javaMathRound(a as f64).clamp(i32::MIN as i64,i32::MAX as i64)as i32}

// rust: Java JSONObject widens float to double; preserve the exact float value in observation transport.
pub fn serialize_java_float<S:serde::Serializer>(v:&f32,s:S)->Result<S::Ok,S::Error>{s.serialize_f64(*v as f64)}

// rust: Character.isDigit(char) leaf; complete BMP Nd ranges generated by the local JDK 17, verified independently of ET.
pub fn javaCharacterIsDigit(c:u16)->bool{const RANGES:&[(u16,u16)]=&[(48,57),(1632,1641),(1776,1785),(1984,1993),(2406,2415),(2534,2543),(2662,2671),(2790,2799),(2918,2927),(3046,3055),(3174,3183),(3302,3311),(3430,3439),(3558,3567),(3664,3673),(3792,3801),(3872,3881),(4160,4169),(4240,4249),(6112,6121),(6160,6169),(6470,6479),(6608,6617),(6784,6793),(6800,6809),(6992,7001),(7088,7097),(7232,7241),(7248,7257),(42528,42537),(43216,43225),(43264,43273),(43472,43481),(43504,43513),(43600,43609),(44016,44025),(65296,65305)];RANGES.iter().any(|(a,z)|c>=*a&&c<=*z)}
pub mod JavaObject;
