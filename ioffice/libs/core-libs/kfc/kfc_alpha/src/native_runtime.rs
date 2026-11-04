//! Native transport for the mutable java.util.Date objects crossing OPC calls.
//! Only epoch time, object identity, construction and getTime/setTime are needed here.
use std::sync::{Arc,atomic::{AtomicI64,Ordering}};
pub mod DecimalInteger;
#[derive(Debug)]
pub struct Date { time:AtomicI64 }
impl Date {
    pub fn new()->Self{Self::fromTime(currentTimeMillis())}
    pub fn fromTime(time:i64)->Self{Self{time:AtomicI64::new(time)}}
    pub fn getTime(&self)->i64{self.time.load(Ordering::Relaxed)}
    pub fn setTime(&self,time:i64){self.time.store(time,Ordering::Relaxed);}
}
pub trait JavaDateArg { fn date(self)->Option<Arc<Date>>; }
impl JavaDateArg for Option<i64>{fn date(self)->Option<Arc<Date>>{self.map(|v|Arc::new(Date::fromTime(v)))}}
impl JavaDateArg for Option<Arc<Date>>{fn date(self)->Option<Arc<Date>>{self}}
thread_local!{static CLOCK:std::cell::RefCell<Option<Box<dyn FnMut()->i64>>>=std::cell::RefCell::new(None);}
pub fn currentTimeMillis()->i64{CLOCK.with(|s|{if let Some(f)=s.borrow_mut().as_mut(){f()}else{match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH){Ok(v)=>v.as_millis()as i64,Err(v)=>(v.duration().as_millis()as i64).wrapping_neg()}}})}
pub fn withClock<T>(clock:impl FnMut()->i64+'static,f:impl FnOnce()->T)->T{
    struct Reset(Option<Box<dyn FnMut()->i64>>);impl Drop for Reset{fn drop(&mut self){CLOCK.with(|s|*s.borrow_mut()=self.0.take());}}
    let previous=CLOCK.with(|s|s.replace(Some(Box::new(clock))));let _reset=Reset(previous);f()
}
