// Java: package cn.wps.crypt;

// Java: import cn.wps.base.assertion.IOAssert;
// Java: import cn.wps.base.log.IOLog;
// Java: import cn.wps.core.runtime.Platform;
// Java: import cn.wps.crypt.provider.ProviderInfo;
// Java: import cn.wps.moffice.kfs.File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile;
// Java: import java.io.IOException;
// Java: import org.apache.poi.hwpf.HWPFDocumentCore;
// Java: import org.apache.poi.hwpf.model.FileInformationBlock;
// Java: import org.apache.poi.hwpf.model.io.HWPFFileSystem;
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem;
// Java: import org.apache.poi.util.IntegerField;
// Java: import org.apache.poi.util.LittleEndian;

use crate::cn::wps::crypt::provider::ProviderInfo::ProviderInfo;
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use crate::org::apache::poi::poifs::filesystem::POIFSFileSystem::POIFSFileSystem;

// Java: public final class CryptDocFile {
pub struct CryptDocFile {
    // Java: public static final String TAG = null;
    // Java: public boolean isOleZip;
    pub isOleZip: bool,
    // Java: private DocumentInputStream mainStream;
    mainStream: Option<*mut DocumentInputStream>,
    // Java: private DocumentInputStream tableStream;
    tableStream: Option<*mut DocumentInputStream>,
    // Java: private DocumentInputStream dataStream;
    dataStream: Option<*mut DocumentInputStream>,
    // Java: private POIFSFileSystem mFileDecrypted;
    mFileDecrypted: Option<*mut POIFSFileSystem>,
    // Java: private String mPassword;
    mPassword: Option<String>,
    // Java: private boolean isVerified;
    isVerified: bool,
    // Java: private ProviderInfo providerInfo;
    providerInfo: Option<ProviderInfo>,
}

impl CryptDocFile {
    // Java: public static String mDocStrmName = "WordDocument";
    pub const M_DOC_STRM_NAME: &'static str = "WordDocument";
    // Java: public static String mTable0StrmName = "0Table";
    pub const M_TABLE0_STRM_NAME: &'static str = "0Table";
    // Java: public static String mTable1StrmName = "1Table";
    pub const M_TABLE1_STRM_NAME: &'static str = "1Table";
    // Java: public static String mDataStrmName = "Data";
    pub const M_DATA_STRM_NAME: &'static str = "Data";
    // Java: public static String mEncryptionStrmName = "encryption";
    pub const M_ENCRYPTION_STRM_NAME: &'static str = "encryption";
    // Java: public static String mSummaryInformationName = "\u0000x05SummaryInformation";
    pub const M_SUMMARY_INFORMATION_NAME: &'static str = "\u{0000}x05SummaryInformation";
    // Java: public static String mDocSummaryInformationName = "\u0000x05DocumentSummaryInformation";
    pub const M_DOC_SUMMARY_INFORMATION_NAME: &'static str = "\u{0000}x05DocumentSummaryInformation";

    // Java: public CryptDocFile() {
    pub fn new() -> Self {
        // Java: super();
        CryptDocFile {
            isOleZip: false,
            mainStream: None,
            tableStream: None,
            dataStream: None,
            mFileDecrypted: None,
            mPassword: None,
            isVerified: false,
            providerInfo: None,
        }
    }

    // Java: public CryptDocFile(DocumentInputStream mainStream, DocumentInputStream tableStream, DocumentInputStream dataStream, String passwd) throws IOException {
    pub fn new_with_streams_passwd(main_stream: *mut DocumentInputStream, table_stream: *mut DocumentInputStream, data_stream: *mut DocumentInputStream, passwd: &str) -> Self {
        // Java: super();
        let mut obj = CryptDocFile {
            isOleZip: false,
            mainStream: None,
            tableStream: None,
            dataStream: None,
            mFileDecrypted: None,
            mPassword: None,
            isVerified: false,
            providerInfo: None,
        };
        // Java: this.init(mainStream, tableStream, dataStream, passwd);
        obj.init_with_passwd(main_stream, table_stream, data_stream, passwd);
        obj
    }

