#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// M-PPT-KMOSLIDE SP2c：按 Java 真身（`KmoMediaData.java` 115 行）重写，替换原 form-E 自动桩
// （28 行 / 21 `todo!` / 用 `use crate::*;` 的假编译面）。
// owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c

// Java: package cn.wps.show.app.media
// Java: import cn.wps.base.log.IOLog
// rust: no-stmt —— 仅 `calcRealUid()` 的 `catch (IOException e)` 分支使用；该分支的
//   `MD4Util.getMD4(File)` 不可达（见该方法的 DEFERRED_NON_FUNCTION），catch 分支随之延后。
//   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
// Java: import cn.wps.moffice.util.FileDataStorage
use util::cn::wps::moffice::util::FileDataStorage::FileDataStorage;
// Java: import cn.wps.moffice.util.MD4Util
// rust: no-stmt —— Rust 侧 `util::cn::wps::moffice::util::MD4Util` **未登记**
//   （`libs/core-libs/util/util/src/cn/wps/moffice/util/mod.rs:16` 仍是 `// pub mod MD4Util;`），
//   见 `calcRealUid()` 的 DEFERRED_NON_FUNCTION。
//   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
// Java: import java.io.IOException
// rust: no-stmt —— Rust 侧错误通道是 `io::Result`（见 `MD4Util` 的端口化），catch 分支随之延后。
use std::cell::RefCell;
use std::rc::Rc;

// rust: 别名（Java 无）—— Java 的 `FileDataStorage` / `KmoMediaData` 都是**引用类型**：同一个对象
//   被媒体库表、map 与调用方共享并原地修改（`calcRealUid()` 缓存 `realUid`、`setShareId()`、
//   `getData().deleteFile()`）。Rust 侧以 `Rc<RefCell<..>>` 承载该共享可变引用语义，命名沿用 repo
//   既有约定 `Shared<T> = Rc<RefCell<T>>`（见 `show/pptxr/handler/shape/SwPictureHandler.rs:33`）。
//   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
pub type Shared<T> = Rc<RefCell<T>>;

// Java: public class KmoMediaData
pub struct KmoMediaData {
    // Java field: private byte[] uid
    //
    // rust: Java `byte[]`（可为 null）⇒ `Option<Vec<i8>>`，与 `MD4Util`/`createMeadUID` 的
    //   `Vec<i8>` 字节承载一致（repo 全仓 `byte[]` → `Vec<i8>`）。
    pub(crate) uid: Option<Vec<i8>>,
    // Java field: private FileDataStorage data
    pub(crate) data: Option<Shared<FileDataStorage>>,
    // Java field: private int type
    pub(crate) r#type: i32,
    // Java field: private String externalPath
    pub(crate) externalPath: Option<String>,
    // Java field: private String sId
    pub(crate) sId: Option<String>,
    // Java field: private String fileId
    pub(crate) fileId: Option<String>,
    // Java field: private boolean downloading = false
    pub(crate) downloading: bool,
    // Java field: private String thumbPath
    pub(crate) thumbPath: Option<String>,
    // Java field: private byte[] realUid
    pub(crate) realUid: Option<Vec<i8>>,
}

// Java: private static final String TAG = null;
//
// rust: 偏差登记 —— Java 该常量的字面值就是 null；Rust 以 `None` 承载（唯一使用点是
//   `IOLog.d(TAG, ..)` 的 catch 分支，本波随 MD4 一起延后）。
const TAG: Option<&str> = None;
// Java: public static final String KSO_CLOUD_RESOURCE_LINK_SID = "KSO_CLOUD_RESOURCE_LINK_SID";
pub const KSO_CLOUD_RESOURCE_LINK_SID: &str = "KSO_CLOUD_RESOURCE_LINK_SID";

