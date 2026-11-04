#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::{self, KoBin as KoBinTrait, Decode, Encode, Factory, Null};
// Java: import cn.wps.kso.kproto.KoClosable
use crate::cn::wps::kso::kproto::KoClosable::KoClosable;
// Java: import cn.wps.kso.kproto.KoZone
use crate::cn::wps::kso::kproto::KoZone::KoZone;
// Java: import cn.wps.kso.kproto.internal.KpBinBox
use crate::cn::wps::kso::kproto::internal::KpBinBox::KpBinBox;
// Java: import cn.wps.kso.kproto.internal.KpList
use crate::cn::wps::kso::kproto::internal::KpList::KpList;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import java.util.ArrayList
// Java: import java.util.Collection

// Java: public class Media implements KoAr, KoClosable
pub struct Media {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: AudioCD audioCD
    pub(crate) audioCD: Option<Box<dyn KoBinTrait>>,
    // Java field: AudioFile audioFile
    pub(crate) audioFile: Option<Box<dyn KoBinTrait>>,
    // Java field: VideoFile videoFile
    pub(crate) videoFile: Option<Box<dyn KoBinTrait>>,
    // Java field: MediaExtAttr mediaExtAttr
    pub(crate) mediaExtAttr: Option<Box<dyn KoBinTrait>>,
    // Java field: QuickTimeFile quickTimeFile
    pub(crate) quickTimeFile: Option<Box<dyn KoBinTrait>>,
    // Java field: EmbeddedWAVAudioFile embeddedWAVAudioFile
    pub(crate) embeddedWAVAudioFile: Option<Box<dyn KoBinTrait>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Media parent
    pub(crate) parent: Option<Box<Media>>,
}

// Java: public class AudioCD
pub struct AudioCD {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: AudioCDTime st
    pub(crate) st: Option<Box<dyn KoBinTrait>>,
    // Java field: AudioCDTime end
    pub(crate) end: Option<Box<dyn KoBinTrait>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: AudioCD parent
    pub(crate) parent: Option<Box<AudioCD>>,
}

// Java: public class AudioCDTime
pub struct AudioCDTime {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: AudioCDTime parent
    pub(crate) parent: Option<Box<AudioCDTime>>,
}

// Java: public class EmbeddedWAVAudioFile
pub struct EmbeddedWAVAudioFile {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: EmbeddedWAVAudioFile parent
    pub(crate) parent: Option<Box<EmbeddedWAVAudioFile>>,
}

// Java: public class AudioFile
pub struct AudioFile {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: AudioFile parent
    pub(crate) parent: Option<Box<AudioFile>>,
}

// Java: public class VideoFile
pub struct VideoFile {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: VideoFile parent
    pub(crate) parent: Option<Box<VideoFile>>,
}

// Java: public class QuickTimeFile
pub struct QuickTimeFile {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: QuickTimeFile parent
    pub(crate) parent: Option<Box<QuickTimeFile>>,
}

// Java: public class Bookmark
pub struct Bookmark {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Bookmark parent
    pub(crate) parent: Option<Box<Bookmark>>,
}

// Java: public class MediaExtAttr
pub struct MediaExtAttr {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: BookmarkLst bookmarkLst
    pub(crate) bookmarkLst: Option<Box<dyn KoBinTrait>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: MediaExtAttr parent
    pub(crate) parent: Option<Box<MediaExtAttr>>,
}

// Java: public class BookmarkLst
pub struct BookmarkLst {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
}

// Java: public class Media implements KoAr, KoClosable
impl Media {
    // Java: static protected static <Value1 extends Media> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Media::Media();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static Media create()
    pub fn create() -> Self {
        // Java: return initialize(Media.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static Media open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(Media.class, bin);
        Self::initialize(bin)
    }

