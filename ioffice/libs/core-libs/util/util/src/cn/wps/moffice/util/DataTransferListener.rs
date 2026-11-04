// Java: public interface DataTransferListener
pub trait DataTransferListener {
    // Java: abstract void started()
    fn started(&self);

    // Java: abstract void transferred(int var1)
    fn transferred(&self, length: i32);

    // Java: abstract void completed(String var1)
    fn completed(&self, file_path: &str);

    // Java: abstract void aborted()
    fn aborted(&self);

    // Java: abstract void failed()
    fn failed(&self);

    // Java: abstract boolean isCanceled(String var1)
    fn isCanceled(&self, file_path: &str) -> bool;
}

// Java: public class DataTransferAdapter implements DataTransferListener
pub struct DataTransferAdapter {
}

// Java: public class DataTransferAdapter implements DataTransferListener
impl DataTransferAdapter {
    // Java: DataTransferAdapter()
    pub fn new() -> Self {
        // Java: super();
        DataTransferAdapter {}
    }
}

impl DataTransferListener for DataTransferAdapter {
    // Java: void started()
    fn started(&self) {
    }

    // Java: void transferred(int length)
    fn transferred(&self, _length: i32) {
    }

    // Java: void completed(String filePath)
    fn completed(&self, _file_path: &str) {
    }

    // Java: void aborted()
    fn aborted(&self) {
    }

    // Java: void failed()
    fn failed(&self) {
    }

    // Java: boolean isCanceled(String filePath)
    fn isCanceled(&self, _file_path: &str) -> bool {
        // Java: return false;
        false
    }
}