    // Java: public CryptDocFile(DocumentInputStream mainStream, DocumentInputStream tableStream, DocumentInputStream dataStream, ProviderInfo providerInfo) throws IOException {
    pub fn new_with_streams_provider(main_stream: *mut DocumentInputStream, table_stream: *mut DocumentInputStream, data_stream: *mut DocumentInputStream, provider_info: ProviderInfo) -> Self {
        // Java: super();
        let mut obj = CryptDocFile {
            isOleZip: false,
            mainStream: None,
            tableStream: None,
            dataStream: None,
            mFileDecrypted: None,
            mPassword: None,
            isVerified: false,
            providerInfo: None,
        };
        // Java: this.init(mainStream, tableStream, dataStream, providerInfo);
        obj.init_with_provider(main_stream, table_stream, data_stream, provider_info);
        obj
    }

    // Java: private void init(DocumentInputStream mainStream, DocumentInputStream tableStream, DocumentInputStream dataStream, String passwd) throws IOException {
    fn init_with_passwd(&mut self, main_stream: *mut DocumentInputStream, table_stream: *mut DocumentInputStream, data_stream: *mut DocumentInputStream, passwd: &str) {
        // Java: this.providerInfo = new ProviderInfo();
        let provider_info = ProviderInfo::new();
        // Java: this.providerInfo.strPassword = passwd;
        // Java: this.init(mainStream, tableStream, dataStream, this.providerInfo);
        self.mPassword = Some(passwd.to_string());
        self.init_with_provider(main_stream, table_stream, data_stream, provider_info);
    }

    // Java: private void init(DocumentInputStream mainStream, DocumentInputStream tableStream, DocumentInputStream dataStream, ProviderInfo providerInfo) throws IOException {
    fn init_with_provider(&mut self, main_stream: *mut DocumentInputStream, table_stream: *mut DocumentInputStream, data_stream: *mut DocumentInputStream, provider_info: ProviderInfo) {
        // Java: this.isVerified = false;
        self.isVerified = false;
        // Java: this.mainStream = mainStream;
        self.mainStream = Some(main_stream);
        // Java: this.tableStream = tableStream;
        self.tableStream = Some(table_stream);
        // Java: this.dataStream = dataStream;
        self.dataStream = Some(data_stream);
        // Java: this.providerInfo = providerInfo;
        self.providerInfo = Some(provider_info);
    }

    // Java: public boolean verifyPassword() throws IOException {
    pub fn verifyPassword(&mut self) -> bool {
        // Java: if (this.mainStream != null && this.tableStream != null) {
        // Java: return this.isVerified;
        // Java: return false;
        self.isVerified
    }

    // Java: public static POIFSFileSystem createPOIFSFileSystem(File file) {
    pub fn createPOIFSFileSystem(_file_path: &str) -> Option<*mut POIFSFileSystem> {
        // Java: IOAssert.assertNotNull("is should not be null", file);
        // Java: return poifsFileSystem;
        None
    }

    // Java: public DocumentInputStream getMainStream() throws IOException {
    pub fn getMainStream(&self) -> Option<*mut DocumentInputStream> {
        // Java: return directory.createDocumentInputStream(mDocStrmName);
        None
    }

    // Java: public DocumentInputStream getTableStream() throws IOException {
    pub fn getTableStream(&self) -> Option<*mut DocumentInputStream> {
        // Java: return directory.createDocumentInputStream(mTable0StrmName);
        None
    }

    // Java: public DocumentInputStream getDataStream() throws IOException {
    pub fn getDataStream(&self) -> Option<*mut DocumentInputStream> {
        // Java: return directory.createDocumentInputStream(mDataStrmName);
        None
    }
}
