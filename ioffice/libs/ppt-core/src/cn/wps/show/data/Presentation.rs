#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.basic.PositiveSize2D
use crate::cn::wps::drawing::data::basic::PositiveSize2D::PositiveSize2D;
// Java: import cn.wps.drawing.data.font.TextFont
use crate::cn::wps::drawing::data::font::TextFont::TextFont;
// Java: import cn.wps.drawing.data.text.TextListStyle
use crate::cn::wps::drawing::data::text::TextListStyle::TextListStyle;
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
// Java: import cn.wps.kso.kproto.util.KoIntList
use crate::cn::wps::kso::kproto::util::KoIntList::KoIntList;
// Java: import cn.wps.show.data.comment.CommentAuthor
// LLM: CommentAuthor module not yet available, using inline stub
// Java: import cn.wps.show.data.docproperty.DocProperty
use crate::cn::wps::show::data::docproperty::DocProperty::DocProperty;
// Java: import cn.wps.show.data.presproperty.PresProperty
use crate::cn::wps::show::data::presproperty::PresProperty::PresProperty;
// Java: import cn.wps.show.data.viewproperty.ViewProperty
// LLM: ViewProperty module not yet available, using inline stub
// Java: import java.util.ArrayList
// LLM Rust No Need: using Vec instead
// Java: import java.util.Collection
// LLM Rust No Need: using Vec/slice instead

pub struct CommentAuthor { zone: &'static KoZone, bin_box: KpBinBox, map: KpMap }
impl CommentAuthor {
    pub fn new() -> Self { Self { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new() } }
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self { Self { zone, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin) } }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Factory::dup(self.bin_box.get().as_ref(), zone) }
    pub fn equals_obj(&mut self, _o: &mut CommentAuthor) -> bool { false }
    pub fn equalsLax(&mut self, _o: &mut CommentAuthor) -> bool { false }
}

pub struct ViewProperty { zone: &'static KoZone, bin_box: KpBinBox, map: KpMap }
impl ViewProperty {
    pub fn new() -> Self { Self { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new() } }
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self { Self { zone, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin) } }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Factory::dup(self.bin_box.get().as_ref(), zone) }
    pub fn dirty(&self) -> bool { self.map.dirty() }
    pub fn saveBin(&mut self, _zone: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    pub fn hashCode(&mut self) -> i32 { 0 }
    pub fn equals_obj(&mut self, _o: &mut ViewProperty) -> bool { false }
    pub fn equalsLax(&mut self, _o: &mut ViewProperty) -> bool { false }
}

// Java: public class Presentation implements KoAr, KoClosable
pub struct Presentation {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // LLM: CustomShowList is a KpList wrapper, not java.util.List
    // Java field: CustomShowList customShowList
    pub(crate) customShowList: Option<Box<CustomShowList>>, // Java field:customShowList // LLM Not Collection: CustomShowList wraps KpList
    // LLM: TextListStyle is a KpList wrapper, not java.util.List
    // Java field: TextListStyle defaultTextStyle
    pub(crate) defaultTextStyle: Option<Box<TextListStyle>>, // Java field:defaultTextStyle // LLM Not Collection: TextListStyle wraps KpList
    // LLM: EmbeddedFontList is a KpList wrapper, not java.util.List
    // Java field: EmbeddedFontList embeddedFontList
    pub(crate) embeddedFontList: Option<Box<EmbeddedFontList>>, // Java field:embeddedFontList // LLM Not Collection: EmbeddedFontList wraps KpList
    // Java field: Kinsoku kinsoku
    pub(crate) kinsoku: Option<Box<Kinsoku>>,
    // Java field: ModificationVerifier modifyVerifier
    pub(crate) modifyVerifier: Option<Box<ModificationVerifier>>,
    // Java field: PositiveSize2D notesSize
    pub(crate) notesSize: Option<Box<PositiveSize2D>>,
    // Java field: SlideSize slideSize
    pub(crate) slideSize: Option<Box<SlideSize>>,
    // Java field: PhotoAlbum photoAlbum
    pub(crate) photoAlbum: Option<Box<PhotoAlbum>>,
    // Java field: ViewProperty viewProperty
    pub(crate) viewProperty: Option<Box<ViewProperty>>,
    // Java field: DocProperty docProperty
    pub(crate) docProperty: Option<Box<DocProperty>>,
    // Java field: PresProperty presProperty
    pub(crate) presProperty: Option<Box<PresProperty>>,
    // LLM: CommentAuthorList is a KpList wrapper, not java.util.List
    // Java field: CommentAuthorList commentAuthorLst
    pub(crate) commentAuthorLst: Option<Box<CommentAuthorList>>, // Java field:commentAuthorLst // LLM Not Collection: CommentAuthorList wraps KpList
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM Not Collection: KpMap is binary protocol, not java.util.Map
    // Java field: Presentation parent
    pub(crate) parent: Option<Box<Presentation>>,
}

// Java: public class CustomShow
pub struct CustomShow {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // LLM: KoIntList is binary protocol list, not java.util.List
    // Java field: KoIntList slideList
    pub(crate) slideList: Option<Box<KoIntList>>, // Java field:slideList // LLM Not Collection: KoIntList is binary protocol, not java.util.List
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM Not Collection: KpMap is binary protocol, not java.util.Map
    // Java field: CustomShow parent
    pub(crate) parent: Option<Box<CustomShow>>,
}

// Java: public class CustomShowList
pub struct CustomShowList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // LLM: KpList is binary protocol list, not java.util.List
    // Java field: KpList list
    pub(crate) list: KpList, // Java field:list // LLM Not Collection: KpList is binary protocol, not java.util.List
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class EmbeddedFontListEntry
pub struct EmbeddedFontListEntry {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TextFont font
    pub(crate) font: Option<Box<TextFont>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM Not Collection: KpMap is binary protocol, not java.util.Map
    // Java field: EmbeddedFontListEntry parent
    pub(crate) parent: Option<Box<EmbeddedFontListEntry>>, // Java field:parent // LLM Not Collection: EmbeddedFontListEntry is a custom class, not java.util.List
}

// Java: public class EmbeddedFontList
pub struct EmbeddedFontList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // LLM: KpList is binary protocol list, not java.util.List
    // Java field: KpList list
    pub(crate) list: KpList, // Java field:list // LLM Not Collection: KpList is binary protocol, not java.util.List
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class Kinsoku
pub struct Kinsoku {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM Not Collection: KpMap is binary protocol, not java.util.Map
    // Java field: Kinsoku parent
    pub(crate) parent: Option<Box<Kinsoku>>,
}

// Java: public class ModificationVerifier
pub struct ModificationVerifier {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM Not Collection: KpMap is binary protocol, not java.util.Map
    // Java field: ModificationVerifier parent
    pub(crate) parent: Option<Box<ModificationVerifier>>,
}

// Java: public class SlideSize
pub struct SlideSize {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM Not Collection: KpMap is binary protocol, not java.util.Map
    // Java field: SlideSize parent
    pub(crate) parent: Option<Box<SlideSize>>,
}

// Java: public class PhotoAlbum
pub struct PhotoAlbum {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // LLM: KpMap is binary protocol map, not java.util.Map
    // Java field: KpMap map
    pub(crate) map: KpMap, // Java field:map // LLM Not Collection: KpMap is binary protocol, not java.util.Map
    // Java field: PhotoAlbum parent
    pub(crate) parent: Option<Box<PhotoAlbum>>,
}

// Java: public class CommentAuthorList
pub struct CommentAuthorList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // LLM: KpList is binary protocol list, not java.util.List
    // Java field: KpList list
    pub(crate) list: KpList, // Java field:list // LLM Not Collection: KpList is binary protocol, not java.util.List
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

fn java_double_hash(v: f64) -> i32 { let bits = v.to_bits() as i64; (bits ^ (bits >> 32)) as i32 }

