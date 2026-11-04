// Java: public class RandomAccessFile 
#[derive(Default)]
pub struct RandomAccessFile {
}

// Java: public class RandomAccessFile 
impl RandomAccessFile {
    pub fn new() -> Self { Self::default() }
    // Java: public void close() 
    pub fn close(&mut self) {
    }

    // Java: public void getFilePointer() 
    pub fn getFilePointer(&mut self) {
    }

    // Java: public void length() 
    pub fn length(&mut self) {
    }

    // Java: public void read(Box<dyn std::any::Any> arg0, Box<dyn std::any::Any> arg1, Box<dyn std::any::Any> arg2) 
    pub fn read(&mut self, arg0: Box<dyn std::any::Any>, arg1: i64, arg2: i32) -> i32 {
        return 0;
    }

    // Java: public void seek(Box<dyn std::any::Any> arg0) 
    pub fn seek(&mut self, arg0: Box<dyn std::any::Any>) {
    }

    // Java: public void setLength(Box<dyn std::any::Any> arg0) 
    pub fn setLength(&mut self, arg0: Box<dyn std::any::Any>) {
    }

    // Java: public void skipBytes(Box<dyn std::any::Any> arg0) 
    pub fn skipBytes(&mut self, arg0: Box<dyn std::any::Any>) {
    }

    // Java: public void write(Box<dyn std::any::Any> arg0, Box<dyn std::any::Any> arg1, Box<dyn std::any::Any> arg2) 
    pub fn write(&mut self, arg0: Box<dyn std::any::Any>, arg1: Box<dyn std::any::Any>, arg2: Box<dyn std::any::Any>) {
    }

}