// Java: public class KmoMediaData
impl KmoMediaData {
    // Java: public KmoMediaData()
    pub fn new() -> Self {
        Self {
            // Java field initializer: private byte[] uid（Java 默认 null）
            uid: None,
            // Java field initializer: private FileDataStorage data（Java 默认 null）
            data: None,
            // Java field initializer: private int type（Java 默认 0）
            r#type: 0,
            // Java field initializer: private String externalPath（Java 默认 null）
            externalPath: None,
            // Java field initializer: private String sId（Java 默认 null）
            sId: None,
            // Java field initializer: private String fileId（Java 默认 null）
            fileId: None,
            // Java field initializer: private boolean downloading = false
            downloading: false,
            // Java field initializer: private String thumbPath（Java 默认 null）
            thumbPath: None,
            // Java field initializer: private byte[] realUid（Java 默认 null）
            realUid: None,
        }
    }

    // Java: public KmoMediaData(FileDataStorage data, byte[] uid, int type)
    //
    // rust: 重载消歧 —— Java 三个构造器在 Rust 无重载，后缀沿用 repo 的 `new_*` 约定
    //   （`FileDataStorage::new_from_data` / `new_from_zip`）；本条对应「已有存储」构造器。
    pub fn new_from_data(data: Shared<FileDataStorage>, uid: Option<Vec<i8>>, r#type: i32) -> Self {
        // Java: this.data = data;
        // Java: this.uid = uid;
        // Java: this.type = type;
        Self {
            data: Some(data),
            uid,
            r#type,
            // Java field initializer: 其余字段取 Java 默认值（null/0/false）
            ..Self::new()
        }
    }

    // Java: public KmoMediaData(String externalPath, byte[] uid, int type)
    pub fn new_from_external_path(externalPath: String, uid: Option<Vec<i8>>, r#type: i32) -> Self {
        // Java: this.externalPath = externalPath;
        // Java: this.uid = uid;
        // Java: this.type = type;
        Self {
            externalPath: Some(externalPath),
            uid,
            r#type,
            // Java field initializer: 其余字段取 Java 默认值（null/0/false）
            ..Self::new()
        }
    }

    // Java: public byte[] getUid()
    //
    // rust: 偏差登记 —— Java 返回数组引用；Rust 侧 `uid` 由 `Rc<RefCell<..>>` 之外的值字段承载，
    //   故返回克隆（调用点只做 `Arrays.equals` 比较，无语义差）。
    pub fn getUid(&self) -> Option<Vec<i8>> {
        // Java: return this.uid;
        self.uid.clone()
    }

    // Java: public byte[] calcRealUid()
    pub fn calcRealUid(&mut self) -> Option<Vec<i8>> {
        // Java: if (null != this.realUid)
        if self.realUid.is_some() {
            // Java: return this.realUid;
            return self.realUid.clone();
        }
        // Java: else if (null != this.data && null != this.data.getFile())
        //
        // rust: Java `FileDataStorage.getFile()`（无参）⇒ Rust `getFile_default()`（`:196`）。
        let has_file = self
            .data
            .as_ref()
            .map(|data| data.borrow_mut().getFile_default().is_some())
            .unwrap_or(false);
        let has_data_with_file = self.data.is_some() && has_file;
        if has_data_with_file {
            // Java: try
            {
                // Java: this.realUid = MD4Util.getMD4(this.data.getFile());
                //
                // DEFERRED_NON_FUNCTION: `MD4Util.getMD4(File)` 在 Rust 侧**不可达**：
                //   (a) `util::cn::wps::moffice::util::MD4Util` 未登记（`MD4Util.rs` 自身是
                //       编译面外的孤儿文件，其 `mod.rs:16` 仍是 `// pub mod MD4Util;`）；
                //   (b) 其唯一依赖 `MD4`（同目录 `MD4.rs`，`:28` `// pub mod MD4;`）同样未登记；
                //   (c) 即便登记：`MD4Util::getMD4_bytes_range`（:31）与 `getMD4_stream`（:39）
                //       的函数体仍是 `todo!()`，且 `getMD4_file`（:87）引用了未定义变量
                //       `file_path`（形参名是 `filepath`）⇒ 无法给出可运行实现。
                //   MD4 摘要本身是**取值型**计算，静默返回空值会把 `KmoFilm.compareMedia` 判成不等，
                //   故按「真阻塞成员」处理为显式 panic 而非空值。
                //   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
                todo!("DEFERRED_NON_FUNCTION")
            }
            // Java: catch (IOException e)
            // Java: {
            // Java:    IOLog.d(TAG, "IOException", e);
            // Java: }
            // DEFERRED_NON_FUNCTION: catch 分支与 `try` 同源（MD4 不可达），且 `IOLog` 的
            //   `d(tag,msg,Throwable)` 端口是 `IOLog::d_tr(&str,&str,&dyn std::error::Error)`。
            //   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
            // Java: return this.realUid;
        }
        // Java: else
        // Java: {
        // Java:    return null;
        // Java: }
        None
    }

