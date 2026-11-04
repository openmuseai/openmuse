#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

// Java: import cn.wps.base.log.IOLog
use compile_surface::IOLog;
// Java: import cn.wps.drawing.app.DgStore
use compile_surface::DgStore;
// Java: import cn.wps.drawing.app.diagrams.DiagramPair
use compile_surface::DiagramPair;
// Java: import cn.wps.drawing.data.blip.Blip
use compile_surface::Blip;
// Java: import cn.wps.drawing.data.hyperlink.Hyperlink
use compile_surface::Hyperlink;
// Java: import cn.wps.drawing.data.media.Media
use compile_surface::{AudioFile, EmbeddedWAVAudioFile, Media, QuickTimeFile, VideoFile};
// Java: import cn.wps.drawing.data.model3d.Model3D
use compile_surface::Model3D;
// Java: import cn.wps.drawing.data.shape.Shape
use compile_surface::Shape;
// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::{HandlerCallback__mode, HandlerCallbackState};
// Java: import cn.wps.kfc.ooxfr.OxfPptRelations
use compile_surface::OxfPptRelations;
// Java: import cn.wps.kfc.opcr.OpcrRelationship
use compile_surface::OpcrRelationship;
// Java: import cn.wps.kfc.opcr.OpcrRelationships
use compile_surface::OpcrRelationships;
// Java: import cn.wps.log.KmoLog
use compile_surface::KmoLog;
// Java: import cn.wps.moffice.kfs.zip.ZipEntry
use compile_surface::ZipEntry;
// Java: import cn.wps.moffice.kfs.zip.ZipFile
use compile_surface::ZipFile;
// Java: import cn.wps.show.app.Ink.KmoInkLib
use compile_surface::KmoInkLib;
// Java: import cn.wps.show.app.media.KmoMediaData
use compile_surface::KmoMediaData;
// Java: import cn.wps.show.app.media.KmoMediaLib
use compile_surface::KmoMediaLib;
// Java: import cn.wps.show.app.media.KmoMediaUtil
use compile_surface::KmoMediaUtil;
// Java: import cn.wps.show.clipxr.ppt14.slides.Ppt14rSlides
use compile_surface::Ppt14rSlides;
// Java: import cn.wps.show.data.custdatalst.CustDataLst
use compile_surface::TagLst;
// Java: import cn.wps.show.pptxr.ppt.media.PptxrMedias
use compile_surface::PptxrMedias;
// Java: import cn.wps.show.pptxr.ppt.tags.PptxrTags
use compile_surface::PptxrTags;
// Java: import cn.wps.util.manager.FileManager
use compile_surface::FileManager;
// Java: import com.hp.hpl.inkml.InkModel
use compile_surface::InkModel;
// Java: import java.io.IOException
use compile_surface::IOException;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Map
// Java: import cn.wps.show.pptxr.ppt.slides.PptxrSlides
use compile_surface::PptxrSlides;
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