    // Java: Media()
    pub fn Media() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        Media { zone: KoZone, audioCD: None, audioFile: None, videoFile: None, mediaExtAttr: None, quickTimeFile: None, embeddedWAVAudioFile: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: Media(KoBin bin)
    pub fn Media_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        Media { zone: KoZone, audioCD: None, audioFile: None, videoFile: None, mediaExtAttr: None, quickTimeFile: None, embeddedWAVAudioFile: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.map.load(this.box.get());
        let b = self.bin_box.get();
        self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(Media value)
    pub fn load_from(&mut self, value: Option<&Media>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.audioCD != null && this.audioCD.dirty())
        // Java: this.map.put(2, this.audioCD.saveBin(zone));
        // Java: if (this.audioFile != null && this.audioFile.dirty())
        // Java: this.map.put(3, this.audioFile.saveBin(zone));
        // Java: if (this.videoFile != null && this.videoFile.dirty())
        // Java: this.map.put(4, this.videoFile.saveBin(zone));
        // Java: if (this.mediaExtAttr != null && this.mediaExtAttr.dirty())
        // Java: this.map.put(5, this.mediaExtAttr.saveBin(zone));
        // Java: if (this.quickTimeFile != null && this.quickTimeFile.dirty())
        // Java: this.map.put(6, this.quickTimeFile.saveBin(zone));
        // Java: if (this.embeddedWAVAudioFile != null && this.embeddedWAVAudioFile.dirty())
        // Java: this.map.put(7, this.embeddedWAVAudioFile.saveBin(zone));
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(Media o)
    pub fn equalsLax(&self, o: &Media) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Media) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &Media, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof Media)
        if true {
        // Java: Media dst = (Media)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasMode() || !dst.hasMode()) && (!this.hasMode() || dst.hasMode()))
        else if (self.hasMode() || !dst.hasMode()) && (!self.hasMode() || dst.hasMode()) {
            // Java: if (this.hasMode() && dst.hasMode() && this.mode() != dst.mode())
            if self.hasMode() && dst.hasMode() && self.mode() != dst.mode() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasAudioCD() || !dst.hasAudioCD()) && (!this.hasAudioCD() || dst.hasAudioCD()))
            else if (self.hasAudioCD() || !dst.hasAudioCD()) && (!self.hasAudioCD() || dst.hasAudioCD()) {
                // Java: if (this.hasAudioCD() && dst.hasAudioCD())
                if self.hasAudioCD() && dst.hasAudioCD() {
                    // Java: if (isStrict && !this.audioCD().equals(dst.audioCD()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.audioCD().equalsLax(dst.audioCD()))
                    if !isStrict {
                        // Java: return false;
                    }
                }
                // Java: if ((this.hasAudioFile() || !dst.hasAudioFile()) && (!this.hasAudioFile() || dst.hasAudioFile()))
                if (self.hasAudioFile() || !dst.hasAudioFile()) && (!self.hasAudioFile() || dst.hasAudioFile()) {
                    // Java: if (this.hasAudioFile() && dst.hasAudioFile())
                    if self.hasAudioFile() && dst.hasAudioFile() {
                        // Java: if (isStrict && !this.audioFile().equals(dst.audioFile()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.audioFile().equalsLax(dst.audioFile()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                    // Java: if ((this.hasVideoFile() || !dst.hasVideoFile()) && (!this.hasVideoFile() || dst.hasVideoFile()))
                    if (self.hasVideoFile() || !dst.hasVideoFile()) && (!self.hasVideoFile() || dst.hasVideoFile()) {
                        // Java: if (this.hasVideoFile() && dst.hasVideoFile())
                        if self.hasVideoFile() && dst.hasVideoFile() {
                            // Java: if (isStrict && !this.videoFile().equals(dst.videoFile()))
                            if isStrict {
                                // Java: return false;
                            }
                            // Java: if (!isStrict && !this.videoFile().equalsLax(dst.videoFile()))
                            if !isStrict {
                                // Java: return false;
                            }
                        }
                        // Java: if ((this.hasMediaExtAttr() || !dst.hasMediaExtAttr()) && (!this.hasMediaExtAttr() || dst.hasMediaExtAttr()))
                        if (self.hasMediaExtAttr() || !dst.hasMediaExtAttr()) && (!self.hasMediaExtAttr() || dst.hasMediaExtAttr()) {
                            // Java: if (this.hasMediaExtAttr() && dst.hasMediaExtAttr())
                            if self.hasMediaExtAttr() && dst.hasMediaExtAttr() {
                                // Java: if (isStrict && !this.mediaExtAttr().equals(dst.mediaExtAttr()))
                                if isStrict {
                                    // Java: return false;
                                }
                                // Java: if (!isStrict && !this.mediaExtAttr().equalsLax(dst.mediaExtAttr()))
                                if !isStrict {
                                    // Java: return false;
                                }
                            }
                            // Java: if ((this.hasQuickTimeFile() || !dst.hasQuickTimeFile()) && (!this.hasQuickTimeFile() || dst.hasQuickTimeFile()))
                            if (self.hasQuickTimeFile() || !dst.hasQuickTimeFile()) && (!self.hasQuickTimeFile() || dst.hasQuickTimeFile()) {
                                // Java: if (this.hasQuickTimeFile() && dst.hasQuickTimeFile())
                                if self.hasQuickTimeFile() && dst.hasQuickTimeFile() {
                                    // Java: if (isStrict && !this.quickTimeFile().equals(dst.quickTimeFile()))
                                    if isStrict {
                                        // Java: return false;
                                    }
                                    // Java: if (!isStrict && !this.quickTimeFile().equalsLax(dst.quickTimeFile()))
                                    if !isStrict {
                                        // Java: return false;
                                    }
                                }
                                // Java: if ((this.hasEmbeddedWAVAudioFile() || !dst.hasEmbeddedWAVAudioFile()) && (!this.hasEmbeddedWAVAudioFile() || dst.hasEmbeddedWAVAudioFile()))
                                if (self.hasEmbeddedWAVAudioFile() || !dst.hasEmbeddedWAVAudioFile()) && (!self.hasEmbeddedWAVAudioFile() || dst.hasEmbeddedWAVAudioFile()) {
                                    // Java: if (this.hasEmbeddedWAVAudioFile() && dst.hasEmbeddedWAVAudioFile())
                                    if self.hasEmbeddedWAVAudioFile() && dst.hasEmbeddedWAVAudioFile() {
                                        // Java: if (isStrict && !this.embeddedWAVAudioFile().equals(dst.embeddedWAVAudioFile()))
                                        if isStrict {
                                            // Java: return false;
                                        }
                                        // Java: if (!isStrict && !this.embeddedWAVAudioFile().equalsLax(dst.embeddedWAVAudioFile()))
                                        if !isStrict {
                                            // Java: return false;
                                        }
                                    }
                                    // Java: return true;
                                    return true;
                                }
                                // Java: else
                                else {
                                    // Java: return false;
                                    return false;
                                }
                            }
                            // Java: else
                            else {
                                // Java: return false;
                                return false;
                            }
                        }
                        // Java: else
                        else {
                            // Java: return false;
                            return false;
                        }
                    }
                    // Java: else
                    else {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: else
                else {
                    // Java: return false;
                    return false;
                }
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            return false;
        }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasMode())
        if self.hasMode() {
            // Java: ret += Integer.valueOf(this.mode()).hashCode();
            ret += self.mode();
        }
        // Java: if (this.hasAudioCD())
        if self.hasAudioCD() {
            // Java: ret += this.audioCD().hashCode();
        }
        // Java: if (this.hasAudioFile())
        if self.hasAudioFile() {
            // Java: ret += this.audioFile().hashCode();
        }
        // Java: if (this.hasVideoFile())
        if self.hasVideoFile() {
            // Java: ret += this.videoFile().hashCode();
        }
        // Java: if (this.hasMediaExtAttr())
        if self.hasMediaExtAttr() {
            // Java: ret += this.mediaExtAttr().hashCode();
        }
        // Java: if (this.hasQuickTimeFile())
        if self.hasQuickTimeFile() {
            // Java: ret += this.quickTimeFile().hashCode();
        }
        // Java: if (this.hasEmbeddedWAVAudioFile())
        if self.hasEmbeddedWAVAudioFile() {
            // Java: ret += this.embeddedWAVAudioFile().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.audioCD != null && this.audioCD.dirty())
        if self.audioCD.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else if (this.audioFile != null && this.audioFile.dirty())
        else if self.audioFile.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else if (this.videoFile != null && this.videoFile.dirty())
        else if self.videoFile.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else if (this.mediaExtAttr != null && this.mediaExtAttr.dirty())
        else if self.mediaExtAttr.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else if (this.quickTimeFile != null && this.quickTimeFile.dirty())
        else if self.quickTimeFile.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.embeddedWAVAudioFile != null && this.embeddedWAVAudioFile.dirty() ? true : this.map.dirty();
            if self.embeddedWAVAudioFile.is_some() { true } else { self.map.dirty() }
        }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(&KoZone);
        // Java: return this.box.get();
        self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get();
        self.loadBin(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanParent();
        self.cleanParent();
        // Java: this.map.load(KoBin.Null.value1());
        let n = Null::value1();
        self.map.load(n.as_ref());
        // Java: this.box.clear();
        self.bin_box.clear();
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
        // Java: this.audioCD = null;
        self.audioCD = None;
        // Java: this.audioFile = null;
        self.audioFile = None;
        // Java: this.videoFile = null;
        self.videoFile = None;
        // Java: this.mediaExtAttr = null;
        self.mediaExtAttr = None;
        // Java: this.quickTimeFile = null;
        self.quickTimeFile = None;
        // Java: this.embeddedWAVAudioFile = null;
        self.embeddedWAVAudioFile = None;
    }

    // Java: void follow(Media parent)
    pub fn follow(&mut self, parent: &mut Media) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() {
                p.discard();
            }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: Media parent()
    pub fn parent_ref(&self) -> Option<&Media> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(&KoZone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: int mode()
    pub fn mode(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 0 : this.parent.mode();
            match &self.parent {
                None => 0,
                Some(p) => p.mode(),
            }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasMode()
    pub fn hasMode(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasMode();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasMode())
    }

    // Java: void deleteMode()
    pub fn deleteMode(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.put_null(1);
    }

    // Java: void putMode(int value)
    pub fn putMode(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &self.zone);
        self.map.put(1, bin.as_ref());
    }

    // Java: AudioCD audioCD()
    pub fn audioCD(&self) -> &dyn KoBinTrait {
        // Java: if (this.audioCD == null)
        // Java: if (this.map.has(2))
        // Java: KoBin bin = this.map.byKey(2);
        // Java: this.audioCD = (AudioCD)KoBin.Decode.toAny(AudioCD.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasAudioCD())
        // Java: this.audioCD = this.parent.audioCD();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.audioCD = (AudioCD)KoBin.Decode.toAny(AudioCD.class, bin, this.zone);
        // Java: return this.audioCD;
        todo!()
    }

    // Java: boolean hasAudioCD()
    pub fn hasAudioCD(&self) -> bool {
        // Java: return this.map.has(2) || this.audioCD != null && this.audioCD.dirty() || this.parent != null && this.parent.hasAudioCD();
        self.map.has(2) || self.audioCD.is_some() || self.parent.as_ref().map_or(false, |p| p.hasAudioCD())
    }

    // Java: void deleteAudioCD()
    pub fn deleteAudioCD(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.put_null(2);
        // Java: if (this.audioCD != null)
        if self.audioCD.is_some() {
            // Java: this.audioCD = null;
            self.audioCD = None;
        }
    }

    // Java: void writeAudioCD(AudioCD value)
    pub fn writeAudioCD(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            // Java: this.map.put(2, bin);
            // Java: if (this.audioCD != null)
            if self.audioCD.is_some() {
                // Java: this.audioCD = null;
                self.audioCD = None;
            }
        }
    }

    // Java: void forceWriteAudioCD(AudioCD value)
    pub fn forceWriteAudioCD(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            // Java: if (this.audioCD != null)
            if self.audioCD.is_some() {
                // Java: this.audioCD = null;
                self.audioCD = None;
            }
        }
    }

    // Java: AudioFile audioFile()
    pub fn audioFile(&self) -> &dyn KoBinTrait {
        // Java: if (this.audioFile == null)
        // Java: if (this.map.has(3))
        // Java: KoBin bin = this.map.byKey(3);
        // Java: this.audioFile = (AudioFile)KoBin.Decode.toAny(AudioFile.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasAudioFile())
        // Java: this.audioFile = this.parent.audioFile();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.audioFile = (AudioFile)KoBin.Decode.toAny(AudioFile.class, bin, this.zone);
        // Java: return this.audioFile;
        todo!()
    }

    // Java: boolean hasAudioFile()
    pub fn hasAudioFile(&self) -> bool {
        // Java: return this.map.has(3) || this.audioFile != null && this.audioFile.dirty() || this.parent != null && this.parent.hasAudioFile();
        self.map.has(3) || self.audioFile.is_some() || self.parent.as_ref().map_or(false, |p| p.hasAudioFile())
    }

    // Java: void deleteAudioFile()
    pub fn deleteAudioFile(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.put_null(3);
        // Java: if (this.audioFile != null)
        if self.audioFile.is_some() {
            // Java: this.audioFile = null;
            self.audioFile = None;
        }
    }

