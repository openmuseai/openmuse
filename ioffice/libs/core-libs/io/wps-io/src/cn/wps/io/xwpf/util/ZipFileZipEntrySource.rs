// Java: import cn.wps.base.assertion.IOAssert
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import cn.wps.moffice.kfs.zip.ZipFile
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.Enumeration

use super::ZipEntrySource::{FakeZipEntry, ZipEntryRef, ZipEntrySource};
use core_libs_kfs::kfs::zip::ZipEntry::ZipEntry as KfsZipEntry;
use core_libs_kfs::kfs::zip::ZipFile::ZipFile;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

// Rust-only: kfs 读侧（`ZipFile`/`Zip64`/`ZipEntry`/`RandomAccessFile`）在移植时把 Java 的
// `throws IOException` 写成了 `panic!`，且**没有 `try_` 层** ⇒ 本类用它把 panic 收敛成 `Err`，
// 保住「坏输入不 panic」契约（见 16_function_zip_open/FUNCTION.md §1.6 不变量 1）。
// ⚠️ 这是桥接：kfs 读侧补齐 try_ 层后应撤掉（§1.7 偏差表已登记）。
fn bridgeKfsPanic<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|payload| {
        payload
            .downcast_ref::<&str>()
            .map(|s| (*s).to_string())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "ZipException: unknown".to_string())
    })
}

// Java: public class ZipFileZipEntrySource implements ZipEntrySource
pub struct ZipFileZipEntrySource {
    // Java field: ZipFile zipArchive
    // Rust: 与 Java **同型**（kfs `ZipFile` 句柄，不加 `RefCell`）。
    zipArchive: Option<ZipFile>,
    // Rust-only: trait `getEntries()` 返回**借用切片**（Java 返回 live `Enumeration`）⇒ 自持一份
    //            物化结果。Java 侧 entry 字节是懒读的（构造期不进堆）⇒ 登记为偏差（§1.7 ①）。
    entries: Vec<FakeZipEntry>,
}

// Java: public class ZipFileZipEntrySource implements ZipEntrySource
impl ZipFileZipEntrySource {
    // Java: ZipFileZipEntrySource(ZipFile zipFile)
    pub fn ZipFileZipEntrySource(zipFile: ZipFile) -> Self {
        // Java: super();
        // Java: IOAssert.assertNotNull("zipFile should not be null.", zipFile);
        // Java: this.zipArchive = zipFile;
        Self::ZipFileZipEntrySource__materialize(zipFile)
    }

    // Rust-only: `new` 别名（模板命名）
    pub fn new(zipFile: ZipFile) -> Self {
        Self::ZipFileZipEntrySource(zipFile)
    }

    // Rust-only: 构造期把 entry 列表读出来（Java 现场调 `zipArchive.entries()`；Rust 因 trait
    //            返回借用切片而必须自持）。**entry 元数据一律取自 kfs 中央目录**
    //            （`size` = 声明尺寸、`method` = 压缩法），字节由 kfs `getInputStream` 物化
    //            ⇒ 与 Java 文件后端的 `ZipEntry` 字段一一对应（旧实现自造解析器，已废）。
    fn ZipFileZipEntrySource__materialize(zipFile: ZipFile) -> Self {
        // Java: return null != this.zipArchive ? this.zipArchive.entries() : null;
        //       kfs `entries()` 已按 `entryOrder` 保物理序（Java `LinkedHashMap` 同序）。
        let kf_entries = zipFile.entries();
        let mut entries: Vec<FakeZipEntry> = Vec::with_capacity(kf_entries.len());
        for kf_entry in &kf_entries {
            // Rust: 委托给句柄（Java 同一句）；kfs 内部每次新开一个 RAF ⇒ **从文件读**、
            //       `isOriginInput` 原样透传（Java 亦然）。
            // Java: this.zipArchive.getInputStream(entry, isOriginInput)
            let stream = bridgeKfsPanic(|| {
                zipFile.getInputStream__zipentry_boolean(kf_entry, false)
            });
            let bytes = match stream {
                Ok(Some(mut stream)) => {
                    let mut data = Vec::new();
                    stream.read_to_end(&mut data).expect("read entry");
                    data
                }
                _ => Vec::new(),
            };
            // Rust: Java 文件后端的 entry **就是** kfs `ZipEntry`（中央目录元数据）⇒ 整个放进
            //       子类的父类部分（`FakeZipEntry extends ZipEntry` 的组合承载）。
            entries.push(FakeZipEntry::from_zip_entry(kf_entry.clone(), bytes));
        }
        ZipFileZipEntrySource {
            zipArchive: Some(zipFile),
            entries,
        }
    }

