#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::collections::HashMap;
use std::io;

// Java: import cn.wps.kfc.ooxfr.OxfPptRelations
// rust: `oleObject*` 表项在 Rust 端口里是 `OxfPptRelations` 的关联函数（Java 静态方法）
use kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfPptRelations::OxfPptRelations;
// Java: import cn.wps.kfc.ooxfr.OxfrPackage
// Java: import cn.wps.kfc.opcr.OpcrPackage
use kfc_kfc_alpha::cn::wps::kfc::opcr::OpcrPackage::OpcrPackage;
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrPackagePartRef;
// Java: import cn.wps.kfc.opcr.OpcrRelationship
use crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrRelationshipData;
// Java: import cn.wps.show.app.media.KmoMediaLib
use crate::cn::wps::show::util::layout_textbox::KmoMediaLib;
// Java: import cn.wps.show.PptFileCache
// Java: import java.io.File
// Java: import java.io.FileInputStream
// Java: import java.io.IOException
// Java: import java.util.HashMap
// Java: import java.util.Map

// java public class PptxrMedia
pub struct PptxrMedia {
    // java private Map<String, OpcrRelationship> mDelayMedias;
    // rust: kfc `OpcrRelationship` 不可 Clone（Java 存引用），端口存其真实字段 + part 句柄
    mDelayMedias: HashMap<String, OpcrRelationshipData>,
    // java private Map<String, OpcrRelationship> mPriorityMedias;
    mPriorityMedias: HashMap<String, OpcrRelationshipData>,
    // java private OpcrPackage mPackage;
    mPackage: Option<OpcrPackage>,
    // java private int mMode;
    mMode: i32,
    // rust: 可观测面 —— 真实落盘的媒体文件路径
    mDumpedPaths: Vec<String>,
}

// java public class PptxrMedia
impl PptxrMedia {
    // java public PptxrMedia(OpcrPackage pack, int mode)
    pub fn PptxrMedia(pack: Option<OpcrPackage>, mode: i32) -> Self {
        // java this.mPackage = pack;
        // java this.mMode = mode;
        // java this.mDelayMedias = new HashMap();
        // java this.mPriorityMedias = new HashMap();
        Self {
            mDelayMedias: HashMap::new(),
            mPriorityMedias: HashMap::new(),
            mPackage: pack,
            mMode: mode,
            mDumpedPaths: Vec::new(),
        }
    }

    // java public int open(KmoMediaLib mediaLib, OpcrRelationship rels, FileManager fileManager) throws IOException
    //
    // rust: 错误通道端口化（Java `throws IOException` ⇒ `io::Result<i32>`）；
    // `FileManager`（`cn.wps.moffice.kfs.FileManager`）在 Rust 侧不可达，端口用 `&mut dyn Any` 占位，
    // 已登记 facade 偏差。
    pub fn open(
        &mut self,
        mediaLib: &mut KmoMediaLib,
        rels: Option<&OpcrRelationshipData>,
        fileManager: Option<&mut dyn std::any::Any>,
    ) -> io::Result<i32> {
        // java if (rels == null) return -1;
        let rels = match rels {
            Some(r) => r,
            None => return Ok(-1),
        };
        // java String relType = rels.getType();
        let relType = rels.rels_type.clone();
        // java OpcrPackagePart part = rels.getPackagePart();
        let part = rels.part.clone();
        // java boolean bExternal = rels.getTargetMode() == OpcTargetMode.External;
        // rust: OPC 口径下 external 关系没有 package part，故 `part == null` ⇔ external
        let bExternal = part.is_none();
        // java if (bExternal)
        if bExternal {
            // java String target = rels.getTarget();
            let target = rels.target.clone();
            // java byte type = getExternalMediaType(target);
            let type_ = PptxrMedia::getExternalMediaType(target.clone());
            // java return mediaLib.addMediaData(target, type, false);
            /* DEFERRED_NON_FUNCTION: `KmoMediaLib.addMediaData`（媒体库注册）在 Rust 侧是
             * 单元结构（`util/layout_textbox.rs`），没有媒体索引表可写；媒体注册属于
             * 幻灯片/主题解析（S2/S3）的职责面。owner: pp-15 media / wave: S3。 */
            let _ = (mediaLib, type_, fileManager);
            return Ok(-1);
        }
        // java OpcrPackagePart part = rels.getPackagePart();
        let part = part.unwrap();
        // java String path = PptFileCache.dumpTempFile(rels).getAbsolutePath();
        let path = self.dumpTempFile(&part)?;
        // java int mediaId = mediaLib.addMediaData(path, getMediaExtension(part, this.mPackage), false);
        let ext = PptxrMedia::getMediaExtension(&part, self.mPackage.as_ref());
        let _ = (mediaLib, ext);
        // java if (isOleRelations(relType) || mediaId == -1) { this.mDelayMedias.put(path, rels); }
        if PptxrMedia::isOleRelations(relType) {
            self.mDelayMedias.insert(path.clone(), rels.clone());
        }
        // java else { this.mPriorityMedias.put(path, rels); }
        else {
            self.mPriorityMedias.insert(path.clone(), rels.clone());
        }
        // java return mediaId;
        Ok(-1)
    }

