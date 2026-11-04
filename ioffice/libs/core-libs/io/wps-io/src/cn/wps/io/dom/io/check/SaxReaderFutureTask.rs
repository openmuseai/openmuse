#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.dom.Document
// Java: import java.util.concurrent.Callable
// Java: import java.util.concurrent.FutureTask

// Java: public class SaxReaderFutureTask extends FutureTask<Document>
pub struct SaxReaderFutureTask {
    // Java field: int TIMEOUT
    _private: (),
}

pub const TIMEOUT: i32 = 10000;

// Java: public class SaxReaderFutureTask extends FutureTask<Document>
impl SaxReaderFutureTask {
    // Java: SaxReaderFutureTask(Callable<Document> callable)
    pub fn new(_callable: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super(callable);
        SaxReaderFutureTask { _private: () }
    }
}
