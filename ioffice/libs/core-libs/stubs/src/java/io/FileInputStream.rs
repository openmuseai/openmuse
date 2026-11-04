// Java: public class FileInputStream 
#[derive(Default)]
pub struct FileInputStream {
}

// Java: public class FileInputStream 
impl FileInputStream {
    pub fn new() -> Self { Self::default() }
    // Java: public void read(Object arg0, Object arg1, Object arg2) 
    pub fn read(&mut self, arg0: Vec<u8>, arg1: i32, arg2: i32) -> i32 {
        return 0;
    }

    // Java: public void skip(Object arg0) 
    pub fn skip(&mut self, arg0: i64) -> i64 {
        return 0;
    }

    // Java: public void available() 
    pub fn available(&mut self) -> i32 {
        return 0;
    }

}
