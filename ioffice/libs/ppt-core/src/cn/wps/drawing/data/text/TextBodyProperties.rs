#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.geometry.GeomGuide
use crate::cn::wps::drawing::data::geometry::GeomGuide::GeomGuide;
// Java: import cn.wps.drawing.data.threeD.Apply3DShapeProperties
use crate::cn::wps::drawing::data::threeD::Apply3DShapeProperties::Apply3DShapeProperties;
// Java: import cn.wps.drawing.data.threeD.Scene3DProperties
use crate::cn::wps::drawing::data::threeD::Scene3DProperties::Scene3DProperties;
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

// Java: public class TextBodyProperties implements KoAr, KoClosable
pub struct TextBodyProperties {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Apply3DShapeProperties sp3d
    pub(crate) sp3d: Option<Box<Apply3DShapeProperties>>,
    // Java field: Autofit textAutofit
    pub(crate) textAutofit: Option<Box<Autofit>>,
    // Java field: PresetTextWarp presetTextWarp
    pub(crate) presetTextWarp: Option<Box<PresetTextWarp>>,
    // Java field: Scene3DProperties scene3d
    pub(crate) scene3d: Option<Box<Scene3DProperties>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextBodyProperties parent
    pub(crate) parent: Option<Box<TextBodyProperties>>,
}

// Java: public class PresetTextWarp
pub struct PresetTextWarp {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: AdjustValues adjustValueLst
    pub(crate) adjustValueLst: Option<Box<AdjustValues>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: PresetTextWarp parent
    pub(crate) parent: Option<Box<PresetTextWarp>>,
}

// Java: public class AdjustValues
pub struct AdjustValues {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class Autofit
pub struct Autofit {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Autofit parent
    pub(crate) parent: Option<Box<Autofit>>,
}

// Java: public class TextBodyProperties implements KoAr, KoClosable
impl TextBodyProperties {
    // Java: static protected static <Value1 extends TextBodyProperties> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TextBodyProperties_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TextBodyProperties create()
    pub fn create() -> Self {
      // Java: return initialize(TextBodyProperties.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TextBodyProperties open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TextBodyProperties.class, bin);
      Self::initialize(bin)
    }