pub mod compile_surface {
    use std::cell::Cell;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct IOLog;
    impl IOLog {
        pub fn d__string_string_throwable(_tag: Option<&str>, _m: &str, _e: ()) {}
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct DgStore;
    impl DgStore {
        pub fn query(&self, _id: &i32) -> () {}
        pub fn update(&mut self, _id: &i32, _bin: ()) {}
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct DiagramPair {
        pub colorsId: String,
        pub quickId: String,
        pub layoutId: String,
        pub dataId: String,
    }
    impl DiagramPair {
        pub fn new(colorsId: String, quickId: String, layoutId: String, dataId: String) -> Self {
            Self { colorsId, quickId, layoutId, dataId }
        }
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Blip {
        pub embedMediaID: Option<i32>,
        pub linkMediaID: Option<i32>,
    }
    impl Blip {
        pub fn putEmbedMediaID(&mut self, value: i32) { self.embedMediaID = Some(value); }
        pub fn putLinkMediaID(&mut self, value: i32) { self.linkMediaID = Some(value); }
    }
    #[derive(Clone, Debug, Default)]
    pub struct Hyperlink;
    impl Hyperlink {
        pub fn putTarget(&mut self, _t: String) {}
        pub fn putTargetInternalMode(&mut self, _b: bool) {}
        pub fn putType(&mut self, _t: String) {}
        pub fn putAction(&mut self, _a: String) {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct Media;
    impl Media {
        pub fn hasAudioFile(&self) -> bool { false }
        pub fn hasVideoFile(&self) -> bool { false }
        pub fn audioFile(&mut self) -> &mut AudioFile { unimplemented!() }
        pub fn videoFile(&mut self) -> &mut VideoFile { unimplemented!() }
    }
    #[derive(Clone, Debug, Default)]
    pub struct AudioFile;
    impl AudioFile {
        pub fn putMediaID(&mut self, _id: i32) {}
        pub fn mediaID(&self) -> i32 { 0 }
    }
    #[derive(Clone, Debug, Default)]
    pub struct QuickTimeFile;
    impl QuickTimeFile {
        pub fn putMediaID(&mut self, _id: i32) {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct VideoFile;
    impl VideoFile {
        pub fn putMediaID(&mut self, _id: i32) {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct EmbeddedWAVAudioFile;
    impl EmbeddedWAVAudioFile {
        pub fn putMediaID(&mut self, _id: i32) {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct Model3D;
    impl Model3D {
        pub fn putEmbedMediaID(&mut self, _id: i32) {}
    }
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Shape;
    impl Shape {
        pub fn open(_bin: ()) -> Self { Self::default() }
        pub fn hasStCxn(&self) -> bool { false }
        pub fn hasEndCxn(&self) -> bool { false }
        pub fn stCxn(&mut self) -> Cxn {
            Cxn { id: 0 }
        }
        pub fn endCxn(&mut self) -> Cxn {
            Cxn { id: 0 }
        }
        pub fn save(&self) {}
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Cxn { pub id: i32 }
    impl Cxn {
        pub fn id(&self) -> i32 { self.id }
        pub fn putId(&mut self, _id: i32) {}
    }
    #[derive(Clone, Debug, Default)]
    pub struct OxfPptRelations;
    #[derive(Clone, Debug, Default)]
    pub struct OpcrRelationship {
        pub id: String,
    }
    impl OpcrRelationship {
        pub fn getId(&self) -> String { self.id.clone() }
        pub fn getPackagePart(&self) -> () {}
        pub fn getTarget(&self) -> String { String::new() }
        pub fn getType(&self) -> String { String::new() }
        pub fn getTargetMode(&self) -> i32 { 0 }
    }
    #[derive(Clone, Debug, Default)]
    pub struct OpcrRelationships {
        pub rels: HashMap<String, OpcrRelationship>,
    }
    impl OpcrRelationships {
        pub fn getRelationship(&self, id: &str) -> Option<OpcrRelationship> {
            self.rels.get(id).cloned()
        }
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct KmoLog;
    impl KmoLog {
        pub fn logPoint(_msg: &str) {}
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct ZipEntry;
    #[derive(Clone, Copy, Debug, Default)]
    pub struct ZipFile;
    #[derive(Clone, Copy, Debug, Default)]
    pub struct KmoInkLib;
    #[derive(Clone, Copy, Debug, Default)]
    pub struct KmoMediaData;
    impl KmoMediaData {
        pub fn getData(&self) -> Option<()> { None }
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct KmoMediaLib;
    impl KmoMediaLib {
        pub fn getMediaData(&self, _id: i32) -> Option<KmoMediaData> { None }
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct KmoMediaUtil;
    impl KmoMediaUtil {
        pub fn getMediaType(_ext: String) -> u8 { 0 }
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Ppt14rSlides;
    impl Ppt14rSlides {
        pub fn getSlideIdByFileName(_f: String) -> i64 { 0 }
    }
    #[derive(Clone, Debug, Default)]
    pub struct TagLst;
    #[derive(Clone, Debug, Default)]
    pub struct PptxrMedias {
        pub open_count: Rc<Cell<i32>>,
        pub last_id: i32,
    }
    impl PptxrMedias {
        pub fn openMedia(&self, _mediaLib: Option<KmoMediaLib>, _mediaRels: &OpcrRelationship) -> i32 {
            let n = self.open_count.get() + 1;
            self.open_count.set(n);
            n
        }
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct PptxrTags;
    impl PptxrTags {
        pub fn new(_t: TagLst, _part: ()) -> Self { Self }
        pub fn open(&self) {}
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct FileManager;
    #[derive(Clone, Copy, Debug, Default)]
    pub struct InkModel;
    #[derive(Clone, Copy, Debug, Default)]
    pub struct IOException;
    #[derive(Clone, Copy, Debug, Default)]
    pub struct PptxrSlides;
    impl PptxrSlides {
        pub fn getRelationShipTargetFileName(_r: OpcrRelationship) -> String { String::new() }
        pub fn getSlideIdByFileName(_f: String) -> i64 { 0 }
        pub fn getSlideIdByFileNameNotAdd(&self, _f: String) -> i64 { 0 }
    }
    // ★W6c-1 去假面（形态 G）：此处原内联重声明 `pub struct SlideHandlerCallback { pub mode: i32 }`，
    //   把 `ppt/slides/SlideHandlerCallback.rs`（真身，Java 9 行）遮蔽成局部类型。已删除。
    //   如本文件需要该类型，请 `use super::SlideHandlerCallback::SlideHandlerCallback;`。
}

// java private static final String TAG = null;
const TAG: Option<&str> = None;

// java public class HandlerCallbackImpl extends HandlerCallback
#[derive(Default)]
pub struct HandlerCallbackImpl {
    // java HandlerCallback super_
    pub(crate) super_: HandlerCallbackState,
    // java private DgStore dgStore;
    dgStore: Option<DgStore>,
    // java private KmoMediaLib mediaLib;
    mediaLib: Option<KmoMediaLib>,
    // java private KmoInkLib inkLib;
    inkLib: Option<KmoInkLib>,
    // java private OpcrRelationships relationships;
    relationships: Option<OpcrRelationships>,
    // java private HashMap<Integer, Integer> shapeIdMap;
    shapeIdMap: HashMap<i32, i32>,
    // java private List<Integer> connectorShapeIdList;
    connectorShapeIdList: Option<Vec<i32>>,
    // java Map<String, Shape> chartMap = null;
    pub(crate) chartMap: HashMap<String, Shape>,
    // java Map<Integer, DiagramPair> diagrams = null;
    pub(crate) diagrams: HashMap<i32, DiagramPair>,
    // java private HashMap<String, Integer> mRidPicMediaIds;
    mRidPicMediaIds: Option<HashMap<String, i32>>,
    // java private PptxrMedias mMedias;
    mMedias: Option<PptxrMedias>,
    // java private PptxrSlides mSlides;
    mSlides: Option<PptxrSlides>,
}

// java public class HandlerCallbackImpl extends HandlerCallback
impl HandlerCallbackImpl {
    // java public  HandlerCallbackImpl(DgStore dgStore, KmoMediaLib mediaLib, KmoInkLib inkLib, OpcrRelationships relationships, PptxrMedias medias, PptxrSlides slides, int mode)
    pub fn HandlerCallbackImpl(dgStore: Option<DgStore>, mediaLib: Option<KmoMediaLib>, inkLib: Option<KmoInkLib>, relationships: Option<OpcrRelationships>, medias: Option<PptxrMedias>, slides: Option<PptxrSlides>, mode: i32) -> Self {
      // java super(mode);
      // java this.dgStore = dgStore;
      Self { super_: HandlerCallback__mode(mode), dgStore: dgStore, mediaLib: mediaLib, inkLib: inkLib, relationships: relationships, shapeIdMap: HashMap::new(), connectorShapeIdList: Default::default(), chartMap: HashMap::new(), diagrams: HashMap::new(), mRidPicMediaIds: Default::default(), mMedias: medias, mSlides: slides }
      // java this.mediaLib = mediaLib;
      // java this.inkLib = inkLib;
      // java this.relationships = relationships;
      // java this.shapeIdMap = new HashMap();
      // java this.chartMap = new HashMap();
      // java this.diagrams = new HashMap();
      // java this.mMedias = medias;
      // java this.mSlides = slides;
    }

    // java public void load()
    pub fn load(&mut self) {
      // java this.updateConnectorSite();
      self.updateConnectorSite();
    }

    // java private void updateConnectorSite()
    pub fn updateConnectorSite(&mut self) {
      // java if (this.connectorShapeIdList != null && this.dgStore != null)
      if self.connectorShapeIdList.is_some() && self.dgStore.is_some() {
        let ids = self.connectorShapeIdList.as_ref().unwrap().clone();
        // java for (Integer connectorShapeId : this.connectorShapeIdList)
        for connectorShapeId in ids.iter() {
          // java Shape connector = Shape.open(this.dgStore.query(connectorShapeId));
          let mut connector: Shape = Shape::open(self.dgStore.as_ref().unwrap().query(connectorShapeId));
          // java if (connector.hasStCxn())
          if connector.hasStCxn() {
            // java int originShapeId = connector.stCxn().id();
            let mut originShapeId: i32 = connector.stCxn().id();
            // java connector.stCxn().putId(this.getCoreShapeId(originShapeId));
            connector.stCxn().putId(self.getCoreShapeId(originShapeId));
          }
          // java if (connector.hasEndCxn())
          if connector.hasEndCxn() {
            // java int originShapeId = connector.endCxn().id();
            let mut originShapeId: i32 = connector.endCxn().id();
            // java connector.endCxn().putId(this.getCoreShapeId(originShapeId));
            connector.endCxn().putId(self.getCoreShapeId(originShapeId));
          }
          // java this.dgStore.update(connectorShapeId, connector.save());
          self.dgStore.as_mut().unwrap().update(connectorShapeId, connector.save());
        }
      }
    }

    // java public void onTags(String id, CustDataLst.TagLst tagLst)
    pub fn onTags(&mut self, id: String, tagLst: TagLst) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OpcrRelationship relationship = this.relationships.getRelationship(id);
      let mut relationship: OpcrRelationship = self.relationships.getRelationship__string(id);
      // java if (null != relationship)
      if true {
        // java PptxrTags pptxrTags = new PptxrTags(tagLst, relationship.getPackagePart());
        let mut pptxrTags: PptxrTags = PptxrTags::new(tagLst, relationship.getPackagePart());
            // java pptxrTags.open();
            pptxrTags.open();
            // java IOException e
            todo!();
              // java IOLog.d(TAG, "IOException", e);
              IOLog::d__string_string_throwable(TAG, "IOException", e);
      }
      */
    }

    // java public void onBlipEmbed(String embed, Blip blip)
    pub fn onBlipEmbed(&mut self, embed: String, blip: &mut Blip) {
      // java if (null != embed && embed.length() != 0)
      if !embed.is_empty() {
        // java OpcrRelationship relationship = this.relationships.getRelationship(embed);
        let relationship: Option<OpcrRelationship> = self.relationships.as_ref().and_then(|r| r.getRelationship(&embed));
        // java if (null == relationship)
        if relationship.is_none() {
          // java KmoLog.logPoint("HandlerCallbackImpl.onBlipEmbed() -> relationship is null");
          KmoLog::logPoint("HandlerCallbackImpl.onBlipEmbed() -> relationship is null");
        }
        // java else
        else {
          let relationship = relationship.unwrap();
          // java int mediaId = this.mMedias.openMedia(this.mediaLib, relationship);
          let mediaId: i32 = self.mMedias.as_ref().map(|m| m.openMedia(self.mediaLib, &relationship)).unwrap_or(0);
          // java blip.putEmbedMediaID(mediaId);
          blip.putEmbedMediaID(mediaId);
          // java if (null == this.mRidPicMediaIds)
          if self.mRidPicMediaIds.is_none() {
            // java this.mRidPicMediaIds = new HashMap();
            self.mRidPicMediaIds = Some(HashMap::new());
          }
          // java this.mRidPicMediaIds.put(relationship.getId(), mediaId);
          self.mRidPicMediaIds.as_mut().unwrap().insert(relationship.getId(), mediaId);
        }
      }
      // java else
      else {
        // java KmoLog.logPoint("HandlerCallbackImpl.onBlipEmbed() -> embed is invalid");
        KmoLog::logPoint("HandlerCallbackImpl.onBlipEmbed() -> embed is invalid");
      }
    }

    // java public void onBlipLink(String link, Blip blip)
    pub fn onBlipLink(&mut self, link: String, blip: &mut Blip) {
      // java OpcrRelationship relationship = this.relationships.getRelationship(link);
      let relationship: Option<OpcrRelationship> = self.relationships.as_ref().and_then(|r| r.getRelationship(&link));
      // java if (null == relationship)
      if relationship.is_none() {
        // java KmoLog.logPoint("HandlerCallbackImpl.onBlipLink() -> relationship is null");
        KmoLog::logPoint("HandlerCallbackImpl.onBlipLink() -> relationship is null");
      }
      // java else
      else {
        let relationship = relationship.unwrap();
        // java int mediaId = this.mMedias.openMedia(this.mediaLib, relationship);
        let mediaId: i32 = self.mMedias.as_ref().map(|m| m.openMedia(self.mediaLib, &relationship)).unwrap_or(0);
        // java blip.putLinkMediaID(mediaId);
        blip.putLinkMediaID(mediaId);
        // java if (null == this.mRidPicMediaIds)
        if self.mRidPicMediaIds.is_none() {
          // java this.mRidPicMediaIds = new HashMap();
          self.mRidPicMediaIds = Some(HashMap::new());
        }
        // java this.mRidPicMediaIds.put(relationship.getId(), mediaId);
        self.mRidPicMediaIds.as_mut().unwrap().insert(relationship.getId(), mediaId);
      }
    }

    // java public void onAudioFileLink(String link, Media.AudioFile audioFile)
    pub fn onAudioFileLink(&mut self, link: String, audioFile: &mut AudioFile) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OpcrRelationship relationship = this.relationships.getRelationship(link);
      let mut relationship: OpcrRelationship = self.relationships.getRelationship__string(link);
      // java int mediaId = this.mMedias.openMedia(this.mediaLib, relationship);
      let mut mediaId: i32 = self.mMedias.openMedia(self.mediaLib, relationship);
      // java audioFile.putMediaID(mediaId);
      audioFile.putMediaID(mediaId);
      */
    }

    // java public void onExtMediaFileLink(String link, Media media)
    pub fn onExtMediaFileLink(&mut self, link: String, media: &mut Media) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (media.hasAudioFile())
      if media.hasAudioFile() {
        // java OpcrRelationship relationship = this.relationships.getRelationship(link);
        let mut relationship: OpcrRelationship = self.relationships.getRelationship__string(link);
        // java if (relationship == null)
        if false {
          // java return;
          return;
        }
        // java KmoMediaData mediaData = this.mediaLib.getMediaData(media.audioFile().mediaID());
        let mut mediaData: KmoMediaData = self.mediaLib.getMediaData(media.audioFile().mediaID());
        // java if (mediaData == null || mediaData.getData() == null)
        if false || mediaData.getData().is_none() {
          // java String fileExtension = relationship.getTarget().substring(relationship.getTarget().lastIndexOf(".") + 1);
          let mut fileExtension: String = relationship.getTarget().substring(relationship.getTarget().lastIndexOf(".") + 1);
          // java byte mediaType = KmoMediaUtil.getMediaType(fileExtension);
          let mut mediaType: u8 = KmoMediaUtil::getMediaType(fileExtension);
          // java if (mediaType > 15 && mediaType < 32)
          if mediaType > 15 && mediaType < 32 {
            // java int mediaId = this.mMedias.openMedia(this.mediaLib, relationship);
            let mut mediaId: i32 = self.mMedias.openMedia(self.mediaLib, relationship);
            // java media.audioFile().putMediaID(mediaId);
            media.audioFile().putMediaID(mediaId);
          }
        }
      }
      // java else if (media.hasVideoFile())
      else if media.hasVideoFile() {
        // java OpcrRelationship relationship = this.relationships.getRelationship(link);
        let mut relationship: OpcrRelationship = self.relationships.getRelationship__string(link);
        // java if (relationship == null)
        if false {
          // java return;
          return;
        }
        // java KmoMediaData mediaData = this.mediaLib.getMediaData(media.videoFile().mediaID());
        let mut mediaData: KmoMediaData = self.mediaLib.getMediaData(media.videoFile().mediaID());
        // java if (mediaData == null || mediaData.getData() == null)
        if false || mediaData.getData().is_none() {
          // java String fileExtension = relationship.getTarget().substring(relationship.getTarget().lastIndexOf(".") + 1);
          let mut fileExtension: String = relationship.getTarget().substring(relationship.getTarget().lastIndexOf(".") + 1);
          // java byte mediaType = KmoMediaUtil.getMediaType(fileExtension);
          let mut mediaType: u8 = KmoMediaUtil::getMediaType(fileExtension);
          // java if (mediaType > 33 && mediaType < 46)
          if mediaType > 33 && mediaType < 46 {
            // java int mediaId = this.mMedias.openMedia(this.mediaLib, relationship);
            let mut mediaId: i32 = self.mMedias.openMedia(self.mediaLib, relationship);
            // java media.videoFile().putMediaID(mediaId);
            media.videoFile().putMediaID(mediaId);
          }
        }
      }
      */
    }

    // java public void onQuickTimeFileLink(String link, Media.QuickTimeFile quickTimeFile)
    pub fn onQuickTimeFileLink(&mut self, link: String, quickTimeFile: &mut QuickTimeFile) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OpcrRelationship relationship = this.relationships.getRelationship(link);
      let mut relationship: OpcrRelationship = self.relationships.getRelationship__string(link);
      // java int mediaId = this.mMedias.openMedia(this.mediaLib, relationship);
      let mut mediaId: i32 = self.mMedias.openMedia(self.mediaLib, relationship);
      // java quickTimeFile.putMediaID(mediaId);
      quickTimeFile.putMediaID(mediaId);
      */
    }

    // java public void onVideoFileLink(String link, Media.VideoFile videoFile)
    pub fn onVideoFileLink(&mut self, link: String, videoFile: &mut VideoFile) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OpcrRelationship relationship = this.relationships.getRelationship(link);
      let mut relationship: OpcrRelationship = self.relationships.getRelationship__string(link);
      // java int mediaId = this.mMedias.openMedia(this.mediaLib, relationship);
      let mut mediaId: i32 = self.mMedias.openMedia(self.mediaLib, relationship);
      // java videoFile.putMediaID(mediaId);
      videoFile.putMediaID(mediaId);
      */
    }

    // java public void onEmbeddedWAVAudioFileEmbed(String embed, Media.EmbeddedWAVAudioFile embeddedWAVAudioFile)
    pub fn onEmbeddedWAVAudioFileEmbed(&mut self, embed: String, embeddedWAVAudioFile: &mut EmbeddedWAVAudioFile) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OpcrRelationship relationship = this.relationships.getRelationship(embed);
      let mut relationship: OpcrRelationship = self.relationships.getRelationship__string(embed);
      // java int mediaId = this.mMedias.openMedia(this.mediaLib, relationship);
      let mut mediaId: i32 = self.mMedias.openMedia(self.mediaLib, relationship);
      // java embeddedWAVAudioFile.putMediaID(mediaId);
      embeddedWAVAudioFile.putMediaID(mediaId);
      */
    }

    // java public void onHyperlinkRid(String rId, Hyperlink hyperlink)
    pub fn onHyperlinkRid(&mut self, rId: String, hyperlink: &mut Hyperlink) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OpcrRelationship relationship = this.relationships.getRelationship(rId);
      let mut relationship: OpcrRelationship = self.relationships.getRelationship__string(rId);
      // java if (relationship == null)
      if false {
        // java hyperlink.putTarget("");
        hyperlink.putTarget("");
      }
      // java else
      else {
        // java switch (relationship.getTargetMode())
        match relationship.getTargetMode() {
          // java case INTERNAL:
          INTERNAL => {
            // java if (relationship.getType().equals(OxfPptRelations.slide().getRelsType()))
            if relationship.getType().equals(OxfPptRelations::slide().getRelsType()) {
              // java String fileName = PptxrSlides.getRelationShipTargetFileName(relationship);
              let mut fileName: String = PptxrSlides::getRelationShipTargetFileName(relationship);
              // java long slideId = 0L;
              let mut slideId: i64 = 0;
              // java if (this.isPPTX())
              if self.super_.isPPTX() {
                // java if (null != this.mSlides)
                if let Some(mSlides) = self.mSlides.as_mut() {
                  // java slideId = this.mSlides.getSlideIdByFileNameNotAdd(fileName);
                  slideId = mSlides.getSlideIdByFileNameNotAdd(fileName);
                }
                // java else
                else {
                  // java slideId = PptxrSlides.getSlideIdByFileName(fileName);
                  slideId = PptxrSlides::getSlideIdByFileName(fileName);
                }
              }
              // java else if (this.isPPT14Slides())
              else if self.super_.isPPT14Slides() {
                // java slideId = Ppt14rSlides.getSlideIdByFileName(fileName);
                slideId = Ppt14rSlides::getSlideIdByFileName(fileName);
              }
              // java hyperlink.putTarget(Long.toString(slideId));
              hyperlink.putTarget(Long::toString(slideId));
            }
            // java else
            else {
              // java hyperlink.putTarget(relationship.getTarget());
              hyperlink.putTarget(relationship.getTarget());
            }
          }
          // java case EXTERNAL:
          EXTERNAL => {
            // java hyperlink.putTargetInternalMode(false);
            hyperlink.putTargetInternalMode(false);
            // java hyperlink.putTarget(relationship.getTarget());
            hyperlink.putTarget(relationship.getTarget());
          }
          // java default:
          _ => {
          }
        }
        // java hyperlink.putType(relationship.getType());
        hyperlink.putType(relationship.getType());
      }
      */
    }

    // java public void onHyperlinkAction(String action, Hyperlink hyperlink)
    pub fn onHyperlinkAction(&mut self, action: String, hyperlink: &mut Hyperlink) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java hyperlink.putAction(action);
      hyperlink.putAction(action);
      */
    }

    // java public void onShapeId(Integer originShapeId, Integer coreShapeId)
    pub fn onShapeId(&mut self, originShapeId: i32, coreShapeId: i32) {
      // java if (!this.shapeIdMap.containsKey(originShapeId))
      if !self.shapeIdMap.contains_key(&originShapeId) {
        // java this.shapeIdMap.put(originShapeId, coreShapeId);
        self.shapeIdMap.insert(originShapeId, coreShapeId);
      }
    }

    // java public Integer getCoreShapeId(Integer originShapeId)
    pub fn getCoreShapeId(&self, originShapeId: i32) -> i32 {
      // java Integer coreShapeId = (Integer)this.shapeIdMap.get(originShapeId);
      let coreShapeId: Option<i32> = self.shapeIdMap.get(&originShapeId).copied();
      // java return coreShapeId == null ? originShapeId : coreShapeId;
      return (if coreShapeId.is_none() { originShapeId } else { coreShapeId.unwrap() });
    }

    // java public void onConnectorShape(Integer connectorShapeId)
    pub fn onConnectorShape(&mut self, connectorShapeId: i32) {
      // java if (this.connectorShapeIdList == null)
      if self.connectorShapeIdList.is_none() {
        // java this.connectorShapeIdList = new ArrayList();
        self.connectorShapeIdList = Some(Vec::new());
      }
      // java this.connectorShapeIdList.add(connectorShapeId);
      self.connectorShapeIdList.as_mut().unwrap().push(connectorShapeId);
    }

    // java public int onOleObject(String rId)
    pub fn onOleObject(&mut self, rId: String) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java OpcrRelationship relationship = this.relationships.getRelationship(rId);
      let mut relationship: OpcrRelationship = self.relationships.getRelationship__string(rId);
      // java int mediaId = this.mMedias.openMedia(this.mediaLib, relationship);
      let mut mediaId: i32 = self.mMedias.openMedia(self.mediaLib, relationship);
      // java return mediaId;
      return mediaId;
      */
    }

    // java public void onChartRelationShip(String rId, Shape shape)
    pub fn onChartRelationShip(&mut self, rId: String, shape: Shape) {
      // java this.chartMap.put(rId, shape);
      self.chartMap.insert(rId, shape);
    }

    // java public Map<String, Shape> getChartMap()
    pub fn getChartMap(&self) -> &std::collections::HashMap<String, Shape> {
      // java return this.chartMap;
      return &self.chartMap;
    }

    // java public void addDiagramPair(int smartArtId, String colorsId, String dataId, String layoutId, String quickId)
    pub fn addDiagramPair(&mut self, smartArtId: i32, colorsId: String, dataId: String, layoutId: String, quickId: String) {
      // java DiagramPair diagramPair = new DiagramPair(colorsId, quickId, layoutId, dataId);
      let mut diagramPair: DiagramPair = DiagramPair::new(colorsId, quickId, layoutId, dataId);
      // java this.diagrams.put(smartArtId, diagramPair);
      self.diagrams.insert(smartArtId, diagramPair);
    }

    // java public Map<Integer, DiagramPair> getDiagramPairs()
    pub fn getDiagramPairs(&self) -> &std::collections::HashMap<i32, DiagramPair> {
      // java return this.diagrams;
      return &self.diagrams;
    }

    // java public int onInk(String rId, FileManager fileManager)
    pub fn onInk(&mut self, rId: String, fileManager: FileManager) -> i32 {
          todo!("DEFERRED_NON_FUNCTION");
          /* DEFERRED_NON_FUNCTION
          // java OpcrRelationship rel = this.relationships.getRelationship(rId);
          let mut rel: OpcrRelationship = self.relationships.getRelationship__string(rId);
          // java ZipFile.RAFStream inputStream = (ZipFile.RAFStream)rel.getPackagePart().getInputStream(true);
          let mut inputStream: ZipFile::RAFStream = { let __x: ZipFile::RAFStream = todo!("no align strategy matched"); __x };
          // java ZipEntry entry = rel.getPackagePart().getZipEntry();
          let mut entry: ZipEntry = rel.getPackagePart().getZipEntry();
          // java return this.inkLib.addInkModel(new InkModel(inputStream, "ink", entry, fileManager));
          return self.inkLib.addInkModel(InkModel::new(inputStream, "ink", entry, fileManager));
          // java IOException e
          todo!();
            // java throw new RuntimeException(e);
            panic!("{}", RuntimeException::new(e));
          */
    }

    // java public void onModel3D(String embed, Model3D model3D)
    pub fn onModel3D(&mut self, embed: String, model3D: &mut Model3D) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != embed && !embed.isEmpty())
      if true && !embed.isEmpty() {
        // java OpcrRelationship relationship = this.relationships.getRelationship(embed);
        let mut relationship: OpcrRelationship = self.relationships.getRelationship__string(embed);
        // java if (null != relationship)
        if true {
          // java int mediaId = this.mMedias.openMedia(this.mediaLib, relationship);
          let mut mediaId: i32 = self.mMedias.openMedia(self.mediaLib, relationship);
          // java model3D.putEmbedMediaID(mediaId);
          model3D.putEmbedMediaID(mediaId);
        }
      }
      */
    }

    // java public HashMap<String, Integer> getRidPicMediaIds()
    pub fn getRidPicMediaIds(&self) -> Option<std::collections::HashMap<String, i32>> {
      // java return this.mRidPicMediaIds;
      return self.mRidPicMediaIds.clone();
    }

}
