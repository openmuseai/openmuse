#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::cell::RefCell;
use std::io;
use std::rc::Rc;

// Java: import cn.wps.kfc.ooxfr.OxfPptRelations
// rust: 端口里 `OxfPptRelations` 是模块（表项是模块级自由函数），与 Java 的静态字段一一对应
use kfc_kfc_alpha::cn::wps::kfc::ooxfr::OxfPptRelations;
// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrPackagePartRef;
// Java: import cn.wps.kfc.opcr.OpcrRelationship
// Java: import cn.wps.kfc.opcr.OpcrRelationships
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.show.PptFileCache
// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::KmoPresentation;
// Java: import cn.wps.show.data.Presentation
// Java: import java.io.IOException
use super::PptxrItemProps::PptxrItemProps;
use crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::relationship_part_by_type;

// java public class PptxrItem
pub struct PptxrItem {
    // java private KmoPresentation mPres;
    mPres: Rc<RefCell<KmoPresentation>>,
    // java private OpcrPackagePart mPart;
    mPart: Option<OpcrPackagePartRef>,
}

// java public class PptxrItem
impl PptxrItem {
    // java public PptxrItem(KmoPresentation pres, OpcrPackagePart part)
    pub fn PptxrItem(pres: Rc<RefCell<KmoPresentation>>, part: Option<OpcrPackagePartRef>) -> Self {
        // java this.mPres = pres;
        // java this.mPart = part;
        Self {
            mPres: pres,
            mPart: part,
        }
    }

    // java public void open() throws IOException
    pub fn open(&mut self) -> io::Result<()> {
        // java if (null != this.mPart)
        if let Some(part) = self.mPart.clone() {
            // java Presentation presentationData = Presentation.open(this.mPres.protocol());
            // java File file = PptFileCache.dumpTempFile(this.mPart);
            // rust: `cn.wps.show.PptFileCache` 在 default feature 下不可达
            // （`show/mod.rs` 里 `pub mod PptFileCache;` 被注释），端口用真实读流 + 真实落盘实现同一观测。
            let path = dumpTempFile(&part)?;
            // java String path = file.getAbsolutePath();
            // java this.mPres.items().add(path);
            self.mPres.borrow_mut().items_mut().push(path);
            // java this.mPres.updateProtocol(presentationData.save());
            // java OpcrRelationships relses = this.mPart.getRelationships();
            let mut relses = part.getRelationships()?;
            // java OpcrRelationship itemProps = relses.getRelationshipByType(OxfPptRelations.ITEMPROPS.getRelsType());
            let itemProps = relationship_part_by_type(
                &mut relses,
                &OxfPptRelations::ITEMPROPS().getRelsType(),
            )?;
            // java if (null != itemProps)
            if let Some(itemProps) = itemProps {
                // java PptxrItemProps itemPropsReader = new PptxrItemProps(this.mPres, itemProps.getPackagePart());
                let mut itemPropsReader =
                    PptxrItemProps::PptxrItemProps(self.mPres.clone(), Some(itemProps));
                // java itemPropsReader.open();
                itemPropsReader.open()?;
            }
        }
        Ok(())
    }
}

/// `java cn.wps.show.PptFileCache.dumpTempFile(OpcrPackagePart)` 的端口
/// （真实读 part 流 → 真实落盘到临时目录 → 返回绝对路径）。
pub fn dumpTempFile(part: &OpcrPackagePartRef) -> io::Result<String> {
    let bytes = part.read_all()?;
    let name = part.getPartName();
    let base = name.rsplit('/').next().unwrap_or("part").to_string();
    let dir = std::env::temp_dir().join("ppt_core_customxml");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}_{}", bytes.len(), base));
    std::fs::write(&path, &bytes)?;
    Ok(path.to_string_lossy().into_owned())
}
