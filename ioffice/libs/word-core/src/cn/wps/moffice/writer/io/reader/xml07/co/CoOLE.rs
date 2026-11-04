// Java: import cn.wps.moffice.drawing.OLE
use drawing::cn::wps::moffice::drawing::OLE::*;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::*;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
use drawing::cn::wps::moffice::drawing::media::IMediaLib::*;
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::*;
// Java: import cn.wps.moffice.writer.VersionState
use crate::cn::wps::moffice::writer::VersionState::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.Xml07Context
use crate::cn::wps::moffice::writer::io::reader::xml07::Xml07Context::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.pi.IProcInfo
use crate::cn::wps::moffice::writer::io::reader::xml07::pi::IProcInfo::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.pi.PiOLE
use crate::cn::wps::moffice::writer::io::reader::xml07::pi::PiOLE::*;
// Java: import cn.wps.moffice.writer.io.reader.xml07.co.CoreObject
use crate::cn::wps::moffice::writer::io::reader::xml07::co::CoreObject::*;

// java public class CoOLE extends CoreObject
#[derive(Default)]
pub struct CoOLE {
    // java CoreObject super_
    pub(crate) super_: CoreObject,
    // java private Shape mTarget;
    mTarget: Option<Shape>,
    // java private String mProgId;
    mProgId: Option<String>,
}

// java public class CoOLE extends CoreObject
impl CoOLE {
    // java public  CoOLE(Shape target, String progId)
    pub fn CoOLE(target: &Shape, progId: &str) -> Self {
      // java this.mTarget = target;
      Self { super_: CoreObject::new(), mTarget: target, mProgId: progId }
      // java this.mProgId = progId;
    }

    // java public void process(Xml07Context ctxt, IProcInfo info)
    pub fn process(&mut self, ctxt: Option<&Xml07Context>, info: Option<&dyn IProcInfo>) {
      // java if (null != this.mTarget && null != ctxt && null != info)
      if let (Some(mTarget), Some(ctxt), Some(info)) = (self.mTarget.as_mut(), ctxt, info) {
        // java PiOLE piOLE = (PiOLE)info;
        let mut piOLE: PiOLE = (info).downcast_ref::<PiOLE>().unwrap();
        // java if (null != piOLE.mRawData)
        if piOLE.mRawData.is_some() {
          // java IMediaLib mediaLib = ctxt.getDocument().getMediaLib();
          let mut mediaLib: Box<dyn IMediaLib> = ctxt.getDocument().getMediaLib();
          // java String path = piOLE.mRawData;
          let mut path: String = piOLE.mRawData;
          // java int mediaId = mediaLib.addMedia(path, path, MediaTypeEnum.OLE);
          let mut mediaId: i32 = mediaLib.addMedia(path, path, MediaTypeEnum::OLE);
          // java OLE ole;
          let mut ole: OLE = OLE::default();
          // java if (VersionState.isProVersion())
          if VersionState::isProVersion() {
            // java ole = new OLE(mediaId, mediaLib);
            ole = OLE::new(mediaId, mediaLib);
          }
          // java else
          else {
            // java ole = new OLE(mediaId);
            ole = OLE::new(mediaId);
          }
          // java ole.setProgId(this.mProgId);
          ole.setProgId(self.mProgId);
          // java this.mTarget.setOLE(ole);
          mTarget.setOLE(Some(&mut ole));
        }
      }
    }

    // java public int getType()
    pub fn getType(&self) -> i32 {
      // java return 3;
      return 3;
    }

}

impl std::ops::Deref for CoOLE {
    type Target = CoreObject;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CoOLE {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
