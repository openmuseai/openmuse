#![allow(non_snake_case)]

// Java: package cn.wps.show.app.slide
// Java: import java.util.concurrent.atomic.AtomicLong
use std::sync::atomic::{AtomicI64, Ordering};

// Java: public class SlideIdGenerator
pub struct SlideIdGenerator {}

// Java: private static AtomicLong curSlideId = new AtomicLong(256L)
static curSlideId: AtomicI64 = AtomicI64::new(256);
// Java: private static AtomicLong curMasterId = new AtomicLong(2147483648L)
static curMasterId: AtomicI64 = AtomicI64::new(2147483648);

// Java: public class SlideIdGenerator
impl SlideIdGenerator {
    // Java: static long createSlideId()
    pub fn createSlideId() -> i64 {
      // Java: return curSlideId.getAndIncrement();
      curSlideId.fetch_add(1, Ordering::SeqCst)
    }

    // Java: static long createMasterId()
    pub fn createMasterId() -> i64 {
      // Java: return curMasterId.getAndIncrement();
      curMasterId.fetch_add(1, Ordering::SeqCst)
    }

    // Java: static long createLayoutId()
    pub fn createLayoutId() -> i64 {
      // Java: return curMasterId.getAndIncrement();
      // Java 的 createLayoutId 与 createMasterId 共用同一个计数器（源码头如此）。
      curMasterId.fetch_add(1, Ordering::SeqCst)
    }
}