    // Rust-only: 路径入口（Java 无独立方法：由 `ZipHelper.openZipFile` + 上面的构造器两步完成）。
    //            `Result` 让错误可见（本仓 API 约定）。
    pub fn from_path(path: &str) -> Result<Self, String> {
        match crate::cn::wps::io::opc::internal::ZipHelper::ZipHelper::openZipFile(path) {
            // Java: ZipFile zipFile = ZipHelper.openZipFile(path); this.zipArchive = new ZipFileZipEntrySource(zipFile);
            Some(zipFile) => Ok(Self::ZipFileZipEntrySource(zipFile)),
            None => Err(format!("Can't open the specified file: '{}'", path)),
        }
    }

    // Rust-only: 文件入口（Java 无对应物）。Java 的 `new ZipFile(File)` 同样**按路径**打开
    //            （`new RandomAccessFile(file, "r")` 用的是 `file.getPath()`），故这里也按 `name`
    //            打开；传入的 `std::fs::File` 句柄不被使用（kfs `File` 是另一个类型）。
    pub fn from_file(_file: File, name: &str) -> Result<Self, String> {
        Self::from_path(name)
    }

    // Rust-only: `PathBuf` / `Path` 入口（保留既有调用习惯）
    pub fn from_pathbuf(path: &PathBuf) -> Result<Self, String> {
        Self::from_path(path.to_string_lossy().as_ref())
    }

    // Rust-only: `Path` 入口
    pub fn from_path_ref(path: &Path) -> Result<Self, String> {
        Self::from_path(path.to_string_lossy().as_ref())
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: IOAssert.assertNotNull("zipArchive should not be null.", this.zipArchive);
        // Java: if (null != this.zipArchive)
        if let Some(mut archive) = self.zipArchive.take() {
            // Java: this.zipArchive.close();
            archive.close();
            // Java: this.zipArchive = null;
        }
    }