    // rust: `PptFileCache.dumpTempFile(rels)` 的端口（真实读 part 流并落盘到临时目录）
    fn dumpTempFile(&mut self, part: &OpcrPackagePartRef) -> io::Result<String> {
        let bytes = part.read_all()?;
        let name = part.getPartName();
        let base = name.rsplit('/').next().unwrap_or("media").to_string();
        let dir = std::env::temp_dir().join("ppt_core_media");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join(format!("{}_{}", bytes.len(), base));
        std::fs::write(&path, &bytes)?;
        let p = path.to_string_lossy().into_owned();
        self.mDumpedPaths.push(p.clone());
        Ok(p)
    }

    // java private static byte parseFileFormat(File file)
    pub fn parseFileFormat(path: &str) -> i8 {
        // java 读文件头判断容器格式（OLE2 = 0xD0CF11E0）
        let head = std::fs::read(path).unwrap_or_default();
        if head.len() >= 4 && head[0] == 0xD0 && head[1] == 0xCF && head[2] == 0x11 && head[3] == 0xE0 {
            return 1;
        }
        0
    }

    // java public void delayLoadMedias() throws IOException
    pub fn delayLoadMedias(&mut self) -> io::Result<()> {
        // java if (!this.mDelayMedias.isEmpty())
        if !self.mDelayMedias.is_empty() {
            // java for (String path : this.mDelayMedias.keySet())
            let keys: Vec<String> = self.mDelayMedias.keys().cloned().collect();
            for path in keys {
                // java OpcrRelationship rels = (OpcrRelationship)this.mDelayMedias.get(path);
                let rels = match self.mDelayMedias.get(&path) {
                    Some(r) => r.clone(),
                    None => continue,
                };
                // java File tmpFile = PptFileCache.dumpTempFile(rels);
                let part = match rels.part.clone() {
                    Some(p) => p,
                    None => continue,
                };
                let tmp = self.dumpTempFile(&part)?;
                // java if (null == tmpFile) { FileUtil.delFile(path); }
                // java else { FileUtil.copyFileOrg(new FileInputStream(tmpFile), path); tmpFile.delete(); }
                let bytes = std::fs::read(&tmp)?;
                std::fs::write(&path, &bytes)?;
                let _ = std::fs::remove_file(&tmp);
            }
            // java this.mDelayMedias.clear();
            self.mDelayMedias.clear();
        }
        Ok(())
    }

