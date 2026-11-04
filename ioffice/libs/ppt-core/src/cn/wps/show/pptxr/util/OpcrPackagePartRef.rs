#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::io;

// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use kfc_kfc_alpha::cn::wps::kfc::opcr::OpcrPackagePart::OpcrPackagePart;
// Java: import cn.wps.kfc.opcr.OpcrRelationship
use kfc_kfc_alpha::cn::wps::kfc::opcr::OpcrRelationship::OpcrRelationship;
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use kfc_kfc_alpha::cn::wps::kfc::opcr::OpcrRelationships::OpcrRelationships;
// Java: import cn.wps.kfc.opcr.ZipFile / ZipEntry
use kfc_kfc_alpha::cn::wps::kfc::opcr::ZipFile::{ZipEntry, ZipFile};

/// Java `OpcrPackagePart` 引用的 Rust 端口载体。
///
/// `rust:` kfc 真实 OPC 类型不可 `Clone`，且
/// `OpcrPackagePart::getRelationships(&mut self)` / `OpcrRelationship::getPackagePart(&mut self)`
/// 都要求**独占借用**（与 Java 的引用语义一致）。Java 里
/// `PptxrPresentation.mPresPart`、`PptxrThemes`/`PptxrTableStyles`/… 的 `part` 参数
/// 都是 `OpcrPackagePart` 引用，且这些引用会跨方法存活。
///
/// Rust 侧用本类型承载**同一个 part 句柄**：`(ZipFile, ZipEntry)` 都是真实、可 `Clone` 的
/// kfc 值，`OpcrPackagePart::OpcrPackagePart(&ZipFile, &ZipEntry)` 就是 kfc 自己的真实构造器，
/// 故 `part()` 每次重建的都是**同一个真实 part**（不是替身/假件）。
#[derive(Clone)]
pub struct OpcrPackagePartRef {
    // java OpcrPackagePart（引用）→ rust (ZipFile, ZipEntry)
    mZipFile: ZipFile,
    mZipEntry: ZipEntry,
}

impl OpcrPackagePartRef {
    // rust: 从真实 `OpcrPackagePart` 取句柄（Java 的 `part` 引用）
    pub fn of(part: &OpcrPackagePart) -> io::Result<Self> {
        Ok(Self {
            mZipFile: part.getZipFile(),
            mZipEntry: part.getZipEntry()?,
        })
    }

    // rust: 从真实 `OpcrRelationship` 取句柄（Java 的 `rels.getPackagePart()`）
    // 返回 `None` ⇔ Java 的 `getPackagePart() == null`（external 关系 / 无 part）
    pub fn of_relationship(rels: &mut OpcrRelationship) -> io::Result<Option<Self>> {
        match rels.getPackagePart() {
            Some(part) => Ok(Some(Self::of(part)?)),
            None => Ok(None),
        }
    }

    // java 每次使用 `part` 引用 → rust 每次重建真实 `OpcrPackagePart`
    pub fn part(&self) -> OpcrPackagePart {
        OpcrPackagePart::OpcrPackagePart(&self.mZipFile, &self.mZipEntry)
    }

    // java part.getPartName()
    pub fn getPartName(&self) -> String {
        self.part().getPartName()
    }

    // java part.getInputStream() / getInputStream(false)
    pub fn getInputStream(&self, isOriginInput: bool) -> io::Result<Box<dyn std::io::Read>> {
        self.part().getInputStream(isOriginInput)
    }

    // java part.getInputStream()
    pub fn getInputStream_default(&self) -> io::Result<Box<dyn std::io::Read>> {
        self.part().getInputStream_default()
    }

    // java part.getRelationships()
    // rust: kfc 的 `getRelationships` 返回借用；端口返回**拥有值的真实** `OpcrRelationships`
    // （用 kfc 自己的 `OpcrRelationships::OpcrRelationships(&zipFile, partName)` 构造，
    //  它读的是包里真实的 `_rels/*.rels` 条目）
    pub fn getRelationships(&self) -> io::Result<OpcrRelationships> {
        let part_name = self.getPartName();
        let mut part = self.part();
        // java 语义等价：先走 kfc 自己的懒加载路径，再取拥有值
        let _ = part.getRelationships()?;
        OpcrRelationships::OpcrRelationships(&self.mZipFile, &part_name)
    }

