#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut, unreachable_code)]

// M-PPT-KMOSLIDE SP2c：按 Java 真身（`KmoMediaLib.java` 261 行）重写，替换原 form-E 自动桩
// （33 行 / 22 `todo!` / `use crate::*;` 的假编译面）。本类型即 SP1b 的**阻塞类型**：
// `KmoFilm::updatePicRef4Copy()`（`dstMedialib.addMediaData(data, type)`）与
// `KmoFilm::compareMedia(Blip, Blip, KmoMediaLib, KmoMediaLib)` 的媒体库形参就是本文件落的
// `KmoMediaLib`；`KmoMediaData` 取自 `getMediaData()`。
// owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c

// Java: package cn.wps.show.app.media
// Java: import cn.wps.kfc.memor4j.util.KmoUndoableList
// rust: no-stmt —— `KmoUndoableList`（`libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/memor4j/util/KmoUndoableList.rs`）
//   自身仍是 form-E 自动桩（33 个 `todo!`）且**未登记**（`memor4j/util/mod.rs` 缺失 ⇒ 不可命名）。
//   Java 侧它只是字段声明类型 `List<KmoMediaData>` 的**具体实现**，本类只用到
//   `add/addAll/get/set/indexOf/size/clear` 七个 `List` 接口方法 ⇒ Rust 以
//   `Vec<Shared<KmoMediaData>>` 承载；其撤销栈语义（`willSwap`/`reverseOp`/`didFinishSwapping`）
//   在本类中从未被调用。
//   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
// Java: import cn.wps.log.KmoLog
use util::cn::wps::log::KmoLog::KmoLog;
// Java: import cn.wps.moffice.kfs.File
// rust: no-stmt —— Rust 侧 `File` 统一由 `std::path::PathBuf` 承载
//   （`FileDataStorage.mFile: Option<PathBuf>`）；`File.getAbsolutePath()` 的对应见 `getAbsolutePath()`。
// Java: import cn.wps.moffice.slim.ISlimToolManager
// rust: 说明 —— 该 import 的唯一使用点（`ISlimToolManager.instance().getSlimTool(70).checkSlim(path)`）
//   本轮 DEFERRED（见两个 `addMediaData` 的 DEFERRED_NON_FUNCTION），故当前无实际引用；
//   保留以对齐 Java import 表，落地该语句时直接可用。
//   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
use util_adapter::cn::wps::moffice::slim::ISlimToolManager::ISlimToolManager;
// Java: import cn.wps.moffice.util.FileDataStorage
use util::cn::wps::moffice::util::FileDataStorage::FileDataStorage;
// Java: import cn.wps.moffice.util.MD4Util
// rust: no-stmt —— `util::cn::wps::moffice::util::MD4Util` **未登记**
//   （`libs/core-libs/util/util/src/cn/wps/moffice/util/mod.rs:16` 仍是 `// pub mod MD4Util;`），
//   见 `createMediaUID()` / `createMeadUID()` 的 DEFERRED_NON_FUNCTION。
//   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
// Java: import cn.wps.moffice.util.UUIDUtil
use util::cn::wps::moffice::util::UUIDUtil::UUIDUtil;
// Java: import cn.wps.util.zip.model.ZipBaseModel
// rust: no-stmt —— `ZipBaseModel` 只经 `FileDataStorage.mZipBaseModel` 字段访问**被推断**，
//   本文件无需按名引入（该类型位于 `util::cn::wps::util::zip::model::ZipBaseModel`）；
//   调用 `getUncompressFile()` 亦不需要 `use`（方法解析不要求类型在作用域内）。
// Java: import java.io.UnsupportedEncodingException
// rust: no-stmt —— Rust 的 `&str::as_bytes()` 没有编码失败分支（无受检异常）；该 import 只服务
//   `createMediaUID` 的 `catch`，随其一起 DEFERRED。
//   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
// Java: import java.util.ArrayList
// rust: no-stmt —— 以 `Vec` 承载（`mCurrMediaPaths` 字段与 `getMediaPaths()` 返回值）。
// Java: import java.util.Arrays
// rust: no-stmt —— `Arrays.equals(byte[], byte[])` 在 Rust 侧由 `Option<Vec<i8>>` 的 `PartialEq`
//   承载（`None == None` 为 true、单侧 null 为 false、双侧非 null 逐元素比较），语义逐一对应。
// Java: import java.util.HashMap
// Java: import java.util.LinkedList
// Java: import java.util.List
// Java: import java.util.Map
// Java: import java.util.Queue
// rust: 别名（Java 无）—— `HashMap` 承载 Java `HashMap`；`VecDeque` 承载 Java `LinkedList`
//   作为 `Queue` 使用（本类用到的 `poll()`/`offer()` 正是 `pop_front()`/`push_back()`）。
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use crate::cn::wps::show::app::media::KmoMediaData::{KmoMediaData, Shared};

