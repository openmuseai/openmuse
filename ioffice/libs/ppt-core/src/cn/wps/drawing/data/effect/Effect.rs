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
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;

// Java: public class Effect implements KoAr, KoClosable
pub struct Effect {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: EffectContainer effectDag
    pub(crate) effectDag: Option<Box<dyn KoBinTrait>>,
    // Java field: EffectReference effect
    pub(crate) effect: Option<Box<dyn KoBinTrait>>,
    // Java field: AlphaBiLevelEffect alphaBiLevel
    pub(crate) alphaBiLevel: Option<Box<dyn KoBinTrait>>,
    // Java field: AlphaInverseEffect alphaInverse
    pub(crate) alphaInverse: Option<Box<dyn KoBinTrait>>,
    // Java field: AlphaModEffect alphaMod
    pub(crate) alphaMod: Option<Box<dyn KoBinTrait>>,
    // Java field: AlphaModFixEffect alphaModFix
    pub(crate) alphaModFix: Option<Box<dyn KoBinTrait>>,
    // Java field: AlphaOutsetEffect alphaOutset
    pub(crate) alphaOutset: Option<Box<dyn KoBinTrait>>,
    // Java field: AlphaReplaceEffect alphaReplacement
    pub(crate) alphaReplacement: Option<Box<dyn KoBinTrait>>,
    // Java field: BiLevelEffect biLevel
    pub(crate) biLevel: Option<Box<dyn KoBinTrait>>,
    // Java field: BlendEffect blend
    pub(crate) blend: Option<Box<dyn KoBinTrait>>,
    // Java field: BlurEffect blur
    pub(crate) blur: Option<Box<dyn KoBinTrait>>,
    // Java field: ColorChangeEffect clrChange
    pub(crate) clrChange: Option<Box<dyn KoBinTrait>>,
    // Java field: ColorReplacementEffect clrReplacement
    pub(crate) clrReplacement: Option<Box<dyn KoBinTrait>>,
    // Java field: DuotoneEffect duotone
    pub(crate) duotone: Option<Box<dyn KoBinTrait>>,
    // Java field: FillEffect fill
    pub(crate) fill: Option<Box<dyn KoBinTrait>>,
    // Java field: FillOverlayEffect fillOverlay
    pub(crate) fillOverlay: Option<Box<dyn KoBinTrait>>,
    // Java field: GlowEffect glow
    pub(crate) glow: Option<Box<dyn KoBinTrait>>,
    // Java field: HSLEffect hsl
    pub(crate) hsl: Option<Box<dyn KoBinTrait>>,
    // Java field: InnerShadowEffect innerShadow
    pub(crate) innerShadow: Option<Box<dyn KoBinTrait>>,
    // Java field: LumEffect lum
    pub(crate) lum: Option<Box<dyn KoBinTrait>>,
    // Java field: OuterShadowEffect outerShadow
    pub(crate) outerShadow: Option<Box<dyn KoBinTrait>>,
    // Java field: PresetShadowEffect presetShadow
    pub(crate) presetShadow: Option<Box<dyn KoBinTrait>>,
    // Java field: ReflectionEffect reflection
    pub(crate) reflection: Option<Box<dyn KoBinTrait>>,
    // Java field: RelOffEffect relativeOffset
    pub(crate) relativeOffset: Option<Box<dyn KoBinTrait>>,
    // Java field: SoftEdgeEffect softEdge
    pub(crate) softEdge: Option<Box<dyn KoBinTrait>>,
    // Java field: TintEffect tint
    pub(crate) tint: Option<Box<dyn KoBinTrait>>,
    // Java field: XfrmEffect xfrm
    pub(crate) xfrm: Option<Box<dyn KoBinTrait>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Effect parent
    pub(crate) parent: Option<Box<Effect>>,
}

// Java: public class Effect implements KoAr, KoClosable
impl Effect {
    // Java: static protected static <Value1 extends Effect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Effect::Effect();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static Effect create()
    pub fn create() -> Self {
        // Java: return initialize(Effect.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static Effect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(Effect.class, bin);
        Self::initialize(bin)
    }