    // Java: void writeAudioFile(AudioFile value)
    pub fn writeAudioFile(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            // Java: this.map.put(3, bin);
            // Java: if (this.audioFile != null)
            if self.audioFile.is_some() {
                // Java: this.audioFile = null;
                self.audioFile = None;
            }
        }
    }

    // Java: void forceWriteAudioFile(AudioFile value)
    pub fn forceWriteAudioFile(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(3, value.lastBin(this.zone));
            // Java: if (this.audioFile != null)
            if self.audioFile.is_some() {
                // Java: this.audioFile = null;
                self.audioFile = None;
            }
        }
    }

    // Java: VideoFile videoFile()
    pub fn videoFile(&self) -> &dyn KoBinTrait {
        // Java: if (this.videoFile == null)
        // Java: if (this.map.has(4))
        // Java: KoBin bin = this.map.byKey(4);
        // Java: this.videoFile = (VideoFile)KoBin.Decode.toAny(VideoFile.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasVideoFile())
        // Java: this.videoFile = this.parent.videoFile();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.videoFile = (VideoFile)KoBin.Decode.toAny(VideoFile.class, bin, this.zone);
        // Java: return this.videoFile;
        todo!()
    }

    // Java: boolean hasVideoFile()
    pub fn hasVideoFile(&self) -> bool {
        // Java: return this.map.has(4) || this.videoFile != null && this.videoFile.dirty() || this.parent != null && this.parent.hasVideoFile();
        self.map.has(4) || self.videoFile.is_some() || self.parent.as_ref().map_or(false, |p| p.hasVideoFile())
    }

    // Java: void deleteVideoFile()
    pub fn deleteVideoFile(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.put_null(4);
        // Java: if (this.videoFile != null)
        if self.videoFile.is_some() {
            // Java: this.videoFile = null;
            self.videoFile = None;
        }
    }

    // Java: void writeVideoFile(VideoFile value)
    pub fn writeVideoFile(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            // Java: this.map.put(4, bin);
            // Java: if (this.videoFile != null)
            if self.videoFile.is_some() {
                // Java: this.videoFile = null;
                self.videoFile = None;
            }
        }
    }

    // Java: void forceWriteVideoFile(VideoFile value)
    pub fn forceWriteVideoFile(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(4, value.lastBin(this.zone));
            // Java: if (this.videoFile != null)
            if self.videoFile.is_some() {
                // Java: this.videoFile = null;
                self.videoFile = None;
            }
        }
    }

    // Java: MediaExtAttr mediaExtAttr()
    pub fn mediaExtAttr(&self) -> &dyn KoBinTrait {
        // Java: if (this.mediaExtAttr == null)
        // Java: if (this.map.has(5))
        // Java: KoBin bin = this.map.byKey(5);
        // Java: this.mediaExtAttr = (MediaExtAttr)KoBin.Decode.toAny(MediaExtAttr.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasMediaExtAttr())
        // Java: this.mediaExtAttr = this.parent.mediaExtAttr();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.mediaExtAttr = (MediaExtAttr)KoBin.Decode.toAny(MediaExtAttr.class, bin, this.zone);
        // Java: return this.mediaExtAttr;
        todo!()
    }

    // Java: boolean hasMediaExtAttr()
    pub fn hasMediaExtAttr(&self) -> bool {
        // Java: return this.map.has(5) || this.mediaExtAttr != null && this.mediaExtAttr.dirty() || this.parent != null && this.parent.hasMediaExtAttr();
        self.map.has(5) || self.mediaExtAttr.is_some() || self.parent.as_ref().map_or(false, |p| p.hasMediaExtAttr())
    }

    // Java: void deleteMediaExtAttr()
    pub fn deleteMediaExtAttr(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.put_null(5);
        // Java: if (this.mediaExtAttr != null)
        if self.mediaExtAttr.is_some() {
            // Java: this.mediaExtAttr = null;
            self.mediaExtAttr = None;
        }
    }

    // Java: void writeMediaExtAttr(MediaExtAttr value)
    pub fn writeMediaExtAttr(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            // Java: this.map.put(5, bin);
            // Java: if (this.mediaExtAttr != null)
            if self.mediaExtAttr.is_some() {
                // Java: this.mediaExtAttr = null;
                self.mediaExtAttr = None;
            }
        }
    }

    // Java: void forceWriteMediaExtAttr(MediaExtAttr value)
    pub fn forceWriteMediaExtAttr(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(5, value.lastBin(this.zone));
            // Java: if (this.mediaExtAttr != null)
            if self.mediaExtAttr.is_some() {
                // Java: this.mediaExtAttr = null;
                self.mediaExtAttr = None;
            }
        }
    }

    // Java: QuickTimeFile quickTimeFile()
    pub fn quickTimeFile(&self) -> &dyn KoBinTrait {
        // Java: if (this.quickTimeFile == null)
        // Java: if (this.map.has(6))
        // Java: KoBin bin = this.map.byKey(6);
        // Java: this.quickTimeFile = (QuickTimeFile)KoBin.Decode.toAny(QuickTimeFile.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasQuickTimeFile())
        // Java: this.quickTimeFile = this.parent.quickTimeFile();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.quickTimeFile = (QuickTimeFile)KoBin.Decode.toAny(QuickTimeFile.class, bin, this.zone);
        // Java: return this.quickTimeFile;
        todo!()
    }

    // Java: boolean hasQuickTimeFile()
    pub fn hasQuickTimeFile(&self) -> bool {
        // Java: return this.map.has(6) || this.quickTimeFile != null && this.quickTimeFile.dirty() || this.parent != null && this.parent.hasQuickTimeFile();
        self.map.has(6) || self.quickTimeFile.is_some() || self.parent.as_ref().map_or(false, |p| p.hasQuickTimeFile())
    }

    // Java: void deleteQuickTimeFile()
    pub fn deleteQuickTimeFile(&mut self) {
        // Java: this.map.put(6, (KoBin)null);
        self.map.put_null(6);
        // Java: if (this.quickTimeFile != null)
        if self.quickTimeFile.is_some() {
            // Java: this.quickTimeFile = null;
            self.quickTimeFile = None;
        }
    }

    // Java: void writeQuickTimeFile(QuickTimeFile value)
    pub fn writeQuickTimeFile(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            // Java: this.map.put(6, bin);
            // Java: if (this.quickTimeFile != null)
            if self.quickTimeFile.is_some() {
                // Java: this.quickTimeFile = null;
                self.quickTimeFile = None;
            }
        }
    }

    // Java: void forceWriteQuickTimeFile(QuickTimeFile value)
    pub fn forceWriteQuickTimeFile(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(6, value.lastBin(this.zone));
            // Java: if (this.quickTimeFile != null)
            if self.quickTimeFile.is_some() {
                // Java: this.quickTimeFile = null;
                self.quickTimeFile = None;
            }
        }
    }

    // Java: EmbeddedWAVAudioFile embeddedWAVAudioFile()
    pub fn embeddedWAVAudioFile(&self) -> &dyn KoBinTrait {
        // Java: if (this.embeddedWAVAudioFile == null)
        // Java: if (this.map.has(7))
        // Java: KoBin bin = this.map.byKey(7);
        // Java: this.embeddedWAVAudioFile = (EmbeddedWAVAudioFile)KoBin.Decode.toAny(EmbeddedWAVAudioFile.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasEmbeddedWAVAudioFile())
        // Java: this.embeddedWAVAudioFile = this.parent.embeddedWAVAudioFile();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.embeddedWAVAudioFile = (EmbeddedWAVAudioFile)KoBin.Decode.toAny(EmbeddedWAVAudioFile.class, bin, this.zone);
        // Java: return this.embeddedWAVAudioFile;
        todo!()
    }

    // Java: boolean hasEmbeddedWAVAudioFile()
    pub fn hasEmbeddedWAVAudioFile(&self) -> bool {
        // Java: return this.map.has(7) || this.embeddedWAVAudioFile != null && this.embeddedWAVAudioFile.dirty() || this.parent != null && this.parent.hasEmbeddedWAVAudioFile();
        self.map.has(7) || self.embeddedWAVAudioFile.is_some() || self.parent.as_ref().map_or(false, |p| p.hasEmbeddedWAVAudioFile())
    }

    // Java: void deleteEmbeddedWAVAudioFile()
    pub fn deleteEmbeddedWAVAudioFile(&mut self) {
        // Java: this.map.put(7, (KoBin)null);
        self.map.put_null(7);
        // Java: if (this.embeddedWAVAudioFile != null)
        if self.embeddedWAVAudioFile.is_some() {
            // Java: this.embeddedWAVAudioFile = null;
            self.embeddedWAVAudioFile = None;
        }
    }

    // Java: void writeEmbeddedWAVAudioFile(EmbeddedWAVAudioFile value)
    pub fn writeEmbeddedWAVAudioFile(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            // Java: this.map.put(7, bin);
            // Java: if (this.embeddedWAVAudioFile != null)
            if self.embeddedWAVAudioFile.is_some() {
                // Java: this.embeddedWAVAudioFile = null;
                self.embeddedWAVAudioFile = None;
            }
        }
    }

    // Java: void forceWriteEmbeddedWAVAudioFile(EmbeddedWAVAudioFile value)
    pub fn forceWriteEmbeddedWAVAudioFile(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(7, value.lastBin(this.zone));
            // Java: if (this.embeddedWAVAudioFile != null)
            if self.embeddedWAVAudioFile.is_some() {
                // Java: this.embeddedWAVAudioFile = null;
                self.embeddedWAVAudioFile = None;
            }
        }
    }
}

// Java: public class AudioCD
impl AudioCD {
    // Java: static protected static <Value1 extends AudioCD> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = AudioCD::AudioCD();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static AudioCD create()
    pub fn create() -> Self {
        // Java: return initialize(AudioCD.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static AudioCD open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(AudioCD.class, bin);
        Self::initialize(bin)
    }