// Java: public class Presentation implements KoAr, KoClosable
impl Presentation {
    // Java: static protected static <Value1 extends Presentation> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Presentation::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }
    // Java: static Presentation create()
    pub fn create() -> Self {
      // Java: return initialize(Presentation.class, KoBin.Null.empty());
      let empty = Null::empty(); Self::initialize(empty.as_ref())
    }
    // Java: static Presentation open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Presentation.class, bin);
      Self::initialize(bin)
    }
    // Java: Presentation()
    pub fn new(zone: &'static KoZone) -> Self {
      Self { zone, customShowList: None, defaultTextStyle: None, embeddedFontList: None, kinsoku: None, modifyVerifier: None, notesSize: None, slideSize: None, photoAlbum: None, viewProperty: None, docProperty: None, presProperty: None, commentAuthorLst: None,
        // Java: this.box = new KpBinBox();
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        // LLM: map init in struct literal (no separate assignment needed)
        map: KpMap::new(), parent: None,
      }
    }
    // Java: Presentation(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self { zone, customShowList: None, defaultTextStyle: None, embeddedFontList: None, kinsoku: None, modifyVerifier: None, notesSize: None, slideSize: None, photoAlbum: None, viewProperty: None, docProperty: None, presProperty: None, commentAuthorLst: None,
        // Java: this.box = new KpBinBox(bin);
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        // LLM: map init in struct literal (no separate assignment needed)
        map: KpMap::KpMap_from(bin), parent: None,
      }
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
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone; self.loadBin(bin, zone);
    }
    // Java: void load(Presentation value)
    pub fn load_value(&mut self, value: Option<&Presentation>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
      }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.customShowList != null && this.customShowList.dirty())
      if self.customShowList.is_some() && self.customShowList.as_ref().unwrap().dirty() {
         // Java: this.map.put(13, this.customShowList.saveBin(zone));
         let b = self.customShowList.as_mut().unwrap().saveBin(zone); self.map.put(13, b);
      }
      // Java: if (this.defaultTextStyle != null && this.defaultTextStyle.dirty())
      if self.defaultTextStyle.is_some() && self.defaultTextStyle.as_ref().unwrap().dirty() {
         // Java: this.map.put(14, this.defaultTextStyle.saveBin(zone));
         let b = self.defaultTextStyle.as_mut().unwrap().saveBin(zone); self.map.put(14, b);
      }
      // Java: if (this.embeddedFontList != null && this.embeddedFontList.dirty())
      if self.embeddedFontList.is_some() && self.embeddedFontList.as_ref().unwrap().dirty() {
         // Java: this.map.put(15, this.embeddedFontList.saveBin(zone));
         let b = self.embeddedFontList.as_mut().unwrap().saveBin(zone); self.map.put(15, b);
      }
      // Java: if (this.kinsoku != null && this.kinsoku.dirty())
      if self.kinsoku.is_some() && self.kinsoku.as_ref().unwrap().dirty() {
         // Java: this.map.put(16, this.kinsoku.saveBin(zone));
         let b = self.kinsoku.as_mut().unwrap().saveBin(zone); self.map.put(16, b);
      }
      // Java: if (this.modifyVerifier != null && this.modifyVerifier.dirty())
      if self.modifyVerifier.is_some() && self.modifyVerifier.as_ref().unwrap().dirty() {
         // Java: this.map.put(17, this.modifyVerifier.saveBin(zone));
         let b = self.modifyVerifier.as_mut().unwrap().saveBin(zone); self.map.put(17, b);
      }
      // Java: if (this.notesSize != null && this.notesSize.dirty())
      if self.notesSize.is_some() && self.notesSize.as_ref().unwrap().dirty() {
         // Java: this.map.put(18, this.notesSize.saveBin(zone));
         let b = self.notesSize.as_mut().unwrap().saveBin(zone); self.map.put(18, b);
      }
      // Java: if (this.slideSize != null && this.slideSize.dirty())
      if self.slideSize.is_some() && self.slideSize.as_ref().unwrap().dirty() {
         // Java: this.map.put(19, this.slideSize.saveBin(zone));
         let b = self.slideSize.as_mut().unwrap().saveBin(zone); self.map.put(19, b);
      }
      // Java: if (this.photoAlbum != null && this.photoAlbum.dirty())
      if self.photoAlbum.is_some() && self.photoAlbum.as_ref().unwrap().dirty() {
         // Java: this.map.put(20, this.photoAlbum.saveBin(zone));
         let b = self.photoAlbum.as_mut().unwrap().saveBin(zone); self.map.put(20, b);
      }
      // Java: if (this.viewProperty != null && this.viewProperty.dirty())
      if self.viewProperty.is_some() && self.viewProperty.as_ref().unwrap().dirty() {
         // Java: this.map.put(21, this.viewProperty.saveBin(zone));
         let b = self.viewProperty.as_mut().unwrap().saveBin(zone); self.map.put(21, b);
      }
      // Java: if (this.docProperty != null && this.docProperty.dirty())
      if self.docProperty.is_some() && self.docProperty.as_ref().unwrap().dirty() {
         // Java: this.map.put(22, this.docProperty.saveBin(zone));
         let b = self.docProperty.as_mut().unwrap().saveBin(zone); self.map.put(22, b);
      }
      // Java: if (this.presProperty != null && this.presProperty.dirty())
      if self.presProperty.is_some() && self.presProperty.as_ref().unwrap().dirty() {
         // Java: this.map.put(23, this.presProperty.saveBin(zone));
         let b = self.presProperty.as_mut().unwrap().saveBin(zone); self.map.put(23, b);
      }
      // Java: if (this.commentAuthorLst != null && this.commentAuthorLst.dirty())
      if self.commentAuthorLst.is_some() && self.commentAuthorLst.as_ref().unwrap().dirty() {
         // Java: this.map.put(24, this.commentAuthorLst.saveBin(zone));
         let b = self.commentAuthorLst.as_mut().unwrap().saveBin(zone); self.map.put(24, b);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }
    // Java: boolean equalsLax(Presentation o)
    pub fn equalsLax(&mut self, o: &mut Presentation) -> bool {
      // Java: return this.equals(o, false);
      self.equals(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals_obj(&mut self, o: &mut Presentation) -> bool {
      // Java: return this.equals(o, true);
      self.equals(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals(&mut self, o: &mut Presentation, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Presentation)
      if true && true {
         // Java: Presentation dst = (Presentation)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasAutoCompressPictures() || !dst.hasAutoCompressPictures()) && (!this.hasAutoCompressPictures() || dst.hasAutoCompressPictures()))
         else if (self.hasAutoCompressPictures() || !dst.hasAutoCompressPictures()) && (!self.hasAutoCompressPictures() || dst.hasAutoCompressPictures()) {
            // Java: if (this.hasAutoCompressPictures() && dst.hasAutoCompressPictures() && this.isAutoCompressPictures() != dst.isAutoCompressPictures())
            if self.hasAutoCompressPictures() && dst.hasAutoCompressPictures() && self.isAutoCompressPictures() != dst.isAutoCompressPictures() {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasBookmardIdSeed() || !dst.hasBookmardIdSeed()) && (!this.hasBookmardIdSeed() || dst.hasBookmardIdSeed()))
            else if (self.hasBookmardIdSeed() || !dst.hasBookmardIdSeed()) && (!self.hasBookmardIdSeed() || dst.hasBookmardIdSeed()) {
               // Java: if (this.hasBookmardIdSeed() && dst.hasBookmardIdSeed() && this.bookmardIdSeed() != dst.bookmardIdSeed())
               if self.hasBookmardIdSeed() && dst.hasBookmardIdSeed() && self.bookmardIdSeed() != dst.bookmardIdSeed() {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasCompatMode() || !dst.hasCompatMode()) && (!this.hasCompatMode() || dst.hasCompatMode()))
               else if (self.hasCompatMode() || !dst.hasCompatMode()) && (!self.hasCompatMode() || dst.hasCompatMode()) {
                  // Java: if (this.hasCompatMode() && dst.hasCompatMode() && this.isCompatMode() != dst.isCompatMode())
                  if self.hasCompatMode() && dst.hasCompatMode() && self.isCompatMode() != dst.isCompatMode() {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasConformace() || !dst.hasConformace()) && (!this.hasConformace() || dst.hasConformace()))
                  else if (self.hasConformace() || !dst.hasConformace()) && (!self.hasConformace() || dst.hasConformace()) {
                     // Java: if (this.hasConformace() && dst.hasConformace() && this.conformace() != dst.conformace())
                     if self.hasConformace() && dst.hasConformace() && self.conformace() != dst.conformace() {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasEmbedTrueTypeFonts() || !dst.hasEmbedTrueTypeFonts()) && (!this.hasEmbedTrueTypeFonts() || dst.hasEmbedTrueTypeFonts()))
                     else if (self.hasEmbedTrueTypeFonts() || !dst.hasEmbedTrueTypeFonts()) && (!self.hasEmbedTrueTypeFonts() || dst.hasEmbedTrueTypeFonts()) {
                        // Java: if (this.hasEmbedTrueTypeFonts() && dst.hasEmbedTrueTypeFonts() && this.isEmbedTrueTypeFonts() != dst.isEmbedTrueTypeFonts())
                        if self.hasEmbedTrueTypeFonts() && dst.hasEmbedTrueTypeFonts() && self.isEmbedTrueTypeFonts() != dst.isEmbedTrueTypeFonts() {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasFirstSlideNum() || !dst.hasFirstSlideNum()) && (!this.hasFirstSlideNum() || dst.hasFirstSlideNum()))
                        else if (self.hasFirstSlideNum() || !dst.hasFirstSlideNum()) && (!self.hasFirstSlideNum() || dst.hasFirstSlideNum()) {
                           // Java: if (this.hasFirstSlideNum() && dst.hasFirstSlideNum() && this.firstSlideNum() != dst.firstSlideNum())
                           if self.hasFirstSlideNum() && dst.hasFirstSlideNum() && self.firstSlideNum() != dst.firstSlideNum() {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasRemovePersonalInfoOnSave() || !dst.hasRemovePersonalInfoOnSave()) && (!this.hasRemovePersonalInfoOnSave() || dst.hasRemovePersonalInfoOnSave()))
                           else if (self.hasRemovePersonalInfoOnSave() || !dst.hasRemovePersonalInfoOnSave()) && (!self.hasRemovePersonalInfoOnSave() || dst.hasRemovePersonalInfoOnSave()) {
                              // Java: if (this.hasRemovePersonalInfoOnSave() && dst.hasRemovePersonalInfoOnSave() && this.isRemovePersonalInfoOnSave() != dst.isRemovePersonalInfoOnSave())
                              if self.hasRemovePersonalInfoOnSave() && dst.hasRemovePersonalInfoOnSave() && self.isRemovePersonalInfoOnSave() != dst.isRemovePersonalInfoOnSave() {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: else if ((this.hasRightToLeft() || !dst.hasRightToLeft()) && (!this.hasRightToLeft() || dst.hasRightToLeft()))
                              else if (self.hasRightToLeft() || !dst.hasRightToLeft()) && (!self.hasRightToLeft() || dst.hasRightToLeft()) {
                                 // Java: if (this.hasRightToLeft() && dst.hasRightToLeft() && this.isRightToLeft() != dst.isRightToLeft())
                                 if self.hasRightToLeft() && dst.hasRightToLeft() && self.isRightToLeft() != dst.isRightToLeft() {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: else if ((this.hasSaveSubsetFonts() || !dst.hasSaveSubsetFonts()) && (!this.hasSaveSubsetFonts() || dst.hasSaveSubsetFonts()))
                                 else if (self.hasSaveSubsetFonts() || !dst.hasSaveSubsetFonts()) && (!self.hasSaveSubsetFonts() || dst.hasSaveSubsetFonts()) {
                                    // Java: if (this.hasSaveSubsetFonts() && dst.hasSaveSubsetFonts() && this.isSaveSubsetFonts() != dst.isSaveSubsetFonts())
                                    if self.hasSaveSubsetFonts() && dst.hasSaveSubsetFonts() && self.isSaveSubsetFonts() != dst.isSaveSubsetFonts() {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: else if ((this.hasServerZoom() || !dst.hasServerZoom()) && (!this.hasServerZoom() || dst.hasServerZoom()))
                                    else if (self.hasServerZoom() || !dst.hasServerZoom()) && (!self.hasServerZoom() || dst.hasServerZoom()) {
                                       // Java: if (this.hasServerZoom() && dst.hasServerZoom() && this.serverZoom() != dst.serverZoom())
                                       if self.hasServerZoom() && dst.hasServerZoom() && self.serverZoom() != dst.serverZoom() {
                                          // Java: return false;
                                          return false;
                                       }
                                       // Java: else if ((this.hasShowSpecialPlsOnTitleSld() || !dst.hasShowSpecialPlsOnTitleSld()) && (!this.hasShowSpecialPlsOnTitleSld() || dst.hasShowSpecialPlsOnTitleSld()))
                                       else if (self.hasShowSpecialPlsOnTitleSld() || !dst.hasShowSpecialPlsOnTitleSld()) && (!self.hasShowSpecialPlsOnTitleSld() || dst.hasShowSpecialPlsOnTitleSld()) {
                                          // Java: if (this.hasShowSpecialPlsOnTitleSld() && dst.hasShowSpecialPlsOnTitleSld() && this.isShowSpecialPlsOnTitleSld() != dst.isShowSpecialPlsOnTitleSld())
                                          if self.hasShowSpecialPlsOnTitleSld() && dst.hasShowSpecialPlsOnTitleSld() && self.isShowSpecialPlsOnTitleSld() != dst.isShowSpecialPlsOnTitleSld() {
                                             // Java: return false;
                                             return false;
                                          }
                                          // Java: else if ((this.hasStrictFirstAndLastChars() || !dst.hasStrictFirstAndLastChars()) && (!this.hasStrictFirstAndLastChars() || dst.hasStrictFirstAndLastChars()))
                                          else if (self.hasStrictFirstAndLastChars() || !dst.hasStrictFirstAndLastChars()) && (!self.hasStrictFirstAndLastChars() || dst.hasStrictFirstAndLastChars()) {
                                             // Java: if (this.hasStrictFirstAndLastChars() && dst.hasStrictFirstAndLastChars() && this.isStrictFirstAndLastChars() != dst.isStrictFirstAndLastChars())
                                             if self.hasStrictFirstAndLastChars() && dst.hasStrictFirstAndLastChars() && self.isStrictFirstAndLastChars() != dst.isStrictFirstAndLastChars() {
                                                // Java: return false;
                                                return false;
                                             }
                                             // Java: else if ((this.hasCustomShowList() || !dst.hasCustomShowList()) && (!this.hasCustomShowList() || dst.hasCustomShowList()))
                                             else if (self.hasCustomShowList() || !dst.hasCustomShowList()) && (!self.hasCustomShowList() || dst.hasCustomShowList()) {
                                                // Java: if (this.hasCustomShowList() && dst.hasCustomShowList())
                                                if self.hasCustomShowList() && dst.hasCustomShowList() {
                                                   // Java: if (isStrict && !this.customShowList().equals(dst.customShowList()))
                                                   if isStrict && !self.get_customShowList_mut().equals(dst.get_customShowList_mut()) {
                                                      // Java: return false;
                                                      return false;
                                                   }
                                                   // Java: if (!isStrict && !this.customShowList().equalsLax(dst.customShowList()))
                                                   if !isStrict && !self.get_customShowList_mut().equalsLax(dst.get_customShowList_mut()) {
                                                      // Java: return false;
                                                      return false;
                                                   }
                                                }
                                                // Java: if ((this.hasDefaultTextStyle() || !dst.hasDefaultTextStyle()) && (!this.hasDefaultTextStyle() || dst.hasDefaultTextStyle()))
                                                if (self.hasDefaultTextStyle() || !dst.hasDefaultTextStyle()) && (!self.hasDefaultTextStyle() || dst.hasDefaultTextStyle()) {
                                                   // Java: if (this.hasDefaultTextStyle() && dst.hasDefaultTextStyle())
                                                   if self.hasDefaultTextStyle() && dst.hasDefaultTextStyle() {
                                                      // Java: if (isStrict && !this.defaultTextStyle().equals(dst.defaultTextStyle()))
                                                      if isStrict && !self.get_defaultTextStyle_mut().equals(dst.get_defaultTextStyle_mut()) {
                                                         // Java: return false;
                                                         return false;
                                                      }
                                                      // Java: if (!isStrict && !this.defaultTextStyle().equalsLax(dst.defaultTextStyle()))
                                                      if !isStrict && !self.get_defaultTextStyle_mut().equalsLax(dst.get_defaultTextStyle_mut()) {
                                                         // Java: return false;
                                                         return false;
                                                      }
                                                   }
                                                   // Java: if ((this.hasEmbeddedFontList() || !dst.hasEmbeddedFontList()) && (!this.hasEmbeddedFontList() || dst.hasEmbeddedFontList()))
                                                   if (self.hasEmbeddedFontList() || !dst.hasEmbeddedFontList()) && (!self.hasEmbeddedFontList() || dst.hasEmbeddedFontList()) {
                                                      // Java: if (this.hasEmbeddedFontList() && dst.hasEmbeddedFontList())
                                                      if self.hasEmbeddedFontList() && dst.hasEmbeddedFontList() {
                                                         // Java: if (isStrict && !this.embeddedFontList().equals(dst.embeddedFontList()))
                                                         if isStrict && !self.get_embeddedFontList_mut().equals(dst.get_embeddedFontList_mut()) {
                                                            // Java: return false;
                                                            return false;
                                                         }
                                                         // Java: if (!isStrict && !this.embeddedFontList().equalsLax(dst.embeddedFontList()))
                                                         if !isStrict && !self.get_embeddedFontList_mut().equalsLax(dst.get_embeddedFontList_mut()) {
                                                            // Java: return false;
                                                            return false;
                                                         }
                                                      }
                                                      // Java: if ((this.hasKinsoku() || !dst.hasKinsoku()) && (!this.hasKinsoku() || dst.hasKinsoku()))
                                                      if (self.hasKinsoku() || !dst.hasKinsoku()) && (!self.hasKinsoku() || dst.hasKinsoku()) {
                                                         // Java: if (this.hasKinsoku() && dst.hasKinsoku())
                                                         if self.hasKinsoku() && dst.hasKinsoku() {
                                                            // Java: if (isStrict && !this.kinsoku().equals(dst.kinsoku()))
                                                            if isStrict && !self.get_kinsoku_mut().equals(dst.get_kinsoku_mut()) {
                                                               // Java: return false;
                                                               return false;
                                                            }
                                                            // Java: if (!isStrict && !this.kinsoku().equalsLax(dst.kinsoku()))
                                                            if !isStrict && !self.get_kinsoku_mut().equalsLax(dst.get_kinsoku_mut()) {
                                                               // Java: return false;
                                                               return false;
                                                            }
                                                         }
                                                         // Java: if ((this.hasModifyVerifier() || !dst.hasModifyVerifier()) && (!this.hasModifyVerifier() || dst.hasModifyVerifier()))
                                                         if (self.hasModifyVerifier() || !dst.hasModifyVerifier()) && (!self.hasModifyVerifier() || dst.hasModifyVerifier()) {
                                                            // Java: if (this.hasModifyVerifier() && dst.hasModifyVerifier())
                                                            if self.hasModifyVerifier() && dst.hasModifyVerifier() {
                                                               // Java: if (isStrict && !this.modifyVerifier().equals(dst.modifyVerifier()))
                                                               if isStrict && !self.get_modifyVerifier_mut().equals(dst.get_modifyVerifier_mut()) {
                                                                  // Java: return false;
                                                                  return false;
                                                               }
                                                               // Java: if (!isStrict && !this.modifyVerifier().equalsLax(dst.modifyVerifier()))
                                                               if !isStrict && !self.get_modifyVerifier_mut().equalsLax(dst.get_modifyVerifier_mut()) {
                                                                  // Java: return false;
                                                                  return false;
                                                               }
                                                            }
                                                            // Java: if ((this.hasNotesSize() || !dst.hasNotesSize()) && (!this.hasNotesSize() || dst.hasNotesSize()))
                                                            if (self.hasNotesSize() || !dst.hasNotesSize()) && (!self.hasNotesSize() || dst.hasNotesSize()) {
                                                               // Java: if (this.hasNotesSize() && dst.hasNotesSize())
                                                               if self.hasNotesSize() && dst.hasNotesSize() {
                                                                  // Java: if (isStrict && !this.notesSize().equals(dst.notesSize()))
                                                                  if isStrict && !self.get_notesSize_mut().equals(dst.get_notesSize_mut()) {
                                                                     // Java: return false;
                                                                     return false;
                                                                  }
                                                                  // Java: if (!isStrict && !this.notesSize().equalsLax(dst.notesSize()))
                                                                  if !isStrict && !self.get_notesSize_mut().equalsLax(dst.get_notesSize_mut()) {
                                                                     // Java: return false;
                                                                     return false;
                                                                  }
                                                               }
                                                               // Java: if ((this.hasSlideSize() || !dst.hasSlideSize()) && (!this.hasSlideSize() || dst.hasSlideSize()))
                                                               if (self.hasSlideSize() || !dst.hasSlideSize()) && (!self.hasSlideSize() || dst.hasSlideSize()) {
                                                                  // Java: if (this.hasSlideSize() && dst.hasSlideSize())
                                                                  if self.hasSlideSize() && dst.hasSlideSize() {
                                                                     // Java: if (isStrict && !this.slideSize().equals(dst.slideSize()))
                                                                     if isStrict && !self.get_slideSize_mut().equals(dst.get_slideSize_mut()) {
                                                                        // Java: return false;
                                                                        return false;
                                                                     }
                                                                     // Java: if (!isStrict && !this.slideSize().equalsLax(dst.slideSize()))
                                                                     if !isStrict && !self.get_slideSize_mut().equalsLax(dst.get_slideSize_mut()) {
                                                                        // Java: return false;
                                                                        return false;
                                                                     }
                                                                  }
                                                                  // Java: if ((this.hasPhotoAlbum() || !dst.hasPhotoAlbum()) && (!this.hasPhotoAlbum() || dst.hasPhotoAlbum()))
                                                                  if (self.hasPhotoAlbum() || !dst.hasPhotoAlbum()) && (!self.hasPhotoAlbum() || dst.hasPhotoAlbum()) {
                                                                     // Java: if (this.hasPhotoAlbum() && dst.hasPhotoAlbum())
                                                                     if self.hasPhotoAlbum() && dst.hasPhotoAlbum() {
                                                                        // Java: if (isStrict && !this.photoAlbum().equals(dst.photoAlbum()))
                                                                        if isStrict && !self.get_photoAlbum_mut().equals(dst.get_photoAlbum_mut()) {
                                                                           // Java: return false;
                                                                           return false;
                                                                        }
                                                                        // Java: if (!isStrict && !this.photoAlbum().equalsLax(dst.photoAlbum()))
                                                                        if !isStrict && !self.get_photoAlbum_mut().equalsLax(dst.get_photoAlbum_mut()) {
                                                                           // Java: return false;
                                                                           return false;
                                                                        }
                                                                     }
                                                                     // Java: if ((this.hasViewProperty() || !dst.hasViewProperty()) && (!this.hasViewProperty() || dst.hasViewProperty()))
                                                                     if (self.hasViewProperty() || !dst.hasViewProperty()) && (!self.hasViewProperty() || dst.hasViewProperty()) {
                                                                        // Java: if (this.hasViewProperty() && dst.hasViewProperty())
                                                                        if self.hasViewProperty() && dst.hasViewProperty() {
                                                                           // Java: if (isStrict && !this.viewProperty().equals(dst.viewProperty()))
                                                                           if isStrict && !self.get_viewProperty_mut().equals(dst.get_viewProperty_mut()) {
                                                                              // Java: return false;
                                                                              return false;
                                                                           }
                                                                           // Java: if (!isStrict && !this.viewProperty().equalsLax(dst.viewProperty()))
                                                                           if !isStrict && !self.get_viewProperty_mut().equalsLax(dst.get_viewProperty_mut()) {
                                                                              // Java: return false;
                                                                              return false;
                                                                           }
                                                                        }
                                                                        // Java: if ((this.hasDocProperty() || !dst.hasDocProperty()) && (!this.hasDocProperty() || dst.hasDocProperty()))
                                                                        if (self.hasDocProperty() || !dst.hasDocProperty()) && (!self.hasDocProperty() || dst.hasDocProperty()) {
                                                                           // Java: if (this.hasDocProperty() && dst.hasDocProperty())
                                                                           if self.hasDocProperty() && dst.hasDocProperty() {
                                                                              // Java: if (isStrict && !this.docProperty().equals(dst.docProperty()))
                                                                              if isStrict && !self.get_docProperty_mut().equals(dst.get_docProperty_mut()) {
                                                                                 // Java: return false;
                                                                                 return false;
                                                                              }
                                                                              // Java: if (!isStrict && !this.docProperty().equalsLax(dst.docProperty()))
                                                                              if !isStrict && !self.get_docProperty_mut().equalsLax(dst.get_docProperty_mut()) {
                                                                                 // Java: return false;
                                                                                 return false;
                                                                              }
                                                                           }
                                                                           // Java: if ((this.hasPresProperty() || !dst.hasPresProperty()) && (!this.hasPresProperty() || dst.hasPresProperty()))
                                                                           if (self.hasPresProperty() || !dst.hasPresProperty()) && (!self.hasPresProperty() || dst.hasPresProperty()) {
                                                                              // Java: if (this.hasPresProperty() && dst.hasPresProperty())
                                                                              if self.hasPresProperty() && dst.hasPresProperty() {
                                                                                 // Java: if (isStrict && !this.presProperty().equals(dst.presProperty()))
                                                                                 if isStrict && !self.get_presProperty_mut().equals(dst.get_presProperty_mut()) {
                                                                                    // Java: return false;
                                                                                    return false;
                                                                                 }
                                                                                 // Java: if (!isStrict && !this.presProperty().equalsLax(dst.presProperty()))
                                                                                 if !isStrict && !self.get_presProperty_mut().equalsLax(dst.get_presProperty_mut()) {
                                                                                    // Java: return false;
                                                                                    return false;
                                                                                 }
                                                                              }
                                                                              // Java: if ((this.hasCommentAuthorLst() || !dst.hasCommentAuthorLst()) && (!this.hasCommentAuthorLst() || dst.hasCommentAuthorLst()))
                                                                              if (self.hasCommentAuthorLst() || !dst.hasCommentAuthorLst()) && (!self.hasCommentAuthorLst() || dst.hasCommentAuthorLst()) {
                                                                                 // Java: if (this.hasCommentAuthorLst() && dst.hasCommentAuthorLst())
                                                                                 if self.hasCommentAuthorLst() && dst.hasCommentAuthorLst() {
                                                                                    // Java: if (isStrict && !this.commentAuthorLst().equals(dst.commentAuthorLst()))
                                                                                    if isStrict && !self.get_commentAuthorLst_mut().equals(dst.get_commentAuthorLst_mut()) {
                                                                                       // Java: return false;
                                                                                       return false;
                                                                                    }
                                                                                    // Java: if (!isStrict && !this.commentAuthorLst().equalsLax(dst.commentAuthorLst()))
                                                                                    if !isStrict && !self.get_commentAuthorLst_mut().equalsLax(dst.get_commentAuthorLst_mut()) {
                                                                                       // Java: return false;
                                                                                       return false;
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
    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasAutoCompressPictures())
      if self.hasAutoCompressPictures() {
         // Java: ret += Boolean.valueOf(this.isAutoCompressPictures()).hashCode();
         ret = ret.wrapping_add(if self.isAutoCompressPictures() { 1231 } else { 1237 });
      }
      // Java: if (this.hasBookmardIdSeed())
      if self.hasBookmardIdSeed() {
         // Java: ret += Integer.valueOf(this.bookmardIdSeed()).hashCode();
         ret = ret.wrapping_add(self.bookmardIdSeed());
      }
      // Java: if (this.hasCompatMode())
      if self.hasCompatMode() {
         // Java: ret += Boolean.valueOf(this.isCompatMode()).hashCode();
         ret = ret.wrapping_add(if self.isCompatMode() { 1231 } else { 1237 });
      }
      // Java: if (this.hasConformace())
      if self.hasConformace() {
         // Java: ret += Integer.valueOf(this.conformace()).hashCode();
         ret = ret.wrapping_add(self.conformace());
      }
      // Java: if (this.hasEmbedTrueTypeFonts())
      if self.hasEmbedTrueTypeFonts() {
         // Java: ret += Boolean.valueOf(this.isEmbedTrueTypeFonts()).hashCode();
         ret = ret.wrapping_add(if self.isEmbedTrueTypeFonts() { 1231 } else { 1237 });
      }
      // Java: if (this.hasFirstSlideNum())
      if self.hasFirstSlideNum() {
         // Java: ret += Integer.valueOf(this.firstSlideNum()).hashCode();
         ret = ret.wrapping_add(self.firstSlideNum());
      }
      // Java: if (this.hasRemovePersonalInfoOnSave())
      if self.hasRemovePersonalInfoOnSave() {
         // Java: ret += Boolean.valueOf(this.isRemovePersonalInfoOnSave()).hashCode();
         ret = ret.wrapping_add(if self.isRemovePersonalInfoOnSave() { 1231 } else { 1237 });
      }
      // Java: if (this.hasRightToLeft())
      if self.hasRightToLeft() {
         // Java: ret += Boolean.valueOf(this.isRightToLeft()).hashCode();
         ret = ret.wrapping_add(if self.isRightToLeft() { 1231 } else { 1237 });
      }
      // Java: if (this.hasSaveSubsetFonts())
      if self.hasSaveSubsetFonts() {
         // Java: ret += Boolean.valueOf(this.isSaveSubsetFonts()).hashCode();
         ret = ret.wrapping_add(if self.isSaveSubsetFonts() { 1231 } else { 1237 });
      }
      // Java: if (this.hasServerZoom())
      if self.hasServerZoom() {
         // Java: ret += Double.valueOf(this.serverZoom()).hashCode();
         ret = ret.wrapping_add(java_double_hash(self.serverZoom()));
      }
      // Java: if (this.hasShowSpecialPlsOnTitleSld())
      if self.hasShowSpecialPlsOnTitleSld() {
         // Java: ret += Boolean.valueOf(this.isShowSpecialPlsOnTitleSld()).hashCode();
         ret = ret.wrapping_add(if self.isShowSpecialPlsOnTitleSld() { 1231 } else { 1237 });
      }
      // Java: if (this.hasStrictFirstAndLastChars())
      if self.hasStrictFirstAndLastChars() {
         // Java: ret += Boolean.valueOf(this.isStrictFirstAndLastChars()).hashCode();
         ret = ret.wrapping_add(if self.isStrictFirstAndLastChars() { 1231 } else { 1237 });
      }
      // Java: if (this.hasCustomShowList())
      if self.hasCustomShowList() {
         // Java: ret += this.customShowList().hashCode();
         ret = ret.wrapping_add(self.get_customShowList_mut().hashCode());
      }
      // Java: if (this.hasDefaultTextStyle())
      if self.hasDefaultTextStyle() {
         // Java: ret += this.defaultTextStyle().hashCode();
         ret = ret.wrapping_add(self.get_defaultTextStyle_mut().hashCode());
      }
      // Java: if (this.hasEmbeddedFontList())
      if self.hasEmbeddedFontList() {
         // Java: ret += this.embeddedFontList().hashCode();
         ret = ret.wrapping_add(self.get_embeddedFontList_mut().hashCode());
      }
      // Java: if (this.hasKinsoku())
      if self.hasKinsoku() {
         // Java: ret += this.kinsoku().hashCode();
         ret = ret.wrapping_add(self.get_kinsoku_mut().hashCode());
      }
      // Java: if (this.hasModifyVerifier())
      if self.hasModifyVerifier() {
         // Java: ret += this.modifyVerifier().hashCode();
         ret = ret.wrapping_add(self.get_modifyVerifier_mut().hashCode());
      }
      // Java: if (this.hasNotesSize())
      if self.hasNotesSize() {
         // Java: ret += this.notesSize().hashCode();
         ret = ret.wrapping_add(self.get_notesSize_mut().hashCode());
      }
      // Java: if (this.hasSlideSize())
      if self.hasSlideSize() {
         // Java: ret += this.slideSize().hashCode();
         ret = ret.wrapping_add(self.get_slideSize_mut().hashCode());
      }
      // Java: if (this.hasPhotoAlbum())
      if self.hasPhotoAlbum() {
         // Java: ret += this.photoAlbum().hashCode();
         ret = ret.wrapping_add(self.get_photoAlbum_mut().hashCode());
      }
      // Java: if (this.hasViewProperty())
      if self.hasViewProperty() {
         // Java: ret += this.viewProperty().hashCode();
         ret = ret.wrapping_add(self.get_viewProperty_mut().hashCode());
      }
      // Java: if (this.hasDocProperty())
      if self.hasDocProperty() {
         // Java: ret += this.docProperty().hashCode();
         ret = ret.wrapping_add(self.get_docProperty_mut().hashCode());
      }
      // Java: if (this.hasPresProperty())
      if self.hasPresProperty() {
         // Java: ret += this.presProperty().hashCode();
         ret = ret.wrapping_add(self.get_presProperty_mut().hashCode());
      }
      // Java: if (this.hasCommentAuthorLst())
      if self.hasCommentAuthorLst() {
         // Java: ret += this.commentAuthorLst().hashCode();
         ret = ret.wrapping_add(self.get_commentAuthorLst_mut().hashCode());
      }
      // Java: return ret;
      ret
    }
    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.customShowList != null && this.customShowList.dirty())
      if self.customShowList.is_some() && self.customShowList.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.defaultTextStyle != null && this.defaultTextStyle.dirty())
      else if self.defaultTextStyle.is_some() && self.defaultTextStyle.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.embeddedFontList != null && this.embeddedFontList.dirty())
      else if self.embeddedFontList.is_some() && self.embeddedFontList.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.kinsoku != null && this.kinsoku.dirty())
      else if self.kinsoku.is_some() && self.kinsoku.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.modifyVerifier != null && this.modifyVerifier.dirty())
      else if self.modifyVerifier.is_some() && self.modifyVerifier.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.notesSize != null && this.notesSize.dirty())
      else if self.notesSize.is_some() && self.notesSize.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.slideSize != null && this.slideSize.dirty())
      else if self.slideSize.is_some() && self.slideSize.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.photoAlbum != null && this.photoAlbum.dirty())
      else if self.photoAlbum.is_some() && self.photoAlbum.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.viewProperty != null && this.viewProperty.dirty())
      else if self.viewProperty.is_some() && self.viewProperty.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.docProperty != null && this.docProperty.dirty())
      else if self.docProperty.is_some() && self.docProperty.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else if (this.presProperty != null && this.presProperty.dirty())
      else if self.presProperty.is_some() && self.presProperty.as_ref().unwrap().dirty() {
         // Java: return true;
         return true;
      }
      // Java: else
      else {
         // Java: return this.commentAuthorLst != null && this.commentAuthorLst.dirty() ? true : this.map.dirty();
         if self.commentAuthorLst.is_some() && self.commentAuthorLst.as_ref().unwrap().dirty() { true } else { self.map.dirty() }
      }
    }
    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone; self.saveBin(zone);
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
    pub fn cleanCache(&mut self) {
      // Java: this.customShowList = null;
      self.customShowList = None;
      // Java: this.defaultTextStyle = null;
      self.defaultTextStyle = None;
      // Java: this.embeddedFontList = null;
      self.embeddedFontList = None;
      // Java: this.kinsoku = null;
      self.kinsoku = None;
      // Java: this.modifyVerifier = null;
      self.modifyVerifier = None;
      // Java: this.notesSize = null;
      self.notesSize = None;
      // Java: this.slideSize = null;
      self.slideSize = None;
      // Java: this.photoAlbum = null;
      self.photoAlbum = None;
      // Java: this.viewProperty = null;
      self.viewProperty = None;
      // Java: this.docProperty = null;
      self.docProperty = None;
      // Java: this.presProperty = null;
      self.presProperty = None;
      // Java: this.commentAuthorLst = null;
      self.commentAuthorLst = None;
    }
    // Java: void follow(Presentation parent)
    pub fn follow(&mut self, parent: Option<Box<Presentation>>) {
      // Java: if (parent != null)
      if let Some(mut p) = parent {
         // Java: parent.discard();
         p.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = Some(p);
      }
    }
    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
      if self.parent.is_some() {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { p.discard(); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }
    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }
    // Java: Presentation parent()
    pub fn parent(&self) -> Option<&Presentation> {
      // Java: return this.parent;
      self.parent.as_deref()
    }
    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      // LLM Rust Renamed: Rust cannot overload methods, using self.discard_with_zone
      let zone = self.zone; self.discard_with_zone(zone);
    }
    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: boolean isAutoCompressPictures()
    pub fn isAutoCompressPictures(&self) -> bool {
      // Java: if (!this.map.has(1))
      if !self.map.has(1) {
         // Java: return this.parent == null ? true : this.parent.isAutoCompressPictures();
         return if self.parent.is_none() { true } else { self.parent.as_ref().unwrap().isAutoCompressPictures() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }
    // Java: boolean hasAutoCompressPictures()
    pub fn hasAutoCompressPictures(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasAutoCompressPictures();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasAutoCompressPictures())
    }
    // Java: void deleteAutoCompressPictures()
    pub fn deleteAutoCompressPictures(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }
    // Java: void putAutoCompressPictures(boolean value)
    pub fn putAutoCompressPictures(&mut self, value: bool) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_bool(value, self.zone));
    }

    // Java: int bookmardIdSeed()
    pub fn bookmardIdSeed(&self) -> i32 {
      // Java: if (!this.map.has(2))
      if !self.map.has(2) {
         // Java: return this.parent == null ? 1 : this.parent.bookmardIdSeed();
         return if self.parent.is_none() { 1 } else { self.parent.as_ref().unwrap().bookmardIdSeed() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }
    // Java: boolean hasBookmardIdSeed()
    pub fn hasBookmardIdSeed(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasBookmardIdSeed();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasBookmardIdSeed())
    }
    // Java: void deleteBookmardIdSeed()
    pub fn deleteBookmardIdSeed(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }
    // Java: void putBookmardIdSeed(int value)
    pub fn putBookmardIdSeed(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(2, Encode::from_int(value, self.zone));
    }

    // Java: boolean isCompatMode()
    pub fn isCompatMode(&self) -> bool {
      // Java: if (!this.map.has(3))
      if !self.map.has(3) {
         // Java: return this.parent == null ? false : this.parent.isCompatMode();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isCompatMode() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }
    // Java: boolean hasCompatMode()
    pub fn hasCompatMode(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasCompatMode();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasCompatMode())
    }
    // Java: void deleteCompatMode()
    pub fn deleteCompatMode(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }
    // Java: void putCompatMode(boolean value)
    pub fn putCompatMode(&mut self, value: bool) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(3, Encode::from_bool(value, self.zone));
    }

    // Java: int conformace()
    pub fn conformace(&self) -> i32 {
      // Java: if (!this.map.has(4))
      if !self.map.has(4) {
         // Java: return this.parent == null ? 1 : this.parent.conformace();
         return if self.parent.is_none() { 1 } else { self.parent.as_ref().unwrap().conformace() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(4);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }
    // Java: boolean hasConformace()
    pub fn hasConformace(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasConformace();
      self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasConformace())
    }
    // Java: void deleteConformace()
    pub fn deleteConformace(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
    }
    // Java: void putConformace(int value)
    pub fn putConformace(&mut self, value: i32) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(4, Encode::from_int(value, self.zone));
    }

    // Java: boolean isEmbedTrueTypeFonts()
    pub fn isEmbedTrueTypeFonts(&self) -> bool {
      // Java: if (!this.map.has(5))
      if !self.map.has(5) {
         // Java: return this.parent == null ? false : this.parent.isEmbedTrueTypeFonts();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isEmbedTrueTypeFonts() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(5);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }
    // Java: boolean hasEmbedTrueTypeFonts()
    pub fn hasEmbedTrueTypeFonts(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasEmbedTrueTypeFonts();
      self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasEmbedTrueTypeFonts())
    }
    // Java: void deleteEmbedTrueTypeFonts()
    pub fn deleteEmbedTrueTypeFonts(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
    }
    // Java: void putEmbedTrueTypeFonts(boolean value)
    pub fn putEmbedTrueTypeFonts(&mut self, value: bool) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(5, Encode::from_bool(value, self.zone));
    }

    // Java: int firstSlideNum()
    pub fn firstSlideNum(&self) -> i32 {
      // Java: if (!this.map.has(6))
      if !self.map.has(6) {
         // Java: return this.parent == null ? 1 : this.parent.firstSlideNum();
         return if self.parent.is_none() { 1 } else { self.parent.as_ref().unwrap().firstSlideNum() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(6);
         let bin = self.map.byKey(6);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }
    // Java: boolean hasFirstSlideNum()
    pub fn hasFirstSlideNum(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasFirstSlideNum();
      self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasFirstSlideNum())
    }
    // Java: void deleteFirstSlideNum()
    pub fn deleteFirstSlideNum(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
    }
    // Java: void putFirstSlideNum(int value)
    pub fn putFirstSlideNum(&mut self, value: i32) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(6, Encode::from_int(value, self.zone));
    }

    // Java: boolean isRemovePersonalInfoOnSave()
    pub fn isRemovePersonalInfoOnSave(&self) -> bool {
      // Java: if (!this.map.has(7))
      if !self.map.has(7) {
         // Java: return this.parent == null ? false : this.parent.isRemovePersonalInfoOnSave();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isRemovePersonalInfoOnSave() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(7);
         let bin = self.map.byKey(7);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }
    // Java: boolean hasRemovePersonalInfoOnSave()
    pub fn hasRemovePersonalInfoOnSave(&self) -> bool {
      // Java: return this.map.has(7) || this.parent != null && this.parent.hasRemovePersonalInfoOnSave();
      self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasRemovePersonalInfoOnSave())
    }
    // Java: void deleteRemovePersonalInfoOnSave()
    pub fn deleteRemovePersonalInfoOnSave(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
    }
    // Java: void putRemovePersonalInfoOnSave(boolean value)
    pub fn putRemovePersonalInfoOnSave(&mut self, value: bool) {
      // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
      self.map.put(7, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isRightToLeft()
    pub fn isRightToLeft(&self) -> bool {
      // Java: if (!this.map.has(8))
      if !self.map.has(8) {
         // Java: return this.parent == null ? false : this.parent.isRightToLeft();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isRightToLeft() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(8);
         let bin = self.map.byKey(8);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }
    // Java: boolean hasRightToLeft()
    pub fn hasRightToLeft(&self) -> bool {
      // Java: return this.map.has(8) || this.parent != null && this.parent.hasRightToLeft();
      self.map.has(8) || self.parent.as_ref().map_or(false, |p| p.hasRightToLeft())
    }
    // Java: void deleteRightToLeft()
    pub fn deleteRightToLeft(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
    }
    // Java: void putRightToLeft(boolean value)
    pub fn putRightToLeft(&mut self, value: bool) {
      // Java: this.map.put(8, KoBin.Encode.from(value, this.zone));
      self.map.put(8, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isSaveSubsetFonts()
    pub fn isSaveSubsetFonts(&self) -> bool {
      // Java: if (!this.map.has(9))
      if !self.map.has(9) {
         // Java: return this.parent == null ? false : this.parent.isSaveSubsetFonts();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isSaveSubsetFonts() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(9);
         let bin = self.map.byKey(9);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }
    // Java: boolean hasSaveSubsetFonts()
    pub fn hasSaveSubsetFonts(&self) -> bool {
      // Java: return this.map.has(9) || this.parent != null && this.parent.hasSaveSubsetFonts();
      self.map.has(9) || self.parent.as_ref().map_or(false, |p| p.hasSaveSubsetFonts())
    }
    // Java: void deleteSaveSubsetFonts()
    pub fn deleteSaveSubsetFonts(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
    }
    // Java: void putSaveSubsetFonts(boolean value)
    pub fn putSaveSubsetFonts(&mut self, value: bool) {
      // Java: this.map.put(9, KoBin.Encode.from(value, this.zone));
      self.map.put(9, Encode::from_bool(value, self.zone));
    }

    // Java: double serverZoom()
    pub fn serverZoom(&self) -> f64 {
      // Java: if (!this.map.has(10))
      if !self.map.has(10) {
         // Java: return this.parent == null ? (double)0.5F : this.parent.serverZoom();
         return if self.parent.is_none() { 0.5f64 } else { self.parent.as_ref().unwrap().serverZoom() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(10);
         let bin = self.map.byKey(10);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }
    // Java: boolean hasServerZoom()
    pub fn hasServerZoom(&self) -> bool {
      // Java: return this.map.has(10) || this.parent != null && this.parent.hasServerZoom();
      self.map.has(10) || self.parent.as_ref().map_or(false, |p| p.hasServerZoom())
    }
    // Java: void deleteServerZoom()
    pub fn deleteServerZoom(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
    }
    // Java: void putServerZoom(double value)
    pub fn putServerZoom(&mut self, value: f64) {
      // Java: this.map.put(10, KoBin.Encode.from(value, this.zone));
      self.map.put(10, Encode::from_double(value, self.zone));
    }

    // Java: boolean isShowSpecialPlsOnTitleSld()
    pub fn isShowSpecialPlsOnTitleSld(&self) -> bool {
      // Java: if (!this.map.has(11))
      if !self.map.has(11) {
         // Java: return this.parent == null ? true : this.parent.isShowSpecialPlsOnTitleSld();
         return if self.parent.is_none() { true } else { self.parent.as_ref().unwrap().isShowSpecialPlsOnTitleSld() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(11);
         let bin = self.map.byKey(11);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }
    // Java: boolean hasShowSpecialPlsOnTitleSld()
    pub fn hasShowSpecialPlsOnTitleSld(&self) -> bool {
      // Java: return this.map.has(11) || this.parent != null && this.parent.hasShowSpecialPlsOnTitleSld();
      self.map.has(11) || self.parent.as_ref().map_or(false, |p| p.hasShowSpecialPlsOnTitleSld())
    }
    // Java: void deleteShowSpecialPlsOnTitleSld()
    pub fn deleteShowSpecialPlsOnTitleSld(&mut self) {
      // Java: this.map.put(11, (KoBin)null);
      self.map.clear_key(11);
    }
    // Java: void putShowSpecialPlsOnTitleSld(boolean value)
    pub fn putShowSpecialPlsOnTitleSld(&mut self, value: bool) {
      // Java: this.map.put(11, KoBin.Encode.from(value, this.zone));
      self.map.put(11, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isStrictFirstAndLastChars()
    pub fn isStrictFirstAndLastChars(&self) -> bool {
      // Java: if (!this.map.has(12))
      if !self.map.has(12) {
         // Java: return this.parent == null ? true : this.parent.isStrictFirstAndLastChars();
         return if self.parent.is_none() { true } else { self.parent.as_ref().unwrap().isStrictFirstAndLastChars() };
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(12);
         let bin = self.map.byKey(12);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }
    // Java: boolean hasStrictFirstAndLastChars()
    pub fn hasStrictFirstAndLastChars(&self) -> bool {
      // Java: return this.map.has(12) || this.parent != null && this.parent.hasStrictFirstAndLastChars();
      self.map.has(12) || self.parent.as_ref().map_or(false, |p| p.hasStrictFirstAndLastChars())
    }
    // Java: void deleteStrictFirstAndLastChars()
    pub fn deleteStrictFirstAndLastChars(&mut self) {
      // Java: this.map.put(12, (KoBin)null);
      self.map.clear_key(12);
    }
    // Java: void putStrictFirstAndLastChars(boolean value)
    pub fn putStrictFirstAndLastChars(&mut self, value: bool) {
      // Java: this.map.put(12, KoBin.Encode.from(value, this.zone));
      self.map.put(12, Encode::from_bool(value, self.zone));
    }

    // Java: CustomShowList customShowList()
    pub fn get_customShowList(&mut self) -> &CustomShowList {
      // Java: if (this.customShowList == null)
      if self.customShowList.is_none() {
         // Java: if (this.map.has(13))
         if self.map.has(13) {
            // Java: KoBin bin = this.map.byKey(13);
            let bin = self.map.byKey(13);
            // Java: this.customShowList = (CustomShowList)KoBin.Decode.toAny(CustomShowList.class, bin, this.zone);
            self.customShowList = Some(Box::new(CustomShowList::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasCustomShowList())
         else if self.parent.as_ref().map_or(false, |p| p.hasCustomShowList()) {
            // Java: this.customShowList = this.parent.customShowList();
            let bin = self.parent.as_mut().unwrap().get_customShowList().lastBin(self.zone);
            self.customShowList = Some(Box::new(CustomShowList::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.customShowList = (CustomShowList)KoBin.Decode.toAny(CustomShowList.class, bin, this.zone);
            self.customShowList = Some(Box::new(CustomShowList::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.customShowList;
      self.customShowList.as_deref().unwrap()
    }
    pub fn get_customShowList_mut(&mut self) -> &mut CustomShowList { if self.customShowList.is_none() { self.get_customShowList(); } self.customShowList.as_deref_mut().unwrap() }
    // Java: boolean hasCustomShowList()
    pub fn hasCustomShowList(&self) -> bool {
      // Java: return this.map.has(13) || this.customShowList != null && this.customShowList.dirty() || this.parent != null && this.parent.hasCustomShowList();
      self.map.has(13) || self.customShowList.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCustomShowList())
    }
    // Java: void deleteCustomShowList()
    pub fn deleteCustomShowList(&mut self) {
      // Java: this.map.put(13, (KoBin)null);
      self.map.clear_key(13);
      // Java: if (this.customShowList != null)
      if self.customShowList.is_some() {
         // Java: this.customShowList = null;
         self.customShowList = None;
      }
    }
    // Java: void writeCustomShowList(CustomShowList value)
    pub fn writeCustomShowList(&mut self, value: Option<&CustomShowList>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(13, bin);
            self.map.put(13, bin);
            // Java: if (this.customShowList != null)
            if self.customShowList.is_some() {
               // Java: this.customShowList = null;
               self.customShowList = None;
            }
         }
      }
    }
    // Java: void forceWriteCustomShowList(CustomShowList value)
    pub fn forceWriteCustomShowList(&mut self, value: Option<&CustomShowList>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(13, value.lastBin(this.zone));
         self.map.put(13, v.lastBin(self.zone));
         // Java: if (this.customShowList != null)
         if self.customShowList.is_some() {
            // Java: this.customShowList = null;
            self.customShowList = None;
         }
      }
    }

    // Java: TextListStyle defaultTextStyle()
    pub fn get_defaultTextStyle(&mut self) -> &TextListStyle {
      // Java: if (this.defaultTextStyle == null)
      if self.defaultTextStyle.is_none() {
         // Java: if (this.map.has(14))
         if self.map.has(14) {
            // Java: KoBin bin = this.map.byKey(14);
            let bin = self.map.byKey(14);
            // Java: this.defaultTextStyle = (TextListStyle)KoBin.Decode.toAny(TextListStyle.class, bin, this.zone);
            self.defaultTextStyle = Some(Box::new(TextListStyle::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasDefaultTextStyle())
         else if self.parent.as_ref().map_or(false, |p| p.hasDefaultTextStyle()) {
            // Java: this.defaultTextStyle = this.parent.defaultTextStyle();
            let bin = self.parent.as_mut().unwrap().get_defaultTextStyle().lastBin(self.zone);
            self.defaultTextStyle = Some(Box::new(TextListStyle::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.defaultTextStyle = (TextListStyle)KoBin.Decode.toAny(TextListStyle.class, bin, this.zone);
            self.defaultTextStyle = Some(Box::new(TextListStyle::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.defaultTextStyle;
      self.defaultTextStyle.as_deref().unwrap()
    }
    pub fn get_defaultTextStyle_mut(&mut self) -> &mut TextListStyle { if self.defaultTextStyle.is_none() { self.get_defaultTextStyle(); } self.defaultTextStyle.as_deref_mut().unwrap() }
    // Java: boolean hasDefaultTextStyle()
    pub fn hasDefaultTextStyle(&self) -> bool {
      // Java: return this.map.has(14) || this.defaultTextStyle != null && this.defaultTextStyle.dirty() || this.parent != null && this.parent.hasDefaultTextStyle();
      self.map.has(14) || self.defaultTextStyle.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasDefaultTextStyle())
    }
    // Java: void deleteDefaultTextStyle()
    pub fn deleteDefaultTextStyle(&mut self) {
      // Java: this.map.put(14, (KoBin)null);
      self.map.clear_key(14);
      // Java: if (this.defaultTextStyle != null)
      if self.defaultTextStyle.is_some() {
         // Java: this.defaultTextStyle = null;
         self.defaultTextStyle = None;
      }
    }
    // Java: void writeDefaultTextStyle(TextListStyle value)
    pub fn writeDefaultTextStyle(&mut self, value: Option<&TextListStyle>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(14, bin);
            self.map.put(14, bin);
            // Java: if (this.defaultTextStyle != null)
            if self.defaultTextStyle.is_some() {
               // Java: this.defaultTextStyle = null;
               self.defaultTextStyle = None;
            }
         }
      }
    }
    // Java: void forceWriteDefaultTextStyle(TextListStyle value)
    pub fn forceWriteDefaultTextStyle(&mut self, value: Option<&TextListStyle>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(14, value.lastBin(this.zone));
         self.map.put(14, v.lastBin(self.zone));
         // Java: if (this.defaultTextStyle != null)
         if self.defaultTextStyle.is_some() {
            // Java: this.defaultTextStyle = null;
            self.defaultTextStyle = None;
         }
      }
    }

    // Java: EmbeddedFontList embeddedFontList()
    pub fn get_embeddedFontList(&mut self) -> &EmbeddedFontList {
      // Java: if (this.embeddedFontList == null)
      if self.embeddedFontList.is_none() {
         // Java: if (this.map.has(15))
         if self.map.has(15) {
            // Java: KoBin bin = this.map.byKey(15);
            let bin = self.map.byKey(15);
            // Java: this.embeddedFontList = (EmbeddedFontList)KoBin.Decode.toAny(EmbeddedFontList.class, bin, this.zone);
            self.embeddedFontList = Some(Box::new(EmbeddedFontList::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasEmbeddedFontList())
         else if self.parent.as_ref().map_or(false, |p| p.hasEmbeddedFontList()) {
            // Java: this.embeddedFontList = this.parent.embeddedFontList();
            let bin = self.parent.as_mut().unwrap().get_embeddedFontList().lastBin(self.zone);
            self.embeddedFontList = Some(Box::new(EmbeddedFontList::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.embeddedFontList = (EmbeddedFontList)KoBin.Decode.toAny(EmbeddedFontList.class, bin, this.zone);
            self.embeddedFontList = Some(Box::new(EmbeddedFontList::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.embeddedFontList;
      self.embeddedFontList.as_deref().unwrap()
    }
    pub fn get_embeddedFontList_mut(&mut self) -> &mut EmbeddedFontList { if self.embeddedFontList.is_none() { self.get_embeddedFontList(); } self.embeddedFontList.as_deref_mut().unwrap() }
    // Java: boolean hasEmbeddedFontList()
    pub fn hasEmbeddedFontList(&self) -> bool {
      // Java: return this.map.has(15) || this.embeddedFontList != null && this.embeddedFontList.dirty() || this.parent != null && this.parent.hasEmbeddedFontList();
      self.map.has(15) || self.embeddedFontList.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasEmbeddedFontList())
    }
    // Java: void deleteEmbeddedFontList()
    pub fn deleteEmbeddedFontList(&mut self) {
      // Java: this.map.put(15, (KoBin)null);
      self.map.clear_key(15);
      // Java: if (this.embeddedFontList != null)
      if self.embeddedFontList.is_some() {
         // Java: this.embeddedFontList = null;
         self.embeddedFontList = None;
      }
    }
    // Java: void writeEmbeddedFontList(EmbeddedFontList value)
    pub fn writeEmbeddedFontList(&mut self, value: Option<&EmbeddedFontList>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(15, bin);
            self.map.put(15, bin);
            // Java: if (this.embeddedFontList != null)
            if self.embeddedFontList.is_some() {
               // Java: this.embeddedFontList = null;
               self.embeddedFontList = None;
            }
         }
      }
    }
    // Java: void forceWriteEmbeddedFontList(EmbeddedFontList value)
    pub fn forceWriteEmbeddedFontList(&mut self, value: Option<&EmbeddedFontList>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(15, value.lastBin(this.zone));
         self.map.put(15, v.lastBin(self.zone));
         // Java: if (this.embeddedFontList != null)
         if self.embeddedFontList.is_some() {
            // Java: this.embeddedFontList = null;
            self.embeddedFontList = None;
         }
      }
    }

    // Java: Kinsoku kinsoku()
    pub fn get_kinsoku(&mut self) -> &Kinsoku {
      // Java: if (this.kinsoku == null)
      if self.kinsoku.is_none() {
         // Java: if (this.map.has(16))
         if self.map.has(16) {
            // Java: KoBin bin = this.map.byKey(16);
            let bin = self.map.byKey(16);
            // Java: this.kinsoku = (Kinsoku)KoBin.Decode.toAny(Kinsoku.class, bin, this.zone);
            self.kinsoku = Some(Box::new(Kinsoku::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasKinsoku())
         else if self.parent.as_ref().map_or(false, |p| p.hasKinsoku()) {
            // Java: this.kinsoku = this.parent.kinsoku();
            let bin = self.parent.as_mut().unwrap().get_kinsoku().lastBin(self.zone);
            self.kinsoku = Some(Box::new(Kinsoku::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.kinsoku = (Kinsoku)KoBin.Decode.toAny(Kinsoku.class, bin, this.zone);
            self.kinsoku = Some(Box::new(Kinsoku::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.kinsoku;
      self.kinsoku.as_deref().unwrap()
    }
    pub fn get_kinsoku_mut(&mut self) -> &mut Kinsoku { if self.kinsoku.is_none() { self.get_kinsoku(); } self.kinsoku.as_deref_mut().unwrap() }
    // Java: boolean hasKinsoku()
    pub fn hasKinsoku(&self) -> bool {
      // Java: return this.map.has(16) || this.kinsoku != null && this.kinsoku.dirty() || this.parent != null && this.parent.hasKinsoku();
      self.map.has(16) || self.kinsoku.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasKinsoku())
    }
    // Java: void deleteKinsoku()
    pub fn deleteKinsoku(&mut self) {
      // Java: this.map.put(16, (KoBin)null);
      self.map.clear_key(16);
      // Java: if (this.kinsoku != null)
      if self.kinsoku.is_some() {
         // Java: this.kinsoku = null;
         self.kinsoku = None;
      }
    }
    // Java: void writeKinsoku(Kinsoku value)
    pub fn writeKinsoku(&mut self, value: Option<&Kinsoku>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(16, bin);
            self.map.put(16, bin);
            // Java: if (this.kinsoku != null)
            if self.kinsoku.is_some() {
               // Java: this.kinsoku = null;
               self.kinsoku = None;
            }
         }
      }
    }
    // Java: void forceWriteKinsoku(Kinsoku value)
    pub fn forceWriteKinsoku(&mut self, value: Option<&Kinsoku>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(16, value.lastBin(this.zone));
         self.map.put(16, v.lastBin(self.zone));
         // Java: if (this.kinsoku != null)
         if self.kinsoku.is_some() {
            // Java: this.kinsoku = null;
            self.kinsoku = None;
         }
      }
    }

    // Java: ModificationVerifier modifyVerifier()
    pub fn get_modifyVerifier(&mut self) -> &ModificationVerifier {
      // Java: if (this.modifyVerifier == null)
      if self.modifyVerifier.is_none() {
         // Java: if (this.map.has(17))
         if self.map.has(17) {
            // Java: KoBin bin = this.map.byKey(17);
            let bin = self.map.byKey(17);
            // Java: this.modifyVerifier = (ModificationVerifier)KoBin.Decode.toAny(ModificationVerifier.class, bin, this.zone);
            self.modifyVerifier = Some(Box::new(ModificationVerifier::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasModifyVerifier())
         else if self.parent.as_ref().map_or(false, |p| p.hasModifyVerifier()) {
            // Java: this.modifyVerifier = this.parent.modifyVerifier();
            let bin = self.parent.as_mut().unwrap().get_modifyVerifier().lastBin(self.zone);
            self.modifyVerifier = Some(Box::new(ModificationVerifier::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.modifyVerifier = (ModificationVerifier)KoBin.Decode.toAny(ModificationVerifier.class, bin, this.zone);
            self.modifyVerifier = Some(Box::new(ModificationVerifier::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.modifyVerifier;
      self.modifyVerifier.as_deref().unwrap()
    }
    pub fn get_modifyVerifier_mut(&mut self) -> &mut ModificationVerifier { if self.modifyVerifier.is_none() { self.get_modifyVerifier(); } self.modifyVerifier.as_deref_mut().unwrap() }
    // Java: boolean hasModifyVerifier()
    pub fn hasModifyVerifier(&self) -> bool {
      // Java: return this.map.has(17) || this.modifyVerifier != null && this.modifyVerifier.dirty() || this.parent != null && this.parent.hasModifyVerifier();
      self.map.has(17) || self.modifyVerifier.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasModifyVerifier())
    }
    // Java: void deleteModifyVerifier()
    pub fn deleteModifyVerifier(&mut self) {
      // Java: this.map.put(17, (KoBin)null);
      self.map.clear_key(17);
      // Java: if (this.modifyVerifier != null)
      if self.modifyVerifier.is_some() {
         // Java: this.modifyVerifier = null;
         self.modifyVerifier = None;
      }
    }
    // Java: void writeModifyVerifier(ModificationVerifier value)
    pub fn writeModifyVerifier(&mut self, value: Option<&ModificationVerifier>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(17, bin);
            self.map.put(17, bin);
            // Java: if (this.modifyVerifier != null)
            if self.modifyVerifier.is_some() {
               // Java: this.modifyVerifier = null;
               self.modifyVerifier = None;
            }
         }
      }
    }
    // Java: void forceWriteModifyVerifier(ModificationVerifier value)
    pub fn forceWriteModifyVerifier(&mut self, value: Option<&ModificationVerifier>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(17, value.lastBin(this.zone));
         self.map.put(17, v.lastBin(self.zone));
         // Java: if (this.modifyVerifier != null)
         if self.modifyVerifier.is_some() {
            // Java: this.modifyVerifier = null;
            self.modifyVerifier = None;
         }
      }
    }

    // Java: PositiveSize2D notesSize()
    pub fn get_notesSize(&mut self) -> &PositiveSize2D {
      // Java: if (this.notesSize == null)
      if self.notesSize.is_none() {
         // Java: if (this.map.has(18))
         if self.map.has(18) {
            // Java: KoBin bin = this.map.byKey(18);
            let bin = self.map.byKey(18);
            // Java: this.notesSize = (PositiveSize2D)KoBin.Decode.toAny(PositiveSize2D.class, bin, this.zone);
            self.notesSize = Some(Box::new(PositiveSize2D::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasNotesSize())
         else if self.parent.as_ref().map_or(false, |p| p.hasNotesSize()) {
            // Java: this.notesSize = this.parent.notesSize();
            let bin = self.parent.as_mut().unwrap().get_notesSize().lastBin(self.zone);
            self.notesSize = Some(Box::new(PositiveSize2D::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.notesSize = (PositiveSize2D)KoBin.Decode.toAny(PositiveSize2D.class, bin, this.zone);
            self.notesSize = Some(Box::new(PositiveSize2D::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.notesSize;
      self.notesSize.as_deref().unwrap()
    }
    pub fn get_notesSize_mut(&mut self) -> &mut PositiveSize2D { if self.notesSize.is_none() { self.get_notesSize(); } self.notesSize.as_deref_mut().unwrap() }
    // Java: boolean hasNotesSize()
    pub fn hasNotesSize(&self) -> bool {
      // Java: return this.map.has(18) || this.notesSize != null && this.notesSize.dirty() || this.parent != null && this.parent.hasNotesSize();
      self.map.has(18) || self.notesSize.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasNotesSize())
    }
    // Java: void deleteNotesSize()
    pub fn deleteNotesSize(&mut self) {
      // Java: this.map.put(18, (KoBin)null);
      self.map.clear_key(18);
      // Java: if (this.notesSize != null)
      if self.notesSize.is_some() {
         // Java: this.notesSize = null;
         self.notesSize = None;
      }
    }
    // Java: void writeNotesSize(PositiveSize2D value)
    pub fn writeNotesSize(&mut self, value: Option<&PositiveSize2D>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(18, bin);
            self.map.put(18, bin);
            // Java: if (this.notesSize != null)
            if self.notesSize.is_some() {
               // Java: this.notesSize = null;
               self.notesSize = None;
            }
         }
      }
    }
    // Java: void forceWriteNotesSize(PositiveSize2D value)
    pub fn forceWriteNotesSize(&mut self, value: Option<&PositiveSize2D>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(18, value.lastBin(this.zone));
         self.map.put(18, v.lastBin(self.zone));
         // Java: if (this.notesSize != null)
         if self.notesSize.is_some() {
            // Java: this.notesSize = null;
            self.notesSize = None;
         }
      }
    }

    // Java: SlideSize slideSize()
    pub fn get_slideSize(&mut self) -> &SlideSize {
      // Java: if (this.slideSize == null)
      if self.slideSize.is_none() {
         // Java: if (this.map.has(19))
         if self.map.has(19) {
            // Java: KoBin bin = this.map.byKey(19);
            let bin = self.map.byKey(19);
            // Java: this.slideSize = (SlideSize)KoBin.Decode.toAny(SlideSize.class, bin, this.zone);
            self.slideSize = Some(Box::new(SlideSize::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasSlideSize())
         else if self.parent.as_ref().map_or(false, |p| p.hasSlideSize()) {
            // Java: this.slideSize = this.parent.slideSize();
            let bin = self.parent.as_mut().unwrap().get_slideSize().lastBin(self.zone);
            self.slideSize = Some(Box::new(SlideSize::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.slideSize = (SlideSize)KoBin.Decode.toAny(SlideSize.class, bin, this.zone);
            self.slideSize = Some(Box::new(SlideSize::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.slideSize;
      self.slideSize.as_deref().unwrap()
    }
    pub fn get_slideSize_mut(&mut self) -> &mut SlideSize { if self.slideSize.is_none() { self.get_slideSize(); } self.slideSize.as_deref_mut().unwrap() }
    // Java: boolean hasSlideSize()
    pub fn hasSlideSize(&self) -> bool {
      // Java: return this.map.has(19) || this.slideSize != null && this.slideSize.dirty() || this.parent != null && this.parent.hasSlideSize();
      self.map.has(19) || self.slideSize.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSlideSize())
    }
    // Java: void deleteSlideSize()
    pub fn deleteSlideSize(&mut self) {
      // Java: this.map.put(19, (KoBin)null);
      self.map.clear_key(19);
      // Java: if (this.slideSize != null)
      if self.slideSize.is_some() {
         // Java: this.slideSize = null;
         self.slideSize = None;
      }
    }
    // Java: void writeSlideSize(SlideSize value)
    pub fn writeSlideSize(&mut self, value: Option<&SlideSize>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(19, bin);
            self.map.put(19, bin);
            // Java: if (this.slideSize != null)
            if self.slideSize.is_some() {
               // Java: this.slideSize = null;
               self.slideSize = None;
            }
         }
      }
    }
    // Java: void forceWriteSlideSize(SlideSize value)
    pub fn forceWriteSlideSize(&mut self, value: Option<&SlideSize>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(19, value.lastBin(this.zone));
         self.map.put(19, v.lastBin(self.zone));
         // Java: if (this.slideSize != null)
         if self.slideSize.is_some() {
            // Java: this.slideSize = null;
            self.slideSize = None;
         }
      }
    }

    // Java: PhotoAlbum photoAlbum()
    pub fn get_photoAlbum(&mut self) -> &PhotoAlbum {
      // Java: if (this.photoAlbum == null)
      if self.photoAlbum.is_none() {
         // Java: if (this.map.has(20))
         if self.map.has(20) {
            // Java: KoBin bin = this.map.byKey(20);
            let bin = self.map.byKey(20);
            // Java: this.photoAlbum = (PhotoAlbum)KoBin.Decode.toAny(PhotoAlbum.class, bin, this.zone);
            self.photoAlbum = Some(Box::new(PhotoAlbum::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasPhotoAlbum())
         else if self.parent.as_ref().map_or(false, |p| p.hasPhotoAlbum()) {
            // Java: this.photoAlbum = this.parent.photoAlbum();
            let bin = self.parent.as_mut().unwrap().get_photoAlbum().lastBin(self.zone);
            self.photoAlbum = Some(Box::new(PhotoAlbum::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.photoAlbum = (PhotoAlbum)KoBin.Decode.toAny(PhotoAlbum.class, bin, this.zone);
            self.photoAlbum = Some(Box::new(PhotoAlbum::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.photoAlbum;
      self.photoAlbum.as_deref().unwrap()
    }
    pub fn get_photoAlbum_mut(&mut self) -> &mut PhotoAlbum { if self.photoAlbum.is_none() { self.get_photoAlbum(); } self.photoAlbum.as_deref_mut().unwrap() }
    // Java: boolean hasPhotoAlbum()
    pub fn hasPhotoAlbum(&self) -> bool {
      // Java: return this.map.has(20) || this.photoAlbum != null && this.photoAlbum.dirty() || this.parent != null && this.parent.hasPhotoAlbum();
      self.map.has(20) || self.photoAlbum.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPhotoAlbum())
    }
    // Java: void deletePhotoAlbum()
    pub fn deletePhotoAlbum(&mut self) {
      // Java: this.map.put(20, (KoBin)null);
      self.map.clear_key(20);
      // Java: if (this.photoAlbum != null)
      if self.photoAlbum.is_some() {
         // Java: this.photoAlbum = null;
         self.photoAlbum = None;
      }
    }
    // Java: void writePhotoAlbum(PhotoAlbum value)
    pub fn writePhotoAlbum(&mut self, value: Option<&PhotoAlbum>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(20, bin);
            self.map.put(20, bin);
            // Java: if (this.photoAlbum != null)
            if self.photoAlbum.is_some() {
               // Java: this.photoAlbum = null;
               self.photoAlbum = None;
            }
         }
      }
    }
    // Java: void forceWritePhotoAlbum(PhotoAlbum value)
    pub fn forceWritePhotoAlbum(&mut self, value: Option<&PhotoAlbum>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(20, value.lastBin(this.zone));
         self.map.put(20, v.lastBin(self.zone));
         // Java: if (this.photoAlbum != null)
         if self.photoAlbum.is_some() {
            // Java: this.photoAlbum = null;
            self.photoAlbum = None;
         }
      }
    }

    // Java: ViewProperty viewProperty()
    pub fn get_viewProperty(&mut self) -> &ViewProperty {
      // Java: if (this.viewProperty == null)
      if self.viewProperty.is_none() {
         // Java: if (this.map.has(21))
         if self.map.has(21) {
            // Java: KoBin bin = this.map.byKey(21);
            let bin = self.map.byKey(21);
            // Java: this.viewProperty = (ViewProperty)KoBin.Decode.toAny(ViewProperty.class, bin, this.zone);
            self.viewProperty = Some(Box::new(ViewProperty::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasViewProperty())
         else if self.parent.as_ref().map_or(false, |p| p.hasViewProperty()) {
            // Java: this.viewProperty = this.parent.viewProperty();
            let bin = self.parent.as_mut().unwrap().get_viewProperty().lastBin(self.zone);
            self.viewProperty = Some(Box::new(ViewProperty::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.viewProperty = (ViewProperty)KoBin.Decode.toAny(ViewProperty.class, bin, this.zone);
            self.viewProperty = Some(Box::new(ViewProperty::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.viewProperty;
      self.viewProperty.as_deref().unwrap()
    }
    pub fn get_viewProperty_mut(&mut self) -> &mut ViewProperty { if self.viewProperty.is_none() { self.get_viewProperty(); } self.viewProperty.as_deref_mut().unwrap() }
    // Java: boolean hasViewProperty()
    pub fn hasViewProperty(&self) -> bool {
      // Java: return this.map.has(21) || this.viewProperty != null && this.viewProperty.dirty() || this.parent != null && this.parent.hasViewProperty();
      self.map.has(21) || self.viewProperty.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasViewProperty())
    }
    // Java: void deleteViewProperty()
    pub fn deleteViewProperty(&mut self) {
      // Java: this.map.put(21, (KoBin)null);
      self.map.clear_key(21);
      // Java: if (this.viewProperty != null)
      if self.viewProperty.is_some() {
         // Java: this.viewProperty = null;
         self.viewProperty = None;
      }
    }
    // Java: void writeViewProperty(ViewProperty value)
    pub fn writeViewProperty(&mut self, value: Option<&ViewProperty>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(21, bin);
            self.map.put(21, bin);
            // Java: if (this.viewProperty != null)
            if self.viewProperty.is_some() {
               // Java: this.viewProperty = null;
               self.viewProperty = None;
            }
         }
      }
    }
    // Java: void forceWriteViewProperty(ViewProperty value)
    pub fn forceWriteViewProperty(&mut self, value: Option<&ViewProperty>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(21, value.lastBin(this.zone));
         self.map.put(21, v.lastBin(self.zone));
         // Java: if (this.viewProperty != null)
         if self.viewProperty.is_some() {
            // Java: this.viewProperty = null;
            self.viewProperty = None;
         }
      }
    }

    // Java: DocProperty docProperty()
    pub fn get_docProperty(&mut self) -> &DocProperty {
      // Java: if (this.docProperty == null)
      if self.docProperty.is_none() {
         // Java: if (this.map.has(22))
         if self.map.has(22) {
            // Java: KoBin bin = this.map.byKey(22);
            let bin = self.map.byKey(22);
            // Java: this.docProperty = (DocProperty)KoBin.Decode.toAny(DocProperty.class, bin, this.zone);
            self.docProperty = Some(Box::new(DocProperty::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasDocProperty())
         else if self.parent.as_ref().map_or(false, |p| p.hasDocProperty()) {
            // Java: this.docProperty = this.parent.docProperty();
            let bin = self.parent.as_mut().unwrap().get_docProperty().lastBin(self.zone);
            self.docProperty = Some(Box::new(DocProperty::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.docProperty = (DocProperty)KoBin.Decode.toAny(DocProperty.class, bin, this.zone);
            self.docProperty = Some(Box::new(DocProperty::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.docProperty;
      self.docProperty.as_deref().unwrap()
    }
    pub fn get_docProperty_mut(&mut self) -> &mut DocProperty { if self.docProperty.is_none() { self.get_docProperty(); } self.docProperty.as_deref_mut().unwrap() }
    // Java: boolean hasDocProperty()
    pub fn hasDocProperty(&self) -> bool {
      // Java: return this.map.has(22) || this.docProperty != null && this.docProperty.dirty() || this.parent != null && this.parent.hasDocProperty();
      self.map.has(22) || self.docProperty.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasDocProperty())
    }
    // Java: void deleteDocProperty()
    pub fn deleteDocProperty(&mut self) {
      // Java: this.map.put(22, (KoBin)null);
      self.map.clear_key(22);
      // Java: if (this.docProperty != null)
      if self.docProperty.is_some() {
         // Java: this.docProperty = null;
         self.docProperty = None;
      }
    }
    // Java: void writeDocProperty(DocProperty value)
    pub fn writeDocProperty(&mut self, value: Option<&DocProperty>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(22, bin);
            self.map.put(22, bin);
            // Java: if (this.docProperty != null)
            if self.docProperty.is_some() {
               // Java: this.docProperty = null;
               self.docProperty = None;
            }
         }
      }
    }
    // Java: void forceWriteDocProperty(DocProperty value)
    pub fn forceWriteDocProperty(&mut self, value: Option<&DocProperty>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(22, value.lastBin(this.zone));
         self.map.put(22, v.lastBin(self.zone));
         // Java: if (this.docProperty != null)
         if self.docProperty.is_some() {
            // Java: this.docProperty = null;
            self.docProperty = None;
         }
      }
    }

    // Java: PresProperty presProperty()
    pub fn get_presProperty(&mut self) -> &PresProperty {
      // Java: if (this.presProperty == null)
      if self.presProperty.is_none() {
         // Java: if (this.map.has(23))
         if self.map.has(23) {
            // Java: KoBin bin = this.map.byKey(23);
            let bin = self.map.byKey(23);
            // Java: this.presProperty = (PresProperty)KoBin.Decode.toAny(PresProperty.class, bin, this.zone);
            self.presProperty = Some(Box::new(PresProperty::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasPresProperty())
         else if self.parent.as_ref().map_or(false, |p| p.hasPresProperty()) {
            // Java: this.presProperty = this.parent.presProperty();
            let bin = self.parent.as_mut().unwrap().get_presProperty().lastBin(self.zone);
            self.presProperty = Some(Box::new(PresProperty::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.presProperty = (PresProperty)KoBin.Decode.toAny(PresProperty.class, bin, this.zone);
            self.presProperty = Some(Box::new(PresProperty::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.presProperty;
      self.presProperty.as_deref().unwrap()
    }
    pub fn get_presProperty_mut(&mut self) -> &mut PresProperty { if self.presProperty.is_none() { self.get_presProperty(); } self.presProperty.as_deref_mut().unwrap() }
    // Java: boolean hasPresProperty()
    pub fn hasPresProperty(&self) -> bool {
      // Java: return this.map.has(23) || this.presProperty != null && this.presProperty.dirty() || this.parent != null && this.parent.hasPresProperty();
      self.map.has(23) || self.presProperty.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPresProperty())
    }
    // Java: void deletePresProperty()
    pub fn deletePresProperty(&mut self) {
      // Java: this.map.put(23, (KoBin)null);
      self.map.clear_key(23);
      // Java: if (this.presProperty != null)
      if self.presProperty.is_some() {
         // Java: this.presProperty = null;
         self.presProperty = None;
      }
    }
    // Java: void writePresProperty(PresProperty value)
    pub fn writePresProperty(&mut self, value: Option<&PresProperty>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(23, bin);
            self.map.put(23, bin);
            // Java: if (this.presProperty != null)
            if self.presProperty.is_some() {
               // Java: this.presProperty = null;
               self.presProperty = None;
            }
         }
      }
    }
    // Java: void forceWritePresProperty(PresProperty value)
    pub fn forceWritePresProperty(&mut self, value: Option<&PresProperty>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(23, value.lastBin(this.zone));
         self.map.put(23, v.lastBin(self.zone));
         // Java: if (this.presProperty != null)
         if self.presProperty.is_some() {
            // Java: this.presProperty = null;
            self.presProperty = None;
         }
      }
    }

    // Java: CommentAuthorList commentAuthorLst()
    pub fn get_commentAuthorLst(&mut self) -> &CommentAuthorList {
      // Java: if (this.commentAuthorLst == null)
      if self.commentAuthorLst.is_none() {
         // Java: if (this.map.has(24))
         if self.map.has(24) {
            // Java: KoBin bin = this.map.byKey(24);
            let bin = self.map.byKey(24);
            // Java: this.commentAuthorLst = (CommentAuthorList)KoBin.Decode.toAny(CommentAuthorList.class, bin, this.zone);
            self.commentAuthorLst = Some(Box::new(CommentAuthorList::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasCommentAuthorLst())
         else if self.parent.as_ref().map_or(false, |p| p.hasCommentAuthorLst()) {
            // Java: this.commentAuthorLst = this.parent.commentAuthorLst();
            let bin = self.parent.as_mut().unwrap().get_commentAuthorLst().lastBin(self.zone);
            self.commentAuthorLst = Some(Box::new(CommentAuthorList::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.commentAuthorLst = (CommentAuthorList)KoBin.Decode.toAny(CommentAuthorList.class, bin, this.zone);
            self.commentAuthorLst = Some(Box::new(CommentAuthorList::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.commentAuthorLst;
      self.commentAuthorLst.as_deref().unwrap()
    }
    pub fn get_commentAuthorLst_mut(&mut self) -> &mut CommentAuthorList { if self.commentAuthorLst.is_none() { self.get_commentAuthorLst(); } self.commentAuthorLst.as_deref_mut().unwrap() }
    // Java: boolean hasCommentAuthorLst()
    pub fn hasCommentAuthorLst(&self) -> bool {
      // Java: return this.map.has(24) || this.commentAuthorLst != null && this.commentAuthorLst.dirty() || this.parent != null && this.parent.hasCommentAuthorLst();
      self.map.has(24) || self.commentAuthorLst.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCommentAuthorLst())
    }
    // Java: void deleteCommentAuthorLst()
    pub fn deleteCommentAuthorLst(&mut self) {
      // Java: this.map.put(24, (KoBin)null);
      self.map.clear_key(24);
      // Java: if (this.commentAuthorLst != null)
      if self.commentAuthorLst.is_some() {
         // Java: this.commentAuthorLst = null;
         self.commentAuthorLst = None;
      }
    }
    // Java: void writeCommentAuthorLst(CommentAuthorList value)
    pub fn writeCommentAuthorLst(&mut self, value: Option<&CommentAuthorList>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(24, bin);
            self.map.put(24, bin);
            // Java: if (this.commentAuthorLst != null)
            if self.commentAuthorLst.is_some() {
               // Java: this.commentAuthorLst = null;
               self.commentAuthorLst = None;
            }
         }
      }
    }
    // Java: void forceWriteCommentAuthorLst(CommentAuthorList value)
    pub fn forceWriteCommentAuthorLst(&mut self, value: Option<&CommentAuthorList>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(24, value.lastBin(this.zone));
         self.map.put(24, v.lastBin(self.zone));
         // Java: if (this.commentAuthorLst != null)
         if self.commentAuthorLst.is_some() {
            // Java: this.commentAuthorLst = null;
            self.commentAuthorLst = None;
         }
      }
    }
}

// Java: public class CustomShow
impl CustomShow {
    // Java: static protected static <Value1 extends CustomShow> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = CustomShow::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }
    // Java: static CustomShow create()
    pub fn create() -> Self {
      // Java: return initialize(CustomShow.class, KoBin.Null.empty());
      let empty = Null::empty(); Self::initialize(empty.as_ref())
    }
    // Java: static CustomShow open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(CustomShow.class, bin);
      Self::initialize(bin)
    }
    // Java: CustomShow()
    pub fn new(zone: &'static KoZone) -> Self {
      Self { zone, slideList: None,
        // Java: this.box = new KpBinBox();
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        // LLM Rust Pattern: struct literal initialization
        map: KpMap::new(), parent: None,
      }
    }
    // Java: CustomShow(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self { zone, slideList: None,
        // Java: this.box = new KpBinBox(bin);
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        // LLM Rust Pattern: struct literal initialization
        map: KpMap::KpMap_from(bin), parent: None,
      }
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
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone; self.loadBin(bin, zone);
    }
    // Java: void load(CustomShow value)
    pub fn load_value(&mut self, value: Option<&CustomShow>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
      }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.slideList != null && this.slideList.dirty())
      if self.slideList.is_some() && self.slideList.as_ref().unwrap().dirty() {
         // Java: this.map.put(3, this.slideList.saveBin(zone));
         let b = self.slideList.as_mut().unwrap().saveBin(zone); self.map.put(3, b);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }
    // Java: boolean equalsLax(CustomShow o)
    pub fn equalsLax(&mut self, o: &mut CustomShow) -> bool {
      // Java: return this.equals(o, false);
      self.equals(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals_obj(&mut self, o: &mut CustomShow) -> bool {
      // Java: return this.equals(o, true);
      self.equals(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals(&mut self, o: &mut CustomShow, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof CustomShow)
      if true && true {
         // Java: CustomShow dst = (CustomShow)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasId() || !dst.hasId()) && (!this.hasId() || dst.hasId()))
         else if (self.hasId() || !dst.hasId()) && (!self.hasId() || dst.hasId()) {
            // Java: if (this.hasId() && dst.hasId() && this.id() != dst.id())
            if self.hasId() && dst.hasId() && self.id() != dst.id() {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
            else if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName()) {
               // Java: if (this.hasName() && dst.hasName() && !this.name().equals(dst.name()))
               if self.hasName() && dst.hasName() && self.name() != dst.name() {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasSlideList() || !dst.hasSlideList()) && (!this.hasSlideList() || dst.hasSlideList()))
               else if (self.hasSlideList() || !dst.hasSlideList()) && (!self.hasSlideList() || dst.hasSlideList()) {
                  // Java: if (this.hasSlideList() && dst.hasSlideList())
                  if self.hasSlideList() && dst.hasSlideList() {
                     // Java: if (isStrict && !this.slideList().equals(dst.slideList()))
                     if isStrict && !self.get_slideList_mut().equals(dst.get_slideList_mut()) {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.slideList().equalsLax(dst.slideList()))
                     if !isStrict && !self.get_slideList_mut().equalsLax(dst.get_slideList_mut()) {
                        // Java: return false;
                        return false;
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
    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasId())
      if self.hasId() {
         // Java: ret += Integer.valueOf(this.id()).hashCode();
         ret += self.id();
      }
      // Java: if (this.hasName())
      if self.hasName() {
         // Java: ret += String.valueOf(this.name()).hashCode();
         let s = self.name(); ret += crate::cn::wps::kso::kproto::util::string_hash(&s);
      }
      // Java: if (this.hasSlideList())
      if self.hasSlideList() {
         // Java: ret += this.slideList().hashCode();
         ret += self.get_slideList_mut().hashCode();
      }
      // Java: return ret;
      ret
    }
    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.slideList != null && this.slideList.dirty() ? true : this.map.dirty();
      if self.slideList.is_some() && self.slideList.as_ref().unwrap().dirty() { true } else { self.map.dirty() }
    }
    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone; self.saveBin(zone);
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
    fn cleanFollow(&mut self) {
    }
    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.slideList = null;
      self.slideList = None;
    }
    // Java: void follow(CustomShow parent)
    pub fn follow(&mut self, parent: Option<&mut CustomShow>) {
      // Java: if (parent != null)
      if let Some(p) = parent {
         // Java: parent.discard();
         p.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = None; // LLM: parent tracking simplified
      }
    }
    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
      if self.parent.is_some() {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { p.discard(); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }
    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }
    // Java: CustomShow parent()
    pub fn get_parent(&self) -> Option<&CustomShow> {
      // Java: return this.parent;
      self.parent.as_deref()
    }
    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      // LLM Rust Renamed: Rust cannot overload methods, using self.discard_with_zone
      let zone = self.zone; self.discard_with_zone(zone);
    }
    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }
    // Java: int id()
    pub fn id(&self) -> i32 {
      // Java: if (!this.map.has(1))
      if !self.map.has(1) {
         // Java: return this.parent == null ? 0 : this.parent.id();
         if let Some(ref p) = self.parent { p.id() } else { 0 }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }
    // Java: boolean hasId()
    pub fn hasId(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasId();
      self.map.has(1) || self.parent.is_some() && self.parent.as_ref().unwrap().hasId()
    }
    // Java: void deleteId()
    pub fn deleteId(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.put_null(1);
    }
    // Java: void putId(int value)
    pub fn putId(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone); self.map.put(1, bin);
    }
    // Java: String name()
    pub fn name(&self) -> String {
      // Java: if (!this.map.has(2))
      if !self.map.has(2) {
         // Java: return this.parent == null ? "" : this.parent.name();
         if let Some(ref p) = self.parent { p.name() } else { String::new() }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toText(bin);
         Decode::toText(bin.as_ref())
      }
    }
    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasName();
      self.map.has(2) || self.parent.is_some() && self.parent.as_ref().unwrap().hasName()
    }
    // Java: void deleteName()
    pub fn deleteName(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.put_null(2);
    }
    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_text(value, self.zone); self.map.put(2, bin);
    }
    // Java: KoIntList slideList()
    pub fn get_slideList_mut(&mut self) -> &mut KoIntList {
      // Java: if (this.slideList == null)
      if self.slideList.is_none() {
         // Java: if (this.map.has(3))
         if self.map.has(3) {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: this.slideList = (KoIntList)KoBin.Decode.toAny(KoIntList.class, bin, this.zone);
            self.slideList = Some(Box::new(KoIntList::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasSlideList())
         else if self.parent.is_some() && self.parent.as_ref().unwrap().hasSlideList() {
            // Java: this.slideList = this.parent.slideList();
            let bin = self.parent.as_ref().unwrap().slideList.as_ref().unwrap().lastBin(self.zone);
            self.slideList = Some(Box::new(KoIntList::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.slideList = (KoIntList)KoBin.Decode.toAny(KoIntList.class, bin, this.zone);
            self.slideList = Some(Box::new(KoIntList::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.slideList;
      self.slideList.as_mut().unwrap()
    }
    // Java: boolean hasSlideList()
    pub fn hasSlideList(&self) -> bool {
      // Java: return this.map.has(3) || this.slideList != null && this.slideList.dirty() || this.parent != null && this.parent.hasSlideList();
      self.map.has(3) || self.slideList.is_some() && self.slideList.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_ref().unwrap().hasSlideList()
    }
    // Java: void deleteSlideList()
    pub fn deleteSlideList(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.put_null(3);
      // Java: if (this.slideList != null)
      if self.slideList.is_some() {
         // Java: this.slideList = null;
         self.slideList = None;
      }
    }
    // Java: void writeSlideList(KoIntList value)
    pub fn writeSlideList(&mut self, value: Option<&KoIntList>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(3, bin);
            self.map.put(3, bin);
            // Java: if (this.slideList != null)
            if self.slideList.is_some() {
               // Java: this.slideList = null;
               self.slideList = None;
            }
         }
      }
    }
    // Java: void forceWriteSlideList(KoIntList value)
    pub fn forceWriteSlideList(&mut self, value: Option<&KoIntList>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.slideList != null)
         if self.slideList.is_some() {
            // Java: this.slideList = null;
            self.slideList = None;
         }
      }
    }
}

// Java: public class CustomShowList
impl CustomShowList {
    // Java: CustomShowList()
    pub fn new(zone: &'static KoZone) -> Self {
      Self {
        // Java: this.box = new KpBinBox();
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::new(),
        // Java: this.list = new KpList(this.zone);
        // LLM Rust Pattern: struct literal initialization
        list: KpList::new(zone),
        zone,
      }
    }
    // Java: CustomShowList(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self {
        // Java: this.box = new KpBinBox(bin);
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.list = new KpList(bin, this.zone);
        // LLM Rust Pattern: struct literal initialization
        list: KpList::KpList_from(bin, zone),
        zone,
      }
    }
    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.list.load(this.box.get(), zone);
      let b = self.bin_box.get(); self.list.load(b.as_ref(), zone);
    }
    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone; self.loadBin(bin, zone);
    }
    // Java: void load(CustomShowList value)
    pub fn load_value(&mut self, value: Option<&CustomShowList>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
      }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
      if self.list.dirty() { let saved = self.list.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }
    // Java: boolean equalsLax(CustomShowList o)
    pub fn equalsLax(&mut self, o: &mut CustomShowList) -> bool {
      // Java: return this.equals(o, false);
      self.equals(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals_obj(&mut self, o: &mut CustomShowList) -> bool {
      // Java: return this.equals(o, true);
      self.equals(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals(&mut self, o: &mut CustomShowList, isStrict: bool) -> bool {
      // Java: if (null == o)
      if false {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: CustomShowList dst = (CustomShowList)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         }
         // Java: else
         else {
            // Java: ArrayList<CustomShow> dstList = new ArrayList();
            let mut dstList: Vec<CustomShow> = Vec::new();
            // Java: dst.read(dstList);
            dst.read(&mut dstList);
            // Java: ArrayList<CustomShow> srcList = new ArrayList();
            let mut srcList: Vec<CustomShow> = Vec::new();
            // Java: this.read(srcList);
            self.read(&mut srcList);
            // Java: if (dstList.size() != srcList.size())
            if dstList.len() != srcList.len() {
               // Java: return false;
               return false;
            }
            // Java: else
            else {
               // Java: int size = dstList.size();
               let size = dstList.len();
               // Java: for(int i = 0; i < size; ++i)
               for i in 0..size {
                  // Java: CustomShow dstClrTrans = (CustomShow)dstList.get(i);
                  let dstClrTrans = &mut dstList[i];
                  // Java: CustomShow srcClrTrans = (CustomShow)srcList.get(i);
                  let srcClrTrans = &mut srcList[i];
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
    }
}
// Java: public class CustomShowList
impl CustomShowList {
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return this.list.hashCode();
      self.list.hashCode()
    }
    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.list.dirty();
      self.list.dirty()
    }
    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone; self.saveBin(zone);
      // Java: return this.box.get();
      self.bin_box.get()
    }
    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
      // Java: this.loadBin(this.box.get(), zone);
      let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone);
    }
    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      Factory::dup(self.bin_box.get().as_ref(), zone)
    }
    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.list.load(KoBin.Null.value1(), this.zone);
      let n = Null::value1(); self.list.load(n.as_ref(), self.zone);
      // Java: this.zone.leave();
      self.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }
    // Java: static protected static <Value1 extends CustomShowList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = CustomShowList::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }
    // Java: static CustomShowList create()
    pub fn create() -> Self {
      // Java: return initialize(CustomShowList.class, KoBin.Null.empty());
      let empty = Null::empty(); Self::initialize(empty.as_ref())
    }
    // Java: static CustomShowList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(CustomShowList.class, bin);
      Self::initialize(bin)
    }
    // Java: void read(Collection<CustomShow> valueLst)
    pub fn read(&self, valueLst: &mut Vec<CustomShow>) {
      // Java: valueLst.clear();
      valueLst.clear();
      // Java: int size = this.list.size();
      let size = self.list.size();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: valueLst.add((CustomShow)KoBin.Decode.toAny(CustomShow.class, this.list.get(i), this.zone));
         let bin = self.list.get(i); valueLst.push(CustomShow::new_from_bin(bin.as_ref(), self.zone));
      }
    }
    // Java: void write(Iterable<CustomShow> valueLst)
    pub fn write(&mut self, valueLst: &[CustomShow]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(CustomShow i : valueLst)
      for i in valueLst.iter() {
         // Java: KoBin bin = i.lastBin(this.zone);
         let bin = i.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.list.add(bin);
            self.list.add(bin);
         }
      }
    }
    // Java: void forceWrite(Iterable<CustomShow> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[CustomShow]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(CustomShow i : valueLst)
      for i in valueLst.iter() {
         // Java: this.list.add(i.lastBin(this.zone));
         self.list.add(i.lastBin(self.zone));
      }
    }
}

// Java: public class EmbeddedFontListEntry
impl EmbeddedFontListEntry {
    // Java: static protected static <Value1 extends EmbeddedFontListEntry> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = EmbeddedFontListEntry::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }
    // Java: static EmbeddedFontListEntry create()
    pub fn create() -> Self {
      // Java: return initialize(EmbeddedFontListEntry.class, KoBin.Null.empty());
      let empty = Null::empty(); Self::initialize(empty.as_ref())
    }
    // Java: static EmbeddedFontListEntry open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(EmbeddedFontListEntry.class, bin);
      Self::initialize(bin)
    }
    // Java: EmbeddedFontListEntry()
    pub fn new(zone: &'static KoZone) -> Self {
      Self { zone, font: None,
        // Java: this.box = new KpBinBox();
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        // LLM Rust Pattern: struct literal initialization
        map: KpMap::new(), parent: None,
      }
    }
    // Java: EmbeddedFontListEntry(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self { zone, font: None,
        // Java: this.box = new KpBinBox(bin);
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        // LLM Rust Pattern: struct literal initialization
        map: KpMap::KpMap_from(bin), parent: None,
      }
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
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone; self.loadBin(bin, zone);
    }
    // Java: void load(EmbeddedFontListEntry value)
    pub fn load_value(&mut self, value: Option<&EmbeddedFontListEntry>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
      }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.font != null && this.font.dirty())
      if self.font.is_some() && self.font.as_ref().unwrap().dirty() {
         // Java: this.map.put(1, this.font.saveBin(zone));
         let b = self.font.as_mut().unwrap().saveBin(zone); self.map.put(1, b);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }
    // Java: boolean equalsLax(EmbeddedFontListEntry o)
    pub fn equalsLax(&mut self, o: &mut EmbeddedFontListEntry) -> bool {
      // Java: return this.equals(o, false);
      self.equals(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals_obj(&mut self, o: &mut EmbeddedFontListEntry) -> bool {
      // Java: return this.equals(o, true);
      self.equals(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals(&mut self, o: &mut EmbeddedFontListEntry, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof EmbeddedFontListEntry)
      if true && true {
         // Java: EmbeddedFontListEntry dst = (EmbeddedFontListEntry)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasFont() || !dst.hasFont()) && (!this.hasFont() || dst.hasFont()))
         else if (self.hasFont() || !dst.hasFont()) && (!self.hasFont() || dst.hasFont()) {
            // Java: if (this.hasFont() && dst.hasFont())
            if self.hasFont() && dst.hasFont() {
               // Java: if (isStrict && !this.font().equals(dst.font()))
               if isStrict && !self.get_font_mut().equals(dst.get_font_mut()) {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.font().equalsLax(dst.font()))
               if !isStrict && !self.get_font_mut().equalsLax(dst.get_font_mut()) {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasRegular() || !dst.hasRegular()) && (!this.hasRegular() || dst.hasRegular()))
            if (self.hasRegular() || !dst.hasRegular()) && (!self.hasRegular() || dst.hasRegular()) {
               // Java: if (this.hasRegular() && dst.hasRegular() && this.regular() != dst.regular())
               if self.hasRegular() && dst.hasRegular() && self.regular() != dst.regular() {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasBold() || !dst.hasBold()) && (!this.hasBold() || dst.hasBold()))
               else if (self.hasBold() || !dst.hasBold()) && (!self.hasBold() || dst.hasBold()) {
                  // Java: if (this.hasBold() && dst.hasBold() && this.bold() != dst.bold())
                  if self.hasBold() && dst.hasBold() && self.bold() != dst.bold() {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasItalic() || !dst.hasItalic()) && (!this.hasItalic() || dst.hasItalic()))
                  else if (self.hasItalic() || !dst.hasItalic()) && (!self.hasItalic() || dst.hasItalic()) {
                     // Java: if (this.hasItalic() && dst.hasItalic() && this.italic() != dst.italic())
                     if self.hasItalic() && dst.hasItalic() && self.italic() != dst.italic() {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasBoldItalic() || !dst.hasBoldItalic()) && (!this.hasBoldItalic() || dst.hasBoldItalic()))
                     else if (self.hasBoldItalic() || !dst.hasBoldItalic()) && (!self.hasBoldItalic() || dst.hasBoldItalic()) {
                        // Java: return !this.hasBoldItalic() || !dst.hasBoldItalic() || this.boldItalic() == dst.boldItalic();
                        return !self.hasBoldItalic() || !dst.hasBoldItalic() || self.boldItalic() == dst.boldItalic();
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
    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasFont())
      if self.hasFont() {
         // Java: ret += this.font().hashCode();
         ret += self.get_font_mut().hashCode();
      }
      // Java: if (this.hasRegular())
      if self.hasRegular() {
         // Java: ret += Integer.valueOf(this.regular()).hashCode();
         ret += self.regular();
      }
      // Java: if (this.hasBold())
      if self.hasBold() {
         // Java: ret += Integer.valueOf(this.bold()).hashCode();
         ret += self.bold();
      }
      // Java: if (this.hasItalic())
      if self.hasItalic() {
         // Java: ret += Integer.valueOf(this.italic()).hashCode();
         ret += self.italic();
      }
      // Java: if (this.hasBoldItalic())
      if self.hasBoldItalic() {
         // Java: ret += Integer.valueOf(this.boldItalic()).hashCode();
         ret += self.boldItalic();
      }
      // Java: return ret;
      ret
    }
    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.font != null && this.font.dirty() ? true : this.map.dirty();
      if self.font.is_some() && self.font.as_ref().unwrap().dirty() { true } else { self.map.dirty() }
    }
    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone; self.saveBin(zone);
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
    fn cleanFollow(&mut self) {
    }
    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.font = null;
      self.font = None;
    }
    // Java: void follow(EmbeddedFontListEntry parent)
    pub fn follow(&mut self, parent: Option<&mut EmbeddedFontListEntry>) {
      // Java: if (parent != null)
      if let Some(p) = parent {
         // Java: parent.discard();
         p.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = None;
      }
    }
    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
      if self.parent.is_some() {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { p.discard(); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }
    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }
    // Java: EmbeddedFontListEntry parent()
    pub fn get_parent(&self) -> Option<&EmbeddedFontListEntry> {
      // Java: return this.parent;
      self.parent.as_deref()
    }
    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      // LLM Rust Renamed: Rust cannot overload methods, using self.discard_with_zone
      let zone = self.zone; self.discard_with_zone(zone);
    }
    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }
    // Java: TextFont font()
    pub fn get_font_mut(&mut self) -> &mut TextFont {
      // Java: if (this.font == null)
      if self.font.is_none() {
         // Java: if (this.map.has(1))
         if self.map.has(1) {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.font = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
            self.font = Some(Box::new(TextFont::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasFont())
         else if self.parent.is_some() && self.parent.as_ref().unwrap().hasFont() {
            // Java: this.font = this.parent.font();
            let bin = self.parent.as_ref().unwrap().font.as_ref().unwrap().lastBin(self.zone);
            self.font = Some(Box::new(TextFont::new_from_bin(bin.as_ref(), self.zone)));
         }
         // Java: else
         else {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.font = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
            self.font = Some(Box::new(TextFont::new_from_bin(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.font;
      self.font.as_mut().unwrap()
    }
    // Java: boolean hasFont()
    pub fn hasFont(&self) -> bool {
      // Java: return this.map.has(1) || this.font != null && this.font.dirty() || this.parent != null && this.parent.hasFont();
      self.map.has(1) || self.font.is_some() && self.font.as_ref().unwrap().dirty() || self.parent.is_some() && self.parent.as_ref().unwrap().hasFont()
    }
    // Java: void deleteFont()
    pub fn deleteFont(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.put_null(1);
      // Java: if (this.font != null)
      if self.font.is_some() {
         // Java: this.font = null;
         self.font = None;
      }
    }
    // Java: void writeFont(TextFont value)
    pub fn writeFont(&mut self, value: Option<&TextFont>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.map.put(1, bin);
            self.map.put(1, bin);
            // Java: if (this.font != null)
            if self.font.is_some() {
               // Java: this.font = null;
               self.font = None;
            }
         }
      }
    }
    // Java: void forceWriteFont(TextFont value)
    pub fn forceWriteFont(&mut self, value: Option<&TextFont>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.font != null)
         if self.font.is_some() {
            // Java: this.font = null;
            self.font = None;
         }
      }
    }
    // Java: int regular()
    pub fn regular(&self) -> i32 {
      // Java: if (!this.map.has(2))
      if !self.map.has(2) {
         // Java: return this.parent == null ? 0 : this.parent.regular();
         if let Some(ref p) = self.parent { p.regular() } else { 0 }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }
    // Java: boolean hasRegular()
    pub fn hasRegular(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasRegular();
      self.map.has(2) || self.parent.is_some() && self.parent.as_ref().unwrap().hasRegular()
    }
    // Java: void deleteRegular()
    pub fn deleteRegular(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.put_null(2);
    }
    // Java: void putRegular(int value)
    pub fn putRegular(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone); self.map.put(2, bin);
    }
    // Java: int bold()
    pub fn bold(&self) -> i32 {
      // Java: if (!this.map.has(3))
      if !self.map.has(3) {
         // Java: return this.parent == null ? 0 : this.parent.bold();
         if let Some(ref p) = self.parent { p.bold() } else { 0 }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }
    // Java: boolean hasBold()
    pub fn hasBold(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasBold();
      self.map.has(3) || self.parent.is_some() && self.parent.as_ref().unwrap().hasBold()
    }
    // Java: void deleteBold()
    pub fn deleteBold(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.put_null(3);
    }
    // Java: void putBold(int value)
    pub fn putBold(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone); self.map.put(3, bin);
    }
    // Java: int italic()
    pub fn italic(&self) -> i32 {
      // Java: if (!this.map.has(4))
      if !self.map.has(4) {
         // Java: return this.parent == null ? 0 : this.parent.italic();
         if let Some(ref p) = self.parent { p.italic() } else { 0 }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(4);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }
    // Java: boolean hasItalic()
    pub fn hasItalic(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasItalic();
      self.map.has(4) || self.parent.is_some() && self.parent.as_ref().unwrap().hasItalic()
    }
    // Java: void deleteItalic()
    pub fn deleteItalic(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.put_null(4);
    }
    // Java: void putItalic(int value)
    pub fn putItalic(&mut self, value: i32) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone); self.map.put(4, bin);
    }
    // Java: int boldItalic()
    pub fn boldItalic(&self) -> i32 {
      // Java: if (!this.map.has(5))
      if !self.map.has(5) {
         // Java: return this.parent == null ? 0 : this.parent.boldItalic();
         if let Some(ref p) = self.parent { p.boldItalic() } else { 0 }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(5);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }
    // Java: boolean hasBoldItalic()
    pub fn hasBoldItalic(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasBoldItalic();
      self.map.has(5) || self.parent.is_some() && self.parent.as_ref().unwrap().hasBoldItalic()
    }
    // Java: void deleteBoldItalic()
    pub fn deleteBoldItalic(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.put_null(5);
    }
    // Java: void putBoldItalic(int value)
    pub fn putBoldItalic(&mut self, value: i32) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone); self.map.put(5, bin);
    }
}

// Java: public class EmbeddedFontList
impl EmbeddedFontList {
    // Java: EmbeddedFontList()
    pub fn new(zone: &'static KoZone) -> Self {
      Self {
        // Java: this.box = new KpBinBox();
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::new(),
        // Java: this.list = new KpList(this.zone);
        // LLM Rust Pattern: struct literal initialization
        list: KpList::new(zone),
        zone,
      }
    }
    // Java: EmbeddedFontList(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self {
        // Java: this.box = new KpBinBox(bin);
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.list = new KpList(bin, this.zone);
        // LLM Rust Pattern: struct literal initialization
        list: KpList::KpList_from(bin, zone),
        zone,
      }
    }
    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.list.load(this.box.get(), zone);
      let b = self.bin_box.get(); self.list.load(b.as_ref(), zone);
    }
    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone; self.loadBin(bin, zone);
    }
    // Java: void load(EmbeddedFontList value)
    pub fn load_value(&mut self, value: Option<&EmbeddedFontList>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
      }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
      if self.list.dirty() { let saved = self.list.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }
    // Java: boolean equalsLax(EmbeddedFontList o)
    pub fn equalsLax(&mut self, o: &mut EmbeddedFontList) -> bool {
      // Java: return this.equals(o, false);
      self.equals(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals_obj(&mut self, o: &mut EmbeddedFontList) -> bool {
      // Java: return this.equals(o, true);
      self.equals(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals(&mut self, o: &mut EmbeddedFontList, isStrict: bool) -> bool {
      // Java: if (null == o)
      if false {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: EmbeddedFontList dst = (EmbeddedFontList)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         }
         // Java: else
         else {
            // Java: ArrayList<EmbeddedFontListEntry> dstList = new ArrayList();
            let mut dstList: Vec<EmbeddedFontListEntry> = Vec::new();
            // Java: dst.read(dstList);
            dst.read(&mut dstList);
            // Java: ArrayList<EmbeddedFontListEntry> srcList = new ArrayList();
            let mut srcList: Vec<EmbeddedFontListEntry> = Vec::new();
            // Java: this.read(srcList);
            self.read(&mut srcList);
            // Java: if (dstList.size() != srcList.size())
            if dstList.len() != srcList.len() {
               // Java: return false;
               return false;
            }
            // Java: else
            else {
               // Java: int size = dstList.size();
               let size = dstList.len();
               // Java: for(int i = 0; i < size; ++i)
               for i in 0..size {
                  // Java: EmbeddedFontListEntry dstClrTrans = (EmbeddedFontListEntry)dstList.get(i);
                  let dstClrTrans = &mut dstList[i];
                  // Java: EmbeddedFontListEntry srcClrTrans = (EmbeddedFontListEntry)srcList.get(i);
                  let srcClrTrans = &mut srcList[i];
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
    }
}
// Java: public class EmbeddedFontList
impl EmbeddedFontList {
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return this.list.hashCode();
      self.list.hashCode()
    }
    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.list.dirty();
      self.list.dirty()
    }
    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone; self.saveBin(zone);
      // Java: return this.box.get();
      self.bin_box.get()
    }
    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
      // Java: this.loadBin(this.box.get(), zone);
      let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone);
    }
    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      Factory::dup(self.bin_box.get().as_ref(), zone)
    }
    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.list.load(KoBin.Null.value1(), this.zone);
      let n = Null::value1(); self.list.load(n.as_ref(), self.zone);
      // Java: this.zone.leave();
      self.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }
    // Java: static protected static <Value1 extends EmbeddedFontList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = EmbeddedFontList::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }
    // Java: static EmbeddedFontList create()
    pub fn create() -> Self {
      // Java: return initialize(EmbeddedFontList.class, KoBin.Null.empty());
      let empty = Null::empty(); Self::initialize(empty.as_ref())
    }
    // Java: static EmbeddedFontList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(EmbeddedFontList.class, bin);
      Self::initialize(bin)
    }
    // Java: void read(Collection<EmbeddedFontListEntry> valueLst)
    pub fn read(&self, valueLst: &mut Vec<EmbeddedFontListEntry>) {
      // Java: valueLst.clear();
      valueLst.clear();
      // Java: int size = this.list.size();
      let size = self.list.size();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: valueLst.add((EmbeddedFontListEntry)KoBin.Decode.toAny(EmbeddedFontListEntry.class, this.list.get(i), this.zone));
         let bin = self.list.get(i); valueLst.push(EmbeddedFontListEntry::new_from_bin(bin.as_ref(), self.zone));
      }
    }
    // Java: void write(Iterable<EmbeddedFontListEntry> valueLst)
    pub fn write(&mut self, valueLst: &[EmbeddedFontListEntry]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(EmbeddedFontListEntry i : valueLst)
      for i in valueLst.iter() {
         // Java: KoBin bin = i.lastBin(this.zone);
         let bin = i.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.list.add(bin);
            self.list.add(bin);
         }
      }
    }
    // Java: void forceWrite(Iterable<EmbeddedFontListEntry> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[EmbeddedFontListEntry]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(EmbeddedFontListEntry i : valueLst)
      for i in valueLst.iter() {
         // Java: this.list.add(i.lastBin(this.zone));
         self.list.add(i.lastBin(self.zone));
      }
    }
}

// Java: public class Kinsoku
impl Kinsoku {
    // Java: static protected static <Value1 extends Kinsoku> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Kinsoku::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }
    // Java: static Kinsoku create()
    pub fn create() -> Self {
      // Java: return initialize(Kinsoku.class, KoBin.Null.empty());
      let empty = Null::empty(); Self::initialize(empty.as_ref())
    }
    // Java: static Kinsoku open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Kinsoku.class, bin);
      Self::initialize(bin)
    }
    // Java: Kinsoku()
    pub fn new(zone: &'static KoZone) -> Self {
      Self { zone,
        // Java: this.box = new KpBinBox();
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        // LLM Rust Pattern: struct literal initialization
        map: KpMap::new(), parent: None,
      }
    }
    // Java: Kinsoku(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self { zone,
        // Java: this.box = new KpBinBox(bin);
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        // LLM Rust Pattern: struct literal initialization
        map: KpMap::KpMap_from(bin), parent: None,
      }
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
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone; self.loadBin(bin, zone);
    }
    // Java: void load(Kinsoku value)
    pub fn load_value(&mut self, value: Option<&Kinsoku>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
      }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }
    // Java: boolean equalsLax(Kinsoku o)
    pub fn equalsLax(&mut self, o: &mut Kinsoku) -> bool {
      // Java: return this.equals(o, false);
      self.equals(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals_obj(&mut self, o: &mut Kinsoku) -> bool {
      // Java: return this.equals(o, true);
      self.equals(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals(&mut self, o: &mut Kinsoku, _isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Kinsoku)
      if true && true {
         // Java: Kinsoku dst = (Kinsoku)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasInvalEndChars() || !dst.hasInvalEndChars()) && (!this.hasInvalEndChars() || dst.hasInvalEndChars()))
         else if (self.hasInvalEndChars() || !dst.hasInvalEndChars()) && (!self.hasInvalEndChars() || dst.hasInvalEndChars()) {
            // Java: if (this.hasInvalEndChars() && dst.hasInvalEndChars() && !this.invalEndChars().equals(dst.invalEndChars()))
            if self.hasInvalEndChars() && dst.hasInvalEndChars() && self.invalEndChars() != dst.invalEndChars() {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasInvalStChars() || !dst.hasInvalStChars()) && (!this.hasInvalStChars() || dst.hasInvalStChars()))
            else if (self.hasInvalStChars() || !dst.hasInvalStChars()) && (!self.hasInvalStChars() || dst.hasInvalStChars()) {
               // Java: if (this.hasInvalStChars() && dst.hasInvalStChars() && !this.invalStChars().equals(dst.invalStChars()))
               if self.hasInvalStChars() && dst.hasInvalStChars() && self.invalStChars() != dst.invalStChars() {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasLang() || !dst.hasLang()) && (!this.hasLang() || dst.hasLang()))
               else if (self.hasLang() || !dst.hasLang()) && (!self.hasLang() || dst.hasLang()) {
                  // Java: return !this.hasLang() || !dst.hasLang() || this.lang().equals(dst.lang());
                  return !self.hasLang() || !dst.hasLang() || self.lang() == dst.lang();
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
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasInvalEndChars())
      if self.hasInvalEndChars() {
         // Java: ret += String.valueOf(this.invalEndChars()).hashCode();
         let s = self.invalEndChars(); ret += crate::cn::wps::kso::kproto::util::string_hash(&s);
      }
      // Java: if (this.hasInvalStChars())
      if self.hasInvalStChars() {
         // Java: ret += String.valueOf(this.invalStChars()).hashCode();
         let s = self.invalStChars(); ret += crate::cn::wps::kso::kproto::util::string_hash(&s);
      }
      // Java: if (this.hasLang())
      if self.hasLang() {
         // Java: ret += String.valueOf(this.lang()).hashCode();
         let s = self.lang(); ret += crate::cn::wps::kso::kproto::util::string_hash(&s);
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
      let zone = self.zone; self.saveBin(zone);
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
    fn cleanFollow(&mut self) {
    }
    // Java: void cleanCache()
    fn cleanCache(&mut self) {
    }
    // Java: void follow(Kinsoku parent)
    pub fn follow(&mut self, parent: Option<&mut Kinsoku>) {
      // Java: if (parent != null)
      if let Some(p) = parent {
         // Java: parent.discard();
         p.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = None;
      }
    }
    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
      if self.parent.is_some() {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { p.discard(); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }
    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }
    // Java: Kinsoku parent()
    pub fn get_parent(&self) -> Option<&Kinsoku> {
      // Java: return this.parent;
      self.parent.as_deref()
    }
    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      // LLM Rust Renamed: Rust cannot overload methods, using self.discard_with_zone
      let zone = self.zone; self.discard_with_zone(zone);
    }
    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }
    // Java: String invalEndChars()
    pub fn invalEndChars(&self) -> String {
      // Java: if (!this.map.has(1))
      if !self.map.has(1) {
         // Java: return this.parent == null ? "" : this.parent.invalEndChars();
         if let Some(ref p) = self.parent { p.invalEndChars() } else { String::new() }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toText(bin);
         Decode::toText(bin.as_ref())
      }
    }
    // Java: boolean hasInvalEndChars()
    pub fn hasInvalEndChars(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasInvalEndChars();
      self.map.has(1) || self.parent.is_some() && self.parent.as_ref().unwrap().hasInvalEndChars()
    }
    // Java: void deleteInvalEndChars()
    pub fn deleteInvalEndChars(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.put_null(1);
    }
    // Java: void putInvalEndChars(String value)
    pub fn putInvalEndChars(&mut self, value: &str) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_text(value, self.zone); self.map.put(1, bin);
    }
    // Java: String invalStChars()
    pub fn invalStChars(&self) -> String {
      // Java: if (!this.map.has(2))
      if !self.map.has(2) {
         // Java: return this.parent == null ? "" : this.parent.invalStChars();
         if let Some(ref p) = self.parent { p.invalStChars() } else { String::new() }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toText(bin);
         Decode::toText(bin.as_ref())
      }
    }
    // Java: boolean hasInvalStChars()
    pub fn hasInvalStChars(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasInvalStChars();
      self.map.has(2) || self.parent.is_some() && self.parent.as_ref().unwrap().hasInvalStChars()
    }
    // Java: void deleteInvalStChars()
    pub fn deleteInvalStChars(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.put_null(2);
    }
    // Java: void putInvalStChars(String value)
    pub fn putInvalStChars(&mut self, value: &str) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_text(value, self.zone); self.map.put(2, bin);
    }
    // Java: String lang()
    pub fn lang(&self) -> String {
      // Java: if (!this.map.has(3))
      if !self.map.has(3) {
         // Java: return this.parent == null ? "" : this.parent.lang();
         if let Some(ref p) = self.parent { p.lang() } else { String::new() }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toText(bin);
         Decode::toText(bin.as_ref())
      }
    }
    // Java: boolean hasLang()
    pub fn hasLang(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasLang();
      self.map.has(3) || self.parent.is_some() && self.parent.as_ref().unwrap().hasLang()
    }
    // Java: void deleteLang()
    pub fn deleteLang(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.put_null(3);
    }
    // Java: void putLang(String value)
    pub fn putLang(&mut self, value: &str) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_text(value, self.zone); self.map.put(3, bin);
    }
}

// Java: public class ModificationVerifier
impl ModificationVerifier {
    // Java: static protected static <Value1 extends ModificationVerifier> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = ModificationVerifier::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }
    // Java: static ModificationVerifier create()
    pub fn create() -> Self {
      // Java: return initialize(ModificationVerifier.class, KoBin.Null.empty());
      let empty = Null::empty(); Self::initialize(empty.as_ref())
    }
    // Java: static ModificationVerifier open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ModificationVerifier.class, bin);
      Self::initialize(bin)
    }
    // Java: ModificationVerifier()
    pub fn new(zone: &'static KoZone) -> Self {
      Self { zone,
        // Java: this.box = new KpBinBox();
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        // LLM: map init in struct literal
        map: KpMap::new(), parent: None,
      }
    }
    // Java: ModificationVerifier(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self { zone,
        // Java: this.box = new KpBinBox(bin);
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        // LLM: map init in struct literal
        map: KpMap::KpMap_from(bin), parent: None,
      }
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
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone; self.loadBin(bin, zone);
    }
    // Java: void load(ModificationVerifier value)
    pub fn load_value(&mut self, value: Option<&ModificationVerifier>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
      }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }
    // Java: boolean equalsLax(ModificationVerifier o)
    pub fn equalsLax(&mut self, o: &mut ModificationVerifier) -> bool {
      // Java: return this.equals(o, false);
      self.equals(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals_obj(&mut self, o: &mut ModificationVerifier) -> bool {
      // Java: return this.equals(o, true);
      self.equals(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals(&mut self, o: &mut ModificationVerifier, _isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof ModificationVerifier)
      if true && true {
         // Java: ModificationVerifier dst = (ModificationVerifier)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasAlgorithmName() || !dst.hasAlgorithmName()) && (!this.hasAlgorithmName() || dst.hasAlgorithmName()))
         else if (self.hasAlgorithmName() || !dst.hasAlgorithmName()) && (!self.hasAlgorithmName() || dst.hasAlgorithmName()) {
            // Java: if (this.hasAlgorithmName() && dst.hasAlgorithmName() && !this.algorithmName().equals(dst.algorithmName()))
            if self.hasAlgorithmName() && dst.hasAlgorithmName() && self.algorithmName() != dst.algorithmName() {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasHashValue() || !dst.hasHashValue()) && (!this.hasHashValue() || dst.hasHashValue()))
            else if (self.hasHashValue() || !dst.hasHashValue()) && (!self.hasHashValue() || dst.hasHashValue()) {
               // Java: if (this.hasHashValue() && dst.hasHashValue() && !this.hashValue().equals(dst.hashValue()))
               if self.hasHashValue() && dst.hasHashValue() && self.hashValue() != dst.hashValue() {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasSaltValue() || !dst.hasSaltValue()) && (!this.hasSaltValue() || dst.hasSaltValue()))
               else if (self.hasSaltValue() || !dst.hasSaltValue()) && (!self.hasSaltValue() || dst.hasSaltValue()) {
                  // Java: if (this.hasSaltValue() && dst.hasSaltValue() && !this.saltValue().equals(dst.saltValue()))
                  if self.hasSaltValue() && dst.hasSaltValue() && self.saltValue() != dst.saltValue() {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasSpinValue() || !dst.hasSpinValue()) && (!this.hasSpinValue() || dst.hasSpinValue()))
                  else if (self.hasSpinValue() || !dst.hasSpinValue()) && (!self.hasSpinValue() || dst.hasSpinValue()) {
                     // Java: return !this.hasSpinValue() || !dst.hasSpinValue() || this.spinValue().equals(dst.spinValue());
                     return !self.hasSpinValue() || !dst.hasSpinValue() || self.spinValue() == dst.spinValue();
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
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasAlgorithmName())
      if self.hasAlgorithmName() {
         // Java: ret += String.valueOf(this.algorithmName()).hashCode();
         let s = self.algorithmName(); ret += crate::cn::wps::kso::kproto::util::string_hash(&s);
      }
      // Java: if (this.hasHashValue())
      if self.hasHashValue() {
         // Java: ret += String.valueOf(this.hashValue()).hashCode();
         let s = self.hashValue(); ret += crate::cn::wps::kso::kproto::util::string_hash(&s);
      }
      // Java: if (this.hasSaltValue())
      if self.hasSaltValue() {
         // Java: ret += String.valueOf(this.saltValue()).hashCode();
         let s = self.saltValue(); ret += crate::cn::wps::kso::kproto::util::string_hash(&s);
      }
      // Java: if (this.hasSpinValue())
      if self.hasSpinValue() {
         // Java: ret += String.valueOf(this.spinValue()).hashCode();
         let s = self.spinValue(); ret += crate::cn::wps::kso::kproto::util::string_hash(&s);
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
      let zone = self.zone; self.saveBin(zone);
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
    fn cleanFollow(&mut self) {
    }
    // Java: void cleanCache()
    fn cleanCache(&mut self) {
    }
    // Java: void follow(ModificationVerifier parent)
    pub fn follow(&mut self, parent: Option<&mut ModificationVerifier>) {
      // Java: if (parent != null)
      if let Some(p) = parent {
         // Java: parent.discard();
         p.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = None;
      }
    }
    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
      if self.parent.is_some() {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { p.discard(); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }
    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }
    // Java: ModificationVerifier parent()
    pub fn get_parent(&self) -> Option<&ModificationVerifier> {
      // Java: return this.parent;
      self.parent.as_deref()
    }
    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      // LLM Rust Renamed: Rust cannot overload methods, using self.discard_with_zone
      let zone = self.zone; self.discard_with_zone(zone);
    }
    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }
    // Java: String algorithmName()
    pub fn algorithmName(&self) -> String {
      // Java: if (!this.map.has(1))
      if !self.map.has(1) {
         // Java: return this.parent == null ? "" : this.parent.algorithmName();
         if let Some(ref p) = self.parent { p.algorithmName() } else { String::new() }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toText(bin);
         Decode::toText(bin.as_ref())
      }
    }
    // Java: boolean hasAlgorithmName()
    pub fn hasAlgorithmName(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasAlgorithmName();
      self.map.has(1) || self.parent.is_some() && self.parent.as_ref().unwrap().hasAlgorithmName()
    }
    // Java: void deleteAlgorithmName()
    pub fn deleteAlgorithmName(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.put_null(1);
    }
    // Java: void putAlgorithmName(String value)
    pub fn putAlgorithmName(&mut self, value: &str) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_text(value, self.zone); self.map.put(1, bin);
    }
    // Java: String hashValue()
    pub fn hashValue(&self) -> String {
      // Java: if (!this.map.has(2))
      if !self.map.has(2) {
         // Java: return this.parent == null ? "" : this.parent.hashValue();
         if let Some(ref p) = self.parent { p.hashValue() } else { String::new() }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toText(bin);
         Decode::toText(bin.as_ref())
      }
    }
    // Java: boolean hasHashValue()
    pub fn hasHashValue(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasHashValue();
      self.map.has(2) || self.parent.is_some() && self.parent.as_ref().unwrap().hasHashValue()
    }
    // Java: void deleteHashValue()
    pub fn deleteHashValue(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.put_null(2);
    }
    // Java: void putHashValue(String value)
    pub fn putHashValue(&mut self, value: &str) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_text(value, self.zone); self.map.put(2, bin);
    }
    // Java: String saltValue()
    pub fn saltValue(&self) -> String {
      // Java: if (!this.map.has(3))
      if !self.map.has(3) {
         // Java: return this.parent == null ? "" : this.parent.saltValue();
         if let Some(ref p) = self.parent { p.saltValue() } else { String::new() }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toText(bin);
         Decode::toText(bin.as_ref())
      }
    }
    // Java: boolean hasSaltValue()
    pub fn hasSaltValue(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasSaltValue();
      self.map.has(3) || self.parent.is_some() && self.parent.as_ref().unwrap().hasSaltValue()
    }
    // Java: void deleteSaltValue()
    pub fn deleteSaltValue(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.put_null(3);
    }
    // Java: void putSaltValue(String value)
    pub fn putSaltValue(&mut self, value: &str) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_text(value, self.zone); self.map.put(3, bin);
    }
    // Java: String spinValue()
    pub fn spinValue(&self) -> String {
      // Java: if (!this.map.has(4))
      if !self.map.has(4) {
         // Java: return this.parent == null ? "" : this.parent.spinValue();
         if let Some(ref p) = self.parent { p.spinValue() } else { String::new() }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(4);
         // Java: return KoBin.Decode.toText(bin);
         Decode::toText(bin.as_ref())
      }
    }
    // Java: boolean hasSpinValue()
    pub fn hasSpinValue(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasSpinValue();
      self.map.has(4) || self.parent.is_some() && self.parent.as_ref().unwrap().hasSpinValue()
    }
    // Java: void deleteSpinValue()
    pub fn deleteSpinValue(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.put_null(4);
    }
    // Java: void putSpinValue(String value)
    pub fn putSpinValue(&mut self, value: &str) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_text(value, self.zone); self.map.put(4, bin);
    }
}

// Java: public class SlideSize
impl SlideSize {
    // Java: static protected static <Value1 extends SlideSize> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = SlideSize::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }
    // Java: static SlideSize create()
    pub fn create() -> Self {
      // Java: return initialize(SlideSize.class, KoBin.Null.empty());
      let empty = Null::empty(); Self::initialize(empty.as_ref())
    }
    // Java: static SlideSize open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(SlideSize.class, bin);
      Self::initialize(bin)
    }
    // Java: SlideSize()
    pub fn new(zone: &'static KoZone) -> Self {
      Self { zone,
        // Java: this.box = new KpBinBox();
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        // LLM Rust Pattern: struct literal initialization
        map: KpMap::new(), parent: None,
      }
    }
    // Java: SlideSize(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self { zone,
        // Java: this.box = new KpBinBox(bin);
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        // LLM Rust Pattern: struct literal initialization
        map: KpMap::KpMap_from(bin), parent: None,
      }
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
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone; self.loadBin(bin, zone);
    }
    // Java: void load(SlideSize value)
    pub fn load_value(&mut self, value: Option<&SlideSize>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
      }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }
    // Java: boolean equalsLax(SlideSize o)
    pub fn equalsLax(&mut self, o: &mut SlideSize) -> bool {
      // Java: return this.equals(o, false);
      self.equals(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals_obj(&mut self, o: &mut SlideSize) -> bool {
      // Java: return this.equals(o, true);
      self.equals(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals(&mut self, o: &mut SlideSize, _isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof SlideSize)
      if true && true {
         // Java: SlideSize dst = (SlideSize)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasCx() || !dst.hasCx()) && (!this.hasCx() || dst.hasCx()))
         else if (self.hasCx() || !dst.hasCx()) && (!self.hasCx() || dst.hasCx()) {
            // Java: if (this.hasCx() && dst.hasCx() && this.cx() != dst.cx())
            if self.hasCx() && dst.hasCx() && self.cx() != dst.cx() {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasCy() || !dst.hasCy()) && (!this.hasCy() || dst.hasCy()))
            else if (self.hasCy() || !dst.hasCy()) && (!self.hasCy() || dst.hasCy()) {
               // Java: if (this.hasCy() && dst.hasCy() && this.cy() != dst.cy())
               if self.hasCy() && dst.hasCy() && self.cy() != dst.cy() {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
               else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType()) {
                  // Java: return !this.hasType() || !dst.hasType() || this.type() == dst.type();
                  return !self.hasType() || !dst.hasType() || self.r#type() == dst.r#type();
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
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasCx())
      if self.hasCx() {
         // Java: ret += Integer.valueOf(this.cx()).hashCode();
         ret += self.cx();
      }
      // Java: if (this.hasCy())
      if self.hasCy() {
         // Java: ret += Integer.valueOf(this.cy()).hashCode();
         ret += self.cy();
      }
      // Java: if (this.hasType())
      if self.hasType() {
         // Java: ret += Integer.valueOf(this.type()).hashCode();
         ret += self.r#type();
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
      let zone = self.zone; self.saveBin(zone);
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
    fn cleanFollow(&mut self) {
    }
    // Java: void cleanCache()
    fn cleanCache(&mut self) {
    }
    // Java: void follow(SlideSize parent)
    pub fn follow(&mut self, parent: Option<&mut SlideSize>) {
      // Java: if (parent != null)
      if let Some(p) = parent {
         // Java: parent.discard();
         p.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = None;
      }
    }
    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
      if self.parent.is_some() {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { p.discard(); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }
    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }
    // Java: SlideSize parent()
    pub fn get_parent(&self) -> Option<&SlideSize> {
      // Java: return this.parent;
      self.parent.as_deref()
    }
    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      // LLM Rust Renamed: Rust cannot overload methods, using self.discard_with_zone
      let zone = self.zone; self.discard_with_zone(zone);
    }
    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }
    // Java: int cx()
    pub fn cx(&self) -> i32 {
      // Java: if (!this.map.has(1))
      if !self.map.has(1) {
         // Java: return this.parent == null ? 0 : this.parent.cx();
         if let Some(ref p) = self.parent { p.cx() } else { 0 }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }
    // Java: boolean hasCx()
    pub fn hasCx(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasCx();
      self.map.has(1) || self.parent.is_some() && self.parent.as_ref().unwrap().hasCx()
    }
    // Java: void deleteCx()
    pub fn deleteCx(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.put_null(1);
    }
    // Java: void putCx(int value)
    pub fn putCx(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone); self.map.put(1, bin);
    }
    // Java: int cy()
    pub fn cy(&self) -> i32 {
      // Java: if (!this.map.has(2))
      if !self.map.has(2) {
         // Java: return this.parent == null ? 0 : this.parent.cy();
         if let Some(ref p) = self.parent { p.cy() } else { 0 }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }
    // Java: boolean hasCy()
    pub fn hasCy(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasCy();
      self.map.has(2) || self.parent.is_some() && self.parent.as_ref().unwrap().hasCy()
    }
    // Java: void deleteCy()
    pub fn deleteCy(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.put_null(2);
    }
    // Java: void putCy(int value)
    pub fn putCy(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone); self.map.put(2, bin);
    }
    // Java: int type()
    pub fn r#type(&self) -> i32 {
      // Java: if (!this.map.has(3))
      if !self.map.has(3) {
         // Java: return this.parent == null ? 6 : this.parent.type();
         if let Some(ref p) = self.parent { p.r#type() } else { 6 }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }
    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasType();
      self.map.has(3) || self.parent.is_some() && self.parent.as_ref().unwrap().hasType()
    }
    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.put_null(3);
    }
    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone); self.map.put(3, bin);
    }
}

// Java: public class PhotoAlbum
impl PhotoAlbum {
    // Java: static protected static <Value1 extends PhotoAlbum> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = PhotoAlbum::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }
    // Java: static PhotoAlbum create()
    pub fn create() -> Self {
      // Java: return initialize(PhotoAlbum.class, KoBin.Null.empty());
      let empty = Null::empty(); Self::initialize(empty.as_ref())
    }
    // Java: static PhotoAlbum open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(PhotoAlbum.class, bin);
      Self::initialize(bin)
    }
    // Java: PhotoAlbum()
    pub fn new(zone: &'static KoZone) -> Self {
      Self { zone,
        // Java: this.box = new KpBinBox();
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::new(),
        // Java: this.map = new KpMap();
        // LLM Rust Pattern: struct literal initialization
        map: KpMap::new(), parent: None,
      }
    }
    // Java: PhotoAlbum(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self { zone,
        // Java: this.box = new KpBinBox(bin);
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.map = new KpMap(bin);
        // LLM Rust Pattern: struct literal initialization
        map: KpMap::KpMap_from(bin), parent: None,
      }
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
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone; self.loadBin(bin, zone);
    }
    // Java: void load(PhotoAlbum value)
    pub fn load_value(&mut self, value: Option<&PhotoAlbum>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
      }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }
    // Java: boolean equalsLax(PhotoAlbum o)
    pub fn equalsLax(&mut self, o: &mut PhotoAlbum) -> bool {
      // Java: return this.equals(o, false);
      self.equals(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals_obj(&mut self, o: &mut PhotoAlbum) -> bool {
      // Java: return this.equals(o, true);
      self.equals(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals(&mut self, o: &mut PhotoAlbum, _isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof PhotoAlbum)
      if true && true {
         // Java: PhotoAlbum dst = (PhotoAlbum)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasBlackAndWhite() || !dst.hasBlackAndWhite()) && (!this.hasBlackAndWhite() || dst.hasBlackAndWhite()))
         else if (self.hasBlackAndWhite() || !dst.hasBlackAndWhite()) && (!self.hasBlackAndWhite() || dst.hasBlackAndWhite()) {
            // Java: if (this.hasBlackAndWhite() && dst.hasBlackAndWhite() && this.isBlackAndWhite() != dst.isBlackAndWhite())
            if self.hasBlackAndWhite() && dst.hasBlackAndWhite() && self.isBlackAndWhite() != dst.isBlackAndWhite() {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasShowCaptions() || !dst.hasShowCaptions()) && (!this.hasShowCaptions() || dst.hasShowCaptions()))
            else if (self.hasShowCaptions() || !dst.hasShowCaptions()) && (!self.hasShowCaptions() || dst.hasShowCaptions()) {
               // Java: if (this.hasShowCaptions() && dst.hasShowCaptions() && this.isShowCaptions() != dst.isShowCaptions())
               if self.hasShowCaptions() && dst.hasShowCaptions() && self.isShowCaptions() != dst.isShowCaptions() {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasLayout() || !dst.hasLayout()) && (!this.hasLayout() || dst.hasLayout()))
               else if (self.hasLayout() || !dst.hasLayout()) && (!self.hasLayout() || dst.hasLayout()) {
                  // Java: if (this.hasLayout() && dst.hasLayout() && this.layout() != dst.layout())
                  if self.hasLayout() && dst.hasLayout() && self.layout() != dst.layout() {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasFrame() || !dst.hasFrame()) && (!this.hasFrame() || dst.hasFrame()))
                  else if (self.hasFrame() || !dst.hasFrame()) && (!self.hasFrame() || dst.hasFrame()) {
                     // Java: return !this.hasFrame() || !dst.hasFrame() || this.frame() == dst.frame();
                     return !self.hasFrame() || !dst.hasFrame() || self.frame() == dst.frame();
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
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasBlackAndWhite())
      if self.hasBlackAndWhite() {
         // Java: ret += Boolean.valueOf(this.isBlackAndWhite()).hashCode();
         ret += if self.isBlackAndWhite() { 1231 } else { 1237 };
      }
      // Java: if (this.hasShowCaptions())
      if self.hasShowCaptions() {
         // Java: ret += Boolean.valueOf(this.isShowCaptions()).hashCode();
         ret += if self.isShowCaptions() { 1231 } else { 1237 };
      }
      // Java: if (this.hasLayout())
      if self.hasLayout() {
         // Java: ret += Integer.valueOf(this.layout()).hashCode();
         ret += self.layout();
      }
      // Java: if (this.hasFrame())
      if self.hasFrame() {
         // Java: ret += Integer.valueOf(this.frame()).hashCode();
         ret += self.frame();
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
      let zone = self.zone; self.saveBin(zone);
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
    fn cleanFollow(&mut self) {
    }
    // Java: void cleanCache()
    fn cleanCache(&mut self) {
    }
    // Java: void follow(PhotoAlbum parent)
    pub fn follow(&mut self, parent: Option<&mut PhotoAlbum>) {
      // Java: if (parent != null)
      if let Some(p) = parent {
         // Java: parent.discard();
         p.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = None;
      }
    }
    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
      if self.parent.is_some() {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { p.discard(); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }
    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }
    // Java: PhotoAlbum parent()
    pub fn get_parent(&self) -> Option<&PhotoAlbum> {
      // Java: return this.parent;
      self.parent.as_deref()
    }
    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      // LLM Rust Renamed: Rust cannot overload methods, using self.discard_with_zone
      let zone = self.zone; self.discard_with_zone(zone);
    }
    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }
    // Java: boolean isBlackAndWhite()
    pub fn isBlackAndWhite(&self) -> bool {
      // Java: if (!this.map.has(1))
      if !self.map.has(1) {
         // Java: return this.parent == null ? false : this.parent.isBlackAndWhite();
         if let Some(ref p) = self.parent { p.isBlackAndWhite() } else { false }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toBool(bin);
         Decode::toBool(bin.as_ref())
      }
    }
    // Java: boolean hasBlackAndWhite()
    pub fn hasBlackAndWhite(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasBlackAndWhite();
      self.map.has(1) || self.parent.is_some() && self.parent.as_ref().unwrap().hasBlackAndWhite()
    }
    // Java: void deleteBlackAndWhite()
    pub fn deleteBlackAndWhite(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.put_null(1);
    }
    // Java: void putBlackAndWhite(boolean value)
    pub fn putBlackAndWhite(&mut self, value: bool) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_bool(value, self.zone); self.map.put(1, bin);
    }
    // Java: boolean isShowCaptions()
    pub fn isShowCaptions(&self) -> bool {
      // Java: if (!this.map.has(2))
      if !self.map.has(2) {
         // Java: return this.parent == null ? false : this.parent.isShowCaptions();
         if let Some(ref p) = self.parent { p.isShowCaptions() } else { false }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toBool(bin);
         Decode::toBool(bin.as_ref())
      }
    }
    // Java: boolean hasShowCaptions()
    pub fn hasShowCaptions(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasShowCaptions();
      self.map.has(2) || self.parent.is_some() && self.parent.as_ref().unwrap().hasShowCaptions()
    }
    // Java: void deleteShowCaptions()
    pub fn deleteShowCaptions(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.put_null(2);
    }
    // Java: void putShowCaptions(boolean value)
    pub fn putShowCaptions(&mut self, value: bool) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_bool(value, self.zone); self.map.put(2, bin);
    }
    // Java: int layout()
    pub fn layout(&self) -> i32 {
      // Java: if (!this.map.has(3))
      if !self.map.has(3) {
         // Java: return this.parent == null ? 0 : this.parent.layout();
         if let Some(ref p) = self.parent { p.layout() } else { 0 }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }
    // Java: boolean hasLayout()
    pub fn hasLayout(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasLayout();
      self.map.has(3) || self.parent.is_some() && self.parent.as_ref().unwrap().hasLayout()
    }
    // Java: void deleteLayout()
    pub fn deleteLayout(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.put_null(3);
    }
    // Java: void putLayout(int value)
    pub fn putLayout(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone); self.map.put(3, bin);
    }
    // Java: int frame()
    pub fn frame(&self) -> i32 {
      // Java: if (!this.map.has(4))
      if !self.map.has(4) {
         // Java: return this.parent == null ? 0 : this.parent.frame();
         if let Some(ref p) = self.parent { p.frame() } else { 0 }
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(4);
         // Java: return KoBin.Decode.toInt(bin);
         Decode::toInt(bin.as_ref())
      }
    }
    // Java: boolean hasFrame()
    pub fn hasFrame(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasFrame();
      self.map.has(4) || self.parent.is_some() && self.parent.as_ref().unwrap().hasFrame()
    }
    // Java: void deleteFrame()
    pub fn deleteFrame(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.put_null(4);
    }
    // Java: void putFrame(int value)
    pub fn putFrame(&mut self, value: i32) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone); self.map.put(4, bin);
    }
}

// Java: public class CommentAuthorList
impl CommentAuthorList {
    // Java: CommentAuthorList()
    pub fn new(zone: &'static KoZone) -> Self {
      Self {
        // Java: this.box = new KpBinBox();
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::new(),
        // Java: this.list = new KpList(this.zone);
        // LLM Rust Pattern: struct literal initialization
        list: KpList::new(zone),
        zone,
      }
    }
    // Java: CommentAuthorList(KoBin bin)
    pub fn new_from_bin(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
      Self {
        // Java: this.box = new KpBinBox(bin);
        // LLM Rust Renamed: box is reserved keyword, using bin_box
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        // Java: this.list = new KpList(bin, this.zone);
        // LLM Rust Pattern: struct literal initialization
        list: KpList::KpList_from(bin, zone),
        zone,
      }
    }
    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.list.load(this.box.get(), zone);
      let b = self.bin_box.get(); self.list.load(b.as_ref(), zone);
    }
    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone; self.loadBin(bin, zone);
    }
    // Java: void load(CommentAuthorList value)
    pub fn load_value(&mut self, value: Option<&CommentAuthorList>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone); self.loadBin(bin.as_ref(), self.zone);
      }
    }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
      if self.list.dirty() { let saved = self.list.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() } else { self.bin_box.get() }
    }
    // Java: boolean equalsLax(CommentAuthorList o)
    pub fn equalsLax(&mut self, o: &mut CommentAuthorList) -> bool {
      // Java: return this.equals(o, false);
      self.equals(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals_obj(&mut self, o: &mut CommentAuthorList) -> bool {
      // Java: return this.equals(o, true);
      self.equals(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals(&mut self, o: &mut CommentAuthorList, isStrict: bool) -> bool {
      // Java: if (null == o)
      if false {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: CommentAuthorList dst = (CommentAuthorList)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _) {
            // Java: return true;
            return true;
         }
         // Java: else
         else {
            // Java: ArrayList<CommentAuthor> dstList = new ArrayList();
            let mut dstList: Vec<CommentAuthor> = Vec::new();
            // Java: dst.read(dstList);
            dst.read(&mut dstList);
            // Java: ArrayList<CommentAuthor> srcList = new ArrayList();
            let mut srcList: Vec<CommentAuthor> = Vec::new();
            // Java: this.read(srcList);
            self.read(&mut srcList);
            // Java: if (dstList.size() != srcList.size())
            if dstList.len() != srcList.len() {
               // Java: return false;
               return false;
            }
            // Java: else
            else {
               // Java: int size = dstList.size();
               let size = dstList.len();
               // Java: for(int i = 0; i < size; ++i)
               for i in 0..size {
                  // Java: CommentAuthor dstClrTrans = (CommentAuthor)dstList.get(i);
                  let dstClrTrans = &mut dstList[i];
                  // Java: CommentAuthor srcClrTrans = (CommentAuthor)srcList.get(i);
                  let srcClrTrans = &mut srcList[i];
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
    }
}
// Java: public class CommentAuthorList
impl CommentAuthorList {
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return this.list.hashCode();
      self.list.hashCode()
    }
    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.list.dirty();
      self.list.dirty()
    }
    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone; self.saveBin(zone);
      // Java: return this.box.get();
      self.bin_box.get()
    }
    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
      // Java: this.loadBin(this.box.get(), zone);
      let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone);
    }
    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      Factory::dup(self.bin_box.get().as_ref(), zone)
    }
    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.list.load(KoBin.Null.value1(), this.zone);
      let n = Null::value1(); self.list.load(n.as_ref(), self.zone);
      // Java: this.zone.leave();
      self.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }
    // Java: static protected static <Value1 extends CommentAuthorList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = CommentAuthorList::new(zone);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }
    // Java: static CommentAuthorList create()
    pub fn create() -> Self {
      // Java: return initialize(CommentAuthorList.class, KoBin.Null.empty());
      let empty = Null::empty(); Self::initialize(empty.as_ref())
    }
    // Java: static CommentAuthorList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(CommentAuthorList.class, bin);
      Self::initialize(bin)
    }
    // Java: void read(Collection<CommentAuthor> valueLst)
    pub fn read(&self, valueLst: &mut Vec<CommentAuthor>) {
      // Java: valueLst.clear();
      valueLst.clear();
      // Java: int size = this.list.size();
      let size = self.list.size();
      // Java: for(int i = 0; i < size; ++i)
      for i in 0..size {
         // Java: valueLst.add((CommentAuthor)KoBin.Decode.toAny(CommentAuthor.class, this.list.get(i), this.zone));
         let bin = self.list.get(i); valueLst.push(CommentAuthor::new_from_bin(bin.as_ref(), self.zone));
      }
    }
    // Java: void write(Iterable<CommentAuthor> valueLst)
    pub fn write(&mut self, valueLst: &[CommentAuthor]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(CommentAuthor i : valueLst)
      for i in valueLst.iter() {
         // Java: KoBin bin = i.lastBin(this.zone);
         let bin = i.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref())) {
            // Java: this.list.add(bin);
            self.list.add(bin);
         }
      }
    }
    // Java: void forceWrite(Iterable<CommentAuthor> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[CommentAuthor]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(CommentAuthor i : valueLst)
      for i in valueLst.iter() {
         // Java: this.list.add(i.lastBin(this.zone));
         self.list.add(i.lastBin(self.zone));
      }
    }
}

impl KoAr for Presentation {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { Presentation::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { Presentation::dirty(self) }
    // LLM Rust Renamed: Rust cannot overload methods, using self.discard_with_zone
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Presentation::lastBin(self, var1) }
}

impl KoClosable for Presentation {
    fn close(&mut self) { Presentation::close(self); }
}
