// Java: import cn.wps.moffice.kfs.zip.ZipEntry
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.Enumeration

use core_libs_kfs::kfs::zip::ZipEntry::ZipEntry;
use std::io::Read;

// Java: public static class FakeZipEntry extends ZipEntry
// Rust-only: Java 的**继承**用**组合**承载 —— 父类部分就是 kfs `ZipEntry`（与 Java 同一个类，本仓已移植），
//            子类只新增 `data`；**父类成员一律转调** `self.zipEntry`，不在子类里复制字段或重写语义。
//            （旧实现把 `name`/`size`/`method` 拍平成自己的字段 ⇒ 结构上与 Java `extends ZipEntry` 不等价。）
#[derive(Clone, Debug)]
pub struct FakeZipEntry {
    // Java: extends ZipEntry —— 父类部分（kfs `zip.ZipEntry`：name / size / compressionMethod / …）
    pub(crate) zipEntry: ZipEntry,
    // Java field: byte[] data（子类**唯一**新增字段）
    pub(crate) data: Vec<u8>,
}

impl FakeZipEntry {
    // Java: FakeZipEntry(String entryName, ZipInputStream inp)
    //   super(entryName);                  → `ZipEntry::new__string`（Java `ZipEntry(String name)`）
    //   this.data = baos.toByteArray();    → `data`
    // ⇒ 继承来的 `size`/`compressionMethod` 保持 `ZipEntry` 字段默认 **-1**（`ZipEntry.java:18-19`）：
    //   该 ctor 从不 `setSize`/`setMethod`（旧 Rust 把 `getSize()` 写成 `data.len()`，属移植错误）。
    pub fn new(entry_name: &str, data: Vec<u8>) -> Self {
        // Java: super(entryName);
        FakeZipEntry {
            zipEntry: ZipEntry::new__string(entry_name.to_string()),
            data,
        }
    }

    // Rust-only: 文件后端的 entry —— Java 侧它**本身就是** kfs `ZipEntry`（元数据来自中央目录），
    //            Rust 侧把该 `ZipEntry` 整个放进父类部分（而不是拆成几个字段复制一遍）。
    //            消费者：`ZipFileZipEntrySource`（构造期读出的 entry 列表）与 `19` 的尺寸门禁。
    pub fn from_zip_entry(zip_entry: ZipEntry, data: Vec<u8>) -> Self {
        FakeZipEntry {
            zipEntry: zip_entry,
            data,
        }
    }

    // Java: public String getName()（继承自 `ZipEntry`）
    pub fn getName(&self) -> &str {
        self.zipEntry.getName()
    }

    // Java: public long getSize()（继承自 `ZipEntry`：声明尺寸；未 `setSize` ⇒ -1）
    pub fn getSize(&self) -> i64 {
        self.zipEntry.getSize()
    }

    // Java: public int getMethod()（继承自 `ZipEntry`：`compressionMethod`；未设置 ⇒ -1）
    pub fn getMethod(&self) -> i32 {
        self.zipEntry.getMethod()
    }

    // Java: public boolean isDirectory()（继承自 `ZipEntry`：名字尾字符 == '/'）
    pub fn isDirectory(&self) -> bool {
        self.zipEntry.isDirectory()
    }

    // Rust-only: 取父类部分（需要 kfs `ZipEntry` 整体时用，例如元数据对照/按名回查）。
    pub fn asZipEntry(&self) -> &ZipEntry {
        &self.zipEntry
    }

    // Rust-only: 子类字段 `data` 的读取口（`getInputStream` 与 `s01-04` 的 part 装载用）。
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    // 说明：`ZipEntry` 其余继承成员（`getCompressedSize`/`getCrc`/`getTime`/`getComment`/…）在 Java 侧由
    // `extends` 免费获得；Rust 侧不逐个转发，需要时经 `asZipEntry()` 调用（避免与 kfs `ZipEntry` 的 API 二次漂移）。
}

// Rust-only: Java `ZipEntry` **未覆写 `equals`**（只有 `hashCode`，`ZipEntry.java:212`）⇒ Java 语义是
//            **引用相等**；本结构体因此**不**实现 `PartialEq`（旧实现的 `derive(PartialEq)` 按平铺字段逐值
//            比较，属 Rust-only 发明）。需要比较时请显式比 `getName()` / `data()`。