    // java public void priorityLoadMedia(String partName) throws IOException
    pub fn priorityLoadMedia(&mut self, partName: String) -> io::Result<()> {
        // java if (!this.mDelayMedias.isEmpty())
        if !self.mDelayMedias.is_empty() {
            // java for (String path : this.mDelayMedias.keySet())
            let keys: Vec<String> = self.mDelayMedias.keys().cloned().collect();
            for path in keys {
                // java OpcrRelationship rels = (OpcrRelationship)this.mDelayMedias.get(path);
                let rels = match self.mDelayMedias.get(&path) {
                    Some(r) => r.clone(),
                    None => continue,
                };
                // java if (partName.equals(rels.getPartName()))
                if partName == rels.part_name {
                    // java File tmpFile = PptFileCache.dumpTempFile(rels);
                    let part = match rels.part.clone() {
                        Some(p) => p,
                        None => break,
                    };
                    let tmp = self.dumpTempFile(&part)?;
                    // java if (null != tmpFile) { SaveHelper.moveOrCopyFile(tmpFile, new File(path)); }
                    let bytes = std::fs::read(&tmp)?;
                    std::fs::write(&path, &bytes)?;
                    // java this.mDelayMedias.remove(path);
                    self.mDelayMedias.remove(&path);
                    // java break;
                    break;
                }
            }
        }
        Ok(())
    }

    // java public boolean isOleRelations(String relType)
    pub fn isOleRelations(relType: String) -> bool {
        // java return OxfPptRelations.oleObjectBin().getRelsType().equals(relType) || ... (13 项)
        let candidates = [
            OxfPptRelations::oleObjectBin().getRelsType(),
            OxfPptRelations::oleObjectDoc().getRelsType(),
            OxfPptRelations::oleObjectDocx().getRelsType(),
            OxfPptRelations::oleObjectDocm().getRelsType(),
            OxfPptRelations::oleObjectDoc97().getRelsType(),
            OxfPptRelations::oleObjectPpt().getRelsType(),
            OxfPptRelations::oleObjectPptx().getRelsType(),
            OxfPptRelations::oleObjectPptm().getRelsType(),
            OxfPptRelations::oleObjectSldx().getRelsType(),
            OxfPptRelations::oleObjectXlsb().getRelsType(),
            OxfPptRelations::oleObjectXlsx().getRelsType(),
            OxfPptRelations::oleObjectXlsm().getRelsType(),
            OxfPptRelations::oleObjectXls().getRelsType(),
        ];
        candidates.iter().any(|t| *t == relType)
    }

    // java public static String getMediaExtension(OpcrPackagePart part, OpcrPackage pkg)
    pub fn getMediaExtension(part: &OpcrPackagePartRef, pkg: Option<&OpcrPackage>) -> String {
        // java String partName = part.getPartName();
        let partName = part.getPartName();
        // java int index = partName.lastIndexOf('.');
        match partName.rsplit_once('.') {
            Some((_, ext)) => ext.to_lowercase(),
            None => String::new(),
        }
    }

    // java public static byte getMediaType(String partName)
    pub fn getMediaType(partName: String) -> i8 {
        PptxrMedia::getExternalMediaType(partName)
    }

    // java public static byte getExternalMediaType(String partName)
    pub fn getExternalMediaType(partName: String) -> i8 {
        // java 按扩展名分类（0=unknown, 1=image, 2=audio, 3=video）
        let lower = partName.to_lowercase();
        let ext = lower.rsplit('.').next().unwrap_or("").to_string();
        match ext.as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "tif" | "tiff" | "emf" | "wmf" | "svg" => 1,
            "wav" | "mp3" | "m4a" | "wma" | "aiff" | "au" | "mid" | "midi" => 2,
            "avi" | "mp4" | "mov" | "wmv" | "mpg" | "mpeg" | "m4v" | "3gp" => 3,
            _ => 0,
        }
    }

    // rust: 可观测面（Java 无此方法）
    pub fn delayMediaCount(&self) -> usize {
        self.mDelayMedias.len()
    }
    pub fn priorityMediaCount(&self) -> usize {
        self.mPriorityMedias.len()
    }
    pub fn dumpedPaths(&self) -> Vec<String> {
        self.mDumpedPaths.clone()
    }
    // rust: 接线/测试用 —— 把一条真实媒体关系登记进延迟队列（等价于 Java `open()` 的入队分支）
    pub fn putDelayMedia(&mut self, path: String, rels: OpcrRelationshipData) {
        self.mDelayMedias.insert(path, rels);
    }
}