    // Java: AudioCD()
    pub fn AudioCD() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        AudioCD { zone: KoZone, st: None, end: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: AudioCD(KoBin bin)
    pub fn AudioCD_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        AudioCD { zone: KoZone, st: None, end: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.map.load(this.box.get());
        let b = self.bin_box.get();
        self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(AudioCD value)
    pub fn load_from(&mut self, value: Option<&AudioCD>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.st != null && this.st.dirty())
        // Java: this.map.put(1, this.st.saveBin(zone));
        // Java: if (this.end != null && this.end.dirty())
        // Java: this.map.put(2, this.end.saveBin(zone));
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(AudioCD o)
    pub fn equalsLax(&self, o: &AudioCD) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &AudioCD) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &AudioCD, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof AudioCD)
        if true {
        // Java: AudioCD dst = (AudioCD)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasSt() || !dst.hasSt()) && (!this.hasSt() || dst.hasSt()))
        else if (self.hasSt() || !dst.hasSt()) && (!self.hasSt() || dst.hasSt()) {
            // Java: if (this.hasSt() && dst.hasSt())
            if self.hasSt() && dst.hasSt() {
                // Java: if (isStrict && !this.st().equals(dst.st()))
                if isStrict {
                    // Java: return false;
                }
                // Java: if (!isStrict && !this.st().equalsLax(dst.st()))
                if !isStrict {
                    // Java: return false;
                }
            }
            // Java: if ((this.hasEnd() || !dst.hasEnd()) && (!this.hasEnd() || dst.hasEnd()))
            if (self.hasEnd() || !dst.hasEnd()) && (!self.hasEnd() || dst.hasEnd()) {
                // Java: if (this.hasEnd() && dst.hasEnd())
                if self.hasEnd() && dst.hasEnd() {
                    // Java: if (isStrict && !this.end().equals(dst.end()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.end().equalsLax(dst.end()))
                    if !isStrict {
                        // Java: return false;
                    }
                }
                // Java: return true;
                return true;
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            return false;
        }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasSt())
        if self.hasSt() {
            // Java: ret += this.st().hashCode();
        }
        // Java: if (this.hasEnd())
        if self.hasEnd() {
            // Java: ret += this.end().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.st != null && this.st.dirty())
        if self.st.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.end != null && this.end.dirty() ? true : this.map.dirty();
            if self.end.is_some() { true } else { self.map.dirty() }
        }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(&KoZone);
        // Java: return this.box.get();
        self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get();
        self.loadBin(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanParent();
        self.cleanParent();
        // Java: this.map.load(KoBin.Null.value1());
        let n = Null::value1();
        self.map.load(n.as_ref());
        // Java: this.box.clear();
        self.bin_box.clear();
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
        // Java: this.st = null;
        self.st = None;
        // Java: this.end = null;
        self.end = None;
    }

    // Java: void follow(AudioCD parent)
    pub fn follow(&mut self, parent: &mut AudioCD) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: AudioCD parent()
    pub fn parent_ref(&self) -> Option<&AudioCD> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(&KoZone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: AudioCDTime st()
    pub fn st(&self) -> &dyn KoBinTrait {
        // Java: if (this.st == null)
        // Java: if (this.map.has(1))
        // Java: KoBin bin = this.map.byKey(1);
        // Java: this.st = (AudioCDTime)KoBin.Decode.toAny(AudioCDTime.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasSt())
        // Java: this.st = this.parent.st();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.st = (AudioCDTime)KoBin.Decode.toAny(AudioCDTime.class, bin, this.zone);
        // Java: return this.st;
        todo!()
    }

    // Java: boolean hasSt()
    pub fn hasSt(&self) -> bool {
        // Java: return this.map.has(1) || this.st != null && this.st.dirty() || this.parent != null && this.parent.hasSt();
        self.map.has(1) || self.st.is_some() || self.parent.as_ref().map_or(false, |p| p.hasSt())
    }

    // Java: void deleteSt()
    pub fn deleteSt(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.put_null(1);
        // Java: if (this.st != null)
        if self.st.is_some() {
            // Java: this.st = null;
            self.st = None;
        }
    }

    // Java: void writeSt(AudioCDTime value)
    pub fn writeSt(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            // Java: this.map.put(1, bin);
            // Java: if (this.st != null)
            if self.st.is_some() {
                // Java: this.st = null;
                self.st = None;
            }
        }
    }

    // Java: void forceWriteSt(AudioCDTime value)
    pub fn forceWriteSt(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(1, value.lastBin(this.zone));
            // Java: if (this.st != null)
            if self.st.is_some() {
                // Java: this.st = null;
                self.st = None;
            }
        }
    }

    // Java: AudioCDTime end()
    pub fn end(&self) -> &dyn KoBinTrait {
        // Java: if (this.end == null)
        // Java: if (this.map.has(2))
        // Java: KoBin bin = this.map.byKey(2);
        // Java: this.end = (AudioCDTime)KoBin.Decode.toAny(AudioCDTime.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasEnd())
        // Java: this.end = this.parent.end();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.end = (AudioCDTime)KoBin.Decode.toAny(AudioCDTime.class, bin, this.zone);
        // Java: return this.end;
        todo!()
    }

    // Java: boolean hasEnd()
    pub fn hasEnd(&self) -> bool {
        // Java: return this.map.has(2) || this.end != null && this.end.dirty() || this.parent != null && this.parent.hasEnd();
        self.map.has(2) || self.end.is_some() || self.parent.as_ref().map_or(false, |p| p.hasEnd())
    }

    // Java: void deleteEnd()
    pub fn deleteEnd(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.put_null(2);
        // Java: if (this.end != null)
        if self.end.is_some() {
            // Java: this.end = null;
            self.end = None;
        }
    }

    // Java: void writeEnd(AudioCDTime value)
    pub fn writeEnd(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            // Java: this.map.put(2, bin);
            // Java: if (this.end != null)
            if self.end.is_some() {
                // Java: this.end = null;
                self.end = None;
            }
        }
    }

    // Java: void forceWriteEnd(AudioCDTime value)
    pub fn forceWriteEnd(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            // Java: if (this.end != null)
            if self.end.is_some() {
                // Java: this.end = null;
                self.end = None;
            }
        }
    }
}