    // Java: Effect()
    pub fn Effect() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        Effect { zone: KoZone, effectDag: None, effect: None, alphaBiLevel: None, alphaInverse: None, alphaMod: None, alphaModFix: None, alphaOutset: None, alphaReplacement: None, biLevel: None, blend: None, blur: None, clrChange: None, clrReplacement: None, duotone: None, fill: None, fillOverlay: None, glow: None, hsl: None, innerShadow: None, lum: None, outerShadow: None, presetShadow: None, reflection: None, relativeOffset: None, softEdge: None, tint: None, xfrm: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: Effect(KoBin bin)
    pub fn Effect_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        Effect { zone: KoZone, effectDag: None, effect: None, alphaBiLevel: None, alphaInverse: None, alphaMod: None, alphaModFix: None, alphaOutset: None, alphaReplacement: None, biLevel: None, blend: None, blur: None, clrChange: None, clrReplacement: None, duotone: None, fill: None, fillOverlay: None, glow: None, hsl: None, innerShadow: None, lum: None, outerShadow: None, presetShadow: None, reflection: None, relativeOffset: None, softEdge: None, tint: None, xfrm: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(Effect value)
    pub fn load_from(&mut self, value: Option<&Effect>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.effectDag != null && this.effectDag.dirty())
        // Java: this.map.put(2, this.effectDag.saveBin(zone));
        // Java: if (this.effect != null && this.effect.dirty())
        // Java: this.map.put(3, this.effect.saveBin(zone));
        // Java: if (this.alphaBiLevel != null && this.alphaBiLevel.dirty())
        // Java: this.map.put(4, this.alphaBiLevel.saveBin(zone));
        // Java: if (this.alphaInverse != null && this.alphaInverse.dirty())
        // Java: this.map.put(5, this.alphaInverse.saveBin(zone));
        // Java: if (this.alphaMod != null && this.alphaMod.dirty())
        // Java: this.map.put(6, this.alphaMod.saveBin(zone));
        // Java: if (this.alphaModFix != null && this.alphaModFix.dirty())
        // Java: this.map.put(7, this.alphaModFix.saveBin(zone));
        // Java: if (this.alphaOutset != null && this.alphaOutset.dirty())
        // Java: this.map.put(8, this.alphaOutset.saveBin(zone));
        // Java: if (this.alphaReplacement != null && this.alphaReplacement.dirty())
        // Java: this.map.put(9, this.alphaReplacement.saveBin(zone));
        // Java: if (this.biLevel != null && this.biLevel.dirty())
        // Java: this.map.put(10, this.biLevel.saveBin(zone));
        // Java: if (this.blend != null && this.blend.dirty())
        // Java: this.map.put(11, this.blend.saveBin(zone));
        // Java: if (this.blur != null && this.blur.dirty())
        // Java: this.map.put(12, this.blur.saveBin(zone));
        // Java: if (this.clrChange != null && this.clrChange.dirty())
        // Java: this.map.put(13, this.clrChange.saveBin(zone));
        // Java: if (this.clrReplacement != null && this.clrReplacement.dirty())
        // Java: this.map.put(14, this.clrReplacement.saveBin(zone));
        // Java: if (this.duotone != null && this.duotone.dirty())
        // Java: this.map.put(15, this.duotone.saveBin(zone));
        // Java: if (this.fill != null && this.fill.dirty())
        // Java: this.map.put(16, this.fill.saveBin(zone));
        // Java: if (this.fillOverlay != null && this.fillOverlay.dirty())
        // Java: this.map.put(17, this.fillOverlay.saveBin(zone));
        // Java: if (this.glow != null && this.glow.dirty())
        // Java: this.map.put(18, this.glow.saveBin(zone));
        // Java: if (this.hsl != null && this.hsl.dirty())
        // Java: this.map.put(19, this.hsl.saveBin(zone));
        // Java: if (this.innerShadow != null && this.innerShadow.dirty())
        // Java: this.map.put(20, this.innerShadow.saveBin(zone));
        // Java: if (this.lum != null && this.lum.dirty())
        // Java: this.map.put(21, this.lum.saveBin(zone));
        // Java: if (this.outerShadow != null && this.outerShadow.dirty())
        // Java: this.map.put(22, this.outerShadow.saveBin(zone));
        // Java: if (this.presetShadow != null && this.presetShadow.dirty())
        // Java: this.map.put(23, this.presetShadow.saveBin(zone));
        // Java: if (this.reflection != null && this.reflection.dirty())
        // Java: this.map.put(24, this.reflection.saveBin(zone));
        // Java: if (this.relativeOffset != null && this.relativeOffset.dirty())
        // Java: this.map.put(25, this.relativeOffset.saveBin(zone));
        // Java: if (this.softEdge != null && this.softEdge.dirty())
        // Java: this.map.put(26, this.softEdge.saveBin(zone));
        // Java: if (this.tint != null && this.tint.dirty())
        // Java: this.map.put(27, this.tint.saveBin(zone));
        // Java: if (this.xfrm != null && this.xfrm.dirty())
        // Java: this.map.put(28, this.xfrm.saveBin(zone));
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(Effect o)
    pub fn equalsLax(&self, o: &Effect) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Effect) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &Effect, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof Effect)
        if true {
        // Java: Effect dst = (Effect)o;
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
            // Java: else if ((this.hasEffectDag() || !dst.hasEffectDag()) && (!this.hasEffectDag() || dst.hasEffectDag()))
            else if (self.hasEffectDag() || !dst.hasEffectDag()) && (!self.hasEffectDag() || dst.hasEffectDag()) {
                // Java: if (this.hasEffectDag() && dst.hasEffectDag())
                if self.hasEffectDag() && dst.hasEffectDag() {
                    // Java: if (isStrict && !this.effectDag().equals(dst.effectDag()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.effectDag().equalsLax(dst.effectDag()))
                    if !isStrict {
                        // Java: return false;
                    }
                }
                // Java: if ((this.hasEffect() || !dst.hasEffect()) && (!this.hasEffect() || dst.hasEffect()))
                if (self.hasEffect() || !dst.hasEffect()) && (!self.hasEffect() || dst.hasEffect()) {
                    // Java: if (this.hasEffect() && dst.hasEffect())
                    if self.hasEffect() && dst.hasEffect() {
                        // Java: if (isStrict && !this.effect().equals(dst.effect()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.effect().equalsLax(dst.effect()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasAlphaBiLevel() || !dst.hasAlphaBiLevel()) && (!this.hasAlphaBiLevel() || dst.hasAlphaBiLevel()))
                if (self.hasAlphaBiLevel() || !dst.hasAlphaBiLevel()) && (!self.hasAlphaBiLevel() || dst.hasAlphaBiLevel()) {
                    // Java: if (this.hasAlphaBiLevel() && dst.hasAlphaBiLevel())
                    if self.hasAlphaBiLevel() && dst.hasAlphaBiLevel() {
                        // Java: if (isStrict && !this.alphaBiLevel().equals(dst.alphaBiLevel()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.alphaBiLevel().equalsLax(dst.alphaBiLevel()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasAlphaInverse() || !dst.hasAlphaInverse()) && (!this.hasAlphaInverse() || dst.hasAlphaInverse()))
                if (self.hasAlphaInverse() || !dst.hasAlphaInverse()) && (!self.hasAlphaInverse() || dst.hasAlphaInverse()) {
                    // Java: if (this.hasAlphaInverse() && dst.hasAlphaInverse())
                    if self.hasAlphaInverse() && dst.hasAlphaInverse() {
                        // Java: if (isStrict && !this.alphaInverse().equals(dst.alphaInverse()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.alphaInverse().equalsLax(dst.alphaInverse()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasAlphaMod() || !dst.hasAlphaMod()) && (!this.hasAlphaMod() || dst.hasAlphaMod()))
                if (self.hasAlphaMod() || !dst.hasAlphaMod()) && (!self.hasAlphaMod() || dst.hasAlphaMod()) {
                    // Java: if (this.hasAlphaMod() && dst.hasAlphaMod())
                    if self.hasAlphaMod() && dst.hasAlphaMod() {
                        // Java: if (isStrict && !this.alphaMod().equals(dst.alphaMod()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.alphaMod().equalsLax(dst.alphaMod()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasAlphaModFix() || !dst.hasAlphaModFix()) && (!this.hasAlphaModFix() || dst.hasAlphaModFix()))
                if (self.hasAlphaModFix() || !dst.hasAlphaModFix()) && (!self.hasAlphaModFix() || dst.hasAlphaModFix()) {
                    // Java: if (this.hasAlphaModFix() && dst.hasAlphaModFix())
                    if self.hasAlphaModFix() && dst.hasAlphaModFix() {
                        // Java: if (isStrict && !this.alphaModFix().equals(dst.alphaModFix()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.alphaModFix().equalsLax(dst.alphaModFix()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasAlphaOutset() || !dst.hasAlphaOutset()) && (!this.hasAlphaOutset() || dst.hasAlphaOutset()))
                if (self.hasAlphaOutset() || !dst.hasAlphaOutset()) && (!self.hasAlphaOutset() || dst.hasAlphaOutset()) {
                    // Java: if (this.hasAlphaOutset() && dst.hasAlphaOutset())
                    if self.hasAlphaOutset() && dst.hasAlphaOutset() {
                        // Java: if (isStrict && !this.alphaOutset().equals(dst.alphaOutset()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.alphaOutset().equalsLax(dst.alphaOutset()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasAlphaReplacement() || !dst.hasAlphaReplacement()) && (!this.hasAlphaReplacement() || dst.hasAlphaReplacement()))
                if (self.hasAlphaReplacement() || !dst.hasAlphaReplacement()) && (!self.hasAlphaReplacement() || dst.hasAlphaReplacement()) {
                    // Java: if (this.hasAlphaReplacement() && dst.hasAlphaReplacement())
                    if self.hasAlphaReplacement() && dst.hasAlphaReplacement() {
                        // Java: if (isStrict && !this.alphaReplacement().equals(dst.alphaReplacement()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.alphaReplacement().equalsLax(dst.alphaReplacement()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasBiLevel() || !dst.hasBiLevel()) && (!this.hasBiLevel() || dst.hasBiLevel()))
                if (self.hasBiLevel() || !dst.hasBiLevel()) && (!self.hasBiLevel() || dst.hasBiLevel()) {
                    // Java: if (this.hasBiLevel() && dst.hasBiLevel())
                    if self.hasBiLevel() && dst.hasBiLevel() {
                        // Java: if (isStrict && !this.biLevel().equals(dst.biLevel()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.biLevel().equalsLax(dst.biLevel()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasBlend() || !dst.hasBlend()) && (!this.hasBlend() || dst.hasBlend()))
                if (self.hasBlend() || !dst.hasBlend()) && (!self.hasBlend() || dst.hasBlend()) {
                    // Java: if (this.hasBlend() && dst.hasBlend())
                    if self.hasBlend() && dst.hasBlend() {
                        // Java: if (isStrict && !this.blend().equals(dst.blend()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.blend().equalsLax(dst.blend()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasBlur() || !dst.hasBlur()) && (!this.hasBlur() || dst.hasBlur()))
                if (self.hasBlur() || !dst.hasBlur()) && (!self.hasBlur() || dst.hasBlur()) {
                    // Java: if (this.hasBlur() && dst.hasBlur())
                    if self.hasBlur() && dst.hasBlur() {
                        // Java: if (isStrict && !this.blur().equals(dst.blur()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.blur().equalsLax(dst.blur()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasClrChange() || !dst.hasClrChange()) && (!this.hasClrChange() || dst.hasClrChange()))
                if (self.hasClrChange() || !dst.hasClrChange()) && (!self.hasClrChange() || dst.hasClrChange()) {
                    // Java: if (this.hasClrChange() && dst.hasClrChange())
                    if self.hasClrChange() && dst.hasClrChange() {
                        // Java: if (isStrict && !this.clrChange().equals(dst.clrChange()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.clrChange().equalsLax(dst.clrChange()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasClrReplacement() || !dst.hasClrReplacement()) && (!this.hasClrReplacement() || dst.hasClrReplacement()))
                if (self.hasClrReplacement() || !dst.hasClrReplacement()) && (!self.hasClrReplacement() || dst.hasClrReplacement()) {
                    // Java: if (this.hasClrReplacement() && dst.hasClrReplacement())
                    if self.hasClrReplacement() && dst.hasClrReplacement() {
                        // Java: if (isStrict && !this.clrReplacement().equals(dst.clrReplacement()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.clrReplacement().equalsLax(dst.clrReplacement()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasDuotone() || !dst.hasDuotone()) && (!this.hasDuotone() || dst.hasDuotone()))
                if (self.hasDuotone() || !dst.hasDuotone()) && (!self.hasDuotone() || dst.hasDuotone()) {
                    // Java: if (this.hasDuotone() && dst.hasDuotone())
                    if self.hasDuotone() && dst.hasDuotone() {
                        // Java: if (isStrict && !this.duotone().equals(dst.duotone()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.duotone().equalsLax(dst.duotone()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasFill() || !dst.hasFill()) && (!this.hasFill() || dst.hasFill()))
                if (self.hasFill() || !dst.hasFill()) && (!self.hasFill() || dst.hasFill()) {
                    // Java: if (this.hasFill() && dst.hasFill())
                    if self.hasFill() && dst.hasFill() {
                        // Java: if (isStrict && !this.fill().equals(dst.fill()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.fill().equalsLax(dst.fill()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasFillOverlay() || !dst.hasFillOverlay()) && (!this.hasFillOverlay() || dst.hasFillOverlay()))
                if (self.hasFillOverlay() || !dst.hasFillOverlay()) && (!self.hasFillOverlay() || dst.hasFillOverlay()) {
                    // Java: if (this.hasFillOverlay() && dst.hasFillOverlay())
                    if self.hasFillOverlay() && dst.hasFillOverlay() {
                        // Java: if (isStrict && !this.fillOverlay().equals(dst.fillOverlay()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.fillOverlay().equalsLax(dst.fillOverlay()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasGlow() || !dst.hasGlow()) && (!this.hasGlow() || dst.hasGlow()))
                if (self.hasGlow() || !dst.hasGlow()) && (!self.hasGlow() || dst.hasGlow()) {
                    // Java: if (this.hasGlow() && dst.hasGlow())
                    if self.hasGlow() && dst.hasGlow() {
                        // Java: if (isStrict && !this.glow().equals(dst.glow()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.glow().equalsLax(dst.glow()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasHsl() || !dst.hasHsl()) && (!this.hasHsl() || dst.hasHsl()))
                if (self.hasHsl() || !dst.hasHsl()) && (!self.hasHsl() || dst.hasHsl()) {
                    // Java: if (this.hasHsl() && dst.hasHsl())
                    if self.hasHsl() && dst.hasHsl() {
                        // Java: if (isStrict && !this.hsl().equals(dst.hsl()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.hsl().equalsLax(dst.hsl()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasInnerShadow() || !dst.hasInnerShadow()) && (!this.hasInnerShadow() || dst.hasInnerShadow()))
                if (self.hasInnerShadow() || !dst.hasInnerShadow()) && (!self.hasInnerShadow() || dst.hasInnerShadow()) {
                    // Java: if (this.hasInnerShadow() && dst.hasInnerShadow())
                    if self.hasInnerShadow() && dst.hasInnerShadow() {
                        // Java: if (isStrict && !this.innerShadow().equals(dst.innerShadow()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.innerShadow().equalsLax(dst.innerShadow()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasLum() || !dst.hasLum()) && (!this.hasLum() || dst.hasLum()))
                if (self.hasLum() || !dst.hasLum()) && (!self.hasLum() || dst.hasLum()) {
                    // Java: if (this.hasLum() && dst.hasLum())
                    if self.hasLum() && dst.hasLum() {
                        // Java: if (isStrict && !this.lum().equals(dst.lum()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.lum().equalsLax(dst.lum()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasOuterShadow() || !dst.hasOuterShadow()) && (!this.hasOuterShadow() || dst.hasOuterShadow()))
                if (self.hasOuterShadow() || !dst.hasOuterShadow()) && (!self.hasOuterShadow() || dst.hasOuterShadow()) {
                    // Java: if (this.hasOuterShadow() && dst.hasOuterShadow())
                    if self.hasOuterShadow() && dst.hasOuterShadow() {
                        // Java: if (isStrict && !this.outerShadow().equals(dst.outerShadow()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.outerShadow().equalsLax(dst.outerShadow()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasPresetShadow() || !dst.hasPresetShadow()) && (!this.hasPresetShadow() || dst.hasPresetShadow()))
                if (self.hasPresetShadow() || !dst.hasPresetShadow()) && (!self.hasPresetShadow() || dst.hasPresetShadow()) {
                    // Java: if (this.hasPresetShadow() && dst.hasPresetShadow())
                    if self.hasPresetShadow() && dst.hasPresetShadow() {
                        // Java: if (isStrict && !this.presetShadow().equals(dst.presetShadow()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.presetShadow().equalsLax(dst.presetShadow()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasReflection() || !dst.hasReflection()) && (!this.hasReflection() || dst.hasReflection()))
                if (self.hasReflection() || !dst.hasReflection()) && (!self.hasReflection() || dst.hasReflection()) {
                    // Java: if (this.hasReflection() && dst.hasReflection())
                    if self.hasReflection() && dst.hasReflection() {
                        // Java: if (isStrict && !this.reflection().equals(dst.reflection()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.reflection().equalsLax(dst.reflection()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasRelativeOffset() || !dst.hasRelativeOffset()) && (!this.hasRelativeOffset() || dst.hasRelativeOffset()))
                if (self.hasRelativeOffset() || !dst.hasRelativeOffset()) && (!self.hasRelativeOffset() || dst.hasRelativeOffset()) {
                    // Java: if (this.hasRelativeOffset() && dst.hasRelativeOffset())
                    if self.hasRelativeOffset() && dst.hasRelativeOffset() {
                        // Java: if (isStrict && !this.relativeOffset().equals(dst.relativeOffset()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.relativeOffset().equalsLax(dst.relativeOffset()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasSoftEdge() || !dst.hasSoftEdge()) && (!this.hasSoftEdge() || dst.hasSoftEdge()))
                if (self.hasSoftEdge() || !dst.hasSoftEdge()) && (!self.hasSoftEdge() || dst.hasSoftEdge()) {
                    // Java: if (this.hasSoftEdge() && dst.hasSoftEdge())
                    if self.hasSoftEdge() && dst.hasSoftEdge() {
                        // Java: if (isStrict && !this.softEdge().equals(dst.softEdge()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.softEdge().equalsLax(dst.softEdge()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasTint() || !dst.hasTint()) && (!this.hasTint() || dst.hasTint()))
                if (self.hasTint() || !dst.hasTint()) && (!self.hasTint() || dst.hasTint()) {
                    // Java: if (this.hasTint() && dst.hasTint())
                    if self.hasTint() && dst.hasTint() {
                        // Java: if (isStrict && !this.tint().equals(dst.tint()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.tint().equalsLax(dst.tint()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                // Java: if ((this.hasXfrm() || !dst.hasXfrm()) && (!this.hasXfrm() || dst.hasXfrm()))
                if (self.hasXfrm() || !dst.hasXfrm()) && (!self.hasXfrm() || dst.hasXfrm()) {
                    // Java: if (this.hasXfrm() && dst.hasXfrm())
                    if self.hasXfrm() && dst.hasXfrm() {
                        // Java: if (isStrict && !this.xfrm().equals(dst.xfrm()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.xfrm().equalsLax(dst.xfrm()))
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
            ret = ret.wrapping_add(self.mode());
        }
        // Java: if (this.hasEffectDag())
        if self.hasEffectDag() {
            // Java: ret += this.effectDag().hashCode();
        }
        // Java: if (this.hasEffect())
        if self.hasEffect() {
            // Java: ret += this.effect().hashCode();
        }
        // Java: if (this.hasAlphaBiLevel())
        if self.hasAlphaBiLevel() {
            // Java: ret += this.alphaBiLevel().hashCode();
        }
        // Java: if (this.hasAlphaInverse())
        if self.hasAlphaInverse() {
            // Java: ret += this.alphaInverse().hashCode();
        }
        // Java: if (this.hasAlphaMod())
        if self.hasAlphaMod() {
            // Java: ret += this.alphaMod().hashCode();
        }
        // Java: if (this.hasAlphaModFix())
        if self.hasAlphaModFix() {
            // Java: ret += this.alphaModFix().hashCode();
        }
        // Java: if (this.hasAlphaOutset())
        if self.hasAlphaOutset() {
            // Java: ret += this.alphaOutset().hashCode();
        }
        // Java: if (this.hasAlphaReplacement())
        if self.hasAlphaReplacement() {
            // Java: ret += this.alphaReplacement().hashCode();
        }
        // Java: if (this.hasBiLevel())
        if self.hasBiLevel() {
            // Java: ret += this.biLevel().hashCode();
        }
        // Java: if (this.hasBlend())
        if self.hasBlend() {
            // Java: ret += this.blend().hashCode();
        }
        // Java: if (this.hasBlur())
        if self.hasBlur() {
            // Java: ret += this.blur().hashCode();
        }
        // Java: if (this.hasClrChange())
        if self.hasClrChange() {
            // Java: ret += this.clrChange().hashCode();
        }
        // Java: if (this.hasClrReplacement())
        if self.hasClrReplacement() {
            // Java: ret += this.clrReplacement().hashCode();
        }
        // Java: if (this.hasDuotone())
        if self.hasDuotone() {
            // Java: ret += this.duotone().hashCode();
        }
        // Java: if (this.hasFill())
        if self.hasFill() {
            // Java: ret += this.fill().hashCode();
        }
        // Java: if (this.hasFillOverlay())
        if self.hasFillOverlay() {
            // Java: ret += this.fillOverlay().hashCode();
        }
        // Java: if (this.hasGlow())
        if self.hasGlow() {
            // Java: ret += this.glow().hashCode();
        }
        // Java: if (this.hasHsl())
        if self.hasHsl() {
            // Java: ret += this.hsl().hashCode();
        }
        // Java: if (this.hasInnerShadow())
        if self.hasInnerShadow() {
            // Java: ret += this.innerShadow().hashCode();
        }
        // Java: if (this.hasLum())
        if self.hasLum() {
            // Java: ret += this.lum().hashCode();
        }
        // Java: if (this.hasOuterShadow())
        if self.hasOuterShadow() {
            // Java: ret += this.outerShadow().hashCode();
        }
        // Java: if (this.hasPresetShadow())
        if self.hasPresetShadow() {
            // Java: ret += this.presetShadow().hashCode();
        }
        // Java: if (this.hasReflection())
        if self.hasReflection() {
            // Java: ret += this.reflection().hashCode();
        }
        // Java: if (this.hasRelativeOffset())
        if self.hasRelativeOffset() {
            // Java: ret += this.relativeOffset().hashCode();
        }
        // Java: if (this.hasSoftEdge())
        if self.hasSoftEdge() {
            // Java: ret += this.softEdge().hashCode();
        }
        // Java: if (this.hasTint())
        if self.hasTint() {
            // Java: ret += this.tint().hashCode();
        }
        // Java: if (this.hasXfrm())
        if self.hasXfrm() {
            // Java: ret += this.xfrm().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.effectDag != null && this.effectDag.dirty())
        // Java: return true;
        // Java: else if (this.effect != null && this.effect.dirty())
        // Java: return true;
        // Java: else if (this.alphaBiLevel != null && this.alphaBiLevel.dirty())
        // Java: return true;
        // Java: else if (this.alphaInverse != null && this.alphaInverse.dirty())
        // Java: return true;
        // Java: else if (this.alphaMod != null && this.alphaMod.dirty())
        // Java: return true;
        // Java: else if (this.alphaModFix != null && this.alphaModFix.dirty())
        // Java: return true;
        // Java: else if (this.alphaOutset != null && this.alphaOutset.dirty())
        // Java: return true;
        // Java: else if (this.alphaReplacement != null && this.alphaReplacement.dirty())
        // Java: return true;
        // Java: else if (this.biLevel != null && this.biLevel.dirty())
        // Java: return true;
        // Java: else if (this.blend != null && this.blend.dirty())
        // Java: return true;
        // Java: else if (this.blur != null && this.blur.dirty())
        // Java: return true;
        // Java: else if (this.clrChange != null && this.clrChange.dirty())
        // Java: return true;
        // Java: else if (this.clrReplacement != null && this.clrReplacement.dirty())
        // Java: return true;
        // Java: else if (this.duotone != null && this.duotone.dirty())
        // Java: return true;
        // Java: else if (this.fill != null && this.fill.dirty())
        // Java: return true;
        // Java: else if (this.fillOverlay != null && this.fillOverlay.dirty())
        // Java: return true;
        // Java: else if (this.glow != null && this.glow.dirty())
        // Java: return true;
        // Java: else if (this.hsl != null && this.hsl.dirty())
        // Java: return true;
        // Java: else if (this.innerShadow != null && this.innerShadow.dirty())
        // Java: return true;
        // Java: else if (this.lum != null && this.lum.dirty())
        // Java: return true;
        // Java: else if (this.outerShadow != null && this.outerShadow.dirty())
        // Java: return true;
        // Java: else if (this.presetShadow != null && this.presetShadow.dirty())
        // Java: return true;
        // Java: else if (this.reflection != null && this.reflection.dirty())
        // Java: return true;
        // Java: else if (this.relativeOffset != null && this.relativeOffset.dirty())
        // Java: return true;
        // Java: else if (this.softEdge != null && this.softEdge.dirty())
        // Java: return true;
        // Java: else if (this.tint != null && this.tint.dirty())
        // Java: return true;
        // Java: else
        // Java: return this.xfrm != null && this.xfrm.dirty() ? true : this.map.dirty();
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
    fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
        // Java: this.effectDag = null;
        self.effectDag = None;
        // Java: this.effect = null;
        self.effect = None;
        // Java: this.alphaBiLevel = null;
        self.alphaBiLevel = None;
        // Java: this.alphaInverse = null;
        self.alphaInverse = None;
        // Java: this.alphaMod = null;
        self.alphaMod = None;
        // Java: this.alphaModFix = null;
        self.alphaModFix = None;
        // Java: this.alphaOutset = null;
        self.alphaOutset = None;
        // Java: this.alphaReplacement = null;
        self.alphaReplacement = None;
        // Java: this.biLevel = null;
        self.biLevel = None;
        // Java: this.blend = null;
        self.blend = None;
        // Java: this.blur = null;
        self.blur = None;
        // Java: this.clrChange = null;
        self.clrChange = None;
        // Java: this.clrReplacement = null;
        self.clrReplacement = None;
        // Java: this.duotone = null;
        self.duotone = None;
        // Java: this.fill = null;
        self.fill = None;
        // Java: this.fillOverlay = null;
        self.fillOverlay = None;
        // Java: this.glow = null;
        self.glow = None;
        // Java: this.hsl = null;
        self.hsl = None;
        // Java: this.innerShadow = null;
        self.innerShadow = None;
        // Java: this.lum = null;
        self.lum = None;
        // Java: this.outerShadow = null;
        self.outerShadow = None;
        // Java: this.presetShadow = null;
        self.presetShadow = None;
        // Java: this.reflection = null;
        self.reflection = None;
        // Java: this.relativeOffset = null;
        self.relativeOffset = None;
        // Java: this.softEdge = null;
        self.softEdge = None;
        // Java: this.tint = null;
        self.tint = None;
        // Java: this.xfrm = null;
        self.xfrm = None;
    }

    // Java: void follow(Effect parent)
    pub fn follow(&mut self, parent: Option<Box<Effect>>) {
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

    // Java: Effect parent()
    pub fn parent_ref(&self) -> Option<&Effect> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(&KoZone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: int mode()
    pub fn mode(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 0 : this.parent.mode();
            match &self.parent { None => 0, Some(p) => p.mode() }
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
        self.map.clear_key(1);
    }

    // Java: void putMode(int value)
    pub fn putMode(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, &KoZone);
        self.map.put(1, bin);
    }

    // Java: EffectContainer effectDag()
    pub fn effectDag(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.effectDag == null)
        if self.effectDag.is_none() {
            // Java: if (this.map.has(2))
            if self.map.has(2) {
                // Java: KoBin bin = this.map.byKey(2);
                let bin = self.map.byKey(2);
                // Java: this.effectDag = (EffectContainer)KoBin.Decode.toAny(EffectContainer.class, bin, this.zone);
                self.effectDag = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasEffectDag())
            else if self.parent.as_ref().map_or(false, |p| p.hasEffectDag()) {
                // Java: this.effectDag = this.parent.effectDag();
                let bin = Null::value1();
                self.effectDag = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.effectDag = (EffectContainer)KoBin.Decode.toAny(EffectContainer.class, bin, this.zone);
                self.effectDag = Some(bin);
            }
        }
        // Java: return this.effectDag;
        self.effectDag.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasEffectDag()
    pub fn hasEffectDag(&self) -> bool {
        // Java: return this.map.has(2) || this.effectDag != null && this.effectDag.dirty() || this.parent != null && this.parent.hasEffectDag();
        self.map.has(2) || self.effectDag.is_some() || self.parent.as_ref().map_or(false, |p| p.hasEffectDag())
    }

    // Java: void deleteEffectDag()
    pub fn deleteEffectDag(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
        // Java: if (this.effectDag != null)
        if self.effectDag.is_some() {
            // Java: this.effectDag = null;
            self.effectDag = None;
        }
    }

    // Java: void writeEffectDag(EffectContainer value)
    pub fn writeEffectDag(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(2, bin);
                // Java: if (this.effectDag != null)
                if self.effectDag.is_some() {
                    // Java: this.effectDag = null;
                    self.effectDag = None;
                }
            }
        }
    }

    // Java: void forceWriteEffectDag(EffectContainer value)
    pub fn forceWriteEffectDag(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(2, bin);
            // Java: if (this.effectDag != null)
            if self.effectDag.is_some() {
                // Java: this.effectDag = null;
                self.effectDag = None;
            }
        }
    }

    // Java: EffectReference effect()
    pub fn effect(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.effect == null)
        if self.effect.is_none() {
            // Java: if (this.map.has(3))
            if self.map.has(3) {
                // Java: KoBin bin = this.map.byKey(3);
                let bin = self.map.byKey(3);
                // Java: this.effect = (EffectReference)KoBin.Decode.toAny(EffectReference.class, bin, this.zone);
                self.effect = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasEffect())
            else if self.parent.as_ref().map_or(false, |p| p.hasEffect()) {
                // Java: this.effect = this.parent.effect();
                let bin = Null::value1();
                self.effect = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.effect = (EffectReference)KoBin.Decode.toAny(EffectReference.class, bin, this.zone);
                self.effect = Some(bin);
            }
        }
        // Java: return this.effect;
        self.effect.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasEffect()
    pub fn hasEffect(&self) -> bool {
        // Java: return this.map.has(3) || this.effect != null && this.effect.dirty() || this.parent != null && this.parent.hasEffect();
        self.map.has(3) || self.effect.is_some() || self.parent.as_ref().map_or(false, |p| p.hasEffect())
    }

    // Java: void deleteEffect()
    pub fn deleteEffect(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
        // Java: if (this.effect != null)
        if self.effect.is_some() {
            // Java: this.effect = null;
            self.effect = None;
        }
    }

    // Java: void writeEffect(EffectReference value)
    pub fn writeEffect(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(3, bin);
                // Java: if (this.effect != null)
                if self.effect.is_some() {
                    // Java: this.effect = null;
                    self.effect = None;
                }
            }
        }
    }

    // Java: void forceWriteEffect(EffectReference value)
    pub fn forceWriteEffect(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(3, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(3, bin);
            // Java: if (this.effect != null)
            if self.effect.is_some() {
                // Java: this.effect = null;
                self.effect = None;
            }
        }
    }

    // Java: AlphaBiLevelEffect alphaBiLevel()
    pub fn alphaBiLevel(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.alphaBiLevel == null)
        if self.alphaBiLevel.is_none() {
            // Java: if (this.map.has(4))
            if self.map.has(4) {
                // Java: KoBin bin = this.map.byKey(4);
                let bin = self.map.byKey(4);
                // Java: this.alphaBiLevel = (AlphaBiLevelEffect)KoBin.Decode.toAny(AlphaBiLevelEffect.class, bin, this.zone);
                self.alphaBiLevel = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasAlphaBiLevel())
            else if self.parent.as_ref().map_or(false, |p| p.hasAlphaBiLevel()) {
                // Java: this.alphaBiLevel = this.parent.alphaBiLevel();
                let bin = Null::value1();
                self.alphaBiLevel = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.alphaBiLevel = (AlphaBiLevelEffect)KoBin.Decode.toAny(AlphaBiLevelEffect.class, bin, this.zone);
                self.alphaBiLevel = Some(bin);
            }
        }
        // Java: return this.alphaBiLevel;
        self.alphaBiLevel.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasAlphaBiLevel()
    pub fn hasAlphaBiLevel(&self) -> bool {
        // Java: return this.map.has(4) || this.alphaBiLevel != null && this.alphaBiLevel.dirty() || this.parent != null && this.parent.hasAlphaBiLevel();
        self.map.has(4) || self.alphaBiLevel.is_some() || self.parent.as_ref().map_or(false, |p| p.hasAlphaBiLevel())
    }

    // Java: void deleteAlphaBiLevel()
    pub fn deleteAlphaBiLevel(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.clear_key(4);
        // Java: if (this.alphaBiLevel != null)
        if self.alphaBiLevel.is_some() {
            // Java: this.alphaBiLevel = null;
            self.alphaBiLevel = None;
        }
    }

    // Java: void writeAlphaBiLevel(AlphaBiLevelEffect value)
    pub fn writeAlphaBiLevel(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(4, bin);
                self.map.put(4, bin);
                // Java: if (this.alphaBiLevel != null)
                if self.alphaBiLevel.is_some() {
                    // Java: this.alphaBiLevel = null;
                    self.alphaBiLevel = None;
                }
            }
        }
    }

    // Java: void forceWriteAlphaBiLevel(AlphaBiLevelEffect value)
    pub fn forceWriteAlphaBiLevel(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(4, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(4, bin);
            // Java: if (this.alphaBiLevel != null)
            if self.alphaBiLevel.is_some() {
                // Java: this.alphaBiLevel = null;
                self.alphaBiLevel = None;
            }
        }
    }

    // Java: AlphaInverseEffect alphaInverse()
    pub fn alphaInverse(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.alphaInverse == null)
        if self.alphaInverse.is_none() {
            // Java: if (this.map.has(5))
            if self.map.has(5) {
                // Java: KoBin bin = this.map.byKey(5);
                let bin = self.map.byKey(5);
                // Java: this.alphaInverse = (AlphaInverseEffect)KoBin.Decode.toAny(AlphaInverseEffect.class, bin, this.zone);
                self.alphaInverse = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasAlphaInverse())
            else if self.parent.as_ref().map_or(false, |p| p.hasAlphaInverse()) {
                // Java: this.alphaInverse = this.parent.alphaInverse();
                let bin = Null::value1();
                self.alphaInverse = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.alphaInverse = (AlphaInverseEffect)KoBin.Decode.toAny(AlphaInverseEffect.class, bin, this.zone);
                self.alphaInverse = Some(bin);
            }
        }
        // Java: return this.alphaInverse;
        self.alphaInverse.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasAlphaInverse()
    pub fn hasAlphaInverse(&self) -> bool {
        // Java: return this.map.has(5) || this.alphaInverse != null && this.alphaInverse.dirty() || this.parent != null && this.parent.hasAlphaInverse();
        self.map.has(5) || self.alphaInverse.is_some() || self.parent.as_ref().map_or(false, |p| p.hasAlphaInverse())
    }

    // Java: void deleteAlphaInverse()
    pub fn deleteAlphaInverse(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.clear_key(5);
        // Java: if (this.alphaInverse != null)
        if self.alphaInverse.is_some() {
            // Java: this.alphaInverse = null;
            self.alphaInverse = None;
        }
    }

    // Java: void writeAlphaInverse(AlphaInverseEffect value)
    pub fn writeAlphaInverse(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(5, bin);
                self.map.put(5, bin);
                // Java: if (this.alphaInverse != null)
                if self.alphaInverse.is_some() {
                    // Java: this.alphaInverse = null;
                    self.alphaInverse = None;
                }
            }
        }
    }

    // Java: void forceWriteAlphaInverse(AlphaInverseEffect value)
    pub fn forceWriteAlphaInverse(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(5, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(5, bin);
            // Java: if (this.alphaInverse != null)
            if self.alphaInverse.is_some() {
                // Java: this.alphaInverse = null;
                self.alphaInverse = None;
            }
        }
    }

    // Java: AlphaModEffect alphaMod()
    pub fn alphaMod(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.alphaMod == null)
        if self.alphaMod.is_none() {
            // Java: if (this.map.has(6))
            if self.map.has(6) {
                // Java: KoBin bin = this.map.byKey(6);
                let bin = self.map.byKey(6);
                // Java: this.alphaMod = (AlphaModEffect)KoBin.Decode.toAny(AlphaModEffect.class, bin, this.zone);
                self.alphaMod = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasAlphaMod())
            else if self.parent.as_ref().map_or(false, |p| p.hasAlphaMod()) {
                // Java: this.alphaMod = this.parent.alphaMod();
                let bin = Null::value1();
                self.alphaMod = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.alphaMod = (AlphaModEffect)KoBin.Decode.toAny(AlphaModEffect.class, bin, this.zone);
                self.alphaMod = Some(bin);
            }
        }
        // Java: return this.alphaMod;
        self.alphaMod.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasAlphaMod()
    pub fn hasAlphaMod(&self) -> bool {
        // Java: return this.map.has(6) || this.alphaMod != null && this.alphaMod.dirty() || this.parent != null && this.parent.hasAlphaMod();
        self.map.has(6) || self.alphaMod.is_some() || self.parent.as_ref().map_or(false, |p| p.hasAlphaMod())
    }

    // Java: void deleteAlphaMod()
    pub fn deleteAlphaMod(&mut self) {
        // Java: this.map.put(6, (KoBin)null);
        self.map.clear_key(6);
        // Java: if (this.alphaMod != null)
        if self.alphaMod.is_some() {
            // Java: this.alphaMod = null;
            self.alphaMod = None;
        }
    }

    // Java: void writeAlphaMod(AlphaModEffect value)
    pub fn writeAlphaMod(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(6, bin);
                self.map.put(6, bin);
                // Java: if (this.alphaMod != null)
                if self.alphaMod.is_some() {
                    // Java: this.alphaMod = null;
                    self.alphaMod = None;
                }
            }
        }
    }

    // Java: void forceWriteAlphaMod(AlphaModEffect value)
    pub fn forceWriteAlphaMod(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(6, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(6, bin);
            // Java: if (this.alphaMod != null)
            if self.alphaMod.is_some() {
                // Java: this.alphaMod = null;
                self.alphaMod = None;
            }
        }
    }

    // Java: AlphaModFixEffect alphaModFix()
    pub fn alphaModFix(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.alphaModFix == null)
        if self.alphaModFix.is_none() {
            // Java: if (this.map.has(7))
            if self.map.has(7) {
                // Java: KoBin bin = this.map.byKey(7);
                let bin = self.map.byKey(7);
                // Java: this.alphaModFix = (AlphaModFixEffect)KoBin.Decode.toAny(AlphaModFixEffect.class, bin, this.zone);
                self.alphaModFix = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasAlphaModFix())
            else if self.parent.as_ref().map_or(false, |p| p.hasAlphaModFix()) {
                // Java: this.alphaModFix = this.parent.alphaModFix();
                let bin = Null::value1();
                self.alphaModFix = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.alphaModFix = (AlphaModFixEffect)KoBin.Decode.toAny(AlphaModFixEffect.class, bin, this.zone);
                self.alphaModFix = Some(bin);
            }
        }
        // Java: return this.alphaModFix;
        self.alphaModFix.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasAlphaModFix()
    pub fn hasAlphaModFix(&self) -> bool {
        // Java: return this.map.has(7) || this.alphaModFix != null && this.alphaModFix.dirty() || this.parent != null && this.parent.hasAlphaModFix();
        self.map.has(7) || self.alphaModFix.is_some() || self.parent.as_ref().map_or(false, |p| p.hasAlphaModFix())
    }

    // Java: void deleteAlphaModFix()
    pub fn deleteAlphaModFix(&mut self) {
        // Java: this.map.put(7, (KoBin)null);
        self.map.clear_key(7);
        // Java: if (this.alphaModFix != null)
        if self.alphaModFix.is_some() {
            // Java: this.alphaModFix = null;
            self.alphaModFix = None;
        }
    }

    // Java: void writeAlphaModFix(AlphaModFixEffect value)
    pub fn writeAlphaModFix(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(7, bin);
                self.map.put(7, bin);
                // Java: if (this.alphaModFix != null)
                if self.alphaModFix.is_some() {
                    // Java: this.alphaModFix = null;
                    self.alphaModFix = None;
                }
            }
        }
    }

    // Java: void forceWriteAlphaModFix(AlphaModFixEffect value)
    pub fn forceWriteAlphaModFix(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(7, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(7, bin);
            // Java: if (this.alphaModFix != null)
            if self.alphaModFix.is_some() {
                // Java: this.alphaModFix = null;
                self.alphaModFix = None;
            }
        }
    }

    // Java: AlphaOutsetEffect alphaOutset()
    pub fn alphaOutset(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.alphaOutset == null)
        if self.alphaOutset.is_none() {
            // Java: if (this.map.has(8))
            if self.map.has(8) {
                // Java: KoBin bin = this.map.byKey(8);
                let bin = self.map.byKey(8);
                // Java: this.alphaOutset = (AlphaOutsetEffect)KoBin.Decode.toAny(AlphaOutsetEffect.class, bin, this.zone);
                self.alphaOutset = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasAlphaOutset())
            else if self.parent.as_ref().map_or(false, |p| p.hasAlphaOutset()) {
                // Java: this.alphaOutset = this.parent.alphaOutset();
                let bin = Null::value1();
                self.alphaOutset = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.alphaOutset = (AlphaOutsetEffect)KoBin.Decode.toAny(AlphaOutsetEffect.class, bin, this.zone);
                self.alphaOutset = Some(bin);
            }
        }
        // Java: return this.alphaOutset;
        self.alphaOutset.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasAlphaOutset()
    pub fn hasAlphaOutset(&self) -> bool {
        // Java: return this.map.has(8) || this.alphaOutset != null && this.alphaOutset.dirty() || this.parent != null && this.parent.hasAlphaOutset();
        self.map.has(8) || self.alphaOutset.is_some() || self.parent.as_ref().map_or(false, |p| p.hasAlphaOutset())
    }

    // Java: void deleteAlphaOutset()
    pub fn deleteAlphaOutset(&mut self) {
        // Java: this.map.put(8, (KoBin)null);
        self.map.clear_key(8);
        // Java: if (this.alphaOutset != null)
        if self.alphaOutset.is_some() {
            // Java: this.alphaOutset = null;
            self.alphaOutset = None;
        }
    }

    // Java: void writeAlphaOutset(AlphaOutsetEffect value)
    pub fn writeAlphaOutset(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(8, bin);
                self.map.put(8, bin);
                // Java: if (this.alphaOutset != null)
                if self.alphaOutset.is_some() {
                    // Java: this.alphaOutset = null;
                    self.alphaOutset = None;
                }
            }
        }
    }

    // Java: void forceWriteAlphaOutset(AlphaOutsetEffect value)
    pub fn forceWriteAlphaOutset(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(8, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(8, bin);
            // Java: if (this.alphaOutset != null)
            if self.alphaOutset.is_some() {
                // Java: this.alphaOutset = null;
                self.alphaOutset = None;
            }
        }
    }

    // Java: AlphaReplaceEffect alphaReplacement()
    pub fn alphaReplacement(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.alphaReplacement == null)
        if self.alphaReplacement.is_none() {
            // Java: if (this.map.has(9))
            if self.map.has(9) {
                // Java: KoBin bin = this.map.byKey(9);
                let bin = self.map.byKey(9);
                // Java: this.alphaReplacement = (AlphaReplaceEffect)KoBin.Decode.toAny(AlphaReplaceEffect.class, bin, this.zone);
                self.alphaReplacement = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasAlphaReplacement())
            else if self.parent.as_ref().map_or(false, |p| p.hasAlphaReplacement()) {
                // Java: this.alphaReplacement = this.parent.alphaReplacement();
                let bin = Null::value1();
                self.alphaReplacement = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.alphaReplacement = (AlphaReplaceEffect)KoBin.Decode.toAny(AlphaReplaceEffect.class, bin, this.zone);
                self.alphaReplacement = Some(bin);
            }
        }
        // Java: return this.alphaReplacement;
        self.alphaReplacement.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasAlphaReplacement()
    pub fn hasAlphaReplacement(&self) -> bool {
        // Java: return this.map.has(9) || this.alphaReplacement != null && this.alphaReplacement.dirty() || this.parent != null && this.parent.hasAlphaReplacement();
        self.map.has(9) || self.alphaReplacement.is_some() || self.parent.as_ref().map_or(false, |p| p.hasAlphaReplacement())
    }

    // Java: void deleteAlphaReplacement()
    pub fn deleteAlphaReplacement(&mut self) {
        // Java: this.map.put(9, (KoBin)null);
        self.map.clear_key(9);
        // Java: if (this.alphaReplacement != null)
        if self.alphaReplacement.is_some() {
            // Java: this.alphaReplacement = null;
            self.alphaReplacement = None;
        }
    }

    // Java: void writeAlphaReplacement(AlphaReplaceEffect value)
    pub fn writeAlphaReplacement(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(9, bin);
                self.map.put(9, bin);
                // Java: if (this.alphaReplacement != null)
                if self.alphaReplacement.is_some() {
                    // Java: this.alphaReplacement = null;
                    self.alphaReplacement = None;
                }
            }
        }
    }

    // Java: void forceWriteAlphaReplacement(AlphaReplaceEffect value)
    pub fn forceWriteAlphaReplacement(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(9, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(9, bin);
            // Java: if (this.alphaReplacement != null)
            if self.alphaReplacement.is_some() {
                // Java: this.alphaReplacement = null;
                self.alphaReplacement = None;
            }
        }
    }

    // Java: BiLevelEffect biLevel()
    pub fn biLevel(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.biLevel == null)
        if self.biLevel.is_none() {
            // Java: if (this.map.has(10))
            if self.map.has(10) {
                // Java: KoBin bin = this.map.byKey(10);
                let bin = self.map.byKey(10);
                // Java: this.biLevel = (BiLevelEffect)KoBin.Decode.toAny(BiLevelEffect.class, bin, this.zone);
                self.biLevel = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasBiLevel())
            else if self.parent.as_ref().map_or(false, |p| p.hasBiLevel()) {
                // Java: this.biLevel = this.parent.biLevel();
                let bin = Null::value1();
                self.biLevel = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.biLevel = (BiLevelEffect)KoBin.Decode.toAny(BiLevelEffect.class, bin, this.zone);
                self.biLevel = Some(bin);
            }
        }
        // Java: return this.biLevel;
        self.biLevel.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasBiLevel()
    pub fn hasBiLevel(&self) -> bool {
        // Java: return this.map.has(10) || this.biLevel != null && this.biLevel.dirty() || this.parent != null && this.parent.hasBiLevel();
        self.map.has(10) || self.biLevel.is_some() || self.parent.as_ref().map_or(false, |p| p.hasBiLevel())
    }

    // Java: void deleteBiLevel()
    pub fn deleteBiLevel(&mut self) {
        // Java: this.map.put(10, (KoBin)null);
        self.map.clear_key(10);
        // Java: if (this.biLevel != null)
        if self.biLevel.is_some() {
            // Java: this.biLevel = null;
            self.biLevel = None;
        }
    }

    // Java: void writeBiLevel(BiLevelEffect value)
    pub fn writeBiLevel(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(10, bin);
                self.map.put(10, bin);
                // Java: if (this.biLevel != null)
                if self.biLevel.is_some() {
                    // Java: this.biLevel = null;
                    self.biLevel = None;
                }
            }
        }
    }

    // Java: void forceWriteBiLevel(BiLevelEffect value)
    pub fn forceWriteBiLevel(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(10, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(10, bin);
            // Java: if (this.biLevel != null)
            if self.biLevel.is_some() {
                // Java: this.biLevel = null;
                self.biLevel = None;
            }
        }
    }

    // Java: BlendEffect blend()
    pub fn blend(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.blend == null)
        if self.blend.is_none() {
            // Java: if (this.map.has(11))
            if self.map.has(11) {
                // Java: KoBin bin = this.map.byKey(11);
                let bin = self.map.byKey(11);
                // Java: this.blend = (BlendEffect)KoBin.Decode.toAny(BlendEffect.class, bin, this.zone);
                self.blend = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasBlend())
            else if self.parent.as_ref().map_or(false, |p| p.hasBlend()) {
                // Java: this.blend = this.parent.blend();
                let bin = Null::value1();
                self.blend = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.blend = (BlendEffect)KoBin.Decode.toAny(BlendEffect.class, bin, this.zone);
                self.blend = Some(bin);
            }
        }
        // Java: return this.blend;
        self.blend.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasBlend()
    pub fn hasBlend(&self) -> bool {
        // Java: return this.map.has(11) || this.blend != null && this.blend.dirty() || this.parent != null && this.parent.hasBlend();
        self.map.has(11) || self.blend.is_some() || self.parent.as_ref().map_or(false, |p| p.hasBlend())
    }

    // Java: void deleteBlend()
    pub fn deleteBlend(&mut self) {
        // Java: this.map.put(11, (KoBin)null);
        self.map.clear_key(11);
        // Java: if (this.blend != null)
        if self.blend.is_some() {
            // Java: this.blend = null;
            self.blend = None;
        }
    }

    // Java: void writeBlend(BlendEffect value)
    pub fn writeBlend(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(11, bin);
                self.map.put(11, bin);
                // Java: if (this.blend != null)
                if self.blend.is_some() {
                    // Java: this.blend = null;
                    self.blend = None;
                }
            }
        }
    }

    // Java: void forceWriteBlend(BlendEffect value)
    pub fn forceWriteBlend(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(11, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(11, bin);
            // Java: if (this.blend != null)
            if self.blend.is_some() {
                // Java: this.blend = null;
                self.blend = None;
            }
        }
    }

    // Java: BlurEffect blur()
    pub fn blur(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.blur == null)
        if self.blur.is_none() {
            // Java: if (this.map.has(12))
            if self.map.has(12) {
                // Java: KoBin bin = this.map.byKey(12);
                let bin = self.map.byKey(12);
                // Java: this.blur = (BlurEffect)KoBin.Decode.toAny(BlurEffect.class, bin, this.zone);
                self.blur = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasBlur())
            else if self.parent.as_ref().map_or(false, |p| p.hasBlur()) {
                // Java: this.blur = this.parent.blur();
                let bin = Null::value1();
                self.blur = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.blur = (BlurEffect)KoBin.Decode.toAny(BlurEffect.class, bin, this.zone);
                self.blur = Some(bin);
            }
        }
        // Java: return this.blur;
        self.blur.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasBlur()
    pub fn hasBlur(&self) -> bool {
        // Java: return this.map.has(12) || this.blur != null && this.blur.dirty() || this.parent != null && this.parent.hasBlur();
        self.map.has(12) || self.blur.is_some() || self.parent.as_ref().map_or(false, |p| p.hasBlur())
    }

    // Java: void deleteBlur()
    pub fn deleteBlur(&mut self) {
        // Java: this.map.put(12, (KoBin)null);
        self.map.clear_key(12);
        // Java: if (this.blur != null)
        if self.blur.is_some() {
            // Java: this.blur = null;
            self.blur = None;
        }
    }

    // Java: void writeBlur(BlurEffect value)
    pub fn writeBlur(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(12, bin);
                self.map.put(12, bin);
                // Java: if (this.blur != null)
                if self.blur.is_some() {
                    // Java: this.blur = null;
                    self.blur = None;
                }
            }
        }
    }

    // Java: void forceWriteBlur(BlurEffect value)
    pub fn forceWriteBlur(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(12, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(12, bin);
            // Java: if (this.blur != null)
            if self.blur.is_some() {
                // Java: this.blur = null;
                self.blur = None;
            }
        }
    }

    // Java: ColorChangeEffect clrChange()
    pub fn clrChange(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.clrChange == null)
        if self.clrChange.is_none() {
            // Java: if (this.map.has(13))
            if self.map.has(13) {
                // Java: KoBin bin = this.map.byKey(13);
                let bin = self.map.byKey(13);
                // Java: this.clrChange = (ColorChangeEffect)KoBin.Decode.toAny(ColorChangeEffect.class, bin, this.zone);
                self.clrChange = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasClrChange())
            else if self.parent.as_ref().map_or(false, |p| p.hasClrChange()) {
                // Java: this.clrChange = this.parent.clrChange();
                let bin = Null::value1();
                self.clrChange = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.clrChange = (ColorChangeEffect)KoBin.Decode.toAny(ColorChangeEffect.class, bin, this.zone);
                self.clrChange = Some(bin);
            }
        }
        // Java: return this.clrChange;
        self.clrChange.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasClrChange()
    pub fn hasClrChange(&self) -> bool {
        // Java: return this.map.has(13) || this.clrChange != null && this.clrChange.dirty() || this.parent != null && this.parent.hasClrChange();
        self.map.has(13) || self.clrChange.is_some() || self.parent.as_ref().map_or(false, |p| p.hasClrChange())
    }

    // Java: void deleteClrChange()
    pub fn deleteClrChange(&mut self) {
        // Java: this.map.put(13, (KoBin)null);
        self.map.clear_key(13);
        // Java: if (this.clrChange != null)
        if self.clrChange.is_some() {
            // Java: this.clrChange = null;
            self.clrChange = None;
        }
    }

    // Java: void writeClrChange(ColorChangeEffect value)
    pub fn writeClrChange(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(13, bin);
                self.map.put(13, bin);
                // Java: if (this.clrChange != null)
                if self.clrChange.is_some() {
                    // Java: this.clrChange = null;
                    self.clrChange = None;
                }
            }
        }
    }

    // Java: void forceWriteClrChange(ColorChangeEffect value)
    pub fn forceWriteClrChange(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(13, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(13, bin);
            // Java: if (this.clrChange != null)
            if self.clrChange.is_some() {
                // Java: this.clrChange = null;
                self.clrChange = None;
            }
        }
    }

    // Java: ColorReplacementEffect clrReplacement()
    pub fn clrReplacement(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.clrReplacement == null)
        if self.clrReplacement.is_none() {
            // Java: if (this.map.has(14))
            if self.map.has(14) {
                // Java: KoBin bin = this.map.byKey(14);
                let bin = self.map.byKey(14);
                // Java: this.clrReplacement = (ColorReplacementEffect)KoBin.Decode.toAny(ColorReplacementEffect.class, bin, this.zone);
                self.clrReplacement = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasClrReplacement())
            else if self.parent.as_ref().map_or(false, |p| p.hasClrReplacement()) {
                // Java: this.clrReplacement = this.parent.clrReplacement();
                let bin = Null::value1();
                self.clrReplacement = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.clrReplacement = (ColorReplacementEffect)KoBin.Decode.toAny(ColorReplacementEffect.class, bin, this.zone);
                self.clrReplacement = Some(bin);
            }
        }
        // Java: return this.clrReplacement;
        self.clrReplacement.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasClrReplacement()
    pub fn hasClrReplacement(&self) -> bool {
        // Java: return this.map.has(14) || this.clrReplacement != null && this.clrReplacement.dirty() || this.parent != null && this.parent.hasClrReplacement();
        self.map.has(14) || self.clrReplacement.is_some() || self.parent.as_ref().map_or(false, |p| p.hasClrReplacement())
    }

    // Java: void deleteClrReplacement()
    pub fn deleteClrReplacement(&mut self) {
        // Java: this.map.put(14, (KoBin)null);
        self.map.clear_key(14);
        // Java: if (this.clrReplacement != null)
        if self.clrReplacement.is_some() {
            // Java: this.clrReplacement = null;
            self.clrReplacement = None;
        }
    }

    // Java: void writeClrReplacement(ColorReplacementEffect value)
    pub fn writeClrReplacement(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(14, bin);
                self.map.put(14, bin);
                // Java: if (this.clrReplacement != null)
                if self.clrReplacement.is_some() {
                    // Java: this.clrReplacement = null;
                    self.clrReplacement = None;
                }
            }
        }
    }

    // Java: void forceWriteClrReplacement(ColorReplacementEffect value)
    pub fn forceWriteClrReplacement(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(14, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(14, bin);
            // Java: if (this.clrReplacement != null)
            if self.clrReplacement.is_some() {
                // Java: this.clrReplacement = null;
                self.clrReplacement = None;
            }
        }
    }

    // Java: DuotoneEffect duotone()
    pub fn duotone(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.duotone == null)
        if self.duotone.is_none() {
            // Java: if (this.map.has(15))
            if self.map.has(15) {
                // Java: KoBin bin = this.map.byKey(15);
                let bin = self.map.byKey(15);
                // Java: this.duotone = (DuotoneEffect)KoBin.Decode.toAny(DuotoneEffect.class, bin, this.zone);
                self.duotone = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasDuotone())
            else if self.parent.as_ref().map_or(false, |p| p.hasDuotone()) {
                // Java: this.duotone = this.parent.duotone();
                let bin = Null::value1();
                self.duotone = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.duotone = (DuotoneEffect)KoBin.Decode.toAny(DuotoneEffect.class, bin, this.zone);
                self.duotone = Some(bin);
            }
        }
        // Java: return this.duotone;
        self.duotone.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasDuotone()
    pub fn hasDuotone(&self) -> bool {
        // Java: return this.map.has(15) || this.duotone != null && this.duotone.dirty() || this.parent != null && this.parent.hasDuotone();
        self.map.has(15) || self.duotone.is_some() || self.parent.as_ref().map_or(false, |p| p.hasDuotone())
    }

    // Java: void deleteDuotone()
    pub fn deleteDuotone(&mut self) {
        // Java: this.map.put(15, (KoBin)null);
        self.map.clear_key(15);
        // Java: if (this.duotone != null)
        if self.duotone.is_some() {
            // Java: this.duotone = null;
            self.duotone = None;
        }
    }

    // Java: void writeDuotone(DuotoneEffect value)
    pub fn writeDuotone(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(15, bin);
                self.map.put(15, bin);
                // Java: if (this.duotone != null)
                if self.duotone.is_some() {
                    // Java: this.duotone = null;
                    self.duotone = None;
                }
            }
        }
    }

    // Java: void forceWriteDuotone(DuotoneEffect value)
    pub fn forceWriteDuotone(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(15, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(15, bin);
            // Java: if (this.duotone != null)
            if self.duotone.is_some() {
                // Java: this.duotone = null;
                self.duotone = None;
            }
        }
    }

    // Java: FillEffect fill()
    pub fn fill(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.fill == null)
        if self.fill.is_none() {
            // Java: if (this.map.has(16))
            if self.map.has(16) {
                // Java: KoBin bin = this.map.byKey(16);
                let bin = self.map.byKey(16);
                // Java: this.fill = (FillEffect)KoBin.Decode.toAny(FillEffect.class, bin, this.zone);
                self.fill = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasFill())
            else if self.parent.as_ref().map_or(false, |p| p.hasFill()) {
                // Java: this.fill = this.parent.fill();
                let bin = Null::value1();
                self.fill = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.fill = (FillEffect)KoBin.Decode.toAny(FillEffect.class, bin, this.zone);
                self.fill = Some(bin);
            }
        }
        // Java: return this.fill;
        self.fill.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
        // Java: return this.map.has(16) || this.fill != null && this.fill.dirty() || this.parent != null && this.parent.hasFill();
        self.map.has(16) || self.fill.is_some() || self.parent.as_ref().map_or(false, |p| p.hasFill())
    }

    // Java: void deleteFill()
    pub fn deleteFill(&mut self) {
        // Java: this.map.put(16, (KoBin)null);
        self.map.clear_key(16);
        // Java: if (this.fill != null)
        if self.fill.is_some() {
            // Java: this.fill = null;
            self.fill = None;
        }
    }

    // Java: void writeFill(FillEffect value)
    pub fn writeFill(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(16, bin);
                self.map.put(16, bin);
                // Java: if (this.fill != null)
                if self.fill.is_some() {
                    // Java: this.fill = null;
                    self.fill = None;
                }
            }
        }
    }

    // Java: void forceWriteFill(FillEffect value)
    pub fn forceWriteFill(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(16, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(16, bin);
            // Java: if (this.fill != null)
            if self.fill.is_some() {
                // Java: this.fill = null;
                self.fill = None;
            }
        }
    }

    // Java: FillOverlayEffect fillOverlay()
    pub fn fillOverlay(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.fillOverlay == null)
        if self.fillOverlay.is_none() {
            // Java: if (this.map.has(17))
            if self.map.has(17) {
                // Java: KoBin bin = this.map.byKey(17);
                let bin = self.map.byKey(17);
                // Java: this.fillOverlay = (FillOverlayEffect)KoBin.Decode.toAny(FillOverlayEffect.class, bin, this.zone);
                self.fillOverlay = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasFillOverlay())
            else if self.parent.as_ref().map_or(false, |p| p.hasFillOverlay()) {
                // Java: this.fillOverlay = this.parent.fillOverlay();
                let bin = Null::value1();
                self.fillOverlay = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.fillOverlay = (FillOverlayEffect)KoBin.Decode.toAny(FillOverlayEffect.class, bin, this.zone);
                self.fillOverlay = Some(bin);
            }
        }
        // Java: return this.fillOverlay;
        self.fillOverlay.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasFillOverlay()
    pub fn hasFillOverlay(&self) -> bool {
        // Java: return this.map.has(17) || this.fillOverlay != null && this.fillOverlay.dirty() || this.parent != null && this.parent.hasFillOverlay();
        self.map.has(17) || self.fillOverlay.is_some() || self.parent.as_ref().map_or(false, |p| p.hasFillOverlay())
    }

    // Java: void deleteFillOverlay()
    pub fn deleteFillOverlay(&mut self) {
        // Java: this.map.put(17, (KoBin)null);
        self.map.clear_key(17);
        // Java: if (this.fillOverlay != null)
        if self.fillOverlay.is_some() {
            // Java: this.fillOverlay = null;
            self.fillOverlay = None;
        }
    }

    // Java: void writeFillOverlay(FillOverlayEffect value)
    pub fn writeFillOverlay(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(17, bin);
                self.map.put(17, bin);
                // Java: if (this.fillOverlay != null)
                if self.fillOverlay.is_some() {
                    // Java: this.fillOverlay = null;
                    self.fillOverlay = None;
                }
            }
        }
    }

    // Java: void forceWriteFillOverlay(FillOverlayEffect value)
    pub fn forceWriteFillOverlay(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(17, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(17, bin);
            // Java: if (this.fillOverlay != null)
            if self.fillOverlay.is_some() {
                // Java: this.fillOverlay = null;
                self.fillOverlay = None;
            }
        }
    }

    // Java: GlowEffect glow()
    pub fn glow(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.glow == null)
        if self.glow.is_none() {
            // Java: if (this.map.has(18))
            if self.map.has(18) {
                // Java: KoBin bin = this.map.byKey(18);
                let bin = self.map.byKey(18);
                // Java: this.glow = (GlowEffect)KoBin.Decode.toAny(GlowEffect.class, bin, this.zone);
                self.glow = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasGlow())
            else if self.parent.as_ref().map_or(false, |p| p.hasGlow()) {
                // Java: this.glow = this.parent.glow();
                let bin = Null::value1();
                self.glow = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.glow = (GlowEffect)KoBin.Decode.toAny(GlowEffect.class, bin, this.zone);
                self.glow = Some(bin);
            }
        }
        // Java: return this.glow;
        self.glow.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasGlow()
    pub fn hasGlow(&self) -> bool {
        // Java: return this.map.has(18) || this.glow != null && this.glow.dirty() || this.parent != null && this.parent.hasGlow();
        self.map.has(18) || self.glow.is_some() || self.parent.as_ref().map_or(false, |p| p.hasGlow())
    }

    // Java: void deleteGlow()
    pub fn deleteGlow(&mut self) {
        // Java: this.map.put(18, (KoBin)null);
        self.map.clear_key(18);
        // Java: if (this.glow != null)
        if self.glow.is_some() {
            // Java: this.glow = null;
            self.glow = None;
        }
    }

    // Java: void writeGlow(GlowEffect value)
    pub fn writeGlow(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(18, bin);
                self.map.put(18, bin);
                // Java: if (this.glow != null)
                if self.glow.is_some() {
                    // Java: this.glow = null;
                    self.glow = None;
                }
            }
        }
    }

    // Java: void forceWriteGlow(GlowEffect value)
    pub fn forceWriteGlow(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(18, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(18, bin);
            // Java: if (this.glow != null)
            if self.glow.is_some() {
                // Java: this.glow = null;
                self.glow = None;
            }
        }
    }

    // Java: HSLEffect hsl()
    pub fn hsl(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.hsl == null)
        if self.hsl.is_none() {
            // Java: if (this.map.has(19))
            if self.map.has(19) {
                // Java: KoBin bin = this.map.byKey(19);
                let bin = self.map.byKey(19);
                // Java: this.hsl = (HSLEffect)KoBin.Decode.toAny(HSLEffect.class, bin, this.zone);
                self.hsl = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasHsl())
            else if self.parent.as_ref().map_or(false, |p| p.hasHsl()) {
                // Java: this.hsl = this.parent.hsl();
                let bin = Null::value1();
                self.hsl = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.hsl = (HSLEffect)KoBin.Decode.toAny(HSLEffect.class, bin, this.zone);
                self.hsl = Some(bin);
            }
        }
        // Java: return this.hsl;
        self.hsl.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasHsl()
    pub fn hasHsl(&self) -> bool {
        // Java: return this.map.has(19) || this.hsl != null && this.hsl.dirty() || this.parent != null && this.parent.hasHsl();
        self.map.has(19) || self.hsl.is_some() || self.parent.as_ref().map_or(false, |p| p.hasHsl())
    }

    // Java: void deleteHsl()
    pub fn deleteHsl(&mut self) {
        // Java: this.map.put(19, (KoBin)null);
        self.map.clear_key(19);
        // Java: if (this.hsl != null)
        if self.hsl.is_some() {
            // Java: this.hsl = null;
            self.hsl = None;
        }
    }

    // Java: void writeHsl(HSLEffect value)
    pub fn writeHsl(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(19, bin);
                self.map.put(19, bin);
                // Java: if (this.hsl != null)
                if self.hsl.is_some() {
                    // Java: this.hsl = null;
                    self.hsl = None;
                }
            }
        }
    }

    // Java: void forceWriteHsl(HSLEffect value)
    pub fn forceWriteHsl(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(19, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(19, bin);
            // Java: if (this.hsl != null)
            if self.hsl.is_some() {
                // Java: this.hsl = null;
                self.hsl = None;
            }
        }
    }

    // Java: InnerShadowEffect innerShadow()
    pub fn innerShadow(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.innerShadow == null)
        if self.innerShadow.is_none() {
            // Java: if (this.map.has(20))
            if self.map.has(20) {
                // Java: KoBin bin = this.map.byKey(20);
                let bin = self.map.byKey(20);
                // Java: this.innerShadow = (InnerShadowEffect)KoBin.Decode.toAny(InnerShadowEffect.class, bin, this.zone);
                self.innerShadow = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasInnerShadow())
            else if self.parent.as_ref().map_or(false, |p| p.hasInnerShadow()) {
                // Java: this.innerShadow = this.parent.innerShadow();
                let bin = Null::value1();
                self.innerShadow = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.innerShadow = (InnerShadowEffect)KoBin.Decode.toAny(InnerShadowEffect.class, bin, this.zone);
                self.innerShadow = Some(bin);
            }
        }
        // Java: return this.innerShadow;
        self.innerShadow.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasInnerShadow()
    pub fn hasInnerShadow(&self) -> bool {
        // Java: return this.map.has(20) || this.innerShadow != null && this.innerShadow.dirty() || this.parent != null && this.parent.hasInnerShadow();
        self.map.has(20) || self.innerShadow.is_some() || self.parent.as_ref().map_or(false, |p| p.hasInnerShadow())
    }

    // Java: void deleteInnerShadow()
    pub fn deleteInnerShadow(&mut self) {
        // Java: this.map.put(20, (KoBin)null);
        self.map.clear_key(20);
        // Java: if (this.innerShadow != null)
        if self.innerShadow.is_some() {
            // Java: this.innerShadow = null;
            self.innerShadow = None;
        }
    }

    // Java: void writeInnerShadow(InnerShadowEffect value)
    pub fn writeInnerShadow(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(20, bin);
                self.map.put(20, bin);
                // Java: if (this.innerShadow != null)
                if self.innerShadow.is_some() {
                    // Java: this.innerShadow = null;
                    self.innerShadow = None;
                }
            }
        }
    }

    // Java: void forceWriteInnerShadow(InnerShadowEffect value)
    pub fn forceWriteInnerShadow(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(20, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(20, bin);
            // Java: if (this.innerShadow != null)
            if self.innerShadow.is_some() {
                // Java: this.innerShadow = null;
                self.innerShadow = None;
            }
        }
    }

    // Java: LumEffect lum()
    pub fn lum(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.lum == null)
        if self.lum.is_none() {
            // Java: if (this.map.has(21))
            if self.map.has(21) {
                // Java: KoBin bin = this.map.byKey(21);
                let bin = self.map.byKey(21);
                // Java: this.lum = (LumEffect)KoBin.Decode.toAny(LumEffect.class, bin, this.zone);
                self.lum = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasLum())
            else if self.parent.as_ref().map_or(false, |p| p.hasLum()) {
                // Java: this.lum = this.parent.lum();
                let bin = Null::value1();
                self.lum = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.lum = (LumEffect)KoBin.Decode.toAny(LumEffect.class, bin, this.zone);
                self.lum = Some(bin);
            }
        }
        // Java: return this.lum;
        self.lum.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasLum()
    pub fn hasLum(&self) -> bool {
        // Java: return this.map.has(21) || this.lum != null && this.lum.dirty() || this.parent != null && this.parent.hasLum();
        self.map.has(21) || self.lum.is_some() || self.parent.as_ref().map_or(false, |p| p.hasLum())
    }

    // Java: void deleteLum()
    pub fn deleteLum(&mut self) {
        // Java: this.map.put(21, (KoBin)null);
        self.map.clear_key(21);
        // Java: if (this.lum != null)
        if self.lum.is_some() {
            // Java: this.lum = null;
            self.lum = None;
        }
    }

    // Java: void writeLum(LumEffect value)
    pub fn writeLum(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(21, bin);
                self.map.put(21, bin);
                // Java: if (this.lum != null)
                if self.lum.is_some() {
                    // Java: this.lum = null;
                    self.lum = None;
                }
            }
        }
    }

    // Java: void forceWriteLum(LumEffect value)
    pub fn forceWriteLum(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(21, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(21, bin);
            // Java: if (this.lum != null)
            if self.lum.is_some() {
                // Java: this.lum = null;
                self.lum = None;
            }
        }
    }

    // Java: OuterShadowEffect outerShadow()
    pub fn outerShadow(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.outerShadow == null)
        if self.outerShadow.is_none() {
            // Java: if (this.map.has(22))
            if self.map.has(22) {
                // Java: KoBin bin = this.map.byKey(22);
                let bin = self.map.byKey(22);
                // Java: this.outerShadow = (OuterShadowEffect)KoBin.Decode.toAny(OuterShadowEffect.class, bin, this.zone);
                self.outerShadow = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasOuterShadow())
            else if self.parent.as_ref().map_or(false, |p| p.hasOuterShadow()) {
                // Java: this.outerShadow = this.parent.outerShadow();
                let bin = Null::value1();
                self.outerShadow = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.outerShadow = (OuterShadowEffect)KoBin.Decode.toAny(OuterShadowEffect.class, bin, this.zone);
                self.outerShadow = Some(bin);
            }
        }
        // Java: return this.outerShadow;
        self.outerShadow.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasOuterShadow()
    pub fn hasOuterShadow(&self) -> bool {
        // Java: return this.map.has(22) || this.outerShadow != null && this.outerShadow.dirty() || this.parent != null && this.parent.hasOuterShadow();
        self.map.has(22) || self.outerShadow.is_some() || self.parent.as_ref().map_or(false, |p| p.hasOuterShadow())
    }

    // Java: void deleteOuterShadow()
    pub fn deleteOuterShadow(&mut self) {
        // Java: this.map.put(22, (KoBin)null);
        self.map.clear_key(22);
        // Java: if (this.outerShadow != null)
        if self.outerShadow.is_some() {
            // Java: this.outerShadow = null;
            self.outerShadow = None;
        }
    }

    // Java: void writeOuterShadow(OuterShadowEffect value)
    pub fn writeOuterShadow(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(22, bin);
                self.map.put(22, bin);
                // Java: if (this.outerShadow != null)
                if self.outerShadow.is_some() {
                    // Java: this.outerShadow = null;
                    self.outerShadow = None;
                }
            }
        }
    }

    // Java: void forceWriteOuterShadow(OuterShadowEffect value)
    pub fn forceWriteOuterShadow(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(22, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(22, bin);
            // Java: if (this.outerShadow != null)
            if self.outerShadow.is_some() {
                // Java: this.outerShadow = null;
                self.outerShadow = None;
            }
        }
    }

    // Java: PresetShadowEffect presetShadow()
    pub fn presetShadow(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.presetShadow == null)
        if self.presetShadow.is_none() {
            // Java: if (this.map.has(23))
            if self.map.has(23) {
                // Java: KoBin bin = this.map.byKey(23);
                let bin = self.map.byKey(23);
                // Java: this.presetShadow = (PresetShadowEffect)KoBin.Decode.toAny(PresetShadowEffect.class, bin, this.zone);
                self.presetShadow = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasPresetShadow())
            else if self.parent.as_ref().map_or(false, |p| p.hasPresetShadow()) {
                // Java: this.presetShadow = this.parent.presetShadow();
                let bin = Null::value1();
                self.presetShadow = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.presetShadow = (PresetShadowEffect)KoBin.Decode.toAny(PresetShadowEffect.class, bin, this.zone);
                self.presetShadow = Some(bin);
            }
        }
        // Java: return this.presetShadow;
        self.presetShadow.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasPresetShadow()
    pub fn hasPresetShadow(&self) -> bool {
        // Java: return this.map.has(23) || this.presetShadow != null && this.presetShadow.dirty() || this.parent != null && this.parent.hasPresetShadow();
        self.map.has(23) || self.presetShadow.is_some() || self.parent.as_ref().map_or(false, |p| p.hasPresetShadow())
    }

    // Java: void deletePresetShadow()
    pub fn deletePresetShadow(&mut self) {
        // Java: this.map.put(23, (KoBin)null);
        self.map.clear_key(23);
        // Java: if (this.presetShadow != null)
        if self.presetShadow.is_some() {
            // Java: this.presetShadow = null;
            self.presetShadow = None;
        }
    }

    // Java: void writePresetShadow(PresetShadowEffect value)
    pub fn writePresetShadow(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(23, bin);
                self.map.put(23, bin);
                // Java: if (this.presetShadow != null)
                if self.presetShadow.is_some() {
                    // Java: this.presetShadow = null;
                    self.presetShadow = None;
                }
            }
        }
    }

    // Java: void forceWritePresetShadow(PresetShadowEffect value)
    pub fn forceWritePresetShadow(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(23, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(23, bin);
            // Java: if (this.presetShadow != null)
            if self.presetShadow.is_some() {
                // Java: this.presetShadow = null;
                self.presetShadow = None;
            }
        }
    }

    // Java: ReflectionEffect reflection()
    pub fn reflection(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.reflection == null)
        if self.reflection.is_none() {
            // Java: if (this.map.has(24))
            if self.map.has(24) {
                // Java: KoBin bin = this.map.byKey(24);
                let bin = self.map.byKey(24);
                // Java: this.reflection = (ReflectionEffect)KoBin.Decode.toAny(ReflectionEffect.class, bin, this.zone);
                self.reflection = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasReflection())
            else if self.parent.as_ref().map_or(false, |p| p.hasReflection()) {
                // Java: this.reflection = this.parent.reflection();
                let bin = Null::value1();
                self.reflection = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.reflection = (ReflectionEffect)KoBin.Decode.toAny(ReflectionEffect.class, bin, this.zone);
                self.reflection = Some(bin);
            }
        }
        // Java: return this.reflection;
        self.reflection.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasReflection()
    pub fn hasReflection(&self) -> bool {
        // Java: return this.map.has(24) || this.reflection != null && this.reflection.dirty() || this.parent != null && this.parent.hasReflection();
        self.map.has(24) || self.reflection.is_some() || self.parent.as_ref().map_or(false, |p| p.hasReflection())
    }

    // Java: void deleteReflection()
    pub fn deleteReflection(&mut self) {
        // Java: this.map.put(24, (KoBin)null);
        self.map.clear_key(24);
        // Java: if (this.reflection != null)
        if self.reflection.is_some() {
            // Java: this.reflection = null;
            self.reflection = None;
        }
    }

    // Java: void writeReflection(ReflectionEffect value)
    pub fn writeReflection(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(24, bin);
                self.map.put(24, bin);
                // Java: if (this.reflection != null)
                if self.reflection.is_some() {
                    // Java: this.reflection = null;
                    self.reflection = None;
                }
            }
        }
    }

    // Java: void forceWriteReflection(ReflectionEffect value)
    pub fn forceWriteReflection(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(24, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(24, bin);
            // Java: if (this.reflection != null)
            if self.reflection.is_some() {
                // Java: this.reflection = null;
                self.reflection = None;
            }
        }
    }

    // Java: RelOffEffect relativeOffset()
    pub fn relativeOffset(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.relativeOffset == null)
        if self.relativeOffset.is_none() {
            // Java: if (this.map.has(25))
            if self.map.has(25) {
                // Java: KoBin bin = this.map.byKey(25);
                let bin = self.map.byKey(25);
                // Java: this.relativeOffset = (RelOffEffect)KoBin.Decode.toAny(RelOffEffect.class, bin, this.zone);
                self.relativeOffset = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasRelativeOffset())
            else if self.parent.as_ref().map_or(false, |p| p.hasRelativeOffset()) {
                // Java: this.relativeOffset = this.parent.relativeOffset();
                let bin = Null::value1();
                self.relativeOffset = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.relativeOffset = (RelOffEffect)KoBin.Decode.toAny(RelOffEffect.class, bin, this.zone);
                self.relativeOffset = Some(bin);
            }
        }
        // Java: return this.relativeOffset;
        self.relativeOffset.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasRelativeOffset()
    pub fn hasRelativeOffset(&self) -> bool {
        // Java: return this.map.has(25) || this.relativeOffset != null && this.relativeOffset.dirty() || this.parent != null && this.parent.hasRelativeOffset();
        self.map.has(25) || self.relativeOffset.is_some() || self.parent.as_ref().map_or(false, |p| p.hasRelativeOffset())
    }

    // Java: void deleteRelativeOffset()
    pub fn deleteRelativeOffset(&mut self) {
        // Java: this.map.put(25, (KoBin)null);
        self.map.clear_key(25);
        // Java: if (this.relativeOffset != null)
        if self.relativeOffset.is_some() {
            // Java: this.relativeOffset = null;
            self.relativeOffset = None;
        }
    }

    // Java: void writeRelativeOffset(RelOffEffect value)
    pub fn writeRelativeOffset(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(25, bin);
                self.map.put(25, bin);
                // Java: if (this.relativeOffset != null)
                if self.relativeOffset.is_some() {
                    // Java: this.relativeOffset = null;
                    self.relativeOffset = None;
                }
            }
        }
    }

    // Java: void forceWriteRelativeOffset(RelOffEffect value)
    pub fn forceWriteRelativeOffset(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(25, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(25, bin);
            // Java: if (this.relativeOffset != null)
            if self.relativeOffset.is_some() {
                // Java: this.relativeOffset = null;
                self.relativeOffset = None;
            }
        }
    }

    // Java: SoftEdgeEffect softEdge()
    pub fn softEdge(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.softEdge == null)
        if self.softEdge.is_none() {
            // Java: if (this.map.has(26))
            if self.map.has(26) {
                // Java: KoBin bin = this.map.byKey(26);
                let bin = self.map.byKey(26);
                // Java: this.softEdge = (SoftEdgeEffect)KoBin.Decode.toAny(SoftEdgeEffect.class, bin, this.zone);
                self.softEdge = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasSoftEdge())
            else if self.parent.as_ref().map_or(false, |p| p.hasSoftEdge()) {
                // Java: this.softEdge = this.parent.softEdge();
                let bin = Null::value1();
                self.softEdge = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.softEdge = (SoftEdgeEffect)KoBin.Decode.toAny(SoftEdgeEffect.class, bin, this.zone);
                self.softEdge = Some(bin);
            }
        }
        // Java: return this.softEdge;
        self.softEdge.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasSoftEdge()
    pub fn hasSoftEdge(&self) -> bool {
        // Java: return this.map.has(26) || this.softEdge != null && this.softEdge.dirty() || this.parent != null && this.parent.hasSoftEdge();
        self.map.has(26) || self.softEdge.is_some() || self.parent.as_ref().map_or(false, |p| p.hasSoftEdge())
    }

    // Java: void deleteSoftEdge()
    pub fn deleteSoftEdge(&mut self) {
        // Java: this.map.put(26, (KoBin)null);
        self.map.clear_key(26);
        // Java: if (this.softEdge != null)
        if self.softEdge.is_some() {
            // Java: this.softEdge = null;
            self.softEdge = None;
        }
    }

    // Java: void writeSoftEdge(SoftEdgeEffect value)
    pub fn writeSoftEdge(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(26, bin);
                self.map.put(26, bin);
                // Java: if (this.softEdge != null)
                if self.softEdge.is_some() {
                    // Java: this.softEdge = null;
                    self.softEdge = None;
                }
            }
        }
    }

    // Java: void forceWriteSoftEdge(SoftEdgeEffect value)
    pub fn forceWriteSoftEdge(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(26, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(26, bin);
            // Java: if (this.softEdge != null)
            if self.softEdge.is_some() {
                // Java: this.softEdge = null;
                self.softEdge = None;
            }
        }
    }

    // Java: TintEffect tint()
    pub fn tint(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.tint == null)
        if self.tint.is_none() {
            // Java: if (this.map.has(27))
            if self.map.has(27) {
                // Java: KoBin bin = this.map.byKey(27);
                let bin = self.map.byKey(27);
                // Java: this.tint = (TintEffect)KoBin.Decode.toAny(TintEffect.class, bin, this.zone);
                self.tint = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasTint())
            else if self.parent.as_ref().map_or(false, |p| p.hasTint()) {
                // Java: this.tint = this.parent.tint();
                let bin = Null::value1();
                self.tint = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.tint = (TintEffect)KoBin.Decode.toAny(TintEffect.class, bin, this.zone);
                self.tint = Some(bin);
            }
        }
        // Java: return this.tint;
        self.tint.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasTint()
    pub fn hasTint(&self) -> bool {
        // Java: return this.map.has(27) || this.tint != null && this.tint.dirty() || this.parent != null && this.parent.hasTint();
        self.map.has(27) || self.tint.is_some() || self.parent.as_ref().map_or(false, |p| p.hasTint())
    }

    // Java: void deleteTint()
    pub fn deleteTint(&mut self) {
        // Java: this.map.put(27, (KoBin)null);
        self.map.clear_key(27);
        // Java: if (this.tint != null)
        if self.tint.is_some() {
            // Java: this.tint = null;
            self.tint = None;
        }
    }

    // Java: void writeTint(TintEffect value)
    pub fn writeTint(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(27, bin);
                self.map.put(27, bin);
                // Java: if (this.tint != null)
                if self.tint.is_some() {
                    // Java: this.tint = null;
                    self.tint = None;
                }
            }
        }
    }