// ══════════════════════════════════════════════════════════════════════════════════════════
// Java `cn.wps.moffice.kfs.zip.ZipEntry` 的**引用值**建模（替代旧 `dyn Any` 承载）
// ══════════════════════════════════════════════════════════════════════════════════════════
//
// ★为什么需要它：Java 侧凡声明为 `ZipEntry` 的字段/参数（`ZipPackagePart.zipEntry`、
//   `PackagePart.getOutputStreamImpl(ZipEntry,…)`、`UnmarshallContext.zipEntry`、
//   `ZipEntrySource.getInputStream(ZipEntry,…)`）**运行期可能指向两种具体实例**：
//     · 基类实例本身 —— 文件后端（`ZipFileZipEntrySource`：元数据来自中央目录）
//     · 子类 `ZipInputStreamZipEntrySource.FakeZipEntry extends ZipEntry` —— 流后端（字节已在堆）
//
// ★原实现的三个问题（`Option<Box<dyn std::any::Any + Send + Sync>>` + `downcast_ref`）：
//     ① **类型信息丢失**：编译器不再检查谁被塞进来，每个消费点都要"试转型 + 兜底"；
//     ② **任意类型可塞入**：旧代码里真的出现了 `downcast_ref::<String>()` 兜底分支
//        （`ZipFileZipEntrySource::getInputStream`），即"按名字传 entry"的 Rust-only 旁路，
//        Java 侧不存在该路径；
//     ③ **静默错误**：转不上型时旧代码 `return Ok(None)` —— 把"类型不对"伪装成"entry 不存在"。
//
// ★Rust 无继承（`FakeZipEntry` 用**组合**承载 `Java extends ZipEntry`，见其定义）⇒ 用**封闭枚举**
//   承载"同一个声明类型、两种具体实例"的 Java 语义：类型重新变成编译期可知的，下游不再有
//   试转型兜底，Java 的显式强转（`(FakeZipEntry)zipEntry`，`ZipInputStreamZipEntrySource.java:93`）
//   变成**穷尽 match**（漏分支 = 编译错误）。
#[derive(Clone, Debug)]
pub enum ZipEntryRef {
    // Java: 基类 `ZipEntry` 实例（文件后端的中央目录条目）
    Base(ZipEntry),
    // Java: `FakeZipEntry extends ZipEntry`（流后端；`ZipInputStreamZipEntrySource.java:119`）
    Fake(FakeZipEntry),
}

impl ZipEntryRef {
    // Java: `entry.getName()`（`ZipEntry` 基类方法，两个具体类型都继承）
    pub fn getName(&self) -> &str {
        match self {
            ZipEntryRef::Base(e) => e.getName(),
            ZipEntryRef::Fake(e) => e.getName(),
        }
    }

    // Java: `(FakeZipEntry)zipEntry`（`ZipInputStreamZipEntrySource.java:93` 的显式强转）
    //    Java 侧强转失败抛 `ClassCastException` ⇒ Rust 侧对应 panic（同一失败类别，**不静默**）
    pub fn as_fake(&self) -> &FakeZipEntry {
        match self {
            ZipEntryRef::Fake(e) => e,
            ZipEntryRef::Base(e) => {
                panic!("ClassCastException: {} is not a FakeZipEntry", e.getName())
            }
        }
    }

    // Rust-only: 取父类部分（元数据对照用；`Fake` 分支经组合字段转调 `asZipEntry()`）
    pub fn as_zip_entry(&self) -> &ZipEntry {
        match self {
            ZipEntryRef::Base(e) => e,
            ZipEntryRef::Fake(e) => e.asZipEntry(),
        }
    }

    // Java: `entry.getSize()`（`ZipEntry` 基类方法）
    pub fn getSize(&self) -> i64 {
        self.as_zip_entry().getSize()
    }
}

// Java: public interface ZipEntrySource
pub trait ZipEntrySource {
    // trait-object 向下转型（Java 强转具体实现的等价；见 translation.md as_any 模式）
    fn as_any(&self) -> &dyn std::any::Any;

    // Java: abstract Enumeration<? extends ZipEntry> getEntries()
    fn getEntries(&self) -> Option<&[FakeZipEntry]>;

    // Java: abstract int size()
    fn size(&self) -> i32;

    // Java: abstract InputStream getInputStream(ZipEntry var1, boolean var2)
    //   ★var1 的声明类型 = Java 基类 `ZipEntry`（**不得**收窄成 `FakeZipEntry`）
    fn getInputStream_with_flag(
        &self,
        var1: &ZipEntryRef,
        var2: bool,
    ) -> Result<Box<dyn Read>, String>;

    // Java: abstract InputStream getInputStream(ZipEntry var1)
    fn getInputStream(&self, var1: &ZipEntryRef) -> Result<Box<dyn Read>, String>;

    // Java: abstract void close()
    fn close(&mut self);
}