// Java: public class AudioCDTime
impl AudioCDTime {
    // Java: static protected static <Value1 extends AudioCDTime> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = AudioCDTime::AudioCDTime();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }
    // Java: static AudioCDTime create()
    pub fn create() -> Self {
        // Java: return initialize(AudioCDTime.class, KoBin.Null.empty());
        let empty = Null::empty(); Self::initialize(empty.as_ref())
    }
    // Java: static AudioCDTime open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(AudioCDTime.class, bin);
        Self::initialize(bin)
    }
    // Java: AudioCDTime()
    pub fn AudioCDTime() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        AudioCDTime { zone: KoZone, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }
    // Java: AudioCDTime(KoBin bin)
    pub fn AudioCDTime_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        AudioCDTime { zone: KoZone, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
    }
    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.map.load(this.box.get());
        let b = self.bin_box.get(); self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(AudioCDTime value)
    pub fn load_from(&mut self, value: Option<&AudioCDTime>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(AudioCDTime o)
    pub fn equalsLax(&self, o: &AudioCDTime) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &AudioCDTime) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &AudioCDTime, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof AudioCDTime)
        if true {
            // Java: AudioCDTime dst = (AudioCDTime)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasTrack() || !dst.hasTrack()) && (!this.hasTrack() || dst.hasTrack()))
            else if (self.hasTrack() || !dst.hasTrack()) && (!self.hasTrack() || dst.hasTrack()) {
                // Java: if (this.hasTrack() && dst.hasTrack() && this.track() != dst.track())
                if self.hasTrack() && dst.hasTrack() && self.track() != dst.track() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasTime() || !dst.hasTime()) && (!this.hasTime() || dst.hasTime()))
                else if (self.hasTime() || !dst.hasTime()) && (!self.hasTime() || dst.hasTime()) {
                    // Java: return !this.hasTime() || !dst.hasTime() || this.time() == dst.time();
                    return !self.hasTime() || !dst.hasTime() || self.time() == dst.time();
                }
                // Java: else
                else {
                    // Java: return false;
                    return false;
                }
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasTrack())
        if self.hasTrack() {
            // Java: ret += Integer.valueOf(this.track()).hashCode();
            ret += self.track();
        }
        // Java: if (this.hasTime())
        if self.hasTime() {
            // Java: ret += Integer.valueOf(this.time()).hashCode();
            ret += self.time();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.map.dirty();
        self.map.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(&KoZone);
        // Java: return this.box.get();
        self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get();
        self.loadBin(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanParent();
        self.cleanParent();
        // Java: this.map.load(KoBin.Null.value1());
        let n = Null::value1();
        self.map.load(n.as_ref());
        // Java: this.box.clear();
        self.bin_box.clear();
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {}

    // Java: void follow(AudioCDTime parent)
    pub fn follow(&mut self, parent: &mut AudioCDTime) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: AudioCDTime parent()
    pub fn parent_ref(&self) -> Option<&AudioCDTime> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(&KoZone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: int track()
    pub fn track(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 0 : this.parent.track();
            match &self.parent { None => 0, Some(p) => p.track() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasTrack()
    pub fn hasTrack(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasTrack();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasTrack())
    }

    // Java: void deleteTrack()
    pub fn deleteTrack(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.put_null(1);
    }

    // Java: void putTrack(int value)
    pub fn putTrack(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &self.zone);
        self.map.put(1, bin.as_ref());
    }

    // Java: int time()
    pub fn time(&self) -> i32 {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? 0 : this.parent.time();
            match &self.parent { None => 0, Some(p) => p.time() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasTime()
    pub fn hasTime(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasTime();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasTime())
    }

    // Java: void deleteTime()
    pub fn deleteTime(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.put_null(2);
    }

    // Java: void putTime(int value)
    pub fn putTime(&mut self, value: i32) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &self.zone);
        self.map.put(2, bin.as_ref());
    }
}

// Java: public class EmbeddedWAVAudioFile
impl EmbeddedWAVAudioFile {
    // Java: static protected static <Value1 extends EmbeddedWAVAudioFile> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = EmbeddedWAVAudioFile::EmbeddedWAVAudioFile();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }
    // Java: static EmbeddedWAVAudioFile create()
    pub fn create() -> Self {
        // Java: return initialize(EmbeddedWAVAudioFile.class, KoBin.Null.empty());
        let empty = Null::empty(); Self::initialize(empty.as_ref())
    }
    // Java: static EmbeddedWAVAudioFile open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(EmbeddedWAVAudioFile.class, bin);
        Self::initialize(bin)
    }
    // Java: EmbeddedWAVAudioFile()
    pub fn EmbeddedWAVAudioFile() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        EmbeddedWAVAudioFile { zone: KoZone, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }
    // Java: EmbeddedWAVAudioFile(KoBin bin)
    pub fn EmbeddedWAVAudioFile_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        EmbeddedWAVAudioFile { zone: KoZone, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
    }
    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.map.load(this.box.get());
        let b = self.bin_box.get(); self.map.load(b.as_ref());
    }
    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, &KoZone);
    }
    // Java: void load(EmbeddedWAVAudioFile value)
    pub fn load_from(&mut self, value: Option<&EmbeddedWAVAudioFile>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone); self.loadBin(last.as_ref(), &KoZone);
        }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }
    // Java: boolean equalsLax(EmbeddedWAVAudioFile o)
    pub fn equalsLax(&self, o: &EmbeddedWAVAudioFile) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &EmbeddedWAVAudioFile) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &EmbeddedWAVAudioFile, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof EmbeddedWAVAudioFile)
        if true {
            // Java: EmbeddedWAVAudioFile dst = (EmbeddedWAVAudioFile)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
            else if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName()) {
                // Java: if (this.hasName() && dst.hasName() && !this.name().equals(dst.name()))
                if self.hasName() && dst.hasName() && self.name() != dst.name() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasMediaID() || !dst.hasMediaID()) && (!this.hasMediaID() || dst.hasMediaID()))
                else if (self.hasMediaID() || !dst.hasMediaID()) && (!self.hasMediaID() || dst.hasMediaID()) {
                    // Java: return !this.hasMediaID() || !dst.hasMediaID() || this.mediaID() == dst.mediaID();
                    return !self.hasMediaID() || !dst.hasMediaID() || self.mediaID() == dst.mediaID();
                }
                // Java: else
                else {
                    // Java: return false;
                    return false;
                }
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasName())
        if self.hasName() {
            // Java: ret += String.valueOf(this.name()).hashCode();
        }
        // Java: if (this.hasMediaID())
        if self.hasMediaID() {
            // Java: ret += Integer.valueOf(this.mediaID()).hashCode();
            ret += self.mediaID();
        }
        // Java: return ret;
        ret
    }
    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.map.dirty();
        self.map.dirty()
    }
    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(&KoZone);
        // Java: return this.box.get();
        self.bin_box.get()
    }
    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone)
    }
    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone);
    }
    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanParent();
        self.cleanParent();
        // Java: this.map.load(KoBin.Null.value1());
        let n = Null::value1(); self.map.load(n.as_ref());
        // Java: this.box.clear();
        self.bin_box.clear();
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }
    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {}
    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {}
    // Java: void follow(EmbeddedWAVAudioFile parent)
    pub fn follow(&mut self, parent: &mut EmbeddedWAVAudioFile) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }
    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }
    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: EmbeddedWAVAudioFile parent()
    pub fn parent_ref(&self) -> Option<&EmbeddedWAVAudioFile> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(&KoZone);
    }
    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }
    // Java: String name()
    pub fn name(&self) -> String {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? "" : this.parent.name();
            match &self.parent { None => String::new(), Some(p) => p.name() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toString(bin.as_ref())
        }
    }
    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasName();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }
    // Java: void deleteName()
    pub fn deleteName(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.put_null(1);
    }
    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, &self.zone); self.map.put(1, bin.as_ref());
    }
    // Java: int mediaID()
    pub fn mediaID(&self) -> i32 {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? 0 : this.parent.mediaID();
            match &self.parent { None => 0, Some(p) => p.mediaID() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }
    // Java: boolean hasMediaID()
    pub fn hasMediaID(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasMediaID();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasMediaID())
    }
    // Java: void deleteMediaID()
    pub fn deleteMediaID(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.put_null(2);
    }
    // Java: void putMediaID(int value)
    pub fn putMediaID(&mut self, value: i32) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &self.zone); self.map.put(2, bin.as_ref());
    }
}

// Java: public class AudioFile
impl AudioFile {
    // Java: static protected static <Value1 extends AudioFile> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = AudioFile::AudioFile();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static AudioFile create()
    pub fn create() -> Self {
        // Java: return initialize(AudioFile.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static AudioFile open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(AudioFile.class, bin);
        Self::initialize(bin)
    }

    // Java: AudioFile()
    pub fn AudioFile() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        AudioFile { zone: KoZone, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: AudioFile(KoBin bin)
    pub fn AudioFile_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        AudioFile { zone: KoZone, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.map.load(this.box.get());
        let b = self.bin_box.get();
        self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(AudioFile value)
    pub fn load_from(&mut self, value: Option<&AudioFile>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(AudioFile o)
    pub fn equalsLax(&self, o: &AudioFile) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &AudioFile) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &AudioFile, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof AudioFile)
        if true {
            // Java: AudioFile dst = (AudioFile)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasMediaID() || !dst.hasMediaID()) && (!this.hasMediaID() || dst.hasMediaID()))
            else if (self.hasMediaID() || !dst.hasMediaID()) && (!self.hasMediaID() || dst.hasMediaID()) {
                // Java: if (this.hasMediaID() && dst.hasMediaID() && this.mediaID() != dst.mediaID())
                if self.hasMediaID() && dst.hasMediaID() && self.mediaID() != dst.mediaID() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasContentType() || !dst.hasContentType()) && (!this.hasContentType() || dst.hasContentType()))
                else if (self.hasContentType() || !dst.hasContentType()) && (!self.hasContentType() || dst.hasContentType()) {
                    // Java: return !this.hasContentType() || !dst.hasContentType() || this.contentType().equals(dst.contentType());
                    return !self.hasContentType() || !dst.hasContentType() || self.contentType() == dst.contentType();
                }
                // Java: else
                else {
                    // Java: return false;
                    return false;
                }
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasMediaID())
        if self.hasMediaID() {
            // Java: ret += Integer.valueOf(this.mediaID()).hashCode();
            ret += self.mediaID();
        }
        // Java: if (this.hasContentType())
        if self.hasContentType() {
            // Java: ret += String.valueOf(this.contentType()).hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.map.dirty();
        self.map.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(&KoZone);
        // Java: return this.box.get();
        self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get();
        self.loadBin(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanParent();
        self.cleanParent();
        // Java: this.map.load(KoBin.Null.value1());
        let n = Null::value1();
        self.map.load(n.as_ref());
        // Java: this.box.clear();
        self.bin_box.clear();
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {}

    // Java: void follow(AudioFile parent)
    pub fn follow(&mut self, parent: &mut AudioFile) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: AudioFile parent()
    pub fn parent_ref(&self) -> Option<&AudioFile> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(&KoZone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: int mediaID()
    pub fn mediaID(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 0 : this.parent.mediaID();
            match &self.parent { None => 0, Some(p) => p.mediaID() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasMediaID()
    pub fn hasMediaID(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasMediaID();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasMediaID())
    }

    // Java: void deleteMediaID()
    pub fn deleteMediaID(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.put_null(1);
    }

    // Java: void putMediaID(int value)
    pub fn putMediaID(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &self.zone);
        self.map.put(1, bin.as_ref());
    }

    // Java: String contentType()
    pub fn contentType(&self) -> String {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? "" : this.parent.contentType();
            match &self.parent { None => String::new(), Some(p) => p.contentType() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toString(bin.as_ref())
        }
    }

    // Java: boolean hasContentType()
    pub fn hasContentType(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasContentType();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasContentType())
    }

    // Java: void deleteContentType()
    pub fn deleteContentType(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.put_null(2);
    }

    // Java: void putContentType(String value)
    pub fn putContentType(&mut self, value: &str) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, &self.zone);
        self.map.put(2, bin.as_ref());
    }
}

