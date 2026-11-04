// Java: import cn.wps.moffice.kfs.File

// Java: public interface IAmazonS3Upload
pub trait IAmazonS3Upload {
    // Java: abstract String upload(File var1, S3ProgressListener var2)
    fn upload(&self, _var1: &dyn std::any::Any, _var2: &dyn std::any::Any) -> String;

}