    // Java: TextBodyProperties()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self { bin_box: KpBinBox::new(), map: KpMap::new(), zone: KoZone::defaultZone(), parent: None,
        sp3d: None, textAutofit: None, presetTextWarp: None, scene3d: None }
    }

    // Java: TextBodyProperties(KoBin bin)
    pub fn TextBodyProperties_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), zone: KoZone::defaultZone(), parent: None,
        sp3d: None, textAutofit: None, presetTextWarp: None, scene3d: None }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
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
      self.loadBin(bin, zone);
    }

    // Java: void load(TextBodyProperties value)
    pub fn load_value(&mut self, value: Option<&TextBodyProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.sp3d != null && this.sp3d.dirty())
        if self.sp3d.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.sp3d.saveBin(zone));
         let bin = self.sp3d.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }
      // Java: if (this.textAutofit != null && this.textAutofit.dirty())
        if self.textAutofit.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(3, this.textAutofit.saveBin(zone));
         let bin = self.textAutofit.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }
      // Java: if (this.presetTextWarp != null && this.presetTextWarp.dirty())
        if self.presetTextWarp.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(4, this.presetTextWarp.saveBin(zone));
         let bin = self.presetTextWarp.as_mut().unwrap().saveBin(zone);
         self.map.put(4, bin);
      }
      // Java: if (this.scene3d != null && this.scene3d.dirty())
        if self.scene3d.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(5, this.scene3d.saveBin(zone));
         let bin = self.scene3d.as_mut().unwrap().saveBin(zone);
         self.map.put(5, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TextBodyProperties o)
    pub fn equalsLax(&self, o: &TextBodyProperties) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextBodyProperties) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TextBodyProperties, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TextBodyProperties)
        if true
      {
         // Java: TextBodyProperties dst = (TextBodyProperties)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasFlatTx() || !dst.hasFlatTx()) && (!this.hasFlatTx() || dst.hasFlatTx()))
            else if (self.hasFlatTx() || !dst.hasFlatTx()) && (!self.hasFlatTx() || dst.hasFlatTx())
         {
            // Java: if (this.hasFlatTx() && dst.hasFlatTx() && this.flatTx() != dst.flatTx())
                if self.hasFlatTx() && dst.hasFlatTx() && self.flatTx() != dst.flatTx()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasSp3d() || !dst.hasSp3d()) && (!this.hasSp3d() || dst.hasSp3d()))
                else if (self.hasSp3d() || !dst.hasSp3d()) && (!self.hasSp3d() || dst.hasSp3d())
            {
               // Java: if (this.hasSp3d() && dst.hasSp3d())
                  if self.hasSp3d() && dst.hasSp3d()
               {
                  // Java: if (isStrict && !this.sp3d().equals(dst.sp3d()))
                     if isStrict
                  {
                     // Java: return false;
                  }

                  // Java: if (!isStrict && !this.sp3d().equalsLax(dst.sp3d()))
                     if !isStrict
                  {
                     // Java: return false;
                  }
               }

               // Java: if ((this.hasTextAutofit() || !dst.hasTextAutofit()) && (!this.hasTextAutofit() || dst.hasTextAutofit()))
                  if (self.hasTextAutofit() || !dst.hasTextAutofit()) && (!self.hasTextAutofit() || dst.hasTextAutofit())
               {
               // Java: if (this.hasTextAutofit() && dst.hasTextAutofit())
                  if self.hasTextAutofit() && dst.hasTextAutofit()
               {
                  // Java: if (isStrict && !this.textAutofit().equals(dst.textAutofit()))
                     if isStrict
                  {
                     // Java: return false;
                  }

                  // Java: if (!isStrict && !this.textAutofit().equalsLax(dst.textAutofit()))
                     if !isStrict
                  {
                     // Java: return false;
                  }
               }

               // Java: if ((this.hasPresetTextWarp() || !dst.hasPresetTextWarp()) && (!this.hasPresetTextWarp() || dst.hasPresetTextWarp()))
                  if (self.hasPresetTextWarp() || !dst.hasPresetTextWarp()) && (!self.hasPresetTextWarp() || dst.hasPresetTextWarp())
               {
               // Java: if (this.hasPresetTextWarp() && dst.hasPresetTextWarp())
                  if self.hasPresetTextWarp() && dst.hasPresetTextWarp()
               {
                  // Java: if (isStrict && !this.presetTextWarp().equals(dst.presetTextWarp()))
                     if isStrict
                  {
                     // Java: return false;
                  }

                  // Java: if (!isStrict && !this.presetTextWarp().equalsLax(dst.presetTextWarp()))
                     if !isStrict
                  {
                     // Java: return false;
                  }
               }

               // Java: if ((this.hasScene3d() || !dst.hasScene3d()) && (!this.hasScene3d() || dst.hasScene3d()))
                  if (self.hasScene3d() || !dst.hasScene3d()) && (!self.hasScene3d() || dst.hasScene3d())
               {
               // Java: if (this.hasScene3d() && dst.hasScene3d())
                  if self.hasScene3d() && dst.hasScene3d()
               {
                  // Java: if (isStrict && !this.scene3d().equals(dst.scene3d()))
                     if isStrict
                  {
                     // Java: return false;
                  }

                  // Java: if (!isStrict && !this.scene3d().equalsLax(dst.scene3d()))
                     if !isStrict
                  {
                     // Java: return false;
                  }
               }

                  // Java: if ((this.hasAnchorCenter() || !dst.hasAnchorCenter()) && (!this.hasAnchorCenter() || dst.hasAnchorCenter()))
                     if (self.hasAnchorCenter() || !dst.hasAnchorCenter()) && (!self.hasAnchorCenter() || dst.hasAnchorCenter())
                  {
                     // Java: if (this.hasAnchorCenter() && dst.hasAnchorCenter() && this.isAnchorCenter() != dst.isAnchorCenter())
                        if self.hasAnchorCenter() && dst.hasAnchorCenter() && self.isAnchorCenter() != dst.isAnchorCenter()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasCompatibleLineSpacing() || !dst.hasCompatibleLineSpacing()) && (!this.hasCompatibleLineSpacing() || dst.hasCompatibleLineSpacing()))
                        else if (self.hasCompatibleLineSpacing() || !dst.hasCompatibleLineSpacing()) && (!self.hasCompatibleLineSpacing() || dst.hasCompatibleLineSpacing())
                     {
                        // Java: if (this.hasCompatibleLineSpacing() && dst.hasCompatibleLineSpacing() && this.isCompatibleLineSpacing() != dst.isCompatibleLineSpacing())
                           if self.hasCompatibleLineSpacing() && dst.hasCompatibleLineSpacing() && self.isCompatibleLineSpacing() != dst.isCompatibleLineSpacing()
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasForceAntiAlias() || !dst.hasForceAntiAlias()) && (!this.hasForceAntiAlias() || dst.hasForceAntiAlias()))
                           else if (self.hasForceAntiAlias() || !dst.hasForceAntiAlias()) && (!self.hasForceAntiAlias() || dst.hasForceAntiAlias())
                        {
                           // Java: if (this.hasForceAntiAlias() && dst.hasForceAntiAlias() && this.isForceAntiAlias() != dst.isForceAntiAlias())
                              if self.hasForceAntiAlias() && dst.hasForceAntiAlias() && self.isForceAntiAlias() != dst.isForceAntiAlias()
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasFromWordArt() || !dst.hasFromWordArt()) && (!this.hasFromWordArt() || dst.hasFromWordArt()))
                              else if (self.hasFromWordArt() || !dst.hasFromWordArt()) && (!self.hasFromWordArt() || dst.hasFromWordArt())
                           {
                              // Java: if (this.hasFromWordArt() && dst.hasFromWordArt() && this.isFromWordArt() != dst.isFromWordArt())
                                 if self.hasFromWordArt() && dst.hasFromWordArt() && self.isFromWordArt() != dst.isFromWordArt()
                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: else if ((this.hasRight2LeftColumns() || !dst.hasRight2LeftColumns()) && (!this.hasRight2LeftColumns() || dst.hasRight2LeftColumns()))
                                 else if (self.hasRight2LeftColumns() || !dst.hasRight2LeftColumns()) && (!self.hasRight2LeftColumns() || dst.hasRight2LeftColumns())
                              {
                                 // Java: if (this.hasRight2LeftColumns() && dst.hasRight2LeftColumns() && this.isRight2LeftColumns() != dst.isRight2LeftColumns())
                                    if self.hasRight2LeftColumns() && dst.hasRight2LeftColumns() && self.isRight2LeftColumns() != dst.isRight2LeftColumns()
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: else if ((this.hasSpcFirstLastPara() || !dst.hasSpcFirstLastPara()) && (!this.hasSpcFirstLastPara() || dst.hasSpcFirstLastPara()))
                                    else if (self.hasSpcFirstLastPara() || !dst.hasSpcFirstLastPara()) && (!self.hasSpcFirstLastPara() || dst.hasSpcFirstLastPara())
                                 {
                                    // Java: if (this.hasSpcFirstLastPara() && dst.hasSpcFirstLastPara() && this.isSpcFirstLastPara() != dst.isSpcFirstLastPara())
                                       if self.hasSpcFirstLastPara() && dst.hasSpcFirstLastPara() && self.isSpcFirstLastPara() != dst.isSpcFirstLastPara()
                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: else if ((this.hasTextUpright() || !dst.hasTextUpright()) && (!this.hasTextUpright() || dst.hasTextUpright()))
                                       else if (self.hasTextUpright() || !dst.hasTextUpright()) && (!self.hasTextUpright() || dst.hasTextUpright())
                                    {
                                       // Java: if (this.hasTextUpright() && dst.hasTextUpright() && this.isTextUpright() != dst.isTextUpright())
                                          if self.hasTextUpright() && dst.hasTextUpright() && self.isTextUpright() != dst.isTextUpright()
                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                       // Java: else if ((this.hasTextHorizontalOverflow() || !dst.hasTextHorizontalOverflow()) && (!this.hasTextHorizontalOverflow() || dst.hasTextHorizontalOverflow()))
                                          else if (self.hasTextHorizontalOverflow() || !dst.hasTextHorizontalOverflow()) && (!self.hasTextHorizontalOverflow() || dst.hasTextHorizontalOverflow())
                                       {
                                          // Java: if (this.hasTextHorizontalOverflow() && dst.hasTextHorizontalOverflow() && this.textHorizontalOverflow() != dst.textHorizontalOverflow())
                                             if self.hasTextHorizontalOverflow() && dst.hasTextHorizontalOverflow() && self.textHorizontalOverflow() != dst.textHorizontalOverflow()
                                          {
                                             // Java: return false;
                                             return false;
                                          }
                                          // Java: else if ((this.hasTextVerticalOverflow() || !dst.hasTextVerticalOverflow()) && (!this.hasTextVerticalOverflow() || dst.hasTextVerticalOverflow()))
                                             else if (self.hasTextVerticalOverflow() || !dst.hasTextVerticalOverflow()) && (!self.hasTextVerticalOverflow() || dst.hasTextVerticalOverflow())
                                          {
                                             // Java: if (this.hasTextVerticalOverflow() && dst.hasTextVerticalOverflow() && this.textVerticalOverflow() != dst.textVerticalOverflow())
                                                if self.hasTextVerticalOverflow() && dst.hasTextVerticalOverflow() && self.textVerticalOverflow() != dst.textVerticalOverflow()
                                             {
                                                // Java: return false;
                                                return false;
                                             }
                                             // Java: else if ((this.hasAnchor() || !dst.hasAnchor()) && (!this.hasAnchor() || dst.hasAnchor()))
                                                else if (self.hasAnchor() || !dst.hasAnchor()) && (!self.hasAnchor() || dst.hasAnchor())
                                             {
                                                // Java: if (this.hasAnchor() && dst.hasAnchor() && this.anchor() != dst.anchor())
                                                   if self.hasAnchor() && dst.hasAnchor() && self.anchor() != dst.anchor()
                                                {
                                                   // Java: return false;
                                                   return false;
                                                }
                                                // Java: else if ((this.hasBottomInset() || !dst.hasBottomInset()) && (!this.hasBottomInset() || dst.hasBottomInset()))
                                                   else if (self.hasBottomInset() || !dst.hasBottomInset()) && (!self.hasBottomInset() || dst.hasBottomInset())
                                                {
                                                   // Java: if (this.hasBottomInset() && dst.hasBottomInset() && this.bottomInset() != dst.bottomInset())
                                                      if self.hasBottomInset() && dst.hasBottomInset() && self.bottomInset() != dst.bottomInset()
                                                   {
                                                      // Java: return false;
                                                      return false;
                                                   }
                                                   // Java: else if ((this.hasLeftInset() || !dst.hasLeftInset()) && (!this.hasLeftInset() || dst.hasLeftInset()))
                                                      else if (self.hasLeftInset() || !dst.hasLeftInset()) && (!self.hasLeftInset() || dst.hasLeftInset())
                                                   {
                                                      // Java: if (this.hasLeftInset() && dst.hasLeftInset() && this.leftInset() != dst.leftInset())
                                                         if self.hasLeftInset() && dst.hasLeftInset() && self.leftInset() != dst.leftInset()
                                                      {
                                                         // Java: return false;
                                                         return false;
                                                      }
                                                      // Java: else if ((this.hasTopInset() || !dst.hasTopInset()) && (!this.hasTopInset() || dst.hasTopInset()))
                                                         else if (self.hasTopInset() || !dst.hasTopInset()) && (!self.hasTopInset() || dst.hasTopInset())
                                                      {
                                                         // Java: if (this.hasTopInset() && dst.hasTopInset() && this.topInset() != dst.topInset())
                                                            if self.hasTopInset() && dst.hasTopInset() && self.topInset() != dst.topInset()
                                                         {
                                                            // Java: return false;
                                                            return false;
                                                         }
                                                         // Java: else if ((this.hasRightInset() || !dst.hasRightInset()) && (!this.hasRightInset() || dst.hasRightInset()))
                                                            else if (self.hasRightInset() || !dst.hasRightInset()) && (!self.hasRightInset() || dst.hasRightInset())
                                                         {
                                                            // Java: if (this.hasRightInset() && dst.hasRightInset() && this.rightInset() != dst.rightInset())
                                                               if self.hasRightInset() && dst.hasRightInset() && self.rightInset() != dst.rightInset()
                                                            {
                                                               // Java: return false;
                                                               return false;
                                                            }
                                                            // Java: else if ((this.hasNumberOfColumns() || !dst.hasNumberOfColumns()) && (!this.hasNumberOfColumns() || dst.hasNumberOfColumns()))
                                                               else if (self.hasNumberOfColumns() || !dst.hasNumberOfColumns()) && (!self.hasNumberOfColumns() || dst.hasNumberOfColumns())
                                                            {
                                                               // Java: if (this.hasNumberOfColumns() && dst.hasNumberOfColumns() && this.numberOfColumns() != dst.numberOfColumns())
                                                                  if self.hasNumberOfColumns() && dst.hasNumberOfColumns() && self.numberOfColumns() != dst.numberOfColumns()
                                                               {
                                                                  // Java: return false;
                                                                  return false;
                                                               }
                                                               // Java: else if ((this.hasSpaceBetweenColumns() || !dst.hasSpaceBetweenColumns()) && (!this.hasSpaceBetweenColumns() || dst.hasSpaceBetweenColumns()))
                                                                  else if (self.hasSpaceBetweenColumns() || !dst.hasSpaceBetweenColumns()) && (!self.hasSpaceBetweenColumns() || dst.hasSpaceBetweenColumns())
                                                               {
                                                                  // Java: if (this.hasSpaceBetweenColumns() && dst.hasSpaceBetweenColumns() && this.spaceBetweenColumns() != dst.spaceBetweenColumns())
                                                                     if self.hasSpaceBetweenColumns() && dst.hasSpaceBetweenColumns() && self.spaceBetweenColumns() != dst.spaceBetweenColumns()
                                                                  {
                                                                     // Java: return false;
                                                                     return false;
                                                                  }
                                                                  // Java: else if ((this.hasRotation() || !dst.hasRotation()) && (!this.hasRotation() || dst.hasRotation()))
                                                                     else if (self.hasRotation() || !dst.hasRotation()) && (!self.hasRotation() || dst.hasRotation())
                                                                  {
                                                                     // Java: if (this.hasRotation() && dst.hasRotation() && this.rotation() != dst.rotation())
                                                                        if self.hasRotation() && dst.hasRotation() && self.rotation() != dst.rotation()
                                                                     {
                                                                        // Java: return false;
                                                                        return false;
                                                                     }
                                                                     // Java: else if ((this.hasVerticalText() || !dst.hasVerticalText()) && (!this.hasVerticalText() || dst.hasVerticalText()))
                                                                        else if (self.hasVerticalText() || !dst.hasVerticalText()) && (!self.hasVerticalText() || dst.hasVerticalText())
                                                                     {
                                                                        // Java: if (this.hasVerticalText() && dst.hasVerticalText() && this.verticalText() != dst.verticalText())
                                                                           if self.hasVerticalText() && dst.hasVerticalText() && self.verticalText() != dst.verticalText()
                                                                        {
                                                                           // Java: return false;
                                                                           return false;
                                                                        }
                                                                        // Java: else if ((this.hasTextWrappingType() || !dst.hasTextWrappingType()) && (!this.hasTextWrappingType() || dst.hasTextWrappingType()))
                                                                           else if (self.hasTextWrappingType() || !dst.hasTextWrappingType()) && (!self.hasTextWrappingType() || dst.hasTextWrappingType())
                                                                        {
                                                                           // Java: if (this.hasTextWrappingType() && dst.hasTextWrappingType() && this.textWrappingType() != dst.textWrappingType())
                                                                              if self.hasTextWrappingType() && dst.hasTextWrappingType() && self.textWrappingType() != dst.textWrappingType()
                                                                           {
                                                                              // Java: return false;
                                                                              return false;
                                                                           }
                                                                           // Java: else if ((this.hasTextFontSizeFit() || !dst.hasTextFontSizeFit()) && (!this.hasTextFontSizeFit() || dst.hasTextFontSizeFit()))
                                                                              else if (self.hasTextFontSizeFit() || !dst.hasTextFontSizeFit()) && (!self.hasTextFontSizeFit() || dst.hasTextFontSizeFit())
                                                                           {
                                                                              // Java: return !this.hasTextFontSizeFit() || !dst.hasTextFontSizeFit() || this.isTextFontSizeFit() == dst.isTextFontSizeFit();
                                                                              return !self.hasTextFontSizeFit() || !dst.hasTextFontSizeFit() || self.isTextFontSizeFit() == dst.isTextFontSizeFit();
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
      // Java: return false;
      return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasFlatTx())
        if self.hasFlatTx()
      {
         // Java: ret += Integer.valueOf(this.flatTx()).hashCode();
         ret = ret.wrapping_add(self.flatTx());
      }
      // Java: if (this.hasSp3d())
        if self.hasSp3d()
      {
         // Java: ret += this.sp3d().hashCode();
         todo!()
      }
      // Java: if (this.hasTextAutofit())
        if self.hasTextAutofit()
      {
         // Java: ret += this.textAutofit().hashCode();
         todo!()
      }
      // Java: if (this.hasPresetTextWarp())
        if self.hasPresetTextWarp()
      {
         // Java: ret += this.presetTextWarp().hashCode();
         todo!()
      }
      // Java: if (this.hasScene3d())
        if self.hasScene3d()
      {
         // Java: ret += this.scene3d().hashCode();
         todo!()
      }
      // Java: if (this.hasAnchorCenter())
        if self.hasAnchorCenter()
      {
         // Java: ret += Boolean.valueOf(this.isAnchorCenter()).hashCode();
         ret = ret.wrapping_add(if self.isAnchorCenter() { 1231 } else { 1237 });
      }
      // Java: if (this.hasCompatibleLineSpacing())
        if self.hasCompatibleLineSpacing()
      {
         // Java: ret += Boolean.valueOf(this.isCompatibleLineSpacing()).hashCode();
         ret = ret.wrapping_add(if self.isCompatibleLineSpacing() { 1231 } else { 1237 });
      }
      // Java: if (this.hasForceAntiAlias())
        if self.hasForceAntiAlias()
      {
         // Java: ret += Boolean.valueOf(this.isForceAntiAlias()).hashCode();
         ret = ret.wrapping_add(if self.isForceAntiAlias() { 1231 } else { 1237 });
      }
      // Java: if (this.hasFromWordArt())
        if self.hasFromWordArt()
      {
         // Java: ret += Boolean.valueOf(this.isFromWordArt()).hashCode();
         ret = ret.wrapping_add(if self.isFromWordArt() { 1231 } else { 1237 });
      }
      // Java: if (this.hasRight2LeftColumns())
        if self.hasRight2LeftColumns()
      {
         // Java: ret += Boolean.valueOf(this.isRight2LeftColumns()).hashCode();
         ret = ret.wrapping_add(if self.isRight2LeftColumns() { 1231 } else { 1237 });
      }
      // Java: if (this.hasSpcFirstLastPara())
        if self.hasSpcFirstLastPara()
      {
         // Java: ret += Boolean.valueOf(this.isSpcFirstLastPara()).hashCode();
         ret = ret.wrapping_add(if self.isSpcFirstLastPara() { 1231 } else { 1237 });
      }
      // Java: if (this.hasTextUpright())
        if self.hasTextUpright()
      {
         // Java: ret += Boolean.valueOf(this.isTextUpright()).hashCode();
         ret = ret.wrapping_add(if self.isTextUpright() { 1231 } else { 1237 });
      }
      // Java: if (this.hasTextHorizontalOverflow())
        if self.hasTextHorizontalOverflow()
      {
         // Java: ret += Integer.valueOf(this.textHorizontalOverflow()).hashCode();
         ret = ret.wrapping_add(self.textHorizontalOverflow());
      }
      // Java: if (this.hasTextVerticalOverflow())
        if self.hasTextVerticalOverflow()
      {
         // Java: ret += Integer.valueOf(this.textVerticalOverflow()).hashCode();
         ret = ret.wrapping_add(self.textVerticalOverflow());
      }
      // Java: if (this.hasAnchor())
        if self.hasAnchor()
      {
         // Java: ret += Integer.valueOf(this.anchor()).hashCode();
         ret = ret.wrapping_add(self.anchor());
      }
      // Java: if (this.hasBottomInset())
        if self.hasBottomInset()
      {
         // Java: ret += Integer.valueOf(this.bottomInset()).hashCode();
         ret = ret.wrapping_add(self.bottomInset());
      }
      // Java: if (this.hasLeftInset())
        if self.hasLeftInset()
      {
         // Java: ret += Integer.valueOf(this.leftInset()).hashCode();
         ret = ret.wrapping_add(self.leftInset());
      }
      // Java: if (this.hasTopInset())
        if self.hasTopInset()
      {
         // Java: ret += Integer.valueOf(this.topInset()).hashCode();
         ret = ret.wrapping_add(self.topInset());
      }
      // Java: if (this.hasRightInset())
        if self.hasRightInset()
      {
         // Java: ret += Integer.valueOf(this.rightInset()).hashCode();
         ret = ret.wrapping_add(self.rightInset());
      }
      // Java: if (this.hasNumberOfColumns())
        if self.hasNumberOfColumns()
      {
         // Java: ret += Integer.valueOf(this.numberOfColumns()).hashCode();
         ret = ret.wrapping_add(self.numberOfColumns());
      }
      // Java: if (this.hasSpaceBetweenColumns())
        if self.hasSpaceBetweenColumns()
      {
         // Java: ret += Integer.valueOf(this.spaceBetweenColumns()).hashCode();
         ret = ret.wrapping_add(self.spaceBetweenColumns());
      }
      // Java: if (this.hasRotation())
        if self.hasRotation()
      {
         // Java: ret += Integer.valueOf(this.rotation()).hashCode();
         ret = ret.wrapping_add(self.rotation());
      }
      // Java: if (this.hasVerticalText())
        if self.hasVerticalText()
      {
         // Java: ret += Integer.valueOf(this.verticalText()).hashCode();
         ret = ret.wrapping_add(self.verticalText());
      }
      // Java: if (this.hasTextWrappingType())
        if self.hasTextWrappingType()
      {
         // Java: ret += Integer.valueOf(this.textWrappingType()).hashCode();
         ret = ret.wrapping_add(self.textWrappingType());
      }
      // Java: if (this.hasTextFontSizeFit())
        if self.hasTextFontSizeFit()
      {
         // Java: ret += Boolean.valueOf(this.isTextFontSizeFit()).hashCode();
         ret = ret.wrapping_add(if self.isTextFontSizeFit() { 1231 } else { 1237 });
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.sp3d != null && this.sp3d.dirty())
        if self.sp3d.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.textAutofit != null && this.textAutofit.dirty())
        else if self.textAutofit.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.presetTextWarp != null && this.presetTextWarp.dirty())
        else if self.presetTextWarp.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.scene3d != null && this.scene3d.dirty() ? true : this.map.dirty();
         return self.scene3d.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty();
      }
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
      let null_bin = Null::empty();
      self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.set(None);
      // Java: this.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.sp3d = null;
      self.sp3d = None;
      // Java: this.textAutofit = null;
      self.textAutofit = None;
      // Java: this.presetTextWarp = null;
      self.presetTextWarp = None;
      // Java: this.scene3d = null;
      self.scene3d = None;
    }

    // Java: void follow(TextBodyProperties parent)
    pub fn follow(&mut self, parent: Option<&mut TextBodyProperties>) {
      // Java: if (parent != null)
        if let Some(p) = parent
      {
         // Java: parent.discard();
         let pz = p.zone;
         p.discard_with_zone(pz);
         // Java: this.discard();
         let z = self.zone;
         self.discard_with_zone(z);
         // Java: this.parent = parent;
         todo!()
      }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
        if self.parent.is_some()
      {
         // Java: this.discard();
         let z = self.zone;
         self.discard_with_zone(z);
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { let pz = p.zone; p.discard_with_zone(pz); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: TextBodyProperties parent()
    pub fn parent(&self) -> Option<&TextBodyProperties> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let z = self.zone;
      self.discard_with_zone(z);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: int flatTx()
    pub fn flatTx(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 0 : this.parent.flatTx();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().flatTx() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasFlatTx()
    pub fn hasFlatTx(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasFlatTx();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasFlatTx())
    }

    // Java: void deleteFlatTx()
    pub fn deleteFlatTx(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putFlatTx(int value)
    pub fn putFlatTx(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(1, bin);
    }

    // Java: Apply3DShapeProperties sp3d()
    pub fn sp3d(&mut self) -> &Apply3DShapeProperties {
      // Java: if (this.sp3d == null)
        if self.sp3d.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.sp3d = (Apply3DShapeProperties)KoBin.Decode.toAny(Apply3DShapeProperties.class, bin, this.zone);
            let bin = self.map.byKey(2);
            self.sp3d = Some(Box::new(Apply3DShapeProperties::Apply3DShapeProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasSp3d())
            else if self.parent.as_ref().map_or(false, |p| p.hasSp3d())
         {
            // Java: this.sp3d = this.parent.sp3d();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.sp3d = (Apply3DShapeProperties)KoBin.Decode.toAny(Apply3DShapeProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.sp3d = Some(Box::new(Apply3DShapeProperties::Apply3DShapeProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.sp3d;
      self.sp3d.as_deref().unwrap()
    }

    // Java: boolean hasSp3d()
    pub fn hasSp3d(&self) -> bool {
      // Java: return this.map.has(2) || this.sp3d != null && this.sp3d.dirty() || this.parent != null && this.parent.hasSp3d();
      self.map.has(2) || self.sp3d.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSp3d())
    }

    // Java: void deleteSp3d()
    pub fn deleteSp3d(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.sp3d != null)
        if self.sp3d.is_some()
      {
         // Java: this.sp3d = null;
         self.sp3d = None;
      }
    }

    // Java: void writeSp3d(Apply3DShapeProperties value)
    pub fn writeSp3d(&mut self, value: Option<&Apply3DShapeProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(2, bin);
            self.map.put(2, bin);
            // Java: if (this.sp3d != null)
               if self.sp3d.is_some()
            {
               // Java: this.sp3d = null;
               self.sp3d = None;
            }
         }
      }
    }

    // Java: void forceWriteSp3d(Apply3DShapeProperties value)
    pub fn forceWriteSp3d(&mut self, value: Option<&Apply3DShapeProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.sp3d != null)
            if self.sp3d.is_some()
         {
            // Java: this.sp3d = null;
            self.sp3d = None;
         }
      }
    }

    // Java: Autofit textAutofit()
    pub fn textAutofit(&mut self) -> &Autofit {
      // Java: if (this.textAutofit == null)
        if self.textAutofit.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            // Java: this.textAutofit = (Autofit)KoBin.Decode.toAny(Autofit.class, bin, this.zone);
            let bin = self.map.byKey(3);
            self.textAutofit = Some(Box::new(Autofit::Autofit_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasTextAutofit())
            else if self.parent.as_ref().map_or(false, |p| p.hasTextAutofit())
         {
            // Java: this.textAutofit = this.parent.textAutofit();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.textAutofit = (Autofit)KoBin.Decode.toAny(Autofit.class, bin, this.zone);
            let null_bin = Null::empty();
            self.textAutofit = Some(Box::new(Autofit::Autofit_from(null_bin.as_ref())));
         }
      }
      // Java: return this.textAutofit;
      self.textAutofit.as_deref().unwrap()
    }

    // Java: boolean hasTextAutofit()
    pub fn hasTextAutofit(&self) -> bool {
      // Java: return this.map.has(3) || this.textAutofit != null && this.textAutofit.dirty() || this.parent != null && this.parent.hasTextAutofit();
      self.map.has(3) || self.textAutofit.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTextAutofit())
    }

    // Java: void deleteTextAutofit()
    pub fn deleteTextAutofit(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.textAutofit != null)
        if self.textAutofit.is_some()
      {
         // Java: this.textAutofit = null;
         self.textAutofit = None;
      }
    }

    // Java: void writeTextAutofit(Autofit value)
    pub fn writeTextAutofit(&mut self, value: Option<&Autofit>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(3, bin);
            self.map.put(3, bin);
            // Java: if (this.textAutofit != null)
               if self.textAutofit.is_some()
            {
               // Java: this.textAutofit = null;
               self.textAutofit = None;
            }
         }
      }
    }

    // Java: void forceWriteTextAutofit(Autofit value)
    pub fn forceWriteTextAutofit(&mut self, value: Option<&Autofit>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.textAutofit != null)
            if self.textAutofit.is_some()
         {
            // Java: this.textAutofit = null;
            self.textAutofit = None;
         }
      }
    }

    // Java: PresetTextWarp presetTextWarp()
    pub fn presetTextWarp(&mut self) -> &PresetTextWarp {
      // Java: if (this.presetTextWarp == null)
        if self.presetTextWarp.is_none()
      {
         // Java: if (this.map.has(4))
            if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            // Java: this.presetTextWarp = (PresetTextWarp)KoBin.Decode.toAny(PresetTextWarp.class, bin, this.zone);
            let bin = self.map.byKey(4);
            self.presetTextWarp = Some(Box::new(PresetTextWarp::PresetTextWarp_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasPresetTextWarp())
            else if self.parent.as_ref().map_or(false, |p| p.hasPresetTextWarp())
         {
            // Java: this.presetTextWarp = this.parent.presetTextWarp();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.presetTextWarp = (PresetTextWarp)KoBin.Decode.toAny(PresetTextWarp.class, bin, this.zone);
            let null_bin = Null::empty();
            self.presetTextWarp = Some(Box::new(PresetTextWarp::PresetTextWarp_from(null_bin.as_ref())));
         }
      }
      // Java: return this.presetTextWarp;
      self.presetTextWarp.as_deref().unwrap()
    }

    // Java: boolean hasPresetTextWarp()
    pub fn hasPresetTextWarp(&self) -> bool {
      // Java: return this.map.has(4) || this.presetTextWarp != null && this.presetTextWarp.dirty() || this.parent != null && this.parent.hasPresetTextWarp();
      self.map.has(4) || self.presetTextWarp.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPresetTextWarp())
    }

    // Java: void deletePresetTextWarp()
    pub fn deletePresetTextWarp(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.presetTextWarp != null)
        if self.presetTextWarp.is_some()
      {
         // Java: this.presetTextWarp = null;
         self.presetTextWarp = None;
      }
    }

    // Java: void writePresetTextWarp(PresetTextWarp value)
    pub fn writePresetTextWarp(&mut self, value: Option<&PresetTextWarp>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(4, bin);
            self.map.put(4, bin);
            // Java: if (this.presetTextWarp != null)
               if self.presetTextWarp.is_some()
            {
               // Java: this.presetTextWarp = null;
               self.presetTextWarp = None;
            }
         }
      }
    }

    // Java: void forceWritePresetTextWarp(PresetTextWarp value)
    pub fn forceWritePresetTextWarp(&mut self, value: Option<&PresetTextWarp>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, v.lastBin(self.zone));
         // Java: if (this.presetTextWarp != null)
            if self.presetTextWarp.is_some()
         {
            // Java: this.presetTextWarp = null;
            self.presetTextWarp = None;
         }
      }
    }

    // Java: Scene3DProperties scene3d()
    pub fn scene3d(&mut self) -> &Scene3DProperties {
      // Java: if (this.scene3d == null)
        if self.scene3d.is_none()
      {
         // Java: if (this.map.has(5))
            if self.map.has(5)
         {
            // Java: KoBin bin = this.map.byKey(5);
            // Java: this.scene3d = (Scene3DProperties)KoBin.Decode.toAny(Scene3DProperties.class, bin, this.zone);
            let bin = self.map.byKey(5);
            self.scene3d = Some(Box::new(Scene3DProperties::Scene3DProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasScene3d())
            else if self.parent.as_ref().map_or(false, |p| p.hasScene3d())
         {
            // Java: this.scene3d = this.parent.scene3d();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.scene3d = (Scene3DProperties)KoBin.Decode.toAny(Scene3DProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.scene3d = Some(Box::new(Scene3DProperties::Scene3DProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.scene3d;
      self.scene3d.as_deref().unwrap()
    }

    // Java: boolean hasScene3d()
    pub fn hasScene3d(&self) -> bool {
      // Java: return this.map.has(5) || this.scene3d != null && this.scene3d.dirty() || this.parent != null && this.parent.hasScene3d();
      self.map.has(5) || self.scene3d.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasScene3d())
    }

    // Java: void deleteScene3d()
    pub fn deleteScene3d(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
      // Java: if (this.scene3d != null)
        if self.scene3d.is_some()
      {
         // Java: this.scene3d = null;
         self.scene3d = None;
      }
    }

    // Java: void writeScene3d(Scene3DProperties value)
    pub fn writeScene3d(&mut self, value: Option<&Scene3DProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(5, bin);
            self.map.put(5, bin);
            // Java: if (this.scene3d != null)
               if self.scene3d.is_some()
            {
               // Java: this.scene3d = null;
               self.scene3d = None;
            }
         }
      }
    }

    // Java: void forceWriteScene3d(Scene3DProperties value)
    pub fn forceWriteScene3d(&mut self, value: Option<&Scene3DProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(5, value.lastBin(this.zone));
         self.map.put(5, v.lastBin(self.zone));
         // Java: if (this.scene3d != null)
            if self.scene3d.is_some()
         {
            // Java: this.scene3d = null;
            self.scene3d = None;
         }
      }
    }

    // Java: boolean isAnchorCenter()
    pub fn isAnchorCenter(&self) -> bool {
      // Java: if (!this.map.has(6))
        if !self.map.has(6)
      {
         // Java: return this.parent == null ? false : this.parent.isAnchorCenter();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isAnchorCenter() };
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

    // Java: boolean hasAnchorCenter()
    pub fn hasAnchorCenter(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasAnchorCenter();
      self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasAnchorCenter())
    }

    // Java: void deleteAnchorCenter()
    pub fn deleteAnchorCenter(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
    }

    // Java: void putAnchorCenter(boolean value)
    pub fn putAnchorCenter(&mut self, value: bool) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_bool(value, self.zone);
      self.map.put(6, bin);
    }

    // Java: boolean isCompatibleLineSpacing()
    pub fn isCompatibleLineSpacing(&self) -> bool {
      // Java: if (!this.map.has(7))
        if !self.map.has(7)
      {
         // Java: return this.parent == null ? false : this.parent.isCompatibleLineSpacing();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isCompatibleLineSpacing() };
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

    // Java: boolean hasCompatibleLineSpacing()
    pub fn hasCompatibleLineSpacing(&self) -> bool {
      // Java: return this.map.has(7) || this.parent != null && this.parent.hasCompatibleLineSpacing();
      self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasCompatibleLineSpacing())
    }

    // Java: void deleteCompatibleLineSpacing()
    pub fn deleteCompatibleLineSpacing(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
    }

    // Java: void putCompatibleLineSpacing(boolean value)
    pub fn putCompatibleLineSpacing(&mut self, value: bool) {
      // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_bool(value, self.zone);
      self.map.put(7, bin);
    }

    // Java: boolean isForceAntiAlias()
    pub fn isForceAntiAlias(&self) -> bool {
      // Java: if (!this.map.has(8))
        if !self.map.has(8)
      {
         // Java: return this.parent == null ? false : this.parent.isForceAntiAlias();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isForceAntiAlias() };
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

    // Java: boolean hasForceAntiAlias()
    pub fn hasForceAntiAlias(&self) -> bool {
      // Java: return this.map.has(8) || this.parent != null && this.parent.hasForceAntiAlias();
      self.map.has(8) || self.parent.as_ref().map_or(false, |p| p.hasForceAntiAlias())
    }

    // Java: void deleteForceAntiAlias()
    pub fn deleteForceAntiAlias(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
    }

    // Java: void putForceAntiAlias(boolean value)
    pub fn putForceAntiAlias(&mut self, value: bool) {
      // Java: this.map.put(8, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_bool(value, self.zone);
      self.map.put(8, bin);
    }

    // Java: boolean isFromWordArt()
    pub fn isFromWordArt(&self) -> bool {
      // Java: if (!this.map.has(9))
        if !self.map.has(9)
      {
         // Java: return this.parent == null ? false : this.parent.isFromWordArt();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isFromWordArt() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(9);
         let bin = self.map.byKey(9);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasFromWordArt()
    pub fn hasFromWordArt(&self) -> bool {
      // Java: return this.map.has(9) || this.parent != null && this.parent.hasFromWordArt();
      self.map.has(9) || self.parent.as_ref().map_or(false, |p| p.hasFromWordArt())
    }

    // Java: void deleteFromWordArt()
    pub fn deleteFromWordArt(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
    }

    // Java: void putFromWordArt(boolean value)
    pub fn putFromWordArt(&mut self, value: bool) {
      // Java: this.map.put(9, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_bool(value, self.zone);
      self.map.put(9, bin);
    }

    // Java: boolean isRight2LeftColumns()
    pub fn isRight2LeftColumns(&self) -> bool {
      // Java: if (!this.map.has(10))
        if !self.map.has(10)
      {
         // Java: return this.parent == null ? false : this.parent.isRight2LeftColumns();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isRight2LeftColumns() };
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

    // Java: boolean hasRight2LeftColumns()
    pub fn hasRight2LeftColumns(&self) -> bool {
      // Java: return this.map.has(10) || this.parent != null && this.parent.hasRight2LeftColumns();
      self.map.has(10) || self.parent.as_ref().map_or(false, |p| p.hasRight2LeftColumns())
    }

    // Java: void deleteRight2LeftColumns()
    pub fn deleteRight2LeftColumns(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
    }

    // Java: void putRight2LeftColumns(boolean value)
    pub fn putRight2LeftColumns(&mut self, value: bool) {
      // Java: this.map.put(10, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_bool(value, self.zone);
      self.map.put(10, bin);
    }

    // Java: boolean isSpcFirstLastPara()
    pub fn isSpcFirstLastPara(&self) -> bool {
      // Java: if (!this.map.has(11))
        if !self.map.has(11)
      {
         // Java: return this.parent == null ? false : this.parent.isSpcFirstLastPara();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isSpcFirstLastPara() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(11);
         let bin = self.map.byKey(11);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasSpcFirstLastPara()
    pub fn hasSpcFirstLastPara(&self) -> bool {
      // Java: return this.map.has(11) || this.parent != null && this.parent.hasSpcFirstLastPara();
      self.map.has(11) || self.parent.as_ref().map_or(false, |p| p.hasSpcFirstLastPara())
    }

    // Java: void deleteSpcFirstLastPara()
    pub fn deleteSpcFirstLastPara(&mut self) {
      // Java: this.map.put(11, (KoBin)null);
      self.map.clear_key(11);
    }

    // Java: void putSpcFirstLastPara(boolean value)
    pub fn putSpcFirstLastPara(&mut self, value: bool) {
      // Java: this.map.put(11, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_bool(value, self.zone);
      self.map.put(11, bin);
    }

    // Java: boolean isTextUpright()
    pub fn isTextUpright(&self) -> bool {
      // Java: if (!this.map.has(12))
        if !self.map.has(12)
      {
         // Java: return this.parent == null ? false : this.parent.isTextUpright();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isTextUpright() };
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

    // Java: boolean hasTextUpright()
    pub fn hasTextUpright(&self) -> bool {
      // Java: return this.map.has(12) || this.parent != null && this.parent.hasTextUpright();
      self.map.has(12) || self.parent.as_ref().map_or(false, |p| p.hasTextUpright())
    }

    // Java: void deleteTextUpright()
    pub fn deleteTextUpright(&mut self) {
      // Java: this.map.put(12, (KoBin)null);
      self.map.clear_key(12);
    }

    // Java: void putTextUpright(boolean value)
    pub fn putTextUpright(&mut self, value: bool) {
      // Java: this.map.put(12, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_bool(value, self.zone);
      self.map.put(12, bin);
    }

    // Java: int textHorizontalOverflow()
    pub fn textHorizontalOverflow(&self) -> i32 {
      // Java: if (!this.map.has(13))
        if !self.map.has(13)
      {
         // Java: return this.parent == null ? 0 : this.parent.textHorizontalOverflow();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().textHorizontalOverflow() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(13);
         let bin = self.map.byKey(13);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasTextHorizontalOverflow()
    pub fn hasTextHorizontalOverflow(&self) -> bool {
      // Java: return this.map.has(13) || this.parent != null && this.parent.hasTextHorizontalOverflow();
      self.map.has(13) || self.parent.as_ref().map_or(false, |p| p.hasTextHorizontalOverflow())
    }

    // Java: void deleteTextHorizontalOverflow()
    pub fn deleteTextHorizontalOverflow(&mut self) {
      // Java: this.map.put(13, (KoBin)null);
      self.map.clear_key(13);
    }

    // Java: void putTextHorizontalOverflow(int value)
    pub fn putTextHorizontalOverflow(&mut self, value: i32) {
      // Java: this.map.put(13, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(13, bin);
    }

    // Java: int textVerticalOverflow()
    pub fn textVerticalOverflow(&self) -> i32 {
      // Java: if (!this.map.has(14))
        if !self.map.has(14)
      {
         // Java: return this.parent == null ? 0 : this.parent.textVerticalOverflow();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().textVerticalOverflow() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(14);
         let bin = self.map.byKey(14);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasTextVerticalOverflow()
    pub fn hasTextVerticalOverflow(&self) -> bool {
      // Java: return this.map.has(14) || this.parent != null && this.parent.hasTextVerticalOverflow();
      self.map.has(14) || self.parent.as_ref().map_or(false, |p| p.hasTextVerticalOverflow())
    }

    // Java: void deleteTextVerticalOverflow()
    pub fn deleteTextVerticalOverflow(&mut self) {
      // Java: this.map.put(14, (KoBin)null);
      self.map.clear_key(14);
    }

    // Java: void putTextVerticalOverflow(int value)
    pub fn putTextVerticalOverflow(&mut self, value: i32) {
      // Java: this.map.put(14, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(14, bin);
    }

    // Java: int anchor()
    pub fn anchor(&self) -> i32 {
      // Java: if (!this.map.has(15))
        if !self.map.has(15)
      {
         // Java: return this.parent == null ? 2 : this.parent.anchor();
         return if self.parent.is_none() { 2 } else { self.parent.as_ref().unwrap().anchor() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(15);
         let bin = self.map.byKey(15);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasAnchor()
    pub fn hasAnchor(&self) -> bool {
      // Java: return this.map.has(15) || this.parent != null && this.parent.hasAnchor();
      self.map.has(15) || self.parent.as_ref().map_or(false, |p| p.hasAnchor())
    }

    // Java: void deleteAnchor()
    pub fn deleteAnchor(&mut self) {
      // Java: this.map.put(15, (KoBin)null);
      self.map.clear_key(15);
    }

    // Java: void putAnchor(int value)
    pub fn putAnchor(&mut self, value: i32) {
      // Java: this.map.put(15, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(15, bin);
    }

    // Java: int bottomInset()
    pub fn bottomInset(&self) -> i32 {
      // Java: if (!this.map.has(16))
        if !self.map.has(16)
      {
         // Java: return this.parent == null ? '늘' : this.parent.bottomInset();
         return if self.parent.is_none() { 45720 } else { self.parent.as_ref().unwrap().bottomInset() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(16);
         let bin = self.map.byKey(16);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasBottomInset()
    pub fn hasBottomInset(&self) -> bool {
      // Java: return this.map.has(16) || this.parent != null && this.parent.hasBottomInset();
      self.map.has(16) || self.parent.as_ref().map_or(false, |p| p.hasBottomInset())
    }

    // Java: void deleteBottomInset()
    pub fn deleteBottomInset(&mut self) {
      // Java: this.map.put(16, (KoBin)null);
      self.map.clear_key(16);
    }

    // Java: void putBottomInset(int value)
    pub fn putBottomInset(&mut self, value: i32) {
      // Java: this.map.put(16, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(16, bin);
    }

    // Java: int leftInset()
    pub fn leftInset(&self) -> i32 {
      // Java: if (!this.map.has(17))
        if !self.map.has(17)
      {
         // Java: return this.parent == null ? 91440 : this.parent.leftInset();
         return if self.parent.is_none() { 91440 } else { self.parent.as_ref().unwrap().leftInset() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(17);
         let bin = self.map.byKey(17);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasLeftInset()
    pub fn hasLeftInset(&self) -> bool {
      // Java: return this.map.has(17) || this.parent != null && this.parent.hasLeftInset();
      self.map.has(17) || self.parent.as_ref().map_or(false, |p| p.hasLeftInset())
    }

    // Java: void deleteLeftInset()
    pub fn deleteLeftInset(&mut self) {
      // Java: this.map.put(17, (KoBin)null);
      self.map.clear_key(17);
    }

    // Java: void putLeftInset(int value)
    pub fn putLeftInset(&mut self, value: i32) {
      // Java: this.map.put(17, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(17, bin);
    }

    // Java: int topInset()
    pub fn topInset(&self) -> i32 {
      // Java: if (!this.map.has(18))
        if !self.map.has(18)
      {
         // Java: return this.parent == null ? '늘' : this.parent.topInset();
         return if self.parent.is_none() { 45720 } else { self.parent.as_ref().unwrap().topInset() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(18);
         let bin = self.map.byKey(18);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasTopInset()
    pub fn hasTopInset(&self) -> bool {
      // Java: return this.map.has(18) || this.parent != null && this.parent.hasTopInset();
      self.map.has(18) || self.parent.as_ref().map_or(false, |p| p.hasTopInset())
    }

    // Java: void deleteTopInset()
    pub fn deleteTopInset(&mut self) {
      // Java: this.map.put(18, (KoBin)null);
      self.map.clear_key(18);
    }

    // Java: void putTopInset(int value)
    pub fn putTopInset(&mut self, value: i32) {
      // Java: this.map.put(18, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(18, bin);
    }

    // Java: int rightInset()
    pub fn rightInset(&self) -> i32 {
      // Java: if (!this.map.has(19))
        if !self.map.has(19)
      {
         // Java: return this.parent == null ? 91440 : this.parent.rightInset();
         return if self.parent.is_none() { 91440 } else { self.parent.as_ref().unwrap().rightInset() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(19);
         let bin = self.map.byKey(19);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasRightInset()
    pub fn hasRightInset(&self) -> bool {
      // Java: return this.map.has(19) || this.parent != null && this.parent.hasRightInset();
      self.map.has(19) || self.parent.as_ref().map_or(false, |p| p.hasRightInset())
    }

    // Java: void deleteRightInset()
    pub fn deleteRightInset(&mut self) {
      // Java: this.map.put(19, (KoBin)null);
      self.map.clear_key(19);
    }

    // Java: void putRightInset(int value)
    pub fn putRightInset(&mut self, value: i32) {
      // Java: this.map.put(19, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(19, bin);
    }

    // Java: int numberOfColumns()
    pub fn numberOfColumns(&self) -> i32 {
      // Java: if (!this.map.has(20))
        if !self.map.has(20)
      {
         // Java: return this.parent == null ? 0 : this.parent.numberOfColumns();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().numberOfColumns() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(20);
         let bin = self.map.byKey(20);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasNumberOfColumns()
    pub fn hasNumberOfColumns(&self) -> bool {
      // Java: return this.map.has(20) || this.parent != null && this.parent.hasNumberOfColumns();
      self.map.has(20) || self.parent.as_ref().map_or(false, |p| p.hasNumberOfColumns())
    }

    // Java: void deleteNumberOfColumns()
    pub fn deleteNumberOfColumns(&mut self) {
      // Java: this.map.put(20, (KoBin)null);
      self.map.clear_key(20);
    }

    // Java: void putNumberOfColumns(int value)
    pub fn putNumberOfColumns(&mut self, value: i32) {
      // Java: this.map.put(20, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(20, bin);
    }

    // Java: int spaceBetweenColumns()
    pub fn spaceBetweenColumns(&self) -> i32 {
      // Java: if (!this.map.has(21))
        if !self.map.has(21)
      {
         // Java: return this.parent == null ? 0 : this.parent.spaceBetweenColumns();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().spaceBetweenColumns() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(21);
         let bin = self.map.byKey(21);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasSpaceBetweenColumns()
    pub fn hasSpaceBetweenColumns(&self) -> bool {
      // Java: return this.map.has(21) || this.parent != null && this.parent.hasSpaceBetweenColumns();
      self.map.has(21) || self.parent.as_ref().map_or(false, |p| p.hasSpaceBetweenColumns())
    }

    // Java: void deleteSpaceBetweenColumns()
    pub fn deleteSpaceBetweenColumns(&mut self) {
      // Java: this.map.put(21, (KoBin)null);
      self.map.clear_key(21);
    }

    // Java: void putSpaceBetweenColumns(int value)
    pub fn putSpaceBetweenColumns(&mut self, value: i32) {
      // Java: this.map.put(21, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(21, bin);
    }

    // Java: int rotation()
    pub fn rotation(&self) -> i32 {
      // Java: if (!this.map.has(22))
        if !self.map.has(22)
      {
         // Java: return this.parent == null ? 0 : this.parent.rotation();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().rotation() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(22);
         let bin = self.map.byKey(22);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasRotation()
    pub fn hasRotation(&self) -> bool {
      // Java: return this.map.has(22) || this.parent != null && this.parent.hasRotation();
      self.map.has(22) || self.parent.as_ref().map_or(false, |p| p.hasRotation())
    }

    // Java: void deleteRotation()
    pub fn deleteRotation(&mut self) {
      // Java: this.map.put(22, (KoBin)null);
      self.map.clear_key(22);
    }

    // Java: void putRotation(int value)
    pub fn putRotation(&mut self, value: i32) {
      // Java: this.map.put(22, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(22, bin);
    }

    // Java: int verticalText()
    pub fn verticalText(&self) -> i32 {
      // Java: if (!this.map.has(23))
        if !self.map.has(23)
      {
         // Java: return this.parent == null ? 1 : this.parent.verticalText();
         return if self.parent.is_none() { 1 } else { self.parent.as_ref().unwrap().verticalText() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(23);
         let bin = self.map.byKey(23);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasVerticalText()
    pub fn hasVerticalText(&self) -> bool {
      // Java: return this.map.has(23) || this.parent != null && this.parent.hasVerticalText();
      self.map.has(23) || self.parent.as_ref().map_or(false, |p| p.hasVerticalText())
    }

    // Java: void deleteVerticalText()
    pub fn deleteVerticalText(&mut self) {
      // Java: this.map.put(23, (KoBin)null);
      self.map.clear_key(23);
    }

    // Java: void putVerticalText(int value)
    pub fn putVerticalText(&mut self, value: i32) {
      // Java: this.map.put(23, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(23, bin);
    }

    // Java: int textWrappingType()
    pub fn textWrappingType(&self) -> i32 {
      // Java: if (!this.map.has(24))
        if !self.map.has(24)
      {
         // Java: return this.parent == null ? 0 : this.parent.textWrappingType();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().textWrappingType() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(24);
         let bin = self.map.byKey(24);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasTextWrappingType()
    pub fn hasTextWrappingType(&self) -> bool {
      // Java: return this.map.has(24) || this.parent != null && this.parent.hasTextWrappingType();
      self.map.has(24) || self.parent.as_ref().map_or(false, |p| p.hasTextWrappingType())
    }

    // Java: void deleteTextWrappingType()
    pub fn deleteTextWrappingType(&mut self) {
      // Java: this.map.put(24, (KoBin)null);
      self.map.clear_key(24);
    }

    // Java: void putTextWrappingType(int value)
    pub fn putTextWrappingType(&mut self, value: i32) {
      // Java: this.map.put(24, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(24, bin);
    }

    // Java: boolean isTextFontSizeFit()
    pub fn isTextFontSizeFit(&self) -> bool {
      // Java: if (!this.map.has(25))
        if !self.map.has(25)
      {
         // Java: return this.parent == null ? false : this.parent.isTextFontSizeFit();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isTextFontSizeFit() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(25);
         let bin = self.map.byKey(25);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasTextFontSizeFit()
    pub fn hasTextFontSizeFit(&self) -> bool {
      // Java: return this.map.has(25) || this.parent != null && this.parent.hasTextFontSizeFit();
      self.map.has(25) || self.parent.as_ref().map_or(false, |p| p.hasTextFontSizeFit())
    }

    // Java: void deleteTextFontSizeFit()
    pub fn deleteTextFontSizeFit(&mut self) {
      // Java: this.map.put(25, (KoBin)null);
      self.map.clear_key(25);
    }

    // Java: void putTextFontSizeFit(boolean value)
    pub fn putTextFontSizeFit(&mut self, value: bool) {
      // Java: this.map.put(25, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_bool(value, self.zone);
      self.map.put(25, bin);
    }

}

impl KoAr for TextBodyProperties {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextBodyProperties { fn close(&mut self) { self.close(); } }

// ============================================================
// Inner class: PresetTextWarp
// ============================================================

impl PresetTextWarp {
    // Java: static protected static <Value1 extends PresetTextWarp> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::PresetTextWarp_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static PresetTextWarp create()
    pub fn create() -> Self {
      // Java: return initialize(PresetTextWarp.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static PresetTextWarp open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(PresetTextWarp.class, bin);
      Self::initialize(bin)
    }

    // Java: PresetTextWarp()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self { bin_box: KpBinBox::new(), map: KpMap::new(), zone: KoZone::defaultZone(), parent: None, adjustValueLst: None }
    }

    // Java: PresetTextWarp(KoBin bin)
    pub fn PresetTextWarp_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), zone: KoZone::defaultZone(), parent: None, adjustValueLst: None }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
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
      self.loadBin(bin, zone);
    }

    // Java: void load(PresetTextWarp value)
    pub fn load_value(&mut self, value: Option<&PresetTextWarp>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.adjustValueLst != null && this.adjustValueLst.dirty())
        if self.adjustValueLst.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(1, this.adjustValueLst.saveBin(zone));
         let bin = self.adjustValueLst.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(PresetTextWarp o)
    pub fn equalsLax(&self, o: &PresetTextWarp) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &PresetTextWarp) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &PresetTextWarp, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof PresetTextWarp)
        if true
      {
         // Java: PresetTextWarp dst = (PresetTextWarp)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasAdjustValueLst() || !dst.hasAdjustValueLst()) && (!this.hasAdjustValueLst() || dst.hasAdjustValueLst()))
            else if (self.hasAdjustValueLst() || !dst.hasAdjustValueLst()) && (!self.hasAdjustValueLst() || dst.hasAdjustValueLst())
         {
            // Java: if (this.hasAdjustValueLst() && dst.hasAdjustValueLst())
               if self.hasAdjustValueLst() && dst.hasAdjustValueLst()
            {
               // Java: if (isStrict && !this.adjustValueLst().equals(dst.adjustValueLst()))
                  if isStrict
               {
                  // Java: return false;
               }

               // Java: if (!isStrict && !this.adjustValueLst().equalsLax(dst.adjustValueLst()))
                  if !isStrict
               {
                  // Java: return false;
               }
            }
            // Java: if ((this.hasPrst() || !dst.hasPrst()) && (!this.hasPrst() || dst.hasPrst()))
               if (self.hasPrst() || !dst.hasPrst()) && (!self.hasPrst() || dst.hasPrst())
            {
               // Java: return !this.hasPrst() || !dst.hasPrst() || this.prst() == dst.prst();
               return !self.hasPrst() || !dst.hasPrst() || self.prst() == dst.prst();
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
      // Java: return false;
      return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasAdjustValueLst())
        if self.hasAdjustValueLst()
      {
         // Java: ret += this.adjustValueLst().hashCode();
         todo!()
      }
      // Java: if (this.hasPrst())
        if self.hasPrst()
      {
         // Java: ret += Integer.valueOf(this.prst()).hashCode();
         ret = ret.wrapping_add(self.prst());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.adjustValueLst != null && this.adjustValueLst.dirty() ? true : this.map.dirty();
      self.adjustValueLst.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty()
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
      let null_bin = Null::empty();
      self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.set(None);
      // Java: this.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.adjustValueLst = null;
      self.adjustValueLst = None;
    }

    // Java: void follow(PresetTextWarp parent)
    pub fn follow(&mut self, parent: Option<&mut PresetTextWarp>) {
      // Java: if (parent != null)
        if let Some(p) = parent
      {
         // Java: parent.discard();
         let pz = p.zone;
         p.discard_with_zone(pz);
         // Java: this.discard();
         let z = self.zone;
         self.discard_with_zone(z);
         // Java: this.parent = parent;
         todo!()
      }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
        if self.parent.is_some()
      {
         // Java: this.discard();
         let z = self.zone;
         self.discard_with_zone(z);
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { let pz = p.zone; p.discard_with_zone(pz); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: PresetTextWarp parent()
    pub fn parent(&self) -> Option<&PresetTextWarp> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let z = self.zone;
      self.discard_with_zone(z);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: AdjustValues adjustValueLst()
    pub fn adjustValueLst(&mut self) -> &AdjustValues {
      // Java: if (this.adjustValueLst == null)
        if self.adjustValueLst.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            // Java: this.adjustValueLst = (AdjustValues)KoBin.Decode.toAny(AdjustValues.class, bin, this.zone);
            let bin = self.map.byKey(1);
            self.adjustValueLst = Some(Box::new(AdjustValues::AdjustValues_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasAdjustValueLst())
            else if self.parent.as_ref().map_or(false, |p| p.hasAdjustValueLst())
         {
            // Java: this.adjustValueLst = this.parent.adjustValueLst();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.adjustValueLst = (AdjustValues)KoBin.Decode.toAny(AdjustValues.class, bin, this.zone);
            let null_bin = Null::empty();
            self.adjustValueLst = Some(Box::new(AdjustValues::AdjustValues_from(null_bin.as_ref())));
         }
      }
      // Java: return this.adjustValueLst;
      self.adjustValueLst.as_deref().unwrap()
    }

    // Java: boolean hasAdjustValueLst()
    pub fn hasAdjustValueLst(&self) -> bool {
      // Java: return this.map.has(1) || this.adjustValueLst != null && this.adjustValueLst.dirty() || this.parent != null && this.parent.hasAdjustValueLst();
      self.map.has(1) || self.adjustValueLst.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasAdjustValueLst())
    }

    // Java: void deleteAdjustValueLst()
    pub fn deleteAdjustValueLst(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.adjustValueLst != null)
        if self.adjustValueLst.is_some()
      {
         // Java: this.adjustValueLst = null;
         self.adjustValueLst = None;
      }
    }

    // Java: void writeAdjustValueLst(AdjustValues value)
    pub fn writeAdjustValueLst(&mut self, value: Option<&AdjustValues>) {
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
            // Java: if (this.adjustValueLst != null)
               if self.adjustValueLst.is_some()
            {
               // Java: this.adjustValueLst = null;
               self.adjustValueLst = None;
            }
         }
      }
    }

    // Java: void forceWriteAdjustValueLst(AdjustValues value)
    pub fn forceWriteAdjustValueLst(&mut self, value: Option<&AdjustValues>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.adjustValueLst != null)
            if self.adjustValueLst.is_some()
         {
            // Java: this.adjustValueLst = null;
            self.adjustValueLst = None;
         }
      }
    }

    // Java: int prst()
    pub fn prst(&self) -> i32 {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? 0 : this.parent.prst();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().prst() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasPrst()
    pub fn hasPrst(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasPrst();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasPrst())
    }

    // Java: void deletePrst()
    pub fn deletePrst(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putPrst(int value)
    pub fn putPrst(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(2, bin);
    }

}

impl KoAr for PresetTextWarp {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for PresetTextWarp { fn close(&mut self) { self.close(); } }

// ============================================================
// Inner class: AdjustValues
// ============================================================

impl AdjustValues {
    // Java: AdjustValues()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.list = new KpList(this.zone);
      let zone = KoZone::defaultZone();
      Self { bin_box: KpBinBox::new(), list: KpList::new(zone), zone }
    }

    // Java: AdjustValues(KoBin bin)
    pub fn AdjustValues_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.list = new KpList(bin, this.zone);
      let zone = KoZone::defaultZone();
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), list: KpList::KpList_from(bin, zone), zone }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.list.load(this.box.get(), zone);
      let bin_ref = self.bin_box.get();
      self.list.load(bin_ref.as_ref(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(AdjustValues value)
    pub fn load_value(&mut self, value: Option<&AdjustValues>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
      if self.list.dirty() { let saved = self.list.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(AdjustValues o)
    pub fn equalsLax(&self, o: &AdjustValues) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &AdjustValues) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &AdjustValues, isStrict: bool) -> bool {
      // Java: if (null == o)
      // Java: return false;
      // Java: else
      {
         // Java: AdjustValues dst = (AdjustValues)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else
            else
         {
            // Java: ArrayList<GeomGuide> dstList = new ArrayList();
            let mut dstList: Vec<GeomGuide> = Vec::new();
            // Java: dst.read(dstList);
            dst.read(&mut dstList);
            // Java: ArrayList<GeomGuide> srcList = new ArrayList();
            let mut srcList: Vec<GeomGuide> = Vec::new();
            // Java: this.read(srcList);
            self.read(&mut srcList);
            // Java: if (dstList.size() != srcList.size())
               if dstList.len() != srcList.len()
            {
               // Java: return false;
               return false;
            }
            // Java: else
               else
            {
               // Java: int size = dstList.size();
               let size = dstList.len();
               // Java: for(int i = 0; i < size; ++i)
                  for i in 0..size
               {
                  // Java: GeomGuide dstClrTrans = (GeomGuide)dstList.get(i);
                  let dstClrTrans = &dstList[i];
                  // Java: GeomGuide srcClrTrans = (GeomGuide)srcList.get(i);
                  let srcClrTrans = &srcList[i];
                  // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                     if isStrict && !dstClrTrans.equals(srcClrTrans)
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                     if !isStrict && !dstClrTrans.equalsLax(srcClrTrans)
                  {
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
      let zone = self.zone;
      self.saveBin(zone);
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
      let null_bin = Null::empty();
      self.list.load(null_bin.as_ref(), self.zone);
      // Java: this.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends AdjustValues> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::AdjustValues_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static AdjustValues create()
    pub fn create() -> Self {
      // Java: return initialize(AdjustValues.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static AdjustValues open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(AdjustValues.class, bin);
      Self::initialize(bin)
    }

    // Java: void read(Collection<GeomGuide> valueLst)
    pub fn read(&self, valueLst: &mut Vec<GeomGuide>) {
      // Java: valueLst.clear();
      valueLst.clear();
      // Java: int size = this.list.size();
      let size = self.list.size();
      // Java: for(int i = 0; i < size; ++i)
        for i in 0..size
      {
         // Java: valueLst.add((GeomGuide)KoBin.Decode.toAny(GeomGuide.class, this.list.get(i), this.zone));
         todo!()
      }
    }

    // Java: void write(Iterable<GeomGuide> valueLst)
    pub fn write(&mut self, valueLst: &[GeomGuide]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(GeomGuide i : valueLst)
        for i in valueLst
      {
         // Java: KoBin bin = i.lastBin(this.zone);
         let bin = i.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.list.add(bin);
            self.list.add(bin);
         }
      }
    }

    // Java: void forceWrite(Iterable<GeomGuide> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[GeomGuide]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(GeomGuide i : valueLst)
        for i in valueLst
      {
         // Java: this.list.add(i.lastBin(this.zone));
         self.list.add(i.lastBin(self.zone));
      }
    }
}

impl KoAr for AdjustValues {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for AdjustValues { fn close(&mut self) { self.close(); } }

// ============================================================
// Inner class: Autofit
// ============================================================

impl Autofit {
    // Java: static protected static <Value1 extends Autofit> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::Autofit_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static Autofit create()
    pub fn create() -> Self {
      // Java: return initialize(Autofit.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Autofit open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Autofit.class, bin);
      Self::initialize(bin)
    }

    // Java: Autofit()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self { bin_box: KpBinBox::new(), map: KpMap::new(), zone: KoZone::defaultZone(), parent: None }
    }

    // Java: Autofit(KoBin bin)
    pub fn Autofit_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), zone: KoZone::defaultZone(), parent: None }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
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
      self.loadBin(bin, zone);
    }

    // Java: void load(Autofit value)
    pub fn load_value(&mut self, value: Option<&Autofit>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(Autofit o)
    pub fn equalsLax(&self, o: &Autofit) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Autofit) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &Autofit, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Autofit)
        if true
      {
         // Java: Autofit dst = (Autofit)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
            else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType())
         {
            // Java: if (this.hasType() && dst.hasType() && this.type() != dst.type())
                if self.hasType() && dst.hasType() && self.r#type() != dst.r#type()
            {
               // Java: return false;
               return false;
            }
            // Java: else
                else
            {
               // Java: if (1 != this.type())
                  if 1 != self.r#type()
               {
                  // Java: if (!this.hasFontScale() && dst.hasFontScale() || this.hasFontScale() && !dst.hasFontScale())
                     if !self.hasFontScale() && dst.hasFontScale() || self.hasFontScale() && !dst.hasFontScale()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (this.hasFontScale() && dst.hasFontScale() && this.fontScale() != dst.fontScale())
                     if self.hasFontScale() && dst.hasFontScale() && self.fontScale() != dst.fontScale()
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasLnSpcReduction() || !dst.hasLnSpcReduction()) && (!this.hasLnSpcReduction() || dst.hasLnSpcReduction()))
                  if (self.hasLnSpcReduction() || !dst.hasLnSpcReduction()) && (!self.hasLnSpcReduction() || dst.hasLnSpcReduction())
               {
                  // Java: if (this.hasLnSpcReduction() && dst.hasLnSpcReduction() && this.lnSpcReduction() != dst.lnSpcReduction())
                     if self.hasLnSpcReduction() && dst.hasLnSpcReduction() && self.lnSpcReduction() != dst.lnSpcReduction()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else
                     else
                  {
                     // Java: return true;
                     return true;
                  }
               }
               // Java: else
                  else
               {
                  // Java: return false;
                  return false;
               }
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
      // Java: return false;
      return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasType())
        if self.hasType()
      {
         // Java: ret += Integer.valueOf(this.type()).hashCode();
         ret = ret.wrapping_add(self.r#type());
      }
      // Java: if (this.hasFontScale())
        if self.hasFontScale()
      {
         // Java: ret += Double.valueOf(this.fontScale()).hashCode();
         ret = ret.wrapping_add((self.fontScale().to_bits() as i64 ^ ((self.fontScale().to_bits() as i64) >> 32)) as i32);
      }
      // Java: if (this.hasLnSpcReduction())
        if self.hasLnSpcReduction()
      {
         // Java: ret += Double.valueOf(this.lnSpcReduction()).hashCode();
         ret = ret.wrapping_add((self.lnSpcReduction().to_bits() as i64 ^ ((self.lnSpcReduction().to_bits() as i64) >> 32)) as i32);
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
      let null_bin = Null::empty();
      self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.set(None);
      // Java: this.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    fn cleanCache(&mut self) {}

    // Java: void follow(Autofit parent)
    pub fn follow(&mut self, parent: Option<&mut Autofit>) {
      // Java: if (parent != null)
        if let Some(p) = parent
      {
         // Java: parent.discard();
         let pz = p.zone;
         p.discard_with_zone(pz);
         // Java: this.discard();
         let z = self.zone;
         self.discard_with_zone(z);
         // Java: this.parent = parent;
         todo!()
      }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
        if self.parent.is_some()
      {
         // Java: this.discard();
         let z = self.zone;
         self.discard_with_zone(z);
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { let pz = p.zone; p.discard_with_zone(pz); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: Autofit parent()
    pub fn parent(&self) -> Option<&Autofit> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let z = self.zone;
      self.discard_with_zone(z);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 0 : this.parent.type();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().r#type() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasType();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(1, bin);
    }

    // Java: double fontScale()
    pub fn fontScale(&self) -> f64 {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.fontScale();
         return if self.parent.is_none() { 0.0 } else { self.parent.as_ref().unwrap().fontScale() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasFontScale()
    pub fn hasFontScale(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasFontScale();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasFontScale())
    }

    // Java: void deleteFontScale()
    pub fn deleteFontScale(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putFontScale(double value)
    pub fn putFontScale(&mut self, value: f64) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_double(value, self.zone);
      self.map.put(2, bin);
    }

    // Java: double lnSpcReduction()
    pub fn lnSpcReduction(&self) -> f64 {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.lnSpcReduction();
         return if self.parent.is_none() { 0.0 } else { self.parent.as_ref().unwrap().lnSpcReduction() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasLnSpcReduction()
    pub fn hasLnSpcReduction(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasLnSpcReduction();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasLnSpcReduction())
    }

    // Java: void deleteLnSpcReduction()
    pub fn deleteLnSpcReduction(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }

    // Java: void putLnSpcReduction(double value)
    pub fn putLnSpcReduction(&mut self, value: f64) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_double(value, self.zone);
      self.map.put(3, bin);
    }

}

impl KoAr for Autofit {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for Autofit { fn close(&mut self) { self.close(); } }
