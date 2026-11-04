// Java: public class FileOutputStream 
#[derive(Default)]
pub struct FileOutputStream {
}

// Java: public class FileOutputStream 
impl FileOutputStream {
    pub fn new() -> Self { Self::default() }
    // Java: public void write(Box<dyn std::any::Any> arg0, Box<dyn std::any::Any> arg1, Box<dyn std::any::Any> arg2) 
    pub fn write(&mut self, arg0: Box<dyn std::any::Any>, arg1: Box<dyn std::any::Any>, arg2: Box<dyn std::any::Any>) {
    }

    // Java: public void close() 
    pub fn close(&mut self) {
    }

}