// Java: public class KmoMediaLib
pub struct KmoMediaLib {
    // Java field: Queue<MediaPriorityReader> priorityReaders = new LinkedList()
    //
    // rust: Java `LinkedList` 作 `Queue` 用时元素可为 null（`offer(null)` 合法、`poll()` 随之返回 null）；
    //   Rust `Box<dyn ..>` 不可为 null ⇒ 该 Java 空元素路径在类型层被禁止（本类调用点均传非空）。
    pub(crate) priorityReaders: VecDeque<Box<dyn MediaPriorityReader>>,
    // Java field: private List<KmoMediaData> mediaDataLst = new KmoUndoableList()
    //
    // rust: Java 列表持有的是 `KmoMediaData` **对象引用**（同一对象被 `mediaDataMap`、调用方与
    //   `copyOf()` 共享并原地修改），故元素类型为 `Shared<KmoMediaData> = Rc<RefCell<..>>`。
    pub(crate) mediaDataLst: Vec<Shared<KmoMediaData>>,
    // Java field: private ArrayList<String> mCurrMediaPaths
    //
    // rust: Java 默认 null ⇒ `Option<Vec<String>>`（`setCurrPathMedias()` 之前恒为 `None`）。
    pub(crate) mCurrMediaPaths: Option<Vec<String>>,
    // Java field: private Map<String, List<KmoMediaData>> mediaDataMap = new HashMap()
    //
    // rust: 内层 `List` 必须是**共享可变**的：`copyOf()` 的 `putAll()` 在 Java 侧把**同一个**
    //   ArrayList 引用放进新 map（两库随即共享内层列表）。故内层用
    //   `Shared<Vec<Shared<KmoMediaData>>>`：`Rc::clone` 即共享同一列表，
    //   `HashMap::extend` 即 `putAll` 的引用共享语义。
    pub(crate) mediaDataMap: HashMap<String, Shared<Vec<Shared<KmoMediaData>>>>,
}

// Java: public interface MediaPriorityReader
//
// rust: Java 的**嵌套接口** `KmoMediaLib.MediaPriorityReader` 在 Rust 侧塌平为同模块 trait。
pub trait MediaPriorityReader {
    // Java: void finishRead(boolean var1)
    //
    // rust: 接收者取 `&mut self` —— Java 侧唯一调用点（`PptxrMedias.priorityReadMedia`）在
    //   同一分支内先 `getMediaId()` 再 `finishRead(true)`，`finishRead` 是状态推进方法。
    fn finishRead(&mut self, var1: bool);
    // Java: int getMediaId()
    fn getMediaId(&self) -> i32;
}

// Java: public class KmoMediaLib
impl KmoMediaLib {
    // Java: KmoMediaLib()   （Java 未显式声明构造器 ⇒ 隐式 public 无参构造器）
    pub fn new() -> Self {
        Self {
            // Java field initializer: Queue<MediaPriorityReader> priorityReaders = new LinkedList()
            priorityReaders: VecDeque::new(),
            // Java field initializer: private List<KmoMediaData> mediaDataLst = new KmoUndoableList()
            mediaDataLst: Vec::new(),
            // Java field initializer: private ArrayList<String> mCurrMediaPaths（Java 默认 null）
            mCurrMediaPaths: None,
            // Java field initializer: private Map<String, List<KmoMediaData>> mediaDataMap = new HashMap()
            mediaDataMap: HashMap::new(),
        }
    }

