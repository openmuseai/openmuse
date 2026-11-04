#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.media.Media
// use crate::cn::wps::drawing::data::media::Media; // TODO: uncomment when Media is properly converted
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
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;

// Java: public class Hyperlink implements KoAr, KoClosable
pub struct Hyperlink {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Media.EmbeddedWAVAudioFile sound
    pub(crate) sound: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Hyperlink parent
    pub(crate) parent: Option<Box<Hyperlink>>,
}

// Java: public class Hyperlink implements KoAr, KoClosable
impl Hyperlink {
    // Java: static protected static <Value1 extends Hyperlink> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::Hyperlink_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static Hyperlink create()
    pub fn create() -> Self {
      // Java: return initialize(Hyperlink.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Hyperlink open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Hyperlink.class, bin);
      Self::initialize(bin)
    }

    // Java: Hyperlink()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), parent: None, sound: None,
      }
    }

    // Java: Hyperlink(KoBin bin)
    pub fn Hyperlink_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), parent: None, sound: None,
      }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.map.load(this.box.get());
      let bin_ref = self.bin_box.get();
      self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(Hyperlink value)
    pub fn load_value(&mut self, value: Option<&Hyperlink>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin_with_zone(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.sound != null && this.sound.dirty())
        if self.sound.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(1, this.sound.saveBin(zone));
         todo!("saveBin for sound field");
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() {
         let saved = self.map.save(zone);
         self.bin_box.set(Some(saved.as_ref()));
         self.bin_box.get()
      } else {
         self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(Hyperlink o)
    pub fn equalsLax(&self, o: &Hyperlink) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Hyperlink) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &Hyperlink, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Hyperlink)
        if true
      {
         // Java: Hyperlink dst = (Hyperlink)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasSound() || !dst.hasSound()) && (!this.hasSound() || dst.hasSound()))
            else if (self.hasSound() || !dst.hasSound()) && (!self.hasSound() || dst.hasSound())
         {
            // Java: if (this.hasSound() && dst.hasSound())
                if self.hasSound() && dst.hasSound()
            {
               // Java: if (isStrict && !this.sound().equals(dst.sound()))
                    if isStrict { todo!("sound().equals"); }
               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.sound().equalsLax(dst.sound()))
                    if !isStrict { todo!("sound().equalsLax"); }
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasInvalidUrl() || !dst.hasInvalidUrl()) && (!this.hasInvalidUrl() || dst.hasInvalidUrl()))
                if (self.hasInvalidUrl() || !dst.hasInvalidUrl()) && (!self.hasInvalidUrl() || dst.hasInvalidUrl())
            {
               // Java: if (this.hasInvalidUrl() && dst.hasInvalidUrl() && !this.invalidUrl().equals(dst.invalidUrl()))
                    if self.hasInvalidUrl() && dst.hasInvalidUrl() && self.get_invalidUrl() != dst.get_invalidUrl()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasAction() || !dst.hasAction()) && (!this.hasAction() || dst.hasAction()))
                    else if (self.hasAction() || !dst.hasAction()) && (!self.hasAction() || dst.hasAction())
               {
                  // Java: if (this.hasAction() && dst.hasAction() && !this.action().equals(dst.action()))
                        if self.hasAction() && dst.hasAction() && self.get_action() != dst.get_action()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasTgtFrame() || !dst.hasTgtFrame()) && (!this.hasTgtFrame() || dst.hasTgtFrame()))
                        else if (self.hasTgtFrame() || !dst.hasTgtFrame()) && (!self.hasTgtFrame() || dst.hasTgtFrame())
                  {
                     // Java: if (this.hasTgtFrame() && dst.hasTgtFrame() && !this.tgtFrame().equals(dst.tgtFrame()))
                            if self.hasTgtFrame() && dst.hasTgtFrame() && self.get_tgtFrame() != dst.get_tgtFrame()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasTooltip() || !dst.hasTooltip()) && (!this.hasTooltip() || dst.hasTooltip()))
                            else if (self.hasTooltip() || !dst.hasTooltip()) && (!self.hasTooltip() || dst.hasTooltip())
                     {
                        // Java: if (this.hasTooltip() && dst.hasTooltip() && !this.tooltip().equals(dst.tooltip()))
                                if self.hasTooltip() && dst.hasTooltip() && self.get_tooltip() != dst.get_tooltip()
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasHistory() || !dst.hasHistory()) && (!this.hasHistory() || dst.hasHistory()))
                                else if (self.hasHistory() || !dst.hasHistory()) && (!self.hasHistory() || dst.hasHistory())
                        {
                           // Java: if (this.hasHistory() && dst.hasHistory() && this.isHistory() != dst.isHistory())
                                    if self.hasHistory() && dst.hasHistory() && self.isHistory() != dst.isHistory()
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasHighlightClick() || !dst.hasHighlightClick()) && (!this.hasHighlightClick() || dst.hasHighlightClick()))
                                    else if (self.hasHighlightClick() || !dst.hasHighlightClick()) && (!self.hasHighlightClick() || dst.hasHighlightClick())
                           {
                              // Java: if (this.hasHighlightClick() && dst.hasHighlightClick() && this.isHighlightClick() != dst.isHighlightClick())
                                        if self.hasHighlightClick() && dst.hasHighlightClick() && self.isHighlightClick() != dst.isHighlightClick()
                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: else if ((this.hasEndSound() || !dst.hasEndSound()) && (!this.hasEndSound() || dst.hasEndSound()))
                                        else if (self.hasEndSound() || !dst.hasEndSound()) && (!self.hasEndSound() || dst.hasEndSound())
                              {
                                 // Java: if (this.hasEndSound() && dst.hasEndSound() && this.isEndSound() != dst.isEndSound())
                                            if self.hasEndSound() && dst.hasEndSound() && self.isEndSound() != dst.isEndSound()
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: else if ((this.hasTarget() || !dst.hasTarget()) && (!this.hasTarget() || dst.hasTarget()))
                                            else if (self.hasTarget() || !dst.hasTarget()) && (!self.hasTarget() || dst.hasTarget())
                                 {
                                    // Java: if (this.hasTarget() && dst.hasTarget() && !this.target().equals(dst.target()))
                                                if self.hasTarget() && dst.hasTarget() && self.get_target() != dst.get_target()
                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: else if ((this.hasTargetInternalMode() || !dst.hasTargetInternalMode()) && (!this.hasTargetInternalMode() || dst.hasTargetInternalMode()))
                                                else if (self.hasTargetInternalMode() || !dst.hasTargetInternalMode()) && (!self.hasTargetInternalMode() || dst.hasTargetInternalMode())
                                    {
                                       // Java: if (this.hasTargetInternalMode() && dst.hasTargetInternalMode() && this.isTargetInternalMode() != dst.isTargetInternalMode())
                                                    if self.hasTargetInternalMode() && dst.hasTargetInternalMode() && self.isTargetInternalMode() != dst.isTargetInternalMode()
                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                       // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
                                                    else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType())
                                       {
                                          // Java: if (this.hasType() && dst.hasType() && !this.type().equals(dst.type()))
                                                        if self.hasType() && dst.hasType() && self.get_type() != dst.get_type()
                                          {
                                             // Java: return false;
                                             return false;
                                          }
                                          // Java: else if ((this.hasClicked() || !dst.hasClicked()) && (!this.hasClicked() || dst.hasClicked()))
                                                        else if (self.hasClicked() || !dst.hasClicked()) && (!self.hasClicked() || dst.hasClicked())
                                          {
                                             // Java: return !this.hasClicked() || !dst.hasClicked() || this.isClicked() == dst.isClicked();
                                             return !self.hasClicked() || !dst.hasClicked() || self.isClicked() == dst.isClicked();
                                          }
                                          // Java: else
                                                        else
                                          {
                                             // Java: return false;
                                             return false;
                                          }
                                       }
                                       // Java: else
                                                    else
                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                    }
                                    // Java: else
                                                else
                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                 }
                                 // Java: else
                                            else
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: else
                                        else
                              {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: else
                                    else
                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: else
                                else
                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: else
                            else
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: else
                        else
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: else
                    else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
                else
            {
               // Java: return false;
               return false;
            }
         }
         // Java: else
            else
         {
            // Java: return false;
            return false;
         }
      }
      // Java: else
        else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasSound())
        if self.hasSound()
      {
         // Java: ret += this.sound().hashCode();
         todo!("sound().hashCode");
      }
      // Java: if (this.hasInvalidUrl())
        if self.hasInvalidUrl()
      {
         // Java: ret += String.valueOf(this.invalidUrl()).hashCode();
         todo!("String hashCode");
      }
      // Java: if (this.hasAction())
        if self.hasAction()
      {
         // Java: ret += String.valueOf(this.action()).hashCode();
         todo!("String hashCode");
      }
      // Java: if (this.hasTgtFrame())
        if self.hasTgtFrame()
      {
         // Java: ret += String.valueOf(this.tgtFrame()).hashCode();
         todo!("String hashCode");
      }
      // Java: if (this.hasTooltip())
        if self.hasTooltip()
      {
         // Java: ret += String.valueOf(this.tooltip()).hashCode();
         todo!("String hashCode");
      }
      // Java: if (this.hasHistory())
        if self.hasHistory()
      {
         // Java: ret += Boolean.valueOf(this.isHistory()).hashCode();
         ret = ret.wrapping_add(if self.isHistory() { 1231 } else { 1237 });
      }
      // Java: if (this.hasHighlightClick())
        if self.hasHighlightClick()
      {
         // Java: ret += Boolean.valueOf(this.isHighlightClick()).hashCode();
         ret = ret.wrapping_add(if self.isHighlightClick() { 1231 } else { 1237 });
      }
      // Java: if (this.hasEndSound())
        if self.hasEndSound()
      {
         // Java: ret += Boolean.valueOf(this.isEndSound()).hashCode();
         ret = ret.wrapping_add(if self.isEndSound() { 1231 } else { 1237 });
      }
      // Java: if (this.hasTarget())
        if self.hasTarget()
      {
         // Java: ret += String.valueOf(this.target()).hashCode();
         todo!("String hashCode");
      }
      // Java: if (this.hasTargetInternalMode())
        if self.hasTargetInternalMode()
      {
         // Java: ret += Boolean.valueOf(this.isTargetInternalMode()).hashCode();
         ret = ret.wrapping_add(if self.isTargetInternalMode() { 1231 } else { 1237 });
      }
      // Java: if (this.hasType())
        if self.hasType()
      {
         // Java: ret += String.valueOf(this.type()).hashCode();
         todo!("String hashCode");
      }
      // Java: if (this.hasClicked())
        if self.hasClicked()
      {
         // Java: ret += Boolean.valueOf(this.isClicked()).hashCode();
         ret = ret.wrapping_add(if self.isClicked() { 1231 } else { 1237 });
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.sound != null && this.sound.dirty() ? true : this.map.dirty();
      if self.sound.as_ref().map_or(false, |s| s.dirty()) { true } else { self.map.dirty() }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone;
      self.saveBin(zone);
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
      self.loadBin_with_zone(bin.as_ref(), zone);
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
      let null_bin = Null::value1();
      self.map.load(null_bin.as_ref());
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
      // Java: this.sound = null;
      self.sound = None;
    }

    // Java: void follow(Hyperlink parent)
    pub fn follow(&mut self, parent: Option<Box<Hyperlink>>) {
      // Java: if (parent != null)
        if let Some(mut p) = parent
      {
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
        if self.parent.is_some()
      {
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

    // Java: Hyperlink parent()
    pub fn get_parent(&self) -> Option<&Hyperlink> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone;
      self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: boolean hasSound()
    pub fn hasSound(&self) -> bool {
      // Java: return this.map.has(1) || this.sound != null && this.sound.dirty() || this.parent != null && this.parent.hasSound();
      self.map.has(1) || self.sound.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSound())
    }

    // Java: void deleteSound()
    pub fn deleteSound(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.sound != null)
        if self.sound.is_some()
      {
         // Java: this.sound = null;
         self.sound = None;
      }
    }

    // Java: void writeSound(Media.EmbeddedWAVAudioFile value)
    pub fn writeSound(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(1, bin);
            // Java: if (this.sound != null)
                if self.sound.is_some()
            {
               // Java: this.sound = null;
               self.sound = None;
            }
         }
      }
    }

    // Java: void forceWriteSound(Media.EmbeddedWAVAudioFile value)
    pub fn forceWriteSound(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.sound != null)
            if self.sound.is_some()
         {
            // Java: this.sound = null;
            self.sound = None;
         }
      }
    }

    // Java: String invalidUrl()
    pub fn get_invalidUrl(&self) -> String {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? "" : this.parent.invalidUrl();
         return self.parent.as_ref().map_or(String::new(), |p| p.get_invalidUrl());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasInvalidUrl()
    pub fn hasInvalidUrl(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasInvalidUrl();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasInvalidUrl())
    }

    // Java: void deleteInvalidUrl()
    pub fn deleteInvalidUrl(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putInvalidUrl(String value)
    pub fn putInvalidUrl(&mut self, value: &str) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(2, Encode::from_string(value, self.zone));
    }

    // Java: String action()
    pub fn get_action(&self) -> String {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? "" : this.parent.action();
         return self.parent.as_ref().map_or(String::new(), |p| p.get_action());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasAction()
    pub fn hasAction(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasAction();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasAction())
    }

    // Java: void deleteAction()
    pub fn deleteAction(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }

    // Java: void putAction(String value)
    pub fn putAction(&mut self, value: &str) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(3, Encode::from_string(value, self.zone));
    }

    // Java: String tgtFrame()
    pub fn get_tgtFrame(&self) -> String {
      // Java: if (!this.map.has(4))
        if !self.map.has(4)
      {
         // Java: return this.parent == null ? "" : this.parent.tgtFrame();
         return self.parent.as_ref().map_or(String::new(), |p| p.get_tgtFrame());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(4);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasTgtFrame()
    pub fn hasTgtFrame(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasTgtFrame();
      self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasTgtFrame())
    }

    // Java: void deleteTgtFrame()
    pub fn deleteTgtFrame(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
    }

    // Java: void putTgtFrame(String value)
    pub fn putTgtFrame(&mut self, value: &str) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(4, Encode::from_string(value, self.zone));
    }

    // Java: String tooltip()
    pub fn get_tooltip(&self) -> String {
      // Java: if (!this.map.has(5))
        if !self.map.has(5)
      {
         // Java: return this.parent == null ? "" : this.parent.tooltip();
         return self.parent.as_ref().map_or(String::new(), |p| p.get_tooltip());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(5);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasTooltip()
    pub fn hasTooltip(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasTooltip();
      self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasTooltip())
    }

    // Java: void deleteTooltip()
    pub fn deleteTooltip(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
    }

    // Java: void putTooltip(String value)
    pub fn putTooltip(&mut self, value: &str) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(5, Encode::from_string(value, self.zone));
    }

    // Java: boolean isHistory()
    pub fn isHistory(&self) -> bool {
      // Java: if (!this.map.has(6))
        if !self.map.has(6)
      {
         // Java: return this.parent == null ? true : this.parent.isHistory();
         return self.parent.as_ref().map_or(true, |p| p.isHistory());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(6);
         let bin = self.map.byKey(6);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasHistory()
    pub fn hasHistory(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasHistory();
      self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasHistory())
    }

    // Java: void deleteHistory()
    pub fn deleteHistory(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
    }

    // Java: void putHistory(boolean value)
    pub fn putHistory(&mut self, value: bool) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(6, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isHighlightClick()
    pub fn isHighlightClick(&self) -> bool {
      // Java: if (!this.map.has(7))
        if !self.map.has(7)
      {
         // Java: return this.parent == null ? false : this.parent.isHighlightClick();
         return self.parent.as_ref().map_or(false, |p| p.isHighlightClick());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(7);
         let bin = self.map.byKey(7);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasHighlightClick()
    pub fn hasHighlightClick(&self) -> bool {
      // Java: return this.map.has(7) || this.parent != null && this.parent.hasHighlightClick();
      self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasHighlightClick())
    }

    // Java: void deleteHighlightClick()
    pub fn deleteHighlightClick(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
    }

    // Java: void putHighlightClick(boolean value)
    pub fn putHighlightClick(&mut self, value: bool) {
      // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
      self.map.put(7, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isEndSound()
    pub fn isEndSound(&self) -> bool {
      // Java: if (!this.map.has(8))
        if !self.map.has(8)
      {
         // Java: return this.parent == null ? false : this.parent.isEndSound();
         return self.parent.as_ref().map_or(false, |p| p.isEndSound());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(8);
         let bin = self.map.byKey(8);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasEndSound()
    pub fn hasEndSound(&self) -> bool {
      // Java: return this.map.has(8) || this.parent != null && this.parent.hasEndSound();
      self.map.has(8) || self.parent.as_ref().map_or(false, |p| p.hasEndSound())
    }

    // Java: void deleteEndSound()
    pub fn deleteEndSound(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
    }

    // Java: void putEndSound(boolean value)
    pub fn putEndSound(&mut self, value: bool) {
      // Java: this.map.put(8, KoBin.Encode.from(value, this.zone));
      self.map.put(8, Encode::from_bool(value, self.zone));
    }

    // Java: String target()
    pub fn get_target(&self) -> String {
      // Java: if (!this.map.has(9))
        if !self.map.has(9)
      {
         // Java: return this.parent == null ? "" : this.parent.target();
         return self.parent.as_ref().map_or(String::new(), |p| p.get_target());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(9);
         let bin = self.map.byKey(9);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasTarget()
    pub fn hasTarget(&self) -> bool {
      // Java: return this.map.has(9) || this.parent != null && this.parent.hasTarget();
      self.map.has(9) || self.parent.as_ref().map_or(false, |p| p.hasTarget())
    }

    // Java: void deleteTarget()
    pub fn deleteTarget(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
    }

    // Java: void putTarget(String value)
    pub fn putTarget(&mut self, value: &str) {
      // Java: this.map.put(9, KoBin.Encode.from(value, this.zone));
      self.map.put(9, Encode::from_string(value, self.zone));
    }

    // Java: boolean isTargetInternalMode()
    pub fn isTargetInternalMode(&self) -> bool {
      // Java: if (!this.map.has(10))
        if !self.map.has(10)
      {
         // Java: return this.parent == null ? true : this.parent.isTargetInternalMode();
         return self.parent.as_ref().map_or(true, |p| p.isTargetInternalMode());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(10);
         let bin = self.map.byKey(10);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasTargetInternalMode()
    pub fn hasTargetInternalMode(&self) -> bool {
      // Java: return this.map.has(10) || this.parent != null && this.parent.hasTargetInternalMode();
      self.map.has(10) || self.parent.as_ref().map_or(false, |p| p.hasTargetInternalMode())
    }

    // Java: void deleteTargetInternalMode()
    pub fn deleteTargetInternalMode(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
    }

    // Java: void putTargetInternalMode(boolean value)
    pub fn putTargetInternalMode(&mut self, value: bool) {
      // Java: this.map.put(10, KoBin.Encode.from(value, this.zone));
      self.map.put(10, Encode::from_bool(value, self.zone));
    }

    // Java: String type()
    pub fn get_type(&self) -> String {
      // Java: if (!this.map.has(11))
        if !self.map.has(11)
      {
         // Java: return this.parent == null ? "" : this.parent.type();
         return self.parent.as_ref().map_or(String::new(), |p| p.get_type());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(11);
         let bin = self.map.byKey(11);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(11) || this.parent != null && this.parent.hasType();
      self.map.has(11) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.put(11, (KoBin)null);
      self.map.clear_key(11);
    }

    // Java: void putType(String value)
    pub fn putType(&mut self, value: &str) {
      // Java: this.map.put(11, KoBin.Encode.from(value, this.zone));
      self.map.put(11, Encode::from_string(value, self.zone));
    }

    // Java: boolean isClicked()
    pub fn isClicked(&self) -> bool {
      // Java: if (!this.map.has(12))
        if !self.map.has(12)
      {
         // Java: return this.parent == null ? false : this.parent.isClicked();
         return self.parent.as_ref().map_or(false, |p| p.isClicked());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(12);
         let bin = self.map.byKey(12);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasClicked()
    pub fn hasClicked(&self) -> bool {
      // Java: return this.map.has(12) || this.parent != null && this.parent.hasClicked();
      self.map.has(12) || self.parent.as_ref().map_or(false, |p| p.hasClicked())
    }

    // Java: void deleteClicked()
    pub fn deleteClicked(&mut self) {
      // Java: this.map.put(12, (KoBin)null);
      self.map.clear_key(12);
    }

    // Java: void putClicked(boolean value)
    pub fn putClicked(&mut self, value: bool) {
      // Java: this.map.put(12, KoBin.Encode.from(value, this.zone));
      self.map.put(12, Encode::from_bool(value, self.zone));
    }

}

impl KoAr for Hyperlink {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { Hyperlink::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Hyperlink::lastBin(self, var1) }
}

impl KoClosable for Hyperlink {
    fn close(&mut self) { Hyperlink::close(self); }
}