// Java: public class VideoFile
impl VideoFile {
    // Java: static protected static <Value1 extends VideoFile> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = VideoFile::VideoFile();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static VideoFile create()
    pub fn create() -> Self {
        // Java: return initialize(VideoFile.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static VideoFile open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(VideoFile.class, bin);
        Self::initialize(bin)
    }

    // Java: VideoFile()
    pub fn VideoFile() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        VideoFile { zone: KoZone, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: VideoFile(KoBin bin)
    pub fn VideoFile_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        VideoFile { zone: KoZone, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.map.load(this.box.get());
        let b = self.bin_box.get();
        self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(VideoFile value)
    pub fn load_from(&mut self, value: Option<&VideoFile>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(VideoFile o)
    pub fn equalsLax(&self, o: &VideoFile) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &VideoFile) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &VideoFile, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof VideoFile)
        if true {
            // Java: VideoFile dst = (VideoFile)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasMediaID() || !dst.hasMediaID()) && (!this.hasMediaID() || dst.hasMediaID()))
            else if (self.hasMediaID() || !dst.hasMediaID()) && (!self.hasMediaID() || dst.hasMediaID()) {
                // Java: if (this.hasMediaID() && dst.hasMediaID() && this.mediaID() != dst.mediaID())
                if self.hasMediaID() && dst.hasMediaID() && self.mediaID() != dst.mediaID() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasContentType() || !dst.hasContentType()) && (!this.hasContentType() || dst.hasContentType()))
                else if (self.hasContentType() || !dst.hasContentType()) && (!self.hasContentType() || dst.hasContentType()) {
                    // Java: return !this.hasContentType() || !dst.hasContentType() || this.contentType().equals(dst.contentType());
                    return !self.hasContentType() || !dst.hasContentType() || self.contentType() == dst.contentType();
                }
                // Java: else
                else {
                    // Java: return false;
                    return false;
                }
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasMediaID())
        if self.hasMediaID() {
            // Java: ret += Integer.valueOf(this.mediaID()).hashCode();
            ret += self.mediaID();
        }
        // Java: if (this.hasContentType())
        if self.hasContentType() {
            // Java: ret += String.valueOf(this.contentType()).hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.map.dirty();
        self.map.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(&KoZone);
        // Java: return this.box.get();
        self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get();
        self.loadBin(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanParent();
        self.cleanParent();
        // Java: this.map.load(KoBin.Null.value1());
        let n = Null::value1();
        self.map.load(n.as_ref());
        // Java: this.box.clear();
        self.bin_box.clear();
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {}

    // Java: void follow(VideoFile parent)
    pub fn follow(&mut self, parent: &mut VideoFile) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: VideoFile parent()
    pub fn parent_ref(&self) -> Option<&VideoFile> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(&KoZone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: int mediaID()
    pub fn mediaID(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 0 : this.parent.mediaID();
            match &self.parent { None => 0, Some(p) => p.mediaID() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasMediaID()
    pub fn hasMediaID(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasMediaID();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasMediaID())
    }

    // Java: void deleteMediaID()
    pub fn deleteMediaID(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.put_null(1);
    }

    // Java: void putMediaID(int value)
    pub fn putMediaID(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &self.zone);
        self.map.put(1, bin.as_ref());
    }

    // Java: String contentType()
    pub fn contentType(&self) -> String {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? "" : this.parent.contentType();
            match &self.parent { None => String::new(), Some(p) => p.contentType() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toString(bin.as_ref())
        }
    }

    // Java: boolean hasContentType()
    pub fn hasContentType(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasContentType();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasContentType())
    }

    // Java: void deleteContentType()
    pub fn deleteContentType(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.put_null(2);
    }

    // Java: void putContentType(String value)
    pub fn putContentType(&mut self, value: &str) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, &self.zone);
        self.map.put(2, bin.as_ref());
    }
}

// Java: public class QuickTimeFile
impl QuickTimeFile {
    // Java: static protected static <Value1 extends QuickTimeFile> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = QuickTimeFile::QuickTimeFile();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static QuickTimeFile create()
    pub fn create() -> Self {
        // Java: return initialize(QuickTimeFile.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static QuickTimeFile open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(QuickTimeFile.class, bin);
        Self::initialize(bin)
    }