    // Java: void forceWriteTint(TintEffect value)
    pub fn forceWriteTint(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(27, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(27, bin);
            // Java: if (this.tint != null)
            if self.tint.is_some() {
                // Java: this.tint = null;
                self.tint = None;
            }
        }
    }

    // Java: XfrmEffect xfrm()
    pub fn xfrm(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.xfrm == null)
        if self.xfrm.is_none() {
            // Java: if (this.map.has(28))
            if self.map.has(28) {
                // Java: KoBin bin = this.map.byKey(28);
                let bin = self.map.byKey(28);
                // Java: this.xfrm = (XfrmEffect)KoBin.Decode.toAny(XfrmEffect.class, bin, this.zone);
                self.xfrm = Some(Factory::dup(bin.as_ref(), &KoZone));
            }
            // Java: else if (this.parent != null && this.parent.hasXfrm())
            else if self.parent.as_ref().map_or(false, |p| p.hasXfrm()) {
                // Java: this.xfrm = this.parent.xfrm();
                let bin = Null::value1();
                self.xfrm = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.xfrm = (XfrmEffect)KoBin.Decode.toAny(XfrmEffect.class, bin, this.zone);
                self.xfrm = Some(bin);
            }
        }
        // Java: return this.xfrm;
        self.xfrm.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasXfrm()
    pub fn hasXfrm(&self) -> bool {
        // Java: return this.map.has(28) || this.xfrm != null && this.xfrm.dirty() || this.parent != null && this.parent.hasXfrm();
        self.map.has(28) || self.xfrm.is_some() || self.parent.as_ref().map_or(false, |p| p.hasXfrm())
    }

    // Java: void deleteXfrm()
    pub fn deleteXfrm(&mut self) {
        // Java: this.map.put(28, (KoBin)null);
        self.map.clear_key(28);
        // Java: if (this.xfrm != null)
        if self.xfrm.is_some() {
            // Java: this.xfrm = null;
            self.xfrm = None;
        }
    }

    // Java: void writeXfrm(XfrmEffect value)
    pub fn writeXfrm(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(28, bin);
                self.map.put(28, bin);
                // Java: if (this.xfrm != null)
                if self.xfrm.is_some() {
                    // Java: this.xfrm = null;
                    self.xfrm = None;
                }
            }
        }
    }

    // Java: void forceWriteXfrm(XfrmEffect value)
    pub fn forceWriteXfrm(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(28, value.lastBin(this.zone));
            let bin = Factory::dup(val, &KoZone);
            self.map.put(28, bin);
            // Java: if (this.xfrm != null)
            if self.xfrm.is_some() {
                // Java: this.xfrm = null;
                self.xfrm = None;
            }
        }
    }

}

impl KoAr for Effect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { Effect::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { Effect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Effect::lastBin(self, var1) }
}
impl KoClosable for Effect { fn close(&mut self) { Effect::close(self); } }