    // Java: private static byte[] createMediaUID(String path)
    fn createMediaUID(path: &str) -> Vec<i8> {
        /* DEFERRED_NON_FUNCTION —— Java 原文（可原样回填）：
        try {
           return MD4Util.getMD4(path.getBytes("UTF-8"));
        } catch (UnsupportedEncodingException e) {
           throw new RuntimeException(e);
        }
        */
        // DEFERRED_NON_FUNCTION: `MD4Util.getMD4(byte[])` 在 Rust 侧不可达 ——
        //   (a) `util::cn::wps::moffice::util::MD4Util` **未登记**（`util/src/cn/wps/moffice/util/mod.rs:16`
        //       仍是 `// pub mod MD4Util;`，其 `MD4Util.rs` 属编译面外孤儿文件）；
        //   (b) 其唯一依赖 `MD4`（同目录 `MD4.rs`，`mod.rs:28` `// pub mod MD4;`）同样未登记；
        //   (c) 即便登记：`MD4Util::getMD4_bytes_range`（`MD4Util.rs:31`）与 `getMD4_stream`（`:39`）
        //       的函数体仍是 `todo!()`，`getMD4_file`（`:87`）体里还引用了未定义变量
        //       （形参名 `filepath`、函数体写 `file_path`）⇒ 没有可用的实现可调；
        //   (d) UID 是**取值型**摘要：静默返回空值会让 `KmoFilm.compareMedia` 把不同媒体判成相同，
        //       故按「真阻塞」显式 panic，而非返回空值。
        //   rust 端口化路径：`path.as_bytes()` → `MD4Util::getMD4_bytes(&[i8])`（`Vec<i8>` 承载）。
        //   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
        todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public static byte[] createMeadUID(String externalPath)
    //
    // rust: 方法名保留 Java 的拼写 `Mead`（非 `Media`），不做"修正"。
    //   Java 调用点（供后续波次定位）：`SlimTool_WPP_CompressPic.java:121`、
    //   `KmoKaiChangeThemeOperator.java:364`、`KmoSmartLayoutOperator.java:164`。
    pub fn createMeadUID(externalPath: &str) -> Vec<i8> {
        /* DEFERRED_NON_FUNCTION —— Java 原文（可原样回填）：
        try {
           return MD4Util.getMD4(externalPath.getBytes("UTF-8"));
        } catch (UnsupportedEncodingException e) {
           throw new RuntimeException(e);
        }
        */
        // DEFERRED_NON_FUNCTION: 与 `createMediaUID()` 同源（同一个 `MD4Util.getMD4(byte[])` 不可达）。
        //   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
        todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private int checkMediaExist(String uidString)
    fn checkMediaExist(&self, uidString: &str) -> i32 {
        // Java: if (null != this.mediaDataLst && this.mediaDataLst.size() > 0)
        //
        // rust: 字段 `mediaDataLst` 是 Rust `Vec`（不可为 null）⇒ 只剩 size 判据。
        if self.mediaDataLst.is_empty() {
            // Java: else { return -1; }
            return -1;
        }
        // Java: List<KmoMediaData> kmoMediaDataList = (List)this.mediaDataMap.get(uidString);
        let kmoMediaDataList = self.mediaDataMap.get(uidString);
        // Java: if (kmoMediaDataList != null && kmoMediaDataList.size() > 0)
        match kmoMediaDataList {
            Some(list) if !list.borrow().is_empty() => {
                // Java: for(int i = 0; i < kmoMediaDataList.size(); ++i)
                // Java: KmoMediaData kmoMediaData = (KmoMediaData)kmoMediaDataList.get(i);
                //
                // rust: Java 的序号变量 `i` 只用于 `get(i)` ⇒ 直接迭代元素。
                for kmoMediaData in list.borrow().iter() {
                    // Java: if (kmoMediaData != null && kmoMediaData.getData() != null)
                    //
                    // rust: 元素类型 `Shared<KmoMediaData>` 不可为 null ⇒ Java 的前半判据在类型层已满足。
                    if kmoMediaData.borrow().getData().is_some() {
                        // Java: return this.mediaDataLst.indexOf(kmoMediaData);
                        //
                        // rust: `List.indexOf` 用 `Object.equals`；`KmoMediaData` 未覆写 `equals`
                        //   ⇒ 身份相等，用 `Rc::ptr_eq` 承载（map 内层列表与 `mediaDataLst`
                        //   持有的是同一批 `Rc` 句柄）。
                        return self
                            .mediaDataLst
                            .iter()
                            .position(|m| Rc::ptr_eq(m, kmoMediaData))
                            .map(|p| p as i32)
                            .unwrap_or(-1);
                    }
                }
                // Java: return -1;（列表非空，但没有一条带 data）
                -1
            }
            // Java: else { return -1; }
            _ => -1,
        }
    }

    // Java: public int addMediaData(String externalPath, int mediaType)
    //
    // rust: 重载消歧（Java 三个 `addMediaData` 在 Rust 无重载）——
    //   · `addMediaData(String,int)`          → `addMediaData(&mut self, &str, i32)`
    //   · `addMediaData(FileDataStorage,int)` → `addMediaData_from_data(&mut self, &Shared<FileDataStorage>, i32)`
    //   · `addMediaData(KmoMediaData,String)` → `addMediaData_media_data(&mut self, Shared<KmoMediaData>, &str)`
    //   `_from_data` 沿用 repo 既有词表（`FileDataStorage::new_from_data` / `KmoMediaData::new_from_data`）。
    pub fn addMediaData(&mut self, externalPath: &str, mediaType: i32) -> i32 {
        // Java: if (null != externalPath && !externalPath.equalsIgnoreCase("NULL") && 0 != externalPath.length())
        //
        // rust: Java 的 `null != externalPath` 由 `&str` 在类型层保证；
        //   `equalsIgnoreCase("NULL")` 对字面量 "NULL" 等价于 ASCII 大小写无关比较
        //   （Java 该方法逐字符 `toUpperCase`/`toLowerCase` 折叠，字面量只含 ASCII）。
        if !externalPath.is_empty() && !externalPath.eq_ignore_ascii_case("NULL") {
            // Java: externalPath = null == externalPath ? "" : externalPath;
            //
            // rust: no-stmt —— 进入本分支已保证 `externalPath` 非 null（类型层）且非空，
            //   该三元式是 Java 的恒等自赋值。
            // Java: byte[] uid = createMeadUID(externalPath);
            let uid: Vec<i8> = Self::createMeadUID(externalPath);
            // Java: String uidString = this.uidToString(uid);
            let uidString = self.uidToString(&uid);
            // Java: int existMediaData = this.checkMediaExist(uidString);
            let existMediaData = self.checkMediaExist(&uidString);
            // Java: if (existMediaData != -1)
            if existMediaData != -1 {
                // Java: return existMediaData;
                return existMediaData;
            }
            // Java: else
            {
                // Java: ISlimToolManager.instance().getSlimTool(70).checkSlim(externalPath);
                //
                // DEFERRED_NON_FUNCTION: `checkSlim` 在 Rust 侧不可达 ——
                //   (a) `ISlimToolManager::instance()`（`libs/core-libs/util/adapter/src/cn/wps/moffice/slim/ISlimToolManager.rs:90`）
                //       的函数体是 `todo!()`；
                //   (b) `getSlimTool(70)`（同文件 `:169`）同样是 `todo!()`，且其返回类型是**类型擦除**的
                //       `Option<&Box<dyn std::any::Any + Send + Sync>>` ⇒ 拿不到 `ISlimTool` 的
                //       `checkSlim`（真身 `ISlimTool::checkSlim_with_obj`，`ISlimTool.rs:111`，
                //       亦只有注释 + `()`），没有任何可用调用路径；
                //   (c) `checkSlim` 是**触发瘦身检查**的副作用调用（返回值被丢弃），调用不到不会改变
                //       媒体数据本身的登记结果；故只本语句停车，不放宽其它语句。
                //   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
                todo!("DEFERRED_NON_FUNCTION");
                // rust: 以下两句在 `checkSlim` 落地前不可达（Java 中是 else 分支内的顺序语句）；
                //   落地时删除上面的 `todo!` 即可原样生效。
                // Java: KmoMediaData mediaData = new KmoMediaData(externalPath, uid, mediaType);
                //
                // rust: Java `new` 产出的对象在 `mediaDataLst` 中是以**引用**保存的（同一对象
                //   还被 `mediaDataMap` 共享）⇒ Rust 侧在此处包一层 `Rc<RefCell<..>>` 得到共享句柄。
                let mediaData = Rc::new(RefCell::new(KmoMediaData::new_from_external_path(
                    externalPath.to_string(),
                    Some(uid),
                    mediaType,
                )));
                // Java: return this.addMediaData(mediaData, uidString);
                self.addMediaData_media_data(mediaData, &uidString)
            }
        } else {
            // Java: return -1;
            -1
        }
    }

    // Java: private String uidToString(byte[] uid)
    //
    // rust: Java `StringBuilder.append(byte)` 追加的是该字节的**有符号十进制**字面量
    //   （`append(byte)` → `String.valueOf((int)b)`，如 -1 → "-1"）⇒ Rust 用 `i8::to_string()`；
    //   不可用 `u8`（否则 -1 会变成 "255"）。
    fn uidToString(&self, uid: &[i8]) -> String {
        // Java: StringBuilder stringBuilder = new StringBuilder();
        // Java: for(byte b : uid)
        // Java: stringBuilder.append(b);
        // Java: return stringBuilder.toString();
        uid.iter().map(|b| b.to_string()).collect()
    }

    // Java: private String getPath(FileDataStorage pData)
    fn getPath(&self, pData: &Shared<FileDataStorage>) -> String {
        // Java: ZipBaseModel zipBaseModel = pData.getZipBaseModel();
        let hasZipBaseModel = pData.borrow().getZipBaseModel().is_some();
        // Java: if (zipBaseModel != null && zipBaseModel.getUncompressFile() != null)
        //
        // rust: `getUncompressFile()`（无参）→ Rust `getUncompressFile_default()`
        //   （`ZipBaseModel.rs:219`，Java 侧即 `getUncompressFile(false)`，不修改状态）；
        //   借用冲突（`FileDataStorage::getZipBaseModel(&self)` 只给共享借用，而
        //   `getUncompressFile` 需要 `&mut`）通过 `FileDataStorage.mZipBaseModel`（`pub` 字段）取可变借用解决。
        if hasZipBaseModel {
            // Java: return zipBaseModel.getUncompressFile().getAbsolutePath();
            let uncompressFile = pData
                .borrow_mut()
                .mZipBaseModel
                .as_mut()
                .and_then(|zipBaseModel| zipBaseModel.getUncompressFile_default().clone());
            if let Some(path) = uncompressFile {
                return Self::getAbsolutePath(&path);
            }
        }
        // Java: else
        {
            // Java: File file = pData.getFile(false);
            let file = pData.borrow_mut().getFile(false).clone();
            // Java: return file != null ? file.getAbsolutePath() : UUIDUtil.generate32ID();
            match file {
                Some(path) => Self::getAbsolutePath(&path),
                None => UUIDUtil::generate32ID(),
            }
        }
    }

    // Java: public int addMediaData(FileDataStorage pData, int mediaType)
    //
    // rust: 见 `addMediaData(String,int)` 的「重载消歧」表。
    //   形参用 `&Shared<FileDataStorage>`（非 `Option`）：Java 该形参可为 null，但首个语句
    //   `pData.getZipBaseModel()` 会立刻 NPE ⇒ 空值判据上推给调用方（Java 语义即"传 null 必崩"）。
    pub fn addMediaData_from_data(&mut self, pData: &Shared<FileDataStorage>, mediaType: i32) -> i32 {
        // Java: String path = this.getPath(pData);
        let path = self.getPath(pData);
        // Java: byte[] uid = createMediaUID(path);
        let uid: Vec<i8> = Self::createMediaUID(&path);
        // Java: String uidString = this.uidToString(uid);
        let uidString = self.uidToString(&uid);
        // Java: int existMediaData = this.checkMediaExist(uidString);
        let existMediaData = self.checkMediaExist(&uidString);
        // Java: if (existMediaData != -1)
        if existMediaData != -1 {
            // Java: return existMediaData;
            return existMediaData;
        }
        // Java: else
        {
            // Java: ISlimToolManager.instance().getSlimTool(70).checkSlim(path);
            //
            // DEFERRED_NON_FUNCTION: 同 `addMediaData(String,int)`（`ISlimToolManager::instance()` /
            //   `getSlimTool()` 均 `todo!()`，且返回类型被擦除为 `dyn Any`）。
            //   owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
            todo!("DEFERRED_NON_FUNCTION");
            // rust: 以下两句在 `checkSlim` 落地前不可达；落地时删除上面的 `todo!` 即可生效。
            // Java: KmoMediaData mediaData = new KmoMediaData(pData, uid, mediaType);
            //
            // rust: 同 `addMediaData(String,int)` —— Java 新对象以引用入表，Rust 包 `Rc<RefCell<..>>`。
            let mediaData = Rc::new(RefCell::new(KmoMediaData::new_from_data(
                pData.clone(),
                Some(uid),
                mediaType,
            )));
            // Java: return this.addMediaData(mediaData, uidString);
            self.addMediaData_media_data(mediaData, &uidString)
        }
    }

    // Java: private synchronized int addMediaData(KmoMediaData mediaData, String uidString)
    //
    // rust: `synchronized` 在 Rust 无对应物 —— 本类以 `Rc`/`RefCell` 承载**单线程共享**引用，
    //   跨线程互斥不由本方法提供（repo 既有约定：模型对象不上 `Arc<Mutex>`）。
    fn addMediaData_media_data(&mut self, mediaData: Shared<KmoMediaData>, uidString: &str) -> i32 {
        // Java: this.mediaDataLst.add(mediaData);
        self.mediaDataLst.push(mediaData.clone());
        // Java: this.addMediaMap(mediaData, uidString);
        self.addMediaMap(mediaData, uidString);
        // Java: return this.mediaDataLst.size() - 1;
        self.mediaDataLst.len() as i32 - 1
    }

    // Java: public KmoMediaData getMediaData(int mediaId)
    //
    // rust: Java 可返回 null ⇒ `Option<Shared<KmoMediaData>>`（调用方 `SwPictureHandler`
    //   正是以此形态接收：`let mediaData: Option<Shared<KmoMediaData>> = ...getMediaData(mediaId)`）。
    pub fn getMediaData(&self, mediaId: i32) -> Option<Shared<KmoMediaData>> {
        // Java: if (mediaId >= 0 && mediaId < this.mediaDataLst.size())
        if mediaId >= 0 && mediaId < self.mediaDataLst.len() as i32 {
            // Java: return (KmoMediaData)this.mediaDataLst.get(mediaId);
            return self.mediaDataLst.get(mediaId as usize).cloned();
        }
        // Java: else
        {
            // Java: KmoLog.logPoint("KmoMediaData: mediaId = " + mediaId + ", mediaDataLst count = " + this.mediaDataLst.size());
            KmoLog::logPoint_msg(&format!(
                "KmoMediaData: mediaId = {}, mediaDataLst count = {}",
                mediaId,
                self.mediaDataLst.len()
            ));
            // Java: return null;
            None
        }
    }

    // Java: public boolean resetMediaData(int mediaIndex, KmoMediaData mediaData)
    pub fn resetMediaData(&mut self, mediaIndex: i32, mediaData: Shared<KmoMediaData>) -> bool {
        // Java: if (mediaIndex < this.mediaCount() && mediaIndex >= 0 && mediaData != null)
        //
        // rust: Java 的 `mediaData != null` 由非空形参在类型层保证（故不取 `Option`）。
        if mediaIndex < self.mediaCount() && mediaIndex >= 0 {
            // Java: KmoMediaData kmoMediaData = (KmoMediaData)this.mediaDataLst.get(mediaIndex);
            let kmoMediaData = self.mediaDataLst.get(mediaIndex as usize).cloned();
            // Java: if (kmoMediaData != null)
            if let Some(kmoMediaData) = kmoMediaData {
                // Java: boolean equals = Arrays.equals(kmoMediaData.getUid(), mediaData.getUid());
                let equals = kmoMediaData.borrow().getUid() == mediaData.borrow().getUid();
                // Java: if (!equals)
                if !equals {
                    // Java: List<KmoMediaData> kmoMediaDataList = (List)this.mediaDataMap.get(this.uidToString(kmoMediaData.getUid()));
                    //
                    // rust: Java `uidToString(byte[])` 对 null 数组会在 for-each 处 NPE
                    //   ⇒ Rust 以 `expect` 显式复现该崩溃（不静默取空串）。
                    let oldUid = kmoMediaData.borrow().getUid();
                    let oldUidString = self.uidToString(
                        oldUid
                            .as_deref()
                            .expect("Java NPE: KmoMediaData.uid is null (uidToString)"),
                    );
                    let kmoMediaDataList = self.mediaDataMap.get(&oldUidString).cloned();
                    // Java: if (kmoMediaDataList != null)
                    if let Some(list) = kmoMediaDataList {
                        // Java: kmoMediaDataList.remove(kmoMediaData);
                        //
                        // rust: `List.remove(Object)` 按 `equals`（此处即身份相等）删除**首个**匹配元素
                        //   ⇒ 先定位下标再 `remove(index)`，与"删除全部"的 `retain` 区分。
                        let index = list
                            .borrow()
                            .iter()
                            .position(|m| Rc::ptr_eq(m, &kmoMediaData));
                        if let Some(index) = index {
                            list.borrow_mut().remove(index);
                        }
                    }
                }
            }
            // Java: String uid = this.uidToString(mediaData.getUid());
            let uid = {
                let uid = mediaData.borrow().getUid();
                self.uidToString(
                    uid.as_deref()
                        .expect("Java NPE: KmoMediaData.uid is null (uidToString)"),
                )
            };
            // Java: this.addMediaMap(mediaData, uid);
            self.addMediaMap(mediaData.clone(), &uid);
            // Java: return this.mediaDataLst.set(mediaIndex, mediaData) != null;
            //
            // rust: `List.set` 返回被替换出的旧元素；Rust 元素类型 `Shared<KmoMediaData>`
            //   不可为 null ⇒ Java 的「旧元素 != null」判据恒真。
            let _previous = std::mem::replace(&mut self.mediaDataLst[mediaIndex as usize], mediaData);
            true
        } else {
            // Java: return false;
            false
        }
    }

    // Java: private void addMediaMap(KmoMediaData mediaData, String uid)
    fn addMediaMap(&mut self, mediaData: Shared<KmoMediaData>, uid: &str) {
        // Java: List<KmoMediaData> kmoMediaDataList = (List)this.mediaDataMap.get(uid);
        // Java: if (kmoMediaDataList == null)
        // Java: {
        // Java:    kmoMediaDataList = new ArrayList();
        // Java:    this.mediaDataMap.put(uid, kmoMediaDataList);
        // Java: }
        //
        // rust: `entry(..).or_insert_with(..)` 即 Java 的「查不到就 new + put」；
        //   新列表以 `Rc::new(RefCell::new(Vec::new()))` 承载（保持"列表对象"身份，
        //   供 `copyOf()` 的 `putAll` 共享同一列表）。
        let kmoMediaDataList = self
            .mediaDataMap
            .entry(uid.to_string())
            .or_insert_with(|| Rc::new(RefCell::new(Vec::new())))
            .clone();
        // Java: kmoMediaDataList.add(mediaData);
        kmoMediaDataList.borrow_mut().push(mediaData);
    }

    // Java: public void setCurrPathMedias(ArrayList<String> fileNames)
    //
    // rust: Java 形参可为 null（`mCurrMediaPaths` 随之为 null，`getCurrMedia` 的首个判据用到）
    //   ⇒ 保留 `Option<Vec<String>>`。
    pub fn setCurrPathMedias(&mut self, fileNames: Option<Vec<String>>) {
        // Java: this.mCurrMediaPaths = fileNames;
        self.mCurrMediaPaths = fileNames;
    }

    // Java: private String getCurrMedia(String path)
    //
    // rust: Java 是 `private`；Rust 侧 `getMediaPath()` 需要它完成"外部媒体路径就近解析"，
    //   保持私有（同模块内可调），与 Java 可见性一致。
    fn getCurrMedia(&self, path: &str) -> Option<String> {
        // Java: if (this.mCurrMediaPaths == null)
        let mCurrMediaPaths = match self.mCurrMediaPaths.as_ref() {
            Some(paths) => paths,
            // Java: return null;
            None => return None,
        };
        // Java: else
        {
            // Java: String fileName = path.substring(path.lastIndexOf("\\") + 1);
            //
            // rust: `lastIndexOf` 未命中返回 -1 ⇒ Java 从 0 截取整串；`rfind` 无命中即整串，同义。
            let fileName = match path.rfind('\\') {
                Some(index) => &path[index + 1..],
                None => path,
            };
            // Java: for(String str : this.mCurrMediaPaths)
            for str in mCurrMediaPaths {
                // Java: if (str.endsWith(fileName))
                if str.ends_with(fileName) {
                    // Java: return str;
                    return Some(str.clone());
                }
            }
            // Java: return null;
            None
        }
    }

    // Java: public String getMediaPath(int mediaId)
    pub fn getMediaPath(&self, mediaId: i32) -> String {
        // Java: KmoMediaData data = this.getMediaData(mediaId);
        let data = self.getMediaData(mediaId);
        // Java: if (data == null)
        let data = match data {
            // Java: return "";
            None => return String::new(),
            Some(data) => data,
        };
        // Java: else if (data.isExternalMedia() && data.getData() == null)
        let isExternalMedia = data.borrow().isExternalMedia();
        let hasData = data.borrow().getData().is_some();
        if isExternalMedia && !hasData {
            // Java: String path = data.getExternalPath();
            let path = data.borrow().getExternalPath();
            // Java: String newPath = this.getCurrMedia(path);
            let newPath = self.getCurrMedia(path.as_deref().unwrap_or(""));
            // Java: return null != newPath ? newPath : "";
            return newPath.unwrap_or_default();
        }
        // Java: else
        {
            // Java: FileDataStorage dataStorage = data.getData();
            //
            // rust: 走到这里 `getData()` 必为 `Some`（`isExternalMedia()` 为 true 时上面已返回；
            //   为 false 且 data 为 null 时 Java 会在 `dataStorage.getSize()` 处 NPE）⇒ 显式复现。
            let dataStorage = data
                .borrow()
                .getData()
                .expect("Java NPE: FileDataStorage data is null (getMediaPath)");
            // Java: if (dataStorage.getSize() <= 0)
            if dataStorage.borrow_mut().getSize() <= 0 {
                // Java: return "";
                return String::new();
            }
            // Java: else
            {
                // Java: File file = dataStorage.getFile();
                let file = dataStorage.borrow_mut().getFile_default().clone();
                // Java: return file.getAbsolutePath();
                //
                // rust: Java 此处对 null `file` 会 NPE ⇒ 显式复现（`getFile_default()` 仅在
                //   `mFile` 与 zip 模型都取不到文件时返回 None）。
                let file = file.expect("Java NPE: File is null (getMediaPath)");
                Self::getAbsolutePath(&file)
            }
        }
    }

    // Java: public void clear()
    pub fn clear(&mut self) {
        // Java: if (null != this.mediaDataLst && this.mediaDataLst.size() > 0)
        //
        // rust: `mediaDataLst` 是 `Vec`（不可为 null）⇒ 只剩 size 判据。
        if !self.mediaDataLst.is_empty() {
            // Java: int i = 0;
            // Java: for(int count = this.mediaDataLst.size(); i < count; ++i)
            //
            // rust: Java 在循环前取一次 `count`（循环内不再取）⇒ `while` + 预取长度。
            let count = self.mediaDataLst.len() as i32;
            let mut i = 0;
            while i < count {
                // Java: KmoMediaData mediaData = (KmoMediaData)this.mediaDataLst.get(i);
                let mediaData = self.mediaDataLst.get(i as usize).cloned();
                // Java: if (null != mediaData && null != mediaData.getData())
                if let Some(mediaData) = mediaData {
                    // Java: FileDataStorage fds = mediaData.getData();
                    let fds = mediaData.borrow().getData();
                    // Java: if (fds != null)
                    if let Some(fds) = fds {
                        // Java: fds.deleteFile();
                        fds.borrow_mut().deleteFile();
                    }
                }
                // Java: ++i;
                i += 1;
            }
            // Java: this.mediaDataMap = new HashMap();
            //
            // rust: Java 换成**新** map（旧内层列表不再被本对象引用）；`HashMap::new()` 同义。
            self.mediaDataMap = HashMap::new();
            // Java: this.mediaDataLst.clear();
            self.mediaDataLst.clear();
        }
    }

    // Java: public MediaPriorityReader getMediaPriorityReader()
    pub fn getMediaPriorityReader(&mut self) -> Option<Box<dyn MediaPriorityReader>> {
        // Java: return (MediaPriorityReader)this.priorityReaders.poll();
        self.priorityReaders.pop_front()
    }

    // Java: public void setMediaPriorityReader(MediaPriorityReader mediaPriorityReader)
    pub fn setMediaPriorityReader(&mut self, mediaPriorityReader: Box<dyn MediaPriorityReader>) {
        // Java: this.priorityReaders.offer(mediaPriorityReader);
        self.priorityReaders.push_back(mediaPriorityReader);
    }

    // Java: public int mediaCount()
    pub fn mediaCount(&self) -> i32 {
        // Java: return this.mediaDataLst.size();
        self.mediaDataLst.len() as i32
    }

    // Java: public List<String> getMediaPaths(int typeBegin, int typeEnd)
    pub fn getMediaPaths(&self, typeBegin: i32, typeEnd: i32) -> Vec<String> {
        // Java: List<String> paths = new ArrayList();
        let mut paths: Vec<String> = Vec::new();
        // Java: for(KmoMediaData mediaData : this.mediaDataLst)
        for mediaData in self.mediaDataLst.iter() {
            // Java: int type = mediaData.getMediaDataType();
            let r#type = mediaData.borrow().getMediaDataType();
            // Java: if (type > typeBegin && type < typeEnd && mediaData.getData() != null && mediaData.getData().getFile() != null)
            //
            // rust: 后半两句是同一 `getData()` 上的两次判据（Java 调了两遍 `getData()` 与
            //   `getFile()`），Rust 合并为一次取值 + 一次文件判据，语义相同。
            let hasFile = mediaData
                .borrow()
                .getData()
                .map(|dataStorage| dataStorage.borrow_mut().getFile_default().is_some())
                .unwrap_or(false);
            if r#type > typeBegin && r#type < typeEnd && hasFile {
                // Java: String path = mediaData.getData().getFile().getAbsolutePath();
                let path = mediaData
                    .borrow()
                    .getData()
                    .and_then(|dataStorage| dataStorage.borrow_mut().getFile_default().clone())
                    .map(|file| Self::getAbsolutePath(&file))
                    // rust: 上面的 `hasFile` 判据已保证此处为 `Some`；`unwrap_or_default` 仅为
                    //   消除 `Option`（Java 同一路径不会取到 null）。
                    .unwrap_or_default();
                // Java: paths.add(path);
                paths.push(path);
            }
        }
        // Java: return paths;
        paths
    }

    // Java: public static KmoMediaLib copyOf(KmoMediaLib other)
    pub fn copyOf(other: &KmoMediaLib) -> KmoMediaLib {
        // Java: KmoMediaLib now = new KmoMediaLib();
        let mut now = KmoMediaLib::new();
        // Java: if (other.mCurrMediaPaths != null)
        if let Some(paths) = other.mCurrMediaPaths.as_ref() {
            // Java: now.mCurrMediaPaths = new ArrayList();
            // Java: now.mCurrMediaPaths.addAll(other.mCurrMediaPaths);
            //
            // rust: Java 复制的是 `String` 引用（不可变对象）⇒ `Vec<String>` 克隆等价。
            now.mCurrMediaPaths = Some(paths.clone());
        }
        // Java: now.mediaDataLst.addAll(other.mediaDataLst);
        //
        // rust: `addAll` 加入的是**元素引用** ⇒ `Rc::clone`（浅拷贝，不深拷贝 `KmoMediaData`）。
        now.mediaDataLst.extend(other.mediaDataLst.iter().cloned());
        // Java: now.mediaDataMap.putAll(other.mediaDataMap);
        //
        // rust: `putAll` 存入的是**同一个内层 List 引用**（两库随之共享内层列表）⇒ 内层值
        //   `Shared<Vec<..>>` 用 `Rc::clone` 共享同一列表，外层 `HashMap` 各自独立。
        now.mediaDataMap
            .extend(other.mediaDataMap.iter().map(|(uid, list)| (uid.clone(), list.clone())));
        // Java: return now;
        now
    }

    // rust: 扩展（Java 无）—— `File.getAbsolutePath()` 在 Rust 侧由 `PathBuf` 承载，
    //   需显式取绝对路径字符串。`std::path::absolute` 与 Java 语义一致（相对路径拼当前工作目录、
    //   不解析符号链接、不做 `..` 规约）；Java `new File("").getAbsolutePath()` 返回当前工作目录，
    //   而 Rust `std::path::absolute("")` 返回 `Err` ⇒ 对空路径显式取 `current_dir()`，消除该处偏差
    //   （`FileDataStorage::new_from_zip` 的 `mFile = Some(PathBuf::from(""))` 会走到）。
    fn getAbsolutePath(path: &std::path::Path) -> String {
        if path.as_os_str().is_empty() {
            return std::env::current_dir()
                .map(|dir| dir.to_string_lossy().into_owned())
                .unwrap_or_default();
        }
        std::path::absolute(path)
            .map(|abs| abs.to_string_lossy().into_owned())
            .unwrap_or_else(|_| path.to_string_lossy().into_owned())
    }
}

// rust: 扩展（Java 无）—— `KmoMediaLib` 在 Java 只有隐式无参构造器；`Default` 便于
//   `..Default::default()` 装配风格，语义等同 `new()`。
impl Default for KmoMediaLib {
    fn default() -> Self {
        Self::new()
    }
}
