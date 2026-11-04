//! Native UTC GregorianCalendar fields for SimpleDateFormat's yyyy-MM-dd'T'HH:mm:ss'Z'.
//! Calendar's original 1582 Gregorian cutover and year-of-era are retained.
pub fn format(millis:i64)->String{
    let epochDay=millis.div_euclid(86_400_000);let second=millis.rem_euclid(86_400_000)/1000;
    let (year,month,day)=if epochDay>=-141427{
        let z=epochDay+719468;let era=z.div_euclid(146097);let doe=z-era*146097;
        let yoe=(doe-doe/1460+doe/36524-doe/146096)/365;let y=yoe+era*400;
        let doy=doe-(365*yoe+yoe/4-yoe/100);let mp=(5*doy+2)/153;
        let day=doy-(153*mp+2)/5+1;let month=mp+if mp<10{3}else{-9};(y+if month<=2{1}else{0},month,day)
    }else{
        let c=epochDay+2440588+32082;let d=(4*c+3).div_euclid(1461);let e=c-(1461*d).div_euclid(4);let m=(5*e+2).div_euclid(153);
        (d-4800+m.div_euclid(10),m+3-12*m.div_euclid(10),e-(153*m+2).div_euclid(5)+1)
    };
    let year=if year<=0{1-year}else{year};
    format!("{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",second/3600,second/60%60,second%60)
}