    // rust: 真实读取整段 part 字节（Java `InputStream` 全量读）
    pub fn read_all(&self) -> io::Result<Vec<u8>> {
        use std::io::Read;
        let mut stream = self.getInputStream_default()?;
        let mut bytes = Vec::new();
        stream.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    pub fn zipFile(&self) -> ZipFile {
        self.mZipFile.clone()
    }
    pub fn zipEntry(&self) -> ZipEntry {
        self.mZipEntry.clone()
    }
}

/// `OpcrRelationships` 上按 `relsType` 取 part 句柄（Java `relses.getRelationshipByType(t).getPackagePart()`）。
///
/// `rust:` `OpcrRelationship::getPackagePart(&mut self)` 需要独占借用，而
/// `OpcrRelationships::getRelationshipByType(&self)` 只给共享借用，
/// 故端口按 kfc 自己的索引访问器（`getRelationship_mut(index)`）取到 `&mut` 关系。
/// 语义与 Java 一致：找不到 type ⇒ `None`；找到但无 part ⇒ `None`。
pub fn relationship_part_by_type(
    relses: &mut OpcrRelationships,
    rels_type: &str,
) -> io::Result<Option<OpcrPackagePartRef>> {
    // java OpcrRelationship rels = relses.getRelationshipByType(relsType);
    let mut index = -1i32;
    for i in 0..relses.size() {
        let t = relses.getRelationship(i).getType().unwrap_or_default();
        if t == rels_type {
            index = i;
            break;
        }
    }
    // java if (rels == null) return null;
    if index < 0 {
        return Ok(None);
    }
    // java rels.getPackagePart()
    let rel = relses.getRelationship_mut(index);
    OpcrPackagePartRef::of_relationship(rel)
}

/// `OpcrRelationships` 上按 `rId` 取 part 句柄（Java `relses.getRelationship(rid).getPackagePart()`）。
pub fn relationship_part_by_rid(
    relses: &mut OpcrRelationships,
    rid: &str,
) -> io::Result<Option<OpcrPackagePartRef>> {
    let mut index = -1i32;
    for i in 0..relses.size() {
        let id = relses.getRelationship(i).getId().unwrap_or_default();
        if id == rid {
            index = i;
            break;
        }
    }
    if index < 0 {
        return Ok(None);
    }
    let rel = relses.getRelationship_mut(index);
    OpcrPackagePartRef::of_relationship(rel)
}

/// `OpcrRelationships` 上按 `relsType` 取关系的真实字段
/// （Java 直接持有 `OpcrRelationship` 引用的地方，端口取它被用到的真实字段）。
#[derive(Clone, Default)]
pub struct OpcrRelationshipData {
    pub id: String,
    pub part_name: String,
    pub rels_type: String,
    pub target: String,
    pub part: Option<OpcrPackagePartRef>,
}

impl OpcrRelationshipData {
    // rust: 从真实关系提取（Java 的 `rels.getXxx()` 逐项）
    pub fn of(relses: &mut OpcrRelationships, rels_type: &str) -> io::Result<Option<Self>> {
        let mut index = -1i32;
        for i in 0..relses.size() {
            let t = relses.getRelationship(i).getType().unwrap_or_default();
            if t == rels_type {
                index = i;
                break;
            }
        }
        if index < 0 {
            return Ok(None);
        }
        let rel = relses.getRelationship_mut(index);
        let id = rel.getId().unwrap_or_default();
        let part_name = rel.getPartName();
        let rels_type = rel.getType().unwrap_or_default();
        let target = rel.getTarget().unwrap_or_default();
        let part = OpcrPackagePartRef::of_relationship(rel)?;
        Ok(Some(Self {
            id,
            part_name,
            rels_type,
            target,
            part,
        }))
    }
}
