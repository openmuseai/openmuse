#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unreachable_code)]

// Java: import cn.wps.io.dom.Document
// Java: import cn.wps.io.dom.io.SAXReader
// Java: import java.io.InputStream
// Java: import java.util.concurrent.Callable

// Java: public class SaxReaderCallable implements Callable<Document>
pub struct SaxReaderCallable {
    // Java field: SAXReader mReader
    mReader: Option<Box<dyn std::any::Any + Send + Sync>>,
    // Java field: InputStream mIs
    mIs: Option<Box<dyn std::any::Any + Send + Sync>>,
}

// Java: public class SaxReaderCallable implements Callable<Document>
impl SaxReaderCallable {
    // Java: SaxReaderCallable(InputStream is)
    pub fn new(is: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: super();
        // Java: this.mIs = is;
        SaxReaderCallable {
            mReader: None,
            mIs: Some(is),
        }
    }

    // Java: Document call()
    pub fn call(&self) -> Option<Box<dyn std::any::Any + Send + Sync>> {
        // Java: return this.mReader.read(this.mIs);
        todo!()
    }
}