// Native SimpleDateFormat leaf for the fixed OPC pattern. Numeric field parsing,
// GregorianCalendar leniency/cutover and Android's zone adjustment are kept here;
// OPC/ET still owns when parsing is called, partial state and negative-date clamp.
use chrono::{TimeZone,DateTime,Utc};
use chrono_tz::{Tz,OffsetComponents};
thread_local!{static DEFAULT_ZONE:std::cell::RefCell<Option<String>>=std::cell::RefCell::new(None);}
pub fn withTimeZone<T>(zone:&str,f:impl FnOnce()->T)->T{
    struct Reset(Option<String>);impl Drop for Reset{fn drop(&mut self){DEFAULT_ZONE.with(|s|*s.borrow_mut()=self.0.take());}}
    let previous=DEFAULT_ZONE.with(|s|s.replace(Some(zone.into())));let _reset=Reset(previous);f()
}
fn zone()->Tz{let name=DEFAULT_ZONE.with(|s|s.borrow().clone()).or_else(||std::env::var("TZ").ok()).or_else(||iana_time_zone::get_timezone().ok()).unwrap_or_else(||"UTC".into());name.parse().unwrap_or(chrono_tz::UTC)}
fn offsets(zone:Tz,millis:i64)->(i64,i64){let dt=DateTime::<Utc>::from_timestamp_millis(millis).unwrap_or(if millis<0{DateTime::<Utc>::MIN_UTC}else{DateTime::<Utc>::MAX_UTC});let offset=zone.offset_from_utc_datetime(&dt.naive_utc());(offset.base_utc_offset().num_milliseconds(),offset.dst_offset().num_milliseconds())}
fn number(text:&[u16],pos:&mut usize)->Option<i32>{
    // java SimpleDateFormat.subParse skips only ' ' and '\t' before NumberFormat.parse.
    while matches!(text.get(*pos),Some(32|9)){*pos+=1;}
    let mut negative=false;if text.get(*pos)==Some(&45){negative=true;*pos+=1;}
    if text.get(*pos..*pos+3)==Some(&[78,97,78][..]){*pos+=3;return Some(0);}
    if text.get(*pos)==Some(&0x221e){*pos+=1;return Some(if negative{i32::MIN}else{i32::MAX});}
    let mut digits=String::new();while let Some(&c)=text.get(*pos){let d=crate::native_runtime::DecimalInteger::characterDigit(c);if d<0{break;}digits.push((b'0'+d as u8)as char);*pos+=1;}
    if digits.is_empty(){return None;}
    let mut exponent=0i32;
    // Android DecimalFormat delegates to ICU, accepting both E and e.
    if matches!(text.get(*pos),Some(69|101)){let start=*pos;*pos+=1;let neg=text.get(*pos)==Some(&45);if neg{*pos+=1;}let mut count=0;while let Some(&c)=text.get(*pos){let d=crate::native_runtime::DecimalInteger::characterDigit(c);if d<0{break;}exponent=exponent.saturating_mul(10).saturating_add(d);count+=1;*pos+=1;}if count==0{*pos=start;exponent=0;}else if neg{exponent=exponent.saturating_neg();}}
    // DecimalFormat returns a Long for exactly representable integral values,
    // otherwise a Double. Number.intValue therefore wraps Long and clamps Double.
    let mut integral=digits.clone();let exact=if exponent>=0&&exponent<=32{integral.extend(std::iter::repeat('0').take(exponent as usize));true}else if exponent<0&&(-(exponent as i64))<=integral.len()as i64&&integral.as_bytes()[integral.len()-(-exponent as usize)..].iter().all(|b|*b==b'0'){integral.truncate(integral.len()-(-exponent as usize));if integral.is_empty(){integral.push('0');}true}else{false};
    if exact{let signed=if negative{format!("-{integral}")}else{integral};if let Ok(value)=signed.parse::<i64>(){return Some(value as i32);}}
    let text=format!("{}{}e{}",if negative{"-"}else{""},digits,exponent);Some(text.parse::<f64>().unwrap_or(if negative{f64::NEG_INFINITY}else{f64::INFINITY})as i32)
}
fn gregorian_day(year:i64,month:i64,day:i64)->i64{let y=year-if month<=2{1}else{0};let era=y.div_euclid(400);let yoe=y-era*400;let mp=month+if month>2{-3}else{9};era*146097+yoe*365+yoe/4-yoe/100+(153*mp+2)/5+day-1-719468}
fn julian_day(year:i64,month:i64,day:i64)->i64{let a=(14-month).div_euclid(12);let y=year+4800-a;let m=month+12*a-3;day+(153*m+2).div_euclid(5)+365*y+y.div_euclid(4)-32083-2440588}
pub fn parse(text:&str)->Option<i64>{
    let text:Vec<_>=text.encode_utf16().collect();let mut pos=0usize;let mut fields=[0i32;6];
    // Fixed compiledPattern: yyyy '-' MM '-' dd 'T' HH ':' mm ':' ss 'Z'.
    for (i,separator)in [45u16,45,84,58,58,90].into_iter().enumerate(){fields[i]=number(&text,&mut pos)?;if text.get(pos)!=Some(&separator){return None;}pos+=1;}
    // java CalendarBuilder.establish: unset fields are cleared; these six fields
    // are set with YEAR literal (yyyy has no two-digit century adjustment).
    let month=fields[1].wrapping_sub(1);let year=fields[0].wrapping_add(month.div_euclid(12))as i64;let month=month.rem_euclid(12)as i64+1;
    // java timeOfDay += (long) internalGet(HOUR_OF_DAY); timeOfDay *= 60;
    // java timeOfDay += internalGet(MINUTE); timeOfDay *= 60;
    // java timeOfDay += internalGet(SECOND); timeOfDay *= 1000;
    let timeOfDay=((fields[3]as i64*60+fields[4]as i64)*60+fields[5]as i64)*1000;
    // java long fixedDate = timeOfDay / ONE_DAY; timeOfDay %= ONE_DAY;
    // java while (timeOfDay < 0) { timeOfDay += ONE_DAY; --fixedDate; }
    let fixedDate=timeOfDay.div_euclid(86_400_000);let timeOfDay=timeOfDay.rem_euclid(86_400_000);
    let gday=fixedDate+gregorian_day(year,month,1)+(fields[2]as i64-1);let jday=fixedDate+julian_day(year,month,1)+(fields[2]as i64-1);
    // java calculateFixedDate: year > cutoverYear first tries Gregorian;
    // java year < cutoverYear starts with Julian (gfd = jfd on crossing);
    // java the lenient missing period selects jfd.
    let day=if fields[0]<1582{jday}else if gday>=-141427{gday}else{jday};
    // java long millis = (fixedDate - EPOCH_OFFSET) * ONE_DAY + timeOfDay;
    let millis=day.wrapping_mul(86_400_000).wrapping_add(timeOfDay);
    let zone=zone();
    // java GregorianCalendar.adjustForZoneAndDaylightSavingsTime, tzMask==0:
    // int gmtOffset = zone.getRawOffset(); long standardTimeInZone = utcTimeInMillis - gmtOffset;
    let raw=offsets(zone,crate::native_runtime::currentTimeMillis()).0;let standardTimeInZone=millis.wrapping_sub(raw);
    // java zoneInfo.getOffsetsByUtcTime(standardTimeInZone, zoneOffsets);
    let (zoneOffset,mut dstOffset)=offsets(zone,standardTimeInZone);
    // java if (dstOffset != 0 && !zone.inDaylightTime(new Date(standardTimeInZone - dstOffset))) { dstOffset = 0; }
    if dstOffset!=0&&offsets(zone,standardTimeInZone.wrapping_sub(dstOffset)).1==0{dstOffset=0;}
    // java return (utcTimeInMillis - zoneOffset) - dstOffset;
    Some(millis.wrapping_sub(zoneOffset).wrapping_sub(dstOffset))
}
