// java public interface KmoBookOpenPasswordProvider {
pub trait KmoBookOpenPasswordProvider:Send+Sync{
    // java String getReadPassword(boolean var1) throws EncryptFileException;
    fn getReadPassword(&self,var1:bool)->std::io::Result<Option<String>>;
    // java void verifyReadPassword(boolean var1);
    fn verifyReadPassword(&self,var1:bool);
    // java String getWritePassword(boolean var1);
    fn getWritePassword(&self,var1:bool)->Option<String>;
    // java void verifyWritePassword(boolean var1);
    fn verifyWritePassword(&self,var1:bool);
    // java boolean tryIfPasswdError();
    fn tryIfPasswdError(&self)->bool;
}
