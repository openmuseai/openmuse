use crate::java::io::FilenameFilter::FilenameFilter;

// Java: public class File 
#[derive(Default)]
pub struct File {
}

// Java: public class File 
impl File {
    pub fn new() -> Self { Self::default() }
    // Java: public void exists() 
    pub fn exists(&mut self) -> bool {
        return false;
    }

    // Java: public void delete() 
    pub fn delete(&mut self) -> bool {
        return false;
    }

    // Java: public void isDirectory() 
    pub fn isDirectory(&mut self) -> bool {
        return false;
    }

    // Java: public void isFile() 
    pub fn isFile(&mut self) -> bool {
        return false;
    }

    // Java: public void lastModified() 
    pub fn lastModified(&mut self) -> i64 {
        return 0;
    }

    // Java: public void setLastModified(Object arg0) 
    pub fn setLastModified(&mut self, arg0: Box<dyn std::any::Any>) -> bool {
        return false;
    }

    // Java: public void length() 
    pub fn length(&mut self) -> i64 {
        return 0;
    }

    // Java: public void list() 
    pub fn list(&mut self) -> Vec<String> {
        return vec![];
    }

    pub fn list__filenamefilter(&mut self, filter: FilenameFilter) -> Vec<String> {
        return vec![];
    }

    // Java: public void renameTo(Object arg0) 
    pub fn renameTo(&mut self, arg0: Box<dyn std::any::Any>) -> bool {
        return false;
    }

    // Java: public void getAbsolutePath() 
    pub fn getAbsolutePath(&mut self) -> String {
        return "".to_string();
    }

    // Java: public void getCanonicalPath() 
    pub fn getCanonicalPath(&mut self) -> String {
        return "".to_string();
    }

    // Java: public void getParent() 
    pub fn getParent(&mut self) -> String {
        return "".to_string();
    }

    // Java: public void getPath() 
    pub fn getPath(&mut self) -> String {
        return "".to_string();
    }

}