    // Java: public void setUid(byte[] uid)
    pub fn setUid(&mut self, uid: Option<Vec<i8>>) {
        // Java: this.uid = uid;
        self.uid = uid;
    }

    // Java: public FileDataStorage getData()
    pub fn getData(&self) -> Option<Shared<FileDataStorage>> {
        // Java: return this.data;
        // rust: 返回同一个共享对象的句柄（`Rc` 克隆），与 Java 返回同一引用等价。
        self.data.clone()
    }

    // Java: public void setData(FileDataStorage data)
    pub fn setData(&mut self, data: Option<Shared<FileDataStorage>>) {
        // Java: this.data = data;
        self.data = data;
    }

    // Java: public int getMediaDataType()
    pub fn getMediaDataType(&self) -> i32 {
        // Java: return this.type;
        self.r#type
    }

    // Java: public void setMediaDataType(int type)
    pub fn setMediaDataType(&mut self, r#type: i32) {
        // Java: this.type = type;
        self.r#type = r#type;
    }

    // Java: public void setExternalPath(String path)
    pub fn setExternalPath(&mut self, path: String) {
        // Java: this.externalPath = path;
        self.externalPath = Some(path);
    }

    // Java: public String getExternalPath()
    pub fn getExternalPath(&self) -> Option<String> {
        // Java: return this.externalPath;
        self.externalPath.clone()
    }

    // Java: public boolean isExternalMedia()
    pub fn isExternalMedia(&self) -> bool {
        // Java: return this.externalPath != null;
        self.externalPath.is_some()
    }

    // Java: public void setShareId(String sId)
    pub fn setShareId(&mut self, sId: String) {
        // Java: this.sId = sId;
        self.sId = Some(sId);
    }

    // Java: public boolean isCloudMedia()
    pub fn isCloudMedia(&self) -> bool {
        // Java: return this.sId != null;
        self.sId.is_some()
    }

    // Java: public String getShareId()
    pub fn getShareId(&self) -> Option<String> {
        // Java: return this.sId;
        self.sId.clone()
    }

    // Java: public void setThumbPath(String thumbPath)
    pub fn setThumbPath(&mut self, thumbPath: String) {
        // Java: this.thumbPath = thumbPath;
        self.thumbPath = Some(thumbPath);
    }

    // Java: public String getThumbPath()
    pub fn getThumbPath(&self) -> Option<String> {
        // Java: return this.thumbPath;
        self.thumbPath.clone()
    }

    // Java: public void setDownloading(boolean downloading)
    pub fn setDownloading(&mut self, downloading: bool) {
        // Java: this.downloading = downloading;
        self.downloading = downloading;
    }

    // Java: public boolean isDownloading()
    pub fn isDownloading(&self) -> bool {
        // Java: return this.downloading;
        self.downloading
    }
}

// rust: 扩展（Java 无）—— `KmoMediaData()` 无参构造器等价于 Java 默认装配；`Default` 便于
//   `..Default::default()` 装配风格。
impl Default for KmoMediaData {
    fn default() -> Self {
        Self::new()
    }
}
