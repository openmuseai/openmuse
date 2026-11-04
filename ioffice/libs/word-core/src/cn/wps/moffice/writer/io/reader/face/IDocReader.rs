// Java: import cn.wps.crypt.provider.ProviderInfo
use io_poi_compact::cn::wps::crypt::provider::ProviderInfo::*;
// Java: import java.io.IOException
use stubs::java::io::IOException::*;

// java public interface IDocReader
pub trait IDocReader {
    // java  boolean read()
    fn read(&mut self) -> bool;

    // java  boolean initEncryptKey(String var1)
    fn initEncryptKey(&mut self, var1: &str) -> bool;

    // java  ProviderInfo getProviderInfo()
    fn getProviderInfo(&self) -> &ProviderInfo;

    // java  void dumpAllPictures()
    fn dumpAllPictures(&mut self);

    // java  void dumpAllOLEFiles()
    fn dumpAllOLEFiles(&mut self);

    // java  void dispose()
    fn dispose(&mut self);

}
