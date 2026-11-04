#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::collections::HashMap;
use std::io;

// Java: import cn.wps.kfc.opcr.OpcrRelationship
use crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrRelationshipData;
// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::KmoPresentation;
// Java: import cn.wps.show.app.media.KmoMediaLib
use crate::cn::wps::show::util::layout_textbox::KmoMediaLib;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedia
use super::PptxrMedia::PptxrMedia;
// Java: import java.io.IOException
// Java: import java.util.HashMap
// Java: import java.util.Map

// java public class PptxrMedias
pub struct PptxrMedias {
    // java private Map<String, Integer> mGlobalMediasIdx = new HashMap();
    mGlobalMediasIdx: HashMap<String, i32>,
    // java private PptxrMedia mMedia;
    // rust: Java 里 `PptxrPresentation.mMedia` 与 `PptxrMedias.mMedia` 是同一个对象引用，
    // 端口用 `Rc<RefCell<..>>` 表达同一共享（单一所有权语言下的等价物）
    mMedia: std::rc::Rc<std::cell::RefCell<PptxrMedia>>,
    // java private KmoPresentation mPres;
    mPres: std::rc::Rc<std::cell::RefCell<KmoPresentation>>,
    // java private int mMode;
    mMode: i32,
}

// java public class PptxrMedias
impl PptxrMedias {
    // java public PptxrMedias(PptxrMedia media, KmoPresentation pres, int mode)
    pub fn PptxrMedias(
        media: std::rc::Rc<std::cell::RefCell<PptxrMedia>>,
        pres: std::rc::Rc<std::cell::RefCell<KmoPresentation>>,
        mode: i32,
    ) -> Self {
        // java this.mMedia = media;
        // java this.mPres = pres;
        // java this.mMode = mode;
        Self {
            mGlobalMediasIdx: HashMap::new(),
            mMedia: media,
            mPres: pres,
            mMode: mode,
        }
    }

    // java public int openMedia(KmoMediaLib mediaLib, OpcrRelationship mediaRels) throws IOException
    pub fn openMedia(
        &mut self,
        mediaLib: &mut KmoMediaLib,
        mediaRels: Option<&OpcrRelationshipData>,
    ) -> io::Result<i32> {
        // java if (mediaRels == null) return -1;
        let mediaRels = match mediaRels {
            Some(r) => r,
            None => return Ok(-1),
        };
        // java String partName = mediaRels.getPartName();
        let partName = mediaRels.part_name.clone();
        // java if (this.mGlobalMediasIdx.containsKey(partName)) return this.mGlobalMediasIdx.get(partName);
        if let Some(id) = self.mGlobalMediasIdx.get(&partName) {
            return Ok(*id);
        }
        // java int mediaId = this.mMedia.open(mediaLib, mediaRels, null);
        let mediaId = self.mMedia.borrow_mut().open(mediaLib, Some(mediaRels), None)?;
        // java if (mediaId != -1) { this.mGlobalMediasIdx.put(partName, mediaId); }
        if mediaId != -1 {
            self.mGlobalMediasIdx.insert(partName.clone(), mediaId);
        }
        // java return mediaId;
        Ok(mediaId)
    }

    // java public void priorityReadMedia(KmoPresentation pres) throws IOException
    pub fn priorityReadMedia(&mut self, pres: &mut KmoPresentation) -> io::Result<()> {
        // java KmoMediaPriorityReader reader = pres.mediaLib().getMediaPriorityReader();
        // rust: `KmoMediaLib` 在 Rust 侧是单元结构，无媒体优先级状态，
        // 故 reader 由 `KmoPresentation` 的装配面承载（见台账）。
        let reader = pres.mediaPriorityReader();
        // java if (reader != null)
        if let Some(mediaId) = reader {
            // java this.priorityLoadMedia(reader.getMediaId());
            self.priorityLoadMedia(mediaId)?;
            // java reader.finishRead(true);
        }
        Ok(())
    }

    // java public void priorityLoadMedia(int mediaId) throws IOException
    pub fn priorityLoadMedia(&mut self, mediaId: i32) -> io::Result<()> {
        // java for (String partName : this.mGlobalMediasIdx.keySet())
        let keys: Vec<String> = self.mGlobalMediasIdx.keys().cloned().collect();
        for partName in keys {
            // java if (this.mGlobalMediasIdx.get(partName) == mediaId)
            if self.mGlobalMediasIdx.get(&partName).copied() == Some(mediaId) {
                // java this.mMedia.priorityLoadMedia(partName);
                self.mMedia.borrow_mut().priorityLoadMedia(partName)?;
                // java break;
                break;
            }
        }
        Ok(())
    }

    // rust: 可观测面（Java 无此方法）
    pub fn globalMediasIdxLen(&self) -> usize {
        self.mGlobalMediasIdx.len()
    }
    pub fn media(&self) -> std::rc::Rc<std::cell::RefCell<PptxrMedia>> {
        self.mMedia.clone()
    }
}