    // Java: QuickTimeFile()
    pub fn QuickTimeFile() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        QuickTimeFile { zone: KoZone, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: QuickTimeFile(KoBin bin)
    pub fn QuickTimeFile_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        QuickTimeFile { zone: KoZone, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.map.load(this.box.get());
        let b = self.bin_box.get();
        self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(QuickTimeFile value)
    pub fn load_from(&mut self, value: Option<&QuickTimeFile>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(QuickTimeFile o)
    pub fn equalsLax(&self, o: &QuickTimeFile) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &QuickTimeFile) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &QuickTimeFile, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof QuickTimeFile)
        if true {
            // Java: QuickTimeFile dst = (QuickTimeFile)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasMediaID() || !dst.hasMediaID()) && (!this.hasMediaID() || dst.hasMediaID()))
            else if (self.hasMediaID() || !dst.hasMediaID()) && (!self.hasMediaID() || dst.hasMediaID()) {
                // Java: return !this.hasMediaID() || !dst.hasMediaID() || this.mediaID() == dst.mediaID();
                return !self.hasMediaID() || !dst.hasMediaID() || self.mediaID() == dst.mediaID();
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasMediaID())
        if self.hasMediaID() {
            // Java: ret += Integer.valueOf(this.mediaID()).hashCode();
            ret += self.mediaID();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.map.dirty();
        self.map.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(&KoZone);
        // Java: return this.box.get();
        self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get();
        self.loadBin(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanParent();
        self.cleanParent();
        // Java: this.map.load(KoBin.Null.value1());
        let n = Null::value1();
        self.map.load(n.as_ref());
        // Java: this.box.clear();
        self.bin_box.clear();
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {}

    // Java: void follow(QuickTimeFile parent)
    pub fn follow(&mut self, parent: &mut QuickTimeFile) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: QuickTimeFile parent()
    pub fn parent_ref(&self) -> Option<&QuickTimeFile> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(&KoZone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: int mediaID()
    pub fn mediaID(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 0 : this.parent.mediaID();
            match &self.parent { None => 0, Some(p) => p.mediaID() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasMediaID()
    pub fn hasMediaID(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasMediaID();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasMediaID())
    }

    // Java: void deleteMediaID()
    pub fn deleteMediaID(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.put_null(1);
    }

    // Java: void putMediaID(int value)
    pub fn putMediaID(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &self.zone);
        self.map.put(1, bin.as_ref());
    }
}

// Java: public class Bookmark
impl Bookmark {
    // Java: static protected static <Value1 extends Bookmark> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Bookmark::Bookmark();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static Bookmark create()
    pub fn create() -> Self {
        // Java: return initialize(Bookmark.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static Bookmark open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(Bookmark.class, bin);
        Self::initialize(bin)
    }

    // Java: Bookmark()
    pub fn Bookmark() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        Bookmark { zone: KoZone, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: Bookmark(KoBin bin)
    pub fn Bookmark_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        Bookmark { zone: KoZone, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.map.load(this.box.get());
        let b = self.bin_box.get();
        self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(Bookmark value)
    pub fn load_from(&mut self, value: Option<&Bookmark>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(Bookmark o)
    pub fn equalsLax(&self, o: &Bookmark) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Bookmark) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &Bookmark, _isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof Bookmark)
        if true {
            // Java: Bookmark dst = (Bookmark)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
            else if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName()) {
                // Java: if (this.hasName() && dst.hasName() && !this.name().equals(dst.name()))
                if self.hasName() && dst.hasName() && self.name() != dst.name() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasTime() || !dst.hasTime()) && (!this.hasTime() || dst.hasTime()))
                else if (self.hasTime() || !dst.hasTime()) && (!self.hasTime() || dst.hasTime()) {
                    // Java: return !this.hasTime() || !dst.hasTime() || this.time() == dst.time();
                    return !self.hasTime() || !dst.hasTime() || self.time() == dst.time();
                }
                // Java: else
                else {
                    // Java: return false;
                    return false;
                }
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasName())
        if self.hasName() {
            // Java: ret += String.valueOf(this.name()).hashCode();
        }
        // Java: if (this.hasTime())
        if self.hasTime() {
            // Java: ret += Double.valueOf(this.time()).hashCode();
            ret += self.time() as i32;
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.map.dirty();
        self.map.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(&KoZone);
        // Java: return this.box.get();
        self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get();
        self.loadBin(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanParent();
        self.cleanParent();
        // Java: this.map.load(KoBin.Null.value1());
        let n = Null::value1();
        self.map.load(n.as_ref());
        // Java: this.box.clear();
        self.bin_box.clear();
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {}

    // Java: void follow(Bookmark parent)
    pub fn follow(&mut self, parent: &mut Bookmark) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: Bookmark parent()
    pub fn parent_ref(&self) -> Option<&Bookmark> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(&KoZone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: String name()
    pub fn name(&self) -> String {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? "" : this.parent.name();
            match &self.parent { None => String::new(), Some(p) => p.name() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toString(bin.as_ref())
        }
    }

    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasName();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }

    // Java: void deleteName()
    pub fn deleteName(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.put_null(1);
    }

    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, &self.zone);
        self.map.put(1, bin.as_ref());
    }

    // Java: double time()
    pub fn time(&self) -> f64 {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? (double)0.0F : this.parent.time();
            match &self.parent { None => 0.0, Some(p) => p.time() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toDouble(bin);
            Decode::toDouble(bin.as_ref())
        }
    }

    // Java: boolean hasTime()
    pub fn hasTime(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasTime();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasTime())
    }

    // Java: void deleteTime()
    pub fn deleteTime(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.put_null(2);
    }

    // Java: void putTime(double value)
    pub fn putTime(&mut self, value: f64) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_double(value, &self.zone);
        self.map.put(2, bin.as_ref());
    }
}

// Java: public class MediaExtAttr
impl MediaExtAttr {
    // Java: static protected static <Value1 extends MediaExtAttr> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = MediaExtAttr::MediaExtAttr();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static MediaExtAttr create()
    pub fn create() -> Self {
        // Java: return initialize(MediaExtAttr.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static MediaExtAttr open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(MediaExtAttr.class, bin);
        Self::initialize(bin)
    }

    // Java: MediaExtAttr()
    pub fn MediaExtAttr() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        MediaExtAttr { zone: KoZone, bookmarkLst: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: MediaExtAttr(KoBin bin)
    pub fn MediaExtAttr_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        MediaExtAttr { zone: KoZone, bookmarkLst: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.map.load(this.box.get());
        let b = self.bin_box.get();
        self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(MediaExtAttr value)
    pub fn load_from(&mut self, value: Option<&MediaExtAttr>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.bookmarkLst != null && this.bookmarkLst.dirty())
        // Java: this.map.put(5, this.bookmarkLst.saveBin(zone));
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }
    // Java: boolean equalsLax(MediaExtAttr o)
    pub fn equalsLax(&self, o: &MediaExtAttr) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &MediaExtAttr) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &MediaExtAttr, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof MediaExtAttr)
        if true {
            // Java: MediaExtAttr dst = (MediaExtAttr)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasFadeOut() || !dst.hasFadeOut()) && (!this.hasFadeOut() || dst.hasFadeOut()))
            else if (self.hasFadeOut() || !dst.hasFadeOut()) && (!self.hasFadeOut() || dst.hasFadeOut()) {
                // Java: if (this.hasFadeOut() && dst.hasFadeOut() && this.fadeOut() != dst.fadeOut())
                if self.hasFadeOut() && dst.hasFadeOut() && self.fadeOut() != dst.fadeOut() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasFadeIn() || !dst.hasFadeIn()) && (!this.hasFadeIn() || dst.hasFadeIn()))
                else if (self.hasFadeIn() || !dst.hasFadeIn()) && (!self.hasFadeIn() || dst.hasFadeIn()) {
                    // Java: if (this.hasFadeIn() && dst.hasFadeIn() && this.fadeIn() != dst.fadeIn())
                    if self.hasFadeIn() && dst.hasFadeIn() && self.fadeIn() != dst.fadeIn() {
                        // Java: return false;
                        return false;
                    }
                    // Java: else if ((this.hasTrimStart() || !dst.hasTrimStart()) && (!this.hasTrimStart() || dst.hasTrimStart()))
                    else if (self.hasTrimStart() || !dst.hasTrimStart()) && (!self.hasTrimStart() || dst.hasTrimStart()) {
                        // Java: if (this.hasTrimStart() && dst.hasTrimStart() && this.trimStart() != dst.trimStart())
                        if self.hasTrimStart() && dst.hasTrimStart() && self.trimStart() != dst.trimStart() {
                            // Java: return false;
                            return false;
                        }
                        // Java: else if ((this.hasTrimEnd() || !dst.hasTrimEnd()) && (!this.hasTrimEnd() || dst.hasTrimEnd()))
                        else if (self.hasTrimEnd() || !dst.hasTrimEnd()) && (!self.hasTrimEnd() || dst.hasTrimEnd()) {
                            // Java: if (this.hasTrimEnd() && dst.hasTrimEnd() && this.trimEnd() != dst.trimEnd())
                            if self.hasTrimEnd() && dst.hasTrimEnd() && self.trimEnd() != dst.trimEnd() {
                                // Java: return false;
                                return false;
                            }
                            // Java: else if ((this.hasBookmarkLst() || !dst.hasBookmarkLst()) && (!this.hasBookmarkLst() || dst.hasBookmarkLst()))
                            else if (self.hasBookmarkLst() || !dst.hasBookmarkLst()) && (!self.hasBookmarkLst() || dst.hasBookmarkLst()) {
                                // Java: if (this.hasBookmarkLst() && dst.hasBookmarkLst())
                                if self.hasBookmarkLst() && dst.hasBookmarkLst() {
                                    // Java: if (isStrict && !this.bookmarkLst().equals(dst.bookmarkLst()))
                                    if isStrict {
                                        // Java: return false;
                                    }
                                    // Java: if (!isStrict && !this.bookmarkLst().equalsLax(dst.bookmarkLst()))
                                    if !isStrict {
                                        // Java: return false;
                                    }
                                }
                                // Java: return true;
                                return true;
                            }
                            // Java: else
                            else {
                                // Java: return false;
                                return false;
                            }
                        }
                        // Java: else
                        else {
                            // Java: return false;
                            return false;
                        }
                    }
                    // Java: else
                    else {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: else
                else {
                    // Java: return false;
                    return false;
                }
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasFadeOut())
        if self.hasFadeOut() {
            // Java: ret += Integer.valueOf(this.fadeOut()).hashCode();
            ret += self.fadeOut();
        }
        // Java: if (this.hasFadeIn())
        if self.hasFadeIn() {
            // Java: ret += Integer.valueOf(this.fadeIn()).hashCode();
            ret += self.fadeIn();
        }
        // Java: if (this.hasTrimStart())
        if self.hasTrimStart() {
            // Java: ret += Integer.valueOf(this.trimStart()).hashCode();
            ret += self.trimStart();
        }
        // Java: if (this.hasTrimEnd())
        if self.hasTrimEnd() {
            // Java: ret += Integer.valueOf(this.trimEnd()).hashCode();
            ret += self.trimEnd();
        }
        // Java: if (this.hasBookmarkLst())
        if self.hasBookmarkLst() {
            // Java: ret += this.bookmarkLst().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.bookmarkLst != null && this.bookmarkLst.dirty() ? true : this.map.dirty();
        if self.bookmarkLst.is_some() { true } else { self.map.dirty() }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(&KoZone);
        // Java: return this.box.get();
        self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get();
        self.loadBin(bin.as_ref(), zone);
    }
    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanParent();
        self.cleanParent();
        // Java: this.map.load(KoBin.Null.value1());
        let n = Null::value1();
        self.map.load(n.as_ref());
        // Java: this.box.clear();
        self.bin_box.clear();
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }
    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
        // Java: this.bookmarkLst = null;
        self.bookmarkLst = None;
    }

    // Java: void follow(MediaExtAttr parent)
    pub fn follow(&mut self, parent: &mut MediaExtAttr) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: MediaExtAttr parent()
    pub fn parent_ref(&self) -> Option<&MediaExtAttr> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(&KoZone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }
    // Java: int fadeOut()
    pub fn fadeOut(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 0 : this.parent.fadeOut();
            match &self.parent { None => 0, Some(p) => p.fadeOut() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasFadeOut()
    pub fn hasFadeOut(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasFadeOut();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasFadeOut())
    }

    // Java: void deleteFadeOut()
    pub fn deleteFadeOut(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.put_null(1);
    }

    // Java: void putFadeOut(int value)
    pub fn putFadeOut(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &self.zone);
        self.map.put(1, bin.as_ref());
    }