    // Java: public Enumeration<? extends ZipEntry> getEntries()
    pub fn getEntries(&self) -> Option<&[FakeZipEntry]> {
        // Java: IOAssert.assertNotNull("zipArchive should not be null.", this.zipArchive);
        // Java: return null != this.zipArchive ? this.zipArchive.entries() : null;
        if self.zipArchive.is_some() {
            Some(self.entries.as_slice())
        } else {
            None
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: IOAssert.assertNotNull("zipArchive should not be null.", this.zipArchive);
        // Java: return null != this.zipArchive ? this.zipArchive.size() : -1;
        match self.zipArchive.as_ref() {
            // Java: this.zipArchive.size() —— kfs 侧返回 entry 个数
            Some(archive) => archive.size() as i32,
            None => -1,
        }
    }

    // Java: InputStream getInputStream(ZipEntry entry, boolean isOriginInput)
    //   ★2026-09-20 修正：形参由 `&dyn std::any::Any` 改为 Java 声明的**真实类型** `ZipEntry`
    //   （Rust 等价 `ZipEntryRef` —— 封闭枚举，覆盖基类实例与 `FakeZipEntry` 子类实例）。
    pub fn getInputStream(
        &self,
        entry: &ZipEntryRef,
        isOriginInput: bool,
    ) -> io::Result<Option<Box<dyn Read>>> {
        // Java: IOAssert.assertNotNull("zipArchive should not be null.", this.zipArchive);
        // Java: IOAssert.assertNotNull("entry should not be null.", entry);
        // Java: if (entry != null) {
        // ★Java 只调 `entry.getName()`（基类方法，两个具体实例都继承）⇒ 这里转调枚举的
        //   `getName()`，**不再**做"试转型"；旧代码里那条 `downcast_ref::<String>()` 兜底
        //   （"按名字传 entry"的 Rust-only 旁路，Java 无此路径）已删除。
        //   原 `else { return Ok(None) }` 把"类型不对"伪装成"entry 不存在"的行为也随之消失。
        // Java: String zipEntryName = entry.getName();
        //   ★Rust 局部沿用 Java 的变量名 `zipEntryName`（§3.8 要求 Java 语句注释与同名 Rust 标识符相邻）
        let zipEntryName = entry.getName().to_string();
        // Java: if (zipEntryName != null && zipEntryName.contains("../"))
        if zipEntryName.contains("../") {
            // Java: throw new IOException("Unsafe zip file.");
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Unsafe zip file."));
        }
        // rust: no-stmt（if (entry != null) 块收尾）
        // Java: return null != this.zipArchive ? this.zipArchive.getInputStream(entry, isOriginInput) : null;
        let Some(archive) = self.zipArchive.as_ref() else {
            return Ok(None);
        };
        // Rust: Java 把同一个 `ZipEntry` 对象交给 `ZipFile.getInputStream`；Rust 侧按名字回查
        //       （kfs `getEntry` 是 `&self`，kfs `ZipEntry` 可 `clone`）。
        let Some(kf_entry) = archive.getEntry(&zipEntryName).cloned() else {
            return Ok(None);
        };
        // Rust: 委托给句柄（Java 同一句）；kfs 内部每次新开一个 RAF ⇒ **从文件读**、entry 字节
        //       不进堆（与 Java 一致），且 `isOriginInput` 原样透传（Java 亦然）。
        match bridgeKfsPanic(|| archive.getInputStream__zipentry_boolean(&kf_entry, isOriginInput)) {
            Ok(Some(stream)) => Ok(Some(stream)),
            Ok(None) => Ok(None),
            Err(error) => Err(io::Error::new(io::ErrorKind::InvalidData, error)),
        }
    }

    // Java: InputStream getInputStream(ZipEntry entry)
    //   ★同上：形参 = Java 声明的真实类型 `ZipEntry`（Rust 等价 `ZipEntryRef`）
    pub fn getInputStream_default(
        &self,
        entry: &ZipEntryRef,
    ) -> io::Result<Option<Box<dyn Read>>> {
        // Java: return this.getInputStream(entry, false);
        self.getInputStream(entry, false)
    }

    // Java: String getName()
    pub fn getName(&self) -> Option<String> {
        // Java: return null != this.zipArchive ? this.zipArchive.getName() : null;
        self.zipArchive
            .as_ref()
            .map(|archive| archive.getName().to_string())
    }

    // Rust-only: 按名取 kfs `ZipEntry`（中央目录元数据）—— 给本 function 的用例与 `19` 面用；
    //            Java 侧调用方本来就持有 `ZipEntry` 对象，不需要这层。
    pub fn getZipEntryMetadata(&self, name: &str) -> Option<KfsZipEntry> {
        self.zipArchive
            .as_ref()
            .and_then(|archive| archive.getEntry(name).cloned())
    }
}

impl ZipEntrySource for ZipFileZipEntrySource {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn getEntries(&self) -> Option<&[FakeZipEntry]> {
        ZipFileZipEntrySource::getEntries(self)
    }

    fn size(&self) -> i32 {
        ZipFileZipEntrySource::size(self)
    }

    fn getInputStream_with_flag(
        &self,
        var1: &ZipEntryRef,
        var2: bool,
    ) -> Result<Box<dyn Read>, String> {
        match ZipFileZipEntrySource::getInputStream(self, var1, var2) {
            Ok(Some(stream)) => Ok(stream),
            Ok(None) => Err("entry not found".to_string()),
            Err(error) => Err(error.to_string()),
        }
    }

    fn getInputStream(&self, var1: &ZipEntryRef) -> Result<Box<dyn Read>, String> {
        ZipEntrySource::getInputStream_with_flag(self, var1, false)
    }

    fn close(&mut self) {
        ZipFileZipEntrySource::close(self);
    }
}
