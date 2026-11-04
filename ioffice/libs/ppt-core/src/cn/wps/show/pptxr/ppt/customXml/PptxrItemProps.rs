#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

use std::cell::RefCell;
use std::io;
use std::rc::Rc;

// Java: import cn.wps.kfc.opcr.OpcrPackagePart
use crate::cn::wps::show::pptxr::util::OpcrPackagePartRef::OpcrPackagePartRef;
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.show.PptFileCache
// Java: import cn.wps.show.app.KmoPresentation
use crate::cn::wps::show::util::layout_shape::KmoPresentation;
// Java: import cn.wps.show.data.Presentation
// Java: import java.io.IOException
use super::PptxrItem::dumpTempFile;

// java public class PptxrItemProps
pub struct PptxrItemProps {
    // java private KmoPresentation mPres;
    mPres: Rc<RefCell<KmoPresentation>>,
    // java private OpcrPackagePart mPart;
    mPart: Option<OpcrPackagePartRef>,
}

// java public class PptxrItemProps
impl PptxrItemProps {
    // java public PptxrItemProps(KmoPresentation pres, OpcrPackagePart part)
    pub fn PptxrItemProps(pres: Rc<RefCell<KmoPresentation>>, part: Option<OpcrPackagePartRef>) -> Self {
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
            // java File file = PptFileCache.dumpTempFile(this.mPart);
            let path = dumpTempFile(&part)?;
            // java if (file != null) { String path = file.getAbsolutePath(); this.mPres.itemProps().add(path); }
            self.mPres.borrow_mut().itemProps_mut().push(path);
            // java Presentation presentationData = Presentation.open(this.mPres.protocol());
            // java this.mPres.updateProtocol(presentationData.save());
        }
        Ok(())
    }
}