    // Java: int fadeIn()
    pub fn fadeIn(&self) -> i32 {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? 0 : this.parent.fadeIn();
            match &self.parent { None => 0, Some(p) => p.fadeIn() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasFadeIn()
    pub fn hasFadeIn(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasFadeIn();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasFadeIn())
    }

    // Java: void deleteFadeIn()
    pub fn deleteFadeIn(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.put_null(2);
    }

    // Java: void putFadeIn(int value)
    pub fn putFadeIn(&mut self, value: i32) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &self.zone);
        self.map.put(2, bin.as_ref());
    }

    // Java: int trimStart()
    pub fn trimStart(&self) -> i32 {
        // Java: if (!this.map.has(3))
        if !self.map.has(3) {
            // Java: return this.parent == null ? 0 : this.parent.trimStart();
            match &self.parent { None => 0, Some(p) => p.trimStart() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasTrimStart()
    pub fn hasTrimStart(&self) -> bool {
        // Java: return this.map.has(3) || this.parent != null && this.parent.hasTrimStart();
        self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasTrimStart())
    }

    // Java: void deleteTrimStart()
    pub fn deleteTrimStart(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.put_null(3);
    }

    // Java: void putTrimStart(int value)
    pub fn putTrimStart(&mut self, value: i32) {
        // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &self.zone);
        self.map.put(3, bin.as_ref());
    }

    // Java: int trimEnd()
    pub fn trimEnd(&self) -> i32 {
        // Java: if (!this.map.has(4))
        if !self.map.has(4) {
            // Java: return this.parent == null ? 0 : this.parent.trimEnd();
            match &self.parent { None => 0, Some(p) => p.trimEnd() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasTrimEnd()
    pub fn hasTrimEnd(&self) -> bool {
        // Java: return this.map.has(4) || this.parent != null && this.parent.hasTrimEnd();
        self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasTrimEnd())
    }

    // Java: void deleteTrimEnd()
    pub fn deleteTrimEnd(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.put_null(4);
    }

    // Java: void putTrimEnd(int value)
    pub fn putTrimEnd(&mut self, value: i32) {
        // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &self.zone);
        self.map.put(4, bin.as_ref());
    }

    // Java: BookmarkLst bookmarkLst()
    pub fn bookmarkLst(&self) -> &dyn KoBinTrait {
        // Java: if (this.bookmarkLst == null)
        if self.bookmarkLst.is_none() {
            // Java: if (this.map.has(5))
            if self.map.has(5) {
                // Java: KoBin bin = this.map.byKey(5);
                let bin = self.map.byKey(5);
                // Java: this.bookmarkLst = (BookmarkLst)KoBin.Decode.toAny(BookmarkLst.class, bin, this.zone);
                todo!()
            // Java: else if (this.parent != null && this.parent.hasBookmarkLst())
            } else if self.parent.as_ref().map_or(false, |p| p.hasBookmarkLst()) {
                // Java: this.bookmarkLst = this.parent.bookmarkLst();
                todo!()
            // Java: else
            } else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.bookmarkLst = (BookmarkLst)KoBin.Decode.toAny(BookmarkLst.class, bin, this.zone);
                todo!()
            }
        }
        // Java: return this.bookmarkLst;
        todo!()
    }

    // Java: boolean hasBookmarkLst()
    pub fn hasBookmarkLst(&self) -> bool {
        // Java: return this.map.has(5) || this.bookmarkLst != null && this.bookmarkLst.dirty() || this.parent != null && this.parent.hasBookmarkLst();
        self.map.has(5) || self.bookmarkLst.is_some() || self.parent.as_ref().map_or(false, |p| p.hasBookmarkLst())
    }

    // Java: void deleteBookmarkLst()
    pub fn deleteBookmarkLst(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.put_null(5);
        // Java: if (this.bookmarkLst != null)
        if self.bookmarkLst.is_some() {
            // Java: this.bookmarkLst = null;
            self.bookmarkLst = None;
        }
    }

    // Java: void writeBookmarkLst(BookmarkLst value)
    pub fn writeBookmarkLst(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(&self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(bin.as_ref()) {
                // Java: this.map.put(5, bin);
                self.map.put(5, bin.as_ref());
                // Java: if (this.bookmarkLst != null)
                if self.bookmarkLst.is_some() {
                    // Java: this.bookmarkLst = null;
                    self.bookmarkLst = None;
                }
            }
        }
    }

    // Java: void forceWriteBookmarkLst(BookmarkLst value)
    pub fn forceWriteBookmarkLst(&mut self, value: &dyn KoBinTrait) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(5, value.lastBin(this.zone));
            self.map.put(5, value.lastBin(&self.zone).as_ref());
            // Java: if (this.bookmarkLst != null)
            if self.bookmarkLst.is_some() {
                // Java: this.bookmarkLst = null;
                self.bookmarkLst = None;
            }
        }
    }
}

// Java: public class BookmarkLst
impl BookmarkLst {
    // Java: BookmarkLst()
    pub fn BookmarkLst() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.list = new KpList(this.zone);
        BookmarkLst { bin_box: KpBinBox::new(), list: KpList::new(&KoZone), zone: KoZone::defaultZone() }
    }
    // Java: BookmarkLst(KoBin bin)
    pub fn BookmarkLst_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.list = new KpList(bin, this.zone);
        BookmarkLst { bin_box: KpBinBox::KpBinBox_from(Some(bin)), list: KpList::new_from(bin, &KoZone), zone: KoZone::defaultZone() }
    }
    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.list.load(this.box.get(), zone);
        let b = self.bin_box.get();
        self.list.load(b.as_ref(), zone);
    }
    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, &KoZone);
    }
    // Java: void load(BookmarkLst value)
    pub fn load_from(&mut self, value: Option<&BookmarkLst>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
        if self.list.dirty() {
            let saved = self.list.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(BookmarkLst o)
    pub fn equalsLax(&self, o: &BookmarkLst) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &BookmarkLst) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &BookmarkLst, isStrict: bool) -> bool {
        // Java: if (null == o)
        // Java: return false;
        // Java: else
        // Java: BookmarkLst dst = (BookmarkLst)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: ArrayList<Bookmark> dstList = new ArrayList();
            let mut dstList: Vec<Box<dyn KoBinTrait>> = Vec::new();
            // Java: dst.read(dstList);
            dst.read(&mut dstList);
            // Java: ArrayList<Bookmark> srcList = new ArrayList();
            let mut srcList: Vec<Box<dyn KoBinTrait>> = Vec::new();
            // Java: this.read(srcList);
            self.read(&mut srcList);
            // Java: if (dstList.size() != srcList.size())
            if dstList.len() != srcList.len() {
                // Java: return false;
                return false;
            // Java: else
            } else {
                // Java: int size = dstList.size();
                let size = dstList.len();
                // Java: for(int i = 0; i < size; ++i)
                for i in 0..size {
                    // Java: Bookmark dstClrTrans = (Bookmark)dstList.get(i);
                    let dstClrTrans = &dstList[i];
                    // Java: Bookmark srcClrTrans = (Bookmark)srcList.get(i);
                    let srcClrTrans = &srcList[i];
                    // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                    if isStrict && !dstClrTrans.equals(srcClrTrans) {
                        // Java: return false;
                        return false;
                    }
                    // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                    if !isStrict && !dstClrTrans.equalsLax(srcClrTrans) {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: return true;
                return true;
            }
        }
    }
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.list.hashCode();
        todo!()
    }
    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.list.dirty();
        self.list.dirty()
    }
    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(&KoZone);
        // Java: return this.box.get();
        self.bin_box.get()
    }
    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get();
        self.loadBin(bin.as_ref(), zone);
    }
    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone)
    }
    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.list.load(KoBin.Null.value1(), this.zone);
        let n = Null::value1();
        self.list.load(n.as_ref(), &KoZone);
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }
    // Java: static protected static <Value1 extends BookmarkLst> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = BookmarkLst::BookmarkLst();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }
    // Java: static BookmarkLst create()
    pub fn create() -> Self {
        // Java: return initialize(BookmarkLst.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static BookmarkLst open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(BookmarkLst.class, bin);
        Self::initialize(bin)
    }
    // Java: void read(Collection<Bookmark> valueLst)
    pub fn read(&self, _valueLst: &mut Vec<Box<dyn KoBinTrait>>) {
        // Java: valueLst.clear();
        // Java: int size = this.list.size();
        // Java: for(int i = 0; i < size; ++i)
        // Java: valueLst.add((Bookmark)KoBin.Decode.toAny(Bookmark.class, this.list.get(i), this.zone));
        todo!();
    }
    // Java: void write(Iterable<Bookmark> valueLst)
    pub fn write(&mut self, _valueLst: &[Box<dyn KoBinTrait>]) {
        // Java: this.list.clear();
        // Java: for(Bookmark i : valueLst)
        // Java: KoBin bin = i.lastBin(this.zone);
        // Java: if (KoBin.Null.notEmpty(bin))
        // Java: this.list.add(bin);
        todo!();
    }
    // Java: void forceWrite(Iterable<Bookmark> valueLst)
    pub fn forceWrite(&mut self, _valueLst: &[Box<dyn KoBinTrait>]) {
        // Java: this.list.clear();
        // Java: for(Bookmark i : valueLst)
        // Java: this.list.add(i.lastBin(this.zone));
        todo!();
    }
}

impl KoAr for Media {
}

impl KoClosable for Media {
}
